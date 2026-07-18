# 27L — block-context lane-payload-v1 (né `24T` R0) landing + residue

AI-authored (Opus builder, r27 lane-payload-v1 session, 2026-07-17). Records what landed for
`270:block-context`'s payload lane (`27J` §2.3). **This note is the durable deliverable.**
Authority: root docs + `spike/CLAUDE.md` rulings + `271`/`274`/`24T`/`27C`/`27D`/`27J`/`27K` outrank
this. Companions: `27J` (build spine / lane order), `274` (the eval'er/reentry spec), `24T` (the
payload keystone), `27K` (lane-wrapper-peel, the peel machinery this composes with).

## Branch / fold state (READ FIRST — the conductor must reconcile)

- Branch `ai/r27-payload-v1`, based on `ai/spike3-r27` @ **`fa7885f`** (the brief's stated base;
  verified at step-zero).
- **The lineage did NOT move during this run** (unlike lane-wrapper-peel): all seven commits are
  based directly on `fa7885f`, no rebase expected. VERIFY the tip before folding.
- Dispatched AHEAD of lane-context-entry (independent; lane-context-entry was gated on pending human
  rulings, this lane was not — `27D` block-close).

## Commits (on `ai/r27-payload-v1`, oldest→newest)

1. `cc816c3` (AI test) — **negative is_diverged pin** (the rider; `27Xf` Tier-2 residue).
2. `9bd3cd8` (AI new ana) — **the invited-rooms type split** (`core::room`) + the `279f` §5
   compile-failure pin (a real `compile_fail` doctest).
3. `4a2c227` (AI new ana) — **the `dorc:sh` three-spelling recognition** (`syntax::sem` §9) + the
   row-3 strip documented-dangle pin.
4. `81065eb` (AI new oracle) — **eval'er reentry detection** (`oracle::evaler`), MODELS-only.
5. `731a937` (AI new ana) — **payload decomposition** (`oracle::payload`): accept/refuse basic-forms,
   nested parse, whole-line fold. MODELS-only.
6. `9911a84` (AI new ana) — **the per-run PATH shim model** (`plan::shim`), pure/DST-clean.
7. `a296811` (AI new e2e) — **two rung-0 e2e cases** (bare `sh -c` wall + composed acceptance shape).

## Acceptance summary (all green)

- **Four gates clean** on the whole workspace: `cargo fmt --check` · `clippy --workspace
  --all-targets -- -D warnings` (0 warnings) · `cargo deny check licenses bans sources` · `typos
  spike`.
- **Full workspace test green** including the `core::room` `compile_fail` doctest + its positive
  control. No failures (one pre-existing ignored).
- **e2e: 82/82** (was 80). The **80 pre-existing cases are BYTE-STABLE** (rung-0; no bless); 2 new
  payload-tier cases.
- **MODELS-only, zero new trust**: nothing this lane built is consumed by `analysis`/`plan` — a
  payload/eval'er book site with no eval'er oracle loaded walls opaquely
  (`empty-world-byte-identical`). The plan-wiring (inner-node classification under ρ, the
  probe-shipping of `dorc-sh` reentries) is the task-14-gated follow-on.

## 1. The invited-rooms type split + the compile-failure pin (report-ask #4)

