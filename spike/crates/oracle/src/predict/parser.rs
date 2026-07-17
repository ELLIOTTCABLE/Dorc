//! The dialect parser — recursive descent over [`lexer`](super::lexer) tokens.
//!
//! The grammar IS the contract (`adj-dialect-parser`, note 203 §4). Anything
//! outside it is a per-function lift failure: a [`Diagnostic`], never a panic
//! (`inv-no-throw`), and the file's other checks still lift (fail-soft). The
//! top-level entry [`lift_predicts`] scans an oracle source for `<name>__predict`
//! function definitions and parses each body; non-check top-level items (bare
//! assignments, helper functions) are ignored — this module owns only the checks.

use super::ast::{
    Annotation, CaseArm, Command, Mark, MarkKind, MarkTarget, Pattern, Predict, PredictSet, Stmt,
    Test, TestOp, Word,
};
use super::lexer::{Tok, Token, lex};
use super::{VERB_BINDING, lift_failure};
use dorc_core::{Carrier, Interner, Span, Symbol};
use dorc_syntax::sem;

/// The provider-name suffix marking a command-keyed check (`apt_get__predict`); the
/// provider before it maps `_` → `-` ([`map_provider_name`]).
const PREDICT_SUFFIX: &str = "__predict";

/// Which role-sibling funcdef this parse scans for (rul-role-split; `277` §4d role menu).
/// The dialect GRAMMAR is identical across siblings — only the name-suffix differs — so one
/// parser lifts them all, selected by [`FnRole`]. Only the BARE munged `<base>__<role>` form
/// is recognized: the period `X.role()` form is DEAD (rul24-totalistic-munge — dots survive
/// only in kind-identity space, never in funcdef names).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FnRole {
    /// `<provider>__predict` — the entity-resolver / probe body.
    Predict,
    /// `<provider>__disturbs` (né touches) — the at-most footprint emitter
    /// (`271:rul-touches-becomes-disturbs`).
    Disturbs,
    /// `<provider>__is_converged` — the guard-verdict function (rul-role-split; sense declared
    /// by name — 0-in-the-named-sense = converged). Its authoring IS the vouch
    /// (rul24-vouch-is-verdict-authoring). `is_diverged` is RETIRED (rul24-ditch-is-diverged):
    /// the inverted sense is spelled with explicit-return `case $?` manual inversion.
    IsConverged,
    /// `<munge(kind)>__resolve` — the identity CANONICALIZER (24F §3, the resid-aliasing
    /// closure). KIND-keyed (contrast the command-keyed trio): the `<munge(kind)>` before the
    /// suffix is the kind's forward-munge (`flag-forward-munge-keying`). Host-run per coordinate.
    Resolve,
    /// `<munge(kind)>__disturbance_reaches_only` (né reaches) — the reach/footprint-EXPANSION
    /// function (`271:rul-at-most-family-names`). KIND-keyed like [`Resolve`](FnRole::Resolve).
    /// Its body is emitting arms, each carrying a TRAILING KIND mark that types the emission
    /// (`printf '%s\n' "$1" : sm.dorc.Service`). The one carve-out from the shared dialect: a
    /// pipeline in this body MAY carry a trailing mark (24G §4).
    DisturbanceReachesOnly,
    /// `<munge(kind)>__state_stored_only_in` — the substrate/invariance member (`277` §4e).
    /// KIND-keyed. Its body carries substrate token marks (`… : fs`) on emission lines plus the
    /// `invariant:<axis>` colon-line. Recognized-but-INERT at this stage: parsed + reservation-
    /// linted, never semantically consumed (topology/keying is a later stage).
    StateStoredOnlyIn,
}

impl FnRole {
    /// The bare munged suffix. Only form recognized (the period form is dead).
    const fn mangled_suffix(self) -> &'static str {
        match self {
            FnRole::Predict => PREDICT_SUFFIX,
            FnRole::Disturbs => "__disturbs",
            FnRole::IsConverged => "__is_converged",
            FnRole::Resolve => "__resolve",
            FnRole::DisturbanceReachesOnly => "__disturbance_reaches_only",
            FnRole::StateStoredOnlyIn => "__state_stored_only_in",
        }
    }
}

/// Lift every `<provider>__predict` function in `src` into a [`PredictSet`], interning
/// provider/local names through `interner`. Fail-soft (`inv-no-throw`): a body that
/// is out of dialect yields a diagnostic and contributes no [`Predict`]; the rest of
/// the file still lifts. Deterministic (`inv-determinism`): functions are processed
/// in source order and the result is `BTreeMap`-backed.
///
/// # Provider-name rule (underscore↔hyphen)
///
/// The name before `__predict` maps `_` → `-` to recover the command word
/// (`apt_get__predict` ⇒ `apt-get`, `command__predict` ⇒ `command`). This is a
/// **lossy** mapping (a real `_` in a command name cannot be expressed) — flagged
/// as a `tc-*`-shaped cross-cutting decision; chosen conservatively here (sh
/// function names cannot contain `-`, so the mapping is the only way to name a
/// hyphenated command, and hyphenated commands vastly outnumber underscored ones).
/// A future wiring task may revisit; see this module's tests and the build report.
#[must_use]
pub fn lift_predicts(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::Predict)
}

