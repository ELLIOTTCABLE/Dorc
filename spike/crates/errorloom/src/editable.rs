//! Generic editable-section transport (`282:phase-generic-editable-sections`)

use std::fmt;

/// The default per-render scalar ceiling: exactly one maximal replay output
/// ([`MAX_REPLAY_OUTPUT_BYTES`](crate::MAX_REPLAY_OUTPUT_BYTES)), so the transport
/// never refuses a render the container itself would accept. The real cost bound is
/// [`TransportLimits::work_ceiling`]; this one only stops the alignment table's
/// dimensions from being computed over unbounded input.
pub const DEFAULT_RENDER_SCALAR_CEILING: usize = 64 * 1024;

/// The default alignment work ceiling — dynamic-programming states plus
/// occurrence checks, counted across one whole transport.
pub const DEFAULT_ALIGNMENT_WORK_CEILING: usize = 1_000_000;

/// The default per-section bound on variable occurrences considered for removal.
/// The removal search is exponential in this (`2^n` masks), so it stays small.
pub const DEFAULT_REMOVABLE_OCCURRENCES: usize = 12;

/// The transport's resource ceilings. [`Default`] is what [`transport_edit`] and
/// [`transport_edit_allow_removal`] use; a consumer whose renders outgrow them (or
/// whose host is smaller than this crate assumes) passes its own through
/// [`transport_edit_with_limits`] / [`transport_edit_allow_removal_with_limits`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TransportLimits {
    /// Maximum Unicode scalars in either the baseline render or the edited text.
    pub scalar_ceiling: usize,
    /// Maximum alignment work for one whole transport.
    pub work_ceiling: usize,
    /// Maximum removable variable occurrences considered per section.
    pub removable_occurrences: usize,
}

impl Default for TransportLimits {
    fn default() -> Self {
        Self {
            scalar_ceiling: DEFAULT_RENDER_SCALAR_CEILING,
            work_ceiling: DEFAULT_ALIGNMENT_WORK_CEILING,
            removable_occurrences: DEFAULT_REMOVABLE_OCCURRENCES,
        }
    }
}

/// An outer render component.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RenderComponent<S, V> {
    /// Immutable layout.
    Structure(String),
    /// Immutable rendered data.
    FixedVariable {
        /// Renderer identity.
        id: V,
        /// Rendered value.
        rendered: String,
    },
    /// Editable prose.
    EditableSection(EditableSection<S, V>),
}

/// A renderer-stamped editable prose section.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditableSection<S, V> {
    id: S,
    fragments: Vec<EditableFragment<V>>,
}

impl<S, V> EditableSection<S, V> {
    /// Constructs a section.
    #[must_use]
    pub fn new(id: S, fragments: Vec<EditableFragment<V>>) -> Self {
        let mut normalized = Vec::new();
        for fragment in fragments {
            match (&mut normalized.last_mut(), fragment) {
                (Some(EditableFragment::Text(previous)), EditableFragment::Text(next)) => {
                    previous.push_str(&next);
                }
                (_, fragment) => normalized.push(fragment),
            }
        }
        Self {
            id,
            fragments: normalized,
        }
    }

    /// Returns the section identity.
    #[must_use]
    pub fn id(&self) -> &S {
        &self.id
    }

    /// Returns the ordered fragments.
    #[must_use]
    pub fn fragments(&self) -> &[EditableFragment<V>] {
        &self.fragments
    }
}

/// A section fragment.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EditableFragment<V> {
    /// Editable text.
    Text(String),
    /// An opaque rendered variable.
    Variable {
        /// Renderer identity.
        id: V,
        /// Rendered value.
        rendered: String,
    },
}

/// A renderer-stamped render.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditableRender<S, V> {
    components: Vec<RenderComponent<S, V>>,
}

impl<S, V> EditableRender<S, V> {
    /// Constructs a render.
    #[must_use]
    pub fn new(components: Vec<RenderComponent<S, V>>) -> Self {
        Self { components }
    }

    /// Returns the ordered components.
    #[must_use]
    pub fn components(&self) -> &[RenderComponent<S, V>] {
        &self.components
    }

    /// Renders without tags.
    #[must_use]
    pub fn text(&self) -> String {
        self.components.iter().map(component_text).collect()
    }
}

/// The section an adapter may compile.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SectionEdit<S, V> {
    section: S,
    fragments: Vec<EditableFragment<V>>,
}

impl<S, V> SectionEdit<S, V> {
    /// Returns the touched section.
    #[must_use]
    pub fn section(&self) -> &S {
        &self.section
    }

    /// Returns the interpreted fragments.
    #[must_use]
    pub fn fragments(&self) -> &[EditableFragment<V>] {
        &self.fragments
    }
}

