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
- Diagnostic/harness work is complete on `ai/r30-receipt-loom-code` at `6c1f8024`; `30Rf` and
  `30Rg` carry the account, and the both-platform completion gate is green.
- Current close lane reuses `ai/r30-receipt` in `.claude/worktrees/r30-receipt`; the receipt
  vocabulary/attention ruling, old-ladder cut, and exact-general-source/durable-locator ruling are
  committed through `cc8fd3c2`. The temporary D5 worktree/branch was removed.
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
- **`30Re:rul-parity-migrates-before-deletion`** — D5 cannot begin until the six why-chain replay
  arms have a receipt-backed replacement and the seventh drift case has an explicit disposition.
  The diagnostic-seat repair is useful but does not substitute for replay parity.
- **`30Re:rul-deletion-must-add-positive-tests`** — a deletion-only diff routes zero checks; D5
  carries its replacement assertions in the same lane and reads the discovery-floor count rather
  than treating refusal or an empty selection as acceptance.
- **`30Re:rul-receipt-root-is-attention-root`** — one selected root receipt plus the causally
  required graph closure approximates the human's run. Disconnected DAGs never merge. The surface is
  `--receipts <folder>` for store location, and why's mutually exclusive `--receipt <file>`,
  `--receipt-id <id>`, `--receipt-last` root selectors. `--all` controls explanation depth only.
- **`30Re:rul-general-source-bytes-and-locator`** — rich plan receipts retain exact acquired
  general-sh bytes that were not accepted as valid `dorc-lang`; valid `dorc-lang` bytes are omitted.
  Recorded sites project the existing locator DAG with exact byte spans instead of gaining a
  line-only locator. Current and historical `path:N` compare only the same physical line and never
  infer moved-line equivalence.

## serial schedule

1. **`30Re:sched-repair-why-diagnostics`** — complete at `6c1f8024`; builder ledgers quarantine
   `30Rf`/`30Rg`.
2. **`30Re:sched-integrated-durable-close`** — in one lane, move the six existing replay-parity
   cases to the receipt store, disposition the seventh drifted old-format case, then execute D5's
   deletion and singular-implementation census. Builder ledger: quarantine `30Rh`.
3. **`30Re:sched-close-receipt-arc`** — fold, reconcile `30R`/status, synthesize only durable
   steering that earns a permanent seat, run the required review, then `gate:arc` from the populated
   branch before folding. The proposed dispatch-diagnostic split is not part of this close.

## current dispatch contract

`30Rh` owns `30Re:sched-integrated-durable-close` in the reused receipt worktree. The receipt
vocabulary and old-ladder deletion are already committed; the address-directed report exposed the
missing source custody/locator projection. The builder first adds exact general-sh custody and the
durable existing-locator projection, then MUST STOP with focused format/projection tests green.
Only after explicit resume does it build the recorded why render, complete the seven-case transition,
and finish D5. It does not chase `30Rg`'s deviations, the recorded-listing Debug spellings, the
dispatch-diagnostic split, corruption-parser expansion, or new test backstops. It commits granularly
and runs one both-platform completion gate only at the final tip.
