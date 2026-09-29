//! One pass over a compiled document (`notes/30Yf` § 3–§ 6, § 8): parse every root once through the
//! adapter, key every command, then for each row in cheap-first order decide whether it is cached,
//! deferred, entailed by its book's conjunction, answered by replaying its last instance, or solved.

use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::book::EVERY_LINE;
use super::emit::Rendered;
use super::key::{self, Fixed};
use super::lock::{Outcome, Row};
use super::replay;
use super::tier::{Caps, DEFER_CLAUSES, Tier};
use crate::alloy_jvm::adapter::{Adapter, Ask, CommandInfo, Parsed, Refusal, Solved};
use crate::alloy_jvm::{self, Jvm};

const SOLVER: &str = "sat4j";

/// Which commands a pass runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Target {
    All,
    Module(String),
    /// A command and what its green needs: its premise twin and its module's consistency run.
    Only {
        module: String,
        label: String,
    },
}

/// Where a computed row's result came from; the report says, the lock does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Provenance {
    Fresh,
    Cached,
    Entailed,
    Replayed,
    Unrun,
}

impl Provenance {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Cached => "cached",
            Self::Entailed => "entailed",
            Self::Replayed => "replayed",
            Self::Unrun => "unrun",
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct Computed {
    pub(super) row: Row,
    pub(super) provenance: Provenance,
    pub(super) wall_ms: Option<u64>,
    pub(super) solve_ms: Option<u64>,
    /// What the report says beside the row: how far a timeout got, why a row went unrun.
    pub(super) note: Option<String>,
    /// The command's position in its module, for the lock's row order.
    pub(super) index: u64,
}

/// Everything a pass needs, borrowed from the compile and the committed lock.
#[derive(Debug)]
pub(super) struct Job<'a> {
    pub(super) out: &'a Path,
    pub(super) modules: &'a [(String, Rendered)],
    pub(super) conjunctions: &'a [(String, Vec<String>)],
    pub(super) committed: &'a [Row],
    pub(super) tier: Tier,
    pub(super) caps: Caps,
    pub(super) target: &'a Target,
}

/// One command, keyed, with what the lock last said of it.
#[derive(Debug, Clone)]
pub(super) struct Entry {
    pub(super) module: String,
    pub(super) info: CommandInfo,
    pub(super) key: Option<String>,
    guard: Option<String>,
    sigs: Vec<(String, Vec<String>)>,
    committed: Option<Row>,
    /// The effective options the key was computed under, which every solve must echo.
    options: String,
    platform_fail: Option<String>,
}

impl Entry {
    fn kind(&self) -> &'static str {
        if self.info.check { "check" } else { "run" }
    }

    fn cost(&self) -> u64 {
        self.committed
            .as_ref()
            .and_then(|c| c.size)
            .map_or(u64::MAX, |s| s.clauses)
    }
}

/// Every root parsed and every command keyed; or the roots Alloy refused, for the parse lint.
#[derive(Debug)]
pub(super) struct Survey {
    pub(super) entries: Vec<Entry>,
    pub(super) refusals: Vec<(String, Refusal)>,
}

fn stem(name: &str) -> &str {
    name.strip_suffix(super::MODULE_SUFFIX).unwrap_or(name)
}

/// The modules a target needs parsed.
fn wanted(target: &Target, module: &str) -> bool {
    match target {
        Target::All => true,
        Target::Module(m) | Target::Only { module: m, .. } => m == module,
    }
}

pub(super) fn survey(adapter: &mut Adapter, job: &Job<'_>) -> Result<Survey, String> {
    let fixed_base = (alloy_jvm::jar_digest()?, alloy_jvm::adapter_digest()?);
    let mut entries = Vec::new();
    let mut refusals = Vec::new();
    for (name, rendered) in job.modules {
        let module = stem(name).to_owned();
        if !wanted(job.target, &module) {
            continue;
        }
        let root = job.out.join(name);
        match adapter.parse(&root, &[], SOLVER) {
            Ok(parsed) => entries.extend(keyed(
                &module,
                rendered,
                &parsed,
                &fixed_base,
                job.committed,
            )),
            Err(r) if r.file.as_deref().is_some_and(|f| f.contains("$alloy4$")) => {
                entries.extend(platform_failed(&module, rendered, &r, job.committed));
            }
            Err(r) => refusals.push((name.clone(), r)),
        }
    }
    Ok(Survey { entries, refusals })
}

