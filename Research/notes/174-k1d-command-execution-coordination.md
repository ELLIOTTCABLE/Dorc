# 174 — K1d: command-execution coordination — the last uncracked component (round 17, 2026-06-07)

> **Final-final K1 round** (human steer, 2026-06-07): the very last component, and the one the human most
> hoped for. Find SOME pattern that involves a **COMMAND EXECUTION** exposing analyzable coordination over a
> shared kind. **EXCLUDED:** `# foo=bar` comments (the floor) **and** `FOO=bar` env-vars (k1c) — the
> coordination must ride the **command execution itself**. **RELAXED constraint (human):** assume Dorc MAY
> bless a small set of capabilities as hardcoded hooks/function-names in illegal-sh syntax (the `mycmd.check()`
> model) — so blessing a few commands/idioms is ON the table. Feeds `plans/17X`. AI-generated; conf-marked.

## Findings (lifted, most-load-bearing first)

- **k1d-answer (+SURE) — YES: with the blessing-allowance, command EXECUTIONS carry real kind-signal, not
  just co-reference.** This is the **first** of the four hunts (packaging, adjacent-fields, env-vars, here) to
  crack *more than co-reference* — precisely because blessing a *bounded vocabulary* is now permitted. Three
  forms, below.

- **k1d-getent (+SURE) — the jewel: a command that NAMES the kind as its argument.** `getent <database>
  <key>` [B-getent-man-2024]: **arg-1 is the kind** — a bounded, system-blessed NSS vocabulary (~16:
  `passwd` `group` `hosts` `services` `networks` `protocols` `ahosts` `ethers` `rpc` `shadow` `gshadow`
  `aliases` `initgroups` `netgroup`), configured in `/etc/nsswitch.conf` — and **arg-2 is the entity**. It is
  **read-only** (a lookup "through the same code path the kernel and apps use" → `getpwnam`/`getgrnam`/
  `gethostbyname`/`getservbyname`…), and its **rc is already a 3-outcome fact** (found 0 / not-found 2 /
  error 1·3). So `getent passwd "$u"` *is* a ready-made, self-kind-describing, kFAIL-withhold-safe fact-probe
  — for ~16 system kinds, the kind read straight off the command. Bless the database-vocabulary as kind-names
  and Dorc gets the kind **from the command execution**, no author handle. The strongest single artifact in
  the entire K1 effort.

- **k1d-wrapper (+SURE) — the command-wrapper category: a command whose *argument is a command*.** The
  `flock` man names the category itself — it *"wrap[s] … around the execution of a command, in a manner
  similar to `su(1)` or `newgrp(1)`"* (`flock file|directory COMMAND`) [B-flock-man-2024]. Extends to
  `sudo`/`su -c`/`ssh host CMD`/`docker exec C CMD`/`timeout N CMD`/`chroot DIR CMD`/`env CMD`/`nice`/`xargs
  CMD`/`find … -exec CMD`. The wrapper runs a **nested, recursively-analyzable** execution in a modified
  **context-kind** (user / host / container / lock / time-budget / root / namespace), and the context-kind is
  the **wrapper identity**. Note `ssh host CMD` and `docker exec C CMD` *are literally Dorc's own execution
  model* — the wrapper tells the analyzer *where/how* the inner command runs. (Caveat: `flock`'s lock target
  is a file; the *non-file* context-kinds live in `sudo`/`ssh`/`docker`/`chroot`/`timeout`.)

- **k1d-prodcons (+SURE) — producer/consumer via command substitution.** `kill "$(pgrep nginx)"` ·
  `rm "$(mktemp)"` · `useradd -u "$(id -u alice)" …`: the **producer** and **consumer** *command identities*
  expose the flowing value's kind (`pgrep`→PID, `mktemp`→tmpfile, `uuidgen`→uuid, `date +%s`→epoch,
  `git rev-parse`→sha), and the substitution `$(…)` is the analyzable dataflow edge (distinct from a pipe —
  it lands as an *argument*). Needs a blessed producer/consumer set; the *link* is free (the `$()` edge), the
  *kind* comes from the blessed identities.

