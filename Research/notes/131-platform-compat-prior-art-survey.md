# 101 — platform-compat: prior-art survey (round 13, 2026-06-03)

> Durable gather-notes for the platform-compatibility round (plan: `plans/130-platform-compatibility-research-plan.md`).
> interactive-research turn 1 — the wide-net prior-art sweep (Ansible / Salt / pyinfra / Win32-OpenSSH /
> WinRM-PSRP / MSYS2 / Rust-CLI). Findings lifted to the top; verbatim citations below carry registered
> `sources.json` slugs + certainty. One finding amended by a forward-pointing correction (→ `notes/132`).
> Target: orchestrator-host platform-compat (primary) + target-host (secondary).

## Operational setup (so it isn't re-derived)
- `jq` is NOT on PATH on this box; `mise exec` global-config blocker (note 080) does NOT apply to
  per-tool provisioning. **Working wrapper for source registration:**
  `mise exec jq@latest -- sh <skill>/scripts/new-source.sh Research <slug> < entry.json`
  (verified: jq=mise, curl=/mingw64, sha256sum=/usr/bin, mktemp, date all resolve inside that env).
- `new-turn.sh`/`validate.sh` need only sh+coreutils (run direct); `new-source.sh`/`validate.sh` need jq (wrap).

## Findings (most-attended first)
- **The single biggest reason Ansible can't run on Windows — the `fork()`-per-task worker model — does NOT
  apply to Dorc.** A Rust async/threaded orchestrator sidesteps barrier #1 entirely. [A-davis-no-windows-controller-2020]
- **Prior-art consensus (3/3 agentless push-tools): the CONTROLLER is *nix-only / *nix-strongly-preferred.**
  Ansible forbids native-Windows control node (WSL/container only; WSL itself "not supported... not for
  production"); Salt has no master on Windows *or even macOS* (Linux-only); pyinfra is the lone exception —
  its controller runs on Windows because it uses gevent, not fork. [A-ansible-windows-guide-2026]
  [B-salt-supported-os-2026] [A-pyinfra-compatibility-2024]
- **The durable barrier is local/`localhost` execution parity, NOT remote orchestration.** Davis: "for the
  majority of tasks where Ansible is managing remote targets of any type, execution parity should be
  achievable." → For Dorc, orchestrating *remote* targets from Windows is the tractable case; `@local`/local
  sh execution is where Windows hurts. [A-davis-no-windows-controller-2020]
- **Windows-as-TARGET universally means a SEPARATE language/execution path (PowerShell), never faking *nix.**
  Ansible: Python core modules "will not work"; uses PowerShell `win_*` modules. Salt: native Windows minion.
  pyinfra: split-off `pyinfra-windows` package. This is prior-art's answer to the pluggable-language Q — but
  it's about TARGETS. Dorc's orchestrator-side sh *analysis* is pure text (platform-independent). [A-ansible-windows-guide-2026]
  - └ corrected at the gate (see `notes/132`): I wrongly extrapolated this to "a PowerShell *target-backend* is deferrable and
    doesn't touch the sh core." User (dead-sure): sh is the *analyzed/authored* language → a 2nd input language
    detonates the whole engine (probe-compiler ceiling is sh-shaped) = a new product, NOT a backend. The
    prior-art "separate path" is the *executor* pattern, which is AGAINST Dorc's welds (agentless +
    what-you-type-is-what-runs). Windows-target is therefore out-of-scope OR sh-precondition-gated, never a
    transpile/foreign-input. New foundational knob `kLANG`. See `notes/132` + `plans/130-…-research-plan` (rewritten).
- **"Everyone has gitbash" has a concrete, named footgun: MSYS path-mangling.** Native-exe args/env that look
  like Unix paths get auto-converted at the boundary (`--dir=/foo` → `--dir=C:/msys64/foo`; path-list `:`→`;`).
  Bites Dorc.exe↔gitbash-sh arg passing; mostly spares sh-shipped-to-remote. [B-msys2-filesystem-paths-2024]
