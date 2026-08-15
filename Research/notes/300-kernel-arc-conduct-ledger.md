# 300 — Round-30 conduct ledger: the 28Q-arc implementation

> Tier: LLM-authored conductor working-ledger (Fable; minted 2026-08-14 at round-30 open,
> human-directed). AUTHORITY ORDER: root human docs > `plans/28Q` (THE kernel plan) +
> `notes/301` (THE minispec/dorc-verify spec) + `spike/CLAUDE.md` > this ledger. This file
> never duplicates those — it carries arc STATE: staffing, dispatch, ack-grades, gates,
> the census bank, and the conductor-handoff protocol. Grades: [TYPED] the human typed
> it · [ACKED] substance confirmed in dialogue · [CONDUCTOR] conductor adjudication,
> unratified unless a human reaction is recorded. Maintenance: compression-resistant;
> folded lanes collapse to a line; newest state at the top of §2.

## §1 — Arc shape and the handoff protocol

- Round 30 splits in two: the FIRST HALF is the correctness-tooling standup (this
  file's §2 + `notes/301`; it reshapes and supersedes the execution-plan half of
  `notes/28T`, which stays the evidence digest, marker-annotated); the SECOND HALF is
  `28Q` stages i–iii (stage-0 landed pre-arc). Ordering forced by `28Q` §8: every
  stage inherits the checker gates (certifier + sparing re-derivation green, both
  planes voting), so the checkers must exist first.
- [TYPED 2026-08-14] Conductor-context management: the arc expects sequential
  conductors and/or rewinds. THE NAMED STOPPING POINT is **wave-one-close** (§4). The
  human's rewind anchor is the 2026-08-14 plan-ack sitting (pre-dispatch, this ledger
  committed, zero subagents in flight). A post-rewind or successor conductor MUST
  distrust conversation memory of anything after that sitting — ground truth is this
  ledger + `notes/301` + `LIVING_STATUS.md` + `git log` (conductor branch:
  `ai/r30-conduct`, worktree `.claude/worktrees/r30-conduct`).
- [TYPED] Round-30 numbering: notes/300 = this ledger; notes/301 = the minispec/verify
  spec; `plans/302` is RESERVED for the solve-certifier mechanical spec
  (conductor-authored, pre-build). Never mint a 29x ID (quarantined round).

## §2 — The Wave-1 stage (the correctness-tooling standup): lanes, staffing, gates

[ACKED, as reshaped through the 2026-08-14 design sittings] Opus builders in isolated
worktrees; every brief carries the `spike/CLAUDE.md` safety block verbatim, step-zero
(reset to the conductor-stated `ai/main` tip + hash verify), step-one root-doc reads,
the no-subagent clamp, naming discipline (`270` §1), the `verified-core-discipline`
skill pointer, and flag-don't-resolve on every judgment call. PLUS the quarantined
builder-prerequisite read, per the conductor skill's quarantine section (durable,
human-committed 2026-08-14): Opus/Sonnet builders and foreign-lineage reviewers,
before any other work; never Fable-class subagents; the conductor never reads it. No lane carries
pilot/measure/kill staging [TYPED — velocity; the human inserts kills if needed].
Sequencing: facade solo FIRST → {derived-defs pipeline + minispec/verify standup} and
{kani, certifier} in parallel → rederivation integration → discipline-close.

- **lane-facade-std-dropping** — **FOLDED 2026-08-14 @ `601364f7`** (four commits;
  conductor-verified both legs green, zero golden drift, zero new deps, no split).
  What exists now: `core::sorted::{SortedSet, SortedMap}` (private-backed, one
  private `position`→`Slot` scan each; total, panic-free, index-walk bodies);
  `Powerset`/`MapL` re-seated on them (`Powerset`'s backing SEALED — the pub field
  died as the Aeneas-prep/refinement-enabling structural change); `Dialect`/
  `selector_covers` moved onto them; `solve.rs` production code untouched (its
  BTreeSets were test-only — census delta); VecDeque stays per lean-vecdeque-stays.
  §2a below banks the seat list + findings the later lanes consume.
