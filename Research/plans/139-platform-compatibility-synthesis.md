# Round 13 — platform-compatibility: synthesis & conclusion

> The round-13 conclusion (interactive-research `conclusion.md`, sited in the `Research/plans/` convention).
> Companion to the charter `plans/130`; raw fronts in `notes/131`–`134`; sources in `../sources.json`.
> **Reconciled against the out-of-band rounds 10–12 (2026-06-04)** — see §6. Prose is AI-generated; the
> human-written repo-root docs (DESIGN/KNOBS/README/TODO) win over anything here.

## The answer, plainly
**Dorc does not need a pluggable language, and must not get one.** The fear that "supporting Windows means a
2-to-N-language backend" rests on conflating two things the round split apart:
1. **The orchestrator has no Windows problem at all.** sh-*analysis* is platform-free text; a Rust async core
   never touches the `fork()` wall that bars a native-Windows Ansible controller [A-davis-no-windows-controller-2020].
   Orchestrating *remote* targets from Windows needs only an ssh client (Windows has one) — no local sh.
2. **`kLANG` (sh-is-the-product) is the foundational weld**, because Dorc analyzes the *authored* language: the
   probe-compiler's derivable-truth ceiling is sh-shaped, so a second *input* language is a new product, not a
   backend. Windows/odd **targets** are therefore handled by a **precondition** ("the target already evaluates
   POSIX sh"), never an executor or sh→pwsh transpile of user code.

The target precondition is *lighter than the market leader's already-accepted contract*: Ansible requires every
managed node to have an interactive POSIX shell **and Python** [A-ansible-install-guide-2026]; Dorc requires only
the sh. So "be wide on targets" (`kTPLATFORMS-wide`) is tractable — gated on a *capability* (runs POSIX sh), with
all platform variance pushed into the oracle layer, exactly where the existing design already puts the long tail.

---

## 1. The pluggable-language question — settled (`kLANG`, welded)
- +SURE the orchestrator is platform-free: parse → CFG → effect-analysis → plan is text processing; identical on
  any OS/arch. The one impl constraint: the probe/apply transform stays in-process, not shelled to a local sh.
- +SURE Dorc dodges Ansible's #1 blocker for free — the `fork()`-per-task worker model ("Windows doesn't have
  fork()") [A-davis-no-windows-controller-2020]; pyinfra already runs its controller on Windows via gevent
  [A-pyinfra-compatibility-2024].
- +SURE a non-sh *input* language is a second product (new analyzer/parser/oracle-idiom; shared remnant = the name
  + the thin pluggable orchestrator) and is not separable from the engine — the human's correction, now `kLANG`.
- +SURE prior art's "separate Windows path" (Ansible `win_*`, `pyinfra-windows`, Salt minion) is the **executor**
  pattern, which Dorc's welds (`kAGENTLESS` + what-you-type-is-what-runs + `kLANG`) *reject* for user code
  [A-ansible-windows-guide-2026][B-salt-supported-os-2026][A-pyinfra-compatibility-2024].

## 2. Windows in the ops universe (the framing the human asked about)
- ~SUSPECT Windows servers are a real but enterprise-concentrated minority: Linux 61% of known-OS web servers vs
  Windows 9–23% by tier [B-w3techs-linux-windows-2026]; near-absent from the homelab/hobbyist world Dorc names first.
- +SURE serious *nix-fleet ops are *not* driven from Windows: Ansible forbids a native-Windows control node
  [A-ansible-windows-guide-2026][A-ansible-install-guide-2026], Salt has no master on Windows or macOS
  [B-salt-supported-os-2026]. "From Windows" in practice = **WSL** (Linux in all but name) or
  **Windows-managing-Windows** (PowerShell DSC / SCCM / agents) — a different ecosystem Dorc isn't in.
- ⇒ Dorc's only real Windows-orchestrator case is the **Windows-daily-driver homelabber** pushing to a *nix
  fleet — who already has/can-get WSL. Hence `kWINLOCAL` low-priority, mild-lean `-nix-only-controller`.

## 3. The target-platform contract (`kTPLATFORMS`, lean wide — the tier split from F-PRECOND)
- +SURE the precondition is well-precedented and *strictly lighter* than Ansible's (SSH + POSIX sh, no Python)
  [A-ansible-install-guide-2026]; "interpreter-as-prerequisite, tool-bootstraps-it" is canonical [A-ansible-bsd-guide-2026][A-ansible-raw-module-2026].
- ~SUSPECT the honest boundary (**corrected by F-BOOTSTRAP**, `notes/135` / `plans/deferred/13A`): a sh-less
  Windows box **can be mechanized** into sh-capable by a fixed `raw`-equivalent — scp/`curl.exe` a static
  busybox.exe (scp needs no target shell; curl.exe is bundled), then invoke by path. Emitting that *one fixed*
  native-shell command is plumbing, not an authored language, so it does **not** breach `kLANG` (exactly as
  Ansible's `raw` bootstraps Python). The irreducible floor is "Dorc emits ≤1 fixed, inspectable native-shell
  command," not "a human must do it." Robust pattern = **scp-then-invoke-by-path** (the `sh -s` stdin-pipe is
  Win32-OpenSSH's buggy zone). True boundary: a target that cannot evaluate *any* POSIX sh is **unsupported**.
- +SURE "wide" splits into two tiers (the analyzer/`kLANG` is unaffected by either; variance is oracle-layer):
  - **tier-A — real POSIX env** (Linux/macOS/BSD/**WSL**/containers): full oracle support.
  - **tier-B — sh-syntax-only env** (busybox-w32; git-bash/cygwin partial): control-flow analysis works, but
    permission/ownership/path/`/dev` oracles **degrade or are unsupportable** — busybox-w32 "permissions are
    totally bogus," `/path`≠absolute, `/etc`→SYSTEMDRIVE, `/dev/*` only as shell redirections
    [A-busybox-w32-readme-2024][A-frippery-busybox-paths-2021]. Practical rule: real POSIX Windows target ⇒ WSL.

## 4. Orchestrator host & transport (`kWINLOCAL`, ssh, own-vs-farm-out)
- +SURE ssh *client* is first-party on Windows (Server 2019/Win10-1809) [A-ms-openssh-server-config-2025] → a
  Windows controller reaching *nix targets is fine.
- ~SUSPECT the one real gap: native Win32-OpenSSH has **no `ControlMaster` multiplexing** (issue #1328, backlog
  since 2019, still unimplemented 2026) [A-win32openssh-controlmaster-1328-2019]; the WinRM→PSRP saga independently
  shows per-task connect+auth is catastrophic and pooling is mandatory [B-sullivan-winrm-psrp-2026]. ⇒ if Dorc
  "owns the orchestrator," owning it should include **in-process connection pooling (Rust-native SSH, e.g. russh)**
  rather than leaning on OpenSSH ControlMaster. (Cross-ref §6: this *is* round-12's open transport item.)
- +SURE "everyone has gitbash" is defensible with one named footgun — MSYS path-mangling at the native↔unix seam
  [B-msys2-filesystem-paths-2024]; spares the sh-shipped-to-remote case.

## 5. Authoring hazard — CRLF (F-CRLF)
- +SURE a Windows-authored `.dorc.sh` (CRLF) breaks on a *nix target *below the shell*: shebang-`\r` is a kernel
  exec failure, un-guardable from within sh [C-unix-se-crlf-shebang-2016]; `\r` also silently corrupts
  comparisons/heredocs/`read`. Fixes are repo-layer (`*.sh text eol=lf` [A-git-gitattributes-2024]) and tool-layer
  (`dos2unix`). ⇒ Dorc controls the wire, so **normalize-to-LF on ship OR detect-and-fail-clear**; detection is
  free (the analyzer already parses the AST). It's the `kWINLOCAL` persona's most likely first breakage. The
  policy choice (silent-normalize vs warn/refuse vs `kFIDELITY-faithful` ships-bytes-verbatim) is a conscious one.

## 6. Cross-round reconciliation (the out-of-band sync — rounds 10–12, 2026-06-04)
- **`kCOMMS` is a false friend of my `kLANG`/`kOOB` vocabulary — DISAMBIGUATED.** `kCOMMS` (round 11;
  `-executor-OOB ↔ -transpilation-inband`) is the in-/out-of-band axis for **Dorc's own controller↔host
  *metadata* plumbing**, *not* the user-authored language. Consequence: **I must not over-claim "nothing runs on
  the target."** `kCOMMS-executor-OOB` — a transient bootstrapped probe-executor that phones home (Ansible-python
  style) — is a *live* option and is `kAGENTLESS`-compatible (no persistent daemon). My §1/§3 claims constrain only
  the **user's** authored language (sh); they say nothing against Dorc shipping its own thin probe-executor. (If
  that executor is a compiled Dorc binary rather than sh, it re-enters `kTPLATFORMS` as a per-arch build concern —
  flag, not resolved here.)
- **`kAGENTLESS` (round 10, welded `-push`) aligns with my usage.** Its security framing cross-cuts mine:
  control-node = whole-fleet blast radius (`plans/102`) reinforces `kWINLOCAL` (the orchestrator is the crown
  jewel); "Dorc is a package manager → supply-chain → *no registry* + defensive-lint" and **version-drift** (same
  version-string ≠ same bytes) bear on the **oracle-distribution** half of `kTPLATFORMS-wide` (community oracles
  per tier/platform). (`plans/101`/`102`.)
- **Round 12 (DST/testing) already owns two of my weaker fronts.** Its `axis-platform` finding — madsim/libc-override
  determinism is OS-specific, "CI failures unreproducible on Macs" (`notes/123` f19/f23) — *is* the Rust
  cross-platform dev-env tension → **F-RUSTCI dropped** (cross-ref `notes/123`; `racum` [B-racum-rust-cli-releases-2026]
  covers the build matrix). Its OPEN transport item ("SSH-a-script-over vs agentless-executor-phones-home; own
  vs build-on-pyinfra," tied to the triple-use `dorc_exec` seam, `notes/124`/`127`) *is* my §4 transport question +
  `kCOMMS` → **F-SSHPOOL folded** here, lib choice deferred.

---

## Competing options & why-not (the frontier, for later re-weighing)
- **Pluggable dorc-on-sh ↔ dorc-on-pwsh core** — *rejected/welded out* (`kLANG`): a second input language is a new
  product and is inseparable from the probe-compiler. (My original round-13 framing of this as a "deferrable
  backend" was wrong; corrected at the gate.)
- **Windows-as-target via an executor/transpiler of *user* code** — *rejected*: it's the prior-art executor pattern,
  against `kAGENTLESS` + what-you-type-is-what-runs. The surviving path is the sh-precondition (§3).
- **Windows-orchestrator support level** — frontier: (i) WSL-only [≈ Ansible's stance, the lean] · (ii) native .exe
  orchestrating *remote* *nix targets, no local sh [tractable, modest] · (iii) native incl. localhost-as-Windows-target
  [hard: needs local sh ⇒ gitbash/WSL + path footguns]. `kWINLOCAL` leans (i)/(ii), low priority.
- **Transport ownership** — own-the-pool (Rust-native SSH) ↔ farm-out to a pyinfra-alike. Not a named knob (DESIGN
  component-3 prose + round-12 open item); the native-Windows no-ControlMaster gap tips "own" toward in-process
  pooling. (`kTRANSPORT` was floated and **dropped** as redundant with this axis.)
- **CRLF policy** — silent-normalize ↔ warn/refuse ↔ faithful-verbatim; conscious choice, `kWINLOCAL`-facing.

## Knobs ledger (this round)
- **New, proposed & applied to KNOBS this round:** `kLANG` (welded → sh-is-the-product), `kTPLATFORMS`
  (mainstream ↔ wide; lean wide, gated by `kLANG`), `kWINLOCAL` (nix-only-controller ↔ windows-supported; lean
  nix-only, low). **`kTRANSPORT` floated then dropped.**
- **Added out-of-band (NOT mine; for awareness/merge):** `kCOMMS` (round 11), `kAGENTLESS` (round 10). My worktree's
  KNOBS does not contain these; a merge must union my 3 with their 2 (likely a Welded-section + Execution&modes
  conflict to hand-resolve). Flag for the human.

## Quarantine / triage list (interesting, correctly not chased)
- **busybox-w32 oracle-tier spike** — *which* oracle categories actually survive on tier-B (perms/ownership are
  out; existence/content/process likely in). A real go/no-go for "Windows-target-without-WSL" support breadth.
- **CRLF wire policy** — decide normalize-on-ship vs detect-and-fail-clear vs faithful default (§5); small but
  `kWINLOCAL`-facing; a TODO, not researched-to-conclusion.
- **F-SSHPOOL library choice** (russh / libssh2 / openssh-rs) — deferred; premature until "own the orchestrator"
  and the `kCOMMS`/transport pole are decided (coupled to round-12 `axis-dst-cost` + the `dorc_exec` seam).
- **`kCOMMS`-executor-as-compiled-binary** — if Dorc's probe-executor is a binary not sh, it needs per-arch builds
  (re-enters `kTPLATFORMS`); intersects round-12's transport-model open item. Unexamined here.
- **Round-15 adversarial premise-review (`notes/150-151`)** — not read; appears orthogonal to platform-compat, but
  if it challenges the sh-substrate premise it would bear on `kLANG`. Flag for the human to point me at it if relevant.
- **KNOBS merge** (my `kLANG`/`kTPLATFORMS`/`kWINLOCAL` + the out-of-band `kCOMMS`/`kAGENTLESS`) — a git-merge chore.

## Bottom line
The thing the human feared — rearchitecting the core into a pluggable language — is the one thing the round
*welds shut* (`kLANG`). Everything Windows/odd-platform that's actually wanted resolves *without* touching the sh
core: the orchestrator is platform-free text + ssh; the target is a **sh-precondition** (lighter than Ansible's),
splitting `kTPLATFORMS-wide` into full-support tier-A vs degraded tier-B; the homelab Windows-author is served by
`kWINLOCAL` + a CRLF-normalize-or-warn policy. Dorc's *own* plumbing may still put a transient executor on the
target (`kCOMMS`) — that's orthogonal to, and does not soften, the sh-only-for-user-code weld.
