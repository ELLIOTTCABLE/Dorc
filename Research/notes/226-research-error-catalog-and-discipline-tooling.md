# 226 — research: error-catalog practice (rq-B) + error-discipline tooling (rq-H)

> Deep-research round, 2026-06-11. Serves PHASE-R of Dorc round-22's error/provenance
> layer. Two fronts: **rq-B** = how real compilers run a large *catalog* of diagnostic
> codes (who maintains them, what rots, retire workflow, per-code severity prior art);
> **rq-H** = the human's hard-add — what *discipline tooling* compilers use to keep
> THEMSELVES honest (completeness gates, type-level emission proofs, golden-test economics,
> warning-ratchets, fault-injection). The build-arc this serves: today Dorc has a
> diagnostics catalog of 5 codes (all severity-Note) while 17 scattered codes bypass it;
> r22 retrofits every code into one catalog with per-code DECLARED severity, then builds a
> CI completeness gate in the Menhir/Pottier direction — *every give-up path must carry a
> registered catalog code*, not merely "every registered code has a template". Findings
> slugged `finding-N` / topical (`friction-fluent-N`, `rot-N`, `ratchet-N`); sources
> `[grade-slug-year]`, graded list in final section. Confidence marks per project
> convention (+SURE / ~SUSPECT / -GUESS / --WONDER).
>
> Predecessor handoff banked below (one gather-turn, re-cited here with full-word slugs;
> its local HTML copies were read, line-numbers re-verified against canonical sources).

## §0 Conclusions up front

**finding-1 (the gate Dorc wants already exists, and it is plain).** +SURE rustc's
error-code completeness gate is *not* a type-system trick — it is a `tidy` check (a plain
Rust test binary the CI runs) that regex-greps the entire `compiler/` tree for `\bE\d{4}\b`
and cross-checks against the registry *both directions*: every registered code must be
emitted somewhere, every emitted code must be registered+documented+tested
[A-rustc-tidy-errorcodes-2026]. This is exactly the Pottier-direction bidirectional gate,
and it is cheap and adoptable in a plain Rust workspace as a `cargo test`/CI step (no
proc-macros). The four stages run in `check()` in order: (1) extract the registry list from
`rustc_error_codes/src/lib.rs`; (2) each code has a long-form `.md` explanation carrying a
`compile_fail` doctest using *its own code*; (3) each code has a UI test (`Exxxx.rs` +
`Exxxx.stderr`) mentioning its own code; (4) each code is actually emitted (the regex sweep)
[A-rustc-tidy-errorcodes-2026].

**finding-2 (rot is real, visible, and *monotone-shrinking* by design).** +SURE the gate
carries TWO hardcoded allow-lists in source — `IGNORE_DOCTEST_CHECK` (5 codes: E0464,
E0570, E0601, E0602, E0717) and `IGNORE_UI_TEST_CHECK` (7 codes: E0461, E0465, E0514,
E0554, E0640, E0717, E0729) — the latter commented verbatim *"This list will eventually be
removed."* [A-rustc-tidy-errorcodes-2026]. This is the visible escape-hatch: codes that
predate a rule are grandfathered. The load-bearing design property for Dorc: the lists are
*hardcoded constants a reviewer sees in the diff*, so the direction of travel is
monotone-shrinking — you cannot silently add to the backlog. The gate also *bidirectionally
polices its own allow-lists*: if a code IS in `IGNORE_UI_TEST_CHECK` but a test file now
exists, that is a hard error ("it shouldn't be listed") — the escape-hatch self-cleans
[A-rustc-tidy-errorcodes-2026].

