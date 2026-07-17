//! `core::coord` — the coordinate comparison vocabulary (`notes/277` §§1–3): the context
//! slot, the ternary relation, the selector dialect, and the two comparison chokepoints
//! (`selector_covers` + `compare`). This is the `dac-B` shared vocabulary every crate agrees
//! on first — the entity-algebra's chokepoints live here so no consumer ever compares
//! coordinate axes inline (`inv-referent-agnostic` + `271:rul-seam-context-slot-and-relational-chokepoint`).
//!
//! # The two chokepoints (the whole design routes through these)
//!
//! * [`selector_covers`] — the ONE place selector tokens compare (`277` §1 selector-chokepoint).
//!   Answers the SURVIVAL question: does a disturbance carrying `claim` reach (collide with) a
//!   fact cell `backing`? The dialect algebra (`277` §3) lives entirely inside it.
//! * [`compare`] — the ONE whole-coordinate chokepoint (`277` §2). Answers the ternary relation
//!   `{ overlaps | provably-disjoint | unknown }` with the consumer map welded: *provably-disjoint*
//!   → survival sparing only (flag-gated), *overlaps* → survival collide, *unknown* → the safe
//!   bottom for both. Transport-grade sameness is NEVER the [`Relation::Overlaps`] variant — it is
//!   `selector_identifies`-gated at the transport consumer (block-context). It MAY answer
//!   relationally — per-axis pointwise decomposition is never baked into the API
//!   (`271:rul-seam-context-slot-and-relational-chokepoint`).
//!
//! # What lives here vs. `plan::survival`
//!
//! The resolve generator ([`plan::survival::Resolutions`]) canonicalizes the ENTITY within its
//! kind and stays in `plan` (it carries the `CanonicalCoord` private-mint + the auto-cell fence);
//! it feeds [`compare`] its canonicalized OUTPUT as an [`EntityResolution`] (the `277` §2
//! generator-registry model: each generator feeds the chokepoint its licensed evidence, the
//! engine mints the license). So a raw coordinate still cannot reach the intersection in a
//! resolver-bearing kind — the caller canonicalizes through `Resolutions` before it can call
//! `compare`.

use std::collections::{BTreeMap, BTreeSet};

use crate::{EntityRef, KindId, ProviderId, SelectorId};

/// The world-qualifier slot of a coordinate (`277` §1). At v1 the sole value is the host-default
/// world; its NAME is deliberately unminted (`277` §1 — no hostname pre-design, `~SUSPECT` it ends
/// up `<hostname>`-root-ish when the multi-host round lands). Populated by NOTHING yet — the
/// wrapper machinery (`273`/block-context) fills it. An opaque default variant reserves the
/// representation room (`277` §5 `seam-context-qualifier-slot`, né `24S:A7`(i)); the relational
/// [`compare`] consumes it, so the fork between a space-tag and a qualifier field already dissolved
/// (`271:rul-seam-context-slot-and-relational-chokepoint`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Context {
    /// The host-default world — the only context at v1 (name unminted).
    #[default]
    HostDefault,
}

/// A whole coordinate as the comparison chokepoint sees it (`277` §1): the flat three-place
/// `(kind, entity, selector)` carried in a representation that ALSO holds the [`Context`] slot.
/// The selector is [`Option`]: `None` is the bare selector-less / ⊤ form (permanently "whole
/// entity"; collides with every cell — `277` §1).
///
/// This is the `277` §1 "one representation everywhere a coordinate appears" at the COMPARISON
/// boundary. A [`crate::FactKey`] injects a `HostDefault` context and a `Some(selector)` when it
/// becomes a `Coord` ([`Coord::of_fact`]); a footprint (disturbs) coordinate injects the
/// emission's selector (`None` for a whole-entity emission). The `FactKey`-LEVEL context slot is
/// reserved for when the wrapper machinery populates it (block-context) — until then every fact
/// is `HostDefault`, so keying is unaffected (`inv-determinism`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    /// The kind (reverse-DNS; the vocabulary fence — resolvers canonicalize entities WITHIN a
    /// kind, never across).
    pub kind: KindId,
    /// The entity (canonicalized by the resolve generator before [`compare`] sees the result).
    pub entity: EntityRef,
    /// The selector cell, or `None` for the whole-entity / ⊤ form (`277` §1). At a consumer a
    /// `None` collides with every cell of the entity (`top-identifies-with-nothing`).
    pub selector: Option<SelectorId>,
    /// The world-qualifier (default `HostDefault` at v1).
    pub context: Context,
}

