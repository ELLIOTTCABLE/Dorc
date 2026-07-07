# 260 — round 26: multi-host — concurrency + networking design & implementation plan

AI-authored (Fable, design-synthesis session), 2026-07-07. Round 26 = **the first multi-host
build**: the concurrent per-host engine (`plans/22H`), the real controller↔host transport
(`plans/142` resolution), and the fleet-orchestration floor — productized into the spike.
Branch: `ai/spike3-r26`, forked from `ai/spike3-r23` @ `75de2ac`; substantive language-surface
work continues on r23 in parallel, and §10 is the merge-disjointness contract that keeps this
branch cleanly re-mergeable over it.

Authority: this plan *composes* settled law — it re-decides nothing already welded. Where it
makes new calls they are enumerated in §9 (`dec-26-*`) for human ratification; defaults are
chosen so a builder can proceed without blocking on them. Written to be executable by a
lesser-capability builder agent: read §1–§7 as the spec, §8 as the build order, and treat every
"MUST/NEVER" as a gate, not advice.

---

## §0. Mission + fences

**Mission.** `dorc plan book.sh -H web1 -H web2 -H db1` probes all three hosts concurrently,
folds each host's probe-report into a per-host plan as that host's results arrive, and emits
one plan artifact per host (`ru-29`). `dorc apply` ships per-host artifacts concurrently,
captures per-host outcomes, and reports fleet status honestly — including the hosts it could
not reach and the hosts whose state became unknown mid-flight. All of it deterministic under
DST via a seeded in-process simulation of the transport.

**The one-sentence value claim** (from `DESIGN.md` "Dorc's approach" §1-2 + `plans/072`): the
probe is compiled per host and shipped once — ~O(hosts) round-trips, not O(sites×hosts) — so
multi-host is where the compile-the-probe architecture's structural win actually cashes out;
and the per-host plan ("a command replaceable on host A may still run on host B") is the
product surface single-host Dorc cannot show at all.

**Hard fences (round 26 does NOT):**
- **No cross-host plan dependency** — no host's plan reads another host's facts. Human-parked
  (`22H` §6, ru-28: "EXTREMELY far-future, zero cycles"); the rolling-update/quorum/drain class
  is out. Acceptance pins the partition (§7 acc-per-host-partition). This is also the security
  posture: a compromised host can lie its *own* plan wrong (bounded by `kFAIL-perform`), and
  structurally cannot influence any other host's plan (`plans/102` E-host-as-adversary).
- **No on-host executor** — `kCOMMS` stays executorless per the `plans/142` resolution; the
  executor re-pin ({no-writable-fs, hard backpressure}) stays deferred.
- **No fan-out tree, no cross-host memoization** — `plans/076` §6 items 5/9 stay reserved;
  `kSTATE` stays parked (with its hostile-host security fence — nothing in this round persists
  or re-ingests state across runs; rec-5 stands).
- **No rolling/canary/serial policy** — `kSCHEDULE` defer-but-reserve; v1 ships a width cap
  only, but the engine's decision loop carries the policy *hook shape* (§3) so serial/canary
  lands later without rework (`plans/076` §4's under-investment-trap mitigation).
- **No language-surface changes** — zero semantic edits to `syntax`/`oracle`/`plan` crates'
  authored-surface behavior; the round consumes the plan kernel's API read-only. (§10.)
- **No TUI** — live display = plain per-host progress lines (ru-20 rich TUI stays deferred).
- **Targets = tier-A real-POSIX hosts** with a writable `$TMPDIR` (`plans/139` §3); busybox/
  no-fs/Windows targets ride the deferred executor corner.

---

## §1. The settled substrate (law the builder must not re-derive or violate)

Read before writing any code. Each item is binding; citations are the authority.

- **law-seam-1.** The unit that crosses the network is a **whole compiled artifact per host,
  per phase** — probe artifact, then apply artifact. A site/leaf is *never* a network op;
  `(host, site)` is a compile-time coordinate (`plans/128` concl-seam/se-1; `plans/111` dac-C).
  The DST substitution point is exactly this boundary: `ship(host, unit) → result-stream`.
- **law-kernel-purity.** Correctness-critical kernels stay dependency-clean and deterministic;
  ALL nondeterminism (clock, network, process-spawn, randomness) lives behind DI at the edge
  (`AGENTS.md` critical-engineering; `an-di-seams`; `inv-determinism`). The new fleet kernel
  (§3) is a **pure state machine** — events in, commands out — and must compile with no
  tokio/network/clock dependency. hostsim remains the sanctioned-nondeterminism exception.
- **law-fail-direction.** `kFAIL` phase-keying is never traded: probe-phase fails toward
  withhold, apply-phase toward perform. Fleet-scale corollaries (`plans/076` §3, `plans/072`):
  an **unreachable host is unknown — never "clean"** (non-response ≠ converged); a
  **watchdog/timeout-killed probe is unknown, not clean**; a mid-apply transport loss makes the
  un-acked remainder **Unknown**, never assumed-ok and never assumed-failed (`plans/128` fc-2).
- **law-no-double-apply.** Under Unknown, a retry must not double-apply (`plans/128` fc-4).
  The sanctioned recovery is **re-probe, then re-plan** — the probe *is* the retry-file,
  derived not stored (`plans/072` resumability dividend). Auto-retry of an apply is forbidden;
  auto-retry of a (read-only) probe is permitted and bounded.
- **law-artifact-floor.** The apply artifact is byte-floored (rec-1): the user's lines ride
  verbatim; no Dorc markers are interleaved into the apply artifact's bytes. Consequence
  accepted for v1: apply-phase progress is observable only at whole-artifact granularity
  (§9 dec-26-apply-visibility). Per-leaf apply markers belong to the deferred
  `kFIDELITY-faithful` leaf-wrap (`plans/077` Seam-2), not to this round.
- **law-lane-discipline.** `23K` §2 vocabulary is binding: **tool-rc** stays inside oracle
  bodies; the **probe-report** travels a Dorc-controlled lane, never conflated with a tool's
  own channels; **plan-verdicts** exist only engine-side. Bare "rc" and bare "skip" stay banned
  words. The engine **measures rc and reproduces it; it never interprets a tool's rc meaning**
  (`24J`).
- **law-transport-shape.** The `plans/142` resolution is the target shape: per-host session
  channels for tool I/O at full fidelity; Dorc-signalling out-of-band of tool channels, split
  short-gating vs large-diagnostic; signalling never shares a lane with freeform (security is
  structural, not escaping — `notes/140` f-sec). §4 states how v1 instantiates this narrowly
  and what is explicitly a reserved growth shape.
- **law-perf-redlines.** Never fork-per-host as the architecture (`plans/072` footgun-1 —
  async/non-blocking edge); pace connection-opens under the target's `MaxStartups` (default
  10:30:60 — over-fanning gets silent drops, footgun-3); analysis plane and execution plane
  stay separable (`plans/076` §6 item-1b). The controller-side big-O ceiling is real but
  **remote command latency dominates everything** (`AGENTS.md` performance) — do not gold-plate
  controller-local paths.
