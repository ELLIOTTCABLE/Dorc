//! The ONE committed diagnostic catalog — the single home for user-facing diagnostic prose
//! (`27V:rul-kill-legacy-diagnostic` · `AID-NEEDS:defining-case-catalog`). Message text lives
//! in `catalog_lock.rs` as generated data, keyed by code slug; render arms pull templates from this table and fill the
//! named params from a [`crate::diag::Diag`]'s typed payload. Nothing else authors user-facing
//! prose.
//!
//! # Shape (conductor-ruled `amendment-catalog-fields-are-data`, 2026-07-18)
//!
//! A generated Rust `const` table of [`CatalogEntry`] struct literals — the compiler is the parser
//! (no `build.rs`, no hand-rolled format, no proc-macro: `inv-no-unsafe` stands). Per-entry
//! metadata (`when_fires` / `why` / `params` / `example`) is STRUCTURED DATA the gate tests
//! check, never a comment block. Editing prose is editing a raw string literal in place, and one
//! final compile takes effect — no per-edit codegen step (`amendment-single-bless-confirmed`).
//! The d4 promote pipeline writes the whole generated target, staying diffable and committed.
//!
//! # The three legal states of a `message` (mechanically gated)
//!
//! Every [`CatalogEntry::message`] (and [`CatalogEntry::help`]) is ONE of:
//! * `sm `-prefixed prior-builder prose migrated VERBATIM from the base tip (`380f2fa`) — the
//!   `sm ` marker means "builder prose awaiting human rewrite" (`27V:rul-error-authorship-tier`,
//!   sharpened by `amendment-prose-boundary`);
//! * the exact placeholder `[unwritten: <slug>]` for any user-facing string that did NOT exist at
//!   the base tip (a new or split code) — builders author ZERO new user-facing prose; or
//! * conductor/human-authored prose, unprefixed, whose slug is listed in the gate test's
//!   `CONDUCTOR_AUTHORED` roster (adding prose without the roster entry fails the gate; a builder
//!   may never extend the roster).
//!
//! The metadata fields (`when_fires` / `why` / `params` / `example`) are conductor/machine-facing,
//! authored by the builder, and carry NO prefix.

/// One catalog entry: the code linkage + the structured metadata + the user-facing prose
/// registers (`27V` §3). Keyed to a [`crate::diag::DiagCode`] by its stable [`slug`](Self::slug)
/// (the wire token `DiagCode::slug()` returns), so the render pulls this entry by slug and fills
/// [`message`](Self::message)'s `{named}` holes from the diag's typed payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CatalogEntry {
    /// Code linkage: the stable slug matching `crate::diag::DiagCode::slug()`.
    pub slug: &'static str,
    /// When this diagnostic fires (conductor/machine-facing metadata; builder-authored).
    pub when_fires: &'static str,
    /// Why the code exists — cites the governing slug(s) (conductor/machine-facing metadata).
    pub why: &'static str,
    /// The named params the templates may interpolate — the closed set of `{holes}`
    /// [`message`](Self::message) and [`help`](Self::help) are allowed to reference (gate-checked).
    pub params: &'static [&'static str],
    /// One concrete example instantiation of the rendered message (metadata; builder-authored).
    pub example: &'static str,
    /// The user-facing PRIMARY message template — `sm `-prefixed base-tip prose, or `None` when
    /// unwritten (`283:dec-message-becomes-option`): the render synthesizes the `[unwritten: <slug>]`
    /// placeholder, never a stored string. `{name}` holes are filled from the payload; `{{`/`}}`
    /// escape literal braces.
    pub message: Option<&'static str>,
    /// The optional user-facing remediation/help register — same two legal states as
    /// [`message`](Self::message), or `None` when the code carries no help.
    pub help: Option<&'static str>,
}

#[path = "catalog_lock.rs"]
mod catalog_lock;
pub use catalog_lock::CATALOG;

