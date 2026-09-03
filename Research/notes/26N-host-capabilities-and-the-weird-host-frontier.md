# 26N — host capabilities, delivery, and `$0`: direction synthesis of the immutable-fs-targets round

> AI-authored (Fable, two design-rubber-duck sittings with the human, 2026-09-03). Notes-tier
> and historical once superseded. What this is: the project-direction-setting TAKEAWAYS of one
> interactive-research round (`.claude/research/immutable-fs-targets/`) and of the design
> sitting that consumed it, written so a successor never re-derives them. Rulings are marked
> `[TYPED]` (the human typed the substance) or `[ACKED]` (the human acked the text as put);
> everything else is a lean or a finding. What it is not: an exhaustive capability table —
> the human wants a robust capability SYSTEM before any centralized table, and
> coverage-per-target is expensive empirical work outside the spike. The round's evidence base:
> `round-charter.md` (THE human-typed ledger — authority on what was typed), `plan.md` (the
> problem-space map with three post-gate addenda), `turn01`/`04`/`05`/`06` (the four graded
> fronts), `turn02`/`03` (the conductor's corpus reads), `sources.json` (117 sources; all
> `graded-by: subagent`; the load-bearing grades are owed main-context re-verification and are
> cited here as provisional). Sibling: `plans/26K`, whose `sit-stdin-copy-exec-amendment` this
> note closes.

## §0 — one screen

The trigger was `26K:sit-stdin-copy-exec-amendment`: every shipped artifact was delivered as the
remote shell's stdin, so any interior stdin-reader ate the rest of the artifact — under
local-exec a pivot book's first bare `ssh host …` line did exactly that, probe and apply alike,
and the marker still reported a clean exit. That question is CLOSED (§2). The sitting's larger
yields, in order of consequence: (a) a refined framing of the `kFAIL` weld — any Dorc act not
provably caused by the author's code is withhold-shaped (§6, human-typed); (b) the `$0`
authority ruling that answers `tc-dollar-zero-is-script-anchored` (§5, acked); (c) the ruling
that an unresolvable book-custody load refuses the plan before network rather than shipping
on a guess or dying halfway (§6, acked); (d) the capability SYSTEM's abstract shape, with its
census question left open as the next investigation (§4); (e) the findings: almost no real
host is unwritable by design, the value cell is the failed host, nothing forces the core
language, and a first weirdness pareto frontier of real hosts and channels (§1, §8); (f) the
THREE FLOORS, typed by the human and not previously written down (§3).

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
  [A-dash-openhere-redir-source-2026]). Dorc's own scaffolding emits no heredocs, and §2 keeps
  it so; only book/oracle bytes are exposed, identically under the off-ramp.
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
  spelling ([B-openwrt-uci-batch-stdin-heredoc-2026]); after §2 a `uci batch` with no heredoc
  reads EOF and silently applies nothing — a named class (stdin-default commands with no
  redirect become silent no-ops: `uci batch`, `cat`, `patch`, `crontab -`), whose lint is
  deferred with all lints and whose oracle-side question is parked (§10).
- ~SUSPECT `finding-transport-pain-population-is-small-and-expert` — orchestrator transport
  threads top out around fifteen reactions; the big "read-only file system" threads are
  applications, not orchestrators (`turn06` Q13 note).

## §2 — the delivery shape, RULED

**`rul-delivery-shape-file-backed-default`** [ACKED 2026-09-03; the sh-visible behaviour is
the ruling, its specifics may vary per target platform and by user configuration — the
interpreter word, the scratch root — and Windows needs its own tune (`plans/139`, `13A`)].
Three routes, each describable in one sentence, which is the standard the human set for
anything that touches `$0`:

- **`route-file-backed`**, the default wherever a writable filesystem exists: Dorc makes a
  private temp directory on the host, lays the plan and every file it sources inside it at
  the same relative positions they had under the directory `dorc plan` ran from, verifies
  each file's bytes, cd's to that root, and runs `sh /that/root/<the book's own filename>`.
  This is what an admin who scp'd the directory over and ran it from inside would get,
  except by absolute path and in a temp directory removed afterwards.
