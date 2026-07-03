//! `core::claim` — the **claim-tier trust algebra** (the round-24 arc-win; `24A` arc-win /
//! `24D §1`). One phantom-indexed type, [`Claim<T, P>`], carries a payload *together with the
//! tier of authority it may act at*. It is the compile-error successor to five hand-maintained
//! prose disciplines that all reduce to one question — *what tier of claim may feed what tier of
//! decision* — the shape of every crisis in the corpus (233 = silence consumed as license; the
//! dangerous middle = partial-description as completeness; the HEAD vouchless elide = a fact-tier
//! observation consumed as a mutation-skip license; plus vouch-never-in-fact-plane,
//! May-never-licenses, at-least-never-at-most).
//!
//! # The three tiers ([`Tier`], sealed)
//!
//! * [`FactTier`] — a probe-measured / derived observation. Licenses READ-reproduction (a
//!   consumed value substituted from what the probe actually saw), NEVER a mutation-skip.
//! * [`JudgmentTier`] — an authored acceptance: the verdict-function authoring act
//!   (rul24-vouch-is-verdict-authoring), a footprint, a bridge. Licenses per its [`Rung`].
//! * [`SilenceTier`] — an explicit absence-of-claim. Licenses NOTHING — *representable and
//!   useless*. This is the anti-233 move made typed: "default"/"unmarked" is a value you can
//!   spell and that buys you nothing, never an ambiguous absence a later site reads as consent.
//!
//! # The four unrepresentability properties (each a COMPILE error, mechanism noted inline)
//!
//! * **TC-tier-1 — demotion is one-way, toward display.** The ONLY tier coercion is
//!   [`Claim::demote`] (any tier → [`SilenceTier`]). There is NO inverse: no
//!   `FactTier → JudgmentTier`, no `SilenceTier → anything`, no upgrade. *Mechanism:* no such
//!   `fn`/`impl` is written anywhere in this module — an upgrade is un-spellable because the
//!   only tier-changing operation lowers, and the tier minters ([`Claim::measured`] etc.) build
//!   a claim FROM a payload, never FROM another tier's claim.
//! * **TC-tier-2 — license-mints DEMAND their tier in the signature.** A mutation-skip mint
//!   (the elide/guard license, in `plan`) takes a [`Judgment<_>`] by value; a read-reproduction
//!   mint takes a [`Fact<_>`]. A [`SilenceTier`] claim satisfies NEITHER signature ⇒ "silence
//!   licenses nothing" is a *type error*, not a runtime check. *Mechanism:* the mint signatures
//!   (elsewhere) name the concrete tier; this module gives no tier-erasing view that would let
//!   one claim stand in for another.
//! * **TC-tier-3 — no function from a [`JudgmentTier`] claim into a fact-plane value type.**
//!   vouch-never-enters-the-fact-plane (rul-guard-license) becomes: there is no
//!   `Judgment<_> → {Observable, Verdict-as-fact, …}` path. A judgment can inform a license; it
//!   can never become an ambient fact another site's reasoning reads. *Mechanism:* a judgment
//!   claim exposes its payload only as `&P`/`P` where `P` is a judgment-plane descriptor (a
//!   vouch), and no `impl` in the tree converts that descriptor (or the claim) into a fact-plane
//!   type — contrast [`Fact<P>`], whose [`observation`](Claim::observation) IS the sanctioned
//!   fact-plane exit.
//! * **TC-tier-4 — the rung lives INSIDE [`JudgmentTier`], and is OPEN.** A judgment claim's
//!   payload is a [`Vouched<P>`] — a vouch PLUS a [`Rung`] (currently always [`Rung::Both`]). A
//!   fact/silence claim's payload is the bare value, with no rung anywhere; the rung place is
//!   *structurally* judgment-only. The future rung-split (the wary-engineer hatch — guards but
//!   not elisions) is an ADD of a [`Rung`] variant, never a re-signing of the mints (which carry
//!   the rung, never match on it).
//!
//! # Honest bound (rul24-overtype addendum — the uncheckable half rides here, verbatim intent)
//!
//! Types protect the PLUMBING (no claim is ever consumed above its authority); they do NOT and
//! cannot make a judgment TRUE. 233 stays permanent — a footprint or vouch can be
//! honestly-authored and still wrong; the algebra guarantees only that a wrong judgment is
//! consumed at the tier it was offered, blamed to its author, never silently promoted.

use core::marker::PhantomData;

/// The sealing module (`24D §1`): the [`Tier`] trait's supertrait lives here, private, so no
/// crate outside `core` can name it — and therefore none can add a fourth tier. The closed
/// three-tier set is the whole point (a rogue `TrustMeTier` would be exactly the 233 sin).
mod sealed {
    pub trait Sealed {}
}

