# 202 — Take-3 input-side design: two faces, one value-plane (pre-build)

> Round-20 design note, written before dispatching builders; this is the shape the build
> executes against, and the artifact a crosscheck should attack. AI-authored, confidence-marked.
> Implements 19H within the 19A §5 settled model; trust R/D/I/K + STALENESS-AUDIT + 19H/19I over
> this. Decisions here are mine (the orchestrator's) under the charter's latitude; each carries
> its reasoning so the human can overrule cheaply.

## §1 The two faces share a value-plane, but they are *different computations*

19H §1 asks for one analysis with a pre-probe and post-probe face. Building shape (+SURE on the
split, ~SUSPECT on naming):

- **face-book** — *book-side propagation*: a flow-sensitive constant/variable propagation over
  the existing CFG via the existing monotone worklist (`analysis::solve`). Domain per program
  point: `MapL<VarName, Flat<StrValue>>` plus the positional-parameter frame (`$0..$n`, `$@`,
  `$#`) as a small vector domain. Transfer: assignments, `case`/`if` joins, `&&`/`||` paths,
  literal-word resolution. Anything non-literal (command-substitution results, arithmetic, env
  defaults `${x:-y}` beyond literal cases) ⇒ that var ⇒ ⊤. Output: for each command-site, the
  argv as `Vec<Flat<StrValue>>` — fully-concrete, partially-⊤, or ⊤.
- **face-check** — *check-body concrete evaluation*: the key simplification this design banks
  on. At a command-site whose argv resolved fully-concrete, "tracing the value through the
  oracle's argparse" is not abstract interpretation at all — it is **concrete evaluation of a
  constrained sh dialect over a known argument list**. The argparse `while`/`case`/`shift`
  loops over a concrete `$@` terminate by construction (each iteration consumes arguments); the
  evaluator is a little deterministic interpreter (no fixpoint, no widening) that runs the
  check's control-flow until it reaches (a) the inline kind-annotation — yielding the entity
  binding, (b) the probe-body arm selected by the derived verb, and (c) the declared-effect
  lookup key. ANY step outside the constrained dialect, any non-concrete value, any
  iteration-budget overrun ⇒ ⊤-with-reason ⇒ the site is unresolved ⇒ un-probeable, runs
  (`kFAIL` both directions: nothing ships, nothing elides).
  <!-- /* update 2026-06-10 (notes/205 §4): §3's closing "(site-id, channel, value)" is the
  binding shape — an earlier draft's "(check-id, …)" wording survives below; results are
  SITE-keyed, never check/command-family-keyed (now inv-site-keyed-results in spike/CLAUDE.md).
  And §4's "no strip/transpile pass gets built" governs oracle FILES only: the probe-artifact
  EMITTER renders the annotation as a plain assignment (205 §1 rule-anno-render) — shipping it
  verbatim breaks the probe under dash. */ -->
  Scope rule for this round (+SURE it's sound, ~SUSPECT it's the right precision point):
  face-check requires the site's argv **fully concrete**; a partially-⊤ argv (e.g.
  `apt-get install -y "$dynamic"`) is an unresolved site, period. Evaluating the argparse with
  ⊤-holes (to still derive the verb, enabling a kind-wide weak-kill instead of full Opaque
  poison) is a real precision upgrade — deferred, recorded here so it isn't re-derived as if
  novel. Deferred-not-irrelevant: it returns under the lazy-admin axis (books with one
  variable package-name are common).

Consequence for 19H §4's substrate question (this is the prior-art ruling, by construction
rather than by survey): **no new substrate.** face-book rides the existing worklist with a new
composed lattice (the combinators already exist: `MapL`, `Flat`, `Product`); face-check is not a
dataflow problem. seam-finite risk is contained (Flat domains are height-2; the map is over
program variables, finite per script); seam-interproc is *avoided*, not solved — the only
cross-file flow is book-argv → check-body, which face-check handles by evaluation, and `. /path`
sourcing stays ⊤ this round (recorded as deferred, not irrelevant: it returns under the
other-user axis — admins do source helper files; the degrade is safe meanwhile).

Per-file complexity dial (19H §1.3): face-book's budget (nesting depth, var-count, loop
handling) is one knob; face-check's iteration budget is another. They can sit at different
values without different machinery — matching "a threshold, not a different analysis."

## §2 The effect-class gains `Query` — the F1 fix falls out

Spike-2's `CommandEffect` knows `Establish/Kill/Pure/Opaque`. The settled model (19A §5 via
audit owed-halfb; 19I group E supersession) adds the read-only guard as a first-class class:

