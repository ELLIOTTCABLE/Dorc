# plan.md — immutable-fs-targets: the problem-space map after the first pass

> Written at the skill's after-first-pass gate (2026-09-03). This is the landscape, the
> options, the axes the decision turns on, what is still ambiguous, and my read — for the
> human to reframe, kill, or reprioritise BEFORE more effort is spent. Nothing here is
> ruled. Repo documents cited as `docID:slug`; graded sources bracketed by slug.

## The question, as sharpened this sitting

The trigger is `26K:sit-stdin-copy-exec-amendment`: the delivery shape of a shipped
artifact (today `ssh -T host 'sh -s; printf <marker>' < artifact`). The round's granule is
UNWRITABLE-FS; today's hunt is language holes a host oddity forces into the CORE LANGUAGE
rather than behind a flag, a negotiation, or a lint; two deliverables ride along — a first
division of capability granules, and the bounding box of odd-but-real hosts the siblings
serve poorly. DREP and return channels are in scope.

## Landscape (graded evidence; details and excerpts in turn01, corpus in turn02/turn03)

- Almost no real host is unwritable BY DESIGN. Every read-only-root system reached ships a
  writable tmpfs as a supported part of its configuration ([A-rhel8-readonly-root-rwtab-2026],
  [A-ubuntu-core-writable-paths-manpage-2014], [A-systemd-exec-protectsystem-privatetmp-2026],
  [A-haos-operating-system-architecture-2024], [B-openwrt-filesystems-techref-2026],
  [A-fedora-atomic-desktops-filesystem-2025], [B-flatcar-faq-immutable-usr-2026],
  [A-truenas-developer-mode-readonly-root-2026]). A separate class is fully writable but
  volatile ([A-raspi-config-overlayfs-source-2026], [B-alpine-diskless-mode-wiki-2026]).
  Nowhere-writable by design is ≈ hardened containers ([B-k8s-security-context-readonly-root-2026])
  and Talos, which has no shell ([A-talos-for-linux-admins-no-writable-fs-2026]).
- The real unwritable cell is FAILURE: root remounted read-only (a distro fstab convention,
  not the filesystem default — [A-e2fsprogs-ext2fs-errors-default-2026],
  [A-kernel-ext4-admin-guide-options-2026]), and ENOSPC where `/tmp` sits on the root fs
  (RHEL; Debian ≤ 12; Debian 13+ moved `/tmp` to tmpfs — [A-debian-trixie-release-notes-issues-2025]).
  The incumbent documents this exact need and closed it unfixed
  ([B-ansible-issue-25831-readonly-root-2017]); no sibling has a no-fs mode
  ([A-ansible-action-make-tmp-path-source-2026], [A-mitogen-ansible-tempfile-ladder-2026],
  [A-saltssh-roster-thin-dir-2026]).
- The one core-sh construct with a filesystem dependency is the here-document, and only
  under bash: dash and busybox never touch the fs ([A-dash-openhere-redir-source-2026],
  [ops-glue-residue:A-busybox-ash-source-2024]); bash < 5.1 always writes a temp file,
  bash ≥ 5.1 only above pipe capacity ([A-bash-53-release-tarball-2025]). Dorc's scaffolding
  emits none; only book/oracle bytes are exposed.
- Detection must be a real write: permission tests lie under ENOSPC/quota/protected_regular
  ([A-bash-53-release-tarball-2025] `file_iswdir`); `findmnt` default output is unstable
  ([A-findmnt-manpage-mount-options-2026]).
- Two sharp edges cut in the copy shape's favour: `noexec /tmp` blocks `execve` not `open`
  ([A-complianceascode-tmp-noexec-rule-2026]) so `sh "$f"` runs; `fs.protected_regular`
  targets world-writable sticky dirs ([A-linux-sysctl-fs-protected-regular-2026]) so a private
  `mkdir -m 700` dir is outside it.
- The corpus already reserved this niche for a deferred RAM-resident executor
  (`142:Resolution`), built the r26 wire with "zero new remote-fs assumptions"
  (`260:dec-26-wire-v1`), and keeps writable scratch opportunistic (the DREP lane degrades to
  `/dev/null`). `kBOOT` prescribes per-host, per-feature capability matching, never a
  tiered ladder. The load plane already distinguishes single-stream (`$0` = `sh`) from
  multipart (a materialized tree, `sh ./plan.sh`) — `30P:rul-dorc-invokes-in-a-modelled-live-spelling`.

