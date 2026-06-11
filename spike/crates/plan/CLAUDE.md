# spike/crates/plan — CLAUDE.md

Elision/replacement, the probe→apply compiler, and render. Read `spike/CLAUDE.md`
(invariants) and `Research/plans/191-spike2-keystone-charter.md` (charter). Trust
the root `DESIGN.md` (the plan/apply UX) + `IMPLEMENTATION.md` ("To execute, or
not to execute?" — the priority order: never under-execute > avoid over-execute >
avoid unnecessary-execute) over `Research/`.

This crate owns: `PhasedVerdict`/`Bias` (the `Unknown`-fold), the `ReplaceLicense`
witness, `Disposition`/`Step`/`Plan` (the leaf-seam), `compile_probe`+`ProbePlan`
(forward half), `build_plan` (apply-2), and the two renders (`render_sh`,
`render_apply`). It is `inv-superposition`'s real second phased caller (`F-FW3`).

## What the keystone does to this crate (`ap-1`; do not pre-build past it)

+SURE: the §3 entity-algebra re-key lands in `core`/`analysis` *first*, and it
re-keys `FactKey`/`SkipClass`/`Polarity` out from under everything here.
`ReplaceLicense::prove_replaceable` destructures `SkipClass::EstablishAmbient(fact)`
and the elision predicate is *per-fact*; when a fact becomes a per-entity selector
(`package-index#fresh` vs `package:nginx#installed`, the poison-wall fix) the
predicate must resolve per-selector, not per-kind. ~SUSPECT the witness shape
survives the re-key (it keys on a `FactKey` + grade + verdict, all of which the
re-key only *refines*); the consumption gate is orthogonal to it. Don't widen the
witness API speculatively — let the re-key push the change.

## The elision predicate stays the sole minting site (`inv-must-may`, `inv-superposition`)

`ReplaceLicense` is the only place an elision is minted (`16P T6` witness; `T13`
the predicate). Keep it honest as the fact-domain gains structure — elide leaf L iff:

  `probe(L.fact)=Converged ∧ ambient ∧ Must ∧ no-consumed-unvouched-observable ∧ ¬⊤-contained`

The first four conjuncts live in `prove_replaceable`; ⊤-containment is a *separate*
guard in `build_plan` (`has_top_successor`, 16G hole-5) — keep it separate, it
guards a different failure (unmodeled execution context, not a stale fact). The
`can't-probe ⇒ can't-elide` link (`an-elision-predicate`): a kind with an effect but
no declared probe is absent from `compile_probe`'s output ⇒ the apply runs it
(`kFAIL-perform`). Consumed `Stdout`/`Stderr` arrive as the engine's un-collapsed
`May<Powerset<Observable>>` and per `inv-must-may` can only *block* (16F §3 / 16J).
Status consumption (19A C-3 honored; channel-set re-keyed by arch-1 — note 214,
`StatusRenderFloor` deleted): `StatusRelaxable` — `&&`/`||` operands, errexit-region
commands, `$?`-readers' predecessors, AND `if`/`elif` guards — a probe-sourced/declared
rc substitutes exactly, a ⊤ rc blocks (the arch-1 guard-elision class rides this: a
known-rc Query guard substitutes in-situ, span-exact). `StatusInvariant` — the bare
`|| true` left operand (door-3, `20V` §4 / note 213): consumed-in-form, dead-in-fact
(both continuations rejoin with identical observables), never blocks even at ⊤;
mark-union still applies, so any OTHER blocking mark on the same site wins.
`StatusIterated` — a `while`/`until` condition: a per-iteration consumed sequence no
single rc reproduces; blocks unconditionally. Under
fork-mutator-rc a mutator's rc is always ⊤, so BARE converged mutators under `set -e`
run (the 206 §2 headline cost; door-3 and the remaining 20V doors are the recovery
program). There is NO establishes-contract rc-0 vouch — that
was the refuted assumption ("converged ⇒ rc 0", bought false three times).

## `ap-2` — executable acceptance is non-negotiable (`an-render-runnable` / `an-render-executability-check`)

`render_apply` must emit runnable / `sh -n`-clean POSIX. Spike-1 shipped
`if true; then # …; fi` green — a syntax error (`then`-clause whose only body is a
comment; `fi` where a command-list is required) — because the e2e *string-diffed*
the artifact. The harness (in `cli`) must **execute or `sh -n`-check** the rendered
artifact, never golden-diff its text.

