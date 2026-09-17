# 312b-exercises/net-sysctls-are-per-namespace — one name, two namespaces; one knob, two spellings

Exercise record; ahistorical; the 2026-09-16 round over `notes/311j`. Book lines are real sh.
Oracle lines are strawman spellings and carry no weight; the `#` glosses are prose in `311j`
vocabulary. Grades on the conductor's claims: +SURE / ~SUSPECT / -GUESS / --WONDER. Exercises
`net-sysctls-are-per-namespace` and, beside it, `one-state-reached-through-two-kinds`. A rich
stdlib is assumed present.

Spelling rules in force for the strawmen (human-typed 2026-09-16, banked in `312c`): a bind is an
ordinary `local x="…"` with a trailer, `local id="$4" : is "sm.AccountId"`, never an inline
form; a runtime value with a type is one record line to the report lane with the type on the
line (`printf 'yields sm.Inode:%s\n' "$ino" >>"${DREP_V1:-/dev/null}"`), never a `printf` of
several bare values, and a `: attribute` carrying a payload on a `printf` line is suspect; a
trailer's `$name` is a reference to a `local` declared earlier in the body, never shell.

## The world

A kernel knob (`net.ipv4.ip_forward`, `kernel.pid_max`) is set by `sysctl -w` or by writing
`/proc/sys/<path>`: one knob, two spellings. A network namespace is a private copy of the
networking slice, every `net.*` knob included; `kernel.*` knobs are one per boot. `ip netns add
blue` labels a new namespace; `ip netns exec blue CMD` runs inside it, and also unshares the
mount table, overlaying `/etc/netns/blue/*`. A label can be re-attached to an existing
namespace; the kernel's identity is the nsfs inode. The dotted spelling is ambiguous where an
interface name contains a dot; the `/proc/sys` path is not (+SURE all).

## The book, the catastrophes, the truths

```sh
#!/bin/sh
set -eu
sysctl -w net.ipv4.ip_forward=1                        # 3   the host forwards
ip netns add blue                                      # 4
ip netns exec blue sysctl -w net.ipv4.ip_forward=1     # 5   and so does blue
sysctl -w net.ipv4.conf.all.rp_filter=1                # 6   the host's reverse-path filter
ip netns exec blue sysctl -w kernel.pid_max=4194304    # 7   contrived: a boot-wide knob, poked from inside blue
sysctl -w kernel.pid_max=4194304                       # 8   the same knob as line 7
printf 0 >/proc/sys/net/ipv4/ip_forward                # 9   a vendor hardening snippet, pasted
sysctl -w net.ipv4.ip_forward=1                        # 10  the admin's own re-enable
```

Catastrophes (`311b:rul-ground-identity-in-final-outcomes`): C1, wrong SAME across the transit,
line 3's converged fact stands in for line 5 and blue never forwards; C2, wrong DISJOINT across
the two spellings, line 9's write is taken to be about something other than line 10's mCell,
line 10's elision survives it, and the host stays dark. Truths: line 5 is a different mCell
from line 3, and line 6 from line 5; line 7 is the same mCell as line 8; line 9 writes line
10's mCell.

## The actors

Here the letters mark who owns what. Alice writes the book. Rachel and Simon own stdlib
vocabularies, present from the start: `rachel-kernel-params.oracle.sh` (`sm.KernelParam`,
primary mScheme `sm.ProcSysPath`) and `simon-namespaces.oracle.sh` (`sm.NetNamespace`,
`sm.MountNamespace`, `sm.Boot`). Michael (procps) and Nathan (iproute2) are a new team's tool
authors: `michael-sysctl.oracle.sh`, `nathan-ip.oracle.sh`. Oscar (`acct`) and Tessa (files)
appear only in the three-seats section, to show the other two seats. Nobody has met.

## The floor: two tool oracles, no yields

Michael and Nathan bind under mSchemes of their own with no `:yields`: the floor of `311j`
§ 1.3 (a primary mScheme of an unnamed mSort, identity `resolve()`, no warrants, the mRoute its
only mParent). Nathan's second lend line is the coarse truth about the mount table;
`lends-a-fresh-instance` is minted here (`273` has a pass-through lend and a valued lend, and no
spelling for an instance the wrapper created that nobody can name).

