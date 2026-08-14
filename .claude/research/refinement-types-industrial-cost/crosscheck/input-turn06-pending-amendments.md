<!-- Verbatim copy (2026-08-13) of the conductor session's post-bank amendments ledger,
     copied from session scratch into the repo so read-only reviewers can reach it.
     Treat as plan-state that SUPERSEDES turn06 where they disagree. -->

# Pending turn06 amendments (accumulating post-bank rulings; fold in when human calls for the update)

## amendment-survival-reprioritized (human nack of expl-survival-lane conclusion, typed)

- HUMAN RULING (typed): the survival lane / elide-half is the CORE PRODUCT, not a minority
  tier. Plane-seats metaphor: elision = seats (what people buy); guarding = luggage
  (heavier, costs most of the logistics, table-stakes, nobody buys a ticket for it). Low
  fire-rate/user-count is balanced by user-retention-when-right and user-pain-when-wrong.
- HUMAN RULING (typed): "more dangerous, so it gets more attention, always" — the survival
  lane already mingles world-chaos risk + user-claim risk; adding a third axis (engine bugs
  in OUR composition algebra) is unacceptable; our part must be MOST correct exactly there.
- HUMAN RULING (typed): elision will never be unshipped — that is a product-is-dead outcome,
  not a design decision. Consequence: kSURVIVAL's "does the tier earn its keep" field-trial
  framing is reframed — the fires-often x bites-rarely measurement decides TUNING/marketing,
  not existence. (Knob status-line implication; human's edit to make if they want it.)
- CONDUCTOR CONCESSIONS: (a) the operational load-bearing test (what depends on it today)
  was the wrong test for design-time assurance — for a pre-release product nothing is
  operationally load-bearing, which would absurdly defer all pre-field verification; correct
  weighting = severity x invisibility x permanence, survival maxes all three; (b) internal
  contradiction conceded — conductor argued model-before-code is where formal methods pay
  best (Cedar/26C) then gated the survival model on post-field evidence.
- CONDUCTOR SHARPENING (accepted into the analysis): an engine bug in the sparing algebra is
  not merely under-execution — it is MIS-ATTRIBUTED under-execution: the why-chain fingers
  link 4 (the author's claim) when the true cause was link 5 (our derivation) — pope-sin tier
  per 271:rul-sin-ordering, AND it poisons the oracle-author trust loop the ecosystem
  bootstrap depends on ("you trusted named authors' claims" becomes false — we also made
  them trust our bugs, billed to the authors).
- SURVIVING STRUCTURE: the three-way decomposition stands (claims = permanently unprovable,
  bought; composition algebra = ours, provable; authored-surface protocol = DST/framing
  territory). Only the ASSURANCE TIER AND TIMING of the middle piece moves.
- REVISED PLAN for the sparing/composition algebra (replaces the field-trial trigger, which
  DIES): staged by churn-cost —
  1. NOW: Kani/property pins (already portfolio item 2);
  2. NOW: an executable REFERENCE MODEL of the sparing algebra in plain naive Rust
     (obviously-correct, ~small), internal-differential against the production
     implementation under DST permutations;
  3. PRODUCTION SELF-CHECK: plan-time sparing re-derivation — every survival's disjointness
     verdict re-derived through the independent reference implementation before the plan
     ships; disagreement ⇒ demote to guard/run + operand-carrying narrative record
     (Refused-shape). NOTE: this does NOT breach the no-runtime-net doctrine — that doctrine
     bars HOST-side re-checking that re-spends user attention; this is controller-internal,
     plan-time, zero user-visible cost, and does not touch the bought-risk contract (claims
     stay trusted; only OUR derivation is double-checked). CHECK against
     271:rul-net-quality-u-curve before building (imperfect-mechanical-nets warning; a full
     redundant re-derivation is an exact computation, not a heuristic net, so ~SUSPECT it
     passes — but check).
  4. AT ALGEBRA-SPEC SETTLE (the new trigger — the settling event, NOT the field trial):
     promote the reference model to Lean/Dafny and prove the laws (universal-meet safety,
     order-independence, pin-no-outcome-as-generator, canonicalize-both-sides, non-empty
     backing sets) + the attribution-correctness obligation (why-chain names the right
     licensor — the narrative-30 duty).
- RESIDUAL CAUTION (kept, not relitigating): a formal model of a still-moving algebra pays a
  sync tax proportional to design churn; the staged shape pays it in cheap currency (Rust
  reference churns easily; Lean waits for settle).
- Explainer #2 delivered and REVISED by this dialogue; task 2 complete with this amendment
  as the corrected conclusion.

## amendment-aeneas-pulled-forward (human lean, typed)

- HUMAN LEAN (typed): Aeneas-over-handwritten-Lean strongly preferred; willing to pay
  DST-shaped discipline costs AND an upfront rewrite of the small provable core; wants
  immediate-or-near-immediate rather than triggered-later. Rationale partly conveyed
  in-chat and deliberately unrecorded (human instruction).
- ARCHITECTURE AS CORRECTED/RESTATED (conductor, acked shape pending): four artifacts —
  (A) bulky production engine (untrusted, churny); (B) runtime checkers in Rust, in
  production: the solve-certifier (checks each ANSWER once per solve, per-edge; NOT a
  re-implementation) + the sparing reference re-derivation (a genuine naive second
  implementation of the algebra, diffed per survival verdict at plan time); (C) the
  disciplined small core (LatticeMap facade + checker + reference algebra) written in
  Aeneas-translatable Rust; (D) Lean proofs written ABOUT the Aeneas-TRANSLATION of (C) —
  Aeneas translates code, it does not extract proofs; proofs are authored (agents) against
  the translated definitions; crown-jewel theorems: certifier-accept ⇒ post-fixpoint ⇒
  path-coverage (the domino lemma), and the sparing-algebra laws.
- LINKAGE LADDER (the human's "shakiest link" concern, answered): (1) small core → Lean:
  MECHANICAL — CI re-runs Charon/Aeneas translation on every change; semantic drift breaks
  proofs; (2) bulky engine → small core: PER-RUN — the runtime checkers validate every
  actual answer in production (stronger than compile-time in that it covers real
  executions; weaker in that it covers only executions that happen); (3) statistical —
  DST + property tests + Kani. PROPERTY TESTING SCOPED IN at human direction: proptest-
  style randomized structural inputs banging engine vs checker vs reference, between
  Kani's exhaustive-small and DST's whole-system tiers. Residual trusted surface, named:
  Charon/Aeneas translation itself (treat like rustc), Lean kernel, the shared lattice-ops
  (Kani-pinned), and the model/transfer layer (explicitly out of scope for all of this —
  calibration territory).
- REVISED SEQUENCING: immediate-with-a-spike-gate, not immediate-blind. The de-risking
  spike (days): carve the core behind the facade, run Charon/Aeneas on it as-is, read
  AeneasVerif/jxl-proofs (the non-crypto proxy), report what translates and what falls
  out. Go/no-go on empirical result, 021 Step-minus-1 style. Pin discipline: vendored
  three-way toolchain pin (rustc-nightly/Charon/Lean) in a dedicated opt-in lane
  (real-tools-lane shape), upgrade cadence chosen by us, proof-repair sessions budgeted.

## amendment-lean-spike-results (builder complete; adversary KILLED by human pre-start; adjudication not run)

- BUILDER RESULT (branch ai/research-lean-sparing-spike, 6 commits ddabc5db..75b3ab60, base
  9b05283f; artifact + committed REPORT.md in its worktree spike-lean-sparing/): ALL FIVE
  theorem families PROVED, zero sorry, axiom profile exactly [propext, Quot.sound] via
  #print axioms; no_outcome_as_generator = API-shape argument (Verdict-free evidence types +
  fail_if_success tripwire); bonus: sparesSet_iff_universal; the vacuous-spare hazard
  reconstructed as an inhabitant of the unguarded encoding. SEVEN spec-gaps + two notes;
  top-3: (1) compare symmetry-vs-directionality (+SURE; only backings carry a minting family
  so the relation is position-typed — CONVERGES with 28R:adj-sparing-two-position-rule:
  strong signal the asymmetry is load-bearing); (2) top-selector self-sameness
  (definite-whole-entity vs failed-top share one encoding; "top identifies with nothing"
  then refuses same for identical whole-entity coordinates — a cost the spec never
  acknowledges choosing); (3) footprint-side empty-set unpinned (§5's empty-set invariants
  are backing-side only; the meet is vacuous over an empty FOOTPRINT too — proved; needs the
  symmetric pin).
- TOOLCHAIN (merge-candidate if accepted): Windows-native ~35min, elan-under-mise
  ("ubi:leanprover/elan" 4.2.3, exe elan-init; ELAN_HOME + path templated to xdg_data_home;
  Lean pinned per-project via lean-toolchain v4.33.0 core-only; resolves on WSL leg;
  one-time: mise exec -- elan-init -y --no-modify-path --default-toolchain none). Direct
  ubi:lean4 fails mise bin-path detection; ubi:->github: deprecation respell untested.
- LLM-LEAN VIABILITY: strongly positive on OUR theorem shapes — 3 compile round-trips, 2
  errors total, both tactic-level (vs the 27%-union benchmark prior). Builder's correct
  caveat: checker validates proofs never statements; all gaps found pre-checking; human
  attention belongs on statement review by spec-owners.
- CHAIN STATE at ledger-write: unprimed adversary was dispatched (forbidden REPORT/README +
  git log; would re-verify CHECKED by building) and KILLED BY THE HUMAN before producing
  anything; conductor adjudication NOT run; Aeneas lane killed twice mid-setup and PARKED
  (conductor guess, unconfirmed: rustup's user-global ~/.rustup breached containment
  expectations; a re-run wants RUSTUP_HOME/CARGO_HOME redirects or explicit blessing). ALL
  autonomous dispatches HALTED pending human direction.
- KERNEL-CURRENCY SCOUT (for the fold): 28Q context-kernel unification ACKED, stage-0 built,
  stages i-iii designed-only BLOCKED on two human rulings (28R:adj-helper-closure-frame-
  plurality; 28R:adj-closure-identity-undefined). Certifier + map-eviction targets
  UNAFFECTED (lattice.rs/solve.rs untouched since round 19; zero LatticeMap collisions).
  Aeneas baseline VALID pre-context-work; re-measure after stage-i/ii. 277 textually current
  but amended by 28R rulings (instantiation-hash-dedup [ACKED]; mixed-custody-suspends-vouch
  [TYPED-BUILD]; two-position-rule [CONFIRMED-open]); 28Q's "computed once, whole-unit"
  sentence ruled DEAD (rul-resolution-matches-shell-loading) but not yet deleted from plan
  text; import 28R:fnd-pessimistic-pass-shape as a certifier design constraint. The one-time
  handoff file was extracted into notes/28S and deleted.
