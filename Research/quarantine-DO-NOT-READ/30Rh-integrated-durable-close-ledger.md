# 30Rh — integrated durable close: builder ledger

> Tier: quarantined builder ledger for `30Re:sched-integrated-durable-close`. Current state, not
> chronology. Owner: this lane's builder. Conductor-facing account is `30Re`.

## lane identity

- Worktree: `.claude/worktrees/r30-receipt`
- Branch: `ai/r30-receipt`
- Base tip at dispatch: `a493aaa62548701f25cfcdd5682e0d3600f5895b` (clean, verified)
- Current tip: `a493aaa6` (no work committed yet)
- Dirt: none at dispatch

## brief deviations found while reading

- **`30Rh:dev-four-crates-have-no-steering`** — the brief directs a full read of crate steering for
  `plan`, `cli`, `aid`, `dorc-loom`, `receipt`, `receipt-crypto`, and `receipt-local`. Only
  `aid`, `analysis`, `cli`, `core`, `hostsim`, `oracle`, `plan`, `syntax`, `weft` carry a
  `CLAUDE.md`; `dorc-loom`, `receipt`, `receipt-crypto`, and `receipt-local` have none. Read what
  exists; authored no steering for the four (the brief forbids steering edits).

## as-built census — the old surface this lane removes

### CLI flags (`cli/src/lib.rs`)

| Current spelling | Field | Meaning today | Target |
|---|---|---|---|
| `--whylog-dir <dir>` | `whylog_dir` | durable write destination + `--last` read root | `--receipts <folder>` |
| `--whylog <file>` | `whylog` | exact old durable to replay (`why` only) | DELETE |
| `--no-whylog` | `no_whylog` | write no durable this run | OPEN — see `30Rh:open-suppression-spelling` |
| `--receipt <id>` | `receipt` | exact recorded identity in the store | `--receipt-id <id>` |
| `--last` | `last` | replay newest durable in `--whylog-dir` | `--receipt-last` |
| (none) | — | — | NEW `--receipt <file>`, explicit report-only root |
| `--all` | `all` | `RecordedSelection::Every` — whole-store enumeration | depth only; `Every` deleted |

Selection seat: `Args::recorded_selection` → `engine::RecordedSelection{Named,Every,Latest}`
(`cli/src/engine.rs:2207-2233`). `Every` is the arm the brief deletes.

### The six gate-8 replay-pair cases (`crates/cli/tests/`)

All six declare `run: round-trip` + `fixpoint: executed` + `why-addr:` + `expect-why-chain:`, and
every one of them asserts exactly ONE needle — `=== OUTCOME ===`. The remaining
`expect-why-chain` lines in `survivebite27` are `#`-comments, which `needles_missing`
(`e2e.rs:644`) skips.

1. `survivebite27-naked-trust-chain.loom` (addr 12, `--risk-faultless-skips`)
2. `whygallery-decline-unsound-arm.loom` (addr 9)
3. `whygallery-elide-and-hand-guard.loom` (addr 8)
4. `whygallery-survive-trusted-footprint.loom` (addr 10, `--risk-faultless-skips`)
5. `whygallery-wall-guards-downstream.loom` (addr 9)
6. `whygallery-webhost-whole.loom` (addr 14, `--risk-faultless-skips`)

Gate: `e2e.rs::scan_why_chain` (`e2e.rs:2752`). Today its replay arm writes with
`--whylog-dir=<scratch>` and reads with `--last --whylog-dir=<scratch>`.

### The seventh case

`whygallery-drifted-book-degraded-receipt.loom` — carries a committed `.whylog` inline and pins the
drifted-replay degrade path (`28F:rul-drift-replay-d1`). Outside gate-8 (no `expect-why-chain`).
Disposition owed; see `30Rh:open-seventh-case`.

### `results::replayed_records`

One production caller — `cli/src/engine.rs:1422`. One census entry —
`receipt/tests/crate_boundary.rs:766,777`. D5 requires caller count zero.

## the blocking finding

- **`30Rh:fnd-store-route-lists-it-does-not-explain`** — `Args::answers_from_the_receipt_store()`
  routes to `main.rs::read_receipt_store` → `engine::report_recorded_store`, which emits a recorded
  LISTING (`sites 1`, `signing-key …`, `opaque … source-path book.sh`) and **takes no why address**.
  The `=== OUTCOME ===` needle is the live why triptych's panel header, rendered from
  `cli/src/why.rs`. So `dorc why <address> --receipt-last` cannot land the existing needle today:
  the receipt-backed arm needs an address-directed, report-only render over the RECORDED model.
  It may not reach it by re-running the kernel off recorded bytes — the brief and
  `plan/CLAUDE.md inv-reingested-material-never-authorizes-action` both forbid that, and
  `results::replayed_records` is on the deletion list.

  This is the substance of section B and the largest single piece of the lane. It is NOT a
  `30Rd` STOP condition: it needs no second format, no compatibility reader, no authority
  conversion, and no new observation — only a report projection over material the receipt
  already carries.

## open items for conductor ruling

- **`30Rh:open-suppression-spelling`** — the brief's D5 list deletes `--no-whylog` parsing/help and
  section A names no successor. But `AID-NEEDS:law-whylog-is-sensitive` holds that a receipt is
  host metadata written unprompted, so refusing one must be typeable, and
  `30Rd:v1-acceptance-and-exit` #16 keeps default-on writing. Deleting the flag outright removes
  the only typed refusal. PROPOSED: rename in place to `--no-receipt`
  (`rul-strawman-formats-no-compat` — rename, no alias, all sites one commit). It is not an alias,
  not a compatibility spelling, and not on the brief's non-goals list. Flagged rather than
  silently resolved because section A enumerates the vocabulary and this spelling is not in it.

- **`30Rh:open-seventh-case`** — pending inspection of what the embedded `.whylog` actually pins
  versus what the recorded source-standing model can express.

## state

- Reading complete: root `README`/`DESIGN`/`IMPLEMENTATION`, `spike/CLAUDE.md`, `plan`/`cli`/`aid`
  crate steering, quarantine `AGENTS.for-builders-only.md`, `30Re`, `30R`,
  `30Ra:receipt-rooted-attention-and-selection`, `30Rb` (reingestion-and-why, Stage 5, Stage 5A,
  Stage 6, DST/product routes, verification tooling, stop conditions),
  `30Rd` (controller-root-resolution, store-enumeration-and-last-selection, D4, D5,
  v1-acceptance-and-exit, test-and-fixture-fences, stop conditions).
- Work committed: none yet.
- Tests/gate: none run yet.