## The options for the delivery shape

- **O1 keep the pipe + lint.** No fs need; the stdin hazard stays for bare `ssh` lines and
  `read`; no pre-execution integrity gate (a severed transport runs a prefix).
- **O2 copy-then-exec default.** `mkdir -m 700 /tmp/dorc.<nonce>` (exclusive, controller
  literal root) · `cat >"$d/a"` · verify completed (`cksum`/size) · `sh "$d/a"` · `rm -f`,
  `rmdir`. Interior readers see EOF; integrity gate; `$0` is a real path (multipart-
  consistent); `noexec`- and `protected_regular`-safe; residue on crash disclosed. Needs
  writable scratch at the literal root.
- **O3 in-memory slurp.** `sh -c 'a=$(cat); <verify>; eval "$a"'`-shaped. Interior readers see
  EOF; integrity gate possible (mind the stripped trailing newline); no fs; `$0` = `sh`
  (single-stream-consistent); whole-parse-before-run; `$LINENO` off; here-documents still
  fail under old bash-as-sh on a no-fs host, exactly as under the off-ramp.
- **O4 = O2 as default, O3 as the measured fallback** (the human's lean), selected host-side
  in ONE connection by the exclusive-create attempt, disclosed on the marker line
  (`mode=copy|slurp`). Adds no floor assumption (`kBOOT`'s triple suffices for O3).

## Axes the decision turns on

- `ax-selection-seat` — host-side self-selection on the remote command line (one exchange;
  consistent with `rul-repeated-probing-reviewed-before-design`) vs a controller-side
  capability handshake (a second exchange ⇒ the opaque-review posture). Lean: host-side.
- `ax-disclosure` — the marker line grows an additive `mode=` key vs a records-lane record
  vs silence. Lean: marker key (transport framing; additive-keys policy).
- `ax-fallback-value-loss` — what a no-fs fallback forfeits: DREP report records, the live
  topology, the multipart form. Which to recover? The DREP sink could become an fd
  (`/dev/fd/3` into a framer) — a cheap front, not yet gathered.
- `ax-heredoc-warning` — a plan-time warning keyed on (no writable scratch × artifact
  carries a here-document × remote `sh` is bash < 5.1). Narrow (macOS, RHEL 8 in a failed
  state) but free once delivery mode and remote-sh identity are measured facts.
- `ax-login-shell` — wrap the remote command line as `sh -c '<single-quoted posix>'` so the
  target's login shell parses one string. Behind interpreter specifics (human's priority),
  but the O4 command line is longer than today's and should be spelled deliberately.
- `ax-granules` — G2 splits STEADY / VOLATILE / FAILED (turn03); VOLATILE is a
  convergence-per-incarnation property wanting an aid hint, not a mechanism.
- `ax-fd0-close-lint` (human lean, 2026-09-03) — judo's close-fd-0 doctrine never as a core
  feature; as an opt-in strict-tier book lint ("no `exec </dev/null` boundary at the top;
  surprise readers will hang or eat input"). NB it is only SAFE to recommend once the
  delivery shape no longer reads the artifact from fd 0 — under today's `sh -s` an
  `exec </dev/null` at the top ends the script.

## What is ambiguous (evidence, not more reading)

- `amb-population-habit` — whether operators of read-only appliances want ssh-push at all.
  Unsourced after front-1. The cheapest instrument is the human (a homelabber).
- `amb-bounding-box` — front-2 is in flight (per-host sh / scratch / channel / login shell /
  `/dev/fd` / Python / sibling pain / population).
- `amb-fd-sink-portability` — `/dev/fd` presence (Linux `/proc`; FreeBSD `fdescfs`; busybox)
  for a no-fs DREP sink. Cheap front if wanted.
- `amb-login-shell-measure` — does today's `printf … "$?"` command line already misbehave
  under fish (`$status`) or csh? A one-line measurement; needs such a shell somewhere.
- `amb-installer-remount-ro` — unverified that the Debian installer writes it; low stakes.

## My read (a lean at the time; SUPERSEDED as direction by `Research/notes/26N` §2, ruled 2026-09-03)

O4 is well supported, and the evidence changed the WHY. The steady odd-host motive for a
no-fs fallback is weak — appliances give you a tmpfs — while the failed-host motive is
strong and sits in the incumbent's tracker as an unmet need. Nothing found forces the core
language: the only channel-forced analyzer carve (ssh's argv re-parse) already exists as a
decline-with-hint; the unwritable host's forcing of single-stream is a value-loss the
off-ramp shares; the here-document hazard is real, narrow, and aid-plane; stdin in shipped
bodies is already fenced. What falls out is a capability-vocabulary obligation (delivery
mode and remote-sh identity as measured facts), one spelling rule (`sh "$f"`, never
exec), and three lints. The mechanism is one string in the transport marker; the cost is
deciding selection and disclosure, and lint prose.

## Fronts proposed

- front-2 (in flight) — the odd-host bounding box.
- front-3 (proposed, cheap) — return channels without a writable fs: fd sinks, `/dev/fd`
  portability, busybox fd handling; r14 already holds the mechanism prior art.
- human-as-debugger — the population/habit question; the fish/csh measurement if a shell
  is handy.

## Post-gate addendum: front-2 folded (the text above is unchanged; turn03 has the detail)

- The shoe-in is not "sh-only" — pyinfra and cdist already claim that floor. What strands
  them on the archetypal weird host (OpenWrt) is the FILE LANE (no sftp-server under
  dropbear; pyinfra's file ops need SFTP/SCP + `setfacl` + `rsync`)
  ([A-pyinfra-ssh-connector-sftp-setfacl-2026], [B-openwrt-sftp-server-dropbear-2026]).
  NEW CONSTRAINT on every option above: the host-side write in O2/O4 must ride the same
  session's stdin (`cat >"$f"`), never a file-transfer subsystem, or the differentiator is
  spent. OpenWrt has tmpfs `/tmp`, so O2 works there.
- The second stranding axis is the login shell (Ansible's other precondition,
  [A-ansible-managed-node-requirements-2026]; deniers sourced: pfSense menu shell, Windows
  `cmd.exe`, CLI-fronted network OSes). CLI-fronted shells are reachable by an AUTHORED
  entry recipe (`27C` seat) — slap-on-top; menu shells and `cmd.exe` are per-host escapes.
  `ax-login-shell` gains one spelling constraint: no `!` in the wrapper (csh expands it
  inside single quotes) — FreeBSD root shells were csh before 14.0
  ([A-freebsd-14-root-shell-sh-2023]).
- G2-STEADY is ≈ containers only, corroborated from a second direction
  ([A-openbmc-flash-layout-squashfs-overlay-2026], [A-esxi-scratch-partition-ramdisk-2026],
  [A-unraid-root-ram-filesystem-2026]). The failed-host motive for O3 stands alone.
- The modal `/bin/sh` on the frontier is bash-as-sh / ash / mksh / FreeBSD sh, not dash
  ([A-apple-zsh-default-shell-varselect-2026], [A-nixos-environment-binsh-bash-default-2026],
  [A-aosp-shell-and-utilities-mksh-toybox-2026]). G3a is the common case; the cheap
  instrument is extending the `kwhichsh-gcd` real-binary battery to those shells. The
  heredoc hazard lands on macOS and NixOS.
- `/dev/fd` sinks are dead under context entry: Linux re-open of an inherited pipe by path is
  ownership-gated ([A-linux-proc-pid-fd-manpage-devfd-2026]); FreeBSD exposes 0–2 only
  ([A-freebsd-fdescfs-manpage-devfs-only-2023]). front-3's value drops; `ax-fallback-value-loss`
  likely resolves to "disclose, do not recover".
- Frontier short list (front-2): OpenWrt · HAOS · Windows · Android/adb · CLI-fronted network
  OS · BMCs · ESXi · Chrome OS. Dorc-reachable with zero host work: OpenWrt, BMCs, the HAOS
  container, ESXi if ash. Gaps to close by hand: ESXi's shell; Synology and QNAP entirely;
  the Ubiquiti page (archiver 403).

## Post-gate addendum 2: front-3 folded (the effort-invested metric; Talos-class; Q10/Q11)

- The human's metric SPLITS the frontier into sh-shaped and not-sh-shaped, and that split is
  the result. sh-shaped (an admin ships a script, the device runs it): Asuswrt-Merlin's
  `/jffs/scripts` hooks ([A-asuswrt-merlin-user-scripts-hooks-2026]), ghettoVCB's seventeen
  years of POSIX sh inside the ESXi shell ([A-ghettovcb-esxi-shell-script-2025]),
  Synology_HDD_db's 5,821-star bash script ([A-synology-hdd-db-shell-script-2026]), UniFi's
  `on_boot.d` runner ([B-unifi-common-on-boot-d-2026]), QNAP's autorun processor
  ([B-qnap-create-autorun-script-processor-2025]). Not-sh-shaped: RouterOS (a package
  manager reimplemented in RouterOS Script, PULL over HTTPS —
  [A-routeros-scripts-pull-distribution-2026]), network OS (netmiko: industrialised
  screen-scraping — [A-netmiko-screen-scraping-drivers-2026]), BMCs (Dell's own Redfish
  library, entirely off-box — [A-dell-idrac-redfish-scripting-offbox-2026]).
- EVERY sh-shaped artefact is a BOOT-HOOK re-invention, each stating the same reason: the
  change does not survive an update or reboot (QNAP resets config every startup —
  [B-qnap-autorun-flash-config-partition-2021]; Bottlerocket's root shell exists but dm-verity
  "will prevent most changes from persisting over a restart" —
  [A-bottlerocket-no-shell-sheltie-ladder-2026]). So on this frontier the scarce capability
  is `fs-durable`, not `fs-write` — the very keyword the charter defers. DIRECTION LEAN
  (conductor's): the frontier's natural Dorc shape is a HOST-RESIDENT book run locally at
  boot (the `dorc-run` shebang story; r31's local-exec lane) alongside push, not push alone;
  `fs-durable` stays deferred for the AMENDMENT (artifacts are per-run) but is central to the
  frontier's product story.
- The effort-set and the unwritable-set are near-DISJOINT: every sh-shaped artefact writes
  somewhere (`/jffs`, `/data`, `/tmp`, a flash config partition). Third independent
  reproduction that G2-STEADY ≈ containers.
- Gaps closed: ESXi's interpreter is busybox ash and its sshd is DROPBEAR (so no sftp-server,
  the OpenWrt trap again) ([A-ghettovcb-esxi-shell-script-2025]); Synology has bash and
  `sudo -s` to root, with `/bin/sh` ash on DSM < 6 and SRM ([B-synocommunity-dsm-srm-ash-binsh-2026]);
  QNAP has bash/curl/sudo, ash login shell, and a per-model flash config partition for
  persistence. VMware's own doctrine points OFF the box (`esxcli --config` fan-out from an
  admin server — [A-esxcli-in-scripts-offbox-admin-server-2026]).
- DIRECT HIT on the round's question: OpenWrt's idiomatic batch spelling is `uci batch << EOI`
  and `uci` reads stdin by default ([B-openwrt-uci-batch-stdin-heredoc-2026]). Under today's
  `sh -s`, a book's `uci batch` WITHOUT its heredoc would eat the artifact; after the
  amendment it reads EOF and silently applies NOTHING. That names a lint class the amendment
  creates: stdin-default commands with no redirect become silent no-ops (`uci batch`, `cat`,
  `patch`, `crontab -`), which is safer than eating the artifact but not safe.
- Talos-class is THREE tiers, not one: ssh-and-edit (Flatcar/FCOS: sshd, `core` user,
  writable `/etc`; immutability binds PROVISIONING only —
  [A-flatcar-customizing-sshd-day2-2026], [A-fedora-coreos-ignition-first-boot-only-2026]);
  shell-behind-a-ladder (Bottlerocket: control container → admin container → `sheltie` root
  shell); no-shell-at-all (Talos: a typed config-patch API, debug via a privileged container
  pushed over the API — [A-talos-machine-config-patching-2026],
  [A-talosctl-debug-privileged-container-2026]). Only the third tier is genuinely ours to
  concede; the first is ordinary G1.
- Q10 (is no-fs structurally reachable): sshd itself does not need a writable fs — privsep
  needs `/var/empty` to EXIST ([A-openssh-readme-privsep-var-empty-2026]) and `login_write()`
  discards every utmp/wtmp/lastlog failure ([A-openssh-loginrec-write-nonfatal-2026]). The
  unsourced half is everything else in a login (PAM session, home dir, rcfiles); Stack
  Exchange refused every fetch. The human's someday-test stands; sshd is not the blocker.
- Q11 (no-fs return channels), the sharpest constraint found: `sudo` closes every fd above 2
  by default, and `-C` is admin-gated ([A-sudo-closes-fds-above-stderr-2026]). Stacked on
  FreeBSD's `/dev/fd/0-2`-only, ANY fd-above-2 sink is dead under privilege change, so the
  only unaided survivor on a no-fs host is in-band framing on stdout/stderr — the cell the
  human named as forced-fallback-only. Framings to steal if it is ever forced: git's
  side-band (4-byte length + 1-byte stream code; error channel non-suppressible —
  [A-git-sideband-64k-stream-codes-2025]) and apt's `Status-Fd`, whose own example uses fd 2
  ([A-apt-status-fd-progress-protocol-2022]). `ax-fallback-value-loss` now resolves to
  "disclose, do not recover" absent a forcing reason. Unreached lead the front named: a
  reverse forward on the EXISTING ssh connection (`ssh -R`-style) as the cheapest
  no-new-tunnel return path — KISS-compatible, unsourced.
- HAOS's hostile channel is first-party: the SSH add-on "does not provide access to the
  underlying host file system"; day-2 is the `ha` verb CLI ([A-haos-common-tasks-ha-cli-2026]).
  pfSense's automation surface is a PHP REPL with record/playback, sh as the OUTER layer
  ([A-pfsense-php-shell-playback-2025]).
- Still open: HPE iLO's CLP shell (support.hpe.com empty bodies); the non-sshd half of a
  read-only login; population numbers (star counts measure script adoption, not installed
  base); printers/IoT as a counter-evidence desert (search-absence only, no source kept).

## Post-gate addendum 3: front-4 folded (prior-art transport/standup pivots; the "why nobody slurps" question)

- CORRECTION to this document's own earlier read: the memory route is NOT novel. The
  ARGUMENT form is what incumbents do — Salt base64-armors a ~7.5 KiB `/bin/sh` shim INTO
  the ssh command "so that it can all be passed in the SSH command and will not need
  special quoting" ([A-saltssh-shim-armored-in-ssh-command-2026]); Mitogen hands its
  compressed first stage to `python -c` as argv ([A-mitogen-boot-command-argv-payload-2026]);
  Ansible-on-Windows uses `-EncodedCommand` ([A-ansible-powershell-shell-encodedcommand-stdin-2026]);
  Homebrew and Oh My Zsh prescribe `sh -c "$(curl …)"`
  ([B-homebrew-installer-bash-c-command-substitution-2026],
  [A-ohmyzsh-installer-command-substitution-tty-2026]). Salt names Dorc's CURRENT shape as
  its own size fallback: "we might try simply invoking '/bin/sh -s' and passing the … SHIM on
  SSH stdin". The genuinely unattested variant is `a=$(cat); eval "$a"` — everyone hands the
  payload to a FRESH interpreter's `-c`; no stated reason against `eval`, only absence.
- The stdin route's stated blocker is the PTY, not the filesystem: pipelining ships off
  because `requiretty` sudo needs `-tt`, and under `-tt` "ssh puts the tty into 'raw' mode …
  no good way … to detect EOF on stdin", so the pipe "hangs forever"
  ([A-ansible-pipelining-default-false-rationale-2026],
  [A-ansible-pr-13487-pipelining-requiretty-closed-2015]); salt-ssh hit the same wall and its
  TODO names copy-then-exec as the only escape ([A-saltssh-shell-tty-blocks-stdin-shim-2026]).
  Dorc has ruled out BOTH halves by other welds — `-T` is required, the transport never
  escalates, and `27C` entry is `sudo -n` inside authored recipes — so Dorc's stdin route is
  structurally exempt from the incumbents' blocker. `H-slurp-is-novel-because-payloads-were-
  not-sh` is supported in the incumbent's words: pipelining "does not work for Python modules
  involving file transfer … or for non-Python modules"
  ([A-ansible-become-unprivileged-tmpfile-ladder-2026]).
- REVISED SHAPE for the in-memory route (conductor's read, un-ruled): prefer the ATTESTED
  form — slurp from stdin, then hand the bytes to a FRESH interpreter, `sh -c "$a"` (or
  `bash -c "$a"` for a bash-floor book), rather than `eval` inside the wrapper. The artifact
  never touches the ssh command line, so Salt's base64 armour and the login-shell quoting
  problem do not arise; `$0` is `sh` (single-stream-consistent); `exit` is contained
  without a subshell; the book-floor problem (three floors, charter) is solved by naming the
  interpreter. The one cost is Linux's 128 KiB single-argument cap
  ([A-linux-binfmts-max-arg-strlen-2026]) — over-cap: the file route if `fs-write`, else
  `eval` for POSIX-floor books only, else refuse by name. Mitogen's length-prefixed stdin read
  (read exactly N bytes so stdin stays reusable) is the idea if a live stdin is ever wanted.
- `H-pivots-walk-toward-files` FALSIFIED as a law; true only for FILE TRANSFER. Toward a
  structured subsystem: OpenSSH 9.0 scp→SFTP, a deliberate compat break over shell-globbing
  quoting ([A-openssh-9-scp-to-sftp-default-2022]). Away from files and side channels:
  Ansible accelerate (a port-5099 daemon) → pipelining ([A-ansible-accelerated-mode-deprecated-2017]);
  Kubernetes never built a file lane (`kubectl cp` is tar over exec stdin —
  [A-kubectl-cp-tar-over-exec-stdin-2026]); Fabric removed recursive `put` and said a revival
  should use "an approach not involving intermediate files"
  ([A-fabric-upgrading-shell-wrapper-removed-2026]). The reliable direction is FEWER
  ASSUMPTIONS ABOUT THE TARGET, which is sometimes a subsystem and sometimes a pipe.
- The login-shell axis, priced by the incumbents (Q16): the root cause is sshd itself —
  `execve(shell, [shell0, "-c", command])` ([A-openssh-sshd-session-login-shell-dash-c-2026])
  plus client-side argv flattening ([A-openssh-ssh-manpage-command-flattening-2026]). The
  industry mostly REFUSED non-Bourne: Ansible ejected csh/fish to a collection
  ([A-ansible-posix-csh-shell-plugin-2026], [A-ansible-posix-fish-shell-plugin-2026]); Bolt's
  `login-shell` list excludes them and is "experimental" ([A-bolt-transports-login-shell-tmpdir-2026]);
  Terraform breaks on PowerShell and tells you to set `cmd.exe`
  ([A-terraform-issue-31423-powershell-scp-2022]). On the `sh -c` wrapper the incumbents
  split: pyinfra wraps everything; Fabric REMOVED its wrapper as "extremely error-prone …
  frustrating escaping rules" ([A-fabric-upgrading-shell-wrapper-removed-2026]). The
  distinction that matters for `ax-login-shell`: Fabric wrapped the USER's arbitrary command
  (quoting hell); Dorc's wrapper is a CONSTANT engine-authored string with no user bytes in
  it (the artifact rides stdin), verified once — the escaping objection does not apply.
  Supports the human's priority (Bourne-family first) and the single-quoted constant wrapper.
- Pain-thread metric (Q13): orchestrator transport threads top out ~15 reactions; the
  high-reaction "read-only file system" threads are APPLICATIONS failing on read-only fs,
  not orchestrators. ~SUSPECT the population hitting orchestrator transport limits is small
  and expert — consistent with G2 being the failed-host cell.
- Effort (Q14) clusters on interpreter and file-lane gaps, never on unwritable fs — the
  fourth independent reproduction. Rundeck formalizes copy-then-exec as two pluggable
  services (File Copier + Node Executor) and its own worked example is an HTTP-upload copier
  ([A-rundeck-file-copier-node-executor-seam-2026]) — the charter's HTTPS seed is a
  documented pattern for DELIVERY to the host, not for results back.
- Marker precedents: salt-ssh's `RSTR` delimiter on BOTH stdout and stderr with a truth table
  ending "RSTR in stderr, No RSTR in stdout: Undefined behavior"
  ([A-saltssh-shim-armored-in-ssh-command-2026]) — a working marker protocol AND an honest
  record of its unhandled cell (Dorc's marker-absent ⇒ Unknown rule covers it). Docker's
  non-tty stream carries a FOURTH id, `Systemerr`, for transport-level failure distinct from
  payload stderr ([A-docker-stdcopy-four-stream-multiplex-2026]) — priced prior art for the
  degraded multiplex cell ONLY.
- The copy-then-exec HANDSHAKE (`mkdir … && echo …`) is a documented high-traffic failure
  surface in the incumbent (intermittently "returned empty string", 128 comments —
  [A-ansible-issue-13876-tmpdir-handshake-empty-2016]), and the file route's permission
  ladder took a decade and a compat break ([A-ansible-become-unprivileged-tmpfile-ladder-2026]).
  Argues for keeping Dorc's selection host-side and single-exchange, and for slurp-first.
- Unmeasured cells: OpenSSH's own exec-request size limit (no first-party statement found);
  the frequency of the `"$(cat script)"` idiom (code search refused the query).
