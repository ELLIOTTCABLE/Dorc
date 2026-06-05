# 170 — spike direction: the apply phase + multi-host state (the missing half)

> **Status (2026-06-05): spike, direction note (planning, not implementation).**
> The spike has built the **probe/plan half** (analyze a book → decide skips → emit
> a plan). The human flagged that two load-bearing things were absent from the
> direction docs: the **apply phase** (actually mutating hosts) and **multi-host
> state** (a fleet, with CAP-flavoured failure modes). This note works them into
> the arc and records how they shape the *existing* pieces — so the build doesn't
> paint itself into a corner. Append-only; nothing here is built yet. Confidence-marked.

## 0. This round's cleanup (the lead-in)
Before this note, a cleanup pass on the cli-capstone findings (note 169):
- **find-cli-4 FIXED (`a8dd3c7`):** the errexit pass seeded merges with `Off`, and
  `Off ⊔ On = ⊤` (they are incomparable atoms), so any merge after a branch/subst
  spuriously went ⊤ — it bit the common `case $(hostname)` host-selection idiom.
  Now merges seed a real `⊥` (`⊥ ⊔ On = On`); genuine `set +e`/`set -e` splits still
  go ⊤. Two regression tests. **This is a prerequisite for sound apply** (§1): a
  spurious failure-edge is unsound *backward* (note 166 find-8), and apply rides the
  backward slice.
- **find-cli-2 (BOM): won't-fix** — an encoding edge-case the synthetic suite simply
  avoids; not worth lexer complexity (the human's steer).
- **find-cli-3 (heredoc span): deferred into the find-cli-1 leaf round** — it is the
  same "a leaf's text isn't one `[lo,hi)` slice" question; fixing it piecemeal now
  would be thrown away when the leaf model is redefined.

## 1. The apply phase — the second soundness, welded opposite (`kFAIL-perform`)
The kernel already carries the *vocabulary*: `core::Phase::{Probe, Apply}`, the
welded `kFAIL` (Probe⇒withhold, Apply⇒perform), `PhasedVerdict<Apply>`, the
`SkipLicense`. What's unbuilt is the **executor**: take a plan, run its
**un-elided leaves** (the mutators) against a host, in order, mutating it.
- **The fail-direction flips.** Probe never mutates; apply never *skips a needed*
  mutation. An un-licensed leaf RUNS; only a `SkipLicense` elides. `Unknown` at
  apply-time ⇒ run (the `Bias` for `Apply` already encodes this).
- **Apply consumes the BACKWARD analysis** (note 163 §2): the apply-minimization
  slice is backward from the dirty set — and `solve` is already direction-generic
  (the day-1 requirement). This is where errexit precision (§0) becomes
  load-bearing: a spurious `cmd→exit` edge makes the backward slice think a
  downstream mutation is conditionally bypassed ⇒ a wrong skip under
  `kFAIL-perform` (note 166 find-8). Forward-only was tolerant of spurious edges;
  backward is not. The §0 fix clears that.
- **Re-probe before apply (note 165 L4 / TOCTOU).** The plan was built from
  probe-time verdicts; a fact Converged at probe but Diverged by apply-time means
  the `SkipLicense` must be *revoked* and the mutation run. Apply re-confirms at the
  irreversible boundary; `hostsim` already supports a fresh `verdict` read.
- **`hostsim` already models the mutation side**: `run(Phase::Apply, HostOp::Establish/Kill)`
  applies; `run(Phase::Probe, …mutate…)` is the kFAIL-withhold violation. The apply
  executor is "run the plan's leaves as `HostOp`s against the host"; against a real
  host (cli) it ships sh, against `hostsim` (DST) it drives the state machine.
- **DST tests apply unlocks:** apply a plan to a seeded host ⇒ assert it reaches the
  desired state (facts established); **idempotence** (re-apply ⇒ all-skipped, no
  HostOps mutate); a mutative probe is caught; a revoked-at-TOCTOU skip re-runs.

## 2. Multi-host state — fan-out + the CAP-flavoured hazards
The spike is single-host; the tool fans a plan across N hosts (push, agentless —
`kAGENTLESS`). The DST model is **N seeded `hostsim`s** + a deterministic scheduler
— the fan-out's *shape* without committing the real transport (the kCOMMS/kCONC
executor fork stays deferred/plan-and-gate, notes 128/142).
- **Plans are host-relative.** Each host has its own state, so `classify` is shared
  (static, host-independent) but the *verdict* and thus the plan are per-host
  (`build_plan` already takes injected verdicts ⇒ per-host verdicts ⇒ per-host
  plans). Provenance reconstructs controller-side per host (note 110
  ch-controller-side; hosts stay dumb).
