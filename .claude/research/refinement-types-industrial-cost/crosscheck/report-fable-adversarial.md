# Adversarial crosscheck — the verification-tooling research effort (turns 01–06 + amendments ledger + Lean sparing spike)

Reviewer: read-only adversarial pass, 2026-08-13. Scope per brief: the six-turn round,
`sources.json`, the amendments ledger (superseding turn06 where they disagree), the Lean
spike in the linked worktree, judged against `notes/277`, `notes/272` §4, the CLAUDE.md
law registries, and the kernel (`lattice.rs`, `solve.rs`, `claim.rs`). One permitted URL
spot-check was spent on the recorded `B-flux-verify-rust-std-tool-assessment-2026` URL.
No builds run; no literature re-gathered; proofs taken as machine-checked — statements
and encodings were the target.

Summary posture up front: the round's method discipline (raw-read-only after the turn01
summariser defect, quarantine block, per-quote citations, slug validation) is genuinely
good and mostly held. Where it breaks, it breaks in a consistent direction: the
synthesis-of-record (turn06) and the amendments ledger repeatedly re-state hedged or
quarantined material one notch more confidently than the underlying record licenses.
Two suspicions of the commissioning brief are confirmed with concrete instances; two
mostly fail; one splits.

---

## Findings, severity-ranked

### 1. fnd-quarantined-claims-stamped-verified — +SURE

**Claim.** Turn06 stamps "(all verified)" on Flux cost claims the round's own quarantine
marked unverified-and-decision-relevant, and asserts the Kani sustainment precedent as
unhedged fact when its only occurrence in the corpus is the turn01 do-not-cite block.

**Evidence.**
- `turn06-2026-08-13-notes.md` portfolio item 3: "Honest costs (all verified):
  nightly-pin + build-from-source + z3/liquid-fixpoint binaries". The build-from-source
  and z3/liquid-fixpoint claims exist in the corpus only in `turn01` §"Unverified
  first-pass material" (Q2), flagged there "VERIFY THIS — it is decision-relevant",
  with the Flux install guide left on the never-read To-read list. No registered source
  covers them. The spot-check confirms the recorded
  `B-flux-verify-rust-std-tool-assessment-2026` page contains no install/toolchain-pin
  content — its Installation section is one sentence deferring to the (unread) Flux
  book. The nightly-pin claim is verified only at mechanism level (turn02: Flux is a
  rustc/MIR plugin, "confirms the nightly-coupling mechanism") — an inference, not the
  quarantined specific.
