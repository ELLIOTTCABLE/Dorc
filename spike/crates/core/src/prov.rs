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
///
/// # The erasability gate's adversarial seam ([`ProvArena::adversarial`])
///
/// The gate's run-B needs an arena that assigns deliberately-WRONG receipt values and
/// *reverses* join-parent order, so any decision that secretly read a receipt value or
/// origin-order would diverge from run-A (`22A` concl-1, the variance-injection mandate;
/// `notes/229` finding-2 — Debian's SENTINEL values, not a tame shift). [`Variation`] is that
/// DI'd seam: in `Adversarial` mode every freshly-assigned id is a high-range SENTINEL with an
/// odd stride (so its value, parity, ordering, and modulo-residues are all unrelated to a
/// `None` run's small append-order ids — a leak doing ANY arithmetic on a receipt diverges),
/// and every [`join`](Self::join) reverses its inbound parents. The arena stays internally
/// consistent (hash-consing still returns the stored id; a parallel `ids` vector makes
/// [`node`](Self::node) resolve any id scheme), so the analyzer runs identically — only the
/// receipt *values/order* differ. The gate asserts the decision output is byte-identical
/// regardless; a divergence IS a receipt-into-decision leak. The seam is the analyzer's only
/// nondeterminism knob here and stays fully injected (`inv-determinism`): production always
/// uses [`new`](Self::new) (`Variation::None`).
#[derive(Debug, Default)]
pub struct ProvArena {
    /// The origins, in insertion order. Append-only. Indexed positionally; the assigned id is
    /// in `ids` at the same position (so any id scheme resolves back via [`node`](Self::node)).
    nodes: Vec<OriginNode>,
    /// The assigned [`ProvId`] per node-index (parallel to `nodes`). Lets [`node`](Self::node)
    /// resolve an id under ANY assignment scheme (the adversarial sentinels are not a simple
    /// arithmetic shift), by matching the id here.
    ids: Vec<ProvId>,
    /// Dedup index: a structurally-equal node maps to its existing id (hash-consing). An
    /// [`IterSuppressedMap`] — equality, not order, is all it needs, and the missing iteration
    /// API guarantees its hash order can never leak into output.
    cons: IterSuppressedMap<OriginNode, ProvId>,
    /// The gate's variance knob (production: `None`).
    variation: Variation,
}

/// How a [`ProvArena`] varies its receipt assignment — the erasability gate's DI'd seam
/// (`22A` concl-1). Production is always [`Variation::None`]; the gate's run-B is
/// [`Variation::Adversarial`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Variation {
    /// Plain append-order ids, parents in offered order — the production arena.
    #[default]
    None,
    /// Receipt values are high-range SENTINELS (odd-strided, seed-shifted) and join parents
    /// REVERSED, so a decision reading a receipt value/order/parity/residue diverges from a
    /// `None` run (the gate's leak probe — `22A` concl-1's sentinel mandate, not a tame shift).
    Adversarial { seed: u32 },
}

/// The adversarial sentinel base — a high u32 so run-B's ids cannot collide with or relate to a
/// `None` run's small append-order ids. Far below `u32::MAX` so the odd-strided counter never
/// overflows for any realistic arena (`inv-no-throw`: a saturating add guards even so).
const SENTINEL_BASE: u32 = 0xF000_0000;
/// An odd stride (so successive sentinels alternate parity — a leak reading id-parity diverges)
/// that is not a small power of two (so modulo-residue leaks diverge too).
const SENTINEL_STRIDE: u32 = 0x0001_9E37;

impl ProvArena {
    /// A fresh empty arena (one per analyzer run) — the production arena ([`Variation::None`]).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A fresh arena that ADVERSARIALLY varies receipt assignment (the erasability gate's
    /// run-B, `22A` concl-1): high-range odd-strided SENTINEL ids, join parents reversed.
    /// Internally consistent (the analyzer runs identically); only receipt values/order differ,
    /// so a decision divergence from a [`new`](Self::new) run is a receipt-into-decision leak.
    #[must_use]
    pub fn adversarial(seed: u32) -> Self {
        Self {
            variation: Variation::Adversarial { seed },
            ..Self::default()
        }
    }

