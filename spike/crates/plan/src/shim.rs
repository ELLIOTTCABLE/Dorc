//! `shim` — the per-run PATH shim for `dorc-sh` (`274` §5; `27J` §2.3). The pure, DST-clean model:
//! host-independent shipped text, run-id-derived naming, and the failure lattice.
//!
//! An INVITED reentry (`dorc:sh -c '…'`) probe-ships rewritten to `dorc-sh` (`274` §1). `dorc-sh` is
//! not a real binary — it is a per-run shim the session MATERIALIZES once, on a PATH-prepend, that
//! execs the session-resolved evaluator. The design win (`274` §5): the pinning lives in the SHIM,
//! not in per-site text, so the shipped probe text is HOST-INDEPENDENT — a DST/golden dividend
//! (goldens stay host-agnostic; the only host variance is the shim's one evaluator line).
//!
//! This module is the pure half — everything that is a deterministic function of its inputs
//! (`inv-determinism`; no clock/RNG/fs/net):
//!
//! * [`shim_script`] — the shim file TEXT (host-independent structure; the one evaluator line is the
//!   sole variance, injected — the DST seam);
//! * [`shim_dir_name`] — the run-id-derived temp dir name (NO `mktemp` randomness; stale dirs inert);
//! * [`classify_shim_rc`] / [`smoke_degrades_session`] — the failure lattice (`274` §5): every shim /
//!   exec failure drains to the flat ≥2 sink (`rul-rc-partition`) ⇒ can't-say ⇒ run; a failed
//!   session-preamble smoke-test converts scattered failures into ONE session-level shimless-degrade.
//!
//! # The I/O edge (NOT here — the cli/hostsim follow-on)
//!
//! Materialization is real I/O (atomic write-then-rename at session-establishment, happens-before all
//! probes by protocol; PATH-prepend; cleanup) and lives at the cli edge, with the in-memory simulator
//! registering the shim as a command keyed to the materialization event (`274` §5 DST story). The
//! actual probe-SHIPPING that emits `dorc-sh` reentries is task-14-gated (`274` §5/§13); this settles
//! the shim's shape and rc-semantics ahead of it. MODELS only — nothing here is wired into the live
//! probe path, so the corpus stays byte-stable (`empty-world-byte-identical`).

/// The host-independent `dorc-sh` shim script text (`274` §5). Structure is fixed; the ONLY host
/// variance is `evaluator` — the session-resolved sh evaluator (`/bin/dash`, `sh`, a pinned posh),
/// injected so the rest of the shipped text (and its goldens) stay host-agnostic. The shim `exec`s the
/// evaluator with the reentry's argv verbatim (`"$@"`), so `dorc-sh -c 'STR' _ a b` runs
/// `<evaluator> -c 'STR' _ a b` — the pinned evaluator, no analysis, transitively composable through
/// PATH (`274` §1 row-3 by construction).
///
/// `emit-never`-clean and floor-dialect (`276`): a bare `#!/bin/sh` + `exec`; no pipefail, no bashism.
#[must_use]
pub fn shim_script(evaluator: &str) -> String {
    // `exec "$@"` through the pinned evaluator: the reentry argv (`-c 'STR' name args…`) is passed
    // verbatim, so positional binding (`24T:L3`) and the child's fresh shell-options (`24T:L1`) are
    // exactly the evaluator's own POSIX semantics — the shim adds no policy, only the pin.
    format!("#!/bin/sh\nexec {evaluator} \"$@\"\n")
}

/// The run-id-derived temp directory NAME the session materializes the shim into (`274` §5). A
/// deterministic function of the run-id — NO `mktemp` randomness (`inv-determinism`; a stale dir from
/// a crashed run is inert, never colliding because the run-id differs). The caller roots this under
/// the host's temp root at the I/O edge; this is the leaf name only (host-path-agnostic ⇒ goldens
/// stay agnostic).
#[must_use]
pub fn shim_dir_name(run_id: &str) -> String {
    format!("dorc-shim-{run_id}")
}