- **lane-derived-definitions-pipeline + lane-minispec-verify-standup** — **FOLDED
  2026-08-14 (combined lane, 13 commits; conductor-verified)**. As-built: the
  translation unit + fence (`spike/verify/aeneas/`; strict-translate 0 holes/40
  axioms; lake green 1707 jobs; byte-idempotent regeneration; hole+axiom census
  wired — a green translate proves nothing without a green lake build AND a census,
  the `304:fnd-mut-closure-emits-ill-typed-lean` law); `minispec/` skeleton (three
  unwritten unit stubs, Proofs/, committed `Generated/`, CLAUDE.md conductor-stub);
  the `dorc-verify` crate + nine `verify:*` tasks + the hk `minispec` step;
  `tests-critical-law` frontmatter key + two-way agreement, both directions. Badges
  REAL: proved · elaborated · interrogated · report/catalogue/mismatch-refusal;
  NAMED SEAMS (structural `needs_external_engine` tier split): `seam-kani-pairing-
  unbuilt` · `seam-decision-record-read-mode` · `seam-statement-mutation-unbuilt`.
  Verso NOT adopted (fallback: module docstrings; three costs in `304`; re-open at
  the first binding). Tripwire 8KB. Tools-built-general: NONE of the imported family
  stood up (would have been minispec-scoped at budget — the law's named-seam path
  taken, correctly). FINDINGS + the reshape table: **`notes/304`** (née 303-at-mint).
  OWED OUT OF THE FOLD: (a) the FIVE one-line `match` respells in the verified core
  (Option-combinator idioms choke Aeneas; proved cousins; `SortedSet::insert` — the
  canonical-form seat — is currently fenced as COLLATERAL of charon's
  inherent-impl-naming limitation, so no Lean law about set insertion until the
  reshape lands; remit-over-`Flat` unaffected) → the reshape rider below; (b) a
  plan-route decision dump emitting `(SiteId, decision)` pairs — the whylog's
  `ApplyLine` records `leaf: u32` only, collapsing in-loop member sites — a real
  product feature, its own dispatch, prerequisite to `demonstrated` bindings;
  (c) `spike/CLAUDE.md` Build/test/run grows the `verify:*` rows at discipline-close.
- **lane-facade-reshape** — **FOLDED 2026-08-15 @ `b9d91fec`** (three commits,
  rebased; conductor-verified both legs green + `verify:check`). The five `match`
  respells landed; the pipeline is end-to-end +SURE (translates AND typechecks);
  the fence is the permanent EXITS class only — `SortedSet::insert`, the
  canonical-form seat, now carries a real Lean body (the collateral cost
  discharged; set-insertion laws are stateable). Census 40→26 as published.
  Banked for law-authoring seat-picks: `alloc.vec.Vec.{remove,is_empty}` and the
  derived pair-`PartialEq` instance remain trusted-base axioms UNDER now-open
  bodies — remove-family and Eq-derived laws prove modulo those;
  insert/get/get_at/position/union/intersection are axiom-free below. Riders
  routed out: `304:fnd-axiom-census-double-counts` (Template files double the
  count; real unique = 13) → the kani lane; the
  keep-borrows-out-of-closure-returns discipline needs its durable home in
  `core`/`analysis` CLAUDE.mds → discipline-close (currently stated only in the
  aeneas Cargo.toml, where a facade editor will never look).
- **lane-kani-harnesses** (after facade) — opt-in mise lane, real-tools-lane shape,
  expected Linux/WSL-only; harness home is `spike/verify/` (`301` §3). Targets: the
  lattice laws per combinator · `MapL` canonical-form · backing-set universal meet +
  non-empty-by-construction + ⊤-never-∅ · ternary consumer-map exhaustiveness ·
  span-edit non-overlap · the facade sortedness/canonicality seats. Tier placement per
  the `verified-core-discipline` ladder (the narrative-fold permutation pins may land
  property/DST-tier — placement flagged, never silently decided). Hand-written
  `#[cfg(kani)] Arbitrary`; checked code stays stable-toolchain, zero annotations.
  Conductor reviews the harness STATEMENTS (law, bounds, what is NOT pinned) at fold.
