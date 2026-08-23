//! `core::influence` — the **influence grade** (`306b` §1): how far a value stands from bytes a
//! managed host produced.
//!
//! The word is deliberately narrow (`306b` §1). This engine already runs several taint-flow
//! analyses over the analyzed program — value-grade provenance, read-set closure, the who-am-I
//! ingredient labelling, the emission-set non-interference derivation. `influence` names exactly
//! ONE of them and is never a synonym for taint in general: derivation from host-reported bytes.
//!
//! # Three grades, one direction (`306b` §1a)
//!
//! * [`AuthoredBeforeContact`] — computed only from controller-supplied invocation material and
//!   operator-authored source text, all of which exists before the first host exchange.
//! * [`HostReported`] — the host-produced bytes themselves, as received.
//! * [`HostInfluenced`] — anything whose value, or whose control-flow path, depended on either.
//!
//! The grade moves one way. Widening is free; **the lowering conversion does not exist**, exactly
//! as [`crate::claim`]'s tiers have no upgrade and `Must → May` has no inverse. The mechanism is the
//! same: no such `fn`/`impl` is written anywhere in this module, and the doctests below pin that
//! absence — a property ordinary tests cannot express and a well-meaning `From` silently repeals.
//!
//! # It reaches the analyzer, not only the analyzed world (`306b` §1b)
//!
//! Influence is a property of the engine's OWN execution: if a host-reported value determines which
//! branch our code takes, everything computed inside that branch is influenced, including values
//! that never touch a coordinate, a fact, or a verdict. Discarding an entire analysis is an
//! influenced result. That is why [`Influenced::read`] hands back a PHASE MARKER beside the payload
//! rather than a bare borrow: reading host-reported material is the act that makes what follows
//! influenced, so the marker is not something a caller can forget to derive.
//!
//! # v0 is positional and global, and that is deliberate
//!
//! The flip happens at first host-byte ingestion; every code path invoked after that point is
//! within its scope. This is a phase property carried BY CONSTRUCTION, not a per-value dataflow
//! analysis — coarse on purpose, and cheap. Gradation (`306b` §1c) is the open item and nothing
//! here may pre-commit it.
//!
//! Carriage is IN-MEMORY ONLY and terminates at the decision plane. Persisting a grade is
//! enrichment of what the durable holds and is deliberately not built here, so `306b` §3a/§3b's
//! rehydration rules are not owed yet.
//!
//! # The properties, pinned
//!
//! The positive control compiles with the same imports and helpers as the negatives, so a
//! `compile_fail` passing for a trivial reason shows up here first.
//!
//! ```
//! use dorc_core::influence::{HostInfluenced, HostReported, Influenced};
//! fn decide(_: Influenced<HostInfluenced, u8>) {}
//!
//! let received: Influenced<HostReported, u8> = Influenced::host_reported(7u8);
//! let (value, _phase) = received.read();
//! assert_eq!(*value, 7);
//! decide(received.widen());
//! ```
//!
//! Narrowing has no inverse: a host-influenced value never becomes host-reported again. This is the
//! one that repeals quietly — it starts compiling the day someone adds a `From` to unstick a build.
//!
//! ```compile_fail
//! use dorc_core::influence::{HostInfluenced, HostReported, Influenced};
//! let widened: Influenced<HostInfluenced, u8> = Influenced::host_reported(7u8).widen();
//! let _narrowed: Influenced<HostReported, u8> = widened.into();
//! ```
//!
//! Nor does an influenced value become authored-before-contact — the grade the round-zero analysis
//! lives at, and the one a host may never reach.
//!
//! ```compile_fail
//! use dorc_core::influence::{AuthoredBeforeContact, HostReported, Influenced};
//! let received: Influenced<HostReported, u8> = Influenced::host_reported(7u8);
//! let _laundered: Influenced<AuthoredBeforeContact, u8> = received.into();
//! ```
//!
//! Uninfluenced material reads WITHOUT minting a phase marker, and influenced material cannot
//! borrow through that accessor: the two are different types with different exits.
//!
//! ```compile_fail
//! use dorc_core::influence::{HostReported, Influenced};
//! let received: Influenced<HostReported, u8> = Influenced::host_reported(7u8);
//! let _free = received.before_contact();
//! ```

