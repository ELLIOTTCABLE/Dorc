use std::collections::BTreeSet;

use dorc_core::catalog::{TemplatePart, TemplateRefusal, parse_template};
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
    /// A marker was not whitespace-delimited.
    AttachedMarker(TemplateVariableName),
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
        .enumerate()
        .map(|(index, fragment)| match fragment {
            EditableFragment::Text(text) => parse_text(
                text,
                nearest_before(source, index),
                nearest_after(source, index),
            ),
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

fn parse_text(
    text: &str,
    external_before: Option<char>,
    external_after: Option<char>,
) -> Result<Vec<CompiledFragment>, CompileRefusal> {
    let parts = parse_template(text).map_err(CompileRefusal::Template)?;
    let mut out = Vec::new();
    for (index, part) in parts.iter().enumerate() {
        match part {
            TemplatePart::Literal(text) => out.push(CompiledFragment::Text(text.clone())),
            TemplatePart::Hole(name) => {
                let previous = parts
                    .get(index.wrapping_sub(1))
                    .and_then(|part| match part {
                        TemplatePart::Literal(text) => text.chars().last(),
                        TemplatePart::Hole(_) => None,
                    });
                let next = index
                    .checked_add(1)
                    .and_then(|next| parts.get(next))
                    .and_then(|part| match part {
                        TemplatePart::Literal(text) => text.chars().next(),
                        TemplatePart::Hole(_) => None,
                    });
                let name = TemplateVariableName(name.clone());
                if matches!(
                    index
                        .checked_sub(1)
                        .and_then(|previous| parts.get(previous)),
                    Some(TemplatePart::Hole(_))
                ) || matches!(
                    index.checked_add(1).and_then(|next| parts.get(next)),
                    Some(TemplatePart::Hole(_))
                ) || previous
                    .or(external_before)
                    .is_some_and(|c| !c.is_whitespace())
                    || next.or(external_after).is_some_and(|c| !c.is_whitespace())
                {
                    return Err(CompileRefusal::AttachedMarker(name));
                }
                out.push(CompiledFragment::Variable(name));
            }
        }
    }
    Ok(out)
}

fn nearest_before(fragments: &[EditableFragment<SectionVariableId>], index: usize) -> Option<char> {
    fragments
        .get(..index)
        .unwrap_or_default()
        .iter()
        .rev()
        .find_map(|fragment| match fragment {
            EditableFragment::Text(text) | EditableFragment::Variable { rendered: text, .. } => {
                text.chars().last()
            }
        })
}
fn nearest_after(fragments: &[EditableFragment<SectionVariableId>], index: usize) -> Option<char> {
    fragments.get(index.saturating_add(1)..).and_then(|rest| {
        rest.iter().find_map(|fragment| match fragment {
            EditableFragment::Text(text) | EditableFragment::Variable { rendered: text, .. } => {
                text.chars().next()
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_markers_require_whole_tokens() {
        assert_eq!(
            parse_text("run {{command}} \0", None, None),
            Ok(vec![
                CompiledFragment::Text(String::from("run ")),
                CompiledFragment::Variable(TemplateVariableName(String::from("command"))),
                CompiledFragment::Text(String::from(" \0")),
            ])
        );
        assert!(matches!(
            parse_text("run({{command}})", None, None),
            Err(CompileRefusal::AttachedMarker(_))
        ));
        assert!(matches!(
            parse_text("{{command}", None, None),
            Err(CompileRefusal::Template(_))
        ));
    }
}
