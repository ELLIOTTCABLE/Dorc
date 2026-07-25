//! The case-first catalog-lock generator (`28A` §4 checkpoint / `282` §8). The whole
//! `catalog_lock.rs` is derived from the defining cases (case-owned rows) plus the current lock
//! (ratcheted, case-less rows carried verbatim). The serializer lives in `dorc_aid::catalog`; this
//! module sources only the case-first fields — frontmatter `when_fires`/`why`, and the `example`
//! rendered from the defining payload.

use std::collections::BTreeMap;
use std::path::Path;

use dorc_aid::catalog::{CATALOG, LockRow, fill_template, refreshed_params, serialize_lock};
use dorc_aid::diag::params_of;
use dorc_core::Interner;
use errorloom::{Case, CaseRenderer};

use crate::DorcConsumer;

/// The fully-preflighted candidate set (`282:rul-promote-is-one-atomic-act`): the regenerated whole
/// lock plus every case's canonical render, computed and fixpoint-checked before any file write.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Publication {
    /// The candidate `catalog_lock.rs` bytes.
    pub lock: String,
    /// Each case's canonical bytes, keyed by defining slug.
    pub cases: BTreeMap<String, String>,
}

/// Compute the entire promote candidate set from the edited mirror, and prove BOTH fixpoints before
/// returning — pure, so a validation failure writes nothing (`282:rul-promote-is-one-atomic-act`).
/// The lock is generated from the whole `corpus` (frontmatter/payload only, unaffected by prose
/// edits). Only the `affected` cases are re-rendered and republished — unaffected cases are
/// byte-identical fixpoints and never touched. The render-level fixpoint: each affected candidate
/// re-renders to itself. The generated-lock fixpoint: regeneration is deterministic (the committed
/// byte-identity gate is the test-suite half, checked after the human rebuilds).
///
/// # Errors
/// Returns a refusal for a render/generation failure or either fixpoint mismatch; the caller then
/// leaves every committed file byte-identical.
pub fn build_publication(
    consumer: &DorcConsumer,
    corpus: &BTreeMap<String, Case>,
    affected: &BTreeMap<String, Case>,
) -> Result<Publication, String> {
    let lock = generate_catalog_lock(consumer, corpus)?;
    let mut rendered: BTreeMap<String, String> = BTreeMap::new();
    for (slug, case) in affected {
        let bytes = consumer
            .render_case(case)
            .map_err(|error| format!("render case `{slug}`: {error}"))?;
        let parsed = Case::parse(&bytes)
            .map_err(|error| format!("regenerated case `{slug}` does not re-parse: {error}"))?;
        let again = consumer
            .render_case(&parsed)
            .map_err(|error| format!("re-render case `{slug}`: {error}"))?;
        if again != bytes {
            return Err(format!("render-level fixpoint failed for `{slug}`"));
        }
        rendered.insert(slug.clone(), bytes);
    }
    if generate_catalog_lock(consumer, corpus)? != lock {
        return Err("generated-lock fixpoint (determinism) failed".to_owned());
    }
    Ok(Publication {
        lock,
        cases: rendered,
    })
}

/// Generate the whole `catalog_lock.rs` bytes from the consumer mirror and the defining cases keyed
/// by slug. Case-owned rows source `when_fires`/`why` from frontmatter and `example` from the
/// compiled message rendered with the defining payload; ratcheted rows carry their current generated
/// row verbatim. Deterministic — the output IS the committed bytes under the byte-identity fixpoint.
///
/// The row list is the UNION of the mirror and the case corpus (`288` §4 mirror-union, sourced from
/// CASES per `289` §2g flag-7): the mirror is closed over slugs that already have a lock row, so
/// without the union a newly-minted code could never gain one. Union rows APPEND in slug order after
/// the mirror's, so a mint is a pure addition to the committed bytes and the diff stays reviewable;
/// once promoted, the slug is in the mirror and the union is idempotent.
///
/// # Errors
/// Returns a refusal for missing frontmatter metadata, an un-fireable defining payload, or a message
/// hole absent from the payload.
pub fn generate_catalog_lock(
    consumer: &DorcConsumer,
    cases: &BTreeMap<String, Case>,
) -> Result<String, String> {
    let mut rows = Vec::with_capacity(consumer.mirror().len());
    for entry in consumer.mirror() {
        let message = entry.message.clone();
        let help = entry.help.clone();
        let params = refreshed_params(message.as_deref(), help.as_deref());
        let (when_fires, why, example) = if let Some(case) = cases.get(&entry.slug) {
            (
                frontmatter_scalar(case, "when-fires", &entry.slug)?,
                frontmatter_scalar(case, "why", &entry.slug)?,
                case_example(consumer, case, message.as_deref(), &entry.slug)?,
            )
        } else {
            let carried = CATALOG
                .iter()
                .find(|c| c.slug == entry.slug)
                .ok_or_else(|| {
                    format!("ratcheted `{}` absent from the current lock", entry.slug)
                })?;
            (
                carried.when_fires.to_owned(),
                carried.why.to_owned(),
                carried.example.to_owned(),
            )
        };
        rows.push(LockRow {
            slug: entry.slug.clone(),
            when_fires,
            why,
            params,
            example,
            message,
            help,
        });
    }
    for (slug, case) in cases {
        if consumer.mirror().iter().any(|entry| &entry.slug == slug) {
            continue;
        }
        rows.push(LockRow {
            slug: slug.clone(),
            when_fires: frontmatter_scalar(case, "when-fires", slug)?,
            why: frontmatter_scalar(case, "why", slug)?,
            params: refreshed_params(None, None),
            example: case_example(consumer, case, None, slug)?,
            message: None,
            help: None,
        });
    }
    Ok(serialize_lock(&rows))
}

