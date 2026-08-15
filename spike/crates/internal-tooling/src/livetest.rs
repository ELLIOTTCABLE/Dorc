//! `livetest` — the live-acceptance loop, and the hermetic baselines it asserts against.
//!
//! Both are shell scripts run through [`crate::posix_script`]; nothing here knows what a container
//! is, nor what a baseline says.

use crate::posix_script;
use std::ffi::OsString;
use std::process::ExitCode;

/// Run `spike/e2e/livetest.sh`, forwarding every argument.
pub(crate) fn run(args: &[String]) -> ExitCode {
    posix_script::run(
        "livetest",
        "spike/e2e/livetest.sh",
        args,
        &[("DORC", dorc())],
    )
}

/// Regenerate `Research/trial/r26/renders/` — the baselines `livetest` asserts against.
pub(crate) fn baselines(args: &[String]) -> ExitCode {
    posix_script::run(
        "livetest:baselines",
        "Research/trial/r26/render-baselines.sh",
        args,
        &[("DORC", dorc())],
    )
}

/// The built binary, passed to both scripts rather than left to their own defaults.
///
/// Two reasons, and each script has only one of them covered: their fallbacks spell
/// `spike/target/debug/dorc`, which is wrong wherever `CARGO_TARGET_DIR` moved the build (the
/// WSL leg, now, always), and one of the two spells it without the platform's executable
/// suffix.
fn dorc() -> OsString {
    OsString::from(
        internal_tooling::target_dir()
            .join("debug")
            .join(format!("dorc{}", std::env::consts::EXE_SUFFIX)),
    )
}
