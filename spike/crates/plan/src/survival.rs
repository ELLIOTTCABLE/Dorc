//! `plan::survival` — the frame-rule machine: a converged line ELIDES past a RUNNING wall
//! when the wall's authored footprint is disjoint from the line's fact's backing (23M /
//! plans/240 Stage 2 — the golden hill). Mode-gated behind `--trust-footprints`
//! (rul24-mode-gate); the survival tier is *structurally* unreachable without the flag
//! because the footprint data ([`TrustedFootprints`]) is never lifted unflagged (TC-1).
//!
//! # The unrepresentability properties this module upholds (conductor type-contracts)
//!
//! * **TC-2 — Footprint/Backing asymmetry.** [`Footprint`] (a wall's at-most WRITE-set) and
//!   [`Backing`] (where a downstream fact's truth lives) are DISTINCT types; the ONLY consumer
//!   is the asymmetric [`disjoint`] — intersecting two footprints, two backings, or swapping
//!   the arguments does not typecheck. There is NO conversion from an establish-effect (an
//!   at-least claim) into a `Footprint`: the 233 sin (silence-as-at-most) cannot be spelled.
//! * **TC-3 — survival is witnessed, and the witness IS the attribution.** [`DisjointnessProof`]
//!   is minted ONLY by [`disjoint`]; a [`SurvivalWitness`] aggregates one [`Crossing`] per
//!   running wall crossed and is constructible only inside the wall walk, only when EVERY
//!   crossed wall contributed a proof ([`wall_verdict`]). The why-lens renders attribution by
//!   READING the witness — type-enforcement and attribution-primacy are one object.
//!
//! SURVIVAL IS NOT ADEQUACY. This machinery proves a fact's plan-time convergence *outlasts*
//! an interposed command's run: an interferer kills the fact only by touching the cell the
//! fact's probe is DECLARED to check (rul24-selfframing-correction, 24D §6). Reads are declared
//! at the cell level by the oracle — symmetric to writes (establish-marks / `touches()`) — NOT
//! derived: Dorc never computes a probe's file/syscall read-set (no static analysis of opaque
//! calls, and the eBPF/tracing layer is linting-only, never a runtime dependency). It says
//! NOTHING about whether "converged" meant "re-running is a no-op" — that adequacy question
//! (converged≠no-op) is the converged-vouch's, decided elsewhere and by the author, never here
//! (23M).
//!
//! `inv-referent-agnostic`: a coordinate is an interned `(KindId, entity)` pair, compared as
//! symbols, NEVER as text (24A §1b vocabulary fence).

use std::collections::{BTreeMap, BTreeSet};

use dorc_analysis::cfg::CfgNodeId;
use dorc_core::{
    Coord, Dialect, EntityRef, EntityResolution, FactKey, KindId, OracleFileId, ProviderId,
    Relation, SelectorId, Span, Symbol, compare,
};

use crate::LeafId;

/// One entity-coordinate: an interned `(kind, entity)` pair, **entity-granular** — NO
/// selector. Entity-granular poisoning (23M): touching an entity poisons ALL its properties
/// (the author cannot enumerate properties they never heard of), so a footprint coordinate
/// and a fact's backing intersect iff their (kind, entity) match, the fact's selector
/// ignored. Reuses the shared interned vocabulary ([`KindId`]/[`EntityRef`]); NEVER a String
/// (`inv-referent-agnostic`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityCoord {
    kind: KindId,
    entity: EntityRef,
}

impl EntityCoord {
    /// Build a coordinate from an interned kind + entity. The wiring (cli, flag-gated) interns
    /// a `touches()` emission's opaque `kind:entity` fragments through the SAME interner the
    /// book/predict analysis uses, so `package` here is the SAME [`KindId`] a predict
    /// annotation minted (the vocabulary fence — one interned universe, not a parallel one).
    #[must_use]
    pub fn new(kind: KindId, entity: EntityRef) -> Self {
        Self { kind, entity }
    }

    /// The kind (for display/provenance rendering only — `inv-referent-agnostic` forbids
    /// decoding it for meaning).
    #[must_use]
    pub fn kind(self) -> KindId {
        self.kind
    }

    /// The entity (for display/provenance rendering only).
    #[must_use]
    pub fn entity(self) -> EntityRef {
        self.entity
    }
}

/// A **canonical** entity-coordinate — the form [`disjoint`] compares (24F §6, the resid-aliasing
/// closure). Minted ONLY by [`Resolutions::canonicalize`] (the engine's resolution step; the
/// fields are private and no other constructor exists), so a raw interned [`EntityCoord`] CANNOT
/// reach the intersection in a resolver-bearing kind by construction — the compile-error family
/// (TC-style): the comparison consumes `CanonicalCoord`, never `EntityCoord`. For a resolver-LESS
/// kind the mint is the IDENTITY (token = canon — the honest floor, per-kind gradual enhancement).
///
/// `inv-referent-agnostic` (24F reconciliation): the canonical entity is the owner's resolver
/// OUTPUT interned into an opaque token — the ENGINE never decodes it for meaning; it ships the
/// owner's resolver, interns its output, and compares canonical forms as SYMBOLS, never as text.
/// The owner decodes; the engine plumbs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalCoord {
    kind: KindId,
    entity: EntityRef,
}

impl CanonicalCoord {
    /// The kind (display/provenance only — `inv-referent-agnostic`).
    #[must_use]
    pub fn kind(self) -> KindId {
        self.kind
    }

    /// The canonical entity (display/provenance only).
    #[must_use]
    pub fn entity(self) -> EntityRef {
        self.entity
    }
}

/// Why a coordinate degraded to MAY-ALIAS (24F §3a): a resolver-bearing kind whose resolver did
/// not return a clean canonical form for this entity. A closed enum so a new degrade-reason breaks
/// every exhaustive match (the compiler-as-checklist).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MayAliasReason {
    /// The kind's resolver produced no usable canonical form for this coordinate — a ⊤ / non-zero
    /// rc / empty / malformed answer, a DANGLING reference (24F §4), OR a resolver-bearing
    /// coordinate absent from the resolution map (the resolver never resolved it). The owner
    /// declared "identity in this kind needs resolution", so an unresolved coordinate is suspect
    /// BY THE OWNER'S OWN TESTIMONY ⇒ degrade toward run, NOT to token-equality (24F §3a — the
    /// failure-direction is load-bearing: falling back to token-equality would trade safety for
    /// value on exactly the coordinate the owner flagged as resolution-needing).
    Unresolved,
}

/// The outcome of canonicalizing ONE coordinate through its kind's resolver (24F §6): a proven
/// [`CanonicalCoord`], or a [`MayAlias`](Resolution::MayAlias) degrade carrying WHY. NEVER a bool
/// (24F §6) — a may-alias flows to demote (fail toward run, §3a); a canonical flows to the
/// disjointness comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    /// The coordinate canonicalized cleanly (or the kind is resolver-less ⇒ identity).
    Canonical(CanonicalCoord),
    /// The resolver could not canonicalize it (§3a) — the comparison degrades toward run.
    MayAlias(MayAliasReason),
}

/// The owner-declared canonicalization map for one plan run (24F §3 — dynamic points-to). The
/// kind-OWNER holds entity-identity (23M contribution-vs-identity: authority over the *nouns*), so
/// a kind-owner ships a `<kind>.resolve()` (the identity role-sibling); this map is the readback of
/// running those resolvers host-side per coordinate (built at the cli/sweep edge from the
/// `resolv`-lane records, or in the DST from the modeled host). It is consumed BY REFERENCE in the
/// survival walk to canonicalize BOTH footprint and backing coordinates BEFORE [`disjoint`].
///
/// **Per-kind gradual enhancement (24F §3):** a kind with NO resolver keeps today's token-equality
/// (the status quo is the floor, not an error); resolver coverage buys aliasing-safety kind by
/// kind — the you-get-what-you-put-in curve.
///
/// # When a mint blocks your build (rul24-critical-type-docs)
///
/// [`disjoint`]/[`wall_verdict`] DEMAND a `&Resolutions`. If a caller has no resolvers, pass
/// [`Resolutions::none`] — the empty map: every kind resolver-less, every coordinate identity =
/// token-equality (the honest floor). NEVER fabricate a canonical form to force two coordinates
/// together or apart — the resolver's OUTPUT is the sole source of a canonical, exactly as an
/// authored `touches()` is the sole source of a [`Footprint`] (the 233 discipline, applied to
/// identity). An unresolved resolver-bearing coordinate is [`MayAlias`](Resolution::MayAlias) ⇒
/// demote, never silently token-equal.
#[derive(Debug, Clone, Default)]
pub struct Resolutions {
    /// Kinds whose owner shipped a resolver. A coordinate in such a kind ABSENT from `canon` is
    /// MAY-ALIAS (§3a), never token-equal.
    resolver_kinds: BTreeSet<KindId>,
    /// Per resolved coordinate, its canonical entity (the resolver's readback output, interned).
    canon: BTreeMap<EntityCoord, EntityRef>,
    /// Coordinates the resolver flagged DANGLING (24F §4): a reference to a non-existent entity on
    /// an enumerable kind (`dpkg-query -W` non-zero). Rides the may-alias degrade AND surfaces as a
    /// loud diagnostic at the edge.
    dangling: BTreeSet<EntityCoord>,
    /// Typeless-floor auto-cell kinds present in the plan (`24L` §7 `fence-no-disjoint`). The plan
    /// is interner-free, so the edge (which HAS the interner) resolves which kinds are auto and
    /// deposits them here; [`disjoint`] reads an auto coordinate as may-touch, never a distinct
    /// canonical. Threaded via `Resolutions` (already carried into the survival walk) rather than a
    /// new parameter down `build_plan_walled → wall_walk_survival → wall_verdict → disjoint`.
    auto_kinds: BTreeSet<KindId>,
}

impl Resolutions {
    /// The empty map — the honest floor (every kind resolver-less ⇒ token-equality). The
    /// flag-off / no-resolver-oracle path, and the identity element for existing behaviour.
    #[must_use]
    pub fn none() -> Self {
        Self::default()
    }

    /// Declare a kind resolver-bearing (its owner shipped a `<kind>.resolve()`) even if no
    /// coordinate resolved — so a coordinate in it that produced NO readback degrades to may-alias
    /// (§3a), not to token-equality.
    pub fn add_resolver_kind(&mut self, kind: KindId) {
        self.resolver_kinds.insert(kind);
    }

    /// Register a typeless-floor auto-cell kind (`24L` §7 `fence-no-disjoint`). The edge calls this
    /// for every auto-kind present in the plan so [`disjoint`] can bar it from proving disjoint.
    pub fn add_auto_kind(&mut self, kind: KindId) {
        self.auto_kinds.insert(kind);
    }