impl Coord {
    /// The comparison-view of a fact: its `(kind, entity, selector)` with a concrete selector and
    /// the default context (`277` §1). Facts always carry a minted selector, so the selector is
    /// `Some`.
    #[must_use]
    pub fn of_fact(fact: crate::FactKey) -> Self {
        Self {
            kind: fact.kind,
            entity: fact.entity,
            selector: Some(fact.selector),
            context: Context::HostDefault,
        }
    }

    /// A coordinate from its axes, at the default context: `(kind, entity)` plus a selector cell
    /// (`None` for the whole-entity / ⊤ form — `rul-emission-selector-on-mark`). Used for footprint
    /// (disturbs-emission) coordinates and for re-viewing an entity-granular backing with its
    /// selector.
    #[must_use]
    pub fn new(kind: KindId, entity: EntityRef, selector: Option<SelectorId>) -> Self {
        Self {
            kind,
            entity,
            selector,
            context: Context::HostDefault,
        }
    }
}

/// The output of the resolve generator (`plan::survival::Resolutions::canonicalize`) as the
/// [`compare`] chokepoint consumes it (`277` §2 generator registry): the ENTITY canonicalized
/// within its kind, or unresolvable. Selectors do NOT canonicalize at v1 (`277` §1). Kept a core
/// type so `compare` can live here without pulling the plan-side resolution machinery down; the
/// caller (survival) canonicalizes through `Resolutions` (the `CanonicalCoord` private-mint) and
/// passes the result — a raw coordinate cannot reach `compare`'s `same`/`disjoint` verdicts
/// without first passing through the owner's resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityResolution {
    /// The resolver produced a clean canonical entity (or the kind is resolver-less ⇒ identity).
    Canonical(EntityRef),
    /// The resolver could not canonicalize it (`24F` §3a: ⊤ / dangling / absent in a
    /// resolver-bearing kind) ⇒ the chokepoint answers [`Relation::Unknown`] (fail toward run).
    Unresolvable,
}

/// The ternary coordinate relation (`277` §2 — the one comparison, everywhere). The consumer map
/// is WELDED (`ternary-compare-consumer-map`): each verdict feeds exactly one consumer, and
/// `Unknown` is the safe bottom for both. It is ternary because of the safety inversion (`273`
/// §4): believed-no-overlap is safe for transport and dangerous for kill-traffic, and vice versa
/// — no binary default is safe for both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    /// The two coordinates are NOT provably cell-separable — they name overlapping cells (the
    /// same cell, a ⊤/whole-entity coordinate over a cell, or two unminted-selector siblings the
    /// dialect could not spare). The overlap-honest name (`27D`
    /// disposition-relation-same-misnomer): this variant is the survival-COLLIDE reading, NOT a
    /// cell-identity assertion — the misnomer `Same` invited exactly that misread.
    ///
    /// Feeds survival as COLLIDE (the wall's disturbance reaches the backing). It does NOT license
    /// transport: transport-grade sameness is `selector_identifies`-gated (`top-identifies-with-
    /// nothing`: a ⊤/whole-entity coordinate OVERLAPS a cell without being *identical* to it, and
    /// two distinct unminted tokens overlap without identifying), and is NEVER this variant. A
    /// transport consumer at block-context re-checks concrete-selector identity through
    /// [`selector_identifies`]; `Overlaps` alone asserts not-separable, never same-referent.
    Overlaps,
    /// The two coordinates are PROVABLY disjoint — a different kind (the movable kind-fence), a
    /// different entity within one kind, or a dialect-spared selector (`277` §3). Feeds survival
    /// SPARING only, consumed under the flag (`rul-flag-is-razor-residue`); NEVER transport.
    ProvablyDisjoint,
    /// The safe bottom (`277` §2): no transport, and collide/run for survival. Safe for BOTH
    /// consumers — a resolver gap (`24F` §3a) or a cross-context pair (keying blocks transport,
    /// never separates — `never-derive-separation`).
    Unknown,
}

