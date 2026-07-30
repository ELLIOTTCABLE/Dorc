use std::collections::BTreeSet;

use dorc_aid::catalog::{TemplatePart, TemplateRefusal, parse_template};
use errorloom::EditableFragment;

use crate::{SectionVariableId, TemplateVariableName};

/// A compiled Dorc section fragment.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CompiledFragment {
    /// Ordinary text.
    Text(String),
    /// An exact-bound variable reference.
    Variable(TemplateVariableName),
}

/// The compiled form of one edited section.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompiledSection {
    fragments: Vec<CompiledFragment>,
    used: Vec<TemplateVariableName>,
    bindings: std::collections::BTreeMap<TemplateVariableName, String>,
}

impl CompiledSection {
    /// Compiled fragments.
    #[must_use]
    pub fn fragments(&self) -> &[CompiledFragment] {
        &self.fragments
    }
    /// Semantic names in first-use order.
    #[must_use]
    pub fn used(&self) -> &[TemplateVariableName] {
        &self.used
    }
    /// Validated exact bindings.
    #[must_use]
    pub fn bindings(&self) -> &std::collections::BTreeMap<TemplateVariableName, String> {
        &self.bindings
    }
    /// Render exact bound bytes.
    #[must_use]
    #[expect(
        clippy::indexing_slicing,
        reason = "compile_section stores every used binding"
    )]
    pub fn text(&self) -> String {
        self.fragments
            .iter()
            .map(|fragment| match fragment {
                CompiledFragment::Text(text) => text.clone(),
                CompiledFragment::Variable(name) => self.bindings[name].clone(),
            })
            .collect()
    }
}

/// Why section compilation refused.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CompileRefusal {
    /// Invalid strict marker syntax.
    Template(TemplateRefusal),
    /// No exact binding exists for a name.
    UnknownVariable(TemplateVariableName),
}

/// Compile generic section fragments with exact bindings.
///
/// # Errors
/// Returns [`CompileRefusal`] for invalid markers or bindings.
pub fn compile_fragments(
    source: &[EditableFragment<SectionVariableId>],
    values: &std::collections::BTreeMap<TemplateVariableName, String>,
) -> Result<CompiledSection, CompileRefusal> {
    let parsed: Result<Vec<_>, _> = source
        .iter()
        .map(|fragment| match fragment {
            EditableFragment::Text(text) => parse_text(text),
            EditableFragment::Variable { id, .. } => {
                Ok(vec![CompiledFragment::Variable(id.name.clone())])
            }
        })
        .collect();
    let parsed = parsed?;
    let explicit: BTreeSet<_> = source
        .iter()
        .zip(&parsed)
        .flat_map(|(source, fragments)| match source {
            EditableFragment::Text(_) => fragments
                .iter()
                .filter_map(|fragment| match fragment {
                    CompiledFragment::Variable(name) => Some(name.clone()),
                    CompiledFragment::Text(_) => None,
                })
                .collect::<Vec<_>>(),
            EditableFragment::Variable { .. } => Vec::new(),
        })
        .collect();
    let mut fragments = Vec::new();
    let mut used = Vec::new();
    for (source_fragment, fragments_for_source) in source.iter().zip(parsed) {
        for fragment in fragments_for_source {
            if let EditableFragment::Variable { id, .. } = source_fragment
                && explicit.contains(&id.name)
            {
                continue;
            }
            if let CompiledFragment::Variable(name) = &fragment {
                if !values.contains_key(name) {
                    return Err(CompileRefusal::UnknownVariable(name.clone()));
                }
                if !used.contains(name) {
                    used.push(name.clone());
                }
            }
            match (&mut fragments.last_mut(), fragment) {
                (Some(CompiledFragment::Text(previous)), CompiledFragment::Text(next)) => {
                    previous.push_str(&next);
                }
                (_, fragment) => fragments.push(fragment),
            }
        }
    }
    let bindings = used
        .iter()
        .map(|name| (name.clone(), values[name].clone()))
        .collect();
    Ok(CompiledSection {
        fragments,
        used,
        bindings,
    })
}

/// Split one edited text fragment at its marker tokens.
///
/// The TOKEN is what the grammar rules — `{{` + a NAME + `}}`, whole, with no interior whitespace
/// or expressions — and nothing about what SURROUNDS it (`28L:rul-attached-markers-land`, amending
/// `282:rul-double-brace-template-only`'s no-attached-punctuation clause). Twenty-six of the
/// corpus's own messages backtick-quote a value, so a whitespace-delimiter requirement made the
/// house idiom the one spelling an author could not newly write. Re-holing — the REVERSE direction,
/// discovering a marker from a rendered value — stays anchor-gated and deliberately stupid
/// (`282:rul-rehole-deliberately-stupid`), so a glued marker costs it nothing it had.
fn parse_text(text: &str) -> Result<Vec<CompiledFragment>, CompileRefusal> {
    let parts = parse_template(text).map_err(CompileRefusal::Template)?;
    Ok(parts
        .into_iter()
        .map(|part| match part {
            TemplatePart::Literal(text) => CompiledFragment::Text(text),
            TemplatePart::Hole(name) => CompiledFragment::Variable(TemplateVariableName(name)),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(text: &str) -> CompiledFragment {
        CompiledFragment::Text(String::from(text))
    }
    fn hole(name: &str) -> CompiledFragment {
        CompiledFragment::Variable(TemplateVariableName(String::from(name)))
    }

    /// The token is whole; its neighbours are none of the grammar's business.
    #[test]
    fn a_marker_is_a_whole_token_wherever_it_is_glued() {
        assert_eq!(
            parse_text("run {{command}} \0"),
            Ok(vec![text("run "), hole("command"), text(" \0")])
        );
        assert_eq!(
            parse_text("the flag `{{flag}}` is"),
            Ok(vec![text("the flag `"), hole("flag"), text("` is")]),
            "the corpus's own backtick idiom compiles"
        );
        assert_eq!(
            parse_text("({{a}}{{b}})"),
            Ok(vec![text("("), hole("a"), hole("b"), text(")")])
        );
        assert!(matches!(
            parse_text("{{command}"),
            Err(CompileRefusal::Template(_))
        ));
    }
}
