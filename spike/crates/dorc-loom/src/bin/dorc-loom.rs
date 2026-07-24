//! `dorc-loom` is the read-only transcript-template inspection command.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, FsReceiptStore, GitRepository, InspectedCompilation,
    InspectedReplay, Repository, build_publication, classify_prose_changes, compile_preview,
    compile_receipt, load_corpus_by_slug, promote_receipt, render_compile_preview,
    replay_case_with_inputs,
};
use errorloom::{
    Case, ReplayInput, ReplayResult, RunEnv, execute_generic, read_case, read_case_text,
};

const USAGE: &str = "usage: dorc-loom <compile|promote [--shell=PATH] [--path=DIR]... CASE...|vars <--used|--all> CASE...|scaffold SLUG>";

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
    Compile { cases: Vec<PathBuf>, env: RunEnv },
    Promote { cases: Vec<PathBuf>, env: RunEnv },
    Vars { used: bool, cases: Vec<PathBuf> },
    Scaffold { slug: String },
}

type SelectedCase = (String, PathBuf);

struct GatedCases {
    repository: GitRepository,
    paths: Vec<SelectedCase>,
    touched: std::collections::BTreeSet<String>,
}

fn run() -> Result<ExitCode, String> {
    let command = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match command {
        Command::Compile { cases, env } => compile_cases(&cases, &env, &mut out),
        Command::Promote { cases, env } => promote_cases(&cases, &env, &mut out),
        Command::Vars { used, cases } => print_variables(used, &cases, &mut out),
        Command::Scaffold { slug } => scaffold_case(&slug, &mut out),
    }
}

fn parse_args() -> Result<Command, String> {
    let mut argv = std::env::args().skip(1);
    match argv.next().as_deref() {
        Some("compile") => {
            let (cases, env) = collect_compile_args(argv)?;
            Ok(Command::Compile { cases, env })
        }
        Some("promote") => {
            let (cases, env) = collect_compile_args(argv)?;
            Ok(Command::Promote { cases, env })
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
        _ => Err(USAGE.to_owned()),
    }
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
) -> Result<(Vec<PathBuf>, RunEnv), String> {
    let mut env = RunEnv::new().path_dir(binary_dir()?);
    let mut cases = Vec::new();
    while let Some(arg) = argv.next() {
        if let Some(shell) = arg.strip_prefix("--shell=") {
            env = env.shell(shell);
        } else if let Some(path) = arg.strip_prefix("--path=") {
            env = env.path_dir(path);
        } else if arg == "--shell" {
            env = env.shell(next_value(&mut argv, "--shell")?);
        } else if arg == "--path" {
            env = env.path_dir(next_value(&mut argv, "--path")?);
        } else if arg.starts_with('-') {
            return Err(format!("unknown option {arg:?}\n{USAGE}"));
        } else {
            cases.push(PathBuf::from(arg));
        }
    }
    if cases.is_empty() {
        return Err(format!("no case files given\n{USAGE}"));
    }
    Ok((cases, env))
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
        cases.push(PathBuf::from(arg));
    }
    if cases.is_empty() {
        return Err(format!("no case files given\n{USAGE}"));
    }
    Ok(cases)
}

fn compile_cases(
    cases: &[PathBuf],
    env: &RunEnv,
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    validate_case_inputs(cases)?;
    let gated = gate_touched_set(cases)?;
    let inspection = inspect_cases(&gated.repository, &gated.paths, &gated.touched, env, out)?;
    let Some((inspection, _consumer)) = inspection else {
        return Ok(ExitCode::from(1));
    };
    let outcome = compile_receipt(&receipt_store()?, &inspection)?;
    if matches!(outcome, dorc_loom::ReceiptWriteOutcome::CleanupPending) {
        writeln!(
            out,
            "compile: receipt published; retained backup requires deliberate resolution; subsequent writes refuse"
        )
        .map_err(|error| error.to_string())?;
    }
    Ok(ExitCode::SUCCESS)
}

fn promote_cases(
    cases: &[PathBuf],
    env: &RunEnv,
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    validate_case_inputs(cases)?;
    let gated = gate_touched_set(cases)?;
    let inspection = inspect_cases(&gated.repository, &gated.paths, &gated.touched, env, out)?;
    let Some((inspection, consumer)) = inspection else {
        return Ok(ExitCode::from(1));
    };
    promote_receipt(&receipt_store()?, &inspection)?;
    let affected = touched_cases(&gated)?;
    publish(&consumer, &affected, out)?;
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
            .ok_or_else(|| format!("touched case {} has no `code`", path.display()))?
            .to_owned();
        cases.insert(slug, case);
    }
    Ok(cases)
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
    let publication = build_publication(consumer, &corpus, affected)?;

    let mut wrote = false;
    let lock_path = catalog_path();
    if file_differs(&lock_path, &publication.lock) {
        publish_file(&lock_path, &publication.lock)?;
        writeln!(out, "promote: wrote {}", lock_path.display()).map_err(|e| e.to_string())?;
        wrote = true;
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("cases")
}

