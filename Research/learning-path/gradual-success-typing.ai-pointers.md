# Gradual / success / soft typing — pointers (AI-generated; verify before trusting)

> **Provenance + status.** AI-generated leads (Claude, from the 2026-06-06 "is-this-a-type-system?"
> conversation), **not** human-curated — kept *out* of `README.md` on purpose (that file is
> human-authored-only by stated policy). Every venue/year below is from model memory: treat as
> **GUESS-unverified** until checked. Re-curate into the README in your own words if any survive.
>
> **The frame (why these and not HM/Coq).** Dorc's *static layer* (kinds, effects, `Opaque`=⊤,
> oracle-as-annotation) is a **gradual effect system**; but Dorc *the tool* is an
> **optimizer / query-planner, not a type-checker** — it never rejects (`⊤ ⇒ run`; `inv-top-reject` +
> the apply-1 floor). So the relevant lineage is the *forgiving* one: type systems engineered for
> untyped-languages-in-the-wild, where "optional / never-block / fall-back-to-runtime" are *design
> goals*, not compromises. These threads feed the "build Layer A deliberately" prescription
> (`dq-entity-algebra`, `Research/plans/16Q-next-spike-and-process.md` §3) and the `kBURDEN` / no-cliff /
> best-effort posture. This is *encoding/contract* theory — **orthogonal** to the dataflow-engine path
> in `README.md` (that's the analysis machinery; this is the type/effect language it carries).

## Tier A — closest to Dorc's posture (read first)

- ★ **Success typings — Lindahl & Sagonas, "Practical type inference based on success typings"**
  (PPDP 2006; the Dialyzer/Erlang basis). -GUESS(venue). *The* "never cry wolf" discipline: infer the
  **largest** type a function *could* succeed with (over-approximate the success set); specs are
  **optional** + gradually addable; never blocks running, only reports a *definite* clash.
  → **Dorc-need:** the entire best-effort / warn-don't-reject posture; `kFAIL` "over-approximate on the
  safe side"; oracles-as-optional-specs. If you read one thing, read Dialyzer's philosophy (note its
  "discrepancy, not error" framing — same instinct as our "warn, never gate").

- ★ **Gradual typing — Siek & Taha, "Gradual Typing for Functional Languages"** (Scheme Workshop 2006),
  **+ the gradual guarantee — Siek, Vitousek, Cimini & Boyland, "Refined Criteria for Gradual Typing"**
  (SNAPL 2015). -GUESS(venue/year). The `Dyn`/`?` type + **consistency** (a *relation*, not subtyping)
  is precisely our `Opaque`/⊤. The **gradual guarantee** — "adding annotations only refines a working
  program, never breaks it" — is the formal statement of DESIGN's **no-cliff**: adding an oracle must
  never break a book that ran without it.
  → **Dorc-need:** the formal device that makes "forgiving" rigorous (not hand-wavy); the ⊤/consistency
  lattice shape for `dq-entity-algebra`.

## Tier B — the forgiving *mechanism* + the *library/social* model

- **Pluggable / optional type systems — Bracha, "Pluggable Type Systems"** (OOPSLA dynamic-languages
  workshop, ~2004). -GUESS. Types that **don't affect runtime semantics**, layered optionally = DESIGN's
  "oracle is a behavioral no-op" contract, exactly. Pairs with the **library model** as working
  proof-of-concept for the *oracle corpus*: **DefinitelyTyped** (`.d.ts` stubs for untyped JS) and
  Python's **typeshed** — community-grown, decoupled, optional type-stubs for third-party untyped code.
  → **Dorc-need:** the authoring/distribution/governance model for oracles (`effort-allocation`:
  bootstrap ~40-50, community grows the tail); the no-runtime-effect contract.

- **Soft typing — Cartwright & Fagan, "Soft Typing"** (PLDI 1991); **Wright & Cartwright, "A Practical
  Soft Type System for Scheme"** (TOPLAS 1997). -GUESS. Infer statically; where you *can't* prove safety,
  **insert a runtime check** rather than reject. Dorc's "fall back to just running the command" *is* that
  runtime check.
  → **Dorc-need:** `kDEPS` static-derive-with-runtime-backstop; the static/dynamic division of labour.

## Tier C — adjacent machinery for specific sub-problems

- **Effect systems — Lucassen & Gifford, "Polymorphic Effect Systems"** (POPL 1988) (orig. Gifford &
  Lucassen, LFP 1986); **Koka — Leijen**, row-polymorphic effect types (MSFP 2014 / POPL 2017). -GUESS.
  Your `(provider, verb) → {establish, kill}` *is* a tiny effect system; this is the formal frame if
  effects ever need enrichment (idempotent / ordered / commuting / region-scoped). ~SUSPECT Koka-grade
  row machinery is heavier than Dorc wants — mine the *framing*, not the apparatus.
  → **Dorc-need:** the effect-map's type-theory; the effect vocabulary in `dq-entity-algebra`.

- **Occurrence typing — Tobin-Hochstadt & Felleisen, "Logical Types for Untyped Languages"** (ICFP 2010);
  Typed Racket. -GUESS. Flow-sensitive *refinement of a type by a runtime test in a conditional* —
  "inside `if (test x) { … }`, x is refined." That is **exactly** the guard-position problem:
  `[ -f X ] && …` tells you something about `file:X#exists` in the then-branch.
  → **Dorc-need:** DESIGN §7-flavoured path/guard sensitivity; the `if cmd; then …` lifting; the spike's
  guard handling. ~SUSPECT this is the single sharpest pointer for the *guard-lifting* half of the
  analyzer (distinct from the effect/skip half).

## Deliberately *not* here
- The **entity-identity** nightmare ($PATH-derivation, cross-manager package identity, the
  versioning-lattice) is a *different* literature (abstract domains for strings; equivalence/ontology)
  and the genuinely-hard, deferred corner of `dq-entity-algebra` — **not** the gradual-*encoding*
  question. Keep it separate so it doesn't contaminate this thread.
- **Sound/total** type theory (Hindley-Milner, dependent types, full Cousot Galois machinery) — `kVERIFY`
  is welded away from it ("TypeScript, not Coq"). The forgiving lineage above is the relevant one; the
  sound lineage would buy a soundness Dorc *structurally cannot collect* (`T12`).