- **law-security-floor.** `plans/102` banked list, the transport-relevant subset: ProxyJump
  semantics only for hops (never `ForwardAgent`); host-key verification never blind-accepted by
  default; no feature that concentrates fleet credentials (Dorc holds NO keys — ssh's own agent
  and config are the credential plane); strip/escape terminal control chars on anything echoed
  from remote streams to the operator's TTY; probe wall-clock is bounded (read-only ≠
  non-blocking, PM-3).
- **law-attention-honesty.** rul-attention-honesty extends to the fleet render: no host's
  surviving lines are hidden; unreached/unknown hosts are surfaced *first*, not buried in a
  summary count.
- **law-determinism-acceptance.** The final per-host plan produced incrementally MUST be
  byte-identical to a single-shot analysis fed that host's full fact-set (`22H` §3 terminal
  determinism) — this is the round's anchor invariant and a standing test.

---

## §2. Architecture — crates and dataflow

Two new crates + additive extensions. Names follow the existing flat convention.

```
                    ┌────────────────────────────── controller ──────────────────────────────┐
 book.sh  ─parse/classify once (existing kernel: syntax→analysis→plan static half)─┐
 hosts    ─┐                                                                        │
           ▼                                                                        ▼
        ┌───────────────  fleet (NEW, pure kernel)  ───────────────┐   per-host: build_plan(
        │ per-host accumulators · session state machine · pacing   │     classes, observe(host),
        │ decisions · failure taxonomy · arrival fold orchestration │     arena)  [existing]
        └──────────── events ▲            ▼ commands ──────────────┘
                             │            │
        ┌────────────────  transport (NEW, edge)  ──────────────────┐
        │ SessionDriver trait: real-ssh (subprocess) · local-sh ·   │
        │ sim (tests). Owns sockets/processes/clock. tokio-or-poll  │
        │ HERE ONLY.                                                │
        └───────────────────────────────────────────────────────────┘
                             │ ssh -F …  (per host: probe artifact → records; apply artifact → rc)
                             ▼
                       [ host₁ … hostₙ ]        hostsim (EXTENDED): per-host synthetic
                                                 record-streams + fault injection, seeded
```

- **`fleet` (new; pure).** The multi-host session kernel. Owns: the host set for a run,
  per-host phase state, per-host fact accumulators, the arrival-fold loop (delegating the
  actual fold to the existing `plan` API — classify once, `build_plan` per host per batch,
  `22H` §3), pacing/width decisions, the failure/Unknown taxonomy (§3), and the emission
  schedule for per-host plan artifacts. **Sans-io**: `step(state, event) → (state, commands)`.
  No dependency on transport, tokio, clock, or hostsim. Ordering comes entirely from the event
  sequence it is fed — which is what makes seeded-interleaving DST trivial (§7).
- **`transport` (new; edge).** Implements the command side: open session, ship artifact,
  stream back events. One trait, three drivers: **ssh-subprocess** (production: system `ssh`
  with a Dorc-authored `-F` config — §5), **local-subprocess** (spawn local `dash -s` —
  hermetic real-transport-shaped tier for e2e + `--local` parity with the r25 P3 runner), and
  **sim** (feeds synthesized streams from hostsim; lives with the tests). All nondeterminism
  lives here. Async runtime, if any, is confined here + cli (§9 dec-26-edge-runtime).
- **`cli` (extended).** New host arguments, the fleet driver loop (pump transport events into
  `fleet::step`, execute returned commands), per-host artifact emission, fleet summary render,
  new exit codes (§6). Single-host stdin/`--results` paths stay byte-identical (regression
  fence).
