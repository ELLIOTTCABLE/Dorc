//! Internal repo tooling — NOT part of Dorc. Subcommand dispatch; see `Cargo.toml`.
#![expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "a developer-facing task runner's output IS its product"
)]

use std::process::ExitCode;

mod bless;
mod coverage;
mod docids;
mod doctor;
mod fmt_detached;
mod hook_selftest;
mod livetest;
mod posix_script;
mod preflight;
mod prose_census;
mod step_globs;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        // Both batteries always run, and the worse verdict wins: each guards a hook failure the
        // other cannot see, so stopping at the first would hide the second's answer.
        Some("hook-selftest") => ExitCode::from(hook_selftest::run().max(step_globs::run())),
        Some("coverage") => coverage::run(args.get(1..).unwrap_or_default()),
        Some("prose-census") => prose_census::run(),
        Some("bless") => bless::run(args.get(1..).unwrap_or_default()),
        Some("livetest") => livetest::run(args.get(1..).unwrap_or_default()),
        Some("baselines") => livetest::baselines(args.get(1..).unwrap_or_default()),
        Some("preflight") => preflight::run(args.get(1..).unwrap_or_default()),
        Some("fmt-detached") => fmt_detached::run(args.get(1..).unwrap_or_default()),
        Some("docids") => docids::run(),
        Some("doctor") => doctor::run(args.get(1..).unwrap_or_default()),
        // The rendered inventory only; the GATE is `xfail_census_is_coherent` in the lib, and this
        // shares its one renderer rather than re-deriving the screen.
        Some("xfail-census") => {
            print!("{}", internal_tooling::xfail::census_report());
            ExitCode::SUCCESS
        }
        other => {
            eprintln!(
                "internal-tooling: unknown task {:?}",
                other.unwrap_or("<none>")
            );
            eprintln!(
                "tasks: hook-selftest, prose-census, coverage, bless, livetest, baselines, \
                 preflight, doctor, xfail-census, fmt-detached, docids"
            );
            ExitCode::from(2)
        }
    }
}
