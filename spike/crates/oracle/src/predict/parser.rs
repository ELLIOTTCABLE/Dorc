//! The dialect parser — recursive descent over [`lexer`](super::lexer) tokens.
//!
//! The grammar IS the contract (`adj-dialect-parser`, note 203 §4). Anything
//! outside it is a per-function lift failure: a [`Diagnostic`], never a panic
//! (`inv-no-throw`), and the file's other checks still lift (fail-soft). The
//! top-level entry [`lift_predicts`] scans an oracle source for `<name>__predict`
//! function definitions and parses each body; non-check top-level items (bare
//! assignments, helper functions) are ignored — this module owns only the checks.

use super::ast::{
    AndOr, AndOrItem, AndOrLink, AndOrOp, Annotation, CaseArm, Command, Mark, MarkKind, MarkTarget,
    Pattern, Predict, PredictSet, RefusedMark, Stmt, Test, TestOp, Word,
};
use super::lexer::{Tok, Token, lex};
use super::{VERB_BINDING, out_of_dialect, unterminated};
use dorc_aid::diag::{
    DiagCode, MarkHashcolonMalformed, MarkOnAndOrList, MarkRcArityExceeded,
    MarkStandaloneRcConsumer, MarkUnknownVerb, PredictLexError, PredictOutOfDialectReason,
    PredictUnterminatedReason,
};
use dorc_aid::{Carrier, Diag};
use dorc_core::{Interner, Span, Symbol};
use dorc_syntax::sem;

/// The provider-name suffix marking a command-keyed check (`apt_get__predict`); the
/// provider before it maps `_` → `-` ([`map_provider_name`]).
///
/// `pub` for the same reason [`VERDICT_SUFFIX`](crate::verdict::VERDICT_SUFFIX) is: the positional
/// visibility gate (`28K` §2) asks the function environment about the exact NAME a lifted role
/// would have been authored under, and re-spelling it at the consumer is how a suffix drifts.
pub const PREDICT_SUFFIX: &str = "__predict";

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
    /// (rul24-vouch-is-verdict-authoring). The sole verdict role (`24C:rul24-ditch-is-diverged`).
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
    /// KIND-keyed. Its body carries substrate token marks (`… : stored-in fs`) on emission lines plus
    /// the `undivided-by-transit-across <axis>` colon-line. Recognized-but-INERT at this stage: parsed + reservation-
    /// linted, never semantically consumed (topology/keying is a later stage).
    StateStoredOnlyIn,
    /// `<provider>__lend_map` — the wrapper's DIMENSION member (`271:rul-lend-map`; `273` §3).
    /// COMMAND-keyed (like the wrapper's `predict`). Its body is one entry per dimension: a
    /// colon-line `: lends user` = full lend, a `printf … : lends user` = mapped lend, a terminal `"$@"`
    /// = the peel boundary. Reuses the predict body dialect; the consumer ([`crate::wrapper`])
    /// reads the per-dimension entries off it. The enumerate-every-dimension law lives in the
    /// consumer (an absent dimension is ⊤ — walls).
    LendMap,
    /// `<provider>__enter` — the wrapper's ENTRY-FORM member (`27C` §3). COMMAND-keyed (like
    /// `predict`/`lend_map`). The ONE licensed seat for REAL context entry: its body is the
    /// non-interactive-by-construction entry command wrapping the guest (`sudo__enter() { sudo -n
    /// "$@" ;}`). Authoring it IS the traversal vouch (`authoring-is-vouching`): the author answers
    /// for the entry's self-effects (`27C:rul-probe-mutation-ownership-split`). Reuses the predict
    /// body dialect; the consumer ([`crate::entry`]) reads the entry head + non-interactivity off it.
    Enter,
}

impl FnRole {
    /// The bare munged suffix. Only form recognized (the period form is dead).
    const fn mangled_suffix(self) -> &'static str {
        match self {
            FnRole::Predict => PREDICT_SUFFIX,
            FnRole::Disturbs => crate::touches::DISTURBS_SUFFIX,
            FnRole::IsConverged => "__is_converged",
            FnRole::Resolve => "__resolve",
            FnRole::DisturbanceReachesOnly => crate::reaches::DISTURBANCE_REACHES_ONLY_SUFFIX,
            FnRole::StateStoredOnlyIn => "__state_stored_only_in",
            FnRole::LendMap => crate::wrapper::LEND_MAP_SUFFIX,
            FnRole::Enter => crate::entry::ENTER_SUFFIX,
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
pub fn lift_predicts(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::Predict)
}

/// Lift every `<provider>__disturbs` (né touches) function in `src` into a [`PredictSet`]
/// (the disturbs funcdefs reuse the predict body dialect). Same fail-soft / deterministic
/// contract as [`lift_predicts`]; only the scanned name-suffix differs. The at-most footprint
/// LIFT (`crate::touches`) walks these bodies to collect the entity-coordinates each verb emits.
pub(crate) fn lift_touches(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::Disturbs)
}

/// Lift every `<provider>.is_converged` / `<provider>__is_converged` funcdef in `src` (the
/// guard-verdict function, converged sense — rul24-vouch-is-verdict-authoring, 24A §1c). Reuses
/// the predict body dialect (one grammar; `FnRole`). Same fail-soft / deterministic contract as
/// [`lift_predicts`]; only the scanned name-suffix differs. The static consumer
/// ([`crate::verdict`]) traces these bodies to decide whether a site's argv reaches a vouching
/// path; the guard emitter ships the STRIPPED body ([`super::strip_verdict`]).
pub(crate) fn lift_verdicts_converged(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::IsConverged)
}

/// Lift every `<munge(kind)>__resolve` funcdef in `src` (the identity canonicalizer — 24F §3,
/// the resid-aliasing closure). Reuses the predict body dialect. Same fail-soft / deterministic
/// contract as [`lift_predicts`]; only the scanned name-suffix differs. KIND-keyed: the lifted
/// symbol is the kind's forward-munge (`flag-forward-munge-keying`), which the lookup side matches
/// by munging the coordinate's kind identically. Host-run only; emitter is [`super::strip_resolve`].
pub(crate) fn lift_resolvers(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::Resolve)
}

/// Lift every `<munge(kind)>__disturbance_reaches_only` (né reaches) funcdef in `src` (the
/// reach/footprint-expansion function — 24G §4). Reuses the predict body dialect, with the
/// pipelines-may-carry-a-mark carve-out ([`FnRole::DisturbanceReachesOnly`], applied in
/// [`Parser::parse_command`]). KIND-keyed (forward-munge). The consumer ([`crate::reaches`])
/// walks these bodies arm-by-arm; the dynamic-arm guard emitter ships the arm body strip-only.
pub(crate) fn lift_reaches(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::DisturbanceReachesOnly)
}

/// Lift every `<munge(kind)>__state_stored_only_in` funcdef in `src` (the substrate/invariance
/// member — `277` §4e). Reuses the predict body dialect. KIND-keyed (forward-munge). Recognized-
/// but-INERT at this stage: lifted for the reservation lint, never semantically consumed.
pub(crate) fn lift_state_stored_only_in(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::StateStoredOnlyIn)
}

/// Lift every `<provider>__lend_map` funcdef in `src` (the wrapper dimension member —
/// `271:rul-lend-map`; `273` §3). Reuses the predict body dialect. Same fail-soft / deterministic
/// contract as [`lift_predicts`]; only the scanned name-suffix differs. COMMAND-keyed (the
/// underscore↔hyphen provider mapping, like `predict`). The consumer ([`crate::wrapper`]) reads
/// the per-dimension lend entries off each body.
pub(crate) fn lift_lend_maps(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::LendMap)
}

