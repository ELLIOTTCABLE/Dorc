# 30Vd — the why-surface reconstruction lane: builder ledger

> Tier: builder ledger for the `30V` §5 close-out lane. Current state and open items only; git
> carries the chronology. The lane landed the RECONSTRUCTION plane and the report widening that
> feeds it; the presentation half (total surface, `--json`, cli wiring, e2e cases) is specified
> here and unbuilt. Nothing quarantined is reproduced.

## what-landed

Three commits on `ai/r30-why-surface`, based `f6317f43`:

- **`dorc-why`** (`spike/crates/why`) — the `30V` §3 model as real types. Deps `dorc-receipt` and
  `dorc-aid` only; no `weft`, no `dorc-core`, no kernel crate, and it never names `Reingested`.
- **the report widening** — `dorc_receipt::report` grew `InvocationFacts`, `NarrativeFacts`,
  `PlanFamily` and `FamilyCoverage`, plus the sealed-row accessors those need. Read-surface
  projection only: no grammar, writer, wire, projection-state or provider change, and no raw model
  or overlay accessor is exposed.
- **the gates** — eleven cases in `crates/why/tests/`, driven through REAL published documents
  (projected, sealed, signed, read back) under inert injected capabilities.

`mise run both gate:full-quiet` is green on both legs at `b07cbb8a`.

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

## durable-gap-audit

`Reconstruction::audit()` is DERIVED from the population, never assembled beside it, and each hole
carries the cause that says whose it is. Post-widening, over a plan root:

| family | coverage | cause / why |
|---|---|---|
| invocation | PROJECTED | mode, attempt, argv + target availability, influence |
| sources | PROJECTED | pre-existing |
| sites | PROJECTED | pre-existing |
| narratives | PROJECTED | the recorded speech acts — the speaker axis rests on it |
| omissions | PROJECTED | pre-existing |
| admission | report-api-lacks | recorded; unprojected |
| presented-plan | report-api-lacks | recorded; unprojected |
| regions | report-api-lacks | recorded; unprojected |
| loads | report-api-lacks | recorded; unprojected |
| classifications | report-api-lacks | recorded; unprojected |
| certifications | report-api-lacks | recorded; unprojected |
| ships | report-api-lacks | recorded; unprojected |
| survivals | report-api-lacks | recorded; unprojected |
| renders | report-api-lacks | recorded; unprojected |
| licensors | report-api-lacks | recorded; unprojected |

**Every remaining hole is the report API's, and none is the carrier's.** That is the lane's central
finding: closing them is projection work in `report::families`, needing no durable change at all.
Ten families remain; each is the same mechanical shape as the two that landed.

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
  the user named, with a typed refusal when no recorded source matches.

## label-vocabulary-proposal

Words are the conductor's to mint; every row seeds `words: None` and renders `[unwritten: <slug>]`.
Hand-seeded rows must be spelled in the serializer's field order (`slug · occurrence · when_used ·
why · words`).

**Reuse, do not mint:** `why-tier-word` (occurrences 0–6) already carries the seven `SpeechAct`
words. The total surface takes its verbs from there — `trust-tier-is-syntax` satisfied by reuse.

Proposed new rows, all `when-used` = "the receipt-rooted `dorc why` total surface renders this
label", `why` = "`30V` §5: the total surface's stable label vocabulary; structure and labels are its
only readability affordances":

- sections: `why-total-section-root` · `-closure` · `-invocation` · `-sources` · `-sites` ·
  `-narratives` · `-loci` · `-omissions` · `-address` · `-gaps`
- field labels (the five axes): `why-total-label-speaker` · `-world-moment` · `-world-host` ·
  `-world-lineage` · `-subject` · `-payload` · `-delivery`
- absence vocabulary: `why-total-absent-run-held-no-value` · `-absent-projection-uncollected` ·
  `-absent-report-api-lacks` · `-withheld-plain` · `-withheld-bound` · `-withheld-region` ·
  `-withheld-encoder` · `-cant-tell-no-comparison` · `-cant-tell-truncated` · `-not-yet-piped` ·
  `-unknowable`
- state words: `why-total-state-authentication` · `-projection` · `-detail` · `-closure` ·
  `-current-source` · `-rederivation`
- coverage words: `why-total-coverage-projected` · `-recorded-but-unprojected` · `-not-carried` ·
  `-not-relevant`

Sentence shape for each: a bare noun-phrase label; values interleave as the seat's own computed
runs. `--json` keys stay hardcoded and out of the registry.

## what-remains

Specified, unbuilt, in dependency order:

1. **Ten more family projections** in `report::families`, retiring their absences. Mechanical; the
   two landed families are the template.
2. **The total surface** — `cli::why_total`, a pure function of `(&Reconstruction, &RenderCtx,
   &mut dyn ValueEncoder)` returning `(RenderParts, Coverage)` where `Coverage` is appended AT THE
   EMIT SITE, so a datum an early return drops is caught by a permutation check rather than by a
   second walk. Exclusion ledger constructed empty.
3. **`--json`** — mechanical serialization; withholds as explicit typed markers, never absent keys;
   the destination encoder composes `aid::display::encode_foreign` with `dorc_lint::json::escape_into`
   so no new encoding rule is invented.
4. **cli wiring** — `read_receipt_store` → root selection → `facts_for` → `reconstruct` → render.
   `facts_for` still has NO production caller. Delete all four listing seats
   (`recorded_{plan,intent,outcome,graph}_listing`). Fix `ReceiptRoot::File`, which today answers
   nothing at all. Drop the whole-store graph listing (`30R` licenses no whole-store mode).
5. **e2e cases** — publish→why as CASES in the existing corpus, extending the runner minimally
   where the case grammar cannot express two invocations.
6. **The orphan-arrangement census** (`30Rk:the-arrangement-mirror-is-its-own-lock`) — untouched.

## proposed-steering

Offered for the conductor to site; not written into steering by this lane.

- **`the-report-surface-is-exhaustive-or-classified`** (`crates/receipt/CLAUDE.md`) — every family
  the recorded model persists is either a typed facts collection on `RecordedWhyFacts` or carries an
  explicit `FamilyCoverage`. The classification is closed and no-wildcard, so a new family cannot
  land unclassified. `RecordedButUnprojected` and `NotCarried` are DIFFERENT facts and must never
  merge: one is repaired by projection work, the other by nothing.
- **`absence-carries-its-cause`** (`crates/why/CLAUDE.md`, if the crate gets one) — every absence in
  the report plane names whose it is. An audit that cannot separate a carrier hole from a
  read-surface hole sends readers to widen a durable that needs no widening.
- **`a-voice-set-is-its-own-leaf`** — a speech ACT and the VOICES performing it are separately
  knowable; a document can record the act and name nobody. Folding them makes an unnamed speaker
  look like an unknown act.

## tc-flags

- **`tc-nonplan-root-depth`** — `Rooted::OtherSpecies` is built and its facts are thin because no
  sealed model covers intent/outcome roots. Whether their families get the same projection treatment
  is a conductor call; reaching them must not reopen receipt internals.
- **`tc-machine-format-flag-spelling`** — `--json` as briefed, against the `lint --format=jsonl`
  precedent. Ruled fold-time taste; recorded so it is not re-litigated by accident.
