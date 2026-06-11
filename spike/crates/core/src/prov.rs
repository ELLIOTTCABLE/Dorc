//! The **receipts plane** — per-value origin provenance, captured for refuse-or-explain
//! ONLY (`Research/plans/22A` concl-1..4, `notes/220` §6, the round-22 arch-1 contract).
//!
//! Every abstract value can answer "where did you come from": a [`ProvArena`] is an
//! append-only, per-run, hash-consed store of [`OriginNode`]s; a value carries one
//! [`ProvId`] receipt. This is the `seam-prov` locator-DAG made concrete (`plan/CLAUDE.md`
//! `an-locator-dag`; `111` dac-A) — a PROV-shaped DAG of located nodes + bounded parent
//! edges, resolved to human text lazily controller-side (rustc `Span`→`SourceMap`, `111`).
//!
//! # The WELD (the one non-negotiable contract, ru-11 / `22A` §1 arch-1)
//!
//! Receipts are FULLY one-way: they may influence **nothing** — not a license, not a join
//! order, not a fold decision, not a disposition. There is no well-typed path from a
//! [`ProvId`] or [`OriginNode`] back into a decision input. Two structural guarantees make
//! that true rather than merely promised:
//!
//! * **The arena is the only thing that can read a node from an id** ([`ProvArena::node`]),
//!   and the decision crates never hold the arena where a license is minted. A receipt is an
//!   opaque `u32`-shaped token at every decision site.
//! * **`ProvId` deliberately is NOT `Ord`** (only `Eq`/`Hash`). It therefore cannot key a
//!   `BTreeMap`/`BTreeSet` whose iteration order is a decision output (`inv-determinism`):
//!   "sort the decision by receipt" will not compile. Equality is permitted (hash-consing
//!   needs it; a `==` is order-free), ordering is not. This is the `f-2` ordering hazard
//!   (`22A` concl-4) turned into a type error.
//!
//! The erasability gate (`plan::erasability`) proves the plane inert end-to-end; this module
//! supplies the representation it strips and adversarially varies.
//!
//! # Why it lives in `core` (`dac-B`)
//!
//! Like every shared id-space, the receipt vocabulary must be agreed *before* the analyzer
//! and the diagnostic/render layers build on it, or they grow two incompatible provenance
//! graphs. `core` is dependency-free, so the arena uses only `std` collections; its internal
//! dedup [`HashMap`] is never iterated to produce output (the same discipline `Interner`
//! holds — `core/CLAUDE.md` `inv-determinism`), and node assignment is append-order, never
//! hashed/random.

use std::num::NonZeroU32;

use crate::Span;
use crate::unord::IterSuppressedMap;

/// A per-value provenance **receipt**: an index into a [`ProvArena`]'s origin nodes.
///
/// `NonZeroU32`-backed so `Option<ProvId>` is niche-packed to one word (`notes/220` §6:
/// "`Option`-niche-friendly") — a value with no captured origin pays nothing. The first arena
/// node is id `1`; `0` is reserved as the niche.
///
/// **Not `Ord` by design** (the WELD's structural half): a `ProvId` may be compared for
/// equality (hash-consing and the gate's sentinel-stripping need `==`) and hashed (to dedup
/// nodes), but it has no total order, so it can never key a decision-output `BTreeMap`/
/// `BTreeSet`. Receipt-induced iteration-order leaks (`22A` concl-4 / `f-2`) become compile
/// errors rather than gate catches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProvId(NonZeroU32);

impl ProvId {
    /// The raw index, for the gate's sentinel construction and lazy controller-side render
    /// only — never to reconstruct a node without the arena (`node` is the sole reader).
    #[must_use]
    pub fn index(self) -> u32 {
        self.0.get()
    }
}

/// What KIND of origin a node records — a **closed** enum (`notes/220` §6: "keep `OriginKind`
/// a closed enum … reserve the re-derivation door"). Adding a variant must break every
/// exhaustive match (the compiler-as-checklist), so no `#[non_exhaustive]`. The kind is pure
/// classification for the lazy human render; it is never branched on by a decision (the WELD).
///
/// The tiers mirror the `notes/220` §6 origin tiers (oracle-claim / book-source /
/// probe-result / runtime) plus the structural ⊤-cause and join (`vp-6`) cases the analyzer
/// actually mints today. Each is the *reason a value exists*, keyed on a stable site, never
/// on visit order (`vp-9`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OriginKind {
    /// A value read straight from the book's source text (a literal argv word, a redirect
    /// target) — the `loc-user-src` tier.
    BookSource,
    /// A ⊤ ("can't-characterize") arose HERE — an unmodeled command word, a ⊤ operand, a
    /// ⊤-rejected region. This is the cause that makes the ⊤-poison cascade attributable
    /// (`21Z`: `Reach::Top` "is causally opaque today — your `Top(cause)` is what makes it
    /// attributable"). The single most load-bearing kind: the analyzer's give-up points.
    TopCause,
    /// A value formed by `join`ing ≥2 inbound origins at a control-flow merge (`vp-6`). The
    /// node's parents are the joined origins, k-capped (see [`ProvArena::join`]).
    Join,
    /// An oracle-declared fact's origin (the `loc-oracle`/claim tier) — reserved for the
    /// claim-vs-receipt trust axis (`225` §0 Carata tail); minted as the effect-map grows.
    OracleClaim,
    /// A host probe-result's origin (the `loc-probe` tier) — reserved for when a probe-sourced
    /// observable carries provenance into the why-lens.
    ProbeResult,
}

