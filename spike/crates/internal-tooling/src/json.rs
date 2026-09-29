//! A std-only JSON writer, since `serde_json` is a dev-dependency here and the crate's dependency
//! bar (see `Cargo.toml`) is not worth spending on a report printer.

use std::fmt::Write as _;

/// A JSON value, built by hand and rendered with two-space indentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Json {
    Null,
    Bool(bool),
    Num(u64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    pub(crate) fn str(s: impl Into<String>) -> Self {
        Self::Str(s.into())
    }

    pub(crate) fn obj<const N: usize>(fields: [(&str, Json); N]) -> Self {
        Self::Obj(fields.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
    }

    /// One line, no trailing newline: a row in a file diffed line by line.
    pub(crate) fn line(&self) -> String {
        match self {
            Self::Arr(items) => {
                let items: Vec<String> = items.iter().map(Self::line).collect();
                format!("[{}]", items.join(", "))
            }
            Self::Obj(fields) => {
                let fields: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", quote(k), v.line()))
                    .collect();
                format!("{{{}}}", fields.join(", "))
            }
            Self::Null | Self::Bool(_) | Self::Num(_) | Self::Str(_) => {
                let mut out = String::new();
                self.write(&mut out, 0);
                out
            }
        }
    }

    pub(crate) fn render(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, 0);
        out.push('\n');
        out
    }

    fn write(&self, out: &mut String, depth: usize) {
        let pad = |out: &mut String, d: usize| out.push_str(&"  ".repeat(d));
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Self::Num(n) => out.push_str(&n.to_string()),
            Self::Str(s) => out.push_str(&quote(s)),
            Self::Arr(items) if items.is_empty() => out.push_str("[]"),
            Self::Obj(fields) if fields.is_empty() => out.push_str("{}"),
            Self::Arr(items) => {
                out.push_str("[\n");
                for (i, item) in items.iter().enumerate() {
                    pad(out, depth.saturating_add(1));
                    item.write(out, depth.saturating_add(1));
                    out.push_str(if i.saturating_add(1) == items.len() {
                        "\n"
                    } else {
                        ",\n"
                    });
                }
                pad(out, depth);
                out.push(']');
            }
            Self::Obj(fields) => {
                out.push_str("{\n");
                for (i, (key, value)) in fields.iter().enumerate() {
                    pad(out, depth.saturating_add(1));
                    out.push_str(&quote(key));
                    out.push_str(": ");
                    value.write(out, depth.saturating_add(1));
                    out.push_str(if i.saturating_add(1) == fields.len() {
                        "\n"
                    } else {
                        ",\n"
                    });
                }
                pad(out, depth);
                out.push('}');
            }
        }
    }
}

/// A JSON string literal.
pub(crate) fn quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len().saturating_add(2));
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if u32::from(c) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", u32::from(c));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The string value of `"key": "…"` in a flat object this crate wrote itself, or `None`.
/// Not a JSON parser: it reads back exactly what [`quote`] writes.
pub(crate) fn read_str(text: &str, key: &str) -> Option<String> {
    let rest = after_key(text, key)?.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(out),
            '\\' => match chars.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'u' => {
                    let hex: String = chars.by_ref().take(4).collect();
                    out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
                }
                other => out.push(other),
            },
            c => out.push(c),
        }
    }
    None
}

/// The number value of `"key": 123` in a flat object this crate wrote itself.
pub(crate) fn read_num(text: &str, key: &str) -> Option<u64> {
    let rest = after_key(text, key)?;
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

/// The first quoted `key` followed by a colon: a string VALUE spelling the key is followed by a
/// comma or a brace instead, and must not shadow the real key after it.
fn after_key<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    let quoted = quote(key);
    text.match_indices(&quoted).find_map(|(at, _)| {
        let rest = text.get(at.saturating_add(quoted.len())..)?;
        Some(rest.trim_start().strip_prefix(':')?.trim_start())
    })
}

/// A parsed JSON value, for replies that nest (the Alloy adapter's). A number keeps its source
/// spelling, so re-rendering one is exact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<Value>),
    Obj(Vec<(String, Value)>),
}

