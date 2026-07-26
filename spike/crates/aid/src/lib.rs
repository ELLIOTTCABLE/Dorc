//! `dorc-aid` — the DESCRIBE plane (`288` §2a): narrative records, the diagnostic
//! catalog and its generated lock, the render seats, the why-lens, and the no-throw
//! [`Carrier`].
//!
//! `core` DECIDES, `aid` DESCRIBES. The dependency edge is `aid → core` and there is no
//! other: a `core → aid` edge would mean a decision reading a narration, which is exactly
//! what `two-plane-aid-law` forbids.
//!
//! Two invariants are load-bearing and enforced here:
//!
//! * **Determinism.** No clock, RNG, filesystem, or network — directly or transitively;
//!   the same bar `core` holds, so the whole pipeline stays a pure function of its inputs
//!   inside deterministic-simulation tests.
//! * **No-throw stages (`dn-7`).** Every pipeline stage yields a [`Carrier<T>`] — a
//!   *result paired with accumulated diagnostics* — and never panics on malformed input.
//!   Errors are data, not control flow.

#![forbid(unsafe_code)]
#![expect(
    missing_docs,
    reason = "relocated round-19/22 seeded diagnostic code (288:phase-aid-crate-extraction); \
              ratchets away as layers are replaced"
)]

/// Severity of a [`Diag`](diag::Diag). `Error` does not abort the pipeline (stages
/// never throw); it marks that the carried result is best-effort / degraded. It is the
/// [`registry`](diag::registry) severity — never set at a construction site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

pub mod diag;
pub use diag::Diag;

pub mod catalog;

pub mod arrangement;

pub mod display;

pub mod instant;

pub mod said;
pub use said::Said;

pub mod tagged;

pub mod weave;

pub mod narrative;
pub use narrative::{CollapseKind, CollapseNarrative, Knowability, SpeechAct};

/// `result × accumulated diagnostics` — the type every pipeline stage returns
/// (research chord `dn-7` / `ch-carrier`). A writer-monad shape: `map` transforms
/// the value, `and_then` sequences a stage while concatenating its diagnostics.
/// Stages never throw; malformed input yields a degraded `value` plus `Error`
/// diagnostics, so downstream stages still run and surface *unrelated* problems.
#[derive(Debug, Clone)]
pub struct Carrier<T> {
    pub value: T,
    pub diags: Vec<Diag>,
}

impl<T> Carrier<T> {
    /// A clean result with no diagnostics.
    #[must_use]
    pub fn pure(value: T) -> Self {
        Self {
            value,
            diags: Vec::new(),
        }
    }

    #[must_use]
    pub fn new(value: T, diags: Vec<Diag>) -> Self {
        Self { value, diags }
    }

    /// Transform the carried value, preserving diagnostics.
    #[must_use]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Carrier<U> {
        Carrier {
            value: f(self.value),
            diags: self.diags,
        }
    }

    /// Sequence a stage, concatenating its diagnostics after `self`'s.
    #[must_use]
    pub fn and_then<U>(mut self, f: impl FnOnce(T) -> Carrier<U>) -> Carrier<U> {
        let mut next = f(self.value);
        self.diags.append(&mut next.diags);
        Carrier {
            value: next.value,
            diags: self.diags,
        }
    }

    pub fn push(&mut self, diag: Diag) {
        self.diags.push(diag);
    }

    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|d| d.severity() == Severity::Error)
    }

    #[must_use]
    pub fn into_parts(self) -> (T, Vec<Diag>) {
        (self.value, self.diags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dorc_core::{BytePos, Span};

    #[test]
    fn carrier_threads_diagnostics_through_stages() {
        // A Warning-severity Diag (severity is registry data, `crib-4`): CfgBuiltinShadowed is
        // registry-Warning, so `has_errors()` stays false.
        let warn = Diag::new(
            diag::DiagCode::CfgBuiltinShadowed(diag::CfgBuiltinShadowed {
                detail: "heads up".to_owned(),
            }),
            Span::new(BytePos(0), BytePos(1)),
        );
        let result = Carrier::pure(2)
            .map(|n| n + 1)
            .and_then(|n| Carrier::new(n * 10, vec![warn]));
        assert_eq!(result.value, 30);
        assert_eq!(result.diags.len(), 1);
        assert!(!result.has_errors());
    }

    #[test]
    fn carrier_reports_errors_without_panicking() {
        // An Error-severity Diag: SyntaxMalformed is registry-Error, so `has_errors()` is true.
        let mut c = Carrier::pure(());
        c.push(Diag::new(
            diag::DiagCode::SyntaxMalformed(diag::SyntaxMalformed {
                detail: "bad input, kept going".to_owned(),
            }),
            Span::new(BytePos(0), BytePos(1)),
        ));
        assert!(c.has_errors());
    }
}
