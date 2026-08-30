# 30Rh — integrated durable close: builder ledger

> Tier: quarantined builder ledger for `30Re:sched-integrated-durable-close`. Current state, not
> chronology. Owner: this lane's builder. Conductor-facing account is `30Re`.

## lane identity

- Worktree: `.claude/worktrees/r30-receipt`
- Branch: `ai/r30-receipt`
- Base tip at dispatch: `a493aaa62548701f25cfcdd5682e0d3600f5895b` (clean, verified)
- Current tip: `2f661ace` — RED, `DORC_KNOWN_BROKEN`-acked (mid-cutover; see `30Rh:state-of-the-build`)
- Dirt: none

## commits

| Tip | What it did |
|---|---|
| `7b5a2c28` | This ledger, opened with the as-built census |
| `2f661ace` | Section A's vocabulary cutover + the old replay ladder's deletion (RED) |

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

## findings that shape section B

- **`30Rh:fnd-store-route-lists-it-does-not-explain`** — a receipt-reading `why` routes to
  `main.rs::read_receipt_store` → `engine::report_recorded_store`, which emits a recorded LISTING
  (`sites 1`, `signing-key …`, `opaque … source-path book.sh`) and **takes no why address**. The
  `=== OUTCOME ===` needle every gate-8 case asserts is the live why triptych's panel header,
  rendered from `cli/src/why.rs`. So `dorc why <address> --receipt-last` cannot land the existing
  needle today: the receipt-backed arm needs an address-directed, report-only render over the
  RECORDED model. It may not reach it by re-running the kernel off recorded bytes — the brief and
  `plan/CLAUDE.md inv-reingested-material-never-authorizes-action` both forbid that, and
  `results::replayed_records` is on the deletion list.

  This is the substance of section B and the largest single piece of the lane. It is NOT a
  `30Rd` STOP condition: it needs no second format, no compatibility reader, no authority
  conversion, and no new observation — only a report projection over material the receipt
  already carries.

- **`30Rh:fnd-recorded-sites-carry-no-line`** — and the constraint that shapes how. A
  `RecordedSiteDecision` (`receipt/src/plan.rs:292`) carries `RecordedSite{leaf, member}`, a
  `RecordedAst` **arena index** (`receipt/src/rows.rs:292` — an index into the parsed syntax arena,
  NOT a line), the disposition, the site's shell text as `OpaqueState`, and a `RecordedInfluence`.
  There is no line number anywhere in the projection. So `dorc why 12` cannot be answered from the
  document alone. Two routes exist and only one is a builder's to take:

  1. **Re-parse the CURRENT book** (its path is in the encrypted region), resolve line 12 to a
     leaf, and look up the RECORDED disposition by leaf id. Sound exactly when the recorded source
     digest matches — which is the drift check that already exists — and degraded when it does not.
     Nothing recorded drives an action; the parse is over current source, and
     `receipt/src/reingested.rs`'s `RecordedCurrent`/`ReDerivedDisposition` is already built to keep
     the recorded and re-derived arms distinct.
  2. **Record the line in the receipt.** A durable-content change, so
     `spike/CLAUDE.md rul-durable-contents-reviewed-before-design` puts it behind opaque review
     BEFORE the design settles. Not available to this lane.

  PROCEEDING ON 1, because 2 is closed to a builder. Reported to the human 2026-08-29 with the
  option to halt the lane and take the durable-content question to review instead; no halt ruled.

- **`30Rh:fnd-receipts-flag-has-no-store-seat`** — `--receipts <folder>` must be the EXACT store
  root, and no such seat exists. `receipt-local`'s `store::store_root` and its private `locations`
  both derive the root as `roots.product_root(RootRole::State).child(STORE_DIR)`, i.e. always
  `<state>/dorc/receipts-v1`. Honouring the ruled spelling needs an explicit-root override carried
  on `LocalReceiptEdgeV1` and threaded into both store opens, leaving the KEY root standard
  (`30Rd:controller-root-resolution`: "it never changes the standard configuration/key root").
  In scope, not yet built.

