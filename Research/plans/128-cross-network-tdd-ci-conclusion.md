# plans/128 — cross-network TDD / CI — round-12 conclusion

Round 12, 2026-06-03. **The round's conclusion.** Synthesized from the front notes
(`notes/120` broad sweep; `notes/122` DST explainer; `notes/123` Rust ecosystem; `notes/124`
seam-onto-Dorc; `notes/125` containerizability quadrant; `notes/126` transient-fault) and the
human's direction recorded in `notes/127`'s ledger. The round-12 map (`plans/121`) is frozen and
**not** rewritten here. 33 sources registered this round (148 total, `../sources.json`); the DST /
Jepsen / IaC-self-test / tier-theory clusters were subagent-graded on the broad sweep and
**re-verified on full read** when narrowing (notes/122–126 regrade headers).

> **Decision discipline.** This document states as *decided* only what `notes/127`'s ledger lists
> under DECIDED. The big correctness-architecture call (how far up the DST ladder) is **deferred by
> human dictate**; the pro-in-process-DST stance is a **provisional lean**, not a commitment; the
> language is **leaning Rust**, not chosen. Where this matters it is marked inline.

## 0. The answer, up front

The sweep converges on a coherent, decades-tested practice-set; the variance is *how far Dorc
commits*, not *what the practices are* (`plans/121` landscape). Three results carry the round:

- **concl-seam — Fronts A and C unify at one chokepoint, and reserving it is the only decide-now
  piece.** `dorc_exec(host, leaf)` is **triple-use**: production wrap-and-spawn of the real program
  (`notes/077`/`plans/077`), the trace/attribution point (`DORC_LEAF_ID`), and the DST/fault
  *substitution* point where a simulator returns a synthetic outcome (ok / timeout /
  drop-after-mutate / partition / reorder) instead of spawning anything (`notes/124` f27). +SURE.
  Because `plans/077` already needs this one indirection for tracing, reserving it *cleanly* —
  a trait/message boundary, with the orchestrator kept free of direct clock/rand/IO — **also**
  reserves the DST seam at ~nil extra cost. This is the single retrofit-hostile, day-1 item; DST is
  "design-it-in-or-never" [B-antithesis-dst-primer-2024]. **The provisional lean (human, not a
  decision) is to bank exactly this (rung L0) and write the kernel async/inverted-control;** whether
  to climb higher is deferred (§3).
- **concl-quadrant — the tier boundary that matters is hand-annotated by *everyone*; nobody infers it
  statically (FRONTLOAD).** §1.
- **concl-rigor — "best-effort" is maximal rigor, not relaxed rigor; it ceilings the *edge*, never the
  *kernel* (FRONTLOAD).** §2.

Everything else (the ladder rung, an inferred user-leaf-containerizability *product feature*, Jepsen
as a CI tool) is a **deferred** or **parked** decision the human will weigh later (§3, ledger §7).

---

## 1. concl-quadrant — the containerizability boundary (FRONTLOAD)

**The crux reframe (human-sharpened, `notes/120` f9 → `notes/125`): the boundary is not a 1-D
fast/slow line but a 2×2 — {fast, slow} × {containerizable, non-containerizable}.** The hard axis is
containerizability: can the behaviour be fully exercised inside a localhost network-of-containers
(*containerizable*: ssh to a box, ship a compiled probe, integrate results), or is it too "real"
(*non-containerizable*: reconfigure sshd and check the tunnel renegotiates/restores state)? +SURE this
seam matches **none** of Dorc's known seams — not invoked-tool, not POSIX-construct, not
domain-of-control (books *and* oracles hit it), not execution-time (a containerizable test can be
slow-but-deterministic; a non-containerizable one instant-by-network-standards) (`plans/121`
axis-quadrant).

**The empirical answer to the driving question — do shops *infer* this or *create+enforce* it? —
is: the network sub-axis is runtime-detected universally; full containerizability is hand-annotated
universally; nobody statically infers it from source.**

