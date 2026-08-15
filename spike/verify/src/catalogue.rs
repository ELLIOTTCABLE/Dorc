//! The catalogue (`301` §5): the generated, promote-gated lock.
//!
//! Organizing principle, borrowed deliberately from the loom: **the authored artifact is the
//! source of truth; everything else is derived and gate-checked; review is git diff.** The
//! authored artifacts here are the unit files and the bound looms' frontmatter; the lock in
//! [`crate::catalogue_lock`] is derived from them plus the badge set that was true at promote
//! time.
//!
//! Metadata is DISLOCATED here rather than living in the unit files, and that dislocation
//! carries its justification: the unit surface must stay diff-quiet so every change in it is a
//! meaningful, adjudicable event, while this lock churns freely with evidence.

use crate::badge::{Badge, Expectation};

/// One law's catalogue row.
#[derive(Clone, Copy, Debug)]
pub struct LawRow {
    /// The `DromedaryCase` slug: unit file stem, catalogue key, and demonstration-loom stem.
    pub slug: &'static str,
    /// The cited seat — the chokepoint function the law is about (`301` §5's `fn-seat` anchor).
    ///
    /// A seat is not minted by this system: it is the chokepoint the house architecture
    /// already mandates per behavior, cited here. It is also mostly DERIVABLE — the
    /// statement's `Prop` already names the derived definition of the same function — so the
    /// citation is a checked confirmation, not new information.
    pub seat: &'static str,
    /// Repo-relative path to the proof, when one is claimed.
    pub proof: Option<&'static str>,
    /// The Kani harness function this law pairs with, resolved by NAME against the real
    /// harness list. `None` until the Kani lane lands.
    pub harness: Option<&'static str>,
    /// The tracked bindings: bound looms and their law-relevant assertion subsets.
    pub bindings: &'static [Binding],
    /// The promoted badge expectation, one entry per [`Badge::ALL`] member, in that order.
    pub expected: [Expectation; Badge::ALL.len()],
}

impl LawRow {
    /// The expectation promoted for `badge`.
    #[must_use]
    pub fn expectation(&self, badge: Badge) -> Expectation {
        Badge::ALL
            .iter()
            .position(|b| *b == badge)
            .and_then(|i| self.expected.get(i).copied())
            .unwrap_or(Expectation::Todo)
    }
}

/// One accepted binding: a whole-product loom that demonstrates this law, and the exact
/// decisions its demonstration rests on.
///
/// A binding is never a bare pointer. The assertion SUBSET is what makes unrelated loom churn
/// (render text, other lines) harmless to the law while a re-bless or deletion that breaks the
/// bound subset trips the binder rather than passing silently.
#[derive(Clone, Copy, Debug)]
pub struct Binding {
    /// Repo-relative path to the bound loom case.
    pub case: &'static str,
    /// The law-relevant `(site, decision)` pairs, checked against the product's own
    /// machine-readable per-decision record — never against the loom's rendered goldens, which
    /// are render-plane and churny (`render-form-unwelded`).
    pub assertions: &'static [SiteDecision],
}

/// One `(SiteId → decision)` pair from a bound loom's decision record.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SiteDecision {
    /// The site, spelled as the decide plane spells it (`site-identity-is-decide-plane`):
    /// a leaf id, optionally with an in-loop member ordinal.
    pub site: &'static str,
    /// The decision that site must carry.
    pub decision: Decision,
}

/// The per-site outcomes a binding may assert over.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Decision {
    /// The line was replaced by an observable-preserving stand-in.
    Elide,
    /// A runtime check was inserted in front of the untouched original bytes.
    Guard,
    /// The line ships and executes.
    Run,
    /// The line survived a sparing/kill analysis.
    Survive,
}

/// The frontmatter key by which a loom PROPOSES itself as law evidence (`301` §2).
///
/// Deliberately alarming, because that is its whole job: a builder editing this case learns
/// FROM THE KEY that they are touching law-evidence, before they re-bless anything. The key is
/// only ever a proposal — acceptance into evidence is a catalogue promote, which is a spec-side
/// act under `301:law-spec-touch-frontier-human-only`. The binder checks the agreement BOTH
/// ways, so a proposal nobody accepted and an accepted binding whose case stopped declaring
/// itself are each visible.
pub const BINDING_KEY: &str = "tests-critical-law";
