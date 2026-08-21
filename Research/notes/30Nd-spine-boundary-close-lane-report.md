# 30Nd — the `309` boundary close: lane report

> Tier: **LLM-authored, builder (Opus-class)**, lane `ai/r30-spine-boundary-close` from
> `ai/r30-conduct@655df12c`. Executes `30M:rec-own-the-309-boundary-close` (§4.3 of the first-half
> crosscheck adjudication) plus `30M:rec-dissolve-trip-must-remember-structurally` (§2), so the
> boundary `30I:step-7-reify-plan-artifact-forms` will freeze is the one the `309` ack described.
> Everything below is as-built and measured; confidence markers where it is not.
>
> Read with: `plans/309` (THE spec) · `notes/30E` (the census, whose §3 audit this closes) ·
> `notes/30F` (the reification lane, whose §4.4 deviation this closes) · `notes/30N` §4 (the human
> queue this lane feeds).

## §1 — What landed

Five commits, `b0da38bf..cef1e6de`.

### `work-trip-witness-by-type` — the cleanup dissolves into a type

`plan::certifier_trip::TripSpent` is a private-field witness with no `Default` and exactly ONE mint,
`spend_certifier_trip(spine, trip, census_unique)`. `project_plan` demands it by reference, so a
producer that never spent its latch has no projection to call.

- The mint is TOTAL over both latch states: an untripped run does no walk and still comes away with
  the proof. That totality is what removes the temptation to mint one beside the walk, and there is
  deliberately no intakeless-style escape (contrast `PlanAuthority::without_intake`) — every
  producer HAS a latch, even a default one.
- `project_censusless` and `dorc_cli::world::demote_on_certifier_trip` are the two seats that spend;
  both now return the witness their callers hand to the projection.
- `every_plan_producer_spends_its_certifier_trip` STAYS, as the brief required, and its `spends`
  needle list gained `spend_certifier_trip(`. Its doc now says what it binds that the type does not:
  a producer that builds a walled plan and hands the Spine somewhere else never reaches the
  projection seat at all.
- `demote_on_trip`'s in-body claim ("does NOT dissolve the must-remember surface … unbuilt") is
  corrected to the new truth.

**The acceptance test.** `a_tripped_plan_cannot_be_projected_while_it_still_elides` lands the
original Sol-adversarial shape from `1dbca1ab` green — a real trip, a real eliding spine, nothing
between them but the projection. The reshaped
`a_censusless_producer_spends_its_trip_before_projecting` also stays; the two prove different
things, and the report says so at the test: the reshaped one proves the ONE tail that exists is
correct, this one proves no other tail can be written. The half no runtime assertion carries is a
`compile_fail` doctest on `TripSpent` — **falsified by hand at authoring time**: threading a real
witness makes the block compile and reddens the assertion, so the refusal is the arity and not some
unrelated error.

### `work-decisions-into-the-plane` — the render is a printer

`plan::DecidedRender` is minted ONCE, by `DecidedRender::decide`, from the settled dispositions +
regions + `(src, ast)`. `Plan::decided` is now the plan's ONLY constructor (`render` is a private
field), and `project_plan` calls it and then `record_render_decisions`, which now TRANSCRIBES rather
than re-deriving. Every render and disclosure surface reads the plane.

`decide` is one function rather than four because the order is load-bearing: pinned bindings are
taken over the guards the render will actually emit, which needs the refusals; the refusals need the
omit neutralisations; and both need to know which regions are still live.

### `work-spine-population-meaning-audit`

§4 carries the table. Two behavioural repairs, six narrowed docs, four flags.

### Riders honoured

