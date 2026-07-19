//! The `281` annotation mark-grammar parser — the NEW-grammar path, landed UNWIRED.
//!
//! This module implements `plans/281` Part I (the mark grammar) against the CP-A typed
//! [`MarkKind`] verb set. It lands UNWIRED — `#[cfg(test)]`-gated at the `mod` declaration
//! (`super`): the production lift/strip path stays on the OLD grammar until the CP-D cutover
//! (`28A:rul-respell-atomic-cutover`), which un-gates and wires it. So the corpus is untouched
//! and e2e stays green on the old spellings. Everything here is exercised only by this module's
//! own unit tests, which pin the license-bearing parse behaviors the conductor reviews at CP-C.
//! (The four parse diagnostics' literal emit sites still satisfy the production-emit grep —
//! `diag_tidy::every_catalog_variant_is_constructed` scans `#[cfg(test)]` modules too.)
//!
//! Design (`281` §3–§9):
//! * intros `: :! :? := #: #:! #:? #:=` ([`decode_intro`]); the `#:` carrier reaches the lexer
//!   through [`super::lexer::lex_marks`] (immediate-colon rule, `281` §1).
//! * three-rule head decode + verb-driven tail ([`decode_line_marks`]); verbs are period-free,
//!   coordinate kinds carry ≥2 periods (`281` §4 keystone).
//! * `@` selector via the parametrized [`super::parser::split_mark_target`]; brace-alternation
//!   both shapes (attached `@{a,b}`, standalone payload word `verb {a,b}`), refused on
//!   asserts/refutes payloads (the reused `MarkBraceVerdictSingleCell`).
//! * continuation lines, rc-arity over the whole block, standalone-block detection
//!   (`28A:rul-continuation-attachment`, `281` §7).
//! * inline bind dispatched on the `= value` tail before head-decode
//!   (`28A:rul-bind-equals-tail-disambiguates`); value-less binds are not recognized
//!   (`28A:rul-singleton-bind-drops`).
//! * statement-leading `:` is a mark intro when followed by content, the null command when lone
//!   (`28A:rul-marked-colon-is-the-grammars`).
//!
//! The four `281` parse diagnostics ([`Code::MarkUnknownVerb`], [`Code::MarkRcArityExceeded`],
//! [`Code::MarkStandaloneRcConsumer`], [`Code::MarkHashcolonMalformed`]) are emitted here — the
//! literal production emit sites `diag_tidy::every_catalog_variant_is_constructed` requires.

use dorc_core::Span;
use dorc_core::diag::{
    Diag, DiagCode as Code, MarkBraceVerdictSingleCell, MarkHashcolonMalformed,
    MarkRcArityExceeded, MarkStandaloneRcConsumer, MarkUnknownVerb,
};
use dorc_syntax::sem;

use super::ast::{Mark, MarkKind, MarkTarget};
use super::lexer::{Tok, Token, lex_marks};
use super::parser::{brace_tokens, is_valid_selector, split_mark_target};

/// The `281` selector introducer: `@` (the respell of the old `#`, `281` §R4).
const SELECTOR: char = '@';

/// The engine-owned mark verb vocabulary (`281` §5), closed at a version, extends by new name
/// only. `bind` is the value-plane verb (`281` §8); the rest map to a coordinate/meta
/// [`MarkKind`]. Order fixes the [`expected_verbs`] disclosure list.
const VERBS: &[(&str, Verb)] = &[
    ("asserts", Verb::Mark(MarkKind::Asserts)),
    ("refutes", Verb::Mark(MarkKind::Refutes)),
    ("reads", Verb::Mark(MarkKind::Reads)),
    ("bind", Verb::Bind),
    ("safe-across", Verb::Mark(MarkKind::SafeAcross)),
    ("disturbs", Verb::Mark(MarkKind::Disturbs)),
    ("lends", Verb::Mark(MarkKind::Lends)),
    ("stored-in", Verb::Mark(MarkKind::StoredIn)),
    (
        "undivided-by-transit-across",
        Verb::Mark(MarkKind::Undivided),
    ),
];

/// A decoded mark verb: a coordinate/meta [`MarkKind`], or the value-plane `bind` (`281` §8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verb {
    /// A verb that annotates the line — the typed CP-A discriminant.
    Mark(MarkKind),
    /// `bind` / `=` sugar — types the assigned VALUE, never a cell (`281` §8).
    Bind,
}

impl Verb {
    /// The verb for `word`, or `None` if it is not a known verb (an `281` §4 rule-3 miss).
    fn of(word: &str) -> Option<Verb> {
        VERBS.iter().find(|(w, _)| *w == word).map(|(_, v)| *v)
    }
}

/// The comma-joined known-verb vocabulary, for the [`Code::MarkUnknownVerb`] disclosure.
fn expected_verbs() -> String {
    VERBS.iter().map(|(w, _)| *w).collect::<Vec<_>>().join(", ")
}

