//! Which bytes of an edited render belong to which section.
//!
//! An edited transcript arrives as one string. Before any section can be compiled, each
//! section's bytes have to be cut out of it, and the only things that can do the cutting are
//! the IMMUTABLE runs between the sections — the renderer's own layout and its fixed values.
//!
//! The obvious implementation takes the FIRST occurrence of the run that follows a section and
//! refuses when that run appears again later. Both halves are wrong in the same direction. The
//! immutable runs in a laid-out report are exactly the strings that recur — `"\n   "`, `": "`,
//! `", "` — so first-occurrence lands in the wrong place, and refuse-if-it-recurs makes the
//! middle of any report with repeated chrome permanently unaddressable. The more structure a
//! render grows, the less of it can be edited.
//!
//! So the cut is an ALIGNMENT over the whole component sequence rather than a search per
//! section: find every assignment of byte ranges to sections that reproduces the immutable runs
//! in order, prefer the one that leaves the most sections untouched, and answer only when that
//! one is unique. An edit is normally to a single section, so the true assignment is the one
//! that explains it with the fewest changed sections — and a render whose chrome genuinely
//! cannot distinguish two readings refuses, instead of silently picking one.

use std::collections::BTreeMap;
use std::ops::Range;

use crate::editable::{EditableFragment, EditableRender, RenderComponent};

/// Why an edited render cannot be cut into its sections.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum SectionAddressRefusal {
    /// No assignment reproduces the render's immutable runs, in order. Something outside an
    /// editable section changed, or a section's own boundary was overwritten.
    Unanchored,
    /// Several assignments are equally good, so which section was edited is genuinely unknown.
    Ambiguous,
    /// The alignment exhausted its work budget.
    LimitExceeded,
}

/// The default work budget: candidate positions examined across one whole alignment.
pub const DEFAULT_ADDRESS_WORK_CEILING: usize = 1_000_000;

/// Cut `edited` into one byte range per editable section of `baseline`, in render order.
///
/// # Errors
/// Returns [`SectionAddressRefusal`] when no assignment exists, when the best one is not unique,
/// or when the search exceeds [`DEFAULT_ADDRESS_WORK_CEILING`].
pub fn address_sections<S, V>(
    baseline: &EditableRender<S, V>,
    edited: &str,
) -> Result<Vec<Range<usize>>, SectionAddressRefusal> {
    address_sections_within(baseline, edited, DEFAULT_ADDRESS_WORK_CEILING)
}

/// [`address_sections`] under a caller-chosen work budget.
///
/// # Errors
/// See [`address_sections`].
pub fn address_sections_within<S, V>(
    baseline: &EditableRender<S, V>,
    edited: &str,
    work_ceiling: usize,
) -> Result<Vec<Range<usize>>, SectionAddressRefusal> {
    let (anchors, unedited) = split_on_sections(baseline);
    let sections = unedited.len();
    let Some(first) = anchors.first() else {
        return Err(SectionAddressRefusal::Unanchored);
    };
    if !edited.starts_with(first.as_str()) {
        return Err(SectionAddressRefusal::Unanchored);
    }

    let mut level: BTreeMap<usize, Reading> = BTreeMap::new();
    level.insert(first.len(), Reading::start());
    let mut work = 0usize;
    for index in 0..sections {
        let anchor = anchors
            .get(index.saturating_add(1))
            .map_or("", String::as_str);
        let want = unedited.get(index).map_or("", String::as_str);
        let mut next: BTreeMap<usize, Reading> = BTreeMap::new();
        for (start, reading) in &level {
            for cut in occurrences(edited, anchor, *start) {
                work = work.saturating_add(1);
                if work > work_ceiling {
                    return Err(SectionAddressRefusal::LimitExceeded);
                }
                let interior = edited.get(*start..cut).unwrap_or_default();
                let cost = reading.cost.saturating_add(usize::from(interior != want));
                let resumed = cut.saturating_add(anchor.len());
                admit(&mut next, resumed, reading.extended(*start..cut, cost));
            }
        }
        level = next;
    }

    let Some(complete) = level.get(&edited.len()) else {
        return Err(SectionAddressRefusal::Unanchored);
    };
    if complete.readings > 1 {
        return Err(SectionAddressRefusal::Ambiguous);
    }
    Ok(complete.ranges.clone())
}

/// One partial assignment: what it cost, how many distinct ways reached it, and the cut it made.
#[derive(Clone)]
struct Reading {
    cost: usize,
    /// Distinct minimum-cost ways of reaching this state, saturating at two — one is unique and
    /// anything above it is ambiguous, so counting further buys nothing.
    readings: usize,
    ranges: Vec<Range<usize>>,
}

impl Reading {
    fn start() -> Self {
        Self {
            cost: 0,
            readings: 1,
            ranges: Vec::new(),
        }
    }

    fn extended(&self, range: Range<usize>, cost: usize) -> Self {
        let mut ranges = self.ranges.clone();
        ranges.push(range);
        Self {
            cost,
            readings: self.readings,
            ranges,
        }
    }
}

