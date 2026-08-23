//! Byte-oriented sh lexer. Produces a flat [`Token`] stream the recursive-descent
//! parser drives. Pure and total (`inv-no-throw`, `inv-determinism`): malformed
//! input never panics — an unterminated quote/substitution closes at EOF and the
//! parser raises the diagnostic.
//!
//! sh tokenisation is context-sensitive in ways a textbook lexer is not, so this
//! layer keeps two pieces of state the parser cannot recover after the fact:
//!
//! * **Here-document bodies** (`haz-redir-as-mutation`, the heredoc target). The
//!   `<<EOF` operator names a delimiter on one line, but the body is the *following
//!   lines*. We follow the POSIX rule: when the operator is seen, the delimiter is
//!   queued; at the next unquoted newline the queued bodies are consumed in order,
//!   up to a line equal to the delimiter. This cannot be done in the parser without
//!   re-lexing, so it lives here.
//! * **Operator vs. word boundaries.** `>`, `>>`, `2>&1`, `|`, `||`, `&&`, `;`,
//!   `;;`, `(`, `)`, `{`/`}` (only when word-isolated) are punctuation; everything
//!   else accretes into a [`TokKind::Word`] whose *parts* preserve quoting losslessly.
//!
//! Reserved words (`if`, `then`, `case`, …) are NOT lexed specially — POSIX
//! reserved words are recognised positionally by the parser (a `Word` whose single
//! literal part equals the keyword, in command position). Keeping them as words
//! here is what lets `echo if` and `x=if` stay correct.
//!
//! Command-substitution bodies are captured as raw inner text in a
//! [`LexPart::CommandSubst`]; the parser re-parses that text into a real sub-AST
//! and only then mints the [`crate::ast::WordPart::CommandSubst`] `AstId`. The lexer
//! never touches the AST arena (keeps lexer ⟂ arena).

use crate::ast::{SubstituteKind, TrimEnd};
use dorc_core::{BytePos, Span};

/// A lexed token: its kind plus the source span it covers.
#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub kind: TokKind,
    pub span: Span,
}

