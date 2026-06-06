# plans/142 — controller↔host transport: problem-space map & gate (round 14)

**Stamp:** round 14 · 2026-06-04 · plan/gate after gather turn 1 (`notes/141`, 7 graded primaries). The `kCOMMS` transport substrate. This is the **variance map + proposed fronts**; per the skill's after-first-pass gate it awaits human adjudication *before* front-work. Do not treat the lean below as decided.

## Shape verdict: was GENUINE VARIANCE — RESOLVED at round-14 close (2026-06-04)
The gather converged on a mechanism and left two forks (`ax-backpressure`, `ax-executor`); both were worked through in-conversation and are now settled. Read the Resolution first — it supersedes the `## My read` lean below. The map/axes/ambiguities/fronts are kept as the reasoning.

## Resolution (round-14 close) — the settled executorless design
- **Topology.** Per host, ~`MaxSessions` (≈9–10) SSH channels, each a *batch* of checks run with internal `&` concurrency — channels = batches, NOT leaves (decouples concurrency-width from the channel cap; the per-leaf-channel ceiling is rejected). `--faithful`/debug mode = one check per batch (no `&`), so a batch channel's output is per-check-attributable.
- **Tool I/O.** Rides the native batch channels at FULL stdout/stderr fidelity, untouched on the remote — no collapse; any merge is controller-side, conditional, post-analysis. Normal-mode within-batch interleaving makes a batch's freeform un-attributable → tossed/shown-raw; debug-mode is clean.
- **Dorc-signalling is out-of-band from the tool channels, split by size/urgency:**
  - short *gating* signal — rc, and the `(verdict, content-key, freshness)` triple (the content-key is a fixed-size proxy for arbitrarily-large state, so gating stays sub-`PIPE_BUF` *by construction*): a fast lane — a shared FIFO (or append-file) drained by ONE reserved channel — atomic-safe because short; supplies the one-live-ordered-stream the real-time plan needs.
  - large *rich* diagnostics — lint, warnings, why-details, oracle-configured-state dumps: **per-leaf files** (single-writer per file ⇒ arbitrary size, no atomicity ceiling), demuxed by *filename* (leaf-ids known a-priori from the compiler), drained by `tail -f`/poll or fetched on batch-close (latency-tolerant; non-gating).
- **Why this isn't papering over the atomicity limit:** for *signalling*, size ⊥ urgency — gating is short (or a short content-key), large is diagnostics that tolerate latency; the only executor-forcing combination (large + urgent + live-multiplexed-into-one-ordered-stream) doesn't arise, because the large-and-live thing is the tool's freeform, which is on its native channel, not the signalling lane.
- **Security = structural, not probabilistic.** Signalling never shares a lane with freeform (separate files/FIFO vs native tool channels), so attacker-controlled *data* in freeform cannot reach the control lane — injection prevented by construction; nonce/escaping are backup. Defeats accidental-collision + attacker-data-in-freeform; malicious-oracle-*code* is out-of-scope (it can mutate the host outright — state that boundary in `plans/102`).
- **Executor (`kCOMMS-executor-OOB`) re-pinned.** Justification shrinks from "concurrency/attribution" (pure-sh batching + per-leaf files cover those) to a narrow corner: {no-writable-fs targets (mitogen runs in RAM; files/FIFO need `/tmp`), hard backpressure (per-leaf files grow unbounded → size-caps + cleanup; an executor flow-controls), and the phantom large+urgent+live-mux cell}. Deferred-not-irrelevant.