/// Lift every `<provider>__disturbs` (né touches) function in `src` into a [`PredictSet`]
/// (the disturbs funcdefs reuse the predict body dialect). Same fail-soft / deterministic
/// contract as [`lift_predicts`]; only the scanned name-suffix differs. The at-most footprint
/// LIFT (`crate::touches`) walks these bodies to collect the entity-coordinates each verb emits.
#[must_use]
pub(crate) fn lift_touches(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::Disturbs)
}

/// Lift every `<provider>.is_converged` / `<provider>__is_converged` funcdef in `src` (the
/// guard-verdict function, converged sense — rul24-vouch-is-verdict-authoring, 24A §1c). Reuses
/// the predict body dialect (one grammar; `FnRole`). Same fail-soft / deterministic contract as
/// [`lift_predicts`]; only the scanned name-suffix differs. The static consumer
/// ([`crate::verdict`]) traces these bodies to decide whether a site's argv reaches a vouching
/// path; the guard emitter ships the STRIPPED body ([`super::strip_verdict`]).
#[must_use]
pub(crate) fn lift_verdicts_converged(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::IsConverged)
}

/// Lift every `<munge(kind)>__resolve` funcdef in `src` (the identity canonicalizer — 24F §3,
/// the resid-aliasing closure). Reuses the predict body dialect. Same fail-soft / deterministic
/// contract as [`lift_predicts`]; only the scanned name-suffix differs. KIND-keyed: the lifted
/// symbol is the kind's forward-munge (`flag-forward-munge-keying`), which the lookup side matches
/// by munging the coordinate's kind identically. Host-run only; emitter is [`super::strip_resolve`].
#[must_use]
pub(crate) fn lift_resolvers(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::Resolve)
}

/// Lift every `<munge(kind)>__disturbance_reaches_only` (né reaches) funcdef in `src` (the
/// reach/footprint-expansion function — 24G §4). Reuses the predict body dialect, with the
/// pipelines-may-carry-a-mark carve-out ([`FnRole::DisturbanceReachesOnly`], applied in
/// [`Parser::parse_command`]). KIND-keyed (forward-munge). The consumer ([`crate::reaches`])
/// walks these bodies arm-by-arm; the dynamic-arm guard emitter ships the arm body strip-only.
#[must_use]
pub(crate) fn lift_reaches(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::DisturbanceReachesOnly)
}

/// Lift every `<munge(kind)>__state_stored_only_in` funcdef in `src` (the substrate/invariance
/// member — `277` §4e). Reuses the predict body dialect. KIND-keyed (forward-munge). Recognized-
/// but-INERT at this stage: lifted for the reservation lint, never semantically consumed.
#[must_use]
pub(crate) fn lift_state_stored_only_in(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::StateStoredOnlyIn)
}

/// Shared lift over a chosen [`FnRole`] — the one parse both siblings route through.
fn lift_role(interner: &mut Interner, src: &str, role: FnRole) -> Carrier<PredictSet> {
    let tokens = lex(src);
    let mut p = Parser {
        toks: &tokens,
        pos: 0,
        interner,
        out: Carrier::pure(PredictSet::default()),
        last_term: None,
        role,
    };
    p.parse_file();
    p.out
}

