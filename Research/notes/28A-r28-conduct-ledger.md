# 28A - errorloom implementation plan and current state

Current-truth implementation plan, rewritten 2026-07-21. This document is not a
chronological ledger. Git preserves the build history; `plans/281`, `plans/282`, and
the root documents preserve the design argument. Keep this file short, current, and
usable by the conductor actively completing errorloom.

Authority order: human-authored root docs and human-typed rulings -> `spike/CLAUDE.md`
-> `plans/281` / `plans/282` -> this implementation plan. `Research/LIVING_STATUS.md`
is the current branch map, not design authority.


## 0. Goal and present position

The product goal is `282:rul-transcript-is-the-authoring-surface`: user-facing prose
is authored while looking at the complete executable transcript a user sees. The
catalog is generated, committed build output, never the prose-editing surface.

The immediate hot-loop goal is stronger: an author can add, move, duplicate, or
remove an already-available typed diagnostic variable directly in a transcript,
inspect the compiler's interpretation, and promote it without editing a Rust catalog
file.

Current position:

| phase | state | result |
|---|---|---|
| Round-28 foundation | COMPLETE | standalone errorloom, syntax v0.2, defining-case corpus, generation-flip/fixpoint foundations |
| `282:phase-generic-editable-sections` | COMPLETE | bounded identity-preserving generic edit transport |
| `282:phase-dorc-template-compiler` | COMPLETE | strict `{{name}}` compiler over the current typed payload; in-memory apply and preview |
| `282:phase-replay-driver-provenance` | NEXT | consumer-neutral driver/result API; Dorc exact-shape dispatch; generic bytes-only fallback; Unit-1 CLI rework |
| `282:phase-compile-promote-loop` | PENDING | receipt, touched-set gate, preflighted generated catalog/case publication |
| `282:phase-command-embed-dogfood` | PENDING | add `{{command}}` through the flagship transcript and prove the complete workflow |
| `282:phase-adjacent-fragment-followup` | DEFERRED | newly-positioned attached/glued markers and broader punctuation support |

Current phase-two closeout base: `ai/r28-errorloom-phase2` at `4b95a9b4`.

Phase-two lineage starts from the phase-one fold at `0c259317`; accepted code closed
at `01523f2c`; current plan/README/steering synchronization closed at `4b95a9b4`.
Preserve the granular history. Do not squash or re-derive it.

Phase-three Unit 1 is in rework on `ai/r28-phase3-unit1-cli` at `c4957366`. Its
argument parsing, inventory queries, and multi-section preview work remain useful;
its replay selection by output skeleton/prefix/template-looking bytes is REJECTED.
No receipt work begins until the replay-driver provenance seam replaces that logic.


## 1. Settled architecture

### `28A:rul-generic-transport-ownership`

errorloom owns only consumer-neutral machinery:

- txtar/frontmatter case parsing, replay execution, hygiene checks, and the generic
  `errorloom run` / structure-bless surface;
- a replay-driver/result API whose handled result carries exact output bytes plus
  optional typed editable provenance, and a reusable controlled shell/process
  executor; decline never silently selects that executor in the library API;
- `EditableRender<SectionId, VariableId>`, containing ordered `Structure`,
  `FixedVariable`, and `EditableSection(Text | Variable)` components;
- bounded edit attribution, untouched-variable identity preservation, unique
  minimum-removal inference, refusal evidence, structure regeneration, and the
  render-level fixpoint gate;
- a small injectable repository-state trait sufficient to reject dirty generated
  catalogs and dirty transcript cases during structure-bless.

errorloom does not own Dorc template syntax, payload discovery, catalog policy,
catalog serialization, command names/flags, JSONL semantics, consumer dispatch
policy, or durable promotion. It never infers editability from command category or
output contents. The deleted tagged-region/
parameter-table/re-holing/prose-bless stack must not return. The nested editable
transport is its replacement, not a compatibility sibling.

