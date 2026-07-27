# 26D — round-26 seed: live execution (the ssh executor + the acceptance loop)

AI-authored (Fable conductor), 2026-07-27, minted at human direction at the r28 close. THE seed
for the r26 planning conductor: remit, settled law, as-built inventory, salvage map, sharp
edges, and open human decisions — self-contained (grounded in a six-agent corpus trawl over
rounds 1–28 plus main-context reads of `142`/`252`/`254`/`255`/`256`/`260`/`262` and
`Research/trial/*`; the code-recon claims below carry file:line pointers into the
`.claude/worktrees/r28-unify` tree at seed time). Root `TODO-ADDTL.md`'s top section is the
paired triage view (per its own header, never cite it from durable docs — the substance is
mirrored here).

**The remit (human-typed 2026-07-27, paraphrased):** r26 = everything to do with piping things
*through operating systems* — the ssh executor, live-acceptance testing (WSL/containers), the
human's Vultr experimentation — at SKELETON-TIER completeness. Explicitly OUT of r26:
the oracle stdlib (human-ruled same day: pending, NOT blocking — "stdlib, multihost, and the
r25-first-blood protocols have mostly stood in the way of actually experimenting"; scrappy
hand-written oracles are part of the experiment itself); multi-host (the r26 revival, `26B`/
`26C`); the r25 ceremony (superseded by an informal live run — its *protocol* is dead, its
*tooling and findings* are salvage, §4/§6); the why-surface W4/W5 tails (their own queues).

## §1. The mission floor (pipe-completeness, human-typed)

`dorc plan … >plan.sh`, then `dorc apply host.somewhere.tld <plan.sh`, and the server is in
the state. Flags and UI are malleable; much can be omitted for experimentation; but manually
ssh'ing artifacts over defeats the point — *dorc itself doing the actual ssh'ing* is the one
functional floor. (Probe-side implication: `dorc plan book.sh <host>` must likewise ship and
execute its own probe rather than reading hand-fed stdin, or the pipe is not complete.)

## §2. Settled law the r26 plan composes (do not re-derive; ratified pointers)

- **`142:Resolution` (r14 close)** — transport is *mechanize-ssh, executorless* (`kAGENTLESS`
  welded r10). The Mitogen-style bootstrapped executor was costed and RE-PINNED to the
  {no-writable-fs, hard-backpressure} corners. The full r14 topology (channels-are-batches,
  FIFO fast-lane, per-leaf diagnostic files) is the *eventual live* shape; single-channel
  whole-artifact collected-after is its sanctioned degenerate start (re-confirmed by
  `260:dec-26-wire-v1` as the "sanctioned degraded-start instance").
- **`plans/260` §5 (+`26A` amendments) — THE adjudicated ssh-transport spec.** Written for the
  fleet but directly consumable at N=1. The single-host-relevant rules, verbatim-tier:
  - invocation: `ssh … -T <host> '<remote-sh>' -s < <artifact>`; `-T` (no pty) REQUIRED
    (a pty cooks streams — `notes/140` f5);
  - config layering (`dec-26-ssh-config`): DEFAULT composes with the user's own ssh config
    via `-o` options (`BatchMode yes`, `ConnectTimeout`, `ServerAliveInterval 15`/`CountMax 4`,
    `ClearAllForwardings yes`, `ForwardAgent no`, `LogLevel ERROR`); hermetic `-F` is an
    opt-in flag. The r25 trial's `-F`-always + `UserKnownHostsFile /dev/null` is explicitly
    NEVER product behavior;
  - host keys: OpenSSH's own enforcement by default (BatchMode makes new-host prompts a clean
    loud failure); `--accept-new` opt-in. Never blind-accept (`102` E2/PM-5);
  - **the completion sentinel (`26A` stop-2)**: the remote *command line* (never the
    byte-floored artifact) runs the artifact then prints an end-marker carrying `$?`. Marker
    present ⇒ genuine remote exit, classify by carried rc; absent ⇒ transport sever ⇒
    UnknownAfterLoss, regardless of ssh's rc-255 or stderr. rc-heuristics demote to diagnosis
    only. This dissolves the rc-255 collision `apply-run.sh` documents;
  - fail-direction at the transport: probe auto-retry bounded (×2, backoff, read-only by
    contract); apply NEVER auto-retries (`law-no-double-apply`) — the sanctioned recovery is
    re-probe-then-re-plan ("the probe is the retry-file");
  - timeouts, all injected: connect ≈15s; probe wall-clock default 120s/host; apply default
    unlimited with `--apply-timeout` opt-in (killing an apply mints Unknown);
  - **CRLF (`dec-26-crlf`)**: assert shipped bytes LF-only; on violation refuse LOUDLY at
    plan time with the one-line fix — *never silently rewrite user bytes*. Re-run the gate on
    the shipped bytes at apply time (the user may have edited the plan file on any OS);
  - privilege: run as whatever user the ssh destination resolves to; no escalation; a probe
    needing root can't-tells ⇒ run;
  - remote-bytes hygiene: anything echoed to the controller TTY passes the control-char
    discipline; full raw streams go to capture files.
