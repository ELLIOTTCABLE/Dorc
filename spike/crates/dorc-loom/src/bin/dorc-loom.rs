//! `dorc-loom` is the read-only transcript-template inspection command.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[cfg(test)]
use dorc_loom::TemplateVariableName;
use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, FsReceiptStore, GitRepository, InspectedCompilation,
    InspectedReplay, Repository, SectionKey, SectionVariableId, build_publication,
    classify_prose_changes, compile_preview, compile_receipt, load_arrangement_corpus,
    load_corpus_by_slug, promote_receipt, render_compile_preview, replay_case,
    replay_case_with_inputs,
};
#[cfg(test)]
use errorloom::EditableSection;
use errorloom::{
    Case, EditableFragment, RenderComponent, ReplayInput, ReplayResult, RunEnv, RunError,
    execute_generic, read_case, read_case_text,
};

const USAGE: &str = "usage: dorc-loom <compile|promote [--quiet] [--shell=PATH] [--path=DIR]... [CASE...]|vars <--used|--all> [CASE...]|scaffold SLUG|add-register CASE help|sections [CASE...]>\n       a CASE is a bare slug (`whylog-unwritten`), a filename, or a path; an omitted list means every crates/aid/tests/*.loom\n       edit a sentence in a case's transcript, then compile and promote it; type {{name}} to insert or move one of its values";

/// The `{{name}}` mechanism has no other trace: every committed case is fully rendered, so a
/// reader who has only ever seen transcripts has no way to learn that a value can be typed at all.
/// Both inventory surfaces say it, once, at the top.
const VALUE_SYNTAX_NOTE: &str = "type {{name}} in a sentence to insert or move one of these values; omitting one bakes it to \
     literal text";

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            let _ = writeln!(io::stderr(), "dorc-loom: {message}");
            ExitCode::from(2)
        }
    }
}

enum Command {
    Compile {
        cases: Vec<PathBuf>,
        env: RunEnv,
        quiet: bool,
    },
    Promote {
        cases: Vec<PathBuf>,
        env: RunEnv,
        quiet: bool,
    },
    Vars {
        used: bool,
        cases: Vec<PathBuf>,
    },
    Scaffold {
        slug: String,
    },
    AddRegister {
        case: PathBuf,
        register: String,
    },
    Sections {
        cases: Vec<PathBuf>,
    },
    Help,
}

type SelectedCase = (String, PathBuf);

/// Refusals carry their count so the closing status line can match the exit code it explains.
enum Inspected {
    Ready(InspectedCompilation, DorcConsumer),
    Refused { cases: usize },
}

struct GatedCases {
    repository: GitRepository,
    paths: Vec<SelectedCase>,
    touched: std::collections::BTreeSet<String>,
    staged: std::collections::BTreeSet<String>,
}

fn run() -> Result<ExitCode, String> {
    let command = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let stderr = io::stderr();
    let mut err = stderr.lock();
    match command {
        Command::Compile { cases, env, quiet } => {
            compile_cases(&cases, &env, quiet, &mut out, &mut err)
        }
        Command::Promote { cases, env, quiet } => promote_cases(&cases, &env, quiet, &mut out),
        Command::Vars { used, cases } => print_variables(used, &cases, &mut out),
        Command::Scaffold { slug } => scaffold_case(&slug, &mut out),
        Command::AddRegister { case, register } => add_register(&case, &register, &mut out),
        Command::Sections { cases } => print_sections(&cases, &mut out),
        Command::Help => writeln!(out, "{USAGE}")
            .map_err(|error| error.to_string())
            .map(|()| ExitCode::SUCCESS),
    }
}

/// Resolve one CASE argument to a real file.
///
/// Four spellings resolve, because a reader who has only ever seen a case's SLUG has no way to know
/// where the collection lives, and the previous single spelling (a path relative to `spike/`) was
/// nowhere stated. In order: the canonical collection by slug, the canonical collection by
/// filename, the path as given, and the path against the workspace root.
fn resolve_case(arg: &str) -> Result<PathBuf, String> {
    let slug = arg.strip_suffix(".loom").unwrap_or(arg);
    let tried = [
        cases_dir().join(format!("{slug}.loom")),
        PathBuf::from(arg),
        spike_dir()?.join(arg),
    ];
    if let Some(found) = tried.iter().find(|path| path.is_file()) {
        return Ok(found.clone());
    }
    Err(format!(
        "no case `{arg}`. A CASE is its bare slug (`whylog-unwritten`), its filename, or a path \
         relative to the current directory or to `spike/`; these were tried, in order: {}. \
         The collection is {} — `dorc-loom sections` with no arguments lists every case in it",
        tried
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", "),
        cases_dir().display()
    ))
}

