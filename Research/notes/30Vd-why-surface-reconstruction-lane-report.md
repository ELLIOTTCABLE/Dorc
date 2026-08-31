# 30Vd — the why-surface reconstruction lane: builder ledger

> Tier: builder ledger for the `30V` §5 close-out lane, carried across three builders. Current state
> and open items only; git carries the chronology. The lane is COMPLETE: the reconstruction plane,
> the exhaustive report projection, the total surface, its `--json` register, the production wiring,
> and the whole-product proof all stand. Nothing quarantined is reproduced.

## what-landed

On `ai/r30-why-surface`, based `ai/main` `996b1519`:

- **`dorc-why`** (`spike/crates/why`) — the `30V` §3 model as real types. Deps `dorc-receipt` and
  `dorc-aid` only; no `weft`, no `dorc-core`, no kernel crate, and it never names `Reingested`.
- **the report projection, now EXHAUSTIVE** — `dorc_receipt::report` projects all fifteen persisted
  plan families. No family answers `FamilyCoverage::RecordedButUnprojected` any more; that word now
  means a projection somebody removed.
- **the total surface** — `dorc_cli::why_total::why_total`, a pure function of
  `(&Reconstruction, &RenderCtx, &mut dyn ValueEncoder) -> (RenderParts, Coverage)`.
- **the `--json` register** — `dorc_cli::why_json::why_json`, the same reconstruction serialized,
  reached by a real `--json` flag.
- **the production route** — `dorc why` is ROOTED at one receipt and renders that surface. The four
  recorded LISTINGS are deleted; `--receipt <file>` answers; an address is placed or refused in the
  answer.
- **the source-comparison seat** — `dorc_receipt::report`'s one visit, and `cli/source_comparison`'s
  one consumer. It closes `file.sh:N`.
- **the orphan-arrangement instrument** — `mise run prose:orphans`, report-only.
- **the gates** — the `why` crate's reconstruction cases over REAL published documents; the receipt
  crate's fifteen report cases; the cli's `recorded_facts_route` and `durable_route` batteries over
  the SHIPPED BINARY; and one whole-product e2e case driving publish → why.