/// A generic edit result.
///
/// Deliberately NOT `#[non_exhaustive]`: a consumer that matched only [`Self::Edited`] must fail
/// to COMPILE when a lossy outcome becomes representable, and a wildcard arm — which
/// `non_exhaustive` would force — is exactly the silence [`Self::EditedWithDrops`] exists to
/// prevent (`271:rul-sin-ordering`: a mis-attributed loss outranks every other failure here).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EditTransport<S, V> {
    /// No bytes changed.
    Unchanged,
    /// One section changed, and every variable the render stamped into it survived.
    Edited(SectionEdit<S, V>),
    /// One section changed at the cost of variables the transport could not keep.
    ///
    /// Only [`transport_edit_allow_removal`] and its `_with_limits` twin can answer this; the
    /// required-retention [`transport_edit`] refuses instead of losing anything. The edit is
    /// sound to compile — it is what the author typed — but the compiled template interpolates
    /// FEWER values than the render did, so a consumer that publishes it without disclosing
    /// `drops` freezes today's world into tomorrow's message.
    EditedWithDrops {
        /// The interpreted section, exactly as [`Self::Edited`] would carry it.
        edit: SectionEdit<S, V>,
        /// Every variable occurrence the interpretation gave up, in render order.
        drops: Vec<VariableDrop<V>>,
    },
}

/// One variable occurrence an accepted edit could not keep.
///
/// Removal is the sanctioned mechanism (`282:rul-variable-edit-section-scope`), so a drop is
/// never itself an error — but the two ways to reach one want opposite repairs, and only the
/// facts here separate them. A variable whose value is GONE from the edited text was deleted on
/// purpose. A variable whose value is still sitting there as literal text was flattened: the
/// author edited the bytes anchoring it, the alignment had no reading that kept it, and the
/// register just froze the current world.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VariableDrop<V> {
    id: V,
    rendered: String,
    value_reappears: bool,
    value_shared: bool,
    retention_refusal: EditRefusalClass,
}

impl<V> VariableDrop<V> {
    /// The renderer's identity for the lost occurrence.
    #[must_use]
    pub fn id(&self) -> &V {
        &self.id
    }

    /// The bytes that occurrence rendered as.
    #[must_use]
    pub fn rendered(&self) -> &str {
        &self.rendered
    }

    /// Whether those bytes NEWLY appear in the edit's own literal text.
    ///
    /// An occurrence-count delta against the baseline section's literal text, never a bare
    /// `contains`: a value that was already spelled out beside its own variable — `` `nginx`: the
    /// {{service}} service `` — would make every genuine deletion of that variable read as a
    /// flattening, which is the one reading a warning must not invent.
    #[must_use]
    pub fn value_reappears_as_text(&self) -> bool {
        self.value_reappears
    }

    /// Whether another occurrence in the same section rendered these exact bytes.
    ///
    /// When it did, WHICH occurrence this drop names is a reading the alignment selected, not a
    /// measurement: the two are interchangeable in the rendered text, so only the prose around them
    /// separated them, and asymmetric prose can leave exactly one interpretation standing without
    /// that interpretation being the author's. Symmetric prose leaves two and refuses as
    /// [`EditRefusalClass::AmbiguousAttribution`], so reaching a drop at all means the selection
    /// already happened — which is why the fact is stated HERE rather than left to a consumer, who
    /// would have to model this search to know that a tie would have refused.
    ///
    /// Nothing is wrong today: the rendered bytes are the same whichever reading is taken. It bites
    /// when the two values later diverge, and the surviving hole turns out to be the other one.
    /// Orthogonal to [`Self::value_reappears_as_text`], which answers what happened to the dropped
    /// occurrence's BYTES rather than which occurrence was dropped.
    #[must_use]
    pub fn value_shared_with_another_occurrence(&self) -> bool {
        self.value_shared
    }

    /// Why the required-retention pass could not keep this section's variables.
    ///
    /// Recovered from the removal search's own first attempt, the one that demotes nothing —
    /// which the search previously threw away. It answers WHAT the edit contacted, and that is a
    /// different question from [`Self::value_reappears_as_text`]: deliberate deletion and
    /// flattening both refuse retention, so this alone does not separate them. It earns its keep
    /// where the contact was something other than the section's own prose, and it stops the
    /// consumer from having to re-run the transport to find out.
    #[must_use]
    pub fn retention_refusal(&self) -> EditRefusalClass {
        self.retention_refusal
    }
}

/// Why attribution refused an edit.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum EditRefusalClass {
    /// Structure changed.
    StructureTouched,
    /// A fixed variable changed.
    FixedVariableTouched,
    /// A variable or its boundary changed.
    EditableVariableTouched,
    /// More than one section changed.
    CrossSection,
    /// No section can own the edit.
    AmbiguousAttribution,
    /// The bounded scalar alignment would exceed its resource limit.
    AlignmentLimitExceeded,
}

/// Bounded, consumer-neutral evidence for a refused edit.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditRefusalEvidence {
    /// Render component contacted by the divergence, when known.
    pub component_ordinal: Option<usize>,
    /// Scalar hunk in the baseline render.
    pub baseline_scalars: std::ops::Range<usize>,
    /// Scalar hunk in the edited render.
    pub edited_scalars: std::ops::Range<usize>,
    /// Bounded baseline context around the hunk.
    pub baseline_context: String,
    /// Bounded edited context around the hunk.
    pub edited_context: String,
}

/// Which ceiling a limit refusal actually hit. Without it the metadata reads as a
/// size complaint even when the sizes were comfortably inside their ceiling and the
/// alignment simply cost too much.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum AlignmentLimit {
    /// The baseline render or the edited text exceeded the scalar ceiling.
    RenderScalars,
    /// A section carried more removable variable occurrences than the search bound.
    RemovableOccurrences,
    /// The alignment exhausted its work budget.
    AlignmentWork,
}

