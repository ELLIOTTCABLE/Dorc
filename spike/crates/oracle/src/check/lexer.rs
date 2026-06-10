//! The dialect lexer — a small, byte-oriented tokenizer producing spanned tokens.
//!
//! Deliberately *not* the book lexer (`adj-dialect-parser`, note 203 §4). It is
//! total and never panics (`inv-no-throw`): hostile input (NUL bytes, unterminated
//! quotes, control chars) becomes [`Tok::Error`] tokens or a clean EOF, which the
//! parser turns into a per-function lift failure — never a crash.
//!
//! Tokenization model: shell-ish but constrained. Whitespace separates tokens;
//! newlines are significant (statement separators, like `;`). Quotes (`"`, `'`)
//! group bytes into one word. The metacharacters the dialect uses — `(){};|` and
//! the test brackets — are recognized as standalone tokens *when not inside a
//! word/quote*. Everything else accretes into a [`Tok::Word`] whose internal
//! structure (positional? variable? `name=value`?) the parser decodes.

use dorc_core::{BytePos, Span};

/// A lexed token with its source span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Token {
    pub kind: Tok,
    pub span: Span,
}

/// Token kinds. A small fixed set; anything not modeled is [`Tok::Error`] so the
/// parser can reject it as out-of-dialect rather than mis-read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Tok {
    /// A maximal word — a run of bytes with quotes resolved into the lexeme. The
    /// stored `String` is the *decoded* lexeme (quotes removed, but `$`/`#`/`{`
    /// preserved); the parser decides if it is a positional, var, annotation
    /// fragment, etc. A separate `single_quoted` flag records whether the whole
    /// word came from a single-quoted run (so `'$1'` is distinguishable).
    Word { lexeme: String, single_quoted: bool },
    /// `(` — open paren (case-arm patterns / subshell-shaped; only the case-arm use
    /// is in dialect).
    LParen,
    /// `)` — close paren (ends a case-arm pattern list).
    RParen,
    /// `{` — open brace (function body).
    LBrace,
    /// `}` — close brace (function body).
    RBrace,
    /// `;` — statement separator.
    Semi,
    /// `;;` — case-arm terminator.
    DSemi,
    /// `|` — case-arm pattern alternation (also sh pipe, but the dialect only uses
    /// it between case patterns).
    Pipe,
    /// `[` — test-command open bracket (a standalone token only when space-flanked,
    /// the sh `[` builtin).
    LBracket,
    /// `]` — test-command close bracket.
    RBracket,
    /// A newline (statement separator).
    Newline,
    /// A redirection operator chunk (`>`, `>/dev/null`, `2>&1`, `2>/dev/null`).
    /// Kept whole and opaque; it is part of a command's verbatim span and the
    /// parser folds it into the preceding command without interpreting it.
    Redirect(String),
    /// A byte the dialect does not model (NUL, an unterminated quote's remainder,
    /// a stray backtick / `$(`). Forces the parser to reject the construct.
    Error(String),
}

/// Lex `src` into tokens. Total: every byte is consumed into some token (or an
/// `Error` token), and the function always terminates. Spans are byte offsets into
/// `src`.
pub(super) fn lex(src: &str) -> Vec<Token> {
    Lexer {
        bytes: src.as_bytes(),
        src,
        pos: 0,
        out: Vec::new(),
    }
    .run()
}

struct Lexer<'a> {
    bytes: &'a [u8],
    src: &'a str,
    pos: usize,
    out: Vec<Token>,
}

