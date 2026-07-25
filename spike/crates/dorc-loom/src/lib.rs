//! dorc-loom — the Dorc editable-render adapter (`282` §4 · §13).

use std::collections::BTreeMap;

use dorc_aid::tagged::{self, RenderPart, RenderParts};
use errorloom::{EditableFragment, EditableRender, EditableSection, RenderComponent};

mod consumer;
pub use consumer::{
    DorcApplyRefusal, DorcConsumer, DorcEditableBaseline, DorcReplayDriver, SectionVariables,
    replay_case, replay_case_with_inputs,
};
mod compile;
pub use compile::{CompileRefusal, CompiledFragment, CompiledSection, compile_fragments};
mod generate;
pub use generate::{
    Publication, build_publication, generate_arrangement_lock, generate_catalog_lock,
    load_arrangement_corpus, load_corpus_by_slug,
};
mod edit;
pub use edit::{
    DorcSectionEdit, DorcSectionEditRefusal, compile_section_edit, compile_section_edits,
};
mod inspect;
pub use inspect::render_compile_preview;
mod preview;
pub use preview::{CompilePreview, SectionPreview, compile_preview};
mod receipt;
pub use receipt::{
    InspectedCompilation, InspectedReplay, MAX_RECEIPT_BYTES, ReceiptError, ValidatedCompilation,
    encode as encode_receipt, validate_current as validate_receipt,
};
mod receipt_store;
pub use receipt_store::{FsReceiptStore, ReceiptStore, ReceiptWriteOutcome};
mod repository;
pub use repository::{GitRepository, ProseClassification, Repository, classify_prose_changes};
mod workflow;
pub use workflow::{compile as compile_receipt, promote as promote_receipt};

/// A semantic template variable name.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TemplateVariableName(pub String);

/// A variable occurrence within one editable section.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SectionVariableId {
    /// The semantic catalog parameter name.
    pub name: TemplateVariableName,
    /// The zero-based occurrence of `name` in this section.
    pub occurrence: usize,
}

/// The identity of one contiguous editable prose segment, in either register.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SectionKey {
    /// Who owns the prose: a diagnostic code slug for the catalog registers, an arrangement
    /// slug for the arrangement registry.
    pub owner: String,
    /// Which register of that owner: `message`/`help` (catalog) or [`ARRANGEMENT_FIELD`].
    pub field: &'static str,
    /// The core-emitted field occurrence.
    pub instance: usize,
    /// The deterministic split ordinal within this render.
    pub segment: usize,
}

/// The [`SectionKey::field`] value naming the arrangement registry, beside the catalog's
/// `message`/`help` (`289:rul-arrangement-home-is-registry-plus-transcripts`). An arrangement
/// section's [`SectionKey::instance`] is its occurrence — the registry resolves an unclaimed
/// occurrence to the whole-slug entry, which is what lets repeated chrome share one entry.
pub const ARRANGEMENT_FIELD: &str = "arrangement";

/// Map a core render-part stream to generic editable sections.
#[must_use]
pub fn to_editable_render(parts: &RenderParts) -> EditableRender<SectionKey, SectionVariableId> {
    let mut components = Vec::new();
    let mut section = None;
    let mut next_segment = 0usize;
    let mut arrangement_occurrences: BTreeMap<&'static str, usize> = BTreeMap::new();
    for part in parts.parts() {
        match part {
            RenderPart::TemplateLiteral {
                text,
                code,
                field,
                instance,
                ..
            } => {
                let current = open_section(
                    &mut components,
                    &mut section,
                    &mut next_segment,
                    (*code, *field, *instance),
                );
                current.fragments.push(EditableFragment::Text(text.clone()));
            }
            RenderPart::ParamValue {
                text,
                code,
                field,
                param,
                instance,
            } => {
                let current = open_section(
                    &mut components,
                    &mut section,
                    &mut next_segment,
                    (*code, *field, *instance),
                );
                let occurrence = current.occurrences.entry(*param).or_default();
                current.fragments.push(EditableFragment::Variable {
                    id: SectionVariableId {
                        name: TemplateVariableName(String::from(*param)),
                        occurrence: *occurrence,
                    },
                    rendered: text.clone(),
                });
                *occurrence = occurrence.saturating_add(1);
            }
            RenderPart::ForeignText { text, param } => {
                flush_section(&mut components, &mut section);
                components.push(RenderComponent::FixedVariable {
                    id: SectionVariableId {
                        name: TemplateVariableName(String::from(*param)),
                        occurrence: 0,
                    },
                    rendered: text.clone(),
                });
            }
            RenderPart::Arrangement { text, .. } => {
                flush_section(&mut components, &mut section);
                components.push(RenderComponent::Structure(text.clone()));
            }
            RenderPart::ArrangementWords {
                text,
                slug,
                occurrence,
            } => {
                flush_section(&mut components, &mut section);
                let position = arrangement_occurrences.entry(slug).or_default();
                components.push(RenderComponent::EditableSection(EditableSection::new(
                    SectionKey {
                        owner: String::from(*slug),
                        field: ARRANGEMENT_FIELD,
                        instance: occurrence.unwrap_or(*position),
                        segment: next_segment,
                    },
                    vec![EditableFragment::Text(text.clone())],
                )));
                *position = position.saturating_add(1);
                next_segment = next_segment.saturating_add(1);
            }
        }
    }
    flush_section(&mut components, &mut section);
    EditableRender::new(components)
}

struct ActiveSection {
    key: (&'static str, tagged::Field, usize),
    segment: usize,
    fragments: Vec<EditableFragment<SectionVariableId>>,
    occurrences: BTreeMap<&'static str, usize>,
}

/// The open catalog section for `key`, flushing the previous one when the key changed. Adjacent
/// parts of one field accumulate into ONE section; a key change is a segment boundary.
fn open_section<'a>(
    components: &mut Vec<RenderComponent<SectionKey, SectionVariableId>>,
    section: &'a mut Option<ActiveSection>,
    next_segment: &mut usize,
    key: (&'static str, tagged::Field, usize),
) -> &'a mut ActiveSection {
    if section.as_ref().is_some_and(|current| current.key != key) {
        flush_section(components, section);
    }
    section.get_or_insert_with(|| {
        let active = ActiveSection {
            key,
            segment: *next_segment,
            fragments: Vec::new(),
            occurrences: BTreeMap::new(),
        };
        *next_segment = next_segment.saturating_add(1);
        active
    })
}

fn flush_section(
    components: &mut Vec<RenderComponent<SectionKey, SectionVariableId>>,
    section: &mut Option<ActiveSection>,
) {
    let Some(section) = section.take() else {
        return;
    };
    components.push(RenderComponent::EditableSection(EditableSection::new(
        SectionKey {
            owner: String::from(section.key.0),
            field: section.key.1.as_str(),
            instance: section.key.2,
            segment: section.segment,
        },
        section.fragments,
    )));
}
