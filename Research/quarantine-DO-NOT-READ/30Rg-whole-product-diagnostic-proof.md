# 30Rg — whole-product diagnostic proof

> Tier: quarantined builder ledger for `30Re:sched-enable-whole-product-code-proof`. Current
> state, not chronology. Rewrite stale claims in place.

## lane

| | |
|---|---|
| Worktree | `.claude/worktrees/r30-receipt-loom-code` |
| Branch | `ai/r30-receipt-loom-code` |
| Base | `3734290f` (carries `30Rf`'s landed work and its one red) |
| Inherited red | `dorc-aid::diag_tidy::every_variant_has_exactly_one_catalog_entry` — CLOSED |

## the three refusals, measured before any repair

A whole-product case at `crates/aid/tests/durable-receipt-ambiguous.loom` declaring
`code:` + `run: round-trip` + `fixpoint: executed` was refused in exactly three places.
Measured at `f41756be`, not predicted:

1. **e2e runner** — `unread frontmatter key 'code'`: `code` was `run_lane: false` in
   `dorc_loom::FRONTMATTER_KEYS`, and so were `when-fires` and `why`, which a `code:` case
   carries.
2. **looms runner** — `hygiene: case: replay block 0 does not surface "durable-receipt-ambiguous"`.
   That runner already DEFERRED the transcript for `fixpoint: executed`, but still demanded the
   in-process drive render the code — which is the one thing a whole-product case cannot do.
3. **catalog** — the inherited red: no row, because `dorc-loom publish` mints rows from `code:`
   cases and this one could not load.

## the capability, as built

**`code:` on a whole-product case is a declaration AND an assertion.** The key that mints the
catalog row is the key the e2e runner checks fired, so the owner and the proof have one source.
That is the mechanical, non-forgeable link: not a filename match, not a comment, not a second
declaration a author could forget to keep in step. A case declaring a code its own run does not
emit is red; a case declaring a dead slug is refused against the generated catalog, exactly as
`expected-diagnostics` already was.

| piece | seat |
|---|---|
| vocabulary | `dorc_loom::FRONTMATTER_KEYS` — `code`/`when-fires`/`why` are `run_lane: true`; `arrangement` deliberately is NOT (a chrome page has no production drive to prove one) |
| looms deferral | `looms.rs::deferred_to_e2e` — a `run:` case's hygiene keeps the marker-collision half and drops the slug-surfacing half |
| the proof | `e2e.rs::defined_code_fired` — validates the slug against the generated catalog, then requires it on the stderr of one of the case's own drives, at any severity |
| stderr reaches it | `run_replay_block` now CAPTURES stderr (it was `Stdio::null()`), and `run_round_trip` hands block 0's back through a parameter; `ExtraReplays` carries blocks 1..N's |
| the case's own world | `Harness::dorc` resolves `<case dir>/.dorc-own-profile` when materialization laid one down, else the shared harness profile |
| publish | `dorc-loom.rs` treats a `run:` case as executed-elsewhere: no edit baseline, no bytes comparison, metadata only |
| inventory | `DorcConsumer::editable_baseline` declines a `run:` case with `dorc_loom::EXECUTED_ELSEWHERE` |

### why the case needs its own profile

`Harness::profile` is ONE `ProfileSandbox` for the whole run, and every plan drive publishes a
receipt into it. A `dorc why --last` there reads every other case's documents: the cohort would be
ambiguous for reasons outside the case, the listing would not reproduce, and the store would
accumulate toward `LocalLimits::V1.store_entries` (4096) across a suite. A case minted to
DEMONSTRATE a capability must OBSERVE it (`30Nf:fnd-multipart-never-placed-anything-in-production`),
so a `code:`-declaring case gets the throwaway profile its own materialization carries. Every other
case keeps the shared one, unchanged.

### the fixture requirement nobody would guess

A run publishes a receipt only when its intake ADMITTED records, and
`plan/src/records.rs` answers `NoObservation` for a framed stream with zero records. In e2e the
record set is derived from the REAL rendered probe, so a book of unmodelled commands can only ever
produce an empty stream — the case's first two shapes published nothing and the store read back
`durable-receipt-unreadable`. The case therefore carries a one-command oracle and one measured
site. That is the whole reason for the fixture, and it is stated in the case's own book.

## the case

`crates/aid/tests/durable-receipt-ambiguous.loom` — in `aid/tests` because
`catalog_defining_cases::is_case_owned` resolves `CARGO_MANIFEST_DIR/tests/<slug>.loom`
(`aid/CLAUDE.md cases-live-here`), and the e2e runner's `discover_looms` walks every
`crates/*/tests`, so siting it there costs nothing.

Three drives, sequential in one materialized dir under one pinned clock: two plans publish two
receipts at one store order, then `dorc why --last` finds a greatest order it cannot name. Its
stdout is discarded (`> /dev/null`) because receipt identities are entropy-derived and no
transcript could hold them; the proof is the code on stderr, and the transcript authority stays
with e2e (`fixpoint: executed`). Catalog row published prose-empty: `message: None`, rendering
`[unwritten: durable-receipt-ambiguous]`.

## verification

- **The failing direction, measured.** With the ambiguity report replaced by `if false`, the case
  reddens naming exactly `durable-receipt-ambiguous`; restored, it passes. Both runs had every
  drive exit 0, which is the second half of the same measurement: a nonzero process status is
  neither necessary for the gate to fire nor sufficient to satisfy it — the gate's only input is
  the slug on stderr.
- **Both runners over both receipt cases**: `mise run test:looms -- durable-receipt` and
  `mise run test:e2e -- durable-receipt-ambiguous`, green.
- **Full suite**: `mise run test` — 3128 run, 3128 passed, 2 skipped.
- Completion gate: see the report; one `mise run both gate:full-quiet` at the final tip.

## collateral, found by the suite and repaired

- `catalog_defining_cases::unwritten_renders_are_greppable_and_pinned` — the unwritten-prose
  ceiling went 21 → 24, the conscious bump the gate asks for, naming the receipt durable's three.
- `dorc-loom::editable_surface::vars_answers_for_every_committed_case` — asserted an inventory for
  EVERY corpus case; a whole-product case has no in-process render to compile an edit against. It
  now asserts the DECLINE by its exact reason rather than skipping, so the day this starts
  answering is not a silent one.

## deviations — OPEN for conductor adjudication

- **`30Rg:dev-whole-product-prose-has-no-path`** — a whole-product `code:` case can hold its
  metadata row but cannot yet gain PROSE: `generate::case_example` fills a written template from
  `consumer.case_diag(case)`, which drives the case in-process and cannot produce a diagnostic the
  real binary emitted. Prose-empty publishes fine (the union row short-circuits on `message: None`),
  so nothing is blocked today and no builder may author prose anyway
  (`27V:rul-error-authorship-tier`). The prose sprint meets this the first time it reaches such a
  code, and the honest repair is a path that reads the EXECUTED transcript. `EXECUTED_ELSEWHERE`
  says so where an author will meet it.
- **`30Rg:dev-own-profile-is-a-marker-directory`** — the per-case profile is selected by a
  directory the materialization lays down, read at the one seat that builds every drive, rather
  than threaded through four call sites. It is idiomatic for this harness (`DORC_FLAGS`,
  `ARTIFACT_SET` are marker files) and contained to one `if`, but it IS a filesystem-carried
  signal rather than a typed one. Flagged rather than resolved.
- **`30Rg:dev-round-trip-hands-stderr-back-by-parameter`** — `run_round_trip` exits at a dozen
  gate verdicts, so its captured stderr is handed back through an out-parameter rather than the
  return type. The alternative was a `Result<String, Failed>` and an edit at every verdict; the
  parameter also carries the right semantics (the code-fired question is about the DRIVE, not the
  verdict, so a case whose code fired and whose golden then diverged still says so).

## no refused route was taken

No `LocalIo` implementation in `dorc-loom`; no blessing of its public boundary; no `StoreReading`
built from case data; no virtual storage, fixture provider, static production key, diagnostic
injection, new `--this defect`, or slug-matched production dispatch; no receipt/key/store/replay
limit widened; no completeness, ownership, transcript, or discovery gate weakened; no prose
authored; no steering file edited. Gate 8, the old whylog surfaces, the seven legacy-dependent
cases, `results::replayed_records`, D5 and `ApplyPlanNotDispatchable` are untouched.

## candidate steering invariants (reported, never written by this lane)

- `cli/CLAUDE.md` (harness contract): a whole-product case may DEFINE a code, and its `code:` is
  both the catalog key and the assertion that one of the case's own drives emitted it. The proof
  spans every drive the case makes, because which drive provokes a diagnostic is the case's
  business; a case that defines a code runs in its own throwaway profile, because an assertion
  about a suite-wide durable is not an assertion about the case.
- `aid/CLAUDE.md` (`cases-live-here` rider): the canonical case for a registered slug stays in
  `aid/tests` even when it is a whole-product case — the e2e runner walks every collection, and
  `is_case_owned` is manifest-local, so siting decides ownership and the runner decides execution.
- The e2e harness's shared profile is a suite-wide MUTABLE durable. Nothing but this lane's case
  reads it back today; anything that does needs its own.

## folding back into `30Rf`

`30Rf` closes red on exactly one failure and names `30Rg` as its closer. To fold:

1. `30Rf`'s **OPEN — `durable-receipt-ambiguous` has no defining case** section is discharged: the
   case exists at `crates/aid/tests/durable-receipt-ambiguous.loom` and the catalog row is
   published. Its three enumerated causes stay accurate as HISTORY of why the code was dropped at
   `1e128fbe`; rewrite the section to say the capability landed and where.
2. `30Rf`'s routes list: `opt-whole-product-code-proof` is the one taken; `opt-loom-virtual-store-walk`
   and `opt-hand-seed-then-own` are closed unbuilt and can be dropped rather than carried.
3. `30Rf`'s **test state** ("ONE DELIBERATE RED", no discovery-floor counts by ruling) is stale:
   this lane's tip is fully green and carries the both-platform gate.
4. `30Rf`'s coverage table gains the ambiguity defining case beside its real-binary row.
5. `30Rf:dev-ambiguity-case-not-delivered` is discharged. Its other two deviations
   (`dev-orphan-deleted`, `dev-extra-deduplication`) and its broader finding
   (`fnd-debug-spellings-on-the-listing`) are untouched by this lane and still stand.

## remaining pre-D5 work, outside this lane

- `30Re:sched-migrate-replay-parity` (`30Rh`) — gate 8's six replay arms onto the receipt store,
  plus `whygallery-drifted-book-degraded-receipt.loom`'s disposition.
- `30Re:sched-delete-legacy-durable` (D5) and `30Re:sched-adjudicate-dispatch-diagnostics`.
