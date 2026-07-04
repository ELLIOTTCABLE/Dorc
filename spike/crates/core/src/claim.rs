//! `core::claim` — the **claim-tier trust algebra** (the round-24 arc-win; `24A` arc-win /
//! `24D §1`). One phantom-indexed type, [`Claim<T, P>`], carries a payload *together with the
//! tier of authority it may act at*. It is the compile-error successor to five hand-maintained
//! prose disciplines that all reduce to one question — *what tier of claim may feed what tier of
//! decision* — the shape of every crisis in the corpus (233 = silence consumed as license; the
//! dangerous middle = partial-description as completeness; the HEAD vouchless elide = a
//! by-observation claim consumed as a mutation-skip license; plus vouch-never-in-fact-plane,
//! May-never-licenses, at-least-never-at-most).
//!
//! # The vocabulary (act/source-based; `24D §6` rul24-tier-names)
//!
//! A claim is read "I hold this **by observation** / **by vouch** / **by silence**." The names
//! are the SOURCE ACT, not an identity noun, so a blocked build cannot be unstuck by relabelling
//! ("expected [`ByVouch`], found [`ByObservation`]" reads as obviously-wrong — you cannot *turn a
//! by-observation into a by-vouch*, that laundering IS the soundness hole this boundary stops).
//!
//! # The three tiers ([`Tier`], sealed)
//!
//! * [`ObservationTier`] — a probe-measured / derived observation. Licenses READ-reproduction (a
//!   consumed value substituted from what the probe actually saw), NEVER a mutation-skip.
//! * [`VouchTier`] — an authored acceptance: the verdict-function authoring act
//!   (rul24-vouch-is-verdict-authoring), a footprint, a bridge. Licenses per its [`Rung`].
//! * [`SilenceTier`] — an explicit absence-of-claim. Licenses NOTHING — *representable and
//!   useless*. This is the anti-233 move made typed: "default"/"unmarked" is a value you can
//!   spell and that buys you nothing, never an ambiguous absence a later site reads as consent.
//!
//! # The four unrepresentability properties (each a COMPILE error, mechanism noted inline)
//!
//! * **TC-tier-1 — demotion is one-way, toward display.** The ONLY tier coercion is
//!   [`Claim::demote`] (any tier → [`SilenceTier`]). There is NO inverse: no
//!   `ObservationTier → VouchTier`, no `SilenceTier → anything`, no upgrade. *Mechanism:* no such
//!   `fn`/`impl` is written anywhere in this module — an upgrade is un-spellable because the
//!   only tier-changing operation lowers, and the tier minters ([`Claim::observed`] etc.) build
//!   a claim FROM a payload, never FROM another tier's claim.
//! * **TC-tier-2 — license-mints DEMAND their tier in the signature.** A mutation-skip mint
//!   (the elide/guard license, in `plan`) takes a [`ByVouch<_>`] by value; a read-reproduction
//!   mint takes a [`ByObservation<_>`]. A [`SilenceTier`] claim satisfies NEITHER signature ⇒
//!   "silence licenses nothing" is a *type error*, not a runtime check. *Mechanism:* the mint
//!   signatures (elsewhere) name the concrete tier; this module gives no tier-erasing view that
//!   would let one claim stand in for another.
//! * **TC-tier-3 — no function from a [`VouchTier`] claim into a fact-plane value type.**
//!   vouch-never-enters-the-fact-plane (rul-guard-license) becomes: there is no
//!   `ByVouch<_> → {Observable, Verdict-as-fact, …}` path. A vouch can inform a license; it
//!   can never become an ambient fact another site's reasoning reads. *Mechanism:* a by-vouch
//!   claim exposes its payload only as `&P`/`P` where `P` is a vouch descriptor, and no `impl`
//!   in the tree converts that descriptor (or the claim) into a fact-plane type — contrast
//!   [`ByObservation<P>`], whose [`observation`](Claim::observation) IS the sanctioned
//!   fact-plane exit.
//! * **TC-tier-4 — the rung lives INSIDE [`VouchTier`], and is OPEN.** A by-vouch claim's
//!   payload is a [`VouchAndRung<P>`] — a vouch PLUS a [`Rung`] (currently always [`Rung::Both`]).
//!   A by-observation/by-silence claim's payload is the bare value, with no rung anywhere; the
//!   rung place is *structurally* vouch-only. The future rung-split (the wary-engineer hatch —
//!   guards but not elisions) is an ADD of a [`Rung`] variant, never a re-signing of the mints
//!   (which carry the rung, never match on it).
//!
//! # Honest bound (rul24-overtype addendum — the uncheckable half rides here, verbatim intent)
//!
//! Types protect the PLUMBING (no claim is ever consumed above its authority); they do NOT and
//! cannot make a vouch TRUE. 233 stays permanent — a footprint or vouch can be
//! honestly-authored and still wrong; the algebra guarantees only that a wrong claim is
//! consumed at the tier it was offered, blamed to its author, never silently promoted.

