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
//! The strip-and-exec BODY lives below the seam in `dorc_cli::compose::shim_strip_and_run`
//! (`30X:inv-division-at-the-narrowest-edge`): this file is edge values plus one call.

#![forbid(unsafe_code)]
// The I/O edge (workspace policy: I/O-edge crates may `#[expect]` these at the crate root, with
// reason) — dorc-sh reads a file, strips it, and runs it; its own diagnostics go to stderr.
#![expect(
    clippy::print_stderr,
    reason = "dorc-sh is an I/O edge: its own errors go to stderr; the stripped script owns stdout"
)]

use std::process::ExitCode;

/// `dorc-sh`'s three errors join the registry like every other surface
/// (`288` §6 rul-dorc-sh-not-carved-out) — slugs, canonical looms, auditable. The terse `dorc-sh: `
/// framing is a print-seat SURFACE SELECTION, not a carve-out. Body-only: an argv has no span.
///
/// The seam note stands and changes nothing now: if `dorc-sh` ever ships host-side, host-side
/// emissions likely stay raw-bytes-upstream with controller-side narration.
fn report(diag: &dorc_aid::Diag) {
    eprint!(
        "{}",
        dorc_cli::shim_error_parts(
            &dorc_aid::RenderCtx::production(),
            diag,
            &dorc_core::Interner::default()
        )
        .text()
    );
}

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let Some(script) = args.next() else {
        report(&dorc_cli::shim_usage_error());
        return ExitCode::from(2);
    };
    let src = match std::fs::read_to_string(&script) {
        Ok(s) => s,
        Err(e) => {
            report(&dorc_cli::shim_script_read_error(
                &script.to_string_lossy(),
                &e,
            ));
            return ExitCode::from(2);
        }
    };
    // The resolved shell is an edge value (`one-shell-answer`): NEVER a bare PATH lookup, which on
    // native Windows resolves `%SystemRoot%\System32\bash.exe`, the WSL launcher.
    let Ok(shell) = dorc_transport::Posix::find() else {
        report(&dorc_cli::shim_no_shell_error());
        return ExitCode::from(127);
    };
    dorc_cli::compose::shim_strip_and_run(&shell, &script, &src, args)
}
