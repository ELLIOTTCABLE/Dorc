//! `dorc-core` — the shared vocabulary every other spike crate agrees on *first*.
//!
//! Research chord `dac-B` (error/provenance synthesis, plans/111): the analyzer
//! and the error/diagnostic layer MUST agree the graph and result types *before*
//! either is built, or they grow two incompatible graphs. This crate is that
//! agreement.
//!
//! Two invariants are load-bearing and enforced here:
//!
//! * **Determinism.** No clock, RNG, filesystem, or network — directly or
//!   transitively. The analyzer kernel is a pure function of its inputs, which is
//!   what lets the whole pipeline run inside deterministic-simulation tests
//!   without dependency-injection ceremony. Keep it that way.
//! * **No-throw stages (`dn-7`).** Every pipeline stage yields a `Carrier<T>` — a
//!   *result paired with accumulated diagnostics* — and never panics on malformed
//!   input. Errors are data, not control flow. The carrier itself, and everything
//!   else on the describe plane, lives in `dorc-aid`, which deps this crate and is
//!   never depended upon by it.
//!
//! Identifiers are newtypes, never bare integers (`make illegal states
//! unrepresentable`): you cannot pass an [`AstId`] where the type wants a fact
//! token, and the compiler enforces it.

#![forbid(unsafe_code)]
// Seeded round-19 code predates the take-3 lint gate; these crate-root expects
// ratchet away during the rebuild (an unfulfilled `expect` warns, so they
// self-remove as the seeded layer is replaced). They never relax the policy for
// new crates — only this seeded substrate.
#![expect(
    missing_docs,
    clippy::indexing_slicing,
    reason = "seeded round-19 code predates the take-3 lint gate; ratchet away during the rebuild"
)]

use std::collections::HashMap;

// ===========================================================================
// Identifiers
// ===========================================================================

/// Index of a node in the parsed AST arena (crate `dorc-syntax`).
///
/// Other id spaces (CFG nodes, executable leaves, facts, kinds, providers) are
/// added to this crate as the phases that need them begin — demand-driven, like
/// the parser itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AstId(pub u32);

/// A stable identifier for one executable leaf (`dn-3`, the leaf-seam): executable
/// work is a list of *individually wrappable* leaves, each with a stable back-map to
/// its source — never one opaque `sh -c "$bigscript"`. The id is the leaf's position
/// in source order.
///
/// Lives in `core` (the `dac-B` shared vocabulary), not `plan`, because the round-22
/// structured diagnostic ([`SiteId`]) keys on it — a diagnostic's first-class
/// site identity must be expressible in the base crate every layer agrees on, the
/// same `dec-seam-ownership` move that pulled [`FactKey`] down here. `plan` re-exports
/// this type rather than holding a parallel one (`inv-site-keyed-results`: one shared
/// site-id, never two).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LeafId(pub u32);

/// A diagnostic's first-class site identity (`22B` `type-sketch-5`; promoted from the cli's
/// `RecordKey`). The `site N.M` keying (`member` = the in-loop fact-family index,
/// `inv-site-keyed-results`) is the FINE key; the COARSE key for fleet rollup is a slot
/// (`GroupingKey`, in the describe plane) the machinery does not yet fill
/// (`22B-fork-scope-key` = STUB coarse=fine).
///
/// Sited in `core` beside [`LeafId`] rather than in the describe plane: it is DECIDE-plane
/// identity (`inv-site-keyed-results`), the same `(leaf, member)` pair the cli's probe-records
/// and the apply plan's steps share.
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

/// Which loaded INPUT file a [`Span`] indexes into — book or oracle, one id space over the whole
/// unit (`28K` §2a Provenance; `27V:mech-minting-line-threading`; `tc-oracle-file-identity`). A
/// [`Span`] is a bare byte-range with no file identity, so a lift span (a vouch/decline arm, a
/// claim mark, a definition being pinned) is ambiguous the moment more than one file is loaded.
/// This is the index into the driver's ordered source list — the ONE disambiguator
/// `AID-NEEDS:law-lineno-identity` presupposed.
///
/// ONE space, deliberately: `28K` makes books first-class definition sources (an in-book role
/// function is an ordinary oracle, recognized by name alone), so "the book" stopped being the
/// single implicit file whose spans needed no id. Ordering is load order — CLI-named sources
/// first, in command-line order, then the book(s) — which is also the ambient-prefix order the
/// function environment reads, so an id comparison IS a load-order comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFileId(pub u32);

/// WHOSE utterance a license rests on (`28M` §8 — the monologue, typed).
///
/// A license is a MONOLOGUE when every load-bearing utterance behind it has one author, and a
/// DIALOGUE when the engine combines two authors' utterances into a composite sentence nobody
/// uttered and both may repudiate. `28M` §8 scouted the license plane and found it monologue
/// EVERYWHERE — but emergently, by three unrelated mechanisms agreeing (lane exclusivity, the
/// establish-⊤ firewall, consumed-⊤ forbids-mint), none of which NAMES custody. This type is that
/// name, so the property is held by the type system rather than by a coincidence that a later
/// widening could quietly break.
///
/// **Construction is deliberately a single seat** ([`of_defining_file`](Self::of_defining_file)),
/// and that is the load-bearing design constraint rather than tidiness. Custody is keyed to the
/// DEFINING FILE today, but `28M` §10 `dir-ownership-is-transitive-inclusion` (UNRULED, under live
/// human adjudication) may re-key it to the transitive sourcing-closure of an entry file. Because
/// every consumer compares custodies (`==`) and none reads the file id to decide anything, that
/// re-key is a change to this type's internals and nothing else. **Never key a NEW decision off a
/// raw [`SourceFileId`]** — reach for this type, or the re-key becomes a workspace-wide edit and
/// the fence stops being movable.
///
/// [`defining_file`](Self::defining_file) exists for PROVENANCE AND DISPLAY only — a diagnostic
/// naming which file spoke. Branching on it re-creates exactly the untyped keying this exists to
/// retire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefinitionCustody(SourceFileId);

impl DefinitionCustody {
    /// The ONE constructor: custody belongs to the file that DEFINED the utterance.
    #[must_use]
    pub const fn of_defining_file(file: SourceFileId) -> Self {
        Self(file)
    }