fn parse_args() -> Result<Command, String> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    // Anywhere, not only first: a reader who has already typed a verb asks the verb for help.
    if argv
        .iter()
        .any(|arg| matches!(arg.as_str(), "--help" | "-h" | "help"))
    {
        return Ok(Command::Help);
    }
    let mut argv = argv.into_iter();
    match argv.next().as_deref() {
        Some("compile") => {
            let (cases, env, quiet) = collect_compile_args(argv)?;
            Ok(Command::Compile { cases, env, quiet })
        }
        Some("promote") => {
            let (cases, env, quiet) = collect_compile_args(argv)?;
            Ok(Command::Promote { cases, env, quiet })
        }
        Some("vars") => {
            let mode = argv
                .next()
                .ok_or_else(|| format!("vars needs --used or --all\n{USAGE}"))?;
            let used = match mode.as_str() {
                "--used" => true,
                "--all" => false,
                _ => return Err(format!("unknown vars mode {mode:?}\n{USAGE}")),
            };
            Ok(Command::Vars {
                used,
                cases: collect_cases(argv)?,
            })
        }
        Some("scaffold") => {
            let slug = argv
                .next()
                .ok_or_else(|| format!("scaffold needs a code slug\n{USAGE}"))?;
            if argv.next().is_some() {
                return Err(format!("scaffold takes exactly one slug\n{USAGE}"));
            }
            Ok(Command::Scaffold { slug })
        }
        Some("add-register") => {
            let case = argv
                .next()
                .ok_or_else(|| format!("add-register needs a case path\n{USAGE}"))?;
            let register = argv
                .next()
                .ok_or_else(|| format!("add-register needs a register name\n{USAGE}"))?;
            if argv.next().is_some() {
                return Err(format!(
                    "add-register takes one case and one register\n{USAGE}"
                ));
            }
            Ok(Command::AddRegister {
                case: resolve_case(&case)?,
                register,
            })
        }
        Some("sections") => Ok(Command::Sections {
            cases: collect_cases(argv)?,
        }),
        _ => Err(USAGE.to_owned()),
    }
}

/// `dorc-loom add-register CASE help` — mint a code's help register so the ordinary transcript loop
/// can fill it (`28L:rul-help-affordance-is-scaffold`).
///
/// The register is a CATALOG fact, so this publishes through the same generator promote uses: the
/// lock gains `HelpRegister::Unwritten` and the case's transcript grows the
/// `= help: [unwritten: <slug>.help]` line the author then overtypes. Nothing here writes prose.
fn add_register(path: &Path, register: &str, out: &mut impl Write) -> Result<ExitCode, String> {
    if register != "help" {
        return Err(format!(
            "`help` is the only register that can be added; `message` exists on every code and \
             `{register}` is not a register"
        ));
    }
    let case = load(path)?;
    let slug = case
        .frontmatter()
        .scalar("code")
        .ok_or_else(|| {
            format!(
                "{} declares no `code`, so it owns no catalog registers",
                path.display()
            )
        })?
        .to_owned();
    let gated = gate_touched_set(std::slice::from_ref(&path.to_path_buf()))?;
    if !gated.touched.is_empty() {
        return Err(format!(
            "{} has a prose edit that is not promoted yet, and adding a register rewrites the \
             case; run `dorc-loom compile {0}` then `dorc-loom promote {0}` first",
            path.display()
        ));
    }
    let mut consumer = DorcConsumer::new();
    consumer.seed_help_register(&slug).map_err(|refusal| match refusal {
        dorc_loom::SeedRefusal::MissingCode(slug) => format!(
            "no catalog row for `{slug}`; promote its defining case first: `dorc-loom promote {}`",
            path.display()
        ),
        dorc_loom::SeedRefusal::AlreadyPresent(slug) => format!(
            "`{slug}` already has a help register; edit its `= help:` line in {}, then \
             `dorc-loom compile {0}` and `dorc-loom promote {0}`",
            path.display()
        ),
    })?;
    publish(
        &consumer,
        &std::collections::BTreeMap::from([(slug.clone(), case)]),
        out,
    )?;
    writeln!(
        out,
        "next: rebuild, then overtype `[unwritten: {slug}.help]` in {} with the remediation words",
        path.display()
    )
    .map_err(|error| error.to_string())?;
    writeln!(
        out,
        "then: dorc-loom compile {0} && dorc-loom promote {0}",
        path.display()
    )
    .map_err(|error| error.to_string())
    .map(|()| ExitCode::SUCCESS)
}

/// Write the empty defining-case skeleton for a freshly-minted code
/// (`288` §4 prop-scaffold-explicit-command). An EXPLICIT command, never a build or test
/// side-effect: tests never write source, and concurrent builders never race over the collection.
///
/// Everything the skeleton omits is deliberately red. Empty `when-fires`/`why` fail
/// `required_metadata_is_non_empty`; an empty replay output fails the same-slug coherence gate
/// (`check_hygiene`) until a genuinely-firing world is authored and blessed — the scaffold-and-forget
/// guard. `message` is never written, so the code renders `[unwritten: <slug>]` at every seat:
/// builders author zero user-facing prose (`error-authorship-tier`).
fn scaffold_case(slug: &str, out: &mut impl Write) -> Result<ExitCode, String> {
    if slug.is_empty()
        || !slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(format!(
            "slug {slug:?} is not a code slug (lowercase letters, digits, and hyphens)"
        ));
    }
    let path = cases_dir().join(format!("{slug}.loom"));
    if path.exists() {
        return Err(format!(
            "{} already exists; scaffold never overwrites an authored case",
            path.display()
        ));
    }
    let skeleton = format!(
        "---\ncode: {slug}\nwhen-fires:\nwhy:\n---\n-- replay --\n$ dorc plan --book=book.sh\n"
    );
    std::fs::write(&path, skeleton)
        .map_err(|error| format!("write {}: {error}", path.display()))?;
    writeln!(out, "scaffold: wrote {}", path.display()).map_err(|error| error.to_string())?;
    writeln!(
        out,
        "next: author `when-fires`/`why`, then replace the replay with a command that really fires `{slug}`"
    )
    .map_err(|error| error.to_string())?;
    writeln!(
        out,
        "then: dorc-loom promote {} (orchestrator-only, on a freshly verified binary)",
        path.display()
    )
    .map_err(|error| error.to_string())?;
    Ok(ExitCode::SUCCESS)
}

