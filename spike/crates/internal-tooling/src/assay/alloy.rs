//! The little of Alloy assay reads: tokens, top-level items split by brace balance, and each
//! item's head and binders (`notes/30Y` § 3). Nothing else is parsed; Alloy parses the rest.

use std::collections::BTreeSet;
use std::fmt::Write as _;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Tok {
    Ident,
    Number,
    Punct,
    Str,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Token {
    pub(super) kind: Tok,
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) line: usize,
}

/// Alloy 6's reserved words, the temporal set included (`30Ya-strawman-3/FINDINGS.md`
/// `fight-before-is-a-keyword`), plus the builtin names no document declares.
const RESERVED: &[&str] = &[
    "abstract",
    "all",
    "and",
    "as",
    "assert",
    "but",
    "check",
    "disj",
    "else",
    "enum",
    "exactly",
    "expect",
    "extends",
    "fact",
    "for",
    "fun",
    "iden",
    "iff",
    "implies",
    "in",
    "Int",
    "int",
    "let",
    "lone",
    "module",
    "no",
    "none",
    "not",
    "one",
    "open",
    "or",
    "pred",
    "private",
    "run",
    "seq",
    "set",
    "sig",
    "some",
    "String",
    "sum",
    "this",
    "univ",
    "after",
    "always",
    "before",
    "eventually",
    "historically",
    "once",
    "releases",
    "since",
    "steps",
    "triggered",
    "until",
    "var",
];

pub(super) fn is_reserved(word: &str) -> bool {
    RESERVED.contains(&word)
}

/// A name assay may mint or accept as a word: an Alloy identifier that is not reserved.
pub(super) fn is_plain_identifier(word: &str) -> bool {
    let mut chars = word.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !is_reserved(word)
}

/// Readable escapes for the shell punctuation a map line carries; every other non-alphanumeric
/// character escapes as `u` and its code point in hex, which no entry here spells.
const MNEMONICS: &[(char, &str)] = &[
    (' ', "sp"),
    ('!', "bang"),
    ('"', "dq"),
    ('#', "hash"),
    ('$', "dollar"),
    ('%', "pct"),
    ('&', "amp"),
    ('\'', "sq"),
    ('(', "lp"),
    (')', "rp"),
    ('*', "star"),
    ('+', "plus"),
    (',', "comma"),
    ('-', "dash"),
    ('.', "dot"),
    ('/', "slash"),
    (':', "colon"),
    (';', "semi"),
    ('<', "lt"),
    ('=', "eq"),
    ('>', "gt"),
    ('?', "qm"),
    ('@', "at"),
    ('[', "lb"),
    ('\\', "bs"),
    (']', "rb"),
    ('^', "caret"),
    ('`', "bq"),
    ('{', "lc"),
    ('|', "pipe"),
    ('}', "rc"),
    ('~', "tilde"),
    ('\t', "tab"),
    ('\n', "nl"),
];

/// The atom a map name spells (`notes/30Y` § 2.1): an Alloy identifier is itself, anything else
/// its [`munge`]. A literal nobody names is its own name.
pub(super) fn atom(name: &str) -> String {
    if is_plain_identifier(name) {
        name.to_owned()
    } else {
        munge(name)
    }
}

/// A deterministic, injective spelling of any text in Alloy's identifier alphabet.
///
/// ASCII letters and digits pass through, `_` doubles, and any other character becomes
/// `_<mnemonic>_`, so every escaped spelling carries an even number of underscores and decodes
/// left to right. Where that spelling does not start with a letter, or is a keyword, it is
/// prefixed `w_`, whose single underscore makes the count odd; the two families cannot meet, and
/// each is injective, so the whole map is.
fn munge(text: &str) -> String {
    let mut body = String::new();
    for c in text.chars() {
        if c.is_ascii_alphanumeric() {
            body.push(c);
        } else if c == '_' {
            body.push_str("__");
        } else {
            body.push('_');
            match MNEMONICS.iter().find(|(m, _)| *m == c) {
                Some((_, name)) => body.push_str(name),
                None => {
                    let _ = write!(body, "u{:x}", u32::from(c));
                }
            }
            body.push('_');
        }
    }
    if body.starts_with(|c: char| c.is_ascii_alphabetic()) && !is_reserved(&body) {
        body
    } else {
        format!("w_{body}")
    }
}

