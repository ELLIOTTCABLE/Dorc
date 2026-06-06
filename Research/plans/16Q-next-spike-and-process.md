# Dorc — 16Q: the next analyzer spike, and how to run it better

> **What this is.** The forward-look companion to `16P-spike-postmortem.md`, and the last `16x`
> document. Where `16P` is the *neutral record* of what the round-16 spike built and found, this is
> *recommendation* — what a second analyzer spike should build, what to settle on paper first, the one
> design-conclusion worth enshrining now, and how to run the next throwaway-spike better. Read it as
> opinion held with marked confidence, not as settled finding.
>
> **Lineage.** `16P` §6 deferred "a separate whole-spike adversarial run." This doc is a
> crosscheck-grounded down-payment on that: its findings come from running the `adversarial-crosscheck`
> pattern (a neutral + a disowned-adversarial pass, clean-context, un-seeded) over `16P` and the
> quarantined spike, then verifying the survivors against the spike's own source. That pass produced one
> load-bearing correction to the `16P` record (§1) and one minor one (§5, `ap-2`).
>
> **Citations.** Same discipline as `16P`: a quarantined note is cited by its non-resolving slug
> `notes/quarantine-DO-NOT-READ/16X` (no `Research/`, no `.md`); the disposable spike's code by **Rust
> symbol path** (`plan::prove_replaceable`) — one realization, not a contract. First-party
> `README`/`DESIGN`/`KNOBS` are ground truth; trust them over this. `16P`'s thread/problem handles
> (`T7`, `DP-4`) and `KNOBS` slugs (`kPRECISION`) are reused so this doc shares vocabulary with both.
>
> **Scope.** Analyzer, not orchestrator. Everything below is *pure-kernel-amenable* — buildable and
> DST-fuzzable with no host, network, async, or wall-clock — which is what kept spike-1 cheap and must
> keep spike-2 cheap. The apply executor, multi-host/fleet, TOCTOU re-probe, and `kSCHEDULE`/`kOBJECTIVE`
> are orchestrator-end and out of scope here by construction.

---

## 1. Correction to the `16P` record: the precision keystone (load-bearing — read first)

`16P` is honest that the precision/recency layer is `NOT BUILT` (§3.2), but it files that omission as one
bullet among many deferrals and frames the resulting behavior as soundness. The sharper, verified truth a
spike-2 planner needs:

**Spike-1 deferred two different things in one move, and only one of them was free.** `KNOBS kPRECISION`
says precision is "safe to trade — cutting precision costs probes/runs, never correctness, while `kFAIL`
holds." That justifies deferring the precision *mechanism* (strong/weak update, selectors): it is
low-lock-in and correctness-safe. It does **not** justify deferring the *fact-domain shape* the mechanism
needs — that piece is high-lock-in (`kCONTEXT` redline: the abstract domain must stay flat or k-CFA goes
EXPTIME; `kFACTS`: the substrate). Note `notes/quarantine-DO-NOT-READ/160` SF-1 said to build recency
*into the fact domain from the start*; spike-1 instead reserved `core::FactDomain`/`core::Fact` and wired
a flat boolean `(kind, entity)` key (`effect::Reach`, reaching-defs gen/kill only). So it deferred a cheap
mechanism (fine) *and* a retrofit-hostile shape (the real miss) together.

**The cost is not hypothetical — the spike cannot elide anything on its own only realistic book.** +SURE.
`plan::fixture_install_runs_despite_converged_probe` asserts that on `fixtures/pi-webhost.book.sh`,
`apt-get install -y nginx` is `Disposition::Run` **even when the host reports the fact `Converged`** —
because the un-oracled `apt-get update` above it is `Opaque` ⇒ poisons downstream ambient-ness ⇒ the
install is `EstablishWritten` ⇒ `plan::prove_replaceable` refuses it. The test's own comment names the
only fix: *"to recover the skip the oracle must model `apt-get update` as package-state-pure; the spike's
does not."* That is the precision layer's absence, exhibited on the dogfood fixture. Every elision spike-1
*does* demonstrate is on synthetic e2e books with the poisoning neighbor stripped out.