- **Q2: native Win32-OpenSSH has NO ControlMaster connection multiplexing** (issue #1328 open/backlog since
  Jan 2019, no PR; neovim #39637 confirms still-unsupported May 2026). The OS-level connection-reuse primitive
  Ansible/Mitogen/Deployer rely on is absent on native Windows → Dorc must pool connections in-process.
  [A-win32openssh-controlmaster-1328-2019]
- **Cross-substrate connection lesson (WinRM/PSRP):** per-task connect+auth is catastrophic at scale (pywinrm
  new-shell+NTLM per task → AD-lockout "forkbomb"); PSRP fixes via pooling/persistent Runspace Pool. Same
  conclusion SSH reached with ControlMaster — and since ControlMaster is unavailable on Windows, the pooling
  must live in Dorc. [B-sullivan-winrm-psrp-2026]
- **Q2: ssh on Windows is first-party** (bundled since Server 2019 / Win10-1809); ssh *client* is fine. But
  default sshd shell = cmd.exe, and Win32-OpenSSH is a SUBSET (no AcceptEnv/X11/PermitTunnel/StreamLocal;
  auth only password/publickey). [A-ms-openssh-server-config-2025]
- **Q3 macOS:** target everywhere; controller OK for Ansible/pyinfra (Unix) but NOT Salt master (Linux-only).
  **ARM Linux:** first-class as both controller & target (Salt full master+minion aarch64/arm64). ARM-as-target
  is a non-issue for sh (arch-agnostic). [B-salt-supported-os-2026] [A-pyinfra-compatibility-2024]
- **Q4 Rust dev-env (least critical):** standard matrix — Windows msvc(rec)/gnu; Linux musl-static(Alpine-safe)/
  glibc; macOS universal-binary(lipo); x86+ARM all three; RISC-V add-later; cargo-dist/GH-Actions automate.
  [B-racum-rust-cli-releases-2026]
- **Market framing:** servers Linux-dominant (61% web; Windows 9-23% enterprise-concentrated); dev workstations
  Windows-plurality → orchestrator-on-Windows is a real homelab/dotfiles want, but serious *nix-fleet ops drive
  from Linux/macOS/WSL. [B-w3techs-linux-windows-2026]
- −GUESS gaps to firm next: busybox/Synology embedded-target sh portability (lightly evidenced); a Puppet/Chef
  "what we got wrong on Windows" implementor retrospective (have Davis + Sullivan; agent-model story is clear
  from docs but no first-person retro yet); SO-survey primary dev-OS numbers (have W3Techs servers-primary).

## Citations

> [A-ansible-windows-guide-2026]:§Using-Windows-as-the-control-node (relevance: +1:SURE)
> Ansible cannot run on Windows as the control node due to API limitations on the platform. However, you can
> run Ansible on Windows using the Windows Subsystem for Linux (WSL) or in a container.
> [Note] The Windows Subsystem for Linux is not supported by Ansible and should not be used for production systems.

> [A-ansible-windows-guide-2026]:§Which-modules-are-available (relevance: +1:SURE)
> The majority of the core Ansible modules are written for a combination of Unix-like machines... As these
> modules are written in Python and use APIs not present on Windows they will not work. There are dedicated
> Windows modules that are written in PowerShell and are meant to be run on Windows hosts.

> [A-davis-no-windows-controller-2020]:§TL;DR + §Worker-Process-Model (relevance: +1:SURE)
> There are a lot of UNIX-isms deeply baked into most of Ansible that prevent it from working on native Windows
> at all, and even if we solved every one of them, the likelihood of real-world playbooks executing with 100%
> fidelity between a *nix controller and a Windows controller is almost zero. If you want to run an Ansible
> controller on Windows anytime soon, use WSL.
> ...Ansible's controller worker model... makes heavy use of the POSIX fork() syscall... So what's the problem?
> Windows doesn't have fork(). This means that the entire worker execution subsystem... is 100% non-functional
> on Windows as currently implemented.

> [A-davis-no-windows-controller-2020]:§Content-Execution-Parity (relevance: +1:SURE)
> Remember: this is only about the behavior of localhost and the local connection plugin- for the majority of
> tasks where Ansible is managing remote targets of any type, execution parity should be achievable.

> [A-pyinfra-compatibility-2024]:§Control-Systems + §Remote-Systems (relevance: +1:SURE)
> pyinfra works on anywhere that runs Python - Mac, Linux & Windows are all supported.
> ...the only requirement on the remote side is shell access. POSIX commands are used where possible...
> [Upgrade 2.x->3.x] Remove @winrm connector, will come back as pyinfra-windows

> [B-salt-supported-os-2026]:§Overview-of-supported-operating-systems (relevance: +1:SURE)
> [Table] Windows Desktop 10/11, Windows 2016/2019/2022/2025: Minion = Yes (Full); Master column blank.
> macOS 13/14/15: Minion = Yes (Full); Master blank. Debian/Ubuntu/RHEL-family arm64/aarch64: Master Yes + Minion Yes.
> "There are no plans for the foreseeable future to develop a Salt Master on Windows." (Salt Windows install doc)

> [A-ms-openssh-server-config-2025]:§Configuring-the-default-shell (relevance: +1:SURE)
> OpenSSH... was added to Windows Server and Windows client operating systems starting with Windows Server 2019
> and Windows 10 (build 1809).
> The default command shell provides the experience a user sees when connecting... The initial default in Windows
> is the Windows command prompt (cmd.exe).
> [§Configuration-arguments] not available: AcceptEnv, X11Forwarding, PermitTunnel, StreamLocalBind*, AuthorizedKeysCommand, PermitUserEnvironment...

> [A-win32openssh-controlmaster-1328-2019]:#1328 (relevance: +1:SURE)
> Support for Control Master #1328 — Open — Labels: 0 - Backlog — opened Jan 23, 2019 (dwatley):
> "I noticed in the previous roadmap the Control Master feature was not going to be ready for the initial
> release. Any plans to support this in the future? We have use cases for multiplexing ssh connections."
> (No assignee, no milestone, no PR as of 2026; neovim#39637 (2026-05) "ControlMaster ... not supported by Win32-OpenSSH".)

> [B-sullivan-winrm-psrp-2026]:§Why-pywinrm-makes-this-a-forkbomb (relevance: +1:SURE)
> pywinrm creates a new WinRM shell with a fresh NTLM authentication for every single Ansible task. No connection
> pooling. No session reuse. No buffering or piping. ...4 × 5 × 15 = 300 [shell attempts]... each of those is a
> real NTLM auth failure against Active Directory.
> [PSRP] One authenticated connection per fork, multiplexing all commands over a persistent PowerShell Runspace Pool.

> [B-msys2-filesystem-paths-2024]:§Automatic-Unix-to-Windows-Path-Conversion (relevance: -0:SUSPECT)
> When calling native executables from the context of Cygwin, then all the arguments that look like Unix paths
> will get auto converted to Windows... python3 ... --dir=/foo  ->  ['--dir=C:/msys64/foo']
> ...--dir=/foo:/bla  ->  ['--dir=C:\\msys64\\foo;C:\\msys64\\bla']
> The only solution here is to avoid mixing Unix/Cygwin and native tools... or convert them when they get passed.

> [B-racum-rust-cli-releases-2026]:§Planning-the-builds (relevance: -0:SUSPECT)
> Windows... msvc vs gnu. Most sources recommend msvc... Pick msvc unless you have a strong reason against it.
> [Linux] I suggest starting with musl: it yields a static binary that can run everywhere, including on Alpine.
> [macOS] ship a "Universal Binary"... build for both targets and combine them using the lipo tool.
> Keep an eye on RISC-V: Linux already supports it, so if it gains traction you can add a target later.

> [B-w3techs-linux-windows-2026]:§Usage-broken-down-by-ranking (relevance: -1:GUESS)
> Linux is used by 61.3% of all the websites whose operating system we know. [Windows 8.6% overall;]
> Windows by ranking: top-1M 20.1%, top-100k 22.9%, top-10k 20.6%, top-1k 16.2%. (W3Techs, 4 June 2026)