/// Tokens of `src`, whose first line is file line `first_line`. Comments vanish; strings and
/// unknown characters survive as single tokens so byte ranges stay honest.
pub(super) fn tokenize(src: &str, first_line: usize) -> Vec<Token> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    let mut line = first_line;
    while let Some(&b) = bytes.get(i) {
        let next = bytes.get(i.saturating_add(1)).copied();
        let start = i;
        if b == b'\n' {
            line = line.saturating_add(1);
            i = i.saturating_add(1);
        } else if b.is_ascii_whitespace() {
            i = i.saturating_add(1);
        } else if (b == b'-' && next == Some(b'-')) || (b == b'/' && next == Some(b'/')) {
            i = skip_while(bytes, i, |c| c != b'\n');
        } else if b == b'/' && next == Some(b'*') {
            let close = src
                .get(i..)
                .and_then(|rest| rest.find("*/"))
                .map_or(src.len(), |at| i.saturating_add(at).saturating_add(2));
            line = line.saturating_add(count_newlines(src.get(i..close).unwrap_or("")));
            i = close;
        } else if b == b'"' {
            let close = skip_while(bytes, i.saturating_add(1), |c| c != b'"');
            i = close.saturating_add(1).min(src.len());
            out.push(Token {
                kind: Tok::Str,
                start,
                end: i,
                line,
            });
        } else if b.is_ascii_alphabetic() || b == b'_' {
            i = skip_while(bytes, i, |c| {
                c.is_ascii_alphanumeric() || c == b'_' || c == b'\''
            });
            out.push(Token {
                kind: Tok::Ident,
                start,
                end: i,
                line,
            });
        } else if b.is_ascii_digit() {
            i = skip_while(bytes, i, |c| c.is_ascii_digit());
            out.push(Token {
                kind: Tok::Number,
                start,
                end: i,
                line,
            });
        } else {
            let two = [b, next.unwrap_or(0)];
            let wide = matches!(
                &two,
                b"->" | b"=>" | b"<=" | b">=" | b"!=" | b"<:" | b":>" | b"++" | b"||" | b"&&"
            );
            let width = if wide {
                2
            } else {
                src.get(i..)
                    .and_then(|r| r.chars().next())
                    .map_or(1, char::len_utf8)
            };
            i = i.saturating_add(width);
            out.push(Token {
                kind: Tok::Punct,
                start,
                end: i,
                line,
            });
        }
    }
    out
}

fn skip_while(bytes: &[u8], from: usize, keep: impl Fn(u8) -> bool) -> usize {
    let mut i = from;
    while bytes.get(i).is_some_and(|&c| keep(c)) {
        i = i.saturating_add(1);
    }
    i
}

fn count_newlines(s: &str) -> usize {
    s.bytes().filter(|&b| b == b'\n').count()
}

/// A token's text.
pub(super) fn text<'a>(src: &'a str, tok: &Token) -> &'a str {
    src.get(tok.start..tok.end).unwrap_or("")
}

/// One top-level declaration: its tokens and its verbatim source.
#[derive(Debug, Clone)]
pub(super) struct Item {
    pub(super) toks: Vec<Token>,
    pub(super) line: usize,
    pub(super) text: String,
}

/// What the head of an item says, and nothing more.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Head {
    Sig {
        names: Vec<String>,
        one: bool,
        /// `extends P` (one parent) or `in P + Q` (subset parents).
        parents: Vec<String>,
        extends: bool,
    },
    Command {
        check: bool,
        name: Option<String>,
        /// The clause after a top-level `for`, verbatim.
        scope: Option<String>,
        /// The item's text up to (not including) that `for`.
        body: String,
    },
    Other,
}

fn is_head_start(src: &str, toks: &[Token], i: usize) -> bool {
    let Some(tok) = toks.get(i) else { return false };
    let word = text(src, tok);
    match word {
        "sig" | "abstract" | "private" | "var" | "enum" | "fact" | "fun" | "pred" | "assert"
        | "check" | "run" | "open" | "module" => tok.kind == Tok::Ident,
        "one" | "lone" | "some" => toks
            .get(i.saturating_add(1))
            .is_some_and(|t| matches!(text(src, t), "sig" | "abstract")),
        _ => false,
    }
}

