//! A tiny, dependency-free JSON reader/writer (the workspace is serde-free by kernel discipline).
//! Two directions: [`parse`] a tool's machine format (shellcheck `-f json1`) tolerantly — a malformed
//! blob yields `None` so the adapter degrades down the ladder (`27R` §4), never panics
//! (`inv-no-throw`); and [`escape_into`] a string for the JSONL OUTPUT envelope (`27R` §5). Cursor is
//! `Vec<char>` + a `usize` pos read through `.get()` and advanced with `saturating_add` — no raw
//! indexing / unchecked arithmetic (the workspace lint gate).

/// A parsed JSON value. Objects keep insertion order as a `Vec` of pairs (lookup by [`get`](Self::get));
/// numbers are `f64` (JSON has one number type) read back as integers where the caller needs them.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    /// `null`.
    Null,
    /// `true` / `false`.
    Bool(bool),
    /// A number.
    Num(f64),
    /// A string (unescaped).
    Str(String),
    /// An array.
    Arr(Vec<Json>),
    /// An object (insertion-ordered key/value pairs).
    Obj(Vec<(String, Json)>),
}

impl Json {
    /// The value under `key` in an object, or `None`.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(pairs) => pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// This value as an array slice, if it is one.
    #[must_use]
    pub fn as_array(&self) -> Option<&[Json]> {
        match self {
            Json::Arr(items) => Some(items),
            _ => None,
        }
    }

    /// This value as a string, if it is one.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    /// This value as a non-negative integer, if it is a representable number.
    #[must_use]
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Json::Num(n) if *n >= 0.0 && n.is_finite() && *n <= f64::from(u32::MAX) => {
                #[expect(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    reason = "guarded: 0.0 <= n <= u32::MAX and finite, so the cast is a well-defined \
                              truncation of a line/code number"
                )]
                let v = *n as u32;
                Some(v)
            }
            _ => None,
        }
    }
}

/// Parse a whole JSON document tolerantly: `Some(Json)` iff the input is one well-formed value
/// (trailing whitespace allowed); `None` on any malformation. Never panics (`inv-no-throw`).
#[must_use]
pub fn parse(src: &str) -> Option<Json> {
    let chars: Vec<char> = src.chars().collect();
    let mut p = Parser { chars, pos: 0 };
    p.skip_ws();
    let v = p.value()?;
    p.skip_ws();
    // A well-formed document has nothing but whitespace after the top value.
    if p.pos == p.chars.len() {
        Some(v)
    } else {
        None
    }
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.chars.get(self.pos).copied();
        if c.is_some() {
            self.pos = self.pos.saturating_add(1);
        }
        c
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ' | '\t' | '\n' | '\r')) {
            self.pos = self.pos.saturating_add(1);
        }
    }

    /// Consume the exact literal `lit` (already peeked its first char), returning `Some(v)` or `None`.
    fn literal(&mut self, lit: &str, v: Json) -> Option<Json> {
        for want in lit.chars() {
            if self.bump()? != want {
                return None;
            }
        }
        Some(v)
    }

    fn value(&mut self) -> Option<Json> {
        self.skip_ws();
        match self.peek()? {
            '{' => self.object(),
            '[' => self.array(),
            '"' => self.string().map(Json::Str),
            't' => self.literal("true", Json::Bool(true)),
            'f' => self.literal("false", Json::Bool(false)),
            'n' => self.literal("null", Json::Null),
            c if c == '-' || c.is_ascii_digit() => self.number(),
            _ => None,
        }
    }

    fn object(&mut self) -> Option<Json> {
        self.bump(); // '{'
        let mut pairs = Vec::new();
        self.skip_ws();
        if self.peek() == Some('}') {
            self.bump();
            return Some(Json::Obj(pairs));
        }
        loop {
            self.skip_ws();
            if self.peek() != Some('"') {
                return None;
            }
            let key = self.string()?;
            self.skip_ws();
            if self.bump()? != ':' {
                return None;
            }
            let val = self.value()?;
            pairs.push((key, val));
            self.skip_ws();
            match self.bump()? {
                ',' => {}
                '}' => return Some(Json::Obj(pairs)),
                _ => return None,
            }
        }
    }

    fn array(&mut self) -> Option<Json> {
        self.bump(); // '['
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(']') {
            self.bump();
            return Some(Json::Arr(items));
        }
        loop {
            let val = self.value()?;
            items.push(val);
            self.skip_ws();
            match self.bump()? {
                ',' => {}
                ']' => return Some(Json::Arr(items)),
                _ => return None,
            }
        }
    }

    fn string(&mut self) -> Option<String> {
        self.bump(); // opening quote
        let mut s = String::new();
        loop {
            match self.bump()? {
                '"' => return Some(s),
                '\\' => match self.bump()? {
                    '"' => s.push('"'),
                    '\\' => s.push('\\'),
                    '/' => s.push('/'),
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    'r' => s.push('\r'),
                    'b' => s.push('\u{8}'),
                    'f' => s.push('\u{c}'),
                    'u' => s.push(self.unicode_escape()?),
                    _ => return None,
                },
                c => s.push(c),
            }
        }
    }

    /// Read exactly four hex digits after a `\u`, returning the code point (a lone surrogate is
    /// replaced with U+FFFD rather than failing — tolerant).
    fn unicode_escape(&mut self) -> Option<char> {
        let mut code: u32 = 0;
        for _ in 0..4 {
            let d = self.bump()?.to_digit(16)?;
            code = code.saturating_mul(16).saturating_add(d);
        }
        Some(char::from_u32(code).unwrap_or('\u{fffd}'))
    }

    fn number(&mut self) -> Option<Json> {
        let mut lexeme = String::new();
        while let Some(c) = self.peek() {
            if c == '-' || c == '+' || c == '.' || c == 'e' || c == 'E' || c.is_ascii_digit() {
                lexeme.push(c);
                self.pos = self.pos.saturating_add(1);
            } else {
                break;
            }
        }
        lexeme.parse::<f64>().ok().map(Json::Num)
    }
}

/// Append `s` to `out` as a JSON string BODY (no surrounding quotes) with the mandatory escapes
/// (`"`, `\`, control chars as `\uXXXX` or the short forms). Used by the JSONL renderer (`27R` §5).
pub fn escape_into(out: &mut String, s: &str) {
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str("\\u");
                // Four lowercase hex digits, zero-padded — control chars only, so the high bytes
                // are always zero.
                let code = c as u32;
                for shift in [12u32, 8, 4, 0] {
                    let nibble = (code >> shift) & 0xf;
                    out.push(char::from_digit(nibble, 16).unwrap_or('0'));
                }
            }
            c => out.push(c),
        }
    }
}
