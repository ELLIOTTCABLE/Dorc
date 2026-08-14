# Crosscheck report — neutral pass

Reviewer: Fable-class, clean context, read-only. Subject: the verification-tooling deep-research
effort (turns 01–06 + `sources.json` + the pending-amendments ledger + the Lean sparing spike),
assessed for (a) internal validity, (b) plan coherence against standing law, (c) formalization
faithfulness, (d) kernel fidelity. Ground rules honored: source base taken as given (grading
critiqued only); CHECKED/green statuses accepted as facts about machine-checking; no re-research;
no execution. Materials read in full: all six turn notes, `sources.json` (22 entries), the
amendments ledger, the spike's `README`/`REPORT` and all five `.lean` files, `notes/277`
(§§1–6), `notes/272` §4, `spike/CLAUDE.md` (separation/worlds/survival cluster + engineering
substrate), `spike/crates/core/CLAUDE.md`, `spike/crates/analysis/CLAUDE.md`, `lattice.rs`,
`solve.rs`, `claim.rs`; targeted reads of `plans/021` §1, `plans/055` item 6, `plans/271`
(net-quality-u-curve, sin-ordering), `plans/262` §1, `notes/26C`, `notes/27Xf`, `notes/28R`,
`KNOBS.md` (kVERIFY), `ANALYZER-NEEDS.md` (an-monotonicity/an-backing-set-meet/an-lcg-quality/
section O), `AID-NEEDS.md` (law-collapse-mints-narrative), and git history for the kernel files.

## Findings, severity-ranked

### 1. finding-flip-three-tense-contradicts-kernel-determinism (~SUSPECT severity; +SURE on the mechanics)

Turn06's FLIP-3 / portfolio item 2 asserts, present-tense, that the 22W Eq-exclusion makes
"explanation var[y] run-to-run while the license plane converges identically." The kernel at HEAD
excludes this by construction: `solve` is a deterministic FIFO worklist over ordered collections
(`solve.rs` header + `solve_is_deterministic` test; `spike/CLAUDE.md` inv-determinism), so
visit-counts, mint multiplicity, and k-cap cuts are identical run-to-run on identical input. The
real exposure is version-to-version churn and, decisively, the parked r26 read-concurrency
revival, where arrival order genuinely varies (`plans/262` §1 binds exactly this, and its scope
sentence explicitly covers *records*) — so the proposed multiset-permutation pin is well-founded
and aligned with standing law, but its billing as "likely the most important NEW finding" rests
partly on a variation mode the current engine cannot exhibit. The finding should have been
conditioned ("under scheduling variation the engine does not yet have") rather than stated as a
live instability. Evidence: turn06 §"Narrative-30 re-analysis" + portfolio item 2;
`spike/crates/analysis/src/solve.rs` (whole file); `plans/262` §1.

### 2. finding-context-gate-shadows-kind-fence (+SURE the textual divergence; ~SUSPECT on intent)

The Lean `compare` (Compare.lean, the chokepoint) tests context inequality FIRST and returns
`unknown` for every cross-context pair, so the kind-fence never fires cross-context. `272` §4
prices the never-derive-separation carve as costing "only the within-kind cross-context
disjointness dividend (**cross-kind disjointness stays free by construction**)", and `277` §3
says "Cross-entity/kind disjointness unchanged" — i.e. the spec-as-written grants cross-kind
disjointness independent of context, and the model silently forfeits it. The divergence is
conservative (loses sparing, never wrong-spares) and dead at v1 (every coordinate carries the
host-default context, so the branch cannot be reached), but it is exactly the kind of
underdetermination the SPEC-GAPS section exists to record, and instead the docstring asserts the
ordering as if spec-derived. It also interacts with gap-top-member-granularity, whose text
("the kind-fence short-circuits cross-kind pairs to provably-disjoint") is true of the model only
same-context. Should be added to the SPEC-GAPS ledger and put in front of the spec-owner alongside
`28R:adj-sparing-two-position-rule`, which governs the same seam. Evidence:
`spike-lean-sparing/SparingAlgebra/Compare.lean` (`compare`, layers 1–2 + docstring);
`Research/notes/272-address-derived-topology.md` §4; `Research/notes/277-entity-algebra-design.md` §3.

### 3. finding-dec-six-exception-site-mismatch (+SURE the mismatch; ~SUSPECT materiality)

