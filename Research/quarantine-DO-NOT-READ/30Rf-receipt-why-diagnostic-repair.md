# 30Rf — receipt-store `why` diagnostic repair

> Tier: quarantined builder ledger for `30Re:sched-repair-why-diagnostics`. Current state, not
> chronology. Rewrite stale claims in place.

## lane

| | |
|---|---|
| Worktree | `.claude/worktrees/r30-receipt-why-repair` |
| Branch | `ai/r30-receipt-why-repair` |
| Base | `ai/r30-receipt` @ `f98f65a7` |
| Current tip | `6c04f632`, plus this ledger commit — ledger-only, no code above it |
| Dirt | none |
| Lane status | **INCOMPLETE**, deliberately, pending `30Rg` |

`ai/r30-receipt-loom-code` @ `1144da1b` carries this lane's first seven commits re-applied on the
pre-merge base; it is the `30Rg` lane and holds no work of its own yet. `30Re` names this lane's
tip as `d948d651`, which is the same content before `ai/r30-receipt` absorbed `ai/main`.

**The lane closes RED, by conductor ruling (2026-08-29).** `durable-receipt-unreadable` is finished;
`durable-receipt-ambiguous` has its code, payload, emit seat and real-binary coverage but no
defining case, so `every_variant_has_exactly_one_catalog_entry` fails. That single failure is
`30Rg`'s to close (`30Re:sched-enable-whole-product-code-proof`), and `30Rg` owns the next full
green `mise run both gate:full-quiet`. Nothing here waits on anything else.

## the measured seat, as built

`dorc why` over the receipt store no longer decides anything in `main.rs`. The route is:

| stage | seat | what it does |
|---|---|---|
| route | `Args::answers_from_the_receipt_store` (`cli/src/lib.rs`) | `reads_the_receipt() && whylog.is_none() && whylog_dir.is_none()`; the free `reads_the_receipt` already carries `Mode::Why`, so the old duplicate mode test is gone |
| read | `main.rs::read_receipt_store` | roots → `open_for_read` → `enumerate` → per-entry bounded read → `StoreReading`; `Err` is the edge's own closed word |
| decide | `engine::report_recorded_store` | selection, the ambiguity report, the empty-listing report, the listing emit |
| loom | `consumer.rs::run_receipt_store_why` | hands the shared seat `Err("no-controller-root")` — the honest answer for a world with no per-user profile |

Both report conditions are typed diagnostics on stderr through the ordinary `report_at` route,
filed under one stage word (`engine::RECEIPT_STAGE = "receipt"`, shared with the write side). The
bare `store-unreadable …` / `ambiguous-order …` stdout tokens are gone.

| condition | code | payload |
|---|---|---|
| edge unopenable / unwalkable / nothing readable | `durable-receipt-unreadable` | `{store}`, `{reason}` |
| several documents at the store's greatest order | `durable-receipt-ambiguous` | `{count}` |

Both registers are `None` and render `[unwritten: <slug>]`. No prose was authored.

### coverage

| seat | proof |
|---|---|
| unreadable, real binary, empty profile | `cli/tests/durable_route.rs::asking_why_creates_nothing_and_says_what_it_found` — asserts the CODE on stderr, and an empty stdout |
| unreadable, in-process | `crates/aid/tests/durable-receipt-unreadable.loom` (rootless loom world) + `dorc-loom/tests/consumer.rs` |
| ambiguity, real binary | `durable_route.rs::two_runs_at_one_recorded_moment_leave_a_last_the_store_cannot_name` — two `dorc plan` runs pinned to one `DORC_FIXTURE_CLOCK_MS`, asserting the code fires, that BOTH cohort members are still listed, and that `--receipt=<id>` does NOT fire it |
| ambiguity, defining case | **owed — see the blocker below** |

The closed reason WORD is not assertable at the binary today: an unwritten register renders the
greppable placeholder and interpolates no parameter. It becomes pinnable the day the code has prose.

## the repair shape, and why it is faithful

The store READ moved to the process edge and the SELECTION/DECISION moved into the lib, which is
the shape every other non-hermetic act on this pipeline already has. `StoreReading` /
`RecordedDocument` are pure lib values, so `cli/CLAUDE.md lib-target-is-a-loom-seam` holds (values
cross, queries do not) and `one-definition-table-two-drivers` gets a second real instance.

