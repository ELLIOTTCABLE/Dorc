//! `payload` — payload decomposition: the accept/refuse basic-forms table, the nested parse at
//! analysis time, and the whole-line fold (`24T` §4/§5 · `274` · `27J` §2.3).
//!
//! When an eval'er reentry ([`crate::evaler`]) carries a `-c <payload>` shape in an INVITED room, the
//! payload STRING is a book fragment analyzed in that room (`24T` §1: "the engine re-parses it as
//! plain sh under the site's evaluation environment, the inner commands join the same analysis as
//! verdict-participating nodes, and the LINE keeps a single disposition"). This module is the
//! self-contained MODEL of that decomposition, in three parts:
//!
//! 1. **[`classify_payload_form`]** — the accept/refuse decision (`24T:P-A3` basic-forms, §5b): a
//!    single-quoted / trivially-constant literal is ACCEPTED (first-class); a double-quoted /
//!    interpolated / spliced payload is REFUSED to ⊤-with-cause (`imp-P1` holes stay walls). This is
//!    the accept/refuse table.
//! 2. **[`parse_payload`]** — the nested parse (`24T` §8): the accepted literal re-parses as
//!    book-code via `dorc_syntax::parse`; an unparsable payload degrades SITE-LOCALLY (`pin3`
//!    parse-failure-degrades), and no nested annotation is tolerated inside the blob
//!    (`271:rul-no-nested-annotation`, plan-time parse-failure tier). The inner leaves carry
//!    payload-relative spans — the derived-text locators (`24T` §8; `rebase` lifts them into book
//!    coordinates).
//! 3. **[`fold_line`]** — the whole-line fold (`24T` §4a, `pin4` whole-line-unit): fine-grained
//!    analysis, coarse-grained disposition. The site elides IFF every payload leaf elides; a
//!    guard-conjunction composes the diverged leaves' checks; anything unresolvable ⇒ RUN.
//!
//! # Scope fence (MODELS only; `24T` §5b/§9 fences)
//!
//! Like `crate::wrapper`/`crate::evaler`, nothing here is consumed by `analysis`/`plan` yet
//! (rung-0 byte-stable). The per-leaf disposition [`fold_line`] folds is SUPPLIED (the real
//! inner-node classification under ρ is the plan-wiring follow-on — `24T` §8's inner-node stage);
//! this settles the fold ALGEBRA and the accept/refuse frontier. FENCED even so (`24T` §5b/§9):
//! general syntax-position holes (`imp-P1`), automata carriage, loop-assembly (`imp-P2`), and R1
//! span-edits inside verbatim bodies all stay out — the accept/refuse frontier draws exactly that
//! line. **The synthesized-payload-render door stays OPEN** (`27D:rul-synthesized-payload-render-
//! stays-unwelded`): nothing here re-serializes a reconstructed payload — the fold decides the
//! OUTER leaf's disposition and never rewrites payload bytes, so a future un-refusal is unforeclosed.

use dorc_syntax::NodeKind;
use dorc_syntax::ast::{Ast, WordPart};
use dorc_syntax::sem::{DORC_PREFIX, const_literal_text};

/// The accept/refuse verdict for a payload word at a book site (`24T:P-A3` basic-forms; §5b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PayloadForm {
    /// **ACCEPTED** — a single-quoted or trivially-constant literal payload (`sh -c 'systemctl …'`).
    /// First-class: it re-parses as book-code and its leaves join the fold (`24T` §2 cell (c) — the
    /// lint-taught fix shape). Carries the resolved literal text.
    Literal(String),
    /// **REFUSED to ⊤-with-cause** — an interpolated / spliced / unresolvable payload
    /// (`sh -c "systemctl restart $SVC"`, the §2 cell (a) hole). The site RUNS; a diagnostic names
    /// the blocker (`inv-top-reject`, loud). Double-quoted/interpolated is refused unless trivially
    /// constant (`24T:P-A3`).
    RefusedTop(PayloadTopCause),
}

