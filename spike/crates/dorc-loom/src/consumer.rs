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

use std::collections::{BTreeMap, BTreeSet};

use dorc_core::catalog::{OwnedEntry, owned_catalog};
use dorc_core::diag::{
    AidUnloadedSiblingOracle, CarriedAcrossSubstrateAxis, CmdsubOperandTop, CommandName,
    DanglingReference, Diag, DiagCode, EscalationPolicy, MarkHashcolonMalformed, MarkRcArityExceeded,
    MarkStandaloneRcConsumer, MarkUnknownVerb, MissingDialectMarker, MungeNameInvalid,
    OperandPosition, RecordsFactTruncated, RenderHeredocRefused, SiteId, SiteUnresolvable,
    SyntaxUnsupported, ToleratesUnknownDimension, WhylogAbsent, WhylogBookDesync, WhylogCorrupt,
    WhylogVersionRefused, WrapperPeelIncoherent, render_cli_tagged, render_cli_with,
};
use dorc_core::{Interner, LeafId, ProvArena, Severity, TopCause};
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

    /// The (diag, source, filename) a case materializes into (`283:dec-world-two-forms`). A case
    /// carrying a materialized `*.oracle.sh` section is WORLD-AS-PIPELINE: the REAL in-process marker
    /// gate fires the diagnostic over that source (the one real-fired proof, `28A` §2n) — a spanned
    /// diag whose caret frame points into it. Otherwise WORLD-AS-PAYLOAD: the canonical constructor
    /// keyed by the frontmatter `code` (spanless roster codes need no source).
    fn world_of(case: &Case) -> Result<(Diag, String, String), String> {
        let slug = case
            .frontmatter()
            .scalar("code")
            .ok_or_else(|| "case has no `code`".to_owned())?;
        if let Some(section) = case
            .sections()
            .iter()
            .find(|s| s.name().ends_with("oracle.sh"))
        {
            return fire_marker_gate(slug, section.name(), section.content());
        }
        if let Some(section) = case.sections().iter().find(|s| s.name() == "book.sh") {
            return fire_book_analysis(slug, section.name(), section.content());
        }
        let diag = canonical_payload(slug)
            .ok_or_else(|| format!("no canonical world for `{slug}` (world-as-payload)"))?;
        Ok((diag, String::new(), String::new()))
    }

    /// The human transcript (the diagnostic's CLI render) for a case from CURRENT mirror state,
    /// plus the diag itself so a sibling `--format=jsonl` replay can render the machine view of the
    /// SAME world (`282:rul-multi-replay-per-case`).
    fn render_world(&self, case: &Case) -> Result<(Diag, String), String> {
        let (diag, src, filename) = Self::world_of(case)?;
        let interner = Interner::default();
        let human = render_cli_with(&self.mirror, &diag, &src, &filename, &interner);
        Ok((diag, human))
    }
}

impl Consumer for DorcConsumer {
    type Key = FieldKey;
    type Error = String;

    fn tagged_render(&self, case: &Case) -> Result<TaggedBaseline<FieldKey>, String> {
        let (diag, src, filename) = Self::world_of(case)?;
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
        edits: BTreeMap<FieldKey, FieldTemplate>,
    ) -> Result<(), String> {
        for (key, template) in edits {
            let entry = self
                .mirror
                .iter_mut()
                .find(|e| e.slug == key.code)
                .ok_or_else(|| format!("no catalog entry for `{}`", key.code))?;
            let flat = flatten_template(&template);
            match key.field {
                "message" => entry.message = Some(flat),
                "help" => entry.help = Some(flat),
                other => return Err(format!("edit named unknown field `{other}`")),
            }
        }
        Ok(())
    }

    fn render_case(&self, case: &Case) -> Result<String, String> {
        let (diag, human) = self.render_world(case)?;
        let outputs = case
            .replay()
            .blocks()
            .iter()
            .map(|block| {
                if block.command().contains("--format=jsonl") {
                    render_diag_jsonl(&diag)
                } else {
                    reflow_to_canonical(&human)
                }
            })
            .collect();
        let mut regenerated = case.clone();
        regenerated.set_replay_outputs(outputs);
        Ok(regenerated.to_text())
    }
}

/// The corpus's pinned canonical render width (`282` §3): committed transcripts word-wrap HERE, not
/// at a terminal — a live surface may wrap adaptively, the corpus does not. The committed file's own
/// hard-wrapping is normalized away on read-in (the whitespace-collapsing prose tokenizer) and
/// regenerated at this width, so the on-disk layout is render-owned, never author-owned.
const CANONICAL_WIDTH: usize = 80;

