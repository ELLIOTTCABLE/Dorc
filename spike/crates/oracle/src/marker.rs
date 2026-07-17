//! `marker` — the `# dorc-lang/v0.1` version-comment gate (`marker-gates-syntax-only`, `24M`
//! `rul24M-version-comment`). The marker gates SYNTAX only: a file using a dialect construct — a
//! bind (`name : kind = value`) or a trailing mark (`:`/`:!`/`:?`) — WITHOUT the marker is a loud
//! error. `__role` NAME-recognition is UNAFFECTED (a bare, markless `foobar__is_converged` pure-
//! POSIX body lifts fine — that IS the typeless floor, `24L` §2). An unmarked file with no dialect
//! construct is plain sh; a stripped off-ramp artifact (dialect erased) stays marker-free and only
//! WARNS on a reserved-namespace squat (`guard23-reingest-collision-verbatim`), never errors here.
//!
//! Design note (tc-marker-gate-home, flagged UP): the gate is a DEDICATED edge pass the driver
//! calls per source, NOT logic inside `lift_predicts` — so the lift stays a pure syntax mechanic
//! (the ~100 lift unit-tests carry unmarked dialect deliberately and must keep lifting), while the
//! gate fires at the user boundary (every oracle/book the cli loads). The parser is marker-blind;
//! this pass flags the dialect-in-unmarked-file post-hoc.

use dorc_core::{DiagCode, Diagnostic, Interner, Span};

use crate::predict::{
    Stmt, lift_predicts, lift_reaches, lift_resolvers, lift_state_stored_only_in, lift_touches,
    lift_verdicts_converged,
};

/// The dialect version marker — exact-match, standalone, within the first 10 physical lines
/// (`24C:rul24-marker-v0.1`). The SOLE sanctioned comment-parse in the product.
pub const MARKER: &str = "# dorc-lang/v0.1";

/// How many leading lines the marker may occupy (`24C:rul24-marker-v0.1`: "the first 10 physical
/// lines"), so a shebang + a purpose header still precede it.
const MARKER_WINDOW: usize = 10;

/// Is `src` marked with the dialect version comment?
#[must_use]
pub fn has_marker(src: &str) -> bool {
    src.lines()
        .take(MARKER_WINDOW)
        .any(|l| l.trim_end() == MARKER)
}

/// The marker gate (`marker-gates-syntax-only`). A dialect construct in an UNMARKED source is ONE
/// loud error naming the missing marker, pointed at the first offending bind/mark. A marked file, a
/// dialect-free file (plain sh or a stripped off-ramp artifact), and bare `__role` floor bodies all
/// pass silently. Re-lifts all six roles (pure, cheap) to scan every funcdef body; `build_vouches`
/// and the effect-map lift do their own lifts, so nothing here is load-bearing beyond the gate.
#[must_use]
pub fn check_dialect_marker(interner: &mut Interner, src: &str) -> Vec<Diagnostic> {
    if has_marker(src) {
        return Vec::new();
    }
    let sets = [
        lift_predicts(interner, src).value,
        lift_touches(interner, src).value,
        lift_verdicts_converged(interner, src).value,
        lift_resolvers(interner, src).value,
        lift_reaches(interner, src).value,
        lift_state_stored_only_in(interner, src).value,
    ];
    let mut offender: Option<Span> = None;
    for set in &sets {
        for sym in set.providers() {
            let Some(predict) = set.get(sym) else {
                continue;
            };
            if let Some(span) = first_dialect_span(&predict.body) {
                offender = Some(match offender {
                    Some(prev) if prev.lo.0 <= span.lo.0 => prev,
                    _ => span,
                });
            }
        }
    }
    match offender {
        Some(span) => vec![Diagnostic::error(
            DiagCode("missing-dialect-marker"),
            Some(span),
            format!(
                "this file uses a dorc-lang dialect construct (a bind `name : kind = …` or a \
                 trailing `:`/`:!`/`:?` mark) but lacks the `{MARKER}` version marker \
                 (marker-gates-syntax-only): add `{MARKER}` as a standalone comment in the first \
                 {MARKER_WINDOW} lines, or drop the dialect (the bare `__role` floor works markerless)"
            ),
        )],
        None => Vec::new(),
    }
}