/// A tier of epistemic authority a [`Claim`] may act at. **Sealed** (supertrait
/// [`sealed::Sealed`] is private to `core`): [`FactTier`] / [`JudgmentTier`] / [`SilenceTier`]
/// are the ONLY inhabitants — no downstream crate can mint a new tier of trust (`24D §1`).
pub trait Tier: sealed::Sealed {}

/// **Fact tier** — a probe-measured / derived observation. Licenses READ-reproduction only
/// (a consumed value reproduced from what the probe actually saw, `inv-probe-sourced-values`),
/// NEVER the erasure of a mutation (that is [`JudgmentTier`]'s alone — proviso-read-erasure,
/// `24A §1c`). Uninhabited: it exists only to index a type. (The derives are vacuous over zero
/// variants; they exist so [`Claim`]'s own derives, which conservatively bound `T`, apply.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactTier {}

/// **Judgment tier** — an authored acceptance. The verdict-function authoring act IS the vouch
/// (rul24-vouch-is-verdict-authoring); a footprint and a bridge are the same tier. Licenses per
/// its [`Rung`]. A judgment is *fallible and attributed* — never a fact (the honest bound).
/// Uninhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JudgmentTier {}

/// **Silence tier** — an explicit absence-of-claim. Licenses NOTHING. Representable so that
/// "default"/"unmarked" is a value you can *hold and pass* and that buys nothing, never an
/// ambiguous absence a later site mis-reads as consent (the anti-233 move typed, `24D §1`).
/// Uninhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SilenceTier {}

impl sealed::Sealed for FactTier {}
impl sealed::Sealed for JudgmentTier {}
impl sealed::Sealed for SilenceTier {}
impl Tier for FactTier {}
impl Tier for JudgmentTier {}
impl Tier for SilenceTier {}

/// The license-ladder **rung** a [`JudgmentTier`] claim authorizes — OPEN (`ORACLE_PROVIDES`
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
/// zero-size phantom, so a [`Fact<P>`] and a [`Judgment<P>`] are DISTINCT types the compiler
/// keeps apart at every boundary — a fact cannot be passed where a judgment is demanded, or vice
/// versa (TC-tier-2). The payload is PRIVATE: the only ways in are the tier-specific minters, and
/// the only ways out are the tier-specific accessors (a judgment claim has no fact-plane exit —
/// TC-tier-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Claim<T: Tier, P> {
    payload: P,
    _tier: PhantomData<T>,
}

/// A **fact-tier** claim (the common shape): a probe-measured / derived observation.
pub type Fact<P> = Claim<FactTier, P>;

/// A **judgment-tier** claim (the common shape): a vouch at a [`Rung`] — the payload is a
/// [`Vouched<P>`], which is why the rung is structurally judgment-only (TC-tier-4).
pub type Judgment<P> = Claim<JudgmentTier, Vouched<P>>;

/// The judgment-tier payload: a caller's vouch descriptor PLUS the reserved [`Rung`] (TC-tier-4).
/// This wrapper is ONLY ever the payload of a judgment claim — a fact/silence claim's payload is
/// the bare value, no rung anywhere — which is how "the rung lives inside [`JudgmentTier`]" is
/// structural, not conventional. Fields private: the sole constructor is [`Claim::authored`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vouched<P> {
    vouch: P,
    rung: Rung,
}

impl<P> Vouched<P> {
    /// The vouch descriptor (a judgment-plane value — NOT a fact-plane type, TC-tier-3).
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

impl<P> Claim<FactTier, P> {
    /// Mint a **fact-tier** claim — a probe-measured / derived observation (the only way to
    /// spell fact-tier authority). Naming the tier at the mint is the point: a fact is authored
    /// as a fact, never coerced up from silence or down from judgment.
    #[must_use]
    pub fn measured(payload: P) -> Self {
        Self {
            payload,
            _tier: PhantomData,
        }
    }

    /// The measured payload — the sanctioned **fact-plane exit** (TC-tier-3's contrast case). A
    /// fact licenses read-reproduction, so its value may enter another site's reasoning. This
    /// accessor exists ONLY on [`FactTier`]; a judgment claim has no analogue, which is exactly
    /// how "a vouch never becomes an ambient fact" is a compile fact and not a discipline.
    #[must_use]
    pub fn observation(&self) -> &P {
        &self.payload
    }