impl Lexer<'_> {
    fn run(mut self) -> Vec<Token> {
        // termination guard: each branch advances `pos` by ≥1, so this only bounds a
        // hypothetical regression — never trips on correct input (`inv-no-throw`).
        let budget = self.bytes.len().saturating_mul(2).saturating_add(16);
        let mut steps = 0usize;
        while self.pos < self.bytes.len() {
            steps = steps.saturating_add(1);
            if steps > budget {
                break;
            }
            let b = self.bytes[self.pos];
            match b {
                b' ' | b'\t' | b'\r' => self.pos = self.pos.saturating_add(1),
                b'\n' => self.punct(Tok::Newline, 1),
                b'#' if self.at_comment_start() => self.skip_comment(),
                b'(' => self.punct(Tok::LParen, 1),
                b')' => self.punct(Tok::RParen, 1),
                b'{' if self.brace_is_standalone() => self.punct(Tok::LBrace, 1),
                b'}' if self.brace_is_standalone() => self.punct(Tok::RBrace, 1),
                b';' => {
                    if self.peek(1) == Some(b';') {
                        self.punct(Tok::DSemi, 2);
                    } else {
                        self.punct(Tok::Semi, 1);
                    }
                }
                b'|' => self.punct(Tok::Pipe, 1),
                b'[' if self.bracket_is_standalone(true) => self.punct(Tok::LBracket, 1),
                b']' if self.bracket_is_standalone(false) => self.punct(Tok::RBracket, 1),
                b'>' | b'<' => self.redirect(),
                b'0'..=b'9' if self.is_fd_redirect() => self.redirect(), // `2>/dev/null`
                0 => self.error_byte(),
                b'`' => self.error_run("backtick command-substitution is out of dialect"),
                _ => self.word(),
            }
        }
        self.out
    }

    fn peek(&self, ahead: usize) -> Option<u8> {
        self.bytes.get(self.pos.saturating_add(ahead)).copied()
    }

    fn prev(&self) -> Option<u8> {
        self.pos
            .checked_sub(1)
            .and_then(|i| self.bytes.get(i))
            .copied()
    }

    fn punct(&mut self, kind: Tok, len: usize) {
        let lo = self.pos;
        let hi = self.pos.saturating_add(len);
        self.out.push(Token {
            kind,
            span: span(lo, hi),
        });
        self.pos = hi;
    }

    fn error_byte(&mut self) {
        let lo = self.pos;
        self.out.push(Token {
            kind: Tok::Error("unmodeled byte".to_owned()),
            span: span(lo, lo.saturating_add(1)),
        });
        self.pos = self.pos.saturating_add(1);
    }

    /// Emit an Error token spanning one metacharacter run and step past it. Used for
    /// constructs (backticks) the dialect rejects wholesale.
    fn error_run(&mut self, msg: &str) {
        let lo = self.pos;
        self.out.push(Token {
            kind: Tok::Error(msg.to_owned()),
            span: span(lo, lo.saturating_add(1)),
        });
        self.pos = self.pos.saturating_add(1);
    }

    /// A `#` starts a comment only at a token boundary (start of line or after
    /// whitespace) — `${1#-}` and `dpkg-query`'s `#` are NOT comments.
    fn at_comment_start(&self) -> bool {
        matches!(self.prev(), None | Some(b' ' | b'\t' | b'\n' | b'\r'))
    }

    fn skip_comment(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos] != b'\n' {
            self.pos = self.pos.saturating_add(1);
        }
    }

    /// A `{`/`}` is a standalone brace token only when flanked by whitespace / line
    /// boundaries. This keeps `${1#-}`'s braces inside the word (they are adjacent
    /// to `$`/digits, not space-flanked).
    fn brace_is_standalone(&self) -> bool {
        let before_ok = matches!(
            self.prev(),
            None | Some(b' ' | b'\t' | b'\n' | b'\r' | b';')
        );
        let after_ok = matches!(
            self.peek(1),
            None | Some(b' ' | b'\t' | b'\n' | b'\r' | b';')
        );
        before_ok && after_ok
    }

    /// `[`/`]` are standalone test tokens only when whitespace-flanked on the
    /// outer side (the sh `[` builtin is a normal word: `[ x = y ]`). `opening`
    /// selects which side must be whitespace.
    fn bracket_is_standalone(&self, opening: bool) -> bool {
        if opening {
            let before_ok = matches!(
                self.prev(),
                None | Some(b' ' | b'\t' | b'\n' | b'\r' | b';')
            );
            let after_ok = matches!(self.peek(1), Some(b' ' | b'\t'));
            before_ok && after_ok
        } else {
            let before_ok = matches!(self.prev(), Some(b' ' | b'\t'));
            let after_ok = matches!(
                self.peek(1),
                None | Some(b' ' | b'\t' | b'\n' | b'\r' | b';')
            );
            before_ok && after_ok
        }
    }

    /// Is the current `<`/`>` (or `N>`) the start of a redirection? `>` always is;
    /// the digit case is handled by [`is_fd_redirect`](Self::is_fd_redirect).
    fn is_fd_redirect(&self) -> bool {
        let mut i = self.pos;
        while i < self.bytes.len() && self.bytes[i].is_ascii_digit() {
            i = i.saturating_add(1);
        }
        matches!(self.bytes.get(i), Some(b'>' | b'<'))
    }

    /// Lex a redirection chunk verbatim: an optional leading fd digit-run, the
    /// `>`/`<`/`>>`/`<<` operator, an optional `&fd`, and an optional target word
    /// (`/dev/null`). Kept opaque — it is part of the command's verbatim span.
    fn redirect(&mut self) {
        let lo = self.pos;
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_digit() {
            self.pos = self.pos.saturating_add(1);
        }
        while matches!(self.peek(0), Some(b'>' | b'<')) {
            self.pos = self.pos.saturating_add(1);
        }
        if self.peek(0) == Some(b'&') {
            self.pos = self.pos.saturating_add(1);
            while matches!(self.peek(0), Some(b) if b.is_ascii_digit()) {
                self.pos = self.pos.saturating_add(1);
            }
        }
        // optional target after one separating space (POSIX allows `> /dev/null`)
        if matches!(self.peek(0), Some(b' ' | b'\t')) {
            let save = self.pos;
            self.pos = self.pos.saturating_add(1);
            if !matches!(self.peek(0), Some(c) if is_word_byte(c)) {
                self.pos = save;
            }
        }
        while matches!(self.peek(0), Some(c) if is_word_byte(c)) {
            self.pos = self.pos.saturating_add(1);
        }
        let text = self.src.get(lo..self.pos).unwrap_or_default().to_owned();
        self.out.push(Token {
            kind: Tok::Redirect(text),
            span: span(lo, self.pos),
        });
    }

    fn peek0(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    /// Lex a maximal word, resolving `"…"` and `'…'` quoting. Stops at unquoted
    /// whitespace or a metacharacter. An unterminated quote becomes an `Error`
    /// token (and consumes to EOF) so the parser rejects the construct.
    fn word(&mut self) {
        let lo = self.pos;
        let mut lexeme = String::new();
        // `single_quoted` is true iff the entire word is exactly one single-quoted
        // run (so `'$1'` ⇒ literal `$1`); a word mixing quotes/literals is not.
        let mut single_quoted_whole = false;
        let mut saw_any_part = false;
        let mut multi_part = false;

        loop {
            match self.peek0() {
                Some(b'"') => {
                    if saw_any_part {
                        multi_part = true;
                    }
                    saw_any_part = true;
                    self.pos = self.pos.saturating_add(1);
                    if !self.take_until_quote(b'"', &mut lexeme) {
                        return self.unterminated(lo);
                    }
                }
                Some(b'\'') => {
                    if saw_any_part {
                        multi_part = true;
                    } else {
                        single_quoted_whole = true;
                    }
                    saw_any_part = true;
                    self.pos = self.pos.saturating_add(1);
                    if !self.take_until_quote(b'\'', &mut lexeme) {
                        return self.unterminated(lo);
                    }
                }
                Some(b) if is_word_byte(b) => {
                    if saw_any_part {
                        multi_part = true;
                    }
                    saw_any_part = true;
                    lexeme.push(b as char);
                    self.pos = self.pos.saturating_add(1);
                }
                // EOF, whitespace, or a metacharacter ends the word.
                None | Some(_) => break,
            }
        }

        let single_quoted = single_quoted_whole && !multi_part;
        self.out.push(Token {
            kind: Tok::Word {
                lexeme,
                single_quoted,
            },
            span: span(lo, self.pos),
        });
    }

    /// Consume bytes into `buf` until the matching `quote` byte (which is consumed
    /// but not stored). Returns false if EOF is hit first (unterminated). Single
    /// quotes are literal; double quotes here keep their contents verbatim too
    /// (the dialect's double-quoted words are `"$1"`/`"$pkg"`, decoded later — we
    /// do not process `\` escapes, which the dialect does not use).
    fn take_until_quote(&mut self, quote: u8, buf: &mut String) -> bool {
        while let Some(b) = self.peek0() {
            self.pos = self.pos.saturating_add(1);
            if b == quote {
                return true;
            }
            buf.push(b as char);
        }
        false
    }

    fn unterminated(&mut self, lo: usize) {
        // Consume to EOF; the unterminated quote poisons the rest of the line, but
        // we have already advanced past it, so emit one Error spanning lo..pos.
        self.pos = self.bytes.len();
        self.out.push(Token {
            kind: Tok::Error("unterminated quote".to_owned()),
            span: span(lo, self.pos),
        });
    }
}

fn byte_pos(i: usize) -> BytePos {
    BytePos(u32::try_from(i).unwrap_or(u32::MAX))
}

/// A half-open span from two byte indices.
fn span(lo: usize, hi: usize) -> Span {
    Span::new(byte_pos(lo), byte_pos(hi))
}

/// Bytes that may appear unquoted inside a word. Excludes whitespace and the
/// metacharacters the lexer handles standalone. `$`, `#`, `{`, `}`, `:`, `=`, `-`,
/// `.`, `/` are word bytes (so `${1#-}`, `name=$1`, `com.debian.apt.Package`,
/// `dpkg-query`, `/dev/null` stay intact). NUL and ASCII control chars are NOT word
/// bytes — they fall to the lexer's Error path. Whitespace, the standalone
/// metacharacters, the redirection operators, and NUL all map to the same "not a
/// word byte" outcome.
fn is_word_byte(b: u8) -> bool {
    let is_meta = matches!(
        b,
        b' ' | b'\t'
            | b'\n'
            | b'\r'
            | b'('
            | b')'
            | b';'
            | b'|'
            | b'`'
            | b'"'
            | b'\''
            | b'>'
            | b'<'
            | 0
    );
    !is_meta && !b.is_ascii_control()
}
