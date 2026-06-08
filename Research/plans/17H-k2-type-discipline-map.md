# 17H — K2 map: the minimal type-discipline (round 17 deliverable)

> **What this is.** The K2 deliverable: a **map-with-a-lean** over the type-discipline design space —
> include/kill boundary, per-mechanism cited prior-art for *each* inclusion and *each* kill. **Not a
> decision:** the human takes the lean into adversarial-crosscheck, then a synthesis reunites K2 with K1.
> `dq-entity-algebra` is left **bounded, not settled** (per charter `170`). Gather notes: `17C`–`17G`
> (f6–f10); reasoning `17B`; plan `17A`. AI-generated; trust repo-root `DESIGN`/`KNOBS`/`README`/
> `IMPLEMENTATION` over this.
>
> **Terminology.** "engine" = the analyzer/compiler (probe-time), per house usage. Where I say *Dorc's own
> machinery* / *the tool* I mean the whole automatic layer (analyzer at probe-time **+** orchestrator at
> apply-time) **as opposed to the oracle-author (a human)**. The load-bearing distinction throughout is
> *tool-automatic* vs *oracle-declared*, not probe vs apply.

> **[REVISED→17N · UNSETTLED · 2026-06-08]** The K1+K2 reunion this map forecasts (§G, "synthesis reunites with K1") **landed as `plans/17N`** — current where it differs. Two recasts there: **kill-1** (success-type inference) is no longer a flat kill but the **may/must certainty mechanic** (corpus-mining = may/linter-hint; coherence over the single program-unit being compiled = can-be-must); **kill-4** keeps the no-higher-order-effects floor but drops the "flat 2-element effect-map" gloss — effects are flow-sensitive typestate (inc-S/inc-6).

## A · The governing criterion is TWO-AXIS (read first; `17B`)
"Disprove/avoid/kill" is the round's orientation, but kill-safety is **not uniform** — it splits:
- **axis-depth** — *how hard the analysis thinks* (interproc, context, precision-mechanism). Killing ⇒
  less certainty ⇒ unknown ⇒ run ⇒ drains to over-run (`IMPLEMENTATION` priorities 2–3, never priority-1).
  **Kill-floor holds; judge on value.** Most kills below live here.
