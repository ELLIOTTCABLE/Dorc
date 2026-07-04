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

use std::collections::BTreeMap;

use dorc_analysis::cfg::CfgNodeId;
use dorc_core::{EntityRef, FactKey, KindId, Symbol};

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
    origin: FootprintOrigin,
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
            origin,
        })
    }

    /// The wall's provider (attribution: whose footprint licensed a crossing).
    #[must_use]
    pub fn provider(&self) -> Symbol {
        self.provider
    }

    /// The claimed coordinates (attribution render; the disjointness test).
    #[must_use]
    pub fn coords(&self) -> &[EntityCoord] {
        &self.coords
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Backing {
    coord: EntityCoord,
}

impl Backing {
    /// The backing of a fact: its own coordinate, selector dropped (entity-granular). This is
    /// the ONLY construction — a backing is always "where THIS fact's truth lives", never a
    /// wider claim.
    #[must_use]
    pub fn of_fact(fact: FactKey) -> Self {
        Self {
            coord: EntityCoord {
                kind: fact.kind,
                entity: fact.entity,
            },
        }
    }

    /// The backed coordinate (attribution render).
    #[must_use]
    pub fn coord(self) -> EntityCoord {
        self.coord
    }
}

/// A witness that a [`Footprint`] does not touch a [`Backing`]'s coordinate — minted ONLY by
/// [`disjoint`] (private field, no other constructor, TC-3). Its existence is the proof; it
/// records the coordinate it cleared (for provenance).
#[derive(Debug, Clone, Copy)]
pub struct DisjointnessProof {
    backing: EntityCoord,
}

impl DisjointnessProof {
    /// The coordinate proven clear of the footprint (attribution render).
    #[must_use]
    pub fn backing(self) -> EntityCoord {
        self.backing
    }
}

/// The one asymmetric intersection test (TC-2): does `footprint` leave `backing` untouched?
/// `Some(proof)` iff NO coordinate in the footprint matches the backing (entity-granular —
/// same `(kind, entity)` is a hit; a different entity or kind is disjoint). `None` iff a hit
/// (the fact's backing is poisoned ⇒ the elision cannot survive). The argument ORDER is
/// load-bearing and type-enforced: `disjoint(&Backing, &Footprint)` does not compile, so a
/// caller cannot silently swap "what was written" for "what is read".
#[must_use]
pub fn disjoint(footprint: &Footprint, backing: &Backing) -> Option<DisjointnessProof> {
    // Entity-granular: compare (kind, entity), never the fact's selector.
    let hit = footprint.coords.contains(&backing.coord);
    (!hit).then_some(DisjointnessProof {
        backing: backing.coord,
    })
}

/// One record of a downstream elision crossing one running wall (TC-3 attribution): which
/// wall (its leaf), whose footprint licensed it (provider + the claimed coordinates), and the
/// disjointness proof. Constructed only inside [`wall_verdict`].
#[derive(Debug, Clone)]
pub struct Crossing {
    wall_leaf: LeafId,
    provider: Symbol,
    footprint: Vec<EntityCoord>,
    origin: FootprintOrigin,
    proof: DisjointnessProof,
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

    /// The disjointness proof this crossing rests on.
    #[must_use]
    pub fn proof(&self) -> DisjointnessProof {
        self.proof
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
    /// A total wall stands, or the backing hit some wall's footprint — demote to `Run`.
    Demoted,
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
) -> WallVerdict {
    if total_wall {
        return WallVerdict::Demoted;
    }
    let mut crossings = Vec::new();
    for wall in walls {
        match disjoint(&wall.footprint, backing) {
            Some(proof) => crossings.push(Crossing {
                wall_leaf: wall.wall_leaf,
                provider: wall.footprint.provider(),
                footprint: wall.footprint.coords().to_vec(),
                origin: wall.footprint.origin().clone(),
                proof,
            }),
            None => return WallVerdict::Demoted, // the backing is poisoned ⇒ run for real
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
        }
    }

    fn coord_of(fact: FactKey) -> EntityCoord {
        EntityCoord::new(fact.kind, fact.entity)
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
        let backing = Backing {
            coord: EntityCoord::new(KindId(i.intern("pkgindex")), EntityRef::Singleton),
        };
        assert!(
            disjoint(&fp, &backing).is_some(),
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
        // backing on the SAME entity but a different selector (#configured vs the footprint's
        // entity-granular claim) ⇒ hit.
        let backing = Backing {
            coord: EntityCoord::new(k, e),
        };
        assert!(disjoint(&fp, &backing).is_none(), "same entity is a hit");
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
    fn total_wall_demotes_even_when_disjoint() {
        let f = fact(1, "nginx", "installed");
        let verdict = wall_verdict(true, &[], &Backing::of_fact(f));
        assert!(
            matches!(verdict, WallVerdict::Demoted),
            "total wall demotes"
        );
    }

    #[test]
    fn no_walls_survives_clean() {
        let f = fact(1, "nginx", "installed");
        let verdict = wall_verdict(false, &[], &Backing::of_fact(f));
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
        match wall_verdict(false, &walls, &Backing::of_fact(f)) {
            WallVerdict::Survived(w) => {
                assert_eq!(w.crossings().len(), 1, "one crossing recorded");
                assert_eq!(w.crossings()[0].wall_leaf(), LeafId(0));
            }
            other => panic!("expected Survived, got {other:?}"),
        }
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
                wall_verdict(false, &walls, &Backing::of_fact(f)),
                WallVerdict::Demoted
            ),
            "a footprint hitting the backing demotes even without a total wall"
        );
    }
}
