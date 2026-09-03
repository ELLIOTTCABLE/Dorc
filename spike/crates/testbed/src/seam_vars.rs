//! The seam-selection environment-variable names, spelled once (`30X:loom-seams-are-sh-lines`).
//!
//! These are the harness's typed environment surface: `cli::seam` PARSES them (the parser and its
//! value grammar live there, importing these names), the runner-defaults seat SETS them, and the
//! run-seed seat reads `DORC_SEED`. The names live here, below `cli`, so a runner and the loom driver
//! reach them without a mirror. A rename touches only this file plus the parser
//! (`rul-strawman-formats-no-compat`).

/// The umbrella seed every `Seeded` member derives from; also the developer's replay affordance
/// (`DORC_SEED=<n> mise run …`) and the per-case regression pin (`$ export DORC_SEED=<n>`).
pub const SEED_ENV: &str = "DORC_SEED";
/// The clock seam variable (`seeded:<fold> | pinned:<ms> | os | absent`).
pub const CLOCK_ENV: &str = "DORC_SEAM_CLOCK";
/// The receipt-id entropy seam variable (`seeded[:<u64>] | os`).
pub const RECEIPT_IDS_ENV: &str = "DORC_SEAM_RECEIPT_IDS";
/// The key-entropy seam variable (`seeded[:<u64>] | os`).
pub const KEY_ENTROPY_ENV: &str = "DORC_SEAM_KEY_ENTROPY";
/// The attempt-nonce seam variable (`seeded[:<u64>] | os`).
pub const NONCE_ENV: &str = "DORC_SEAM_NONCE";
/// The stdout-posture seam variable (`pinned:interactive | pinned:kept | os`).
pub const POSTURE_ENV: &str = "DORC_SEAM_STDOUT_POSTURE";
/// The source-match seam variable (`pinned:off | pinned:<commit> | os`).
pub const SOURCE_MATCH_ENV: &str = "DORC_SEAM_SOURCE_MATCH";
/// The transport seam variable (`local:<shell>[;<interpreter>]`).
pub const TRANSPORT_ENV: &str = "DORC_SEAM_TRANSPORT";
/// The receipt-roots seam variable (`pinned:<absolute dir>`, required).
pub const ROOTS_ENV: &str = "DORC_SEAM_ROOTS";

/// Every seam variable the parser reads, so the harness binary can refuse when NONE is set.
pub const SEAM_ENV_VARS: &[&str] = &[
    SEED_ENV,
    CLOCK_ENV,
    RECEIPT_IDS_ENV,
    KEY_ENTROPY_ENV,
    NONCE_ENV,
    POSTURE_ENV,
    SOURCE_MATCH_ENV,
    TRANSPORT_ENV,
    ROOTS_ENV,
];
