# 283 — the generation-flip proposal (`282` phase 4; map half of the map-then-execute split)

AI-authored (Opus builder, round-28 `lane-errorloom-unify` dispatch 2, 2026-07-20).
MAP HALF ONLY (`27U:map-then-execute-split`): this note is a proposal + a mechanical
spec; the conductor rules; a fresh-budget executor runs it. NO engine edits were made
producing it. Authority: root docs, `spike/CLAUDE.md`, human-typed rulings outrank;
`plans/282` (§6/§8/§9-phase-4) is the plan-of-record this dispatch executes;
`plans/280` §3 is the lane; `notes/28A` §2b/§2g/§2l/§2m carry the binding riders.

Base: `ff8f55b` (branch `ai/r28-flip-map`). Code-grounded against the as-built at that
tip: `core/src/catalog.rs` (const table + promote v1 + the three-state prose gates),
`core/src/diag.rs` (`render_body`/`render_body_tagged`/`render_cli`/`params_of`),
`core/src/tagged.rs`, `core/tests/catalog_defining_cases.rs`, `core/tests/diag_tidy.rs`,
`crates/errorloom/` (README + `src/bless.rs` + `container.rs` + `runner.rs` +
`tests/toy_consumer.rs`), `crates/dorc-loom/src/lib.rs`.

Confidence grades throughout: `+SURE` / `~SUSPECT` / `-GUESS` / `--WONDER`.

---

## §0 — Orienting facts the executor must hold (as-built, code-verified)

- **f-errorloom-is-complete** (`+SURE`): the generic engine is DONE and folded
  (`28A` §2g). `errorloom::bless` already ships the whole loop: the four-method
  `Consumer` trait (`tagged_render` · `editable_text` · `apply_field_edits` ·
  `render_case`), the two-method `Git` trait with `SubprocessGit`+`FakeGit`,
  `infer_mode`, `prose_bless`, `structure_bless`, `fixpoint_check`. The `toy_consumer`
  test drives the FULL prose-bless → catalog-mutate → re-render → fixpoint loop
  dorc-free. Phase 4 writes a Dorc `Consumer`; it does NOT re-implement orchestration.
- **f-dorc-loom-is-a-span-mapper-only** (`+SURE`): `dorc-loom` today is ONLY
  `to_errorloom()` — it maps `core::tagged::Span` → `errorloom::Span`. It does NOT yet
  implement `Consumer`. That impl is the heart of phase 4 and lands here.
- **f-render-seat-is-render-body** (`+SURE`, corrects my brief's stale pointer — the
  same staleness `28A` §2m flagged): the prose fill seat is
  `catalog::fill_template` / `fill_template_tagged`, reached through
  `diag::render_body` / `render_body_tagged`; the full CLI diagnostic is
  `diag::render_cli` (title-split + `frame_region` caret frames + body tail). The
  aid-chain CLI walker is a SEPARATE prose surface, untouched by this dispatch.
- **f-promote-v1-carries-prose** (`+SURE`): `catalog::promote_catalog_source()` is
  codegen-to-source that reads the compiled-in `CATALOG` const and re-emits it,
  carrying `message`/`help`/`when_fires`/`why` VERBATIM and regenerating `params`
  (=holes-of-prose) + `example` (=schematic render). It has NO path that WRITES prose.
  Promote-v2's whole job is to add that path (from an edited transcript). Gated
  runner: the `DORC_CATALOG_PROMOTE=1` test writes `target/catalog-promoted.rs`.
- **f-catalog-counts** (`+SURE`): 56 catalog codes total. `covered()` = 21 codes with
  fragment-golden defining cases (63 files: `.machine`/`.terse`/`.prose`).
  `DEFINING_CASE_RATCHET` = 35 codes. 21 + 35 = 56. Five codes render
  `[unwritten:]` (the four `281` mark codes + would-be new mints); five carry
  unprefixed conductor-authored prose (`whylog-*` ×4 + `aid-unloaded-sibling-oracle`),
  gated by the `CONDUCTOR_AUTHORED` roster; the rest are `sm `-prose.
- **f-fragment-count-is-21-not-17** (`+SURE`, FLAG): `282` §8/§9 and my brief say "17
  unit-tier fragment goldens." At `ff8f55b` it is **21 covered codes / 63 fragment
  files** — the syntax lane added the four `281` mark codes as `covered()` with
  `[unwritten:]` prose (`28A` §2b). "35 case-less" is still exact. Every "17" in the
  ladder below is really 21; sizing uses 21.

---

