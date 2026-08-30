# 30Re — manual receipt conduct

> Tier: quarantined conductor state for the resumed receipt close. This is current state,
> not chronology. `30Rc` remains the inherited arc account; this file owns the manual-dispatch
> continuation after the in-process subagent harness repeatedly stopped without work.

## current objective

Conductor work is closed. The human owns review of the unified secure-durables tip and may remit one
focused repair afterward. No further work is dispatched from this ledger; all worktrees remain
standing until the human rules review/repair complete.

## branch state

- `ai/r30-receipt` and `ai/r30-conduct` both point to unified tip `e2a2e80d`, parallel to
  `ai/main`; the tip includes the current `ai/main` line through its explicit merge plus all receipt
  implementation, living-register, and conduct-ledger work.
- The unified tip carries the explicit store root and sealed
  `dorc-receipt::report::RecordedWhyFacts` boundary. It has not been upstreamed.
- Earlier diagnostic/harness lanes are fully represented in the unified branch. Their worktrees stay
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

## durable handoff

Built and unified:

- local keyset/provider and immutable receipt store, including explicit `--receipts` siting;
- plan/intent/outcome receipt species, exact grammar, rich/plain projection, graph, and rooted
  selectors;
- exact general-sh source custody, valid-`dorc-lang` withholding, and durable locator projection;
- sealed pure `dorc-receipt::report::RecordedWhyFacts` integrated from the real receipt-reading
  route with encoder-mediated arbitrary-value exit;
- coherent receipt CLI vocabulary and `--no-receipt` suppression;
- living analyzer/aid/research registers and the complete `30R` / extended `30Ra` target.

Review/repair residue, not silently complete:

- three report-API close items from `30Rh`: site newtype, parsed order token, and graph-derived
  closure membership;
- receipt crate public-export/steering close and final D5 deletion/singular implementation census;
- loaded-source locator composition remains explicit uncollected V1 residue;
- full recorded-why arrangement and replacement user-facing cases are rehomed outside quarantine;
- report-only kernel re-derivation remains a separate kernel-authorized round;
- final builder completion and conductor `gate:arc` have not run at this unified tip.

Unified branches `ai/r30-receipt` and `ai/r30-conduct` point together, include the current `ai/main`
line by merge, and have not been upstreamed. Worktrees and earlier lane branches are intentionally
left untouched for post-review repair.
