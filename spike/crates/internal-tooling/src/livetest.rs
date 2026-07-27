//! `livetest` — the live-acceptance loop, and the hermetic baselines it asserts against.
//!
//! Both are shell scripts run through [`crate::posix_script`]; nothing here knows what a container
//! is, nor what a baseline says.

use crate::posix_script;
use std::ffi::OsString;
use std::process::ExitCode;

/// Run `spike/e2e/livetest.sh`, forwarding every argument.
pub(crate) fn run(args: &[String]) -> ExitCode {
    posix_script::run("livetest", "spike/e2e/livetest.sh", args, &[])
}

/// Regenerate `Research/trial/r26/renders/` — the baselines `livetest` asserts against.
///
/// `DORC` is passed explicitly rather than left to the script's default, which spells the binary
/// without a platform executable suffix.
pub(crate) fn baselines(args: &[String]) -> ExitCode {
    let dorc = internal_tooling::repo_root()
        .join("spike/target/debug")
        .join(format!("dorc{}", std::env::consts::EXE_SUFFIX));
    posix_script::run(
        "livetest:baselines",
        "Research/trial/r26/render-baselines.sh",
        args,
        &[("DORC", OsString::from(dorc))],
    )
}
