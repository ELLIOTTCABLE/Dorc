# 190 — Round-19: the keystone implementation spike (spike-2) charter

> **What this is.** The on-ramp for spike-2: a second, deliberately-disposable implementation spike that
> builds *the analysis the round-16 spike's flat skeleton couldn't do* — correctness/taint-tracing,
> elision/replacement, and mutation-tracking — on the structured entity-algebra (the keystone re-key) those
> require — on top of the forked round-16 workspace (`<root>/spike/`, green). Its job is the same as
> spike-1's: **surface design problems by building**, now aimed squarely at the part that determines whether
> Dorc elides anything on a real book. It folds in the human's round-19 fork-rulings (recorded inline as
> `ch-*`), the code-grounded build reality (`16P`/`16Q`), the named-kind discipline (`17N` + `17O`), and the
> committed substrate decision (`notes/180`+`190`).
>
> AI-generated; confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). **Trust the root
> `DESIGN`/`KNOBS`/`README`/`IMPLEMENTATION`/`AGENTS` over this**, and `16P`/`16Q`/`17N` over the prose here
> where they conflict. The locked decisions are the human's (`ch-*`); everything else is synthesis to argue
> with.

---

## 0. The frame — what spike-2 is, and is not

- **Goal: state-space exploration, not a product.** Build something close to `17N`'s named-kind discipline,
  far enough to generate *working-context for design narrowing*. The deliverable is **what strains and
  where** (the `notes/19*` series), not a shipped analyzer. Pursue correctness — but in order *to see which
  parts make pursuing-correctness hard.* Baking a wrong shape is acceptable and expected (see `ch-wrong`).
- **Find problems; don't rabbit-hole.** Some effort spent attacking a wall explores that wall's state-space
  and is valuable; *giving up and going a different direction to explore that space is equally valid* when a
  wall is hit. Take notes either way. (`16Q ap-1`/`ap-3` are the process spine — §5.)