- **lane-solve-certifier** — **FOLDED 2026-08-15 @ `a1535601`** (8 commits;
  conductor-verified both legs 1967/1963 green, zero golden drift). As-built per the
  settled `302` + the checkpoint rulings: `certify.rs` (~700 lines incl. tests;
  `SolveConsistency` private-mint; `core::sorted` throughout, no facade extension
  needed); the observer seam in `solve` (`run` + `Unobserved` ZST; loop extracted
  once); `trusted()` = certified at all nine call sites (the lane's ONE widening,
  spec-settled: consistent-at-cap is the lfp and is used); four named consumer floors
  (funcenv fold-BREAK with `folded_edges=∅` pinned); aid plane registered
  (`solver-consistency-failure` + `SolvePass` + `CollapseKind::SolverConsistencyFailure`,
  prose `[unwritten:]`); cli reports root-cause-only pre-network. Adjudications at
  fold: **F9 accepted-as-disclosed** — the reach/self-reach floors are gate-level
  (boolean, cardinal-sin-safe direction), end-to-end drive priced 1–2h, natural home
  = the classify rework 28Q stage-iii forces · F10 (`mise run fmt` refuses under
  agent env; working spelling `mise exec -- cargo fmt --all --manifest-path
  spike/Cargo.toml`) + F11 (WSL keeps a separate mise trust store; new worktrees need
  a WSL-side `mise trust`) → discipline-close Build/test/run lines · whylog-spine
  chafes banked for enrichment: `FailedCheck` carries run-scoped CFG node indices
  (partial node→`SiteId` mapping OR a distinct run-scoped row species — undecided) +
  `Inconsistency<Reach>` holds `ProvId` (resolve-or-drop at any future durable edge —
  spine boundary (1) doing its job). Open: `302:pin-blast-radius-escalation`
  [HUMAN?]. The post-land cross-lineage review RAN (Sol, read-only; raw report
  committed at `.claude/reports/r30-certifier-review/report.md`) and RETURNED FIVE
  FINDINGS, every one conductor-verified in the code before crediting (provenance:
  OpenAI lineage, adjudicated under maximum skepticism): (1) missing-states
  fail-open — `certify_solution`'s `let-else continue` lets a truncated/empty states
  vector certify clean with zero checks, contradicting `302` §2's pessimistic rider
  (production-latent only while `solve` is correct — which is the exact assumption
  the instrument exists to drop); (2) the `run`/`Unobserved` `pub(crate)` seam
  bypasses the `solve(`-needle lexical fence; (3) the origin-round reach/self-reach
  consistency diags surface AFTER the probe-mode return and probe shipping,
  violating the §4 pre-network posture (floors held; disclosure late/lost);
  (4) the SelfReach account is materially false (solve-count reported in the
  failing-checks field; fabricated advisory) — mis-attribution-adjacent under
  `271:rul-sin-ordering`; (5) the `effect.rs` debug_assert fires BEFORE
  demote-and-report, so debug builds (and DST) panic instead of exercising the
  machinery. The review also independently VERIFIED the funcenv grant-seal from the
  code, and REFUTED "raw solve unreachable" as an enforcement claim (true today,
  unenforced tomorrow). REPAIR LANE dispatched (the certifier builder resumed;
  branch `ai/r30-lane-certifier-repair` @ `da66a918`): R1 length-mismatch ⇒
  Inconsistent (edge-guard mirror stays) · R2 fence gains the `run(` needle ·
  R3 origin-round diags surface pre-network · R4 honest-or-typed-absent account ·
  R5 assert deleted · plus the F9 end-to-end floor drive folded in. Human
  push-notified 2026-08-15. **REPAIRS FOLDED 2026-08-15** (four commits; builder
  gates green both legs 1974/1970, zero drift, fences falsified both directions;
  conductor gate over the combined tip at fold). Execution found TWO findings worse
  than reviewed: the fence's `production_half` cut each file at its first
  `#[cfg(test)]`, blinding it to production items after a test module (both fences
  now scan whole files — simpler AND stronger); and the origin round's consistency
  diags were DROPPED entirely (`origin.diags` discarded for `round.diags`), never
  merely late. R1's landed shape separates graph-fact (solver-mirror stays) from
  solution-defect (missing state/seed ⇒ `failing.boundary`; `inconsistencies()` may
  be legitimately empty while `total > 0`). R4 gave the record real
  failing-check/solve counts + a measured `SolverRounds` advisory. F9's ×4 is
  complete (`a_real_reach_inconsistency` — genuine perturbation, genuine checker,
  `ProvId`-bearing — drives both floors; non-vacuity controls included). No de-dup
  between origin- and fixpoint-round failures: different solves, different events
  (builder call, endorsed).
