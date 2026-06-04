# notes/127 — round-12 synthesis handoff (corpus map · must-surface index · decided/deferred ledger · quarantine)

**Stamp:** round 12 · 2026-06-03 · the single entry-point for a cleared-context agent writing the round-12 conclusion (`references/final-synthesis-and-conclusion.md`). Everything propagate-worthy is in the notes below; this file indexes it and adds the quarantine list the conclusion step requires.

## Corpus (read these; the conclusion synthesizes from notes, not raw sources)
- `plans/121` — the frozen round-12 map: gate-decision, axes (`axis-dst-cost`/`axis-quadrant`/`axis-infer`/`axis-platform`), the 3 fronts, the `concl-*` callouts, the agentic goose/gander lens. **Do not rewrite it.**
- `notes/120` — broad-sweep prior art (f1–f13 + r12 human-sharpening pointers on f3/f9/f13). The tier-axis, DST-as-highest-tier, idempotency-oracle, CAP third-state, counter-thesis, agentic-greenfield reframe.
- `notes/122` — DST from first principles (the explainer).
- `notes/123` — Rust DST ecosystem + the transitive-dep wall (f14–f23): madsim cfg-swap, turmoil, shuttle, sled/Polar-Signals state-machine, moonpool provider-trait; the three architectures; libc-narrow-waist + `[patch]`.
- `notes/124` — DST seam ↔ Dorc mapping + the `axis-dst-cost` ladder (f24–f30) + **human verdict / open Qs**.
- `notes/125` — containerizability quadrant, infer-vs-annotate (f31–f38, incl. the f38 human corrections).
- `notes/126` — transient-fault / error-handling (f39–f44): Jepsen `:ok/:fail/:info`, `concl-jepsen`, Sometimes-assertions, the seam-subsumes-fault-injection thesis.
- `sources.json` — 148 graded sources (33 new this round; round-12 slugs grep: tigerbeetle|jepsen|madsim|risingwave|sled|polarsignals|s2-dst|turmoil|shuttle|moonpool|bazel|saflate|pytest-test-categories|kselftest|k8s-e2e|antithesis|fdb-sigmod|metr|coplien|rainsberger|hughes|google|fowler|terraform|hashicorp|litmus|beaker|test-kitchen|molecule|testcontainers|eatonphil|warpstream|etcd|tc-netem|scylladb|foundationdb).

