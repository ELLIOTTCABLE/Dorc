# 312b-exercises/net-sysctls-are-per-namespace — one name, two namespaces; one knob, two spellings

Exercise record; ahistorical; the 2026-09-16 round over `notes/311j`. Book lines are real sh.
Oracle lines are strawman spellings under `312c` § 1 and carry no weight; the `#` glosses are
prose in `311j` vocabulary. Grades on the conductor's claims: +SURE / ~SUSPECT / -GUESS /
--WONDER. Exercises `net-sysctls-are-per-namespace` and, beside it,
`one-state-reached-through-two-kinds`.

## The world, in plain words

The Linux kernel has tunable knobs: whether the machine forwards packets between its network
interfaces (`net.ipv4.ip_forward`, 0 or 1); the largest process id it will hand out
(`kernel.pid_max`). A knob is set two ways, and they are one knob: `sysctl -w
net.ipv4.ip_forward=1`, or a write to the pseudo-file `/proc/sys/net/ipv4/ip_forward`. That is
`one-state-reached-through-two-kinds`.

A namespace is a private copy of one slice of the kernel's world, handed to a group of
processes; containers are built from them. A network namespace is a private copy of the
networking slice: its own interfaces, routes, firewall, and its own copies of every `net.*`
knob. `ip netns add blue` creates one and labels it `blue`; the label is a file under
`/var/run/netns/`, and a label can be attached to a namespace that already exists, so labels are
not identity. The identity the kernel exposes is an inode number, visible as
`net:[4026531992]` through `/proc/self/ns/net`. `ip netns exec blue CMD` runs CMD inside it. A
process inside `blue` reads and writes `blue`'s copy of `net.ipv4.ip_forward`; a process
outside reads the host's. `kernel.*` knobs are not copied per namespace: there is one per
running kernel, from power-on to reboot (a boot). That is `net-sysctls-are-per-namespace`,
both halves.

Two details the strawmen lean on (+SURE): `ip netns exec` also gives the command a private
mount table and overlays `/etc/netns/blue/*` over `/etc/*` where present, so a file inside
`blue` may not be the file outside; and sysctl's dotted spelling is ambiguous where an
interface name contains a dot (`eth0.100`), while the `/proc/sys` path is not, so the path is
the honest primary mScheme and the dotted name a way of writing it.

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
from line 3, and line 6 is a different mCell from line 5; line 7 is the same mCell as line 8;
line 9 writes line 10's mCell.

## The actors

Here the letters mark who owns what. Alice writes the book. Michael (procps) and Nathan
(iproute2) write tool oracles: `michael-sysctl.oracle.sh`, `nathan-ip.oracle.sh`. Rachel and
Simon own stdlib vocabularies: `rachel-kernel-params.oracle.sh` (`sm.KernelParam`) and
`simon-namespaces.oracle.sh` (`sm.NetNamespace`, `sm.MountNamespace`, `sm.Boot`). Nobody has met.

## Stage 0 — nothing authored

Eight tool lines run. Alice spent nothing and got nothing.

## Stage 1 — Michael's oracle

Michael binds under a mScheme nobody has declared: the floor of `311j` § 1.3 (a primary
mScheme of an unnamed mSort, identity `resolve()`, no warrants, the mRoute its only mParent).

```sh
# dorc-lang/v0.2   michael-sysctl.oracle.sh
sysctl__is_converged() {
   [ "${1-}" = -w ] && shift
   [ $# -eq 1 ] && [ "${1#*=}" != "$1" ] || return 2       # one KEY=VALUE, nothing else
   key : sm.SysctlKey = "${1%%=*}"
   [ "$(sysctl -n -- "$key")" = "${1#*=}" ]   : asserts "sm.SysctlKey:$key"
}
sysctl__disturbs() {
   [ "${1-}" = -w ] && shift
   [ $# -eq 1 ] || return
   printf '%s\n' "${1%%=*}"   : disturbs "sm.SysctlKey"
}
```

Render, steady state (host and blue converged; blue exists):

```
 3  # sysctl -w net.ipv4.ip_forward=1                       # converged
 4  ip netns add blue                                       # runs: unmodeled ('ip')
 5  ip netns exec blue sysctl -w net.ipv4.ip_forward=1      # runs: unmodeled ('ip')
 6  ( sysctl_check -w net.ipv4.conf.all.rp_filter=1 ) \
 6     || sysctl -w net.ipv4.conf.all.rp_filter=1           # verify: converged, but past 'ip' (line 4)
 7  ip netns exec blue sysctl -w kernel.pid_max=4194304     # runs: unmodeled ('ip')
 8  ( sysctl_check -w kernel.pid_max=4194304 ) \
 8     || sysctl -w kernel.pid_max=4194304                  # verify: converged, but past 'ip' (line 4)
 9  printf 0 >/proc/sys/net/ipv4/ip_forward                # runs: a shell write nobody vouches; a wall
10  ( sysctl_check -w net.ipv4.ip_forward=1 ) \
10     || sysctl -w net.ipv4.ip_forward=1                   # verify: converged, but past line 9
plan: 4 to run, 3 to verify (1 skipped)
```

