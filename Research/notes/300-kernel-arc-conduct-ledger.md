# 300 — Round-30 conduct ledger: the 28Q-arc implementation

> Tier: LLM-authored conductor working-ledger (Fable; minted 2026-08-14 at round-30 open,
> human-directed). AUTHORITY ORDER: root human docs > `plans/28Q` (THE kernel plan) +
> `notes/28T` (THE tooling plan) + `spike/CLAUDE.md` > this ledger. This file NEVER
> duplicates 28Q/28T content — it carries arc STATE only: staffing, dispatch, ack-grades,
> gates, and the conductor-handoff protocol. Grades: [TYPED] the human typed it ·
> [ACKED] substance confirmed in dialogue · [CONDUCTOR] conductor adjudication,
> unratified unless a human reaction is recorded. Maintenance: compression-resistant;
> folded lanes collapse to a line; newest state at the top of §2.

## §1 — Arc shape and the handoff protocol

- The arc = `28Q` stages i–iii (stage-0 landed pre-arc), OPENED by the `28T` Wave-1
  tooling stage — ordering forced by `28Q` §8: every stage inherits the checker gates
  (certifier + sparing re-derivation green, both planes voting), so the checkers must
  exist first.
- [TYPED 2026-08-14] Conductor-context management: the arc expects sequential
  conductors and/or rewinds. THE NAMED STOPPING POINT is **wave-one-close** (§4). The
  human's rewind anchor is the 2026-08-14 plan-ack sitting (pre-dispatch, this ledger
  committed, zero subagents in flight). A post-rewind or successor conductor MUST
  distrust conversation memory of anything after that sitting — ground truth is this
  ledger + `LIVING_STATUS.md` + `git log` (conductor branch: `ai/r30-conduct`,
  worktree `.claude/worktrees/r30-conduct`).
- [TYPED] Round-30 numbering minted for this arc: this file is notes/300; `plans/301`
  is RESERVED for the solve-certifier mechanical spec (conductor-authored, pre-build).
  Never mint a 29x ID (quarantined round).

## §2 — The Wave-1 stage (28T tooling onboarding): lanes, staffing, gates

[ACKED 2026-08-14, plan-ack] Six lanes + one evidence rider. Opus builders in isolated
worktrees; every brief carries the `spike/CLAUDE.md` safety block verbatim, step-zero
(reset to the conductor-stated `ai/main` tip + hash verify), step-one root-doc reads,
the no-subagent clamp, naming discipline (`270` §1), the `verified-core-discipline`
skill pointer, and flag-don't-resolve on every `tc-*`-shaped judgment.
Sequencing: facade solo FIRST → {kani, certifier, flux} parallel (+ rederivation-(a)
anytime) → rederivation-(b) → discipline-close.

- **lane-facade-std-dropping** (first, solo) — `28T:w1-latticemap-facade`. Evict
  BTreeMap/BTreeSet from the algebra tier — `analysis/src/lattice.rs` (`Powerset` →
  sorted-dedup `Vec<T>`; `MapL` → key-sorted `Vec<(K,V)>`), `core/src/coord.rs`
  (compare/backing-set machinery), `analysis/src/solve.rs`'s BTreeSets — behind small
  owned total-API facades; `core/src/unord.rs` is the API-taste precedent. The
  `solve.rs` VecDeque worklist STAYS [ACKED lean-vecdeque-stays]: the Aeneas-door
  residue (`turn08`: "the one rewrite that would need genuine design work"), not
  invented under a pure-refactor gate. Gates: byte-identical goldens (`bless:dry`
  verify, never bless) · `mise run both gate:full-quiet` · zero new deps (core stays
  dependency-free) · no crate split (a split urge is a tc-flag) · each facade
  invariant gets ONE named seat + tests now, and the builder REPORTS the
  invariant-seat list (feeds lane-kani; the honour-system counterweight said out loud).
- **lane-aeneas-churn-remeasure** (after facade; bounded; non-blocking) — re-run the
  pinned Aeneas pipeline (`spike-aeneas/mise.toml` tasks; WSL leg) over the
  post-facade algebra and re-prove the three lattice laws; measure translation/proof
  breakage. This IS `28T:w3-churn-axis-remeasure` taken at its cheapest moment (the
  facade is exactly the real-refactor the trigger wanted); evidence for §3's vehicle
  ruling. Abort-on-budget is a legal outcome; report, never force.