**finding-3 (retire-don't-delete; codes are append-only, numbers never reused).** +SURE a
*separate* sub-check, `check_removed_error_code_explanation`, runs `git diff --name-status`
against the base commit and HARD-ERRORS if any `error_codes/*.md` is deleted: *"Error code
explanations should never be removed!"* [A-rustc-tidy-errorcodes-2026]. Retirement is done
by prepending a magic marker line `#### Note: this error code is no longer emitted by the
compiler` to the `.md`; the parser (`check_explanation_has_doctest`) special-cases that
exact prefix to mark the code `no_longer_emitted`, which exempts it from stages 3 and 4 (it
must NOT have a live doctest or be emitted) [A-rustc-tidy-errorcodes-2026]. So the registry
is monotonic: numbers are allocated forward, never reclaimed.

**finding-4 (the Fluent/i18n discipline lints were DOWNGRADED — the canonical regret doc).**
+SURE rustc's attempt at a *stricter* discipline — internal lints `untranslatable_diagnostic`
and `diagnostic_outside_of_impl` forcing every user-facing string through the Fluent
translation system at `deny` level — was explicitly walked back to `allow` in Oct 2024
[A-rustc-tracking-132181-2024][A-rustc-pr-132182-2024]. The tracking issue #132181, authored
by a T-compiler MEMBER, lists four friction points verbatim (see `friction-fluent-1..4`) and
states the downgrade decision in an IMPORTANT callout. This is the strongest counter-thesis
in hand: a deny-level "all diagnostics must be structured" discipline imposed real
contributor friction and was relaxed rather than enforced. The lesson for Dorc is NOT "don't
have discipline" — the *grep-based* tidy gate (finding-1) survived and thrives; it was the
*authoring-DSL* mandate (edit-four-files, complex derive) that broke. Cheap structural gates
endure; heavyweight authoring mandates get downgraded.

**finding-5 (ErrorGuaranteed: the type-level emission proof, and its two holes).** +SURE
rustc has near-exactly the shape Dorc wants for "every give-up path carries a diagnostic":
`ErrorGuaranteed` is a zero-sized type unconstructable outside `rustc_errors`, minted only by
`.emit()`. Holding a value statically proves an error was reported → compilation will fail
[A-rustc-devguide-errorguaranteed-2024]. Two caveats bound its use, both relevant to Dorc's
design: (a) it carries NO information about error *kind* (it can be minted by a *delayed* bug
too), so you must not branch emission decisions on possessing one; (b) it means "already
emitted", never "will emit". The hole: it can be minted *wrongly* via `delayed_bug` /
`span_delayed_bug` mechanisms that promise an error will surface later — if that promise is
broken, the guarantee was vacuous (deepened below; a sibling agent covers its
cascade-suppression USE).

**finding-6 (TypeScript: codes are forever, messages are free — the stability split).**
~SUSPECT (strong, pending the policy-doc full-read) TypeScript's model is a useful contrast:
~2000+ numeric codes live in one generated registry (`diagnosticMessages.json`), but the
*message text* is explicitly NOT covered by semver and changes freely, while *code numbers*
are treated as stable identifiers that downstream tooling (eslint, error-suppression
comments, baseline files) keys on. [topic developed below.]

**finding-7 (Menhir .messages: completeness is mechanically enforceable but the *content*
rots).** ~SUSPECT the Menhir model splits cleanly into two costs: (a) *completeness* — that
every error state has a message — is machine-checked by `--list-errors` /
`--compare-errors`, so it cannot silently rot; but (b) the *quality/accuracy* of the message
prose rots hard when the grammar moves, because state numbers churn and messages get
auto-migrated to wrong states. Real projects (CompCert, Catala) keep the *completeness*
green via CI but carry stale message *content*. [developed below.]

**finding-8 (severity-declaration prior art clusters into two granularities, and override
is the norm).** +SURE every mature system lets severity be *overridden* by the consumer, and
the prior art splits by *grouping* granularity: rustc lint levels are per-lint
(allow/warn/deny/forbid, plus `forbid` = un-overridable, plus a `future-incompatible`
ratchet class); Clang ships per-diagnostic *groups* with `-Werror=<group>` selectively
promoting; ESLint is per-rule `off/warn/error` (0/1/2). The fragmentation failure mode Dorc
should fear is real (developed in `finding-13`): when severity is overridable per-call-site,
catalogs drift toward everything-is-a-warning. The one un-overridable level (`forbid`) exists
precisely to prevent that for a chosen few. [developed below.]

---

## §1 rq-H — the completeness gate (rustc tidy `error_codes.rs`)

This is the load-bearing artifact for Dorc's gate. Full canonical source read top-to-bottom
(362 lines, `rust-lang/rust@main`, `src/tools/tidy/src/error_codes.rs`)
[A-rustc-tidy-errorcodes-2026].

**The module docstring states the whole contract** (verbatim, lines 1-17):

> ```
> //! Tidy check to ensure error codes are properly documented and tested.
> //! Overview of check:
> //! 1. We create a list of error codes used by the compiler. Error codes are extracted from `compiler/rustc_error_codes/src/lib.rs`.
> //! 2. We check that the error code has a long-form explanation in `compiler/rustc_error_codes/src/error_codes/`.
> //!   - The explanation is expected to contain a `doctest` that fails with the correct error code. (`EXEMPT_FROM_DOCTEST` *currently* bypasses this check)
> //! 3. We check that the error code has a UI test in `tests/ui/error-codes/`.
> //!   - We ensure that there is both a `Exxxx.rs` file and a corresponding `Exxxx.stderr` file.
> //!   - We also ensure that the error code is used in the tests.
> //! 4. We check that the error code is actually emitted by the compiler.
> //!   - This is done by searching `compiler/` with a regex.
> ```

(Note: the docstring's names `EXEMPT_FROM_DOCTEST`/`EXEMPTED_FROM_TEST` are stale — the
actual constants are `IGNORE_DOCTEST_CHECK` / `IGNORE_UI_TEST_CHECK`. A small instance of the
self-documentation rotting even here.)

**The driver `check()`** runs five steps, the retire-guard first (verbatim):

> ```rust
> pub fn check(root_path: &Path, search_paths: &[&Path], tidy_ctx: TidyCtx) {
>     let mut check = tidy_ctx.start_check("error_codes");
>     // Check that no error code explanation was removed.
>     check_removed_error_code_explanation(&tidy_ctx.base_commit, &mut check);
>     // Stage 1: create list
>     let error_codes = extract_error_codes(root_path, &mut check);
>     // Stage 2: check list has docs
>     let no_longer_emitted = check_error_codes_docs(root_path, &error_codes, &mut check);
>     // Stage 3: check list has UI tests
>     check_error_codes_tests(root_path, &error_codes, &mut check, &no_longer_emitted);
>     // Stage 4: check list is emitted by compiler
>     check_error_codes_used(search_paths, &error_codes, &mut check, &no_longer_emitted);
> }
> ```

**Stage 4 (the bidirectional emission check)** — the half Dorc cares most about, because it
is the "every code is reachable / every give-up is a registered code" direction. Verbatim
(the regex and both error arms):

> ```rust
> // Search for error codes in the form `E0123`.
> let regex = Regex::new(r#"\bE\d{4}\b"#).unwrap();
> ...
>     if !error_codes.contains(&error_code) {
>         // This error code isn't properly defined, we must error.
>         check.error(format!("Error code `{error_code}` is used in the compiler but not defined and documented in `compiler/rustc_error_codes/src/lib.rs`."));
>         continue;
>     }
> ...
> for code in error_codes {
>     if !found_codes.contains(code) && !no_longer_emitted.contains(code) {
>         check.error(format!(
>             "Error code `{code}` exists, but is not emitted by the compiler!\n\
>             Please mark the code as no longer emitted by adding the following note to the top of the `EXXXX.md` file:\n\
>             `#### Note: this error code is no longer emitted by the compiler`\n..."));
>     }
> }
> ```

Note this stage *excludes comment lines* (`if line.trim_start().starts_with("//") { continue; }`)
to avoid counting codes mentioned in prose. Dorc analog: a give-up site that merely *names* a
code in a comment must not satisfy the "is emitted" half — the analyzer must actually
construct/emit it.

**The retire-guard** (`check_removed_error_code_explanation`), verbatim core:

> ```rust
> if diff.lines().any(|line| {
>     line.starts_with('D') && line.contains("compiler/rustc_error_codes/src/error_codes/")
> }) {
>     check.error(format!(
>         r#"Error code explanations should never be removed!
> Take a look at E0001 to see how to handle it."#
>     ));
> ```

**The retire-marker parser** (`check_explanation_has_doctest`) special-cases the exact prefix:

> ```rust
> } else if line.starts_with("#### Note: this error code is no longer emitted by the compiler") {
>     no_longer_emitted = true;
>     found_code_example = true;
>     found_proper_doctest = true;
> }
> ```

**The allow-lists, verbatim with their comments** (the visible rot):

> ```rust
> // Error codes that (for some reason) can't have a doctest in their explanation. ...
> const IGNORE_DOCTEST_CHECK: &[&str] = &["E0464", "E0570", "E0601", "E0602", "E0717"];
> // Error codes that don't yet have a UI test. This list will eventually be removed.
> const IGNORE_UI_TEST_CHECK: &[&str] = &["E0461", "E0465", "E0514", "E0554", "E0640", "E0717", "E0729"];
> ```

**Self-policing allow-lists** — both lists hard-error if a member no longer needs to be
listed (verbatim, stage 3):

> ```rust
> if IGNORE_UI_TEST_CHECK.contains(&code.as_str()) {
>     if test_path.exists() {
>         check.error(format!("Error code `{code}` has a UI test in `tests/ui/error-codes/{code}.rs`, it shouldn't be listed in `EXEMPTED_FROM_TEST`!"));
>     }
>     continue;
> }
> ```

and symmetrically in stage 2 for the doctest list (`has a compile_fail doctest with its own
error code, it shouldn't be listed in IGNORE_DOCTEST_CHECK`).