`inv-determinism` (the plane is sorted `BTreeSet`s and mint-ordered `Vec`s; the digest is FNV over
the canonical plane, unmoved) · `inv-must-may` and `claim-tier-gating` (no mint signature touched) ·
`pin-no-outcome-as-generator` (the relocation creates NO read-back: `decide` consumes dispositions
and produces render answers; nothing feeds a render answer into a decision, and the Spine records
are written from the same value the render prints rather than being read back by it) ·
`acts-and-dispositions-mint-together` and `one-settlement-one-world` (untouched — `decide` runs
strictly after the settlement sealed) · `rul-host-evidence-is-not-the-narrative-plane` (no intake
type renamed or aliased) · `two-plane-aid-law` (`DecidedRender` is decide-plane; the narratives it
feeds still flow one way) · the region-plane laws (`region-decisions-meet-universally`,
`shared-edit-before-erasure`, `no-specialized-shell` — the meet and the license mint are untouched;
only WHERE the liveness answer is computed moved).

No new plan-producing path was added, so the roster fence is unchanged in membership.

## §2 — The relocation inventory

| decision | from-seat | to-seat |
|---|---|---|
| `dec-pinned-definitions` | `Plan::pinned_definitions(src, ast)`, called inside `render_apply` | `pin_definitions(…)` under `DecidedRender::decide`; `Plan::pinned_definitions()` is now a reader |
| `dec-render-refusal` | `Plan::refused_render_steps(ast)`, recomputed by three disclosure surfaces and again by `collect_edits` | `DecidedRender::refused` — one `Vec<RefusedEdit>` all four read |
| `dec-omit-neutralisation` | `is_neutralised(…)` evaluated inside `collect_edits` and again in `omit_neutralisations(ast)` | `DecidedRender::neutralised` |
| `dec-defensive-emission` | a `pub` `Plan` field the driver poked after construction, read at render time by `pinned_definitions` | an INPUT to `Plan::decided`; the field is gone |
| `dec-certifier-trip-cleanup` | already a Spine write for its RESULT; the ACT was a call to remember | `TripSpent`, above |
| **`dec-region-liveness`** (found; not in the `30E` five) | `Plan::live_regions(ast)`, recomputed by `pinned_definitions`, `rendered_guards` and `collect_edits` | `DecidedRender::live_regions` |

`dec-region-liveness` is a genuine sixth: `30L:pin-whole-helper-derived-only` decides whether a
shared region's authored edit lands AT ALL, which is exactly what `dorc why` must account for when
answering "why was this definition not edited". The census predates `30L`, which is why it is not in
the five.

### Kept render-side, with the reason

Applying the brief's discriminator — could `dorc why` or a second artifact form ever need to account
for it:

- **The commented-original wrapping** (`collect_edits`'s `StandIn::True` ⇒ `# <original bytes>`
  branch, with its `top_level_simple` / `is_alone_on_line` / single-line-span conditions). This
  prints ONE `Replace` decision two ways: same license, same stand-in, same observables, same
  `rul-attention-honesty` posture. A second artifact form would present the `Replace` and its
  `StandIn`; it would have nothing to say about which of two byte-shapes the sh render chose. Pure
  presentation.
- **The hoisted preamble STRING** (`PinnedDefinitions::hoisted`). The DECISION — which body each
  guard invokes and under what name — is in the plane and is what the Spine records; the
  concatenated bytes are that decision typeset for one artifact form. A second form would re-typeset
  from the same bindings. The two travel together today only because splitting them would be churn
  with no consumer; if `30I` step 7 mints a second form, the split lands there and the binding map
  is already the thing it needs.
- **Span arithmetic** (`normalise_edits`, `emit_span_edits`, `command_text`). Byte placement of a
  decided edit. Not a decision at any altitude.

## §3 — The typed-witness shape

```
spend_certifier_trip(&mut Spine, CertifierTrip, impl Fn(&str) -> bool) -> (TripCleanup, TripSpent)
project_plan(&mut Spine, &str, &Ast, &PlanAuthority, &TripSpent) -> Plan
```

Two witnesses, both by reference, neither decoration: `PlanAuthority` is the intake's and `TripSpent`
is the latch's. The Spine is now `&mut` at the projection because the projection RECORDS what it
decided; that mutability is also what makes "project without recording" unspellable.

