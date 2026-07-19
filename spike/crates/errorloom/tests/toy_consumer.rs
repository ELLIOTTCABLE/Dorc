//! The toy-consumer end-to-end proof (`28A` §1 product cut): a tiny in-crate
//! consumer drives the WHOLE bless loop through the public API only — run,
//! prose-edit, prose-bless (fake git), catalog regenerate, corpus re-render,
//! the fixpoint gate, a structure-bless after an arrangement change, and the
//! never-both refusals. No Dorc types, no external process (the runner is proven
//! separately); this de-risks the phase-4 consumption.

use std::collections::BTreeMap;

use errorloom::{
    ArrangementSlug, BlessError, Case, CaseFile, Consumer, FakeGit, FieldTemplate, Fragment,
    ModeRefusal, ParamTables, Region, Span, TaggedBaseline, TaggedRender, fixpoint_check,
    prose_bless, structure_bless,
};

type Key = String;

/// A minimal CLI-tool stand-in: a catalog of `code → prose words`, and an
/// arrangement word (the message prefix) whose change models a code/structure
/// edit. Its transcript is `<arrangement>[<code>]: <prose>`.
struct Toy {
    catalog: BTreeMap<Key, Vec<String>>,
    arrangement: String,
}

impl Toy {
    fn render_message(&self, code: &str) -> (String, Vec<Span<Key>>) {
        let words = self.catalog.get(code).cloned().unwrap_or_default();
        let mut text = String::new();
        let mut spans: Vec<Span<Key>> = Vec::new();
        push(
            &mut text,
            &mut spans,
            arr("prefix"),
            &format!("{}[{code}]: ", self.arrangement),
        );
        push(&mut text, &mut spans, tl(code), &words.join(" "));
        push(&mut text, &mut spans, arr("newline"), "\n");
        (text, spans)
    }

    fn rendered_case_text(&self, case: &Case) -> Result<String, String> {
        let code = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        let (text, _spans) = self.render_message(code);
        let mut regenerated = case.clone();
        regenerated.set_replay_outputs(vec![text]);
        Ok(regenerated.to_text())
    }
}

impl Consumer for Toy {
    type Key = Key;
    type Error = String;

    fn tagged_render(&self, case: &Case) -> Result<TaggedBaseline<Key>, String> {
        let code = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        let (text, spans) = self.render_message(code);
        let render = TaggedRender::new(text, spans).map_err(|e| e.to_string())?;
        Ok(TaggedBaseline::new(render, ParamTables::new()))
    }

    fn editable_text(&self, case: &Case) -> Result<String, String> {
        Ok(case
            .replay()
            .blocks()
            .first()
            .map(|b| b.output().to_owned())
            .unwrap_or_default())
    }

    fn apply_field_edits(&mut self, edits: &BTreeMap<Key, FieldTemplate>) -> Result<(), String> {
        for (code, template) in edits {
            self.catalog.insert(code.clone(), template_words(template));
        }
        Ok(())
    }

    fn render_case(&self, case: &Case) -> Result<String, String> {
        self.rendered_case_text(case)
    }
}

const CASE_PATH: &str = "cases/the-slug.txt";
const CATALOG_PATH: &str = "src/catalog.rs";
const CODE_PATH: &str = "src/render.rs";

fn committed() -> String {
    "---\ncode: the-slug\n---\n-- replay --\n$ toy explain the-slug\nerror[the-slug]: alpha beta gamma\n".to_owned()
}

fn fresh_toy() -> Toy {
    let mut catalog = BTreeMap::new();
    catalog.insert(
        "the-slug".to_owned(),
        vec!["alpha".to_owned(), "beta".to_owned(), "gamma".to_owned()],
    );
    Toy {
        catalog,
        arrangement: "error".to_owned(),
    }
}