fn collect_compile_args(
    mut argv: impl Iterator<Item = String>,
) -> Result<(Vec<PathBuf>, RunEnv, bool), String> {
    let mut env = RunEnv::new().path_dir(binary_dir()?);
    let mut cases = Vec::new();
    let mut quiet = false;
    while let Some(arg) = argv.next() {
        if let Some(shell) = arg.strip_prefix("--shell=") {
            env = env.shell(shell);
        } else if let Some(path) = arg.strip_prefix("--path=") {
            env = env.path_dir(path);
        } else if arg == "--shell" {
            env = env.shell(next_value(&mut argv, "--shell")?);
        } else if arg == "--path" {
            env = env.path_dir(next_value(&mut argv, "--path")?);
        } else if arg == "--quiet" {
            quiet = true;
        } else if arg.starts_with('-') {
            return Err(format!("unknown option {arg:?}\n{USAGE}"));
        } else {
            cases.push(resolve_case(&arg)?);
        }
    }
    if cases.is_empty() {
        return Ok((corpus_cases()?, env, quiet));
    }
    Ok((cases, env, quiet))
}

fn next_value(argv: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    argv.next()
        .ok_or_else(|| format!("{option} needs a value\n{USAGE}"))
}

fn binary_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|error| format!("locate built tools: {error}"))?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| "locate built tools: executable has no parent".to_owned())
}

fn collect_cases(argv: impl Iterator<Item = String>) -> Result<Vec<PathBuf>, String> {
    let mut cases = Vec::new();
    for arg in argv {
        if arg.starts_with('-') {
            return Err(format!("unknown option {arg:?}\n{USAGE}"));
        }
        cases.push(resolve_case(&arg)?);
    }
    if cases.is_empty() {
        return corpus_cases();
    }
    Ok(cases)
}

/// Every committed defining case, in a stable order — what a verb operates on when given no
/// explicit CASE list.
///
/// The default is the WHOLE corpus rather than an error because `compile` and `promote` must
/// see the same list for the receipt to match, and the tool already narrows to the
/// prose-changed subset itself (`gate_touched_set`). So "all of them" reads as "publish what
/// I edited", not as a blunderbuss, and spares every caller from keeping two lists in sync.
/// Sorted because a `read_dir` order is not guaranteed and the receipt is order-sensitive.
fn corpus_cases() -> Result<Vec<PathBuf>, String> {
    let dir = cases_dir();
    let read =
        std::fs::read_dir(&dir).map_err(|error| format!("read {}: {error}", dir.display()))?;
    let mut cases: Vec<PathBuf> = read
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("read {}: {error}", dir.display()))?
        .into_iter()
        .filter(|path| path.extension().is_some_and(|kind| kind == "loom"))
        .collect();
    if cases.is_empty() {
        return Err(format!("no .loom cases under {}", dir.display()));
    }
    cases.sort();
    Ok(cases)
}

fn compile_cases(
    cases: &[PathBuf],
    env: &RunEnv,
    quiet: bool,
    out: &mut impl Write,
    err: &mut impl Write,
) -> Result<ExitCode, String> {
    validate_case_inputs(cases)?;
    let gated = gate_touched_set(cases)?;
    let total = gated.paths.len();
    let (inspection, _consumer) = match inspect_cases(&gated, env, quiet, out)? {
        Inspected::Ready(inspection, consumer) => (inspection, consumer),
        Inspected::Refused { cases } => {
            status(err, &format!("{total} cases, {cases} refused"))?;
            return Ok(ExitCode::from(1));
        }
    };
    let store = receipt_store()?;
    let outcome = compile_receipt(&store, &inspection)?;
    if matches!(outcome, dorc_loom::ReceiptWriteOutcome::CleanupPending) {
        writeln!(
            out,
            "compile: receipt published; retained backup requires deliberate resolution; subsequent writes refuse"
        )
        .map_err(|error| error.to_string())?;
    }
    note_staged_cases(&gated.staged, &std::collections::BTreeSet::new(), out)?;
    status(
        err,
        &format!(
            "{total} cases, {} touched, receipt {}",
            gated.touched.len(),
            store.path().display()
        ),
    )?;
    Ok(ExitCode::SUCCESS)
}

