# 30Rc — receipt build: conductor ledger

> Tier: quarantined, conductor state. `30Ra` owns design/rationale; `30Rb` owns the
> build specification. This file duplicates neither: it carries only lane state,
> adjudications, and deviations.

## lanes

Conductor: `ai/r30-conduct` @ `.claude/worktrees/r30-conduct`.
Build lane: `ai/r30-receipt` @ `.claude/worktrees/r30-receipt` (serial stages).
Stage-2 lanes branch from the Stage-1 tip; fold 2A → 2B → 2C.

| Stage | Lane | State |
|---|---|---|
| 0 laws/crate/deps/vectors | `ai/r30-receipt` | not started |
| 1 identity + plain kernel | `ai/r30-receipt` | not started |
| 2A apply image | tbd | not started |
| 2B overlay + age | tbd | not started |
| 2C recorded models + graph | tbd | not started |
| 3 presented plan + PlanReceipt | `ai/r30-receipt` | not started |
| 4 intent/dispatch/outcome | `ai/r30-receipt` | not started |
| 5 why/correlation/re-derivation | `ai/r30-receipt` | not started |
| 6 rip old implementation | `ai/r30-receipt` | not started |

Every builder: scout → STOP → conductor ack → implement.

## standing brief riders (carry into every dispatch this arc)

- Opacity: out-quarantine artifacts carry mechanism, never reasoning. `adversarial`
  and `threat` never appear outside quarantine; `untrusted`/`authenticated` are code
  identifiers only, not prose. Comment budget near zero — a doc-comment states the
  mechanical contract and stops.
- Citations spell `quarantine/<docID>[:slug]`. Never the directory's real name in a
  citation, never a resolvable path; the non-resolving form is the speed-bump.
- Non-correctness tooling chafe (lints, hk routing, mise ergonomics, config) is
  FIXED, not documented or worked around — human-authorized 2026-08-24. Correctness,
  testing, and verification machinery is excluded and escalates to me.
- Read-only Sonnet scouts: 1–2 concurrent, ~3–4 per stage; find and collate, never
  opine or decide. One carve: a narrow read-write scout may repair tooling per above.
- Every builder: scout → STOP → conductor ack → implement.

## adjudications

Stage 0 checkpoint, 2026-08-24:

- **Crate split — a deliberate deviation from `30Rb:result-and-exit` item 1.** `age`
  pulls `rand`, and `plan -> dorc-receipt -> age` would land an RNG in the kernel's
  graph. Root `AGENTS.md` (human-authored, outranks `30Rb`) requires kernels stay
  clean of nondeterministic deps. A cargo feature gate fails to workspace feature
  unification. So: pure `dorc-receipt` (models, grammar, ids, limits, overlay
  validator, PAE, digests, reader/writer states, graph, reingestion, capability
  traits) + `dorc-receipt-crypto` (Age/Ed25519 impls only). `plan` takes the pure
  crate; `cli` takes both. Every authority mint stays pure-side — the impl crate
  cannot mint checked/trusted/complete states — so the split strengthens
  verify-before-interpret rather than relocating it. `sha2` is the one dependency
  reaching the kernel; deterministic, and chosen over moving the hand-rolled
  vector-pinned SHA-256 in, because taking a second unforced deviation on top of a
  forced one is creativity.
- `age` pinned 0.12.x with `armor` explicit (not a default feature). 0.11 measured
  as a data point only; a switch returns to me.
- `core::spine::InvocationMode` untouched (human-ruled at `30N` §4); the receipt
  mints its own mode enum from the CLI dispatch seat. `attempt` ADDED to the
  `invocation` record — `sinv-controller-attribution` binds regardless of the
  field list, and the checkpoint is where the reviewed table is set.
- `render-decision`: subject as integer-or-`absent`, axis carried in `kind`, detail
  tag selected by kind. Protects `30N:rul-region-refusal-discloses-region-keyed`.
- Splits accepted rather than merges: `survival.outcome` (8) — a solver defect
  reported as a book fact is mis-attribution, `271:rul-sin-ordering`'s worst tier;
  `site-classification.class` (8) — the bools decide licensing.
  `SessionOutcome::LostAfterSend` maps to `unknown`, never `transport-failed`.
- `apply-context` added at order 16 of the opaque-field-tag table.
- `solve-certification.pass` projects the closed five-token set now, not waiting on
  `30M`. `region-decision.routes` keeps the total; the keyed/unkeyed split takes a
  projection-omission.

## deviations reported by builders

(none yet)

## banked for later stages

- Stage 4: `transport::SessionOutcome` is whole-artifact, not per-site, and no apply
  executor exists — V1 per-site rows come from hostsim/DST only.
- Stage 2A: `ArtifactSet`/`ArtifactFile` discard modes, roots, and edges, and hold
  bytes as `String`; the image container is binary-safe. 2A is "carry topology the
  live type throws away", not "add a container".
- Stage 5: `dorc-loom` reads receipts and so needs a verifier; decide there whether
  it takes the impl crate or a fixture verifier.
- `mise run lint:docids` accepts and checks `quarantine/<docID>` (prefix transparent
  to the matcher, resolved against a name-only listing — no quarantined file opened).
  It scans Markdown only; `.rs` citations are unvalidated, a crate `CLAUDE.md` is not.

## invariant prose (raw; for the design agent to synthesize — do not pre-format)

(collecting)

## conductor-owed at close

- `plans/30R` reconciliation against `30Rb` (out-of-quarantine surface; opacity
  discipline — mechanism yes, threat reasoning no).
- `LIVING_STATUS` entry; `FORFEITS` row for no-append (`30Ra:no-append-in-v1`).
- prose queue: every register minted `None` this arc.
- `gate:arc` from the populated branch before the fold.