/// Reflow a flat CLI diagnostic render to [`CANONICAL_WIDTH`] (`282` §3, item-6 layout): the title
/// line wraps under a 3-space hanging indent (the message body under the title) and each
/// `= help:` / `= note:` block wraps under a 6-space hanging indent, its `=` marker re-aligned to the
/// frame's gutter column; the caret-frame lines pass through verbatim. This corpus surface owns the
/// wrap so a committed file's editing-time layout never reaches the fixpoint assertion.
fn reflow_to_canonical(render: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    for (i, line) in render.lines().enumerate() {
        if i == 0
            && let Some(cut) = line.find("]: ")
        {
            let (prefix, text) = line.split_at(cut.saturating_add(3));
            out.push(wrap_words(prefix, "   ", text));
            continue;
        }
        if let Some(rest) = line.trim_start().strip_prefix("= ")
            && let Some(cut) = rest.find(": ")
        {
            let marker = rest.get(..cut.saturating_add(2)).unwrap_or(rest);
            let text = rest.get(cut.saturating_add(2)..).unwrap_or("");
            out.push(wrap_words(&format!("   = {marker}"), "      ", text));
            continue;
        }
        out.push(line.to_owned());
    }
    out.join("\n")
}

/// Greedy word-wrap of `text` beneath `prefix` (the un-wrapped first-line lead-in) at
/// [`CANONICAL_WIDTH`], every continuation line carrying `cont_indent`. Column counting is by
/// `char`, so the one-column `—`/`…` glyphs the prose uses count as one. Pure; total.
fn wrap_words(prefix: &str, cont_indent: &str, text: &str) -> String {
    let mut out = String::from(prefix);
    let mut col = prefix.chars().count();
    let mut started = false;
    for word in text.split_whitespace() {
        let wlen = word.chars().count();
        if !started {
            out.push_str(word);
            col = col.saturating_add(wlen);
            started = true;
        } else if col.saturating_add(1).saturating_add(wlen) <= CANONICAL_WIDTH {
            out.push(' ');
            out.push_str(word);
            col = col.saturating_add(1).saturating_add(wlen);
        } else {
            out.push('\n');
            out.push_str(cont_indent);
            out.push_str(word);
            col = cont_indent.chars().count().saturating_add(wlen);
        }
    }
    out
}

/// The compact machine view of a single diagnostic for a `--format=jsonl` replay block (`282` §2
/// machine-format replay · `282:rul-multi-replay-per-case`): one JSON object carrying the code slug
/// (the same-slug coherence gate every replay must pass) and its registry severity word. Both are
/// bare identifiers — no user text, so no escaping is possible — and it is a tool-corpus surface, not
/// a product API (`27V:rul-output-form-unwelded`; the machine format is free to churn). Trailing LF so
/// the block round-trips through the container's `set_replay_outputs`/`to_text` unchanged.
fn render_diag_jsonl(diag: &Diag) -> String {
    let severity = match diag.severity() {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Note => "note",
    };
    format!(
        "{{\"code\":\"{}\",\"severity\":\"{severity}\"}}\n",
        diag.code.slug(),
    )
}

