//! The one finding model + the coverage block (`27R` §5 dir-two-renders-one-model, §8b
//! dir-envelope-carries-coverage). Every lint source emits [`Finding`]s into ONE shape; the two
//! renderers (`crate::render`) are the only things that read it. Advisory-only
//! (`dir-no-license-plane-contact`): a finding is a report line, never a claim/license/fact.

/// A finding's tier. Its own three-value scale (NOT `core::Severity`): lint adds [`Info`](Self::Info)
/// for advisory disclosures a `core::Severity::Note` maps onto, and — crucially — an absent external
/// tool (`27R` §4 dir-absent-is-info). The `--fail-on` threshold (`27R` §5) keys on this; `Info`
/// NEVER gates an exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintSeverity {
    /// A correctness give-up or hard error (`core::Severity::Error`, a shellcheck `error`).
    Error,
    /// A portability/quality warning (`core::Severity::Warning`, a shellcheck `warning`).
    Warn,
    /// An advisory disclosure — a `core::Severity::Note`, or a "tool absent" run-note. Never gates.
    Info,
}

impl LintSeverity {
    /// The stable machine token (the JSONL `severity` field; `27R` §5 dir-stability-split — the
    /// field schema is versioned/stable). Lowercase, additive-only vocabulary.
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            LintSeverity::Error => "error",
            LintSeverity::Warn => "warn",
            LintSeverity::Info => "info",
        }
    }
}

/// How faithfully a finding's location was mapped back to the user's original source (`27R` §4
/// dir-tolerant-adapters). A dorc-native finding is always [`Exact`](Self::Exact) (it reads the
/// real source span); an external-tool finding degrades down the ladder as the adapter falls back
/// from machine-format parse → tolerant text remap → raw passthrough.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RemapFidelity {
    /// A machine-format parse whose line mapped cleanly through the strip line-map (or a native
    /// finding reading the real span).
    Exact,
    /// A tolerant text remap ("looks like a line number"), or a machine line that fell outside the
    /// strip line-map — the location may be off if upstream output drifted.
    Approximate,
    /// No location at all — the raw-passthrough tier (`27R` §4(c): the tool's output shipped as one
    /// opaque finding block).
    None,
}

impl RemapFidelity {
    /// The stable machine token (the JSONL `remap` field).
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            RemapFidelity::Exact => "exact",
            RemapFidelity::Approximate => "approximate",
            RemapFidelity::None => "none",
        }
    }
}

/// One lint finding — the whole reporting surface (`27R` §5). Paths and lines are ALWAYS the user's
/// ORIGINAL file and line (`27R` §4 dir-paths-stay-yours): temp paths and stripped-line numbers
/// never leak here, because the external adapters remap before minting a `Finding`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The user's ORIGINAL file path (never a temp/stdin surrogate).
    pub path: String,
    /// The 1-based ORIGINAL source line, if the finding has one (`None` for a whole-file / raw
    /// finding).
    pub line: Option<u32>,
    /// The 1-based column, if known.
    pub col: Option<u32>,
    /// The finding's severity tier.
    pub severity: LintSeverity,
    /// The emitting source's stable name (`27R` §8 delta-named-sources-selectable; the value the
    /// `--list-sources` / name-a-source surface uses).
    pub source: &'static str,
    /// The finding's code: a dorc-native slug (namespaced, append-only — `27R` §5 dir-stability-split)
    /// or an external tool's own stable code (`SC2086`, or a checkbashisms class tag). Never re-read
    /// to mean something else.
    pub code: String,
    /// The human message (unstable text — the CODE is the stable key, per rustc/shellcheck doctrine).
    pub message: String,
    /// Location-mapping fidelity (`27R` §4). Location-only; there is no fix/range remap in v0
    /// (`27R` §8 delta-eslint-processor-precedent).
    pub remap: RemapFidelity,
}

impl Finding {
    /// The deterministic sort key (`27R` §5 dir-deterministic-output): `(path, line, source, code)`.
    /// A `None` line sorts before any concrete line (whole-file findings lead their file). Returned
    /// by value so the caller can `sort_by_key` without borrowing self across the sort.
    #[must_use]
    pub fn sort_key(&self) -> (String, u32, &'static str, String) {
        (
            self.path.clone(),
            self.line.unwrap_or(0),
            self.source,
            self.code.clone(),
        )
    }
}

/// Whether a source actually ran for this invocation (`27R` §8b dir-envelope-carries-coverage). A
/// CI policy diffs the coverage block to catch silent scope-shrinkage (a tool vanishing from the
/// image ⇒ `Absent`; `--no-tools` ⇒ `Off`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceStatus {
    /// The source ran over the inputs.
    Ran,
    /// An external tool the source needs was not on PATH (`27R` §4 dir-absent-is-info).
    Absent,
    /// The source was disabled for this run (`--no-tools`).
    Off,
}

impl SourceStatus {
    /// The stable machine token (the JSONL coverage `status` field).
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            SourceStatus::Ran => "ran",
            SourceStatus::Absent => "absent",
            SourceStatus::Off => "off",
        }
    }
}

/// One source's coverage row (`27R` §8b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCoverage {
    /// The source's stable name.
    pub name: &'static str,
    /// Whether it ran.
    pub status: SourceStatus,
}

/// The coverage block carried in every machine-mode envelope (`27R` §8b dir-envelope-carries-coverage):
/// the lintable-file list + per-source status. State-free — an external CI policy diffs it; dorc owns
/// no manifest (`seam-lint-lock-manifest` is named-not-built).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coverage {
    /// The lintable files this run covered, in the order given (original paths).
    pub files: Vec<String>,
    /// Per-source run status.
    pub sources: Vec<SourceCoverage>,
}

/// The whole lint result: the sorted findings + the coverage block. The cli edge renders it and
/// computes the exit code (`27R` §5 exit trichotomy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LintReport {
    /// Findings, sorted by [`Finding::sort_key`] (`inv-determinism`).
    pub findings: Vec<Finding>,
    /// The coverage block.
    pub coverage: Coverage,
}

impl LintReport {
    /// The count of findings at or above `threshold` — the exit-code input (`27R` §5). A `None`
    /// threshold (`--fail-on=never`) yields 0. Because [`LintSeverity`] orders `Error < Warn < Info`,
    /// "at or above" means severity `<=` the threshold value.
    #[must_use]
    pub fn count_at_or_above(&self, threshold: Option<LintSeverity>) -> usize {
        match threshold {
            None => 0,
            Some(t) => self.findings.iter().filter(|f| f.severity <= t).count(),
        }
    }

    /// Totals for the human summary + JSONL counts (`27R` §8b): `(errors, warns, infos)`.
    #[must_use]
    pub fn severity_counts(&self) -> (usize, usize, usize) {
        let mut e: usize = 0;
        let mut w: usize = 0;
        let mut i: usize = 0;
        for f in &self.findings {
            match f.severity {
                LintSeverity::Error => e = e.saturating_add(1),
                LintSeverity::Warn => w = w.saturating_add(1),
                LintSeverity::Info => i = i.saturating_add(1),
            }
        }
        (e, w, i)
    }
}
