//! `cargo fmt` for the verification units the workspace fmt gate cannot reach.
//!
//! `spike/verify/kani` and `spike/verify/aeneas` each declare their own empty `[workspace]` —
//! Kani's nightly and aeneas's shadowing rustc pin each force it — so the `cargo fmt --all
//! --manifest-path spike/Cargo.toml` in `hk.pkl` never sees them, while that step's
//! `spike/**/*.rs` glob happily MATCHES their files. Editing one therefore drew a green fmt
//! gate over unformatted code, and the failure direction was open: nothing printed, nothing
//! refused. The kani unit's `extern crate` block had sat unsorted for the life of the crate.
//!
//! Why one process rather than one hk step per manifest: each unit `#[path]`-includes the real
//! algebra sources, so every run here also walks a dozen `crates/` files the workspace step
//! owns. Running them in series keeps two rustfmt processes off one path in fix mode, and the
//! step's `depends` on `cargo_fmt` closes the same hazard against the workspace run itself.

use std::process::{Command, ExitCode};

/// Every cargo manifest outside `spike/Cargo.toml`'s workspace. A new detached unit is added
/// HERE — this list is the whole of the fmt gate's knowledge that they exist.
const DETACHED: [&str; 2] = [
    "spike/verify/kani/Cargo.toml",
    "spike/verify/aeneas/Cargo.toml",
];

pub(crate) fn run(args: &[String]) -> ExitCode {
    let checking = args.iter().any(|arg| arg == "--check");
    let mut refused = Vec::new();
    for manifest in DETACHED {
        let mut cargo = Command::new("cargo");
        cargo.current_dir(internal_tooling::repo_root()).args([
            "fmt",
            "--all",
            "--manifest-path",
            manifest,
        ]);
        if checking {
            cargo.arg("--check");
        }
        match cargo.status() {
            // An absent cargo is the lane failing to run, never a verdict about the sources.
            Err(why) => {
                eprintln!("fmt-detached: could not run cargo fmt for {manifest}: {why}");
                return ExitCode::from(2);
            }
            Ok(status) if status.success() => {}
            Ok(_) => refused.push(manifest),
        }
    }
    if refused.is_empty() {
        return ExitCode::SUCCESS;
    }
    for manifest in &refused {
        eprintln!("fmt-detached: {manifest} is unformatted (rustfmt's diff is above)");
    }
    eprintln!("fmt-detached: `mise run fmt` repairs it");
    ExitCode::from(1)
}