> design-takeaway for Dorc: this is the cheapest possible completeness gate — a CI test that
> (a) parses a registry, (b) greps source for emit-sites, (c) cross-checks both directions,
> (d) git-diffs to forbid deletion, (e) carries *hardcoded, reviewer-visible, self-cleaning*
> allow-lists for grandfathered exceptions. Every piece is plain Rust + regex + `git diff`;
> nothing needs proc-macros. ~SUSPECT the single most transplantable idea is the
> *self-cleaning allow-list*: it makes the escape-hatch monotone without any external ratchet
> tool.

## §2 rq-B/rq-H — the Fluent migration regret (tracking issue #132181)

The canonical first-party "we tried stricter diagnostic discipline and walked it back"
document. Issue #132181, "Tracking Issue for rustc's translatable diagnostics
infrastructure", opened 2024-10-26 by **jieyouxu** (author_association: **MEMBER**), labels
include `T-compiler`, `WG-diagnostics`, `D-diagnostic-infra`, `S-tracking-needs-deep-research`
[A-rustc-tracking-132181-2024].

**The four friction points, verbatim** (`friction-fluent-1..4`):

> Unfortunately, we have found that the current implementation of diagnostic translation
> infrastructure causes significant friction for compiler contributors when trying to work on
> diagnostics, including but not limited to:
> - Having to edit multiple files (fluent file, `errors.rs` and the emission site, etc.)
> - The diagnostics derive DSL is quite complex and exhibits some quirks
> - Fluent DSL also has its own quirks
> - Sometimes not sufficiently flexible to accommodate diagnostic needs, e.g. see `rustc_const_eval` or other not-migrated examples.

