//! The exact `dorc-receipt/1` skeleton vocabulary: every record kind, its ordered field
//! keys, and the closed token set or numeric width of each field.
//!
//! This module is a TABLE. It generates no code and drives no behaviour on its own; the
//! writer and the reader both read it, which is what makes them agree by construction
//! rather than by review. [`RecordKind::fields`] has one arm per kind and no wildcard, so a
//! new kind cannot land unclassified, and a new key is appended immediately before
//! `account`, which is last on every kind.

/// The five states an opaque-capable field may report in the skeleton.
pub const OPAQUE_STATE: &[&str] = &[
    "captured",
    "withheld-plain",
    "unavailable",
    "uncollected",
    "omitted-limit",
];

/// The four states an apply-image slot may report. Narrower than [`OPAQUE_STATE`]: an image
/// is never partially collected, so `uncollected` has no meaning here.
pub const IMAGE_STATE: &[&str] = &["captured", "withheld-plain", "unavailable", "omitted-limit"];

/// Where a recorded object stood relative to host contact.
pub const ACCOUNT: &[&str] = &["authored-before-contact", "host-influenced", "untracked"];

/// The boolean spelling. Never `true`/`false`, never `0`/`1`.
pub const BOOL: &[&str] = &["yes", "no"];

/// The per-site and per-region plan outcome.
pub const DISPOSITION: &[&str] = &["run", "replace", "omit", "guard"];

/// Whether an assignment knows which presented plans it came from.
pub const ORIGIN_STATE: &[&str] = &["unavailable", "known"];

/// The invocation shapes that can produce a receipt. Projected from the CLI's own command
/// dispatch; the shapes that write no receipt have no token here.
pub const MODE: &[&str] = &["plan", "apply", "round-trip"];

/// What a recorded source was to the run.
pub const SOURCE_ROLE: &[&str] = &[
    "book",
    "named-load",
    "book-sourced",
    "load-dependency",
    "plain-inclusion",
];

/// Which dialect a source was accepted as, which is what decides whether its bytes are kept.
///
/// MECHANICAL, and deliberately not the user-facing book/oracle gloss
/// (`30Ra:planning-book-bytes-and-durable-locators`): the question is whether the file was accepted
/// as valid `dorc-lang`, not what anybody called it. A book carrying a `# dorc-lang/v0.2` marker
/// and passing is `dorc-lang`; an oracle-shaped file that did not pass is general sh. The gloss
/// beneath it — general source may mutate, `dorc-lang` is mutation-pure by contract — is what makes
/// that the right boundary for byte custody.
pub const SOURCE_CLASS: &[&str] = &["dorc-lang", "general-sh"];

/// The closed intake answer. `refused` is representable here and unreachable at the current
/// writer, because a refusal returns before the recording seat.
pub const ADMISSION_OUTCOME: &[&str] = &["admitted", "no-observation", "refused"];

/// What the definition plane decided for one name.
pub const LOAD_OUTCOME: &[&str] = &["bound", "contested", "unprovable", "helper-conflict"];

/// One site's analysis classification. The two source variants carrying a decision-bearing
/// boolean are split into separate tokens rather than merged, because that boolean is what
/// decides whether the row licenses anything.
pub const SITE_CLASS: &[&str] = &[
    "must-run",
    "establish-probe-ambient",
    "establish-probe-written",
    "query-resolvable-valid",
    "query-resolvable-stale",
    "establish-members-self-reached",
    "establish-members-reached",
    "inline-call",
];

/// Which dataflow answer a certification row is about.
pub const SOLVE_PASS: &[&str] = &[
    "whole-window",
    "value-flow",
    "function-environment",
    "reaching-defs",
    "self-reach",
    "effective-reach",
];

/// Which body a probe site shipped.
pub const SHIP_LANE: &[&str] = &["verdict", "predict", "unresolvable"];

/// What the survival walk decided. The four demotion causes stay distinct: two are claims
/// about the book's mutators, one is a finding about resolver quality, and one is a finding
/// about our own solver, and spelling any of them as another names the wrong cause.
pub const SURVIVAL_OUTCOME: &[&str] = &[
    "clean",
    "survived-standalone",
    "survived-aggregate",
    "demoted-total-wall",
    "demoted-poisoned",
    "demoted-may-alias",
    "demoted-solve-inconsistent",
    "rederivation-disagreed",
];