use core::marker::PhantomData;

/// The sealing module: the [`Grade`] trait's supertrait is private to `core`, so no crate outside
/// can add a fourth grade. Gradation is an OPEN design question (`306b` §1c) and a downstream
/// invention would foreclose it.
mod sealed {
    pub trait Sealed {}
}

/// How far a value stands from host-produced bytes. **Sealed**: [`AuthoredBeforeContact`],
/// [`HostReported`] and [`HostInfluenced`] are the only inhabitants.
pub trait Grade: sealed::Sealed {}

/// Computed only from controller-supplied invocation material and operator-authored source text —
/// everything that exists before the first host exchange. Uninhabited: it exists to index a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoredBeforeContact {}

/// The host-produced bytes themselves, as received. Uninhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostReported {}

/// Anything whose value, or whose control-flow path, depended on host-reported material —
/// including the engine's own scheduling, iteration counts, and which passes ran. Uninhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostInfluenced {}

impl sealed::Sealed for AuthoredBeforeContact {}
impl sealed::Sealed for HostReported {}
impl sealed::Sealed for HostInfluenced {}
impl Grade for AuthoredBeforeContact {}
impl Grade for HostReported {}
impl Grade for HostInfluenced {}

/// A payload `P` carried together with its [`Grade`] `G`. The grade is a zero-size phantom, so two
/// grades of one payload are DISTINCT types the compiler keeps apart at every boundary. The payload
/// is private: the only ways in are the grade-specific mints, and the only ways out are the
/// grade-specific accessors.
///
/// **When-blocked:** if this type blocks your build — a seat wants uninfluenced material and you
/// hold host-reported or host-influenced material — you hold the WRONG value, not the wrong type.
/// There is no conversion down, deliberately; obtain the value from controller-owned invocation
/// context, or let the influenced answer stay influenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Influenced<G: Grade, P> {
    payload: P,
    _grade: PhantomData<G>,
}

/// The phase marker `306b` §1b describes: proof that host-reported material has been read, and
/// therefore that everything computed downstream of that read is influenced.
///
/// Decision-inert by construction — its payload is `()`, so it carries no value any decision could
/// branch on, which is what keeps `306b` §6b (no host-derived scalar gates engine control flow)
/// true of it by shape rather than by discipline.
pub type InfluencePhase = Influenced<HostInfluenced, ()>;

impl<P> Influenced<AuthoredBeforeContact, P> {
    /// Mint material that exists before the first host exchange.
    #[must_use]
    pub fn authored_before_contact(payload: P) -> Self {
        Self {
            payload,
            _grade: PhantomData,
        }
    }

    /// Borrow uninfluenced material. No phase marker is owed: reading material the host never
    /// touched influences nothing.
    #[must_use]
    pub fn before_contact(&self) -> &P {
        &self.payload
    }

    /// Widen to [`HostInfluenced`]. Free, and one-way.
    #[must_use]
    pub fn widen(self) -> Influenced<HostInfluenced, P> {
        Influenced {
            payload: self.payload,
            _grade: PhantomData,
        }
    }
}

impl<P> Influenced<HostReported, P> {
    /// **THE ONE MINTING SEAT** (`306c` §2): the intake edge where host-produced bytes become
    /// anything else. A second minting site is the regression this seat exists to make greppable,
    /// and a lexical fence asserts there is exactly one caller.
    #[must_use]
    pub fn host_reported(payload: P) -> Self {
        Self {
            payload,
            _grade: PhantomData,
        }
    }

    /// Read host-reported material, receiving the [`InfluencePhase`] marker with it.
    ///
    /// The pairing is the point (`306b` §1b): a caller cannot borrow the payload without also
    /// holding the fact that everything it goes on to compute is influenced.
    #[must_use]
    pub fn read(&self) -> (&P, InfluencePhase) {
        (
            &self.payload,
            Influenced {
                payload: (),
                _grade: PhantomData,
            },
        )
    }

