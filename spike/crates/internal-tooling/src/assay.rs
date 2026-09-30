//! `assay`: compile a literate spec document into one flat directory of Alloy modules
//! (`notes/30Y` § 2), and on request parse them, run them through the Alloy adapter, and
//! hold the verdicts in a lock beside the spec (`30Y` § 2.7). Markdown in, `.als` out, one JSON
//! report per document: on stdout, or for a solving pass in a file its stdout summary names.

mod alloy;
mod book;
mod commit;
mod drive;
mod emit;
mod key;
mod lock;
mod pass;
mod progress;
mod replay;
mod report;
mod sh;
mod tier;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::json::{Json, Value};
use alloy::{Head, Item};
use emit::ORIGIN;
use sh::{Command, Component, Fence};

/// The harness every generated module opens (`30Y` § 2.2).
const HARNESS: &str = "module assay

sig Shword { class: set Class }
sig Class {}
abstract sig Claim {}
sig Line { above: set Line, speech: set Claim, cmd: one Shword, argv: seq Shword }
";

/// The corpus book's null command, a word of assay's own (`30Y` § 2.3).
const NULL_WORD: &str = "assay_colon";
const MODULE_SUFFIX: &str = ".als";
const LINE_MAP: &str = "assay-map.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Lint {
    JoinKeyCoherence,
    MapWordCount,
    MapLineFollowsCommand,
    LoadStemResolves,
    FenceHeader,
    NoStrayComments,
    ThisOnAtomlessLine,
    ParentIsAKnownSig,
    UnscopedRunIsAPremiseTwin,
    LabelIsUniqueInModule,
    TokenizerIsCertain,
    AlloyParses,
}

impl Lint {
    const ALL: [Self; 12] = [
        Self::JoinKeyCoherence,
        Self::MapWordCount,
        Self::MapLineFollowsCommand,
        Self::LoadStemResolves,
        Self::FenceHeader,
        Self::NoStrayComments,
        Self::ThisOnAtomlessLine,
        Self::ParentIsAKnownSig,
        Self::UnscopedRunIsAPremiseTwin,
        Self::LabelIsUniqueInModule,
        Self::TokenizerIsCertain,
        Self::AlloyParses,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::JoinKeyCoherence => "join-key-coherence",
            Self::MapWordCount => "map-word-count",
            Self::MapLineFollowsCommand => "map-line-follows-command",
            Self::LoadStemResolves => "load-stem-resolves",
            Self::FenceHeader => "fence-header",
            Self::NoStrayComments => "no-stray-comments",
            Self::ThisOnAtomlessLine => "this-on-atomless-line",
            Self::ParentIsAKnownSig => "parent-is-a-known-sig",
            Self::UnscopedRunIsAPremiseTwin => "unscoped-run-is-a-premise-twin",
            Self::LabelIsUniqueInModule => "label-is-unique-in-module",
            Self::TokenizerIsCertain => "tokenizer-is-certain",
            Self::AlloyParses => "alloy-parses",
        }
    }
}

/// One lint refusal: where, and the offending text as data, never prose.
#[derive(Debug, Clone)]
pub(crate) struct Finding {
    lint: Lint,
    file: String,
    line: usize,
    fields: Vec<(&'static str, String)>,
}

impl Finding {
    fn new(kind: Lint, file: &str, line: usize) -> Self {
        Self {
            lint: kind,
            file: file.to_owned(),
            line,
            fields: Vec::new(),
        }
    }

    fn with(mut self, key: &'static str, value: impl Into<String>) -> Self {
        self.fields.push((key, value.into()));
        self
    }
}

/// A source file assay reads: its name for line comments, and its text.
#[derive(Debug, Clone, Copy)]
struct Source<'a> {
    name: &'a str,
    text: &'a str,
}

/// The document and the two shared halves found beside it (`30Y` § 2.2).
#[derive(Debug, Clone, Copy)]
struct Inputs<'a> {
    doc: Source<'a>,
    shared: Option<Source<'a>>,
    laws: Option<Source<'a>>,
}