`mise run both gate:full-quiet` is green on both legs at `8cca8edf`; `mise run test` is
3165 passed / 0 failed on the Windows leg.

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
a recorded closed vocabulary (TWELVE of them now — the three apply words joined with the shallow
non-plan fill), and `Payload::Flag(RecordedFlag)` carries one NAMED predicate with its answer — a
bare `bool` would have been substitutable with any other bool, so a tripped certifier and an
invalidating site would have read alike. `IdentityFact::Count` widened to `u64` (a region's route
tally and an admission's record tally are `u64` on the wire) and gained `Operands`. `Subject` gained
`Region` and `Load`: a region is one authored edit many executions share
(`30L:rul-two-identities-never-conflated`), so keying it by a leaf was never an option.

The wiring added two arms, both about the QUESTION rather than about any document:

- **`Subject::Question`** — the subject a fact about the request is hung on. Hanging one off the root
  document instead would read as something the document said.
- **`Payload::Unplaceable(UnplaceableAddress)`** — the address the question named, which the edge
  could not place: not `<file>:<line>`, a current file it could not read, or a file whose bytes match
  no recorded source. The refusal is a ROW of the answer, not a replacement for it
  (`30R`: one unanswerable address is not a reason to stop explaining the rest).

`VoiceSet::Committee` is MINTED now, and only there: a recorded `vouched-severally` custody says
several authors each vouched and names none, so its speaker is an INSEPARABLE committee with no
named members. Spelling that as one unnamed voice would understate the fork a remedy has to take
(`30V` §2 rul-remedies-may-fork). The committee's render carries its separability and its named
members and deliberately no COUNT: a bare `0` beside the word would read as a committee of nobody.

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

A NON-PLAN root now carries its own shallow facts beside the identity and standing it always had:
an intent's `policy` · `origin_state` · `assignment_count` · every originating plan it names; an
outcome's `terminal` · `site_count` · the intent it answers. All seven come from accessors
`Reingested<RecordedApplyIntent>` / `<RecordedApplyOutcome>` already expose, so no receipt internals
were opened. Every plan-shaped family still answers `CarrierGap` there — honestly, and for a
different reason than on a plan root: the report model does not cover the species at all.

Genuine NON-family holes:

- **`30Vd:fnd-addresses-cannot-be-spelled-file-line`** — CLOSED by the source-comparison packet. Both
  halves of `30V` §2's minimum address now arrive at the one seat: the recorded PATH through
  `visit_for_comparison`, the LINE by counting the seat's offset map to the span's start.
  Ordinal-and-span survives as the fallback where a source's content is not held.
- **`30Vd:fnd-nee-state-is-not-derivable`** — STANDS. `30V` §3's `né <oldline>` moved-line state
  cannot be minted: the correspondence rule refuses moved-line matching outright, so nothing could
  produce it. `SourceAgreement` carries three honest states and no fourth.
- **`30Vd:fnd-current-source-is-user-named-only`** — CLOSED, and the closure inverted it: an
  authenticated receipt's own paths are read without the user naming anything. A user-named file
  stays the route for imported or self-asserted material, which is the packet's asymmetry rather
  than a workaround for a missing exit.

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
  `ValueClass` exhaustively by NAME, and they are the only exits a recorded byte has.
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
- **`30Vd:res-addresses-echo-the-recorded-path`** — an address now renders the path the RUN recorded,
  which on an ordinary invocation is absolute. That is the honest namespace (it is where the run
  found the file) and it is why no committed transcript may carry one: the e2e receipt gate asserts
  over live output and commits nothing (`282` §7 refuses a machine-specific path in a transcript).
  Rendering it relative to anything would be a second correspondence rule at a render seat.

## the-production-route-as-built

`dorc why` is ROOTED at one receipt: `main.rs::read_rooted_receipt` (the edge) → one
`recorded::StoreAnswer` → `engine::report_recorded_store` (the render).

- **The four listing seats are GONE**, and so is the whole-store graph string. There is no
  coexistence and no legacy copy. `30R:receipt-rooted-attention-and-cli` licenses no whole-store
  explanation mode, so the store walk survives only as bounded DISCOVERY of typed reverse edges —
  it fills the graph that resolves a root's closure, siblings and correlations, and no document
  outside the rooted closure reaches the answer. Pinned by the identity, explicit-store and
  ambiguity cases in `durable_route.rs`.
- **`StoreAnswer` is three-way** and none of the arms is interchangeable: `Rooted` (a root was
  selected), `Ambiguous(n)` (the greatest order names a COHORT), `Unreadable(word)` (the edge's own
  closed refusal). A cohort now explains NOTHING — reported by
  `DiagCode::DurableReceiptAmbiguous` and never tie-broken, where the listing used to print every
  member. That narrowing is the rooted-surface law, not a scope cut: a surface rooted at one
  document cannot be rooted at two.
- **`ReceiptRoot::File` answers.** `--receipt <file>` opens the KEYSET alone
  (`LocalReceiptEdgeV1::open_documents_for_read`), reads the named path under
  `ReceiptLimits::V1.outer_bytes`, and identifies the species by trying each typed read in turn —
  the header's own `species` line refuses the wrong one, so this ASKS the document rather than
  trusting a filename. `--receipts` stays orthogonal: where a store opens, its walk goes into the
  same graph, so a named outcome still reaches the intent it answers.
- **The address is placed by the ONE comparison seat** (below). `named_address` parses
  `<file>:<line>` (splitting at the LAST colon, so a Windows drive letter survives) and reads the
  named file under `source_content_bytes`; everything after that is the seat's. Three refusals, each
  its own word, each rendered as a datum: `NotAFileAndLine`, `CurrentSourceUnreadable`,
  `NoRecordedSourceMatches`.
- **`--json` is parsed** and refused where nothing would emit it: outside `dorc why`
  (`cli-flag-requires-mode`) and beside `--results`, which selects the live route
  (`cli-flags-mutually-exclusive`). `--all` on this route is a labelled synonym for the default,
  byte for byte.
- **`cli/src/recorded.rs` still names `Reingested`** — the shallow non-plan fill takes the sealed
  wrapper by reference — so entry 2 of `MAY_NAME_THE_READ_BACK_WRAPPER` stays true and no entry was
  added or removed. `why_total.rs` and `why_json.rs` name it nowhere.

### the-source-comparison-seat

Landed on an opaque packet delivered mid-lane, which resolved the `file.sh:N` question and
authorized the one receipt-crate addition it names. The split it rules:

- **`dorc-receipt` releases, and compares nothing.** `RecordedWhyFacts::source_material()` answers a
  `RecordedSourceMaterial`, whose `visit_for_comparison(&mut dyn SourceComparisonConsumer)` hands one
  `SourceComparison` per recorded source: exact recorded PATH bytes, identity/digest/length, exact
  recorded content where held, the authored PLACEMENTS naming it, the document's authentication
  state, and a `line_starts` OFFSET map. Source-specific, never a generic raw-detail accessor; every
  byte-carrying member is a sealed `RecordedValue` whose one exit is a `ValueEncoder`, and nothing
  gains `Display`, a revealing `Debug`, or serde. `SourceFacts::path_text` is crate-private and the
  visit is its only exit.
- **`cli/src/source_comparison.rs` is the seat, and the only one.** It owns platform-aware path
  rehydration, the bounded non-following regular-file read, exact current-vs-recorded path and
  content comparison, the correspondence rule, and the line map's use. Its
  `FilesystemBytes` encoder is a SECOND destination, not a second display seat: a path about to be
  opened and a path about to be shown are different sinks, and it refuses every class but a path and
  every non-UTF-8 spelling.
- **The authentication asymmetry.** For `AuthenticationState::Trusted` material the seat rehydrates
  the recorded path and reads it UNASKED, so `dorc why` now compares a run's own sources with no
  file named. For anything else it opens nothing a receipt named; a user-named file is still
  compared, because that path was typed rather than supplied by somebody's document.
- **Correspondence: exact path, then exact content.** The path rule is `30R`'s own (the same physical
  path in the current and recorded book); the content rule identifies the same file under a
  different spelling. Both exact. There is no nearest-match and no moved-file search.