The wrapper is unmodeled, so lines 5 and 7 are never entered; no identity question has arisen.

## Stage 2 — Nathan's oracle

Nathan describes `ip netns add` and the `netns exec` entry. `sm.NetnsName` is another floor
mScheme. The lend's second line is the coarse truth about the mount table.
`lends-a-fresh-instance` is minted in this exercise: `273` has a bare lend (the caller's
instance passes through) and a valued lend (an instance with a key), and no spelling for an
instance the wrapper created that nobody can name.

```sh
# dorc-lang/v0.2   nathan-ip.oracle.sh
ip__lend_map() {                                 # the peel is 273's: the guest starts after the namespace name
   [ "$1 $2" = "netns exec" ] || return
   printf '%s\n' "$3"   : lends "sm.NetnsName"
   :                    : lends-a-fresh-instance "sm.MountNamespace"
   printf 'lends nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
ip__is_converged() {
   [ "$1 $2" = "netns add" ] && [ $# -eq 3 ] || return 2
   ns : sm.NetnsName = "$3"
   [ -e "/var/run/netns/$ns" ]   : asserts "sm.NetnsName:$ns"          # the entry's existence
}
ip__disturbs() {
   [ "$1 $2" = "netns add" ] && [ $# -eq 3 ] || return
   printf '%s\n' "$3"   : disturbs "sm.NetnsName"                        # a routing write
}
```

Render, steady state:

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

The steady-state shape is reached. Lines 5 and 7 are entered through the wrapper (root reused,
never acquired, `27C`) and probed where they run. Every identity comparison reads UNKNOWN: line
5's mKey and line 3's are one floor mScheme across a transit, two mPlaceholders, no warrant;
likewise 7 and 8, and 6 against 5. Unknown is safe for both consumers: C1 cannot fire
(nothing stands in for anything) and C2 cannot fire (line 9 is a total wall). On a
blue-drifted day line 5 runs and lines 6, 7, 8, and 10 verify: the engine cannot say line 5's
write missed any of them (`plan: 2 to run, 4 to verify (2 skipped)`).

## Stage 3 — Rachel and Simon name the parents; Michael and Nathan yield into them

Rachel owns the knob vocabulary and the one fact Michael cannot hold: which shapes live in a
namespace and which in the boot. Simon owns namespaces and the boot. Michael and Nathan each
add one lookup that yields into a stdlib primary mScheme, the glue line `312b` § 7 promised a
stranger.

```sh
# dorc-lang/v0.2   rachel-kernel-params.oracle.sh
sm_ProcSysPath__resolve() {                      # primary: identity on the path; the trailers are the per-shape declarations
   : : primary-of "sm.KernelParam"
   case "$1" in
   net/*)          printf '%s\n' "$1"   : identified-in "sm.NetNamespace" warrants "guarantees-unique-name,guarantees-unique-referent,sole-route" ;;
   kernel/*|vm/*)  printf '%s\n' "$1"   : identified-in "sm.Boot"         warrants "guarantees-unique-name,guarantees-unique-referent,sole-route" ;;
   *)              return 2 ;;                   # fs/*, user/*, abi/*: not surveyed; unknown from this level
   esac
}
sm_KernelParam__disturbance_reaches() {          # a knob drags nothing along
   printf 'disturbs nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
sm_KernelParam__lives_in() {                     # the placement set is closed at just-the-parent; the member name carries no weight
   printf 'lives-in nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
```

```sh
# dorc-lang/v0.2   simon-namespaces.oracle.sh
sm_NetnsInode__resolve() {
   : : primary-of "sm.NetNamespace"
   printf '%s\n' "$1"   : identified-in "sm.Boot" warrants "guarantees-unique-name,sole-route"   # no unique-referent: nsfs inode numbers are reissued
}
sm_NetnsInode__here() {                          # binds the ambient placeholder at standup, wherever the probe runs
   readlink /proc/self/ns/net | sed 's/^net:\[\(.*\)\]$/\1/'   : yields "sm.NetnsInode"
}
sm_BootId__resolve() {                           # primary of sm.Boot; no parent declared, so scoped in the route
   : : primary-of "sm.Boot"
   printf '%s\n' "$1"   : warrants "guarantees-unique-referent"   # a cloned boot resolving SAME is this line's fault: the stdlib owns that horizon (human lean, 2026-09-16)
}
```