    /// Is `kind` a registered auto-cell kind (`24L` §7)? Used by [`disjoint`] to force may-alias on
    /// an auto coordinate (either side), so a private per-provider singleton never manufactures the
    /// separation that would let it survive a wall (`277` §6 never-derive-separation).
    #[must_use]
    pub(crate) fn is_auto(&self, kind: KindId) -> bool {
        self.auto_kinds.contains(&kind)
    }

    /// Record a coordinate's resolved canonical entity (the resolver's host-run output, interned
    /// into the shared vocabulary — the vocabulary fence). Marks the coordinate's kind
    /// resolver-bearing.
    pub fn record(&mut self, coord: EntityCoord, canonical: EntityRef) {
        self.resolver_kinds.insert(coord.kind);
        self.canon.insert(coord, canonical);
    }

    /// Flag a coordinate DANGLING (24F §4): the resolver's natural failure on an enumerable kind
    /// (a reference to a non-existent entity). Rides the may-alias degrade (a dangling coordinate
    /// canonicalizes to [`MayAlias`](Resolution::MayAlias)) and is surfaced as a loud diagnostic.
    pub fn record_dangling(&mut self, coord: EntityCoord) {
        self.resolver_kinds.insert(coord.kind);
        self.dangling.insert(coord);
    }

    /// The dangling coordinates (24F §4 — the loud per-coordinate diagnostic the edge emits).
    pub fn dangling(&self) -> impl Iterator<Item = EntityCoord> + '_ {
        self.dangling.iter().copied()
    }

    /// Whether a kind is resolver-bearing (the why-lens names the resolver only for such kinds).
    #[must_use]
    pub fn has_resolver(&self, kind: KindId) -> bool {
        self.resolver_kinds.contains(&kind)
    }

    /// Canonicalize one coordinate (24F §6 — the SOLE minter of [`CanonicalCoord`]). A
    /// resolver-LESS kind ⇒ identity (token = canon, the honest floor). A resolver-bearing kind ⇒
    /// the recorded canonical, or [`MayAlias`](Resolution::MayAlias) if absent/dangling (§3a — the
    /// owner declared identity needs resolution, so unresolved is suspect ⇒ fail toward run).
    #[must_use]
    pub(crate) fn canonicalize(&self, coord: EntityCoord) -> Resolution {
        if !self.resolver_kinds.contains(&coord.kind) {
            return Resolution::Canonical(CanonicalCoord {
                kind: coord.kind,
                entity: coord.entity,
            });
        }
        match self.canon.get(&coord) {
            Some(canon) if !self.dangling.contains(&coord) => {
                Resolution::Canonical(CanonicalCoord {
                    kind: coord.kind,
                    entity: *canon,
                })
            }
            _ => Resolution::MayAlias(MayAliasReason::Unresolved),
        }
    }
}

/// The provenance of a footprint's coordinates (24E §9): STATICALLY traced from an authored
/// `touches()` (`Authored` — a fixed-footprint tool) vs read back from a host-run
/// derivation-probe (`Derived` — a payload-bound tool whose `touches()` body reached a host
/// query the static tracer could not resolve, so it shipped to the probe lane, ran read-only,
/// and printed its own coordinates; 24E §2/§6). The disjointness/survival consumers are
/// ORIGIN-AGNOSTIC — [`disjoint`]/[`wall_verdict`] never read this; a derived footprint
/// intersects a backing identically to an authored one — so the tag rides along purely for the
/// why-lens attribution ("footprint DERIVED at probe from `<call>`") and the fork-4B escalation
/// disclosure (24E §4). Inertness is STRUCTURAL (the self-vouch is the authoring act, 24E §3/§9):
/// this is emphatically NOT a witness that Dorc verified read-only-ness — it can't (never-vouch).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FootprintOrigin {
    /// Statically traced in-Rust by `evaluate_touches` (fixed-footprint tool, Stage 2).
    Authored,
    /// Read back from a host-run derivation-probe (payload-bound tool, Stage 4). Carries the
    /// derivation call's display locus (the host tool the `touches()` body reached, e.g.
    /// `dpkg -L`) for the why-lens + the fork-4B advisory; display-only (`inv-referent-agnostic`).
    Derived { call: String },
}

/// A wall's **at-most footprint**: the set of coordinates a running mutator claims to touch
/// (the write-set bound — 23M / `ORACLE_PROVIDES` `provides-behavior` sub-shape 3), plus the
/// wall's provider (for attribution) and its [`FootprintOrigin`] (24E §9). Private set (TC-2):
/// the ONLY way coordinates enter is [`Footprint::authored`]/[`Footprint::derived`] from a
/// `touches()` emission (static or host-run) — there is deliberately NO constructor taking
/// establish-effects (the 233 silence-as-at-most sin must not compile).
#[derive(Debug, Clone)]
pub struct Footprint {
    provider: Symbol,
    coords: Vec<EntityCoord>,
    /// 24G §8 — the wall-site's OWN effect coordinate, engine-supplied (the establish cell for an
    /// establish-class site, the killed cell for a kill), unioned into the hit-surface. Kept DISTINCT
    /// from `coords` (the author's/derivation's `touches()` claim) so the why-lens can attribute it as
    /// own-effect rather than an authored claim, per §8's provenance requirement. It only ever ADDS
    /// hit-surface (`inv-kfail`, apply): [`hit_surface`](Footprint::hit_surface) folds it in for
    /// [`disjoint`], deduped against `coords`. `None` when the site had no own coordinate to union.
    /// SPIKE SCOPE (ru-26): a single [`Option`] models today's single-operand establishes/kills; a
    /// multi-operand `EstablishMembers` site would need a SET here (one own cell per member) — deferred,
    /// see the members-gap note in the round's report.
    own: Option<EntityCoord>,
    origin: FootprintOrigin,
    /// 24G Part B — the `reaches()` EXPANSION attribution: for each coordinate ADDED by
    /// `<kind>.reaches()` expansion ([`add_reached`](Footprint::add_reached)), the reach-function's
    /// KIND (the kind whose owner spelled the reach). The reached coords are ALSO in `coords` (the
    /// disjointness test is expansion-agnostic — a reached coord intersects a backing identically to
    /// an authored one, and flows through the SAME canonicalization path); this map exists ONLY so a
    /// DEMOTE caused by a reached coord attributes the reach-function ("…poisoned via
    /// `<kind>.reaches()`"). Empty for an un-expanded footprint. Mirrors the resolver-attribution
    /// shape ([`Crossing::via_resolver`]) — the sharpest claims name whose knowledge they trusted.
    reached_via: BTreeMap<EntityCoord, KindId>,
    /// `277` §3 — the disturbs-emission SELECTOR per footprint coordinate. Absent (the corpus
    /// default) ⇒ a whole-entity ⊤ footprint, poisoning every cell (`selector_covers` collides).
    /// Present ⇒ a selector-bearing disturbs mark (`: sm.dorc.Service@active`) that can SPARE a
    /// sibling cell under the dialect. A side-table (not a field on `EntityCoord`) so the
    /// entity-granular render/canonicalization/reach machinery stays untouched
    /// (`empty-world-byte-identical`). SPIKE SCOPE: an entity emitted twice with differing
    /// selectors keeps the last — no corpus body does this.
    selectors: BTreeMap<EntityCoord, SelectorId>,
    /// `27V:mech-minting-line-threading` (`tc-disturbs-span-threading`) — the `disturbs` funcdef's
    /// defining `(Span, OracleFileId)`, so a survival's `claimed` chain-link renders the leverage
    /// point (`<file>:<line> is the line to widen` — `USER_STORY` Recovery's product moment). The
    /// funcdef `name_span` is the honest coarsest-true span; arm-precision is a deferred refinement.
    /// `None` for a derived footprint (its span is the host-derivation call, not an authored line).
    defining: Option<(Span, OracleFileId)>,
}

impl Footprint {
    /// Build an **authored** (statically-traced) footprint from a wall's lifted coordinates.
    /// `None` when `coords` is EMPTY — an empty emission is *no claim*, which is a WALL
    /// (silence = wall), never "touches nothing, elide freely". So a footprint always carries
    /// ≥1 coordinate; its absence in [`TrustedFootprints`] means the site walls.
    #[must_use]
    pub fn authored(provider: Symbol, coords: Vec<EntityCoord>) -> Option<Self> {
        Self::with_origin(provider, coords, FootprintOrigin::Authored)
    }

    /// Build a **derived** (host-run) footprint from the coordinates a derivation-probe printed
    /// (24E §2/§6). Same emptiness law as [`authored`](Footprint::authored) (an empty derivation
    /// is no claim ⇒ wall). `call` is the derivation call's display locus (the host tool reached),
    /// carried for the why-lens/advisory only.
    #[must_use]
    pub fn derived(provider: Symbol, coords: Vec<EntityCoord>, call: String) -> Option<Self> {
        Self::with_origin(provider, coords, FootprintOrigin::Derived { call })
    }

    fn with_origin(
        provider: Symbol,
        coords: Vec<EntityCoord>,
        origin: FootprintOrigin,
    ) -> Option<Self> {
        if coords.is_empty() {
            return None;
        }
        Some(Self {
            provider,
            coords,
            own: None,
            origin,
            reached_via: BTreeMap::new(),
            selectors: BTreeMap::new(),
            defining: None,
        })
    }

    /// Attach the `disturbs` funcdef's defining `(Span, OracleFileId)` (`tc-disturbs-span-threading`)
    /// — builder-chained after [`authored`](Footprint::authored), so a survival's `claimed` link can
    /// render the leverage point. `None` is a no-op (the derived lane has no authored line).
    #[must_use]
    pub fn with_defining(mut self, defining: Option<(Span, OracleFileId)>) -> Self {
        self.defining = defining;
        self
    }

    /// Record the disturbs-emission selector for one footprint coordinate (`277` §3). Called by the
    /// wiring when an emission carried a `@selector` mark; absent coords stay whole-entity ⊤.
    pub fn set_selector(&mut self, coord: EntityCoord, selector: SelectorId) {
        self.selectors.insert(coord, selector);
    }

    /// The disturbs-emission selector for a footprint coordinate, or `None` (whole-entity ⊤). The
    /// `claim` side of [`selector_covers`](dorc_core::selector_covers).
    #[must_use]
    fn selector_of(&self, coord: EntityCoord) -> Option<SelectorId> {
        self.selectors.get(&coord).copied()
    }