#[derive(Debug)]
struct Compiled {
    modules: Vec<(String, emit::Rendered)>,
    report: Vec<(&'static str, Json)>,
    conjunctions: Vec<(String, Vec<String>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Mode {
    Compile,
    /// The Alloy-parse lint alone (`30Y` § 2.6, fourth item).
    Parse,
    Staged,
    Check,
    Write,
}

/// Exit codes (`30Y` § 2.7); the worst over several documents wins.
const RED: u8 = 1;
const REFUSED: u8 = 2;
const RUNNER_FAILED: u8 = 3;
const UNMEASURED: u8 = 4;
const CONTENTION: u8 = 75;

fn severity(code: u8) -> u8 {
    match code {
        0 => 0,
        UNMEASURED => 1,
        RED => 2,
        RUNNER_FAILED => 3,
        CONTENTION => 4,
        _ => 5,
    }
}

#[derive(Debug, Clone)]
struct Ask {
    mode: Mode,
    /// `--check` given with `--write`: the lock is written, and the exit is the check's verdict.
    checked: bool,
    tier: tier::Tier,
    target: drive::Target,
    caps: Vec<String>,
    report: report::Sink,
}

/// Assay's own arguments, and the runner caps after its `--`.
fn split_caps(args: &[String]) -> (&[String], &[String]) {
    match args.iter().position(|a| a == "--") {
        Some(at) => {
            let (ours, rest) = args.split_at(at);
            (ours, rest.get(1..).unwrap_or_default())
        }
        None => (args, &[][..]),
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "the command line, parsed and then validated top to bottom"
)]
pub(crate) fn run(args: &[String]) -> ExitCode {
    let (ours, caps) = split_caps(args);
    let mut specs = Vec::new();
    let mut out = None;
    let mut ask = Ask {
        mode: Mode::Compile,
        checked: false,
        tier: tier::Tier::Gate,
        target: drive::Target::All,
        caps: caps.to_vec(),
        report: report::Sink::Stdout,
    };
    let mut json = None;
    let mut quiet = false;
    let mut it = ours.iter();
    while let Some(arg) = it.next() {
        let wanted = match arg.as_str() {
            "--out" => {
                out = it.next().map(|d| resolve(d));
                continue;
            }
            "--quiet" => {
                quiet = true;
                continue;
            }
            "--json" => {
                json = Some(it.next());
                continue;
            }
            "--help" | "-h" => {
                print!("{HELP}");
                return ExitCode::SUCCESS;
            }
            "--hot" | "--gate" | "--official" => {
                ask.tier = match arg.as_str() {
                    "--hot" => tier::Tier::Hot,
                    "--official" => tier::Tier::Official,
                    _ => tier::Tier::Gate,
                };
                continue;
            }
            "--module" | "--only" => {
                let Some(name) = it.next() else {
                    return usage(&format!("{arg} names a module or command"));
                };
                ask.target = match (arg.as_str(), name.split_once('.')) {
                    ("--module", _) => drive::Target::Module(name.clone()),
                    (_, Some((module, label))) => drive::Target::Only {
                        module: module.to_owned(),
                        label: label.to_owned(),
                    },
                    (_, None) => drive::Target::Only {
                        module: String::new(),
                        label: name.clone(),
                    },
                };
                continue;
            }
            "--parse" => Mode::Parse,
            "--staged" => Mode::Staged,
            "--check" => Mode::Check,
            "--write" => Mode::Write,
            flag if flag.starts_with('-') => return usage(&format!("unknown option {flag}")),
            path => {
                specs.push(resolve(path));
                continue;
            }
        };
        if ask.mode != Mode::Compile {
            // The one pair: a single solve that both judges the lock and yields the one that
            // would pass, which is what an expensive run hands back when it fails.
            if !matches!(
                (&ask.mode, &wanted),
                (Mode::Check, Mode::Write) | (Mode::Write, Mode::Check)
            ) {
                return usage(
                    "--parse, --staged, --check, and --write exclude each other, bar --check --write",
                );
            }
            ask.mode = Mode::Write;
            ask.checked = true;
            continue;
        }
        ask.mode = wanted;
    }
    if ask.target != drive::Target::All {
        match ask.mode {
            Mode::Write => return usage("a targeted run never writes the lock"),
            Mode::Compile => ask.mode = Mode::Check,
            Mode::Parse | Mode::Staged | Mode::Check => {}
        }
    }
    let specs = documents(&specs);
    if specs.is_empty() {
        return usage("no spec document named");
    }
    if let Some(other) = specs
        .iter()
        .find(|p| !p.to_string_lossy().ends_with(DOC_SUFFIX))
    {
        return usage(&format!(
            "{} is not a {DOC_SUFFIX} document",
            other.display()
        ));
    }
    if out.is_some() && specs.len() > 1 {
        return usage("--out names one document's directory, and more than one was named");
    }
    let solving = matches!(ask.mode, Mode::Check | Mode::Write);
    ask.report = match json {
        Some(_) if !solving => return usage("--json is where --check or --write reports"),
        Some(None) => return usage("--json names a file, or - for stdout"),
        Some(Some(to)) => report::Sink::named(to, specs.len()),
        None => report::Sink::Dir(report::default_dir()),
    };
    if solving {
        progress::install(quiet);
    }
    let worst = specs
        .iter()
        .map(|spec| pass::one(spec, out.clone(), &ask))
        .max_by_key(|code| severity(*code))
        .unwrap_or(0);
    ExitCode::from(worst)
}

/// The documents a list of paths stands for: a shared half stands for every document beside it,
/// and a lock for its document, so a gate handed only those still checks what they change.
fn documents(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = BTreeSet::new();
    for path in paths {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if is_shared_half(&name) {
            let siblings = path
                .parent()
                .and_then(|dir| std::fs::read_dir(dir).ok())
                .into_iter()
                .flatten()
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.file_name().is_some_and(|n| {
                        let n = n.to_string_lossy();
                        n.ends_with(DOC_SUFFIX) && !is_shared_half(&n)
                    })
                });
            out.extend(siblings);
        } else if let Some(stem) = name.strip_suffix(LOCK_SUFFIX) {
            out.insert(path.with_file_name(format!("{stem}{DOC_SUFFIX}")));
        } else {
            out.insert(path.clone());
        }
    }
    out.into_iter().collect()
}

fn is_shared_half(name: &str) -> bool {
    matches!(name, SHARED_HALF | LAWS_HALF)
}

const LOCK_SUFFIX: &str = ".lock.json";
/// The spelling of every document in the `30Z` format. A document's stem is its name minus this
/// suffix, and names its lock (`<stem>.lock.json`) and its out directory (`<stem>/`).
const DOC_SUFFIX: &str = ".assay.md";
/// The two shared halves, found by these names beside a document (`30Y` § 2.2).
const SHARED_HALF: &str = "shared.assay.md";
const LAWS_HALF: &str = "shared-laws.assay.md";

const HELP: &str = "usage: assay <spec.assay.md>... [flags] [-- <caps>]
  --out <dir>                  write the modules there (one document only)
  --parse                      Alloy's parse lint, no solver, no heavy-work lock
  --staged                     the pre-commit form: staged bytes, parse and key diff, never solves
  --check                      solve and compare against <stem>.lock.json
  --write                      solve and rewrite <stem>.lock.json
  --check --write              both: rewrite the lock, and exit with --check's verdict
  --hot                       tier: 120s CPU per command, deferral, 540s batch
  --gate                       tier (default): 600s CPU per command, 540s batch
  --official                   tier: 1800s CPU per command, 4096 MB, keys ignored, 8h batch
  --module <m>                 solve every command of one module; never writes
  --only <[module.]command>    solve one command, its premise twin, and its book's run; never writes
  --json <path|->              where --check or --write puts its JSON report, - for stdout; a
                               directory when several documents are named; default
                               .tmp/assay/<stem>-<UTC time>.json under the repository
  --quiet                      no progress lines on stderr
  --help                       this text
caps, after a second --:
  --cpu <s>                    CPU per command; alone, the wall cap becomes twice it
  --timeout <s>                wall-clock per command; alone, the CPU cap becomes it too
  --heap <MB>                  each child JVM's heap
  --procs <n>                  processors each child JVM may use
  --batch-timeout <s>          wall-clock for the whole pass; later commands get what is left, or none
";

fn usage(problem: &str) -> ExitCode {
    eprintln!(
        "assay: {problem}
usage: assay <spec.assay.md>... [--out <dir>] [--parse | --staged | --check | --write | --check --write] [--hot | --gate | --official] [--module <m>] [--only <[module.]command>] [--json <path|->] [--quiet] [-- <runner caps>]"
    );
    ExitCode::from(REFUSED)
}

/// Relative paths name what the caller meant from where they ran mise, not the task's own dir.
fn resolve(arg: &str) -> PathBuf {
    let path = PathBuf::from(arg);
    match std::env::var_os("MISE_ORIGINAL_CWD") {
        Some(cwd) if path.is_relative() => PathBuf::from(cwd).join(path),
        _ => path,
    }
}

/// Repo-relative with `/` where the spec lives in the repo, so generated bytes are portable
/// (`spike/verify/AGENTS.md` `rul-published-paths-are-repo-relative`).
fn display_path(spec: &Path) -> String {
    let canon = |p: &Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let spec = canon(spec);
    spec.strip_prefix(canon(dorc_testbed::repo_root()))
        .map_or_else(
            |_| spec.display().to_string(),
            |rel| {
                let parts: Vec<String> = rel
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect();
                parts.join("/")
            },
        )
}

fn write_modules(out: &Path, modules: &[(String, emit::Rendered)]) -> std::io::Result<Vec<String>> {
    std::fs::create_dir_all(out)?;
    let fresh: BTreeSet<&str> = modules.iter().map(|(name, _)| name.as_str()).collect();
    let sidecar = out.join(LINE_MAP);
    let previous = std::fs::read_to_string(&sidecar)
        .ok()
        .and_then(|t| Value::parse(&t))
        .map(|v| match v.get("modules") {
            Some(Value::Obj(fields)) => fields.iter().map(|(k, _)| k.clone()).collect(),
            _ => Vec::new(),
        })
        .unwrap_or_default();
    for stale in previous.iter().filter(|p| !fresh.contains(p.as_str())) {
        let path = out.join(stale);
        if stale.ends_with(MODULE_SUFFIX) && path.is_file() {
            std::fs::remove_file(&path)?;
        }
    }
    let entries: Vec<String> = modules
        .iter()
        .map(|(name, r)| {
            let lines: Vec<String> = r
                .map_json_lines()
                .iter()
                .map(|l| format!("    {l}"))
                .collect();
            format!(
                "  {}: [\n{}\n  ]",
                crate::json::quote(name),
                lines.join(",\n")
            )
        })
        .collect();
    std::fs::write(
        &sidecar,
        format!("{{\"modules\": {{\n{}\n}}}}\n", entries.join(",\n")),
    )?;
    modules
        .iter()
        .map(|(name, rendered)| {
            let path = out.join(name);
            std::fs::write(&path, &rendered.text)?;
            Ok(path.display().to_string())
        })
        .collect()
}

fn num(n: usize) -> Json {
    Json::Num(u64::try_from(n).unwrap_or(u64::MAX))
}

fn lints_json(findings: &[Finding]) -> Json {
    Json::Obj(
        Lint::ALL
            .iter()
            .map(|lint| {
                let rows = findings.iter().filter(|f| f.lint == *lint).map(|f| {
                    let mut row = vec![
                        ("file".to_owned(), Json::str(&f.file)),
                        ("line".to_owned(), num(f.line)),
                    ];
                    row.extend(
                        f.fields
                            .iter()
                            .map(|(k, v)| ((*k).to_owned(), Json::str(v))),
                    );
                    Json::Obj(row)
                });
                (lint.name().to_owned(), Json::Arr(rows.collect()))
            })
            .collect(),
    )
}

/// A Markdown fence assay reads.
#[derive(Debug)]
struct MdFence<'a> {
    alloy: bool,
    first_line: usize,
    lines: Vec<&'a str>,
}