- **lane-kani-harnesses** (after facade) — `28T:w1-kani-lane`. Opt-in mise lane in
  real-tools-lane shape (off by default; requested-but-absent fails LOUD); expected
  Linux/WSL-only. Targets: the `28T` list + the facade sortedness/canonicality seats.
  Tier placement per the `verified-core-discipline` ladder — the narrative-fold
  permutation pins may land property/DST-tier; placement is FLAGGED, never silently
  decided. Harnesses outside the kernel; hand-written `#[cfg(kani)] Arbitrary`;
  checked code stays stable-toolchain, zero annotations. Conductor reviews the harness
  STATEMENTS (law, bounds, what is NOT pinned) at fold.
- **lane-solve-certifier** (after facade AND `plans/301`) — `28T:w1-solve-certifier`.
  Opus implements against the conductor-authored mechanical spec; the spec carries the
  five turn07 brief obligations verbatim + `28R:fnd-pessimistic-pass-shape` + the
  fresh-ack pedigree note. Shape: `Certified | Refused(EdgeWitness)`; `Refused` ⇒
  degrade to the ⊤/stage-0 floor (license plane) + an operand-carrying narrative
  record (aid plane; `collapse-mints-narrative`); cap-trip certifies the PARTIAL
  solution; `Must<L>` duality, one checker; ships in the DEFAULT suite. Post-land: a
  codex-reviewer cross-lineage pass (`28T:law-independent-guard`).
- **lane-sparing-rederivation** — `28T:w1-sparing-reference-rederivation`.
  (a) [TYPED: codex acked 2026-08-14] a codex-worker (foreign lineage, per the turn07
  implementation-diversity condition; `22W:fb-same-model`) authors the naive reference
  model FROM the Lean theorem statements
  (`.claude/research/refinement-types-industrial-cost/spike-lean-sparing/SparingAlgebra/Laws.lean`
  + its defs); statement-vs-`277`-spec disagreements are FLAGGED to the conductor,
  never resolved by the worker; zero shared helpers with `core/src/coord.rs`.
  (b) Opus integrates: DST-permutation internal differential + plan-time re-derivation
  of every survival verdict before a plan ships; disagreement ⇒ demote to guard/run +
  narrative record; the demote-only structure recorded explicitly (the
  `271:rul-net-quality-u-curve` pass condition; engage
  `271:struck-falsifiability-license-leg` as the nearest prior art).
- **lane-flux-pilot** (after facade; independent) — `28T:w2-flux-adopt-early-scoped`,
  pulled early per the human's stage naming. MEASURE-FIRST: the install-cost claims
  are quarantine-contaminated and UNVERIFIED (turn07 adj-quarantine-claims-stamped-
  verified) — bounded stand-up budget, abort-and-report is a legal outcome. First
  surface: the facade signatures (RVec-style). Nightly pin NESTED per the ratification
  below; the meta-process learnings (CLAUDE.md riders, churn budget, complaint
  reflexes) are a first-class deliverable.
- **lane-discipline-close** (conductor) — the owed `28T:w1-discipline-artifacts`
  halves: verified-core sections appended to `spike/crates/{core,analysis}/CLAUDE.md`
  (incl. the `inv-determinism` sharpening: facade sortedness = named-seat + Kani-pin,
  the honour-system move stated in law text) · the three stale `turn07`→`turn08`
  references (`28T` lines ~262/299/302; `turn06-amendments-ledger` ~137/173-175) ·
  LIVING_STATUS / `28T` §3 / FORFEITS updates · prompt-review pass on the CLAUDE.md
  edits · wave-one-close gate run.

[CONDUCTOR ratification, 2026-08-14] `turn08:tc-nested-mise-config-vs-root` →
toolchain-SHADOWING pins live in nested mise configs (the Aeneas precedent);
additive-only pins (elan) may live at root.

[CONDUCTOR staffing, presented 2026-08-14, awaiting human reaction] The Fable/Opus
split for lanes kani+certifier: Fable authors SPECS and reviews STATEMENTS
(`plans/301`; harness-statement review at fold); Opus authors bodies, tests, and
toolchain wiring. Neither lane runs full-Fable or in-conductor-implementation.

