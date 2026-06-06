# 133 — F-PRECOND: the sh-precondition + busybox-w32 viability (round 13, 2026-06-03)

> Durable per-front notes for the platform-compatibility round (plan: `plans/130-platform-compatibility-research-plan.md`).
> interactive-research front **F-PRECOND** — does the "target must already speak POSIX sh" precondition hold up,
> and is "interpreter-as-prerequisite" sound prior art? Cast wide incl. the counter-thesis (busybox-w32 is too
> un-sh to count). Gather-and-grade only; the cross-front re-reasoning comes next. Five A-grade primaries registered.

## Findings (most-attended first)
- **+SURE Dorc's target precondition is a STRICT SUBSET of the market leader's.** Ansible's managed node "requires
  Python... [and] a user account that can connect through SSH... with an interactive POSIX shell"
  [A-ansible-install-guide-2026]. Dorc needs **SSH + POSIX sh, no Python** → strictly less. "The target must speak
  sh" is not a novel or heavy ask; it is a documented, weaker version of what Ansible already demands.
- **+SURE interpreter-as-prerequisite + bootstrap-the-interpreter is canonical prior art.** "Ansible is agentless...
  however, it requires Python on managed nodes. Only the raw module will operate without Python... use of Python is
  required to make full use of Ansible's features" [A-ansible-bsd-guide-2026]; `raw` "does not require python on the
  remote... passes the command directly into the connection string... also supported for Windows" [A-ansible-raw-module-2026].
  So "the interpreter is the user's prerequisite; the tool bootstraps it" is blessed, not a Dorc dodge.
- **~SUSPECT the Dorc-specific asymmetry (the honest boundary):** Ansible self-bootstraps Python via the target's
  *native* shell (`raw` → sh on Unix, cmd/pwsh on Windows). Dorc, being sh-the-product (`kLANG`), has no native
  non-sh shell to fall back on — to install sh onto a sh-less Windows box it would have to *speak cmd/pwsh*, which
  breaches `kLANG`. ⇒ the **first** sh on a Windows target is a pure out-of-band human/image step (bake it in, or
  install git-bash/WSL/busybox-w32 by hand), slightly less convenient than Ansible's in-band `raw`-bootstrap but
  consistent with every weld. [A-ansible-raw-module-2026][A-ansible-bsd-guide-2026]
  - └ corrected in `notes/135` (F-BOOTSTRAP, 2026-06-04): the "speak cmd/pwsh ⇒ `kLANG` breach" reasoning was
    WRONG and "pure human step" overstated. Emitting *one fixed* native-shell fetch (scp **or** `curl.exe` a static
    busybox.exe, then invoke by path) is plumbing, not an authored language — the literal `raw` analog — so it does
    NOT breach `kLANG`. A mechanized bootstrap is weld-consistent (sketch: `plans/deferred/13A`); robust pattern =
    scp-then-invoke-by-path, NOT the `sh -s` stdin-pipe (Win32-OpenSSH's buggy zone).
- **+SURE busybox-w32 is the thinnest sh, and the counter-thesis has teeth.** A single static ash.exe (i686/x86_64/
  **aarch64**) that runs sh control-flow → satisfies `kLANG`. BUT it presents a *Windows* environment, not POSIX:
  "Handling of users, groups and permissions is totally bogus... synthesise uid, gid and permission values";
  divergent path model (`c:/`, `//host/share`, `c:path`, `/path`=relative-to-root; hardcoded `/etc`→SYSTEMDRIVE;
  `/bin/sh`→applet); `/dev/*` emulated only as shell redirections, "can't be used as arguments to other programs";
  `-X` backslash mode; no fork. [A-busybox-w32-readme-2024][A-frippery-busybox-paths-2021]
- **⇒ the sh-precondition splits `kTPLATFORMS-wide` into two target tiers** (the analyzer/probe-compiler `kLANG`
  is unaffected by either; variance is entirely in oracles, *as round-13 predicted* — but now with a sharper edge):
  - **tier-A real POSIX env** (Linux/macOS/BSD/**WSL**/containers): full oracle support.
  - **tier-B sh-syntax-only env** (busybox-w32; git-bash/cygwin partially): control-flow analysis works, but
    permission/ownership/path/`/dev` oracles **degrade or are outright unsupportable** (perms are bogus). Real
    POSIX Windows targets ⇒ use WSL; busybox-w32 is "sh runs, but don't trust POSIX semantics."
- minor (no quarantine needed): busybox-w32 prefers built-in applets over PATH externals unless `BB_OVERRIDE_APPLETS`
  is set — i.e. *which commands a tier-B target even has* is itself configurable/divergent, another oracle-tier wrinkle.

## Citations

> [A-ansible-install-guide-2026]:§Managed-node-requirements (relevance: +1:SURE)
> The managed node (the machine that Ansible is managing) does not require Ansible to be installed, but requires
> Python to run Ansible-generated Python code. The managed node also needs a user account that can connect through
> SSH to the node with an interactive POSIX shell.

> [A-ansible-bsd-guide-2026]:§Bootstrapping-BSD (relevance: +1:SURE)
> Ansible is agentless by default, however, it requires Python on managed nodes. Only the raw module will operate
> without Python. Although this module can be used to bootstrap Ansible and install Python on BSD variants... it is
> very limited and the use of Python is required to make full use of Ansible's features.
> ...  ansible -m raw -a "pkg install -y python" myfreebsdhost

> [A-ansible-raw-module-2026]:§Synopsis+Examples (relevance: +1:SURE)
> This module does not require python on the remote system, much like the ansible.builtin.script module.
> This module is also supported for Windows targets. ... [platform] requires no Python on the remote as it passes
> the command directly into the connection string.
> - name: Bootstrap a host without Python installed / ansible.builtin.raw: dnf install -y python3 python3-libdnf

> [A-busybox-w32-readme-2024]:§Hints (relevance: +1:SURE)
> Things may never work because of huge differences between Linux and Windows.
> Handling of users, groups and permissions is totally bogus. The system only admits to knowing about the current
> user and employs various heuristics to synthesise uid, gid and permission values.
> Emulations of several Unix-style device files are provided: /dev/null, /dev/tty, /dev/zero and /dev/urandom...
> They can't be used as arguments to other programs.

> [A-frippery-busybox-paths-2021]:§Pathnames (relevance: -0:SUSPECT)
> Thirdly, there's a path that has no root specifier and starts with a single slash (e.g. /path/to/file). This is
> relative to the current root. On Unix it would be an absolute path.
> Hardcoded references to Unix-style paths within busybox-w32 (e.g. /etc/profile) are treated as relative to the
> system drive... The shell detects attempts to run executables from paths traditionally used to store binaries on
> Unix (e.g. /bin/sh) and runs the corresponding applet, if it exists.
