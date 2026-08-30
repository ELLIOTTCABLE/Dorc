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

## the settled store root

The explicit store root lives on `RootInputs`, beside the two platform bases, reached by
`with_store_root` and read back by `explicit_store`. It is a ROOT — controller-supplied, resolved
once — so it belongs where the other roots are rather than threaded past every seat that would
then have to remember it. That siting is also what makes `30Rd`'s "never changes the standard
configuration/key root" structural: the keyset reads `RootRole::Configuration` through `base`, and
no spelling lets a store root reach it.

`store::locations` answers an ORDERED list of Dorc-owned components, outermost first, store root
last: two for the standard selection (product root, then `receipts-v1` beneath it), and exactly ONE
for an explicit folder — itself. Nothing above an admin's folder is Dorc's to validate or create,
and nothing is appended beneath it. Both opens walk that one list, so neither can validate a
component the other does not; `open_or_create` ensures outermost-first (a component is created only
inside a validated parent) and syncs innermost-first (the entry that makes a directory reachable
lives in its parent). `store_root` answers the explicit folder where there is one, so the first-use
gate probes the store the run will actually use.

Resolution to an absolute controller path happens at exactly one seat,
`main::absolute_controller_path`, called only from `production_receipt_edge` — the one place
entitled to consult the process's working directory. It is LEXICAL, not `canonicalize`:
canonicalizing demands the directory already exist, which would make a first run refuse the folder
it was about to create. A relative or empty folder is refused at `with_store_root` under the state
role, because the edge is supposed to have resolved it.

Publication remains gated solely by `--no-receipt`. Naming or omitting a store moves WHERE, never
WHETHER.

## the byte domain

Spans index the ACQUIRED bytes, unnormalized. LF indexes physical lines and a CR in CRLF is an
input byte like any other, so a newline conversion is source DRIFT rather than an invisible
equivalence. Nothing normalizes, transcodes, or reserializes on the way in.

## the recorded-facts boundary

`dorc_receipt::report` is the sealed report-only model a receipt-rooted `why` question produces.
Homed in the receipt crate because everything it reads is receipt-owned — sealed `Reingested`
values, the durable locator, the recorded source table, the graph. Putting it in `cli` or `aid`
would site the decomposition of recorded material behind a crate that also knows how to render,
and the seal would then be one refactor from leaking.

**The API.** `RecordedWhyFacts` with `root`/`closure`/`address`/`sites`/`sources`/`omissions`/
`rederivation`, plus `addressed_site()`. Built by `report::derive(&WhyFactsInput)`. Every field is
private; every state is a closed enum rather than a bool or a string.

**Independence is the design.** Authentication (`AuthenticationState`), closure completeness
(`ClosureCompleteness`), material presence (`MaterialState`), source comparison
(`CurrentSourceState`) and re-derivation (`ReDerivationState`) are five separate answers and stay
that way. A document can be authenticated and incomplete, or complete and unopenable; folding any
pair would let a reader infer a fact from a value that never stated it. `MaterialState::of` takes
the region's standing as a SECOND input for exactly this reason — a slot the skeleton calls
`captured` inside a region that did not open is `Undecodable`, not `Held`, and not `WithheldPlain`
either, because a projection decision and a failure are different things to tell somebody.

**The byte exit.** `RecordedValue` carries a `ValueClass` and has no `as_bytes`, no `as_str`, no
`into_inner`. Its one exit is `render(&mut dyn ValueEncoder)` — the caller supplies the encoder, and
`aid`'s destination encoders satisfy it from the CLI side, so the dependency points the safe way.
It implements no `Display`, `PartialEq`, `Ord`, `Hash` or serde. Equality is the subtle one and the
reason is worth keeping: a caller-driven comparison against probe bytes leaks the value a byte at a
time, so the one comparison the model needs happens INSIDE the crate on the private field and
answers a typed `ByteAgreement`. `Debug` is hand-written to print class and length, never content —
a derived one would put host bytes into panic messages and test output, which are exactly the
places `sinv-sink-encoding` says they may not arrive unencoded.

**Address resolution.** Recorded physical line N against current physical line N of the SAME
source, byte for byte. Identical admits the recorded site; differing is `ChangedLine` (ambiguity,
recorded site retained as a true statement about the past, address-specific answer refused);
uncomparable is `ComparisonUnavailable` with the standing that made it so; and four
`UnresolvedReason` arms cover the absences. There is no content-similarity search, no nearest
match, no fuzzy window — each would answer confidently about a line the author moved, which is
`271:rul-sin-ordering`'s worst rung. A site belongs to a line when its authored span STARTS inside
it: start rather than overlap, because a multi-line construct is addressed at the line a reader
typed, and an overlap test would answer the same site for every line it covers. LF indexes lines
and a CR in CRLF stays in the line's bytes, so a newline conversion reads as a changed line.

**`Reingested::record_kinds`** is new and load-bearing. The model needs the record ORDINAL a detail
entry is keyed by, and the alternative — counting which record species the projection emits first —
makes every consumer a second copy of the projection's ordering. When those copies disagree, every
enrichment lands on whichever row shares its integer and the document still validates cleanly. So
the ordinal is read off the document's own record stream, which is the stream the entries were
keyed against. Kinds only: no atoms, no payload.