Implication for the invested question "was the spike well-run?": its core kernel and its review method
were well-run (§5), but it met its single most viability-determining question — *does this elide anything
real?* — by hitting a poison wall and pinning it as a regression test, rather than by building the layer
that answers it. That makes §2–3 below the actual point of a second spike.

---

## 2. What spike-2 should build

Two buckets, because "what wasn't exercised" splits into machinery that exists-but-is-dead and analysis
that is unbuilt-but-cheap.

### 2a. Built but never instantiated — turn it on

- **`q1-backward` — instantiate one real backward analysis and build apply-3.** +SURE, highest-leverage
  *free* exploration. `analysis::solve` is generic over `Direction{Forward, Backward}` and the whole
  `Must`/`BoundedLattice`/order-dual tower exists, but **no backward analysis and no must-analysis are
  instantiated** — the apparatus is exercised only by its own unit tests (`16P` T4, §3.2). apply-3
  (`dorc try`, the targeted desired-set mode DESIGN names under "General design principals") is apply-2
  plus a *backward relevance-reduction*: `apply-3 ⊃ apply-2` (`16P` T13). Building it is the honest
  load-test of the orientation machinery spike-1 built "as state-space exploration" (the
  `notes/quarantine-DO-NOT-READ/168` "calibrate-up" ruling) but never ran against a real consumer. If the
  `May`/`Must` superposition survives a real backward caller, that retroactively earns the locks; if it
  breaks, the locks were premature. Spike-1 produced neither verdict — only unexercised surface.

### 2b. Unbuilt, cheaply buildable, ranked by surfacing-value

- **`q1-precision` — the recency/selector layer (the §1 keystone).** +SURE, #1 by value. Build:
  strong/weak update; a per-entity selector (`installed` vs `version`; `svc#enabled` vs `#active`); the
  uniqueness/aliasing gate that licenses "same entity." Pure lattice+transfer work, fully DST-able.
  **Do not start this before settling `dq-entity-algebra` (§3) on paper** — the domain shape it needs is
  retrofit-hostile.
