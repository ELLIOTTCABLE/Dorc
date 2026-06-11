# 221 — Execution substrates & independent adjudicators (deep-research, round 21)
<!-- /* slug corrected 2026-06-11: drafted under provisional title "notes/21I" before
being filed under the 22x research convention; 21I was never a filed note. Old slug
preserved here for grep. (note 217 §7) */ -->

**Stamp:** round 21 · 2026-06-11 · web deep-dive (deep-research agent). Feeds arch-7
("hostsim DST at scale — seeded-random book/oracle generation through the gate-5
differential harness", 211 §1) and the standing differential epistemology: *real dash
adjudicates the engine's beliefs about shell semantics; a self-built simulated dash
would test our beliefs against themselves — correlated error.*
**Grounding:** README/DESIGN (read in full); `spike/e2e/run.sh` (gate-5 argv-echo
differential, read in full) [A-dorc-run-sh-2026]; notes/122/123/124 (DST ladder, L0–L3)
and notes/020 (CoLiS verdict) — referenced, not re-litigated. NB: the priming prompt
named `21D-arch7-differential-harness.md`; that note does not exist at HEAD (21D was
"overflow" in the 211 §4 reservations and was never written) — the harness ground-truth
used here is run.sh itself plus 211/20K's gate-5 description.
<!-- /* OVERTAKEN 2026-06-11: 21D-arch7-differential-harness.md EXISTS now — the arch-7
build landed after this research note was drafted (commit 8d87e15). The NB was true at
writing; the harness ground-truth is now 21D itself. (note 217 §7) */ -->
**Method:** interactive-research discipline; ~33 sources surveyed, 12 full-read;
source-grades [A|B|C|D-slug-year]; all synthesis confidence-marked
(+SURE/~SUSPECT/-GUESS/--WONDER).

---

## §0 Conclusions up front

**concl-r1 (Smoosh).** Smoosh is real, good, and dormant: an executable small-step
semantics for POSIX sh in Lem/OCaml, the *only* shell with zero failures on the Open
Group's VSC-PCTS2016 shell suite (418 locale-independent tests) and on its own
161-case cross-shell suite; it found confirmed bugs in dash, yash, the POSIX *test
suite* (10), and the POSIX *spec* itself [A-smoosh-popl-2020]. It is a /bin/sh-shaped
executable, so it drops into a differential harness as just another shell column; build
is the weak point (OCaml+Lem+libdash, pinned versions, Docker strongly recommended;
last human commit Dec 2022) [A-smoosh-repo-2023]. One subtle anti-correlation gotcha:
Smoosh *parses with dash's own parser* (libdash), so in a dash+smoosh+engine ring it is
an independent adjudicator of *semantics* (expansion, evaluation, exit status) but a
*correlated* witness on parsing. Verdict: +SURE viable as a third adjudicator for the
value-plane (gate-5's plane), pinned via Docker, treated as bitrot-risky; its paper's
catalog of where-shells-disagree (non-lexical `break`, `getopts` hidden state,
`local`/scope, assignment-prefix-on-function) is a ready-made audit list for our
⊤-reject and refusal zones.

**concl-r2 (Hermit / DetTrace / rr).** The ptrace-determinization lineage is proven on
shell-shaped workloads and abandoned as products: DetTrace (ASPLOS 2020) made 12,130
Debian package *builds* (fork/exec/make/sh-heavy) bit-reproducible [B-dettrace-2020],
then died (last commit 2020-09); its successor Hermit is first-party "maintenance
mode... long tail of unsupported syscalls" [A-hermit-repo-2026] with only bot
dep-bumps since 2022 — but its own test suite runs `bash -c` pipelines under
`hermit run --verify`, and `--chaos --sched-seed=N` gives exactly *seeded* schedule
randomization (the thing arch-7 wants and rr does not offer). rr is a flake *capturer*,
not a seeded rail: chaos mode provokes scheduling bugs and the artifact is a
forever-replayable recording [A-rr-chaos-2016]; both rr and (~SUSPECT) Hermit's
preemption timing need hardware perf counters — likely excluding WSL2 and many CI VMs;
Linux bare-metal/KVM tier only. Verdict: real dash CAN run seed-deterministically
under Hermit on a Linux tier (+SURE in principle, ~SUSPECT in practice pending a smoke
test); but for *our* books (single-threaded dash, nondeterminism ≈ pipeline-stage
interleaving + PIDs + time + FS order), a namespaces/pinning/env-freeze tier captures
most determinism for ~zero cost — full userspace determinism is provably unreachable
anyway (rdrand is untrappable; hence Antithesis is a hypervisor)
[A-josnyder-determinism-2026].

**concl-r3 (Antithesis/FoundationDB school).** Three transferable lessons, all
boundary-shaped. (1) *The seam lesson*: FDB runs real production code above an
interface seam with simulated network/disk/time below it; everything below the seam is
a model whose fidelity is a standing risk, which FDB hedged with separate hardware
failure-testing ("combined regime") [A-fdb-testing-doc]. (2) *The dependency lesson*:
whatever sits OUTSIDE the deterministic boundary stays untested and becomes your bug
source — FDB famously deleted ZooKeeper and wrote their own Paxos to pull it inside
[A-antithesis-bugging-2024]; Antithesis exists to flip the boundary outward (a
deterministic hypervisor around the whole computer) so unmodified dependencies come
inside. Mapped to us: dash is our "real component" — the school's answer is *put the
real thing inside a determinized boundary; never hand-model it* (a simulated dash is
precisely the correlated-error the standing epistemology forbids). (3) *The BUGGIFY
lesson*: determinism is necessary but unproductive alone; the bug-finding power came
from white-box, in-code fault points (25%-armed per-run), knob randomization, and a
damage-control phase [A-buggify-2021]. Bug-classes only-full-determinism caught, per
practitioners: a data race missed by 10k+ CI-hours of Go race-detector runs, found in
233 s; a network-fault × microsecond-race compound found ~1/hr in sim
[A-warpstream-antithesis-2024]; FDB's swizzle-clogging class [A-fdb-testing-doc].

**concl-r4 (shells on wasm).** Further along than expected, and still the wrong rail
for adjudication. WASIX (Wasmer, 2023) adds fork/exec/wait to WASI; `proc_fork` does a
real full-memory-copy fork (or vfork mode) [A-wasix-fork-2023]; bash AND dash are
published wasm packages (`sharrattj/dash` on the Wasmer registry — dash-on-wasm
already exists) [B-wasmer-dash-pkg-2022], and secondary sources call bash/CPython
"proven to work" under WASIX [C-wasmruntime-2026]. Architecturally wasm is
deterministic-by-construction modulo imports (the embedder owns clock/random/fs), which
is the clean version of what Hermit fights for — BUT (a) WASIX is a single-vendor
"rogue superset" off the standards track (WASI Preview 2 has no fork; POSIX-compat is
an explicit non-goal, WASI#122); (b) Wasmer's host is not a deterministic simulator —
we'd write a custom seeded-scheduler host (~SUSPECT feasible, real project); and (c)
decisive: dash-compiled-for-WASIX is a *different artifact* from the dash on target
hosts — substrate-induced divergence (signals, job control, fds, /dev, umask) would
contaminate the adjudicator. Interesting tech; not our referee. -GUESS it becomes
relevant later for a browser-demo/`shtepper`-style DX toy, not correctness.

**concl-r5 (differential prior art beyond Smoosh).** The richest reusable asset is
Oils' spec-test corpus: ~225 `spec/*.test.sh` files (thousands of cases), Apache-2.0,
in a trivially parseable format (`#### case` + `## status/STDOUT` assertions) whose
per-shell qualifiers `## OK <shell>` / `## BUG <shell>` / `## N-I <shell>` encode
*adjudicated* cross-shell divergence — someone already ruled which shell is wrong, with
documented judgment norms ("if one shell disagrees with others, that is generally a
BUG") [A-oils-spec-2026]. Methodology is explicitly ours-shaped: "figure out how OSH
should behave by taking an automated survey of the behavior of other shells." Reuse
verdict: mine the corpus (dash-relevant files: errexit, exit-status, word-split,
command-sub, here-doc, case/if/loop), do NOT adopt the runner (sh_spec.py compares
shell stdout/status columns; our engine is an analyzer, not a shell — gate-5's
projection differs); precedent for cross-suite reuse is established (Oils' own CI runs
the Smoosh suite) [A-oils-quality-2025]. Elsewhere: ShellCheck never attempted
evaluation semantics (historically syntactic; v0.10+ ships an *optional* CFG/dataflow
pass with an off-switch for big scripts — semantic analysis as expensive add-on)
[B-shellcheck-man-2025]; CoLiS's interpreter was independently measured at 8/161 on
Smoosh's suite (a core-calculus elaboration is not a usable shell — do not ring it),
though Morbig (their independent parser) remains attractive as a *parse-layer* referee
[A-smoosh-popl-2020][B-colis-sttt-2022-blocked]; the Open Group VSC-PCTS2016 shell
suite is alive (v3.1, Nov 2025), free for 12 months to open-source implementations,
and non-redistributable (Smoosh could only publish result journals)
[A-opengroup-testsuites-2026]; there is NO open-source POSIX-shell conformance suite —
a 2024 unix.SE question asking for one has zero answers [B-unixse-769818-2024]; the
open "POSIX Test Suite" covers system interfaces, not sh. Modernish's BUG_*/QRK_*
capability catalog and Mascheck's pages (`set -e`, `"$@"`, echo/printf, IFS, cmd-subst)
are curated where-shells-disagree knowledge bases worth mining, not executing
[B-modernish-2025][B-mascheck-pages].

**concl-r6 (ranking).** Two needs, ranked by effort-to-value with anti-correlation
explicit (details §6):

| rank | option | need | effort | value | anti-correlation | verdict |
|---|---|---|---|---|---|---|
| 1 | harness-discipline tier: env-freeze + sandbox/namespaces + 1-cpu pin around real dash (formalize what run.sh half-does) | det-rail | S | high | n/a (dash stays real) | adopt r22 |
| 2 | Oils spec-corpus mining → dash-adjudicated cases for sem/value-plane | referee-corpus | S | high | corpus authored outside our beliefs | adopt r22 |
| 3 | second/third shell columns across lineages (dash[ash] + yash + mksh; busybox-ash is dash-correlated — same Almquist lineage) | referee | S–M | med-high | independent codebases; lineage caveat | adopt when Linux CI exists |
| 4 | Smoosh as third adjudicator (Docker-pinned), value-plane only | referee | M | high | semantics independent; parser = libdash (dash-correlated) | evaluate r22+ |
| 5 | Hermit smoke-test: `hermit run --verify -- dash book.sh`; if green, seeded-chaos rail for pipeline interleavings | det-rail | M | med-high | n/a | evaluate (1-day spike, Linux x86_64) |
| 6 | Morbig as parser-referee vs our parser (offline, corpus-wide) | referee (parse) | M | med | fully independent parser | evaluate; parser is our highest-risk surface |
| 7 | rr chaos as flake-capturer for rare gate-5 disagreements | det-rail (capture) | S–M | targeted | n/a | keep in pocket (Linux bare-metal) |
| 8 | VSC-PCTS2016 free 12-mo license, private conformance audit of `syntax::sem` claims | referee (de-jure) | M (paperwork) | med | independent, authoritative | defer until we publicly claim dash-compat |
| 9 | BUGGIFY-style fault points in hostsim + book-generator knob randomization | (arch-7 quality) | M | high | n/a (in-sim by design) | adopt as arch-7 design input |
| 10 | WASIX/wasm dash custom deterministic host | det-rail | L | low-med | tests a *different dash artifact* — contaminated | avoid |
| 11 | Antithesis SaaS | det-rail | $$$/L | overkill | n/a | avoid now; lessons already taken |
| 12 | full-system emulation (container2wasm / v86 / qemu-icount) | det-rail | L | low | real binary, heavy substrate | avoid |

---

## §1 Smoosh (r-1 evidence)

Primary: the POPL 2020 paper, §7–§9 read in full via ar5iv [A-smoosh-popl-2020]; repo
README + tree + commit log [A-smoosh-repo-2023].

- **What it is.** Executable small-step operational semantics of the POSIX shell
  standard, written in Lem (compiled to OCaml), parsing via **libdash** — dash's
  parser extracted as a linkable library with OCaml bindings. Runs in *system mode*
  (real syscalls; a usable, slow shell) and *symbolic mode* (their symbolic stepper /
  the web "Shtepper"). Symbolic mode's FS model tracks hierarchy but not file
  contents — "a matter of engineering effort" to extend; their in-paper scheduler for
  pipelines is deliberately simplified ("determinism without rigidity").
- **Conformance evidence.** Three suites: (1) Open Group VSC-PCTS2016 v2.15 shell
  suite — 494 tests, 418 usable locale-independent; Smoosh is the only shell tested
  with zero failures. (2) Modernish diagnostic — Smoosh shows no quirks, one bug
  (multibyte/8-bit chars — inherited limitation around the dash parser). (3) Their own
  161 external system tests (script + expected stdout/stderr/status; 2 speed, 67
  builtins, 2 parsing, 82 semantics, 8 sh-interface): Smoosh passes all; the suite
  runs against ANY shell via `TEST_SHELL=dash make -C tests` — i.e. it *is* a small
  cross-shell differential harness already.
- **As test oracle, what it caught.** dash: arithmetic-expansion mishandling of
  unset/empty vars; `times` output; empty-alias handling (patches submitted). yash:
  async commands' stdin not redirected to /dev/null; `fg` over-output. POSIX test
  suite: 10 confirmed bugs. POSIX spec: typos + the non-lexical `break` and `getopts`
  ambiguities raised with the Austin Group. OSH (2019): 77/161 failures, plus a
  scoping/getopts bug fixed upstream. CoLiS interpreter: 8/161.
- **Where shells disagree (their synthesis).** "Implementations of the POSIX shell do
  not have nearly the same level of agreement as JavaScript interpreters do...
  [shells] frequently disagree in corner cases. Since none of these shells is
  perfectly POSIX compliant, we declined to precisely match any of them." Catalogued
  zones: non-lexical `break`/`continue` (unspecified; old-bash/zsh quirk), `getopts`
  grouped-option hidden state (every shell hides it differently; yash exposes
  `OPTIND=1:2`), `local`/readonly-shadowing/nested-scope (half the shells nest,
  half don't; `local -p` divergence), prefix-assignments on function calls
  (persistence unspecified). +SURE these map onto our ⊤-trigger/refusal audit:
  constructs whose *cross-shell envelope is open* are constructs our engine must
  never claim a Must-fact about.
- **Embeddability.** Build: OCaml + opam pins + Lem + libdash submodule + autotools;
  README itself recommends Docker/Vagrant because "Smoosh depends on many parts and
  specific versions of some libraries". CI exists (GitHub Actions badge). Speed: ~4×
  slower than C shells on the POSIX suite — irrelevant at our corpus sizes (+SURE).
  Interface: a `smoosh` executable on PATH — a drop-in shell column for run.sh-style
  harnesses; no library embedding needed. Activity: last human commit 2022-12-06; one
  community build-fix PR merged 2023-02; no successor project found (searched).
  ~SUSPECT the Docker image bitrots slowly (Debian 9 base in the artifact-era docs);
  pinning a built image is the mitigation.
- **Anti-correlation analysis (the load-bearing nuance).** Ring = {dash, Smoosh,
  engine}: engine shares no code/beliefs with either (+SURE; our parser and
  `syntax::sem` are homegrown). Smoosh-vs-dash: *parse* layer literally shared
  (libdash), so parse-level disagreement is structurally invisible between them;
  *semantics* fully independent (Lem vs dash's C). Consequence: Smoosh referees the
  value/expansion/status plane (exactly gate-5's plane); for parser beliefs we need a
  different referee (Morbig, §5) or shell-matrix behavioral probes.

## §2 Hermit, DetTrace lineage, rr (r-2 evidence)

- **DetTrace** [B-dettrace-2020]: ptrace/seccomp "reproducible container"; all
  computation a pure function of initial FS state; flagship result: 12,130 Debian
  package builds (800M+ LoC) made bit-for-bit reproducible *unmodified* — package
  builds are fork/exec/make/sh-saturated, so this is the existence proof that
  ptrace-determinization handles shell process-trees (+SURE). Repo dead since
  2020-09 (verified via commit log); the people/ideas continued into Meta's Reverie →
  Hermit.
- **Hermit** [A-hermit-repo-2026]: README warning verbatim: "no longer under active
  development within Meta and is in maintenance mode. There is a long tail of
  unsupported system calls that may cause your program to fail." Commit feed since
  2022 is exclusively meta-codesync bot version bumps (verified) — the repo *looks*
  alive in feeds but isn't. Mechanism: Reverie syscall interception; serialize all
  threads/processes onto one logical CPU; deterministic next-thread choice; preemption
  measured in retired-conditional-branches via the CPU PMU; seeded virtual time and
  RNG. Capabilities relevant to us: `hermit run --verify` (run twice, compare);
  `--chaos --sched-seed=N` (seeded schedule exploration — *seed-reproducible chaos*);
  `hermit analyze --search` (find failing/passing schedule pairs and diff them);
  `record`/`replay`. Fork-heavy evidence: its own tests determinize
  `bash -c 'find ...'` pipelines and vfork+exec C tests (verified in-tree). Scope:
  x86_64 Linux only (aarch64 "in progress", presumably frozen); needs fixed FS image +
  no external network as preconditions.
  - ~SUSPECT constraint: RCB-preemption needs working perf counters → WSL2 and
    PMU-less cloud VMs likely out (rr documents the same constraint class); for
    *non-preemptive* fully-cooperative workloads (sequential dash forks) Hermit may
    work without precise PMU, but I found no doc either way — flag for the 1-day
    smoke spike, do not assume.
- **rr chaos mode** [A-rr-chaos-2016]: designed from real Mozilla flakes; key insight
  was that pure runnable-thread permutation (CHESS-style) misses starvation bugs —
  chaos mode instead randomizes priorities AND inserts intervals where low-priority
  threads don't run at all (≤20% of runtime). Role-shape for us: not a seeded rail —
  the *recording* is the reproducer (replay forever, reverse-execute); chaos exposes
  interleaving bugs during recording. rr records whole process trees (dash + children
  fine, +SURE). Linux + perf counters required.
- **DIY tier** [A-josnyder-determinism-2026] (Apr 2026, practitioner, full-read):
  namespaces (net/mount), no threads, mock clock/getrandom/getpid, ASLR off,
  vdso/vvar zeroing, then the long tail: `rseq` (pin CPU), AT_RANDOM (zero via
  ptrace), `rdtsc` (PR_SET_TSC trap+emulate), `cpuid` (ARCH_SET_CPUID — *reset by
  every execve*, relevant for fork/exec-heavy shells), and `rdrand` — **no trap
  mechanism exists**; hypervisor or binary-rewriting only. "Deterministic computing is
  impossible in x86 [userspace]... now we understand why tools like Antithesis are
  hypervisor-based." Also: "if your language runtime is mostly single threaded... you
  have a massive leg up."
- **Synthesis for the dash rail.** A dash book's nondeterminism surface, observed at
  gate-5's plane (argv/rc/stdout), is small: pipeline-stage interleaving (the known
  tc-pipe-ran-order, ~1-in-15), `$$`/`$!` PIDs, time, FS enumeration order, locale/env
  leakage. None of rdtsc/rdrand/AT_RANDOM *reaches the observable plane* of a dash
  script that doesn't call binaries consuming them (-GUESS, qualified: mock shims are
  ours, dash itself reads none of these into script-visible state ~SUSPECT — dash has
  no $RANDOM). So: env-freeze (LC_ALL/TZ/umask/HOME pinned), PATH=mocks (already),
  sandbox cwd (already), PID namespace (`unshare -p` → deterministic-ish PIDs), 1-cpu
  pin, no-net namespace ≈ a reproducible-enough rail for corpus-scale DST *without*
  determinizing stage interleaving; Hermit is the only found tool that adds *seeded
  exploration* of that interleaving (turn RAN_ORDER=lax cases back into asserted-
  under-all-schedules cases).

## §3 Antithesis / FoundationDB school (r-3 evidence)

- **The seam architecture** [A-fdb-testing-doc][A-buggify-2021]: same production code
  runs in sim and reality; only the I/O implementations behind interfaces are swapped
  (Flow's Sim2: network, disk, time). Simulation modeled disk fullness, machine
  reboots/deaths, datacenter failures; "swizzle-clogging" (stop a random subset's NICs
  one-by-one, unclog in random order) won their internal competition for
  deepest-bug-finding fault pattern. Volume: ~"one trillion CPU-hours" equivalent
  claimed; ~10:1 sim-time compression. The page's opening line is the hedge that
  matters: a *combined regime* of simulation + live perf testing + hardware-based
  failure testing — i.e. even FDB never trusted the sim's model of the world alone;
  the model below the seam is itself a belief that needs real-world calibration.
- **The boundary/dependency lesson** [A-antithesis-bugging-2024] (Wilson, first-party
  FDB history, full-read): sim-first ("wrote a fully-deterministic event-based network
  simulation *before* the database"); "found all the bugs" (1–2 customer-reported bugs
  ever; aphyr declined to Jepsen it); and the part most quoted at us: "We deleted all
  of our dependencies (including Zookeeper) **because they had bugs**, and wrote our
  own Paxos" — dependencies outside the deterministic boundary are where the bugs
  live; either pull them inside or own the consequences. Antithesis productized the
  inverse: a deterministic *hypervisor* so unmodified software (whole Docker container
  sets) sits inside the boundary; per their docs the trade is mocking/plugging
  whatever still crosses it (external networking), amd64 containers, single-core
  hypervisor instances [A-antithesis-dst-docs-2025][C-whynow-dst-2026].
- **Bug-classes that needed full determinism** (asked-for specifically):
  - WarpStream [A-warpstream-antithesis-2024]: (a) a data race in a *metrics
    instrumentation library* — present from month one, missed by "10s of thousands of
    hours" of CI with the Go race detector enabled, found by Antithesis in 233 s of
    execution; class = races in code adjacent to yours that example-based tests never
    stress. (b) A regression where a failed-flush file was, for <1 µs of program
    order, visible as commit-ready to a 5 ms-poll background goroutine — network fault
    × scheduling window compound; sim hit it ~1/wall-clock-hour; class = fault ×
    interleaving conjunctions with effectively-zero natural probability per run but
    certainty at fleet-scale. (c) Their meta-finding: coverage/behavior-guided
    branching ("snapshot at interesting behavior, explore branches") is what made the
    hypervisor *productive*, echoing BUGGIFY.
  - FDB: the swizzle-clogging class — "deep issues that only happen in the rarest
    real-world cases" — discoverable only because schedules/faults were both
    controllable and replayable.
- **BUGGIFY** [A-buggify-2021]: "deterministic simulation framework with random fault
  injection... *can* find bugs. The question is how quickly?" Black-box faults match
  low-level dangers (packet loss) but not high-level dangerous *situations* (minimal
  quorum twice in a row). FDB's answer: ~white-box fault points compiled into the
  production binary, only armable in sim; each point enabled-or-not per run (then 25%
  per evaluation); knob randomization (748 knobs; constants promoted to knobs *so they
  could be buggified*); `speedUpSimulation` damage-control phase (stop injecting,
  let the system prove it can recover). Lesson restated for arch-7: hostsim seeds
  alone won't be productive; the *generator and hostsim need first-class sabotage
  points* (oracle-lies, probe-corruption, partial-host mutations, knob-randomized
  book shapes), plus a recover-and-assert phase.
- **Mapping onto the standing epistemology** (+SURE this is the right frame): our
  engine kernel + hostsim is the FDB-style side (DI seams, deterministic, our
  beliefs); real dash is the component the school says to *bring inside the boundary
  unmodified* (Hermit/namespace tier), never to re-model — re-modeling it is exactly
  the ZooKeeper-shaped mistake plus the correlated-error mistake at once. And per
  FDB's own hedging, even with dash in-ring, the harness's *model of the host*
  (mocks, probe-results fixtures) stays a belief needing occasional real-host
  calibration — the cm-1 product-gate direction, not a reason to distrust gate-5.

## §4 Shells on wasm (r-4 evidence)

- **WASIX** [A-wasix-fork-2023][B-hn-wasix-2023]: Wasmer's POSIX-flavored superset of
  WASI Preview1: adds `proc_fork` (true fork with full linear-memory copy;
  `copy_memory=false` ⇒ vfork semantics until `proc_exec`), exec, wait, threads,
  signals(-ish), longjmp. First-party demo: wasmer.sh is bash-compiled-to-wasm piped
  to xterm.js; `wasmer run <pkg>` forks subprocesses. **`sharrattj/dash` exists** on
  the Wasmer registry ("Dash is a modern POSIX-compliant implementation of /bin/sh")
  [B-wasmer-dash-pkg-2022]; busybox-on-WASIX builds exist [C-busybox-wasix-2024];
  2026 secondary: "CPython, Bash, PostgreSQL have all been proven to work"
  [C-wasmruntime-2026].
- **Standards posture**: WASI itself rejected POSIX-compat as a goal (WASI#122,
  2019); WASIX is single-vendor and called a "rogue superset" in the academic
  literature [C-lindwasm-arxiv-2025]. WASI Preview 2 has no fork. ~SUSPECT WASIX is
  demo-grade beyond the showcase apps: job control, signal delivery timing, tty
  semantics, fd inheritance corners are exactly where dash's behavior would diverge.
- **Determinism angle**: wasm's import-boundary makes the embedder the owner of every
  nondeterminism source — architecturally cleaner than ptrace-chasing (the josnyder
  list simply doesn't exist inside wasm; no rdtsc/rdrand/cpuid). A custom WASIX host
  with an FDB-style event loop scheduling forked instances would be a *genuinely
  seed-deterministic dash* (~SUSPECT feasible; nobody found doing it; real
  engineering).
- **Why it still loses for adjudication** (+SURE): the artifact under test would be
  dash-compiled-to-wasm-against-WASIX-libc — a different build, different libc,
  different syscall surface than the `/bin/dash` on any target host. The differential
  ring exists to test our beliefs against *the deployed reality*; a substrate that
  itself perturbs shell-observable behavior injects exactly the silent divergence the
  harness is meant to catch. Full-system emulation (container2wasm, v86, linux-wasm
  [C-linux-wasm]) fixes artifact-fidelity (real dash binary on an emulated kernel) at
  the cost of a huge, slow, clock-injection-still-needed substrate — C-tier curiosity.

## §5 Differential prior art beyond Smoosh (r-5 evidence)

- **Oils/OSH spec tests** [A-oils-spec-2026] (spec/README.md, wiki page, sh_spec.py
  header, errexit.test.sh — all read):
  - *Scale/shape*: ~225 files in `spec/`, each tens of cases ⇒ thousands of cases
    (-GUESS 3–6k; countable mechanically on import). File metadata:
    `## compare_shells: bash dash mksh ash` (available matrix: ash bash bash-4.4 dash
    mksh zsh zsh-5.9 yash), `## oils_failures_allowed: N`, `## suite:`.
  - *Case format*: `#### name`, body is plain sh, assertions `## status:`,
    `## stdout:`/`## stdout-json:` (JSON for exactness), `## STDOUT: ... ## END`;
    per-case cwd isolation (`_tmp/spec-tmp/<file>/<case>-<shell>`).
  - *The treasure — adjudication vocabulary* (sh_spec.py header, verbatim semantics):
    PASS (ideal) / OK (not ideal but accepted) / N-I (not implemented) / BUG
    (verified known-bug value) / FAIL; "If ALL shells agree on a broken behavior,
    they are all marked OK... if the behavior is NOT POSIX compliant, then it will be
    a BUG. If one shell disagrees with others, that is generally a BUG." This is a
    hand-adjudicated divergence map across the exact shell matrix we care about.
  - *Method statement* (wiki): "figure out how OSH should behave... by taking an
    automated survey of the behavior of other shells" — i.e. the discipline arch-7
    generalizes (survey real dash instead of trusting ourselves).
  - *License*: Apache-2.0 (repo LICENSE.txt read). *Precedent*: Oils CI runs the
    Smoosh suite as an external conformance corpus [A-oils-quality-2025] — cross-suite
    borrowing is community-normal.
  - *Reuse boundary* (+SURE): the runner treats columns as shells (spawn snippet,
    compare streams); our engine can't be a column. Reuse = corpus-mining into our
    e2e/sem shapes, not harness adoption. See §7.
- **ShellCheck**: historically syntactic/AST-heuristic (the Smoosh paper's
  characterization: "purely syntactic"); since ~v0.10 ships optional "extended
  dataflow analysis" with a documented opt-out for 2000+-line scripts because of
  CPU/RAM cost [B-shellcheck-man-2025]. Lesson we take: the most successful shell
  static-analyzer in existence ships semantic analysis as a *bounded optional
  extra* and makes no evaluation-semantics claims at all — corroborates our
  modeled-subset/⊤-reject posture rather than challenging it. (No deep-dive
  retrospective from the maintainer surfaced in this pass; vidarholen.net has a
  shellcheck tag worth a later skim [C-vidarholen-blog].)
- **Morbig/CoLiS**: the STTT 2022 platform retrospective [B-colis-sttt-2022-blocked]
  is the right lessons-learned source but was inaccessible this session (HAL is
  behind an Anubis bot-wall; Springer paywalled; PMC carries only front/back matter)
  — **human-fetch flag**, see §7. What stands without it: CoLiS scaled to ~28k
  Debian maintainer scripts, filed 150+ policy-violation bugs, with a Why3-verified
  symbolic interpreter over feature-tree constraints and hand-written UNIX-utility
  specs; their *concrete* interpreter, measured externally, passed 8/161 Smoosh tests
  (unsupported features dominated) [A-smoosh-popl-2020] — a core-calculus elaboration
  is not a shell, so CoLiS is not ring material. Morbig (SLE 2018; JCL 2020) is an
  independent static POSIX parser — the only credible independent *parse-layer*
  referee found (Smoosh's is dash's; Oils' is OSH-integrated). Our notes/020 verdict
  (engine/oracle split = crib; relational-FS machinery = drop; calibration =
  differential+property testing, not proof) stands unmodified by this pass.
- **POSIX conformance suites** [A-opengroup-testsuites-2026]: VSC-PCTS2016 v3.1
  released 2025-11-11 — actively maintained; latest versions free to organizations
  submitting for certification; separately, a "twelve month free license" is offered
  to open-source projects implementing the standards (page text; mechanics via
  obconformance@opengroup). Non-redistributable (Smoosh's artifact: "The POSIX test
  suite cannot be distributed... we have permission to distribute the resulting
  journals" — those journals live in `smoosh/posix-journals`, a free secondary
  artifact we *can* read). The "Open POSIX Test Suite" (sourceforge-era, mirrored by
  bytecodealliance/emscripten) is IEEE 1003.1 *System Interfaces* (threads, timers,
  mqueues) — not sh. Gap confirmation: unix.SE 769818 ("test harness that can check
  how POSIX compliant a shell is") — zero answers since Feb 2024
  [B-unixse-769818-2024].
- **Divergence catalogs (knowledge, not executables)**: Modernish's install-time
  diagnostic detects per-shell capabilities/quirks/bugs under stable IDs (README
  "Appendix A: List of shell cap IDs") [B-modernish-2025] — the densest
  machine-checkable encoding of real-shell brokenness found; Mascheck's pages
  [B-mascheck-pages] catalog `set -e` semantics history, `"$@"` edge cases,
  echo/printf portability, IFS, `$( )`-vs-`)` parsing, plus Bourne/Almquist family
  *genealogies* — the latter grounding the lineage-correlation argument in §6 (dash
  and busybox-ash are both Almquist descendants; agreement between them is weaker
  evidence than dash↔yash agreement).

## §6 Ranking rationale (r-6, expands the §0 table)

**The anti-correlation requirement, made precise.** A referee adds information
proportional to its belief-independence on the plane being adjudicated. Taxonomy of
candidate pairs:
- engine ↔ dash: independent on every plane (+SURE). The incumbent gate-5 pairing is
  already the strongest available two-node ring.
- Smoosh ↔ dash: parse-plane *identical* (libdash); semantic plane independent.
  Smoosh therefore strengthens the ring only on semantics — which is fine, because
  that's where gate-5 lives; but a Smoosh+dash agreement on a *parse* question is one
  vote, not two.
- busybox-ash ↔ dash: same Almquist lineage [B-mascheck-pages] — correlated
  implementations; prefer yash (independent, strict-POSIX) and mksh (Korn lineage)
  for matrix columns; bash only in `sh`-mode and only as a fourth opinion.
- hostsim/engine-generated expectations ↔ engine: same-author beliefs — never
  adjudicative, only generative (the standing epistemology, unchanged; hostsim's job
  per §3 is *sabotage*, not truth).
- Oils corpus ↔ engine: authored outside our beliefs, adjudicated against the same
  shell matrix we'd use — high-information test *inputs* even before any new referee
  binary is added.

**Why the top two are the top two.** Both are S-effort and compound immediately:
rank-1 hardens the rail the harness already drives every round (env-freeze +
namespaces formalizes determinism we currently get by luck; it is also the substrate
any later Hermit/rr tier wraps); rank-2 multiplies corpus pressure on `syntax::sem`
and the value plane with cases that real-shell maintainers already fought over —
the cheapest possible import of decades of divergence knowledge.

**Why Smoosh is evaluate-not-adopt.** High ceiling (the only zero-failure POSIX
artifact in existence; symbolic stepper as a side-benefit), but M-effort with bitrot
risk (dormant 3.5 y; OCaml/Lem pin stack), and its marginal value over a
dash+yash+mksh matrix is concentrated in *spec-versus-implementations* questions
(where shells are all wrong, Smoosh knows the standard) — valuable exactly when we
firm up `syntax::sem` claims, which is a r22+ shape, not a today-blocker.

**Why Hermit is a 1-day spike, not a plan.** If `hermit run --verify -- dash book.sh`
works on a stock Linux x86_64 runner, we gain (a) a determinism *checker* for the
rail (verify-mode as a CI canary: "this book's observable plane is
schedule-independent") and (b) seeded interleaving exploration that converts
RAN_ORDER=lax pipeline cases into asserted-under-all-seeds cases. If the syscall
long-tail bites dash/mocks immediately, we walk away having spent a day. ~SUSPECT
even odds; maintenance-mode means failures will be ours to debug or abandon.

## §7 Design consequences for r22+

- **dc-1 (adopt, S): formalize the determinism rail at the harness level.** Add to
  run.sh-or-successor: pinned `LC_ALL=C TZ=UTC umask`, scrubbed env allowlist, and —
  on Linux runners — `unshare -rpn`-style PID/net isolation + single-CPU pinning
  around every dash exec. Document the *residual* nondeterminism (pipeline stages;
  `$!`/`$$` if a book reads them) as the explicit lax-set instead of folklore. This is
  the josnyder/DetTrace lesson scaled to our actual surface, and it is a precondition
  for arch-7's "seeded-random generation" being debuggable at all.
- **dc-2 (adopt, S–M): arch-7 generator gets BUGGIFY-shaped sabotage points, not just
  seeds.** Per §3: seeded book/oracle generation + black-box faults will plateau;
  design the generator/hostsim with named, per-run-armed sabotage classes (oracle
  lies about a channel; probe record corrupted/dropped; host state mutated between
  probe and apply within the modeled set; knob-randomized book shapes: nesting depth,
  loop widths, errexit-region density) and a recover-and-assert tail phase. Each
  sabotage class maps to a named invariant (kFAIL-perform / kFAIL-withhold /
  inv-probe-sourced-values) the run must then prove it honored.
- **dc-3 (adopt, S): Oils corpus import, concretely.** A one-shot (re-runnable)
  importer that: filters `spec/*.test.sh` to files with `dash` ∈ compare_shells;
  extracts `####` cases whose bodies parse inside our modeled subset (parser as the
  filter — cases that ⊤-reject are *also* interesting as trigger-set regression
  pins, keep them in a second bin); emits (a) `syntax::sem`/value-plane differential
  inputs adjudicated by OUR dash runs (never trust the recorded goldens blindly —
  shell versions drift; the `## BUG dash`-annotated cases are the priority bin
  because they mark adjudicated dash-vs-POSIX divergence we must decide policy on),
  and (b) e2e-corpus candidates only where a case is book-shaped. Apache-2.0
  attribution: NOTICE line + per-case provenance comment. Explicit non-goals: do not
  adopt sh_spec.py; do not treat OSH itself as a needed column yet.
- **dc-4 (evaluate, M): Smoosh column behind a Docker pin.** Build once, freeze the
  image digest, add as an *optional* third column to the gate-5 plane (argv-echo and
  rc/stdout compare on the bare book under mocks); triage policy pre-agreed: a
  dash↔Smoosh disagreement is *spec-ambiguity evidence* (file into the ⊤/refusal
  audit), not automatically an engine bug. Also import `smoosh/tests/shell` (162
  cases, runnable against any `TEST_SHELL`) the same way as dc-3 — it's
  redistributable, unlike the POSIX suite, and Oils already established the reuse
  precedent. And read `smoosh/posix-journals` (free, distributable result journals
  of the real POSIX suite) for a no-license taste of the de-jure suite's coverage.
- **dc-5 (evaluate, 1-day): Hermit smoke spike** per §6. Success criterion is binary:
  `--verify` green on three representative corpus books (incl. one piping book) on a
  stock x86_64 Linux runner. Record PMU/WSL2 findings either way — they price every
  future determinism-tier discussion.
- **dc-6 (evaluate, M): Morbig as parse-referee.** Offline job: run Morbig over the
  e2e corpus + generated books; compare accept/reject + coarse tree shape against our
  parser (exact AST isomorphism is not the bar; refusal-parity and word-boundary
  parity are). Catches parser-belief errors dash-execution structurally cannot
  surface (we never see dash's parse). Priced M for OCaml toolchain + mapping
  shim; CLAUDE.md already names the parser our highest-risk surface, which is what
  justifies a second independent witness there.
- **dc-7 (defer): VSC-PCTS2016 12-month license** — paperwork when `syntax::sem`'s
  claims firm into something resembling "dash-compatible modeled subset"; run
  privately, publish journals only (Smoosh precedent). **(avoid):** WASIX/wasm rail
  for adjudication (artifact-fidelity poison, §4); Antithesis (overkill; lessons
  already absorbed); any hand-written simulated dash (the correlated-error
  anti-goal, now with the FDB-ZooKeeper story as its parable).
- **dc-8 (process): two human-pending items.** (h-1) CoLiS STTT 2022 lessons-learned
  full text is bot-walled (HAL Anubis) + paywalled (Springer) — if wanted, a human
  browser fetch of hal-03737886 takes a minute; the TACAS 2020 OA chapter PDF also
  resisted scripted download (UA-gated). (h-2) If the Hermit spike is greenlit it
  needs a real Linux x86_64 box (the rackmounted PC qualifies; WSL2 ~SUSPECT does
  not, PMU).

## §8 Graded source list

Full-read (load-bearing):
- [A-smoosh-popl-2020] Greenberg & Blatt, "Executable Formal Semantics for the POSIX
  Shell", POPL 2020 — §§6–10 read via ar5iv (arxiv 1907.05308). Conformance tables,
  bugs-found, divergence catalog, related-work assessments (CoLiS 8/161; ShellCheck
  "purely syntactic").
- [A-smoosh-repo-2023] github.com/mgree/smoosh — README full; tree; commit log (last
  human commit 2022-12-06; build-fix PR 2023-02-16). BUG.*/UNK.* root files;
  posix-journals/; modernish + libdash submodules.
- [A-hermit-repo-2026] github.com/facebookexperimental/hermit — README full
  (maintenance-mode warning; RCB/PMU scheduling; chaos --sched-seed); commit log
  (bot-only since 2022); in-tree tests running `bash -c` under --verify (code search).
- [A-dorc-run-sh-2026] spike/e2e/run.sh @ HEAD — gate-1..5 mechanics; gate-5
  one-directional argv⊆logged discipline; RAN_ORDER=lax carve-out.
- [A-josnyder-determinism-2026] josnyder.com/blog/2026/deterministic.html (Apr 2026)
  — DIY determinization ladder; rseq/AT_RANDOM/rdtsc/cpuid/rdrand specifics; "Hermit
  (Facebook OSS, abandoned)"; companion post deterministic_vmm.html not read.
- [A-rr-chaos-2016] robert.ocallahan.org "Introducing rr Chaos Mode" — design
  rationale incl. why runnable-thread permutation fails; recording-as-artifact.
- [A-fdb-testing-doc] apple.github.io/foundationdb/testing.html — Simulation/Flow
  seam; swizzle-clogging; combined regime incl. hardware failure testing.
- [A-antithesis-bugging-2024] antithesis.com/blog/is_something_bugging_you (Wilson)
  — FDB sim-first history; ZooKeeper deletion; aphyr; hypervisor rationale.
- [A-buggify-2021] transactional.blog/simulation/buggify (Alex Miller) — BUGGIFY
  rules/usages; knob randomization; speedUpSimulation; "black box solutions"
  contrast.
- [A-warpstream-antithesis-2024] warpstream.com blog "DST for our entire SaaS" —
  233s-vs-10k-CI-hours race; flush/commit compound bug; behavior-search plateau at
  ~160 app-hours; whole-system-boundary reasoning.
- [A-oils-spec-2026] oils-for-unix/oils: spec/README.md (full), wiki/Spec-Tests
  (full), test/sh_spec.py header (PASS/OK/N-I/BUG semantics, verbatim),
  spec/errexit.test.sh (format exemplar), LICENSE.txt (Apache-2.0), spec/ tree (~225
  files).
- [A-opengroup-testsuites-2026] posix.opengroup.org/testsuites.html (page dated
  2026-02-17) — VSC-PCTS2016 v3.1 (2025-11-11); free-for-certifiers; 12-month
  open-source license offer (page text via search snippet of same URL; fetched page
  truncated above that line ~SUSPECT on exact wording, +SURE on existence).
- [A-wasix-fork-2023] wasix.org docs proc_fork (full) + wasmer.io/posts/announcing-
  wasix (snippet: wasmer.sh = bash-on-wasm) — fork/vfork semantics on wasm.
- [A-antithesis-dst-docs-2025] antithesis.com/docs/resources/deterministic_simulation
  _testing (full) — DST definition; FDB+AWS provenance; pluggable-vs-hypervisor
  implementation split; "external dependencies be mocked or otherwise plugged".

Partial-read / verified-existence (B); snippets-only (C):
- [B-dettrace-2020] "Reproducible Containers" ASPLOS 2020 (dl.acm abstract +
  gatowololo PDF snippet: 12,130 Debian builds) + github.com/dettrace/dettrace commit
  log (dead 2020-09).
- [B-oils-quality-2025] oils.pub/release/0.36.0/quality.html (full page, but it is an
  index) — OSH Survey; wild tests; *Smoosh suite in Oils CI*.
- [B-shellcheck-man-2025] shellcheck.1.md / arch man page — extended-analysis
  dataflow toggle + cost note.
- [B-wasmer-dash-pkg-2022] wasmer.io/sharrattj/dash — package exists, wasi interface.
- [B-mascheck-pages] in-ulm.de/~mascheck index (full) — set -e, "$@", echo/printf,
  IFS, cmd-subst, Bourne/ash genealogies (subpages not deep-read).
- [B-modernish-2025] modernish README appendix-A existence (search snippet) + heavy
  corroboration via Smoosh paper's use of it.
- [B-unixse-769818-2024] unix.SE 769818 — unanswered conformance-harness question.
- [B-colis-sttt-2022-blocked] Becker/Jeannerod/Marché/Régis-Gianas/Sighireanu/Treinen,
  STTT 2022 (springer abstract + author/ref matter; HAL Anubis-walled; TACAS 2020 OA
  chapter CC-BY confirmed but PDF download UA-gated). Human-fetch flagged (dc-8).
- [B-hn-wasix-2023] HN 36126032 WASIX thread — fork/exec/wait addition framing.
- [B-golangnuts-rr-2024] golang-nuts thread — rr chaos applicability framing,
  recording efficiency.
- [C-wasmruntime-2026] wasmruntime.com WASI-vs-WASIX 2026 — "CPython, Bash,
  PostgreSQL proven"; marketing-adjacent.
- [C-lindwasm-arxiv-2025] arXiv 2312.03858 — WASIX "rogue superset" characterization.
- [C-whynow-dst-2026] whynowtech substack on Antithesis — single-core-per-hypervisor
  detail.
- [C-busybox-wasix-2024] MentalGear/busybox-wasm-wasi; [C-linux-wasm]
  joelseverin.github.io/linux-wasm; [C-vidarholen-blog] vidarholen.net shellcheck tag
  (unread); [C-materializedview-2024] open-source-deterministic-hypervisors post;
  [C-pierrezemb-2025] FDB-simulation architecture deep-dive (surfaced, unread);
  [C-hotos25-shell-static-2025] HotOS'25 shell-static-analysis position paper
  (surfaced, unread); [C-wasi-122-2019] WASI issue #122 (POSIX non-goal).
