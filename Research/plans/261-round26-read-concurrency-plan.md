# 261 — round 26: the read-only concurrency story — within-host probe parallelism + the ordering theory

AI-authored (Fable, design-synthesis session), 2026-07-07. Companion to `plans/260` (same
branch, `ai/spike3-r26`): 260 covers *across-host* concurrency (the fleet engine + transport);
this plan covers *within-host* read-parallelism and the dependence/scheduling theory that
makes reads reorderable at all. Together they are round 26's full read-only concurrency story.
The apply phase is explicitly untouched: apply stays **linear per host** — the elided CFG runs
as ordinary `dash`, in book order, welded (`notes/140` f11); every mechanism below is
probe-lane-only, under `kFAIL-withhold`.

Charge (human, 2026-07-07, near-verbatim): the plan phase has been depending on
read-parallelism to make sense — dispatching a thousand reading-tasks of varying complexity
using varying tools is not massively cheap, and the entire time the admin is sitting there
waiting; this is not a tertiary feature. Reads must be topologically re-orderable (depending
on the runbook CFG) and splittable — only things that don't depend on other things may
parallelize. And leave a designed slot for someday-ordering-by-perf-feedback, so notoriously
long tasks don't get sorted into the same reader-lane serially.

**Rulings received (human, 2026-07-07) + the spine extraction:**
- **The organizing principle:** bake in early the *ability* to re-order for perf; defer the
  tuning of *how*. Made structural in **`plans/262`** (the extracted build-spine shared with
  260): the order-independence invariant + the policy ports are the ability; everything in
  this doc's §3–§4 is the deferred how. The one guard the deferral rests on: ordering policy
  stays bolt-on-able only while `262` §1's spine-inv-order-free holds — police it.
- **Cross-run re-ingest: DEFER, full stop** — what-reorders-things can change later and must
  stay invisible to the user, so it bolts onto `262` port-cost later. dec-261-timing-cache
  below is thereby RESOLVED-DEFERRED; the rec-5/kSTATE fence question stays parked un-asked.
- **Golden posture:** r24's language work will churn goldens massively anyway; these plans
  are being passed to the r24 implementor sibling for extractables (`262` §7 handoff note).

---

## §0. Why this is load-bearing, in numbers

The probe phase is the one phase where the human is synchronously waiting (`DESIGN.md`
"probing phase": "the user is sitting there, waiting on a coherent plan (i.e. for all the
probes to finish and return), before they can hit 'submit'"). It is `kOBJECTIVE-latency`
territory by construction.

