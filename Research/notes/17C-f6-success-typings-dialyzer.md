# 17C — f6 gather: success typings / Dialyzer (round 17, 2026-06-07)

> First gather round (K2 thread, charter `170` front f6). Primary read **in full** + graded:
> [A-lindahl-sagonas-success-typings-2006] (PPDP'06; the Dialyzer/TypEr basis). Closes f6's include/kill
> rows and surfaces several cross-front bonuses — notably a citeable support for the oracle-owns-the-
> direction correction (`17B`). My notes use the `17[A-M]` letter range; the numeric `17x` range + `plans/177` are the still-running K1-series agent's.

## Findings (lifted)
- **f6-1 · inc-1 CONFIRMED — the polarity is Dorc's posture.** A success typing `(ᾱ)→β`
  *over-approximates the set of inputs for which the function can succeed*; using `f` outside `ᾱ` ⇒
  *definite* failure. The tool reports **only definite clashes** ("never cry wolf"); **"no program is ever
  rejected"**; an *unknown* function gets the ⊤ signature `(any())→any()`. = Dorc's optimizer-not-checker +
  ⊤-on-unknown + warn-don't-gate. +SURE.
- **f6-2 · kill-1 CONFIRMED — the inference *engine* is theirs, not ours.** The paper *is* a constraint-
  based, compositional bottom-up inference (call-graph SCC DAG; constraint gen/solve; k-depth + union
  widening for termination). Dorc *declares* anchors; corpus-mining is MAY-grade/offline (`096`). Take the
  polarity, kill the engine. *Nuance:* their "⊤-floor + refine-from-call-sites" two-phase shape mirrors
  Dorc's "declare-floor + infer-propagation" (base-1) — posture shared, machinery not. +SURE.
- **f6-3 · BONUS (load-bearing for oq-1) — a forgiving type-system may BAKE a global safety-direction only
  when it has a uniform, free runtime backstop.** Dialyzer bakes "never cry wolf" (fail toward
  *under*-reporting) because Erlang is runtime-type-safe: every value tagged + checked at runtime, so a
  missed warning degrades uniformly to "runtime still throws," never silent corruption. Dorc's backstop
  (re-run) is **not** free (`over-execute` = priority-2, `IMPLEMENTATION.md`) and **not** uniform (a skip's
  safe-direction is per-oracle/per-phase). ⇒ the precedent *supports the human's correction*: baking the
  direction is licensed by a uniform-free-backstop Dorc lacks ⇒ the **oracle** must declare the direction.
  ~SUSPECT→SURE; the sharpest cross-front find of the round.
- **f6-4 · BONUS — "dangerous to auto-generate types from comments/documentation"** (the `split/2`
  doc-vs-code discrepancy, App. A.1) independently corroborates the **kOOB no-comment-parsing redline** from
  a first-party angle: comments rot/mislead; derive from code-structure, not prose. +SURE.
- **f6-5 · BONUS (→ dq-entity-algebra) — union-of-singletons + widening-past-a-size-limit** is prior-art
  for the **≥enum bounded entity-state model** (nit-1): singleton types (`42`, `foo`) + disjoint unions,
  widened to a supertype past a fixed limit (+ depth-k abstraction) to stay finite = N-states-per-entity
  kept bounded under the `kCONTEXT` flat-domain pressure. Direct f6→dq-entity-algebra bridge. +SURE.
- **f6-6 · → kill-7 NUANCE + kill-8 support.** HM can't type idiomatic Erlang (`and`/`send`) without
  rewriting = "starting to program in a different language … not viable" (supports **kill-8**: HM breaks
  the no-rewrite/off-ramp). But they *need* subtyping (any/none/union) — **bounded** by widening. ⇒
  **kill-7 needs care:** kill *unbounded* subtyping lattices (cost/precision), but a bounded
  any/none + finite-union-widened lattice ≈ the flat domain Dorc already wants (it *is* how the enum-states
  sit). Don't kill the bounded version. ~SUSPECT.

## Citations
> [A-lindahl-sagonas-success-typings-2006]:p4 (relevance: +1:SURE)
> "A success typing is a type signature that over-approximates the set of types for which the function can
> evaluate to a value." … "DEFINITION 1 … whenever an application f(p̄) reduces to a value v, then v ∈ β
> and p̄ ∈ ᾱ."

> [A-lindahl-sagonas-success-typings-2006]:p5 (relevance: +1:SURE)
> "this application will definitely fail. This is precisely the property that a defect detection tool which
> never 'cries wolf' needs."

> [A-lindahl-sagonas-success-typings-2006]:p3 (relevance: +1:SURE)
> "One important property of a soft type system is that no program is ever rejected by the type checker."
> … "To eliminate noise and all false warnings, we optimistically assume that any expression will evaluate
> successfully if we cannot prove that it will result in some type clash."

> [A-lindahl-sagonas-success-typings-2006]:p5 (relevance: +1:SURE)
> "since the type signature (any()) → any() is a success typing, the analysis is free to use this signature
> for all functions which are unknown; because e.g. their code is not available. Besides making the analysis
> modular, it allows for some type clashes to be discovered early in the development process."

> [A-lindahl-sagonas-success-typings-2006]:p2 (relevance: -0:SUSPECT)
> "imposing a Hindley-Milner type system on Erlang requires modifications to existing code and amounts to
> starting to program in a different language, not in Erlang as we currently know it. … this is not a
> viable option." (kill-8: HM forces rewrites, breaks the off-ramp.)

> [A-lindahl-sagonas-success-typings-2006]:p11 (relevance: -0:SUSPECT)
> "it is very dangerous to automatically generate type signatures from comments or documentation."
> (corroborates the kOOB no-comment-parsing redline.)

## Carry-forward
- f6 "how it aged": in-paper (§6.4) — hundreds of bugs found in 10³–10⁶-LOC commercial code; ~700kLOC
  analyzed in ~30 min (scalable, faster than native compile). Dialyzer's ~20-yr production use in
  Erlang/OTP is well-known but **uncited here** (-GUESS; not a graded source).
- The optional-`-spec`-contracts × success-typings follow-on (Jiménez/Lindahl/Sagonas, Erlang Wksp 2007 —
  "A language for specifying type contracts…") is the **declare+infer-gradient** evidence (`kBURDEN`) →
  fold into **f7/f8** gather, not re-graded here.