fn keyed(
    module: &str,
    rendered: &Rendered,
    parsed: &Parsed,
    (jar, adapter): &(String, String),
    committed: &[Row],
) -> Vec<Entry> {
    let fixed = Fixed {
        jar: jar.clone(),
        adapter: adapter.clone(),
        options: parsed.options.compact(),
    };
    let mut unnamed = 0usize;
    parsed
        .commands
        .iter()
        .filter(|c| !c.synthesized)
        .map(|c| {
            let nth = if c.label.contains('$') {
                unnamed = unnamed.saturating_add(1);
                unnamed.saturating_sub(1)
            } else {
                0
            };
            let text = key::command_text(&rendered.text, &c.label, nth).unwrap_or_default();
            Entry {
                module: module.to_owned(),
                key: Some(key::key(&parsed.loaded, &text, &c.scope, &fixed)),
                guard: Some(replay::guard(&parsed.loaded, &c.scope)),
                sigs: parsed.sigs.clone(),
                committed: committed
                    .iter()
                    .find(|r| r.module == module && r.name == c.label)
                    .cloned(),
                info: c.clone(),
                options: fixed.options.clone(),
                platform_fail: None,
            }
        })
        .collect()
}

/// A root this jar refuses on this platform inside its own library: every command it holds is
/// unmeasured here, named from the text assay emitted, since Alloy never listed them.
fn platform_failed(
    module: &str,
    rendered: &Rendered,
    r: &Refusal,
    committed: &[Row],
) -> Vec<Entry> {
    super::alloy::items(&rendered.text, 1)
        .into_iter()
        .filter_map(|item| match item.head() {
            super::Head::Command {
                check,
                name: Some(label),
                ..
            } => Some((check, label)),
            _ => None,
        })
        .zip(0u64..)
        .map(|((check, label), index)| {
            let committed = committed
                .iter()
                .find(|c| c.module == module && c.name == label)
                .cloned();
            Entry {
                module: module.to_owned(),
                info: CommandInfo {
                    index,
                    scope: committed
                        .as_ref()
                        .map(|c| c.scope.clone())
                        .unwrap_or_default(),
                    label,
                    check,
                    expects: None,
                    bitwidth: 0,
                    unbounded_steps: false,
                    synthesized: false,
                },
                key: None,
                guard: None,
                sigs: Vec::new(),
                committed,
                platform_fail: Some(r.message.clone()),
                options: String::new(),
            }
        })
        .collect()
}

/// The entries a target runs.
fn slice(entries: Vec<Entry>, target: &Target) -> Vec<Entry> {
    let Target::Only { module, label } = target else {
        return entries;
    };
    let twin = format!("{label}_premise");
    let consistency = module.strip_prefix("book_").unwrap_or(module).to_owned();
    entries
        .into_iter()
        .filter(|e| {
            &e.module == module
                && (e.info.label == *label
                    || e.info.label == twin
                    || (module.starts_with("book_")
                        && (e.info.label == consistency || e.info.label == *module)))
        })
        .collect()
}

/// A pass's rows, or the roots Alloy refused (then no row ran).
pub(super) type Ran = (Vec<Computed>, Vec<(String, Refusal)>);

/// Run a pass: parse, key, then decide every row. `Err` only when the adapter cannot start.
pub(super) fn run(jvm: &Jvm, job: &Job<'_>) -> Result<Ran, String> {
    let mut adapter = Adapter::new(jvm, job.caps.machine)?;
    let survey = survey(&mut adapter, job)?;
    if !survey.refusals.is_empty() {
        return Ok((Vec::new(), survey.refusals));
    }
    let entries = slice(survey.entries, job.target);
    let deadline = job.caps.batch_s.map(|s| {
        Instant::now()
            .checked_add(Duration::from_secs(s))
            .unwrap_or_else(Instant::now)
    });
    let mut groups: Vec<Vec<Entry>> = Vec::new();
    for e in entries {
        match groups.last_mut() {
            Some(g) if g.first().is_some_and(|f| f.module == e.module) => g.push(e),
            _ => groups.push(vec![e]),
        }
    }
    groups.sort_by_key(|g| g.iter().map(Entry::cost).min().unwrap_or(u64::MAX));
    let mut adapters = vec![adapter];
    if job.tier == Tier::Official {
        for _ in 1..parallelism(job.caps) {
            adapters.push(Adapter::new(jvm, job.caps.machine)?);
        }
    }
    let queue = Mutex::new(groups.into_iter().rev().collect::<Vec<_>>());
    let done = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for adapter in &mut adapters {
            let (queue, done) = (&queue, &done);
            s.spawn(move || {
                while let Some(group) = queue.lock().ok().and_then(|mut q| q.pop()) {
                    let rows = run_group(adapter, job, group, deadline);
                    if let Ok(mut d) = done.lock() {
                        d.extend(rows);
                    }
                }
            });
        }
    });
    let mut rows: Vec<Computed> = done.into_inner().unwrap_or_default();
    let module_at = |m: &str| {
        job.modules
            .iter()
            .position(|(n, _)| stem(n) == m)
            .unwrap_or(usize::MAX)
    };
    rows.sort_by_key(|c| (module_at(&c.row.module), c.index));
    Ok((rows, Vec::new()))
}

