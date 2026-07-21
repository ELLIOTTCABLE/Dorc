# 282 — the transcript-case prose pipeline (`errorloom`)

AI-authored (Fable conductor, 2026-07-19, from the two-day design dialogue with the
human; every §0 ruling is human-typed unless marked as a lean). PLAN-OF-RECORD for an
implementor. Authority: root docs, `spike/CLAUDE.md`, root `AID-NEEDS.md` outrank.
Companions: `notes/27U` (aid as-built ledger) · `notes/27V` (evidence plane) ·
`notes/27W` (decline classes) · `plans/281` (authored mark grammar) · `plans/280`
(round charter).

Implementor read-first: root `README.md`/`DESIGN.md` → `spike/CLAUDE.md` (User-aid
law block + Boundaries + Build/test/run) → root `AID-NEEDS.md` (Law section) →
`spike/crates/errorloom/README.md` → current `errorloom` transport/bless code +
`dorc-loom` consumer/fixpoint code + `core` catalog/tagged-render code → this plan
whole. Use `notes/27U` §1/§4 only for the as-built provenance ledger.

## §0 — Ruling ledger (the design is settled; spellings inside are latitude)

Human-typed, 2026-07-18/19 sitting:

- **`282:rul-transcript-is-the-authoring-surface`** — the purpose of the whole
  machinery: humans AND LLMs authoring user-facing prose look ONLY at what a user
  sees — same headspace, same error-model, including carets, visible code, and what
  is NOT visible. Writing a user-facing string at line 700 of a monotone pass is the
  named footgun (analyzer-headspace: you know things the user doesn't). Therefore
  the committed, executable transcript CASE is the authoring surface, and the
  compiled catalog is DERIVED from it (`AID-NEEDS:law-defining-case-catalog`): the
  committed catalog intermediate regenerates only through explicit promotion, and
  the lag is the assertion.
- **`282:rul-new-code-empty-loop`** — minting order for every new code: builder
  mints the slug with NO prose → the render shows a loud placeholder → the author
  supplies executable, triggering input-data (the case) → conductor/human writes
  the words while LOOKING AT that render. Prose cannot exist without a case; the
  machinery functions as the templating engine that produces exactly the
  what-does-the-user-know report.
- **`282:rul-words-and-paragraphs-only`** — the only authored value is an ordered
  series of words, grouped into paragraphs (two-plus linebreaks = paragraph break).
  ALL other formatting is render-owned and slurped/discarded at read-in (LLMs wrap
  words strangely; that noise must die at the boundary). The model may grow later;
  it starts this small.
- **`282:rul-multi-replay-per-case`** — one case file may carry MULTIPLE replay
  blocks (`--verbose` / `--terse` / machine-format views of the SAME input-state);
  every replay in a defining case must surface the case's code-slug (the same-slug
  coherence gate).
- **`282:rul-passthrough-type-gated`** — passthrough prose (foreign text riding a
  `detail`-style hole) is gated by TYPE, not by lint: the hole's type is a
  tagged-at-the-I/O-layer user-sourced value, NEVER constructible from a string
  literal. This reuses machinery the codebase already believes in (user-sourced
  data is distrusted and tagged for an ocean of independent reasons); our own
  sentences physically cannot ride a passthrough.
- **`282:rul-own-crate-own-tests`** — the machinery is its own small crate with its
  own tests. Reuse outside Dorc is nice-not-priority: honor the cheap two-layer
  split (§5), invest nothing further.