/// Split a fence body into top-level items: a new item begins at depth zero where a declaration
/// keyword opens a line or follows a closing brace.
pub(super) fn items(src: &str, first_line: usize) -> Vec<Item> {
    let toks = tokenize(src, first_line);
    let mut groups: Vec<Vec<Token>> = Vec::new();
    let mut depth: usize = 0;
    let mut prev: Option<Token> = None;
    for (i, tok) in toks.iter().enumerate() {
        let opens_line = prev.is_none_or(|p| p.line != tok.line);
        let after_brace = prev.is_some_and(|p| text(src, &p) == "}");
        if depth == 0 && (opens_line || after_brace) && is_head_start(src, &toks, i)
            || groups.is_empty()
        {
            groups.push(Vec::new());
        }
        match text(src, tok) {
            "(" | "[" | "{" if tok.kind == Tok::Punct => depth = depth.saturating_add(1),
            ")" | "]" | "}" if tok.kind == Tok::Punct => depth = depth.saturating_sub(1),
            _ => {}
        }
        if let Some(group) = groups.last_mut() {
            group.push(*tok);
        }
        prev = Some(*tok);
    }
    groups
        .into_iter()
        .filter_map(|toks| {
            let first = toks.first()?;
            let last = toks.last()?;
            Some(Item {
                line: first.line,
                text: src.get(first.start..last.end)?.to_owned(),
                toks: toks
                    .iter()
                    .map(|t| Token {
                        start: t.start.saturating_sub(first.start),
                        end: t.end.saturating_sub(first.start),
                        ..*t
                    })
                    .collect(),
            })
        })
        .collect()
}

impl Item {
    pub(super) fn word(&self, i: usize) -> &str {
        self.toks.get(i).map_or("", |t| text(&self.text, t))
    }

    pub(super) fn head(&self) -> Head {
        let mut i = 0;
        let mut one = false;
        while matches!(
            self.word(i),
            "abstract" | "private" | "var" | "one" | "lone" | "some"
        ) {
            one |= self.word(i) == "one";
            i = i.saturating_add(1);
        }
        match self.word(i) {
            "sig" => self.sig_head(i.saturating_add(1), one),
            word @ ("check" | "run") => self.command_head(word == "check"),
            _ => Head::Other,
        }
    }

    fn sig_head(&self, mut i: usize, one: bool) -> Head {
        let mut names = Vec::new();
        while self.toks.get(i).is_some_and(|t| t.kind == Tok::Ident) {
            names.push(self.word(i).to_owned());
            i = i.saturating_add(1);
            if self.word(i) != "," {
                break;
            }
            i = i.saturating_add(1);
        }
        let extends = self.word(i) == "extends";
        let mut parents = Vec::new();
        if extends || self.word(i) == "in" {
            i = i.saturating_add(1);
            while self.toks.get(i).is_some_and(|t| t.kind == Tok::Ident) {
                parents.push(self.word(i).to_owned());
                if self.word(i.saturating_add(1)) != "+" {
                    break;
                }
                i = i.saturating_add(2);
            }
        }
        Head::Sig {
            names,
            one,
            parents,
            extends,
        }
    }

    fn command_head(&self, check: bool) -> Head {
        let name = self
            .toks
            .get(1)
            .filter(|t| t.kind == Tok::Ident && !matches!(text(&self.text, t), "for"))
            .map(|t| text(&self.text, t).to_owned());
        let mut depth: usize = 0;
        for (i, tok) in self.toks.iter().enumerate() {
            match text(&self.text, tok) {
                "(" | "[" | "{" => depth = depth.saturating_add(1),
                ")" | "]" | "}" => depth = depth.saturating_sub(1),
                "for" if depth == 0 && tok.kind == Tok::Ident => {
                    let body_end = i
                        .checked_sub(1)
                        .and_then(|p| self.toks.get(p))
                        .map_or(0, |t| t.end);
                    return Head::Command {
                        check,
                        name,
                        scope: self.text.get(tok.end..).map(|s| s.trim().to_owned()),
                        body: self.text.get(..body_end).unwrap_or("").to_owned(),
                    };
                }
                _ => {}
            }
        }
        Head::Command {
            check,
            name,
            scope: None,
            body: self.text.clone(),
        }
    }

    /// Every name this item declares: sig names and fields, fun/pred/assert names, enum members.
    pub(super) fn declared(&self) -> Vec<String> {
        let mut out = Vec::new();
        match self.head() {
            Head::Sig { names, .. } => {
                out.extend(names);
                out.extend(binders(&self.text, &self.toks));
            }
            Head::Command { .. } => {}
            Head::Other => {
                let mut i = 0;
                while matches!(self.word(i), "private") {
                    i = i.saturating_add(1);
                }
                match self.word(i) {
                    "fun" | "pred" | "assert" => {
                        out.push(self.word(i.saturating_add(1)).to_owned());
                    }
                    "enum" => out.extend(
                        self.toks
                            .iter()
                            .filter(|t| t.kind == Tok::Ident)
                            .skip(i.saturating_add(1))
                            .map(|t| text(&self.text, t).to_owned()),
                    ),
                    _ => {}
                }
            }
        }
        out.retain(|n| !n.is_empty());
        out
    }
}