/// Why a payload was refused to ⊤ (`24T` §5a cliffs; §2 the stage-rule). Each names the exact
/// blocker for the loud diagnostic (`24T` §8 dq-payload-splice-⊤).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadTopCause {
    /// A live parameter/command-substitution splice reaches the payload (`"$SVC"`, `$(…)`): the
    /// child re-lexes the VALUE as code — the `imp-P1` hole. The one-line repair is the move to a
    /// literal-payload positional form (`24T` §2 hint-nudge corollary).
    InterpolatedSplice,
    /// An empty payload (`sh -c ''`) — degenerate, nothing to decompose; runs (the safe direction,
    /// not a claimed elision).
    Empty,
}

/// Classify a book site's `-c` payload WORD (its `dorc_syntax` [`WordPart`]s) into accept/refuse
/// (`24T:P-A3`; §5b basic-forms). Pure/total.
///
/// * every part literal (single-quoted, or a const double-quoted/unquoted run) ⇒
///   [`PayloadForm::Literal`] — the resolved-literal rung (`24T` §5a), first-class;
/// * ANY expansion (a `$var`/`$()`/arithmetic/operator splice, quoted or not) ⇒
///   [`PayloadForm::RefusedTop`]`(InterpolatedSplice)` — the hole stays a wall (`imp-P1`;
///   double-quoted/interpolated refused unless trivially constant).
///
/// This deliberately draws the accept frontier at [`const_literal_text`] (the "no variables at all"
/// case): the value-plane-resolved template rung (`CMD="…"; sh -c "$CMD"`, `24T` pin5) and the
/// basic-set-form (`24T` §5b bounded literal-SET) are the punt-empowered EXPLORATION cells — recorded
/// as follow-ons, not accepted here (the conservative spike floor; `24T` §5b "puntable there if it
/// proves research-grade").
#[must_use]
pub fn classify_payload_form(parts: &[WordPart]) -> PayloadForm {
    match const_literal_text(parts) {
        Some(text) if text.is_empty() => PayloadForm::RefusedTop(PayloadTopCause::Empty),
        Some(text) => PayloadForm::Literal(text),
        None => PayloadForm::RefusedTop(PayloadTopCause::InterpolatedSplice),
    }
}

/// The nested parse of an ACCEPTED literal payload (`24T` §8): the inner book leaves + their
/// payload-relative spans (the derived-text locators), or a site-local degrade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedPayload {
    /// The payload parsed as book-code. `leaves` is the count of top-level command leaves (the
    /// fold's inputs — each becomes a verdict-participating node once analysis wiring lands). `spans`
    /// are payload-relative byte ranges (the derived-text locators; [`rebase`](ParsedPayload::rebase)
    /// lifts them into book coordinates).
    Parsed {
        /// Number of top-level command leaves in the payload.
        leaves: usize,
        /// Payload-relative `(lo, hi)` spans of the leaves, in source order.
        spans: Vec<(u32, u32)>,
    },
    /// Site-local degrade (`pin3` parse-failure-degrades): the payload is out-of-dialect / carries a
    /// ⊤-reject node, or a nested annotation (`271:rul-no-nested-annotation`). The SITE walls
    /// (runs); NEVER a book-level error. Carries the cause.
    Degrade(PayloadParseDegrade),
}

/// Why a payload parse degraded site-locally (`24T` pin3; `271:rul-no-nested-annotation`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadParseDegrade {
    /// The payload contains an unmodeled construct (a `⊤`-reject node — bashisms, exotica): the
    /// per-site-refuse posture (`24T` pin3; `dq-payload-unparsable`).
    Unparsable,
    /// The payload carries dorc annotation syntax (a `dorc:` prefix mark) inside the opaque blob —
    /// forbidden (`271:rul-no-nested-annotation`; `274` §1: annotation-syntax inside body-blobs is a
    /// plan-time parse-failure). Payloads are plain sh, permanently (`imp-P6`).
    NestedAnnotation,
}

