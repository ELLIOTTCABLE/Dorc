# 301 — minispec + dorc-verify: the literate law-corpus and its earned-badge binder

> Tier: LLM-authored spec (Fable conductor, from the 2026-08-14 human design dialogue;
> round 30, "first half"). Subordinate to root docs and `spike/CLAUDE.md`; sibling of
> `notes/300` (the arc conduct ledger — lanes, staffing, and everything not
> minispec/verify-specific live THERE, never here). Grades: **[TYPED]** the human typed
> it · **[ACKED]** substance confirmed in dialogue · **[CONDUCTOR]** conductor-derived,
> unratified. Names marked STRAWMAN rename freely (`rul-strawman-formats-no-compat`).
> Prior-art grounding: the turn09 gather in
> `.claude/research/refinement-types-industrial-cost/` (duvet's typed citations; the
> frozen-TLS rot case; Verso; the tautological-contract finding).

## §0 — What this is, and its posture

**minispec** is a root-level, deliberately tiny corpus of literate Lean law-units — the
project's reviewable statement of the few kernel laws it opts to verify. **dorc-verify**
(STRAWMAN name) is the binder: a workspace crate + binary that computes each law's
earned coverage badges from machine-checkable evidence and emits the one generated
report. Both are internal instruments for wrangling LLM maintainers and for reviewers
who are not proof-literate; **neither is ever user-facing or marketed** [TYPED].