- **`file.sh:N` LANDED.** `LocusAddress` gains `line: Known<u32>`, counted over the seat's line map
  to the span's start, and the recorded path travels beside the population in
  `dorc_why::ComparedSources` (a sealed value has no equality, and an address stays `Eq`). Both
  surfaces spell `<path>:<line>` where the seat supplied both halves and fall back to
  ordinal-and-span where it did not — a `dorc-lang` source holds no content, so it has no lines to
  count and the address says so rather than inventing one.
- **The enumeration hygiene the packet asked for** is `crate_boundary.rs`'s
  `the_source_comparison_seat_is_the_only_one`: a two-way lexical roster over implementors and call
  sites, because Rust cannot say "one implementation" about a public trait. Minted at explicit human
  direction rather than by reflex, which is the whole of what
  `lexical-fences-are-human-ack-instruments` asks.

### the e2e proof

One whole-product case, `crates/cli/tests/why30-receipt-rooted-surface.loom`, declares
`expect-why-receipt:` — a new run-lane frontmatter key, mapped to `expected-why-receipt` by
`materialize_loom`, and the twenty-fourth row of `dorc_loom::FRONTMATTER_KEYS`.

The RUNNER EXTENSION is one gate, `e2e.rs::scan_why_receipt`. It creates a throwaway per-user profile
(its three role bases, since the write path creates a store beneath a base and never the base), drives
the case's own publish into it, and then asks that store six questions. The case declares only
NEEDLES; every cross-invocation law rides the declaration rather than being restated per case, because
a law a case can forget to assert is a law nothing holds: `--all` is byte-identical to the default,
`--json` parses and carries BOTH slot spellings, `--receipt <file>` answers identically to the store's
own derivation of the same document, an address over the case's own book places, and one over a file
the document never saw renders `address-unplaceable`. A dedicated discovery floor in `main` refuses a
corpus where no case declares the key.

The gate owns its profile deliberately: the suite profile is shared, so `--receipt-last` there would
answer about whichever case published most recently. Nothing it drives is committed — a receipt
identity is OS entropy and a transcript carrying one could never be a fixpoint, which is why the proof
is needle-shaped rather than golden-shaped.

### the orphan-arrangement census

