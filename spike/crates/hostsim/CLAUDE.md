# spike/crates/hostsim — CLAUDE.md

The seeded, deterministic DST host-model + the `kFAIL-withhold` monitor. Read `spike/CLAUDE.md` and `Research/plans/190-spike2-keystone-charter.md`.

Keep this the sole home of nondeterminism (`inv-determinism`); the kernel it answers stays pure (`an-pure-kernel`). The kernel crates (`syntax`/`analysis`/`plan`/`core`) depend on *none* of this — that's what lets the whole pipeline run under DST with no DI ceremony (`an-di-seams`: this crate *is* the one DI seam; the kernel never reaches a clock/RNG/net/disk transitively). DST over seeds is the regression backbone — keep it green as the entity-algebra changes (`ap-1`: verdicts become per-selector, not one bit per kind/entity).

## What this crate is, in the 128 model

This is the in-process stand-in for **Seam-1**, the controller↔host transport (`128 concl-seam`/`se-1`): in simulation you *synthesize* the host's answers from a `u64` seed; you never spawn `ssh`/`apt`/`docker`. +SURE the real opaque host is "the mocked edge by design" (`128 se-2`) — `hostsim` answers fact-probes, it does not run sh. The three-valued verdict (`Converged`/`Diverged`/`Unknown`) is Jepsen's `:ok`/`:fail`/`:info` as the native unit (`128 fc-2`); `Unknown` is the kernel's own fold for an un-probeable/unreachable fact, not something a *reachable, modeled* host emits today (`Host::verdict` returns only Converged/Diverged — see the doc-comment on `verdict`).

## The two jobs (built — `16P-T15`)

- **Answer fact-probes** against a modeled system-state — the concrete `verdict_of` the plan stage injects (`Host::verdict`). `Converged` iff the fact holds.
- **The `kFAIL-withhold` monitor** (`an-withhold-monitor`, `16P-T15`/`DP-4`): a read-only probe that tries to `Establish`/`Kill` in `Phase::Probe` is RECORDED and REFUSED — host state is unchanged (`Host::run`). In `Phase::Apply` the mutation applies.

~SUSPECT the most load-bearing line in this crate is the DP-4 caveat: **the monitor is a DST stand-in, NOT a sandbox.** `an-withhold-sandbox` (real seccomp/sandbox enforcement) is a *separate, unbuilt* mechanism, and the contract frame provably *cannot* enforce probe-inertness on its own (`16P-DP-4`). Do not let a green monitor read as "probes are proven inert" — it proves the *modeled* op was refused, nothing about a real host. (`an-reflexive-inertness` — running the effect-analyzer over a lifted probe body to flag a cmd another oracle mutates — is the cross-oracle backstop, also out of this crate.)

## The probe model — speculate-and-intercept (`17O R2-PROBEGATE`)

Oracles ship an interceptor that *replaces* the real command (an `id__check` ships and replaces `id`); a probe-gated branch is resolved by running the read-only probe **for real** — unlike Ansible check-mode, which is blind past a register-gated `when:`. The probe is compiled from oracle bodies + minimal CFG fragments, never the book's contents, so it never inherits the book's ambient `trap`s. Model that here as the host answers a *probed* fact; an un-probed fact must surface `Unknown` upstream (`can't-probe ⇒ can't-elide`, `an-elision-predicate`), never a synthesized Converged.

## The keystone re-key ripples *into this crate* (`ap-1`)

The structured entity-algebra (charter §3) re-keys `FactKey` — today `{kind: KindId, entity: OpaqueToken}` (`analysis::effect`), tomorrow selector-bearing (`package:nginx#installed` vs `package-index#fresh`; that distinction is the poison-wall fix). `Host`'s fact-store and `verdict`'s parameter ride that change. Two musts as you thread it:
- ~SUSPECT a selector-keyed fact-store wants strong/weak-update semantics (`an-strong-weak-update`, `notes/180` fnd-4) to *model* mutation faithfully — but keep the host a plain set-membership oracle unless a test genuinely needs more; the host models *effects*, not real sh.
- +SURE do not let the re-key churn reintroduce nondeterminism: no `HashMap`/`HashSet` iteration where order is observable; the LCG (`Lcg`, the hand-rolled 64-bit LCG — no `rand` dep) stays the only entropy. `Host::seeded` must remain bit-for-bit reproducible from its seed.

## Extending the fault-space (`an-host-fault-model`, status D — reach for it as the spike needs)

Today the PRNG drives *only* initial host state (`16P-§3.2`). The injectable per-host fault space to grow toward (`128`/IMPLEMENTATION): **unreachable / timeout / wedged-read-only / truncated-stream / forged-verdict**. Discipline when you add them:
- Model the *outcome*, not the kernel mechanism (`128 fc-5` / `axis-platform`): a synthetic `Unknown`/drop at the seam, not `tc`/netem packet-loss. That's what keeps it hot-loop, all-OS, deterministic.
- **forged-verdict** is `an-host-as-adversary` (§L, status O): a managed host forging `Converged` → silent suppression of a needed apply. The kernel's defense is `kFAIL-perform` (`an-verdict-failsafe-default`: default failed/unknown until finalized); this crate's job is to *inject* the forgery so a test proves the apply still runs (or that the elision was licensed only by a `Must` fact, `inv-must-may`).
- **probe-flakiness** (`an-probe-flakiness`, D): probe returns `Unknown` with seeded probability / host transiently unreachable — model unreliable oracles. Use `Lcg::chance` for the seeded coin.
- Every fault path wants a **`an-sometimes-assert`** (`128 fc-5`): assert the drop-after-mutate / `Unknown` path *is* reachable, so a sim that never exercises it fails loudly. (This is only the reachability half — coverage stays unsolved; inherit the humility, `128 rg-1`.)
- **`an-replay-seed`** (`128 L0`/`16P-T15`): a failing seed (+ commit) must deterministically reproduce. This is the single highest-value agent-feedback signal — surface the seed in every DST failure message (the existing tests already interpolate `seed {seed}`; keep that).

## Honor

`inv-determinism` (this crate is the *only* sanctioned exception, and only because seeded); `inv-no-unsafe`; `inv-no-throw` (the host is total — no panic on a malformed op); `inv-kfail` (the monitor *is* `kFAIL-withhold` made observable). Mark uncertainty (`+SURE`/`~SUSPECT`/`-GUESS`/`--WONDER`) in `notes/19x-*`; the deliverable is *where the host-model strains*, not green tests.

## A tension to flag, not resolve

The seed-fuzz scenario generator and the fault-injector pull opposite ways on **fidelity vs. coverage**, and it bears on `ch-priority` #2 (correctness/taint) vs the `seam-*` deliverables. The richer the host model gets — selector-keyed facts, strong/weak update, the five-way fault space, transient flakiness — the closer it edges to *re-implementing a real host's state machine inside the simulator*, which (a) grows a second source of modeling-bugs that can mask or manufacture analyzer bugs (`128 rg-1`: a weak oracle gives false confidence — ScyllaDB's "can we read?" passing while nodes were broken), and (b) costs exactly the maintainability/simplicity the spike-wide "be boring" rule prizes. Against that, the charter's leading goals (`an-host-as-adversary`, the fault model) are *success-criteria* — "it works on the easy host states" is the `ap-1`/`ap-2` self-confirmation trap spike-1 fell into. --WONDER where the line sits: how much host-state realism is *needed to strain the analyzer at the seams*, versus realism that just re-models sh and buys false confidence. I haven't resolved it; surface it to the human when the fault-space build forces the call.
