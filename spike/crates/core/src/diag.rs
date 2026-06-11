//! The structured-diagnostic API spine — the round-22 arch-3 design (`Research/notes/22B`,
//! ratified; `plans/22A` concl-7/concl-8; `held-4` sanctioned design-for-keeps exception).
//!
//! This is the GOOD shape made the cheapest to write (`22B` §0): a [`Diag`] is a typed
//! [`DiagCode`] payload + a MANDATORY primary [`SpanLabel`] + ordered [`SubDiag`] children +
//! an optional [`Suggestion`]; severity comes ONLY from the [`registry`], never a constructor.
//! Cribs rustc's `Diag` data model (`crib-1`/`crib-2`) and Elm's narrative render tone
//! (`crib-6`); REFUSES rustc's Fluent/derive authoring DSL by name (`refuse-1` — also forced by
//! `inv-no-unsafe`: proc-macros forbidden workspace-wide). The *types* do the work the derive
//! DSL did, with the stock compiler as the only enforcement engine.
//!
//! # Coexistence (round-22 migration scope)
//!
//! Exactly THREE codes are migrated onto this spine as the proving set (`22B` §5):
//! [`DiagCode::SiteUnresolvable`], [`DiagCode::RenderHeredocRefused`],
//! [`DiagCode::CmdsubOperandTop`]. Every other legacy give-up stays on [`crate::Diagnostic`]
//! (the gate-grep allow-list in `core/tests/diag_tidy.rs` names them); a later mechanical
//! sweep (B4) empties that list. A [`Diag`] lowers to the legacy stream via
//! [`Diag::to_legacy`] so the existing `report()`/erasability/coverage consumers keep working
//! unchanged while the spine is proven end-to-end.
//!
//! # Invariants honored here (cite the slug)
//!
//! * `inv-no-throw` — every constructor returns data; nothing panics (`22B` §3).
//! * `inv-determinism` — ordered collections only (`Vec`, never a hashed map iterated to
//!   output); [`registry`] is a pure `match`.
//! * `inv-no-unsafe` — stock `#[derive]`s only; no macros, no proc-macros.
//! * `inv-referent-agnostic` — a payload's text excerpt is an [`OutClaim`] (an interned
//!   handle), never decoded for meaning; the [`ProvId`] cause is opaque and non-`Display`.
//! * `inv-site-keyed-results` — [`SiteId`] preserves command-site keying (promoted from the
//!   cli's `RecordKey`).

use crate::{LeafId, OutClaim, ProvId, Severity, Span};

// ===========================================================================
// The catalog enum (exhaustive spine) + typed per-variant payloads (type-sketch-1)
// ===========================================================================

/// The exhaustive catalog of every diagnostic the analyzer emits *through this spine*. One
/// variant per give-up/disclosure class; the compiler enforces handle-every-code (`226` §12 /
/// `22A` concl-7) — every `match` on this enum (the [`registry`], [`render_cli`],
/// [`render_artifact_comment`], the OOB projection) breaks until a new variant is handled.
///
/// NO `#[non_exhaustive]` (conductor decision, verified against the workspace): `#[non_exhaustive]`
/// forces DOWNSTREAM-crate matches to add a wildcard arm — the exact opposite of the
/// workspace-wide handle-every-code the catalog exists for. Every consumer here is an internal
/// workspace crate, so exhaustiveness is the feature, not a hazard.
///
/// Each variant carries a TYPED payload demanding exactly the objects the diagnostic cites
/// (`22B` `type-sketch-1`, the capability instinct made structural): you cannot author the
/// diagnostic wrong because you cannot NAME the wrong objects. Adding a code is ONE variant
/// here + ONE [`registry`] arm + ONE arm in each render — the `22B` §7 friction test, bounded
/// and compiler-guided.
///
/// Scope: only the three `22B` §5 worked examples are migrated this round; the rest stay on
/// [`crate::Diagnostic`] (coexistence is deliberate — `22B` §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagCode {
    /// A `$(…)`/runtime-dynamic operand (or the command word itself) forced a command to ⊤
    /// (`Opaque` ⇒ it runs, never elided). The find-3 no-silent-phantoms disclosure.
    CmdsubOperandTop(CmdsubOperandTop),
    /// A probe could not ship a read-only check for this site ⇒ the apply runs it
    /// (`kFAIL-perform`). The cli-edge readout of `ProbePlan::unresolvable`.
    SiteUnresolvable(SiteUnresolvable),
    /// The leaf-exact render REFUSED to elide a licensed leaf because it carries a heredoc —
    /// its span covers the `<<` opener, not the body — so the leaf runs verbatim
    /// (`kFAIL-perform`; arch-1 d-6). An Error-class give-up (a broken artifact would ship
    /// otherwise).
    RenderHeredocRefused(RenderHeredocRefused),
}

impl DiagCode {
    /// The stable wire/grep slug for this code (`22B-fork-wire-code` = string slug; a
    /// WIRE-FORMAT COMMITMENT — flagged). Stable across variant reordering (unlike a numeric
    /// discriminant), greppable, and the key the OOB lane's `code=` field carries
    /// (`226` finding-6, TS's code-stable discipline). These slugs match the legacy
    /// [`crate::DiagCode`] strings the migrated sites used, so existing `expected-diagnostics`
    /// fixtures and the coverage bridge keep matching.
    #[must_use]
    pub fn slug(&self) -> &'static str {
        match self {
            DiagCode::CmdsubOperandTop(_) => "dq-cmdsub-operand-top",
            DiagCode::SiteUnresolvable(_) => "dq-site-unresolvable",
            DiagCode::RenderHeredocRefused(_) => "render-heredoc-refused",
        }
    }
}

/// The position in a command's argv that went ⊤ (`22B` `type-sketch-1`): the command word
/// itself, or a 1-based operand index (excluding the command word). A newtype over the bare
/// `&str` the legacy `cmdsub_operand_top` took — the value plane is cause-erased to ⊤ at this
/// point, so the diagnostic names the POSITION, never the original text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandPosition {
    /// `argv[0]` — the command word is itself a `$(…)`/dynamic value.
    CommandWord,
    /// Operand `n` (1-based, command word excluded) is a `$(…)`/dynamic value.
    Operand(u32),
}