/// The outcome of running a `dorc-sh` reentry probe, classified by its rc through the failure lattice
/// (`274` §5; `rul-rc-partition`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShimOutcome {
    /// rc 0/1 — the child evaluator RAN and produced the tool's real verdict (0 = the named sense
    /// holds, 1 = its complement). Not a shim failure.
    Ran {
        /// 0 = holds, 1 = absent (the tool's own verdict rc).
        tool_rc: u8,
    },
    /// Any other rc — the flat ≥2 sink (`rul-rc-partition`): a shim/exec failure (127 not-found /
    /// shim not on PATH, 126 not-executable, 125 env failure) OR a tool rc ≥ 2. Drains to can't-say ⇒
    /// RUN (`274` §5). The rc-partition is accidentally robust here — the exec-failure codes
    /// (125/126/127) all land in this sink already (`274` §12 r4), so an unavailable shim degrades
    /// SAFE with nothing extra to do.
    CantSay {
        /// The raw rc that landed in the sink (opaque to the verdict; kept for the why-lane note).
        rc: i32,
    },
}

/// Classify a `dorc-sh` reentry probe's rc through the failure lattice (`274` §5). 0/1 ⇒
/// [`ShimOutcome::Ran`]; everything else (including the shim-unavailability codes 125/126/127) ⇒
/// [`ShimOutcome::CantSay`] ⇒ run. Pure/total.
#[must_use]
pub fn classify_shim_rc(rc: i32) -> ShimOutcome {
    match rc {
        0 => ShimOutcome::Ran { tool_rc: 0 },
        1 => ShimOutcome::Ran { tool_rc: 1 },
        other => ShimOutcome::CantSay { rc: other },
    }
}

/// Whether a failed session-preamble smoke-test degrades the WHOLE session shimless (`274` §5). The
/// preamble runs the shim once (`dorc-sh -c 'exit 0'`-ish) at session-establishment; a non-zero smoke
/// rc means the shim could not be materialized/executed (a noexec `/tmp`, a PATH-scrub body, a
/// Windows/noexec target), so EVERY marked-reentry probe in the session pre-degrades WITHOUT shipping
/// — one session-level shimless-degrade instead of scattered per-site 127s (`274` §5; the honest
/// value-zero-for-reentry-there outcome). Non-reentry probes are unaffected.
#[must_use]
pub fn smoke_degrades_session(smoke_rc: i32) -> bool {
    smoke_rc != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shim_script_is_host_independent_but_for_the_evaluator_line() {
        // Two evaluators differ only in the one exec line; the rest (shebang, exec form, "$@") is
        // byte-identical — the host-independence property that keeps goldens host-agnostic (`274` §5).
        let dash = shim_script("/bin/dash");
        let posh = shim_script("posh");
        assert_eq!(dash, "#!/bin/sh\nexec /bin/dash \"$@\"\n");
        assert_eq!(posh, "#!/bin/sh\nexec posh \"$@\"\n");
        // Everything but the evaluator token is shared.
        assert_eq!(
            dash.replace("/bin/dash", "X"),
            posh.replace("posh", "X"),
            "only the evaluator line varies"
        );
    }

    #[test]
    fn shim_dir_name_is_run_id_derived_no_randomness() {
        // Deterministic in the run-id — same run-id ⇒ same name (no mktemp randomness); different
        // run-ids never collide (stale dirs inert).
        assert_eq!(shim_dir_name("r7"), "dorc-shim-r7");
        assert_eq!(shim_dir_name("r7"), shim_dir_name("r7"));
        assert_ne!(shim_dir_name("r7"), shim_dir_name("r8"));
    }

    #[test]
    fn rc_partition_zero_one_run_else_cant_say() {
        assert_eq!(classify_shim_rc(0), ShimOutcome::Ran { tool_rc: 0 });
        assert_eq!(classify_shim_rc(1), ShimOutcome::Ran { tool_rc: 1 });
        assert_eq!(classify_shim_rc(2), ShimOutcome::CantSay { rc: 2 });
    }

    #[test]
    fn shim_unavailability_codes_drain_to_the_sink() {
        // 125 (env failure) / 126 (not executable) / 127 (not found — shim not on PATH) all land in
        // the ≥2 sink ⇒ can't-say ⇒ run (`274` §12 r4 — the rc-partition is accidentally robust).
        for rc in [125, 126, 127] {
            assert_eq!(
                classify_shim_rc(rc),
                ShimOutcome::CantSay { rc },
                "shim-unavailability rc {rc} drains to can't-say ⇒ run"
            );
        }
    }

    #[test]
    fn failed_smoke_degrades_the_whole_session() {
        // A clean smoke (rc 0) keeps the session shimmed; any non-zero smoke degrades it shimless —
        // one session-level degrade, not scattered per-site 127s (`274` §5).
        assert!(!smoke_degrades_session(0));
        assert!(smoke_degrades_session(127));
        assert!(smoke_degrades_session(1));
    }
}
