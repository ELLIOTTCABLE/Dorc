# 236c — independent review of the 233 crisis-document (adversarial-but-honest pass)

> Working notes for a from-clean-slate review of `plans/233-rubber-ducking-the-oracle-contract.md`.
> Constraints honored: human-docs prioritized; corpus read only up through round-22 (+ the permitted
> `plans/230` seed and `notes/218a`/`plans/21W`/`plans/22H`); NO other 23x-series documents read
> (231, 232, 234, 23Z untouched); quarantine untouched. Confidence marks +SURE/~SUSPECT/-GUESS/--WONDER.
> AI-generated; process evidence, never proof. Findings slugged `236c-fdN`; forks `236c-forkN`.

## §1 Understanding of the product (pre-233 baseline), as read from human docs

Sources: README, DESIGN, IMPLEMENTATION, KNOBS, TODO, TODO-ADDTL, Research/README, 22W, 230, 17N,
218a, 21W (doors-relevant parts), 22H, STALENESS-AUDIT (dir-soundiness-ux), spike/CLAUDE.md +
crates/analysis/CLAUDE.md + effect.rs source.

- Product: Ansible-alike where books + oracles are idiomatic POSIX sh; the analyzer lifts oracle
  checks into a non-mutative massively-parallel probe phase, then elides/replaces converged
  mutators in the apply phase, presented as plan/apply UX.
- Priorities (DESIGN, ordered): (1) correctness *within contracts*; (2) low user effort vs value;
  (3) cross-network wallclock perf; (4) invisibility. The founding complaint (DESIGN "Dorc's
  approach"): "it can be quite silly to actually *execute* a deep tree of these check-then-execute
  blocks"; the founding promise: the plan "reduced to one or two shell commands."
- Welded: kLANG (sh-is-the-product; trivial off-ramp), kFAIL phase-keyed, kVOLATILES-exclude,
  kVERIFY-calibrate, kDEPS-accept-partial, kAGENTLESS-push.
- Execution ladder (IMPLEMENTATION): never under-execute > avoid over-execute > avoid
  unnecessary-execute; floor = "no worse than just running the script, blind."
- The gradient criterion (human): gradient iff partial-benefit exists; probe-mutation has NO
  gradient; apply-coverage HAS one.
- Standing round-20 rulings (spike/CLAUDE.md): mutation-analysis impossible permanently; TOCTOU
  re-probe-before-apply deferred-to-actively-WONTFIX; no intra-host reordering, apply-phase speed
  comes from elision only, probe-phase parallelism is where wall-clock is won; identity declared
  never inferred.
- Round-21 doors history (21W, 218, 218a): door-4 = guard-insertion, designed in full (218a),
  ruled (dq-errexit-3, human): CLI-flag-gated, default `Never` provably-zero-transforms, builds
  LAST, product hard-defers; rationale includes the trust-boundary taxonomy "a bad oracle must
  never cause novel apply-phase actions." dq-errexit-2 (who owns the bare middle: oracle-default /
  engine-global / admin-per-book) ruled GENUINELY OPEN, human "unconvinced" by oracle-default.
  218a hunt-A: claim-noop is FALSE for the flagship oracle as naively written (apt-get install
  upgrades outdated packages); "if the canonical oracle can't honestly declare, door-2/4's
  reachable population collapses and the design must say so."
- 22H (live-plan, deferred): "classify once, re-fold per batch is sound"; the human leans STATIC
  apply-scripts; re-probe is "apply-script-embedded, far-future."
- STALENESS-AUDIT L100-104: elision was ALREADY known to be "gated multiplicatively on upstream
  oracle coverage (one un-oracled reaching mutator poisons everything below it)" since spike-2
  (193 strain-5). The wall is old news; 233 formalizes its consequences.
- kSILO (KNOBS pseudo-knob): where correctness-code accrues (books vs oracles); the "biggest
  near-term shover" is which half the UX visibly rewards; watch-item, high stakes when baked.

## §2 The 233 problem-half: VERIFIED, and genuinely two-horned (+SURE)