    /// The defining file, for provenance and display ONLY (see the type doc — never a decision).
    #[must_use]
    pub const fn defining_file(self) -> SourceFileId {
        self.0
    }
}

/// Whose utterance a replacement license rests on (`28M` §8's monologue, made a type).
///
/// The variants are the only two ways a replacement is currently single-author, and naming them
/// apart is the whole mechanism: a widening that reproduced a value measured by a DIFFERENT
/// author's `predict()` under this author's license fits NEITHER, so it cannot be written without
/// adding a variant here — a visible, reviewable act in the one file that defines what custody
/// means, instead of a quiet edit at a mint site. That is the "re-entry becomes a type error"
/// property `28M` §8 asked for, sited where it is cheap to keep.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseCustody {
    /// A converged-establish elision: it rests on ONE author's reached verdict vouch, and this is
    /// that author. The agreement gate already proved this definition is the one live at the site,
    /// so the custody here and the custody the gate compared are the same value by construction.
    Vouched(DefinitionCustody),
    /// An AGGREGATE erasure (a member-loop, an inlined call): several establishes vanish under one
    /// license, and each carries its OWN author's reached vouch, cardinality- and identity-matched
    /// (`rul-every-erased-establish-is-vouched`). So this is a CONJUNCTION of monologues — each
    /// author independently licensed their own line — and not a composite sentence, which is why it
    /// is admissible. It is named apart from [`Vouched`](Self::Vouched) anyway, because the day
    /// anything reads ACROSS these establishes rather than merely conjoining them, that read IS a
    /// dialogue and this variant is where it will be sitting. The per-establish custodies ride the
    /// aggregate's own vouch receipts.
    VouchedSeverally,
    /// A read-only Query-guard substitution: it rests on no authored vouch at all. Its reproduced
    /// value is the probe's own measurement OF THE VERY COMMAND being substituted, so there is no
    /// second utterance to combine with and no author to attribute it to but the site itself
    /// (`28M` §8's `QueryGuard` cell — measured-never-asserted, probe-provenance-only).
    ///
    /// Reproduction that reaches BEYOND that cell must never reuse this variant: the moment a
    /// reproduced value comes from somewhere other than the substituted command's own measurement,
    /// "self" is a lie and the composite needs a custody that names both speakers.
    MeasuredSelf,
}

// ===========================================================================
// Source positions
// ===========================================================================

/// A byte offset into a single source script. Byte- (not char-) indexed: the
/// lexer works over bytes, and POSIX sh is effectively byte-oriented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(pub u32);

/// A half-open `[lo, hi)` byte range in one source script.
///
/// Kept as a compact pair (research chord `ch-handle`): the hot analysis path
/// carries spans, never source text; text is resolved lazily for reporting.
///
/// Ordered lexicographically on `(lo, hi)` — earlier start first, then shorter first — so a span
/// can ride inside a map key (`inv-determinism-here`). [`DefinitionId`] is why: a definition's
/// identity is its file plus its span, and two definitions of one name in one file are told apart
/// by nothing else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl Span {
    #[must_use]
    pub fn new(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }

    /// The covering span of `self` and `other` (smallest range containing both).
    #[must_use]
    pub fn to(self, other: Span) -> Span {
        Span {
            lo: BytePos(self.lo.0.min(other.lo.0)),
            hi: BytePos(self.hi.0.max(other.hi.0)),
        }
    }
}

pub mod prov;
pub use prov::{
    JOIN_PARENT_CAP, OriginKind, OriginNode, Parents, ProbeStamp, ProvArena, ProvId, RunInstant,
    Variation, Witness,
};

pub mod sorted;
pub use sorted::{SortedMap, SortedSet};

pub mod unord;
pub use unord::IterSuppressedMap;

pub mod claim;
pub use claim::{
    ByObservation, BySilence, ByVouch, Claim, ObservationTier, Rung, SilenceTier, Tier,
    VouchAndRung, VouchTier,
};

pub mod influence;
pub use influence::{
    AuthoredBeforeContact, HostInfluenced, HostReported, InfluencePhase, Influenced,
};

pub mod coord;
pub use coord::{
    Context, ContextKey, Coord, Dialect, EntityResolution, Relation, compare, selector_covers,
    selector_identifies,
};

pub mod room;
pub use room::{HintOnly, Invited, Room, RoomFact, RoomTag};

pub mod escalation;
pub use escalation::{Capability, EscalationDial};

pub mod contested;
pub use contested::ContestedFamilies;

pub mod definition;
pub use definition::{DefinitionId, LiveDefinition, answering_row};

pub mod spine;
pub use spine::{CensusArm, DecidePlane, Spine, SpineSpecies};

pub mod custody;
pub use custody::{CustodyClosures, custody_reaches};

pub mod loadpath;

pub mod region;
pub use region::{ElisionRegion, IterationSlot, RegionUniverse};

// ===========================================================================
// String interning + the referent-agnostic opaque token (dn-4, W4)
// ===========================================================================

/// An interned string handle. Cheap to copy and compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(u32);

impl Symbol {
    /// The interning-order index — a stable scalar identity for serialization/canonicalization
    /// (the erasability digest renders a `FactKey` by its symbols' ids). Referent-agnostic
    /// (`inv-referent-agnostic`): an identity, never decoded text. Stable within one run's
    /// [`Interner`] (order-of-interning), which is all the intra-run digest needs.
    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

/// Interns strings to [`Symbol`]s. Deterministic: equal input → equal symbol,
/// and symbol assignment is order-of-interning (never hashed/random).
#[derive(Debug, Default)]
pub struct Interner {
    strings: Vec<Box<str>>,
    lookup: HashMap<Box<str>, Symbol>,
}

impl Interner {
    pub fn intern(&mut self, text: &str) -> Symbol {
        if let Some(&sym) = self.lookup.get(text) {
            return sym;
        }
        let sym = Symbol(u32::try_from(self.strings.len()).unwrap_or(u32::MAX));
        let boxed: Box<str> = text.into();
        self.strings.push(boxed.clone());
        self.lookup.insert(boxed, sym);
        sym
    }