/// The two mark carriers (`281` §1): the salient default `:` and the inert comment `#:`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Carrier {
    /// `:` — the salient colon form (highlights as shell; corrupts under a raw run).
    Colon,
    /// `#:` — the inert hash-colon comment carrier.
    Hash,
}

/// The head-only sugar characters (`281` §3): the core cell-and-value shortcuts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sugar {
    /// `!` — the complement verdict (`refutes`).
    Bang,
    /// `?` — the read (`reads`).
    Question,
    /// `=` — the bind (`281` §8).
    Equals,
}

/// A decoded intro `( ':' | '#:' ) [ SUGAR ]` (`281` §3): the carrier plus optional head sugar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Intro {
    carrier: Carrier,
    sugar: Option<Sugar>,
}

/// Decode an intro lexeme to its carrier + optional sugar (`281` §3), or `None` if `lexeme` is
/// not one of the eight legal intros. The immediate-colon `#:` disambiguation happens in the
/// lexer; here the lexeme is already the whole intro word.
fn decode_intro(lexeme: &str) -> Option<Intro> {
    let (carrier, rest) = match lexeme.strip_prefix("#:") {
        Some(rest) => (Carrier::Hash, rest),
        None => (Carrier::Colon, lexeme.strip_prefix(':')?),
    };
    let sugar = match rest {
        "" => None,
        "!" => Some(Sugar::Bang),
        "?" => Some(Sugar::Question),
        "=" => Some(Sugar::Equals),
        _ => return None,
    };
    Some(Intro { carrier, sugar })
}

/// The verb a head sugar fixes (`281` §3/§4 rule 1): `!`→refutes, `?`→reads, `=`→bind.
fn sugar_verb(sugar: Sugar) -> Verb {
    match sugar {
        Sugar::Bang => Verb::Mark(MarkKind::Refutes),
        Sugar::Question => Verb::Mark(MarkKind::Reads),
        Sugar::Equals => Verb::Bind,
    }
}

/// A word token with its source span — the unit the mark decode consumes.
#[derive(Debug, Clone)]
struct SpannedWord {
    lexeme: String,
    span: Span,
}

/// The outcome of decoding one intro line's marks: the typed coordinate/meta marks, any bind
/// payloads (value-plane), and accumulated diagnostics.
#[derive(Debug, Default)]
struct LineMarks {
    marks: Vec<Mark>,
    binds: Vec<Bind>,
    diags: Vec<Diag>,
}

/// A `bind KIND` payload (`281` §8): types the assigned value as an entity of this kind.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Bind {
    kind: String,
    span: Span,
}

/// Is a verb rc-consuming (a verdict, `281` §5/§7)? Only `asserts`/`refutes` map one exit code
/// onto one cell; every other verb is rc-free.
fn is_rc_consumer(kind: MarkKind) -> bool {
    matches!(kind, MarkKind::Asserts | MarkKind::Refutes)
}

/// Decode one intro line's marks (`281` §4): the three-rule head decode driven by this line's
/// intro sugar, then a verb-driven tail. `words` are the payload words after the intro. Emits
/// `mark-unknown-verb` on a rule-3 miss and `mark-brace-verdict-single-cell` on a brace over a
/// verdict payload (reused, `281` §6). NB the head decode runs per intro line — each
/// continuation re-intros (`281` §3) — a reading flagged for the conductor.
fn decode_line_marks(intro: Intro, words: &[SpannedWord]) -> LineMarks {
    let mut out = LineMarks::default();
    let mut i = 0;
    if let Some(sugar) = intro.sugar {
        i = consume_verb(sugar_verb(sugar), words, i, &mut out);
    } else if let Some(first) = words.first() {
        if first.lexeme.contains('.') {
            i = consume_verb(Verb::Mark(MarkKind::Asserts), words, i, &mut out);
        } else if let Some(next) = consume_verb_word(words, i, &mut out) {
            i = next;
        } else {
            return out;
        }
    }
    while let Some(next) = consume_verb_word(words, i, &mut out) {
        i = next;
    }
    out
}

/// Read a verb WORD at `words[i]` and consume it + its payload (`281` §4 verb-driven tail),
/// returning the next index — or `None` on a `mark-unknown-verb` miss, which HALTS the line
/// decode (root-cause only: the block is ⊤, so a period-bearing coordinate landing in verb
/// position is not re-flagged, `AGENTS.md` fail-fast).
fn consume_verb_word(words: &[SpannedWord], i: usize, out: &mut LineMarks) -> Option<usize> {
    let word = words.get(i)?;
    if let Some(verb) = Verb::of(&word.lexeme) {
        return Some(consume_verb(verb, words, i.saturating_add(1), out));
    }
    out.diags.push(Diag::new(
        Code::MarkUnknownVerb(MarkUnknownVerb {
            token: word.lexeme.clone(),
            expected: expected_verbs(),
        }),
        word.span,
    ));
    None
}

