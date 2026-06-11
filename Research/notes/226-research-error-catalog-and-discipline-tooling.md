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

## §5 rq-B/rq-H — Menhir `.messages` completeness (the central analog)

This is the closest structural analog to Dorc's "every give-up path carries a registered code"
gate: Menhir enumerates *every error state* of an LR(1) automaton and mechanically checks that a
hand-written message database covers them all. Source: Menhir Reference Manual §11.1-11.3
(version 20260209) [A-menhir-manual-2026]; real-project build files from CompCert
[A-compcert-cparser-makefile-2026], Stan [B-stan-dune-messages-2026].

**The three properties, verbatim** (§11.2):

> Ideally, the set of input sentences in a .messages file should be correct (that is, every
> sentence causes an error on its last token), irredundant (that is, no two sentences lead to the
> same error state), and complete (that is, every error state is reached by some sentence).

**The mechanism = two CLI checks** (§11.2):

- `--compile-errors <file>` verifies *correctness + irredundancy*, and on success compiles the
  database to an OCaml function `message : int -> string` mapping a state number to a message.
  The completeness consequence is encoded *in the generated code's totality*: verbatim, *"It
  raises the exception `Not_found` if its argument is not the number of a state for which a
  message has been defined. If the set of input sentences is complete, then it cannot raise
  `Not_found`."* [A-menhir-manual-2026]. So completeness = the generated function is total.
- `--list-errors` generates *from scratch* a minimal sentence reaching every error state;
  `--compare-errors A B` checks the state-set of A ⊆ B. Completeness check = `--list-errors >
  generated; --compare-errors generated handwritten`.

**The maintenance cost is acknowledged IN THE MANUAL** (the load-bearing rot-economics quote,
§11.2, `finding-7`):

> In the case of a grammar that evolves fairly often, it can take significant human time and
> effort to update the .messages file and ensure correctness, irredundancy, and completeness. A
> tempting way of reducing this effort is to abandon completeness. ... We prefer to discourage
> this approach, as it implies that the end user is exposed to a mixture of specific and generic
> syntax error messages ... Instead, we recommend waiting for the grammar to become stable and
> enforcing completeness.

Two further cost signals from the manual: `--list-errors` *"may require large amounts of time
(typically in the tens of seconds, possibly more) and memory (typically in the gigabytes,
possibly more). It requires a 64-bit machine."* [A-menhir-manual-2026]. And there is an explicit
`--merge-errors` command solely to reconcile two contributors' partial `.messages` files — i.e.
the maintenance burden is heavy enough that Menhir ships a *merge tool* for collaborative editing.

