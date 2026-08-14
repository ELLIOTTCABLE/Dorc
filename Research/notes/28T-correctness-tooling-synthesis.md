# 28T — Correctness-tooling synthesis: the multi-tier adoption plan

> Doc tier: LLM-authored (conductor synthesis, 2026-08-13), human-steered throughout — the
> tail of the r28 arc. This is the ONE coherent durable of the verification-tooling research
> effort: the plan up front, the evidence compressed behind it, decisions-status ledgered,
> rejections footnoted. The graded evidence base (22 sources, 7 turn notes, four crosscheck
> reports, two experiment branches) lives in `.claude/research/refinement-types-industrial-cost/`
> — this doc cites it; it does not restate it. Sits beside `plans/28Q` + `notes/28R` (the
> context-kernel work): several Wave-1 items below are candidate PREREQUISITES for 28Q's
> stage-i/ii dispatches. Nothing herein makes a new decision: [TYPED] = the human typed it;
> [LEAN] = a stated human lean, unratified; [OPEN] = awaiting a human ruling.

## §0 · Posture (all [TYPED], this arc)

- **post-build-for-keeps** — the spiking phase of this axis is OVER: what we build next is
  for keeps, at least for the remaining duration of the current codebase. Enough is known to
  make useful decisions and run with them.
- **post-error-tier-ladder** — three failure tiers: (1) compile-time prevention, (2)
  pre-probe/pre-network fail-fast (~<1s after user invocation), (3) runtime
  fallback-and-continue — and tier 3 costs ~two orders of magnitude more (deep
  Narrative integration). Compile-time value is inflated accordingly; runtime
  nets deflated; a mechanism whose failure lands in tier 1/2 is categorically
  preferred.
- **post-narrative-thirty** — narrative coherence is a first-class engine output, as tracked
  and provable as produced-actions (prose itself excluded). Expected real-world output mix
  ≈ 20% elisions / 80% exhaustively-correct reporting on why-only-20%. Order-instability and
  unexplained no-answers are INCORRECT OUTPUTS on the aid plane.
- **post-two-plane-firewall** — license plane and narrative/aid plane are two consumers
  making separate demands of one engine: separate files, separate passes, separate votes;
  EITHER can kill a kernel change. Asymmetric by design (license values flow into narrative,
  never back).
- **post-aid-seam-exists** — within the aid machinery there is a cost/value seam: engine
  production of narrative-required HISTORY is owed correctness machinery; TEXTUAL output is
  traditional-testing territory. Seam location deliberately unsettled this arc.
