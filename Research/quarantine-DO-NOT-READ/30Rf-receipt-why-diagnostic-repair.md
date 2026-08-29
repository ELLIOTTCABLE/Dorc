# 30Rf — receipt-store `why` diagnostic repair

> Tier: quarantined builder ledger for `30Re:sched-repair-why-diagnostics`. Current state, not
> chronology. Rewrite stale claims in place.

## lane

| | |
|---|---|
| Worktree | `.claude/worktrees/r30-receipt-why-repair` |
| Branch | `ai/r30-receipt-why-repair` |
| Base | `ai/r30-receipt` @ `1cbee21a` |
| Current tip | `1cbee21a` (ledger only) |
| Dirt | none |

## the measured seat

`dorc why` over the receipt store is `main.rs::why_from_receipt_store`, reached from
`run_analysis` BEFORE `dorc_cli::engine::run` is ever called
(`main.rs:193`, gated by `answers_from_the_receipt_store`). It performs the whole route itself:
resolve roots, `open_for_read`, `enumerate`, per-entry read into `recorded::*_listing`, graph
build, cohort selection, and stdout emission.

Its two decision conditions are emitted as bare stdout text:

| condition | current emission | source |
|---|---|---|
| edge unopenable / unwalkable / empty | `store-unreadable <word>\n` | `main.rs:599` `unreadable()`; words = `EdgeRefusal::token()`, `walk-failed`, `no-receipt` |
| several documents at the store's greatest order | `ambiguous-order <n>\n` | `main.rs:583` |

`30Re:fnd-why-selection-is-under-tested` is confirmed: `cli/tests/durable_route.rs:228` asserts
only the `store-unreadable ` PREFIX, and nothing anywhere drives the ambiguity seat.

### the codes existed and were deleted

`durable-receipt-unreadable` and `durable-receipt-ambiguous` were minted at `40977599` with
payloads, `CodeSpec` rows and `params_of_raw` arms, and DELETED at `1e128fbe` — the same commit
that added `durable-receipt-unwritten` WITH a defining case. Mechanically that is what a code with
no defining case costs: `catalog_defining_cases::DEFINING_CASE_RATCHET` is EMPTY and shrink-only,
so a new code MUST have a case at `crates/aid/tests/<slug>.loom`. Nothing about the two codes was
wrong; the route they fired from is undrivable by `dorc-loom`, so no honest case could exist.

### why the route is undrivable today

`dorc_loom::consumer::run_engine` returns `None` when `args.reads_the_receipt()`
(`consumer.rs:1008`). It could not do otherwise: the seat is in `main.rs`, on the far side of the
`lib-target-is-a-loom-seam` boundary, so `engine::run` never sees the mode at all.

## the repair shape

Move the store READ to an injected `EngineEdges` edge and the SELECTION/DECISION to the lib, which
is the shape every other non-hermetic act in this pipeline already has (`observe`,
`publish_receipt`, `source_match`).

- new `EngineEdges::read_receipt_store(&mut self) -> Result<StoreReading, String>`; the `Err` is
  the edge's own closed refusal word, exactly as `publish_receipt`'s is.
- `StoreReading` is a pure lib value: per-document `(receipt_id, listing)` in store order, the
  receipt identities sharing the store's greatest order (production's own
  `maximum_order_cohort()`), and the correlated graph listing.
- production `ProductionEdges` implements it over `LocalReceiptEdgeV1` + `NativeIo` — the same
  code that is in `main.rs` today, moved, not rewritten.
- the lib owns: `--receipt` / `--all` / cohort selection, the ambiguity decision, the empty-listing
  decision, and the two diagnostic mints through the ordinary `report_at` route.

This is faithful because it changes no store policy, no grammar, and no reason word: the words the
diagnostics carry are the edge tokens the listing already prints.

## OPEN — the ambiguity defining case (blocking half the remit)

`durable-receipt-unreadable` gets an honest in-process loom case for free: a loom world has no
per-user profile (its own `receipt_label` is already `<no state root>`), so the loom edge refusing
with the roots-unavailable word is a TRUE statement about its world, not an injected fault. The
real-binary battery covers a different word (`store-not-initialized`) over a real empty profile.

`durable-receipt-ambiguous` has no such route, and the reason is structural:

1. a defining case must be an in-process loom case — `code` is `run_lane: false` in
   `dorc_loom::FRONTMATTER_KEYS`, so a whole-product `run:` case is refused if it declares one;
2. the condition needs a store holding two documents at one order, which the loom cannot have:
   its drive is hermetic and clockless, key generation is nondeterministic, and
   `BoundedReceiptEntries` has no constructor outside `LocalReceiptStoreV1::enumerate`;
3. minting such a constructor would be a fixture intake into a production store type — the exact
   shape `rul-fixture-identity-never-production` and `sinv-production-fences` fence, and inside the
   brief's store-policy exclusion.

Candidate routes, for conductor/human ruling:

- **`30Rf:opt-loom-virtual-store-walk`** — `dorc-loom` implements `dorc_receipt_local::io::LocalIo`
  (already a public trait; `dorc-cli` takes `&mut dyn LocalIo`), answers the directory walk from a
  new ordinary case section listing store FILENAMES, and calls production's own
  `LocalReceiptStoreV1::open_for_read` + `enumerate` + `maximum_order_cohort`. No new production
  constructor, no fixture selectable from production, and `30Rd:testing-is-part-of-the-security-boundary`
  layer 3 already sanctions a virtual I/O model — but sited in `dorc-loom` rather than
  `receipt-local`. Cost: the case's documents are unreadable (no bytes, no keys), so its transcript
  honestly shows the ambiguity line AND the no-readable-content code together.
- **`30Rf:opt-real-binary-only`** — land the seat move, `durable-receipt-unreadable`, and real-binary
  ambiguity coverage in `durable_route.rs` (two `dorc plan` runs under a pinned
  `DORC_FIXTURE_CLOCK_MS` share one order, so the ambiguity is genuine); leave the ambiguity as a
  listing line, ledgered as owed. Fails the brief's "restore two typed codes".
- **`30Rf:opt-stop-the-lane`** — the brief's own stop clause.

## test state

Nothing changed yet.

## commits

- ledger only, so far.

## deviations and unresolved `tc-*`

- none yet beyond the OPEN above.

## candidate steering invariants (reported, never written by this lane)

- `30Rd:deterministic-I/O-model` says the local I/O trait "is not exported from the crate". As
  built it IS: `dorc_receipt_local::io::LocalIo` is public because `dorc-cli` passes
  `&mut dyn LocalIo` into `LocalReceiptEdgeV1::open_for_*`. Whatever is ruled above, that sentence
  and the tree disagree today.

## remaining pre-D5 work, explicitly outside this lane

- `30Re:sched-migrate-replay-parity`: gate 8's six replay arms onto the receipt store, and the
  seventh case `whygallery-drifted-book-degraded-receipt.loom`.
- `30Re:sched-delete-legacy-durable` (D5), `30Re:sched-adjudicate-dispatch-diagnostics`.
