# 28P — Oracle loading and resolution: the resume conduct-ledger

Conductor ledger for the post-checkpoint half of the `plans/28K` lane (branch
`ai/r28-oracle-loading`, worktree `r28-oracle-loading`), session
`r28-megamerge-continuation-impl`, resumed 2026-07-31. Predecessor build ledger:
`notes/28O` (historical; stages A/B/G/D/E + the rebase). The ONLY live implementation
plan is `28K` §10 (bitem0–bitem9 + fold checklist); on committee-corner conflict,
`28M` governs (§7 ack-ledger, §8 license-plane ground truth). Confidence marks per
`spike/CLAUDE.md`.

## Standing state at resume

- Lane re-rebased onto `ai/main` tip "(AI dsn re) Rewrite the build shape into the
  slugged resume plan; bank the demotion-is-not-deletion lean" — 38 commits replayed,
  zero conflicts; main-side delta was docs-only (28K §10 rewrite, 28M §7/§8 growth,
  LIVING_STATUS, TODO, CONTRIBUTING, USER_STORY line). Quick-gate verification run
  post-rebase.
- E→F checkpoint CLEARED per LIVING_STATUS (28K §9 fully typed-closed; full-positional
  regime ACKED spike-tier; committee fence unresolved-but-motion-authorized,
  build-as-spiked, marked unratified in code + ledger).
- Stage-F riders banked in `28M` §7 carried into the bitem briefs: the
  `WhyReport.oracle_paths`/`oracle_srcs` rename (bitem7) · fold-glance at the
  load-inert refusal newly firing on three loom-era lint cases · the pre-existing
  Windows-only `mise run loom:compile` stack overflow on
  `syntax-unsupported-nesting-bound` (NOT this lane's).

## Dispatch log (compressed as lanes land)

- (pending) builder-1: bitem0 positional-regime conversion → bitem1
  pin-by-definition-bytes → bitem2 resolution-seat unification.

## Findings / deviations

(none yet)