### `28A:rul-dorc-template-policy-ownership`

dorc-loom owns all Dorc-specific authoring policy:

- exact-shape recognition and in-process driving of supported Dorc replay
  invocations, plus explicit routing of declines to a tightly configured generic
  executor;
- association of each exact replay result with renderer-produced editable
  provenance where available; handling an invocation and exposing prose are separate
  capabilities;
- conversion from core-owned `RenderParts` into errorloom editable sections;
- strict whole-token `{{name}}` parsing;
- resolution against the current diagnostic's ordinary typed payload;
- separate used-variable and full-payload inventories;
- section compilation, inspectable previews, catalog field application, case/corpus
  regeneration, and the phase-three command workflow.

core owns the typed diagnostic payload, the single production render seat,
`RenderParts`, catalog data structures, and catalog serialization. core does not
depend on errorloom.

### `28A:rul-template-marker-grammar`

The visible/stored template spelling is exactly `{{name}}`, where `name` matches
`[A-Za-z_][A-Za-z0-9_]*`.

- The marker is one whitespace-delimited token.
- Whitespace, paths, filters, expressions, nesting, and attached punctuation are not
  accepted.
- Single braces are literal text; there is no compatibility parse for old catalog
  `{name}` holes in the final generated surface.
- Unknown, malformed, attached, cross-section, or contradictory interpretations
  refuse visibly.
- A marker may name any ordinary value already present in the current typed payload,
  even when the current catalog template does not use it.
- Foreign/passthrough values are not available to template compilation.

The marker name is the authority for a newly introduced occurrence. Rendered bytes
are never sufficient to infer a new typed variable.

### `28A:rul-variable-edit-semantics`

- Unrelated prose edits preserve every untouched variable by renderer identity before
  tokenization, regardless of empty, NUL-containing, Unicode, punctuation-heavy,
  repeated, or glued rendered bytes.
- An occurrence may be removed by omission, or moved/duplicated with an explicit
  marker, only inside its renderer-stamped editable section.
- Multiple equal rendered values with different IDs remain distinct. Ambiguous
  minimum interpretations refuse instead of choosing iteration order.
- Surrounding prose may change the byte offset of an untouched rendered variable
  without requiring `{{name}}`; unchanged immediate anchors and fragment ordering
  preserve its identity. A rendered value may relocate without a marker when the
  same-section interpretation is unique. Markers are the fail-clear fallback for
  destroyed anchors, ambiguity, duplication/new occurrence, or cross-section moves.
- Successful compilation produces an ordered `Text | Variable` series, first-use
  used-variable inventory, exact bindings, and a concrete re-render.
- Applying a compiled field derives catalog params from compiled message/help holes;
  authors do not update a hand-maintained params allowlist.

Current conservative limits are intentional:

- fields split by immutable arrangement/fixed components refuse as
  `SplitEditableField` until whole-field reconstruction is designed;
- new attached/glued marker positions wait for phase 5;
- paragraph addition/removal remains outside the v1 words-and-paragraphs model;
- there is no global, cross-section, fuzzy, or value-substring re-holer.

### `28A:rul-case-and-catalog-authority`

- Every user-facing diagnostic code has one defining case or remains on the shrinking
  case-ownership ratchet.
- Builders mint codes and structured cases with empty prose. They do not author
  user-facing error text.
- `[unwritten: <slug>]` is synthesized from `None` at render time; it is not stored
  prose.
- `sm ` prose remains legal migration residue until the dedicated prose pass.
- Every replay in a defining case surfaces its own code slug. Any replay result
  without typed editable provenance is structural and never prose-editable, whatever
  its command or format. A machine renderer may expose editable regions only by
  returning provenance for that exact result.
- The committed catalog lock is generated output. Hand-editing it is always refused
  by the workflow or caught by a fixpoint gate.


## 2. Completed phase-two contract

Phase two is accepted and has one complete green verification run.