/// The one line a compile always emits, quiet included, on stderr.
///
/// A compile changes no tracked file, so without this its only trace is a receipt under `target/`
/// that nothing announces — and a reader who cannot see that durable state has no way to learn
/// that promote depends on it.
fn status(err: &mut impl Write, summary: &str) -> Result<(), String> {
    writeln!(err, "compile: {summary}").map_err(|error| error.to_string())
}

fn promote_cases(
    cases: &[PathBuf],
    env: &RunEnv,
    quiet: bool,
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    validate_case_inputs(cases)?;
    let gated = gate_touched_set(cases)?;
    let Inspected::Ready(inspection, consumer) = inspect_cases(&gated, env, quiet, out)? else {
        return Ok(ExitCode::from(1));
    };
    promote_receipt(&receipt_store()?, &inspection)?;
    let affected = touched_cases(&gated)?;
    let before = staged_bytes(&gated)?;
    publish(&consumer, &affected, out)?;
    note_staged_cases(&gated.staged, &rewritten_staged(&gated, &before)?, out)?;
    Ok(ExitCode::SUCCESS)
}

/// The touched defining cases (dirty on-disk bytes) keyed by their `code` slug — the only cases a
/// prose edit re-renders and republishes.
fn touched_cases(gated: &GatedCases) -> Result<std::collections::BTreeMap<String, Case>, String> {
    let mut cases = std::collections::BTreeMap::new();
    for (relative_path, path) in &gated.paths {
        if !gated.touched.contains(relative_path) {
            continue;
        }
        let case = load(path)?;
        let slug = case
            .frontmatter()
            .scalar("code")
            .or_else(|| case.frontmatter().scalar("arrangement"))
            .ok_or_else(|| {
                format!(
                    "touched case {} declares neither `code` nor `arrangement`",
                    path.display()
                )
            })?
            .to_owned();
        cases.insert(slug, case);
    }
    Ok(cases)
}

/// Naming a staged case IS the remedy: dorc-loom mutates no index
/// (`282:rul-promote-is-one-atomic-act`), so a rewrite otherwise strands the author's `git add`
/// on their own pre-promote bytes.
fn note_staged_cases(
    staged: &std::collections::BTreeSet<String>,
    rewritten: &std::collections::BTreeSet<String>,
    out: &mut impl Write,
) -> Result<(), String> {
    for path in staged {
        let note = if rewritten.contains(path) {
            format!(
                "note: {path} was staged and has been rewritten; `git add` it again before \
                 committing -- dorc-loom never touches your index"
            )
        } else {
            format!("note: {path} is staged; dorc-loom read your worktree and never the index")
        };
        writeln!(out, "{note}").map_err(|error| error.to_string())?;
    }
    Ok(())
}

/// Read before publication, so the note can name exactly the cases a rewrite left stale.
fn staged_bytes(gated: &GatedCases) -> Result<std::collections::BTreeMap<String, Vec<u8>>, String> {
    gated
        .staged
        .iter()
        .map(|path| {
            gated
                .repository
                .current_bytes(path)
                .map(|bytes| (path.clone(), bytes))
        })
        .collect()
}

fn rewritten_staged(
    gated: &GatedCases,
    before: &std::collections::BTreeMap<String, Vec<u8>>,
) -> Result<std::collections::BTreeSet<String>, String> {
    let mut rewritten = std::collections::BTreeSet::new();
    for (path, bytes) in before {
        if gated.repository.current_bytes(path)? != *bytes {
            rewritten.insert(path.clone());
        }
    }
    Ok(rewritten)
}