Resolved axes: `ax-live` → live; `ax-backpressure` → per-leaf files primary, FIFO demoted to the fast lane for *short verdicts only*; `ax-executor` → executorless default, executor re-pinned to {no-fs, backpressure}; `ax-conc` → resolved by batching (`&` within batches), transport no longer gates on `kCONC`; `ax-channels` → channels = batches ≤ `MaxSessions` + one reserved drain channel.
*(Supersedes the `## My read` lean below, and `notes/140` f13's "concurrency is the main executor justification" + its "file = not live" framing.)*

Residual open (the real remaining work):
- `front-C` / `amb-1` — writable-fs (`mkfifo` + `/tmp`) availability across targets, esp. Windows-as-host (BusyBox/msys2/WSL2) + read-only-rootfs appliances. Now THE residual: gates the file-based design and pins the executor's no-fs justification.
- backpressure hygiene — per-leaf file size-caps + cleanup.
- hung-check timeouts (`flag-1`) — a hung check stalls its batch's `wait`; needs per-check/per-batch `timeout`.
- dependency: the `(verdict, content-key, freshness)` shape (`TODO-ADDTL`) — the fast lane assumes the gating signal is short, which *requires* that shape. Cross-locked.

## The landscape — 5 coherent architectures
(live = status visible as the probe runs; pristine = the N probe channels carry only real tool rc+stdout+stderr, zero Dorc bytes)

| id | live? | executorless? | pristine N? | backpressure | BusyBox-safe | concurrency-clean | knob |
|---|---|---|---|---|---|---|---|
| `arch-multiplex-inband` | yes | yes (pure-sh) | **no** (Dorc bytes ride stdout) | n/a | yes | no (interleave/spoof) | `kCOMMS-transpilation-inband` |
| `arch-collected-file` | **no** | yes | yes | none (writer never blocks) | yes | yes (per-leaf files) | OOB, executorless |
| `arch-live-oob-fifo` | yes | yes | yes | **couples probe→controller** (full 64KiB FIFO blocks the probe `printf`) | mkfifo? | verdicts yes, freeform-attrib no | OOB, executorless |
| `arch-live-oob-file-tail` | ~live (poll latency) | yes | yes | **decoupled** (writer appends, never blocks) | tail -f? | verdicts yes | OOB, executorless |
| `arch-executor` | yes | **no** (bootstrap a shim) | yes | shim buffers/flow-controls | needs the shim | **yes** (one framed protocol) | `kCOMMS-executor-OOB` |

Grounding: the reserved-lane idea is independently reinvented by apt `APT::Status-Fd` [A-debian-apt-progress-reporting-2022], bats fd 3 [A-bats-core-writing-tests-2024], and pdsh's second-connection-for-stderr [A-pdsh-readme-2024]; FIFO multi-writer atomicity is the mux primitive [A-man7-pipe-2024]; the lane must be remote-side because fds don't transit ssh [B-unix-se-ssh-fds-not-transportable-2025]; no-pty is forced because a pty mangles streams [B-kamil-ssh-separate-streams-tty-2021]; debconf shows the pure-sh line-protocol-over-standard-streams precedent [A-debian-debconf-devel-protocol-2014].

## Axes the decision turns on
- `ax-live` — collected-after ↔ live-streamed. **Largely decided → live** (`notes/140` f12: you want the plan built in front of the user in real-time). Kills `arch-collected-file` as the *primary* (it survives as a degraded fallback).
- `ax-backpressure` — FIFO ↔ file+tail. **Open, genuine pareto.** FIFO is truly live but a stalled controller-reader blocks the probe's own `printf` (full-pipe), coupling probe progress to controller liveness; file+`tail -f` decouples (the writer always appends) at the cost of an on-disk artifact + poll latency. [A-man7-pipe-2024] (FIFO blocking) vs the tail model.
- `ax-executor` — pure-sh OOB ↔ bootstrapped executor. **Open, the big one.** The executor's *only* distinctive buy is clean freeform-attribution under intra-host concurrency (`notes/140` f13); everything else pure-sh+FIFO does. Coupled to `ax-conc` and to your `kAGENTLESS`/trivial-off-ramp values. Counter-example cost is small — mitogen bootstraps in a 400-byte cmdline + 8 KB of source — but it abandons "it's just shell."
- `ax-conc` — how much intra-host probe concurrency? (`kCONC`, parked; overlaps `kFLATTEN`.) **Drives `ax-executor`.** If probes stay linear-ish per host, pure-sh wins outright; if you want wide intra-host fan-out with attributable freeform, the executor gets pulled in. Corpus-sized.
- `ax-channels` — reserved-channel count vs `MaxSessions=10`/host (`notes/140` f4; pdsh's 256-socket `rresvport` pool + fanout-exhaustion [A-pdsh-readme-2024]). A constraint, not a dial: 1 reserved + N probe channels must fit the per-connection cap, or spill to more connections (hop-expensive).

## What's ambiguous (empirical / needs corpus, not more reading)
- amb-1. BusyBox/stripped-target prevalence of `mkfifo` and `tail -f` (gates `arch-live-oob-fifo`/`-file-tail` on the lowest targets). oq1.
- amb-2. How often real oracle code needs **freeform-attribution-under-concurrency** — the one thing pure-sh loses and the executor's whole justification. If rare, the executor is dead weight.
- amb-3. How common the **EOF-inheritance hazard** is (g5: a backgrounded child inherits the reserved fd → the drain hangs). Determines whether the FIFO drain needs an explicit end-sentinel instead of EOF-detection. [A-bats-core-writing-tests-2024]

## My read (a lean, NOT a decision)
Executorless is the right default and the evidence backs it: `arch-live-oob-file-tail` for the common case (decoupled backpressure suits a probe phase that must never block on controller liveness; latency is sub-second and the probe phase tolerates it), with `arch-live-oob-fifo` where true-live matters and the controller drain is guaranteed prompt. The executor (`arch-executor`) earns its bootstrap *only* if amb-2 says freeform-attribution-under-wide-intra-host-concurrency is common — otherwise it's a `kAGENTLESS`/off-ramp regression for a corner. The reserved lane is nonce-prefixed colon-delimited line records (apt's `pmstatus:…` shape [A-debian-apt-progress-reporting-2022]); drain keys off an explicit end-sentinel, not FIFO-EOF (amb-3 mitigation).

## Proposed research fronts (the gate picks which to run)
- front-A — **live-framing & fan-in at scale:** ClusterShell tree-mode grooming/aggregation, `mcp-ssh-live` SPEC (channel-per-job ring-buffer + `tail(since_line)` near-live + the block-buffering gotcha), tmux control-mode `%output`/`%begin`/`%end` framing. *Q: how do mature tools frame + aggregate live multi-stream, and does any do it executorless?*
- front-B — **the executor option, costed:** mitogen minimal-bootstrap deep-read (the 400B/8KB technique) + whether a thin *pure-sh-spawned* shim (not full Python) captures the freeform-attribution buy at a fraction of the `kAGENTLESS` cost.
- front-C — **lowest-target reality (amb-1):** mkfifo/`tail -f`/named-pipe availability across BusyBox/toybox/dash; the corpus check that gates the FIFO/file options.
- front-D — **backpressure & resumability:** how live-tail tools survive reader-stall + reconnect (ssh-obi bounded-ring eviction; bounded-buffer + end-sentinel; the `notes/140` "concurrent+live" corner).

Gate outcome (round-14 close): worked in-conversation, see Resolution. `front-B` (executor) resolved → re-pinned to {no-writable-fs, hard backpressure}. `front-C` (writable-fs / Windows-host) is the live residual. `front-A` (live-framing at scale) + `front-D` (resumability) deferred — not needed for the decision. `ax-executor` reframed: the real need is *not* freeform-attribution-under-concurrency (pure-sh per-leaf files cover it) but {no-fs, backpressure}; `ax-backpressure` → per-leaf files primary, FIFO demoted to the short-verdict fast lane.
