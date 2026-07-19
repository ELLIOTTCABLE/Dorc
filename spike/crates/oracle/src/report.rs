//! `report` — the tier-1 static report-sink inventory (`27W` §3 `rul-static-first-three-tier`;
//! `decline-class-emission`).
//!
//! An oracle-alone AST walk over a verdict body that recognizes the deliberate-decline emission
//! idiom — `printf '<verb> <class> <tail…>' >>"${DREP_V1:-/dev/null}"` on a declining path — and
//! value-threads the LITERAL format string into a per-arm `(arm span, class)` inventory. The
//! recognition of the sink redirect is done at PARSE ([`crate::predict::Command::report_sink`],
//! the engine-owned sink-name list); this module only reads the flagged commands and parses the
//! `<verb> <class>` header out of the format literal.
//!
//! Three consumers ride one spelling (`27W` §2): this static inventory (tiers 1–2), the runtime
//! probe emission (tier-3), and an off-ramp harness setting the sink var. This module is the
//! static half. A DYNAMICALLY-built format string (`printf "$fmt" …`) is NOT a literal ⇒ its class
//! is unreadable here (`class: None`) and is left to tier-3 (that degradation is by design —
//! `27W` §2 "one honest loss").
//!
//! `inv-referent-agnostic` / decision-inert (`two-plane-aid-law`): the verb/class tokens are
//! engine-owned vocabulary (`DeclineClass::from_token`); the free tail is opaque author text,
//! never decoded. Nothing here feeds the license plane — classes route AID only.

use dorc_core::Span;
use dorc_core::evidence::DeclineClass;

use crate::predict::{Command, Predict, Stmt, Word};

/// One recognized `decline-class-emission` in a verdict body (`27W` §2): the emitting command's
/// span (the PRECISE arm — the same span the tracer's decline site reports), the parsed decline
/// class (or `None` for an unknown/free-form class token — the degrade-generic posture), and
/// whether the `<verb> <class>` header was FULLY recognized (`decline` verb + a starter-set class).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArmReport {
    /// The emitting command's source span (the arm the emission sits on).
    pub arm: Span,
    /// The recognized decline class, or `None` (unknown token / non-literal format ⇒ degrade-generic).
    pub class: Option<DeclineClass>,
    /// Whether both the `decline` verb and a known class were recognized (else a generic author-note).
    pub verb_recognized: bool,
}

/// Enumerate the report-sink emissions in `verdict`'s body (tier-1: the oracle alone, zero sites).
/// Recurses every control-flow body so an emission buried in a `case`/`if`/`while` arm (the corpus
/// idiom) is found; deterministic source order (`inv-determinism`).
#[must_use]
pub fn report_inventory(verdict: &Predict) -> Vec<ArmReport> {
    let mut out = Vec::new();
    collect(&verdict.body, &mut out);
    out
}

/// Does this verdict body emit ANY report line (tier-3 gate)? The probe report-drain scaffold is
/// emitted ONLY for emission-bearing bodies, so an oracle that emits nothing stays byte-identical
/// (`empty-world-byte-identical`).
#[must_use]
pub fn emits_report(verdict: &Predict) -> bool {
    !report_inventory(verdict).is_empty()
}

fn collect(body: &[Stmt], out: &mut Vec<ArmReport>) {
    for stmt in body {
        match stmt {
            Stmt::Command(c) if c.report_sink => out.push(read_emission(c)),
            Stmt::Case { arms, .. } => {
                for a in arms {
                    collect(&a.body, out);
                }
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                collect(then_body, out);
                collect(else_body, out);
            }
            Stmt::While { body, .. } => collect(body, out),
            Stmt::Command(_) | Stmt::Assign { .. } | Stmt::Shift { .. } | Stmt::Annotation(_) => {}
        }
    }
}

/// Value-thread a sink-emitting command's LITERAL format string into a class. `words[1]` is the
/// `printf` format (the `<verb> <class> <tail>` header); a non-literal format ⇒ unreadable
/// (`class: None`, tier-3 fallback). Split on whitespace: token 0 is the verb, token 1 the class.
fn read_emission(cmd: &Command) -> ArmReport {
    let fmt = cmd.words.get(1).and_then(literal_text);
    let Some(fmt) = fmt else {
        return ArmReport {
            arm: cmd.span,
            class: None,
            verb_recognized: false,
        };
    };
    let mut toks = fmt.split_whitespace();
    let verb = toks.next();
    let class = toks.next().and_then(DeclineClass::from_token);
    ArmReport {
        arm: cmd.span,
        class,
        verb_recognized: verb == Some("decline") && class.is_some(),
    }
}