- **Built on the fork.** `<root>/spike/` is the round-16 workspace, copied out of quarantine and **verified
  green** (cold build 6.2s; the full suite passes green, with one `ignored` test — the deferred 16G HOLE#1, not a regression).
  The keystone-gap demonstrator `plan::fixture_install_runs_despite_converged_probe` passes — the poison wall
  (§3) is intact on the fork. Extend in place; it can be re-quarantined later. *(One-time setup: `mise trust`
  the two relocated configs — see the spike-root `CLAUDE.md`.)*
- **NOT in scope:** the production engine, the orchestrator, multi-host/fleet, the real apply-executor
  (only a thin backward/`bump` skeleton — `ch-scope`), transport/`kCOMMS`, incrementality, and anything
  market-fit / value-prop / corpus-matching (hard to measure, out of scope now — and an explicit
  dead-end to carve out of any adversarial pass). Out-of-scope ≠ untouchable — **stub** any of it where a stub exercises an in-scope goal (§6).

---

## 1. The locked charter decisions (round-19 fork-rulings — human)

- **`ch-scope` — keystone-scoped; executor thinner than everything else.** The spike is the analyzer
  keystone (§3). The executor is extended *only* as far as exercising the keystone needs — concretely:
  instantiate the **backward / `Must` direction that the round-16 adversarial flagged as never-run** (the
  whole `May`/`Must`/`Backward`/`BoundedLattice` tower exists but no backward or must-analysis is
  instantiated — `16P T4`, `16Q q1-backward`), behind a `dorc bump`-equivalent **apply-3 skeleton**
  (`an-apply-3`/`an-backward-slice`). Nothing more — no host mutation over time, no TOCTOU.
- **`ch-shape-anno` — `kTYANNOT-inline`, half-violating `kOOB`.** For *this spike*, pursue the inline
  type-annotation strawman (the one the human called attention to). Accept that it **breaks the off-ramp
  weld** (`17O F-OFFRAMP`, verified live) as known debt — do **not** build the correctness-critical
  strip/transpile pass. The **parser is a disposable test front-end**: it is explicitly allowed to *massage
  input scripts/values* to get them past parsing; accepting arbitrary shell-input is **not** a goal. Don't
  sink time into parser/lexer nightmares (`KNOBS kTYANNOT`).
- **`ch-entity-algebra` — recursive, kind-typed, and the first thing allowed to give.** The entity-algebra
  is a **recursive structure whose fields are typed by the named-kind namespace itself** — a `service`'s
  field can be typed `file` or `user`; kinds embed kinds ("Wombat"-style handle-to-kind), *using types
  established elsewhere, not a bespoke value-type system*. Shape lean (`17N §4` / `17O F-ALGEBRA`):
  present-key = `true`, `!`-pun for false, values = direct types / kind-handles / nested structs, **absent ≠
  asserted-false**. This is **the first thing allowed to give when the going gets hard** — strictly lower
  priority to prove out than analysis-core, correctness-taint-tracing, elision/replacement, and
  mutation-tracking (`ch-priority`).
  **Keystone vs richness — the build-first/give-first split (don't conflate):** the *poison-wall fix* is
  per-entity **selectors** + strong/weak update — a structured key with **no recursion** — and *that* is the
  re-key built first (§3, the corrective to `ap-1`). The **recursive kind-embedding** above is the
  *richness*: it is what gives first when hard, and where `seam-finite`'s finite-height hang-risk lives. So
  "build the keystone first" and "the entity-algebra is first-to-give" are not in tension — they name the
  selector re-key and the recursion, respectively.
- **`ch-wrong` — baking a wrong shape is fine.** We are trying to see *what bakes and what goes poorly*. Do
  not over-invest in getting a high-lock shape "right" on paper first; build it, push it, and record where it
  hurts. (This deliberately inverts `16Q`'s "settle retrofit-hostile decisions on paper first" — *because*
  the spike's purpose is to generate the evidence that settling would otherwise lack.)
- **`ch-priority` — the build-priority order:** (1) analysis-core, (2) correctness/taint-tracing, (3)
  elision & replacement, (4) mutation-tracking, then (5) entity-algebra *richness* (first-to-give). The
  three substrate-seam leading-goals (§4) overlay this — they are *where* (1)–(4) are pressure-tested.

---

## 2. The build reality inherited (code-grounded — `16P` ledger)

What the fork already is, traced in source (not just via the postmortem):

- **The pipeline** `parse → cfg::build → effect::classify → plan::{compile_probe, build_plan}`, a pure
  `Carrier<T>` (value + diagnostics, never-throw) kernel; the generic monotone-dataflow worklist (`solve`
  over `Graph`+`Lattice`+`Direction`) with composable lattices (`Powerset`/`Flat`/`Product`/`MapL`) and the
  `May`/`Must` order-dual + `BoundedLattice` (a must-over-bare-powerset is a *compile error*). `16P T1`–`T6`.
- **The one instantiated analysis:** `effect::classify` = reaching-defs over the oracle effect-map →
  `{MustRun, EstablishAmbient, EstablishWritten}`, gated by entry-reachability + converged-trust;
  `Opaque ⇒ Reach::Top ⇒ poisons all downstream ambient-ness`. The `ReplaceLicense` witness, observable/
  replace, superposition, the leaf-seam, the DST hostsim + `kFAIL-withhold` monitor. `16P T7`–`T16`.
- **The crux (the durable framing):** spike-1 is the **skeleton of `17N` Part-I and none of its
  type-discipline content** — the flat `FactKey{kind, entity}` (one bit per pair), binary `Polarity{Establish,
  Kill}`, no states/typestate (`inc-S`/`inc-7`), no occurrence-typing narrowing (`inc-6`), no cross-oracle
  Seam (Part II), no blessing/`getent`. The richer `core::{Fact, FactDomain}` vocabulary exists but the
  analysis **does not use it** — it keys on the leaner flat pair.
- The whole `Must`/`Backward` tower is **built but never instantiated** (`16P T4`, "generality flag NOT
  BUILT") — that is `ch-scope`'s target.

---

## 3. The keystone + the poison wall (the actual point — `16Q §1`)

+SURE, verified in source on the fork: **nothing elides on a realistic book.** On
`fixtures/pi-webhost.book.sh` the un-oracled `apt-get update` is `Opaque ⇒ Reach::Top`, which poisons the
`apt-get install -y nginx` below it to `EstablishWritten`, so `prove_replaceable` refuses it *even when the
host reports `Converged`* (the passing test `fixture_install_runs_despite_converged_probe`). Every elision
spike-1 demonstrates is on synthetic books with the poisoning neighbor deleted.

The fix is **not** band-aidable (marking `update` "pure" is a lie — it mutates the package index): it is the
**structured/selector entity-algebra** — `update` establishes `package-index#fresh`, `install` establishes
`package:nginx#installed`; different cells ⇒ no poison. That is `an-entity-shape` + `an-strong-weak-update` +
`an-per-entity-selector`, and re-keying `FactKey` propagates through `Reach`, `command_effect`, `classify`,
the oracle effect-map (`Polarity` → a typestate transition, `inc-7`), `ProbeCheck`, and `prove_replaceable`
— i.e. nearly the whole engine. **The selector re-key is the spike's keystone — build it first** (the
corrective to `16Q`'s `ap-1`; §5): the cells that kill the poison wall are per-entity *selectors* +
strong/weak update, which need **no recursion**. The *recursive kind-embedding* richness (`ch-entity-algebra`)
layers on top and is the first-to-give piece (`ch-priority`, `seam-finite`).

Riding on it: occurrence-typing narrowing (`inc-6`, unbuilt — guards refine *state* per-branch), and the
strong/weak-update + uniqueness gate (`an-entity-uniqueness`; the singleton-bit mechanism, `notes/180`
fnd-4).

---

## 4. The substrate decision + the three leading-goal seams (round-19 commit — `notes/180`+`190`)

**DECISION (`dq-substrate` / `an-substrate` / `055` decision-1): keep and extend the hand-rolled
monotone-dataflow worklist.** Not IFDS-the-algorithm; not Soufflé/external-Datalog. `top-level-agent`
committed; the substrate prior-art is graded in `notes/180` (wave-1) + `notes/190` (wave-2, gap-closing),
the round-19 sources `graded-by:subagent` (provisional until main-context re-verify — but the decision does
not hinge on their fine detail).

Why (+SURE on direction):
- precision-equivalent to IFDS for the gen/kill core — substrate is not a precision question (`180` fnd-1);
- the **recursive kind-typed entity-algebra is non-distributive and not a finite flat powerset** — it
  composes from `lattice.rs`'s combinators but fights IFDS's finite-distributive requirement and flat
  Datalog's relational model (`180` fnd-5);
- the **dep-free, pure kernel is welded for DST** — an external C++/Datalog engine breaks it;
- the **perf-inversion** moots the heavy substrates' scale win (`180` fnd-2 / `an-h-sparsity`);
- the worklist is already built + green, and `ch-wrong` wants the costs **surfaced, not pre-paid.**

The contender that genuinely engaged it: **Ascent** `[A-ascent-seamless-deductive-macros-2022]`
`[A-ascent-repo-docs-2026]` is a real, current, *Rust-native* lattice-Datalog (no C++ FFI; lattice domain
composes from `Product`/`Box`/`Set`), weakening two objections. Held off by three: **no provenance/why-trees**
(the `kFIDELITY` lever it was wanted for is absent), a **DST-dependency hazard** (`par`→rayon/dashmap by
default; serial still drags hashbrown/boxcar — against the dep-clean kernel), and the corroboration that
**Datafrog** `[B-datafrog-engine-2026]` (rust-lang's own polonius engine) *is* essentially a worklist. Salsa
`[B-salsa-incremental-framework-2026]` is an incremental-memoization **layer**, not a substrate — out of
spike scope.

**Substrate re-evaluation — the TL-agent's judgment, not a pre-set trigger.** The worklist is committed; if
the seams below strain badly, whether and where to graduate substrates is for the TL-agent to weigh on the
*seam evidence*, not a fixed firing condition. The map for that call: Ascent (Rust-native lattice-Datalog)
would ease hand-rolling a structured lattice but gives **no provenance** — so a heavy why-tree need points the
other way, to Soufflé/Datalog (provenance ~free, at FFI + dependency + DST-hermeticity cost). Let the strain
drive it; don't pre-commit a condition.

**The three leading-goal seams.** The substrate is *deliberately underpowered*, so the spike only *informs*
the substrate decision by being aimed at the seams where the overpowered substrate (IFDS summaries / Datalog
provenance) would have carried us. These are **success-criteria, not passive watch-items** — the deliverable
for each is *where and how badly it strains* (the `re-eval-trigger` evidence), never a green checkmark.
"It works on the easy cases" is exactly the `ap-1`/`ap-2` self-confirmation trap spike-1 fell into.
- **`seam-prov` — provenance / why-trees, hand-built** (`an-queryable-factbase`/`an-locator-dag`). Datalog
  gives these ~free; the worklist hand-builds them. ↔ correctness/taint-tracing (`ch-priority` #2). The
  strongest later case for a relational layer. Watch: does the hand-built derivation-DAG stay tractable as
  taint + the locator-DAG grow?
- **`seam-interproc` — interprocedural summaries, hand-built** (`q1-interproc`/`an-summary-edge`/
  `an-call-return-edges`). IFDS summary-edges amortize a procedure's effect; the worklist ⊤-forces detached
  regions (function bodies / `. /path` source-following / traps — `16P T8`/`DP-8`). ↔ analysis-core
  (`ch-priority` #1). Watch: does the worklist scale to the supergraph, or beg for realizable-path summaries?
- **`seam-finite` — finite-height termination under the *recursive* entity-algebra** (`an-finite-domain`/
  `an-monotonicity`). A non-monotone / infinite-height transfer **hangs** (empirically 435 & 783 CPU-s,
  `16P DP-2`); unbounded kind-nesting threatens finite-height. **Depth-bound the recursion** as a guard, and
  keep `solve`'s convergence-cap. Watch: does kind-embedding stay bounded-height in practice?

**SPA wholesale-read chapters** (`[B-moller-schwartzbach-static-program-analysis-2025]`, for the
orchestrator's grounding read): **4** Lattice Theory and **5** Dataflow Analysis with Monotone Frameworks
(the engine — core); **8** Interprocedural Analysis and **9** Distributive Analysis Frameworks (IFDS/IDE —
*the deferred overpowered alternative + summary mechanism, so the graduate-to signal is recognizable*); **12**
Abstract Interpretation (soundness/optimality/completeness — the `kFAIL` / ⊤-on-unknown frame); plus **6**
Widening (the `seam-finite` termination tool) and **11** Pointer Analysis (Andersen/Steensgaard/flow-sensitive
— the strong/weak-update + uniqueness mechanism behind the keystone). Skippable: 3 Type Analysis (HM —
welded out, `kill-8`), 7 Path-Sensitivity/Relational (`kCONTEXT`-insensitive default), 10 Control-Flow
Analysis (no higher-order, `kill-4`). *(No slicing chapter exists — the `021`/`055` co-mention is imprecise;
the slicing source is Horwitz–Reps–Binkley.)*

---

## 5. The adversarial playbook + the oracle-quality regression class (correctives for `16Q`'s `ap-*`, `17O`)

`16Q` §5b logged four process *anti-patterns* spike-1 fell into (`ap-1..4`, under "Avoid these"; its positive
keepers are `kp-1..4`). The rules below are their **correctives**, baked into the spike, not left to subagent
discretion. (`ap-N` always names 16Q's *anti-pattern*, never the directive — the directive is its corrective.)
- **Corrective to `ap-1` (scaffolding-instead-of-the-keystone): keystone-first.** Build the §3 selector
  re-key **before** any more type-machinery — it invalidates anything built on the old flat `FactKey`, and
  spike-1's documented failure was building the `May`/`Must` scaffolding instead.
- **Corrective to `ap-2` (a harness that validates the wrong invariant, green): executable acceptance.** The
  harness **must execute or `sh -n`-check** the rendered artifact (`an-render-executability-check`), never
  text-diff it — spike-1 shipped non-runnable POSIX (`if true; then #…; fi`) green because the e2e
  golden-diffed text. For a "functioning" goal this is day-1.
- **Corrective to `ap-3` (an adversarial pass finds only what it's aimed at): rotate the target.** Aim
  `/adversarial-crosscheck` passes at the *harness*, *keystone-adherence*, and the *three seams* — not only
  core soundness.

The **oracle-quality regression class** (`17O` R2-SHADOW / R2-ORTRUE / R2-IDCACHE / F-BLESSED /
F-GETENT-HOSTS — human-dispositioned as *not* engine-shape holes): stdlib oracles must be good, battle-tested
sh, kept as regression tests. `command -v X` must defend the shell namespace (resolves to an executable
*file*, not a function/alias); membership via `getent group` field-4, never `id` (cache-stale); per-database
hermeticity (`getent hosts` = live DNS); the lifter must **refuse to treat an errexit-masked rc (`|| true`/
`|| :`) as a verdict**. Terminology: **"blessing" = a stdlib oracle shipped day-1**, not a separate magic
mechanism.

Two `17O` model-findings to encode as tests, not new dimensions:
- **R2-CHANGEDELTA → a `q1-precision` acceptance test.** "Do B because A changed" (config-write → reload-iff-
  changed): the author's `changed=1` flag is a **consumed observable the elision discipline must preserve,
  not synthesize** — so *track the `changed` variable across `cp → reload`* is a concrete test. Dorc must
  never elide a delta-gated effect via a *state*-probe nor synthesize the cross-kind edge; the un-probeable
  change-gated effect class is a `TODO.md`-into-DESIGN item, **not spike scope.**
- **R2-PROBEGATE → the speculate-and-intercept probe model** (the compiled-probe strawman,
  `17x-strawmen/adversarial/compiled-probe.straw.sh`, inlined at `17N §3`): lift read-only probes from the
  CFG, dispatch concurrently, oracles **intercept** (`id__check` ships + replaces `id`); resolve a
  probe-gated branch by *running the read-only probe for real* (unlike Ansible check-mode). The probe is
  compiled from oracle bodies + minimal CFG fragments, **never the book's contents** — so it never inherits
  the book's ambient `trap`s.

---

## 5b. Tension adjudication — the cross-cutting judgment-calls the TL-agent owns

The per-component invariants (`inv-*`) are the "always do X" rules. Their dangerous opposites are the
*tensions* — sites where there is no "always," only a context-bearing judgment, and a wrong orientation is a
*silent* wrong-elision (the note-165 minefield). Most are **cross-cutting**: discharging them needs the phase,
the user-type, and the soundness-orientation — context a single-crate worker lacks. So they are the
**TL-agent's to adjudicate, not a per-component agent's to settle locally**: a component flags such a call
*up*; it never resolves it in isolation. This is `inv-superposition` lifted to the orchestration level — the
component *emits* the fact; the context-bearing caller *collapses* it.

These are type-encoded restrictively wherever the type-system can carry them — `May→Must` is a compile error,
`PhasedVerdict<P>` phase-locks, `ReplaceLicense` is mint-only-by-proof, and a `Grounded<T>`/taint wrapper
*should* mark oracle-claimed-vs-engine-proven (`16P T12` — NOT BUILT). But the type carries the *shape*, never
the *application-judgment* (the compiler stops you collapsing `May→Must`, never tells you whether you *should*
at a given site). So the editors keep it honest by hand, and the residual escalates. The recurring calls
(extensible — flag new ones up):
- `tc-phase` — which phase's `kFAIL` orientation applies (probe-withhold vs apply-perform)? [`inv-kfail`; type: `PhasedVerdict<P>`]
- `tc-collapse` — safe to collapse the `May`/`Must` superposition here, in which orientation? [`inv-superposition`; type: one-way `May→Must`]
- `tc-mint` — may an elision-voucher be minted for this leaf? [`inv-must-may`; type: the `ReplaceLicense` witness]
- `tc-taint` — is the minted container's fact proof-level (engine-derived) or oracle/human-inaccuracy-tainted? [`an-claimed-vs-proven`; type: `Grounded<T>`/`OracleConditional<T>`, unbuilt]
- `tc-user` — which user does this serve (admin/we-infer vs engineer/declares)? [`kBURDEN`; the two-user gradient]
- `tc-uniqueness` — is the entity a provable singleton (license a strong/overwriting update) or not (weak/accumulate)? [`an-entity-uniqueness`/`an-strong-weak-update`]
- `tc-reliability` — does it still hold under an unreliable/lying oracle, not just a well-behaved one? [`an-host-as-adversary`]

The AGENTS.md **exclusion-check** is the standing discipline behind all of these — re-test any decision under
its four axes (reverse-propagation-direction · other-phase · other-user · other-reliability) before excluding a
case; `tc-phase`/`tc-user`/`tc-reliability` are three of those axes as live calls, and the reverse-direction
axis is the backward/apply-3 work (`ch-scope`). *(The `tc-*` set is a synthesis of DESIGN/KNOBS + the human's
round-19 steer; `tc-phase` and `tc-uniqueness` are the AI's completion of the human's "+ one or two I forget"
— sanity-check them.)*

---

## 6. Scope boundaries (in / out)

- **IN:** the structured entity-algebra re-key + per-entity selectors + occurrence-typing narrowing +
  strong/weak update + the uniqueness/singleton gate; correctness/taint provenance, hand-built
  (`seam-prov`); elision/replacement on the new domain; mutation-tracking; a thin backward / apply-3 skeleton
  instantiating the `Must`/`Backward` tower (`ch-scope`); an executable-artifact acceptance harness (`ap-2`).
- **OUT / deferred / wontfix-for-now:** the real apply-executor over time, multi-host/fleet, TOCTOU
  re-probe; transport/`kCOMMS`; incrementality / `dorc try` (the Salsa layer); **execution-context kinds**
  (`sudo`/`ssh`/`docker exec` — `17O R2-CONTEXT`: ssh/docker are eval-class pathological, `become` is future
  first-class work); the `kTYANNOT` off-ramp stripper/transpiler (`ch-shape-anno`); market-fit / value-prop
  / corpus-matching.
- **Stub, don't build.** Any OUT item may be *stubbed* where a stub is needed to exercise an in-scope goal —
  a trivial in-process host-loop to drive the apply-3 skeleton, a no-op transport, a hard-coded probe answer.
  The boundary is *don't build it out properly, don't let it become the work* — not *never touch it*.

---

## 7. What this round feeds / hands forward

The high-lock `O`-cluster decisions (`ANALYZER-NEEDS` §C entity-algebra, §M substrate) stay open — the spike
**informs them by building, it does not settle them on paper** (`ch-wrong`). Specifically it hands forward:
the `dq-entity-algebra` shape as a *strawman stressed to failure*; `dq-kOOB`/`kTYANNOT` still a human ruling
(inline accepted **for this spike only**); the `re-eval-trigger` for the substrate; and `F-FW3` — the
probe-plan-builder is the **only** place `inv-superposition` gets a real second phased caller, so it is the
load-test of "engine emits, caller collapses" (a one-caller fiction until then). The round's product is the
`notes/19*` record of *what strained and where* — the evidence the next design-narrowing runs on.
