# 312b-exercises/containment-by-path-prefix-lies — a region bound, an alias at any level, and a sort that places files

Exercise record; ahistorical; the 2026-09-19 to 2026-09-23 arc over `notes/311`. Book lines are
real sh. Oracle lines are strawman spellings and carry no weight; the `#` glosses are prose in
`311` vocabulary. Grades on the conductor's claims: +SURE / ~SUSPECT / -GUESS / --WONDER.
Exercises `containment-by-path-prefix-lies` and, beside it, `a-directory-can-have-two-parents`
and `daemon-effects-escape-the-traced-process`. A rich stdlib is assumed present. What the
model says at the end of the arc is `notes/311` §1.5 (the per-level closure), §2.5 (a
container's entailment), and §2.8 (the region test and `:places`); this record shows the road
to it, the dead ends included, so they are not re-walked.

Spelling rules in force for the strawmen (the sysctl record's, plus this arc's): a bind is an
ordinary `local x="…"` with a trailer, never an inline form; a runtime value with a type is one
record line to the report lane; a trailer's `$name` is a reference to a `local` declared earlier
in the body, never shell; records are `may-read`, `may-write`, `yields`, `looked-up-in`,
`identified-in`, `alias`, each closed by its own `nothing-else`; every selector and sort name is
full reverse-DNS; a sort's own speech is a `__declaration()` member with `: :` trailers, and the
`places` trailer and the `__member_of()` member are provisional names.

## The world

A directory is an inode whose contents are entries, each a name for another inode. One inode
may have several entries, in one directory or in several: a hardlink. A directory itself may
answer at two paths: a bind mount gives it a second mount, and HFS+ hardlinks whole directories
for Time Machine; Linux's on-disk tree refuses only the literal `link(2)` of a directory. A mount
namespace is a private mount table; a daemon started by a snap or a systemd unit with its own
mount namespace sees a different table than a login shell does. Docker keeps its images, layers,
and volumes under a data root, `/var/lib/docker` by default, often relocated by a symlink or a
mount; a named volume of type `bind` makes a host directory appear under the data root; a
container with a bind mount writes host directories directly. dpkg records which package owns a
file (`dpkg -S`) and which files a package owns (`dpkg -L`); a regular file has at most one owner,
a directory may be listed by many packages, and `dpkg-divert` lets two packages provide one path
with one of them diverted. (+SURE all.)

## The books, the catastrophes, the truths

Book A, on a single-root host:

```sh
#!/bin/sh
set -eu
docker pull registry.example/app:stable            # 3   drifted on this morning: a new digest
cp ./app.env /etc/app/app.env                      # 4   converged: content match
chmod 640 /etc/app/app.env                         # 5   converged
docker compose -f /srv/app/compose.yml up -d       # 6   unmodelled verb: runs, walls
```

Book B, the same host, an nginx upgrade available:

```sh
apt-get install -y nginx                           # 3   installed but outdated, so it upgrades
cp ./nginx.conf /etc/nginx/nginx.conf              # 4   converged
cp ./app.env /etc/app/app.env                      # 5   converged
```

Four variants of the host, each true somewhere: (i) `ln /etc/app/app.env /var/lib/docker/x`
gives the file a second entry under the data root; (ii) `mount --bind /etc/app
/var/lib/docker/volumes/cfg/_data` gives its directory a second path under the data root;
(iii) `/var/lib/docker` is a symlink to `/mnt/data/docker`; (iv) dockerd runs in a mount
namespace of its own.

Catastrophes (`311b:rul-ground-identity-in-final-outcomes`): C1, wrong DISJOINT, line 4 of
book A survives line 3 under variant (i) or (ii) while the file sits beneath the data root by
another name; C2, wrong DISJOINT, line 4 of book B survives the upgrade while nginx owns that
file; C3, wrong DISJOINT, line 4 of book A survives line 3 under variant (iv) while dockerd
writes a tree the book's mount namespace does not show. V, the value failure that is not a
catastrophe: every file fact on the host walls behind every region bound, so book A's lines 4
and 5 guard on every drifted morning.

Truths: on the plain host, line 4 and line 5 of book A are not reached through the data root
and survive line 3; under (i) and (ii) line 4 is reached through it and does not; in book B
line 4 is nginx's and does not survive, line 5 is nobody's and does.

## The actors

Alice writes both books. The stdlib is present from the start: Tessa owns files and paths
(`tessa-file.oracle.sh`: `sm.File`, primary `sm.Inode`; the secondary `sm.Path` and its
entries), Simon owns filesystems, mounts, and mount namespaces (`simon-mounts.oracle.sh`:
`sm.Filesystem`, `sm.MountNamespace`, `sm.Boot`), and Pavel owns packages
(`pavel-apt.oracle.sh`: `sm.Package`, primary `sm.PackageName`). Dmitri writes the docker
oracle from outside the stdlib (`dmitri-docker.oracle.sh`). Nobody has met.

## The floor: a key given whole

Dmitri's first oracle names the data root and nothing finer:

```sh
# dorc-lang/v0.2   dmitri-docker.oracle.sh
dmitri__root() {                                 # never hardcoded: a relocated data root is common (variant iii)
   [ -z "${DOCKER_HOST-}" ] || return 2          # a remote daemon: its path means nothing here (an-env-variable-selects-the-referent)
   docker info -f '{{.DockerRootDir}}'
}
docker__is_converged() {
   [ "${1-}" = pull ] && [ $# -eq 2 ] || return 2
   local root; root=$(dmitri__root) || return 2
   local ref="$2"   : is "dmitri.ImageRef" identified-in "sm.Path:$root"     # hung on something comparable, or nothing of his spares
   docker image inspect -- "$ref" >/dev/null 2>&1   : asserts "dmitri.ImageRef:$ref"
}
docker__may_write() {                            # the verb's Writeset, per matched shape
   case "${1-}" in
   pull) local root; root=$(dmitri__root) || return 2
         printf 'may-write dmitri.ImageRef:%s\n' "$2"   >>"${DREP_V1:-/dev/null}"
         printf 'may-write sm.Path:%s\n'         "$root" >>"${DREP_V1:-/dev/null}"   # a key given whole: the directory and everything reached through it
         printf 'may-write nothing-else\n'               >>"${DREP_V1:-/dev/null}" ;;
   esac                                          # run, compose up, exec: no arm; a bind mount lets a container write anywhere
}
```

The bare key is deep by ruling: it denotes the directory's mReferent and every mReferent whose
identity chain or access path passes through it (`311` §2.8, the sentence folded 2026-09-19).
That is safe and it is V: with nothing more said, the region collides with every mKey the path
mScheme can yield in the mount namespace, and lines 4 and 5 of book A guard behind line 3. Under
`311` as written before this arc the floor was worse than V, not better: a directory inode
compared with a file inode reads DISJOINT as two siblings in one filesystem, which says only
that the file is not the directory, and line 4 was spared under every variant. The folded
sentence closed that first.

## Dead end one: the prefix

"It touches only that directory and below" is a sentence about paths. The fact is held by inode.
Sparing line 4 needs "inode 8812 is not reachable under `/var/lib/docker`", and no two path
strings settle that: variant (i) gives the inode a second name under the root, (ii) gives its
directory one, (iii) moves the root's real place, and a rename moves an inode across the
boundary with no write to it. The engine never reads an mKey's syntax (`311` §2.8), so it
cannot even ask the string question; a lookup could, and would be wrong. `containment-by-path-
prefix-lies`, exactly. Dead.

## Dead end two: the directory as a store

Identify files in their directories, so that the data root is an ancestor of everything
beneath it and §3.2 answers. Dead on the model's own terms: a directory gives its own names to
mReferents of its filesystem, and one inode may have entries in two directories, so a directory
is the kind of store that can never honestly say `:aliases-nothing-else`, and a silent store
separates nothing. A directory boundary has no arbiter. `/proc` carves cleanly because it is a
mount, which the kernel enforces and labels in the mount table; that is also why variant (iii)
with a real mount at `/var/lib/docker`, the common advice, buys value with no authoring at all.

## Dead end three: the leaf alone

Check the file: link count one, and its one entry's chain avoids the root. Under variant (ii)
that passes, and it is wrong: the file's directory answers at a second path beneath the root,
and nothing about the file shows it. `a-directory-can-have-two-parents` is this at the
directory level, on HFS+ by hardlink and on any Linux by bind mount. The alias may sit at any
level of the path, and only that level's own lookup can see it. A check that stops at the leaf
cannot be made sound by any care at the leaf. Dead, and it is what "every level" means in the
rule.

## The repair, part one: the chain the lookups produce, and the closure at every level

Tessa's path lookup yields one entry and names the shorter path it was looked up in; the engine
runs the same lookup on that path, and so on to the root. The splitting is `dirname` in
Tessa's body. Each level emits, where it can measure it, that its mReferent is reachable by
exactly this one entry.

```sh
# dorc-lang/v0.2   tessa-file.oracle.sh
sm_File__declaration() {
   : : primary-scheme "sm.Inode"
}
sm_Path__resolve() {                             # a path is one entry looked up in a shorter path; the splitting is here, never in the engine
   case "$1" in
   /)  printf 'yields sm.RootDirectory:/\n'                          >>"${DREP_V1:-/dev/null}" ;;
   /*) printf 'yields sm.DirEntry:%s\n'   "$(basename -- "$1")"      >>"${DREP_V1:-/dev/null}"
       printf 'looked-up-in sm.Path:%s\n' "$(dirname  -- "$1")"      >>"${DREP_V1:-/dev/null}" ;;
   *)  return 2 ;;                               # relative: the cwd is the catalog, supplied by the ambient seat, not this arm
   esac
}
sm_DirEntry__resolve() {                         # $1 the entry name; $2 the directory it is looked up in, resolved (the ABI the sysctl record implied)
   local st; st=$(stat -c '%i %d %h' -- "$2/$1") || return 2
   local ino="${st%% *}"; st="${st#* }"; local dev="${st%% *}"; local links="${st#* }"
   printf 'yields sm.Inode:%s\n' "$ino"                  >>"${DREP_V1:-/dev/null}"
   printf 'identified-in sm.Filesystem:%s\n' "$dev"      >>"${DREP_V1:-/dev/null}"   # the yield seat supplies the store
   if tessa__links_are_honest "$dev" && [ "$links" -eq 1 ] && ! tessa__another_mount_exposes "$2/$1"
   then printf 'alias nothing-else\n'                    >>"${DREP_V1:-/dev/null}"   # exactly this one entry, in this mount namespace
   fi                                            # else: no closure at this level; the region test reads unknown
}
# tessa__links_are_honest:      `stat -f -c %T` is on a short allowlist (ext, xfs, btrfs, tmpfs); FUSE, NFS, and FAT decline.
# tessa__another_mount_exposes: in /proc/self/mountinfo, more than one mount of this device has a root that
#                               prefixes the entry's in-filesystem path; a bind mount of any directory on the route is two.
```

Read what each line carries in `311`'s terms. `looked-up-in` is the routing parent, one per
mKey, supplied by the mKey's own lookup (§1.6). `alias nothing-else` is the thing's-end
statement at the routing level, per mKey, measured on the path that emitted it (§1.5); it is a
statement separate from Simon's per-shape `:guarantees-unique-name` on inodes, and how the two
license together is not this arc's. A mountpoint entry yields into Simon's `sm.Mounted` instead
of `sm.Inode`, one more level owned by another author; the record does not spell it.

The region test (`311` §2.8), over the mTraversal these lookups produced, leaf first, for the
data root D against a fact's mKey x: x's leaf SAME with D, SAME; any level SAME with D, D's
region covers x, UNKNOWN; every level DISJOINT with D and every level closed, DISJOINT; otherwise
UNKNOWN. Only x's chain is walked; D needs no closure. On the plain host, book A line 4:

- `app.env`, inode 8812, against D: two inodes in one filesystem under Simon's unique-name,
  DISJOINT; link count one, closure present.
- `/etc/app` against D: DISJOINT; no other mount exposes it, closure present.
- `/etc` against D: DISJOINT, closed. `/` against D: DISJOINT, closed.

Outside. The region entry never meets the fact's entry, and line 4 stays out of the plan. Line
5 likewise. Under variant (i) the leaf's link count is two, the closure is absent, UNKNOWN,
line 4 guards. Under variant (ii) the leaf is clean and the `/etc/app` level finds a second
mount, UNKNOWN, line 4 guards. Under variant (iii) `dmitri__root` reads the real root, and the
walk is the same. Alice wrote nothing in any case.

## The repair, part two: the daemon in its own mount namespace

Variant (iv) is `daemon-effects-escape-the-traced-process`. The bound `may-write sm.Path:$root`
names a path in the book's mount namespace, the catalog instance the ambient seat supplied
(§1.10). dockerd writes from its own. Under a snap or a hardened unit its mount table differs,
and the path Dmitri read from `docker info` names a directory in a table the book never sees.
The region test then compares the wrong D, and its DISJOINT is about the wrong thing: C3.
Nothing in the model detects this for him, since the engine never holds a mount table; the seat
is Dmitri's, and the honest arm measures:

```sh
dmitri__same_mount_namespace() {                 # the bound names a path in THIS mount namespace; dockerd must write from the same one
   [ "$(readlink /proc/self/ns/mnt)" = "$(readlink "/proc/$(cat /var/run/docker.pid)/ns/mnt")" ]
}
# in docker__may_write, arm pull, before the records:
#        dmitri__same_mount_namespace || return 2       # another namespace: decline; the pull walls as before this arc
```

A decline is the floor, V, and correct. A better arm would resolve the root inside dockerd's
namespace (`nsenter --mount=/proc/$pid/ns/mnt`) and name the mKey there; that is Simon's
`sm.MountNamespace` as a catalog instance and is not spelled here.

## The repair, part three: a sort that places files

Book B has no directory. Nothing is named through a package, so line 4's chain has no package
level, and `may-write sm.Package:nginx` given whole meets it as known-unspoken: collide, V again,
every file fact on the host behind every package bound. Pavel, who describes packages, publishes
the upward lookup, declared against the sort it places:

```sh
# dorc-lang/v0.2   pavel-apt.oracle.sh
sm_Package__declaration() {
   : : primary-scheme "sm.PackageName" identified-in "sm.DpkgDatabase:self"
   : : places "sm.File"                          # this sort can say, of a file, which package it is in
}
sm_Package__member_of() {                        # the placing lookup: invoked with a File key's value; the arms decide which spellings it answers
   case "$1" in
   /*) local owner; owner=$(dpkg -S -- "$1" 2>/dev/null) || return 0          # unowned: the closure alone
       case "$owner" in *"diverted by"*|*", "*) return 0 ;; esac                # a diversion, or a directory several packages list: no closure
       printf 'looked-up-in sm.Package:%s\n' "${owner%%:*}" >>"${DREP_V1:-/dev/null}"
       printf 'looked-up-in nothing-else\n'                  >>"${DREP_V1:-/dev/null}" ;;
   *)  return 2 ;;                               # an inode number: dpkg cannot be asked
   esac
}
sm_Package__may_write_entailment() {             # the store's end, as write reach: USER_STORY stage 7's dpkg -L, unchanged
   dpkg -L -- "$1"   : may-write sm.File
   printf 'may-write nothing-else\n' >>"${DREP_V1:-/dev/null}"
}
```

What the engine does with it (`311` §2.8, the last paragraph): a bound names an mKey of
`sm.Package` given whole; line 4's fact reads an mKey of `sm.File`; that mKey has no route of
mSort `sm.Package`; `sm.Package` declares it places `sm.File`; so the engine invokes Pavel's
lookup, in the probe pass, with every mKey it holds for that mReferent, the path Alice bound
and the inode it yielded, and the arms answer for the path. Line 4 gets `looked-up-in
sm.Package:nginx`, SAME with the bound, covered, collide. Line 5 gets the closure alone,
nothing meets the bound, survive. The route is a mTraversal for the region test and never the
mKey's parent, which is the route Tessa's own lookup supplied. A write to dpkg's database, the
upgrade itself, perishes the route (§3.3), and the next probe re-resolves it.

Three shapes were tried for this lookup before the one above and are recorded in `311` §4.2.
A second mScheme of files that accepts `sm.Path` values and is looked up in the package: killed,
because mSchemes stay singular and a file's package is a relation between two mReferents, not a
spelling of the file. The downward list alone: sound, in the model already, and one `dpkg -L`
per package named in a bound against every file fact, which a `dist-upgrade` makes expensive.
The one kept is a body on the placing sort, invoked with the placed mKey's mValue, whose arms
carry the input's shape where every other body carries it.

## The write side: a container's own entailment

The same arc found the write side of containment. Simon's loop filesystem L is mounted from
`disk.img`, inode 100 on the outer filesystem O. Alice's book:

```sh
sha256sum -c /srv/disk.img.sha256          # 4  converged: the checksum of inode 100 holds
cp ./payload /mnt/img/payload              # 6  drifted, runs: writes inode 7 in L
```

Line 6's Writeset is {inode 7 in L}. The walk reads inode 7 and inode 100 as DISJOINT once loop0
and sda are keyed in one block-device scheme and both filesystems honestly claim
`:aliases-nothing-else`, and line 4 survives a write that changed the bytes it checks. The
first repair tried was a rule over may-read, that a write to an mKey is a write to its
containers' may-read entries; it is in `311` §4.2 as killed, by a resolver cache that honestly
may-reads `/etc/hosts` and whose entries a query populates without ever writing that file. The
repair kept is `311` §2.5: a write inside a container is a write to that container, so the
container's own may-write entailment joins the line's Writeset, below the level both mKeys
share. Simon says it of loop filesystems and nobody says it of caches:

```sh
# dorc-lang/v0.2   simon-mounts.oracle.sh
sm_LoopFilesystem__may_write_entailment() {     # writing anything in me writes my image
   printf 'may-write sm.Inode:%s\n' "$image"    >>"${DREP_V1:-/dev/null}"
   printf 'may-write nothing-else\n'            >>"${DREP_V1:-/dev/null}"
}
```

Line 6's Writeset gains inode 100, SAME with line 4's entry, collide. A loop filesystem whose
describer declared no entailment has an unbounded set, and writes inside it collide with
everything; one whose describer closed the set without the image wrote a false `nothing-else`,
attributed. Silence still spares nothing.

## Dangers

Every closure in this record is a knife held by the one party who can measure or know it, and
every positive record only adds collisions.

- Tessa's `alias nothing-else` is false where the filesystem lies about link counts (FUSE, NFS,
  FAT) or where directories have several parents (HFS+); her allowlist arm declines there, and
  a describer who copies the closure without the arm licenses C1.