- **`282:rul-internal-tool-sharp-edges`** — this is self-consumed internal tooling.
  It does NOT get the product's properties (gradual enhancement, friendliness,
  accountability). Sharp edges are fine; it just needs to WORK. Refusals may be
  blunt dumps. (Composes with `27V:rul-aid-survives-the-spike`: what is keeps-tier
  is the DATA — catalog, corpus, format — and the transport-correctness property;
  the tool's UX is not.)
- **`282:rul-arrangement-words-exempt-v1`** — walker-owned structure words
  (connectives, tier words, the epilogue frame) are exempt from transcript-editing
  at v1; they stay co-located in arrangement code. (Seam: §10.)
- **`282:rul-frontmatter-txtar-container`** — the container is txtar-with-
  YAML-frontmatter: structured metadata at the head, file/CLI state as txtar
  sections after. Precedent the human located: terraform-plugin-docs acceptance
  tests; always lean on prior art for formats and tooling.
- **`282:rul-git-repo-dependence-accepted`** — the tool may depend on running
  inside a git repository; the access mechanism is conductor latitude (taken:
  subprocess `git` behind a trait — §6).

Conductor leans, standing unvetoed:

- **`282:lean-machinery-now-prose-lazy`** — build the machinery now; burn prose
  down deliberately lazily (`sm `-tier text stays legal); the concentrated
  prose-quality sprint waits for a surface-stability moment (field-trial prep is
  the natural one). Rationale: r26 reshapes records/renders and block-stdlib mints
  many new codes — sentence-polish now is investment into wet cement; the
  machinery's value (the `282:rul-new-code-empty-loop`) starts immediately.
- **`282:lean-flat-frontmatter-subset`** — the frontmatter is a FLAT YAML subset:
  `key: value` string scalars (+ string lists if needed), delimited `---` fences;
  nested structures refuse. Keeps "very barely structured", dodges YAML footguns,
  keeps LLM-familiarity. Parser latitude: hand-rolled subset or a vetted dep.
- **`282:lean-terse-single-paragraph`** — terse-register fields gate to one
  paragraph; multi-paragraph is legal only in deep/prose registers. Cheap to
  change; the human's stated position was "idk".

## §1 — What the thing is (one screen)

A case file is one txtar archive: frontmatter metadata, embedded source files
(book, oracles), embedded world (probe results; later per-host), and one final
replay section holding one or more `$ dorc …` command lines each followed by
EXACTLY what that command prints. At regeneration time the commands are literally
executed in a materialized temp dir and the output is re-inlined.

Three actions stay mechanically distinct:

- **compile**: read dirty transcript prose, attribute it through renderer tags, and
  show the editor the exact template interpretation and concrete re-render. It writes
  only an ignored, content-bound receipt.
- **promote**: accept only that fresh compiled interpretation, atomically regenerate
  the catalog lock and every affected case, then prove the fixpoint.
- **structure-bless**: with transcript prose clean, regenerate cases after arrangement
  or engine changes. A touched-set spanning prose and structure refuses.

Prior art (steal, don't invent): Mercurial t-tests / cram (`$ cmd` + inlined
output, bless re-runs), Go txtar + testscript (archive + driven commands), rustc
`tests/ui` + `--bless` (corpus-scale golden discipline), insta (snapshot review
flow), terraform-plugin-docs (txtar + frontmatter marriage). The ONLY novel leg is
the diff-driven extraction back into the catalog (§5).

## §2 — The case-file format

```
---
code: render-heredoc-refused
when-fires: the leaf-exact render would elide a heredoc-bearing leaf
why: kFAIL-perform; arch-1 d-6
---
-- book.sh --
#!/bin/sh
cat <<EOF >/etc/motd
hello
EOF
-- probe-results.txt --
site 0 effect=holds
-- replay --
$ dorc plan --book=book.sh < probe-results.txt
render: error[render-heredoc-refused]: leaf-exact render refuses to elide ...
  = help: split the heredoc body to its own leaf, or mark the kind un-elidable
$ dorc plan --book=book.sh --format=jsonl < probe-results.txt
{"envelope":"dorc-lint-format/1", ...}
```

- **Frontmatter** (flat subset, `282:lean-flat-frontmatter-subset`): `code` (the
  defining slug; absent for non-defining corpus cases), `when-fires`, `why`.
  Params are NOT declared here — they derive from the typed payload and templates
  into the generated catalog lock. Keep the key set minimal; every addition is a
  format commitment.
- **File sections**: verbatim, LF-only (materializer pins LF; CRLF in a section is
  a regeneration-time refusal). Multihost convention (format-neutral, build nothing now):
  `hosts/<name>/probe-results.txt` section names; the replay command names hosts.
- **The replay section** (always last): a sequence of blocks; each block = one
  `$ `-prefixed command line + its inlined output (until the next `$ ` line or
  section end). Commands run SEQUENTIALLY in one materialized temp dir with a
  shared per-case scratch — required for run-then-`dorc why --last` sequences (the
  whylog flows between commands). Each command's spelling is the user-shaped
  invocation with case-relative paths; harness-only environment must not appear
  (framed records in fixtures, not `DORC_ALLOW_LEGACY_RESULTS`).
- **Used-variable replay**: every defining case commits a generated `dorc-loom vars
  --used CASE` block before its diagnostic replay. Its output names the defining
  code/field and lists only variables used by editable prose, in first-use order,
  beside their exact rendered values. It is editor aid, derived data, and never an
  authority.
- **Machine-format replays**: a replay whose render carries no human prose regions
  (e.g. `--format=jsonl`) is whole-block structural — never prose-editable, always
  regenerated. This is how machine-envelope coverage lives in the same case as the
  human views (`282:rul-multi-replay-per-case`), replacing the retired fragment
  goldens (§8).
- **Coherence gates**: a defining case's every replay must surface its `code` slug
  at least once; a case whose replay output contains a line that parses as a txtar
  marker refuses at bless (no escaping exists — sharp edge, acceptable).
- txtar caveats honored: newline-termination on round-trip; text-only. Rust `txtar`
  crates exist but are ~100 lines to own — vendor-or-implement is builder latitude
  with a one-line rationale.

## §3 — The prose model (words and paragraphs)

- Authored value = ordered paragraphs of ordered words. Read-in normalization:
  within a paragraph every whitespace run (including single newlines) collapses to
  one space; two-plus consecutive newlines = one paragraph break; nothing else
  exists. Catalog prose fields store the normalized form.
- The renderer owns ALL layout: wrapping (committed transcripts render at ONE
  pinned canonical width), indentation, paragraph spacing, connectives. Live
  surfaces may wrap adaptively; the corpus does not.
- Registers: `282:lean-terse-single-paragraph`.
- Heuristic re-holing is optional and deliberately narrow (§5/§13). Untouched
  variables preserve their renderer identity regardless of their rendered bytes;
  only values eligible for heuristic rediscovery need satisfy its conservative
  length and charset floor.

## §4 — The tagged render

The renderer fills templates at one seat. Alongside the bytes it emits a generic,
consumer-neutral tree:

- **Structure** — numbering, connectives, carets, gutters, excerpts, and layout;
  immutable under prose editing.
- **FixedVariable** — typed rendered data outside editable prose, including
  passthrough/foreign text; immutable and never template-movable.
- **EditableSection(section-id)** — one renderer-stamped prose instance containing
  an ordered series of **Text** and **Variable(variable-id, rendered-value)**.

Section and variable IDs are opaque to errorloom. The map, not a diff heuristic, is
the attribution authority; word diffing only aligns edits within one section.
Nothing here leaks into product renders: tags ride beside identical bytes.

## §5 — The transport engine and consumer boundary

Layer 1, zero Dorc types: errorloom transports edits over the §4 tree. It preserves
untouched variables by identity before tokenization, confines edits to one editable
section, refuses structure/fixed-variable/cross-section changes, and hands a consumer
the touched section plus opaque variable IDs. It does not parse template syntax,
discover variables, or own a catalog. Stop abstracting there
(`282:rul-own-crate-own-tests`).

Layer 2 is consumer policy. `dorc-loom` parses the strict `{{name}}` subset, resolves
names against Dorc payload values, applies the conservative same-section re-holer,
and compiles the resulting `Text | Variable` series into catalog fields. It alone
enforces defining-case ownership and applies edits to the catalog lock.

- Unchanged variables survive with arbitrary bytes, punctuation, and mixed/glued
  spans. Only a hunk touching a variable or its adjacent fixed text requires an
  explicit consumer marker.
- Insertions at section boundaries, cross-section variable movement, edits to
  structure/fixed variables, ambiguous re-holing, and contradictory interpretations
  of one template refuse.
- Refusals are blunt (`282:rul-internal-tool-sharp-edges`): dump the section streams,
  region table, interpretation, and offending hunk; never fuzzy-match.
- The hard-tested generic property: any unrelated text edit preserves every untouched
  variable identity, and consumer compile → regenerate → re-render reproduces the
  accepted edited words modulo whitespace normalization. Property-test mixed/glued
  spans and every refusal class.

## §6 — Compile, promote, and git gating

- Dependence: Dorc promotion requires a git repository
  (`282:rul-git-repo-dependence-accepted`); generic errorloom run/regenerate works
  anywhere. Git access remains behind the small injectable trait.
- **Mode inference**: case-prose edits plus a byte-clean catalog lock may compile;
  structure edits plus clean case prose may structure-bless. Mixed classes or a
  hand-edited generated lock refuse.
- **`dorc-loom compile CASE...`**: produce the exact interpreted template series,
  variable bindings, refusals, and concrete re-render; write only an ignored receipt
  bound to case bytes, catalog input, consumer/tool semantics, and touched set.
- **`dorc-loom promote CASE...`**: require that fresh receipt, recompute and compare,
  then atomically overwrite the generated catalog lock and all affected cases. Any
  failure leaves committed files byte-identical. The review surface is the git diff.
- **Structure-bless**: regenerate cases from a clean catalog lock after renderer or
  arrangement changes; it never consumes transcript prose edits.
- **CI fixpoint**: compilation/promotion over the committed corpus reproduces the
  committed catalog lock and cases byte-identically.

## §7 — The execution harness

- Materialize sections to a temp dir (LF pinned); run replay commands in order
  with a controlled environment (`env -i`-style, PATH pinned to the built `dorc` +
  the case's inert mocks; cwd = the temp dir so paths render RELATIVE — absolute
  host paths in a transcript are a regeneration-time refusal); capture combined output
  (`2>&1`) v1 — a command wanting stream separation spells its own redirection in
  the command line. Flag: combined-capture interleaving is deterministic only
  because dorc's surfaces are effectively single-stream per invocation; if that
  ever breaks, split-capture is the escape hatch.
- Safety: identical rails to the e2e harness (inert mocks only, no real mutators,
  worktree-local, no network). Case execution is git-free.
- Determinism: same DST discipline as e2e; committed transcripts are byte-stable
  under re-execution or the bless refuses. Multihost transcripts additionally
  require deterministic cross-host output ordering — a named CHECK on the r26
  reactive work (`26B` confluence targets plans, not stderr streams), not work here.

## §8 — Dorc integration (the adapter, and what changes at home)

- **The catalog table is fully generated**: prose fields, metadata (frontmatter),
  and params/example (typed payload machinery) live in one wholly generator-owned,
  committed Rust target (`catalog_lock.rs`, spelling latitude). Promotion overwrites
  the whole file; handwritten catalog machinery lives elsewhere. The corpus is the
  only prose edit surface.
- **Transitional carry-forward**: promote sources prose from cases-where-they-
  exist ∪ current-catalog-prose-where-not, so case-less codes keep their
  `sm `-tier prose mid-migration. THE RATCHET (as-built:
  `DEFINING_CASE_RATCHET`, a shrink-only allowlist gate — entries may be removed,
  never added, so coverage only grows) is REDEFINED to "codes whose prose is not
  yet case-owned"; the completeness gate stays covered ∪ ratchet == all slugs.
- **Placeholder semantics**: `[unwritten: <slug>]` stops being a stored string;
  an absent prose field renders the placeholder at render time
  (`282:rul-new-code-empty-loop`). The `sm ` migration markers survive as ordinary
  words until rewritten — at the transcript surface.
- **Template syntax**: stored catalog holes and dirty transcript markers use only
  `{{name}}`; single braces are literal text. The committed transcript and used-variable
  inventory render concrete values, never template markers.
- **Roster retirement**: builders still author zero prose
  (`27V:rul-error-authorship-tier` stands); enforcement moves from the
  `CONDUCTOR_AUTHORED` gate-roster to promote-privilege (BLESS-law:
  orchestrator-only) + the §6 fixpoint gate.
- **`282:rul-passthrough-type-gated` work**: mint (or extend `OutBytes` into) a
  user-sourced text type under the sealed-room pattern (`core::room` precedent) —
  constructible ONLY at I/O edges (parser input relays, tool stderr, host bytes);
  passthrough catalog holes type to it; string literals cannot reach it. Audit
  every current `detail: String` payload: genuinely-foreign relays wrap at their
  edge; sentences composed at emit sites from OUR words de-passthrough into real
  templates (world-variant siblings where needed —
  `AID-NEEDS:law-codes-vary-by-world-not-grammar`). Convergence note: this same
  type is what `an-output-sanitization` will key on later — the taint tag does
  double duty.
- **The final surface excludes**: unit-tier fragment goldens +
  `DORC_DEFINING_BLESS`; the roster; the stored-string placeholder; the env-gated
  prose-promote test; `target/catalog-promoted.rs`; manual catalog splicing;
  single-brace holes.
  **What stays**: the `DiagCode` enum + typed payloads + registry; `diag_tidy`'s
  emit-site gate (the fires-half backstop) — though defining cases now REALLY
  fire, closing `27U:finding-corpus-blind-edge-codes`; the e2e corpus and its
  plan-render goldens (different product surface); machine-envelope shape
  assertions (move to a machine-format replay block or stay unit-tier; latitude).
- Registry/law sync (root `AID-NEEDS.md` law wording, `spike/CLAUDE.md` aid block)
  rides the INTEGRATION landing, not this plan-mint — one sync commit when the
  direction is built truth, not paper truth.

## §9 — Phases (the implementor's ladder; serial, each gated)

1. **`282:phase-generic-editable-sections`** — errorloom's nested generic region
   model, identity-preserving untouched-variable transport, consumer compilation
   seam, and adversarial mixed/glued-span tests.
2. **`282:phase-dorc-template-compiler`** — double-brace grammar; section-local
   movement/removal/duplication; conservative re-holing; committed used inventory;
   optional easy current-payload unused-variable insertion.
3. **`282:phase-compile-promote-loop`** — interpretation render + bound receipt;
   direct atomic catalog-lock/case generation; legacy promote paths absent.
4. **`282:phase-command-embed-dogfood`** — perform the motivating command-variable
   reorder through the case alone; pin compile output, used inventory, canonical
   transcript, stale-receipt refusal, and whole-file lock overwrite.
5. **`282:phase-adjacent-fragment-followup`** — punctuation/backtick adjacency for
   newly-positioned markers and the broader glued-variable seam; explicitly after
   the whole-token loop, never silently approximated in it.

Dispatch follows the standing safety, map-then-execute, comment-budget, gate, and
foreground-verification law. The generated-lock ownership change gets a proposal/go
checkpoint before execution.

## §10 — Out / deferred (named seams)

- Arrangement-prose promotion (connectives/tier-words as transcript-editable
  entries) — the kFLOW seat; v2, after the corpus exists to argue over.
- gix/library embedding; SARIF-style exports; any reuse packaging beyond the
  layer split.
- Split-stream capture; TTY-adaptive width in transcripts.
- Multihost transcript determinism — the r26 check (§7).
- The prose-quality sprint — scheduled at a surface-stability moment, not here.
- The prose-register schema (terse/deep/first-encounter) + remediation-hint prose
  migration (`27U` §7 item 7) — pairs with the human's slow `sm `-rewrite pass;
  wants its own short sitting once transcripts exist to stare at.

## §11 — Confidence

+SURE: the as-built inventory; single-seat render fill; human-typed product boundary,
double-brace, untouched-variable, compile-before-promote, and generated-lock rulings.
~SUSPECT: the generic nested transport remains small; current-payload unused-variable
insertion is nearly free; a content-bound receipt is sufficient to prevent unseen
promotion. -GUESS: phase sizing; the generic/Dorc split may expose one additional
adapter seam during implementation.

## §12 — Follow-up: human-directed flagship-render polish (rider, 2026-07-20)

Prompted by the flagship case (`crates/dorc-loom/cases/cmdsub-operand-top.txt`) and
the human's hand-edited target render (`e6edf5e`). These refine the RENDER FORM
(`AID-NEEDS:kFLOW` render-form-unwelded — exactly the "resting point decidable only
from real generated output" this rider IS) and the errorloom PROSE MODEL (§3). Being
executed live under human steer on `ai/r28-flagship-polish`; each is a small cleanup,
PUNT-if-invasive (human-sanctioned). Grep-anchor: `flagship-render-polish`.

Render-form (touch `core::diag` render_cli / caret plumbing — Dorc-side, cousins of
`AID-NEEDS:kFLOW`):
- **span-caret `\__/`** (LANDED) — a source-SPAN (AST region, start+end) underlines
  with `\`+`_`…`_`+`/`, not `^^^`. A single lexeme keeps `^`; a secondary span keeps
  `-`. Three fixed forms — no style-system.
- **invocation-global gutter** (`28A:rul-gutter-width-invocation-global`) — width
  `W = max(3, maxDigits)`, `maxDigits` = digits of the LARGEST line-number rendered
  ANYWHERE in the invocation (all replay blocks), so code columns never shift between
  blocks; `|` sits at column ≥ 4 always. Placement: right-align each number in W by
  default (ones-places line up, rustc-standard); when every rendered line-number
  shares ONE digit-width ≤ 2, apply the slack aesthetic — 1-digit CENTERED (` 6 |`),
  all-2-digit LEFT-aligned (`60 |`). ≥3 digits fill (`600|`, `6000|`); mixed widths
  right-align (`  6|` beneath `600|`, `   6|` beneath `6000|`). AS-BUILT (`77ebd8e`):
  `maxDigits` is computed per replay-frame, NOT threaded invocation-global — identical
  for the single-frame corpus cases; true cross-frame threading is deferred (`28A`
  §2z-post-2).
- **`{{command}}` — typed, not a bare string** (`28A:rul-command-name-typed-three-state`):
  a command-name type expressing static-literal / dynamic-but-const-prop-resolved /
  no-single-clear-name, threaded from the analysis site where value-flow is known (not
  synthesized late). Render: literal → the name; resolved-dynamic → "This dynamic
  command-word, which resolves to `apt-get`, …"; unresolvable → fallback phrasing (no
  `{{command}}` fill). Literal path end-to-end now; the dynamic variants get the TYPE +
  RENDER shaped now, analysis-population may be a marked follow-up.
- **`= repair:` not `= help:`** (`28A:rul-connective-minimal-remediation-map`) — keyed
  on registry `RemediationClass`, MINIMAL for now: `ResolveDynamism`→"repair", all else
  "help". Core-side (production `dorc plan` says `repair` too); the fuller class→word
  map is tuned iteratively as errors surface — none exists yet.
- **following-ness punctuation — DEFERRED** (`28A:rul-following-ness-deferred-punt`):
  renderer-owned terminal `:`/`.` (`:` when a block follows, `.` when terminal, prose
  authored without it) is the desired architecture but PUNTED this pass — it risks the
  compile/promote byte-equality invariant for a refinement the flagship already gets right
  with a baked `:`. Mechanically-composed grammar stays refused regardless
  (`AID-NEEDS:law-codes-vary-by-world-not-grammar`).

Errorloom render (`render_case`, corpus surface — NOT core, so the tagged-twin
byte-equality gate stays untouched):
- **canonical wrap** (LANDED) — committed case prose is hard-wrapped + indented for
  editing (message continuations 3-sp, repair 6-sp); `render_case` reflows stored
  prose to a pinned canonical width, so source-wrap is invisible. Read-in collapses
  `\n`+indent → one space (§3). Applies to ALL prose-bearing cases (16), not just the
  flagship (human: nicer to edit in-editor).
- **inter-block blank line** (`28A:rul-blank-line-is-errorloom`) — a blank line after
  a replay block when another block follows (not trailing). An errorloom presentation
  choice, NOT dorc-production output; a future dorc beauty-newline would stack to a
  double blank, accepted.
- **txtar section spacing** (`28A:rul-blank-line-is-errorloom`) — a blank line before
  each `-- header --`, separating it from the end of the previous body-text. errorloom
  emits it on `render_case` and tolerates it on read-in; same family as the inter-block
  blank, applies to every case (book-body → `-- replay --`, etc.).

Not code: the human also rewrote actual prose ("Dorc" → "I", etc.) — voice, no
mechanics owed.


## §13 — Follow-up: compiled template-edit loop (polish ledger, 2026-07-21)

Human-directed polish after first use exposed the minimum useful variable-edit loop.
Spellings below are ruled unless marked latitude.

### Product boundary

- **`282:rul-errorloom-enables-template-consumers`** — errorloom remains a pleasant
  standalone product and does NOT acquire Dorc's template language, parameter
  discovery, catalog generator, or authoring policy. It owns the generic transport:
  renderer-stamped `Structure | FixedVariable | EditableSection`, with an editable
  section containing an ordered `Text | Variable` series; exact section/variable
  identity; untouched-variable preservation; edit attribution; and consumer hooks
  sufficient to compile an edited section. Variable IDs and compiled fragments are
  opaque to errorloom. This is eager support for template consumers, not a fossilized
  templating methodology.
- **`282:rul-dorc-loom-owns-template-policy`** — `dorc-loom` owns the concrete
  `{{name}}` grammar, Dorc payload lookup, used/all inventories, conservative
  re-holing policy, interpretation display, catalog compilation, and the
  compile/promote commands. The generic errorloom CLI stays generic; the Dorc adapter
  is the product-specific authoring harness.

### Template and edit model

- **`282:rul-double-brace-template-only`** — one spelling everywhere templates are
  visible or stored: `{{command}}`. Single-brace holes are removed, with no
  compatibility path. v1 is a strict Mustache-shaped subset:
  `{{` + `[A-Za-z_][A-Za-z0-9_]*` + `}}`, as one whole token; no whitespace,
  expressions, paths, filters, nesting, or attached punctuation. Literal single
  braces are ordinary text. Unknown/malformed double-brace forms refuse.
- **`282:rul-variable-edit-section-scope`** — a variable may be preserved, removed,
  duplicated, or moved only within its renderer-stamped editable section. Moving or
  adding an occurrence uses `{{name}}`; removing an occurrence omits it. Cross-section
  movement refuses. The committed transcript remains rendered values only.
- **`282:rul-untouched-variable-preservation`** — unrelated prose edits NEVER require
  retyping variables, regardless of a variable's bytes, length, charset, punctuation,
  or adjacency. An untouched variable is preserved by renderer identity before word
  tokenization, not rediscovered from its rendered value. This invariant lands in the
  first pass, including mixed-span/glued words. If an edit touches or overlaps the
  variable or its immediately-adjacent fixed text (remove a backtick; change
  `apt-get` into `apt-get yourarg`), the author must write `{{command}} yourarg`.
  Full spacing/glue support for newly-positioned markers remains the next phase;
  unsupported adjacency refuses rather than emitting spacing-corrupted output.
- **`282:rul-rehole-deliberately-stupid`** — heuristic re-holing is only an aid for a
  rendered value already present in the SAME editable series. It is exact, anchored by
  surviving text, minimum-length/charset-gated, and never fuzzy, substring,
  cross-section, or project-global. Destroyed anchors require an explicit `{{name}}`.
  Exact thresholds are builder latitude, chosen conservatively and pinned in tests.

### Variable discovery

- **`282:rul-used-inventory-is-committed`** — each defining case commits a separate,
  generated replay entry listing only variables used by that case's editable prose,
  with their exact rendered values (`{{command}} => 'apt-get'`). It is derived from
  the tagged render, ordered by first use, regenerated on promotion, and never an
  authority. It deliberately sits above/beside the flowing diagnostic so the editor
  knows which rendered words require marker-preserving edits.
- **`282:rul-full-inventory-is-pulled`** — `dorc-loom vars --all CASE` is an explicit
  out-of-flow query for the full typed payload inventory. Introducing a variable not
  yet used in this case is an opportunistic first-pass goal ONLY when the current
  diagnostic payload already supplies an ordinary typed value and accepting it is a
  small extension of the same resolver. Project-global borrowing, payload growth,
  passthrough/foreign values, or new Rust data remain out; those still require Rust.
  The used inventory is never widened merely because `--all` can see more.

### Compile, inspect, promote

- **`282:rul-compile-before-promote`** — editing is compiler-shaped, not direct bless.
  `dorc-loom compile CASE...` reads dirty transcripts and prints exactly how every
  touched editable section was interpreted: the resulting `Text | Variable` series,
  variable bindings, refusals, and the concrete re-rendered user view. It writes no
  committed source. Complex interpretation is never silently accepted.
- **`282:rul-promote-requires-fresh-compilation`** — `dorc-loom promote CASE...`
  accepts only a prior successful compile receipt bound to the exact case bytes,
  catalog input, consumer/tool semantics, and touched set. A stale or absent receipt
  refuses; promote may recompute and equality-check but never substitute an unseen
  interpretation. The receipt is an ignored cache under `target/`, distinct from the
  committed catalog lock.
- **`282:rul-promote-is-one-atomic-act`** — promotion atomically writes the generated
  catalog and all affected canonical cases, then runs the render/promote fixpoint.
  Failure leaves committed files byte-identical. No env-gated Cargo-test entry,
  `target/catalog-promoted.rs`, or manual Rust splice remains. Default leaves a
  reviewable git diff; explicit staging is optional follow-up latitude, never implicit.
- **`282:rul-catalog-lock-is-generated-whole`** — the catalog table moves into one
  wholly generator-owned Rust compilation target (spelling latitude:
  `catalog_lock.rs`). Promotion ignores and overwrites its entire contents; user edits
  never coexist with generation. Handwritten catalog machinery remains elsewhere.
  The committed lock is diffable build output and fixpoint-checked, not an authoring
  surface or runtime verdict cache.
