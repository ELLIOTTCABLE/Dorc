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
}