/// The lowest-offset span of the first dialect construct (a bind [`Stmt::Annotation`] or a trailing
/// [`Command`](crate::predict) mark) reachable in `body`, recursing every control-flow body; `None`
/// if the body is pure floor sh. Deterministic, allocation-free.
fn first_dialect_span(body: &[Stmt]) -> Option<Span> {
    let mut best: Option<Span> = None;
    let mut note = |span: Span| {
        best = Some(match best {
            Some(prev) if prev.lo.0 <= span.lo.0 => prev,
            _ => span,
        });
    };
    for stmt in body {
        match stmt {
            Stmt::Annotation(a) => note(a.span),
            Stmt::Command(c) => {
                if let Some(mark) = &c.mark {
                    note(mark.span);
                }
            }
            Stmt::While { body, .. } => {
                if let Some(s) = first_dialect_span(body) {
                    note(s);
                }
            }
            Stmt::Case { arms, .. } => {
                for arm in arms {
                    if let Some(s) = first_dialect_span(&arm.body) {
                        note(s);
                    }
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                if let Some(s) = first_dialect_span(then_body) {
                    note(s);
                }
                if let Some(s) = first_dialect_span(else_body) {
                    note(s);
                }
            }
            Stmt::Assign { .. } | Stmt::Shift { .. } => {}
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marked_file_passes_silently() {
        let mut i = Interner::default();
        let src = "# dorc-lang/v0.1\napt_get__predict() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; }";
        assert!(
            check_dialect_marker(&mut i, src).is_empty(),
            "a marked file with a bind lifts clean"
        );
    }

    #[test]
    fn unmarked_bind_is_a_loud_error() {
        let mut i = Interner::default();
        let src = "apt_get__predict() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; }";
        let diags = check_dialect_marker(&mut i, src);
        assert_eq!(diags.len(), 1, "one file-level error");
        assert_eq!(diags[0].code.0, "missing-dialect-marker");
    }

    #[test]
    fn unmarked_trailing_mark_is_a_loud_error() {
        let mut i = Interner::default();
        // A markless VERDICT body with a trailing verdict mark is dialect ⇒ needs the marker.
        let src = "svc__is_converged() { systemctl is-active -- \"$1\"  : sm.dorc.Service:\"$1\"#active ; }";
        let diags = check_dialect_marker(&mut i, src);
        assert_eq!(diags.len(), 1, "a trailing mark in an unmarked file errors");
        assert_eq!(diags[0].code.0, "missing-dialect-marker");
    }

    #[test]
    fn unmarked_bare_floor_body_passes() {
        // THE typeless floor (`24L` §2): a markerless, pure-POSIX verdict body — no bind, no mark —
        // lifts by NAME regardless. The gate must NOT fire (that IS the floor).
        let mut i = Interner::default();
        let src = "foobar__is_converged() {\n   case \"$1\" in\n   sync) foobar status \"$1\" ;;\n   *) return 2 ;;\n   esac\n}";
        assert!(
            check_dialect_marker(&mut i, src).is_empty(),
            "a bare markless floor body needs no marker (name-recognition is unaffected)"
        );
    }

    #[test]
    fn stripped_offramp_artifact_stays_marker_free() {
        // A stripped off-ramp artifact carries a reserved-name funcdef but NO dialect construct
        // (binds/marks erased) — it stays marker-free and never errors here
        // (guard23-reingest-collision-verbatim keeps warning, not error).
        let mut i = Interner::default();
        let src = "apt_get__predict() { dpkg-query -W \"$1\"; }";
        assert!(
            check_dialect_marker(&mut i, src).is_empty(),
            "a stripped (dialect-free) reserved-name funcdef needs no marker"
        );
    }
}
