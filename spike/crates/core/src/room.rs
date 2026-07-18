//! `core::room` — the **invited-rooms type split** (descend-don't-license at the TYPE level;
//! `274` §1 · `271:rider-invited-rooms-typing` · `279f` §5).
//!
//! The `dorc:sh` prefix-mark surface (`dorc-sh-trio`) has three spellings, two of which produce
//! analysis that DESCENDS into a payload string. They differ in ONE load-bearing way — whether the
//! facts that descent produces may license anything:
//!
//! * **`dorc:sh -c '…'`** — an INVITED room. The author invited full analysis; a fact derived from
//!   the payload MAY mint a license (elide/guard).
//! * **bare `sh -c '…'`** — a HINT-ONLY room (THE escape hatch, the long-owed `unsafe`,
//!   `276:rul-unsafe-is-bare-sh`). Analysis descends only to produce HINTS ("this bit won't elide —
//!   did you want `dorc:`?"); a fact derived here may NEVER license, probe, or rearrange. Even a
//!   WRONG parse of an unlicensed payload cannot under-execute, because nothing it produces is
//!   admissible to a mint — the no-keyword option's omission-failure is structurally impossible.
//!
//! `274` §1 pins the enforcement tier: **TYPESYSTEM, not test-pin** — "incorrectness-inexpressible
//! type-differentiation between invited-room analysis (may mint licenses) and hint-only rooms (may
//! not)." This module is that differentiation. A [`RoomFact<R, P>`] carries a payload-derived belief
//! tagged with the [`Room`] it was derived in; the license-input exit ([`RoomFact::into_license_input`])
//! exists ONLY on the [`Invited`] room, so a [`HintOnly`] fact is refused by any license-consuming
//! signature at COMPILE TIME (the `279f` §5 pin — see the module-level `compile_fail` doctest below).
//!
//! # Composition with the claim-tier algebra ([`crate::claim`])
//!
//! The room split is ORTHOGONAL to — and the OUTER gate over — the claim tiers. A payload-inner
//! verdict is a [`ByVouch`](crate::ByVouch) (the inner command's oracle vouched); to feed `plan`'s
//! license mint it must clear BOTH gates: the room (invited?) THEN the tier (`ByVouch`?). The
//! canonical mint-input is therefore a `RoomFact<Invited, ByVouch<_>>`: the room gate hands out the
//! inner `ByVouch` via [`into_license_input`](RoomFact::into_license_input), which the existing
//! `ByVouch`-demanding mint then consumes. A `RoomFact<HintOnly, ByVouch<_>>` holds a perfectly good
//! vouch that it can NEVER surrender to a mint — the room forbids it before the tier is even asked.
//!
//! # The one-way, widen-only descent (`274` §12 finding-descent-edges-widen-only)
//!
//! Hint-lane edges may widen a `dorc bump` dependency walk's scope (more selected, more checked —
//! safe) but never NARROW it and never license. That is a consumer-side discipline; the type here
//! enforces the hard half — no license — structurally.
//!
//! # The compile-failure pin (`279f` §5, moved here from a prose commitment)
//!
//! ```compile_fail
//! use dorc_core::room::{RoomFact, mint_from_room};
//! use dorc_core::{ByVouch, Rung};
//!
//! // A hint-only room's fact — bare `sh -c '…'` descent (THE escape hatch).
//! let hint: RoomFact<dorc_core::room::HintOnly, ByVouch<u32>> =
//!     RoomFact::hint_only(ByVouch::vouched(7u32, Rung::Both));
//!
//! // A license mint demands an INVITED-room fact. Handing it a hint-only fact is a TYPE ERROR —
//! // "descend-don't-license" is a compile fact, not a runtime check (`274` §1).
//! let _ = mint_from_room(hint);
//! ```
//!
//! And the invited counterpart DOES compile (the positive control):
//!
//! ```
//! use dorc_core::room::{RoomFact, mint_from_room};
//! use dorc_core::{ByVouch, Rung};
//!
//! let invited: RoomFact<dorc_core::room::Invited, ByVouch<u32>> =
//!     RoomFact::invited(ByVouch::vouched(7u32, Rung::Both));
//! let vouch = mint_from_room(invited); // the invited room surrenders its inner ByVouch to the mint
//! assert_eq!(vouch.rung(), Rung::Both);
//! ```