    /// Resolve a symbol minted by *this* interner back to its text.
    #[must_use]
    pub fn resolve(&self, sym: Symbol) -> &str {
        &self.strings[sym.0 as usize]
    }
}

/// An opaque state-entity token (research wall `W4`, chord `referent-agnostic`):
/// the analyzer keeps relational contracts over symbols it is forbidden to
/// *understand*. You may compare two `OpaqueToken`s for equality (intra-script
/// co-reference) and resolve one for display/provenance — but you must NEVER
/// branch on its decoded text to infer meaning (that what-is-`nginx` job belongs
/// to the oracle, not the engine). Cross-oracle identity binds to a named kind,
/// never to a shared token (chord `cross-oracle-named-kind`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpaqueToken(pub Symbol);

/// A named, oracle-declared *kind* (`package`, `service`, …) — the anchor for
/// cross-oracle identity (wall `W4`, the dn-1 hinge). Like [`OpaqueToken`], the
/// name is NEVER decoded for meaning; two oracles declaring the same kind name
/// are coherent providers of one kind (chord `cross-oracle-named-kind`). The
/// Tier-A blessed forms use well-known kind names (`file`, `tool`, `freshness`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KindId(pub Symbol);

/// An oracle *provider* (`apt-get`, `dpkg`, …) — the `(provider, verb)` key of
/// the fact-centric effect map (note 162). An interned name, never decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProviderId(pub Symbol);

// ===========================================================================
// Analysis vocabulary: phase, verdict, grade, fact-domain, fact
// ===========================================================================

/// Which pass we are in. The two soundnesses are *phase-keyed*, with opposite
/// fail-directions (welded knob `kFAIL`, chords `two-soundnesses`/`phase-flip`):
/// the probe pass fails toward "don't touch it", the apply pass toward "don't
/// skip it". A shortcut is only legal if it fails the conservative way for its
/// phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    /// Read-only probe projection — never mutates (`kFAIL-withhold`).
    Probe,
    /// Mutating apply — never skips a needed mutation (`kFAIL-perform`).
    Apply,
}

/// Three-valued convergence verdict (chord `ch-verdict`: ok/fail/unknown, kept
/// distinct from the diagnostic stream). `Unknown` is first-class and folds
/// conservatively — an unreachable host or an un-probeable fact is `Unknown`,
/// never silently `Converged` (that would be a `kFAIL-perform` violation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Desired state already established → the mutation may be skipped.
    Converged,
    /// Desired state not established → the mutation must run.
    Diverged,
    /// Cannot tell → must act conservatively for the phase.
    Unknown,
}

/// A concrete observed **exit status** (`19A §5`, `an-probe-shape`/`DP-3`): the
/// *value* a leaf's command yields, held opaquely. The apply abstract-interpreter
/// folds `&&`/`||`/`if`/`!` over this value (`9 || cmd` ⇒ `cmd` runs, by the shell's
/// own semantics) and the substitution reproduces it exactly. **rc is opaque to
/// Dorc** (`inv-referent-agnostic`-adjacent): we hold `9`, never interpret what `9`
/// *means* — the author already encoded the meaning by choosing `!`/`&&`/`||`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rc(pub i32);

/// The **byte-content** of the `Stdout`/`Stderr` channels (`inv-one-observable`, `19F` §3 tuple
/// completion): the captured text a substitution would have to reproduce. Named for what it IS —
/// channel CONTENT, not a claim: "claim" is the derived-license tier (`is_converged` CLAIMS
/// convergence), and these bytes carry no license, so the former `OutClaim` name clashed tier-wise
/// (`275` care-outclaim-rename; `271:rul-value-prediction-species` — captured bytes are a
/// value-PREDICTION, whose provenance/backing are DERIVED, never a claim carried on the content).
/// An interned [`Symbol`] (the cheapest deterministic `Copy` representation — keeps
/// [`Observable`] `Copy`, and the interner is order-of-interning so it never leaks
/// nondeterminism, `inv-determinism`). The engine NEVER decodes it (`inv-referent-agnostic`):
/// a substitution compares/reproduces the content, the analyzer does not branch on its text.
///
/// NOTHING produces a non-⊤ `OutBytes` in the kernel this round (the existing consumed-stdout/
/// stderr gate stays the unconditional block it is — a consumed channel with a ⊤ prediction
/// blocks, today's rule). The newtype exists so a future stdout-producing probe is a
/// value-plumbing change, not a representation change (the `19F` failure was exactly
/// representation drift).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OutBytes(pub Symbol);

/// Why a value resolved to `⊤` — the cause-named diagnostics of `219` q-2 become real error-lane
/// content (`270:block-rebuild` value-recipe-reshape: "every ⊤ names its CAUSE"). A ⊤ is a
/// correctness boundary (`inv-top-reject`), never a silent best-effort; naming its cause lets the
/// hint/why-lane say WHAT a `$(…)`/positional/dynamic value blocked. Held on [the value plane's
/// `ValueOf::Top`](../../dorc_analysis/value/enum.ValueOf.html) and carried per-fragment on the
/// recipe (so a mixed word records which fragment forced the ⊤).
///
/// `Copy` + no payload — a cause is a coarse category, not a span (the span is the node's). Two ⊤s
/// with different causes are UNEQUAL; no consumer relies on `⊤ == ⊤` across causes (`ValueOf` is
/// pattern-matched `Top(_)`, never compared for cross-cause equality — see the value-plane
/// consumers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopCause {
    /// An unmodeled expansion collapsed the word: a `$(…)` command-substitution, `$(( ))`
    /// arithmetic, or a `${x:-y}`/`${#x}` operator form (`219` q-1.b — the formerly-silent ⊤
    /// path). The `$()` capture lane un-⊤s the subst case at block-context; until then it walls.
    UnmodeledExpansion,
    /// A positional/special parameter with no static binding: bare `$@`/`$*`/`"$*"`, an unbound
    /// `$N`, or a `$N` past the call's operands / bound to a ⊤ operand
    /// (`rider-positional-modeling-hardening`; `24C:fd-headline-oneliner-gap`).
    UnresolvablePositional,
    /// A non-positional special parameter read as a value: `$?`/`$$`/`$!`/`$-`. Dynamic state,
    /// never statically fixed. (An `rv=$?` STORED into the dataflow is a value-prediction — the
    /// `care-site-vs-stored` boundary; a bare `$?` word here is just ⊤.)
    DynamicParameter,
    /// A tracked variable that resolved to `⊤` at a use site — unset (`unset`-is-⊤), or the
    /// lattice join of two disagreeing branches. The value is runtime-dynamic.
    DynamicValue,
    /// An unquoted expansion that field-splits or globs unmodelably: a split under a non-pristine
    /// `IFS`, or a glob char in a resulting field (`209` brk-3 / `20O`).
    SplitOrGlob,
    /// The value-flow worklist did not converge ⇒ the whole result folds to ⊤ (`16P` DP-9): a
    /// capped solve is an under-approximation we must not trust (`inv-probe-sourced-values`).
    NonConvergent,
    /// A read walled by an unmodeled/unvouched context — a captured value whose producing read is
    /// ⊤ (`silence-licenses-nothing`). RESERVED: no producer at this stage (captures are ⊤; the
    /// value plane runs before the probe — `seam-pipeline-order`), named now so the slot exists.
    WalledRead,
}