    /// Consume the fact claim into its measured payload (the by-value fact-plane exit).
    #[must_use]
    pub fn into_observation(self) -> P {
        self.payload
    }
}

impl<P> Claim<JudgmentTier, Vouched<P>> {
    /// Mint a **judgment-tier** claim — an authored acceptance at the given [`Rung`] (the
    /// verdict-function authoring act, a footprint, a bridge). The rung is carried, not selected
    /// by a spelling this stage owns (TC-tier-4); a caller passes [`Rung::Both`] today.
    #[must_use]
    pub fn authored(vouch: P, rung: Rung) -> Self {
        Self {
            payload: Vouched { vouch, rung },
            _tier: PhantomData,
        }
    }

    /// The vouch payload, for a **license mint only** (`plan`'s elide/guard license). Returns the
    /// judgment-plane descriptor `P` (a vouch), which the caller turns into a license *witness* —
    /// it is NOT and must never become a fact-plane value (TC-tier-3: no `impl` in the tree maps
    /// `P` or this claim into `Observable`/`Verdict`-as-fact/any ambient fact type).
    #[must_use]
    pub fn vouch(&self) -> &P {
        self.payload.vouch()
    }

    /// The reserved [`Rung`] (TC-tier-4) — currently always [`Rung::Both`].
    #[must_use]
    pub fn rung(&self) -> Rung {
        self.payload.rung()
    }

    /// Consume the judgment claim into its vouch payload, for the mint that takes it by value
    /// (TC-tier-2: the mint DEMANDS a [`Judgment<_>`], so calling this IS the tier check).
    #[must_use]
    pub fn into_vouch(self) -> P {
        self.payload.into_vouch()
    }
}

impl<P> Claim<SilenceTier, P> {
    /// Mint a **silence-tier** claim — an explicit absence-of-authority. Representable so that a
    /// caller can hold/pass "no claim here" as a value; it satisfies no license mint's signature
    /// (TC-tier-2), so it is spellable-and-useless by construction.
    #[must_use]
    pub fn silent(payload: P) -> Self {
        Self {
            payload,
            _tier: PhantomData,
        }
    }

    /// The payload of a silence claim — for DISPLAY only (silence licenses nothing, so there is
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
    /// anywhere: no `fn`/`impl` raises a claim's tier, so "promote a fact to a judgment" cannot
    /// be spelled. The payload survives (display still wants to show it); only the authority is
    /// erased.
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

    // A throwaway judgment-plane payload (a stand-in for the real `VerdictVouch` the verdict-fn
    // lift adds). The tests exercise the ALGEBRA's properties, tier-agnostic in payload.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Vouch(u32);

    #[test]
    fn fact_tier_exposes_its_observation() {
        // TC-tier-3 contrast: a FACT claim's payload IS a sanctioned fact-plane exit.
        let c = Fact::measured(7u32);
        assert_eq!(*c.observation(), 7);
        assert_eq!(c.into_observation(), 7);
    }

    #[test]
    fn judgment_tier_carries_a_rung_and_a_vouch() {
        let c = Judgment::authored(Vouch(3), Rung::Both);
        assert_eq!(c.rung(), Rung::Both);
        assert_eq!(c.vouch(), &Vouch(3));
        // The by-value mint-exit (TC-tier-2: a mint consumes exactly this).
        assert_eq!(c.into_vouch(), Vouch(3));
    }

    #[test]
    fn silence_is_representable_and_display_only() {
        // The anti-233 move: silence is a VALUE (spellable) that feeds no decision (useless).
        let c = Claim::<SilenceTier, _>::silent(Vouch(9));
        assert_eq!(c.for_display(), &Vouch(9));
    }

    #[test]
    fn demotion_is_one_way_toward_display() {
        // TC-tier-1: every tier demotes to Silence; the payload survives for display.
        let fact = Fact::measured(1u32);
        let s1: Claim<SilenceTier, u32> = fact.demote();
        assert_eq!(s1.for_display(), &1);

        let judgment = Judgment::authored(Vouch(2), Rung::Both);
        let s2: Claim<SilenceTier, Vouched<Vouch>> = judgment.demote();
        assert_eq!(s2.for_display().vouch(), &Vouch(2));

        // The INVERSE is unrepresentable: there is no method turning `s1`/`s2` back into a
        // FactTier/JudgmentTier claim. This test can only assert the forward direction works;
        // the reverse's absence is a compile fact (no such fn exists — grep this module).
    }

    #[test]
    fn distinct_tiers_are_distinct_types() {
        // TC-tier-2 in miniature: a fact and a judgment claim are NOT interchangeable — a fn
        // taking one rejects the other at compile time. We witness the distinctness by
        // round-tripping each through its OWN accessor (a cross-call would not compile).
        let fact = Fact::measured(5u32);
        let judg = Judgment::authored(5u32, Rung::Both);
        assert_eq!(*fact.observation(), 5);
        assert_eq!(*judg.vouch(), 5);
    }
}