/// Which render-time decision a row records. The key axis rides the token, because a region
/// owns no execution and a row keyed by a contributing invocation would name the wrong thing.
pub const RENDER_KIND: &[&str] = &[
    "pinned-binding",
    "refused-heredoc-site",
    "refused-blocking-redirect-site",
    "refused-heredoc-region",
    "refused-blocking-redirect-region",
    "omit-neutralised",
    "omit-not-neutralised",
    "defensive-emission-on",
    "defensive-emission-off",
    "certifier-trip-demote",
    "import-repointed",
    "import-inlined",
];

/// The speech act of a narrative row.
pub const SPEECH_ACT: &[&str] = &[
    "measured",
    "vouched",
    "ran",
    "claimed",
    "derived",
    "consented",
    "declined",
];

/// The collapse class of a narrative row. The reserved cancellation class is deliberately
/// absent: its source variant is unconstructable, and a token that cannot be written is a
/// promise.
pub const NARRATIVE_KIND: &[&str] = &[
    "fact-merge-disagreement",
    "verdict-decline",
    "wall-formation",
    "substitution-refusal",
    "entry-denial",
    "wrapper-pair-incoherent",
    "entry-failure",
    "demotion",
    "render-refusal",
    "fixpoint-cap-degrade",
    "role-family-shadowed",
    "solver-consistency-failure",
    "composition-suspended",
    "projection-drop",
];

/// Which irreversible verb a licensor row attributes.
pub const LICENSE_VERB: &[&str] = &["replace", "guard"];

/// Whose utterance a replacement license rests on.
pub const LICENSE_CUSTODY: &[&str] = &["vouched", "vouched-severally", "measured-self"];

/// Every in-memory decision species a projection can decline to carry.
pub const SPINE_SPECIES: &[&str] = &[
    "invocation",
    "record-stream",
    "disposition",
    "presented-plan",
    "load-decision",
    "site-classification",
    "solve-certification",
    "vouch",
    "probe-ship",
    "admission",
    "observation",
    "validity-round",
    "survival",
    "render-decision",
    "region-decision",
    "outcome",
];

/// Why a projection did not carry a population.
pub const OMISSION_REASON: &[&str] = &[
    "unminted",
    "not-projected-v1",
    "content-excluded",
    "over-limit",
];

/// Which publication route authorized an apply.
pub const APPLY_POLICY: &[&str] = &["required-rich", "configured-bypass"];

/// The graceful terminal state an apply reached. A session that produced no completion
/// marker is `unknown`, never `not-attempted`: absence of output cannot prove absence of
/// execution, and only a spawn that never happened may claim nothing ran.
pub const TERMINAL_STATE: &[&str] = &[
    "complete",
    "command-failed",
    "unknown",
    "not-attempted",
    "transport-failed",
    "mutation-integrity-aborted",
    "cancelled",
];

/// Whether the terminal report itself reached durable storage. Narration only.
pub const DURABLE_STATE: &[&str] = &["published", "failed", "not-attempted"];

/// What one site did during an apply.
pub const SITE_STATUS: &[&str] = &[
    "ran",
    "guard-passed",
    "guard-fell-through",
    "replaced",
    "omitted",
    "not-reached",
    "unknown",
];

/// The token a semantically-absent scalar carries. Never an empty value, never an omitted
/// field.
pub const ABSENT: &str = "absent";

/// The scalar shape of one field. Every closed vocabulary in the format is a
/// [`FieldType::Closed`] over one of the constants above, so there is one acceptance path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    /// A canonical decimal integer that fits `u32`.
    Count,
    /// A canonical decimal integer that fits `u64`.
    Wide,
    /// [`FieldType::Count`], or [`ABSENT`].
    OptionalCount,
    /// [`FieldType::Wide`], or [`ABSENT`].
    OptionalWide,
    /// Exactly 64 lowercase hexadecimal characters.
    Digest,
    /// [`FieldType::Digest`], or [`ABSENT`].
    OptionalDigest,
    /// Exactly one token from a closed set.
    Closed(&'static [&'static str]),
}

impl FieldType {
    /// Whether `atom` is a legal spelling for this field. The one acceptance predicate both
    /// the writer's self-check and the reader consult.
    #[must_use]
    pub fn admits(self, atom: &str) -> bool {
        match self {
            Self::Count => canonical_u32(atom).is_some(),
            Self::Wide => canonical_u64(atom).is_some(),
            Self::OptionalCount => atom == ABSENT || canonical_u32(atom).is_some(),
            Self::OptionalWide => atom == ABSENT || canonical_u64(atom).is_some(),
            Self::Digest => is_digest(atom),
            Self::OptionalDigest => atom == ABSENT || is_digest(atom),
            Self::Closed(tokens) => tokens.contains(&atom),
        }
    }
}