- **`plans/262` §2 — the `dorc-records/1` wire contract**, PARTLY LANDED via
  `270:wire-records-v1-import` (framing, terminal token, `book=`/`host=`/`sites=`/`attempt=`
  integrity keys, torn/glued refusal, truncation-by-lane, merge-by-meet, additive-keys).
  r26 must audit as-built vs spec, and thread REAL host/nonce/attempt identity into the
  `Expect` — today's `Framing::spike` constants (`plan/src/records.rs:65-90`; sole production
  call sites `cli/src/main.rs:1041`/`1573`) make the host-mismatch integrity check vacuous.
- **Standing engine law**: ssh spawning is `cli`-edge only, behind a DI seam with a sim driver
  (`inv-determinism`; `260` §2's SessionDriver trait shape: ssh-subprocess · local-subprocess ·
  sim — the local driver MUST share ~all of the production code path); `kFAIL` phase-keying;
  whole-artifact-per-phase is the network unit (`law-seam-1`); no reordering; unreachable ≠
  converged, timeout-killed ≠ clean.

## §3. What exists today (code recon, 2026-07-27, `r28-unify` worktree)

- **CLI**: modes probe/plan/apply/round-trip/why/strip/lint (`cli/src/lib.rs:82-104`); results
  from `--results FILE` or stdin (`main.rs:1085-1097`); admission is the closed
  `Admitted`/`NoObservation`/`Refused` (`records.rs:594-872`). **No host concept anywhere.**
- **No execution machinery** beyond two local `Command::new` sites: the lint tools runner
  (`main.rs:290-333`) and the `dorc-sh` shebang bin (`bin/dorc-sh.rs:67-72`). No ssh code in
  any crate. `hostsim` never spawns (answers-facts-never-runs-sh).
