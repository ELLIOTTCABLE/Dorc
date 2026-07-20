//! The Dorc [`errorloom::Consumer`] (`282` §2 · `283:dec-consumer-in-dorc-loom`): the four methods
//! errorloom drives the bless loop over, implemented against a MUTABLE owned-catalog mirror
//! ([`dorc_core::catalog::OwnedEntry`]). errorloom owns extraction/orchestration; this consumer owns
//! the catalog, the tagged-render seat, and how a case's world materializes into a diagnostic.
//!
//! World-form dispatch (`283:dec-world-two-forms`): a `-- world --`-only case is WORLD-AS-PAYLOAD (a
//! canonical constructor keyed by slug — the phase-4 floor for the artificial/expensive-world codes);
//! a case carrying a materialized oracle/book section is WORLD-AS-PIPELINE (the real in-process kernel
//! fires the diagnostic — the marker pilot). Phase 4 lands the payload path; the pipeline arm is the
//! marker-version-unrecognized pilot.

use std::collections::BTreeMap;

use dorc_core::Interner;
use dorc_core::catalog::{OwnedEntry, owned_catalog};
use dorc_core::diag::{
    AidUnloadedSiblingOracle, DanglingReference, Diag, DiagCode, WhylogAbsent, WhylogBookDesync,
    WhylogCorrupt, WhylogVersionRefused, render_cli_tagged, render_cli_with,
};
use errorloom::{
    Case, Consumer, FieldTemplate, Fragment, ParamName, ParamTables, ParamValues,
    Region as LoomRegion, TaggedBaseline, Token, Word, tokenize,
};

use crate::{FieldKey, to_errorloom};

/// The Dorc consumer of the errorloom bless loop. Holds the mutable catalog mirror prose-bless edits
/// into; renders every case through the one production render seat parameterized by that mirror.
#[derive(Debug)]
pub struct DorcConsumer {
    mirror: Vec<OwnedEntry>,
}

impl Default for DorcConsumer {
    fn default() -> Self {
        Self::new()
    }
}

impl DorcConsumer {
    /// A consumer seeded from the compiled-in catalog (the carry-forward starting state).
    #[must_use]
    pub fn new() -> Self {
        DorcConsumer {
            mirror: owned_catalog(),
        }
    }

    /// The current mirror (test/inspection surface).
    #[must_use]
    pub fn mirror(&self) -> &[OwnedEntry] {
        &self.mirror
    }

    /// Overwrite a code's message in the mirror (models a raw catalog hand-edit for the fixpoint gate).
    pub fn set_message(&mut self, slug: &str, message: Option<String>) {
        if let Some(e) = self.mirror.iter_mut().find(|e| e.slug == slug) {
            e.message = message;
        }
    }

    /// The (diag, source, filename) a case materializes into. World-as-payload builds the canonical
    /// constructor keyed by the frontmatter `code`; world-as-pipeline is the marker pilot (added with
    /// that code). Spanless codes need no source.
    fn world_of(&self, case: &Case) -> Result<(Diag, String, String), String> {
        let slug = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        let diag = canonical_payload(slug)
            .ok_or_else(|| format!("no canonical world for `{slug}` (world-as-payload)"))?;
        Ok((diag, String::new(), String::new()))
    }

    /// The rendered transcript for a case from CURRENT mirror state — the shared body of
    /// `render_case` and the committed-case seed.
    fn render_transcript(&self, case: &Case) -> Result<String, String> {
        let (diag, src, filename) = self.world_of(case)?;
        let interner = Interner::default();
        Ok(render_cli_with(
            &self.mirror,
            &diag,
            &src,
            &filename,
            &interner,
        ))
    }
}

impl Consumer for DorcConsumer {
    type Key = FieldKey;
    type Error = String;

    fn tagged_render(&self, case: &Case) -> Result<TaggedBaseline<FieldKey>, String> {
        let (diag, src, filename) = self.world_of(case)?;
        let interner = Interner::default();
        let core = render_cli_tagged(&self.mirror, &diag, &src, &filename, &interner);
        let render = to_errorloom(&core).map_err(|e| e.to_string())?;
        let params = param_tables(&render);
        Ok(TaggedBaseline::new(render, params))
    }