## §1 — The case corpus: home, format, world materialization, the new codes

### 1a — Home

**`283:dec-cases-live-under-core-tests`** (`~SUSPECT`, recommend, flag the tier split):
site the Dorc corpus at `spike/crates/core/tests/cases/<slug>.txt`, beside the
retiring `tests/defining_cases/`. Rationale: the fixpoint gates (§4) that consume the
corpus are `core` tests, hermetic and DST-clean (`283:dec-fixpoint-is-in-process`
below); a case's render is reconstructed in-process, not by shelling to a built
binary, so the corpus belongs in the crate whose kernel produces the diagnostics.
The e2e corpus stays where it is (`e2e/`, a different product surface — the PLAN
render, `282` §8 "what stays"). FLAG: if the conductor rules that phase-4 cases must
REALLY fire through a subprocess `dorc` (the `282` §7 runner, real-firing), then the
runner-tier cases want `e2e/` instead; see `283:flag-in-process-vs-real-firing`.

### 1b — Format (txtar + flat-YAML frontmatter — errorloom's `Case`, verbatim)

The container is already built and generic (`errorloom::Case::parse` / `to_text` /
`frontmatter().scalar` / `replay().blocks()` / `set_replay_outputs` /
`materialized_files` / `check_hygiene`). Dorc reuses it unchanged. Format per `282` §2:

```
---
code: render-heredoc-refused
when-fires: the leaf-exact render would elide/guard a heredoc-bearing leaf
why: kFAIL-perform; arch-1 d-6
---
-- world --
slug: render-heredoc-refused
site: 7
verb: elide
command: cat <<EOF
source: |
  cat <<EOF >/etc/motd
  hello
  EOF
-- replay --
$ dorc plan --book=book.sh < probe-results.txt
error[render-heredoc-refused]: leaf-exact render refuses to elide a heredoc-bearing …
  --> book.sh:1:1
   |
 1 | cat <<EOF >/etc/motd
   | ^^^^^^^^^ …
  = help: sm split the heredoc body to its own leaf, or mark the kind un-elidable
```