- **k1d-bound (+SURE) — the honest catch: this works only for a BLESSED, BOUNDED vocabulary.** getent's ~16
  NSS kinds; the finite wrapper-category; a blessed producer/consumer set. For an **arbitrary / opaque** kind
  (the `frobctl`/wombat case), command execution exposes nothing beyond co-reference — you are back to the
  *blessed hook* (`mycmd.check()`) the human authorized, which is itself a blessing. So: **command-execution
  is a genuine kind-signal channel, but only via blessing**; the unbounded case stays co-reference-only,
  consistent with `notes/171`–`173`'s no-magic. **The blessing-allowance is the enabling condition** — and
  with it granted, this is the one place command-execution out-performs every prior channel.

- **k1d-frame (+SURE) — where it slots.** getent/`command -v`/`id` extend `09A` Tier-A (blessed structural
  idioms) *off-file*; getent is the multi-kind generalization. It also sharpens `plans/17X` S8 (read-existing
  -metadata): getent is the canonical read-existing-metadata probe **that self-names the kind**. The wrapper
  category is a *new* axis (execution-context coordination) prior rounds didn't surface.

## Citations (verbatim; [slug]:loc, cite-certainty)

> [B-getent-man-2024]:SYNOPSIS / DESCRIPTION (relevance: +1:SURE)
> getent [option ...] database key ...
> The getent command displays entries from databases supported by the Name Service Switch libraries, which
> are configured in /etc/nsswitch.conf. … The database may be any of those supported by the GNU C Library,
> listed below: … group … hosts … networks … passwd … protocols … rpc … services … shadow …
> [group→getgrnam/getgrgid · hosts→gethostbyname · passwd→getpwnam/getpwuid · services→getservbyname …]

> [B-flock-man-2024]:NAME / SYNOPSIS / DESCRIPTION (relevance: +1:SURE)
> flock - manage locks from shell scripts
> flock [options] file|directory command [arguments]
> The first and second of the above forms wrap the lock around the execution of a command, in a manner
> similar to su(1) or newgrp(1).

> command substitution (POSIX/bash language mechanism; the pgrep/kill idiom, kagi-surfaced) (relevance: +1:SURE)
> `kill "$(pgrep name)"` — command substitution runs `pgrep` in a subshell and substitutes its stdout as
> `kill`'s arguments (distinct from a pipe, which would feed kill's stdin, which kill does not read). The
> producer (pgrep→PID) and consumer (kill←PID) command identities expose the flowing kind.

## Verdict (→ plans/17X)
**Command-execution DOES carry kind-signal — for blessed/bounded vocabularies — and getent is the proof.**
`getent <kind> <entity>` is a real, canonical, read-only, self-kind-describing, 3-outcome fact-probe for ~16
system kinds; the wrapper category exposes execution-context-kinds; producer/consumer exposes flowing-kinds
via blessed identities. This is the *first* channel to beat co-reference — gated entirely on the human's
blessing-allowance. The arbitrary/opaque kind still needs the declared handle (no-magic holds there); but for
the blessed set, Dorc can read the kind **off the command itself**.