/// Consume a verb's single payload word (`281` §5 arities are all one payload) starting at `i`,
/// pushing the resulting mark/bind. Returns the next index.
fn consume_verb(verb: Verb, words: &[SpannedWord], i: usize, out: &mut LineMarks) -> usize {
    let Some(payload) = words.get(i) else {
        return i;
    };
    match verb {
        Verb::Bind => out.binds.push(Bind {
            kind: payload.lexeme.clone(),
            span: payload.span,
        }),
        Verb::Mark(kind) => push_coordinate_mark(kind, payload, out),
    }
    i.saturating_add(1)
}

/// Build a coordinate/meta mark from `payload` for `kind` and push it, refusing a brace on a
/// verdict payload (`281` §6, the reused single-cell rule). A payload word is a coordinate/token
/// here; the `= value` verdict tail is captured by the statement classifier, not this scan.
fn push_coordinate_mark(kind: MarkKind, payload: &SpannedWord, out: &mut LineMarks) {
    let Some(parsed) = split_mark_target(&payload.lexeme, SELECTOR) else {
        return;
    };
    if let Some(sel) = &parsed.prop {
        if !is_valid_selector(sel) {
            return;
        }
        if is_rc_consumer(kind) && brace_tokens(sel).is_some() {
            out.diags.push(Diag::new(
                Code::MarkBraceVerdictSingleCell(MarkBraceVerdictSingleCell),
                payload.span,
            ));
            return;
        }
    }
    out.marks.push(Mark {
        kind,
        target: MarkTarget {
            kind: parsed.kind,
            entity: parsed.entity,
            prop: parsed.prop,
            value: None,
        },
        span: payload.span,
    });
}

/// Whether a mark block stands alone (`28A:rul-continuation-attachment`): a standalone block has
/// no command to bind, so an rc-consumer or `reads` there is unbacked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockBinding {
    /// The block trails a command (its rc/value is the backing).
    Trailing,
    /// The block stands alone (no command; meta verbs only are well-formed).
    Standalone,
}

/// The rc-arity + standalone diagnostics for an assembled block (`281` §7 ·
/// `28A:rul-continuation-attachment`), computed over ALL the block's marks (continuations
/// included). A second rc-consumer is `mark-rc-arity-exceeded`; an rc-consumer/`reads` in a
/// standalone block is `mark-standalone-rc-consumer`. Both drop the block to ⊤.
fn block_diags(marks: &[Mark], binding: BlockBinding) -> Vec<Diag> {
    let mut diags = Vec::new();
    let mut rc_seen = false;
    for m in marks {
        if is_rc_consumer(m.kind) {
            if rc_seen {
                diags.push(Diag::new(
                    Code::MarkRcArityExceeded(MarkRcArityExceeded),
                    m.span,
                ));
            }
            rc_seen = true;
        }
        if binding == BlockBinding::Standalone
            && matches!(
                m.kind,
                MarkKind::Asserts | MarkKind::Refutes | MarkKind::Reads
            )
        {
            diags.push(Diag::new(
                Code::MarkStandaloneRcConsumer(MarkStandaloneRcConsumer),
                m.span,
            ));
        }
    }
    diags
}

/// One statement's mark-relevant classification (`281` §4/§8 · `28A` rulings).
#[derive(Debug)]
enum StmtKind {
    /// A command with an optional trailing mark block (`marks` empty ⇒ a plain command).
    Command { marks: Vec<Mark> },
    /// A statement-leading intro mark block (standalone unless a continuation attaches it).
    Standalone { marks: Vec<Mark>, intro: Intro },
    /// An inline bind `name : KIND = value` (`281` §8; the `= value` tail disambiguates).
    InlineBind {
        name: String,
        kind: String,
        span: Span,
    },
    /// A trailing bind riding an assignment (`FOO=bar := KIND` / `: bind KIND`).
    TrailingBind { bind: Bind, intro: Intro },
    /// A lone `:` — the POSIX null command (`28A:rul-marked-colon-is-the-grammars`).
    NullColon,
    /// A plain statement carrying no mark (keywords, ordinary commands, assignments).
    Plain,
}

/// One parsed statement: its classification, its physical-line index (for continuation), and
/// the diagnostics its own decode produced (unknown-verb, brace, hashcolon-malformed).
#[derive(Debug)]
struct ParsedStmt {
    kind: StmtKind,
    line: usize,
    diags: Vec<Diag>,
}

/// The full new-grammar parse of an oracle body: every statement classified, plus the
/// block-level diagnostics (rc-arity, standalone) computed after continuation assembly.
#[derive(Debug, Default)]
struct Parsed {
    stmts: Vec<ParsedStmt>,
    block_diags: Vec<Diag>,
}