- **`q1-probe-projection` — the unbuilt half of the round-trip, and underrated.** +SURE. Spike-1's probe
  emits the oracle body with `$1` unbound (`16P` T16, "illustrative"); there is no per-entity binding and
  no separate probe plan-builder (`16P` T11 calls the probe plan-builder "the honest forcing-point …
  deferred"). This is the *only* place `inv-superposition` ever acquires a real **second caller** —
  spike-1 had one consumer (apply) standing in as phase-agnostic. A probe plan-builder reveals whether
  "engine emits, caller collapses" holds with two real phased callers, or was a one-caller fiction.
- **`q1-interproc` — intra-file call/return edges + literal `. /path` source-following + trap handlers.**
  ~SUSPECT high. Spike-1 treats every function body as a *detached region* (seeded errexit-`Off`, forced
  `MustRun` by the entry-reachability gate — `16P` T8/DP-8); `. /path` parses as a plain command and its
  effects are not followed (`16P` §3.2). Building Tier-B-lite forces the IFDS/summary question into the
  open and is the natural pressure-test of `dq-substrate` (§3).
- **`q1-envstate` — general shell-env-state through the *generic* `solve`.** -GUESS medium. Spike-1
  modeled only errexit, and as a *bespoke* forward pass inside `cfg::build`, not the `Product`-lattice
  `solve` that `notes/quarantine-DO-NOT-READ/163` sketched (`16P` T9). Adding `cwd`/`pipefail`/`ifs` as
  generic-engine analyses tests whether the engine actually *composes* multiple env-facts — or whether
  errexit was hand-rolled because the generic path does not really work.
- **`q1-flaggrammar` — per-provider flag grammars vs the coarse operand rule.** -GUESS lower, but cheap.
  Spike-1 uses a single-literal-non-flag-operand rule; pre-verb flags / multi-entity / non-literal all ⇒
  `MustRun` (`16P` T7, §3.2). Modeling `apt-get`/`systemctl`/`ufw` grammars makes the sound-XOR-useful
  tension (`DP-5`, `kBURDEN`) concrete instead of abstract.

---

## 3. What to decide before spike-2 (retrofit-hostile — settle on paper first)

These gate §2 and are high-`kLOCKIN`; spike-1 made progress precisely by *not* committing to them, which
is exactly why they keep getting deferred.

- **`dq-entity-algebra` — what *is* an entity? The most-deferred decision in the program.** +SURE. The
  keystone under `q1-precision`, sitting at the intersection of `kPRECISION` (mechanism, low-lock),
  `kCONTEXT` (flat-domain redline, **high-lock**), and `kFACTS` (substrate, **high-lock**). Reserved as
  `core::FactDomain`/`core::Fact` but unwired since spike-1, and flagged "build from the start" by
  `notes/quarantine-DO-NOT-READ/160` SF-1 and the `055` reference design (§1B, "the keystone precision
  lever," TAJS 87%→<2% without it). Answer before coding: is an entity flat (`package:nginx`) or
  structured (`package:nginx{installed,version,held}`)? What licenses a *strong* (overwriting — requires
  proving the entity a unique singleton) vs *weak* (accumulating) update? Wrong shape re-keys every
  transfer function and the substrate.
- **`dq-substrate` — `KNOBS kFACTS`, explicitly "the engine-substrate decision."** ~SUSPECT
  under-pre-thought. Hand-rolled monotone worklist (spike-1) vs IFDS/IDE demand vs Datalog/Soufflé
  materialized; the `055` substrate `[DECISION]` stays open. `q1-interproc` will *force* it (does the
  worklist scale to the supergraph, or do you need realizable-path summaries?). The undercounted coupling:
  the *provenance/why-tree* query model. `kFIDELITY` (high-lock) commits to a per-host N-tier locator DAG
  (`notes/quarantine-DO-NOT-READ` rounds 110/111); Datalog yields queryable provenance ~free, the worklist
  makes you hand-build it. Decide the *query model* before `q1-interproc`, or hand-roll a provenance layer
  you then discard.
- **`dq-reflexive-probe-inertness` — the undercounted one, because it is mis-filed.** ~SUSPECT. `DP-4`
  ("the probe's read-only-ness / `kFAIL-withhold` is not enforceable by the contract frame") was
  categorized as *needs-a-runtime-sandbox*, and the **static** option was dropped: the analyzer is
  *already an effect-analyzer* — point it at the lifted probe body and have it *certify* the probe touches
  no modeled mutation, refusing to ship a probe it cannot prove inert. Spike-1 lifts the oracle statically
  but never effect-analyzes the probe body (the oracle lift only diagnoses a *declared* top-level mutator,
  not the probe's own reachable effects). Bound honestly: this certifies only against *modeled* effects —
  an unknown tool inside a probe still needs the sandbox — so it is a cheap *first layer*, not a `DP-4`
  replacement. It serves DESIGN's #1 promise ("plan stage doesn't mutate") with machinery already built,
  and was undercounted because the reflex was to reach for seccomp.
- **`dq-kOOB` — the one *decision* owed, not an exploration.** Is `oracle_kind=package` legitimate in-band
  sh, or "config-in-disguise" the `kOOB-in-band` lean forbids? Spike-1 held the *entire oracle sh
  spelling* as a strawman pending this ruling (`16P` §5, "Open human ruling"); it blocks firming the
  oracle contract, so any `q1` touching oracle syntax stalls until it is ruled. `KNOBS kOOB` is
  `directional`; this wants to become `welded`.

---

## 4. The one design-conclusion to enshrine now

**Carry the fact-centric, named-kind oracle anchor into `DESIGN.md`'s "Inference limitations" section** —
the `wombat`/`hork` passage that today dead-ends on `(UNSETTLED, CONTINUE)`.

DESIGN walks up to this and stops: it states the chicken-and-egg ("to infer `wombat` is trackable state
you'd need to know `hork` is a probe; to infer `hork` is a probe you'd need to know `wombat` is a
K-entity") and concludes only "cross-oracle compatibility will *have* to be anchored with some sort of
grounded anchor. A type-declaration, effectively." Spike-1's most load-bearing, most-adversarially-tested
finding (`16P` T7 / `DP-1`) *is* the continuation, and it carries a correction that took an adversarial
refutation to find and that nothing downstream will re-derive: the anchor must be **fact-centric, not
command-centric** — probe a *fact* ("does `kind:entity` hold"), never dry-run the mutator. (The first
command-centric strawman, `notes/quarantine-DO-NOT-READ/161`, was refuted in
`notes/quarantine-DO-NOT-READ/162` because the named-kind index was *decorative* in its skip path.)

The settled shape to write down (and only the shape): cross-oracle identity binds to a **named kind**,
never a shared token; the relation is **3-place** `(kind, provider, verb) → effect`, not a 1-place naming
convention; the oracle is **plain sh the analyzer lifts statically**, never sources or runs. Carry the
*shape*, not the spelling — the exact `oracle_kind=` syntax stays out, explicitly fenced as pending
`dq-kOOB`. The edit fills a labeled hole with the settled half and leaves one clean pointer to the open
ruling.

**Runner-up (do this too, but it is second):** the explicit elision predicate — `elide iff
probe=Converged ∧ ambient ∧ Must ∧ no-consumed-unvouched-observable ∧ ¬⊤-contained`, plus
`can't-probe ⇒ can't-elide` (`16P` T13) — into `KNOBS kELISION` (which only half-distinguishes the two
elision kinds today) and as a sharpening of DESIGN's apply-phase paragraph (which blurs *already-correct*
vs *not-relevant-now* in one sentence; `KNOBS kELISION` keeps them distinct). It loses the top slot
because it is a mechanical contract `kFAIL` drags the implementer back toward anyway, and it sits lower in
the design tree than the symbol-grounding anchor — which shapes the oracle contract DESIGN itself calls
"what makes our product, more than anything else."

*Selection note:* this is drawn from the **settled** findings, not §3's open questions — answers belong in
ground-truth docs, questions stay in `plans/`/`KNOBS`. The most valuable *open* item,
`dq-entity-algebra`, deliberately stays a flag; writing an unsettled keystone into DESIGN as if decided is
exactly the contamination `16P` built its whole apparatus to prevent.

---

## 5. How to run the next throwaway-spike better (process)

The round-16 spike is a usable data point for the broader practice of *writing disposable code with an LLM
to refine a design*. The keepers below are validated by this spike; the anti-patterns are ones it
exemplifies. This section is heavier than the rest on purpose — the meta-process is the most transferable
output, and the cheapest thing to get wrong on the next one.

### 5a. Keep these — they earned it

- **`kp-1` — adversarial-crosscheck on *just-landed* code, un-seeded from the author's notes;
  convergence-as-signal; trace-don't-relay.** The single most reusable artifact (`16P` T17). It caught a
  convergent soundness kill-shot in code landed one note earlier
  (`notes/quarantine-DO-NOT-READ/16I`), two wrong-skips (`notes/quarantine-DO-NOT-READ/167`), and the
  command-centric `dn-1` error (`notes/quarantine-DO-NOT-READ/162`); the framework's infinite-loop was
  *empirically reproduced* by the adversarial scratch (`notes/quarantine-DO-NOT-READ/164`: 435 & 783
  CPU-seconds before kill), not merely argued. Un-seeding the reviewer from the author's own notes is what
  bought the `16I` catch — denied the author's rationalizations, the pair could not echo the blind spot.
- **`kp-2` — execution earns its keep at the author-belief vs author-implementation gap.** Both top
  findings (the hang; the leaf-local observable-gate bypass) live in the delta between what the author
  thought the code did and what it did — invisible to design discussion, visible only by running code
  against a hostile reader. This is *why* the throwaway is worth writing at all.
- **`kp-3` — make the one catastrophic action structurally unspellable, and use the throwaway to test
  whether the invariant is *livable*.** The private-field witness (`plan::ReplaceLicense`, mintable only
  by `plan::prove_replaceable`) concentrates the irreversible decision in one reviewable place; *building*
  it surfaced `build_plan never consults diagnostics` (a `⊤`-containment breach at the plan layer) — a
  hole that only appears once you try to *use* the witness in an emitter. Disposable code is for
  validating that a proposed invariant is ergonomic, not only for finding bugs.
- **`kp-4` — re-read the primary `README`/`DESIGN`/`KNOBS` after any context compaction; trust
  AI-authored working notes (and even `inv-*` slugs) *below* first-party docs.** `notes/quarantine-DO-NOT-READ/16J`
  is the worked failure: a compaction dropped `DESIGN`/`KNOBS`, the agent reasoned from its own notes plus
  an analogy, manufactured an "open question," and re-reading the primary docs answered it outright. (This
  doc applied `kp-4` to itself before §4 — the `DESIGN`/`KNOBS` claims here were re-read from source, not
  taken from `16P`'s characterization of them.)

### 5b. Avoid these — this spike exemplifies them

- **`ap-1` — building the scaffolding for the deferred hard part instead of the hard part.** Spike-1 built
  the full `May`/`Must`/`Backward`/`BoundedLattice` apparatus (for an apply-3 that was deferred) but not
  the recency layer (the keystone `055`/`160` both flagged as viability-determining). Type-machinery is
  cheap and *feels* like progress to an LLM; the load-bearing analysis is hard and got cited away — and a
  human "calibrate-up, lean into the locks" ruling actively rewarded the cheap direction. Lesson: gate a
  "surface the hard problems by building" spike on building the reference design's *named keystone first*,
  before any ergonomic type-machinery.
- **`ap-2` — a harness that validates the wrong invariant, green.** +SURE, and the second (minor)
  correction to the `16P` record. The sh e2e (`notes/quarantine-DO-NOT-READ/spike/e2e/run.sh`)
  golden-diffs stdout *text* (`[ "$got" = "$want" ]`) and never executes or even `sh -n`-checks the
  rendered artifact. So `plan::render_apply` emitting **non-runnable POSIX** ships green: the `guarded`
  showcase case renders `if true; then` / `#    apt-get install … # dorc: elided` / `fi` — a `then`-clause
  whose only content is a comment, which is a syntax error (`fi` where a command list is required). `16P`
  T14 cites that very case as proof `render_apply` "preserves control-flow structure"; it preserves the
  *text* of the structure while producing shell that does not run, and the harness is structurally blind
  to it. For a tool whose entire contract is "the output is just shell you can run," the acceptance test
  must *execute or `-n`-check* the generated artifact, not string-compare it. (The defect itself is minor
  — throwaway sketch code explicitly fenced `NOT BUILT: faithful control-flow rewrite`; the *process*
  signal is the valuable part.)
- **`ap-3` — an adversarial pass finds only what it is aimed at; vary the target, not just the cadence.**
  The sharpest process lesson here. Spike-1 ran `kp-1` repeatedly and well — but always aimed at the
  *analyzer core's soundness*. It never aimed a pass at "is the e2e testing the right invariant?" or "does
  the showcase output actually run?" or "did we build the keystone or scaffold around it?", so `ap-2`'s
  defect survived multiple adversarial rounds untouched. Running the pattern is not enough; rotate its
  *target* across the core, the harness, the synthesis, and charter-adherence, or it deepens coverage of
  one face while the others rot. (This doc is `kp-1` aimed one level up — at `16P` and the spike's
  choices; the two things it found that the spike's own passes missed, §1 and `ap-2`, are exactly the
  targets no in-spike pass was pointed at. The lesson is recursive.)
- **`ap-4` — synthesis ceremony scaling with effort spent, not questions closed.** The wader→writer split,
  the workdir trinity, the writer-brief, the non-resolving-citation scheme — substantial machinery for
  disposable code, whose output (the postmortem) is largely a re-prose of one workdir file. The
  neutrality-by-fresh-context goal was real and the durable distilled doc earns its keep, but it could
  have been met with roughly one fewer layer. Budget meta-process against design questions *closed*, not
  against the spike's line-count; a disposable spike does not need a multi-stage publication pipeline.

---

*This is the final `16x` document. The round-16 notes and spike code remain quarantined
(`notes/quarantine-DO-NOT-READ/`); reach last-mile evidence through `16P`'s citations and this doc's, and
do not pull the quarantine back in wholesale.*