- **CAP, concretely** (the human's ask — the DST failure-modes to test):
  - *Partition / unreachable host:* its verdicts are `Unknown` ⇒ kFAIL folds to
    "don't skip; state unknown." The host's apply is marked **incomplete**; the rest
    of the fleet proceeds (availability over a stuck node). DST: flip a host to
    unreachable mid-fan-out, assert its mutations are withheld/incomplete and others
    finish.
  - *Consistency / cross-host shared state* (note 099 W5-D5 write-skew, e.g. an LB +
    its backends): facts are **host-local by default** (W4 referent-agnostic — a
    package on host A ≠ host B). Cross-host coordination is a *separate, harder*
    layer (an oracle declaring a cross-host kind); **reserve the seam, defer the
    build**. Most of the value is embarrassingly-parallel host-local work.
  - *Availability / best-effort:* apply to reachable hosts, report per-host
    outcomes, never block the fleet on one slow/dead host (DESIGN's best-effort).
- **DST tests multi-host unlocks:** a fleet of varied seeded hosts; per-host
  convergence; injected partitions (Unknown ⇒ withhold, others proceed); a
  cross-host write-skew is either absent (host-local facts) or surfaced where an
  oracle declares a cross-host kind.

## 3. Unreliable + mutative oracles (DST) — the host's adversarial side
- **Unreliable:** a probe that times out / errors / flakes. `hostsim`'s seeded PRNG
  (currently used only for initial state) should also model **probe flakiness** —
  a verdict is `Unknown` with seeded probability, or a host is transiently
  unreachable. The kernel already folds `Unknown` conservatively; DST asserts a
  flaky probe *never* yields a wrong skip (Unknown ⇒ run).
- **Mutative:** a probe that violates kFAIL-withhold. `hostsim` already detects this
  (`run(Probe, Establish/Kill)` ⇒ `Violation`); the future seccomp/sandbox is the
  real-host analog (note 162 DP-4). DST asserts a mutative probe is caught + the run
  flagged — and that the contract frame alone can NOT prevent it (only the
  host/sandbox can), the standing finding.

## 4. How this shapes the existing pieces (so we don't corner ourselves)
- **`hostsim`** grows: single fact-store → (a) apply-mutation [have it], (b) seeded
  flakiness [PRNG, latent], (c) a **fleet** (`HostId → Host`), (d) partition. Keep
  it the *only* nondeterminism home.
- **`plan`** stays the per-host decision; a NEW **apply executor** runs a plan's
  leaves as `HostOp`s (DST) or sh (cli), with re-probe-before-apply.
- **`Phase::Apply` + the backward slice**: build with apply; `solve` is ready
  (direction-generic), errexit is now precise (§0).
- **The leaf-seam (find-cli-1, the NEXT round) is a prerequisite for apply** — apply
  *executes* leaves, so "what is a runnable leaf" (top-level/branch/subshell yes;
  `$()`-internal no) must be settled before you can run them. Good ordering falls
  out below.

## 5. The arc (ordered; none built yet beyond §0)
1. **find-cli-1 — leaf-seam scope** (next): define a plan/apply leaf precisely;
   prerequisite for faithful plans AND apply.
2. **apply executor (single-host):** run un-elided leaves as `HostOp`s vs `hostsim`;
   re-probe-before-apply; idempotence + mutative-probe DST.
3. **multi-host fan-out (DST):** fleet of seeded hosts; per-host plans; partition /
   CAP-flavoured failure-mode tests.
4. **unreliable-oracle DST** (PRNG flakiness) woven with the existing mutative-probe
   detection, across the fleet.
- Throughout (the human's "towards the end"): the big **test-suite expansion** over
  input × host-state × fleet × failure-mode. The §0 errexit fix + the leaf round
  are what let realistic books be tested without spurious noise.

**NOTES INDEX:** …167 effect adversarial review · 168 round-16 build summary · 169
cli capstone findings · 170 (this — apply + multi-host direction).