- **`hostsim` (extended, additive).** Grows the pre-declared fault seam (`hostsim/CLAUDE.md`
  an-host-fault-model): per-host synthetic record-streams derived from `Host` state, plus
  seeded faults — unreachable / connect-timeout / wedged (no records, no close) /
  truncated-stream (records stop mid-way, no sentinel) / forged-verdict / duplicate-delivery /
  reorder-across-hosts. Existing single-host API untouched; `Host` remains the verdict oracle
  (RESERVE-NOT-COLLIDE, `24B` C-altitude: this round *builds* the coordination-DST tier ON TOP
  of `Host`; the elision-soundness net is not touched).
- **`sweep`** — unchanged this round; optional fleet scenarios noted in §11.

Type seeds (builder refines; keep them in `fleet`, NOT in `core` — §10):
`HostId` (the ssh destination string, verbatim — an alias resolved by the user's ssh config is
first-class; Dorc never parses it), `RunNonce` (edge-minted, DI'd — never from an ambient RNG
in kernel code), `HostPhase` (Pending → Connecting → Probing → Planned → Shipping → Applying →
Applied | Unreachable{stage} | UnknownAfterLoss{acked_sites}), `FleetEvent`, `FleetCommand`.

---

## §3. The fleet kernel — semantics

**s3-1. Per-host independence (the partition law).** Every fact carries its `HostId` from the
moment it enters the controller; per-host accumulators are disjoint maps; `build_plan` for host
X consumes only X's facts. There is deliberately no cross-host lookup API on the accumulator —
make the wrong thing unrepresentable rather than linted. (Ties to acc-per-host-partition, and
to the forged-verdict containment test: a hostile host's influence ends at its own plan.)

**s3-2. Classify once, fold per arrival.** The static half (parse → cfg → value → classify →
compile_probe) runs once per book — it is host-independent (`22H` §3, +SURE, standing). Each
arriving per-host record batch: merge into that host's fact-store (`merge_observable` is
commutative/idempotent — order-safe within one host), then re-run the (pure, cheap) fold for
that host. Do NOT attempt an incremental fold — the full re-fold per batch is the settled
cheap-enough answer (`22H` §3: "do not oversell it as incremental"); network dominates. v1
batching note: with the §4 v1 wire there is typically ONE records batch per host (arrival
granularity = host), so the fold-per-batch machinery must be built but will usually fire once
per host; the DST tier (§7) exercises multi-batch arrival sequences anyway — the engine
semantics, not the v1 wire, are the contract.

**s3-3. Replacement-stability is best-effort, not invariant.** Across arrivals a cell can go
⊤ → Value → (disagreeing second fact) → ⊤, flipping a Replace back to Run (`22H` §1). Do not
"fix" this; do not suppress it; surface it in the render as ordinary tightening/loosening.
The acceptance property is the weaker monotone-run-count-per-host on the curated DST books
(§7 acc-monotone-tightening), asserted as *expected-shape*, not as a typed guarantee — the
typed source-distinction question (`OriginKind` decision-inertness, ru-11) stays open and is
NOT this round's to answer (`22H` §1; flag any pressure on it to the human).

**s3-4. Failure taxonomy (per host, per phase).** The named cells and their REQUIRED handling:

| cell | when | plan/report consequence |
|---|---|---|
| unreachable-preprobe | connect/auth fails before probe ships | host's plan = the book **as-written** (zero elisions, every site run), loudly marked `host unreachable — plan is the unprobed book`; fleet continues; exit-code taints (§6) |
| probe-timeout / wedged | probe exceeds wall-clock or stream stalls | kill session; treat as unreachable-preprobe (facts partial ⇒ discard to ⊤ for un-sentineled sites per §4; a partial-record host is *probed-partial*: received sites keep facts, missing sites are Unknown ⇒ run) |
| probe-truncation | records end without the §4 sentinel | as above: sites with received records keep them; the un-received range is Unknown ⇒ run; render marks the boundary (`plans/128` fc-2 at the probe lane) |
| forged/garbage records | nonce-mismatch or unparseable lines | ignore + count + warn (attention-honesty: one aggregated note per host); never fail the whole run on a hostile line (`notes/140` f-sec: freeform cannot reach the control lane; here the control lane *rejects* non-conforming bytes) |
| apply-transport-loss | ssh dies mid-apply (rc 255 + transport-error heuristics, cf. the r25 P3 runner) | host → UnknownAfterLoss; NO auto-retry (law-no-double-apply); report offers the re-probe recovery ("state unknown on <host>; re-probe to localize — the probe is the retry-file") |
| apply-nonzero | artifact exits ≠ 0, transport clean | host FailedApply{rc}; captured streams referenced; other hosts unaffected (batch unrelated errors — AGENTS fail-fast) |
| host-vanishes-mid-fleet | some hosts fine, others any-of-above | fleet completes all independent work; summary orders bad news first |