- The §0 headline unsoundness is real at HEAD, verified in source: `effect.rs` `reach_transfer`
  gens ONLY declared cells (`Establishes/Kills ⇒ state.with(fact)`), `Opaque ⇒ join(Top)`
  poisons all; `command_effect` returns `[Opaque]` for no-oracle/no-effect-row/⊤-operand. So:
  un-oracled command = safe poison-all floor; modeled-but-partial oracle = silence-as-license =
  the §0 wrong-elision. 233's own line-359 correction ("safe floor, dangerous middle") is the
  accurate framing; the opening "broken as documented in the corpus" is half-right — the corpus
  documented BOTH horns in different places (an-effect-class location-sets = poison-if-undeclared;
  the take-3 keystone re-key "what kills the poison wall" = silence-as-license) without ever
  confronting the completeness question that makes them collide.
- The project's own history walked both horns empirically: spike-2's flat FactKey = everything
  poisons = nothing elides on a realistic book (16Q keystone); take-3's per-entity re-key = the
  §0 unsoundness. This is strong evidence the dilemma is fundamental (the frame problem), not a
  patchable bug. hard-1/hard-2 are correct as stated.
- The four approaches (§1-§4 of 233) genuinely exhaust the *declaration-only, single-shot*
  design-space corner: they vary only WHERE the closed-world assumption sits (per-property ACK /
  per-type vouch / silence-trust / kind-scoped hybrid). What they share — the unexamined premise —
  is that elision-licenses must come from *predictions* (declarations about what interposed
  commands will do), never from *re-observation after the interposition*. See 236c-fork1.