- Pavel's `looked-up-in nothing-else` is false on a diverted file and on a directory several
  packages list; his arm declines there.
- Dmitri's `may-write nothing-else` on `pull` is true; the same closure on `run` or `compose
  up` is false in the world, since a bind mount lets the container write anywhere, and no
  model rescues it. He writes no arm for those verbs, and they wall.
- Dmitri's bound in the wrong mount namespace (variant iv) is C3, and the model cannot see it;
  the measuring arm is his.
- Two regions against each other, `may-write sm.Path:/var/lib/docker` against a fact's
  Readset naming `/srv/backup` whole, stay unknown unless one owner enumerates; the region test
  is asked only of a thing's own chain.

## Cost, briefly

Host reads are what count. For a file fact under a directory region, a `stat` per level of its
path and one scan of `mountinfo`, deduplicated per path across every bound. For a file fact
under a package bound, one `dpkg -S`, which scans every installed package's file list, tens of
milliseconds; the downward `dpkg -L` is a millisecond per package and cheaper when few packages
face many files. Both ride the one probe pass, and the engine's choice between them where both
exist is probe planning below this model. Controller work is a table hit and a chain compare
per pair.

## Observations

- `obs-a-prefix-is-a-naming-convention-and-a-mount-is-a-boundary` (+SURE) — the crossings the
  prefix test cannot see are each labelled on the thing crossed (a link count; the mount table),
  so membership is measurable and never a warrant; a directory cannot say
  `:aliases-nothing-else`, a mount can, which is why `/proc` carves and `/var/lib/docker` does
  not until it is a mount.
- `obs-the-alias-may-sit-at-any-level` (+SURE) — the leaf's own closure cannot see a second
  parent of its directory; every level of the chain is asked, and only that level's lookup can
  answer for it.
- `obs-the-engine-never-splits-a-path` (+SURE of the text) — the mTraversal is the chain
  Tessa's lookups produced, one level per lookup, and the model has no sentence about key
  syntax left.
- `obs-a-sort-places-what-nothing-is-named-through` (~SUSPECT of the shape; the siting acked
  "mostly") — a package, a cgroup, a controller, a VPC: the placed thing's chain has no level in
  the grouping, the grouping's owner publishes the upward lookup, its records are the route
  records any lookup emits, and its route is never a parent. The relation's name `:places` is
  provisional.
- `obs-every-sort-walls-until-mapped-into-a-shared-key-space` (the human's reading, +SURE of
  the default) — a bound in one sort against a fact of another collides unless the bound's
  describer translates it into a key space both sides share, by the write-path row (what writing
  it writes) or by the route row (membership); the stdlib's key spaces are those spaces.
- `obs-the-write-side-is-the-containers-own-entailment` (+SURE of the counterexample) — a
  may-read entry is influence toward the container and says nothing about what a write inside
  it lands on; the cache and the loop filesystem are indistinguishable under any rule over
  may-read and separate under the container's may-write.
- Open, by the arc's own record: the identity-level thing's-end closure; the licensing law
  relating the store's per-shape warrant and the thing's per-level closure; two regions against
  each other; the placing lookup's name. None is spelled here.
