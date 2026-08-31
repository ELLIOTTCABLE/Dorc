# 30Vd — the why-surface reconstruction lane: builder ledger

> Tier: builder ledger for the `30V` §5 close-out lane, continued across two builders. Current state
> and open items only; git carries the chronology. The lane has landed the RECONSTRUCTION plane, the
> EXHAUSTIVE report projection, the TOTAL SURFACE and its `--json` sibling. What remains is the cli
> WIRING and the e2e cases that ride it — specified below, unbuilt. Nothing quarantined is reproduced.

## what-landed

On `ai/r30-why-surface`, based `ai/main` `996b1519`:

- **`dorc-why`** (`spike/crates/why`) — the `30V` §3 model as real types. Deps `dorc-receipt` and
  `dorc-aid` only; no `weft`, no `dorc-core`, no kernel crate, and it never names `Reingested`.
- **the report projection, now EXHAUSTIVE** — `dorc_receipt::report` projects all fifteen persisted
  plan families. No family answers `FamilyCoverage::RecordedButUnprojected` any more; that word now
  means a projection somebody removed.
- **the total surface** — `dorc_cli::why_total::why_total`, a pure function of
  `(&Reconstruction, &RenderCtx, &mut dyn ValueEncoder) -> (RenderParts, Coverage)`.
- **the `--json` sibling** — `dorc_cli::why_json::why_json`, the same reconstruction serialized.
- **the gates** — the `why` crate's twelve reconstruction cases over REAL published documents; the
  receipt crate's fifteen report cases; three new cli cases driving documents the SHIPPED BINARY
  published through reconstruct → render → serialize.

`mise run both gate:full-quiet` is green on both legs at `624c4e95`; `mise run test` is
3153 passed / 0 failed on the Windows leg.

## the-model-as-built

Two nested wrappers, each answering a different question, separated by construction:

- `Known<T> = Knowable(Held<T>) | KnowableNYI | Unknowable` — is this answerable at all.
- `Held<T> = Present | AbsentFromCarrier(CarrierAbsence) | Withheld(WithholdReason) |
  CouldNotTell(CantTell)` — the answer, which may be an affirmative not-knowing.

Laundering NYI into Unknowable is prevented structurally rather than lexically
(`lexical-fences-are-human-ack-instruments`): no `From`, no `Into`, no method on either yielding
the other, separate constructors, and `Unknowable`'s reason is consumed and dropped at the seat so
it can never reach a render. `KnowableNYI` is argument-free and a gate asserts no slot of a real
reconstruction carries one.

`Datum` holds the five `30V` §3 fields privately behind one mint taking all five by value. Leaf
granularity is real rather than nominal: `WorldCoordinate` is a product of three independently
wrapped leaves, and `Speaker` carries its act and its voice-set as separate answers — which is what
lets a recorded narrative say *a vouch was spoken* while honestly saying *nobody is named*.

Two payload kinds joined at the family widening: `Payload::Token(RecordedToken)` carries one word of
a recorded closed vocabulary (nine of them), and `Payload::Flag(RecordedFlag)` carries one NAMED
predicate with its answer — a bare `bool` would have been substitutable with any other bool, so a
tripped certifier and an invalidating site would have read alike. `IdentityFact::Count` widened to
`u64` (a region's route tally and an admission's record tally are `u64` on the wire) and gained
`Operands`. `Subject` gained `Region` and `Load`: a region is one authored edit many executions
share (`30L:rul-two-identities-never-conflated`), so keying it by a leaf was never an option.

## durable-gap-audit

`Reconstruction::audit()` is DERIVED from the population, never assembled beside it, and each hole
carries the cause that says whose it is. Over a plan root:

| family | coverage | cause / why |
|---|---|---|
| invocation | PROJECTED | mode, attempt, argv + target availability, influence |
| sources | PROJECTED | pre-existing |
| sites | PROJECTED | pre-existing |
| narratives | PROJECTED | the recorded speech acts — the speaker axis rests on it |
| omissions | PROJECTED | pre-existing |
| admission | PROJECTED · `NotCarried` when the run wrote no admission row | outcome, records, bytes, stream text, influence |
| presented-plan | PROJECTED · `NotCarried` when the run wrote no row | three approval identities, influence |
| regions | PROJECTED | region ordinal, ast, disposition, routes, shell text, influence |
| loads | PROJECTED | ordinal, outcome, name, custody, influence |
| classifications | PROJECTED | site, ast, class, verdict-lane, invalidator, cells, influence |
| certifications | PROJECTED | pass, consistent, tripped, influence |
| ships | PROJECTED | site, lane, defining source text, influence |
| survivals | PROJECTED | site, outcome, wall, aggregate, poison, influence |
| renders | PROJECTED | subject (leaf/region/none), kind, detail, influence |
| licensors | PROJECTED | site, verb, custody, locus, influence |