impl OperandPosition {
    /// The fact-plane prose for this position (`the command word` / `operand N`). Pure, no
    /// allocation beyond the formatted operand index.
    #[must_use]
    pub fn describe(self) -> String {
        match self {
            OperandPosition::CommandWord => "the command word".to_owned(),
            OperandPosition::Operand(n) => format!("operand {n}"),
        }
    }
}

/// Payload of [`DiagCode::CmdsubOperandTop`]: the ⊤-origin site, WHICH position went ⊤, and an
/// optional ⊤-cause receipt (`228` dc-1 — the exempt-plane hook that links this origin to its
/// poisoned downstream consumers without each consumer emitting). The `cause` is EXEMPT-plane
/// (it is a [`ProvId`], opaque and non-`Display`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmdsubOperandTop {
    /// The command-site that went ⊤.
    pub site: SiteId,
    /// Which argv position is the `$(…)`/dynamic value.
    pub position: OperandPosition,
    /// The ⊤-cause origin (arch-1 `ProvId` arena), if minted. EXEMPT-plane (`Exempt::ReceiptId`):
    /// it rides the diagnostic for the why-lens/dashboard dedup but reaches no artifact and
    /// drives no decision.
    pub cause: Option<ProvId>,
}

/// Payload of [`DiagCode::SiteUnresolvable`]: the probe-unresolvable site and the
/// referent-agnostic source excerpt naming it. The typed payload replaces the legacy
/// constructor's bare `&str` leaf/source pair (`22B` `worked-1`): the [`SiteId`] is first-class
/// and the excerpt is an [`OutClaim`] (an interned handle), never a fumble-able `&str`.
///
/// NB (conductor re-inventory): `22B` `type-sketch-1` sketched a `probe: ProbeSiteRef` field,
/// but at HEAD the cli's `ProbePlan::unresolvable` is a bare `Vec<LeafId>` — there is no
/// first-class probe-record handle to demand, and minting one is out of this round's scope. The
/// [`SiteId`] IS the blamed handle (the probe record keys back to it by `LeafId`); flagged
/// `tc-probe-site-ref` in the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SiteUnresolvable {
    /// The probe-unresolvable command-site (the apply runs it).
    pub site: SiteId,
    /// The site's source command text, referent-agnostic (`inv-referent-agnostic`): rendered
    /// for display, never decoded to infer meaning.
    pub source_excerpt: OutClaim,
}

/// Payload of [`DiagCode::RenderHeredocRefused`]: the heredoc-bearing site the leaf-exact render
/// refused to elide (`22B` `worked-2`). The legacy form was an inline literal (`21Z`: "not even
/// a named const"); the typed payload makes it a first-class enum variant the grep gate sees
/// and the dashboard can group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderHeredocRefused {
    /// The heredoc-carrying command-site that runs verbatim instead of being elided.
    pub site: SiteId,
}

// ===========================================================================
// First-class site identity (type-sketch-5) — the slot, not the fleet machinery
// ===========================================================================

/// A diagnostic's first-class site identity (`22B` `type-sketch-5`; promoted from the cli's
/// `RecordKey`). The `site N.M` keying (`member` = the in-loop fact-family index,
/// `inv-site-keyed-results`) is the FINE key; the COARSE key for fleet rollup is a slot
/// ([`GroupingKey`]) the machinery does not yet fill (`22B-fork-scope-key` = STUB coarse=fine).
///
/// Promoting this into `core` preserves site-keying end-to-end (`inv-site-keyed-results`): the
/// same `(leaf, member)` pair the cli's probe-records and the apply plan's steps share.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SiteId {
    /// The stable command-site leaf (the plan's `Step::leaf` for the same source command).
    pub leaf: LeafId,
    /// The MEMBER index for an in-loop Members site (`site N.M`): `Some(m)` ⇒ member `m` of a
    /// fact-family, `None` ⇒ an ordinary single-fact site.
    pub member: Option<u32>,
}

impl SiteId {
    /// A single-fact (non-member) site.
    #[must_use]
    pub fn leaf(leaf: LeafId) -> Self {
        Self { leaf, member: None }
    }
}

/// The hierarchical grouping keys for ⊤-cascade dedup and fleet aggregation (`22B`
/// `type-sketch-5`; `228` dc-3: `CodeChecker` context-free-v2 + Sentry match-either-hash). The
/// FINE key distinguishes per-host detail (the engineer debugging one host); the COARSE key
/// collapses M manifestations of one cause for the admin ("one rot, 12 hosts"). Both served, per
/// the AGENTS two-user exclusion-check.
///
/// This is a TRAIT SLOT, not a built subsystem (`22A` arch-2: "design the slot, don't build the
/// fleet machinery"): the fleet rollup that CONSUMES coarse keys is out of scope this round.
/// `22B-fork-scope-key` is STUBBED — [`coarse_key`](GroupingKey::coarse_key) degenerates to the
/// fine key, because a real coarse key needs an enclosing-structural-scope id the spike does not
/// yet surface as first-class. A degenerate coarse=fine is honest for now (it just means no
/// cross-site collapse happens yet); the trait shape is the deliverable.
pub trait GroupingKey {
    /// The fine key — distinguishes per-site detail. Today: `(code-slug, site)`.
    fn fine_key(&self) -> FineKey;
    /// The coarse key — collapses manifestations of one cause for fleet rollup. STUBBED to the
    /// fine key this round (`22B-fork-scope-key`): an honest degenerate that simply does no
    /// collapsing until enclosing-scope ids are surfaced.
    fn coarse_key(&self) -> CoarseKey;
}

/// The fine grouping key (`228` dc-3): a code slug paired with the site. Distinguishes the
/// per-host detail an engineer debugging one host wants. Ordered/`Hash` so a render can group by
/// it deterministically (`inv-determinism`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FineKey {
    /// The diagnostic code's stable slug.
    pub code: &'static str,
    /// The originating site.
    pub site: SiteId,
}