/// Compute the entire preflighted candidate set from the edited mirror, then publish the changed
/// files — the lock first, then affected cases in lexical order — by per-target temp-file-and-rename
/// (`282:rul-promote-is-one-atomic-act`). All bytes and both fixpoints precede every write, so a
/// validation failure leaves committed files byte-identical; a mid-publication interruption is loud
/// in git and repaired by rerun. No journal, staging, rollback, or index mutation.
fn publish(
    consumer: &DorcConsumer,
    affected: &std::collections::BTreeMap<String, Case>,
    out: &mut impl Write,
) -> Result<(), String> {
    let cases_dir = cases_dir();
    let corpus = load_corpus_by_slug(&cases_dir)?;
    let arrangements = load_arrangement_corpus(&cases_dir)?;
    let publication = build_publication(consumer, &corpus, &arrangements, affected)?;

    let mut wrote = false;
    for (path, bytes) in [
        (catalog_path(), &publication.lock),
        (arrangement_path(), &publication.arrangement_lock),
    ] {
        if file_differs(&path, bytes) {
            publish_file(&path, bytes)?;
            writeln!(out, "promote: wrote {}", path.display()).map_err(|e| e.to_string())?;
            wrote = true;
        }
    }
    for (slug, bytes) in &publication.cases {
        let path = cases_dir.join(format!("{slug}.loom"));
        if !path.is_file() {
            return Err(format!("defining case `{slug}` is not `{slug}.loom`"));
        }
        if file_differs(&path, bytes) {
            publish_file(&path, bytes)?;
            writeln!(out, "promote: wrote {}", path.display()).map_err(|e| e.to_string())?;
            wrote = true;
        }
    }
    if !wrote {
        writeln!(out, "promote: corpus already at the generated fixpoint")
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn file_differs(path: &Path, bytes: &str) -> bool {
    std::fs::read_to_string(path).map_or(true, |current| current != bytes)
}

/// Replace one target by writing a sibling temp file and renaming over it (same directory, so the
/// rename does not cross a mount point). Not a crash-atomic transaction; the preflight above is where
/// atomicity lives.
fn publish_file(path: &Path, bytes: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("no parent dir for {}", path.display()))?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("no filename for {}", path.display()))?;
    let tmp = parent.join(format!(".{name}.dorc-loom-tmp"));
    std::fs::write(&tmp, bytes).map_err(|e| format!("write {}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, path).map_err(|e| format!("rename into {}: {e}", path.display()))
}

fn cases_dir() -> PathBuf {
    crates_dir().join("aid").join("tests")
}

/// `spike/crates`, so every path this tool prints reads as a real location rather than as a
/// traversal out of whichever crate happens to host the binary.
fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map_or_else(|| PathBuf::from("crates"), Path::to_path_buf)
}

/// Drive one case's replays through the Dorc adapter, routing declines to the generic executor.
type DrivenReplays = Vec<ReplayResult<SectionKey, SectionVariableId>>;

fn drive_replays(
    case: &Case,
    consumer: &DorcConsumer,
    env: &RunEnv,
    path: &Path,
    source: &str,
) -> Result<DrivenReplays, String> {
    let input = ReplayInput::new(case_name(path)?, source.to_owned())
        .map_err(|error| format!("{}: {error}", path.display()))?;
    catch_arity_panic(path, || {
        replay_case_with_inputs(case, consumer, env, &[input], |command, context| {
            execute_generic(command, context).map(ReplayResult::bytes)
        })
        .map_err(|error| match error {
            // The raw refusal names neither the flag that supplies a shell nor the decline that
            // needed one.
            RunError::ShellNotConfigured => format!(
                "{}: a replay declined the in-process Dorc driver and would need the generic \
                 executor, which has no shell. Rerun with `--shell=PATH` (e.g. `--shell=/bin/sh`), \
                 or make the replay a shape the driver handles",
                path.display()
            ),
            other => format!("{}: {other}", path.display()),
        })
    })
}

/// Traps a hand-seeded row's arity-mismatch panic (`dorc_aid::arrangement::sentence_words`'s
/// `debug_assert!`) as a typed refusal instead of crashing the process. Hook suppressed for the
/// call only; sound because `drive` is read-only.
fn catch_arity_panic<T>(
    path: &Path,
    drive: impl FnOnce() -> Result<T, String> + std::panic::UnwindSafe,
) -> Result<T, String> {
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = std::panic::catch_unwind(drive);
    std::panic::set_hook(previous_hook);
    match outcome {
        Ok(result) => result,
        Err(payload) => Err(format!(
            "{}: rendering this case panicked ({}). A hand-seeded arrangement row's `words` list \
             must carry exactly one more word than the values its seat interleaves; fix the named \
             row's word count in crates/aid/src/arrangement_lock.rs to match, then rebuild",
            path.display(),
            panic_message(&payload)
        )),
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&str>() {
        (*message).to_owned()
    } else {
        "no panic message available".to_owned()
    }
}

/// A case with nothing to report costs a `--quiet` caller nothing: the header is written only once
/// its body is, so the untouched majority of the corpus falls silent while every refusal,
/// interpretation, and note survives verbatim.
fn emit_case(out: &mut impl Write, path: &Path, body: &[u8], quiet: bool) -> Result<(), String> {
    if quiet && body.is_empty() {
        return Ok(());
    }
    writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
    out.write_all(body).map_err(|error| error.to_string())
}

fn inspect_cases(
    gated: &GatedCases,
    env: &RunEnv,
    quiet: bool,
    out: &mut impl Write,
) -> Result<Inspected, String> {
    let (mut consumer, mut refused, mut selected) = (DorcConsumer::new(), 0usize, Vec::new());
    let mut inspected_cases = Vec::new();
    for (relative_path, path) in &gated.paths {
        let relative_path = relative_path.clone();
        let (case, source) = load_with_text(path)?;
        let head = gated.repository.head_bytes(&relative_path)?;
        let head = std::str::from_utf8(&head)
            .map_err(|_| format!("HEAD case is not UTF-8: {relative_path}"))?;
        let head_case = Case::parse(head)
            .map_err(|error| format!("parse HEAD case {relative_path}: {error}"))?;
        selected.push(relative_path.clone());
        let mut body = Vec::new();
        let mut previews = Vec::new();
        let mut case_refusal = None;
        let results = drive_replays(&case, &consumer, env, path, &source)?;
        let mut inspected_replays = Vec::new();
        for (index, ((block, head_block), routed)) in case
            .replay()
            .blocks()
            .iter()
            .zip(head_case.replay().blocks())
            .zip(results)
            .enumerate()
        {
            let changed_from_head = block.output() != head_block.output();
            // The committed bytes ARE the render's bytes, so an edit compiles against them
            // directly (`28L:rul-editability-is-stamped-never-re-derived`).
            let dirty = block.output().to_owned();
            if let Some(render) = routed.editable_render().cloned() {
                let baseline = consumer
                    .baseline_from_render(&case, render)
                    .map_err(|error| format!("{}: {error}", path.display()))?;
                if changed_from_head {
                    match compile_preview(&baseline, &dirty) {
                        Ok(preview) => previews.push((index, preview)),
                        Err(error) => case_refusal = Some((index, error, dirty)),
                    }
                }
            } else {
                if block.output() != routed.output() || head_block.output() != routed.output() {
                    case_refusal = Some((
                        index,
                        DorcSectionEditRefusal::Unchanged,
                        "bytes-only replay changed".to_owned(),
                    ));
                }
                // Structure, not a change: a bytes-only replay that actually diverged took the
                // refusal branch above, so quiet loses nothing by dropping the inventory line.
                if !quiet {
                    writeln!(body, "replay: {index} bytes-only")
                        .map_err(|error| error.to_string())?;
                }
            }
            inspected_replays.push((index, block.command().to_owned(), routed));
        }
        if let Some((index, error, dirty)) = case_refusal {
            refused = refused.saturating_add(1);
            writeln!(body, "refusal in replay {index}: {}", error.explain(path))
                .map_err(|write| write.to_string())?;
            writeln!(body, "class: {error:?}").map_err(|write| write.to_string())?;
            writeln!(body, "baseline: exact renderer provenance")
                .map_err(|write| write.to_string())?;
            writeln!(body, "edited:\n{}", bounded_evidence(&dirty))
                .map_err(|write| write.to_string())?;
            emit_case(out, path, &body, quiet)?;
            continue;
        }
        let mut compiled = emit_previews(&mut consumer, previews, path, &mut body)?;
        let replays = inspected_replays
            .into_iter()
            .map(|(index, command, routed)| match routed.editable_render() {
                Some(render) => InspectedReplay::editable(
                    index,
                    command,
                    routed.output().to_owned(),
                    render,
                    &compiled.remove(&index).into_iter().collect::<Vec<_>>(),
                ),
                None => InspectedReplay::bytes(index, command, routed.output().to_owned()),
            })
            .collect();
        emit_case(out, path, &body, quiet)?;
        let is_touched = gated.touched.contains(&relative_path);
        inspected_cases.push((relative_path, source, is_touched, replays));
    }
    if refused > 0 {
        return Ok(Inspected::Refused { cases: refused });
    }
    let catalog = std::fs::read_to_string(catalog_path())
        .map_err(|error| format!("read catalog input: {error}"))?;
    let touched_cases = inspected_cases
        .iter()
        .filter(|(_, _, touched, _)| *touched)
        .map(|(path, _, _, _)| path.clone())
        .collect();
    InspectedCompilation::new(catalog, selected, touched_cases, inspected_cases)
        .map(|inspection| Inspected::Ready(inspection, consumer))
        .map_err(|error| error.to_string())
}

/// Emit each compiled preview, apply it to the mirror (the promote edited-mirror seam), and collect
/// the previews keyed by replay index for receipt inspection.
fn emit_previews(
    consumer: &mut DorcConsumer,
    previews: Vec<(usize, dorc_loom::CompilePreview)>,
    path: &Path,
    out: &mut impl Write,
) -> Result<std::collections::BTreeMap<usize, dorc_loom::CompilePreview>, String> {
    let mut compiled = std::collections::BTreeMap::new();
    for (index, preview) in previews {
        writeln!(out, "replay: {index}").map_err(|error| error.to_string())?;
        let rendered = render_compile_preview(&preview);
        consumer
            .apply_preview(&preview)
            .map_err(|error| format!("{}: {}", path.display(), error.explain(path)))?;
        compiled.insert(index, preview);
        writeln!(out, "{rendered}").map_err(|error| error.to_string())?;
    }
    Ok(compiled)
}

fn receipt_store() -> Result<FsReceiptStore, String> {
    // No `..` components — the store's directory-tree check rejects them (`spike/target`).
    let target = spike_dir()?.join("target");
    FsReceiptStore::new(target)
}

fn catalog_path() -> PathBuf {
    crates_dir().join("aid").join("src").join("catalog_lock.rs")
}

fn arrangement_path() -> PathBuf {
    crates_dir()
        .join("aid")
        .join("src")
        .join("arrangement_lock.rs")
}

fn spike_dir() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "locate spike dir".to_owned())
}

