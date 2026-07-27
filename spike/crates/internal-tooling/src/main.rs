//! Internal repo tooling — NOT part of Dorc. Subcommand dispatch; see `Cargo.toml`.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a developer-facing task runner's output IS its product"
)]

use std::process::ExitCode;

mod bless;
mod coverage;
mod hook_selftest;
mod livetest;
mod posix_script;
mod step_globs;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        // Both batteries always run, and the worse verdict wins: each guards a hook failure the
        // other cannot see, so stopping at the first would hide the second's answer.
        Some("hook-selftest") => ExitCode::from(hook_selftest::run().max(step_globs::run())),
        Some("coverage") => coverage::run(args.get(1..).unwrap_or_default()),
        Some("bless") => bless::run(args.get(1..).unwrap_or_default()),
        Some("livetest") => livetest::run(args.get(1..).unwrap_or_default()),
        Some("baselines") => livetest::baselines(args.get(1..).unwrap_or_default()),
        other => {
            eprintln!(
                "internal-tooling: unknown task {:?}",
                other.unwrap_or("<none>")
            );
            eprintln!("tasks: hook-selftest, coverage, bless, livetest, baselines");
            ExitCode::from(2)
        }
    }
}