```sh
# dorc-lang/v0.2   michael-sysctl.oracle.sh
sysctl__is_converged() {
   [ "${1-}" = -w ] && shift
   [ $# -eq 1 ] && [ "${1#*=}" != "$1" ] || return 2       # one KEY=VALUE, nothing else
   local key="${1%%=*}"   : is "sm.SysctlKey"
   local want="${1#*=}"
   [ "$(sysctl -n -- "$key")" = "$want" ]   : asserts "sm.SysctlKey:$key"
}
sysctl__disturbs() {
   [ "${1-}" = -w ] && shift
   [ $# -eq 1 ] || return
   local key="${1%%=*}"   : is "sm.SysctlKey"
   printf 'disturbs sm.SysctlKey:%s\n' "$key" >>"${DREP_V1:-/dev/null}"
}
```

```sh
# dorc-lang/v0.2   nathan-ip.oracle.sh
ip__lend_map() {                                 # the peel is 273's: the guest starts after the namespace name
   [ "$1 $2" = "netns exec" ] || return
   local ns="$3"   : is "sm.NetnsName"
   printf 'lends sm.NetnsName:%s\n' "$ns"              >>"${DREP_V1:-/dev/null}"
   printf 'lends-a-fresh-instance sm.MountNamespace\n' >>"${DREP_V1:-/dev/null}"
   printf 'lends nothing-else\n'                       >>"${DREP_V1:-/dev/null}"
}
ip__is_converged() {
   [ "$1 $2" = "netns add" ] && [ $# -eq 3 ] || return 2
   local ns="$3"   : is "sm.NetnsName"
   [ -e "/var/run/netns/$ns" ]   : asserts "sm.NetnsName:$ns"          # the entry's existence
}
ip__disturbs() {
   [ "$1 $2" = "netns add" ] && [ $# -eq 3 ] || return
   local ns="$3"   : is "sm.NetnsName"
   printf 'disturbs sm.NetnsName:%s\n' "$ns" >>"${DREP_V1:-/dev/null}"   # a routing write
}
```

Render, steady state (host and blue converged; blue exists):

```
 3  # sysctl -w net.ipv4.ip_forward=1                       # converged
 4  # ip netns add blue                                     # converged: namespace blue exists
 5  # ip netns exec blue sysctl -w net.ipv4.ip_forward=1    # converged: probed inside blue
 6  # sysctl -w net.ipv4.conf.all.rp_filter=1               # converged
 7  # ip netns exec blue sysctl -w kernel.pid_max=4194304   # converged: probed inside blue
 8  # sysctl -w kernel.pid_max=4194304                      # converged: probed again; not known to be line 7's cell
 9  printf 0 >/proc/sys/net/ipv4/ip_forward                # runs: a wall
10  ( sysctl_check -w net.ipv4.ip_forward=1 ) \
10     || sysctl -w net.ipv4.ip_forward=1                   # verify: converged, but past line 9
plan: 1 to run, 1 to verify (6 skipped)
```

The steady-state shape is reached at the floor. Lines 5 and 7 are entered through the wrapper
(root reused, never acquired, `27C`) and probed where they run. Every identity comparison
reads UNKNOWN: line 5's mKey and line 3's are one floor mScheme across a transit, two
mPlaceholders, no warrant; likewise 7 against 8, and 6 against 5. Unknown is safe for both
consumers: C1 cannot fire (nothing stands in for anything) and C2 cannot fire (line 9 is a
total wall). On a blue-drifted day line 5 runs and lines 6, 7, 8, and 10 verify: the engine
cannot say line 5's write missed any of them (`plan: 2 to run, 4 to verify (2 skipped)`).

## The stdlib, and the two glue lines