- **post-survival-is-the-product** — the elide-half/survival lane is the CORE product
  (elision = the seats people buy; guarding = the luggage: heavier, most of the logistics,
  table-stakes). More dangerous ⇒ more attention, always; a third risk-axis (engine bugs in
  OUR composition algebra) on top of world-chaos + user-claims is unacceptable. Elision is
  never unshipped short of product death. (Consequence for `KNOBS:kSURVIVAL`'s status line —
  the fires-often × bites-rarely measurement decides tuning/marketing, not existence — is
  the human's edit to make.)
- **post-knobs-not-overweighted** — tooling choices must survive knob changes; several welds
  are spike-scoped; fitting tooling to a temporarily-welded simplification (e.g. current
  domain flatness) is the named failure.

## §1 · THE PLAN — what is owed for building, and when

### Wave 1 — now, for keeps (independent of 28Q's blocked stages)

- **w1-latticemap-facade** [TYPED — "ack the restructuring to Vec"] — evict raw std collections
  from the algebra tier behind a small owned facade (~6 total operations over a sorted
  `Vec<(K,V)>`), strict-core scope only (aid/cli keep std freely). One move serves three
  doors: Flux refines the facade signatures (RVec-style), Kani checks the implementation
  exhaustively, Aeneas translates it (measured: `BTreeMap` CRASHES charon inside `alloc` —
  the facade is not optional for that door). Named counterweight to carry into the design:
  eviction moves half of `inv-determinism` from the type system to the honour system;
  mitigation is that the facade's sortedness/canonicality invariants are exactly the
  Kani/property-pinnable shape.
- **w1-kani-lane** — an opt-in mise lane (real-tools-lane shape) of exhaustive bounded
  harnesses, stable toolchain, harnesses outside the kernel, zero annotation in checked
  code. First targets: the lattice laws per combinator (currently five hand-picked samples);
  `MapL` canonical-form; backing-set universal meet + non-empty-by-construction + ⊤-never-∅
  (the measured design-bug class); the ternary consumer map's exhaustiveness; span-edit
  non-overlap. Plus the narrative-fold permutation pins: minted-record MULTISET equality
  under worklist/arrival permutation (scope per crosscheck correction: cross-input and
  permutation variance — the deterministic solver already excludes run-to-run variance), and
  deterministic + DISCLOSED k-cap selection.
- **w1-solve-certifier** [TYPED — acked within the checker triad, §3; pedigree note
  stands: `plans/055` dec-6 names the inert-classification kernel and `plans/021` §1 is
  GUESS-grade "do not adopt up front", so this ack is fresh, never inherited] — the
  per-answer post-fixpoint validator at the solve seam: per edge,
  `transfer(v, state[v]) ⊑ state[w]`, plus boundary conditions; `Must<L>` duality covers
  both orientations with one checker. Returns `Certified | Refused(EdgeWitness)` — never a
  bool; a refusal degrades to the ⊤/stage-0 floor (tier-2, pre-network) and mints an
  operand-carrying narrative record; on cap-trip it certifies the partial solution to
  localize the oscillation. Brief obligations recorded by the crosscheck, to be satisfied in
  the build brief: generic degrade-to-⊤ is unspellable outside `BoundedLattice` (the
  Refused-shape carries it); the must-boundary seeding case must be handled non-vacuously;
  a witness names an edge, not a root cause (narrative duty priced accordingly); the domino
  lemma (certified ⇒ path-coverage) requires a named hypothesis — monotonicity or
  γ-soundness — chosen explicitly.
- **w1-sparing-reference-rederivation** — a naive, obviously-correct second implementation
  of the sparing/composition algebra, run two ways: internal differential under DST
  permutations, and a plan-time re-derivation of every survival verdict before a plan ships
  (disagreement ⇒ demote to guard/run + narrative record). Controller-internal: zero user
  attention spent; the bought-risk contract untouched (claims stay exactly as trusted; only
  OUR derivation is double-checked — the third risk-axis closed per post-survival-is-the-
  product). The `271:rul-net-quality-u-curve` check is CLOSED with conditions (crosscheck
  adjudication): passes as mechanism iff the demote-only structure is recorded and
  implementation diversity is genuinely addressed (`22W` fb-same-model applies — a copy is
  not a check).
- **w1-discipline-artifacts** — the `verified-core-discipline` skill is SHIPPED
  (`.claude/skills/verified-core-discipline/`, prompt-review-audited): fires when
  unrelated-task agents wander near the verified core; carries the tier ladder, the
  anti-gaming law (never weaken the question; flaky-vs-broken protocol), per-tier praxis.
  Still owed: the location-bound halves as appended sections in
  `spike/crates/{core,analysis}/CLAUDE.md` (code-shape rules that bind today), and a Lean
  model directory `CLAUDE.md` when/where that artifact lands. The future comprehensive
  Rust-writing skill (human-flagged as owed, undispatched) absorbs DST-writing practice as a
  section, not a separate skill.

### Wave 2 — with the spec/kernel work (sequencing couples to 28Q/28R rulings)

- **w2-lean-tier-governance** [NARROWED — the verified mini-model is TYPED-acked as a
  maintained artifact; the Aeneas vehicle is PROVISIONALLY acked (§3); residual: the
  post-review confirmation + the multi-vehicle budget] — three
  coexisting vehicles for Lean-tier sparing assurance: (a) the extant hand-written model
  (proved, checked, but with divergences beyond its declared gaps — see §2), (b)
  Aeneas-derived definitions from the disciplined Rust core (measured viable: 0 fundamental
  walls, ~a day of within-discipline rewrite), (c) at-settle promotion of a Rust reference.
  One governing decision + budget owed; multi-vehicle maintenance is unpriced. Aeneas-lane
  facts to weigh: type-level guarantees DO NOT cross the translation (May/Must arrive as
  bare `L`; sealed tiers as empty structures — the claim-tier system stays Rust-enforced
  and its properties would need restating as explicit theorems); lenient-mode emits SILENT
  `sorry`s (a green build proves nothing without a `sorry` census — operational law);
  Aeneas's `Clone`/`Eq` models are lawless (each generic seat carries a hand-written
  lawfulness hypothesis = a named trusted-base entry); Aeneas's own Lean library ships 4
  `sorry`s. Toolchain is solved and pinned for both vehicles (elan-under-mise; nested
  `mise.toml` for the Aeneas lane because its rust pin shadows the workspace compiler).
