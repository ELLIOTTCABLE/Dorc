//! `dorc-plan` — the elision path: decide, per command, run-or-skip, behind the
//! orientation locks of `Research/notes/165`.
//!
//! The catastrophic bug this crate is built to make *unrepresentable* is a wrong
//! skip: eliding a command that actually needed to run (`kFAIL-perform`). Three
//! locks, hardest first:
//!
//! * **`PhasedVerdict<P>`** (note 165 L1) — a host verdict carries its phase in
//!   the type, so a probe verdict cannot be silently consumed as an apply verdict,
//!   and [`Bias`] forces the `Unknown`-fold per phase. No code path folds
//!   `Unknown` to a skip.
//! * **[`SkipLicense`]** (note 165 L2) — the witness for the one irreversible verb
//!   (*elide*). Its fields are private, so the only way to obtain one is
//!   [`SkipLicense::prove_skippable`]; a plan emitter takes a `SkipLicense`, never
//!   a `bool`, so "skip" cannot be spelled without the proof.
//! * **`inv-must-may` + the ambient gate**, enforced inside `prove_skippable`:
//!   only a [`Grade::Must`] fact that `analysis` classified [`SkipClass::EstablishAmbient`]
//!   (no upstream same-run mutation reaches it — note 162 O-1) and that the host
//!   probe found `Converged` may be elided.
//!
//! Determinism (`inv-determinism`): a pure function of its inputs; the host
//! verdict is injected (the real host / `hostsim` is a later seam).

#![forbid(unsafe_code)]

use core::marker::PhantomData;

use dorc_analysis::effect::{FactKey, SkipClass};
use dorc_core::{Grade, Verdict};

// ===========================================================================
// Phase markers + the Unknown-fold bias (note 165 L1)
// ===========================================================================

/// Type-level marker for the **probe** phase — distinct from the runtime
/// [`dorc_core::Phase`] enum. Uninhabited: it exists only to parameterise a type,
/// never to be constructed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Probe {}

/// Type-level marker for the **apply** phase. See [`Probe`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Apply {}

/// The definite action a verdict folds to once `Unknown` is resolved per phase.
/// A plan may elide a command only when it holds a [`Resolved::Skippable`], and
/// `Skippable` is reachable ONLY from a definite [`Verdict::Converged`] — never
/// from `Unknown` (that is the wrong-skip this crate forbids).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolved {
    /// The command's effect is already established → it may be elided.
    Skippable,
    /// The command must run (diverged, or the conservative fold of unknown).
    Run,
}

/// The phase-keyed safe default for an `Unknown` verdict (welded `kFAIL`). No
/// implementation may return [`Resolved::Skippable`] — folding `Unknown` to a skip
/// is the catastrophic error (note 165). Keeping the rule in one trait, one impl
/// per phase, means it is reviewed in exactly one place instead of re-derived at
/// every `match` on a verdict.
pub trait Bias {
    /// What an `Unknown` verdict folds to in this phase. Must never be `Skippable`.
    fn on_unknown() -> Resolved;
}

impl Bias for Probe {
    /// Probe phase (`kFAIL-withhold`): an `Unknown` means the read-only check could
    /// not confirm convergence → treat as not-established → [`Resolved::Run`].
    fn on_unknown() -> Resolved {
        Resolved::Run
    }
}

impl Bias for Apply {
    /// Apply phase (`kFAIL-perform`): never skip a needed mutation → an `Unknown`
    /// verdict [`Resolved::Run`]s.
    fn on_unknown() -> Resolved {
        Resolved::Run
    }
}

/// A host convergence [`Verdict`], tagged with the phase that produced it. The
/// phase tag is the lock: a `PhasedVerdict<Probe>` cannot be passed where a
/// `PhasedVerdict<Apply>` is wanted, and [`resolve`](PhasedVerdict::resolve)
/// folds `Unknown` through the phase's [`Bias`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhasedVerdict<P: Bias> {
    raw: Verdict,
    _phase: PhantomData<P>,
}

impl<P: Bias> PhasedVerdict<P> {
    /// Tag a raw host verdict with this phase.
    #[must_use]
    pub fn new(raw: Verdict) -> Self {
        Self { raw, _phase: PhantomData }
    }

    /// Fold to a definite action; `Unknown` uses this phase's [`Bias`]. The only
    /// route to [`Resolved::Skippable`] is a definite [`Verdict::Converged`].
    #[must_use]
    pub fn resolve(self) -> Resolved {
        match self.raw {
            Verdict::Converged => Resolved::Skippable,
            Verdict::Diverged => Resolved::Run,
            Verdict::Unknown => P::on_unknown(),
        }
    }

    /// The underlying three-valued verdict (for display / provenance).
    #[must_use]
    pub fn raw(self) -> Verdict {
        self.raw
    }
}

// ===========================================================================
// The skip witness (note 165 L2)
// ===========================================================================

/// Why a skip was licensed — the audit trail a plan UI greys-out as the "why"
/// (note 165 L2). Readable, but only ever constructed inside
/// [`SkipLicense::prove_skippable`], so every field reflects a checked condition.
#[derive(Debug, Clone)]
pub struct Derivation {
    /// The fact whose established-ness licenses the skip.
    pub fact: FactKey,
    /// `analysis` classified this command [`SkipClass::EstablishAmbient`]: no
    /// upstream same-run mutation reaches it (the W5 ambient gate, note 162 O-1).
    pub ambient: bool,
    /// The fact is oracle-declared [`Grade::Must`] (a mined `May` never licenses —
    /// `inv-must-may`).
    pub grade: Grade,
    /// The host probe found the fact already holds ([`Verdict::Converged`]).
    pub verdict: Verdict,
}