`CertifierTrip` travels by value throughout (it is a 1-byte `Copy` newtype; clippy's
`trivially_copy_pass_by_ref` is the proximate reason, correctness is indifferent).

## §4 — The meaning audit: species × field

Every Spine record species, every field, against the population its writer actually produces. **pin**
= verified and machine-held; **doc** = the field could not carry its documented claim and now states
the narrower truth; **flag** = a representation question, raised not answered.

### `SpineInvocation` (durable arm)

| field | verdict | note |
|---|---|---|
| `mode` | **flag** | FALSE: hard-coded `"whylog-replay"` from a seat unreachable on the replay branch (`30Mc` F3). DURABLE-persisted and replay-re-ingested ⇒ `stop-spine-mode-is-durable`. Untouched; a doc note now says so at the field so the next reader does not "tidy" it. |
| `argv` · `book` · `oracles` · `nonce` · `attempt` · `host` | pin | controller-minted; the whylog byte-identity gate holds them |
| `started_at` | pin | `None` on every clockless path; already documented |
| `grade` | **flag** | the doc CLAIMED `authored-before-contact`; the record is written after intake and the grade is object-global, so it wears the run's phase. `30M:ask-spine-grade-boundary`. Doc now states the discrepancy without choosing a pole; no representation touched. |

### `SpineRecordStream` (durable arm)

| field | verdict | note |
|---|---|---|
| `records` | pin | the admitted buffer, by the plane's own type |
| `instants` | **doc** | SPARSE, not one-per-record: a clockless run (`RunClock::Absent`, every loom path) stamps nothing, so this is empty beside a full buffer. "each record" was false. |
| `grade` | pin | host-influenced by construction |

### `SpineDisposition` (durable arm)

| field | verdict | note |
|---|---|---|
| `site` | **doc** | the member axis is `None` on EVERY row; the settlement decides per leaf and a member population arrives with the propagation lane. Fine keying is future-proofing, not evidence that members are distinguished. |
| `ast` · `sh` · `decision` | pin | golden byte-identity + the `region30-*` cases |
| `grade` | pin | stamped by `Spine::minted_at` |

### `SpineDigest` (durable arm)

| field | verdict | note |
|---|---|---|
| `digest` | pin | verified FNV-1a, 16 hex chars, at `erasability::decision_digest` |

### `SpineLoadDecision` (new arm)

| field | verdict | note |
|---|---|---|
| *species* | **doc** | only WITHHOLDINGS are recorded; the "which body a role name binds to" half reaches no record |
| `name` | **doc** | NOT one kind of thing: `Contested` carries a munged family base, `Unprovable` carries a synthetic `load@<ast-id>` locator. Display only. |
| `custody` | **doc** | universally `None` — the unbuilt column (`30F` §4.5). "Not recorded", never "no custody". |
| `withheld` | **doc** | `Some` on every row that exists |
| `WithheldCause::HelperConflict` | **doc** | NO WRITER: helper conflicts report as load-edge diagnostics and reach no record |

### `SpineSiteClassification` (new arm)

| field | verdict | note |
|---|---|---|
| `site` | pin | bridged through the plan's `ast → leaf` back-map (`30F`'s `fnd-classification-was-keyed-by-the-wrong-id-space`) |
| `class` | pin | now via `class_label`, a total `const fn`; referent-agnostic |
| `verdict_lane` | pin | |
| `invalidator` | pin (audited-by-repair) | the remit's repair holds; `a_classification_record_states_what_its_fields_promise` asserts `true` for an ordinary establish |
| `cells` | **repaired** | the remit's repair was INCOMPLETE — it matched only the two establish arms and dropped every other member class, so an `InlineCall` with a QUERY member reported a narrower account than the decision it describes. Now `class_cells`, recursive over aggregates. The pin gained a query member and was **falsified** (reverting the widening reddens it). |

### `SpineSolveCertification` (new arm)