/// The bounded fan-in of an [`OriginNode`] (`notes/220` §6: a bounded parent list). A join's
/// inbound origins past the cap are dropped with a [`truncated`] marker
/// rendered as "…and N more" (`vp-6`) — values are many and capped. A non-join node has ≤1
/// parent (or none, for a leaf origin). Kept as an explicit small struct (not a bare `Vec`)
/// so the truncation is part of the type, never a silently-lossy `Vec::truncate`.
///
/// [`truncated`]: Parents::truncated
/// [`JOIN_PARENT_CAP`]: crate::prov::JOIN_PARENT_CAP
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parents {
    /// The retained parent receipts, in the order they were offered (a stable, caller-fixed
    /// order — the join offers them by sorted site identity, never visit order). At most
    /// [`JOIN_PARENT_CAP`] for a join.
    parents: Vec<ProvId>,
    /// How many parents were dropped by the cap (`vp-6`'s "…and N more"). `0` for an
    /// un-truncated node.
    truncated: u32,
}

impl Parents {
    /// No parents — a leaf origin (a book literal, a give-up site).
    #[must_use]
    pub fn none() -> Self {
        Self {
            parents: Vec::new(),
            truncated: 0,
        }
    }

    /// The retained parent receipts.
    #[must_use]
    pub fn ids(&self) -> &[ProvId] {
        &self.parents
    }

    /// How many parents the cap dropped (the "…and N more" count; `0` ⇒ none dropped).
    #[must_use]
    pub fn truncated(&self) -> u32 {
        self.truncated
    }
}

/// One origin in the arena: WHY a value exists, WHERE (a stable [`Span`] site), and its
/// bounded parent receipts (`notes/220` §6: "Node ≈ (kind, site, parents)"). Hash-consed —
/// two structurally-identical origins share one [`ProvId`] (the memory lever: fixpoint
/// iteration re-derives identical origins constantly; sharing is the whole game — `ProvSQL`
/// circuits, rustc hygiene chains).
///
/// `Hash`/`Eq` over all fields is what the hash-cons keys on; it is NEVER used to order or
/// to drive a decision (the WELD — and the node is reachable only through the arena anyway).
/// `Hash` is hand-written (below) because [`Span`] derives `Eq` but not `Hash`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OriginNode {
    /// Why this value exists (closed classification, never branched-on by a decision).
    pub kind: OriginKind,
    /// The stable source site this origin keys on (`vp-9`: stable site identity, never visit
    /// order). `None` for an origin with no single source point (a synthetic join).
    pub site: Option<Span>,
    /// The bounded inbound origins (≤1 for a leaf/unary node; k-capped for a [`OriginKind::Join`]).
    pub parents: Parents,
}

/// `Span` is `Copy + Eq` but intentionally not `Hash` in the wider crate; the arena needs a
/// hashable key for consing, so hash the `Span`'s byte coordinates here (a private detail of
/// the dedup map, never observable). Keeping it local avoids widening `Span`'s derives.
impl std::hash::Hash for OriginNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.kind.hash(state);
        // Span is two BytePos(u32); hash the coordinates (Span derives Eq but not Hash).
        match self.site {
            Some(s) => {
                1u8.hash(state);
                s.lo.0.hash(state);
                s.hi.0.hash(state);
            }
            None => 0u8.hash(state),
        }
        self.parents.hash(state);
    }
}

impl std::hash::Hash for Parents {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parents.hash(state);
        self.truncated.hash(state);
    }
}

/// The cap on a join node's retained parents (`notes/220` §6 / `vp-6`: "Join nodes k-capped").
/// A small constant: provenance is a refuse-or-explain aid, and "…and N more" is a faithful
/// summary past a handful. Licenses are EXEMPT from this cap (they store the full granted
/// witness — `plan`'s concern; `vp-17`/`vp-18`): values are many and capped, licenses are
/// few and exact.
pub const JOIN_PARENT_CAP: usize = 4;