/// The provenance grade of a value-prediction (`275` §2 · `271:rul-value-prediction-species`): a
/// taint-style **weakest-fragment** grade over a value's recipe. DERIVED, never declared — the
/// authored surface for value-predictions is THE EMPTY SET (`value-predictions`); this is a
/// shape-division the engine computes, never a claim an author writes.
///
/// The four value-prediction grades (`275` §2) plus [`ProgramText`](ValueGrade::ProgramText) (a
/// value that is NOT a prediction — pure program text, the seam-literal-provenance distinction).
/// The order below is weakest→strongest; [`weakest`](ValueGrade::weakest) is the meet the
/// derivation folds over fragments ("a value concatenated from a delegation read and a composed
/// decoration grades as composed" — `275` §2). NOTHING consumes the grade at this stage
/// (`270:block-rebuild`: represent + derive, do NOT consume — no validity table); the composed
/// gate stays DEFERRED (`271:rul-composed-bytes-defer-and-floor` — represent the grade, gate
/// nothing).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueGrade {
    /// `⊤`: unknown provenance — a ⊤ fragment interposes. The weakest.
    Top,
    /// **author-composed**: printf-produced — the author asserted it (a composed
    /// output-prediction). Weaker than world-spoken (the delegation actually ran); the composed
    /// gate is DEFERRED (`rul-composed-bytes-defer-and-floor`).
    AuthorComposed,
    /// **world-spoken**: delegation-produced — the tool itself spoke at probe time (a captured
    /// `$(getent …)`). The knife-tier "real bytes" (`rul-composed-bytes-defer-and-floor`'s
    /// world-spoken floor).
    WorldSpoken,
    /// **register**: the analyzer's own context register — a lend-mapped user value / who-am-I,
    /// certain-by-construction, resolved analytically with no staleness (`275` §3 register regime).
    Register,
    /// NOT a value-prediction: pure program text (a source literal, or a value derived only from
    /// source text). The strongest — it never weakens a concatenation. A value graded
    /// `ProgramText` is program text, not a prediction; the four grades above are the
    /// value-prediction species (`275` §1 — "byte-shaped beliefs BEYOND program text").
    ProgramText,
}

impl ValueGrade {
    /// The **weakest-fragment** meet (`275` §2): the taint join a value's recipe folds over its
    /// fragments — the minimum grade, since a value is only as trustworthy as its weakest part.
    /// `ProgramText` is the identity (never weakens); `Top` is the absorbing bottom.
    #[must_use]
    pub fn weakest(self, other: Self) -> Self {
        self.min(other)
    }
}

/// A predicted value for one observable channel (`inv-one-observable`): a concrete
/// value, or a loud out-of-band ⊤ "can't-predict". A `Top` on a *consumed* channel
/// forces the consuming leaf to run (`inv-kfail`/`kFAIL-perform`): the check could not
/// predict the value a downstream context reads, so no stand-in can reproduce it. (The
/// fold's former `AbstractRc` was this type by another name — `Known`/`Top`.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Predicted<T> {
    /// The check predicts this exact value (an oracle-declared converged-rc, …).
    Value(T),
    /// ⊤: the check cannot predict this channel ⇒ no fold / no substitution through it.
    Top,
}