| field | verdict | note |
|---|---|---|
| *species* | **doc** | the sole production writer emits ONE `whole-window` row per run, not one per pass. `30M:ask-certification-row-shape` is the pending direction; the row is NOT reshaped. |
| `pass` | **doc** | the one production value is `"whole-window"`; the per-pass vocabulary is where the pending direction would take it |
| `consistent` | **doc** | on a whole-window row this is exactly `!tripped` — one bit spelled twice. Under a per-pass row the two separate, which is why both fields exist. |
| `tripped` | pin | |

### `SpineVouch` · `SpineObservation` · `SpineValidityRound` (new arm)

**NOT MINTED** (`30F` §4.5, confirmed at tip). Each type doc now says so at the TYPE, so a reader
who never reaches `record_new_arm`'s doc-comment still learns that every field below describes an
empty population. Field-level audit is vacuous until a writer exists.

### `SpineProbeShip` (new arm)

| field | verdict | note |
|---|---|---|
| `site` | pin | carries `check.member`, the one place a member axis IS populated |
| `lane` | pin | verdict / predict / unresolvable, from the ship seat |
| `defining_file` | **doc** | `None` where the seat resolved no defining span, and ALWAYS `None` on an `Unresolvable` row |

### `SpineAdmission` (new arm)

| field | verdict | note |
|---|---|---|
| *species* | **doc** | the seat runs after the refusal path returned, so only the two authority-carrying arms are ever written |
| `outcome` | **doc** | `Refused` is representable and unreachable here; a refused run has NO admission record rather than a `Refused` one |
| `fault` | **doc** | universally `None`, because the arm that would carry one never reaches this record |

### `SpineSurvival` (new arm)

| field | verdict | note |
|---|---|---|
| `leaf` | pin | `SurvivalAccount::Silent` mints no row; the species is per-OUTCOME, not per-site, and reads that way |
| `outcome` | pin | held by the `sparing_differential` + `rederivation-is-demote-only` lanes |
| `poisoned_by` | pin | `Some` only for `Poisoned { via_reach: Some }`; "where one did" is accurate |

### `SpineRenderDecision` (new arm)

| field | verdict | note |
|---|---|---|
| `site` | pin | `None` only on `DefensiveEmission`, which is a unit property |
| `PinnedBinding.invoked` | pin | now read off the decided plane |
| `Refused.cause` | **repaired** | HARD-CODED `RefusalCause::Heredoc`, so the record stated a falsehood for every redirect-refused guard — the class `30Mf` F2 had just made reachable, and nothing read the record, so nothing said so. Now the real cause; pinned in `a_redirect_refused_guard_is_disclosed_on_every_surface` and **falsified**. |
| `OmitNeutralised.neutralised` | pin | |
| `DefensiveEmission.defensive` | pin | now an input to `Plan::decided` |
| `CertifierTripDemote` | pin | |

### `SpineRegionDecision` (new arm)

| field | verdict | note |
|---|---|---|
| `region` · `ast` · `sh` · `decision` | pin (audited-by-repair) | the `region30-*` cases and `a_real_trip_evicts_a_shared_region_elision_too` hold them |
| `routes` | **doc** | narrower than the census population in TWO ways, and only one is visible in the value: the cap reports what it dropped, while a route whose invocation the round could not key to a leaf is filtered out SILENTLY and leaves `dropped` at zero. Both narrow in the safe direction at the one consumer (a capped or empty account reads as still-live), but nothing may read this as the complete route set. |

### `SpineOutcome` (new arm)

**NOT MINTED — the fifth unminted species, where `30F` §4.5 disclosed four.** Its seat is the cli's
exit-code computation, which runs past every projection and holds no Spine there. Recording it means
deciding what an outcome record means for a run that refused before planning — the same question
`SpineAdmission` answers by absence. Named at the type and in `record_new_arm`'s doc.

## §5 — Findings