**The downgrade decision, verbatim** (an IMPORTANT callout):

> Based on these friction points, we want to downgrade the internal lints
> `untranslatable_diagnostic`/`diagnostic_outside_of_impl` requiring usage of current
> translatable diagnostic infra from `deny` to `allow`.
>
> If someone wants to continue the translatable diagnostics effort, then they will need to come
> up with a better redesign that causes less friction for compiler contributors.

The unresolved-questions section is itself evidence of how expensive ripping out a diagnostic
discipline scheme is once shipped: *"It's a lot of work and churn to rip it out, as well."*

**The downgrade was executed** in PR #132182 ("Downgrade `untranslatable_diagnostic` and
`diagnostic_outside_of_impl` to `allow`"), authored by jieyouxu, merged, **changedFiles: 1**
[A-rustc-pr-132182-2024]. The PR body adds a telling note about *not* mass-editing the
existing `#[allow(...)]` instances "because that seems like unnecessary additional churn" —
i.e., the discipline had already metastasized into hundreds of per-site allow attributes, and
even *removing* them was deemed not worth it. A second MEMBER (RalfJung) commented linking four
exemplar issues (#121077, #113117, #137223, #128340) showing the friction in the wild.

> design-takeaway for Dorc: the discipline that broke was the *authoring mandate* (force every
> string through a complex derive + Fluent DSL + multi-file edit), enforced by `deny` lints. The
> discipline that *survived* (§1 tidy gate) is a *structural completeness check* that does not
> dictate HOW you author, only that the registry and emit-sites stay in sync. Dorc's gate should
> be the latter shape. ~SUSPECT a deny-level "you must phrase give-ups through THIS API"
> internal lint is adoptable ONLY if the API is genuinely low-friction; otherwise it earns
> hundreds of `#[allow]`s and a downgrade.

## §3 rq-H — ErrorGuaranteed (the type-level "an error was reported" token)

Source: rustc-dev-guide "Guaranteeing an error was emitted" page [A-rustc-devguide-errorguaranteed-2024],
deepened below with compiler source. Verbatim core:

> ErrorGuaranteed is a zero-sized type that is unconstructable outside of the `rustc_errors`
> crate. It is generated whenever an error is reported to the user, so that if your compiler code
> ever encounters a value of type ErrorGuaranteed, the compilation is statically guaranteed to
> fail.
> * It does not convey information about the *kind* of error ... you should not rely on
>   ErrorGuaranteed when deciding whether to emit an error
> * ErrorGuaranteed should not be used to indicate that a compilation will emit an error in the
>   future. It should be used to indicate that an error has already been emitted

**The mechanism (compiler source).** The type lives in `rustc_span` (defined via a newtype
macro with a private inner field, hence unconstructable elsewhere) and is re-exported through
`rustc_errors`; it is minted by `Diag::emit()`. The dev-guide's closing line is the tell on
its *holes*: *"Thankfully, **in most cases**, it should be statically impossible to abuse
`ErrorGuaranteed`."* [A-rustc-devguide-errorguaranteed-2024] — "most cases" because two
escape hatches exist:

**hole-eg-1 (delayed-bug minting).** An `ErrorGuaranteed` can be produced by the *delayed bug*
machinery, not only by a real user-facing error. Direct source evidence from `stash_diagnostic`
(`compiler/rustc_errors/src/lib.rs:584-620`), where stashing an `Error`-level diagnostic mints
its guarantee through a *delayed* bug rather than an emitted error (verbatim):

> ```rust
> let guar = match diag.level {
>     ...
>     // We delay a bug here so that `-Ztreat-err-as-bug -Zeagerly-emit-delayed-bugs`
>     // can be used to create a backtrace at the stashing site ...
>     Error => Some(self.span_delayed_bug(span, format!("stashing {key:?}"))),
>     DelayedBug => { return self.inner.borrow_mut().emit_diagnostic(...); }
>     ForceWarning | Warning | Note | ... | Allow | Expect => None,
> };
> ```

A `span_delayed_bug` is a *promise* that an error will surface before compilation ends; if that
promise is broken (the delayed bug is never "promoted" to a real error because the code path
that should re-encounter it is skipped), the `ErrorGuaranteed` was, in effect, minted for an
error that never reached the user. rustc backstops this with an end-of-compilation assertion
that any unflushed delayed bugs ICE the compiler — i.e. the guarantee is enforced by a *runtime
self-check*, not purely by the type system. ~SUSPECT this is the precise analog Dorc must
worry about: a "give-up token" minted optimistically must be backed by an actual end-of-run
flush check, or it silently becomes vacuous.

**hole-eg-2 (no kind information → cannot drive control flow).** Because the token can come
from a delayed bug or an unrelated earlier error, branching emission decisions on possessing one
is explicitly forbidden by the dev-guide. For Dorc: an `ErrorGuaranteed`-shaped "a refusal was
recorded" token proves *a* give-up happened, but cannot tell you *which* — so it is sound for
"this path is known-failing, suppress cascades" but unsound as the *input* to "should I emit
code X". (The sibling agent covers the cascade-suppression use; the discipline point here is
that the token's deliberate information-poverty is what keeps it sound as a *proof of
emission* while making it useless as a *decision input*.)