use core::marker::PhantomData;

/// The sealing module (`24D §1`): the [`Tier`] trait's supertrait lives here, private, so no
/// crate outside `core` can name it — and therefore none can add a fourth tier. The closed
/// three-tier set is the whole point (a rogue `TrustMeTier` would be exactly the 233 sin).
mod sealed {
    pub trait Sealed {}
}

/// A tier of epistemic authority a [`Claim`] may act at. **Sealed** (supertrait
/// [`sealed::Sealed`] is private to `core`): [`ObservationTier`] / [`VouchTier`] /
/// [`SilenceTier`] are the ONLY inhabitants — no downstream crate can mint a new tier of trust
/// (`24D §1`).
pub trait Tier: sealed::Sealed {}

/// **Observation tier** — a claim held BY OBSERVATION (probe-measured / derived). Licenses
/// READ-reproduction only (a consumed value reproduced from what the probe actually saw,
/// `inv-probe-sourced-values`), NEVER the erasure of a mutation (that is [`VouchTier`]'s alone —
/// proviso-read-erasure, `24A §1c`). Uninhabited: it exists only to index a type. (The derives
/// are vacuous over zero variants; they exist so [`Claim`]'s own derives, which conservatively
/// bound `T`, apply.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationTier {}

/// **Vouch tier** — a claim held BY VOUCH (an authored acceptance). The verdict-function
/// authoring act IS the vouch (rul24-vouch-is-verdict-authoring); a footprint and a bridge are
/// the same tier. Licenses per its [`Rung`]. A vouch is *fallible and attributed* — never a fact
/// (the honest bound). Uninhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VouchTier {}

/// **Silence tier** — a claim held BY SILENCE (an explicit absence-of-claim). Licenses NOTHING.
/// Representable so that "default"/"unmarked" is a value you can *hold and pass* and that buys
/// nothing, never an ambiguous absence a later site mis-reads as consent (the anti-233 move
/// typed, `24D §1`). Uninhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SilenceTier {}

impl sealed::Sealed for ObservationTier {}
impl sealed::Sealed for VouchTier {}
impl sealed::Sealed for SilenceTier {}
impl Tier for ObservationTier {}
impl Tier for VouchTier {}
impl Tier for SilenceTier {}

/// The license-ladder **rung** a [`VouchTier`] claim authorizes — OPEN (`ORACLE_PROVIDES`
/// provides-license; `24D §4`). The ladder is rung-0 display / rung-1 in-position=guard /
/// rung-2 carried=elide; the rung-SPLIT (the wary-engineer's hatch — an author who licenses
/// guards but NOT elisions) is human-reserved. **Currently ALWAYS [`Rung::Both`]**: one
/// verdict-function authoring act licenses both guard and elide (rul24-vouch-is-verdict-authoring).
///
/// TC-tier-4: this is the reserved *place* for the rung. A future rung-1-only hatch slots in as a
/// NEW variant here, and because the license mints CARRY a `Rung` but never MATCH on it, that
/// addition re-signs nothing. Do NOT invent a rung-SELECTION spelling (`case`-arm grain, a flag)
/// — that is the human's open call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rung {
    /// Authorizes BOTH rungs — guard (in-position) and elide (carried). The current weld.
    Both,
}