/// Parse the marks in an oracle `src` under the new grammar (`281`), UNWIRED from production.
/// Lexes with the hash-colon-aware [`lex_marks`], splits into statements (on `;`/`;;`/newline),
/// classifies each, assembles continuation blocks (`28A:rul-continuation-attachment`), and runs
/// the block-level rc-arity + standalone diagnostics over each assembled block. The landed
/// entry point CP-D wires into lift/strip; nothing in the production pipeline calls it yet.
#[must_use]
fn parse_marks(src: &str) -> Parsed {
    let tokens = lex_marks(src);
    let mut out = Parsed::default();
    let statements = split_statements(&tokens);
    for stmt in &statements {
        out.stmts.push(classify_statement(stmt));
    }
    out.block_diags = assemble_blocks(&out.stmts);
    out
}

/// A statement slice: its word/redirect tokens (separators dropped) plus the physical-line
/// index of its first token (for continuation attachment).
#[derive(Debug)]
struct RawStmt {
    words: Vec<SpannedWord>,
    line: usize,
}

/// Split the token stream into statements on `;`/`;;`/newline, tracking the physical line of
/// each. Only Word tokens carry marks; non-word tokens are kept as empty-lexeme placeholders so
/// a command is distinguishable from a bare intro (their exact bytes do not matter here).
fn split_statements(tokens: &[Token]) -> Vec<RawStmt> {
    let mut stmts = Vec::new();
    let mut cur: Vec<SpannedWord> = Vec::new();
    let mut line = 0usize;
    let mut cur_line = 0usize;
    for tok in tokens {
        match &tok.kind {
            Tok::Newline => {
                flush_stmt(&mut cur, cur_line, &mut stmts);
                line = line.saturating_add(1);
            }
            Tok::Semi | Tok::DSemi => flush_stmt(&mut cur, cur_line, &mut stmts),
            other => {
                if cur.is_empty() {
                    cur_line = line;
                }
                let lexeme = if let Tok::Word { lexeme, .. } = other {
                    lexeme.clone()
                } else {
                    String::new()
                };
                cur.push(SpannedWord {
                    lexeme,
                    span: tok.span,
                });
            }
        }
    }
    flush_stmt(&mut cur, cur_line, &mut stmts);
    stmts
}

/// Flush the current statement's accumulated words into `stmts` (a no-op when empty).
fn flush_stmt(cur: &mut Vec<SpannedWord>, line: usize, stmts: &mut Vec<RawStmt>) {
    if !cur.is_empty() {
        stmts.push(RawStmt {
            words: std::mem::take(cur),
            line,
        });
    }
}

/// Shell keywords that open/continue a block; a statement led by one carries no mark of its own.
/// Kept minimal — enough to keep `while :; do` honest (`28A:rul-marked-colon-is-the-grammars`).
fn is_keyword(lexeme: &str) -> bool {
    matches!(
        lexeme,
        "while"
            | "until"
            | "if"
            | "elif"
            | "then"
            | "else"
            | "fi"
            | "do"
            | "done"
            | "case"
            | "in"
            | "esac"
            | "for"
    )
}

/// Classify one statement (`281` §4/§8 · `28A` rulings). The inline-bind `= value` tail decides
/// first (`28A:rul-bind-equals-tail-disambiguates`), then the intro position fixes the shape.
fn classify_statement(stmt: &RawStmt) -> ParsedStmt {
    let words = &stmt.words;
    let Some(pos) = words.iter().position(|w| decode_intro(&w.lexeme).is_some()) else {
        return done(StmtKind::Plain, stmt.line, Vec::new());
    };
    let intro = decode_intro(&words[pos].lexeme).unwrap_or(Intro {
        carrier: Carrier::Colon,
        sugar: None,
    });
    let intro_span = words[pos].span;
    let payload = words.get(pos.saturating_add(1)..).unwrap_or(&[]);

    let leading_kw = words.first().is_some_and(|w| is_keyword(&w.lexeme));
    if pos == 0 || (leading_kw && pos == 1) {
        if intro.carrier == Carrier::Colon && intro.sugar.is_none() && payload.is_empty() {
            return done(StmtKind::NullColon, stmt.line, Vec::new());
        }
        return standalone_or_malformed(intro, intro_span, payload, stmt.line);
    }

    if let Some(bind) = inline_bind(words, pos, intro) {
        return done(bind, stmt.line, Vec::new());
    }
    let mut lm = decode_line_marks(intro, payload);
    let diags = hashcolon_gate(intro, intro_span, &lm);
    if is_trailing_bind(intro, payload) {
        return match lm.binds.pop() {
            Some(bind) => done(StmtKind::TrailingBind { bind, intro }, stmt.line, diags),
            None => done(StmtKind::Plain, stmt.line, diags),
        };
    }
    done(StmtKind::Command { marks: lm.marks }, stmt.line, diags)
}