- **Frontmatter key set** (`282` §2, `28A` §1 "frontmatter is opaque to errorloom;
  the schema belongs to the consumer"): `code` (the defining slug) · `when-fires` ·
  `why`. FLAT scalars only (`282:lean-flat-frontmatter-subset`). `params` are NEVER
  declared here — they derive from the prose's holes in code
  (`refreshed_params`, unchanged). **`283:dec-metadata-from-frontmatter`** (`~SUSPECT`):
  for a case-OWNED code, `when-fires`/`why` become DERIVED from this frontmatter (the
  §8 "metadata from frontmatter" clause), so a hand-edit to those fields in
  `catalog.rs` is caught by the Dorc-side fixpoint gate (§4). Case-LESS codes keep
  their const-authored `when-fires`/`why` (carry-forward; their metadata hand-edits
  are an accepted, shrinking migration gap).
- **The world section** (`283:dec-world-two-forms`, `~SUSPECT`, the biggest fork —
  see §6 flag-1): a Dorc case's world materializes in ONE of two legal forms, both
  feeding the SAME `render_cli_tagged`:
  - **world-as-payload** (the phase-4 floor): a `-- world --` section carrying the
    render inputs — the code slug (→ the compiled-in canonical constructor, keyed by
    slug, reusing `covered()`'s `build: fn() -> DiagCode` transitionally), plus the
    synthetic `source`/`site`/`span` the caret frame needs. `render_case` looks up the
    constructor by slug, builds the payload, renders `render_cli_tagged`. Works for
    EVERY code including the hard tail (`records-*`, `whylog-*`), needs no triggering
    book, stays hermetic. The "decorative book" is a sanctioned sharp edge
    (`282:rul-internal-tool-sharp-edges`). The world section is the case's "embedded
    world" in `282` §1's sense — the input, not the prose.
  - **world-as-pipeline** (the `282`-faithful upgrade, phase-5+): `book.sh` +
    `*.oracle.sh` + `probe-results.txt` sections; `render_case` drives the in-process
    kernel pipeline (`syntax → analysis → probe → plan`, pure, DST-clean) over the
    materialized-in-memory world and renders the REAL diagnostics it produces. Real
    triggering (honest — the diagnostic actually comes from the book+world); closes
    `27U:finding-corpus-blind-edge-codes` for that code. Cost: a triggering world per
    code (the `DEFINING_CASE_RATCHET` notes ARE these worlds); the hard tail stays
    world-as-payload or ratcheted.
  Recommendation: phase 4 uses world-as-payload for its pilots (§1d); phase 5 fans
  out, preferring world-as-pipeline where the world is cheap, world-as-payload for the
  expensive tail. The FORMAT supports both from day one; `render_case` dispatches on
  which sections are present.
- **The replay section** is always last (`282` §2): `$ dorc …` command lines +
  inlined output. For a phase-4 thin case it is ONE `$ dorc …` caption (Arrangement)
  over the full `render_cli` transcript. Multi-replay (`--verbose`/`--terse`/
  `--format=jsonl`) is `282:rul-multi-replay-per-case` — the machine block is
  whole-structural, never prose-diffed (§2 `editable_text`).
- **Hygiene** is errorloom-generic and already built (`check_hygiene`): CRLF refusal,
  txtar-marker-collision refusal, sandbox-abs-path refusal, and a required-token gate
  the consumer keys to `code` (every replay block must surface its own slug — the
  `282:rul-multi-replay-per-case` same-slug coherence gate).

### 1c — The four syntax-lane codes + `marker-version-unrecognized`

The four `281` codes (`mark-unknown-verb`, `mark-rc-arity-exceeded`,
`mark-standalone-rc-consumer`, `mark-hashcolon-malformed`) exist today as `covered()`
cases with `[unwritten:]` prose (`28A` §2b). Under the flip they become case FILES
(phase 5) whose transcript is the prose surface; the conductor authors real prose via
the empty loop (`282:rul-new-code-empty-loop`: builder minted the slug, the render
shows the loud `[unwritten:]` placeholder, the author writes the words looking at the
render). They are world-as-payload (parser-diagnostic codes; their "world" is the
malformed oracle snippet + the constructor).

**`marker-version-unrecognized`** (`28A` §2l rider, ceiling→6 pre-authorized) is a NEW
code minted through the empty loop — the FIRST born under the flip machinery, and the
natural phase-4 pilot (§1d). Executor work: mint the `DiagCode` variant + a `covered()`
constructor + the emit site in `oracle/marker.rs` distinguishing an unrecognized
`# dorc-lang/vX.Y` from a wholly-missing marker (today both fall to the loud generic
`MissingDialectMarker` — `28A` §2l) + a case file with `[unwritten:]` prose; the
conductor authors the prose from the render. Bump the `unwritten_renders_are_greppable`
ceiling 5→6. I discovered **no other rider codes strictly needed** (`~SUSPECT`): the
`28A` §2l "lax-order bless nit" is a bless ergonomics item, not a code.

### 1d — Phase-4 pilots (why not land empty)

`283:dec-phase4-lands-pilots` (`~SUSPECT`): the flip COULD land with zero cases
(carry-forward makes promote-v2 an identity — every code keeps const prose — so the
gates pass vacuously). That proves nothing on real Dorc render output. Land two pilots:
(i) **`marker-version-unrecognized`** exercises the whole new-code empty loop end to
end; (ii) the **five roster codes** (`whylog-*` + `aid-unloaded-sibling-oracle`) MUST
become case-owned for the roster to actually retire (§4, `283:dec-roster-mini-backport`).
So phase 4's mini-backport = those six, world-as-payload. The bulk 15 remaining
old-covered codes stay fragment-golden through phase 4 and backport in phase 5.

---

## §2 — The Consumer implementation (dorc-loom)

`283:dec-consumer-in-dorc-loom` (`+SURE`): the four-method `errorloom::Consumer` impl
lives in `dorc-loom` (it deps both `errorloom` and `dorc-core`; `core` stays
errorloom-free — kernel-dep-cleanliness, `28A` §1). Key `= FieldKey { code, field }`
already exists there. The consumer holds a MUTABLE catalog mirror (see §3
`283:dec-mirror-via-catalog-lookup`).

- **`tagged_render(case) → TaggedBaseline`** — the `282` §2m rider, and the dominant
  code chunk of this dispatch. It renders the case's defining diagnostic as the FULL
  `render_cli` transcript, tagged. Build a `render_cli_tagged` twin in `core::diag`
  (additive, `render_cli` untouched) composing:
  - title line `severity[slug]: <problem>` — `severity`/`[`/slug/`]: ` are
    `Arrangement`; `<problem>` is the FIRST LINE of `render_body_tagged`'s spans,
    RELOCATED to title position (offsets re-based);
  - the primary `frame_region` caret frame (`-->` locator, gutter, source line, `^^^`
    underline) — WHOLE as `Arrangement` (it is structure, not prose; `frame_region`
    is already one self-contained string);
  - each secondary `---` frame — `Arrangement`;
  - the body TAIL (help connective + help prose + any notes) — `render_body_tagged`'s
    POST-first-line spans, RELOCATED to after the frames (offsets re-based);
  - the `$ dorc …` command echo (replay caption) — `Arrangement`.
  **Sizing honesty** (`~SUSPECT`): the load-bearing complexity is the TITLE-SPLIT span
  relocation. `render_cli` splits `render_body`'s first line onto the title and puts
  the rest after the carets (`diag.rs:1699-1729`); the tagged twin must split
  `render_body_tagged`'s span vector at the first `\n`, re-base both halves into the
  composed output's byte offsets, and preserve `instance`/`ForeignText` classification
  and the gap-free total cover across the relocation. Multi-diag transcripts iterate
  this per diagnostic. This is ~the single biggest engine edit in phase 4; everything
  else is plumbing. The `dorc-loom::to_errorloom` mapper already exists to carry the
  resulting `core::tagged` spans onto errorloom's schema and validate the total cover.
  `TaggedBaseline` also needs `ParamTables` (errorloom re-holes param values out of the
  edited prose) — populate from `params_of` for the case's payload.
- **`editable_text(case) → String`** — return the on-disk bytes of the prose-editable
  replay block(s). Selection is consumer knowledge (`28A` §2g): the HUMAN `render_cli`
  block(s) are editable; any `--format=…` machine block is whole-structural, excluded.
  Phase-4 thin cases have one human block ⇒ `editable_text` = that block's `output()`
  (mirrors `toy_consumer`). FLAG the selection heuristic (command-line `--format=`
  sniff vs a per-block convention) as latitude; recommend the `--format=` sniff.
- **`apply_field_edits(edits) → ()`** — write the extracted `(code, field) →
  FieldTemplate` prose into the mutable mirror. `FieldTemplate` is errorloom's
  words-and-paragraphs model (`282` §3); flatten it to the mirror's `Option<String>`
  message/help (single-paragraph today, `28A` §2c). This is the ONLY prose-write path;
  it runs inside `errorloom::prose_bless`, orchestrator-only (§4 BLESS-law).
- **`render_case(case) → String`** — re-render the case's FULL transcript from CURRENT
  (post-edit) mirror state and return the case's `to_text()`. This is the
  structure-bless / fixpoint path (mirrors `toy_consumer::rendered_case_text`:
  reconstruct the render, `set_replay_outputs`, `to_text`). It renders through the SAME
  `render_cli_tagged` seat (text half), sourcing the entry from the mirror, so
  prose edits flow into the regenerated transcript. Dispatches on world-form
  (`283:dec-world-two-forms`): world-as-payload rebuilds the payload from the
  constructor; world-as-pipeline runs the in-process kernel.

---

## §3 — The catalog's generated form

**`283:dec-catalog-stays-generated-const`** (`+SURE`): `catalog.rs` STAYS a generated
Rust `const CATALOG: &[CatalogEntry]` table — NOT a build-input data file, NOT an
`include!` of a hand-rolled format. The compiler-is-the-parser law (`amendment-
catalog-fields-are-data`; no `build.rs`, no macros, `inv-no-unsafe`) forbids any
build-time parse of a data file; promote v1 is ALREADY codegen-to-this-source, and v2
keeps that shape. The only reader of `catalog.rs` is `rustc`; promote reads the
COMPILED-IN const, never the source text. So structurally the file is unchanged; what
changes is the PROVENANCE of the prose fields (promote-authored from cases, not
hand-migrated) plus two schema edits:

- **`283:dec-message-becomes-option`** (`~SUSPECT`): `CatalogEntry::message` changes
  `&'static str` → `Option<&'static str>`; `None` = unwritten. `[unwritten: <slug>]`
  STOPS being a stored string (`282` §8) — `render_message`/`render_body`/
  `render_body_tagged` synthesize `format!("[unwritten: {slug}]")` when `message` is
  `None` (a one-line change at each seat; `render_body_tagged` already has the
  `None`→`unwritten-placeholder` Arrangement arm). `help` is already `Option`. The
  five current stored `[unwritten:]` strings migrate to `None`. "Make illegal states
  unrepresentable" (code style) favours `Option` over an empty-string sentinel; FLAG
  the sub-choice if the executor finds `Option` threads awkwardly. The
  `unwritten_renders_are_greppable_and_pinned` test re-keys on `message.is_none()`.
- **The mutable mirror** (`283:dec-mirror-via-catalog-lookup`, `~SUSPECT`, the crux —
  and where the as-built promote tool's shape fights `282` §6, §6 flag-2): promote-v2
  must MUTATE prose (`apply_field_edits`), but `CatalogEntry` fields are `&'static str`
  — you cannot build an owned entry with runtime prose, and promote v1 never needed to
  (it only carried prose). So introduce an owned mirror + a lookup seam:
  - `core::catalog` gains an owned `OwnedEntry { slug, when_fires, why, message:
    Option<String>, help, … }`, a `CATALOG.to_owned()`, and generalizes
    `promote_catalog_source()` to `serialize(&[OwnedEntry]) -> String` (same codegen,
    owned input). Stays in `core`, errorloom-free, pure.
  - Parameterize the render seat by a `CatalogLookup` trait (`fn message(slug) ->
    Option<&str>` etc.) implemented by BOTH the `'static` const (a thin wrapper) and
    the owned mirror. `render_body`/`render_message`/`render_body_tagged`/
    `render_cli(_tagged)` take `&dyn CatalogLookup`; production passes the const
    wrapper (byte-identical, gate-pinned); promote passes the mutated mirror. This
    keeps ONE render seat (no duplicated `render_cli` in dorc-loom) and lets
    `render_case` render from post-edit state in-process, before any rebuild. This
    threading is additive and mechanical but touches every render call site — the
    widest blast-radius step (§5 step 1).
- **The promote-v2 pipeline** (`283:dec-promote-v2-composes-errorloom`, `+SURE`):
  extract → regenerate → re-render, is `errorloom::prose_bless` DRIVING the Dorc
  Consumer, then `core::catalog::serialize(mirror)` codegen, then the orchestrator
  splices + `cargo fmt` + rebuilds (exactly promote v1's write→splice→fmt ritual, now
  with a prose-carrying mirror). It LIVES: the generic loop in `errorloom::bless`
  (built); the Consumer + mirror + orchestration entry in `dorc-loom`; the serializer
  in `core::catalog`. The gated runner stays `DORC_CATALOG_PROMOTE=1`
  (orchestrator-only; BLESS-law; the builder builds it, never runs it).
- **`params`/`example` stay code-driven** (`+SURE`, unchanged): `refreshed_params` =
  the prose's holes (dedup, first-occurrence); `schematic_example` = the message
  filled with `<param>` placeholders. When prose changes from a case edit, both
  auto-refresh; no new machinery.
- **`sm ` survival** (`+SURE`): `sm `-prose stays stored verbatim (`Some("sm …")`) and
  renders as ordinary words at the transcript surface until an author rewrites it via a
  case edit. No special handling — `sm ` is words.

---

## §4 — Gates + the retirement map

### 4a — The two fixpoint gates (BOTH required — `28A` §2g rider)

- **errorloom render-level fixpoint** (`fixpoint_check(consumer, corpus)`, built): for
  each committed case, `render_case(case)` == committed bytes. Catches PROSE hand-edits
  to the catalog (a prose change moves the transcript render). Blind to catalog
  metadata (metadata is not in the transcript). Runs over the CASE corpus. Git-free.
- **the Dorc-side promote→catalog-byte-identical gate** (`28A` §2g rider,
  consumer-side): regenerate `catalog.rs` from its sources of truth (case frontmatter →
  `when-fires`/`why` and case transcript → prose for case-OWNED codes; the const
  carry-forward for case-LESS codes; `params`/`example` always regenerated) and assert
  BYTE-IDENTICAL to the committed `catalog.rs`. Catches METADATA hand-edits the
  render-level gate misses. This STRENGTHENS promote v1's `promote_is_a_prose_fixpoint`
  test into a full-block byte-identity CI gate. Case-less codes are self-consistent
  under carry-forward (their metadata cannot be caught) — an accepted, shrinking
  migration gap. Git-free.
- Together: `cases ←render→ catalog ←derive→ sources` are all pinned; a hand-edit
  anywhere trips one gate. THIS is what retires the `CONDUCTOR_AUTHORED` roster.

### 4b — Git mode-gates (`283:dec-gates-are-git-free`, `~SUSPECT` — clarifies §2g's
"SubprocessGit vs FakeGit in CI")

Neither CI fixpoint gate needs git. Git is ONLY needed by the interactive BLESS
(`infer_mode`/`prose_bless`/`structure_bless` classify the dirty touched-set). So:
- **`SubprocessGit`** rides the orchestrator-only bless (`DORC_CATALOG_PROMOTE`), never
  CI. `errorloom`'s `subprocess_git.rs` already skips hermetically when git is absent.
- **`FakeGit`** drives the dorc-loom Consumer-loop UNIT tests (mirroring
  `toy_consumer`: prose-bless, both-classes-refuse, dirty-catalog-refuse,
  structure-drift, fixpoint-catches-hand-edit) — hermetic, no subprocess.
- The CI GATES themselves are pure regeneration/re-render checks, git-free.

### 4c — Retirement map (WHAT retires, WHEN — phase 4 is additive; goldens retire in 5)

- **`CONDUCTOR_AUTHORED` roster** → retires in PHASE 4 (§6/§8). Enforcement moves to
  promote-privilege (BLESS-law) + the two fixpoint gates. PRECONDITION
  (`283:dec-roster-mini-backport`, `~SUSPECT`, §6 flag-3): the five roster codes carry
  UNPREFIXED prose that is neither `sm ` nor `[unwritten:]`; the
  `message_registers_are_sm_or_unwritten` gate's third arm IS the roster. Retiring the
  roster re-keys that arm to "case-owned" (fixpoint-protected). So the five roster
  codes MUST become case-owned in phase 4 (the mini-backport) — otherwise the gate
  fails on unprefixed-uncased prose. This couples cleanly: exactly the codes that lose
  roster protection are the ones that gain fixpoint protection.
- **stored-string `[unwritten:]` placeholder** → retires in PHASE 4
  (`283:dec-message-becomes-option`): absent field renders the placeholder.
- **the 21 fragment goldens (63 files) + `DORC_DEFINING_BLESS`** → retire in PHASE 5
  (backport), code-by-code as each old-covered code gains a case file. Phase 4 leaves
  them INTACT (additive). NB the count is 21/63, not 17 (`f-fragment-count-is-21`).
- **the `covered()` `build:` constructor table** → transitional scaffold; retires in
  phase 5 as world-as-pipeline cases replace it, OR survives as world-as-payload for
  the hard tail.

What STAYS (`282` §8): the `DiagCode` enum + typed payloads + registry (the wire
spine); `diag_tidy`'s emit-site gate (the fires-half backstop — `every_catalog_variant_is_constructed`
+ the bijection); the e2e corpus + its plan-render goldens (a different product
surface); machine-envelope shape assertions (move to a `--format=jsonl` replay block,
or stay unit-tier — latitude).

### 4d — The ratchet, REDEFINED without regrowing (`283:flag-ratchet-redefine-not-regrow`,
`~SUSPECT`, §6 flag-4)