/// The catalog entry for `slug`, or `None` when the slug has no entry (dead code path pre-sweep;
/// the phase-2 completeness gate makes a missing entry a test failure once every variant is
/// populated). Linear scan — the table is small and analysis-side big-O never constrains
/// (`spike/CLAUDE.md perf-doctrine`).
#[must_use]
pub fn entry(slug: &str) -> Option<&'static CatalogEntry> {
    CATALOG.iter().find(|e| e.slug == slug)
}

/// The render seat's view of the prose catalog (`283:dec-mirror-via-catalog-lookup`): the
/// message/help templates keyed by slug, so a render can source prose from the compiled-in const
/// OR a promote-time mutable mirror through ONE seat. `None` from [`message`](Self::message) means
/// "no written message" (either no entry, or an unwritten one) — the render synthesizes the
/// `[unwritten: <slug>]` placeholder in both cases; `None` from [`help`](Self::help) means "no help
/// register". Metadata (`when_fires`/`why`/`params`/`example`) is never read at render time and is
/// not on this trait.
pub trait CatalogLookup {
    /// The written message template for `slug`, or `None` to render the unwritten placeholder.
    fn message(&self, slug: &str) -> Option<&str>;
    /// The help template for `slug`, or `None` when the code carries no help register.
    fn help(&self, slug: &str) -> Option<&str>;
}

/// The production [`CatalogLookup`]: the compiled-in [`CATALOG`] const. Every production render
/// passes [`CONST_CATALOG`]; promote passes an owned mirror instead (byte-identical renders,
/// gate-pinned).
#[derive(Debug)]
pub struct ConstCatalog;

/// The one production [`CatalogLookup`] value — the compiled-in catalog.
pub const CONST_CATALOG: ConstCatalog = ConstCatalog;

impl CatalogLookup for ConstCatalog {
    fn message(&self, slug: &str) -> Option<&str> {
        entry(slug).and_then(|e| e.message)
    }
    fn help(&self, slug: &str) -> Option<&str> {
        entry(slug).and_then(|e| e.help)
    }
}

/// An owned catalog entry — the promote-time MUTABLE mirror's element (`283:dec-mirror-via-catalog-
/// lookup`). The compiled-in [`CatalogEntry`] holds `&'static str`, so it cannot carry runtime prose
/// an author just edited; this owned twin can. `params`/`example` are NOT stored — [`serialize`]
/// regenerates them from the prose's holes (same as the const codegen). `message: None` is the
/// unwritten state (`283:dec-message-becomes-option`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OwnedEntry {
    /// The stable slug (matches [`crate::diag::DiagCode::slug`]).
    pub slug: String,
    /// When this diagnostic fires (machine-facing metadata).
    pub when_fires: String,
    /// Why the code exists (machine-facing metadata).
    pub why: String,
    /// The primary message template, or `None` when unwritten.
    pub message: Option<String>,
    /// The help register template, or `None` when the code carries no help.
    pub help: Option<String>,
    /// Template holes in first-use order across message then help.
    pub params: Vec<String>,
}

/// The compiled-in catalog as an owned, mutable mirror (`283:dec-mirror-via-catalog-lookup`) — the
/// starting state promote edits before re-serializing. Carry-forward is by construction: an entry
/// whose prose is not touched serializes back verbatim.
#[must_use]
pub fn owned_catalog() -> Vec<OwnedEntry> {
    CATALOG
        .iter()
        .map(|e| OwnedEntry {
            slug: e.slug.to_owned(),
            when_fires: e.when_fires.to_owned(),
            why: e.why.to_owned(),
            message: e.message.map(str::to_owned),
            help: e.help.map(str::to_owned),
            params: e.params.iter().map(|param| (*param).to_owned()).collect(),
        })
        .collect()
}

impl CatalogLookup for Vec<OwnedEntry> {
    fn message(&self, slug: &str) -> Option<&str> {
        self.iter()
            .find(|e| e.slug == slug)
            .and_then(|e| e.message.as_deref())
    }
    fn help(&self, slug: &str) -> Option<&str> {
        self.iter()
            .find(|e| e.slug == slug)
            .and_then(|e| e.help.as_deref())
    }
}

/// Why a catalog template cannot be rendered.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TemplateRefusal {
    /// The template contains malformed double braces.
    Malformed,
    /// The template names a parameter the payload does not provide.
    UnknownParam(String),
}