/// A payload `P` wrapped with the [`Tier`] `T` of authority it carries (`24D §1`). The tier is a
/// zero-size phantom, so a [`ByObservation<P>`] and a [`ByVouch<P>`] are DISTINCT types the
/// compiler keeps apart at every boundary — a by-observation claim cannot be passed where a
/// by-vouch is demanded, or vice versa (TC-tier-2). The payload is PRIVATE: the only ways in are
/// the tier-specific minters, and the only ways out are the tier-specific accessors (a by-vouch
/// claim has no fact-plane exit — TC-tier-3).
///
/// **When-blocked (rul24-critical-type-docs, `24D §6`):** if this type blocks your build — a mint
/// wants a [`ByVouch`] and you hold a [`ByObservation`]/[`BySilence`] — you likely have the WRONG
/// claim. Do NOT convert it to satisfy the signature (that laundering IS the soundness hole this
/// boundary prevents); obtain the real vouch (author the verdict function), or let the command run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Claim<T: Tier, P> {
    payload: P,
    _tier: PhantomData<T>,
}

/// A claim held **by observation** (the common shape): a probe-measured / derived observation.
pub type ByObservation<P> = Claim<ObservationTier, P>;

/// A claim held **by vouch** (the common shape): a vouch at a [`Rung`] — the payload is a
/// [`VouchAndRung<P>`], which is why the rung is structurally vouch-only (TC-tier-4).
pub type ByVouch<P> = Claim<VouchTier, VouchAndRung<P>>;

/// A claim held **by silence** (the common shape): an explicit absence-of-authority.
pub type BySilence<P> = Claim<SilenceTier, P>;

/// The vouch-tier payload: a caller's vouch descriptor PLUS the reserved [`Rung`] (TC-tier-4).
/// This wrapper is ONLY ever the payload of a [`ByVouch`] claim — a by-observation/by-silence
/// claim's payload is the bare value, no rung anywhere — which is how "the rung lives inside
/// [`VouchTier`]" is structural, not conventional. Fields private: the sole constructor is
/// [`Claim::vouched`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VouchAndRung<P> {
    vouch: P,
    rung: Rung,
}

impl<P> VouchAndRung<P> {
    /// The vouch descriptor (a vouch-plane value — NOT a fact-plane type, TC-tier-3).
    #[must_use]
    pub fn vouch(&self) -> &P {
        &self.vouch
    }

    /// The reserved [`Rung`] (TC-tier-4) — currently always [`Rung::Both`].
    #[must_use]
    pub fn rung(&self) -> Rung {
        self.rung
    }

    /// Consume into the vouch descriptor (the by-value exit a license mint uses).
    #[must_use]
    pub fn into_vouch(self) -> P {
        self.vouch
    }
}

impl<P> Claim<ObservationTier, P> {
    /// Mint a claim held **by observation** — a probe-measured / derived observation (the only
    /// way to spell observation-tier authority). Naming the tier at the mint is the point: an
    /// observation is authored as an observation, never coerced up from silence or down from a
    /// vouch.
    #[must_use]
    pub fn observed(payload: P) -> Self {
        Self {
            payload,
            _tier: PhantomData,
        }
    }

    /// The observed payload — the sanctioned **fact-plane exit** (TC-tier-3's contrast case). An
    /// observation licenses read-reproduction, so its value may enter another site's reasoning.
    /// This accessor exists ONLY on [`ObservationTier`]; a by-vouch claim has no analogue, which
    /// is exactly how "a vouch never becomes an ambient fact" is a compile fact and not a
    /// discipline.
    #[must_use]
    pub fn observation(&self) -> &P {
        &self.payload
    }

    /// Consume the by-observation claim into its measured payload (the by-value fact-plane exit).
    #[must_use]
    pub fn into_observation(self) -> P {
        self.payload
    }
}

impl<P> Claim<VouchTier, VouchAndRung<P>> {
    /// Mint a claim held **by vouch** — an authored acceptance at the given [`Rung`] (the
    /// verdict-function authoring act, a footprint, a bridge). The rung is carried, not selected
    /// by a spelling this stage owns (TC-tier-4); a caller passes [`Rung::Both`] today.
    #[must_use]
    pub fn vouched(vouch: P, rung: Rung) -> Self {
        Self {
            payload: VouchAndRung { vouch, rung },
            _tier: PhantomData,
        }
    }

    /// The vouch payload, for a **license mint only** (`plan`'s elide/guard license). Returns the
    /// vouch descriptor `P`, which the caller turns into a license *witness* — it is NOT and must
    /// never become a fact-plane value (TC-tier-3: no `impl` in the tree maps `P` or this claim
    /// into `Observable`/`Verdict`-as-fact/any ambient fact type).
    #[must_use]
    pub fn vouch(&self) -> &P {
        self.payload.vouch()
    }