/// Resource metadata for a limit refusal.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AlignmentLimitMetadata {
    /// Which ceiling was hit.
    pub exceeded: AlignmentLimit,
    /// True baseline scalar count — never a `ceiling + 1` sentinel, because the one
    /// number a size refusal owes its reader is HOW FAR over the input was.
    pub baseline_scalars: usize,
    /// True edited scalar count.
    pub edited_scalars: usize,
    /// Per-render scalar ceiling.
    pub scalar_ceiling: usize,
    /// Total dynamic-programming and occurrence-check work ceiling.
    pub work_ceiling: usize,
}

/// A generic transport refusal.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditRefusal {
    class: EditRefusalClass,
    evidence: Vec<EditRefusalEvidence>,
    limit: Option<AlignmentLimitMetadata>,
}

impl EditRefusal {
    /// Returns the refusal category.
    #[must_use]
    pub fn class(&self) -> EditRefusalClass {
        self.class
    }

    /// Returns bounded hunk/contact evidence.
    #[must_use]
    pub fn evidence(&self) -> &[EditRefusalEvidence] {
        &self.evidence
    }

    /// Returns resource metadata for a limit refusal.
    #[must_use]
    pub fn limit(&self) -> Option<&AlignmentLimitMetadata> {
        self.limit.as_ref()
    }
}

