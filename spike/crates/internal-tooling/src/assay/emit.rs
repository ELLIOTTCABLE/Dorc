//! Generated modules carry no comments and one item per line (`notes/30Yf` § 3, W1): every gap
//! between two tokens becomes one space, adjacent tokens stay adjacent, and where each stretch of a
//! line came from goes to the sidecar instead. The key hashes these emitted bytes, the very bytes
//! Alloy parses, so a defect here shows as a parse error or a moved verdict, never a silent match.

use super::alloy::{self, Token};
use crate::json::Json;

/// Marks the start of an emitted block in the emitter's own internal text: `\u{1}file\u{1}line`.
/// A spec carrying this byte is refused, so it can only ever be ours.
pub(super) const ORIGIN: char = '\u{1}';

/// Where a stretch of a generated line came from: a spec file and line, or assay itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Segment {
    pub(super) column: usize,
    pub(super) file: String,
    pub(super) line: usize,
}

/// One generated module: its bytes, and a segment list per line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Rendered {
    pub(super) text: String,
    pub(super) map: Vec<Vec<Segment>>,
}

/// Render the emitter's internal text, or say which spec line holds what the tokenizer is unsure of.
pub(super) fn render(internal: &str) -> Result<Rendered, (String, usize, &'static str)> {
    let mut blocks: Vec<(String, usize, String)> = vec![("assay".to_owned(), 0, String::new())];
    for line in internal.split('\n') {
        if let Some(header) = line.strip_prefix(ORIGIN) {
            let (file, at) = header.split_once(ORIGIN).unwrap_or((header, "0"));
            blocks.push((file.to_owned(), at.parse().unwrap_or(0), String::new()));
        } else if let Some((_, _, text)) = blocks.last_mut() {
            text.push_str(line);
            text.push('\n');
        }
    }
    let mut out = Rendered {
        text: String::new(),
        map: Vec::new(),
    };
    for (file, first, text) in &blocks {
        if let Some(why) =
            alloy::unsure(text).or_else(|| text.contains(ORIGIN).then_some("a control byte"))
        {
            return Err((file.clone(), *first, why));
        }
        for item in alloy::items(text, *first) {
            let (line, segments) = one_line(&item.text, &item.toks, file);
            out.text.push_str(&line);
            out.text.push('\n');
            out.map.push(segments);
        }
    }
    Ok(out)
}

fn one_line(src: &str, toks: &[Token], file: &str) -> (String, Vec<Segment>) {
    let mut line = String::new();
    let mut segments: Vec<Segment> = Vec::new();
    let mut prev: Option<&Token> = None;
    for tok in toks {
        if prev.is_some_and(|p| p.end < tok.start) {
            line.push(' ');
        }
        if segments.last().is_none_or(|s| s.line != tok.line) {
            segments.push(Segment {
                column: line.chars().count().saturating_add(1),
                file: file.to_owned(),
                line: tok.line,
            });
        }
        line.push_str(alloy::text(src, tok));
        prev = Some(tok);
    }
    (line, segments)
}

impl Rendered {
    /// One JSON array per generated line, of `[column, file, line]` segments.
    pub(super) fn map_json_lines(&self) -> Vec<String> {
        let num = |n: usize| Json::Num(u64::try_from(n).unwrap_or(u64::MAX));
        self.map
            .iter()
            .map(|segs| {
                Json::Arr(
                    segs.iter()
                        .map(|s| Json::Arr(vec![num(s.column), Json::str(&s.file), num(s.line)]))
                        .collect(),
                )
                .line()
            })
            .collect()
    }

    /// The spec position a generated line and column came from, for walking an Alloy message back.
    pub(super) fn origin(&self, line: usize, column: usize) -> Option<&Segment> {
        self.map
            .get(line.checked_sub(1)?)?
            .iter()
            .rev()
            .find(|s| s.column <= column.max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::{ORIGIN, render};

    fn block(file: &str, line: usize, text: &str) -> String {
        format!("{ORIGIN}{file}{ORIGIN}{line}\n{text}")
    }

    #[test]
    fn gaps_collapse_to_one_space_and_adjacency_survives() {
        // `this/A` and `1..3` must stay glued: a space inside either is another program.
        let internal = format!(
            "module m\nopen util/ordering[A]\n\n{}\n{}",
            block(
                "s.md",
                10,
                "fact  x   {\n   this/A.f  in  \"a  b\" -- note\n}"
            ),
            block("s.md", 20, "run r {} for 3 but 1..3 steps /* c */")
        );
        let got = render(&internal).expect("renders");
        assert_eq!(
            got.text,
            "module m\nopen util/ordering[A]\nfact x { this/A.f in \"a  b\" }\nrun r {} for 3 but 1..3 steps\n"
        );
        let lines: Vec<Vec<(usize, usize)>> = got
            .map
            .iter()
            .map(|segs| segs.iter().map(|s| (s.column, s.line)).collect())
            .collect();
        assert_eq!(lines[2], vec![(1, 10), (10, 11), (29, 12)]);
        assert_eq!(got.origin(3, 12).map(|s| s.line), Some(11));
    }

    #[test]
    fn what_the_tokenizer_cannot_split_refuses() {
        for bad in ["fact { \"open }", "fact { /* x }", "fact { x\"y\" }"] {
            assert!(render(&block("s.md", 3, bad)).is_err(), "{bad}");
        }
    }
}
