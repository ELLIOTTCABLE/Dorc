//! Author-facing refusal text (`28L:rul-refusals-name-the-next-command`).
//!
//! Every sentence here ends in the exact command to run or the exact file to open. The audience is
//! someone editing a `.loom` who may not open the crate that refused them — for them a refusal is
//! not a log line, it is the entire teaching surface
//! (`28L:rul-rust-and-loom-are-the-only-edit-surfaces`). `282:rul-internal-tool-sharp-edges`
//! permits blunt; it does not permit unactionable.

use std::path::Path;

use dorc_aid::catalog::TemplateRefusal;

use crate::{CompileRefusal, DorcApplyRefusal, DorcSectionEditRefusal};

/// The Rust file a new diagnostic value is added in — quoted verbatim by the refusals below,
/// because a loom author has no other way to learn it.
const PAYLOAD_SOURCE: &str = "spike/crates/aid/src/diag.rs";

impl DorcSectionEditRefusal {
    /// This refusal as a sentence ending in the next command or edit, for the loom at `case`.
    #[must_use]
    pub fn explain(&self, case: &Path) -> String {
        let case = case.display().to_string();
        match self {
            Self::Unchanged => format!(
                "no transcript bytes changed. Edit the prose inside {case}, then: \
                 dorc-loom publish {case}"
            ),
            Self::UnknownVariable(name) => unknown_value(&name.0, &case),
            Self::Template(refusal) => template_refusal(refusal, &case),
            Self::Compile(refusal) => compile_refusal(refusal, &case),
            Self::Transport(refusal) => format!(
                "the edit could not be attributed to one editable region ({refusal:?}). Undo it \
                 and change the words of a single sentence, leaving the surrounding structure \
                 byte-identical; then: dorc-loom publish {case}"
            ),
            Self::AmbiguousCandidate => format!(
                "the edit matches more than one editable region, so nothing here can know which \
                 one you meant. Make the change in one region at a time, then: \
                 dorc-loom publish {case}"
            ),
            Self::MarkerOutsideEditableSection => format!(
                "the edit touched bytes the renderer computed — a severity word, a caret frame, a \
                 line break — rather than the prose inside an editable region. Restore those bytes \
                 in {case} and edit only the sentence text, then: dorc-loom publish {case}"
            ),
            Self::CandidateMismatch => format!(
                "the marker and the edited bytes point at different regions. Put the marker in the \
                 same sentence you edited, then: dorc-loom publish {case}"
            ),
            Self::SplitEditableField(key) => format!(
                "`{}`'s {} register renders in more than one place here, so no single edit owns \
                 it — rewriting one piece would leave the rest saying the old thing. This is the \
                 render's problem rather than the edit's: restore {case} to its committed bytes \
                 and report the case.",
                key.owner, key.field
            ),
            Self::ForeignComponent { component, owner } => format!(
                "`{component}` is authored in {owner} — edit it there; this case only renders it. \
                 Undo the change here, make it in {owner}, then: dorc-loom publish {owner}"
            ),
        }
    }
}

impl DorcApplyRefusal {
    /// This refusal as a sentence ending in the next command or edit, for the loom at `case`.
    #[must_use]
    pub fn explain(&self, case: &Path) -> String {
        let case = case.display().to_string();
        match self {
            Self::MissingCode(slug) => format!(
                "no catalog row for `{slug}` yet. Run: dorc-loom publish {case}, then rebuild \
                 — the build reads the committed lock, so a freshly promoted row needs one"
            ),
            Self::MissingArrangement(slug) => format!(
                "no registry row for chrome `{slug}` yet. Run: dorc-loom publish {case}, then \
                 rebuild"
            ),
            Self::IllegalField(field) => format!(
                "`{field}` is not an editable register. Edit the diagnostic's message or help \
                 prose in {case} instead, then: dorc-loom publish {case}"
            ),
            Self::ArrangementEntryEditedTwice {
                slug,
                first,
                second,
            } => format!(
                "chrome `{slug}` is rendered twice in this transcript and the two copies were \
                 edited differently ({} vs {}). They are ONE registry row, so applying both would \
                 silently keep the last. Make both copies say the same thing in {case}, then: \
                 dorc-loom publish {case}",
                quoted(first),
                quoted(second)
            ),
            Self::ArrangementValueSequenceChanged {
                slug,
                expected,
                found,
                editable_words,
            } => value_sequence_changed(slug, expected, found, editable_words, &case),
        }
    }
}