/// Every ```` ```alloy ```` and ```` ```sh ```` fence; every other byte of Markdown is ignored.
fn md_fences(text: &str) -> Vec<MdFence<'_>> {
    let mut out = Vec::new();
    let mut open: Option<(char, usize, Option<MdFence<'_>>)> = None;
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        let marker = trimmed.chars().next().filter(|c| matches!(c, '`' | '~'));
        let run = marker.map_or(0, |m| trimmed.chars().take_while(|&c| c == m).count());
        match &mut open {
            Some((m, len, fence)) => {
                if marker == Some(*m)
                    && run >= *len
                    && trimmed.chars().skip(run).all(char::is_whitespace)
                {
                    out.extend(fence.take());
                    open = None;
                } else if let Some(fence) = fence {
                    fence.lines.push(line);
                }
            }
            None if run >= 3 && line.len().saturating_sub(trimmed.len()) <= 3 => {
                let info = trimmed.get(run..).unwrap_or("").trim();
                let fence = matches!(info, "alloy" | "sh").then(|| MdFence {
                    alloy: info == "alloy",
                    first_line: i.saturating_add(2),
                    lines: Vec::new(),
                });
                open = marker.map(|m| (m, run, fence));
            }
            None => {}
        }
    }
    out.extend(open.and_then(|(_, _, fence)| fence));
    out
}

