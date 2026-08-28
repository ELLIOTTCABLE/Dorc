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
    let (code, output) = run_directives(&args, Vec::new())?;
    publish_direct(output)?;
    Ok(code)
}

fn run_shell(command: &str) -> Result<ExitCode, String> {
    let (command, merged) = command
        .strip_prefix("exec 2>&1\n")
        .map_or((command, false), |command| (command, true));
    let words = command.split_ascii_whitespace().collect::<Vec<_>>();
    let mut args = Vec::new();
    let mut stdin = Vec::new();
    let mut stdout = ShellDestination::Stdout;
    let mut stderr = if merged {
        ShellDestination::Stdout
    } else {
        ShellDestination::Stderr
    };
    let mut index = 0usize;
    while let Some(word) = words.get(index).copied() {
        if word == "2>&1" {
            stderr = stdout.clone();
            index = index.saturating_add(1);
            continue;
        }
        if let Some((kind, attached)) = shell_redirection(word) {
            let target = if let Some(target) = attached {
                target
            } else {
                index = index.saturating_add(1);
                words
                    .get(index)
                    .copied()
                    .ok_or_else(|| format!("missing redirection target in {command:?}"))?
            };
            match kind {
                ShellRedirection::Input => {
                    stdin = fs::read(target).map_err(|error| error.to_string())?;
                }
                ShellRedirection::Stdout => {
                    fs::File::create(target).map_err(|error| error.to_string())?;
                    stdout = ShellDestination::File(target.to_owned());
                }
                ShellRedirection::Stderr => {
                    fs::File::create(target).map_err(|error| error.to_string())?;
                    stderr = ShellDestination::File(target.to_owned());
                }
            }
            index = index.saturating_add(1);
            continue;
        }
        args.push(word.to_owned());
        index = index.saturating_add(1);
    }
    if args.first().map(String::as_str) != Some("loom-mock-tool") {
        publish_shell(
            vec![DirectiveOutput::Stderr(
                format!("unsupported shell command {command:?}\n").into_bytes(),
            )],
            &stdout,
            &stderr,
        )?;
        return Ok(ExitCode::from(2));
    }
    let (code, output) = run_directives(args.get(1..).unwrap_or_default(), stdin)?;
    publish_shell(output, &stdout, &stderr)?;
    Ok(code)
}

fn run_directives(
    args: &[String],
    stdin_bytes: Vec<u8>,
) -> Result<(ExitCode, Vec<DirectiveOutput>), String> {
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

    let mut output = Vec::new();
    let mut exit: u8 = 0;

    for arg in args {
        if let Some(text) = arg.strip_prefix("out:") {
            output.push(DirectiveOutput::Stdout(format!("{text}\n").into_bytes()));
        } else if let Some(text) = arg.strip_prefix("err:") {
            output.push(DirectiveOutput::Stderr(format!("{text}\n").into_bytes()));
        } else if let Some(count) = arg.strip_prefix("repeat:") {
            let count = count.parse::<usize>().map_err(|e| e.to_string())?;
            output.push(DirectiveOutput::Stdout(vec![b'x'; count]));
        } else if let Some(name) = arg.strip_prefix("env:") {
            let value = env::var(name).unwrap_or_default();
            output.push(DirectiveOutput::Stdout(format!("{value}\n").into_bytes()));
        } else if let Some(file) = arg.strip_prefix("read:") {
            let content = fs::read(file).map_err(|e| e.to_string())?;
            output.push(DirectiveOutput::Stdout(content));
        } else if let Some(file) = arg.strip_prefix("write:") {
            fs::write(file, &stdin_bytes).map_err(|e| e.to_string())?;
        } else if arg == "cat" {
            output.push(DirectiveOutput::Stdout(stdin_bytes.clone()));
        } else if arg == "cwd" {
            let cwd = env::current_dir().map_err(|e| e.to_string())?;
            output.push(DirectiveOutput::Stdout(
                format!("{}\n", cwd.display()).into_bytes(),
            ));
        } else if let Some(code) = arg.strip_prefix("rc:") {
            exit = code.parse::<u8>().map_err(|e| e.to_string())?;
        } else {
            return Err(format!("unknown directive {arg:?}"));
        }
    }
    Ok((ExitCode::from(exit), output))
}

#[derive(Clone)]
enum ShellDestination {
    Stdout,
    Stderr,
    File(String),
}

enum DirectiveOutput {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
}

#[derive(Clone, Copy)]
enum ShellRedirection {
    Input,
    Stdout,
    Stderr,
}

fn shell_redirection(word: &str) -> Option<(ShellRedirection, Option<&str>)> {
    for (prefix, kind) in [
        ("2>", ShellRedirection::Stderr),
        ("1>", ShellRedirection::Stdout),
        (">", ShellRedirection::Stdout),
        ("<", ShellRedirection::Input),
    ] {
        if let Some(target) = word.strip_prefix(prefix) {
            return Some((kind, (!target.is_empty()).then_some(target)));
        }
    }
    None
}

fn publish_direct(output: Vec<DirectiveOutput>) -> Result<(), String> {
    for event in output {
        match event {
            DirectiveOutput::Stdout(bytes) => io::stdout()
                .lock()
                .write_all(&bytes)
                .map_err(|error| error.to_string())?,
            DirectiveOutput::Stderr(bytes) => io::stderr()
                .lock()
                .write_all(&bytes)
                .map_err(|error| error.to_string())?,
        }
    }
    Ok(())
}

fn publish_shell(
    output: Vec<DirectiveOutput>,
    stdout: &ShellDestination,
    stderr: &ShellDestination,
) -> Result<(), String> {
    for event in output {
        let (destination, bytes) = match event {
            DirectiveOutput::Stdout(bytes) => (stdout, bytes),
            DirectiveOutput::Stderr(bytes) => (stderr, bytes),
        };
        match destination {
            ShellDestination::Stdout => io::stdout()
                .lock()
                .write_all(&bytes)
                .map_err(|error| error.to_string())?,
            ShellDestination::Stderr => io::stderr()
                .lock()
                .write_all(&bytes)
                .map_err(|error| error.to_string())?,
            ShellDestination::File(path) => fs::OpenOptions::new()
                .append(true)
                .open(path)
                .and_then(|mut file| file.write_all(&bytes))
                .map_err(|error| error.to_string())?,
        }
    }
    Ok(())
}
