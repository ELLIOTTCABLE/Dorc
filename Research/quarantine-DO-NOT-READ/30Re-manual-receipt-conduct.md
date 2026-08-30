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

- `ai/r30-receipt` in `.claude/worktrees/r30-receipt` carries all implementation through the
  explicit store root and sealed `dorc-receipt::report::RecordedWhyFacts` boundary.
- `ai/r30-conduct` in `.claude/worktrees/r30-conduct` carries this conductor account; it folds into
  the final receipt tip before review.
- Earlier diagnostic/harness lanes are fully represented in the receipt branch. Their worktrees stay
  standing until post-review repair is ruled complete.

## testing adjudication

- **`30Re:fnd-production-durable-route-is-real`** — `cli/tests/durable_route.rs` drives the
  real binary across process restart with sandboxed standard roots, real local key/store I/O, and
  real receipt read-back. D4's central route has meaningful acceptance coverage even though its
  late builder gate report is not evidence.
- **`30Re:fnd-seven-cases-were-why-surface-tests`** — six gate-8 replay pairs and the drifted
  seventh case exercised the retired whylog renderer, not the secure receipt boundary. Their
  replacement is rehomed with recorded-why presentation outside quarantine; D5 may remove the old
  fixtures without pretending the sealed facts API is a finished user render.
- **`30Re:rul-deletion-must-select-real-checks`** — the deletion-heavy D5 diff must still route
  substantive existing receipt/CLI checks and report the discovery-floor count; no new meta-gate is
  added merely to count deletions.
- **`30Re:rul-receipt-root-is-attention-root`** — one selected root receipt plus the causally
  required graph closure approximates the human's run. Disconnected DAGs never merge. The surface is
  `--receipts <folder>` for store location, and why's mutually exclusive `--receipt <file>`,
  `--receipt-id <id>`, `--receipt-last` root selectors. `--all` controls explanation depth only.
- **`30Re:rul-general-source-bytes-and-locator`** — rich plan receipts retain exact acquired
  general-sh bytes that were not accepted as valid `dorc-lang`; valid `dorc-lang` bytes are omitted.
  Recorded sites project the existing locator DAG with exact byte spans instead of gaining a
  line-only locator. Current and historical `path:N` compare only the same physical line and never
  infer moved-line equivalence.
- **`30Re:rul-kernel-rederivation-is-deferred-not-deleted`** — replaying original inputs through a
  report-only kernel remains a 30R target, but the correctness kernel is frozen in this arc. The
  secure close seals `RecordedWhyFacts`; a later kernel-authorized round supplies re-derivation.
  Why arrangement over the sealed model is rehomed outside quarantine.
- **`30Re:rul-receipt-crate-is-the-boundary`** — `dorc-receipt::report` owns the pure sealed facts
  API and encoder-mediated arbitrary-value exit. CLI composes I/O and aid/weft rendering;
  `dorc-aid` remains receipt-unaware. The security arc closes the crate's curated internal-public
  surface and steering before handing it to ordinary Dorc work.

## serial schedule

1. **`30Re:sched-repair-why-diagnostics`** — complete at `6c1f8024`; builder ledgers quarantine
   `30Rf`/`30Rg`.
2. **`30Re:sched-secure-durable-close`** — rooted graph/source comparison and sealed
   `dorc-receipt::report::RecordedWhyFacts` are built. Remaining: close the curated receipt
   producer/read/report API, write crate steering, execute D5 deletion, and prove the singular
   provider/store/reader/writer census. Builder ledger: quarantine `30Rh`.
3. **`30Re:sched-close-receipt-security-arc`** — reconcile current truth, synthesize only durable
   steering that earns a permanent seat, run the required review and `gate:arc`, then offer the
   populated branch as the `ai/main` synthesis point. The dispatch-diagnostic split is not included.
4. **`30Re:sched-rehome-why-surface`** — after synthesis, a new non-quarantined conductor arranges
   and renders sealed recorded facts. Report-only kernel re-derivation is a separate deferred round.

## current dispatch contract

The sealed facts boundary is complete. Its API-close residue is narrow and security-relevant:
replace the bare site tuple and order string with receipt newtypes, derive closure membership from the
receipt graph rather than caller input, and preserve the encoder-mediated byte exit. Loaded-source
locator composition remains explicit V1 residue rather than a guessed locator. A fresh secure builder
may then close public exports/steering and D5 in one reviewed lane. Final why arrangement and the
six-plus-one user-facing cases are outside this arc. Kernel work, recorded-listing polish, the
dispatch split, corruption-parser expansion, and new test backstops remain excluded.
