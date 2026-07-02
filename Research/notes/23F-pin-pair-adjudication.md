# 23F — adjudication of the pin-set review pair (23B neutral · 23C adversarial)

Conductor adjudication, 2026-07-02, under the standing skepticism calibration (convergence =
signal; adversarial-only = suspect-until-checked; demonstrated > argued; never credulous toward
hostility). Sources harvested to `notes/23B` / `notes/23C`. AI process-evidence, not a
correctness claim. Repairs recorded in an addendum below once executed.

## The verdict, cross-compared

**Both passes independently conclude: build against this set.** The neutral pass verified every
checkable register-claim by execution (baseline reproduced; all six lens-lifted failure
signatures match; hand-authored goldens self-consistent; floors discriminate hand-crafted
wrong-builds). The kill-brief pass states plainly the safety-critical core held: the
fall-through trio has real exec teeth, the controls are well-toothed, the vouch-inert
differential is sound. The pins-as-behaviour survived a genuine attack. What did NOT survive
cleanly: the *insertion form's environment interaction* (composition holes outside any single
pin) and the *build-window promotion path* (one-sided xfails + bless). Ledger:

## Convergences (credited at full weight)

- **conv-1 · the build-window blindness** (23B-fd1 ≡ 23C-fd4/fd5, independent, different
  angles): XFAIL cases skip the content golden-diff, so (a) a regression that starts wrongly
  ELIDING vouched past-wall sites keeps xfails red with a changed signature — undetectable
  mechanically during the build window; and (b) XPASS fires on gates alone, so every
  artifact-shape law living only in golden text (the two nevers, bytes-survive-verbatim,
  strip-only) is documentation without teeth on the promotion path — 23C demonstrated a fake
  engine violating never-1 AND mutating fall-through bytes being offered promotion. Repairs:
  two-sided xfails (optional `head-expected.ran` asserted while XFAIL present); gate-6 selftest
  guard-confounds BEFORE the build round; a grep-floor asserting `<check> || <original bytes>`
  shape at guarded sites; a written diff-before-bless promotion rule.
- **conv-2 · the hand-guarded floor passes for the wrong reason** (23B minor ≡ 23C-fd6,
  identical): no `dpkg-query` mock, so a forbidden stacked guard's check 127s invisibly and the
  ran-set is byte-identical — 23C produced the forbidden artifact and passed the floor. Repair:
  exit-1 shim (either rc then reds it); same shim-absence weakens `top-argv-runs`.
- **conv-3 · the SyncThing conflict artifact** committed in the flagship case dir (both
  flagged): delete; glance for conflicted golden twins.
- **conv-4 · st-2-shaped probe halves vs the signed sourcing text** (23B-fd3 ≡ 23C minor): the
  goldens model the probe/check split while rul-ternary-verdict says "same bytes, both lanes" —
  the signed rationale currently leans on the deferred one-body property. ALREADY IN FLIGHT:
  the phase-0.5 Opus reconciliation dismantles the split; after it lands, probe ships stripped
  check bodies and the rationale becomes structurally true. Carry as integration-check, not new
  work.

## The demonstrated adversarial finds (single-source but executed; conductor-verified mechanism)

- **adv-1 · variable-namespace clobbering (23C-fd1) — the material design finding.** POSIX
  functions execute in the caller's variable namespace; the pinned preamble is the check body
  verbatim (strip-only is law); the corpus's own strawman check-bodies assign `verb`/`pkg`
  bare. Composition (every piece individually pinned): `pkg=vim; hork wombat; apt-get install
  -y curl; apt-get install -y "$pkg"` — curl's guard runs `pkg="$1"` ⇒ clobbers the book's
  variable ⇒ vim never installs AND the suppressed curl-install re-runs. Silent, rc 0, both
  execution commandments broken. Mechanism +SURE (POSIX scoping is certain; the demo executed).
  No pin composes a guard with downstream variable reads.
- **adv-2 · nounset kills the book at the guard (23C-fd2).** Under `set -u`, the check body's
  unconditional `"$2"` read is fatal on single-operand invocations — bare book completes,
  pinned artifact dies rc 2 at the guard, the tail under-executes. (The 218a set-u hazard,
  demonstrated; distinct from the deferred errexit-CONSUMPTION axis.)
- **adv-3 · runtime path-drift fail-open (23C-fd3) — argued, not executed; hazard-register
  tier.** The vouch scopes by static reach; the shipped whole body path-selects at runtime;
  host-conditional branches (capability fallbacks) can land on unvouched rc-0 paths ⇒ guard
  passes with no live verification. Both fixture bodies rc-0 their refuse paths (verified).
  Forward-looking oracle-idiom hazard → oracle-contract lint territory; not pin-now.