impl fmt::Display for EditRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "errorloom: editable transport refused: {:?}", self.class)?;
        if let Some(limit) = &self.limit {
            write!(
                f,
                " ({:?}): baseline {} scalars, edited {} scalars, scalar ceiling {}, work ceiling {}",
                limit.exceeded,
                limit.baseline_scalars,
                limit.edited_scalars,
                limit.scalar_ceiling,
                limit.work_ceiling
            )?;
        }
        for evidence in &self.evidence {
            write!(
                f,
                " at component {:?}, {:?}/{:?} near {:?}/{:?}",
                evidence.component_ordinal,
                evidence.baseline_scalars,
                evidence.edited_scalars,
                evidence.baseline_context,
                evidence.edited_context
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for EditRefusal {}

/// Attributes ordinary-text edits to exactly one renderer-stamped section.
///
/// The alignment is over Unicode scalars. Variables and immediately adjacent text
/// scalars are anchors, so the consumer receives cloned variable identities and
/// rendered bytes before it performs any tokenization.
///
/// # Errors
/// Returns [`EditRefusal`] for immutable, variable, ambiguous, or over-limit edits.
pub fn transport_edit<S: Clone, V: Clone>(
    baseline: &EditableRender<S, V>,
    edited: &str,
) -> Result<EditTransport<S, V>, EditRefusal> {
    transport_edit_with_limits(baseline, edited, TransportLimits::default())
}

/// [`transport_edit`] under caller-chosen ceilings.
///
/// # Errors
/// Returns [`EditRefusal`] for immutable, variable, ambiguous, or over-limit edits.
pub fn transport_edit_with_limits<S: Clone, V: Clone>(
    baseline: &EditableRender<S, V>,
    edited: &str,
    limits: TransportLimits,
) -> Result<EditTransport<S, V>, EditRefusal> {
    Ok(
        match transport_edit_with_budget(baseline, edited, None, &mut WorkBudget::new(limits))? {
            Some(edit) => EditTransport::Edited(edit),
            None => EditTransport::Unchanged,
        },
    )
}

/// Attributes an edit while allowing a uniquely inferred omission of variables.
///
/// A selected occurrence is omitted before the required-anchor alignment runs.
/// Adjacent text coalesces through [`EditableSection::new`]. The search considers
/// at most [`TransportLimits::removable_occurrences`] occurrences per section.
///
/// # Errors
/// Returns [`EditRefusal`] when no unique minimum-removal interpretation exists.
pub fn transport_edit_allow_removal<S: Clone, V: Clone>(
    baseline: &EditableRender<S, V>,
    edited: &str,
) -> Result<EditTransport<S, V>, EditRefusal> {
    transport_edit_allow_removal_with_limits(baseline, edited, TransportLimits::default())
}

/// [`transport_edit_allow_removal`] under caller-chosen ceilings.
///
/// # Errors
/// Returns [`EditRefusal`] when no unique minimum-removal interpretation exists.
pub fn transport_edit_allow_removal_with_limits<S: Clone, V: Clone>(
    baseline: &EditableRender<S, V>,
    edited: &str,
    limits: TransportLimits,
) -> Result<EditTransport<S, V>, EditRefusal> {
    if let Some(refusal) = scalar_ceiling_refusal(baseline, edited, limits) {
        return Err(refusal);
    }
    if baseline.text() == edited {
        return Ok(EditTransport::Unchanged);
    }

    let candidates = removal_candidates(baseline, edited, limits)?;
    let mut work = WorkBudget::new(limits);
    let mut retention_refusals: Vec<(usize, EditRefusalClass)> = Vec::new();
    for removals in 0..=limits.removable_occurrences {
        let mut successes = Vec::new();
        for &(component_index, occurrences) in &candidates {
            if removals > occurrences {
                continue;
            }
            let candidate_count = 1usize << occurrences;
            for mask in 0..candidate_count {
                if mask.count_ones() as usize != removals {
                    continue;
                }
                let transformed = demote_occurrences(baseline, component_index, mask);
                match transport_edit_with_budget(
                    &transformed,
                    edited,
                    Some(component_index),
                    &mut work,
                ) {
                    Ok(Some(edit)) => successes.push((component_index, mask, edit)),
                    Ok(None) => {
                        if let Some(RenderComponent::EditableSection(transformed_section)) =
                            transformed.components.get(component_index)
                        {
                            successes.push((
                                component_index,
                                mask,
                                SectionEdit {
                                    section: transformed_section.id.clone(),
                                    fragments: transformed_section.fragments.clone(),
                                },
                            ));
                        }
                    }
                    Err(refusal) if refusal.class() == EditRefusalClass::AlignmentLimitExceeded => {
                        return Err(refusal);
                    }
                    Err(refusal) => {
                        if removals == 0 {
                            retention_refusals.push((component_index, refusal.class()));
                        }
                    }
                }
            }
        }
        match successes.len() {
            0 => {}
            1 => {
                let (component_index, mask, edit) = successes.remove(0);
                let drops = dropped_occurrences(
                    baseline,
                    component_index,
                    mask,
                    &edit,
                    retention_class(&retention_refusals, component_index),
                );
                return Ok(if drops.is_empty() {
                    EditTransport::Edited(edit)
                } else {
                    EditTransport::EditedWithDrops { edit, drops }
                });
            }
            _ => {
                return Err(refuse(
                    EditRefusalClass::AmbiguousAttribution,
                    &baseline.text(),
                    edited,
                ));
            }
        }
    }
    Err(classify_refusal(baseline, edited))
}

/// Every section the edited bytes could belong to, with its removable-occurrence count.
///
/// A section past the bound refuses as a LIMIT rather than being quietly excluded: the search is
/// `2^n` masks wide, so the machine's answer is "too expensive to ask", which is a different
/// thing from "the edit is ambiguous" and must read differently to whoever hits it.
///
/// The bound is the caller's OR the mask width, whichever is smaller. Selection rides a `usize`
/// bitmask, so a section wider than that has no representable mask at all, and a caller who
/// raised the ceiling past it would otherwise reach a shift overflow rather than a refusal
/// (`inv-no-throw`). Nothing real approaches either number — the whole search is unaffordable
/// long before it.
fn removal_candidates<S, V>(
    baseline: &EditableRender<S, V>,
    edited: &str,
    limits: TransportLimits,
) -> Result<Vec<(usize, usize)>, EditRefusal> {
    let representable = usize::BITS.saturating_sub(1) as usize;
    let affordable = limits.removable_occurrences.min(representable);
    let mut candidates = Vec::new();
    for (component_index, component) in baseline.components.iter().enumerate() {
        let RenderComponent::EditableSection(section) = component else {
            continue;
        };
        if section_interior(baseline, component_index, edited).is_none() {
            continue;
        }
        let occurrences = section
            .fragments
            .iter()
            .filter(|fragment| matches!(fragment, EditableFragment::Variable { .. }))
            .count();
        if occurrences > affordable {
            return Err(limit_refusal(
                AlignmentLimit::RemovableOccurrences,
                baseline,
                edited,
                limits,
            ));
        }
        candidates.push((component_index, occurrences));
    }
    Ok(candidates)
}

/// The class the required-retention pass produced for one section.
///
/// Always recorded by the time a drop exists: reaching a removal count above zero means every
/// candidate's demote-nothing attempt refused. The fallback is the shape of that refusal anyway,
/// so an unrecorded section reports the same thing rather than an absence a consumer must model.
fn retention_class(recorded: &[(usize, EditRefusalClass)], component: usize) -> EditRefusalClass {
    recorded
        .iter()
        .find(|(index, _)| *index == component)
        .map_or(EditRefusalClass::EditableVariableTouched, |(_, class)| {
            *class
        })
}

/// The variable occurrences `mask` gave up, paired with the evidence separating a deliberate
/// deletion from a flattening (see [`VariableDrop`]).
fn dropped_occurrences<S, V: Clone>(
    baseline: &EditableRender<S, V>,
    component_index: usize,
    mask: usize,
    edit: &SectionEdit<S, V>,
    retention_refusal: EditRefusalClass,
) -> Vec<VariableDrop<V>> {
    let Some(RenderComponent::EditableSection(section)) = baseline.components.get(component_index)
    else {
        return Vec::new();
    };
    let before = literal_text(&section.fragments);
    let after = literal_text(&edit.fragments);
    let rendered_values: Vec<&str> = section
        .fragments
        .iter()
        .filter_map(|fragment| match fragment {
            EditableFragment::Variable { rendered, .. } => Some(rendered.as_str()),
            EditableFragment::Text(_) => None,
        })
        .collect();
    let mut occurrence = 0usize;
    let mut drops = Vec::new();
    for fragment in &section.fragments {
        let EditableFragment::Variable { id, rendered } = fragment else {
            continue;
        };
        let this = occurrence;
        let selected = mask & (1usize << occurrence) != 0;
        occurrence = occurrence.saturating_add(1);
        if !selected {
            continue;
        }
        drops.push(VariableDrop {
            id: id.clone(),
            value_reappears: occurrences_of(&after, rendered) > occurrences_of(&before, rendered),
            value_shared: rendered_values
                .iter()
                .enumerate()
                .any(|(other, value)| other != this && *value == rendered.as_str()),
            rendered: rendered.clone(),
            retention_refusal,
        });
    }
    drops
}

fn literal_text<V>(fragments: &[EditableFragment<V>]) -> String {
    fragments
        .iter()
        .filter_map(|fragment| match fragment {
            EditableFragment::Text(text) => Some(text.as_str()),
            EditableFragment::Variable { .. } => None,
        })
        .collect()
}

/// Non-overlapping occurrences of a NON-EMPTY needle; an empty rendered value can never
/// reappear, and asking `str::matches` about one answers with the haystack's length.
fn occurrences_of(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    haystack.matches(needle).count()
}

/// The required-retention transport core, parameterized so bounded callers can
/// share one alignment budget across multiple candidate renders. `Ok(None)` means the edited
/// bytes already equal the render's; it never loses a variable, so it has no drops to report.
#[expect(
    clippy::indexing_slicing,
    reason = "enumerate-derived component bounds"
)]
fn transport_edit_with_budget<S: Clone, V: Clone>(
    baseline: &EditableRender<S, V>,
    edited: &str,
    only_component: Option<usize>,
    work: &mut WorkBudget,
) -> Result<Option<SectionEdit<S, V>>, EditRefusal> {
    if let Some(refusal) = scalar_ceiling_refusal(baseline, edited, work.limits) {
        return Err(refusal);
    }
    let original = baseline.text();
    if original == edited {
        return Ok(None);
    }

    let mut successful = Vec::new();
    let mut saw_limit = false;
    for (index, component) in baseline.components.iter().enumerate() {
        if only_component.is_some_and(|only| only != index) {
            continue;
        }
        let RenderComponent::EditableSection(section) = component else {
            continue;
        };
        let prefix: String = baseline.components[..index]
            .iter()
            .map(component_text)
            .collect();
        let suffix: String = baseline.components[index.saturating_add(1)..]
            .iter()
            .map(component_text)
            .collect();
        let Some(interior) = edited
            .strip_prefix(&prefix)
            .and_then(|rest| rest.strip_suffix(&suffix))
        else {
            continue;
        };
        match align_section(section, interior, work) {
            Ok(fragments) => successful.push(SectionEdit {
                section: section.id.clone(),
                fragments,
            }),
            Err(EditRefusalClass::AlignmentLimitExceeded) => saw_limit = true,
            Err(_) => {}
        }
    }
    match successful.len() {
        _ if saw_limit => Err(limit_refusal(
            AlignmentLimit::AlignmentWork,
            baseline,
            edited,
            work.limits,
        )),
        1 => Ok(Some(successful.remove(0))),
        0 => Err(classify_refusal(baseline, edited)),
        _ => Err(refuse(
            EditRefusalClass::AmbiguousAttribution,
            &original,
            edited,
        )),
    }
}

