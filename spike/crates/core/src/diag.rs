//! The diagnostic catalog — the ONE place a registered [`DiagCode`] is paired
//! with its message TEMPLATE (`219` q-2 / `21G` §3 rq-1..rq-3, the dislocated-index
//! embryo).
//!
//! Why a catalog (`21G` §2, the Pottier/Menhir lineage `[A-pottier-reachability-2016]`):
//! the real codebase's layer-1 ambition is a dislocated index of error messages
//! (slug → catalog entry) plus a mechanical completeness gate asserting every
//! give-up code-path carries a registered, catalogued code. This module is the
//! spike-scale seed of that:
//!
//! * **rq-1** — codes are registered HERE, and the phrasing lives HERE
//!   ([`template`]); emit sites pass *structured params* to a constructor
//!   ([`cmdsub_operand_top`] / [`cmdsub_inner_nonleaf`] / [`site_unresolvable`])
//!   and never hand-write the prose. The catalog owns the words.
//! * **rq-2** — [`CATALOG`] is the registry; the completeness test
//!   (`every_registered_code_has_a_nonempty_template`) asserts every registered
//!   code resolves to a non-empty template. Trivial now, load-bearing once the
//!   path-enumeration gate (the full Pottier story) lands in a later round.
//! * **rq-3** — the constructors are the ONLY way these codes reach a
//!   [`Diagnostic`]; there is no free-text [`Diagnostic::note`] on the new `$()`
//!   ⊤-diagnostic paths.
//!
//! Scope (`219` q-2 / `21G` §3): this is the floor slice — the three silent-`$()`
//! ⊤-degradation sites get an honest Note. Existing scattered codes
//! (`effect-kind-disagreement`, `cfg-top-node`, …) are NOT retrofitted here (the
//! direction is "fold existing codes only where trivial; no analyzer is built").

use crate::{DiagCode, Diagnostic, Severity, Span};

/// A `$()` / runtime-dynamic operand or command word forced a command to `Opaque`
/// (it cannot be characterized ⇒ it runs, never elided). The find-3
/// no-silent-phantoms floor: this degradation used to be silent (`219` q-1.f
/// silent-2). Generic ⊤-cause wording this round (`fork-cmdsub-top-cause` floor,
/// `21G` §4): the value plane is cause-erased at the emit site, so the message
/// names the *position* that went ⊤, not the original `$()` text.
pub const CMDSUB_OPERAND_TOP: DiagCode = DiagCode("dq-cmdsub-operand-top");

/// A command runs inside a `$( … )` substitution body: it is effect-bearing (it
/// stays in the reaching-defs, so it poisons/establishes) but is NOT independently
/// elidable (it runs whenever its enclosing line runs). The `exec-subst-body-nonleaf`
/// disclosure — invisible today (`219` q-1.f silent-1/silent-4 shape).
pub const CMDSUB_INNER_NONLEAF: DiagCode = DiagCode("dq-cmdsub-inner-nonleaf");

/// A probe-unresolvable site (cli-edge): the rendered probe could not ship a
/// read-only check for this command, so the apply runs it (`kFAIL-perform`). The
/// stderr echo of `ProbePlan::unresolvable` — a `skip-unresolvable` comment lands
/// in the artifact today, but nothing reaches stderr (`219` q-1.f silent-3).
pub const SITE_UNRESOLVABLE: DiagCode = DiagCode("dq-site-unresolvable");

/// A WRITE-shaped redirect (`>`/`>>`) to a DYNAMIC/unresolved target (`>> "$dyn"`):
/// the path cannot be resolved to a literal, so the engine cannot key a per-path
/// `file` cell ⇒ the write joins ⊤ (the Opaque-poison shape) and the command runs
/// (y-1, `21F` imp-1). Discloses the un-keyable write (the redirect-effects analog of
/// `dq-cmdsub-operand-top` — a ⊤ that forces poison, surfaced not silent).
pub const REDIR_TARGET_TOP: DiagCode = DiagCode("dq-redir-target-top");

/// Every code registered in this catalog (the rq-2 completeness-gate input). A code
/// that resolves to an empty [`template`] is a catalog bug the test catches.
pub const CATALOG: &[DiagCode] = &[
    CMDSUB_OPERAND_TOP,
    CMDSUB_INNER_NONLEAF,
    SITE_UNRESOLVABLE,
    REDIR_TARGET_TOP,
];

/// The message TEMPLATE for a registered code, or `""` for an unregistered one
/// (rq-1: phrasing lives HERE, decoupled from the emit site). The `{}` placeholder
/// is filled by the matching constructor from its structured params. Kept as a
/// single match so adding a code is one arm + one [`CATALOG`] entry + one
/// constructor — the trivial shape the future path-enumeration gate extends.
#[must_use]
pub fn template(code: DiagCode) -> &'static str {
    match code {
        CMDSUB_OPERAND_TOP => {
            "command forced to run (never elided): {position} is a command-substitution \
             `$(…)` or runtime-dynamic value ⇒ its identity is unresolved (⊤)"
        }
        CMDSUB_INNER_NONLEAF => {
            "command `{inner}` runs inside a `$(…)` substitution ⇒ effect-bearing but not \
             independently elidable (it runs whenever its enclosing line runs)"
        }
        SITE_UNRESOLVABLE => {
            "site {leaf} (`{source}`) is probe-unresolvable ⇒ the apply runs it \
             (no read-only check could be shipped)"
        }
        REDIR_TARGET_TOP => {
            "write-redirect to a dynamic/unresolved target ⇒ no per-path `file` cell can be \
             keyed, so the write joins ⊤ and the command runs (never elided)"
        }
        _ => "",
    }
}