/// The channels of a command's observable output — the single shared vocabulary of the
/// ONE Observable (`inv-one-observable`). A **closed** enum: adding a channel must break
/// every exhaustive `match` (the compiler-as-checklist), so it carries NO
/// `#[non_exhaustive]`. Replaces the former `analysis::cfg::Observable` consumption enum,
/// unifying it with `Verdict`/`Observed` — the round-19 three-way split (`19F`).
///
/// Two views key off this one vocabulary: an [`Observable`] *predicts* a value per
/// value-bearing channel; an enclosing context *consumes* a `Powerset<Channel>` (the
/// liveness set). The `Effect` channel is vouched by convergence (the forward gate), so
/// it never enters the *consumed* set — it gates the elision license instead.
///
/// The status consumers split by **what reproduces the read**, not construct identity
/// (`206` §3, executed in task-O; refined by arch-1, note 214). The leaf-exact (span-based)
/// apply render (arch-1) substitutes a leaf's exact byte-span in-situ, so the round-21
/// render-EXPRESSIBILITY floor (`StatusRenderFloor` — "the line-granular render cannot
/// substitute a guard sharing its line with `if`/`then`/`fi`") is GONE: an `if`/`elif` guard
/// is now an ordinary `StatusRelaxable` reader (a probe-sourced KNOWN rc reproduces the read
/// exactly; ⊤ blocks). What remains keyed on a REAL reason, not render capability:
/// `StatusRelaxable` (a KNOWN rc reproduces the consumer's decision — `&&`/`||` operands,
/// errexit-region commands, `$?`-readers' predecessors, and now if/elif guards),
/// `StatusInvariant` (the consumer decides nothing observable — the `cmd || true` shape),
/// and `StatusIterated` (the consumed value is a per-iteration SEQUENCE no single predicted
/// rc can reproduce — a `while`/`until` condition). The `AndOrStatus` (round-19) and
/// `StatusRenderFloor` (round-20/21) names are both retired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Channel {
    /// The command's effect on managed state (mutation). Vouched by convergence ⇒ never
    /// in the consumed set; its predicted value is the [`Observable::effect`] verdict.
    Effect,
    /// Status consumed by a value-relaxable reader — `&&`/`||` operands, errexit-region
    /// commands, `$?`-readers' predecessors, and (since arch-1, note 214) `if`/`elif`
    /// **guards** (`206` §3 + the leaf-exact render). A ⊤ rc blocks; a probe-sourced/known
    /// rc substitutes (the value-preserving stand-in reproduces the exact status, so the
    /// branch decides identically). Eliding a ⊤-rc operand to a fabricated rc-0 `true` would
    /// suppress a `|| fallback` — the `kFAIL-perform` under-execute (`19D`). The if/elif
    /// guard joined this channel when the leaf-exact render retired the `StatusRenderFloor`
    /// expressibility block: a guard's command byte-span is now substitutable in-situ
    /// (`if (exit 1); then` is dash-clean), so the only remaining question is the value one
    /// this channel already asks — does a KNOWN rc reproduce the guard's branch decision?
    StatusRelaxable,
    /// Status consumed-in-form but dead-in-fact — the `cmd || true` shape (door-3, charter
    /// `20V` §4). The `||` *reads* the left rc, yet both continuations rejoin with identical
    /// observables: cmd rc=0 ⇒ short-circuit, list rc 0; cmd rc≠0 ⇒ `true` runs (no
    /// observable) ⇒ list rc 0; `$?` after the list is 0 on both paths, and `set -e` sees 0
    /// on both paths (the left of `||` is errexit-exempt). So a ⊤ prediction is harmless and
    /// ANY stand-in rc is extensionally faithful — this channel NEVER blocks a license,
    /// regardless of prediction (⊤ included). It is the admin's own spelled-in-sh "this rc
    /// is not load-bearing" declaration. Distinct from [`StatusRelaxable`] (which a ⊤ rc
    /// blocks): there the readers' DECISIONS differ by rc, so a fabricated rc-0 would
    /// under-execute; here the decisions converge. Still RECORDED in the consumed set —
    /// disclosure/provenance must see the read; only the *blocking* judgment differs.
    StatusInvariant,
    /// Status consumed **per-iteration** by a `while`/`until` **condition** (arch-1, note
    /// 214 — the honest successor to the retired `StatusRenderFloor` block for loop
    /// conditions). The condition is re-evaluated every iteration, so the value it consumes
    /// is a SEQUENCE of rc's (one per pass), not a single value — and a substitution emits
    /// ONE predicted rc, which can never reproduce a sequence. Worse, a `while CMD` whose
    /// condition is replaced by a *constant* `true` is an **infinite loop** (the
    /// disaster-class shape), and a constant `false` runs the body zero times: either way
    /// the iteration count is wrong. So this channel **blocks unconditionally**, even with a
    /// known rc — keyed on the REAL reason (iteration), not on render capability (which the
    /// leaf-exact render removed). Distinct from [`StatusRelaxable`] (a single-shot guard a
    /// known rc reproduces) precisely because the loop condition is multi-shot. NB the
    /// in-loop structural floor (`Cfg::in_loop_body`) ALSO forces a loop-condition leaf to
    /// run this round (defense in depth); this mark stands independently so the block is
    /// honest about *why* even if that floor later lifts.
    StatusIterated,
    /// fd 1 captured to a real (non-`/dev/null`) sink ⇒ value-bearing, vouched by
    /// nothing ⇒ a consumed `Stdout` always blocks (16F §3).
    Stdout,
    /// fd 2 captured to a real sink — as `Stdout`.
    Stderr,
}

/// The ONE Observable (`inv-one-observable`): a command's predicted output over
/// [`Channel`]s. Replaces the round-19 three-way split — the `analysis::cfg::Observable`
/// consumption enum, the standalone `core::Verdict`, and the bolted `Observed{verdict,
/// rc}` (`19F`). The oracle `.predict()` PREDICTS it; an enclosing context CONSUMES some
/// channels; a substitution REPRODUCES the consumed channels' predicted values, and is
/// licensed only when the `Effect` channel predicts no-mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observable {
    /// `Effect` channel: the host-reported convergence [`Verdict`], refined to
    /// no-mutation by the ambient gate downstream. "Convergence is the *derived* state
    /// of the Effect channel" (`19F` §3) — `Verdict` is its value, no longer a separate
    /// probe-reported concept.
    pub effect: Verdict,
    /// The predicted exit **status** when converged — the oracle's declared converged-rc.
    /// `Predicted::Top` ⇒ undeclared ⇒ no fold through this leaf (the `19D` `kFAIL-perform`
    /// floor: never fabricate a conforming rc-0). The consuming side decides which status
    /// channel reads it: a [`Channel::StatusRelaxable`] reader (now including an if/elif
    /// guard, arch-1) folds/substitutes a known value; a [`Channel::StatusIterated`]
    /// `while`/`until` condition blocks regardless (the per-iteration sequence no single rc
    /// reproduces).
    pub status: Predicted<Rc>,
    /// `Stdout` channel: the predicted fd-1 [`OutBytes`] a substitution must reproduce.
    /// ALWAYS `Predicted::Top` this round (`19F` §3 shape completion — nothing produces a
    /// value yet): a *consumed* `Stdout` with a ⊤ prediction blocks the license
    /// unconditionally, which is exactly today's rule (`consumption_ok`, 16F §3), now
    /// expressed through the tuple rather than a side-channel.
    pub stdout: Predicted<OutBytes>,
    /// `Stderr` channel: the predicted fd-2 [`OutBytes`] — as [`stdout`](Self::stdout).
    pub stderr: Predicted<OutBytes>,
}