/// The selector dialect (`277` §3): per `(family, kind)`, the set of selector tokens that
/// family's VERDICT/OBSERVE marks minted for that kind. `dialect(family, kind)` is that family's
/// declared kill-surface control (`279f` delta-kill-surface-blessing); there is no global per-kind
/// vocabulary. Family per `271:rul-family` — name-derived (a command [`ProviderId`]), never file-
/// or author-derived.
///
/// Claim/disturbs emissions NEVER mint (`sparing-algebra`); only verdict (`:`/`:!`) and observe
/// (`:?`) marks do. The engine cost the comparison pays (`277` §3): these per-`(kind × family)`
/// sets + the backing's minting family carried into [`compare`].
#[derive(Debug, Clone, Default)]
pub struct Dialect {
    minted: BTreeMap<(ProviderId, KindId), BTreeSet<SelectorId>>,
}

impl Dialect {
    /// The empty dialect — the empty world (`empty-world-byte-identical`): with nothing minted,
    /// [`selector_covers`] never spares, so the comparison is entity-granular, byte-identical to
    /// the pre-dialect HEAD.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Record that `family`'s verdict/observe mark minted `selector` for `kind` (`277` §3
    /// minting). Idempotent; the set grows only by authored marks parsed at oracle-read
    /// (`fence-divergent-meaning` — a host can never mint a selector at runtime).
    pub fn mint(&mut self, family: ProviderId, kind: KindId, selector: SelectorId) {
        self.minted
            .entry((family, kind))
            .or_default()
            .insert(selector);
    }

    /// `dialect(family, kind)` — the minted tokens for one `(family, kind)`; empty when the family
    /// minted nothing for the kind. The set [`compare`] hands [`selector_covers`].
    #[must_use]
    pub fn tokens(&self, family: Option<ProviderId>, kind: KindId) -> &BTreeSet<SelectorId> {
        static EMPTY: BTreeSet<SelectorId> = BTreeSet::new();
        family
            .and_then(|f| self.minted.get(&(f, kind)))
            .unwrap_or(&EMPTY)
    }

    /// The families that minted `selector` for `kind` (`277` §3 — the backing provenance recovery
    /// when it is not threaded on the fact). Exactly one family ⇒ the backing's minting family;
    /// zero or several ⇒ ambiguous (the several case is `fence-divergent-meaning`, resolved to the
    /// safe floor `None` ⇒ no sparing). Deterministic order (`inv-determinism`).
    #[must_use]
    pub fn families_minting(&self, kind: KindId, selector: SelectorId) -> Vec<ProviderId> {
        self.minted
            .iter()
            .filter(|((_, k), sels)| *k == kind && sels.contains(&selector))
            .map(|((f, _), _)| *f)
            .collect()
    }

    /// Recover the SOLE minting family of `(kind, selector)`, or `None` if zero or several minted
    /// it (`277` §3 backing provenance; the several case is `fence-divergent-meaning` ⇒ safe floor
    /// `None` ⇒ collide). Used where the fact does not carry threaded provenance.
    #[must_use]
    pub fn sole_family(&self, kind: KindId, selector: SelectorId) -> Option<ProviderId> {
        match self.families_minting(kind, selector).as_slice() {
            [only] => Some(*only),
            _ => None,
        }
    }
}

/// The selector chokepoint (`277` §1 selector-chokepoint / §3 sparing-algebra) — the ONE place
/// selector tokens compare. Answers the SURVIVAL question: does a disturbance carrying `claim`
/// COVER (collide with) the fact cell `backing`? `true` = collides (the conservative floor);
/// `false` = SPARES (the dialect refinement, consumed only under the survival flag).
///
/// SPARES iff BOTH sides carry MINTED selectors (∈ `dialect`) AND `claim ≠ backing` (`277` §3 as
/// amended by `279f:fix-spare-top-backing`). Everything else COLLIDES: a ⊤/selector-less
/// coordinate on EITHER side (`None`), an unminted token (∉ `dialect`), a cross-dialect token (∉
/// THIS dialect), or `claim == backing`. An empty `dialect` ⇒ never spares ⇒ entity-granular
/// (`empty-world-byte-identical`).
///
/// The regression pin (`279f:fix-spare-top-backing`): a ⊤ backing (`backing == None`) collides —
/// the pre-amendment bug spared whole-entity backings, an under-execution path (`inv-must-may`).
#[must_use]
pub fn selector_covers(
    claim: Option<SelectorId>,
    backing: Option<SelectorId>,
    dialect: &BTreeSet<SelectorId>,
) -> bool {
    match (claim, backing) {
        // SPARE iff both minted (∈ dialect) and distinct; else COLLIDE.
        (Some(c), Some(b)) => !(c != b && dialect.contains(&c) && dialect.contains(&b)),
        // ⊤/selector-less on either side collides with every cell (top-identifies-with-nothing;
        // 279f:fix-spare-top-backing — the backing side especially).
        _ => true,
    }
}