/// Names bound in `toks`: every identifier list followed by `:` (quantifiers, comprehensions,
/// parameters, fields), and every `let` name.
pub(super) fn binders(src: &str, toks: &[Token]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut i = 0;
    while let Some(tok) = toks.get(i) {
        // `x: a.f, y: B` must not bind `f`: a binder list opens a declaration, never an expression.
        let opens_decl = i.checked_sub(1).and_then(|p| toks.get(p)).is_none_or(|p| {
            matches!(
                text(src, p),
                "all"
                    | "some"
                    | "no"
                    | "lone"
                    | "one"
                    | "sum"
                    | "disj"
                    | "var"
                    | "{"
                    | "["
                    | "("
                    | ","
                    | "|"
            )
        });
        if tok.kind == Tok::Ident && !is_reserved(text(src, tok)) && opens_decl {
            let mut names = vec![text(src, tok)];
            let mut j = i.saturating_add(1);
            while toks.get(j).is_some_and(|t| text(src, t) == ",")
                && toks
                    .get(j.saturating_add(1))
                    .is_some_and(|t| t.kind == Tok::Ident)
            {
                names.push(toks.get(j.saturating_add(1)).map_or("", |t| text(src, t)));
                j = j.saturating_add(2);
            }
            if toks.get(j).is_some_and(|t| text(src, t) == ":") {
                out.extend(names.into_iter().map(str::to_owned));
            }
        }
        if text(src, tok) == "let" {
            let mut j = i.saturating_add(1);
            while let (Some(name), Some(eq)) = (toks.get(j), toks.get(j.saturating_add(1))) {
                if name.kind != Tok::Ident || text(src, eq) != "=" {
                    break;
                }
                out.insert(text(src, name).to_owned());
                j = j.saturating_add(2);
                while toks
                    .get(j)
                    .is_some_and(|t| !matches!(text(src, t), "," | "|"))
                {
                    j = j.saturating_add(1);
                }
                if toks.get(j).is_none_or(|t| text(src, t) != ",") {
                    break;
                }
                j = j.saturating_add(1);
            }
        }
        i = i.saturating_add(1);
    }
    out
}

/// Identifiers in `src` that nothing declares and nothing binds: the names a statement
/// introduces (`notes/30Y` § 2.3), in order of first appearance.
pub(super) fn introduced(
    src: &str,
    first_line: usize,
    declared: &BTreeSet<String>,
) -> Vec<(String, usize)> {
    let toks = tokenize(src, first_line);
    let bound = binders(src, &toks);
    let mut seen = BTreeSet::new();
    toks.iter()
        .filter(|t| t.kind == Tok::Ident)
        .map(|t| (text(src, t), t.line))
        .filter(|(w, _)| !is_reserved(w) && !declared.contains(*w) && !bound.contains(*w))
        .filter(|(w, _)| seen.insert(w.to_owned()))
        .map(|(w, line)| (w.to_owned(), line))
        .collect()
}

/// Does `src` mention the identifier `this`?
pub(super) fn mentions_this(src: &str) -> bool {
    tokenize(src, 0)
        .iter()
        .any(|t| t.kind == Tok::Ident && text(src, t) == "this")
}

/// `src` with every `this` token replaced by `atom`.
pub(super) fn replace_this(src: &str, atom: &str) -> String {
    let mut out = String::new();
    let mut at = 0;
    for tok in tokenize(src, 0)
        .iter()
        .filter(|t| t.kind == Tok::Ident && text(src, t) == "this")
    {
        out.push_str(src.get(at..tok.start).unwrap_or(""));
        out.push_str(atom);
        at = tok.end;
    }
    out.push_str(src.get(at..).unwrap_or(""));
    out
}