## rulings and open items

- **`30Rh:open-suppression-spelling`** — RULED, human, 2026-08-29: **ACK**. `--no-whylog` renames
  in place to `--no-receipt`. The brief's D5 list deletes the old parsing/help and section A named
  no successor, but `AID-NEEDS:law-whylog-is-sensitive` holds that a receipt is host metadata
  written unprompted, so refusing one must be typeable, and `30Rd:v1-acceptance-and-exit` #16 keeps
  writing default-on — deleting outright would remove the only typed refusal. A rename, not an
  alias, so `rul-strawman-formats-no-compat` is satisfied. LANDED at `2f661ace`.

- **`30Rh:open-seventh-case`** — pending inspection of what the embedded `.whylog` actually pins
  versus what the recorded source-standing model can express. Note that `2f661ace` already removed
  the drifted-replay path this case rides (`AcquiredEngine::Drifted`, `dorc_cli::drifted_receipt`),
  so the case is currently unbacked and its disposition is now load-bearing rather than tidy-up.

## state-of-the-build

- **Section A — LANDED at `2f661ace`, tree RED.** `--whylog-dir`→`--receipts`,
  `--receipt <id>`→`--receipt-id`, `--last`→`--receipt-last`, new `--receipt <file>`,
  `--whylog <file>` deleted, `--no-whylog`→`--no-receipt`, the three root selectors made mutually
  exclusive, `RecordedSelection::Every` (the `--all` whole-store arm) deleted so `--all` is depth
  only. `RecordedSelection` → `engine::ReceiptRoot{File,Id,Last}`; `Args::recorded_selection` →
  `Args::receipt_root`; `Args::answers_from_the_receipt_store` deleted (with `--whylog*` gone every
  receipt-reading `why` is store-answering, which is what its own doc comment said would happen).
  `StoreReading::cohort` → `terminal`, with `recorded::collapse_predecessors` implementing
  `30Rd:store-enumeration-and-last-selection`'s collapse: within the maximum-order cohort, members
  that are typed graph predecessors of another member drop out; a sole survivor is selectable,
  several incomparable survivors stay the ambiguity report, never a tie-break.
- **D5, partially executed early** (forced by the rename, not scope creep): `load_whylog_replay`,
  `read_replay_source`, `refuse_replay`, `receipt_has_nowhere_to_read`, the `Replay`/`ReplayLoad`
  types' use, the `AcquiredEngine::Drifted` arm, and — the one that matters —
  `PlanAuthority::authorise(dorc_plan::whylog::admit_unscoped_whylog_replay(…))`, the replay plan
  authority `30Rb:reingestion-and-why` requires deleted.
- **Why the tree is RED:** `durable_destination` now reads `--receipts`, but that value still feeds
  the OLD `whylog_store` write path rather than the receipt edge; `dorc-loom/src/consumer.rs` still
  names `answers_from_the_receipt_store`, `recorded_selection` and `whylog_dir`; and
  `main.rs::read_receipt_store` does not yet pass edges to `StoreReading::of`. Next commit.
- **Not started:** the `--receipts` store seat (`30Rh:fnd-receipts-flag-has-no-store-seat`), the
  address-directed recorded render (section B), the gate-8 rewrite, the seventh case (section C),
  the rest of D5's census, `results::replayed_records`' removal.
- **Reading complete:** root `README`/`DESIGN`/`IMPLEMENTATION`, `spike/CLAUDE.md`,
  `plan`/`cli`/`aid` crate steering, quarantine `AGENTS.for-builders-only.md`, `30Re`, `30R`,
  `30Ra:receipt-rooted-attention-and-selection`, `30Rb` (reingestion-and-why, Stage 5, Stage 5A,
  Stage 6, DST/product routes, verification tooling, stop conditions),
  `30Rd` (controller-root-resolution, store-enumeration-and-last-selection, D4, D5,
  v1-acceptance-and-exit, test-and-fixture-fences, stop conditions).
- **Tests/gate:** none run yet — the tree does not compile.