Rachel holds the one fact Michael cannot: which shapes live in a namespace and which in the
boot. Simon holds namespaces and the boot. Michael and Nathan each add one lookup that yields
into a stdlib primary mScheme, the glue line `312b` § 7 promised a stranger. Each arm of a
primary's `resolve()` records the store of the shape it matched, as one record line with the
spelling and the instance on it; the read of "which namespace is this process in" is Simon's
helper, called from Rachel's arm (its home is under discussion in `312c`).

```sh
# dorc-lang/v0.2   rachel-kernel-params.oracle.sh
sm_KernelParam__declaration() {                  # sort-level speech; the member's name carries no weight
   : : primary-scheme "sm.ProcSysPath"
}
sm_ProcSysPath__resolve() {                      # identity on the path; each arm records the store of that shape
   case "$1" in
   net/*)          printf 'identified-in sm.NetnsInode:%s warrants guarantees-unique-name,guarantees-unique-referent,sole-route\n' "$(netns_here)" >>"${DREP_V1:-/dev/null}" ;;
   kernel/*|vm/*)  printf 'identified-in sm.BootId:%s warrants guarantees-unique-name,guarantees-unique-referent,sole-route\n'     "$(boot_here)"  >>"${DREP_V1:-/dev/null}" ;;
   *)              return 2 ;;                   # fs/*, user/*, abi/*: not surveyed; unknown from this level
   esac
}
sm_KernelParam__disturbance_reaches() {          # a knob drags nothing along
   printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
sm_KernelParam__lives_in() {                     # the placement set is closed at just-the-parent; the member's name carries no weight
   printf 'lives-in nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
```

```sh
# dorc-lang/v0.2   simon-namespaces.oracle.sh
sm_NetNamespace__declaration() { : : primary-scheme "sm.NetnsInode" }
sm_NetnsInode__resolve() {                       # one shape, identified in the boot; no unique-referent: nsfs inode numbers are reissued
   printf 'identified-in sm.BootId:%s warrants guarantees-unique-name,sole-route\n' "$(boot_here)" >>"${DREP_V1:-/dev/null}"
}
netns_here() {                                   # which network namespace is this process in?
   readlink /proc/self/ns/net | sed 's/^net:\[\(.*\)\]$/\1/'
}
sm_Boot__declaration() { : : primary-scheme "sm.BootId" }
sm_BootId__resolve() {                           # no store declared, so scoped in the route
   printf 'warrants guarantees-unique-referent\n' >>"${DREP_V1:-/dev/null}"   # a cloned boot resolving SAME is this line's fault: the stdlib owns that horizon (human lean, 2026-09-16)
}
boot_here() { cat /proc/sys/kernel/random/boot_id; }
```

```sh
# added to michael-sysctl.oracle.sh
sm_SysctlKey__resolve() {                        # the dotted name, into the /proc/sys path; runs where the mKey was bound
   local p; p=$(printf '%s' "$1" | tr . /)
   [ -e "/proc/sys/$p" ] || return 2             # a name I cannot find here: cannot say
   printf 'yields sm.ProcSysPath:%s\n' "$p" >>"${DREP_V1:-/dev/null}"
}
# added to nathan-ip.oracle.sh
sm_NetnsName__resolve() {                        # the label, into the namespace's nsfs inode
   local ino; ino=$(stat -L -c %i -- "/var/run/netns/$1") || return 2
   printf 'yields sm.NetnsInode:%s\n' "$ino" >>"${DREP_V1:-/dev/null}"
}
```

The walks:

- Line 5 against line 3 (the C1 guard). Line 5's mKey: Michael's bind, Michael's yield run
  inside blue, Rachel's `net/*` arm, whose record names the namespace the arm ran in (the lent
  `blue`, which Nathan's yield turns into nsfs inode B), then Simon's arm putting that
  namespace in the boot, and the boot and the route inherited from the caller through
  Nathan's sentinel. Line 3's mKey: the same down to the namespace level, where Rachel's arm
  ran on the host and recorded inode H. From the top: the route and the boot are the
  identical inherited instances; at the namespace level H and B differ inside one shared
  mParent, that shape carries `:guarantees-unique-name` (Simon), and every level from there
  to the leaf carries `:sole-route` (Simon for the namespace in its boot, Rachel for `net/*`
  in its namespace). DISJOINT. Line 5 gets its own probe; line 3 never stands in. Line 6
  against line 5 walks the same way.
