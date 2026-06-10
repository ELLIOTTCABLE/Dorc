# 210 — The soundness whack-a-mole: prognosis + deterministic countermeasures; st-1/2/3 judged

> Round-20. §1–§3 answer the human's direct question ("where is this headed; what are the
> robust/deterministic options?" — excluding constrained-input and fuzz-at-scale by his
> instruction). §4 records the three delegated judgments (st-1/st-2/st-3 from the steering
> summary). AI-authored, confidence-marked; §1–§3 are analysis for the human's direction-setting,
> not rulings.

## §1 Diagnosis: two families, different cures

- **fam-A — conservatism-removal reveals latent wrongness.** The poison wall / ⊤ floors were
  load-bearing for soundness in unenumerated ways; every precision increment (selector re-key,
  set-blessed-Pure, find-3 removal, future: Query folds, loops) activates previously-dead paths,
  some wrong. 196 §2 stated it as law; rounds 17/19/20 each re-instantiated it. The system's
  correctness was never established — it was vacuously protected.
- **fam-B — wrong models of shell semantics.** Hand-written beliefs about dash that are simply
  false: "converged ⇒ rc 0" (19A §2, thrice), errexit-not-a-reader (C-3 violation), prefix-env
  argv visibility (this round, accepted in review, caught by crosscheck), `${N#pat}` literal-vs-
  glob (ditto). Nothing structurally connects the engine's model to dash's reality; each
  re-implementation of sh semantics (book parser, errexit pass, value-plane expansion, dialect
  evaluator, render quoting) is an independent divergence surface.

Prognosis unmitigated (+SURE on direction): the 209 enrichment roadmap multiplies fam-B surface
(word-splitting, IFS, expansions, loops, functions are ALL semantics-modeling); fam-A fires at
every floor-removal. Observed fam-B injection rate this round: ~2 disaster-class bugs per ~2k
lines of new precision code, surviving orchestrator review, caught only by Fable crosscheck —
which is expensive, sampled, and aimed; it does not scale as the primary defense.

## §2 Countermeasures, ranked by (early leverage × determinism × long-term reliability)

- **cm-1 · The differential product-gate (DST-of-the-product) — the one to build.** We possess
  both artifacts the core promise relates: the bare book and the rendered apply. Run BOTH under
  the same hermetic mocked dash (the e2e mock machinery, already built), and assert the
  relation the four-outcome lattice promises — every command the bare run executes is either in
  the apply's run-set or carries an elision-license whose probed facts the harness itself fed;
  probe artifacts' run-sets ⊆ declared-read-only commands (vouch-closure, executable). Make it
  TOTAL, not sampled: per book, enumerate the full host-state space over its modeled facts
  (2^n, n ≤ ~6 for corpus books — exhaustive, deterministic). Catches any wrong-elision
  REGARDLESS of which component lied (no shared blind spot with the engine: the oracle is
  execution, not re-analysis), and — the key property — **its coverage scales automatically
  with the enrichment roadmap**: when loops/functions/Query-folds land, a wrong elision
  surfaces as a run-set divergence with zero new gate code. fam-A's "what was the floor
  hiding?" becomes a disposition-diff you READ instead of archaeology. Buildable this spike on
  run.sh's bones.