/// How many adapter children the official tier can afford: RAM over each child's heap plus its
/// non-heap, at most four.
fn parallelism(caps: Caps) -> usize {
    let per = caps
        .machine
        .heap_mb
        .saturating_add(512)
        .saturating_mul(1024 * 1024);
    crate::preflight::available_ram()
        .map_or(1, |free| free.checked_div(per).unwrap_or(1))
        .clamp(1, 4)
        .try_into()
        .unwrap_or(1)
}

fn run_group(
    adapter: &mut Adapter,
    job: &Job<'_>,
    mut group: Vec<Entry>,
    deadline: Option<Instant>,
) -> Vec<Computed> {
    group.sort_by_key(|e| (e.info.label != EVERY_LINE, e.cost()));
    let members: Vec<String> = job
        .conjunctions
        .iter()
        .find(|(m, _)| group.first().is_some_and(|e| &e.module == m))
        .map(|(_, ls)| ls.clone())
        .unwrap_or_default();
    let mut conjunction_green = false;
    let mut out = Vec::new();
    for e in group {
        let entailed = conjunction_green && members.contains(&e.info.label);
        let c = decide(adapter, job, &e, entailed, deadline);
        if e.info.label == EVERY_LINE && c.row.result == Outcome::NoCounterexample {
            conjunction_green = true;
        }
        out.push(c);
    }
    out
}

fn row(e: &Entry, result: Outcome, job: &Job<'_>) -> Row {
    Row {
        module: e.module.clone(),
        name: e.info.label.clone(),
        kind: e.kind().to_owned(),
        scope: e.info.scope.clone(),
        result,
        phase: None,
        budget: Some(job.caps.budget.cpu_s),
        heap: None,
        size: None,
        key: e.key.clone(),
        platform: Some(alloy_jvm::platform().to_owned()),
        message: None,
    }
}

fn unrun(e: &Entry, result: Outcome, job: &Job<'_>, note: &str) -> Computed {
    Computed {
        row: row(e, result, job),
        provenance: Provenance::Unrun,
        wall_ms: None,
        solve_ms: None,
        note: Some(note.to_owned()),
        index: e.info.index,
    }
}