Nothing about store policy, receipt grammar, key handling or the walk changed. The words the two
diagnostics carry are the edge tokens the old listing already printed; `RecordedDocument::unread`
keeps "the store holds this identity" and "this identity had something to say" as separate facts,
which is what stops a store of unopenable documents from reporting as empty.

## OPEN — `durable-receipt-ambiguous` has no defining case (the lane's one red)

`dorc-aid::diag_tidy::every_variant_has_exactly_one_catalog_entry` fails, and only that. The chain
that produces it is mechanical:

1. every `DiagCode` variant needs a `CatalogEntry`;
2. `catalog_lock.rs` is generated, and `generate::generate_catalog_lock`'s union loop keys new rows
   off cases loaded BY `code:` — an `owns:`-only claim keeps an existing mirror row alive but mints
   none;
3. `code:` is `run_lane: false` in `dorc_loom::FRONTMATTER_KEYS`, so a `code:` case is an
   in-process loom drive;
4. an in-process drive is hermetic and profile-less, so `run_receipt_store_why` can only honestly
   hand the seat `Err(no-controller-root)`. A two-document cohort is not a thing that world has.

`durable-receipt-unreadable` escapes this because the rootless refusal IS true of a loom world.

Routes, measured:

- **`30Rf:opt-whole-product-code-proof`** — the conductor's own `30Re:rul-whole-product-loom-proves-code`:
  let a `run:` case declare `code:` and let the real e2e execution own the transcript/code proof.
  Measured cost beyond the vocabulary flag: NO existing e2e drive reaches this route at all.
  `scan_why_chain` (gate 8) is the only `dorc why` the runner drives and it passes `--whylog-dir`,
  which routes to the legacy durable by construction. A receipt-store ambiguity drive needs a
  sandboxed profile, two plan runs at one pinned clock, and a `dorc why --last` with no
  `--whylog*` — a new gate, not a flag flip. This is `30Re:sched-enable-whole-product-code-proof`
  in full, and `30Re` assigns it to `30Rg`, not here.
- **`30Rf:opt-loom-virtual-store-walk`** — `dorc-loom` implements `dorc_receipt_local::io::LocalIo`
  and answers the walk from a case section listing store filenames. NACKed by
  `30Re:fnd-whole-product-code-proof-is-missing`; recorded only so the route is not re-derived.
- **`30Rf:opt-hand-seed-then-own`** — hand-seed the catalog row and have a whole-product case claim
  the slug via `owns:`. It IS a generator fixpoint (a mirror row whose slug is `owns:`-claimed and
  has no `code:` case re-reads its carried metadata), but it still needs `opt-whole-product-code-proof`'s
  new drive to make the code actually FIRE, so it saves nothing and spends an orchestrator-tier
  hand-edit carve. Not recommended.

Nothing here is doable inside this brief's remit without taking `30Rg`'s assigned work.

## test state

- `mise run test` (Windows): 3126 run, 3125 pass, 1 fail — the OPEN above, and nothing else.
- `mise run both check-quiet`: clean on BOTH legs. This is what proves the Linux compile of the
  whole workspace, tests included, under `-D warnings` — the `one-platform-green-is-not-cross-platform-green`
  hazard, which matters here because the receipt store carries a Windows-only rename-backup path.
  WSL trust is established for this worktree and for the nested `spike/verify/aeneas` config.
- Both-platform completion gate: **deliberately NOT run** (conductor ruling, 2026-08-29). Spending a
  full `gate:full-quiet` on both legs to reconfirm an inherited, named catalog-completeness red buys
  nothing; the focused suite plus both `check-quiet` legs are the handoff evidence for an
  intermediate branch, and `30Rg` runs the next full green after it supplies the defining case.
  So this ledger reports NO discovery-floor counts, by ruling rather than by omission.

## commits

| | |
|---|---|
| `5fe9f2d7` | ledger opened with the measured seat map |
| `f6d9b4e8` | the two codes, payloads, `CodeSpec` rows, `params_of_raw` arms |
| `4a50e786` | the seat move: `StoreReading`/`RecordedDocument`, `report_recorded_store`, the loom driver arm |
| `7ada380d` | the rootless-world loom case |
| `d943e020` | the `durable-receipt-unreadable` catalog row, prose-empty |
| `4bab0fbb` | the loom consumer test expects an answer, not a decline |
| `f8bccd75` | the real-binary ambiguity test, both directions |
| `3665fa52` | clippy: `unnecessary_wraps` on the loom arm, `doc_lazy_continuation` in the test |
| `8101835c` | one spelling of the rootless store label (`engine::NO_STATE_ROOT`) |
| `9fcdb047` | `durable::entry_by_receipt_id` deleted — the moved selection orphaned it |
| `e53e9a93` | comment pass |
| `6c04f632` | this ledger, rewritten from the stale opener to the as-built map |

