# 301 — minispec + dorc-verify: the literate law-corpus and its earned-badge binder

> Tier: LLM-authored spec (Fable conductor, from the 2026-08-14 human design sittings;
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

- **law-spec-touch-frontier-human-only** [TYPED, hard project law] — ONLY
  frontier-class models touch minispec content, and only with explicit human
  authorization. No exceptions; no hot-loop edits, ever. Builders may surface chafe
  against the spec and propose refinements; every change routes through the human.
  This is workable precisely because minispec is an EXTERNAL check, not an acceptance
  gate the builder owns — LLMs are extremely prone to gaming acceptance criteria, and
  an acceptance surface the worker cannot write to cannot be gamed by the worker.
- **law-spec-leads-the-build** [TYPED] — the working order is: plan; decide whether
  the plan touches the spec; if so, modify the spec FIRST (through the authorized
  lane); build toward spec-green. A builder whose build looks right while the
  pre-modified spec disagrees REPORTS and stops; further spec massaging is a very,
  very last resort.
- **post-scope-is-a-judgment** [TYPED] — minispec structurally does NOT aspire to
  define every rule the product is built under, nor to encode engineering practice.
  Out-of-scope is a standing human call, exercised freely. Deliberately outside,
  today: engine hyperproperties (determinism — DST's territory), process/conduct law
  (CLAUDE.md's), foreign-shell semantics (the two-binary floor differential's),
  cross-host/world semantics (DST + the runtime checkers'). The catalogue never
  becomes a registry of everything; no machinery decides scope.
- **post-no-chasing** [TYPED substance] — gate-mode, never chase-mode: an algebra
  semantic change lands as a rule/statement change in the same fold as the code
  (spec-leads, above), badge movements are loud promote-acts (§4), and coverage grows
  only by deliberate increment — there is no backlog to fall behind on. (Rot evidence:
  the turn09 frozen-TLS case died precisely because nothing broke when its referent
  moved.)
- **post-halo-is-the-hazard** — the named failure this system exists to prevent:
  readers (maintainers, implementing LLMs, future reviewers) inferring "there are
  proofs, so it's covered" from the existence of a lean directory. Every mechanism
  below either earns confidence mechanically or renders its absence.
- **post-checkable-states-are-a-feature** [TYPED] — some law-writing praxis will
  involve type-system/runtime refinements to the engine itself so that a law's states
  become checkable at all. That is a feature of the process, not scope-creep; such
  refactors are priced and dispatched like any engine work.

## §1 — The law unit

- **naming** [TYPED] — Lean forbids hyphens in module names, so law-slugs leave
  kebab-case; we lean fully into the ecosystem's DromedaryCase: the law file and the
  catalogue slug are ONE identical string (`UnknownMemberCollides.lean` /
  `UnknownMemberCollides`), and a law's dedicated demonstration loom (§2) carries the
  same stem. Convention dividend: DromedaryCase in prose or in a filename = a hard law
  with machinery behind it; kebab-case = soft corpus reference. The ≥3-full-words slug
  discipline carries over.
- **the unit file is hard-minimal and diff-quiet** [TYPED] — Verso prose carrying the
  English-authoritative law text, the `Prop` statement over the derived definitions,
  and the instance battery. NO metadata in-file (dislocated to the catalogue, §4) and
  NO churny material ever (transcripts re-bless freely; the law surface must stay
  diff-quiet so every change in it is a meaningful, adjudicable event). The design
  intent: an LLM near a law loads the ENTIRE file, prose first, habitually — file
  length is the enemy.
- **byte-budget tripwire** [TYPED] — the binder carries a byte-length (never
  line-length) advisory limit on unit files: exceeding it trips a one-time
  consider-decomposing warning to the editing model, and is trivially overridden —
  readability and sanity trump the limit. The threshold is a builder-calibrated seam
  (§6), not a spec constant.
- **vocabulary extraction, three-way** — proofs always live outside the unit
  (`Proofs/`, §2). Generic mathematical scaffolding (fold lemmas, order-theory
  plumbing) lives outside with NO prose-parallel owed — it is mathematics, not Dorc
  law. Dorc-MEANING-bearing predicates stay in-unit by default; they may move out only
  into the governed shared vocabulary home, where each entry carries its own
  micro-prose and examples and changes are ceremony — a squeezed author quietly
  laundering a law's content into an unreviewed helper import is the failure this rule
  exists to stop. The in-unit instance battery is the standing anti-laundering device:
  concrete witnesses evaluate through whatever the vocabulary actually means, wherever
  it lives.
- **one rendered page** — the Verso doc-build transcludes the law's bound
  demonstration transcripts (§2, cross-directory), so the RENDERED unit is a single
  continuous read: prose, statement, instances, live product demonstration. (~SUSPECT
  on the exact Verso mechanism; the builder confirms; fallback is plain structured
  doc-comments + include-by-generation. Verso is the Lean team's own literate
  framework and the default substrate [ACKED].)

## §2 — Demonstrations: bound product looms

Demonstrations do NOT live in minispec. They are ordinary whole-product looms, sited
where they project-purpose-belong (usually near the user-aid surfaces they pin,
sometimes elsewhere), bound to laws through the catalogue. [ACKED — this siting also
keeps loom churn in builder-space, cleanly outside `law-spec-touch-frontier-human-only`.]

- **the habit: one authoritative-ish loom per law** [TYPED, carried as habit, never
  tooling-enforced] — the default act is to WRITE a dedicated `<LawSlug>.loom`
  minimally targeting the law, in the project-purpose-appropriate dir, as its
  authoritative example — unless a nearly-perfect loom already exists, or a law
  genuinely benefits from several. Never shoehorn a loom with a different primary
  purpose into minimally/mutation-testedly representing a law. The tooling itself
  stays many-to-many: several looms may exercise one law in the course of their
  primary interface-pinning duty, and one loom may bear on several laws.
- **the binding is a ratchet, and it carries assertions** — a binding is never a bare
  pointer: the catalogue row records the law-relevant ASSERTION SUBSET of the loom —
  the specific site-outcomes the law needs (per-site `{elide, guard, run, survive}`),
  the reach-of-seat requirement, and the recorded kill (§4 `demonstrated`) — and the
  binder re-verifies that subset at gate, independent of the loom's own goldens. So
  unrelated loom churn (render text, other lines) never touches the law, while a
  re-bless or deletion that breaks a bound subset trips the BINDER, whose badge
  demotion routes through promote ceremony. Demo assertions sit at the OUTCOME tier;
  rendered transcripts stay illustrative and re-bless freely (`render-form-unwelded`).
- **the loom-side key** — a bound loom declares its duty in frontmatter with a
  deliberately alarming name: `tests-critical-law: UnknownMemberCollides` (STRAWMAN
  spelling; kebab key style per the existing vocabulary; joins `FRONTMATTER_KEYS` in
  the same commit that mints it, per that vocabulary's own rule). The name exists so a
  builder editing the loom knows FROM THE KEY that they are touching law-evidence. The
  key is a PROPOSAL; only a catalogue promote (spec-side, human-authorized) accepts a
  binding into evidence — the two-way agreement is binder-checked.
- **user-pain closure** [ACKED] — a bound loom may declare `expect-diagnostic:`
  needles (structural, catalog-validated, asserted-to-fire), closing the chain
  law → seat → demonstration → the exact user-facing error, end to end, with existing
  machinery.
- **execution** — bound looms run under the unchanged central runners and gate battery
  (the sole-sanctioned-executor law) in their existing collections; minispec itself
  contains no cases and needs no runner root. Later affordance, not v0: the reach
  instrumentation doubles as binding DISCOVERY — a loom whose execution enters a law's
  cited seat is a suggested candidate.

## §3 — Layout

- **root `minispec/`** — a lake package (lakefile, `lean-toolchain`, manifest: the
  whole unavoidable noise budget). `minispec/Minispec/` holds the units — propositions
  and prose, zero tactic noise. `minispec/Minispec/Proofs/` holds proof files
  importing the units and proving their `Prop`s — the LLM tactic-churn zone,
  structurally unable to touch a unit (the statement/proof split is idiomatic Lean).
  `minispec/Generated/` holds the Aeneas-derived definitions: committed, `@generated`,
  regenerated only by the pipeline — committed so that regeneration diffs are
  reviewable drift-alarms. `minispec/CLAUDE.md` carries the remit (§4) and the two
  §0 access laws.
- **`spike/verify/`** — the dorc-verify crate + binary; the Kani harness home
  (harnesses live outside the kernel); the Aeneas pipeline config (nested mise —
  toolchain-shadowing pins nest, additive pins may root [CONDUCTOR ratified]). The
  generic-core / Dorc-consumer split is kept as an INTERNAL seam only; extraction is
  deferred until the tool earns it [TYPED].
- **crate-local homing is the default** [TYPED] — components of this system live in
  the crates they belong to, like anything else: the facade invariant seats and the
  hand-written `#[cfg(kani)] Arbitrary` impls sit in `core`/`analysis` beside the
  types they serve, and future law-adjacent machinery homes locally by preference.
  It is moving something OUT to a dislocated location that requires justification —
  the dislocations above (the catalogue, `Generated/`, the harness home) each carry
  theirs (derived-not-authored; machine-produced; visibility across crates).

## §4 — The remit (the spec/tooling separation) [TYPED]

The Lean spec proper is critical, maximum-attention, and expensive in human time; the
harness/tooling around it is ordinary engineering. They are separated structurally
(§0's access laws are the enforcement):

- **minispec's standing remit is the absolute minimum provable surface**: two or three
  claims of basic, zero-controversy mathematics with no Dorc design content
  (candidates: `JoinCommutative` · `JoinIdempotent` · `LeqReflexive` — STRAWMAN; the
  final pick is a triviality settled at dispatch). Purpose: build the process, praxis,
  gates, and habits on terrain that cannot generate design emergencies.
- **enrichment is a standalone work-item**, human-heavily-in-loop, question-budgeted,
  never a side-effect of any other lane. Its input menu — the graded census of the
  most-settled algebra-dependent rules — is banked in `notes/300`. Multi-writer
  clean-context divergence-mining (independent formalizations of one rule-list,
  diffed; divergence = surfaced underdetermination) is reserved for that item,
  one-time, converging to ONE artifact.
- **question-routing law** — a modeler who hits underdetermination FLAGS it as a
  concrete, strawman-grounded ruling-request and never resolves it: silent resolution
  is unratified design laundered through a proof artifact, the worst outcome this
  process can produce.
- **the research spikes are quarry, never seed** [TYPED] — their gap-lists and
  toolchain accounts are evidence; product units are written clean from the ratified
  English + the derived definitions.

## §5 — dorc-verify: the binder

Organizing principle (the loom's, deliberately): **the authored artifact is the source
of truth; everything else is derived and gate-checked; review is git diff.**

- **the catalogue** [TYPED — metadata dislocated, loom-adjacent] — one generated,
  promote-gated lock: per law, the slug, the cited seat, the resolved citations
  (statement · harness · proof · bindings with their assertion subsets), and the
  expected badge-set. `verify:compile` / `verify:promote` (STRAWMAN task names) mirror
  the loom flow. The gate compares computed badges against the committed expectation
  and refuses mismatch in EITHER direction — no silent demotion (rot caught) and no
  silent ambition (the surface never claims more than was promoted). The promote act
  IS the ceremony; there is no other. Promotes are spec-side acts under
  `law-spec-touch-frontier-human-only`.
- **badges — an independent SET, not a ladder** (cross-states are real; a scalar would
  force an unnatural order). Computed from evidence at gate time, never declared:
  - **`elaborated`** — the statement elaborates against `Generated/`: every name
    resolves, the types fit; the law is speakable in the code's own vocabulary and
    cannot refer to stale structure.
  - **`interrogated`** — the in-unit instance battery is green AND non-vacuous: at
    least one positive witness (precondition genuinely satisfied, property doing
    non-trivial work — the anti-vacuity probe) plus the standard boundary battery for
    each quantified vocabulary type (empty · singleton · ⊤). Machine-generated where
    possible, hand-added freely, committed in-unit — these worked examples are the
    review surface for non-proof-literate readers.
  - **`pinned`** — the paired Kani harness is green at its declared bounds, resolved
    by name against the real harness list (toolchain-resolved pairing over
    string-matching, per the turn09 `proof_for_contract` steal).
  - **`proved`** — a sorry-free proof of the `Prop` exists in `Proofs/`.
  - **`demonstrated`** — at least one tracked binding is green under the full battery,
    PLUS non-vacuity certification: **reach** (the bound loom's execution enters the
    cited seat — small per-demo instrumentation; NB `mise run coverage` is
    corpus-coverage of shell examples, unrelated — this is new, boolean-per-binding,
    never a percentage metric) and **load-bearing** (at least one `cargo-mutants`
    mutant scoped to the cited seat flips a bound assertion subset — a recorded kill;
    the DO-333 ablation move; no ablation flags ever enter product code).
  - **`kill-tested`** — the owed mutation badge for statements themselves [TYPED
    gentle-must]: defined from day one, rendering `todo` until built, so the report
    nags structurally and forgetting is impossible. Property-testing is deliberately
    NOT a badge [TYPED] — it guards code, not spec artifacts, and stays in the general
    check-ladder.
  - per-badge **`excepted(reason)`** / **`todo`** — typed non-coverage (duvet's
    steal): deliberate absence rendered with the same mechanical weight as presence.
- **anchors** — v0 is `fn-seat` (the cited chokepoint function), and v0 builds only
  that. **Witnessed-counterfactual** is the named evidence CATEGORY [ACKED] with two
  known mechanisms: mutant-kills (behavioral seats) and `compile_fail` doctests
  (type-seals — committed ablation witnesses: "here is the forbidden program, verified
  rejected on every build"). Further anchor kinds arrive only when an admitted law
  demands one, each with its own non-vacuity story — and per
  `post-checkable-states-are-a-feature`, sometimes the right answer is refining the
  engine until the law HAS a checkable seat.
- **the report** — generated, never authored: per-law badge rows, typed exceptions,
  and the enumerated **verified boundary** — "the subsets of the analysis engine we
  have opted to verify with Lean" as the computed census of cited seats [TYPED — the
  one named, maintained-for-free vocabulary boundary among the kernel/engine/algebra
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

## §6 — rustdoc coherence (v0-relevant slice)

Doc roles stay distinct, so no duplication: rustdoc doc-comments carry USAGE (and the
already-load-bearing `compile_fail` seals); minispec instances carry LAW WITNESSES; a
covered law's statement lives in exactly one place (its unit), and every other surface
points. The cited seat's doc-comment carries a `Laws:` line naming its laws; the
binder holds the seat↔law mapping, so bidirectional link-checking is a cheap later
gate. The natural second increment (not v0): type-tier laws entering the catalogue
with their `compile_fail` doctests as evidence, unifying the sealed-law species into
the same report.

## §7 — Open items (complete; nothing else is open here)

1. Verso genre feasibility for the unit format + cross-directory loom-transcript
   transclusion — builder confirms during the build; fallback named in §1.
2. The reach-certification instrumentation choice (llvm-cov-style, scoped
   per-binding).
3. The `tests-critical-law:` frontmatter key spelling + its `FRONTMATTER_KEYS` entry —
   lands in one commit with the first binding.
4. The byte-budget tripwire threshold — builder-calibrated against the first real
   units.
5. The final pick of the 2–3 remit claims — human glance at dispatch.
6. The enrichment work-item's charter — separate, human-led, out of this doc's scope.