    /// [`read`](Self::read)'s owning twin, for a seat that must move the payload onward.
    #[must_use]
    pub fn into_read(self) -> (P, InfluencePhase) {
        (
            self.payload,
            Influenced {
                payload: (),
                _grade: PhantomData,
            },
        )
    }

    /// Convert the payload while KEEPING the grade — a conversion, never a mint.
    ///
    /// This is what lets one minting seat serve a chain of intake conversions: the bytes become
    /// records become typed results without any of those steps re-asserting a grade.
    #[must_use]
    pub fn map<Q>(&self, convert: impl FnOnce(&P) -> Q) -> Influenced<HostReported, Q> {
        Influenced {
            payload: convert(&self.payload),
            _grade: PhantomData,
        }
    }

    /// Widen to [`HostInfluenced`]. Free, and one-way.
    #[must_use]
    pub fn widen(self) -> Influenced<HostInfluenced, P> {
        Influenced {
            payload: self.payload,
            _grade: PhantomData,
        }
    }
}

impl<P> Influenced<HostInfluenced, P> {
    /// Borrow influenced material. Named so a read site is obviously downstream of host contact.
    #[must_use]
    pub fn influenced(&self) -> &P {
        &self.payload
    }
}

/// Where one STABLE SEMANTIC OBJECT stands relative to host contact
/// (`306b:rul-influence-carried-by-entities`).
///
/// [`Influenced`] wraps a VALUE in transit; this is the account a decision, a license, a Spine
/// record, or a projection result CARRIES — private, immutable, non-optional, and joined at the
/// object's own mint from every contributing data and control input
/// (`306b:rul-semantic-mints-join-influence`).
///
/// # Three points, a total chain, `join = max`
///
/// `AuthoredBeforeContact ⊏ HostInfluenced ⊏ Untracked`. A total order gives commutativity,
/// associativity and idempotence for free, so the order-independence
/// `core/CLAUDE.md pin-set-meet-order-independence` asks of every universal meet is a property of
/// the SHAPE here rather than of a proof.
///
/// `Untracked` sits at the TOP because "we did not compute it" is strictly less informative than
/// "we computed it and it is influenced", and the safe reading of less-informative is
/// more-influenced. Joining it with an authored account therefore yields `Untracked`, which IS
/// `306b:rul-untracked-is-not-authored` spelled as an algebra.
///
/// `Untracked` is an ARM of this account and NOT the reserved fourth [`Grade`]: gradation
/// (`306b` §1c) asks how MUCH influence, and this axis asks how much of the derivation we bothered
/// to compute. Spending the reserved slot on it would foreclose the open question.
///
/// # The two mints are not the same kind of claim
///
/// [`of_phase`](Self::of_phase) is EVIDENCE — an [`InfluencePhase`] is obtainable only by having
/// read host-reported material. [`authored_before_contact`](Self::authored_before_contact) is an
/// ASSERTION, and it stays one: no affine clean-of-host witness is built. It is therefore a NAMED
/// POSTURE a seat spells rather than a default, and a lexical fence enumerates its callers.
///
/// # No consumer exists at v0; a future one is a typed human act
///
/// Nothing in the engine reads an account to decide anything today. That is a FACT about v0, never
/// a law: a future decision consumer — per-host contamination gating a revived cross-host planner
/// at the type level is the strawman — is a deliberate human act, never inferred from this absence.
///
/// Reading which of an object's inputs were influenced is the ONE exempt consumer of
/// `306b` §6b (`tc-accounting-reads-are-not-gating`), and the window is narrow by construction:
/// [`of_phase`](Self::of_phase) is the only phase→account transition in the codebase and every
/// other seat merely [`join`](Self::join)s accounts it was handed.
///
/// ```
/// use dorc_core::influence::{Influenced, InfluenceAccount};
/// let authored = InfluenceAccount::authored_before_contact();
/// let influenced = InfluenceAccount::of_phase(Influenced::authored_before_contact(()).widen());
/// assert!(!authored.is_influenced());
/// assert!(influenced.is_influenced());
/// assert!(authored.join(influenced).is_influenced());
/// assert!(InfluenceAccount::untracked().join(authored).is_influenced());
/// ```
///
/// An ABSENT account cannot spell itself authored: there is no `Default`, which is what stops
/// `None = authored` returning in a new shape (`306b` §10).
///
/// ```compile_fail
/// use dorc_core::influence::InfluenceAccount;
/// let _absent = InfluenceAccount::default();
/// ```
///
/// No generic conversion manufactures an account, so no `From` — and no future deserialization
/// route — can mint one at a standing its inputs did not earn.
///
/// ```compile_fail
/// use dorc_core::influence::{Influenced, InfluenceAccount};
/// let phase = Influenced::authored_before_contact(()).widen();
/// let _manufactured: InfluenceAccount = phase.into();
/// ```
///
/// There is no meet. A lowering combinator is what would quietly repeal `306b` §1a's one-way
/// direction, exactly as a `From` repeals [`Influenced`]'s.
///
/// ```compile_fail
/// use dorc_core::influence::InfluenceAccount;
/// let _lowered = InfluenceAccount::untracked().meet(InfluenceAccount::authored_before_contact());
/// ```
///
/// And the standing is unreachable, so nothing outside this module can read one account's point in
/// order to build a lesser one.
///
/// ```compile_fail
/// use dorc_core::influence::InfluenceAccount;
/// let InfluenceAccount(_standing) = InfluenceAccount::untracked();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfluenceAccount(Standing);

