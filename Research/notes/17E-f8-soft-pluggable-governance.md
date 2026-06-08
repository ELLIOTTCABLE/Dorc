# 17E — f8 gather: soft/pluggable typing + the stub-library governance model (round 17, 2026-06-07)

> Charter `170` front f8. Read full + graded: [B-bracha-pluggable-types-2004] (optional/pluggable typing).
> Read full + graded (governance): [B-definitelytyped-governance-2020] (DefinitelyTyped's actual process)
> + [B-pep-561-distributing-type-info-2017] (stub distribution + resolution order). Soft typing
> (Cartwright-Fagan) NOT separately graded — characterized via the f6 read (success-typings §2.2) + Bracha's
> related-work. This front lands the optional-lint/back-propagation framing (17D correction) on real prior-art.

## Findings (lifted)
- **f8-1 · inc-3 CONFIRMED — pluggable/optional typing IS the oracle-no-op contract, by its prior-art
  name.** Bracha's optional type system: (1) **no effect on run-time semantics** ("much more significant …
  a very stringent requirement") + (2) no mandated annotations. = DESIGN's "oracle is a behavioral no-op",
  and = the human's "runtime enforcement as optional lint" (17D correction). "Pluggable" = zero/one/many
  analyses coexist, all optional, all no-runtime-effect; "a very wide range of static analyses can be cast
  as type systems" = Dorc's multiple oracles/analyses-as-plugins. +SURE.
- **f8-2 · Bracha independently motivates defense-in-depth tenet-0 (from the typing side).** Mandatory
  types breed an "irresistible temptation to rely upon" them ⇒ they become "a basis for optimizations and
  security guarantees that fail ungracefully if the underlying assumptions … do not hold. If the type
  system fails, system behavior is completely undefined." And real type systems *do* fail (formalizations
  make simplifying assumptions; implementations have bugs). ⇒ **no layer should depend on the type
  system's correctness** = tenet-0, and = the T12 resolution (oracles can lie → don't let the floor /
  apply depend on oracle correctness). The forgiving lineage reaches the same conclusion the human did. +SURE.
- **f8-3 · take Bracha's PRINCIPLE, reject his MECHANISM (a kOOB landmine).** Bracha's optional/no-runtime-
  effect/pluggable *principle* is exactly Dorc's. But his proposed *spelling* — "types are just a kind of
  metadata" (annotations on AST nodes) — is the **kOOB-forbidden** form (sidecar annotation). Dorc spells
  the same information via idiomatic-sh-narrowing (`092`), not annotations. Principle yes, annotation-
  mechanism no. +SURE. (This is the cleanest statement yet of *why* Dorc diverges from the PLT default.)
- **f8-4 · inference is optional + may be UNSOUND + must be DECOUPLED (Bracha §4) — blesses MAY-grade
  mining + the back-prop loop.** "The inferencer need not be sound — it can use heuristics that can fail …
  it can infer types based on variable names. Multiple inferencers … pluggable type inference." = Dorc's
  MAY-grade distributional hints (`096`) and the back-propagation loop (an unsound inferencer *suggests*,
  the human *confirms* into the static corpus). Supports **kill-1** (inference is separable/optional, not
  load-bearing) and **kill-8** (HM's mandatory inference is *restrictive* — "cannot support polymorphic
  recursion"). Bracha's critique that soft typing "coupled [inference] … not truly optional w.r.t. the type
  checker" = a tenet-0 violation to avoid: keep Dorc's analysis decoupled from inference. +SURE.
- **f8-5 · inc-5 CONFIRMED — the corpus-governance model, and the real-world proof the back-prop loop
  scales.** DefinitelyTyped (~10k contributors, ~250 PRs/wk) + typeshed/PEP-561 give a concrete oracle-
  corpus model with four transferable parts:
  - **decoupled stubs** — type-knowledge lives in a community corpus *separate* from both the upstream
    source and the tool (DT monorepo; PEP-561 `foopkg-stubs` shipped separately). = oracle decoupled from
    the wrapped command *and* from Dorc; anyone can contribute one for anything.
  - **machine-enforced, NOT author-disciplined** — DT stays trustworthy via independent CI (dts-critic:
    stub-shape ~matches the JS; downstream-break tests; per-module dtslint; nightly compiler-vs-corpus),
    not author trust. = `151`-X4's "machine-enforced not author-disciplined" lesson confirmed at scale, and
    a textbook tenet-0 instance (an independent CI layer that does not depend on the author layer).
  - **tiered governance by impact** — popular libs (>5M dl/mo) need a maintainer; the long tail self-merges
    after owner review. = `effort-allocation` (bootstrap the high-impact core, let the tail self-serve).
  - **a precedence/resolution order** — PEP-561: manual-override > user-code > stub-packages > inline
    py.typed > typeshed. A declared precedence over *overlapping* type-sources → a **4th coordination
    option for oq-2/#7** (when several oracles describe one kind, precedence resolves it — beside the
    monolithic / RAL-enum / per-facet splits).