fn inspect_cases(
    repository: &GitRepository,
    paths: &[SelectedCase],
    touched: &std::collections::BTreeSet<String>,
    env: &RunEnv,
    out: &mut impl Write,
) -> Result<Option<(InspectedCompilation, DorcConsumer)>, String> {
    let (mut consumer, mut refused, mut selected) = (DorcConsumer::new(), false, Vec::new());
    let mut inspected_cases = Vec::new();
    for (relative_path, path) in paths {
        let relative_path = relative_path.clone();
        let (case, source) = load_with_text(path)?;
        let head = repository.head_bytes(&relative_path)?;
        let head = std::str::from_utf8(&head)
            .map_err(|_| format!("HEAD case is not UTF-8: {relative_path}"))?;
        let head_case = Case::parse(head)
            .map_err(|error| format!("parse HEAD case {relative_path}: {error}"))?;
        selected.push(relative_path.clone());
        writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
        let mut previews = Vec::new();
        let mut case_refusal = None;
        let input = ReplayInput::new(case_name(path)?, source.clone())
            .map_err(|error| format!("{}: {error}", path.display()))?;
        let results =
            replay_case_with_inputs(&case, &consumer, env, &[input], |command, context| {
                execute_generic(command, context).map(ReplayResult::bytes)
            })
            .map_err(|error| format!("{}: {error}", path.display()))?;
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
            let dirty = unreflow(block.output());
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
                writeln!(out, "replay: {index} bytes-only").map_err(|error| error.to_string())?;
            }
            inspected_replays.push((index, block.command().to_owned(), routed));
        }
        if let Some((index, error, dirty)) = case_refusal {
            refused = true;
            writeln!(out, "refusal in replay {index}: {error:?}")
                .map_err(|write| write.to_string())?;
            writeln!(out, "baseline: exact renderer provenance")
                .map_err(|write| write.to_string())?;
            writeln!(out, "edited:\n{}", bounded_evidence(&dirty))
                .map_err(|write| write.to_string())?;
            continue;
        }
        let mut compiled = emit_previews(&mut consumer, previews, path, out)?;
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
        let is_touched = touched.contains(&relative_path);
        inspected_cases.push((relative_path, source, is_touched, replays));
    }
    if refused {
        return Ok(None);
    }
    let catalog = std::fs::read_to_string(catalog_path())
        .map_err(|error| format!("read catalog input: {error}"))?;
    let touched_cases = inspected_cases
        .iter()
        .filter(|(_, _, touched, _)| *touched)
        .map(|(path, _, _, _)| path.clone())
        .collect();
    InspectedCompilation::new(catalog, selected, touched_cases, inspected_cases)
        .map(|inspection| Some((inspection, consumer)))
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
            .map_err(|error| format!("{}: apply compiled section: {error:?}", path.display()))?;
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
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/src/catalog_lock.rs")
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
    let classification = classify_prose_changes(&repository, selected, &catalog)?;
    Ok(GatedCases {
        repository,
        paths,
        touched: classification.touched().clone(),
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

// case files wrap prose; core tags do not
fn unreflow(render: &str) -> String {
    let mut lines = render.lines().peekable();
    let mut out = Vec::new();
    if let Some(first) = lines.next() {
        out.push(join_continuations(first, &mut lines, "   "));
    }
    while let Some(line) = lines.next() {
        if line.trim_start().starts_with("= ") {
            out.push(normalize_layout(&join_continuations(
                line, &mut lines, "      ",
            )));
        } else {
            out.push(normalize_layout(line));
        }
    }
    out.join("\n")
}

fn normalize_layout(line: &str) -> String {
    if let Some(rest) = line.strip_prefix("   = ") {
        return format!("  = {rest}");
    }
    if let Some(rest) = line.strip_prefix("  ")
        && rest
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
    {
        return format!(" {rest}");
    }
    line.to_owned()
}

fn join_continuations<'a>(
    first: &str,
    lines: &mut std::iter::Peekable<impl Iterator<Item = &'a str>>,
    indent: &str,
) -> String {
    let mut joined = first.to_owned();
    while lines
        .peek()
        .is_some_and(|line| line.starts_with(indent) && !line.trim_start().starts_with("-->"))
    {
        let line = lines.next().unwrap_or_default();
        joined.push(' ');
        joined.push_str(line.trim());
    }
    joined
}