/// The coarse grouping key (`228` dc-3): for fleet rollup, drops the call-site so M
/// manifestations of one cause collapse. STUBBED this round (`22B-fork-scope-key`): it wraps the
/// fine key unchanged (degenerate coarse=fine), so no collapse happens yet. When
/// enclosing-structural-scope ids are surfaced, this gains an `enclosing-scope` field and drops
/// `site.leaf`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CoarseKey {
    /// The fine key, verbatim (the degenerate stub — `22B-fork-scope-key`).
    pub fine: FineKey,
}

impl GroupingKey for Diag {
    fn fine_key(&self) -> FineKey {
        FineKey {
            code: self.code.slug(),
            site: self.code.site(),
        }
    }
    fn coarse_key(&self) -> CoarseKey {
        CoarseKey {
            fine: self.fine_key(),
        }
    }
}

impl DiagCode {
    /// The originating [`SiteId`] this code's payload cites — every migrated payload carries a
    /// `site`, so the grouping keys can be computed uniformly without each render arm digging it
    /// out. (A future code with no single site would need a different grouping story; none
    /// exists yet.)
    #[must_use]
    fn site(&self) -> SiteId {
        match self {
            DiagCode::CmdsubOperandTop(p) => p.site,
            DiagCode::SiteUnresolvable(p) => p.site,
            DiagCode::RenderHeredocRefused(p) => p.site,
        }
    }
}

// ===========================================================================
// The Diag value (type-sketch-2): message + labeled spans + children + suggestion
// ===========================================================================

/// One diagnostic, ready to render three ways or ride the OOB lane (`22B` `type-sketch-2`). The
/// structured shape cribbed from rustc (`crib-1`): a typed [`DiagCode`] payload, a MANDATORY
/// primary [`SpanLabel`] (the region the render points at), a window of optional secondary
/// labels (a ⊤-cause-site and a poisoned-site live in ONE diagnostic — `228`), and ordered
/// [`SubDiag`] children. Severity is NOT a field — it is looked up from the [`registry`] by
/// `code` (`crib-4`), so it cannot drift per-site.
///
/// `inv-no-throw`: a `Diag` is data; constructing it never panics. The mandatory primary span
/// is the structural fix for `21Z` drop-A/drop-B together — there is no span-less `Diag`, so the
/// CLI render cannot drop what was never optional and an author cannot forget it.
#[derive(Debug, Clone)]
pub struct Diag {
    /// The catalog code, carrying its typed payload (`type-sketch-1`).
    pub code: DiagCode,
    /// The one span the region renders around — MANDATORY (drop-A/drop-B impossible).
    pub primary: SpanLabel,
    /// Additional labeled spans (the cause-site, the poisoned-site — `228`).
    pub secondary: Vec<SpanLabel>,
    /// Ordered notes/helps (`crib-1`/`crib-3`): facts then remediation.
    pub children: Vec<SubDiag>,
    /// The actionable fix, if any (`crib-2`).
    pub suggestion: Option<Suggestion>,
}

/// A span with an optional label — the rustc primary/secondary-label model (`crib-1`). Fixes
/// `21Z` drop-B: a span is no longer `Option`-on-the-whole-`Diag`; the PRIMARY span is mandatory,
/// secondaries are the optional extras.
#[derive(Debug, Clone)]
pub struct SpanLabel {
    /// The source span (mandatory on the primary).
    pub span: Span,
    /// The caret-label prose ("this went ⊤", "first poisoned here"), if any.
    pub label: Option<String>,
}

/// A note or help child (`crib-1`/`crib-3`). `Help` is remediation-facing (CLI-only,
/// fact-plane-exempt); `Note` is additional fact context. The split lets the render model drop
/// helps from artifact-eligible output while keeping notes (`22B` `type-sketch-2`).
#[derive(Debug, Clone)]
pub enum SubDiag {
    /// Additional fact context (the primary message states the fact; a Note adds to it).
    Note(String),
    /// Remediation guidance (`crib-3`: "only the help should suggest how to fix"). CLI-only.
    Help(String),
}

// ===========================================================================
// Suggestion + Applicability + RemediationClass (type-sketch-3)
// ===========================================================================

/// An actionable fix (`22B` `type-sketch-3`). Cribbed from rustc (`crib-2`): a message + a
/// confidence the tooling reads to decide auto-apply. Dorc adds the human-ratified
/// [`RemediationClass`] (ru-6): the [`Applicability`] says HOW confident, the class says WHAT
/// kind of fix. Together they drive a render's grouping and a future `dorc fix` story.
///
/// No machine-applicable SPAN-EDIT to a shipped `.sh` artifact this round: a suggestion is
/// admin-facing guidance (CLI), not an artifact rewrite — the artifact stays fact-plane (ru-12).
#[derive(Debug, Clone)]
pub struct Suggestion {
    /// The remediation prose ("declare nginx's `installed` selector in the oracle").
    pub message: String,
    /// How confident the fix is correct (`crib-2`, verbatim from rustc).
    pub applicability: Applicability,
    /// Which user action clears the origin (ru-6 — the render/grouping axis).
    pub remediation: RemediationClass,
}

/// rustc's confidence model, verbatim (`crib-2`, re-verified live against `rustc_lint_defs`). The
/// discipline that matters: a tool decides whether to auto-apply from the applicability, not from
/// the prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    /// Auto-apply; preserves meaning.
    MachineApplicable,
    /// Valid code but uncertain — consult the user.
    MaybeIncorrect,
    /// Contains `(…)`-style holes; cannot auto-apply.
    HasPlaceholders,
    /// Confidence unspecified.
    Unspecified,
}

