# spike/crates/hostsim — CLAUDE.md

Role: the seeded, deterministic DST host-model + the `kFAIL-withhold` monitor —
the ONLY sanctioned home of nondeterminism (seeded, injected), and the one DI
seam; the kernel crates depend on none of this. Read `spike/CLAUDE.md` first.
Registry discipline: one rule per bullet, slugged; append to the matching
section.

## Law

- **monitor-not-sandbox** (the most load-bearing line in this crate) — the
  withhold-monitor is a DST *stand-in*, NOT a sandbox: a green monitor proves
  the MODELED op was refused, nothing about a real host. Never let it read as
  "probes are proven inert" — probe-inertness comes only from the structural
  self-vouch; real enforcement is a separate, unbuilt mechanism.
- **answers-facts-never-runs-sh** — hostsim answers fact-probes against modeled
  state; it never spawns `ssh`/`apt`/`docker`; the real opaque host is the
  mocked edge by design (Seam-1, the controller↔host transport).
- **unknown-is-the-kernel-fold** — an un-probed/unreachable fact must surface
  `Unknown` upstream (can't-probe ⇒ can't-elide); never synthesize a
  `Converged`.
- **lcg-only-entropy** — the hand-rolled LCG is the only entropy (no `rand`
  dep); `Host::seeded` stays bit-for-bit reproducible from its seed; no
  hash-iteration where order is observable.
- **host-models-effects-not-sh** — keep the host a plain fact-store unless a
  test genuinely needs more: a rich host model is a second source of
  modeling-bugs that can mask or manufacture analyzer bugs.

## Fault-space discipline (grow it as the spike needs it)

- **model-the-outcome** — inject the outcome, never the kernel mechanism (a
  synthetic Unknown/drop at the seam, not `tc`/netem) — that keeps it hot-loop,
  all-OS, deterministic.
- **forged-verdict** — the host-as-adversary case: inject a forged `Converged`
  and prove the apply still runs / that any elision was `Must`-licensed
  (`kFAIL-perform` is the kernel's defense; this crate's job is to attack it).
- **probe-flakiness** — seeded `Unknown`/transient-unreachable models unreliable
  oracles; use the seeded coin.
- **sometimes-assert** — every fault path asserts its own reachability, so a sim
  that never exercises it fails loudly (reachability half only; coverage stays
  unsolved — inherit the humility).
- **replay-seed** — a failing seed (+ commit) must deterministically reproduce;
  surface the seed in every DST failure message (the single highest-value
  agent-feedback signal).
- **sigpipe-race-injection** (`279f` rider) — inject the pipefail/SIGPIPE
  early-exit race so rc-141 sink-landings cannot flap goldens.

## Direction

- **re-key** — the fact-store and `verdict`'s parameter ride the flat
  `(kind, entity, selector)` coordinate + context slot; verdicts go
  per-selector.
- **context-qualified-injection** (`plans/27C` §3/§9) — coming consumers:
  context-qualified verdict injection and two-context e2e fixtures under inert
  mocks (a wrapped site's probe is answered in the site's DENOTED context, not
  the ambient one).

## Tension (flag, don't resolve)

- **fidelity-vs-coverage** — the richer the host model (selector-keyed facts,
  fault five-ways, flakiness), the closer it edges to re-implementing a real
  host inside the simulator: false confidence + maintainability cost, against
  the charter's adversarial-host success-criteria. Surface where the line
  bites; the human decides.