/// `(formula, scope)`: a trailing top-level `for` clause split off an outcome.
pub(super) fn split_scope(src: &str) -> (String, Option<String>) {
    let mut depth: usize = 0;
    for tok in tokenize(src, 0) {
        match text(src, &tok) {
            "(" | "[" | "{" => depth = depth.saturating_add(1),
            ")" | "]" | "}" => depth = depth.saturating_sub(1),
            "for" if depth == 0 && tok.kind == Tok::Ident => {
                let formula = src.get(..tok.start).unwrap_or("").trim().to_owned();
                let scope = src.get(tok.end..).unwrap_or("").trim().to_owned();
                return (formula, Some(scope));
            }
            _ => {}
        }
    }
    (src.trim().to_owned(), None)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        Head, atom, binders, introduced, is_plain_identifier, items, munge, replace_this,
        split_scope, tokenize,
    };

    #[test]
    fn munges_are_distinct_identifiers_and_an_identifier_names_itself() {
        // Two names sharing an atom would silently become one word; the underscore-bearing and
        // prefixed pairs are the ones a careless escape merges.
        let nasty = [
            "-c",
            "'%i %d'",
            "2>&1",
            ">>/var/log/hork.log",
            "a_b",
            "a-b",
            "a__b",
            "443/tcp",
            "set",
            "w_set",
            "",
            "_",
            "\u{2603}",
        ];
        let names: Vec<String> = nasty.iter().map(|l| munge(l)).collect();
        for (literal, name) in nasty.iter().zip(&names) {
            assert!(
                is_plain_identifier(name),
                "{literal:?} minted {name:?}, not an identifier"
            );
        }
        let distinct: BTreeSet<&String> = names.iter().collect();
        assert_eq!(distinct.len(), names.len(), "{names:?}");
        assert_eq!(munge("-c"), "w__dash_c");
        assert_eq!(munge("a-b"), "a_dash_b");
        assert_eq!(atom("a_b"), "a_b");
        assert_eq!(atom("set"), "w_set");
    }

    fn head_of(src: &str) -> Head {
        items(src, 1).first().map_or(Head::Other, super::Item::head)
    }

    #[test]
    fn items_split_at_top_level_heads_only() {
        // A `one` inside a fun's return type and a scope clause on its own line must not open
        // items; a head after a closing brace on the same line must.
        let src = "fun f[l: Line]: one Ans {\n  l.x\n}\ncheck c { a }\n   for 3 but 4 Int\nsig A {} one sig B extends A {}";
        let got: Vec<String> = items(src, 1).into_iter().map(|i| i.text).collect();
        assert_eq!(
            got,
            vec![
                "fun f[l: Line]: one Ans {\n  l.x\n}",
                "check c { a }\n   for 3 but 4 Int",
                "sig A {}",
                "one sig B extends A {}",
            ]
        );
    }

    #[test]
    fn heads_read_names_parents_and_scope() {
        assert_eq!(
            head_of("one sig a, b extends P {} { x = y }"),
            Head::Sig {
                names: vec!["a".into(), "b".into()],
                one: true,
                parents: vec!["P".into()],
                extends: true
            }
        );
        assert_eq!(
            head_of("sig W in A + B {}"),
            Head::Sig {
                names: vec!["W".into()],
                one: false,
                parents: vec!["A".into(), "B".into()],
                extends: false
            }
        );
        assert_eq!(
            head_of("check k { all x: A | x in B } for 5 but 4 Int"),
            Head::Command {
                check: true,
                name: Some("k".into()),
                scope: Some("5 but 4 Int".into()),
                body: "check k { all x: A | x in B }".into()
            }
        );
    }

    #[test]
    fn binders_cover_quantifiers_disj_lists_and_let() {
        let src = "all disj a, b: K | some x: a.f, y: b.g | let z = x, q = y | z = q";
        let got = binders(src, &tokenize(src, 1));
        let want: BTreeSet<String> = ["a", "b", "x", "y", "z", "q"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        assert_eq!(got, want);
    }

    #[test]
    fn introduced_names_skip_keywords_binders_numbers_and_declarations() {
        let declared: BTreeSet<String> = ["w", "Key", "cells"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        let got = introduced(
            "all k: Key | k.w = frob_db and cells = 0 + 1 and this in w.frob_db + gizmo_heap",
            7,
            &declared,
        );
        assert_eq!(
            got,
            vec![("frob_db".to_owned(), 7), ("gizmo_heap".to_owned(), 7)]
        );
    }

    #[test]
    fn this_is_replaced_as_a_token_and_a_trailing_for_is_split() {
        assert_eq!(
            replace_this("{ of = this  thisish = this }", "line_3"),
            "{ of = line_3  thisish = line_3 }"
        );
        assert_eq!(
            split_scope("this in X for 3 but 2 Int"),
            ("this in X".to_owned(), Some("3 but 2 Int".to_owned()))
        );
        assert_eq!(
            split_scope("all x: A | { y: B | y in x } in X"),
            ("all x: A | { y: B | y in x } in X".to_owned(), None)
        );
    }
}
