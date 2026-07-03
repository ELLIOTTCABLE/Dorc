# 240 — round-24 charter: the oracle↔Dorc interface round

AI-drafted charter, 2026-07-02, promoted-and-rewritten from the round-23 closing dialogue
(formerly `notes/23K`, now a pointer-stub; the human's ground-truths are reproduced verbatim in the appendix —
they outrank every synthesized sentence here). Plans-tier maintenance applies: keep scanned
for currency; annotate, don't rewrite, when superseded.

## Why this round exists

Round 23 closed the oracle-contract crisis with the ternary verdict {elide, guard, run} and
built the guard's licensing law — but the external direction-review (`23J`) found one
load-bearing gap the round had walked past four separate times: **the design keeps conflating
"convergence-state" with "exit codes read or created at various points."** The probe lane
interprets a check body's outcome THROUGH engine-read annotations (inversion, which-property,
nothing-claimed); the apply-lane guard, as welded (`<check-invocation> || <original bytes>`),
consumes the body's raw aggregate exit status with no interpreter present. Three concrete
failure shapes fall out of that one gap (a vouched rc-inverted arm mints a guard that skips
exactly when the world drifted to needing the command; refuse-paths that exit 0 pass a guard
having verified nothing; rich fact-reporting bodies — the shape the elide-half wants — have
semantically-vacuous aggregate rc and are structurally guard-unsound). The human's response
(appendix) set the ground-truths and wishes; this round settles the design.

Everything here gates the build slice: the guard emitter and its witness cannot be built
until the verdict's transport is decided.

## Goals (what this round settles)

1. **Guard-verdict transport.** How the authored is-this-converged boolean travels from the
   oracle's shipped work to the apply-artifact's deciding construct — without unqualified
   exit-status multiplexing. Includes ruling the **apply-rc mint's owner**: the mint itself
   (verdict → one Unix exit code driving the `||`-fails-toward-run machinery) is SETTLED as
   happening once, in one controlled way; the open question is who owns it — oracle-contract
   discipline (a dedicated function whose exit status IS the mint, by contract), an
   interposed shipped helper, or true cross-compilation of oracle bodies (in visible tension
   with the never-engine-synthesized-sh law; if it wins, that law gets a conscious re-weld).
2. **The vouch spelling** — the dq-kOOB cluster, finally due. The vouch's semantics are
   welded (explicit, path-scoped, judgment-tier, attributed); its concrete sh spelling has
   been strawman-tier since the crisis. Decisions reserved to the human.
3. **The sibling-function family.** Whether a rich oracle grows beyond one `check()` into
   authored ROLE-siblings — `foo.predict()` (invoked only in rc-insertion/replacement
   position; the FIRST sanctioned instance of `inv-probe-sourced-values`' reserved
   oracle-declared-output carve-out) and `foo.converged()` / `foo_is_converged` (invoked to
   decide elision-potential and inlined as the local guard) — with strong contracts on shape
   and outputs, each a reasonable standalone shell function outside Dorc.
4. **The rc/verdict discipline's type-encoding plan.** The naming discipline (below,
   in-effect) becomes minted types and lints in the spike: ProbeRc/GuardRc-class newtypes
   that cannot unify, exactly two blessed crossings, non-correlation as a compile-shape
   invariant. This round designs the encoding; the build slice implements it.
5. **The two deferred rc-soundness pins** (from `23J`; deferred because they respell against
   whatever channel wins): the vouched-inverted-arm backwards-guard pin and the
   refuse-path-rc0 pin — re-derived against the winning design and authored red.
6. A **witness shape** settled enough for the build slice to construct against.

## Success-state

Each goal ends as: a welded ruling with the original rationale engaged where a standing weld
moves (rul-ternary-verdict's insertion-form letter carries rc-consumption and is a named
re-weld candidate); strawman-or-final spellings grounded in actual sh; the pins authored and
red for their designed reasons; and the package adversarially crosschecked by a clean-context
pair before any weld (exclusions-not-inclusions; prompts per the standing methodology). The
round produces its own plans-tier synthesis on close.

## The rc/verdict naming discipline (IN EFFECT since 23K; binding on all round documents)

Parallel to skip→{elide, guard}: the blurry words decompose, and no term crosses lanes.