- Line 7 against line 8. Both yield `kernel/pid_max`; Rachel's `kernel/*` arm puts the
  mParent in the boot. Read in every context, the two boot ids are equal bytes and SAME
  rests on Simon's `:guarantees-unique-referent`; shared through Nathan's sentinel, they are
  one instance and no warrant is consulted. Either way SAME, attributed. At the leaf the keys
  are equal and the shape carries `:guarantees-unique-referent`. One probe answers both.
- Line 9 against line 10 (the C2 guard). The shell routes bytes to
  `/proc/sys/net/ipv4/ip_forward`; the locator goes to `30T`'s File binder, which declines on
  procfs; line 9 is a total wall; line 10 guards. Safe, and the identity model is never
  consulted.
- Line 4 on a drifted day. It runs; its footprint is a routing write to `sm.NetnsName:blue`;
  lines 5 and 7 resolved `blue` through that entry, so their mResolutions perish and they run.
  Correct with nobody speaking.

Render, steady state: as the floor, except line 8 reads "the same cell as line 7". Render,
blue-drifted day, with the flag:

```
 3  # sysctl -w net.ipv4.ip_forward=1                       # converged
 4  # ip netns add blue                                     # converged: namespace blue exists
 5  ip netns exec blue sysctl -w net.ipv4.ip_forward=1      # runs: diverged (blue does not forward)
 6  # sysctl -w net.ipv4.conf.all.rp_filter=1               # converged; survives line 5 (rachel: sole-route on net/*; simon: unique-name, sole-route on the namespace)
 7  ( sysctl_check, inside blue ) \
 7     || ip netns exec blue sysctl -w kernel.pid_max=4194304   # verify: converged, but past line 5
 8  ( sysctl_check -w kernel.pid_max=4194304 ) \
 8     || sysctl -w kernel.pid_max=4194304                  # verify: converged, but past line 5
 9  printf 0 >/proc/sys/net/ipv4/ip_forward                # runs: a wall
10  ( sysctl_check -w net.ipv4.ip_forward=1 ) \
10     || sysctl -w net.ipv4.ip_forward=1                   # verify: converged, but past line 9
plan: 2 to run, 3 to verify (3 skipped)
```

## The three seats that can supply a store