- **axis-fidelity / coordination** — *what states exist & how finely distinguished; when one oracle's
  state-test discharges another's state-dependency*. Killing here = **coarsening**, which reads as
  confident-wrong sameness ⇒ over-correlation ⇒ **under-execute (priority-1, worst)**. Failure is
  *non-directional*; complexity is the wrong axis. **Kill-floor does NOT hold; judge on the fidelity-floor**
  (the model must distinguish ≥ the world's real *mutation-gating* states — `≥enum`, never boolean) **+ the
  rule in §B-1.**

## B · The two spines (the load-bearing emergent results)
- **spine-1 · runtime enforcement is the thing Dorc omits ⇒ the safety-direction is the ORACLE's, not the
  tool's.** Forgiving type systems that *bake* a safety-direction can do so only because they have a
  **uniform, free runtime backstop**: Dialyzer "never cries wolf" because Erlang is runtime-type-safe
  [A-lindahl-sagonas-success-typings-2006]; gradual typing's *add*-direction safety is enforced by runtime
  casts, and that same cast machinery is what Takikawa's performance "death" kills
  [A-siek-refined-criteria-gradual-typing-2015] + [A-takikawa-sound-gradual-typing-dead-2016]. Dorc has **no
  uniform free backstop** (re-run = `over-execute`, priority-2, non-uniform) and Bracha's argument says
  *don't let any layer depend on the type system's correctness* anyway [B-bracha-pluggable-types-2004]
  (= tenet-0). ⇒ **Dorc's own machinery must not pick a correlation's safety-direction; the oracle declares
  it** (spelled in-band sh = `kOOB`; naming is just sugar and can't carry a 3-place relation alone — `151`
  X3). Absence of declaration ⇒ no correlation ⇒ ⊤ ⇒ run (the floor — *not* the tool choosing a direction).
- **spine-2 · cross-referent MEANING-agreement is the un-dodgeable core.** Four hazards gathered
  separately are **one**: coherence — instances of a class must agree
  [A-wadler-blott-ad-hoc-polymorphism-1989] ≡ polymorphic-variant *same-tag-different-meaning*
  [B-garrigue-polymorphic-variants-1998] ≡ DefinitelyTyped's *shared-name/divergent-meaning*
  [B-definitelytyped-governance-2020] ≡ the *over-correlation* that causes under-execute (`17B`). All say:
  *agreement on what a kind's name/state MEANS* is load-bearing, un-dodgeable, and **not** covered by
  ⊤-on-conflict (silent same-name-different-meaning never conflicts). This is precisely the K1/K2 seam.

## C · The map (include / kill / posture — per mechanism, cited)
`[axis]`; prior-art in brackets; one-line why. Lean = INCLUDE/KILL as marked.

### INCLUDE
- **inc-1 success-typing polarity** `[posture]` [A-lindahl-sagonas-success-typings-2006] — over-approximate
  the success set; report only *definite* clashes ("never cry wolf"); unknown ⇒ ⊤; never reject = Dorc's
  optimizer-not-checker stance.
- **inc-2 the gradual guarantee, as the no-cliff law** `[law/test]`
  [A-siek-refined-criteria-gradual-typing-2015] — adding/removing precision must not break a working
  program. **Asymmetric for Dorc:** the *remove* half is a real guarantee (drop an oracle ⇒ floor ⇒ still
  runs = off-ramp); the *add* half (wrong oracle caught loudly) is **best-effort only** (cast-free ⇒
  uncaught = the T12 hazard). Usable as a differential test.
- **inc-3 pluggable / no-runtime-effect** `[contract]` [B-bracha-pluggable-types-2004] — an optional type
  system has *no effect on runtime semantics* = the oracle-is-a-behavioral-no-op contract; also the
  prior-art for *optional, lint-like* runtime checks (`17D` correction).
- **inc-4 soft-typing's static/dynamic split** `[division]` [B-bracha-pluggable-types-2004] (+ `17C` §2.2)
  — can't-prove-safe ⇒ insert a runtime check, don't reject = Dorc's ⊤-run + probe.
- **inc-5 the stub-corpus governance model** `[governance]` [B-definitelytyped-governance-2020] +
  [B-pep-561-distributing-type-info-2017] — decoupled community stubs, **machine-enforced not
  author-trusted** (CI checks stub-matches-source), **tiered by impact** (= effort-allocation), with a
  **declared precedence order** over overlapping sources. Real-world proof the back-prop loop scales.
- **inc-6 occurrence-typing guard-lifting** `[spine]` [A-tobin-hochstadt-logical-types-2010] — a guard's
  test refines state per-branch; the carried-in narrowing spine.
- **inc-7 the minimal effect-map ≡ a typestate transition-table** `[mechanism]`
  [A-lucassen-gifford-effect-systems-popl-1988] + [B-aldrich-typestate-oriented-2009] — `(provider,verb) →
  {establish,kill}` *is* "this verb transitions the kind's state"; one mechanism, two readings.
- **inc-8 nominal kinds** `[identity]` (A1; contrast [B-typescript-design-goals-2020], which is structural)
  — a kind is a declared name, not inferred-from-structure (structure-inference = grounding = K1).
- **inc-9 coherence, realized as a CONTRACT** `[fidelity]` [A-wadler-blott-ad-hoc-polymorphism-1989] —
  kind = class, oracle = instance; instances must *agree* for meaning to be well-defined. The one candidate
  un-dodgeable rule (spine-2). Dorc can't *enforce* it (never rejects) ⇒ a kind-owner-declared contract +
  best-effort CI lint (tenet-0), not a checked property.
- **inc-S the typestate STATE-MODEL** `[fidelity]` [B-aldrich-typestate-oriented-2009] — states +
  transitions + state-specific data; this is the *kept* half of typestate (≥enum; = `dq-entity-algebra`'s
  structured pole). The probe = typestate's own *dynamic-state-test* fallback for when uniqueness can't be
  proven (prior-art-validated, not a Dorc hack).
- **inc-10 ML open/closed-variant FRAMING** `[framing]` [B-garrigue-polymorphic-variants-1998] — open(`>`)
  vs closed(`<`) = an open vs closed kind-state-enum (may a provider add a state?); width-subtyping =
  handle-a-subset-of-states. Take the framing; **kill the apparatus** (see kill-8). Tag-reuse footgun = spine-2.

### KILL (degrades to the floor / welded out)
- **kill-1 success-type *inference engine*** `[depth]` [A-lindahl-sagonas-success-typings-2006] — Dorc
  *declares* anchors; corpus-mining is MAY-grade/offline (`096`), never a per-run skip license.
- **kill-2 consistency-relation + cast/blame machinery** `[depth]`
  [A-siek-refined-criteria-gradual-typing-2015] — keep the static `⋆`-consistent-with-all idea (= ⊤); kill
  the cast-insertion + blame calculus (we never check or cast).
- **kill-3 the sound-gradual *performance death*** `[depth]` [A-takikawa-sound-gradual-typing-dead-2016] —
  the death is the runtime cost of *mandatory* boundary checks; Dorc is cast-free and enforces nothing at
  runtime, so it's absent *by design* (opposite choice, not mere N/A).
- **kill-4 Koka row-polymorphic effect apparatus** `[depth]` (framing-only; pointers-file lead) — heavier
  than a 2-element `{establish,kill}` map needs; mine the framing, drop the row machinery.
- **kill-5 typestate *enforcement*** `[depth]` [B-aldrich-typestate-oriented-2009] — rejecting operations
  invalid in the current state is the part Dorc drops (never rejects). Its precondition (aliasing/uniqueness
  control) is undecidable = SF-1 (fw-2) anyway.
- **kill-6 structural typing (for kind-identity)** `[fidelity-via-identity]`
  [A-siek-refined-criteria-gradual-typing-2015] §5.5 + [B-typescript-design-goals-2020] — structural
  type-*tests* break the gradual guarantee; structure-inference = grounding (K1). Stay nominal (inc-8).
- **kill-7 *unbounded* subtyping lattices** `[depth]` ([A-lindahl-sagonas-success-typings-2006], the f6-6
  nuance) — kill unbounded subtyping (`kCONTEXT` flat-domain), but **keep** the bounded any/none +
  finite-union-widened lattice — that's how the ≥enum states sit. (Not a blanket kill.)
- **kill-8 HM / full inference** `[welded]` [B-bracha-pluggable-types-2004] (no polymorphic recursion) +
  [A-lindahl-sagonas-success-typings-2006] (forces code rewrites) — welded out (`kVERIFY`); restrictive +
  rewrite-hostile (breaks the off-ramp).
- **kill-9 dependent types / Cousot-Galois full soundness** `[welded]`
  [B-livshits-soundiness-manifesto-2015] — welded out; no realistic whole-program analysis achieves full
  soundness; pursuing it destroys precision/scalability.

### POSTURE (frames the lean)
- **pos-1 soundiness** [B-livshits-soundiness-manifesto-2015] — sound core + deliberately-and-*documentedly*
  unsound on the hard subset = Dorc's exact stance ("soundy is the new sound").
- **pos-2 TypeScript's unsound-by-design non-goals** [B-typescript-design-goals-2020] — erasable +
  unsound-by-design + productivity-over-correctness; the industrial precedent (diverge: TS structural,
  Dorc nominal).

## D · `dq-entity-algebra` — bounded, with a floor (the call left for the human)
- **Floor (settled by the world, not us):** `≥enum` — a flat list of N states per entity (boolean is the
  degenerate default). Real entities gate mutations on >2 states (a systemd unit: installed / enabled /
  active). Below the floor ⇒ over-correlation ⇒ under-execute (§A axis-fidelity).
- **Open (bounded, NOT settled):** flat (`package:nginx ∈ {…}`) vs structured
  (`package:nginx{installed,version,held}`). Selectors push structured
  [A-tobin-hochstadt-logical-types-2010]; typestate states carry state-specific data, also structured
  [B-aldrich-typestate-oriented-2009]; but structure multiplies facts and hits `kCONTEXT`/`kFACTS`
  (high-lock). I map both poles; I don't pick.
- **The strong-update dependency (→ K1):** whether a state-change may *overwrite* (strong update) vs
  *accumulate* (weak) is gated by entity-**uniqueness** — SF-1, undecidable, Dorc holds only opaque tokens.
  Bounded here; the identity half is K1's.

## E · The coordination contract (#7, mapped — the decision is the human's)
When one multi-state entity is touched by several oracles, *who declares what?* Four author-obligation
options, by coordination cost:
- **opt-1 monolithic** — one oracle owns the kind + all states. Zero coordination; doesn't compose.
- **opt-2 RAL-enum** — a kind-owner declares the state-vocabulary; providers slot in per `(state,provider)`
  (the Puppet model, `092` f17). Composes; risk = shared-name/divergent-meaning (spine-2).
- **opt-3 per-facet + cross-facet invalidation** — separate oracles per facet; one declares "writing facet
  X invalidates facet Y". Most expressive, most coordination.
- **opt-4 precedence/resolution order** — a declared precedence over overlapping sources
  ([B-pep-561-distributing-type-info-2017]: manual > user > stub-package > inline > typeshed). Resolves
  *which oracle wins* when several describe one kind.
- **Unifying rule (the lean): MUST-grade-to-correlate.** Only an oracle's explicit, structurally-anchored
  declaration creates a correlation *and* supplies its safe-fail-direction (spine-1); absence/ambiguity ⇒
  no-match ⇒ run. Coherence (inc-9) is the contract that makes opt-2/opt-4 sound. **Open (oq-2):** which
  option is the lean, or is it per-kind (a `mode`)? — informed by inc-5's real-world evidence; not decided.

## F · Firewall hand-offs for the synthesis (K1/K2 seam)
- **fw-1 strong-update licensing** = entity-uniqueness (SF-1) — K1's identity work; §D bounds the *shape*.
- **fw-2 typestate linearity** = the *same* uniqueness (kill-5; [B-aldrich-typestate-oriented-2009]) — flag
  that typestate-enforcement and the strong-update keystone are one problem.
- **fw-3 verdict channel / 3-valued verdict across phases** (SF-3) — borders the map; the channel design is
  apply-phase/synthesis, but the type-discipline must not bake a phase default (superposition).
- **fw-4 (the big one) meaning-agreement** (spine-2 / f10-6) — coherence ≡ over-correlation is exactly
  where K2's coordination semantics meets K1's *grounding of what a name/state means*. The synthesis must
  treat these as one object.

## G · The lean, in one paragraph + what stays open
Build the **dumbest forgiving discipline**: nominal kinds carrying a `≥enum` state-model (a typestate
state-machine read as an effect-map), narrowed by occurrence-typing guards, with everything unanchored ⇒ ⊤
⇒ run (soundiness). Take the *postures and properties* of the forgiving lineage (success-typing polarity;
the gradual guarantee as a no-cliff *test*; pluggable no-runtime-effect; soft-typing's static/dynamic
split; the stub-corpus governance model) and **kill all the runtime-enforcement and heavy-inference
machinery** (casts/blame, success-type inference, Koka rows, typestate enforcement, HM, dependent types) —
because Dorc omits runtime enforcement, so the safety-direction must come from the **oracle, not the
tool** (spine-1), and the only genuinely un-dodgeable obligation is **cross-oracle meaning-agreement**
(coherence-as-contract, spine-2). **Open for the human:** oq-1 (is MUST-grade-to-correlate provable, or
left for the design-spike?), oq-2 (which coordination option / is it a `mode`?), and `dq-entity-algebra`
flat-vs-structured. **Stop here** — the round ends at the map; adversarial-crosscheck is the human's, then
synthesis reunites with K1 (the named kind is one object: K1's identity-anchor + K2's nominal type).