- **f8-6 · inc-4 CONFIRMED (soft typing) — no new source.** Soft typing inserts a runtime check where it
  can't prove safety rather than rejecting (f6 success-typings §2.2; Bracha [CF91]) = Dorc's `⊤-run` +
  probe / `kDEPS` static-derive-with-runtime-backstop. Caveat (Bracha): soft typing wrongly *coupled*
  inference to checking — Dorc keeps them decoupled (tenet-0, f8-4).

## Caution carried to oq-2 / #7
DT handles many typings coexisting via **namespacing** (the contributing guide: "use a module to avoid
conflicts … from another typings") + the PEP-561 **precedence order** — i.e. the real world's answer to
the 1-place-clobber / 3-place-relation problem (X3, `151`) is *namespacing + declared precedence*, not a
shared global slot. Feeds the coordination contract.

## Citations
> [B-bracha-pluggable-types-2004]:p2 (relevance: +1:SURE)
> "An optional type system is one that: 1. has no effect on the run-time semantics of the programming
> language, and 2. does not mandate type annotations in the syntax. The former point is much more
> significant than the latter. It is in fact a very stringent requirement."

> [B-bracha-pluggable-types-2004]:p2 (relevance: +1:SURE)
> "Once a mandatory type system is in place, the temptation to rely upon it is irresistable. It becomes a
> basis for optimizations and for security guarantees that fail ungracefully if the underlying assumptions
> of the type system do not hold. If the type system fails, system behavior is completely undefined."

> [B-bracha-pluggable-types-2004]:p5 (relevance: +1:SURE)
> "The inferencer need not be sound - it can use heuristics that can fail on occasion. For example, it can
> infer types based on variable names. Multiple inferencers … can coexist … pluggable type inference."

> [B-bracha-pluggable-types-2004]:p4 (relevance: -0:SUSPECT)
> "one takes the view that types are just a kind of metadata." (the annotation MECHANISM Dorc rejects — kOOB.)

> [B-definitelytyped-governance-2020]:§"How did we get here" (relevance: +1:SURE)
> "We run performance and integration tests on every change … dts-critic, which gives everyone some more
> confidence that their types fit the JavaScript … the change doesn't break downstream DT libraries."

> [B-definitelytyped-governance-2020]:§"What PRs Can Be Merged" (relevance: +1:SURE)
> "Changes to extremely popular libraries with over 5,000,000 downloads per month … cannot be merged by
> definition owners and still require a DT maintainer … most of the pull requests … are quick changes to
> smaller libraries." (tiered governance by impact.)

> [B-pep-561-distributing-type-info-2017]:§"Module Resolution Order" (relevance: +1:SURE)
> "1. Stubs … manually put in the beginning of the path … 2. User code … 3. Stub packages … SHOULD
> supersede any installed inline package … 4. Packages with a py.typed marker … 5. Typeshed." (precedence
> over overlapping type-sources.)

## Carry-forward
- Soft typing (Cartwright-Fagan PLDI'91) ungraded — pull only if the map needs the original soft-typing
  formalism beyond the f6/Bracha characterization (-GUESS: it won't).
- The PEP-561 precedence order is a genuinely new oq-2 option (precedence/override) — record it in the
  coordination map (#7) alongside the 3 author-obligation splits.
- JavaCop ("A Framework for Implementing Pluggable Type Systems", OOPSLA'06) surfaced as the *mechanism*
  for pluggable checkers — only relevant if we ever build a pluggable-checker framework; note, don't gather.