## Missing pins accepted for repair (cheap, behaviour-only)

- cant-tell-at-plan ⇒ run (23C-fd8; matches the m-a converged-only mint both reviewers endorse
  — pending the human's one-line ratification).
- cross-oracle vouch scoping: vouched-A never licenses unvouched-B's site (23C-fd9).
- redirect-carrying lines refuse guards for now (23C-fd10; consistent with HEAD's elide-tier
  treatment of non-devnull redirects; needs the human's nod as a refuse-home addition).
- composition pins for adv-1/adv-2: guarded artifact ≡ bare book w.r.t. book variables and
  under `set -u` (pin the BEHAVIOUR; the mechanism — subshell-wrapped invocation vs an
  oracle-contract `local`/`${2:-}` discipline + lint — is the human's ruling, below).
- why-attribution substrings conjoined per-line (23C minor); cmdsub exec-pin upgrade (23B
  minor).

## Discounted / de-biased

- 23C's "the artifact-shape law has no mechanical teeth" — true TODAY only in the build-window
  sense (the gate-6 widenings its demo used don't exist yet); calibrated to conv-1's repair
  list, not a today-hole.
- 23C-fd8's "front-load breach" rhetoric — overdone; m-a already implies the fix; kept as the
  cheap pin.
- The banned-word `skip-unresolvable` renderer token (23C minor) — real, pre-existing at HEAD,
  golden-churn-coupled → deferred to the phase-0.5 integration or the build round, not this
  repair pass.
- 23B-fd2 (strip-rule hyphen munge letter-defect) — REAL and needs a human one-liner (dash
  rejects both `apt-get.check(){}` and `apt_get-…` forms; fixtures pre-munge as
  `apt_get__check`, which "nothing else changed" doesn't authorize). Not discounted — routed to
  the ask-list; recorded here because it is ruling-text debt, not pin debt.

## Asks put to the human (h-slugs)

- **h1** ratify the m-a mint reading: a guard mints only where the plan-time verdict is
  *converged* (cant-tell ⇒ run) — both reviewers endorse as the strictly-safer reading.
- **h2** the strip-rule munge clause: the rewrite maps the provider through the existing
  funcname munge (non-alnum → underscore; the engine's established convention) — one line
  amending rul-ternary-verdict's strip description.
- **h3** the insertion-form mechanism for environment isolation (answers adv-1/adv-2):
  subshell-wrapped call-site (`( name_check args ) || original` — engine-side, contract-free,
  isolates variables; ~ms fork per guard) VS an oracle-contract discipline (`local` + `${n:-}`
  + lift-refusal of bare assignments in check bodies — no fork, but a new authored burden and
  a lint to build). Behaviour-pins land either way; the mechanism is yours.
- **h4** ratify redirect-carrying lines as a refuse-home (guards refuse; site runs) pending
  real design.

## Addendum: rulings received + repair dispatch (2026-07-02)

- **h1 RATIFIED:** converged-only mint. A guard mints only where the plan-time verdict is
  *converged*; can't-tell ⇒ run.
- **h2:** munge fine-for-now — spike-temporary territory; map the provider through the standard
  funcname munge; the human is not married to it either way.
- **h3:** the language target is dash-ish, barely-more-than-classic-POSIX ("POSIX2024-ish",
  which carves out `local` without specifying it) — LEAN INTO `local`: expect/encourage
  sanitary check bodies as good hygiene, never as sandboxing (the rm-rf example reigns; Dorc
  is not and never will be a sandbox). Forking gently deferred, both directions sanctioned —
  "or hell, do both": subshell-wrap acceptable now (removable later; more expensive to ADD
  later if anything starts depending on leaked context), local-hygiene encouraged regardless.
  Behaviour pins land independent of mechanism.
- **h4 ACK:** redirect-carrying lines refuse guards loudly; anything easy that avoids
  long-term design lock-in.
- **h5 DROPPED** by the human — do not pursue.

Repair pass dispatched to an Opus-class agent (isolated worktree; spec = this note + 23B/23C):
the five new pins (var-namespace, nounset, cross-oracle vouch scoping, cant-tell-runs,
redirect-refuses), two-sided xfails (`head-expected.ran` markers), the dpkg-query shims,
why-attribution per-line conjunction, the cmdsub exec upgrade, gate-6 selftest guard-confounds
+ the artifact-shape grep-floor, conflict-artifact deletion, and the diff-before-bless
promotion rule. Record: `notes/23G` on integration.
