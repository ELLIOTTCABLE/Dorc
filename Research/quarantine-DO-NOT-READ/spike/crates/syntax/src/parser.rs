//! Recursive-descent parser for the modeled sh subset. Token stream in (from
//! [`crate::lexer`]), arena [`Ast`] + diagnostics out. Total and pure
//! (`inv-no-throw`, `inv-determinism`): malformed or unmodeled input becomes a
//! [`NodeKind::Unsupported`] node plus an `Error` diagnostic and parsing
//! continues, so the carrier surfaces *unrelated* problems too (`dn-7`).
//!
//! Grammar (grown demand-driven to the `pi-webhost` fixture + the three oracle
//! idioms), lowest-to-highest precedence:
//!
//! ```text
//! script      := (and_or (sep))*
//! and_or      := pipeline (('&&' | '||') pipeline)*        (left-assoc)
//! pipeline    := ['!'] command ('|' command)*
//! command     := if | case | subshell | group | funcdef | simple
//! simple      := (assign)* (word)* (redir)*
//! ```
//!
//! Reserved words are recognised **positionally** (`inv` note in lexer): a bare
//! `Word` of a single literal equal to `if`/`then`/`case`/… is a keyword only when
//! it appears where the grammar expects a command to start. `echo if` keeps `if`
//! as an argument because it is not in command position.

use dorc_core::{BytePos, Carrier, DiagCode, Diagnostic, Span};

use crate::ast::{
    AndOrOp, Ast, AstBuilder, CaseArm, ElseIf, Node, NodeKind, RedirOp, RedirTarget,
    UnsupportedReason, WordPart,
};
use crate::lexer::{lex, LexPart, RedirToken, TokKind, Token};

/// Diagnostic codes this parser emits (kept greppable; `ch-catalog`).
const UNSUPPORTED: DiagCode = DiagCode("syntax-unsupported");
const MALFORMED: DiagCode = DiagCode("syntax-malformed");

/// Parse sh `src` into an arena AST + diagnostics. The single public entry of the
/// crate's parser (see [`crate::parse`]).
pub(crate) fn parse(src: &str) -> Carrier<Ast> {
    let tokens = lex(src);
    let src_len = u32::try_from(src.len()).unwrap_or(u32::MAX);
    let mut parser = Parser {
        tokens,
        cursor: 0,
        builder: AstBuilder::default(),
        diags: Vec::new(),
        depth: 0,
    };
    let items = parser.parse_command_list(&[]);
    let root = parser.builder.alloc(Node {
        span: Span::new(BytePos(0), BytePos(src_len)),
        kind: NodeKind::Script { items },
    });
    let ast = parser.builder.finish(root);
    Carrier::new(ast, parser.diags)
}