    /// Mint (or hash-cons) an origin, returning its receipt. A structurally-identical origin
    /// already in the arena returns the SAME id without growing — the sharing that keeps the
    /// arena bounded under fixpoint re-derivation.
    ///
    /// Append-order id assignment is deterministic (`Variation::None`: id = `index + 1`);
    /// `u32::MAX` is the saturating ceiling (the no-throw posture). Under
    /// [`Variation::Adversarial`] the value is a high-range odd-strided SENTINEL (deliberately
    /// unrelated to a `None` run's ids — the gate's leak probe), but hash-consing still returns
    /// the stored id for a repeat, so the arena stays internally consistent regardless.
    pub fn mint(&mut self, node: OriginNode) -> ProvId {
        if let Some(&id) = self.cons.get(&node) {
            return id;
        }
        let index = u32::try_from(self.nodes.len()).unwrap_or(u32::MAX - 1);
        let raw = match self.variation {
            // id 0 reserved for the Option niche ⇒ index + 1.
            Variation::None => index.saturating_add(1),
            // A high SENTINEL with an odd stride + seed shift: value/parity/ordering/residue all
            // unrelated to a `None` run's ids (`22A` concl-1). Saturating throughout (no panic).
            Variation::Adversarial { seed } => SENTINEL_BASE
                .saturating_add(seed)
                .saturating_add(index.saturating_mul(SENTINEL_STRIDE))
                .max(1),
        };
        let id = ProvId(NonZeroU32::new(raw).unwrap_or(NonZeroU32::MAX));
        self.nodes.push(node.clone());
        self.ids.push(id);
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
    ///
    /// Under [`Variation::Adversarial`] the inbound order is REVERSED before capping (the
    /// gate's run-B origin-order perturbation, `22A` concl-1): if any decision read the
    /// surviving-parent set or its order, run-B would diverge from a `None` run.
    pub fn join(&mut self, site: Option<Span>, inbound: &[ProvId]) -> Option<ProvId> {
        match inbound {
            [] => None,
            [only] => Some(*only),
            many => {
                let mut offered: Vec<ProvId> = many.to_vec();
                if matches!(self.variation, Variation::Adversarial { .. }) {
                    offered.reverse();
                }
                let kept: Vec<ProvId> = offered.iter().take(JOIN_PARENT_CAP).copied().collect();
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
    ///
    /// Resolves under ANY id scheme (the `None` index-arithmetic OR an adversarial sentinel) by
    /// the parallel `ids` vector: the fast `None` path is direct indexing; otherwise a scan
    /// (arenas are tiny and `node` is lazy/controller-side, never hot).
    #[must_use]
    pub fn node(&self, id: ProvId) -> Option<&OriginNode> {
        match self.variation {
            Variation::None => self.nodes.get((id.index() as usize).checked_sub(1)?),
            Variation::Adversarial { .. } => {
                let pos = self.ids.iter().position(|&stored| stored == id)?;
                self.nodes.get(pos)
            }
        }
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
    fn prov_id_is_option_niche_packed() {
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

    #[test]
    fn adversarial_arena_assigns_different_ids_but_reads_back() {
        // The gate's run-B seam (22A concl-1): an adversarial arena assigns DIFFERENT receipt
        // VALUES than a plain one (so a decision reading a specific value would diverge), yet
        // a node still reads back through `node()` (the offset is reversed). This is the
        // sentinel-ProvId mechanism: run-B's ids are deliberately not run-A's.
        let mut plain = ProvArena::new();
        let mut adv = ProvArena::adversarial(1000);
        let p = plain.leaf(OriginKind::TopCause, Some(span(0, 3)));
        let v = adv.leaf(OriginKind::TopCause, Some(span(0, 3)));
        assert_ne!(
            p.index(),
            v.index(),
            "adversarial ids differ from plain (the leak probe)"
        );
        // …but the adversarial node still resolves (offset reversed in `node`).
        let node = adv.node(v).expect("adversarial node reads back");
        assert_eq!(node.kind, OriginKind::TopCause);
        assert_eq!(node.site, Some(span(0, 3)));
    }

    #[test]
    fn adversarial_arena_reverses_join_parents() {
        // The gate's origin-order perturbation (22A concl-1): an adversarial join reverses the
        // offered parents before capping, so the surviving-parent SET differs when truncation
        // bites. Over-cap by one so reversal changes which parent is dropped.
        let n = JOIN_PARENT_CAP + 1;
        let mut plain = ProvArena::new();
        let p_parents: Vec<ProvId> = (0..u32::try_from(n).unwrap())
            .map(|i| plain.leaf(OriginKind::BookSource, Some(span(i, i + 1))))
            .collect();
        let p_join = plain.join(Some(span(0, 99)), &p_parents).unwrap();
        let p_kept = plain.node(p_join).unwrap().parents.ids().to_vec();

        let mut adv = ProvArena::adversarial(0); // seed 0 ⇒ same ids, but join still reverses
        let a_parents: Vec<ProvId> = (0..u32::try_from(n).unwrap())
            .map(|i| adv.leaf(OriginKind::BookSource, Some(span(i, i + 1))))
            .collect();
        let a_join = adv.join(Some(span(0, 99)), &a_parents).unwrap();
        let a_kept = adv.node(a_join).unwrap().parents.ids().to_vec();

        // Plain keeps the FIRST cap parents; adversarial keeps the LAST cap (reversed), so the
        // dropped parent differs — the order perturbation is observable in the witness.
        assert_ne!(
            p_kept, a_kept,
            "adversarial join reverses parents ⇒ a different survivor set when truncated"
        );
    }
}