fn section_interior<'a, S, V>(
    baseline: &'a EditableRender<S, V>,
    component_index: usize,
    edited: &'a str,
) -> Option<&'a str> {
    let prefix: String = baseline
        .components
        .get(..component_index)?
        .iter()
        .map(component_text)
        .collect();
    let suffix: String = baseline
        .components
        .get(component_index.saturating_add(1)..)?
        .iter()
        .map(component_text)
        .collect();
    edited
        .strip_prefix(&prefix)
        .and_then(|rest| rest.strip_suffix(&suffix))
}

fn demote_occurrences<S: Clone, V: Clone>(
    baseline: &EditableRender<S, V>,
    component_index: usize,
    mask: usize,
) -> EditableRender<S, V> {
    let mut components = baseline.components.clone();
    let Some(RenderComponent::EditableSection(section)) = components.get(component_index) else {
        return EditableRender::new(components);
    };
    let mut occurrence = 0usize;
    let fragments = section
        .fragments
        .iter()
        .cloned()
        .filter_map(|fragment| match fragment {
            EditableFragment::Variable { .. } if mask & (1usize << occurrence) != 0 => {
                occurrence = occurrence.saturating_add(1);
                None
            }
            EditableFragment::Variable { id, rendered } => {
                occurrence = occurrence.saturating_add(1);
                Some(EditableFragment::Variable { id, rendered })
            }
            EditableFragment::Text(text) => Some(EditableFragment::Text(text)),
        })
        .collect();
    let section_id = section.id.clone();
    if let Some(component) = components.get_mut(component_index) {
        *component = RenderComponent::EditableSection(EditableSection::new(section_id, fragments));
    }
    EditableRender::new(components)
}