> design-takeaway for Dorc: the ZST-minted-only-by-emit pattern is directly transplantable to
> Rust with no proc-macros (a `#[non_exhaustive]`-or-private-field ZST in the diagnostics module,
> returned only by the `.emit()`-equivalent). But finding-5's lesson is that the type-level proof
> is only as good as (a) the absence of an `unchecked`-style public constructor and (b) a runtime
> end-of-run assertion that nothing was promised-but-not-delivered. The type system gets you 90%;
> the last 10% is a `debug_assert!`-grade flush check.

## §4 rq-B — TypeScript's numeric error codes

Source: `microsoft/TypeScript@main`, `src/compiler/diagnosticMessages.json` (8560 lines) +
`DiagnosticCategory` enum [A-typescript-diagmessages-2026].

**The registry shape.** One generated JSON file maps each English message *string* (the key)
to `{category, code}`. Verbatim sample (lines 1-12):

> ```json
> {
>     "Unterminated string literal.": { "category": "Error", "code": 1002 },
>     "Identifier expected.": { "category": "Error", "code": 1003 },
>     "'{0}' expected.": { "category": "Error", "code": 1005 },
> ```

A build step (`generateDiagnostics`) compiles this JSON into `diagnosticInformationMap.generated.ts`,
giving each message a typed accessor. The *key is the message text*, which has a consequence:
renaming a message means touching the JSON key and the generated map, but the *code number*
stays put — codes and messages are decoupled at the storage layer.

