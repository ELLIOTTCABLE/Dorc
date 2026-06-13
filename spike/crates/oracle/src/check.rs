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
/// the message as `detail`. The `span` is ALWAYS real: every caller that previously had
/// no token (an EOF give-up — an unterminated body, a `fail_here`/`true_with` at
/// end-of-input) now synthesizes a zero-width end-of-input span via
/// [`Parser::eof_span`](super::parser) (human ruling 22-q1: pointing the UI at
/// end-of-file is right for a truncated/chopped check body), so these two codes lower
/// through [`Diag::new`] like every other and do NOT join the spanless-mint allow-list.
///
/// Routed through the typed [`Diag`] spine, NOT `Diagnostic::error` (x3a-5/t-4 fix,
/// `224` §10): severity comes from [`dorc_core::diag::registry`] keyed on the code, never
/// hardcoded here. Both check codes are registry-declared `Error`, so the lowered output is
/// byte-identical to the prior `Diagnostic::error(…)` form — but a future registry edit now
/// actually takes effect instead of being a silent no-op.
///
/// The two `Code::Check…(…)` payloads are spelled as LITERALS at each `Diag::new` site (not
/// built once into a variable and threaded), so the `diag_tidy` constructed-scan actually SEES
/// these emits — a `Diag::new(var, …)` form would be invisible to the needle-shape scanner (t-4
/// non-literal bypass) and read as dead catalog. Verbose-on-purpose; the literals are the gate's
/// eyes.
pub(crate) fn lift_failure(
    is_unterminated: bool,
    span: Span,
    message: impl Into<String>,
    interner: &mut Interner,
) -> Diagnostic {
    let msg = message.into();
    // `Diag::to_legacy` rebuilds the legacy `message` from the primary label plus the render body
    // (empty for these `detail`-only payloads), so `.label(msg)` reproduces the prior bare text
    // exactly. Severity/code/span all flow from the typed value (severity via `registry`).
    let diag = if is_unterminated {
        Diag::new(
            Code::CheckUnterminated(CheckUnterminated {
                detail: msg.clone(),
            }),
            span,
        )
    } else {
        Diag::new(
            Code::CheckOutOfDialect(CheckOutOfDialect {
                detail: msg.clone(),
            }),
            span,
        )
    }
    .label(msg);
    diag.to_legacy(interner)
}

#[cfg(test)]
mod lift_failure_tests {
    use super::{Code, lift_checks, lift_failure};
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
    ///
    /// Span is ALWAYS real now (human ruling 22-q1): `lift_failure` takes a [`Span`], not
    /// `Option<Span>`, so this exercises the single real-span path per code (the EOF-synthesis path
    /// is pinned separately in [`eof_give_up_carries_a_real_end_span`]).
    #[test]
    fn lift_failure_severity_agrees_with_registry() {
        let mut interner = Interner::default();
        let span = Span::new(BytePos(3), BytePos(7));
        let want_unterm = registry(&Code::CheckUnterminated(CheckUnterminated {
            detail: String::new(),
        }))
        .severity;
        let d = lift_failure(true, span, "unterminated", &mut interner);
        assert_eq!(d.code.0, "check-unterminated");
        assert_eq!(
            d.severity, want_unterm,
            "unterminated severity must equal the registry's, not a hardcoded value"
        );
        assert_eq!(d.span, Some(span), "span flows through unchanged");
        assert_eq!(
            d.message, "unterminated",
            "message is the bare text (no body added)"
        );

        let want_dialect = registry(&Code::CheckOutOfDialect(CheckOutOfDialect {
            detail: String::new(),
        }))
        .severity;
        let d = lift_failure(false, span, "out of dialect", &mut interner);
        assert_eq!(d.code.0, "check-out-of-dialect");
        assert_eq!(
            d.severity, want_dialect,
            "out-of-dialect severity must equal the registry's, not a hardcoded value"
        );
        assert_eq!(d.span, Some(span));
        assert_eq!(d.message, "out of dialect");
    }

    /// An EOF give-up now carries a REAL zero-width end-of-input span, never a span-less mint
    /// (human ruling 22-q1: point the UI at end-of-file for a truncated/chopped body). Pins the
    /// observable change — a truncated check body's diagnostic gains a `Some(span)` where it
    /// previously had `None` — through the public [`lift_checks`] entry, so it exercises the real
    /// `eof_span()`-via-`fail_here`/`true_with` wiring rather than a hand-built span.
    #[test]
    fn eof_give_up_carries_a_real_end_span() {
        let mut interner = Interner::default();
        // An unterminated function body: the lexer runs out of tokens inside `parse_block`, so
        // `true_with` fires at EOF (the pre-22-q1 span-less case).
        let src = "x__check() { x : K = \"$1\"";
        let lifted = lift_checks(&mut interner, src);
        let diag = lifted
            .diags
            .first()
            .expect("an unterminated body yields a lift diagnostic");
        let span = diag
            .span
            .expect("the EOF give-up carries a real span now, not None (22-q1)");
        assert_eq!(
            span.lo, span.hi,
            "the synthesized EOF span is zero-width (a caret at end-of-input)"
        );
        // It lands at end-of-input — the last real token's `hi`, somewhere PAST the file start
        // (so it is genuinely the EOF position, not the byte-0 `ZERO_SPAN` fallback) and within
        // the source bytes. We avoid pinning the exact offset (lexer token-end detail), only the
        // load-bearing property: a real, non-zero end-of-input caret.
        let src_len = u32::try_from(src.len()).expect("fixture fits u32");
        assert!(
            span.hi.0 > 0 && span.hi.0 <= src_len,
            "the EOF caret lands where input ran out (0 < {} <= {src_len}), not at byte 0",
            span.hi.0
        );
    }
}
