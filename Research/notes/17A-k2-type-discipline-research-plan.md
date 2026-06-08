# 17A — K2 research plan: the minimal type-discipline (round 17, 2026-06-07)

> **What this is.** K2's execution plan (the interactive-research GATE artifact, repo-`plans`-style but
> sited in notes as my series-opener). K1 holds `171`–`174` + the `plans/177` synthesis; both quarantined
> from this context per the round-17 firewall. Durable charter: `plans/170` (A0–A6 = the soundness
> machinery this map respects; Part C = fronts f6–f10). Deliverable: a **map-with-a-lean** — include/kill
> boundary, per-mechanism cited prior-art for each inclusion *and* each kill — closing as a `plans/17x`.
> Not a decision: the human runs the lean through adversarial-crosscheck after; `dq-entity-algebra` stays
> bounded. Trust repo-root `DESIGN`/`KNOBS`/`README`/`IMPLEMENTATION` over this AI prose.
>
> **Revised (round-17, turn 5):** integrated the human's kill-floor correction + 3 nits + the
> `IMPLEMENTATION.md` four-outcome ordering. The reasoning trail is `notes/17B` (append-only); this file is
> the living plan and was rewritten, not patched.

## The task in one line
Find the **least type-system that won't force worse work later.** Dorc's static layer is type-system-
*adjacent* (nominal kinds, a 2-element effect-map, `Opaque`=⊤, oracle-as-annotation), but Dorc *the tool*
is an **optimizer / query-planner, never a checker**: it never rejects. The in-bounds lineage is the
**forgiving** one (success / gradual / soft / pluggable typing, typestate, small effect systems,
occurrence typing). Orientation: **disprove / avoid / kill** — but the kill-criterion is *two-axis* (next
section); "simpler = safer" is true on one axis and false on the other.

## base — the floor, stated correctly (it is phase-keyed, and re-running is not free)
- **base-1 · declare-anchor + infer-propagation** (`kBURDEN`, `099 §5`): the anchor (a kind + how to
  probe/establish it) is **declared** by an oracle or directly-implied by an idiomatic book-guard;
  propagation is **inferred**. Spines in-corpus: [A-tobin-hochstadt-logical-types-2010] (latent-proposition
  narrowing), [A-foster-flow-sensitive-qualifiers-2002] (flow-sensitive qualifiers, strong/weak update),
  [A-lucassen-gifford-effect-systems-popl-1988] (effect lineage).
- **base-2 · ⊤-run is the floor, and it is phase-keyed** (`IMPLEMENTATION.md` §"To execute, or not":
  outcome priority `under-execute` ≫ `over-execute` ≫ `unnecessary-execute`). The *safe direction differs
  by phase*: probe-phase ⇒ withhold/skip-the-probe (no pre-plan mutation — a categorical promise);
  apply-phase ⇒ perform/run. The analyzer + type-discipline mostly live in the *probe* stage, where "skip
  it" is safe; it only reduces to "run it" at the whole-system level (probe-withheld ⇒ no fact ⇒ apply
  runs). A killed mechanism degrades *to this floor*.
- **base-3 · the floor is "no worse than blind-run" — with one exception.** Blind-running re-runs once and
  may hit idempotence errors; those we may replicate (minimize, not assume-away — idempotence is
  best-effort-hoped, never assumed). The exception: *multiple*-execution within a run (`over-execute`) is
  **worse than blind** (blind never multi-executes) and is priority-2, *above* the value-prop. So
  "re-running is harmless" is false; it is the least-bad direction when forced, still a failure-mode.
- **base-4 · the welds I stay inside** (A6): in-bounds = forgiving lineage; welded *out* = HM/full
  inference, dependent types, Cousot–Galois (`kVERIFY`); welded `kFAIL` (phase-keyed), `kCONTEXT`
  (flat-domain redline), `kLANG` (sh), `kVOLATILES` (hermetic).

## The keep/kill criterion is TWO-AXIS (the round-17 correction — read this before the map)
The kill-floor licenses aggressive killing on **one** axis only. (Full reasoning + strawmen: `notes/17B`.)

- **axis-depth** — *how hard the analysis thinks* (interproc, context-sensitivity, the precision
  *mechanism*). Killing ⇒ less certainty ⇒ *unknown* ⇒ run. Drains to over-run (priorities 2–3, never
  priority-1). **Kill-floor HOLDS**; kill judged on speed/value. +SURE.
- **axis-fidelity** — *what states exist & how finely distinguished; when oracle-A's state-dependency is
  discharged by oracle-B's state-test*. Killing here ≠ "remove certainty"; it means **coarsening**
  (collapsing distinct states/facts), which reads as *confident-wrong sameness* ⇒ over-correlation ⇒
  wrong-discharge ⇒ `under-execute` (priority-1, worst). **Kill-floor does NOT hold.** Failure is
  *non-directional* (false-neg ⇒ over-run safe; false-pos ⇒ under-run dangerous), so complexity is the
  wrong axis to reason about. +SURE.