/// The one seat that teaches where a diagnostic's values come from (`28L` map §8.3 item 4). This
/// is the ONLY thing a loom author is shown when they ask for a value that does not exist, so it
/// names the command that lists the real ones and the Rust file that mints a new one.
fn unknown_value(name: &str, case: &str) -> String {
    format!(
        "no value `{name}` on this diagnostic's payload. The values it carries are listed by: \
         dorc-loom vars --all {case}. To add one, add the field to its payload struct and its \
         `params_of_raw` arm in {PAYLOAD_SOURCE} (the arm is a compile error until you do), fill \
         it in at the emit site the compiler names, then rebuild."
    )
}

/// A computed value is an ordinary-looking English word the render produced, so an author edits it
/// without knowing it was never theirs (`28L` map fnd-computed-words-are-invisible). Say which
/// words on the line ARE theirs.
fn value_sequence_changed(
    slug: &str,
    expected: &[String],
    found: &[String],
    editable_words: &[String],
    case: &str,
) -> String {
    let words = if editable_words.is_empty() {
        "this line has no authored words yet".to_owned()
    } else {
        format!("the words you can edit are {}", quoted(editable_words))
    };
    format!(
        "chrome `{slug}`: some of what this line prints is a value the renderer computed — a \
         count, a name, a severity — not words anyone authored, and the edit moved, dropped or \
         duplicated one (stamped {}, edited to {}). Rephrase around each value, leaving it where \
         the render put it: {words}. Then: dorc-loom publish {case}",
        quoted(expected),
        quoted(found)
    )
}

fn template_refusal(refusal: &TemplateRefusal, case: &str) -> String {
    match refusal {
        TemplateRefusal::Malformed => format!(
            "a double-brace marker in the edit is malformed. A marker is exactly {{{{name}}}} — \
             two braces each side, no spaces inside. Fix it in {case}, then: \
             dorc-loom publish {case}"
        ),
        TemplateRefusal::UnknownParam(name) => unknown_value(name, case),
    }
}

fn compile_refusal(refusal: &CompileRefusal, case: &str) -> String {
    match refusal {
        CompileRefusal::Template(refusal) => template_refusal(refusal, case),
        CompileRefusal::UnknownVariable(name) => unknown_value(&name.0, case),
    }
}

fn quoted(words: &[String]) -> String {
    if words.is_empty() {
        return "nothing".to_owned();
    }
    words
        .iter()
        .map(|word| format!("`{word}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TemplateVariableName;

    /// Remit B of the acceptance gate (`28L` map §8.2) is "add a value that does not exist yet,
    /// from Rust", and this refusal is the ONLY surface that persona is permitted to read. If it
    /// stops naming the listing command and the payload file, that remit is a dead end again.
    #[test]
    fn an_unknown_value_names_the_listing_command_and_the_rust_file() {
        let refusal =
            DorcSectionEditRefusal::UnknownVariable(TemplateVariableName(String::from("host")));
        let text = refusal.explain(Path::new("crates/aid/tests/cli-file-not-found.loom"));
        assert!(
            text.contains("dorc-loom vars --all crates/aid/tests/cli-file-not-found.loom"),
            "the refusal must name the command that lists the real values: {text}"
        );
        assert!(
            text.contains(PAYLOAD_SOURCE) && text.contains("params_of_raw"),
            "the refusal must name the Rust file AND the seat that publishes a value: {text}"
        );
    }
}