- `turn06` portfolio item 2: "multi-year ordinary-team sustainment precedent
  (Firecracker/s2n-quic CI)". Grep of the round directory: the only prior occurrence of
  Firecracker/s2n-quic is `turn01` quarantine Q6 ("Kani reportedly runs in production CI
  at…" — summariser-derived, do-not-cite). Turn06 carries it citation-free and
  hedge-free into the ranked portfolio.

**Weight.** Both claims are, as far as general knowledge goes, probably true in the
world — so the substantive damage is bounded. The process damage is not: the round's
own opening finding was that summariser-derived claims ran a measured non-trivial error
rate (two load-bearing figures verified wrong), which is exactly why the quarantine
exists. The synthesis-of-record spends the word "verified" — the grading ritual's
currency — on material the ritual never touched. This is the commissioning brief's
suspicion-1 (grading launders secondary claims into load-bearing ones), confirmed with
the word "verified" itself as the laundering instrument.

### 2. fnd-certifier-authorization-provenance-stretched — +SURE on the textual mismatch; ~SUSPECT on consequence

**Claim.** Turn06 item 1 says the solve-seam certifier "executes plans/021 §1 +
plans/055 dec-6's own pre-authorized narrow exception — kVERIFY weld NOT reopened."
The cited exception does not pre-authorize this mechanism at this site.

**Evidence.** `Research/plans/021-static-analysis-engine.md` §1: the exception is
GUESS-graded, "a v2 unit-confidence tool for one function, not a project methodology.
Do not adopt up front", and names two targets: the inert-classification kernel
("⊥ ⇒ provably no mutation") and elision's "skip ⇒ proven-converged" — as "a small
property-test suite or even a Why3 lemma". `Research/plans/055-analysis-architecture.md`
(item 6, ~line 99): "The one place a targeted property/Why3 check could be justified
(v2): the inert-classification kernel." The solve-seam post-fixpoint certifier is
neither named target, and is production runtime machinery with narrative-minting
duties, not a property suite or a lemma. Turn04 discloses the honest chain (both named
kernels were later discharged architecturally; the certifier targets the
un-structuralizable residue) — but that is an argument that the exception's *spirit*
transfers, made by the same document that then claims *pre-authorization*.
"Pre-authorized" presents a new authorization question as already answered. Note the
portfolio's item 2 (Kani/property law-harnesses) is a much closer fit to what 021 §1
actually authorized; the certifier borrows item 2's provenance.

**Guard note.** The kVERIFY weld itself (KNOBS.md ~L279: end-to-end proof unattainable)
is arguably not reopened by a targeted certifier — that half of the sentence survives.
And the round leaves execution "awaiting human go", so nothing was built on the
stretched authorization. The fix is one honest sentence: "consistent with the spirit of
a deferred, GUESS-graded exception that named different kernels; wants a fresh ack."

### 3. fnd-lean-model-divergences-beyond-declared-gaps — +SURE the divergences exist; ~SUSPECT on intended spec readings

**Claim.** The spike's REPORT declares seven spec-gaps and presents them as the
residual statement-risk. At least two further semantic divergences between the model
and the spec corpus are unflagged, and one of them violates the spike's own stated
modeling standard.

**Evidence.**
- **Context checked before the kind-fence.** `SparingAlgebra/Compare.lean`, `compare`:
  layer 1 is `claim.ctx ≠ b.coord.ctx ⇒ unknown`, so a cross-kind pair in different
  contexts yields `unknown`. `Research/notes/272-address-derived-topology.md` §4 states
  the never-derive-separation carve costs "only the within-kind cross-context
  disjointness dividend (**cross-kind disjointness stays free by construction**…)" —
  i.e. cross-kind pairs should be provably-disjoint across contexts too. `notes/277`
  §1's kind-fence carries no same-context qualifier. The model is strictly more
  conservative than the spec (under-spares — a product-value loss under the amendments
  ledger's survival-is-the-core-product ruling), and the choice is nowhere flagged; the
  REPORT's gap list and the file's doc-comments are silent on it. 272 §4 is explicitly
  in the spike's declared scope (README: "plus `notes/272` §4").
- **Equal-but-unminted tokens compare `same`.** `Compare.lean` `selectorTier`:
  `.tok c, .tok b => if c = b then .same …` — no dialect-row check on the `same`
  branch. `277` §3 says unminted tokens "are all ⊤-selector", and `277` §6 says "⊤
  identifies with nothing (including itself)"; under that reading two byte-equal
  *unminted* tokens must not compare `same` (same feeds transport). The engine
  invariant (backing selectors come from minting lines) makes the case unreachable in
  practice — but the spike's own gap-both-sides-minted-redundancy declares the standard
  "checked anyway, so the formal algebra does not silently depend on that engine
  invariant" for the *disjoint* branch, then silently depends on exactly that invariant
  on the *same* branch, in the transport-licensing (dangerous) direction. The exactly
  analogous selector-less ambiguity got a flagged gap
  (gap-top-selector-self-sameness); this one did not.

**Weight.** Both are small; one is safe-direction. What they break is the meta-claim —
the REPORT's implicit "the seven gaps are the statement-risk residue" — and they land
precisely where the commissioning brief's suspicion-4 predicted divergences would hide:
in encoding choices the green build cannot speak to. Statement review by spec-owners
(which the REPORT itself recommends) should treat the gap list as a floor, not a
census.

### 4. fnd-lean-tier-plan-state-unreconciled — +SURE

**Claim.** The combined plan-state (turn06 as amended) holds three coexisting,
unreconciled shapes for the same deliverable — Lean-tier assurance of the sparing
algebra — and the ledger never says which governs.

**Evidence.** (a) `amendment-survival-reprioritized` step 4: promote the Rust reference
model to Lean/Dafny and prove the laws **at algebra-spec settle** (the new trigger;
"Lean waits for settle" is the stated churn-tax defense). (b)
`amendment-aeneas-pulled-forward`: human lean typed as "immediate-or-near-immediate",
with artifact (D) = Lean proofs about the Aeneas-translation of the disciplined core,
crown-jewel theorems including "the sparing-algebra laws" — i.e. Lean proofs of the
same laws, now, spike-gated. (c) `amendment-lean-spike-results`: a hand-written Lean
model of 277-as-written already exists with the laws proved — the "handwritten-Lean"
alternative the same ledger records the human as dispreferring
("Aeneas-over-handwritten-Lean strongly preferred"), justifiable as a de-risking
statement-probe but never reconciled: does it discharge step 4 early? does the
at-settle trigger still exist? must the laws be re-proved against the Aeneas
translation, and is the hand-model then dead weight or a permanent second sync
obligation? The Lean-vs-Dafny "language tension held open" of turn06 item 5 is
meanwhile silently closed by facts on the ground (spike in Lean; Aeneas targets Lean)
with no recorded decision. A resuming conductor cannot reconstruct the governing
trigger from the record. This is suspicion-2 confirmed at the bookkeeping level —
though note the amendments are individually honest about what they replace (each names
its supersession explicitly); the incoherence is across amendments, not within them.

### 5. fnd-aeneas-gate-measures-wrong-risk — +SURE on what the gate tests; ~SUSPECT on how much it matters

**Claim.** The Aeneas go/no-go spike gate tests translation feasibility, but the
round's own grading says the load-bearing unknown is proof-repair-under-refactor — and
the gate does not touch it.

**Evidence.** `amendment-aeneas-pulled-forward` sequencing: "The de-risking spike
(days): carve the core behind the facade, run Charon/Aeneas on it as-is, read
jxl-proofs, report **what translates and what falls out**." Turn05 (mirrored in
turn06): "Proof stability under Rust refactoring — the vendor claim is
regenerate-and-replay… **no data, no effort figures, no failure rate**. ~SUSPECT —
this is the load-bearing claim for the churn question and it rests entirely on a
promotional blog" [B-symcrypt-verifying-rust-cryptography-2026]; plus the independent
team's finding that loop invariants on Aeneas-extracted recursive functions stayed
manual [B-rust-lean-pipeline-ai-provers-2026] — "loop invariants are what a fixpoint
engine is made of" (turn06's own words). The amendment answers with "proof-repair
sessions budgeted" (unpriced) and a gate that cannot observe repair rate, because a
days-scale one-shot translation never refactors anything. The hedge exists in the turn
record; it did not migrate into the amendment's gate design. Cheap fix: the gate
should include one deliberate refactor-and-replay cycle (rename, split a function,
change a container) with repair effort recorded.

### 6. fnd-certifier-priced-before-narrative-duties — ~SUSPECT

**Claim.** The certifier's price ("~50 lines", "certifier spike… ~1 agent-day") was set
before the narrative-30 re-weighting added duties, and never revisited after.

**Evidence.** Turn03 prices the bare check ("~10 lines, O(E)"); turn06 item 1 keeps
"~50 lines against the existing Lattice trait" while, in the same paragraph, adding:
closed-outcome `Certified | Refused(EdgeWitness)`, an operand-carrying collapse record
per `AID-NEEDS:law-collapse-mints-narrative`, partial-solution certification with
first-failing-edge localization, and (item 2's sibling duty) deterministic disclosed
k-cap. The session's own typed `rul-session-error-tier-ladder` holds that
Narrative-integrated nets are the ~2-orders-of-magnitude-expensive tier — the cost
driver named there is the aid-machinery integration, which the refusal lane now has
(at plan time, so the tier-3 label doesn't apply, but the integration cost does). The
"~50 lines"/"~1 agent-day" figures survive from the pre-narrative analysis into the
post-narrative portfolio unadjusted, and "the certifier ROSE twice under re-analysis"
books the added duties as added value with no added cost. Suspicion-3 confirmed in
miniature: the conductor's own favored mechanism is the one whose price never moved
while its scope grew.

### 7. fnd-domino-lemma-statement-hazard — ~SUSPECT (forward-looking; nothing wrong is yet on record)

**Claim.** The planned crown-jewel theorem — "certifier-accept ⇒ post-fixpoint ⇒
path-coverage (the domino lemma)" (`amendment-aeneas-pulled-forward` artifact D) — as
worded, needs one of two hypotheses the plan elsewhere disclaims, and the plan never
says which.

**Evidence.** Against `spike/crates/analysis/src/solve.rs` semantics the per-edge check
`transfer(v, states[v]) ⊑ states[w]` does establish equation-consistency
(post-fixpoint), and turn06's laws-condensed correctly notes this needs no quantifiers.
But "path-coverage" is an inductive claim: (i) read as abstract path-composition
coverage (MOP-inclusion), the induction step needs **transfer monotonicity** —
precisely the caller-upheld, un-type-enforceable precondition the whole certifier
argument celebrates not needing (turn02: "monotonicity is not a soundness
obligation"); it would re-enter as an unverifiable hypothesis on the very functions
that once violated the preconditions empirically. (ii) Read as concrete coverage
(every execution's state within γ of the node state), it needs per-operator soundness
of the transfer model — which `amendment-aeneas` explicitly scopes out ("the
model/transfer layer — explicitly out of scope for all of this — calibration
territory"). Either reading is workable if stated; the current wording implies the
certifier alone yields path-coverage, which no reading supports. This should be
settled before the Lean statement is authored — it is exactly the "proves a tidy
statement that is not the spec's statement" failure mode, one round early.

### 8. fnd-narrative-instability-overstated-against-determinism — +SURE on the phrasing being wrong; the underlying point survives

**Claim.** FLIP-3 (turn06), the round's self-declared "most important NEW finding",
says the 22W Eq-exclusion means "explanation varies **run-to-run** while the license
plane converges identically." At HEAD this is false as stated.

**Evidence.** The Eq-exclusion and k-cap are real (`spike/CLAUDE.md`
collapse-mints-narrative, ~L597-603: narrative records Eq-EXCLUDED from lattice
equality per 22W §2, k-capped). But `solve.rs` is deterministic by construction (FIFO
worklist, ordered collections, `solve_is_deterministic` test; `inv-determinism`):
identical inputs produce identical visit sequences, hence identical mint multisets.
There is no run-to-run variance to observe. The defensible content is
counterfactual/schedule-level: the explanation multiset is an artifact of the
scheduling policy rather than of the input's semantics, so it is unpinned under
worklist-policy change, node-renumbering refactors, or a future reactive engine (26C)
with real arrival orders — and convergence detection is blind to that by construction.
That is worth a pin; it is not a live nondeterminism, and "varies run-to-run" would
fail its own DST. The proposed permutation pins also require building a
schedule-permutation capability the current solver deliberately lacks — an unpriced
prerequisite.

### 9. fnd-seventeen-day-figure-unsupported — +SURE it doesn't match the recorded dates; ~SUSPECT confabulated

**Claim.** Turn06 twice says the vacuous-∅ bug class "shipped acked and survived 17
days" / was "caught 17 days later by crosscheck." The corpus's own dates give ~4 days.

**Evidence.** The set-lifting universal-meet law entered `notes/277` §5 on 2026-07-13
(`279f:fix-set-lifting`; 279f header date confirmed); it was acked typed 2026-07-16
(277 §5 inline ack, 2/5); the ∅-vacuity hole was credited 2026-07-17
(`27Xf:cr-set-lifting-vacuous-at-empty`; 27Xf header date confirmed). Ship→catch = 4
days; ack→catch = 1 day. No chain through 275 (2026-07-12) or the 271 sittings
(2026-07-10..12) yields 17. The likeliest origin is an echo of the catch *date* (the
17th). Minor in isolation; not minor as a pattern — a manufactured-feeling precision
in the synthesis-of-record, inflating (~4x) the survived-window that motivates
portfolio item 2's urgency.

### 10. fnd-minor-accuracy-cluster — +SURE each; individually trivial

- "FIVE hand-picked samples per combinator" (turn03/turn06 item 2): only the Powerset
  law-test uses five samples; Flat/Product/MapL/orientation tests use four
  (`lattice.rs` tests). Cosmetic, but it is the kind of over-neat number the record
  should not carry.
- `amendment-lean-spike-results`: "2 errors total, both tactic-level (**vs the
  27%-union benchmark prior**)" — an apples-to-oranges juxtaposition. The vericoding
  benchmark's Lean tasks are program-verification with loops and arithmetic; the spike
  is ~25 declarations of finite enums and Bool folds, terrain the spike's own REPORT
  calls "the best possible terrain for LLM-authored Lean." The REPORT scopes its
  viability claim correctly ("for THIS project's theorem shapes"); the ledger's
  benchmark comparison invites the unscoped inference.
- Turn06 item framing of the spike theorems generally: several of the five "families"
  are definitional unfoldings of functions the model itself defines
  (`sparesSet_iff_universal`, `no_sparing_without_flag`, wall-collides), and
  `cross_family_monotone` is true by the model's own structural choice (row-typed
  `selectorTier`) — the REPORT admits this ("structurally rooted"; "the
  structural-encoding trick bought more assurance per line than the theorems did").
  The proof content is thin by design; the value was the statement-pressure that
  produced the gap list. Downstream summaries should inherit the REPORT's framing,
  not "five theorem families PROVED."

---

## Considered and rejected

- **rej-kernel-claims-survive-reading** (suspicion-5 substantially fails). Every
  checked kernel claim held: the certifier formula `transfer(v, states[v]) ⊑ states[w]`
  matches `solve.rs`'s propagate-and-join semantics exactly (states[v] is the in-state;
  out flows to `flows_to(v)`); the iteration cap is `n*1024+4096` as quoted; `Must<L>`'s
  ⊑ is the dual order, so one checker does cover both orientations; `claim.rs` has
  precisely the four unrepresentability properties with compile_fail tripwires as
  described; MapL's canonical-form/semantic-Eq claim matches the code; the
  ANALYZER-NEEDS rows cited (an-monotonicity 435/783, an-backing-set-meet,
  an-lcg-quality, an-verdict-phase-keyed, the two 28P measured wrong-elisions) all
  exist and are characterized faithfully; `normalise_edits` exists
  (`spike/crates/plan/src/lib.rs`); `28R:adj-sparing-two-position-rule` exists (~L350)
  and matches the spike REPORT's convergence reading. Two pedantic asterisks only:
  "lattice.rs/solve.rs untouched since round 19" ignores a content-neutral
  encoding-repair commit (77d55a2e, 2026-07-26); and the five-samples nit above.
- **rej-pricing-flattered-flux** (half of suspicion-3 fails). The record shows the
  opposite of motivated pricing on Flux: turn03 argued Flux's marginal value on this
  kernel *down* against the human's stated prior, and the human's typed ruling
  (`rul-session-flux-lean-add-early`) reframed it back up. The analysis pushed against
  the preferred direction and was overruled — the healthy shape. (The motivated-pricing
  residue that does exist attaches to the conductor's own certifier — finding 6.)
- **rej-zero-sorry-rhetoric-at-source** (the suspicion against the spike's
  self-presentation mostly fails). README and REPORT carry the load-bearing caveat
  prominently and repeatedly ("the checker validates proofs, not STATEMENTS"; "no
  green build defends against formalizing the wrong sentence"; statement-uncertainty
  flags on #4, `SoundCompare`, `Member.wall`; "throwaway-grade except for the theorem
  statements"). The modeling-note-conditional-soundness entry is an exemplary honest
  hedge. The rhetorical residue lives in the ledger's compression, not the artifact
  (finding 10).
- **rej-amendment-survival-internally-incoherent**. `amendment-survival-reprioritized`
  is internally clean: it names what it replaces (the field-trial trigger "DIES"),
  preserves the three-way decomposition explicitly, records the conductor's two
  concessions including an admitted internal contradiction, and correctly defers the
  KNOBS `kSURVIVAL` status-line edit to the human (KNOBS still carries the old
  field-trial framing — deferred, not contradicted). The cross-amendment Lean-tier
  incoherence (finding 4) is the real defect; the intra-amendment suspicion fails.
- **rej-cedar-figures-misread**. The Table-1 arithmetic reproduces (5714/1673 ≈ 3.4;
  681 > 347), the ratio-correction against the mangled render is a genuine catch, and
  Cedar's use throughout (model+difftest shape, parser declined, custom set/map tax,
  Dafny abandonment at ~2k-LOC proofs) matches the quoted passages. The vericoding
  figures were re-verified raw with the model-union nuance, and the nuance
  substantially survives into turn06 ("27%-union/18%-best").
- **rej-aid-needs-cited-while-unread**. Turn06 discloses AID-NEEDS was never read yet
  cites `AID-NEEDS:law-collapse-mints-narrative` for the certifier's Refused shape.
  Checked: the law exists (root `AID-NEEDS.md` ~L76) and matches the usage — the
  conductor had it secondhand via `spike/CLAUDE.md`'s replicated bullet. The
  disclosure was honest and the citation accurate; process risk noted, no finding.
- **rej-citation-hygiene-broken**. All 22 distinct bracketed slugs across the six
  turns resolve against `sources.json`; the grading-reasoning fields consistently
  argue both directions (not-A/not-C); `A-verifying-rust-standard-library-2026` is
  honestly held at grading -0:SUSPECT and the n=1 SPARK cost figure is explicitly
  fenced ("do not quote it as a measured result") and, so far as found, never quoted.
  The spike REPORT's judgment-call flags (base-hash correction, label vocabulary,
  ELAN_HOME containment deviation, ubi deprecation) all check against the actual git
  record (9b05283f exists; the six commits ddabc5db..75b3ab60 exist as described).
  Substantial credit is due here.
- **rej-monotonicity-vs-finite-height-contradiction**. Turn03 attributes the empirical
  hang to `an-monotonicity`; turn06 calls it a finite-height violation ("fresh key
  minted per visit"). Not a contradiction: the ANALYZER-NEEDS row (~L323) lumps all
  three solve preconditions (monotone transfer + finite-height + semantic Eq) under
  the one slug; turn06's is a refinement within the row's scope.

---

## Closing judgment: typed decisions

No finding invalidates a decision the record marks as human-typed. The typed rulings
(`rul-session-error-tier-ladder`, `rul-session-knobs-not-load-bearing`,
`rul-session-code-is-a-model`, `rul-session-narrative-thirty`,
`rul-session-two-plane-firewall`, the survival-reprioritization rulings, and the two
typed *leans* — flux-lean-add-early and aeneas-pulled-forward) rest on meta-cost and
product-shape arguments that none of the findings touch; the laundered claims
(finding 1) decorate portfolio items but are not premises of any typed ruling, and are
probably true in the world besides. Two cautions short of invalidation: the certifier
spike, listed as "awaiting human go", should not be greenlit on turn06's
"pre-authorized" framing — the authorization it cites names different kernels at a
weaker grade (finding 2), so the go should be taken as a fresh decision with finding
6's repricing in hand; and the Aeneas lean's spike-gate should be amended per finding
5 before it is treated as the de-risking event the ledger presents it as. The
evict-maps/LatticeMap recommendation is correctly recorded as NOT yet human-acked;
nothing here changes its standing.