/// Token kinds for the modeled sh subset. Operators are pre-split into the exact
/// shapes the grammar needs; everything lexical-but-not-operator is a `Word`.
#[derive(Debug, Clone)]
pub(crate) enum TokKind {
    /// A maximal run of word characters + adjacent quotings, quoting kept lossless
    /// in `parts` (`haz-unquoted`). Never empty.
    Word { parts: Vec<LexPart> },
    /// `<<`-style heredoc operator, already resolved: body text + whether the
    /// delimiter was quoted (`<<'EOF'` ⇒ no expansion in body), plus optional fd.
    HereDoc {
        body: String,
        quoted: bool,
        fd: Option<u32>,
    },
    /// `|`
    Pipe,
    /// `||`
    OrIf,
    /// `&&`
    AndIf,
    /// `&` (async) — recognised so we can ⊤-reject it loudly rather than mislex.
    Amp,
    /// `;`
    Semi,
    /// `;;` (case arm terminator)
    DSemi,
    /// newline (a command separator like `;`, but also the heredoc-body trigger)
    Newline,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{` as an isolated token (brace-group open)
    LBrace,
    /// `}` as an isolated token (brace-group close)
    RBrace,
    /// `<`, `>`, `>>`, `<&`, `>&` style redirection operator (non-heredoc), with
    /// optional leading fd (`2>`). `Dup` distinguishes `>&`/`<&` from `>`/`<`.
    Redir { op: RedirToken, fd: Option<u32> },
    /// End of input.
    Eof,
}

/// The redirection operators the lexer recognises (heredoc is separate — it
/// carries a resolved body, so it cannot share this enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RedirToken {
    /// `<`
    Read,
    /// `>`
    Write,
    /// `>>`
    Append,
    /// `>&` / `<&`
    Dup,
}

/// A word fragment as the lexer sees it — mirrors [`crate::ast::WordPart`] but
/// keeps command-substitution bodies as *raw inner text* (the lexer cannot build
/// AST nodes). The parser lowers these to `ast::WordPart`, re-parsing subst bodies.
#[derive(Debug, Clone)]
pub(crate) enum LexPart {
    Literal(String),
    SingleQuoted(String),
    DoubleQuoted(Vec<LexPart>),
    Param {
        name: String,
    },
    /// `$( … )` / `` `…` `` — raw inner source (no surrounding delimiters).
    CommandSubst(String),
    /// `${x:-y}`, `${0%/*}`, `${#x}`, … — an operator-form parameter expansion, decoded.
    /// See [`crate::ast::WordPart::ParamExpansion`], whose shape this mirrors.
    ParamExpansion {
        base: String,
        op: LexParamOp,
    },
    /// `$(( … ))` — arithmetic expansion (opaque, a ⊤-trigger when used as a word).
    Arithmetic,
}

/// The lexer's [`crate::ast::ParamOp`], carrying operand words as [`LexPart`]s.
#[derive(Debug, Clone)]
pub(crate) enum LexParamOp {
    EmptyDefault {
        colon: bool,
    },
    Substitute {
        kind: SubstituteKind,
        colon: bool,
        word: Vec<LexPart>,
    },
    Trim {
        end: TrimEnd,
        greedy: bool,
        pattern: Vec<LexPart>,
    },
    Length,
    Unmodelled,
}

/// An operator recognised at a `${…}` body's base, before its operand has been lexed.
#[derive(Debug, Clone, Copy)]
enum Operator {
    Substitute { kind: SubstituteKind, colon: bool },
    Trim { end: TrimEnd, greedy: bool },
}

impl Operator {
    const fn trim(end: TrimEnd, greedy: bool) -> Self {
        Operator::Trim { end, greedy }
    }
}

/// A still-pending heredoc: the operator's index in the token stream (so we can
/// backfill the resolved body) and the parsed delimiter.
struct PendingHeredoc {
    token_index: usize,
    delimiter: String,
    quoted: bool,
    fd: Option<u32>,
}

struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
    tokens: Vec<Token>,
    /// Heredoc operators seen on the current logical line, awaiting their bodies.
    pending: Vec<PendingHeredoc>,
}

/// Lex `src` into a token stream terminated by [`TokKind::Eof`]. Total: any byte
/// sequence yields tokens (unterminated constructs are closed at EOF).
pub(crate) fn lex(src: &str) -> Vec<Token> {
    let mut lexer = Lexer {
        src: src.as_bytes(),
        pos: 0,
        tokens: Vec::new(),
        pending: Vec::new(),
    };
    lexer.run();
    lexer.tokens
}

fn is_blank(b: u8) -> bool {
    b == b' ' || b == b'\t'
}

/// Bytes that force the end of an unquoted word run. Note: backtick is *not* here
/// — a word may start with / contain a `` `…` `` command substitution, which the
/// word lexer consumes via its backtick arm. (If backtick were a terminator, a
/// leading backtick would dispatch to `lex_word`, which would refuse to advance,
/// spinning `run` into an unbounded token Vec — a 24 GiB-allocation foot-gun.)
fn is_word_terminator(b: u8) -> bool {
    is_blank(b) || matches!(b, b'\n' | b'|' | b'&' | b';' | b'(' | b')' | b'<' | b'>')
}

impl Lexer<'_> {
    fn run(&mut self) {
        loop {
            self.skip_blanks_and_comments();
            if self.pos >= self.src.len() {
                self.push(TokKind::Eof, self.pos, self.pos);
                return;
            }
            let b = self.src[self.pos];
            match b {
                b'\n' => self.lex_newline(),
                b'|' => self.lex_pipe(),
                b'&' => self.lex_amp(),
                b';' => self.lex_semi(),
                b'(' => self.one(TokKind::LParen),
                b')' => self.one(TokKind::RParen),
                b'<' | b'>' => self.lex_redir(),
                // `{`/`}` are operators only when word-isolated; otherwise ordinary
                // word bytes (`${x}`, `a{b}`). The word lexer covers the non-isolated
                // case, so we only reach here for a standalone brace.
                b'{' if self.brace_is_isolated() => self.one(TokKind::LBrace),
                b'}' if self.brace_is_isolated() => self.one(TokKind::RBrace),
                _ => self.lex_word(),
            }
        }
    }

    fn push(&mut self, kind: TokKind, lo: usize, hi: usize) {
        self.tokens.push(Token {
            kind,
            span: Span::new(BytePos(lo as u32), BytePos(hi as u32)),
        });
    }

    fn one(&mut self, kind: TokKind) {
        let lo = self.pos;
        self.pos += 1;
        self.push(kind, lo, self.pos);
    }

    fn skip_blanks_and_comments(&mut self) {
        loop {
            while self.pos < self.src.len() && is_blank(self.src[self.pos]) {
                self.pos += 1;
            }
            if self.pos + 1 < self.src.len()
                && self.src[self.pos] == b'\\'
                && self.src[self.pos + 1] == b'\n'
            {
                self.pos += 2; // line-continuation is whitespace
                continue;
            }
            // `#` between tokens always begins a comment (we are at a word boundary).
            if self.pos < self.src.len() && self.src[self.pos] == b'#' {
                while self.pos < self.src.len() && self.src[self.pos] != b'\n' {
                    self.pos += 1;
                }
                continue;
            }
            break;
        }
    }

    fn lex_newline(&mut self) {
        let lo = self.pos;
        self.pos += 1;
        self.push(TokKind::Newline, lo, self.pos);
        if !self.pending.is_empty() {
            self.consume_heredoc_bodies();
        }
    }

    fn lex_pipe(&mut self) {
        let lo = self.pos;
        if self.peek(1) == Some(b'|') {
            self.pos += 2;
            self.push(TokKind::OrIf, lo, self.pos);
        } else {
            self.pos += 1;
            self.push(TokKind::Pipe, lo, self.pos);
        }
    }

    fn lex_amp(&mut self) {
        let lo = self.pos;
        if self.peek(1) == Some(b'&') {
            self.pos += 2;
            self.push(TokKind::AndIf, lo, self.pos);
        } else {
            self.pos += 1;
            self.push(TokKind::Amp, lo, self.pos);
        }
    }

    fn lex_semi(&mut self) {
        let lo = self.pos;
        if self.peek(1) == Some(b';') {
            self.pos += 2;
            self.push(TokKind::DSemi, lo, self.pos);
        } else {
            self.pos += 1;
            self.push(TokKind::Semi, lo, self.pos);
        }
    }

    /// A brace is an operator token only when it stands alone as a word. `run`
    /// dispatches here at a token boundary, so the leading side is already a
    /// boundary; we additionally require the *next* byte to be a word terminator.
    fn brace_is_isolated(&self) -> bool {
        match self.peek(1) {
            None => true,
            Some(next) => is_word_terminator(next),
        }
    }

    /// Lex a `<`/`>` family redirection with NO leading fd (fd-prefixed forms are
    /// produced by [`Self::redir_with_fd`] from the word lexer).
    fn lex_redir(&mut self) {
        let lo = self.pos;
        self.lex_redir_inner(lo, None);
    }

    /// Emit a redirection operator with optional leading fd. `lo` is the operator
    /// (or fd-digit) start. Cursor must be on `<` or `>`.
    fn lex_redir_inner(&mut self, lo: usize, fd: Option<u32>) {
        match self.src[self.pos] {
            b'>' => {
                if self.peek(1) == Some(b'>') {
                    self.pos += 2;
                    self.push(
                        TokKind::Redir {
                            op: RedirToken::Append,
                            fd,
                        },
                        lo,
                        self.pos,
                    );
                } else if self.peek(1) == Some(b'&') {
                    self.pos += 2;
                    self.push(
                        TokKind::Redir {
                            op: RedirToken::Dup,
                            fd,
                        },
                        lo,
                        self.pos,
                    );
                } else {
                    self.pos += 1;
                    self.push(
                        TokKind::Redir {
                            op: RedirToken::Write,
                            fd,
                        },
                        lo,
                        self.pos,
                    );
                }
            }
            b'<' => {
                if self.peek(1) == Some(b'<') {
                    self.lex_heredoc_op(lo, fd);
                } else if self.peek(1) == Some(b'&') {
                    self.pos += 2;
                    self.push(
                        TokKind::Redir {
                            op: RedirToken::Dup,
                            fd,
                        },
                        lo,
                        self.pos,
                    );
                } else {
                    self.pos += 1;
                    self.push(
                        TokKind::Redir {
                            op: RedirToken::Read,
                            fd,
                        },
                        lo,
                        self.pos,
                    );
                }
            }
            _ => unreachable!("redir entered on non-redir byte"),
        }
    }

    /// `>&`/`<&` and `>`/`>>`/`<` with a leading fd digit-run (`2>`, `2>&1` LHS).
    fn redir_with_fd(&mut self, lo: usize, fd: u32) {
        self.lex_redir_inner(lo, Some(fd));
    }

    /// Lex `<<` (and `<<-`), parse the delimiter word, queue the heredoc. The body
    /// is backfilled at the next newline; the token is pushed now (placeholder body)
    /// so its stream position is fixed.
    fn lex_heredoc_op(&mut self, lo: usize, fd: Option<u32>) {
        self.pos += 2; // consume `<<`
        if self.peek(0) == Some(b'-') {
            self.pos += 1; // `<<-` tab-stripping; accepted, dash ignored for the subset
        }
        while self.pos < self.src.len() && is_blank(self.src[self.pos]) {
            self.pos += 1;
        }
        let (delimiter, quoted) = self.read_heredoc_delimiter();
        let op_hi = self.pos;
        let token_index = self.tokens.len();
        self.push(
            TokKind::HereDoc {
                body: String::new(),
                quoted,
                fd,
            },
            lo,
            op_hi,
        );
        self.pending.push(PendingHeredoc {
            token_index,
            delimiter,
            quoted,
            fd,
        });
    }

    /// Read a heredoc delimiter word. A quoted delimiter (`'EOF'`, `"EOF"`, or a
    /// backslash) disables expansion in the body. Returns (literal-text, was-quoted).
    fn read_heredoc_delimiter(&mut self) -> (String, bool) {
        let mut out = String::new();
        let mut quoted = false;
        while self.pos < self.src.len() {
            let b = self.src[self.pos];
            match b {
                b'\'' => {
                    quoted = true;
                    self.pos += 1;
                    while self.pos < self.src.len() && self.src[self.pos] != b'\'' {
                        out.push(self.src[self.pos] as char);
                        self.pos += 1;
                    }
                    if self.pos < self.src.len() {
                        self.pos += 1;
                    }
                }
                b'"' => {
                    quoted = true;
                    self.pos += 1;
                    while self.pos < self.src.len() && self.src[self.pos] != b'"' {
                        out.push(self.src[self.pos] as char);
                        self.pos += 1;
                    }
                    if self.pos < self.src.len() {
                        self.pos += 1;
                    }
                }
                b'\\' => {
                    quoted = true;
                    self.pos += 1;
                    if self.pos < self.src.len() {
                        out.push(self.src[self.pos] as char);
                        self.pos += 1;
                    }
                }
                _ if is_word_terminator(b) => break,
                _ => {
                    out.push(b as char);
                    self.pos += 1;
                }
            }
        }
        (out, quoted)
    }

    /// At a newline with queued heredocs: fill each body from successive physical
    /// lines until a delimiter line (or EOF), in queue order.
    fn consume_heredoc_bodies(&mut self) {
        let pending = std::mem::take(&mut self.pending);
        for heredoc in pending {
            let body = self.read_one_heredoc_body(&heredoc.delimiter);
            if let Some(tok) = self.tokens.get_mut(heredoc.token_index) {
                tok.kind = TokKind::HereDoc {
                    body,
                    quoted: heredoc.quoted,
                    fd: heredoc.fd,
                };
            }
        }
    }

    /// Consume physical lines for one heredoc until the delimiter line or EOF.
    /// Body lines are joined with `\n` (trailing newline kept when terminated).
    fn read_one_heredoc_body(&mut self, delimiter: &str) -> String {
        let mut body = String::new();
        loop {
            if self.pos >= self.src.len() {
                return body; // unterminated heredoc: body runs to EOF
            }
            let line_start = self.pos;
            while self.pos < self.src.len() && self.src[self.pos] != b'\n' {
                self.pos += 1;
            }
            let line = &self.src[line_start..self.pos];
            if self.pos < self.src.len() {
                self.pos += 1; // consume the newline ending this line
            }
            if line == delimiter.as_bytes() {
                return body;
            }
            body.push_str(&String::from_utf8_lossy(line));
            body.push('\n');
        }
    }

    fn peek(&self, ahead: usize) -> Option<u8> {
        self.src.get(self.pos + ahead).copied()
    }

    /// Lex a maximal word: a run of word characters, quotings, and expansions, all
    /// abutting with no blank between. A pure-digit run immediately followed by
    /// `<`/`>` is instead an fd-prefixed redirection (`2>file`).
    fn lex_word(&mut self) {
        let lo = self.pos;

        if self.src[self.pos].is_ascii_digit() {
            let mut scan = self.pos;
            while scan < self.src.len() && self.src[scan].is_ascii_digit() {
                scan += 1;
            }
            if scan < self.src.len()
                && matches!(self.src[scan], b'<' | b'>')
                && let Ok(fd_str) = std::str::from_utf8(&self.src[self.pos..scan])
                && let Ok(fd) = fd_str.parse::<u32>()
            {
                self.pos = scan;
                self.redir_with_fd(lo, fd);
                return;
            }
        }

        let mut parts: Vec<LexPart> = Vec::new();
        let mut literal = String::new();

        while self.pos < self.src.len() {
            let b = self.src[self.pos];
            if is_word_terminator(b) {
                break;
            }
            match b {
                b'\'' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_single_quoted());
                }
                b'"' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_double_quoted());
                }
                b'$' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_dollar());
                }
                b'`' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_backtick());
                }
                b'\\' => {
                    self.pos += 1;
                    if self.pos < self.src.len() {
                        let next = self.src[self.pos];
                        if next == b'\n' {
                            self.pos += 1; // line continuation inside a word
                        } else {
                            literal.push(next as char);
                            self.pos += 1;
                        }
                    } else {
                        literal.push('\\'); // trailing backslash at EOF is literal
                    }
                }
                _ => {
                    literal.push(b as char);
                    self.pos += 1;
                }
            }
        }
        Self::flush(&mut parts, &mut literal);

        // Hard no-progress guard (`inv-no-throw`): `run` only dispatches here on a
        // non-terminator byte, so the loop above must have consumed ≥1 byte. If a
        // future edit ever lets `lex_word` start on a byte it does not consume,
        // force one byte of progress rather than spin `run` into an unbounded Vec.
        if self.pos == lo {
            self.pos += 1;
        }
        if parts.is_empty() {
            parts.push(LexPart::Literal(String::new()));
        }
        self.push(TokKind::Word { parts }, lo, self.pos);
    }

    /// Push the accumulated literal run (if any) as a `Literal` part and clear it.
    fn flush(parts: &mut Vec<LexPart>, literal: &mut String) {
        if !literal.is_empty() {
            parts.push(LexPart::Literal(std::mem::take(literal)));
        }
    }

    /// `'...'` — fully literal; no escapes, no expansion. Unterminated ⇒ to EOF.
    fn lex_single_quoted(&mut self) -> LexPart {
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.src.len() && self.src[self.pos] != b'\'' {
            s.push(self.src[self.pos] as char);
            self.pos += 1;
        }
        if self.pos < self.src.len() {
            self.pos += 1;
        }
        LexPart::SingleQuoted(s)
    }

    /// `"..."` — expansions occur inside; result does not word-split. Nested parts
    /// recorded so `"$x"` is `DoubleQuoted([Param])`. Unterminated ⇒ to EOF.
    fn lex_double_quoted(&mut self) -> LexPart {
        self.pos += 1;
        let mut parts: Vec<LexPart> = Vec::new();
        let mut literal = String::new();
        while self.pos < self.src.len() && self.src[self.pos] != b'"' {
            let b = self.src[self.pos];
            match b {
                b'$' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_dollar());
                }
                b'`' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_backtick());
                }
                b'\\' => {
                    self.pos += 1;
                    if self.pos < self.src.len() {
                        let next = self.src[self.pos];
                        if next == b'\n' {
                            self.pos += 1;
                        } else if matches!(next, b'$' | b'`' | b'"' | b'\\') {
                            literal.push(next as char);
                            self.pos += 1;
                        } else {
                            literal.push('\\');
                            literal.push(next as char);
                            self.pos += 1;
                        }
                    } else {
                        literal.push('\\');
                    }
                }
                _ => {
                    literal.push(b as char);
                    self.pos += 1;
                }
            }
        }
        Self::flush(&mut parts, &mut literal);
        if self.pos < self.src.len() {
            self.pos += 1;
        }
        LexPart::DoubleQuoted(parts)
    }

    /// Dispatch a `$`-introduced expansion. `$name`, `${name}`, `${complex}`,
    /// `$(...)`, `$((...))`. A lone `$` with no valid introducer is a literal `$`.
    fn lex_dollar(&mut self) -> LexPart {
        self.pos += 1;
        match self.peek(0) {
            Some(b'(') => {
                if self.peek(1) == Some(b'(') {
                    self.lex_arith()
                } else {
                    self.lex_command_subst_paren()
                }
            }
            Some(b'{') => self.lex_braced_param(),
            Some(b) if b == b'_' || b.is_ascii_alphabetic() => {
                let start = self.pos;
                while self.pos < self.src.len()
                    && (self.src[self.pos] == b'_' || self.src[self.pos].is_ascii_alphanumeric())
                {
                    self.pos += 1;
                }
                let name = String::from_utf8_lossy(&self.src[start..self.pos]).into_owned();
                LexPart::Param { name }
            }
            Some(b) if b.is_ascii_digit() => {
                let name = (b as char).to_string(); // `$1` positional (single digit)
                self.pos += 1;
                LexPart::Param { name }
            }
            Some(b'@' | b'*' | b'#' | b'?' | b'-' | b'$' | b'!') => {
                // Special params `$@ $* $# $? $- $$ $!`: dynamic referent (arg
                // vector / pid / status). Kept as Param for co-reference, but the
                // analyzer must treat them as non-fixed; `$@`/`$*` are arity-affecting.
                let name = (self.src[self.pos] as char).to_string();
                self.pos += 1;
                LexPart::Param { name }
            }
            _ => LexPart::Literal("$".to_string()),
        }
    }

    /// `${...}` — simple `${name}` (name-chars only) ⇒ [`LexPart::Param`]; an operator form
    /// (`${x:-y}`, `${0%/*}`, `${#x}`) ⇒ a decoded [`LexPart::ParamExpansion`].
    ///
    /// The operand is lexed IN PLACE with the ordinary word machinery rather than captured and
    /// re-lexed, so its spans are real and its quoting is honest (`tn-coarse-subst-provenance` is
    /// what re-lexing costs).
    fn lex_braced_param(&mut self) -> LexPart {
        self.pos += 1; // consume `{`
        let body_start = self.pos;
        let body_end = self.scan_braced_body();
        let part = self.decode_braced_body(body_start, body_end);
        self.pos = body_end;
        if self.pos < self.src.len() {
            self.pos += 1; // consume `}`
        }
        part
    }

    /// Where this `${…}` body ends: the matching `}`, or EOF. Nested `{}` balance, and a `}`
    /// inside a quoted pattern (`${x%"}"}`) does NOT close the expansion.
    fn scan_braced_body(&mut self) -> usize {
        let mut depth = 1u32;
        while self.pos < self.src.len() {
            match self.src[self.pos] {
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        return self.pos;
                    }
                }
                b'\'' => {
                    self.skip_single_quoted_raw();
                    continue;
                }
                b'"' => {
                    self.skip_double_quoted_raw();
                    continue;
                }
                b'`' => {
                    self.skip_backtick_raw();
                    continue;
                }
                b'\\' if self.pos + 1 < self.src.len() => self.pos += 1,
                _ => {}
            }
            self.pos += 1;
        }
        self.src.len()
    }

    /// Split a `${…}` body into its BASE and its OPERATOR, lexing any operand word in place.
    ///
    /// Whatever this cannot name is [`LexParamOp::Unmodelled`] — a variant a consumer must match,
    /// never a silent identity. Deciding what an operator MEANS is the analyzer's
    /// (`semantic-top-not-here`): decoding mints no diagnostic here.
    fn decode_braced_body(&mut self, body_start: usize, body_end: usize) -> LexPart {
        let body = &self.src[body_start..body_end];
        if !body.is_empty() && body.iter().all(|&c| c == b'_' || c.is_ascii_alphanumeric()) {
            return LexPart::Param {
                name: String::from_utf8_lossy(body).into_owned(),
            };
        }
        let unmodelled = |base: String| LexPart::ParamExpansion {
            base,
            op: LexParamOp::Unmodelled,
        };
        if let Some(name) = body.strip_prefix(b"#")
            && !name.is_empty()
            && name.iter().all(|&c| c == b'_' || c.is_ascii_alphanumeric())
        {
            return LexPart::ParamExpansion {
                base: String::from_utf8_lossy(name).into_owned(),
                op: LexParamOp::Length,
            };
        }
        let base_len = Self::base_name_len(body);
        let base = String::from_utf8_lossy(&body[..base_len]).into_owned();
        let Some((op_len, op)) = Self::operator_at(&body[base_len..]) else {
            return unmodelled(base);
        };
        self.pos = body_start.saturating_add(base_len).saturating_add(op_len);
        let operand = self.lex_parts_until(body_end);
        LexPart::ParamExpansion {
            base,
            op: match op {
                Operator::Substitute { kind, colon } => {
                    if operand.is_empty() && kind == SubstituteKind::Default {
                        LexParamOp::EmptyDefault { colon }
                    } else {
                        LexParamOp::Substitute {
                            kind,
                            colon,
                            word: operand,
                        }
                    }
                }
                Operator::Trim { end, greedy } => LexParamOp::Trim {
                    end,
                    greedy,
                    pattern: operand,
                },
            },
        }
    }

    /// How many leading bytes of a `${…}` body name the parameter: a POSIX name, a positional
    /// digit run, or one special parameter. Zero when the body opens with something else.
    fn base_name_len(body: &[u8]) -> usize {
        let named = body
            .iter()
            .take_while(|&&c| c == b'_' || c.is_ascii_alphanumeric())
            .count();
        if named > 0 {
            return named;
        }
        usize::from(matches!(
            body.first(),
            Some(b'@' | b'*' | b'#' | b'?' | b'-' | b'$' | b'!')
        ))
    }

    /// The operator immediately after the base, and how many bytes it spans.
    fn operator_at(rest: &[u8]) -> Option<(usize, Operator)> {
        let substitute = |byte: u8, colon: bool| {
            let kind = match byte {
                b'-' => SubstituteKind::Default,
                b'=' => SubstituteKind::Assign,
                b'+' => SubstituteKind::Alternate,
                _ => SubstituteKind::Error,
            };
            Operator::Substitute { kind, colon }
        };
        match rest {
            [b':', k @ (b'-' | b'=' | b'+' | b'?'), ..] => Some((2, substitute(*k, true))),
            [k @ (b'-' | b'=' | b'+' | b'?'), ..] => Some((1, substitute(*k, false))),
            [b'%', b'%', ..] => Some((2, Operator::trim(TrimEnd::Suffix, true))),
            [b'%', ..] => Some((1, Operator::trim(TrimEnd::Suffix, false))),
            [b'#', b'#', ..] => Some((2, Operator::trim(TrimEnd::Prefix, true))),
            [b'#', ..] => Some((1, Operator::trim(TrimEnd::Prefix, false))),
            _ => None,
        }
    }

    /// Lex word-parts from the current position up to `end` — the ordinary word machinery, bounded
    /// by the enclosing `${…}` rather than by a word terminator.
    fn lex_parts_until(&mut self, end: usize) -> Vec<LexPart> {
        let mut parts: Vec<LexPart> = Vec::new();
        let mut literal = String::new();
        while self.pos < end {
            match self.src[self.pos] {
                b'\'' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_single_quoted());
                }
                b'"' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_double_quoted());
                }
                b'$' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_dollar());
                }
                b'`' => {
                    Self::flush(&mut parts, &mut literal);
                    parts.push(self.lex_backtick());
                }
                b'\\' if self.pos + 1 < end => {
                    self.pos += 1;
                    literal.push(self.src[self.pos] as char);
                    self.pos += 1;
                }
                b => {
                    literal.push(b as char);
                    self.pos += 1;
                }
            }
        }
        Self::flush(&mut parts, &mut literal);
        parts
    }

    /// `$( ... )` command substitution: capture raw inner text, balancing nested
    /// parens and skipping quotes/backticks so `$(echo ")")` does not close early.
    fn lex_command_subst_paren(&mut self) -> LexPart {
        self.pos += 1; // consume `(`
        let inner_start = self.pos;
        let mut depth = 1u32;
        while self.pos < self.src.len() && depth > 0 {
            match self.src[self.pos] {
                b'(' => {
                    depth += 1;
                    self.pos += 1;
                }
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    self.pos += 1;
                }
                b'\'' => self.skip_single_quoted_raw(),
                b'"' => self.skip_double_quoted_raw(),
                b'`' => self.skip_backtick_raw(),
                _ => self.pos += 1,
            }
        }
        let inner = String::from_utf8_lossy(&self.src[inner_start..self.pos]).into_owned();
        if self.pos < self.src.len() {
            self.pos += 1; // consume `)`
        }
        LexPart::CommandSubst(inner)
    }

    /// `` `...` `` legacy command substitution. Backslash escapes the next byte;
    /// closes at the next unescaped backtick (or EOF).
    fn lex_backtick(&mut self) -> LexPart {
        self.pos += 1;
        let inner_start = self.pos;
        while self.pos < self.src.len() && self.src[self.pos] != b'`' {
            if self.src[self.pos] == b'\\' && self.pos + 1 < self.src.len() {
                self.pos += 2;
            } else {
                self.pos += 1;
            }
        }
        let inner = String::from_utf8_lossy(&self.src[inner_start..self.pos]).into_owned();
        if self.pos < self.src.len() {
            self.pos += 1;
        }
        LexPart::CommandSubst(inner)
    }

    /// `$(( ... ))` arithmetic expansion, balanced to the matching `))`. Opaque.
    fn lex_arith(&mut self) -> LexPart {
        self.pos += 2; // consume `((`
        let mut depth = 2u32;
        while self.pos < self.src.len() && depth > 0 {
            match self.src[self.pos] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                _ => {}
            }
            self.pos += 1;
        }
        LexPart::Arithmetic
    }

    fn skip_single_quoted_raw(&mut self) {
        self.pos += 1;
        while self.pos < self.src.len() && self.src[self.pos] != b'\'' {
            self.pos += 1;
        }
        if self.pos < self.src.len() {
            self.pos += 1;
        }
    }

    fn skip_double_quoted_raw(&mut self) {
        self.pos += 1;
        while self.pos < self.src.len() && self.src[self.pos] != b'"' {
            if self.src[self.pos] == b'\\' && self.pos + 1 < self.src.len() {
                self.pos += 1;
            }
            self.pos += 1;
        }
        if self.pos < self.src.len() {
            self.pos += 1;
        }
    }

    fn skip_backtick_raw(&mut self) {
        self.pos += 1;
        while self.pos < self.src.len() && self.src[self.pos] != b'`' {
            if self.src[self.pos] == b'\\' && self.pos + 1 < self.src.len() {
                self.pos += 1;
            }
            self.pos += 1;
        }
        if self.pos < self.src.len() {
            self.pos += 1;
        }
    }
}
