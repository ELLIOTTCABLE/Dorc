# 135 — F-BOOTSTRAP: busybox→Win32 bootstrap mechanics (round 13, 2026-06-04)

> Durable per-front notes for the platform-compatibility round (plan: `plans/130`; conclusion `plans/139`).
> interactive-research front **F-BOOTSTRAP** — verifying the three mechanics I asserted in `plans/139` but never
> tested, and correcting the overstated "first-sh = human-only" boundary. Post-conclusion addendum; sketch lands
> in `plans/deferred/13A`. Gather-and-grade; mechanics that remain untested-on-metal are flagged ~SUSPECT.

## Findings (most-attended first)
- **CORRECTION (the load-bearing flip): the "first sh is a pure out-of-band human/image step" boundary
  (`notes/133`, `plans/139 §3`) was OVERSTATED.** A *mechanized* `raw`-equivalent bootstrap is weld-consistent:
  emitting one fixed, Dorc-supplied native-shell command (fetch a static busybox.exe) is plumbing, not an authored
  language — exactly as Ansible's `raw` bootstraps Python [A-ansible-raw-module-2026]. The irreducible floor is
  "Dorc emits ≤1 fixed, inspectable native-shell command," not "a human must do it."
- **+SURE the naive form is the trap: do NOT pipe `sh -s` to a Windows target.** Piping a script to the
  default-shell session fails outright ("shell request failed on channel 0"); naming the interpreter as an arg
  works [A-win32openssh-pipe-1545-2020]. Compounded by historical binary-stdin corruption (#658, fixed 2017) and a
  *current* (OpenSSH_for_Windows 9.5p1, 2024) stdin-pipe hang/stdout-suppression report. ⇒ the DESIGN.md
  `ssh host 'dash -s' <script` idiom is *nix-clean but does **not** transfer to Windows targets.
- **+SURE the robust pattern is scp-then-invoke-by-path.** Land the job as a file (scp/sftp is bundled and works
  even with a non-cmd `DefaultShell`), then `ssh winbox 'C:\dorc\busybox.exe sh C:\dorc\job.sh'` — command-arg
  form, no stdin-pipe, no PTY. [A-win32openssh-pipe-1545-2020][A-ms-openssh-server-config-2025]
- **+SURE transport-layer CRLF: a PTY translates LF→CRLF and corrupts the stream** [C-unix-se-ssh-pty-crlf-2021];
  use `-T`/command-arg form (no PTY). This is *on top of* the script's own CRLF (F-CRLF, `notes/134`) — both must
  be LF. Two CRLF hazards stack: the file's bytes, and the transport's PTY.
- **~SUSPECT exit codes:** the `-t` flag breaks ERRORLEVEL propagation (Win32-OpenSSH #1737) → use exec/command
  form (no `-t`); whether busybox's exit status propagates cleanly cmd→busybox→channel is untested-on-metal.
- **+SURE fetch baseline:** `curl.exe` ships in every Win10/11 (since 1803) [B-curl-microsoft-windows-2025]; scp/
  sftp bundled with Win32-OpenSSH [A-ms-openssh-server-config-2025]. So *landing* the binary is robust two ways
  (scp = no target shell needed; curl.exe = one fixed cmd one-liner). PowerShell gotcha: `curl` is an alias →
  call `curl.exe`.
- **busybox-w32 end-state suffices for the minimal goal.** It runs ash (sh syntax) [A-busybox-w32-readme-2024]; the
  non-POSIX-env limits (perms/paths) are explicitly DEFERRED for this goal ("run any sh-shaped script at all," not
  platform-parity). Aarch64/x86_64/i686 builds exist. (README also flags an AV/Defender perf hit → exclude the proc.)

## Citations
> [A-win32openssh-pipe-1545-2020]:#1545 (relevance: +1:SURE)
> `echo "echo test" | ssh -T win64bot3` → "shell request failed on channel 0" … but `ssh -T win64bot3 "echo test"`
> (command as arg) works; `echo "echo test" | ssh win64bot3 cmd` works (interpreter named). [maintainer] "Fixed,
> this will be part of our next release." debug3: shell: "c:\\windows\\system32\\cmd.exe" / shell_option: /c.

> [C-unix-se-ssh-pty-crlf-2021]:§Answer (relevance: +1:SURE)
> when `ssh` is invoked with no command argument … a pseudo-terminal is by default allocated on the remote system
> … usually configured to translate line feed characters into carriage return + line feed sequences (LF → CRLF).
> Prevent via: `-T` on the client (or RequestTTY no); `no-pty` in authorized_keys; `PermitTTY no` in sshd_config.

> [B-curl-microsoft-windows-2025]:§Windows-10/11-bundle-curl (relevance: -0:SUSPECT)
> Every installation of Microsoft Windows 10 and Windows 11 has curl installed by default. [since insider build
> 17063] … A workaround is to invoke curl as "curl.exe" to prevent powershell from treating it as an alias.