**Per-code declared severity = the `category` field.** TypeScript ships exactly four categories
(`DiagnosticCategory` enum, `Warning, Error, Suggestion, Message`)
[A-typescript-diagmessages-2026]. Each registry entry hardcodes one. This is *per-code declared
severity baked into the registry* — the closest direct prior art to what Dorc wants. Note the
ordering quirk in the enum (`Warning = 0, Error = 1, Suggestion = 2, Message = 3`) is *not*
ascending severity, so the enum is an identity tag, not an orderable rank.

**Code-number stability vs message freedom — the policy split (finding-6).** ~SUSPECT (pending a
verbatim policy-doc quote; this is the widely-documented TS team practice rather than a
single-line policy file): TypeScript treats *message text* as explicitly outside semver — wording
changes between minor versions freely — while *code numbers* are de-facto stable identifiers.
Downstream tooling relies on this: `// @ts-expect-error` and error-suppression comments,
`tsc --build` baseline diffs, eslint integrations, and editor quick-fix registries all key on the
numeric code, never the message string. The maintenance economics that follow: adding a diagnostic
= add a JSON entry with the next free code (codes are loosely grouped by subsystem — 1xxx
syntactic/parser, 2xxx semantic, 5xxx command-line, 6xxx etc.), no central registrar; codes are
effectively never reused even when a message is deleted, mirroring rustc's append-only discipline.
[fetch-request filed for the canonical stability statement.]

> design-takeaway for Dorc: TS validates the *decouple code from message* principle — the stable
> identifier is the number/slug, the prose is free to improve. Dorc's catalog should treat the
> code-slug as the API surface (what tooling/tests/users key on) and the message as mutable. The
> `category`-in-registry pattern is the cheap severity-declaration model; TS's four-way split (and
> the fact that downstream keys on code, never severity) suggests severity is metadata *on* the
> code, not part of its identity.
