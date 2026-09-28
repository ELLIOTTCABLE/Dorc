//! A std-only JSON writer, since `serde_json` is a dev-dependency here and the crate's dependency
//! bar (see `Cargo.toml`) is not worth spending on a report printer.

use std::fmt::Write as _;

/// A JSON value, built by hand and rendered with two-space indentation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Json {
    Null,
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
            Self::Null | Self::Num(_) | Self::Str(_) => {
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

#[cfg(test)]
mod tests {
    use super::{Json, quote, read_num, read_str};

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