`core::room` mints the descend-don't-license TYPE differentiation (`274` §1; `271:rider-invited-
rooms-typing`): a `RoomFact<R: Room, P>` phantom-tagged by a SEALED `Room` (`Invited` | `HintOnly`,
mirroring the `claim.rs` sealing). The license-input exit `into_license_input()` exists ONLY on
`RoomFact<Invited, _>` — a `RoomFact<HintOnly, _>` has NO such method, so a hint-lane fact is refused
by any license-consuming signature at COMPILE time. The `279f` §5 pin is a real `compile_fail`
doctest (module-level, `room.rs`): passing a `hint_only` fact to `mint_from_room` (which demands
`RoomFact<Invited, _>`) does not compile; a positive-control doctest shows the invited counterpart
DOES. It composes as the OUTER gate over the claim-tier algebra — the canonical mint-input is
`RoomFact<Invited, ByVouch<_>>`: clearing the room gate hands the inner `ByVouch` to the existing
tier gate. `RoomTag` (Invited/HintOnly) is the runtime witness for diagnostics only — never a
license branch (the branch is the TYPE).

`syntax::sem::classify_evaler_head` bridges recognition → room: bare `sh` ⇒ `HintOnly`, `dorc:sh` ⇒
`Invited`, `dorc-sh` ⇒ no room (RuntimeObject).

## 2. The accept/refuse table — basic forms (report-ask #2; `24T:P-A3`)

`oracle::payload::classify_payload_form` draws the accept frontier at `sem::const_literal_text` (the
"no variables at all" case):

| book payload | form | disposition | why |
|---|---|---|---|
| `sh -c 'systemctl restart nginx'` (single-quoted literal) | **ACCEPT** `Literal` | decomposes | resolved-literal rung (`24T` §5a); the lint-taught cell (c) |
| `sh -c "systemctl restart nginx"` (const double-quoted) | **ACCEPT** `Literal` | decomposes | trivially constant (`24T:P-A3`) |
| `sh -c "systemctl restart $SVC"` (interpolated) | **REFUSE** ⊤ `InterpolatedSplice` | RUN | the §2 cell (a) hole; `imp-P1` stays a wall |
| `sh -c "$(build_cmd)"` (cmdsub splice) | **REFUSE** ⊤ `InterpolatedSplice` | RUN | `24T` §5a cmdsub cliff |
| `sh -c ''` (empty) | **REFUSE** ⊤ `Empty` | RUN | degenerate; no vacuous elision claim |

Nested parse (`parse_payload`) of an ACCEPTED literal degrades SITE-LOCALLY (never book-level) in two
cases: a ⊤-reject construct inside (`Unparsable`, `pin3`) or dorc annotation syntax inside
(`NestedAnnotation`, `271:rul-no-nested-annotation` / `imp-P6` payloads-are-plain-sh). Accepted leaves
carry payload-relative derived-text locators (`rebase(offset)` → book coords; the single-quote-literal
case where provenance is nearly free).

Whole-line fold (`fold_line`, `24T` §4a): elide iff EVERY leaf elides; else guard-conjunction of the
diverged leaves' checks; any unresolvable leaf ⇒ RUN; empty leaf set ⇒ RUN (the conservative floor).
The per-leaf disposition is SUPPLIED — the real inner-node classification under ρ is the plan-wiring
follow-on; this settles the fold ALGEBRA and the accept/refuse frontier.

**Punt-empowered exploration cells recorded, NOT accepted (`24T` §5b, conservative spike floor):**
the value-plane-resolved template rung (`CMD="…"; sh -c "$CMD"`, `24T` pin5) and the basic-set-form
(bounded literal-SET carriage). Both are follow-ons; the accept frontier at `const_literal_text`
draws the line exactly at the `24T` §9 fences (general holes / automata / loop-assembly all OUT).

## 3. Where the synthesized-payload-render door is, and what would open it (report-ask #3)

**The door is at `oracle::payload::fold_line` and `ParsedPayload`** — specifically, the fact that the
fold decides the OUTER leaf's disposition {elide, guard-conjunction, run} and NEVER re-serializes
payload bytes. `27D:rul-synthesized-payload-render-stays-unwelded` (né R2) is honored structurally:
nothing in this lane re-embeds, re-quotes, or engine-authors payload string content. `ParsedPayload`
carries only leaf COUNTS + SPANS (locators into the author's verbatim bytes), never reconstructed
text; `LineFold` names a disposition, never a rewrite. The refusal is a code-path fact, not an
architectural assumption: no type FORECLOSES a future un-refusal.

**What would open it (a future un-refusal, on discovered need):** a new variant/function that maps a
resolved payload value back into an emitted apply artifact — e.g. a `LineFold::RenderSynthesized {
text }` arm feeding a new render path in `plan`, consuming a value-propagated constant. Every existing
type admits that addition additively (the fold is an enum, the parse carries spans not text) — so the
door is open, and nothing here must be re-signed to walk through it. The re-entry trigger is
discovered need (`27D`), not a scheduled revisit.

## 4. The eval'er surface + the `24T` §6 asserted-semantics (report-asks #4/#5)

`oracle::evaler::detect_evaler` reads the eval'er off an ORDINARY predict body
(`271:rul-evaler-merge-no-structure-member`: no structure member). A body that delegates to the
`dorc:sh`/`dorc-sh` reentry primitive IS an eval'er; the reentry head chooses the room (Invited /
none), the env-idiom gives the ρ-claim (`RhoClaim`, reused from `wrapper`), and the reentry argv gives
the payload shape (`-c <word>` / `-s` / file). A bare-`sh` delegation is NOT a recognized reentry —
it is the escape hatch (`274` §12 finding-scope-clarification). Authored delegation is an ACTUAL
COMMAND (the `dorc:sh` primitive), never `eval` (`dialect-quality-law`).

**`24T` §6 asserted-semantics landed as tests:**

- **L3** (positional binding `sh -c CODE NAME ARGS` ⇒ `$0`=NAME, `$@`=ARGS) — PINNED. Modeled by
  `EvalerShape::DashC { binds_argv }` (the trailing `"$@"` in the reentry) + `plan::shim::shim_script`
  passing `"$@"` verbatim to the pinned evaluator. Tests: `dash_c_reentry_invited_full_ambient`,
  `dash_s_reentry_is_stdin_code` (evaler); `shim_script_is_host_independent_but_for_the_evaluator_line`
  (shim).
- **L1** (child sh doesn't inherit `set -e`; fresh shell-options) / **L2** (export-only var
  inheritance) / **L4** (unquoted-heredoc no field-splitting) / **L5** (commented heredoc lines still
  outer-expand) / **L7** (quoting-reconstruction fidelity) — **DIFFERENTIAL-vs-dash obligations,
  transferred not discharged** (`274` §9: "the differential obligations transfer, they do not
  disappear"). They discharge against the reentry FORM + the shim, but the differential harness needs
  the actual reentry EXECUTION (probe-shipping, task-14-gated). Recorded as the standing sweep-axis
  for the shipping lane. **L6** (bash `BASH_FUNC_*` leak) stays note-only (a ⊤-risk edge, unmodeled).

## 5. The per-run PATH shim (report-ask; `274` §5)

`plan::shim` — the pure/DST-clean half: `shim_script(evaluator)` (host-independent text; the one
`exec <evaluator> "$@"` line is the sole variance = the DST seam, so goldens stay host-agnostic) ·
`shim_dir_name(run_id)` (run-id-derived, no mktemp randomness; stale dirs inert) · `classify_shim_rc`
+ `smoke_degrades_session` (the failure lattice: 0/1 ⇒ Ran, everything else including the
shim-unavailability codes 125/126/127 ⇒ CantSay ⇒ RUN; a failed preamble smoke degrades the WHOLE
session shimless — one session-level degrade, not scattered 127s). Materialization (atomic
write-then-rename at session-establishment, PATH-prepend, cleanup, the hostsim command-registration
seam) is the cli/hostsim I/O edge, task-14-coupled — NOT built this lane.

## 6. tc-* flags carried forward (NEVER resolved here) + findings to flag UP

- **`tc-room-tag-on-fact-vs-factkey`** (NEW) — when payload-inner facts become REAL (plan-wiring), the
  room (`RoomTag`) and the reentry's ρ/context must key into the fact plane so an Invited-room fact and
  a HostDefault fact of the same cell do not collide, AND so a HintOnly fact never reaches a mint. This
  is the SAME cross-cutting shape as lane-wrapper-peel's `tc-context-slot-on-coord-not-factkey` (`27K`
  §7): the room is a second axis on the fact key. FLAGGED for the plan-wiring brief; NOT resolved.
- **`tc-book-bare-sh-room-source`** (NEW, a design tension worth a human eye) — `classify_evaler_head`
  maps a BARE book `sh -c` head to `HintOnly`, but `274` §12 finding-scope-clarification says a book
  bare `sh -c` site DECOMPOSES (and may license) via the stdlib sh-oracle's `dorc:sh` reentry. So the
  operative room for a book site is the HANDLING ORACLE's reentry head, NOT the book head. The
  head-classifier is correct as a LEXICAL primitive; the plan-wiring must source the room from the
  dispatched eval'er's reentry (`detect_evaler(...).room`), never from the book-site head. Flagged so
  the wiring does not wrongly wall admin bare-`sh` sites as hint-only.
- **`inv-superposition`**: nothing needed flagging UP — all models are phase-agnostic data (no phase
  baking).

**Build-surfaced findings (facts, not tc-judgments):**

- **finding-while-true-argparse-out-of-dialect** — the `274` §4 flagship sh-oracle strawman uses
  `while :; do case … esac; done`, but the predict dialect requires a `[ ]` test (`while :` fails to
  lift: "expected `[` to open a test"). The eval'er strawmen were re-spelled to the `case`-based
  argparse (which lifts). Flagged for the stdlib eval'er-authoring brief: either the strawmen adopt
  `case`-argparse or the dialect grows `while :` (a deliberate `syntactic-top-triggers` shrink).
- **finding-dorc-colon-unmockable-on-windows** — a `dorc:sh` command word cannot be a mock file
  (colon illegal in Windows filenames), and a `dorc:sh` apply site 127s under stock sh (by design —
  grammar-valid/world-invalid). The e2e cases therefore use bare `sh` (mockable) + no-`dorc:sh`-exec;
  a `dorc:sh` exec case would need the shim materialized (task-14). Not a defect — a fixture
  constraint the shipping lane inherits.

## 7. e2e tally (report-ask #7, verbatim)

```
all 82 e2e round-trips passed (ap-2 dash -n + apply/probe exec gates, redirect sandbox,
ordered run-set, stderr floor, argv-echo differential, dual-rail license judge, why-lens emission)
```

Two new cases (corpus idiom, inert mocks under `PATH=mocks-only`):

- `payload-bare-sh-c-walls` — `sh -c 'hork tune'`, no eval'er oracle ⇒ the payload site walls
  opaquely, runs verbatim (`274` §12; empty-world-byte-identical for the payload construct). The inert
  `mocks/sh` swallows the payload (never decomposes it).
- `payload-composed-shape-walls` — `echo data | sudo sh -c 'cat /etc/motd'` (the `24T` §1 composed
  shape: pipeline ∘ context ∘ payload) analyzes end-to-end without crashing; every leg walls, the
  `sudo` context leg takes the honest wall, the site runs verbatim. Upgrades transparently when
  lane-context-entry + eval'er oracles land. (A read-only payload replaces the brief's `>> /etc/f`
  redirect for exec-safety; the `sudo` wall never evaluates it anyway.)

## 8. Fences honored (`24T` §5b/§9)

No general syntax-position holes (`imp-P1`), no automata carriage, no loop-assembly (`imp-P2`); R1
span-edits inside verbatim payload bodies stay follow-on; the 23J elevated-wrapper carve stays parked.
The accept/refuse frontier (§2) draws exactly this line. The synthesized-payload-render door (§3) stays
open by construction.

## 9. Where the next lane picks up

The plan-wiring follow-on (task-14-coupled) consumes the shapes landed here: `RoomFact`/`RoomTag`
(`core::room`) for the license gate, `detect_evaler`/`EvalerShape` (`oracle::evaler`) for the reentry,
`classify_payload_form`/`parse_payload`/`fold_line` (`oracle::payload`) for the decomposition, and
`plan::shim` for the probe-shipping. The two hard hand-offs: (a) `tc-room-tag-on-fact-vs-factkey` —
thread the room + ρ/context into `FactKey` so payload-inner facts key correctly and a HintOnly fact
never reaches a mint; (b) `tc-book-bare-sh-room-source` — source the room from the dispatched
eval'er's reentry head, not the book-site head. Both flagged, neither resolved.
