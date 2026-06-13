//! `check` — the command-keyed `check()` contract (19H §2 / 202 §1 face-check).
//!
//! An oracle ships one sh function per command-family, `<provider>__check`, that
//! argparses the command the way the real tool does, inline-annotates which value
//! is which named kind, and is itself the read-only probe body. This module is the
//! *static half* (202 §2.7's "read twice"): a dedicated mini-parser for the
//! constrained contract dialect plus a concrete evaluator that traces a known argv
//! through a check's argparse to its kind-annotation, yielding a [`Resolution`].
//!
//! # Why a separate parser (`adj-dialect-parser`, note 203 §4)
//!
//! The contract dialect is *not* arbitrary sh and is *not* the book dialect. The
//! book parser (`dorc-syntax`) ⊤-rejects loops by design; a `check()`'s argparse
//! *needs* `while`. Extending the book parser would drag its CFG-lowering, errexit
//! and consumption machinery along. Instead this module owns a small parser whose
//! grammar *is* the contract: anything it cannot parse is a loud per-function lift
//! failure (`inv-no-throw`: a diagnostic, never a panic; the file's other checks
//! still lift). The dialect parser structurally enforces "the dialect is NOT
//! arbitrary sh" (19G §2) — outside-dialect input fails to parse ⇒ unresolvable.
//!
//! # Soundness posture (`inv-kfail`, both directions)
//!
//! The evaluator never guesses. A flag the argparse does not consume, an arm `case`
//! cannot select concretely, an annotation whose value-position is not a positional
//! or known binding, a missing annotation, a budget overrun — every one is
//! [`Top`](Resolution::Top) with a reason. `Top` is always safe (the site stays
//! un-probeable and un-elidable). A *wrong* resolution is the disaster class (19H
//! §1.3: propagation-correctness has no floor), so every ambiguity biases to `Top`.
//!
//! `inv-referent-agnostic`: the evaluator resolves which **argv element** is the
//! entity (by tracing the oracle's own argparse); it never branches on what the
//! entity's text *means*. Kind strings are opaque coordination handles.

use dorc_core::diag::{CheckOutOfDialect, CheckUnterminated, Diag, DiagCode as Code};
use dorc_core::{Diagnostic, Interner, Span};

mod ast;
mod eval;
mod lexer;
mod parser;

pub use ast::{Check, CheckSet};
pub use eval::{Resolution, Resolved, ResolvedEntity, TopReason, evaluate};
pub use parser::lift_checks;

/// Map a check function's provider-name fragment to the command word: `_` → `-`
/// (`apt_get` ⇒ `apt-get`). The **single** home of the underscore↔hyphen convention
/// (204 §3, the `tc-*`-flagged provider-name rule): the dialect parser keys a
/// [`CheckSet`] by the mapped name, AND the engine's wiring (`analysis::effect`)
/// re-derives the provider symbol from a book's command word through this same
/// function, so the book's command-word interning, `KindIndex`'s `ProviderId`
/// interning, and the `CheckSet` key all agree (204 §6 seam #2). Exported so the
/// mapping is never duplicated; a future provider-name escape lands here alone.
///
/// **Lossy** (a literal `_` in a command name cannot be expressed); flagged `tc-*`.
#[must_use]
pub fn map_provider_name(raw: &str) -> String {
    raw.replace('_', "-")
}

// B4 sweep: check codes migrated onto Diag spine.

/// The conventional local variable name an oracle assigns the verb to (`verb=$1`,
/// 19H §2.1/§2.5). Recognizing it is a *structural convention in the oracle's own
/// code* (like the `__check` suffix or the annotation shape), NOT decoding entity
/// text — so it does not breach `inv-referent-agnostic`. Whether `verb` should be a
/// reserved dialect name is a `tc-*`-shaped cross-cutting question (flagged in the
/// build report); the conservative local choice is: if the oracle does not use this
/// name, the [`Resolution`] simply carries no verb (always safe).
const VERB_BINDING: &str = "verb";