/// A statement-leading intro block: standalone marks, or (for a `#:` carrier that did not
/// parse) a `mark-hashcolon-malformed` note leaving the line a comment (`281` §9).
fn standalone_or_malformed(
    intro: Intro,
    intro_span: Span,
    payload: &[SpannedWord],
    line: usize,
) -> ParsedStmt {
    let lm = decode_line_marks(intro, payload);
    let diags = hashcolon_gate(intro, intro_span, &lm);
    if intro.carrier == Carrier::Hash && !lm.diags.is_empty() {
        return done(StmtKind::Plain, line, diags);
    }
    done(
        StmtKind::Standalone {
            marks: lm.marks,
            intro,
        },
        line,
        diags,
    )
}

/// `281` §9 hash-colon gate: a `#:` block that failed to parse (any decode diag) is diagnosed
/// `mark-hashcolon-malformed` (anchored at the `#:` intro) and left a comment; a colon block
/// surfaces its decode diags as-is (committed syntax ⇒ loud ⊤).
fn hashcolon_gate(intro: Intro, intro_span: Span, lm: &LineMarks) -> Vec<Diag> {
    if intro.carrier == Carrier::Hash {
        if lm.diags.is_empty() {
            return Vec::new();
        }
        return vec![Diag::new(
            Code::MarkHashcolonMalformed(MarkHashcolonMalformed),
            intro_span,
        )];
    }
    lm.diags.clone()
}

/// The inline bind `name : KIND = value` (`281` §8): a single name word, a bare `:` intro, a
/// bare kind, then a `= value` tail. Returns `None` when any part is absent — no `=` tail ⇒ the
/// `:` is a mark intro (`28A:rul-bind-equals-tail-disambiguates`); a value-less form is NOT a
/// bind (`28A:rul-singleton-bind-drops`).
fn inline_bind(words: &[SpannedWord], pos: usize, intro: Intro) -> Option<StmtKind> {
    if pos != 1 || intro.carrier != Carrier::Colon || intro.sugar.is_some() {
        return None;
    }
    let name = words.first()?;
    if !sem::is_name(&name.lexeme) {
        return None;
    }
    let kind = words.get(pos.saturating_add(1))?;
    let eq = words.get(pos.saturating_add(2))?;
    let value = words.get(pos.saturating_add(3))?;
    if eq.lexeme != "=" || value.lexeme.is_empty() {
        return None;
    }
    Some(StmtKind::InlineBind {
        name: name.lexeme.clone(),
        kind: kind.lexeme.clone(),
        span: name.span.to(value.span),
    })
}

/// A trailing bind rides an assignment (`FOO=bar := KIND` / `FOO=bar : bind KIND`): the `=`
/// sugar, or a `bind` word verb, follows the assignment word (`281` §8).
fn is_trailing_bind(intro: Intro, payload: &[SpannedWord]) -> bool {
    intro.sugar == Some(Sugar::Equals) || payload.first().is_some_and(|w| w.lexeme == "bind")
}

/// Assemble continuation blocks and run their block-level diagnostics
/// (`28A:rul-continuation-attachment`): a statement-leading intro on a physical line that
/// follows a line ending with a mark block continues that block; otherwise it opens a new
/// (standalone) block. rc-arity + standalone verdicts are computed over each assembled block.
fn assemble_blocks(stmts: &[ParsedStmt]) -> Vec<Diag> {
    let mut diags = Vec::new();
    let mut open: Option<(Vec<Mark>, BlockBinding)> = None;
    let mut prev_marked = false;
    let mut prev_line = usize::MAX;
    for st in stmts {
        let new_line = st.line != prev_line;
        match &st.kind {
            StmtKind::Command { marks } => {
                flush_block(&mut open, &mut diags);
                prev_marked = !marks.is_empty();
                if prev_marked {
                    open = Some((marks.clone(), BlockBinding::Trailing));
                }
            }
            StmtKind::Standalone { marks, .. } => {
                if prev_marked && new_line && open.is_some() {
                    if let Some((acc, _)) = open.as_mut() {
                        acc.extend(marks.clone());
                    }
                } else {
                    flush_block(&mut open, &mut diags);
                    open = Some((marks.clone(), BlockBinding::Standalone));
                }
                prev_marked = true;
            }
            _ => {
                flush_block(&mut open, &mut diags);
                prev_marked = false;
            }
        }
        prev_line = st.line;
    }
    flush_block(&mut open, &mut diags);
    diags
}

/// Emit the accumulated block's rc-arity + standalone diagnostics and clear it.
fn flush_block(open: &mut Option<(Vec<Mark>, BlockBinding)>, diags: &mut Vec<Diag>) {
    if let Some((marks, binding)) = open.take() {
        diags.extend(block_diags(&marks, binding));
    }
}

/// Assemble a [`ParsedStmt`] (a small ctor keeping `classify_statement` readable).
fn done(kind: StmtKind, line: usize, diags: Vec<Diag>) -> ParsedStmt {
    ParsedStmt { kind, line, diags }
}

#[cfg(test)]
mod tests {
    //! License-bearing behaviors of the `281` new-grammar parser (mission point 4 · the CP-C
    //! checkpoint surface). Each test pins one rule from `plans/281` + the `28A` rulings; the
    //! parser is UNWIRED, so these are the only exercise it gets pre-cutover.