/// The human-ratified render axis (ru-6, `224` §7; `22A` arch-2): classify every remediable
/// origin by what USER ACTION clears it, and rank/group the render by that. The dashboard's
/// four-cause decomposition, generalized per-site.
///
/// The two-user exclusion-check (AGENTS): [`AuthorOracle`](Self::AuthorOracle) speaks to the
/// dev-team author, [`FixBookLine`](Self::FixBookLine) to the ops admin — the two users get
/// separate remediation verbs. [`Structural`](Self::Structural) is the honest "no user action
/// clears this; it's a Dorc limitation" bucket — load-bearing for not lying to an admin that
/// they can fix a ⊤ that is really ours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationClass {
    /// An oracle must be written/extended (the dev-team author).
    AuthorOracle,
    /// A missing kind/selector/Query declaration (oracle or book).
    AddDeclaration,
    /// The book line itself is wrong/ambiguous (the admin).
    FixBookLine,
    /// Unmodeled construct — Dorc itself must grow; no user fix.
    Structural,
}

// ===========================================================================
// The registry: severity-as-data with a floor tier (type-sketch-4)
// ===========================================================================

/// Per-code severity (`crib-4`) and un-overridable floor (`crib-5`). The ONLY place severity is
/// decided — never at a construction site (the new API has no severity constructor). A single
/// `match` keyed on the code's discriminant; adding a code adds one arm (the friction test).
///
/// The floor column is PROPOSED (`22B-fork-floor-membership` / `22A` gate2-ask-1): the human
/// disposes which codes pin to the floor at the PR. My per-code judgment, clearly marked:
/// * [`DiagCode::RenderHeredocRefused`] ⇒ Error + [`Floor::WarnOrDeny`] — a kFAIL-correctness
///   give-up: silencing it below a warning would hide a converged mutator running because the
///   render could not safely elide it (a broken-artifact-adjacent class).
/// * [`DiagCode::SiteUnresolvable`] / [`DiagCode::CmdsubOperandTop`] ⇒ Note + [`Floor::None`] —
///   pure disclosures (the apply runs the site either way; the floor would over-constrain a
///   benign "ran on every apply" note).
#[must_use]
#[expect(
    clippy::match_same_arms,
    reason = "the catalog's friction test is one ROW PER CODE (22B §7) — adding a code adds one \
              arm; merging the two Note-class arms by `|` would hide that a code has a declared \
              row and break the per-code-grading shape, so each code keeps its own arm even when \
              two share a CodeSpec value"
)]
pub fn registry(code: &DiagCode) -> CodeSpec {
    match code {
        DiagCode::CmdsubOperandTop(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
        },
        DiagCode::SiteUnresolvable(_) => CodeSpec {
            severity: Severity::Note,
            floor: Floor::None,
        },
        DiagCode::RenderHeredocRefused(_) => CodeSpec {
            severity: Severity::Error,
            // PROPOSED floor (22B-fork-floor-membership): a render-refusal that would otherwise
            // ship a broken artifact must never be silenced below a warning.
            floor: Floor::WarnOrDeny,
        },
    }
}

/// A code's declared severity + floor (the [`registry`] row). Severity comes from HERE, never a
/// constructor (`crib-4`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeSpec {
    /// The declared severity (the gate-3 floor keys on `Error`).
    pub severity: Severity,
    /// The un-overridable floor (`crib-5`).
    pub floor: Floor,
}

/// The un-overridable floor (`crib-5`; rustc `future-incompatible` = a floor, not a level). When
/// admin override lands (NOT this round), the floor-pinned codes cannot be silenced — the
/// few-chosen non-negotiables rustc's `forbid`/`force-warn` protect (`226` sev-1).
///
/// NB `22B-fork-severity-help` = NO top-level `Severity::Help` (conductor decision): help is a
/// [`SubDiag::Help`] child, so the registry never returns a `Help` severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Floor {
    /// No floor — an admin/oracle may silence the code freely.
    None,
    /// May raise to Error but NEVER drop below Warning.
    WarnOrDeny,
    /// The rustc `forbid`/`force-warn` analog: exactly this severity, no override.
    Pinned,
}

// ===========================================================================
// The builder / constructor API (type-sketch-7) — the friction surface
// ===========================================================================

impl Diag {
    /// The one mint (`22B` `type-sketch-7` layer A): name the code (with its typed payload
    /// constructed inline, where the give-up site already holds the objects), point at the
    /// primary span. There is no severity constructor — severity is [`registry`] data
    /// (`crib-4`). This is the cheapest authoring path and lands the GOOD shape by default.
    #[must_use]
    pub fn new(code: DiagCode, primary: Span) -> Self {
        Self {
            code,
            primary: SpanLabel {
                span: primary,
                label: None,
            },
            secondary: Vec::new(),
            children: Vec::new(),
            suggestion: None,
        }
    }

    /// Label the primary span (`type-sketch-7` layer B — small-`f` fluent chaining, NOT
    /// Fluent-the-i18n-system). One obvious call; nothing beyond [`new`](Self::new) is mandatory.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.primary.label = Some(label.into());
        self
    }

    /// Add a labeled SECONDARY span — the cause-site or the poisoned-site, in ONE diagnostic
    /// (`228`; `crib-1`).
    #[must_use]
    pub fn secondary(mut self, span: Span, label: impl Into<String>) -> Self {
        self.secondary.push(SpanLabel {
            span,
            label: Some(label.into()),
        });
        self
    }

    /// Add a [`SubDiag::Note`] child (additional fact context).
    #[must_use]
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.children.push(SubDiag::Note(note.into()));
        self
    }

    /// Add a [`SubDiag::Help`] child (remediation guidance; CLI-only).
    #[must_use]
    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.children.push(SubDiag::Help(help.into()));
        self
    }

    /// Attach the actionable [`Suggestion`] (`crib-2`).
    #[must_use]
    pub fn suggest(mut self, suggestion: Suggestion) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    /// This diagnostic's registry-declared severity ([`registry`] keyed by `code`; `crib-4`).
    #[must_use]
    pub fn severity(&self) -> Severity {
        registry(&self.code).severity
    }
}

// ===========================================================================
// The render model (§4): one Diag value, three lanes, authored once
// ===========================================================================

