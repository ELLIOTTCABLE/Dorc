# 17G — f10 gather: minimality kills + posture + coherence + ML open-sums (round 17, 2026-06-07)

> Charter `170` front f10 (the kill front). Graded: [B-livshits-soundiness-manifesto-2015] (pos-1, read
> full), [B-typescript-design-goals-2020] (pos-2, fetched), [A-wadler-blott-ad-hoc-polymorphism-1989]
> (inc-9 coherence, read intro+mechanism), [B-garrigue-polymorphic-variants-1998] (inc-10 open sums, read
> intro+basics). kill-6/7/8/9 rest on these + prior fronts (no new dedicated source). This front also
> converges four separately-found hazards into one.

## Findings (lifted)
- **f10-1 · pos-1 soundiness IS Dorc's posture, named.** A *soundy* analysis = a **sound core**
  (over-approximate most features) + a **deliberately under-approximated** subset of hard features
  (eval/reflection/…), with that unsoundness **documented and evaluated** against observable behavior.
  Every realistic whole-program analysis is soundy; "soundy is the new sound." = Dorc exactly: sound floor
  + deliberate ⊤ on the undecidable subset (eval, dynamic words) + best-effort calibration (`kVERIFY`).
  The manifesto's *call to document + empirically evaluate* the unsoundness = Dorc's differential testing
  + the back-prop loop (17D correction). It even names TypeScript's unsoundness as a useful exemplar. +SURE.
- **f10-2 · pos-2 TypeScript = the industrial statement of the same posture.** TS non-goal #3: "Apply a
  sound or 'provably correct' type system. Instead, strike a balance between correctness and productivity."
  + goal #9 "fully erasable … type system" + non-goal #5 "do not … rely on run-time type information …
  encourage patterns that do not require run-time metadata." ⇒ TS is **erasable (no-runtime-effect) AND
  unsound-by-design AND productivity-over-correctness** = Dorc's posture (and reinforces cast-free, f7).
  DIVERGENCE: TS is **structural** (goal #9); Dorc is **nominal** (A1, inc-8) — see kill-6. +SURE.
- **f10-3 · inc-9 coherence — REAL requirement, NOT fully dodgeable, NOT enforceable ⇒ a contract+lint.**
  Type classes (Wadler-Blott): a **class** groups operations; an **instance** implements them per type;
  resolution picks the instance (dictionary). Map: **kind = class, oracle = instance**. Coherence = the
  meaning is well-defined only if the instances are *unique/agree*. Adjudication for Dorc:
  - it is NOT fully covered by `⊤-on-conflict` — that fires only on *explicit* disagreement; the dangerous
    case is *silent* same-name-different-meaning (both oracles say `installed=true`, meaning different
    things) → no conflict → wrong discharge (= 17B over-correlation, f8 shared-name/divergent-meaning).
  - Dorc can't ENFORCE coherence (never rejects). ⇒ coherence = a **required cross-oracle contract** (the
    kind owns the canonical meaning of each state; providers must honor it), **machine-checked best-effort**
    (DT-style CI, f8) and otherwise **trusted but not depended-on** (tenet-0). The one candidate
    un-dodgeable *rule*, realized as a contract, not a checked property. ~SUSPECT (lean: INCLUDE as contract).
- **f10-4 · inc-10 ML open-sums — mine the framing, kill the apparatus (as the human cautioned).**
  Polymorphic variants (Garrigue): an open variant `[> ...]` "is polymorphic, and can be extended by the
  addition of new tags"; tags are reusable across types; and "types do not interfere with evaluation"
  (parametric ⇒ no-runtime-effect, aligns with inc-3). Useful FRAMING: the open(`>`)/closed(`<`) distinction
  = an **open vs closed kind-state-enum** (may a provider add a state?); width-subtyping = **handle a subset
  of states**. KILL the apparatus (structural-polymorphism type *inference* / row unification = HM-family,
  welded `kVERIFY`). And the footgun — tag-reuse means **the same tag can mean different things in different
  contexts** — is *the same hazard as coherence* (f10-3). So poly-variants give the open-extensibility
  shape **and** independently re-raise the coherence requirement. ~SUSPECT (framing-only).