/// An append-only, per-run, hash-consed store of [`OriginNode`]s — the receipts plane's
/// backing arena (`notes/220` §6: "One append-only per-run arena of origin nodes").
///
/// Append-only + hash-consed: minting an origin that structurally equals an existing one
/// returns the existing [`ProvId`] (no growth), so a fixpoint that re-derives the same origin
/// every iteration adds nothing after the first — the termination-safety the `Reach::Top`
/// reshape relies on (a fresh id per iteration would defeat the lattice `Eq`).
///
/// Determinism (`inv-determinism`): id assignment is append-order (`nodes.len()+1`), never
/// hashed/random; the internal dedup index is an [`IterSuppressedMap`] (no iteration API at
/// all — the order leak is a compile error, not a discipline-by-convention), used only for
/// `mint`'s already-present-lookup. The arena is a write-only-then-read log; nothing
/// re-ingests it across runs (`kSTATE` stays parked — `ru-12`/`f-6`).
#[derive(Debug, Default)]
pub struct ProvArena {
    /// The origins, indexed by `ProvId(i+1)`. Append-only.
    nodes: Vec<OriginNode>,
    /// Dedup index: a structurally-equal node maps to its existing id (hash-consing). An
    /// [`IterSuppressedMap`] — equality, not order, is all it needs, and the missing iteration
    /// API guarantees its hash order can never leak into output.
    cons: IterSuppressedMap<OriginNode, ProvId>,
}

impl ProvArena {
    /// A fresh empty arena (one per analyzer run).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Mint (or hash-cons) an origin, returning its receipt. A structurally-identical origin
    /// already in the arena returns the SAME id without growing — the sharing that keeps the
    /// arena bounded under fixpoint re-derivation.
    ///
    /// Append-order id assignment is deterministic; `u32::MAX` is the saturating ceiling (the
    /// no-throw posture — an arena that large is a pathological corpus, not a correctness bug).
    pub fn mint(&mut self, node: OriginNode) -> ProvId {
        if let Some(&id) = self.cons.get(&node) {
            return id;
        }
        // The next id is the current count + 1 (id 0 reserved for the Option niche). Saturate
        // rather than panic (`inv-no-throw`); a clash at the ceiling re-cons's onto MAX, which
        // is sound (it only ever over-shares provenance, never a decision).
        let raw = u32::try_from(self.nodes.len())
            .unwrap_or(u32::MAX - 1)
            .saturating_add(1);
        let id = ProvId(NonZeroU32::new(raw).unwrap_or(NonZeroU32::MAX));
        self.nodes.push(node.clone());
        self.cons.insert(node, id);
        id
    }

    /// Mint a leaf origin at `site` of `kind` (no parents) — the common case for a book
    /// literal or a ⊤-cause give-up point.
    pub fn leaf(&mut self, kind: OriginKind, site: Option<Span>) -> ProvId {
        self.mint(OriginNode {
            kind,
            site,
            parents: Parents::none(),
        })
    }

    /// Mint a JOIN origin over `inbound` (`vp-6`): a [`OriginKind::Join`] node whose parents
    /// are the FIRST [`JOIN_PARENT_CAP`] of `inbound`, with the remainder recorded as the
    /// truncation count ("…and N more").
    ///
    /// THE CONTRACT (`vp-9`, the WELD's order half): the caller MUST offer `inbound` in a
    /// stable, site-derived order (never dataflow visit order), so which parents survive the
    /// cap is a deterministic function of the program, not of iteration. This method does not
    /// re-sort (it cannot — `ProvId` has no `Ord`); it trusts the caller's stable order and
    /// caps positionally. A single-element `inbound` is returned as-is (no join node — a join
    /// of one origin IS that origin), keeping the arena minimal.
    pub fn join(&mut self, site: Option<Span>, inbound: &[ProvId]) -> Option<ProvId> {
        match inbound {
            [] => None,
            [only] => Some(*only),
            many => {
                let kept: Vec<ProvId> = many.iter().take(JOIN_PARENT_CAP).copied().collect();
                let truncated =
                    u32::try_from(many.len().saturating_sub(JOIN_PARENT_CAP)).unwrap_or(u32::MAX);
                Some(self.mint(OriginNode {
                    kind: OriginKind::Join,
                    site,
                    parents: Parents {
                        parents: kept,
                        truncated,
                    },
                }))
            }
        }
    }

    /// Resolve a receipt to its node — the SOLE reader (the WELD: a decision crate that does
    /// not hold the arena cannot read receipt data, so a license cannot depend on one). For
    /// the lazy controller-side why-render and the gate's structural assertions. Returns
    /// `None` for an id not from this arena (defensive; never panics — `inv-no-throw`).
    #[must_use]
    pub fn node(&self, id: ProvId) -> Option<&OriginNode> {
        self.nodes.get((id.index() as usize).checked_sub(1)?)
    }