impl Observable {
    /// An observable carrying only the convergence verdict, with an **unpredicted**
    /// status and unpredicted stdout/stderr (all `Predicted::Top` ⇒ ⊤ for the fold). The
    /// conservative shape: convergence still drives elision, but no branch folds through
    /// this leaf's status, and a consumed stdout/stderr blocks (16F §3).
    #[must_use]
    pub fn verdict_only(effect: Verdict) -> Self {
        Self {
            effect,
            status: Predicted::Top,
            stdout: Predicted::Top,
            stderr: Predicted::Top,
        }
    }
}

/// Belief grade (Engler MUST/MAY, chord `must-may`) — the sound/unsound line.
/// Only a `Must` fact may license a skip; `May` (mined/distributional) is a hint
/// that bootstraps the oracle library and never authorizes elision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    Must,
    May,
}

/// One independently-mutation-gating facet of a kind's ≥enum state-model
/// (`17N inc-S` / `an-per-entity-selector`). An interned name; never decoded
/// (`inv-referent-agnostic`) — compared for co-reference, resolved for display.
///
/// The selector is what splits a flat per-(kind,entity) bit into independent
/// cells: `service@enabled` and `service@active` are *separately* mutation-gating
/// (`systemctl enable --now` writes both; an `is-active` probe must not discharge
/// an unmet `@enabled`), which a flat key could not hold (`notes/193` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectorId(pub Symbol);

/// The cell a fact is about: an operand-named cell, or the kind's implicit
/// singleton (`notes/193` §3; `an-host-identity-fact`-adjacent).
///
/// `apt-get update` is a nullary mutator on the one package index — no operand —
/// so the key must carry [`Singleton`](EntityRef::Singleton), not require an
/// [`OpaqueToken`]. The old flat key required a token, so a no-operand mutator
/// fell through to `Opaque ⇒ Reach::Top ⇒ the poison wall`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityRef {
    /// A cell named by a literal operand (`package:nginx`). Two operand tokens
    /// denote the same cell iff they compare equal (`an-entity-coref`).
    Operand(OpaqueToken),
    /// The kind's implicit single cell (`package-index`, the one apt index).
    Singleton,
}

/// A system-state fact-key, re-keyed for spike-2 (`notes/193` §3 / charter §3 /
/// `16Q §1`). The flat `(kind, entity)` pair gains a [`selector`](FactKey::selector)
/// — the cell coordinate the whole engine reaches over.
///
/// `dec-seam-ownership` (closed → `core`): the structured entity-algebra is the
/// shared vocabulary every crate agrees on first (`dac-B`), so it is *defined here*
/// and `analysis::effect::FactKey` re-exports this type rather than holding a
/// parallel key. Carries NO source span (provenance is the node's). Two keys are
/// equal iff `kind` + `entity` + `selector` all match.
///
/// `Copy`/`Ord`/`Hash` are preserved: `Reach`'s `BTreeSet<FactKey>` needs `Ord`,
/// and [`EntityRef`]/[`SelectorId`]/[`Context`] are themselves `Copy`+`Ord`, so the bound holds.
/// `inv-determinism`: any map/set keyed on `FactKey` stays `BTree*`, never
/// hashed-into-output.
///
/// # The context slot (`27C` §3 / `27L` `tc-context-slot-on-coord-not-factkey`, resolved here)
///
/// The fourth field is the wrapper-denoted world a fact is born in ([`Context`]). It defaults to
/// [`Context::HostDefault`] — every unwrapped fact, which is EVERY fact in the wrapper-free corpus,
/// so keying is byte-identical to the three-place key (`empty-world-byte-identical`: the field is a
/// constant across a wrapper-free run, so it partitions nothing). A wrapped site's measurement is
/// born [`Context::Wrapped`], keyed by the composed-shift [`ContextKey`]: two same-cell facts in
/// different contexts are now UNEQUAL (they can never collide in a `BTreeSet<FactKey>`), and the
/// context reaches `compare` via [`Coord::of_fact`] so a cross-context pair answers
/// [`Relation::Unknown`] — transport-by-collision is unrepresentable (`never-derive-separation`).
/// Two keys are equal iff `kind` + `entity` + `selector` + `context` all match. The `context`
/// field is FIRST-in-`Ord` after the three cell axes (declared last) so context-distinct facts
/// still sort by cell — the render/census order is unperturbed for HostDefault-only runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactKey {
    pub kind: KindId,
    pub entity: EntityRef,
    pub selector: SelectorId,
    /// The wrapper-denoted world this fact is born in (`27C` §3). `HostDefault` for every unwrapped
    /// fact; [`Context::Wrapped`] for an in-context measurement. Construct the common case with
    /// [`FactKey::cell`] (defaults `HostDefault`); a wrapped fact re-keys via [`FactKey::in_context`].
    pub context: Context,
}

impl FactKey {
    /// The common constructor — an ambient (`HostDefault`) fact cell. Every pre-`27C` construction
    /// site is exactly this (the context slot defaults, so the migration is mechanical and
    /// rung-0 byte-stable).
    #[must_use]
    pub const fn cell(kind: KindId, entity: EntityRef, selector: SelectorId) -> Self {
        Self {
            kind,
            entity,
            selector,
            context: Context::HostDefault,
        }
    }

    /// Re-key this fact into a wrapper-denoted world (`27C` §3): same cell, a distinct
    /// [`Context::Wrapped`] key. A `HostDefault` fact and its `in_context` re-key are UNEQUAL — the
    /// no-collision guarantee, pinned by [`tests::wrapped_and_ambient_same_cell_never_collide`].
    #[must_use]
    pub const fn in_context(self, context: Context) -> Self {
        Self { context, ..self }
    }
}