## §3 — The Lean-tier vehicle question (OPEN; human-flagged important)

[TYPED 2026-08-14] Aeneas vs hand-maintained model vs combination is OPEN; the
conductor MAY make the call but it is important and must be made correctly. Human
concern recorded verbatim-in-substance: "Aeneas apparently can't handle types, which
sounds nearly absurd and drops its value-prop to near zero — unless I misunderstand."

[CONDUCTOR answer, banked for the ruling] What fails to cross the translation is
compile-time-only DISCIPLINE — sealed tiers, smart-constructor privacy, phantom
`Must`/`May` distinctions — not types structurally (generics/traits/closures/`solve`
translated whole, `turn08`). This is expected of ANY extraction tool: those guarantees
live in the type CHECKER, not in the data or bodies. They also do not NEED to cross:
rustc enforces them continuously over the real code at zero marginal cost, and the
compile_fail tripwires + Kani pin them. Lean's job is equational law over the BODIES,
which translate faithfully — so Aeneas's value-prop is ZERO-TRANSCRIPTION-DRIFT
definitions (theorems about the shipped bytes), and drift is a live threat, not
hypothetical: the crosscheck found divergences in our days-old hand model beyond its
declared gaps. Aeneas's REAL costs sit elsewhere: zero documentation value
(machine-shaped definitions); toolchain fragility; lawless `Clone`/`Eq` hypotheses
(named trusted-base entries); silent-`sorry` policing; churn cost unmeasured.
Candidate synthesis: hand-written STATEMENTS (the readable review/spec surface) proved
over Aeneas-DERIVED definitions (the drift-free substrate) — the spike's three lattice
laws are a small existence proof of the shape.

[CONDUCTOR lean, held loosely] The hand model stays the statement/spec surface (it
exists, is proved, and is lane-rederivation's diversity source); Aeneas demotes to a
periodic DRIFT-CHECK INSTRUMENT rather than a maintained vehicle — UNLESS
lane-aeneas-churn-remeasure comes back surprisingly cheap. The ruling lands at
wave-one-close (or with the successor conductor) with two numbers in hand: the Aeneas
churn measurement, and the hand model's first real maintenance bill
(`pin-two-position-sparing` lands in the sparing mini-model first, this arc). The
human's "early to lean into documentation/spec" cuts against hand-model EXPANSION,
not against keeping the existing small one aligned.

## §4 — wave-one-close (the handoff gate)

All lanes folded to `ai/main` · `mise run both gate:full-quiet` green + `bless:dry`
clean · certifier + re-derivation live in the DEFAULT suite · Kani/Flux lanes opt-in
and documented · ledgers current (this file, LIVING_STATUS, `28T` §3, FORFEITS) ·
CLAUDE.md discipline sections landed + prompt-reviewed · conductor worktree/branch
cleaned or handed over deliberately. Successor boot order: LIVING_STATUS → this file →
`plans/28Q` → `notes/28T` → `spike/CLAUDE.md`. The NEXT stage's first acts: a full
root `ANALYZER-NEEDS.md` read (it owes the `an-flat-domain` reconciliation paragraph,
`28Q` §7), then 28Q stage-i's fixtures-first commissioning per `28Q` §8 — the
differential cells land BEFORE the conversion.

## §5 — Ack-ledger (what the human has TYPED this arc; silence is never ack)

- 2026-08-14, the plan-ack sitting: the six-lane Wave-1 plan ACKED (including the
  stated leans: rederivation-in-scope · vecdeque-stays · needs-ledgers-deferred) ·
  codex-worker dispatch ACKED ("use it as you see fit") · the two research branches
  deleted by the human's own hand (post-fold; worktrees gone with them) · round-30
  minted, notes/300 assigned, 28Q remains live-and-authoritative · the
  sequential-conductor/rewind protocol directed (§1) · the Lean-vehicle question
  flagged open-and-important (§3) · "do lanes 2/3 deserve Fable tokens?" answered by
  conductor (§2 staffing block), reaction pending.
- Standing inheritances: `28T` §0 postures all [TYPED]; the `28T` §3 checker-triad,
  mini-model, and Vec-facade acks all [TYPED]; Aeneas [PROVISIONAL] with §3 now the
  live venue; Flux [LEAN → stage-ratified by the human's stage naming, scope still
  pilot-tier].