Turn06 item 1 claims the certifier "executes plans/021 §1 + plans/055 dec-6's own pre-authorized
narrow exception." Half holds: `plans/021` §1's deferred exception names two invariants, and its
second — elision's "skip ⇒ proven-converged" — plausibly covers a post-fixpoint certifier at the
solve seam. But `plans/055` item 6 authorizes a targeted property/Why3 check at ONE named site:
"the inert-classification kernel" — not the solver. Citing both as pre-authorization overstates
the pedigree; the certifier is a new (if well-argued) siting decision that deserves its own ack,
not an inherited one. Turn04 read 055 correctly ("Decision-6 re-names the inert-classification
kernel as the one justified targeted property/Why3 site"); the compression into turn06 is where
the over-claim entered. Evidence: turn06 portfolio item 1; turn04 findings;
`Research/plans/021-static-analysis-engine.md` §1 (exception paragraph);
`Research/plans/055-analysis-architecture.md` item 6.

### 4. finding-kverify-weld-quietly-outgrown (~SUSPECT)

Turn06 asserts "kVERIFY weld NOT reopened," which is true of portfolio items 1–2 (a runtime
check and Kani harnesses are calibration-tier). But the COMBINED plan-state — amendment-survival-
reprioritized step 4 (promote the reference model to Lean/Dafny and *prove the laws*) plus
amendment-aeneas-pulled-forward artifact D (Lean proofs over the Aeneas translation of the carved
core, CI-mechanically re-translated) — is the `kVERIFY-prove` pole for the small core, well beyond
021 §1's "a v2 unit-confidence tool for one function, not a project methodology." The moves are
human-typed leans, so nothing was done without authority; the gap is bookkeeping honesty: no
document flags that the weld's registry text ("TypeScript, not Coq"; owner: welded) now
understates the standing plan, while the ledger does contemplate the analogous status-line edit
for kSURVIVAL. The knob registry is the project's named instrument for exactly this, and the
plan-state should either carry the kVERIFY status note or explicitly argue scoped-core proofs sit
inside the weld. Evidence: `KNOBS.md` kVERIFY entry; amendments ledger both amendment sections;
turn06 portfolio item 1.

### 5. finding-seventeen-day-figure-unsupported (+SURE)

Turn06 item 2 (and the scout-B summary it banks) states the `277` §5 vacuous-∅ bug class
"shipped acked and survived 17 days." The corpus's own dates give 1–4 days: the set-lifting law
was minted 2026-07-13 (`279f:fix-set-lifting`, per `277` §3/§5 amendment notes), acked 2026-07-16,
and the hole was credited 2026-07-17 (`27Xf:cr-set-lifting-vacuous-at-empty`; the 27Xf note is
dated 2026-07-17). The "17" appears to be the catch DATE (07-17) bleeding into a duration — a
scout-derived factoid repeated unverified in the synthesis-of-record, which is precisely the
claim-class turn01's method note warns about. Decorative to the portfolio ranking (the bug class
is real and the pins remain justified), but it inflates the "shipped and survived" rhetoric and
should be corrected in any digestion into `plans/`. Evidence: turn06 item 2 + scout-B summary;
`Research/notes/277-entity-algebra-design.md` §3/§5 amendment dates;
`Research/notes/27Xf-crosscheck-adjudication.md` header + cr-set-lifting-vacuous-at-empty.

### 6. finding-reference-model-differential-not-foreign (~SUSPECT)

The amendments' sparing-algebra plan (steps 2–3: naive Rust reference model,
internal-differential + plan-time re-derivation) does not restate turn06's own cross-cutting
anti-gaming plank against itself: a reference model authored by agents from the same `277` spec
shares spec-misreadings with the production implementation, so the differential catches
divergence-class bugs only — the same correlated-blind-spot family as `22W:fb-same-model`
("same-model convergence is NOT independent confirmation"). The conductor applied this honesty to
the certifier ("catches solver bugs, not model bugs") but not to the reference model. Adjacent and
uncited: `271:struck-falsifiability-license-leg` struck a prior argument precisely for leaning on
a mechanical differential — the dip of `271:rul-net-quality-u-curve` — which cuts against the
ledger's "~SUSPECT it passes" lean. The strongest pro-passing argument (the re-derivation is
demote-only: disagreement demotes to guard/run, agreement licenses nothing new, so it cannot
manufacture confidence the way the struck leg did) is available but never made. The owed u-curve
check should engage that precedent explicitly. Evidence: amendments ledger
amendment-survival-reprioritized steps 2–3 + the NOTE paragraph; turn06 cross-cutting block;
`Research/plans/22W-round22-close-report.md` fb-same-model;
`Research/plans/271-block-settle-rulings-ledger.md` rul-net-quality-u-curve +
struck-falsifiability-license-leg.