/// The survival-backing provenance of an ESTABLISH fact (`277` §5 backing-SETS): the minting
/// FAMILY of the fact (threaded from the lift — the `(provider, verb)` that classified the site),
/// plus the observe-backing-widening SELECTORS (`277` §5 observe-backing-widening / `271`). Built
/// by `analysis::effect` where the fact and its `(provider, verb)` are both in scope, and consumed
/// by `plan::survival` to build the fact's [`Backing`](../../dorc_plan) SET.
///
/// # Why threaded, not reverse-looked-up (`27D` disposition-backing-family-recovery)
///
/// Stage-3 recovered a backing's minting family via a dialect reverse-lookup (`sole_family(kind,
/// selector)`), which falls to `None` (the safe collide floor) when TWO families mint the same
/// `(kind, selector)` — `fence-divergent-meaning`. Threading the TRUE establishing family from the
/// lift is exact: the family the site's `(provider, verb)` names. Both members and the fact's own
/// coordinate carry it (all minted by ONE provider's predict body — its verdict AND observe marks).
///
/// # `family: Option` — the collision floor
///
/// `None` when TWO sites establish the SAME fact via DIFFERENT providers (a collision merged toward
/// the safe floor: `None` ⇒ the empty dialect ⇒ no sparing ⇒ collide, exactly as the reverse-lookup
/// would answer an ambiguous `(kind, selector)`). One establishing provider ⇒ `Some(that provider)`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FactBacking {
    /// The fact's minting family (the establishing `(provider, verb)`'s provider), or `None` on a
    /// cross-provider establish collision (the safe floor — no sparing).
    pub family: Option<ProviderId>,
    /// The observe-backing-widening sibling SELECTORS (`277` §5): the `:?` observe cells that
    /// co-occurred with the verdict in the establishing predict body. Each widens the fact's
    /// backing with a sibling cell `(fact.kind, fact.entity, selector)` — kill-surface only grows.
    pub observed: std::collections::BTreeSet<SelectorId>,
}

/// The reserved-namespace prefix for the **typeless-floor auto-cell** (`24L` §2/§7). A
/// verdict-bearing provider with no marked effect gets a synthetic establish-cell keyed at
/// `<AUTO_KIND_PREFIX><provider>`, so its own-line elision/guard tier lights up (the whole
/// point of the floor) without an authored coordinate. The `:` is what makes the namespace
/// **unnameable** (`fence-unnameable`): an authored kind is DNS-labels-before-the-first-colon
/// (`277` §4b), so it can never contain a `:` — no oracle can mint into or reference this
/// namespace. `inv-referent-agnostic`: the engine never decodes this text for meaning; the
/// prefix is an engine-reserved sentinel, checked only by [`is_auto_kind`].
pub const AUTO_KIND_PREFIX: &str = "dorc-auto:";

/// The auto-cell's fixed selector (`24L` §2 "property: opaque/auto"). One reserved token; the
/// cell is a per-provider singleton, so the selector never discriminates — it exists only to
/// fill the flat coordinate's third slot and render (`dorc-auto:foobar@converged`).
pub const AUTO_SELECTOR: &str = "converged";

/// Mint the typeless-floor **auto-cell** for `provider` (`24L` §2/§3 — THE floor coordinate).
/// The SOLE constructor: kind = the reserved per-provider namespace (`fence-unnameable`),
/// entity = [`EntityRef::Singleton`] ALWAYS (`fence-no-entity` — a typeless oracle has no bind,
/// so no operand is ever promoted to a referent; §3's caught referential-abstraction break),
/// selector = the fixed [`AUTO_SELECTOR`]. All of one command's sites share this one cell (the
/// singleton coarseness §3 prices; more same-cell staleness ⇒ more forced runs, never fewer).
#[must_use]
pub fn auto_fact(interner: &mut Interner, provider: &str) -> FactKey {
    let kind = KindId(interner.intern(&format!("{AUTO_KIND_PREFIX}{provider}")));
    FactKey::cell(
        kind,
        EntityRef::Singleton,
        SelectorId(interner.intern(AUTO_SELECTOR)),
    )
}

/// Is `kind` an auto-cell kind (`24L` §7 `fence-unnameable`)? The load-bearing predicate for the
/// two privacy fences the engine enforces structurally: the probe lane ships the VERDICT body
/// for these (not a `predict`, `compile_probe`), and the survival tier reads an auto-coordinate
/// as may-touch, never a distinct canonical (`fence-no-disjoint`, `survival::disjoint`). A prefix
/// check is sound because no authored kind can carry the `:` (`277` §4b) — this is a namespace
/// membership test, not a decode of oracle meaning (`inv-referent-agnostic` is unbroken).
///
/// # The whole consumer set (audited 2026-07-27; keep this list exhaustive)
///
/// The two fences answer to DIFFERENT questions and only one of them is really about the kind:
///
/// * WHICH-BODY-SHIPS — `cli`'s `ship_auto` closure gates on this predicate, and `compile_probe`
///   reads a `Some` from it as "ship the stripped verdict body under the `__is_converged` name".
///   The question it is really asking is *"is this provider verdict-borne with no predict to
///   ship"*, and the kind is only today's proxy for that, sound only while the auto-cell is the
///   sole cell a verdict-only provider can carry. A verdict-marked site keying an AUTHORED
///   coordinate would fall through to the predict lane, find nothing, and run — so a keying
///   change must move this discriminator off the kind, not reuse it.
/// * FENCE-NO-DISJOINT — `plan::survival::disjoint`, via `Resolutions::is_auto`, fed by
///   `add_auto_kind` at the `cli` edge for every verdict-provider. This one IS about the kind:
///   the synthetic singleton must never manufacture separation, whatever else changes.
///
/// Nothing else reads it. [`auto_fact`] is the sole mint ([`AUTO_KIND_PREFIX`]/[`AUTO_SELECTOR`]
/// have no other callers), and `analysis::effect::auto_or_opaque` is its sole production caller.
#[must_use]
pub fn is_auto_kind(interner: &Interner, kind: KindId) -> bool {
    interner.resolve(kind.0).starts_with(AUTO_KIND_PREFIX)
}

/// Harness support for the Kani lane, homed beside the types (`300` §2a / `301` §3). Every item
/// is `#[cfg(kani)]`, so no ordinary build sees any of it and no constructor widens.
///
/// # Why a bounded symbol domain is the whole statement, not a shortcut
///
/// [`Symbol`] is an opaque identity: `inv-referent-agnostic` says the engine may compare two of
/// them and may resolve one for display, and may never decode one to infer meaning. So every
/// behaviour reachable from a set of symbols depends only on WHICH of them are equal — the
/// values are interchangeable up to renaming. A harness over `N` distinct identities therefore
/// covers every equality pattern its arity can express, and the `u32` width buys nothing. The
/// harness names its `N`.
#[cfg(kani)]
mod kani_support {
    use super::{EntityRef, KindId, OpaqueToken, ProviderId, SelectorId, Symbol};