/// Exactly 64 lowercase hexadecimal characters.
#[must_use]
pub fn is_digest(atom: &str) -> bool {
    atom.len() == 64
        && atom
            .bytes()
            .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
}

/// A canonical decimal `u64`: no sign, no leading zero except the literal `0`, and at most
/// [`crate::limits::MAX_INTEGER_DIGITS`] digits.
#[must_use]
pub fn canonical_u64(atom: &str) -> Option<u64> {
    if atom.is_empty() || atom.len() > crate::limits::MAX_INTEGER_DIGITS {
        return None;
    }
    if !atom.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if atom.len() > 1 && atom.starts_with('0') {
        return None;
    }
    atom.parse().ok()
}

/// A canonical decimal `u32`, on [`canonical_u64`]'s terms.
#[must_use]
pub fn canonical_u32(atom: &str) -> Option<u32> {
    u32::try_from(canonical_u64(atom)?).ok()
}

/// Every skeleton record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RecordKind {
    /// The producing invocation and its controller-minted identity.
    Invocation,
    /// One file the run acquired.
    Source,
    /// The closed intake outcome.
    Admission,
    /// The identities of one complete approval surface.
    PresentedPlan,
    /// One site's plan outcome.
    SiteDecision,
    /// One authored region's shared outcome.
    RegionDecision,
    /// One definition-plane outcome.
    LoadDecision,
    /// One site's analysis classification.
    SiteClassification,
    /// One dataflow certification answer.
    SolveCertification,
    /// Which body one probe site shipped.
    ProbeShip,
    /// One survival-tier outcome.
    Survival,
    /// One render-time decision.
    RenderDecision,
    /// One decision-inert narrative.
    Narrative,
    /// What licensed one irreversible verb.
    Licensor,
    /// A population the projection did not carry.
    ProjectionOmission,
    /// The apply's pre-dispatch commitment.
    ApplyIntent,
    /// One target the apply was assigned to.
    ApplyAssignment,
    /// One presented plan an assignment came from.
    PlanOrigin,
    /// What the apply reached.
    ApplyOutcome,
    /// What one site did during the apply.
    SiteOutcome,
}

/// One field's key and scalar shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
    /// The literal key as it appears left of `=`.
    pub key: &'static str,
    /// What the atom right of `=` may be.
    pub ty: FieldType,
}

const fn f(key: &'static str, ty: FieldType) -> Field {
    Field { key, ty }
}

/// The account field every record ends with.
const ACCOUNT_FIELD: Field = f("account", FieldType::Closed(ACCOUNT));

const INVOCATION_FIELDS: &[Field] = &[
    f("mode", FieldType::Closed(MODE)),
    f("started", FieldType::OptionalWide),
    f("argv", FieldType::Closed(OPAQUE_STATE)),
    f("target", FieldType::Closed(OPAQUE_STATE)),
    f("attempt", FieldType::Count),
    ACCOUNT_FIELD,
];

