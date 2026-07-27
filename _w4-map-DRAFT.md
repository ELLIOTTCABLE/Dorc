# `_w4-map-DRAFT` — Phase W4 map (read-only; STOPPED at the checkpoint)

Lane `lane-w4-map`, branch `ai/r28-w4-map` off `ai/r28-unify` @ `ec7fba32`. MAP ONLY:
no code, no locks, no looms, no bless. Charter: `plans/28G` §1 Phase W4. Design
record: `notes/28E`. Predecessor arc: `notes/28F`. Seams: `notes/289`.

Method: every number below is counted in-tree in THIS worktree, by script over
`crates/aid/src/arrangement_lock.rs` and by grep over `crates/cli/src/main.rs`;
nothing is quoted from a prior doc's estimate. Gates are spelled as `mise run …`
throughout — `mise run test:e2e` is the only sanctioned executor of fixture
material, and no lane hand-rolls a `cargo` invocation.

Confidence marks: `+SURE` verified by reading the code · `~SUSPECT` strongly
implied · `-GUESS` sizing/judgment · `--WONDER` open.

---

## §0 — Bottom line, before the detail

1. **`fnd-class-c-is-the-majority`** (+SURE) — of the why surface's 56
   sentence-bearing registry rows, **45 (80%) are value-interleaved multi-word
   entries the edit transport refuses today**; they hold **61% of the surface's
   registry prose by character**. Class (a) — already loom-editable — is
   **0 of 111 why rows**. The deferred transport work (`28G` §2 "Transport
   enrichment") therefore stands directly between the human and W5.
2. **`fnd-fix-is-dorc-side-not-errorloom`** (~SUSPECT, the map's most
   consequential claim) — the refusal is minted in `dorc-loom`
   (`DorcApplyRefusal::ArrangementIsSequenceStructured`) and CAUSED by a
   dorc-side rule (`to_editable_render` opens a NEW editable section at every
   computed run). errorloom already supports an ordered `Text | Variable`
   series inside ONE section with untouched-variable preservation by identity —
   the exact machinery the catalog path uses. So `28E:prop-span-boundary-
   tokenization`'s "fix the word-model IN errorloom" is probably over-priced.
3. **`fnd-span-map-has-no-production-consumer`** (+SURE) — `rendered.spans()`
   is read by tests only. `print_document` calls `weft::render_framed(..).text()`
   and drops the map. That single line IS `28F`'s span-map-unconsumed seam.
4. **`fnd-print-sites-are-26`** (+SURE) — counted, not estimated: 26
   report-family call sites, 14 inside analysis helpers, 12 at `run`'s seats.
5. **`fnd-carrier-lane-is-churn-free`** (+SURE) — e2e pins stderr by NEEDLES
   (`scan_diagnostics`/`scan_why`/`scan_hint`), never byte-goldens it;
   `expected.out` is stdout. So the Carrier lane costs zero re-bless.

---

## §A — Parts at birth

### `a1-the-one-producer`

`Explanation.reason: String` has exactly ONE producer: `dorc_aid::diag::why()`
(`crates/aid/src/diag.rs:2720`). It fires only for `DiagCode::CmdsubOperandTop`
with a resolvable `cause` (fd-G honesty); every other code yields `None`.

The reason is a `format!` of four materially different things:

| part | source | today's class |
|---|---|---|
| the opener sentence | hardcoded `format!` in `diag.rs` | `289:finding-reason-opener-still-hardcoded` — a §0 violation |
| the operand position | `OperandPosition::describe()` (`diag.rs:424`) — `format!("operand {n}")` / `"the command word"` | a second §0 violation, unnoticed |
| the cause locus | `render_span(span, src)` (`diag.rs:2847`) — `"{lo}:{hi} \`{text}\`"`, **book bytes, unencoded** | mixed computed + FOREIGN |
| the remediation hint | `arrangement::arrangement_text(…, remediation_hint_slug(class))` | a registry row, read as PLAIN TEXT (faceless — `289:seam-whylens-render-seat`) |

`ask-why-lens-stderr-unencoded` falls out of row 3 (§G).

### `a2-the-two-consumers-and-their-skeletons`

Exactly two, matching `289:seam-whylens-render-seat`'s "a FRAGMENT embedded
mid-line by two consumers":

- **`consumer-plan-stderr-lens`** — `emit_why_lens` (`cli/main.rs:3287`, called
  once at `:1267`) → `why_lens_lines` → `eprintln!("why: {line}")`.
  *Skeleton*: the `why: ` prefix + one line per explanation + the `(cause, site)`
  dedup. Not weft, not span-covered, not registry-homed, not encoded.
- **`consumer-why-report-run-reason`** — `top_run_reason` (`cli/main.rs:5659`)
  → `Said::Lens(reason)` at `:5146`, one arm of `emit_why_report`'s
  `Disposition::Run` match. That `Said` then reaches three sub-skeletons:
  `plain_chain`'s OUTCOME because-clause (as the 4th value of
  `why-outcome-contrastive`), `plain_chain`'s ANALYSIS rows (as
  `ChainLink.payload`), and `aggregate_item`'s reason rows.
  *Skeleton*: the triptych / the aggregate item.

