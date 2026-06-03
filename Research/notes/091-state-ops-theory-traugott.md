# 091 — F3 (ops-native state theory): Traugott, *Why Order Matters* (round 9, 2026-06-02)

> Durable per-front notes for the state-tracking round (plan: `plans/090-...`).
> Front F3, source 1 of 2 (Traugott done; a Burgess *promise-theory/convergence*
> primary is queued but low-marginal — Traugott gives a rich critical account of
> convergence from Dorc's own camp, and DESIGN's "not convergence" link already
> pins the contrast). Per §0.5 of the plan: findings tagged **knob/contract ×
> weld/adjusts** and bound to **q-floor/frontier/ceiling**; sources judged
> load-bearing / vocabulary-only / false-friend. **Deliberately did NOT read §8**
> (the formal Turing-equivalence proof) — that is the "proving" minefield the human
> flagged; the load-bearing content is conceptual (§1–6). Read in main context.

**Verdict on the source: LOAD-BEARING (highest-value external hit so far).** Ops-native,
prose, proof-light; it states Dorc's exact problem and hands Dorc its *coordinates*.

## Findings (lifted)

- **f1 — Dorc's coordinates, located (the "Weird Thing").** Traugott's trichotomy is
  **divergent / convergent / congruent**. Dorc targets *congruent outcomes* (ordered,
  predictable, single-correct-execution, "fire-and-forget") but **refuses the congruent
  precondition** (a *fully-descriptive baseline* + lifetime change-journal = exactly
  `kDEPS-declare-world`, which DESIGN rejects). It *also* refuses the *convergent method*
  (re-sample-and-react to a fixed point, order-avoiding, incomplete baseline). **No tool
  in Traugott's survey occupies Dorc's cell**: congruence-grade results from
  convergence-grade (partial, lazy) description, bridged by *static analysis + probing*
  substituted for the *journal*. +SURE this is the sharpest available statement of what
  makes Dorc unusual. [positioning; q-ceiling = congruent-grade single-execution]
- **f2 — convergence is a *different axis*, not Dorc's floor (refines the earlier note).**
  The human's "convergence is one end of a spectrum, I'm not sure" → Traugott separates
  *method* (divergent/convergent/congruent) from *outcome-quality*. Dorc's floor ("plain
  ordered shell run") is **congruent-degenerate** (ordered, single-pass, just un-skipped),
  *not* convergent (re-run-til-fixpoint, order-avoiding). So Dorc's real spectrum is
  *how much of the congruent baseline we infer-vs-require* (= floor↔ceiling), and
  **convergence is a *fallback method* Dorc may borrow** (DESIGN's reserved "trivial
  convergence" re-run) **when ordering/analysis fails — not the floor it rests on.**
  ~SUSPECT (my interpretation layered on his trichotomy).
- **f3 — the unsolvability spine is ops-native and explicit (undecidable).** §4.2 states
  future-change dependency is *undecidable* because the future change-set is unknown.
  Strongest external validation yet of the human's "fundamentally unsolvable" premise —
  and from *ops*, 2002, not PLT. Directly justifies **q-floor/q-frontier**: you cannot
  fully infer the frame, so the user must declare *or* we conservatively retain/run.
- **f4 — state is describable, behavior is not → the inference ceiling.** §4.3: "Congruence
  is defined in terms of disk state rather than behavior, because disk state can be fully
  described, while behavior cannot." Dorc reasons over *state facts* (describable), never
  *behavior*. **"Does `do_y` need `x`?" is a behavior/intent question → not state-recoverable
  → this is *precisely* what the q-floor contract must carry.** +SURE this cleanly explains
  *why* a mandatory-declaration floor exists at all. [wall · q-ceiling] → [contract · q-floor]
- **f5 — the canonicalization hazard refines `kVOLATILES`.** §4.2 (the `/etc/inetd.conf`
  comments example): deciding which bit-differences are "functional" is interpretation-
  dependent *and time-sensitive* — "currently deemed not functional … may still affect the
  viability of future change directives." So Dorc's "strip volatile/non-functional state for
  the skip-cache" is **not a safe inference**; it is a *contract* (oracle-author declares
  what's canonicalizable) carrying a documented across-time failure mode. The `kVOLATILES`
  weld (exclude non-determinism) holds, but *what counts as volatile* is a per-fact contract
  with a staleness caveat. [contract · user-adjusts · refines `kVOLATILES`]
- **f6 — circular dependency / "think like a kernel developer" → static analysis is
  necessarily insufficient.** §5: an admin tool "executes in the context of the target
  operating system; changes can affect the behavior of the tool itself." You cannot analyze
  a command as standalone; it *perturbs the substrate it runs on*. → validates (a) the probe
  must be hermetic/sanitized to break the loop during the read phase (`kFAIL-withhold`), and
  (b) pure static analysis can't suffice — **you must probe** (couples to `kDEPS-accept-partial`:
  static-derive + runtime-trace are complementary). [wall]
