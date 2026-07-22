//! `dorc-loom` is the read-only transcript-template inspection command.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, ExitCode};

use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, FsReceiptStore, InspectedCompilation, InspectedReplay,
    ReceiptStore, compile_preview, encode_receipt, render_compile_preview, replay_case_with_inputs,
    validate_receipt,
};
use errorloom::{
    Case, ReplayInput, ReplayResult, RunEnv, execute_generic, read_case, read_case_text,
};

const USAGE: &str = "usage: dorc-loom <compile|promote [--shell=PATH] [--path=DIR]... CASE...|vars <--used|--all> CASE...>";

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
}

fn run() -> Result<ExitCode, String> {
    let command = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match command {
        Command::Compile { cases, env } => compile_cases(&cases, &env, &mut out),
        Command::Promote { cases, env } => promote_cases(&cases, &env, &mut out),
        Command::Vars { used, cases } => print_variables(used, &cases, &mut out),
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
        _ => Err(USAGE.to_owned()),
    }
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
    let touched = gate_touched_set(cases)?;
    let inspection = inspect_cases(cases, &touched, env, out)?;
    let Some(inspection) = inspection else {
        return Ok(ExitCode::from(1));
    };
    let packet = encode_receipt(&inspection).map_err(|error| error.to_string())?;
    receipt_store().publish(&packet)?;
    Ok(ExitCode::SUCCESS)
}

fn promote_cases(
    cases: &[PathBuf],
    env: &RunEnv,
    out: &mut impl Write,
) -> Result<ExitCode, String> {
    let packet = receipt_store()
        .read()
        .map_err(|error| format!("promote receipt: {error}"))?;
    validate_case_inputs(cases)?;
    let touched = gate_touched_set(cases)?;
    let inspection = inspect_cases(cases, &touched, env, out)?;
    let Some(inspection) = inspection else {
        return Ok(ExitCode::from(1));
    };
    validate_receipt(&packet, &inspection).map_err(|error| format!("promote refused: {error}"))?;
    writeln!(
        out,
        "promote: receipt matches current inspected compilation; ready"
    )
    .map_err(|error| error.to_string())?;
    Ok(ExitCode::SUCCESS)
}

fn inspect_cases(
    cases: &[PathBuf],
    touched: &std::collections::BTreeSet<String>,
    env: &RunEnv,
    out: &mut impl Write,
) -> Result<Option<InspectedCompilation>, String> {
    let consumer = DorcConsumer::new();
    let mut refused = false;
    let mut paths: Vec<_> = cases
        .iter()
        .map(|path| Ok((canonical_case_path(path)?, path)))
        .collect::<Result<_, String>>()?;
    paths.sort_by(|left, right| left.0.cmp(&right.0));
    if paths.windows(2).any(|pair| {
        pair.first()
            .zip(pair.get(1))
            .is_some_and(|(left, right)| left.0 == right.0)
    }) {
        return Err("duplicate selected case".to_owned());
    }
    let mut selected = Vec::new();
    let mut inspected_cases = Vec::new();
    for (relative_path, path) in paths {
        let (case, source) = load_with_text(path)?;
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
        for (index, (block, routed)) in case.replay().blocks().iter().zip(results).enumerate() {
            let dirty = unreflow(block.output());
            if let Some(render) = routed.editable_render().cloned() {
                let baseline = consumer
                    .baseline_from_render(&case, render)
                    .map_err(|error| format!("{}: {error}", path.display()))?;
                match compile_preview(&baseline, &dirty) {
                    Ok(preview) => previews.push((index, preview)),
                    Err(DorcSectionEditRefusal::Unchanged) => {}
                    Err(error) => case_refusal = Some((index, error, dirty)),
                }
            } else {
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
        let mut compiled = std::collections::BTreeMap::new();
        for (index, preview) in previews {
            writeln!(out, "replay: {index}").map_err(|error| error.to_string())?;
            let rendered = render_compile_preview(&preview);
            compiled.insert(index, preview);
            writeln!(out, "{rendered}").map_err(|error| error.to_string())?;
        }
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
        .map(Some)
        .map_err(|error| error.to_string())
}

fn receipt_store() -> FsReceiptStore {
    FsReceiptStore::new(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/dorc-loom/compile.receipt"),
    )
}

fn catalog_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../core/src/catalog.rs")
}

fn canonical_case_path(path: &Path) -> Result<String, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let absolute =
        std::fs::canonicalize(path).map_err(|error| format!("canonicalize case: {error}"))?;
    let root =
        std::fs::canonicalize(root).map_err(|error| format!("canonicalize repository: {error}"))?;
    let relative = absolute
        .strip_prefix(root)
        .map_err(|_| "case is outside spike worktree".to_owned())?;
    let path = relative.to_string_lossy().replace('\\', "/");
    if path.is_empty() || path.contains("../") {
        return Err("unsafe case path".to_owned());
    }
    Ok(path)
}

/// The receipt may bind only transcript-prose edits. Git is deliberately queried
/// at this CLI edge; the classification itself is a closed, deterministic policy.
fn gate_touched_set(cases: &[PathBuf]) -> Result<std::collections::BTreeSet<String>, String> {
    let root = git_root()?;
    let selected: std::collections::BTreeSet<_> = cases
        .iter()
        .map(|path| repo_path(&root, path))
        .collect::<Result<_, _>>()?;
    let catalog = repo_path(&root, &catalog_path())?;
    let mut touched = std::collections::BTreeSet::new();
    for (index, worktree, path) in git_status(&root)? {
        if path == catalog {
            return Err("catalog is not clean against HEAD".to_owned());
        }
        if !selected.contains(&path) || index != ' ' || !matches!(worktree, ' ' | 'M') {
            return Err(format!("dirty path outside selected prose edits: {path}"));
        }
        if worktree == 'M' {
            validate_prose_only(&root, &path)?;
            touched.insert(path);
        }
    }
    Ok(touched)
}

fn git_root() -> Result<PathBuf, String> {
    let output = ProcessCommand::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|error| format!("locate git repository: {error}"))?;
    if !output.status.success() {
        return Err("dorc-loom requires a git repository".to_owned());
    }
    let text = String::from_utf8(output.stdout).map_err(|_| "git root is not UTF-8".to_owned())?;
    Ok(PathBuf::from(text.trim()))
}

