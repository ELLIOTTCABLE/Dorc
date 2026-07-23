# Round 29 Ingress Closeout Handoff

## Critical

Any successor MUST stop before acting and reread 298, the 297 immediate and phase-three packet, the relevant builder invariants, and this handoff. Do not resume from this summary alone.

## Lineage

Branch: `ai/r29-resume-ingress-cli-live`.

The current chain is phase one/two closeout (`4b95a9b4`), records 3A bounded admission (`e2b8958a` through `3356df0e`), whylog 3B (`db29c669` through `f2481a33`), diagnostic admission (`a17c5f92`, `9a20c9a8`), live 3C1 (`ce8341c9`), replay/writer 3C2 (`daa788b3`, `ff168b2b`, `d19e19a4`, `e0d16667`), then the incomplete fixture checkpoint `e8dc3f12`.

## Implemented, Not Accepted

The checkpoint contains strict harness evidence normalization, deterministic legacy derivation-family closure, the narrow loom terminal-newline restoration, and the hostsim byte-fault route through bounded `read_host_evidence` plus unscoped admission.

The immediate unit is NOT accepted. Full gates are NOT green. Opaque review has NOT run. Last known green components were loom fixpoint, focused hostsim admission, and the previously reported production build, clippy, and focused tests.

The latest strict e2e result is 7/97 failures:

- `probe-operand-quoting`: empty fixture records conflict with the current `sites=2` census.
- `glob-for-word-runs`, `strawman24-mixed-real`, `strawman24-opaque-wall`, and `strawman24-partial-oracle`: fixture site-record counts differ from the current emitted `sites=N` census, so strict admission refuses them.
- `decline27-tier3-dynamic`: report-only `sites=0` is returned as `NoObservation`, discarding the report and preventing its expected why diagnostic.
- `survivebite27-naked-trust-chain`: replay fails on the corresponding strict admission/replay path.

The generic harness path strips stale fixture framing, regenerates current identity, preserves normalized inner records, supplies legacy-default `rc=0`, and emits missing `deriv-end` closures in source order. It is committed, but the above census and report semantics require adjudication.

## Next Decision

A fresh conductor/builder must decide strict `sites=N` semantics for empty or mismatched fixture records, report-only `sites=0`, and replay BEFORE editing. No BLESS and no expectation changes without that adjudication.

Preserve separate branch `ai/r29-ingress` dirty evidence and unrelated `ai/r28-phase3-unit3-lock`; do not touch either. Phases 4 and 5 remain out of scope.
