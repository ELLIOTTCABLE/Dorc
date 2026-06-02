# Pluggability & the hook surface — core-tool constraints so a privileged tracing tool can later exist

> ⟢ 2026-06 — "there is no metadata layer / all-sh" is now the knob `kOOB` (lean: *minimize* out-of-band, not deny it exists; the irreducible floor is open, measured by `Q-INFER`). The wrappable-leaf / optimized-vs-faithful seam is `kFIDELITY`. Grep the slug for current handling.

**Status: live constraint on the core tool, NOW.** (Companion: `deferred/078-privileged-tracing-tool.md` = the eventual tool that consumes this surface; that one is deferred. This one is not — these are things the core must *not optimize away* from day 1, because retrofitting them is the kind of lock-in the perf round warned about.)

**Framing (the user's):** all Dorc UX is shell. There is **no metadata layer.** A tracing tool's output is *a proposed `sh` guard* (`if [ -f /mnt/blah ]; then …`) or *a complaint* ("leaf X has an undeclared network dependency; add a guard"). So the hook surface below carries shell + provenance, never a metadata sidecar.

## The one core requirement: leaf execution is a wrappable seam, in both phases
The probe-runner and the apply-runner must execute each leaf command through an **indirection that an external process can intercept**, not inline everything into one opaque `sh -c "$bigscript"`. Concretely the seam must allow, per leaf:
1. **Command-prefix wrapping** — run the leaf as `<wrapper> -- <leaf>` where `<wrapper>` is pluggable (`fsatrace --`, `strace -f --`, a seccomp-installing shim, an eBPF-window marker, or nothing). The external tool supplies the wrapper; Dorc supplies the leaf + its identity.
2. **Environment augmentation** — inject env per leaf (`LD_PRELOAD=…`, a correlation token like `DORC_LEAF_ID=…` so a system-wide tracer can attribute syscalls back).
3. **Stable leaf identity / provenance** — every executed leaf carries an ID that maps deterministically back to its **source AST node** (the oracle + line). This already exists for the plan-as-shell provenance (note 70); it is **dual-use** — the trace tool needs the exact same map to turn "syscall S happened in leaf L" into "annotate oracle L's source with a guard." Build it once, both consumers use it.

This seam is **process-level, not in-process** (not a library/FFI API): the things being traced are arbitrary external programs (`docker`, `apt`, a vendor installer), and the tracer is a separate privileged binary. The contract is "spawn this leaf in a way I can wrap + name," nothing richer.

Both runners (probe AND apply) expose the **same** seam. The apply phase is the one where the user said capable-fleet runtime tracing might live; the probe phase is where the cheap in-tool backstop (below) lives. Neither may be special-cased into un-wrappable execution.

## The optimization hazard (the "don't assume/optimize-away the pluggable bit")
The probe optimizer (hoist-and-batch cheap checks; keep-under-guard expensive ones; drop un-provably-inert leaves — note 70) **destroys the 1:1 leaf↔execution mapping** that trace-attribution needs: a batched `dpkg -l` covering 40 checks, or an elided leaf, can't be attributed back to one source line. So:

- **Requirement: a `--faithful` execution mode** — one leaf, one execution, control flow preserved, no batching/hoisting/elision — that the trace tool runs against. This is the *un-optimized* probe; correctness-identical, just slow and attributable. The optimized probe is for production; the faithful probe is for the tracer (and for debugging — it doubles as the "show me exactly what runs, in order" mode the realtime-output/debuggability requirement wants anyway). **GUESS this is free dual-use, not extra scope.**
- **Even in optimized mode, preserve a recoverable mapping where cheap** (a batched check knows which source leaves it stands in for), so the cheap in-tool backstop (below) can still attribute. Don't collapse provenance irretrievably for a few ms.

## The easy-to-secure backstop that ships in the core tool (all users, both phases, unprivileged)
This is the "wide quality backstop — the whole community tests your oracle just by probing" the user asked for, and it's **separable from the privileged tool** because it uses a fundamentally securable primitive: **seccomp-BPF**.

- **Mechanism:** before exec'ing leaves, the runner self-installs a seccomp filter (unprivileged via `PR_SET_NO_NEW_PRIVS`; mainline since Linux 3.5) that **logs** (`SECCOMP_RET_LOG`, 4.14+) or notifies on `socket(AF_INET/AF_INET6, …)`. The domain is a **scalar register arg**, so classic seccomp can match it *without dereferencing pointers* — which is exactly why it's cheap and safe. AF_UNIX (the docker socket, local IPC) is **not** flagged; a real outbound `socket(AF_INET)` **is**.
- **What it buys, for everyone, free:** "⚠ leaf X opened a network socket" at probe- and apply-time → (a) the **network cost-class** signal for the optimizer (keep-under-guard, note 74) with zero annotation, and (b) an **undeclared-network-dependency backstop** surfaced to the deployer (lax audience) without them installing anything. Answers the user's Q3 directly.
- **Why it's "easy-to-secure":** seccomp can only **observe or deny**, never grant — it is *fail-closed by construction*, so shipping it fleet-wide carries none of the LD_PRELOAD/eBPF "compromise = privilege escalation" risk. It's the security category the privileged tool is *not* in.
- **Honest limits (state them, don't hide):** (1) **coarse** — "a network socket was opened," not which host/addr (no pointer deref); fine for cost-class + backstop, useless for rich deps. (2) **in-process-tree only** — catches the probe's own shell+coreutils, **NOT** daemon-mediated calls (dockerd's network pull is invisible; see `deferred/078-privileged-tracing-tool.md` for why that needs the privileged tool). (3) Linux-only (the target). (4) observe-don't-block by default (a check legitimately pinging the config server is reported, not killed; a strict mode could deny).

## Net constraints to bank now (the retrofit-hostile list for pluggability)
1. **Leaf execution = a process-level wrappable seam** (prefix + env + stable ID), in **both** probe and apply runners. Never an opaque single `sh -c`.
2. **Provenance/leaf-ID is dual-use** (plan UX + trace attribution) — one map, designed in.
3. **A `--faithful` un-optimized execution mode** exists alongside the optimized probe (the optimizer must not be the *only* path).
4. **The optimizer preserves a recoverable leaf-mapping** even when batching, where cheap.
5. **Ship the seccomp network-class backstop in core** (unprivileged, all users) — and structure the runner's pre-exec hook so it's the *same* hook the privileged tool later uses. Building the easy version correctly *establishes the seam* for the hard version.
6. **Everything crosses the seam as shell + provenance, never metadata.**