struct Parser<'a> {
    toks: &'a [Token],
    pos: usize,
    interner: &'a mut Interner,
    out: Carrier<PredictSet>,
    /// Which terminator [`Parser::parse_block`] last consumed. Read by
    /// [`Parser::parse_if`] to tell an `else` branch from a bare `fi`.
    last_term: Option<BlockTerm>,
    /// Which role-sibling suffix pair this parse scans for ([`FnRole`]).
    role: FnRole,
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

    /// Scan top-level items, parsing each `<name>__predict() { … }` and ignoring all
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
            if let Some(name_info) = self.at_predict_funcdef() {
                self.parse_predict_funcdef(name_info);
            } else {
                // Not a check definition — skip this one top-level item. We do not
                // diagnose (the file legitimately holds bare assignments / other
                // helper functions); we just advance past it.
                self.skip_one_toplevel_item();
            }
        }
    }

    /// If the cursor is at a role-funcdef header `<base>__<role> (`, return the keyed symbol
    /// plus the name span. Does not consume. Only the bare munged form is recognized (the period
    /// `X.role()` form is dead — rul24-totalistic-munge). For a COMMAND-keyed role the base
    /// maps `_` → `-` to recover the command word ([`map_provider_name`]); for a KIND-keyed
    /// role the base is kept AS-IS (the kind's forward-munge — `flag-forward-munge-keying`).
    /// The role is [`FnRole`]-selected, so siblings coexist in one file without cross-contam.
    fn at_predict_funcdef(&mut self) -> Option<PredictHeader> {
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
        let base = lexeme.strip_suffix(self.role.mangled_suffix())?;
        if base.is_empty() {
            return None;
        }
        // Store the RAW funcdef base segment (`apt_get`, `sm_dorc_Package`) for BOTH species
        // (`24C:rul24-totalistic-munge`): the FORWARD munge lives at the lookup sites
        // (`map_provider_name` → `to_funcname_segment`), and the collision lint needs the raw
        // source name to detect two distinct sources munging to one segment. The old command-keyed
        // backward un-munge (`_`→`-`) is deleted — it lost a literal-`_` book word and collapsed
        // the source distinction.
        let provider_name = base.to_owned();
        // Must be followed by `(` `)` for a function definition.
        if !matches!(
            self.toks.get(self.pos.saturating_add(1)).map(|t| &t.kind),
            Some(Tok::LParen)
        ) {
            return None;
        }
        let name_span = self.peek_span()?;
        let provider = self.interner.intern(&provider_name);
        Some(PredictHeader {
            provider,
            name_span,
        })
    }

    /// Parse `<name>__predict ( ) { BODY }`. On any out-of-dialect construct in the
    /// body, emit a diagnostic, drop the whole check, and resync past `}`.
    fn parse_predict_funcdef(&mut self, header: PredictHeader) {
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
                let check = Predict {
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

        // Bare statement-position marks are retired (`277` §4a — no ACK/POISON). A leading
        // `:` is now the sh no-op COMMAND (the `state_stored_only_in` colon-line, `277`
        // §4e); it falls through to `parse_command`, which recognizes a leading `:` as the
        // colon builtin and its ` : <token>` tail as the trailing mark.

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
        // kind: a single plain word (reverse-DNS string, or a short kind name — the
        // derivation keys the effect-map on it, so annotation-kind == effect-map kind).
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
        let mut mark_sigil: Option<MarkSigil> = None;
        // 24E §14: once a `|` is seen, this list-item is a PIPELINE — everything to the
        // list-item end folds into one span-covering, byte-exact-shipping Command the tracers ⊤ on.
        let mut pipeline = false;
        // §2 stdout DECLINE (`271:rul-only-oracle-bytes-ship` rider 1): whether a redirect voids
        // fd 1. Consumed only by the composed-probe coverage rule; the strip ships the verbatim span.
        let mut stdout_void = false;
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
                // without being consumed; a standalone `:`/`:!`/`:?` marker begins a
                // TRAILING mark (`277` §4a); anything else is a command word.
                Some(Tok::Word {
                    lexeme,
                    single_quoted: false,
                }) => {
                    if is_block_keyword(lexeme) {
                        CmdTok::End
                    } else if let Some(sigil) = mark_marker(lexeme) {
                        CmdTok::MarkStart(sigil)
                    } else {
                        CmdTok::Word
                    }
                }
                // A single-quoted word is always a plain command word (`':'` is a literal).
                Some(Tok::Word { .. }) => CmdTok::Word,
                Some(Tok::Redirect(t)) => CmdTok::Redirect(t.clone()),
                Some(Tok::Error(msg)) => CmdTok::Error(msg.clone()),
                // A pipe `|` (24E §14): ACCEPT it — the whole list-item is a pipeline that ships
                // byte-exact and ⊤s at trace (parse-permissively / trace-conservatively).
                Some(Tok::Pipe) => CmdTok::Pipe,
                // Any OTHER metacharacter (`(`, subshells, brackets) inside a command is still out
                // of dialect ⇒ ⊤-reject at parse (PIPES ONLY were lifted — a subshell has no
                // strip-fidelity story yet). The bias stays hard here for genuinely-unmodeled syntax.
                Some(_) => CmdTok::Other,
            };
            match class {
                CmdTok::End => break,
                CmdTok::MarkStart(sigil) => {
                    // A leading `:` (words still empty) is the sh no-op COMMAND, not a mark
                    // introducer (the `state_stored_only_in` colon-line, `277` §4e): consume
                    // it as the command word so its own ` : <token>` tail parses as the mark.
                    // Only the plain `:` sigil colon-commands; a leading `:!`/`:?` is a mark
                    // with no command ⇒ falls through to the empty-command reject below.
                    if words.is_empty() && sigil == MarkSigil::Verdict {
                        end_span = self.peek_span().unwrap_or(end_span);
                        if let Some((lexeme, single_quoted, _)) = self.take_word() {
                            words.push(parse_word_lexeme(&lexeme, single_quoted, self.interner));
                        }
                    }
                    // Inside a pipeline, an (unquoted) `:` is normally opaque pipeline text — it
                    // ships verbatim + ⊤s at trace, so it is NOT a dialect mark (a pipeline
                    // establishes/vouches nothing): consume it and keep folding (24E §14).
                    // CARVE-OUT (24G §4): in a reaches body the trailing mark TYPES the emission, so
                    // a pipeline MAY carry one — a `:` marker BREAKS the pipeline and becomes the
                    // trailing mark, exactly as for a simple command. (A `:` INSIDE a quoted
                    // pipeline arg — `sed 's|x|file:|'` — lexes single-quoted ⇒ `CmdTok::Word`,
                    // never a marker, so it stays untouched.)
                    else if pipeline && self.role != FnRole::DisturbanceReachesOnly {
                        end_span = self.peek_span().unwrap_or(end_span);
                        self.bump();
                    } else {
                        mark_sigil = Some(sigil);
                        break;
                    }
                }
                CmdTok::Word => {
                    end_span = self.peek_span().unwrap_or(end_span);
                    if let Some((lexeme, single_quoted, _)) = self.take_word() {
                        words.push(parse_word_lexeme(&lexeme, single_quoted, self.interner));
                    }
                }
                CmdTok::Redirect(text) => {
                    stdout_void = stdout_void || redirect_voids_stdout(&text);
                    end_span = self.peek_span().unwrap_or(end_span);
                    self.bump();
                }
                // 24E §14: fold the `|` (and, via the loop, every downstream stage) into ONE
                // span-covering Command. `words` keeps only the first stage's words (never
                // interpreted — the tracers ⊤ on `pipeline`). Parse-permissively; trace-conservatively.
                CmdTok::Pipe => {
                    pipeline = true;
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
        // mark), so the strip deletes exactly `[span.hi .. mark.span.hi]`. A PIPELINE (24E §14)
        // spans the whole `cmd | cmd | …` byte-exact; it carries NO mark EXCEPT in a reaches body
        // (24G §4 carve-out — there the trailing mark types the emission and IS parsed).
        let span = start_span.to(end_span);
        let carries_mark = !pipeline || self.role == FnRole::DisturbanceReachesOnly;
        let mark = if carries_mark {
            match mark_sigil {
                Some(sigil) => Some(self.parse_mark(sigil)?),
                None => None,
            }
        } else {
            None
        };
        Ok(Stmt::Command(Command {
            words,
            span,
            mark,
            pipeline,
            stdout_void,
        }))
    }

    /// Parse a dialect mark starting at the `:`/`:!`/`:?` marker token: consume the
    /// marker, the target word, and an optional `= value` tail; split the target and
    /// classify by [`MarkSigil`] (`277` §4a). All marks are TRAILING now (bare
    /// statement-position ACK/POISON marks are retired). Malformed ⇒ ⊤-reject (the
    /// parser's standing bias — never guess).
    fn parse_mark(&mut self, sigil: MarkSigil) -> Result<Mark, bool> {
        let marker_span = self.peek_span().unwrap_or(ZERO_SPAN);
        self.bump(); // the `:` / `:!` / `:?` marker word
        let Some((lexeme, _sq, target_span)) = self.take_word() else {
            return Err(self.fail_here("dialect mark requires a `kind:entity#selector` target"));
        };
        // Optional `= value` tail (a verdict-mark explicit-value assignment).
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
                self.fail_here("malformed dialect mark target (expected `kind:entity#selector`)")
            );
        };
        Ok(Mark {
            kind: classify_mark(sigil),
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

/// Header info for a recognized `<name>__predict` function.
#[derive(Clone, Copy)]
struct PredictHeader {
    provider: Symbol,
    name_span: Span,
}

/// Classification of the current token inside [`Parser::parse_command`], computed
/// while borrowing `self.toks`, then matched after the borrow is released.
enum CmdTok {
    /// A statement terminator / block keyword — ends the command (not consumed).
    End,
    /// A standalone `:`/`:!`/`:?` marker begins a trailing mark — ends the command (not
    /// consumed), carrying the [`MarkSigil`]. (A leading `:` is instead consumed as the
    /// colon-command word; see [`Parser::parse_command`].)
    MarkStart(MarkSigil),
    /// A plain word to add to the command.
    Word,
    /// A redirection chunk to fold into the verbatim span (carrying its verbatim text so
    /// [`Parser::parse_command`] can decide whether it voids fd 1 — the §2 stdout DECLINE
    /// used by the composed-probe coverage rule, `271:rul-only-oracle-bytes-ship`).
    Redirect(String),
    /// A pipe `|` (24E §14): this "command" is a PIPELINE. Accepted (parse-permissively) — the
    /// rest of the list-item folds into one span-covering Command flagged `pipeline`, which the
    /// tracers ⊤ on (trace-conservatively). NOT hard-killed (the kLANG mirror-invariant: valid sh
    /// degrades). Distinct from a case-arm pattern `|` (that is [`parse_case_arm`]'s own grammar).
    Pipe,
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
        // `${N-DEFAULT}` / `${N:-DEFAULT}` — a positional with a default when unset (the
        // `${2-}` nounset idiom, `24P` §2). The `${N#…}` prefix-strip forms are handled
        // above, so any remaining `-`/`:-` split whose head is an integer is this form.
        if let Some((num, default)) = inner.split_once(":-").or_else(|| inner.split_once('-'))
            && let Ok(n) = num.parse::<u32>()
        {
            return Word::PositionalDefault {
                n,
                default: default.to_owned(),
            };
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

/// The mark sigil family (`277` §4a): `:` verdict named sense, `:!` verdict complement
/// sense, `:?` observe. Polarity rides the sigil, never a coordinate suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkSigil {
    /// `:` — verdict, named sense (also the bare-kind emission / substrate token mark).
    Verdict,
    /// `:!` — verdict, complement sense.
    Inverted,
    /// `:?` — observe (read-only depends-upon).
    Observe,
}

/// Is `lexeme` a standalone inline-dialect mark marker? Recognizes the `277` §4a sigil
/// family (`:` / `:!` / `:?`); anything else ⇒ `None`. The space-flanked marker lexes as
/// its own word (the target `kind:entity#selector` is a separate, adjacent word).
fn mark_marker(lexeme: &str) -> Option<MarkSigil> {
    match lexeme {
        ":" => Some(MarkSigil::Verdict),
        ":!" => Some(MarkSigil::Inverted),
        ":?" => Some(MarkSigil::Observe),
        _ => None,
    }
}

/// A syntactically-split mark target — every fragment OPAQUE (`inv-referent-agnostic`,
/// never decoded). See [`split_mark_target`].
struct ParsedTarget {
    kind: String,
    entity: Option<String>,
    prop: Option<String>,
}

/// Split a mark target lexeme `kind[:entity[#selector]]` into its opaque fragments
/// (`277` §4a): split the body on the FIRST `:` (kind ⟂ rest) and the rest on the FIRST
/// `#` (entity ⟂ selector). `None` if empty or kind-less (⊤-reject upstream). The kind
/// fragment keeps its reverse-DNS dots; `.` no longer introduces anything in coordinate
/// position (the `.prop` production is dead — `277` §4a). An empty entity before the `#`
/// is the empty-entity transitional form `kind:#sel` (`entity = Some("")`, a real value,
/// not absence — `inv-referent-agnostic`: empty ≠ None). No fragment is interpreted.
fn split_mark_target(lexeme: &str) -> Option<ParsedTarget> {
    if lexeme.is_empty() {
        return None;
    }
    let (kind, rest) = match lexeme.split_once(':') {
        Some((k, r)) => (k.to_owned(), Some(r)),
        None => (lexeme.to_owned(), None),
    };
    if kind.is_empty() {
        return None;
    }
    let (entity, prop) = match rest {
        None | Some("") => (None, None),
        // The entity/selector split is on the FIRST `#` (the attached selector-introducer,
        // `277` §4a). No `#` ⇒ a whole-entity coordinate (the emission-mark and bare-entity
        // shapes). A leading `#` ⇒ the empty-entity form `kind:#sel`.
        Some(r) => match r.split_once('#') {
            Some((e, s)) if !s.is_empty() => (Some(e.to_owned()), Some(s.to_owned())),
            // `kind:entity#` (empty selector) is malformed — treat the whole rest as the
            // entity (the selector is dropped; the differential gate catches a mis-spell).
            _ => (Some(r.to_owned()), None),
        },
    };
    Some(ParsedTarget { kind, entity, prop })
}

/// Classify a parsed mark into a [`MarkKind`] from its [`MarkSigil`] (`277` §4a). All
/// marks trail a command; bare statement-position ACK/POISON marks are retired (deleted
/// from the grammar). The sigil alone decides polarity — the coordinate carries none.
fn classify_mark(sigil: MarkSigil) -> MarkKind {
    match sigil {
        MarkSigil::Verdict => MarkKind::Establish,
        MarkSigil::Inverted => MarkKind::EstablishInverted,
        MarkSigil::Observe => MarkKind::Observe,
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

/// Does a redirect chunk send fd 1 (stdout) away from where it would otherwise flow?
/// The §2 per-channel STDOUT DECLINE (`271:rul-only-oracle-bytes-ship` rider 1), used by the
/// composed-probe coverage rule: an upstream pipe stage that voids stdout starves the byte
/// consumer downstream, so the compound cannot ship (can't-say ⇒ run — the safe direction).
///
/// The chunk verbatim ([`Tok::Redirect`]) is `[fd-digits][>|<|>>|<<][&fd][target]`. Only an
/// OUTPUT redirect (`>`/`>>`) on fd 1 voids stdout: `>/dev/null`, `>file`, `>&2`, `1>…` all
/// take fd 1 off the pipe; a stderr redirect (`2>&1`, `2>/dev/null`) or any input redirect
/// (`<…`) leaves stdout on the pipe (`false`). A default (digit-less) `>` targets fd 1.
/// Bias-to-void on a malformed/unparsed fd (the refuse-direction is safe here).
fn redirect_voids_stdout(text: &str) -> bool {
    let bytes = text.as_bytes();
    let digits_end = bytes.iter().take_while(|b| b.is_ascii_digit()).count();
    // An output redirect on this fd; input redirects (`<`) never touch stdout.
    if bytes.get(digits_end) != Some(&b'>') {
        return false;
    }
    // No explicit fd ⇒ `>` defaults to fd 1; an explicit fd voids stdout only if it IS 1.
    digits_end == 0 || text.get(..digits_end) == Some("1")
}

#[cfg(test)]
mod redirect_tests {
    //! The fd discrimination in [`super::redirect_voids_stdout`] — load-bearing for the composed-probe
    //! coverage rule (`271:rul-only-oracle-bytes-ship` rider 1): only an OUTPUT redirect on fd 1 takes
    //! stdout off the pipe. A stderr redirect misread as a stdout decline would refuse valid compounds
    //! (safe but lossy); a stdout redirect missed would ship a starved compound (unsafe). Both matter.
    use super::redirect_voids_stdout;

    #[test]
    fn fd1_output_redirects_void_stdout() {
        for chunk in [">/dev/null", ">out", ">>out", ">&2", "1>/dev/null", "1>&2"] {
            assert!(
                redirect_voids_stdout(chunk),
                "`{chunk}` sends fd 1 off the pipe ⇒ voids stdout"
            );
        }
    }

    #[test]
    fn stderr_and_input_redirects_leave_stdout_on_the_pipe() {
        for chunk in ["2>&1", "2>/dev/null", "2>>log", "</dev/null", "0<in"] {
            assert!(
                !redirect_voids_stdout(chunk),
                "`{chunk}` does not touch fd 1 ⇒ stdout stays on the pipe"
            );
        }
    }
}

#[cfg(test)]
mod dialect_tests {
    //! The `277` §4 inline-mark dialect. These tests reach the internal AST (Marks aren't
    //! re-exported), so they live here. Every ambiguity ⊤-rejects (`inv-top-reject` bias); a lift
    //! failure is a diagnostic, never a panic (`inv-no-throw`).
    use super::{Interner, Mark, MarkKind, Stmt, lift_predicts};

    /// Lift `src`, assert exactly one check, and return its body statements.
    fn body_of(src: &str) -> Vec<Stmt> {
        let mut i = Interner::default();
        let out = lift_predicts(&mut i, src);
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
    fn only_the_bare_munged_funcdef_form_lifts() {
        // The period `X.predict()` form is DEAD (rul24-totalistic-munge). Only the bare
        // `<base>__predict` munged form is recognized; `apt_get__predict` keys on `apt-get`
        // (command-keyed, `_`→`-`).
        let mut i = Interner::default();
        let out = lift_predicts(
            &mut i,
            "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }",
        );
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        assert!(
            out.value.get(i.intern("apt_get")).is_some(),
            "the bare form keys the check on `apt-get`"
        );
    }

    #[test]
    fn trailing_verdict_mark_splits_kind_entity_selector() {
        // `dpkg-query … : sm.dorc.Package:"$pkg"#installed` — a trailing verdict mark. kind keeps
        // its reverse-DNS dots; entity/selector split on the attached `#` (`277` §4a).
        let body = body_of(
            "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\" : sm.dorc.Package:\"$pkg\"#installed; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Establish);
        assert_eq!(m.target.kind, "sm.dorc.Package");
        assert_eq!(m.target.entity.as_deref(), Some("$pkg"));
        assert_eq!(m.target.prop.as_deref(), Some("installed"));
    }

    #[test]
    fn inverted_sigil_is_establish_inverted() {
        // `… :! sm.dorc.Package:"$pkg"#installed` — polarity rides the `:!` sigil now, never a
        // coordinate suffix (`277` §4a). The coordinate is a pure name.
        let body = body_of(
            "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\" :! sm.dorc.Package:\"$pkg\"#installed; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::EstablishInverted);
        assert_eq!(m.target.prop.as_deref(), Some("installed"));
    }

    #[test]
    fn observe_sigil_is_observe() {
        // `… :? sm.dorc.GrepMatch:"$pat"#matched` — the observe mark (read-only depends-upon).
        let body = body_of(
            "grep__predict() { pat : sm.dorc.GrepMatch = \"$1\"; grep -q -- \"$pat\" :? sm.dorc.GrepMatch:\"$pat\"#matched; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Observe);
        assert_eq!(m.target.kind, "sm.dorc.GrepMatch");
        assert_eq!(m.target.prop.as_deref(), Some("matched"));
    }

    #[test]
    fn empty_entity_form_carries_an_explicit_empty_entity() {
        // `… :? io.opentelemetry.Collector:#v0155` — the empty-entity transitional form `kind:#sel`
        // (`277` §4a): the entity slot is a DELIBERATE empty string (the-one), never `None`.
        let body = body_of(
            "otelcol__predict() { case $1 in --version) otelcol --version >/dev/null 2>&1 :? io.opentelemetry.Collector:#v0155 ;; esac }",
        );
        let m = first_command_mark(&body).expect("an empty-entity observe mark");
        assert_eq!(m.kind, MarkKind::Observe);
        assert_eq!(m.target.kind, "io.opentelemetry.Collector");
        assert_eq!(
            m.target.entity.as_deref(),
            Some(""),
            "the entity slot is an EXPLICIT empty string, never None"
        );
        assert_eq!(m.target.prop.as_deref(), Some("v0155"));
    }

    #[test]
    fn bare_kind_emission_mark_has_no_entity_or_selector() {
        // `printf '%s\n' "$1" : sm.dorc.Package` — the disturbs()/reaches() emission shape: the
        // kind rides the trailing mark, no entity, no selector.
        let body = body_of("apt_get__predict() { printf '%s\\n' \"$1\" : sm.dorc.Package; }");
        let m = first_command_mark(&body).expect("a bare-kind emission mark");
        assert_eq!(m.kind, MarkKind::Establish);
        assert_eq!(m.target.kind, "sm.dorc.Package");
        assert_eq!(m.target.entity, None);
        assert_eq!(m.target.prop, None);
    }

    #[test]
    fn colon_line_is_a_command_carrying_a_trailing_token_mark() {
        // The `state_stored_only_in` colon-line shape (`277` §4e): a leading `:` is the sh no-op
        // COMMAND (not a mark introducer), and its ` : <token>` tail is the trailing mark. A
        // substrate `: fs` and the `: invariant:user` axis line both parse this way, inert.
        let body =
            body_of("dpkg__predict() { printf '/var/lib/dpkg\\n' : fs; : : invariant:user; }");
        // First stmt: `printf … : fs`.
        let Some(Stmt::Command(first)) = body.first() else {
            panic!("expected a command: {body:?}");
        };
        let m = first.mark.as_ref().expect("the `: fs` substrate mark");
        assert_eq!(m.target.kind, "fs");
        // Second stmt: the colon-command `:` carrying `: invariant:user`.
        let Some(Stmt::Command(second)) = body.get(1) else {
            panic!("expected a colon-command: {body:?}");
        };
        let m2 = second.mark.as_ref().expect("the invariant colon-line mark");
        assert_eq!(m2.target.kind, "invariant");
        assert_eq!(m2.target.entity.as_deref(), Some("user"));
    }

    #[test]
    fn dotted_entity_parses_unambiguously_now() {
        // With the `.prop` production dead (`277` §4a), an unquoted dotted entity parses cleanly —
        // the ⊤-reject corner is DISSOLVED. `sm.dorc.File:/etc/nginx.conf` ⇒ kind before the `:`,
        // everything after is the entity (no `#`, so no selector).
        let body =
            body_of("writeconf__predict() { conf-exists \"$1\" : sm.dorc.File:/etc/nginx.conf; }");
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.target.kind, "sm.dorc.File");
        assert_eq!(m.target.entity.as_deref(), Some("/etc/nginx.conf"));
        assert_eq!(m.target.prop, None);
    }

    #[test]
    fn verdict_mark_with_explicit_value_parses() {
        // `… : sm.dorc.Service:"$svc"#active = false` — the explicit-value tail on a verdict mark.
        let body = body_of(
            "systemctl__predict() { svc : sm.dorc.Service = \"$1\"; systemctl is-active -- \"$svc\" : sm.dorc.Service:\"$svc\"#active = false; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Establish);
        assert!(m.target.value.is_some(), "the `= value` tail is captured");
    }

    #[test]
    fn positional_default_nounset_idiom_parses() {
        // `${2-}` (the nounset idiom, `24P` §2) parses as PositionalDefault, NOT Unmodeled — so
        // `[ "${2-}" = "" ]` resolves the operand-count guard (the site would be un-probeable if it
        // degraded to ⊤). Adversarial: a non-empty default and the `:-` spelling both parse.
        use super::{Word, parse_word_lexeme};
        let mut i = Interner::default();
        assert_eq!(
            parse_word_lexeme("${2-}", false, &mut i),
            Word::PositionalDefault {
                n: 2,
                default: String::new()
            }
        );
        assert_eq!(
            parse_word_lexeme("${1:-def}", false, &mut i),
            Word::PositionalDefault {
                n: 1,
                default: "def".to_owned()
            }
        );
    }

    #[test]
    fn identity_annotation_still_parses_unchanged() {
        // The identity annotation `pkg : sm.dorc.Package = "$1"` (bare kind, no inner `:`) stays an
        // Annotation, not mis-read as a trailing mark.
        let body = body_of(
            "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\"; }",
        );
        assert!(
            body.iter().any(|s| matches!(s, Stmt::Annotation(_))),
            "the identity annotation survives: {body:?}"
        );
    }

    #[test]
    fn mark_without_selector_is_whole_entity() {
        // `sm.dorc.Package:"$pkg"` (no `#`) ⇒ a whole-entity coordinate: entity present, selector
        // absent. The near-miss the differential gate would catch if a selector were intended.
        let body = body_of("apt_get__predict() { dpkg-query -W \"$1\" : sm.dorc.Package:\"$1\"; }");
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.target.kind, "sm.dorc.Package");
        assert_eq!(m.target.entity.as_deref(), Some("$1"));
        assert_eq!(m.target.prop, None, "no `#` ⇒ no selector");
    }

    /// The sole body command of a lifted reaches funcdef (the pipeline-mark carve-out tests inspect
    /// its `pipeline`/`mark` directly).
    fn reaches_command(src: &str) -> super::Command {
        let mut i = Interner::default();
        let out = super::lift_reaches(&mut i, src);
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        let kind = out.value.providers().next().expect("one reaches kind");
        let body = out
            .value
            .get(kind)
            .expect("the reaches funcdef")
            .body
            .clone();
        match body.into_iter().next() {
            Some(Stmt::Command(c)) => c,
            other => panic!("expected a single command, got {other:?}"),
        }
    }

    /// 24G §4 CARVE-OUT: a reaches-body PIPELINE carries its trailing mark (the mark types the
    /// emission) — unlike every other role, where a pipeline carries none (24E §14). Adversarial:
    /// the mark rides AFTER a MULTI-STAGE pipe.
    #[test]
    fn reaches_pipeline_carries_trailing_mark_after_multi_stage_pipe() {
        let c = reaches_command(
            "sm_dorc_Package__disturbance_reaches_only() { dpkg -L \"$1\" | grep x | sed y : sm.dorc.File ; }",
        );
        assert!(c.pipeline, "a multi-stage pipe is a pipeline");
        let m = c
            .mark
            .expect("the reaches pipeline carries its trailing mark (the carve-out)");
        assert_eq!(m.kind, MarkKind::Establish);
        assert_eq!(m.target.kind, "sm.dorc.File");
    }

    /// A `:` INSIDE a quoted pipeline arg (`'s|x|file:|'`) is NOT a mark — a single-quoted token
    /// lexes as one opaque word, so the inner `:` stays UNTOUCHED and the mark is the TRAILING one.
    #[test]
    fn reaches_pipeline_quoted_colon_inside_arg_is_not_a_mark() {
        let c = reaches_command(
            "sm_dorc_Package__disturbance_reaches_only() { dpkg -L \"$1\" | sed 's|x|file:|' : sm.dorc.File ; }",
        );
        assert!(c.pipeline);
        let m = c
            .mark
            .expect("the TRAILING mark (the quoted inner colon is not)");
        assert_eq!(m.target.kind, "sm.dorc.File");
    }

    /// The carve-out is REACHES-ONLY: in a PREDICT body the same `pipe … : foo` shape carries NO
    /// mark — the `:` folds as opaque pipeline text (24E §14), so the carve-out cannot leak into
    /// probe/verdict bodies where a trailing mark is meaningless.
    #[test]
    fn predict_pipeline_folds_the_colon_no_mark_carveout_is_reaches_only() {
        let mut i = Interner::default();
        let out = lift_predicts(
            &mut i,
            "apt_get__predict() { dpkg -L \"$1\" | grep x : foo ; }",
        );
        assert!(out.diags.is_empty(), "clean lift: {:?}", out.diags);
        let r = out
            .value
            .get(i.intern("apt_get"))
            .expect("the predict funcdef");
        let Some(Stmt::Command(c)) = r.body.first() else {
            panic!("expected a command: {:?}", r.body)
        };
        assert!(c.pipeline);
        assert!(
            c.mark.is_none(),
            "a predict pipeline carries no mark (the `: foo` is folded pipeline text)"
        );
    }
}