/// A per-function lift failure: the named function is in the file but its body is
/// out of dialect. Fail-soft (`inv-no-throw`): the function contributes no [`Check`]
/// and the rest of the file still lifts.
///
/// `is_unterminated`: selects `CheckUnterminated` vs `CheckOutOfDialect`. Both carry
/// the message as `detail`; the span is `Option<Span>` because EOF sites (an
/// unterminated body, a `fail_here` at end-of-input) have no token to point at.
///
/// Routed through the typed [`Diag`] spine, NOT `Diagnostic::error` (x3a-5/t-4 fix,
/// `224` §10): severity comes from [`dorc_core::diag::registry`] keyed on the code, never
/// hardcoded here. Both check codes are registry-declared `Error`, so the lowered output is
/// byte-identical to the prior `Diagnostic::error(…)` form — but a future registry edit now
/// actually takes effect instead of being a silent no-op. A real span lowers via [`Diag::new`];
/// a span-less EOF site lowers via [`Diag::new_spanless_site`] (the arch-3-residual-2 second-class
/// door), so these two codes JOIN the spanless-mint allow-list (`core/tests/diag_tidy.rs` grows
/// 6→8 — a HUMAN-disposed amendment to the stated six-code spanless boundary; see that gate).
///
/// The four `Code::Check…(…)` payloads are spelled as LITERALS at each `Diag::new`/
/// `new_spanless_site` site (not built once into a variable and threaded), so the `diag_tidy`
/// grep-shape scans actually SEE these emits — a `new_spanless_site(var)` form would be invisible
/// to the needle-shape scanner (t-4 non-literal bypass) and silently defeat the spanless gate for
/// exactly the two codes this fix adds to it. Verbose-on-purpose; the literals are the gate's eyes.
pub(crate) fn lift_failure(
    is_unterminated: bool,
    span: Option<Span>,
    message: impl Into<String>,
    interner: &mut Interner,
) -> Diagnostic {
    let msg = message.into();
    // `Diag::to_legacy` rebuilds the legacy `message` from the primary label plus the render body
    // (empty for these `detail`-only payloads), so `.label(msg)` reproduces the prior bare text
    // exactly. Severity/code/span all flow from the typed value (severity via `registry`).
    let diag = match (is_unterminated, span) {
        (true, Some(s)) => Diag::new(
            Code::CheckUnterminated(CheckUnterminated {
                detail: msg.clone(),
            }),
            s,
        ),
        (true, None) => Diag::new_spanless_site(Code::CheckUnterminated(CheckUnterminated {
            detail: msg.clone(),
        })),
        (false, Some(s)) => Diag::new(
            Code::CheckOutOfDialect(CheckOutOfDialect {
                detail: msg.clone(),
            }),
            s,
        ),
        (false, None) => Diag::new_spanless_site(Code::CheckOutOfDialect(CheckOutOfDialect {
            detail: msg.clone(),
        })),
    }
    .label(msg);
    diag.to_legacy(interner)
}

#[cfg(test)]
mod lift_failure_tests {
    use super::{Code, lift_failure};
    use dorc_core::diag::{CheckOutOfDialect, CheckUnterminated, registry};
    use dorc_core::{BytePos, Interner, Span};

    /// The emit-vs-registry AGREEMENT pin (x3a-5/t-4 fix, `224` §10): `lift_failure` must source
    /// its severity from [`registry`], never hardcode it. This catches the exact regression the
    /// crosscheck found — a reversion to `Diagnostic::error(…)` would hardcode `Error`, so if a
    /// future registry edit moved either check code OFF `Error`, the emit would DISAGREE and this
    /// test trips. (At HEAD both are registry-`Error`, so the agreement holds AND the lowered output
    /// is byte-identical to the old hardcoded form — but the agreement is now structural, not luck.)
    ///
    /// Adversarial discipline (`inv-probe-sourced-values` spirit): the assertion does NOT hardcode
    /// `Severity::Error` on the emit side — it compares the EMITTED severity against the registry's,
    /// so the test stays correct (and keeps protecting) if the human re-grades the code at harvest.
    #[test]
    fn lift_failure_severity_agrees_with_registry() {
        let mut interner = Interner::default();
        let span = Some(Span::new(BytePos(3), BytePos(7)));
        // CheckUnterminated, with and without a span.
        let want_unterm = registry(&Code::CheckUnterminated(CheckUnterminated {
            detail: String::new(),
        }))
        .severity;
        for s in [span, None] {
            let d = lift_failure(true, s, "unterminated", &mut interner);
            assert_eq!(d.code.0, "check-unterminated");
            assert_eq!(
                d.severity, want_unterm,
                "unterminated severity must equal the registry's, not a hardcoded value"
            );
            assert_eq!(
                d.span, s,
                "span flows through unchanged (Some preserved, None at EOF)"
            );
            assert_eq!(
                d.message, "unterminated",
                "message is the bare text (no body added)"
            );
        }
        // CheckOutOfDialect, with and without a span.
        let want_dialect = registry(&Code::CheckOutOfDialect(CheckOutOfDialect {
            detail: String::new(),
        }))
        .severity;
        for s in [span, None] {
            let d = lift_failure(false, s, "out of dialect", &mut interner);
            assert_eq!(d.code.0, "check-out-of-dialect");
            assert_eq!(
                d.severity, want_dialect,
                "out-of-dialect severity must equal the registry's, not a hardcoded value"
            );
            assert_eq!(d.span, s);
            assert_eq!(d.message, "out of dialect");
        }
    }
}
