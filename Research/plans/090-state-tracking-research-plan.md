# Round 9 — state-tracking: Phase-0 research plan (the reviewable charter)

> **Status (2026-06-02): Phase-0 map, awaiting human review.** This is the
> *interactive-research* skill's `plan.md` for round 9, sited in the repo's
> `Research/plans/` convention. It reifies the **state-tracking** topic (TODO
> line 3: "design-pass on *state*-tracing … models keep getting confused and
> thinking about *variable-closure*, whereas we need to track *state-closure*"),
> maps the prior-art domains + correct terminology, and proposes the deep-read
> fronts. Per-front raw findings will land in `Research/notes/091+`; the round-9
> synthesis in a later `Research/plans/09x`. Design tensions by `KNOBS.md` slug;
> confidence markers (`+SURE/~SUSPECT/-GUESS/--WONDER`) throughout. **Nothing here
> is graded-and-read yet** — sources are *candidates* with provisional grades, to
> be acquired + graded in the fronts (the existing `sources.json` A-corpus is
> reused where it already covers a domain).
>
> ⟢ **DOC-ROLE (2026-06-03):** this is the round-9 **research charter** (the `interactive-research` `plan.md`) — its domain-map (§2) and constraints/hypotheses `P1`–`P6` (§3) are *provisional, deliberately un-graded candidates*, **not findings**. The round-9 **conclusions** are `099-state-tracking-synthesis.md` (+ companion `09A-specimen-grounding-synthesis.md`). Don't cite this doc's framing as a result.

## ⚑ Findings-vs-stated-positions watch (per human direction, 2026-06-02)

The human is *extremely* judicious with DESIGN.md and owns that synthesis over time. **Do NOT
accumulate DESIGN-change-suggestions here** — they won't land, and the durable findings already
live in `notes/09x` (the human may *link* some from DESIGN later). The one signal worth surfacing
— *immediately, in chat* — is a finding that **disagrees with something the human has stated** (in
chat or DESIGN); that is the trigger to revisit. **Status so far: no contradictions — findings are
*validating* (F3 cross-camp), at most *refining* a chat hunch** (e.g. "convergence = one end of a
spectrum?" → sharpened to *a neighbouring axis*: method vs outcome-quality; Dorc =
idempotence-not-convergence). Watch continues per front.

## 0. The problem, reified (what "state-closure" actually means)

The motivating example (TODO + DESIGN probe/apply model): a Dorc script does
`do_x; … if x; then do_y; fi … ; undo_x`, where `x` is system state the script
*itself* mutates.

// HUMAN'd:
*One version* of this class of hazard, stated precisely:

- The **probe** observes the target at its **resting / quiescent state** (probe-time
  `T₀`), and is sanitized read-only + massively-parallelized (DESIGN "probing phase").
- The **apply** walks the system through a **trajectory** of intermediate states.
  A guard reads state *at its own position along that trajectory*.
- `do_x` establishes `X`; `if x` reads `X` (true here, mid-trajectory); `undo_x`
  restores `¬X`. At rest (`T₀`, and again at end) `X` is **false**. So a global
  probe that hoists `if x` to `T₀` computes the guard as *false* and **elides
  `do_y`** — but `do_y` was needed every run. That is a **wrong skip**:
  an **elision-soundness** violation (`kFAIL-perform`), the one thing never traded.

// HUMAN'd: ------
There are, however, other, related classes that aren't precisely the same shape; this is about the general questions of:
1. Dorc having to do a control-flow/state-flow analysis of the remote system's
  prospective-state-during-apply, and tracking *taint* of that control-graph by
  various state-facts; then
2. how/when/where to communicate with a (cooperating, non-lazy,
  extra-effort-doing user) about both A. contributors-to and B. dependants-on
  that state-graph; and finally
3. how to best-effort *recover*-from / extract-maximum-value-for users who *are*
  lazy, non-cooperating, or worst-case. (in the above case, what if we see
  *literal* `do_y`. We know nothing about that, have no oracle. What can we
  infer, what can we do, how do we fail-happy as much as possible during the
  gradual-enhancement case.)
// --------

+SURE the crux is a **typing of system-state facts by temporal stability**, not a
control-flow nicety:

- **Ambient / persistent** facts have a stable resting value → **probeable** at `T₀`
  ("is nginx installed?", "does `/etc/foo` exist?").
- **Transient / ephemeral** facts are *created and destroyed within a single run* →
  they have **no resting value** → **fundamentally un-probeable** (the `X` above).

A guard is **soundly hoistable** to the probe *iff* the fact it reads is **ambient
AND invariant** from `T₀` to the guard's execution point — i.e. (a) no in-script
`gen`/`kill` of that fact reaches the guard, **and** (b) no concurrent/external
mutation can (the hermeticity + TOCTOU condition). **Transient facts must be
tracked by pure in-script dataflow** (the over-approximating apply analysis),
*never* by the probe. Misclassifying a transient as ambient = the wrong skip.

This is the human's **variable-closure vs state-closure** distinction made formal:
*variable-closure* (SSA/dataflow over shell `$vars`) is compiler-local and easy;
**state-closure** is the transitive closure of *system-state* facts that the
script's own oracle-effects `gen`/`kill` and that its guards `read` — computed over
an **abstract** store that **IS the remote system**, not the program's variable
environment. The TODO's "CFG must be retained in the probe" is exactly: *a
transient-dependent guard is non-hoistable, so its control-flow stays in the
shipped probe* (`kFLATTEN-maintain-cfg`).

## 0.5 The governing frame (human-set; threaded through everything below)

The §0 inserts re-center the round. +SURE the spine:

**Unsolvable by analysis alone — and we are not trying.** Static analysis cannot
answer "does `do_y` *need* `x`, or is `do_y` incidental to other reasons `x` was
manipulated?" — still less "does the *probe/dry-run* version of `do_y` need `x`?".
Those are questions about *intent* that no sound analysis of opaque shell decides.
So the user is *necessarily* in the loop; the round maps **how**, it does not pretend
the engine goes it alone. (Guard against the PLT literature here: it overwhelmingly
pursues *proving* soundness/hermeticity — but `kVERIFY` is welded to
*calibrate-not-prove*. Mine those papers for **vocabulary + design-constraints**,
never for proof-machinery we'll never run. The human's worry — "ALL THE SCIENCE AT
ONCE" / model-induced over-connection — is a standing instruction to be
*discriminating*: for each source say plainly whether it is **load-bearing**,
**vocabulary-only**, or **a false friend to drop**.)

**Promote "shared state" as the canonical phrase** (human directive). It reads
cleanly in *every* domain in play — PLT (shared mutable state), databases (shared
data / concurrency control), ops (shared system state), build systems (shared file
state), CPU pipelines (shared registers → hazards). The recurring hazards (TOCTOU,
RAW/WAR/WAW, write-skew) are all *shared-state interference* hazards. Lead with it.

**Convergence is the foil/floor, not the model** (human correction). Burgess/cfengine
*convergence* = re-apply until a fixed point, tolerating not-knowing current state by
being idempotent. Dorc instead wants a **single correct execution** derived from
analysis + probing — explicitly *avoiding* implicit attainment-by-convergence. But
DESIGN reserves "trivial convergence" (just re-run) as the *fallback floor*. So they
are two ends of one spectrum, and that spectrum *is* the value axis:

- **Floor** = plain linear shell execution / trivial convergence — zero declaration,
  zero inferred value-add, *fail-happy* default. Never worse than not using Dorc.
- **Ceiling** = single, minimal, correct execution — maximal *sound* skip, full
  knowledge of the shared-state graph.
- **Frontier** = the pareto curve between, climbed by two engines: *what we infer
  from what the user already does* + *what the user chooses to declare*.

**The three bound-questions = the round's deliverable shape.** Every finding is
placed against:
- **q-floor** — what must the user declare under *absolutely all* circumstances for
  us to be sound at all? (the irreducible mandatory contract)
- **q-frontier** — the pareto frontier of *fail-to-declare → best-job-we-still-do*
  (graceful degradation; the literal-`do_y`-no-oracle case lives here).
- **q-ceiling** — given unlimited engineering + tooling + smartest *sound* analysis,
  the *upper bound* on work we can save. (bounds the value-prop)

**The segregation quadrant — hold at all times; pass into every subagent.** The
literature conflates two things we must keep apart, *and* mixes two axes of
who-settles-it-when. Classify every finding on both:

- **knob vs contract.** A *knob* is an A-vs-B design tension we tune (`KNOBS.md`). A
  *contract* is an obligation on an author (oracle-author or ops-author) that, if kept,
  we *rely on* for soundness. The command-frame is the trap: it is *both* a knob
  (infer ↔ declare) *and*, on its declared side, a contract (what the oracle promises
  it touches). Segregate the facets; don't let a paper's single fused notion blur them.
- **weld-now vs user-adjusts-but-we-design-how.** Orthogonal. Some things we *weld* at
  design-time (settle + bake in — `kFAIL`, `kVOLATILES`, `kVERIFY`). Some the *user
  adjusts* at runtime/config, but *we* must design the *mechanism/shape* now (the
  gradient, the mode) without picking the value — `kBURDEN` (the user "designs the
  gradient"), `kELISION` (mode). "Open" = *we haven't yet decided which of these two it
  is*; that decision is itself round-output.

This 2×2 (knob/contract × weld/user-adjusts), keyed to q-floor/frontier/ceiling, is
the frame every finding sorts into — the design-space-narrowing deliverable (q-scope
(a) is confirmed; §6): walls, knobs, contracts, and where each is owned — *not* a
resolved mechanism.

**The contract notation does not exist yet (human note).** There is today *no*
written Dorc contract — no syntax, no pragma, no oracle-declaration form. Designing
it is downstream (language-design + UX, plus frank "market-vibes"), likely not this
session — and this round is *the first/core component of deciding how/what to
contract*. What it produces — the gradient, the knob, the floor/frontier/ceiling, the
quadrant — *is the state-space that the eventual contract-design explores*. Concretely,
every contract reduces to a triple: **(A) what** semantic promise we attach
("treat-as-don't-hoist", "this-depends-on `<that>`", "ambient/hermetic", …) · **(B)
who-type** declares it (oracle-author vs ops-author) · **(C) where** it lives in/around
POSIX sh (the `kOOB` in-band↔sidecar surface). q-floor/q-frontier feed (A)+(B); `kOOB`
owns (C). Phrase every finding so it drops cleanly into that `{what, who-type, where}`
triple — that is the shape the design-space hands forward.

### Subagent preamble (paste into any gathering/grading subagent for this round)
> Gathering for Dorc's *state-tracking* round. (1) Dorc does **not** *prove*
> soundness — it *calibrates* (tests) and delegates inertness/hermeticity to
> user-authored oracles; do **not** bring back proof-machinery as if we'll run it —
> mine for **vocabulary + design-constraints**. (2) Canonical phrase: **shared state**.
> (3) For every finding tag two axes — **knob vs contract**, and **weld-now vs
> user-adjusts-we-design-how** — and say which of **q-floor / q-frontier / q-ceiling**
> it bears on. (4) Return **verbatim excerpts + citations**, never conclusions;
> self-label sources `graded-by: subagent`. (5) Beware over-connection: per source,
> state whether it is *load-bearing*, *vocabulary-only*, or *a false friend to drop*.

## 1. Where this sits in the existing design (no new knobs yet)

The topic lands on knobs already named — it does not (yet) propose a new one:
- **`kFLATTEN`** (`hoist ↔ maintain-cfg`) — state-closure *is* the hoist-safety
  predicate. Transient-dependent guard ⇒ cannot hoist ⇒ `maintain-cfg`. +SURE.
- **`kPROBING`** (`probe-first ↔ just-run`) — transient facts are un-probeable, so
  they force `just-run` (or in-script reasoning), independent of cost. ~SUSPECT.
- **`kFAIL`** (welded) — the whole point: the wrong skip is the `kFAIL-perform`
  violation this round exists to prevent. The probe-soundness twin (`kFAIL-withhold`)
  also bites: you cannot "probe a transient by running `do_x`" — that would mutate.
- **`kVOLATILES`** (welded → exclude) — *related but distinct*. `kVOLATILES` is
  *non-deterministic* state (clock/`$RANDOM`/network); **transient** state is
  *deterministic but unstable-at-rest*. ~SUSPECT they need disentangling — a
  transient fact can be perfectly hermetic yet still un-probeable. Front F3/F6.
- **`kBURDEN`** (`we-infer ↔ user-declares`) + **`kOOB`** — the *open knob* the
  prompt named ("contracts on both ends, both authors"): does the **oracle-author
  declare** each command's effect-frame (modifies-clause style) or do we **infer**
  it (bi-abduction style)? See §3 front F2.

## 2. Prior-art map — the eight domains + the correct terminology

The single highest-value Phase-0 output: this problem has been named and attacked
in *at least* eight literatures, none of which the planning log has connected to
state-tracking yet. ~SUSPECT this cross-domain net is most of the round's value —
*but* under the §0.5 anti-over-connection guard, here is my honest first cut of
which actually pay (to be tested, not trusted):

- **Load-bearing** (pursue): **D3 ops-theory** (Traugott/Burgess — our users' own
  theory of self-modifying state + the convergence spectrum; prose, low minefield);
  **D7 build-hazards/Rattle** (speculate-then-fall-back + the hazard taxonomy;
  empirical, in-corpus, low minefield); **D4 TOCTOU** (names the probe→apply gap;
  a bug-class framing, not a proof framework).
- **Vocabulary-only** (mine the terms, do *not* chase the formalism — the minefield
  the human flagged): **D1 sequential effects** (the *ordered-effect* concept + do/undo
  as an effect-with-inverse; *not* the quantale proofs); **D2 frame/separation**
  (footprint + infer-vs-declare as the contract shape; *not* the separation-logic
  proving); **D6 sagas** (compensation / net-zero-ambient vocabulary); **D8
  strong/weak-update** (already internal via TAJS; refinement, not new).
- **Scrutinize, likely-demote** (a possible false friend): **D5 OCC/snapshot**. The
  *read→validate→write* ≈ *probe→plan→apply* isomorphism is seductive; the part that
  may genuinely pay is the **validation-phase** question (re-assert read-set before
  acting); the rest (aborts, atomicity, serializability proofs) is analogy. Keep the
  one question, drop the machinery unless F4 earns it.

- **D1 — Sequential effect systems.** Order-*sensitive* effect algebra. Foundational
  [A-lucassen-gifford-effect-systems-popl-1988, in-corpus] is *commutative*; the
  order-aware generalization is Gordon's **effect quantale**
  [cand. A-gordon-effect-quantale-2021, arXiv 1808.02010 / TOPLAS 3450272].
  → the exact algebra for composing oracle effects *in sequence*; `do_x;do_y` ≠
  `do_y;do_x`. ~SUSPECT this is the type-theoretic spine for the ambient/transient
  typing and for "what restores what."
- **D2 — Frame problem / separation logic / footprints.** The **frame rule**
  `{P}C{Q} ⊢ {P∗R}C{Q∗R}` — `R` is the untouched **frame**; `C`'s footprint is what
  it touches. An oracle's effect-set *is* its footprint; everything else is framed
  (invariant across it). **Inference** side = **bi-abduction**
  [A-biabduction-popl-2009, in-corpus] (Infer infers footprints). **Declaration**
  side = **modifies-clauses / frame conditions / dynamic frames** (JML, Dafny,
  Kassios) [cand. C/B-toctou-and-frames sources]. +SURE this domain *is* the open
  knob (`kBURDEN`): infer the frame vs declare it.
- **D3 — Config-management state theory (the ops-native prior art).** Traugott,
  **"Why Order Matters: Turing Equivalence in Automated System Administration"**
  (LISA 2002) [cand. A-traugott-order-matters-2002, infrastructures.org] —
  *self-modifying* admin actions ⇒ "self-referential chaos" ⇒ ordering is
  load-bearing ⇒ Turing-equivalent in general. **This is the do_x/undo_x problem,
  named in the ops domain 24 years ago.** Burgess **promise theory / convergence**
  [cand. B-burgess-*] — *convergence* (repeated application → fixed point) is a
  *stronger* property than idempotence and is cfengine's basis; the DESIGN
  "not convergence" link is Burgess-adjacent. +SURE worth a front; these are *our
  users' own theory*.
- **D4 — TOCTOU (time-of-check-to-time-of-use).** The **probe→apply gap is a TOCTOU
  window** [cand. B-cwe-367-toctou]: Dorc checks at probe-time, acts at apply-time;
  between them state can change (the script's own upstream effects, a concurrent
  admin, an attacker). Security's fix — "anchor the use to a handle from check-time,
  or make check+use atomic" — is the soundness discipline for *every hoisted guard*.
  +SURE this reframes hoisting as "a TOCTOU bet; only ambient+hermetic+unraced
  facts are safe bets."
- **D5 — Optimistic concurrency control / snapshot isolation.** **Structural
  isomorphism**: OCC = *read phase → validation phase → write phase*
  [cand. A-kung-robinson-occ-1981]; Dorc = *probe → plan → apply*. OCC's validation
  re-checks the **read-set** wasn't invalidated before committing writes — exactly
  the missing step that would make hoisting sound across the gap. **Write-skew**
  (snapshot isolation) = the *fleet* hazard: independently-validated per-host probes
  can be *jointly* wrong if cross-host state exists. +SURE this hands us a
  *validation-phase* design.
- **D6 — Sagas / compensating transactions.** `do_x…undo_x` is literally a **saga**
  with a **compensating action** (Garcia-Molina & Salem, "Sagas", SIGMOD 1987, not acquired;
  + the Azure compensating-transaction pattern). Vocabulary: forward/pivot action + compensation;
  a compensated pair has **net-zero ambient effect** but a **non-trivial transient
  trajectory**. ~SUSPECT recognizing balanced pairs is a *precision* lever (skip
  probing the transient; maybe elide the pair) — soundly only if provably balanced.
- **D7 — Build-system hazards / forward-build speculation.** Rattle borrows the **CPU
  pipeline hazard taxonomy** (RAW / WAR / WAW + *speculative write-before-read*) for
  *commands over shared file state* [A-spall-mitchell-rattle-perfect-dependencies-2020]
  (in-corpus; its hazard-formalization sequel — Spall & Mitchell, "Forward Build Systems,
  Formally", CPP 2022 — not acquired).
  Dorc's hoist/elide **is speculation**; Rattle's discipline — *speculate, detect a
  hazard, fall back to non-speculative re-run* — is a directly transferable backstop,
  and matches DESIGN's "always reserve the right to fail back to multiple executions
  / trivial convergence." +SURE the hazard taxonomy names *which* reorderings
  state-closure forbids.
- **D8 — Abstract store / strong-weak update.** The "heap" is system state; **strong
  update** = "this exact fact is now precisely V", **weak update** = "may also be V";
  **recency/singleton** abstraction enables strong-update on a freshly-touched fact
  [A-jensen-moller-tajs-…-sas-2009, in-corpus; cand. B-dillig-fluid-updates-2010].
  Already noted internally as the precision keystone (note 055 §1B); the *new* use is
  that **transient state is exactly where strong-update + must-restore reasoning
  lives**. ~SUSPECT.

## 3. Design-space constraints (walls / knob-candidates / contract-candidates)

Recast per §0.5: these are not "directions to resolve" but *constraints to place on
the map*, each to be tagged knob-vs-contract × weld-vs-user-adjusts and bound to
q-floor/frontier/ceiling. Marked low-confidence on purpose — the fronts test them.
Provisional quadrant tags in brackets.

- **[contract · q-floor]** The **ambient/transient typing of facts** is the wall:
  transient (no resting value) ⇒ un-probeable ⇒ its guard is non-hoistable. The
  *floor contract* candidate is whatever minimal declaration lets us tell transient
  from ambient when inference can't (P4 below).
- **[knob · user-adjusts]** Hoisting (`kFLATTEN`) and probe-vs-just-run (`kPROBING`)
  are tuned *downstream* of the typing; the typing decides legality, the knob decides
  worth-it.

- **P1 (ambient/transient partition is a first-class analysis output).** ~SUSPECT.
  It gates `kFLATTEN` (hoist) and `kPROBING` (probe). Transient ⇒ never probe, never
  hoist its guard, retain CFG. This is the precise mechanizable content of the TODO.
- **P2 (state-closure = reaching-definitions over the system-state store).** ~SUSPECT.
  Oracle `gen`/`kill` (= frame/footprint per command) are the transfer functions;
  a guard on fact `F` is hoistable iff *no in-script def of `F` reaches it*. The
  IFDS gen/kill machinery already exists (note 052/055); the *new* parts are
  (a) naming the store as system-state and (b) the ambient/transient **typing** of
  facts. The data-structure is the existing fact-domain (note 053), re-purposed.
- **P3 (the probe→apply gap needs an OCC-style validation, or hoisting restricted to
  hermetic-ambient).** -GUESS on *which*. Options: (a) atomicity (defeats parallel
  probe), (b) re-assert hoisted read-set facts cheaply at apply (OCC validation),
  (c) only ever hoist contractually-stable hermetic-ambient facts. Pessimistic
  default: (c), with (b) as the precision recovery. Front F4 sizes it.
- **P4 (the open knob = frame-declaration: infer vs declare).** ~SUSPECT. Soundness
  needs *zero* author burden (don't-hoist-when-unsure is sound and free); *precision*
  (recovered hoists/skips) is what oracle-author frame-declarations buy. The knob is
  the precision-per-burden curve, not a soundness lever. Front F2.
- **P5 (compensation-awareness is precision, not soundness).** -GUESS. Balanced
  do/undo ⇒ skip probing the transient; unbalanced/conditional ⇒ ⊤ ⇒ retain+run.
- **P6 (speculate + hazard-fallback as the execution backstop).** ~SUSPECT. Dorc has
  it *easier* than Rattle: the read-only probe can't cause real hazards; only apply
  can, and idempotence absorbs over-runs. So Dorc's "fallback" is cheaper than Rattle's.

## 4. Proposed fronts (the deep-read pass — Phase 1+)

Re-ordered by *value × (low minefield-risk)* per §0.5, not by domain: lead with the
ops-native, prose-heavy, proof-light sources; treat the PLT-formalism fronts as
vocabulary raids. **Execution order: F3 → F2 → F5 → F1 → F4 → F6.** Each front's
output is not a summary but a set of **quadrant-tagged constraints** (knob/contract ×
weld/adjusts) bound to q-floor/frontier/ceiling — including the *literal-`do_y`-no-
oracle* graceful-degradation case, which every front should be asked to stress.

- **F1 — Sequential effects algebra.** Read Gordon's effect-quantale; ask: does the
  quantale give the exact ordered-composition algebra for oracle effects, and does any
  published instance model **restorable/transient** effects (a `do`/`undo` element with
  an identity)? Connect to D8 strong-update.
- **F2 — Frame: infer vs declare (the open knob).** Re-read [A-biabduction-popl-2009]
  for the *infer* side; acquire a modifies-clause/dynamic-frames source for the
  *declare* side. Deliverable: the **minimum the oracle-author must declare for
  soundness** vs what inference can recover, mapped onto `kBURDEN`/`kOOB`.
- **F3 — Ops-domain state theory.** Read Traugott (full) + a Burgess
  convergence/promise artifact. Ask: does the ops literature already have a *name* and
  a *solution-shape* for self-modifying state, and does its idempotence/convergence/
  congruence trichotomy refine `kVOLATILES` vs the transient distinction?
- **F4 — OCC / TOCTOU validation phasing.** Read [cand. A-kung-robinson-occ-1981];
  decide whether Dorc adopts an explicit **validation step** between probe and apply,
  and size its cost. Write-skew → the cross-host/fleet hazard (couples to `kSTATE`,
  `kCONTEXT`).
- **F5 — Build hazards as the reorder-legality oracle.** Read
  [cand. A-spall-mitchell-rattle-formally-2022]; map each hazard (RAW/WAR/WAW/
  speculative-WbR) to a state-closure forbidden-reordering, and adopt
  speculate-then-fallback as the execution backstop.
- **F6 — CoLiS's actual state/ordering treatment.** Re-read the in-corpus CoLiS
  papers *specifically for ordering / installation-scenarios* — they symbolically
  execute over feature-tree FS state; do they hit the do/undo / self-modifying case,
  and how? (Closest applied precedent; cheap, already local.)

### Status + re-prioritization (2026-06-02)
Done: F3 (notes 091), F2-corrected (note 092), and the impossibility dive (note 093,
the "one more dive"). Deferred as low-marginal (revisit only if synthesis shows a gap):
F1 (sequential-effects — vocabulary-only, the quantale formalism is the proving-minefield),
F4 (OCC — the flagged false-friend), F5 (Rattle — in-corpus, hazard-taxonomy already mapped),
F6 (CoLiS — in-corpus, decidable-fragment subsumed by the IFDS floor f20). **Two NEW
priority fronts now lead, then synthesis. Also owed: full-reads + re-grades of the four
abstract-graded sources registered 2026-06-02 (occurrence-typing, CQual, Ramalingam,
frame-problem) per the tightened gather-discipline.**

- **F8 — grounding / type-discovery (the chicken-and-egg).** *Do before F7.* The bootstrap
  limit: you cannot *soundly* co-infer {what a kind is} and {what a command does to it} from
  structure alone — Rice's theorem at the bootstrap; the **symbol-grounding problem**
  (`if widget wombat; then hork_an snuffler wombat`). Guards give **propagation**, not
  **grounding**. Pull + grade: symbol grounding (Harnad 1990), type-inference bootstrapping
  (HM needs grounded literals), typeclass-instance-as-grounding, distributional semantics
  (Firth — the *unsound* corpus escape). **Deliverable:** the *shape of the irreducible
  q-floor* — `≥1 grounded anchor per kind`, guards propagate, ungrounded→⊤→run; and that
  cross-oracle kind-identity is RAL-grounded (a *named* kind), never inferred from a shared
  argument-token (name-collision-unsound). [contract · q-floor · the floor's shape]
- **F7 — specification mining.** *The human added this to the core README → must confirm it is
  **very** relevant, not merely adjacent; pursue after the banked/priority fronts.* The
  established SE/PLT name for *deriving a spec from un-annotated idiomatic code* (seed: Ammons
  et al., "Mining Specifications," POPL 2002; lineage: Engler "bugs as deviant behaviour",
  dynamic protocol/API-spec inference). **Full-read + re-grade** (not abstract-level). Question:
  how much of the probe/establish/frame spec is *mineable* from idiomatic corpora (the
  distributional hints of F8) vs must be *anchored* (the q-floor)? This is the sound/unsound
  boundary of the whole inference story.

## 5. The shape of the answer (why this is genuine-variance, not single-answer)

+SURE this is **genuine variance**: eight domains, a live open knob (`kBURDEN`
infer-vs-declare), and at least one real architectural choice (P3: validation-phase
vs hermetic-ambient-only). So per the skill it gets the plan/review gate, not a
straight-to-conclusion. The eventual round-9 synthesis should deliver: (a) the
reified vocabulary (this map, sharpened), (b) the **ambient/transient typing**
spec'd enough to mechanize, (c) the `kFLATTEN`/`kPROBING`/`kBURDEN` direction it
sets, and (d) the soundness discipline for the probe→apply gap.

## 6. Gate resolution (2026-06-02)

- **q-scope → (a), settled by the human.** The deliverable is *design-space
  narrowing*: constraints, knobs, contracts, walls — correlated to prior-art — per
  the §0.5 quadrant + floor/frontier/ceiling. Mechanism-sketch only insofar as a wall
  forces it. Later passes synthesize toward implementation.
- **q-knob → retracted (malformed).** It posed a binary on a knob `KNOBS.md` already
  defines as a *gradient* (`kBURDEN`: deployer→we-infer, engineer→user-declares,
  Status open, the user "designs the gradient"), and asked to *resolve* an
  intentionally-open knob this round only *maps*. The infer-vs-declare question
  survives not as "which pole" but as the **q-floor/q-frontier** pair: what is the
  irreducible must-declare, and what is the graceful-degradation curve above it. F2's
  job, not a human decision.