fn found(e: &Entry, sat: bool) -> Outcome {
    match (sat, e.info.check) {
        (true, true) => Outcome::Counterexample,
        (true, false) => Outcome::Sat,
        (false, true) => Outcome::NoCounterexample,
        (false, false) => Outcome::Unsat,
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "one decision per row, in the order the ledger ranks them"
)]
fn decide(
    adapter: &mut Adapter,
    job: &Job<'_>,
    e: &Entry,
    entailed: bool,
    deadline: Option<Instant>,
) -> Computed {
    if let Some(message) = &e.platform_fail {
        let mut c = unrun(e, Outcome::PlatformFail, job, message);
        c.row.message = Some(portable(message, job.out));
        return c;
    }
    if e.info.unbounded_steps {
        return unrun(
            e,
            Outcome::UnsupportedHere,
            job,
            "unbounded steps need a complete checker this jar lacks",
        );
    }
    if let Some(c) = &e.committed
        && job.tier.trusts_keys()
        && c.result.definite()
        && c.key.is_some()
        && c.key == e.key
    {
        let row = c.clone();
        return Computed {
            row,
            provenance: Provenance::Cached,
            wall_ms: None,
            solve_ms: None,
            note: None,
            index: e.info.index,
        };
    }
    if job.tier.defers()
        && e.committed.as_ref().is_some_and(|c| {
            matches!(c.result, Outcome::Timeout | Outcome::OutOfMemory)
                || c.size.is_some_and(|s| s.clauses > DEFER_CLAUSES)
        })
    {
        return unrun(
            e,
            Outcome::Deferred,
            job,
            "deferred by the hot tier: its last measurement was too large",
        );
    }
    if entailed {
        let mut c = unrun(
            e,
            Outcome::NoCounterexample,
            job,
            "entailed by the book's green conjunction",
        );
        c.provenance = Provenance::Entailed;
        c.note = None;
        return c;
    }
    if deadline.is_some_and(|d| Instant::now() >= d) {
        return unrun(
            e,
            Outcome::NotRun,
            job,
            "the batch cap was reached before this command started",
        );
    }
    let root = job
        .out
        .join(format!("{}{}", e.module, super::MODULE_SUFFIX));
    let started = Instant::now();
    let elapsed = |s: Instant| u64::try_from(s.elapsed().as_millis()).unwrap_or(u64::MAX);
    if let (Some(guard), Some((stored, xml))) =
        (&e.guard, replay::load(job.out, &e.module, &e.info.label))
        && *guard == stored
        && replay::fits(&xml, e.info.bitwidth, &e.info.scope, &e.sigs)
        && adapter.eval(&root, e.info.index, &xml, job.caps.budget) == Some(true)
    {
        return Computed {
            row: row(e, found(e, true), job),
            provenance: Provenance::Replayed,
            wall_ms: Some(elapsed(started)),
            solve_ms: None,
            note: None,
            index: e.info.index,
        };
    }
    let ask = Ask {
        index: e.info.index,
        solver: SOLVER,
        xml: true,
        text: false,
    };
    let solved = adapter.solve(&root, &[], &ask, job.caps.budget);
    let wall_ms = Some(elapsed(started));
    let mut c = Computed {
        row: row(e, Outcome::Error, job),
        provenance: Provenance::Fresh,
        wall_ms,
        solve_ms: None,
        note: None,
        index: e.info.index,
    };
    match solved {
        Solved::Found { options, .. } if options.compact() != e.options => {
            let message = format!(
                "the adapter solved under options {} where the key was computed under {}",
                options.compact(),
                e.options
            );
            c.row.message = Some(message.clone());
            c.note = Some(message);
        }
        Solved::Found {
            sat,
            solve_ms,
            size,
            xml,
            ..
        } => {
            c.row.result = found(e, sat);
            c.row.size = size;
            c.solve_ms = Some(solve_ms);
            match (&xml, &e.guard) {
                (Some(xml), Some(guard)) => {
                    replay::store(job.out, &e.module, &e.info.label, guard, xml);
                }
                _ => replay::forget(job.out, &e.module, &e.info.label),
            }
        }
        Solved::Timeout {
            exceeded,
            phase,
            size,
            translated_ms,
        } => {
            c.row.result = Outcome::Timeout;
            c.row.phase = Some(phase.name().to_owned());
            c.row.size = size;
            c.row.heap = Some(job.caps.machine.heap_mb);
            c.note = Some(format!(
                "exceeded {}{}",
                match exceeded {
                    alloy_jvm::adapter::Exceeded::Cpu(s) => format!("{s}s cpu"),
                    alloy_jvm::adapter::Exceeded::Wall(s) => format!("{s}s wall-clock"),
                },
                translated_ms.map_or_else(
                    || "; still translating".to_owned(),
                    |ms| format!("; translated in {ms}ms")
                ),
            ));
        }
        Solved::OutOfMemory(message) => {
            c.row.result = Outcome::OutOfMemory;
            c.row.heap = Some(job.caps.machine.heap_mb);
            c.note = Some(message);
        }
        Solved::Error(message) => {
            c.row.message = Some(portable(&message, job.out));
            c.note = Some(message);
        }
    }
    c
}

/// An Alloy message with the out directory's own path taken out, so a lock row reads the same in
/// every worktree.
pub(super) fn portable(message: &str, out: &Path) -> String {
    let slashed = |s: &str| s.replace('\\', "/");
    let dir = format!(
        "{}/",
        slashed(&out.display().to_string()).trim_end_matches('/')
    );
    slashed(message).replace(&dir, "")
}