**Re-derivation** is `PendingKernelSupport` and never an absence: a reader must be able to tell
"checked, and they agree" from "nobody checked". `ReDerivedDisposition` stays unpopulated.

**The CLI seat** is `dorc_cli::recorded_facts::facts_for`, taking a `SelectedRoot` the edge decoded
plus `ObservedSource` readings it took. The edge owns every filesystem act and hands outcomes over
as data; the digest comparison stays the caller's, which keeps a hash implementation off the pure
path. Current user output is deliberately unchanged — the listing surface has not moved, and
joining these facts to a rendered surface is the next conductor's work.

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

- **`30Rh:open-report-api-close-residue`** — recorded for the following secure builder, per the
  brief's instruction not to run a whole-crate export audit in this bite. `report` exports a
  curated surface, but three things want an API-close decision: the site identity is a bare
  `(u32, Option<u32>)` tuple rather than a newtype (substitutable with any other pair of integers);
  `RootFacts::order` is a `String` rather than a re-parsed `ReceiptOrderToken`; and
  `ClosureFacts::of` takes `reached` as a caller-supplied `Vec` rather than deriving it, so a caller
  could name a document the graph never reached. None is a leak — all three are report-plane values
  — but each is a place a later consumer could be handed something the model did not establish.

- **`30Rh:fnd-the-boundary-fences-catch-a-new-consumer`** — the `Reingested`-consumer roster and the
  rehydration-floor roster in `receipt/tests/crate_boundary.rs` both fired on this work, which is
  the governed review event they exist to create. `cli/src/recorded_facts.rs` joins the consumer
  roster as a genuine new consumer. The rehydration hit was different in kind and is worth the
  distinction: `report.rs` named `MostInfluenced` in a DOC COMMENT explaining the conservative-grade
  rule, not in code reading it. That was fixed by rewording the comment rather than widening a
  security roster, because a roster widened to admit prose stops meaning anything.

- **`30Rh:fnd-the-old-whylog-followed-the-receipt-flag`** — found while seating the store. The
  vocabulary cutover left `durable_destination` feeding the OLD whylog writer from `--receipts`, so
  naming a store would have dropped `whylog-NNNN.txt` files into the receipt store, where the
  bounded walk counts them as unknown entries against its own budget. `durable_destination` now
  answers `None` unconditionally: `--whylog-dir` was the lane's only destination surface and it is
  gone, so the lane is inert until D5 deletes it. Two durables sharing one directory is not a
  smaller change than none.

- **`30Rh:fnd-publication-was-gated-on-a-named-store`** — introduced and fixed inside this lane,
  recorded because the shape recurs: `durable_destination` fed BOTH the old whylog write path and
  the `DurableOutput` gate, so re-pointing it at `--receipts` silently turned default-on
  publication off for every run that named no store. Publication is now gated on `--no-receipt`,
  the admin's refusal, which is the only thing that should ever decide it.

## remaining work

- **`30Rh:fnd-store-route-lists-it-does-not-explain`** — a receipt-reading `why` routes to
  `engine::report_recorded_store`, which emits a recorded LISTING and takes no why address. The
  `=== OUTCOME ===` needle every gate-8 case asserts is the live why triptych's panel header. The
  MODEL that answers it now exists (`report::RecordedWhyFacts`, reachable from the real reading path
  through `recorded_facts::facts_for`) and reaches it without the live decision kernel or
  `results::replayed_records`. What is left is the CLI presentation that joins it to aid/weft —
  the next non-quarantined conductor's, and it never enters receipt internals.
- **`30Rh:owed-help-rows-for-the-new-vocabulary`** — the vocabulary cutover minted five help slugs
  (`cli-help-option-receipts` · `-no-receipt` · `-receipt-last` · `-receipt-id` · `-receipt`) and
  orphaned the four they replace; `arrangement_lock.rs` still carries the old rows and none of the
  new. So `cli-help-page` renders `[unwritten:]` and its fixpoint is red. Seeding unwritten `None`
  rows is builder-legal (`aid/CLAUDE.md arrangement-lock-is-generated-too`), but closing it needs a
  `dorc-loom publish` + rebuild + e2e bless cycle, and the corpus churns wholesale at item 6 — so
  it is NAMED here for the conductor to sequence rather than published twice. This is the one
  surface this lane broke that items 4–6 do not otherwise visit.

- Item 5's authentication posture: do not encode an early-stop/no-explanation invariant. The model
  already keeps `AuthenticationState` independent of completeness and material presence, so a
  failed signature narrows what may be CALLED authenticated without emptying the report.
- Item 6: the six gate-8 pairs onto the recorded renderer, the seventh case's disposition, and the
  rest of the D5 census — old whylog implementation/flags/fixtures/codes/consumers gone,
  `results::replayed_records` caller count zero, one production provider/store/reader/writer.
- The old whylog write lane is inert but still present; D5 deletes it outright.