~SUSPECT the current line-granular `render_apply` (comment a line iff a `Replace`
leaf is on it and no `Run` leaf is) still has this trap latent: comment out the lone
body of an `if`/`while`/`case` arm and you reproduce the empty-clause syntax error
the line-comment approach was supposed to dodge. The flat `render_sh` sidesteps it by
dropping guards entirely (it shows dispositions, not a runnable rewrite — a known
first-cut limitation, `16P T14`). An `sh -n` gate on `render_apply` is exactly what
surfaces this; if it fires, that *is* the deliverable (note it), not a thing to
silently paper over with a `:` no-op filler unless that's the honest fix.

## `seam-prov` (a leading goal) — provenance / the why-tree, hand-built (`an-locator-dag`)

`Derivation` is the hand-built audit trail the plan UI greys-out as the "why". The
target model (`111` dac-A, the `re-eval-trigger`'s strongest later case) is a
PROV-shaped derivation-DAG `[B-prov-primer-2013]` of located-nodes + typed-edges:
**N-tier and per-host-forking**, a *variable-length list* of typed loci
(`loc-host`/`loc-user-src`/`loc-probe`/…), never pre-flattened — composition
collapses to the *coarsest* tier `[B-mozilla-sourcemap-2024]`. Today's `Derivation`
is the degenerate one-tier case. The leading-goal deliverable is *where the hand-built
DAG strains as taint + the locator-DAG grow* (the `re-eval-trigger` evidence), not a
green checkmark — Datalog gives why-trees ~free, the worklist must hand-build them.
Resolving interned tokens to text here is for display/provenance only
(`inv-referent-agnostic`); never branch on the resolved text.

## `ch-scope` — instantiate the backward / apply-3 / `dorc bump` skeleton

The whole `May`/`Must`/`Backward`/`BoundedLattice` tower exists but **no backward or
must-analysis is instantiated** (`16P T4`, `16Q q1-backward`). `build_plan` is apply-2
(forward-only). apply-3 (`dorc bump`, targeted desired-set) is **apply-2 + a backward
relevance-reduction** — `apply-3 ⊃ apply-2` (`an-apply-3`/`an-backward-slice`, `16P
T13`): a strict superset of effort, not a separate path. This crate is the second real
phased caller, so it is the load-test of "engine emits, caller collapses" (`F-FW3`,
`17O`) — until now one caller stood in as phase-agnostic, a one-caller fiction. If the
`May`/`Must` superposition survives a real backward caller it earns the locks
retroactively; if it breaks, the locks were premature (`16Q q1-backward`). Scope floor
only — no host mutation over time, no TOCTOU, no executor.

## `R2-CHANGEDELTA` → a `q1-precision` acceptance test (`17O`)

"Do B because A changed" (config-write → reload-iff-changed): the author's `changed=1`
flag is a **consumed observable the elision discipline must preserve, never
synthesize**. Eliding the config-write *removes its `changed=1` side-effect* — a real
`q1-interproc` hazard. Concrete test: track the `changed` variable across `cp →
reload`; Dorc must never elide a delta-gated effect via a *state*-probe nor synthesize
the cross-kind `file:`→`service:` edge. The un-probeable change-gated effect class is a
`TODO.md`-into-`DESIGN` item, **not spike scope** — encode the precision test, don't
add an effect-map dimension.

## Determinism

`inv-determinism`: `build_plan`/`compile_probe` are pure functions of their inputs;
the host verdict is *injected* (`verdict_of: impl Fn(FactKey) -> Verdict`; the real
host / `hostsim` is a later seam). Never reach for clock/RNG/fs/net here, directly or
transitively. Output order is observable (the rendered plan) → sort by span, key on
`BTreeSet`/sorted vecs, never iterate a `HashMap`.

## Tension RESOLVED (round-21, arch-1 — note 214; kept for history)

`an-render-modes` posed line-granular faithfulness vs leaf-exact provenance as a
possible genuine tension. arch-1's span-edit render IS the faithful control-flow
rewrite the --WONDER hypothesised: a Replace splices the leaf's exact byte-span
in-situ (the stand-in for Replaces, a `:` for dead Omits — arms never empty), the
`LeafId → AstId` back-map is byte-exact rather than line-blurred, and the artifact
stays runnable/`sh -n`-clean (gate ap-2). Render-fidelity and leaf-exact provenance
were NOT in tension; the line was simply the wrong substitution unit. The carve-out
family (`inline_arm_subst`/`inline_scaffold_subst`/`commented_line` + the
`classify_lines` walk) and the `StatusRenderFloor` channel retired with it.
