# Round 13 — platform-compatibility: problem-space map & research plan

*Status: GATE artifact (the interactive-research skill's `plan.md`, sited in the `Research/plans/` convention),
**revised at the gate** after the human's steer (the original Part A2 was wrong — see the correction box). Leads
with what the graded evidence settles, then maps the genuine variance + fronts. Per-front raw findings live in
`notes/131`–`102`; the round-13 synthesis will land in a later `plans/139-…`. Sources graded in `notes/131`;
steer in `notes/132` → `../sources.json`. Prose is AI-generated; trust the human-written repo-root docs over it.*

Target: **orchestrator-host platform-compat (primary)** + **target-host platform-compat (secondary)**.
The user is a *nix person intentionally on Windows, and is **dead-set against** turning the core into a
pluggable dorc-on-sh ↔ dorc-on-PowerShell split — which the gate steer elevated to a foundational weld (`kLANG`).

> **Gate correction (load-bearing):** the original A2 claimed a PowerShell *target-backend* was "deferrable,
> doesn't touch the sh core." Wrong. Dorc analyzes the *authored* language; the probe-compiler emits sh and its
> derivable-truth ceiling is sh-shaped, so a second *input* language is a *new product*, not a backend. And the
> prior-art "separate path" is the *executor* pattern, which is **against** Dorc's welds. Corrected throughout.

---

## Part A — what the evidence + welds settle (low variance)

### A0. The framing question: Windows in the ops universe
- **Windows servers?** Real but a *minority concentrated in enterprise* (.NET/IIS/AD/Exchange/MSSQL). W3Techs
  2026-06: Linux 61.3% vs Windows 8.6% of known-OS websites overall, rising to 16–23% in the top-1k…1M tiers
  [B-w3techs-linux-windows-2026]. Near-absent from the homelab/hobbyist/web-startup world Dorc names first.
- **Orchestrate *from* Windows?** Effectively never for *nix fleets. The three closest analogs make the
  controller *nix-only/strongly-preferred: Ansible forbids a native-Windows control node
  [A-ansible-windows-guide-2026]; Salt has no master on Windows *or even macOS* (Linux-only)
  [B-salt-supported-os-2026]; pyinfra is the lone controller-on-Windows case, only because it avoids fork()
  [A-pyinfra-compatibility-2024]. "From Windows" in practice = **WSL** (Linux in all but name) or
  **Windows-managing-Windows** (PowerShell Remoting/DSC, SCCM/Intune, Puppet/Chef agents) — a different ecosystem.
- **Net:** Windows-as-orchestrator matters for Dorc in one scenario — the Windows-daily-driver homelabber
  pushing to a *nix homelab/VPS/Pi — and that population already has/can-get WSL. → low-priority knob `kWINLOCAL`.

### A1. The orchestrator has no Windows problem (two independent reasons)
1. **sh-analysis is platform-free text.** Parse → CFG → effect-analysis → plan needs no local `sh`; a Rust
   binary does it byte-identically on any OS/arch. (Impl constraint: the probe/apply transform must be
   in-process, not shelled out to a local `sh` — DESIGN already implies this; flag, not a given.)
2. **A local `sh` is needed only to execute sh *locally* (`@local`/`dorc apply localhost`).** For *remote*
   targets the shipped sh runs in the *target's* shell. Davis's load-bearing line: "for the majority of tasks
   where Ansible is managing remote targets… execution parity should be achievable" — `localhost` is the killer
   [A-davis-no-windows-controller-2020]. ⇒ A Windows Dorc orchestrating a *nix fleet needs only an ssh client
   (Windows has one); no local sh.
3. **Dorc dodges Ansible's #1 blocker for free:** Ansible's controller is `fork()`-per-task and "Windows doesn't
   have fork()"; a Rust async orchestrator never forks; pyinfra (gevent) already runs on Windows
   [A-davis-no-windows-controller-2020][A-pyinfra-compatibility-2024].

### A2. Windows-as-TARGET — the corrected picture (`kLANG` weld + the sh-precondition escape)
- **Why it can't be "just a backend":** Dorc's value is analysis of the *authored* language. The probe-compiler
  emits sanitized sh; what a probe can determine about a host / the network / cross-host truth is shaped by sh's
  semantics. A second *input* language (PowerShell) ⇒ new analyzer, parser, oracle-contract idiom — a *new
  product* sharing only the name + the thin pluggable orchestrator. ⇒ **new foundational weld `kLANG`
  (sh-is-the-product ↔ pluggable-language)**, proposed welded to `kLANG-sh-is-the-product`. Verified NOT in
  KNOBS.md today.
- **Prior art is the *executor* pattern — and that's against Dorc's welds.** Ansible `win_*` PowerShell modules,
  `pyinfra-windows`, Salt's Windows minion all put *something on the target that speaks the target's language*
  [A-ansible-windows-guide-2026][A-pyinfra-compatibility-2024][B-salt-supported-os-2026]. Dorc is agentless +
  what-you-type-is-what-runs + `kLANG`-sh → it rejects executors *and* transpilation. So a "Windows-target
  backend" is not a deferral; it is either out-of-scope or precondition-gated.
- **The bootstrap dilemma the human posed** (ship the first `install-wsl` to a sh-less Windows box) had two
  hated horns: *speak-pwsh* (pwsh as input) or *transpile sh→pwsh* — both breach `kLANG`. **Third door — the
  sh-precondition pattern (Ansible⟷Python analogy):** make "a reachable POSIX-sh evaluator" a *target
  prerequisite*, exactly as Ansible makes Python a prereq (you don't `ansible`-install the first Python; `raw`
  runs pre-Python over the bare shell, the first Python is image-baked/manual). For Windows the precondition =
  git-bash / WSL / cygwin / **busybox-w32** (single static `ash.exe`), set once (+ sshd `DefaultShell`). No
  pwsh-input, no transpile; agentless + what-you-type-is-what-runs intact. (~SUSPECT — busybox-w32 fidelity and
  the raw-bootstrap analogy need a verification front; see F-PRECOND.)
- **Why the precondition doesn't breach `kLANG` and makes `kTPLATFORMS-wide` tractable:** the *authored language*
  stays sh everywhere (constant analysis ceiling); per-target variance lives in the *commands* = the oracle
  library (`Get-Service` vs `systemctl`) — the same community long-tail as every other platform. Target breadth
  is gated on a *capability* ("can evaluate POSIX sh"), not on Dorc learning N input languages. **Honest
  boundary:** a target that cannot run *any* POSIX sh is unsupported.

### A3. ssh substrate — minor gaps, all pointing the way Dorc already leans
- ssh *client* first-party since Server 2019/Win10-1809 [A-ms-openssh-server-config-2025] → Windows→*nix is fine.
- Native Win32-OpenSSH has **no `ControlMaster` multiplexing** (issue #1328, open/backlog since 2019, unimpl.
  2026) [A-win32openssh-controlmaster-1328-2019]; the WinRM→PSRP saga independently shows per-task connect+auth
  is catastrophic and pooling is mandatory [B-sullivan-winrm-psrp-2026]. ⇒ if Dorc "owns the orchestrator"
  (DESIGN component-3), owning it **may include Rust-native SSH (russh) with in-process pooling** rather than
  leaning on OpenSSH ControlMaster. (Human accepted this sub-point. NB: own-vs-farm-out is DESIGN.md prose, not
  a named knob; `kTRANSPORT` dropped as redundant.)
- If Windows is ever a *target* over ssh: default sshd shell = cmd.exe (needs HKLM `DefaultShell`), and
  Win32-OpenSSH is a subset (no AcceptEnv/X11/PermitTunnel/StreamLocal) [A-ms-openssh-server-config-2025].

### A4. "Everyone has gitbash" — defensible, one named footgun
MSYS path-mangling: native-exe args/env that look like Unix paths get auto-rewritten at the boundary
(`--dir=/foo`→`--dir=C:/msys64/foo`; list `:`→`;`) [B-msys2-filesystem-paths-2024]. Bites Dorc.exe↔gitbash-sh
arg exchange + `@local`; spares sh-shipped-to-remote. Fine as the homelab local-sh **iff** Dorc avoids passing
path-args across the native↔MSYS seam. Not a core-language problem.

---

## Part B — knobs (proposed; KNOBS.md is human-authoritative on naming → paste-ready entries in chat)
- **`kLANG` (NEW, foundational)** — `kLANG-sh-is-the-product ↔ kLANG-pluggable-language`. Propose **WELD** →
  sh-is-the-product. The day-one decision. About the *authored* language, not the target's *commands*.
- **`kTPLATFORMS` (NEW, med)** — `kTPLATFORMS-mainstream ↔ kTPLATFORMS-wide`. Lean **wide**, but gated by
  `kLANG`: "wide" = "any target that can already evaluate POSIX sh" (sh-precondition + per-platform oracles).
- **`kWINLOCAL` (NEW, low)** — `kWINLOCAL-nix-only-controller ↔ kWINLOCAL-windows-supported`. Mild-lean
  *nix-only; WSL is the escape hatch; analyzer is platform-free text so low lock-in.
- **`kTRANSPORT` — dropped** (subsumed by DESIGN's own-orchestration↔farm-out; that is prose, not yet a knob).

The live design question these encode: **`kTPLATFORMS-wide` × `kLANG`-sh = the sh-precondition contract** — how
thin can "the target speaks sh" be made, and how honestly is it surfaced (UX/docs), per target class.

---

## Part C — research fronts (gate: keep / kill / reprioritize)
- **F-PRECOND (NEW) — the sh-precondition path.** Verify the busybox-w32 single-binary-ash option (fidelity,
  path/quirk limits on native Windows) + the Ansible `raw`-bootstraps-Python pattern as the precise analogy +
  how Salt/Chocolatey bootstrap a sh-less Windows box. Grounds the `kLANG`×`kTPLATFORMS` contract. *Proposed:
  KEEP — now the highest-leverage front (it's the resolution's load-bearing assumption).*
- **F-CRLF — line-ending/shebang hazard.** Windows-authored `.dorc.sh` shipping `\r` to a *nix target breaks
  shebangs/`read`/heredocs. Concrete correctness item, under-mined. *Proposed: KEEP.*
- **F-SSHPOOL — Rust in-process SSH-pool landscape** (russh/libssh2/openssh-rs; how Ansible/pyinfra/Mitogen
  structure connection plugins). Grounds "own the orchestrator may include Rust-native SSH." *Proposed: KEEP-med.*
- **F-WINHOST — WSL/gitbash homelab reality** (adoption + how painful gitbash-as-sh is). Sizes `kWINLOCAL`.
  *Proposed: LOW (kWINLOCAL is low-priority).*
- **F-WINTARGET — Windows-target executor deep dive.** *Proposed: KILL (executor pattern is welded out).* 
- **F-RUSTCI — Rust cross-platform CI/build** (cargo-dist/ripgrep matrix). *Proposed: LOW (racum
  [B-racum-rust-cli-releases-2026] already covers the matrix).*

## Open questions for the human (slugged)
- **q-1 (kLANG):** Confirm the name `kLANG` (or rename) and that it's **welded** → sh-is-the-product? Welded
  section, or its own slot? (I read it as your day-one weld; confirming before I propose KNOBS placement.)
- **q-2 (target boundary):** Confirm the honest boundary "**a target that cannot evaluate POSIX sh is
  unsupported**," with the **sh-precondition** (git-bash/WSL/busybox-w32 set once) as the *only* Windows-target
  path — i.e. no executor, no transpile, ever?
- **q-3 (fronts):** Run order. My default: **F-PRECOND → F-CRLF → F-SSHPOOL**; kill F-WINTARGET; F-WINHOST/F-RUSTCI low.
- **q-4 (knobs):** Want me to hand you paste-ready `kLANG`/`kTPLATFORMS`/`kWINLOCAL` entries (done, in chat) —
  apply them yourself, or shall I apply to KNOBS.md on your say-so?
