# 23L — the oracle↔Dorc interface rulings (STAMPED in-conversation, human, 2026-07-02)

The outcome of the in-round interface work (seeds: `23K` ground-truths + `23J`
conv-rc-soundness; the human ruled this did not warrant its own round). Dialogue-derived,
human-stamped ("I think that's a solid plan… Stamp it."). Naming discipline (`23K §2`) in
effect throughout.

## rul-role-split (STAMPED)

A rich oracle is a family of role-sibling functions — separate functions giving separate
channels for separate rc-meanings, all spelled-sh. SCOPE OF THE OOB-REJECTION (human
clarification, same day): printf/OOB transport is ruled out *for this specific pair of
purposes only* — predicted-rc and guard-verdict both have natural rc-shaped consumers, so
forcing either out-of-band is ugly. OOB/printf communication remains alive and sanctioned
for everything else (the probe-report lane, refusal records, facts, diagnostics) — do not
read this ruling as a general OOB ban:

- **`foo.check()`** — facts and prediction. Its aggregate status keeps the INCUMBENT meaning:
  predicted-rc (the round-20 elision-substitution weld, untouched and un-reopened). Marks
  carry facts; richness unlimited.
- **`foo.is_converged()` / `foo.is_diverged()`** — the guard-verdict function; the SENSE IS DECLARED
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

- **0** = the named sense holds (converged for `is_converged()`; diverged for `is_diverged()`);
- **1** = the named sense's complement;
- **≥ 2** = CONFUSED — and confusion always lands on run.

Consequences, all mechanical:
- The minimal guard-capable oracle collapses back to ~4 lines: a bare passthrough
  `apt-get.is_converged() { dpkg-query -W -- "$1" >/dev/null 2>&1 ;}` is fully
  confusion-handled (127 → run) with ZERO boilerplate, because the tools already speak the
  partition. The three-way `case` boilerplate is dead.