- **`30Nd:fnd-region-refusal-is-undisclosed`** — the span render refuses a REGION's edit on the same
  predicate it refuses a leaf's (heredoc; blocking redirect for a guard), but all three disclosure
  surfaces are leaf-keyed, and a region has no leaf. So a region whose authored span carries a
  heredoc runs verbatim with NO diagnostic, no narrative, and no `SpineRenderDecision`. Pre-existing,
  not introduced. The plane now RECORDS it (`DecidedRender::refused` carries `leaf: None`), which is
  what makes it findable; closing it needs a region-keyed record species or a region-keyed
  diagnostic site, both of which are `30I` step-7-adjacent design.
- **`30Nd:fnd-inline-call-cells-dropped-query-members`** — see §4; the remit's repair matched only
  the establish arms. Repaired and pinned here.
- **`30Nd:fnd-refusal-cause-was-hard-coded`** — see §4. Repaired and pinned here.
- **`30Nd:fnd-plan-steps-stay-publicly-mutable`** — `Plan::steps` and `Plan::regions` remain `pub`,
  so a caller CAN mutate a decided plan and leave its render plane describing a plan that no longer
  exists. Exactly one site in the tree ever did (a `render_corpus` test re-homing a license onto a
  redirect leaf); it now re-decides instead, and says why. REMEDY PRICED AND DECLINED for this lane:
  private fields plus `steps()`/`regions()` readers, ~90 external call sites of pure mechanical
  churn, against a hazard with one historical instance which was itself a test. Worth doing on the
  next lane that already churns those files.
- **`30Nd:fnd-the-canon-does-not-destructure-plan`** — `erasability::canonical_decision`
  exhaustively destructures a `Step` (so a new STEP field stops it compiling) but walks `Plan` field
  by field. A new `Plan` field is therefore caught only if it changes rendered bytes. `30E` §7 read
  "the erasability canon's exhaustive destructures" as covering the transition's blast radius; it
  covers the step tier only. Cheap to close (destructure `Plan` in `canonical_decision` and classify
  each field identity-or-exempt); not in this brief's scope.
- **`30Nd:fnd-typos-fix-rewrites-corpus-docids`** — TOOLING. `mise run fmt` runs `typos
  --write-changes`, which silently rewrote every `30Nd` in this lane's sources to `30And`. A
  corrupted docID is a dangling cross-reference `lint:docids` then reports, at a distance, with no
  hint that a fixer did it. Fixed at source: `spike/_typos.toml` now ignores the corpus docID shape
  `\b[0-9]{2,3}[A-Z][a-z]?\b`, keyed on the UPPERCASE letter so ordinal words (`30th`) stay
  spell-checked. Verified: `mise run fmt` no longer touches the slug. Anyone whose round letters
  produce another correction pair inherits the fix.
- **`30Nd:fnd-oob-astids-reached-a-panicking-read`** — `is_neutralised` and the refusal predicates
  read `ast.node(id)` unguarded. Before this lane only `rendered_guards` carried an OOB check, so
  `refused_render_steps` would have panicked on a synthetic plan; nothing reached it because the
  disclosure surfaces were only ever called with real trees. The decide walk runs for EVERY plan, so
  both seats now guard OOB and answer the run-it direction (`inv-no-throw`). Strictly more
  defensive; no in-arena behaviour moved.

## §6 — Deviations, OPEN