- **lane-sparing-rederivation (a) — FOLDED 2026-08-15** (codex/Sol-authored,
  foreign lineage, shim-backstop-committed; three commits; conductor law-review
  passed; gate over the combined tip at fold). `dorc-sparing-reference`: 614 lines,
  zero deps, own opaque token vocabulary, `BackingSet` non-empty by construction,
  22 law-named tests; authored blind to production code as briefed. THE THREE
  MODEL-AMENDMENT RULINGS (conductor, from the fold review — two flagged by the
  model's author, one a bundle omission of MINE; all are additions to the ruled
  English, and lane-(b) applies them to the model with citations):
  `rul-reference-entity-name-floor` — within-kind unequal entities answer
  ProvablyDisjoint under the name-comparison floor (`canonical-coord-continuity`'s
  no-resolver floor; USER_STORY stage-6); the model stays resolver-blind and the
  (b)-adapter feeds canonicalized entities where a resolver exists. EPISTEMIC
  SHARPENING [human-corrected 2026-08-15]: "ProvablyDisjoint" is the algebra's
  verdict vocabulary — disjoint GIVEN the claims — never machine-proof of
  referent-inequality. Entity-disjointness rests on a SPEECH-ACT (the resolver
  author's claim, or the disclosed-weak name-floor), must wear its tier
  mechanically in every why-chain (`trust-tier-is-syntax`; a claim never renders
  in measurement's clothing), and no aid surface may spell it "proven".
  Discipline-close check item: verify the survival why-chain DISTINGUISHES
  resolver-claimed disjointness from name-floor-derived disjointness; a gap there
  is an aid-plane finding to ledger, never to silently build ·
  `rul-reference-kind-fence-disjoint` — cross-kind pairs short-circuit
  ProvablyDisjoint (the v1 kind fence, `kind-fence-movable`; my bundle stated only
  no-cross-kind-same, so the model answers Unknown and would demote every real
  cross-kind meet) · `rul-reference-empty-footprint-assert` — the model KEEPS its
  conservative empty-footprint collide; the (b)-adapter asserts production never
  feeds ∅ to the meet, and a violation is a differential FINDING, not a
  normalization. Shim anomalies logged: the Windows codex self-commit ACL grant was
  blocked by the permission classifier (backstop path used; a settings-allowlist
  question for the human someday); a sibling-codex-in-same-worktree observation
  (~SUSPECT process-ancestry misread; no corruption; logged only).
  Original (a) charter, for the record: the naive reference model of the
  sparing/composition algebra, authored FROM the ratified English law-set under
  structural-simplicity constraints: the checker's value is STRUCTURAL difference —
  written under different constraints, from the machinery-free description of the
  goal, one pass, no worklist — never authorial lineage [TYPED — the
  independent-voices framing was deweighted; a foreign-model author (codex, ACKED
  available) is incidental, not load-bearing]. Zero shared code with `coord.rs`;
  statement-vs-spec disagreements FLAGGED, never resolved. (b) Opus integrates:
  DST-permutation internal differential + plan-time re-derivation of every survival
  verdict before a plan ships; disagreement ⇒ demote to guard/run + narrative record;
  the demote-only structure recorded explicitly (the `271:rul-net-quality-u-curve`
  pass condition).
- **lane-flux-engine-hardening** — [TYPED 2026-08-14: DEFERRED, penciled] not in
  Wave-1 (scoping it in would bloat; enough is on the table). Penciled MID-r30: after
  everything Lean-related is stood up (wave-one-close), before the proper kernel
  rewrite (28Q stage-i). Intent stands: another defense-in-depth instrument —
  ENGINE-tier (intake byte-budget, span/interval arithmetic; the churny tier no other
  instrument reaches at compile time), explicitly NOT part of the verification core
  (Kani+Lean+binder own the algebra; triple-covering rejected); nightly pin nested;
  meta-process learnings a deliverable. EXCEPTION [TYPED]: any typesystem or
  architecture change REQUIRED for Flux to be possible at all belongs in the
  Aeneas-prep work — the facade lane's scope, not the deferred lane's.
- **lane-discipline-close** (conductor) — the verified-core CLAUDE.md sections for
  `core`/`analysis` (incl. the `inv-determinism` sharpening: facade sortedness =
  named-seat + Kani-pin, the honour-system move stated in law text) · FORFEITS rows if
  any arise · prompt-review pass on all CLAUDE.md edits · ledger/LIVING_STATUS
  currency · the wave-one-close gate run (§4).

### §2a — Facade-fold bank (consumed by lane-kani, the derived-defs lane, and Flux)

Invariant seats (seat · invariant · pinning test; all tests in the default suite):
- `core::sorted::SortedSet::insert` — strictly-ascending, duplicate-free backing —
  `set_insert_sorts_and_dedups`; and structural `PartialEq` == semantic set equality
  (what `solve`'s `joined != state[w]` fixpoint test rests on) —
  `set_structural_eq_is_semantic_eq`.
- `core::sorted::SortedSet::position` — membership agrees with backing at every
  boundary — `set_contains_and_remove_agree_with_membership`.
- `SortedSet::union`/`intersection` — canonical results; ∪/∩ commute; ∅ identity/
  absorbing — `set_union_and_intersection_stay_canonical`.
- `core::sorted::SortedMap::insert` — ascending unique keys; rebind replaces+returns —
  `map_insert_sorts_keys_and_replaces_values`; structural==semantic Eq —
  `map_structural_eq_is_semantic_eq`.
- `SortedMap::remove`/`get_at` — order survives removal; `get_at` walks key order —
  `map_remove_and_get_at_keep_key_order`.
- `analysis::lattice::MapL::insert` (pre-existing) — no key maps to `V::bottom()` —
  `maplattice_is_pointwise_and_canonical`; plus insertion-order-independence of
  `Powerset`/`MapL` equality — `collection_domains_are_insertion_order_independent`.

Kani-lane guidance (builder-supplied, conductor-endorsed): the canonical predicate is
strict ascent — `∀i: get_at(i) < get_at(i+1)` (sortedness+dedup in one; maps over
`get_at(i).0`); NO `pub is_canonical()` was added (harnesses express it; add in the
Kani lane only if needed). `#[cfg(kani)] Arbitrary` homes in `core::sorted` (reaches
the private field, no widening) and must construct via arbitrary `Vec` +
`kani::assume(canonical)` — building via repeated `insert` would make the `insert`
harnesses circular. The asymmetric risk the harnesses exist to close: a bug making two
semantically-DIFFERENT values compare equal stops the solver's climb early
(under-approximated may-set ⇒ potential wrong elision, invisible to goldens); the
opposite bug only trips `converged: false`. Until Kani lands, the seat tests are the
whole net.

Findings + conductor adjudications:
- `fnd-reach-lattice-outside-scope` — `analysis::effect::Reach` (a `Lattice` impl in
  engine-tier `effect.rs`) still holds a raw `BTreeSet<FactKey>` + a hand-written
  cause-excluding `Eq`; the algebra tier is NOT BTree-free. ADJUDICATED: eviction
  deferred (not this wave; careful territory — the cause-excluding Eq is
  correctness-critical); the derived-defs lane EXCLUDES `Reach` from translation scope
  at v0 and says so in its config; revisit when a Lean statement first needs
  reaching-defs.
- `fnd-generic-ord-blocks-refinement` — facades stay generic over `T: Ord`; Flux needs
  concrete decidable orders, so the Flux lane (mid-r30) prices harness-side
  monomorphic instantiations (`SortedSet<SelectorId>` etc.), never product-code
  monomorphization.
- `fnd-iterator-exits-may-not-translate` — `iter()`/`IntoIterator`/`FromIterator` are
  grouped+commented as the translation boundary; the algebra proper avoids them. If
  the Aeneas pipeline chokes on the ALGEBRA (not the exits), the `while let
  Some(x) = v.get(i)` shape is unusable and the facade needs re-shaping — report,
  don't patch.
- `dec-shared-facade-home-in-core` — RATIFIED: one shared facade in `core` (both
  crates consume; core stays dependency-free) is the justified dislocation from
  `301` §3's crate-local default; dividend: one Kani harness set covers both crates.

[CONDUCTOR ratification, 2026-08-14] Nested-vs-root mise configs: toolchain-SHADOWING
pins live in nested configs (the in-repo Aeneas precedent); additive-only pins (elan)
may live at root.

[CONDUCTOR staffing, standing] Fable authors SPECS and reviews STATEMENTS
(`plans/302`; minispec content under the `301` access laws; harness-statement review
at fold); Opus authors bodies, tests, and toolchain wiring. Neither Kani nor the
certifier runs full-Fable or in-conductor-implementation.

### §2b — Late-wave folds (2026-08-15)

- **lane-sparing-rederivation (b) — FOLDED @ `0c1cefcd`** (four commits; builder gates
  green both legs 2008/2004 + bless:dry, zero drift; conductor gate over the combined
  tip at fold). As-built: the three model amendments landed with flags retired and the
  epistemic rider applied (the "proves" wording in the model's own variant doc
  sharpened too); `plan::rederive` (adapter: canonicalized entities, typed refusal on
  unresolvables, ∅-footprint = named differential failure; kind-fence pairing leaves
  entities unread — a real adapter bug the differential caught at seed 3);
  `recheck_survival` seated INSIDE `wall_walk_survival`'s Survived arm (a post-pass
  demote would un-wall downstream sites — the lane's one genuine correctness trap,
  correctly dodged), by-value witness in / Confirmed-or-Demoted out, demote-only
  pinned three ways; 8000-seed differential (two tests + non-vacuity censuses + a
  mutant control): ZERO production↔model disagreements. FLAG ADJUDICATIONS
  (conductor): `flag-backing-mintedness-is-translated` ACCEPTED (the backing-side
  dialect-membership conjunct is adapter-computed — the differential's one disclosed
  coverage limit; documented in the test header; revisit if the model ever grows a
  dialect-lookup of its own) · `flag-plan-gains-a-dependency` ACKED (dorc-sparing-
  reference production + dorc-hostsim dev; both in-workspace zero-dep) ·
  `flag-diag-not-in-identity-diags` ACCEPTED (the demotion rides the decision digest;
  double-count avoided). The proposed plan/CLAUDE.md bullets ride discipline-close.
- **THE MINISPEC REMIT IS REAL (conductor-authored, 2026-08-15)** — `TrustedBase`
  vocabulary (LawfulClone/LawfulEq as named trusted-base entries + the lawful-by-
  construction U32 battery ground) + the three law units (JoinIsCommutative ·
  JoinIsIdempotent · JoinIsAssociative — the third renamed from the LeqIsReflexive
  placeholder; derived-leq reflexivity is idempotence in disguise), stated over the
  DERIVED `Flat` join with hypotheses in the statements, batteries + non-vacuity
  proven by reduction (`rfl`), lake green (the 4 dependency-closure holes are the
  known aeneas-own trusted base, unchanged). Badges: elaborated + interrogated EARNED
  ×3, claimed through the promote ceremony (silent-ambition refusal observed working,
  then satisfied); the verified boundary renders its first real seat
  (`dorc_analysis::lattice::Lattice::join`). TWO CHAFE FINDS from authoring (the
  harness's first real user): `fnd-vocabulary-home-was-unrepresentable` — the binder
  had no governed-vocabulary concept and demanded a law of TrustedBase; FIXED
  properly (`Minispec/Vocabulary/` walk: unit-contract-exempt, hole-censused — a
  vocabulary hole vacates every importing unit; fixtures + two tests) ·
  `fnd-promote-subcommand-missing` — `catalogue_lock.rs`'s header names a
  `dorc-verify promote` generator that DOES NOT EXIST; the promote act is currently a
  sanctioned hand-edit (the header's own review-is-the-git-diff rule); the generator
  is residue for the harness's next builder.

## §3 — The Lean-tier vehicle (RESOLVED in substance, 2026-08-14)

- [TYPED] Aeneas is a must/of-course, if the tier exists at all: machine-correlation
  is the entire point where correlation is available, and the seam's brittleness under
  regeneration is the drift-alarm working, not a cost to engineer away.
- The maintained artifact is **minispec** (`notes/301`): hand-written statements +
  instances over Aeneas-DERIVED definitions, proofs where cheap. The earlier
  hand-model and Aeneas research spikes are QUARRY, never seed [TYPED].
- The recorded translation limits (sealed tiers, phantom `Must`/`May`,
  smart-constructor privacy do not cross) are compile-time discipline that rustc keeps
  enforcing over the real code — they never needed to cross; the Lean tier's job is
  equational law over the bodies, which translate faithfully.
- The churn-measurement question dissolved with no-pilots: the derived-defs pipeline
  is simply maintained; there is no vehicle decision left to gate on it.

## §4 — wave-one-close (the handoff gate)

[TYPED 2026-08-15] CLOSE DELIVERABLE, chat-tier: a brief **human-QA list** that
EXERCISES the arc's work — tests to break (and in what way), subtle incorrectness
to inject that must trip the new safeguards, specific CLI invocations
demonstrating the new tooling — optimized for minimal human effort × maximal
chance of surfacing unexpected choices, holes, mistakes, or underspecifications.
(Prior arcs smuggled sharp edges under flashy acceptance criteria via the human's
own limited exercise time; this is the counter.) Refined [human, 2026-08-15]:
items need NOT be single commands — fuzzier exercises are expected; anything
mechanically checkable the conductor runs itself pre-close, or routes through an
IDIOT-REVIEW lane (an unprompted agent, FORBIDDEN the docs, given only a goal,
recording what chafes — the loom blind-reviewer precedent generalized; plan one
over the dorc-verify/minispec flows at close). The human's list keeps only what
genuinely needs a human: judgment, taste, fuzzy-seam poking. The list itself is
conversation output at close, never a durable; THIS obligation note is what
survives compression. Conductor collects candidates per fold.

All lanes folded to `ai/main` · `mise run both gate:full-quiet` green + `bless:dry`
clean · certifier + re-derivation live in the DEFAULT suite · the Kani lane opt-in and
documented · minispec standing (skeleton; the remit claims at their earned badge-sets;
binder v0 + the generated report; the first bound demonstration; `minispec/CLAUDE.md`)
· the derived-defs lane green · CLAUDE.md discipline sections landed + prompt-reviewed
· ledgers current (this file, `notes/301` if amended, LIVING_STATUS, FORFEITS) ·
conductor worktree/branch cleaned or handed over deliberately. Successor boot order:
LIVING_STATUS → this file → `notes/301` → `plans/28Q` → `spike/CLAUDE.md`. The NEXT
stage's first acts: a full root `ANALYZER-NEEDS.md` read (it owes the `an-flat-domain`
reconciliation paragraph, `28Q` §7), then 28Q stage-i's fixtures-first commissioning
per `28Q` §8.

## §5 — Ack-ledger (what the human has TYPED this arc; silence is never ack)

- 2026-08-14, the plan-ack sitting: the six-lane Wave-1 plan ACKED (with the stated
  leans: rederivation-in-scope · vecdeque-stays · needs-ledgers-deferred) · codex
  dispatch ACKED ("use as you see fit") · the research branches deleted by the human ·
  round-30 minted, notes/300 assigned · the sequential-conductor/rewind protocol
  directed (§1).
- 2026-08-14, the design sittings (the reshape this §2 reflects): Aeneas
  must/of-course · the small reviewable surface is a core product (literate
  colocation; the rationale is LLM attention-forcing, per the errorloom precedent) ·
  model-writing is design-work — the spike models are quarry; minispec's remit is the
  2–3-claim minimum; enrichment is a standalone human-led item · the runtime checkers
  and formalization-as-question-generator hard-ACKED · no pilots / no measure-kill
  stages, velocity · the independent-voices/lineage framing deweighted — a checker's
  value is structural asymmetry (finder/checker under different constraints) ·
  out-of-scope is a human judgment, never machinery (taxonomy/strength-axis repairs
  nacked) · mutation-testing is a gentle-must (badge defined day-one, `301` §5);
  property-testing stays the general check-ladder, never a spec badge · an automated
  performance-regression lane (CI graphs + hard gates) is banked for someday, out of
  scope · the whylog decision record is the assertion substrate, under the [TYPED]
  framing that huge amounts of Dorc are modelable as a deterministic mapping from
  source through probe-results to whylog result · doc routing: notes/301 minted as THE
  minispec/verify spec; this file carries the rest; `plans/28Q` edits minimal; `28T`
  markers-only; `plans/302` = the certifier spec (renumbered under the routing).
  The 301-interior rulings (access laws, remit, badges, bindings, naming, byte
  tripwire, local-homing default) live in `301` and are not duplicated here.
- 2026-08-14, session close: Flux DEFERRED [TYPED] — penciled mid-r30 (post-Lean
  standup, pre-stage-i), defense-in-depth intent standing, with the
  required-changes-ride-Aeneas-prep exception (§2's facade rider).
- 2026-08-14, the greenlight sitting (post-rewind): certification machinery ruled
  sketch-until-demanded (`301:post-certification-sketch-until-demanded` — architecture
  + cheap tooling + named seams; upfront depth is conductor/builder latitude) ·
  imported tools built general, never minispec-scoped
  (`301:law-imported-tools-built-general`) · proceed GREENLIT, conductor discretion
  ("I am here to work with you").
- 2026-08-14/15, the certifier duck-sitting (sibling-conductor context; product =
  the `plans/302` recut): NO RECOVERY [TYPED — "nothing here to salvage; it doesn't
  get a row": no FORFEITS entry; rationale distilled in `302` §9, the one recorded
  direction-not-taken] · aid-leads-the-engine [TYPED, standing, product-wide: aid is
  part of the correct output; the certifier triggers, the engine self-reports] ·
  "whole-window demotion" is the term (the conversational "kill" retired, never used
  in durables) · the naming set RULED: `SolveConsistency`/`Consistent | Inconsistent`
  · component stays "solve-certifier" · `Inconsistency{Boundary,Edge}` items ·
  `SolvePass` reason enum · `solver-consistency-failure` DiagCode ·
  first-break/unstable-component summaries · replay vocabulary · floors unchanged ·
  old notes left un-annotated by choice (rot endemic; r30 made current instead).
- 2026-08-14, the builder-prerequisite dictum [TYPED]: landed as durable law in the
  conductor skill itself (its quarantine section; human commit). §2 carries the
  pointer; successor conductors get it from the skill at boot.
- 2026-08-14, certifier-spec HOLD [TYPED]: the human is reworking `plans/302` with a
  sibling conductor. Everything certifier-shaped HOLDS until it settles: the phase-1
  proposal agent completes as read-only recon (its census/floor-table halves are
  spec-independent; its type-shape half will be re-cut against the settled spec), but
  NO phase-2 greenlight, and lane-sparing-rederivation stays queued behind it. The
  `302` §3 refusal-loudness [HUMAN?] flag is dropped as pending — it resolves inside
  the human's sitting. This conductor's `302` is superseded-in-place by whatever the
  sitting produces; re-read before any certifier act.
- 2026-08-14, phase-1 LANDED under the hold → **`notes/303`** (census · the four
  existing Refused-floors · eleven spec findings, several of which correct `302` as
  written — the sitting's input). Kani-lane list grows one row from it:
  `303:fnd-reach-equality-excludes-its-cause` (`Reach::eq` is trusted by the
  certifier with nothing pinning it). Lane branch `ai/r30-lane-certifier` exists,
  clean, zero commits — the phase-1 agent's context resumes for phase-2 if this
  session survives the hold; else redispatch fresh against settled-`302` + `303`.
- 2026-08-15, **rul-whylog-is-the-spine** [TYPED substance]: the whylog is the CORE
  PRODUCT — every engine decision passes through it, and every other surface (the
  plan render first among them) is a filtered VIEW over whylog × input-files. The
  standalone "decision-dump" DISSOLVES into whylog enrichment: full `SiteId` keying
  of the decision digest (the `leaf: u32` member-collapse fixed), phase-growth
  minting (a plan invocation mints the decision half; apply appends its report), and
  a read surface reachable from test/loom contexts. Conductor-stated boundaries,
  accepted as refinements not nacks: (1) the spine records DECISIONS and their
  scalar/interned accounts — never arena handles or working lattice state
  (`operands-are-pure-and-capped` generalizes to the file; the certifier's by-value
  items stay in-memory/pull-tier, F8); (2) `law-whylog-is-sensitive` stands —
  enrichment adds engine-derived rows only, host-sourced material stays in its typed
  intake lane within the file, the secrets-round re-grade before real estates still
  binds; (3) rec-5 stands — consumers read the whylog OF THEIR OWN RUN, never a
  stored one; the spine never becomes the kSTATE cache by accretion. Consequences
  banked: the records-8 pending decision leans WIRE (a spine wants its emitters);
  renderer-consumes-whylog is a banked refactor direction, opportunistic, never a
  big-bang; enrichment work stays demand-driven per sketch-until-demanded (the
  first product-behavior law binding is the natural trigger).
- 2026-08-15, certifier-lane worktree incident: the phase-2 builder's UNLOCKED
  harness worktree was reaped mid-task (the locked reshape worktree survived);
  zero commits lost, nothing mutated; the builder's near-miss (silent cwd
  fallback into the conductor tree with a `reset --hard` queued) is now law —
  `spike/CLAUDE.md` worktree-file-access-law sharpened (git -C absolute paths;
  verify before every mutating git command; stop-don't-improvise on a vanished
  tree). Builder resumed with a conductor-directed self-minted worktree.
- 2026-08-15, THE WSL-DEATH INCIDENT (post-mortem, cause settled): the harness
  process (claude.exe, child of a WSL-zsh) died with the human's terminals when the
  WSL2 VM OOM'd under the Kani lane's CBMC solver load — a measured 3.6GB/21min CBMC
  earlier in its window; SIX further runs stopped mid-climb; and the demonstrated
  hazard that **TaskStop does not kill WSL-side CBMC** (orphaned multi-GB solvers
  accumulate; reap explicitly with `pkill -9 -x cbmc` — exact-name, never `-f`,
  which once matched the lane's own wrapper shell). At death, an unmeasured
  37-harness battery was running into exactly the blow-up shape. No shutdown
  command was ever issued by any lane; the rederivation lane is exculpated (zero
  WSL contact; full clean ledger). RULINGS: the implicit
  `rustup toolchain install nightly-2025-11-21` into `~/.rustup` (fired by kani's
  own first-time setup, undisclosed-until-after by the tool) is WITHIN the
  pre-authorized kani-setup exception — disclosed properly, additive, reversible;
  keep it, document it in the lane's toolchain notes. Standing discipline from
  here: WSL-heavy lanes run SERIALIZED (one lane's WSL work at a time; conductor
  sequences); the Kani battery resumes only under a hard per-harness timeout + an
  explicit CBMC reaper (the lane's own proposal, made mandatory); its three probe
  cache dirs get deleted at resume. Discipline-close item: the
  TaskStop-does-not-kill-remote-children hazard + the exact-name-pkill rule belong
  in `spike/CLAUDE.md`'s build/run section. The blow-up fix itself is already in
  the lane's tree and measured (capacity-headroom generator: 21min/3.6GB → 2.2s).
  Forensics sharpening (scout, +SURE on kernel-log evidence): the fatal CBMC grew
  to ~15.16GB RSS — the ENTIRE WSL VM budget — and was the OOM-killer's sole
  victim (single-process blow-up, not a swarm); the terminals actually closed
  ~90s LATER via a `wsl --shutdown`-shaped teardown of the session tree, issuer
  unrecorded (~SUSPECT a reaction to the thrashed VM; Windows logs don't capture
  `wsl.exe` invocations). The box has NO `.wslconfig`, so the VM defaults to
  ~15GiB of the 31.7GiB host. OPTIONAL human hardening item: a `.wslconfig`
  memory raise buys headroom, but the lane disciplines are the real fix (a
  runaway solver eats any cap).
- Standing carry-overs: the `KNOBS:kSURVIVAL` status-line edit remains the human's
  (28T inheritance); silence ≠ ack; only typed text counts.

## §6 — The settled-rules census (BANKED; the enrichment item's tabled menu)

Gathered 2026-08-14 by a criteria-driven scout over KNOBS · `spike/CLAUDE.md` ·
`crates/{core,analysis}/CLAUDE.md` · FORFEITS · `277` · `271` · `28Q`;
conductor-adjudicated. The MENU IS TABLED [TYPED] — selection happens at the
enrichment item, never before. Criteria: explicit ratification evidence ∧ statable as
value-algebra ∧ off 28Q's moving edge.

Passing (evidence as found):
- `ternary-compare-consumer-map` — acked (`271` task-12 closing sweep 2026-07-12;
  `277` §9). Caveat: the relation shape + consumer map only; the fuller generator
  registry is still conductor-proposed. Named in `28Q` §6's preserved wall.
- `set-lifting-universal-meet` · `pin-set-meet-order-independence` ·
  `pin-no-outcome-as-generator` — ACKED, typed, 2026-07-16 (`277` §5 / the `279f`
  ack batch). The first is named in `28Q` §6; the third is not individually
  (~SUSPECT rider).
- `inv-backing-set-nonempty-by-construction` · `inv-top-never-encoded-as-empty` —
  acked 2026-07-17 (`27Xf:cr-set-lifting-vacuous-at-empty`); the measured
  vacuous-∀ design-bug class.
- `never-derive-separation` — acked "spike-tier-because-foundational" (`271`,
  2026-07-12); named in `28Q` §6.
- `top-identifies-with-nothing` — WEAKEST evidence in the set: "unchanged" across
  three rounds, NO dated typed marker found anywhere. Candidate calibration probe for
  the enrichment item's question-router (it should ask for confirmation).
- `rul-coordinate-shape-flat-three-place` — typed (`271`, 2026-07-10); light moving
  edge only (`28Q` §3 extends the context slot; the flat shape itself unchanged).
- `silence-licenses-nothing` · `inv-top-reject` — named unchanged in `28Q` §6.
- Settled but STRONG moving-edge (excluded from near-term proving): `rul-family`
  (typed, but `28Q` §1/§2 reshape membership frame/closure-relative) ·
  `pure-predicate-carry` (human-opted 2026-07-17, but `28Q` §3 grows its axis
  vocabulary).
- Settled but not value-algebra: `empty-world-byte-identical` (whole-system
  differential property; its evidence stays the corpus differential).

Conductor adjudication deltas:
- `inv-must-may` SPLITS: the coercion ban is compiler-tier (evidence = the
  `compile_fail` seals); the `Must`-as-order-dual SEMANTICS is genuine value-algebra
  and underlies the certifier's one-checker duality.
- MapL canonical-form (structural-Eq = semantic-Eq) fails the scout's
  settledness-marker criterion but enters anyway via the facade lane as its
  honour-system invariant.
- `rul-rc-partition`'s ≥2-flat-sink ("flat FOREVER") is borderline algebra-content;
  benched.

Excluded as not-settled (soft/forfeit/refused): the sparing dialect-resolution core
(typed-spike-provisional + acked-SOFT + `pin-two-position-sparing` extremely-soft +
its FORFEITS row) · `forfeit-committee-fence-sparing-inert` (UNRATIFIED) ·
`kind-fence-movable` (a reserved seam, not a ruling) · the `275` transport
ratifications (REFUSED, `279f` §3).

Doc-coherence note, repaired in `28Q` §6 this arc: "the sparing algebra" in the
preserved wall means the set-meet SUBSTRATE (hard-acked, above); the
dialect-resolution rule is `28Q` §9 `pin-two-position-sparing` territory (soft).
