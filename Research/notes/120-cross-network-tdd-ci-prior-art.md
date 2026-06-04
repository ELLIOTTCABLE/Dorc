# notes/120 — cross-network TDD/CI prior-art (round 12, broad sweep)

**Stamp:** round 12 · 2026-06-03 · broad-sweep gather-and-grade (interactive-research skill). f-numbers are round-12-scoped (f1…).
**Frame.** Greenfield-transition worry: agentic coding on a *new* network appliance, with no legacy-structure backstop. Question: how do real ops/distsys projects do precise, correctness-critical testing & CI **above the unit tier** — (Q1) integration testing that crosses the network; (Q2) whether projects split into *multiple* tiers beyond unit/integration (fast container/kernel vs slow high-fidelity cluster); (Q3) correctness testing of transient network events / error-handling. Unit+mocks treated as already-understood.
**Method note:** 32 sources registered this round (`../sources.json`). Clusters DST / Jepsen+fault-injection / IaC-self-test / tier-theory were fully-read+graded by Opus subagents (`graded-by: subagent` — provisional, re-verify when narrowing); FoundationDB, Google ch14, and the counter-thesis + PBT + agentic anchors were read top-level (some via substantial extract, flagged `-0:SUSPECT` grading-certainty).
**Cross-links:** `notes/077` (the wrappable leaf-execution / network-IO seam — the DST precondition); `notes/112` f50 (K8s True/False/**Unknown** three-valued verdict = the CAP third state, f4 below); `KNOBS` (`kAGENTLESS` push-model single-collector; `kSTATE`; `kSCHEDULE`; `kELISION`); `DESIGN.md` (soundness is capped — matches f13); `notes/076` (perf: network dominates, f9 cost).

## Findings

**The load-bearing four (most transferable to Dorc):**
- f1. **The tier boundary that matters for Dorc is "may this code touch the network / another machine?" — and at least one major shop made it a *mechanically-enforced* axis, not a human label.** Google's Test Sizes defines tiers by *resource reach*: Small = no network, Medium = **localhost only**, Large = cross-machine; and "it's possible to get the tests to police these limits" (a Small test is *sandboxed* from the network). +SURE this is simultaneously the cleanest answer to Q2 *and* a direct rhyme with the Dorc thesis (infer the tier from analysis/taint; enforce it; don't make the user annotate it). [A-google-test-sizes-2010][A-google-small-medium-large-2011]
- f2. **Deterministic Simulation Testing (DST) is the highest-correctness tier for exactly Dorc's problem (correctness across an unreliable network) — and classic DST is retrofit-hostile: it demands that *all* nondeterminism (network, disk, clock, RNG) flow through one pluggable seam, designed in from day one.** For greenfield Dorc this is a day-one architectural commitment, and the single obvious seam is the network/SSH-leaf-execution boundary (`notes/077`). The escape hatch (Antithesis deterministic hypervisor) removes the retrofit cost but still requires mocking external deps. ~SUSPECT that full DST is worth Dorc's cost, +SURE that the *seam* it requires is worth reserving now. [B-antithesis-dst-primer-2024][A-foundationdb-simulation-testing-2022][A-tigerbeetle-vopr-2024][B-eatonphil-dst-bigdeal-2024]
- f3. **The domain-specific correctness oracle the PLT/unit world does *not* supply, and that every IaC tool converges on: idempotency = converge twice, second run must report zero changes.** Litmus states it as near-free proof that desired state was reached *and* config is valid/consistent — sidestepping expensive bespoke post-state assertions. +SURE this is the cheapest high-value cross-network oracle Dorc can adopt, and it's already latent in the plan/apply model. [A-puppet-litmus-concepts-2024]
   - └ r12 human-recontext: idempotency-as-a-test-check reads as a *post-hoc backstop / meta-test* (coverage-like, cheap-to-add-later), not a day-1 design driver — filed with the defense-in-depth bucket. See `plans/121` gate-decision.
- f4. **The CAP-adjacent *third state* is unavoidable and must be first-class in both the harness and the analyzer:** a crashed/timed-out op over an unreliable link is neither success nor failure — Jepsen models it as one that "may or may not take place," and its checkers are built to tolerate that. +SURE this is the same three-valued verdict round 11 already landed (K8s True/False/**Unknown**, `notes/112` f50) — testing and analysis must agree on it. [A-jepsen-nemesis-fault-injection-tutorial-2022][A-jepsen-etcd-3-4-3-2020]

**The counter-thesis (strong, and it cuts toward Dorc's design):**
- f5. **The bugs that matter live "between the units"; mocks test your *assumptions about* deps, not the deps — and Coplien's frame names Dorc's own lever:** unit tests are near-worthless "unless you have an extrinsic requirements oracle for the unit under test." A Dorc *oracle* is exactly that. +SURE so the oracle concept is what could make a Dorc unit-tier meaningful where generic mock-heavy unit testing is not. [A-coplien-unit-testing-waste-2014]
- f6. **But the opposite extreme is also a documented trap:** Rainsberger's arithmetic — broad integrated tests run ~50/sec, so 100k take 34 min and still cover a vanishing fraction of the path-space, breed false confidence, and self-replicate. +SURE the prescription is a deliberate *middle*: many fast deterministic tests that pin boundaries + a *small* high-fidelity cross-network tier — neither "mock everything" nor "integrate everything." This is the same shape the modern practitioner zeitgeist (honeycomb/trophy) reaches for now that containers made a real-dependency middle cheap. [B-rainsberger-integrated-tests-scam-2009][A-diverse-fantastical-shapes-testing-2021]

**The agentic-greenfield framing (your motivation — the evidence contradicts then vindicates the premise):**
- f7. **The literature's consensus is the *opposite* of the stated intuition — agents look *strongest* on greenfield (no tacit knowledge to violate; intent dominates over history) and degrade on mature codebases — yet the reconciliation *is* your worry.** METR's RCT: 16 experienced devs on their *own* mature repos were **19% slower** with early-2025 AI while predicting 24% faster and *feeling* 20% faster (a ~39pt perception gap). The brownfield/greenfield essays add: greenfield agentic velocity manufactures "epistemic debt"/style-drift/"locally plausible but globally underexplained changes" fast, and the *only* backstop every source names is a fast, trustworthy verification loop — "the agent itself only gets better when it can run, test, and verify its own work… only as effective as your test infrastructure" (Crookston, andrewcrookston.com 2026). -0:SUSPECT (synthesis across one A-anchor + C-grade essays): the missing "legacy sanity backstop" you fear is *replaceable* by a test backstop you build first — which is precisely the front-loaded TDD/CI discipline you're proposing. [A-metr-ai-developer-productivity-2025][C-tianpan-expertise-cliff-2026]

**Q2 — yes, real projects split into >2 tiers, keyed on resource-reach + cost (not test-type names):**
- f8. Observed tier ladders: **Google** — Small(no net)/Medium(localhost)/Large(cross-machine)/Enormous(too big for shared pool) + "local" tags for corp-infra deps; hermetic-only in the main CI (TAP), non-hermetic large tests run on *separate* infra. **IaC** — lint → unit(plan-mode/mocked provider, ~0 cost) → contract → integration(real short-lived resources, apply+destroy, 15min+) → e2e; expensive tiers gated post-merge, minimal, in an isolated account. **DST world** — fast CI chaos-injection tier ↑ high-fidelity hypervisor tier (WarpStream); Maelstrom(fast/local/simulated-net) ↓ full-cluster Jepsen(real IP net/VMs). +SURE the split is real and universal; the *axis* is reach+cost. [A-google-larger-testing-2020][A-hashicorp-testing-terraform-2024][B-warpstream-dst-entire-saas-2024][A-maelstrom-distributed-systems-workbench-2022]
- f9. **Fidelity/cost is explicit and quantified, and containers shifted the economics.** Containers deploy in seconds but are *not* production-representative (no real init/SSH — "common public images are usually not representative of production on full VMs"); throwaway VMs are minutes but faithful; the real-network tier is the expensive top (Jepsen single test 5–10 min, suite hours). Testcontainers made a real-dependency middle cheap enough that the "fat middle" beats the classic pyramid for I/O-heavy systems. +SURE — directly informs where Dorc draws its own fast/slow line. [A-puppet-litmus-concepts-2024][B-scylladb-extending-jepsen-2016][B-testcontainers-toxiproxy-2024]
   - └ r12 human-sharpening (load-bearing): NOT a fast/slow *line* but a fast/slow × **containerizable/non-containerizable** quadrant; the containerizability seam matches *no* known Dorc seam (not tool / POSIX-construct / domain-of-control / exec-time) and may need constant tracking. Open Q: do shops *infer* it or *create+enforce* it artificially? See `plans/121` axis-quadrant / concl-quadrant.

**Q1 & Q3 — network-fault injection toolbox + the schedule/oracle:**
- f10. **Standard fault-injection at three altitudes.** *Kernel:* `tc`/netem (delay, loss incl. bursty Gilbert-Elliot/Markov, dup, corrupt, reorder, rate) + `iptables` DROP for partitions; Jepsen's actual code: partition = `iptables -A INPUT -s <ip> -j DROP` (directional, flush-to-heal), impairment = `tc` prio-qdisc + per-dest-IP filter → netem band (selective, per-peer). *Proxy:* Toxiproxy / Testcontainers-Toxiproxy — toxics (latency/bandwidth/slicer/timeout/slowClose) + full cut-then-restore between containers. *Simulated:* in-process net (Maelstrom `--latency`, FoundationDB sim) — fastest/deterministic, can't test the real transport. Caveats: local netem loss can be masked by TCP retransmit; reorder needs delay; ingress-vs-egress placement matters. +SURE [A-jepsen-net-clj-source-2026][A-tc-netem-network-emulator-2026][B-testcontainers-toxiproxy-2024]
- f11. **The fault *schedule* is itself code/data, and transient = inject-then-HEAL-then-assert-reconvergence.** Jepsen's nemesis is an op on a timeline (`sleep/start/sleep/stop` cycle, `--nemesis-interval`); FoundationDB's "swizzle-clog" (clog connections one-by-one, unclog in random order) is the champion deep-bug finder. +SURE [A-jepsen-nemesis-fault-injection-tutorial-2022][A-foundationdb-simulation-testing-2022]
- f12. **The verification *oracle*, not the fault injection, bounds coverage.** ScyllaDB's candid lesson: a weak checker ("can we read?") gives false confidence even with great faults. Jepsen asserts strict-serializability via cycle-detection over a real-time dependency graph (Elle). PBT (Hughes): find bugs by comparing two *independent* descriptions (impl vs a compact stateful model) and surfacing inconsistencies — "the specification is the real weakness, not the testing." The Dropbox paper: a black-box model talking *only* via the filesystem found data-loss bugs in Dropbox & ownCloud, handling nondeterminism + hidden state, and named network-partition ops as the next fault to add. +SURE — Dorc's oracle = the model; the analyzer is the second independent description. [B-scylladb-extending-jepsen-2016][A-hughes-quickcheck-fun-profit-2016][A-hughes-dropbox-pbt-2016]

**The soundness ceiling (matches DESIGN.md):**
- f13. **Every high-correctness practice here is explicitly testing-not-proof and says so:** Jepsen "we can prove the presence of bugs, but not their absence"; DST is "terrifyingly easy to build… [so it] appears to be doing a ton of testing, but actually never explores very much of the state space"; seeds break on code change. +SURE Dorc should inherit the same humility and the same bug-finding (not proving) framing DESIGN.md already commits to. [A-jepsen-etcd-3-4-3-2020][B-eatonphil-dst-bigdeal-2024]
   - └ r12 human-sharpening (load-bearing, scary): "best-effort" is NOT a rigor-escape-hatch. For Dorc it means provable-algorithm-level rigor (best-in-the-universe effort) so the *user* needn't be correct; the soundness ceiling only bounds dangerous algorithmic assumptions + honestly acknowledges user-facing imperfection from the day-1 oracle contract. Front-load. See `plans/121` concl-rigor.

**To-acquire (gaps surfaced, not yet read):**
- TCP/IP-stack model-based testing via QuickCheck (Arts/Hughes, ERLANG'09) — ACM-paywalled; need open copy / OOB. The most on-point "test a network protocol's correctness" primary.
- Will Wilson, "Testing Distributed Systems w/ Deterministic Simulation" (Strange Loop 2014) — the canonical DST talk; video, not captured as text.
- Molecule Part-2 (converge→idempotence→verify cycle) — 404'd; the idempotency oracle for Ansible specifically (Litmus covers the concept).
- Terratest per-topic pages (Idempotent, Cleanup, Iterating-locally-using-Docker) — the assigned best-practices page was a stub.
- METR full paper (arXiv 2507.09089), GitClear 211M-line analysis, Anthropic skill-formation RCT (arXiv 2601.20245) — agentic corroboration, captured only via summary.

## Citations

### Tier theory — the resource/network boundary (Q2 core)
> [A-google-test-sizes-2010]:§resource table (relevance: +1:SURE) — the network axis, three-valued and enforceable
> Network access = No / localhost / Yes (Small / Medium / Large). Database = No / Yes / Yes. Use of external systems = No / Discouraged / Yes. Time limit (seconds) = 60 / 300 / 900.
> The major advantage that these test definitions have is that it's possible to get the tests to police these limits. For example, in Java it's easy to install a [security manager] … configured for a particular test size and disallows certain activities.
> A Small test equates neatly to a unit test, a Large test to an end-to-end or system test and a Medium test to tests that ensure that two tiers in an application can communicate properly (often called an integration test).

> [A-google-small-medium-large-2011]:§"A rose by any other name" (relevance: +1:SURE) — reach, not runtime, defines the tier; extra tiers encode network/host reach
> No matter if a test runs in a millisecond, if it interacts with multiple other processes or the file system, it's still of a fundamentally different nature than a test that exercises pure algorithms or business logic.
> the 'enormous' size was added to indicate a test that was somehow 'too big' to execute automatically in the shared datacenter execution pool … tests [that] had some dependencies on NFS or other corp infrastructure inaccessible from the datacenters could be marked 'local' to be run on the developer's workstation.
> roughly 70% small, 20% medium, and 10% large for the common case … these numbers essentially were pulled out of a hat. But they were a very useful starting point.

> [A-google-larger-testing-2020]:§"What Are Larger Tests?" / "Common Gaps" (relevance: +1:SURE) — size vs scope; the vacuum effect; config = #1 outage cause; hermetic-only CI
> Small tests are restricted to one thread, one process, one machine. Larger tests do not have the same restrictions. But Google also has notions of test scope. A unit test necessarily is of smaller scope than an integration test.
> A unit test is like a problem in theoretical physics: ensconced in a vacuum, neatly hidden from the mess of the real world, which is great for speed and reliability but misses certain defect categories.
> At Google, configuration changes are the number one reason for our major outages.
> when TAP replaced C/J Build as our formal continuous build system, it was only able to do so for tests that met TAP's eligibility requirements: hermetic tests buildable at a single change that could run on our build/test cluster within a maximum time limit. Although most unit tests satisfied this requirement, larger tests mostly did not.

> [A-practical-test-pyramid-2018]:§Integration Tests / §pipeline (relevance: +1:SURE) — network = the narrow/broad divider; stages cut by speed+scope not type
> Integrating with a service over the network is a typical characteristic of a broad integration test and makes your tests slower and usually harder to write.
> I like to treat integration testing more narrowly and test one integration point at a time by replacing separate services and databases with test doubles.
> defining the stages of your deployment pipeline is not driven by the types of tests but rather by their speed and scope.

> [A-diverse-fantastical-shapes-testing-2021]:¶shapes/terminology (relevance: -0:SUSPECT) — fat-middle shapes; fix tier semantics before counting
> The pyramid argues that you should have most testing done as unit tests, the honeycomb and trophy instead say you should have a relatively small amount of unit tests and focus mostly on integration tests.
> their definition of 'unit test' is specifically what I would call a solitary unit test. Similarly their notion of integration test sounds very much like what I would call a sociable unit test. This makes the pyramid versus honeycomb discussion moot.

### DST — the highest-correctness tier, and its seam (Q1/Q3, retrofit-hostility)
> [A-foundationdb-simulation-testing-2022]:§Simulation (relevance: +1:SURE) — deterministic whole-cluster sim; the network is modelled; swizzle-clog
> Simulation is able to conduct a deterministic simulation of an entire FoundationDB cluster within a single-threaded process. Determinism is crucial in that it allows perfect repeatability of a simulated run.
> Simulation also models the network, allowing a small amount of code to specify delivery of packets. We use Simulation to simulate failures modes at the network, machine, and datacenter levels, including connection failures, degradation of machine performance, machine shutdowns or reboots, machines coming back from the dead.
> the reigning champion is called "swizzle-clogging". To swizzle-clog, you first pick a random subset of nodes … "clog" (stop) each of their network connections one by one … Finally, you unclog them in a random order … particularly good at finding deep issues that only happen in the rarest real-world cases.

> [A-tigerbeetle-vopr-2024]:§The VOPR / Assertions (relevance: +1:SURE) — the seam (stub clock/net/disk); seed+commit replay; assertions on in prod
> In the simulator, all non-deterministic parts of the system are stubbed out. This includes the clock, network, and disk operations.
> Because our simulator is deterministic based on a seed number and the Git commit, we can perfectly reproduce any bugs discovered in testing … One minute of VOPR time is equivalent to days of real-world testing.
> it may drop and reorder packets, partition the network, or corrupt reads and writes to the "disk".
> TigerBeetle is somewhat unique in that it keeps these assertions on, even in production. The logic is that it is far better to stop operating than to continue operating in an incorrect state.

> [B-antithesis-dst-primer-2024]:§"How do I implement DST" (relevance: +1:SURE) — THE retrofit fork; two hard costs
> One approach, popularized by FoundationDB, is to design the system under test so that all nondeterministic components are pluggable. Since this requires the system and all its dependencies to be built with deterministic simulation testing in mind, this approach is generally impractical for systems already in production.
> While building a fully deterministic system is often viewed as the main technical challenge, achieving thorough and efficient exploration of the state space – which in most software systems is extremely large – is a complex undertaking as well.
> DST platforms like Antithesis … still require external dependencies to be mocked or otherwise plugged to ensure determinism.

> [B-eatonphil-dst-bigdeal-2024]:§Edges / Consideration-4 (relevance: +1:SURE) — DI of clock/seed; single-thread+async-IO; the candid limits
> To "control" randomness or time basically means you support dependency injection … Rather than referring to a global clock or a global seed, you need to be able to receive a clock or a seed from someone.
> since using blocking IO would mean an entire class of concurrency bugs could not be discovered while running the simulator in a single thread, we must limit ourselves to asynchronous IO. Single threaded and asynchronous IO. These are already two big limitations.
> it's terrifyingly easy to build a DST system that appears to be doing a ton of testing, but actually never explores very much of the state space of your system.

> [A-etcd-antithesis-robustness-2025]:§"How We Tested" (relevance: +1:SURE) — closest analog; declarative props; net vs container faults; cost
> The platform works by running the entire etcd cluster inside a deterministic hypervisor … complete control over every source of non-determinism, such as network behavior, thread scheduling, and system clocks.
> this approach uses declarative, property-based assertions about system behavior … "data consistency is never violated" or "a watch event is never dropped."
> Network faults: latency, congestion, and partitions. Container-level faults: thread pauses, process kills, clock jitter, and CPU throttling. … In total, we ran 830 wall-clock hours of testing, which simulated 4.5 years of usage.

> [B-warpstream-dst-entire-saas-2024]:§"what's the big deal"/"bugs" (relevance: +1:SURE) — two tiers on one workload; the payoff number; diminishing returns
> Our correctness tests even inject faults all over the WarpStream stack using a custom chaos injection library that we wrote … unlike our correctness tests, the Antithesis hypervisor is really smart and automatically fuzzes the system under test in an intelligent way.
> Our correctness tests had run in our regular CI workflows for literally 10s of thousands of hours … and not once ever caught this bug. Antithesis caught this bug in its first 233 seconds of execution.
> it took about 160 "application hours" for Antithesis to "stall" and stop discovering new "behaviors" … running the tests for longer than 160 hours has diminishing returns.

### Jepsen — black-box correctness under faults; the CAP third state (Q1/Q3)
> [A-jepsen-nemesis-fault-injection-tutorial-2022]:§Introducing Faults (relevance: +1:SURE) — fault-as-timeline-op; the may-or-may-not-happen third state
> This one partitions the network into two halves, selected randomly, when it receives a `:start` op, and heals the network when it receives a `:stop`.
> (gen/nemesis (cycle [(gen/sleep 5) {:type :info, :f :start} (gen/sleep 5) {:type :info, :f :stop}]))
> letting every error crash the process is still safe: jepsen's checkers understand that a crashed operation may or may not take place.

> [A-jepsen-etcd-3-4-3-2020]:§2 Test Design / §3.2 / §4 (relevance: +1:SURE) — fault menu; checker = cycle-detection; quantified loss; prove-presence-not-absence
> We introduced … network partitions isolating single nodes, separating the cluster into majority and minority components, and non-transitive partitions … We crashed and paused random subsets of nodes, as well as specifically targeting leaders … clock skew up to hundreds of seconds.
> we constructed a dependency graph between transactions on the basis of real-time precedence … Checking that graph for cycles allowed us to determine whether the history was strict serializable.
> With two-second leases TTLs … we could reliably induce the loss of ~18% of acknowledged updates.
> we can prove the presence of bugs, but not their absence.

> [A-jepsen-net-clj-source-2026]:§iptables/net-shape! (relevance: +1:SURE) — the exact partition + impairment mechanism
> (drop! [net test src dest] … (su (exec :iptables :-A :INPUT :-s (control.net/ip src) :-j :DROP :-w))) … heal! … (exec :iptables :-F :-w)
> filter dst ip's to netem qdisc (exec tc :filter :add … :match :ip :dst (control.net/ip target) :flowid "1:4")
> :loss - When used locally (not on a bridge or router), the loss is reported to the upper level protocols. This may cause TCP to resend and behave as if there was no loss.

> [A-maelstrom-distributed-systems-workbench-2022]:§Design Overview (relevance: +1:SURE) — the cheap simulated-net tier below full-cluster Jepsen; tier-portable architecture
> running a full cluster of virtual machines connected by a real IP network is tricky for many users. Maelstrom strips these problems away … runs those nodes as processes on your local machine, and connects them via a simulated network.
> Maelstrom's checkers can verify sophisticated safety properties … up to strict serializability.
> `--latency MILLIS`: Approximate simulated network latency … `--nemesis FAULT_TYPE` … `--nemesis-interval SECONDS`: How long between nemesis operations, on average.

> [A-tc-netem-network-emulator-2026]:§DESCRIPTION/OPTIONS/LIMITATIONS (relevance: +1:SURE) — the kernel palette and its hard edges
> The netem queue discipline provides Network Emulation functionality for testing protocols … delay, loss, duplication, and packet corruption.
> gemodel … Use a Gilbert-Elliot (burst loss) model … Use a 4-state Markov chain to describe packet loss.
> For any method of reordering to work, some delay is necessary … for TCP performance test results to be realistic netem must be placed on the ingress of the receiver host.

> [B-scylladb-extending-jepsen-2016]:§nemesis/next-steps (relevance: +1:SURE) — tc delay/reorder; per-container faketime; weak-checker warning; cost
> Nemesis can also change the time on a node, and impose network delays, using the tc utility. Using tc you can also reorder network packets.
> The ScyllaDB version of Jepsen uses faketime to enable ScyllaDB processes in different containers on the same host to have different ideas of the correct time.
> The current Jepsen checker just verifies that the data can be successfully read, which can happen even if a node has failed. So we want to verify the state of all nodes.
> A single test test takes 5-10 minutes, and the whole suite takes several hours.

### IaC self-test — the idempotency oracle, the lifecycle, the cost-gated tiers (Q2/Q3, most-analogous)
> [A-puppet-litmus-concepts-2024]:§Test Assertion / Test Infrastructure (relevance: +1:SURE) — idempotency as a near-free correctness oracle; container vs VM fidelity
> If puppet runs the same manifest a second time without errors or changes, this already implies that the desired system state has been reached … A service starting up and staying running implies that its configuration was valid and consistent. This is a check that would be very hard, nay prohibitively expensive, to implement in a test. Idempotency checking makes this (almost) free.
> In acceptance tests the first way to check a system's target state is idempotency. This is implemented as first-class operation idempotent_apply in the Litmus DSL … serverspec is preconfigured inside Litmus.
> Throw-away VMs … accurate representation of production and full isolation. The downside … high resource usage and provisioning times in the order of minutes … To reduce resource usage … docker … deploy in seconds … common public images are usually not representative of production on full VMs.

> [A-terraform-native-tests-2024]:§"Integration or Unit testing"/Cleanup (relevance: +1:SURE) — apply=integration vs plan=unit in one harness; reverse-order teardown
> By default, tests within Terraform create real infrastructure and can run assertions and validations against that infrastructure. This is analogous to integration testing … Replacing the command value with command = plan instructs Terraform to not create new infrastructure … analogous to unit testing.
> Terraform v1.7.0 introduced the ability to mock data returned by the providers during a terraform test execution.
> At the conclusion of a test file, Terraform attempts to destroy every resource it created … in reverse run block order.

> [A-hashicorp-testing-terraform-2024]:§testing pyramid/integration (relevance: +1:SURE) — the 5-tier IaC pyramid + cost cliff + CI gating (via Internet Archive)
> Higher-level tests in the pyramid take more time to run and cost more due to the higher number of resources you have to configure and create.
> Since you have to set up and tear down the resources, you will find that integration tests can take 15 minutes or more … implement as much unit and contract testing as possible to fail quickly on wrong configurations instead of waiting for resources to create and delete.
> run integration tests after merging feature branches and select the minimum number of resources you need … Conduct module tests in a different project or account so that you can independently track the cost.

> [B-test-kitchen-lifecycle-2024]:§"kitchen test" (relevance: +1:SURE) — the lifecycle; fail-fast preserves state for diagnosis; full cycle is CI-only
> 1. Destroys the instance if it exists 2. Creates the instance 3. Converges the instance 4. Verifies the instance with InSpec 5. Destroys the instance
> Test Kitchen will abort the run … at the first sign of trouble … the instance won't be destroyed. This gives you a chance to inspect the state of the instance and fix any issues.
> If you're using this in your test-code-verify development cycle it's going to quickly become very slow and frustrating. You're better off running the converge and verify sub-commands in development and save the test sub-command [for] end-to-end.

> [B-molecule-podman-rgerardi-2020]:§drivers (relevance: -0:SUSPECT) — multi-driver = multi-fidelity; container fidelity caveat
> Molecule uses drivers to provision testing instances using different technologies, including Linux containers, virtual machines and cloud providers.
> because this role starts the Apache web server as a system service, we need to use container images that enable 'systemd' … registry.access.redhat.com/ubi8/ubi-init.

> [B-beaker-multinode-acceptance-2024]:§README top (relevance: -0:SUSPECT) — the multi-node real/virtual-machine acceptance tier
> Beaker is a test harness focused on acceptance testing via interactions between multiple (virtual) machines. It provides platform abstraction between different Systems Under Test (SUTs), and it can also be used as a virtual machine provisioner.

### Network-fault substrate at the proxy altitude (Q3)
> [B-testcontainers-toxiproxy-2024]:§module (relevance: +1:SURE) — in-test transient-fault injection between containers; the cut-then-heal assertion pattern
> You can simulate network failures: between Java code and containers … between containers, for testing resilience and emergent behaviour of multi-container systems … Toxics: bandwidth, latency, slicer, slowClose, timeout, limitData.
> we can disable the proxy to simulate a complete interruption to the network connection … expect failure when the connection is cut … and with the connection re-established, expect success.

### Property-based testing of distributed/networked systems (Q1/Q3 — the model-as-oracle)
> [A-hughes-quickcheck-fun-profit-2016]:§(retrospective) (relevance: +1:SURE) — two independent descriptions reveal bugs; spec is the weakness; found a Dynamo/Riak bug
> we find bugs by comparing two independent descriptions of the desired behaviour—the implementation, and the specification—and it is the inconsistencies between the two that reveal errors.
> The need for a specification is the real weakness of property-based testing—not that we use testing, rather than static analysis or proof, to relate specifications and implementations. Testing works well enough!
> we helped Basho test their no-SQL database, Riak, for the key property of eventual consistency—and found a bug … that was present, not only in Riak, but in the original [Dynamo] paper … that kicked off the no-SQL trend.

> [A-hughes-dropbox-pbt-2016]:§Abstract/§IX (relevance: +1:SURE) — black-box model via the filesystem; nondeterminism+hidden-state; partitions named as next fault
> Our model is based on a technique for testing nondeterministic systems that avoids requiring that the system's internal choices be made visible to the testing framework.
> Since it's written in a black-box style, avoiding synchronizer-specific APIs and communicating only via the filesystem, we were able to apply it to three popular synchronizers … and found surprising behaviors in two of them.
> Another rich source of incorrect behaviors in distributed systems is network partitions. To provoke such behaviors, it might be useful to extend our test cases with operations to disconnect and reconnect hosts from the network.

### Counter-thesis (above-unit value; the oracle hook; the integrated-test trap)
> [A-coplien-unit-testing-waste-2014]:§1.2 (relevance: +1:SURE) — the extrinsic-requirements-oracle hook (= a Dorc oracle)
> Be humble about what your unit tests can achieve, unless you have an extrinsic requirements oracle for the unit under test. Unit tests are unlikely to test more than one trillionth of the functionality of any given method in a reasonable testing cycle.

> [B-rainsberger-integrated-tests-scam-2009]:¶arithmetic (relevance: +1:SURE) — broad integrated tests can't cover the path-space; false-confidence self-replication
> Integrated tests typically touch the file system or a network connection, meaning that they run on average at a rate of no more than 50 tests per second. Your 100,000-test integrated test suite executes in 2000 seconds or 34 minutes.
> The more integrated tests you write, the more of a false sense of security you feel … Over time your code path coverage decreases because the complexity of your code base grows more quickly than your capacity to write enough integrated tests to cover it.

### Agentic-greenfield (the motivation; A-anchor + practitioner reframe)
> [A-metr-ai-developer-productivity-2025]:§headline (relevance: -0:SUSPECT) — the perception gap on mature repos
> when developers use AI tools, they take 19% longer than without—AI makes them slower.

> [C-tianpan-expertise-cliff-2026]:¶thesis (relevance: -0:SUSPECT) — agents strongest where there's no tacit knowledge (greenfield), reframing the worry
> AI agents are remarkably productive in greenfield systems precisely because there is little tacit knowledge to violate. They degrade in mature codebases for exactly the same reason.