/// Parse an accepted literal payload as book-code (`24T` §8). Detects the two site-local degrades
/// (`ParsedPayload::Degrade`) before counting leaves. Pure/total (`inv-no-throw`).
///
/// The nested annotation check is SYNTACTIC and conservative: a `dorc:` prefix token anywhere in the
/// payload (the only annotation that survives into a string interior — marks need surrounding
/// whitespace/structure the book parser would surface differently) refuses. Payloads are plain sh,
/// permanently (`imp-P6`), so this is a hard fence, not a lint.
#[must_use]
pub fn parse_payload(literal: &str) -> ParsedPayload {
    // `271:rul-no-nested-annotation`: no dorc annotation syntax inside the opaque blob.
    if literal.contains(DORC_PREFIX) {
        return ParsedPayload::Degrade(PayloadParseDegrade::NestedAnnotation);
    }
    let ast = dorc_syntax::parse(literal).value;
    if ast_has_unsupported(&ast) {
        return ParsedPayload::Degrade(PayloadParseDegrade::Unparsable);
    }
    let spans = top_level_leaf_spans(&ast);
    ParsedPayload::Parsed {
        leaves: spans.len(),
        spans,
    }
}

impl ParsedPayload {
    /// Lift the payload-relative leaf spans into BOOK coordinates by adding the payload's byte offset
    /// within the book (`24T` §8 derived-text locators). A single-quoted `-c 'STR'` payload's STR
    /// bytes are verbatim in the book at `payload_offset`, so the derived locator is exact (the
    /// quoted-heredoc / single-quote-literal case where provenance is nearly free). A no-op on a
    /// degrade.
    #[must_use]
    pub fn rebase(&self, payload_offset: u32) -> Vec<(u32, u32)> {
        match self {
            ParsedPayload::Parsed { spans, .. } => spans
                .iter()
                .map(|&(lo, hi)| {
                    (
                        lo.saturating_add(payload_offset),
                        hi.saturating_add(payload_offset),
                    )
                })
                .collect(),
            ParsedPayload::Degrade(_) => Vec::new(),
        }
    }
}

/// Whether the AST contains any `Unsupported` (⊤-reject) node — the payload is out of the modeled
/// book dialect ⇒ site-local degrade (`24T` pin3).
fn ast_has_unsupported(ast: &Ast) -> bool {
    ast.iter()
        .any(|(_, n)| matches!(n.kind, NodeKind::Unsupported { .. }))
}

/// The payload-relative spans of the top-level command leaves (the fold's inputs), in source order.
/// A leaf is a `Simple`/`Pipeline`/`AndOr` item under the `Script` root — the whole-line units the
/// fold decides over (the payload is "a separate book", `24T` §4c R1 framing).
fn top_level_leaf_spans(ast: &Ast) -> Vec<(u32, u32)> {
    let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
        return Vec::new();
    };
    items
        .iter()
        .map(|&id| {
            let s = ast.node(id).span;
            (s.lo.0, s.hi.0)
        })
        .collect()
}

/// One payload leaf's fold contribution (`24T` §4a). SUPPLIED by the caller (the real inner-node
/// classification under ρ is the plan-wiring follow-on); this module folds the algebra.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeafFold {
    /// The leaf's effect predicts converged-and-vouched ⇒ it contributes NO obstacle to eliding the
    /// whole line (`24T` §4a Elide clause: "every effect-bearing inner node has a reached
    /// converged-vouch").
    Elides,
    /// The leaf can't elide but has a licensed check ⇒ it contributes a guard CONJUNCT (`24T` §4a
    /// Guard clause: the guard is a conjunction of inner checks placed OUTSIDE the line).
    Guards,
    /// The leaf is unresolvable / unmodeled (a bounded wall, no check) ⇒ it forces the whole line to
    /// RUN (`24T` §4a Run clause; the lowest-tier sin, unnecessary-execution).
    Runs,
}