/// Build the [`CMDSUB_OPERAND_TOP`] Note from structured params (rq-1/rq-3):
/// `position` names the blocking word's position (`the command word`, or `operand N`).
/// The blocking word's *text* is unavailable (the value plane is cause-erased to ⊤ by
/// this point — `219` q-1.f), so `span` carries the site location for provenance.
#[must_use]
pub fn cmdsub_operand_top(span: Option<Span>, position: &str) -> Diagnostic {
    Diagnostic::note(
        CMDSUB_OPERAND_TOP,
        span,
        fill(template(CMDSUB_OPERAND_TOP), &[("position", position)]),
    )
}

/// Build the [`CMDSUB_INNER_NONLEAF`] Note from structured params (rq-1/rq-3):
/// `inner` is the inner command's resolved text (the mutator the disclosure surfaces).
#[must_use]
pub fn cmdsub_inner_nonleaf(span: Option<Span>, inner: &str) -> Diagnostic {
    Diagnostic::note(
        CMDSUB_INNER_NONLEAF,
        span,
        fill(template(CMDSUB_INNER_NONLEAF), &[("inner", inner)]),
    )
}

/// Build the [`SITE_UNRESOLVABLE`] Note from structured params (rq-1/rq-3): `leaf`
/// is the site id, `source` the site's source command text.
#[must_use]
pub fn site_unresolvable(span: Option<Span>, leaf: &str, source: &str) -> Diagnostic {
    Diagnostic::note(
        SITE_UNRESOLVABLE,
        span,
        fill(
            template(SITE_UNRESOLVABLE),
            &[("leaf", leaf), ("source", source)],
        ),
    )
}

/// Build the [`REDIR_TARGET_TOP`] Note (y-1): a write-redirect to a ⊤ target. The
/// message is parameterless (the offending word is ⊤, so its text is unavailable —
/// `dq-cmdsub-operand-top`'s cause-erasure, mirrored); `span` carries the redirect's
/// source location for provenance.
#[must_use]
pub fn redir_target_top(span: Option<Span>) -> Diagnostic {
    Diagnostic::note(
        REDIR_TARGET_TOP,
        span,
        template(REDIR_TARGET_TOP).to_owned(),
    )
}

/// Substitute `{key}` placeholders in `template` from `params`. A deterministic,
/// allocation-light fill (the spike has at most two params per template); an
/// unmatched placeholder is left verbatim so a template/param mismatch is visible
/// rather than silently dropped. `inv-determinism`: pure, order-stable.
fn fill(template: &str, params: &[(&str, &str)]) -> String {
    let mut out = template.to_owned();
    for (key, value) in params {
        let needle = format!("{{{key}}}");
        out = out.replace(&needle, value);
    }
    out
}

/// Severity of every catalogued code this round — all `Note` (`219` q-2.b: the
/// floor slice is pure disclosure, so it never trips the gate-3 error-floor). Kept
/// as a tiny function so the completeness test can assert the invariant.
#[must_use]
pub fn severity(code: DiagCode) -> Option<Severity> {
    if CATALOG.contains(&code) {
        Some(Severity::Note)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// rq-2 (the embryonic Pottier completeness gate, `21G` §3): every registered
    /// code resolves to a non-empty template. Trivial today; the point is the GATE
    /// exists, so a future code added to [`CATALOG`] without a template fails CI.
    #[test]
    fn every_registered_code_has_a_nonempty_template() {
        for &code in CATALOG {
            assert!(
                !template(code).is_empty(),
                "registered code `{}` has no catalog template (rq-2 completeness gate)",
                code.0
            );
        }
    }

    /// The floor-slice severity invariant (`219` q-2.b): a catalogued code is
    /// Note-severity, so it never trips the e2e stderr error-floor (gate-3 keys on
    /// `error[…]` only). If a future code needs Error severity, this test forces a
    /// deliberate change rather than a silent floor-breach.
    #[test]
    fn every_registered_code_is_note_severity() {
        for &code in CATALOG {
            assert_eq!(
                severity(code),
                Some(Severity::Note),
                "catalogued code `{}` must be Note-severity this round (gate-3 floor)",
                code.0
            );
        }
    }

    /// rq-1/rq-3: the constructors fill their templates from structured params, and
    /// the catalogued code/severity ride through. Pins that a param actually lands
    /// in the message (a template/param drift would leave a `{placeholder}` behind).
    #[test]
    fn constructors_fill_templates_from_params() {
        let d = cmdsub_operand_top(None, "operand 2");
        assert_eq!(d.code, CMDSUB_OPERAND_TOP);
        assert_eq!(d.severity, Severity::Note);
        assert!(
            d.message.contains("operand 2"),
            "position param must land: {}",
            d.message
        );
        assert!(
            !d.message.contains('{'),
            "no unfilled placeholder: {}",
            d.message
        );

        let d = cmdsub_inner_nonleaf(None, "apt-get install -y nginx");
        assert!(d.message.contains("apt-get install -y nginx"));
        assert!(!d.message.contains('{'));

        let d = site_unresolvable(None, "1", ": > /etc/x");
        assert!(d.message.contains(": > /etc/x"));
        assert!(!d.message.contains('{'));
    }
}