- **w2-spec-repairs** — fold the formalization-pressure findings back into the spec text
  (spec-owner acts; the spike's gap list is a FLOOR, not a census, per crosscheck): compare
  directionality (§2 writes `compare` symmetric; §3's dialect tier is position-typed —
  independently converges with `28R:adj-sparing-two-position-rule`); ⊤-selector
  self-sameness (definite-whole-entity vs failed-⊤ share an encoding); footprint-side
  ∅-invariant unpinned (the meet is vacuous over an empty footprint too — proved);
  plus the crosscheck's model-divergence list (ctx-gate-before-kind-fence forfeits
  `272` §4's free cross-kind disjointness; equal-unminted-tokens compare `same`; §2
  registry subsetted; §5 transport half unmodeled). Also: `28R`'s acked amendments
  (instantiation-hash-dedup; mixed-custody-suspends-vouch) have no referent in `277` yet.
- **w2-flux-adopt-early-scoped** [LEAN, human: add-early, not ship-it-now] — one scoped
  crate + the seam-encoding rule ("an invariant lives at one named seat" — already house
  style, promoted to law so a later types→refinement-contracts migration stays mechanical);
  nightly pin contained to its own check lane; the meta-process learning (CLAUDE.md riders,
  churn budget, complaint reflexes) is a first-class deliverable while sharp edges are
  cheap. Candidate surfaces beyond the kernel: the intake byte-budget, span arithmetic, and
  the weft/arrangement numerics (the aid plane is a legitimate target under
  post-narrative-thirty).
- **w2-narrative-plane-law-set** — the aid plane's own laws, sealed from the license plane
  ("as provable, differently proved"): mint-completeness (built) + fold-confluence pins
  (w1-kani-lane carries the mechanism) + consumption-totality (an `[unexplained: <class>]`
  render + census gate closing `289:seam-narrative-render-unconsumed`) + replay-fidelity
  (whylog re-derivation bit-equality). STEP ONE before building any of it: a full read of
  root `AID-NEEDS.md` and reconciliation against its law section (deferred this arc under
  the earlier weighting; wrong to defer further once building starts).

### Wave 3 — triggered (events, not dates)

- **w3-reactive-model-before-code** — when the reactive/multi-host fixpoint build (`26C`)
  starts: its confluence/termination theorems are stated, unproven, unbuilt, and its own
  audit found a contradicting prior rule — the one place model-before-code applies at full
  Cedar strength.
- **w3-churn-axis-remeasure** — the Aeneas gate measured TRANSLATION-distance, not
  churn-cost (crosscheck): re-run the pipeline across a real refactor after 28Q stage-i/ii
  land, and measure proof/translation breakage, before the Lean-tier vehicle choice is
  treated as settled economics.