#[expect(
    clippy::indexing_slicing,
    reason = "table dimensions are checked before allocation"
)]
fn align_section<V: Clone>(
    section: &EditableSection<impl Sized, V>,
    edited: &str,
    work: &mut WorkBudget,
) -> Result<Vec<EditableFragment<V>>, EditRefusalClass> {
    let (slots, anchors) = section_parts(&section.fragments);
    let edit: Vec<char> = edited.chars().collect();
    let Some(states) = anchors.len().checked_add(1).and_then(|rows| {
        edit.len()
            .checked_add(1)
            .and_then(|columns| rows.checked_mul(columns))
    }) else {
        return Err(EditRefusalClass::AlignmentLimitExceeded);
    };
    work.reserve(states)?;

    let width = edit.len().saturating_add(1);
    let mut table: Vec<Option<Path>> = vec![None; states];
    table[0] = Some(Path {
        cost: 0,
        previous: None,
    });
    for anchor_index in 0..anchors.len() {
        let anchor = &anchors[anchor_index];
        for start in 0..=edit.len() {
            let state = anchor_index.saturating_mul(width).saturating_add(start);
            let Some(path) = table[state].clone() else {
                continue;
            };
            let Some(last) = edit.len().checked_sub(anchor.text.len()) else {
                continue;
            };
            let positions = if anchor.text.is_empty() {
                start..=start
            } else {
                start..=last
            };
            for found in positions {
                work.reserve(1)?;
                if edit.get(found..found.saturating_add(anchor.text.len())) != Some(&anchor.text) {
                    continue;
                }
                let Some(slot_cost) = slot_cost(&slots[anchor_index], &edit[start..found]) else {
                    continue;
                };
                let cost = path.cost.saturating_add(slot_cost);
                let end = found.saturating_add(anchor.text.len());
                let next = anchor_index
                    .saturating_add(1)
                    .saturating_mul(width)
                    .saturating_add(end);
                let replace = table[next].as_ref().is_none_or(|old| cost < old.cost);
                if replace {
                    table[next] = Some(Path {
                        cost,
                        previous: Some((start, found)),
                    });
                }
            }
        }
    }

    let mut best: Option<(usize, usize)> = None;
    for start in 0..=edit.len() {
        let state = anchors.len().saturating_mul(width).saturating_add(start);
        let Some(path) = table[state].as_ref() else {
            continue;
        };
        let Some(slot_cost) = slot_cost(&slots[anchors.len()], &edit[start..]) else {
            continue;
        };
        let cost = path.cost.saturating_add(slot_cost);
        if best.is_none_or(|(_, old)| cost < old) {
            best = Some((start, cost));
        }
    }
    let Some((end, _)) = best else {
        return Err(EditRefusalClass::EditableVariableTouched);
    };
    reconstruct_section(section, &slots, &anchors, &edit, &table, width, end)
}

#[expect(
    clippy::indexing_slicing,
    reason = "alignment paths and scalar bounds are checked by construction"
)]
fn reconstruct_section<V: Clone>(
    section: &EditableSection<impl Sized, V>,
    slots: &[Slot],
    anchors: &[Anchor],
    edit: &[char],
    table: &[Option<Path>],
    width: usize,
    mut end: usize,
) -> Result<Vec<EditableFragment<V>>, EditRefusalClass> {
    let mut pieces = vec![Vec::new(); anchors.len().saturating_add(1)];
    pieces[anchors.len()] = edit[end..].to_vec();
    for anchor_index in (0..anchors.len()).rev() {
        let state = anchor_index
            .saturating_add(1)
            .saturating_mul(width)
            .saturating_add(end);
        let Some((start, found)) = table[state].as_ref().and_then(|path| path.previous) else {
            return Err(EditRefusalClass::EditableVariableTouched);
        };
        pieces[anchor_index] = edit[start..found].to_vec();
        end = start;
    }
    if end != 0 {
        return Err(EditRefusalClass::EditableVariableTouched);
    }
    let mut fragments = section.fragments.clone();
    let last_slot = slots.len().saturating_sub(1);
    let mut leading = String::new();
    let mut trailing = String::new();
    for (index, (slot, replacement)) in slots.iter().zip(pieces).enumerate() {
        let Some(fragment) = slot.fragment else {
            if slot.edge && !replacement.is_empty() {
                let inserted: String = replacement.into_iter().collect();
                if index == 0 {
                    leading = inserted;
                } else if index == last_slot {
                    trailing = inserted;
                }
            }
            continue;
        };
        if let Some(EditableFragment::Text(text)) = fragments.get_mut(fragment) {
            let mut rebuilt = slot.prefix.clone();
            rebuilt.extend(replacement);
            rebuilt.extend(&slot.suffix);
            *text = rebuilt.into_iter().collect();
        }
    }
    if !leading.is_empty() {
        fragments.insert(0, EditableFragment::Text(leading));
    }
    if !trailing.is_empty() {
        fragments.push(EditableFragment::Text(trailing));
    }
    Ok(fragments)
}

#[derive(Clone)]
struct Path {
    cost: usize,
    previous: Option<(usize, usize)>,
}

struct Slot {
    fragment: Option<usize>,
    /// A slot standing at a section EDGE rather than between a variable and its neighbouring
    /// text. It owns no fragment yet is still writable: text typed there is ordinary interior
    /// prose, and [`reconstruct_section`] mints the fragment to hold it
    /// (`282:rul-untouched-variable-preservation` — a section edge is not an anchor, so the
    /// commonest prose edits there are, appending a clause or prefixing a word, must not
    /// destroy a variable that happens to sit first or last).
    edge: bool,
    text: Vec<char>,
    prefix: Vec<char>,
    suffix: Vec<char>,
}
struct Anchor {
    text: Vec<char>,
}

