# notes/122 — Front A deliverable #1: Deterministic Simulation Testing, from first principles

**Stamp:** round 12 · 2026-06-03 · Front A (seam / DST-readiness), first deliverable. Per `plans/121` Front A, this is the from-first-principles DST explainer (the human asked for the beginner explanation before any "what will we *do*" decision; that decision stays deferred — `axis-dst-cost`).
**Regrade (this turn, full-read in main context):** [A-foundationdb-simulation-testing-2022] A holds (canonical first-party). [B-eatonphil-dst-bigdeal-2024] B holds — practitioner synthesis with original pseudocode, reviewed by Will Wilson and Alex Miller; not A only because it is one engineer's overview, not primary docs/peer-reviewed. The TigerBeetle/etcd/Antithesis/WarpStream claims below are `graded-by: subagent` extracts, flagged where load-bearing.
**Remaining Front-A gather (queued):** Will Wilson Strange Loop 2014 talk (video → transcript); FoundationDB SIGMOD 2021 paper (testing §); `madsim`/Pekka Enberg/Tyler Neely for the typed-language seam. Then: the seam-onto-Dorc mapping + `axis-dst-cost` write-up.

---

## The beginner explainer

### 1. The problem DST exists to kill
The bugs that actually take down a network appliance are not logic bugs in a single function — those, ordinary unit tests catch. They are **interaction bugs**: a packet drops *here*, a node reboots *then*, two acknowledgements arrive in *that* order. Three properties make them uniquely awful:
- **chaotic** — they only appear in a specific interleaving of timing + failures, one of astronomically many;
- **non-reproducible** — you cannot re-run the exact interleaving, so even a bug you *saw* may never recur on demand;
- **invisible to unit tests** — which mock the network away precisely to be fast and deterministic, and so never exercise the interleavings. (Google's own framing: a unit test "is like a problem in theoretical physics: ensconced in a vacuum… misses certain defect categories." [A-google-larger-testing-2020])

So the dream is: get the chaos of a distributed system into the one place a test is good at — a deterministic, replayable, property-checked harness. That dream is DST.

### 2. The one trick (and the two superpowers it buys)
Run the **entire** system on top of a **fake world that the test controls** — fake clock, fake network, fake disk, fake randomness — and drive every bit of that fake world from **one random seed**. Because *every* source of nondeterminism now flows through the controllable layer, the whole run becomes a **pure function of the seed**. Two superpowers fall out, and they are the entire payoff:

- **Fuzz the failure-space, fast.** The harness picks seeds, and each seed is a different randomized scenario: drop these packets, add that latency, reboot this node mid-write. Because it is all one process in simulated time, you run *thousands* of scenarios per minute with no real network waits. (TigerBeetle: "One minute of VOPR time is equivalent to days of real-world testing." [A-tigerbeetle-vopr-2024]. FoundationDB estimates ~1 trillion CPU-hours of simulation run this way [A-foundationdb-simulation-testing-2022].)
- **Replay any failure, exactly.** When a seed trips a bug, you hand that *same seed* back and the entire run — every "random" drop and reboot — reproduces bit-for-bit, so you can debug a distributed failure as easily as a failing unit test. (Eaton: "you allow the user to enter the same seed again… recreate the entire program run… debug the program trivially." [B-eatonphil-dst-bigdeal-2024])

That second one is the quiet revolution: distributed bugs go from "good luck" to "paste the seed."

### 3. How it actually works (build it up in four steps — Eaton's ladder)
The mechanism is just **dependency injection of every nondeterministic thing**, plus a discipline about threads. +SURE on all four (Eaton gives runnable pseudocode for each [B-eatonphil-dst-bigdeal-2024]).

**(a) Randomness & time → injected, not global.** Code must never reach for a global clock or RNG; it must *receive* them. The whole app gets a `start(clock, seed)` entrypoint; production passes the real clock, the simulator passes a fake one whose `now` only advances when the harness ticks it, and whose seed it owns. That is the whole "control time/randomness" requirement — ordinary DI.

**(b) A worked bug.** Eaton's `read_file` loops reading a buffer and (bug) copies `read_buffer[0:sizeof(read_buffer)]` instead of `[0:n_read]`. The simulator supplies a fake `io` whose `read` returns a *random partial* number of bytes each call, asserts the reassembled bytes equal the source, and — at some seed — the partial-read trips the bug. The point: the fault (short reads) is a normal, legal thing the real world does that you'd almost never hit in a hand-written test.

**(c) The hard constraint: single-thread + async I/O.** To run "many nodes" deterministically you collapse them into *one thread* (you cannot cheaply make the OS scheduler deterministic without a hypervisor). But on one thread, *blocking* I/O would hide every concurrency bug — so the code must use **async I/O**. "Single threaded and asynchronous I/O. These are already two big limitations." [B-eatonphil-dst-bigdeal-2024] (This is why the language matters: Go fights you — Polar Signals forked the Go runtime to control goroutine scheduling; Rust's `tokio` couldn't be fully made deterministic, hence `turmoil`/`madsim`.)

**(d) A whole distributed system in one process.** If the system is *self-contained* (e.g. embeds its own consensus), you start N nodes on N ports *in the same process*, give them all a `sim_io` that injects random `bad write`/`bad read`/`bad open` + random latency on every call, run a workload (insert keys, record a history), **assert an invariant after every step** (the history stays valid), and **crash/restart random nodes** along the way. When the invariant breaks, print the seed. That is a DST harness in ~40 lines.

A sh-flavoured sketch of the *seam* this all hangs from (per Dorc's idiom — one chokepoint, swappable backend):

```sh
dorc_exec() { "$DORC_BACKEND" "$@"; }     # prod: real_ssh   |   test: sim_backend
# the simulator backend is a pure function of the seed:
sim_backend() {                            # $1=host $2=cmd
   case "$(seeded_rand "$DORC_SEED" "$1$2")" in
      0|1|2) emit_ok        "$1" "$2" ;;   # succeeded
      3)     emit_timeout   "$1" "$2" ;;   # connection hung
      4)     emit_drop_after_mutate "$1" ;; # the CAP nightmare: it ran, ack lost
   esac
}
```
Re-run with the same `$DORC_SEED` and you get the identical sequence of "host 7 dropped right after the mutation landed" — deterministically.

### 4. Two flavours: build-it-in vs buy-a-hypervisor
- **Build-it-in (FoundationDB, TigerBeetle).** You design the system so all nondeterminism is pluggable, and you write your own simulator. Maximum control; the simulator becomes a first-class part of the codebase. The catch (load-bearing, +SURE): "this requires the system and all its dependencies to be built with deterministic simulation testing in mind… generally impractical for systems already in production." [B-antithesis-dst-primer-2024] → **it is a day-1 decision or never.**
- **Buy-a-hypervisor (Antithesis).** Run the *unmodified* binary inside a deterministic hypervisor that controls the whole machine's nondeterminism (scheduling, clocks, network). Avoids the rewrite, and adds intelligent state-space search. But it is an external dependency, and you *still* "require external dependencies to be mocked or otherwise plugged" [B-antithesis-dst-primer-2024]. etcd ran its existing tests this way and got hard numbers: 830 wall-hours = 4.5 simulated years, surfacing a watch bug present in every stable release [A-etcd-antithesis-robustness-2025]. (Per `plans/121`: for greenfield Dorc the *interesting* thing is the build-it-in seam; the hypervisor is cheap-to-add-later defense-in-depth.)

### 5. What it actually buys (the receipts)
- FoundationDB: built the simulator for ~18 months *before* writing real storage; "swizzle-clogging" (clog each node's connections one-by-one, then unclog in random order) is their champion deep-bug finder [A-foundationdb-simulation-testing-2022].
- WarpStream: a data race that "had run in our regular CI workflows for literally 10s of thousands of hours… and not once ever caught this bug. Antithesis caught this bug in its first 233 seconds." [B-warpstream-dst-entire-saas-2024]
- TigerBeetle keeps assertions **on in production** — "far better to stop operating than to continue operating in an incorrect state" [A-tigerbeetle-vopr-2024] — the same fail-stop instinct as Dorc fail-fast.

### 6. The honest limits (why it is not magic — and these matter for Dorc)
- **The edges are untested.** You swap out the nondeterministic parts, so by construction "you are not actually testing the entirety of your code… there will always be the non-deterministic edges." Even with Antithesis, "you cannot test the integration between your system and external systems. You must mock out the external systems." [B-eatonphil-dst-bigdeal-2024] -SURE this is the crux for Dorc: the *real remote host* is exactly such an external system.
- **You only test the faults you imagined.** "the benefits of DST are tied to your understanding of the spectrum of behavior that may happen in the real world." [B-eatonphil-dst-bigdeal-2024] Mock the network too kindly and you test nothing.
- **"Is my DST even working?" is the real hard problem.** Will Wilson, in Eaton's piece: "it's terrifyingly easy to build a DST system that appears to be doing a ton of testing, but actually never explores very much of the state space… This process often resembles science more than it does engineering." Branch coverage is a poor signal; FDB/Antithesis use "Sometimes assertions" to check that interesting states are *reached*. [B-eatonphil-dst-bigdeal-2024]
- **Determinism is a spectrum**, not a binary — most DST shops ignore CPU/`malloc`/syscall-level nondeterminism and still win.

### 7. Why this is (probably) critical for Dorc — the bridge
~SUSPECT-to-+SURE: Dorc's "network" is SSH connections to remote hosts running sh; its worst bugs are exactly the interaction kind ("what does the orchestrator *do* when host 7 of 12 drops connection right after the mutation lands but before the ack?"). DST is the one technique that makes that question **deterministic, fast, and replayable** instead of a once-a-quarter heisenbug. The seam it demands — one chokepoint through which all remote execution flows (`notes/077`'s wrappable leaf-execution seam) — is cheap to reserve on a greenfield and, per §4, *impossible to bolt on later*. That asymmetry, not the simulator itself, is the day-1 decision (`axis-dst-cost`, deferred).

Two connections to commitments already on record:
- The untested **edges** (§6) are precisely where the day-1 *oracle contract* lives: Dorc can be maximally rigorous about the orchestration *logic* via DST, while honestly flagging that the mocked remote-host edge is best-effort. That is `concl-rigor` in action — "best-effort" ceilings the *edge*, not the *kernel*; the kernel gets provable-grade rigor.
- The **timed-out op that "may or may not have happened"** is the CAP third state Dorc already committed to (True/False/**Unknown**, `notes/112` f50) — and a DST seam is the natural place to *inject* it and *assert* the orchestrator handles it.

## Citations
> [B-eatonphil-dst-bigdeal-2024]:§Randomness-and-time / §single-thread / §Considerations (relevance: +1:SURE)
> DST merely assumes that you have a global seed for all randomness in your program and that the simulator controls the seed… Once you observe a bad state… you allow the user to enter the same seed again.
> since using blocking IO would mean an entire class of concurrency bugs could not be discovered while running the simulator in a single thread, we must limit ourselves to asynchronous IO. Single threaded and asynchronous IO. These are already two big limitations.
> because you must swap out non-deterministic parts of your code, you are not actually testing the entirety of your code… even with Antithesis you cannot test the integration between your system and external systems. You must mock out the external systems.
> it's terrifyingly easy to build a DST system that appears to be doing a ton of testing, but actually never explores very much of the state space of your system… This process often resembles science more than it does engineering.

> [A-foundationdb-simulation-testing-2022]:§Simulation (relevance: +1:SURE)
> Simulation is able to conduct a deterministic simulation of an entire FoundationDB cluster within a single-threaded process. Determinism is crucial in that it allows perfect repeatability of a simulated run.

> [B-antithesis-dst-primer-2024]:§How-do-I-implement (relevance: +1:SURE)
> this requires the system and all its dependencies to be built with deterministic simulation testing in mind, this approach is generally impractical for systems already in production.

> [A-tigerbeetle-vopr-2024]:§The-VOPR (relevance: +1:SURE)
> all non-deterministic parts of the system are stubbed out. This includes the clock, network, and disk operations. … One minute of VOPR time is equivalent to days of real-world testing.

> [A-etcd-antithesis-robustness-2025]:§How-We-Tested (relevance: -0:SUSPECT — subagent-graded)
> we ran 830 wall-clock hours of testing, which simulated 4.5 years of usage.

> [B-warpstream-dst-entire-saas-2024]:§bugs (relevance: -0:SUSPECT — subagent-graded)
> Our correctness tests had run in our regular CI workflows for literally 10s of thousands of hours… and not once ever caught this bug. Antithesis caught this bug in its first 233 seconds of execution.
