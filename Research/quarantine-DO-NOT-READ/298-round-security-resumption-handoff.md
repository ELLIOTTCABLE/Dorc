# 298 - Round 29 resumption handoff

> SUPERSEDED, kept for its construction record (ported to the `ai/main` lineage
> 2026-07-24; it had lived only on `ai/r29-resume`, which made `299`'s "reread 298"
> instruction unfollowable from `ai/main`). Phases zero through three subsequently
> landed via the `ai/r29-resume-*` lanes and are merged; the implementation-ledger
> table below is the PRE-resume state and no longer describes the tree. Current
> status, the corrected gate disposition, and the outstanding work: `29A`.

This is the sole current handoff for resuming round 29 after the 2026-07-20 harness
crash. It records implementation state, not a new design or adjudication. `290` through
`295` remain the review record; `297-security-refresh-build-plan.md` remains the build
plan. Do not copy this status into non-quarantine documents, including
`Research/LIVING_STATUS.md`.

## Resume point

Resume from branch `ai/r29-resume`, which is a documentation-only checkpoint based on
`ai/main` at `0c259317b4b13a8aa8af1098f1f3f6784b9cee74`. It intentionally contains no
unreviewed round-29 code port. Its purpose is to give the next conductor one current-main
starting point without falsely presenting stale security work as integrated.

Do not merge `ai/r29-ingress` wholesale. At this checkpoint it is seven commits ahead of
and 133 commits behind `ai/main`; Git reports a content conflict in
`spike/crates/plan/src/lib.rs` and auto-merges ten other semantically overlapping files.
The old branch is evidence and a source for a deliberate port, not an integration branch.

## Review disposition

The round-29 review is complete. `295` adjudicated two hold-now defects:

1. The report-capture pathname protocol can clobber a host file.
2. Aggregate mutation elision can bypass the per-establish verdict-vouch requirement.

The review also banked host ingress, identity, sensitive artifacts, and reactive
revocation as later re-entry gates. The exported construction rules are in
`AGENTS.for-builders-only.md`; do not weaken or reinterpret them while resuming.

## Implementation ledger

| phase | state | authoritative revision / location |
|---|---|---|
| 0 - seam map and freeze | complete, not ported to current main | `c69b1b00` and the phase packet below |
| 1 - owned report channel | complete, not ported | `54749181`; runtime report capture is disabled rather than routed through a pathname |
| 2 - every establish vouched | complete, not ported | `e24ce519` plus `7ed9136d` |
| 3 - bounded host evidence | implemented but rejected | `ai/r29-ingress@ac58daf7` plus the dirty patch recorded below |
| 4 - sink encoding and artifacts | not begun | intentionally unfrozen by `297` |
| 5 - production fences and authority regression gates | not begun | intentionally unfrozen by `297` |

Phase-three design detail and its execution rulings live only at
`ai/r29-ingress:Research/quarantine-DO-NOT-READ/297-security-refresh-phase-packets.md`.
Read its phase-three section before modifying ingress, replay, or whylog code. The file
has not been ported to `ai/main`; that omission is deliberate until the phase-three work
is accepted.

## Stranded worktree state

The sole dirty round-29 worktree is `.claude/worktrees/r29-ingress`, at
`ac58daf72c83cd83eb7ee70fe7f8fa3c7778efb4`. Its unstaged patch has content hash
`331156030e63f328f9aed40a149537ed14ff2cd4` and touches only:

- `spike/crates/cli/src/main.rs`
- `spike/crates/plan/src/records.rs`
- `spike/crates/plan/src/whylog.rs`

The patch closes two conductor-found gaps: a public raw `AttemptScope` constructor and
pre-admission cloning of the complete host buffer. It also prevents a no-observation
attempt from writing a whylog and drops oversized whylogs whole rather than truncating
them into corrupt durables.

It is not acceptable to commit or port unchanged. Direct review found these remaining
phase-three violations of the frozen packet:

- `records::deframe(&str, ..., LegacyPolicy)` remains public and accepts raw input.
- `whylog::parse(&str)` remains public and accepts raw input.
- `AttemptScope::width_one(&Framing)` is public; its current width-one projection
  conflates target and generation with the book digest, and source-set with the nonce.
- `admit_whylog` takes one host-evidence budget and returns an unscoped `WhylogDoc`,
  rather than separately bounded outer durable and inner host evidence with scope.
- The packet's independent free-field ceiling is not demonstrably enforced on every
  retained free-text path.
- Full verification predates this dirty patch; no exact-revision gate result exists.

The `r29-ingress` working tree must be preserved until a successor either repairs and
commits it after review or deliberately reconstructs its useful subset on a fresh branch.
No later round-29 repair commit exists in branch reflogs or unreachable-commit review.

## Required resumption sequence

1. Read `290`, `295`, `297`, this document, the builder invariants, and the phase-three
   packet from `ai/r29-ingress`.
2. Repair phase three against the frozen packet. Keep raw bytes, fixture identity,
   durable text, and admitted production evidence in distinct types and conversion paths.
3. Run the phase-three boundary matrix and the complete required gate set on the exact
   repaired revision. Do not bless fixtures.
4. Port only accepted phases 0 through 3 onto a fresh branch from current `ai/main`.
   Resolve `plan/src/lib.rs` deliberately and review every auto-merged overlap; do not
   use a blind merge as evidence of correctness.
5. Freeze phase four only after the accepted ingress types establish where hostile bytes
   stop. Then implement sink-specific encoding, sensitivity separation, and artifact
   handling.
6. Freeze and implement phase five: production/fixture fences, authority-mint manifest,
   exact source-set identity, and compile-fail bypass coverage.
7. Run the required opaque accrual review and cold verification against the exact merged
   revision. Only then update this ledger's implementation state.

## Status hygiene

`Research/LIVING_STATUS.md`, `Research/README.md`, and other ordinary project status
documents intentionally remain untouched. This round is quarantined; this document and
the numbered round-29 materials are its only status surface.