- **The report lane is already remote-ready**: the `DREP_V1` scratch machinery is baked into
  the shipped artifact's own sh (`plan/src/render.rs:219-266`; `SCRATCH_ROOT=/tmp`, `mkdir -m
  700`, drain-to-stdout-records, degrade-to-/dev/null) — works over ssh unchanged.
- **The whylog is controller-local by design** and needs nothing for remote hosts. OPEN
  QUESTION (do not build silently): ingesting a *real* apply's outcome into the whylog — today
  the "apply report" is plan-time prediction (`whylog.rs:97`) — may collide with
  `whylog-write-only-replay`/rec-5; human ruling wanted.
- **The loop has never been closed, even test-only**: `probe_exec_check` (e2e.rs:1694-1812)
  really executes rendered probes under inert mocks but only *compares* output against the
  authored `probe-results.txt`; `exec_check` (e2e.rs:1601-1688) really executes apply
  artifacts built from that same authored fixture. Captured-real-probe-output → `--results` →
  apply-build exists NOWHERE. Closing this chain is the seed of the acceptance tier (§5).
- Environment, measured: Windows controller (git-bash; docker NOT on PATH); WSL2 Ubuntu
  RUNNING (`/bin/sh` → dash, `apt-get` present, docker absent, posh absent); OpenSSH 10.3.
  No mise task touches ssh/remote execution.

## §4. Salvage map (r25 tooling — squash-merged to mainline at `Research/trial/`)

- **`trial/apply/apply-run.sh`** — the proven ssh-a-script wrapper:
  `ssh -F <sibling config> -i "$SSH_KEY" -T <host> "$REMOTE_SH" -s < plan`, timeout-wrapped
  (124/137 detection), rc/stdout/stderr captured to a run-dir, `C-run/1` JSON summary +
  transcript. Has a `--local` mode (local dash). Immediately reusable for *experimentation*;
  the product internalizes the same shape per `260` §5 (its rc-255 transport heuristic is
  superseded by the completion sentinel).
- **`trial/apply/ssh_config`** — trial-only posture (accept-new + known-hosts /dev/null,
  root login, IdentitiesOnly): NEVER product (`dec-26-ssh-config`). Carries the load-bearing
  **usekeychain scar**: the human's `~/.ssh/config` contains macOS-only `UseKeychain`, which
  git-bash OpenSSH rejects (exit 255) — so on THIS controller, `260`'s
  compose-with-user-config default will break out of the box. r26 must decide the posture
  here (candidates: `-o IgnoreUnknown=UseKeychain` in the composed options; or hermetic-`-F`
  default on Windows controllers) and test it on this exact machine.
- **`trial/vultr/vultr.sh`** — full throwaway-VPS lifecycle (provision/snapshot/restore/
  destroy/status/destroy-all/run) under the `252` §5.1 guardrail (tag `dorc-r25`, ≤3 boxes,
  cheapest tier ~$0.007/hr, teardown-always, key sourced from `~/.temp/vultr.env` only,
  auth-failure = HARD STOP, refuses `set -x`). Debian-12 (`os_id 2136`). Reusable for the
  opt-in real-VPS tier; re-tag for r26; the key/allowlist may have aged — verify before spend,
  and every spend stays human-acked per the guardrail.
- **`trial/observe/{observe,recon}.sh`** — the dorc-independent machine-delta instrument
  (P2, wider-than-dorc's-model by rule). Optional for r26; becomes interesting again if the
  acceptance tier ever wants differential (bare-apply vs dorc-apply) verification —
  the full method incl. A/A noise calibration is `252` §7/F2.
- **`trial/hhhf/`** — human-session instrumentation; irrelevant to r26.
- Findings that survive the discarded ceremony: the differential/diff-of-deltas method
  (`252` §1 P4), the pre-registration discipline (`254` F1: numbers before the run), and the
  `255` prediction ledger (§6 below).

## §5. The acceptance loop (real ssh + real apt at gate/bless tier — never hot-loop)

The human's standing ask: during LLM work, final acceptance of large bodies of work should
exercise *actual ssh* and *actual apt-get*. Recommended tier ladder (r26 builds T1, wires T2;
T3 stays manual/spend-gated):

- **T0 (exists, unchanged)** — e2e under inert PATH-mocks, scrubbed env. The hot-loop.
- **T1 (the r26 gate deliverable)** — the CLOSED LOOP over the local-subprocess driver:
  `dorc probe` → real execution under local dash (Windows: via `wsl -e`) → captured records →
  plan/apply build → real apply execution → assert converged/re-run-elides. Book mutates only
  a sandbox directory (no root, no packages). Hermetic enough for `gate:full`/bless; this is
  also the test that finally closes the never-closed chain (§3).
- **T2 (opt-in env-gated, `DORC_E2E_SSH=1`-style)** — the same loop over REAL sshd
  (sshd-in-WSL on localhost is the zero-spend target; any reachable host works). Covers what
  no sim can: BatchMode/key/auth behavior, stream capture over a real channel, sever-mid-line
  (`260` §7's non-hermetic smoke sketch). Bless-adjacent, not default-gate.
- **T3 (manual, spend-gated)** — `vultr.sh` throwaway Debian-12 + a real apt-get book: the
  human's experimentation session, optionally scripted as a "full-blood" check. Docker-in-WSL
  slots between T2 and T3 (disposable-root apt-get with zero spend) *iff* the human chooses to
  install docker — a system mutation that is theirs alone; absent that, T2+T3 suffice.

## §6. Sharp edges for the first live run (operator notes — verified in-corpus)

- **The `255` book AS-WRITTEN measures elide=0**: its `case "$(hostname)"` host-guard is an
  unmodeled-command substitution and walls the ENTIRE book (`255` §5.1 vf-1, in-repo verified
  2026-07-05). Drop the guard, or expect zero elision and treat it as the known artifact. Also
  vf-2: a *bare* `apt-get update` under `set -eu` does not elide (errexit-consumed status);
  the `dpkg -s x || apt-get install x` guards DO lift (errexit-exempt `||`-left).
- The stdout-consuming version-guard (`svc --version | grep -q X || download`) DOES lift as of
  r24's connected-pipes work (`252` §8 annotation: LANDED, tripwire promoted green) — the
  three-vendor value-curve in `255` §2C is reachable, modulo oracles being written on the day.
- No stdlib exists: every unmodeled command runs, honestly walled — safe, but the run
  demonstrates elision only where the human hand-writes scrappy oracles (their stated intent).
  Authoring traps for those scrappy oracles: converged≠no-op judgment (`24U` §2), the `27Q` §2
  quality bars, and `255` dec-5's expired-cert case as the cleanest adequacy demo.
- `su -c`/sudo-wrapped sites: opaque payloads and context-gating — expect walls/guards, not
  elision (`27C`; `255` §3's residue map is the honest floor: ~4 sites never elide).
- Sizing: the `255` book wants a ≥2GB VPS (grafana+prometheus+HA; `255` §8).
- The whylog is now default-ON and holds raw host metadata unsanitized
  (`AID-NEEDS:law-whylog-is-sensitive`) — acceptable for a throwaway box, re-grade before any
  real estate.
- CRLF: the refuse-loudly gate is UNBUILT today; until r26 lands it, hand-verify the book/plan
  files are LF before shipping (a CRLF shebang is an un-guardable kernel-level exec failure on
  the remote — `139` §5). This authoring workflow (Windows checkout → Linux host) is the
  live one.

## §7. Suggested r26 shape (seed opinion, NOT a charter; the next conductor cuts it)

- **lane-engine-executor** — host argument + the SessionDriver seam in `cli`/(new `transport`
  module): ssh-subprocess + local-subprocess + sim drivers (`260` §2 shape, fleet kernel
  explicitly NOT built — single host, no `-H` fan-out, no pacing); the completion sentinel;
  real records `Expect` identity; the CRLF refuse gate; the `260` s3-6 timeout set. ~SUSPECT
  days-not-weeks at Opus tier, given the wire/admission side is already built.
- **lane-acceptance-gate** — T1 closed-loop case family + T2 env-gated sshd lane; mise task +
  bless wiring; never hot-loop.
- **lane-experiment-kit** — thin glue marrying `vultr.sh` (re-tagged, guardrail intact) to the
  new executor for the human's Vultr session; keep it disposable-tier.
- Standing riders it should NOT absorb: multi-host anything (r26), the locator-DAG N-tier
  (first consumer is multi-host), `289:seam-narrative-render-unconsumed` (arrangement-home
  round), stdlib authoring.
- **Open human decisions to collect at charter time**: (d1) ssh-config posture on Windows
  controllers (IgnoreUnknown vs hermetic default — §4's scar); (d2) docker-in-WSL install
  yes/no (unlocks the zero-spend real-apt tier); (d3) whether real apply outcomes enter the
  whylog (the rec-5/write-only collision, §3); (d4) whether the `260` exit-code family (11/12)
  ships at N=1 or waits for the fleet; (d5) sshd-in-WSL setup (a system mutation on the
  controller box — human-owned).

## §8. Fold-state warnings (measured 2026-07-27; verify before r26 branches)

- `ai/main` does NOT contain the W4 arc: `ai/r28-unify` (tip incl. the `w4-carrier`/`-map`/
  `-parts`/`-span`/`-drifted-driver` lanes; conductor-blessed green @ `747ab48d` per the
  worktree LIVING_STATUS) is unmerged; the W1–W3/weft/ascii/loom-cleanup/opaque/speechact/
  d1-drift lanes ARE merged. A fold conflict is pre-banked: the human's `f4f48316` webhost
  redline vs the unify branch (`28H:item-webhost-redline-orphaned` — 28H/28I/28J are
  worktree-resident notes until the fold).
- `ai/r28-declined-rerank` and `ai/r28-precommit-honesty` are also unmerged (small lanes;
  disposition unknown to this seed).
- `ai/r27-aid`, `ai/r28-phase3-close`, `ai/r28-errorloom-phase2`, `ai/spike3-r23`, and
  `ai/spike3-r25` no longer exist as branches (measured absence; ~SUSPECT folded-then-pruned —
  r25's content is confirmed in-mainline at `Research/trial/`, merge `2d5176dd`).
- Three `*.sync-conflict-*-PHNHRER` twin branches exist (SyncThing incursions); the
  `.stignore` repair remains human-owned (`27U` §2).