impl Slot {
    fn unowned() -> Self {
        Self {
            fragment: None,
            edge: false,
            text: Vec::new(),
            prefix: Vec::new(),
            suffix: Vec::new(),
        }
    }
}

#[expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "enumerated fragment and scalar bounds are checked locally"
)]
fn section_parts<V>(fragments: &[EditableFragment<V>]) -> (Vec<Slot>, Vec<Anchor>) {
    let mut slots = Vec::new();
    let mut anchors = Vec::new();
    slots.push(Slot::unowned());
    for (index, fragment) in fragments.iter().enumerate() {
        match fragment {
            EditableFragment::Variable { rendered, .. } => {
                anchors.push(Anchor {
                    text: rendered.chars().collect(),
                });
                slots.push(Slot::unowned());
            }
            EditableFragment::Text(text) => {
                let chars: Vec<char> = text.chars().collect();
                let left_variable =
                    index > 0 && matches!(fragments[index - 1], EditableFragment::Variable { .. });
                let right_variable = index.saturating_add(1) < fragments.len()
                    && matches!(fragments[index + 1], EditableFragment::Variable { .. });
                let start = usize::from(left_variable && !chars.is_empty());
                let end = chars
                    .len()
                    .saturating_sub(usize::from(right_variable && chars.len() > start));
                if left_variable && !chars.is_empty() {
                    anchors.push(Anchor {
                        text: vec![chars[0]],
                    });
                    slots.push(Slot::unowned());
                }
                let owns_slot = if chars.is_empty() {
                    !left_variable && !right_variable
                } else if left_variable && right_variable {
                    start < end
                } else {
                    true
                };
                if owns_slot && let Some(slot) = slots.last_mut() {
                    slot.fragment = Some(index);
                    slot.text = chars[start..end].to_vec();
                    slot.prefix = chars[..start].to_vec();
                    slot.suffix = chars[end..].to_vec();
                }
                if right_variable && chars.len() > start {
                    anchors.push(Anchor {
                        text: vec![chars[end]],
                    });
                    slots.push(Slot::unowned());
                }
            }
        }
    }
    open_the_section_edges(&mut slots, fragments);
    (slots, anchors)
}

/// Open the slot outside a leading or trailing variable, which the walk above leaves unowned
/// because there is no neighbouring text fragment to own it.
///
/// A variable's anchors are the bytes immediately beside it; an unowned slot demands emptiness and
/// so makes the section's own EDGE behave like an anchor. It is not one: nothing of the render
/// lies out there, so text typed past the first or last fragment is ordinary interior insertion,
/// and treating it as abutment silently destroyed the variable
/// (`282:rul-rendered-variable-offsets-may-move`).
fn open_the_section_edges<V>(slots: &mut [Slot], fragments: &[EditableFragment<V>]) {
    let leads = matches!(fragments.first(), Some(EditableFragment::Variable { .. }));
    let trails = matches!(fragments.last(), Some(EditableFragment::Variable { .. }));
    if let Some(slot) = slots.first_mut() {
        slot.edge = leads;
    }
    if let Some(slot) = slots.last_mut() {
        slot.edge = slot.edge || trails;
    }
}

struct WorkBudget {
    spent: usize,
    limits: TransportLimits,
}

impl WorkBudget {
    fn new(limits: TransportLimits) -> Self {
        Self { spent: 0, limits }
    }

    fn reserve(&mut self, amount: usize) -> Result<(), EditRefusalClass> {
        let Some(next) = self.spent.checked_add(amount) else {
            return Err(EditRefusalClass::AlignmentLimitExceeded);
        };
        if next > self.limits.work_ceiling {
            return Err(EditRefusalClass::AlignmentLimitExceeded);
        }
        self.spent = next;
        Ok(())
    }
}

fn text_cost(original: &[char], replacement: &[char]) -> usize {
    if original == replacement {
        0
    } else {
        original.len().saturating_add(replacement.len())
    }
}

fn slot_cost(slot: &Slot, replacement: &[char]) -> Option<usize> {
    if slot.fragment.is_none() && !slot.edge && !replacement.is_empty() {
        None
    } else {
        Some(text_cost(&slot.text, replacement))
    }
}

fn classify_refusal<S, V>(baseline: &EditableRender<S, V>, edited: &str) -> EditRefusal {
    let original = baseline.text();
    let prefix = common_prefix_scalars(&original, edited);
    let suffix = common_suffix_scalars(&original, edited, prefix);
    let changed = prefix..scalar_len(&original).saturating_sub(suffix);
    let mut offset: usize = 0;
    let mut touched = Vec::new();
    for (ordinal, component) in baseline.components.iter().enumerate() {
        let end = offset.saturating_add(scalar_len(&component_text(component)));
        if scalar_hunk_touches(&changed, offset, end) {
            touched.push((ordinal, component));
        }
        offset = end;
    }
    let class = if touched.is_empty() {
        EditRefusalClass::AmbiguousAttribution
    } else if touched.len() > 1 {
        EditRefusalClass::CrossSection
    } else {
        touched.first().map_or(
            EditRefusalClass::EditableVariableTouched,
            |(_, component)| match component {
                RenderComponent::Structure(_) => EditRefusalClass::StructureTouched,
                RenderComponent::FixedVariable { .. } => EditRefusalClass::FixedVariableTouched,
                RenderComponent::EditableSection(_) => EditRefusalClass::EditableVariableTouched,
            },
        )
    };
    refuse_with_component(
        class,
        &original,
        edited,
        touched.first().map(|(ordinal, _)| *ordinal),
    )
}