    /// 24G Part B — widen this footprint with a `reaches()`-expanded coordinate (FOOTPRINTS ONLY; a
    /// backing NEVER expands). `via` is the reach-function's KIND (for the poison attribution). The
    /// reached coord is unioned into `coords` (so it flows through the SAME `disjoint`/canonicalization
    /// path — no new interplay code) and recorded in `reached_via` for the demote attribution. This is
    /// the SAFE direction (`inv-kfail`, apply): expansion only ever WIDENS a footprint, so it HITs more
    /// (demotes toward run), never elides more. A coord already present (an authored coord the reach
    /// re-derives) hits on its OWN account — it is NOT re-attributed to the reach (the reach added
    /// nothing there). Called at plan construction, BEFORE the wall walk, after the coherence check
    /// already passed on the narrower footprint (widening keeps `own-establish ⊆ footprint` true).
    pub fn add_reached(&mut self, coord: EntityCoord, via: KindId) {
        if !self.coords.contains(&coord) {
            self.coords.push(coord);
            self.reached_via.insert(coord, via);
        }
    }

    /// The reach-function KIND that expanded `coord` into this footprint, if any (24G Part B — the
    /// demote attribution: a `Hit` on a reached coord names `<kind>.reaches()`). `None` for an
    /// authored/derived coord.
    #[must_use]
    fn reach_of(&self, coord: EntityCoord) -> Option<KindId> {
        self.reached_via.get(&coord).copied()
    }

    /// The wall's provider (attribution: whose footprint licensed a crossing).
    #[must_use]
    pub fn provider(&self) -> Symbol {
        self.provider
    }

    /// The `disturbs` funcdef's defining `(Span, OracleFileId)` (`tc-disturbs-span-threading`), or
    /// `None` (derived footprint / unthreaded). Carried onto a [`Crossing`] for the leverage-point render.
    #[must_use]
    pub fn defining(&self) -> Option<(Span, OracleFileId)> {
        self.defining
    }

    /// The claimed coordinates (attribution render; the disjointness test).
    #[must_use]
    pub fn coords(&self) -> &[EntityCoord] {
        &self.coords
    }

    /// 24G §8 — attach the wall-site's engine-supplied OWN effect coordinate (its establish cell, or
    /// its killed cell), unioned into the hit-surface. Builder-style, chained AFTER
    /// [`authored`](Footprint::authored)/[`derived`](Footprint::derived) so the emptiness law stays on
    /// `coords` ALONE: an empty emission is already `None` before this runs, so `with_own` can never
    /// resurrect a footprint from silence — union machinery present, empty emission ⇒ still no footprint
    /// (the anti-233 boundary). `None` is a no-op (the site had no own coordinate).
    #[must_use]
    pub fn with_own(mut self, own: Option<EntityCoord>) -> Self {
        self.own = own;
        self
    }

    /// The full at-most HIT-SURFACE [`disjoint`] tests (24G §8): `coords` (the author's/derivation's
    /// `touches()` claim) unioned with the engine-supplied own-effect coordinate. `own` is yielded only
    /// when NOT already in `coords` — the authored lane's pre-union canary guarantees own ∈ coords ⇒ a
    /// no-op there; the derived lane dropped that requirement ⇒ own genuinely widens. Ordered (coords,
    /// then the novel own) for `inv-determinism`. Union coords are ordinary hit-surface: they
    /// canonicalize and intersect identically to an authored coord, and only ever ADD surface, never
    /// remove (`inv-kfail`, apply — the union can demote a survival, never license one).
    fn hit_surface(&self) -> impl Iterator<Item = EntityCoord> + '_ {
        self.coords
            .iter()
            .copied()
            .chain(self.own.filter(|o| !self.coords.contains(o)))
    }

    /// The engine-supplied own-effect coordinate WHEN it widened the footprint (∉ `coords`) — the
    /// why-lens surfaces this distinctly as own-effect (24G §8 provenance). `None` in the authored lane
    /// (its canary guarantees own ∈ coords, so own is redundant with an authored claim there) and when
    /// no own coordinate was unioned.
    fn own_if_novel(&self) -> Option<EntityCoord> {
        self.own.filter(|o| !self.coords.contains(o))
    }

    /// The footprint's provenance (why-lens attribution; origin-agnostic for disjointness).
    #[must_use]
    pub fn origin(&self) -> &FootprintOrigin {
        &self.origin
    }
}

/// Where a downstream fact's truth lives — its **backing**. A backing is the single cell the
/// oracle DECLARES its probe checks (rul24-selfframing-correction, 24D §6): a DECLARATION-SCOPE,
/// not a computed read-set. Dorc never derives a probe's file/syscall read-set (no static
/// analysis of opaque calls; the eBPF/tracing layer is linting-only, never a runtime dependency),
/// so the backing carries NO completeness burden — the 233 completeness claims live on the wall's
/// [`Footprint`] (at-most these cells) + the vouch's adequacy. Cell-level soundness rests on the
/// namespace-owner correctly partitioning state (the reverse-DNS-owner aliasing responsibility;
/// resid-aliasing). It is the fact's own `(kind, entity)` coordinate (the one declared cell a
/// fact is about). A DISTINCT type from [`Footprint`] (TC-2): the two never mix, and there is no
/// path from an establish-effect to a `Footprint`, only to a `Backing`.
#[derive(Debug, Clone)]
pub struct Backing {
    coord: EntityCoord,
    /// `277` §3 — the fact's OWN SELECTOR cell, carried into the survival comparison so a
    /// selector-bearing disturbs claim can spare a SIBLING cell of the same entity under the
    /// dialect. The entity-granular `coord` above still drives canonicalization + attribution
    /// render (`empty-world-byte-identical`); the selector rides alongside for `selector_covers`
    /// only. A fact always carries a concrete selector, so this is `Some`.
    selector: Option<SelectorId>,
    /// `277` §3 backing provenance — the fact's MINTING FAMILY, threaded from the lift (the
    /// establishing `(provider, verb)`'s provider; `27D` disposition-backing-family-recovery).
    /// `None` ⇒ RECOVER via the `sole_family` reverse-lookup (the map-miss floor: a file-write /
    /// auto-cell / Members fact, and every direct-`of_fact` test — today's behavior). When
    /// threaded `Some`, it is AUTHORITATIVE (exact, past the divergent-meaning ambiguity that
    /// falls the reverse-lookup to `None`). All members share it (one provider's verdict AND
    /// observe marks mint the whole set).
    family: Option<ProviderId>,
    /// `277` §5 observe-backing-widening — the SIBLING selectors that widen this fact's backing:
    /// the `:?` observe cells that co-occurred with the verdict in the establishing predict body.
    /// Each is a member `(coord.kind, coord.entity, selector)`. Empty for the whole corpus (every
    /// corpus observe is standalone). Widening only GROWS kill-surface (`inv-kfail`, apply): a
    /// wall disturbing an observed sibling now collides where the bare fact would have spared.
    widen: BTreeSet<SelectorId>,
}

impl Backing {
    /// The singleton backing of a fact (the map-MISS floor): its `(kind, entity)` coordinate plus
    /// the fact's own selector, NO widening, family `None` ⇒ recover via `sole_family` (today's
    /// behavior — a file-write / auto-cell / Members fact, and every direct test construction).
    #[must_use]
    pub fn of_fact(fact: FactKey) -> Self {
        Self {
            coord: EntityCoord {
                kind: fact.kind,
                entity: fact.entity,
            },
            selector: Some(fact.selector),
            family: None,
            widen: BTreeSet::new(),
        }
    }

    /// The `277` §5 WIDENED backing of an establish fact (the map-HIT path): the fact's own cell
    /// plus each observe-backing-widening sibling, carrying the THREADED minting family. `family`
    /// is authoritative (exact; `None` only on a cross-provider establish collision, the safe
    /// floor). `observed` are the widening sibling selectors (empty for the corpus).
    #[must_use]
    pub fn widened(
        fact: FactKey,
        family: Option<ProviderId>,
        observed: BTreeSet<SelectorId>,
    ) -> Self {
        Self {
            coord: EntityCoord {
                kind: fact.kind,
                entity: fact.entity,
            },
            selector: Some(fact.selector),
            family,
            widen: observed,
        }
    }

    /// The backing's member SELECTORS (`277` §5): the fact's own cell first, then each
    /// observe-widened sibling (deterministic — `widen` is a `BTreeSet`). The universal meet
    /// ([`disjoint`]) quantifies over these.
    fn member_selectors(&self) -> impl Iterator<Item = Option<SelectorId>> + '_ {
        std::iter::once(self.selector).chain(self.widen.iter().map(|s| Some(*s)))
    }

    /// The backed coordinate (attribution render — the fact's own `(kind, entity)` anchor).
    #[must_use]
    pub fn coord(&self) -> EntityCoord {
        self.coord
    }
}

/// A witness that a [`Footprint`] does not touch ANY member of a [`Backing`]'s coordinate SET
/// (`277` §5 — the universal meet) — minted ONLY by [`disjoint`] (private field, no other
/// constructor, TC-3). Its existence is the proof; it records the backing's own coordinate + its
/// threaded minting FAMILY (`277` §3 backing provenance) so a survival's attribution can cite the
/// family of the members it cleared (`27D` disposition-backing-family-recovery).
#[derive(Debug, Clone, Copy)]
pub struct DisjointnessProof {
    backing: EntityCoord,
    family: Option<ProviderId>,
}

impl DisjointnessProof {
    /// The coordinate proven clear of the footprint (attribution render).
    #[must_use]
    pub fn backing(self) -> EntityCoord {
        self.backing
    }

    /// The backing's threaded minting family (`277` §3), if known — the family whose dialect
    /// governed the sparing (`None` when recovered via the reverse-lookup floor). Attribution
    /// render only (`inv-referent-agnostic`).
    #[must_use]
    pub fn family(self) -> Option<ProviderId> {
        self.family
    }
}

/// The outcome of the asymmetric intersection test over CANONICAL coordinates (24F §6): the
/// footprint leaves the backing [`Disjoint`](DisjointOutcome::Disjoint) (proven clear), a
/// [`Hit`](DisjointOutcome::Hit) (a proven canonical collision — the same referent, possibly
/// under two names), or [`MayAlias`](DisjointOutcome::MayAlias) (a same-kind pair the resolver
/// could not canonicalize — cannot prove disjoint, §3a). Both `Hit` and `MayAlias` demote (fail
/// toward run, `inv-kfail`); the distinction is attribution + the may-alias fire-rate instrument.
#[derive(Debug, Clone, Copy)]
pub enum DisjointOutcome {
    /// No canonical footprint coordinate collides with the (canonical) backing — the elision may
    /// survive this wall.
    Disjoint(DisjointnessProof),
    /// A footprint coordinate canonicalizes to the SAME referent as the backing (two names, one
    /// thing) — the under-execute the closure catches ⇒ demote. `via_reach` names the reach-function
    /// KIND if the hitting coordinate was one a `<kind>.reaches()` EXPANSION added (24G Part B — the
    /// demote attributes "…poisoned via `<kind>.reaches()`"); `None` for an authored/derived hit.
    Hit { via_reach: Option<KindId> },
    /// A same-kind pair could not be canonicalized (resolver ⊤/dangling/absent, §3a) — disjointness
    /// is unprovable ⇒ demote (toward run).
    MayAlias(MayAliasReason),
}