**The correction (human, this turn): the engine must NOT choose the correlation's safety-direction.**
Baking a fail-direction into the engine is the superposition anti-pattern (A4/T11) and SF-3 (one fact
can't carry both phases' `kFAIL`). The safety-direction is the **oracle's to declare** — domain knowledge
only the author has. That is **kOOB** (author-config spelled in-band sh); naming-conventions are a prettier
spelling of the same oracle-declaration, and (X3, `151`) a 1-place namespace can't carry the 3-place
`(kind, provider, selector)` relation anyway ⇒ it bottoms into an analyzer-internal index lifted from
oracle sh. Absence of declaration ⇒ no correlation ⇒ ⊤ ⇒ run (the floor, *not* an engine direction-choice).

So the criterion splits:
- **kf-1** depth-axis features → kill-floor (judge on value).
- **kf-2** fidelity-axis features → the **fidelity-floor**: the model must distinguish ≥ the real entity's
  *mutation-gating* states (world-set, corpus-measured; **≥ enum**, never boolean — nit-1; systemd alone
  forces installed/enabled/active). Below it = priority-1 risk; *above* it the kill-floor re-applies.
- **kf-3** coordination features → keep only the **vocabulary** for oracles to declare correlations +
  directions, plus the lift into an internal index. The engine stays direction-**agnostic**; the choice is
  never ours.

## The map skeleton (provisional leans — earned/refuted by gather; each row tagged by axis)
### f6 — success typings / Dialyzer
- **inc-1 (depth/posture)** the success-typing *polarity* ("discrepancy, not error"; over-approximate the
  success set; warn-never-gate) = the optimizer-not-checker posture.
- **kill-1 (depth)** the success-type *inference engine* — floor covers it (declare, base-1; mining is
  MAY-grade/offline, `096`).
- *targets:* Lindahl & Sagonas, "Practical type inference based on success typings" (PPDP 2006); a
  Dialyzer retrospective for how it aged.

### f7 — gradual typing + the gradual guarantee
- **inc-2 (posture/law)** the gradual guarantee as the formal no-cliff law (adding an oracle never breaks a
  book that ran without it) — design law + differential test.
- **kill-2 (depth)** consistency-relation / cast-insertion machinery — `⊤` already is `Dyn`/`?` top; we
  never check or cast ⇒ collapses to `⊤ ⇒ run`.
- **kill-3 (depth)** the sound-gradual *performance* death — cast-free; the probe is an observation, not a
  boundary cast. (Verify the death is boundary-cast cost.)
- *targets:* Siek & Taha (Scheme Wksp 2006); Siek/Vitousek/Cimini/Boyland Refined Criteria (SNAPL 2015);
  Takikawa et al. "Is Sound Gradual Typing Dead?" (POPL 2016).

### f8 — soft + pluggable typing; the stub-library social model
- **inc-3 (welded-confirm)** pluggable-types' no-runtime-effect = the "oracle is a behavioral no-op"
  contract. *Now load-bearing for the correction:* pluggable types are the prior-art for
  *oracle-declares, engine-consumes, runtime-unchanged* — the shape the safety-direction declaration must
  take.
- **inc-4 (depth)** soft-typing's static/dynamic division (can't-prove ⇒ runtime check, not reject) =
  `⊤-run` + probe.
- **inc-5 (governance)** DefinitelyTyped/typeshed = the oracle-corpus model (community optional stubs;
  `effort-allocation`). Also the corpus evidence for *how authors split obligation* (feeds oq-2 + #7).
- *targets:* Bracha "Pluggable Type Systems" (~2004); Cartwright & Fagan "Soft Typing" (PLDI 1991) +
  Wright & Cartwright (TOPLAS 1997); DefinitelyTyped + typeshed (repos + governance).

### f9 — typestate + effect systems
- **inc-6 (carried)** occurrence-typing guard-lifting — [A-tobin-hochstadt-logical-types-2010].
- **inc-7 (fidelity+depth)** the minimal effect-map `(provider,verb)→{establish,kill}` —
  [A-lucassen-gifford-effect-systems-popl-1988].
- **kill-5 RE-SPLIT (was "kill heavy typestate"):** *keep* typestate's **state-distinctions** (axis-
  fidelity — must-keep to ≥enum: install/enable/active transitions are the multi-state model); *kill* only
  the **protocol-enforcement** (axis-depth — we never reject). The split is the point.
- **kill-4 (depth)** Koka-grade row-polymorphic effect apparatus — mine the framing, not the machinery.
- *targets:* Strom & Yemini "Typestate" (IEEE TSE 1986); Aldrich et al. "Typestate-Oriented Programming"
  (Onward! 2009); Leijen/Koka (MSFP 2014 / POPL 2017).

### f10 — minimality (the kill front) + posture anchors
- **inc-8 (fidelity)** nominal kinds (a declared name, not inferred-from-structure; A1).
- **inc-9 (fidelity, uncertain)** coherence (Wadler–Blott): multiple oracles grounding one kind must agree
  — candidate **un-dodgeable** rule (C3, `099`). Test: structurally required, or does the oracle-declared
  correlation + `⊤-on-conflict` already cover it? (Tied to the correction: coherence *is* the cross-oracle
  agreement the author declares.)