### `a3-parts-vocabulary-proposed`

The vocabulary already exists and is in the wrong crate. `Said`
(`cli/main.rs:3971`) is `Words(&'static str, String) | Value(String) |
Lens(String)`; `Said::run()` maps straight onto `aid::weave::{words, value}`;
`Said::words()` calls `aid::arrangement::arrangement_sentence`. Every dependency
is already `aid`'s, and `aid` already deps `weft`.

- **`prop-hoist-said-into-aid`** — move `Said` to `crates/aid` (beside `weave`).
  Add `Said::Foreign { text, source }` (mapping to `weave::foreign`, encoding at
  mint). **Delete `Said::Lens` — that deletion is the phase's point.**
- **`prop-explanation-carries-parts`** — `Explanation { parts: Vec<Said>,
  remediation: RemediationClass }`. `why()` emits:
  `[Words("why-reason-cmdsub-opener", [position]), Value("{lo}:{hi}"),
  Foreign{book bytes}, Words(remediation_hint_slug(class), [])]`.
- **`prop-position-word-is-a-row`** — `OperandPosition::describe()` becomes a
  registry row with occurrence 0 (`the command word`) / 1 (`operand ` + n).
  Verbatim migration ⇒ zero rendered-byte change.
- **`prop-opener-is-a-row`** — the hardcoded opener migrates verbatim as
  `Words::Migrated`, discharging `289:finding-reason-opener-still-hardcoded`.

Both consumers then own real skeletons over `Vec<Said>`: the stderr lens gets a
seat that stamps runs and prints through one function; the report seat drops
`Said::Lens` and treats the reason as ordinary parts.

### `a4-honouring-the-three-clauses`

- **born-DAG-shaped** — `Explanation` stays a LEAF part-stream; it must never
  become a chain node. DAG shape lives at `ChainRender.links` + `weft::Join`
  (already present, `weft-joins-are-dag-shaped`). No change owed here.
- **truncation-legal-at-any-link** (`28E:prop-register-per-node`) — weft already
  carries `Mark { register, criticality }` per node (`weft-register-slot-reserved`).
  A `Said` cannot yet render down to one word because a registry entry stores one
  full sentence. The room is a THIRD key axis on `ArrangementEntry` (see §D and
  `ask-register-key-axis-reserved`); build none of it.
- **never bake chain order as semantic** — `survival_chain` (`cli/main.rs:4121`)
  constructs links in a fixed order (report → vouch → claims → derives). That
  order is currently implicit in straight-line code. See
  `ask-chain-link-order-is-a-render-default`.

---

## §B — Carrier to the edge

### `b1-the-count` (counted; the plan said "~25")

**26 report-family call sites** in `crates/cli/src/main.rs` (excluding the two
internal calls inside `report_at`/`report_by_oracle_file` and lint's own
`.report()` method at `:426`):

| enclosing fn | sites | lines | class |
|---|---|---|---|
| `run` | 12 | 691, 749, 753, 764, 771, 832, 845, 1104, 1186, 1220, 1324, 1349 | edge-seat (already at the edge; still uncarried) |
| `load_whylog_replay` | 2 | 1524, 1587 | loader/edge |
| `refuse_replay` | 1 | 1705 | in-body |
| `report_whylog_unwritten` | 1 | 1779 | report seat (error-floor) |
| `build_survival_footprints` | 2 | 2012, 2077 | **in-body analysis helper** |
| `merge_derived_footprints` | 1 | 2352 | **in-body analysis helper** |
| `build_kind_resolvers` | 2 | 2464, 2529 | **in-body analysis helper** |
| `build_kind_reaches` | 2 | 2793, 2849 | **in-body analysis helper** |
| `build_vouches` | 1 | 3065 | **in-body analysis helper** |
| `emit_unloaded_sibling_oracles` | 1 | 6941 | emit seat |
| `emit_escalation_policy` | 1 | 7012 | emit seat |

**14 are inside helpers that return bare values instead of `Carrier<T>`** (the
8 helper-body sites plus the 2 loader + 1 refuse + 1 unwritten + 2 emit seats);
12 are `run`'s own stage seats.

### `b2-classification-by-severity-lane`

- **advisory-vs-error-floor is not per-SITE, it is per-DIAG**: `advisory_filter`
  (`:7474`) drops Warning/Note when `!advisory` (the receipt-free `apply`
  off-ramp) and keeps Error always (`22F:advisory-vs-error-cut`). So every one
  of the 26 sites is "advisory-filtered"; none is intrinsically error-floor
  except `report_whylog_unwritten` (`:1779`), which calls `report` DIRECTLY,
  bypassing the filter — deliberate (`28F:rul-write-failure-is-error-floor`).
- **chrome**: `report()` composes `"{stage}: {word}"` and ANSI-styles the
  severity word outside the typed render bytes; the `stage` token is chrome, not
  catalog prose (`invocation-errors-are-registry-codes`).