- **qd-1. The network-reach sub-boundary is enforced by runtime interception, never static
  inference — universally.** +SURE. Google's Test Sizes makes resource-reach a *mechanically policed*
  axis (Small = no net / Medium = localhost / Large = cross-machine; "it's possible to get the tests
  to police these limits") [A-google-test-sizes-2010][A-google-small-medium-large-2011]; Bazel's
  sandbox default-denies network and *fails* an untagged test that reaches it
  (`--sandbox_default_allow_network=false`) [A-bazel-test-encyclopedia-2024]; pytest-test-categories
  patches `socket.connect()` per declared size [B-pytest-test-categories-network-isolation-2025].
  Pattern: tier is hand-declared, violation is auto-detected at the syscall/socket layer (`notes/125`
  f31).
- **qd-2. The full containerizability boundary is hand-annotated everywhere.** +SURE. Kubernetes
  hand-labels environment-need via Ginkgo (`[Serial]`/`[Slow]`/`[Disruptive]`/`[Conformance]`,
  migrating to `framework.WithSerial`) [B-k8s-e2e-best-practices-2023]; kselftest self-checks "is this
  kernel feature present, else `$KSFT_SKIP`" [B-kselftest-docs-2026]. These encode "needs a real
  kernel/cluster/host a container cannot fake" and are *never inferred from source* (`notes/125` f33).
- **qd-3. The academic frontier infers only the *network* dependency, and only *dynamically* —
  because hand-annotation of it is the acknowledged weak point.** +SURE. `saflate`: "it is difficult
  or even impossible for engineers to write test fixtures or assumptions to reliably control or check
  the availability of networked resources," so it instruments network-exception sites and analyzes
  stack traces to *infer-and-skip-with-provenance* [B-saflate-network-assumption-inference-2022]. The
  transferable lesson is observe-the-effect, not static source analysis (`notes/125` f34).

**The two-object disentanglement (human, `notes/125` f38 — do not conflate):**

- **Object A — how Dorc tiers its *own* tests.** This is **Rust**, tiered by **Rust** annotation (test
  attrs / `cfg` / nextest filters — the Bazel-tag / pytest-marker / K8s-label analog), infer-answer
  *mostly-no, like everyone*. +SURE. The portable low-tier enforcer is the **DST seam itself**:
  synthetic outcomes mean no real socket opens, so "no network" holds *by construction on every OS*.
  A Linux-only seccomp escape-detector can sit in CI as a backstop, never in the hot loop
  (`axis-platform`). This half is **conventional**.
- **Object B — inferring a *user* sh leaf's containerizability** (effect/taint analysis over user sh +
  oracle-effect-knowledge; "an oracle for `sshd`/`docker` knows which of its effects are
  non-containerizable," spelled in `sh` per `kOOB`, never a metadata tag). ~SUSPECT this is Dorc's
  *only* lever beyond hand-annotation, and it is the **genuine-novelty bet** — but it rests on the
  oracle layer (**PARKED**), and even the frontier manages only *dynamic* inference of the *easy*
  (network) axis. Whether to name Object B a Dorc differentiator is **DEFERRED** (`notes/127`).

> **Correction banked (human, f38):** an earlier pass overclaimed that `plans/077`'s seccomp was an
> "already-banked self-test primitive." It is not. `plans/077` seccomp is a **product** DX tool that
> detects network in the executed *user leaf* — Linux-only, scoped for oracle authors. Using seccomp
> to tier Dorc's *own* Rust suite is a *different*, Linux-only application → CI-only. For Dorc's own
> tests the primary, portable enforcer is the DST seam (qd, Object A).

**Why this leads.** It is unintuitive, cross-cuts the known seams, and may need constant tracking
(`plans/121`). The honest position to carry forward: *network-reach is detectable today; full
containerizability is annotate-via-sh-effects with static inference as an oracle-dependent research
bet, proven fallback = hand/sh-annotation.*

---

## 2. concl-rigor — "best-effort" is maximal rigor (FRONTLOAD)

**The scary one (human-sharpened, `notes/120` f13 → `notes/124`): "best-effort" is NOT an
escape-hatch.** For Dorc it means *provable-algorithm-level rigor* — "best-in-the-universe effort" —
so that the **user** needn't be correct. The soundness ceiling exists only to (a) bound *dangerous
algorithmic assumptions* and (b) honestly acknowledge the user-facing imperfection that arises from
the day-1 oracle contract. Testing rigor must be **maximal**, never relaxed by appeal to "we're
best-effort."

This is not in tension with the universal prior-art humility — it *is* it, read correctly:

- **rg-1. Every high-correctness practice here is testing-not-proof and says so.** +SURE. Jepsen: "we
  can prove the presence of bugs, but not their absence" [A-jepsen-etcd-3-4-3-2020]; DST is
  "terrifyingly easy to build [such that it] appears to be doing a ton of testing, but actually never
  explores very much of the state space" and seeds break on code change
  [B-eatonphil-dst-bigdeal-2024]; FoundationDB simulation "is unable to test third-party libraries or
  dependencies" [A-fdb-sigmod-paper-2021]. Dorc inherits the same bug-finding (not bug-proving) frame
  `DESIGN.md` already commits to, and `KNOBS` already welds (`kVERIFY-calibrate`: "TypeScript, not
  Coq").
- **rg-2. The ceiling falls on the *edge*, not the *kernel* — and that maps cleanly onto Dorc's two
  zones.** +SURE. DST tests the orchestration *logic* to provable-grade rigor; it can only **mock**
  the opaque external program and the real remote host — "you must mock out the external systems"
  [B-eatonphil-dst-bigdeal-2024], the same edge FDB could not cross [A-fdb-sigmod-paper-2021]. That
  mocked edge is *exactly* the day-1 oracle contract's territory and the non-containerizable region of
  §1. So `concl-rigor` is operational, not aspirational: **kernel gets provable-grade rigor; the edge
  is honestly best-effort, bounded by oracle quality** (`notes/122` §7, `notes/124` f25).

The practitioner middle-ground reinforces, not relaxes, this: broad integrated tests cannot cover the
path-space (Rainsberger's arithmetic — 100k integrated tests at ~50/sec = 34 min and a vanishing
path-fraction, breeding false confidence) [B-rainsberger-integrated-tests-scam-2009], and weak oracles
give false confidence even with great fault-injection (ScyllaDB: a "can we read?" checker passed while
nodes were broken) [B-scylladb-extending-jepsen-2016]. Maximal rigor means *the right* rigor at each
tier, not *more* tests.

---

## 3. The correctness-architecture decision — laid out, not made

**concl-seam, expanded.** The structural facts (Front A, `notes/124`):

- **se-1. Dorc's leaf seam is PROCESS-level; DST's seam is IN-process — the gap is the whole crux.**
  +SURE. `plans/077` wraps/spawns *arbitrary external programs* and remote hosts ("process-level, not
  in-process"); DST *requires* the system's own logic to run inside the simulator with every IO
  replaced by an in-process fake [A-fdb-sigmod-paper-2021]. You cannot run `apt` or a remote `sshd`
  "inside the simulator." They are seams at different layers that **meet at one decision point**
  (`dorc_exec`).
- **se-2. What DST CAN vs CANNOT test for Dorc.** +SURE. CAN: the orchestration kernel under network
  chaos — the multi-host `kSCHEDULE` DAG walk, partial-failure handling, retries, the plan/apply
  state machine, the three-valued verdict, "host 7 of 12 drops right after the mutation lands but
  before the ack." That is precisely Dorc's *differentiating* correctness vs Ansible. CANNOT: whether
  the opaque program actually worked, or real-host behaviour (the user's own sshd-renegotiation
  example) — the mocked edge (`notes/124` f25).
- **se-3. DST is unusually CHEAP for Dorc — the literature's headline cost mostly does not apply
  here.** ~SUSPECT (human verdict, `notes/124`; `notes/123`). Three reasons: (a) Rust's swap story is
  **zero production cost** — `cfg`-conditional shim crates, `[patch.crates-io]`, and libc-symbol
  overrides, all identical to the originals in a normal build [A-madsim-deterministic-simulator-2025]
  [A-risingwave-deterministic-sim-2023]; (b) the single-thread + async-IO + inverted-control
  discipline the literature counts as DST's main tax [B-eatonphil-dst-bigdeal-2024] is ~free for an
  async-native developer and is independently warranted for other Dorc components (Menhir-style IoC
  parser error-handling); (c) **the transitive-dependency wall is *smaller* for Dorc than for a DB**
  — the analyzer kernel is near-pure (AST→facts), and Dorc's most dangerous "dependency," the remote
  host, is not a crate to `[patch]` but **THE seam you mock by design** (`notes/123` f23, `notes/124`
  f30). The state-machine architecture (sled / Polar Signals — deps-as-messages, fault-injection in
  the bus only) sidesteps the wall entirely [A-sled-simulation-guide-2020]
  [B-polarsignals-dst-state-machines-2025]. Residual cost ≈ one upfront seam choice + crate-gating
  that bites *only* kernel-reachable deps.
- **se-4. `kFIDELITY` gains a third mode for free.** +SURE. `plans/077`'s optimized-vs-faithful
  becomes optimized / faithful / **simulated**; the faithful mode's "one leaf, one execution,
  control-flow preserved" discipline is the *same* 1:1 leaf↔decision mapping a DST sim needs — the
  anti-batching requirement and the DST seam want the same structural property (`notes/124` f29).

**The `axis-dst-cost` ladder (laid out for the deferred decision — NOT decided):**

- **L0 · Reserve-the-seam — the day-1 bank (provisional lean: yes).** Make `dorc_exec` a clean
  trait/message boundary (already required by `plans/077`) and keep orchestrator logic injecting
  clock/rand/IO. Cost ≈ nil beyond `plans/077`; buys DST staying *possible*; the only retrofit-hostile
  piece. Serves priorities #1/#2 with no maintainability tax.
- **L1 · In-process orchestrator sim — the plausible sweet spot, deferred.** A synthetic-outcome
  backend + seeded scheduler fuzzing drop/timeout/partition/reorder/crash across the multi-host DAG,
  asserting invariants after every step, printing+replaying the seed (sled-recipe-sized
  [A-sled-simulation-guide-2020]). Tests the differentiating correctness (se-2 CAN).
- **L2 · Full DST — likely overkill.** Control *all* nondeterminism incl. internal tokio/threads/
  transitive deps (libc overrides, single-thread+async everywhere). Heavy #1/#2 tax; marginal gain
  small because Dorc's edge is mocked anyway.
- **L3 · Buy a deterministic hypervisor (Antithesis) — defense-in-depth, cheap-to-add-later.**
  Containerize Dorc + fakes, run unmodified in the hypervisor [B-antithesis-dst-primer-2024]
  [A-etcd-antithesis-robustness-2025]. Explicitly *not* a day-1 concern (`plans/121` retrofit
  demotion).

- **se-5. THE FORK the human must weigh (adversarial, `notes/124` f26):** DST's payoff for Dorc ∝ how
  much genuine concurrency/ordering/failure-interleaving complexity lives in the **orchestrator**
  (DST-testable) vs at the **mocked edge** (oracle territory). For a database the kernel *is* the
  system; for Dorc a large share of "did it actually work" lives at the edge DST can only mock. So
  **DST is necessary-not-sufficient for Dorc**, and L1's value rests on `kAGENTLESS` single-controller
  multi-host coordination under partial failure being non-trivial — which it plausibly is. ~SUSPECT;
  this is the input to the deferred L0→L1+ rung decision, and it is **gated by two OPEN facts**
  (ledger §7): if the transport becomes an agentless-temporary-executor-phones-home channel, it is
  tokio-socket-shaped and madsim/turmoil apply directly; if the orchestrator is delegated (pyinfra or
  similar), the DST-testable kernel shrinks and se-5 re-resolves toward "less."

---

## 4. concl-tdd — the testing posture (Object A, the conventional half)

**Target posture (`notes/120` f6): static-typing-discipline + contracts/boundary-testing + fuzzing +
tests-first, spanning unit/integration/types conceptually. 100%-unit-coverage is an antipattern.**
+SURE. The shape is a deliberate *middle*: many fast deterministic tests that pin boundaries + a
*small* high-fidelity cross-network tier — neither "mock everything" nor "integrate everything"
[A-diverse-fantastical-shapes-testing-2021][A-practical-test-pyramid-2018]. Three load-bearing
qualifications:

- **td-1. The unit tier is meaningful for Dorc *specifically because* of oracles.** +SURE. Coplien:
  unit tests are near-worthless "unless you have an extrinsic requirements oracle for the unit under
  test" [A-coplien-unit-testing-waste-2014] — and a Dorc oracle is exactly that extrinsic oracle. The
  generic mock-heavy unit tier the counter-thesis derides is *not* what Dorc's oracle-anchored tier
  is.
- **td-2. Fuzzing ↔ property-based testing is the bridge, and PBT's model = Dorc's oracle/analyzer.**
  +SURE. PBT finds bugs by comparing two *independent* descriptions (impl vs a compact stateful model)
  — "the specification is the real weakness, not the testing" [A-hughes-quickcheck-fun-profit-2016];
  the Dropbox black-box model talking *only* via the filesystem found data-loss bugs in two
  synchronizers [A-hughes-dropbox-pbt-2016]. Dorc's oracle is the model; the analyzer is the second
  independent description (`notes/120` f12).
- **td-3. The IaC-world's near-free cross-network oracle — idempotency — is real prior-art but
  PARKED-as-product.** +SURE the technique (converge twice; second run must report zero changes) is
  the cheapest high-value cross-network check, and Litmus states it as near-free proof that desired
  state was reached *and* config is valid [A-puppet-litmus-concepts-2024]. **But** the human recontext
  (`notes/120` f3): idempotency-as-a-product-property is **PARKED**; idempotency *as a test oracle*
  (retry-safety under Unknown, §5 fc-4) is fair game now. Do not promote it to a day-1 design driver.

This whole section is **Object A / conventional** (§1). The agent-process lens (§6) raises its payoff
but does not change its shape.

---

## 5. concl-jepsen + Front C — fault testing, the three-valued verdict, "is the sim working?"

- **fc-1. concl-jepsen: prior-art-to-mine primarily; optionally a slow real-cluster CI tier with a
  *custom* checker; NOT the hot loop.** +SURE→~SUSPECT (`notes/126` f40). Jepsen's *shape* maps onto
  Dorc remarkably (a control node SSHes into target nodes running os/db/client/generator/nemesis/
  checker, partitions via `iptables`/`tc`) [A-jepsen-readme-architecture-2026][A-jepsen-net-clj-source-2026].
  Three blockers stop it being Dorc's tool of choice: (a) its built-in checkers are **consistency
  models** (linearizability/serializability via Knossos/Elle) — Dorc's correctness is "did the host
  reach desired state," a different oracle → you'd write a **custom checker**; (b) Clojure/JVM
  adoption cost (WarpStream: "none of our engineers had to learn Clojure") [B-warpstream-dst-entire-saas-2024];
  (c) it is the slow real-cluster tier, while the DST seam gives portable/deterministic/fast injection
  Jepsen cannot. Most reusable: the `:ok/:fail/:info` op model, the generator→nemesis→checker
  architecture, the fault catalogue, "prove presence not absence." Maelstrom fits *less* (Dorc is a
  controller, not a peer speaking a protocol over stdin/stdout) [A-maelstrom-distributed-systems-workbench-2022].
  **Whether to actually adopt Jepsen/Maelstrom as a CI tier is DEFERRED** (ledger §7).
- **fc-2. Jepsen's op model IS the three-valued verdict — independent convergence with round 11.**
  +SURE. Every completion is `:ok` / `:fail` / `:info` ("we're not sure"; a timeout/exception
  auto-converts to `:info`, "allow that it might or might not have taken place")
  [A-jepsen-client-op-model-2026] — i.e. True/False/**Unknown** as the *native unit* of a
  distributed-correctness harness, matching `notes/112` f50 / `plans/111`. **How Dorc tests its
  verdict:** inject a timeout / drop-after-mutate at `dorc_exec`, assert the orchestrator records
  Unknown, not a false ok/fail (`notes/126` f39). This is the CAP third state Dorc already committed
  to, and the seam is the natural place to *inject* and *assert* it.
- **fc-3. Fronts A and C unify at the seam (counter-thesis resolved).** ~SUSPECT→+SURE (`notes/126`
  f42). Synthetic adverse-outcome injection at `dorc_exec` is deterministic, portable, hot-loop, and
  tests the orchestrator's *logic*. Real netem/iptables/Jepsen only add value for *transport
  integration* (does Dorc's ssh-wrapping survive a real mid-stream drop?) and end-to-end assurance —
  which for Dorc is mostly the *mocked edge*, not Dorc's own code → a slow high-tier CI complement,
  not the primary. **Front C's apparatus ≈ Front A's seam + adverse outcomes + Sometimes-assertions +
  a custom checker.**
- **fc-4. Testing error-provenance under faults + the retry-safety hazard (bridge to round 11).**
  +SURE (`notes/126` f43). Inject a fault, then assert the resulting error carries correct provenance
  — `loc-host`, the leaf-ID, the Unknown verdict, the depends-on edge (`notes/110`–`113`,
  `plans/111`). And under Unknown a retry must **not** double-apply: test by injecting
  drop-after-mutate, retrying, asserting no double-apply (idempotency as a *test oracle* here, even
  though idempotency-as-product is parked — td-3).
- **fc-5. "Is my fault-injection actually exploring anything?" — the half-answer is
  Sometimes-assertions; coverage is the unsolved half.** +SURE on the mechanism, --WONDER on the hard
  part. A *sometimes-assertion* asserts a state IS reachable, so a sim that never exercises the error
  path fails loudly [B-antithesis-sometimes-assertions-2026]; Dorc should `assert_sometimes` the
  drop-after-mutate / Unknown path is hit. The Rust ecosystem's determinism guard (rerun the seed,
  panic/diff-TRACE-logs on divergence — madsim `MADSIM_TEST_CHECK_DETERMINISM`, S2's CI meta-test
  "down to the last bytes on the wire") proves *determinism* [A-madsim-deterministic-simulator-2025]
  [B-s2-dst-async-rust-2025]. But neither solves *coverage* — "this process often resembles science
  more than engineering" [B-eatonphil-dst-bigdeal-2024]. Inherit the humility (concl-rigor rg-1).

**The portable fault-injection ladder (`axis-platform`, `notes/126` f44):** seam-synthetic (hot loop,
all-OS, deterministic) → Toxiproxy userspace TCP proxy (real sockets, cross-platform, CI-ish,
cut-then-restore [B-testcontainers-toxiproxy-2024]) → `tc`/netem + `iptables` (real kernel faults,
Linux-only, slow CI [A-tc-netem-network-emulator-2026]) → full Jepsen (real cluster, Linux, Clojure).
The hot loop must live at the portable top; kernel-altitude is CI-only. Note the caveat that local
netem loss can be masked by TCP retransmit and reorder needs delay (`notes/120` f10) — a reason the
*synthetic* seam (which models the *outcome*, not the kernel mechanism) is cleaner for the hot loop.

---

## 6. The agentic lens — goose/gander, not a silo

**`notes/120` f7 (motivation; -0:SUSPECT synthesis across one A-anchor + C-grade essays):** the
literature's consensus is the *opposite* of the naive intuition — agents look *strongest* on
greenfield (no tacit knowledge to violate) and degrade on mature codebases [C-tianpan-expertise-cliff-2026]
— **yet the reconciliation IS the worry.** METR's RCT: 16 experienced devs on their own mature repos
were **19% slower** with early-2025 AI while *predicting* 24% faster and *feeling* 20% faster (a ~39pt
perception gap) [A-metr-ai-developer-productivity-2025]. Greenfield agentic velocity manufactures
"epistemic debt" fast, and the only backstop every source names is a fast, trustworthy verification
loop — "the agent itself only gets better when it can run, test, and verify its own work… only as
effective as your test infrastructure."

**How this weights the round (human, DECIDED as a lens):** research the testing topics on their
decades-of-merit — do **not** re-derive "testing, but for AIs" and discard 30 years of prior art —
but weigh each technique knowing this codebase may be ~89% agent-implemented and that foreign
contributors bring heterogeneous/unknown agentic setups. Strict typing / LSP / CI / TDD / integration
tests / lint are *higher* payoff in an agent-heavy process; that raised payoff is the evaluation
weight across all fronts. Concretely it front-loads concl-tdd (§4) and the L0 seam bank (§3): a
deterministic replay-from-seed loop is the single highest-value feedback signal for an agent
implementer, and error-paths — where agent-written code is weakest — are exactly what fc-3's
fault-injection covers.

---

## 7. The frontier — competing options, why-not, and the open ledger

Stated exactly per `notes/127` (state nothing as decided that isn't DECIDED):

**DECIDED (explicit, human).** The 3 fronts A/B/C; the oracle layer **parked**; the DST-ambition
**deferred**; agentic = goose/gander **lens** (not a silo); the ID convention (`f`-numbers +
`axis-*`/`concl-*`); the map (`plans/121`) frozen and conclusions kept separate (this file).

**LEAN (provisional, NOT a decision).** Heavily pro **in-process-DST-discipline** — bank L0 (reserve
the seam) + write the kernel async / inverted-control — and defer whether to climb to L1+. Language
**leaning Rust** (the DST-maturity input, `notes/123` f22, is *minor* and the choice is separate/out
of scope).

**DEFERRED (decision postponed to pre/post-conclusion).** (a) The `axis-dst-cost` rung — how far up
L0→L2 — gated on se-5 + the two OPEN facts; (b) whether to name "infer a *user leaf's*
containerizability" (Object B) a Dorc differentiator; (c) whether to *use* Jepsen/Maelstrom as a CI
tier.

**PARKED (late-stage; needs a Thing-worth-improving + Users).** The oracle layer / testing-*for*-oracle-authors;
idempotency-as-product (distinct from idempotency-as-test-oracle, td-3/fc-4, which is live); the
privileged tracing tool (`plans/078`). `plans/077`'s seccomp is a *product* DX tool for oracle authors
(Linux-only), **not** an already-banked self-test primitive (§1 correction).

**OPEN (unresolved facts that gate later design).** Transport model — "SSH a script over" is a
*starting point*, not a goal; an agentless-temporary-executor-phones-home model is live (if it wins,
the controller↔executor channel is tokio-socket-shaped → madsim/turmoil apply directly, `notes/124`
Open-Q1). Orchestrator ownership — Dorc's own vs built-on-pyinfra/similar (if delegated, the
DST-testable kernel shrinks and se-5 re-resolves, `notes/124` Open-Q2).

**Competing options explicitly weighed and set aside (why-not):**
- *Full DST (L2) / hypervisor (L3) as day-1* — why-not: retrofit-friendly and cheap-to-add-later, so
  not day-1; L2's marginal gain is small because Dorc's edge is mocked regardless (se-3, se-5).
- *madsim's tokio-shim as Dorc's seam mechanism* — why-not: Dorc's leaf IO is process-spawn + ssh, not
  tokio sockets, so the tokio-shim covers only *internal* controller networking; the seam wants
  provider-trait or state-machine treatment (`notes/123` f21, f28). Re-opens **iff** transport becomes
  socket-shaped (OPEN).
- *Jepsen as the primary harness* — why-not: consistency-model checkers don't fit Dorc's "reached
  desired state" oracle; Clojure cost; slow real-cluster tier (fc-1).
- *Idempotency-check as a day-1 design driver* — why-not: human recontext to a cheap post-hoc
  backstop / test-oracle (td-3).
- *shuttle/loom (concurrency-schedule testing)* — why-not: a *different altitude* (shared-memory
  schedules), relevant only if the orchestrator carries shared-memory concurrency
  [B-shuttle-concurrency-testing-2024]; quarantined.

## 8. Quarantine — interesting, correctly not chased (human triage → TODOs/tasks)

- **qn-1. TCP/IP-stack QuickCheck** (París & Arts, Erlang Factory 2009 slides) — open copy located,
  **unread**; the most on-point "model-based test of a network protocol's error-handling" primary, but
  *corroborating only* (the PBT-for-networked-systems thesis is already carried by
  [A-hughes-dropbox-pbt-2016] / [A-hughes-quickcheck-fun-profit-2016]).
- **qn-2. Will Wilson, "Testing Distributed Systems w/ Deterministic Simulation"** (Strange Loop 2014)
  — video; superseded as a written source by [A-fdb-sigmod-paper-2021] + [B-eatonphil-dst-bigdeal-2024].
- **qn-3. A push/agentless-adjacent Jepsen analysis** (closer to `kAGENTLESS`) — not located/read; the
  most directly-architecture-relevant Jepsen would be one of a push-model tool.
- **qn-4. Molecule Part-2** (converge→idempotence→verify cycle) — 404'd; needs a working mirror /
  web.archive. Concept covered by [A-puppet-litmus-concepts-2024]; Ansible-specific mechanics
  uncaptured.
- **qn-5. Terratest per-topic pages** (Idempotent, Cleanup) — unread (the assigned page was a stub).
- **qn-6. `shuttle`/`loom`** — shared-memory concurrency-schedule testing; a different altitude from
  network DST; relevant *only* if the orchestrator has shared-memory concurrency
  [B-shuttle-concurrency-testing-2024].
- **qn-7. `jepsen.antithesis` bridge** — the Jepsen and Antithesis lineages converging; noted
  (`notes/126` f40), not explored.
- **qn-8. The "is my DST exploring the state space?" coverage problem** — Sometimes-assertions
  (fc-5) are the *reachability* half; coverage remains the unsolved hard part ("resembles science more
  than engineering"). A standing research item, not a round-12 deliverable.
- **qn-9. Agent-network-sandbox tooling** (sandcat, NVIDIA OpenShell) — surfaced in a Front-B search;
  orthogonal (sandboxing AI agents), not chased.
- **qn-10. The parked versioning/binary-identity spike** (already in `../TODO.md`) — an oracle's
  grounding is silently parameterised by the exact binary; content-hash guard + purl-style version
  coordinate. Cross-links here via concl-rigor (a same-version distro-backport breaks both the
  read-only and oracle-soundness claims). Belongs to the security/oracle-contract thread, not this
  round.

## 9. Status & sources

Fronts: A ✓ (DST explainer `notes/122` + Rust ecosystem `notes/123` + seam-onto-Dorc `notes/124` +
`axis-dst-cost` ladder laid out) · B ✓ (containerizability quadrant `notes/125`, infer-vs-annotate
resolved) · C ✓ (transient-fault `notes/126`, Fronts A+C unified at the seam). Round-12 sources (33
new; the DST/Jepsen/IaC/tier clusters subagent-graded then re-verified on full read):
- DST core — [A-foundationdb-simulation-testing-2022] [A-fdb-sigmod-paper-2021] [A-tigerbeetle-vopr-2024] [B-eatonphil-dst-bigdeal-2024] [B-antithesis-dst-primer-2024] [A-etcd-antithesis-robustness-2025] [B-warpstream-dst-entire-saas-2024]
- Rust DST ecosystem — [A-madsim-deterministic-simulator-2025] [A-risingwave-deterministic-sim-2023] [A-sled-simulation-guide-2020] [B-s2-dst-async-rust-2025] [B-polarsignals-dst-state-machines-2025] [B-turmoil-tokio-2023] [B-shuttle-concurrency-testing-2024] [C-moonpool-provider-pattern-2024]
- Tier theory + IaC self-test — [A-google-test-sizes-2010] [A-google-small-medium-large-2011] [A-google-larger-testing-2020] [A-practical-test-pyramid-2018] [A-diverse-fantastical-shapes-testing-2021] [A-puppet-litmus-concepts-2024] [A-terraform-native-tests-2024] [A-hashicorp-testing-terraform-2024] [B-test-kitchen-lifecycle-2024] [B-molecule-podman-rgerardi-2020] [B-beaker-multinode-acceptance-2024]
- Containerizability quadrant — [A-bazel-test-encyclopedia-2024] [B-saflate-network-assumption-inference-2022] [B-pytest-test-categories-network-isolation-2025] [B-kselftest-docs-2026] [B-k8s-e2e-best-practices-2023]
- Fault-injection + Jepsen — [A-jepsen-readme-architecture-2026] [A-jepsen-client-op-model-2026] [A-jepsen-nemesis-fault-injection-tutorial-2022] [A-jepsen-etcd-3-4-3-2020] [A-jepsen-net-clj-source-2026] [B-antithesis-sometimes-assertions-2026] [A-tc-netem-network-emulator-2026] [B-testcontainers-toxiproxy-2024] [B-scylladb-extending-jepsen-2016] [A-maelstrom-distributed-systems-workbench-2022]
- PBT + counter-thesis + agentic — [A-hughes-quickcheck-fun-profit-2016] [A-hughes-dropbox-pbt-2016] [A-coplien-unit-testing-waste-2014] [B-rainsberger-integrated-tests-scam-2009] [A-metr-ai-developer-productivity-2025] [C-tianpan-expertise-cliff-2026]

Cross-links: `plans/121` (frozen map) · `plans/077`/`notes/077` (the seam — concl-seam precondition) ·
`plans/078` (privileged tracing — parked) · `notes/112` f50 + `plans/111` (three-valued verdict,
round-11 provenance) · `notes/110`–`113` (error-provenance bridge, fc-4) · `notes/076` (network
dominates perf — `axis-quadrant`) · `KNOBS`: `kFIDELITY` (third mode, se-4), `kAGENTLESS` (single
controller = orchestration complexity, se-5), `kVERIFY-calibrate` (TypeScript-not-Coq = concl-rigor),
`kVOLATILES-exclude` (hermeticity = the determinism precondition DST shares), `kFAIL` (phase-keyed
safe directions = the three-valued verdict's grounding), `kSTATE`/`kSCHEDULE`/`kELISION` ·
`DESIGN.md` (soundness capped — read via concl-rigor, not as a rigor-escape-hatch) · `AGENTS.md`
(`axis-platform` cross-platform constraint; pro-DST lean).