1. **The `(src, ast)` pair is threaded twice.** `Plan::decided` decides against one pair and
   `render_apply(src, ast)` is handed one; nothing checks they are the same tree. Every producer
   holds exactly one, so this is unreachable in practice, but it is not unrepresentable. Closing it
   means the plan carrying its own source — a real design question (the byte floor, the loom seam,
   and `bundle::project`'s occurrence identity all have opinions about who owns book bytes), not a
   local fix. **Owed, named.**
2. **`PinnedDefinitions` still carries the typeset preamble beside the binding map** (§2's second
   keep-render-side). The decision half is what the Spine records; the bytes half rides along. A
   second artifact form is what would force the split, and that is `30I` step 7's.
3. **The five audited decisions are recorded but still not CONSUMED from the Spine by anything.**
   `record_render_decisions` transcribes; the render reads the same in-memory value the transcription
   came from, not the record. That is the honest closure of `30F` §4.4 (the render no longer DECIDES),
   but a reader hoping the Spine record is now load-bearing for the artifact should not: it is a
   diffable account, as designed (`309:law-spine-outside-the-kernel` forbids the loop-back anyway).

## §7 — `tc-*` judgment calls, flagged UP

- **`tc-region-refusal-disclosure-home`** — does a refused REGION get a region-keyed
  `SpineRenderDecision` (new key axis on the species), a region-keyed diagnostic site (new locator
  shape in `aid`), or an explicit ruling that a region's refusal is disclosed at its contributing
  invocations? Each answer costs something different, and the third re-opens
  `30L:rul-two-identities-never-conflated`. Not settled here.
- **`tc-plan-owns-its-source`** — deviation 1. Should `Plan` carry the `(src, ast)` it was decided
  against, so the pair cannot be re-supplied? It makes the plan self-describing and kills a class of
  stale-render bug; it also gives the plan a second copy of book bytes, which the byte-floor and
  bundle lanes may object to.
- **`tc-certification-consistent-is-redundant`** — under the whole-window row `consistent` is
  `!tripped`. Whether to drop the field or keep it as room for the per-pass shape rides
  `30M:ask-certification-row-shape` and is deliberately NOT decided here.

## §8 — Excluded, untouched (as briefed)

- `30M:ask-spine-grade-boundary` — no grade representation touched. `Spine::minted_at` and the
  one-grade-per-Spine shape are exactly as at `655df12c`; the only change is a doc-comment on
  `SpineInvocation::grade` stating the discrepancy without choosing a pole. Neither
  stamp-authored-records-early nor record-local-grades is foreclosed.
- `30M:ask-certification-row-shape` — the row is audited (§4) and NOT reshaped.
- `stop-spine-mode-is-durable` — `SpineInvocation.mode` still writes `"whylog-replay"`. Nothing in
  this lane alters what the `.whylog` persists or what replay re-ingests: `DurableView`,
  `try_serialize_v2`, `WhylogV2Metadata` and the admission path are untouched, and the durable's
  bytes are inside the golden gate (`bless:dry` clean). The only change is a doc-comment naming the
  stop.

## §9 — Proposed steering text (conductor's to place; NOT edited by this lane)

`spike/crates/plan/CLAUDE.md`, replacing the last two sentences of
`certifier-trip-cleanup-runs-in-every-driver` ("The reification moved the cleanup's RESULT into the
decision plane, never the ACT of calling it; dissolving that surface by type is
`30M:rec-dissolve-trip-must-remember-structurally`, unbuilt."):

> The must-remember surface is DISSOLVED: `project_plan` demands a `certifier_trip::TripSpent`,
> whose one mint is `spend_certifier_trip`, which cannot be reached without a `CertifierTrip` in
> hand — so a producer that never spent its latch has no projection to call. The lexical roster
> stays as belt-and-braces, because it binds a different thing: a producer that builds a walled plan
> and hands the Spine somewhere else never reaches the projection seat.

`spike/crates/plan/CLAUDE.md`, new bullet under **Law — render**:

> - **the-render-decides-nothing** (`30E` §3's audit, closed) — every render-time answer is taken
>   ONCE, at `Plan::decided`, from the settled dispositions: which body a guard invokes, which
>   licensed edits the span render refuses, which `Omit`s have a neutralised controller, which
>   regions are still live, and the whole-artifact defensive-emission regime. `Plan::decided` is the
>   only constructor and `render` is private, so a plan whose render is undecided is
>   unrepresentable; `render_apply` and the three disclosure surfaces READ the plane and decide
>   nothing. A choice stays render-side only when neither `dorc why` nor a second artifact form
>   could ever need to account for it — the elided line's commented-original wrapping is the
>   exemplar (one `Replace`, two byte-shapes, same observables). `project_plan` records what it
>   decided in the same act, so a projection whose render decisions nothing wrote down cannot exist.

`spike/crates/core/CLAUDE.md`, new bullet under the Spine section (or wherever the conductor sites
the `30F` §7 texts):

> - **a-record-says-what-its-population-holds** (`30Nd` meaning-audit) — the census proves every
>   species PROJECTS; it does not prove a field means what its name says. A Spine field's doc states
>   the population its writer actually produces: universally-`None` columns say "not recorded" and
>   never "absent", filtered accounts say what they filtered, unminted species say so AT THE TYPE,
>   and a field that cannot carry its documented claim is narrowed rather than left aspirational. A
>   silent widening of a doc is the same defect as a silent widening of a field.

`spike/_typos.toml` — no steering change owed; the fix is in-file with its measurement.

## §10 — Gate results

- `mise run both gate:full-quiet` — **BOTH LEGS GREEN** at `cef1e6de`, Windows leg first
  (`preflight-bounds-before-spend`). Quiet is silent on success by design; rc 0 on both.
- `mise run test` — 2469 passed, 2 skipped (the ordinary platform-gated pair). Doctests included
  (12, of which the two `compile_fail` seals).
- `mise run clippy` — clean, `-D warnings`.
- `mise run check` — all four lint gates clean.
- `mise run bless:dry` — `bless: gates ok | e2e not blessed (dry)`; working tree clean afterward.
- `mise run xfail:census` — renders; no horizon expired, and `xfail_census_is_coherent` is green in
  the suite.
- **GOLDEN DRIFT: ZERO.** `git diff 655df12c..HEAD --stat -- spike/crates/cli/tests
  spike/crates/aid/tests` touches ONE file, `spine_baseline.rs`, and only its Rust source (two
  lines: `plan.pinned_definitions()` loses its now-unneeded arguments, and one variable is
  underscore-prefixed). No `expected.out`, no `expected.ran`, no transcript, no lock byte moved.
  This was the brief's pin on the relocation and it held without a single re-bless.
- **Comment budget**: 26 added inline `//` lines against a briefed 25, by
  `git diff 655df12c..HEAD -- "*.rs" | grep -cE "^\+\s*//($|[^/])"`. Three of the 26 are
  RELOCATED lines the function move re-adds (`// Distinct bodies per funcname…`,
  `// Conservative in exactly one direction…`, and the omit-arm note), so newly-authored narration
  is 23. Over-count disclosed rather than trimmed further; the remaining lines each carry a why the
  code cannot.

## §11 — Handoff to `30I:step-7-reify-plan-artifact-forms`

What step 7 inherits, and what it should NOT have to re-derive:

1. **`Plan::render_plane()` is the boundary.** A second artifact form reads `DecidedRender` — the
   binding map, the refusal list, the neutralised set, the live-region set — and does its own
   typesetting. It must not re-derive any of them, and it must not need `(src, ast)` to answer any
   of them.
2. **`PinnedDefinitions` is the one thing that will need splitting** (§6.2): the `invoked` map is
   form-neutral, `hoisted` is sh bytes. Step 7 is the forcing function.
3. **The region-refusal gap is step 7's most likely bite** (`fnd-region-refusal-is-undisclosed`,
   `tc-region-refusal-disclosure-home`). A second form that shows refusals will show a hole where a
   region's belongs, which is when the disclosure home has to be chosen.
4. **`Plan::decided` is the only constructor**, and `Plan::steps`/`regions` are still `pub` for
   reads. If step 7 churns those files anyway, `fnd-plan-steps-stay-publicly-mutable`'s remedy comes
   nearly free.
5. **The bundle lane's `LoadAccount` occurrence identity and the render plane are independent** —
   nothing in this lane touched `bundle::project`, and the plane keys by `AstId` in the book's own
   tree.
6. **Grades are still one-per-Spine.** Anything in step 7 that wants a per-record grade is blocked on
   `30M:ask-spine-grade-boundary`, unchanged.