/// Keep the cheapest reading of a state, and remember when the cheapest is not alone.
fn admit(level: &mut BTreeMap<usize, Reading>, position: usize, reading: Reading) {
    match level.get_mut(&position) {
        Some(existing) if existing.cost < reading.cost => {}
        Some(existing) if existing.cost == reading.cost => {
            existing.readings = existing.readings.saturating_add(reading.readings).min(2);
        }
        _ => {
            level.insert(position, reading);
        }
    }
}

/// The render's immutable runs (one more than there are sections) and each section's own bytes.
fn split_on_sections<S, V>(baseline: &EditableRender<S, V>) -> (Vec<String>, Vec<String>) {
    let mut anchors = Vec::new();
    let mut sections = Vec::new();
    let mut immutable = String::new();
    for component in baseline.components() {
        match component {
            RenderComponent::EditableSection(section) => {
                anchors.push(std::mem::take(&mut immutable));
                sections.push(
                    section
                        .fragments()
                        .iter()
                        .map(|fragment| match fragment {
                            EditableFragment::Text(text)
                            | EditableFragment::Variable { rendered: text, .. } => text.as_str(),
                        })
                        .collect(),
                );
            }
            RenderComponent::Structure(text)
            | RenderComponent::FixedVariable { rendered: text, .. } => immutable.push_str(text),
        }
    }
    anchors.push(immutable);
    (anchors, sections)
}

/// Where `anchor` can start, at or after `from`.
///
/// An EMPTY anchor is two sections with nothing between them: every character boundary is a
/// candidate, and the alignment's own cost rule is what picks among them.
fn occurrences(text: &str, anchor: &str, from: usize) -> Vec<usize> {
    let Some(rest) = text.get(from..) else {
        return Vec::new();
    };
    if anchor.is_empty() {
        return rest
            .char_indices()
            .map(|(offset, _)| from.saturating_add(offset))
            .chain(std::iter::once(text.len()))
            .collect();
    }
    rest.match_indices(anchor)
        .map(|(offset, _)| from.saturating_add(offset))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editable::EditableSection;

    fn render(
        parts: Vec<RenderComponent<&'static str, &'static str>>,
    ) -> EditableRender<&'static str, &'static str> {
        EditableRender::new(parts)
    }

    fn section(id: &'static str, text: &str) -> RenderComponent<&'static str, &'static str> {
        RenderComponent::EditableSection(EditableSection::new(
            id,
            vec![EditableFragment::Text(text.to_owned())],
        ))
    }

    fn structure(text: &str) -> RenderComponent<&'static str, &'static str> {
        RenderComponent::Structure(text.to_owned())
    }

    /// The failure this module replaces: three sections fenced by an anchor that RECURS. First
    /// occurrence lands in the wrong place and refuse-on-recurrence gives up entirely, but the
    /// sequence pins each section exactly.
    #[test]
    fn a_recurring_anchor_still_addresses_every_section() {
        let baseline = render(vec![
            structure("\n   "),
            section("a", "first"),
            structure("\n   "),
            section("b", "second"),
            structure("\n   "),
            section("c", "third"),
            structure("\n"),
        ]);
        let edited = "\n   first\n   SECOND EDITED\n   third\n";
        let ranges = address_sections(&baseline, edited).expect("the sequence pins the cut");
        let cut: Vec<&str> = ranges
            .iter()
            .map(|range| edited.get(range.clone()).unwrap_or_default())
            .collect();
        assert_eq!(cut, ["first", "SECOND EDITED", "third"]);
    }

    /// An edit to the LAST section of a recurring-anchor render — the one shape first-occurrence
    /// addressing happened to get right — must keep working.
    #[test]
    fn an_edit_to_the_last_section_still_addresses() {
        let baseline = render(vec![
            structure("\n   "),
            section("a", "first"),
            structure("\n   "),
            section("b", "second"),
            structure("\n"),
        ]);
        let edited = "\n   first\n   REWRITTEN\n";
        let ranges = address_sections(&baseline, edited).expect("addresses");
        assert_eq!(edited.get(ranges[1].clone()), Some("REWRITTEN"));
    }

    /// Two readings that are equally good are a genuine unknown, and the honest answer is a
    /// refusal rather than a guess. Here the edited text can be read as either section having
    /// become the other's content.
    #[test]
    fn an_edit_no_anchor_can_separate_refuses() {
        let baseline = render(vec![section("a", "x"), structure("|"), section("b", "x")]);
        assert_eq!(
            address_sections(&baseline, "x|y|x"),
            Err(SectionAddressRefusal::Ambiguous)
        );
    }

    /// Touching the immutable runs is not an edit to prose; nothing may attribute it.
    #[test]
    fn a_changed_immutable_run_is_unanchored() {
        let baseline = render(vec![
            structure("= help: "),
            section("a", "words"),
            structure("\n"),
        ]);
        assert_eq!(
            address_sections(&baseline, "= NOTE: words\n"),
            Err(SectionAddressRefusal::Unanchored)
        );
    }

    /// An unchanged render addresses to exactly its own sections.
    #[test]
    fn an_unchanged_render_addresses_to_itself() {
        let baseline = render(vec![
            structure("a"),
            section("s", "one"),
            structure("b"),
            section("t", "two"),
            structure("c"),
        ]);
        let ranges = address_sections(&baseline, "aonebtwoc").expect("addresses");
        assert_eq!(ranges, vec![1..4, 5..8]);
    }
}