use core::marker::PhantomData;

/// The sealing module (mirrors [`crate::claim`]'s): the [`Room`] trait's supertrait is private to
/// `core`, so no downstream crate can add a third room. The closed two-room set is the whole point —
/// a rogue `TrustMeRoom` would be exactly the descend-don't-license hole this split exists to close.
mod sealed {
    pub trait Sealed {}
}

/// A room a payload-derived fact was analyzed IN (`274` §1). **Sealed** (supertrait
/// [`sealed::Sealed`] is private to `core`): [`Invited`] and [`HintOnly`] are the ONLY inhabitants.
/// [`TAG`](Room::TAG) is the room's runtime witness (for diagnostics only — never a license branch).
pub trait Room: sealed::Sealed {
    /// The runtime witness of this room (diagnostics/provenance only; the license branch is the
    /// TYPE, never this value).
    const TAG: RoomTag;
}

/// The **invited** room — `dorc:sh -c '…'` (`271:rul-dorc-prefix-head-synthesis`). The author
/// invited full analysis license; a fact derived here MAY feed a license mint (via
/// [`RoomFact::into_license_input`]). Uninhabited: it exists only to index a type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Invited {}

/// The **hint-only** room — bare `sh -c '…'`, THE escape hatch / the long-owed `unsafe`
/// (`276:rul-unsafe-is-bare-sh`). Analysis descends for HINTS ONLY; a fact derived here licenses
/// NOTHING — it has no [`into_license_input`](RoomFact::into_license_input) exit, so a mint cannot
/// consume it (a compile fact, `279f` §5). Uninhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintOnly {}

impl sealed::Sealed for Invited {}
impl sealed::Sealed for HintOnly {}
impl Room for Invited {
    const TAG: RoomTag = RoomTag::Invited;
}
impl Room for HintOnly {
    const TAG: RoomTag = RoomTag::HintOnly;
}

/// A belief derived by descending into a payload string, tagged with the [`Room`] `R` it was derived
/// in (`274` §1). The room is a zero-size phantom, so `RoomFact<Invited, P>` and
/// `RoomFact<HintOnly, P>` are DISTINCT types the compiler keeps apart at every boundary. The
/// payload is PRIVATE: the only ways in are the room-specific minters ([`invited`](RoomFact::invited)
/// / [`hint_only`](RoomFact::hint_only)); the license-input exit is [`Invited`]-only.
///
/// **When-blocked:** if a license mint wants a `RoomFact<Invited, _>` and you hold a
/// `RoomFact<HintOnly, _>`, you are trying to license off a BARE-`sh` payload — the escape hatch that
/// licenses nothing by design (`276:rul-unsafe-is-bare-sh`). Do NOT re-tag it invited (there is no
/// such coercion — the descent that produced it was hint-only); the payload's site RUNS, and the
/// facts stay hints. To license, the author must write `dorc:sh` (invite the analysis).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomFact<R: Room, P> {
    payload: P,
    _room: PhantomData<R>,
}

impl<P> RoomFact<Invited, P> {
    /// Tag a payload-derived belief as coming from an INVITED room (`dorc:sh -c '…'`). Naming the
    /// room at the mint is the point: an invited fact is authored as invited (the `dorc:` prefix was
    /// present), never coerced up from a hint-only descent.
    #[must_use]
    pub fn invited(payload: P) -> Self {
        Self {
            payload,
            _room: PhantomData,
        }
    }

    /// Surrender the payload to a **license mint only** — the sanctioned exit that a license-consuming
    /// signature accepts. This accessor exists ONLY on [`Invited`]; a [`HintOnly`] fact has no
    /// analogue, which is exactly how "descend-don't-license" is a compile fact and not a discipline.
    /// The payload `P` is typically a [`ByVouch`](crate::ByVouch) — so clearing the room gate hands
    /// the inner claim to the tier gate, composing the two (see the module docs).
    #[must_use]
    pub fn into_license_input(self) -> P {
        self.payload
    }
}