### 7. finding-certifier-boundary-half-under-specified (~SUSPECT)

The certifier outline says "per edge v→w check transfer(v, states[v]) ⊑ states[w], plus boundary
conditions" — correct against the actual solver's semantics (states are input-states; propagation
is join-into-flows_to). But two boundary subtleties go unremarked where they are the load-bearing
half: (i) for Must-oriented runs, `Must<L>`'s ⊥-start seeds interior nodes at L's ⊤, and
`lattice.rs`'s own boundary note records that entry seeding for forward-must analyses is
deliberately unbuilt (note 167 DP-8) — entry nodes have no incoming edges, so the per-edge check
is vacuous exactly where must-orientation soundness lives; (ii) the literal Blazy fallback
(`else top`) is unspellable for the kernel's unbounded may-domains (`Powerset`/`MapL` are
deliberately not `BoundedLattice`), so refusal must be a value the consumer degrades on — which
the amended `Certified | Refused(EdgeWitness)` shape happens to provide, but no turn notices the
type-level mismatch it dodges. Neither invalidates the architecture; both belong in the certifier
spike brief before it is cut. Evidence: turn06 portfolio item 1; turn03 first finding;
`spike/crates/analysis/src/lattice.rs` (Must boundary note; Powerset/BoundedLattice docs);
`spike/crates/analysis/src/solve.rs`.

### 8. finding-transport-set-lifting-unmodeled-unflagged (+SURE absent; -GUESS materiality)

`277` §5's set-lifting law has two halves: sparing (universal disjointness) and "transport over a
backing-set requires every member to transport." The spike models and proves the sparing half
(`sparesSet` + laws) and models transport only at pair level (`transportLicensed`); the
set-lifted transport fold is absent and is not named in the REPORT's out-of-scope list (which
covers cross-context transport and the 28R amendments, but not this). The vacuity hazard the
spike explores applies to the transport half too (guarded by the same nonempty-by-construction
encoding, so no new hazard — but the symmetry deserved a line). Evidence:
`spike-lean-sparing/SparingAlgebra/Sparing.lean` (transportLicensed + docstrings);
`REPORT.md` "Known-unmodeled amendments"; `notes/277` §5 set-lifting law.

### 9. finding-gap-severity-order-vs-sin-ordering (-GUESS)

The SPEC-GAPS ordering puts the two safe-direction ambiguities (compare-directionality,
top-selector self-sameness) above the one unsafe-direction hazard (footprint-side ∅ — a vacuous
spare-everything if the unstated invariant fails, i.e. under-execution, the cardinal sin per
`271:rul-sin-ordering` and the execution-priority-order). Defensible if the axis is
"spec-underdetermination severity" (gap-1 is the most structurally load-bearing, and gap-4 is
probably engine-true today, as the spike says); but the project's own severity vocabulary ranks
by danger, and by that scale gap-footprint-empty-set-unpinned arguably belongs at #2. A
one-line statement of the ordering axis would dissolve the ambiguity. Evidence: `REPORT.md` §(b)
ordering + gap-footprint-empty-set-unpinned; `spike/CLAUDE.md` execution-priority-order.

### 10. finding-grading-rubric-preprint-inconsistency (~SUSPECT, low)

Two arXiv preprints receive different letter grades on partially overlapping rationale:
[B-prover-is-the-judge-spark-agents-2026] is "not A: arXiv preprint, single author," while
[A-vericoding-benchmark-2025] is A despite "arXiv preprint (not yet peer-reviewed)." The
differentiators offered (n=1 headline figure vs. largest-benchmark-of-kind) are real, but the
peer-review criterion is applied as a B-reason in one entry and waved through in the other.
Downstream weighting is not obviously wrong — the round consumed vericoding's own measurements
with the ordering-robust/absolute-floor caveat — but the rubric drift is worth knowing when the
A/B letters are read at face value later. Evidence: `sources.json`, grading-reasoning fields of
the two entries.

### 11. finding-subagent-regrade-obligation-undischarged (~SUSPECT, low)

