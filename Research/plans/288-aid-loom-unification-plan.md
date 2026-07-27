# 288 — the aid/loom unification plan (architecture · strings · minting · lint-merge · test-tree)

PLANS-TIER (promoted from `notes/288`, 2026-07-24, human-directed; the notes-tier copy is
retired). Ahistorical and kept-current: if this document is wrong, rewrite it. AI-authored
(Fable, design sittings WITH the human, 2026-07-24); §0 separates human-typed rulings from
plan proposals. Authority: root docs, `spike/CLAUDE.md`, human-typed rulings outrank.
Companions: `notes/287` (errorloom as-built) · `plans/282` (loom-pipeline design authority)
· root `AID-NEEDS.md` (registry + law). Evidence bases banked herein: the string-trawl
builder report (§6) and the kernel aid-threading scout report (§2e) — both 2026-07-24,
both conductor-verified at their load-bearing claims; builders wrote no notes.

## §0 — Ack-ledger

HUMAN-TYPED (2026-07-24; binding):

- **rul-errors-human-authored-review-surface** — human-finding trumps LLM-finding for
  errors; errors are intended to be human-authored. Errorloom is *the review-surface of
  the entire project*: the navigation direction that matters is FROM the loom (the review
  entry-point) TO the code that references/generates it.
- **rul-claudemd-fires-per-directory** — loom crate-siting matters because `CLAUDE.md`
  steering fires per directory read/write: the hosting crate decides which invariant
  registry auto-loads for every loom edit.
- **rul-core-stays-light-custody** — `core` stays light; the analyzer machinery is
  sacrosanct and its `CLAUDE.md` must not clutter with error-machinery notes.
- **rul-slug-decides-loom-placement** — mechanical, not judgment: a canonical loom for a
  registered aid-slug lives in the ONE primary "all tracked aids" collection; tertiary
  looms (unregistered behaviours) stay in their causative crate.
- **rul-lints-join-one-registry** — native lint findings join the unified namespace;
  separate catalogs defang the global policy machinery (floors, severity, correlation,
  push/pull); centralizing forces finding-logic and finding-discussion to be global.
- **rul-corpus-term-reserved-shell** — "corpus" is reserved for SHELL-corpora; this
  machinery says looms / cases / suite.
- **rul-loom-mint-guarantee** — `dorc-loom` handles minting: a new slug referenced
  anywhere GUARANTEES a case comes to exist, failing loudly until the builder authors it.
- **rul-narrative-layer-naming** — the decision-inert record plane is named
  **Narrative** (`aid::Narrative`, `CollapseNarrative`), deliberately soft and distant
  from correctness machinery; "evidence" is avoided (correctness may someday need it);
  "account" rejected (ops-domain collision: user accounts). Register-watching stays
  informal by explicit direction — no written reserved-word law.
- **rul-prose-authorship-follows-looms-sacrosanct** (née rul-prose-pass-is-fable-this-arc;
  superseded 2026-07-26) — the remaining `sm `/`[unwritten:]` prose is authored at the
  loom surface under the AGENTS.md looms-sacrosanct law: AI-authored user-facing prose
  only under explicit, single-case, narrowly-scoped human ack. End-state unchanged: the
  human edits ONLY loom files, ever — no half-transitioned codebase, no
  "where did that string come from".
- **rul-dorc-sh-not-carved-out** — `dorc-sh` inherits the same machinery: still
  user-surface, still auditable, similar parse errors; the off-ramp does not exempt it.
- **rul-help-text-is-loomable** — help text is not out-of-scope chrome; it is
  loom-able and expected to integrate with the explain machinery (§6, §7b).
- **rul-flat-test-tree** — all cases are peers in a flat `tests/` dir, selected by
  runners via extension/shape: `crates/C/tests/X.loom` · `crates/C/tests/X/X.loom`
  (multi-file) · `crates/C/tests/X/…` (un-loom-able e2e residue). No `looms/` nesting.
- **nit-needles-rot** — hand-authored must-contain assertions cargo-cult and rot; a
  design constraint on §7.