### `b3-the-retreat`

`advisory: bool` appears in **12 signatures**, all in `cli/main.rs`
(`:1508, :1704, :2005, :2270, :2455, :2784, :3057, :6899, :6961, :7444, :7453,
:7474`). It appears in **no** kernel crate — `plan`, `analysis`, `oracle`, `aid`
are already clean (+SURE, grepped). So "retreats to the edge" means: delete it
from the 8 analysis-helper/emit signatures, keep it on `advisory_filter` and one
edge seat. That is a smaller job than `289` §2v implies.

### `b4-the-accumulation-path`

Each helper returns `Carrier<T>` (already the crate-wide spine — `inv-no-throw`,
`aid::Carrier`); `run` folds the diags and reports at the SAME points it does
today. **Recommendation: preserve the flush points, so stderr is
byte-identical.** Re-ordering emissions is a separate question
(`ask-emission-order-may-move`, §G) — `289` §2v explicitly named
emission-order-by-scheduling a preview of the multi-host concurrency cell, so it
is not a free refactor.

### `b5-the-libtest-red-frame`

`report()` (`cli/main.rs:7505`) writes real fd 2 via `anstream::stderr()`.
libtest's capture intercepts the `print!`/`eprint!` macros' output-capture hook,
**not** direct `io::stderr()` writes — so the bytes go straight to the console
of `mise run test`. Any unit test that drives a printing helper (e.g.
`resolver_confusability_conflict_refuses_both_collision_keeps`, `:7745`, which
drives `build_kind_resolvers`) prints a full red `error[resolver-conflict]`
caret frame interleaved into a GREEN run. Carrier accumulation kills it by
construction: the helper returns diags as data, the test asserts on them, and
nothing writes fd 2. (+SURE on the mechanism; the fix is not a capture trick.)

---

## §C — Span coverage → loom round-trip (the measurement)

### `c1-the-as-built-path`

```
survival_chain / decline_chain / guard_chain / plain_chain   (cli/main.rs)
   -> ChainRender { links: Vec<ChainLink>, ... }             cli-local structs
   -> chain_nodes / chain_rows / step_nodes / receipt_banner  weft Node<Face>
   -> Said::run(part)  ->  aid::weave::{words,value,mark,foreign}  ->  weft::Run<Face>
   -> weft::render_framed(&Document, &Frame) -> Rendered<Face> { text, spans }
   -> print_document:  print!("{}", rendered.text())          <-- SPAN MAP DROPPED
```

The parallel, WORKING path for every other surface:

```
render_cli_parts / render_staged_cli_parts / lint .human()  -> aid::tagged::RenderParts
   -> dorc_loom::to_editable_render(&parts) -> EditableRender<SectionKey, SectionVariableId>
   -> ReplayResult::editable(...)  [output := editable.text(), welded]
   -> errorloom transport -> compile -> promote -> catalog_lock.rs / arrangement_lock.rs
```

**Nothing joins the two.** There is no `weft -> RenderParts` bridge in the tree
(+SURE, grepped: `RenderParts` appears in no `cli/src`, no `weave.rs`, no
`weft/`). And `DorcConsumer::replay` (`dorc-loom/src/consumer.rs:316`) has arms
for `dorc plan`, `dorc lint`, `dorc why <whylog-file>` (the whylog REFUSAL
diagnostics only), `dorc-loom vars`, arrangement pages, and invocation errors —
**no arm renders a `dorc why <addr>` triptych.** The six `whygallery-*.loom`
cases live in `crates/cli/tests/` with `run: round-trip` + `fixpoint: executed`,
so the looms runner defers their proof to the real binary
(`one-fixpoint-authority-per-case`) and no editable provenance exists for them.

### `c2-what-is-missing`, precisely

