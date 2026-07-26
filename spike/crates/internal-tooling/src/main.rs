//! Internal repo tooling — NOT part of Dorc. Subcommand dispatch; see `Cargo.toml`.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a developer-facing task runner's output IS its product"
)]

use std::process::ExitCode;

mod coverage;
mod hook_selftest;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("hook-selftest") => hook_selftest::run(),
        Some("coverage") => coverage::run(args.get(1..).unwrap_or_default()),
        other => {
            eprintln!(
                "internal-tooling: unknown task {:?}",
                other.unwrap_or("<none>")
            );
            eprintln!("tasks: hook-selftest, coverage");
            ExitCode::from(2)
        }
    }
}