- **f7 — ordering is the keystone → state-closure must be order-aware.** The whole paper's
  thesis: deterministic, repeatable *order* is the keystone. Validates **D1's *concept***
  (sequential, not commutative, effects) as load-bearing — *even though we skip the quantale
  formalism*. The state-flow analysis is inherently ordered; `do_x; do_y ≠ do_y; do_x`.
- **f8 — safe-default floor: independent arrival.** §2: tools "need to have some reasonable
  default behavior that is safe if the user lacks this theoretical knowledge"; ISconf
  "implemented ordered change by default." = Dorc's fail-happy floor (ordered plain-shell
  run, safe regardless of declaration). [contract · weld · q-floor]
- **f9 — testing is irreducible (validates `kVERIFY-calibrate`).** §2/§4.2: "No tool or
  language can remove this need [for testing], because no testing is capable of validating a
  change in any conditions other than those tested"; "Unless we can prove congruence, we
  cannot validate … without thorough testing." Ops-native confirmation that proof is
  unavailable and testing/calibration is the substitute. [welded — confirms `kVERIFY`]

## Quadrant population (Traugott's contribution to the §0.5 map)

- **[contract · weld · q-floor]** *Order-preserving execution by default* is the safe floor:
  Dorc must keep source order unless it can *prove* a reorder/skip safe. (f8)
- **[wall · q-ceiling]** *Behavior is undecidable from state* (f4): the inference ceiling is
  "state facts only"; intent/behavior is never inferable, only declarable-or-run.
- **[contract · q-floor]** The *must-declare floor* = exactly the behavior/intent facts that
  state-inspection cannot recover (canonical case: "`do_y` depends on `x`" when `x` is
  transient and no oracle covers `do_y`). (f3, f4)
- **[contract · user-adjusts · refines `kVOLATILES`]** *What counts as canonicalizable-volatile*
  is a per-fact contract with a time-sensitivity hazard. (f5)
- **[method-backstop, not a knob]** *Convergence (idempotent re-run)* = a fallback method Dorc
  borrows only when ordering/analysis fails; couples to DESIGN's reserved "trivial
  convergence". (f2)

## Citations
> [A-traugott-order-matters-2002]:§abstract (relevance: +1:SURE)
> "no tool, written in any language, can predictably administer an enterprise infrastructure
> without maintaining a deterministic, repeatable order of changes on each host. The runtime
> environment for any tool always executes in the context of the target operating system;
> changes can affect the behavior of the tool itself, creating circular dependencies."
> "self-administered hosts execute self-modifying code. They do not behave according to simple
> state machine rules, but can incorporate complex feedback loops and evolutionary recursion."

> [A-traugott-order-matters-2002]:§4 (relevance: +1:SURE)
> "All computer systems management methods can be classified into one of three categories:
> divergent, convergent, and congruent."

> [A-traugott-order-matters-2002]:§4.2 (relevance: +1:SURE)
> "The baseline description in a converging infrastructure is characteristically an incomplete
> description of machine state."
> "Because convergence typically includes an intentional process of managing a specific subset
> of files, there will always be unmanaged files on each host. Whether current differences
> between unmanaged files will have an impact on future changes is undecidable, because at any
> point in time we do not know the entire set of future changes, or what files they will depend on."
> "Deciding what bit differences are 'functional' is often open to individual interpretation.
> For instance, do we care about the order of lines and comments in /etc/inetd.conf? … even
> non-machine-readable bit differences can be meaningful when attempting to prove congruence."
> "Bit differences that are currently deemed not functional … may still affect the viability of
> future change directives. If we cannot predict the viability of future change actions, we
> cannot predict the future viability of the machine."

> [A-traugott-order-matters-2002]:§1 (relevance: -0:SUSPECT)
> "I found ordering surprisingly difficult to justify to an audience practiced in the use of
> convergent tools, where ordering is often considered a constraint to be specifically avoided"

> [A-traugott-order-matters-2002]:§4.3 (relevance: +1:SURE)
> "Congruence is the practice of maintaining production hosts in complete compliance with a fully
> descriptive baseline. Congruence is defined in terms of disk state rather than behavior, because
> disk state can be fully described, while behavior cannot."
> "A congruence tool typically works by maintaining a journal of all changes to be made to each
> machine … The journal is usually specified in a declarative language that is optimized for
> expressing ordered sets and subsets."

> [A-traugott-order-matters-2002]:§5 (relevance: +1:SURE)
> "we cannot analyze the behavior of any application-layer tool as if it were a standalone program."
> "When using automated administration tools we cannot consider the underlying layers to be
> axiomatic; the administration tool itself perturbs those underlying layers."
> "Inspection of high-level code alone is not enough. Without considering the entire system and its
> resulting machine language code, we cannot prove correctness."