- **inc-10 (fidelity, uncertain; oq-2 lead, human-suggested) · ML-family open extensible sums** — row
  polymorphism + polymorphic variants (OCaml poly-variants/Garrigue; Wand/Rémy row variables; Leijen
  scoped labels): prior-art for *open, decentrally-extended* sums/variants — the shape of "a kind's
  state-enum stays open and providers extend it" (the structured pole of `dq-entity-algebra`; the
  open-extensible end of oq-2 — may a provider add a state without the kind-owner's prior declaration?).
  Lean (kill-orientation, per the human's overengineering caution): mine the *framing* (open variants;
  width subtyping; handling a case-subset); **kill** the inference apparatus (row unification, principal
  typing = HM-family, welded-out `kVERIFY`). Test: does an open state-set need any variant *machinery*, or
  just a declared open enum + ⊤-on-unrecognized-tag? *targets:* OCaml polymorphic variants (Garrigue
  1998/2000); row polymorphism (Wand 1987; Rémy; Leijen, "Extensible records with scoped labels", 2005).
- **kill-6 (fidelity-via-identity)** structural typing — nominal by A1; structure-inference = grounding.
- **kill-7 (depth)** subtyping lattices — `kCONTEXT` flat-domain; flat + ⊤ suffices.
- **kill-8 / kill-9 (welded-out)** HM/full-inference; dependent types / Cousot–Galois — named only.
- **pos-1** soundiness (Livshits et al., CACM 2015); **pos-2** TypeScript's unsound-by-design non-goals.
- *targets:* Livshits et al. (CACM 2015); Wadler & Blott (POPL 1989); TypeScript design-goals/non-goals
  (first-party); nominal-vs-structural (TAPL).

## dq-entity-algebra — bounded, with a floor (nit-1)
Flat (`package:nginx`) vs structured (`package:nginx{installed,version,held}`). **Lower bound = ≥enum**: a
flat *list of N states* per entity (boolean is the degenerate default), because real entities gate
mutations on >2 states (the fidelity-floor, kf-2). Above that, bounded-not-settled — selectors
([A-tobin-hochstadt-logical-types-2010]) push structured; strong-vs-weak update is gated by uniqueness
([A-foster-flow-sensitive-qualifiers-2002]); the shape is high-lock (`kCONTEXT`/`kFACTS`). I map both
poles + what each licenses; I don't pick.

## Firewall — relaxed (oq-3, human this turn)
The firewall was **K1-protection** (keep type-theory's Turing/correctness tar-pit from drowning K1's
spelling thread), *not* a K2 constraint. So I **follow the research where it leads**, including into
grounding/identity when it bears on the type-discipline (e.g. selector-*meaning* pinning — split-2's risk,
`17B`). I still *flag* cross-referent identity for synthesis (K1 is working it separately), but I no longer
hard-stop-and-hand-off on it.

## Method & sequencing
- **m-1 · gather f6 → f10, in main context.** Each turn → one appended note in the `17[A-M]` **letter**
  range (plan=17A, reasoning=17B, f6=17C; next f7=17D…). The numeric `17x` range + `plans/177` belong to the
  still-running K1-series agent — I do **not** touch its files (defensive: avoid clobbering a live agent).
  Echo TaskList each turn.
- **m-2 · grade every kept source** via `new-source.sh` (read-then-grade); verify every pointers-file
  venue/year (all AI-GUESS). Close artifact-touching turns with `validate.sh`.
- **m-3 · oq-1 is a working hypothesis,** not a settled premise (human: "probably correct, not entirely
  convinced"). Reason about it as I gather; keep notes; if it doesn't emerge as provable, leave it open
  pending the design-spike. Thread #7 (coordination contract) carries it.
- **m-4 · close with the map** (`plans/17x`, after the notes-series), carrying the two-axis criterion
  front-and-center; `dq-entity-algebra` left open.

## Open questions
- **oq-1** (MUST-grade-to-correlate; reframed by the correction): only an oracle's explicit declaration
  creates a correlation *and* supplies its safe-fail-direction; the engine never infers/directs one;
  no-declaration ⇒ run. *Working hypothesis* — prove or leave-open-for-spike.
- **oq-2** (author-obligation split): for a multi-state entity touched by several oracles — do we bless
  *one* structure authors must use (uniform, easy to lift, may not fit all kinds), or leave it *per-kind*
  (a `mode`: monolithic vs RAL-enum vs per-facet, author's choice — flexible, more coordination-surface)?
  Open; informed by f8 governance evidence + #7.
- **oq-3** *(answered: firewall relaxed — see above)*.

*Provenance: `plans/170` (charter), `16P`/`16Q` (spike T4/T5/T11/T12, dq-*), `099`/`092` (the spines),
`151` (CONVERGENCE/SF-1/SF-3/X3), `IMPLEMENTATION.md` (the four-outcome ordering), `notes/17B` (this
round's kill-floor reasoning), `learning-path/gradual-success-typing.ai-pointers.md` (Tier-A/B/C).*
