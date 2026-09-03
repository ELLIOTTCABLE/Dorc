//! The ONE home for the runner's seam defaults (`30X:loom-seams-are-sh-lines`, `one-shell-answer`).
//!
//! Both drivers of a committed loom read the same seam selections from here: the process driver
//! (`cli/tests/e2e.rs`) exports them into a real shell, and the in-process driver
//! (`consumer.rs`) feeds them to `HarnessSeams::from_env` over a modelled environment. A second
//! copy of a seam VALUE is exactly how the two drivers would derive a different clock for one block
//! and stop agreeing (`30X:loom-driver-is-derived-and-reported` → `gate-two-drivers-agree`), so the
//! values live once, here, and the variable NAMES live once in `dorc_testbed::seam_vars` (imported
//! below, never re-spelled).
//!
//! The run's seed is `dorc_testbed::run_seed()` — `DORC_SEED` from the process env, or drawn
//! once from OS entropy (`30X:seed-varied-by-default`). The clock is derived from that seed and the
//! block ordinal by ONE formula, so an author's `$ export DORC_SEED=7` pins every later block's clock
//! and ids together with one line, and the two drivers date a block identically.

use dorc_testbed::run_seed::run_seed;
use dorc_testbed::seam_vars::{CLOCK_ENV, POSTURE_ENV, ROOTS_ENV, SEED_ENV, SOURCE_MATCH_ENV};

/// The synthetic absolute root the in-process session keys its model store under.
///
/// Only ever a `ModelIo` key, never a disk path: it is absolute on both families
/// (`roots.rs::is_absolute`) so `RootInputs::of` accepts it on either platform, and the modelled
/// filesystem never touches a real directory.
pub const SESSION_ROOT: &str = "/dorc-loom-session";

/// How many days the run seed spreads the clock across before wrapping, so a 64-bit seed times one
/// day cannot overflow the base instant (`30X` §6: fold the seed, dates stay plausible). ~100 years,
/// so distinct seeds are almost always distinct mornings — a case whose render churns run-to-run has
/// hidden nondeterminism — while every date still reads as a plausible 2020s-2120s morning.
const CLOCK_SEED_DAY_SPAN: u64 = 36_525;

/// A block's clock seed: the run seed folded to a plausible day offset (so a huge seed cannot
/// overflow the base instant) PLUS the block ordinal (so two publishes in one session take distinct
/// order tokens). The shell driver computes the SAME value from `$DORC_SEED` in its shadow line, so
/// both drivers date a block identically (`30X` §6; `rul-runner-varies-only-what-it-set`).
#[must_use]
pub fn fold_clock_seed(seed: u64, ordinal: usize) -> u64 {
    (seed % CLOCK_SEED_DAY_SPAN).saturating_add(ordinal as u64)
}

/// The `DORC_SEAM_CLOCK` value for a block, from a concrete seed — the in-process twin of the shell's
/// [`clock_seam_shell_value`].
#[must_use]
pub fn clock_seam_value(seed: u64, ordinal: usize) -> String {
    format!("seeded:{}", fold_clock_seed(seed, ordinal))
}

/// The sh the shell driver injects to compute a block's clock from `$DORC_SEED` at block time — the
/// EXACT twin of [`clock_seam_value`]. `$DORC_SEED` is read live, so an author's `export DORC_SEED`
/// governs it; the ordinal and the day span are runner-known literals. A `$(( … ))` here is
/// runner-authored sh, not book bytes (`30X` §6), and matches `fold_clock_seed` because both a `u64`
/// and sh's `intmax_t` fold a seed under 2^62 to the same value.
#[must_use]
pub fn clock_seam_shell_value(ordinal: usize) -> String {
    format!("seeded:$(( (${SEED_ENV} % {CLOCK_SEED_DAY_SPAN}) + {ordinal} ))")
}

/// The per-block value-seam `(variable name, value)` pairs both drivers inject at session start.
///
/// `DORC_SEED` is this run's seed (`30X:seed-varied-by-default`). `DORC_SEAM_CLOCK` is derived from
/// that seed and the block ordinal by [`clock_seam_value`]; the per-block RE-injection recomputes it
/// from the CURRENT seed (an author's `export DORC_SEED` then governs it), so the clock a block sees
/// is always a function of the seed live at that block. The entropy stays on the umbrella `DORC_SEED`
/// (no per-seam override), so a receipt id is stable across a session's blocks under one seed.
#[must_use]
pub fn value_seam_pairs(block_ordinal: usize) -> [(&'static str, String); 4] {
    let seed = run_seed();
    [
        (SEED_ENV, seed.to_string()),
        (CLOCK_ENV, clock_seam_value(seed, block_ordinal)),
        (POSTURE_ENV, "pinned:interactive".to_owned()),
        (SOURCE_MATCH_ENV, "pinned:off".to_owned()),
    ]
}

/// The receipt-roots `(variable name, value)` pair, keyed under `root` (the consumer supplies it:
/// the runner's throwaway directory in the shell, [`SESSION_ROOT`] in-process).
///
/// The in-process driver seeds its own modelled environment ([`crate::session_env::SessionEnv`]) from
/// these pairs and reads the exported subset through the one parser, so there is no second `Seams`
/// constructor here — the session env IS the seam source (`30X:loom-seams-are-sh-lines`).
#[must_use]
pub fn roots_seam_pair(root: &str) -> (&'static str, String) {
    (ROOTS_ENV, format!("pinned:{root}"))
}