/// The CLI narrative render (`22B` `render-1`, the render-plane half of rec-1 two-surfaces).
/// Elm's four-part narrative (`crib-6`) over rustc's data (`crib-1`): title (severity+code) /
/// region (primary span) / problem (the label) / hints (notes then helps, and the suggestion as
/// a remediation-classed help). The simple no-caret-art form (`22B` `render-1` ~SUSPECT: a
/// simpler first cut satisfies every STRUCTURAL requirement; the caret window is a render-quality
/// refinement, ungated per `crib-7`). `src` resolves a span to source text; `interner` resolves
/// the [`OutClaim`] excerpt.
///
/// This is `render_cli` — EVERYTHING (the render plane): title, region, prose, helps, the
/// suggestion. The artifact-bound projection is [`render_artifact_comment`], which admits only
/// fact-plane fields. Returns the full multi-line narrative as a standalone `String`.
///
/// (The cli's `report()` does NOT call this directly — it lowers via [`Diag::to_legacy`] into the
/// existing `<stage>: <sev>[<code>]: <message>` + ` --> <span>` plumbing, so the title/region are
/// produced there. This function is the standalone render for a future direct surface and the
/// shape the unit tests pin.)
#[must_use]
pub fn render_cli(diag: &Diag, src: &str, interner: &crate::Interner) -> String {
    use std::fmt::Write;
    let spec = registry(&diag.code);
    let mut out = String::new();
    // title: `<severity>[<slug>]: <problem>` — the primary label is the problem statement
    // (Elm's "problem"); the registry severity is the level (crib-4).
    let _ = write!(
        out,
        "{}[{}]: {}",
        severity_word(spec.severity),
        diag.code.slug(),
        diag.primary.label.as_deref().unwrap_or("")
    );
    // region: the primary span as `<lo>:<hi> \`source\`` (the simple no-caret form). drop-A fix:
    // the span is rendered, never dropped.
    let _ = write!(out, "\n  --> {}", render_span(diag.primary.span, src));
    // secondary labeled spans (cause-then-effect in one window — 228).
    for sec in &diag.secondary {
        let _ = write!(out, "\n  --> {}", render_span(sec.span, src));
        if let Some(l) = &sec.label {
            let _ = write!(out, "  {l}");
        }
    }
    out.push_str(&render_body(diag, interner));
    out
}

/// The PROSE BODY of a diagnostic (`render-1`'s hints) — the children (notes then helps), the
/// remediation-classed suggestion, and the referent-agnostic excerpt — WITHOUT the title or
/// region. This is what [`Diag::to_legacy`] folds into a legacy `message` (the title + span are
/// produced by the cli's `report()` from the legacy `code`/`severity`/`span` fields, so the body
/// must not repeat them). Each part is one ` = note:`/` = help:` continuation line.
fn render_body(diag: &Diag, interner: &crate::Interner) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    // referent-agnostic excerpt: a SiteUnresolvable carries the source command text as an
    // OutClaim; surface it (resolved for display only — inv-referent-agnostic). First so the
    // legacy message keeps naming the source command (the cli test pins `make install`).
    if let DiagCode::SiteUnresolvable(p) = &diag.code {
        let _ = write!(
            out,
            "\n  = note: site runs `{}`",
            interner.resolve(p.source_excerpt.0)
        );
    }
    // hints: notes then helps (children, in authored order).
    for child in &diag.children {
        match child {
            SubDiag::Note(n) => {
                let _ = write!(out, "\n  = note: {n}");
            }
            SubDiag::Help(h) => {
                let _ = write!(out, "\n  = help: {h}");
            }
        }
    }
    // the suggestion, as a remediation-classed help (its [class] tag inline).
    if let Some(s) = &diag.suggestion {
        let _ = write!(
            out,
            "\n  = help: {} [{}]",
            s.message,
            remediation_tag(s.remediation)
        );
    }
    out
}

/// The artifact-comment render (`22B` `render-3`, the ru-12 weld). A shipped `.sh` artifact may
/// carry AT MOST a FACT-PLANE projection of a diagnostic — a provenance comment naming the
/// fact, never the narrative prose, the help/remediation, or any [`ProvId`]-derived receipt
/// text. The enforcement is THIS function's type: the exempt-plane fields (`suggestion`, helps,
/// `cause`) are simply not read here. Returns `None` when no fact-plane comment is warranted (a
/// disclosure that belongs only in the render plane).
///
/// rec-1 two-surfaces (round-22 standing ruling): this is the artifact surface; [`render_cli`] is
/// the render/overlay surface. The adversarial erasability gate asserts the artifact is
/// byte-identical with receipts stripped — this partition is what makes that TRUE by
/// construction (the receipt fields never reach the bytes). The Error-class render-refusal is the
/// one migrated code whose fact is artifact-relevant (a leaf ran verbatim); the Notes are
/// render-plane disclosures and return `None`.
#[must_use]
pub fn render_artifact_comment(diag: &Diag) -> Option<String> {
    match &diag.code {
        // A render-refusal is a fact about the artifact (this leaf was NOT elided). Surface the
        // fact-plane site, never the prose. (Today the cli does not weave this into the artifact —
        // the existing provenance comments cover the elided sites; this is the SLOT, fact-plane by
        // construction, for when refusals annotate the artifact.)
        DiagCode::RenderHeredocRefused(p) => Some(format!(
            "# render-refused (heredoc): site {}{} runs verbatim",
            p.site.leaf.0,
            p.site.member.map(|m| format!(".{m}")).unwrap_or_default()
        )),
        // Pure render-plane disclosures: no fact-plane artifact comment (the apply runs the site;
        // the existing skip-unresolvable comment, if any, is the cli's, not this projection's).
        DiagCode::SiteUnresolvable(_) | DiagCode::CmdsubOperandTop(_) => None,
    }
}

/// The OOB-lane projection (`22B` `render-2`): the FACT-PLANE fields a diagnostic contributes to
/// the out-of-band site-keyed record lane — `{ site, code-slug, severity }`. No prose, no help,
/// no [`ProvId`]. The [`SiteId`] is already the lane's key, so a diagnostic and its OOB record
/// share identity for free. The `code=` field is the stable string slug (`22B-fork-wire-code` —
/// a WIRE-FORMAT COMMITMENT, flagged).
///
/// This is the slot (string-slug wire code), authored as a pure function of the `Diag`, not a
/// second authoring. The lane grammar's growth to actually CARRY this is the cli's, not built
/// here (the cli's record protocol is `site <key> effect=… rc=…` today).
#[must_use]
pub fn project_oob(diag: &Diag) -> OobProjection {
    OobProjection {
        site: diag.code.site(),
        code: diag.code.slug(),
        severity: registry(&diag.code).severity,
    }
}