Implemented:

- bounded disjoint-hunk alignment and attributed refusals in errorloom;
- inclusive variable-boundary treatment and unique minimum-removal search;
- seeded owning-layer properties covering mixed/glued/empty/Unicode/repeated values,
  equal-cost ambiguity, every refusal class, and resource ceilings;
- core `RenderParts` as the sole provenance stream;
- Dorc editable baseline with exact section/occurrence identities;
- strict marker compilation, current-payload insertion, used/all inventories,
  deterministic preview, in-memory catalog apply, and concrete re-render;
- structure-bless refusal for dirty generated catalogs or dirty transcript cases;
- removal of both Dorc and generic legacy tagged-promotion interpreters.

The accepted phase-two contract ends at the exact `EditableRender` and its transport.
It does not license locating that render inside replay output by matching skeletons,
prefixes, JSON shapes, command names, or `{{...}}` bytes. The replay-driver seam is
phase three's new prerequisite and leaves the phase-two transport intact.

Important repair commits:

- `dcbb1555` - current typed payload insertion and derived params;
- `53ea1591` - legacy promotion stack removal;
- `9d2fb9e` - dirty catalog structure-bless refusal;
- `f1c2b37c` / `ae302cda` - generic transport property restoration and lint repair.

Verification at phase-two close:

- fresh WSL workspace build: PASS;
- workspace tests: PASS;
- foreground e2e: 97/97 PASS;
- fmt: PASS;
- cold clippy with warnings denied: PASS;
- cargo-deny licenses/bans/sources: PASS;
- typos: PASS, apart from the known historical-hash false positive when scanning this
  planning corpus directly.

Environment lesson: build the WSL-native `target/debug/dorc` immediately before WSL
e2e. If only the Windows `.exe` exists after package-clean, WSL environment propagation
differs and the flag self-test can conservatively report identical plans. The harness
already prefers the native binary; do not weaken the semantic assertion.


## 3. Phase-three target behavior

Three user actions remain mechanically distinct.

### Compile

`dorc-loom compile CASE...` passes every replay through the Dorc consumer router.
Direct supported invocations are driven in-process; declined shell forms use the
explicitly configured generic executor and return bytes only. Compile compares dirty
transcript prose only against typed provenance attached to that exact replay result,
transports and compiles each touched editable section, and prints:

- the interpreted `Text | Variable` series;
- exact variable bindings and used-variable order;
- every refusal with bounded attribution evidence;
- every bytes-only replay as tested but non-editable;
- the complete concrete user view that the interpretation would regenerate.

Compile changes no committed source. After unit 2 it writes only an ignored,
content-bound receipt under `target/`.

### Promote

`dorc-loom promote CASE...` accepts only a successful, fresh compile receipt. It
recomputes the interpretation, requires exact equality with the receipt, precomputes
the wholly generated catalog lock and affected canonical cases, and runs both
fixpoint gates before publishing. Validation failure leaves committed files
byte-identical. Final per-target temp-file replacement is not a crash transaction;
interruption is loud in git and recoverable by rerun or git.

### Structure-bless

Structure-bless regenerates cases after renderer/arrangement/code changes. It requires
clean case prose and a clean generated catalog. It never consumes prose edits.


## 4. Phase-three implementation units

Use three fresh builders. Each builder owns code, focused compilation/tests, granular
commits, and its final foreground verification. The conductor owns rulings, checkpoint
adjudication, generated-output review, and the final integrated gate.

### Unit 1 - read-only command and inspection surfaces

Status: `[~] rework required on ai/r28-phase3-unit1-cli`

Build:

- a consumer-neutral replay-driver/result seam in errorloom: original command text +
  materialized context -> decline OR exact bytes plus optional typed
  `EditableRender` provenance;
- a reusable controlled generic shell/process executor in errorloom, selected only
  by explicit embedding policy;
