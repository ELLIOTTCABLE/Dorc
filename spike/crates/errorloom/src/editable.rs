//! Generic editable-section transport (`282:phase-generic-editable-sections`)

use std::fmt;

/// An outer render component
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RenderComponent<S, V> {
    /// Immutable layout
    Structure(String),
    /// Immutable rendered data
    FixedVariable {
        /// Renderer identity
        id: V,
        /// Rendered value
        rendered: String,
    },
    /// Editable prose
    EditableSection(EditableSection<S, V>),
}

/// A renderer-stamped editable prose section
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditableSection<S, V> {
    id: S,
    fragments: Vec<EditableFragment<V>>,
}

impl<S, V> EditableSection<S, V> {
    /// Constructs a section
    #[must_use]
    pub fn new(id: S, fragments: Vec<EditableFragment<V>>) -> Self {
        Self { id, fragments }
    }

    /// Returns the section identity
    #[must_use]
    pub fn id(&self) -> &S {
        &self.id
    }

    /// Returns the ordered fragments
    #[must_use]
    pub fn fragments(&self) -> &[EditableFragment<V>] {
        &self.fragments
    }
}

/// A section fragment
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EditableFragment<V> {
    /// Editable text
    Text(String),
    /// An opaque rendered variable
    Variable {
        /// Renderer identity
        id: V,
        /// Rendered value
        rendered: String,
    },
}

/// A renderer-stamped render
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditableRender<S, V> {
    components: Vec<RenderComponent<S, V>>,
}

impl<S, V> EditableRender<S, V> {
    /// Constructs a render
    #[must_use]
    pub fn new(components: Vec<RenderComponent<S, V>>) -> Self {
        Self { components }
    }

    /// Returns the ordered components
    #[must_use]
    pub fn components(&self) -> &[RenderComponent<S, V>] {
        &self.components
    }
}

impl<S, V> EditableRender<S, V> {
    /// Renders without tags
    #[must_use]
    pub fn text(&self) -> String {
        self.components.iter().map(component_text).collect()
    }
}

/// The section an adapter may compile
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SectionEdit<S, V> {
    section: S,
    fragments: Vec<EditableFragment<V>>,
}

impl<S, V> SectionEdit<S, V> {
    /// Returns the touched section
    #[must_use]
    pub fn section(&self) -> &S {
        &self.section
    }

    /// Returns the interpreted fragments
    #[must_use]
    pub fn fragments(&self) -> &[EditableFragment<V>] {
        &self.fragments
    }
}

/// A generic edit result
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EditTransport<S, V> {
    /// No bytes changed
    Unchanged,
    /// One section changed
    Edited(SectionEdit<S, V>),
}

/// Why attribution refused an edit
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum EditRefusalClass {
    /// Structure changed
    StructureTouched,
    /// A fixed variable changed
    FixedVariableTouched,
    /// A variable or its boundary changed
    EditableVariableTouched,
    /// More than one section changed
    CrossSection,
    /// No section can own the edit
    AmbiguousAttribution,
}

/// A generic transport refusal
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EditRefusal {
    class: EditRefusalClass,
}

impl EditRefusal {
    /// Returns the refusal category
    #[must_use]
    pub fn class(&self) -> EditRefusalClass {
        self.class
    }
}

impl fmt::Display for EditRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "errorloom: editable transport refused: {:?}", self.class)
    }
}

impl std::error::Error for EditRefusal {}