- **rul-error-slugs-are-semantic** — error-slugs are part of the USER-FACING surface:
  someday users google them, and they are the one name-class that becomes a real
  backwards-compat surface at publication. Mint them semantic-first, never as a
  file-naming/siting decision; loom file-count falls out of whatever codes are
  semantically true (no lean toward or away from shared-slug families — "if many CLI
  flags have the same error-slug, great; if not, also great"). Composes with the
  strawman-formats order: freely renameable today, which is exactly why they are
  minted semantic-first.
- ACKED as proposed: **prop-aid-crate-extraction** — the §2 re-cut shape (whole
  describe crate · Narrative rename · prov-stays-core), conditional DISCHARGED on the
  human's read of §2e (2026-07-24) · **prop-scaffold-explicit-command** ·
  **prop-migrate-all-cli-argument-errors** ("noisy but we care about errors,
  hardcore") · **prop-normalizer-closed-vocabulary** + **prop-structural-needles-only**
  (§7) · the §7b phase-7 slot · the §8 ordering, with the compression lean below.
- LEANS (not rulings): lint-surface render tunes by verbosity (`--terse` / default /
  `--verbose`) rather than a fixed body-only choice — adopted into §5. Phase
  execution leans one-lane-with-checkpoints and/or parallelization for wallclock
  savings — adopted into §8.
- NACKED: a codified two-register naming law (watched informally instead; writing
  reserved words down pre-primes builders to use them).

PLAN PROPOSALS still open: prop-mint-completeness-hardening (§2c) only.

## §1 — The destination, one screen

Every tracked aid is a registry slug; every slug has exactly one canonical loom in ONE
central, flat, review-optimized collection; the loom is the authoring AND review surface
(prose human-authored, transcripts executable); every user-facing string on every product
surface — diagnostics, lint findings, CLI argument errors, `dorc-sh`, help text, chrome —
is loom-editable by arc close; native lint findings are registry codes like everything
else; and the golden/behavior test tier converges onto looms run by one central runner.
The DST seed-sweeps, differential harness, unit tests, and component-contract tests stay
exactly where they are. Strawman names throughout; rename freely, no compat mapping,
ever (standing human order, 2026-07-24; see also `spike/CLAUDE.md` at its next sync).

## §2 — Architecture: crates, types, seams

### §2a — The two crates

```
crates/core   the DECIDE plane: shared primitives + license vocabulary + provenance ids
crates/aid    the DESCRIBE plane: narrative records, diagnostics, catalog, render, Carrier
```

Dependency direction: `aid → core`; kernel crates (`syntax`, `analysis`, `oracle`,
`plan`) dep BOTH; edges (`cli`, `lint`, `dorc-loom`, `hostsim`, `coverage`) dep both. No
cycles anywhere (scout-verified: nothing in the would-stay-core modules references any
would-move type — §2e). `aid` is DST-clean: pure data + render, no clock/fs/net; the
kernel-dep-cleanliness law is untouched.

### §2b — Type inventory (scout-audited, LOC at 2026-07-24 tip `ce460f6b`)

STAYS `core` (~3,200 LOC): `lib.rs` shared primitives (AstId/LeafId/OracleFileId/Span/
BytePos · Symbol/Interner/OpaqueToken/KindId · Phase/Verdict/Rc · OutBytes/ValueGrade/
Predicted/Channel/Observable/Grade · SelectorId/EntityRef/FactKey/FactBacking; ~90% of
the file) · `claim.rs` (342; sealed tiers) · `coord.rs` (861; comparison chokepoint) ·
`room.rs` (243; seal mechanism) · `unord.rs` (150) · `escalation.rs` (63) · `prov.rs`
(661; ProvId/ProvArena/TopCause — dep-free, decision-inert by construction, CONSTRUCTED
by kernel passes and READ by the describe plane; the frictionless disposition is stay).

MOVES to `aid` (~5,000 LOC): `diag.rs` (3,026; DiagCode/payloads/render seats/why-lens
reader) · `catalog.rs` (597) + `catalog_lock.rs` (530, generated) · `tagged.rs` (114;
RenderParts) · `evidence.rs` → **`narrative.rs`** (634; renamed per
rul-narrative-layer-naming: `CollapseEvidence` → `CollapseNarrative`, `CollapseKind` and
`TrustTier` ride with module-local naming latitude) · `Carrier` + `Severity` peeled out
of `core/lib.rs` (~70 lines; Carrier's whole reason is diagnostic accumulation —
inv-no-throw — and zero core functions return it).

The rename rides the extraction commit (one mechanical lane, all sites, no aliases).

### §2c — The Narrative plane (né evidence)

- Semantics unchanged, name changed: decision-inert records minted at every
  safety-narrowing, carrying the collapse's operands, k-capped, Eq-excluded from lattice
  equality. The two-plane law's wording updates at steering-sync ("aid-narrative is
  decision-inert at the type level"); law SLUGS in historical docs stay as written (doc
  citations are not code).
- The seal survives the move (scout §7): enforcement is type-level — private fields, no
  method yields a license-plane input, `ProvId` `!Ord` — not co-location. The
  `compile_fail` doctest that feeds a narrative to `room::mint_from_room` becomes a
  cross-crate reference (`aid` deps `core`; works unchanged). License values still flow
  IN freely (`TrustTier::from_vouch` reads `claim::ByVouch` across the crate seam,
  downward).
- **prop-mint-completeness-hardening** (NEW, from the scout's unknown-unknowns; needs
  ack): the silence-hole closure is weaker than `27V` §1's documentation implies — the
  static-merge mint is a separable post-pass (`mint_merge_evidence` mirroring
  `mint_top_causes`) guarded by a `debug_assert`, not a constructor weld; a future
  collapse site that forgets its mint pass under-narrates silently. Pre-existing, not
  created by the split. Proposed hardening (cheap, rides any nearby lane): promote the
  pairing assert to a release-mode test gate, plus a tidy-style completeness test
  enumerating collapse sites against minted classes.

### §2d — Seams and mechanical notes (scout-grounded)

- `SiteId` currently sits in `diag.rs` (its own doc says it belongs in core). Under this
  cut both its definer and its `narrative.rs` consumer move together, so relocation is
  NOT forced; relocating it to `core/lib.rs` beside `LeafId` remains a nicety for the
  extraction lane (flag, don't block).
- The arrangement walker is NOT captured by any core-side split: `tier_word` +
  `survival_chain`/`render_chain` live in `cli`, `wall_walk_survival` in `plan`
  (plan's render sub-lane, `plan/invocation.rs` carrying RenderParts as inert payload).
  Consolidating the walker into `aid` is a separate, later question — out of this arc.
- Most crates already straddle both planes; the split relabels fan-in rather than
  reducing it. The wins are custody, steering-locality, and extraction-rehearsal — not
  dependency simplification. Priced knowingly.
- Churn shape: module moves + workspace-wide import rewrites (`dorc_core::{diag,catalog,
  tagged,Carrier}` → `dorc_aid::…`; `oracle` heaviest at ~56 Carrier sites) + the
  narrative rename. Zero behavior change; goldens byte-identical; one atomic cutover
  commit.

### §2e — Scout census, condensed (the conditional-ack evidence; full report banked in
the conduct chat, this is its durable summary)

- **Core's own aid-emission census: EMPTY.** Zero production sites in
  `claim/coord/room/unord/escalation/prov` construct Diag, return Carrier, mint
  narratives, or touch tagged/render. `lib.rs` holds only the Carrier/Severity
  definitions themselves (+2 test uses). The earlier-feared landmine — narrative
  constructor-demand at core-resident chokepoints forcing `core→aid` — DOES NOT EXIST
  as-built (+SURE, verified).
- **All nine collapse-class mints live in `analysis` (1: fact-merge post-pass),
  `plan` (3: decline/demotion/wall), `cli` (5: substitution/entry×2/merge/decline).**
  The value-shape demand (non-defaultable operand fields on `CollapseKind`) is the
  enforced half; the control-flow half is the post-pass discipline of §2c.
- **Diag payloads reference no license-plane types** — plain data + `ProvId`/`TopCause`
  in exactly one payload (`CmdsubOperandTop`). `catalog.rs` has zero license-plane
  references. The describe/decide boundary already IS the file boundary: decide-side
  files contain no strings and no aid types; describe-side files contain no license
  types (the strongest grain signal).
- **No mid-kernel decision reads diagnostics or narratives.** Every `.diags`/severity
  read is render-edge, exit-code-edge (`book_unmodeled`), or test. The narrative plane
  has zero decision-consumers.
- **Render seats** (`render_cli_parts`, `render_staged_cli_parts`, `render_body*`,
  `why()`) are called only from edges (cli, lint) and the loom toolchain; oracle's
  render calls are `#[cfg(test)]`-only.
- Weight: this cut leaves `core` at ~31–39% of its current LOC (prov-in-core end of the
  scout's range).

## §3 — Loom placement and the flat test tree

- **Primary collection**: `crates/aid/tests/<slug>.loom` — flat, slug-named, one
  canonical loom per registered aid (rul-slug-decides-loom-placement). The CLAUDE.md
  that fires on every edit is `aid`'s, i.e. the aid-law registry. No view-subfolders:
  lint/why/plan are views over one model, and one canonical loom may carry replays
  through several views (`282` multi-replay). The `plans/286` explain surface's
  CONCEPT cases (`dorc explain wall`-shaped; no DiagCode) are registry-tier and share
  this collection at their unpark — placement reads "registered aid" broadly, never
  DiagCode-constructor-narrowly.
- **Tertiary looms**: `crates/<c>/tests/<case>.loom` in the causative crate —
  transcripts pinning unregistered behaviour. Data only.
- **Whole-product cases**: `crates/cli/tests/<case>.loom`, or `<case>/…` dir-form for
  multi-file fixtures and the un-loom-able e2e residue (rul-flat-test-tree). In-dir
  manifest convention (what marks a dir a case vs an .rs test's fixture space) is
  builder latitude, one-line rationale.
- **Runners**: `cli/tests/looms.rs` + `cli/tests/e2e.rs` (`harness = false`,
  libtest-mimic shape) walk every `crates/*/tests/` for their shapes by extension; one
  named filterable test per case; Cargo compiles only top-level `tests/*.rs`, so data
  files and case dirs coexist freely with `.rs` integration tests in the same flat dir.
- Discovery is grep-first regardless of tree: the slug in the filename and the
  `code:` frontmatter make loom↔code navigation one search in either direction.

## §4 — The mint-seam + scaffold

Trawl-proven gap: the post-flip machinery cannot mint a new code at all — the lock
generator derives rows from `consumer.mirror()`, seeded exclusively from
`owned_catalog()` over the existing table; all mutators refuse unknown slugs; hand-rows
fail the byte-identity fixpoint; the ratchet is shrink-only. `282`'s empty-loop mint
half is unbuilt (the phase-three dogfood only edited an existing code).

Build (one small lane):

- **mirror-union** — the generator unions mirror rows with enum-derived rows so a new
  slug yields a `message: None` row (rendering `[unwritten: <slug>]`).
- **the guarantee** (rul-loom-mint-guarantee) — the completeness partition already goes
  red for a caseless new slug; keep that red and make the failure message name the
  repair command verbatim.
- **scaffold** (prop-scaffold-explicit-command, ACKED) — `dorc-loom scaffold <slug>`
  writes the empty case skeleton into the primary collection. Explicit command, never a
  build/test side-effect (tests never write source; concurrent-builder races; conscious
  ownership). Scaffold-and-forget stays red for free: the same-slug coherence gate
  fails an empty replay section until a genuinely-firing world is authored.

## §5 — Lint unification

- Native lint findings become registry codes; the lane-local namespace retires; slugs
  keep-or-rename freely (no mapping). Foreign relays (`shellcheck:SC2086`) stay
  source-tagged relays forever.
- Render: the lint surface stays a selection policy. Default keeps the compact
  line-per-finding shape; `--verbose` may add source frames; `--terse` compresses
  further — the human's verbosity lean, riding `KNOBS:kFLOW` /
  render-form-unwelded rather than a fixed body-only weld. (Body-only vs framed is a
  per-call seat choice — `render_body*` vs `render_cli_parts` — orthogonal to
  catalog-sourcing.)
- The machine envelope reshapes freely with the model fold if the unified shape wants
  different keys. CI gates on codes/severity, never finding-set identity.
- Payoff: floors, severity policy, dedup keying, push/pull selection,
  `dorc why`-addressability, and §4's canonical-loom guarantee uniformly cover lint.

## §6 — The string-centralization backlog (trawl inventory, 2026-07-24, banked)

Reference path: `DiagCode` → generated `CATALOG` row → params fill →
`render_staged_cli_parts` (framed) or `render_body` (body-only). Everything below
bypasses it at the trawl tip (`aaddb106`; line numbers drift — re-grep by string).

- **CLI argument/usage/file errors** (ACKED to migrate): the `dorc: {msg}` family —
  dispatch seats `cli/main.rs:203,214,219` (+ lint variants `:843-:917`), producers in
  `parse_args_from` (`:338–:498` — strip-needs-path, flag-needs-value ×6, unknown-mode/
  flag ±suggestion, no-book-given, whylog exclusivity ×2), `humane_read_error` (`:581`),
  lint operational trio (`:889,:903,:918`). Codes are minted per
  rul-error-slugs-are-semantic — whatever slugs are semantically true, no
  file-count target in either direction. Precondition: the
  **invocation-error route** — these fire trivially in a replay (`$ dorc strip` IS the
  world), but `case_example` → `case_diag` → `world_of` assumes a book/oracle world;
  the generator needs the worldless path.
- **`dorc-sh` errors** (rul-dorc-sh-not-carved-out): `cli/src/bin/dorc-sh.rs:30,36,55`
  join the registry like every other surface (slugs, canonical looms, auditable).
  Render stays terse (surface selection). One SEAM note, not a carve: if `dorc-sh`
  ever ships host-side, host-side emissions likely stay raw-bytes-upstream with
  controller-side narration (the aid plane is controller-plane) — decide then, change
  nothing now.
- **Help/version text** (rul-help-text-is-loomable): re-bucketed from out-of-scope to
  loom-able. Help is the natural PILOT for the arrangement/transcript-sections home
  (§7b): static, param-free, deterministic — the simplest possible editable transcript
  — and its content is expected to share register machinery with `plans/286`'s explain
  surface (density registers; validated command-block embeds). Lands with §7b, not
  before.
- **Arrangement/chrome** (inventory; home decided at §7b): lint render arrangement
  (`lint/src/render.rs` slugs lint-structure/-group/-summary/-indent/…; the clean
  sentence; the advisory preamble; the JSONL envelope) · CLI chrome (plan-summary
  `:3335`, decision-digest `:1706`, why-pointer `:1621`, `--list-sources` `:836`) ·
  plan-render annotations (`plan/src/render.rs` elision comments/replace/banners/guard
  suffixes). IMMEDIATE exception: **fix-lint-tally-pluralization** (`render.rs:58`) is
  phase 0.
- **Class-keyed prose**: `remediation_hint` (`diag.rs:2247-2262`) + `why()`'s reason
  format — needs a class-prose register; parked to the prose-register sitting, relocates
  with `aid` untouched.
- **Verified-tagged relays, no action**: external-tool relays; the PASSTHROUGH
  `detail`-param codes (de-passthrough is the opaque sibling lane's, `284`).
- **Out of scope**: `--debug-argv` machine differential, byte-floored artifact stdout,
  the `dorc-loom` tool's own surface (internal-tool sharp-edges), coverage/sweep bins.

## §7 — e2e convergence onto looms

- **Converges**: the golden/behavior tier — plan renders, worlds, lint cases,
  exec-under-mocks (`.ran` logs are bytes), `dash -n`, the strip-floor differential
  (replay commands). **Never converges**: DST/hostsim seed-sweeps, the differential
  harness (its finding-DRAFT flow re-points at loom skeletons), unit tests,
  component-contract tests.
- **prop-normalizer-closed-vocabulary** — tolerated nondeterminism is declared
  per-replay in frontmatter from a CLOSED engine-owned vocabulary (e.g.
  `tolerate: pipe-stage-order`), applied identically at bless-capture and at check; the
  committed transcript is the canonical form and the declaration is the honesty
  disclosure. One named normalizer per named nondeterminism class; never free regex.
- **prop-structural-needles-only** (shaped by nit-needles-rot) — re-bless-surviving
  intent assertions are structural wherever possible: the slug needle is derived; a
  `must-contain` naming a code slug validates against the catalog (dead slug ⇒ refused,
  self-cleaning). Free-text needles legal, rare, reason-carrying.
- **Sanctioned-executor transfer** — retiring `sh e2e/run.sh` into the central runner
  moves the ONE-sanctioned-fixture-executor role: a deliberate safety-law edit in
  `spike/CLAUDE.md` + every brief's safety block, landing with the porting phase.

### §7b — The arrangement/chrome home (SETTLED; built at phase 7)

Options 2+3 COMPOSE, per `289` §2o (`289:rul-arrangement-home-is-registry-plus-transcripts`):
a generated ARRANGEMENT REGISTRY is the storage — `aid/src/arrangement.rs` plus the generated
`arrangement_lock.rs`, keyed by arrangement-slug + an optional occurrence, entries holding
ORDERED WORDS that a seat interleaves with its computed values — and renderer-stamped
`ArrangementWords` spans are the edit surface, so a chrome-word edit in a transcript flows to its
registry entry exactly as catalog prose does. It generalizes the catalog pipeline rather than
minting a second one: mirror-union generation, both fixpoint gates, one `dorc-loom promote`
publishing both locks. `282:rul-arrangement-words-exempt-v1` is thereby LIFTED for migrated
chrome. Help text was the pilot (`$ dorc --help` as a whole-page loom); usage joined as a
seat-appended entry. Three fences held in the build: the migrated marker is TYPED
(`Words::Migrated`), never the catalog's in-band `sm ` prefix, because chrome renders verbatim
into product bytes; artifact-plane strings (every `plan/src/render.rs` emitter) stay hardcoded
under the byte floor; and layout is not a word (indents, line breaks, punctuation frames stay
computed). A value-bearing chrome line renders as ONE editable SECTION holding interleaved word/value
fragments — never split ACROSS sections, whose computed fences would break the transport's
anchoring for every other prose section in the same render (the 2026-07-24 lesson, preserved).
The line-field path re-splits an edit on the STAMPED fragment series (landed at the W4 span
lane; `28H`, `aid/CLAUDE.md` a-chrome-line-is-one-section), so multi-word entries are
transcript-editable wherever a driven replay stamps them; the page path stays verbatim.

## §8 — Phases (each one lane; granular; gates green; atomic where marked)

0. **phase-lint-tally-pluralization** — tiny, first; stops transcript-churn
   amplification.
1. **phase-aid-crate-extraction** (§2) — map-then-execute; atomic cutover; the
   Narrative rename rides; SiteId relocation as a flagged nicety.
2. **phase-mint-seam-and-scaffold** (§4) — plus a mint-to-green walkthrough as its own
   canonical loom (the empty loop finally exercised end-to-end).
3. **phase-lint-unification** (§5).
4. **phase-cli-error-migration** (§6) — invocation-error route + the parameterized
   family + `dorc-sh`.
5. **phase-flat-tree-move** (§3) — collections into `crates/*/tests/`, central runners,
   run.sh retirement + safety-law/steering edits, `conduct-bless` re-point. Atomic,
   paths only.
6. **phase-e2e-loom-conversion** (§7) — normalizers + needles as demanded;
   opportunistic tail thereafter.
7. **phase-arrangement-home** (§7b) — the design sitting + build; help text as pilot.
8. **phase-prose-burn-down** (rul-prose-authorship-follows-looms-sacrosanct) — the
   remaining `sm ` + `[unwritten:]` prose is authored at the transcript surface
   (underway 2026-07-26; the why-surface rows wait on `28G` Phase W4's span/transport
   work for faces); the `28A` doc-comment/message coupling
   (finding-old-prose-coupled-to-message-strings) discharges here. ARC-CLOSE
   INVARIANT: zero user-facing strings without an editable loom; zero `sm ` markers;
   the human's only future edit surface is loom files.

Execution shape (human lean, acked): compress phases 2–4 into one checkpointed lane
and/or parallelize file-disjoint phases for wallclock — e.g. 0∥1 (disjoint), then the
2–4 lane, 5 serial after. Dispatch shapes are conductor latitude within that lean.

Riders that attach wherever cheap: prop-mint-completeness-hardening (§2c) · the
`covered()⊆case-owned` drift guard (`28A` §2u) · `touches`→`disturbs` fixture residue
(verify-in-other-cells first).

## §9 — Steering-sync (lands with the phases, never before)

`spike/CLAUDE.md`: the strawman-formats bullet (stability-ledger region) · User-aid
block relocation pointer + narrative rename (phase 1) · safety-block executor line
(phase 5) · loom-placement law (phase 5). `AID-NEEDS.md`: lint-namespace caveats out
(phase 3); CLI-error rows in (phase 4). New `crates/aid/CLAUDE.md` at phase 1.
`AGENTS.md` opaque-review annotation: the human's own hand.

## §10 — Open asks

1. prop-mint-completeness-hardening (§2c) — build the release-gate + completeness
   test, or accept the debug_assert posture? (The only open item; everything else in
   §0 is acked.)

## §11 — Confidence

+SURE: both banked evidence bases (trawl §6, scout §2e — load-bearing claims
conductor-verified in-tree); the §0 ledger. ~SUSPECT: the invocation-error/worldless
route is small (`case_diag`/`world_of` already stretch for whylog cases — `287` §11
precedent); phase-1 churn stays mechanical (the scout found no semantic coupling, but
oracle's ~56 Carrier sites make it the widest import rewrite). -GUESS: phase sizing;
the arrangement-home sitting is the least-specified work in the arc.