> [A-traugott-order-matters-2002]:§2 (relevance: +1:SURE)
> "the tools themselves need to have some reasonable default behavior that is safe if the user lacks
> this theoretical knowledge."
> "No tool or language can remove this need [for testing], because no testing is capable of
> validating a change in any conditions other than those tested."

## F3 source 2 — Burgess, cfengine/promise-theory (the convergence camp's self-account)

**Verdict: more relevant than its acquisition grade (`-0:SUSPECT`) implied — it earned its keep.**
Forward-correction to [B-burgess-cfengine-2010] in `sources.json`: the conclusion-slide cross-camp
validation + the promise-as-contract primitive are load-bearing, not mere foil.

### Findings (lifted)
- **f10 — CROSS-CAMP VALIDATION (the headline).** Burgess's 2010 *conclusion* declares convergent
  self-healing **solved** and names the *remaining* problem as "Knowledge Management: **Tracking
  state · Understanding intentions** · Aligning with business goals." So the *convergent* camp's
  founder and the *congruent* camp (Traugott) **independently converge** on "tracking state +
  understanding intent = the hard/unsolved part." +SURE this is strong external corroboration of
  (a) round-9's topic and (b) the unsolvability spine — *intent* (≈ "does `do_y` need `x`") is the
  named-undecidable. [strengthens **dm-unsolvability-principle**]
- **f11 — convergence = "run many times, system never gets worse" (= convergence + idempotence),
  formalized as *more than* idempotence.** Dorc's DESIGN explicitly picks **idempotence, NOT
  convergence** (the "not convergence" link). So Dorc deliberately sits on the simpler idempotence
  rung and rejects cfengine's repeated-application self-healing model. Confirms `f2` + the foil. [foil]
- **f12 — PROMISE-AS-CONTRACT-PRIMITIVE (feeds F2).** cfengine: "configuration = promises +
  patterns"; a *promise* = an autonomous agent's *best-effort* declaration; idioms "reduce the
  information required for system description"; "local system always has last word" (**autonomy**:
  you can only promise *your own* behavior). Candidate model for Dorc's contract: an oracle
  *promises* its footprint/effect — best-effort, local, info-reducing — which is philosophically
  *closer to* `kBURDEN-minimize` / `kDEPS-accept-partial` than Terraform/Nix total-declaration. Key
  divergence to respect: cfengine promises *desired end-state* (declarative); Dorc's contract is
  about *effects/frames* + *intent*. Autonomy → compositional principle: each oracle promises only
  *its own* command's footprint, never the whole system. [feeds **dm-contract-as-component** + F2]
- **f13 — corroborates the DESIGN shell-fallback thesis.** "Many users use the framework but don't
  use the tools as intended, embedding shell commands because they don't see a better way." Even
  cfengine's *own* userbase falls back to shell — the exact DESIGN bet (gradual-enhancement on a
  shell substrate). [corroborates DESIGN]

### Citations
> [B-burgess-cfengine-2010]:slide "What is cfengine?" (relevance: -0:SUSPECT)
> "A largely declarative language for describing desired (or "promised") states"

> [B-burgess-cfengine-2010]:slide "History" / "Uncompromised Principles" (relevance: +1:SURE)
> "1999-2002 Formalized concept of "convergence" and limits for system correctness (more than idempotence)."
> "Run many times — "system never gets worse"  (convergence + idempotence)"
> "Autonomy of control — not allowed to send Cfengine instructions from outside. Local system always has last word."

> [B-burgess-cfengine-2010]:slide "Cfengine 3 - redesigned" (relevance: -0:SUSPECT)
> "Model: configuration = promises + patterns / Cfengine "promises" these patterns. / Offers generic idioms that reduce the information required for system description"

> [B-burgess-cfengine-2010]:slide "Assessment" (relevance: -0:SUSPECT)
> "Many users use the framework but don't use the tools as intended, embedding shell commands because they don't see a better way."

> [B-burgess-cfengine-2010]:slide "Conclusions" (relevance: +1:SURE)
> "We know how to do convergent self-healing now / Main problem is one of Knowledge Management"
> "— Tracking state / — Understanding intentions / — Aligning with business goals"

## Open threads / next
- Queue: one **Burgess primary** (promise theory or the cfengine "convergence > idempotence"
  account) for the convergent camp's *self*-account — but low-marginal; Traugott + DESIGN's
  "not convergence" link already pin the contrast. ~SUSPECT a single skim suffices; don't
  over-invest.
- f1/f2 (Dorc's coordinates: congruence-outcomes-without-the-baseline) is a candidate **DESIGN
  framing** to surface to the human — it may deserve a sentence in DESIGN's positioning, but that
  is the human's call (AGENTS: don't edit root .md).
- The `kVOLATILES` time-sensitivity caveat (f5) is a genuine refinement to flag at synthesis.
