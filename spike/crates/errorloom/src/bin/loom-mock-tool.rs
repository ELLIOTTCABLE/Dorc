//! `loom-mock-tool` — a deterministic echo tool for errorloom's replay-runner
//! self-tests (`282` §7). It stands in for a real consumer CLI: no shell, no real
//! tools, cross-platform, and located by tests via `env!("CARGO_BIN_EXE_...")`.
//!
//! Each argument is a directive, applied in order:
//! `out:TEXT` / `err:TEXT` write a line to stdout / stderr; `env:NAME` prints an
//! injected env var (proving `RunEnv` injection); `cat` copies stdin; `write:FILE`
//! saves stdin to a cwd-relative file and `read:FILE` prints it (proving state
//! flows through the shared cwd); `cwd` prints the absolute cwd (to exercise the
//! sandbox-leak gate); `rc:N` sets the exit code.
//!
//! Output routes through `writeln!`/`write_all` on locked handles rather than the
//! `print*!` macros so the crate's `print_stdout`/`print_stderr` lints hold
//! without a crate-root `#[expect]`.

use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            let _ = writeln!(io::stderr(), "loom-mock-tool: {message}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode, String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if let [flag, command] = args.as_slice()
        && flag == "-c"
    {
        return run_shell(command);
    }
    run_directives(&args, Vec::new(), false)
}

fn run_shell(command: &str) -> Result<ExitCode, String> {
    let command = command.strip_prefix("exec 2>&1\n").unwrap_or(command);
    let mut words = command.split_ascii_whitespace().collect::<Vec<_>>();
    if words.first() != Some(&"loom-mock-tool") {
        let stdout = io::stdout();
        writeln!(stdout.lock(), "unsupported shell command {command:?}")
            .map_err(|e| e.to_string())?;
        return Ok(ExitCode::from(2));
    }
    words.remove(0);
    let stdin = if let Some(position) = words.iter().position(|word| *word == "<") {
        let path = words
            .get(position.saturating_add(1))
            .ok_or_else(|| format!("missing stdin path in {command:?}"))?
            .to_string();
        words.truncate(position);
        fs::read(path).map_err(|error| error.to_string())?
    } else {
        Vec::new()
    };
    run_directives(
        &words.into_iter().map(String::from).collect::<Vec<_>>(),
        stdin,
        true,
    )
}

fn run_directives(
    args: &[String],
    stdin_bytes: Vec<u8>,
    stderr_to_stdout: bool,
) -> Result<ExitCode, String> {
    let needs_stdin = args.iter().any(|a| a == "cat" || a.starts_with("write:"));
    let stdin_bytes = if needs_stdin && stdin_bytes.is_empty() {
        let mut buf = Vec::new();
        io::stdin()
            .read_to_end(&mut buf)
            .map_err(|e| e.to_string())?;
        buf
    } else {
        stdin_bytes
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let stderr = io::stderr();
    let mut err = stderr.lock();
    let mut exit: u8 = 0;

    for arg in args {
        if let Some(text) = arg.strip_prefix("out:") {
            writeln!(out, "{text}").map_err(|e| e.to_string())?;
        } else if let Some(text) = arg.strip_prefix("err:") {
            if stderr_to_stdout {
                writeln!(out, "{text}").map_err(|e| e.to_string())?;
            } else {
                writeln!(err, "{text}").map_err(|e| e.to_string())?;
            }
        } else if let Some(count) = arg.strip_prefix("repeat:") {
            let count = count.parse::<usize>().map_err(|e| e.to_string())?;
            for _ in 0..count {
                out.write_all(b"x").map_err(|e| e.to_string())?;
            }
        } else if let Some(name) = arg.strip_prefix("env:") {
            let value = env::var(name).unwrap_or_default();
            writeln!(out, "{value}").map_err(|e| e.to_string())?;
        } else if let Some(file) = arg.strip_prefix("read:") {
            let content = fs::read(file).map_err(|e| e.to_string())?;
            out.write_all(&content).map_err(|e| e.to_string())?;
        } else if let Some(file) = arg.strip_prefix("write:") {
            fs::write(file, &stdin_bytes).map_err(|e| e.to_string())?;
        } else if arg == "cat" {
            out.write_all(&stdin_bytes).map_err(|e| e.to_string())?;
        } else if arg == "cwd" {
            let cwd = env::current_dir().map_err(|e| e.to_string())?;
            writeln!(out, "{}", cwd.display()).map_err(|e| e.to_string())?;
        } else if let Some(code) = arg.strip_prefix("rc:") {
            exit = code.parse::<u8>().map_err(|e| e.to_string())?;
        } else {
            return Err(format!("unknown directive {arg:?}"));
        }
    }
    out.flush().map_err(|e| e.to_string())?;
    err.flush().map_err(|e| e.to_string())?;
    Ok(ExitCode::from(exit))
}