/// The three points of the chain. Private: the account's only surface is its mints and its join.
///
/// `HostInfluenced` KEEPS carrying the [`InfluencePhase`] rather than collapsing to a unit variant.
/// The phase's `()` payload is the reserved slot for a future host-scope identity, at which point
/// the account becomes a set of hosts and the join becomes union — a widening the payload already
/// has room for, and one a unit variant would have to re-open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Standing {
    AuthoredBeforeContact,
    HostInfluenced(InfluencePhase),
    Untracked,
}

impl InfluenceAccount {
    /// The NAMED POSTURE for an object computed only from controller-supplied invocation material
    /// and operator-authored source text (`306b` §1's `authored-before-contact`).
    ///
    /// An assertion, not evidence — see the type doc. Spell it where it is true and where the seat
    /// can say why; never reach for it because a build wanted an account.
    #[must_use]
    pub const fn authored_before_contact() -> Self {
        Self(Standing::AuthoredBeforeContact)
    }

    /// The ONE phase→account transition (`tc-accounting-reads-are-not-gating`'s narrow window).
    ///
    /// The [`InfluencePhase`] argument is the evidence: it can only be obtained by having read
    /// host-reported material, so this mint cannot over- or under-claim.
    #[must_use]
    pub const fn of_phase(phase: InfluencePhase) -> Self {
        Self(Standing::HostInfluenced(phase))
    }

    /// An unconverted or unenumerable contributor (`306b:rul-untracked-is-not-authored`).
    ///
    /// Reads maximally influenced, which is what lets the threading stay staged without laundering
    /// its missing region. Every seat spelling this is enumerated by a lexical census, because the
    /// point of the discipline is to WATCH whether these accumulate.
    #[must_use]
    pub const fn untracked() -> Self {
        Self(Standing::Untracked)
    }

    /// The chain's join — `max`, and therefore commutative, associative and idempotent.
    #[must_use]
    pub const fn join(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }

    /// Did host-reported material reach this object, or do we not know? TRUE at both upper points.
    #[must_use]
    pub const fn is_influenced(self) -> bool {
        self.rank() > 0
    }