- **f10-5 · the KILLS (rest on prior fronts; named with citations):**
  - **kill-6 structural typing** — Dorc nominal (A1); structural type-*tests* break the gradual guarantee
    (`17D` §5.5); structure-inference = the identity/grounding problem (K1). TS is structural-and-unsound
    (f10-2) — a cautionary contrast, not a model for kind-identity.
  - **kill-7 subtyping lattices** — kill *unbounded* subtyping (`kCONTEXT` flat-domain; precision `kPRECISION`
    trades away); KEEP the *bounded* any/none + finite-union-widened lattice (f6-6) — that's how the ≥enum
    states sit. (Nuance, not a blanket kill.)
  - **kill-8 HM / full inference** — welded (`kVERIFY`); HM forces code rewrites (f6, success-typings) and is
    *restrictive* — "cannot support polymorphic recursion" (Bracha, f8). Named only.
  - **kill-9 dependent types / Cousot-Galois full soundness** — welded (`kVERIFY`, A6); soundiness (f10-1) is
    the empirical backing: no realistic whole-program analysis achieves full soundness; pursuing it destroys
    precision/scalability. Named only.
- **f10-6 · THE CONVERGENCE (the round's second spine).** Four separately-gathered hazards are ONE: inc-9
  coherence (instances must agree) ≡ inc-10 poly-variant same-tag-different-meaning ≡ f8 DefinitelyTyped
  shared-name/divergent-meaning ≡ 17B over-correlation (false-positive correlation ⇒ under-execute). All
  say: **cross-referent agreement on what a name/selector MEANS is the load-bearing, un-dodgeable core** —
  and it is exactly the K1/K2 seam (the *meaning* of a kind's states). This is the strongest cross-front
  result and feeds #7 + the synthesis directly.

## Citations
> [B-livshits-soundiness-manifesto-2015]:p1 (relevance: +1:SURE)
> "typical realistic analysis implementations have a sound core … some specific language features … are
> best under-approximated … We introduce the term soundy for such analyses … as sound as possible without
> excessively compromising precision and/or scalability."

> [B-livshits-soundiness-manifesto-2015]:p3 (relevance: +1:SURE)
> "the type system of TypeScript is unsound, yet practically very useful for large-scale development. Soundy
> is the new sound … Papers involving soundy analyses should both explain the general implications of their
> unsoundness and evaluate the implications for the benchmarks being analyzed."

> [B-typescript-design-goals-2020]:§Non-goals (relevance: +1:SURE)
> "Apply a sound or 'provably correct' type system. Instead, strike a balance between correctness and
> productivity." (+ Goal 9 "Use a consistent, fully erasable, structural type system"; Non-goal 5 "Add or
> rely on run-time type information … encourage programming patterns that do not require run-time metadata.")

> [A-wadler-blott-ad-hoc-polymorphism-1989]:p1 (relevance: +1:SURE)
> "Type classes permit overloading … During the inference process, it is possible to translate a program
> using type classes to an equivalent program that does not use overloading." (class = kind, instance =
> oracle; the translation is well-defined only if instances cohere.)

> [B-garrigue-polymorphic-variants-1998]:p1 (relevance: +1:SURE)
> "'>' means that a type is polymorphic, and can be extended by the addition of new tags." … "it is indeed
> parametric, in that types do not interfere with evaluation … but instances are restricted." (open
> extensible sums; no-runtime-effect.)

## Carry-forward
- coherence (inc-9) is realized as a CONTRACT (kind owns canonical meaning) + best-effort CI lint, not an
  enforced rule — carry into #7 and the synthesis; it is the same object as the over-correlation hazard.
- f10-6 (the meaning-agreement convergence) is the K1/K2 seam; flag prominently for the synthesis (it is
  where K2's coordination semantics meets K1's grounding).
- Dependent-types / Cousot-Galois not separately sourced (welded, named) — fine for a kill.
