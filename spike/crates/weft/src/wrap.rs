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

#[derive(Clone, Copy)]
enum Token<'a, K> {
    Word(&'a str, &'a Provenance<K>),
    Space(&'a str, &'a Provenance<K>),
}

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
pub(crate) fn wrap<K: Clone>(sink: &mut Sink<K>, runs: &[Run<K>], frame: &Frame) {
    let mut line = 0usize;
    let mut pending: Option<(&str, &Provenance<K>)> = None;
    let mut placed = false;

    for token in tokenize(runs) {
        match token {
            Token::Space(text, provenance) => {
                if placed {
                    pending = Some((text, provenance));
                }
            }
            Token::Word(text, provenance) => {
                let (left, right) = frame.usable(line);
                if placed {
                    let gap = pending.map_or(0, |(space, _)| space.len());
                    let end = sink
                        .column()
                        .saturating_add(gap)
                        .saturating_add(text.len());
                    if end > right {
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
                sink.run(text, provenance);
                placed = true;
            }
        }
    }
}