impl<P> RoomFact<HintOnly, P> {
    /// Tag a payload-derived belief as coming from a HINT-ONLY room (bare `sh -c '…'`). Representable
    /// so descent can produce facts that drive HINTS; it has no license-input exit, so it is
    /// spellable-and-unlicensable by construction (the anti-omission move typed — the escape hatch
    /// cannot silently license).
    #[must_use]
    pub fn hint_only(payload: P) -> Self {
        Self {
            payload,
            _room: PhantomData,
        }
    }
}

impl<R: Room, P> RoomFact<R, P> {
    /// The payload for a HINT / display / `dorc bump` scope-widening read — available in EVERY room
    /// (hints are always allowed, even off an invited fact). Named to make the read-site obviously
    /// inert w.r.t. licensing: a value read through here can drive a did-you-mean diagnostic or widen
    /// a dependency walk, never a mint (`274` §12 finding-descent-edges-widen-only).
    #[must_use]
    pub fn for_hint(&self) -> &P {
        &self.payload
    }

    /// The room this fact was derived in, at runtime (for a diagnostic / why-lens that wants to name
    /// the room). Reads the room's compile-time [`TAG`](Room::TAG) — never a license branch.
    #[must_use]
    pub fn room(&self) -> RoomTag {
        R::TAG
    }
}

/// A runtime witness of which [`Room`] a [`RoomFact`] carries — for diagnostics/provenance only
/// (`inv-referent-agnostic` spirit: the engine never BRANCHES license decisions on this; the
/// branch is the TYPE). Obtained via [`RoomFact::room`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomTag {
    /// The fact came from `dorc:sh -c '…'` (invited).
    Invited,
    /// The fact came from bare `sh -c '…'` (hint-only, THE escape hatch).
    HintOnly,
}

/// A stand-in for a license mint over a room-tagged fact — it DEMANDS an [`Invited`] room and
/// surrenders the inner payload (the real mint, in `plan`, then demands the inner claim's tier).
/// Exists in `core` so the `279f` §5 compile-failure pin (module docs) has a license-consuming
/// signature to point at without a `core → plan` dependency. Its whole contract is the type: a
/// `RoomFact<HintOnly, _>` cannot be passed to it.
#[must_use]
pub fn mint_from_room<P>(fact: RoomFact<Invited, P>) -> P {
    fact.into_license_input()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ByVouch, Rung};

    #[test]
    fn invited_room_surrenders_its_payload_to_a_mint() {
        // The composition path: an invited-room fact carrying a ByVouch hands the vouch to the mint,
        // which the tier gate then consumes (the two gates in series).
        let fact = RoomFact::invited(ByVouch::vouched(3u32, Rung::Both));
        assert_eq!(fact.room(), RoomTag::Invited);
        let vouch = mint_from_room(fact);
        assert_eq!(vouch.rung(), Rung::Both);
        assert_eq!(*vouch.vouch(), 3);
    }

    #[test]
    fn hint_only_room_reads_for_hints_only() {
        // A hint-only fact holds a real vouch it can never surrender to a mint (no such method
        // exists — the compile_fail doctest is the proof). It reads only for hints.
        let fact = RoomFact::hint_only(ByVouch::vouched(9u32, Rung::Both));
        assert_eq!(fact.room(), RoomTag::HintOnly);
        assert_eq!(*fact.for_hint().vouch(), 9);
    }

    #[test]
    fn both_rooms_expose_the_hint_read() {
        // `for_hint` is available in EVERY room — hints are always allowed, even off an invited fact.
        let invited = RoomFact::invited(42u32);
        let hint = RoomFact::hint_only(42u32);
        assert_eq!(*invited.for_hint(), 42);
        assert_eq!(*hint.for_hint(), 42);
    }

    #[test]
    fn distinct_rooms_are_distinct_types() {
        // In miniature (the doctest is the real pin): the two rooms are not interchangeable. We
        // witness distinctness by round-tripping each through its OWN room's accessor — `mint_from_room`
        // takes only the invited one; the hint-only one has no license-input exit at all.
        let invited = RoomFact::invited(5u32);
        let hint = RoomFact::hint_only(5u32);
        assert_eq!(mint_from_room(invited), 5);
        assert_eq!(*hint.for_hint(), 5);
    }
}