- **`route-in-memory`**, the floor, taken under `auto` only: if the host has nowhere
  writable, Dorc reads the whole flat plan into memory and runs it under `sh`, so `$0` is
  `sh` and nothing touches disk; a plan that sources files by `$0` or by relative path is
  refused for such a host by name before anything ships (`30P`'s single-stream rules).
- **`route-paste`**, the human face, unchanged: `ssh host sh <plan.sh` behaves like the
  in-memory route because it is the same shape (`26K:concl-offline-compile-is-a-face-not-a-product`).

Mechanics, ruled with it:

- `mech-constant-wrapper` — the wrapper is a CONSTANT, engine-authored, single-quoted string
  on the remote command line, ONE physical line (csh forbids newlines inside single quotes;
  salt-ssh flattens for the same reason), containing no `!` (csh history expansion) and no
  user bytes; the stream rides the session's stdin; the login shell parses one token. The
  incumbents' escaping objection (Fabric) does not apply, because no user bytes are in it.
- `mech-tree-crosses-the-pipe-as-a-stream` [ACKED, for the spike; the human is suspicious and
  will prove it out] — a multipart artifact reaches a remote host as ONE nonce-delimited
  stream, which the wrapper materialises with a `while IFS= read -r` loop, writing each
  section to its path under the root (`mkdir -p` for subdirectories, all inside the owned
  scratch). Never a heredoc in scaffolding; never a `printf` of a slurped variable (posh's
  `printf` is external, and Linux caps a single argument at 128 KiB). The single-file case
  is the degenerate tree. Every file is `cksum`-verified against controller-computed values
  before execution; a mismatch withholds (`rul-integrity-failure-withholds-mutation`). NUL
  refused at plan time; the CRLF gate unchanged. Both routes hand every interior
  stdin-reader EOF, so the founding hazard is fixed identically by either.
- `mech-in-memory-evaluation` — a POSIX-floor plan evaluates in a subshell `eval` (`exit`,
  traps, `set -e` contained; the human's spike lean, not a ruling); a non-POSIX book uses a
  named interpreter's `-c`, whose 128 KiB single-argument cap is an honest refusal by name,
  never a behaviour switch (`lean-routes-keyed-on-book-floor-not-size`, the human's). Dark
  corners, each ~SUSPECT and owed to the measurement lane: the interpreter is PATH-resolved
  by the login shell; `$$` is the wrapper's PID; error text cites lines relative to the
  string; the wrapper's variables are visible to the book (munged names, closed-set
  discipline); `$(cat)` reads to EOF; `$(…)` strips trailing newlines.
- `mech-cleanup-by-manifest` — `rm -f` per manifest file, `rmdir` per created directory,
  empty-only, in reverse order; never `rm -rf`, never a retry by name; residue after a
  severed session is disclosed, never chased. `rul-probe-writes-only-what-it-owns` and
  `rul-scratch-root-never-read-from-host` apply verbatim: the root is a controller literal.
- `mech-route-selection` — `auto` self-selects host-side in ONE exchange by the exclusive
  create; an explicitly requested route or form refuses at plan time when the probe's
  measurement cannot carry it (only defaults negotiate; `30I:lean-every-detected-mode-is-also-requestable`);
  the probe artifact is always flat and self-selects freely; the apply artifact's form is
  committed at plan time from the SAME invocation's probe measurement and re-verified at the
  apply standup. No receipt is ever read for this or anything else (rec-5, hard line, the
  human 2026-09-03: any kSTATE softening is an explicit, separate store).
- `nack-ship-both-forms` [human 2026-09-03] — carrying a tree and its flat fallback in one
  stream, the wrapper choosing in situ, is NACKED: the reviewed artifact is the shipped
  artifact byte for byte, and "proven-identity relocation" is not enough rationale to hide
  a second form from an admin. The wrapper lives outside the artifact.
- `mech-file-naming` — the plan file carries the input book's own filename, so `basename "$0"`
  is the author's; the generated-file marker inside the file (`ELLIOTTCABLE/Dorc#1`) says it
  is generated. Bundles carry engine names. This narrows `30I:rul-main-book-is-not-a-pre-source`'s
  accepted visible difference to the absolute prefix alone.
- `lean-never-sftp-never-base64` — the host-side write rides stdin (the OpenWrt/ESXi
  differentiator); base64 armour exists only to put payloads on command lines, which we never
  do. scp/sftp is RESERVED as a future capability-gated lane for arbitrary files, never a
  requirement for a plan to run; and Dorc moves no file whose loading path it does not manage
  (`.` targets only) — a controller path the book copies (`cp ./x`) stays the admin's, and
  whether the scp-then-apply pattern can be converged parks with `30T`'s content-establishment
  territory, unexpanded.
- `lean-capabilities-on-the-marker` [TYPED: `capabilities`, an unordered keyword set, not
  `mode`] — the marker line discloses what the standup measured; transport framing,
  additive-keys policy.
- What the shape does NOT fix, by design: a here-document under bash-as-sh < 5.1 with no
  writable temp fails at that line — identically under the off-ramp. The answer is §4's deny.

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
step if the host has it). Keep the door open. TABLED [human 2026-09-03]: the shebang as the
book's dialect declaration — "a shattered, unreliable, uneven floor"; a file extension is the
likelier start; bash books are Someday.

Conductor wrinkle (`plan.md` addendum 3, unobjected): (2) has two layers. Its pre-measurement
KERNEL — the constant wrapper — runs under whatever login shell and `sh` the host has, so its
construct set is bounded by the INTERSECTION of every interpreter we stand up on, smaller
than (1); only the post-measurement expansion exceeds (1), and freely. The link between (1)
and (2) is PROBING: oracle bodies execute through the transport, so every transport
interpreter must accept the language floor — which is why the unmeasured supersets are a
transport problem as much as a language one.

## §4 — the capability system: abstract shape and cardinality (the human's central ask)

### §4.1 what a capability is

- `def-capability-is-a-measured-predicate` — a capability is a predicate about what OUR
  machinery can do at a (context, resource) at standup time, MEASURED by attempt, never
  declared by an author and never inferred from identity or version (`KNOBS:kBOOT`'s
  feature-detection-not-user-agent-sniffing; front-1: permission tests lie under ENOSPC,
  quota, `fs.protected_regular`; the attempt IS the measurement, and for our own scratch the
  measurement and the use are the same syscall, so there is no gap to exploit). Tri-state:
  present / absent / unmeasured, and unmeasured reads as absent for every licensing
  consumer (`silence-licenses-nothing`, extended). Writability is a property of (mount
  namespace × unit sandbox × LSM policy × dm-verity × path) at the syscall, not of the host —
  so capabilities are keyed by CONTEXT, never by host.
- `fence-capabilities-are-engine-vocabulary` — the keyword set is engine-owned, closed at a
  version, extended by new name only (the `__role` posture), and strawman-tier until
  publication (`rul-strawman-formats-no-compat`). Refag is preserved because a capability
  describes the engine's own scaffolding's environment ("can I create a private directory at
  my literal root here?"), never a referent of the user's world; no capability may ever be
  something an oracle author would otherwise author as a fact. Authored ENTRY RECIPES
  (`plans/27C`) may contribute landing steps (the `13A` busybox bootstrap as an entry form in
  a Windows-host oracle), never capability values. If a declared capability is ever wanted
  (`fs-durable`, deferred), it must arrive as an oracle-surface measurement in the Host kind,
  never as a flag or sidecar, or `KNOBS:kOOB` breaks.
- `fence-capabilities-are-never-plan-lines` — a capability is consumed by the standup, the
  emission planner, and the aid plane; it never appears as a plan line, never licenses an
  elision, and never feeds survival (`rul-flag-is-razor-residue` unchanged).

### §4.2 supply, demand, and the join

- SUPPLY — measured by the standup wrapper in EACH entered context (the command-line target;
  every `27C` entry; every book-spelled transit the probe descends into — reliable probing of
  nested targets is the meta-orchestration requirement, human-typed), disclosed on the marker
  line, recorded in the receipt per context. Channel-static capabilities
  (`channel-interactive`, `channel-returns`, the request/response in/out caps) are known from
  the driver, not measured.
- DEMAND — `census-required-capabilities`: a per-chunk, ⊤-biased structural census over the
  bytes of every transport-bound chunk, DERIVED, never declared: a here-document IS the
  `heredoc` requirement; a `#!/bin/bash` IS `interp:bash`; a `read` or a stdin-default
  command with no redirect IS `stdin-live` (today: a lint class, since every route hands
  EOF); a `$0` use in a single-stream artifact IS `$0`-sensitivity; a `.` line that must
  mirror IS `fs-tree`. Spelled in sh, no annotations — `KNOBS:kOOB` intact. WITHDRAWN
  [2026-09-03]: the earlier "one AST walk, no fixpoint" characterisation. Requirements that
  involve values ride the existing value plane (`$0`-sensitivity is the load plane's `$0`
  flow, `30P`); book code admits dynamic forms Dorc cannot refuse, so the census is
  ⊤-biased wherever it cannot follow. **`open-census-needs-value-plane`** — whether the
  general census needs constant propagation or a lattice seat is OPEN and is the next
  investigation (§10). The human PINNED the seam (charter): required capabilities pushed
  INTO/THROUGH the lattice so sections of a book bound for different hosts fail fast AND
  granularly, without One Weird Lil Guy forcing all infra to avoid heredocs; tightly coupled
  to the owed sitting on how far Dorc reaches into argv/heredocs/herestrings.
- WHERE A HEREDOC LANDS is two requirements, not one: the construct is evaluated by the shell
  of the ENCLOSING chunk (its world's interpreter and filesystem); the payload bytes demand
  things in the DENOTED world only once the payload-declaration speech-act (`notes/26M`)
  names them as sh. Without a declaration the bytes are opaque and demand nothing — the wall
  is the cost, as everywhere. In the plain-argv form a remainder of simple words carries no
  heredoc, no `$0`, no `.`, so within r31's scope demand is empty beyond the transport's own.
- THE JOIN — `required(chunk) ⊑ measured(context(chunk))`, with execution certainty from the
  settle the world plane already computes for walls: won't-run chunks (elided, omitted, dead)
  demand nothing; a chunk bound for world Y is checked against Y's supply, never X's; a
  MUST-run chunk whose world lacks a requirement is a hard deny — "we see the book, count the
  bytes, know there is no fs and that it is bash 5.1, so we KNOW a section will misbehave"
  (human-typed; `rul-integrity-failure-withholds-mutation`); a MAY-run chunk (a guard
  fall-through, an unknown branch) ~SUSPECT proceeds and is annotated, since `kFAIL-perform`
  says unsure means run and denying on possibility would make defensive books unusable on
  constrained hosts on converged days — a fail-direction ruling the human has not typed;
  an unmeasured world is honest ignorance: annotate, never deny.
- **`rul-preflight-over-probe-time`** [TYPED 2026-09-03] — plan-time refuses only what the
  bytes alone prove (an explicitly requested form the book cannot take; NUL; CRLF).
  Probe-time measures only what changes the reviewed artifact — the `auto` form choice, so
  that what is reviewed is what ships. The APPLY STANDUP verifies everything else immediately
  before mutation, in the fail-fast window: the delivery route, `fs-write`, `fs-tree` for a
  committed tree, the interpreter, heredoc behaviour, and entered-context capabilities where
  entry is consented (the `27C` recipe under reuse-never-acquire; fd-over-2 is lost across
  `sudo`; `fs-write` may differ under a unit sandbox). Cheap checks may DOUBLE at probe time
  so the plan can warn. Ground: risking TOCTOU for a probe-time check exists only because
  humans take time to review; per-plan forward human attention is nearly never owed here —
  only last-minute fail-fast in edge cases, or first-minute provable-nope. "Full and upfront
  wherever at all possible" is the human's LEAN, not a weld.
- `rul-lint-at-authoring-deny-at-apply` [TYPED] — plan-time WARNINGS are expensive and
  fraught; the shape is a normal lint during authorship and a hard deny at apply. Making
  every line × every host work is not the product's job. All lints and hints named in this
  note are DEFERRED with the rest of the aid machinery while correctness and basic
  functionality land (human 2026-09-03).
- `mech-standup-is-compiled-on-demand-and-tree-shaped` [ACKED] — the standup is compiled
  per book × form, measures only the capabilities that pair needs, skips the dependents of a
  failed test, and ships as one artifact; never a fixed battery.

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
- GENERATION — the emission planner selects the FORM from the probe's measured supply, in the
  same invocation, under `auto`: single-stream needs nothing beyond the floor;
  bundle-at-dorc-lang-boundaries (`30Ng`) needs `fs-write`; the mirrored tree and every
  `$0`-relative load need `fs-tree` and the scratch-root cwd (§5); the compiled guards-only
  face needs no return channel at all. The controller makes the emission decision from
  measured capabilities (the human's intuition), and the self-deciding wrapper remains the
  floor for channels that cannot measure first.
- TRANSPORT — the wrapper measures; the marker discloses; the simulated driver scripts every
  capability both ways so the DST tier carries a sometimes-assert per keyword per direction.
- WHY-SURFACE / AID — the receipt records measured supply per context, which is a
  receipt-contents change and therefore clears `rul-durable-contents-reviewed-before-design`
  before it is designed; a withheld section is explained by NAME with BOTH remedies (§4.5);
  candidate row `aid-capability-denial-remedy`; the denial is `trust-tier-is-syntax`
  material (measured, never claimed). FORFEITS is untouched by any of this (a capability
  withhold is a transport condition, not an analysis limitation).
- SEAT — the r31 moment to reserve the representation is `310:unit-context-slot-product`,
  where the context key is being reshaped: a per-context measured-supply projection keyed by
  that key, and a per-chunk requirement set beside the name-observation census. Reserve;
  build nothing. The system itself builds with `seam-payload-forms`, when demand first
  becomes non-empty.

### §4.5 gradual enhancement and onboarding — the principled way

Two user ladders meet Dorc's feature matrix at the join, and the discipline is monotone in
all three directions:

- `ladder-host` — capabilities a HOST gains by admin work: give it a writable tmpfs, install
  sftp-server, set `DefaultShell`, disable `requiretty`, bootstrap busybox, mount `fdescfs`.
- `ladder-book` — requirements a BOOK sheds by author work: POSIX-ify a bash book, keep
  here-documents below pipe capacity or off no-fs hosts, add `</dev/null` to stdin-default
  lines, load siblings by `${0%/*}` (§5), spell a helper as a `.`-sourced file rather than a
  mirrored tree.
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
  (vi) LINT-AT-AUTHORING, DENY-AT-APPLY — never a plan-time warning (human-typed).

### §4.6 rulings taken, and what stays the human's

Taken at conductor tier, ledgered: capabilities are measured, never declared; keyed by
context; engine-owned closed vocabulary, extend-by-new-name; never a plan line, never a
license; the v1 keyword list above is provisional under `rul-strawman-formats-no-compat`.
Human-typed or acked this sitting: §2 whole; `rul-preflight-over-probe-time`;
`mech-standup-is-compiled-on-demand-and-tree-shaped`; `capabilities=` as the marker key. The
human's, open: eval as transport (lean, spike-trial); DREP-degraded licensure; the may-run
fail-direction cell (§4.2); the no-return cell's product story (punted); `fs-durable`
(deferred); the four human-as-debugger items (the Ubiquiti page; the iLO scripting guide; a
fish/csh shell for the marker measurement; population/habit).

## §5 — `$0` under Dorc: the authority spelling

### §5.1 grounding

`$0` is the name a script was invoked by, and `${0%/*}` — the idiom for "the directory I live
in", the world's standard for loading a sibling — behaves differently under each spelling:

```sh
1  sh /home/a/ops/webhost.sh      # $0 absolute;  ${0%/*} = /home/a/ops   (never consults the cwd)
2  sh ./webhost.sh                # $0 relative;  ${0%/*} = .             (resolves against the cwd at that line)
3  sh webhost.sh                  # $0 bare word; ${0%/*} = webhost.sh    (dead: webhost.sh/x is fatal)
4  sh -s <webhost.sh              # $0 = sh;      ${0%/*} = sh            (dead the same way; no file exists)
```

Line 4 was Dorc's delivery before §2. The load plane's law, all acked in `30P`: Dorc never
reads `$0` from a shell. In analysis `$0` is modelled symbolically from the book path the
admin typed, with two live spellings, slash-bearing and bare-word-with-cwd. A `. "${0%/*}/x"`
is EXACT when every live spelling resolves to one snapshot file, and EXACT is the sole source
of authority: bindings below the line, vouches, shipping. A spelling under which the load is
fatal is dead, not unsound. Dorc invokes what it ships in a spelling the analysis modelled as
live (`30P:rul-dorc-invokes-in-a-modelled-live-spelling`), and mirrors an EXACT-but-not-
explicit dependency at its authored relative path so the author's verbatim line lands.
`30I` §3.1 keeps three coordinates distinct by representation: the controller load cwd, the
per-target execution cwd, the artifact location; `30I` §7.4 begins multipart execution in
the artifact directory and §7.6 forbids changing cwd afterwards to find dependencies.

### §5.2 the ruling

**`rul-dollar-zero-authority-spelling`** [ACKED 2026-09-03; answers `tc-dollar-zero-is-script-anchored`
yes] — amends `30P:model-symbolic-dollar-zero`, which now carries the text: for deciding
EXACT, the live spelling is Dorc's OWN invocation, absolute, into the scratch root
(`route-file-backed`); the as-given spellings stay modelled and feed only the existing
off-ramp lint ("dies under `sh book.sh`"), never authority. Corollaries:

- `${0%/*}` is cwd-immune under Dorc by sh semantics: `$0` is unassignable, a sourced file
  cannot change it, and `${0%/*}` of an absolute path never consults the cwd. So a `$0`-headed
  load below a blind act is EXACT, ships, and binds. This is the capture named on
  `FORFEITS:forfeit-shell-parity-immunity-model`; that row now lists it as designed.
- The invariant that makes a mirrored `${0%/*}/x` land is RELATIVE POSITION: the plan file
  sits at the same position relative to its mirrored dependencies as the book did to its own,
  under the load cwd. The prefix decides only cwd-immunity.
- The cwd is the scratch root and Dorc never changes it afterwards (`30I` §7.4/§7.6). A
  `cd` whose operand is `$0`-headed rides the same `$0` evaluation the `.` operand does, so
  `cd "${0%/*}"` re-establishes a known cwd below a blind act (a build note for the lane).
- The in-memory route keeps `$0` as `sh`; there an EXACT-via-`$0` load is pasted inline at the
  `.` line or the form refuses by name (`30P` unchanged). The EXACT decision itself is made on
  the controller under Dorc's spelling and never depends on the host route; only the emission
  mechanism differs (mirror or paste), and the definitions land identically.
- `both-worlds-are-carried` (the human's framing, agreed): the analysis is a lattice over
  the world where Dorc runs the book and the world where the author off-ramps. Dorc's
  spelling feeds authority; the as-given spellings feed the off-ramp lint. No claim survives
  the off-ramp — the plan's dispositions die with the run, the generated plan is
  self-sufficient sh — so nothing Dorc claims can become untrue later. The bytes are
  untouched; the `${0%/*}` idiom is the one the world already uses, so nothing is taught.
- The honest residue, stated once: this is the one place the analysis models Dorc's
  invocation rather than the author's possible ones, so a guard can appear in a plan that the
  author's own `sh ./webhost.sh` after a `cd`-ing profile would not have earned — because
  under that invocation the load dies. What Dorc masks is a fragility the book already had
  under one of its invocations; the same masking already happens when Dorc cd's into the
  artifact directory. Two lints proposed for it were DROPPED (human: meaningless here). The
  one-sentence description of `route-file-backed` is the whole disclosure.
- Implied by "a tempdir", accepted without machinery: relative writes (`>./out.txt`) land in
  the scratch and vanish at cleanup; `$0` is ephemeral, so a book that persists it captures a
  dead path; the prefix of `$0` is absolute. Horizoned, not designed: a read-only cwd as
  defence in depth against unanalysable dynamic code.
- Retracted: the worry that probe and apply must share a route or the blessed-lift three-way
  check (`30P:mech-two-standups`) sees two `$0` values. That check runs the tool on modelled
  input shapes; the shell's actual `$0` is not an input to it.

### §5.3 the flagship red, corrected

**`fnd-blind-act-fixture-is-guard-at-most`** [ACKED] — `load30-point-havoc-and-script-relative`
is three lines: a blind act (an env-read `.`), then `. "${0%/*}/hork.dorc.sh"`, then
`hork tune web`. Under this ruling the package load becomes EXACT and ships, so the verdict
body is live below the havoc and the site can GUARD. It can never ELIDE: the blind act ran
commands and is a total wall (`30P:law-no-unsoundness-below-a-blind-act`, the human's own
words: elision below a blind load is out regardless). The fixture's empty-run-set expectation
predates that law and is re-read by the lane that builds this; havoc is an algebraic fact
about analysis, never a denial (human-typed), and the honest model claims exactly what sh
guarantees, no more.

## §6 — the withhold framing, and the unresolvable-load refusal

**`rul-dorc-acts-are-withhold-shaped`** [TYPED 2026-09-03, hard ack; a refined framing of
`KNOBS:kFAIL`] — any Dorc act that would not PROVABLY be caused by the author's code executing
naturally, under a completely sound, settled, zero-variance analysis (Must across all paths),
is `kFAIL-withhold`. Only explicitly-authored, unmodified lines that Dorc can Must-conclude
would have run under plain sh are `kFAIL-perform`. In the medical field's words: first, do no
harm. Consequences already law, now with their receipt: shipping a file is a Dorc act on the
world and takes the withhold direction when unproven; running the author's line takes the
perform direction; analyzer uncertainty IS uncertainty of intent — anything the shell could
have done could have been something the author intended, and the engine cannot judge what
`. /etc/odd-thing` did (refag: no world-referent may be weighed).

**`rul-unresolvable-book-load-refuses`** [ACKED 2026-09-03; closes
`30Q:ask-ship-explicit-targets-below-a-clobber` as NO-SHIP] — the cell:

```sh
1  . /etc/odd-thing               # blind act: every binding below is ⊤, cwd included
2  . ./helpers.sh                 # explicit, relative; resolves against a cwd Dorc cannot prove
3  widget frob                    # its oracle lives in helpers.sh
```

An unresolvable BOOK-CUSTODY load — a computed operand, or a relative operand below a cwd
clobber or any blind act — refuses the plan before network, under ONE code, naming the load
line, the clobbering line, and the two remedies: `. "${0%/*}/helpers.sh"`, or one
`cd "${0%/*}"` after the havoc. Never a warning; never ship-nothing-and-die (an approved
plan mutating above the load and dying there, on the network timescale, is the worst cell).
Grounds: the withhold receipt above; route-uniformity (the in-memory route has nowhere to
mirror and pasting is forbidden under ¬EXACT, so the outcome must not depend on which route
the host got); and reject-but-stay-agile (`30P:rul-floor-valid-text-never-parse-fails`
already starts the computed `.` as a pre-network refusal with softening reserved as a UX
change — this is the same unresolvable-load class with one difference, Dorc happens to hold
a candidate file). The transmission harm, generalised from the human's example: a same-named
book-custody file from elsewhere in the estate, carrying whatever it carries, sent to a host
it was never meant for, on the strength of a resolution the analysis explicitly declined to
vouch. Mirroring is placement, not selection — the author's `.` stays sovereign — and that
was the argument for shipping; it loses to the receipt, because the file leaving the
controller is the act, not what the host does with it.

- `res-dorc-lang-mirroring-without-authority-reserved` — the human reserves the right to
  mirror a validated dorc-lang target below a clobber without authority (pure, non-mutative,
  no secrets by contract, probably from a public repository anyway). RESERVED, unbuilt: it
  is harmless and also useless (no authority attaches; the site runs blind), route-dependent,
  and a placement-seat toggle later. Never paste or re-say under ¬EXACT, for any custody.
- `gloss-floor-unevenness-has-trumps` [human 2026-09-03] — `30P:rul-floor-is-uneven-across-forms`
  binds emission forms and is trumped by `kFAIL-withhold` and by secrets-management; it
  cannot satisfy every corner of "we'll make everything work" and deserves the qualification.
  This refusal is the first construct whose cost to the author is support rather than a form;
  payable because the remedy is best practice and works under every off-ramp invocation.
- The population cost, recorded honestly: the refusal cannot tell a `cd`-ing profile from a
  harmless one and fires on every blind act followed by a relative load; ~SUSPECT the shape
  is common. One line fixes it.
- Remedies must be engine-recognised or the loop is not tight: a plain variable captured
  before the havoc is ⊤ below it like every other binding; `readonly` would genuinely
  survive a sourced file by sh semantics but is unmodelled — a candidate capture on the
  parity-immunity row, not a hint. Hint machinery itself is deferred with all lints/hints.
- Supersedes `30Q` §3's sentence that cwd-⊤ never costs mirroring; the as-built
  (`cli/CLAUDE.md`: nothing shipped under ¬EXACT) stands.

## §7 — language holes: the verdict

Nothing found forces the CORE language. The one channel-forced analyzer carve already exists
(`310`'s "simple words are identity across the remote re-parse; else decline" — the ssh argv
re-parse, `28Q` §3; the round's Q16 evidence is its substrate proof, and simple words survive
every login shell found). An unwritable host forcing single-stream is a value-loss the
off-ramp shares (`FORFEITS:forfeit-plain-sh-inclusion-analysis` stands). The here-document
hazard is real, narrow, and aid-plane. stdin in shipped bodies is already fenced
(`26Lb:rul-interactivity-is-local-books`; the predict tracer treats `read` as run), and §2
makes it true by mechanism. What fell out instead: the capability vocabulary and census (§4),
the `$0` authority spelling (§5), the withhold framing and the load refusal (§6), one spelling
rule (`sh "$f"`, never exec — `noexec /tmp` blocks `execve`, not `open`), one contract
sentence for oracle authors (the DREP sink is write-only, append-only, and may be a device —
never read it back; pinned, human), and DEFERRED lint candidates: `$0` read in a single-stream
artifact (the `30P` open hint); stdin-default commands with no redirect (silent no-op);
here-documents under bash-as-sh < 5.1; judo's close-fd-0 as an opt-in strict-tier lint
(never core; human-typed; safe to recommend only now that delivery no longer reads the
artifact from fd 0). Banked for a joint turn, probable nack given the meta-orchestration
focus: how a book could spell, in sh alone, that a host is Windows and needs `cmd` first.

## §8 — the weirdness pareto frontier (first cut; brief; `turn04`/`turn05` tables hold the evidence)

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
  a legacy unit is dogfood-available.
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

## §9 — prior-art digest that binds direction (one line each; `turn06` holds the table)

The incumbents' history says: the reliable pivot direction is FEWER ASSUMPTIONS about the
target; the memory route is common in argument form and blocked on stdin only by the PTY —
which Dorc's welds already exclude (`-T` required; the transport never escalates; entry is
`sudo -n`, failing rather than prompting under `requiretty`, at the price of context entry
on such hosts, itself a capability); non-Bourne login shells were priced and mostly refused
(the constant wrapper is exempt from Fabric's escaping objection because it carries no user
bytes); the copy-then-exec HANDSHAKE (`mkdir && echo`) is a documented intermittent failure
surface in Ansible, arguing for single-exchange host-side selection; the incumbents carry a
file lane because their payloads include arbitrary files and binaries ("pipelining does not
work for modules involving file transfer"), which Dorc's sh-text tree does not; Rundeck
formalizes copy-then-exec as two pluggable services (File Copier + Node Executor), our
entry-recipe versus interpreter split, and its worked example is an HTTP-upload copier (the
charter's HTTPS seed is a documented DELIVERY pattern, not a results pattern); Salt's `RSTR`
marker protocol documents its own undefined cell, which Dorc's marker-absent ⇒ Unknown rule
covers.

## §10 — open, deferred, parked

- **`open-census-needs-value-plane`** — NEXT: whether the requirement census needs constant
  propagation or a lattice seat, given that book code admits dynamic forms; the seam the
  human pinned (§4.2). Also open beside it, tabled at the human's direction: whether Dorc's
  responsibility ends where it starts moving, shipping, and mutating the author's shippables.
- `park-drep-remote-home-explainer` — `dec-drep-remote-home` still gates
  `310:unit-host-index-and-entry`; what the report sink is, why an entered or remote world
  loses its records today, and the candidate remote home (the entered context's own scratch,
  drained by the entry scaffold, re-framed by the outer records lane) are owed as an
  explainer before any ruling.
- `park-oracles-knowing-stdin-stdout` — unruled: whether a predict body may speak for its
  tool's stdin consumption (the `uci batch` class) under refag; the census can derive stdin
  consumption only for `read` and for modelled commands whose predict body reads stdin.
- `park-re-parse-carve-explainer` — what `28Q:pin-ssh-entry-shape` carves and why the round's
  Q16 evidence grounds it.
- `front-embedding-contracts` — NEEDS INVESTIGATION (human-ack; possibly a successor with
  the human): what cloud-init `runcmd`, init containers, Ansible `script`, Packer
  provisioners, and systemd `ExecStartPre` promise a foreign sh chunk and what a Dorc-wrapped
  chunk offers back; the Dorc-as-child story. Partly in `plans/24R`/`26K`.
- `front-request-response-mechanics` — mildly less: exact caps, encodings, truncation sides,
  exit and timeout semantics of SSM, Azure run-command, Proxmox guest-exec, Bottlerocket's
  SSM path, and how Ansible's and Packer's connectors drive them.
- `lane-interpreter-and-login-shell-measurement` — extend the real-binary battery
  (`.claude/research/kwhichsh-gcd/`) to busybox ash, mksh, FreeBSD sh, bash in POSIX mode,
  and to csh/tcsh/fish/zsh as login shells; exercise the wrapper, `eval` versus `sh -c`
  versus `sh file`, and `cksum`. A measurement errand, not design; the only check on the
  language floor's presumed-superset gap. Deferred on the constrained machine.
- `someday-capability-coverage-table` — per-target coverage of each keyword; expensive,
  empirical, after a robust capability system exists (human).
- `someday-no-fs-structural-test` — whether sshd + sh + PAM stand up a session with NO
  writable filesystem (sshd itself does not need one; the rest unsourced).
- Also open: the no-return cell's product story; `fs-durable` and log retention on
  non-durable hosts; a reverse forward on the existing ssh connection as a no-new-tunnel
  return path (unsourced lead); the four human-as-debugger items (§4.6); the Windows-in-sh
  spelling (banked; probable nack); readonly immunity as a parity capture; the shebang as
  dialect declaration (tabled, §3).

## §11 — register state

Touched by this sitting (pointers only, content lives here or in `30P`): `plans/26K` §0c
(closed); `ROADMAP.md` (the r31 gate text; an owed row for the capability system);
`plans/310` (lane 2 gains the delivery-shape unit; the `$0` and load-refusal rulings; lane
3's gate narrows to `dec-drep-remote-home`; the reserved seat at `unit-context-slot-product`);
`plans/30P` (`model-symbolic-dollar-zero` amended in place per §5.2; the "nothing shipped on
a guess" clause carries §6's ruling; the floor-unevenness gloss; the `$0` hint deferred);
`FORFEITS:forfeit-shell-parity-immunity-model` (the `$0` capture designed; readonly added);
`KNOBS:kBOOT` and `kFAIL` (one pointer each); `notes/30Q` §3 and §5c (superseded
annotations); `Research/README.md`; `LIVING_STATUS.md` standing truths; the round charter.

Owed to the lane that builds, not touched here: `plans/260` §5 (the invocation line and driver 2);
`spike/CLAUDE.md` (one delivery-shape bullet under host evidence; the DREP sink contract
sentence; the `$0` note); `cli/CLAUDE.md` (the ¬EXACT ship rule now carries its ruling);
`ANALYZER-NEEDS` (two rows: required-host-capabilities census; measured-host-capabilities
supply, keyed by context); `AID-NEEDS` (`aid-capability-denial-remedy`; the deferred lints);
the `USER_STORY`-adjacent authoring doc (the DREP write-only sentence). The 117 source
grades stay `graded-by: subagent` until a load-bearing one is cited in a stamped plan.