**s3-5. Pacing (the decision loop's v1 policy).** Two caps, both fleet-kernel decisions so
they are DST-testable: `open-cap` — concurrent not-yet-authenticated connection opens (default
8, below sshd's `MaxStartups` 10-start throttle; on transport-refused, exponential backoff +
retry bounded ×2 — probe-phase only); `width-cap` — concurrent active sessions (`-j`, default
`min(hosts, 16)`). The policy hook: the kernel asks one pure function `admit(phase, state) →
Vec<HostId>` for who proceeds; v1 implements width/open caps there; serial/canary later
replaces only that function (`kSCHEDULE` seam honored).

**s3-6. Timeouts (all injected, never ambient).** connect ≈15s (ssh `ConnectTimeout`);
probe wall-clock default 120s/host (whole-artifact; per-batch later — `plans/142` flag-1);
apply wall-clock default 0 (unlimited) with `--apply-timeout` opt-in (an apply is the user's
real work; killing it mints Unknown — the flag's doc says so); keepalive `ServerAliveInterval
15 / CountMax 4` (≈1min detection of a dead peer mid-apply, per the r25 config).

**s3-7. Aggregate outcome.** Fleet outcome = worst-cell ordering: any UnknownAfterLoss >
FailedApply > Unreachable > clean. §6 maps this to exit codes. The per-host detail is the
product surface; the aggregate is for scripts/CI.

---

## §4. Wire protocol v1 — the probe-report lane

**Shape (v1, deliberately narrow).** One session channel per host per phase; the probe
artifact's **stdout is the records lane**, framed; stderr is freeform passthrough (captured
per host, never parsed for control). Rationale: at v1 there is exactly one remote process per
phase and it is wholly Dorc-compiled — the "pristine tool channels vs signalling lane"
separation of `plans/142` becomes load-bearing only when within-host batch-parallelism (N
channels) arrives; building the remote FIFO/per-leaf-file machinery now would be speculative
plumbing with no consumer. The full 142 layout (N pristine batch channels + one reserved drain
channel + per-leaf rich-diagnostic files) is the **reserved growth shape**: nothing below
contradicts it, and the record grammar is designed to survive the move unchanged. (§9
dec-26-wire-v1 records the tradeoff; `plans/142` front-C writable-fs residual is thereby also
deferred — v1 needs no remote scratch dir for the probe lane.)

**Framing grammar (versioned from day 1 — the `24Kc` cluster-compat lesson applied to the one
NEW machine-shaped surface this round mints):**

```
dorc-records/1 nonce=<nonce> host=<hostid> book=<sha256-of-book-bytes> sites=<N>
<nonce> site 0 <existing record grammar, unchanged>
<nonce> site 3 <...>
...
dorc-records-end/1 nonce=<nonce> seen=<K>
```

- Header first; end-sentinel last (**never EOF-detect** — the bats fd-inheritance hazard,
  `notes/141` g5: a backgrounded child holding the pipe must not wedge the drain).
- Every record line is nonce-prefixed; the nonce is minted at the edge per run (DI'd). Lines
  without the nonce prefix = freeform leakage from a sloppy oracle body: logged, counted,
  warned once per host, ignored (never parsed as control — `notes/140` f-sec by construction).
- `book=` binds results to the exact book bytes analyzed — a stale/mismatched stream refuses
  fold (this discharges the r22 `tc-probe-no-digest`/`tc-probe-results-roundtrip` items folded
  into `22H` §6). `host=` must equal the session's expected HostId (mis-plumbed streams refuse
  rather than cross-pollute — the partition law's wire-level tripwire).
- Truncation semantics: absent end-sentinel, or `seen < sites`, ⇒ the un-received site set is
  Unknown for this host (§3 s3-4). The header's `sites=` is what makes the *range* computable.
- The inner `site N …` record grammar is untouched (owned by the existing emitter/parser —
  `cli` parse_results; note its planned entity-re-key churn, `cli/CLAUDE.md` ap-1: the framing
  wrapper is deliberately agnostic to the inner grammar so both can move independently).
- Emission: the framing lines are emitted by the probe artifact itself (compiler adds the
  header/sentinel `printf`s and the per-line nonce prefix at compile time). MUST remain plain
  POSIX-sh emission — no new runtime dependency on the host.

**Apply lane (v1).** No records. The apply artifact ships byte-floored (law-artifact-floor);
observables = exit status + captured stdout/stderr + the existing `DORC_EXIT=<n>` crash-guard
marker convention. State-truth after an apply is NOT its rc (`plans/252` §8 finding: rc=0 does
not prove services healthy) — it is the §6 `--verify` re-probe.

---

## §5. Transport spec — ssh mechanics

**Driver 1: ssh-subprocess (production default).**
- Invocation shape (per host, per phase): `ssh -F <dorc-ssh-config> [-i unchanged: user's
  business] -T <hostid> '<remote-sh>' -s < <artifact>` — the welded ssh-a-script floor; `-T`
  (no pty) is REQUIRED (a pty merges/cooks streams — `notes/140` f5, `notes/141` g4).
- **Config layering (differs from the r25 throwaway deliberately):** Dorc generates a small
  config carrying only its non-negotiables and *includes nothing secret*:
  `BatchMode yes` (never prompt mid-fleet), `ConnectTimeout`, `ServerAlive*` (§3 s3-6),
  `ClearAllForwardings yes` + `ForwardAgent no` (law-security-floor), `LogLevel ERROR`,
  and — on non-Windows where it works — `ControlMaster auto` + `ControlPersist 60s` +
  `ControlPath` under the run-dir (probe→apply→verify reuse; `plans/072` footgun-7). It is
  passed via `-F` ONLY when the user asks for hermetic mode; the DEFAULT composes with the
  user's own `~/.ssh/config` (aliases, ProxyJump, keys are *their* config — Dorc must not
  bypass it the way the trial did) by passing these as `-o` options instead. (§9
  dec-26-ssh-config records the choice; the r25 `usekeychain` scar is handled by documentation
  + a clear transport-error surface, not by hijacking the user's config resolution.)
- **Host keys:** default = OpenSSH's own behavior (known_hosts enforced; new hosts prompt
  refused under BatchMode ⇒ clean loud failure telling the user to connect once manually or
  pass the flag); `--accept-new` opt-in sets `StrictHostKeyChecking accept-new`. The trial's
  `UserKnownHostsFile /dev/null` is NEVER a product behavior (law-security-floor / PM-5).
- **rc discipline:** ssh exit 255 = transport-layer failure (plus a stderr heuristic table for
  diagnosis, cribbed from the P3 runner's `_transport_error`); anything else is the remote
  artifact's own exit status, passed through opaque (the engine never interprets — 23K). The
  collision (a plan genuinely exiting 255) is documented; transport_failed is derived from 255
  ∧ stderr-heuristic, not 255 alone.
- **CRLF gate (an-wire-transform; `plans/139` §5):** before shipping ANY artifact, assert its
  bytes are LF-only. On violation: **refuse loudly at plan time** with the one-line fix
  (`dos2unix`/gitattributes) — never silently rewrite user bytes (never-lie beats convenience;
  §9 dec-26-crlf). Detection is free (the parser already saw the bytes).
- **Fleet-frame stripping:** any remote bytes echoed into the controller's own TTY rendering
  (progress lines, error excerpts) pass through the existing control-char discipline
  (law-security-floor E5); full raw streams go to per-host capture FILES, not the shared TTY.

**Driver 2: local-subprocess.** `dash -s < artifact` as a child process with identical event
surface (used by: hermetic e2e for the full fleet path, `--local` smoke parity, and any
docker-fixture future). MUST share ≥95% of the driver code path with ssh-subprocess (only the
argv differs) so the hermetic tier genuinely exercises the production path.

**Driver 3: sim.** Feeds `fleet` from hostsim-derived synthetic streams under a seeded
interleaver; lives in test/dev builds. This is Seam-1's in-process substitution exactly as
`plans/128` L0/L1 specifies — no socket is ever opened in the DST tier.

**Concurrency at the edge (§9 dec-26-edge-runtime):** the edge driver pumps N child processes'
pipes concurrently. Requirement: no thread-per-host architecture *assumption* (law-perf-
redlines) — but at v1 scale (tens of hosts) implementation may be modest. Default: tokio
(process + io) confined to `transport`+`cli`, exact-pinned, mirroring the anstream precedent
(edge-only deps, kernel stays clean, `00664b1`). Acceptable alternative if the builder finds
tokio disproportionate: a small poll loop over blocking reads in scoped threads with a
bounded pool — provided the fleet kernel stays sans-io either way.

**Backpressure floor (`plans/072` footgun-8, scoped to v1):** per-host capture is file-backed
(bounded memory: stream-to-file, render-from-summary); the records lane is parsed
incrementally with a line-length cap + a per-host records-bytes cap (default 8 MiB, loud on
trip ⇒ that host degrades to probe-truncation semantics). Verdict/error lines are never
dropped; freeform capture may be truncated with an explicit `[truncated at N bytes]` marker.

---

## §6. CLI surface + UX floor

- **Host spelling (invocation-plane ONLY — `kOOB` guarded):** repeatable `-H/--host <dest>` +
  `--hosts <file>` (flat file: one ssh destination per line, `#` comments — an *inventory
  consumed, not built*, `plans/064`; ssh-config aliases are the expected currency). No hosts
  ⇒ existing single-host behavior byte-identical (also the merge-safety fence). Mixing
  `--results` with `-H` is a usage error (rc=2).
- **`dorc plan book.sh -H a -H b`** → per-host artifacts `book.dorc-plan.<host>.sh` (exact
  naming = builder's, but MUST embed the host and MUST be the ru-29 user-editable interface),
  plus per-host `plan-summary` lines extended with `host=<id>` (additive key — existing
  needles unaffected), plus the fleet summary (bad news first: unreachable/unknown hosts, then
  per-host `sites= elide= guard= run=`). Live-ness at v1 = per-host granularity: each host's
  summary prints as that host's fold completes (arrival order), which is the honest concurrent
  UX without a TUI. Plans are not dumped to stdout for N>1 (attention-honesty at fleet scale:
  files + summaries + `dorc why`).
- **`dorc apply`** — fleet form ships each host its own artifact concurrently (width-capped)
  and reports per-host outcome + aggregate. Consent flow unchanged from the existing
  single-host contract (plan → user reads → apply); apply consumes the emitted per-host
  artifacts (possibly user-edited — ru-29's whole point), with the book-hash check advisory:
  an edited plan is legitimate; a plan for a *different book* refuses.
- **`--verify` (stage 4):** after apply, re-probe and report per-site convergence per host —
  the state-truth read-back (rc ≠ health, `plans/252` §8(a)); output = one line per
  still-diverged site. This is derived machinery (probe reuse), no new analysis.
- **`dorc why … --host <id>`** (stage 4): host-scopes the existing why surface; unscoped with
  multiple hosts = per-host sections, unreached hosts first.
- **Exit codes (extends the `c6774dc` family additively; §9 dec-26-exit-codes):** existing
  0/2/10 unchanged; NEW `11` = plan completed with ≥1 unreachable/unprobed host (plans still
  emitted); `12` = apply partial failure (≥1 FailedApply/UnknownAfterLoss; detail per-host).
  `DORC_EXIT=<n>` marker convention carries the new codes; `run.sh` crash-guard learns them
  (additive).
- **Words:** the render vocabulary for host cells uses §3 s3-4's names verbatim (`unreachable`,
  `probed-partial`, `unknown after transport loss`, `apply failed (rc N)`) — never "skipped",
  never "offline≈ok".

---

## §7. Testing — the coordination-DST tier (this round's differentiating net)

This round BUILDS the tier `24B` §5 reserved (C-altitude): coordination-DST at the transport
stream, riding ON TOP of `hostsim::Host` as the per-host verdict oracle. The elision-soundness
net (`dorc-sweep`) is untouched. New tests live in `fleet`'s test suite + a small e2e addition.

**The harness:** N seeded `Host`s → synthetic per-host record-streams → a seeded interleaver
(the `22H` §2 arrival-ordering seam: one logical clock, DI'd; the ONLY new sanctioned
nondeterminism source, and it is seed-derived) → `fleet::step` consumes → assertions on the
state + emitted plans. Fault injection = stream-level mutations (drop/truncate/reorder/dup/
forge/stall), modeling the *outcome* never the kernel mechanism (`plans/128` fc-5 /
axis-platform — no netem, no real sockets, all-OS, hot-loop).

**Acceptance set (each a named pin; wording = the contract):**
- **acc-terminal-determinism** — for every host: incremental-final plan == single-shot plan on
  the full fact-set, byte-identical (law-determinism-acceptance).
- **acc-per-host-partition** — under adversarial interleaving of N hosts' streams, no fact
  ever influences another host's plan: run with host B's stream mutated arbitrarily, assert
  host A's plan byte-stable. (The REAL accumulator is driven — `22H` §4's warning that the
  pure-function version of this test is vacuous; the shared-accumulator version is the one
  that can fail.)
- **acc-interleave-invariance** — same per-host fact-sets under any arrival interleaving ⇒
  same final per-host plans (merge commutativity lifted to the fleet).
- **acc-monotone-tightening** — on the curated guard+body DST books, each host's run-count is
  monotone non-increasing across its arrival sequence (`22H` §4(a); expected-shape assertion —
  a violation is a finding to surface, not necessarily a bug, per s3-3).
- **acc-truncation-unknown-range** — truncate a host's stream after site k: sites ≤k keep
  facts, sites >k fold Unknown ⇒ run; the plan marks the boundary; never a false
  converged/diverged past the last record (`plans/128` fc-2).
- **acc-retry-is-reprobe** — after apply-transport-loss, assert the kernel emits NO re-ship
  command for the apply artifact (law-no-double-apply), and a subsequent re-probe+re-plan
  converges the host's state honestly.
- **acc-unreachable-never-converged** — an unreachable host's plan contains zero elisions and
  zero guards; every site runs; the render carries the unreachable marker (`plans/076` §3a).
- **acc-forged-verdict-contained** — a host forging Converged-for-everything elides only its
  OWN plan (contained blast radius), and only within what `kFAIL-perform` licenses; no other
  host's plan changes (the `plans/102` host-as-adversary cell, made a pin).
- **acc-pacing-cap** — with 50 sim hosts and open-cap 8, the command trace never exceeds 8
  concurrent opens; a seeded transport-refused burst triggers backoff, and the run still
  completes (no starvation).
- **acc-seed-bit-identical** — rerun any seed: the full command trace + final plans are
  bit-identical (the `24B` determinism guard; any divergence = a real bug — an ordering leak).
- **an-sometimes-assert coverage** — every fault path above carries a sometimes-assert
  (the path IS reachable under the seed corpus), per `plans/128` fc-5 / hostsim discipline.

**e2e (small, hermetic-first):** one new case family exercising the full CLI fleet path over
the **local-subprocess driver** (3 "hosts" = 3 local dash processes fed distinct fixture
oracle-results — real artifacts, real framing, real parse-back); one gated non-hermetic smoke
(`ssh localhost`, skipped unless `DORC_E2E_SSH=1`) for the ssh argv/config path. The in-memory
tier carries the logic (the `24I` de-graduation doctrine — do NOT balloon e2e); the e2e adds
the one-shot `dash -n` gate on every emitted per-host artifact as usual (ap-2).

---

## §8. The stage ladder (build order; each stage lands green before the next)

Gates for every stage: fresh `cargo build --workspace` · clippy `-D warnings` · full suites ·
existing e2e byte-stable (untouched cases MUST NOT churn — that is the merge-disjointness
tripwire firing) · the stage's own named acceptance pins green.

- **stage-26-0 — skeleton + determinism rig.** `fleet` + `transport` crates exist (workspace
  members added); event/command vocabulary compiles; seeded logical clock + interleaver
  harness; the determinism guard (rerun-seed → bit-identical trace) proven by deliberately
  breaking it once (inject a HashMap-ordered iteration; watch it fail; remove). Deliverable:
  acc-seed-bit-identical green on a trivial 2-host no-fault scenario.
- **stage-26-1 — in-memory fleet plan.** N hostsim hosts through the sim driver: per-host
  accumulators, arrival fold, per-host plan emission (library-level). Acceptance:
  acc-terminal-determinism, acc-per-host-partition, acc-interleave-invariance,
  acc-monotone-tightening. No CLI yet.
- **stage-26-2 — wire v1 + CLI plan fan-out.** Probe-artifact framing emission (compiler-side:
  header/sentinel/nonce-prefix printfs) + framing parser + book-hash refusal; the ssh-subprocess
  AND local-subprocess drivers; `-H`/`--hosts` + per-host artifacts + fleet summary + rc 11;
  pacing caps; CRLF gate. Acceptance: acc-truncation-unknown-range, acc-unreachable-never-
  converged, acc-pacing-cap, the hermetic e2e family, and the single-host byte-stability fence.
- **stage-26-3 — apply fan-out + failure taxonomy.** Concurrent apply shipping, per-host
  capture (file-backed), transport-loss → UnknownAfterLoss, rc 12, aggregate ordering,
  `DORC_EXIT` extension. Acceptance: acc-retry-is-reprobe, acc-forged-verdict-contained, the
  apply e2e case.
- **stage-26-4 — the read-back + polish.** `--verify` re-probe convergence report; `dorc why
  --host`; unreached-first render ordering; transport-error diagnosis table. Acceptance: a
  verify e2e case (diverged site survives a mocked apply ⇒ verify reports it).
- **stage-26-5 — measure, conclude, extract.** DST seed-corpus breadth run (all faults ×
  interleavings; sometimes-assert coverage report); the honest boundary write-up (what
  coordination complexity actually materialized — the `plans/128` se-5 question, now
  answerable with evidence); residue ledger; fold conclusions to LIVING_STATUS + a 26x note;
  propose KNOBS/ANALYZER-NEEDS row updates (do not edit human docs).

Estimated shape: stages 0–1 are the deep-thought half (kernel semantics); 2–3 are wide but
mechanical; 4 is small. A `notes/261` narrow-implementation companion (exact type signatures,
the framing emitter's placement in the compile pipeline, the tokio-vs-poll decision record)
is sanctioned as follow-up if the builder wants it pre-written.

---

## §9. Decision ledger (new calls this plan makes — ratify or veto; defaults are live)

Knob-registered tensions are NOT relisted (kCOMMS/kSCHEDULE/kSTATE/kFIDELITY/kAGENTLESS/
kWINLOCAL stand as registered). These are the plan-level calls beneath them:

- **dec-26-sansio-kernel** — the fleet kernel is a pure sans-io state machine (events in /
  commands out), not an async-native orchestrator. Buys: trivial seeded DST (the whole §7 tier),
  kernel dep-cleanliness by construction, runtime-agnostic edge. Costs: edge-driver boilerplate;
  slightly unfashionable. Alternative (tokio-through-the-kernel) rejected for law-kernel-purity.
- **dec-26-transport-impl** — v1 production driver = **system-ssh subprocess** behind the
  driver trait; in-process russh is a RESERVED swap (the `plans/139` §4 own-the-pool lean is
  honored as a seam, not built). Buys: proven on this exact estate (P1/P3 scars encoded),
  user's ssh config/agent/ProxyJump all just work, zero crypto deps. Costs: no ControlMaster on
  Windows controllers (accepted: 2–3 handshakes/host, dominated by remote work per `plans/076`);
  subprocess-pipe plumbing.
- **dec-26-wire-v1** — records ride the probe artifact's framed stdout (§4) instead of the full
  142 remote-FIFO/per-leaf-file layout. Buys: zero new remote-fs assumptions, smallest
  correct thing, grammar survives the later move. Costs: within-host record-liveness deferred
  (fleet-liveness is per-host at v1); freeform leakage from sloppy oracle bodies shares the
  channel (mitigated: nonce-framing + count-and-warn). The 142 layout remains the growth shape;
  nothing here forecloses it.
- **dec-26-liveness-tier** — v1 "live plan" = per-HOST arrival granularity (plans print as
  hosts complete), not per-record streaming. Honest reading of `DESIGN`'s realtime aspiration
  at v1 host-counts; the fast-lane drain upgrade is additive later. (The 22H §2 engine is
  still built multi-batch-capable and DST-exercised — only the v1 WIRE is single-batch.)
- **dec-26-ssh-config** — compose with the user's own ssh config via `-o` options by default
  (their aliases/ProxyJump/keys are the credential plane; Dorc holds nothing); a hermetic `-F`
  mode exists as a flag. The r25 trial's config-hijack (`-F` always + known-hosts /dev/null)
  is explicitly NOT product behavior. Host keys: enforced by default, `--accept-new` opt-in.
- **dec-26-crlf** — CRLF artifacts refuse loudly at plan time with the one-line fix; never
  silent normalization (never-lie > convenience; `plans/139` §5's conscious choice, taken).
- **dec-26-apply-visibility** — v1 apply observability is whole-artifact (rc + captured
  streams + DORC_EXIT); NO per-leaf apply markers (law-artifact-floor). Consequence: mid-apply
  loss minted as whole-host UnknownAfterLoss, localized by re-probe (which the product makes
  one command). Per-leaf apply progress belongs to the deferred faithful-mode leaf-wrap
  (`plans/077` Seam-2) and is NOT smuggled in early.
- **dec-26-edge-runtime** — tokio at the edge crates only, exact-pinned (anstream precedent);
  kernel unaffected either way; builder may substitute a bounded-thread poll loop if tokio
  proves disproportionate — the requirement is law-perf-redlines (no fork-per-host
  architecture), not a specific runtime.
- **dec-26-hosts-spelling** — hosts are invocation-plane only (`-H` + `--hosts FILE`,
  ssh-destination strings consumed verbatim); no in-book host metadata (kOOB redline), no
  inventory system (consume, don't build — `plans/064`). Groups/patterns deferred.
- **dec-26-exit-codes** — 11 (plan, partial-unreachable) and 12 (apply, partial-failure)
  extend the existing 0/2/10 family; builder verifies non-collision and updates the crash-guard
  contract additively.
- **dec-26-probe-retry** — transient transport failure during probe = bounded auto-retry (×2,
  backoff) because probes are read-only by contract; apply NEVER auto-retries. (The asymmetry
  is the kFAIL phase-keying spelled at the transport.)

---

## §10. Merge-disjointness contract (the tight-timeline insurance)

r26 will be merged back over in-flight language-surface changes on r23. The contract:

**r26-owned (new files only, conflict-free by construction):** `spike/crates/fleet/**`,
`spike/crates/transport/**`, new e2e case dirs (`fleet26-*`), `Research/plans/260*` +
`Research/notes/26*`.

**Shared-file touchpoints (keep each edit minimal + additive):**
- `spike/Cargo.toml` — workspace members += 2 lines.
- `spike/crates/cli/` — ONE new module (`fleet_cmd.rs` or similar) + a bounded hook in
  `main.rs` (arg recognition + dispatch; target ≤ ~30 lines of diff in main.rs). If the
  sibling's CLI work collides, the hook re-lands trivially; the module does not.
- `spike/crates/hostsim/` — additive module(s) for the fault/stream seam; no edits to existing
  `Host` API surface.
- The probe-compiler emission point (framing printfs) — the ONE kernel-adjacent edit; keep it
  a self-contained function call at artifact-assembly, clearly bounded, so a language-side
  render change rebases past it.
- `spike/e2e/run.sh` — crash-guard learns rc 11/12 (a few lines, additive).

**Explicitly NOT touched:** `syntax`/`oracle`/`analysis`/`plan` semantic code paths;
`core` (HostId etc. live in `fleet` — resist the temptation to "properly" home types in core
until AFTER the merge); all existing e2e cases + goldens (byte-stability is a standing gate,
§8); root docs (human-owned); KNOBS/AN (propose edits at stage 5, don't apply).

**Consumed-API watch:** `plan`'s `classify`/`build_plan`/render entry points and
`parse_results`. If the r23 language work changes these signatures, the rebase cost is
localized to `fleet`'s call-sites + the framing parser — flag, don't fork. Rebase cadence:
after each stage lands, rebase `ai/spike3-r26` onto the then-current `ai/spike3-r23` and
re-run the full gates (the branch is set up tracking-by-rebase already).

---

## §11. Deferred / reserved ledger (named, with re-entry pointers)

- Within-host batch-parallel probes (N pristine channels + reserved drain + per-leaf files) —
  `plans/142` resolution, re-enter when per-host probe latency actually hurts (kCONC parked).
- Live record-streaming fast-lane (FIFO/tail drain) + rich TUI — `plans/142` + ru-20.
- Executor corner {no-writable-fs, hard backpressure} — `plans/142` re-pin.
- Rolling/serial/canary + readiness-gate-as-re-probe — `kSCHEDULE` + `plans/076` §4 (the
  `admit()` hook is the landing site); throttle/jitter/shared-resource herd derivation —
  `plans/072` (needs oracle cost-classes, unbuilt).
- Fan-out tree / result-aggregation relay — `plans/076` §6-5 (Salt-syndic lesson recorded).
- Cross-host facts, bridges, rolling-update coordination — ru-28 (human re-opens or nobody).
- Cross-host verdict memoization `(verdict, content-key, freshness)` — `plans/076` §4b, HARD
  gated behind kSTATE's hostile-host security fence.
- In-process ssh (russh) driver — `plans/139` §4; slot exists behind the driver trait.
- Windows/tier-B targets, busybox bootstrap — `plans/139` §3 / `plans/deferred/13A`.
- Per-leaf apply progress (faithful-mode leaf-wrap, `DORC_LEAF_ID`) — `plans/077` Seam-2.
- Historical note: a round-16 direction sketch on apply+multihost exists in quarantine
  (`16A`, unread per the fence); if the human wants it weighed against this plan, that is a
  human call to open it.

## Cross-refs
`plans/22H` (the engine spec this productizes) · `plans/142`+`notes/140`/`141` (transport
resolution) · `plans/128`+`plans/121` (Seam-1/DST architecture + fault ladder) ·
`notes/072`+`plans/076` (perf redlines/ceilings) · `plans/102`+`101` (threat model) ·
`notes/23K` (lane discipline) · `plans/064` (SEAM scoping) · `plans/139` (platform/ssh) ·
`notes/24B` §5 (coordination-DST reservation honored) · `Research/trial/apply/*` (P3 scars:
usekeychain, 255-discipline, timeout table) · `plans/111` (loc-host provenance vocabulary,
consumed not extended) · KNOBS: kCOMMS · kSCHEDULE · kOBJECTIVE · kFIDELITY · kAGENTLESS ·
kSTATE · kWINLOCAL · kTPLATFORMS · kFAIL.