- **tool-rc** — a tool's raw exit status, read only INSIDE oracle bodies; opaque to Dorc.
- **probe-report** — everything the probe lane carries back (facts, observables, refusals,
  can't-tells); travels out-of-band, never in-band.
- **plan-verdict** — engine-side, per-site, TERNARY {converged, diverged, can't-tell},
  derived at plan time from the probe-report via the reached marks.
- **guard-verdict** — host-side, apply-time, the ONE authored BOOLEAN: truth ⇒ converged ⇒
  runtime-skip; anything else ⇒ run the byte-for-byte line. No can't-tell exists at the host.
- **apply-rc** — the exit status MINTED from a guard-verdict to drive the artifact's sh
  connectives; minted once, one sanctioned mechanism (goal 1 owns the who). Distinct from —
- **predicted-rc** — a probe-sourced replacement VALUE emitted in an elided command's
  rc-position (`inv-probe-sourced-values`); data, never a decision.
- Bare "rc" is a banned word in design text. Exactly TWO blessed crossings between the
  rc-world and the verdict-world: tool-rc → verdict (authored, ONE place per oracle) and
  verdict → apply-rc (the mint). Everywhere else, non-correlation is an invariant.

## Boundaries — explicitly NOT attempted this round

- The elide-half machinery (footprints, demand-disjointness, grounding bridges) — its own
  arc, human-keyed; this round must not foreclose its needs (two-halves discipline: tensions
  flag to the human, never resolve silently).
- The placement-spectrum round (task #11) — queues BEHIND this round; nothing here decides
  per-site-vs-hoisted-wave placement.
- The transitional freeze's dissolution (EstablishInverted → MustRun) — stays frozen; it
  dissolves during the build, against this round's rulings, never before.
- Multi-host plan-surface, escape-hatches, check-cost banding, TOCTOU/open-world drift —
  parked exactly where they were.
- Re-opening settled ground-truth: check-is-the-oracle, strip-only, arbitrary-sh,
  never-declared-output-in-guard-position all STAND; this round designs within them (the one
  named possible re-weld is the insertion-form's rc-consumption letter, goal 1).
- Building the lints/types (goal 4 designs; the build implements).
- Anything corpus/market-shaped; anything in the quarantined directories.

## Constraints (the human's wishes — binding as strong priors, appendix for the verbatim)

No manipulating authored code (expensive, subtle, surprising). No multiplexing/demuxing
information through collapsed channels — especially not collapsing information into exit
codes more than the substrate demands. No shipping complex non-oracle behaviour to hosts (no
powerful on-host executor — deliberately walked back from, prior rounds). Everything
spelled-as-sh: any new authored surface must be reasonable, valuable shell on its own,
outside Dorc, merely strongly-contracted. The off-ramp holds at every point.

## Method

The round-23 pattern, which held under hostile review: design dialogue with the human,
grounded constantly in strawman sh → durable notes per topic (naming discipline observed) →
a clean-context adversarial crosscheck of the package → re-welds with rationale engagement →
pins. Fable-tier reviewers by the human's dispatch rules; adjudication under the standing
skepticism calibration.

## Seeds and inputs

`23J` conv-rc-soundness (the three-facet gap, verified mechanisms); the appendix
ground-truths; the two in-flight intuitions banked with them (no-rc-at-the-interface, minted
non-unifiable types with one blessed interpretation site — both now absorbed into the
discipline above); `plans/23D` §1 (oracle ground-truth) and §3 (guards-can't-serve rulings);
`plans/239` (the closure whose re-welds this round may touch); the sibling≠st-2 fence: the
retired probe/check split filed MEASUREMENT per (kind, selector), engine-keyed — the sibling
family splits by authored ROLE, whole functions invoked with the site's argv, authors write
behaviour, the engine never files or synthesizes it. Any drift back toward per-kind filing
re-opens a settled grave.

## Open questions the round must answer (beyond the goals' letter)

How the probe-report and the guard-verdict channels relate (one output format for both
lanes, or two?); what a broken/absent sibling does at the glue (the 127-conflation must land
on run); whether `foo.predict()` and `foo.converged()` share measurement work and how
(performance; information-sharing; the rejected env-var-branching alternative); what the
guard glue looks like as rendered sh at each candidate (strawman every option); how the
existing 9 guard23 xfails re-read under the winning channel (lens re-verification).

## Appendix — the human's ground-truths (2026-07-02, near-verbatim; the round's bedrock)

> The only *non*-optional part: for guards, we *have* to produce *a boolean* value, somewhere,
> at the end of all this. That boolean needs to encode *convergence* (in the constrained,
> literal sense stated previously): the run-wishness of the immediate-following command, a
> byte-for-byte line from the runbook. "falseness" → unconvergence → run it; "truthness" →
> convergence → runtime-skip.
>
> All else is, to some degree or another, mutable: we can control the oracle contract, we can
> control what we compile out of the probe (depending on what we unweld), we can control all
> the other channels and lanes of communication.
>
> *To get* the semantic value of that, to *know* whether to run or not at that moment, we
> need work from the oracle author: thus, a second uncontrovertible truth, we *must* ship
> *something* from the oracle's work to the host. Behaviour, not static information — the
> 'is this converged' may be a complex *predicate* on system-state, that we (Dorc) can't
> know ahead-of-time.
>
> The mutable wishes: we don't *want* to manipulate code. we don't *want* to multiplex/demux
> from information channels, especially multiple times (i.e. collapse information into rc).
> we don't *want* to ship complex non-oracle behaviour to the hosts.
>
> The moving parts needed for implementation reasons: probe-time information beyond "is this
> converged" must travel somewhere (collect + display); we want to *replace* the
> candidate-command inline with a "predicted rc" in the observed-skip case; and we want to
> *guard* the candidate-command with an "is this converged" boolean.
>
> Maybe it's time to re-enrich the oracle language to "more than just one function" — not the
> accidental probe-vs-oracle split just removed, but *siblings* to `foo.check()`: perhaps
> `foo.predict()` (only invoked in rc-insertion-position) and `foo.converged()` (invoked to
> decide elision-potential, and further inlined as a local guard). The author has the full
> richness of turing-complete sh to D.R.Y./dedupe them. `foo_predict` and `foo_is_converged`
> are reasonable, valuable shell-script functions on their own, outside of Dorc, that we
> strongly-contract with specific demands on their shape and outputs.
>
> [On the verdict → exit-code direction:] we *need* a Unix exit-code to drive the
> ||-fails-correctly-in-subshell machinery; *that* is the thing to be minted once, in a
> controlled way — whether through oracle-contract, an interposed helper, or true
> cross-compilation of oracle-bodies is an open question; that it should happen *once, in a
> controlled way* is settled.