`3665fa52` matters as evidence: the lane's first seven commits had never been through
`mise run check-quiet`, and two clippy walls were standing.

## deviations and unresolved `tc-*`

The first is RULED; the other two are OPEN for conductor adjudication. Neither was resolved locally.

- **`30Rf:dev-ambiguity-case-not-delivered`** — brief item 3 asks for honest defining cases for
  BOTH codes; only `durable-receipt-unreadable` has one. RULED (conductor, 2026-08-29): the lane closes
  red and `30Rg` supplies the case. The lane could not close it without taking `30Rg`'s remit
  (measured, above).
- **`30Rf:dev-orphan-deleted`** — `cli/src/durable.rs`'s `entry_by_receipt_id` was DELETED rather
  than ledgered. It had no caller left in the tree: the seat move replaced its exact-identity match
  with `engine::RecordedSelection::Named`, and its ruling ("retrieval, not a second RANKING; the
  store offers exactly one selection, and a second way to PREFER a candidate would reopen the
  fallback that property exists to forbid") survives verbatim on that enum's doc. Being `pub` in a
  `publish = false` crate, it would have drawn no dead-code lint and sat as a second live spelling
  of one selection rule. Flagged because deletion is fractionally wider than the brief's "repair
  only strings produced by the same moved seat"; reversible in one commit if the conductor wants it
  back.
- **`30Rf:dev-extra-deduplication`** — three repairs beyond the brief's literal remit, all inside
  the moved seat's own surface, none changing a decision:
  1. `"<no state root>"` had THREE spellings across two crates (`main.rs` twice, `dorc-loom`'s own
     const). It is simultaneously the read side's `{store}` payload and the write side's
     `receipt_label`, so a divergent spelling would report a store the operator cannot match
     against the one the writer named. Now one `dorc_cli::engine::NO_STATE_ROOT`.
  2. Two standing clippy walls (`unnecessary_wraps` on the loom arm, `doc_lazy_continuation` on a
     test doc-comment). The inherited seven commits had never been through `mise run check-quiet`,
     so the lane could not have committed anything further without clearing them.
  3. A comment pass: one restating doc-line deleted, one test note de-narrated.

## broader findings, NOT repaired here (brief item 5)

- **`30Rf:fnd-debug-spellings-on-the-listing`** — `cli/src/recorded.rs` puts two Rust `Debug`
  renderings on stdout: `model-unavailable {refusal:?}` (three species arms) and
  `partial {:?}`. Every other line in that module is a receipt-grammar field name or a closed
  token, which the module header states as its own law. A `Debug` spelling is neither, and it
  leaks Rust type names onto a surface the module documents as the document's own words. Not
  produced by the moved seat, so not repaired: it is D4 listing content and wants either a closed
  token or a typed report.

## candidate steering invariants (reported, never written by this lane)

- `30Rd:deterministic-I/O-model` says the local I/O trait "is not exported from the crate". As
  built it IS: `dorc_receipt_local::io::LocalIo` is public because `dorc-cli` passes
  `&mut dyn LocalIo` into `LocalReceiptEdgeV1::open_for_*`. That sentence and the tree disagree.
- `cli/CLAUDE.md` has no bullet for the receipt-store why route. Candidate, for the conductor to
  word: the store READ is an edge act and the SELECTION is the lib's, so both drivers report the
  same two conditions through the typed route — the `one-definition-table-two-drivers` shape, now
  with a second instance.
- Candidate rider on `stdout-contract`: a report ABOUT the store is aid on stderr; stdout carries
  only the recorded listing. The ambiguity seat went untested for exactly as long as one test
  helper discarded the other stream.

## remaining pre-D5 work, explicitly outside this lane

- `30Re:sched-enable-whole-product-code-proof` (`30Rg`) — and it now also owns the missing e2e
  drive, not just the vocabulary flag.
- `30Re:sched-migrate-replay-parity` (`30Rh`) — gate 8's six replay arms onto the receipt store,
  plus the seventh case `whygallery-drifted-book-degraded-receipt.loom`, which carries an old
  `.whylog` directly and needs its own disposition.
- `30Re:sched-delete-legacy-durable` (D5) and `30Re:sched-adjudicate-dispatch-diagnostics`.