    use super::*;
    use dorc_core::BytePos;

    /// A [`SpannedWord`] with a distinct 1-byte span per position, for [`decode_line_marks`]
    /// unit inputs (the exact spans are irrelevant to the verb/coordinate assertions).
    fn words(lexemes: &[&str]) -> Vec<SpannedWord> {
        lexemes
            .iter()
            .enumerate()
            .map(|(i, l)| {
                let pos = u32::try_from(i).unwrap_or(0);
                SpannedWord {
                    lexeme: (*l).to_owned(),
                    span: Span::new(BytePos(pos), BytePos(pos.saturating_add(1))),
                }
            })
            .collect()
    }

    fn colon() -> Intro {
        decode_intro(":").expect("colon intro")
    }

    fn slugs(diags: &[Diag]) -> Vec<&'static str> {
        diags.iter().map(|d| d.code.slug()).collect()
    }

    /// All diagnostic slugs across a whole `parse_marks` result (per-statement + block-level).
    fn all_slugs(p: &Parsed) -> Vec<&'static str> {
        let mut out: Vec<&'static str> = p.stmts.iter().flat_map(|s| slugs(&s.diags)).collect();
        out.extend(slugs(&p.block_diags));
        out
    }

    fn marks_of(stmt: &ParsedStmt) -> &[Mark] {
        match &stmt.kind {
            StmtKind::Command { marks } | StmtKind::Standalone { marks, .. } => marks,
            _ => &[],
        }
    }

    /// The eight legal intros decode to the right carrier + sugar; nothing else is an intro
    /// (`281` §3). `# dorc-lang/v0.1`-style comments never reach here — the lexer keeps them.
    #[test]
    fn intros_decode_the_eight_legal_forms() {
        assert_eq!(
            colon(),
            Intro {
                carrier: Carrier::Colon,
                sugar: None
            }
        );
        assert_eq!(decode_intro(":!").unwrap().sugar, Some(Sugar::Bang));
        assert_eq!(decode_intro(":?").unwrap().sugar, Some(Sugar::Question));
        assert_eq!(decode_intro(":=").unwrap().sugar, Some(Sugar::Equals));
        assert_eq!(decode_intro("#:").unwrap().carrier, Carrier::Hash);
        assert_eq!(decode_intro("#:!").unwrap().sugar, Some(Sugar::Bang));
        assert_eq!(decode_intro("#:?").unwrap().sugar, Some(Sugar::Question));
        assert_eq!(decode_intro("#:=").unwrap().sugar, Some(Sugar::Equals));
        for not_intro in ["", "x", ":x", "::", "#", "# ", "#x", ":!?"] {
            assert!(
                decode_intro(not_intro).is_none(),
                "`{not_intro}` is not an intro"
            );
        }
    }

    /// Head rule 1 (`281` §4): an intro sugar fixes the first mark's verb — `:?`→reads,
    /// `:!`→refutes; `:=` routes to a bind, never a coordinate mark.
    #[test]
    fn head_rule_one_sugar_fixes_the_verb() {
        let q = decode_line_marks(decode_intro(":?").unwrap(), &words(&["sm.dorc.X@on"]));
        assert_eq!(q.marks[0].kind, MarkKind::Reads);
        let bang = decode_line_marks(decode_intro(":!").unwrap(), &words(&["sm.dorc.X@on"]));
        assert_eq!(bang.marks[0].kind, MarkKind::Refutes);
        let eq = decode_line_marks(decode_intro(":=").unwrap(), &words(&["sm.dorc.Package"]));
        assert!(eq.marks.is_empty() && eq.binds[0].kind == "sm.dorc.Package");
    }

    /// Head rule 2 (`281` §4): a bare `:` intro whose first token carries a period is a
    /// coordinate, verb `asserts` (the omitted-verb default).
    #[test]
    fn head_rule_two_bare_coordinate_is_asserts() {
        let lm = decode_line_marks(colon(), &words(&["sm.dorc.Service:svc@active"]));
        assert_eq!(lm.marks[0].kind, MarkKind::Asserts);
        assert_eq!(lm.marks[0].target.kind, "sm.dorc.Service");
        assert_eq!(lm.marks[0].target.entity.as_deref(), Some("svc"));
        assert_eq!(lm.marks[0].target.prop.as_deref(), Some("active"));
    }

    /// Head rule 3 (`281` §4 keystone): a period-free first token is a verb word; the tail is
    /// verb-driven (verb + payload, repeat), spanning multiple marks on one line.
    #[test]
    fn head_rule_three_and_verb_driven_tail() {
        let lm = decode_line_marks(
            colon(),
            &words(&["disturbs", "sm.dorc.File", "reads", "sm.dorc.Policy@p"]),
        );
        assert_eq!(lm.marks[0].kind, MarkKind::Disturbs);
        assert_eq!(lm.marks[0].target.kind, "sm.dorc.File");
        assert_eq!(lm.marks[1].kind, MarkKind::Reads);
        assert_eq!(lm.marks[1].target.kind, "sm.dorc.Policy");
    }

    /// The meta verbs carry their non-coordinate token payloads (`281` §5): `safe-across`/`lends`
    /// a dimension, `stored-in` a substrate, `undivided-by-transit-across` an axis.
    #[test]
    fn meta_verb_payload_arities() {
        let lm = decode_line_marks(
            colon(),
            &words(&[
                "safe-across",
                "user",
                "lends",
                "fs-view",
                "stored-in",
                "net-kernel",
                "undivided-by-transit-across",
                "fs-view",
            ]),
        );
        let kinds: Vec<MarkKind> = lm.marks.iter().map(|m| m.kind).collect();
        assert_eq!(
            kinds,
            vec![
                MarkKind::SafeAcross,
                MarkKind::Lends,
                MarkKind::StoredIn,
                MarkKind::Undivided,
            ]
        );
        assert_eq!(lm.marks[0].target.kind, "user");
        assert_eq!(lm.marks[2].target.kind, "net-kernel");
    }

    /// A period-free non-verb token is `mark-unknown-verb` (`281` §4 rule-3 miss); the block ⊤s.
    #[test]
    fn unknown_verb_is_diagnosed() {
        let lm = decode_line_marks(colon(), &words(&["frobnicate", "sm.dorc.X"]));
        assert_eq!(slugs(&lm.diags), vec!["mark-unknown-verb"]);
    }

    /// The `@` selector splits the coordinate (`281` §R4), including the entity-less transitional
    /// `KIND:@SEL` and the no-entity `KIND@SEL` (`281` §6).
    #[test]
    fn at_selector_and_entity_less_forms() {
        let full = decode_line_marks(colon(), &words(&["sm.dorc.X:ent@sel"]));
        assert_eq!(full.marks[0].target.entity.as_deref(), Some("ent"));
        assert_eq!(full.marks[0].target.prop.as_deref(), Some("sel"));
        let no_ent = decode_line_marks(colon(), &words(&["sm.dorc.X@sel"]));
        assert_eq!(no_ent.marks[0].target.entity, None);
        assert_eq!(no_ent.marks[0].target.prop.as_deref(), Some("sel"));
        let empty_ent = decode_line_marks(colon(), &words(&["sm.dorc.X:@sel"]));
        assert_eq!(empty_ent.marks[0].target.entity.as_deref(), Some(""));
        assert_eq!(empty_ent.marks[0].target.prop.as_deref(), Some("sel"));
    }

    /// Brace-alternation both shapes (`281` §6): attached `@{a,b}` and the standalone payload word
    /// `verb {a,b}`. Refused on a verdict payload (the reused `mark-brace-verdict-single-cell`),
    /// accepted on `disturbs`/`safe-across`.
    #[test]
    fn brace_refused_on_verdict_allowed_on_emission() {
        let verdict = decode_line_marks(colon(), &words(&["sm.dorc.X@{a,b}"]));
        assert_eq!(
            slugs(&verdict.diags),
            vec!["mark-brace-verdict-single-cell"]
        );
        assert!(verdict.marks.is_empty());
        let disturbs = decode_line_marks(colon(), &words(&["disturbs", "sm.dorc.X@{a,b}"]));
        assert!(disturbs.diags.is_empty() && disturbs.marks.len() == 1);
        let dims = decode_line_marks(colon(), &words(&["safe-across", "{user,fs-view}"]));
        assert!(dims.diags.is_empty() && dims.marks[0].kind == MarkKind::SafeAcross);
        assert_eq!(dims.marks[0].target.kind, "{user,fs-view}");
    }

    /// rc-arity is over the WHOLE block, continuations included (`281` §7): two verdicts across a
    /// continuation line is `mark-rc-arity-exceeded`.
    #[test]
    fn rc_arity_over_block_including_continuations() {
        let p = parse_marks("dpkg -s nginx : sm.a.B@x\n: refutes sm.a.C@y\n");
        assert_eq!(all_slugs(&p), vec!["mark-rc-arity-exceeded"]);
    }

    /// A standalone block carrying a verdict/observe is `mark-standalone-rc-consumer` (nothing to
    /// back, `28A:rul-continuation-attachment`); a standalone `disturbs` block is fine.
    #[test]
    fn standalone_rc_consumer_is_diagnosed() {
        let bad = parse_marks(": sm.a.B@x\n");
        assert_eq!(all_slugs(&bad), vec!["mark-standalone-rc-consumer"]);
        let ok = parse_marks(": disturbs sm.dorc.File\n");
        assert!(all_slugs(&ok).is_empty());
        assert_eq!(marks_of(&ok.stmts[0])[0].kind, MarkKind::Disturbs);
    }

    /// Continuation attaches a mark-only line to the preceding marked line's block; a mark-only
    /// line after a PLAIN command stands alone (`28A:rul-continuation-attachment`).
    #[test]
    fn continuation_vs_standalone_attachment() {
        let cont =
            parse_marks("systemctl is-active nginx : sm.dorc.Service@active\n: safe-across user\n");
        assert!(
            all_slugs(&cont).is_empty(),
            "the second line continues the command's block"
        );
        assert!(matches!(cont.stmts[1].kind, StmtKind::Standalone { .. }));
        let standalone = parse_marks("plain_command\n: reads sm.a.B@x\n");
        assert_eq!(all_slugs(&standalone), vec!["mark-standalone-rc-consumer"]);
    }

    /// A lone `:` stays the POSIX null command (`28A:rul-marked-colon-is-the-grammars`):
    /// `while :; do …` survives, with no marks and no diagnostics.
    #[test]
    fn lone_colon_survives_as_null_command() {
        let p = parse_marks("while :; do echo hi; done\n");
        assert!(all_slugs(&p).is_empty());
        assert!(
            p.stmts
                .iter()
                .any(|s| matches!(s.kind, StmtKind::NullColon)),
            "the `:` loop condition is the null command, not a mark intro"
        );
        // A statement-leading `:` FOLLOWED by content is a mark intro, not the null command.
        let marked = parse_marks(": disturbs sm.dorc.File\n");
        assert!(matches!(marked.stmts[0].kind, StmtKind::Standalone { .. }));
    }

    /// The `#:` carrier (`281` §1/§9): a valid block parses as marks; a malformed one is
    /// `mark-hashcolon-malformed` (Warning) and left a comment; the version marker never collides.
    #[test]
    fn hashcolon_valid_invalid_and_marker_noncollision() {
        let valid = parse_marks("#: disturbs sm.dorc.File\n");
        assert!(slugs(&valid.stmts[0].diags).is_empty());
        assert!(matches!(
            &valid.stmts[0].kind,
            StmtKind::Standalone { intro, .. } if intro.carrier == Carrier::Hash
        ));
        let malformed = parse_marks("#: frobnicate\n");
        assert_eq!(all_slugs(&malformed), vec!["mark-hashcolon-malformed"]);
        assert!(
            matches!(malformed.stmts[0].kind, StmtKind::Plain),
            "left a comment"
        );
        let marker = parse_marks("# dorc-lang/v0.1\ncmd : sm.dorc.X@y\n");
        assert!(
            marker
                .stmts
                .iter()
                .all(|s| !matches!(s.kind, StmtKind::Standalone { .. })),
            "the space after `#` keeps the version marker a comment (no `#:` intro)"
        );
    }

    /// Inline bind is keyed on the `= value` tail (`28A:rul-bind-equals-tail-disambiguates`): a
    /// `name : KIND = value` statement binds; a multi-word command with a trailing coordinate is a
    /// whole-kind assert; a value-less `name : KIND` is NOT a bind (`28A:rul-singleton-bind-drops`).
    #[test]
    fn inline_bind_vs_whole_kind_assert() {
        let bind = parse_marks("pkg : sm.dorc.Package = \"$1\"\n");
        match &bind.stmts[0].kind {
            StmtKind::InlineBind { name, kind, span } => {
                assert_eq!(name, "pkg");
                assert_eq!(kind, "sm.dorc.Package");
                assert!(span.hi.0 > span.lo.0);
            }
            other => panic!("expected an inline bind, got {other:?}"),
        }
        let assert_mark = parse_marks("dpkg-query -W \"$pkg\" : sm.dorc.Package:pkg@installed\n");
        assert_eq!(marks_of(&assert_mark.stmts[0])[0].kind, MarkKind::Asserts);
        let no_tail = parse_marks("pkg : sm.dorc.Package\n");
        assert!(
            !matches!(no_tail.stmts[0].kind, StmtKind::InlineBind { .. }),
            "no `= value` tail ⇒ the `:` is a mark intro, never a bind"
        );
    }

    /// The trailing bind rides an assignment (`281` §8): `:= KIND` (sugar) and `: bind KIND`
    /// (word), each carrying the kind whose entity is the assigned value.
    #[test]
    fn trailing_bind_forms() {
        let sugar = parse_marks("FOO=\"bar\" := sm.dorc.Package\n");
        match &sugar.stmts[0].kind {
            StmtKind::TrailingBind { bind, intro } => {
                assert_eq!(bind.kind, "sm.dorc.Package");
                assert_eq!(intro.sugar, Some(Sugar::Equals));
            }
            other => panic!("expected a trailing bind, got {other:?}"),
        }
        let word = parse_marks("FOO=\"bar\" : bind sm.dorc.Package\n");
        assert!(matches!(
            &word.stmts[0].kind,
            StmtKind::TrailingBind { bind, .. } if bind.kind == "sm.dorc.Package"
        ));
    }
}
