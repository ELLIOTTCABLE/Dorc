# Round 13 (deferred) — busybox→Win32 bootstrap: a sketched, non-parity Windows-target path

> **Status: SKETCH · prospective · DEFERRED.** A *very* rough prospective plan, stamped at the human's request as
> the close of round 13's F-BOOTSTRAP front. NOT a commitment and NOT designed — it exists so the path isn't
> re-discovered. Evidence + grades: `notes/135` (+ `notes/133`/`134`). Corrects the overstated boundary in
> `plans/139 §3`. Strawman commands are illustrative, **untested on metal**. Human-written repo docs win.

## Goal (deliberately minimal — read this twice)
Take a bare box running only **Win32-OpenSSH** (default shell cmd.exe) and leave it able to **receive ssh and run
*any* sh-shaped script at all**. That is the whole target. **Explicitly deferred / out of scope:** platform-parity,
POSIX-env semantics (busybox-w32's perms/paths are "bogus" — tier-B, `notes/133`), and oracles with *nix-isms
baked in. We are buying *"sh runs"*, nothing more.

## End-state
`busybox-w32` (a single static `ash.exe`, i686/x86_64/aarch64 [A-busybox-w32-readme-2024]) present on the box;
Dorc invokes it per-leaf **by path**. No persistent Dorc daemon — a shell binary is not an agent (`kAGENTLESS`
intact), exactly as Python-for-Ansible isn't one.

## The bootstrap = the `raw`-equivalent (one fixed, inspectable, Dorc-supplied step)
The chicken-and-egg ("the first command lands in cmd.exe, and Dorc speaks sh not cmd") is dissolved the way
Ansible dissolves it for Python: a **fixed** native-shell command, not an authored language [A-ansible-raw-module-2026].
Two interchangeable ways to land the binary, both robust:

```sh
# (a) push the binary — needs NO target shell (Win32-OpenSSH bundles sftp/scp; works even if DefaultShell≠cmd):
scp busybox.exe  user@winbox:C:/ProgramData/dorc/busybox.exe

# (b) …or one fixed fetch in the box's native cmd (curl.exe ships in every Win10/11 since 1803 [B-curl-microsoft-windows-2025]):
ssh user@winbox 'curl.exe -fLo C:\ProgramData\dorc\busybox.exe https://frippery.org/files/busybox/busybox.exe'
#   (PowerShell alias trap: it must be `curl.exe`, not `curl`.)
```

Optional: point sshd's default shell at it for convenience —
`HKLM\SOFTWARE\OpenSSH\DefaultShell = …\busybox.exe` [A-ms-openssh-server-config-2025] — but **not required** (we
invoke by path, below). Plan/apply UX can print this exact bootstrap command for approval before it runs.

## Per-leaf execution = the Windows variant of the `dorc_exec`/`kFIDELITY` seam
The naive idiom `ssh host 'sh -s' <script` (DESIGN.md's *nix off-ramp) **does not transfer to Windows** — piping a
script to the default-shell session fails outright and stdin-piping is Win32-OpenSSH's buggiest zone, *still* in
2024 [A-win32openssh-pipe-1545-2020]. The robust Windows wrapper is **scp-then-invoke-by-path**:

```sh
# 1. ship the (LF-normalized!) job as a file:
scp ./job.sh  user@winbox:C:/ProgramData/dorc/job.sh
# 2. invoke by path — command-arg form (no stdin-pipe), -T (no PTY → no LF↔CRLF mangling), never -t:
ssh -T user@winbox 'C:\ProgramData\dorc\busybox.exe sh C:\ProgramData\dorc\job.sh'
```

Two CRLF hazards stack and both must be handled: the **file's** bytes (F-CRLF, `notes/134` → normalize-to-LF on
ship) and the **transport's** PTY (`-T`/no-PTY [C-unix-se-ssh-pty-crlf-2021]).

## Why it stays inside every weld
- `kLANG` (sh-is-the-product): the user's authored ops stay sh; the fetch + the `busybox sh` wrapper are fixed
  Dorc plumbing, *not* an authored/analyzed language. (This is the correction — emitting a fixed native-shell
  byte ≠ adopting pwsh as a language.)
- what-you-type-is-what-is-run: `job.sh` runs verbatim (modulo the benign LF normalization); the wrapper is transport.
- `kAGENTLESS-push`: nothing persistent installed; a shell, not a daemon.
- `kCOMMS`: this is the controller→host *plumbing* axis. The sketch uses the in-band-real-shell pole; it does
  **not** preclude the `kCOMMS-executor-OOB` pole later (that's the open knob, not this sketch's call).
- `kTPLATFORMS-wide`: this is what makes "wide" self-onboarding rather than manual-prep — strengthens it.

## Open risks / untested-on-metal (what stops this from being more than a sketch)
- ~SUSPECT exit-status propagation `busybox → cmd → ssh channel` (and the `-t`-breaks-ERRORLEVEL trap, #1737) —
  Dorc *needs* leaf exit codes for guard/probe semantics; unverified.
- ~SUSPECT busybox-w32 ash running non-trivial real ops sh end-to-end (only its existence + env-limits are graded).
- the 2024 stdin-pipe hang is *avoided* by scp-then-invoke, but argues for never reintroducing the pipe form.
- DefaultShell-set vs invoke-by-path: tradeoff (convenience/interactive vs explicitness) unresolved.
- AV/Defender perf hit on busybox processes (README) — needs a process exclusion at scale.
- the whole strawman is unrun; first real step is a container/VM fixture (`kVERIFY` harness; cf. round-12 DST/`dorc_exec` seam).

## Explicitly NOT in this sketch (deferred)
Platform-parity; tier-B POSIX-semantic oracles (perms/ownership/paths); the compiled-Dorc-executor variant
(`kCOMMS-executor-OOB`, which would re-enter `kTPLATFORMS` as a per-arch build); macOS/BSD/ARM-Linux targets (those
are tier-A — sh already present, no bootstrap). No ControlMaster on Windows [A-win32openssh-controlmaster-1328-2019]
→ pooling stays an in-process (own-the-orchestrator) concern, out of this sketch.

## One-paragraph essence
A Windows target is onboarded by one fixed `raw`-equivalent (scp **or** `curl.exe`) that lands a static
`busybox.exe`, after which every leaf runs `ssh -T host 'busybox.exe sh <path>'` over an LF-clean, no-PTY,
no-`-t` channel. It buys "runs any sh," touches no weld, and is the natural — if gritty — generalization of
DESIGN.md's `dash -s` off-ramp to a shell-less platform.
