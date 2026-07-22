//! `dorc-loom` is the read-only transcript-template inspection command.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use dorc_loom::{
    DorcConsumer, DorcSectionEditRefusal, compile_preview, render_compile_preview,
    replay_case_with_inputs,
};
use errorloom::{Case, ReplayInput, ReplayResult, RunEnv, execute_generic, read_case};

const USAGE: &str =
    "usage: dorc-loom <compile [--shell=PATH] [--path=DIR]... CASE...|vars <--used|--all> CASE...>";

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
    Vars { used: bool, cases: Vec<PathBuf> },
}

fn run() -> Result<ExitCode, String> {
    let command = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match command {
        Command::Compile { cases, env } => compile_cases(&cases, &env, &mut out),
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
    let consumer = DorcConsumer::new();
    let mut refused = false;
    for path in cases {
        let case = load(path)?;
        writeln!(out, "case: {}", path.display()).map_err(|error| error.to_string())?;
        let mut previews = Vec::new();
        let mut case_refusal = None;
        let input = ReplayInput::new(case_name(path)?, case.to_text())
            .map_err(|error| format!("{}: {error}", path.display()))?;
        let results =
            replay_case_with_inputs(&case, &consumer, env, &[input], |command, context| {
                execute_generic(command, context).map(ReplayResult::bytes)
            })
            .map_err(|error| format!("{}: {error}", path.display()))?;
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
        for (index, preview) in previews {
            writeln!(out, "replay: {index}").map_err(|error| error.to_string())?;
            writeln!(out, "{}", render_compile_preview(&preview))
                .map_err(|error| error.to_string())?;
        }
    }
    Ok(if refused {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
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

fn load(path: &PathBuf) -> Result<Case, String> {
    read_case(path).map_err(|error| format!("{}: {error}", path.display()))
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