1. **`gap-span-map-unconsumed`** — `print_document` discards `Rendered::spans()`.
2. **`gap-no-weft-to-parts-bridge`** — needs a mapping
   `Provenance<Face> -> RenderPart`:
   - `Arrangement{Some(Face::Row(slug))}` → `ArrangementWords` (editable)
   - `Arrangement{Some(Face::Part(p))}` → `Arrangement` (computed)
   - `Arrangement{None}` → `Arrangement` (weft's own layout)
   - `Param{key: Face::Row(slug), param: Face::Part(p)}` → **no part class exists**
   - `Foreign{Face::Source(String)}` → `ForeignText{param: &'static str}` —
     **shape mismatch**: weft carries a runtime path, tagged carries a static
     param name.
3. **`gap-occurrence-lost-at-the-weave-seat`** — `weave::words(text, slug)` takes
   NO occurrence. 16 of the reached rows are occurrence-keyed (`why-tier-word`
   ×7, `why-next-step-label` ×5, `why-wall-payload` ×2, `why-declines-*` ×4+).
   `to_editable_render` would then key them by RENDER POSITION
   (`instance: occurrence.unwrap_or(*position)`), silently mis-attributing an
   edit to the wrong entry. Latent the moment the surface gets a face.
4. **`gap-wrapped-lines-fragment-a-section`** — weft wraps. It drops the
   whitespace token at a break and emits its own `Arrangement{key: None}`
   newline+pad (`wrap.rs:96-101`), and it NEVER hard-breaks a token (an
   over-long word overruns). So a wrapped chrome line's bytes are
   `Row … Arrangement{None} … Row`, and `to_editable_render` flushes at the
   Arrangement — one logical row becomes N sections, each with a distinct
   `segment`, and `apply_arrangement_edit` would be called N times with partial
   words. **This is the mechanical root of the whole class-(c) refusal, and it
   is a dorc-side rule.**
5. **`gap-no-why-replay-arm`** — `dorc-loom` cannot drive a `dorc why <addr>`
   invocation in-process, so no case can carry editable why bytes.

### `c3-the-classification` (counted)

Registry census, `crates/aid/src/arrangement_lock.rs` at `ec7fba32`: 128 rows —
111 `why-*`, 12 `lint-*`, 5 `cli-*`. Prose states: 1 `Authored`, 106 `Migrated`,
21 `Unwritten`.

Reached by the why render (82 slugs referenced by literal in `cli/main.rs`, plus
4 `why-remediation-*` from `diag.rs`; 17 further literals are `Face::Part` seat
names, not rows):

| class | what | count | share of reached prose chars |
|---|---|---|---|
| **(a) editable today** | — | **0 rows** | 0% |
| **(b) editable with span coverage alone** (single-run entries) | 40 rows / 1739 chars | 40 | 39% |
| **(c) blocked by the word-model** (value-interleaved multi-word entries) | 45 rows / 2748 chars | 45 | **61%** |
| **(d) computed placeholders** | see `c4` | — | exempt |
| **(e) foreign text** | see `c4` | — | exempt |

Class (b) is inflated by labels. Splitting it by length:

- **21 rows under 25 chars** — one-word labels: 7 `why-tier-word` occurrences
  (`reported`/`vouches`/`claims`/`derives`/`ran`/`consented`/`declines`), 5
  `why-next-step-label`, 4 `why-outcome-word`, 3 panel headings,
  `why-alternative-connective` (`OR`).
- **8 rows 25–59 chars** — short fragments.
- **11 rows ≥ 60 chars** — genuine value-free sentences: `why-mark-legend`,
  `why-claims-covers-unmeasured`, `why-next-step-verify`, `why-receipt-footer`,
  `why-reason-run-unprobed`, `why-reason-run-not-elidable`,
  `why-reason-render-refused`, `why-wall-payload`, `why-declines-explanation`,
  `why-declines-derives-cannot-say-runs`, `why-declines-join`.

**So of 56 sentence-bearing rows, 45 (80%) are class (c).** Every one of the
strawman's load-bearing sentences is in that 45: `why-outcome-contrastive`,
`why-outcome-because-survived`, `why-analysis-opener`, `why-analysis-join`,
`why-claims-payload`, `why-vouch-payload-site`, `why-derives-payload-disjoint`,
`why-next-steps-opener`, `why-next-step-suspect-sole-claim`,
`why-next-step-fix-widen`, `why-next-step-fix-replan`, `why-next-step-review`,
`why-receipt-plan-tally`, `why-trust-spent-item-reason`,
`why-improvement-quantified`, `why-chain-event-received`, …

The 21 `Words::Unwritten` rows have no arity yet; they will land wherever their
seat's value count puts them. `-GUESS`: seat shapes suggest most land in (c).

### `c4-classes-d-and-e`, structurally

- **(d) computed** — `[unwritten: <slug>]` and `[unnarrated: <class>]`
  (`28F:rul-placeholders-are-computed`, the stated carve); every `Said::Value`
  (coordinates, `N|command` references, `file:line` speakers, counts, digests,
  timestamps, risk-profile strings); `weave::mark` glyphs (`*` / `!`); every
  gutter line-number; all weft geometry (`===`, ` | `, quotes, `[`, indentation,
  truncation glyphs) under `weft-geometry-vs-words`. Exempt by design.
- **(e) foreign** — `as-written:` excerpt bodies (`weave::foreign`, keyed
  `Face::Source(path)`), the shipped-guard block, the participating-lines code
  bodies. Exempt by design (`28G` §0's foreign-text carve). In the
  `whygallery-survive-trusted-footprint` transcript: 3 foreign runs.

### `c5-the-crux — price the transport work`

Class (c) is not small. It is the surface's prose. Leaving it deferred means the
W5 human authors ~20% of why sentences by editing looms and ~80% by hand-editing
`crates/aid/src/arrangement_lock.rs` — a `@generated … DO NOT EDIT` file where
hand-seeding is only "approximately" sanctioned (`28F` loom-friction accretion),
and where authoring happens OUTSIDE the render: exactly the analyzer-headspace
footgun `282:rul-transcript-is-the-authoring-surface` was built to prevent.
AGENTS.md's anti-deferral bullet ("don't stage or defer tasks that can simply be
done now") points the same way.

**`prop-one-section-many-fragments`** — the fix, and it is smaller than the
standing proposal assumes:

1. Mint `RenderPart::ArrangementValue { text, slug, occurrence, index }` — a
   computed value INSIDE a chrome line that does **not** flush the section.
2. Teach `to_editable_render` to accumulate adjacent `ArrangementWords` /
   `ArrangementValue` parts sharing `(slug, occurrence)` into ONE
   `EditableSection` whose fragments are `Text | Variable`, exactly as
   `open_section` already does for the catalog's
   `TemplateLiteral | ParamValue`. Variable names are positional (`v0`, `v1`, …).
3. Rewrite `apply_arrangement_edit`: instead of `words.concat()` + a refusal,
   walk the compiled fragment series — `Text` → a word, `Variable` → a boundary
   — and require the Variable sequence (by name, in order) to equal the stamped
   sequence. Reorder/drop/duplicate REFUSES (a new, narrower refusal class); the
   ordinary case compiles to `words.len() == values.len() + 1`, which is exactly
   `arrangement_sentence`'s arity contract.
4. The weft bridge absorbs a pure-whitespace `Arrangement{None}` run that lies
   BETWEEN two spans of the SAME key into that key's section as Text; the
   compile-back collapses whitespace runs to one space (`282` §3's read-in
   normalization, already the law). Sound because weft only ever replaces an
   existing whitespace run with newline+pad and never hard-breaks a token
   (`wrap.rs`, read).

**errorloom needs no change under this shape** (~SUSPECT — it already preserves
untouched variables by identity before tokenization,
`282:rul-untouched-variable-preservation`, phase 1 COMPLETE). The 2026-07-24
observation that fragmenting a line "broke attribution for every OTHER prose
section in the same render" was about splitting one line into MANY SECTIONS
separated by computed Structure; this proposal keeps it ONE section. That
distinction is the whole design.

**Cost of the fence it lifts**: amending `aid/CLAUDE.md`'s
`a-chrome-line-is-one-span` + `arrangement-words-are-a-sequence-nothing-splits`,
and `plans/288` §7b's closing sentence. That is a LAW change → `ask-lift-one-
span-per-chrome-line` (§G).

---

## §D — kTASTE type room

`28E:ask-tasty-productive-knob` demands the model retain (1) the welded
conclusion, (2) the narrative residue, (3) the selection metadata relating them.

**As-built**: `ChainRender` (`cli/main.rs:4061`) holds only the SELECTED links;
the walkers discard the rest at construction; `--all` reaches exactly one thing
(`deepest_tier` → the `[unnarrated:]` census, `cli/main.rs:5033`) and does
nothing to the chain. Meanwhile the render PRINTS
`why-next-step-review` = "`dorc why <addr> --all` (every link, unselected,
exhaustive)". That promise is false today → `ask-all-flag-promises-exhaustive`.

**`prop-ktaste-minimal-shape`** — types only, zero machinery, sited in `aid`
beside the hoisted `Said` (describe plane; the walkers stay in `cli`/`plan`):

```rust
/// Everything the walker derived, plus what a render selected and why.
struct ChainModel {
    links: Vec<ChainLink>,            // ALL of them — the residue included
    conclusion: Said,                 // the welded synthesis (today: ChainRender.join)
    selection: Vec<LinkSelection>,    // parallel to `links`
}

struct LinkSelection {
    relevance: Relevance,             // ONE variant today, weft::Register's precedent
    superseded_by: Option<LinkRef>,   // a DAG EDGE, never an order
    implied_by: Vec<LinkRef>,         // a DAG EDGE, never an order
}

enum Relevance { #[default] Selected }
struct LinkRef(usize);                // index into `links`
```

- The default render reads `links` filtered by `relevance`; `--all` reads
  `links` whole — which discharges the copy-paste-truth gap for free.
- `weft`'s `Mark { register, criticality }` stays the MARKING channel
  (`28E:rul-renderer-owns-layout`); `ChainModel` is the semantics side. Do not
  duplicate one into the other.

**Must NOT foreclose** (state these in the work order):

- `relevance` must never become a `bool` — kTASTE is two GOALS × densities, not
  one axis.
- the residue must be stored as PARTS (`Said`), never as pre-rendered strings —
  flattening it puts the tasty pole's material at the productive pole's
  resolution, which is the exact collapse the ruling forbids.
- `superseded_by` / `implied_by` are edges by `LinkRef`, never a sorted position
  (`28E:lean-ordering-is-a-seam`).
- the arrangement-registry key is `(slug, occurrence)` and occurrence is ALREADY
  spent on position discriminators. A per-register entry needs a THIRD key axis.
  Do not spend `occurrence` on register; see `ask-register-key-axis-reserved`.
- build no register machinery, no density selection, no `--terse` for why.

---

## §E — Churn analysis

Evidence base: e2e pins stderr by NEEDLE scans (`scan_diagnostics`, `scan_why`,
`scan_hint` in `crates/cli/tests/e2e.rs`), never by byte-golden; `expected.out`
carries STDOUT only, and a replay block's stderr is dropped (`e2e.rs:1235`).

| step | render-preserving? | expected re-bless |
|---|---|---|
| `lane-w4-carrier` (26 sites → `Carrier`, `advisory` retreat) | **YES, provably**, if flush points are preserved | **zero.** Even an emission-order shift would not move a golden byte — but see `ask-emission-order-may-move` |
| `lane-w4-parts` (`Explanation` → parts; opener + position → rows; `Said` hoist; kTASTE types) | **YES**, if the migration is VERBATIM | **zero** if verbatim. One needle file, `crates/cli/tests/exec-opaque-var-runs/expected-why`, substring-matches `ran because operand 3 is a command-substitution` and survives. `mise run loom:promote` regenerates the arrangement lock with the 2–3 new `Migrated` rows |
| `lane-w4-span` (bridge + occurrence-carrying `Face` + interleaved-value parts + apply re-split + `dorc why` replay arm) | **YES by design** — the bridge only ADDS a consumer of the span map; the printed bytes come from the same `Rendered::text()` | zero rendered churn intended; **prove it with an empty diff over the 6 `whygallery-*.loom` + `survivebite27-naked-trust-chain.loom`**. NEW: 1–2 why-surface cases under `crates/aid/tests/`. Lock regenerates if any seat's word-split changes |

**WORDING-churn flags (not this arc's):**

- `w-flag-verbatim-only` — the opener, `OperandPosition::describe()`'s two
  strings, and every migrated row land BYTE-VERBATIM as `Words::Migrated`.
  Builders author zero prose (`error-authorship-tier`, `prose-three-state`); any
  temptation to "improve while moving" is refused.
- `w-flag-all-row-may-need-rewording` — if `ask-all-flag-promises-exhaustive` is
  answered by shrinking the promise instead of by the residue model, the fix is
  PROSE and belongs to W5/the human, not to a W4 builder.
- `w-flag-declined-rows-unwritten` — 21 `Unwritten` rows render
  `[unwritten: <slug>]` today. W4 must not fill any of them.

---

## §F — The lane cut

Three lanes, serial on `crates/cli/src/main.rs` (which all three touch heavily —
9,730 lines, one file; parallel lanes on it would collide). One optional split
buys wallclock if the conductor wants it.

### `lane-w4-carrier` — **S/M**, first, no dependencies

*Work order*: the 26 sites; helpers return `Carrier<T>`; `advisory: bool` leaves
the 8 helper/emit signatures and survives only at `advisory_filter` + the edge
seats; flush points preserved so stderr stays byte-identical; the printing helper
`report()` keeps writing fd 2 but is reached only from `run`. Pin the libtest
noise dead: a test asserting that `build_kind_resolvers` returns its
`resolver-conflict` diag as a VALUE (the diag is the asserted behaviour today —
it must stay asserted, just not printed).

*Gate*: `mise run gate:full-quiet`, then `mise run both gate:full-quiet`. Empty
diff over `crates/*/tests` proves the zero-churn claim.
*Looms*: none touched.

### `lane-w4-parts` — **M**, after `carrier` folds

*Work order*: hoist `Said` into `aid` + `Said::Foreign`; delete `Said::Lens`;
`Explanation { parts: Vec<Said>, remediation }`; the opener and
`OperandPosition::describe()` become `Words::Migrated` registry rows, verbatim;
`render_span`'s book bytes split into `Value` + `Foreign` (which fixes the
unencoded stderr path for free); both consumers own skeletons; `ChainModel` /
`LinkSelection` / `Relevance` types minted, unconsumed except that `--all`
renders `links` unfiltered.

*Gate*: `mise run gate:full-quiet`; `mise run loom:compile` then
`mise run loom:promote` for the new rows; `mise run bless` ONLY if a rendered
byte actually moved (it should not) — conductor-tier, promote-before-bless per
`two-bless-paths-split-by-directory`.
*Looms*: touched only if a byte moves. Whole-loom read required if so.

### `lane-w4-span` — **L**, after `parts`

*Work order (checkpointed at the midpoint)*:
- **leg A (transport, file-disjoint from `cli/main.rs`)** — `Face::Row` carries
  the occurrence (or a `Face::RowAt`); `weave::words_at`;
  `RenderPart::ArrangementValue`; `RenderPart::ForeignText` reshaped to carry a
  runtime source; `to_editable_render` accumulates one section per
  `(slug, occurrence)`; the whitespace-absorption rule; `apply_arrangement_edit`
  re-splits on the compiled fragment series with a narrow reorder refusal.
- **CHECKPOINT** — leg A green with a unit-tier round-trip over a synthetic
  multi-run row before leg B starts.
- **leg B (cli + cases)** — `aid::weave::to_render_parts(&Rendered<Face>)`;
  `print_document` renders once and hands the map to the seat; a `dorc why
  <addr>` arm in `DorcConsumer::replay` **and** in `render_direct_replay` (the
  two must agree — see §H); one or two why-surface cases in
  `crates/aid/tests/`, one single-run and one multi-run, proving the loop.

*Gate*: `mise run test:looms`; `mise run loom:compile` + `mise run loom:promote`
over the new cases; `mise run gate:full-quiet`; `mise run both gate:full-quiet`;
empty-diff proof over the 6 whygallery looms + `survivebite27`.
*Looms*: authors NEW cases. **Zero user-facing prose** — a new case's row is
seeded `Words::Unwritten` or migrated verbatim; the render shows the greppable
placeholder. Whole-loom reads mandatory when touching any existing loom.

### `f1-optional-parallel-variant`

`lane-w4-span` leg A touches only `weft/`, `aid/src/{tagged,weave,arrangement}.rs`
and `dorc-loom/src/` — disjoint from `cli/src/main.rs`. It can run PARALLEL with
`lane-w4-parts` if wallclock matters. `-GUESS`: saves ~30% of the arc; costs one
cross-merge. Recommend serial unless the conductor is time-pressed.

### `f2-platform-exposure` — **NONE, confirmed**

`cfg(unix)`/`cfg(windows)` in the touched crates lives only in
`cli/src/whylog_store.rs` (5), `dorc-loom/src/receipt_store.rs` (19), and one
block in `cli/src/main.rs::materialize_shim_dir` (`:524`). No W4 lane touches
any of them. The `mise run both gate:full-quiet` leg is still owed at each fold
(`wsl-unix-leg-at-fold`) — cheap insurance, not a live risk.

---

## §G — ASK list (conductor / human rulings; recommendations inline)

1. **`ask-pull-transport-into-this-arc`** — class (c) is 80% of the why
   surface's sentence-bearing rows and 61% of its prose characters, and it
   directly blocks W5. `28G` §2 defers it. **Recommend: PULL IN**, as
   `lane-w4-span` leg A. AGENTS.md's anti-deferral bullet and
   `289:steer-errorloom-best-to-use` both point the same way.
2. **`ask-word-model-fix-lives-dorcside`** — `28E:prop-span-boundary-tokenization`
   frames the fix as an errorloom word-model change. My reading says the refusal
   is dorc-side (`to_editable_render` flushing at every computed run;
   `apply_arrangement_edit` concatenating) and that errorloom's existing
   `Text | Variable`-within-a-section machinery suffices unchanged.
   **Recommend: build the dorc-side shape first, prove it on two cases, and
   touch errorloom only if a refusal class actually demands it.** ~SUSPECT, and
   it contradicts a standing proposal, so it needs a ruling rather than a
   builder decision.
3. **`ask-lift-one-span-per-chrome-line`** — doing (1)+(2) amends
   `aid/CLAUDE.md`'s `a-chrome-line-is-one-span` and
   `arrangement-words-are-a-sequence-nothing-splits`, plus `plans/288` §7b.
   **Recommend: amend to "one editable SECTION per chrome line"** — the section
   may hold interleaved value fragments; it may never be split into multiple
   sections. That preserves the observed-2026-07-24 failure's actual lesson.
4. **`ask-hoist-said-into-aid`** — `Said` is describe-plane vocabulary living at
   the cli edge by accident; `aid` already carries every dependency it needs.
   **Recommend: yes**, in `lane-w4-parts`.
5. **`ask-all-flag-promises-exhaustive`** — `why-next-step-review` prints
   "`--all` (every link, unselected, exhaustive)" but `--all` reaches only the
   `[unnarrated:]` census. Copy-paste-truth (`28E` §7 held-placement-reread).
   **Recommend: the kTASTE residue model discharges it** (`--all` renders
   `links` unfiltered). If deferred instead, the ROW must be reworded — which is
   prose, i.e. human/W5, not a builder act.
6. **`ask-emission-order-may-move`** — `tc-`shaped, flagged UP, not resolved
   here. Carrier accumulation can change stderr interleaving relative to the
   why-lens and survival-attribution lanes. **Recommend: preserve flush points
   (byte-identical) this arc**; bank re-ordering with `289` §2v's multi-host
   concurrency note.
7. **`ask-arrangement-normalization-fork`** — whitespace-collapse on
   compile-back is right for a weft-REFLOWED section and WRONG for
   `cli-help-page` (a verbatim whole page with alignment and blank lines).
   **Recommend: normalize only sections the weft bridge minted**; the
   arrangement-page path stays verbatim. Needs a ruling because it forks the
   part class.
8. **`ask-why-case-collection-placement`** — `tc-`shaped (a case-class mint),
   flagged UP. `288:rul-slug-decides-loom-placement` puts a registered aid slug's
   canonical case in `crates/aid/tests/`, but a why triptych needs a
   book+oracles+records world, which the param-free arrangement-page shape has
   no room for. **Recommend: a new in-process-driven why case shape in
   `crates/aid/tests/`**, with the existing `whygallery-*.loom` whole-product
   cases retained unchanged as executed evidence.
9. **`ask-register-key-axis-reserved`** — the registry key is
   `(slug, occurrence)` and occurrence is already spent on position
   discriminators (7 tier words, 5 step labels, …). A kTASTE register needs a
   third axis. **Recommend: do NOT spend occurrence on register; state in
   `aid/CLAUDE.md` that a third key axis is a cheap lock reshape under
   `rul-strawman-formats-no-compat`.** No machinery now.
10. **`ask-chain-link-order-is-a-render-default`** — `survival_chain` bakes the
    row order (report → vouch → claims → derives) into straight-line
    construction, which `28E:lean-ordering-is-a-seam` forbids treating as
    semantic. **Recommend: a one-function seat plus a doc-comment naming it a
    RENDER DEFAULT** so `28E:prop-distrust-order-default` can land later. No
    ordering machinery.
11. **`ask-why-lens-stderr-unencoded`** — `emit_why_lens` prints
    `render_span`'s raw book bytes to stderr with no `encode_foreign` and no
    ASCII guarantee, while the weft path encodes the identical bytes
    (`28D:must-encode-per-surface`, `rul-ascii-output-forever`; the corpus ASCII
    gate carves echo-of-input as the author's voice). **Recommend:
    `lane-w4-parts` fixes it for free** via `Said::Foreign`. Reported as a
    law-compliance observation, deliberately NOT as a security item.

---

## §H — LOOM / ERRORLOOM FRICTION (observations only; nothing fixed)

1. **`friction-unreflow-is-shape-specific`** —
   `dorc-loom/src/bin/dorc-loom.rs:729` `unreflow` is a hand-rolled de-wrapper
   that knows the CLI-diagnostic shape (title continuation indent `   `,
   `= help:` blocks at `      `, caret gutters detected by `is_caret_gutter`).
   It has already produced two near-misses (`289:rul-reflow-fix-in-phase-four`:
   a lost trailing newline refused EVERY case edit; a swallowed leading space
   refused every compact-lint transcript edit). It cannot be extended to weft's
   box-model output. Note: weft's span map makes it unnecessary for that
   surface, because renderer-inserted layout is already typed
   `Arrangement{key: None}` — a structural improvement that exists for free, not
   a fix to make now.
2. **`friction-editable-text-is-welded-to-output`** —
   `errorloom::ReplayResult::editable` sets `output := editable.text()`. There is
   no "logical baseline plus separately-wrapped transcript" mode, so any bridge
   must reproduce the wrapped bytes exactly. Named, not built.
3. **`friction-two-replay-arm-chains`** — `DorcConsumer::replay`
   (`consumer.rs:316`) and `DorcConsumer::render_direct_replay`
   (`consumer.rs:847`) are two near-duplicate `if let` chains that must agree
   about which invocations exist and what they render; `28F` already recorded
   one divergence between them (the w2a A5 widening). Adding a `dorc why` arm
   doubles the entry, in two places, with nothing mechanical holding them equal.
4. **`friction-to-editable-flushes-on-every-computed-byte`** —
   `to_editable_render` (`dorc-loom/src/lib.rs:75`) calls `flush_section` for
   every `RenderPart::Arrangement` / `ForeignText`, so ANY renderer-computed
   byte inside a logical prose line splits it into two sections. This is the
   mechanical root of `289:seam-multiword-chrome-render-only`, and it is a
   dorc-side rule, not an errorloom one — worth stating, because the seam is
   filed against the transport.
5. **`friction-refusal-keys-on-storage-not-on-stamp`** —
   `apply_arrangement_edit` refuses when the CURRENT entry stores > 1 word. So a
   row is un-editable because of what it holds rather than because of what the
   render stamped; a row that happened to be migrated as one word would silently
   become editable even if its seat interleaves values.
6. **`friction-foreign-key-shape-mismatch`** — `weft::Provenance::Foreign` keys
   on `Face::Source(String)` (a runtime path), while
   `tagged::RenderPart::ForeignText` keys on `param: &'static str`. A bridge
   cannot map one to the other without reshaping one of them.
7. **`friction-weave-words-drops-the-occurrence`** — `aid::weave::words(text,
   slug)` has no occurrence parameter, while 16 reached registry rows are
   occurrence-keyed. The occurrence is resolved at `why_words_at` and then
   thrown away, so the span map cannot carry it and `to_editable_render` would
   fall back to render position. Latent mis-attribution, invisible until the
   surface gets a face.
8. **`friction-new-row-unreachable-until-promote-and-rebuild`** —
   `arrangement_page` refuses a slug with no committed `ARRANGEMENTS` row
   ("promote the case, then rebuild"), which is the same lag `28F` banked for
   the catalog and for "promote requires the case committed-at-HEAD". Authoring
   a NEW why-surface case therefore needs a red lockstep step.
9. **`friction-no-normalizer-reaches-rendered-output`** —
   `seam-tolerated-nondeterminism-stops-at-the-run-log` still holds: the
   `tolerate:` vocabulary normalizes `expected.ran` only. A why transcript
   carrying `(received HH:MM:SS, rc N)` is made deterministic at the source
   (`DORC_FIXTURE_CLOCK_MS`), and there is no declared-class escape hatch if a
   future why surface acquires an honest nondeterminism.