/// Fill a message template's `{{name}}` holes from `params` (name → value). The named-params-only
/// render primitive (`27V` §3 · `AID-NEEDS:law-trust-tier`): prose never hand-writes values; the
/// engine substitutes them here. Single braces are literal; malformed holes and missing params
/// refuse. Pure.
///
/// # Errors
/// Returns [`TemplateRefusal`] for invalid template syntax or an unknown parameter.
pub fn fill_template(template: &str, params: &[(&str, &str)]) -> Result<String, TemplateRefusal> {
    let mut out = String::with_capacity(template.len());
    for part in parse_template(template)? {
        match part {
            TemplatePart::Literal(text) => out.push_str(&text),
            TemplatePart::Hole(name) => {
                if let Some((_, v)) = params.iter().find(|(k, _)| *k == name) {
                    out.push_str(v);
                } else {
                    return Err(TemplateRefusal::UnknownParam(name));
                }
            }
        }
    }
    Ok(out)
}

/// Fill a template into ordered render parts without dropping empty parameter values.
///
/// # Errors
/// Returns [`TemplateRefusal`] for invalid template syntax or an unknown parameter.
pub fn fill_template_parts(
    template: &str,
    params: &[(&'static str, &str)],
    code: &'static str,
    field: crate::tagged::Field,
    instance: usize,
) -> Result<crate::tagged::RenderParts, TemplateRefusal> {
    use crate::tagged::{RenderPart, RenderParts};

    let mut parts = RenderParts::new();
    for part in parse_template(template)? {
        match part {
            TemplatePart::Literal(text) if !text.is_empty() => {
                parts.push(RenderPart::TemplateLiteral {
                    text,
                    code,
                    field,
                    paragraph: 0,
                    instance,
                });
            }
            TemplatePart::Literal(_) => {}
            TemplatePart::Hole(name) => {
                let Some(&(param, value)) = params.iter().find(|(key, _)| *key == name) else {
                    return Err(TemplateRefusal::UnknownParam(name));
                };
                let text = String::from(value);
                if is_foreign_param(param) {
                    parts.push(RenderPart::ForeignText { text, param });
                } else {
                    parts.push(RenderPart::ParamValue {
                        text,
                        code,
                        field,
                        param,
                        instance,
                    });
                }
            }
        }
    }
    Ok(parts)
}

/// Whether a declared param carries passthrough foreign text (`282:rul-passthrough-type-gated`).
/// Keyed conservatively on the `detail`
/// passthrough convention ([`crate::diag::params_of`] yields `detail` for every PASSTHROUGH code);
/// the type-gated user-sourced distinction is the `282` §8 de-passthrough work, LATER.
#[must_use]
pub fn is_foreign_param(param: &str) -> bool {
    param == "detail"
}

/// One run in a parsed catalog template.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TemplatePart {
    /// Literal text, including any single braces.
    Literal(String),
    /// A named `{{name}}` substitution hole.
    Hole(String),
}

/// Parse a strict catalog template into ordered literal and substitution runs.
///
/// # Errors
/// Returns [`TemplateRefusal::Malformed`] for invalid double-brace syntax.
pub fn parse_template(template: &str) -> Result<Vec<TemplatePart>, TemplateRefusal> {
    let mut parts = Vec::new();
    let mut literal = String::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                if !literal.is_empty() {
                    parts.push(TemplatePart::Literal(std::mem::take(&mut literal)));
                }
                let Some(first) = chars.next() else {
                    return Err(TemplateRefusal::Malformed);
                };
                if !first.is_ascii_alphabetic() && first != '_' {
                    return Err(TemplateRefusal::Malformed);
                }
                let mut name = String::from(first);
                loop {
                    match chars.next() {
                        Some('}') if chars.next() == Some('}') => break,
                        Some(next) if next.is_ascii_alphanumeric() || next == '_' => {
                            name.push(next);
                        }
                        _ => return Err(TemplateRefusal::Malformed),
                    }
                }
                parts.push(TemplatePart::Hole(name));
            }
            '}' if chars.peek() == Some(&'}') => return Err(TemplateRefusal::Malformed),
            _ => literal.push(c),
        }
    }
    if !literal.is_empty() {
        parts.push(TemplatePart::Literal(literal));
    }
    Ok(parts)
}