- a Dorc router that claims only exact supported direct invocations, drives them
  through production render seats, and routes declines to a configured generic
  fallback; pipelines such as `dorc plan --format=jsonl | jq --pretty` are bytes-only
  unless a future transformation-aware driver preserves provenance;
- a thin `dorc-loom` binary or equivalent crate-local command entry;
- `compile CASE...` over driver-returned provenance and the existing pure compiler,
  initially preview-only;
- `vars --used CASE` and `vars --all CASE` queries over the existing deterministic
  inventories;
- deterministic, blunt output showing sections, fragments, bindings, concrete render,
  and refusal evidence;
- case selection and corpus-loading boundaries without writing source files.

Delete/reject the current `matches_editable_skeleton`, `resembles_diagnostic`,
first-human-replay, command-category, output-prefix, and template-looking-byte
selection strategies. Exact driver provenance is the only edit-authority input.

Acceptance:

- the flagship-shaped case can preview insertion of `{{command}}` without a Rust
  catalog edit;
- `You called the command \`apt-get\` first` -> `You called \`apt-get\` first`
  preserves the existing `command` variable without braces because its immediate
  anchors and identity survive despite the changed byte offset;
- one case may carry a direct editable Dorc replay, an in-process bytes-only machine
  replay, and a generic-executed pipeline; all are tested, only the exact
  provenance-carrying result is prose-editable;
- no command or output-content inspection grants edit authority;
- used inventory is first-use ordered and contains only variables used by editable
  prose;
- full inventory contains only ordinary values present in that case's current typed
  payload;
- foreign values and unknown names remain unavailable;
- the command has no hidden write, git mutation, network, clock, or randomness;
- all command output is deterministic and tested structurally, not by unstable prose
  accidents.

Do not finalize receipt or generated-lock formats in this unit.

### Unit 2 - content-bound receipt and touched-set gate

Status: `[ ] pending`

Build:

- the ignored receipt written by successful compile;
- binding to exact case bytes, catalog input bytes, compiler/consumer semantics,
  touched case set, each exact replay result/provenance, and each interpreted
  section/result;
- a collision-resistant content identity suitable for real files. Fixed IDs, FNV,
  width-one spike identities, or path-only keys are not acceptable;
- git/touched-set classification enforcing prose-only compile/promote and rejecting a
  dirty generated catalog or mixed prose/structure changes;
- promote-side recomputation and exact receipt equality checks;
- stale, absent, cross-case, cross-catalog, or semantics-mismatched receipt refusals.

Acceptance:

- changing any bound input after compile makes promote refuse;
- unrelated dirty structure/code cannot be silently combined with transcript prose;
- receipt parsing is bounded and typed; malformed or oversized input refuses;
- compile receipt creation is atomic and worktree-local;
- no promote path can proceed from an interpretation the editor did not inspect.

The digest/dependency and receipt schema are design-bearing. The builder maps the
available choices and flags a `tc-*` call before selecting a new dependency or public
format if the answer is not already mechanically forced.

### Generated-lock checkpoint

Status: `[ ] conductor review required before Unit 3`

Before implementation, map the exact split between:

- handwritten catalog types, lookups, formatters, and serializer machinery; and
- the wholly generated table containing code linkage, case-derived metadata, params,
  examples, message, and help.

The checkpoint output is a mechanical file/field ownership map and proposed
preflight/publication set. The conductor rules it before any source migration. No handwritten prose or
metadata may share the generated target after cutover.

### Unit 3 - preflighted promotion and generated lock

Status: `[ ] pending after checkpoint`

Build:

- the wholly generated catalog compilation target (`catalog_lock.rs` spelling is
  latitude); handwritten machinery remains elsewhere;
- complete candidate generation and both fixpoints before publication, then
  per-target temp-file replacement of the generated catalog and every affected case;
- committed `dorc-loom vars --used CASE` replay output before each defining
  diagnostic replay;