## Open / next (→ plans/17X)
- Fold into `plans/17X`: a candidate **P9** (getent-as-blessed-self-kind-describing-probe) + a note that
  command-execution carries kind-signal *for blessed vocabularies only*; the wrapper category is a new
  execution-context axis worth a `dq` of its own (`ssh host`/`docker exec` = Dorc's own model).
- Untaken (declared): the full wrapper zoo (`nsenter`/`unshare`/`ip netns exec`/`setpriv`/`runuser`);
  `update-alternatives --config` style; D-Bus `busctl call` (method-as-command). None expected to change the
  blessed-vs-arbitrary split.

## Wide catalog — the getent pattern replicated (the "kind-as-argument probe" family)

> Dug *wide* (human-directed extension of K1d, 2026-06-07): two general-purpose subagents fanned across
> cross-OS + Linux-security/hardware, **verifying against real man pages**; Linux-core read in main context.
> **The pattern is universal — every OS has it.** Confidence: ✓ = man-verified (me or subagent), ~ =
> subagent-reported-not-fetched (treat exact vocab as -GUESS), ° = from-knowledge. `[slug]` = registered
> source. Fanned-out rows are `graded-by: subagent` (provisional). Format: `tool` — shape/example — KIND
> vocabulary — mutativity.

**Three shapes of "kind-as-arg":** (A) **positional kind-first** `cmd <KIND> <key>`; (B) **`-t`/`--type`
flag** `cmd --type=<KIND>`; (C) **arg *is* a member of a kind-namespace** `cmd <kind.member>`.

### Name-service / DNS (the archetype)
- `getent` [B-getent-man-2024] ✓ — `getent <db> <key>` — passwd·group·hosts·ahosts·networks·protocols·services·rpc·ethers·netgroup·aliases·initgroups·shadow·gshadow — **read-only**, rc=3-outcome.
- `dig` [B-dig-dnsutils-man-2024] ✓ — `dig <name> <TYPE>` / `-t` — A·AAAA·MX·NS·SOA·TXT·PTR·SRV·CNAME·CAA·ANY… — read-only. (also macOS/BSD). `host -t`/`nslookup -query=` ~ same vocab.

### Linux networking / storage objects (A)
- `ip` [B-ip-iproute2-man-2024] ✓ — `ip <OBJECT> show` — address·link·route·rule·neighbour·netns·maddress·mroute·tunnel·tuntap·addrlabel·l2tp·macsec·mptcp·netconf·nexthop·ntable·tcpmetrics·token·vrf·xfrm·sr·fou·ila·ioam — read-only (show/list).
- `tc` ~ — `tc <obj> show` — qdisc·class·filter·action·chain·monitor. `nft list <obj>` ~ — tables·ruleset·chains·sets·maps·counters·quotas. `ethtool -<sub> <if>` ~ — -i driver·-k features·-g ring·-S stats. `udevadm info --query=<type>` ~ — name·symlink·path·property·all.
- `zfs` ° — `zfs get <property> <dataset>`; `zfs list -t <type>` (filesystem·snapshot·volume). `zpool get/status` °. `btrfs <group> <cmd>` °.

### Linux security (richest cluster, A)
- `semanage` [B-semanage-selinux-man-2024] ✓ — `semanage <object-type> -l` — login·user·port·interface·module·node·fcontext·boolean·permissive·dontaudit·ibpkey·ibendport — read-only (-l).
- `seinfo` ✓ — `seinfo <-component> [NAME]` — -t type·-c class·-b bool·-r role·-u user·-a attribute·--portcon·--genfscon… `sesearch <--rule-type>` ✓ — --allow·--auditallow·--dontaudit·--neverallow·--type_trans·--role_allow…
- `getsebool` ✓ — `getsebool <boolean>` / `-a` — the policy-boolean set — read-only. `matchpathcon <path>` ~, `secon -t/-u/-r` ~ (context lookups).
- `getfattr` ✓ — `getfattr -n <name> <file>` — xattr ns user.·security.·trusted.·system. `getcap` ✓ / `getpcaps` ~ / `capsh --print` ✓ (capabilities). `getfacl` ° (ACL entry-kinds user/group/mask/other).
- `ausearch` ✓ — `ausearch -m <msg-type>` — AVC·SYSCALL·PATH·USER_LOGIN·CONFIG_CHANGE… `auditctl -l` ~. `aureport --<report>` ~ (--auth·--login·--file·--syscall…).

### Linux namespaces / cgroups / modules / hardware (A+B)
- `lsns` [B-lsns-utillinux-man-2024] ✓ — `lsns -t <nstype>` — mnt·net·ipc·user·pid·uts·cgroup·time (exhaustive) — read-only.
- `dmidecode` [B-dmidecode-man-2024] ✓ — `dmidecode -t <type>` — bios·system·baseboard·chassis·processor·memory·cache·connector·slot (+0–127); `-s <keyword>`.
- `modinfo` ✓ — `modinfo -F <field> <mod>` — author·license·depends·parm·alias·filename·vermagic. `cgget -r <controller.param>` ~ (memory.·cpu.·pids.·io.). `lshw -C <class>` ✓ — disk·storage·network·memory·processor·bridge·display. `hwinfo --<class>` ~. `getconf <var>` ✓ — PAGE_SIZE·ARG_MAX·NAME_MAX·PATH_MAX·_NPROCESSORS_ONLN…

### "What KIND is this?" reporters (kind-as-output / format-token, C)
- `stat -c %F` ✓ — file-type flavour (regular·directory·block special·socket·symbolic link…); `%T` fs-type. `file --mime-type` ~. `type <name>` / `command -V` ° — alias·keyword·function·builtin·file·hashed. `tput <capname>` ✓ — colors·lines·cols·setaf… `locale <keyword|category>` ✓. `xdg-mime query <filetype|default>` ✓. `systemd-detect-virt` ✓ — emits kvm·vmware·docker·lxc·wsl… (inverse: kind-as-output).

### Config / capability / limit namespaces (C)
- `sysctl <param>` ✓ — kernel.·net.·vm.·fs.·dev. dotted tree — read-only (read; `-w` mutates). `prlimit --<resource> --pid` ~ — as·core·cpu·nofile·nproc·stack·memlock·rtprio. `chrt -p` ✓ / `ionice -p` ~ / `taskset -p` ~ (read with -p). `tuned-adm active` ~.

### Distro / systemd query (verb-first; adjacent)
- `update-alternatives --query/--display/--list <name>` ✓ — link-groups editor·pager·java·iptables·x-www-browser… (RH `alternatives --display`). `loginctl`/`busctl`/`hostnamectl`/`timedatectl`/`localectl` ✓~ (show/status). `debconf-show <pkg>` ~, `needs-restarting` ~.

### Cross-OS — macOS
- `system_profiler` [C-system-profiler-darwin-man-2024] ✓ — `system_profiler <SPDataType>` — SPHardwareDataType·SPSoftwareDataType·SPNetworkDataType·SPStorageDataType·SPDisplaysDataType·SPUSBDataType… (~40, `-listDataTypes`). `dscl . -read /<RecordType>/<name>` ~ — Users·Groups·Computers·Mounts·Hosts·Networks·Aliases·Printers·Protocols·RPC·Services.
- `defaults read <domain> [key]` ~, `scutil --get <pref>` ~ (ComputerName·LocalHostName·HostName), `pmset -g <category>` ~ (batt·therm·assertions·sched…), `launchctl print <target>` ~, `diskutil info <dev>` ~, `networksetup -get<thing>` ~, `mdls -name <attr>` ~, `sysctl <mib>` ~.

### Cross-OS — *BSD
- `sysctl <mib>` ✓ — hw.·kern.·vm.·net.·dev. `pfctl -s <modifier>` ✓ — rules·nat·states·info·Tables·labels·timeouts·Interfaces·queue. `rcctl get <daemon> <var>` ✓ — class·flags·status·logger·rtable·timeout·user. `sysrc <name>` ✓, `service <name> status` ✓, `kenv <var>` ✓ (smbios.·hint.), `gpart show` ✓, `ifconfig <if> <af>` ✓.

### Cross-OS — Solaris / illumos
- `dladm` [B-dladm-illumos-man-2024] ✓ — `dladm show-<class>` — link·phys·aggr·vlan·vnic·etherstub·iptun·overlay·bridge·wifi·linkprop·secobj. `ipadm show-<class>` ✓ — if·addr·ifprop·addrprop·prop. `kstat <module:instance:name:statistic>` ✓. `svcs <fmri>` ✓, `getent <db>` ✓ (archetype), `prtconf` ~, `zoneadm list` ✓.

### Cross-OS — AIX / HP-UX
- `lsattr -E -l <dev> [-a <attr>]` ✓ — modes -E·-D·-P·-R·-F; key by -c class·-s subclass·-t type. `lsdev -C -c <class>` ~ (disk·tape·adapter·if·memory·processor). `odmget <ObjectClass>` ~ (CuDv·CuAt…). `ioscan -fnC <class>` ~ (HP-UX). `getconf <var>` ° (POSIX).

### Cross-OS — Android (via `adb shell`)
- `getprop <prop>` ~ — ro.·persist.·sys.·dalvik.·net. `settings get <namespace> <key>` ✓ — **system·secure·global** (exactly 3, AOSP-verified). `pm list <kind>` ✓ — packages·users·permissions·permission-groups·features·libraries·instrumentation. `dumpsys <service>` ~ (`dumpsys -l` self-lists), `cmd <service>` ~ (mutativity-mixed).

### Cross-OS — Windows (CMD + PowerShell)
- `Get-CimInstance` [B-get-ciminstance-mslearn-2024] ~ — `-ClassName <Win32_*|CIM_*>` — Win32_Process·Win32_Service·Win32_LogicalDisk·Win32_BIOS·Win32_ComputerSystem… (hundreds; `Get-CimClass` enumerates). `Get-WmiObject -Class` ~ (legacy).
- `wmic <ALIAS> get` ✓ — CPU·OS·PROCESS·SERVICE·BIOS·BASEBOARD·DISKDRIVE·LOGICALDISK·MEMORYCHIP·NIC·USERACCOUNT… (~80). `sc query type=<driver|service|all> / state=<active|inactive|all>` ✓. `reg query <HKLM|HKCU|HKU|HKCR|HKCC>\…` ✓. `Get-ItemProperty <PSDrive>:\…` ~ (HKLM:·HKCU:·Cert:·Env:). `Get-Counter '<\Object(inst)\Counter>'` ~. `Get-PnpDevice -Class <Net|Disk|Display|USB…>` ~.

## Catalog synthesis (k1d-cat)
- **k1d-cat-universal (+SURE).** The getent shape — a bounded, higher-kinded **type/flavour named as a
  command argument**, used read-only for query/test — recurs on **every OS** and in **every subsystem**
  (name-service, networking, storage, security, namespaces, hardware, audit, config, service-management). It
  is arguably *the* dominant idiom for "interrogate system state by kind."
- **k1d-cat-readyprobes (+SURE).** Many are *already* read-only, 3-ish-outcome **fact-probes** directly
  blessable as multi-kind oracle probes with ~no wrapper: `getent`, `getsebool`, `sc query`,
  `systemctl is-active`, `dig`, `ip … show`, `Get-CimInstance`. A handful of blessed commands would cover a
  large slice of the high-frequency oracle library (the `effort-allocation` bootstrap).
- **k1d-cat-richveins (~SUSPECT).** Densest single vocabularies: **NSS** (getent ~16), **SELinux**
  (semanage/seinfo/sesearch ~12 object-types × rule-types), **link-layer** (ip/dladm/ipadm ~25 objects),
  **CIM/WMI** (Get-CimInstance, hundreds of classes), **SMBIOS** (dmidecode), **terminfo** (tput, hundreds).
- **k1d-cat-bound (+SURE, unchanged).** All are **blessed/bounded** vocabularies — the enabling condition.
  An arbitrary/opaque kind still needs the declared handle (`plans/17X` P1). But the catalog shows the
  *blessed* set is large, cross-OS, and high-value — a real oracle-library bootstrap, not a toy.

## Citations (verbatim; [slug]:loc)
> [B-ip-iproute2-man-2024]:SYNOPSIS (relevance: +1:SURE)
> ip [ OPTIONS ] OBJECT { COMMAND | help }
> OBJECT := { address | addrlabel | … | link | … | neighbour | … | netns | … | route | rule | … | xfrm }

> [B-semanage-selinux-man-2024]:SYNOPSIS (relevance: +1:SURE)
> semanage {login|user|port|interface|module|node|fcontext|boolean|permissive|dontaudit|ibpkey|ibendport} -l
> (object-type as arg-1; -l lists, read-only)

> [B-lsns-utillinux-man-2024]:OPTIONS -t (relevance: +1:SURE)
> -t, --type type — Display the specified namespace type only. The supported types are mnt, net, ipc, user,
> pid, uts, cgroup and time.

> [B-dladm-illumos-man-2024]:show-* (relevance: -0:SUSPECT, subagent)
> dladm show-link | show-phys | show-aggr | show-vlan | show-vnic | show-etherstub | show-linkprop … —
> read-only; kind welded into the subcommand, mirroring `ip <object> show`.

