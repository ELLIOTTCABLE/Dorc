# 30Re — manual receipt conduct

> Tier: quarantined conductor state for the resumed receipt close. This is current state,
> not chronology. `30Rc` remains the inherited arc account; this file owns the manual-dispatch
> continuation after the in-process subagent harness repeatedly stopped without work.

## current objective

Conductor work is closed. The human owns review of the unified secure-durables tip and may remit one
focused repair afterward. No further work is dispatched from this ledger; all worktrees remain
standing until the human rules review/repair complete.

## branch state

- `ai/r30-receipt` and `ai/r30-conduct` point to the same unified review tip, parallel to
  `ai/main`; it includes the current `ai/main` line through its explicit merge plus all receipt
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

Security review repair is complete:

- prepared intent, image accounting, required publication, and permit are one ownership chain; the
  production/fixture bypass is deleted;
- receipt core claims cryptographic validity only; local-policy authentication is a private CLI
  envelope minted by the validated local-keyset edge;
- validated opaque detail has one class-aware encoder exit and no raw/revealing convenience path;
- Unix opens are handle-relative/non-following with ownership checks; cleanup declines on both
  platforms when removal cannot be conditioned on object identity, leaving bounded partial files.

Accepted carried limitations:

- production cannot clean interrupted files until an identity-conditioned removal or retention
  design exists; the deterministic model retains that success path because its node identity is real;
- loaded-source locator composition remains explicit uncollected V1 residue;
- three report-API close items from `30Rh` (site newtype, parsed order token, graph-derived closure)
  remain ordinary engineering work;
- recorded-why arrangement/help/cases are rehomed outside quarantine, and report-only kernel
  re-derivation remains a separate kernel-authorized round.

Transition is permitted, but not falsely green: the inert old whylog/replayed-record implementation
still awaits D5 deletion and the pre-existing CLI vocabulary/help failure set still blocks the full
gate. Both are now governed by the receipt crate contracts and can be completed without reopening
quarantined reasoning. Final completion and `gate:arc` remain owed by the successor.

`ai/r30-receipt` carries the repair commits; `ai/r30-conduct` must fast-forward to its final handoff
tip. Nothing is upstreamed. Worktrees and earlier lane branches remain intentionally standing.
