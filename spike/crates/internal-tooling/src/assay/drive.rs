use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use super::book::EVERY_LINE;
use super::emit::Rendered;
use super::key::{self, Fixed};
use super::lock::{Outcome, Row};
use super::replay;
use super::tier::{Caps, DEFER_CLAUSES, Tier};
use crate::alloy_jvm::adapter::{
    Adapter, Ask, Budget, CommandInfo, Parsed, Refusal, Solved, Warning, human,
};
use crate::alloy_jvm::{self, Jvm};

const SOLVER: &str = "sat4j";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Target {
    All,
    Module(String),
    Only { module: String, label: String },
}

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
    pub(super) note: Option<String>,
    pub(super) index: u64,
    /// The command's `expect`, reported beside the row and never written to the lock.
    pub(super) expects: Option<u64>,
}

#[derive(Debug)]
pub(super) struct Job<'a> {
    pub(super) out: &'a Path,
    pub(super) modules: &'a [(String, Rendered)],
    pub(super) conjunctions: &'a [(String, Vec<String>)],
    pub(super) committed: &'a [Row],
    pub(super) tier: Tier,
    pub(super) caps: Caps,
    pub(super) target: &'a Target,
    pub(super) stem: &'a str,
}

/// Rows answered so far across every child, for the `[n/total]` on each solve's end line.
#[derive(Debug)]
struct Tally {
    finished: AtomicUsize,
    total: usize,
}