**No hole is the report API's any more.** The two OPTIONAL singletons answer `NotCarried` when the
document holds no such row, which is a CARRIER fact repaired by nothing — deliberately not
`Projected(0)`, which would read as a projection that found an empty intake.

`FamilyCoverage::NotRelevant` is now the one arm nothing mints. It is reachable in principle from a
non-plan root, whose coverage answer is not on `RecordedWhyFacts` at all; leaving it unminted is a
disclosed residue rather than a promise (see `tc-nonplan-root-depth`).

Genuine NON-family holes, which no projection closes:

- **`30Vd:fnd-addresses-cannot-be-spelled-file-line`** — `30V` §2's minimum address is `file.sh:N`
  and NEITHER half is derivable. A source's PATH has no exit (the raw-detail accessor is
  crate-private by design, and the encoder exit is display-lossy and capped); the byte-offset-to-line
  map is crate-private too. `LocusAddress` therefore carries ordinal-and-span, which is what is
  true. Closing this is a report-API question about a path-class projection, not a durable one.
- **`30Vd:fnd-nee-state-is-not-derivable`** — `30V` §3's `né <oldline>` moved-line state cannot be
  minted: the address rule refuses moved-line matching outright, so nothing could produce it.
  `SourceAgreement` carries three honest states and no fourth.
- **`30Vd:fnd-current-source-is-user-named-only`** — with no path exit, the edge cannot open a
  recorded source to compare it. The production route must resolve the ordinal by DIGEST over a file
  the user named, with a typed refusal when no recorded source matches. STILL OWED (item 1 below).

## label-vocabulary-as-minted

