//! Criticality weights — the line-count stand-in for the future 1A-matrix weights.
//!
//! The north-star number is **criticality-weighted** (`211` §1: "count- AND
//! criticality-weighted (1A matrix when it lands; line-count stand-in)"). Until the
//! round-1A capability matrix lands real per-site criticality, the stand-in is the
//! site's line-span (a one-line command weighs 1; a heredoc-bearing or multi-line
//! command weighs its line-count) — a crude proxy for "how much of the book this site
//! represents".
//!
//! This module is the CLEARLY-MARKED adapter seam: when the 1A weights land (as
//! per-line or per-site criticality scores from `.claude/worktrees/ai-r1A-H2SALS`),
//! [`Weights::from_line_scores`] is the single swap-point; nothing else in the crate
//! knows where a weight came from.

use std::collections::BTreeMap;

/// Per-line criticality weights. The default (empty map) yields weight 1 for every
/// line (the pure count stand-in — every site weighs the same). A non-empty map (from
/// the future 1A matrix, via [`Weights::from_line_scores`]) overrides specific lines.
#[derive(Debug, Clone, Default)]
pub struct Weights {
    /// 1-based source line → criticality weight. Absent ⇒ the [`Self::default_weight`].
    by_line: BTreeMap<u32, u32>,
    /// The weight for a line not in `by_line` (1 for the count stand-in).
    default_weight: u32,
}

impl Weights {
    /// The line-count stand-in: every site weighs `1` (so criticality-weighted ==
    /// count-weighted until real weights land). This is today's only mode.
    #[must_use]
    pub fn line_count_standin() -> Self {
        Self {
            by_line: BTreeMap::new(),
            default_weight: 1,
        }
    }

    /// **The 1A adapter seam** (FUTURE — not wired today): build weights from the
    /// round-1A matrix's per-line criticality scores. When that artifact lands at
    /// `.claude/worktrees/ai-r1A-H2SALS`, parse it into `line → score` and hand it
    /// here; the rest of the crate is unchanged. `default` is the weight for a line the
    /// matrix did not score (a structural/blank line — typically 0 or 1).
    #[must_use]
    pub fn from_line_scores(by_line: BTreeMap<u32, u32>, default: u32) -> Self {
        Self {
            by_line,
            default_weight: default,
        }
    }

    /// The criticality weight for a 1-based source line.
    #[must_use]
    pub fn weight_for_line(&self, line: u32) -> u32 {
        self.by_line
            .get(&line)
            .copied()
            .unwrap_or(self.default_weight)
    }

    /// The criticality weight for a site spanning 1-based lines `[start, end]`
    /// inclusive — the SUM of its lines' per-line weights. For the count stand-in
    /// (every line 1) this is the site's line-count (`20V`-charter: "criticality =
    /// line-count stand-in"), so a one-liner weighs 1 and a 5-line heredoc weighs 5.
    /// Clamped to ≥1 (a degenerate `end < start` still weighs its start line).
    #[must_use]
    pub fn weight_for_span(&self, start: u32, end: u32) -> u32 {
        let end = end.max(start);
        (start..=end)
            .map(|l| self.weight_for_line(l))
            .fold(0u32, u32::saturating_add)
            .max(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_count_standin_is_uniform_one() {
        // The default stand-in: every line weighs 1, so criticality == count today.
        let w = Weights::line_count_standin();
        assert_eq!(w.weight_for_line(1), 1);
        assert_eq!(w.weight_for_line(999), 1);
    }

    #[test]
    fn from_line_scores_overrides_specific_lines() {
        // The 1A adapter seam: scored lines override, unscored fall to the default.
        let mut scores = BTreeMap::new();
        scores.insert(10, 5);
        scores.insert(20, 3);
        let w = Weights::from_line_scores(scores, 1);
        assert_eq!(w.weight_for_line(10), 5, "scored line uses its score");
        assert_eq!(w.weight_for_line(20), 3);
        assert_eq!(w.weight_for_line(15), 1, "unscored line uses the default");
    }
}