Turn01 flags all seven of its sources "graded-by: subagent — due re-verification." Turn02
re-verified the overlapping ones incidentally (the two summariser errors, the Flux figures), but
the round closed with no explicit discharge for the rest — notably
[B-prover-is-the-judge-spark-agents-2026], which carries the portfolio's cross-cutting anti-gaming
plank, and [A-spark-user-guide-applying-in-practice-2026], which anchors the level-discipline
overlay. Mitigation: the citations are verbatim-block quotes with sha256-pinned archives, so the
residual risk is quote-selection bias, not figure corruption. Evidence: turn01 "State of this
research dir" + Manifest note; turn02 header; `sources.json` graded-by fields.

### 12. finding-aid-needs-cited-while-disclosed-unread (+SURE the tension, low)

Turn06 item 1 loads `AID-NEEDS:law-collapse-mints-narrative` as a design constraint on the
certifier while turn06's own gap-disclosure says AID-NEEDS.md was never read. The law's content
was available secondhand (`spike/CLAUDE.md` collapse-mints-narrative) and I verified the AID-NEEDS
text matches the use, so the conclusion survives — but the round's own citation discipline
(read-before-rely; reading gates task 7) is bent in the one place the unread doc is cited
normatively. Evidence: turn06 item 1 + "GAP DISCLOSED" line; `AID-NEEDS.md`
law-collapse-mints-narrative; `spike/CLAUDE.md` same-named bullet.

### Nits (no severity)

- nit-five-samples-per-combinator — "FIVE hand-picked samples per combinator" (turn03/turn06):
  actual sample sets are 3–5 per lattice (`lattice.rs` tests). Immaterial to the Kani argument.
- nit-untouched-since-round-19 — the amendments' "lattice.rs/solve.rs untouched since round 19"
  is substantively true (last real change 2026-06-10, the reseed) but a 2026-07-26 encoding/BOM
  repair (`77d55a2e`) did touch both files.
- nit-scout-assert-census — scout A's "7 runtime asserts (all debug_assert)" undercounts;
  a workspace grep finds 16 occurrences across 10 files (9 in non-kernel non-test code). The two
  named release-silent asserts (aid arrangement words==values+1; weft measure/paint left edge)
  do exist exactly as described, so portfolio item 4's substance stands.

## What HOLDS

The load-bearing judgements I examined and found sound — this list is as much the deliverable as
the faults above.

- **holds-kernel-figures-exact** (+SURE) — the round's kernel claims check against the code
  precisely: 480+301 lines of algebra; the cap formula `n*1024+4096`; `converged==false` with no
  operand (a "globally-smeared symptom" is a fair description of `Solution`); the caller-upheld,
  un-type-enforceable preconditions; the 435/783 CPU-s empirical hang (`ANALYZER-NEEDS`
  an-monotonicity, verified verbatim); MapL's private-field canonicality with structural-Eq-as-
  semantic-equality that convergence detection relies on; `Must<L>` as order-dual with the
  BoundedLattice gate making union-for-intersection a compile error.
- **holds-certifier-architecture-reasoning** (+SURE on the logic) — the central move (a
  post-fixpoint is checkable per-edge with no quantifiers; monotonicity is a termination/precision
  concern, not a soundness obligation, under result certification; how the solution was found is
  irrelevant to its safety) is correct against the actual solver's semantics, and the
  degrade-toward-run mapping is uniformly safe in both orientations because the Must dual makes
  lattice-⊤ conservative for either. The Must-duality one-checker claim holds (`leq` is derived
  from `join`, so the dual order comes for free). The turn02→turn03 composition — certifier
  defuses the turn01 monotonicity-must-be-central worry — is valid inference, properly hedged.
- **holds-algebra-engine-split** (+SURE) — the decomposition (small spec-frozen algebra vs churny
  engine) is real in the tree, and the round consistently routed tooling by that split rather
  than by tool enthusiasm; the NOT-tooling item (protocol-atomicity classes stay DST/framing
  territory) is confirmed by ANALYZER-NEEDS section O — both 28P measured wrong-elisions are
  transport/closure-shaped, exactly as claimed.
- **holds-claim-rs-assessment** (+SURE) — claim.rs is as described: sealed tiers, the four
  TC-tier properties as absence-of-code, compile_fail doctests as tripwires, one-way demotion.
  The judgment that neither Flux nor Kani strengthens whole-tree absence properties is correct,
  and "tooling value concentrates elsewhere" follows.
