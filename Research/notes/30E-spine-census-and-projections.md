# 30E — The Spine census: decision-state, projections, and the durable's exclusion set

> Tier: **LLM-authored, builder (Opus-class)**, lane `lane-spine-reification` on
> `ai/r30-lane-spine-reification` from `ai/main@41bb7ef3`. This is `plans/309` §5.1's
> `stage-spine-census` deliverable, measured against code rather than reasoned from the
> corpus; confidence markers where a claim is not measured. Adjudicated by the conductor
> 2026-08-17 (§8); the rulings there are what the build executes.
>
> Vocabulary and laws: `notes/306b` (authoritative for influence), `plans/309` (THE spec
> for this lane), `plans/306c` (the stopped remainder), `notes/307a` (the seams lane's
> as-built `core::influence`).

## §0 — What the census is for

`309:rul-durable-by-exclusion` inverts an inclusion list: the `.whylog` projection is
defined by what it EXCLUDES from a totalistically-tracking Spine. That inversion is only
as good as the inclusion list it starts from, and the corpus's statement of that list is
stale in two places (§1). Everything below is the measured list, the decision-state that
must hang off Spine, and the projections that read it.

## §1 — The durable as-built

The live durable is **v2 only**: `plan::whylog::WhylogV2Metadata` serialized by
`try_serialize_v2`, written from `cli::main::write_whylog`. Twelve fields:

`mode` · `argv` · `book(path, digest)` · `oracles[](path, digest)` · `nonce` · `attempt` ·
`host` · `decision_digest` · `started_at` · `instants[](ordinal, RunInstant)` ·
`apply[]{leaf: u32, disposition: String, predicted: bool}` · the records stream
as-received.

### `fnd-timings-are-already-durable` [measured]

`started_at` and the per-record arrival `instants` persist today, minted controller-side
(`28F:rul-probe-instants-host-says-no-times`). `306b` §2c lists timings as a SOFT
*suggested addition* — "Timings, when they arrive". They arrived. The census classifies
them durable-on-day-one; this is recording reality, not a tripwire entrance.

Consequence for `plans/309` §5 stage-3: grade-stamping lands directly on an
already-durable species that `306b` itself calls `host-influenced` by construction (the
host determines a duration) and notes "additionally disclose state that no record stated".
**Owed doc correction**: `306b` §2c should move timings from suggested-addition to
persisted-today.

### `fnd-two-durable-grammars-one-writer` [measured]

The v1 family — `WhylogDoc`, `whylog::serialize`, `whylog::parse` — has **zero production
callers in either direction**. Writes are v2; replay reads v2 (`admit_unscoped_whylog` →
`parse_v2`, which admits exactly `WHYLOG_V2_TAG`). v1's `parse` accepts only
`WHYLOG_TAG` and politely refuses a v2 durable as `WhylogVersionRefused`.