    impl Symbol {
        /// An arbitrary symbol out of `N` distinct identities.
        pub fn any_of<const N: u32>() -> Self {
            let raw: u32 = kani::any();
            kani::assume(raw < N);
            Symbol(raw)
        }
    }

    /// Two identities: enough for same-vs-different, the only distinction a referent-agnostic
    /// engine can draw from one pair.
    impl kani::Arbitrary for Symbol {
        fn any() -> Self {
            Self::any_of::<2>()
        }
    }

    impl kani::Arbitrary for OpaqueToken {
        fn any() -> Self {
            OpaqueToken(kani::any())
        }
    }

    impl kani::Arbitrary for KindId {
        fn any() -> Self {
            KindId(kani::any())
        }
    }

    impl kani::Arbitrary for ProviderId {
        fn any() -> Self {
            ProviderId(kani::any())
        }
    }

    impl kani::Arbitrary for SelectorId {
        fn any() -> Self {
            SelectorId(kani::any())
        }
    }

    impl kani::Arbitrary for EntityRef {
        fn any() -> Self {
            if kani::any() {
                EntityRef::Operand(kani::any())
            } else {
                EntityRef::Singleton
            }
        }
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_grade_weakest_is_the_taint_meet() {
        use ValueGrade::{AuthorComposed, ProgramText, Register, Top, WorldSpoken};
        // The `275` §2 example: a value from a delegation read (world-spoken) concatenated with a
        // composed decoration (author-composed) grades as composed (the weaker).
        assert_eq!(WorldSpoken.weakest(AuthorComposed), AuthorComposed);
        // ⊤ is the absorbing bottom; ProgramText is the identity (never weakens a concatenation).
        assert_eq!(Top.weakest(Register), Top);
        assert_eq!(ProgramText.weakest(WorldSpoken), WorldSpoken);
        assert_eq!(ProgramText.weakest(ProgramText), ProgramText);
        // The full weakest→strongest order (a value is only as trustworthy as its weakest part).
        assert!(Top < AuthorComposed);
        assert!(AuthorComposed < WorldSpoken);
        assert!(WorldSpoken < Register);
        assert!(Register < ProgramText);
    }

    #[test]
    fn interner_dedups_and_roundtrips() {
        let mut i = Interner::default();
        let nginx_a = i.intern("nginx");
        let nginx_b = i.intern("nginx");
        let apt = i.intern("apt");
        assert_eq!(nginx_a, nginx_b, "equal text must intern to equal symbol");
        assert_ne!(nginx_a, apt);
        assert_eq!(i.resolve(nginx_a), "nginx");
        assert_eq!(i.resolve(apt), "apt");
    }

    #[test]
    fn interner_symbol_assignment_is_deterministic() {
        let mut a = Interner::default();
        let mut b = Interner::default();
        for s in ["one", "two", "three", "two"] {
            let _ = a.intern(s);
            let _ = b.intern(s);
        }
        assert_eq!(a.intern("one"), b.intern("one"));
        assert_eq!(a.intern("three"), b.intern("three"));
    }

    #[test]
    fn auto_cell_is_per_provider_singleton_and_unnameable() {
        // `24L` §2/§3/§7: the auto-cell is a per-provider singleton (entity-free), and its kind
        // lives in a namespace no authored kind can reach (`fence-unnameable` — an authored kind is
        // DNS-labels-before-a-colon, so it can never carry the `:` the prefix embeds).
        let mut i = Interner::default();
        let foobar = auto_fact(&mut i, "foobar");
        let mycmd = auto_fact(&mut i, "mycmd");
        assert_eq!(
            foobar.entity,
            EntityRef::Singleton,
            "auto-cell is entity-free (fence-no-entity)"
        );
        assert_ne!(
            foobar.kind, mycmd.kind,
            "distinct providers ⇒ distinct auto-kinds"
        );
        assert_eq!(
            auto_fact(&mut i, "foobar").kind,
            foobar.kind,
            "same provider ⇒ the one shared singleton cell (§3 coarseness)"
        );
        assert!(is_auto_kind(&i, foobar.kind), "an auto-kind is recognised");
        // An authored reverse-DNS kind is NEVER mistaken for auto (the prefix carries a `:`).
        let authored = KindId(i.intern("com.debian.apt.Package"));
        assert!(
            !is_auto_kind(&i, authored),
            "an authored kind is never auto (fence-unnameable)"
        );
    }

    #[test]
    fn wrapped_and_ambient_same_cell_never_collide() {
        // `27C` §3 (the non-negotiable no-collision requirement): two facts naming the SAME cell
        // (kind, entity, selector) but born in DIFFERENT contexts must be UNEQUAL — so a
        // `BTreeSet<FactKey>` (the host fact store, the probe results lane) holds them as distinct
        // entries and a wrapped-context measurement can never overwrite/alias an ambient one.
        let mut i = Interner::default();
        let kind = KindId(i.intern("com.debian.apt.Package"));
        let entity = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let selector = SelectorId(i.intern("installed"));
        let ambient = FactKey::cell(kind, entity, selector);
        let root_ctx = Context::Wrapped(ContextKey(i.intern("user=root")));
        let target_ctx = Context::Wrapped(ContextKey(i.intern("fs-view=/mnt/target")));
        let in_root = ambient.in_context(root_ctx);
        let in_target = ambient.in_context(target_ctx);
        assert_ne!(ambient, in_root, "ambient ≠ wrapped-root: no collision");
        assert_ne!(in_root, in_target, "two distinct contexts never collide");
        let set: std::collections::BTreeSet<FactKey> =
            [ambient, in_root, in_target].into_iter().collect();
        assert_eq!(set.len(), 3, "three distinct cells-in-worlds, none aliased");
        // Same cell, SAME context ⇒ equal (the keying is by the composed key, not by identity):
        // a re-measurement in the same wrapper-denoted world hits the same slot (self-healing).
        assert_eq!(ambient.in_context(root_ctx), in_root);
    }
}
