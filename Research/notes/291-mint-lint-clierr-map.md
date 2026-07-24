# 291 — the mint-seam / lint-unification / CLI-error MAP (phases 2–4, one checkpointed lane)

MAP-tier under `27U:map-then-execute-split`, claimed by `289` §2f. This document is the
EXECUTION SPEC: a fresh executor runs it with ZERO re-derivation. Every claim was
verified in-tree at base `35954157ef7e1745c6c70e5f363752c44623766c` (lane branch
`ai/r28-unify-p24`; the post-extraction fold tip — `crates/aid` exists, `diag`/`catalog`/
`tagged`/`narrative` live there). File:line citations are AT THIS BASE.

Charter: `plans/288` §4 (mint seam) · §5 (lint) · §6 (CLI errors) · §8 phases 2–4 ·
`289` §2 (`rul-mint-hardening-package`) · `289` §2e/§2f (riders). Authority order:
root docs > `spike/CLAUDE.md` > `plans/288` > `289` rulings > this file.

FLAGS for the conductor are collected in §8 and are NOT resolved here. Three of them
(`flag-item-four-has-no-render`, `flag-lint-render-default-selection`,
`flag-sm-migration-vs-unwritten`) gate real work and should be ruled before the executor
starts; the rest are ruleable at the mid-lane checkpoint.

---

## §1 — What the executor produces

Three phase-sections, executed IN ORDER inside one lane (phase 2 before phase 3 is
deliberate dogfood — `288` §8; phase 3 before phase 4 because phase 3 widens the tidy
scan-set the phase-4 codes then rely on). ~19 commits (§7a). Expected end-state:
workspace builds; four gates green; `sh e2e/run.sh` 94 cases + 3 lint-cases green; the
`catalog_lock.rs` byte-identity fixpoint green after a conductor-run promote.

No behaviour outside the diagnostics/lint/CLI-error surfaces changes. No probe/plan/
apply licensing changes anywhere (`two-plane-aid-law`: everything this lane touches is
the DESCRIBE plane).

---

## §2 — Grounding: the as-built machinery (do not re-derive)

### §2a — Why a new code cannot exist today (the trawl claim, re-verified)

`dorc_loom::generate::generate_catalog_lock` (`crates/dorc-loom/src/generate.rs:76-115`)
builds its row list by iterating `consumer.mirror()` (`:80-81`). `DorcConsumer::new`
seeds `mirror` from `dorc_aid::catalog::owned_catalog()`
(`crates/dorc-loom/src/consumer.rs:125`), which maps over the compiled-in `CATALOG`
(`crates/aid/src/catalog.rs:128-141`). `CATALOG` is the generated lock. The loop is
therefore closed over the codes that ALREADY have rows: a new `DiagCode` variant can
never gain a lock row, and `catalog_lock.rs` is `@generated` with a byte-identity
fixpoint (`crates/dorc-loom/tests/fixpoint.rs:28-34`) plus a hand-edit refusal
(`crates/aid/src/catalog.rs:446-461`), so a hand-written row is caught. +SURE.

### §2b — Every gate a new code trips (the complete checklist)

| # | gate | file:line | what it demands of a new code |
|---|---|---|---|
| g1 | `slug()` exhaustive match | `aid/src/diag.rs:247` | one arm |
| g2 | `registry()` exhaustive match | `aid/src/diag.rs:1171` | one `CodeSpec` arm (severity + `Floor` + `RemediationClass`) |
| g3 | `params_of()` exhaustive match | `aid/src/diag.rs:1641` | one arm (named params, or `vec![]`) |
| g4 | `every_migrated_payload_name_is_a_real_variant` + `MIGRATED_PAYLOADS` | `aid/tests/diag_tidy.rs:49-123` | payload-struct name added |
| g5 | `MIGRATED_SLUGS` + `assert_no_slug_vanished` | `aid/tests/diag_tidy.rs:128-198` | slug added |
| g6 | `every_catalog_variant_is_constructed` | `aid/tests/diag_tidy.rs:359` | a LITERAL `DiagCode::Payload(` / `Code::Payload(` emit in a non-`aid` SCANNED crate |
| g7 | `spanless_mint_allow_list_is_exact` | `aid/tests/diag_tidy.rs:579` | payload listed in `SPANLESS_SITE_PAYLOADS` iff it mints via `new_spanless_site` (both directions, self-cleaning) |
| g8 | `every_variant_has_exactly_one_catalog_entry` | `aid/tests/diag_tidy.rs:537` | a `CATALOG` row must exist; and no orphan row may name a non-variant |
| g9 | `required_metadata_is_non_empty` | `aid/src/catalog.rs:484` | non-empty `when_fires`, `why`, `example`; `message != Some("")` |
| g10 | `template_holes_are_declared_params` | `aid/src/catalog.rs:466` | every `{{hole}}` in message/help is a declared param |
| g11 | `message_registers_are_sm_or_unwritten` | `aid/src/catalog.rs:516` | a WRITTEN register is `sm `-prefixed OR the slug is case-owned |
| g12 | `every_code_is_case_owned_or_ratcheted` | `aid/tests/catalog_defining_cases.rs:469` | a `.loom` case file exists, or a ratchet row (and never both) |
| g13 | `ratchet_only_shrinks` | `aid/tests/catalog_defining_cases.rs:502` | the ratchet may not grow past the committed baseline (35) |
| g14 | `unwritten_renders_are_greppable_and_pinned` | `aid/tests/catalog_defining_cases.rs:560` | `message: None` count ≤ 6 |
| g15 | `corpus_cases_are_hygienic` (`check_hygiene(Some("code"))`) | `dorc-loom/tests/fixpoint.rs:77`; impl `errorloom/src/container.rs:534-558` | EVERY replay block's output must CONTAIN the slug token |
| g16 | `generated_lock_reproduces_the_committed_bytes` | `dorc-loom/tests/fixpoint.rs:28` | regeneration is byte-identical to the committed lock |
| g17 | `generated_lock_owns_the_complete_catalog_table` | `aid/src/catalog.rs:447` | row-count parity; zero `CatalogEntry {` literals in `catalog.rs` |

