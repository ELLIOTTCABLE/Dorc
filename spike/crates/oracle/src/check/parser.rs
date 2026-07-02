//! The dialect parser — recursive descent over [`lexer`](super::lexer) tokens.
//!
//! The grammar IS the contract (`adj-dialect-parser`, note 203 §4). Anything
//! outside it is a per-function lift failure: a [`Diagnostic`], never a panic
//! (`inv-no-throw`), and the file's other checks still lift (fail-soft). The
//! top-level entry [`lift_checks`] scans an oracle source for `<name>__check`
//! function definitions and parses each body; non-check top-level items are
//! ignored (a real oracle file also carries `oracle_kind=`/`oracle_effect`, which
//! this module does not own — that is the existing [`crate::lift`]).

use super::ast::{
    Annotation, CaseArm, Check, CheckSet, Command, Mark, MarkKind, MarkTarget, Pattern, Stmt, Test,
    TestOp, Word,
};
use super::lexer::{Tok, Token, lex};
use super::{VERB_BINDING, lift_failure, map_provider_name};
use dorc_core::{Carrier, Interner, Span, Symbol};
use dorc_syntax::sem;

/// The legacy provider-name suffix marking a command-keyed check (`apt_get__check`);
/// the provider before it maps `_` → `-` ([`map_provider_name`]).
const CHECK_SUFFIX: &str = "__check";

/// The period-form check suffix (233 / 23D §1: the opt-in oracle semaphore
/// `<provider>.check()`). The provider before it is the LITERAL command word (it may
/// carry `-`, e.g. `apt-get.check`), so no `_`→`-` mapping is applied. The strip (R1c)
/// rewrites `<provider>.check` to the legacy `<mangled>__check` form the engine keys on
/// (jc-strip-funcname: 23D/R1a write it `name_check`, but the engine's `CHECK_SUFFIX`
/// and the flagship golden use `__check`; the `__check` convention is authoritative).
const PERIOD_CHECK_SUFFIX: &str = ".check";

/// Lift every `<provider>__check` function in `src` into a [`CheckSet`], interning
/// provider/local names through `interner`. Fail-soft (`inv-no-throw`): a body that
/// is out of dialect yields a diagnostic and contributes no [`Check`]; the rest of
/// the file still lifts. Deterministic (`inv-determinism`): functions are processed
/// in source order and the result is `BTreeMap`-backed.
///
/// # Provider-name rule (underscore↔hyphen)
///
/// The name before `__check` maps `_` → `-` to recover the command word
/// (`apt_get__check` ⇒ `apt-get`, `command__check` ⇒ `command`). This is a
/// **lossy** mapping (a real `_` in a command name cannot be expressed) — flagged
/// as a `tc-*`-shaped cross-cutting decision; chosen conservatively here (sh
/// function names cannot contain `-`, so the mapping is the only way to name a
/// hyphenated command, and hyphenated commands vastly outnumber underscored ones).
/// A future wiring task may revisit; see this module's tests and the build report.
#[must_use]
pub fn lift_checks(interner: &mut Interner, src: &str) -> Carrier<CheckSet> {
    let tokens = lex(src);
    let mut p = Parser {
        toks: &tokens,
        pos: 0,
        interner,
        out: Carrier::pure(CheckSet::default()),
        last_term: None,
    };
    p.parse_file();
    p.out
}

struct Parser<'a> {
    toks: &'a [Token],
    pos: usize,
    interner: &'a mut Interner,
    out: Carrier<CheckSet>,
    /// Which terminator [`Parser::parse_block`] last consumed. Read by
    /// [`Parser::parse_if`] to tell an `else` branch from a bare `fi`.
    last_term: Option<BlockTerm>,
}

/// The concrete terminator a [`BlockEnd`] matched — needed because `else` and `fi`
/// (and `;;` vs `esac`) share one [`BlockEnd`] but drive different continuations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockTerm {
    Brace,
    Keyword(&'static str),
    DSemi,
    Else,
    Fi,
}