- The probe lane reads the SAME partition as the ternary plan-verdict
  (0/1/≥2 → converged/diverged/can't-tell); apply collapses can't-tell → run in the glue.
  One authored function, both lanes, one source of convergence-truth.
- Direct-sense glue is the existing welded insertion, unchanged by a byte:
  `( foo_is_converged args ) || <original bytes>`.
- Declared-dual glue is engine-emitted mechanical sense-flipping (sanctioned scaffolding):
  `( foo_is_diverged args; [ $? -eq 1 ] ) || <original bytes>` — lossless inversion: present(0)
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

## Addendum (same day) — the rename ruling

`check()` → **`predict()`**, corpus-wide, RULED: "unclear/compressed naming"; the rename
aligns the function's name with its own rc-semantic (aggregate = predicted-rc) and with
inv-one-observable's own verb ("the oracle … *predicts* per-channel values"). Mechanical
global churn explicitly sanctioned, historical naming-truth explicitly waived (task #18;
sequenced AFTER the in-flight P5 closer integrates — it is mid-flight in these files).
The reservation-collapse is ACKED (human, same day): the whole predict-output-shaped
contract-surface is exactly two invocation-contracts — "we call `predict()` when we need you
to SIMULATE the command" and "we call `is_converged()`/`is_diverged()` when we need to know whether
system-state is STEADY." Future declared-output values EXTEND `predict()`; no third sibling.
Conductor boundary-notes from the logic-check (neither breaks the collapse): (i) state-FACTS
(the marks/establishes) are neither output- nor verdict-shaped — they ride `predict()`'s body
as annotations and travel the probe-report lane; identity-annotations' canonical home stays
`predict()` (the verdict-functions just consume argv). (ii) THE ONE TO NOT FORGET (build-slice
reconciliation): post-build there must be ONE convergence source for LICENSING at vouched
sites — the verdict-function's partition result — with the fact-plane's ambient-convergence
machinery serving reasoning/display/propagation, never a second license-source; two
convergence oracles for one site is the model-disagreement bug re-entering through the back
door. Mark the classifier accordingly when the tier builds.

## Residue + routing

- The check-rc/verdict conflation (the "fifth appearance", `23J` conv-rc-soundness +
  the predicted-rc laundering) is resolved AT THE ROOT: one function's status no longer
  serves two masters. An `is_converged()` written as a passthrough to `predict()` asserts the
  predicted-rc≈convergence coincidence as an explicit, attributable one-line claim — visible,
  licensed, no longer ambient. `jc-predicted-rc-provenance` retires with a residual note:
  vacuous `check()` bodies still yield garbage predicted-rc, and consumed-status sites
  refuse exactly as the existing machinery already does.
- Pins to author (re-derived from `23J`'s deferred pair; routed to task #15): the
  declared-dual glue-flip is lossless (127 → RUN — the pin that makes the backwards-guard
  unspellable); partition floors (a ≥2 verdict-status never skips; a `!`-collapsed
  is_converged() is a contract violation — pin the attribution surface, since the collapse
  itself is undetectable in arbitrary sh).
- Vouch SPELLING stays human-reserved (dq-kOOB), with one new open interplay for that pass:
  whether authoring a verdict-function on a verb-path partially IS the vouching act.
- Doc-consequences (human's queue, task #16): USER_STORY's minimal-oracle stages and
  stamped-233's exemplars eventually teach `is_converged()` + the partition + don't-collapse;
  the type-encoding of the whole discipline (ProbeRc/GuardRc-class newtypes, two blessed
  crossings) lands in the build slice.

## Addendum 3 (same day) — strip-fidelity ruled (jc-vouch-mark-strip-fidelity → resolution A)

The human's principle, verbatim-in-spirit: *"the last substantive command written by the
author should be the last exit-status-affecting statement in the stripped body. Our
`:`-lookalikes are noop annotation-lines, equivalent to comments, NOT equivalent to POSIX-sh
`:` commands."* So the strip deletes bare-mark STATEMENTS whole (vouch/ACK/POISON lines that
are only a mark) — they were never commands, the `:` was only the sh-native carrier.
Execution: the `strip_check` change + its unit pin ride task #15; the guard23 vouch-mark
conversion + flagship golden re-derivation stay deferred into the vouch/is_*verged respell
(converting to a throwaway strawman buys nothing). Welded home: the strip-fidelity
clarification appended to rul-ternary-verdict in spike/CLAUDE.md.

## Addendum 2 (same day) — the is_ prefix; centering; "the oracle" defined

- **Naming RULED:** the `is_` prefix — `foo.is_converged()` / `foo.is_diverged()` (stripped:
  `foo_is_converged` etc.). Applied to the live rulings text above; rides task #18's sweep
  everywhere else.
- **The verdict-function is the guaranteed-to-run function** (convergence must be known at
  ALL phases), and is therefore the DEFAULT-ASSUMED HOME for fact-establishes — authors will
  naturally mark facts there. Marked bears-more-thought / good-enough-for-now (human); the
  dialect stays available in every oracle function (no placement restriction); the lift walks
  the UNION regardless.
- **Expected authoring order + centering:** authors write `is_*verged()` FIRST; `predict()`
  arrives later, on Dorc's hinting, when an observable-consumer makes it necessary. Docs,
  defaults, hints, and discussion center on `is_*verged()`; historical material centers on
  check()/predict() and that division is accepted as fine.
- **"The oracle", defined precisely (ends the check-vs-oracle conflation, now purposeful):**
  the oracle = *the union of all `<somecmd>.<someinterrogator>` bodies discovered through a
  complete abstract-interpretation of the whole constructed code-unit* — subsuming both
  `predict()` and `is_*verged()`. "check()"/"predict()" name specific members, never the
  whole.
- The human's evaluation stance, on the record: consequences of pushing correctness to the
  contract-surface are ASSESSED POST-BUILD — "we'll build the thing and make sure it's
  secure, and then see how painful the straightjacket of failures is and how they map to the
  real-world-straightjacket. I suspect we're doing a good job of keeping them sore in all the
  same places people already have callouses… but that'll be hard to evaluate without
  finishing an entire product and trying it on, myself."