**Why message CONTENT rots even when completeness does not (§11.3 "Writing accurate diagnostic
messages").** The deep reason, verbatim:

> The first thing to keep in mind is that a diagnostic message is associated with a *state* s, as
> opposed to a sentence. ... The diagnostic message should not be specific of the sentence w: it
> should make sense regardless of how the state s is reached.

State numbers churn when the grammar changes; `--update-errors` re-generates the `##`
auto-comments but the *human prose* attached to a state may now describe the wrong state. Worse,
in the default (noncanonical) automaton the LR(1) lookahead sets are *over-approximated* (the
manual's Figure 19 worked example: state 8 appears to allow both `SEMICOLON` and `RPAREN` but
"It is *never* the case that both ... are valid continuations") — so even a *complete, correct*
database can carry *inaccurate* messages. The manual offers three workarounds (`--canonical` =
state explosion; selective duplication via phantom params; `%on_error_reduce`), all manual
grammar surgery. So: completeness is cheap to *enforce* (machine-checkable, binary) but message
*accuracy* is expensive to *maintain* (human, fuzzy, churns with the grammar).

**Real-project evidence (do they keep it green?).**

- CompCert: `cparser/GNUmakefile` defines `.PHONY: correct complete update`
  [A-compcert-cparser-makefile-2026]. Verbatim the `complete` rule re-generates and compares,
  printing *"OK. The set of erroneous inputs is complete."*; `correct` prints *"OK. The set of
  erroneous inputs is correct and irredundant."*; `update` runs `--update-errors` to refresh the
  `##` comments after grammar changes. The committed `handcrafted.messages` is **5283 lines**
  (authored partly by Pottier himself — it is the reference exemplar) [A-compcert-cparser-makefile-2026].
  +SURE CompCert keeps completeness green (the `complete` target is wired and the file is large
  and current).
- Stan (`stanc3`): `src/frontend/dune` wires the same two commands directly into the *build
  graph* — a dune rule runs `menhir --list-errors` to produce `parser_new.messages`, then
  `--compare-errors` against the checked-in `parser.messages`, so a stale database FAILS THE
  BUILD [B-stan-dune-messages-2026]. This is the strongest "kept green by CI mechanically" data
  point: completeness is a build dependency, not a manual ritual.
- Tooling friction signal: the dune-native support for `.messages` (auto-generate/update/compile)
  was filed as a feature request (ocaml/dune #3284, 2020) — i.e. for years projects hand-rolled
  the rules in raw `dune`/`Makefile` because the build tool had no first-class support
  [C-dune-issue-3284-2020]. The discipline is real and adopted, but the ergonomics were
  do-it-yourself.

> design-takeaway for Dorc: Menhir is the proof that the *completeness* half of Dorc's gate (every
> give-up state is covered by a registered code) is mechanically enforceable and projects DO keep
> it green when it is wired into the build (Stan) rather than a manual target (weaker). But the
> §11.3 lesson is the warning: the part that rots is *message accuracy/relevance*, which no gate
> catches — Dorc's analog is that a give-up code can be *registered and emitted* yet carry a
> *misleading* explanation, and completeness-checking will never flag it. ~SUSPECT Dorc's
> equivalent of the "state vs sentence" trap is: a give-up code's message must describe the
> *analysis condition* (why the license failed) not the *specific script* that tripped it, or it
> will mislead when the same code fires from a different script. The cheap win: wire the
> completeness check into the build (Stan-style `cargo test`/CI dependency), accept that
> message-quality is a separate, human, un-gateable maintenance line.

## §6 rq-B — per-code DECLARED severity prior art (and the override/fragmentation question)

Four schemes, ordered by granularity of *grouping* and *override power*. The recurring lesson:
every scheme makes severity *overridable by the consumer*, and the systems that wanted a few
diagnostics to be NON-negotiable invented explicit un-overridable levels for exactly that
purpose (`finding-8`).

**rustc lint levels — SIX levels, two of them un-overridable** [A-rustc-lint-levels-2026].
Verbatim ordering: *"lints are divided into six levels: 1. allow 2. expect 3. warn 4. force-warn
5. deny 6. forbid."* The two that resist drift:

> 'force-warn' is a special lint level. It's the same as 'warn' in that a lint at this level will
> produce a warning, but unlike the 'warn' level, the 'force-warn' level cannot be overridden. If
> a lint is set to 'force-warn', it is guaranteed to warn: no more, no less.

> 'forbid' ... is the same as 'deny' in that a lint at this level will produce an error, but
> unlike the 'deny' level, the 'forbid' level can not be overridden to be anything lower than an
> error.

Two more rustc mechanisms map *directly* onto Dorc's "every give-up path must carry a code":

- `expect` level — verbatim: *"it can be helpful to suppress lints, but at the same time ensure
  that the code in question still emits them ... If the lint in question is not emitted, the
  `unfulfilled_lint_expectations` lint triggers ... notifying you that the expectation is no
  longer fulfilled."* This is *the inverse-completeness check at the type/lint level*: a way to
  assert "this site MUST produce diagnostic X" and get told when it stops. ~SUSPECT this is the
  single most relevant severity-prior-art idea for Dorc beyond the tidy gate — an `#[expect]`-style
  assertion on a give-up site that fails CI if that site stops giving up.
- `future-incompatible` lints — a distinguished class that *cannot be silenced*, only warn-or-deny
  (rust issue #34596: *"the behavior for future-incompatible lints would be that they can either
  be 'warn' or 'deny' but can never be completely silenced"*) [B-rust-issue-34596-2016]. A
  severity *floor*, not a fixed level.
- `--cap-lints` — a global ceiling that lowers ALL levels (even `forbid` → warn), used when
  compiling dependencies. So even the "un-overridable" levels have one global escape, deliberately.

**Clang — per-diagnostic severity *class* in tablegen, default-overridable, grouped**
[A-clang-diagnostic-td-2026]. Each diagnostic is declared with a severity-bearing class in
`Diagnostic.td`; verbatim the class hierarchy:

> ```tablegen
> class Error<string str>     : Diagnostic<str, CLASS_ERROR, SEV_Error>, SFINAEFailure { ... }
> class Warning<string str>   : Diagnostic<str, CLASS_WARNING, SEV_Warning>;   // default-on
> class Remark<string str>    : Diagnostic<str, CLASS_REMARK, SEV_Ignored>;
> class Extension<string str> : Diagnostic<str, CLASS_EXTENSION, SEV_Ignored>; // on via -pedantic
> class ExtWarn<string str>   : Diagnostic<str, CLASS_EXTENSION, SEV_Warning>; // default-on
> class DefaultIgnore { Severity DefaultSeverity = SEV_Ignored; }
> class DefaultWarn   { Severity DefaultSeverity = SEV_Warning; }
> ```

Severity is thus declared structurally (the class you pick), modifiable per-diagnostic
(`DefaultIgnore`/`DefaultWarn`), and groupable via `class InGroup<DiagGroup G>`
[A-clang-diagnostic-td-2026]. Consumers override by group: `-Werror=<group>` promotes a whole
group to error, `-Wno-<group>` silences it. So Clang's granularity is *per-diagnostic declaration,
per-group override* — finer than ESLint, coarser-overridable than rustc's per-lint.

**ESLint — three numeric levels, per-rule, last-wins** [B-eslint-rules-2026]. Verbatim: *"off"
or 0; "warn" or 1; "error" or 2*. The config is an array `[severity, ...options]`, fully
consumer-controlled, no un-overridable tier. This is the *maximally fragmentable* end: every
project re-decides every rule's severity, and the well-known failure mode is exactly that
(thousand-line `.eslintrc` severity maps, "everything is warn" drift).

**clippy lint *groups*** (correctness/suspicious/style/complexity/perf/pedantic/nursery/cargo) —
severity is assigned *by group*, with `correctness` defaulting to `deny` and most others to
`warn`/`allow`; the whole group is toggleable (`-W clippy::pedantic`). Group-level default
severity, per-lint override. [covered via A-rustc-lint-levels-2026 ecosystem; not separately
deep-read — ~SUSPECT.]

> design-takeaways for Dorc (severity model):
> - sev-1. EVERY mature scheme makes severity overridable; the only way to keep a chosen few
>   non-negotiable is an explicit un-overridable tier (rustc `forbid`/`force-warn`). +SURE Dorc
>   should declare per-code severity AND mark which codes are floor-pinned (cannot be downgraded
>   by an admin/oracle), or the catalog WILL drift toward all-warnings (ESLint's fate).
> - sev-2. severity-as-a-class-you-instantiate (Clang) couples the declaration site to the
>   severity cheaply and greppably; severity-as-config (ESLint) maximizes fragmentation. Dorc's
>   "spelled in the catalog registry, not at the call site" (TS `category`, Clang class) is the
>   anti-fragmentation choice.
> - sev-3. rustc's `expect` is the sleeper: a positive assertion that a site MUST emit a given
>   diagnostic, CI-failing when it stops. This is the *severity-system expression* of Dorc's
>   completeness wish and worth stealing alongside the tidy-style gate.
> - sev-4. a *severity floor* (future-incompat: warn-or-deny, never off) is a lighter-weight
>   non-negotiability than full `forbid`; useful for "this give-up is always at least a warning."

## §7 rq-B — Elm's error-message ecosystem (the counter-data-point: NO catalog, NO test suite)

Elm is famous for best-in-class compiler errors ("Compiler Errors for Humans", 2015
[B-elm-errors-humans-2015]) yet, crucially for Dorc's maintenance-cost question, maintains them
with *neither a code registry NOR a golden-test suite*. Direct primary evidence: Evan Czaplicki
(Elm's author), answering "how is the Elm compiler tested?" [A-elm-compiler-testing-2018]:

> I have found that testing the compiler gives a relatively uncommon set of tradeoffs. It is a
> project where things may not compile for a week or a month at a time. ... So in the actual
> process of development, I have not found that having tests that run on every single build give
> significant benefits. ... So the time where testing is important is in the lead up to a public
> release ... there are a great deal of things written in Elm, so the process of going through
> things that exist to make them work actually finds pretty much everything.

He explicitly declines to maintain even a regression corpus of "weird programs": *"I am not
personally able to collect and maintain such a list given my existing tasks."* Elm's error
messages have no numeric codes at all — they are hand-crafted prose, version-controlled as code,
validated by *dogfooding against the ecosystem* rather than by any mechanical catalog/snapshot
gate.

> design-takeaway for Dorc: Elm is the existence-proof of the *opposite* pole from rustc/Menhir —
> world-class diagnostics with ZERO catalog/registry/golden machinery, sustained by single-author
> craft + ecosystem battle-testing. The lesson is NOT "skip the gate" (Elm is a one-language,
> one-author project; Dorc is multi-author infra with a correctness mandate). The lesson is that
> the *catalog/gate buys you regression-safety and multi-author consistency, not message quality*
> — quality is orthogonal and comes from craft. ~SUSPECT this sharpens the case that Dorc's gate
> should be scoped narrowly to *completeness/registration* (the thing that rots silently across
> many authors) and NOT over-built into prose-quality enforcement (which Elm shows is a human
> craft problem no gate solves). It also flags a risk: golden/snapshot tests (§8) impose a churn
> cost Elm deliberately refused — adopt them only where the regression-safety payoff is real.

## §8 rq-H — diagnostic snapshot / golden-test economics

The question: what does it cost to golden-test diagnostic output, and is it sustainable?

**rustc UI tests** [A-rustc-devguide-uitests-2026]. The model: a `.rs` test in `tests/ui/`,
expected compiler output stored in adjacent `.stderr`/`.stdout` snapshots, regenerated with
`--bless` and *"then inspect them manually to verify they contain what you expect."* Economics
signals from the dev-guide:

- *Normalization is mandatory and substantial.* Output is normalized (`$DIR`, `$SRC_DIR`,
  pointer-width, etc.) to survive platform differences — i.e. raw golden comparison is too brittle;
  a whole normalization layer exists to keep goldens stable [A-rustc-devguide-uitests-2026].
- *Stray-output tracking is a named problem.* The filename grammar
  (`test-name.revision.compare_mode.extension`) exists explicitly so the harness can *"pattern
  match on [it] in order to track stray test output files."* Golden suites accumulate orphan files;
  rustc engineers a convention to police them.
- *Bless is a footgun.* Issue #134793: a `--bless` run *"ended up generating a bunch of stderr
  files for compiler runs that shouldn't fail"* — blessing blindly accepts whatever the compiler
  currently emits, including newly-wrong output [B-rust-issue-134793-2024]. The review burden is
  the actual cost: bless is cheap, *reviewing the blessed diff* is the work, and skipping that
  review silently codifies regressions.
- *Anti-dedup for honesty.* UI tests run with `-Zdeduplicate-diagnostics=no` *"which disables
  rustc's built-in diagnostic deduplication ... This helps illuminate situations where duplicate
  diagnostics are being generated."* [A-rustc-devguide-uitests-2026] — a deliberate
  discipline choice to make the goldens surface duplicate-emission bugs.

**insta** (the de-facto Rust snapshot library) [B-insta-docs-2026]. Workflow: `assert_snapshot!`
macros store `.snap` files next to tests; failing tests write `.snap.new`; `cargo insta review`
interactively accepts/rejects; controllable via `INSTA_UPDATE` (`auto` writes `.new` only when no
CI is detected). The load-bearing economics line from its own docs: *"Snapshot tests are
particularly useful if your reference values are very large or change often."* — and the CI-aware
default (won't auto-write snapshots when CI is detected) is the built-in guard against the
bless-footgun.

> design-takeaways for Dorc (golden economics):
> - golden-1. golden diagnostic tests are cheap to WRITE/REGENERATE and expensive to REVIEW; the
>   sustainability hinge is review discipline, not tooling. rustc's `--bless` and insta's
>   `.snap.new` both make regeneration trivial — which is exactly why the regression risk moves to
>   the human reviewing the diff.
> - golden-2. raw output is too brittle to golden directly; budget for a normalization layer
>   (rustc's `$DIR` substitutions). For Dorc this means snapshotting the *structured diagnostic*
>   (code + slug + structured fields) not the rendered prose, or the goldens churn on every wording
>   tweak — aligning with finding-6 (code stable, message free).
> - golden-3. CI-detection (insta) is the cheap correctness guard: never auto-accept snapshots in
>   CI, force local review. Directly adoptable.
> - golden-4. Elm (§7) refused goldens precisely for the churn; the reconciliation is golden-2 —
>   snapshot the stable structured layer, not the volatile prose, and the churn Elm feared mostly
>   evaporates.

## §9 rq-H — warning-ratchet / no-new-warnings CI enforcement

The question: how do projects mechanically prevent NEW warnings without the costs of a blanket
deny? The Rust-ecosystem answer is unusually crisp, and it is a *cautionary* one.

**`#![deny(warnings)]` in source is a documented ANTI-PATTERN** [A-rust-patterns-denywarnings-2026].
Verbatim drawback: *"By disallowing the compiler to build with warnings, a crate author opts out of
Rust's famed stability. Sometimes new features or old misfeatures need a change in how things are
done, thus lints are written that `warn` for a certain grace period before being turned to
`deny`. ... All this conspires to potentially break the build whenever something changes."*
Reinforced by the widely-cited "PSA: `deny(warnings)` is actively harmful" — when a future rustc
adds a warning, *every* `deny(warnings)` crate breaks, and (the Crater angle) it pollutes the
compiler team's regression signal [C-reddit-denywarnings-2020].

**The recommended decoupling** [A-rust-patterns-denywarnings-2026]: keep the ratchet OUT of the
source and IN the CI environment — `RUSTFLAGS="-D warnings" cargo build` *"can be done by any
individual developer (or be set in a CI tool ...) without requiring a change to the code."* — or,
if in-source, enumerate specific lints rather than the blanket `warnings` group (the doc ships a
curated ~25-lint list "safe to deny as of rustc 1.48.0"). The internals discussion
[C-internals-notice-2023] surfaces the gap Dorc cares about: *"lints you want reported to the user
but you don't want to deny"* — handled by `RUSTFLAGS=-D warnings --allow <specific>` (deny-all,
allow-some), the standard ratchet shape.

> design-takeaways for Dorc (warning ratchet):
> - ratchet-1. +SURE the lesson is *environment-side ratchet, not source-side*: enforce
>   no-new-warnings via a CI flag (`-D warnings`-equivalent on Dorc's own analyzer build), NOT a
>   blanket in-source deny that breaks every time the toolchain or the catalog grows. This keeps
>   the analyzer buildable as Rust evolves while still failing CI on a regression.
> - ratchet-2. the deny-all-allow-specific pattern is the adoptable shape for Dorc's *own* code
>   hygiene; it is orthogonal to Dorc's user-facing diagnostic severities (don't conflate the
>   analyzer's internal lint posture with the catalog's per-code severity model from §6).
> - ratchet-3. a true "no-NEW-warnings" baseline ratchet (count warnings, fail if the count rises
>   above a committed baseline) is the heavier Chromium/Google-style mechanism; the Rust ecosystem
>   mostly does the binary `-D warnings` instead. --WONDER whether Dorc needs the counting-ratchet
>   at all given a clean from-scratch catalog; the binary gate is far cheaper and probably
>   sufficient this round.

## §10 rq-H — error-path coverage / fault-injection in CI

The question: does anyone mechanically verify that *give-up paths* are actually exercised by tests?

**cargo-mutants** [A-cargo-mutants-2026] is the most directly relevant Rust tool. Its premise:
*"finding places where bugs can be inserted without causing any tests to fail."* It replaces
function bodies with default/alternative values and reports *missed mutants* — mutations no test
caught. For Dorc this is exactly the "is this give-up path tested?" check inverted: if cargo-mutants
can replace a `return Err(GiveUp(code))` (or delete the give-up condition) and all tests still pass,
that error path is unverified. Two limits the tool itself documents [B-cargo-mutants-missed-2026]:
mutations that are genuinely equivalent (e.g. `||`→`&&` where the OS makes both equal) produce
*unactionable* missed-mutants, and *"attributes on expressions are experimental"* so you cannot
always annotate a line to exempt it — the noise has to be managed at the config/glob level. So
cargo-mutants gives error-path coverage signal but with a false-positive tax that needs curation.

**failpoints** (the `fail` crate / FreeBSD-style fault injection) — the other mechanism: explicit
injection points compiled in, triggered at runtime to force error branches. Heavier
(instrumentation in source), more common in distributed-systems testing than compiler work.
~SUSPECT this is closer to Dorc's existing DST/DI discipline than cargo-mutants is: Dorc already
injects clock/network/disk/randomness via DI seams (per AGENTS.md), so forcing a probe to "fail" to
exercise a give-up path is *already* expressible through the existing mock seams — Dorc likely does
NOT need the `fail` crate, it needs to point its existing DST fuzzer at give-up branches.

> design-takeaways for Dorc (fault-injection):
> - fault-1. ~SUSPECT cargo-mutants is the cheapest *adoptable-this-round* mechanism to answer "are
>   our give-up paths tested" — run it scoped to the analyzer's error-emitting functions; treat a
>   missed mutant on a give-up site as a coverage gap. Budget for curating equivalent-mutant noise.
> - fault-2. Dorc's DST harness already provides fault-injection *for free* via its DI seams — the
>   higher-value move is a DST scenario that forces each probe/oracle interaction to fail and
>   asserts a registered give-up code is emitted (this composes with the §6-`expect` idea: assert
>   *which* code). This is more targeted than blanket mutation testing and reuses existing
>   machinery.
> - fault-3. the completeness gate (§1) and fault-injection are COMPLEMENTARY, not redundant: the
>   gate proves every give-up *site* carries a registered code (static); fault-injection proves
>   every give-up *path* is reachable and tested (dynamic). Dorc wants both halves.

## §11 rq-H — how rustc keeps ITSELF honest: the internal-lint family

Beyond the tidy gate (§1), rustc enforces internal discipline through a family of *compiler-only
lints* declared with `declare_tool_lint!` in `compiler/rustc_lint/src/internal.rs`
[A-rustc-internal-lints-2026]. The diagnostic-discipline lints (`untranslatable_diagnostic`,
`diagnostic_outside_of_impl`) that were downgraded in §2 are members of this exact family. The
mechanism is uniform and instructive:

- Each lint is declared `pub rustc::SOME_LINT, Allow, "...", report_in_external_macro: true` — i.e.
  default-`Allow`, then *enabled crate-wide for the compiler's own build*. So the discipline is
  opt-in by the policed codebase, invisible to downstream users.
- The policed API is *marked with an attribute* (e.g. `#[rustc_lint_query_instability]`), and a
  lint-pass greps the HIR for calls to attribute-marked items. Verbatim example from the file:

> ```rust
> /// The `potential_query_instability` lint detects use of methods which can lead to potential
> /// query instability, such as iterating over a `HashMap`. ... queries must return
> /// deterministic, stable results. `HashMap` iteration order can change between compilations,
> /// and will introduce instability if query results expose the order.
> pub rustc::POTENTIAL_QUERY_INSTABILITY, Allow, ...
> ```

This `POTENTIAL_QUERY_INSTABILITY` lint is a near-perfect structural analog to Dorc's *own*
hermeticity discipline (AGENTS.md: clock/network/disk/randomness only through DI seams,
correctness-kernels dependency-clean): rustc marks non-deterministic-order methods with an
attribute and lints any compiler code that touches them. Dorc could mark non-hermetic primitives
and lint kernel code that reaches them — the same attribute-plus-lint-pass shape. The file also
carries `usage_of_ty_tykind`, `untracked_query_information`, a `rustc_must_match_exhaustively`
exhaustiveness lint, and `bad_opt_access` — a whole self-policing suite [A-rustc-internal-lints-2026].

> design-takeaway for Dorc: the `declare_tool_lint!` + marker-attribute + lint-pass pattern is how a
> Rust project enforces "don't call the dangerous thing" discipline ON ITSELF without proc-macros
> (these are `rustc::`-tool lints, not proc-macros). BUT finding-4's caution applies hardest here:
> the *diagnostic-structure* members of this very family (`untranslatable_diagnostic`) were the ones
> downgraded to `allow` under friction. ~SUSPECT the durable members are those policing a *crisp,
> mechanical* property (query determinism, TyKind usage); the fragile member policed a *high-friction
> authoring mandate*. Dorc's hermeticity lint (crisp property) is in the durable category; a "you
> must phrase give-ups through this exact builder" lint (authoring mandate) is in the fragile
> category — keep the give-up API friction near-zero if it is to be lint-enforced. (Caveat: a
> custom `rustc::`-style tool lint requires either compiler-plugin machinery or clippy; for a plain
> workspace the cheaper realization of the same idea is a tidy-style grep test, §1, or
> dylint — flagged as an open thread.)

## §12 rq-H — exhaustiveness-as-completeness analogs beyond Menhir

The question: who else turns parser/automaton state enumeration (or match-exhaustiveness) into a
diagnostic-completeness gate?

- **LALRPOP** — has *error recovery* (the `!` token producing `ErrorRecovery`
  [B-lalrpop-errorrecovery-2026]) but, crucially, NO `.messages`-style completeness gate. Error
  messages are produced ad-hoc from the recovered state, not from an enumerated-and-checked
  database. So LALRPOP is a same-domain tool that *declined* the Menhir discipline — evidence that
  the completeness gate is a deliberate, costly choice, not a free byproduct of LR parsing.
- **tree-sitter / Lezer** — error recovery produces `ERROR`/`MISSING` nodes
  [B-treesitter-errornode-2026] but emits *no diagnostic messages at all* by design; downstream
  tools (editors) interpret the error nodes. There is no message catalog to keep complete. Another
  "declined the discipline" data point, for a different reason (the tool's job is a tree, not
  messages).
- **rustc match-exhaustiveness as a catalog driver** — the compiler's own
  `rustc_must_match_exhaustively` internal lint (§11) and Rust's ordinary non-exhaustive-match
  error are the *language-level* version: an `enum` of give-up reasons + exhaustive `match` makes
  "did you handle every variant" a *type-checker-enforced* completeness gate, for free, with no
  external tool. +SURE this is the cheapest completeness mechanism available to Dorc that it is not
  yet fully using: if every give-up reason is a variant of one `enum DiagnosticCode` (or the catalog
  is one exhaustive enum), then *both* "every code has a template" (match on emit) *and* "every
  give-up maps to a code" (the give-up function returns the enum) become exhaustiveness checks the
  Rust compiler already performs. The tidy-style grep gate (§1) then only needs to cover what the
  type system cannot: that the enum variants are actually *reached* (stage-4 emission) and
  *documented/tested* (stages 2-3).

> design-takeaway for Dorc: the spectrum is (a) type-system exhaustiveness (free, but only proves
> "every variant handled", not "every variant reachable/documented"); (b) Menhir-style enumerate +
> compare (proves reachability-completeness, costs a generate+compare CI step and human message
> upkeep); (c) tidy-style bidirectional grep (proves registry↔emit-site sync + docs + tests, costs
> a regex test). +SURE Dorc should lean on (a) as the spine — make the catalog one exhaustive Rust
> enum so the compiler enforces handle-every-code for free — and add (c) for the reachability/docs
> half the type system misses. (b)'s heavyweight generate-from-scratch is unnecessary for Dorc
> because give-up sites are *explicit code points*, not *implicit automaton states* — Dorc can grep
> its own emit-sites (like tidy stage 4) rather than having to *derive* the complete set the way
> Menhir must derive error states from the grammar. This is a genuine structural advantage: Dorc's
> "states" are nameable in source; Menhir's are not.
