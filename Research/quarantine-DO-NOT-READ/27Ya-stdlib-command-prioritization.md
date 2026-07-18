<!-- 27Yb-DROP-START -->
# 27Ya — stdlib command-prioritization scan (round 27)

> **QUARANTINED — memetic-hazard.** AI-authored (Fable-class conductor, r27, 2026-07-17),
> at human direction, as INPUT to a much-later stdlib-implementation session (not in this
> author's context). This document reasons openly about the Anthropic cyber-safety gate and
> about security-sensitive ops commands; that is *why* it lives under quarantine and must be
> read only by the human. A safe subset (fable-risk == 1 rows only) is mechanically extracted
> to `Research/notes/27Yb-*` for a Fable-class checker.
>
> Authority: root docs + human-typed rulings outrank this. This is a prioritization SCAN, not
> a measurement — it ranks *what to model first*, it does not measure elision rates.

## What this is

A prioritized survey of the shell commands a Dorc "stdlib" (the ~40-50 bootstrap oracles;
`KNOBS:effort-allocation`, `270:block-stdlib`) should model, each graded on four axes. The
scan feeds the authoring phase; the human asked that ranking be dominated NOT by raw
project-importance but by **complexity-in-our-model** first, **infosec/fable-risk** second.

Each command is graded 1..5 on four columns (5 = most of the named quality):

- **C1 complexity-to-model** — how much of Dorc's subtle machinery correct modeling exercises.
  The two probing questions: *will it ever elide at all?* and *could it ever license
  post-divergence survival under `--risk-faultless-skips`?* (né `--allow-faultless-skips` in
  the human's prompt — the shipped flag is `--risk-faultless-skips`.)
- **C2 fable-risk** — how dangerous it is to feed this command's ops-corner as a
  problem-statement to an infosec-gated Fable-class conductor. **This column drives the 27Yb
  filter and drives whether an oracle can be authored by a gated model at all.**
- **C3 prevalence** — frequency/importance in real ops shell (lightly bumped for
  early-in-setup / early-in-defensive-script position; frequency mostly rules).
- **C4 punt-independence** — ship-now value. LOW when the command's realistic value depends
  on a PUNTED feature (chiefly the value-plane/output-capture fold-in, deferred to r26);
  HIGH when its value lands fully on machinery shipping this round. (Anchor: `hostname` = 1.)
<!-- 27Yb-DROP-END -->

<!-- 27Yb-DROP-START -->
## §1 — The Fable security-gate (calibration basis for C2)

Grounding (Kagi + Anthropic cyber-safeguards support article + the Fable/Mythos system-card
framing, retrieved 2026-07-17):

- **Fable 5 = the generally-available model with the cyber/bio classifier guards switched ON;
  Mythos 5 = the same underlying model without them** (approved orgs only). The classifier
  flags "cybersecurity or biology topics" and — by Anthropic's own admission and abundant
  field reports — **flags safe/normal content too** (high false-positive rate on legitimate
  defensive and analytical work).
- The gate blocks **two tiers**:
  1. **Prohibited use** — inseparable-from-harm offensive ops: mass data exfiltration,
     ransomware / malware development, C2 frameworks, anti-forensics / log-tampering. *Never*
     liftable (no CVP).
  2. **High-risk dual-use** — legitimate-but-dangerous: **vulnerability exploitation,
     offensive-security tooling, privilege escalation, auth backdooring, firewall/IDS evasion,
     sandbox escape, credential/secret access.** Blocked by default; liftable only via the
     Cyber Verification Program (org-scoped, retention-required).
- **Operative reading for this scan (human-directed, matches field experience): the classifier
  fires on the TOPIC, not on intent.** Dorc's use is *always* defensive (modeling a command's
  state-effects for a static analyzer), but that does not lower the score, because the gate
  cannot see the intent — it sees "reasoning about sudo / PAM / iptables / firewall / ssh-keys"
  and drops the guillotine. **Bias toward dangerous whenever reasoning about the command could
  even plausibly trigger security reasoning.**

Consequences that outlive this doc:

- A downstream Fable-class conductor asked to author the `sudo` / `sshd` / `pam` / `iptables`
  oracle will likely be interrupted mid-task or refuse. Such oracles need a **non-gated model
  (Mythos/Opus-tier) or very careful framing** — flag them at dispatch. C2 is therefore not
  just a ranking axis; it is a **build-routing** axis.
- The 27Yb safe-subset exists so the human can have a Fable-class agent check the *low-risk*
  rows without the doc itself tripping the gate.

## §2 — What ships this round vs what is punted (calibration basis for C4)

"This round" = block-rebuild (in flight) → block-context → block-stdlib, landing over the near
term. C4 grades ship-now value against this split.

**SHIPPING (value counts fully):**
- rc-based convergence (`is_converged`), the elide/guard verdict machinery.
- disturbance footprints (`disturbs`), the survival tier + `--risk-faultless-skips`.
- kind machinery: `resolve` (aliasing), `disturbance_reaches_only` (reach).
- the wrapper/context machinery for **privilege + environment** (sudo/su/env) — block-context /
  context-entry probing (`plans/27C`). *(CONFIRMED live by the human, this session — the wrapper
  C4 grades stand as authored.)*

**PUNTED (value depending on these scores LOW on C4):**
- **value-plane / output-capture fold-in** ("pass 2", deferred to r26; `26B`) — threading a
  command's captured STDOUT into downstream decisions. **The dominant C4 downgrade.** Anything
  whose only value is its captured output (`hostname`, `uname`, `id`, `getent`-as-value,
  `command -v`-as-value) scores C4 = 1.
- multi-host / read-concurrency (r26); cross-run state persistence (`kSTATE`, parked).
- **MH2 version-window reasoning** (deferred) — "installed but upgrade-pending = converged?".
  Docks version-sensitive package convergence.
- **full fs-view / mount-table truth** (Hard cell, unowned) — docks `mount`/`chroot`/bind ops.
- **netns network-context** (design-pending; `271:rul-networking-unpunt`) — docks
  `nsenter`/`unshare`/netns-scoped network commands.
- guard-insertion under ELEVATED wrappers (undesigned); doas/become privilege prior-art
  (hard-deferred behind a human ack).

## §3 — The complexity model (calibration basis for C1)

Dorc models a command through oracle functions in sh: `__is_converged()` (read-only
convergence check; licenses elision), `__predict()` (per-channel output/effect model),
`__disturbs()` (mutated-cell footprint), and per-kind `__resolve()` / `__disturbance_reaches_only()`.
State is `(kind, entity, selector)` cells (e.g. `sm.dorc.Service:nginx#enabled`).

Complexity tiers (ascending):

1. **No convergence sense** — pure side-effect or pure output (`echo`, `sleep`, `rm`, `printf`).
   Trivially walls or runs; nothing subtle to get right. *Low C1 — but also can't elide.*
2. **Single-cell converge** — one clean read-back check (`mkdir -p`, `ln -sf`, `chmod`,
   `dpkg -s`). Easy oracle; elides cleanly.
3. **Multi-cell + argparse** — several verbs/selectors, distinct cells per verb (`systemctl`
   enable/start → `#enabled`/`#active`; `useradd`). Moderate.
4. **Broad disturbance + kind-ownership** — wide footprints, provides/alias `resolve`,
   package→files `reaches` (`apt-get`, package managers, `mount`). The survival machinery and
   kind-owner functions live here.
5. **Context-transiting / wrapper** — runs *another* command in a modified privilege / env /
   fs-root / namespace context (`sudo`, `su`, `chroot`, `nsenter`, `xargs`, `find -exec`).
   Dorc's subtlest corner: peel, map the transition, measure the inner command in-context.

The two probing questions decide the high end: a command that **can never elide** (no
convergence sense) caps low on the "will it pay off" reading even if its plumbing is fussy;
a command that can license **post-divergence survival** needs a clean bounded footprint, which
adds real complexity.

## §4 — Methodology

- **Frequency (C3):** the H2SaLS census (one real 696-line Debian hardening book —
  security-hardening-biased, over-weights firewall/auth/crypto, under-weights docker/systemd/
  generic-deploy) + a mechanical multi-repo first-token frequency sweep across diverse
  provisioning/dotfiles shell repos (thoughtbot/laptop, mathiasbynens/dotfiles, nvm, bash-it,
  …) + training-data prevalence priors. **Honest limitation:** this is a coarse prevalence
  signal, not a measured corpus study; C3 is the softest column and the lightest-weighted.
  (Sweep result: 386 shell files across 6 repos, but bash-it dominates ~89%, so it over-weights
  shell-framework/text tokens — it CONFIRMS the high-frequency core rather than reordering it, and
  surfaced `diff`/`defaults`/`chsh` + a sort/head/tail/wc/uniq value-tool cluster, folded in.)
- **Grading:** 8 domain-batched Sonnet subagents graded the four columns against a shared,
  anchored rubric (self-contained; no repo reading), then the conductor **re-calibrated
  cross-batch** (subagents drift on a 1..5 scale) against the anchors in §1-§3 and folded in
  the frequency data. Grades below are the conductor's calibrated values, not raw agent output.
- **Quarantine ownership:** corpora consolidated under `Research/quarantine-DO-NOT-READ/`
  (the notes-quarantine moved up a level; `corpora` + `Vendor` moved in) this session.
<!-- 27Yb-DROP-END -->

<!-- 27Yb-KEEP-HEADER-START -->
## §5 — The prioritization table

Columns are 1..5, 5 = most of the named quality. **C1** complexity-to-model · **C2** fable-risk
(topical infosec-adjacency; drives build-routing) · **C3** prevalence · **C4** punt-independence
(ship-now value; low = value gated behind punted work). **elide?** = yes / guard-only / no.

> The safe subset (`27Yb`) contains the **C2 <= 2** rows of this table (fable-risk 2 and below).
> Rows with C2 >= 3 are, by construction, absent from 27Yb.
<!-- 27Yb-KEEP-HEADER-END -->

<!-- 27Yb-TABLE-START -->
| command | domain | C1 | C2 | C3 | C4 | elide? | justification |
|---|---|---|---|---|---|---|---|
| apt-get | package | 5 | 2 | 5 | 5 | yes/no-update | broad index+package+files disturbance, provides-`resolve`; install/remove elide via dpkg-status, `update` is time-based (no convergence). |
| apt | package | 5 | 2 | 4 | 5 | yes/no-update | same dpkg machinery as apt-get; scripts prefer apt-get (apt CLI self-declared unstable), slightly lower prevalence. |
| dpkg | package | 4 | 2 | 3 | 5 | yes | named-`.deb` only (no repo graph) but still needs package->files disturbance; `dpkg -s` is the canonical rc-guard. |
| dpkg-query | package | 2 | 2 | 3 | 1 | no | pure read; dominant `-W -f` value-extraction = punted value-plane. |
| dpkg-statoverride | package | 3 | 3 | 1 | 5 | yes | narrow perm-DB cell (`--list` vs desired), distinct from live-fs (not the fs-view punt); perm-override = mild security valence. |
| add-apt-repository | package | 4 | 3 | 3 | 4 | yes | sources-entry + implicit trust-key import (2-cell); `ppa:` needs resolve (possibly network I/O — open question); key-import = trust valence. |
| apt-key | package | 3 | 5 | 2 | 5 | yes | deprecated but copy-pasted; fingerprint-set-membership check; GPG/trust-store management = C2-top band. |
| update-alternatives | package | 3 | 1 | 2 | 4 | yes | auto/manual+priority state-machine over one symlink-target cell; no infosec adjacency. |
| snap | package | 4 | 2 | 2 | 3 | yes/guard-refresh | separate Package kind + own resolve; channel/revision tracking = MH2-adjacent (docks C4). |
| dnf | package | 5 | 2 | 4 | 5 | yes/no-checkupd | structurally == apt-get; first-class `dnf provides` resolve; RPM-world default (Fedora/RHEL8+). |
| yum | package | 5 | 2 | 3 | 5 | yes/no-checkupd | == dnf (legacy shim on modern RHEL, primary on RHEL7/AL2); near-zero marginal cost once dnf modeled. |
| rpm | package | 4 | 2 | 4 | 5 | yes | named-rpm (no dep graph); dual kind (Package via -i/-q/-e, TrustedKey via --import); `rpm -q` = rc-guard. |
| brew | package | 4 | 1 | 3 | 5 | yes | multi-kind (tap + formula/cask + `brew services`); no sudo, no infosec adjacency. |
| pip3 | package | 5 | 1 | 4 | 2 | guard-only | version-RANGE default idiom = MH2 punt (docks C4 hard); venv/site-packages scoping wrinkle. |
| npm | package | 5 | 1 | 5 | 2 | guard/yes-ci/no-run | `install`=semver-range MH2 punt; `npm ci`=exact-lockfile (ships fully); `npm run`=arbitrary, no convergence. |
| gem | package | 3 | 1 | 2 | 4 | yes | simpler (exact-version pin; ranges live in `bundle`); Ruby ecosystem prevalence declined. |
| flatpak | package | 4 | 3 | 1 | 3 | yes/guard-update | separate kind + branch/commit tracking (version-adjacent); `flatpak override` = sandbox-escape-adjacent; niche in server ops. |
| systemctl | service | 4 | 2 | 5 | 5 | yes | distinct `#enabled`/`#active` cells; broad verb argparse; daemon-reload/restart have no convergence sense. |
| service | service | 3 | 2 | 4 | 5 | yes | single `#active`-ish cell via status; narrower than systemctl (no enable on Debian). |
| journalctl | service | 1 | 2 | 3 | 1 | no | pure read-only log query; value = captured output (punted); never an elision target. |
| update-rc.d | service | 3 | 2 | 2 | 4 | yes | Debian legacy `#enabled` via rcN.d symlink presence; enable-slice only. |
| sysctl | service | 3 | 3 | 3 | 5 | yes | single-key runtime-value cell (`/proc/sys` compare); runtime-vs-persisted duality; hardening knobs raise C2. |
| systemd-tmpfiles | service | 3 | 2 | 2 | 5 | yes | per-path existence/mode/owner cells from tmpfiles.d; rarely author-invoked (boot/postinst). |
| systemd-sysusers | service | 3 | 4 | 2 | 5 | yes | declarative user/group existence cells (useradd-like); account-creation = auth band. |
| loginctl | service | 2 | 2 | 2 | 4 | guard-only | `enable-linger` = clean boolean cell; session actions have no convergence (heterogeneous). |
| timedatectl | service | 2 | 3 | 3 | 5 | yes | set-timezone/set-ntp single-value cells; C2 up for clock-manipulation anti-forensics adjacency. |
| hostnamectl | service | 2 | 3 | 3 | 4 | yes | `set-hostname` single string-cell (genuine mutation, ships now — unlike bare `hostname` value-capture). |
| invoke-rc.d | service | 3 | 2 | 2 | 4 | guard-only | same `#active` cell as service; `policy-rc.d` hook layer -> guard-only; mostly maintainer-scripts. |
| ufw | firewall/net | 4 | 4 | 4 | 5 | yes | rule-presence via `ufw status`; nft/iptables translation adds argparse; ships now. |
| iptables | firewall/net | 5 | 5 | 4 | 3 | guard-only | rule-ORDERING undermines confident elision even with `-C`; rules netns-scoped (docks C4). |
| ip6tables | firewall/net | 5 | 5 | 3 | 3 | guard-only | 1:1 with iptables; fewer scripts do IPv6-specific rules. |
| nftables | firewall/net | 5 | 5 | 3 | 3 | guard-only/yes-file | richer type system; incremental `add` shares ordering-subtlety; whole-file `nft -f` ~ content-diff cell. |
| firewall-cmd | firewall/net | 4 | 4 | 3 | 4 | yes | `--query-*` first-class rc-convergence (cleanest here); runtime-vs-`--permanent` dual-cell, trackable now. |
| ip | firewall/net | 5 | 2 | 4 | 3 | guard-only | multi-kind (addr/link/route/rule/netns); `ip netns exec`=context-transiter (netns punted); `show`=value-capture. SPLIT candidate. |
| route | firewall/net | 3 | 2 | 2 | 3 | yes | legacy add/del; convergence via `route -n` parse; declining (superseded by `ip route`). |
| netplan | firewall/net | 4 | 2 | 2 | 2 | guard-only | whole-file YAML apply, broad disturbance; `netplan apply` rc is NOT a convergence signal; Ubuntu-only. |
| resolvectl | firewall/net | 3 | 2 | 2 | 3 | guard-only | per-link DNS cell via status; runtime settings ephemeral (networkd re-applies); systemd-resolved-specific. |
| ss | firewall/net | 1 | 2 | 3 | 1 | no | pure diagnostic read; value = captured output (punted). |
| netstat | firewall/net | 1 | 2 | 3 | 1 | no | == ss (legacy net-tools); still prevalent in scripts/docs. |
| ping | firewall/net | 1 | 2 | 4 | 1 | no | reachability is point-in-time, non-cacheable (true wall); rc-gate or parsed-stats, no Dorc value either way. |
| cp | filesystem | 3 | 1 | 5 | 5 | yes | content-match cell (src bytes == dest); archetypal ubiquitous elide. |
| mv | filesystem | 2 | 1 | 3 | 4 | no | non-idempotent (errors on absent source) so no "already achieved" state; clean rename footprint useful for survival. |
| rm | filesystem | 2 | 1 | 4 | 4 | guard-only | `rm -f` absent-path = no-op (mkdir -p-like) but bare `rm` errors -> flag-conditional elision. |
| mkdir | filesystem | 2 | 1 | 5 | 5 | yes | dir-exists cell under `-p`; textbook clean elide. |
| rmdir | filesystem | 2 | 1 | 2 | 4 | no | no `-p`-equivalent force; errors on missing/non-empty; rare (rm -rf supersedes). |
| ln | filesystem | 3 | 1 | 4 | 5 | yes | symlink-target cell under `-sf`; entity-cell compare, no cross-kind reach. |
| touch | filesystem | 2 | 1 | 3 | 5 | yes | existence-cell (sentinel idiom); mtime-refresh caveat needs a separate cell if a script depends on it. |
| cat | filesystem | 2 | 1 | 5 | 2 | no/guard | as heredoc/redirect config-writer -> content-cell (like tee); plain `cat file` reader = value-capture (punted); ubiquitous. |
| chmod | filesystem | 2 | 2 | 5 | 5 | yes | mode-bitmask cell; symbolic-mode resolution wrinkle; permissions = access-control valence. |
| chown | filesystem | 2 | 2 | 3 | 5 | yes | owner-cell + uid/gid name-resolution; ownership mildly security-flavored. |
| chgrp | filesystem | 2 | 2 | 2 | 5 | yes | group-cell subset of chown; rarer (chown user:group subsumes). |
| install | filesystem | 3 | 2 | 3 | 5 | yes | compound cell (content+mode+owner+dir via -D/-t/-m/-o/-g); real argparse; File kind. |
| rsync | filesystem | 4 | 2 | 3 | 3 | guard-only | full tree-diff convergence + `--delete` broad disturbance = CHECK-TAX; bulk-transfer mild adjacency. |
| tar | filesystem | 4 | 2 | 4 | 3 | no | create/extract = output/side-effect; extract footprint unbounded until members enumerated (`-t`); path-traversal adjacency. |
| mount | filesystem | 4 | 4 | 3 | 2 | guard-only | cross-kind (device/fstype/mountpoint/options) vs live mount-table (fs-view PUNTED); nosuid/nodev = security options. |
| umount | filesystem | 3 | 2 | 2 | 2 | guard-only | mirror of mount but needs the same punted mount-table truth; narrower argparse. |
| chattr | filesystem | 3 | 4 | 1 | 4 | yes | attribute-bitmask via lsattr; immutable/append-only bits = security-adjacent; very rare. |
| tee | filesystem | 1 | 1 | 2 | 2 | no | stdin-stream writer; content from upstream pipeline (no static convergence w/o running it). |
| sync | filesystem | 1 | 1 | 1 | 2 | no | global buffer-flush barrier; no Dorc cell (structurally thin). |
| mktemp | filesystem | 1 | 2 | 3 | 1 | no | value-capture (`X=$(mktemp)`); never converges (uniqueness); CWE-377 mild adjacency. |
| useradd | auth/security | 3 | 4 | 4 | 5 | yes | existence/attr via `getent passwd` licenses convergence; bounded User+Group disturbance; account/credential band. |
| adduser | auth/security | 3 | 4 | 3 | 5 | yes | Debian wrapper, same `getent passwd` convergence as useradd; narrower (Debian-only). |
| usermod | auth/security | 4 | 4 | 3 | 4 | yes | per-attribute convergence via id/getent; many flag->cell maps; `-aG sudo/wheel/docker` = privilege-adjacent. |
| groupadd | auth/security | 2 | 4 | 3 | 5 | yes | single existence check (`getent group`); minimal flags; group creation gates privilege (sudo/docker). |
| gpasswd | auth/security | 3 | 4 | 2 | 4 | yes | membership add/remove converges via getent group; group-password submode has no readback. |
| passwd | auth/security | 2 | 4 | 3 | 4 | guard-only | hash-set has no readback (never elides); `-l`/`-u`/`-S` lock-state converges via getent shadow. |
| chpasswd | auth/security | 1 | 4 | 2 | 4 | no | batch hash setter, no readback -> never elides; trivial footprint (password cell) ships now. |
| deluser | auth/security | 3 | 4 | 2 | 5 | yes | inverse of useradd; converged if getent shows absent; rarer than creation. |
| chage | auth/security | 3 | 4 | 2 | 5 | yes | password-aging policy readable per-flag via `chage -l`/shadow; compliance/hardening command. |
| visudo | auth/security | 3 | 5 | 2 | 3 | guard-only | editor/syntax-check gate over sudoers content-cell; `-c` check is safe but not classic elision; edit-mode opaque. |
| ssh-keygen | auth/security | 3 | 5 | 3 | 4 | yes | `[ -f key ] ||` file-existence convergence (ships); exact type/bits/comment match needs content-cell (docks). |
| sshd | auth/security | 3 | 5 | 2 | 3 | guard-only | scripted usage dominated by `sshd -t` config-validation gate over sshd_config content-cell (runs every time). |
| pam-auth-update | auth/security | 4 | 5 | 2 | 2 | guard-only | PAM profile state only via parsing generated /etc/pam.d; broad/opaque footprint (every auth path) — survival can't handle. |
| fail2ban-client | auth/security | 3 | 5 | 2 | 3 | guard-only | jail enable/disable queryable via status (convergence); ban/unban is dynamic runtime state, not a book target. |
| unattended-upgrade | auth/security | 4 | 4 | 3 | 1 | no | its whole purpose IS the punted MH2 version-window question; no elision value w/o it + broad cross-package disturbance. |
| semanage | auth/security | 5 | 5 | 2 | 3 | yes | very broad heterogeneous subcommand/kind space (booleans/ports/fcontext/login); per-subdomain convergence exists but only a fraction modelable now. |
| setsebool | auth/security | 2 | 5 | 2 | 5 | yes | single boolean with exact `getsebool` readback — cleanest SELinux convergence; ships now. |
| apparmor_parser | auth/security | 4 | 5 | 1 | 2 | guard-only | loading a compiled profile has no simple readback (needs content-hash vs kernel policy) — punted opaque-content case. |
| aa-enforce | auth/security | 2 | 5 | 1 | 4 | yes | single mode-flip per profile, readable via `aa-status`; close to setsebool's clean boolean convergence. |
| gpg | auth/security | 4 | 5 | 3 | 3 | guard-only | key-existence/import converges via keyring listing (ships); dominant crypto-transforms are output-producers (value-plane punted). |
| openssl | auth/security | 4 | 5 | 3 | 2 | guard-only | vast heterogeneous subcommands; almost no generic convergence beyond a weak file-existence guard; value is captured output (punted). |
| dpkg-reconfigure | auth/security | 4 | 2 | 3 | 2 | no | invokes arbitrary package-specific debconf scripts, no generic readback; broad/opaque disturbance; not itself a security tool. |
| sudo | wrapper | 5 | 5 | 5 | 4 | yes | context-transiting (privilege); transparent — verdict = inner command measured under elevated actor; wrapper ships, elevated-guard corners punted. |
| su | wrapper | 5 | 5 | 3 | 4 | yes | same privilege-context-transition class as sudo; `su -c`/legacy entrypoints, less common in modern scripts. |
| doas | wrapper | 5 | 5 | 2 | 4 | yes | same wrapper machinery as sudo; narrower flags but the hard part is the context-transition itself. |
| runuser | wrapper | 5 | 5 | 2 | 4 | yes | privilege-switch (no-auth variant; init/PAM/DB-init scripts); topic-analogous to su/doas. |
| env | wrapper | 4 | 2 | 3 | 4 | yes | context-transiting but FLAT (KV env overlay, no entity/kind resolution); mild LD_PRELOAD/PATH-hijack dual-use edge. |
| nice | wrapper | 2 | 1 | 2 | 4 | yes | identity-ish wrapper — scheduler priority isn't a tracked cell; verdict = inner command's; no infosec adjacency. |
| ionice | wrapper | 2 | 1 | 2 | 4 | yes | == nice class (I/O scheduling, not a tracked cell); rarer (backup/rsync scripts). |
| nohup | wrapper | 2 | 1 | 3 | 4 | guard-only | identity-ish wrapper; dominant `nohup cmd &` detaches rc from sync availability, so realistic elision rarely fires. |
| timeout | wrapper | 3 | 1 | 3 | 4 | yes | adds a new outcome class (rc 124 = deadline) distinct from inner divergence; not entity/kind heavy. |
| chroot | wrapper | 5 | 4 | 2 | 2 | guard-only | context-transiting (filesystem root); measuring inner command in-root needs fs-view/mount-table truth (PUNTED). |
| nsenter | wrapper | 5 | 4 | 1 | 2 | guard-only | context-transiting (enters existing namespaces); netns truth PUNTED (docks C4, forces guard-only). |
| unshare | wrapper | 5 | 4 | 1 | 2 | guard-only | namespace-context-transition (creates new ns; larger flag space); userns-creation is a real privil(esc history — arguably C2=5). |
| setpriv | wrapper | 5 | 5 | 1 | 4 | yes | most granular privilege tool (uid/gid/caps/no-new-privs/LSM independently) — hardest single privilege wrapper to model. |
| xargs | wrapper | 5 | 1 | 4 | 2 | guard-only | fan-out invocation over a (often dynamic, non-statically-enumerable) argument set; fan-out machinery NOT in this round's wrapper scope. |
| find | wrapper | 5 | 2 | 4 | 2 | guard-only | graded on `-exec`/`-delete` fan-out (dynamic-set, unshipped fan-out); bare read-only `find` is a trivial separate row. SPLIT candidate. |
| flock | wrapper | 3 | 1 | 2 | 1 | guard-only | execution-gating wrapper; lock-acquired/contended is a THIRD outcome class; value = punted concurrency/cross-run reasoning. |
| setsid | wrapper | 2 | 1 | 2 | 4 | guard-only | identity-ish wrapper (new session/pgroup, untracked); dominant `setsid cmd &` detaches rc like nohup. |
| hostname | read/text/proc | 1 | 3 | 3 | 1 | no | pure read; value only via captured stdout (branch/log on FQDN) — punted value-plane; recon adjacency. |
| uname | read/text/proc | 1 | 2 | 4 | 1 | no | pure read (arch/OS/kernel), ubiquitous in bootstrap arch-detection; value = captured stdout (punted); mild fingerprinting. |
| id | read/text/proc | 1 | 3 | 3 | 1 | no | pure read; `$(id -u)` value-capture (its rc is ~always 0, not a guard); privilege-recon adjacency. |
| whoami | read/text/proc | 1 | 3 | 2 | 1 | no | == id but less detailed/common (superseded by `id -un`/`$USER`); value-capture; recon adjacency. |
| getent | read/text/proc | 2 | 4 | 3 | 2 | guard-only | dual-use: `getent passwd u >/dev/null ||` existence-guard (ships) vs `$(getent ... | cut)` value-capture (punted); shadow/passwd = auth band. |
| command -v | read/text/proc | 1 | 2 | 5 | 4 | guard-only | `command -v x >/dev/null ||` existence-guard (ships; Dorc elides the gated install); value-capture variant docks; POSIX-canonical. |
| which | read/text/proc | 1 | 2 | 4 | 3 | guard-only | == command -v shape but non-POSIX/less portable (busybox); varying exit semantics dock guard-reliability. |
| type | read/text/proc | 1 | 2 | 2 | 3 | guard-only | shell-builtin existence check; POSIX documents output as human-oriented (command -v is the scripting form) -> less used. |
| test / [ | read/text/proc | 2 | 1 | 5 | 5 | guard-only | THE canonical POSIX guard primitive — Dorc's is_converged/guard-recognition is built around parsing test-shaped conditions; ships now. |
| grep | read/text/proc | 2 | 1 | 5 | 4 | guard-only | `grep -q` guard-primitive (ships); non-guard filter/extract usage is value-capture (docks); ubiquitous. |
| sed | read/text/proc | 3 | 1 | 5 | 3 | yes | `sed -i` mutates a file; "already-replaced" is a genuine content-cell convergence via companion read; per-call authoring effort docks C4. |
| awk | read/text/proc | 1 | 1 | 3 | 1 | no | dominant portable use is field/value EXTRACTION, no convergence (GNU `awk -i inplace` would be sed-like but non-dominant). |
| cut | read/text/proc | 1 | 1 | 2 | 1 | no | pure field-splitter, no mutation, no rc to guard on — unambiguous value-producer (punted). |
| tr | read/text/proc | 1 | 1 | 2 | 1 | no | pure stdin->stdout transliteration, almost always inside `$(...|tr...)` capture chains (punted). |
| echo | read/text/proc | 1 | 1 | 4 | 1 | no | pure side-effect/output; `echo x >> file` append is a content-cell edge (see notes) but bucketed no-convergence. |
| printf | read/text/proc | 1 | 1 | 5 | 1 | no | == echo bucket, POSIX-preferred; census-top command (15) — highest raw prevalence in the read/text batch. |
| sleep | read/text/proc | 1 | 1 | 3 | 1 | no | temporal delay — no "already-slept" state (convergence is a category error); retry/backoff idioms. |
| date | read/text/proc | 1 | 2 | 3 | 1 | no | dominant `$(date +%F)` value-capture (punted); C2 up for `date -s` clock/log-timestamp tampering adjacency. |
| kill | read/text/proc | 1 | 3 | 4 | 2 | no | direct-PID signal (mutation) but no convergence; `kill -0` existence-probe is a separate guard idiom (see notes); impair-defenses adjacency. |
| killall | read/text/proc | 2 | 3 | 2 | 2 | no | name-pattern->process resolution before signaling; legacy/distro-dependent; impair-defenses adjacency. |
| pkill | read/text/proc | 2 | 3 | 3 | 2 | no | == killall (pattern+signal) but more portable (procps); impair-defenses adjacency. |
| pgrep | read/text/proc | 1 | 2 | 4 | 4 | guard-only | pure READ; `pgrep -x svc >/dev/null || start` liveness-guard (ships); value-capture variant docks. |
| wait | read/text/proc | 1 | 1 | 2 | 1 | no | background-job sync primitive; "already waited" isn't a state (structurally no convergence). |
| cmp | read/text/proc | 1 | 1 | 3 | 4 | guard-only | byte-exact compare whose whole role IS the idempotent-update guard (`cmp -s new old || cp`); rc-consumed, ships now. |
| logger | read/text/proc | 1 | 2 | 2 | 1 | no | one-way syslog write, no idempotency; C2 mild for log-injection/anti-forensics adjacency. |
| mail | read/text/proc | 1 | 2 | 2 | 1 | no | one-way send, no convergence (twice = two emails); generic external-send channel (tier-2 like curl/wget). |
| docker | container/orch | 4 | 1 | 4 | 4 | yes | broad cross-kind cells (container/image/network/volume) + heavy `run` argparse; `docker inspect` local rc-convergence; no infosec adjacency. |
| docker-compose | container/orch | 4 | 1 | 3 | 4 | yes | multi-service desired-state; delegates to `compose ps`/config-diff (delegation-oracle); per-service resolution + correlated disturbance. |
| podman | container/orch | 4 | 2 | 2 | 4 | yes | docker-compatible surface; rootless/userns-by-default nudges C2 up one; narrower install base. |
| kubectl | container/orch | 4 | 2 | 3 | 2 | yes | many resource kinds + resolve/reaches (Deployment->RS->Pod); `kubectl diff` is trustworthy but diffs REMOTE state (multi-host dock). |
| helm | container/orch | 5 | 2 | 2 | 2 | guard-only | k8s-kind surface + opaque chart-templating (must render for footprint); trustworthy diff needs helm-diff plugin; remote — double dock. |
| terraform | container/orch | 2 | 1 | 2 | 2 | yes | near-pure delegation (`plan -detailed-exitcode` 0/2) — low modeling cost; diffed state is remote (multi-host dock). |
| ansible-playbook | container/orch | 3 | 2 | 3 | 4 | no | canonical DECLINE — `--check` untrustworthy (uneven module coverage), oracle declines, line always runs; broad wall correctly protects downstream (non-punted). |
| chezmoi | container/orch | 2 | 1 | 2 | 5 | yes | clean delegation to `chezmoi verify`/`status` (local whole-tree drift check); niche dotfiles audience. |
| git | container/orch | 4 | 1 | 4 | 3 | yes | broad VCS cells (refs/index/worktree/remotes); `git status`/`diff --quiet` local delegation; `pull`/`fetch`/`push` need REMOTE compare (dock). SPLIT candidate. |
| curl | container/orch | 2 | 2 | 4 | 2 | guard-only | no native convergence (always re-fetches); elides only via a wrapping file-exists guard; dominant API/health-check usage feeds value-plane (punted). |
| wget | container/orch | 2 | 2 | 3 | 3 | guard-only | == curl but narrower; dominant plain-download usage + `-nc`/`-N` quasi-native skip-if-present nudges C4 above curl. |
| make | container/orch | 3 | 1 | 4 | 4 | no | opaque recipe runner; native mtime tracking is real but `.PHONY` opts out and Dorc doesn't parse Makefiles -> declines; broad wall = non-punted value. |
| systemd-nspawn | container/orch | 5 | 4 | 1 | 2 | guard-only | true context-transiting (mount/PID/UTS/net ns at once) — hardest tier; compounds TWO punts (fs-view + netns) vs chroot's one. |
| diff | read/text/proc | 1 | 1 | 4 | 4 | guard-only | `diff -q new old || cp` idempotent-update guard (like cmp); also value-capture usage; corpus-surfaced (dotfiles-frequent). |
| defaults | macos-config | 3 | 1 | 3 | 5 | yes | macOS per-(domain,key) value cell via `defaults read`; dotfiles-ubiquitous on macOS (USER_STORY's own unmodeled example); rc-convergence ships now. |
| chsh | auth/security | 2 | 4 | 2 | 5 | yes | login-shell cell via `getent passwd` readback; account/auth-adjacent; corpus-surfaced. |
<!-- 27Yb-TABLE-END -->

<!-- 27Yb-DROP-START -->
## §6 — Priority views (conductor synthesis)

**The strategic finding (the two priority axes pull apart).** The human ranked complexity (C1)
first and fable-risk (C2) second. These are POSITIVELY correlated in the worst way: the
machinery-exercising showcases (C1=5) cluster heavily in the gate-tripping band (C2>=4) —
privilege wrappers, packet-filtering, MAC, crypto. So "build the hard machinery to surface design
problems" and "let a gated Fable model author breadth cheaply" are **largely disjoint sets**. The
build should split into two tracks accordingly.

**(a) Hard-machinery track — C1>=4, route to a NON-GATED model (Mythos/Opus) or hand-author.** The
design-surfacing showcases, most of which a Fable-class conductor cannot safely be handed:
`sudo` `su` `doas` `runuser` `setpriv` (privilege wrappers — the marquee C1=5/C2=5 cell); `iptables`
`ip6tables` `nftables` (rule-ordering); `semanage` (subcommand breadth); `apt-get` `dnf` `yum`
(disturbance + provides-resolve — these are C2=2 so gated-safe); `mount` `chroot` `nsenter` `unshare`
`systemd-nspawn` `ip`(netns) (context-transiting but C4=2 punt-blocked — build the wrapper SHELL now,
defer in-context measurement).

**(b) Gated-safe breadth track — C2<=2, high C3/C4, any model can author.** The stdlib's cheap spine:
`systemctl` `service` `apt-get` `apt` `dpkg` `cp` `mkdir` `ln` `touch` `cat` `install` `test/[`
`grep` `sed` `cmp` `diff` `command -v` `pgrep` `docker` `docker-compose` `chezmoi` `terraform`
`git`(local) `brew` `make`(decline) `defaults`. (`chmod`/`chown` at C2=2 sit here with a light
"permissions topic" caution.)

**(c) Punt-blocked — defer until the gating feature lands (low C4), regardless of risk.**
- value-plane / output-capture (r26): `hostname` `uname` `id` `whoami` `dpkg-query` `getent`(value)
  `awk` `cut` `tr` `echo` `printf` `sleep` `date` `logger` `mail` `mktemp` `tee` `sync` `journalctl`
  `ss` `netstat` `ping` `wait` (most are also never-elide).
- fs-view / netns: `mount` `umount` `chroot` `nsenter` `unshare` `systemd-nspawn` `ip`(netns).
- MH2 version-window: `pip3` `npm`(install) `unattended-upgrade` `snap`/`flatpak`(refresh).
- multi-host / remote: `kubectl` `helm` `terraform` `git`(pull/fetch/push).

**(d) Never-elide — model only for disturbance/wall-safety, not elision.** `journalctl` `ss` `netstat`
`ping` `mv` `rmdir` `tar` `sync` `tee` `mktemp` `chpasswd` `unattended-upgrade` `dpkg-reconfigure`
`awk` `cut` `tr` `echo` `printf` `sleep` `date` `kill` `killall` `pkill` `wait` `logger` `mail`
`ansible-playbook` `make`. The last two are the valuable DECLINE cases — their broad walls correctly
protect downstream facts even though the command itself always runs.

**Highest single-command value (build these first within track (b), lift the most walls):** the
early-in-book churn-formers whose elision buys back everything downstream — `apt-get`
(update/install), `systemctl`, `cp`/`install` (config placement), and the guard primitives
`test`/`grep`/`command -v` that Dorc's whole recognition model rests on.

### Build-routing: the three lanes (human-directed, this session)

The build plan is a priority x risk 2x2 with three populated cells (priority = C3 plus design-value;
risk = C2 >= 3, i.e. OUTSIDE the Fable-safe C2<=2 subset):

- **Lane 1 — high-priority x low-risk (C2 <= 2) -> autonomous Fable implementers.** The 27Yb set's
  high-value slice: the packages/services/files/guards/containers spine. Cheap breadth; burn tokens.
- **Lane 2 — low-priority x high-risk (C2 >= 3) -> autonomous Opus implementers.** The risky tail
  that is not must-have: dpkg-statoverride, flatpak, chattr, chsh, deluser/chage/gpasswd,
  apparmor_parser/aa-enforce/setsebool, fail2ban-client, apt-key, openssl, doas/runuser/setpriv,
  nsenter/unshare/systemd-nspawn (also punt-blocked), pam-auth-update, killall/pkill,
  hostname/id/whoami (also value-plane-punted).
- **Lane 3 — high-priority x high-risk (C2 >= 3) -> human / outside-lineage / Opus-shepherd.** The
  must-haves a gated Fable model cannot safely author. This audit shows the lane is too big to
  hand-manage the guillotine turn-by-turn; the five below are the ones to route to an outside lineage
  or a shepherded Opus first.

**Lane-3 top five (most critical high-risk):**
1. `sudo` — C1=5/C2=5/C3=5/C4=4. The wrapper linchpin; the context-entry machinery (now confirmed
   live) stands or falls on it. Highest prevalence AND complexity — route to the most careful hand.
2. `ufw` — C1=4/C2=4/C3=4/C4=5. Firewall archetype, the USER_STORY's own example line, ships fully,
   elides via `ufw status`. Moderate complexity (nft/iptables rule translation) -> outside-lineage
   one-shot candidate.
3. `useradd` (+adduser/usermod/groupadd) — C1=3/C2=4/C3=4/C4=5. Account archetype, the User/Group
   kind-owner, elides via `getent` existence, ships now. The whole auth-family convergence hangs off
   it. Lower complexity -> good outside-lineage candidate.
4. `iptables` (+ip6tables/nftables) — C1=5/C2=5/C3=4/C4=3. The real packet-filter under ufw and the
   single hardest firewall-modeling problem: rule-ORDERING defeats confident `-C` elision (guard-only)
   and rules are netns-scoped. The design-surfacing one — the *modeling question* may need the human,
   not just a strong implementer.
5. `sysctl` — C1=3/C2=3/C3=3/C4=5. Kernel-hardening staple; ships fully, elides via single-key
   `/proc/sys` compare, with a genuine runtime-vs-persisted (`/etc/sysctl.d`) cell duality. The knobs
   that matter (rp_filter, kptr_restrict, syncookies) are exactly the defense-hardening topics the
   gate flags.

## §7 — Sensitivities & assumptions

- **block-context: CONFIRMED live** (the human, this session). The sudo/su/env wrapper machinery
  ships; all wrapper C4 grades stand as authored, and the earlier if-it-slips caveat is discharged.
- **C2 is deliberately conservative-high** per the human's field experience that the
  defensive-use carve-out is unreliable. Expect over-inclusion (safe commands graded 2) over
  under-inclusion (a risky command leaking into 27Yb).
- **C3 is the softest axis** — coarse prevalence, not measured; do not over-trust ±1.
- The scan surveys BROADER than the ~40 to build, deliberately including low-priority rows to
  justify their exclusion.
- **Split-candidates** (graded as one row on dominant usage; the implementation session should split):
  `ip` (addr/route mutate + ship now, vs `netns exec` context-transit punt), `git` (local ops ship
  vs remote-sync punt), `find` (bare read is trivial vs `-exec` fan-out), `sysctl` (generic vs
  hardening-knob C2), `getent`/`command -v`/`grep` (rc-consumed guard ships vs value-capture punt).
- **Omitted as individual rows:** the pure-value text cluster `sort`/`head`/`tail`/`wc`/`uniq`
  (corpus-frequent) shares the `cut`/`tr`/`awk` profile exactly (C1=1, C2=1, C4=1, never-elide,
  value-plane-punted) — modeling adds nothing until the value-plane lands.
- **`unshare` C2** normalized to 4 with the namespace family; its unprivileged-user-namespace-creation
  history is an arguable C2=5 (kept at 4 for family consistency, noted).
<!-- 27Yb-DROP-END -->