/// The whole-line fold disposition (`24T` §4a pin4 whole-line-unit): one outcome for the OUTER leaf
/// (only the USER's bytes are never subdivided — `24T` §4a).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineFold {
    /// Every payload leaf elides ⇒ the whole site elides (`24T` §4a: "Elide(line) ⟺ every
    /// effect-bearing inner node has a reached converged-vouch").
    Elide,
    /// Not elidable, but every diverged-relevant leaf has a licensed check ⇒ a guard-conjunction of
    /// `conjuncts` inner checks placed OUTSIDE the line (`24T` §4a Guard clause).
    GuardConjunction {
        /// How many leaves contribute a guard conjunct.
        conjuncts: usize,
    },
    /// Some leaf is unresolvable ⇒ the whole line RUNS (`24T` §4a; the safe direction). Also the
    /// degenerate empty-payload / parse-degrade fold.
    Run,
}

/// Fold a payload's per-leaf dispositions into ONE line disposition (`24T` §4a). The precedence is
/// the fail-direction (`execution-priority-order`): ANY `Runs` ⇒ the whole line runs; else if EVERY
/// leaf elides ⇒ elide; else a guard-conjunction of the `Guards` leaves. An EMPTY leaf set (a payload
/// that parsed to nothing) ⇒ `Run` — the conservative floor, never a vacuous elision claim.
#[must_use]
pub fn fold_line(leaves: &[LeafFold]) -> LineFold {
    if leaves.is_empty() {
        return LineFold::Run;
    }
    // Any unresolvable leaf ⇒ the whole line runs (re-running individually-converged inners is the
    // lowest-tier sin; the safe direction).
    if leaves.iter().any(|l| matches!(l, LeafFold::Runs)) {
        return LineFold::Run;
    }
    // Every leaf elides ⇒ the site elides.
    if leaves.iter().all(|l| matches!(l, LeafFold::Elides)) {
        return LineFold::Elide;
    }
    // Mixed elide/guard ⇒ a guard-conjunction of the diverged leaves' checks.
    let conjuncts = leaves
        .iter()
        .filter(|l| matches!(l, LeafFold::Guards))
        .count();
    LineFold::GuardConjunction { conjuncts }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parse a book fragment's first command word's operand as a `-c` payload — i.e. lex a
    /// `sh -c <PAYLOAD>` book line and return the PAYLOAD word's parts. Keeps the tests grounded in
    /// real book bytes (the accept/refuse table is over what an admin actually writes).
    fn payload_parts(book_line: &str) -> Vec<WordPart> {
        let ast = dorc_syntax::parse(book_line).value;
        let NodeKind::Script { items } = &ast.node(ast.root()).kind else {
            panic!("script root");
        };
        let NodeKind::Simple { words, .. } = &ast.node(items[0]).kind else {
            panic!("simple command");
        };
        // words = [sh, -c, PAYLOAD]; take the third.
        let NodeKind::Word { parts } = &ast.node(words[2]).kind else {
            panic!("payload word");
        };
        parts.clone()
    }

    // ── the accept/refuse basic-forms table (`24T:P-A3`) ────────────────────────

    #[test]
    fn single_quoted_literal_payload_is_accepted() {
        // `sh -c 'systemctl restart nginx'` — the lint-taught cell (c): first-class literal.
        let parts = payload_parts("sh -c 'systemctl restart nginx'");
        assert_eq!(
            classify_payload_form(&parts),
            PayloadForm::Literal("systemctl restart nginx".to_owned())
        );
    }

    #[test]
    fn const_double_quoted_payload_is_accepted() {
        // `sh -c "systemctl restart nginx"` with NO splice — trivially constant ⇒ accepted.
        let parts = payload_parts("sh -c \"systemctl restart nginx\"");
        assert_eq!(
            classify_payload_form(&parts),
            PayloadForm::Literal("systemctl restart nginx".to_owned())
        );
    }

    #[test]
    fn interpolated_payload_is_refused_to_top() {
        // `sh -c "systemctl restart $SVC"` — the §2 cell (a) HOLE: a live splice reaches the child's
        // parse ⇒ refused to ⊤-with-cause (`imp-P1`). The site runs.
        let parts = payload_parts("sh -c \"systemctl restart $SVC\"");
        assert_eq!(
            classify_payload_form(&parts),
            PayloadForm::RefusedTop(PayloadTopCause::InterpolatedSplice)
        );
    }

    #[test]
    fn command_subst_payload_is_refused_to_top() {
        // `sh -c "$(build_cmd)"` — a command-substitution splice ⇒ refused (`24T` §5a cmdsub cliff).
        let parts = payload_parts("sh -c \"$(build_cmd)\"");
        assert_eq!(
            classify_payload_form(&parts),
            PayloadForm::RefusedTop(PayloadTopCause::InterpolatedSplice)
        );
    }

    #[test]
    fn empty_payload_is_refused_empty() {
        let parts = payload_parts("sh -c ''");
        assert_eq!(
            classify_payload_form(&parts),
            PayloadForm::RefusedTop(PayloadTopCause::Empty)
        );
    }

    // ── nested parse + derived-text locators (`24T` §8) ─────────────────────────

    #[test]
    fn literal_payload_parses_into_leaves_with_spans() {
        // A two-command payload parses into two leaves; the spans are payload-relative locators.
        let p = parse_payload("systemctl restart nginx; systemctl reload nginx");
        let ParsedPayload::Parsed { leaves, spans } = &p else {
            panic!("parsed, got {p:?}");
        };
        assert_eq!(*leaves, 2, "two command leaves");
        assert_eq!(spans.len(), 2);
        // First leaf begins at payload offset 0.
        assert_eq!(spans[0].0, 0);
        // Rebase lifts them into book coordinates (the derived-text locator).
        let rebased = p.rebase(8); // payload sits at book byte 8 (e.g. after `sh -c '`)
        assert_eq!(rebased[0].0, 8);
    }

    #[test]
    fn nested_annotation_in_payload_degrades() {
        // A `dorc:` prefix inside the payload is forbidden (`271:rul-no-nested-annotation`): payloads
        // are plain sh, permanently (`imp-P6`). Site-local degrade.
        let p = parse_payload("dorc:sh -c 'echo hi'");
        assert_eq!(
            p,
            ParsedPayload::Degrade(PayloadParseDegrade::NestedAnnotation)
        );
    }

    #[test]
    fn unparsable_payload_degrades_site_local() {
        // A ⊤-reject construct (an `eval`) inside the payload ⇒ Unparsable degrade (`pin3`), never a
        // book-level error.
        let p = parse_payload("eval \"$x\"");
        assert_eq!(p, ParsedPayload::Degrade(PayloadParseDegrade::Unparsable));
    }

    // ── the whole-line fold (`24T` §4a) ─────────────────────────────────────────

    #[test]
    fn line_elides_iff_every_leaf_elides() {
        assert_eq!(
            fold_line(&[LeafFold::Elides, LeafFold::Elides]),
            LineFold::Elide
        );
    }

    #[test]
    fn one_running_leaf_forces_the_whole_line_to_run() {
        // The fail-direction: any unresolvable leaf ⇒ run, even alongside eliding leaves.
        assert_eq!(
            fold_line(&[LeafFold::Elides, LeafFold::Runs, LeafFold::Guards]),
            LineFold::Run
        );
    }

    #[test]
    fn mixed_elide_and_guard_folds_to_guard_conjunction() {
        // Not all elide, no running leaf ⇒ a guard-conjunction of the diverged leaves' checks.
        assert_eq!(
            fold_line(&[LeafFold::Elides, LeafFold::Guards, LeafFold::Guards]),
            LineFold::GuardConjunction { conjuncts: 2 }
        );
    }

    #[test]
    fn empty_leaf_set_folds_to_run() {
        // The conservative floor: nothing to fold ⇒ run, never a vacuous elision claim.
        assert_eq!(fold_line(&[]), LineFold::Run);
    }
}
