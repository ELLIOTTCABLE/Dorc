# 30Rh — integrated durable close: builder ledger

> Tier: quarantined builder ledger for `30Re:sched-integrated-durable-close`, continued under the
> source-custody redirect. Rationale, findings, deviations, and handoff only — no tip hashes, no
> per-test status, no commit lists (git carries those).

## the settled source-class boundary

A source is `dorc-lang` iff `dorc_oracle::marker::has_marker` accepts it — the `# dorc-lang/v0.2`
version marker, and nothing else. Everything else the run acquired is `general-sh`, and its exact
acquired bytes ride the rich overlay under `OpaqueFieldTag::SourceContent`.

Deliberately NOT `sourcing::satisfies_the_contract`, which additionally demands load-inertness.
That is the contract a `.` OPERAND must meet — a different question from which dialect a file was
accepted as — and folding the two would withhold the bytes of a marked file that merely runs
something at load time. The marker alone is also what `snapshot::role_of` already splits
`BookSourced` from `PlainInclusion` on, so there is one answer to "which dialect is this file"
rather than two.

Custody follows the class and nothing else: `general-sh` captures, `dorc-lang` reads
`uncollected`, plain narrowing turns either captured slot into `withheld-plain`, and a source over
either bound reads `omitted-limit` while allocating nothing. A source the caller described nothing
about defaults to `dorc-lang`-shaped, so an absent entry persists nothing — there is no branch that
could turn a path into a read.

## the durable locator shape

`dorc-receipt-locator/1`, a receipt-owned inert mirror in `receipt::durable_locator` — not serde
over `aid::locator::Locator`, because that type holds process-local `SourceFileId`s and `StageId`s
and deserializing it would hand a reader back a LIVE locator. There is no conversion back, no
accessor yielding a `SourceFileId`, and nothing in the module reaches a `ProvId`, an arena, or an
authority input.

It preserves the closed five-stage vocabulary (`authored`/`loaded`/`copied`/`generated`/`claimed`),
source ordinals or generated-artifact identity, exact byte spans, ordered bounded origins, and one
head. Wire form is ASCII structural lines with LENGTH-PREFIXED raw runs, so an artifact label or a
bundle claim carrying any byte — a newline, invalid UTF-8, a control sequence — round-trips exactly
without an escape alphabet to get wrong. Validation is at the CONSTRUCTOR, so an invalid locator is
unrepresentable rather than merely unrendered; an origin must be STRICTLY EARLIER than the stage
citing it, which makes acyclicity structural rather than a walk.

The slot is per-SITE and never per-region: a region is one authored edit many executions share, so
it has no single provenance to carry, and giving it the slot would invite one instance's locator to
stand for every other invocation of the same body.

## the byte domain

Spans index the ACQUIRED bytes, unnormalized. LF indexes physical lines and a CR in CRLF is an
input byte like any other, so a newline conversion is source DRIFT rather than an invisible
equivalence. Nothing normalizes, transcodes, or reserializes on the way in.

## deviations and open items

- **`30Rh:dev-four-crates-have-no-steering`** — the first brief directed a full read of crate
  steering for `dorc-loom`, `receipt`, `receipt-crypto`, and `receipt-local`; none of the four
  carries a `CLAUDE.md`. Read what exists; authored no steering (the brief forbids it).

- **`30Rh:open-suppression-spelling`** — RULED, human, ACK: `--no-whylog` renames in place to
  `--no-receipt` and stays. `AID-NEEDS:law-whylog-is-sensitive` requires a typeable refusal and
  `30Rd:v1-acceptance-and-exit` #16 keeps writing default-on, so deleting it outright would remove
  the only way to say no.

- **`30Rh:open-seventh-case`** — `whygallery-drifted-book-degraded-receipt.loom` embeds an old
  `.whylog` and rides the drifted-replay path. That path (`AcquiredEngine::Drifted`,
  `dorc_cli::drifted_receipt`) went out with the replay ladder, so the case is currently unbacked
  and its disposition is load-bearing rather than tidy-up. Owed at item 6.

- **`30Rh:fnd-receipts-flag-has-no-store-seat`** — `--receipts <folder>` parses and gates nothing
  yet. `receipt-local`'s `store::store_root` and its private `locations` both derive the root as
  `roots.product_root(RootRole::State).child(STORE_DIR)`, so the flag cannot name an exact store.
  Honouring the ruled spelling needs an explicit-root override on `LocalReceiptEdgeV1` threaded
  into both store opens, leaving the KEY root standard. In scope, NOT yet built — and until it is,
  the flag is accepted and ignored, which is the one live dishonesty in the surface.

- **`30Rh:fnd-recorded-sites-carry-no-line`** — a `RecordedSiteDecision` carries `RecordedSite`,
  a `RecordedAst` ARENA INDEX, disposition, shell text, and influence. No line number, by design:
  the durable locator's authored span plus the recorded exact bytes are what recover a historical
  physical line. Recording a line instead would be a durable-content change behind
  `rul-durable-contents-reviewed-before-design`, and is not this lane's to take.

- **`30Rh:dev-site-locators-are-book-authored-only`** — `custody::site_locators` builds a
  one-stage `Authored` locator per site from the book's own AST span. A site whose bytes arrived
  through a `.` would compose a `Loaded` stage above it, and the representation carries that shape
  already; what is missing is the per-site SOURCE identity to build it from, which lives in the
  loader rather than in `SpineDisposition`. Sites the book's arena cannot answer are ABSENT from
  the map rather than carrying a guessed span — the projection reads absence as uncollected, and an
  uncollected locator is a slot a reader knows to distrust.

- **`30Rh:fnd-publication-was-gated-on-a-named-store`** — introduced and fixed inside this lane,
  recorded because the shape recurs: `durable_destination` fed BOTH the old whylog write path and
  the `DurableOutput` gate, so re-pointing it at `--receipts` silently turned default-on
  publication off for every run that named no store. Publication is now gated on `--no-receipt`,
  the admin's refusal, which is the only thing that should ever decide it.

## remaining work

- **`30Rh:fnd-store-route-lists-it-does-not-explain`** — a receipt-reading `why` routes to
  `engine::report_recorded_store`, which emits a recorded LISTING and takes no why address. The
  `=== OUTCOME ===` needle every gate-8 case asserts is the live why triptych's panel header. So
  the address-directed recorded renderer (item 4) is the substance of what is left, and it must
  reach it without the live decision kernel or `results::replayed_records`.
- Item 5's authentication posture: do not encode an early-stop/no-explanation invariant.
- Item 6: the six gate-8 pairs onto the recorded renderer, the seventh case's disposition, and the
  rest of the D5 census — old whylog implementation/flags/fixtures/codes/consumers gone,
  `results::replayed_records` caller count zero, one production provider/store/reader/writer.
- `--receipts` still needs its store seat before any of that is honest.