    fn editable_text(&self, case: &Case) -> Result<String, String> {
        // The human render block — the first replay block whose command is NOT a `--format=` machine
        // view (`28A` §2n): machine blocks are whole-structural, never prose-edited.
        Ok(case
            .replay()
            .blocks()
            .iter()
            .find(|b| !b.command().contains("--format="))
            .map(|b| b.output().to_owned())
            .unwrap_or_default())
    }

    fn apply_field_edits(
        &mut self,
        edits: &BTreeMap<FieldKey, FieldTemplate>,
    ) -> Result<(), String> {
        for (key, template) in edits {
            let entry = self
                .mirror
                .iter_mut()
                .find(|e| e.slug == key.code)
                .ok_or_else(|| format!("no catalog entry for `{}`", key.code))?;
            let flat = flatten_template(template);
            match key.field {
                "message" => entry.message = Some(flat),
                "help" => entry.help = Some(flat),
                other => return Err(format!("edit named unknown field `{other}`")),
            }
        }
        Ok(())
    }

    fn render_case(&self, case: &Case) -> Result<String, String> {
        let output = self.render_transcript(case)?;
        let mut regenerated = case.clone();
        regenerated.set_replay_outputs(vec![output]);
        Ok(regenerated.to_text())
    }
}

/// The world-as-payload canonical constructors, keyed by slug (`283:dec-world-two-forms`). The five
/// roster codes are spanless in production (their emit context has no source point), so the
/// transcript is the frame-less title+body render. `dangling-reference` (also spanless, a
/// space-delimited `{coord}`) is carried for the prose-bless round-trip exercise.
fn canonical_payload(slug: &str) -> Option<Diag> {
    let code = match slug {
        "whylog-version-refused" => DiagCode::WhylogVersionRefused(WhylogVersionRefused {
            found: "dorc-whylog/2".to_owned(),
        }),
        "whylog-book-desync" => DiagCode::WhylogBookDesync(WhylogBookDesync {
            which: "book".to_owned(),
        }),
        "whylog-absent" => DiagCode::WhylogAbsent(WhylogAbsent {
            dir: "./.dorc/whylog".to_owned(),
        }),
        "whylog-corrupt" => DiagCode::WhylogCorrupt(WhylogCorrupt {
            detail: "no end-sentinel — a partial write?".to_owned(),
        }),
        "aid-unloaded-sibling-oracle" => {
            DiagCode::AidUnloadedSiblingOracle(AidUnloadedSiblingOracle {
                detail: "1 sibling oracle exists on disk but was not loaded: `redis.oracle.sh`"
                    .to_owned(),
            })
        }
        "dangling-reference" => DiagCode::DanglingReference(DanglingReference {
            coord: "sm.dorc.Package:nginx".to_owned(),
        }),
        _ => return None,
    };
    Some(Diag::new_spanless_site(code))
}

/// Flatten an errorloom [`FieldTemplate`] to the mirror's single-`String` form (`28A` §2c v1 — one
/// paragraph today): words verbatim, holes as `{param}`, paragraphs joined by a blank line.
fn flatten_template(template: &FieldTemplate) -> String {
    template
        .paragraphs()
        .iter()
        .map(|p| {
            p.fragments()
                .iter()
                .map(|f| match f {
                    Fragment::Word(w) => w.as_str().to_owned(),
                    Fragment::Hole(name) => format!("{{{}}}", name.as_str()),
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// Build the re-holing param tables errorloom needs from the tagged render's own `ParamValue` spans:
/// each `(code, field)` key maps its declared params to the word-sequence they rendered as. Foreign
/// (`ForeignText`) holes are excluded — they are never re-holed prose.
fn param_tables(render: &errorloom::TaggedRender<FieldKey>) -> ParamTables<FieldKey> {
    let mut per_key: BTreeMap<FieldKey, ParamValues> = BTreeMap::new();
    for span in render.spans() {
        if let LoomRegion::ParamValue { key, param, .. } = &span.region {
            let value = render.text().get(span.range.clone()).unwrap_or_default();
            let words: Vec<Word> = tokenize(value)
                .into_iter()
                .filter_map(|t| match t {
                    Token::Word(w) => Some(w),
                    Token::ParagraphBreak => None,
                })
                .collect();
            per_key
                .entry(key.clone())
                .or_default()
                .insert(ParamName::new(param.as_str()), words);
        }
    }
    let mut tables = ParamTables::new();
    for (key, values) in per_key {
        tables.insert(key, values);
    }
    tables
}
