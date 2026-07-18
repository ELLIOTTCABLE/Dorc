# 26B — Reactive plan-construction & the capture fold: direction, rulings, problem-space bank

AI-authored (Fable, design-rubber-duck sitting WITH the human, 2026-07-17) — minted
INTO the r26 series **out-of-order, during r27 implementation** (block-context
mid-flight; lane-integration in flight on `ai/r27-book-integration`), because its
content rides the multihost / executor / transport cluster that r26 owns on revival.
It records: (a) a set of human-TYPED direction rulings on how plan-construction is
*eventually* supposed to work (reactive, not staged); (b) the resulting deferral of
the read-value/capture-fold lane out of round 27; and (c) a thorough problem-space
bank — the concerns discovered while designing the deferred lane — to ground the
r26-revival design-conductor. Authority: root docs and the §1 human-typed rulings
outrank everything else here; §§2–6 are conductor synthesis, confidence-marked.
Companions: `plans/22H` (the live-plan seed this direction confirms) ·
`plans/260`/`261`/`262` + `notes/26A` (the r26 spec this joins) · `notes/275` (the
value-prediction species) · `notes/27H`/`27I` (the landed capture representation) ·
`notes/219` (origin analysis; round-21 vintage) · `plans/27C` (context machinery) ·
`notes/277` §5 (the fixpoint-soundness clause + pins this note operationalizes) ·
`notes/279f` §5 (the read-value riders re-banked in §4 here) · **`notes/26C`** (the
follow-on deep pass, same day: semantics made precise incl. a §2 correction, the
quiet-welding audit against the r27 tip, and the R0–R4 revival ladder — read it
WITH this note).

## §0 — How this note came to exist, and what DIED

The TODO-ADDTL capture-lane item was graded PRIME deep-design (2026-07-17); a Fable
design session researched it and produced a fork-map framed around "a second
value-flow pass" vs "a fold-time substitution channel", with an optional "second
probe wave". The human rejected the *frame*: staging the analysis into waves — one,
two, any fixed N — is the wrong mental model. The eventual system is **reactive**
(§1). Consequences, settled in-sitting:

- DEAD: the single-wave/batch framing of the post-probe re-bind.
- DEAD: the fold-time substitution channel (it was a single-wave economy measure;
  it also never could deliver arm-killing, which is value analysis).
- DEMOTED: the "tier ladder" (value-rebind → reclassification-guards → new-probe-
  wants) stops being an architecture menu; the tiers are just consequences that
  fall out of the reactive engine's first iterations. The reclassification-guard
  tier is the human-graded *minimum* (§1).
- DEFERRED: the whole capture-fold build, out of r27, into this round
  (`26B:dec-capture-fold-deferred`, §1) — precisely BECAUSE building it
  single-wave now would bake pipeline-order and provenance decisions in the frame
  just declared wrong. Not-building is the door-open move.

## §1 — Human-TYPED rulings (transcribed from the 2026-07-17 sitting)

- **`26B:rul-plan-construction-is-reactive`** — the probe/plan-construction phase
  is, in the eventual design, *effectively real-time*: incoming information (probe
  records, facts) re-analyzes and changes output; produces NEW analysis and new
  probing; and can *mutate and update ongoing analysis*. The end-state includes
  executors in constant contact with the orchestrator, which can **add and
  discharge work** based on the total-world-knowledge the orchestrator is
  building. Framing this as "a second wave" is unproductive (it is >1, unknown N);
  so is framing it as "waves" at all. Spike direction drawn from this: *do not
  half-ass* the eventual mechanism's semantics, or the holes stay hidden.
- **`26B:rul-consent-cut-absolute`** — reactivity NEVER extends into the apply, in
  the current design; *never* in default behaviour. Default is fully-welded:
  **one plan, one moment of consent** — the second of only TWO interactions we
  ever have with most users: (1) the CLI invocation (flags chosen, files
  written); (2) the presented plan. All state must be static by presentation.
  (Flags may eventually modify this — auto-run-plan removes the second moment; a
  user-is-present-during-apply flag is conceivable — but these are graded
  almost-dangerous-to-consider: the attention==product constraint is already hard
  to keep LLM work focused on. For now: one user-moment.)