const SOURCE_FIELDS: &[Field] = &[
    f("ordinal", FieldType::Count),
    f("role", FieldType::Closed(SOURCE_ROLE)),
    f("digest", FieldType::Digest),
    f("bytes", FieldType::Wide),
    f("path", FieldType::Closed(OPAQUE_STATE)),
    f("excerpt", FieldType::Closed(OPAQUE_STATE)),
    f("class", FieldType::Closed(SOURCE_CLASS)),
    f("content", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const ADMISSION_FIELDS: &[Field] = &[
    f("outcome", FieldType::Closed(ADMISSION_OUTCOME)),
    f("records", FieldType::Wide),
    f("bytes", FieldType::Wide),
    f("stream", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const PRESENTED_PLAN_FIELDS: &[Field] = &[
    f("planning-input", FieldType::Digest),
    f("presented-plan", FieldType::Digest),
    f("planned-image", FieldType::OptionalDigest),
    ACCOUNT_FIELD,
];

const SITE_DECISION_FIELDS: &[Field] = &[
    f("leaf", FieldType::Count),
    f("member", FieldType::OptionalCount),
    f("ast", FieldType::Count),
    f("disposition", FieldType::Closed(DISPOSITION)),
    f("shell", FieldType::Closed(OPAQUE_STATE)),
    f("locator", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const REGION_DECISION_FIELDS: &[Field] = &[
    f("region", FieldType::Count),
    f("ast", FieldType::Count),
    f("disposition", FieldType::Closed(DISPOSITION)),
    f("routes", FieldType::Wide),
    f("shell", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const LOAD_DECISION_FIELDS: &[Field] = &[
    f("ordinal", FieldType::Count),
    f("outcome", FieldType::Closed(LOAD_OUTCOME)),
    f("name", FieldType::Closed(OPAQUE_STATE)),
    f("custody", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const SITE_CLASSIFICATION_FIELDS: &[Field] = &[
    f("leaf", FieldType::Count),
    f("member", FieldType::OptionalCount),
    f("ast", FieldType::Count),
    f("class", FieldType::Closed(SITE_CLASS)),
    f("verdict-lane", FieldType::Closed(BOOL)),
    f("invalidator", FieldType::Closed(BOOL)),
    f("operands", FieldType::Count),
    f("dropped", FieldType::Count),
    ACCOUNT_FIELD,
];

const SOLVE_CERTIFICATION_FIELDS: &[Field] = &[
    f("pass", FieldType::Closed(SOLVE_PASS)),
    f("consistent", FieldType::Closed(BOOL)),
    f("tripped", FieldType::Closed(BOOL)),
    ACCOUNT_FIELD,
];

const PROBE_SHIP_FIELDS: &[Field] = &[
    f("leaf", FieldType::Count),
    f("member", FieldType::OptionalCount),
    f("lane", FieldType::Closed(SHIP_LANE)),
    f("source", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const SURVIVAL_FIELDS: &[Field] = &[
    f("leaf", FieldType::Count),
    f("member", FieldType::OptionalCount),
    f("outcome", FieldType::Closed(SURVIVAL_OUTCOME)),
    f("wall", FieldType::OptionalCount),
    f("aggregate", FieldType::OptionalCount),
    f("poison", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

// `subject` is polymorphic and read off `kind`: a leaf for the site-keyed kinds, a region
// ordinal (the `region-decision.region` space) for the region-keyed ones, and absent for the
// import and defensive-emission kinds, which own neither axis. `member` is the second site
// axis and is absent wherever `subject` is not a leaf.
const RENDER_DECISION_FIELDS: &[Field] = &[
    f("subject", FieldType::OptionalCount),
    f("member", FieldType::OptionalCount),
    f("kind", FieldType::Closed(RENDER_KIND)),
    f("detail", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const NARRATIVE_FIELDS: &[Field] = &[
    f("ordinal", FieldType::Count),
    f("speech", FieldType::Closed(SPEECH_ACT)),
    f("kind", FieldType::Closed(NARRATIVE_KIND)),
    f("operands", FieldType::Count),
    f("dropped", FieldType::Count),
    ACCOUNT_FIELD,
];

const LICENSOR_FIELDS: &[Field] = &[
    f("leaf", FieldType::Count),
    f("member", FieldType::OptionalCount),
    f("license", FieldType::Closed(LICENSE_VERB)),
    f("custody", FieldType::Closed(LICENSE_CUSTODY)),
    f("locus", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

const PROJECTION_OMISSION_FIELDS: &[Field] = &[
    f("species", FieldType::Closed(SPINE_SPECIES)),
    f("count", FieldType::Count),
    f("reason", FieldType::Closed(OMISSION_REASON)),
    ACCOUNT_FIELD,
];

const APPLY_INTENT_FIELDS: &[Field] = &[
    f("session", FieldType::Digest),
    f("generation", FieldType::Digest),
    f("policy", FieldType::Closed(APPLY_POLICY)),
    f("assignments", FieldType::Count),
    f("origin-state", FieldType::Closed(ORIGIN_STATE)),
    ACCOUNT_FIELD,
];

const APPLY_ASSIGNMENT_FIELDS: &[Field] = &[
    f("ordinal", FieldType::Count),
    f("target", FieldType::Closed(OPAQUE_STATE)),
    f("context", FieldType::Closed(OPAQUE_STATE)),
    f("image", FieldType::Digest),
    f("image-state", FieldType::Closed(IMAGE_STATE)),
    f("origins", FieldType::Count),
    ACCOUNT_FIELD,
];

const PLAN_ORIGIN_FIELDS: &[Field] = &[
    f("assignment", FieldType::Count),
    f("ordinal", FieldType::Count),
    f("receipt", FieldType::Digest),
    f("presented", FieldType::Digest),
    ACCOUNT_FIELD,
];

const APPLY_OUTCOME_FIELDS: &[Field] = &[
    f("intent", FieldType::Digest),
    f("terminal", FieldType::Closed(TERMINAL_STATE)),
    f("sites", FieldType::Count),
    f("durable", FieldType::Closed(DURABLE_STATE)),
    ACCOUNT_FIELD,
];

const SITE_OUTCOME_FIELDS: &[Field] = &[
    f("ordinal", FieldType::Count),
    f("assignment", FieldType::Count),
    f("leaf", FieldType::Count),
    f("member", FieldType::OptionalCount),
    f("status", FieldType::Closed(SITE_STATUS)),
    f("tool-rc", FieldType::OptionalCount),
    f("stdout", FieldType::Closed(OPAQUE_STATE)),
    f("stderr", FieldType::Closed(OPAQUE_STATE)),
    ACCOUNT_FIELD,
];

impl RecordKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 20] = [
        Self::Invocation,
        Self::Source,
        Self::Admission,
        Self::PresentedPlan,
        Self::SiteDecision,
        Self::RegionDecision,
        Self::LoadDecision,
        Self::SiteClassification,
        Self::SolveCertification,
        Self::ProbeShip,
        Self::Survival,
        Self::RenderDecision,
        Self::Narrative,
        Self::Licensor,
        Self::ProjectionOmission,
        Self::ApplyIntent,
        Self::ApplyAssignment,
        Self::PlanOrigin,
        Self::ApplyOutcome,
        Self::SiteOutcome,
    ];

    /// The literal kind word in a record line.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Invocation => "invocation",
            Self::Source => "source",
            Self::Admission => "admission",
            Self::PresentedPlan => "presented-plan",
            Self::SiteDecision => "site-decision",
            Self::RegionDecision => "region-decision",
            Self::LoadDecision => "load-decision",
            Self::SiteClassification => "site-classification",
            Self::SolveCertification => "solve-certification",
            Self::ProbeShip => "probe-ship",
            Self::Survival => "survival",
            Self::RenderDecision => "render-decision",
            Self::Narrative => "narrative",
            Self::Licensor => "licensor",
            Self::ProjectionOmission => "projection-omission",
            Self::ApplyIntent => "apply-intent",
            Self::ApplyAssignment => "apply-assignment",
            Self::PlanOrigin => "plan-origin",
            Self::ApplyOutcome => "apply-outcome",
            Self::SiteOutcome => "site-outcome",
        }
    }

    /// The kind a literal word names.
    #[must_use]
    pub fn of_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.token() == token)
    }

    /// This kind's fields, in the exact order a record line spells them.
    ///
    /// No wildcard arm: a new kind stops this compiling until its fields are declared.
    #[must_use]
    pub const fn fields(self) -> &'static [Field] {
        match self {
            Self::Invocation => INVOCATION_FIELDS,
            Self::Source => SOURCE_FIELDS,
            Self::Admission => ADMISSION_FIELDS,
            Self::PresentedPlan => PRESENTED_PLAN_FIELDS,
            Self::SiteDecision => SITE_DECISION_FIELDS,
            Self::RegionDecision => REGION_DECISION_FIELDS,
            Self::LoadDecision => LOAD_DECISION_FIELDS,
            Self::SiteClassification => SITE_CLASSIFICATION_FIELDS,
            Self::SolveCertification => SOLVE_CERTIFICATION_FIELDS,
            Self::ProbeShip => PROBE_SHIP_FIELDS,
            Self::Survival => SURVIVAL_FIELDS,
            Self::RenderDecision => RENDER_DECISION_FIELDS,
            Self::Narrative => NARRATIVE_FIELDS,
            Self::Licensor => LICENSOR_FIELDS,
            Self::ProjectionOmission => PROJECTION_OMISSION_FIELDS,
            Self::ApplyIntent => APPLY_INTENT_FIELDS,
            Self::ApplyAssignment => APPLY_ASSIGNMENT_FIELDS,
            Self::PlanOrigin => PLAN_ORIGIN_FIELDS,
            Self::ApplyOutcome => APPLY_OUTCOME_FIELDS,
            Self::SiteOutcome => SITE_OUTCOME_FIELDS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_ends_with_account_so_a_new_key_has_one_append_position() {
        // The append rule is what keeps the required field order stable as keys are added:
        // `account` is last on every kind, so a new key has exactly one legal position.
        for kind in RecordKind::ALL {
            let last = kind.fields().last().map(|field| field.key);
            assert_eq!(last, Some("account"), "{}", kind.token());
        }
    }

    #[test]
    fn no_kind_repeats_a_field_key() {
        // Duplicate fields are refused by the grammar, so a table that declared one would
        // make a record unwritable rather than ambiguous — fail here instead.
        for kind in RecordKind::ALL {
            let mut keys: Vec<&str> = kind.fields().iter().map(|field| field.key).collect();
            let before = keys.len();
            keys.sort_unstable();
            keys.dedup();
            assert_eq!(before, keys.len(), "{} repeats a key", kind.token());
        }
    }

    #[test]
    fn kind_tokens_are_distinct_and_round_trip() {
        for kind in RecordKind::ALL {
            assert_eq!(RecordKind::of_token(kind.token()), Some(kind));
        }
        let mut tokens: Vec<&str> = RecordKind::ALL.iter().map(|k| k.token()).collect();
        let before = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(before, tokens.len(), "two kinds share a token");
    }

    #[test]
    fn every_closed_token_matches_the_field_key_alphabet() {
        // The record grammar's field-key production is `lower-alpha *(lower-alpha | digit |
        // "-")`, and every closed token is spelled in the same alphabet so a token can never
        // need quoting or escaping inside a record line.
        for kind in RecordKind::ALL {
            for field in kind.fields() {
                assert!(is_key_shaped(field.key), "key {}", field.key);
                if let FieldType::Closed(tokens) = field.ty {
                    for token in tokens {
                        assert!(is_key_shaped(token), "token {token} on {}", kind.token());
                    }
                }
            }
        }
    }

    fn is_key_shaped(text: &str) -> bool {
        let mut bytes = text.bytes();
        let Some(first) = bytes.next() else {
            return false;
        };
        first.is_ascii_lowercase()
            && bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    }

    #[test]
    fn canonical_integers_refuse_leading_zeros_signs_and_overlong_runs() {
        assert_eq!(canonical_u64("0"), Some(0));
        assert_eq!(canonical_u64("10"), Some(10));
        assert_eq!(canonical_u64("00"), None, "leading zero");
        assert_eq!(canonical_u64("01"), None, "leading zero");
        assert_eq!(canonical_u64("+1"), None, "sign");
        assert_eq!(canonical_u64("-1"), None, "sign");
        assert_eq!(canonical_u64(""), None);
        assert_eq!(canonical_u64(" 1"), None);
        assert_eq!(canonical_u64("1 "), None);
        assert_eq!(canonical_u64("1_0"), None);
        assert_eq!(canonical_u64(&"9".repeat(21)), None, "digit cap");
        assert_eq!(
            canonical_u32(&u64::from(u32::MAX).to_string()),
            Some(u32::MAX)
        );
        assert_eq!(
            canonical_u32(&(u64::from(u32::MAX) + 1).to_string()),
            None,
            "a u32 field refuses a u64 value"
        );
    }

    #[test]
    fn a_digest_is_exactly_sixty_four_lowercase_hex() {
        assert!(is_digest(&"a".repeat(64)));
        assert!(is_digest(&"0".repeat(64)));
        assert!(!is_digest(&"a".repeat(63)), "short");
        assert!(!is_digest(&"a".repeat(65)), "long");
        assert!(!is_digest(&"A".repeat(64)), "uppercase");
        assert!(!is_digest(&"g".repeat(64)), "non-hex");
    }

    #[test]
    fn absent_is_admitted_only_where_the_table_says_so() {
        assert!(FieldType::OptionalCount.admits(ABSENT));
        assert!(FieldType::OptionalWide.admits(ABSENT));
        assert!(FieldType::OptionalDigest.admits(ABSENT));
        assert!(!FieldType::Count.admits(ABSENT));
        assert!(!FieldType::Wide.admits(ABSENT));
        assert!(!FieldType::Digest.admits(ABSENT));
        assert!(!FieldType::Closed(BOOL).admits(ABSENT));
    }

    #[test]
    fn a_closed_field_admits_its_own_tokens_and_nothing_else() {
        let ty = FieldType::Closed(DISPOSITION);
        for token in DISPOSITION {
            assert!(ty.admits(token));
        }
        assert!(!ty.admits("Run"), "case is exact");
        assert!(!ty.admits("run "), "no trailing space");
        assert!(!ty.admits(""), "no empty atom");
        assert!(!ty.admits("survive"), "not a disposition");
    }
}