- **Query(cell)** — a command the oracle declares *reads* a cell and mutates nothing
  (`command -v`, `dpkg -s`, `getent …`). Its check() IS the guard, shipped and run read-only in
  the probe; its **probed rc is the site's Status channel** (probe-sourced, per
  inv-probe-sourced-values) and flows into the fold to resolve the guarded branch. This — not
  an unconditional render-floor block — is how `if`/`||` guards participate (supersedes 19I
  group E's stopgap).
- **Staleness discipline (found by exclusion-check, other-phase axis; +SURE it's load-bearing):
  a Query's probe-time rc is valid at apply only if its cell is *ambient at that site* — the
  same reaching-defs gate establishes use. `apt-get purge nginx; command -v nginx || apt-get
  install nginx`: the purge kills `tool:nginx#exists` upstream, so the guard's probe-time rc
  (taken on the resting host) is stale ⇒ Status=⊤ at that site ⇒ the guard runs for real at
  apply. Without this, the Query class is a wrong-elision factory. The ambient gate therefore
  applies to query-rc-USE, not only to establish-elision.**
  <!-- /* superseded 2026-06-10 (round-20 crosscheck, notes/205 §2 find-stale-crosskind): the
  rule above is WRONG as written — it is same-cell-only, and this very example defeats it:
  purge kills `package:nginx#installed` while the guard's cell is `tool:nginx#present`
  (DIFFERENT kind; also the `#exists` vocabulary above matches no fixture). Cross-kind
  dependence is invisible to the cell-model, so an oracled upstream mutator leaves the guard
  "valid" and the fold under-executes. Replaced by rule-query-validity (205 §2): a Query's
  probed rc folds only when NO effect-bearing command reaches it from entry (pristine-prefix).
  Do not implement the ambient-gate version. */ -->
- Effect-map declaration shape (opt-c1, ~SUSPECT — cheapest faithful reading of "the union of
  the book's ordering-and-args with the oracles' declared check-bodies"): the per-verb effect
  declarations remain a separate oracle-side declared map — now keyed by the *argparse-derived*
  verb (face-check output), never word-position; verbless providers (useradd) key on (provider,
  ε). The exact sh spelling of that map stays the old marker idiom for this spike (pending
  dq-kOOB; disposable). The alternative — effects declared inline inside check() case-arms via
  marker calls — is prettier but couples the evaluator to more dialect; deferred, noted as a
  candidate for the real kOOB ruling.

## §3 Probe results re-key: site-keyed observables, convergence derived

Spike-2's stdin lane is `kind:entity#sel verdict [rc=N]` — fact-keyed, with the rc injected.
Take-3 (+SURE on direction, the exact line-grammar is disposable):

- The probe artifact contains, per *resolvable command-site*, the oracle's check() as a
  function plus one invocation with the site's **full verbatim argv** (C-1); each invocation
  reports against a stable **site-id** (the LeafId/AstId back-map — inv-leaf-seam pays off
  here).
- The results lane carries `(site-id, channel, value)` triples (still line-oriented text for
  the spike): Effect as holds/absent/cant-tell (the fact-probe's three-outcome), Status as the
  probed rc *for Query sites* (a mutator site never reports Status — fork-mutator-rc adopted:
  un-probeable ⇒ ⊤ ⇒ runs; the `rc=N` injection lane is deleted in the same change, stage-2).
- **Convergence is derived engine-side** from Effect + the cell-model + the ambient gate —
  never read off the wire as a verdict-word. `Verdict` survives only as the Effect channel's
  value-type (19G deviation-2 stands).
- hostsim re-keys accordingly: the DST host answers per-site check-invocations (its modeled
  fact-store keyed by cell as today), and the kFAIL-withhold monitor still trips on any modeled
  mutation attempt from a shipped probe.

## §4 Annotation spelling (fork-annotation-spelling, executed as charter-sanctioned debt)

Inline form per ch-shape-anno: `pkg : com.debian.apt.Package = "$1"`. Implementation lean
(~SUSPECT until builder verifies): this line already parses as a *plain command* under the
spike parser (word `pkg`, args `:`, kind, `=`, `"$1"`), so **no parser change** — the
face-check evaluator recognizes the `[name, ':', kind, '=', value]` word-shape inside check
bodies as the annotation form. Books never contain it; if one does, it ⊤-rejects at classify
like any unknown command. The off-ramp breakage (17O F-OFFRAMP) stays accepted spike-debt; no
strip/transpile pass gets built (191 ch-shape-anno verbatim).

## §5 What dies, what stays (the stand-in ledger)

- find-3 flag-strip + verb=word-1 (`effect.rs`): **dies** once face-book+face-check resolve
  entities; the greppable `find-3 STAND-IN` markers come out with it. `useradd deploy` resolves
  through its check's argparse (no-verb case) — the baked-verb fixtures re-express naturally.
- stdin `rc=N` + `andor-rc-vouch-wrong` + matrix masking tests: **die** (stage-1 cuts the
  fork-mutator-rc-mooted pieces now; the stdin lane itself dies with §3's re-key in stage-2 so
  fold behavior never goes untested in between).
- `StandIn{True,False,Exit(i32)}` value-preserving substitution machinery: **stays** — its
  remaining live use is reproducing *probe-sourced* values (a folded Query guard's rc), exactly
  inv-probe-sourced-values' sanctioned source (a).
- The cell-model, ambient gate, fold direction, leaf-seam, render gates: **stay** (19F keep-list).

## §6 Dispatch plan (model-tier rationale recorded for the meta-goal)

- task-A (face-book propagation, `analysis::value`): Opus, tight spec — classical dataflow on
  an existing substrate with existing test idioms to copy; failure modes are visible (tests).
- task-C (face-check evaluator + oracle re-key to command-keyed check()): the design-heaviest;
  spec'd by me in detail (this note + a brief), built by Opus, with an explicit flag-up rule
  for any tc-* shaped call; graduate to Fable if it bogs.
- task-D (probe-projection + cli/hostsim re-key + fold re-grounding + stage-2 de-cruft):
  sequenced after A+C; likely Opus with the §3 contract spelled out.
- Crosscheck (Fable-class, clean-context pair) after the keystone (A+C wired into classify):
  target propagation-correctness first — the no-floor obligation (19H §1.3): aliasing two
  operands, `shift` off-by-one, `"$@"` vs `$*`, quote-loss — the bug class that licenses wrong
  elisions with no degrade to catch them.