/// Attributes an edit to one editable text fragment.
///
/// Variable boundaries are checked before consumer tokenization.
///
/// # Errors
/// Returns [`EditRefusal`] for immutable, variable, or ambiguous edits.
pub fn transport_edit<S: Clone, V: Clone>(
    baseline: &EditableRender<S, V>,
    edited: &str,
) -> Result<EditTransport<S, V>, EditRefusal> {
    let original = baseline.text();
    if original == edited {
        return Ok(EditTransport::Unchanged);
    }

    let prefix = common_prefix_boundary(&original, edited);
    let suffix = common_suffix_boundary(&original[prefix..], &edited[prefix..]);
    let original_end = original.len().saturating_sub(suffix);
    let edited_end = edited.len().saturating_sub(suffix);
    let changed = prefix..original_end;

    let mut offset: usize = 0;
    let mut touched: Vec<(&RenderComponent<S, V>, usize)> = Vec::new();
    for component in &baseline.components {
        let text = component_text(component);
        let end = offset.saturating_add(text.len());
        if hunk_touches(&changed, offset, end) {
            touched.push((component, offset));
        }
        offset = end;
    }
    if touched.len() > 1 {
        return Err(refuse(EditRefusalClass::CrossSection));
    }
    let Some((component, offset)) = touched.pop() else {
        return Err(refuse(EditRefusalClass::AmbiguousAttribution));
    };
    transport_component(component, offset, &changed, &edited[prefix..edited_end])
}

fn transport_component<S: Clone, V: Clone>(
    component: &RenderComponent<S, V>,
    offset: usize,
    changed: &std::ops::Range<usize>,
    replacement: &str,
) -> Result<EditTransport<S, V>, EditRefusal> {
    match component {
        RenderComponent::Structure(_) => Err(refuse(EditRefusalClass::StructureTouched)),
        RenderComponent::FixedVariable { .. } => {
            Err(refuse(EditRefusalClass::FixedVariableTouched))
        }
        RenderComponent::EditableSection(section) => {
            let local_start = changed.start.saturating_sub(offset);
            let local_end = changed.end.saturating_sub(offset);
            let mut position: usize = 0;
            for fragment in &section.fragments {
                let end = position.saturating_add(fragment_text(fragment).len());
                if matches!(fragment, EditableFragment::Variable { .. })
                    && hunk_touches(&(local_start..local_end), position, end)
                {
                    return Err(refuse(EditRefusalClass::EditableVariableTouched));
                }
                position = end;
            }

            position = 0;
            let mut replacement_slot = None;
            for (index, fragment) in section.fragments.iter().enumerate() {
                let text = fragment_text(fragment);
                let end = position.saturating_add(text.len());
                match fragment {
                    EditableFragment::Text(_) if local_start >= position && local_end <= end => {
                        replacement_slot = Some((
                            index,
                            local_start.saturating_sub(position),
                            local_end.saturating_sub(position),
                        ));
                        break;
                    }
                    _ => {}
                }
                position = end;
            }
            let Some((index, start, end)) = replacement_slot else {
                return Err(refuse(EditRefusalClass::EditableVariableTouched));
            };
            let mut fragments = section.fragments.clone();
            if let Some(EditableFragment::Text(text)) = fragments.get_mut(index) {
                text.replace_range(start..end, replacement);
            }
            Ok(EditTransport::Edited(SectionEdit {
                section: section.id.clone(),
                fragments,
            }))
        }
    }
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

fn hunk_touches(hunk: &std::ops::Range<usize>, start: usize, end: usize) -> bool {
    if hunk.start == hunk.end {
        return hunk.start >= start && hunk.start <= end;
    }
    hunk.start < end && start < hunk.end
}

fn common_prefix_boundary(left: &str, right: &str) -> usize {
    let mut end = 0;
    for ((left_index, left_char), (right_index, right_char)) in
        left.char_indices().zip(right.char_indices())
    {
        if left_char != right_char {
            break;
        }
        end = left_index.saturating_add(left_char.len_utf8());
        if right_index.saturating_add(right_char.len_utf8()) != end {
            break;
        }
    }
    end
}

fn common_suffix_boundary(left: &str, right: &str) -> usize {
    let mut bytes: usize = 0;
    for (left_char, right_char) in left.chars().rev().zip(right.chars().rev()) {
        if left_char != right_char {
            break;
        }
        bytes = bytes.saturating_add(left_char.len_utf8());
    }
    bytes
}

fn refuse(class: EditRefusalClass) -> EditRefusal {
    EditRefusal { class }
}
