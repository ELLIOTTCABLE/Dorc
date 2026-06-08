# Round 9 synthesis — state-tracking: the design-space map (conclusion)

> **Status (2026-06-02): round-9 conclusion.** The interactive-research skill's `conclusion.md`, in the
> repo's `plans/` convention. Deliverable per **q-scope (a)**: *design-space narrowing* — walls / floor /
> contracts / knobs, correlated to the graded prior-art (notes 091–096) and `KNOBS.md` slugs. **Not
> DESIGN.md** (the human owns that synthesis; some findings here may get *linked* from it later).
> Confidence-marked. A parallel research-agent is grounding several of these concepts in real-world
> source-code; those examples may augment the prior-art column — this doc states the *shape*, not the
> sample.

## 0. The result in one paragraph
Dorc tracks **relational contracts over referent-agnostic shared-state symbols** — it need not know what a
`package` *is*, only faithfully keep the promise "this establishes / this requires / these don't conflict"
made by whoever does (the human-adjudicated relational frame, 095). A skip is licensed **only** by a
**MUST-grade** contract — one *directly implied by idiomatic structure* (an idempotency guard) **or**
*declared by an oracle* (Engler's MUST vs MAY, 096); everything **mined/distributional is MAY-grade** — a
hint that bootstraps the oracle library, never a licence to elide. The analysis that propagates these
contracts must live on the **IFDS decidable floor** (finite facts + distributive transfer), because every
richer question is provably out of reach (Rice / Ramalingam / the frame problem, 093). Anything **unanchored,
undecidable, or transient** collapses to ⊤ → just run it — which is always safe and never worse than not
using Dorc. The probe is not magic and not "grounding": it merely **executes the oracle's promised check**
on the real host, supplying the one live observation static analysis is forbidden to deduce.

## 1. Dorc's coordinates — what kind of thing this is (091)
~SUSPECT the sharpest available framing (Traugott's trichotomy): Dorc seeks **congruent-grade outcomes**
(ordered, single-correct-execution, fire-and-forget) from **convergent-grade inputs** (partial, lazy
description), substituting *static-analysis + probing* for the fully-descriptive baseline `kDEPS-declare-world`
demands. No tool in the prior art occupies that cell. **Convergence is a neighbouring axis, not the floor**:
it is a *fallback method* Dorc may borrow (DESIGN's reserved "trivial convergence" re-run), not the ground it
stands on. The **unsolvability spine** is cross-camp-confirmed — both Traugott (ordered/congruent) and Burgess
(convergent) independently name "tracking state + understanding intentions" as the unsolved core — so the
user is *necessarily* in the loop, and the round's job is the **floor / frontier / ceiling** of that
involvement, not its elimination.

## 2. The walls — proved or forced, non-negotiable (093, 095)
- **W1 — Rice.** Non-trivial *semantic* properties are undecidable; *syntactic* ones are not. ⇒ Dorc **never
  decides behaviour**; it recognizes *contracted syntax* (decidable) + **probes** (observes) + **delegates**
  the residue to the human contract. The master ceiling *and* its escape.
- **W2 — Ramalingam.** Precise alias/footprint is undecidable — *even intraprocedurally* (PCP reduction). ⇒
  the footprint is **over-approximated** (⊤-on-unknown); the oracle **declares** what it touches.
- **W3 — the frame problem.** Non-effects can't be enumerated. ⇒ a **closed-world frame axiom**:
  *assume-unchanged-unless-an-operation-declares-otherwise*. The unsound-but-necessary core of every skip.
- **W4 — relational, not grounded (human-adjudicated, 095).** Symbol meaning is *parasitic* — it lives with
  the author/oracle, not the analyzer. Dorc is **referent-agnostic**: it keeps relational contracts about
  possibly-meaningless symbols; the symbol-grounding problem is the *human's*, off Dorc's plate. The
  chicken-and-egg (can't co-infer kind + command-semantics) is resolved **relationally** (declare the
  contract; Dorc relays), never by inference.
- **W5 — ambient vs transient (the original TODO).** A state-fact is **ambient** (stable resting value →
  probeable) or **transient** (created-and-destroyed within a run → *no* resting value → **un-probeable**).
  A guard reading a *transient* fact (`do_x; if x; …; undo_x`) is **non-hoistable** and must be tracked by
  pure in-script dataflow — never lifted to the resting-state probe. Misclassifying transient-as-ambient is
  *the* wrong-skip (`kFAIL-perform`).

## 3. The decidable floor — the one positive result to stand on (093 f20)
+SURE the soundness-floor discipline: **keep state-facts finite + transfer-functions distributive** (the IFDS
result: finite + distributive ⇒ precise, polynomial, decidable). Concretely = `kCONTEXT`'s "keep the abstract
domain flat" + a gen/kill effect-lattice. Step off it (infinite fact-domain, non-distributive transfer) and
you leave the decidable island → ⊤. This is where CQual's **strong-vs-weak update** trades precision against
W2: strong-update only on a provably-unique entity, else weak-update (⊤-ward).

## 4. The contracts — what the user supplies (the q-floor / frontier)
The deliverable's heart. The contract is **spelled in idiomatic sh-AST, not metadata/DSL** (AGENTS), and its
unit is the **idempotency guard** `if ! PROBE; then ESTABLISH; fi` / `PROBE || ESTABLISH`, where the **shared
argument** is the entity-link and **guard polarity** marks probe-vs-establisher (094). The `{what · who-type ·
where}` triple:

> **[REVISED→17N · UNSETTLED · 2026-06-08]** The claim that *the shared argument **is** the entity-link* — and C2's *"a consumer guard **is** a MUST-belief, directly-implied"* — is **downgraded to a may-grade *hint***, not a must-grade link. A guard's operand need not be its body's operand (`if ! dpkg -s conflicting_package; then apt-get install something`), so guard-structure does **not** establish the probe/establish linkage; even a shared variable is not a guarantee. **C1 (≥1 *declared* anchor) and C3 (cross-oracle = *named kind*) are unaffected** — those were always must-grade-by-declaration. How much is derivable oracle-less is genuinely unsettled; see `plans/17N` (F3, §3, and the may/must split). NB: line 121 of this doc already flagged the shared-arg-link as "to test" — the test came back negative for the must-grade reading.

- **C1 — q-floor (mandatory).** ≥1 **MUST-grade relational anchor per kind**: declare the kind + how to
  probe/establish it (the oracle's latent-proposition / qualifier-lattice, 092). Without a reachable anchor a
  command is ⊤ → run. This is the *irreducible* floor — the chicken-and-egg proves it non-empty (095).
- **C2 — frontier (graceful degradation).** Idiomatic guards in the *book* **propagate** the anchor for free
  (a consumer guard *is* a MUST-belief, directly-implied). Consumer-guard ≡ oracle: the same spec written
  incidentally (book) or deliberately+reusably (oracle); a **bare** command needs the oracle to supply the
  guard it omitted. The more idiomatically-guarded the book, the more is derived gratis.
- **C3 — coherence (cross-oracle).** Multiple oracles grounding one kind must **agree** (type-class coherence,
  095 f28): the kind is the *class*, an oracle is an *instance*. Cross-oracle identity binds to the **named
  kind**, never an accidental shared token.
- **C4 — hermeticity + a caveat.** Oracle verdicts must be hermetic (`kVOLATILES`), but *what counts as
  canonicalizable-volatile* is a per-fact contract with a **time-sensitivity hazard** (Traugott 091 f5: a
  bit "non-functional" now may matter to a future change). Weld holds; the boundary is contracted, not inferred.

## 5. Knobs touched — `KNOBS.md` deltas
- **`kFLATTEN`** — state-closure *is* the hoist-safety predicate: hoist a guard iff the fact it reads is
  **ambient ∧ invariant** to its execution point; transient/written-upstream ⇒ `maintain-cfg`. (W5.)
- **`kBURDEN`** — realized concretely as **declare+infer**: the *anchor* is declared (the latent-proposition /
  constant-qualifier), *propagation* is inferred (occurrence-typing narrowing / qualifier inference, 092).
  *Where on the gradient* stays open by design.
- **`kVOLATILES`** — weld stands; add the **time-sensitivity caveat** (C4).
- **`kFAIL` / `kPROBING`** (welded) — transient facts are *un-probeable* (probing one would mutate,
  `kFAIL-withhold`), so they force in-script reasoning or `just-run`, independent of cost.
- **No new knob proposed.** The round refines existing ones; it does not add to `KNOBS.md`.

## 6. The probe's role, clarified (095, human-adjudicated)
The probe **executes the oracle's promised check** on the real host and returns the verdict — the single
**apply-relevant live observation** that W1 forbids static analysis to deduce. It is *referent-agnostic*
(Dorc doesn't understand `nginx`; it runs the contracted check and trusts the promise) and it is **not
"grounding."** This keeps the probe firmly an *engineering* mechanism, not a semantic one.

## 7. Specification-mining's placement — off the elision path (096, human-refined)
Spec-mining (Ammons POPL'02; the field) is **statistical/voting by nature** (some sampled programs are buggy)
→ it produces **MAY-grade** beliefs (Engler). A *single Dorc run sees one book, not a corpus*, so mining has
**no role in per-run elision**. It lives entirely **offline**, in oracle-library bootstrap + author-linting:
rank which ~40–50 oracles to write first (the network-effect, DESIGN #4/#5). Restated plainly (the human's
read): "we need a good author-linter, and good linters need corpus analysis" — both already known; F7 is
*confirmation*, and it donates the **MUST/MAY vocabulary** that names the sound/unsound line.

## 8. The implementer's vocabulary — two formal spines (092)
- **Occurrence typing** (Tobin-Hochstadt & Felleisen): `(latent-proposition, object, substitution)` — a probe
  carries a *latent proposition* (its true/false claim), narrowing = substituting the accessed *object* into
  it. The **latent proposition is where the anchor (C1) attaches**; selectors ⇒ kinds are *structured*
  (`package` has installed *and* version). Lineage: "latent" is borrowed from Lucassen–Gifford effect systems.
- **CQual** (Foster et al.): state-facts as **flow-sensitive qualifiers**, strong/weak update gated by
  uniqueness (= W2 in operation); the **frame falls out of the effect-set** ("functions don't join locations
  they don't use"); declare+infer = `kBURDEN`. The qualifier *lattice* is the second anchor-locus.

## 9. Open / handed forward (not resolved here, correctly)
- **Where on `kBURDEN`** to land (open by design; corpus + user own it).
- **The contract notation is unwritten** — `{what/who/where}` is the *design-space* for a future
  language/UX pass; this round bounds it, doesn't author it.
- **Optional deepenings** (tasks #6/#7): type-class *coherence* mechanics (C3), the distributional primary,
  and a first-hand Ammons read.
- **Cross-host / fleet** (write-skew, `kSTATE`, `kCONTEXT` per-host) — lightly touched; a separate concern.
- **Real-world grounding** — the parallel agent's source-examples will test whether these idioms (the
  idempotency guard, shared-arg link, ambient/transient) actually occur as assumed.

## 10. Prior-art index (the graded base for this round)
| note | front | load-bearing sources |
| --- | --- | --- |
| 091 | ops-native state theory | [A-traugott-order-matters-2002], [B-burgess-cfengine-2010] |
| 092 | flow-typing / tainting / RAL | [A-tobin-hochstadt-logical-types-2010], [A-foster-flow-sensitive-qualifiers-2002], [A-lucassen-gifford-effect-systems-popl-1988] (in-corpus), Puppet RAL (archive); *demoted: ACSL/ShellCheck = `kOOB`-floor only* |
| 093 | impossibility ceilings/floor | [A-ramalingam-undecidability-aliasing-1994], [B-sep-frame-problem-2004], [A-reps-horwitz-sagiv-ifds-popl-1995] (in-corpus), Rice (canonical) |
| 094 | the idempotency-guard carrier | (design-reasoning; archive RAL) |
| 095 | grounding → **relational** (corrected) | [A-harnad-symbol-grounding-1990] (downgraded ~`-1:GUESS`, human-adjudicated over-reach); real content in RAL (092) + Burgess promises (091) |
| 096 | spec-mining / MUST-MAY boundary | [A-engler-deviant-behavior-2001]; Ammons POPL'02 (named) |