/// The witness authorising the one irreversible verb — *elide a command*. Its
/// fields are private, so the ONLY way to obtain one is
/// [`prove_skippable`](SkipLicense::prove_skippable); a plan emitter accepts a
/// `SkipLicense`, never a `bool`, so a skip cannot be spelled without the proof
/// (note 165 L2). Carries its [`Derivation`] for provenance.
#[derive(Debug, Clone)]
pub struct SkipLicense {
    fact: FactKey,
    derivation: Derivation,
}

impl SkipLicense {
    /// Mint a license iff EVERY condition holds; otherwise `None` — the
    /// conservative *run-it* direction (note 165 L2 / `inv-must-may` / the ambient
    /// gate):
    ///
    /// 1. the command's effect is [`SkipClass::EstablishAmbient`] (classify proved
    ///    no upstream same-run mutation reaches it — else its resting state is
    ///    stale and the probe is not authoritative);
    /// 2. the fact is [`Grade::Must`] (oracle-declared; a `May` hint never licenses);
    /// 3. the probe verdict folds to [`Resolved::Skippable`] — a definite
    ///    `Converged`; `Diverged` and (via [`Bias`]) `Unknown` do not.
    #[must_use]
    pub fn prove_skippable(
        class: &SkipClass,
        grade: Grade,
        verdict: PhasedVerdict<Probe>,
    ) -> Option<SkipLicense> {
        let SkipClass::EstablishAmbient(fact) = class else {
            return None;
        };
        if grade != Grade::Must {
            return None;
        }
        if verdict.resolve() != Resolved::Skippable {
            return None;
        }
        Some(SkipLicense {
            fact: *fact,
            derivation: Derivation {
                fact: *fact,
                ambient: true,
                grade,
                verdict: Verdict::Converged,
            },
        })
    }

    /// The fact whose established-ness licensed this skip.
    #[must_use]
    pub fn fact(&self) -> FactKey {
        self.fact
    }

    /// The audit trail (the greyed-out "why" for the plan UI).
    #[must_use]
    pub fn derivation(&self) -> &Derivation {
        &self.derivation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{Interner, KindId, OpaqueToken};

    fn nginx_fact() -> FactKey {
        let mut i = Interner::default();
        FactKey {
            kind: KindId(i.intern("package")),
            entity: OpaqueToken(i.intern("nginx")),
        }
    }

    #[test]
    fn license_minted_for_ambient_must_converged() {
        // The one path that authorises a skip: classify said ambient, the oracle
        // declared Must, and the probe found it already holds.
        let f = nginx_fact();
        let Some(lic) = SkipLicense::prove_skippable(
            &SkipClass::EstablishAmbient(f),
            Grade::Must,
            PhasedVerdict::<Probe>::new(Verdict::Converged),
        ) else {
            panic!("ambient + must + converged must license a skip");
        };
        assert_eq!(lic.fact(), f);
        assert!(lic.derivation().ambient);
        assert_eq!(lic.derivation().verdict, Verdict::Converged);
    }

    #[test]
    fn no_license_when_verdict_not_converged() {
        // Diverged ⇒ run; Unknown ⇒ run (the Bias fold) — neither licenses.
        let f = nginx_fact();
        for v in [Verdict::Diverged, Verdict::Unknown] {
            assert!(
                SkipLicense::prove_skippable(
                    &SkipClass::EstablishAmbient(f),
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(v),
                )
                .is_none(),
                "verdict {v:?} must NOT license a skip"
            );
        }
    }

    #[test]
    fn no_license_for_may_grade() {
        // inv-must-may: a mined/distributional May-grade fact never authorises a skip.
        let f = nginx_fact();
        assert!(SkipLicense::prove_skippable(
            &SkipClass::EstablishAmbient(f),
            Grade::May,
            PhasedVerdict::<Probe>::new(Verdict::Converged),
        )
        .is_none());
    }

    #[test]
    fn no_license_for_written_or_mustrun_class() {
        // Only EstablishAmbient is elidable. EstablishWritten (an upstream same-run
        // mutation reaches it) and MustRun must run even with a Converged probe.
        let f = nginx_fact();
        for class in [SkipClass::EstablishWritten(f), SkipClass::MustRun] {
            assert!(
                SkipLicense::prove_skippable(
                    &class,
                    Grade::Must,
                    PhasedVerdict::<Probe>::new(Verdict::Converged),
                )
                .is_none(),
                "{class:?} must not license a skip"
            );
        }
    }

    #[test]
    fn unknown_folds_to_run_in_both_phases() {
        // The kFAIL fold: Unknown is never Skippable, in either phase.
        assert_eq!(PhasedVerdict::<Probe>::new(Verdict::Unknown).resolve(), Resolved::Run);
        assert_eq!(PhasedVerdict::<Apply>::new(Verdict::Unknown).resolve(), Resolved::Run);
        // Sanity on the definite verdicts.
        assert_eq!(PhasedVerdict::<Probe>::new(Verdict::Converged).resolve(), Resolved::Skippable);
        assert_eq!(PhasedVerdict::<Apply>::new(Verdict::Diverged).resolve(), Resolved::Run);
    }
}
