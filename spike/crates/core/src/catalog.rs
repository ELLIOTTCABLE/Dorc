//! The ONE committed diagnostic catalog — the single home for user-facing diagnostic prose
//! (`27V:rul-kill-legacy-diagnostic` · `AID-NEEDS:defining-case-catalog`). Message text lives
//! HERE as data, keyed by code slug; render arms pull templates from this table and fill the
//! named params from a [`crate::diag::Diag`]'s typed payload. Nothing else authors user-facing
//! prose.
//!
//! # Shape (conductor-ruled `amendment-catalog-fields-are-data`, 2026-07-18)
//!
//! A plain Rust `const` table of [`CatalogEntry`] struct literals — the compiler is the parser
//! (no `build.rs`, no hand-rolled format, no proc-macro: `inv-no-unsafe` stands). Per-entry
//! metadata (`when_fires` / `why` / `params` / `example`) is STRUCTURED DATA the gate tests
//! check, never a comment block. Editing prose is editing a raw string literal in place, and one
//! final compile takes effect — no per-edit codegen step (`amendment-single-bless-confirmed`).
//! The d4 promote pipeline later becomes codegen-to-this-source, staying diffable and committed.
//!
//! # The two legal states of a `message` (mechanically gated)
//!
//! Every [`CatalogEntry::message`] (and [`CatalogEntry::help`]) is EITHER:
//! * `sm `-prefixed prior-builder prose migrated VERBATIM from the base tip (`380f2fa`) — the
//!   `sm ` marker means "builder prose awaiting human rewrite" (`27V:rul-error-authorship-tier`,
//!   sharpened by `amendment-prose-boundary`); OR
//! * the exact placeholder `[unwritten: <slug>]` for any user-facing string that did NOT exist at
//!   the base tip (a new or split code) — builders author ZERO new user-facing prose.
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
    /// The user-facing PRIMARY message template — `sm `-prefixed base-tip prose, or the exact
    /// `[unwritten: <slug>]` placeholder. `{name}` holes are filled from the payload; `{{`/`}}`
    /// escape literal braces.
    pub message: &'static str,
    /// The optional user-facing remediation/help register — same two legal states as
    /// [`message`](Self::message), or `None` when the code carries no help.
    pub help: Option<&'static str>,
}

/// The committed catalog table (`amendment-catalog-fields-are-data`). Order is stable/deterministic
/// (source order; `inv-determinism`). SEED SCAFFOLDING (dispatch d1 phase 1): a representative pair
/// proving the shape end-to-end — the phase-2 sweep populates one entry per migrated `DiagCode`
/// variant and the completeness gate (every variant ⇒ exactly one entry) lands with it.
pub const CATALOG: &[CatalogEntry] = &[
    CatalogEntry {
        slug: "dq-site-unresolvable",
        when_fires: "a probe could not ship a read-only check for a command-site, so the apply \
                     runs it.",
        why: "spike/CLAUDE.md two-phases-opposite-fail-directions (kFAIL-perform): unsure ⇒ run; \
              the disclosure names the site that will run every apply.",
        params: &["source_excerpt"],
        example: "sm site runs `make install`",
        message: "sm site runs `{source_excerpt}`",
        help: None,
    },
    CatalogEntry {
        slug: "render-heredoc-refused",
        when_fires: "the leaf-exact render would elide/guard a licensed leaf whose span covers a \
                     `<<` heredoc opener (not its body), so the leaf runs verbatim instead.",
        why: "spike/CLAUDE.md inv-kfail (kFAIL-perform, arch-1 d-6): substituting the opener span \
              would strand the heredoc body — an Error-class give-up (a broken artifact otherwise).",
        params: &["verb", "command"],
        example: "sm leaf-exact render refuses to elide a heredoc-bearing command (`cat <<EOF`): \
                  its span covers the `<<` operator, not the body lines, so substituting it would \
                  strand the heredoc body — it runs verbatim",
        message: "sm leaf-exact render refuses to {verb} a heredoc-bearing command (`{command}`): \
                  its span covers the `<<` operator, not the body lines, so substituting it would \
                  strand the heredoc body — it runs verbatim",
        help: Some("sm split the heredoc body to its own leaf, or mark the kind un-elidable"),
    },
];

/// The catalog entry for `slug`, or `None` when the slug has no entry (dead code path pre-sweep;
/// the phase-2 completeness gate makes a missing entry a test failure once every variant is
/// populated). Linear scan — the table is small and analysis-side big-O never constrains
/// (`spike/CLAUDE.md perf-doctrine`).
#[must_use]
pub fn entry(slug: &str) -> Option<&'static CatalogEntry> {
    CATALOG.iter().find(|e| e.slug == slug)
}

