//! `errorloom` — the thin generic CLI (`282` §5 / `28A` §1): the cram mode.
//! `errorloom run` executes cases and reports drift (nonzero on divergence);
//! `errorloom bless` re-runs and re-inlines (structure-bless). The prose-promote
//! flow stays library-only — it needs consumer callbacks the CLI cannot supply.
//!
//! Sharp edges are fine here (`282:rul-internal-tool-sharp-edges`). Output routes
//! through `writeln!` on locked handles so the crate's `print_stdout`/
//! `print_stderr` lints hold without a crate-root `#[expect]`.

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use errorloom::{Case, RunEnv, bless_structure, check_run, read_case, read_case_text};

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            let _ = writeln!(io::stderr(), "errorloom: {message}");
            ExitCode::from(2)
        }
    }
}

enum Mode {
    Run,
    Bless,
}

struct Args {
    mode: Mode,
    env: RunEnv,
    require_token: Option<String>,
    cases: Vec<PathBuf>,
}

const USAGE: &str = "usage: errorloom <run|bless> --shell=PATH [--path=DIR]... [--env=K=V]... [--require-token=KEY] CASE...";

fn run() -> Result<ExitCode, String> {
    let args = parse_args()?;
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match args.mode {
        Mode::Run => run_cases(&args, &mut out),
        Mode::Bless => bless_cases(&args, &mut out),
    }
}

fn run_cases(args: &Args, out: &mut impl Write) -> Result<ExitCode, String> {
    let mut any_drift = false;
    for path in &args.cases {
        let case = load(path)?;
        let report = check_run(&case, &args.env).map_err(|e| e.to_string())?;
        if report.is_clean() {
            writeln!(out, "ok     {}", path.display()).map_err(|e| e.to_string())?;
        } else {
            any_drift = true;
            writeln!(out, "DRIFT  {}", path.display()).map_err(|e| e.to_string())?;
            for drift in report.drifts() {
                writeln!(out, "  block {} `{}`", drift.block(), drift.command())
                    .map_err(|e| e.to_string())?;
                writeln!(out, "  --- expected ---\n{}", drift.expected())
                    .map_err(|e| e.to_string())?;
                writeln!(out, "  --- actual ---\n{}", drift.actual()).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(if any_drift {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn bless_cases(args: &Args, out: &mut impl Write) -> Result<ExitCode, String> {
    for path in &args.cases {
        let source =
            read_case_text(path).map_err(|error| format!("{}: {error}", path.display()))?;
        let mut case =
            Case::parse(&source).map_err(|error| format!("{}: {error}", path.display()))?;
        bless_structure(&mut case, &args.env, args.require_token.as_deref())
            .map_err(|e| e.to_string())?;
        let updated = case.to_text();
        if updated == source {
            writeln!(out, "clean    {}", path.display()).map_err(|e| e.to_string())?;
        } else {
            std::fs::write(path, &updated).map_err(|e| format!("{}: {e}", path.display()))?;
            writeln!(out, "blessed  {}", path.display()).map_err(|e| e.to_string())?;
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn load(path: &PathBuf) -> Result<Case, String> {
    read_case(path).map_err(|error| format!("{}: {error}", path.display()))
}

fn parse_args() -> Result<Args, String> {
    let mut argv = std::env::args().skip(1);
    let mode = match argv.next().as_deref() {
        Some("run") => Mode::Run,
        Some("bless") => Mode::Bless,
        _ => return Err(USAGE.to_owned()),
    };
    let mut env = RunEnv::new();
    let mut require_token: Option<String> = None;
    let mut cases: Vec<PathBuf> = Vec::new();
    while let Some(arg) = argv.next() {
        if let Some(dir) = arg.strip_prefix("--path=") {
            env = env.path_dir(dir);
        } else if arg == "--path" {
            env = env.path_dir(next_value(&mut argv, "--path")?);
        } else if let Some(shell) = arg.strip_prefix("--shell=") {
            env = env.shell(shell);
        } else if arg == "--shell" {
            env = env.shell(next_value(&mut argv, "--shell")?);
        } else if let Some(pair) = arg.strip_prefix("--env=") {
            let (name, value) = split_env(pair)?;
            env = env.var(name, value);
        } else if arg == "--env" {
            let raw = next_value(&mut argv, "--env")?;
            let (name, value) = split_env(&raw)?;
            env = env.var(name, value);
        } else if let Some(key) = arg.strip_prefix("--require-token=") {
            require_token = Some(key.to_owned());
        } else if arg == "--require-token" {
            require_token = Some(next_value(&mut argv, "--require-token")?);
        } else if let Some(flag) = arg.strip_prefix("--") {
            return Err(format!("unknown flag --{flag}\n{USAGE}"));
        } else {
            cases.push(PathBuf::from(arg));
        }
    }
    if cases.is_empty() {
        return Err(format!("no case files given\n{USAGE}"));
    }
    Ok(Args {
        mode,
        env,
        require_token,
        cases,
    })
}

fn next_value(argv: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    argv.next().ok_or_else(|| format!("{flag} needs a value"))
}

fn split_env(pair: &str) -> Result<(String, String), String> {
    pair.split_once('=')
        .map(|(name, value)| (name.to_owned(), value.to_owned()))
        .ok_or_else(|| format!("--env expects NAME=VALUE, got {pair:?}"))
}
