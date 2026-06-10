# 203 — dir-timed-probe (human reservation), stage-1 de-cruft record, dispatch decisions

> Round-20 note. §1 is the human's voice (2026-06-10, mid-round message), recorded near-verbatim
> because it's a reserved design direction with no other durable home. AI-authored framing,
> confidence-marked.

## §1 dir-timed-probe — the human's reserved right to ship CFG-structure into the probe

Context: my Query-staleness discipline (202 §2 — a guard's probe-time rc degrades to ⊤ at any
site whose cell an upstream in-script mutation reaches). The human's response, near-verbatim:

> human: "That staleness discipline" is part of why I've *reserved* the right (although not yet
> planned in specific) to ship CFG to the probe, instead of just saying 'a probe is a bag of
> unordered massively-parallelized checks.' If it's engineering-time-cheap (because we already
> have a value-tracking system and abstract interpretation *anyway*) … and sound (not sound*y*,
> but genuinely sound) … then I imagine at some point we might try and *figure out where two
> correlated guards depend upon one-another*, and then do a *correctly-timed* probe. […]
> intentionally ordering our check *after* the probe for `purge`, so that our "poisoned global
> context for effect Y" has actually checked that "y" is true AFTER THE POISONER'S
> KNOWN-FINAL-STATE. Or, in other words, that 'if' we had shipped the script wholesale, and 'if'
> the script's implementation were fully correct and idempotent … *then* we've correctly
> established what-would-have-happened-had-we-done-so […] ugh, that explanation was very
> complicated, don't over-trust it, it's a subtle ordering problem that would need exhaustive
> correctness-exercising, and is very much an edge-case, I hope. deferred prolly. tl;dr I have
> aspirations to *predict* the information we need to safely map the live-probed-state *onto*
> the (CFG u value-flow-analysis) superstructure; and then structure our probe such that it
> yields the maximum provably-rely-upon-able set of that state, *not* just a set of flat,
> boolean, "will-be-true-at-apply-start-modulo-TOCTOU-global-ass-values"

Status: reservation/aspiration, explicitly not direction-setting, "deferred prolly", flagged by
its author as not-to-be-over-trusted. Recorded connections (mine, +SURE on the first two):

- **This is `kFLATTEN`'s axis.** The knob's `kFLATTEN-maintain-cfg` pole ("keeps the
  apply-phase control-flow in the shipped probe, leaving probe-checks under
  (probing-versions-of-) their original guards") is the structural half of this reservation;
  the new semantic content is *virtual timing* — answers keyed to program-points ("y, as-of
  after-the-purge-point"), derived by abstract-interpreting declared effects forward from
  probed initial state. Don't re-derive this under a new name; it's kFLATTEN growing a
  timing dimension.
- **Today's sound floor vs. tomorrow's precision recovery.** The 202 §2 staleness gate loses
  exactly the elisions dir-timed-probe would recover: degrade-to-⊤-where-non-ambient is the
  conservative floor; program-point-keyed probing is the precise refinement. Floor first
  (this round), refinement reserved.
- **Seam already preserved.** 202 §3's site-keyed probe results (`(site-id, channel, value)`,
  never fact-keyed flat verdicts) is precisely the transport a timed probe needs — a site-keyed
  answer IS a program-point-keyed answer. Standing rule from this note: do not regress the
  results lane to fact-keyed booleans; that would weld out the reservation.
- ~SUSPECT, worth saying once (exclusion-check, other-reliability axis): timed-probes deepen
  the coupling between elision-validity and oracle-truthfulness. A wrong declared-effect today
  mis-handles its own command; under derived post-poisoner state it additionally invalidates
  *downstream* probe answers derived through it. That blast-radius change is presumably why
  "exhaustive correctness-exercising" — record it so the future evaluation starts there.

## §2 Hook resolution (human: "drop the hook if it's blocking you")

Implemented as `HK=0` in `.claude/settings.json` `env` (worktree-local; every agent shell
inherits it; the hook's own designed bypass), NOT as removal of the `hook.hk-pre-commit.*`
entries — those live in the SHARED `.git/config` and serve the main checkout, where hk works.
Cost accepted: gates no longer run automatically on agent commits; spike/CLAUDE.md now makes
the four-gate run a mandatory pre-commit step by discipline. (If agents are ever observed
committing ungated, the harder fix is the human's: drop the config-hook or absolutize
worktrees.)

## §3 Stage-1 de-cruft — what was cut, what deliberately stayed (commit 808931f)

- Cut: e2e `andor-rc-vouch-wrong` (its premise — a declared mutator rc=9 — has no sanctioned
  source under fork-mutator-rc); matrix test
  `nonconforming_establish_andor_left_operand_substitutes_exact_rc` + its `plan_for_user_oror`
  helper + the `replace_standin` helper (sole consumer gone); matrix test
  `andand_left_operand_declared_rc0_relaxes_and_replaces` (same masking class — injected
  mutator rc=0; 19I §2 named the rc-9 one explicitly, the rc-0 sibling is the same fabrication
  by my judgment, recorded here).
- Re-pinned to the fork's semantics: `pins_converged_status_via_{andand,oror}_replaced` →
  `…_runs_mutator_rc_top` — a branch-consumed converged mutator RUNS (the engine already
  behaved this way once the helper stopped injecting rc=0; the flip was test-layer only,
  zero engine change — +SURE, verified by the suite passing immediately).
- Deliberately stayed: the stdin `rc=N` lane in cli + `fold-oror-guard-omits` (a GUARD-rc
  case — the behavior probe-projection re-grounds in stage-2; cutting it now would gap
  fold-coverage); `andor-rc-undeclared-runs` (the keeper; its baked-verb `user.oracle.sh` wart
  re-expresses when check()-lifting lands); the `StandIn{True,False,Exit}` machinery and the
  engine's AndOrStatus-relaxes-on-declared-rc seam (what Query-guard rcs will ride);
  errexit's structural vouch (`exec-errexit-elide-vouched` green; the non-conforming-under-
  `set -e` priority-2 over-execute remains documented, 19E/19F §6).
- Suite state after: 136 cargo tests pass / 1 ignored; e2e 42/42 (xfail intact); all four
  lint gates green.

## §4 Dispatch decisions for the keystone build (adj-*)

- **adj-dialect-parser** (~SUSPECT-leaning-SURE; an implementation-balance call, 191 §5b
  category 3): the oracle-contract dialect gets its OWN mini-parser inside `dorc-oracle`
  (funcdef / while / case / if / shift / assignment / annotation / plain command), rather than
  extending `dorc-syntax` with loops. Reasons: the main parser ⊤-rejects loops by design and
  extending it drags `cfg::build`'s exhaustive lowering + errexit + consumption marking along
  (a mid-round blast radius); a dedicated parser *structurally enforces* "the dialect is NOT
  arbitrary sh" (19G §2) — outside-dialect input fails to parse ⇒ Top(reason) ⇒ unresolvable
  site, the right degrade; and it keeps the book-parser's conservative posture intact (books
  with loops stay ⊤ — sound). Cost: a second small parser to maintain; accepted for the spike
  (the front-ends are disposable by charter). Books and oracles still share ONE value-plane
  (the 19H §1.1 uniformity) — the dialect parser is a *front-end*, not a second analysis.
- **adj-parallel-additive**: task-A (`analysis::value`, face-book) and task-C (`oracle::check`,
  face-check) run as parallel background agents, additive-only (no existing API may change, no
  cross-crate breaks), no agent commits (shared git index; the orchestrator commits after
  review). Wiring into `effect::classify` (replacing find-3) is task-W, sequential, after both.
