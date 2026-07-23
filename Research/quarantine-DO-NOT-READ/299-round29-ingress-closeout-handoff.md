# Round 29 Ingress Closeout Handoff

## Critical

Any successor MUST stop before acting and reread 298, the 297 immediate and phase-three packet, the relevant builder invariants, and this handoff. Do not resume from this summary alone.

## Lineage

Merged into `ai/main` after `dfe8fdb5`. The source branch was
`ai/r29-resume-ingress-cli-live`.

## Implemented, Awaiting Review

The immediate implementation is merged. Strict live and replay admission, controller
scope, bounded whylog v2, fixture normalization, derivation closure, report-only evidence,
and hostsim byte-fault admission are complete.

The exact merged lineage passed the WSL workspace tests, 97/97 e2e cases, fmt, clippy
with warnings denied, cargo-deny, typos, and the loom fixpoint. Native shell-dependent
tests remain unavailable because `dash`/`sh` is absent from the Windows PATH.

The immediate unit is NOT YET accepted because focused opaque accrual review has not run.

## Next Decision

Run the focused opaque accrual review over the exact merged revision. Resolve any
constraint, then mark only the four immediate ledger rows accepted. Do not begin phases
four or five in that review.

Preserve separate branch `ai/r29-ingress` dirty evidence and unrelated `ai/r28-phase3-unit3-lock`; do not touch either. Phases 4 and 5 remain out of scope.