/// The receipt may bind only transcript-prose edits. Repository reads are isolated
/// in `GitRepository`; this command owns only selection and inspection orchestration.
fn gate_touched_set(cases: &[PathBuf]) -> Result<GatedCases, String> {
    let repository = GitRepository::open()?;
    let mut paths: Vec<_> = cases
        .iter()
        .map(|path| {
            repository
                .repository_path(path)
                .map(|relative| (relative, path.clone()))
        })
        .collect::<Result<_, _>>()?;
    paths.sort_by(|left, right| left.0.cmp(&right.0));
    let selected = paths.iter().map(|(path, _)| path.clone()).collect();
    let catalog = repository.repository_path(&catalog_path())?;
    let arrangement = repository.repository_path(&arrangement_path())?;
    let classification = classify_prose_changes(&repository, selected, &catalog, &arrangement)?;
    Ok(GatedCases {
        repository,
        paths,
        touched: classification.touched().clone(),
        staged: classification.staged().clone(),
    })
}

const MAX_REFUSAL_EVIDENCE: usize = 4096;

fn bounded_evidence(text: &str) -> String {
    if text.len() <= MAX_REFUSAL_EVIDENCE {
        return text.to_owned();
    }
    let mut end = MAX_REFUSAL_EVIDENCE;
    while !text.is_char_boundary(end) {
        end = end.saturating_sub(1);
    }
    format!("{}\n[truncated]", &text[..end])
}

