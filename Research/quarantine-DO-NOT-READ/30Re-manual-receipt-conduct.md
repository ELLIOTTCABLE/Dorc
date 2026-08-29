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
- Current lane: `ai/r30-receipt-why-repair` in
  `.claude/worktrees/r30-receipt-why-repair`, clean at `1cbee21a` before dispatch.
- Conduct branch: `ai/r30-conduct`; this document is its resumption account.
- The two failed harness dispatches produced no commit and no worktree mutation.

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

## serial schedule

1. **`30Re:sched-repair-why-diagnostics`** — move receipt-store why selection into the shared
   engine seat; restore typed catalog diagnostics and honest defining cases; no legacy deletion.
   Builder ledger: quarantine `30Rf`.
2. **`30Re:sched-migrate-replay-parity`** — migrate gate 8's six replay arms to the receipt store,
   adjudicate the seventh drifted-receipt case, and prove the replacement while legacy remains.
   Builder ledger: quarantine `30Rg`.
3. **`30Re:sched-delete-legacy-durable`** — D5: delete the old format, writer, reader, flags,
   fixtures, and laundering caller only after stage 2 is green. Builder ledger: quarantine `30Rh`.
4. **`30Re:sched-adjudicate-dispatch-diagnostics`** — re-check the proposed three-code split
   against `28L:rul-reason-enums-not-sibling-codes` using the post-D5 world and measured reachability;
   do not inherit the predecessor's conclusion as a ruling. Builder ledger: quarantine `30Ri`.
5. **`30Re:sched-close-receipt-arc`** — fold serially, reconcile `30R`/status, synthesize steering
   prose, run the required review gates, then `gate:arc` from the populated branch before folding.

## current dispatch contract

`30Rf` owns only `30Re:sched-repair-why-diagnostics`. It must not touch gate 8, any of the seven
legacy-dependent cases, legacy flags, `results::replayed_records`, D5, or the dispatch diagnostic.
New catalog registers remain prose-empty. The builder records current truth, commits granularly,
and ends with one both-platform completion gate at one final tip.