- **w3-foreign-ground-truth** — the open investigation (no cheap dash/posh-analogue floor
  exists for the ANALYSIS plane): candidate grounds are the real-machine end-state
  differential (`an-calibration-delta` / the r25 P2/P4 observer hooks), independent-lineage
  executable models, metamorphic book-transformations (equal plans AND equal explanations),
  and lying-oracle DST; the explanation plane has NO external oracle at all — its checks
  are internal-coherence + replay-fidelity. Needs its own sitting; the certifier is NOT
  foreign (same transfer model — catches solver bugs, never model bugs).

### Cross-cutting build laws (bind every wave)

- **law-never-weaken-the-question** — the anti-gaming law, verbatim in the skill: no
  sorry/assume/statement-edit/harness-weakening/cap-raising to get unblocked; fix the code
  or escalate with the artifact in hand. Flaky-vs-broken: deterministic tiers cannot flake;
  solver-backed tiers get one classifying re-run, then escalate AS instability.
- **law-deterministic-feedback-ordering** — for agent loops, prefer tools by failure-signal
  determinism: types > Kani > property/DST seeds > liquid inference > SMT proof > tactics.
- **law-independent-guard** — agent-authored code checked by agent-authored tests shares
  wrong assumptions; every tier keeps a guard the author cannot influence (foreign
  binaries, kernel-checked proofs, cross-lineage review — the crosscheck measured genuine
  decorrelation: each lineage-pair certified sound what the other flagged).
- **law-null-result-honesty** — on a well-tested codebase, verification's marginal BUG
  yield may be ~zero; the purchased goods are the documented-blind-spot classes
  (goldens-cannot-see positional agreement; DST's own harness bugs; correlated crosscheck
  blind spots) and the hardened feedback signal. Market the receipts accordingly, never as
  "the product is proved good".

## §2 · The evidence, compressed (full detail: the research dir)

- **ev-industrial-base** — 22 graded sources, read raw (a summariser-contamination incident
  in turn 1 was caught and quarantined; two figures it corrupted were corrected from the
  raw artifacts). Load-bearing results: Cedar (model+differential shape, 3.4:1
  proof-to-model, 4 proof-bugs vs 21 testing-bugs, the Dafny→Lean abandonment at
  meta-theory scale); Everest (SMT instability as the churn mechanism; six codegen bugs
  past all checkers; the §4.5 hindsight endorsing model/impl layering; verified-codegen as
  the sustainability existence-proof); Verasco (34k-line Coq ceiling case; internal
  a-posteriori validation for its hardest domain); Blazy/Pichardie (the result-certification
  architecture, fallback-to-⊤); seL4/HACMS (cost baselines; red-team validation); Gamboa
  (the nine liquid-type adoption barriers; silent-weak-spec acceptance; the
  invariant-identification cost Dorc has ALREADY paid via its English-theorem corpus);
  vericoding (model-union 82/44/27 Dafny/Verus/Lean — and the spike's own counter-evidence
  below); prover-is-the-judge (agents game weak checks; enforcement was human review).
- **ev-kernel-reads** — the algebra tier is ~1.2k lines and spec-frozen; the engine ~11k
  and churny. The solver's preconditions are caller-upheld and un-type-enforceable
  (empirical hang, 435/783 CPU-s; `converged` is a per-consumer obligation). `claim.rs` is
  already at the lightweight-tier ceiling (sealed tiers, compile_fail tripwires) — with the
  crosscheck's narrowing: accident-proof, not determined-author-proof. Census: ~zero
  unwrap/expect outside tests; 1361 invariant-comments vs 7 runtime asserts; narrowing
  arithmetic 100% checked-routed. Documented harness blind spots exist
  (goldens-cannot-see positional agreement; the LCG thinning incident).