    /// The reserved [`Rung`] (TC-tier-4) — currently always [`Rung::Both`].
    #[must_use]
    pub fn rung(&self) -> Rung {
        self.payload.rung()
    }

    /// Consume the by-vouch claim into its vouch payload, for the mint that takes it by value
    /// (TC-tier-2: the mint DEMANDS a [`ByVouch<_>`], so calling this IS the tier check).
    #[must_use]
    pub fn into_vouch(self) -> P {
        self.payload.into_vouch()
    }
}

impl<P> Claim<SilenceTier, P> {
    /// Mint a claim held **by silence** — an explicit absence-of-authority. Representable so that
    /// a caller can hold/pass "no claim here" as a value; it satisfies no license mint's signature
    /// (TC-tier-2), so it is spellable-and-useless by construction.
    #[must_use]
    pub fn silent(payload: P) -> Self {
        Self {
            payload,
            _tier: PhantomData,
        }
    }

    /// The payload of a by-silence claim — for DISPLAY only (silence licenses nothing, so there is
    /// no decision this can feed). Named to make the read-site obviously inert.
    #[must_use]
    pub fn for_display(&self) -> &P {
        &self.payload
    }
}

impl<T: Tier, P> Claim<T, P> {
    /// **TC-tier-1 — the one-way coercion, toward display.** Demote a claim of ANY tier to
    /// [`SilenceTier`], stripping its authority (what a display/why-lens surface wants — the
    /// tier no longer matters once nothing is being licensed). There is deliberately NO inverse
    /// anywhere: no `fn`/`impl` raises a claim's tier, so "promote a by-observation into a
    /// by-vouch" cannot be spelled. The payload survives (display still wants to show it); only
    /// the authority is erased.
    #[must_use]
    pub fn demote(self) -> Claim<SilenceTier, P> {
        Claim {
            payload: self.payload,
            _tier: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A throwaway vouch-plane payload (a stand-in for the real `VerdictVouch` the verdict-fn
    // lift adds). The tests exercise the ALGEBRA's properties, tier-agnostic in payload.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Vouch(u32);

    #[test]
    fn observation_tier_exposes_its_observation() {
        // TC-tier-3 contrast: a by-observation claim's payload IS a sanctioned fact-plane exit.
        let c = ByObservation::observed(7u32);
        assert_eq!(*c.observation(), 7);
        assert_eq!(c.into_observation(), 7);
    }

    #[test]
    fn vouch_tier_carries_a_rung_and_a_vouch() {
        let c = ByVouch::vouched(Vouch(3), Rung::Both);
        assert_eq!(c.rung(), Rung::Both);
        assert_eq!(c.vouch(), &Vouch(3));
        // The by-value mint-exit (TC-tier-2: a mint consumes exactly this).
        assert_eq!(c.into_vouch(), Vouch(3));
    }

    #[test]
    fn silence_is_representable_and_display_only() {
        // The anti-233 move: silence is a VALUE (spellable) that feeds no decision (useless).
        let c = BySilence::silent(Vouch(9));
        assert_eq!(c.for_display(), &Vouch(9));
    }

    #[test]
    fn demotion_is_one_way_toward_display() {
        // TC-tier-1: every tier demotes to Silence; the payload survives for display.
        let observed = ByObservation::observed(1u32);
        let s1: BySilence<u32> = observed.demote();
        assert_eq!(s1.for_display(), &1);

        let vouched = ByVouch::vouched(Vouch(2), Rung::Both);
        let s2: BySilence<VouchAndRung<Vouch>> = vouched.demote();
        assert_eq!(s2.for_display().vouch(), &Vouch(2));

        // The INVERSE is unrepresentable: there is no method turning `s1`/`s2` back into a
        // by-observation/by-vouch claim. This test can only assert the forward direction works;
        // the reverse's absence is a compile fact (no such fn exists — grep this module).
    }

    #[test]
    fn distinct_tiers_are_distinct_types() {
        // TC-tier-2 in miniature: a by-observation and a by-vouch claim are NOT interchangeable —
        // a fn taking one rejects the other at compile time. We witness the distinctness by
        // round-tripping each through its OWN accessor (a cross-call would not compile).
        let observed = ByObservation::observed(5u32);
        let vouched = ByVouch::vouched(5u32, Rung::Both);
        assert_eq!(*observed.observation(), 5);
        assert_eq!(*vouched.vouch(), 5);
    }
}
