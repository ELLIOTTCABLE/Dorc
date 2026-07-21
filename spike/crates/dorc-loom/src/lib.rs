//! dorc-loom — the Dorc↔errorloom adapter (`282` §4 · `28A` §1).
//!
//! Dorc's render seat emits a core-owned tagged render
//! ([`dorc_core::tagged::TaggedRender`]); dorc-core takes NO dependency on
//! errorloom, so this crate is the ordinary consumer shape that maps those
//! core spans 1:1 onto errorloom's generic span schema, keyed by Dorc's
//! `(code, field)` ([`FieldKey`]). The mapping is validated through
//! `errorloom::TaggedRender::new`, which enforces the gap-free, non-overlapping
//! total cover (`28A:rul-span-cover-stays-total`).
//!
//! Instance ids are stamped ALWAYS on the template/param regions
//! (`28A:rul-tagged-render-emits-instance-ids`) — the per-key all-or-nothing
//! floor. The prose-promote flow itself lives in errorloom; this adapter only
//! produces the tagged baseline it consumes.

use std::collections::BTreeMap;

use dorc_core::tagged::{self, Region as CoreRegion, RenderPart, RenderParts};
use errorloom::{
    ArrangementSlug, EditableFragment, EditableRender, EditableSection, InstanceId, ParamName,
    Region, RenderComponent, Span, TaggedRender, TaggedRenderError,
};

mod consumer;
pub use consumer::{DorcApplyRefusal, DorcConsumer, DorcEditableBaseline, SectionVariables};
mod compile;
pub use compile::{CompileRefusal, CompiledFragment, CompiledSection, compile_fragments};
mod edit;
pub use edit::{DorcSectionEdit, DorcSectionEditRefusal, compile_section_edit};
mod inspect;
pub use inspect::render_compile_preview;
mod preview;
pub use preview::{CompilePreview, compile_preview};

/// The opaque consumer key errorloom groups prose fields by: Dorc's
/// `(code, field)` (`28A` §1). errorloom compares/sorts it but never inspects it;
/// the derives satisfy `errorloom::ConsumerKey` (`Clone + Ord + Debug`).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct FieldKey {
    /// The diagnostic code slug the field belongs to.
    pub code: String,
    /// Which prose register (`message`/`help`).
    pub field: &'static str,
}

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

/// The identity of one contiguous editable catalog field segment.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SectionKey {
    /// The diagnostic code slug.
    pub code: String,
    /// The catalog field name.
    pub field: &'static str,
    /// The core-emitted field occurrence.
    pub instance: usize,
    /// The deterministic split ordinal within this render.
    pub segment: usize,
}

/// Map a core render-part stream to generic editable sections.
#[must_use]
pub fn to_editable_render(parts: &RenderParts) -> EditableRender<SectionKey, SectionVariableId> {
    let mut components = Vec::new();
    let mut section = None;
    let mut next_segment = 0usize;
    for part in parts.parts() {
        match part {
            RenderPart::TemplateLiteral {
                text,
                code,
                field,
                instance,
                ..
            } => {
                let key = (*code, *field, *instance);
                if section
                    .as_ref()
                    .is_some_and(|current: &ActiveSection| current.key != key)
                {
                    flush_section(&mut components, &mut section);
                }
                let current = section.get_or_insert_with(|| {
                    let active = ActiveSection {
                        key,
                        segment: next_segment,
                        fragments: Vec::new(),
                        occurrences: BTreeMap::new(),
                    };
                    next_segment = next_segment.saturating_add(1);
                    active
                });
                current.fragments.push(EditableFragment::Text(text.clone()));
            }
            RenderPart::ParamValue {
                text,
                code,
                field,
                param,
                instance,
            } => {
                let key = (*code, *field, *instance);
                if section
                    .as_ref()
                    .is_some_and(|current: &ActiveSection| current.key != key)
                {
                    flush_section(&mut components, &mut section);
                }
                let current = section.get_or_insert_with(|| {
                    let active = ActiveSection {
                        key,
                        segment: next_segment,
                        fragments: Vec::new(),
                        occurrences: BTreeMap::new(),
                    };
                    next_segment = next_segment.saturating_add(1);
                    active
                });
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

fn flush_section(
    components: &mut Vec<RenderComponent<SectionKey, SectionVariableId>>,
    section: &mut Option<ActiveSection>,
) {
    let Some(section) = section.take() else {
        return;
    };
    components.push(RenderComponent::EditableSection(EditableSection::new(
        SectionKey {
            code: String::from(section.key.0),
            field: section.key.1.as_str(),
            instance: section.key.2,
            segment: section.segment,
        },
        section.fragments,
    )));
}

/// Map a core-owned tagged render onto an `errorloom::TaggedRender`, keyed by
/// [`FieldKey`], validating the gap-free total cover through
/// `errorloom::TaggedRender::new`.
///
/// # Errors
/// Returns [`TaggedRenderError`] when the core spans are not a gap-free,
/// non-overlapping cover of exactly the render bytes — a core-emitter bug,
/// caught fail-fast (the `inv-top-reject` posture).
pub fn to_errorloom(
    tagged: &tagged::TaggedRender,
) -> Result<TaggedRender<FieldKey>, TaggedRenderError> {
    let spans = tagged.spans().iter().map(map_span).collect();
    TaggedRender::new(tagged.text().to_owned(), spans)
}

fn map_span(span: &tagged::Span) -> Span<FieldKey> {
    Span {
        range: span.range.clone(),
        region: map_region(&span.region),
    }
}

fn map_region(region: &CoreRegion) -> Region<FieldKey> {
    match *region {
        CoreRegion::TemplateLiteral {
            code,
            field,
            paragraph,
            instance,
        } => Region::TemplateLiteral {
            key: key(code, field),
            paragraph,
            instance: Some(InstanceId::new(instance)),
        },
        CoreRegion::ParamValue {
            code,
            field,
            param,
            instance,
        } => Region::ParamValue {
            key: key(code, field),
            param: ParamName::new(param),
            instance: Some(InstanceId::new(instance)),
        },
        CoreRegion::ForeignText { param } => Region::ForeignText {
            param: ParamName::new(param),
        },
        CoreRegion::Arrangement { slug } => Region::Arrangement {
            slug: ArrangementSlug::new(slug),
        },
    }
}

fn key(code: &str, field: tagged::Field) -> FieldKey {
    FieldKey {
        code: code.to_owned(),
        field: field.as_str(),
    }
}
