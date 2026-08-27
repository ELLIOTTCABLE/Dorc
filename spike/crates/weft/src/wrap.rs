//! Greedy line-filling over attributed runs.
//!
//! One decision here is worth naming, because it is the shape the sibling
//! prior-art converged on and the alternative is a bug farm: runs are flattened
//! into a token stream carrying attribution *per token* before any wrapping
//! happens, rather than the wrapper being taught about run boundaries. A wrap
//! point that lands inside a run is then simply a wrap point, not a special
//! case, and no attribution can be lost at a break.
//!
//! The fill itself is deliberately naive — first-fit, no lookahead, no
//! optimal-break scoring. The document-algebra machinery that would do better
//! is out of scope, and this leaves it a clean seam: everything above reads
//! geometry from the frame, so a smarter filler swaps in underneath without the
//! node layouts noticing.

use crate::frame::Frame;
use crate::provenance::{Provenance, Run};
use crate::sink::Sink;

enum Token<'a, K> {
    Word(&'a str, &'a Provenance<K>),
    Space(&'a str, &'a Provenance<K>),
}

// A token is a pair of borrows and is Copy for every `K`; the derive would
// demand `K: Copy`, which no consumer key satisfies.
impl<K> Clone for Token<'_, K> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<K> Copy for Token<'_, K> {}

/// Splits runs into maximal word and whitespace tokens, each keeping its run's
/// authorship.
fn tokenize<K>(runs: &[Run<K>]) -> Vec<Token<'_, K>> {
    let mut tokens = Vec::new();
    for run in runs {
        let mut rest = run.text.as_str();
        while !rest.is_empty() {
            let leading_space = rest.starts_with(char::is_whitespace);
            let split = rest
                .find(|character: char| character.is_whitespace() != leading_space)
                .unwrap_or(rest.len());
            let (head, tail) = rest.split_at(split);
            tokens.push(if leading_space {
                Token::Space(head, &run.provenance)
            } else {
                Token::Word(head, &run.provenance)
            });
            rest = tail;
        }
    }
    tokens
}

/// The width the runs would occupy on one unbroken line.
pub(crate) fn runs_width<K>(runs: &[Run<K>]) -> usize {
    runs.iter()
        .fold(0, |total, run| total.saturating_add(run.text.len()))
}

/// Emits runs verbatim at the current position, without wrapping.
pub(crate) fn emit_runs<K: Clone>(sink: &mut Sink<K>, runs: &[Run<K>]) {
    for run in runs {
        sink.run(&run.text, &run.provenance);
    }
}

/// Fills runs into the frame, breaking at whitespace.
///
/// Content is never dropped to satisfy the width: a word wider than its line
/// overruns the right edge rather than being silently cut. Truncation on this
/// surface must always be a visible, consumer-authored act.
///
/// The frame's line numbering starts at the line the sink is currently on, so a
/// caller that has already emitted a label or a column prefix gets a hanging
/// indent for free — the first line simply begins wherever the cursor already
/// is, and later lines pad to the frame's left edge.
/// A word is a word however many runs contributed it.
///
/// Adjacent word tokens with no whitespace between them are one unbreakable
/// unit, even when they came from different runs — a consumer that splits
/// `line 12:` into a prose run, a value run and a punctuation run has changed
/// the attribution, not the text, and a break weft inserted at one of those
/// seams would be a line-break the consumer never wrote.
fn chunk_end<K>(tokens: &[Token<'_, K>], start: usize) -> usize {
    tokens
        .get(start..)
        .unwrap_or_default()
        .iter()
        .position(|token| matches!(token, Token::Space(..)))
        .map_or(tokens.len(), |offset| start.saturating_add(offset))
}

fn chunk_width<K>(tokens: &[Token<'_, K>]) -> usize {
    tokens.iter().fold(0, |total, token| match token {
        Token::Word(text, _) => total.saturating_add(text.len()),
        Token::Space(..) => total,
    })
}

pub(crate) fn wrap<K: Clone>(sink: &mut Sink<K>, runs: &[Run<K>], frame: &Frame) {
    let mut line = 0usize;
    let mut pending: Option<(&str, &Provenance<K>)> = None;
    // Continuing a line that already has content, rather than starting one, is
    // what makes hanging indents and post-quote trailers fall out for free —
    // and it is why leading whitespace survives there but is dropped at the
    // start of a fresh line, where it would just be indent noise.
    let mut placed = !sink.line_is_empty();

    let tokens = tokenize(runs);
    let mut index = 0usize;
    while let Some(&token) = tokens.get(index) {
        match token {
            Token::Space(text, provenance) => {
                if text.bytes().filter(|byte| *byte == b'\n').count() >= 2 {
                    sink.blank_line();
                    line = line.saturating_add(2);
                    pending = None;
                    placed = false;
                    index = index.saturating_add(1);
                    continue;
                }
                if placed {
                    pending = Some((if text.contains('\n') { " " } else { text }, provenance));
                }
                index = index.saturating_add(1);
            }
            Token::Word(..) => {
                let end = chunk_end(&tokens, index);
                let chunk = tokens.get(index..end).unwrap_or_default();
                let (left, right) = frame.usable(line);
                if placed {
                    let gap = pending.map_or(0, |(space, _)| space.len());
                    let stop = sink
                        .column()
                        .saturating_add(gap)
                        .saturating_add(chunk_width(chunk));
                    if stop > right {
                        sink.newline();
                        line = line.saturating_add(1);
                        let (next_left, _) = frame.usable(line);
                        sink.pad_to(next_left);
                        pending = None;
                    }
                } else {
                    sink.pad_to(left);
                }
                if let Some((space, space_provenance)) = pending.take() {
                    sink.run(space, space_provenance);
                }
                for &token in chunk {
                    if let Token::Word(text, provenance) = token {
                        sink.run(text, provenance);
                    }
                }
                placed = true;
                index = end;
            }
        }
    }
}
