//! The ONE home for the runner's seam defaults (`30X:loom-seams-are-sh-lines`, `one-shell-answer`).
//!
//! Both drivers of a committed loom read the same seam selections from here: the process driver
//! (`cli/tests/e2e.rs`) exports them into a real shell, and the in-process driver
//! (`consumer.rs`) feeds them to `HarnessSeams::from_env` over a modelled environment. A second
//! copy of `RUN_SEED` or of a seam VALUE is exactly how the two drivers would derive a different
//! clock for one block and stop agreeing (`30X:loom-driver-is-derived-and-reported` →
//! `gate-two-drivers-agree`), so the values live once, here, and the variable NAMES live once in
//! `dorc_cli::seam` (referenced below, never re-spelled).

/// The run-wide seed the seeded entropy members (receipt ids, key material, nonce) derive from.
///
/// Constant for now (`30X:seed-varied-by-default` is lane C3's widening); a stable value keeps the
/// store names the seeded entropy mints reproducible. Relocated from `e2e.rs` so it is not spelled
/// twice.
pub const RUN_SEED: u64 = 0x0030_A15E_EDED;

/// The synthetic absolute root the in-process session keys its model store under.
///
/// Only ever a `ModelIo` key, never a disk path: it is absolute on both families
/// (`roots.rs::is_absolute`) so `RootInputs::of` accepts it on either platform, and the modelled
/// filesystem never touches a real directory.
pub const SESSION_ROOT: &str = "/dorc-loom-session";

/// The per-block value-seam `(variable name, value)` pairs both drivers inject.
///
/// `block_ordinal` is the 0-based replay-block index — `ReplayContext::block()` in-process, the
/// `drive_session` enumerate index in the shell — so the clock is `seeded:<ordinal>` and two
/// publishes in one session take distinct order tokens. The entropy stays on the umbrella
/// `RUN_SEED` (no per-seam override), so a receipt id is stable across a session's blocks.
#[must_use]
pub fn value_seam_pairs(block_ordinal: usize) -> [(&'static str, String); 4] {
    [
        (dorc_cli::seam::SEED_ENV, RUN_SEED.to_string()),
        (dorc_cli::seam::CLOCK_ENV, format!("seeded:{block_ordinal}")),
        (dorc_cli::seam::POSTURE_ENV, "pinned:interactive".to_owned()),
        (dorc_cli::seam::SOURCE_MATCH_ENV, "pinned:off".to_owned()),
    ]
}

/// The receipt-roots `(variable name, value)` pair, keyed under `root` (the consumer supplies it:
/// the runner's throwaway directory in the shell, [`SESSION_ROOT`] in-process).
#[must_use]
pub fn roots_seam_pair(root: &str) -> (&'static str, String) {
    (dorc_cli::seam::ROOTS_ENV, format!("pinned:{root}"))
}

/// A modelled environment backed by a map, so the in-process driver feeds the same seam selections
/// the shell exports to the ONE parser (`HarnessSeams::from_env`; `30X:loom-seams-are-sh-lines`).
struct MapEnv(std::collections::BTreeMap<&'static str, String>);

impl dorc_cli::seam::SeamEnv for MapEnv {
    fn var(&self, name: &str) -> Option<String> {
        self.0.get(name).cloned()
    }
}

/// This block's `Seams` for the in-process session — the SAME selections the shell exports, keyed
/// under [`SESSION_ROOT`], through the one parser both drivers use.
///
/// `expect` is the runner asserting its own selections parse: the map is built from these
/// functions, never from host input, so a malformed value would be a bug in this seat, not an
/// untrusted-input path (`inv-no-throw` binds the latter).
#[must_use]
pub fn session_seams(block_ordinal: usize) -> dorc_cli::seam::Seams {
    let mut map: std::collections::BTreeMap<&'static str, String> =
        value_seam_pairs(block_ordinal).into_iter().collect();
    let (root_name, root_value) = roots_seam_pair(SESSION_ROOT);
    map.insert(root_name, root_value);
    dorc_cli::seam::HarnessSeams::from_env(&MapEnv(map))
        .expect("the runner seat's own seam selections always parse")
        .into()
}