Thirty-seven rows hand-seeded into `crates/aid/src/arrangement_lock.rs`, every one `words: None`,
rendering `[unwritten: <slug>]` until a conductor mints words. They are a generator fixpoint (the
lock gate is green), spelled in the serializer's field order (`slug · occurrence · when_used · why ·
words`), and appended at the end of the registry so the mirror-union carries them through unchanged.

**Reused, not minted:** `why-tier-word` (occurrences 0–6) carries the seven `SpeechAct` words, and
the total surface reaches it through `dorc_cli::why::verb_said`, the ONE seat that renders a
`SpeechAct` (`AID-NEEDS:law-trust-tier-is-syntax`). It was made `pub(crate)` for that; a second
mapping would have been a second vocabulary.

What was minted, by group:

- sections (4): `why-total-section-carriers` · `-data` · `-correlations` · `-loci`
- carrier labels (6): `why-total-label-document` · `-species` · `-role` ·
  `why-total-state-authentication` · `-projection` · `-detail`
- datum labels (7): `why-total-label-subject` · `-speaker` · `-payload` · `-world-moment` ·
  `-world-host` · `-world-lineage` · `-delivery`
- structure labels (3): `why-total-label-correlation` · `-locus` · `-locus-edge`
- absence vocabulary (11): `why-total-absent-run-held-no-value` · `-absent-projection-uncollected` ·
  `-absent-report-api-lacks` · `-withheld-plain` · `-withheld-bound` · `-withheld-region` ·
  `-withheld-encoder` · `-cant-tell-no-comparison` · `-cant-tell-truncated` · `-not-yet-piped` ·
  `-unknowable`
- voice words (3): `why-total-voice-mine` · `-authored-in` · `-committee`
- role words (3): `why-total-role-root` · `-reached` · `-sibling`

Departures from the earlier proposal, and why: the per-FAMILY section list is gone because the
reconstruction is a FLAT datum population and the render follows it rather than re-grouping (a
grouped render would be a selection); the `why-total-coverage-*` words are unminted because there is
no separate audit SECTION — an audited hole is already a datum in the data section, and rendering it
twice would break the exactly-once claim; and `why-total-state-{closure,current-source,rederivation}`
are unminted because those are `StateFact` PAYLOADS (values), not labels.

## the-total-surface-as-built

`why_total(&Reconstruction, &RenderCtx, &mut dyn ValueEncoder) -> (RenderParts, Coverage)`.

- **Coverage is appended AT THE EMIT SITE**, inside the same `map` that produces a datum's node, so a
  datum an early return dropped is missing from the ledger rather than merely uncounted. The gate is
  a PERMUTATION over `0..data.len()`, never a count.
- **The exclusion ledger is empty BY CONSTRUCTION**: `ExclusionReason` is an UNINHABITED enum, so
  `30V` §2's "the absent-curation tier is literally the empty exclusion set" is a fact about the type.
  The first arm belongs to whichever curated surface first decides to exclude something.
- **It weaves.** Nodes are `weft::Section` + `LabeledRow` (+ attachments), rendered through
  `dorc_cli::why_parts`. That satisfies `sinv-sink-encoding`'s bypass shape directly: every value is
  a `Said::Value`, and `weave::value` encodes at mint. Registry words are ours and are never encoded.
- **Two destination encoders, one shape.** `why_total::TerminalValues` (the display seat, capped) and
  `why_json::JsonValues` (the display seat, then `dorc_lint::json::escape_into`). Both match
  `ValueClass` exhaustively by NAME. `cli/src/recorded.rs`'s private terminal encoder was DELETED and
  its listing now uses `TerminalValues` — one seat, not two.
- **Register, as it actually renders**: `=== [unwritten: why-total-section-data] ===` headers, one
  labelled block per datum with the other four axes hanging under the subject, columns squared up by
  weft's named table, printable ASCII. It is verbose and unpretty ON PURPOSE (`30V` §5: intentionally
  temporary, replaced without ceremony).

### disclosed shapes of this cut

- **`30Vd:res-world-coordinate-repeats-per-datum`** — the moment/host/lineage triple is per-datum by
  `30V` §3's ruling ("never a document frame") and the total surface makes no selection, so a
  document-shared coordinate is restated on every datum. De-duplicating it is a CURATION decision and
  belongs to the settled register, not here.
- **`30Vd:res-report-states-spell-through-debug`** — `AuthenticationState`, `ProjectionState`,
  `DetailState`, `ClosureCompleteness`, `CurrentSourceState` and `ReDerivationState` are report-plane
  states with no wire token, and they render through `Debug` (`Trusted`, `Rich`, `Available`). Minting
  `token()` words for them would be a builder authoring user-facing vocabulary
  (`error-authorship-tier`), so the interim spelling is machine-shaped and greppable. Recorded WIRE
  vocabularies render through their own `token()` and are unaffected.
- **`30Vd:res-json-markers-are-a-second-match`** — the JSON absence markers
  (`absent-run-held-no-value`, …) are their own no-wildcard match (`why_total::absence_word`) beside
  the registry-slug one (`absence_slug`). Deriving one from the other would couple a wire word to a
  render slug's spelling; both being no-wildcard is what stops them drifting.

## what-remains

Specified, unbuilt, in dependency order.

### 1. cli wiring

`read_receipt_store` → root selection → `facts_for` → `reconstruct` → `why_total` / `why_json`.
`facts_for` STILL has no production caller: today `main.rs::read_receipt_store` walks the store,
builds a per-document LISTING, and `engine::report_recorded_store` prints whichever listing the
selection takes plus the whole-store graph listing.

The work, precisely:

- DELETE all four listing seats — `recorded_{plan,intent,outcome,graph}_listing` in
  `cli/src/recorded.rs`. No coexistence, no legacy copy. `StoreReading::graph` (the whole-store graph
  string) goes with them: `30R:receipt-rooted-attention-and-cli` licenses NO whole-store mode
  ("There is no whole-store explanation mode"), and the store walk keeps only its role as bounded
  discovery of typed reverse edges. Pin the narrowing with a case.
- KEEP `cli/src/recorded.rs` naming `Reingested` in whatever replaces the listings. It is entry 2 of
  `MAY_NAME_THE_READ_BACK_WRAPPER` in `receipt/tests/crate_boundary.rs`, and that fence is TWO-WAY: a
  stale entry fails. If the module genuinely stops naming the wrapper, the entry must be removed in
  the same commit. `why_total.rs` and `why_json.rs` deliberately name it nowhere, so no new entry is
  owed — keep it that way.
- FIX `ReceiptRoot::File`, which today answers `false` for every store entry and therefore selects
  nothing at all. Per `30R`: `--receipt <file>` roots the question at an explicit file OUTSIDE any
  store; it never publishes and never authorizes; `--receipts` stays orthogonal, giving that root a
  BOUNDED place to resolve graph siblings by typed identity. The read is `ReadEdge::read_plan` /
  `read_intent` / `read_outcome` over bytes the edge read from the named path, species chosen by the
  document's own header rather than by a filename it may not have.
- ADDRESS RESOLUTION: `RequestedAddress` names `(source ordinal, line)`, and the user names a FILE.
  Resolve the ordinal by DIGEST over the file the user named, with a typed refusal when no recorded
  source matches (`30Vd:fnd-current-source-is-user-named-only`). `LocusAddress` stays
  ordinal-and-span; the `file.sh:N` question is with the human — do not build toward it, do not fake it.
- The non-plan root reaches `dorc_why::recorded::Rooted::OtherSpecies` with a `NonPlanRoot` the edge
  fills; `is_modelled(species)` is the discriminator.

Useful API notes gathered while reading, so the next builder does not re-derive them:
`ReadEdge::store().enumerate(io)` answers `BoundedReceiptEntries`; `recognized()` gives
`OwnedReceiptEntry`s carrying `name().receipt_id()`, `species()` and `order()`;
`store.read_into_budget(io, entry, &mut budget)` reads one within the aggregate budget;
`ReceiptGraph::closure_from(&RecordedDocumentId)` is the ONE mint of a `ReachedClosure`;
`SelectedRoot` (in `cli/src/recorded_facts.rs`) is the struct `facts_for` takes, and
`crates/cli/tests/recorded_facts_route.rs` already builds one from a real store walk — that helper is
the shape the production seat wants.

### 2. e2e cases

publish→why proofs as CASES in the existing e2e corpus (`crates/cli/tests/e2e.rs`-driven flat-tree
cases), per the human-typed ruling `30Va:rul-whole-product-proof-rides-the-e2e-corpus`: NO new
standalone `[[test]]` battery, no new test type. If the case grammar cannot express a two-invocation
publish-then-why sequence, EXTEND THE E2E RUNNER / case shape minimally so it can — that extension is
authorized and preferred over any workaround; keep it cross-platform (`task-bodies-are-shell-free`,
`one-shell-answer`). `--all` byte-identity and `--json` well-formedness get pinned here too. Non-empty
discovery floors everywhere.

Note for whoever lands this: wiring the total surface into `dorc why` REPLACES the listing output, so
every committed `dorc why` golden churns. Bless is orchestrator-only and scoped
(`bless-honours-the-trial-filter`); plan for a reviewed, case-by-case diff rather than a sweep.

### 3. the orphan-arrangement census

`30Rk:the-arrangement-mirror-is-its-own-lock` — WITHHELD pending a human ruling on
`lexical-fences-are-human-ack-instruments` grounds. Do not build it.

## tc-flags

- **`tc-nonplan-root-depth`** — RULED that intent/outcome rows get the same projection treatment
  where the `Rooted::{Plan, OtherSpecies}` face needs them. NOT ACTED ON in this lane, and the reason
  is a fence rather than a budget: `RecordedWhyFacts` is plan-TYPED, so a non-plan root's families
  would need either a second sealed facts model or a widening of the existing one, and both are more
  than read-surface projection. What IS cheaply available and still unused: `Reingested<
  RecordedApplyIntent>` already exposes `policy()`, `origin_state()`, `assignment_count()` and
  `origin_receipts()`, and `Reingested<RecordedApplyOutcome>` exposes `terminal()`, `site_count()`
  and `intent()` — all public today, so filling `NonPlanRoot` from them needs no receipt internals at
  all. Deeper than that (per-assignment rows, per-site outcome rows) DOES need new sealed accessors.
  Conductor call.
- **`tc-machine-format-flag-spelling`** — `--json` as briefed, against the `lint --format=jsonl`
  precedent. Ruled fold-time taste; recorded so it is not re-litigated by accident. The flag itself is
  UNPARSED at this cut: `why_json` exists and no argv reaches it until the wiring lands.
- **`tc-licensor-custody-speaks-its-own-act`** — a licensor datum speaks in the act its recorded
  CUSTODY names (`vouched`/`vouched-severally` → `Vouched`, `measured-self` → `Measured`) with its
  voices honestly unnamed, rather than in the engine's own `Derived` voice. Ground:
  `30V` §2 rul-first-person-register puts the tool's "I" only where no more-correct register exists,
  and a recorded custody is one. The map is many-to-one on the severally arm, which is why the
  severally-ness also rides its own payload token rather than being folded into the act. Flagged
  because it is the one place this lane chose a speaker rather than reading one.