- One enumeration gap inside that corner: the fluent-centric inversion (each cell/kind declares
  its invalidator-set; dependers declare cross-kind sensitivity — exactly 17N §9 opt-3 "per-facet
  + cross-facet invalidation") is absent from the approaches list. It fixes story-1's social shape
  (the scan_cve author, who owns the fact and the incentive, declares "package-mutation kills
  cve_clean" — the apt author never needs to know the future) though not story-2 (opaques). Not a
  dissolution of the dilemma; a materially better burden-distribution within it. 233's "I don't
  see what else to do" slightly over-despairs the enumeration.

## §3 The proposed solution's mechanics: mostly SOUND, and better-hedged than the prompt implies

Honest credit, verified against the corpus:

- The ternary {elide, guard, run} verdict + in-sequence `check || cmd` guard is round-21 door-4
  (218a), a carefully-designed mechanism: `||`-left errexit-exempt; preamble-function shipping;
  output-silencing at call-site; name-collision refusal; already-guarded refusal (no accretion,
  admin's hand-guard wins); the four-world trace. 233 inherits this design and says so.
- Per-site, a guard is STRICTLY SAFER than the static elision it replaces: world-2 (diverged
  since probe) immune by re-measure; world-3 canary-suppression NARROWER than static elision
  (probe-visible sicknesses fall through to the real mutator). 218a u-4 already observed door-4
  ~dominates declared-static elision. The re-measure move is the project's own epistemics
  (observation over prediction) applied at the site.
- The guard-license (converged-vouch) is the correct answer-shape to 218a hunt-A: re-type the
  license from fact-claim ("re-run does nothing" — refutable, and 233 refutes it) to attributed
  fallible judgment ("skippable-when-converged, per oracle X"), floor = the hand-written
  `check || cmd` idiom. The fence (vouch licenses ONLY its own command's sites; witness-type;
  never enters the fact-plane) correctly prevents the *mechanical* re-laundering of local
  judgments into global non-interference.
- Monotonicity within the guard tier is genuinely restored (silence neither vouches nor
  collapses; the hork floor-oracle stops being a landmine). The §0 dilemma's *default* is
  genuinely dissolved: full-poison now degrades elide→guard instead of →run.
- The downsides list is unusually honest (attention-product dead past opaques; check-tax;
  ineligible site classes; the vouch tier as permanent sharp-knife; the posture/weld changes
  named for conscious re-welding).

## §4 Findings — where it breaks, overclaims, or must not hold (the review's product)

### 236c-fd1 (STRONGEST INTERNAL INCONSISTENCY) — the elide tier is rebuilt on the exact
### epistemic object the document itself refutes same-day
233 kills the universally-quantified guard-license ("when converged, re-running does *nothing*")
as "vacuous as human testimony — its quantifier ranges over exactly the observables the author
never attended to" (the hork-writes-into-apt's-cache example). But the surviving elide tier's
license — family-participation, "every retained command upstream of mine, VOUCHED WITH RESPECT TO
MY STATE" — is the same object: a universally-quantified non-interference claim over cells the
vouching author never attended to. The document's own open-fork paragraph admits this shape may
be a hard no *for opaques* ("pinkie-promises, rot, and cargo-culting"); it does not notice the
argument applies equally to *modeled* commands whose payload is arbitrary code.

Concrete counter-example (the flagship): write the truthful completeness-vouch for
`apt-get install nginx` w.r.t. fs.Path and systemd.Service. There isn't one:
- payload file-lists are per-package and unbounded across packages (fs.Path:* touched);
- Debian maintainer-scripts are arbitrary root sh; dh_systemd postinst routinely does
  `systemctl enable --now` (so `: systemd.Service~` — the §2 strawman's own line — is false for
  most daemon packages; 233 §1 already flags the fs.Path ACKs as "*incorrect* … footgun firing in
  practice" and then doesn't carry the consequence forward);
- triggers cascade to unrelated packages (man-db);
- even `apt-get update` is host-config-conditional (APT Post-Invoke hooks; unattended-upgrades
  registers one that can install packages).
Consequence: "where the world is described" is not effort-bounded ("a normal engineering cost");
for the arbitrary-code-executing command class (package managers, curl|sh, make, pip, in-house
deploy tools) truthful vouches are unwritable at ANY effort level, permanently (hard-2 says so).
The describable class is the bounded-footprint class (coreutils, narrow daemon-control,
config-file tools). Since real books LEAD with package operations, the post-wall elide-recovery
story is ~empty for the canonical workload; what actually survives of the attention-product is
(a) converged-elision (elided mutators are not retained, so cast no wall — the steady-state
reconcile narrows fine pre-opaque) and (b) walls at every always-retained opaque, permanent.
+SURE on the epistemics; ~SUSPECT on exact real-book prevalence (no corpus numbers used here).

Mitigation direction 233 doesn't consider: ground non-interference in *probeable system metadata*
where it exists (dpkg -L manifests, trigger tables) — vouches-as-probes rather than
vouches-as-testimony; shrinks but does not eliminate the testimony residue. Expensive, per-PM.

### 236c-fd2 (THE PIVOTAL OVERCLAIM) — "past an opaque, the attention-product isn't buyable at
### all without new trust-machinery" is false as stated; it conflates prediction with observation
The frame problem forbids *predicting across* an unmodeled interposition. It does not forbid
*re-observing after* it. A controller-driven re-probe after the wall-forcing command has executed
converts every post-wall "can't prove, world may have changed" into a fresh, current fact-set —
and the already-planned live-plan engine (22H: "classify once, re-fold per batch is sound"; the
per-host accumulator; streaming re-fold) is most of the machinery. Epoch-shaped apply:
ship epoch-1 artifact (pre-wall elisions licensed as today) → epoch boundary at the retained
wall-former → parallel re-probe (one round-trip; the whole compiled check-suite, cheap per 072)
→ re-fold → epoch-2 artifact with FULL elision licensed by observation, zero vouches, zero new
trust → repeat. Post-wall elision becomes sound *without any of §4's vouch tier*, because the
license is re-observation, not testimony. 233's own upside-bullet contains the seed and misses
it: "cross-oracle vocabulary becomes necessary only to be *fast*, never to be *correct*" — the
same is true of the entire completeness-vouch tier under re-observation.

The guard is then correctly understood as the *degenerate rung* of a re-observation ladder
(host-local, zero-round-trip, per-site, serial, no plan-narrowing), not THE mechanism; the
epoch is the batched rung (round-trip cost, parallel, recovers plan-narrowing + elision). A
cost-model chooses per-region — exactly the query-planner muscle the project already committed
to (kPROBING banding). What genuinely blocks the epoch rung is not the frame problem but two
recorded SOFT leans: the human's static-apply-scripts lean (22H §1 — reconcilable: epochs are
a *sequence of static artifacts*, each still user-editable per 22H §5) and the TOCTOU-WONTFIX
ruling's scope (aimed at environmental drift; 233's guards already re-measure, so the
"never re-check at apply" posture is already conceded in spirit). Presenting "not shorter,
structurally" as forced-by-the-math when it is actually forced-by-the-math PLUS
never-re-observe-mid-apply (a revisable choice) is the document's central framing error.
Costs to state honestly: plan-finality UX (epoch-2+ plans are provisional until their re-probe),
mid-apply failure semantics (epoch boundaries are natural checkpoints — arguably an improvement),
round-trips ~O(retained-wall-batches), cost-model work. +SURE the mechanism is sound;
~SUSPECT it wins the cost-benefit at typical wall-densities (needs sizing).

### 236c-fd3 (ECOSYSTEM EQUILIBRIUM, wish-A vs wish-E unresolved) — the vouch tier re-creates
### §0's failure mode socially, one storey up, or else delivers ~nothing
The fence stops *mechanical* laundering; nothing stops *social* laundering. The design gates the
entire headline product (elide + attention) behind completeness-vouches while hard-2 guarantees
they rot and fd1 shows the flagship class can't write them truthfully. Two equilibria:
(a) authors are honest → almost nobody vouches → elide ≈ dead everywhere → the shipped product
IS the guard tier + hints (see fd4); or (b) the incentive gradient (vouch or your users get no
attention-value) produces cargo-culted blanket vouches — 233 §2's own "lost in the sea of
exclusions" footgun, copied from oracle templates — and wrong vouches "statically delete someone
else's command — cross-site, silent, rot-activated" (233's words). That is §0's blast pattern
with extra steps and third-party blame. The document acknowledges the sharp-knife framing but
never picks an equilibrium or a governance answer (inc-5's machine-enforced-not-author-trusted
model, CI-lint of vouches against container fixtures, provenance-tiering per 236c-fork2 — all
available, none engaged). Absent that, wish-E ("encourage completeness") directly fights wish-A
through the vouch tier, unresolved — the same collision the doc says forced the redesign.

### 236c-fd4 (VALUE ACCOUNTING PAST THE WALL) — what's left is hand-written-guard parity,
### serial, re-paid every apply; the founding complaint reinstated as the product's own output
DESIGN's founding complaint is that serially executing check-then-execute trees is "quite silly";
the guard-tail is exactly that tree, machine-generated, run per-apply per-host, forever, in the
steady state (the wall-formers are always-retained opaques, so there is no "shadow of whatever
real mutation forced the wall" to hide in — that phrase is true only for the genuinely-mutating
wall case and oversells the common one; the doc's own check-tax bullet is the honest version).
Vs bare execution and vs Ansible (per-task round-trips) it still wins; vs the founding promise it
is the pre-Dorc diligent-author baseline, automated. The honest competitive statement for the
undescribed region: "we compile the idempotency discipline a diligent author would have written,
plus attribution, plus claims-not-proof hints" — real, modest, and deliverable by a far smaller
tool than the analyzer (see fd5). Also under-stated: guard-ineligible classes (consumed-stdout,
rc-consumed, run-delta verbs, loops, multi-operand) are COMMON in real books, so the "~97-guard
artifact" picture is optimistic; ineligible sites run bare. And the post-wall probe-suite spend
buys hints only — under kPROBING's own banding, hint-only probes on expensive checks should be
dropped, which then thins the hints too (the "expected: 96 no-op" UX assumes free probing).

### 236c-fd5 (PRODUCT-IDENTITY DILUTION; the user's instinct, made precise) — the machinery the
### project exists to build stops being what delivers the value, in the region that dominates
The elision-licensing core (ambient gate, reaching-defs, per-entity re-key, family reasoning —
the keystone of three spikes) is load-bearing only where elide is reachable: the pre-wall prefix
+ vouch-chains (fd1: narrow) or under re-observation (fd2: not proposed). The guard tier needs:
constant-prop through the oracle argparse (path-reach for the vouch), the check-lift, and a
renderer — no reaching-defs, no fact-plane, no cross-site reasoning. If the undescribed region
dominates real books (STALENESS-AUDIT L100-104 says the multiplicative gating was already the
binding constraint), Dorc-as-shipped converges toward "a guard-compiler with a parallel
hint-probe" — defensible software, but not the analyzer-product the corpus spent rounds 1-22
building toward, and the document never re-prices the machinery against the shrunken licensing
role. Not an inconsistency; an unpriced consequence the human should weigh explicitly.

### 236c-fd6 (PROCESS/RULING HYGIENE) — two human-held-open rulings quietly resolved, one weld
### reversed by engaging its letter but not its reason
- dq-errexit-3's REASON — the trust-boundary taxonomy, "a bad oracle must never cause novel
  apply-phase actions" — is definitionally violated by guard-as-default (oracle code executes in
  the apply lane, in the book's shell environment, at scale). 233 cites the flag-gating/build-last
  ruling and proposes conscious re-welding, but never answers the taxonomy argument itself. The
  supporting security lesson it also skips: Chef why-run's "read-only ≠ side-effect-free" (round
  10) now applies at apply-time with NO containment story — the probe lane's sandbox/seccomp
  ambitions (an-withhold-sandbox, 077) do not transfer to guards interleaved with legitimate
  network-using mutations. Guard-body hygiene (218a's set -u/collision/stdin list) is named but
  the *blast-category change* (probe-phase mutation is auditable pre-plan; guard mutation is
  interleaved with real mutations, per-apply, forever) is not.
- dq-errexit-2 (bare-middle owner) was "genuinely open," human "unconvinced" by oracle-default;
  233's converged-vouch quietly IS oracle-default consent (engineer judges skippability for the
  admin's book; admin per-site override "parked"). Under DESIGN's two-users doctrine the admin's
  recourse must be designed WITH the tier, not parked: an oracle-library upgrade silently changes
  book behavior; disclosure ≠ recourse. (If the same-day conversation was the human ruling it,
  fine — but the doc should say so against the recorded open status.)
- kSILO: guard-auto-insertion is the strongest silo-shove yet (Dorc writes your guards, so books
  stop accruing them; correctness migrates to oracle libraries; KNOBS names this the watch-item
  whose constituent decisions are baked by the time habits show). Unmentioned in 233. Partial
  offset: the ARTIFACT off-ramp genuinely improves (a rendered book is a diligent check-then-act
  script) — the book-source off-ramp degrades while the artifact off-ramp improves; worth naming.
- Scale-shift of an accepted risk: 218a hunt-B's "disaster class" (consumer-tag mis-tagging lets
  a declaration suppress a written `|| fallback`) was priced against a flag-gated, build-last,
  rare population; guard-as-default-middle-verdict makes the same precision problem the bulk
  path for every site in every book. The precision bar moves ~two orders of magnitude with no
  discussion.

### 236c-fd7 (SMALLER, honest-nits kept out of the report) — for completeness of these notes
- "Converged vs no-op" conflation is load-bearing inside every vouch (the upgrade-suppression
  judgment) and deferred; TODO.md item 8 is the same item; fine as deferral, but the vouch
  SPELLING will bake one reading — settle before spelling, or the deferral is illusory.
- Artifact-bloat/mindshare: the plan the user "approves/edits" (DESIGN; 22H §5) is now
  book + preamble + ~N guards; the doc's own render-obligation ("uniform, visually-inert,
  foldable… or the mindshare cost is total") is a hard UX bet stated but not designed. Region-
  guards (one compound predicate gating a converged region, over-execute-on-any-failure inside)
  are an unexplored middle rung that shrinks both tail-cost and visual noise.
- Guard recognition on re-analysis ("nothing accretes") is solved for unmodified artifacts
  (already-guarded refusal, 218a d4-6) but silently degrades if the user edits near a fat-pole
  guard blob; double-guarding is cosmetic-but-real. Engineering, not design.
- The probe phase's value past the wall (hints) degrades exactly when it matters (post-opaque
  states are the ones the probe can't have seen) — hints are honest as claims, but the UX copy
  ("expected no-op") will be least reliable at the moment of genuine change. Wording problem.

## §5 Counter-theses / forks to put in front of the human

### 236c-fork1 — re-observation ladder (the fd2 alternative, the big one)
{guard = host-local per-site rung} ⊂ {region-guard} ⊂ {epoch re-probe + re-fold (22H machinery)}.
License post-wall elision by observation, not testimony; vouches demote to perf hints (probe-
pruning, plan-time prediction) and OFF the soundness-critical path entirely — which also
dissolves fd3 (no incentive to lie; a wrong hint costs accuracy of provisional plans, never
someone else's command). Requires human adjudication of the static-artifact lean + the
TOCTOU-WONTFIX scope; costs plan-finality + round-trips; keeps kFAIL intact (probes stay
read-only; epochs run post-approval, in book order).

### 236c-fork2 — provenance-tiered elide-licensing (if the vouch tier is kept)
Only accountable, centrally-maintained models (the ~40-50 bootstrap/blessed oracles of
effort-allocation; Dorc-owned coreutils semantics) may carry family-participation weight;
community oracles cap at guard until promoted through a governance gate (inc-5's
machine-enforced-not-author-trusted precedent; container-fixture CI against declared footprints;
the 077-arc runtime tracing as a post-hoc vouch auditor). Concedes part of wish-C for the elide
tier only — which fd1 argues is already conceded in truth, just not in the document.

### 236c-fork3 — depender-declared invalidation (story-1's missing inversion)
17N §9 opt-3 as a first-class oracle spelling: the fact-owner declares what kills its fact
("any package-mutation kills cve_clean"). Composes with either fork; removes the
know-the-future burden from mutator-oracles; leaves opaques to fork1's re-observation.

### 236c-fork4 — say the quiet part in the product statement
Post-wall attention-narrowing ALREADY has a shipped-by-design vehicle: kELISION-scoped /
`dorc bump` (user-directed narrowing, user assumes the frame risk — DESIGN's "magic" principle).
The honest product statement under ANY of these designs is three-legged: proof-narrowed where
described/converged; user-narrowed under bump; guarded elsewhere. 233's statement omits the
second leg entirely, making the concession look total when the daily-driver loop (bump after a
small edit) was never wall-bound in the first place.

## §6 Verdict (for the final report)

- The problem-half of 233 is correct, source-verified, and genuinely fundamental. Any review
  claiming "just fix the poison default" has not understood it; both horns were already lived.
- The guard mechanism is sound, prior-art-grounded (218a), per-site safer than static elision,
  and the correct bottom rung of the design. The monotonicity/safety upsides are real.
- The proposal-half overclaims twice (fd1: the surviving "described-world" elide tier rests on
  testimony its own argument refutes for the flagship class; fd2: "not shorter, structurally" is
  forced by an unexamined never-re-observe premise, not by the frame problem), leaves the vouch
  tier's ecosystem equilibrium unresolved (fd3), re-prices neither the surviving value (fd4) nor
  the machinery (fd5), and reverses/settles human-held rulings without engaging their reasons
  (fd6). The goals-change itself (attention → perf+safety where undescribed) is honest in
  direction but conceded too broadly and framed as forced when it is chosen.
- Recommendation shape: keep the ternary verdict + guard rung + fence (build-worthy); do NOT
  build the community completeness-vouch tier as the elide-license (fd1/fd3) — route elide
  recovery through fork1 (re-observation) and/or fork2 (provenance-tiered licensing); surface
  fork4 in the product statement; take the fd6 items back through the humans' rulings explicitly.
