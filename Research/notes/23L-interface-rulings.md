# 23L — the oracle↔Dorc interface rulings (STAMPED in-conversation, human, 2026-07-02)

The outcome of the in-round interface work (seeds: `23K` ground-truths + `23J`
conv-rc-soundness; the human ruled this did not warrant its own round). Dialogue-derived,
human-stamped ("I think that's a solid plan… Stamp it."). Naming discipline (`23K §2`) in
effect throughout.

## rul-role-split (STAMPED)

A rich oracle is a family of role-sibling functions — separate functions giving separate
channels for separate rc-meanings, all spelled-sh, chosen OVER out-of-band noise (printf
verdict-tokens etc., rejected as ugly/anti-idiomatic/anti-offramp):

- **`foo.check()`** — facts and prediction. Its aggregate status keeps the INCUMBENT meaning:
  predicted-rc (the round-20 elision-substitution weld, untouched and un-reopened). Marks
  carry facts; richness unlimited.
- **`foo.converged()` / `foo.diverged()`** — the guard-verdict function; the SENSE IS DECLARED
  BY THE NAME. Its body is the ONE authored place tool-rc → guard-verdict crosses; its
  aggregate status, read under rul-rc-partition, IS the verdict — which settles the apply-rc
  mint's owner: the oracle contract, via this function.
- Eliminations on record: lifted-conclusion sub-invocation (code-manipulation, fragile);
  mode-flag/env-var protocols (not spelled-as-sh); wrapper-passthrough (collapses INTO this
  design as its floor idiom). Fence re-affirmed: sibling ≠ st-2 — split by authored ROLE,
  whole functions invoked with the site's argv; never per-kind filing.
- `foo.predict()` remains reserved-unbuilt (the future occupant of
  `inv-probe-sourced-values`' sanctioned-declared-output carve-out; needed only when a
  consumer wants more than check()'s probe-record provides).

## rul-rc-partition (STAMPED)

The verdict-function's exit status is read against ONE fixed mechanical table — the POSIX
utility convention, blessed rather than transformed:

- **0** = the named sense holds (converged for `converged()`; diverged for `diverged()`);
- **1** = the named sense's complement;
- **≥ 2** = CONFUSED — and confusion always lands on run.

Consequences, all mechanical:
- The minimal guard-capable oracle collapses back to ~4 lines: a bare passthrough
  `apt-get.converged() { dpkg-query -W -- "$1" >/dev/null 2>&1 ;}` is fully
  confusion-handled (127 → run) with ZERO boilerplate, because the tools already speak the
  partition. The three-way `case` boilerplate is dead.
- The probe lane reads the SAME partition as the ternary plan-verdict
  (0/1/≥2 → converged/diverged/can't-tell); apply collapses can't-tell → run in the glue.
  One authored function, both lanes, one source of convergence-truth.
- Direct-sense glue is the existing welded insertion, unchanged by a byte:
  `( foo_converged args ) || <original bytes>`.
- Declared-dual glue is engine-emitted mechanical sense-flipping (sanctioned scaffolding):
  `( foo_diverged args; [ $? -eq 1 ] ) || <original bytes>` — lossless inversion: present(0)
  → run, absent(1) → skip, confused(127/2/254) → run. **This RESTORES structural protection
  for inverted logic** (better than the earlier refuse-to-guard proposal): the engine sees
  the sense in the function name and flips in glue; no information is destroyed.
- The author's negative contract (judgment-tier until linted, same trust-channel as the
  vouch): DON'T COLLAPSE STATUSES on the way out of a verdict-function — no `!`, no
  `|| true`, and mind that a pipeline keeps only its last command's status. sh's `!` is the
  named enemy: it destroys the 1-vs-127 distinction inside the body where no mechanical rule
  can recover it.
- The corpus's existing refuse idiom (`exit 254`) lands in the confused bucket automatically
  — the hz-refusepath hazard resolves by the convention already taught.

## Residue + routing

- The check-rc/verdict conflation (the "fifth appearance", `23J` conv-rc-soundness +
  the predicted-rc laundering) is resolved AT THE ROOT: one function's status no longer
  serves two masters. A `converged()` written as a passthrough to `check()` asserts the
  predicted-rc≈convergence coincidence as an explicit, attributable one-line claim — visible,
  licensed, no longer ambient. `jc-predicted-rc-provenance` retires with a residual note:
  vacuous `check()` bodies still yield garbage predicted-rc, and consumed-status sites
  refuse exactly as the existing machinery already does.
- Pins to author (re-derived from `23J`'s deferred pair; routed to task #15): the
  declared-dual glue-flip is lossless (127 → RUN — the pin that makes the backwards-guard
  unspellable); partition floors (a ≥2 verdict-status never skips; a `!`-collapsed
  converged() is a contract violation — pin the attribution surface, since the collapse
  itself is undetectable in arbitrary sh).
- Vouch SPELLING stays human-reserved (dq-kOOB), with one new open interplay for that pass:
  whether authoring a verdict-function on a verb-path partially IS the vouching act.
- Doc-consequences (human's queue, task #16): USER_STORY's minimal-oracle stages and
  stamped-233's exemplars eventually teach `converged()` + the partition + don't-collapse;
  the type-encoding of the whole discipline (ProbeRc/GuardRc-class newtypes, two blessed
  crossings) lands in the build slice.
- The human's evaluation stance, on the record: consequences of pushing correctness to the
  contract-surface are ASSESSED POST-BUILD — "we'll build the thing and make sure it's
  secure, and then see how painful the straightjacket of failures is and how they map to the
  real-world-straightjacket. I suspect we're doing a good job of keeping them sore in all the
  same places people already have callouses… but that'll be hard to evaluate without
  finishing an entire product and trying it on, myself."