**g14 is AT ITS CEILING**: the committed lock carries 58 rows, exactly 6 `message: None`
and 46 `sm `-prefixed (`grep -c` at base). Any code landing unwritten trips it. Every
phase below that mints an unwritten code MUST bump the ceiling as its own conscious act
(the test's own words) — see `rule-unwritten-ceiling-bumps-consciously` (§3a).

**g15 is the scaffold-and-forget guarantee, verified**: `check_hygiene` fails with
`MissingRequiredToken` when a replay block's output does not contain the frontmatter
`code` token. A scaffolded case whose replay output is EMPTY therefore stays red until a
genuinely-firing world is authored and blessed. `288` §4's claim holds as-built. +SURE.

### §2c — The narrative plane as-built — THE `Unexplained` FINDING

**`finding-missing-narrative-silently-omits` (+SURE, the conductor's item-4 question,
answered): a missing narrative SILENTLY OMITS. There is no `Unexplained` anywhere in the
tree.**

Evidence:
1. `rg 'Unexplained' --include=*.rs .` returns ZERO hits at base. `CollapseKind`
   (`aid/src/narrative.rs:319-364`) has nine variants: `FactMergeDisagreement`,
   `VerdictDecline`, `WallFormation`, `SubstitutionRefusal`, `EntryDenial`,
   `EntryFailure`, `Demotion`, `RenderRefusal`, `Cancellation(Reserved)`. No
   `Unexplained`. `spike/CLAUDE.md`'s `collapse-mints-narrative` bullet ("`Unexplained`
   is constructible but renders self-advertisingly") and `AID-NEEDS`'s sibling text are
   ASPIRATIONAL/STALE — a steering-doc drift this lane should correct (§6e).
2. The why-chain does not read narratives at all. `survival_chain`
   (`cli/src/main.rs:3828-3930`) builds its links purely from
   `license.derivation()` + `SurvivalWitness`; `render_chain` (`:3935`) renders that
   struct. Neither takes a `CollapseNarrative`.
3. The one seat that was supposed to consume them ignores them by signature:
   `fn emit_why_lens(…, _collapse_narrative: &[CollapseNarrative])`
   (`cli/src/main.rs:3353-3358`), whose own doc says "Carried through here and IGNORED
   for now — the render arrangement is d4's".
4. The ONLY narrative that reaches a user surface is
   `CollapseKind::VerdictDecline { authored_reason: Some(_) }` via
   `emit_static_decline_notes` (`cli/src/main.rs:4937-4966`) — one `why:` line. All
   eight other classes are minted, threaded, and dropped.

**Consequence for `289:rul-mint-hardening-package` item 4**: it is HARD-MANDATORY by the
conductor's own rule (silent-omit, not self-advertising). But its second clause ("assert
the chain renders it") is CURRENTLY UNSATISFIABLE for eight of nine classes, because no
render consumes them. See `flag-item-four-has-no-render` (§8) and the decomposition in
§3e.

Mint sites (the nine of `288` §2e, re-verified at base):

| # | class | file:line |
|---|---|---|
| m1 | `FactMergeDisagreement` (static, `Derived`) | `analysis/src/effect.rs:1075-1088` (`mint_merge_narrative`), called at `:1380` |
| m2 | `VerdictDecline` (`Vouched`) | `plan/src/lib.rs:1264-1276` |
| m3 | `Demotion` | `plan/src/lib.rs:3254-3262` |
| m4 | `WallFormation` | `plan/src/lib.rs:3270-3278` |
| m5 | `FactMergeDisagreement` within-site (`Measured`) | `cli/src/main.rs:4481` → helper `measured_merge_disagreement` `:4575-4590` |
| m6 | `SubstitutionRefusal` | `cli/src/main.rs:4487-4494` |
| m7 | `EntryFailure` | `cli/src/main.rs:4506-4513` |
| m8 | `FactMergeDisagreement` cross-site (`Measured`) | `cli/src/main.rs:4518-4519` |
| m9 | `EntryDenial` | `cli/src/main.rs:5693-5696` |

The pairing `debug_assert` `288` §2c names is at `analysis/src/effect.rs:1054-1058` — it
lives inside `mint_top_causes` and asserts every Opaque-bearing node has a pre-minted
`Top(cause)`. `mint_merge_narrative` carries NO assert at all: its mirroring of
`mint_top_causes` is by construction only (same `contains(&CommandEffect::Opaque)`
filter, same node-index order). That is the gap item 3 closes (§3e).

### §2d — The loom world dispatch and the worldless gap

`DorcConsumer::world_of` (`dorc-loom/src/consumer.rs:358-378`) dispatches:
`*oracle.sh` section ⇒ `fire_lint_case` (real in-process lint, refuses on slug mismatch,
`:887-914`) · `book.sh` section ⇒ `fire_book_analysis` (real parse→cfg→value→classify,
`:922-948`) · else ⇒ `canonical_payload(slug)` (`:818-881`, world-as-payload).

So `world_of` ALREADY has a worldless arm. The gap is on the REPLAY side:
`DorcConsumer::replay` (`:241-313`) and `render_direct_replay` (`:639-732`) recognize
exactly four command shapes — `dorc-loom vars --used|--all PATH`, `dorc why --last
--whylog=PATH` (`parse_direct_why` `:438`), `dorc lint PATH --no-tools`
(`parse_direct_lint` `:446`), and `dorc plan …` (`parse_direct_plan` `:474`). A
`$ dorc strip` replay matches none, so `replay()` returns `None` (falls to the generic
bytes-only executor, no editable provenance) and `render_direct_replay` returns
`Err("unsupported replay …")` — the case cannot be blessed or regenerated. That is the
whole of the phase-4 precondition. +SURE.

`crates/cli` has NO lib target (`crates/cli/Cargo.toml`: two `[[bin]]`s, no `[lib]`), so
`dorc-loom` cannot reach `parse_args_from` today. `dorc-loom` does not appear in
`crates/cli/Cargo.toml`, so a `dorc-loom → dorc-cli` dep introduces no cycle. +SURE.

`cases_dir()` already exists in the loom bin (`dorc-loom/src/bin/dorc-loom.rs:253-255` =
`CARGO_MANIFEST_DIR/cases`), and `catalog_path()` (`:388`) already points at
`../aid/src/catalog_lock.rs` post-extraction. Scaffold reuses both.

### §2e — The lint finding model as-built

`Finding` (`lint/src/finding.rs:88-113`) carries `code: String`, `message: String`,
`severity: dorc_aid::Severity`, `source: &'static str`, and
`provenance: Option<NativeDiag>`. Code mints:

| finding code | file:line | provenance | span |
|---|---|---|---|
| any `DiagCode` slug (analysis diagnostics) | `lint/src/source_analysis.rs:66` | `Some` | yes/no |
| any `DiagCode` slug (oracle-solo) | `lint/src/source_oracle_solo.rs:95` | `Some` | yes/no |
| `authored-decline-class` | `lint/src/source_oracle_solo.rs:140` | `None` | line+col |
| `unmodeled-wall-inventory` | `lint/src/source_unmodeled.rs:71` | `None` | line+col |
| `verdict-terminal-pipeline` | `lint/src/source_verdict.rs:55` | `None` | line+col |
| `tool-absent` | `lint/src/source_external.rs:91` | `None` | none |
| `external-raw` | `lint/src/source_external.rs:138` | `None` | none |
| `external-text` | `lint/src/source_external.rs:258,272` | `None` | approximate |
| `external-operational` | `lint/src/source_external.rs:335` | `None` | none |
| a foreign tool's own code (`SC2086`, …) | `lint/src/source_external.rs:319` (`code: raw.code`) | `None` | remapped |

Render selection today is keyed on `provenance.is_some()`, NOT on spannedness:
`append_finding_parts` (`lint/src/render.rs:86-109`) emits a FRAMED `render_cli_parts`
block for provenance-carrying findings and the compact `render_finding_line`
(`:113-131`) otherwise. Both shapes appear side-by-side in one committed golden —
`e2e/lint-cases/eval-wall-findings/expected.out` shows two framed
`error[cfg-top-node]`/`error[syntax-unsupported]` blocks followed by a compact
`1:1 info [unmodeled-inventory:unmodeled-wall-inventory] …` line. That file is
HAND-AUTHORED and NEVER blessed (`e2e/run.sh:1418-1425`).

Golden surface affected by any lint-render change: 3 `e2e/lint-cases/*/expected.out` +
the 8 `.loom` cases named in `dorc-loom/tests/fixpoint.rs:90-99` (`LINT_CASES`).

`SCANNED_CRATES` (`aid/tests/diag_tidy.rs:240-242`) = aid, syntax, analysis, oracle,
plan, cli, coverage, hostsim — it OMITS `lint` and `dorc-loom`. Until it is widened
(rider `289:rider-diag-tidy-scan-set`), a lint-crate emit site does not satisfy g6.

### §2f — The `dorc: {msg}` producer inventory (re-grepped at base)

`main()` prints `dorc: {msg}` at four seats: `cli/src/main.rs:201` (strip),
`:212` (analyze), `:217` (parse), and the lint variants `:838`, `:845`, `:852`.
Producers:

`parse_args_from` (`:322-514`) — `:336` strip-needs-a-path · `:338` strip-got-a-flag ·
`:395` unknown-mode + did-you-mean · `:408 :410 :416 :420 :445 :453` flag-needs-a-value
(×6) · `:437` unknown `--probe-capability` value · `:476` unknown-flag + did-you-mean ·
`:479` unknown-flag bare · `:488` no-book-given · `:493` `--whylog`/`--whylog-dir`
mutually exclusive · `:496` `--whylog` only with `why --last`.

`parse_lint_args` (`:648-712`) — `:665 :667 :673 :677 :686 :693` flag-needs-a-value (×6)
· `:695` unknown lint flag. `parse_lint_format` `:718` · `parse_fail_on` `:731` ·
`parse_expect_count` `:739` (three "not one of the accepted values" shapes).

`humane_read_error` (`:570-577`) — three world-states: `NotFound` ⇒ "no such file",
`PermissionDenied` ⇒ "permission denied", other ⇒ the raw OS string. Called from
`strip_command` `:559`, `read_books` `:588`, `resolve_oracle_paths` `:601`,
`read_lint_inputs` `:930`.

Lint operational trio — `:884` no-lintable-files · `:897` `--expect-files` drift ·
`:912` `--require-tools` missing. Plus `:1746` (whylog dir unset under `--last`).

`dorc-sh` (`cli/src/bin/dorc-sh.rs`) — `:30` usage · `:36` cannot-read-script ·
`:55` cannot-exec-sh.

**`finding-cli-error-prefix-has-no-goldens` (+SURE)**: NOTHING pins the `dorc: `/
`dorc-sh: ` prefix. `grep -rl` over `e2e/`, `crates/*/tests`, and
`crates/dorc-loom/cases` for `^dorc: `, `dorc: lint`, `dorc-sh: ` returns EMPTY. Phase
4's render change therefore causes ZERO golden churn — the migration's risk is
concentrated entirely in the worldless-route design, not in expectation churn.

---

## §3 — PHASE 2: the mint seam, scaffold, and the hardening package

### §3a — `step-mirror-union-over-case-slugs` (`288` §4 mirror-union)

Change `generate_catalog_lock` (`dorc-loom/src/generate.rs:76-115`) to iterate the UNION
of mirror slugs and case slugs rather than `consumer.mirror()` alone:

- keep the existing mirror rows in existing order (this is what keeps the committed lock
  byte-identical and g16 green);
- then APPEND, sorted by slug, a row for every `cases` key with no mirror entry:
  `message: None`, `help: None`, `params: refreshed_params(None, None)` (empty),
  `when_fires`/`why` from the case frontmatter, `example` from `case_example` (which
  returns `format!("[unwritten: {slug}]")` for a `None` message — `generate.rs:150-158`).

Order rationale (`rule-new-rows-append-sorted`): appending keeps every existing row's
bytes untouched, so the first regeneration after a mint is a pure addition and the diff
is reviewable; after promote the new slug is in `CATALOG`, the mirror carries it, and the
union is idempotent. Do NOT re-sort the whole table — that is a one-time 58-row churn
buying nothing.

Deliberate deviation from `288` §4's wording: `288` says "unions mirror rows with
ENUM-derived rows". Take CASE-derived instead. Rationale: `282:rul-transcript-is-the-
authoring-surface` already makes the case file the authoring surface; the case file must
exist anyway under `rul-loom-mint-guarantee`; and an enum-derived union would need a new
hand-maintained slug list in `aid` duplicating `MIGRATED_SLUGS`. A case naming a
non-variant slug is caught by g8's orphan direction, so the union stays self-cleaning.
(Recorded as `flag-mirror-union-source-is-cases`, §8 — low stakes, but it is a departure
from the charter's literal words.)

`rule-unwritten-ceiling-bumps-consciously`: g14's ceiling (6, currently exactly met) is
bumped in the SAME commit as each mint, to the exact new count, with the bump's reason in
the assert message. Never bump speculatively, never bump by a round number.

### §3b — `step-repair-command-named-verbatim` (`rul-loom-mint-guarantee`)

Two failure messages must name the repair command VERBATIM. Both are edits to existing
assert strings; neither changes logic.

1. `aid/tests/diag_tidy.rs:550-551` (`every_variant_has_exactly_one_catalog_entry`)
   currently says "Add its entry to aid/src/catalog.rs." — WRONG post-flip (the lock is
   generated; a hand-row is refused by g17/g16). Replace with the mint recipe naming
   `dorc-loom scaffold <slug>` and `dorc-loom promote` verbatim.
2. `aid/tests/catalog_defining_cases.rs:477-482`
   (`every_code_is_case_owned_or_ratcheted`) currently says "backport it to
   `crates/dorc-loom/cases/{slug}.loom`". Replace the manual instruction with
   `dorc-loom scaffold <slug>` verbatim (keep the ratchet alternative sentence).

`rule-repair-string-is-test-pinned`: add one assertion that the literal
`dorc-loom scaffold ` substring appears in both messages, so a later reword cannot
silently drop the repair command. (Cheapest form: a `const REPAIR_HINT: &str` used in
both messages plus a trivial `assert!(REPAIR_HINT.contains("dorc-loom scaffold"))` — the
point is greppability, not cleverness.)

### §3c — `step-scaffold-explicit-command` (`prop-scaffold-explicit-command`, ACKED)

Add `dorc-loom scaffold <slug>` to the bin's command surface
(`dorc-loom/src/bin/dorc-loom.rs`): a `Command::Scaffold { slug: String }` arm in
`enum Command` (`:29-33`) and `parse_args` (`:54-79`), plus `USAGE` (`:17`).

Behaviour, exactly:
- resolve the target as `cases_dir().join(format!("{slug}.loom"))` (reuse `cases_dir()`
  `:253`). The phase-5 tree move relocates the collection later — **do NOT pre-create any
  new tree shape, do NOT parameterize the directory**;
- REFUSE if the file exists (never overwrite an authored case);
- REFUSE a slug that is not `[a-z0-9-]+` (the observed slug charset across all 23
  committed cases and `MIGRATED_SLUGS`);
- write the skeleton below and print the path plus the next two steps;
- write NOTHING else. Not a build step, not a test side-effect (`prop-scaffold-explicit-
  command`'s whole point: tests never write source; concurrent builders never race).

Skeleton (matches the committed shape — cf. `dorc-loom/cases/syntax-unsupported.loom`,
7 lines, and `records-fact-truncated.loom`):

```
---
code: <slug>
when-fires:
why:
---
-- replay --
$ dorc plan --book=book.sh
```

Everything is deliberately empty or red:
- empty `when-fires`/`why` ⇒ g9 (`required_metadata_is_non_empty`) FAILS until the
  builder authors the machine-facing metadata (which IS builder work — `catalog.rs:29-31`
  makes `when_fires`/`why`/`params`/`example` conductor/machine-facing and
  builder-authored, unlike `message`/`help`);
- an empty replay OUTPUT ⇒ g15 (`check_hygiene`) FAILS with `MissingRequiredToken` until
  a genuinely-firing world is authored and blessed. That is the scaffold-and-forget guard
  `288` §4 promises, and it is free — verified in §2b;
- `message` is never written by scaffold, so the code renders `[unwritten: <slug>]` at
  every seat (`catalog.rs:66-77`: a missing entry and an unwritten entry both yield
  `None`, and the render synthesizes the placeholder). Builders author ZERO user-facing
  prose, ever (`error-authorship-tier`).

The placeholder replay command in the skeleton is a HINT, not a contract; the builder
replaces it with whatever world the code actually fires in.

### §3d — `step-mint-to-green-walkthrough-loom` (the `282` empty loop, exercised)

Mint one genuinely-new code end-to-end THROUGH the new machinery, and commit its case as
the canonical walkthrough. This is the dogfood; it is also the only proof the seam works.

Pick the code from phase 4's inventory so the walkthrough is not throwaway: take
`cli-strip-needs-path` (`main.rs:336`) — the smallest, most obviously-worldless
invocation error. Do it AFTER §5's worldless route lands if the executor prefers one pass
over the CLI code; do it with a world-as-payload replay first if the checkpoint pushes
the worldless route later. Either way the walkthrough case is the artifact.

The walkthrough sequence, which the executor must actually run and which becomes the
commit's evidence:

1. `dorc-loom scaffold cli-strip-needs-path` → red (g9, g15).
2. Add the payload struct + `DiagCode` variant + g1/g2/g3 arms + the emit site + g4/g5
   (+ g7 if spanless — it is) → g8 still red (no lock row).
3. Author the case's `when-fires`/`why` + a real replay whose output carries the slug →
   `dorc-loom compile <case>` shows the interpretation.
4. Conductor runs `dorc-loom promote <case>` (promotion is orchestrator-only on a freshly
   verified binary — `287` §5); the lock gains the appended `message: None` row.
5. Bump g14's ceiling by exactly the number of new unwritten codes.
6. All gates green; the rendered case shows `[unwritten: cli-strip-needs-path]`.

`rule-walkthrough-prose-stays-unwritten`: the walkthrough loom's prose stays
`[unwritten:]`. Do not write user-facing text for it, ever — that is phase 8's, held for
the human (`289` §0). The builder's when/why report is what the prose pass later reads.

### §3e — `step-hardening-package-four-parts` (`289:rul-mint-hardening-package`)

**h1 — no-wildcard exhaustive `match CollapseKind` in a completeness gate.** There is no
completeness gate today; build one. Site it in `aid` (the crate that owns the type) as a
new integration test `crates/aid/tests/narrative_completeness.rs`. It must contain a
`match` over a `CollapseKind` value with ONE ARM PER VARIANT and NO `_` arm, mapping each
variant to its expected mint-site census key (§h2). The compiler then forces every future
collapse class to visit the pairing site. `Cancellation(Reserved)` is uninhabited —
handle it as `CollapseKind::Cancellation(r) => match *r {}` (the pattern already used at
`aid/src/narrative.rs:541-543`).

**h2 — tidy-style census over collapse-constructor call-sites.** Model it on
`diag_tidy.rs`'s `every_catalog_variant_is_constructed` (`:359-368`): a lexical scan of
`concat_crate_src(&["analysis", "plan", "cli"])` for the literal marker
`CollapseKind::<Variant>` (both the `dorc_aid::CollapseKind::` and bare `CollapseKind::`
spellings appear at base — m1 uses the fully-qualified form, m6/m7/m9 the bare one).
Assert each constructible variant appears at ≥1 site. Reuse `diag_tidy.rs`'s
`rs_files`/`concat_crate_src` shape rather than inventing a second scanner; copying ~30
lines is correct here (test code is not DRY — `spike/CLAUDE.md` Code style).

Disclose the same needle-shape limit `diag_tidy.rs:350-357` discloses (a non-literal
construction is invisible), and the same `#[cfg(test)]`-blindness caveat
(`diag_tidy.rs:296-304`). Do not overclaim: this is a belt-and-braces backstop.

**h3 — promote the merge-mint pairing assert to a release-mode test gate.** The
`debug_assert` at `analysis/src/effect.rs:1054-1058` is inside `mint_top_causes` and
covers the CAUSE mint, not the NARRATIVE mint. `mint_merge_narrative` (`:1075-1088`) has
no assert. Add a release-mode unit test in `analysis/src/effect.rs`'s existing test module
asserting, over at least one Opaque-bearing and one Opaque-free book, the cardinality and
ORDER parity: `mint_merge_narrative(&effects).len() == effects.iter().filter(|e|
e.contains(&CommandEffect::Opaque)).count()`, and that the minted `cell` leaf ids equal
the Opaque-bearing node indices in ascending order. The existing collapse helper at
`effect.rs:1801-1819` already builds the fixture shape; extend rather than duplicate.
Keep the `debug_assert` where it is.

**h4 — DST fault-injection per collapse class. HARD-MANDATORY (§2c), BUT SPLIT.**

- **h4a (build now, all classes)**: per collapse class, force the seam and assert the
  narrative IS minted with the right tier and operands. Six of nine already have partial
  pins that the executor extends rather than replaces: `cli/src/main.rs:6698-6713`
  (`SubstitutionRefusal` both directions), `:6447/:6472` (`EntryFailure`),
  `:6827-6855` (`FactMergeDisagreement` measured, both directions),
  `plan/src/lib.rs:6234-6260` (`WallFormation` + `Demotion`),
  `plan/src/lib.rs:4882-4886` (`VerdictDecline`),
  `analysis/src/effect.rs:1801-1819` (`FactMergeDisagreement` derived). Genuinely
  unpinned: `EntryDenial` (m9) and `RenderRefusal`. Add those two; add an
  agreement/negative direction wherever a class has only a positive pin. Honor
  `anti-masking-tests` — never hand-inject a narrative the seam should mint (the
  existing tests already state this: `effect.rs:1778`).
- **h4b (the "AND the chain renders it" clause)**: BLOCKED, see
  `flag-item-four-has-no-render` (§8). Today only `VerdictDecline`+`authored_reason`
  renders (`cli/src/main.rs:4937-4966`), so h4b is satisfiable for exactly one class.
  Build it for that one class (extend the existing `pair_authored_reasons` tests at
  `cli/src/main.rs:6348-6378` with a render-side assertion), and record the other eight as
  a named, dated gap in the new test file's module docs. Do NOT build a narrative render
  to satisfy the clause: that is the d4 arrangement walker, deliberately deferred
  (`cli/src/main.rs:3351-3352`), and surfacing it early welds `render-form-unwelded`
  output the phase-7 arrangement-home sitting owns.

**h5 — the escalation seam stays UNBUILT.** `289:rul-mint-hardening-package` names
value-carriage-in-the-join as the priced next rung, to build only if the census ever leaks
a real under-narration. Build NOTHING toward it. One sentence in the new test file's docs
naming it is the whole deliverable.

### §3f — `rider-narrative-prose-sweep` (`289:rider-narrative-prose-sweep`)

Sweep doc-comment/comment PROSE saying "evidence" → "narrative" ONLY where it names the
narrative plane. The complete enumeration at base (verified line by line):

**SWEEP (narrative-plane prose) — 19 comment/doc mentions:**
- `cli/src/main.rs`: `:1567` ("union the collapse-evidence onto the why-lens seam") ·
  `:3351` ("decision-inert evidence seam") · `:3804` ("evidence-kinds the chain carries") ·
  `:3827` ("derived from evidence-kind presence only") · `:4432` ("`Measured` fact-merge
  evidence minted beside the ⊤-fold") · `:4567` ("Build the `Measured`-tier fact-merge
  evidence") · `:4876` ("`VerdictDecline` evidence via …") · `:4927` ("`VerdictDecline`
  evidence carries an `authored_reason`") · `:5476` ("`EntryDenial` evidence minted").
- `plan/src/lib.rs`: `:1163` ("decline-evidence mint") · `:1179` ("VerdictDecline
  evidence") · `:1447` ("`WallFormation` / `Demotion` evidence") · `:1468` ("wall/demotion
  collapse-evidence").
- `analysis/src/effect.rs`: `:1062` ("fact-merge evidence") · `:1067` ("the evidence rides
  OUT") · `:1310` ("collapse-evidence") · `:1334` ("the C3 collapse-evidence aid plane") ·
  `:1379` ("narrate the give-up as decision-inert evidence") · `:1778` ("evidence (one
  `Derived` FactMergeDisagreement …)").
- `aid/src/narrative.rs`: 19 occurrences in module + method docs, ALL narrative-plane (the
  module IS the type home): the `compile_fail` doctest comment `:28-30`, `:35` (`27V:rul-
  collapse-mints-evidence` citation), `:44-48` (the Eq-at-the-carrier block), `:90`,
  `:377-380` (the `CollapseNarrative` doc), `:388-390`, `:409-414`
  (`with_authored_reason`), plus the test-module comments `:461-463`, `:618`. Sweep all;
  keep the historical law-slug citation `27V:rul-collapse-mints-evidence` intact where it
  cites a DOCUMENT (doc citations are not code — `288` §2c) but subscript it once as
  "(né …)" per the naming discipline.

**DO NOT TOUCH (the r29 fence, `289` §2d) — verified license/host-evidence plane:**
- `aid/src/diag.rs`: ALL 15 occurrences are `HostEvidence*` (`:185-186`, `:288`,
  `:713-736`, `:1396`, `:1738`, `:2571-2586`). The file is entirely fenced. +SURE.
- `plan/src/records.rs` (all `read_host_evidence`/admission prose and locals),
  `plan/src/whylog.rs:575`, `:1400`, `:1438`.
- `cli/src/main.rs:183` ("Host evidence failed admission"), `:1371-1384` (the
  `read_host_evidence` call-site and its local `evidence`), `:4688` (`ScopedHostEvidence`
  — "live evidence participates in planning").

**AMBIGUOUS — flagged, not swept without a ruling** (`flag-narrative-identifier-scope`,
§8): local bindings and parameter names spelled `evidence` in narrative-plane code —
`cli/src/main.rs:4888/4892` (the `pair_authored_reasons` parameter), the test locals at
`:6348`, `:6366`, `:6698`, `:6700`, `:6709`, `:6711`, `:6838-6855`,
`plan/src/lib.rs:4882-4883`, `analysis/src/effect.rs:1801-1819`, and the test NAMES
`plan/src/lib.rs:6234` (`survival_walk_mints_wall_and_demotion_evidence`),
`cli/src/main.rs:6827` (`same_cell_disagreement_mints_measured_evidence_…`). The rider's
letter says doc-comment PROSE. Identifiers are a wider, purely-mechanical diff.
Recommendation: sweep the parameter name and the two test names (they are read as prose by
the next person), leave short-lived test locals. Conductor rules.

### §3g — Phase 2 commit sequence and gates

See §7a for the whole-lane sequence. Phase-2 tests to run per commit:
`cargo test -p dorc-aid -p dorc-loom -p dorc-analysis -p dorc-plan -p dorc-cli`.

---

## §4 — PHASE 3: lint unification (`288` §5)

### §4a — `step-native-findings-become-registry-codes`

The seven dorc-MINTED lint codes (`§2e`) become `DiagCode` variants. The foreign relay
codes (`raw.code` at `source_external.rs:319`) stay source-tagged relays forever
(`288` §5) — untouched.

Slug minting per `288:rul-error-slugs-are-semantic` (semantic-first, user-googleable,
never file-naming-driven; no target count in either direction). Recommended mapping
(conductor may re-cut any of these; the count is not a target):

| today | recommended code | note |
|---|---|---|
| `unmodeled-wall-inventory` | `unmodeled-wall-inventory` | already semantic; KEEP the slug (no mapping layer needed — it becomes a registry code under the same name) |
| `verdict-terminal-pipeline` | `verdict-terminal-pipeline` | KEEP |
| `authored-decline-class` | `authored-decline-class` | KEEP |
| `tool-absent` | `lint-tool-absent` | the bare word is too generic for a googleable global namespace |
| `external-raw` | `lint-tool-output-unparsable` | says what happened, not which tier produced it |
| `external-text` | (no code) | it is a RELAY tier marker, not a finding class — see below |
| `external-operational` | `lint-tool-failed-without-findings` | |

`external-text` is the odd one: it is not a finding class at all, it is the tolerant-text
adapter's placeholder code on a finding whose real content is the foreign tool's message.
Recommendation: treat it as a RELAY (leave it a bare string like the foreign codes) and
let `RemapFidelity::Approximate` carry the "we guessed the location" signal it already
carries. Recorded as `flag-external-text-is-a-relay-not-a-code` (§8).

Each new code walks the g1–g17 checklist of §2b. All seven are span-BEARING except
`lint-tool-absent` and `lint-tool-failed-without-findings` (which have `path`/no span) —
but note they are minted as `Finding`s, not via `Diag::new_spanless_site`, so g7 only
engages if the executor routes them through a `Diag`. See §4c.

`rule-lint-codes-ship-covered-cases` (`28A:rul-new-codes-ship-covered-cases`, now via the
phase-2 seam): every new/renamed code ships a covered defining case, minted with
`dorc-loom scaffold`. The lint cases have a working world-as-pipeline path already
(`fire_lint_case`, and `lint_cases_replay_the_complete_production_report` at
`dorc-loom/tests/fixpoint.rs:88-140`), so these cases are HONEST-trigger for free: add
each new slug's filename to the `LINT_CASES` array (`:90-99`) — it is a hand-maintained
`[&str; 8]`, so the array length changes with it.

### §4b — `step-lint-prose-migrates-verbatim`

The existing hand-written messages (`source_unmodeled.rs:60-64`,
`source_verdict.rs:56-61`, `source_external.rs:92-96`, `:139-142`, `:336`, and
`decline_message`) become catalog templates. **They migrate VERBATIM with the `sm `
prefix, not as `[unwritten:]`** — see `flag-sm-migration-vs-unwritten` (§8) for why this
needs a ruling, and §8's recommendation. Parameterize only what is already interpolated
(`{tool}`, `{count}`, `{downstream}`, `{rc}`), and only via `params_of` arms; never
hand-format a value into prose (`law-trust-tier-is-syntax`).

Note the visible consequence either way: today
`e2e/lint-cases/eval-wall-findings/expected.out` already renders `sm `-prefixed catalog
prose for `cfg-top-node`/`syntax-unsupported`, so `sm `-migration keeps the surface
consistent; `[unwritten:]` would make three constantly-firing lint findings render as
placeholders for the rest of the arc.

### §4c — `step-lint-render-is-selection-policy` (`288` §5)

`288` §5 and the human's verbosity lean: the lint surface is a SELECTION POLICY; default
keeps the compact line-per-finding shape; `--verbose` may add source frames; `--terse`
compresses further. Riding `KNOBS:kFLOW` / `27V:rul-output-form-unwelded`.

The mechanical problem: today's default is not a policy, it is an accident of
`provenance.is_some()` (`lint/src/render.rs:87`). Once every native finding carries a
`Diag`, "framed iff provenance" would flip every native finding to framed and churn 11
goldens. See `flag-lint-render-default-selection` (§8).

**Recommended shape (zero default churn, policy made explicit):** add a `Verbosity`
selection parameter to `render_human_parts` and a per-finding `frame: FrameChoice` field
(or a small `fn frames_by_default(finding) -> bool` policy fn colocated in `render.rs`)
whose DEFAULT reproduces today's split exactly — framed for findings minted from an
analysis/oracle-solo `Diag`, compact for the inventory/tool findings — so `--verbose`
promotes compact→framed wherever provenance exists and `--terse` demotes framed→compact.
Zero golden churn at default; the policy is now a named, changeable thing rather than an
emergent property of a struct field. Wire the flags at the cli edge
(`parse_lint_args`, `main.rs:648-712`) as ordinary lint flags, and note that they become
two more phase-4 arg-error producers if given bad values (they take no values — prefer
bare `--terse`/`--verbose`).

Machine envelope: `288` §5 licenses free reshaping. Keep `JSONL_FORMAT` = the same name
(`lint/src/render.rs:16`) unless a field is REMOVED; the envelope is additive-only by its
own policy (`27R` §8), and pre-user renames are free but pointless here. CI gates on
codes/severity, never finding-set identity — no change needed, but do NOT add
finding-count assertions anywhere.

### §4d — `rider-diag-tidy-scan-set` (`289:rider-diag-tidy-scan-set`, phase 3 seat)

Widen `SCANNED_CRATES` (`aid/tests/diag_tidy.rs:240-242`) to include `"lint"` and
`"dorc-loom"`. This must land BEFORE the lint emit sites move, or g6
(`every_catalog_variant_is_constructed`) cannot see them and the new codes read as dead
catalog. Expect the widening alone to be green at base (nothing in lint/dorc-loom
constructs a `DiagCode` payload literally today — `fire_lint_case` reads slugs, it does
not construct); if it is NOT green, that is a finding worth reporting, not a reason to
narrow the set back.

### §4e — `step-license-plane-contact-pin` (`27U:finding-emission-would-vouch`)

The named license-adjacent hazard class for this lane is EMISSION-VOUCHING: an aid-plane
emission minting an elision license. The historical instance —
a `27W` decline-emission `printf` with no trailing `return 2` exiting 0 and VOUCHING — was
fixed by making recognized sink-emissions inert in the tracer
(`oracle/src/predict/ast.rs:247-250`, `parser.rs:1711`) and by making emission-only bodies
decline ⇒ run. Its regression pins are
`crates/oracle/src/verdict.rs:688` (`emission_only_body_declines_never_vouches`) and
`:704` (`canonical_emission_then_return_two_declines_with_arm_captured`).

Phases 2–4 add no new sh-side emission shape, so the hazard here is the SHAPE-ANALOGUE:
`dorc-lint` gaining `DiagCode`s makes it look, structurally, like a decision-plane
participant. Design the pin as follows:

1. **Keep the two named oracle tests green and CITE them by name in the lane's
   verification report** — a lane that touches aid-plane emission must show it did not
   move the vouching boundary.
2. **Add one new tidy-style pin** in `lint` (a `crates/lint/tests/no_license_plane_contact.rs`,
   or an inline `#[cfg(test)]` scan): assert that no `.rs` under `crates/lint/src`
   contains any of `ByVouch`, `ByObservation`, `BySilence`, `claim::`, `Grade::Must`,
   `mint_from_room`, or `RoomFact`. This is the mechanical form of
   `dir-no-license-plane-contact`, which today is a DOC CLAIM ONLY
   (`lint/src/finding.rs:4`, `lint/src/lib.rs:11`, `lint/src/source_oracle_solo.rs:12`)
   with no gate behind it. `dorc-lint` deps `dorc-core`, so `claim` is reachable — the
   claim is currently unenforced. Cheap, exact, and directly on the hazard.
3. Do NOT add a `lint → aid`-direction assertion: `lint` deps `dorc-aid` legitimately and
   must (`Severity`, `Diag`, `render_cli_parts`). The one-way law is aid←license, not
   crate-graph shape.

---

## §5 — PHASE 4: CLI-error migration (`288` §6)

### §5a — `step-worldless-invocation-route` (the least-specified leg — design it FIRST)

The named precondition, restated precisely from §2d: `world_of` already falls through to
`canonical_payload` when a case carries no `book.sh`/`*oracle.sh` section, so the WORLD
half is solved. What is missing is REPLAY RECOGNITION: `replay()` and
`render_direct_replay` do not recognize an invocation-error command, so a
`$ dorc strip` case can neither be driven nor regenerated.

Two options; the executor builds **W2** unless the conductor rules otherwise
(`flag-worldless-route-honest-vs-payload`, §8).

**W1 — world-as-payload floor.** Add a `parse_direct_invocation(words)` recognizer that
matches any `dorc …`/`dorc-sh …` command that is NOT one of the four existing shapes,
and render `canonical_payload(slug)` for the case's declared slug. Cheap (~40 lines, no
crate changes). Dishonest: the replay COMMAND is decorative — nothing binds the argv the
case shows to the code it declares, so a case's command can drift from its code forever.
Given ~28 codes, that is 28 decorative cases on the project's primary review surface
(`288:rul-errors-human-authored-review-surface`).

**W2 — honest trigger via a real parse (RECOMMENDED).** Mirror `fire_lint_case` exactly:
run the REAL argument parser over the case's replay argv and refuse unless the declared
slug fires.

1. **Add a lib target to `dorc-cli`.** `crates/cli/Cargo.toml` gains
   `[lib] name = "dorc_cli" path = "src/lib.rs"`; the two `[[bin]]`s stay. `src/lib.rs`
   holds ONLY the pure invocation surface, moved verbatim from `main.rs`:
   `USAGE`/`LINT_USAGE`, `Invocation`, `Args`, `LintArgs`, `Mode`, `LintFormat`,
   `parse_args_from` (`:322-514`), `parse_lint_args` (`:648-712`), `parse_lint_format`,
   `parse_fail_on`, `parse_expect_count`, `nearest` (`:518`), `levenshtein` (`:533`),
   `humane_read_error` (`:570`). `main.rs` keeps `parse_args()` (the `std::env::args`
   edge — I/O stays at the edge, `io-at-edges-only`) and `use dorc_cli::…`. Fields on
   `Args`/`LintArgs` become `pub`; the crate-root `#![expect(missing_docs, …)]` posture
   in `main.rs` does not carry to a new lib, so either doc every pub field or add a
   scoped `#![expect(missing_docs, reason = …)]` to `lib.rs` (prefer docs — most fields
   already carry them).
   ~420 lines moved, zero logic change.
2. **Change the error type.** `parse_args_from` / `parse_lint_args` return
   `Result<Invocation, Diag>` (not `String`). Each producer becomes
   `Diag::new_spanless_site(DiagCode::<Payload>( … ))` — these errors have no source and
   no span. Every new payload therefore joins `SPANLESS_SITE_PAYLOADS`
   (`aid/tests/diag_tidy.rs:212-236`), and note g7 scans `production_emit_source()`
   which INCLUDES `cli`, so the list must be exact in both directions.
3. **Change the print seats.** `main.rs:201/212/217/838/845/852` become
   `eprintln!("{}", dorc_aid::diag::render_body(&d, &interner))` (body-only keeps the
   terse one-line shape the CLI wants; `render_cli_parts` would frame a span that does
   not exist). Exit codes are UNCHANGED — `EXIT_USAGE`/`EXIT_LINT_OPERATIONAL` stay
   exactly where they are; severity is registry data and never decides an exit code here.
4. **`dorc-loom` gains a `dorc-cli` dep** (no cycle — §2d) and a
   `fire_invocation_error(slug, argv) -> Result<(Diag, String, String), String>` arm
   modelled on `fire_lint_case` (`consumer.rs:887-914`): tokenize the replay command with
   the existing `exact_words` (`:453`), drop the leading `dorc`/`dorc-sh` word, call
   `dorc_cli::parse_args_from`, and REFUSE unless the returned `Diag`'s slug equals the
   case's declared `code`. Wire it into `world_of` (before the `canonical_payload`
   fallback) and into `replay`/`render_direct_replay` behind a
   `parse_direct_invocation` recognizer. Source and filename are `""` (spanless), exactly
   as the `canonical_payload` arm returns them (`:377`).

`rule-worldless-route-refuses-on-mismatch`: the refusal on slug mismatch is the whole
value — without it W2 degenerates into W1 with extra steps.

`dorc-sh`'s three errors (`rul-dorc-sh-not-carved-out`): `dorc-sh.rs` is a bin in the same
package, so it can `use dorc_cli::…` and `dorc_aid::…` freely (the package already deps
`dorc-aid`). Its usage/read/exec errors become spanless codes rendered with
`render_body`, keeping the terse `dorc-sh: ` framing at the print seat (surface selection,
not a carve — `288` §6). The seam note stands and changes nothing now: if `dorc-sh` ever
ships host-side, host-side emissions likely stay raw-bytes-upstream with controller-side
narration.

### §5b — `step-invocation-slug-cut` (`288:rul-error-slugs-are-semantic`)

Semantic-first; no file-count target in either direction. The load-bearing law is
`AID-NEEDS:law-codes-vary-by-world-not-grammar`: **siblings come from world-state/license
variants ONLY, never grammar-fit.** Applied to §2f's inventory that gives a clean cut:

- GRAMMAR-fit ⇒ ONE code, parameterized. The twelve flag-needs-a-value producers
  (`:408 :410 :416 :420 :445 :453 :665 :667 :673 :677 :686 :693`) are one semantic
  thing — "a flag that takes a value, given without one" — differing only in WHICH flag.
  One code, `{flag}` param.
- WORLD-state ⇒ SIBLING codes. `humane_read_error` (`:570-577`) branches on
  `io::ErrorKind`: `NotFound` / `PermissionDenied` / other. Those are three different
  states of the world with three different remediations ⇒ THREE sibling codes, not one
  `{why}`-parameterized code. The `{kind}` label ("source"/"book"/"oracle
  directory"/"file") stays a param.

Recommended cut (~13 codes; re-cut freely):

`cli-flag-needs-value` · `cli-unknown-flag` (with an optional `{suggestion}`; keep the
did-you-mean/bare split as ONE code with an optional param, not two — the difference is
grammar-fit) · `cli-unknown-mode` · `cli-flag-value-not-recognized` (the
`--probe-capability`/`--format`/`--fail-on` trio: params `{flag}`, `{got}`, `{expected}`)
· `cli-flag-value-not-a-number` (`--expect-files`) · `cli-no-book-given` ·
`cli-strip-needs-path` (params carry the got-a-flag variant) · `cli-flags-mutually-exclusive`
· `cli-flag-requires-mode` (`--whylog` only with `why --last`; also covers `:1746`) ·
`cli-file-not-found` / `cli-file-permission-denied` / `cli-file-unreadable` (the world-state
triple) · `lint-no-lintable-files` · `lint-file-count-drift` ·
`lint-required-tools-missing` · `dorc-sh-usage` · `dorc-sh-script-unreadable` (or reuse
the file triple) · `dorc-sh-exec-failed`.

`rule-cli-codes-reuse-existing-registry-shapes`: `registry()` arms for all of these are
`Severity::Error` + `Floor::None` + `RemediationClass::…` — an invocation error is
always the user's to fix, never a Dorc-modeling limitation, so `Structural` is wrong for
all of them. Pick between the existing classes at `diag.rs:1135-1143`; do not add a new
`RemediationClass` variant (that widens `remediation_hint`, which `288` §6 PARKS).

### §5c — `step-cli-cases-ship-covered`

Every code above ships a covered defining case via `dorc-loom scaffold`, with
`[unwritten:]` or `sm `-migrated prose per `flag-sm-migration-vs-unwritten` (§8). Under
W2 each case's replay is honest: `$ dorc plan --bogus` really fires
`cli-unknown-flag`. Expect ~16-19 new `.loom` files — the largest single addition to the
corpus so far; keep them minimal (7-8 lines each, the `syntax-unsupported.loom` shape).

### §5d — Explicitly OUT of scope (do not map, do not build)

Help/version text (phase 7 pilot, `rul-help-text-is-loomable`) · arrangement/chrome
(phase 7: lint render arrangement slugs, CLI chrome, plan-render annotations) ·
`remediation_hint` class-prose (`diag.rs:2234`, parked to the prose-register sitting) ·
anything the `284` opaque lane owns (passthrough/taint, the `detail`-param codes).

---

## §6 — Cheap riders (attached where cheapest)

### §6a — `289:rider-dead-diagcode-link` — FIRST COMMIT, one word

`crates/aid/src/diag.rs:243` doc-comment links `[`crate::DiagCode`]`; that legacy
string-slug type is gone (`one-catalog-no-legacy`), so the intra-doc link is dead
(`rustdoc::broken_intra_doc_links = warn`, workspace lints). De-link to backticked plain
text. Rides the lane's first commit per `289` §2e.

### §6b — `28A` §2u — the `covered() ⊆ case-owned` one-test drift guard

`covered()` (`aid/tests/catalog_defining_cases.rs:49-254`) and the loom's
`canonical_payload` (`consumer.rs:818-881`) duplicate constructors by design
(`28A:rul-keep-covered-with-drift-guard`). The unclosed gap: completeness (g12) does not
force `covered()`-slugs ⊆ case-owned. Add ONE test in
`catalog_defining_cases.rs` asserting every `covered()` slug satisfies `is_case_owned`
(`:456`). Three lines. Attach to phase 2 (it is a catalog-gate change and belongs beside
§3b's message edits).

### §6c — `28A:finding-touches-rename-half-done` — **DROP FROM THIS LANE, ESCALATE**

The brief anticipated doc + fixture residue. It is not. **This is a live production
funcname mismatch in the derivation-probe lane.** Verified:

- `dorc_oracle::predict::strip_touches` emits the funcdef with the suffix `__disturbs`
  (`crates/oracle/src/predict.rs:115-118`: `strip_role(src, touches, interner,
  "__disturbs")`).
- `dorc_plan::touches_fn_name` builds the INVOCATION name as `{}__touches`
  (`crates/plan/src/lib.rs:2076-2082`), and `DerivationPlan::render_sh` emits that def and
  that invocation into the same probe block (`plan/src/lib.rs:2093-2110`).
- Its own doc-comment claims they "agree byte-for-byte" (`plan/src/lib.rs:2073-2075`).
  They do not.
- The committed golden proves it:
  `spike/e2e/cases/strawman24-derived-survive/expected.out:31` emits
  `apt_get__disturbs() {` while `:41` and `:43` invoke `apt_get__touches 'install' …`.

Verified in other cells before claiming (per AGENTS.md's corollary):
- **Fail direction: SAFE.** An undefined function yields rc 127, no coord lines, and an
  empty derived footprint; `Footprint::authored(…, vec![])` is `None` — "an empty emission
  is no claim ⇒ no footprint ⇒ wall" (`plan/src/survival.rs:1063-1085`, and the anti-233
  companion at `:1072`). So the failure collides/demotes toward RUN. Not a wrong-elision.
- **Why it is green:** the case's `mocks/` dir carries `dpkg` and `sed`, and its
  `expected.ran` records only `apt-get install -y oldpkg` — neither `dpkg` nor `sed` ever
  runs, i.e. the committed expectation already encodes the dead lane. The two unit
  fixtures that mention the old name hand-BUILD the `DerivationShip.sh` string
  (`plan/src/lib.rs:4965` + its assertion `:4983`; `cli/src/main.rs:6216`), so they
  supply both sides and can never catch the mismatch.
- `strip_reaches` (`predict.rs:150`, suffix `__disturbance_reaches_only`) has NO
  production caller; the `<kind>__reaches_<n>` per-arm wrapper (`cli/src/main.rs:2942-3010`)
  is cli-synthesized on both sides and is NOT affected.

Disposition: **drop from this lane and flag** (`flag-touches-funcname-mismatch-is-a-bug`,
§8). The fix is one word plus a real regression pin (a test that strips a touches body and
asserts the emitted invocation name equals the emitted def name — the invariant
`touches_fn_name`'s doc asserts and nothing checks), two hand-built fixture respellings,
one golden re-bless, and the `predict.rs:105-113` / `plan/src/lib.rs:1903,2073` doc
respell. It is a behaviour change to emitted probe bytes in a lane this map does not
otherwise touch; folding it into a three-phase checkpointed lane would muddy the one diff
the conductor reviews.

### §6d — steering-sync (`288` §9), phases 3 and 4 only

Phase 3: remove the lint-namespace caveats from `AID-NEEDS.md` (the
`aid-lint-*` rows' `mech` column and the `dorc lint` surface bullet still describe a
lane-local namespace). Phase 4: add the CLI-error rows to `AID-NEEDS.md`. Both are
LLM-maintained root docs (edit-and-commit for in-place human review), conductor-tier
edits — the executor DRAFTS them in the lane and the conductor lands them.

### §6e — the `Unexplained` steering correction

`spike/CLAUDE.md`'s `collapse-mints-narrative` bullet and `AID-NEEDS.md`'s
`law-collapse-mints-narrative` both imply an `Unexplained` that renders
self-advertisingly. No such thing exists (§2c). Correct both to describe the as-built
posture (minted-but-mostly-unrendered; the render is d4/phase-7 work) rather than
deleting the aspiration. Conductor-tier edit; draft in the lane.

---

## §7 — Commit sequencing, tests, and gates

### §7a — Commit sequence (19 commits; `.gitlabels` style, `AI` mandatory, no filenames/slugs in messages)

Phase 2 (7):
1. `(AI doc -)` de-link the dead `DiagCode` doc reference (§6a).
2. `(AI re aid)` union case-derived rows into the generated lock (§3a) — lock bytes
   unchanged; g16 green proves it.
3. `(AI feat aid)` add the explicit case-scaffolding command (§3c).
4. `(AI doc test)` name the repair command in both completeness failures (§3b) + the
   `covered()` drift guard (§6b).
5. `(AI test aid)` the collapse-class completeness gate + census (h1, h2).
6. `(AI test ana)` promote the merge-mint pairing to a release-mode parity gate (h3);
   `(AI test)` per-class fault-injection pins (h4a) + the one renderable class (h4b).
7. `(AI doc -)` the narrative prose sweep (§3f).
   Then the walkthrough (§3d) lands with phase 4's first code, or as its own
   `(AI new aid)` commit if the conductor rules W1.

Phase 3 (5):
8. `(AI test -)` widen the tidy scan set (§4d).
9. `(AI feat ana aid)` mint the native lint codes + payloads + registry/params arms.
10. `(AI re aid)` route the lint sources through the registry codes; migrate prose.
11. `(AI feat)` the lint render selection policy + `--terse`/`--verbose` (§4c).
12. `(AI test)` the license-plane-contact pin (§4e) + the new lint defining cases.

Phase 4 (7):
13. `(AI re !!)` extract the invocation-arg surface into a `dorc-cli` lib target
    (pure move, zero logic change — the diff must read as a move).
14. `(AI feat aid)` mint the invocation-error codes + payloads + registry/params arms +
    the spanless allow-list rows.
15. `(AI re)` return typed diagnostics from the argument parsers; re-seat the prints.
16. `(AI re)` the same for `dorc-sh`'s three errors.
17. `(AI feat)` the loom's honest-trigger invocation route (§5a step 4).
18. `(AI new)` the invocation-error defining cases (scaffolded, authored, blessed).
19. `(AI doc dsn)` the registry/steering drafts (§6d, §6e).

Conductor-only steps interleave: `dorc-loom promote` runs (never a builder action —
`287` §5) after commits 2/9/14, on a freshly-verified binary, diff inspected row by row.

### §7b — Verification plan

Per commit: `cargo fmt --check` · the touched packages' tests · `cargo clippy -p <pkg>
--all-targets -- -D warnings`.

Per phase close (foreground): `mise exec -- cargo build --workspace` ·
`mise exec -- cargo test --workspace` · `sh e2e/run.sh` (94 cases + 3 lint-cases;
count the dirs, never trust a literal) · `cargo fmt --check` ·
COLD `cargo clippy --workspace --all-targets -- -D warnings` (the
`28A:finding-incremental-clippy-serves-stale` rider: incremental clippy serves stale
results) · `cargo deny check licenses bans sources` · `mise x -- typos spike`.

Lane close: all of the above foreground, plus the two fixpoint gates
(`dorc-loom/tests/fixpoint.rs::generated_lock_reproduces_the_committed_bytes` and
`::direct_plan_render_fixpoint`), plus an explicit report naming
`emission_only_body_declines_never_vouches` and
`canonical_emission_then_return_two_declines_with_arm_captured` as green (§4e).

`rule-final-verification-runs-foreground` (`27U:foreground-final-verification`): ending a
turn awaiting your own backgrounded task strands the lane.

`rule-bless-is-conductor-only`: `BLESS=1` is exclusive and orchestrator-only
(`spike/CLAUDE.md`). Where a phase needs a golden re-bless, the executor HAND-EDITS the
expectation and says so; `e2e/lint-cases/*/expected.out` is hand-authored and NEVER
blessed by construction (`e2e/run.sh:1418-1425`).

### §7c — Comment budget (`24P` §8 rider)

This lane churns tests and fixtures heavily. Hard budget: **≤ 45 net new comment lines
across all 19 commits**, counted as
`git diff <base>..HEAD -- '*.rs' | grep -c '^+\s*//'`. Rip-don't-update: a comment
describing behaviour that changed gets DELETED, not rewritten, unless it explains a WHY
that survives. Structural banners in the new census/allow-list data tables are noted
separately and not billed. The executor must run the counting command and report the
number before ending its turn.

---

## §8 — FLAGS (flagged UP; NOT resolved here)

Ruling needed BEFORE the executor starts:

1. **`flag-item-four-has-no-render`** (§2c, §3e-h4) — `289:rul-mint-hardening-package`
   item 4 says "force the seam, assert the narrative minted AND the chain renders it", and
   is hard-mandatory because the as-built behaviour is silent-omit (there is no
   `Unexplained`). But eight of nine collapse classes have NO render consumer at all: the
   why-chain is built from `SurvivalWitness`, `emit_why_lens` takes the narrative slice as
   `_collapse_narrative` and ignores it, and only `VerdictDecline`+`authored_reason`
   reaches a `why:` line. Options: (a) build h4a for all nine + h4b for the one renderable
   class, record the gap (RECOMMENDED — it is the honest 60% the human's lean asks for);
   (b) build a narrative render to make h4b satisfiable — that is the d4 arrangement
   walker, deliberately deferred, and welds output phase 7 owns; (c) reduce item 4 to
   mint-assertions only. My read: (a).

2. **`flag-lint-render-default-selection`** (§4c) — does the phase-3 default preserve
   today's provenance-keyed split byte-for-byte (framed for diag-backed findings, compact
   for the rest — zero churn, policy made explicit), or does `288` §5's "default keeps the
   compact line-per-finding shape" mean uniformly compact with `--verbose` adding frames
   (hand-editing 3 `e2e/lint-cases/*/expected.out` + 8 `.loom` cases, and losing caret
   frames from default lint output)? This is a `tc-*`-shaped UX judgment. Recommendation:
   preserve today's split; make it an explicit named policy.

3. **`flag-sm-migration-vs-unwritten`** (§4b, §5c) — the brief says phase-3/4 codes ship
   with `[unwritten:]` prose. Taken literally, three constantly-firing lint findings and
   ~16 CLI errors render as `[unwritten: <slug>]` placeholders for the rest of the arc
   (phase 8 is HELD for the human). The alternative is `sm `-migration: the existing
   hand-written text moves VERBATIM with the `sm ` prefix, which is exactly what `sm `
   means ("prior-builder prose migrated verbatim from the base tip") and what the
   currently-rendered lint goldens already show. Recommendation: `sm `-migrate wherever
   verbatim ancestor text exists; `[unwritten:]` ONLY for genuinely new codes (e.g. the
   `NotFound`/`PermissionDenied` sibling split, where one sibling has no ancestor
   sentence). Either way builders author ZERO new prose — this is about migration, not
   authorship.

Ruleable at the mid-lane checkpoint:

4. **`flag-worldless-route-honest-vs-payload`** (§5a) — W2 (honest trigger; needs a
   `dorc-cli` lib target, ~420 lines moved verbatim, and a `dorc-loom → dorc-cli` dep with
   no cycle) vs W1 (world-as-payload floor; ~40 lines, but 28 decorative cases on the
   project's primary review surface). Recommendation: W2. The lib extraction also makes
   the argument parsers unit-testable for the first time.

5. **`flag-external-text-is-a-relay-not-a-code`** (§4a) — `external-text` is the tolerant
   adapter's placeholder on findings whose content is a foreign tool's own message. Treat
   it as a relay (leave it a bare string) rather than minting a registry code for it?
   Recommendation: relay.

6. **`flag-narrative-identifier-scope`** (§3f) — does `289:rider-narrative-prose-sweep`
   cover identifiers (`evidence` locals/params/test-names) in narrative-plane code, or
   doc-comment prose only? Recommendation: sweep the one parameter name and the two test
   names, leave short-lived test locals.

7. **`flag-mirror-union-source-is-cases`** (§3a) — this map takes case-derived union rows
   where `288` §4 says "enum-derived". Low stakes; the orphan-row direction of g8 keeps it
   self-cleaning. Confirm or override.

8. **`flag-touches-funcname-mismatch-is-a-bug`** (§6c) — NOT a rename residue: production
   ships `apt_get__disturbs()` and invokes `apt_get__touches`, proven by the committed
   golden at `e2e/cases/strawman24-derived-survive/expected.out:31` vs `:41`/`:43`. Fails
   SAFE (empty footprint ⇒ wall ⇒ run) and is masked by hand-built fixtures. Dropped from
   this lane; wants its own small dispatch with a real def↔invocation regression pin.

9. **`flag-cli-lib-widens-a-bin-only-crate`** (§5a) — giving `dorc-cli` a lib target makes
   ~15 previously-private types public API of a `publish = false` crate. Consistent with
   `scope-boundary` ("keep the binary a thin driver")? My read: yes — moving the parse
   surface OUT of the 7000-line `main.rs` serves that law rather than straining it. Worth
   an explicit nod.

10. **`flag-unwritten-ceiling-becomes-noise`** (§2b g14) — the numeric ceiling
    (`unwritten_renders_are_greppable_and_pinned`, currently 6/6) is bumped once per mint
    for the rest of the arc, which turns "a conscious conductor act" into a per-commit
    chore. Consider re-keying the gate to the meaningful invariant post-mint-seam ("every
    unwritten code is case-owned") and keeping the number as a recorded, not-asserted,
    count. Not blocking; the executor bumps as specified unless ruled.

---

## §9 — Executor sizing

**One builder, ONE mid-lane checkpoint** (recommended), not three separate dispatches and
not a single unchecked run.

Reasoning. Phase 2 is small and self-contained (~7 commits, no cross-crate surface
change) and phase 3 is medium (~5 commits, confined to `lint` + `aid`). Phase 4 is the
expensive one: a ~420-line verbatim extraction, an error-type change threading through
~30 producers and 6 print seats, a new inter-crate dependency, and ~16-19 new case files.
Its risk is concentrated in ONE decision (`flag-worldless-route-honest-vs-payload`) that
the conductor should see resolved-in-practice before ~19 cases get authored against it.

Recommended checkpoint: **after commit 12 (phase 3 close)**, before the phase-4 lib
extraction. At that point the mint seam has been dogfooded once, the lint codes prove the
end-to-end recipe, and the conductor can rule flags 4/5 with real diffs in hand rather
than in the abstract. A second, cheaper checkpoint after commit 13 (the pure move) is
worth taking if the extraction's diff does not read as a pure move.

Against three dispatches: phases 2 and 3 share the whole g1–g17 checklist and the same
`dorc-loom promote` loop; re-onboarding a fresh builder onto that costs more than it
saves. Against zero checkpoints: phase 4's ~19 authored cases are expensive to redo if
the worldless route is ruled the other way.

Budget shape (-GUESS): phase 2 ≈ 25%, phase 3 ≈ 25%, phase 4 ≈ 50% of the lane.

---

## §10 — Confidence

**+SURE** (verified in-tree at base, with file:line): the mint-path closure (§2a); every
gate in §2b including g14 sitting exactly at its ceiling and g15's scaffold-and-forget
property; the absence of `Unexplained` anywhere in the tree and the fact that the
why-chain reads no narratives (§2c); the nine mint sites; the `dorc: ` prefix having zero
golden coupling (§2f); the touches/disturbs production mismatch and its fail-safe
direction (§6c); `dorc-cli` having no lib target and `dorc-loom → dorc-cli` introducing no
cycle (§2d); the exact evidence-prose enumeration and the fact that `aid/src/diag.rs` is
entirely r29-fenced (§3f).

**~SUSPECT**: that the W2 lib extraction is a genuinely mechanical ~420-line move — I read
the parse surface end-to-end but did not attempt the move, and `Args`/`LintArgs` are
consumed widely inside `main.rs`, so field-visibility churn may exceed the estimate; that
widening `SCANNED_CRATES` to lint+dorc-loom is green at base (nothing constructs a payload
literally there today, but the scan also picks up `#[cfg(test)]` code I did not read
exhaustively); that the recommended slug cut in §5b survives contact — the
grammar-vs-world-state razor is clean, but the individual cuts are taste.

**-GUESS**: the 19-commit count and the phase-weight split in §9; that `sm `-migration
(flag 3) is what the human wants — it follows from the `sm ` definition and from the
current lint goldens, but it is an inference, not a ruling.

**--WONDER**: whether the narrative plane's near-total lack of render consumers (§2c) is
itself the finding the hardening package was reaching for — h1/h2/h3 harden a mint
schedule whose product almost nothing reads, which is a strange place to spend
correctness effort. That is a question for the conductor about phase 7's arrangement-home
sitting, not something this lane should act on.