/// Reserved words that, in command position, change how the following tokens are
/// parsed. `for`/`while`/`until` are recognised here so they can be ⊤-rejected as
/// loops (rather than mis-parsed as simple commands named "for").
fn reserved_word(s: &str) -> Option<Reserved> {
    Some(match s {
        "if" => Reserved::If,
        "then" => Reserved::Then,
        "elif" => Reserved::Elif,
        "else" => Reserved::Else,
        "fi" => Reserved::Fi,
        "case" => Reserved::Case,
        "esac" => Reserved::Esac,
        "in" => Reserved::In,
        "for" => Reserved::For,
        "while" => Reserved::While,
        "until" => Reserved::Until,
        "do" => Reserved::Do,
        "done" => Reserved::Done,
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reserved {
    If,
    Then,
    Elif,
    Else,
    Fi,
    Case,
    Esac,
    In,
    For,
    While,
    Until,
    Do,
    Done,
}

/// Recursion-depth bound (`inv-no-throw`): every nested compound command and every
/// command-substitution body descends one level through [`Parser::parse_command`].
/// Hostile input (`(((((…`, `$( $( $( …`) could otherwise blow the native stack —
/// a panic-equivalent. Past this depth we ⊤-reject the over-nested construct and
/// stop descending. 256 is far beyond any real book; deep enough never to clip
/// legitimate scripts, shallow enough to stay well inside the default stack.
const MAX_DEPTH: u32 = 256;

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    builder: AstBuilder,
    diags: Vec<Diagnostic>,
    /// Current nesting depth (see [`MAX_DEPTH`]).
    depth: u32,
}

impl Parser {
    // ---- token cursor helpers -------------------------------------------------

    fn peek(&self) -> &TokKind {
        // The lexer always terminates with Eof, so this never indexes past end.
        &self.tokens[self.cursor.min(self.tokens.len() - 1)].kind
    }

    fn peek_span(&self) -> Span {
        self.tokens[self.cursor.min(self.tokens.len() - 1)].span
    }

    fn at_eof(&self) -> bool {
        matches!(self.peek(), TokKind::Eof)
    }

    fn bump(&mut self) -> Token {
        let tok = self.tokens[self.cursor.min(self.tokens.len() - 1)].clone();
        if self.cursor < self.tokens.len() {
            self.cursor += 1;
        }
        tok
    }

    /// Skip newlines and `;` separators (used between list items and inside
    /// compound bodies where blank lines are allowed).
    fn skip_separators(&mut self) {
        while matches!(self.peek(), TokKind::Newline | TokKind::Semi) {
            self.cursor += 1;
        }
    }

    /// If the current token is a bare reserved word, return which one.
    fn peek_reserved(&self) -> Option<Reserved> {
        match self.peek() {
            TokKind::Word { parts } => single_literal(parts).and_then(reserved_word),
            _ => None,
        }
    }

    fn push_error(&mut self, code: DiagCode, span: Span, msg: impl Into<String>) {
        self.diags.push(Diagnostic::error(code, Some(span), msg));
    }

    /// Allocate an `Unsupported` ⊤-node and emit the paired `Error` diagnostic
    /// (`inv-top-reject`: loud, never silent). `salvaged` keeps any sub-nodes we
    /// still parsed so unrelated analysis can proceed.
    fn unsupported(
        &mut self,
        reason: UnsupportedReason,
        span: Span,
        salvaged: Vec<dorc_core::AstId>,
        msg: impl Into<String>,
    ) -> dorc_core::AstId {
        self.push_error(UNSUPPORTED, span, msg);
        self.builder.alloc(Node {
            span,
            kind: NodeKind::Unsupported { reason, salvaged },
        })
    }

    // ---- list / script --------------------------------------------------------

    /// Parse a sequence of complete commands until EOF or one of `terminators`
    /// (reserved words that close an enclosing construct, e.g. `fi`, `esac`,
    /// `done`, `else`). Returns the item ids in source order.
    fn parse_command_list(&mut self, terminators: &[Reserved]) -> Vec<dorc_core::AstId> {
        let mut items = Vec::new();
        loop {
            self.skip_separators();
            if self.at_eof() {
                break;
            }
            if let Some(r) = self.peek_reserved() {
                if terminators.contains(&r) {
                    break;
                }
            }
            // `}` / `)` close a brace-group / subshell body.
            if matches!(self.peek(), TokKind::RBrace | TokKind::RParen) {
                break;
            }
            let before = self.cursor;
            let item = self.parse_and_or();
            items.push(item);
            // Defensive anti-stall: if no token was consumed (shouldn't happen),
            // force progress by ⊤-rejecting one token, so parse() always terminates.
            if self.cursor == before {
                let tok = self.bump();
                let node = self.unsupported(
                    UnsupportedReason::Unmodeled("stalled token"),
                    tok.span,
                    Vec::new(),
                    "parser made no progress; token skipped",
                );
                items.push(node);
            }
        }
        items
    }

    // ---- and-or (lowest precedence) -------------------------------------------

    /// `pipeline (('&&'|'||') pipeline)*`, left-associative: `a && b || c` parses
    /// as `(a && b) || c`. Each `&&`/`||` nests an [`NodeKind::AndOr`].
    fn parse_and_or(&mut self) -> dorc_core::AstId {
        let mut left = self.parse_pipeline();
        loop {
            let op = match self.peek() {
                TokKind::AndIf => AndOrOp::And,
                TokKind::OrIf => AndOrOp::Or,
                _ => break,
            };
            self.bump(); // operator
            self.skip_newlines_after_operator();
            let right = self.parse_pipeline();
            let span = self.span_of(left).to(self.span_of(right));
            left = self.builder.alloc(Node { span, kind: NodeKind::AndOr { op, left, right } });
        }
        left
    }

    /// After a binary/pipe operator, sh permits newlines before the next operand.
    fn skip_newlines_after_operator(&mut self) {
        while matches!(self.peek(), TokKind::Newline) {
            self.cursor += 1;
        }
    }

    // ---- pipeline -------------------------------------------------------------

    /// `['!'] command ('|' command)*`. The optional leading `!` negates the
    /// pipeline status (used as an `if` condition: `if ! command -v nginx; …`).
    fn parse_pipeline(&mut self) -> dorc_core::AstId {
        let start_span = self.peek_span();
        let negated = self.eat_bang();

        let first = self.parse_command();
        let mut stages = vec![first];
        while matches!(self.peek(), TokKind::Pipe) {
            self.bump(); // `|`
            self.skip_newlines_after_operator();
            stages.push(self.parse_command());
        }

        if stages.len() == 1 && !negated {
            return stages[0]; // not a pipeline; surface the bare command
        }
        let last = *stages.last().unwrap_or(&first);
        let span = start_span.to(self.span_of(last));
        self.builder.alloc(Node { span, kind: NodeKind::Pipeline { negated, stages } })
    }

    /// A leading `!` is the pipeline-negation reserved word only when it stands as
    /// its own word token. `!` lexes as part of a `Word` (it is not an operator
    /// byte), so we detect a `Word` whose single literal is exactly `!`.
    fn eat_bang(&mut self) -> bool {
        if let TokKind::Word { parts } = self.peek() {
            if single_literal(parts) == Some("!") {
                self.bump();
                return true;
            }
        }
        false
    }

    // ---- command (compound dispatch) ------------------------------------------

    /// Depth-guarding wrapper around [`Self::parse_command_inner`] (`inv-no-throw`):
    /// bounds native-stack recursion on hostile nesting. Past [`MAX_DEPTH`] we
    /// consume one token and ⊤-reject, guaranteeing forward progress without
    /// descending further.
    fn parse_command(&mut self) -> dorc_core::AstId {
        if self.depth >= MAX_DEPTH {
            let tok = self.bump();
            return self.unsupported(
                UnsupportedReason::Unmodeled("nesting too deep"),
                tok.span,
                Vec::new(),
                "nesting exceeds the parser depth bound",
            );
        }
        self.depth += 1;
        let id = self.parse_command_inner();
        self.depth -= 1;
        id
    }

    /// Dispatch on the leading reserved word / token to a compound command, else a
    /// simple command. ⊤-rejects loop constructs here so they never mis-parse.
    fn parse_command_inner(&mut self) -> dorc_core::AstId {
        if let Some(r) = self.peek_reserved() {
            match r {
                Reserved::If => return self.parse_if(),
                Reserved::Case => return self.parse_case(),
                Reserved::For | Reserved::While | Reserved::Until => {
                    return self.parse_loop_rejected(r);
                }
                // A bare closing keyword in command position is malformed; reject
                // the single token so the enclosing parser can resync.
                Reserved::Then
                | Reserved::Elif
                | Reserved::Else
                | Reserved::Fi
                | Reserved::Esac
                | Reserved::In
                | Reserved::Do
                | Reserved::Done => {
                    let tok = self.bump();
                    return self.unsupported(
                        UnsupportedReason::Unmodeled("misplaced reserved word"),
                        tok.span,
                        Vec::new(),
                        "reserved word in command position",
                    );
                }
            }
        }
        match self.peek() {
            TokKind::LParen => self.parse_subshell(),
            TokKind::LBrace => self.parse_brace_group(),
            // `(`-less compound openers handled above; everything else is simple.
            _ => self.parse_simple_or_funcdef(),
        }
    }

    // ---- if -------------------------------------------------------------------

    /// `if cond; then body; [elif cond; then body;]* [else body;] fi`. The
    /// condition is itself a command list (commonly a negated pipeline).
    fn parse_if(&mut self) -> dorc_core::AstId {
        let kw = self.bump(); // `if`
        let cond = self.parse_condition_until(&[Reserved::Then]);
        self.expect_reserved(Reserved::Then, "expected `then` after `if` condition");
        let then_body = self.parse_body_until(&[Reserved::Elif, Reserved::Else, Reserved::Fi]);

        let mut elifs = Vec::new();
        while self.peek_reserved() == Some(Reserved::Elif) {
            self.bump(); // `elif`
            let econd = self.parse_condition_until(&[Reserved::Then]);
            self.expect_reserved(Reserved::Then, "expected `then` after `elif` condition");
            let ebody = self.parse_body_until(&[Reserved::Elif, Reserved::Else, Reserved::Fi]);
            elifs.push(ElseIf { cond: econd, body: ebody });
        }

        let else_body = if self.peek_reserved() == Some(Reserved::Else) {
            self.bump(); // `else`
            Some(self.parse_body_until(&[Reserved::Fi]))
        } else {
            None
        };

        let end = self.expect_reserved(Reserved::Fi, "expected `fi` to close `if`");
        let span = kw.span.to(end);
        self.builder.alloc(Node {
            span,
            kind: NodeKind::If { cond, then_body, elifs, else_body },
        })
    }

    /// Parse a condition (command list) up to a terminator keyword, wrapping it in
    /// a `List` node so the `If`/`elif` cond is always a single id.
    fn parse_condition_until(&mut self, terminators: &[Reserved]) -> dorc_core::AstId {
        let lo = self.peek_span();
        let items = self.parse_command_list(terminators);
        self.wrap_list(items, lo)
    }

    /// Parse a compound body (command list) up to a terminator keyword.
    fn parse_body_until(&mut self, terminators: &[Reserved]) -> dorc_core::AstId {
        let lo = self.peek_span();
        let items = self.parse_command_list(terminators);
        self.wrap_list(items, lo)
    }

    /// Wrap a list of items in a `List` node spanning them (empty ⇒ zero-width).
    fn wrap_list(&mut self, items: Vec<dorc_core::AstId>, lo_hint: Span) -> dorc_core::AstId {
        let span = match (items.first(), items.last()) {
            (Some(&f), Some(&l)) => self.span_of(f).to(self.span_of(l)),
            _ => Span::new(lo_hint.lo, lo_hint.lo),
        };
        self.builder.alloc(Node { span, kind: NodeKind::List { items } })
    }

    // ---- case -----------------------------------------------------------------

    /// `case word in [(] pat (| pat)* ) body ;; … esac`. Patterns are words (incl.
    /// `*` and `|`-alternation); the leading `(` of an arm is optional.
    fn parse_case(&mut self) -> dorc_core::AstId {
        let kw = self.bump(); // `case`
        let word = self.parse_word_or_placeholder();
        self.expect_reserved(Reserved::In, "expected `in` after `case` word");
        self.skip_separators();

        let mut arms = Vec::new();
        while !self.at_eof() && self.peek_reserved() != Some(Reserved::Esac) {
            if let Some(arm) = self.parse_case_arm() {
                arms.push(arm);
            } else {
                break; // malformed arm already ⊤-reported; stop to resync at esac
            }
            self.skip_separators();
        }

        let end = self.expect_reserved(Reserved::Esac, "expected `esac` to close `case`");
        let span = kw.span.to(end);
        self.builder.alloc(Node { span, kind: NodeKind::Case { word, arms } })
    }

    /// One case arm: optional `(`, `pat (| pat)*`, `)`, body, `;;` (or `esac` for
    /// the last arm). Returns `None` if it cannot find a `)` (malformed).
    fn parse_case_arm(&mut self) -> Option<CaseArm> {
        let arm_lo = self.peek_span();
        if matches!(self.peek(), TokKind::LParen) {
            self.bump(); // optional leading `(`
        }

        let mut patterns = Vec::new();
        loop {
            // A pattern is a word; `|` separates alternatives, `)` ends the list.
            if matches!(self.peek(), TokKind::RParen) {
                break;
            }
            if self.at_eof() || self.peek_reserved() == Some(Reserved::Esac) {
                self.push_error(
                    MALFORMED,
                    arm_lo,
                    "unterminated `case` arm (no `)` before esac/EOF)",
                );
                return None;
            }
            let pat = self.parse_word_or_placeholder();
            patterns.push(pat);
            if matches!(self.peek(), TokKind::Pipe) {
                self.bump(); // `|` between patterns
                continue;
            }
            break;
        }

        if matches!(self.peek(), TokKind::RParen) {
            self.bump(); // `)`
        } else {
            self.push_error(MALFORMED, arm_lo, "expected `)` after case pattern");
            return None;
        }

        // Body runs until `;;` or `esac`. (`;;` is the standard terminator; the
        // final arm may omit it and end directly at `esac`.)
        let body = self.parse_case_arm_body();
        let span = arm_lo.to(self.span_of(body));
        Some(CaseArm { patterns, body, span })
    }

    /// Parse a case-arm body: a command list ending at `;;` or `esac`. Consumes the
    /// `;;` if present (but not `esac`, which the arm loop needs to see).
    fn parse_case_arm_body(&mut self) -> dorc_core::AstId {
        let lo = self.peek_span();
        let mut items = Vec::new();
        loop {
            self.skip_separators();
            if matches!(self.peek(), TokKind::DSemi) {
                self.bump(); // consume `;;`
                break;
            }
            if self.at_eof() || self.peek_reserved() == Some(Reserved::Esac) {
                break;
            }
            let before = self.cursor;
            items.push(self.parse_and_or());
            // `;;` may abut the last command with no separator; loop re-checks.
            if self.cursor == before {
                self.cursor += 1; // anti-stall
            }
        }
        self.wrap_list(items, lo)
    }

    // ---- subshell / group -----------------------------------------------------

    /// `( list )` — subshell. Carries trailing redirections.
    fn parse_subshell(&mut self) -> dorc_core::AstId {
        let open = self.bump(); // `(`
        let items = self.parse_command_list(&[]);
        let body = self.wrap_list(items, open.span);
        let close_hi = if matches!(self.peek(), TokKind::RParen) {
            self.bump().span
        } else {
            self.push_error(MALFORMED, open.span, "unterminated subshell `(` (no `)`）");
            self.peek_span()
        };
        let redirs = self.parse_redirs();
        let span = open.span.to(close_hi);
        self.builder.alloc(Node { span, kind: NodeKind::Subshell { body, redirs } })
    }

    /// `{ list; }` — brace group (current shell). Carries trailing redirections.
    fn parse_brace_group(&mut self) -> dorc_core::AstId {
        let open = self.bump(); // `{`
        let items = self.parse_command_list(&[]);
        let body = self.wrap_list(items, open.span);
        let close_hi = if matches!(self.peek(), TokKind::RBrace) {
            self.bump().span
        } else {
            self.push_error(MALFORMED, open.span, "unterminated brace group `{` (no `}`)");
            self.peek_span()
        };
        let redirs = self.parse_redirs();
        let span = open.span.to(close_hi);
        self.builder.alloc(Node { span, kind: NodeKind::Group { body, redirs } })
    }

    // ---- loops (⊤-reject) -----------------------------------------------------

    /// `for`/`while`/`until` are outside the modeled subset → ⊤-reject as `Loop`.
    /// We consume the whole construct (to its matching `done`, balancing nested
    /// loops) so the surrounding parser resyncs cleanly, salvaging nothing (the
    /// body's effects are unknown-by-design under ⊤).
    fn parse_loop_rejected(&mut self, _kind: Reserved) -> dorc_core::AstId {
        let start = self.peek_span();
        let end = self.consume_balanced_loop();
        let span = start.to(end);
        self.unsupported(
            UnsupportedReason::Loop,
            span,
            Vec::new(),
            "loop constructs (for/while/until) are not in the modeled subset",
        )
    }

    /// Consume tokens from the loop keyword through its matching `done`, tracking
    /// nested `for`/`while`/`until … do … done` depth. Returns the span of the
    /// closing `done` (or the last token at EOF).
    fn consume_balanced_loop(&mut self) -> Span {
        let mut depth = 0u32;
        let mut last = self.peek_span();
        loop {
            if self.at_eof() {
                return last;
            }
            let r = self.peek_reserved();
            match r {
                Some(Reserved::For | Reserved::While | Reserved::Until) => depth += 1,
                Some(Reserved::Done) => {
                    depth -= 1;
                    last = self.peek_span();
                    self.bump();
                    if depth == 0 {
                        return last;
                    }
                    continue;
                }
                _ => {}
            }
            last = self.peek_span();
            self.bump();
        }
    }

    // ---- simple command / funcdef ---------------------------------------------

    /// A simple command: leading `name=value` assignments, then words (command
    /// name + args), then redirections (which may also interleave — sh allows
    /// `> f cmd`, but the fixture/idioms keep them trailing, which we accept in any
    /// position). Also detects `name() { … }` function definitions, and applies the
    /// ⊤-triggers (`eval`, dynamic command name, `$(( ))`-as-command, lvalue builtins).
    fn parse_simple_or_funcdef(&mut self) -> dorc_core::AstId {
        let start_span = self.peek_span();

        // Function definition: `name ( ) { … }`. Detect a Word(name) followed by
        // `(` `)`. (POSIX funcname is a name; we accept any single-literal word.)
        if self.looks_like_funcdef() {
            return self.parse_funcdef();
        }

        let mut assigns = Vec::new();
        let mut words = Vec::new();
        let mut redirs = Vec::new();
        let mut had_token = false;

        // Leading assignments: `name=value` only while no command word has appeared.
        while words.is_empty() {
            match self.peek() {
                TokKind::Word { parts } => {
                    if let Some((name, name_span, rest)) = split_assignment(parts, self.peek_span())
                    {
                        let value = self.lower_assignment_value(rest);
                        let span = name_span; // name_span covers the lhs; value spans separately
                        let id = self.builder.alloc(Node {
                            span: self.peek_span(),
                            kind: NodeKind::Assign { name, name_span: span, value },
                        });
                        assigns.push(id);
                        self.bump();
                        had_token = true;
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        // Command word + args + redirs (interleaved redirs allowed).
        loop {
            match self.peek() {
                TokKind::Word { .. } => {
                    // Stop if a reserved word appears in argument position that
                    // closes an enclosing construct — but that is handled by the
                    // list terminator; here a Word is always an argument.
                    let w = self.parse_word_or_placeholder();
                    words.push(w);
                    had_token = true;
                }
                TokKind::Redir { .. } | TokKind::HereDoc { .. } => {
                    if let Some(r) = self.parse_one_redir() {
                        redirs.push(r);
                        had_token = true;
                    }
                }
                _ => break,
            }
        }

        if !had_token {
            // Nothing consumed: emit a ⊤ on the offending token so the caller's
            // anti-stall does not loop, classifying the common cases by kind.
            let (reason, msg): (UnsupportedReason, &str) = match self.peek() {
                TokKind::Amp => (
                    UnsupportedReason::Unmodeled("background `&`"),
                    "background/async `&` is not in the modeled subset",
                ),
                TokKind::Pipe | TokKind::OrIf | TokKind::AndIf => (
                    UnsupportedReason::Unmodeled("operator without command"),
                    "binary operator with no preceding command",
                ),
                TokKind::DSemi => (
                    UnsupportedReason::Unmodeled("misplaced `;;`"),
                    "`;;` outside a case arm",
                ),
                _ => (UnsupportedReason::Unmodeled("unexpected token"), "expected a command"),
            };
            let tok = self.bump();
            return self.unsupported(reason, tok.span, Vec::new(), msg);
        }

        // ⊤-trigger checks on the assembled simple command.
        if let Some(reject) = self.check_simple_triggers(&assigns, &words) {
            return reject;
        }

        let span = self.span_covering(start_span, &assigns, &words, &redirs);
        self.builder.alloc(Node { span, kind: NodeKind::Simple { assigns, words, redirs } })
    }

    /// Look ahead for `name ( )` to decide funcdef vs simple. Does not consume.
    fn looks_like_funcdef(&self) -> bool {
        let is_name = matches!(self.peek(), TokKind::Word { parts } if single_literal(parts).is_some_and(is_func_name));
        if !is_name {
            return false;
        }
        matches!(self.nth_kind(1), TokKind::LParen) && matches!(self.nth_kind(2), TokKind::RParen)
    }

    fn nth_kind(&self, ahead: usize) -> &TokKind {
        let i = (self.cursor + ahead).min(self.tokens.len() - 1);
        &self.tokens[i].kind
    }

    /// `name() compound` — body is a brace-group (the common form) or any compound.
    fn parse_funcdef(&mut self) -> dorc_core::AstId {
        let name_tok = self.bump(); // name word
        let name = match &name_tok.kind {
            TokKind::Word { parts } => single_literal(parts).unwrap_or("").to_string(),
            _ => String::new(),
        };
        let name_span = name_tok.span;
        self.bump(); // `(`
        self.bump(); // `)`
        self.skip_separators();
        let body = self.parse_command(); // typically a brace group
        let span = name_span.to(self.span_of(body));
        self.builder.alloc(Node {
            span,
            kind: NodeKind::FuncDef { name, name_span, body },
        })
    }

    /// Apply the ⊤-triggers that depend on the *whole* simple command. Returns a
    /// replacement ⊤-node id if one fires (the already-built children are salvaged).
    /// Triggers (synthesis ⊤-set): `eval`; `.`/`source` of a non-literal target;
    /// dynamic command name (first word not a literal); `$(( ))` as the command;
    /// lvalue-taking builtins (`unset "$x"`, `printf -v`, `test -v`).
    fn check_simple_triggers(
        &mut self,
        assigns: &[dorc_core::AstId],
        words: &[dorc_core::AstId],
    ) -> Option<dorc_core::AstId> {
        let first = *words.first()?;
        let span = self.span_of(first);
        let salvage = || {
            let mut v: Vec<dorc_core::AstId> = assigns.to_vec();
            v.extend_from_slice(words);
            v
        };

        // `$(( … ))` used as a command (the whole first word is one Arithmetic part).
        if self.word_is_sole_arithmetic(first) {
            return Some(self.unsupported(
                UnsupportedReason::ArithmeticExpansion,
                span,
                salvage(),
                "arithmetic expansion `$(( … ))` used as a command",
            ));
        }

        // Dynamic command name: the command word is not a fixed literal (e.g.
        // `"$cmd" arg`, `${x}-y`, a command-substitution as the name).
        let first_literal = self.word_single_literal(first);
        if first_literal.is_none() {
            return Some(self.unsupported(
                UnsupportedReason::DynamicExecution,
                span,
                salvage(),
                "dynamic command name (first word is not a fixed literal)",
            ));
        }
        let name = first_literal.unwrap_or_default();

        match name {
            "eval" => Some(self.unsupported(
                UnsupportedReason::DynamicExecution,
                span,
                salvage(),
                "`eval` executes constructed code (un-analyzable)",
            )),
            "." | "source" => {
                // `. file` is fine only when the target is a literal path; a dynamic
                // target (`. "$x"`) is a ⊤-trigger. With no second word it is malformed.
                let target_literal = words.get(1).is_some_and(|&w| self.word_single_literal(w).is_some());
                if words.len() >= 2 && !target_literal {
                    Some(self.unsupported(
                        UnsupportedReason::DynamicExecution,
                        span,
                        salvage(),
                        "`.`/`source` of a non-literal target",
                    ))
                } else {
                    None
                }
            }
            "unset" => {
                // `unset "$x"` / `unset $x` — dynamic lvalue. A literal `unset FOO`
                // is in principle modelable, but the ⊤-set lists unset of a dynamic
                // name; we reject only the dynamic form, keep literal `unset FOO`.
                let dynamic = words
                    .iter()
                    .skip(1)
                    .any(|&w| self.word_single_literal(w).is_none());
                if dynamic {
                    Some(self.unsupported(
                        UnsupportedReason::DynamicLValue,
                        span,
                        salvage(),
                        "`unset` of a dynamic lvalue",
                    ))
                } else {
                    None
                }
            }
            "printf" => {
                // `printf -v VAR …` assigns to VAR (an lvalue) — ⊤. Plain printf is fine.
                let has_v = words
                    .iter()
                    .skip(1)
                    .any(|&w| self.word_single_literal(w) == Some("-v"));
                if has_v {
                    Some(self.unsupported(
                        UnsupportedReason::DynamicLValue,
                        span,
                        salvage(),
                        "`printf -v` writes to a variable lvalue",
                    ))
                } else {
                    None
                }
            }
            "test" | "[" => {
                // `test -v NAME` / `[ -v NAME ]` queries a *variable name* (lvalue-ish
                // indirection) — ⊤ per the trigger set.
                let has_v = words
                    .iter()
                    .skip(1)
                    .any(|&w| matches!(self.word_single_literal(w), Some("-v")));
                if has_v {
                    Some(self.unsupported(
                        UnsupportedReason::DynamicLValue,
                        span,
                        salvage(),
                        "`test -v` / `[ -v ]` references a variable lvalue",
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Is this word a single `Arithmetic` part (`$(( … ))` alone)?
    fn word_is_sole_arithmetic(&self, id: dorc_core::AstId) -> bool {
        matches!(
            &self.builder_node(id).kind,
            NodeKind::Word { parts } if matches!(parts.as_slice(), [WordPart::Arithmetic])
        )
    }

    /// If this word is a single literal/single-quoted part, return it (the only
    /// statically-fixed-string case — mirrors `ast::Word::as_literal`).
    fn word_single_literal(&self, id: dorc_core::AstId) -> Option<&str> {
        match &self.builder_node(id).kind {
            NodeKind::Word { parts } => match parts.as_slice() {
                [WordPart::Literal(s)] | [WordPart::SingleQuoted(s)] => Some(s),
                _ => None,
            },
            _ => None,
        }
    }

    // ---- redirections ---------------------------------------------------------

    /// Parse zero or more trailing redirections.
    fn parse_redirs(&mut self) -> Vec<dorc_core::AstId> {
        let mut redirs = Vec::new();
        while matches!(self.peek(), TokKind::Redir { .. } | TokKind::HereDoc { .. }) {
            if let Some(r) = self.parse_one_redir() {
                redirs.push(r);
            } else {
                break;
            }
        }
        redirs
    }

    /// Parse a single redirection node from a `Redir`/`HereDoc` token (+ its target
    /// word, for the file-taking forms). `haz-redir-as-mutation`: redirs are
    /// first-class effect sites.
    fn parse_one_redir(&mut self) -> Option<dorc_core::AstId> {
        let tok = self.bump();
        match tok.kind {
            TokKind::HereDoc { body, quoted, fd } => {
                let op = RedirOp::HereDoc;
                let node = Node {
                    span: tok.span,
                    kind: NodeKind::Redir { op, fd, target: RedirTarget::HereDoc { body, quoted } },
                };
                Some(self.builder.alloc(node))
            }
            TokKind::Redir { op: RedirToken::Dup, fd } => {
                // `>&2`, `>&-`, `2>&1`: target is a word that is an fd number or `-`.
                let (target, hi) = self.parse_dup_target();
                let span = tok.span.to(hi);
                Some(self.builder.alloc(Node {
                    span,
                    kind: NodeKind::Redir { op: RedirOp::Dup, fd, target },
                }))
            }
            TokKind::Redir { op, fd } => {
                let ast_op = match op {
                    RedirToken::Read => RedirOp::Read,
                    RedirToken::Write => RedirOp::Write,
                    RedirToken::Append => RedirOp::Append,
                    RedirToken::Dup => unreachable!("dup handled above"),
                };
                // File-taking redirection: the next word is the path target.
                let path = self.parse_word_or_placeholder();
                let span = tok.span.to(self.span_of(path));
                Some(self.builder.alloc(Node {
                    span,
                    kind: NodeKind::Redir { op: ast_op, fd, target: RedirTarget::Word(path) },
                }))
            }
            _ => None,
        }
    }

    /// Parse the target of a `>&`/`<&` dup: a word that is an fd digit-run, or `-`
    /// (close). Returns the [`RedirTarget`] and the target's span hi.
    fn parse_dup_target(&mut self) -> (RedirTarget, Span) {
        match self.peek() {
            TokKind::Word { parts } => {
                let span = self.peek_span();
                if let Some(lit) = single_literal(parts) {
                    if let Ok(n) = lit.parse::<i32>() {
                        self.bump();
                        return (RedirTarget::Fd(n), span);
                    }
                    if lit == "-" {
                        self.bump();
                        return (RedirTarget::Fd(-1), span); // `-` ⇒ close (sentinel -1)
                    }
                }
                // `>&word` with a non-fd word: treat the word as the target path
                // (rare; keeps it lossless rather than dropping).
                let w = self.parse_word_or_placeholder();
                (RedirTarget::Word(w), span)
            }
            _ => {
                // `>&` with nothing after (malformed) — sentinel close, no consume.
                (RedirTarget::Fd(-1), self.peek_span())
            }
        }
    }

    // ---- words ----------------------------------------------------------------

    /// Parse the current token as a word, lowering its lexer parts to AST parts
    /// (re-parsing command-substitution bodies into sub-ASTs). If the current token
    /// is not a word, emit a malformed ⊤ placeholder so callers always get an id.
    fn parse_word_or_placeholder(&mut self) -> dorc_core::AstId {
        match self.peek() {
            TokKind::Word { .. } => {
                let tok = self.bump();
                let span = tok.span;
                let parts = match tok.kind {
                    TokKind::Word { parts } => parts,
                    _ => unreachable!(),
                };
                let lowered = self.lower_parts(parts);
                self.builder.alloc(Node { span, kind: NodeKind::Word { parts: lowered } })
            }
            _ => {
                let span = self.peek_span();
                self.unsupported(
                    UnsupportedReason::Unmodeled("expected word"),
                    span,
                    Vec::new(),
                    "expected a word here",
                )
            }
        }
    }

    /// Lower the value side of an assignment (`name=<rest>`) into a `Word` node, or
    /// `None` for a bare `name=`. `rest` is the lexer parts after the `=`.
    fn lower_assignment_value(&mut self, rest: Vec<LexPart>) -> Option<dorc_core::AstId> {
        if rest.is_empty() {
            return None; // `name=` (explicit empty)
        }
        let lowered = self.lower_parts(rest);
        // Span is unknown precisely here (we split inside a single word token); use a
        // zero-width span at current token. Provenance is coarse for assignment RHS;
        // acceptable for the spike (the lhs name_span carries the locator).
        let span = self.peek_span();
        Some(self.builder.alloc(Node { span, kind: NodeKind::Word { parts: lowered } }))
    }

    /// Lower lexer word-parts to AST word-parts. The only non-trivial case is
    /// command substitution: its raw inner text is parsed as a nested script and
    /// the resulting root id stored in [`WordPart::CommandSubst`].
    fn lower_parts(&mut self, parts: Vec<LexPart>) -> Vec<WordPart> {
        parts.into_iter().map(|p| self.lower_part(p)).collect()
    }

    fn lower_part(&mut self, part: LexPart) -> WordPart {
        match part {
            LexPart::Literal(s) => WordPart::Literal(s),
            LexPart::SingleQuoted(s) => WordPart::SingleQuoted(s),
            LexPart::DoubleQuoted(inner) => WordPart::DoubleQuoted(self.lower_parts(inner)),
            LexPart::Param { name } => WordPart::Param { name },
            LexPart::ParamComplex => WordPart::ParamComplex,
            LexPart::Arithmetic => WordPart::Arithmetic,
            LexPart::CommandSubst(inner) => {
                let id = self.parse_subst_body(&inner);
                WordPart::CommandSubst(id)
            }
        }
    }

    /// Parse a command-substitution body (`$( … )` / backticks) into a nested
    /// `List` node in *this* arena, by re-lexing `inner` and recursing on the SAME
    /// builder (so no id-remapping is needed — `inv-determinism` stays trivial).
    /// The outer token stream is saved and restored around the nested parse.
    ///
    /// Inner-parse diagnostic spans are relative to `inner`, not the outer source —
    /// coarse provenance the spike accepts (a precise mapping would add the subst's
    /// source offset). Termination: `inner` is strictly shorter than the enclosing
    /// source, so the recursion bottoms out.
    fn parse_subst_body(&mut self, inner: &str) -> dorc_core::AstId {
        let saved_tokens = std::mem::replace(&mut self.tokens, lex(inner));
        let saved_cursor = std::mem::replace(&mut self.cursor, 0);
        let lo = self.peek_span();
        let items = self.parse_command_list(&[]);
        let body = self.wrap_list(items, lo);
        self.tokens = saved_tokens;
        self.cursor = saved_cursor;
        body
    }

    // ---- span / arena access helpers ------------------------------------------

    fn builder_node(&self, id: dorc_core::AstId) -> &Node {
        self.builder.node(id)
    }

    fn span_of(&self, id: dorc_core::AstId) -> Span {
        self.builder_node(id).span
    }

    /// Smallest span covering a simple command's pieces (falling back to the
    /// command's start when it has no children with spans).
    fn span_covering(
        &self,
        start: Span,
        assigns: &[dorc_core::AstId],
        words: &[dorc_core::AstId],
        redirs: &[dorc_core::AstId],
    ) -> Span {
        let mut span = start;
        for &id in assigns.iter().chain(words).chain(redirs) {
            span = span.to(self.span_of(id));
        }
        span
    }

    /// Consume an expected reserved word, emitting a malformed diagnostic if it is
    /// absent (but never blocking — returns the span to use as the construct end).
    fn expect_reserved(&mut self, want: Reserved, msg: &str) -> Span {
        if self.peek_reserved() == Some(want) {
            self.bump().span
        } else {
            let span = self.peek_span();
            self.push_error(MALFORMED, span, msg.to_string());
            span
        }
    }
}

// ===========================================================================
// Free helpers (pure; no parser state)
// ===========================================================================

/// If a lexer word is exactly one `Literal` part, return it. Reserved-word
/// recognition and command-name fixedness both key off this.
fn single_literal(parts: &[LexPart]) -> Option<&str> {
    match parts {
        [LexPart::Literal(s)] => Some(s),
        _ => None,
    }
}

/// A valid sh function name: `name` chars, not starting with a digit. Used to
/// gate funcdef detection so `2()` (invalid) is not treated as a definition.
fn is_func_name(s: &str) -> bool {
    let mut bytes = s.bytes();
    match bytes.next() {
        Some(b) if b == b'_' || b.is_ascii_alphabetic() => {}
        _ => return false,
    }
    bytes.all(|b| b == b'_' || b.is_ascii_alphanumeric())
}

/// Split a word's lexer parts into `(name, name_span, value_parts)` if it is a
/// leading assignment `name=…`. Only fires when the word *begins* with a literal
/// part containing `=` and the pre-`=` text is a valid assignment name. The value
/// parts are the remainder (the post-`=` literal tail + any later parts), so
/// `x="$y"` and `x=$(cmd)` lower correctly.
fn split_assignment(parts: &[LexPart], word_span: Span) -> Option<(String, Span, Vec<LexPart>)> {
    let first = parts.first()?;
    let LexPart::Literal(lit) = first else {
        return None;
    };
    let eq = lit.find('=')?;
    let name = &lit[..eq];
    if name.is_empty() || !is_assignment_name(name) {
        return None;
    }
    let tail = &lit[eq + 1..];
    let mut value_parts: Vec<LexPart> = Vec::new();
    if !tail.is_empty() {
        value_parts.push(LexPart::Literal(tail.to_string()));
    }
    value_parts.extend(parts[1..].iter().cloned());
    // name_span: approximate as the word's start..start+name.len() (byte-accurate
    // for the common single-token case; assignment lhs is plain ASCII here).
    let name_span = Span::new(
        word_span.lo,
        BytePos(word_span.lo.0 + u32::try_from(name.len()).unwrap_or(0)),
    );
    Some((name.to_string(), name_span, value_parts))
}

/// A valid assignment target name: `[A-Za-z_][A-Za-z0-9_]*`.
fn is_assignment_name(s: &str) -> bool {
    is_func_name(s) // same lexical rule
}
