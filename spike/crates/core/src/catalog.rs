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

/// The committed catalog table (`amendment-catalog-fields-are-data`). Order is stable/deterministic
/// (source order; `inv-determinism`). ONE entry per `DiagCode` variant (the completeness gate —
/// `core/tests/diag_tidy.rs::every_variant_has_exactly_one_catalog_entry` — pins the bijection).
/// PASSTHROUGH codes carry the message on a runtime `detail` (template `sm {detail}`); TEMPLATIZED
/// codes carry a real `sm <template>` filled from named payload params. All user-facing prose is
/// `sm `-prefixed base-tip prose awaiting a human rewrite (`27V:rul-error-authorship-tier`).
pub const CATALOG: &[CatalogEntry] = &[
    // ── round-22 spine + former legacy survivors ────────────────────────────
    CatalogEntry {
        slug: "cmdsub-operand-top",
        when_fires: "a `$(…)`/runtime-dynamic operand (or the command word) forced a command to ⊤, \
                     so it runs (never elided). effect.rs finalize_cmdsub_tops.",
        why: "no-silent-phantoms disclosure (find-3); the template fills `{position}` from \
              `OperandPosition::describe()` and `{cause}` from `TopCause::describe()`. \
              NOTE plain-language pass owed: describe()-interpolated + ⊤/top wording \
              (law-plain-language-surfaces, 24H ack-4).",
        params: &["position", "cause"],
        example: "sm command forced to run (never elided): operand 1 is a command-substitution \
                  `$(…)` / arithmetic / operator-form expansion ⇒ its identity is unresolved (⊤)",
        message: Some(
            "sm command forced to run (never elided): {position} is {cause} ⇒ its identity \
                  is unresolved (⊤)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "site-unresolvable",
        when_fires: "a probe could not ship a read-only check for one or more command-sites, so \
                     the apply runs each. cli unresolvable_diagnostics (né dq-site-unresolvable).",
        why: "kFAIL-perform: unsure ⇒ run. PASSTHROUGH — `detail` reproduces BOTH the aggregate \
              label (N sites …) AND the folded `\\n  = note: site runs `{excerpt}`` line the old \
              render_body emitted, so the migrated output is byte-identical bar the `sm ` prefix.",
        params: &["detail"],
        example: "sm 2 sites run unprobed (no read-only check could be shipped): `make install`, \
                  `ldconfig` — run `dorc why` for the per-site detail (the apply runs each anyway, \
                  to stay safe)",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "render-heredoc-refused",
        when_fires: "the leaf-exact render would elide/guard a licensed leaf whose span covers a \
                     `<<` heredoc opener (not its body), so the leaf runs verbatim instead.",
        why: "kFAIL-perform, arch-1 d-6: substituting the opener span would strand the heredoc \
              body — an Error-class give-up (a broken artifact otherwise). `{verb}` = elide/guard.",
        params: &["verb", "command"],
        example: "sm leaf-exact render refuses to elide a heredoc-bearing command (`cat <<EOF`): \
                  its span covers the `<<` operator, not the body lines, so substituting it would \
                  strand the heredoc body — it runs verbatim",
        message: Some(
            "sm leaf-exact render refuses to {verb} a heredoc-bearing command (`{command}`): \
                  its span covers the `<<` operator, not the body lines, so substituting it would \
                  strand the heredoc body — it runs verbatim",
        ),
        help: Some("sm split the heredoc body to its own leaf, or mark the kind un-elidable"),
    },
    CatalogEntry {
        slug: "cmdsub-inner-nonleaf",
        when_fires: "an effect-bearing command runs inside a `$(…)` substitution body, so it has \
                     no independent leaf. effect.rs classify (né dq-cmdsub-inner-nonleaf).",
        why: "q-1.f silent-1/silent-4 disclosure; the inner command runs whenever its enclosing \
              line runs. `{inner}` = the resolved inner argv.",
        params: &["inner"],
        example: "sm command `id -u` runs inside a `$(…)` substitution ⇒ effect-bearing but not \
                  independently elidable (it runs whenever its enclosing line runs)",
        message: Some(
            "sm command `{inner}` runs inside a `$(…)` substitution ⇒ effect-bearing but not \
                  independently elidable (it runs whenever its enclosing line runs)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "redir-target-top",
        when_fires: "a write-redirect (`>`/`>>`) to a dynamic/unresolved target joins ⊤. \
                     effect.rs classify Redir arm (né dq-redir-target-top).",
        why: "y-1 / 21F imp-1: the target is unresolvable so no per-path `file` cell can be keyed. \
              NOTE plain-language pass owed: ⊤/top wording (law-plain-language-surfaces, 24H ack-4).",
        params: &[],
        example: "sm write-redirect to a dynamic/unresolved target ⇒ no per-path `file` cell can \
                  be keyed, so the write joins ⊤ and the command runs (never elided)",
        message: Some(
            "sm write-redirect to a dynamic/unresolved target ⇒ no per-path `file` cell can \
                  be keyed, so the write joins ⊤ and the command runs (never elided)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "depth-2-positional-unthreaded",
        when_fires: "a depth-2 inlined call's argument references a positional that does not thread \
                     two inline levels. cfg.rs (né dq-depth-2-positional-unthreaded).",
        why: "216 §1.2 correction: the inner body's positional resolves ⊤, so the call runs \
              verbatim. `{name}` = the refused call's function name. NOTE plain-language pass \
              owed: ⊤/top wording (law-plain-language-surfaces, 24H ack-4).",
        params: &["name"],
        example: "sm call `deploy` not inlined: its argument references a positional \
                  (`$1`..`$9`/`$#`) that does not thread through two inline levels ⇒ the inner \
                  body's positional is ⊤ — it runs as an ordinary unmodeled command (depth-2 \
                  positional threading is out of the modeled subset)",
        message: Some(
            "sm call `{name}` not inlined: its argument references a positional \
                  (`$1`..`$9`/`$#`) that does not thread through two inline levels ⇒ the inner \
                  body's positional is ⊤ — it runs as an ordinary unmodeled command (depth-2 \
                  positional threading is out of the modeled subset)",
        ),
        help: None,
    },
    // ── syntax/parser.rs (PASSTHROUGH) ──────────────────────────────────────
    CatalogEntry {
        slug: "syntax-unsupported",
        when_fires: "the parser hit an unmodeled/out-of-scope sh construct; it becomes an \
                     `Unsupported` ⊤-node and parsing continues.",
        why: "inv-top-reject: under-modeling is a loud correctness boundary. PASSTHROUGH — the \
              parser's own description of the construct rides `detail`.",
        params: &["detail"],
        example: "sm process substitution `<(…)` is not modeled",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "syntax-malformed",
        when_fires: "the parser hit a structurally malformed sh construct (a parse error); parsing \
                     continues fail-soft.",
        why: "inv-no-throw: errors are data. PASSTHROUGH — the parser's description rides `detail`.",
        params: &["detail"],
        example: "sm unterminated double-quote",
        message: Some("sm {detail}"),
        help: None,
    },
    // ── analysis/cfg.rs (PASSTHROUGH) ───────────────────────────────────────
    CatalogEntry {
        slug: "cfg-top-node",
        when_fires: "an `Unsupported` AST ⊤-node became a CFG `Top` node (an unsupported construct, \
                     or the CFG nesting bound). cfg.rs lower_top + fresh(Top).",
        why: "the conservative ⊤-absorbing semantics; any command after it may mutate anything. \
              PASSTHROUGH — the CFG builder's reason rides `detail`. NOTE plain-language pass owed: \
              slug carries top/⊤ wording (law-plain-language-surfaces, 24H ack-4).",
        params: &["detail"],
        example: "sm unsupported construct (⊤): un-probeable and un-skippable",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "cfg-errexit-unknown",
        when_fires: "the errexit-region pass hit an unknown/unmodeled command; the `set -e` \
                     failure-edge is conservatively assumed. cfg.rs (SPANLESS — spans a region).",
        why: "over-approximate, sound. PASSTHROUGH — the pass's description rides `detail`.",
        params: &["detail"],
        example: "sm errexit state is ⊤ at one or more commands; failure-edges added conservatively \
                  (over-approximate, sound)",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "cfg-inline-refused",
        when_fires: "a function call could not be inlined. cfg.rs — SEVEN distinct emit paths under \
                     one slug: redefinition, recursion, inline-depth budget, unmodeled positional \
                     in body, unmodeled write-redirect in body (tc-M2), per-call node budget, \
                     per-book node budget. Each names its own reason.",
        why: "the call runs as an ordinary unmodeled command (MustRun, safe). PASSTHROUGH — the \
              per-path reason rides `detail`.",
        params: &["detail"],
        example: "sm call to `helper` exceeds the inline-depth budget (8); not inlined — it runs \
                  as an ordinary unmodeled command",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "cfg-builtin-shadowed",
        when_fires: "a book funcdef shadows a shell builtin the engine relies on (dash resolves a \
                     function before a regular builtin). cfg.rs warn_shadowed_relied_builtins.",
        why: "find-I: builtin-dependent conclusions may be unsound for this book. PASSTHROUGH — the \
              disclosure text rides `detail`; the primary span is the funcdef name_span.",
        params: &["detail"],
        example: "sm function `test` shadows a shell builtin the engine relies on (dash resolves a \
                  function before a regular builtin): analysis treats the bare word `test` as the \
                  builtin when classifying effects and minting stand-ins, so builtin-dependent \
                  conclusions may be unsound for this book",
        message: Some("sm {detail}"),
        help: None,
    },
    // ── analysis/effect.rs (PASSTHROUGH) ────────────────────────────────────
    CatalogEntry {
        slug: "effect-kind-disagreement",
        when_fires: "a check's annotation kind disagrees with the effect-map kind for the same \
                     verb; the annotation wins. effect.rs (SPANLESS — mid-resolution, no leaf).",
        why: "204 §6 open seam: declared identity wins. PASSTHROUGH — the disagreement rides \
              `detail`.",
        params: &["detail"],
        example: "sm check annotation kind `sm.dorc.Package` disagrees with the effect-map kind \
                  `sm.dorc.File` for this verb — the annotation (declared identity) wins",
        message: Some("sm {detail}"),
        help: None,
    },
    // ── oracle/predict.rs (PASSTHROUGH) ─────────────────────────────────────
    CatalogEntry {
        slug: "predict-out-of-dialect",
        when_fires: "a check function body uses a construct outside the check dialect (a strict \
                     subset of sh). oracle/predict.rs lift_failure.",
        why: "out-of-dialect input is a lift failure. PASSTHROUGH — the check parser's description \
              rides `detail`.",
        params: &["detail"],
        example: "sm check body uses `[[ … ]]`, outside the check dialect",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "predict-unterminated",
        when_fires: "a check function body is structurally unterminated (missing `;;`/`esac`). \
                     oracle/predict.rs lift_failure.",
        why: "the check cannot be lifted. PASSTHROUGH — the check parser's description rides \
              `detail`.",
        params: &["detail"],
        example: "sm check body ends mid-`case` (missing `esac`)",
        message: Some("sm {detail}"),
        help: None,
    },
    // ── oracle/reserved.rs (TEMPLATIZED) ────────────────────────────────────
    CatalogEntry {
        slug: "munge-name-invalid",
        when_fires: "an emitted `<munged>__<role>` funcname is not a legal sh NAME (leading digit, \
                     dot, non-ASCII). oracle/reserved.rs lint_oracle_reserved_names.",
        why: "ca-munge-charclass (24M §4b): a broken function name cannot ship — REFUSED. \
              `{problem}` is `ShNameProblem::describe()`.",
        params: &["source", "funcname", "problem"],
        example: "sm `9pkg` munges to the sh function name `9pkg`, which is not a legal NAME: \
                  starts with a digit (ca-munge-charclass, 24M §4b) — REFUSED (a broken function \
                  name cannot ship; the munger must transliterate or the name must be renamed)",
        message: Some(
            "sm `{source}` munges to the sh function name `{funcname}`, which is not a legal \
                  NAME: {problem} (ca-munge-charclass, 24M §4b) — REFUSED (a broken function name \
                  cannot ship; the munger must transliterate or the name must be renamed)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "munge-name-collision",
        when_fires: "two DISTINCT source names munge to one sh funcname. oracle/reserved.rs. \
                     `{count}` interpolates twice (the count and the funcdef count).",
        why: "non-injective munge: refuse-and-run (never silently last-writer-wins).",
        params: &["source", "funcname", "count", "names"],
        example: "sm `a.b` munges to the sh function name `a_b`, shared by 2 distinct source names \
                  (a.b, a-b) — REFUSED, never silently merged (the shipped artifact would carry 2 \
                  same-named funcdefs, last-writer-wins; align with the reingest-collision floor: \
                  refuse-and-run)",
        message: Some(
            "sm `{source}` munges to the sh function name `{funcname}`, shared by {count} \
                  distinct source names ({names}) — REFUSED, never silently merged (the shipped \
                  artifact would carry {count} same-named funcdefs, last-writer-wins; align with \
                  the reingest-collision floor: refuse-and-run)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "reserved-namespace-squat",
        when_fires: "a book funcdef coincidentally named `*__<role>` squats the reserved oracle \
                     namespace. oracle/reserved.rs lint_book_reserved_names. `{role}` twice.",
        why: "rul24M-bare-dorcism-names: accepted-not-prevented; the disclosure is loud (warnings \
              tune high this era).",
        params: &["name", "role"],
        example: "sm book function `nginx__predict` squats the reserved `__predict` oracle \
                  namespace (rul24M-bare-dorcism-names): if unintended, it coincidentally matches \
                  an emitted oracle function name — it is treated as an ordinary opaque command \
                  here (run-verbatim), but a shipped oracle preamble of the same name would collide \
                  (last-writer-wins). Rename it to stay clear of `*__predict`.",
        message: Some(
            "sm book function `{name}` squats the reserved `{role}` oracle namespace \
                  (rul24M-bare-dorcism-names): if unintended, it coincidentally matches an emitted \
                  oracle function name — it is treated as an ordinary opaque command here \
                  (run-verbatim), but a shipped oracle preamble of the same name would collide \
                  (last-writer-wins). Rename it to stay clear of `*{role}`.",
        ),
        help: None,
    },
    // ── oracle/marker.rs (static) ───────────────────────────────────────────
    CatalogEntry {
        slug: "missing-dialect-marker",
        when_fires: "a dorc-lang dialect construct (a bind or trailing mark) appears in a file \
                     lacking the `# dorc-lang/v0.2` version marker. oracle/marker.rs.",
        why: "marker-gates-syntax-only: a loud file-level refusal. Static — the marker text is \
              inline (MARKER / MARKER_WINDOW compile-time constants).",
        params: &[],
        example: "sm this file uses a dorc-lang dialect construct (a bind `name : kind = …` or a \
                  trailing `:`/`:!`/`:?` mark) but lacks the `# dorc-lang/v0.2` version marker \
                  (marker-gates-syntax-only): add `# dorc-lang/v0.2` as a standalone comment in the \
                  first 10 lines, or drop the dialect (the bare `__role` floor works markerless)",
        message: Some(
            "sm this file uses a dorc-lang dialect construct (a bind `name : kind = …` or a \
                  trailing `:`/`:!`/`:?` mark) but lacks the `# dorc-lang/v0.2` version marker \
                  (marker-gates-syntax-only): add `# dorc-lang/v0.2` as a standalone comment in the \
                  first 10 lines, or drop the dialect (the bare `__role` floor works markerless)",
        ),
        help: None,
    },
    // ── oracle/marker.rs (version-recognition) ──────────────────────────────
    //    UNWRITTEN prose (`27V:rul-error-authorship-tier`): minted through the empty loop as the
    //    phase-4 pilot (`28A` §2l); the conductor authors the message from the render.
    CatalogEntry {
        slug: "marker-version-unrecognized",
        when_fires: "a dorc-lang dialect construct appears in a file whose `# dorc-lang/vX.Y` version \
                     marker names a version this binary does not recognize (only v0.2 today), distinct \
                     from a wholly-missing marker. oracle/marker.rs.",
        why: "marker-gates-syntax-only + versioned-additive: an unrecognized version is a loud \
              file-level refusal SEPARATE from missing-marker, so a vNEXT/typo'd-version file is not \
              mis-blamed as markerless (`28A` §2l). `{found}` = the version tag read from the marker.",
        params: &[],
        example: "[unwritten: marker-version-unrecognized]",
        message: None,
        help: None,
    },
    // ── oracle/entry.rs (tolerance vouch + corroboration) ───────────────────
    CatalogEntry {
        slug: "tolerates-unknown-dimension",
        when_fires: "an unknown context-dimension token appears on a `tolerates:` vouch. \
                     oracle/entry.rs collect_tolerance. `{expected}` = the known-dimension list.",
        why: "27C §2: the mark vouches nothing and the site stays walled on that dimension \
              (engine-owned closed vocabulary).",
        params: &["token", "expected"],
        example: "sm `netns2` is not a known context dimension on a `tolerates:` vouch (expected \
                  one of user, netns, fs-view); the mark vouches nothing and the site stays walled \
                  on that dimension (`27C` §2).",
        message: Some(
            "sm `{token}` is not a known context dimension on a `tolerates:` vouch (expected \
                  one of {expected}); the mark vouches nothing and the site stays walled on that \
                  dimension (`27C` §2).",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "tolerates-over-identity-dependence",
        when_fires: "a `tolerates:user` vouch sits over a body that visibly reads identity \
                     (`id`/`$USER`/`$HOME`). oracle/entry.rs corroborate_tolerance_over_identity.",
        why: "27C §6 corroboration (recognize-never-license): the ask never blocks. Static.",
        params: &[],
        example: "sm this `is_converged` carries `tolerates:user` but VISIBLY reads the caller's \
                  identity (`id`/`$USER`/`$HOME`): are you sure the body is read-only under a user \
                  shift, not just answer-varying? A shifted user must not make it MUTATE (`27C` §2 \
                  corroboration).",
        message: Some(
            "sm this `is_converged` carries `tolerates:user` but VISIBLY reads the caller's \
                  identity (`id`/`$USER`/`$HOME`): are you sure the body is read-only under a user \
                  shift, not just answer-varying? A shifted user must not make it MUTATE (`27C` §2 \
                  corroboration).",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "heavy-context-no-tolerance",
        when_fires: "a body reads identity but carries no tolerance vouch. oracle/entry.rs \
                     hint_heavy_context_no_vouch (reverse-direction corroboration hint).",
        why: "27C §6 (recognize-never-license): a one-line vouch would make it context-shiftable. \
              Static.",
        params: &[],
        example: "sm this `is_converged` reads the caller's identity but carries no tolerance \
                  vouch — a wrapped site (`sudo …`) will run/guard instead of eliding. One line \
                  makes it context-shiftable: `: safe-across user` (`27C` §2).",
        message: Some(
            "sm this `is_converged` reads the caller's identity but carries no tolerance \
                  vouch — a wrapped site (`sudo …`) will run/guard instead of eliding. One line \
                  makes it context-shiftable: `: safe-across user` (`27C` §2).",
        ),
        help: None,
    },
    // ── oracle/wrapper.rs (TEMPLATIZED) ─────────────────────────────────────
    CatalogEntry {
        slug: "lend-map-unknown-dimension",
        when_fires: "an unknown lend_map dimension token appears on a `__lend_map` line. \
                     oracle/wrapper.rs walk_lend_body. `{expected}` = the known-dimension list.",
        why: "273 §8: the line mints no lend and the dimension it meant to answer walls. NOTE \
              plain-language pass owed: ⊤/top wording (law-plain-language-surfaces, 24H ack-4).",
        params: &["token", "expected"],
        example: "sm `netns2` is not a known lend_map dimension (expected one of user, netns, \
                  fs-view); the line mints no lend and the dimension it meant to answer stays ⊤ \
                  (walls). Dimension marks are an engine-owned closed vocabulary (`273` §8).",
        message: Some(
            "sm `{token}` is not a known lend_map dimension (expected one of {expected}); the \
                  line mints no lend and the dimension it meant to answer stays ⊤ (walls). \
                  Dimension marks are an engine-owned closed vocabulary (`273` §8).",
        ),
        help: None,
    },
    // ── oracle/carry.rs (TEMPLATIZED) ───────────────────────────────────────
    CatalogEntry {
        slug: "carry-netns-on-net-kernel-forbidden",
        when_fires: "a kind's per-netns `net-kernel` store claims `invariant:netns` (a \
                     contradiction). oracle/carry.rs lift. `{kind_munged}` = the munged kind.",
        why: "27C §4(a): network kernel state is namespaced; the false invariance line is dropped.",
        params: &["kind_munged"],
        example: "sm `invariant:netns` is forbidden on the per-netns `net-kernel` store of \
                  `sm_dorc_KernelParam` — network kernel state is namespaced, never netns-invariant",
        message: Some(
            "sm `invariant:netns` is forbidden on the per-netns `net-kernel` store of \
                  `{kind_munged}` — network kernel state is namespaced, never netns-invariant",
        ),
        help: None,
    },
    // ── oracle/predict/derive.rs (static) ───────────────────────────────────
    CatalogEntry {
        slug: "mark-brace-verdict-single-cell",
        when_fires: "a brace-alternation `@{a,b}` appears on a single-cell verdict/observe mark. \
                     oracle/predict/derive.rs. Static (literal braces escaped in the template).",
        why: "277 §4c: a verdict/observe mark asserts exactly one cell; the brace mints no cell \
              and the site runs (a role-aware rejection the parser cannot make).",
        params: &[],
        example: "sm verdict and observe marks are single-cell; brace alternation `@{a,b}` is \
                  claim-emission-only (`277` §4c) — this mark mints NO cell and the site will run. \
                  Split it into one marked probe line per cell.",
        message: Some(
            "sm verdict and observe marks are single-cell; brace alternation `@{{a,b}}` is \
                  claim-emission-only (`277` §4c) — this mark mints NO cell and the site will run. \
                  Split it into one marked probe line per cell.",
        ),
        help: None,
    },
    // ── oracle/predict (the `281` mark-grammar parse — new-grammar path) ─────
    //    UNWRITTEN prose (`27V:rul-error-authorship-tier`): message is the `[unwritten:]`
    //    placeholder; conductor authors it after the `282` flip. `example` is a strawman, not prose.
    CatalogEntry {
        slug: "mark-unknown-verb",
        when_fires: "the new-grammar mark parser hit a period-free head/continuation token that is \
                     not a known verb (`281` §4 rule-3 miss). oracle/predict/mark_grammar.rs.",
        why: "281 §4 keystone (rul-verbs-dotless-kinds-dotted): a dotless mark token is a verb; an \
              unknown one is malformed committed syntax ⇒ the block drops to ⊤ (`inv-top-reject`). \
              `{token}` = the bad token, `{expected}` = the known-verb vocabulary.",
        params: &[],
        example: "an oracle mark `: frobnicate sm.dorc.X` names no known verb; the block drops to ⊤",
        message: None,
        help: None,
    },
    CatalogEntry {
        slug: "mark-rc-arity-exceeded",
        when_fires: "the new-grammar mark parser found two rc-consuming marks (`asserts`/`refutes`) \
                     in one block, incl. continuations (`281` §7). oracle/predict/mark_grammar.rs.",
        why: "281 §7 rc-arity: one exit code witnesses one cell, so two verdicts on one block is \
              unmeasurable ⇒ the block drops to ⊤ (`inv-top-reject`).",
        params: &[],
        example: "a block `cmd : sm.a.B@x refutes sm.a.B@y` carries two verdicts on one rc",
        message: None,
        help: None,
    },
    CatalogEntry {
        slug: "mark-standalone-rc-consumer",
        when_fires: "the new-grammar mark parser found a standalone mark-block (no command to bind) \
                     carrying an rc-consumer or `reads`. oracle/predict/mark_grammar.rs.",
        why: "28A:rul-continuation-attachment: a standalone block has no statement to measure/back, \
              so a verdict/observe there is unbacked ⇒ the block drops to ⊤ (`inv-top-reject`).",
        params: &[],
        example: "a bare `: sm.a.B@x` line with no preceding command and no continuation",
        message: None,
        help: None,
    },
    CatalogEntry {
        slug: "mark-hashcolon-malformed",
        when_fires: "the new-grammar mark parser found a `#:` comment that looks like a mark-block \
                     but did not parse (`281` §9). oracle/predict/mark_grammar.rs.",
        why: "281 §9 graceful degradation: the hash-colon carrier is left a plain comment (never \
              mis-erased) but diagnosed (Warning) so a broken one is never silently ignored.",
        params: &[],
        example: "a `#: frobnicate` comment reads like a mark but names no known verb",
        message: None,
        help: None,
    },
    // ── plan/records.rs (framed deframer; all SPANLESS) ─────────────────────
    CatalogEntry {
        slug: "records-headerless-refused",
        when_fires: "a records stream carried no framing at all (headerless) on the strict \
                     production path. plan/records.rs deframe_headerless_refused. Static.",
        why: "27D E4: a real dorc probe always frames, so a headerless stream is corruption or an \
              alien source — refuse (kFAIL-withhold, the fold is withheld, the host runs).",
        params: &[],
        example: "sm a records stream carried no `dorc-records/1` framing at all (headerless — \
                  truncated before the header, or a non-dorc source) — refused on the strict \
                  production path, the fold is withheld and the host runs (kFAIL-withhold)",
        message: Some(
            "sm a records stream carried no `dorc-records/1` framing at all (headerless — \
                  truncated before the header, or a non-dorc source) — refused on the strict \
                  production path, the fold is withheld and the host runs (kFAIL-withhold)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-glued-line",
        when_fires: "a records line carried bytes after its terminal token (two atomic writes \
                     glued). plan/records.rs deframe_framed. Static.",
        why: "262 §2: reject the whole read unit, the safe direction (kFAIL-perform).",
        params: &[],
        example: "sm a records line carried bytes after its terminal token (two writes glued) — \
                  the whole read unit is refused, the host runs (kFAIL-perform)",
        message: Some(
            "sm a records line carried bytes after its terminal token (two writes glued) — \
                  the whole read unit is refused, the host runs (kFAIL-perform)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-header-missing",
        when_fires: "a framed records stream carried no header (torn or absent). \
                     plan/records.rs finalize. Static.",
        why: "262 §1/§2: a missing header refuses the read unit (kFAIL-perform).",
        params: &[],
        example: "sm a framed records stream carried no `dorc-records/1` header (torn/absent) — \
                  the read unit is refused, the host runs (kFAIL-perform)",
        message: Some(
            "sm a framed records stream carried no `dorc-records/1` header (torn/absent) — \
                  the read unit is refused, the host runs (kFAIL-perform)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-sentinel-nonce",
        when_fires: "the end-sentinel carried a nonce that is not this attempt's. \
                     plan/records.rs deframe_framed. Static.",
        why: "26A amend-retry-hygiene: ignored — the stream's own records are keyed independently.",
        params: &[],
        example: "sm the end-sentinel carried a nonce that is not this attempt's — ignored (the \
                  stream's own records are keyed independently)",
        message: Some(
            "sm the end-sentinel carried a nonce that is not this attempt's — ignored (the \
                  stream's own records are keyed independently)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-fact-truncated",
        when_fires: "fewer site records arrived than the header declared. plan/records.rs \
                     finalize. `{received}`/`{declared}`/`{unseen}` are the site counts.",
        why: "plans/128 fc-2: a computable range, not a refusal — the unseen sites fold Unknown ⇒ \
              run on their own. NOTE plain-language pass owed: Unknown/⊤-join wording \
              (law-plain-language-surfaces, 24H ack-4).",
        params: &["received", "declared", "unseen"],
        example: "sm fact lane truncated: 3 of 5 declared site records received — the 2 unseen \
                  site(s) fold Unknown (run)",
        message: Some(
            "sm fact lane truncated: {received} of {declared} declared site records received \
                  — the {unseen} unseen site(s) fold Unknown (run)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-integrity-refused",
        when_fires: "the records header failed an integrity key (nonce/attempt/host/book). \
                     plan/records.rs read_header. `{which}` names the mismatched key.",
        why: "262 §2: any known-key mismatch refuses the whole read unit (kFAIL-perform).",
        params: &["which"],
        example: "sm the records header failed integrity on host (a mis-plumbed peer host's \
                  stream) — the whole read unit is refused, the host runs (kFAIL-perform)",
        message: Some(
            "sm the records header failed integrity on {which} — the whole read unit is \
                  refused, the host runs (kFAIL-perform)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-torn-line",
        when_fires: "torn record lines (fragments that lost their terminating write) were counted. \
                     plan/records.rs finalize. `{count}` = the aggregate count.",
        why: "262 §1 pin-late-and-alien-records: counted, never folded; one aggregated warning.",
        params: &["count"],
        example: "sm 2 torn (no terminal token) record line(s) discarded (counted, never folded)",
        message: Some(
            "sm {count} torn (no terminal token) record line(s) discarded (counted, never \
                  folded)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-alien-line",
        when_fires: "alien (non-nonce) record lines were counted. plan/records.rs finalize. \
                     `{count}` = the aggregate count.",
        why: "262 §1 pin-late-and-alien-records: counted, never folded; one aggregated warning.",
        params: &["count"],
        example: "sm 1 alien (non-nonce) record line(s) discarded (counted, never folded)",
        message: Some(
            "sm {count} alien (non-nonce) record line(s) discarded (counted, never folded)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "records-late-line",
        when_fires: "late (after the end-sentinel) record lines were counted. plan/records.rs \
                     finalize. `{count}` = the aggregate count.",
        why: "262 §1 pin-late-and-alien-records: counted, never folded; one aggregated warning.",
        params: &["count"],
        example: "sm 1 late (after the end-sentinel) record line(s) discarded (counted, never \
                  folded)",
        message: Some(
            "sm {count} late (after the end-sentinel) record line(s) discarded (counted, \
                  never folded)",
        ),
        help: None,
    },
    // ── cli/main.rs (footprint / escalation / carry) ────────────────────────
    CatalogEntry {
        slug: "footprint-incoherent",
        when_fires: "a touches() footprint is incoherent. cli/main.rs — TWO emit paths: the SPANNED \
                     own-coordinate canary (footprint omits its own effect coordinate), and the \
                     SPANLESS malformed-derived-coordinate refusal (the SPANLESS_SITE_PAYLOADS one).",
        why: "24A §1b / 24E §7: an at-most claim cannot be partial — refuse ⇒ the site walls. \
              PASSTHROUGH — the per-path text rides `detail`.",
        params: &["detail"],
        example: "sm touches() footprint omits this command's own effect coordinate (at-least ⊄ \
                  at-most) — footprint refused, the site walls",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "touches-escalated",
        when_fires: "a payload-bound touches() escalated to host-derivation. cli/main.rs \
                     merge_derived_footprints. `{site}` = node id, `{call}` = the escalated call.",
        why: "ru-26 SPIKE-ONLY: makes the static→dynamic boundary visible in the render; must not \
              leak into greenfield as a permanent per-escalation requirement.",
        params: &["site", "call"],
        example: "sm site 4: touches() escalated to host-derivation (dpkg-query -W nginx)",
        message: Some("sm site {site}: touches() escalated to host-derivation ({call})"),
        help: None,
    },
    CatalogEntry {
        slug: "deriv-family-incomplete",
        when_fires: "a derived footprint family did not close completely (missing deriv-end, or a \
                     count mismatch). cli/main.rs. `{site}` = node id, `{reason}` = the match.",
        why: "262 §2 / 26A stop-1: an at-most family cannot be partial — refuse ⇒ the site walls \
              total.",
        params: &["site", "reason"],
        example: "sm site 4: derived footprint family incomplete (declared n=3, received 2) — \
                  footprint refused, the site walls total (an at-most claim cannot be partial)",
        message: Some(
            "sm site {site}: derived footprint family incomplete ({reason}) — footprint \
                  refused, the site walls total (an at-most claim cannot be partial)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "escalation-policy",
        when_fires: "the authority-disclosure line for the probe-escalation policy. cli/main.rs \
                     (SPANLESS). Consent legibility.",
        why: "27C §2: the disclosure varies by dial (default vs --escalate-any-probe). PASSTHROUGH \
              — the policy text rides `detail`.",
        params: &["detail"],
        example: "sm escalation policy: probe re-uses connection authority (cap-net-admin) for \
                  `tolerates:`-vouched functions only (default); entry forms: sudo. Forbid with \
                  --no-probe-escalation; widen with --escalate-any-probe.",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "carried-across-substrate-axis",
        when_fires: "a cross-context elision carried a substrate-axis fact via pure-predicate \
                     carry. cli/main.rs. Spanned (the carried site's span).",
        why: "27C §9: every cross-context elision renders its attribution chain from day one. \
              PASSTHROUGH — the chain text rides `detail`.",
        params: &["detail"],
        example: "sm elision carried across the fs-view axis: backing kind `sm_dorc_File` vouches \
                  `invariant:fs-view`; the verdict body is read-set-closed",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "wrapped-site-adoption-hint",
        when_fires: "a wrapped BOOK site degraded on a missing `tolerates:` vouch. cli/main.rs \
                     (27N). Spanned (the wrapped site's span).",
        why: "27C §2 (recognize-never-license): the one-line adoption hint. PASSTHROUGH — the hint \
              text rides `detail`.",
        params: &["detail"],
        example: "sm this `sudo`-wrapped site could elide with a `tolerates:user` vouch on its \
                  is_converged (adoption hint)",
        message: Some("sm {detail}"),
        help: None,
    },
    // ── cli/main.rs (resolver / reaches confusability; SPANLESS) ────────────
    CatalogEntry {
        slug: "resolver-conflict",
        when_fires: "two oracle files declare one kind's resolver. cli/main.rs. `{kind}` = the \
                     kind, `{count}` = the resolver count.",
        why: "24F §3 at-most-one-resolver-per-kind: BOTH refused (never first-wins-silently); the \
              kind keeps token-equality.",
        params: &["kind", "count"],
        example: "sm kind 'sm.dorc.Package' has 2 resolvers across oracle files — \
                  at-most-one-resolver-per-kind (24F §3): BOTH refused, the kind keeps \
                  token-equality (never first-wins-silently)",
        message: Some(
            "sm kind '{kind}' has {count} resolvers across oracle files — \
                  at-most-one-resolver-per-kind (24F §3): BOTH refused, the kind keeps \
                  token-equality (never first-wins-silently)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "resolver-provider-collision",
        when_fires: "a resolver is keyed to a name matching a known COMMAND provider. cli/main.rs. \
                     `{name}` = the colliding name.",
        why: "corr-kind-keying §10: resolvers are keyed by KIND, not command — a likely mis-key \
              (kept; the warning surfaces the risk).",
        params: &["name"],
        example: "sm resolver 'nginx.resolve()' is keyed to a name matching a known COMMAND \
                  provider — resolvers are keyed by KIND, not command (corr-kind-keying §10); this \
                  mints identity for a kind no coordinate may use (a likely mis-key)",
        message: Some(
            "sm resolver '{name}.resolve()' is keyed to a name matching a known COMMAND \
                  provider — resolvers are keyed by KIND, not command (corr-kind-keying §10); this \
                  mints identity for a kind no coordinate may use (a likely mis-key)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "dangling-reference",
        when_fires: "a coordinate resolved DANGLING (no such entity on an enumerable kind). \
                     cli/main.rs dangling_diagnostics. `{coord}` = the rendered coordinate.",
        why: "24F §4: turns a third-party-typo from silent value-loss into a pointed hint; the \
              coord rides the may-alias degrade (the site runs). ADVISORY (fail toward run).",
        params: &["coord"],
        example: "sm coordinate sm.dorc.Package:nginx resolved DANGLING — the kind's resolver \
                  reports no such entity (a likely typo / stale name); it degrades to may-alias \
                  (the site runs)",
        message: Some(
            "sm coordinate {coord} resolved DANGLING — the kind's resolver reports no such \
                  entity (a likely typo / stale name); it degrades to may-alias (the site runs)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "reaches-conflict",
        when_fires: "two oracle files declare one kind's reach-function. cli/main.rs. `{kind}` = \
                     the kind, `{count}` = the reach-function count.",
        why: "24G §4 at-most-one-reaches-per-kind: BOTH refused (never first-wins-silently); the \
              kind's footprints do not expand.",
        params: &["kind", "count"],
        example: "sm kind 'sm.dorc.Package' has 2 reach-functions across oracle files — \
                  at-most-one-reaches-per-kind (24G §4): BOTH refused, the kind's footprints do \
                  not expand (never first-wins-silently)",
        message: Some(
            "sm kind '{kind}' has {count} reach-functions across oracle files — \
                  at-most-one-reaches-per-kind (24G §4): BOTH refused, the kind's footprints do \
                  not expand (never first-wins-silently)",
        ),
        help: None,
    },
    CatalogEntry {
        slug: "reaches-provider-collision",
        when_fires: "a reach-function is keyed to a name matching a known COMMAND provider. \
                     cli/main.rs. `{name}` = the colliding name.",
        why: "24G §4: reaches is keyed by KIND, not command — a likely mis-key.",
        params: &["name"],
        example: "sm reach-function 'nginx.reaches()' is keyed to a name matching a known COMMAND \
                  provider — reaches is keyed by KIND, not command (24G §4); this expands a kind \
                  no coordinate may use (a likely mis-key)",
        message: Some(
            "sm reach-function '{name}.reaches()' is keyed to a name matching a known COMMAND \
                  provider — reaches is keyed by KIND, not command (24G §4); this expands a kind \
                  no coordinate may use (a likely mis-key)",
        ),
        help: None,
    },
    // ── cli/main.rs (wrapper coherence fail-fast; PASSTHROUGH) ──────────────
    CatalogEntry {
        slug: "wrapper-entry-incoherent",
        when_fires: "a wrapper's `__enter` and `__lend_map` disagree on argv flow. cli/main.rs. \
                     Spanned (the entry name_span).",
        why: "27C:rul-fold-entry-coherence-failfast (declarations-genuinely-contradict): a \
              pre-network fail-fast. PASSTHROUGH — the refusal text rides `detail`.",
        params: &["detail"],
        example: "sm wrapper `sudo`: __enter and __lend_map disagree on argv flow (entry consumes \
                  1 leading arg(s), the lend-fold consumes 0) — static incoherence \
                  (27C:rul-fold-entry-coherence-failfast, declarations-genuinely-contradict). The \
                  entry form drops/transforms args the fold relied on; make the entry pass the \
                  fold's guest verbatim.",
        message: Some("sm {detail}"),
        help: None,
    },
    CatalogEntry {
        slug: "wrapper-peel-incoherent",
        when_fires: "a wrapper's `__predict` and `__lend_map` disagree on the peel tail position. \
                     cli/main.rs. Spanned (the predict name_span).",
        why: "273 §5 (declarations-genuinely-contradict): a pre-network fail-fast. PASSTHROUGH — \
              the refusal text rides `detail`.",
        params: &["detail"],
        example: "sm wrapper `sudo`: __predict and __lend_map disagree on the peel tail position \
                  (predict reaches \"$@\" after 1 argv token(s), lend_map after 0) — static \
                  incoherence (273 §5, declarations-genuinely-contradict). The guest would start at \
                  a different token depending on which member dispatched; fix the argparse so both \
                  peel to the same tail.",
        message: Some("sm {detail}"),
        help: None,
    },
    // ── `dorc why --last` durable reader (`27V` Lane B) — the phase's FIRST `[unwritten:]` entries;
    //    prose is a conductor act from this metadata (`27V:rul-error-authorship-tier`). `example` is a
    //    STRAWMAN model for the prose author, not committed prose.
    CatalogEntry {
        slug: "whylog-version-refused",
        when_fires: "`dorc why --last` opened a durable whose `dorc-whylog/N` header tag names a \
                     format version this binary does not understand, so replay is refused (we never \
                     replay a format we cannot parse). plan/whylog.rs parse; cli --last reader.",
        why: "whylog-write-only-replay + versioned-additive format: a durable is version-tagged and \
              NOT byte-stable across versions, so a newer/older format is refused politely rather \
              than mis-parsed. Pull-surface (the user asked): Warning, Floor::None. `{found}` = the \
              tag read from the header. Remediation register (help) wanted: re-run the live analysis \
              (`dorc why` without --last) since the old durable cannot be replayed by this binary.",
        params: &["found"],
        example: "this whylog was written in format `dorc-whylog/2`, which this dorc \
                  (understands `dorc-whylog/1`) cannot replay — re-run `dorc why` live instead",
        message: Some(
            "the saved why-log uses format `{found}`, which this version of dorc cannot \
                  read back",
        ),
        help: Some(
            "ask the question live instead: `dorc why` (without `--last`) re-analyzes the \
                    current book directly",
        ),
    },
    CatalogEntry {
        slug: "whylog-book-desync",
        when_fires: "`dorc why --last` found a durable whose recorded book/oracle content digest (or \
                     its stored decision digest) diverges from the current on-disk inputs, so a \
                     deterministic replay would NOT reconstruct the recorded run. cli --last reader \
                     (the `22F` book-identity/desync guard, cer-2-shaped).",
        why: "determinism-is-the-replay-license: the durable stores digests, not book/oracle \
              CONTENT, and re-reads them from disk; a changed input breaks the replay tie, so it is \
              refused rather than silently replayed against the wrong source. Pull-surface: Warning, \
              Floor::None. `{which}` = the diverged input (`book`, an oracle path, or \
              `decision-digest`). Remediation register (help) wanted: the book/oracle changed since \
              that run — re-run the live analysis for a current answer.",
        params: &["which"],
        example: "the book has changed since this whylog was written (recorded digest ≠ current \
                  `book.sh`), so its recorded decisions cannot be faithfully replayed — re-run \
                  `dorc why` live",
        message: Some(
            "the saved why-log no longer matches what is on disk: `{which}` has changed \
                  since it was written",
        ),
        help: Some(
            "replaying old decisions against changed files would mislead; re-run \
                    `dorc why` live for a current answer",
        ),
    },
    CatalogEntry {
        slug: "whylog-absent",
        when_fires: "`dorc why --last` was asked to replay the last run but no durable exists in the \
                     whylog directory (no prior run wrote one, or the wrong directory). cli --last \
                     reader.",
        why: "the durable is written only when a plan/apply/round-trip run was asked to (spike: the \
              `--whylog-dir` opt-in; product: quietly beside its work); a `--last` with nothing to \
              replay is a benign no-answer, not a crash. Pull-surface: Warning, Floor::None. `{dir}` \
              = the whylog directory searched. Remediation register (help) wanted: run a plan/apply \
              first (or point --whylog-dir at the right directory) to produce a durable to replay.",
        params: &["dir"],
        example: "no whylog to replay in `./.dorc/whylog` — run a plan or apply first (its run \
                  writes the durable that `dorc why --last` reads back)",
        message: Some("no saved why-log to read back in `{dir}`"),
        help: Some(
            "a why-log is saved when a plan or apply runs with `--whylog-dir`; run one \
                    first, or point `--whylog-dir` at the right directory",
        ),
    },
    CatalogEntry {
        slug: "whylog-corrupt",
        when_fires: "`dorc why --last` found a durable but it is truncated or otherwise unparseable \
                     (a partial write, a clobbered file). plan/whylog.rs parse (`inv-no-throw`: \
                     malformed bytes are DATA, never a panic).",
        why: "inv-no-throw: a corrupt durable is diagnostics, never a crash — the reader refuses \
              politely and names the parse-failure reason. Pull-surface: Warning, Floor::None. \
              `{detail}` = the parse-failure reason (e.g. missing header/end sentinel, an \
              unrecognized section). Remediation register (help) wanted: the durable is damaged — \
              re-run the live analysis to regenerate it.",
        params: &["detail"],
        example: "the whylog durable is damaged (no end-sentinel — a partial write?) and cannot be \
                  replayed — re-run `dorc why` live",
        message: Some("the saved why-log is damaged and cannot be read back ({detail})"),
        help: Some(
            "this usually means an interrupted write; re-run `dorc why` live, and the \
                    next plan or apply will save a fresh why-log",
        ),
    },
    // ── cli/main.rs (aid hint) — `AID-NEEDS:aid-unloaded-sibling-oracle` (gap-5 / `24H` ack-6).
    //    UNWRITTEN prose (`27V:rul-error-authorship-tier` — the builder mints the code + metadata
    //    with an empty prose block; the conductor authors the message from this metadata). `example`
    //    is a STRAWMAN model for the prose author, not committed prose.
    CatalogEntry {
        slug: "aid-unloaded-sibling-oracle",
        when_fires: "the cli-edge scan found `*.oracle.sh` files on disk beside the loaded oracles \
                     (or the book) that were NOT loaded this run. cli emit_unloaded_sibling_oracles.",
        why: "24H ack-6 (suggest, never auto-load): a likely-forgotten oracle is a common cause of a \
              wall that a present-but-unloaded oracle would lift; the run is unchanged (advisory Note). \
              PASSTHROUGH — the cli builds `{detail}` (the sorted unloaded-sibling list).",
        params: &["detail"],
        example: "1 sibling oracle exists on disk but was not loaded: `redis.oracle.sh` — load it \
                  with `--oracle` (or `--oracle-dir`) to model its tool; dorc never auto-loads siblings",
        message: Some("sibling oracle files exist on disk but were not loaded: {detail}"),
        help: Some(
            "load them with `--oracle <file>` (or point `--oracle-dir` at their directory); \
             dorc never loads an oracle you did not name",
        ),
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

/// The span-emitting twin of [`fill_template`] (`282` §4): fill `template` into `out`, and for every
/// run push a [`crate::tagged::Span`] classifying it — literal prose as a
/// [`TemplateLiteral`](crate::tagged::Region::TemplateLiteral), a filled declared hole as a
/// [`ParamValue`](crate::tagged::Region::ParamValue), or a `detail`-style passthrough hole
/// ([`is_foreign_param`]) as [`ForeignText`](crate::tagged::Region::ForeignText). Ranges index into
/// `out`, so a caller composing several fills (message, `= help:` connective, help) accumulates ONE
/// gap-free cover. Byte-identical to [`fill_template`] (gate-pinned); an unknown `{name}` folds into
/// the literal run (defensive — the `holes ⊆ params` gate makes it unreachable for committed
/// entries). Every catalog template is single-line today, so each field is one paragraph (index 0);
/// the multi-paragraph split is the `282` §3 seam, deliberately not built. Pure; `inv-no-throw`.
pub fn fill_template_tagged(
    out: &mut String,
    spans: &mut Vec<crate::tagged::Span>,
    template: &str,
    params: &[(&'static str, &str)],
    code: &'static str,
    field: crate::tagged::Field,
    instance: usize,
) {
    use crate::tagged::{Region, Span};
    let literal = |range: std::ops::Range<usize>| Span {
        range,
        region: Region::TemplateLiteral {
            code,
            field,
            paragraph: 0,
            instance,
        },
    };
    let mut lit_start = out.len();
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
                if let Some(&(param, value)) = params.iter().find(|(k, _)| *k == name) {
                    if out.len() > lit_start {
                        spans.push(literal(lit_start..out.len()));
                    }
                    let hole_start = out.len();
                    out.push_str(value);
                    if out.len() > hole_start {
                        let region = if is_foreign_param(param) {
                            Region::ForeignText { param }
                        } else {
                            Region::ParamValue {
                                code,
                                field,
                                param,
                                instance,
                            }
                        };
                        spans.push(Span {
                            range: hole_start..out.len(),
                            region,
                        });
                    }
                    lit_start = out.len();
                } else {
                    out.push('{');
                    out.push_str(&name);
                    out.push('}');
                }
            }
            _ => out.push(c),
        }
    }
    if out.len() > lit_start {
        spans.push(literal(lit_start..out.len()));
    }
}

/// Whether a declared param carries passthrough foreign text (`282:rul-passthrough-type-gated`) —
/// classified as [`ForeignText`](crate::tagged::Region::ForeignText) rather than
/// [`ParamValue`](crate::tagged::Region::ParamValue). Keyed conservatively on the `detail`
/// passthrough convention ([`crate::diag::params_of`] yields `detail` for every PASSTHROUGH code);
/// the type-gated user-sourced distinction is the `282` §8 de-passthrough work, LATER.
#[must_use]
pub fn is_foreign_param(param: &str) -> bool {
    param == "detail"
}

/// Collect a template's `{name}` holes (skipping `{{`/`}}` escapes) — the gate-test primitive
/// (`holes ⊆ declared params`) AND the [`promote_catalog_source`] param-refresh source. Order-
/// preserving, NOT deduped (a hole used twice appears twice); callers that need a param SET dedup.
/// Pure.
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

// ===========================================================================
// The promote pipeline (`27V` §3 · `AID-NEEDS:law-one-defining-case-per-code`)
// ===========================================================================

/// The refreshed param SET for a prose pair — the first-occurrence-ordered, deduped union of the
/// holes in the `message` and `help` templates. Promote sets `params` to EXACTLY the holes the prose
/// uses (tightening the gate's `holes ⊆ params` to `holes == params`). An unwritten (`None`) message
/// contributes no holes.
fn refreshed_params(message: Option<&str>, help: Option<&str>) -> Vec<String> {
    let mut params: Vec<String> = Vec::new();
    for template in message.into_iter().chain(help) {
        for hole in template_holes(template) {
            if !params.contains(&hole) {
                params.push(hole);
            }
        }
    }
    params
}

/// The refreshed `example` — the measured render of the current prose (ru-27 / conductor ruling): the
/// `message` template filled with `<param>` placeholders. Drift-proof by construction (it changes iff
/// the prose does), and payload-free (no canonical `DiagCode` needed), so promote stays a pure
/// function of the committed catalog. An unwritten (`None`) message renders its `[unwritten: <slug>]`
/// placeholder (the same synthesis the render seat performs). The particulars ride
/// `27V:rul-output-form-unwelded`.
fn schematic_example(slug: &str, message: Option<&str>, params: &[String]) -> String {
    let placeholders: Vec<(&str, String)> = params
        .iter()
        .map(|p| (p.as_str(), format!("<{p}>")))
        .collect();
    let refs: Vec<(&str, &str)> = placeholders.iter().map(|(k, v)| (*k, v.as_str())).collect();
    match message {
        Some(template) => fill_template(template, &refs),
        None => format!("[unwritten: {slug}]"),
    }
}

/// The d4b PROMOTE pipeline (`27V` §3; BLESS-law — orchestrator-only, fresh binary, diff inspected;
/// the builder builds this, NEVER runs it): codegen the committed `CATALOG` const from the current
/// catalog, DIFFABLE and committed. It refreshes the machine-facing fields and CARRIES the prose:
/// * `params` ⇐ [`refreshed_params`] (exactly the holes the prose uses);
/// * `example` ⇐ [`schematic_example`] (the measured render of the current prose — drift-proof);
/// * `message` / `help` / `when_fires` / `why` ⇐ carried VERBATIM from the current entry.
///
/// PROSE PROVABLY UNTOUCHED (`tc-promote-refresh-boundary`, conductor-confirmed): promote has no code
/// path that writes a `message`/`help` string other than copying the current entry's — the
/// `promote_is_a_prose_fixpoint` gate pins that structurally. Strings are emitted via `{:?}` (valid
/// Rust escaping); a `cargo fmt` pass over the spliced source is the orchestrator's final step. The
/// `inv-no-unsafe` family stands: this is codegen-to-committed-source, never a macro.
#[must_use]
pub fn promote_catalog_source() -> String {
    serialize(&owned_catalog())
}

/// Codegen the committed `CATALOG` const from an OWNED mirror (`283:dec-catalog-stays-generated-
/// const`) — the promote-v2 serializer, generalized from [`promote_catalog_source`] to owned input so
/// an author's edited prose (carried on the mirror) becomes committed source. Same shape as the const
/// codegen: `message`/`help`/`when_fires`/`why` carried verbatim; `params` ⇐ [`refreshed_params`];
/// `example` ⇐ [`schematic_example`]. Strings emit via `{:?}` (valid Rust escaping); the orchestrator
/// splices + `cargo fmt`s. `inv-no-unsafe` stands (codegen-to-source, not a macro).
#[must_use]
pub fn serialize(entries: &[OwnedEntry]) -> String {
    use std::fmt::Write as _;
    let mut out = String::from("pub const CATALOG: &[CatalogEntry] = &[\n");
    for e in entries {
        let message = e.message.as_deref();
        let help = e.help.as_deref();
        let params = refreshed_params(message, help);
        let example = schematic_example(&e.slug, message, &params);
        out.push_str("    CatalogEntry {\n");
        let _ = writeln!(out, "        slug: {:?},", e.slug);
        let _ = writeln!(out, "        when_fires: {:?},", e.when_fires);
        let _ = writeln!(out, "        why: {:?},", e.why);
        out.push_str("        params: &[");
        for (i, p) in params.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            let _ = write!(out, "{p:?}");
        }
        out.push_str("],\n");
        let _ = writeln!(out, "        example: {example:?},");
        match message {
            Some(m) => {
                let _ = writeln!(out, "        message: Some({m:?}),");
            }
            None => out.push_str("        message: None,\n"),
        }
        match help {
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
            for template in e.message.into_iter().chain(e.help) {
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
            // `message` is `Option` (`283:dec-message-becomes-option`): `None` is the legal unwritten
            // state; a written message must be non-empty (an empty string is an authoring slip).
            assert!(
                e.message != Some(""),
                "`{}`: empty message (use None for unwritten)",
                e.slug
            );
        }
    }

    /// The three-state prose protocol's third state: slugs whose user-facing prose was authored
    /// at conductor/human tier (`27V:rul-error-authorship-tier`). A builder adding prose must
    /// also add the slug HERE — a two-place claim the conductor's diff review catches; never
    /// extend this list from a builder brief.
    const CONDUCTOR_AUTHORED: &[&str] = &[
        "whylog-version-refused",
        "whylog-book-desync",
        "whylog-absent",
        "whylog-corrupt",
        "aid-unloaded-sibling-oracle",
    ];

    /// Gate (`amendment-prose-boundary`): every WRITTEN user-facing register is `sm `-prefixed
    /// base-tip prose or conductor/human-authored (listed in [`CONDUCTOR_AUTHORED`]) — the mechanical
    /// enforcement that builders author no new user-facing prose (`27V:rul-error-authorship-tier`).
    /// Unwritten is now `None` (`283:dec-message-becomes-option`), so a stored `[unwritten:]` string
    /// is no longer legal — it falls through to a loud failure demanding `None`.
    #[test]
    fn message_registers_are_sm_or_unwritten() {
        for e in CATALOG {
            for (field, text) in [("message", e.message), ("help", e.help)] {
                let Some(text) = text else { continue };
                assert!(
                    text.starts_with("sm ") || CONDUCTOR_AUTHORED.contains(&e.slug),
                    "catalog `{}` {field}: a written register must be `sm `-prefixed base-tip prose \
                     or a CONDUCTOR_AUTHORED slug (unwritten prose is `None`), got: {text:?}",
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

    /// PROMOTE is a PROSE FIXPOINT (`tc-promote-refresh-boundary`, conductor-confirmed — "that gate
    /// is the mechanism I wanted"): the generated source carries every entry's `message`/`help`
    /// VERBATIM (promote never regenerates prose), and it is deterministic (idempotent). Together
    /// these make "prose provably untouched" structural, not disciplinary.
    #[test]
    fn promote_is_a_prose_fixpoint() {
        let src = promote_catalog_source();
        for e in CATALOG {
            let message_line = match e.message {
                Some(m) => format!("message: Some({m:?}),"),
                None => "message: None,".to_owned(),
            };
            assert!(
                src.contains(&message_line),
                "promote must carry `{}`'s message VERBATIM (prose never regenerated)",
                e.slug
            );
            if let Some(h) = e.help {
                assert!(
                    src.contains(&format!("help: Some({h:?}),")),
                    "promote must carry `{}`'s help VERBATIM",
                    e.slug
                );
            }
            // when_fires / why are carried too (machine-facing, hand-authored — never regenerated).
            assert!(
                src.contains(&format!("when_fires: {:?},", e.when_fires)),
                "promote must carry `{}`'s when_fires VERBATIM",
                e.slug
            );
        }
        assert_eq!(
            src,
            promote_catalog_source(),
            "promote is deterministic (idempotent)"
        );
    }

    /// The DORC-SIDE metadata gate (`283:dec-promote-v2` · `28A` §2g — the catch the render-level
    /// fixpoint misses): serialize regenerates `params` from the prose's holes, so a hand-edit to any
    /// entry's `params` diverges from the regeneration and trips loudly here. This is the whole-catalog
    /// form of `promote_refreshes_params_and_example`'s spot-checks — the `params` half of the
    /// promote→catalog byte-identity, achievable under carry-forward (params ALREADY match the holes).
    ///
    /// FLAGGED, not enforced here: the `example` half needs the committed examples canonicalized to
    /// their schematic form (47/56 are pre-promote hand-authored strawmen), and a WHOLE-BLOCK byte
    /// gate additionally needs the hand-wrapped literals collapsed to promote's single-line `{:?}`
    /// form — both are a `DORC_CATALOG_PROMOTE` orchestrator canonicalization (BLESS-law), NOT a
    /// builder edit. Wire the whole-block byte gate once the conductor's first promote canonicalizes.
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

    /// PROMOTE refreshes the machine-facing fields: `params` becomes EXACTLY the prose's holes, and
    /// `example` becomes the schematic measured render (holes → `<param>`), drift-proof.
    #[test]
    fn promote_refreshes_params_and_example() {
        // A PASSTHROUGH code (`sm {detail}`): params ⇒ [detail]; example ⇒ the prose with `<detail>`.
        let e = entry("site-unresolvable").expect("passthrough entry");
        let params = refreshed_params(e.message, e.help);
        assert_eq!(
            params,
            vec!["detail".to_owned()],
            "params = the prose's holes"
        );
        assert_eq!(
            schematic_example(e.slug, e.message, &params),
            "sm <detail>",
            "example = the measured render of the current prose (drift-proof)"
        );
        // A code whose template uses a hole TWICE (`{count}` in munge-name-collision) dedups in params.
        let coll = entry("munge-name-collision").expect("collision entry");
        let cp = refreshed_params(coll.message, coll.help);
        assert_eq!(
            cp.iter().filter(|p| *p == "count").count(),
            1,
            "a param used twice appears once in the refreshed set: {cp:?}"
        );
        // The generated source is valid-shaped: the const opener + a closing `];`.
        let src = promote_catalog_source();
        assert!(src.starts_with("pub const CATALOG: &[CatalogEntry] = &[\n"));
        assert!(src.trim_end().ends_with("];"));
    }

    /// The orchestrator's RUN entry for promote (BLESS-law — orchestrator-only, fresh binary, diff
    /// inspected; the builder NEVER runs it with the env set): `DORC_CATALOG_PROMOTE=1` writes the
    /// regenerated `CATALOG` block to `target/catalog-promoted.rs` for diff + splice into
    /// `catalog.rs`, followed by `cargo fmt`. A no-op without the env, so the ordinary suite is inert.
    #[test]
    fn promote_writer_gated() {
        if std::env::var("DORC_CATALOG_PROMOTE").as_deref() != Ok("1") {
            return;
        }
        let out = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/catalog-promoted.rs");
        std::fs::write(&out, promote_catalog_source()).expect("write promoted catalog");
        eprintln!("promote: wrote {} (diff, splice, cargo fmt)", out.display());
    }
}