#[derive(Debug, Clone)]
pub(super) struct Entry {
    pub(super) module: String,
    pub(super) info: CommandInfo,
    pub(super) key: Option<String>,
    guard: Option<String>,
    sigs: Vec<(String, Vec<String>)>,
    committed: Option<Row>,
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

#[derive(Debug)]
pub(super) struct Survey {
    pub(super) entries: Vec<Entry>,
    pub(super) stops: Stops,
}

/// What stops a pass before it solves: each module Alloy refused, and each warning it raised on
/// a module it accepted, keyed by the module parsed as root.
#[derive(Debug, Default)]
pub(super) struct Stops {
    pub(super) refusals: Vec<(String, Refusal)>,
    pub(super) warnings: Vec<(String, Warning)>,
}

impl Stops {
    pub(super) fn is_empty(&self) -> bool {
        self.refusals.is_empty() && self.warnings.is_empty()
    }
}

fn stem(name: &str) -> &str {
    name.strip_suffix(super::MODULE_SUFFIX).unwrap_or(name)
}

fn wanted(target: &Target, module: &str) -> bool {
    match target {
        Target::All => true,
        Target::Module(m) | Target::Only { module: m, .. } => m == module,
    }
}

pub(super) fn survey(adapter: &mut Adapter, job: &Job<'_>) -> Result<Survey, String> {
    let fixed_base = (alloy_jvm::jar_digest()?, alloy_jvm::adapter_digest()?);
    let mut entries = Vec::new();
    let mut stops = Stops::default();
    for (name, rendered) in job.modules {
        let module = stem(name).to_owned();
        if !wanted(job.target, &module) {
            continue;
        }
        let root = job.out.join(name);
        match adapter.parse(&root, &[], SOLVER) {
            Ok(parsed) => {
                entries.extend(keyed(
                    &module,
                    rendered,
                    &parsed,
                    &fixed_base,
                    job.committed,
                ));
                stops
                    .warnings
                    .extend(parsed.warnings.into_iter().map(|w| (name.clone(), w)));
            }
            Err(r) if r.file.as_deref().is_some_and(|f| f.contains("$alloy4$")) => {
                entries.extend(platform_failed(&module, rendered, &r, job.committed));
            }
            Err(r) => stops.refusals.push((name.clone(), r)),
        }
    }
    Ok(Survey { entries, stops })
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

pub(super) type Ran = (Vec<Computed>, Stops);

pub(super) fn run(jvm: &Jvm, job: &Job<'_>) -> Result<Ran, String> {
    let mut adapter = Adapter::new(jvm, job.caps.machine)?;
    let survey = survey(&mut adapter, job)?;
    if !survey.stops.is_empty() {
        return Ok((Vec::new(), survey.stops));
    }
    let entries = slice(survey.entries, job.target);
    let tally = Tally {
        finished: AtomicUsize::new(0),
        total: entries.len(),
    };
    let capped_before = entries
        .iter()
        .filter(|e| {
            e.committed
                .as_ref()
                .is_some_and(|c| matches!(c.result, Outcome::Timeout | Outcome::OutOfMemory))
        })
        .count();
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
    tracing::info!(
        "{}",
        plan_line(
            job,
            &Plan {
                commands: tally.total,
                modules: groups.len(),
                children: adapters.len(),
                capped_before,
            },
        )
    );
    let queue = Mutex::new(groups.into_iter().rev().collect::<Vec<_>>());
    let done = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for adapter in &mut adapters {
            let (queue, done, tally) = (&queue, &done, &tally);
            s.spawn(move || {
                while let Some(group) = queue.lock().ok().and_then(|mut q| q.pop()) {
                    let rows = run_group(adapter, job, group, deadline, tally);
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
    Ok((rows, Stops::default()))
}

#[derive(Debug, Clone, Copy)]
struct Plan {
    commands: usize,
    modules: usize,
    children: usize,
    capped_before: usize,
}

fn plan_line(job: &Job<'_>, plan: &Plan) -> String {
    let batch = job
        .caps
        .batch_s
        .map_or_else(|| "no batch cap".to_owned(), |s| format!("batch {s}s"));
    format!(
        "{} tier, {} commands in {} modules, {} children, cpu {}s wall {}s per command, {batch}, {} last recorded timeout or out-of-memory",
        job.tier.name(),
        plan.commands,
        plan.modules,
        plan.children,
        job.caps.budget.cpu_s,
        job.caps.budget.wall_s,
        plan.capped_before,
    )
}

fn solve_starts_line(budget: Budget, committed: Option<&Row>) -> String {
    let last = committed.map_or_else(String::new, |c| {
        let size = c
            .size
            .map_or_else(String::new, |s| format!(" at {} clauses", s.clauses));
        format!(", last {}{size}", c.result.name())
    });
    format!(
        "solving, cpu {}s wall {}s{last}",
        budget.cpu_s, budget.wall_s
    )
}

fn solve_ends_line(c: &Computed, finished: usize, total: usize) -> String {
    let phase = c
        .row
        .phase
        .as_ref()
        .map_or_else(String::new, |p| format!(" while {p}"));
    let wall = c.wall_ms.map_or_else(String::new, |ms| {
        format!(" in {}", human(Duration::from_millis(ms)))
    });
    format!("{}{phase}{wall} [{finished}/{total}]", c.row.result.name())
}

/// Rows by provenance, then by result: the pass-end line's body.
pub(super) fn row_counts(rows: &[Computed]) -> String {
    let by_provenance: Vec<String> = [
        Provenance::Fresh,
        Provenance::Cached,
        Provenance::Entailed,
        Provenance::Replayed,
        Provenance::Unrun,
    ]
    .into_iter()
    .filter_map(|p| {
        let n = rows.iter().filter(|c| c.provenance == p).count();
        (n > 0).then(|| format!("{} {n}", p.name()))
    })
    .collect();
    let mut by_result: BTreeMap<&str, usize> = BTreeMap::new();
    for c in rows {
        let n = by_result.entry(c.row.result.name()).or_default();
        *n = n.saturating_add(1);
    }
    let by_result: Vec<String> = by_result
        .into_iter()
        .map(|(r, n)| format!("{r} {n}"))
        .collect();
    format!("{} | {}", by_provenance.join(", "), by_result.join(", "))
}

fn parallelism(caps: Caps) -> usize {
    let per = crate::preflight::jvm_ram(caps.machine.heap_mb);
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
    tally: &Tally,
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
        let entailed =
            job.tier != Tier::Official && conjunction_green && members.contains(&e.info.label);
        // Concurrent children interleave by line, so every line inside names document and command;
        // a row answered without a solve emits nothing, so its span prints nothing.
        let _span = tracing::info_span!(
            "solve",
            stem = %job.stem,
            module = %e.module,
            label = %e.info.label
        )
        .entered();
        let c = decide(adapter, job, &e, entailed, deadline);
        if e.info.label == EVERY_LINE && c.row.result == Outcome::NoCounterexample {
            conjunction_green = true;
        }
        let finished = tally
            .finished
            .fetch_add(1, Ordering::Relaxed)
            .saturating_add(1);
        if c.provenance == Provenance::Fresh {
            tracing::info!("{}", solve_ends_line(&c, finished, tally.total));
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
        expects: e.info.expects,
    }
}

/// Whether the hot tier defers a row, named by the recorded measurement that decided it.
fn deferral(c: &Row) -> Option<String> {
    match c.result {
        Outcome::Timeout => Some(format!(
            "deferred: last timeout at {}",
            c.budget
                .map_or_else(|| "an unrecorded budget".to_owned(), |b| format!("{b}s"))
        )),
        Outcome::OutOfMemory => Some(format!(
            "deferred: last out-of-memory at {}",
            c.heap
                .map_or_else(|| "an unrecorded heap".to_owned(), |h| format!("{h} MB"))
        )),
        _ => c.size.filter(|s| s.clauses > DEFER_CLAUSES).map(|s| {
            format!(
                "deferred: {} clauses over the {DEFER_CLAUSES} threshold",
                s.clauses
            )
        }),
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
            expects: e.info.expects,
        };
    }
    if job.tier.defers()
        && let Some(why) = e.committed.as_ref().and_then(deferral)
    {
        return unrun(e, Outcome::Deferred, job, &why);
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
    let mut budget = job.caps.budget;
    if let Some(d) = deadline {
        let left = d.saturating_duration_since(Instant::now()).as_secs();
        if left == 0 {
            return unrun(
                e,
                Outcome::NotRun,
                job,
                "the batch cap was reached before this command started",
            );
        }
        budget.wall_s = budget.wall_s.min(left);
    }
    let clipped = budget != job.caps.budget;
    let root = job
        .out
        .join(format!("{}{}", e.module, super::MODULE_SUFFIX));
    let started = Instant::now();
    let elapsed = |s: Instant| u64::try_from(s.elapsed().as_millis()).unwrap_or(u64::MAX);
    if let (Some(guard), Some((stored, xml))) =
        (&e.guard, replay::load(job.out, &e.module, &e.info.label))
        && job.tier.trusts_keys()
        && *guard == stored
        && replay::fits(&xml, e.info.bitwidth, &e.info.scope, &e.sigs)
        && adapter.eval(&root, e.info.index, &xml, budget) == Some(true)
    {
        return Computed {
            row: row(e, found(e, true), job),
            provenance: Provenance::Replayed,
            wall_ms: Some(elapsed(started)),
            solve_ms: None,
            note: None,
            index: e.info.index,
            expects: e.info.expects,
        };
    }
    let ask = Ask {
        index: e.info.index,
        solver: SOLVER,
        xml: true,
        text: false,
    };
    tracing::info!("{}", solve_starts_line(budget, e.committed.as_ref()));
    let solved = adapter.solve(&root, &[], &ask, budget);
    let wall_ms = Some(elapsed(started));
    let mut c = Computed {
        row: row(e, Outcome::Error, job),
        provenance: Provenance::Fresh,
        wall_ms,
        solve_ms: None,
        note: None,
        index: e.info.index,
        expects: e.info.expects,
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
            if clipped {
                c.row.budget = Some(budget.cpu_s.min(budget.wall_s));
            }
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

pub(super) fn portable(message: &str, out: &Path) -> String {
    let slashed = |s: &str| s.replace('\\', "/");
    let dir = format!(
        "{}/",
        slashed(&out.display().to_string()).trim_end_matches('/')
    );
    slashed(message).replace(&dir, "")
}

#[cfg(test)]
mod tests {
    use super::deferral;
    use crate::alloy_jvm::adapter::Size;
    use crate::assay::lock::{Outcome, Row};

    fn row(result: Outcome, clauses: Option<u64>) -> Row {
        Row {
            module: "laws".to_owned(),
            name: "k".to_owned(),
            kind: "check".to_owned(),
            scope: String::new(),
            result,
            phase: None,
            budget: Some(1800),
            heap: Some(4096),
            size: clauses.map(|clauses| Size {
                primary_vars: 1,
                vars: 1,
                clauses,
            }),
            key: None,
            platform: None,
            message: None,
        }
    }

    #[test]
    fn a_deferral_names_the_measurement_that_decided_it() {
        // The hot tier's decision moved into this function when its note learned to name its
        // cause, so the decision is pinned here along with the cause.
        assert_eq!(
            deferral(&row(Outcome::Timeout, None)).as_deref(),
            Some("deferred: last timeout at 1800s")
        );
        assert_eq!(
            deferral(&row(Outcome::OutOfMemory, None)).as_deref(),
            Some("deferred: last out-of-memory at 4096 MB")
        );
        assert_eq!(
            deferral(&row(Outcome::NoCounterexample, Some(2_381_046))).as_deref(),
            Some("deferred: 2381046 clauses over the 2000000 threshold")
        );
        assert_eq!(
            deferral(&row(Outcome::NoCounterexample, Some(2_000_000))),
            None
        );
        assert_eq!(deferral(&row(Outcome::Sat, None)), None);
    }
}