`30Rk:the-arrangement-mirror-is-its-own-lock` — BUILT, as `mise run prose:orphans`
(`internal-tooling`'s `arrangement-census`), after the human sharpened the lexical ruling: the rule
targets grep-gates standing in for TYPECHECKING, and an arrangement slug is declared in inherently
textual material — a case's frontmatter, or a literal at the seat that renders it — so scanning text
is what asking the question looks like.

An INSTRUMENT, never a gate, on `doctor-inventories-never-reaps`'s footing and for a sharper reason:
a red gate would make "delete the row" the shortest agent path, and removing a user-facing string is
a conductor-at-least decision. It prints, sorted, and stops.

What it finds today — five rows nobody declares, every one a render that went out with the retired
durable, all awaiting a conductor's word rather than a builder's delete:

`syntax-unsupported-source-of-dynamic-target` · `why-drift-address-unanswerable` ·
`why-drift-analysis-suppressed` · `why-receipt-plan-tally-unsplit` · `why-receipt-when-replayed`

## golden-churn, as inspected

Two committed files moved, and no `expected.out` in the corpus did:

- `crates/aid/tests/cli-help-page.loom` — ONE added line, `--json   [unwritten:
  cli-help-option-json]`, from the new help row. Refreshed through `DORC_LOOM_DUMP` (a render change,
  not a prose edit, so `dorc-loom publish` correctly refuses it) and its arrangement row hand-seeded
  `words: None`; the generator fixpoint gate accepts the seed unchanged.
- `crates/cli/tests/why30-receipt-rooted-surface.loom` — the new case's own transcript, minted by a
  filter-scoped `mise run bless:case -- why30-receipt-rooted-surface`. It is the ordinary
  probe+apply round-trip render for its book; nothing receipt-shaped is in it.

No other golden byte moved. The listing replacement churned no `dorc why` golden because no e2e case
ever drove the receipt route — the corpus's only `dorc why` drive is gate-8's LIVE `--results` one.

## tc-flags

- **`tc-nonplan-root-depth`** — the SHALLOW half is built (`ShallowIntent` / `ShallowOutcome`, from
  accessors the read-back wrapper already exposes). The DEEP half stands open and the reason is a
  fence rather than a budget: `RecordedWhyFacts` is plan-TYPED, so per-assignment rows and per-site
  outcome rows need either a second sealed facts model or a widening of the existing one, and both
  are more than read-surface projection. Conductor call, unchanged.
- **`tc-machine-format-flag-spelling`** — `--json` as briefed and now PARSED, against the
  `lint --format=jsonl` precedent. Ruled fold-time taste; recorded so it is not re-litigated by
  accident.
- **`tc-licensor-custody-speaks-its-own-act`** — a licensor datum speaks in the act its recorded
  CUSTODY names (`vouched`/`vouched-severally` → `Vouched`, `measured-self` → `Measured`) rather than
  in the engine's own `Derived` voice, and `vouched-severally` now populates an INSEPARABLE committee
  whose members are unnamed. Ground: `30V` §2 rul-first-person-register puts the tool's "I" only
  where no more-correct register exists, and a recorded custody is one. Flagged because it is the one
  place this lane chose a speaker rather than reading one. NOT EXERCISED by any corpus case: no
  fixture reaches a `vouched-severally` licensor, so the committee arm is built and unwitnessed
  end-to-end.
- **`tc-address-refusal-is-a-datum-not-a-diagnostic`** — an address the edge cannot place renders as
  a ROW of the answer (`Subject::Question` + `Payload::Unplaceable`) rather than as a typed stderr
  diagnostic. Ground: `30R` says an unanswerable address still renders every unrelated receipt fact,
  and the alternative costs a new `DiagCode` whose defining case is not deterministically driveable —
  a loom world publishes no receipt (its clock is absent, and the store refuses an undated document),
  so the "no recorded source matches" arm cannot be provoked in-process. The counter-argument is real
  and unadjudicated: `30V` §2 rul-intent-disambiguation-fails-fast wants the tool to STOP AND ASK on
  a question it cannot resolve, and a row in the middle of a total surface is not stopping. Conductor
  call.
- **`tc-file-root-order-comes-from-the-name`** — a receipt named by `--receipt <file>` takes its store
  ORDER from the filename, and a file renamed out of the store's grammar is refused
  (`receipt-file-unnamed`) rather than dated UNDATED. The document carries its own order and the read
  surface has no exit for it; adding one is a receipt-crate accessor, which is past the boundary this
  lane was fenced to. Conductor call: widen the read surface, or keep the refusal.
- **`tc-correspondence-falls-back-to-content`** — the seat matches a user-named file by exact
  recorded PATH first and exact recorded CONTENT second. The path rule is `30R`'s own; the content
  rule is a correspondence POLICY this seat now owns, and it is what lets a user who is standing in
  a different directory name the same file. Both are exact and neither guesses about movement, but
  the second is a choice rather than a ruling, and the packet puts future correspondence policy at
  this seat — so it is flagged rather than assumed.

## still-owed, named

- **`dorc-loom`'s usage text** still offers `whylog-unwritten` as its example case slug, which names
  no case any more. It is user-facing prose, so a builder may not rewrite it — conductor's.
- **The authentication ASYMMETRY is unwitnessed end-to-end.** The production edge answers
  `AuthenticationState::Trusted` for every read it can make — the local-authentication envelope is
  what a successful read produces — so `SelfAsserted` and `Failed` material cannot be manufactured
  from outside `dorc-receipt` and the no-implicit-read half of the rule has no case driving it. The
  rule is implemented and its two branches are one `match` apart; what is missing is a world that
  reaches the second. Closing it needs an imported-receipt route, which nothing has yet.
- **The non-following read is unwitnessed too**, for the same reason: arranging a symlink AT a
  recorded path needs a run whose book was one.
- **The seat's stat-then-open window** is disclosed at its own module header
  (`churn-avoidance-disclosure`): a path replaced between the two syscalls is read as whatever
  replaced it. Bounded — a wrong comparison answer, never an action — and closing it wants an
  open-then-fstat handle this edge does not have.
- **`crates/cli/CLAUDE.md` and `crates/why/CLAUDE.md` have no rule for the comparison seat yet.** The
  invariants worth siting there: the one-seat split, the two-destination encoder rule, and the
  authentication asymmetry. Conductor-managed files, so this ledger holds them until then.