/// The world-as-payload canonical constructors, keyed by slug (`283:dec-world-two-forms`). The
/// phase-5 backport (`283` §5.9) renders every non-pipeline covered code SPANLESS: a code may carry a
/// span in production, but its defining case pins the frame-less title+body prose registers (the
/// authoring surface), not the caret frame — that is the marker pilot's world-as-pipeline job.
fn canonical_payload(slug: &str) -> Option<Diag> {
    let code = match slug {
        // phase-5 backport: the covered give-up / records / mark-grammar codes.
        "cmdsub-operand-top" => DiagCode::CmdsubOperandTop(CmdsubOperandTop {
            site: SiteId::leaf(LeafId(3)),
            position: OperandPosition::Operand(1),
            cause: None,
            top_cause: TopCause::UnmodeledExpansion,
            command: CommandName::Literal("apt-get".to_owned()),
        }),
        "site-unresolvable" => DiagCode::SiteUnresolvable(SiteUnresolvable {
            site: SiteId::leaf(LeafId(4)),
            detail: "2 sites run unprobed (no read-only check could be shipped): \
                     `make install`, `ldconfig`"
                .to_owned(),
        }),
        "render-heredoc-refused" => DiagCode::RenderHeredocRefused(RenderHeredocRefused {
            site: SiteId::leaf(LeafId(7)),
            verb: "elide",
            command: "cat <<EOF".to_owned(),
        }),
        "syntax-unsupported" => DiagCode::SyntaxUnsupported(SyntaxUnsupported {
            detail: "process substitution `<(…)` is not modeled".to_owned(),
        }),
        "missing-dialect-marker" => DiagCode::MissingDialectMarker(MissingDialectMarker),
        "munge-name-invalid" => DiagCode::MungeNameInvalid(MungeNameInvalid {
            source: "9pkg".to_owned(),
            funcname: "9pkg".to_owned(),
            problem: "starts with a digit".to_owned(),
        }),
        "tolerates-unknown-dimension" => {
            DiagCode::ToleratesUnknownDimension(ToleratesUnknownDimension {
                token: "netns2".to_owned(),
                expected: "user, netns, fs-view".to_owned(),
            })
        }
        "records-fact-truncated" => DiagCode::RecordsFactTruncated(RecordsFactTruncated {
            received: 3,
            declared: 5,
            unseen: 2,
        }),
        "escalation-policy" => DiagCode::EscalationPolicy(EscalationPolicy {
            detail: "escalation policy: probe re-uses connection authority for \
                     `tolerates:`-vouched functions only (default)"
                .to_owned(),
        }),
        "carried-across-substrate-axis" => {
            DiagCode::CarriedAcrossSubstrateAxis(CarriedAcrossSubstrateAxis {
                detail: "elision carried across the fs-view axis: backing kind `sm_dorc_File` \
                         vouches `invariant:fs-view`; the verdict body is read-set-closed"
                    .to_owned(),
            })
        }
        "wrapper-peel-incoherent" => DiagCode::WrapperPeelIncoherent(WrapperPeelIncoherent {
            detail: "wrapper `sudo`: __predict and __lend_map disagree on the peel tail \
                     position (predict reaches \"$@\" after 1 argv token(s), lend_map after 0)"
                .to_owned(),
        }),
        "mark-unknown-verb" => DiagCode::MarkUnknownVerb(MarkUnknownVerb {
            token: "frobnicate".to_owned(),
            expected: "asserts, refutes, reads, bind, safe-across, disturbs, lends, \
                       stored-in, undivided-by-transit-across"
                .to_owned(),
        }),
        "mark-rc-arity-exceeded" => DiagCode::MarkRcArityExceeded(MarkRcArityExceeded),
        "mark-standalone-rc-consumer" => {
            DiagCode::MarkStandaloneRcConsumer(MarkStandaloneRcConsumer)
        }
        "mark-hashcolon-malformed" => DiagCode::MarkHashcolonMalformed(MarkHashcolonMalformed),
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

/// World-as-pipeline for the marker pilot (`28A` §2n): fire the REAL in-process marker gate over the
/// materialized oracle `source`, returning its (spanned) diagnostic + the source the caret frame
/// resolves against. Refuses if the gate fired nothing or a different code than the case declares
/// (the honest-trigger coherence the world-as-pipeline form buys).
fn fire_marker_gate(
    slug: &str,
    filename: &str,
    source: &str,
) -> Result<(Diag, String, String), String> {
    let mut interner = Interner::default();
    let diag = dorc_oracle::marker::check_dialect_marker(&mut interner, source)
        .into_iter()
        .next()
        .ok_or_else(|| format!("world-as-pipeline `{slug}` fired no diagnostic"))?;
    if diag.code.slug() != slug {
        return Err(format!(
            "world-as-pipeline `{slug}` fired `{}` — the case's `code` must match the fired diagnostic",
            diag.code.slug()
        ));
    }
    Ok((diag, source.to_owned(), filename.to_owned()))
}

/// World-as-pipeline for the cmdsub flagship (`28A` §2n, extended to the analysis kernel): fire the
/// REAL pipeline (parse → cfg → value → classify with NO oracles loaded) over the materialized
/// `book.sh`, returning the (spanned) diagnostic whose slug matches the case's `code` + the source its
/// caret frame resolves against. The ⊤-operand disclosure fires before any oracle argparse, so an
/// empty [`dorc_oracle::KindIndex`] suffices; the whole path is kernel-pure (`inv-determinism`).
/// Refuses if the pipeline fired nothing matching the declared slug (honest-trigger coherence).
fn fire_book_analysis(
    slug: &str,
    filename: &str,
    source: &str,
) -> Result<(Diag, String, String), String> {
    let mut interner = Interner::default();
    let parsed = dorc_syntax::parse(source);
    let cfg = dorc_analysis::cfg::build(&parsed.value);
    let value = dorc_analysis::value::analyze(&cfg.value, &parsed.value, &mut interner);
    let idx = dorc_oracle::KindIndex::default();
    let mut arena = ProvArena::new();
    let diag = dorc_analysis::effect::classify(
        &cfg.value,
        &value,
        &parsed.value,
        &idx,
        &[],
        &BTreeSet::new(),
        &mut interner,
        &mut arena,
    )
    .diags
    .into_iter()
    .find(|d| d.code.slug() == slug)
    .ok_or_else(|| format!("world-as-pipeline `{slug}` fired no `{slug}` diagnostic"))?;
    Ok((diag, source.to_owned(), filename.to_owned()))
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