`282` §8 redefines the ratchet from "codes with no defining case" to "codes whose prose
is not yet case-owned," completeness `covered ∪ ratchet == all`. THE TRAP: naively
listing all 56 as ratcheted at the flip (no cases exist yet) GROWS the list 35→56 and
trips `ratchet_only_shrinks` (which compares to `git show HEAD`, i.e. 35). Resolution:
the ratchet LIST and its shrink-only direction are PRESERVED; the redefinition is
CONCEPTUAL. Concretely, a THIRD transient state exists during 4→5: `fragment-covered`
(old-style, goldens still present). The completeness partition through the transition
is `case-owned ∪ fragment-covered ∪ ratchet == all`, ratchet frozen at 35. Phase 5
moves each code fragment-covered → case-owned (goldens deleted one at a time), ratchet
untouched. When phase 5 closes: case-owned = 21, fragment-covered = 0, ratchet = 35.
Phase 7 shrinks the ratchet. `ratchet_only_shrinks` keeps passing unchanged (35 ≤ 35).
FLAG for the conductor: confirm this transient-third-state reading vs a one-time
baseline reset of `ratchet_only_shrinks` at the flip commit (I judge the transient
reading cleaner — no gate is weakened).

### 4e — Transitional carry-forward (`+SURE`)

promote-v2 sources prose from cases-where-they-exist ∪ current-catalog-prose-where-not
(`282` §8), so the 35 case-less codes keep their `sm `-prose mid-migration; coverage
only grows; no code loses prose in flight. Mechanically: `apply_field_edits` only
touches keys the dirty cases produced; every other entry's prose is carried by the
mirror unchanged.

