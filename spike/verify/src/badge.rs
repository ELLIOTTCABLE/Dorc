//! The badge set (`301` §5): an independent SET, never a ladder.
//!
//! Cross-states are real — a law can be `proved` but not `demonstrated`, `demonstrated` but
//! not `pinned` — so a scalar coverage score would force an order the evidence does not
//! have. Every badge is COMPUTED from evidence at gate time and never declared; what the
//! catalogue declares is the EXPECTATION, and the gate refuses a mismatch in either
//! direction (no silent demotion, no silent ambition).

use std::fmt;

/// One earnable kind of evidence.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Badge {
    /// The statement elaborates against `Generated/`: every name resolves and the types fit,
    /// so the law is speakable in the code's own vocabulary and cannot refer to stale
    /// structure.
    Elaborated,
    /// The in-unit instance battery is green AND non-vacuous — at least one positive witness
    /// with the precondition genuinely satisfied, plus the boundary battery.
    Interrogated,
    /// The paired Kani harness is green at its declared bounds, resolved by NAME against the
    /// real harness list rather than by string-matching.
    Pinned,
    /// A sorry-free proof of the statement's `Prop` exists in `Proofs/`.
    Proved,
    /// At least one tracked binding is green under the full battery, plus the non-vacuity
    /// certifications: reach (the bound loom's execution enters the cited seat) and
    /// load-bearing (a mutant scoped to the seat flips a bound assertion subset).
    Demonstrated,
    /// The owed mutation badge for the STATEMENTS themselves. Defined from day one and
    /// rendering `todo` until built, so the report nags structurally.
    KillTested,
}

impl Badge {
    /// Every badge, in report order. The report renders all of them for every law — an
    /// unrendered badge is exactly the halo the system exists to prevent.
    pub const ALL: [Self; 6] = [
        Self::Elaborated,
        Self::Interrogated,
        Self::Pinned,
        Self::Proved,
        Self::Demonstrated,
        Self::KillTested,
    ];

    /// The badge's spelling, as the report and the lock write it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Elaborated => "elaborated",
            Self::Interrogated => "interrogated",
            Self::Pinned => "pinned",
            Self::Proved => "proved",
            Self::Demonstrated => "demonstrated",
            Self::KillTested => "kill-tested",
        }
    }

    /// Whether computing this badge's evidence needs an external toolchain the ordinary
    /// gate does not run (Lean, Kani, `cargo-mutants`).
    ///
    /// This is the `301` §5 gate-tier split expressed once: the cheap tier compares only the
    /// badges it can genuinely recompute, and says so about the rest rather than trusting a
    /// committed value.
    #[must_use]
    pub const fn needs_external_engine(self) -> bool {
        match self {
            Self::Proved => false,
            Self::Elaborated
            | Self::Interrogated
            | Self::Pinned
            | Self::Demonstrated
            | Self::KillTested => true,
        }
    }
}

impl fmt::Display for Badge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// What the catalogue DECLARES about a badge — typed non-coverage rendered with the same
/// mechanical weight as coverage (the duvet steal).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Expectation {
    /// The badge is expected to be earned; the gate refuses if the evidence disagrees.
    Earned,
    /// Not built yet, and owed. Renders as a nag.
    Todo,
    /// Deliberately absent, with a reason a reader can weigh.
    Excepted(&'static str),
}

impl Expectation {
    /// The spelling the report writes.
    #[must_use]
    pub fn render(self) -> String {
        match self {
            Self::Earned => "earned".to_owned(),
            Self::Todo => "todo".to_owned(),
            Self::Excepted(why) => format!("excepted({why})"),
        }
    }
}

/// What the EVIDENCE says, once computed.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Evidence {
    /// The evidence is present and green.
    Earned,
    /// The evidence was looked for and is not there. The string names what was missing, so a
    /// refusal is actionable without re-running anything.
    Absent(String),
    /// This tier does not run the engine this badge's evidence comes from, so nothing was
    /// computed. NOT a value to compare against, and never a cached "earned".
    NotAtThisTier,
}

impl Evidence {
    /// The spelling the report writes.
    #[must_use]
    pub fn render(&self) -> String {
        match self {
            Self::Earned => "earned".to_owned(),
            Self::Absent(why) => format!("absent({why})"),
            Self::NotAtThisTier => "not-recomputed-here".to_owned(),
        }
    }

    /// Whether this evidence satisfies `expectation`.
    ///
    /// `NotAtThisTier` agrees with everything by construction: a tier that did not look
    /// cannot contradict, and pretending otherwise would turn the cheap gate into a
    /// rubber stamp for the expensive one's answers.
    #[must_use]
    pub fn agrees_with(&self, expectation: Expectation) -> bool {
        matches!(
            (self, expectation),
            (Self::NotAtThisTier, _)
                | (Self::Earned, Expectation::Earned)
                | (
                    Self::Absent(_),
                    Expectation::Todo | Expectation::Excepted(_)
                )
        )
    }
}
