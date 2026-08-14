//! Idiom census for the type-level devices in `spike/crates/core/src/{claim,coord}.rs`.
//! Not an extract — a distillation: the smallest faithful spelling of each device,
//! so a charon/aeneas failure names ONE idiom instead of a 400-line file. Shapes,
//! in source order: the sealed-trait fence, uninhabited marker enums, a
//! `PhantomData`-indexed newtype, `&'static str` payloads, an interned newtype
//! behind a comparison chokepoint, and a `#[derive(PartialOrd, Ord, Hash)]` set.

#![allow(dead_code)]

// --- claim.rs: the sealed-trait fence -------------------------------------
mod sealed {
    pub trait Sealed {}
}

pub trait Tier: sealed::Sealed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationTier {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VouchTier {}

impl sealed::Sealed for ObservationTier {}
impl sealed::Sealed for VouchTier {}
impl Tier for ObservationTier {}
impl Tier for VouchTier {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rung {
    Both,
}

// --- claim.rs: the PhantomData-indexed claim -------------------------------
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Claim<T: Tier, P> {
    payload: P,
    _tier: PhantomData<T>,
}

pub type ByObservation<P> = Claim<ObservationTier, P>;
pub type ByVouch<P> = Claim<VouchTier, VouchAndRung<P>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VouchAndRung<P> {
    vouch: P,
    rung: Rung,
}

impl<P> Claim<ObservationTier, P> {
    #[must_use]
    pub fn observed(payload: P) -> Self {
        Self {
            payload,
            _tier: PhantomData,
        }
    }

    #[must_use]
    pub fn observation(&self) -> &P {
        &self.payload
    }
}

impl<P> Claim<VouchTier, VouchAndRung<P>> {
    #[must_use]
    pub fn vouched(vouch: P, rung: Rung) -> Self {
        Self {
            payload: VouchAndRung { vouch, rung },
            _tier: PhantomData,
        }
    }

    #[must_use]
    pub fn into_vouch(self) -> P {
        self.payload.vouch
    }
}

// REWRITE 9 (mechanical-local): the `&'static str` literal is hoisted to a named
// `const`. Returning the literal INLINE aborts aeneas ("There should be no bottoms
// in the value"); bound to a const it translates as `toStr "..."` and the mint that
// carries it is unremarkable. See `strings/` for the four-way classification.
const VOUCH_SITE: &str = "systemctl.oracle.sh:12";

pub fn vouch_a_site() -> ByVouch<&'static str> {
    Claim::vouched(VOUCH_SITE, Rung::Both)
}

// --- coord.rs: interned newtypes behind one comparison chokepoint ----------
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectorId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct KindId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    kind: KindId,
    entity: u32,
    selector: Option<SelectorId>,
}

/// The ternary compare verdict — `same` feeds transport, `provably-disjoint` feeds
/// survival-sparing, `unknown` is the safe bottom for both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compare {
    Same,
    ProvablyDisjoint,
    Unknown,
}

/// The one chokepoint every selector comparison goes through. A selector-less
/// coordinate is the ⊤-selector and collides with every cell.
#[must_use]
pub fn selector_covers(a: Option<SelectorId>, b: Option<SelectorId>) -> bool {
    match (a, b) {
        (None, _) => true,
        (_, None) => true,
        (Some(x), Some(y)) => x == y,
    }
}

#[must_use]
pub fn compare(a: &Coord, b: &Coord) -> Compare {
    if a.kind != b.kind {
        return Compare::ProvablyDisjoint;
    }
    if a.entity != b.entity {
        return Compare::Unknown;
    }
    if selector_covers(a.selector, b.selector) {
        Compare::Same
    } else {
        Compare::ProvablyDisjoint
    }
}
