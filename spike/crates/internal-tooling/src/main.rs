//! Internal repo tooling — NOT part of Dorc. Subcommand dispatch; see `Cargo.toml`.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a developer-facing task runner's output IS its product"
)]

use std::process::ExitCode;

mod hook_selftest;

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("hook-selftest") => hook_selftest::run(),
        other => {
            eprintln!(
                "internal-tooling: unknown task {:?}",
                other.unwrap_or("<none>")
            );
            eprintln!("tasks: hook-selftest");
            ExitCode::from(2)
        }
    }
}
