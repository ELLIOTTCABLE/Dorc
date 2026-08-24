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

(none yet)

## deviations reported by builders

(none yet)

## conductor-owed at close

- `plans/30R` reconciliation against `30Rb` (out-of-quarantine surface; opacity
  discipline — mechanism yes, threat reasoning no).
- `LIVING_STATUS` entry; `FORFEITS` row for no-append (`30Ra:no-append-in-v1`).
- prose queue: every register minted `None` this arc.
- `gate:arc` from the populated branch before the fold.