fn alloy_items(src: Source<'_>) -> Vec<(Item, &str)> {
    md_fences(src.text)
        .into_iter()
        .filter(|f| f.alloy)
        .flat_map(|f| alloy::items(&f.lines.join("\n"), f.first_line))
        .map(|item| (item, src.name))
        .collect()
}

/// One `#=` statement, sorted by what it is (`30Y` § 2.1).
#[derive(Debug)]
enum Stmt {
    Declaration {
        names: Vec<String>,
        claim: bool,
        text: String,
    },
    Outcome {
        formula: String,
        scope: Option<String>,
    },
    Fact(String),
}

fn head_of(text: &str) -> Head {
    alloy::items(text, 0)
        .first()
        .map_or(Head::Other, Item::head)
}

/// One concrete line of a book: the command, the word each position names, its statements.
#[derive(Debug)]
struct Row<'a> {
    cmd: &'a Command,
    names: Vec<String>,
    stmts: Vec<(usize, Stmt)>,
}

impl Row<'_> {
    fn is_atom(&self) -> bool {
        self.stmts
            .iter()
            .any(|(_, s)| matches!(s, Stmt::Outcome { .. }))
    }
}

#[derive(Debug)]
struct Book<'a> {
    name: &'a str,
    line: usize,
    loads: Vec<String>,
    rows: Vec<Row<'a>>,
}

/// What every book module shares: the document's name, the words, and the default scope.
#[derive(Debug)]
struct Ctx<'a> {
    file: &'a str,
    default_scope: Option<&'a str>,
    words: usize,
    classes: usize,
    doc_claims: usize,
}