/// The fact-plane fields a [`Diag`] projects to the OOB site-keyed lane ([`project_oob`]). A pure
/// projection — prose/help/receipts are NOT here (the OOB lane is fact-plane, `render-2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OobProjection {
    /// The lane key (shared with the diagnostic's site identity).
    pub site: SiteId,
    /// The stable wire slug (`22B-fork-wire-code`).
    pub code: &'static str,
    /// The registry severity.
    pub severity: Severity,
}

// ===========================================================================
// Legacy bridge — coexistence with crate::Diagnostic (the migration seam)
// ===========================================================================

impl Diag {
    /// Lower this structured [`Diag`] to a legacy [`crate::Diagnostic`] for the existing
    /// stderr/erasability/coverage plumbing (the round-22 coexistence seam — `22B` §1). The
    /// legacy `message` is the [`render_cli`] narrative (so `report()` surfaces the span — the
    /// drop-A fix); the legacy `severity`/`code`/`span` are the registry severity, the stable
    /// slug, and the primary span. This keeps the 96 untouched consumers working while the spine
    /// is proven; the B4 sweep migrates them and this bridge's callers shrink.
    ///
    /// The span is `Some(primary.span)` — ALWAYS, never `None` (the mandatory-primary-span fix,
    /// `21Z` drop-B). The erasability gate's `canon_diag` keys an Error-class diagnostic on
    /// `(code, span, severity)`; this lowering preserves all three, so a migrated Error-class
    /// code lands on the identity plane exactly as its legacy form did.
    ///
    /// The legacy `message` is `<primary label><body>` — NO title and NO region line (the cli's
    /// `report()` produces `<stage>: <sev>[<code>]: ` and renders the span itself, so embedding
    /// them here would double them). The body's continuation lines (` = note:`/` = help:`) ride
    /// after, exactly as the legacy multi-param messages did not — but gate-3 only keys on the
    /// `error[`-shaped first line, so the continuations are inert to it.
    #[must_use]
    pub fn to_legacy(&self, interner: &crate::Interner) -> crate::Diagnostic {
        let mut message = self.primary.label.clone().unwrap_or_default();
        message.push_str(&render_body(self, interner));
        crate::Diagnostic {
            severity: self.severity(),
            code: crate::DiagCode(self.code.slug()),
            span: Some(self.primary.span),
            message,
        }
    }
}

// ===========================================================================
// Small render helpers (pure, allocation-light)
// ===========================================================================

/// The severity word for the title line (matches the cli `report()` vocabulary so gate-3's
/// `<sev>[<code>]` floor keys identically).
fn severity_word(severity: Severity) -> &'static str {
    match severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    }
}

/// The `[remediation-class]` inline tag (`render-1`): a stable lowercase slug per class.
fn remediation_tag(class: RemediationClass) -> &'static str {
    match class {
        RemediationClass::AuthorOracle => "author-oracle",
        RemediationClass::AddDeclaration => "add-declaration",
        RemediationClass::FixBookLine => "fix-book-line",
        RemediationClass::Structural => "structural",
    }
}

/// Render a span as the simple no-caret region form `<lo>:<hi>` followed by the resolved source
/// excerpt when it is in range (the drop-A fix — the span reaches the user). Referent-agnostic:
/// the source text is shown for orientation, never decoded.
fn render_span(span: Span, src: &str) -> String {
    let lo = span.lo.0 as usize;
    let hi = span.hi.0 as usize;
    match src.get(lo..hi) {
        Some(text) => format!("{}:{} `{}`", span.lo.0, span.hi.0, text),
        None => format!("{}:{}", span.lo.0, span.hi.0),
    }
}

// ===========================================================================
// Legacy survivors — the not-yet-migrated catalog codes (B4 sweep target)
// ===========================================================================

/// The three give-up codes that lived in the round-21 `core::diag` catalog and are NOT migrated
/// this round (`22B` §1 coexistence): they stay on [`crate::Diagnostic`] until the B4 mechanical
/// sweep folds them onto the [`Diag`] spine and empties the gate-grep allow-list. Kept verbatim
/// (templates + structured-param constructors) so the analysis crate's emit sites are unchanged
/// while the spine is proven on the three `22B` §5 worked examples only.
///
/// (`render-heredoc-refused` is NOT here — it migrated; `dq-site-unresolvable` and
/// `dq-cmdsub-operand-top` are NOT here — they migrated. This module is exactly the residual.)
pub mod legacy {
    use crate::{DiagCode, Diagnostic, Span};

    /// A command runs inside a `$( … )` substitution body — effect-bearing but not independently
    /// elidable (`219` q-1.f silent-1/silent-4). NOT migrated this round.
    pub const CMDSUB_INNER_NONLEAF: DiagCode = DiagCode("dq-cmdsub-inner-nonleaf");

    /// A WRITE-shaped redirect (`>`/`>>`) to a DYNAMIC/unresolved target joins ⊤ (y-1, `21F`
    /// imp-1). NOT migrated this round.
    pub const REDIR_TARGET_TOP: DiagCode = DiagCode("dq-redir-target-top");

    /// A transitively-inlined (depth-2) call whose own call-argument references a positional that
    /// does not thread two inline levels (`216` §1.2 correction). NOT migrated this round.
    pub const DEPTH2_POSITIONAL_UNTHREADED: DiagCode = DiagCode("dq-depth-2-positional-unthreaded");

    /// The not-yet-migrated codes (the allow-list cross-check reads this to confirm the residual
    /// is exactly these three legacy catalog codes).
    pub const RESIDUAL_CATALOG: &[DiagCode] = &[
        CMDSUB_INNER_NONLEAF,
        REDIR_TARGET_TOP,
        DEPTH2_POSITIONAL_UNTHREADED,
    ];