- **cm-2 · dash-as-semantic-oracle for the value-plane (argv-echo differential).** fam-B's
  source-cure for propagation: per corpus book, replace every command word with an argv-logging
  shim and RUN the book under hermetic dash; the logged argvs are ground truth for `value.rs`'s
  per-site resolutions on the executed path. Same trick for the dialect evaluator: run the real
  check() under dash with the same argv (annotation rendered to assignment + logger) and
  compare entity/verb/arm selection. The prefix-env bug dies by construction under this gate.
  Deterministic, one dash run per case, rides the same harness. Limitation: validates executed
  paths only (branch coverage = the corpus's job).
- **cm-3 · One shell-semantics module.** Structural: collapse the N hand-rolled re-implementations
  of expansion/quoting/splitting/special-params into ONE module consumed by parser, value-plane,
  evaluator, and render — validated once by cm-2, reused everywhere. Reduces fam-B surface from
  O(components) to O(1). Cheapest now, while the code is young; compounds forever.
- **cm-4 · Externally-checkable certificates (small trusted checker).** The witness pattern
  upgraded: each run serializes its Derivations (this elision rests on probed-fact F, consumed
  channels C, stand-in S); a SEPARATE small validator — dumb, stable, human-reviewed once,
  agent-untouched — re-checks certificates against the artifacts. Concentrates trust in a
  kernel small enough to actually review; the big engine becomes untrusted-by-design (the
  agentic-construction thesis taken to its conclusion). Caveat honestly: a checker that
  re-ANALYZES shares blind spots with the engine (consumption-coverage holes migrate into it);
  its strength is production/run-time checking where execution-grounding (cm-1) isn't
  available. Sequence after cm-1.
- **cm-5 · The precision-increment protocol (process, near-free).** Removing any floor/⊤/poison
  requires: run cm-1's gate before/after, diff dispositions; every case that newly elides needs
  a positive one-line justification in the round note. Turns 196 §2's law into a checklist with
  a deterministic input.
- **cm-6 · Total-dispatch discipline extended.** The errexit hole was an ABSENCE (nothing forced
  the "who reads status?" question). Pattern: every semantic dimension = a closed enum +
  exhaustive match + deny-by-default arm, and a single registry site per channel enumerating
  its known consumers — completeness stays unprovable, but every item becomes an explicit,
  greppable, crosscheckable decision. Incremental; already half-practiced (the lowering's
  no-catch-all rule).
- Horizon (named, not proposed): bounded-exhaustive model-checking of the lattice relation
  (Alloy-flavored small-scope) is cm-1's academic grown-up; kVERIFY-calibrate ("TypeScript, not
  Coq") welds out the proof-assistant pole, and cm-1/cm-2 ARE the calibrate-pole's strongest
  practical form.

## §3 The strategic reframe

Move soundness-checking from REVIEW-time (probabilistic, expensive, aimed — crosschecks) to
HARNESS-time (deterministic, total over the corpus, free per-commit). Crosschecks then return
to their real job — design judgment and unknown-unknowns — instead of being the last net under
semantics bugs. Division of labor that results: deterministic machinery proves "Dorc never
contradicts dash + its own declared facts" (everything this round's bugs violated); oracle
quality stays empirical and shelf-lived (rul-mutation-impossible, permanently out of machine
reach). ~SUSPECT the right build order: cm-2 (days, kills the active bleeding) → cm-1 (the
keystone gate; some harness design) → cm-3 (refactor as the enrichment rounds touch each
component anyway) → cm-5/cm-6 (process, immediate) → cm-4 (when a production runner exists).

## §4 Delegated judgments (recorded; reasoning in the chat log)

- **st-1 ruled: short-form kinds ARE the kind, this spike.** The engine treats kind strings as
  opaque tokens — nothing depends on dots — so the "papering" is harmless-by-construction; the
  reverse-DNS form's job is ecosystem-scale collision-avoidance, which is the deferred
  res-collision/res-curation design (social/namespacing), and inventing an alias mechanism now
  would be namespacing-by-accident. 19H §2's reverse-DNS examples stay aspirational vocabulary.
- **st-2 ruled: probes stay a separate declaration this spike; the check() resolves identity
  only.** The dialect does NOT grow pipelines (today's findings make vivid that every dialect
  production is new dash-divergence surface); the check's probe_body spans stay
  computed-but-unused; per-selector probes continue in whatever declared form the corpus
  already exercises. This deliberately retreats from 19H §2's "the check IS the probe body"
  unification — recorded as revisitable at dq-kOOB (the oracle-spelling ruling, the human's),
  where the two-declarations-vs-one question properly lives. rule-anno-render still applies to
  whatever ships.
- **st-3 ruled: pristine-prefix stands, with the one free sound refinement — upstream QUERIES
  and blessed-pure builtins do not invalidate downstream queries (reads don't write; the
  guard-stack idiom `command -v a || …; command -v b || …` keeps all its folds).** Only an
  upstream oracled MUTATOR or Opaque command invalidates. The update-first cost stands until
  cross-kind dependency edges exist (linked: recursive algebra + dir-timed-probe prerequisite).
  Same-kind-only softening stays rejected (demonstrated unsound, 205 §2).