impl Parser<'_> {
    // --- token cursor -------------------------------------------------------

    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos).map(|t| &t.kind)
    }

    fn peek_span(&self) -> Option<Span> {
        self.toks.get(self.pos).map(|t| t.span)
    }

    /// A ZERO-WIDTH span at end-of-input: `[hi, hi)` where `hi` is the last real token's `hi`
    /// (or `BytePos(0)` for the empty-token degenerate case). The honest location for an
    /// EOF give-up (a truncated/chopped check body): the caret lands exactly where input ran
    /// out, pointing the UI at end-of-file (human ruling 22-q1). Distinct from [`ZERO_SPAN`]
    /// (byte 0): byte-0 would point at the file START, wrong for an EOF diagnostic.
    fn eof_span(&self) -> Span {
        let hi = self
            .toks
            .last()
            .map_or(dorc_core::BytePos(0), |t| t.span.hi);
        Span::new(hi, hi)
    }

    fn bump(&mut self) -> Option<&Token> {
        let t = self.toks.get(self.pos);
        if t.is_some() {
            self.pos = self.pos.saturating_add(1);
        }
        t
    }

    /// If the current token is a [`Tok::Word`], clone its lexeme + flag + span out
    /// (releasing the borrow on `self.toks`) and advance. Lets a caller then re-borrow
    /// `self.interner` for [`parse_word_lexeme`] without a borrow conflict.
    fn take_word(&mut self) -> Option<(String, bool, Span)> {
        let (lexeme, single_quoted, span) = match self.toks.get(self.pos) {
            Some(Token {
                kind:
                    Tok::Word {
                        lexeme,
                        single_quoted,
                    },
                span,
            }) => (lexeme.clone(), *single_quoted, *span),
            _ => return None,
        };
        self.pos = self.pos.saturating_add(1);
        Some((lexeme, single_quoted, span))
    }

    /// Skip newlines and bare `;` separators (statement boundaries).
    fn skip_separators(&mut self) {
        while matches!(self.peek(), Some(Tok::Newline | Tok::Semi)) {
            self.pos = self.pos.saturating_add(1);
        }
    }

    /// Skip only blank lines (newlines), not `;` — used where a `;` is meaningful.
    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Some(Tok::Newline)) {
            self.pos = self.pos.saturating_add(1);
        }
    }

    /// The decoded lexeme of the current token if it is a plain (non-single-quoted)
    /// word equal to `kw`. Used to match keywords (`while`, `do`, …), which are
    /// ordinary words to the lexer.
    fn at_keyword(&self, kw: &str) -> bool {
        matches!(self.peek(), Some(Tok::Word { lexeme, single_quoted })
            if !*single_quoted && lexeme == kw)
    }

    // --- file scan ----------------------------------------------------------

    /// Scan top-level items, parsing each `<name>__check() { … }` and ignoring all
    /// else. A malformed check body is diagnosed and skipped past its closing brace
    /// (best-effort resync) so a later check still parses.
    fn parse_file(&mut self) {
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        while self.pos < self.toks.len() {
            steps = steps.saturating_add(1);
            if steps > guard {
                break; // termination guard; bump() always advances on real input
            }
            self.skip_separators();
            if self.pos >= self.toks.len() {
                break;
            }
            if let Some(name_info) = self.at_check_funcdef() {
                self.parse_check_funcdef(name_info);
            } else {
                // Not a check definition — skip this one top-level item. We do not
                // diagnose (the file legitimately holds oracle_kind=/oracle_effect/
                // other functions); we just advance past it.
                self.skip_one_toplevel_item();
            }
        }
    }

    /// If the cursor is at a check-function header — the period form `<provider>.check (`
    /// (233 / 23D §1) or the legacy `<provider>__check (` — return the provider symbol +
    /// the name span. Does not consume. The period form's provider is the literal command
    /// word (may hold `-`); the legacy form's maps `_` → `-` ([`map_provider_name`]).
    fn at_check_funcdef(&mut self) -> Option<CheckHeader> {
        let Some(Tok::Word {
            lexeme,
            single_quoted,
        }) = self.peek()
        else {
            return None;
        };
        if *single_quoted {
            return None;
        }
        // Prefer the period form (the opt-in oracle semaphore); fall back to the legacy
        // mangled form. `apt-get.check` ⇒ provider `apt-get` (literal); `apt_get__check`
        // ⇒ provider `apt-get` (via `_`→`-`). Both recover the same command word.
        let provider_name = if let Some(p) = lexeme.strip_suffix(PERIOD_CHECK_SUFFIX) {
            if p.is_empty() {
                return None;
            }
            p.to_owned()
        } else if let Some(p) = lexeme.strip_suffix(CHECK_SUFFIX) {
            if p.is_empty() {
                return None;
            }
            map_provider_name(p)
        } else {
            return None;
        };
        // Must be followed by `(` `)` for a function definition.
        if !matches!(
            self.toks.get(self.pos.saturating_add(1)).map(|t| &t.kind),
            Some(Tok::LParen)
        ) {
            return None;
        }
        let name_span = self.peek_span()?;
        let provider = self.interner.intern(&provider_name);
        Some(CheckHeader {
            provider,
            name_span,
        })
    }

    /// Parse `<name>__check ( ) { BODY }`. On any out-of-dialect construct in the
    /// body, emit a diagnostic, drop the whole check, and resync past `}`.
    fn parse_check_funcdef(&mut self, header: CheckHeader) {
        self.bump(); // the name word
        // `(` `)`
        if !self.expect(&Tok::LParen) || !self.expect(&Tok::RParen) {
            self.fail(
                header.name_span,
                "malformed function header (expected `()`)",
            );
            self.resync_past_brace();
            return;
        }
        self.skip_newlines();
        if !self.expect(&Tok::LBrace) {
            self.fail(header.name_span, "function body must start with `{`");
            self.resync_past_brace();
            return;
        }
        match self.parse_block(BlockEnd::Brace) {
            Ok(body) => {
                let verb_sym = self.interner.intern(VERB_BINDING);
                // `parse_block(Brace)` consumed the closing `}`; its span ends the funcdef.
                let close_hi = self
                    .pos
                    .checked_sub(1)
                    .and_then(|i| self.toks.get(i))
                    .map_or(header.name_span.hi, |t| t.span.hi);
                let check = Check {
                    provider: header.provider,
                    name_span: header.name_span,
                    span: Span::new(header.name_span.lo, close_hi),
                    verb_sym,
                    body,
                };
                self.out.value.checks.insert(header.provider, check);
            }
            Err(diag_emitted) => {
                if !diag_emitted {
                    self.fail(header.name_span, "check body is out of dialect");
                }
                self.resync_past_brace();
            }
        }
    }

    // --- statement blocks ---------------------------------------------------

    /// Parse statements until the block terminator. Returns `Err(true)` if a
    /// diagnostic was already emitted for the failure, `Err(false)` if the caller
    /// should emit a generic one. The block terminator token is consumed on success.
    fn parse_block(&mut self, end: BlockEnd) -> Result<Vec<Stmt>, bool> {
        let mut stmts = Vec::new();
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        loop {
            steps = steps.saturating_add(1);
            if steps > guard {
                return Err(false); // termination guard
            }
            self.skip_separators();
            match self.peek() {
                None => return Err(true_with(self, end)), // ran off the end unterminated
                Some(tok) if end.matches(tok) => {
                    let term = end.term_of(tok);
                    self.last_term = Some(term);
                    // A case arm's terminating `esac` (the last arm omits `;;`) is
                    // left for the enclosing `parse_case` loop to consume; every
                    // other terminator is consumed here.
                    if !(matches!(end, BlockEnd::CaseArmEnd) && term == BlockTerm::Keyword("esac"))
                    {
                        self.bump();
                    }
                    return Ok(stmts);
                }
                Some(_) => {
                    let stmt = self.parse_stmt()?;
                    stmts.push(stmt);
                }
            }
        }
    }

    /// Parse one dialect statement. `Err` propagates an out-of-dialect failure.
    fn parse_stmt(&mut self) -> Result<Stmt, bool> {
        if self.at_keyword("while") {
            return self.parse_while();
        }
        if self.at_keyword("if") {
            return self.parse_if();
        }
        if self.at_keyword("case") {
            return self.parse_case();
        }
        if self.at_keyword("shift") {
            return self.parse_shift();
        }
        // Otherwise it is a word-led line: an assignment, an annotation, or a plain
        // command. Decide by looking at the word shape and what follows.
        self.parse_word_led()
    }

    fn parse_while(&mut self) -> Result<Stmt, bool> {
        self.bump(); // `while`
        let test = self.parse_bracket_test()?;
        self.skip_separators();
        if !self.eat_keyword("do") {
            return Err(self.fail_here("expected `do` after `while` test"));
        }
        let body = self.parse_block(BlockEnd::Keyword("done"))?;
        Ok(Stmt::While { test, body })
    }

    fn parse_if(&mut self) -> Result<Stmt, bool> {
        self.bump(); // `if`
        let test = self.parse_bracket_test()?;
        self.skip_separators();
        if !self.eat_keyword("then") {
            return Err(self.fail_here("expected `then` after `if` test"));
        }
        let then_body = self.parse_block(BlockEnd::IfThenEnd)?;
        // `parse_block` recorded which terminator it consumed (`else` vs `fi`).
        let else_body = if self.last_term == Some(BlockTerm::Else) {
            self.parse_block(BlockEnd::Keyword("fi"))?
        } else {
            Vec::new()
        };
        Ok(Stmt::If {
            test,
            then_body,
            else_body,
        })
    }

    fn parse_case(&mut self) -> Result<Stmt, bool> {
        self.bump(); // `case`
        let scrutinee = self.parse_word()?;
        self.skip_separators();
        if !self.eat_keyword("in") {
            return Err(self.fail_here("expected `in` after `case` scrutinee"));
        }
        let mut arms = Vec::new();
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        loop {
            steps = steps.saturating_add(1);
            if steps > guard {
                return Err(false);
            }
            self.skip_separators();
            if self.eat_keyword("esac") {
                break;
            }
            if self.peek().is_none() {
                return Err(self.fail_here("unterminated `case` (expected `esac`)"));
            }
            let arm = self.parse_case_arm()?;
            arms.push(arm);
        }
        Ok(Stmt::Case { scrutinee, arms })
    }

    /// Parse `[ ( ] PATTERN ( | PATTERN )* ) BODY ;;`. A leading `(` before the
    /// pattern list is optional sh syntax; we accept and ignore it.
    fn parse_case_arm(&mut self) -> Result<CaseArm, bool> {
        if matches!(self.peek(), Some(Tok::LParen)) {
            self.bump();
        }
        let mut patterns = Vec::new();
        loop {
            let pat = self.parse_pattern()?;
            patterns.push(pat);
            match self.peek() {
                Some(Tok::Pipe) => {
                    self.bump();
                }
                Some(Tok::RParen) => {
                    self.bump();
                    break;
                }
                _ => return Err(self.fail_here("expected `|` or `)` in case-arm pattern")),
            }
        }
        // The arm body runs until `;;` (arm end) or `esac` (last arm, no `;;`).
        let body = self.parse_block(BlockEnd::CaseArmEnd)?;
        Ok(CaseArm { patterns, body })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, bool> {
        match self.peek() {
            Some(Tok::Word {
                lexeme,
                single_quoted,
            }) => {
                let lexeme = lexeme.clone();
                let single_quoted = *single_quoted;
                self.bump();
                if !single_quoted && lexeme == "*" {
                    Ok(Pattern::Wildcard)
                } else if lexeme.contains(['*', '?', '[']) && !single_quoted {
                    // A non-trivial glob pattern is out of dialect — arm selection
                    // must be a concrete equality, never a pattern-match (kFAIL:
                    // bias to Top, so reject rather than under-model).
                    Err(self.fail_here("only literal and `*` case patterns are in dialect"))
                } else {
                    Ok(Pattern::Literal(lexeme))
                }
            }
            _ => Err(self.fail_here("expected a case-arm pattern")),
        }
    }

    fn parse_shift(&mut self) -> Result<Stmt, bool> {
        self.bump(); // `shift`
        // Optional numeric argument — a plain (non-single-quoted) word.
        let Some(Tok::Word {
            lexeme,
            single_quoted: false,
        }) = self.peek()
        else {
            return Ok(Stmt::Shift { count: None });
        };
        if let Ok(n) = lexeme.parse::<u32>() {
            self.bump();
            return Ok(Stmt::Shift { count: Some(n) });
        }
        // A word that actually begins the next statement ⇒ `shift` had no count.
        if is_statement_terminator_word(lexeme) {
            return Ok(Stmt::Shift { count: None });
        }
        // Anything else (`shift $x`, `shift foo`) is a dynamic/invalid count ⇒ out
        // of dialect (kFAIL: reject rather than under-model).
        Err(self.fail_here("`shift` count must be a literal integer"))
    }

    /// Parse a word-led line: an annotation (`name : kind = value`), an assignment
    /// (`name=value`), or a plain command (`dpkg-query -W "$pkg"`).
    fn parse_word_led(&mut self) -> Result<Stmt, bool> {
        // Peek the first word's raw lexeme to classify.
        let Some(Tok::Word {
            lexeme,
            single_quoted,
        }) = self.peek()
        else {
            // A line that does not start with a word (e.g. a stray `]`, redirect,
            // or error token) is out of dialect.
            return Err(self.fail_here("statement does not start with a word"));
        };
        let first = lexeme.clone();
        let first_sq = *single_quoted;
        let start_span = self.peek_span().unwrap_or(ZERO_SPAN);

        // Bare inline-dialect mark in statement position (233 §1–§4, R1b): a leading
        // standalone `:` / `:?` marker word (`: kind`, `: kind:entity.prop~`,
        // `: provider:verb~`). The dialect uses a leading marker ONLY for a mark (probe
        // bodies never begin with `:`), so this is unambiguous.
        if let Some(observe) = (!first_sq).then(|| mark_marker(&first)).flatten() {
            return self.parse_bare_mark(observe, start_span);
        }

        // `name=value` assignment: an unquoted word of the form IDENT=REST (a bare
        // `name=` is degenerate; its value is the empty literal).
        if let Some((name, rest)) = (!first_sq).then(|| split_assignment(&first)).flatten() {
            self.bump();
            let value = parse_word_lexeme(rest, false, self.interner);
            return Ok(Stmt::Assign {
                name: self.interner.intern(name),
                value,
            });
        }

        // Identity annotation `name : kind [= value]`: first word is a plain ident, next
        // is the standalone word `:`, AND the kind word after it is a BARE kind (no inner
        // `:`). A `kind:entity.prop` after the `:` means this is a trailing ESTABLISH on
        // the single-word command `name` (not an identity annotation) — fall through to
        // `parse_command`, which re-sees the `:` marker and parses the trailing mark.
        if !first_sq
            && sem::is_name(&first)
            && self.next_word_is(":")
            && self.kind_after_colon_is_bare()
        {
            return self.parse_annotation(&first, start_span);
        }

        // Otherwise: a plain command (optionally with a trailing ESTABLISH/OBSERVE mark).
        // Consume words/redirects to the statement end or a `:`/`:?` marker.
        self.parse_command(start_span)
    }

    /// Peek: is the token at `pos+1` the standalone word `s`?
    fn next_word_is(&self, s: &str) -> bool {
        matches!(
            self.toks.get(self.pos.saturating_add(1)).map(|t| &t.kind),
            Some(Tok::Word { lexeme, single_quoted: false }) if lexeme == s
        )
    }

    /// Peek: is the word at `pos+2` (after a `name :`) a BARE kind — a plain word with no
    /// inner `:` (so it is an identity-annotation kind, not a `kind:entity.prop` mark
    /// target)? Absent/quoted/`:`-bearing ⇒ not bare.
    fn kind_after_colon_is_bare(&self) -> bool {
        matches!(
            self.toks.get(self.pos.saturating_add(2)).map(|t| &t.kind),
            Some(Tok::Word { lexeme, single_quoted: false }) if !lexeme.contains(':')
        )
    }

    /// Parse the inline annotation `name : kind = value` (the operand form) or
    /// `name : kind` (the **nullary/Singleton** form — a verb whose resource has no
    /// operand, e.g. `apt-get update`; 202 §2 / task-W §4). The caller verified the
    /// first word is `name` and the next is `:`.
    fn parse_annotation(&mut self, name: &str, start_span: Span) -> Result<Stmt, bool> {
        let name_sym = self.interner.intern(name);
        self.bump(); // name
        self.bump(); // `:`
        // kind: a single plain word (reverse-DNS string, or the file's short
        // oracle_kind — task-W keeps them identical so annotation-kind == effect-map kind).
        let Some((kind, false, kind_span)) = self.take_word() else {
            return Err(self.fail_here("annotation kind must be a single literal word"));
        };
        // The `= value` tail is OPTIONAL. Present ⇒ the ordinary operand annotation.
        // Absent ⇒ the nullary/Singleton spelling (`value = None`): the evaluator
        // resolves a [`super::ast::AnnotatedValue::Singleton`] and the wiring keys the
        // cell on [`dorc_core::EntityRef::Singleton`]. A value-less annotation is the
        // EXPLICIT opt-in — a wholly *missing* annotation still degrades to
        // `Top(MissingAnnotation)` (the safe direction), so no accidental Singleton.
        if !matches!(self.peek(), Some(Tok::Word { lexeme, .. }) if lexeme == "=") {
            return Ok(Stmt::Annotation(Annotation {
                name: name_sym,
                kind,
                value: None,
                span: start_span.to(kind_span),
                name_span: start_span,
                value_span: None,
            }));
        }
        self.bump(); // `=`
        let Some((lexeme, single_quoted, val_span)) = self.take_word() else {
            return Err(self.fail_here("annotation requires a value word after `=`"));
        };
        let value = parse_word_lexeme(&lexeme, single_quoted, self.interner);
        Ok(Stmt::Annotation(Annotation {
            name: name_sym,
            kind,
            value: Some(value),
            span: start_span.to(val_span),
            name_span: start_span,
            value_span: Some(val_span),
        }))
    }

    /// Parse a plain command: a run of words and redirects up to a statement
    /// terminator (`;`, `;;`, newline, `}`, or a block keyword). Records the
    /// verbatim source span (`Command::span`) for shipping into the probe.
    fn parse_command(&mut self, start_span: Span) -> Result<Stmt, bool> {
        let mut words = Vec::new();
        let mut end_span = start_span;
        let mut mark_observe: Option<bool> = None;
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        loop {
            steps = steps.saturating_add(1);
            if steps > guard {
                return Err(false);
            }
            // Classify the current token without holding a borrow across the body.
            let class = match self.peek() {
                None | Some(Tok::Newline | Tok::Semi | Tok::DSemi | Tok::RBrace) => CmdTok::End,
                // An unquoted word: a block-ending keyword (`done`/`fi`/…) ends the command
                // without being consumed; a standalone `:`/`:?` marker begins a TRAILING
                // effect mark (233 ESTABLISH/OBSERVE); anything else is a command word.
                Some(Tok::Word {
                    lexeme,
                    single_quoted: false,
                }) => {
                    if is_block_keyword(lexeme) {
                        CmdTok::End
                    } else if let Some(observe) = mark_marker(lexeme) {
                        CmdTok::MarkStart(observe)
                    } else {
                        CmdTok::Word
                    }
                }
                // A single-quoted word is always a plain command word (`':'` is a literal).
                Some(Tok::Word { .. }) => CmdTok::Word,
                Some(Tok::Redirect(_)) => CmdTok::Redirect,
                Some(Tok::Error(msg)) => CmdTok::Error(msg.clone()),
                // Any other metacharacter (`(`, `|`, brackets) inside a command is
                // out of dialect (we do not model pipelines/subshells in probe
                // bodies for this round).
                Some(_) => CmdTok::Other,
            };
            match class {
                CmdTok::End => break,
                CmdTok::MarkStart(observe) => {
                    mark_observe = Some(observe);
                    break;
                }
                CmdTok::Word => {
                    end_span = self.peek_span().unwrap_or(end_span);
                    if let Some((lexeme, single_quoted, _)) = self.take_word() {
                        words.push(parse_word_lexeme(&lexeme, single_quoted, self.interner));
                    }
                }
                CmdTok::Redirect => {
                    end_span = self.peek_span().unwrap_or(end_span);
                    self.bump();
                }
                CmdTok::Error(msg) => {
                    return Err(self.fail_here(&format!("out-of-dialect token in command: {msg}")));
                }
                CmdTok::Other => return Err(self.fail_here("unexpected token in command")),
            }
        }
        if words.is_empty() {
            return Err(self.fail_here("empty command"));
        }
        // The command span ends at its last real word/redirect (EXCLUDING the trailing
        // mark), so the strip deletes exactly `[span.hi .. mark.span.hi]`.
        let span = start_span.to(end_span);
        let mark = match mark_observe {
            Some(observe) => Some(self.parse_mark(observe, false)?),
            None => None,
        };
        Ok(Stmt::Command(Command { words, span, mark }))
    }

    /// Parse a bare inline-dialect mark statement (`: TARGET` / `:? TARGET`): a POISON
    /// no-op mention, an ACK vouch, or the CONVERGED-VOUCH placeholder (233 §1–§4). The
    /// caller verified the first token is a `:`/`:?` marker; `observe` is true for `:?`.
    fn parse_bare_mark(&mut self, observe: bool, _start_span: Span) -> Result<Stmt, bool> {
        Ok(Stmt::Mark(self.parse_mark(observe, true)?))
    }

    /// Parse a dialect mark starting at the `:`/`:?` marker token: consume the marker,
    /// the target word, and an optional `= value` tail; split the target and classify by
    /// (`observe`, `bare`, suffix). `bare` ⇒ a statement-position mark (POISON/ACK/vouch);
    /// `!bare` ⇒ a trailing mark on a command (ESTABLISH/OBSERVE). Malformed ⇒ ⊤-reject
    /// (the parser's standing bias — never guess).
    fn parse_mark(&mut self, observe: bool, bare: bool) -> Result<Mark, bool> {
        let marker_span = self.peek_span().unwrap_or(ZERO_SPAN);
        self.bump(); // the `:` / `:?` marker word
        let Some((lexeme, _sq, target_span)) = self.take_word() else {
            return Err(self.fail_here("dialect mark requires a `kind:entity.prop` target"));
        };
        // Optional `= value` tail (233 §1: an ESTABLISH explicit-value assignment).
        let mut end_span = target_span;
        let value = if matches!(self.peek(), Some(Tok::Word { lexeme, .. }) if lexeme == "=") {
            self.bump(); // `=`
            let Some((v, vsq, vspan)) = self.take_word() else {
                return Err(self.fail_here("dialect mark `=` requires a value word"));
            };
            end_span = vspan;
            Some(parse_word_lexeme(&v, vsq, self.interner))
        } else {
            None
        };

        let Some(parsed) = split_mark_target(&lexeme) else {
            return Err(
                self.fail_here("malformed dialect mark target (expected `kind:entity.prop`)")
            );
        };
        let kind = match classify_mark(observe, bare, &parsed) {
            Ok(k) => k,
            Err(reason) => return Err(self.fail_here(reason)),
        };
        Ok(Mark {
            kind,
            target: MarkTarget {
                kind: parsed.kind,
                entity: parsed.entity,
                prop: parsed.prop,
                value,
            },
            span: marker_span.to(end_span),
        })
    }

    // --- words & tests ------------------------------------------------------

    /// Parse a single word token into a [`Word`].
    fn parse_word(&mut self) -> Result<Word, bool> {
        match self.take_word() {
            Some((lexeme, single_quoted, _span)) => {
                Ok(parse_word_lexeme(&lexeme, single_quoted, self.interner))
            }
            None => Err(self.fail_here("expected a word")),
        }
    }

    /// Parse a `[ LHS OP RHS ]` test. The dialect admits only `=`/`!=` string
    /// comparisons (the flag-strip idiom). The brackets are standalone tokens.
    fn parse_bracket_test(&mut self) -> Result<Test, bool> {
        let lo = self.peek_span().unwrap_or(ZERO_SPAN);
        if !self.expect(&Tok::LBracket) {
            return Err(self.fail_here("expected `[` to open a test"));
        }
        let lhs = self.parse_word()?;
        let op = match self.peek() {
            Some(Tok::Word {
                lexeme,
                single_quoted: false,
            }) if lexeme == "=" => TestOp::Eq,
            Some(Tok::Word {
                lexeme,
                single_quoted: false,
            }) if lexeme == "!=" => TestOp::Ne,
            _ => {
                return Err(self.fail_here("test operator must be `=` or `!=` (string comparison)"));
            }
        };
        self.bump();
        let rhs = self.parse_word()?;
        let hi = self.peek_span().unwrap_or(lo);
        if !self.expect(&Tok::RBracket) {
            return Err(self.fail_here("expected `]` to close a test"));
        }
        Ok(Test {
            lhs,
            op,
            rhs,
            span: lo.to(hi),
        })
    }

    // --- helpers ------------------------------------------------------------

    /// Consume the current token iff it equals `want`; else leave it and return
    /// false.
    fn expect(&mut self, want: &Tok) -> bool {
        if self.peek() == Some(want) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Consume the current token iff it is the plain word keyword `kw`.
    fn eat_keyword(&mut self, kw: &str) -> bool {
        if self.at_keyword(kw) {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Emit an out-of-dialect diagnostic pointing at the current token (or, at end-of-input,
    /// a synthesized EOF span; human ruling 22-q1) and return `true` (the "diagnostic already
    /// emitted" signal for `parse_block`).
    fn fail_here(&mut self, msg: &str) -> bool {
        let span = self.peek_span().unwrap_or_else(|| self.eof_span());
        let diag = lift_failure(false, span, msg.to_owned(), self.interner);
        self.out.push(diag);
        true
    }

    /// Emit an out-of-dialect diagnostic at a specific span.
    fn fail(&mut self, span: Span, msg: &str) {
        let diag = lift_failure(false, span, msg.to_owned(), self.interner);
        self.out.push(diag);
    }

    /// Skip one top-level non-check item: advance to the next statement boundary,
    /// and if it is a `name() { … }` function, skip its whole body.
    fn skip_one_toplevel_item(&mut self) {
        // Detect `word (` → a funcdef; skip to matching brace.
        let is_funcdef = matches!(self.peek(), Some(Tok::Word { .. }))
            && matches!(
                self.toks.get(self.pos.saturating_add(1)).map(|t| &t.kind),
                Some(Tok::LParen)
            );
        if is_funcdef {
            self.resync_past_brace();
            return;
        }
        // Otherwise skip to the next newline/`;`.
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        while let Some(tok) = self.peek() {
            steps = steps.saturating_add(1);
            if steps > guard {
                break;
            }
            if matches!(tok, Tok::Newline | Tok::Semi) {
                self.bump();
                break;
            }
            self.bump();
        }
    }

    /// Resync after a malformed check: scan forward to the first `}` brace token at
    /// any depth-0 (we do not track nesting precisely; a check body's only braces
    /// are its own delimiters in the dialect, so the next `}` is the body close).
    /// Consumes through it.
    fn resync_past_brace(&mut self) {
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        while let Some(tok) = self.peek() {
            steps = steps.saturating_add(1);
            if steps > guard {
                break;
            }
            let is_brace = matches!(tok, Tok::RBrace);
            self.bump();
            if is_brace {
                break;
            }
        }
    }
}

/// Header info for a recognized `<name>__check` function.
#[derive(Clone, Copy)]
struct CheckHeader {
    provider: Symbol,
    name_span: Span,
}

/// Classification of the current token inside [`Parser::parse_command`], computed
/// while borrowing `self.toks`, then matched after the borrow is released.
enum CmdTok {
    /// A statement terminator / block keyword — ends the command (not consumed).
    End,
    /// A standalone `:`/`:?` marker begins a trailing effect mark — ends the command
    /// (not consumed); the bool is the observe flag (`:?` ⇒ true).
    MarkStart(bool),
    /// A plain word to add to the command.
    Word,
    /// A redirection chunk to fold into the verbatim span.
    Redirect,
    /// An out-of-dialect token (carries the lexer's message).
    Error(String),
    /// Any other unexpected metacharacter ⇒ out of dialect.
    Other,
}

/// What ends a statement block.
#[derive(Clone, Copy)]
enum BlockEnd {
    /// `}` (function body).
    Brace,
    /// A keyword word (`done`, `fi`, `esac`).
    Keyword(&'static str),
    /// `;;` or `esac` (a case arm — the last arm omits `;;`).
    CaseArmEnd,
    /// `else` or `fi` (an if's then-branch). On `else`, the parser records
    /// `just_consumed_else` so [`Parser::parse_if`] knows to parse an else-branch.
    IfThenEnd,
}

impl BlockEnd {
    fn matches(self, tok: &Tok) -> bool {
        match self {
            BlockEnd::Brace => matches!(tok, Tok::RBrace),
            BlockEnd::Keyword(kw) => {
                matches!(tok, Tok::Word { lexeme, single_quoted: false } if lexeme == kw)
            }
            BlockEnd::CaseArmEnd => {
                matches!(tok, Tok::DSemi)
                    || matches!(tok, Tok::Word { lexeme, single_quoted: false } if lexeme == "esac")
            }
            BlockEnd::IfThenEnd => {
                matches!(tok, Tok::Word { lexeme, single_quoted: false }
                    if lexeme == "else" || lexeme == "fi")
            }
        }
    }

    /// The concrete [`BlockTerm`] for the token this `BlockEnd` matched. Caller has
    /// already checked [`matches`](Self::matches), so the token is one this arm
    /// recognizes.
    fn term_of(self, tok: &Tok) -> BlockTerm {
        match self {
            BlockEnd::Brace => BlockTerm::Brace,
            BlockEnd::Keyword(kw) => BlockTerm::Keyword(kw),
            BlockEnd::CaseArmEnd => {
                if matches!(tok, Tok::DSemi) {
                    BlockTerm::DSemi
                } else {
                    BlockTerm::Keyword("esac")
                }
            }
            BlockEnd::IfThenEnd => {
                if matches!(tok, Tok::Word { lexeme, .. } if lexeme == "else") {
                    BlockTerm::Else
                } else {
                    BlockTerm::Fi
                }
            }
        }
    }
}

/// `parse_block` needs to communicate, for the if-then case, *which* terminator it
/// hit (`else` vs `fi`) and, for case arms, not to consume `esac`. This is handled
/// with a small bit of parser state set just before returning. We thread it via a
/// field; this free fn computes the unterminated-error code.
fn true_with(p: &mut Parser<'_>, end: BlockEnd) -> bool {
    // Always a real span: a `parse_block` that ran off the end is at EOF, so synthesize an
    // end-of-input span there (human ruling 22-q1 — point the UI at end-of-file).
    let span = p.peek_span().unwrap_or_else(|| p.eof_span());
    let msg = match end {
        BlockEnd::Brace => "unterminated function body (expected `}`)",
        BlockEnd::Keyword(kw) => {
            return {
                let diag = lift_failure(
                    true,
                    span,
                    format!("unterminated block (expected `{kw}`)"),
                    p.interner,
                );
                p.out.push(diag);
                true
            };
        }
        BlockEnd::CaseArmEnd => "unterminated case arm (expected `;;` or `esac`)",
        BlockEnd::IfThenEnd => "unterminated `if` (expected `else`/`fi`)",
    };
    let diag = lift_failure(true, span, msg.to_owned(), p.interner);
    p.out.push(diag);
    true
}

const ZERO_SPAN: Span = Span {
    lo: dorc_core::BytePos(0),
    hi: dorc_core::BytePos(0),
};

// === word-lexeme decoding ===================================================

/// Decode a lexer word lexeme into a [`Word`]. `single_quoted` ⇒ the whole token
/// was single-quoted, so `$`/`#` are literal (`'$1'` ⇒ the literal string `$1`).
fn parse_word_lexeme(lexeme: &str, single_quoted: bool, interner: &mut Interner) -> Word {
    if single_quoted {
        return Word::SingleQuotedLiteral(lexeme.to_owned());
    }
    // `${N#PREFIX}` — positional with a leading LITERAL prefix stripped. dash treats the
    // prefix as a GLOB pattern (fnmatch, shortest match), and `${N##…}` as longest-match;
    // our evaluator does a literal strip only. The is-modelable predicate is shared
    // (`sem::parse_prefix_strip`): a globby prefix (`${1#*=}`) or the `##` form yields
    // `None` ⇒ we fall through to the unmodeled path ⇒ the evaluator fails to resolve ⇒
    // Top — symmetric with `parse_pattern`'s glob rejection, the safe direction. Misreading
    // a glob form as a literal strip was round-20 crosscheck finding 2 (a wrong concrete).
    if let Some(inner) = lexeme.strip_prefix("${").and_then(|s| s.strip_suffix('}')) {
        if let Some(strip) = sem::parse_prefix_strip(inner) {
            return Word::PositionalStripPrefix {
                n: strip.n,
                prefix: strip.prefix.to_owned(),
            };
        }
        // `${name}` (no `#`) — a braced variable reference. NB the dialect keys ONLY a
        // plain name here; a braced multi-digit positional `${12}` is intentionally left
        // `Unmodeled` (it never appears in the §2 idioms, and modeling it would be a
        // silent behaviour widening — 20D divergence dv-2).
        if sem::is_name(inner) {
            return Word::Var(interner.intern(inner));
        }
        // Any other `${…}` parameter expansion is not modeled ⇒ `Unmodeled`, which
        // fails to resolve in EVERY position (annotation value AND `[ ]` test) ⇒ the
        // check degrades to Top — the safe direction. (NOT `Literal`: a literal would
        // *evaluate as its own text* in test-position — a wrong concrete.)
        return Word::Unmodeled(lexeme.to_owned());
    }
    // `$N` — positional, or `$name` — variable.
    if let Some(rest) = lexeme.strip_prefix('$') {
        if let Ok(n) = rest.parse::<u32>() {
            return Word::Positional(n);
        }
        if sem::is_name(rest) {
            return Word::Var(interner.intern(rest));
        }
        // `$@`, `$*`, `$#`, `$?` and the like: not modeled as a single resolvable
        // value here. Keep literal ⇒ evaluator yields Top if it reaches a
        // value-position. (`$@` re-expansion is a deferred precision item, 202 §1.)
        return Word::Literal(lexeme.to_owned());
    }
    // A bare literal. If a `$` appears mid-word (`pre$1`), we conservatively keep
    // the whole thing literal — the dialect's resolvable words are simple `$N`/
    // `$name`/`"$N"`, and a mixed word degrades to a non-matching literal ⇒ Top.
    Word::Literal(lexeme.to_owned())
}

/// Is `lexeme` a standalone inline-dialect mark marker? `:` ⇒ `Some(false)` (ESTABLISH /
/// POISON / ACK / vouch), `:?` ⇒ `Some(true)` (OBSERVE), anything else ⇒ `None`. The
/// space-flanked marker lexes as its own word (the target `kind:entity.prop` is a
/// separate, adjacent word).
fn mark_marker(lexeme: &str) -> Option<bool> {
    match lexeme {
        ":" => Some(false),
        ":?" => Some(true),
        _ => None,
    }
}

/// A syntactically-split mark target — every fragment OPAQUE (`inv-referent-agnostic`,
/// never decoded). See [`split_mark_target`].
struct ParsedTarget {
    kind: String,
    entity: Option<String>,
    prop: Option<String>,
    /// A trailing `~` (vouch/ack) or `!` (invert), if present.
    suffix: Option<char>,
}

/// Split a mark target lexeme `kind[:entity[.prop]][~|!]` into its opaque fragments
/// (R1b, "parse mediocre"): strip a single trailing `~`/`!`; split the body on the FIRST
/// `:` (kind ⟂ rest) and the rest on the LAST `.` (entity ⟂ prop). `None` if empty or
/// kind-less (⊤-reject upstream). Kind fragments keep any inner `.` (reverse-DNS stays
/// intact); no fragment is interpreted.
fn split_mark_target(lexeme: &str) -> Option<ParsedTarget> {
    if lexeme.is_empty() {
        return None;
    }
    let (body, suffix) = if let Some(b) = lexeme.strip_suffix('~') {
        (b, Some('~'))
    } else if let Some(b) = lexeme.strip_suffix('!') {
        (b, Some('!'))
    } else {
        (lexeme, None)
    };
    if body.is_empty() {
        return None;
    }
    let (kind, rest) = match body.split_once(':') {
        Some((k, r)) => (k.to_owned(), Some(r)),
        None => (body.to_owned(), None),
    };
    if kind.is_empty() {
        return None;
    }
    let (entity, prop) = match rest {
        None | Some("") => (None, None),
        Some(r) => match r.rsplit_once('.') {
            Some((e, p)) if !p.is_empty() && !e.is_empty() => {
                (Some(e.to_owned()), Some(p.to_owned()))
            }
            _ => (Some(r.to_owned()), None),
        },
    };
    Some(ParsedTarget {
        kind,
        entity,
        prop,
        suffix,
    })
}

/// Classify a parsed mark into a [`MarkKind`] from (`observe`, `bare`, suffix, entity?,
/// prop?). Malformed combinations ⊤-reject (`Err(reason)`) — the parser's standing bias
/// (never guess). `bare` ⇒ statement-position (POISON/ACK/vouch); `!bare` ⇒ a trailing
/// command mark (ESTABLISH/OBSERVE). The ACK-vs-vouch split is the dot-presence heuristic
/// (jc-vouch-vs-ack): `kind:entity.prop~` ⇒ ACK, two-level `provider:verb~` ⇒ vouch.
fn classify_mark(
    observe: bool,
    bare: bool,
    target: &ParsedTarget,
) -> Result<MarkKind, &'static str> {
    if observe {
        // `:?` OBSERVE (233 depends-upon). A `~`/`!` suffix on it is malformed.
        return if target.suffix.is_some() {
            Err("`:?` observe mark takes no `~`/`!` suffix")
        } else {
            Ok(MarkKind::Observe)
        };
    }
    match target.suffix {
        Some('!') => {
            if bare {
                Err("`!` (invert) is a trailing ESTABLISH suffix, not a bare mark")
            } else {
                Ok(MarkKind::EstablishInverted)
            }
        }
        Some('~') => {
            if !bare {
                Err("`~` (vouch/ack) is a bare statement mark, not a trailing command mark")
            } else if target.prop.is_some() {
                Ok(MarkKind::Ack)
            } else if target.entity.is_some() {
                Ok(MarkKind::ConvergedVouch)
            } else {
                Ok(MarkKind::Ack)
            }
        }
        Some(_) => Err("unrecognized mark suffix"),
        None => {
            if bare {
                Ok(MarkKind::Poison)
            } else {
                Ok(MarkKind::Establish)
            }
        }
    }
}

/// Split `name=value` if `name` is a valid POSIX name (`sem::is_name`) and the lexeme
/// contains `=` at the boundary. Returns `(name, value)`. A bare `name=` yields
/// `("name", "")`.
fn split_assignment(lexeme: &str) -> Option<(&str, &str)> {
    let (name, value) = lexeme.split_once('=')?;
    if name.is_empty() || !sem::is_name(name) {
        return None;
    }
    Some((name, value))
}

/// `case`/`while`/`if` block keywords that end a plain command when they appear in
/// command position.
fn is_block_keyword(s: &str) -> bool {
    matches!(s, "do" | "done" | "then" | "else" | "fi" | "esac" | "in")
}

/// A word that, if it appeared where a `shift` count is expected, actually starts
/// the next statement (so `shift` had no count). Conservative: only the block
/// keywords.
fn is_statement_terminator_word(s: &str) -> bool {
    is_block_keyword(s)
}

#[cfg(test)]
mod dialect_tests {
    //! R1a/R1b: the period-form funcdef + the 233 inline-mark dialect. These tests
    //! reach the internal AST (Marks aren't re-exported), so they live here rather than
    //! in `tests/check.rs`. Every ambiguity ⊤-rejects (`inv-top-reject` bias); a lift
    //! failure is a diagnostic, never a panic (`inv-no-throw`).
    use super::{Interner, Mark, MarkKind, Stmt, lift_checks};

    /// Lift `src`, assert exactly one check, and return its body statements.
    fn body_of(src: &str) -> Vec<Stmt> {
        let mut i = Interner::default();
        let out = lift_checks(&mut i, src);
        assert!(
            out.diags.is_empty(),
            "expected a clean lift, got diags: {:?}",
            out.diags
        );
        assert_eq!(out.value.len(), 1, "expected exactly one check");
        let provider = out.value.providers().next().expect("one provider");
        out.value.get(provider).expect("the check").body.clone()
    }

    /// Find the first trailing [`Mark`] on any command in `body`.
    fn first_command_mark(body: &[Stmt]) -> Option<Mark> {
        fn walk(stmts: &[Stmt]) -> Option<Mark> {
            for s in stmts {
                match s {
                    Stmt::Command(c) => {
                        if let Some(m) = &c.mark {
                            return Some(m.clone());
                        }
                    }
                    Stmt::Case { arms, .. } => {
                        for a in arms {
                            if let Some(m) = walk(&a.body) {
                                return Some(m);
                            }
                        }
                    }
                    Stmt::If {
                        then_body,
                        else_body,
                        ..
                    } => {
                        if let Some(m) = walk(then_body).or_else(|| walk(else_body)) {
                            return Some(m);
                        }
                    }
                    Stmt::While { body, .. } => {
                        if let Some(m) = walk(body) {
                            return Some(m);
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        walk(body)
    }

    #[test]
    fn period_form_funcdef_recovers_the_hyphenated_provider() {
        // `apt-get.check()` (the 23D §1 opt-in semaphore) resolves to provider `apt-get`
        // — the literal command word, no `_`→`-` mapping (that is the legacy form's rule).
        let mut i = Interner::default();
        let out = lift_checks(
            &mut i,
            "apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; }",
        );
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        let apt = i.intern("apt-get");
        assert!(
            out.value.get(apt).is_some(),
            "the period form keys the check on `apt-get`"
        );
    }

    #[test]
    fn period_and_legacy_forms_agree_on_provider() {
        // `systemctl.check` and `systemctl__check` name the SAME provider (no hyphen, so
        // the two spellings coincide) — the strip's job is to turn the former into the
        // latter, so they must lift identically.
        let mut i = Interner::default();
        let period = lift_checks(
            &mut i,
            "systemctl.check() { svc : service = \"$1\"; systemctl is-active -- \"$svc\"; }",
        );
        let legacy = lift_checks(
            &mut i,
            "systemctl__check() { svc : service = \"$1\"; systemctl is-active -- \"$svc\"; }",
        );
        let sc = i.intern("systemctl");
        assert!(period.value.get(sc).is_some() && legacy.value.get(sc).is_some());
    }

    #[test]
    fn trailing_establish_mark_splits_kind_and_selector() {
        // `dpkg-query … : package:"$pkg".installed` — a trailing ESTABLISH. kind/selector
        // are split opaquely; the entity fragment (`$pkg`) is carried but not decoded.
        let body = body_of(
            "apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\" : package:\"$pkg\".installed; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Establish);
        assert_eq!(m.target.kind, "package");
        assert_eq!(m.target.prop.as_deref(), Some("installed"));
        assert_eq!(m.target.entity.as_deref(), Some("$pkg"));
    }

    #[test]
    fn trailing_bang_is_establish_inverted() {
        // `… : package:"$pkg".installed!` — the `!` pun (233 §1): the verb makes the fact
        // NOT hold ⇒ the lift reads Kill polarity.
        let body = body_of(
            "apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\" : package:\"$pkg\".installed!; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::EstablishInverted);
        assert_eq!(m.target.prop.as_deref(), Some("installed"));
    }

    #[test]
    fn trailing_query_marker_is_observe() {
        // `command -v … :? tool:"$tool".present` — OBSERVE (read-only depends-upon).
        let body = body_of(
            "command.check() { tool : tool = \"$1\"; command -v -- \"$tool\" :? tool:\"$tool\".present; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Observe);
        assert_eq!(m.target.kind, "tool");
        assert_eq!(m.target.prop.as_deref(), Some("present"));
    }

    #[test]
    fn bare_poison_mention_parses() {
        // `: fs.Path` — a bare no-op POISON mention; kind-only (the `.` stays in the
        // opaque kind), no entity/prop.
        let body = body_of(
            "apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; : fs.Path; }",
        );
        let Some(Stmt::Mark(m)) = body.iter().find(|s| matches!(s, Stmt::Mark(_))) else {
            panic!("expected a bare Mark statement: {body:?}");
        };
        assert_eq!(m.kind, MarkKind::Poison);
        assert_eq!(m.target.kind, "fs.Path");
        assert_eq!(m.target.entity, None);
    }

    #[test]
    fn bare_three_level_tilde_is_ack() {
        // `: package:"$pkg".held~` — a three-level considered-untouched ACK.
        let body = body_of(
            "apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; : package:\"$pkg\".held~; }",
        );
        let Some(Stmt::Mark(m)) = body.iter().find(|s| matches!(s, Stmt::Mark(_))) else {
            panic!("expected a bare Mark statement: {body:?}");
        };
        assert_eq!(m.kind, MarkKind::Ack, "three-level ~ ⇒ ACK");
        assert_eq!(m.target.prop.as_deref(), Some("held"));
    }

    #[test]
    fn bare_two_level_tilde_is_converged_vouch() {
        // `: apt-get:install~` — the CONVERGED-VOUCH placeholder (two-level provider:verb~,
        // no `.prop`). STRAWMAN spelling (dq-kOOB); distinguished from ACK by dot-absence.
        let body = body_of(
            "apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; : apt-get:install~; }",
        );
        let Some(Stmt::Mark(m)) = body.iter().find(|s| matches!(s, Stmt::Mark(_))) else {
            panic!("expected a bare Mark statement: {body:?}");
        };
        assert_eq!(
            m.kind,
            MarkKind::ConvergedVouch,
            "two-level ~ ⇒ converged-vouch"
        );
        assert_eq!(m.target.kind, "apt-get");
        assert_eq!(m.target.entity.as_deref(), Some("install"));
        assert_eq!(m.target.prop, None);
    }

    #[test]
    fn establish_with_explicit_value_parses() {
        // `… : service:"$svc".active = false` — the 233 §1 explicit-value ESTABLISH.
        let body = body_of(
            "systemctl.check() { svc : service = \"$1\"; systemctl is-active -- \"$svc\" : service:\"$svc\".active = false; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Establish);
        assert!(m.target.value.is_some(), "the `= value` tail is captured");
    }

    #[test]
    fn identity_annotation_still_parses_unchanged() {
        // The existing identity annotation `pkg : package = "$1"` (bare kind, no inner `:`)
        // must NOT be mis-read as a trailing mark — it stays an Annotation.
        let body = body_of("apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\"; }");
        assert!(
            body.iter().any(|s| matches!(s, Stmt::Annotation(_))),
            "the identity annotation survives: {body:?}"
        );
    }

    #[test]
    fn malformed_bang_on_bare_mark_rejects() {
        // `!` is a trailing-ESTABLISH suffix; on a bare statement mark it is malformed and
        // ⊤-rejects loudly (a diagnostic, never a silent mis-read).
        let mut i = Interner::default();
        let out = lift_checks(
            &mut i,
            "apt-get.check() { pkg : package = \"$1\"; : package:\"$pkg\".installed!; }",
        );
        assert!(
            !out.diags.is_empty(),
            "a bare `!` mark must ⊤-reject with a diagnostic"
        );
    }

    #[test]
    fn tilde_on_trailing_command_mark_rejects() {
        // `~` (vouch/ack) is a bare statement mark; trailing a command it is malformed.
        let mut i = Interner::default();
        let out = lift_checks(
            &mut i,
            "apt-get.check() { pkg : package = \"$1\"; dpkg-query -W \"$pkg\" : package:\"$pkg\".installed~; }",
        );
        assert!(
            !out.diags.is_empty(),
            "a trailing `~` mark must ⊤-reject with a diagnostic"
        );
    }
}
