//! The ONE seat answering "what is this run's seed" (`30X:seed-varied-by-default`,
//! `seed-declared-is-regression`, `seed-two-affordances`).
//!
//! Every runner and sweep draws the run seed from here, exactly once per process, and names it the
//! same way in its banner and its failures. A render that does not depend on entropy, clock, or
//! ordering reproduces under any seed, so varying it per run is a free invariance test; a render
//! that legitimately shows a seed-dependent byte (a receipt id, a date) pins its seed as regression
//! with ONE spelling — `$ export DORC_SEED=<n>` — and the failure note names that spelling.
//!
//! Drawing the seed is a nondeterministic edge (env, then OS entropy) and lives here, in the suite's
//! shared substrate, never in a kernel crate (`inv-determinism`): the kernel only ever sees the seed
//! as a value crossing a seam. The value is bounded below 2^62 so both a Rust `u64` and a shell
//! `$(( … ))` (which is `intmax_t`/i64) fold it to the same clock — the two drivers must agree.

use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::seam_vars::SEED_ENV;

/// The largest seed value drawn, exclusive: kept under 2^62 so the clock fold is identical whether a
/// `u64` or a shell `$(( … ))` computes it. An env-supplied seed is the developer's own (they take
/// it from the banner, which is bounded); only the DRAWN value is masked.
const DRAWN_SEED_CEILING: u64 = 1 << 62;

/// This run's seed: `DORC_SEED` from the process environment if set (the developer's replay
/// affordance — `DORC_SEED=<n> mise run test:looms -- <case>`), else drawn ONCE from OS entropy and
/// memoized for the life of the process.
///
/// The runner is an edge, so the OS read happens here and nowhere the kernel can reach. The drawn
/// value is a splitmix of the wall clock and the pid — enough variation to make an unpinned run a
/// fresh invariance test each time, never cryptographic.
#[must_use]
pub fn run_seed() -> u64 {
    static SEED: OnceLock<u64> = OnceLock::new();
    *SEED.get_or_init(|| {
        std::env::var(SEED_ENV)
            .ok()
            .and_then(|raw| raw.trim().parse::<u64>().ok())
            .unwrap_or_else(drawn_seed)
    })
}

/// Draw a fresh seed from the wall clock and the pid, mixed so nearby draws diverge, and bounded
/// below [`DRAWN_SEED_CEILING`] for shell-arithmetic parity. Seconds and subsecond-nanos are read
/// separately so no `u128`→`u64` truncation is needed; both fit a `u64` exactly.
fn drawn_seed() -> u64 {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let mixed = elapsed
        .as_secs()
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add(u64::from(elapsed.subsec_nanos()).wrapping_mul(0x2545_F491_4F6C_DD1D))
        .wrapping_add(u64::from(std::process::id()).wrapping_mul(0xD1B5_4A32_D192_ED03));
    let mut z = mixed;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    z % DRAWN_SEED_CEILING
}

/// A constant xor'd into a seed to get a DIFFERENT second seed for the bless reproduction check
/// (`30X:seed-two-affordances`). Nonzero, so the second seed always differs; low-bit-only, so the
/// result stays under 2^62 for shell-arithmetic parity. The ONE derivation both blessing
/// authorities share — the e2e runner and `dorc-loom publish` (`30Xa:Checkpoint D1`, rider d).
pub const SECOND_SEED_XOR: u64 = 0x5EED_5EED_5EED;

/// The second seed a bless re-renders the candidate under, to prove its transcript reproduced and is
/// not baked-in nondeterminism (`30X:seed-two-affordances`).
#[must_use]
pub fn second_seed(seed: u64) -> u64 {
    seed ^ SECOND_SEED_XOR
}

/// The start-of-run banner every runner prints once (`30X:seed-two-affordances`): the seed, and the
/// replay spelling that reproduces this exact run.
#[must_use]
pub fn seed_banner(seed: u64) -> String {
    format!("seed: {seed} — replay this run with `{SEED_ENV}={seed}`")
}

/// The failure note every runner and sweep appends when it reddens under a seed
/// (`30X:seed-two-affordances`, `seed-declared-is-regression`): both affordances the human asked
/// for — the replay spelling (this run again) and the pin spelling (this case as regression forever).
#[must_use]
pub fn seed_failure_note(seed: u64, task: &str) -> String {
    format!(
        "run seed {SEED_ENV}={seed} — replay: `{SEED_ENV}={seed} mise run {task} -- <case>`; \
         pin a case as regression by adding `$ export {SEED_ENV}={seed}` as its first session line"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_drawn_seed_stays_inside_the_shell_arithmetic_range() {
        for _ in 0..1000 {
            assert!(
                drawn_seed() < DRAWN_SEED_CEILING,
                "a drawn seed must fold identically in sh"
            );
        }
    }

    #[test]
    fn the_run_seed_is_stable_within_a_process() {
        assert_eq!(
            run_seed(),
            run_seed(),
            "the run seed is drawn once and memoized"
        );
    }
}