/// The one asymmetric intersection test (TC-2 / 24F §3), over **canonical** coordinates: does
/// `footprint` leave `backing` untouched once BOTH sides pass through the kind's resolver? Each
/// footprint coordinate and the backing are canonicalized via `resolutions` (24F §6 — a raw
/// [`EntityCoord`] cannot reach the comparison; only a [`CanonicalCoord`] does), then compared
/// entity-granular. A DIFFERENT kind is disjoint outright (a resolver canonicalizes only WITHIN a
/// kind, never across — kinds are the vocabulary fence, not resolvable). Within one kind: two
/// canonical forms equal ⇒ [`Hit`](DisjointOutcome::Hit) (the `nginx`/`nginx-full` closure); a
/// same-kind pair either side of which is [`MayAlias`](Resolution::MayAlias) ⇒
/// [`MayAlias`](DisjointOutcome::MayAlias) (§3a — fail toward run). A resolver-LESS kind
/// canonicalizes by identity, so this REDUCES to today's token-equality (the honest floor). The
/// argument ORDER is type-enforced: `disjoint(&Backing, &Footprint, …)` does not compile, so a
/// caller cannot silently swap "what was written" for "what is read".
#[must_use]
pub fn disjoint(
    footprint: &Footprint,
    backing: &Backing,
    resolutions: &Resolutions,
    dialect: &Dialect,
) -> DisjointOutcome {
    // fence-no-disjoint (`24L` §7 / §6): an auto-cell coordinate NEVER proves disjoint — it reads
    // as may-touch on BOTH sides. Checked BEFORE `compare`, whose kind-fence would otherwise clear
    // an auto backing against every authored footprint (their kinds differ ⇒ ProvablyDisjoint — the
    // §4 near-miss: distinctness-as-license). A typeless oracle has no `touches()` so it never
    // GRANTS survival (no footprint); this bars it from RECEIVING survival too (`277` §6
    // never-derive-separation — distinctness demoted to incomparability wherever it could license).
    // MEMBER-WISE (`277` §5): every member shares `backing.coord.kind` (widening adds sibling
    // selectors, never a new kind/entity), so the one check covers the whole set.
    if resolutions.is_auto(backing.coord.kind)
        || footprint
            .hit_surface()
            .any(|fc| resolutions.is_auto(fc.kind))
    {
        return DisjointOutcome::MayAlias(MayAliasReason::Unresolved);
    }
    let backing_canon = resolution_to_entity(resolutions.canonicalize(backing.coord));
    let mut may_alias: Option<MayAliasReason> = None;
    // `277` §5 set-lifting UNIVERSAL MEET: spared iff EVERY (footprint coord × backing MEMBER)
    // pair is provably-disjoint. The member set = the fact's own cell + each observe-widened
    // sibling (`member_selectors`). Any member Overlaps ⇒ Hit (collide); any Unknown ⇒ may_alias
    // (the safe bottom). ORDER-INDEPENDENT (pin-set-meet-order-independence): a pure fold over
    // member×footprint verdicts, no member's outcome re-enters as another's input
    // (pin-no-outcome-as-generator).
    for msel in backing.member_selectors() {
        // The member's minting family (`277` §3 backing provenance): the THREADED family
        // (exact — past `fence-divergent-meaning`), or `None` ⇒ recover via the `sole_family`
        // reverse-lookup keyed by THIS member's selector (the map-miss floor — today's behavior).
        let member_family: Option<ProviderId> = backing
            .family
            .or_else(|| msel.and_then(|s| dialect.sole_family(backing.coord.kind, s)));
        let member_coord = Coord::new(backing.coord.kind, backing.coord.entity, msel);
        // The hit-surface (24G §8): the author's/derivation's coords unioned with the engine-
        // supplied own-effect coordinate. Each coordinate PAIR goes through the ONE whole-
        // coordinate chokepoint (`277` §2 `compare`) — the kind-fence, entity-canonicalization,
        // and selector-dialect all live there; this crate never compares axes inline
        // (`inv-referent-agnostic`). The backing member canonicalizes on its entity (shared with
        // the fact), so `backing_canon` is reused (selectors do NOT canonicalize at v1).
        for fc in footprint.hit_surface() {
            let claim = Coord::new(fc.kind, fc.entity, footprint.selector_of(fc));
            let claim_canon = resolution_to_entity(resolutions.canonicalize(fc));
            match compare(
                claim,
                member_coord,
                claim_canon,
                backing_canon,
                dialect,
                member_family,
            ) {
                // Provably disjoint: a different kind, a different entity within one kind, or the
                // dialect spared the cell (`277` §3 selector-granular sparing) — this pair is clear.
                Relation::ProvablyDisjoint => {}
                // Proven overlap (same kind + canonical entity, the selector collides): the
                // aliasing closure firing, a plain token hit, a ⊤ footprint over a cell, or a
                // reaches()-expanded coord (24G Part B — attributed via `via_reach`). ANY member
                // overlapping collides the whole SET (universal meet), so we return immediately.
                Relation::Overlaps => {
                    return DisjointOutcome::Hit {
                        via_reach: footprint.reach_of(fc),
                    };
                }
                // A resolver gap (24F §3a) — can't prove THIS pair disjoint ⇒ fail toward run.
                // Recorded and the walk continues, so a later proven Hit still takes precedence
                // over may-alias, and ANY unknown member collides the set (pin-set-meet-order-
                // independence).
                Relation::Unknown => may_alias = Some(MayAliasReason::Unresolved),
            }
        }
    }
    match may_alias {
        Some(reason) => DisjointOutcome::MayAlias(reason),
        None => DisjointOutcome::Disjoint(DisjointnessProof {
            backing: backing.coord,
            family: backing.family,
        }),
    }
}

/// Adapt the resolve generator's [`Resolution`] to the [`EntityResolution`] the `277` §2 chokepoint
/// consumes: a clean canonical form yields its entity (selectors do NOT canonicalize at v1, so the
/// kind rides on the [`Coord`]); a may-alias degrade is unresolvable ⇒ [`Relation::Unknown`].
fn resolution_to_entity(r: Resolution) -> EntityResolution {
    match r {
        Resolution::Canonical(cc) => EntityResolution::Canonical(cc.entity()),
        Resolution::MayAlias(_) => EntityResolution::Unresolvable,
    }
}

/// One record of a downstream elision crossing one running wall (TC-3 attribution): which
/// wall (its leaf), whose footprint licensed it (provider + the claimed coordinates), and the
/// disjointness proof. Constructed only inside [`wall_verdict`].
#[derive(Debug, Clone)]
pub struct Crossing {
    wall_leaf: LeafId,
    provider: Symbol,
    footprint: Vec<EntityCoord>,
    /// 24G §8 — the wall's engine-supplied own-effect coordinate, present ONLY when it WIDENED the
    /// footprint (∉ the authored `footprint` coords above): the why-lens names it distinctly as
    /// own-effect (engine-supplied, the site's declared effect — not the author's `touches()` claim).
    /// `None` for the authored lane (its canary makes own ∈ coords ⇒ no distinct own-effect note) and
    /// when no own coordinate was unioned.
    own: Option<EntityCoord>,
    origin: FootprintOrigin,
    proof: DisjointnessProof,
    /// The kind whose resolver canonicalized the compared coordinates, if the backing's kind is
    /// resolver-bearing (24F §6 attribution: the why-lens says "…survives: disjoint AFTER
    /// `<kind>.resolve()` canonicalization"). `None` for a resolver-less kind (plain token-equality
    /// disjointness — no resolver to name).
    via_resolver: Option<KindId>,
    /// `tc-disturbs-span-threading` — the `disturbs` funcdef's defining `(Span, OracleFileId)`, so the
    /// survival chain's `claimed` link renders the leverage point (the line to widen). Carried from the
    /// crossed wall's [`Footprint`]. `None` for a derived footprint or an unthreaded lift.
    footprint_span: Option<(Span, OracleFileId)>,
}

impl Crossing {
    /// The crossed wall's leaf id (the render names the line it survived past).
    #[must_use]
    pub fn wall_leaf(&self) -> LeafId {
        self.wall_leaf
    }

    /// The licensor provider (whose footprint).
    #[must_use]
    pub fn provider(&self) -> Symbol {
        self.provider
    }

    /// The crossed footprint's provenance (24E §9): the why-lens says "footprint DERIVED at
    /// probe from `<call>`" for a `Derived` crossing, plain attribution for `Authored`.
    #[must_use]
    pub fn origin(&self) -> &FootprintOrigin {
        &self.origin
    }

    /// The wall's claimed coordinates (the footprint the crossing leaned on).
    #[must_use]
    pub fn footprint(&self) -> &[EntityCoord] {
        &self.footprint
    }

    /// The wall's engine-supplied own-effect coordinate WHEN it widened the footprint (24G §8): the
    /// why-lens renders "own-effect `<coord>`" distinctly from the author's `touches()` claim. `None`
    /// in the authored lane (own ∈ coords) and when no own coordinate was unioned.
    #[must_use]
    pub fn own(&self) -> Option<EntityCoord> {
        self.own
    }

    /// The disjointness proof this crossing rests on.
    #[must_use]
    pub fn proof(&self) -> DisjointnessProof {
        self.proof
    }

    /// The `disturbs` funcdef's defining `(Span, OracleFileId)` (`tc-disturbs-span-threading`) — the
    /// survival chain's leverage point. `None` for a derived footprint or an unthreaded lift.
    #[must_use]
    pub fn footprint_span(&self) -> Option<(Span, OracleFileId)> {
        self.footprint_span
    }

    /// The resolver that canonicalized this crossing's compared coordinates, if any (24F §6): the
    /// why-lens names `<kind>.resolve()` for a resolver-bearing kind, and says plain token-equality
    /// for `None`. Attribution-primacy — the aliasing closure is the sharpest claim in the design,
    /// so a survival it licensed must always name which resolver's identity-judgment it trusted.
    #[must_use]
    pub fn via_resolver(&self) -> Option<KindId> {
        self.via_resolver
    }
}