The store of a primary key (`311j`'s mParent through the primary mScheme) is a value like any
other, and the language already has three homes for a value with a type: on a bind's trailer
as a coordinate, on a record line from a yield, or on a record line from the primary's own
arm. Exactly one seat speaks per key: two that disagree refuse (a contradiction, attributed
to both); none is unknown. Where the bytes inside the seat came from (a literal, a second
field of the same `stat`, an environment variable, a helper that reads the world) is
ordinary sh, graded by the value plane (`275`), never declared.

```sh
# the bind has it: oscar-acct.oracle.sh (the store is on the command line)
acct__is_converged() {
   [ "${1-}" = --db ] && [ "${3-}" = enable ] && [ $# -eq 4 ] || return 2
   local db="$2"   : is "sm.Path"
   local id="$4"   : is "sm.AccountId" identified-in "sm.Path:$db"
   acct --db "$db" is-enabled "$id"   : asserts "sm.AccountId:$id"
}
# the yield has it: tessa-file.oracle.sh (the filesystem comes out of the same stat as the inode)
sm_Path__resolve() {
   local both; both=$(stat -c '%i %d' -- "$1") || return 2
   local ino="${both%% *}" dev="${both#* }"
   printf 'yields sm.Inode:%s\n' "$ino"                 >>"${DREP_V1:-/dev/null}"
   printf 'identified-in sm.DeviceNumber:%s\n' "$dev"   >>"${DREP_V1:-/dev/null}"
}
# the primary's arm has it: Rachel, above (the store is wherever the probe stands)
```

What each seat asks of a stranger: nothing beyond emitting the primary key, except where the
primary cannot compute its store from the key alone (files), in which case a spelling that
does not also emit the store leaves its keys unknown, `dorc why` names the spelling, and the
fix is one record from data the lookup already had.

## Observations

- `obs-the-sentinel-is-the-keystone` (+SURE) — every SAME and every DISJOINT passes through
  the boot and the route as inherited instances, and Nathan's `lends nothing-else` is what
  makes them inherited rather than ⊤. Without that one line the glue lines buy nothing. The
  cheapest line in the set carries the most.
- `obs-the-parent-instance-is-a-value` (+SURE; acked in chat 2026-09-16) — `311j` § 1.6 fixes
  who supplies a key's store by the kind of key (the yield for a yielded key, the mVantage for
  a direct bind), which put Rachel's knowledge on Michael's line. The case shows three seats
  (bind, yield, primary arm), any one of which may supply it as a typed value; § 2.1's
  `:parent-key-of` is subsumed by the record naming the spelling. Supply "modes" are not a
  concept: a literal store is a fixed name resolved in context like any key (configuration is
  world-state, and a literal is the owner's assumption about it, attributed); a store on the
  command line is visible at the bind by construction; an environment variable and an
  argument are both shell words; a store read from the world is a read. The sort, not the
  spelling, declares which spelling is primary.
- `obs-identical-instances-need-no-warrant` (~SUSPECT of § 3.2's text; +SURE it must hold) —
  an inherited mParent is one mKey on both sides, not two equal values from two lookups.
  § 3.2's "every level's shape carries `:guarantees-unique-referent`" applies to equal values;
  read strictly over inherited levels, no same-host pair would ever be SAME. Whether the
  engine shares the placeholder through the wrapper's sentinel or reads twice and rests on
  the owner's warrant is an engine choice; both are sound and attributed.
- `obs-the-natural-key-is-the-ambiguous-one` (+SURE) — the path is the primary mScheme; the
  dotted name yields into it by a lookup that must run where the mKey was bound, because a
  knob exists only in the namespace that has its interface.
- `obs-gotcha-30-is-guarded-by-the-binder` (+SURE) — line 9's safety is `30T`'s File binder
  declining procfs; `:yields` is never asked. A binder per mScheme (Rachel claiming
  `/proc/sys/*` locators as `sm.ProcSysPath` keys) would let line 9 wall one mCell instead of
  everything; in this book only line 10 is below it, and it collides either way.
- `obs-same-sort-different-depth-reads-unknown` (+SURE of the walk) — lines 7 and 8 verify on
  the drifted day because under the shared boot the children are a namespace key and a knob
  key, two mSchemes of two mSorts; § 3.2 is written for aligned chains and is silent, and the
  generous reading (each shape carries `:guarantees-unique-name`, so disjoint) is refuted by
  `one-state-reached-through-two-kinds` itself, since two mSorts over one referent would then
  read DISJOINT. "A namespace is not its boot" has no seat in `311j`: § 1.2 forbids the engine
  assuming it and no owner can say it. It recurs wherever one mSort's shapes identify at
  different depths (`systemctl --user` units in a user's manager beside system units in the
  boot's). The one sound seat visible is a third lookup warrant, declared by a spelling's owner
  about their own spelling only: distinct keys never reach one referent, whatever their
  mParents; it licenses DISJOINT from key inequality with no shared mParent. Sysctl paths, unit
  names across managers, and dpkg names hold it; paths do not (hardlinks), and the pid
  spelling refutes a lazy declaration (pid 1 and pid 4821 are distinct keys and one process).
  Unacked; the guard is honest without it.
- `obs-the-glue-lines-pay-on-drifted-days` (+SURE) — the floor already reaches the
  steady-state shape; the two yields buy line 6's survival on a blue-drifted day and one probe
  on any day. USER_STORY stage 5's lesson, replayed at the identity tier.
- `obs-cloned-boots-are-simons-line` ([HUMAN] lean, 2026-09-16) — the model expresses the
  cloned-boot horizon as a stdlib warrant with a name on it, never as a rule of its own.