/// Fill a message template's `{name}` holes from `params` (name → value), leaving `{{`/`}}` as the
/// literal `{`/`}`. The named-params-only render primitive (`27V` §3 · `AID-NEEDS:law-trust-tier`):
/// prose never hand-writes values; the engine substitutes them here. An unknown `{name}` (not in
/// `params`) renders as the literal `{name}` — the gate `template_holes_are_declared_params` makes
/// that unreachable for committed entries, so this is only a defensive fallback (`inv-no-throw`:
/// returns data, never panics). Pure.
#[must_use]
pub fn fill_template(template: &str, params: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                out.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                out.push('}');
            }
            '{' => {
                let mut name = String::new();
                for nc in chars.by_ref() {
                    if nc == '}' {
                        break;
                    }
                    name.push(nc);
                }
                if let Some((_, v)) = params.iter().find(|(k, _)| *k == name) {
                    out.push_str(v);
                } else {
                    out.push('{');
                    out.push_str(&name);
                    out.push('}');
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// Collect a template's `{name}` holes (skipping `{{`/`}}` escapes) — the gate-test primitive that
/// enforces `holes ⊆ declared params`. Pure; order-preserving.
#[cfg(test)]
#[must_use]
fn template_holes(template: &str) -> Vec<String> {
    let mut holes = Vec::new();
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
            }
            '{' => {
                let mut name = String::new();
                for nc in chars.by_ref() {
                    if nc == '}' {
                        break;
                    }
                    name.push(nc);
                }
                holes.push(name);
            }
            _ => {}
        }
    }
    holes
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `fill_template` substitutes declared holes and passes brace-escapes / unknown-hole /
    /// `[unwritten:]` text through faithfully (`inv-no-throw`: never panics).
    #[test]
    fn fill_template_substitutes_and_escapes() {
        assert_eq!(
            fill_template(
                "sm site runs `{source_excerpt}`",
                &[("source_excerpt", "make install")]
            ),
            "sm site runs `make install`"
        );
        assert_eq!(
            fill_template("a {{literal}} brace", &[]),
            "a {literal} brace"
        );
        // An [unwritten:] placeholder has no holes ⇒ renders greppably verbatim.
        assert_eq!(
            fill_template("[unwritten: dq-foo]", &[]),
            "[unwritten: dq-foo]"
        );
        // An unknown hole is left literal (defensive; the gate forbids it for committed entries).
        assert_eq!(fill_template("hi {absent}", &[]), "hi {absent}");
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

    /// Gate (`amendment-catalog-fields-are-data`): every `{hole}` in a message/help template is a
    /// declared param — templates can only interpolate the named params the payload supplies.
    #[test]
    fn template_holes_are_declared_params() {
        for e in CATALOG {
            for template in std::iter::once(e.message).chain(e.help) {
                for hole in template_holes(template) {
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
            assert!(!e.message.is_empty(), "`{}`: empty message", e.slug);
        }
    }

    /// Gate (`amendment-prose-boundary`): every user-facing register is EITHER `sm `-prefixed
    /// base-tip prose OR the exact `[unwritten: <slug>]` placeholder — the mechanical enforcement
    /// that builders author no new user-facing prose (`27V:rul-error-authorship-tier`).
    #[test]
    fn message_registers_are_sm_or_unwritten() {
        for e in CATALOG {
            let unwritten = format!("[unwritten: {}]", e.slug);
            for (field, text) in [("message", Some(e.message)), ("help", e.help)] {
                let Some(text) = text else { continue };
                assert!(
                    text.starts_with("sm ") || text == unwritten,
                    "catalog `{}` {field}: user-facing text must be `sm `-prefixed base-tip prose \
                     or the exact `{unwritten}` placeholder, got: {text:?}",
                    e.slug
                );
            }
        }
    }

    /// The seeded slugs resolve through [`entry`] and are known `DiagCode` wire tokens (catalog ⊆
    /// enum, one direction; the reverse completeness direction is the phase-2 gate).
    #[test]
    fn seeded_slugs_resolve_and_are_real_codes() {
        for slug in ["dq-site-unresolvable", "render-heredoc-refused"] {
            assert!(entry(slug).is_some(), "seed slug `{slug}` resolves");
        }
        // Cross-check against the enum's own wire tokens (constructed instances name their slug).
        assert_eq!(
            crate::diag::DiagCode::RenderHeredocRefused(crate::diag::RenderHeredocRefused {
                site: crate::diag::SiteId::leaf(crate::LeafId(0)),
            })
            .slug(),
            "render-heredoc-refused"
        );
    }
}