/// Collect a template's `{{name}}` holes — the gate-test primitive
/// (`holes ⊆ declared params`) AND the [`refreshed_params`] source. Order-preserving, NOT deduped
/// (a hole used twice appears twice); callers that need a param SET dedup. Pure.
fn template_holes(template: &str) -> Result<Vec<String>, TemplateRefusal> {
    Ok(parse_template(template)?
        .into_iter()
        .filter_map(|part| match part {
            TemplatePart::Literal(_) => None,
            TemplatePart::Hole(name) => Some(name),
        })
        .collect())
}

// ===========================================================================
// The catalog-lock serializer (`28A` §4 checkpoint — the handwritten serializer
// stays here; case-first field sourcing is the dorc-loom generator's, `282` §8)
// ===========================================================================

/// The refreshed param SET for a prose pair — the first-occurrence-ordered, deduped union of the
/// holes in the `message` and `help` templates: `params` is EXACTLY the holes the prose uses. An
/// unwritten (`None`) message contributes no holes.
#[must_use]
pub fn refreshed_params(message: Option<&str>, help: Option<&str>) -> Vec<String> {
    let mut params: Vec<String> = Vec::new();
    for template in message.into_iter().chain(help) {
        if let Ok(holes) = template_holes(template) {
            for hole in holes {
                if !params.contains(&hole) {
                    params.push(hole);
                }
            }
        }
    }
    params
}

/// One fully-sourced generated catalog row (`28A` §4 checkpoint). The case-first fields —
/// `when_fires`/`why` from the defining-case frontmatter, `example` from the compiled message
/// rendered with the defining payload — are computed by the dorc-loom generator; core owns only the
/// serializer.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LockRow {
    /// The stable slug.
    pub slug: String,
    /// When the diagnostic fires.
    pub when_fires: String,
    /// Why the code exists.
    pub why: String,
    /// Template holes in first-use order across message then help.
    pub params: Vec<String>,
    /// One concrete example render.
    pub example: String,
    /// The primary message template, or `None` when unwritten.
    pub message: Option<String>,
    /// The help register template, or `None` when the code carries no help.
    pub help: Option<String>,
}

