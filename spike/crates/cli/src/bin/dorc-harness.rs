//! The `dorc-harness` binary: the SAME [`dorc_cli::compose::run`] engine the shipped `dorc` drives,
//! but with a bundle parsed from the environment (`30X:bin-harness-sibling-not-produced-cli`).
//!
//! It is test-only tooling, never shipped. Its bundle is a [`HarnessSeams`], which structurally
//! cannot name the production transport (`RealSsh`) or the production roots (`Os`) — the type is the
//! fence (`30X:inv-fixture-state-never-typeable-into-main`). It REFUSES loudly, nonzero, when NO
//! seam variable is set, so it can never be mistaken for the product; a malformed selection is a
//! typed refusal (`30X:loom-seams-are-sh-lines`), never a silent default.
#![forbid(unsafe_code)]
// The composition root prints to stdout/stderr; this bin's own two refusal lines go to stderr.
#![expect(
    clippy::print_stderr,
    reason = "dorc-harness is test-only I/O tooling: its own refusals go to stderr"
)]

use std::process::ExitCode;

use dorc_cli::seam::{HarnessSeams, ProcessSeamEnv};

fn main() -> ExitCode {
    let environment = ProcessSeamEnv;
    if !HarnessSeams::any_seam_set(&environment) {
        eprintln!(
            "dorc-harness: refusing — no seam configured. This is test-only tooling, not the product; \
             set at least one DORC_SEAM_* variable (or DORC_SEED) to drive it."
        );
        return ExitCode::from(2);
    }
    match HarnessSeams::from_env(&environment) {
        Ok(harness) => dorc_cli::compose::run(&harness.into()),
        Err(diag) => {
            let parts = dorc_aid::diag::render_staged_cli_parts(
                "dorc-harness",
                &dorc_aid::RenderCtx::production(),
                &diag,
                "",
                "",
                &dorc_core::Interner::default(),
            );
            eprint!("{}", parts.text());
            ExitCode::from(2)
        }
    }
}
