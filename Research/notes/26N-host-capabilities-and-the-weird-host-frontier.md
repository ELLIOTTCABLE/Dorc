# 26N — host capabilities and the weird-host frontier: direction synthesis of the immutable-fs-targets round

> AI-authored (Fable, design-rubber-duck sitting with the human, 2026-09-03). Notes-tier and
> historical once superseded. What this is: the project-direction-setting TAKEAWAYS of one
> interactive-research round (`.claude/research/immutable-fs-targets/`), written so a
> successor never re-derives them. What it is not: an exhaustive or forever-authoritative
> capability table — the human wants a robust capability SYSTEM before any centralized table,
> and coverage-per-target is expensive empirical work outside the spike. The round's durable
> evidence base: `round-charter.md` (THE human-typed ledger — authority on what was typed),
> `plan.md` (the problem-space map with three post-gate addenda), `turn01`/`04`/`05`/`06`
> (the four graded fronts), `turn02`/`03` (the conductor's corpus reads), `sources.json`
> (117 sources; all `graded-by: subagent`; the load-bearing grades are owed main-context
> re-verification and are cited here as provisional). Sibling: `plans/26K`. Nothing here is
> ruled unless marked human-typed; leans are marked as leans.

## §0 — one screen

The trigger was `26K:sit-stdin-copy-exec-amendment`: every shipped artifact is delivered as
the remote shell's stdin, so any interior stdin-reader eats the rest of the artifact — under
local-exec a pivot book's first bare `ssh host …` line does exactly that, on probe and apply
alike, and the marker still reports a clean exit (`plan.md` §landscape). The amendment's shape
is now well supported (§2). The larger yield is elsewhere: (a) almost no real host is
unwritable by design — the value cell is the FAILED host, and the incumbent's own tracker
documents that need unfixed; (b) nothing found forces the core language — what falls out is a
CAPABILITY SYSTEM whose abstract shape is §4, the human's central ask; (c) a first weirdness
pareto frontier of real hosts and channels (§6); (d) the THREE FLOORS, typed by the human and
not previously written down (§3).

## §1 — what the round established (lifted; certainty-marked; detail in `plan.md`)

- +SURE `granule-unwritable-is-failure-not-design` — every read-only-root system reached
  ships a writable tmpfs as a supported part of its configuration (OpenWrt, Home Assistant
  OS, the rpm-ostree family, Flatcar, TrueNAS, RHEL read-only-root, Ubuntu Core, systemd's
  hardened services); a second class is fully writable but volatile (Pi overlay, Alpine
  diskless, Unraid); nowhere-writable by design ≈ hardened containers, plus Talos which has
  no shell. Four independent reproductions (fronts 1, 2, 3 and the effort-metric selection).
  The real unwritable cell is FAILURE: root remounted read-only (an fstab convention, not the
  fs default), ENOSPC where `/tmp` sits on root (RHEL; Debian ≤ 12; Debian 13 moved it to
  tmpfs), and Ansible issue 25831 is verbatim that scenario, closed unfixed
  ([B-ansible-issue-25831-readonly-root-2017]). No sibling has a no-fs mode.
- +SURE `finding-shoe-in-is-the-file-lane-and-the-login-shell` — "sh-only target" is already
  pyinfra's and cdist's claim. What strands them on the archetypal weird host is the FILE
  LANE (pyinfra needs SFTP/SCP; vanilla OpenWrt's dropbear ships no sftp-server; ESXi is
  dropbear too) and the LOGIN SHELL (Ansible's second precondition, "an interactive POSIX
  shell", denied by pfSense's menu-bound `admin`, Windows' `cmd.exe`, CLI-fronted network
  OSes). Dorc's session-carried artifact needs no file lane. Standing constraint: the core
  delivery path never grows a dependency on sftp/scp.
- +SURE `finding-modal-sh-is-not-dash` — on the frontier `/bin/sh` is bash (macOS, NixOS),
  busybox ash (OpenWrt, Alpine, ESXi, Synology's router OS), mksh (Android), FreeBSD sh (the
  BSD appliances). The language floor is measured against dash∩posh only; these are
  presumed supersets, and presumption is the gap (§3's wrinkle; `local` under mksh is the
  canary). A measurement lane is owed when a machine can afford it.
- +SURE `finding-heredoc-hazard-is-narrow` — dash and busybox never touch the filesystem for
  a here-document; bash < 5.1 always writes a temp file (macOS `/bin/sh` 3.2, RHEL 8's 4.4),
  bash ≥ 5.1 only above pipe capacity ([A-bash-53-release-tarball-2025],
  [A-dash-openhere-redir-source-2026]). Dorc's own scaffolding emits no heredocs; only
  book/oracle bytes are exposed, identically under the off-ramp.
- +SURE `finding-memory-route-is-not-novel` — the ARGUMENT form is what incumbents do (Salt
  base64-armours a shim into the ssh command; Mitogen hands its stage to `python -c`;
  Ansible-on-Windows `-EncodedCommand`; Homebrew/Oh My Zsh `sh -c "$(curl …)"`), and Salt
  names `/bin/sh -s` on stdin as its OWN size fallback ([A-saltssh-shim-armored-in-ssh-command-2026]).
  The stdin route ships disabled in Ansible because of the PTY, not the filesystem
  (`requiretty` sudo ⇒ `-tt` ⇒ raw mode ⇒ no EOF ⇒ hang;
  [A-ansible-pipelining-default-false-rationale-2026]). The unattested variant is `eval`
  from a shell variable; no stated reason against it, only absence.
- +SURE `finding-pivots-walk-toward-fewer-assumptions` — not toward files: Ansible retired a
  daemon transport TOWARD stdin pipelining; Kubernetes never built a file lane (`kubectl cp`
  is tar over exec); OpenSSH moved scp onto SFTP for file TRANSFER specifically, with a
  deliberate compat break; Fabric removed its `sh -c` wrapper and its file-transfer extras.
- +SURE `finding-incumbents-refused-non-bourne` — sshd runs the account's shell with
  `-c command` and the client flattens argv into that string; Ansible ejected csh/fish to a
  side collection; Bolt's list excludes them and is experimental; Terraform breaks on
  PowerShell and tells you to set `cmd.exe` (`plan.md` addendum 3).
- +SURE `finding-effort-metric-splits-the-frontier` — the human's metric ("has somebody put
  a ton of effort into supporting X?") separates sh-shaped efforts (Merlin's hooks,
  ghettoVCB, Synology_HDD_db, UniFi `on_boot.d`, QNAP autorun) from not-sh-shaped ones
  (RouterOS's pull-only package manager in its own language; netmiko's screen-scraping;
  Dell's off-box Redfish). Every sh-shaped one is a BOOT-HOOK re-invention because changes
  do not survive an update or reboot — on the frontier the scarce capability is
  `fs-durable`, not `fs-write` (`turn05` Findings).
- +SURE `finding-fd-sinks-die-under-privilege` — `sudo` closes every fd above 2 by default
  (admin-gated override; [A-sudo-closes-fds-above-stderr-2026]); FreeBSD exposes only
  `/dev/fd/0-2` without an extra mount; Linux re-open of an inherited pipe by path is
  ownership-gated. Any fd-above-2 sink is a per-context capability, never an assumption.
- +SURE `finding-no-multiplex-except-forced` (human-typed caution, charter) — the project
  escaped multiplexed channels at real cost; a second in-band lane is a regression, not a
  fix; multiplexing may be forced back as a fallback for specific degraded hosts only. On a
  no-fs host in-band framing is nevertheless the only unaided survivor (framings priced:
  git side-band, apt `Status-Fd`, salt's `RSTR`, Docker's `Systemerr` id).
- +SURE `finding-sshd-is-not-the-no-fs-blocker` — sshd's privsep directory must exist, not
  be writable, and login-record writes discard failures ([A-openssh-loginrec-write-nonfatal-2026]);
  the rest of a login (PAM, home dir) is unsourced. The human's someday-test stands.
- +SURE `finding-uci-batch-eats-stdin-and-is-a-heredoc` — OpenWrt's idiomatic batch
  spelling ([B-openwrt-uci-batch-stdin-heredoc-2026]); after the amendment a `uci batch`
  with no heredoc reads EOF and silently applies nothing — a named lint class (stdin-default
  commands with no redirect become silent no-ops: `uci batch`, `cat`, `patch`, `crontab -`).
- ~SUSPECT `finding-transport-pain-population-is-small-and-expert` — orchestrator transport
  threads top out around fifteen reactions; the big "read-only file system" threads are
  applications, not orchestrators (`turn06` Q13 note).

## §2 — the delivery shape (lean-tier; the amendment's answer as it stands)

The wrapper is a CONSTANT, engine-authored, single-quoted string on the remote command line,
ONE physical line (csh forbids newlines inside single quotes; salt-ssh flattens for the same
reason), containing no `!` (csh history expansion) and no user bytes; the artifact rides the
session's stdin; the login shell parses one token. Strawman (illustration, not spelling):

```sh
sh -c 'a=$(cat) || exit 70; <integrity check on "$a">; d=/tmp/dorc.<nonce>; if mkdir -m 700 "$d" 2>/dev/null && printf "%s\n" "$a" >"$d/a"; then caps=fs-write; <interp> "$d/a"; rc=$?; else caps=; ( eval "$a" ); rc=$?; fi; rm -f "$d/a" 2>/dev/null; rmdir "$d" 2>/dev/null; printf "\n<nonce> dorc-session/1 attempt=<n> rc=%s capabilities=%s @@dorc-session@@\n" "$rc" "$caps"'
```

- `lean-slurp-first-then-materialize` — read the whole artifact into memory BEFORE any
  decision: a write that fails partway (ENOSPC after `mkdir` succeeded) would otherwise lose
  the unread bytes from the pipe and leave a truncated file. Both routes then see identical
  bytes; both give interior stdin-readers EOF (the stdin hazard is fixed identically by
  either); a pre-execution integrity check (`cksum`, POSIX and busybox) becomes possible,
  which the streamed pipe never allowed — the single-stream form gains what
  `30P:mech-two-standups` already demands of the multipart form.
- `lean-routes-keyed-on-book-floor-not-size` (the human's correction) — a POSIX-floor book
  always evaluates in memory the same way (subshell `eval`: `exit`, traps, `set -e`
  contained; `$0` = `sh`); a non-POSIX book uses a named interpreter's `-c`, whose Linux
  128 KiB single-argument cap is an honest refusal-by-name, never a behaviour switch; the
  file route runs whenever `fs-write` measured. `sh "$f"`, never `./f` (`noexec /tmp` is a
  live CIS/DISA baseline and blocks `execve`, not `open`). `mkdir -m 700` at a
  controller-literal root, never `mktemp` (BSD template warts; and a private dir is outside
  `fs.protected_regular`), cleanup `rm -f` then only-empty `rmdir`, residue disclosed — the
  DREP scratch precedent verbatim (`spike/CLAUDE.md` rul-probe-writes-only-what-it-owns).
- `lean-capabilities-on-the-marker` (human-typed: `capabilities`, an unordered keyword set,
  not `mode`) — the marker line discloses what the standup measured; transport framing,
  additive-keys policy.
- `lean-host-side-self-select-is-the-floor` — one exchange, no handshake: it works on every
  channel class including request/response (payload inline, no mid-session input). The
  in-session HANDSHAKE (wrapper measures, prints capabilities, then reads the body the
  controller chose) is a duplex-channel ENHANCEMENT that lets the controller choose the
  emission FORM; it is one exchange, so ~SUSPECT it does not trip
  `rul-repeated-probing-reviewed-before-design`; the human's KISS lean is self-select first,
  a second tunnel only if something unforeseen forces it.
- `lean-never-sftp-never-base64` — the host-side write rides stdin through `cat`/`printf`
  (the OpenWrt/ESXi differentiator); base64 armour exists only to put payloads on command
  lines, which we never do (human: bullet-proof quoting is owed anyway; investigability).
- `dark-corners-of-eval-as-transport` (~SUSPECT each; none fatal; owed to the measurement
  lane): the interpreter is PATH-resolved by the login shell (per-host parameter; Android
  has no `/bin/sh`); `$$` inside the subshell is the wrapper's PID; error text cites lines
  relative to the string, so the receipt must record the mode for `dorc why`; the wrapper's
  variables are visible to the book (munged names, closed-set discipline); `$(cat)` reads
  to EOF, foreclosing a live stdin (Mitogen's length-prefixed read is the alternative);
  `$(…)` strips trailing newlines and NUL is unspecified (refuse NUL at plan time);
  whole-parse-before-run (a feature, given the plan-time syntax gate). The human's standing
  posture: eval is a lean for the spike, not a ruling.
- What the amendment does NOT fix, by design: a here-document under bash-as-sh < 5.1 with
  no writable temp dir fails at that line — identically under the off-ramp. The answer is
  §4's hard deny, not a mechanism.

## §3 — the three floors (human-typed; recorded here because it was not written down)

Different PRIORITIES produce three floors. (1) The oracle/dorc-lang LANGUAGE floor is
MAXIMAL-by-choice: users write it, write a lot of it, and must off-ramp without hating us;
so it targets slightly above the nominal standard (`local` and other defensive habits) and
we do the compatibility work per feature — `KNOBS:kWHICHSH`'s dash∩posh weld. (2) The
TRANSPORT floor may be larger: it is constrained neither by the wider-compatible-sh mission
oracles carry nor by backwards compatibility; we continuously measure hosts, map
interpreters to capabilities, and do whatever stands up transport and boots the user's book.
(3) The BOOK-language floor is a disjoint set at base and gently a maybe-superset of (1):
bash/zsh support aspired to for newcomers' existing scripts, but books must execute oracle
code; unruled, lean wide. Possible only because standup/transport/probing is DECOUPLED from
book-byte shipping and evaluation (ash/dash for our purposes, then `bash` for the mutative
step if the host has it). Keep the door open.

Conductor wrinkle (`plan.md` addendum 3 and chat, unobjected): (2) has two layers. Its
pre-measurement KERNEL — the constant wrapper — runs under whatever login shell and `sh`
the host has, so its construct set is bounded by the INTERSECTION of every interpreter we
stand up on, smaller than (1); only the post-measurement expansion exceeds (1), and freely.
The link between (1) and (2) is PROBING: oracle bodies execute through the transport, so
every transport interpreter must accept the language floor — which is why the unmeasured
supersets are a transport problem as much as a language one.

## §4 — the capability system: abstract shape and cardinality (the human's central ask)

### §4.1 what a capability is

- `def-capability-is-a-measured-predicate` — a capability is a predicate about what OUR
  machinery can do at a (context, resource) at standup time, MEASURED by attempt, never
  declared by an author and never inferred from identity or version (`KNOBS:kBOOT`'s
  feature-detection-not-user-agent-sniffing; front-1: permission tests lie under ENOSPC,
  quota, `fs.protected_regular`; the attempt IS the measurement, and for our own scratch the
  measurement and the use are the same syscall, so there is no gap to exploit). Tri-state:
  present / absent / unmeasured, and unmeasured reads as absent for every licensing
  consumer (`silence-licenses-nothing`, extended). The human's standing correction from an
  orthogonal session: writability is not a property of the host but of (mount namespace ×
  unit sandbox × LSM policy × dm-verity × path) at the syscall — so capabilities are keyed
  by CONTEXT, never by host.
- `fence-capabilities-are-engine-vocabulary` — the keyword set is engine-owned, closed at a
  version, extended by new name only (the `__role` posture), and strawman-tier until
  publication (`rul-strawman-formats-no-compat`). Refag is preserved because a capability
  describes the engine's own scaffolding's environment ("can I create a private directory
  at my literal root here?"), never a referent of the user's world; the fence is that no
  capability may ever be something an oracle author would otherwise author as a fact.
  Authored ENTRY RECIPES (`plans/27C`) may contribute landing steps (the `13A` busybox
  bootstrap as an entry form in a Windows-host oracle), never capability values.
- `fence-capabilities-are-never-plan-lines` — a capability is consumed by the standup, the
  emission planner, and the aid plane; it never appears as a plan line, never licenses an
  elision, and never feeds survival (`rul-flag-is-razor-residue` unchanged).

### §4.2 supply, demand, and the join

- SUPPLY — measured by the standup wrapper in EACH entered context (the command-line
  target; every `27C` entry; every book-spelled transit the probe descends into — reliable
  probing of nested targets is the meta-orchestration requirement, human-typed), disclosed on
  the marker line, recorded in the receipt per context. Channel-static capabilities
  (`channel-interactive`, `channel-returns`, the request/response in/out caps) are known
  from the driver, not measured.
- DEMAND — `census-required-capabilities`: a per-chunk, ⊤-biased structural census over the
  bytes of every transport-bound chunk (one AST walk, no fixpoint — the shape of the
  existing name-observation census), DERIVED, never declared: a here-document IS the
  `heredoc` requirement; a `#!/bin/bash` IS `interp:bash`; a `read` or a stdin-default
  command with no redirect IS `stdin-live` (today: a lint, since every route hands EOF); a
  `$0` use in a single-stream artifact IS `$0`-sensitivity; a `.` line that must mirror IS
  `fs-tree`. Spelled in sh, no annotations — `KNOBS:kOOB` intact. The human PINNED this as
  a language/analyzer seam (charter): required capabilities pushed INTO/THROUGH the lattice
  so sections of a book bound for different hosts fail fast AND granularly, without One
  Weird Lil Guy forcing all infra to avoid heredocs; tightly coupled to the owed sitting on
  how far Dorc reaches into argv/heredocs/herestrings (the same walk).
- THE JOIN — `required(chunk) ⊑ measured(context(chunk))` at the two standups
  (`30P:mech-two-standups`). Apply: a mismatch is INTEGRITY-class, not world-uncertainty —
  "we see the book, count the bytes, know there is no fs and that it is bash 5.1, so we KNOW
  a section will misbehave" — hence WITHHOLD before shipping a single mutative byte
  (human-typed hard deny; `rul-integrity-failure-withholds-mutation`). Probe: withhold the
  FEATURE (no report lane, no tree, no in-memory route for that book), never a plan line.
  Unmeasured-but-required: ~SUSPECT withhold, same class. Exclusion-checked: the join is per
  (chunk × context), so an entered context's measured set governs the chunks that run there
  (fd-over-2 is lost across `sudo`; `fs-write` may differ under a unit sandbox).
- `rul-lint-at-authoring-deny-at-apply` (human-typed) — plan-time WARNINGS are expensive and
  fraught; the shape is a normal lint during authorship and a hard deny at apply. Making
  every line × every host work is not the product's job.

### §4.3 cardinality and the shapes seen

- Two axes: KEYWORD (small, closed, engine-owned — O(10–20) for the spike) × CONTEXT (the
  existing coordinate context slot, heading toward `30W`'s product over index-kinds —
  capabilities are keyed exactly as facts are keyed, so the axis already exists). A third,
  implicit: the RESOURCE, always one the engine owns (its literal scratch root; the artifact
  path) — never a book path.
- Shapes observed this round: (a) measured boolean per context — `fs-write`, `fs-tree`
  (nested dirs + several files; a host may allow a file but not a tree — human), `fs-exec`
  (only if we ever exec rather than read), `fd-over-2`, `drep-oob` (a correct out-of-band
  report lane), `sudo-without-tty`, `cksum-present`; (b) measured boolean per (context ×
  interpreter) — `heredoc` (run a tiny here-document at standup; feature-detect, never
  version-sniff); (c) measured SET per context — `interp:<name>` presence; (d) numeric caps —
  `argv ≤ 128 KiB` (kernel), request/response in/out caps; (e) driver-static —
  `channel-interactive`, `channel-returns`, `stdin-payload` (Windows' stdin pipe is unreliable
  → file lane by scp; `13A`). The rule for granularity: key each capability at the COARSEST
  context that keeps every consumer's decision sound, then exclusion-check it under entry,
  `sudo`, other-user, FreeBSD, and the failed-host cell; refine only when a consumer breaks.
- NOT capabilities: `fs-durable` (unmeasurable; only a pivot book that reboots the host
  while we watch could know; deferred — human; the consequence is a retention story for rich
  logs on non-durable hosts), version numbers (never), login-shell identity (implicit — the
  constant wrapper survives or the session is lost), anything under secrets (quarantined
  from this model, `TODO.md`).

### §4.4 threading through the engines

- ANALYSIS — the census (§4.2) beside the existing censuses; requirements keyed by the
  chunk's context coordinate; the hoist/emission planner (`28Q:pin-emission-planner-universal`)
  reads them.
- GENERATION — the emission planner selects the FORM from measured supply: single-stream
  needs nothing beyond the floor; bundle-at-dorc-lang-boundaries (`30Ng`) needs `fs-write`;
  the mirrored tree and every `$0`-relative load need `fs-tree` and cwd parity (`30P`'s
  multipart standup); the compiled guards-only face needs no return channel at all. The
  human's intuition that the CONTROLLER should make emission decisions from capabilities is
  satisfiable under the handshake or a prior run's receipt; the self-deciding body remains
  the floor for channels that cannot handshake.
- TRANSPORT — the wrapper measures; the marker discloses; the simulated driver scripts every
  capability both ways so the DST tier carries a sometimes-assert per keyword per direction.
- WHY-SURFACE / AID — the receipt records measured supply per context; a withheld section is
  explained by NAME with BOTH remedies (§4.5); candidate row `aid-capability-denial-remedy`
  (push at apply-standup refusal; pull in `dorc why`); the denial is `trust-tier-is-syntax`
  material (measured, never claimed). FORFEITS is untouched (a capability withhold is a
  transport condition, not an analysis limitation).

### §4.5 gradual enhancement and onboarding — the principled way

Two user ladders meet Dorc's feature matrix at the join, and the discipline is monotone in
all three directions:

- `ladder-host` — capabilities a HOST gains by admin work: give it a writable tmpfs, install
  sftp-server, set `DefaultShell`, disable `requiretty`, bootstrap busybox, mount `fdescfs`.
- `ladder-book` — requirements a BOOK sheds by author work: POSIX-ify a bash book, keep
  here-documents below pipe capacity or off no-fs hosts, add `</dev/null` to stdin-default
  lines, avoid `$0` in single-stream books, spell a helper as a `.`-sourced file rather than
  a mirrored tree.
- `matrix-features` — Dorc's own features CONDITIONED on supply: in-memory delivery (POSIX
  book, or `interp:<x>` under the cap) · file route (`fs-write`) · bundle (`fs-write`) ·
  mirrored tree and `$0`-relative loads (`fs-tree`) · report lane out-of-band (`drep-oob`;
  in the entered context) · report lane multiplexed (forced fallback ONLY; the human's open
  lean: elision LICENSURE denied under an unreliable/multiplexed DREP — load-bearing records
  `predicts` and `nothing-else` need the reliable lane; aid-only `decline` records may ride
  a degraded one) · live topology (`fs-write` plus FIFO/fd) · context entry
  (`sudo-without-tty`; `fd-over-2` lost across it) · here-document chunks (`heredoc`) ·
  probes on request/response channels (payload ≤ in-cap; results ≤ out-cap; sentinel on the
  surviving side of truncation) · no-return channels (no probe; the guards-only face; the
  cell's product story is punted — human).
- Invariants: (i) MONOTONE — climbing either ladder never removes a feature; adding a
  keyword never removes one (`24A:rul24-threefunc-monotonic`'s shape). (ii) NEVER SILENT —
  every withheld feature names its capability and both remedies; `silence-licenses-nothing`
  applies to capabilities. (iii) NEVER ENGINE-CLIMBED — Dorc never climbs a ladder for the
  user: no environment mutation (`26Lb:rul-no-engine-environment-mutation`), no rewriting
  bytes to fit a host (`KNOBS:kBACKFLIPS` welded); the user chooses which ladder to climb.
  (iv) DEMAND IS SPELLED IN SH — a requirement is what the bytes already say, never an
  annotation. (v) GRANULAR AND FAIL-FAST — per chunk × context, before network on apply.
  (vi) LINT-AT-AUTHORING, DENY-AT-APPLY — never a plan-time warning (human-typed). The
  onboarding story this yields: a newcomer's bash book on ordinary hosts loses nothing; the
  first weird host in the book costs exactly the chunks bound for it, each named with the
  two-minute fix on whichever ladder is cheaper for that user.

### §4.6 rulings taken at conductor tier, and what stays the human's

Taken (low-risk, ledgered per `rule-yourself-dont-pile-asks`): capabilities are measured,
never declared; keyed by context; engine-owned closed vocabulary, extend-by-new-name;
never a plan line, never a license; the v1 keyword list above is provisional under
`rul-strawman-formats-no-compat`. The human's, open: eval as transport (lean, spike-trial);
DREP-degraded licensure; handshake timing (KISS: self-select first, typed); the no-return
cell's product story (punted); `fs-durable` (deferred); which of the four owed human-as-
debugger items to close by hand (the Ubiquiti page; the iLO scripting guide; a fish/csh shell for
the marker measurement; population/habit).

## §5 — language holes: the verdict

Nothing found forces the CORE language. The one channel-forced analyzer carve already exists
(`310`'s "simple words are identity across the remote re-parse; else decline" — the ssh
argv re-parse, `28Q` §3). An unwritable host forcing single-stream is a value-loss the
off-ramp shares (`FORFEITS:forfeit-plain-sh-inclusion-analysis` stands). The here-document
hazard is real, narrow, and aid-plane. stdin in shipped bodies is already fenced
(`26Lb:rul-interactivity-is-local-books`; the predict tracer treats `read` as run). What
fell out instead: the capability vocabulary and census (§4), one spelling rule (`sh "$f"`),
one contract sentence for oracle authors (the DREP sink is write-only, append-only, and may
be a device — never read it back; pinned, human), and lints: `$0` read in a single-stream
artifact (the `30P` open hint); stdin-default commands with no redirect (silent no-op);
here-documents under bash-as-sh < 5.1; judo's close-fd-0 as an opt-in strict-tier lint
(never core; human-typed; safe to recommend only once delivery no longer reads the
artifact from fd 0). Banked for a joint turn, probable nack given the meta-orchestration
focus: how a book could spell, in sh alone, that a host is Windows and needs `cmd` first.

## §6 — the weirdness pareto frontier (first cut; brief; `turn04`/`turn05` tables hold the evidence)

Entries are non-dominated weirdnesses; dominated hosts ride as sub-bullets; ruled-out ones
are listed. Testability is -GUESS (from memory, one confirming pass owed): most of the
frontier has a container or VM form; the bare-metal-only residue is consumer routers and
vendor BMCs.

- `frontier-openwrt` — busybox ash (superset unmeasured), dropbear with no sftp, no Python,
  tiny flash with tmpfs `/tmp`, minimal PATH, `uci batch` eats stdin, changing root's shell
  bricks ssh; official container rootfs exists. Sub-bullets: Asuswrt-Merlin (firmware-
  invoked hook points with timeouts; bare metal), OpenBMC (tens of MB of flash; QEMU
  targets), Alpine (dominated: OpenSSH present).
- `frontier-esxi` — busybox ash and dropbear like OpenWrt, plus shell and ssh off by default,
  break-fix framing, ramdisk scratch, vendor doctrine off-box; nested-virt only. The
  vendor-discourages-shell axis: pfSense (sshd off; `admin` menu-bound; root works), Ubiquiti
  (steers to its Debug Console; uncited — archiver refused; dogfood-available), Juniper.
- `frontier-home-assistant-os` — largest measured population; port 22 is a container; host
  ssh needs a physical USB stick; `ha` CLI is day-2; ZRAM `/tmp`; VM images exist;
  dogfood-wanted.
- `frontier-windows` — no sh; `cmd.exe` login shell; bootstrap a static busybox by scp or
  `curl.exe` then invoke by path; stdin pipe unreliable; CRLF twice; first-class target
  (human); VM.
- `frontier-android-adb` — mksh and toybox; no `/bin/sh`; exec-API channel with a 4 KB
  legacy command cap; emulator.
- `frontier-cli-fronted-network-os` (Junos; NX-OS, EOS) — a CLI parses first; a real shell
  behind a privilege or feature gate; csh root on older FreeBSD bases; the industry's answer
  is screen-scraping; lab VMs/containers exist.
- `frontier-bsd-appliance-login` (pfSense; OPNsense, TrueNAS CORE) — `/dev/fd/0-2` only
  without `fdescfs`; csh-era root shells; a PHP REPL as the automation surface; VM images.
- `frontier-nas-volatile-config` (QNAP; Synology) — config reset each boot / updates wipe
  customisations; persistence via a per-model flash partition; ash or bash; Python not base;
  a legacy Synology unit is dogfood-available.
- `frontier-volatile-root` (Pi overlay; Alpine diskless, Unraid) — everything writable,
  nothing survives reboot; convergence per incarnation.
- `frontier-bash-as-sh` (macOS; NixOS, RHEL 8) — here-documents hit disk on bash < 5.1;
  otherwise ordinary.
- `frontier-hardened-container` (Kubernetes pods, `docker --read-only`) — the only steady
  nowhere-writable class; exec-API channel; no sshd; trivially standable.
- `frontier-shell-behind-a-ladder` (Bottlerocket) — no ssh or shell in the image, a root shell
  three hops away, writes do not persist under dm-verity, 16 KiB user-data cap.
- `frontier-request-response-channel` (Proxmox guest-exec; SSM, Azure run-command) — payload
  inline, no mid-session input, output after exit, hard caps; the host may be ordinary;
  Proxmox needs nested virtualisation to exercise.
- `frontier-one-shot-channel` (cloud-init user-data; installer hooks) — no return path;
  guards-only face; cloud-init ruled especially attention-worthy (human) as the
  meta-orchestration standup channel and as Dorc-as-child territory.
- `frontier-serial-paste` — echo, merged streams, human-mediated; the paste rung.
- Ruled OUT: MikroTik RouterOS (no sh; no multi-line over ssh; pull-only effort in its own
  language); Talos (no shell, no writable fs, API-only; dominates Chrome OS); iDRAC/iLO
  (probably a restricted CLP shell; unverified); printers and consumer IoT (an effort
  desert). Ruled ORDINARY despite looks: Proxmox VE, Flatcar and Fedora CoreOS after first
  boot, NixOS, OPNsense, TrueNAS SCALE.

## §7 — prior-art digest that binds direction (one line each; `turn06` holds the table)

The incumbents' history says: the reliable pivot direction is FEWER ASSUMPTIONS about the
target; the memory route is common in argument form and blocked on stdin only by the PTY —
which Dorc's welds already exclude (`-T` required; the transport never escalates; entry is
`sudo -n`, failing rather than prompting under `requiretty`, at the price of context entry
on such hosts, itself a capability); non-Bourne login shells were priced and mostly refused
(the constant wrapper is exempt from Fabric's escaping objection because it carries no user
bytes); the copy-then-exec HANDSHAKE (`mkdir && echo`) is a documented intermittent failure
surface in Ansible, arguing for single-exchange host-side selection; Rundeck formalizes
copy-then-exec as two pluggable services (File Copier + Node Executor), our entry-recipe
versus interpreter split, and its worked example is an HTTP-upload copier (the charter's
HTTPS seed is a documented DELIVERY pattern, not a results pattern); Salt's `RSTR` marker
protocol documents its own undefined cell, which Dorc's marker-absent ⇒ Unknown rule
covers.

## §8 — deferred and unexplored (mentioned, not explored)

- `front-embedding-contracts` — NEEDS INVESTIGATION (human-ack; possibly a successor with
  the human): what cloud-init `runcmd`, init containers, Ansible `script`, Packer
  provisioners, and systemd `ExecStartPre` promise a foreign sh chunk (exit-code meaning,
  idempotence expectation, retries, size caps, logging) and what a Dorc-wrapped chunk offers
  back; the Dorc-as-child story. Partly in `plans/24R`/`26K`; the contract surfaces are unread.
- `front-request-response-mechanics` — mildly less: exact caps, encodings, truncation sides,
  exit and timeout semantics of SSM, Azure run-command, Proxmox guest-exec, Bottlerocket's
  SSM path, and how Ansible's and Packer's connectors drive them.
- `lane-interpreter-and-login-shell-measurement` — extend the real-binary battery
  (`.claude/research/kwhichsh-gcd/`) to busybox ash, mksh, FreeBSD sh, bash in POSIX mode,
  and to csh/tcsh/fish/zsh as login shells; exercise the wrapper, `eval` versus `sh -c`
  versus `sh file`, and `cksum`. Deferred: too expensive on the constrained machine.
- `someday-capability-coverage-table` — per-target coverage of each keyword; expensive,
  empirical, after a robust capability system exists (human).
- `someday-no-fs-structural-test` — whether sshd + sh + PAM stand up a session with NO
  writable filesystem (sshd itself does not need one; the rest unsourced).
- Also open: the no-return cell's product story; `fs-durable` and log retention on
  non-durable hosts; a reverse forward on the existing ssh connection as a no-new-tunnel
  return path (unsourced lead); the four human-as-debugger items (§4.6); the Windows-in-sh
  spelling (banked; probable nack).

## §9 — register touch-list when this firms (pointers only; nothing edited here)

`KNOBS:kBOOT` (floor statement gains "writable scratch is a probed per-context capability
selecting the delivery form"; the DREP lane named under it) · `ANALYZER-NEEDS` (two rows:
required-host-capabilities census; measured-host-capabilities supply, keyed by context) ·
`AID-NEEDS` (`aid-capability-denial-remedy`; the stdin-default and heredoc lints) ·
`spike/CLAUDE.md` (one delivery-shape bullet under host evidence; the DREP sink contract
sentence; the `$0`-mode note) · `plans/260` §5 (the invocation line and driver 2) ·
`ROADMAP` (the r31 gate text for `unit-host-index-and-entry`) · `plans/26K` §0c (from
"banked" to whatever the human types) · `USER_STORY`-adjacent authoring doc (the DREP
write-only sentence; the `sh "$f"` rule is engine-side only).