    /// The message TEMPLATE for a residual code (the round-21 `rq-1` shape — phrasing lives here).
    #[must_use]
    pub fn template(code: DiagCode) -> &'static str {
        match code {
            CMDSUB_INNER_NONLEAF => {
                "command `{inner}` runs inside a `$(…)` substitution ⇒ effect-bearing but not \
                 independently elidable (it runs whenever its enclosing line runs)"
            }
            REDIR_TARGET_TOP => {
                "write-redirect to a dynamic/unresolved target ⇒ no per-path `file` cell can be \
                 keyed, so the write joins ⊤ and the command runs (never elided)"
            }
            DEPTH2_POSITIONAL_UNTHREADED => {
                "call `{name}` not inlined: its argument references a positional (`$1`..`$9`/`$#`) \
                 that does not thread through two inline levels ⇒ the inner body's positional is ⊤ \
                 — it runs as an ordinary unmodeled command (depth-2 positional threading is out of \
                 the modeled subset)"
            }
            _ => "",
        }
    }

    /// Build the [`CMDSUB_INNER_NONLEAF`] Note (`inner` = the inner command's resolved text).
    #[must_use]
    pub fn cmdsub_inner_nonleaf(span: Option<Span>, inner: &str) -> Diagnostic {
        Diagnostic::note(
            CMDSUB_INNER_NONLEAF,
            span,
            fill(template(CMDSUB_INNER_NONLEAF), &[("inner", inner)]),
        )
    }

    /// Build the [`REDIR_TARGET_TOP`] Note (parameterless; the offending word is ⊤, text
    /// unavailable; `span` carries the redirect's source location).
    #[must_use]
    pub fn redir_target_top(span: Option<Span>) -> Diagnostic {
        Diagnostic::note(
            REDIR_TARGET_TOP,
            span,
            template(REDIR_TARGET_TOP).to_owned(),
        )
    }

    /// Build the [`DEPTH2_POSITIONAL_UNTHREADED`] Note (`name` = the refused call's function name).
    #[must_use]
    pub fn depth2_positional_unthreaded(span: Option<Span>, name: &str) -> Diagnostic {
        Diagnostic::note(
            DEPTH2_POSITIONAL_UNTHREADED,
            span,
            fill(template(DEPTH2_POSITIONAL_UNTHREADED), &[("name", name)]),
        )
    }

    /// Substitute `{key}` placeholders from `params` — a deterministic, allocation-light fill (an
    /// unmatched placeholder is left verbatim so a mismatch is visible). `inv-determinism`.
    fn fill(template: &str, params: &[(&str, &str)]) -> String {
        let mut out = template.to_owned();
        for (key, value) in params {
            let needle = format!("{{{key}}}");
            out = out.replace(&needle, value);
        }
        out
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Every residual code resolves to a non-empty template (the round-21 `rq-2` gate, kept
        /// for the legacy survivors until B4 sweeps them).
        #[test]
        fn every_residual_code_has_a_nonempty_template() {
            for &code in RESIDUAL_CATALOG {
                assert!(
                    !template(code).is_empty(),
                    "residual code `{}` has no template",
                    code.0
                );
            }
        }

        /// The constructors fill their templates (a drift would leave a `{placeholder}`).
        #[test]
        fn residual_constructors_fill_templates() {
            let d = cmdsub_inner_nonleaf(None, "apt-get install -y nginx");
            assert!(d.message.contains("apt-get install -y nginx"));
            assert!(!d.message.contains('{'));
            let d = depth2_positional_unthreaded(None, "b");
            assert!(d.message.contains('b'));
            assert!(!d.message.contains('{'));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BytePos, Interner};

    fn span(lo: u32, hi: u32) -> Span {
        Span::new(BytePos(lo), BytePos(hi))
    }

    fn site(n: u32) -> SiteId {
        SiteId::leaf(LeafId(n))
    }

    /// The mandatory primary span is structural (`21Z` drop-A/drop-B): a `Diag` always has a
    /// span, and `to_legacy` always sets `Some(span)` — the legacy `None` that dropped the span
    /// is unrepresentable through this spine.
    #[test]
    fn primary_span_is_mandatory_and_reaches_legacy() {
        let d = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused { site: site(3) }),
            span(10, 20),
        );
        assert_eq!(d.primary.span.lo.0, 10);
        let i = Interner::default();
        let legacy = d.to_legacy(&i);
        assert_eq!(legacy.span, Some(span(10, 20)), "the span is never None");
    }

    /// Severity comes ONLY from the registry (`crib-4`): there is no severity constructor, and a
    /// Note-class and an Error-class code resolve their declared severities.
    #[test]
    fn severity_is_registry_data_not_a_constructor() {
        let note = Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: site(0),
                source_excerpt: OutClaim(Interner::default().intern("x")),
            }),
            span(0, 1),
        );
        assert_eq!(note.severity(), Severity::Note);
        let err = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused { site: site(0) }),
            span(0, 1),
        );
        assert_eq!(err.severity(), Severity::Error);
    }

    /// The PROPOSED floor column (`22B-fork-floor-membership`): the render-refusal pins
    /// [`Floor::WarnOrDeny`]; the disclosures are floorless.
    #[test]
    fn proposed_floor_column() {
        let refused = DiagCode::RenderHeredocRefused(RenderHeredocRefused { site: site(0) });
        assert_eq!(registry(&refused).floor, Floor::WarnOrDeny);
        let unresolvable = DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: site(0),
            source_excerpt: OutClaim(Interner::default().intern("x")),
        });
        assert_eq!(registry(&unresolvable).floor, Floor::None);
    }

    /// The gate-3 interaction, EVOLVED (conductor flag): the round-21 `core::diag` carried an
    /// `every_registered_code_is_note_severity` invariant protecting the e2e stderr error-floor
    /// (gate-3 keys on `error[…]`-shaped lines; an undeclared Error-class code fails a case). The
    /// spine's catalog is now a deliberate MIX — the two disclosures stay `Note` (they must never
    /// silently become Error and trip gate-3 undeclared), and the render-refusal is `Error` (it
    /// always was, as a legacy inline literal; the e2e `render21-heredoc-refusal` case already
    /// declares it in `expected-diagnostics`). This pins that split so a future severity drift on
    /// the Note-class codes is a deliberate change, not a silent gate-3 breach.
    #[test]
    fn gate3_floor_note_codes_stay_note_error_code_is_declared() {
        // The disclosures MUST stay Note (else they trip gate-3 on every ⊤/unresolvable case).
        for code in [
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: site(0),
                position: OperandPosition::CommandWord,
                cause: None,
            }),
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: site(0),
                source_excerpt: OutClaim(Interner::default().intern("x")),
            }),
        ] {
            assert_eq!(
                registry(&code).severity,
                Severity::Note,
                "a disclosure code must stay Note or it trips the gate-3 error-floor undeclared: {}",
                code.slug()
            );
        }
        // The one Error-class code (the render-refusal) — its presence is deliberate and
        // declared by the e2e case; pin that it is Error (a downgrade to Note would mask a
        // broken-artifact refusal).
        let refused = DiagCode::RenderHeredocRefused(RenderHeredocRefused { site: site(0) });
        assert_eq!(registry(&refused).severity, Severity::Error);
    }

    /// The render partition (`render-3`, the ru-12 weld): the artifact comment is fact-plane
    /// (returns `Some` only for the fact-relevant refusal, `None` for render-plane disclosures);
    /// the CLI render carries the full narrative including the remediation help.
    #[test]
    fn render_partition_artifact_is_fact_plane() {
        let i = Interner::default();
        // A Note disclosure: no artifact comment (render-plane only).
        let note = Diag::new(
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: site(2),
                position: OperandPosition::Operand(1),
                cause: None,
            }),
            span(0, 4),
        )
        .label("this `$(…)` is unresolvable (⊤)");
        assert_eq!(
            render_artifact_comment(&note),
            None,
            "a disclosure contributes no fact-plane artifact comment"
        );
        // The CLI render carries the label (drop-A: the span + label reach the user).
        let cli = render_cli(&note, "echo TAIL", &i);
        assert!(cli.contains("this `$(…)` is unresolvable"), "{cli}");
        assert!(cli.starts_with("note["), "title is severity-keyed: {cli}");
        // An Error refusal: a fact-plane comment naming the site, no prose.
        let refused = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused { site: site(7) }),
            span(0, 4),
        )
        .help("split the heredoc body to its own leaf");
        let comment = render_artifact_comment(&refused).expect("a refusal is artifact-relevant");
        assert!(comment.starts_with('#'), "a comment: {comment}");
        assert!(
            comment.contains("site 7"),
            "names the fact-plane site: {comment}"
        );
        assert!(
            !comment.contains("split the heredoc"),
            "the help (exempt-plane) must NOT reach the artifact: {comment}"
        );
    }

    /// The OOB projection (`render-2`) is fact-plane: site + slug + severity, no prose. The slug
    /// is the stable wire token (`22B-fork-wire-code`).
    #[test]
    fn oob_projection_is_fact_plane_slug_keyed() {
        let d = Diag::new(
            DiagCode::SiteUnresolvable(SiteUnresolvable {
                site: SiteId {
                    leaf: LeafId(4),
                    member: Some(2),
                },
                source_excerpt: OutClaim(Interner::default().intern("make install")),
            }),
            span(0, 12),
        );
        let p = project_oob(&d);
        assert_eq!(p.code, "dq-site-unresolvable", "stable wire slug");
        assert_eq!(p.site.leaf, LeafId(4));
        assert_eq!(p.site.member, Some(2));
        assert_eq!(p.severity, Severity::Note);
    }

    /// The builder chains the GOOD shape (`type-sketch-7`): a label, a secondary cause-span, a
    /// note, and a remediation-classed suggestion — all from `new` + chained calls, no DSL.
    #[test]
    fn builder_assembles_the_good_shape() {
        let d = Diag::new(
            DiagCode::CmdsubOperandTop(CmdsubOperandTop {
                site: site(0),
                position: OperandPosition::CommandWord,
                cause: None,
            }),
            span(0, 5),
        )
        .label("this `$(…)` is unresolvable (⊤)")
        .secondary(span(10, 20), "and so this command cannot be elided")
        .note("downstream commands run unconditionally")
        .suggest(Suggestion {
            message: "declare the kind's selector in its oracle".to_owned(),
            applicability: Applicability::MaybeIncorrect,
            remediation: RemediationClass::AuthorOracle,
        });
        assert_eq!(d.secondary.len(), 1);
        assert_eq!(d.children.len(), 1);
        assert!(d.suggestion.is_some());
        let cli = render_cli(&d, "01234_56789poisoned_", &Interner::default());
        // the secondary label and the remediation tag both render.
        assert!(cli.contains("cannot be elided"), "{cli}");
        assert!(cli.contains("[author-oracle]"), "{cli}");
    }

    /// `OperandPosition::describe` matches the legacy prose the migrated emit site produced (so
    /// the disclosure text is stable across the migration).
    #[test]
    fn operand_position_describe_matches_legacy_prose() {
        assert_eq!(OperandPosition::CommandWord.describe(), "the command word");
        assert_eq!(OperandPosition::Operand(2).describe(), "operand 2");
    }

    /// The grouping keys (`type-sketch-5`): fine keys on (slug, site); coarse STUBS to fine
    /// (`22B-fork-scope-key` — degenerate coarse=fine, no collapse yet).
    #[test]
    fn grouping_keys_fine_and_stubbed_coarse() {
        let d = Diag::new(
            DiagCode::RenderHeredocRefused(RenderHeredocRefused { site: site(5) }),
            span(0, 4),
        );
        let fine = d.fine_key();
        assert_eq!(fine.code, "render-heredoc-refused");
        assert_eq!(fine.site, site(5));
        // STUB: the coarse key wraps the fine key unchanged this round.
        assert_eq!(d.coarse_key().fine, fine);
    }
}