/// Serialize the wholly-generated `catalog_lock.rs` from ordered [`LockRow`]s
/// (`282:rul-catalog-lock-is-generated-whole`). The whole file is generator-owned and overwritten by
/// promotion. `#[rustfmt::skip]` keeps the single-line string emission byte-stable under `cargo fmt`,
/// so the generator output IS the committed bytes (the byte-identity fixpoint gate). Strings emit via
/// `{:?}` (valid Rust escaping); `inv-no-unsafe` stands (codegen-to-source, never a macro).
#[must_use]
pub fn serialize_lock(rows: &[LockRow]) -> String {
    use std::fmt::Write as _;
    let mut out = String::from(
        "// @generated by dorc-loom; DO NOT EDIT.\n\
         // This whole file is overwritten by catalog promotion.\n\n\
         use super::CatalogEntry;\n\n\
         #[rustfmt::skip]\n\
         pub const CATALOG: &[CatalogEntry] = &[\n",
    );
    for r in rows {
        out.push_str("    CatalogEntry {\n");
        let _ = writeln!(out, "        slug: {:?},", r.slug);
        let _ = writeln!(out, "        when_fires: {:?},", r.when_fires);
        let _ = writeln!(out, "        why: {:?},", r.why);
        out.push_str("        params: &[");
        for (i, p) in r.params.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            let _ = write!(out, "{p:?}");
        }
        out.push_str("],\n");
        let _ = writeln!(out, "        example: {:?},", r.example);
        match &r.message {
            Some(m) => {
                let _ = writeln!(out, "        message: Some({m:?}),");
            }
            None => out.push_str("        message: None,\n"),
        }
        match &r.help {
            Some(h) => {
                let _ = writeln!(out, "        help: Some({h:?}),");
            }
            None => out.push_str("        help: None,\n"),
        }
        out.push_str("    },\n");
    }
    out.push_str("];\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_template_substitutes_and_refuses_invalid_holes() {
        assert_eq!(
            fill_template(
                "sm site runs `{{source_excerpt}}`",
                &[("source_excerpt", "make install")]
            ),
            Ok(String::from("sm site runs `make install`"))
        );
        assert_eq!(
            fill_template("a {literal} brace", &[]),
            Ok(String::from("a {literal} brace"))
        );
        assert_eq!(
            fill_template("[unwritten: dq-foo]", &[]),
            Ok(String::from("[unwritten: dq-foo]"))
        );
        assert_eq!(
            fill_template("hi {{absent}}", &[]),
            Err(TemplateRefusal::UnknownParam(String::from("absent"))),
        );
        for template in ["{{", "{{name}", "{{ name}}", "{{name!}}", "{{a{{b}}", "}}"] {
            assert_eq!(
                fill_template(template, &[]),
                Err(TemplateRefusal::Malformed),
                "{template:?}",
            );
        }
    }

    /// Gate: no two catalog entries share a slug (each code has AT MOST one entry — the
    /// exactly-one completeness direction lands with the phase-2 sweep).
    #[test]
    fn no_duplicate_slugs() {
        let mut seen = std::collections::BTreeSet::new();
        for e in CATALOG {
            assert!(seen.insert(e.slug), "duplicate catalog slug `{}`", e.slug);
        }
    }

    #[test]
    fn generated_lock_owns_the_complete_catalog_table() {
        let lock = include_str!("catalog_lock.rs");
        assert!(lock.starts_with("// @generated by dorc-loom; DO NOT EDIT.\n"));
        assert!(lock.contains("pub const CATALOG: &[CatalogEntry] = &["));
        assert_eq!(
            lock.matches("\n    CatalogEntry {\n").count(),
            CATALOG.len()
        );
        assert_eq!(
            include_str!("catalog.rs")
                .matches("\n    CatalogEntry {\n")
                .count(),
            0
        );
    }

    /// Gate (`amendment-catalog-fields-are-data`): every `{hole}` in a message/help template is a
    /// declared param — templates can only interpolate the named params the payload supplies.
    #[test]
    fn template_holes_are_declared_params() {
        for e in CATALOG {
            for template in e.message.into_iter().chain(e.help) {
                for hole in template_holes(template).expect("catalog template syntax") {
                    assert!(
                        e.params.contains(&hole.as_str()),
                        "catalog `{}`: template hole `{{{hole}}}` is not a declared param {:?}",
                        e.slug,
                        e.params
                    );
                }
            }
        }
    }

    /// Gate: the conductor/machine-facing metadata fields are non-empty (a code with no
    /// when/why/example is under-documented — the fields exist to be consumed, not blank).
    #[test]
    fn required_metadata_is_non_empty() {
        for e in CATALOG {
            assert!(!e.slug.is_empty(), "empty slug");
            assert!(!e.when_fires.is_empty(), "`{}`: empty when_fires", e.slug);
            assert!(!e.why.is_empty(), "`{}`: empty why", e.slug);
            assert!(!e.example.is_empty(), "`{}`: empty example", e.slug);
            assert!(
                e.message != Some(""),
                "`{}`: empty message — unwritten is None, not \"\"",
                e.slug
            );
        }
    }

    /// Whether `slug`'s prose is CASE-OWNED: a defining case file exists for it in the dorc-loom
    /// corpus (`283` flip / `28A` §2o). This is where prose ownership moved when the
    /// `CONDUCTOR_AUTHORED` roster retired — a case-owned code's unprefixed prose is protected by the
    /// render-level `fixpoint_check` (a catalog hand-edit moves the render off the committed case
    /// bytes), so the roster's two-place bookkeeping is no longer needed.
    fn is_case_owned(slug: &str) -> bool {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(|crates| crates.join("dorc-loom/cases").join(format!("{slug}.loom")))
            .is_some_and(|case| case.exists())
    }

    /// Gate (`amendment-prose-boundary`, re-keyed at the `283` flip): every WRITTEN user-facing
    /// register is `sm `-prefixed base-tip prose or CASE-OWNED (a defining case in the dorc-loom
    /// corpus, fixpoint-protected) — the mechanical enforcement that builders author no new
    /// user-facing prose (`27V:rul-error-authorship-tier`). Unwritten is `None`
    /// (`283:dec-message-becomes-option`), so a stored `[unwritten:]` string is not legal either.
    #[test]
    fn message_registers_are_sm_or_unwritten() {
        for e in CATALOG {
            for (field, text) in [("message", e.message), ("help", e.help)] {
                let Some(text) = text else { continue };
                assert!(
                    text.starts_with("sm ") || is_case_owned(e.slug),
                    "catalog `{}` {field}: a written register must be `sm `-prefixed base-tip prose \
                     or case-owned (a dorc-loom corpus case; unwritten prose is `None`), got: {text:?}",
                    e.slug
                );
            }
        }
    }

    /// Sample slugs resolve through [`entry`] and are known `DiagCode` wire tokens (catalog ⊆
    /// enum, one direction; the reverse completeness direction is the tidy-gate bijection).
    #[test]
    fn sample_slugs_resolve_and_are_real_codes() {
        for slug in ["site-unresolvable", "render-heredoc-refused"] {
            assert!(entry(slug).is_some(), "slug `{slug}` resolves");
        }
        // Cross-check against the enum's own wire tokens (constructed instances name their slug).
        assert_eq!(
            crate::diag::DiagCode::RenderHeredocRefused(crate::diag::RenderHeredocRefused {
                site: crate::diag::SiteId::leaf(crate::LeafId(0)),
                verb: "elide",
                command: "cat <<EOF".to_owned(),
            })
            .slug(),
            "render-heredoc-refused"
        );
    }

    /// Every committed `params` list equals the deduped first-use holes of its prose
    /// (`refreshed_params`): a `params` hand-edit diverges from the regeneration and trips here. This
    /// is the `params` half of the generated-lock byte-identity gate the dorc-loom generator owns.
    #[test]
    fn promote_regenerates_params_byte_identical() {
        for e in CATALOG {
            let refreshed = refreshed_params(e.message, e.help);
            let refreshed: Vec<&str> = refreshed.iter().map(String::as_str).collect();
            assert_eq!(
                refreshed, e.params,
                "catalog `{}`: committed params diverge from the prose's holes — a metadata \
                 hand-edit (serialize regenerates params from the message/help holes)",
                e.slug
            );
        }
    }

    /// A param used twice (`{count}` in `munge-name-collision`) appears once in the refreshed set.
    #[test]
    fn refreshed_params_dedups_repeated_holes() {
        let coll = entry("munge-name-collision").expect("collision entry");
        let cp = refreshed_params(coll.message, coll.help);
        assert_eq!(
            cp.iter().filter(|p| *p == "count").count(),
            1,
            "a param used twice appears once in the refreshed set: {cp:?}"
        );
    }

    /// The serializer emits a `#[rustfmt::skip]` generated const with the pinned header, so
    /// single-line string emission is `cargo fmt`-stable (the byte-identity fixpoint precondition).
    #[test]
    fn serialize_lock_emits_the_pinned_generated_header() {
        let src = serialize_lock(&[LockRow {
            slug: "x".to_owned(),
            when_fires: "w".to_owned(),
            why: "y".to_owned(),
            params: vec!["a".to_owned()],
            example: "e".to_owned(),
            message: Some("m {{a}}".to_owned()),
            help: None,
        }]);
        assert!(src.starts_with("// @generated by dorc-loom; DO NOT EDIT.\n"));
        assert!(src.contains("#[rustfmt::skip]\npub const CATALOG: &[CatalogEntry] = &[\n"));
        assert!(src.contains("        message: Some(\"m {{a}}\"),\n"));
        assert!(src.contains("        help: None,\n"));
        assert!(src.trim_end().ends_with("];"));
    }
}