/// Compile, or refuse with every lint finding at once.
#[expect(
    clippy::too_many_lines,
    reason = "one pass over the document, read top to bottom in the order `30Y` § 2 states it"
)]
fn compile(inputs: &Inputs<'_>) -> Result<Compiled, Vec<Finding>> {
    let mut findings = Vec::new();
    let file = inputs.doc.name;
    let harness: Vec<(Item, &str)> = alloy::items(HARNESS, 1)
        .into_iter()
        .map(|i| (i, "assay"))
        .collect();
    let shared = inputs.shared.map(alloy_items).unwrap_or_default();
    let appended = inputs.laws.map(alloy_items).unwrap_or_default();
    let doc_items = alloy_items(inputs.doc);
    let fences: Vec<Fence> = md_fences(inputs.doc.text)
        .into_iter()
        .filter(|f| !f.alloy)
        .filter_map(|f| sh::read(file, f.first_line, &f.lines, &mut findings))
        .collect();

    // Every declaration and sig head: the harness, both halves, the document, and book lines.
    let mut declared: BTreeSet<String> = BTreeSet::new();
    let mut heads: Vec<(Head, String, usize)> = Vec::new();
    for (item, src) in harness
        .iter()
        .chain(&shared)
        .chain(&appended)
        .chain(&doc_items)
    {
        declared.extend(item.declared());
        if let head @ Head::Sig { .. } = item.head() {
            heads.push((head, (*src).to_owned(), item.line));
        }
    }
    let mut load_files: BTreeMap<&str, &[(String, usize)]> = BTreeMap::new();
    let mut books_raw = Vec::new();
    for fence in &fences {
        match fence {
            Fence::Load { name, stems, .. } => {
                load_files.insert(name.as_str(), stems.as_slice());
                declared.insert(name.clone());
            }
            Fence::Book {
                name,
                line,
                loads,
                commands,
            } => books_raw.push((name.as_str(), *line, loads, commands)),
        }
    }
    for (_, _, _, commands) in &books_raw {
        for (line, text) in commands.iter().flat_map(|c| &c.stmts) {
            for item in alloy::items(text, *line) {
                declared.extend(item.declared());
                if let head @ Head::Sig { .. } = item.head() {
                    heads.push((head, file.to_owned(), *line));
                }
            }
        }
    }
    let mut parents: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (head, _, _) in &heads {
        if let Head::Sig {
            names, parents: ps, ..
        } = head
        {
            for name in names {
                parents
                    .entry(name.clone())
                    .or_default()
                    .extend(ps.iter().cloned());
            }
        }
    }
    for (head, src, line) in &heads {
        if let Head::Sig { parents: ps, .. } = head {
            for parent in ps.iter().filter(|p| {
                !parents.contains_key(*p) && !matches!(p.as_str(), "univ" | "Int" | "String")
            }) {
                findings
                    .push(Finding::new(Lint::ParentIsAKnownSig, src, *line).with("parent", parent));
            }
        }
    }
    let is_claim = |head: &Head| match head {
        Head::Sig {
            one,
            extends,
            parents: ps,
            ..
        } => *one && *extends && ps.iter().any(|p| reaches_claim(p, &parents)),
        Head::Command { .. } | Head::Other => false,
    };

    // Claim atoms, scope carriers, and the module every other declaration lands in (`30Y` § 2.3).
    let mut carriers: BTreeMap<String, String> = BTreeMap::new();
    let mut shared_blocks = Vec::new();
    for (item, src) in &shared {
        match item.head() {
            Head::Command {
                check: false,
                name: Some(name),
                scope,
                ..
            } if is_carrier(&name) => {
                carriers.insert(name, scope.unwrap_or_default());
            }
            _ => shared_blocks.push(block(src, item.line, &item.text)),
        }
    }
    let check_scopes: BTreeMap<String, Option<String>> = doc_items
        .iter()
        .filter_map(|(item, _)| match item.head() {
            Head::Command {
                check: true,
                name: Some(name),
                scope,
                ..
            } => Some((name, scope)),
            _ => None,
        })
        .collect();
    let mut claim_items = Vec::new();
    let mut species = Vec::new();
    let mut laws = Vec::new();
    let mut corpus: Vec<(usize, String, Option<String>)> = Vec::new();
    for (item, _) in &doc_items {
        let head = item.head();
        if is_claim(&head) {
            claim_items.push(item);
            continue;
        }
        let Head::Command {
            check,
            name,
            scope,
            body,
        } = head
        else {
            species.push(block(file, item.line, &item.text));
            continue;
        };
        let name = name.unwrap_or_default();
        if !check && is_carrier(&name) {
            carriers.insert(name, scope.unwrap_or_default());
            continue;
        }
        // A premise twin follows its check; `None` inside means that check is a corpus check.
        let premise_of = name
            .strip_suffix("_premise")
            .filter(|_| !check)
            .and_then(|stem| check_scopes.get(stem));
        match (premise_of, &scope) {
            (None, None) if !check => findings.push(
                Finding::new(Lint::UnscopedRunIsAPremiseTwin, file, item.line).with("run", &name),
            ),
            (Some(None), _) | (None, None) => corpus.push((item.line, body, scope)),
            (Some(Some(inherited)), None) => {
                laws.push(block(file, item.line, &format!("{body} for {inherited}")));
            }
            (_, Some(_)) => laws.push(block(file, item.line, &item.text)),
        }
    }
    let claim_atoms: Vec<String> = claim_items
        .iter()
        .flat_map(|item| match item.head() {
            Head::Sig { names, .. } => names,
            Head::Command { .. } | Head::Other => Vec::new(),
        })
        .collect();

    // Book lines: every `#=` statement is a declaration, an outcome, or a fact (`30Y` § 2.1).
    let mut books: Vec<Book<'_>> = books_raw
        .iter()
        .map(|(name, line, loads, commands)| Book {
            name,
            line: *line,
            loads: loads.iter().map(|(stem, _)| stem.clone()).collect(),
            rows: commands
                .iter()
                .map(|cmd| Row {
                    cmd,
                    names: Vec::new(),
                    stmts: cmd
                        .stmts
                        .iter()
                        .map(|(line, text)| (*line, stmt(text, &is_claim)))
                        .collect(),
                })
                .collect(),
        })
        .collect();

    let resolves =
        |stem: &str| claim_atoms.iter().any(|c| c == stem) || load_files.contains_key(stem);
    for fence in &fences {
        let (Fence::Load { stems, .. } | Fence::Book { loads: stems, .. }) = fence;
        for (stem, line) in stems.iter().filter(|(s, _)| !resolves(s)) {
            findings.push(Finding::new(Lint::LoadStemResolves, file, *line).with("stem", stem));
        }
    }
    for row in books.iter().flat_map(|b| &b.rows).filter(|r| !r.is_atom()) {
        for (line, stmt) in &row.stmts {
            if let Stmt::Declaration { text, .. } = stmt
                && alloy::mentions_this(text)
            {
                findings
                    .push(Finding::new(Lint::ThisOnAtomlessLine, file, *line).with("text", text));
            }
        }
    }

    // The join key: one literal carries at most one name and one name covers at most one literal,
    // where a literal no bare component names is named after itself (`30Y` § 2.1).
    let mut names_of: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut classes_of: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut first_seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut class_seen: BTreeMap<String, usize> = BTreeMap::new();
    for cmd in books.iter().flat_map(|b| &b.rows).map(|r| r.cmd) {
        for (literal, component) in &cmd.words {
            let names = names_of.entry(literal.clone()).or_default();
            match component {
                Component::Name(name) => {
                    names.insert(alloy::atom(name));
                }
                Component::Class(class) => {
                    let class = alloy::atom(class);
                    classes_of
                        .entry(literal.clone())
                        .or_default()
                        .insert(class.clone());
                    class_seen.entry(class).or_insert(cmd.line);
                }
            }
            first_seen.entry(literal.clone()).or_insert(cmd.line);
        }
    }
    let mut literals_of: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (literal, names) in &mut names_of {
        if names.is_empty() {
            names.insert(alloy::atom(literal));
        }
        for name in names.iter() {
            literals_of
                .entry(name.clone())
                .or_default()
                .insert(literal.clone());
        }
    }
    let seen_at = |literal: &str| first_seen.get(literal).copied().unwrap_or(0);
    let joined = |set: &BTreeSet<String>| set.iter().cloned().collect::<Vec<_>>().join(" ");
    for (literal, names) in names_of.iter().filter(|(_, n)| n.len() > 1) {
        findings.push(
            Finding::new(Lint::JoinKeyCoherence, file, seen_at(literal))
                .with("literal", literal)
                .with("names", joined(names)),
        );
    }
    for (name, literals) in literals_of.iter().filter(|(_, l)| l.len() > 1) {
        let line = literals.iter().map(|l| seen_at(l)).min().unwrap_or(0);
        findings.push(
            Finding::new(Lint::JoinKeyCoherence, file, line)
                .with("name", name)
                .with("literals", joined(literals)),
        );
    }
    if !findings.is_empty() {
        findings.sort_by_key(|f| (f.lint, f.line));
        return Err(findings);
    }
    let atom_of: BTreeMap<&str, &str> = names_of
        .iter()
        .filter_map(|(literal, names)| Some((literal.as_str(), names.first()?.as_str())))
        .collect();
    let literal_of_name: BTreeMap<&str, &str> = atom_of.iter().map(|(l, n)| (*n, *l)).collect();
    let word_for = |literal: &str| {
        atom_of
            .get(literal)
            .map_or_else(String::new, |n| (*n).to_owned())
    };
    let mut word_order: Vec<(usize, String)> = books
        .iter()
        .flat_map(|b| &b.rows)
        .flat_map(|r| r.cmd.words.iter().map(|(l, _)| (r.cmd.line, word_for(l))))
        .collect();
    for row in books.iter_mut().flat_map(|b| &mut b.rows) {
        row.names = row
            .cmd
            .words
            .iter()
            .map(|(literal, _)| word_for(literal))
            .collect();
    }

    // Words: map names, then the names statements and claim bodies introduce (`30Y` § 2.3). A
    // class is a `Class` atom, never a word.
    declared.extend(classes_of.values().flatten().cloned());
    for item in &claim_items {
        word_order.extend(
            alloy::introduced(&item.text, item.line, &declared)
                .into_iter()
                .map(|(w, l)| (l, w)),
        );
    }
    for (line, stmt) in books.iter().flat_map(|b| &b.rows).flat_map(|r| &r.stmts) {
        let (Stmt::Declaration { text, .. }
        | Stmt::Fact(text)
        | Stmt::Outcome { formula: text, .. }) = stmt;
        word_order.extend(
            alloy::introduced(text, *line, &declared)
                .into_iter()
                .map(|(w, l)| (l, w)),
        );
    }
    word_order.sort_by_key(|(line, _)| *line);
    let mut minted = BTreeSet::new();
    let mut words: Vec<(String, usize)> = word_order
        .into_iter()
        .filter(|(_, w)| minted.insert(w.clone()))
        .map(|(l, w)| (w, l))
        .collect();
    words.push((NULL_WORD.to_owned(), 0));
    let class_rows: Vec<(String, String)> = classes_of
        .iter()
        .flat_map(|(literal, classes)| classes.iter().map(|c| (word_for(literal), c.clone())))
        .collect();
    let classes: Vec<(String, usize)> = class_seen.into_iter().collect();

    let mut modules = Vec::new();
    let mut conjunctions: Vec<(String, Vec<String>)> = Vec::new();
    let mut add = |name: &str, body: String| modules.push((name.to_owned(), body));
    add("assay.als", HARNESS.to_owned());
    add("shared.als", module("shared", &["assay"], &shared_blocks));
    add(
        "species.als",
        module("species", &["assay", "shared"], &species),
    );

    let mut word_blocks: Vec<String> = words
        .iter()
        .map(|(w, line)| {
            let decl = format!("one sig {w} extends Shword {{}}");
            match line {
                0 => block("assay", 0, &decl),
                _ => block(file, *line, &decl),
            }
        })
        .collect();
    word_blocks.extend(
        classes
            .iter()
            .map(|(c, line)| block(file, *line, &format!("one sig {c} extends Class {{}}"))),
    );
    word_blocks.push(block(
        "assay",
        0,
        &if class_rows.is_empty() {
            "fact { no class }".to_owned()
        } else {
            let pairs: Vec<String> = class_rows
                .iter()
                .map(|(n, c)| format!("{n}->{c}"))
                .collect();
            format!("fact {{ class = {} }}", pairs.join(" + "))
        },
    ));
    add("words.als", module("words", &["shared"], &word_blocks));

    let mut claim_blocks: Vec<String> = claim_items
        .iter()
        .map(|item| block(file, item.line, &item.text))
        .collect();
    for fence in &fences {
        if let Fence::Load { name, line, stems } = fence {
            let members: Vec<&str> = stems.iter().map(|(s, _)| s.as_str()).collect();
            let set = if members.is_empty() {
                "none".to_owned()
            } else {
                members.join(" + ")
            };
            claim_blocks.push(block(
                file,
                *line,
                &format!("fun {name}: set Claim {{ {set} }}"),
            ));
        }
    }
    add(
        "claims.als",
        module("claims", &["species", "words"], &claim_blocks),
    );

    if let Some(src) = inputs.laws {
        for fence in md_fences(src.text).into_iter().filter(|f| f.alloy) {
            laws.push(block(src.name, fence.first_line, &fence.lines.join("\n")));
        }
    }
    add("laws.als", module("laws", &["species"], &laws));

    let ctx = Ctx {
        file,
        default_scope: carriers.get("bookScope").map(String::as_str),
        words: words.len(),
        classes: classes.len(),
        doc_claims: claim_atoms.len(),
    };
    let corpus_bounds = ctx.bounds(1, 0);
    let mut corpus_blocks = vec![block(
        "assay",
        0,
        &format!(
            "one sig line_1 extends Line {{}}\n\
             fact {{ line_1.cmd = {NULL_WORD}  no line_1.argv  no line_1.above }}\n\
             fact {{ {} }}",
            speech("line_1", &claim_atoms)
        ),
    )];
    let mut corpus_conjoined = Vec::new();
    for (line, body, own) in &corpus {
        let clause = scope(own.as_deref().or(ctx.default_scope), &corpus_bounds);
        corpus_blocks.push(block(file, *line, &format!("{body} for {clause}")));
        if own.is_none()
            && let Some((name, formula)) = book::check_formula(body)
        {
            corpus_conjoined.push((name, formula));
        }
    }
    let corpus_clause = scope(ctx.default_scope, &corpus_bounds);
    if let Some(every) = book::every_line(&corpus_conjoined, &corpus_clause) {
        corpus_blocks.push(block("assay", 0, &every));
        conjunctions.push((
            "book_corpus".to_owned(),
            corpus_conjoined.iter().map(|(n, _)| n.clone()).collect(),
        ));
    }
    // With every claim in force the universe may be empty, and then every corpus check greens
    // vacuously; this run must be sat (`30Y` § 2.5 item 5).
    corpus_blocks.push(block(
        "assay",
        0,
        &format!("run book_corpus {{}} for {corpus_clause}"),
    ));
    add(
        "book_corpus.als",
        module("book_corpus", &["claims"], &corpus_blocks),
    );

    let mut book_rows = Vec::new();
    for book in &books {
        let own = carriers
            .get(&format!("bookScope_{}", book.name))
            .map(String::as_str);
        let (text, row, conjoined) = emit_book(book, &ctx, own);
        add(&format!("book_{}.als", book.name), text);
        book_rows.push(row);
        if !conjoined.is_empty() {
            conjunctions.push((format!("book_{}", book.name), conjoined));
        }
    }
    let modules = finish(modules)?;

    let literal_of = |name: &str| match name {
        NULL_WORD => Json::str(":"),
        _ => literal_of_name
            .get(name)
            .map_or(Json::Null, |l| Json::str(*l)),
    };
    let report = vec![
        (
            "words",
            Json::Arr(
                words
                    .iter()
                    .map(|(w, _)| Json::obj([("name", Json::str(w)), ("literal", literal_of(w))]))
                    .collect(),
            ),
        ),
        (
            "classes",
            Json::Obj(
                classes
                    .iter()
                    .map(|(c, _)| {
                        (
                            c.clone(),
                            Json::Arr(
                                class_rows
                                    .iter()
                                    .filter(|(_, k)| k == c)
                                    .map(|(n, _)| Json::str(n))
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),
        ),
        (
            "corpus",
            Json::obj([
                ("commands", num(corpus.len())),
                ("claims", num(claim_atoms.len())),
                ("scope", Json::str(scope(ctx.default_scope, &corpus_bounds))),
            ]),
        ),
        ("books", Json::Arr(book_rows)),
        ("lints", lints_json(&[])),
    ];
    Ok(Compiled {
        modules,
        report,
        conjunctions,
    })
}

fn finish(modules: Vec<(String, String)>) -> Result<Vec<(String, emit::Rendered)>, Vec<Finding>> {
    let mut findings = Vec::new();
    let mut out = Vec::new();
    for (name, internal) in modules {
        match emit::render(&internal) {
            Ok(rendered) => {
                let mut seen = BTreeSet::new();
                for item in alloy::items(&rendered.text, 1) {
                    if let Head::Command {
                        name: Some(label), ..
                    } = item.head()
                        && !seen.insert(label.clone())
                    {
                        let origin = rendered.origin(item.line, 1);
                        findings.push(
                            Finding::new(
                                Lint::LabelIsUniqueInModule,
                                origin.map_or(name.as_str(), |o| o.file.as_str()),
                                origin.map_or(0, |o| o.line),
                            )
                            .with("module", &name)
                            .with("label", label),
                        );
                    }
                }
                out.push((name, rendered));
            }
            Err((file, line, why)) => findings.push(
                Finding::new(Lint::TokenizerIsCertain, &file, line)
                    .with("module", &name)
                    .with("construct", why),
            ),
        }
    }
    if findings.is_empty() {
        Ok(out)
    } else {
        Err(findings)
    }
}

fn stmt(text: &str, is_claim: &dyn Fn(&Head) -> bool) -> Stmt {
    match head_of(text) {
        head @ Head::Sig { .. } => {
            let claim = is_claim(&head);
            let Head::Sig { names, .. } = head else {
                return Stmt::Fact(text.to_owned());
            };
            Stmt::Declaration {
                names,
                claim,
                text: text.to_owned(),
            }
        }
        Head::Command { .. } | Head::Other if alloy::mentions_this(text) => {
            let (formula, scope) = alloy::split_scope(text);
            Stmt::Outcome { formula, scope }
        }
        Head::Command { .. } | Head::Other => Stmt::Fact(text.to_owned()),
    }
}

fn is_carrier(name: &str) -> bool {
    name == "bookScope" || name.starts_with("bookScope_")
}

fn reaches_claim(sig: &str, parents: &BTreeMap<String, Vec<String>>) -> bool {
    let mut stack = vec![sig];
    let mut seen = BTreeSet::new();
    while let Some(s) = stack.pop() {
        if s == "Claim" {
            return true;
        }
        if seen.insert(s) {
            stack.extend(parents.get(s).into_iter().flatten().map(String::as_str));
        }
    }
    false
}

/// An emitted item with the spec line it came from (`30Y` § 3, the line-mapping culture).
fn block(file: &str, line: usize, text: &str) -> String {
    format!("{ORIGIN}{file}{ORIGIN}{line}\n{text}")
}

fn module(name: &str, opens: &[&str], blocks: &[String]) -> String {
    let mut out = format!("module {name}\n");
    for open in opens {
        out.push_str("open ");
        out.push_str(open);
        out.push('\n');
    }
    for block in blocks {
        out.push('\n');
        out.push_str(block);
        out.push('\n');
    }
    out
}

fn speech(atom: &str, set: &[String]) -> String {
    if set.is_empty() {
        format!("no {atom}.speech")
    } else {
        format!("{atom}.speech = {}", set.join(" + "))
    }
}

/// The four kinds assay sizes itself, exactly (`30Y` § 2.4 `mech-scope-is-spelled-in-alloy`).
#[derive(Debug, Clone, Copy)]
struct Bounds {
    words: usize,
    classes: usize,
    lines: usize,
    claims: usize,
}

impl Ctx<'_> {
    fn bounds(&self, lines: usize, book_claims: usize) -> Bounds {
        Bounds {
            words: self.words,
            classes: self.classes,
            lines,
            claims: self.doc_claims.saturating_add(book_claims),
        }
    }
}

/// A spec's scope clause with assay's exact bounds appended, and `seq` sized wherever `Int` is,
/// since Alloy clamps an unspelled `seq` silently (`30Y` § 2.4).
fn scope(base: Option<&str>, b: &Bounds) -> String {
    let exact = format!(
        "exactly {} Shword, exactly {} Class, exactly {} Line, exactly {} Claim",
        b.words, b.classes, b.lines, b.claims
    );
    let Some(base) = base.map(str::trim).filter(|s| !s.is_empty()) else {
        return exact;
    };
    let toks = alloy::tokenize(base, 0);
    let words: Vec<&str> = toks.iter().map(|t| alloy::text(base, t)).collect();
    let mut clause = base.to_owned();
    if !words.contains(&"seq")
        && let Some(bits) = words
            .windows(2)
            .find(|w| w.get(1) == Some(&"Int"))
            .and_then(|w| w.first()?.parse::<u32>().ok())
        && let Some(len) = bits
            .checked_sub(1)
            .and_then(|b| 1u64.checked_shl(b))
            .and_then(|n| n.checked_sub(1))
    {
        clause = format!("{clause}, {len} seq");
    }
    let bare_number =
        toks.first().is_some_and(|t| t.kind == alloy::Tok::Number) && !words.contains(&"but");
    format!(
        "{clause}{}{exact}",
        if bare_number { " but " } else { ", " }
    )
}

/// One book module (`30Y` § 2.4), and its report row.
#[expect(
    clippy::too_many_lines,
    reason = "one straight-line emitter, in module order"
)]
fn emit_book(
    book: &Book<'_>,
    ctx: &Ctx<'_>,
    own_scope: Option<&str>,
) -> (String, Json, Vec<String>) {
    let file = ctx.file;
    let base = own_scope.or(ctx.default_scope);
    let declared_claims: usize = book
        .rows
        .iter()
        .flat_map(|r| &r.stmts)
        .map(|(_, s)| match s {
            Stmt::Declaration {
                names, claim: true, ..
            } => names.len(),
            Stmt::Declaration { .. } | Stmt::Outcome { .. } | Stmt::Fact(_) => 0,
        })
        .sum();
    let bounds = ctx.bounds(
        book.rows.iter().filter(|r| r.is_atom()).count(),
        declared_claims,
    );

    let mut facts = Vec::new();
    let mut decls = Vec::new();
    let mut shapes = Vec::new();
    let mut speeches = Vec::new();
    let mut checks = Vec::new();
    let mut overrides = Vec::new();
    let mut conjoined: Vec<(String, String)> = Vec::new();
    let mut premises: Vec<String> = Vec::new();
    let mut atoms: Vec<String> = Vec::new();
    let mut in_force: Vec<String> = Vec::new();
    for stem in &book.loads {
        if !in_force.contains(stem) {
            in_force.push(stem.clone());
        }
    }
    for row in &book.rows {
        let me = format!("line_{}", row.cmd.number);
        let this = |text: &str| {
            if row.is_atom() {
                alloy::replace_this(text, &me)
            } else {
                text.to_owned()
            }
        };
        let mut outcome = Vec::new();
        let mut line_scope = None;
        for (line, stmt) in &row.stmts {
            match stmt {
                Stmt::Fact(text) => facts.push(block(file, *line, &format!("fact {{ {text} }}"))),
                Stmt::Declaration { names, claim, text } => {
                    decls.push(block(file, *line, &this(text)));
                    if *claim {
                        in_force.extend(names.iter().cloned());
                    }
                }
                Stmt::Outcome { formula, scope: s } => {
                    outcome.push(format!("({})", this(formula)));
                    if s.is_some() {
                        line_scope.clone_from(s);
                    }
                }
            }
        }
        if !row.is_atom() {
            continue;
        }
        let argv: Vec<String> = row
            .names
            .iter()
            .skip(1)
            .enumerate()
            .map(|(i, w)| format!("{i}->{w}"))
            .collect();
        let argv = if argv.is_empty() {
            format!("no {me}.argv")
        } else {
            format!("{me}.argv = {}", argv.join(" + "))
        };
        let above = if atoms.is_empty() {
            format!("no {me}.above")
        } else {
            format!("{me}.above = {}", atoms.join(" + "))
        };
        let cmd = row.names.first().map_or("", String::as_str);
        shapes.push(block(
            file,
            row.cmd.line,
            &format!("fact {{ {me}.cmd = {cmd}  {argv}  {above} }}"),
        ));
        let set: Vec<String> = match atoms.last() {
            None => std::mem::take(&mut in_force),
            Some(prev) => std::iter::once(format!("{prev}.speech"))
                .chain(in_force.drain(..))
                .collect(),
        };
        speeches.push(block(
            file,
            row.cmd.line,
            &format!("fact {{ {} }}", speech(&me, &set)),
        ));
        let conj = |ps: &[String]| {
            if ps.len() == 1 {
                ps.concat()
            } else {
                format!("({})", ps.join(" and "))
            }
        };
        let body = if premises.is_empty() {
            conj(&outcome)
        } else {
            format!("{} implies {}", conj(&premises), conj(&outcome))
        };
        if let Some(s) = &line_scope {
            overrides.push(Json::obj([
                ("line", Json::str(&me)),
                ("scope", Json::str(s)),
            ]));
        }
        let clause = scope(line_scope.as_deref().or(base), &bounds);
        checks.push(block(
            file,
            row.cmd.line,
            &format!("check {me} {{ {body} }} for {clause}"),
        ));
        if line_scope.is_none() {
            conjoined.push((me.clone(), body));
        }
        premises.extend(outcome);
        atoms.push(me);
    }
    let book_clause = scope(base, &bounds);
    let every = book::every_line(&conjoined, &book_clause);
    if let Some(every) = &every {
        checks.push(block(file, book.line, every));
    }
    let mut blocks = facts;
    if !atoms.is_empty() {
        blocks.push(block(
            file,
            book.line,
            &format!("one sig {} extends Line {{}}", atoms.join(", ")),
        ));
    }
    blocks.extend(decls);
    blocks.extend(shapes);
    blocks.extend(speeches);
    blocks.extend(checks);
    blocks.push(block(
        file,
        book.line,
        &format!(
            "run {} {{ {} }} for {book_clause}",
            book.name,
            premises.join(" and ")
        ),
    ));
    let row = Json::obj([
        ("name", Json::str(book.name)),
        ("lines", num(atoms.len())),
        ("declared_claims", num(declared_claims)),
        ("scope", Json::str(book_clause)),
        ("overrides", Json::Arr(overrides)),
    ]);
    let members = if every.is_some() {
        conjoined.into_iter().map(|(name, _)| name).collect()
    } else {
        Vec::new()
    };
    (
        module(&format!("book_{}", book.name), &["claims"], &blocks),
        row,
        members,
    )
}