```sh
# added to michael-sysctl.oracle.sh
sm_SysctlKey__resolve() {                        # the dotted name, into the /proc/sys path; runs where the mKey was bound
   p=$(printf '%s' "$1" | tr . /)
   [ -e "/proc/sys/$p" ] || return 2             # a name I cannot find here: cannot say
   printf '%s\n' "$p"   : yields "sm.ProcSysPath"
}
# added to nathan-ip.oracle.sh
sm_NetnsName__resolve() {                        # the label, into the namespace's nsfs inode
   ino=$(stat -L -c %i -- "/var/run/netns/$1") || return 2
   printf '%s\n' "$ino"   : yields "sm.NetnsInode"
}
```

The walks:

- Line 5 against line 3 (the C1 guard). Line 5's mKey: Michael's bind, Michael's yield run
  inside blue, Rachel's `net/*` shape, its mParent the lent `blue` through Nathan's yield,
  nsfs inode B, `:identified-in sm.Boot`, the boot and the route inherited from the caller
  through Nathan's sentinel. Line 3's mKey: the same down to the namespace level, where
  Simon's ambient read binds inode H. From the top: the route and the boot are the identical
  inherited instances; at the namespace level H and B differ inside one shared mParent, that
  shape carries `:guarantees-unique-name` (Simon), and every level from there to the leaf
  carries `:sole-route` (Simon for the namespace in its boot, Rachel for `net/*` in its
  namespace). DISJOINT. Line 5 gets its own probe; line 3 never stands in. Line 6 against
  line 5 walks the same way.
- Line 7 against line 8. Both yield `kernel/pid_max`; Rachel's `kernel/*` shape puts the
  mParent in `sm.Boot`; Nathan lent nothing on that mSort, so line 7's boot is line 8's boot,
  one instance. At the leaf the keys are equal and the shape carries
  `:guarantees-unique-referent`. SAME. One probe answers both; nobody read a boot id.
- Line 9 against line 10 (the C2 guard). The shell routes bytes to
  `/proc/sys/net/ipv4/ip_forward`; the locator goes to `30T`'s File binder, which declines on
  procfs; line 9 is a total wall; line 10 guards. Safe, and the identity model is never
  consulted.
- Line 4 on a drifted day. It runs; its footprint is a routing write to `sm.NetnsName:blue`;
  lines 5 and 7 resolved `blue` through that entry, so their mResolutions perish and they run.
  Correct with nobody speaking.

Render, steady state: as stage 2, except line 8 reads "the same cell as line 7". Render,
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

## Observations

- `obs-the-sentinel-is-the-keystone` (+SURE) — every SAME and every DISJOINT in stage 3 passes
  through the boot and the route as inherited instances, and Nathan's `lends nothing-else` is
  what makes them inherited rather than ⊤. Without that one line stage 3 buys nothing. The
  cheapest line in the set carries the most.
- `obs-identical-instances-need-no-warrant` (~SUSPECT of § 3.2's text; +SURE it must hold) —
  an inherited mParent is one mKey on both sides, not two equal values from two lookups.
  § 3.2's "every level's shape carries `:guarantees-unique-referent`" applies to equal values;
  read strictly over inherited levels, no same-host pair would ever be SAME.
- `obs-the-natural-key-is-the-ambiguous-one` (+SURE) — the path is the primary mScheme; the
  dotted name yields into it by a lookup that must run where the mKey was bound, because a
  knob exists only in the namespace that has its interface.
- `obs-the-parent-rides-the-primary-shape` (+SURE it differs from `311j` § 1.6) — Michael's
  yield emits only the path; Rachel's shape says the mParent is the ambient namespace or the
  ambient boot. Under § 1.6 as written the yielding `resolve()` supplies the mParent instance,
  so Michael would have to emit the namespace inode, which is the knowledge B(b) said a
  stranger never needs. Where the mParent rides the bytes (an inode's device number) the
  primary's shape would say so; not this exercise's case.
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
  boot's). The only sound seat visible is an mSort owner's open-world claim that no mScheme of
  another mSort names their referents: the identity twin of `30U`'s finished definition,
  knife-tier, flag-gated. Not adopted here; the guard is honest.
- `obs-stage-three-pays-on-drifted-days` (+SURE) — stage 2 already reaches the steady-state
  shape; stage 3 buys line 6's survival on a blue-drifted day and one probe on any day.
  USER_STORY stage 5's lesson, replayed at the identity tier.
- `obs-cloned-boots-are-simons-line` ([HUMAN] lean, 2026-09-16) — the model expresses the
  cloned-boot horizon as a stdlib warrant with a name on it, never as a rule of its own; this
  book never consults it, since its boots are inherited.