    /// How many origins the arena holds — for the gate's growth assertions and the digest's
    /// size accounting. Not a decision input.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Is the arena empty?
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BytePos;

    fn span(lo: u32, hi: u32) -> Span {
        Span::new(BytePos(lo), BytePos(hi))
    }

    #[test]
    fn provid_is_option_niche_packed() {
        // notes/220 §6: ProvId must be Option-niche-friendly so a value with no origin pays
        // nothing. NonZeroU32 backing makes Option<ProvId> one word.
        assert_eq!(
            size_of::<Option<ProvId>>(),
            size_of::<ProvId>(),
            "Option<ProvId> must niche-pack (NonZeroU32 backing)"
        );
        assert_eq!(size_of::<ProvId>(), 4, "ProvId is one u32");
    }

    #[test]
    fn hash_cons_shares_identical_origins() {
        // THE memory lever (notes/220 §6): a re-minted structurally-identical origin returns
        // the SAME id without growing. This is what keeps the arena bounded under fixpoint
        // re-derivation (and what lets the Reach::Top reshape stay terminating — a fresh id
        // per iteration would defeat the lattice Eq; hunt-6).
        let mut a = ProvArena::new();
        let first = a.leaf(OriginKind::TopCause, Some(span(0, 3)));
        let again = a.leaf(OriginKind::TopCause, Some(span(0, 3)));
        assert_eq!(first, again, "identical origins hash-cons to one id");
        assert_eq!(a.len(), 1, "no growth on a re-derived origin");
        // A different site is a distinct origin.
        let other = a.leaf(OriginKind::TopCause, Some(span(4, 7)));
        assert_ne!(first, other);
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn ids_are_append_order_deterministic() {
        // inv-determinism: id assignment is append-order, never hashed. Two arenas fed the
        // same origins in the same order assign the same ids.
        let mut a = ProvArena::new();
        let mut b = ProvArena::new();
        let a1 = a.leaf(OriginKind::BookSource, Some(span(0, 1)));
        let a2 = a.leaf(OriginKind::TopCause, Some(span(2, 3)));
        let b1 = b.leaf(OriginKind::BookSource, Some(span(0, 1)));
        let b2 = b.leaf(OriginKind::TopCause, Some(span(2, 3)));
        assert_eq!(a1.index(), b1.index());
        assert_eq!(a2.index(), b2.index());
        assert_eq!(a1.index(), 1, "first id is 1 (0 is the niche)");
        assert_eq!(a2.index(), 2);
    }

    #[test]
    fn join_caps_parents_with_truncation_marker() {
        // vp-6: a join past the cap keeps the first JOIN_PARENT_CAP parents and records the
        // remainder as "…and N more". Values are many and capped (licenses, exempt, are not).
        let mut a = ProvArena::new();
        let parents: Vec<ProvId> = (0..u32::try_from(JOIN_PARENT_CAP + 3).unwrap())
            .map(|i| a.leaf(OriginKind::BookSource, Some(span(i, i + 1))))
            .collect();
        let joined = a
            .join(Some(span(0, 100)), &parents)
            .expect("a join of many");
        let node = a.node(joined).expect("joined node");
        assert_eq!(node.kind, OriginKind::Join);
        assert_eq!(
            node.parents.ids().len(),
            JOIN_PARENT_CAP,
            "retained parents are capped"
        );
        assert_eq!(
            node.parents.truncated(),
            3,
            "the remainder is '…and 3 more'"
        );
    }

    #[test]
    fn join_of_one_is_that_origin() {
        // A join of a single inbound origin IS that origin — no spurious join node (keeps the
        // arena minimal; a degenerate join carries no information).
        let mut a = ProvArena::new();
        let only = a.leaf(OriginKind::BookSource, Some(span(0, 3)));
        let joined = a.join(Some(span(0, 3)), &[only]);
        assert_eq!(joined, Some(only), "a join of one is that one");
        assert_eq!(a.len(), 1, "no join node minted for a singleton");
    }

    #[test]
    fn join_of_none_is_none() {
        let mut a = ProvArena::new();
        assert_eq!(a.join(None, &[]), None, "an empty join has no origin");
    }

    #[test]
    fn node_reads_back_and_unknown_id_is_none() {
        let mut a = ProvArena::new();
        let id = a.leaf(OriginKind::ProbeResult, Some(span(5, 9)));
        let node = a.node(id).expect("read back");
        assert_eq!(node.kind, OriginKind::ProbeResult);
        assert_eq!(node.site, Some(span(5, 9)));
        // An id whose index exceeds the arena resolves to None (defensive; never panics).
        let bogus = ProvId(NonZeroU32::new(999).unwrap());
        assert_eq!(a.node(bogus), None);
    }
}