Strawman arithmetic (order-of-magnitude, ~SUSPECT but the right shape): a site-probe costs
one `fork+exec` of a subshell + its tool invocations. `dpkg-query`-class ≈ 10–50 ms;
`systemctl show`-class ≈ 20–100 ms; a `--version`-spawning vendor binary ≈ 50–300 ms; anything
network-touching (`getent hosts`, an index freshness read) ≈ 100 ms–seconds. A 300-site book
with oracle coverage probing serially: 300 × ~50 ms ≈ **15 s per host** in the good case,
minutes when a few network-class or wedge-prone probes sit in the chain — and `plans/072`'s
regime analysis (Mitogen: "most potent on … many short-lived actions, where overhead dominates
the cost of the operation") says exactly this workload is where dispatch shape dominates.
Cross-host fan-out (260) does not help the per-host tail: fleet plan latency = **max over
hosts of the per-host probe makespan**. Width-K parallelism inside the artifact divides the
common case by ~K and — with cost-aware placement (§4) — keeps the long poles off one lane.

Fences: read-only lane only · no new network round-trips ever added to recover anything (§2
h2 — round-trips dominate, `plans/076`) · no cross-run persistence built this round (§4 t2's
fence) · `--faithful` keeps its meaning: width 1, book order, one check per batch
(`plans/142` resolution; `kFIDELITY`).

---

## §1. Ground truth — what exists, and what is actually stated where

The human asked whether reorderability is stated anywhere in the corpus. Answer: **the
components are; the contract is not.** This section is the audit; §2 mints the contract.

**The incumbent artifact (verified against the goldens, 2026-07-07 — e.g.
`strawman24-derived-survive/expected.out`):**
- A flat, **strictly sequential** task list: per-site invocation lines
  (`apt_get__predict 'install' '-y' 'nginx'; _rc=$?; … printf 'site 1 effect=%s rc=%s\n' …`)
  one after another, top to bottom; then the derivation-probe section (`deriv N coord=…`
  lines from escalated `touches()` bodies piped through `while read`).
- **Shared top-level scratch**: `_rc`/`_e` and the oracle bodies' own locals (`verb`, `pkg`,
  `dest`) live in ONE shell namespace, re-assigned per task (observed as "benign today,
  uncontracted" in the 24K census). Naive `&`-backgrounding of the existing lines RACES on
  these — per-task subshell isolation is the enabling transform (§5).
- **Argv is compile-time-static**: every shipped probe-task carries literal argv (value-flow
  resolved at compile; an unresolvable value ⇒ ⊤ ⇒ *no probe ships at all*). Consequence,
  +SURE and nowhere stated: **today's shipped probe set is an antichain by construction** —
  no probe consumes another probe's output, no probe mutates (read-only contract), so the
  whole set is mutually order-free. Reorderability is currently an *accident of compilation
  strength*, about to become law (this doc) precisely because richer probe classes (§2) will
  break the accident.
- **Speculative across branches**: probes ship for every modeled site regardless of whether
  its branch will turn out live; relevance is resolved controller-side at fold (dead arms'
  facts discarded — e.g. the door1 family: the guard's Query probe folds the arm dead). So
  `kFLATTEN` sits de-facto at the hoist pole for relevance; maintain-cfg is unbuilt.
- The record lane: `site <leafid> effect=<…> rc=<n>` / `deriv <leafid> coord=<…>`, one line
  per record, self-describing header — already leafid-keyed, i.e. **order-free by design**
  on the consuming side (the fold's `merge_observable` is commutative/idempotent — `22H` §3).

**What the corpus already states (the pieces):**
- `plans/142` (resolution): per host, channels = **batches with internal `&` concurrency**
  ("channels = batches, NOT leaves"); `--faithful` = one check per batch. The topology is
  settled; what a *batch contains* and *in what order* was never designed — that is this doc.
- `notes/140` f13 + KNOBS history: **`kCONC` (how much intra-host probe concurrency) was
  deliberately parked un-minted** ("overlaps kFLATTEN's runtime half; left out of KNOBS").
  This plan is its de-facto design round — §9 dec-261-mint-kconc asks the human whether to
  register it properly now.
- `notes/074` (cost model): the three-tier cost signal (conservative default / static class /
  profile-guided from the probe's own telemetry) + **the guard-purity precondition**: "every
  retained guard must be read-only + initial-state-evaluable" — the S0-evaluability half of
  §2's commutation axiom, stated for `kFLATTEN` and inherited here.
- `plans/076`/`notes/072`: "embarrassingly parallel by design" is the *cross-host* claim
  (note 70's); the within-host half was never argued. The osquery watchdog rider ("don't
  degrade the monitored thing"; a watchdog-killed probe is unknown, not clean) and the
  shared-resource herd vector both bind within-host too.
- `notes/24J`: connected read-only check-pipes ship as **ONE probe unit** (stdin-connectivity
  forces it) — the first multi-command atomic probe unit, i.e. the first internal-ordering
  constraint inside the probe lane.
- `spike/e2e/run.sh` gate-1: the harness **executes** probe artifacts under mocks and
  byte-compares records against the authored `probe-results.txt` (parity) — a harness-level
  order-sensitivity that parallel emission must address (§7).

---

## §2. The dependence theory (the contract this round mints)

**Unit: the probe-task.** The schedulable atom. Today's classes, each read-only by contract:
- **site-probe** — a predict-interceptor invocation for one site (`site N …` record).
- **deriv-probe** — an escalated `touches()` body run for its footprint (`deriv N coord=…`).
- **resolve-probe** — a kind-owner `resolve()` canonicalization run (Stage-5A machinery).
- **reach-probe** — a kind-owner `reaches()` dynamic-arm expansion (Stage-5B; its
  derived-coord second-round-trip residual stays deferred — `resid-resolve-derived`).
- **connected-pipe-probe** — a whole vouched-read-only pipeline as ONE unit (`24J`);
  internally ordered by the pipe itself, externally an ordinary task.

**The commutation axiom.** Any two shipped probe-tasks commute — same records either order or
concurrently — given three preconditions, each already law:
1. **read-only** (`kFAIL-withhold`; vouched or Dorc-provenance) — no task changes the state
   another reads;
2. **S0-evaluability** — every probe evaluates against the host's *initial* state (nothing
   has been applied when probes run; `notes/074`'s precondition generalized from guards to
   all probe-tasks);
3. **hermetic canonicalization** (`kVOLATILES-exclude`) — volatile reads are canonicalized/
   excluded by the oracle contract, so timing-adjacent variance is out-of-scope by law.
A violation (a "read-only" probe that mutates, a volatile un-canonicalized read) is an
**oracle bug in the adequacy class**, not a scheduler bug — same trust boundary as everything
else on the grounding side (`plans/102` E3), backstopped someday by the differential. The
scheduler's own obligation is only: never *create* an ordering hazard the axiom doesn't cover.

**The hazard taxonomy (what CAN impose order), and the rule for each:**
- **h1 · value edges** — task B consumes a value task A produces. Today: none ship (the
  antichain finding, §1); the classes that will mint them: probe-readback resolving book
  values (the `$(hostname)`-case, once a read-value oracle exists), reach-probe second
  round-trips, any future predict-body chaining. RULE (rewritten per 26A amend-h1-mechanism —
  the prior "consumer runs in a later wave" was mechanism-free, a genuine composition error
  three lanes caught: probe values return to the CONTROLLER; a later wave in the same
  shipped artifact has no channel by which to consume them): h1 edges are explicit in the
  schedule graph and resolve by exactly one of two mechanisms — (a) **in-artifact
  connected-unit composition** (the `24J` shape generalized: the producer's value is captured
  host-locally inside ONE compiled unit that contains its consumer), or (b) **controller-fold
  consumption** (the value returns as a record and is consumed controller-side at fold, never
  by another shipped probe). **Waves exist for width/pacing only** — never as a value-passing
  mechanism; no wave boundary carries an h1 edge. Enforcement is built at S0/P0 as a real
  compile step: the h1-edge extraction pass, with a zero-edges-on-today's-corpus pin plus one
  synthetic injected edge proving the compiler→schedule wiring — antichain-by-proof, not by
  accident. These are the ONLY true edges.
- **h2 · control-relevance** — probe results decide which sites are *live*, hence which
  probes were *worth running*. NOT a soundness edge (running a dead arm's read is safe,
  merely wasted) — a **cost** edge. Three positions: (i) **speculate** (the incumbent: ship
  all modeled probes, fold discards) — maximal parallelism, bounded waste; (ii) **in-artifact
  maintain-cfg** (`kFLATTEN` pole: arm probes nested under probe-versions of their guards) —
  zero waste, serial along control chains, compile complexity; (iii) **staged round-trips**
  (probe, fold, ship wave-2) — REJECTED as a relevance mechanism, permanently: network
  round-trips dominate everything (`plans/076`); relevance is recovered in-artifact or paid
  for, never re-crossed. v1 keeps (i); (ii) becomes worth building only if measurement shows
  dead-arm waste dominating (§9 dec-261-speculation-default).
- **h3 · resource contention** — tasks sharing a lock/daemon/device (dpkg DB, docker socket,
  network egress, one wedge-prone daemon — PM-3). Not an ordering edge but a **width/
  placement** constraint: v1 = the global width cap only (conservative); the designed slot =
  a per-task *resource-key* (same key ⇒ never co-scheduled in one wave-lane), fed by the same
  static classifier as §4 t1 (a dpkg-family body ⇒ key `dpkg-db`). Build the key plumbing
  only when a real contention bite shows (§11).
- **h4 · namespace/scratch** — the shared `_rc`/`_e`/oracle-locals of the incumbent (§1).
  Purely an artifact-mechanics hazard; dissolved by per-task subshell isolation (§5), which
  also isolates cwd/umask/set-flags per task. After the transform, h4 does not exist.

**The stated contract (the sentence the corpus lacked):** *every shipped probe-task is either
argv-static + S0-evaluable — and thereby freely reorderable and parallelizable with all its
peers — or carries explicit h1 edges / atomic-unit membership that the schedule graph
represents; the compiler proves membership at emission, and anything it cannot prove does not
ship (⊤, the existing floor).* Scheduling = antichain decomposition of that graph; today the
graph is one antichain, and the machinery below is built so that stays *true by proof* rather
than *true by accident* as h1-minting features land.

---

## §3. The schedule model

**Shape: waves × width.** The schedule graph topo-sorts into antichain layers (today: one).
Within a layer, tasks are binned into K lanes; the artifact runs each wave as K concurrent
subshell chains and `wait`s the wave before the next. (Per 26A amend-h1-mechanism: waves are
a width/pacing device ONLY — h1 edges never resolve across a wave boundary; they resolve by
connected-unit composition or controller-fold, §2 h1 as amended. An earlier draft claimed
h1 layers as wave boundaries; that was mechanism-free and is retracted.) Width K: default
**4**, flag-settable (`--probe-width` / `DORC_PROBE_WIDTH`),
`--faithful` forces 1 + book order. K stays modest by design: the target is a production host
(osquery lesson — the reader must not degrade the read), fork-cost is trivial at K=4..16, and
the Graham anomaly warns against treating width as monotone (`plans/076` §4).

**Why waves (a convoy, honestly priced), not a refilling pool:** POSIX sh has no `wait -n`
(bash 4.3+) and no portable job-slot primitive; a refilling pool needs either a bashism, a
`mkfifo` token pool (re-opens `plans/142` front-C writable-fs), or busy-wait polling. The
wave barrier costs convoy time — a wave lasts as long as its longest member — and the
mitigation is placement, not machinery: **LPT ordering** (longest-expected-first across
lanes, the classic 4/3-approximation for makespan) + class segregation so network-class
long-poles land in the same *early* wave spread across lanes, rather than queueing behind
each other in one lane or straggling one-per-wave at the end. The pool upgrade is named in
§11, gated on measured convoy pain, not built now (§9 dec-261-wave-vs-pool).

**Placement algorithm (v1, deliberately boring):** within a wave, sort tasks by descending
cost estimate (§4), assign each to the currently **least-loaded lane** (greedy-LPT — the
wording fix per 26A amend-smalls: round-robin is NOT LPT and forfeits the 4/3 bound).
Deterministic: ties break by leafid. The whole scheduler is a pure function
`schedule(tasks, edges, K, cost) → waves/lanes` — trivially unit/property-testable (§7) and
living outside the emission code (§10).

**Interaction with `kFLATTEN`:** this doc operationalizes the hoist pole for *scheduling*
(flat waves) while leaving the *relevance* question (h2) at the incumbent speculate position.
If maintain-cfg is ever built, its arm-nested probes become sub-graphs whose control edges
the same scheduler consumes — the model doesn't change, the graph does.

---

## §4. The cost signal (074's three tiers, applied to placement)

- **t0 · conservative default (ships first):** all tasks equal-cost ⇒ LPT degenerates to
  arbitrary-but-deterministic placement. Parallelism alone buys the ~K-fold win; ordering
  adds nothing yet. Zero knowledge required.
- **t1 · static class, inferred from the body (ships this round):** a small pure classifier
  over the already-parsed oracle/probe body assigns one of
  `stat-class < exec-class < pkgdb-class < daemon-class < network-class` (exact set =
  builder's, small and closed) — e.g. any `curl`/`wget`/`ssh`/`getent hosts` in the body ⇒
  network-class; `dpkg-query`/`rpm -q` ⇒ pkgdb-class. This is `notes/074`'s "infer the class
  from the predict body" option, chosen over oracle-declared cost annotations (rejected:
  ceremony against the `kOOB` grain and the cargo-cult razor — the body already says what it
  reaches; ~SUSPECT the classifier covers the useful 90%). The class drives LPT rank AND the
  h3 resource-key slot AND which tasks get per-task timeouts first (§5).
- **t2 · measured (the slot the human asked for — designed, mostly NOT built):** records gain
  an additive `ms=<int>` field (one printf still; §6 grammar policy). Within one run this is
  telemetry only (the schedule was fixed at compile). Its *scheduling* value is cross-run —
  "notoriously long" is a memory — and cross-run reuse hits TWO standing fences squarely:
  **rec-5** (the probe-TAPE is write-only; "nothing re-ingests receipts across runs" —
  WELDED) and **kSTATE** (parked, with the human's hostile-host security note). An
  advisory-only timing cache (host-keyed `probe-key → EWMA-ms`, licenses nothing, worst case
  = a slow schedule, never a wrong plan) is *plausibly* outside those fences' intent — but
  they are human-owned fences, so this round ships: the `ms=` field, the in-run telemetry
  surface (`dorc why`-adjacent: "slowest probes this run"), and a one-page slot design — and
  builds NO persistence. §9 dec-261-timing-cache is the decision package.

---

## §5. The artifact transform (what's 261-specific; the mechanics live in the spine)

The transform itself — per-task subshell isolation (which dissolves §2 h4, the shared-scratch
race verified in the incumbent), wave barriers with the sentinel after the final `wait`,
single-printf record atomicity, the escaped-grandchild discard rule, the width flag with its
width=1 byte-identical default, and `--faithful` = width 1 + book order — is the build-spine:
**`plans/262` §2 (lane rules) + §3 (the emission locus)**. Kept here, the two 261-specific
riders:

- **Per-task timeout (ALL classes, best-effort; class-gating dropped per 26A
  amend-timeouts — the dead-NFS `stat` is the classic hang and lives in the cheapest
  class):** `timeout <n> …` wrapped around every probe-task, IF the artifact's own
  `command -v timeout` feature-test passes (GNU/busybox common, not POSIX); class may still
  set the *duration*. Degrade silently to untimed — the whole-artifact timeout (260 §3 s3-6)
  remains the backstop, and the loss shape without `timeout` is documented there (one wedged
  task holds its wave); a timed-out task's record is `effect=cant-tell` with its real rc
  (the existing ≥2 ⇒ can't-tell fold — no new semantics, and `kFAIL` holds: killed-read ⇒
  unknown ⇒ run).
- **Niceness:** not applied by default; `--probe-nice` opt-in (`nice` IS POSIX) for
  admins probing tender hosts. (Deliberately not default: probes are short; K is small.)

---

## §6. Wire / fold / fleet compatibility (nothing in 260 moves)

- The lane rules that make parallel emission safe — leafid-keyed order-free records,
  sentinel-after-final-wait, controller-side census by leafid accounting (`seen=` was
  DROPPED by the 26A stop-1 rewrite — a shared counter is unimplementable by concurrent
  pure-sh subshells), per-record terminal tokens, family end-records, and the additive-keys
  versioning policy that makes `ms=` a within-/1 addition — are the spine: `plans/262` §1–§2.
  Parallel emission changes arrival ORDER, never content (spine-inv-order-free).
- **Fleet interaction:** per-host makespan drops ~K-fold; fleet plan latency = max over
  hosts; the 260 pacing/width caps (connection-level) are orthogonal to K (process-level,
  on-host). No fleet-kernel change.
- **Harness (gate-1 parity):** resolved in the spine (`262` §1 pin-no-order-keyed-consumers):
  harness probe executions run at width=1; ONE dedicated jitter-case family exercises
  width>1 (`262` §5). The discovery stands recorded here: gate-1 byte-compares probe records
  against authored `probe-results.txt`, i.e. it was the codebase's one order-sensitive
  records consumer.

---

## §7. Testing

- **Grouping-soundness (pure, property-based; the load-bearing net):** generate random
  task-graphs (sites × classes × injected h1 edges × atomic units); assert the scheduler
  never places a consumer at-or-before its producer's wave, never splits an atomic unit,
  emits every task exactly once, is deterministic (same input ⇒ same schedule), and
  `--faithful` reproduces book order exactly. Lives with the pure scheduler fn. (Lands at
  spine S0 with the port; stated here because it is the theory's executable half.)
- **Fold permutation + runtime jitter:** spine pins/rig — `plans/262` §1
  (pin-fold-permutation) and §5 (the jitter family + `DORC_MOCK_JITTER_SEED`); both are
  shared with 260's needs and land at spine S1. The standing `dash -n` gate (ap-2) covers
  the parallel artifact shape — subshell-`&`-wait is POSIX-clean, the gate proves it stays so.
- **Determinism fence:** compile-side, same book+oracles+width ⇒ byte-identical artifact
  (the schedule is deterministic; only runtime interleaving varies). Golden-diffable as ever.
- **The makespan yardstick (stage P4):** a generated strawman family ("the thousand-reads
  scenario": N sites across cost classes, mock durations from the class table) measured at
  width 1 vs 4 vs 8 under the jitter harness — a regression tripwire + the evidence for the
  width-default and speculation decisions. NOT a benchmark race; wall-clock on mocks is
  directional only (~SUSPECT good enough to rank policies, not to promise numbers).
- **hostsim/sweep:** no change — hostsim answers verdicts, it does not run sh; stream-level
  interleaving/fault coverage is 260 §7's tier. (The elision-soundness sweep is untouched;
  RESERVE-NOT-COLLIDE continues to hold.)

---

## §8. Stages (the P-track; interleaves with 260's ladder — P0 may start immediately after stage-26-0)

Gates per stage: as 260 §8 (fresh build · clippy · suites · e2e byte-stable — which the
width=1 default guarantees through P3).

- **P0 — the contract + the scheduler. [ABSORBED into `plans/262` S0.]** §2's contract
  stated in the plan/emission docs; the pure `schedule()` fn (= `262` port-schedule, constant
  cost) + grouping-soundness property tests; task-class enumeration typed. No artifact
  change. Gate: properties green; zero behavioral diff.
- **P1 — the parallel artifact, flag-gated. [ABSORBED into `plans/262` S1.]** Subshell
  isolation + wave emission at the `262` §3 locus; harness answer + jitter family + parser
  discard rules per `262` §1/§5. Gate: jitter family green across seeds; all existing
  goldens untouched.
- **P2 — cost-aware placement.** The t1 static classifier (pure fn over parsed bodies) +
  LPT within waves + per-task `timeout` (all classes, §5 as amended) + `--probe-nice` +
  **h3-lite** (26A amend-smalls): the class doubles as a coarse resource-key —
  daemon-class tasks get concurrency ~1 per wave (never co-scheduled against the same
  wedge-prone daemon), the cheap slice of §2 h3 without building the full key plumbing.
  Gate: classifier unit pins (each class exemplar); schedule remains deterministic; jitter
  family still green.
- **P3 — telemetry.** `ms=` on records (grammar policy sentence lands with it) + slowest-
  probes surface + the timing-cache slot design WRITTEN (one page, in-doc §11 pointer) —
  no persistence code. Gate: additive-key round-trip pin (old parser ignores ms=).
- **P4 — measure + decide.** The makespan yardstick; evidence package to the human: width
  default (4 vs 8), speculation (keep vs build maintain-cfg), pool upgrade (worth it?),
  timing-cache ruling request. Extract conclusions; propose KNOBS text if kCONC is minted.

---

## §9. Decision ledger (dec-261-*; defaults live, ratify or veto)

- **dec-261-width-default** — K=4 gentle default (production-host courtesy + Graham
  caution), `--probe-width` to raise; P4's yardstick revisits. Alternative (8/16) deferred to
  evidence, not taste.
- **dec-261-wave-vs-pool** — wave barriers + LPT (POSIX-pure, convoy accepted) over a
  refilling pool (`wait -n` bashism / mkfifo tokens / busy-wait — each breaks a standing
  constraint). Pool = named upgrade in §11, gated on measured convoy pain.
- **dec-261-speculation-default** — keep the incumbent speculate-across-branches; relevance
  waste is bounded and parallel; maintain-cfg only if P4 shows dead-arm cost dominating;
  staged relevance round-trips rejected permanently (network dominates — `plans/076`).
- **dec-261-golden-stability — context updated (human, 2026-07-07):** r24's language work
  churns goldens massively anyway, so byte-stability-vs-the-sibling is no longer the primary
  motive. The width=1 default is RETAINED regardless — as harness-determinism insurance, the
  `--faithful` floor, and the P4 A/B lever (`262` §3 golden posture); the width-default flip
  stays a deliberate later decision package with its one-time re-bless.
- **dec-261-timing-cache — RESOLVED: DEFER (human, 2026-07-07).** Cross-run re-ingest is
  punted entirely: *what* re-orders things can change later, must stay invisible to the
  user, and therefore bolts onto `262` port-cost once core concurrency exists — enabled by
  spine-inv-order-free. This round ships the `ms=` measurement lane (write-only telemetry,
  rec-5-clean) and nothing else. PARKED for whenever the punt lifts, verbatim: *does rec-5's
  no-re-ingest intent cover an advisory-only, license-nothing timing profile, or was it
  scoped to verdict/fact reuse?* (touches welded rec-5 + parked kSTATE with its hostile-host
  note; nobody walks through the fence silently).
- **dec-261-per-task-timeout** — feature-tested best-effort `timeout` on daemon/network
  classes; degrade untimed; whole-artifact timeout remains the guarantee. (Rejected: a pure-sh
  per-task watchdog — background-sleep-and-kill plumbing costs more correctness than it buys.)
- **dec-261-ms-field** — `ms=` lands as an additive key at P3 (write-only telemetry; the
  data exists whenever the re-ingest punt lifts). The additive-keys policy itself is spine
  law now (`262` §2).
- **dec-261-mint-kconc** — propose registering `kCONC` as a real knob (poles ≈
  `kCONC-linear ↔ kCONC-wide`, status directional-wide-bounded, owner corpus+user, this doc
  as the design record) — KNOBS is human-authored, so this is a proposal, not an edit.
- **dec-261-classifier-over-annotation** — cost knowledge comes from body inference (t1) and
  measurement (t2), never from a new authored annotation surface (kOOB grain + razor; also
  keeps this branch fully clear of the in-flight language-surface work — no new spelling is
  minted anywhere in this plan).

---

## §10. Merge-disjointness (amends 260 §10)

The emission-site touchpoint and its flag-gating are now spine law (`plans/262` §3: one
bounded locus shared with 260's framing; cognition outside in pure modules; shared merge
rules + the r24 handoff note at `262` §7). 261-specific residue only: The records parser change
(ignore-unknown-keys + late-record discard) is additive. No `syntax`/`oracle` semantic paths
touched; no new authored spelling exists (dec-261-classifier-over-annotation); `core`
untouched.

## §11. Deferred / reserved (named, with re-entry pointers)

- Refilling worker pool (convoy-free width) — re-enter on P4 evidence; mechanisms: `wait -n`
  where bash-detected (two-tier artifact? distasteful), or the mkfifo token pool riding
  `plans/142` front-C's writable-fs answer.
- h3 resource-keys (contention-aware placement: dpkg-db, docker-socket, net-egress) — the
  classifier already produces the raw material; build on first real contention bite.
- maintain-cfg relevance (`kFLATTEN` other pole) + probe-readback staging (h1-minting;
  `resid-resolve-derived`'s second round-trip) — both re-enter through §2's graph, no model
  change.
- Cross-run timing cache — gated on dec-261-timing-cache's human ruling.
- Within-host batch channels (N parallel SSH channels per host, `plans/142` topology) —
  becomes interesting only past single-channel saturation; the wave machinery is
  channel-count-agnostic by construction.
- Cross-host shared-resource herd (jitter/throttle across the fleet) — `plans/072`; needs
  h3 keys + fleet-level scheduling; explicitly not v1.
- Live per-record TUI — ru-20, unchanged.

## Cross-refs
`plans/260` (the fleet/transport half; §4 wire, §8 ladder, §10 contract) · `plans/142` +
`notes/140`/`141` (batch topology, PIPE_BUF atomicity, EOF hazard) · `notes/074` (cost tiers,
guard-purity/S0) · `plans/076`/`notes/072` (round-trips dominate; osquery watchdog; Graham;
LPT/RCPSP framing) · `notes/24J` (connected units) · `24E`/`24F`/`24G` (deriv/resolve/reach
probe classes) · `notes/22H` §3 (merge commutativity, terminal determinism) ·
`spike/e2e/run.sh` gate-1 (parity; the harness touchpoint) · KNOBS: `kFLATTEN` · `kPROBING` ·
`kOBJECTIVE` · `kFIDELITY` · `kVOLATILES` · `kFAIL` · (`kCONC` — unminted, §9) · rec-5 +
`kSTATE` (the t2 fence).