v1 survives solely through `whylog::inspect`, called from `dorc-loom`'s consumer against
hand-authored `.loom` fixture SECTIONS. That is a fixture-only durable grammar behind a
permissive parser whose shape has already diverged from the product's — the exact rot
`rul-fixture-identity-never-production` describes ("comments are not a fence — absence of
a constructor is") and `sinv-production-fences` governs. Disposition: §8.4.

## §2 — The census: three arms over Spine record species

`309:mech-census-three-states`, following the `CollapseKind` completeness-census pattern
(`aid/tests/narrative_completeness.rs`): a no-wildcard match over species, so a new one
cannot land unclassified.

### durable-via-`DurableView` — 4 species; ENTERING is the tripwire

| species | as-built home | the View's fields |
|---|---|---|
| `SpineInvocation` | `Framing` + argv + the source table | mode · argv · book · oracles · nonce · attempt · host · `started_at` |
| `SpineRecordStream` | `AdmittedUnscopedHostRecords` | the as-received buffer + `instants` |
| `SpineDisposition` | `Plan.steps[].disposition` | **`{leaf, tag, predicted}` only** |
| `SpineDigest` | `erasability::decision_digest` | the digest string |

`SpineDisposition` is the case that vindicates `DurableView` over a species-arity census
(`309` critical-2): the RECORD is `SiteId`-keyed and license-bearing, while the VIEW emits
`leaf` plus a tag. Field-level exclusion becomes structural — the influence grade is
excluded *by not existing in any View*, silent field-growth is unrepresentable, and
lifting an exclusion is one field added to one View, a diff that IS the tripwire's
mechanical form.

### excluded — ruled non-durable

The influence grade (`306c` §2's load-bearing scope fence: v0 is in-memory precisely so
this arc does not fire the tripwire) · narrative operands, `ProvId`, arena handles
(`operands-are-pure-and-capped`) · freeform host output (`306b` §2b) · working lattice
state.

### `new` — 11 species; transitory, debug-dump-only, structurally unable to ship

`SpineLoadDecision` (definition binding · custody · contested families · never-live ·
helper conflicts) · `SpineSiteClassification` (the classify 7-tuple: SkipClass ·
verdict-lane · kills · kill-coords · fact-backings · degrade causes) ·
`SpineSolveCertification` (per-pass consistency + the `CertifierTrip` latch) · `SpineVouch`
(attached/suspended + custody) · `SpineProbeShip` (which body shipped per site, or
unresolvable + cause; plus the derivation/resolver/reach lanes) · `SpineAdmission`
(Admitted / NoObservation / Refused(RecordsFault) + the influence phase marker) ·
`SpineObservation` (per-site observable, the `by_fact` merge, collapsed cells) ·
`SpineValidityRound` (the erasure ledger + cascades) · `SpineSurvival` (witnesses ·
crossings · demotions · re-derivation disagreements) · `SpineRenderDecision` (§3) ·
`SpineOutcome` (`RunOutcome` → exit code · advisory routing · whylog eligibility).

## §3 — The hidden-decision audit

Decisions that are license-relevant and made at plan-build or render time, invisible to
the structured decision plane. These are why the smoke-diff (`309` §4) is not optional:
none moves a byte gate on its own.

- **`dec-pinned-definitions`** — `309`'s named exemplar, confirmed as-built.
  `Plan::pinned_definitions` runs INSIDE `render_apply` and decides which body each guard
  invokes and under what name (content-dedup / already-in-place / bare-if-singleton /
  hash-munge). A misalignment swaps WHOSE judgment executes — pope-sin tier
  (`271:rul-sin-ordering`).
- **`dec-render-refusal`** — `refused_render_steps`: a leaf the disposition layer LICENSED
  to elide or guard that the span render REFUSES (heredoc; and, for a guard, a blocking
  output redirect). The `Step` still reads `Replace`/`Guard` while the artifact runs the
  bytes verbatim. The structured decision and the artifact disagree by design, with only a
  diagnostic and a `RenderRefusal` narrative between them.
- **`dec-omit-neutralisation`** — `is_neutralised` decides at render whether an `Omit`
  becomes `:` or stays verbatim, recursing through the controller. This is
  `erasure-demands-a-proof-and-a-rendered-death`'s wrong-yes fence, evaluated at render
  time.
- **`dec-defensive-emission`** — `plan.defensive_emission` is assigned in the cli AFTER
  `build_plan_walled`, from a lexical definition-vector scan plus
  `env.unresolvable_loads()`. A whole-artifact emission-regime decision living as a
  post-construction field poke.
- **`dec-certifier-trip-cleanup`** — `demote_on_certifier_trip` mutates dispositions after
  construction, and `plan/CLAUDE.md` warns that a NEW driver MUST call it. A
  must-remember-to-ask surface is exactly what reifying Spine should dissolve
  (`withdrawal-is-applied-once-never-consulted`'s shape).

## §4 — Projections and their authority class (`309:pin-authority-exit-list`)

| projection | sink | authority |
|---|---|---|
| apply artifact | stdout | **mutation** — the primary exit |
| probe artifact | stdout / transport | host **execution**, read-only by construction |
| orchestrator connections | `transport_edge::ship_probe` / `apply_to_host` | **credential + context-entry** |
| shim dir | disk | controller-owned scratch |
| `.whylog` | disk | none; **sensitive** (`law-whylog-is-sensitive`) |
| why report · why-lens · attributions · summary · digest line | stdout / stderr | none |
| **exit code** (`RunOutcome`) | process | **authority-adjacent** |

The exit-code row earns its place: `EXIT_BOOK_UNMODELED` exists precisely so a
`dorc … && deploy` chain STOPS. A projection that gates a downstream deploy belongs in the
enumeration even though it carries no mutation itself.

### `fnd-apply-authority-detaches-from-spine` [measured; confirmed-by-design]

`ship_consented_apply` reads the artifact from a file or stdin and runs NO analysis: there
is no Spine on that path at all. So `306b:rul-report-only-output-cannot-plan` cannot be
enforced there, and containment must bite at **artifact production** — which is exactly
what `306c` §3a already requires ("contain at the analysis output, not at plan emission").
Worth naming rather than building against: once the bytes exist the artifact is a detached
authority-bearer, and that is the design, not a gap.

Confirmed alongside: `cli::why::WhyReport` holds `plan: &dorc_plan::Plan` and `WhyWorld`
owns one. `307a`'s `tc-report-substrate-is-the-plan` is the as-built accident `309` §0
calls it, not an architectural truth.

## §5 — Type shapes and homes

- `core::spine` — pure data, dependency-clean (`core`'s own law), `SiteId`-keyed
  throughout, `Ord`-deterministic. Spine is decide-plane IDENTITY, which is why it sits
  beside `SiteId`/`LeafId` rather than in the describe plane.
- `law-spine-outside-the-kernel` is realized by POSITION, not by a guard: Spine is written
  post-decision from outside anything the solver compares, so Spine values never enter
  compared state and no `Eq`-exclusion is needed anywhere. The `CollapseNarrative`
  `Eq`-exclusion (`22W` §2) is cited as the failure-mode this AVOIDS, never a technique to
  generalize.
- `DurableView` types live in `plan::whylog`, beside the serializer they feed. Records
  themselves never implement serialization — that absence is the field-level exclusion
  mechanism.
- The `aid → core` edge is untouched; nothing here gives `core` a describe-plane
  dependency (`aid-is-the-describe-plane`).
- `pin-debug-dump-gating`: the `cli::results::admit_fixture_records` shape — a signature
  that CANNOT NAME a production sink (no `Framing`, host, nonce or attempt parameter, none
  addable by a caller), plus the lexical non-empty-walk gate for the caller half that no
  type can fence.

## §6 — The smoke-diff dump (`309` §4)

Build-to-kill migration scaffolding, frozen ONCE at the base commit, walked from the OLD
code: site-keyed decisions · definition-identity + custody per binding · witness sets ·
the digest at **SITE granularity on both sides**, so the known keying change needs no
whitelist. Deterministic and sorted. Two fences hold: it is never the whylog (no
durable-tripwire contact) and never the census `new`-arm debug dump (different mechanism,
different lifetime). Its schema INFORMS the owed `SiteId` decision-dump product feature and
must never become it. It is a **smoke-testing machine, not an acceptance gate**: non-empty
output is material for judgment by eye at the fold sitting.

Honest residual, restated: the dump covers only decision-state the old code makes explicit
enough to walk. A fully-implicit decision is invisible to the baseline too, which is why
§3's audit is checkpoint-tier and the diff is never sold as total.

## §7 — Red-list (big-bang blast radius)

20 source files touch `Disposition` / `build_plan` / `Plan{}`; 164 case entries under
`crates/cli/tests`, 199 under `crates/aid/tests`; 24 files reference the whylog.

Expected red: the erasability canon (its exhaustive destructures stop compiling BY DESIGN
— that red is the gate working, per `plans/22A` concl-2) · `decision_digest` · every e2e
golden and transcript, which must return BYTE-IDENTICAL · the whylog loom fixtures ·
`narrative_completeness` (new projection-tier classes) · the four cli/analysis lexical
fences · the `plan::rederive` and `dorc-sparing-reference` adapters that consume `Plan` and
survival.

End-gate: every golden byte-identical, the durable's bytes byte-identical, both legs, the
standing checker gates (certifier · sparing re-derivation) green.

## §8 — Adjudicated rulings (conductor, 2026-08-17)

1. **`stop-siteid-digest-rekey`** — the in-memory Spine is `SiteId`-keyed from day one (the
   member-collapse is FIXED at the Spine tier) and the smoke-diff dump is site-granular on
   both sides; the DURABLE projection keeps `apply[].leaf: u32` byte-stable this stage. The
   durable-side re-key is an exclusion-lift, deferred behind the tripwire (§9).
2. **`stop-drop-accounting-destination`** — drop-accounting is IN-MEMORY / render-only for
   stage-2. `309:rul-drop-accounting-completes-the-narrative-law`'s "the durable says what
   it chose not to keep" becomes true at a later lift, under review, not now (§9).
3. **`stop-timings-already-durable`** — classification accepted; `started_at`/`instants`
   are durable-on-day-one, and stage-3 grade-stamps them as `host-influenced` per `306b`.
   The `306b` §2c doc correction is routed to the conductor.
4. **`flg-v1-durable-is-fixture-only`** — AUTHORIZED in scope: delete the v1 grammar and
   re-cut the loom fixtures onto v2, in one commit, under
   `rul-strawman-formats-no-compat` (pre-user: rename/reshape in place, never an adapter).
   The loom editability and prose-provenance laws survive the re-cut intact.
5. **`pin-spine-crate-home`** and **`pin-debug-dump-gating`** — endorsed as §5.
6. The exit-code row joins the authority enumeration;
   `fnd-apply-authority-detaches-from-spine` is confirmed-by-design and nothing extra is
   built for it.

## §9 — Owed later-lifts (each a tripwire event, human-gated)

Recorded here so neither is lost to the transition's own green:

- **`lift-durable-siteid-keying`** — re-key `apply[]` and the decision digest to `SiteId`
  in the DURABLE. Adds a field and changes committed decision detail, so
  `rul-durable-contents-reviewed-before-design` binds. The in-memory half lands this
  round; the durable half does not.
- **`lift-durable-drop-accounting`** — persist the projection-drop account. New record
  species in the durable, same tripwire.

Both are enrichment of what the durable holds. Neither may be taken as licensed by this
lane's work merely because the in-memory structure now makes them cheap.

## §10 — Fences held, and the parallel lane

Untouched and unaffected by this census: rec-5 (`probe-tape-not-a-cache` — the Spine is
never a cache; re-ingestion stays aid-plane only) · `two-plane-aid-law` ·
`law-whylog-is-sensitive` · `306b` §6b (influenced values never gate engine control flow;
its enforcement mechanism stays an open pin, not this lane's).

Parallel-lane fence observed: nothing in this census requires an edit to
`spike/crates/oracle/` custody seats, `analysis::funcenv`'s keying, `core::DefinitionCustody`,
or `verdict_cell_or_auto`'s shared slot — the last of which is the custody lane's
(`307:§ack-veto-review`). `dec-pinned-definitions` READS `DefinitionCustody` but only ever
COMPARES custodies, per `custody-is-one-newtype-and-one-crossing`.