/// Load every `<dir>/*.loom` defining case keyed by its frontmatter `code` slug. The edge I/O the
/// pure generator, the promote path, and the fixpoint gate all share.
///
/// # Errors
/// Returns a refusal for an unreadable directory/file, a malformed case, or a case without a `code`.
pub fn load_corpus_by_slug(dir: &Path) -> Result<BTreeMap<String, Case>, String> {
    let mut cases = BTreeMap::new();
    let entries = std::fs::read_dir(dir).map_err(|error| format!("read corpus dir: {error}"))?;
    for entry in entries {
        let path = entry
            .map_err(|error| format!("read corpus entry: {error}"))?
            .path();
        if path.extension().is_none_or(|extension| extension != "loom") {
            continue;
        }
        let text = std::fs::read_to_string(&path)
            .map_err(|error| format!("read case {}: {error}", path.display()))?;
        let case = Case::parse(&text)
            .map_err(|error| format!("parse case {}: {error}", path.display()))?;
        let slug = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| format!("case {} has no `code`", path.display()))?
            .to_owned();
        if cases.insert(slug.clone(), case).is_some() {
            return Err(format!("duplicate defining case for `{slug}`"));
        }
    }
    Ok(cases)
}

/// The concrete `example`: the compiled message rendered with the defining replay's typed payload; an
/// unwritten (`None`) message renders its `[unwritten: <slug>]` placeholder.
fn case_example(
    consumer: &DorcConsumer,
    case: &Case,
    message: Option<&str>,
    slug: &str,
) -> Result<String, String> {
    let Some(template) = message else {
        return Ok(format!("[unwritten: {slug}]"));
    };
    let diag = consumer.case_diag(case)?;
    let interner = Interner::default();
    let payload = params_of(&diag.code, &interner);
    let refs: Vec<(&str, &str)> = payload
        .iter()
        .map(|(key, value)| (*key, value.as_str()))
        .collect();
    fill_template(template, &refs).map_err(|error| format!("`{slug}` example: {error:?}"))
}

fn frontmatter_scalar(case: &Case, key: &str, slug: &str) -> Result<String, String> {
    case.frontmatter()
        .scalar(key)
        .map(str::to_owned)
        .ok_or_else(|| format!("case `{slug}` frontmatter missing `{key}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scaffolded_case(slug: &str) -> Case {
        let text = format!(
            "---\ncode: {slug}\nwhen-fires: a freshly minted code.\nwhy: the mint seam.\n---\n\
             -- replay --\n$ dorc plan --book=book.sh\nerror[{slug}]: placeholder\n"
        );
        Case::parse(&text).expect("scaffolded case parses")
    }

    /// The mint seam: a case whose slug has no mirror row still reaches the generated lock, as an
    /// APPENDED unwritten row. Without the union a new `DiagCode` variant could never gain a row
    /// (the mirror is seeded from the lock itself), which is the whole of `288` §4's trawl gap.
    #[test]
    fn a_caseless_new_slug_appends_an_unwritten_row() {
        let consumer = DorcConsumer::new();
        let cases = BTreeMap::from([(
            "aaa-brand-new-code".to_owned(),
            scaffolded_case("aaa-brand-new-code"),
        )]);
        let generated = generate_catalog_lock(&consumer, &cases).expect("generate lock");

        let row = generated
            .split("    CatalogEntry {\n")
            .last()
            .expect("at least one row");
        assert!(
            row.contains("slug: \"aaa-brand-new-code\","),
            "the new row is LAST despite sorting before every existing slug: {row}"
        );
        assert!(
            row.contains("message: None,"),
            "a minted row is unwritten: {row}"
        );
        assert!(
            row.contains("example: \"[unwritten: aaa-brand-new-code]\","),
            "the example renders the greppable placeholder: {row}"
        );
    }

    /// Union idempotence: once a slug is in the mirror, the case contributes its frontmatter to that
    /// row rather than a duplicate appended one.
    #[test]
    fn a_slug_already_in_the_mirror_gains_no_second_row() {
        let consumer = DorcConsumer::new();
        let cases =
            load_corpus_by_slug(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../aid/tests"))
                .expect("load corpus");
        let generated = generate_catalog_lock(&consumer, &cases).expect("generate lock");
        assert_eq!(
            generated.matches("slug: \"syntax-unsupported\",").count(),
            1,
            "a case-owned slug appears exactly once"
        );
    }
}