- **ev-corpus-priors** — `plans/021` §1 / `plans/055` dec-6 litigated proof-assistant
  adoption to "NO, with one narrow deferred exception" — the exception names the
  INERT-CLASSIFICATION kernel (crosscheck correction: it does NOT pre-authorize the solve
  certifier; that ack must be fresh). `KNOBS:kVERIFY` stays welded — this arc's work is
  kernel-internal assurance + calibration instruments, not whole-system proof.
- **ev-lean-spike** — the sparing algebra formalized in Lean 4 on the first attempt: five
  theorem families proved, zero `sorry`, axiom-clean, Windows-native toolchain in ~35min
  (elan-under-mise, committable config). Viability signal sharply better than benchmark
  priors for OUR theorem shapes (3 compile round-trips, 2 tactic-level errors). SEVEN
  spec-gaps found pre-checking — formalization pressure works — and the crosscheck found
  divergences BEYOND the declared list plus `SoundCompare` assuming (not proving)
  compare-soundness. The artifact is evidence and seed, not yet law.
- **ev-aeneas-experiment** — translation of the real `lattice.rs`/`solve.rs` extracts:
  0 fundamental walls; generics/traits/PhantomData/derives/impl-Fn all translated
  untouched (the monomorphic-boundary discipline is Kani-driven, NOT Aeneas-driven —
  turn-5's intersection claim softened); std collections are the whole cost; three lattice
  laws proved axiom-clean against the derived definitions; the silent-`sorry` and lawless
  Clone/Eq findings (§1 w2). Toolchain pinned and reproducible.
- **ev-crosscheck** — four reviewers (2× Fable, 2× Sol) + suborchestrator adjudication over
  the ENTIRE arc: unanimous that no human-typed decision is invalidated; five conductor
  assertions corrected (the "(all verified)" mislabel on two quarantine-tier claims; the
  certifier-pedigree stretch; "seventeen days" → ~4 days; flip-3's run-to-run overstatement;
  the unrepresentability narrowing). Method: genuine cross-lineage decorrelation
  demonstrated; neither pair sufficed alone.
- **ev-kernel-currency** — 28Q ACKED with stage-0 built and stages i–iii blocked on two
  human rulings; `lattice.rs`/`solve.rs` untouched since the founding round and deliberately
  contract-preserved by 28Q's migration gate; `277` textually current but amended by 28R
  rulings not yet folded in. `28R:fnd-pessimistic-pass-shape` imported as a certifier
  design constraint.

## §3 · Decision ledger

- [TYPED] everything in §0; property-testing scoped into the check-ladder; the crosscheck
  fleet shape (2×Fable + 2×Sol, suborchestrated); no further spiking; 28-series homing of
  this document; no round-numbered naming for the research artifacts.
- [TYPED] **the checker triad** — human gloss, recorded verbatim: "the three checkers —
  aid/non-aid per-transition, as well as a final walk for elision purposes, I believe it
  was?". Conductor mapping (mine, offered back for confirmation): (1) the two-plane
  firewall's separate per-kernel-change votes (aid gate + license gate), (2) the
  solve-certifier's post-fixpoint walk, (3) the sparing reference re-derivation — the
  elision-purposes walk. Mapping CONFIRMED after an explicit evaluation pass (see §4's
  checker-expansion rejection): the aid half of the per-transition vote is gates + pins +
  render-totality, deliberately not a runtime module; the post-probe walk is
  survival-scoped, deliberately not a whole-chain re-walk.
- [TYPED] **the verified mini-model** — a small machine-checked model of the sparing
  algebra is a maintained artifact of this codebase.
- [TYPED] **the Vec restructuring** — w1-latticemap-facade acked.
- [PROVISIONAL] **the Aeneas vehicle** — acked provisionally; human review owed
  post-rewind; stated concern: possibly underpowered for the important concepts (the
  type-guarantees-do-not-cross and lawless-Clone/Eq findings are the live evidence); the
  named fallback if it cannot model what matters is maintaining the hand-written model.
- [LEAN] Flux add-early-scoped; the human's parenthetical this-arc: nothing heard so far
  disliked.
- [OPEN] the w2-lean-tier-governance residual (post-review Aeneas confirmation + the
  multi-vehicle budget); the `KNOBS:kSURVIVAL` status-line edit; branch/worktree
  disposition (`ai/research-lean-sparing-spike`, `ai/research-aeneas-spike` — both
  additive, unmerged; NOTE both the crosscheck (on `ai/main`) and the Aeneas branch minted
  a `turn07` note — the Aeneas branch's renumbers at merge); the aid-seam location
  (deliberately unsettled); AID-NEEDS reconciliation before w2-narrative work.

## §4 · Considered and rejected (footnote-tier; reasons in the research dir)

- Lean-extracted production kernel (ship the proof-assistant artifact): contraindicated by
  the full evidence stack — kept only as the never-chosen pole of w2 governance.
- Field-trial trigger for survival-algebra assurance: DIED under post-survival-is-the-
  product (severity × invisibility × permanence outranks operational load).
- Flux workspace-wide now / Verus / Prusti / MIRAI / Creusot-now: deferred or dead
  (toolchain maturity, annotation economics, maintenance status) — re-entry conditions in
  the round notes.
- One mega-skill, or one-skill-per-tool: replaced by task-moment routing (one skill) +
  location law (CLAUDE.mds).
- Separate DST skill: absorbed — a section of the future Rust-writing skill.
- Protocol-atomicity classes as refinement targets: tool-shaped-hammer error; they stay
  framing/closure/DST territory.
- The operational load-bearing test for assurance priority: conceded and replaced (§0).
- Treating the certifier as foreign ground truth: it is not (shares the transfer model).
- Expanding the checker set by symmetry: a per-phase-product results-checker beyond the
  solve seat, a whole-chain re-walk of every elision's license conjuncts, and a RUNTIME
  aid-plane checker module were each evaluated and rejected. The admission tests a checker
  must pass: (T1) large find/check asymmetry in the producer (only the fixpoint qualifies —
  classify/erasure/wall-walk are single-pass deterministic code, where a "checker" is
  N-version programming with `22W` fb-same-model correlated-blind-spot risk), or (T2)
  maximal severity × invisibility (only the survival lane qualifies; ordinary elisions'
  conjuncts are covered at their sources — reach by the certifier, the vouch by the
  by-value type seal, the verdict by the single classify seat — and fail attributed).
  The aid plane additionally fails a third way: decision-inert by law, a runtime check has
  no decision to protect; its real protections are structurally non-runtime
  (class-total census gates, cross-run permutation pins, render-totality with a loud
  `[unexplained:]` face).

## §5 · Pointers

- Evidence base: `.claude/research/refinement-types-industrial-cost/` — `turn01`–`turn06`
  (round + synthesis), `turn06-amendments-ledger.md` (the post-synthesis rulings ledger,
  banked through crosscheck close), `turn07` (crosscheck adjudication) + `crosscheck/`
  (four reports, dispatch bundle), `sources.json` (+ local `sources/`, gitignored),
  `spike-lean-sparing/` (on branch `ai/research-lean-sparing-spike`).
- Aeneas experiment: branch `ai/research-aeneas-spike` (its own `turn07` note + nested
  `spike-aeneas/mise.toml`).
- Discipline artifact: `.claude/skills/verified-core-discipline/SKILL.md`.
- Kernel-currency context: `plans/28Q`, `notes/28R` (+ `28S` for the arc that renamed the
  branch under this session's feet).
- Commits this arc on `ai/main`: `9b05283f` (round bank) → `ad57964d` (skill) →
  `d3ccc17c` (crosscheck durables) → `0b86f537` (amendments ledger).