- **post-scope-is-a-judgment** [TYPED] — minispec structurally does NOT aspire to
  define every rule the product is built under, nor to encode engineering practice.
  Out-of-scope is a standing human call, exercised freely; deliberately outside, today:
  engine hyperproperties (determinism — DST's territory), process/conduct law
  (CLAUDE.md's), foreign-shell semantics (the two-binary floor differential's),
  cross-host/world semantics (DST + the runtime checkers'). The kill-search that
  stress-tested this design found the unprovable classes were all already
  out-of-scope — minor evidence FOR the approach. The catalogue never becomes a
  registry of everything; there is no taxonomy machinery deciding scope.
- **post-no-chasing** [TYPED substance] — gate-mode, never chase-mode: an algebra
  semantic change lands as a rule/statement change in the same fold as the code
  (model-leads), badge movements are loud promote-acts (§4), and coverage grows only by
  deliberate increment — there is no backlog to fall behind on. (Rot evidence: the
  turn09 frozen-TLS case died precisely because nothing broke when its referent moved.)
- **post-halo-is-the-hazard** — the named failure this system exists to prevent:
  readers (maintainers, implementing LLMs, future reviewers) inferring "there are
  proofs, so it's covered" from the existence of a lean directory. Every mechanism
  below either earns confidence mechanically or renders its absence.
- **post-checkable-states-are-a-feature** [TYPED] — some law-writing praxis will
  involve type-system/runtime refinements to the engine itself so that a law's states
  become checkable at all. That is a feature of the process, not scope-creep; such
  refactors are priced and dispatched like any engine work.

## §1 — The law unit: two siblings, one name

[ACKED — option B of the worked comparison; single-file embedding was rejected for
hot/cold churn-mixing (transcripts re-bless freely; the law file must stay diff-quiet)
and for forcing a second loom container format.]

- **naming** [TYPED] — Lean forbids hyphens in module names, so law-slugs leave
  kebab-case; we lean fully into the ecosystem's DromedaryCase: the law file, the loom
  file, and the catalogue slug are ONE identical string differing only in extension
  (`UnknownMemberCollides.lean` / `UnknownMemberCollides.loom` / `UnknownMemberCollides`).
  Convention dividend: DromedaryCase in prose = a hard law with machinery behind it;
  kebab-case = soft corpus reference. The ≥3-full-words slug discipline carries over.
- **the `.lean` unit** — hard-minimal and diff-QUIET: Verso prose carrying the
  English-authoritative law text, the `Prop` statement over the derived definitions,
  and the instance battery. NO metadata in-file [TYPED — metadata is dislocated to the
  catalogue so LLMs habitually load the ENTIRE file, prose first, before changing
  anything]. Churny material never lands here.
- **the `.loom` demo** — exactly the existing whole-product loom format (`run:` lane,
  `fixpoint: executed`, txtar fixture sections, `mocks/`, `expected.ran`, committed
  transcript), executed by the unchanged central runners and gate battery (the
  sole-sanctioned-executor law). ONE new frontmatter key, `law:`, joins
  `FRONTMATTER_KEYS` in the same commit that mints it (that vocabulary's own rule).
  Demo assertions sit at the OUTCOME tier — per-site `{elide, guard, run, survive}` —
  while the rendered transcript stays illustrative and re-blesses freely
  (`render-form-unwelded`; the USER_STORY discipline applied to spec demos).
- **user-pain closure** [ACKED] — a demo may declare `expect-diagnostic:` needles
  (structural, catalog-validated, asserted-to-fire), closing the chain
  law → seat → demo → the exact user-facing error, end to end, with existing machinery.
- **one rendered page** — the Verso doc-build includes the sibling loom's transcript so
  the RENDERED unit is a single continuous read: prose, statement, instances, live
  demo. (~SUSPECT on the exact Verso mechanism; the builder confirms; fallback is plain
  structured doc-comments + include-by-generation. Verso is the Lean team's own
  literate framework and the default substrate [ACKED].)

## §2 — Layout

- **root `minispec/`** — a lake package (lakefile, `lean-toolchain`, manifest: the
  whole unavoidable noise budget). `minispec/Minispec/` holds the units with their
  sibling looms — propositions and prose, zero tactic noise. `minispec/Minispec/Proofs/`
  holds proof files importing the units and proving their `Prop`s — the LLM
  tactic-churn zone, structurally unable to touch a unit (the statement/proof split is
  idiomatic Lean). `minispec/Generated/` holds the Aeneas-derived definitions:
  committed, `@generated`, regenerated only by the pipeline — committed so that
  regeneration diffs are reviewable drift-alarms. `minispec/CLAUDE.md` carries the
  remit law (§3).
- **runners** — the two central runners (`crates/cli/tests/{e2e,looms}.rs`) gain the
  minispec collection as a walk root WITH its own non-empty discovery floor (the
  `count-drifts` lesson: a broken root discovers zero cases and exits green).
- **`spike/verify/`** — the dorc-verify crate + binary; the Kani harness home
  (harnesses live outside the kernel); the Aeneas pipeline config (nested mise —
  toolchain-shadowing pins nest, additive pins may root [CONDUCTOR ratified]). The
  generic-core / Dorc-consumer split is kept as an INTERNAL seam only; extraction is
  deferred until the tool earns it [TYPED].
- **in-crate residue** (`core`/`analysis`) — only what Rust visibility forces: the
  facade invariant seats and hand-written `#[cfg(kani)] Arbitrary` impls. Nothing else
  of this system lives in product crates.

## §3 — The remit (the spec/tooling separation) [TYPED]

The Lean spec proper is critical, maximum-attention, and expensive in human time; the
harness/tooling around it is ordinary engineering. They are separated structurally:

- **minispec's standing remit is the absolute minimum provable surface**: two or three
  claims of basic, zero-controversy mathematics with no Dorc design content
  (candidates: `JoinCommutative` · `JoinIdempotent` · `LeqReflexive` — STRAWMAN; the
  final pick is a triviality settled at dispatch). Purpose: build the process, praxis,
  gates, and habits on terrain that cannot generate design emergencies.
- **enrichment is a standalone work-item**, human-heavily-in-loop, question-budgeted,
  never a side-effect of any other lane. Its input menu — the graded census of the
  most-settled algebra-dependent rules — is banked in `notes/300`. Multi-writer
  clean-context divergence-mining (independent formalizations of one rule-list, diffed;
  divergence = surfaced underdetermination) is reserved for that item, one-time,
  converging to ONE artifact.
- **question-routing law** — a modeler who hits underdetermination FLAGS it as a
  concrete, strawman-grounded ruling-request and never resolves it: silent resolution
  is unratified design laundered through a proof artifact, the worst outcome this
  process can produce.
- **the research spikes are quarry, never seed** [TYPED] — their gap-lists and
  toolchain accounts are evidence; product units are written clean from the ratified
  English + the derived definitions.

## §4 — dorc-verify: the binder

Organizing principle (the loom's, deliberately): **the authored artifact is the source
of truth; everything else is derived and gate-checked; review is git diff.**

- **the catalogue** [TYPED — metadata dislocated, loom-adjacent] — one generated,
  promote-gated lock: per law, the slug, the cited seat, the resolved citations
  (statement · harness · loom · proof), and the expected badge-set.
  `verify:compile` / `verify:promote` (STRAWMAN task names) mirror the loom flow. The
  gate compares computed badges against the committed expectation and refuses mismatch
  in EITHER direction — no silent demotion (rot caught) and no silent ambition (the
  surface never claims more than was promoted). The promote act IS the ceremony; there
  is no other.
- **badges — an independent SET, not a ladder** [human correction: cross-states are
  real; a scalar would force an unnatural order]. Computed from evidence at gate time,
  never declared:
  - **`elaborated`** (née `stated` — renamed per the act-vs-evidence critique) — the
    statement elaborates against `Generated/`: every name resolves, the types fit; the
    law is speakable in the code's own vocabulary and cannot refer to stale structure.
  - **`interrogated`** — the in-unit instance battery is green AND non-vacuous: at
    least one positive witness (precondition genuinely satisfied, property doing
    non-trivial work — the anti-vacuity probe) plus the standard boundary battery for
    each quantified vocabulary type (empty · singleton · ⊤). Machine-generated where
    possible, hand-added freely, committed in-unit — these worked examples are the
    review surface for non-proof-literate readers.
  - **`pinned`** — the paired Kani harness is green at its declared bounds, resolved by
    name against the real harness list (toolchain-resolved pairing over
    string-matching, per the turn09 `proof_for_contract` steal).
  - **`proved`** — a sorry-free proof of the `Prop` exists in `Proofs/`.
  - **`demonstrated`** — the sibling loom is green under the full battery, PLUS
    non-vacuity certification: **reach** (the demo's execution enters the cited seat —
    small per-demo instrumentation; NB `mise run coverage` is corpus-coverage of shell
    examples, unrelated — this is new, boolean-per-demo, never a percentage metric) and
    **load-bearing** (at least one `cargo-mutants` mutant scoped to the cited seat
    flips the demo's asserted outcome — a recorded kill; the DO-333 ablation move; no
    ablation flags ever enter product code).
  - **`kill-tested`** — the owed mutation badge for statements themselves [TYPED
    gentle-must]: defined from day one, rendering `todo` until built, so the report
    nags structurally and forgetting is impossible. Property-testing is deliberately
    NOT a badge [TYPED] — it guards code, not spec artifacts, and stays in the general
    check-ladder.
  - per-badge **`excepted(reason)`** / **`todo`** — typed non-coverage (duvet's steal):
    deliberate absence rendered with the same mechanical weight as presence.
- **anchors** — v0 is `fn-seat` (the cited chokepoint function), and v0 builds only
  that. **Witnessed-counterfactual** is the named evidence CATEGORY [ACKED] with two
  known mechanisms: mutant-kills (behavioral seats) and `compile_fail` doctests
  (type-seals — committed ablation witnesses: "here is the forbidden program, verified
  rejected on every build"). Further anchor kinds arrive only when an admitted law
  demands one, each with its own non-vacuity story — and per
  `post-checkable-states-are-a-feature`, sometimes the right answer is refining the
  engine until the law HAS a checkable seat.
- **the report** — generated, never authored: per-law badge rows, typed exceptions, and
  the enumerated **verified boundary** — "the subsets of the analysis engine we have
  opted to verify with Lean" as the computed census of cited seats [TYPED — the one
  named, maintained-for-free vocabulary boundary among the kernel/engine/algebra
  layers].
- **gate tiers** — unit-parse / slug / citation-resolution checks are cheap and ride
  the ordinary gate; full evidence recomputation (Lean build, Kani, mutants) runs at
  the fold/bless tier. Principle over placement: evidence is recomputed at whatever
  tier runs it — never a hand-updated cache.
- **external engines, adopted as-is** — Kani (opt-in lane, Linux/WSL); Lean 4 +
  Aeneas/charon (pinned; the nested-mise pattern in-repo); `cargo-mutants`; Verso.
  `duvet` is not adoptable (its document model is RFC-text coverage, not
  theorem/harness binding — ~SUSPECT, deep-read pending) but its typed-citation idiom
  is absorbed above. The NIH surface is the thin binder only.

## §5 — rustdoc coherence (v0-relevant slice)

Doc roles stay distinct, so no duplication: rustdoc doc-comments carry USAGE (and the
already-load-bearing `compile_fail` seals); minispec instances carry LAW WITNESSES; a
covered law's statement lives in exactly one place (its unit), and every other surface
points. The cited seat's doc-comment carries a `Laws:` line naming its laws; the binder
holds the seat↔law mapping, so bidirectional link-checking is a cheap later gate. The
natural second increment (not v0): type-tier laws entering the catalogue with their
`compile_fail` doctests as evidence, unifying the sealed-law species into the same
report.

## §6 — Open items (complete; nothing else is open here)

1. Verso genre feasibility for the unit format + loom-transcript inclusion — builder
   confirms during the build; fallback named in §1.
2. The reach-certification instrumentation choice (llvm-cov-style, scoped per-demo).
3. The `law:` frontmatter key + minispec runner root + discovery floor — lands in one
   commit with the first demo.
4. The final pick of the 2–3 remit claims — human glance at dispatch.
5. The enrichment work-item's charter — separate, human-led, out of this doc's scope.
