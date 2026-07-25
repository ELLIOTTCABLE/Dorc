//! `dorc-sh` — the strip-and-exec shebang runner (`24P` §9 decision-dorc-sh-semantics; the runtime
//! object the `#!/usr/bin/env dorc-sh` corpus stamp names). Zero-arg form only:
//! `dorc-sh <script> [args…]`. It STRIPS the script if it is marked (identity on plain sh —
//! [`dorc_oracle::strip_file`] is marker-gated), then runs the stripped text as sh with `$0`/`$@`
//! fidelity: `sh -c "$stripped" "$script" "$@"` (POSIX `sh -c cmd name args…` assigns `$0` from
//! `name`, so no temp file is needed). It NEVER reads shebang CONTENT — a `#!` line is an ordinary
//! comment to sh (`24P` §9 decision-strip-leaves-shebang: nothing in dorc parses shebang bytes).
//!
//! The executor-bearing `dorc-sh [cmd…] -- script` form is a spec-note only (`24Q` §3 portability);
//! the spike ships the zero-arg form. `ARG_MAX` bounds the `-c` string for pathological script
//! sizes — disclosed (`ru-26`), fine for fixtures.
//!
//! SPIKE NOTE (`churn-avoidance-disclosure`): the spike uses spawn-and-wait forwarding the child's
//! exit code, not true process-replacement `exec` — portable (incl. msys), and the replacement is an
//! optimization, not a correctness requirement.

#![forbid(unsafe_code)]
// The I/O edge (workspace policy: I/O-edge crates may `#[expect]` these at the crate root, with
// reason) — dorc-sh reads a file, strips it, and runs it; its own diagnostics go to stderr.
#![expect(
    clippy::print_stderr,
    reason = "dorc-sh is an I/O edge: its own errors go to stderr; the stripped script owns stdout"
)]

use std::process::{Command, ExitCode};

/// `dorc-sh`'s three errors join the registry like every other surface
/// (`288` §6 rul-dorc-sh-not-carved-out) — slugs, canonical looms, auditable. The terse `dorc-sh: `
/// framing is a print-seat SURFACE SELECTION, not a carve-out. Body-only: an argv has no span.
///
/// The seam note stands and changes nothing now: if `dorc-sh` ever ships host-side, host-side
/// emissions likely stay raw-bytes-upstream with controller-side narration.
fn report(diag: &dorc_aid::Diag) {
    eprintln!(
        "dorc-sh: {}",
        dorc_aid::diag::render_body(diag, &dorc_core::Interner::default())
    );
}

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let Some(script) = args.next() else {
        report(&dorc_aid::Diag::new_spanless_site(
            dorc_aid::diag::DiagCode::DorcShUsage(dorc_aid::diag::DorcShUsage),
        ));
        return ExitCode::from(2);
    };
    let src = match std::fs::read_to_string(&script) {
        Ok(s) => s,
        Err(e) => {
            report(&dorc_aid::Diag::new_spanless_site(
                dorc_aid::diag::DiagCode::DorcShScriptUnreadable(
                    dorc_aid::diag::DorcShScriptUnreadable {
                        path: script.to_string_lossy().into_owned(),
                        detail: e.to_string(),
                    },
                ),
            ));
            return ExitCode::from(2);
        }
    };

    let mut interner = dorc_core::Interner::default();
    let stripped = dorc_oracle::strip_file(&mut interner, &src).value;

    // sh -c "$stripped" "$script" "$@": the script path becomes $0, the remaining argv is "$@".
    let status = Command::new("sh")
        .arg("-c")
        .arg(&stripped)
        .arg(&script) // $0
        .args(args) // "$@"
        .status();
    match status {
        // A POSIX exit status is 0..=255; `try_from` keeps it lint-clean (no truncating `as`).
        Ok(s) => ExitCode::from(u8::try_from(s.code().unwrap_or(1)).unwrap_or(1)),
        Err(e) => {
            report(&dorc_aid::Diag::new_spanless_site(
                dorc_aid::diag::DiagCode::DorcShExecFailed(dorc_aid::diag::DorcShExecFailed {
                    detail: e.to_string(),
                }),
            ));
            ExitCode::from(127)
        }
    }
}