fn repo_path(root: &Path, path: &Path) -> Result<String, String> {
    let path =
        std::fs::canonicalize(path).map_err(|error| format!("canonicalize path: {error}"))?;
    let root =
        std::fs::canonicalize(root).map_err(|error| format!("canonicalize git root: {error}"))?;
    let relative = path
        .strip_prefix(root)
        .map_err(|_| "path is outside git repository".to_owned())?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn git_status(root: &Path) -> Result<Vec<(char, char, String)>, String> {
    let output = ProcessCommand::new("git")
        .args(["status", "--porcelain=v1", "-z"])
        .current_dir(root)
        .output()
        .map_err(|error| format!("read git status: {error}"))?;
    if !output.status.success() {
        return Err("read git status failed".to_owned());
    }
    let mut entries = Vec::new();
    for record in output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|record| !record.is_empty())
    {
        let Some((&index_byte, rest)) = record.split_first() else {
            return Err("malformed git porcelain status".to_owned());
        };
        let Some((&worktree_byte, rest)) = rest.split_first() else {
            return Err("malformed git porcelain status".to_owned());
        };
        let Some((&separator, path)) = rest.split_first() else {
            return Err("malformed git porcelain status".to_owned());
        };
        if separator != b' ' {
            return Err("malformed git porcelain status".to_owned());
        }
        let index = char::from(index_byte);
        let worktree = char::from(worktree_byte);
        let path = std::str::from_utf8(path)
            .map_err(|_| "git status path is not UTF-8".to_owned())?
            .to_owned();
        // Rename/copy records have a second NUL-delimited source path. They are rejected
        // by their XY class before it can be mistaken for an ordinary prose modification.
        entries.push((index, worktree, path));
    }
    Ok(entries)
}

fn validate_prose_only(root: &Path, path: &str) -> Result<(), String> {
    let current = std::fs::read_to_string(root.join(path))
        .map_err(|error| format!("read selected case: {error}"))?;
    let output = ProcessCommand::new("git")
        .args(["show", &format!("HEAD:{path}")])
        .current_dir(root)
        .output()
        .map_err(|error| format!("read HEAD case: {error}"))?;
    if !output.status.success() {
        return Err("selected case is not present in HEAD".to_owned());
    }
    let head = String::from_utf8(output.stdout).map_err(|_| "HEAD case is not UTF-8".to_owned())?;
    let current_case =
        Case::parse(&current).map_err(|error| format!("parse selected case: {error}"))?;
    let head_case = Case::parse(&head).map_err(|error| format!("parse HEAD case: {error}"))?;
    if current_case
        .sections()
        .iter()
        .map(|section| (section.name(), section.content()))
        .ne(head_case
            .sections()
            .iter()
            .map(|section| (section.name(), section.content())))
        || current_case
            .replay()
            .blocks()
            .iter()
            .map(errorloom::ReplayBlock::command)
            .ne(head_case
                .replay()
                .blocks()
                .iter()
                .map(errorloom::ReplayBlock::command))
        || current_case.frontmatter().scalar("code") != head_case.frontmatter().scalar("code")
    {
        return Err(format!("selected case has non-prose changes: {path}"));
    }
    Ok(())
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