/// Lift every `<provider>__enter` funcdef in `src` (the wrapper ENTRY-FORM member — `27C` §3).
/// Reuses the predict body dialect. Same fail-soft / deterministic contract as [`lift_predicts`];
/// only the scanned name-suffix differs. COMMAND-keyed (the underscore↔hyphen provider mapping, like
/// `predict`/`lend_map`). The consumer ([`crate::entry`]) reads the entry head + its
/// non-interactivity off each body.
pub(crate) fn lift_enters(interner: &mut Interner, src: &str) -> Carrier<PredictSet> {
    lift_role(interner, src, FnRole::Enter)
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
    fn take_word(&mut self) -> Option<(String, WordQuoting, Span)> {
        let (lexeme, single_quoted, double_quoted, span) = match self.toks.get(self.pos) {
            Some(Token {
                kind:
                    Tok::Word {
                        lexeme,
                        single_quoted,
                        double_quoted,
                    },
                span,
            }) => (lexeme.clone(), *single_quoted, *double_quoted, *span),
            _ => return None,
        };
        self.pos = self.pos.saturating_add(1);
        Some((
            lexeme,
            WordQuoting {
                single_quoted,
                double_quoted,
            },
            span,
        ))
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
        matches!(self.peek(), Some(Tok::Word { lexeme, single_quoted, .. })
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
                // Recorded BEFORE the body is attempted: whatever happens next — a give-up, a
                // resync, a drop path nobody has hit yet — the file DECLARED this funcdef, and
                // `PredictSet::unlifted` is what notices when it then fails to appear.
                self.out.value.detected.push(super::ast::DetectedFn {
                    name: name_info.name.clone(),
                    provider: name_info.provider,
                    name_span: name_info.name_span,
                });
                self.parse_predict_funcdef(&name_info);
            } else {
                // Not a check definition — skip this one top-level item. We do not
                // diagnose (the file legitimately holds bare assignments / other
                // helper functions); we just advance past it.
                self.skip_one_toplevel_item();
            }
        }
    }

    fn parse_mark_validation_body(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.pos < self.toks.len() {
            self.skip_separators();
            if self.pos >= self.toks.len() {
                break;
            }
            match self.parse_stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(_) => break,
            }
        }
        stmts
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
            ..
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
        let name = format!("{provider_name}{}", self.role.mangled_suffix());
        let provider = self.interner.intern(&provider_name);
        Some(PredictHeader {
            provider,
            name_span,
            name,
        })
    }

    /// Parse `<name>__predict ( ) { BODY }`. On any out-of-dialect construct in the
    /// body, emit a diagnostic, drop the whole check, and resync past `}`.
    fn parse_predict_funcdef(&mut self, header: &PredictHeader) {
        self.bump(); // the name word
        // `(` `)`
        if !self.expect(&Tok::LParen) || !self.expect(&Tok::RParen) {
            self.fail(
                header.name_span,
                PredictOutOfDialectReason::MalformedFunctionHeader,
            );
            self.resync_past_brace();
            return;
        }
        self.skip_newlines();
        if !self.expect(&Tok::LBrace) {
            self.fail(
                header.name_span,
                PredictOutOfDialectReason::FunctionBodyMustStartWithBrace,
            );
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
                self.out
                    .diags
                    .extend(validate_mark_subset(&check.body, self.toks));
                self.out.value.checks.insert(header.provider, check);
            }
            Err(diag_emitted) => {
                if !diag_emitted {
                    self.fail(
                        header.name_span,
                        PredictOutOfDialectReason::CheckBodyOutOfDialect,
                    );
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
        if self.at_test_led_list() {
            let test = self.parse_bracket_test()?;
            return self.parse_and_or_tail(AndOrItem::Test(test));
        }
        // Otherwise it is a word-led line: an assignment, an annotation, or a plain
        // command. Decide by looking at the word shape and what follows.
        let stmt = self.parse_word_led()?;
        match (self.and_or_op(), stmt) {
            (Some(_), Stmt::Command(first)) => self.parse_and_or_tail(AndOrItem::Command(first)),
            (Some(_), _) => {
                Err(self.fail_here(PredictOutOfDialectReason::AndOrListNotLedByCommand))
            }
            (None, stmt) => Ok(stmt),
        }
    }

    /// Does an IN-DIALECT bracket test lead an and-or list here — `[ W = W ] &&`/`||`?
    ///
    /// A `[ … ]` test is a statement ONLY in this position (`281`-era dialect: a bare test
    /// statement stays out of dialect, so the tracers never meet a rc they cannot place). Decided
    /// by LOOKAHEAD, in the shape [`parse_word_led`](Self::parse_word_led) already uses to tell an
    /// annotation from a command: anything that is not exactly this closed form is left to the
    /// word-led path and its diagnostic, so opening this form moves no other input's give-up.
    fn at_test_led_list(&self) -> bool {
        if !matches!(self.peek(), Some(Tok::LBracket)) {
            return false;
        }
        let kind = |at: usize| self.toks.get(at).map(|t| &t.kind);
        let is_op = matches!(
            kind(self.pos.saturating_add(2)),
            Some(Tok::Word { lexeme, single_quoted: false, .. }) if lexeme == "=" || lexeme == "!="
        );
        let closes = matches!(kind(self.pos.saturating_add(4)), Some(Tok::RBracket));
        let joins = matches!(
            kind(self.pos.saturating_add(5)),
            Some(Tok::DAmp | Tok::DPipe)
        );
        matches!(kind(self.pos.saturating_add(1)), Some(Tok::Word { .. }))
            && is_op
            && matches!(kind(self.pos.saturating_add(3)), Some(Tok::Word { .. }))
            && closes
            && joins
    }

    /// The and-or operator at the cursor, if any.
    fn and_or_op(&self) -> Option<AndOrOp> {
        match self.peek()? {
            Tok::DAmp => Some(AndOrOp::AndThen),
            Tok::DPipe => Some(AndOrOp::OrElse),
            Tok::Amp => Some(AndOrOp::Async),
            _ => None,
        }
    }

    /// Assemble an [`Stmt::AndOr`] from an already-parsed first item and the operator run that
    /// follows it. Each item parses by the SAME `parse_command` the standalone form uses, so a
    /// list ships byte-exact and its items keep their redirect/pipeline/sink analysis.
    fn parse_and_or_tail(&mut self, first: AndOrItem) -> Result<Stmt, bool> {
        let start = item_span(&first);
        let mut end = start;
        let mut rest: Vec<AndOrLink> = Vec::new();
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        while let Some(op) = self.and_or_op() {
            steps = steps.saturating_add(1);
            if steps > guard {
                return Err(false); // termination guard
            }
            let op_span = self.peek_span().unwrap_or(end);
            self.bump();
            self.skip_newlines(); // sh continues a list across a newline after its operator

            let Stmt::Command(cmd) = self.parse_word_led()? else {
                return Err(self.fail_here(PredictOutOfDialectReason::AndOrListItemNotCommand));
            };
            end = cmd.span;
            rest.push(AndOrLink {
                op,
                op_span,
                item: AndOrItem::Command(cmd),
            });
        }
        let mut list = AndOr {
            first,
            rest,
            span: start.to(end),
            refused_marks: Vec::new(),
        };
        self.refuse_item_marks(&mut list);
        Ok(Stmt::AndOr(list))
    }

    /// Strip every trailing mark off an and-or list's items into
    /// [`AndOr::refused_marks`](super::ast::AndOr::refused_marks), one loud diagnostic each. See
    /// that field for why a list may not carry one; the bytes still erase, so `strip` is unaffected.
    fn refuse_item_marks(&mut self, list: &mut AndOr) {
        let mut refused = Vec::new();
        let items =
            std::iter::once(&mut list.first).chain(list.rest.iter_mut().map(|l| &mut l.item));
        for item in items {
            let AndOrItem::Command(cmd) = item else {
                continue;
            };
            let Some(mark) = cmd.mark.take() else {
                continue;
            };
            refused.push(RefusedMark {
                host: cmd.span,
                mark,
            });
        }
        for r in &refused {
            self.out.push(Diag::new(
                DiagCode::MarkOnAndOrList(MarkOnAndOrList),
                r.mark.span,
            ));
        }
        list.refused_marks = refused;
    }

    fn parse_while(&mut self) -> Result<Stmt, bool> {
        self.bump(); // `while`
        let test = self.parse_bracket_test()?;
        self.skip_separators();
        if !self.eat_keyword("do") {
            return Err(self.fail_here(PredictOutOfDialectReason::ExpectedDoAfterWhileTest));
        }
        let body = self.parse_block(BlockEnd::Keyword("done"))?;
        Ok(Stmt::While { test, body })
    }

    fn parse_if(&mut self) -> Result<Stmt, bool> {
        self.bump(); // `if`
        let test = self.parse_bracket_test()?;
        self.skip_separators();
        if !self.eat_keyword("then") {
            return Err(self.fail_here(PredictOutOfDialectReason::ExpectedThenAfterIfTest));
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
            return Err(self.fail_here(PredictOutOfDialectReason::ExpectedInAfterCaseScrutinee));
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
                return Err(self.fail_here(PredictOutOfDialectReason::UnterminatedCaseExpectedEsac));
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
                _ => {
                    return Err(self.fail_here(
                        PredictOutOfDialectReason::ExpectedPipeOrRparenInCaseArmPattern,
                    ));
                }
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
                ..
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
                    Err(self.fail_here(PredictOutOfDialectReason::CasePatternOutOfDialect))
                } else {
                    Ok(Pattern::Literal(lexeme))
                }
            }
            _ => Err(self.fail_here(PredictOutOfDialectReason::ExpectedCaseArmPattern)),
        }
    }

    fn parse_shift(&mut self) -> Result<Stmt, bool> {
        self.bump(); // `shift`
        // Optional numeric argument — a plain (non-single-quoted) word.
        let Some(Tok::Word {
            lexeme,
            single_quoted: false,
            ..
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
        Err(self.fail_here(PredictOutOfDialectReason::ShiftCountNotLiteralInteger))
    }

    /// Parse a word-led line: an annotation (`name : kind = value`), an assignment
    /// (`name=value`), or a plain command (`dpkg-query -W "$pkg"`).
    fn parse_word_led(&mut self) -> Result<Stmt, bool> {
        // Peek the first word's raw lexeme to classify.
        let Some(Tok::Word {
            lexeme,
            single_quoted,
            ..
        }) = self.peek()
        else {
            // A line that does not start with a word (e.g. a stray `]`, redirect,
            // or error token) is out of dialect.
            return Err(self.fail_here(PredictOutOfDialectReason::StatementDoesNotStartWithWord));
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
            // The RHS is a sub-lexeme of the `name=rest` word; a whole-word double-quote
            // never survives the `=` split (`x="$@"` lexes multi-part ⇒ neither flag), so
            // the value's quoting is UNQUOTED here — correct: an assignment RHS `"$@"` is a
            // value-position ⊤ regardless (`27H`).
            let value = parse_word_lexeme(rest, WordQuoting::unquoted(), self.interner);
            return Ok(Stmt::Assign {
                name: self.interner.intern(name),
                value,
            });
        }

        // Identity annotation `name : kind [= value]`: first word is a plain ident, next
        // is the standalone word `:`, AND the kind word after it is a BARE kind (no inner
        // `:`), AND its tail is an annotation tail. A Singleton annotation ends at its kind.
        // A `kind:entity.prop` after the `:` means this is a trailing ESTABLISH on the
        // single-word command `name` (not an identity annotation) — fall through to
        // `parse_command`, which re-sees the `:` marker and parses the trailing mark.
        if !first_sq
            && sem::is_name(&first)
            && self.next_word_is(":")
            && self.annotation_tail_is_valid()
            && let Some((kind, kind_span)) = self.bare_kind_after_colon()
        {
            return self.parse_annotation(&first, start_span, kind, kind_span);
        }

        // Otherwise: a plain command (optionally with a trailing ESTABLISH/OBSERVE mark).
        // Consume words/redirects to the statement end or a `:`/`:?` marker.
        self.parse_command(start_span)
    }

    /// Peek: is the token at `pos+1` the standalone word `s`?
    fn next_word_is(&self, s: &str) -> bool {
        matches!(
            self.toks.get(self.pos.saturating_add(1)).map(|t| &t.kind),
            Some(Tok::Word { lexeme, single_quoted: false, .. }) if lexeme == s
        )
    }

    /// Peek: the BARE kind word at `pos+2` (after a `name :`) — a plain word with no inner `:`
    /// (so it is an identity-annotation kind, not a `kind:entity.prop` mark target).
    /// Absent/quoted/`:`-bearing ⇒ `None`.
    ///
    /// It hands the word BACK rather than answering yes/no so that [`Self::parse_annotation`]
    /// receives a kind it cannot fail to read: the recognition and the take were two acts, and the
    /// second one carried a refusal reason for a state the first had already excluded.
    fn bare_kind_after_colon(&self) -> Option<(String, Span)> {
        match self.toks.get(self.pos.saturating_add(2)) {
            Some(Token {
                kind:
                    Tok::Word {
                        lexeme,
                        single_quoted: false,
                        ..
                    },
                span,
            }) if !lexeme.contains(':') => Some((lexeme.clone(), *span)),
            _ => None,
        }
    }

    /// A value-less Singleton annotation ends after its kind.
    fn annotation_tail_is_valid(&self) -> bool {
        let tail = self.toks.get(self.pos.saturating_add(3)).map(|t| &t.kind);
        matches!(
            tail,
            None | Some(Tok::Newline | Tok::Semi | Tok::DSemi | Tok::RBrace)
        ) || matches!(tail, Some(Tok::Word { lexeme, single_quoted: false, .. }) if lexeme == "=")
    }

    /// Parse the inline annotation `name : kind = value` (the operand form) or
    /// `name : kind` (the **nullary/Singleton** form — a verb whose resource has no
    /// operand, e.g. `apt-get update`; 202 §2 / task-W §4). The caller verified the first word is
    /// `name`, the next is `:`, and the third is the bare `kind` it hands in — the derivation keys
    /// the effect-map on that string, so annotation-kind == effect-map kind.
    fn parse_annotation(
        &mut self,
        name: &str,
        start_span: Span,
        kind: String,
        kind_span: Span,
    ) -> Result<Stmt, bool> {
        let name_sym = self.interner.intern(name);
        self.bump(); // name
        self.bump(); // `:`
        self.bump(); // kind
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
        let Some((lexeme, quoting, val_span)) = self.take_word() else {
            return Err(self.fail_here(PredictOutOfDialectReason::AnnotationNeedsValueWord));
        };
        let value = parse_word_lexeme(&lexeme, quoting, self.interner);
        Ok(Stmt::Annotation(Annotation {
            name: name_sym,
            kind,
            value: Some(value),
            span: start_span.to(val_span),
            name_span: start_span,
            value_span: Some(val_span),
        }))
    }

    /// Classify the current token for [`parse_command`](Self::parse_command), without holding a
    /// borrow across the loop body. A block-ending keyword (`done`/`fi`/…) and a statement
    /// terminator END the command (not consumed); a `281` mark intro begins a trailing mark; a
    /// pipe folds the list-item into a byte-exact pipeline; a subshell/metachar is out-of-dialect.
    fn classify_cmd_tok(&self) -> CmdTok {
        match self.peek() {
            None
            | Some(
                Tok::Newline
                | Tok::Semi
                | Tok::DSemi
                | Tok::RBrace
                | Tok::DPipe
                | Tok::DAmp
                | Tok::Amp,
            ) => CmdTok::End,
            Some(Tok::Word {
                lexeme,
                single_quoted: false,
                ..
            }) => {
                if is_block_keyword(lexeme) {
                    CmdTok::End
                } else if let Some(intro) = mark_marker(lexeme) {
                    CmdTok::MarkStart(intro)
                } else {
                    CmdTok::Word
                }
            }
            // A single-quoted word is always a plain command word (`':'` is a literal).
            Some(Tok::Word { .. }) => CmdTok::Word,
            Some(Tok::Redirect(t)) => CmdTok::Redirect(t.clone()),
            Some(Tok::Error(reason)) => CmdTok::Error(*reason),
            // A pipe `|` (24E §14): ACCEPT it — the whole list-item ships byte-exact and ⊤s at
            // trace (parse-permissively / trace-conservatively).
            Some(Tok::Pipe) => CmdTok::Pipe,
            // Any OTHER metacharacter (`(`, subshells, brackets) is out of dialect ⇒ ⊤-reject.
            Some(_) => CmdTok::Other,
        }
    }

    /// Parse a plain command: a run of words and redirects up to a statement
    /// terminator (`;`, `;;`, newline, `}`, or a block keyword). Records the
    /// verbatim source span (`Command::span`) for shipping into the probe.
    fn parse_command(&mut self, start_span: Span) -> Result<Stmt, bool> {
        let mut words = Vec::new();
        let mut end_span = start_span;
        let mut mark_intro: Option<MarkIntro> = None;
        // 24E §14: once a `|` is seen, this list-item is a PIPELINE — everything to the
        // list-item end folds into one span-covering, byte-exact-shipping Command the tracers ⊤ on.
        let mut pipeline = false;
        // §2 stdout DECLINE (`271:rul-only-oracle-bytes-ship` rider 1): whether a redirect voids
        // fd 1. Consumed only by the composed-probe coverage rule; the strip ships the verbatim span.
        let mut stdout_void = false;
        // `27W` §2 report-sink recognition (`>>"${DREP_V1:-…}"`): `awaits_report_target` bridges
        // the two tokens the quoted-sink idiom splits into — the `>>` chunk, then the target word.
        let mut report_sink = false;
        let mut awaits_report_target = false;
        let guard = self.toks.len().saturating_add(1);
        let mut steps = 0usize;
        loop {
            steps = steps.saturating_add(1);
            if steps > guard {
                return Err(false);
            }
            // One-shot: only the token IMMEDIATELY after a bare `>>` can be the sink target.
            let target_pending = awaits_report_target;
            awaits_report_target = false;
            match self.classify_cmd_tok() {
                CmdTok::End => break,
                CmdTok::MarkStart(intro) => {
                    // A statement-leading intro (words still empty). A lone `:` (colon carrier, no
                    // sugar) with no following mark word is the POSIX null command — consume it as a
                    // command word (`28A:rul-marked-colon-is-the-grammars`; `while :; do` survives).
                    // Any other leading intro synthesizes a `:` host so the mark trails a bare-colon
                    // command (keeps `is_bare_colon_host` + strip whole-line; the intro token is NOT
                    // consumed here — `parse_mark` bumps it). The source bytes strip by span.
                    if words.is_empty() {
                        let next_is_word = matches!(
                            self.toks.get(self.pos.saturating_add(1)).map(|t| &t.kind),
                            Some(Tok::Word { .. })
                        );
                        if intro.carrier == MarkCarrier::Colon
                            && intro.sugar.is_none()
                            && !next_is_word
                        {
                            end_span = self.peek_span().unwrap_or(end_span);
                            if let Some((lexeme, quoting, _)) = self.take_word() {
                                words.push(parse_word_lexeme(&lexeme, quoting, self.interner));
                            }
                        } else {
                            end_span = self.peek_span().unwrap_or(end_span);
                            words.push(Word::Literal(":".to_owned()));
                            mark_intro = Some(intro);
                            break;
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
                        mark_intro = Some(intro);
                        break;
                    }
                }
                CmdTok::Word => {
                    end_span = self.peek_span().unwrap_or(end_span);
                    if let Some((lexeme, quoting, _)) = self.take_word() {
                        // A recognized sink target after `>>` flags a `27W` §2 emission (the word
                        // still pushes to `words`; the tracer skips a `report_sink` cmd first).
                        report_sink =
                            report_sink || (target_pending && word_is_report_sink(&lexeme));
                        words.push(parse_word_lexeme(&lexeme, quoting, self.interner));
                    }
                }
                CmdTok::Redirect(text) => {
                    stdout_void = stdout_void || redirect_voids_stdout(&text);
                    awaits_report_target = redirect_awaits_target(&text);
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
                    return Err(
                        self.fail_here(PredictOutOfDialectReason::OutOfDialectToken { lex: msg })
                    );
                }
                CmdTok::Other => {
                    return Err(self.fail_here(PredictOutOfDialectReason::UnexpectedTokenInCommand));
                }
            }
        }
        if words.is_empty() {
            return Err(self.fail_here(PredictOutOfDialectReason::EmptyCommand));
        }
        // The command span ends at its last real word/redirect (EXCLUDING the trailing
        // mark), so the strip deletes exactly `[span.hi .. mark.span.hi]`. A PIPELINE (24E §14)
        // spans the whole `cmd | cmd | …` byte-exact; it carries NO mark EXCEPT in a reaches body
        // (24G §4 carve-out — there the trailing mark types the emission and IS parsed).
        let span = start_span.to(end_span);
        let carries_mark = !pipeline || self.role == FnRole::DisturbanceReachesOnly;
        let mark = if carries_mark {
            match mark_intro {
                Some(intro) => Some(self.parse_mark(intro)?),
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
            report_sink,
        }))
    }

    /// Parse ONE `281` mark starting at the intro token (`28A:rul-single-mark-production-subset`):
    /// bump the intro, head-decode the verb by the `281` §4 three-rule (sugar → the sugar's verb;
    /// else a dotted first token is an `asserts` coordinate; else a dotless first token is a verb
    /// word), then consume the ONE payload word and split it (`@` selector). A trailing bind (`:=` /
    /// `bind`) and an unknown verb word ⊤-reject (`inv-top-reject`; the block-model rc-arity and
    /// multi-mark surface stay in the reference module). Malformed ⇒ ⊤-reject — never guess.
    fn parse_mark(&mut self, intro: MarkIntro) -> Result<Mark, bool> {
        let marker_span = self.peek_span().unwrap_or(ZERO_SPAN);
        self.bump(); // the intro token (`:` / `:!` / `#:` / …)
        // Head-decode the verb. A dotless verb WORD is consumed here (rule 3); sugar (rule 1) and
        // a dotted coordinate (rule 2) leave the payload word for the split below.
        let kind = match intro.sugar {
            Some(Sugar::Bang) => MarkKind::Refutes,
            Some(Sugar::Question) => MarkKind::Reads,
            Some(Sugar::Equals) => {
                return Err(self.fail_mark(
                    intro,
                    marker_span,
                    PredictOutOfDialectReason::TrailingBindMarkWithValue,
                ));
            }
            None => {
                let head_span = self.peek_span().unwrap_or(marker_span);
                let head = match self.peek() {
                    Some(Tok::Word { lexeme, .. }) => lexeme.clone(),
                    _ => {
                        return Err(self.fail_mark(
                            intro,
                            marker_span,
                            PredictOutOfDialectReason::MarkNeedsVerbOrCoordinate,
                        ));
                    }
                };
                if head.contains('.') {
                    MarkKind::Asserts // rule 2: a dotted coordinate, the omitted-verb default
                } else {
                    self.bump(); // rule 3: consume the verb word
                    if head == "bind" {
                        return Err(self.fail_mark(
                            intro,
                            marker_span,
                            PredictOutOfDialectReason::TrailingBindMarkWord,
                        ));
                    }
                    if let Some(kind) = mark_verb(&head) {
                        kind
                    } else {
                        if intro.carrier == MarkCarrier::Hash {
                            return Err(self.fail_hashcolon(marker_span));
                        }
                        self.out.push(Diag::new(
                            DiagCode::MarkUnknownVerb(MarkUnknownVerb {
                                token: head,
                                expected: expected_verbs().to_owned(),
                            }),
                            head_span,
                        ));
                        return Err(true);
                    }
                }
            }
        };
        let Some((lexeme, _quoting, target_span)) = self.take_word() else {
            return Err(self.fail_mark(
                intro,
                marker_span,
                PredictOutOfDialectReason::MarkNeedsPayload,
            ));
        };
        let Some(parsed) = split_mark_target(&lexeme, '@') else {
            return Err(self.fail_mark(
                intro,
                marker_span,
                PredictOutOfDialectReason::MalformedMarkTarget,
            ));
        };
        // rider-selector-charset-unenforced (`277` §4b / `281` §6): a selector is a POSIX name in
        // spirit (or a brace-alternation `{a,b}`, expanded consumer-side). A violating selector is
        // a LOUD ⊤-reject, never a silent ⊤ (`inv-top-reject`).
        if let Some(prop) = &parsed.prop
            && !is_valid_selector(prop)
        {
            return Err(self.fail_mark(
                intro,
                marker_span,
                PredictOutOfDialectReason::SelectorNotPosixName,
            ));
        }
        Ok(Mark {
            kind,
            target: MarkTarget {
                kind: parsed.kind,
                entity: parsed.entity,
                prop: parsed.prop,
            },
            span: marker_span.to(target_span),
        })
    }

    fn fail_mark(
        &mut self,
        intro: MarkIntro,
        marker_span: Span,
        reason: PredictOutOfDialectReason,
    ) -> bool {
        if intro.carrier == MarkCarrier::Hash {
            self.fail_hashcolon(marker_span)
        } else {
            self.fail_here(reason)
        }
    }

    /// The `#:` carrier's own failure. A block that does not parse stays a plain comment
    /// (`strip-is-pure-erasure`), so the carrier answers for it and the out-of-dialect reason its
    /// caller was carrying never reached a reader.
    fn fail_hashcolon(&mut self, marker_span: Span) -> bool {
        self.out.push(Diag::new(
            DiagCode::MarkHashcolonMalformed(MarkHashcolonMalformed),
            marker_span,
        ));
        true
    }

    // --- words & tests ------------------------------------------------------

    /// Parse a single word token into a [`Word`].
    fn parse_word(&mut self) -> Result<Word, bool> {
        match self.take_word() {
            Some((lexeme, quoting, _span)) => {
                Ok(parse_word_lexeme(&lexeme, quoting, self.interner))
            }
            None => Err(self.fail_here(PredictOutOfDialectReason::ExpectedAWord)),
        }
    }

    /// Parse a `[ LHS OP RHS ]` test. The dialect admits only `=`/`!=` string
    /// comparisons (the flag-strip idiom). The brackets are standalone tokens.
    fn parse_bracket_test(&mut self) -> Result<Test, bool> {
        let lo = self.peek_span().unwrap_or(ZERO_SPAN);
        if !self.expect(&Tok::LBracket) {
            return Err(self.fail_here(PredictOutOfDialectReason::ExpectedLbracketToOpenTest));
        }
        let lhs = self.parse_word()?;
        let op = match self.peek() {
            Some(Tok::Word {
                lexeme,
                single_quoted: false,
                ..
            }) if lexeme == "=" => TestOp::Eq,
            Some(Tok::Word {
                lexeme,
                single_quoted: false,
                ..
            }) if lexeme == "!=" => TestOp::Ne,
            _ => {
                return Err(
                    self.fail_here(PredictOutOfDialectReason::TestOperatorNotStringComparison)
                );
            }
        };
        self.bump();
        let rhs = self.parse_word()?;
        let hi = self.peek_span().unwrap_or(lo);
        if !self.expect(&Tok::RBracket) {
            return Err(self.fail_here(PredictOutOfDialectReason::ExpectedRbracketToCloseTest));
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
    fn fail_here(&mut self, reason: PredictOutOfDialectReason) -> bool {
        let span = self.peek_span().unwrap_or_else(|| self.eof_span());
        self.out.push(out_of_dialect(span, reason));
        true
    }

    /// Emit an out-of-dialect diagnostic at a specific span.
    fn fail(&mut self, span: Span, reason: PredictOutOfDialectReason) {
        self.out.push(out_of_dialect(span, reason));
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
#[derive(Clone)]
struct PredictHeader {
    provider: Symbol,
    name_span: Span,
    /// The funcdef name as the file spells it — carried for the marks-lost backstop's diagnostic,
    /// which must name a function the author can find by grepping their own file.
    name: String,
}

/// The verbatim span of an and-or list item.
fn item_span(item: &AndOrItem) -> Span {
    match item {
        AndOrItem::Command(c) => c.span,
        AndOrItem::Test(t) => t.span,
    }
}

/// Classification of the current token inside [`Parser::parse_command`], computed
/// while borrowing `self.toks`, then matched after the borrow is released.
enum CmdTok {
    /// A statement terminator / block keyword — ends the command (not consumed).
    End,
    /// A `281` mark intro (`:`/`:!`/`:?`/`:=`/`#:`…) begins a trailing mark — ends the command
    /// (not consumed), carrying the [`MarkIntro`]. (A statement-leading intro is instead handled
    /// by synthesizing a `:` host command; see [`Parser::parse_command`].)
    MarkStart(MarkIntro),
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
    /// An out-of-dialect token (carries the lexer's typed reason).
    Error(PredictLexError),
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
                matches!(tok, Tok::Word { lexeme, single_quoted: false, .. } if lexeme == kw)
            }
            BlockEnd::CaseArmEnd => {
                matches!(tok, Tok::DSemi)
                    || matches!(tok, Tok::Word { lexeme, single_quoted: false, .. } if lexeme == "esac")
            }
            BlockEnd::IfThenEnd => {
                matches!(tok, Tok::Word { lexeme, single_quoted: false, .. }
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
    let reason = match end {
        BlockEnd::Brace => PredictUnterminatedReason::FunctionBody,
        BlockEnd::Keyword(keyword) => PredictUnterminatedReason::Block { keyword },
        BlockEnd::CaseArmEnd => PredictUnterminatedReason::CaseArm,
        BlockEnd::IfThenEnd => PredictUnterminatedReason::IfThen,
    };
    p.out.push(unterminated(span, reason));
    true
}

const ZERO_SPAN: Span = Span {
    lo: dorc_core::BytePos(0),
    hi: dorc_core::BytePos(0),
};

// === word-lexeme decoding ===================================================

/// The whole-word quoting of a lexed token — whether it was exactly one single- or
/// double-quoted run (`273`/`27H`). `single_quoted` makes `$`/`#` literal; `double_quoted`
/// is consulted ONLY for `"$@"` vs bare `$@` (the faithful list-form vs word-splitting), the
/// oracle-side positional model's one quoting-sensitive decision. A word mixing quotes or
/// bare bytes is neither.
#[derive(Debug, Clone, Copy)]
struct WordQuoting {
    single_quoted: bool,
    double_quoted: bool,
}

impl WordQuoting {
    /// Neither quote — a bare word or a sub-lexeme where whole-word quoting cannot apply.
    const fn unquoted() -> Self {
        Self {
            single_quoted: false,
            double_quoted: false,
        }
    }
}

/// Decode a lexer word lexeme into a [`Word`]. `single_quoted` ⇒ the whole token
/// was single-quoted, so `$`/`#` are literal (`'$1'` ⇒ the literal string `$1`).
/// `double_quoted` is the `"$@"`-vs-`$@` discriminator (`273`/`27H`).
fn parse_word_lexeme(lexeme: &str, quoting: WordQuoting, interner: &mut Interner) -> Word {
    if quoting.single_quoted {
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
        // `"$@"` — the faithful positional list (`273` §1: command-position `"$@"` runs the
        // argument-slot ⇒ peel). ONLY the double-quoted form models: bare `$@` word-splits,
        // and `$*`/`"$*"` IFS-join, so none of the three preserve the argument list — they
        // route to `Unmodeled` ⇒ ⊤ in every position (`27H` bare-forms-route-to-top,
        // `271:rul-env-claim-inversion`: bare `"$@"` claims NOTHING). This is the
        // wrong-`Word::Literal("$@")` fix (`27H` finding-positional-oracle-side-couples-founding-pin):
        // the old literal resolved to the text `$@`, a wrong concrete.
        if rest == "@" && quoting.double_quoted {
            return Word::PositionalArgs;
        }
        // `$@`, `$*`, `"$*"`, `$#`, `$?` and the like: not a single resolvable value and not
        // the faithful list ⇒ `Unmodeled`, which fails to resolve in EVERY position (the safe
        // direction; NOT `Literal`, which would evaluate as its own text — a wrong concrete).
        return Word::Unmodeled(lexeme.to_owned());
    }
    // A bare literal. If a `$` appears mid-word (`pre$1`), we conservatively keep
    // the whole thing literal — the dialect's resolvable words are simple `$N`/
    // `$name`/`"$N"`, and a mixed word degrades to a non-matching literal ⇒ Top.
    Word::Literal(lexeme.to_owned())
}

/// The two `281` §1 mark carriers: the salient default `:` and the inert comment `#:`. The
/// production single-mark subset (`28A:rul-single-mark-production-subset`) parses both — a valid
/// `#:` block behaves identically to its `:` twin; the carrier only changes strip/off-ramp behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkCarrier {
    /// `:` — the salient colon form (highlights as shell; corrupts under a raw run).
    Colon,
    /// `#:` — the inert hash-colon comment carrier.
    Hash,
}

/// The head-only sugar characters (`281` §3): the core cell-and-value shortcuts (`!`=refutes,
/// `?`=reads, `=`=bind). Meta verbs are always spelled as words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sugar {
    /// `!` — the complement verdict (`refutes`).
    Bang,
    /// `?` — the read (`reads`).
    Question,
    /// `=` — the bind (`281` §8).
    Equals,
}

/// A decoded mark intro `( ':' | '#:' ) [ SUGAR ]` (`281` §3): the carrier plus optional head
/// sugar. Replaces the old `MarkSigil` — the head-decode (`parse_mark`) turns it into one
/// [`MarkKind`] verb via the `281` §4 three-rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MarkIntro {
    carrier: MarkCarrier,
    sugar: Option<Sugar>,
}

/// Decode a lexeme to a `281` mark intro (`281` §3), or `None` if it is not one of the eight
/// legal intros (`:` `:!` `:?` `:=` `#:` `#:!` `#:?` `#:=`). The space-flanked intro lexes as its
/// own word; the payload (verb / coordinate) follows as adjacent words. (Production-local twin of
/// [`super::mark_grammar::decode_intro`]; DUPLICATED, not shared, so production stays independent
/// of the `#[cfg(test)]` reference module — `28A` CP-D handoff.)
fn mark_marker(lexeme: &str) -> Option<MarkIntro> {
    let (carrier, rest) = match lexeme.strip_prefix("#:") {
        Some(rest) => (MarkCarrier::Hash, rest),
        None => (MarkCarrier::Colon, lexeme.strip_prefix(':')?),
    };
    let sugar = match rest {
        "" => None,
        "!" => Some(Sugar::Bang),
        "?" => Some(Sugar::Question),
        "=" => Some(Sugar::Equals),
        _ => return None,
    };
    Some(MarkIntro { carrier, sugar })
}

/// The engine-owned mark verb vocabulary (`281` §5), closed at v0.2, extends by new name only.
/// `Some(kind)` maps a verb WORD to its [`MarkKind`]; `bind` is the value-plane verb with no
/// coordinate `MarkKind` (`281` §8) and is handled at the call site (⊤-reject in the production
/// subset). (Twin of the reference module's `VERBS`; DUPLICATED per the CP-D handoff — the
/// vocabulary is closed, so the two-table drift risk is bounded and both are unit-tested.)
fn mark_verb(word: &str) -> Option<MarkKind> {
    Some(match word {
        "asserts" => MarkKind::Asserts,
        "refutes" => MarkKind::Refutes,
        "reads" => MarkKind::Reads,
        "safe-across" => MarkKind::SafeAcross,
        "disturbs" => MarkKind::Disturbs,
        "lends" => MarkKind::Lends,
        "stored-in" => MarkKind::StoredIn,
        "undivided-by-transit-across" => MarkKind::Undivided,
        _ => return None,
    })
}

/// The comma-joined known-verb vocabulary, for the unknown-verb ⊤-reject diagnostic (`281` §4).
fn expected_verbs() -> &'static str {
    "asserts, refutes, reads, bind, safe-across, disturbs, lends, stored-in, \
     undivided-by-transit-across"
}

/// Validate production AST marks, never raw physical lines.
fn validate_mark_subset(stmts: &[Stmt], toks: &[Token]) -> Vec<Diag> {
    let mut diags = Vec::new();
    validate_mark_block(stmts, toks, &mut diags);
    diags
}

/// Validate top-level fragments through the production parser.
#[must_use]
pub(crate) fn lint_mark_subset(src: &str) -> Vec<Diag> {
    let tokens = lex(src);
    let mut interner = Interner::default();
    let mut parser = Parser {
        toks: &tokens,
        pos: 0,
        interner: &mut interner,
        out: Carrier::pure(PredictSet::default()),
        last_term: None,
        role: FnRole::Predict,
    };
    let body = parser.parse_mark_validation_body();
    parser
        .out
        .diags
        .extend(validate_mark_subset(&body, parser.toks));
    parser.out.diags
}

fn validate_mark_block(stmts: &[Stmt], toks: &[Token], diags: &mut Vec<Diag>) {
    let mut prior_trailing_rc = false;
    for stmt in stmts {
        match stmt {
            Stmt::Command(command) => {
                let Some(mark) = &command.mark else {
                    prior_trailing_rc = false;
                    continue;
                };
                let rc_consumer = matches!(mark.kind, MarkKind::Asserts | MarkKind::Refutes);
                let standalone_consumer = rc_consumer || mark.kind == MarkKind::Reads;
                let standalone = command.words.len() == 1
                    && matches!(command.words.first(), Some(Word::Literal(word)) if word == ":");
                if standalone && standalone_consumer {
                    let code = if prior_trailing_rc && rc_consumer {
                        DiagCode::MarkRcArityExceeded(MarkRcArityExceeded)
                    } else {
                        DiagCode::MarkStandaloneRcConsumer(MarkStandaloneRcConsumer)
                    };
                    diags.push(Diag::new(code, mark_target_span(mark, toks)));
                }
                prior_trailing_rc = !standalone && rc_consumer;
            }
            Stmt::Case { arms, .. } => {
                for arm in arms {
                    validate_mark_block(&arm.body, toks, diags);
                }
                prior_trailing_rc = false;
            }
            Stmt::If {
                then_body,
                else_body,
                ..
            } => {
                validate_mark_block(then_body, toks, diags);
                validate_mark_block(else_body, toks, diags);
                prior_trailing_rc = false;
            }
            Stmt::While { body, .. } => {
                validate_mark_block(body, toks, diags);
                prior_trailing_rc = false;
            }
            _ => prior_trailing_rc = false,
        }
    }
}

/// Strip needs the carrier span; diagnostics point at the payload.
fn mark_target_span(mark: &Mark, toks: &[Token]) -> Span {
    toks.iter()
        .find(|token| matches!(token.kind, Tok::Word { .. }) && token.span.hi == mark.span.hi)
        .map_or(mark.span, |token| token.span)
}

/// A syntactically-split mark target — every fragment OPAQUE (`inv-referent-agnostic`,
/// never decoded). See [`split_mark_target`].
pub(crate) struct ParsedTarget {
    pub(crate) kind: String,
    pub(crate) entity: Option<String>,
    pub(crate) prop: Option<String>,
}

/// Split a mark target lexeme `kind[:entity[<sel>selector]]` into its opaque fragments
/// (`277` §4a / `281` §6): split the body on the FIRST `:` (kind ⟂ rest) and the rest on the
/// FIRST selector-introducer `sel` (entity ⟂ selector). `sel` is `#` for the OLD grammar and
/// `@` for the `281` respell (`281` §R4) — the only difference between the two, so ONE impl
/// serves both. `None` if empty or kind-less (⊤-reject upstream). The kind fragment keeps its
/// reverse-DNS dots; `.` no longer introduces anything in coordinate position (the `.prop`
/// production is dead — `277` §4a). An empty entity before `sel` is the empty-entity
/// transitional form `kind:<sel>selector` (`entity = Some("")`, a real value, not absence —
/// `inv-referent-agnostic`: empty ≠ None). No fragment is interpreted.
pub(crate) fn split_mark_target(lexeme: &str, sel: char) -> Option<ParsedTarget> {
    if lexeme.is_empty() {
        return None;
    }
    let Some((kind, rest)) = lexeme.split_once(':') else {
        // No `:` — the emission form `KIND<sel>SELECTOR` (the selector rides the mark; the entity
        // comes from the printf line — `rul-emission-selector-on-mark`) or a bare `KIND` (`277` §4a).
        // The selector splits off the FIRST `sel`; the kind keeps its reverse-DNS dots (no `sel` is a
        // valid kind char). Entity is `None` (no entity in the mark).
        return match lexeme.split_once(sel) {
            Some((kind, s)) if !kind.is_empty() && !s.is_empty() => Some(ParsedTarget {
                kind: kind.to_owned(),
                entity: None,
                prop: Some(s.to_owned()),
            }),
            // `KIND<sel>` (empty selector) or `<sel>…` (empty kind) — no clean split ⇒ bare kind (a
            // trailing `sel` stays in the kind, caught by the kind-charset differential; the corpus
            // never does this).
            _ => Some(ParsedTarget {
                kind: lexeme.to_owned(),
                entity: None,
                prop: None,
            }),
        };
    };
    if kind.is_empty() {
        return None;
    }
    let (entity, prop) = match rest {
        "" => (None, None),
        // The entity/selector split is on the FIRST `sel` (the attached selector-introducer,
        // `277` §4a / `281` §6). No `sel` ⇒ a whole-entity coordinate. A leading `sel` ⇒ the
        // empty-entity form `kind:<sel>selector`.
        r => match r.split_once(sel) {
            Some((e, s)) if !s.is_empty() => (Some(e.to_owned()), Some(s.to_owned())),
            // `kind:entity<sel>` (empty selector) is malformed — treat the whole rest as the
            // entity (the selector is dropped; the differential gate catches a mis-spell).
            _ => (Some(r.to_owned()), None),
        },
    };
    Some(ParsedTarget {
        kind: kind.to_owned(),
        entity,
        prop,
    })
}

/// Is `sel` a valid selector (`277` §4b/§4c): a POSIX name (letter/underscore first, then
/// letters/digits/underscores), or a brace-alternation `{tok,tok[,…]}` where every token is a
/// POSIX name (`rider-selector-charset-unenforced`). The role-scoping of brace-alternation
/// (claim-emission marks only; verdict/observe stay single-cell — `277` §4c) is enforced by the
/// consumers (`derive_predict`), not here: the parser is role-agnostic (`:` serves both verdict and
/// disturbs marks), so it accepts the brace SHAPE and leaves the single-cell rejection to the
/// role-aware lift.
pub(crate) fn is_valid_selector(sel: &str) -> bool {
    sem::is_name(sel) || is_brace_alternation(sel)
}

/// Is `sel` a well-formed brace-alternation `{tok,tok[,…]}` (`277` §4c)?
pub(crate) fn is_brace_alternation(sel: &str) -> bool {
    brace_tokens(sel).is_some()
}

/// The tokens of a brace-alternation selector `{tok,tok[,…]}` (`277` §4c): `Some` iff braces, ≥2
/// comma-separated POSIX-name tokens, no internal whitespace; `None` for a plain selector. Opaque
/// (never decoded). Shared with the touches lift (the claim-emission expansion) and the predict
/// derive (the verdict/observe single-cell rejection).
pub(crate) fn brace_tokens(sel: &str) -> Option<Vec<String>> {
    let inner = sel.strip_prefix('{')?.strip_suffix('}')?;
    let tokens: Vec<&str> = inner.split(',').collect();
    (tokens.len() >= 2 && tokens.iter().all(|t| sem::is_name(t)))
        .then(|| tokens.into_iter().map(str::to_owned).collect())
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

/// The engine-owned recognized report-sink variable names (`27W:rul-versioned-entry`). A new
/// report format mints a NEW name and APPENDS it here (the `__role`-name posture: recognized
/// names are permanent once published, and adding one is a list append, never surgery —
/// `report-lane-versioned-entry`, `tc-sink-recognition-mechanism`).
const REPORT_SINK_NAMES: &[&str] = &["DREP_V1"];

/// Is `chunk` an OUTPUT redirect operator (`>`/`>>`, optional fd digits) with NO inline target —
/// so its target is the FOLLOWING word (the quoted-sink case: the lexer stops the redirect chunk
/// at the opening `"` of `>>"${DREP_V1:-…}"`)? Distinguishes `>>` (awaits a target word) from
/// `>/dev/null` (inline target) and `>&2` (fd dup, no file target).
fn redirect_awaits_target(chunk: &str) -> bool {
    let op = chunk.trim_start_matches(|c: char| c.is_ascii_digit());
    matches!(op, ">" | ">>")
}

/// Does a redirect-target word reference a recognized report sink (`27W` §2)? Extracts the
/// referenced variable NAME from `${NAME:-def}` / `${NAME-def}` / `${NAME}` / `$NAME` (the
/// `:-/dev/null`-default idiom that makes the emission total off-Dorc) and matches it against
/// [`REPORT_SINK_NAMES`]. A dynamic / non-sink target ⇒ `false` (⇒ tier-3 runtime fallback).
fn word_is_report_sink(lexeme: &str) -> bool {
    let inner = match lexeme.strip_prefix("${").and_then(|s| s.strip_suffix('}')) {
        Some(braced) => braced,
        None => lexeme.strip_prefix('$').unwrap_or(lexeme),
    };
    let name = inner
        .split_once(":-")
        .or_else(|| inner.split_once('-'))
        .map_or(inner, |(n, _)| n);
    REPORT_SINK_NAMES.contains(&name)
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
    use super::{AndOrOp, Interner, Mark, MarkKind, Stmt, Word, lift_predicts};

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

    /// Every and-or shape ACCEPTS at parse and degrades at trace — never a lift failure. Load-
    /// bearing, not stylistic: the corpus's one and-or list lives in a `sm_dorc_Package__resolve()`
    /// body, and a resolver is host-run strip-only, so a parse rejection would delete a working
    /// resolver over a construct nothing traces.
    #[test]
    fn every_and_or_shape_lifts_rather_than_killing_the_funcdef() {
        for body in [
            "dpkg-query -W -f '${Package}\\n' -- \"$1\" 2>/dev/null || printf '%s\\n' \"$1\"",
            "w precheck && w probe",
            "w precheck & w probe",
            "w a || w b || w c",
        ] {
            let mut i = Interner::default();
            let out = lift_predicts(&mut i, &format!("w__predict() {{ {body} }}"));
            assert!(out.diags.is_empty(), "{body} lifts clean: {:?}", out.diags);
            assert_eq!(out.value.len(), 1, "{body} keeps its funcdef");
        }
    }

    /// An and-or list parses as ONE statement holding its items — not as N statements, and not as
    /// one command with the operator folded in as a WORD (which is how `&&` used to scan).
    #[test]
    fn an_and_or_list_is_one_statement_holding_its_items() {
        let body = body_of("w__predict() { w precheck && shift }");
        assert_eq!(body.len(), 1, "one statement: {body:?}");
        let Stmt::AndOr(list) = &body[0] else {
            panic!("an and-or list: {body:?}");
        };
        assert_eq!(list.rest.len(), 1);
        assert_eq!(list.rest[0].op, AndOrOp::AndThen);
        assert_eq!(list.commands().count(), 2, "both items are visible");
        for cmd in list.commands() {
            assert!(
                !cmd.words
                    .iter()
                    .any(|w| matches!(w, Word::Literal(l) if l == "&&" || l == "||" || l == "&")),
                "no operator survives as a command word: {cmd:?}"
            );
        }
    }

    /// Making `&` a metacharacter must not disturb the redirect forms that carry one: `redirect()`
    /// consumes their `&` before the word arm ever sees the byte.
    #[test]
    fn ampersand_bearing_redirects_are_untouched() {
        for body in [
            "w q \"$1\" >/dev/null 2>&1",
            "w q \"$1\" 2>&1",
            "w q \"$1\" >&2",
        ] {
            let stmts = body_of(&format!("w__predict() {{ {body} ;}}"));
            assert_eq!(stmts.len(), 1, "{body} is one command: {stmts:?}");
            assert!(
                matches!(&stmts[0], Stmt::Command(_)),
                "{body} stays a plain command, not a list: {stmts:?}"
            );
        }
    }

    /// A TRAILING `&` (`cmd &`, backgrounding with nothing after it) has no right-hand item, so it
    /// is a lift failure rather than a list — loud, and funcdef-wide. It used to scan as a stray
    /// `&` WORD appended to the command's argv, which is the mis-modelling this lexing closes; a
    /// lift failure is the ⊤-ward and louder direction, and the shape is absent from the corpus.
    #[test]
    fn a_trailing_background_operator_is_a_loud_lift_failure() {
        let mut i = Interner::default();
        let out = lift_predicts(&mut i, "w__predict() {\n   w precheck &\n}\n");
        assert!(!out.diags.is_empty(), "the give-up is loud");
        assert!(out.value.is_empty(), "the funcdef does not lift");
    }

    /// A quoted `&` is ordinary word text — quoting is resolved before the metacharacter set
    /// applies, so a printf format keeps its ampersand.
    #[test]
    fn a_quoted_ampersand_stays_inside_its_word() {
        let stmts = body_of("w__predict() { printf 'a & b\\n' }");
        let Stmt::Command(c) = &stmts[0] else {
            panic!("a plain command: {stmts:?}");
        };
        assert!(
            matches!(c.words.get(1), Some(Word::SingleQuotedLiteral(s)) if s.contains('&')),
            "the quoted `&` rides its word: {:?}",
            c.words
        );
    }

    /// A mark on a list item is REFUSED — loudly, and off the item, so no mark-consumer can reach
    /// it. See `AndOr::refused_marks` for why a list may not carry one.
    #[test]
    fn a_mark_on_an_and_or_item_is_refused_loudly() {
        let mut i = Interner::default();
        let out = lift_predicts(
            &mut i,
            "w__predict() {\n   thing : sm.dorc.Thing = \"$1\"\n   w q \"$thing\" : sm.dorc.Thing:\"$thing\"@present || return 2\n}",
        );
        assert_eq!(out.diags.len(), 1, "exactly one complaint: {:?}", out.diags);
        assert_eq!(out.diags[0].code.slug(), "mark-on-and-or-list");
        assert_eq!(out.diags[0].severity(), dorc_aid::Severity::Warning);
        let check = out
            .value
            .get(i.intern("w"))
            .expect("the funcdef still lifts");
        let Some(Stmt::AndOr(list)) = check.body.last() else {
            panic!("the list is the last statement: {:?}", check.body);
        };
        assert_eq!(
            list.refused_marks.len(),
            1,
            "the mark is held for the strip"
        );
        assert!(
            list.commands().all(|c| c.mark.is_none()),
            "no item keeps the mark"
        );
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
        // `dpkg-query … : sm.dorc.Package:"$pkg"@installed` — a trailing verdict mark. kind keeps
        // its reverse-DNS dots; entity/selector split on the attached `#` (`277` §4a).
        let body = body_of(
            "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\" : sm.dorc.Package:\"$pkg\"@installed; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Asserts);
        assert_eq!(m.target.kind, "sm.dorc.Package");
        assert_eq!(m.target.entity.as_deref(), Some("$pkg"));
        assert_eq!(m.target.prop.as_deref(), Some("installed"));
    }

    #[test]
    fn inverted_sigil_is_establish_inverted() {
        // `… :! sm.dorc.Package:"$pkg"@installed` — polarity rides the `:!` sigil now, never a
        // coordinate suffix (`277` §4a). The coordinate is a pure name.
        let body = body_of(
            "apt_get__predict() { pkg : sm.dorc.Package = \"$1\"; dpkg-query -W \"$pkg\" :! sm.dorc.Package:\"$pkg\"@installed; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Refutes);
        assert_eq!(m.target.prop.as_deref(), Some("installed"));
    }

    #[test]
    fn observe_sigil_is_observe() {
        // `… :? sm.dorc.GrepMatch:"$pat"@matched` — the observe mark (read-only depends-upon).
        let body = body_of(
            "grep__predict() { pat : sm.dorc.GrepMatch = \"$1\"; grep -q -- \"$pat\" :? sm.dorc.GrepMatch:\"$pat\"@matched; }",
        );
        let m = first_command_mark(&body).expect("a trailing mark");
        assert_eq!(m.kind, MarkKind::Reads);
        assert_eq!(m.target.kind, "sm.dorc.GrepMatch");
        assert_eq!(m.target.prop.as_deref(), Some("matched"));
    }

    #[test]
    fn empty_entity_form_carries_an_explicit_empty_entity() {
        // `… :? io.opentelemetry.Collector:@v0155` — the empty-entity transitional form `kind:@sel`
        // (`281` §6): the entity slot is a DELIBERATE empty string (the-one), never `None`.
        let body = body_of(
            "otelcol__predict() { case $1 in --version) otelcol --version >/dev/null 2>&1 :? io.opentelemetry.Collector:@v0155 ;; esac }",
        );
        let m = first_command_mark(&body).expect("an empty-entity observe mark");
        assert_eq!(m.kind, MarkKind::Reads);
        assert_eq!(m.target.kind, "io.opentelemetry.Collector");
        assert_eq!(
            m.target.entity.as_deref(),
            Some(""),
            "the entity slot is an EXPLICIT empty string, never None"
        );
        assert_eq!(m.target.prop.as_deref(), Some("v0155"));
    }

    #[test]
    fn non_name_selector_is_a_loud_reject() {
        // rider-selector-charset-unenforced (`277` §4b): a selector that is not a POSIX name (nor a
        // brace-alternation) is a LOUD ⊤-reject diagnostic, never a silent ⊤.
        let mut i = Interner::default();
        let out = lift_predicts(
            &mut i,
            "x__predict() { case $1 in a) foo : sm.dorc.K:e@bad-sel ;; esac }",
        );
        assert!(
            !out.diags.is_empty(),
            "a non-name selector must be diagnosed, not silently accepted as opaque"
        );
    }

    #[test]
    fn brace_alternation_selector_parses_as_a_valid_selector() {
        // `277` §4c: the brace-alternation SHAPE is a valid selector (the role-scoping to
        // claim-emission marks is enforced downstream). Here the mark's selector holds the raw
        // `{enabled,active}` (the touches lift expands it; verdict/observe reject it in derive).
        let body = body_of(
            "x__predict() { case $1 in a) foo : sm.dorc.Service:nginx@{enabled,active} ;; esac }",
        );
        let m = first_command_mark(&body).expect("a brace-alternation mark parses");
        assert_eq!(m.target.prop.as_deref(), Some("{enabled,active}"));
    }

    #[test]
    fn bare_kind_emission_mark_has_no_entity_or_selector() {
        // `printf '%s\n' "$1" : sm.dorc.Package` — the disturbs()/reaches() emission shape: the
        // kind rides the trailing mark, no entity, no selector.
        let body = body_of("apt_get__predict() { printf '%s\\n' \"$1\" : sm.dorc.Package; }");
        let m = first_command_mark(&body).expect("a bare-kind emission mark");
        assert_eq!(m.kind, MarkKind::Asserts);
        assert_eq!(m.target.kind, "sm.dorc.Package");
        assert_eq!(m.target.entity, None);
        assert_eq!(m.target.prop, None);
    }

    #[test]
    fn colon_line_is_a_command_carrying_a_trailing_token_mark() {
        // The `state_stored_only_in` colon-line shape (`281` §4/§11): a statement-leading intro
        // synthesizes a `:` no-op host COMMAND carrying the mark. A trailing `: stored-in fs`
        // substrate mark and a leading `: undivided-by-transit-across user` axis line both parse
        // this way, inert.
        let body = body_of(
            "dpkg__predict() { printf '/var/lib/dpkg\\n' : stored-in fs; : undivided-by-transit-across user; }",
        );
        // First stmt: `printf … : stored-in fs`.
        let Some(Stmt::Command(first)) = body.first() else {
            panic!("expected a command: {body:?}");
        };
        let m = first
            .mark
            .as_ref()
            .expect("the `: stored-in fs` substrate mark");
        assert_eq!(m.kind, MarkKind::StoredIn);
        assert_eq!(m.target.kind, "fs");
        // Second stmt: the synthetic colon-command `:` hosting `: undivided-by-transit-across user`
        // (the axis token in the uniform `.kind` payload home).
        let Some(Stmt::Command(second)) = body.get(1) else {
            panic!("expected a colon-command: {body:?}");
        };
        let m2 = second
            .mark
            .as_ref()
            .expect("the invariance colon-line mark");
        assert_eq!(m2.kind, MarkKind::Undivided);
        assert_eq!(m2.target.kind, "user");
        assert_eq!(m2.target.entity, None);
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
    fn positional_default_nounset_idiom_parses() {
        // `${2-}` (the nounset idiom, `24P` §2) parses as PositionalDefault, NOT Unmodeled — so
        // `[ "${2-}" = "" ]` resolves the operand-count guard (the site would be un-probeable if it
        // degraded to ⊤). Adversarial: a non-empty default and the `:-` spelling both parse.
        use super::{Word, WordQuoting, parse_word_lexeme};
        let mut i = Interner::default();
        assert_eq!(
            parse_word_lexeme("${2-}", WordQuoting::unquoted(), &mut i),
            Word::PositionalDefault {
                n: 2,
                default: String::new()
            }
        );
        assert_eq!(
            parse_word_lexeme("${1:-def}", WordQuoting::unquoted(), &mut i),
            Word::PositionalDefault {
                n: 1,
                default: "def".to_owned()
            }
        );
    }

    #[test]
    fn positional_args_only_quoted_at_models_the_list() {
        // The oracle-side positional model (`273`/`27H`): ONLY `"$@"` (double-quoted) is the
        // faithful list-form `Word::PositionalArgs`. bare `$@`, `$*`, and `"$*"` route to
        // `Word::Unmodeled` (⊤ everywhere — they word-split / IFS-join, not the arg list). This
        // is the `27H` finding-positional-oracle-side-couples-founding-pin fix: the old
        // `Word::Literal("$@")` was a wrong concrete.
        use super::{Word, WordQuoting, parse_word_lexeme};
        let dq = WordQuoting {
            single_quoted: false,
            double_quoted: true,
        };
        let bare = WordQuoting::unquoted();
        let mut i = Interner::default();
        assert_eq!(
            parse_word_lexeme("$@", dq, &mut i),
            Word::PositionalArgs,
            "quoted `\"$@\"` is the faithful positional list"
        );
        assert_eq!(
            parse_word_lexeme("$@", bare, &mut i),
            Word::Unmodeled("$@".to_owned()),
            "bare `$@` word-splits ⇒ ⊤ (not the list)"
        );
        assert_eq!(
            parse_word_lexeme("$*", dq, &mut i),
            Word::Unmodeled("$*".to_owned()),
            "`\"$*\"` IFS-joins ⇒ ⊤ (not the list)"
        );
        assert_eq!(
            parse_word_lexeme("$*", bare, &mut i),
            Word::Unmodeled("$*".to_owned()),
            "bare `$*` ⇒ ⊤"
        );
        // A single positional is quoting-insensitive (both `$1` and `"$1"` ⇒ Positional(1)).
        assert_eq!(parse_word_lexeme("$1", dq, &mut i), Word::Positional(1));
        assert_eq!(parse_word_lexeme("$1", bare, &mut i), Word::Positional(1));
    }

    #[test]
    fn positional_args_lexes_with_double_quote_flag() {
        // The lexer must PRESERVE the whole-word double-quote so the parser can tell `"$@"`
        // from `$@` (the flag is otherwise decoded away). A command `mycmd "$@"` ⇒ the second
        // word is `Word::PositionalArgs`; `mycmd $@` ⇒ `Word::Unmodeled`.
        use super::Word;
        let quoted = body_of("w__predict() { mycmd \"$@\"; }");
        let bare = body_of("w__predict() { mycmd $@; }");
        let last_word = |body: &[Stmt]| -> Word {
            match body.last().expect("a command") {
                Stmt::Command(c) => c.words.last().expect("a word").clone(),
                other => panic!("expected a command, got {other:?}"),
            }
        };
        assert_eq!(last_word(&quoted), Word::PositionalArgs);
        assert_eq!(last_word(&bare), Word::Unmodeled("$@".to_owned()));
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
            "sm_dorc_Package__disturbance_reaches_only() { dpkg -L \"$1\" | grep x | sed y : disturbs sm.dorc.File ; }",
        );
        assert!(c.pipeline, "a multi-stage pipe is a pipeline");
        let m = c
            .mark
            .expect("the reaches pipeline carries its trailing mark (the carve-out)");
        assert_eq!(m.kind, MarkKind::Disturbs);
        assert_eq!(m.target.kind, "sm.dorc.File");
    }

    /// A `:` INSIDE a quoted pipeline arg (`'s|x|file:|'`) is NOT a mark — a single-quoted token
    /// lexes as one opaque word, so the inner `:` stays UNTOUCHED and the mark is the TRAILING one.
    #[test]
    fn reaches_pipeline_quoted_colon_inside_arg_is_not_a_mark() {
        let c = reaches_command(
            "sm_dorc_Package__disturbance_reaches_only() { dpkg -L \"$1\" | sed 's|x|file:|' : disturbs sm.dorc.File ; }",
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