/// The transport-direction selector primitive (`top-identifies-with-nothing`): do `a` and `b`
/// name the SAME cell for IDENTITY (transport)? `true` iff both are the same concrete token. A ⊤
/// coordinate (`None`) identifies with NOTHING, including itself — it overlaps without being
/// identical, so it never transports. Built now, consumed by the transport consumer at
/// block-context (`277` §2 consumer map); `compare`'s [`Relation::Overlaps`] is the survival-collide
/// reading and does NOT assert identity (a transport consumer re-checks through this).
#[must_use]
pub fn selector_identifies(a: Option<SelectorId>, b: Option<SelectorId>) -> bool {
    matches!((a, b), (Some(x), Some(y)) if x == y)
}

/// The whole-coordinate chokepoint (`277` §2) — the ONE place a coordinate PAIR compares, minting
/// the ternary [`Relation`]. Composes the axes in order: context, the movable kind-fence,
/// entity-canonicalization (fed in as [`EntityResolution`] from the resolve generator), then the
/// selector dialect. It MAY answer relationally (`271:rul-seam-context-slot-and-relational-chokepoint`).
///
/// `claim` is the disturbance/footprint side, `backing` the fact side (the dialect is keyed by the
/// BACKING's minting family — `277` §3). The consumer maps the verdict: *provably-disjoint* →
/// spare, {*same*, *unknown*} → collide, with the survival demote-reason (`Poisoned` vs
/// `MayAlias`) derived by the caller from the canonicalization outcome (byte-identity —
/// `may_alias_fires` is digest-exempt).
///
/// No generator ever produces cross-kind *same* (`top-identifies-with-nothing`; the co-reference
/// mechanism is parked behind the MOVABLE kind-fence — `24C:strain-coreference-crosskind`).
#[must_use]
pub fn compare(
    claim: Coord,
    backing: Coord,
    claim_canon: EntityResolution,
    backing_canon: EntityResolution,
    dialect: &Dialect,
    backing_family: Option<ProviderId>,
) -> Relation {
    // Context: a mismatch is the safe bottom (keying blocks transport, never separates —
    // never-derive-separation). At v1 every context is HostDefault, so this branch is dormant;
    // it reserves the behavior for block-context. Checked BEFORE the kind-fence: two coordinates
    // in DIFFERENT worlds are not "provably disjoint cells" (they may be the same cell keyed
    // twice), so the ProvablyDisjoint short-circuit below must not fire across a context gap.
    if claim.context != backing.context {
        return Relation::Unknown;
    }
    // The movable kind-fence (`277` §1/§6): cross-kind pairs short-circuit disjoint BEFORE
    // canonicalization. Kept MOVABLE for the parked co-reference mechanism
    // (`24C:strain-coreference-crosskind`) — no generator produces cross-kind *same* at v1.
    if claim.kind != backing.kind {
        return Relation::ProvablyDisjoint;
    }
    // Entity canonicalization within the kind (the resolve generator's output). A resolver gap on
    // EITHER side ⇒ Unknown (`24F` §3a — fail toward run); selectors do NOT canonicalize at v1.
    let (EntityResolution::Canonical(ca), EntityResolution::Canonical(cb)) =
        (claim_canon, backing_canon)
    else {
        return Relation::Unknown;
    };
    if ca != cb {
        // Distinct canonical entities in one kind ⇒ provably disjoint (no resolver crosses).
        return Relation::ProvablyDisjoint;
    }
    // Same (kind, canonical entity): the selector chokepoint decides, keyed by the backing's
    // minting family's dialect (`277` §3).
    let tokens = dialect.tokens(backing_family, backing.kind);
    if selector_covers(claim.selector, backing.selector, tokens) {
        Relation::Overlaps
    } else {
        Relation::ProvablyDisjoint
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Interner, OpaqueToken};

    fn sel(i: &mut Interner, s: &str) -> SelectorId {
        SelectorId(i.intern(s))
    }

    fn dialect_of(i: &mut Interner, family: ProviderId, kind: KindId, tokens: &[&str]) -> Dialect {
        let mut d = Dialect::empty();
        for t in tokens {
            let s = sel(i, t);
            d.mint(family, kind, s);
        }
        d
    }

    // ── selector_covers: the §3 sparing algebra ────────────────────────────────────────────

    #[test]
    fn selector_covers_spares_only_two_minted_distinct_tokens() {
        // §3: a claim SPARES a backing iff BOTH carry minted selectors AND claim ≠ backing.
        let mut i = Interner::default();
        let enabled = sel(&mut i, "enabled");
        let active = sel(&mut i, "active");
        let d: BTreeSet<SelectorId> = [enabled, active].into_iter().collect();
        // Both minted, distinct ⇒ SPARES (covers == false).
        assert!(!selector_covers(Some(active), Some(enabled), &d));
        // Same token ⇒ COLLIDES (the same cell — no self-sparing).
        assert!(selector_covers(Some(enabled), Some(enabled), &d));
    }

    #[test]
    fn selector_covers_top_on_either_side_collides() {
        // 279f:fix-spare-top-backing — a ⊤/selector-less coordinate on EITHER side collides. The
        // backing side especially (the pre-amendment under-execution bug spared whole-entity
        // backings).
        let mut i = Interner::default();
        let enabled = sel(&mut i, "enabled");
        let active = sel(&mut i, "active");
        let d: BTreeSet<SelectorId> = [enabled, active].into_iter().collect();
        assert!(selector_covers(None, Some(enabled), &d), "⊤ claim collides");
        assert!(
            selector_covers(Some(active), None, &d),
            "⊤ backing collides (the 279f regression pin)"
        );
        assert!(selector_covers(None, None, &d), "⊤ vs ⊤ collides");
    }

    #[test]
    fn selector_covers_unminted_and_cross_dialect_collide() {
        // §3: everything but two-minted-distinct collides — unminted tokens and cross-dialect
        // tokens are ⊤-selector.
        let mut i = Interner::default();
        let enabled = sel(&mut i, "enabled");
        let active = sel(&mut i, "active");
        let ghost = sel(&mut i, "ghost"); // never minted
        let d: BTreeSet<SelectorId> = [enabled, active].into_iter().collect();
        assert!(
            selector_covers(Some(ghost), Some(enabled), &d),
            "unminted claim collides"
        );
        assert!(
            selector_covers(Some(active), Some(ghost), &d),
            "unminted backing collides"
        );
    }

    #[test]
    fn selector_covers_empty_dialect_never_spares() {
        // empty-world-byte-identical: no dialect ⇒ never spares ⇒ entity-granular collide.
        let mut i = Interner::default();
        let enabled = sel(&mut i, "enabled");
        let active = sel(&mut i, "active");
        let empty = BTreeSet::new();
        assert!(selector_covers(Some(active), Some(enabled), &empty));
    }

    // ── selector_identifies: the transport primitive (top-identifies-with-nothing) ──────────

    #[test]
    fn selector_identifies_top_identifies_with_nothing_including_itself() {
        let mut i = Interner::default();
        let enabled = sel(&mut i, "enabled");
        assert!(
            selector_identifies(Some(enabled), Some(enabled)),
            "same token identifies"
        );
        assert!(
            !selector_identifies(None, Some(enabled)),
            "⊤ never identifies"
        );
        assert!(
            !selector_identifies(Some(enabled), None),
            "⊤ never identifies"
        );
        assert!(
            !selector_identifies(None, None),
            "⊤ never identifies itself"
        );
    }

    // ── compare: the whole-coordinate ternary chokepoint ────────────────────────────────────

    fn coord(kind: KindId, entity: EntityRef, selector: Option<SelectorId>) -> Coord {
        Coord::new(kind, entity, selector)
    }

    #[test]
    fn compare_different_kind_is_provably_disjoint() {
        // The movable kind-fence: cross-kind pairs short-circuit disjoint before canonicalization.
        let mut i = Interner::default();
        let ka = KindId(i.intern("com.a.K"));
        let kb = KindId(i.intern("com.b.K"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("x")));
        let a = coord(ka, e, None);
        let b = coord(kb, e, None);
        assert_eq!(
            compare(
                a,
                b,
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &Dialect::empty(),
                None,
            ),
            Relation::ProvablyDisjoint
        );
    }

    #[test]
    fn compare_distinct_entity_same_kind_is_provably_disjoint() {
        let mut i = Interner::default();
        let k = KindId(i.intern("com.a.K"));
        let ea = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let eb = EntityRef::Operand(OpaqueToken(i.intern("curl")));
        assert_eq!(
            compare(
                coord(k, ea, None),
                coord(k, eb, None),
                EntityResolution::Canonical(ea),
                EntityResolution::Canonical(eb),
                &Dialect::empty(),
                None,
            ),
            Relation::ProvablyDisjoint
        );
    }

    #[test]
    fn compare_unresolvable_either_side_is_unknown() {
        // 24F §3a: a resolver gap on either side ⇒ the safe bottom.
        let mut i = Interner::default();
        let k = KindId(i.intern("com.a.K"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("x")));
        assert_eq!(
            compare(
                coord(k, e, None),
                coord(k, e, None),
                EntityResolution::Unresolvable,
                EntityResolution::Canonical(e),
                &Dialect::empty(),
                None,
            ),
            Relation::Unknown
        );
        assert_eq!(
            compare(
                coord(k, e, None),
                coord(k, e, None),
                EntityResolution::Canonical(e),
                EntityResolution::Unresolvable,
                &Dialect::empty(),
                None,
            ),
            Relation::Unknown
        );
    }

    #[test]
    fn compare_same_entity_no_dialect_is_same_collide() {
        // empty-world-byte-identical: same (kind, canon entity), no dialect ⇒ Overlaps (collide),
        // whatever the selectors — the entity-granular floor.
        let mut i = Interner::default();
        let k = KindId(i.intern("com.a.K"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("x")));
        let s = sel(&mut i, "installed");
        // ⊤ footprint vs concrete backing (the survive-fixture shape) ⇒ Overlaps (Poisoned).
        assert_eq!(
            compare(
                coord(k, e, None),
                coord(k, e, Some(s)),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &Dialect::empty(),
                None,
            ),
            Relation::Overlaps
        );
    }

    #[test]
    fn compare_same_entity_dialect_spares_is_provably_disjoint() {
        // §3: same (kind, canon entity), two minted distinct selectors ⇒ the disturbance MISSES
        // this cell ⇒ provably-disjoint (the selector-granular sparing).
        let mut i = Interner::default();
        let k = KindId(i.intern("sm.dorc.Service"));
        let family = ProviderId(i.intern("systemctl"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let enabled = sel(&mut i, "enabled");
        let active = sel(&mut i, "active");
        let d = dialect_of(&mut i, family, k, &["enabled", "active"]);
        // claim disturbs #active, backing fact is #enabled ⇒ spared.
        assert_eq!(
            compare(
                coord(k, e, Some(active)),
                coord(k, e, Some(enabled)),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &d,
                Some(family),
            ),
            Relation::ProvablyDisjoint
        );
        // But a ⊤ backing collides even with a minted claim (279f:fix-spare-top-backing).
        assert_eq!(
            compare(
                coord(k, e, Some(active)),
                coord(k, e, None),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &d,
                Some(family),
            ),
            Relation::Overlaps
        );
    }

    #[test]
    fn compare_cross_family_monotone_no_foreign_dialect_sparing() {
        // §3 cross-family monotone: a claim minted by ANOTHER family never spares against this
        // family's backing (the dialect is keyed by the BACKING's family). Loading `otherctl`'s
        // dialect does not alter comparisons against `systemctl`'s backings.
        let mut i = Interner::default();
        let k = KindId(i.intern("sm.dorc.Service"));
        let systemctl = ProviderId(i.intern("systemctl"));
        let otherctl = ProviderId(i.intern("otherctl"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let enabled = sel(&mut i, "enabled");
        let active = sel(&mut i, "active");
        // systemctl mints {enabled, active}; otherctl mints only {active}.
        let mut d = dialect_of(&mut i, systemctl, k, &["enabled", "active"]);
        d.mint(otherctl, k, active);
        // Backing family = systemctl ⇒ dialect(systemctl) has both ⇒ spare.
        assert_eq!(
            compare(
                coord(k, e, Some(active)),
                coord(k, e, Some(enabled)),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &d,
                Some(systemctl),
            ),
            Relation::ProvablyDisjoint
        );
        // Backing family = otherctl ⇒ dialect(otherctl) lacks `enabled` ⇒ collide (no sparing
        // against another family's minted-elsewhere token — cross-family monotone).
        assert_eq!(
            compare(
                coord(k, e, Some(active)),
                coord(k, e, Some(enabled)),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &d,
                Some(otherctl),
            ),
            Relation::Overlaps
        );
    }

    #[test]
    fn compare_no_backing_family_never_spares() {
        // Backing provenance absent (family None) ⇒ empty dialect ⇒ never spares ⇒ collide (the
        // safe floor when provenance cannot be recovered, incl. fence-divergent-meaning ambiguity).
        let mut i = Interner::default();
        let k = KindId(i.intern("sm.dorc.Service"));
        let family = ProviderId(i.intern("systemctl"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("nginx")));
        let active = sel(&mut i, "active");
        let enabled = sel(&mut i, "enabled");
        let d = dialect_of(&mut i, family, k, &["enabled", "active"]);
        assert_eq!(
            compare(
                coord(k, e, Some(active)),
                coord(k, e, Some(enabled)),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &d,
                None, // no recovered family
            ),
            Relation::Overlaps
        );
    }

    #[test]
    fn dialect_sole_family_disambiguates_and_divergent_meaning_falls_to_none() {
        // §3 backing provenance recovery + fence-divergent-meaning: one family minting (kind,
        // selector) ⇒ recovered; two families minting the SAME token ⇒ None (safe floor).
        let mut i = Interner::default();
        let k = KindId(i.intern("com.widget.K"));
        let owner = ProviderId(i.intern("widgetctl"));
        let interloper = ProviderId(i.intern("evilctl"));
        let clean = sel(&mut i, "clean");
        let mut d = Dialect::empty();
        d.mint(owner, k, clean);
        assert_eq!(
            d.sole_family(k, clean),
            Some(owner),
            "one minter ⇒ recovered"
        );
        d.mint(interloper, k, clean); // the same token, another family (divergent meaning)
        assert_eq!(
            d.sole_family(k, clean),
            None,
            "two families minting the same token ⇒ ambiguous ⇒ safe floor None (fence-divergent-meaning)"
        );
    }

    // ── §6 fences pinned at the chokepoint ──────────────────────────────────────────────────

    #[test]
    fn compare_cross_kind_has_no_same_generator() {
        // `277` §6 top-identifies-with-nothing: NO generator produces cross-kind *same* (the
        // co-reference mechanism is parked behind the movable kind-fence). A cross-kind pair is
        // ProvablyDisjoint, NEVER Overlaps — even with identical entities and selectors.
        let mut i = Interner::default();
        let ka = KindId(i.intern("com.a.K"));
        let kb = KindId(i.intern("com.b.K"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("x")));
        let s = sel(&mut i, "installed");
        let r = compare(
            coord(ka, e, Some(s)),
            coord(kb, e, Some(s)),
            EntityResolution::Canonical(e),
            EntityResolution::Canonical(e),
            &Dialect::empty(),
            None,
        );
        assert_ne!(
            r,
            Relation::Overlaps,
            "cross-kind never identifies (no generator)"
        );
        assert_eq!(r, Relation::ProvablyDisjoint, "the movable kind-fence");
    }

    #[test]
    fn compare_never_derives_separation_from_unknown() {
        // `277` §6 never-derive-separation: an UNKNOWN (a resolver gap, an unminted selector) never
        // becomes ProvablyDisjoint — keying/address-inequality is not referent-inequality. The ONLY
        // sources of ProvablyDisjoint are a different kind (ground truth), a distinct canonical
        // entity (the resolve generator), or a dialect selector-spare (authored marks). An
        // unresolvable pair is Unknown, and an unminted-selector same-entity pair is Overlaps (collide)
        // — neither manufactures separation.
        let mut i = Interner::default();
        let k = KindId(i.intern("com.a.K"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("x")));
        let ghost = sel(&mut i, "ghost"); // never minted
        let other = sel(&mut i, "other");
        // Unresolvable ⇒ Unknown, never ProvablyDisjoint.
        assert_eq!(
            compare(
                coord(k, e, Some(ghost)),
                coord(k, e, Some(other)),
                EntityResolution::Unresolvable,
                EntityResolution::Canonical(e),
                &Dialect::empty(),
                None,
            ),
            Relation::Unknown
        );
        // Unminted selectors on a same canonical entity ⇒ Overlaps (collide), never disjoint.
        assert_eq!(
            compare(
                coord(k, e, Some(ghost)),
                coord(k, e, Some(other)),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &Dialect::empty(),
                None,
            ),
            Relation::Overlaps
        );
    }

    #[test]
    fn compare_same_token_divergent_meaning_spares_across_families() {
        // fence-divergent-meaning (`277` §6, differential-tested per `271:rul-net-quality-u-curve`):
        // a claim-token is interpreted in the BACKING family's dialect, so two families spelling the
        // SAME tokens for their own cells will each spare the other's sibling-cell backing under the
        // flag. THIS IS THE PRICED FOOTGUN — documented, never lint-rescued. Both `widgetctl` and
        // `evilctl` mint {clean, dirty} for kind K; a `#dirty` claim spares a `#clean` backing of
        // EITHER family.
        let mut i = Interner::default();
        let k = KindId(i.intern("com.widget.K"));
        let widget = ProviderId(i.intern("widgetctl"));
        let evil = ProviderId(i.intern("evilctl"));
        let e = EntityRef::Operand(OpaqueToken(i.intern("w1")));
        let clean = sel(&mut i, "clean");
        let dirty = sel(&mut i, "dirty");
        let mut d = dialect_of(&mut i, widget, k, &["clean", "dirty"]);
        d.mint(evil, k, clean);
        d.mint(evil, k, dirty);
        let spare = |family| {
            compare(
                coord(k, e, Some(dirty)),
                coord(k, e, Some(clean)),
                EntityResolution::Canonical(e),
                EntityResolution::Canonical(e),
                &d,
                Some(family),
            )
        };
        assert_eq!(
            spare(widget),
            Relation::ProvablyDisjoint,
            "widget's own backing spared"
        );
        assert_eq!(
            spare(evil),
            Relation::ProvablyDisjoint,
            "evil's backing ALSO spared by the same token — the divergent-meaning footgun"
        );
    }

    #[test]
    fn universal_meet_any_unknown_member_collides_order_independent() {
        // `277` §5 set-lifting (pin-set-meet-order-independence + pin-no-outcome-as-generator).
        // Backing-SETS are a RESERVED seam (singletons at v1), so this pins the LAW with a SYNTHETIC
        // set — the universal meet the value-recipe-reshape will implement. A set SPARES iff EVERY
        // member is ProvablyDisjoint; any non-disjoint member ⇒ collide, whatever the resolution
        // order. `set_spares` is a PURE fold over member verdicts — no member's outcome re-enters as
        // another's input (pin-no-outcome-as-generator).
        fn set_spares(members: &[Relation]) -> bool {
            members
                .iter()
                .all(|r| matches!(r, Relation::ProvablyDisjoint))
        }
        let with_unknown = [
            Relation::ProvablyDisjoint,
            Relation::Unknown,
            Relation::ProvablyDisjoint,
        ];
        let mut reversed = with_unknown;
        reversed.reverse();
        assert!(!set_spares(&with_unknown), "any Unknown member ⇒ collide");
        assert!(
            !set_spares(&reversed),
            "order-independent: reversed still collides"
        );
        assert!(
            !set_spares(&[Relation::ProvablyDisjoint, Relation::Overlaps]),
            "an Overlaps member also collides the set"
        );
        assert!(
            set_spares(&[Relation::ProvablyDisjoint, Relation::ProvablyDisjoint]),
            "all-disjoint ⇒ the set spares"
        );
    }
}