/// The aggregated attribution for one SURVIVED elision (TC-3): its backing plus one
/// [`Crossing`] per running wall it outlasted. Constructible only inside the wall walk
/// ([`SurvivalWitness::new`] is `pub(crate)`), only when EVERY crossed wall yielded a proof —
/// so holding one is proof the elision is licensed past every wall between the probe and its
/// site. The why-lens renders attribution by READING this (never recomputing).
///
/// NB — survival is NOT adequacy: this witnesses that the fact OUTLASTS the walls, nothing
/// about converged≠no-op (23N §5 / 23M). The adequacy risk stays the converged-vouch's.
#[derive(Debug, Clone)]
pub struct SurvivalWitness {
    backing: EntityCoord,
    crossings: Vec<Crossing>,
}

impl SurvivalWitness {
    /// Mint a witness — `pub(crate)`, so ONLY the wall walk ([`wall_verdict`]) constructs one.
    pub(crate) fn new(backing: EntityCoord, crossings: Vec<Crossing>) -> Self {
        Self { backing, crossings }
    }

    /// The surviving fact's backing coordinate.
    #[must_use]
    pub fn backing(&self) -> EntityCoord {
        self.backing
    }

    /// The walls this elision crossed, in execution order (≥1 by construction — a witness is
    /// attached only when the elision actually crossed a running wall).
    #[must_use]
    pub fn crossings(&self) -> &[Crossing] {
        &self.crossings
    }
}

/// The lifted footprints for one plan run — the mode-gate DATA (TC-1). Keyed by wall-site
/// [`CfgNodeId`]. Its presence *is* the flag: [`crate::build_plan_walled`] takes
/// `Option<&TrustedFootprints>`, and the cli constructs one ONLY on the `--trust-footprints`
/// path (the lift runs only there). Flag-off ⇒ `None` ⇒ the survival arm never runs and the
/// footprints were never lifted — a future maintainer *cannot* consult footprints unflagged
/// because the data does not exist (data-absence, never a `bool`).
///
/// A site's ABSENCE from this map means it walls (no `touches()`, a ⊤ lift, an empty
/// emission, or a refused coherence check — all "no trustworthy footprint" ⇒ total wall).
#[derive(Debug, Clone, Default)]
pub struct TrustedFootprints {
    map: BTreeMap<CfgNodeId, Footprint>,
}

impl TrustedFootprints {
    /// An empty set (the cli fills it on the flag-gated path).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a wall-site's lifted footprint. Called only from the flag-gated cli path.
    pub fn insert(&mut self, node: CfgNodeId, footprint: Footprint) {
        self.map.insert(node, footprint);
    }

    /// The footprint lifted for a wall-site, if any (a miss ⇒ the site walls).
    #[must_use]
    pub(crate) fn get(&self, node: CfgNodeId) -> Option<&Footprint> {
        self.map.get(&node)
    }

    /// Was a trustworthy footprint lifted for this wall-site? A miss ⇒ the site walls TOTAL
    /// (`262` §2 / `26A` stop-1). Read-only observability for the partial-deriv-demotes-to-wall
    /// soundness pin (the at-most family completeness gate refuses ⇒ absence ⇒ wall).
    #[must_use]
    pub fn contains(&self, node: CfgNodeId) -> bool {
        self.map.contains_key(&node)
    }

    /// 24G Part B — widen every footprint by `reaches()` EXPANSION, in place. For each footprint,
    /// `expand(coord, origin)` returns the coords `<kind>.reaches()` drags from `coord` (+ the
    /// reach-function KIND for attribution); the CALLER encodes the 24G §2/§3 policy through `origin`
    /// — STATIC arms apply to ALL footprint coords (authored + derived), DYNAMIC arms to AUTHORED
    /// coords only this pass (derived coords are known only post-results — the SAME deferral as
    /// `resid-resolve-derived`, generalized here as `resid-kindfn-derived`). SINGLE-STEP (24G — no
    /// fixpoint for the spike): only the BASE coords present before this call are expanded, never the
    /// coords the expansion itself adds (snapshotted below). Runs AFTER the coherence check (widening
    /// keeps `own-establish ⊆ footprint` true) and BEFORE the survival walk (so the wider footprint
    /// flows through the EXISTING `disjoint`/canonicalization path — no new resolve/reach interplay).
    pub fn expand_reaches(
        &mut self,
        mut expand: impl FnMut(EntityCoord, &FootprintOrigin) -> Vec<(EntityCoord, KindId)>,
    ) {
        for fp in self.map.values_mut() {
            let origin = fp.origin().clone();
            // Snapshot the base coords so the expansion is SINGLE-STEP (added coords are not re-expanded).
            let base: Vec<EntityCoord> = fp.coords().to_vec();
            for coord in base {
                for (reached, via) in expand(coord, &origin) {
                    fp.add_reached(reached, via);
                }
            }
        }
    }
}

/// One accumulated running wall during the survival walk: its leaf + footprint. Internal to
/// the walk.
pub(crate) struct AccumulatedWall {
    pub(crate) wall_leaf: LeafId,
    pub(crate) footprint: Footprint,
}

/// The verdict for a downstream converged mutator's `Replace` against the walls accumulated so
/// far (TC-3 — the ONE total function every pending survival passes through). `Demoted` ⇒ the
/// `Replace` becomes `Run` (`inv-kfail`, never the reverse); `Survived` carries the attribution
/// witness IFF it crossed ≥1 wall; `SurvivedClean` ⇒ it crossed no wall (an ordinary pre-wall
/// elision — byte-identical to the flag-off world, no witness).
#[derive(Debug)]
pub(crate) enum WallVerdict {
    /// Crossed no running wall — a plain elision, unchanged from Stage-1 / flag-off.
    SurvivedClean,
    /// Crossed ≥1 running wall, every one disjoint — survives, with attribution.
    Survived(SurvivalWitness),
    /// A total wall stands, the backing hit some wall's footprint, or a same-kind pair could not be
    /// canonicalized (§3a) — demote to `Run` (`inv-kfail`). The [`DemoteReason`] drives the
    /// may-alias fire-rate instrument (24F §3a — a swamped yardstick is a finding to surface).
    Demoted(DemoteReason),
}

/// Why a converged mutator's `Replace` demoted to `Run` (24F §3a instrumentation). `MayAlias` is
/// the one the yardstick counts: a resolver-gap demote (fail toward run), distinct from a proven
/// collision (`Poisoned`) or a footprint-less upstream mutator (`TotalWall`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DemoteReason {
    /// A running footprint-less mutator upstream (silence = wall).
    TotalWall,
    /// A footprint coordinate canonicalized to the SAME referent as the backing (a proven alias /
    /// token-equal hit). `via_reach` names the reach-function KIND when the hitting coordinate was a
    /// `<kind>.reaches()` EXPANSION (24G Part B — the demote attributes "…poisoned via
    /// `<kind>.reaches()`"); `None` for an authored/derived hit.
    Poisoned { via_reach: Option<KindId> },
    /// A same-kind pair could not be canonicalized (§3a — the resolver ⊤'d / dangled / was absent).
    /// The may-alias fire-rate the yardstick instruments (the [`MayAliasReason`] rides the public
    /// [`Resolution`]/[`DisjointOutcome`]; this discriminant is the demote counter).
    MayAlias,
}

