//! dorc-loom — the Dorc editable-render adapter (`282` §4 · §13).

use std::collections::BTreeMap;

use dorc_aid::tagged::{self, RenderPart, RenderParts};
use errorloom::{EditableFragment, EditableRender, EditableSection, RenderComponent};

mod consumer;
pub use consumer::{
    DorcApplyRefusal, DorcConsumer, DorcEditableBaseline, DorcReplayDriver, SectionVariables,
    SeedRefusal, replay_case, replay_case_with_inputs,
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
mod ownership;
pub use ownership::{
    CaseOwnership, ComponentRef, EDIT_LOOP_KEY, ENVELOPE_KEY, ENVELOPE_STDERR, OWNS_KEY,
    corpus_ownership, edit_loop_hint, is_registered_component,
};
mod preview;
mod refusal;
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

/// The [`SectionKey::field`] value naming a WHOLE-PAGE arrangement entry
/// (`289:rul-arrangement-home-is-registry-plus-transcripts`): its bytes are the author's, laid
/// out by the author, and compile back VERBATIM. An arrangement section's
/// [`SectionKey::instance`] is its occurrence — the registry resolves an unclaimed occurrence to
/// the whole-slug entry, which is what lets repeated chrome share one entry.
pub const ARRANGEMENT_FIELD: &str = "arrangement";

/// The [`SectionKey::field`] value naming a chrome LINE a renderer laid out.
///
/// Distinct from [`ARRANGEMENT_FIELD`] because the whitespace means different things on the two
/// paths, and typing the difference is what stops one path's rule reaching the other
/// (`28H` ruling 7): a laid-out line's inter-word whitespace is the RENDERER's — a wrap it chose
/// at this width — so it collapses on the way back to storage, while a page's alignment and blank
/// lines are the author's and survive byte for byte.
pub const ARRANGEMENT_LINE_FIELD: &str = "arrangement-line";

/// The semantic name of the `index`-th value interleaved into a chrome line.
///
/// Positional rather than declared: an arrangement entry stores WORDS and never grew a param
/// vocabulary (`289` §2o), so the line's own value order is the only name available — and the
/// order IS the check, since a re-split whose variables are not `v0, v1, …` in sequence has moved
/// a value the render placed.
#[must_use]
pub fn arrangement_variable(index: usize) -> TemplateVariableName {
    TemplateVariableName(format!("v{index}"))
}

/// Map a core render-part stream to generic editable sections.
#[must_use]
pub fn to_editable_render(parts: &RenderParts) -> EditableRender<SectionKey, SectionVariableId> {
    let mut open = OpenSections::default();
    for part in parts.parts() {
        match part {
            RenderPart::TemplateLiteral {
                text,
                code,
                field,
                instance,
                ..
            } => open.catalog_text(code, *field, *instance, text),
            RenderPart::ParamValue {
                text,
                code,
                field,
                param,
                instance,
            } => open.catalog_value(code, *field, *instance, param, text),
            RenderPart::ForeignText { text, source } => open.fixed(source, text),
            RenderPart::Arrangement { text, .. } => open.structure(text),
            RenderPart::ArrangementPage { text, slug } => open.page(slug, text),
            RenderPart::ArrangementWords {
                text,
                slug,
                occurrence,
            } => open.line_text(slug, *occurrence, text),
            RenderPart::ArrangementValue {
                text,
                slug,
                occurrence,
                index,
            } => open.line_value(slug, *occurrence, *index, text),
        }
    }
    EditableRender::new(open.finish())
}

/// Which register an accumulating section belongs to. Two shapes rather than one because the two
/// registers are keyed differently: a catalog field by `(code, field, instance)`, a chrome line by
/// `(slug, occurrence)` with the render position as the fallback discriminator.
#[derive(Clone, PartialEq, Eq)]
enum ActiveKey {
    Catalog(&'static str, tagged::Field, usize),
    ArrangementLine {
        slug: &'static str,
        occurrence: Option<usize>,
    },
}

struct ActiveSection {
    key: ActiveKey,
    instance: usize,
    field: &'static str,
    owner: &'static str,
    segment: usize,
    fragments: Vec<EditableFragment<SectionVariableId>>,
    occurrences: BTreeMap<&'static str, usize>,
}

/// The part-stream walk's carried state: what has been emitted, and the section still open.
#[derive(Default)]
struct OpenSections {
    components: Vec<RenderComponent<SectionKey, SectionVariableId>>,
    section: Option<ActiveSection>,
    next_segment: usize,
    positions: BTreeMap<&'static str, usize>,
}

impl OpenSections {
    /// Open the catalog section for `(code, field, instance)`, flushing the previous one when the
    /// key changed. Adjacent parts of one field accumulate into ONE section.
    fn catalog_section(&mut self, code: &'static str, field: tagged::Field, instance: usize) {
        let key = ActiveKey::Catalog(code, field, instance);
        if self
            .section
            .as_ref()
            .is_some_and(|current| current.key != key)
        {
            self.flush();
        }
        if self.section.is_none() {
            self.section = Some(self.new_section(key, code, field.as_str(), instance));
        }
    }

    fn catalog_text(
        &mut self,
        code: &'static str,
        field: tagged::Field,
        instance: usize,
        text: &str,
    ) {
        self.catalog_section(code, field, instance);
        if let Some(open) = self.section.as_mut() {
            open.fragments
                .push(EditableFragment::Text(String::from(text)));
        }
    }

    fn catalog_value(
        &mut self,
        code: &'static str,
        field: tagged::Field,
        instance: usize,
        param: &'static str,
        text: &str,
    ) {
        self.catalog_section(code, field, instance);
        if let Some(open) = self.section.as_mut() {
            let occurrence = open.occurrences.entry(param).or_default();
            open.fragments.push(EditableFragment::Variable {
                id: SectionVariableId {
                    name: TemplateVariableName(String::from(param)),
                    occurrence: *occurrence,
                },
                rendered: String::from(text),
            });
            *occurrence = occurrence.saturating_add(1);
        }
    }

    /// Open the chrome-LINE section for `(slug, occurrence)`.
    ///
    /// This is the whole of `28H` ruling 3: adjacent words and interleaved values of ONE line
    /// accumulate into ONE section, so a value never splits a line into fragments the transport
    /// would have to re-anchor between — and the section's fragment series records exactly where
    /// the words re-divide, which is what makes the edit compile back.
    fn line_section(&mut self, slug: &'static str, occurrence: Option<usize>) {
        let key = ActiveKey::ArrangementLine { slug, occurrence };
        if self
            .section
            .as_ref()
            .is_some_and(|current| current.key != key)
        {
            self.flush();
        }
        if self.section.is_none() {
            // The render position is the discriminator only for an UNSTAMPED slug, and it counts
            // lines rather than parts, so it advances exactly when a new section opens.
            let position = self.positions.entry(slug).or_default();
            let instance = occurrence.unwrap_or(*position);
            *position = position.saturating_add(1);
            self.section = Some(self.new_section(key, slug, ARRANGEMENT_LINE_FIELD, instance));
        }
    }

    fn line_text(&mut self, slug: &'static str, occurrence: Option<usize>, text: &str) {
        self.line_section(slug, occurrence);
        if let Some(open) = self.section.as_mut() {
            open.fragments
                .push(EditableFragment::Text(String::from(text)));
        }
    }

    fn line_value(
        &mut self,
        slug: &'static str,
        occurrence: Option<usize>,
        index: usize,
        text: &str,
    ) {
        self.line_section(slug, occurrence);
        if let Some(open) = self.section.as_mut() {
            open.fragments.push(EditableFragment::Variable {
                id: SectionVariableId {
                    name: arrangement_variable(index),
                    occurrence: 0,
                },
                rendered: String::from(text),
            });
        }
    }

    fn new_section(
        &mut self,
        key: ActiveKey,
        owner: &'static str,
        field: &'static str,
        instance: usize,
    ) -> ActiveSection {
        let segment = self.next_segment;
        self.next_segment = segment.saturating_add(1);
        ActiveSection {
            key,
            instance,
            field,
            owner,
            segment,
            fragments: Vec::new(),
            occurrences: BTreeMap::new(),
        }
    }

    fn structure(&mut self, text: &str) {
        self.flush();
        self.components
            .push(RenderComponent::Structure(String::from(text)));
    }

    fn fixed(&mut self, source: &str, text: &str) {
        self.flush();
        self.components.push(RenderComponent::FixedVariable {
            id: SectionVariableId {
                name: TemplateVariableName(String::from(source)),
                occurrence: 0,
            },
            rendered: String::from(text),
        });
    }

    fn page(&mut self, slug: &'static str, text: &str) {
        self.flush();
        let segment = self.next_segment;
        self.next_segment = segment.saturating_add(1);
        self.components
            .push(RenderComponent::EditableSection(EditableSection::new(
                SectionKey {
                    owner: String::from(slug),
                    field: ARRANGEMENT_FIELD,
                    instance: 0,
                    segment,
                },
                vec![EditableFragment::Text(String::from(text))],
            )));
    }

    fn flush(&mut self) {
        let Some(section) = self.section.take() else {
            return;
        };
        self.components
            .push(RenderComponent::EditableSection(EditableSection::new(
                SectionKey {
                    owner: String::from(section.owner),
                    field: section.field,
                    instance: section.instance,
                    segment: section.segment,
                },
                section.fragments,
            )));
    }

    fn finish(mut self) -> Vec<RenderComponent<SectionKey, SectionVariableId>> {
        self.flush();
        self.components
    }
}
