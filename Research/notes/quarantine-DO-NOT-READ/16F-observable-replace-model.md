# 16F — the observable model: "replace," not "skip"; the unified stand-in contract

> **Status (2026-06-05): spike, round-16 design note — human-ratified.** Append-only;
> it **corrects 16C/16E by reference** (does not edit them — the rewound history
> stays intact). Mostly **future / outside-spike relevance**: it fixes the
> vocabulary and the conceptual model that "skip" was corrupting, and records the
> oracle-bridge enrichment. The *in-spike* floor it implies is small and bounded
> (§7). Confidence: the model is +SURE (worked through adversarially + with the
> human); the outside-spike bridge is a design target, not a commitment.

## 0. "skip" is verboten — and why it drove illogic
"skip" connotes *omit the line*, so it framed the decision as unary
(delete-or-keep a leaf) and silently installed **line-deletion** as the mental
model. That hid the real operation and pushed the implementation toward "elide = a
node nothing reads." The correction:

The operation is **replace**: put in a leaf's place the cheapest **stand-in** that
reproduces the **observables** its consumers read. Running the leaf unchanged is
the fallback; the trivial stand-in (`true`) is the cheap end; an oracle *bridge* is
the rich middle. "skip" was only ever the *degenerate* replace — stand-in = `true`
and nothing read it. ("replace" also does anti-illogic work for free: it forces the
question *"replace with **what**?"*, which is exactly the observable-preservation
question "skip" suppressed.)

## 1. Terminology (ratified)
- **replace** (verb) — the operation. `ReplaceLicense`, `prove_replaceable`.
  (was: skip / `SkipLicense` / `prove_skippable`.)
- **observable** (noun) — a thing a consumer can detect about a leaf having run:
  `Observable::{Effect, Status, Stdout, Stderr}`. (was: "channel" — dropped, it
  collides with SSH/mux terminology in this codebase.)
- **stub** — the trivial stand-in `true` (effect none, status 0, empty stdout/stderr).
- **bridge** — an oracle-supplied stand-in (gather@probe / compute@apply) for a
  consumed observable (16C).
- **elision / omission** — RESERVED for the *degenerate* replace (stand-in `true`,
  no consumer reads any observable) — the only case where the line truly vanishes.
- **"skip"** — verboten. **"subst"** — avoided in this sense; it is
  *command-substitution* `$()` throughout this codebase.

## 2. The unified model
Replacing a leaf = substituting a stand-in. A replacement is sound iff, for **every
observable a downstream consumer is value-sensitive to**, the stand-in reproduces
it. The trivial stub provides per-observable *defaults* — effect: none; status: 0;
stdout/stderr: empty — so the only question, per observable, is: *is that default
acceptable here?* What vouches each default:
- **Effect** — vouched by **convergence** (the forward gate: fact already holds,
  ambient, Must). Already built. Non-converged ⇒ run.
- **Status** — vouched by the **`establishes` contract**: declaring "(provider,verb)
  establishes F" *is* the claim "when F is converged, this command is a successful
  no-op" — i.e. status 0. Free, and load-bearing: under `set -e` every status is
  consumed, so without this vouch *nothing* could ever be replaced.
- **Stdout / Stderr** — vouched by **nothing** (`establishes` says nothing about
  output). So a value-bearing-consumed stdout/stderr makes the stub default (`""`)
  unsound ⇒ **run, unless an oracle `bridge` supplies the real value.**

## 3. One backward obligation — not a stdout special-case
There is a single backward (consumer→producer) obligation: *value-bearing
consumption of an **unvouched** observable forbids the stub.* It is **uniform over
observables**; the asymmetry between status (replaceable freely) and stdout
(run-or-bridge) is **entirely** "does the `establishes` contract vouch this
observable's default" — status yes, stdout no. Do **not** write separate
status-liveness and stdout-liveness analyses; write one **observable-liveness**, and
let the establish-contract discharge the status observable.

## 4. We never reason about RC (or stdout) *values* — the smell, named
The analyzer must not analyze return-code or output *values*. It substitutes a stub
and trusts the `establishes` contract to vouch status-0. A command that is
*converged-but-fails* (`mkdir d` returns non-zero when `d` exists) declared as a
plain establish is an **oracle bug** (16D oracle-degradation, blast-radius-bounded),
or it carries an explicit rc-**bridge**. There is no analyzer "converged ⇒ non-zero"
reasoning and no "status-liveness because mkdir" case. *(This corrects the
RC-value-leaning framing in 16C §1/§2 and 16E §3a/§4.)*

## 5. The decision is undecidable; the floor is the conservative surrogate
The precise predicate — *does a consumer behave differently given the stub-default
than given the real value?* — needs the real value (no value-plane to synthesize it,
16C brk-1) and the consumer's opaque semantics. **Undecidable.** We don't solve it;
we over-approximate with a decidable, structural surrogate: *the observable is
consumed in a value-bearing position.* Sound (kFAIL / 16D conservative fold); it
costs only replacements — e.g. a no-output command whose `""` is compared to `""`
(real == default, benign) is needlessly run. Unsolvable-precisely,
soundly-approximable — the standard Dorc cap, not a blocker.

## 6. Corrections to 16C / 16E (recorded by reference, not edited)
- 16C "× {stdout, rc}" bridge: the **rc half is the rare escape-hatch**
  (converged-non-zero tools); the **stdout half is the real new capability**.
- 16E §4 "backward result-liveness = stdout + status": **status is not an analyzer
  obligation** (stub + establish-vouch handle it); the obligation is
  **stdout/stderr-liveness only**.
- 16E §3a "status-liveness bites when converged⇒non-zero": that is an
  oracle-contract matter (§4 here), not an analyzer analysis.
- 16B/16C "are `$()`-internal commands leaves?": under the observable model they are
  leaves like any other; the `expansion_internal` exclusion is a correct *temporary
  floor* (they cannot be value-bearing-consumed-safely until the subst-offset fix +
  bridge land, so MustRun-equivalent is right for now).

## 7. In-spike floor (small) vs outside-spike enrichment (large)
**IN-SPIKE — sound today, the bounded next step:** add a **stdout/stderr
observable-liveness** obligation to `prove_replaceable`, conservatively: *a leaf
whose stdout or stderr is consumed in a value-bearing position ⇒ MustRun* (no bridge
in-spike), beside the existing forward effect-convergence gate and the free
status-vouch. This closes the live mis-replacement (`apt-get install x | tee log`,
`x=$(apt-get install …)`); add the regression. Do the skip→replace /
channel→observable rename while there. **That is the whole in-spike step.** Do
**not** build status/rc analysis (that is the §4 smell), the bridge, or `Grounded<T>`.
**OUTSIDE-SPIKE — design-corpus:** the oracle `bridge` (gather/compute) that
*discharges* a consumed stdout so a leaf can be replaced anyway; `Grounded<T>` (16D
typed best-effort under degradation); cross-host. Parked for the human's go/no-go:
is the recovered replace-rate worth a dn-1 contract that large?

**NOTES INDEX:** …16C skip-and-substitute · 16D degradation lens · 16E state/CFG
read-write model · 16F (this — the observable/replace model + the stdout-only floor;
corrects 16C/16E; bans "skip").
