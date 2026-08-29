# 30Re — manual receipt conduct

> Tier: quarantined conductor state for the resumed receipt close. This is current state,
> not chronology. `30Rc` remains the inherited arc account; this file owns the manual-dispatch
> continuation after the in-process subagent harness repeatedly stopped without work.

## current objective

Finish the receipt family without trusting the predecessor's final deletion sequence. Work remains
serial. Each builder receives one `.claude/worktrees/*` lane and maintains a quarantined sibling
ledger (`30Rf`, `30Rg`, …). The human manually carries `_tmp-*.prompt.md` briefs from the primary
checkout into an external Opus harness.

## branch state

- Build source: `ai/r30-receipt` at `1cbee21a`.
- Diagnostic lane: `ai/r30-receipt-why-repair` in
  `.claude/worktrees/r30-receipt-why-repair`, clean at `d948d651`; implementation and binary
  coverage landed, with the ambiguity catalog row deliberately red pending an honest defining case.
- Current harness lane: `ai/r30-receipt-loom-code` in
  `.claude/worktrees/r30-receipt-loom-code`, clean at `d948d651` before dispatch.
- Conduct branch: `ai/r30-conduct`; this document is its resumption account.
- The two failed in-process harness dispatches produced no commit and no worktree mutation.

## testing adjudication

- **`30Re:fnd-production-durable-route-is-real`** — `cli/tests/durable_route.rs` drives the
  real binary across process restart with sandboxed standard roots, real local key/store I/O, and
  real receipt read-back. D4's central route has meaningful acceptance coverage even though its
  late builder gate report is not evidence.
- **`30Re:fnd-gate-eight-is-legacy-only`** — gate 8's live/replay pair writes and reads the old
  whylog through `--whylog-dir`; none of its six replay arms exercises the receipt store.
- **`30Re:fnd-seven-cases-depend-on-legacy`** — the six gate-8 cases are not the full deletion
  account. `whygallery-drifted-book-degraded-receipt.loom` directly carries an old `.whylog` and
  requires its own explicit disposition.
- **`30Re:fnd-why-selection-is-under-tested`** — the current receipt-store why seat writes bare
  stdout tokens. Only the no-receipt prefix is binary-tested; the ambiguity render is untested at
  the CLI seat although its store primitive is tested.
- **`30Re:rul-parity-migrates-before-deletion`** — D5 cannot begin until the six why-chain replay
  arms have a receipt-backed replacement and the seventh drift case has an explicit disposition.
  The diagnostic-seat repair is useful but does not substitute for replay parity.
- **`30Re:rul-deletion-must-add-positive-tests`** — a deletion-only diff routes zero checks; D5
  carries its replacement assertions in the same lane and reads the discovery-floor count rather
  than treating refusal or an empty selection as acceptance.
- **`30Re:fnd-whole-product-code-proof-is-missing`** — a real-binary test now honestly drives the
  same-order ambiguity, but the loom grammar cannot make that production diagnostic the defining
  case. A virtual `LocalIo` in `dorc-loom` would entrench an already-wrong public I/O boundary and
  duplicate store semantics; returning ambiguity to stdout would violate the catalog/stream laws.
- **`30Re:rul-whole-product-loom-proves-code`** — the repair is a narrow harness capability: a
  registered code may use a whole-product loom whose transcript/code proof is owned by the real e2e
  execution, just as whole-product loom transcript fixpoints already are. No diagnostic injection,
  virtual store, fixture provider, or production parser widening substitutes.

## serial schedule

1. **`30Re:sched-repair-why-diagnostics`** — implementation is landed at `d948d651`; completion
   waits on the honest ambiguity defining case. Builder ledger: quarantine `30Rf`.
2. **`30Re:sched-enable-whole-product-code-proof`** — let a registered diagnostic use a
   whole-product loom whose real e2e execution owns the code/transcript proof; finish the ambiguity
   case without synthetic store state. Builder ledger: quarantine `30Rg`.
3. **`30Re:sched-migrate-replay-parity`** — migrate gate 8's six replay arms to the receipt store,
   adjudicate the seventh drifted-receipt case, and prove the replacement while legacy remains.
   Builder ledger: quarantine `30Rh`.
4. **`30Re:sched-delete-legacy-durable`** — D5: delete the old format, writer, reader, flags,
   fixtures, and laundering caller only after stage 3 is green. Builder ledger: quarantine `30Ri`.
5. **`30Re:sched-adjudicate-dispatch-diagnostics`** — re-check the proposed three-code split
   against `28L:rul-reason-enums-not-sibling-codes` using the post-D5 world and measured reachability;
   do not inherit the predecessor's conclusion as a ruling. Builder ledger: quarantine `30Rj`.
6. **`30Re:sched-close-receipt-arc`** — fold serially, reconcile `30R`/status, synthesize steering
   prose, run the required review gates, then `gate:arc` from the populated branch before folding.

## current dispatch contract

`30Rg` owns only `30Re:sched-enable-whole-product-code-proof`. It must preserve the real-binary
ambiguity scenario already landed at `d948d651`, add no synthetic store state, and touch neither gate
8 nor any legacy durable surface. The builder reads the commit skill before work and commits the
red-first harness proof, implementation, and repairs granularly — intermediate broken commits are
expected evidence, not something to squash away. It ends with one both-platform completion gate at
one final tip.