fn component_text<S, V>(component: &RenderComponent<S, V>) -> String {
    match component {
        RenderComponent::Structure(text) => text.clone(),
        RenderComponent::FixedVariable { rendered, .. } => rendered.clone(),
        RenderComponent::EditableSection(section) => {
            section.fragments.iter().map(fragment_text).collect()
        }
    }
}

fn fragment_text<V>(fragment: &EditableFragment<V>) -> String {
    match fragment {
        EditableFragment::Text(text) | EditableFragment::Variable { rendered: text, .. } => {
            text.clone()
        }
    }
}

fn scalar_len(text: &str) -> usize {
    text.chars().count()
}
fn render_scalar_len<S, V>(render: &EditableRender<S, V>) -> usize {
    let mut count: usize = 0;
    for component in &render.components {
        match component {
            RenderComponent::Structure(text)
            | RenderComponent::FixedVariable { rendered: text, .. } => {
                count = count.saturating_add(scalar_len(text));
            }
            RenderComponent::EditableSection(section) => {
                for fragment in &section.fragments {
                    let text = match fragment {
                        EditableFragment::Text(text)
                        | EditableFragment::Variable { rendered: text, .. } => text,
                    };
                    count = count.saturating_add(scalar_len(text));
                }
            }
        }
    }
    count
}

/// The scalar-ceiling gate. Counting is exact rather than capped-with-a-sentinel:
/// both operands are already in memory, so the walk costs what the ceiling check
/// would have cost anyway, and the refusal can then say how far over the input was.
fn scalar_ceiling_refusal<S, V>(
    baseline: &EditableRender<S, V>,
    edited: &str,
    limits: TransportLimits,
) -> Option<EditRefusal> {
    let baseline_scalars = render_scalar_len(baseline);
    let edited_scalars = scalar_len(edited);
    (baseline_scalars > limits.scalar_ceiling || edited_scalars > limits.scalar_ceiling)
        .then(|| limit_refusal(AlignmentLimit::RenderScalars, baseline, edited, limits))
}
fn common_prefix_scalars(left: &str, right: &str) -> usize {
    left.chars()
        .zip(right.chars())
        .take_while(|(a, b)| a == b)
        .count()
}
#[expect(
    clippy::indexing_slicing,
    reason = "prefix comes from both scalar streams"
)]
fn common_suffix_scalars(left: &str, right: &str, prefix: usize) -> usize {
    let left: Vec<char> = left.chars().collect();
    let right: Vec<char> = right.chars().collect();
    left[prefix..]
        .iter()
        .rev()
        .zip(right[prefix..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count()
}

fn scalar_hunk_touches(hunk: &std::ops::Range<usize>, start: usize, end: usize) -> bool {
    if hunk.start == hunk.end {
        hunk.start >= start && hunk.start <= end
    } else {
        hunk.start < end && start < hunk.end
    }
}

fn refuse(class: EditRefusalClass, baseline: &str, edited: &str) -> EditRefusal {
    refuse_with_component(class, baseline, edited, None)
}

fn refuse_with_component(
    class: EditRefusalClass,
    baseline: &str,
    edited: &str,
    component_ordinal: Option<usize>,
) -> EditRefusal {
    let (baseline_scalars, edited_scalars) = changed_hunks(baseline, edited);
    EditRefusal {
        class,
        limit: None,
        evidence: vec![EditRefusalEvidence {
            component_ordinal,
            baseline_context: scalar_excerpt(baseline, &baseline_scalars),
            edited_context: scalar_excerpt(edited, &edited_scalars),
            baseline_scalars,
            edited_scalars,
        }],
    }
}

fn limit_refusal<S, V>(
    exceeded: AlignmentLimit,
    baseline: &EditableRender<S, V>,
    edited: &str,
    limits: TransportLimits,
) -> EditRefusal {
    EditRefusal {
        class: EditRefusalClass::AlignmentLimitExceeded,
        evidence: Vec::new(),
        limit: Some(AlignmentLimitMetadata {
            exceeded,
            baseline_scalars: render_scalar_len(baseline),
            edited_scalars: scalar_len(edited),
            scalar_ceiling: limits.scalar_ceiling,
            work_ceiling: limits.work_ceiling,
        }),
    }
}

fn changed_hunks(baseline: &str, edited: &str) -> (std::ops::Range<usize>, std::ops::Range<usize>) {
    let prefix = common_prefix_scalars(baseline, edited);
    let suffix = common_suffix_scalars(baseline, edited, prefix);
    (
        prefix..scalar_len(baseline).saturating_sub(suffix),
        prefix..scalar_len(edited).saturating_sub(suffix),
    )
}

fn scalar_excerpt(text: &str, hunk: &std::ops::Range<usize>) -> String {
    let chars: Vec<char> = text.chars().collect();
    let start = hunk.start.saturating_sub(12);
    let end = hunk.end.saturating_add(12).min(chars.len());
    chars.get(start..end).unwrap_or_default().iter().collect()
}