## Must-surface in the conclusion (the answer-shape; pointers, not a re-dump)
The 4 tagged callouts in `plans/121` — **`concl-quadrant`** (FRONTLOAD: the containerizable-vs-not boundary is hand-annotated by everyone; nobody statically infers it; academic frontier infers only the network sub-axis, dynamically), **`concl-rigor`** (best-effort = provable-grade rigor, NOT an escape-hatch; ceilings the *edge*, never the *kernel*), **`concl-tdd`** (posture: types + contracts/boundary + fuzzing + tests-first spanning unit/integration/types; coverage-maximization is the antipattern), **`concl-jepsen`** (prior-art-to-mine, optionally a slow real-cluster CI tier with a *custom* checker; NOT the hot loop; its consistency-checkers don't fit Dorc) — PLUS these load-bearing ones not tagged `concl-*`:
- **DST is unusually cheap for Dorc** (notes/124 verdict, notes/123): Rust's swap story is zero-prod-cost; the async/single-thread/inverted-control discipline is the dev's native style (Menhir-IoC parallel), not a tax; the transitive-dep wall is *smaller* than for a DB (near-pure analyzer kernel; the remote host is a mocked seam, not a crate). Residual cost ≈ one upfront seam choice + crate-gating that bites only kernel-reachable deps.
- **The `dorc_exec(host, leaf)` chokepoint is triple-use** (notes/124 f27): prod wrap-and-spawn (`plans/077`) · trace point (seccomp/`DORC_LEAF_ID`) · DST/fault substitution. Reserving it cleanly is the *day-1 bank* and the only retrofit-hostile piece — process-level (`plans/077`) and in-process-DST seams meet here.
- **`axis-dst-cost` ladder + the f26 fork** (notes/124): L0 reserve-the-seam (cheap, bank now) · L1 in-process orchestrator sim (sweet-spot *if* the orchestration carries real complexity) · L2 full DST (overkill) · L3 Antithesis (defense-in-depth, later). The fork the human must weigh: *how much of Dorc's correctness lives in cross-host coordination (DST-testable) vs at the mocked edge (oracle territory)?*
- **Fronts A & C unify at the seam** (notes/126 f42): synthetic adverse outcomes at `dorc_exec` are the portable hot-loop fault test; real netem/iptables/Jepsen test the transport + real tool = mostly the mocked edge → slow CI complement.
- **Agentic = goose/gander lens** (notes/120 f7): front-load the test/verification backstop because greenfield agentic velocity manufactures epistemic debt; the literature (METR 19%-slower; "agent only as good as your test infra") *vindicates* the worry. Don't silo as "testing for AIs."

## Decided / deferred / parked / open ledger (state NOTHING as decided that isn't here under DECIDED)
- **DECIDED (explicit, human):** the 3 fronts (A/B/C); oracle-layer **parked**; DST-ambition **deferred**; agentic = goose/gander **lens** (not a silo); the ID convention (`f`-numbers + `axis-*`/`concl-*`); plan frozen & conclusions kept separate.
- **LEAN (provisional, NOT a decision):** heavily pro **in-process-DST-discipline** (bank L0 + write the kernel async/IoC) — defer whether to climb to L1+. Language **leaning Rust** (not decided).
- **DEFERRED (decision postponed to pre/post-conclusion):** the `axis-dst-cost` rung (how far up L0→L2); whether to name "infer a *user leaf's* containerizability from oracle-effects" as a Dorc differentiator (Object B); whether to *use* Jepsen/Maelstrom as a CI tier.
- **PARKED (late-stage, needs a Thing + Users):** the oracle layer / testing-for-oracle-authors; idempotency-as-product; the privileged tracing tool (`plans/078`). `plans/077`'s seccomp is a *product* DX tool for oracle authors (Linux-only), **not** an already-banked self-test primitive.
- **OPEN (unresolved facts that gate later design):** transport model — "SSH a script over" is a *starting point*, an agentless-temporary-executor-phones-home model is live (if it wins, madsim/turmoil apply directly); orchestrator ownership — Dorc's own vs built-on-pyinfra (if delegated, the DST-testable kernel shrinks).

## Quarantine (interesting, correctly not chased — triage as TODOs)
- TCP/IP-stack QuickCheck (París & Arts, Erlang Factory 2009 slides) — open copy found, **unread**; corroborating only (PBT-for-networked-systems already carried by [A-hughes-dropbox-pbt-2016]).
- Will Wilson "Testing Distributed Systems w/ Deterministic Simulation" (Strange Loop 2014) — video; superseded as written form by [A-fdb-sigmod-paper-2021].
- A push/agentless-adjacent Jepsen analysis (closer to `kAGENTLESS`) — not located/read.
- Molecule Part-2 (converge→idempotence→verify) — 404'd; needs a working mirror / web.archive.
- Terratest per-topic pages (Idempotent, Cleanup) — unread (the assigned page was a stub).
- `shuttle`/`loom` — shared-memory concurrency-schedule testing; a *different* altitude from network DST; relevant only if the orchestrator has shared-memory concurrency.
- `jepsen.antithesis` bridge — the Jepsen/Antithesis lineages converging; noted, not explored.
- The "is my DST exploring the state space?" problem — Sometimes-assertions (notes/126 f41) are the reachability *half*; coverage remains the unsolved hard part (FDB/Will-Wilson: "resembles science more than engineering").
- Agent-network-sandbox tooling (sandcat, NVIDIA OpenShell) surfaced in Front-B search — orthogonal (sandboxing AI agents), not chased.

## Toolchain note for the next agent (hard-won)
`jq` is not on PATH in git-bash on this box; the skill scripts' `mise exec -- sh` self-relaunch does NOT work here. Prefix Bash invocations with: `export PATH="/c/Users/ec/AppData/Local/mise/installs/jq/1.8.1:$PATH"` then run `sh /c/Users/ec/.claude/skills/interactive-research/scripts/{validate,new-source}.sh Research …`. `curl` is at `/mingw64/bin`. Register sources via the file+loop pattern (write `slug {json}` lines, loop `printf '%s' "$json" | sh new-source.sh Research "$slug"`) to dodge shell-quoting on apostrophes. Git: main tree, branch `ai/snapshot`, all round-12 work uncommitted, read-only git per user.