/// The FULLY-recognized decline class of a sink-emitting command (`decline <class>` header,
/// literal format), or `None` for an unknown class / dynamic format / non-`decline` verb (an
/// `advise` verb is deferred — `27W:rul-advise-verb-deferred`). The tier-2 per-site class the
/// verdict tracer captures when a site's argv reaches this emission (`27W` §3). Same
/// value-threading as the tier-1 inventory.
pub(crate) fn recognized_class(cmd: &Command) -> Option<DeclineClass> {
    let r = read_emission(cmd);
    r.class.filter(|_| r.verb_recognized)
}

/// The literal text of a word, or `None` for any expansion-bearing form (a dynamic format defeats
/// static reading — `27W` §2).
fn literal_text(w: &Word) -> Option<&str> {
    match w {
        Word::Literal(s) | Word::SingleQuotedLiteral(s) => Some(s.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verdict::VerdictSet;
    use dorc_core::Interner;

    fn inventory(src: &str) -> Vec<ArmReport> {
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        assert!(set.diags.is_empty(), "clean lift: {:?}", set.diags);
        let p = set.value.providers().next().expect("one verdict funcdef");
        report_inventory(set.value.get(p).expect("the verdict funcdef"))
    }

    const SYSCTL: &str = "\
sysctl__is_converged() {
   key=$1
   case $key in
   vm.drop_caches|vm.compact_memory)
      printf 'decline unsound %s is a write-only trigger key\\n' \"$key\" >>\"${DREP_V1:-/dev/null}\"
      return 2 ;;
   kernel.core_pattern)
      printf 'decline unmodeled %s cascade not modeled\\n' \"$key\" >>\"${DREP_V1:-/dev/null}\"
      return 2 ;;
   *) sysctl -n -- \"$key\" >/dev/null 2>&1 ;;
   esac
}";

    #[test]
    fn inventory_reads_the_classes_from_literal_formats() {
        // Tier-1: the oracle alone yields per-arm (class) — no book, no site.
        let inv = inventory(SYSCTL);
        assert_eq!(inv.len(), 2, "two emitting arms");
        assert_eq!(inv[0].class, Some(DeclineClass::Unsound));
        assert!(inv[0].verb_recognized);
        assert_eq!(inv[1].class, Some(DeclineClass::Unmodeled));
        assert!(inv[1].verb_recognized);
        // The check arm (`sysctl -n …`) is not an emission — not inventoried.
    }

    #[test]
    fn unknown_class_token_degrades_generic_never_errors() {
        // `27W:rul-report-noise-tolerant`: an unrecognized class is kept (as an arm) with `class:
        // None`, never dropped, never an error.
        let src = "\
x__is_converged() {
   case $1 in
   weird) printf 'decline bogus-class whatever\\n' >>\"${DREP_V1:-/dev/null}\"
      return 2 ;;
   esac
}";
        let inv = inventory(src);
        assert_eq!(inv.len(), 1);
        assert_eq!(inv[0].class, None);
        assert!(!inv[0].verb_recognized);
    }

    #[test]
    fn dynamic_format_is_not_inventoried_by_class() {
        // A `$fmt`-built format defeats static reading ⇒ recognized as an emission (the sink is
        // static) but its class is unreadable (tier-3 fallback). The redirect target is static, so
        // `report_sink` fires, but `words[1]` is not a literal.
        let src = "\
x__is_converged() {
   fmt='decline unsound x'
   case $1 in
   z) printf \"$fmt\" >>\"${DREP_V1:-/dev/null}\"
      return 2 ;;
   esac
}";
        let inv = inventory(src);
        assert_eq!(inv.len(), 1, "the emission is recognized (static sink)");
        assert_eq!(
            inv[0].class, None,
            "but the dynamic format is unreadable statically"
        );
    }

    #[test]
    fn a_body_that_emits_nothing_has_an_empty_inventory() {
        // `empty-world-byte-identical`: an ordinary verdict body emits no report ⇒ no tier-3 drain.
        let src = "x__is_converged() { case $1 in on) systemctl is-active -- \"$2\" ;; esac }";
        assert!(inventory(src).is_empty());
        let mut i = Interner::default();
        let set = VerdictSet::lift(&mut i, src);
        let p = set.value.providers().next().unwrap();
        assert!(!emits_report(set.value.get(p).unwrap()));
    }
}