- **`26B:rul-one-attention-moment`** (the human's precision pass on the above) —
  "static by presentation" means presentation *to the CLI user*, the primary user
  we design around; sugar can be sprinkled on top, and some of that sugar WILL be
  realtime-results-flowing-in TUI fanciness. It does not change the core
  value-prop: what is *required* of the user to complete their ops task is **one
  synthesized attention-moment**, whether CLI or TUI. In the CLI this is literal —
  it displays the final plan(s) and exits / dumps the files.
- **`26B:rul-iteration-waste-acceptable`** — performance of the reactive loop is
  puntable: it is fine to run too many iterations now and tune the count down
  later with more intelligence. (Consequence, conductor-drawn and confirmed
  in-flow: recompute-from-scratch-per-iteration is a legitimate implementation of
  the reactive semantics; in-place incremental update is efficiency engineering,
  not design work. See §2.)
- **`26B:rul-reclassification-guards-are-floor`** — "tier-b is a minimum": using
  re-bound values to re-classify downstream sites and reach their vouches (which
  licenses *guards* with no probe measurement) is the floor of what the capture
  machinery must deliver, not a candidate stopping point.
- **`26B:dec-capture-fold-deferred`** — the human's lean, adopted: defer the
  capture-fold build to a future round; let r27 end with the then-current
  builder's existing work. This note is the deferral's design bank; the r26
  revival owns the build (the reactive engine, executors/transport, and multihost
  are one design cluster — `22H` is the seed, `262` the spine it folds into).
- **Aside, banked** (human-typed, same sitting): the multihost firewall — no host
  fact ever enters/affects logic or decisions about another host — is welded *for
  now*; there is a far-distant universe where conductor-knowledge *synthesizes
  and synchronizes between hosts*. See `26B:seam-per-host-partition` (§4).
- **Motivating example** (human-typed, near-verbatim; drives
  `26B:need-cancellation-finality-gate` in §4): a very-slow-to-probe, read-only
  check. Ideally that probe ships in parallel and can be *cancelled* by the
  executor when a separate, in-parallel fact disnecessitates it — say the site
  degraded to must-guard: if the expensive check will run at apply-time anyway,
  keeping it in the probe-time work-queue is waste. And that state can become
  true *partway through* a probe.

## §2 — The technical reframe (conductor synthesis; +SURE on the theory)

What §1 describes is, in PLT terms, **asynchronous chaotic fixpoint iteration over
a monotone knowledge system**. During plan-construction, Dorc's knowledge is
naturally monotone: a captured value refines ⊤ toward concrete; a record adds a
fact; a killed arm shrinks the wall-set, which only ever *unlocks* further
refinement. The classic chaotic-iteration result then applies: re-evaluate on any
input change, in any order — the fixpoint is the same. Three consequences:

- **Confluence is THE correctness statement.** The final plan is a pure function
  of the total fact-set, independent of arrival order. `277` §5's
  fixpoint-soundness clause and its two pins (`pin-no-outcome-as-generator`,
  `pin-set-meet-order-independence`) stop being defensive fences and become the
  engine's advertised property — and a DST target: hostsim shuffles
  record-arrival order across seeds; the pin is *byte-identical plan under every
  ordering*.
  <!-- /* corrected by 26C §1 (2026-07-17, same sitting-cluster): byte-identical-
  under-shuffle is guaranteed only for CONFLICT-FREE runs — merge_observable's
  conflict-meet (⊤→Value→⊤, 22H §1) makes want-generation path-dependent, so
  conflicted runs can gather order-varying record SUPERSETS (sound, but not
  byte-reproducible). Fold-confluence over a fixed record-set is unconditional.
  See 26C:finding-confluence-needs-conflict-carve + ask-confluence-carve-choice. */ -->
  A single-wave build could never exercise this property; that is the
  concrete sense in which single-wave "hides the holes".
- **The batch pipeline is the degenerate schedule** of the reactive engine, not
  the other way round. The honest spike-scale mechanism is an event-shaped pure
  kernel step — `(state, new-facts) → (state′, new-probe-wants)` — driven to
  quiescence by a trivial batch driver, with recompute-from-scratch per iteration
  (`26B:rul-iteration-waste-acceptable` licenses exactly this; analysis is free
  per the perf-doctrine, and confluence makes recompute semantically identical to
  in-place mutation). All async/scheduling lives at the edges; `inv-determinism`
  holds; DST fuzzes delivery order against the confluence pin.
- **kFAIL supplies the safety story for the whole reactive zone**: every mistake
  a stale intermediate state can cause on the probe side is a wasted or withheld
  *read* (kFAIL-withhold) — never harm. The one place correctness re-enters is
  the mint (§3).

## §3 — The boundary structure (where "real-time" starts and stops)

- **The reactive zone is plan-start → plan-mint.** The mint is a consistency cut:
  the presented plan must be a coherent snapshot with all in-flight work quiesced
  or accounted (`26B:rul-consent-cut-absolute`). In the batch driver, quiescence
  is trivial (loop until no new probe-wants). With real executors it is genuine
  distributed termination-detection — a known-hard problem class; see
  `26B:need-quiescence-witness-at-mint`.
- **The apply is fenced** by `26B:rul-consent-cut-absolute` and, independently, by
  the standing `rul-divergence-proceed` (spike/CLAUDE.md): apply-time events are
  report-items only; no second-guess layer; all decisions front-load into the one
  approval. Nothing in this direction touches that.
- Multi-host composes as partitions: per-host fact-stores iterate independently
  (the welded firewall); the one *shared* resource is the orchestrator's
  scheduler. Cross-host synthesis, if it ever comes, is a partition-lift on the
  fact-store, and the store should be SHAPED so that is true (§4).

## §4 — The obligations & concerns bank (each slugged; the revival conductor's list)

- **`26B:need-cancellation-finality-gate`** — economy-cancellation (the §1
  motivating example) is only plan-preserving if the disnecessitation is *final*.
  Mid-iteration, dispositions can still improve (monotone knowledge lifts walls:
  a later fact can turn today's must-guard back into an elision candidate).
  Cancelling on a non-final judgment makes the *final plan* arrival-order-
  dependent — breaking the confluence pin and plan reproducibility (a trust
  surface). The design owes a finality gate ("no possible future fact improves
  this site") before economy-cancellation may fire; alternatives (conservative
  cancel-classes; accepting bounded conservative-direction nondeterminism) must
  be argued against the reproducibility cost, not adopted by drift. ~SUSPECT
  finality is derivable cheaply for common cases (a site below a *confirmed*-run
  un-modeled wall can never elide), but that is unverified.
- **`26B:need-quiescence-witness-at-mint`** — the mint takes a quiescence witness.
  Batch driver: trivial. Executor era: termination-detection (prior art exists —
  Dijkstra-Scholten / credit schemes; do not improvise). r27's only obligation:
  plan-mint stays a single choke-point function so the witness can wrap it later.
- **`26B:need-probe-want-diffing`** — iteration N+1 ships `wants(N+1) \
  shipped(≤N)`. Termination: monotone descent on a finite lattice — each
  iteration either resolves a capture/fact or quiesces; capture-dependency chains
  (`A=$(f); B=$(g "$A")`) bound rounds by chain depth. DST-pinnable.
- **`26B:need-provenance-through-rounds`** — a fact learned in iteration 2 must
  cite the iteration-1 capture that made its probe compilable; why-lens chains
  lengthen; the attribution machinery must thread iterations. (The sin-ordering
  demands this: a wrong late-iteration elision must still land attributed.)
- **`26B:need-arrival-order-shuffle-pins`** — the DST harness gains: per-round
  record service from hostsim; seeded arrival-order shuffling; the
  byte-identical-plan-under-shuffle pin; plus the deferred-lane must-covers
  re-banked below.
- **`26B:gate-binding-site-coherence`** — THE opening design-question of the
  revival, deliberately left un-ruled (human choice, 2026-07-17). The knot: true
  case-arm removal requires the captured (frozen) value to govern the artifact;
  but `275` §4 postpones artifact-entering substitution, so the capture-binding
  line stays LIVE in the apply — and a live re-capture that diverges from the
  frozen value can dispatch into structure the plan amputated ⇒ silent
  fall-through ⇒ under-execution, the cardinal sin. Sharpened by
  `26B:rul-consent-cut-absolute`: a guard's live divergence fails toward *running
  the shown line* (consented); a stale capture under killed arms fails toward
  *silently skipping structure* (un-consented). So live-binding-plus-killed-arms
  is doubly disqualified, and the realistic menu (conductor-proposed, NOT acked)
  narrows to: (a) **freeze-in-artifact** — unpark artifact-entering substitution
  narrowly, world-spoken-pinned exactly as `275` §4's postponement clause
  pre-anticipates, with `26B:need-scrub-before-freeze` as precondition; or
  (b) **all-or-nothing folding** — a case folds only when the entire construct's
  fate is decided; otherwise display-tier dimming only (render-honest, buys less
  attention). Divergence-*detection* (a live re-read compared to the frozen
  value, report-only) composes with `rul-divergence-proceed` under either.
- **`26B:need-scrub-before-freeze`** — if freeze-in-artifact wins, captured host
  bytes enter rendered artifacts for the first time: the scrub-at-capture seam
  (`22A`) and the plan-display-vs-secret-hygiene tension (`24R:repurp-finding12`)
  must be *named* before this round lands the fold. (The secrets round itself
  stays human-designated and separately owned; this is a sequencing dependency,
  not ownership.) With the r27 deferral, the previously-flagged urgency stands
  down to this round's precondition.
- **`26B:bank-deferred-lane-riders`** — the read-value riders that were queued on
  r27's struck lane, re-homed here so they don't orphan with the stale `27J`
  §2.4: the `279f` hard gate (never elide a capture-binding whose variable has
  live apply-time consumers outside the folded region); the walls-patrol
  clarification (the value-freeze patrol IS the walls machinery; unmodeled
  interposers wall the fold by default); the merged-streams capture fence checked
  AT capture (`v=$(cmd 2>&1)`; `271:rider-merged-streams-capture-fence`); the
  nested-wrapper lend/ρ composition rule (pointwise; ⊤ propagates);
  case-scrutinee arm-resolution precision (POSIX fnmatch semantics; un-modelable
  patterns refuse the whole fold); stdin-code carriage (`274` §11, unwalked); the
  DST must-covers (spaces, empty output, nonzero rc, merged stderr, hidden walls,
  probe/apply value disagreement); the `275` §4 validity table + the world-spoken
  floor (`271:rul-composed-bytes-defer-and-floor`) as the fold's license terms.
- **`26B:need-context-qualified-captures`** — `FactKey.context` is real as of
  `27M`; captures can be context-qualified (a read inside a `sudo`/`su` region
  measures in its denoted context per `plans/27C`). The re-bind must key captured
  values by (site, context); folding a value across a context boundary rides
  `27C`'s machinery, never this note's.
- **`26B:watch-dependent-chain-scheduling`** — shipping dependent chains on-host
  (`PKG=$(cat__predict /etc/pkg); dpkg__predict -s "$PKG"` inside one probe) is
  the executor discharging work locally to save round-trips: a *scheduling
  optimization* of the same fixpoint, natural in the executor era. Doctrinally
  compatible (oracle bytes + synthesized scaffolding satisfy
  `271:rul-only-oracle-bytes-ship` / probe-composition-walls) but the returned
  fact's coordinate is then host-computed — host-minted bytes participating in
  cell-naming, the hostile-host hook (`plans/102`; `275` §7). Controller-side
  cross-check (re-derive the coordinate from the returned capture; compare) is
  the candidate mitigation. Reserve; do not privilege it at v1 — controller-side
  iterations exercise the mechanism this round exists to prove.
- **`26B:watch-kfacts-substrate-lean`** — a fact-accumulation-driven engine leans
  the `kFACTS` substrate knob (high lock-in) toward materialized/relational.
  Recompute-per-iteration over immutable structures stays substrate-agnostic;
  the revival must make any substrate commitment an explicit decision, never a
  drift. Related: `kSTATE` stays parked — all iteration is intra-run; nothing
  persists (rec-5 untouched; "just re-plan" can never substitute for intra-run
  iteration, which is *why* the loop must exist).
- **`26B:seam-per-host-partition`** — key the fact-store per-host from day one so
  the welded firewall is a *partition*, not an architecture; the far-future
  cross-host synthesis (§1 aside) becomes a partition-lift. Facts already carry
  site/host-shaped keys (`inv-site-keyed-results`; `FactKey.context`); keep it
  true as the store evolves. Build nothing.
- **`26B:need-per-round-harness`** — e2e/hostsim must serve records per-iteration
  rather than one fixture file; the e2e case shape changes accordingly (real cost
  line-item; hostsim's seeded-injection design anticipates it).
- **`26B:split-semantic-versus-concurrency-holes`** — honesty about coverage:
  the batch-driven reactive engine surfaces the *semantic* holes (re-analysis
  soundness, termination, order-independence, provenance-through-rounds,
  quiescence-at-mint, cancellation-finality). It CANNOT surface the *concurrency*
  holes: backpressure, partial-failure mid-stream, executor liveness, transport
  interleavings — those need `142`/`kCOMMS`'s real executor work, which this same
  round owns (`260`/`262`). Do not mistake the first list's green for the second
  list's coverage.

## §5 — What r27 already reserves (don't rebuild), and the deferral's terms

Landed representation, verified reserve-shaped (`27H`'s foreclosure walk; `27I`):
cause-tagged fragment-preserving recipes with `TopCause::WalledRead` reserved; the
`ValueGrade` lattice (world-spoken vs program-text distinction ready for capture
fragments); per-channel backing-SETS with the universal meet + the `277` §5
vacuity invariants (backing-sets non-empty by construction; ⊤ never encoded ∅);
the wire's single-line stdout field with the embedded-spaces round-trip pin;
site-keyed individually-addressable work items (`inv-leaf-seam` — cancellation
gets its work-identity for free). The probe remains flat per-site composed
predicts (no book CFG ships) — the reactive engine changes the *driver*, not the
`rul-only-oracle-bytes-ship` emission law.

The deferral's terms for r27 (so the block-context conductor's obligations are
explicit): `lane-read-value-slice` is STRUCK from the round — not trimmed,
removed; block-stdlib and the field-trial revival proceed unaffected (neither
depends on capture folding); the only code-shape obligations left behind are
negative — plan-mint stays a single choke-point; the reserved seams stay open;
nothing new closes them.

## §6 — Evidence ask on the field trial

**`26B:ask-trial-counts-capture-walls`** — the r25 trial now runs WITHOUT the
capture fold, which converts it into the sizing instrument for this round: every
site the trial walls *because of a dynamic value* (`$(hostname)`-shaped scrutinee,
captured-operand argv, dynamic-named guard) is a fires-often data-point for the
capture lane's worth. The trial protocol (`252`) should count these as a named
category at adjudication, so the revival's investment is sized by evidence rather
than enthusiasm. (One-line rider for the trial-revival conductor; `270` §5's
owed-on-revival list is the natural home.)

## §7 — Status table

| component | status |
|---|---|
| reactive direction; consent cut; one-attention-moment; iteration-waste punt; guards-floor; the deferral | HUMAN-TYPED, 2026-07-17 (§1) |
| single-wave + fold-time-substitution DEAD | consequence of §1, stated in-sitting, unobjected |
| chaotic-fixpoint reframe; confluence-as-property; batch-as-degenerate-schedule | conductor synthesis (+SURE on theory; spike-fit ~SUSPECT until built) |
| cancellation-finality gate | conductor-derived from the human's example; mechanism unowned |
| binding-site coherence menu (freeze-in-artifact vs all-or-nothing) | conductor-proposed; deliberately UN-ruled; THE revival gate-question |
| rider bank (§4) | re-homed from `27J` §2.4 / `279f` §5; unchanged in substance |
| trial counting ask | proposed; wants a typed ack at trial revival |