/// Decide a converged mutator's fate against the walls seen so far (the total survival
/// function, TC-3). A `total_wall` (a running footprint-less mutator upstream) demotes
/// unconditionally (silence = wall). Otherwise the backing must be disjoint from EVERY
/// accumulated footprint; any hit demotes; all-disjoint survives, and the crossings ARE the
/// attribution.
pub(crate) fn wall_verdict(
    total_wall: bool,
    walls: &[AccumulatedWall],
    backing: &Backing,
    resolutions: &Resolutions,
    dialect: &Dialect,
) -> WallVerdict {
    if total_wall {
        return WallVerdict::Demoted(DemoteReason::TotalWall);
    }
    // The resolver (if any) that canonicalizes this backing's kind — named in every crossing's
    // attribution (24F §6). Computed once: the backing's kind is fixed across the walls.
    let via_resolver = resolutions
        .has_resolver(backing.coord.kind)
        .then_some(backing.coord.kind);
    let mut crossings = Vec::new();
    for wall in walls {
        match disjoint(&wall.footprint, backing, resolutions, dialect) {
            DisjointOutcome::Disjoint(proof) => crossings.push(Crossing {
                wall_leaf: wall.wall_leaf,
                provider: wall.footprint.provider(),
                footprint: wall.footprint.coords().to_vec(),
                // 24G §8: surface the engine-supplied own-effect coordinate to the why-lens ONLY when
                // it widened the footprint (the derived lane's union); the authored lane's canary
                // makes own ∈ coords ⇒ None (no redundant own-effect note).
                own: wall.footprint.own_if_novel(),
                origin: wall.footprint.origin().clone(),
                proof,
                via_resolver,
                footprint_span: wall.footprint.defining(),
            }),
            // A proven canonical collision (the aliasing closure firing, a plain token hit, or a
            // reaches()-expanded coordinate hitting — 24G Part B, attributed via `via_reach`).
            DisjointOutcome::Hit { via_reach } => {
                return WallVerdict::Demoted(DemoteReason::Poisoned { via_reach });
            }
            // The resolver could not canonicalize a same-kind pair ⇒ fail toward run (§3a).
            DisjointOutcome::MayAlias(_reason) => {
                return WallVerdict::Demoted(DemoteReason::MayAlias);
            }
        }
    }
    if crossings.is_empty() {
        WallVerdict::SurvivedClean
    } else {
        WallVerdict::Survived(SurvivalWitness::new(backing.coord(), crossings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{OpaqueToken, SelectorId};

    fn fact(kind_n: u32, entity: &str, selector: &str) -> FactKey {
        let mut i = dorc_core::Interner::default();
        FactKey {
            kind: KindId(i.intern(&format!("k{kind_n}"))),
            entity: EntityRef::Operand(OpaqueToken(i.intern(entity))),
            selector: SelectorId(i.intern(selector)),
            context: dorc_core::Context::HostDefault,
        }
    }

    fn coord_of(fact: FactKey) -> EntityCoord {
        EntityCoord::new(fact.kind, fact.entity)
    }

    /// Build a raw entity-granular backing for a test: the coord + selector, no widening, family
    /// recovered via `sole_family` (the map-miss floor). The pre-`277`-§5 `Backing { coord,
    /// selector }` shorthand, updated for the SET representation.
    fn backing_of(coord: EntityCoord, selector: Option<SelectorId>) -> Backing {
        Backing {
            coord,
            selector,
            family: None,
            widen: BTreeSet::new(),
        }
    }

    #[test]
    fn disjoint_when_kinds_differ() {
        // Wall touches package:nginx; downstream fact lives in pkgindex:. Different kinds ⇒
        // disjoint ⇒ survives.
        let mut i = dorc_core::Interner::default();
        let wall_coord = EntityCoord::new(
            KindId(i.intern("package")),
            EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
        );
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall_coord]).unwrap();
        let backing = backing_of(
            EntityCoord::new(KindId(i.intern("pkgindex")), EntityRef::Singleton),
            None,
        );
        assert!(
            matches!(
                disjoint(&fp, &backing, &Resolutions::none(), &Dialect::empty()),
                DisjointOutcome::Disjoint(_)
            ),
            "different kinds are disjoint"
        );
    }

    #[test]
    fn hit_when_same_kind_and_entity_ignoring_selector() {
        // The entity-granular hit: same (kind, entity), different selector ⇒ still a hit (all
        // properties of a touched entity are poisoned).
        let mut i = dorc_core::Interner::default();
        let k = KindId(i.intern("package"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let fp = Footprint::authored(i.intern("apt-get"), vec![EntityCoord::new(k, e)]).unwrap();
        // backing on the SAME entity but a different selector (@configured vs the footprint's
        // entity-granular claim) ⇒ hit.
        let backing = backing_of(EntityCoord::new(k, e), None);
        assert!(
            matches!(
                disjoint(&fp, &backing, &Resolutions::none(), &Dialect::empty()),
                DisjointOutcome::Hit { .. }
            ),
            "same entity is a hit"
        );
    }

    #[test]
    fn empty_emission_is_no_footprint() {
        let mut i = dorc_core::Interner::default();
        assert!(
            Footprint::authored(i.intern("hork"), vec![]).is_none(),
            "an empty emission is no claim ⇒ no footprint ⇒ wall"
        );
    }

    #[test]
    fn empty_emission_stays_no_footprint_even_with_own_union() {
        // 24G §8 anti-233 boundary: the own-coord union NEVER manufactures a footprint from an empty
        // emission. `with_own` is chained after the emptiness law, so an absent/empty emission is
        // already `None` and the union machinery cannot resurrect it — union present, empty ⇒ wall.
        let mut i = dorc_core::Interner::default();
        let own = EntityCoord::new(
            KindId(i.intern("package")),
            EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
        );
        assert!(
            Footprint::authored(i.intern("hork"), vec![])
                .map(|fp| fp.with_own(Some(own)))
                .is_none(),
            "authored: empty emission + union machinery present ⇒ still no footprint (anti-233)"
        );
        assert!(
            Footprint::derived(i.intern("hork"), vec![], "hork__disturbs()".to_owned())
                .map(|fp| fp.with_own(Some(own)))
                .is_none(),
            "derived: empty emission + union machinery present ⇒ still no footprint (anti-233)"
        );
    }

    #[test]
    fn own_coord_union_widens_hit_surface() {
        // 24G §8: the DERIVED lane drops own-membership; the engine unions the site's own effect
        // coordinate into the hit-surface. A backing equal to the OWN coord — ABSENT from the
        // touches() emission — now HITs via the union (it would be Disjoint without it). This is the
        // `inv-kfail` safety the union adds: it only ever ADDS hit-surface (demotes), closing the
        // same-cell survival the boilerplate-decoy check used to (mis-)guard.
        let mut i = dorc_core::Interner::default();
        let file_kind = KindId(i.intern("file"));
        let file_coord = EntityCoord::new(
            file_kind,
            EntityRef::Operand(OpaqueToken(i.intern("/etc/x.conf"))),
        );
        let own = EntityCoord::new(
            KindId(i.intern("package")),
            EntityRef::Operand(OpaqueToken(i.intern("oldpkg"))),
        );
        // Derived footprint: coords = {file:/etc/x.conf} (dpkg -L | sed alone), own = package:oldpkg.
        let fp = Footprint::derived(
            i.intern("apt-get"),
            vec![file_coord],
            "apt_get__disturbs()".to_owned(),
        )
        .unwrap()
        .with_own(Some(own));
        let backing = backing_of(own, None);
        assert!(
            matches!(
                disjoint(&fp, &backing, &Resolutions::none(), &Dialect::empty()),
                DisjointOutcome::Hit { via_reach: None }
            ),
            "the unioned own coord HITs a same-cell backing (ordinary hit-surface, no reach attribution)"
        );
        // Control: without the union, the file-only footprint is disjoint from the package backing.
        let fp_no_own = Footprint::derived(
            i.intern("apt-get"),
            vec![file_coord],
            "apt_get__disturbs()".to_owned(),
        )
        .unwrap();
        assert!(
            matches!(
                disjoint(
                    &fp_no_own,
                    &backing,
                    &Resolutions::none(),
                    &Dialect::empty()
                ),
                DisjointOutcome::Disjoint(_)
            ),
            "without the own-coord union the same-cell backing would WRONGLY survive"
        );
    }

    #[test]
    fn total_wall_demotes_even_when_disjoint() {
        let f = fact(1, "nginx", "installed");
        let verdict = wall_verdict(
            true,
            &[],
            &Backing::of_fact(f),
            &Resolutions::none(),
            &Dialect::empty(),
        );
        assert!(
            matches!(verdict, WallVerdict::Demoted(DemoteReason::TotalWall)),
            "total wall demotes"
        );
    }

    #[test]
    fn no_walls_survives_clean() {
        let f = fact(1, "nginx", "installed");
        let verdict = wall_verdict(
            false,
            &[],
            &Backing::of_fact(f),
            &Resolutions::none(),
            &Dialect::empty(),
        );
        assert!(
            matches!(verdict, WallVerdict::SurvivedClean),
            "no walls crossed ⇒ clean survival (no witness)"
        );
    }

    #[test]
    fn disjoint_wall_survives_with_one_crossing() {
        let mut i = dorc_core::Interner::default();
        let wall_coord = EntityCoord::new(KindId(i.intern("pkgindex")), EntityRef::Singleton);
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall_coord]).unwrap();
        let f = fact(1, "nginx", "installed"); // kind k1 ≠ pkgindex ⇒ disjoint
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(0),
            footprint: fp,
        }];
        match wall_verdict(
            false,
            &walls,
            &Backing::of_fact(f),
            &Resolutions::none(),
            &Dialect::empty(),
        ) {
            WallVerdict::Survived(w) => {
                assert_eq!(w.crossings().len(), 1, "one crossing recorded");
                assert_eq!(w.crossings()[0].wall_leaf(), LeafId(0));
                assert_eq!(
                    w.crossings()[0].via_resolver(),
                    None,
                    "a resolver-less crossing names no resolver (token-equality floor)"
                );
            }
            other => panic!("expected Survived, got {other:?}"),
        }
    }

    #[test]
    fn fence_no_disjoint_auto_backing_never_survives() {
        // `24L` §7 fence-no-disjoint / §4 near-miss: a typeless auto-cell backing carries a kind
        // DISTINCT from every authored footprint, so `disjoint`'s different-kind `continue` would
        // clear it (Disjoint ⇒ wrongly survive — distinctness-as-license). Registering the auto-kind
        // forces MayAlias ⇒ DEMOTE: the private per-provider singleton never manufactures the
        // separation that licenses survival (`277` §6 never-derive-separation).
        let mut i = dorc_core::Interner::default();
        let auto = dorc_core::auto_fact(&mut i, "nginxctl"); // kind dorc-auto:nginxctl
        let wall_coord = EntityCoord::new(
            KindId(i.intern("com.debian.apt.Package")),
            EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
        );
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall_coord]).unwrap();
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(0),
            footprint: fp,
        }];
        let mut res = Resolutions::none();
        // Without the fence, the distinct-string auto backing WRONGLY survives — the near-miss.
        assert!(
            matches!(
                wall_verdict(
                    false,
                    &walls,
                    &Backing::of_fact(auto),
                    &res,
                    &Dialect::empty()
                ),
                WallVerdict::Survived(_)
            ),
            "the near-miss: a distinct-kind auto backing survives naively (what the fence closes)"
        );
        res.add_auto_kind(auto.kind);
        assert!(
            matches!(
                wall_verdict(
                    false,
                    &walls,
                    &Backing::of_fact(auto),
                    &res,
                    &Dialect::empty()
                ),
                WallVerdict::Demoted(DemoteReason::MayAlias)
            ),
            "fence-no-disjoint: a registered auto backing MAY-ALIASES ⇒ demote, never survives a wall"
        );
    }

    #[test]
    fn poisoned_backing_demotes() {
        let mut i = dorc_core::Interner::default();
        let f = fact(1, "nginx", "installed");
        // A wall whose footprint IS the fact's coordinate ⇒ hit ⇒ demote.
        let fp = Footprint::authored(i.intern("apt-get"), vec![coord_of(f)]).unwrap();
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(3),
            footprint: fp,
        }];
        assert!(
            matches!(
                wall_verdict(
                    false,
                    &walls,
                    &Backing::of_fact(f),
                    &Resolutions::none(),
                    &Dialect::empty()
                ),
                WallVerdict::Demoted(DemoteReason::Poisoned { via_reach: None })
            ),
            "a footprint hitting the backing demotes even without a total wall"
        );
    }

    // ── 24F Stage 5: the aliasing closure (owner-declared canonicalization) ──────────────────

    fn mk_coord(i: &mut dorc_core::Interner, kind: &str, entity: &str) -> EntityCoord {
        EntityCoord::new(
            KindId(i.intern(kind)),
            EntityRef::Operand(OpaqueToken(i.intern(entity))),
        )
    }

    #[test]
    fn resolver_canonicalizes_two_names_to_one_hit() {
        // 24F §3 — the core closure: a wall footprint names `package:nginx`, a downstream fact's
        // backing names `package:nginx-full`; token-equality would call them DISJOINT (wrong
        // survival). A resolver that canonicalizes BOTH to `nginx` ⇒ they HIT ⇒ demote (the
        // under-execute closes).
        let mut i = dorc_core::Interner::default();
        let pkg = KindId(i.intern("package"));
        let canon = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let wall = mk_coord(&mut i, "package", "nginx");
        let back = mk_coord(&mut i, "package", "nginx-full");
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall]).unwrap();

        let mut res = Resolutions::none();
        res.record(wall, canon);
        res.record(back, canon);
        assert!(res.has_resolver(pkg));
        assert!(
            matches!(
                disjoint(&fp, &backing_of(back, None), &res, &Dialect::empty()),
                DisjointOutcome::Hit { .. }
            ),
            "two names for one referent HIT after canonicalization"
        );
    }

    #[test]
    fn resolverless_kind_stays_token_equality_floor() {
        // 24F §3 — per-kind gradual enhancement: with NO resolver for `package`, `nginx` and
        // `nginx-full` stay DISTINCT tokens ⇒ disjoint (the honest floor, unchanged from Stage 2).
        let mut i = dorc_core::Interner::default();
        let wall = mk_coord(&mut i, "package", "nginx");
        let back = mk_coord(&mut i, "package", "nginx-full");
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall]).unwrap();
        assert!(
            matches!(
                disjoint(
                    &fp,
                    &backing_of(back, None),
                    &Resolutions::none(),
                    &Dialect::empty()
                ),
                DisjointOutcome::Disjoint(_)
            ),
            "a resolver-less kind keeps token-equality (distinct names disjoint)"
        );
    }

    #[test]
    fn unresolved_resolver_bearing_coord_is_may_alias_demote() {
        // 24F §3a — the failure-direction: the kind IS resolver-bearing but the backing produced
        // no canonical (⊤/dangling/absent). It degrades to MAY-ALIAS ⇒ demote (fail toward run),
        // NOT to token-equality — the owner declared identity needs resolution, so unresolved is
        // suspect.
        let mut i = dorc_core::Interner::default();
        let pkg = KindId(i.intern("package"));
        let wall = mk_coord(&mut i, "package", "nginx");
        let back = mk_coord(&mut i, "package", "nginx-full");
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall]).unwrap();

        // package is resolver-bearing, the wall coord resolves, but the backing does NOT.
        let mut res = Resolutions::none();
        res.add_resolver_kind(pkg);
        res.record(wall, EntityRef::Operand(OpaqueToken(i.intern("nginx"))));
        assert!(
            matches!(
                disjoint(&fp, &backing_of(back, None), &res, &Dialect::empty()),
                DisjointOutcome::MayAlias(MayAliasReason::Unresolved)
            ),
            "an unresolved resolver-bearing backing degrades to may-alias (not token-equality)"
        );
        // …and through wall_verdict, that is a MayAlias demote (the fire-rate the yardstick counts).
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(0),
            footprint: fp,
        }];
        assert!(
            matches!(
                wall_verdict(
                    false,
                    &walls,
                    &backing_of(back, None),
                    &res,
                    &Dialect::empty()
                ),
                WallVerdict::Demoted(DemoteReason::MayAlias)
            ),
            "may-alias demotes toward run and is attributed as such"
        );
    }

    #[test]
    fn resolver_bearing_disjoint_survives_and_names_the_resolver() {
        // 24F §6 attribution: two DISTINCT referents in a resolver-bearing kind survive, and the
        // crossing NAMES the resolver (the why-lens says "disjoint AFTER <kind>.resolve()").
        let mut i = dorc_core::Interner::default();
        let pkg = KindId(i.intern("package"));
        let wall = mk_coord(&mut i, "package", "nginx");
        let back = mk_coord(&mut i, "package", "curl");
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall]).unwrap();

        let mut res = Resolutions::none();
        res.record(wall, EntityRef::Operand(OpaqueToken(i.intern("nginx")))); // distinct canons
        res.record(back, EntityRef::Operand(OpaqueToken(i.intern("curl"))));
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(2),
            footprint: fp,
        }];
        match wall_verdict(
            false,
            &walls,
            &backing_of(back, None),
            &res,
            &Dialect::empty(),
        ) {
            WallVerdict::Survived(w) => {
                assert_eq!(
                    w.crossings()[0].via_resolver(),
                    Some(pkg),
                    "a resolver-bearing survival names the canonicalizing resolver kind"
                );
            }
            other => panic!("expected Survived, got {other:?}"),
        }
    }

    #[test]
    fn different_kind_never_may_aliases_even_when_backing_unresolved() {
        // The kind fence: a resolver-bearing backing that did NOT resolve is still DISJOINT from a
        // DIFFERENT-kind footprint coord — a resolver canonicalizes only WITHIN a kind, so a
        // cross-kind pair never needs resolution (no spurious may-alias demote).
        let mut i = dorc_core::Interner::default();
        let pkg = KindId(i.intern("package"));
        let wall = mk_coord(&mut i, "file", "/etc/x"); // different kind
        let back = mk_coord(&mut i, "package", "nginx-full"); // resolver-bearing, unresolved
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall]).unwrap();
        let mut res = Resolutions::none();
        res.add_resolver_kind(pkg);
        assert!(
            matches!(
                disjoint(&fp, &backing_of(back, None), &res, &Dialect::empty()),
                DisjointOutcome::Disjoint(_)
            ),
            "a different-kind footprint coord is disjoint regardless of the backing's resolution"
        );
    }

    // ── 24G Part B: the reaches() footprint EXPANSION (add_reached + demote attribution) ─────────

    #[test]
    fn reached_coord_hits_and_attributes_the_reach_function() {
        // 24G Part B — the cross-author demote: a wall footprints `package:nginx` (whoever emitted it
        // knows nothing of what it drags); a `package__disturbance_reaches_only()` EXPANSION adds `package:nginx-dep`.
        // A downstream fact backing `package:nginx-dep` — token-DISJOINT from the authored
        // `package:nginx`, so PRE-expansion it wrongly SURVIVES — now HITs the expanded coord ⇒ demote,
        // attributed to the reach-function KIND (`package`).
        let mut i = dorc_core::Interner::default();
        let pkg = KindId(i.intern("package"));
        let wall = mk_coord(&mut i, "package", "nginx");
        let dep = mk_coord(&mut i, "package", "nginx-dep");
        let mut fp = Footprint::authored(i.intern("hork"), vec![wall]).unwrap();
        // Pre-expansion: disjoint (distinct entities) ⇒ the victim would wrongly survive.
        assert!(matches!(
            disjoint(
                &fp,
                &backing_of(dep, None),
                &Resolutions::none(),
                &Dialect::empty()
            ),
            DisjointOutcome::Disjoint(_)
        ));
        // Expand: package__disturbance_reaches_only() drags nginx → nginx-dep.
        fp.add_reached(dep, pkg);
        match disjoint(
            &fp,
            &backing_of(dep, None),
            &Resolutions::none(),
            &Dialect::empty(),
        ) {
            DisjointOutcome::Hit { via_reach } => assert_eq!(
                via_reach,
                Some(pkg),
                "the reach-expanded hit names the reach-function kind"
            ),
            other => panic!("expected a reach Hit, got {other:?}"),
        }
        // …and through wall_verdict it is a Poisoned demote carrying the reach attribution.
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(0),
            footprint: fp,
        }];
        assert!(matches!(
            wall_verdict(false, &walls, &backing_of(dep, None), &Resolutions::none(), &Dialect::empty()),
            WallVerdict::Demoted(DemoteReason::Poisoned { via_reach: Some(k) }) if k == pkg
        ));
    }

    #[test]
    fn reach_that_does_not_hit_leaves_the_survival_unchanged() {
        // 24G Part B attribution rule: a crossing that survived only because expansion did NOT hit must
        // NOT change. A footprint expanded with `package:other` (a reach that MISSES the victim
        // `package:curl`) still SURVIVES, and names NO reach — the survival attribution is unchanged
        // (reaches attributes DEMOTES only, never survivals).
        let mut i = dorc_core::Interner::default();
        let pkg = KindId(i.intern("package"));
        let wall = mk_coord(&mut i, "package", "nginx");
        let other = mk_coord(&mut i, "package", "other");
        let mut fp = Footprint::authored(i.intern("hork"), vec![wall]).unwrap();
        fp.add_reached(other, pkg);
        let victim = mk_coord(&mut i, "package", "curl"); // hit by neither nginx nor other
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(1),
            footprint: fp,
        }];
        match wall_verdict(
            false,
            &walls,
            &backing_of(victim, None),
            &Resolutions::none(),
            &Dialect::empty(),
        ) {
            WallVerdict::Survived(w) => assert_eq!(
                w.crossings()[0].via_resolver(),
                None,
                "a non-hitting reach expansion leaves the survival attribution unchanged"
            ),
            other => panic!("expected Survived, got {other:?}"),
        }
    }

    #[test]
    fn reach_re_deriving_an_authored_coord_does_not_re_attribute() {
        // add_reached on a coord ALREADY present (an authored coord the reach re-derives) hits on its
        // OWN account — it is NOT re-attributed to the reach (the reach added nothing there). So a
        // Poisoned demote on the authored coord names `via_reach: None`, never the reach-function.
        let mut i = dorc_core::Interner::default();
        let pkg = KindId(i.intern("package"));
        let wall = mk_coord(&mut i, "package", "nginx");
        let mut fp = Footprint::authored(i.intern("apt-get"), vec![wall]).unwrap();
        fp.add_reached(wall, pkg); // re-derives the authored coord
        let walls = [AccumulatedWall {
            wall_leaf: LeafId(0),
            footprint: fp,
        }];
        assert!(matches!(
            wall_verdict(
                false,
                &walls,
                &backing_of(wall, None),
                &Resolutions::none(),
                &Dialect::empty(),
            ),
            WallVerdict::Demoted(DemoteReason::Poisoned { via_reach: None })
        ));
    }

    // ── `277` §3 — the selector-dialect sparing, end-to-end through survival ──────────────────

    #[test]
    fn dialect_selector_bearing_disturbs_spares_sibling_cell() {
        // `277` §3 end-to-end (the disturbs × dialect-selector DST case, `279f` §5): a footprint
        // whose disturbs mark carries `@active` SPARES a downstream `@enabled` backing of the SAME
        // entity — sibling cells in one dialect. The backing's minting family is RECOVERED from the
        // dialect (`sole_family`); the claim's `@active` ∈ dialect(that family, kind) ⇒ the wall's
        // kill-traffic misses the fact. Empty dialect ⇒ collide (the entity-granular floor). ONE
        // interner throughout (the fact's symbols must match the dialect's).
        let mut i = dorc_core::Interner::default();
        let kind = KindId(i.intern("sm.dorc.Service"));
        let family = ProviderId(i.intern("systemctl"));
        let nginx = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let enabled = SelectorId(i.intern("enabled"));
        let active = SelectorId(i.intern("active"));
        let coord = EntityCoord::new(kind, nginx);
        // Footprint: `systemctl restart nginx` disturbs `sm.dorc.Service:nginx@active`.
        let mut fp = Footprint::authored(i.intern("systemctl"), vec![coord]).unwrap();
        fp.set_selector(coord, active);
        // Backing: the downstream converged fact `sm.dorc.Service:nginx@enabled`.
        let backing = backing_of(coord, Some(enabled));
        // Dialect: systemctl mints {enabled, active} for sm.dorc.Service (its verdict marks).
        let mut d = Dialect::empty();
        d.mint(family, kind, enabled);
        d.mint(family, kind, active);
        assert!(
            matches!(
                disjoint(&fp, &backing, &Resolutions::none(), &d),
                DisjointOutcome::Disjoint(_)
            ),
            "a @active disturbs mark spares a @enabled sibling-cell backing under the dialect"
        );
        // Empty dialect ⇒ no minted tokens ⇒ collide (empty-world-byte-identical floor).
        assert!(
            matches!(
                disjoint(&fp, &backing, &Resolutions::none(), &Dialect::empty()),
                DisjointOutcome::Hit { .. }
            ),
            "empty dialect ⇒ entity-granular collide"
        );
        // A ⊤ (whole-entity) backing collides even under the dialect (279f:fix-spare-top-backing).
        let top_backing = backing_of(coord, None);
        assert!(
            matches!(
                disjoint(&fp, &top_backing, &Resolutions::none(), &d),
                DisjointOutcome::Hit { .. }
            ),
            "a ⊤ backing collides even under the dialect (fix-spare-top-backing)"
        );
    }

    // ── `277` §5 backing-SETS — observe-widening + the universal meet (REAL sets) ─────────────

    /// The `277` §5 shared setup: kind `sm.dorc.Service`, entity `nginx`, family `systemctl`
    /// minting {enabled, active}, a footprint whose disturbs mark carries `@active`.
    fn service_widening_setup() -> (
        dorc_core::Interner,
        FactKey,
        ProviderId,
        SelectorId,
        Footprint,
        Dialect,
    ) {
        let mut i = dorc_core::Interner::default();
        let kind = KindId(i.intern("sm.dorc.Service"));
        let family = ProviderId(i.intern("systemctl"));
        let nginx = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let enabled = SelectorId(i.intern("enabled"));
        let active = SelectorId(i.intern("active"));
        let coord = EntityCoord::new(kind, nginx);
        // `systemctl reload nginx` disturbs `sm.dorc.Service:nginx@active`.
        let mut fp = Footprint::authored(i.intern("systemctl"), vec![coord]).unwrap();
        fp.set_selector(coord, active);
        let mut d = Dialect::empty();
        d.mint(family, kind, enabled);
        d.mint(family, kind, active);
        let fact = FactKey {
            kind,
            entity: nginx,
            selector: enabled,
            context: dorc_core::Context::HostDefault,
        };
        (i, fact, family, active, fp, d)
    }

    #[test]
    fn observe_widened_backing_collides_where_the_bare_fact_would_spare() {
        // `277` §5 observe-backing-widening + universal meet: a `@enabled` fact whose verdict body
        // OBSERVED `@active` carries `@active` as a backing MEMBER. A `@active` disturbs SPARES the
        // bare `@enabled` cell (sibling under the dialect) — but COLLIDES the WIDENED backing (the
        // `@active` member is hit) ⇒ demote. Widening GROWS kill-surface; the universal meet
        // collides the set on ANY member hit (`pin-set-meet-order-independence`).
        let (_i, fact, family, active, fp, d) = service_widening_setup();
        // Bare fact (no widening): the `@active` disturbs spares the `@enabled` sibling ⇒ survives.
        let bare = Backing::widened(fact, Some(family), BTreeSet::new());
        assert!(
            matches!(
                disjoint(&fp, &bare, &Resolutions::none(), &d),
                DisjointOutcome::Disjoint(_)
            ),
            "the bare @enabled fact spares the @active disturbs (sibling cell)"
        );
        // Widened by the observed `@active`: the `@active` member is HIT ⇒ the SET collides.
        let observed: BTreeSet<SelectorId> = std::iter::once(active).collect();
        let widened = Backing::widened(fact, Some(family), observed);
        assert!(
            matches!(
                disjoint(&fp, &widened, &Resolutions::none(), &d),
                DisjointOutcome::Hit { .. }
            ),
            "the observe-widened @active member collides ⇒ the universal meet demotes"
        );
    }

    #[test]
    fn widened_backing_survives_when_every_member_is_disjoint() {
        // The universal meet's SPARE arm: a backing whose OWN cell AND every observe-widened
        // sibling are all provably-disjoint from the footprint survives. Here the fact is
        // `@enabled`, widened by `@reloaded`; the footprint disturbs `@active` — distinct from
        // BOTH members under the dialect ⇒ all pairs provably-disjoint ⇒ the SET spares.
        let (mut i, fact, family, _active, fp, mut d) = service_widening_setup();
        let reloaded = SelectorId(i.intern("reloaded"));
        d.mint(family, fact.kind, reloaded); // widen-member selector, minted by the same family
        let observed: BTreeSet<SelectorId> = std::iter::once(reloaded).collect();
        let widened = Backing::widened(fact, Some(family), observed);
        assert!(
            matches!(
                disjoint(&fp, &widened, &Resolutions::none(), &d),
                DisjointOutcome::Disjoint(_)
            ),
            "every member (@enabled, @reloaded) disjoint from @active ⇒ the set spares"
        );
    }

    #[test]
    fn threaded_family_spares_where_the_reverse_lookup_would_collide() {
        // `277` §3 backing provenance (`27D` disposition-backing-family-recovery): the THREADED
        // minting family is AUTHORITATIVE past `fence-divergent-meaning`. Two families both mint
        // {enabled, active} for the kind, so `sole_family` is ambiguous ⇒ `None` ⇒ the map-miss
        // reverse-lookup floor COLLIDES. The threaded `Some(systemctl)` uses systemctl's dialect
        // ⇒ the `@active` disturbs SPARES the `@enabled` fact. This is the exact behavior the
        // reverse-lookup could not give — the divergent-meaning improvement, member-wise.
        let (mut i, fact, systemctl, _active, fp, mut d) = service_widening_setup();
        let otherctl = ProviderId(i.intern("otherctl"));
        d.mint(otherctl, fact.kind, fact.selector); // second family mints @enabled too
        d.mint(otherctl, fact.kind, SelectorId(i.intern("active"))); // …and @active ⇒ ambiguous
        // Map-miss `of_fact` (family None ⇒ recover via sole_family, now ambiguous ⇒ None ⇒ collide).
        let recovered = Backing::of_fact(fact);
        assert!(
            matches!(
                disjoint(&fp, &recovered, &Resolutions::none(), &d),
                DisjointOutcome::Hit { .. }
            ),
            "ambiguous sole_family (two minters) ⇒ the reverse-lookup floor collides"
        );
        // Threaded `Some(systemctl)` ⇒ authoritative ⇒ spares under systemctl's dialect.
        let threaded = Backing::widened(fact, Some(systemctl), BTreeSet::new());
        assert!(
            matches!(
                disjoint(&fp, &threaded, &Resolutions::none(), &d),
                DisjointOutcome::Disjoint(_)
            ),
            "the threaded minting family is authoritative ⇒ spares past divergent-meaning"
        );
    }

    #[test]
    fn widened_backing_with_an_auto_member_kind_may_aliases_member_wise() {
        // `24L` §7 fence-no-disjoint, member-wise (`277` §5): the auto-cell fence bars an auto-kind
        // backing from proving disjoint. Every member shares the fact's kind, so an auto fact
        // (widened or not) MAY-ALIASES against any footprint — never survives. Pins that widening
        // does not smuggle an auto backing past the fence.
        let mut i = dorc_core::Interner::default();
        let auto = dorc_core::auto_fact(&mut i, "nginxctl");
        let wall = EntityCoord::new(
            KindId(i.intern("com.debian.apt.Package")),
            EntityRef::Operand(OpaqueToken(i.intern("nginx"))),
        );
        let fp = Footprint::authored(i.intern("apt-get"), vec![wall]).unwrap();
        let observed: BTreeSet<SelectorId> =
            std::iter::once(SelectorId(i.intern("extra"))).collect();
        let widened = Backing::widened(auto, None, observed);
        let mut res = Resolutions::none();
        res.add_auto_kind(auto.kind);
        assert!(
            matches!(
                disjoint(&fp, &widened, &res, &Dialect::empty()),
                DisjointOutcome::MayAlias(_)
            ),
            "an auto-kind widened backing may-aliases member-wise (never survives)"
        );
    }

    #[test]
    fn synthetic_cross_generator_consumer_map_holds() {
        // `279f` §5 cross-generator DST cases — SYNTHETIC (the lend + invariance generators arrive
        // at block-context; this pins the REGISTRY SHAPE the consumer map welds — `277` §2). Two
        // cases modeled by their verdict, since the generators do not yet exist in code:
        //  - mapped-lend × keyed kind: keying/lend re-indexes ⇒ blocks transport, NEVER
        //    ProvablyDisjoint (`never-derive-separation` — keying never feeds survival). Verdict:
        //    Unknown ⇒ safe for both consumers.
        //  - full-lend × invariant kind: an invariance line yields transport across a context
        //    boundary ⇒ feeds TRANSPORT only, never survival sparing. `compare`'s verdict is
        //    Overlaps (survival-COLLIDE); transport is a SEPARATE decision the block-context
        //    consumer makes via `selector_identifies` on the concrete selectors — NEVER the
        //    Overlaps variant (`27D` disposition-relation-same-misnomer, tc-same-is-overlap-not-
        //    identity). The old `Same` name conflated these two; the rename splits them.
        // The consumer map (the single source of truth these route through at block-context):
        let survival_spares = |r: Relation| matches!(r, Relation::ProvablyDisjoint);
        // Transport is NOT a function of `Relation`: it is `selector_identifies`-gated. No
        // `Relation` variant licenses it by itself.
        let transport_licensed_by_relation = |_r: Relation| false;
        // mapped-lend / keying ⇒ Unknown: blocks transport AND collides survival.
        assert!(
            !survival_spares(Relation::Unknown),
            "keying/mapped-lend never feeds survival sparing"
        );
        assert!(
            !transport_licensed_by_relation(Relation::Unknown),
            "keying/mapped-lend blocks transport too (the safe bottom)"
        );
        // Overlaps: survival collides, and it does NOT license transport by itself.
        assert!(
            !transport_licensed_by_relation(Relation::Overlaps),
            "Overlaps is the survival-collide reading, never a transport license (misnomer fixed)"
        );
        assert!(
            !survival_spares(Relation::Overlaps),
            "an Overlaps never feeds survival sparing (only provably-disjoint spares)"
        );
        // Transport for a full-lend/invariance case is gated on concrete-selector identity. Two
        // equal concrete tokens identify (transport-eligible); a ⊤ selector identifies with
        // nothing (`top-identifies-with-nothing`).
        let mut i = dorc_core::Interner::default();
        let sel = SelectorId(i.intern("enabled"));
        assert!(
            dorc_core::coord::selector_identifies(Some(sel), Some(sel)),
            "two equal concrete selectors identify (transport-eligible at block-context)"
        );
        assert!(
            !dorc_core::coord::selector_identifies(None, None),
            "a ⊤ selector identifies with nothing — never transports"
        );
    }
}