#[test]
fn full_prose_bless_loop_then_structure_bless() {
    let toy = fresh_toy();
    // run: the committed catalog reproduces the committed transcript (fixpoint).
    let corpus = vec![CaseFile::new(CASE_PATH, committed())];
    fixpoint_check(&toy, &corpus).expect("committed corpus is a fixpoint");

    // The author edits prose in the transcript (beta -> revised).
    let edited = committed().replace("alpha beta gamma", "alpha revised gamma");
    let git = FakeGit::new()
        .commit(CASE_PATH, committed())
        .mark_dirty(CASE_PATH);

    let mut toy = fresh_toy();
    let edited_corpus = vec![CaseFile::new(CASE_PATH, edited.clone())];
    let result = prose_bless(&mut toy, &git, &edited_corpus, CATALOG_PATH.as_ref())
        .expect("prose-bless succeeds");

    // The catalog absorbed the edit; every case re-rendered with the new prose.
    assert_eq!(
        toy.catalog.get("the-slug"),
        Some(&vec![
            "alpha".to_owned(),
            "revised".to_owned(),
            "gamma".to_owned()
        ])
    );
    let regenerated = result
        .regenerated()
        .get(std::path::Path::new(CASE_PATH))
        .expect("case regenerated");
    assert!(regenerated.contains("alpha revised gamma"));

    // The regenerated corpus is itself a fixpoint.
    let after = vec![CaseFile::new(CASE_PATH, regenerated.clone())];
    fixpoint_check(&toy, &after).expect("regenerated corpus is a fixpoint");

    // A code/arrangement change: structure-bless regenerates from the (clean) catalog.
    toy.arrangement = "problem".to_owned();
    let git = FakeGit::new()
        .commit(CASE_PATH, regenerated.clone())
        .mark_dirty(CODE_PATH);
    let structural = structure_bless(&toy, &git, &after, CATALOG_PATH.as_ref())
        .expect("structure-bless succeeds");
    let restructured = structural
        .regenerated()
        .get(std::path::Path::new(CASE_PATH))
        .expect("case regenerated");
    assert!(restructured.contains("problem[the-slug]: alpha revised gamma"));
}

#[test]
fn both_classes_dirty_refuses() {
    let mut toy = fresh_toy();
    let git = FakeGit::new()
        .commit(CASE_PATH, committed())
        .mark_dirty(CASE_PATH)
        .mark_dirty(CODE_PATH);
    let corpus = vec![CaseFile::new(CASE_PATH, committed())];
    let err = prose_bless(&mut toy, &git, &corpus, CATALOG_PATH.as_ref()).unwrap_err();
    assert_eq!(err, BlessError::Mode(ModeRefusal::BothClasses));
}

#[test]
fn dirty_catalog_refuses() {
    let mut toy = fresh_toy();
    let git = FakeGit::new()
        .commit(CASE_PATH, committed())
        .mark_dirty(CASE_PATH)
        .mark_dirty(CATALOG_PATH);
    let corpus = vec![CaseFile::new(CASE_PATH, committed())];
    let err = prose_bless(&mut toy, &git, &corpus, CATALOG_PATH.as_ref()).unwrap_err();
    assert_eq!(err, BlessError::Mode(ModeRefusal::DirtyCatalog));
}

#[test]
fn structure_drift_within_prose_bless_refuses() {
    // The arrangement changed in code (prefix error -> problem) AND the author
    // edited case prose, but only the case shows dirty — the baseline-verify
    // catches the structural drift and demands structure-bless first.
    let mut toy = fresh_toy();
    toy.arrangement = "problem".to_owned();
    let edited = committed().replace("alpha beta gamma", "alpha revised gamma");
    let git = FakeGit::new()
        .commit(CASE_PATH, committed())
        .mark_dirty(CASE_PATH);
    let corpus = vec![CaseFile::new(CASE_PATH, edited)];
    let err = prose_bless(&mut toy, &git, &corpus, CATALOG_PATH.as_ref()).unwrap_err();
    assert!(matches!(err, BlessError::StructureDrift { .. }));
}

#[test]
fn fixpoint_gate_catches_a_catalog_hand_edit() {
    // Hand-edit the catalog (prose) without regenerating transcripts: the
    // fixpoint gate sees the committed transcript no longer reproduce.
    let mut toy = fresh_toy();
    toy.catalog
        .insert("the-slug".to_owned(), vec!["tampered".to_owned()]);
    let corpus = vec![CaseFile::new(CASE_PATH, committed())];
    let err = fixpoint_check(&toy, &corpus).unwrap_err();
    assert!(matches!(err, BlessError::Fixpoint { .. }));
}

fn template_words(template: &FieldTemplate) -> Vec<String> {
    let mut words = Vec::new();
    for paragraph in template.paragraphs() {
        for fragment in paragraph.fragments() {
            if let Fragment::Word(word) = fragment {
                words.push(word.as_str().to_owned());
            }
        }
    }
    words
}

fn push(text: &mut String, spans: &mut Vec<Span<Key>>, region: Region<Key>, content: &str) {
    if content.is_empty() {
        return;
    }
    let start = text.len();
    text.push_str(content);
    spans.push(Span {
        range: start..text.len(),
        region,
    });
}

fn tl(code: &str) -> Region<Key> {
    Region::TemplateLiteral {
        key: code.to_owned(),
        paragraph: 0,
        instance: None,
    }
}

fn arr(slug: &str) -> Region<Key> {
    Region::Arrangement {
        slug: ArrangementSlug::new(slug),
    }
}