fn print_variables(
    used: bool,
    cases: &[PathBuf],
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    let consumer = DorcConsumer::new();
    writeln!(out, "{VALUE_SYNTAX_NOTE}").map_err(|error| error.to_string())?;
    for path in cases {
        let case = load(path)?;
        let baseline = consumer
            .editable_baseline(&case)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
        if used {
            for (name, value) in baseline.used_variables() {
                writeln!(out, "{{{{{}}}}} = {value:?}", name.0)
                    .map_err(|error| error.to_string())?;
            }
        } else {
            for (name, value) in baseline.all_variables() {
                writeln!(out, "{{{{{}}}}} = {value:?}", name.0)
                    .map_err(|error| error.to_string())?;
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

/// `dorc-loom sections CASE...` — per replay, print each editable section's key and its ordered
/// `Text | Variable` fragment series plus the computed spans around it. Drives like `vars`, just
/// without dropping every replay after the first.
fn print_sections(cases: &[PathBuf], out: &mut impl Write) -> Result<ExitCode, String> {
    let consumer = DorcConsumer::new();
    // Which bytes these describe is the one thing a reader can get wrong here, and getting it
    // wrong reads as the tool lying: this is the PUBLISHED baseline an edit is attributed against,
    // never the on-disk transcript.
    writeln!(
        out,
        "sections of the published baseline — the render your edit is attributed against; what \
         you have typed on disk is what `mise run loom:compile` reads"
    )
    .map_err(|error| error.to_string())?;
    writeln!(out, "{VALUE_SYNTAX_NOTE}").map_err(|error| error.to_string())?;
    for path in cases {
        let case = load(path)?;
        writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
        let results = replay_case(&case, &consumer, &RunEnv::new(), |_command, _context| {
            Ok(ReplayResult::bytes(String::new()))
        })
        .map_err(|error| format!("{}: {error}", path.display()))?;
        for (index, result) in results.iter().enumerate() {
            let Some(render) = result.editable_render() else {
                writeln!(out, "replay {index}: bytes-only").map_err(|error| error.to_string())?;
                continue;
            };
            let baseline = consumer
                .baseline_from_render(&case, render.clone())
                .map_err(|error| format!("{}: {error}", path.display()))?;
            writeln!(out, "replay {index}:").map_err(|error| error.to_string())?;
            for component in baseline.render().components() {
                print_component(out, component)?;
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn print_component(
    out: &mut impl Write,
    component: &RenderComponent<SectionKey, SectionVariableId>,
) -> Result<(), String> {
    match component {
        RenderComponent::Structure(text) => {
            writeln!(out, "  computed: {text:?}").map_err(|error| error.to_string())
        }
        RenderComponent::FixedVariable { id, rendered } => {
            writeln!(out, "  computed {{{{{}}}}} = {rendered:?}", id.name.0)
                .map_err(|error| error.to_string())
        }
        RenderComponent::EditableSection(section) => {
            let key = section.id();
            writeln!(
                out,
                "  section {}/{}#{} (segment {}):",
                key.owner, key.field, key.instance, key.segment,
            )
            .map_err(|error| error.to_string())?;
            for fragment in section.fragments() {
                match fragment {
                    EditableFragment::Text(text) => {
                        writeln!(out, "    text: {text:?}").map_err(|error| error.to_string())?;
                    }
                    EditableFragment::Variable { id, rendered } => {
                        writeln!(out, "    var {{{{{}}}}} = {rendered:?}", id.name.0)
                            .map_err(|error| error.to_string())?;
                    }
                }
            }
            Ok(())
        }
    }
}

fn load(path: &Path) -> Result<Case, String> {
    read_case(path).map_err(|error| format!("{}: {error}", path.display()))
}

fn load_with_text(path: &Path) -> Result<(Case, String), String> {
    let source = read_case_text(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let case = Case::parse(&source).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok((case, source))
}

fn validate_case_inputs(cases: &[PathBuf]) -> Result<(), String> {
    for path in cases {
        let _ = load_with_text(path)?;
    }
    Ok(())
}

fn case_name(path: &Path) -> Result<String, String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| format!("{}: case file has no UTF-8 filename", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Under-naming is the failure that matters: a rewritten case's staged bytes are the author's
    /// own pre-promote text, so a bare `git commit` would take those and drop the promotion.
    #[test]
    fn only_a_rewritten_staged_case_is_told_to_restage() {
        let staged =
            std::collections::BTreeSet::from(["kept.loom".to_owned(), "rewritten.loom".to_owned()]);
        let rewritten = std::collections::BTreeSet::from(["rewritten.loom".to_owned()]);
        let mut out = Vec::new();
        note_staged_cases(&staged, &rewritten, &mut out).expect("note");
        let out = String::from_utf8(out).expect("notes are utf-8");
        assert!(
            out.contains("rewritten.loom was staged and has been rewritten"),
            "{out}"
        );
        assert!(out.contains("kept.loom is staged;"), "{out}");
        assert!(!out.contains("kept.loom was staged"), "{out}");
    }

    /// Quiet may drop a header, never a report — the corpus is ~50 cases and all but the edited one
    /// have nothing to say, but a refusal buried in that listing is the whole reason to look.
    #[test]
    fn quiet_drops_only_a_case_that_said_nothing() {
        let mut silent = Vec::new();
        emit_case(&mut silent, Path::new("silent.loom"), b"", true).expect("emit");
        assert!(silent.is_empty());

        let mut speaking = Vec::new();
        emit_case(&mut speaking, Path::new("loud.loom"), b"replay: 0\n", true).expect("emit");
        let speaking = String::from_utf8(speaking).expect("notes are utf-8");
        assert!(speaking.contains("case: "), "{speaking}");
        assert!(speaking.contains("replay: 0"), "{speaking}");

        let mut verbose = Vec::new();
        emit_case(&mut verbose, Path::new("silent.loom"), b"", false).expect("emit");
        assert!(!verbose.is_empty());
    }

    /// `sections` names a computed span, an editable section's key, and its fragment series —
    /// structure, not prose bytes: real catalog wording is free to churn without retouching this.
    #[test]
    fn sections_prints_computed_spans_and_editable_fragment_series() {
        let components = vec![
            RenderComponent::Structure("error[".to_owned()),
            RenderComponent::FixedVariable {
                id: SectionVariableId {
                    name: TemplateVariableName("code".to_owned()),
                    occurrence: 0,
                },
                rendered: "some-code".to_owned(),
            },
            RenderComponent::EditableSection(EditableSection::new(
                SectionKey {
                    owner: "some-code".to_owned(),
                    field: "message",
                    instance: 0,
                    segment: 0,
                },
                vec![
                    EditableFragment::Text("do the ".to_owned()),
                    EditableFragment::Variable {
                        id: SectionVariableId {
                            name: TemplateVariableName("thing".to_owned()),
                            occurrence: 0,
                        },
                        rendered: "widget".to_owned(),
                    },
                    EditableFragment::Text(" now".to_owned()),
                ],
            )),
        ];
        let mut out = Vec::new();
        for component in &components {
            print_component(&mut out, component).expect("print");
        }
        let out = String::from_utf8(out).expect("utf8");
        assert!(out.contains("computed: \"error[\""), "{out}");
        assert!(out.contains("computed {{code}} = \"some-code\""), "{out}");
        assert!(
            out.contains("section some-code/message#0 (segment 0):"),
            "{out}"
        );
        assert!(out.contains("text: \"do the \""), "{out}");
        assert!(out.contains("var {{thing}} = \"widget\""), "{out}");
        assert!(out.contains("text: \" now\""), "{out}");
    }

    /// A hand-seeded row's arity mismatch panics deep inside the shared renderer
    /// (`dorc_aid::arrangement::sentence_words`'s own `debug_assert!`) the first time some case's
    /// render reaches it — a whole-PAGE entry's arity is always "exactly one word", so seeding a
    /// second one reproduces the wiring defect without needing a value-bearing seat. This proves
    /// dorc-loom's own driving boundary catches that panic instead of taking the whole process
    /// down, and reports the row, the diagnosis, and the fix.
    #[test]
    fn a_hand_seeded_arity_mismatch_refuses_instead_of_crashing_the_process() {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests/cli-help-page.loom"),
        )
        .expect("read fixture case");
        let case = Case::parse(&text).expect("case parses");
        let mut consumer = DorcConsumer::new();
        consumer.set_arrangement_words(
            "cli-help-page",
            dorc_aid::arrangement::OwnedWords::Authored(vec![
                "one word".to_owned(),
                "an extra word a page never takes".to_owned(),
            ]),
        );
        let error = drive_replays(
            &case,
            &consumer,
            &RunEnv::new(),
            Path::new("crates/aid/tests/cli-help-page.loom"),
            &text,
        )
        .expect_err("a bad-arity row must refuse, not panic");
        assert!(error.contains("cli-help-page"), "{error}");
        assert!(error.contains("panicked"), "{error}");
        assert!(error.contains("arrangement_lock.rs"), "{error}");
    }
}