---

## §5 — Ordering, blast-radius, the checkpoint, sizing

The execution commit ladder — additive-first, the ownership flip LAST
(`283:dec-additive-then-flip`, mirrors `28A` §2h/§2k's stop-before-cutover):

1. **render-seat parameterization** — introduce `CatalogLookup` + `OwnedEntry` +
   `serialize(&[OwnedEntry])`; thread `&dyn CatalogLookup` through the render seat;
   production passes the const wrapper. ADDITIVE. Blast-radius: WIDEST (every render
   call site) but mechanical; gate = renders byte-identical (existing goldens + the
   defining-case triple test unmoved).
2. **`message: Option` + absent-renders-placeholder** — schema edit; migrate the five
   `[unwritten:]` strings to `None`; re-key `unwritten_renders_are_greppable`. ADDITIVE
   (renders byte-identical). Blast-radius: `CatalogEntry` + 5 entries + 3 render seats
   + 2 tests.
3. **`render_cli_tagged`** — the §2m full-transcript tagged twin (title-split span
   relocation + caret frames as Arrangement). ADDITIVE (`render_cli` untouched); gate =
   text half byte-identical to `render_cli`, span map a gap-free total cover through
   `to_errorloom`. Blast-radius: `core::diag` + a defining-case tagged-twin test.
   LARGEST code chunk.
4. **the dorc-loom Consumer** — the four methods over the mirror; the `FakeGit`
   Consumer-loop unit tests (port `toy_consumer`'s five scenarios to Dorc). ADDITIVE.
5. **promote-v2 orchestration entry + both fixpoint gates** — the
   `DORC_CATALOG_PROMOTE` path (prose_bless → serialize → write → splice → fmt); the
   render-level `fixpoint_check` CI test + the Dorc-side promote→byte-identical CI
   gate, run over the (initially two-pilot) corpus. ADDITIVE (gates pass under
   carry-forward identity).
6. **the empty-loop pilot** — mint `marker-version-unrecognized` (variant + emit +
   constructor + case + `[unwritten:]` prose; ceiling 5→6); author its prose from the
   render (conductor act). ADDITIVE.
7. **⟵ CHECKPOINT (the executor STOPS here for conductor review)** — everything above
   is additive, byte-identical, gates green, corpus = pilots. The conductor reviews the
   render-seat parameterization, the tagged-twin span relocation, and the Consumer
   before the flip.
8. **THE FLIP** — mini-backport the five roster codes to case files (world-as-payload);
   retire the `CONDUCTOR_AUTHORED` roster + re-key `message_registers_are_sm_or_unwritten`
   to "sm | None | case-owned"; wire the Dorc-side fixpoint gate as a hard CI gate. This
   is the ownership-transfer commit; blast-radius = the two gate tests + the roster
   deletion + six case files.
9. **(PHASE 5, a SEPARATE dispatch — not this one)** — backport the remaining 15
   old-covered codes; delete the 63 fragment goldens + `DORC_DEFINING_BLESS` + the
   constructor scaffold as worlds land; the four `281` codes get real prose via the
   empty loop.

Checkpoint the executor honors: STOP after step 6 (before the flip). Sizing
(`-GUESS`): steps 1–2 are a half-day of mechanical threading; step 3 (`render_cli_tagged`)
is the risk-and-time sink (~1–1.5 days — the span relocation is delicate and
gate-heavy); steps 4–6 are ~a day (the errorloom loop is proven, so the Consumer is
plumbing); step 8 is small once 1–6 land. Total ~3–4 focused days, checkpoint-split.

---

## §6 — Ambiguity / flag list (for the conductor to rule before execution)

1. **`283:flag-in-process-vs-real-firing`** (the biggest) — phase-4's fixpoint gate
   reconstructs each render IN-PROCESS (world-as-payload or in-process pipeline),
   NOT by executing a subprocess `dorc` (`282` §7's runner). This keeps `core` tests
   hermetic/DST-clean and dodges needing a triggering world per code. `282` §1/§7 wants
   the replay commands LITERALLY executed. My read: the errorloom RUNNER (real
   subprocess) is a phase-5+/e2e-tier concern for real-firing cases; phase 4 uses
   in-process. RULE NEEDED: is in-process render_case acceptable for the phase-4 flip,
   or must phase 4 land ≥1 real-runner case to prove the `282` §7 harness against Dorc?
2. **`283:flag-mirror-fights-promote-v1`** — the `&'static str` const cannot be mutated;
   promote-v2 needs an owned mirror + a `CatalogLookup` render seam (§3). This is where
   the as-built promote tool's shape genuinely fights `282` §6 (v1 only ever CARRIED
   prose). The seam is clean but is real new surface in `core`. RULE: accept the
   `CatalogLookup` parameterization (my recommendation) vs an alternative (e.g.
   dorc-loom re-implements `render_cli` over its mirror — rejected: duplicates the seat).
3. **`283:flag-roster-retire-needs-mini-backport`** — retiring the roster in phase 4
   REQUIRES the five conductor-authored codes to be case-owned (else the sm/unwritten
   gate fails on their unprefixed prose). Recommend the phase-4 mini-backport of those
   five (world-as-payload). ALTERNATIVE: keep the roster vestigial until phase 5. RULE
   which. (I prefer the mini-backport — it makes the roster retirement REAL, per §8.)
4. **`283:flag-ratchet-redefine-not-regrow`** — the ratchet must not regrow 35→56 at the
   flip (§4d). Confirm the transient-third-state reading vs a one-time baseline reset.
5. **`283:flag-metadata-frontmatter-vs-const`** — do `when-fires`/`why` for case-owned
   codes DERIVE from case frontmatter (§1b, catches metadata hand-edits) or stay
   const-authored (simpler, but the Dorc-side gate then can't catch their drift)?
   Recommend frontmatter-derived (it is `282` §8's letter and makes the §2g gate
   meaningful), accepting the case-less carry-forward gap.
6. **`283:flag-message-option-vs-sentinel`** — `message: Option<&'static str>` (my
   recommendation, illegal-states-unrepresentable) vs an empty-string sentinel. Minor;
   flagged only if `Option` threads awkwardly through the render seat.
7. **`283:flag-editable-text-selection`** — how `editable_text` picks the prose-editable
   block in a multi-replay case: a `--format=` command-line sniff (recommended) vs a
   per-block frontmatter convention. Consumer latitude; low-stakes for phase-4 thin
   cases.
8. **`283:flag-fragment-count-is-21`** — the plan's "17 fragment goldens" is stale; it
   is 21 codes / 63 files. Non-blocking, but the executor's retirement accounting must
   use 21.
9. **BLESS-law confirmation** (no tension found, `+SURE`): promote-v2 stays
   orchestrator-only (`DORC_CATALOG_PROMOTE`-gated, fresh binary, diff inspected); the
   CI fixpoint gates are READ-ONLY verifications that RUN every test (they never mutate
   — they assert reproduction), which is correct and not a BLESS. `errorloom::prose_bless`
   is a library entrypoint the builder builds and never runs. No BLESS-law conflict.
10. **`283:flag-multi-paragraph-stays-unbuilt`** (no action, confirm) — catalog
    templates are single-line/single-paragraph (`28A` §2c/§2m); `render_cli`'s caret
    frames are multi-LINE Arrangement, not multi-paragraph PROSE, so the unbuilt
    multi-paragraph model (`28A:rul-paragraph-model-v1-refuses-restructure`) is not
    stressed by the tagged render. No conflict.

---

## §7 — Exclusion-check (per AGENTS.md; the flip re-tested across the cells)

- **other phase** (probe vs apply): the flip touches only the diagnostic-render/
  authoring plane (aid), which is decision-inert (`two-plane-aid-law`); neither probe
  nor apply licensing is touched. `render_cli_tagged` is a tool-mode output (`282` §4);
  it never enters a product surface. `+SURE` no license-plane leak.
- **other user** (admin vs engineer): errorloom is self-consumed internal tooling
  (`282:rul-own-crate-own-tests`, `282:rul-internal-tool-sharp-edges`); neither the
  admin nor the engineer sees it — they see the RENDER, which is byte-unchanged
  (additive twins). `+SURE`.
- **other reliability** (unreliable oracles): the world-as-payload cases carry canonical
  payloads, not oracle judgments; world-as-pipeline cases run inert fixtures. No oracle
  trust is at stake in the corpus. `+SURE`.
- **reverse propagation**: the tagged render is a forward render-composition; there is
  no analyzer propagation direction to reverse. N/A.

## §8 — Confidence summary

`+SURE`: the errorloom loop is complete and the Consumer is the phase-4 deliverable
(code-verified); the render seat is `render_body`/`render_cli` (not the aid-chain);
promote v1 only carries prose so v2's novel leg is the mutable-mirror write path;
catalog stays a generated const; the counts (56/21/35, 63 fragment files). `~SUSPECT`:
the `CatalogLookup`/`OwnedEntry` mirror seam as the mutable-catalog answer; the
title-split span relocation as the dominant chunk; the roster-mini-backport coupling;
the transient-third-state ratchet reading; metadata-from-frontmatter. `-GUESS`: the
sizing (~3–4 days, checkpoint-split); world-as-payload vs world-as-pipeline mix per
code (a phase-5 authoring call). `--WONDER`: whether the conductor will want phase 4 to
land a real-subprocess-runner case (flag-1) — that would enlarge phase 4 materially.