- canonical case regeneration and both fixpoint gates;
- retirement of manual splicing, `target/catalog-promoted.rs`, env-gated writers, and
  every old durable-promotion entry;

Acceptance:

- every validation or verification failure before publication leaves all committed
  targets unchanged; publication interruption is loud and git-recoverable, not hidden
  behind transaction machinery;
- the used-variable replay is generated, non-authoritative, first-use ordered, and
  byte-stable;
- promotion overwrites the entire generated lock, never patches handwritten regions;
- committed corpus plus generated catalog reproduce byte-identically;
- the git diff is the complete human review surface.


## 5. Verification and review law

Per builder commit:

- inspect status and diff; stage only intended files;
- run focused tests for the changed layer;
- use the commit skill and granular `(AI ...)` commits; preserve discovered bugs and
  their repairs as separate history;
- never push.

At every unit close:

- run review-pass for substantive code;
- run package-clean or cold clippy so stale incremental state cannot serve a false
  clean result;
- run the unit's focused tests and relevant integration tests foreground;
- record any `tc-*` decision rather than silently choosing it.

At phase-three close:

1. Fresh WSL workspace build.
2. Workspace tests.
3. Foreground `sh e2e/run.sh` (currently 97 cases; count may grow).
4. `cargo fmt --check`.
5. Cold `cargo clippy --workspace --all-targets -- -D warnings`.
6. `cargo deny check licenses bans sources`.
7. `typos spike`.
8. Inspect every generated catalog/case diff; generation cannot prove semantic prose
   correctness.
9. Synchronize `plans/282`, this plan, `LIVING_STATUS`, errorloom README, and
   `spike/CLAUDE.md`; run prompt-review if steering changes substantively.
10. Run the required opaque-review gate and obtain explicit ACK before final
    integration.

No BLESS/generation mode is a routine builder action. The conductor runs the final
promotion/generation on a fresh verified binary and inspects the complete diff.


## 6. Worktree and dispatch state

- Create a new phase-three worktree and `ai/*` branch from
  `ai/r28-errorloom-phase2` at or after `4b95a9b4`.
- Do not reuse the compacted phase-two builder. Use one fresh builder per Unit 1, Unit
  2, and Unit 3.
- Builders do the work themselves and do not spawn subagents.
- Every builder reads root `README.md` / `DESIGN.md` / `IMPLEMENTATION.md`, current
  `LIVING_STATUS`, all of `spike/CLAUDE.md`, this file, all of `plans/282`, relevant
  crate steering, and the specifically named code/tests before editing.
- Builders apply the required builder-only steering silently.
- Every read/edit/citation after worktree selection stays inside that builder's own
  worktree.
- The primary `ai/main` checkout is human-dirty in
  `spike/crates/errorloom/README.md` and the flagship case. Builders never access or
  modify it. The human resolves those concurrent edits during fold/rebase.
- Phase two is not pushed. Final fold/rebase remains human-managed.


## 7. Settled baseline outside phase three

Do not reopen these while implementing the durable workflow:

- Dorc language v0.2 and its `@` selector/word-verb/`#:` grammar are complete;
  `plans/281` is the authoritative grammar.
- Production currently accepts one mark per physical line. The full multi-mark block
  model is reference-tested and additive later; adopting it requires no respell.
- Value-less singleton binds and verdict-position value tails are absent in v0.2.
- Bare `sh -c` remains descend-for-hints/no-license; `dorc:sh` is the licensed
  invitation.
- Book order is sacred; none of this work changes execution or licensing behavior.
- Render form remains unstable-and-improving; transcript/case byte stability is a
  test contract, not a promise that diagnostic wording or layout is a product API.
- The report/evidence plane remains decision-inert. Errorloom changes prose/catalog
  authorship only, never command licensing.


## 8. Live deferrals and human decisions

Not phase-three work:

- `282:phase-adjacent-fragment-followup`: attached/glued newly-positioned markers,
  including backtick-adjacent cases;