- **holds-lean-spike-core-faithfulness** (+SURE on what I checked) — the fold ⇔ ∀-form
  equivalence, nonempty-by-construction, wall-collides, order-independence (member and footprint
  sides), flag-gating wholesale (correctly resolved via `271:rul-flag-is-razor-residue`), the
  amended both-sides top_never_spares, the qualified-only cross-family monotonicity (correctly
  NOT restating the falsified absolute per `279f`), and the pre-amendment reconstruction (claim-
  side-only ⊤ special-case with the 279a-A5 inhabitant) are all faithful to `277`-as-written.
  The safety-inversion witness is a genuine inhabitant, and the conditional-soundness framing
  (misfires trace to generators, never the composition algebra) is the right reading of §2's
  bite-class list.
- **holds-spec-gaps-mostly-genuine** (+SURE for gaps 1, 3, 5; ~SUSPECT-agree for 2, 4, 6) —
  gap-compare-symmetry-vs-directionality is real (only backings carry a minting family; the spec
  never acknowledges position-typing) and its claimed convergence with
  `28R:adj-sparing-two-position-rule` is verified against 28R's text;
  gap-top-member-granularity correctly catches that the unqualified "collides with every
  footprint" cannot be delivered by an entity-scoped ⊤ under the kind-fence;
  gap-entity-disjoint-generator-missing is verified — `277` §2's registry genuinely has no row
  producing cross-entity disjointness while §3 assumes the entity-granular baseline. The spike's
  statement-uncertainty flags (the #4 API-shape honesty: greppability, not unrepresentability)
  are exactly the right epistemics.
- **holds-method-hygiene** (+SURE) — the summariser-defect catch, the quarantine of first-pass
  material, the two verified-wrong figures (Cedar 3.4:1 arithmetic checks out from the quoted
  Table 1), the read-before-act discipline on Blazy (flagged unread in turn01, read before the
  recommendation firmed), and the model-union vs best-single-model decomposition of the
  vericoding numbers are all genuine quality moves that survived scrutiny.
- **holds-honest-hedging-mostly** — the SymCrypt churn claim held at SUSPECT/promotional; the
  n=1 SPARK cost figure fenced; the null-result warning carried into the portfolio's honest-case
  framing; the ack-ledger's typed/floated separation maintained (evict-maps explicitly NOT
  human-acked; flux-lean marked lean-not-decision); the amendments ledger disclosing the killed
  adversary, the un-run adjudication, and the parked Aeneas lane rather than papering over them.
  The exceptions are the specific flat-stated items in findings 1, 3, and 5.
- **holds-knob-survival-reweighting** (+SURE) — turn03's mid-course downgrade of the
  flat-domains-fit-Flux argument from load-bearing to incidental, on the human's
  knobs-not-load-bearing directive, is exactly right, and the corresponding strengthening of the
  domain-generic certifier argument is valid.
- **holds-portfolio-item-six-restraint** — declining to put the survival-lane protocol classes on
  the refinement backlog (tool-shaped-hammer guard) is correct and verified against the actual
  failure records.

## Do the findings invalidate any human-typed decision?

No. The typed rulings in turn06's ack-ledger (error-tier ladder, knobs-not-load-bearing,
code-is-a-model, flux-lean-add-early as a lean, narrative-thirty, aid-seam-exists,
two-plane-firewall) and the amendments' typed rulings (survival-is-the-core-product,
more-danger-more-attention, elision-never-unshipped, aeneas-pulled-forward as a lean) all rest on
premises my findings leave standing: finding 1 corrects the *tense* of a supporting mechanism
(narrative multiset instability is churn/r26-conditional, not run-to-run at HEAD) but
narrative-thirty itself was typed on independent grounds ("I can't explain why" as a wrong
answer) and the pin it motivates is independently mandated by `262` §1's order-free invariant;
findings 3 and 4 correct conductor-asserted *pedigree and bookkeeping* around decisions the human
typed directly, not the decisions; finding 5 corrects a decorative duration on a bug class whose
existence and repair are independently recorded in `27Xf`; and findings 2 and 6–9 bear on the
throwaway-grade model and on plan details explicitly staged for later human checks (the u-curve
check is owed, not skipped). The two 279f typed acks the spike touches (spare-top-backing,
set-lifting + its fixpoint clause) are *confirmed* by the machine-checked reconstruction, not
undermined. The one action I would put in front of the human as a consequence of this review:
the kVERIFY status-line note (finding 4) and the certifier's own ack (finding 3) should be
obtained explicitly rather than inherited — both are cheap, and both close real gaps between
what was typed and what the record claims was typed.