    /// The account's display token. Referent-agnostic: for rendering and for the durable's closed
    /// grammar, never branched on (`inv-referent-agnostic`).
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self.0 {
            Standing::AuthoredBeforeContact => "authored-before-contact",
            Standing::HostInfluenced(_) => "host-influenced",
            Standing::Untracked => "untracked",
        }
    }

    const fn rank(self) -> u8 {
        match self.0 {
            Standing::AuthoredBeforeContact => 0,
            Standing::HostInfluenced(_) => 1,
            Standing::Untracked => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reading_host_reported_material_yields_the_phase_marker() {
        // `306b` §1b: the marker is not derived by a caller who remembers to; it arrives with the
        // borrow, which is the whole reason `read` is not a bare accessor.
        let received = Influenced::<HostReported, u8>::host_reported(3);
        let (value, phase) = received.read();
        assert_eq!(*value, 3);
        assert_eq!(
            phase,
            Influenced::<HostReported, ()>::host_reported(()).widen()
        );
    }

    #[test]
    fn widening_is_monotone_and_reaches_one_fixed_top() {
        // Both lower grades widen to the SAME top, so a consumer that demands `HostInfluenced`
        // accepts either without either one having to be lowered to meet it.
        let from_authored = Influenced::<AuthoredBeforeContact, u8>::authored_before_contact(9);
        let from_reported = Influenced::<HostReported, u8>::host_reported(9);
        assert_eq!(from_authored.widen(), from_reported.widen());
        assert_eq!(*from_reported.widen().influenced(), 9);
    }

    #[test]
    fn a_conversion_keeps_the_grade_it_was_handed() {
        // The property that lets ONE mint serve a whole intake chain: bytes → records → results
        // without any step re-asserting a grade.
        let received = Influenced::<HostReported, u8>::host_reported(4);
        let converted = received.map(|value| u32::from(*value) * 2);
        let (value, _phase) = converted.read();
        assert_eq!(*value, 8);
    }

    #[test]
    fn uninfluenced_material_reads_without_minting_a_phase() {
        let authored =
            Influenced::<AuthoredBeforeContact, &str>::authored_before_contact("book.sh");
        assert_eq!(*authored.before_contact(), "book.sh");
    }

    fn phase() -> InfluencePhase {
        Influenced::<HostReported, ()>::host_reported(()).widen()
    }

    #[test]
    fn the_account_chain_climbs_and_never_descends() {
        // `306b` §1a at the ACCOUNT tier. Every pair in the chain joins to the higher point, in
        // both argument orders — which is the whole of "no operation lowers a grade" once the
        // combinator is `max` over a total order.
        let points = [
            InfluenceAccount::authored_before_contact(),
            InfluenceAccount::of_phase(phase()),
            InfluenceAccount::untracked(),
        ];
        for (lower_index, lower) in points.iter().enumerate() {
            for higher in &points[lower_index..] {
                assert_eq!(lower.join(*higher), *higher);
                assert_eq!(higher.join(*lower), *higher);
            }
        }
    }

    #[test]
    fn an_untracked_seam_never_reads_authored() {
        // `306b:rul-untracked-is-not-authored` — the property the whole staging rests on: an
        // unconverted contributor must not cleanse the object it contributes to.
        let untracked = InfluenceAccount::untracked();
        assert!(untracked.is_influenced());
        assert!(
            untracked
                .join(InfluenceAccount::authored_before_contact())
                .is_influenced()
        );
        assert!(
            untracked
                .join(InfluenceAccount::of_phase(phase()))
                .is_influenced()
        );
    }

    #[test]
    fn the_join_answers_the_same_whatever_order_the_contributors_arrive() {
        // `pin-set-meet-order-independence`: a fold over a contributor set is the shape every
        // conversion seat uses, so the answer must not depend on which contributor is the head.
        let authored = InfluenceAccount::authored_before_contact();
        let influenced = InfluenceAccount::of_phase(phase());
        let untracked = InfluenceAccount::untracked();
        let forwards = [authored, influenced, untracked]
            .into_iter()
            .fold(authored, InfluenceAccount::join);
        let backwards = [untracked, influenced, authored]
            .into_iter()
            .fold(authored, InfluenceAccount::join);
        assert_eq!(forwards, backwards);
        assert_eq!(forwards, untracked);
        assert_eq!(influenced.join(influenced), influenced);
    }
}