- paragraph add/remove support beyond words-and-paragraphs v1;
- type-gated foreign/passthrough migration owned by the separate opaque lane;
- arrangement prose promotion and following-dependent `:` / `.` punctuation;
- full multi-mark production model;
- invocation-global gutter width across multiple frames;
- analysis population of `CommandName::Resolved`;
- broad prose-quality pass over `[unwritten:]` and `sm ` text;
- `touches` -> `disturbs` shell-function fixture/doc residue;
- the one-test `covered() subset-of case-owned` drift guard;
- lax-order `.ran` bless rewrite noise;
- the standing `TODO-ADDTL.md` rider queue.

Human-owned publication choices:

- errorloom LICENSE and `publish = false` flip;
- Cargo publication metadata;
- whether public growable error enums remain `#[non_exhaustive]` at publication;
- final prose voice and catalog canonicalization diff acceptance.

Catalog canonicalization waits for the new phase-three generated-lock workflow. Do not
use the still-present legacy `DORC_CATALOG_PROMOTE` writer for it.


## 9. Source map

Generic errorloom:

- `spike/crates/errorloom/src/editable.rs` - nested transport and bounds;
- `spike/crates/errorloom/src/bless.rs` - structure regeneration and fixpoint;
- `spike/crates/errorloom/src/container.rs` - case/frontmatter/txtar model;
- `spike/crates/errorloom/src/runner.rs` - current controlled replay execution; Unit 1
  introduces the consumer-neutral driver/result + reusable executor boundary here or
  in a focused sibling module;
- `spike/crates/errorloom/README.md` - public product/API boundary.

Dorc integration:

- `spike/crates/core/src/tagged.rs` - core-owned `RenderParts`;
- `spike/crates/core/src/catalog.rs` - current catalog types/parser/serializer/table;
- `spike/crates/core/src/diag.rs` - typed payload and production render seat;
- `spike/crates/dorc-loom/src/lib.rs` - editable adapter and identities;
- `spike/crates/dorc-loom/src/consumer.rs` - worlds, inventories, apply/regeneration;
- `spike/crates/dorc-loom/src/bin/dorc-loom.rs` - Unit-1 command surface; current
  content-skeleton replay selection is rejected and must be replaced by Dorc-owned
  exact-shape dispatch over errorloom driver results;
- `spike/crates/dorc-loom/src/compile.rs` - strict marker compiler;
- `spike/crates/dorc-loom/src/edit.rs` - edit transport to compiled section;
- `spike/crates/dorc-loom/src/preview.rs` / `inspect.rs` - concrete preview;
- `spike/crates/dorc-loom/tests/` - compiler, adapter, coverage, consumer, fixpoint.

Design and steering:

- `Research/plans/282-transcript-case-prose-pipeline.md` - complete product/design
  contract;
- `Research/plans/281-annotation-mark-grammar.md` - Dorc language v0.2 grammar;
- `spike/CLAUDE.md` - current implementation law;
- `Research/LIVING_STATUS.md` - live branch/round state.


## 10. Phase-three completion definition

Phase three is complete only when all are true:

- the transcript is the only prose/template edit surface;
- every replay is tested as exact bytes, while edit authority comes only from typed
  provenance attached to that exact replay result;
- generic errorloom knows no Dorc command names or output formats; Dorc owns exact
  invocation dispatch and explicit generic fallback policy;
- compile exposes the exact interpretation before any source write;
- promote requires the exact fresh receipt the editor inspected;
- catalog and cases are wholly generated artifacts, fully preflighted before
  per-target publication;
- used/all inventories are available through the command surface and committed used
  inventory is regenerated with the case;
- old promote/splice/env-gated paths are absent;
- both fixpoint gates and the full green suite pass;
- live docs describe only the new workflow;
- opaque review returns explicit ACK;
- the branch is clean, unpushed, and ready for the human's fold/rebase.