impl Value {
    /// One whole JSON text, or `None` for anything malformed or trailing.
    pub(crate) fn parse(text: &str) -> Option<Self> {
        let mut p = Parser {
            bytes: text.as_bytes(),
            at: 0,
        };
        let value = p.value(0)?;
        p.space();
        (p.at == p.bytes.len()).then_some(value)
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Obj(fields) => fields.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    pub(crate) fn str(&self, key: &str) -> Option<&str> {
        match self.get(key)? {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub(crate) fn u64(&self, key: &str) -> Option<u64> {
        match self.get(key)? {
            Self::Num(n) => n.parse().ok(),
            _ => None,
        }
    }

    pub(crate) fn bool(&self, key: &str) -> Option<bool> {
        match self.get(key)? {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub(crate) fn arr(&self, key: &str) -> &[Self] {
        match self.get(key) {
            Some(Self::Arr(items)) => items,
            _ => &[],
        }
    }

    /// Compact, in source order: the canonical spelling a key hashes.
    pub(crate) fn compact(&self) -> String {
        match self {
            Self::Null => "null".to_owned(),
            Self::Bool(b) => b.to_string(),
            Self::Num(n) => n.clone(),
            Self::Str(s) => quote(s),
            Self::Arr(items) => {
                let items: Vec<String> = items.iter().map(Self::compact).collect();
                format!("[{}]", items.join(","))
            }
            Self::Obj(fields) => {
                let fields: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}:{}", quote(k), v.compact()))
                    .collect();
                format!("{{{}}}", fields.join(","))
            }
        }
    }
}

/// Nesting deeper than this is refused rather than recursed into.
const MAX_DEPTH: usize = 64;

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.at = self.at.saturating_add(1);
        Some(b)
    }

    fn space(&mut self) {
        while self.peek().is_some_and(|b| b.is_ascii_whitespace()) {
            self.at = self.at.saturating_add(1);
        }
    }

    fn literal(&mut self, word: &str, value: Value) -> Option<Value> {
        let end = self.at.checked_add(word.len())?;
        (self.bytes.get(self.at..end)? == word.as_bytes()).then(|| {
            self.at = end;
            value
        })
    }

    fn value(&mut self, depth: usize) -> Option<Value> {
        if depth > MAX_DEPTH {
            return None;
        }
        self.space();
        match self.peek()? {
            b'n' => self.literal("null", Value::Null),
            b't' => self.literal("true", Value::Bool(true)),
            b'f' => self.literal("false", Value::Bool(false)),
            b'"' => self.string().map(Value::Str),
            b'[' => {
                self.bump();
                let mut items = Vec::new();
                self.space();
                if self.peek() == Some(b']') {
                    self.bump();
                    return Some(Value::Arr(items));
                }
                loop {
                    items.push(self.value(depth.saturating_add(1))?);
                    self.space();
                    match self.bump()? {
                        b',' => {}
                        b']' => return Some(Value::Arr(items)),
                        _ => return None,
                    }
                }
            }
            b'{' => {
                self.bump();
                let mut fields = Vec::new();
                self.space();
                if self.peek() == Some(b'}') {
                    self.bump();
                    return Some(Value::Obj(fields));
                }
                loop {
                    self.space();
                    let key = self.string()?;
                    self.space();
                    (self.bump()? == b':').then_some(())?;
                    fields.push((key, self.value(depth.saturating_add(1))?));
                    self.space();
                    match self.bump()? {
                        b',' => {}
                        b'}' => return Some(Value::Obj(fields)),
                        _ => return None,
                    }
                }
            }
            _ => {
                let start = self.at;
                while self.peek().is_some_and(|b| {
                    b.is_ascii_digit() || matches!(b, b'-' | b'+' | b'.' | b'e' | b'E')
                }) {
                    self.at = self.at.saturating_add(1);
                }
                let text = std::str::from_utf8(self.bytes.get(start..self.at)?).ok()?;
                text.parse::<f64>().ok()?;
                Some(Value::Num(text.to_owned()))
            }
        }
    }

    fn string(&mut self) -> Option<String> {
        (self.bump()? == b'"').then_some(())?;
        let mut out: Vec<u8> = Vec::new();
        loop {
            match self.bump()? {
                b'"' => return String::from_utf8(out).ok(),
                b'\\' => {
                    let c = match self.bump()? {
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'b' => '\u{8}',
                        b'f' => '\u{c}',
                        b'u' => {
                            let hi = self.hex4()?;
                            if (0xd800..0xdc00).contains(&hi) {
                                (self.bump()? == b'\\' && self.bump()? == b'u').then_some(())?;
                                let lo = self.hex4()?;
                                let code = 0x10000_u32
                                    .checked_add(hi.checked_sub(0xd800)?.checked_shl(10)?)?
                                    .checked_add(lo.checked_sub(0xdc00)?)?;
                                char::from_u32(code)?
                            } else {
                                char::from_u32(hi)?
                            }
                        }
                        other => char::from(other),
                    };
                    let mut buf = [0u8; 4];
                    out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
                }
                b => out.push(b),
            }
        }
    }

    fn hex4(&mut self) -> Option<u32> {
        let end = self.at.checked_add(4)?;
        let text = std::str::from_utf8(self.bytes.get(self.at..end)?).ok()?;
        self.at = end;
        u32::from_str_radix(text, 16).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{Json, Value, quote, read_num, read_str};

    #[test]
    fn a_nested_reply_parses_and_re_renders_exactly() {
        // The adapter's options object is hashed into every key, so its compact spelling must be
        // a function of the reply, byte for byte.
        let text = r#" {"ok": true, "unrolls": -1, "loaded": [{"path": "a\\b", "text": "x\ny é 😀"}], "none": null} "#;
        let v = Value::parse(text).expect("the reply should parse");
        assert_eq!(v.bool("ok"), Some(true));
        assert_eq!(v.get("unrolls"), Some(&Value::Num("-1".to_owned())));
        assert_eq!(
            v.arr("loaded")[0].str("text"),
            Some("x\ny \u{e9} \u{1f600}")
        );
        assert_eq!(
            v.compact(),
            r#"{"ok":true,"unrolls":-1,"loaded":[{"path":"a\\b","text":"x\ny é 😀"}],"none":null}"#
        );
        assert_eq!(Value::parse("{\"a\": 1} x"), None, "trailing bytes refuse");
        assert_eq!(Value::parse(&"[".repeat(100)), None, "depth is bounded");
    }

    #[test]
    fn a_written_string_reads_back_exactly() {
        // The lock file round-trips a Windows cwd; a lossy read would name the wrong holder.
        let tricky = r#"C:\Users\x "q"	tab"#;
        let text = Json::obj([("cwd", Json::str(tricky)), ("pid", Json::Num(42))]).render();
        assert_eq!(read_str(&text, "cwd").as_deref(), Some(tricky));
        assert_eq!(read_num(&text, "pid"), Some(42));
        assert_eq!(read_str(&text, "absent"), None);
        let shadowed = Json::obj([("name", Json::str("result")), ("result", Json::str("sat"))]);
        assert_eq!(read_str(&shadowed.line(), "result").as_deref(), Some("sat"));
        assert_eq!(quote("\u{1}"), "\"\\u0001\"");
    }
}
