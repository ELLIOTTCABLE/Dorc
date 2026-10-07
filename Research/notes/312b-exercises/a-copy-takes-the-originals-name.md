# 312b-exercises/a-copy-takes-the-originals-name — content-derived names under a copy

Exercise record for `312f`, after the mechanization of 311. It exercises
`a-copy-takes-the-originals-name`, beside `identity-tokens-have-clone-horizons`,
`resolution-is-set-valued`, `effects-land-after-the-command-returns`, and
`daemon-effects-escape-the-traced-process`. The store is real: the Linux block-device naming
layer. Nothing here was executed. Tool behaviour comes from the conductor's general knowledge
and is graded +SURE, ~SUSPECT, -GUESS, or --WONDER where it matters. The 311 walks are hand
walks over the fences at `55571543`; no solver has seen them. Oracle lines are illustrative and
carry no weight for the 312 language.

This record was asked for as a case that is not settled by counting: no position, offset, or
index shifts anywhere below. It is not a ruling and does not recommend a repair. Section 8
lists questions; it does not answer them.

## § 1-the-store-one-partition-many-names

One partition on one host is reachable under several names at once. Each name has its own
authority and its own dependency.

| name | example | what decides it | what moves it |
| --- | --- | --- | --- |
| kernel name | `/dev/sdb1` | the order in which the kernel probes disks | reboot, hotplug, a SCSI delete or rescan |
| device number | `8:17` | the kernel's allocation at probe | the same events; a number is reused after removal |
| by-path | `/dev/disk/by-path/pci-…-ata-2-part1` | the port the disk is attached to | moving the cable |
| by-id | `/dev/disk/by-id/ata-WDC_…-part1` | the drive's model and serial | replacing the drive |
| filesystem UUID, label | `UUID=2f6c…`, `LABEL=DATA` | bytes in the filesystem's own superblock | writing those bytes, on any device |
| LVM names | `/dev/vg0/data` | LVM metadata, and which physical volume LVM selects | metadata writes; duplicate physical volumes |

+SURE: the kernel names, device numbers, and the `/dev/disk/by-*` links exist as described.
udev creates the `by-*` links from probe results. `blkid` and `findfs` resolve `UUID=` and
`LABEL=` specifications by scanning devices. ~SUSPECT on every tie-break detail below.

The filesystem UUID is the name that ops guidance recommends as stable. The Debian installer
writes `UUID=` lines into `/etc/fstab`, and the Arch wiki recommends UUIDs and labels over kernel
names (+SURE). It is also the name that a byte copy duplicates.

## § 2-the-world

Alice runs one host with two disks. `sdb` holds the data partition `sdb1`: ext4, filesystem
UUID `U` (`2f6c4a1e-…`), label `DATA`, mounted at `/data` from an fstab line `UUID=U`. `sdc`
is a spare with one partition, `sdc1`, which holds an unrelated filesystem. Nothing else on the
host carries `U` or `DATA`. No other actor writes to either disk during the book.

## § 3-the-books

### § 3.1-the-copy

```sh
#!/bin/sh
set -eu
dd if=/dev/sdb1 of=/dev/sdc1 bs=4M conv=fsync                       # 1  copy the data partition onto the spare
tune2fs -l "$(findfs UUID=2f6c4a1e-…)" | grep -q '^Maximum mount count: *-1$' \
   || tune2fs -c 0 "$(findfs UUID=2f6c4a1e-…)"                       # 2  no forced fsck on the data filesystem
mountpoint -q /data || mount UUID=2f6c4a1e-… /data                   # 3  /data, by its filesystem UUID
```

Line 1 writes every byte of `sdc1` and reads `sdb1`. `sdb1` is not written. After line 1, two
devices carry `U`. `findfs UUID=U` then returns one of them. The conductor does not know which
one it returns, or whether the choice is stable across calls (~SUSPECT that it depends on
probe order and cache state).

Three things follow, each graded:

- Immediately after a byte copy, the two filesystems answer every content question the same
  way. A content fact measured through `U` at probe time is therefore true of both devices
  right after line 1 (+SURE). The fact is correct here, but only because the two devices
  hold the same bytes.
- If line 2 had been diverged, `tune2fs -c 0` would write whichever device `findfs` returned.
  It could change the copy and leave the original alone (~SUSPECT). A line that runs then
  changes the wrong device, and every plan line still looks honest.
- Line 3 stays converged while `/data` is mounted from `sdb1`. At the next mount by `UUID=U`,
  for example at the next boot, the device that gets mounted depends on the scanner's choice
  (~SUSPECT). Ops reports of a cloned disk booting or mounting in place of the original are
  common (~SUSPECT on frequency).

### § 3.2-the-label-the-smallest-case

```sh
e2label /dev/sdc1 DATA                                               # 1  label the spare
grep -qx 1 "/sys/class/block/$(basename "$(readlink -f /dev/disk/by-label/DATA)")/ro" \
   || blockdev --setro /dev/disk/by-label/DATA                       # 2  the DATA filesystem's device is read-only
```

Line 1 writes a few bytes of `sdc1`'s superblock. It does not touch `sdb1`. After udev
processes the change event, `/dev/disk/by-label/DATA` names one of two devices (~SUSPECT:
udev keeps one link per name; with equal link priority the last device processed wins). Line
2's guard and line 2's mutation both follow the link. One small write to one device moved a
name that another device had held, and no copy was involved.

The change event is handled after `e2label` returns (+SURE that udev processes events
asynchronously; `udevadm settle` is the usual wait). A line that runs immediately after line 1
can still see the old link.

### § 3.3-duplicate-physical-volumes

```sh
dd if=/dev/sdb2 of=/dev/sdc2 bs=4M conv=fsync                       # 1  copy an LVM physical volume onto the spare
lvextend -r -L +1G vg0/data                                          # 2  grow a logical volume in that volume group
```

After line 1, two devices carry one physical-volume UUID. LVM prints a duplicate warning and
uses one of the two (+SURE that duplicate-PV warnings exist; ~SUSPECT on the selection rules,
which consider devices in use, multipath, the `filter` setting, and the `devices` file). Line 2
can then extend the logical volume on the copy. This changes where a later write lands, not
only what a later read sees.

### § 3.4-a-snapshot-and-a-store-that-refuses

```sh
lvcreate -s -n data-snap -L 1G vg0/data                              # 1  snapshot the data volume
mount /dev/vg0/data-snap /mnt/snap                                   # 2  inspect the snapshot
```

A snapshot's filesystem carries its origin's filesystem UUID (+SURE). The `by-uuid` link can
move to the snapshot (~SUSPECT). XFS refuses to mount a second filesystem with a UUID that is
already mounted, unless `-o nouuid` is given (+SURE). That store turns the hazard into a loud
error. Btrfs documentation warns against block-level copies that are visible to one kernel at
the same time, since devices with one filesystem UUID can be taken as members of one filesystem
(~SUSPECT on current kernels, which added mitigations for some cases).

### § 3.5-a-rescan-the-contrast

```sh
echo 1 >/sys/block/sdb/device/delete                                 # 1  detach the data disk
echo '- - -' >/sys/class/scsi_host/host0/scan                        # 2  rescan the adapter
```

After a delete and a rescan, the disk can come back under a different kernel name and device
number (~SUSPECT). Its filesystem UUID and label are unchanged, since no superblock was
written. This is the reverse sensitivity of § 3.1: here the kernel's names move and the
content-derived names stay.

## § 4-actors-and-what-each-can-know

Alice is the admin. Dora describes `dd` and other byte-copying tools. Tessa describes
filesystems. Ulla describes the scanning naming layer: `blkid`, `findfs`, and the udev
`by-*` links. Lev describes LVM. The stdlib describes the kernel's block devices: kernel names
and device numbers. None of them has read another's description.

| knowledge | nearest speaker | what is hard about it |
| --- | --- | --- |
| which bytes a copy writes | Dora | nothing: the target device, whole |
| that a UUID and a label are bytes in the superblock | Tessa | each filesystem type stores them differently; some have several identifiers |
| that a scanned name depends on every device's superblock, absent copies included | Ulla | the dependency is an absence over the whole population |
| how a scanner breaks a tie between duplicates | Ulla, Lev | order, cache, configuration; often undocumented |
| whether duplicates can exist on this host | Alice | a property of this deployment: cloning habits, filters, the `devices` file |
| when the kernel reassigns names and numbers | the stdlib | hotplug, delete, rescan, reboot |

No one party knows the whole chain from `dd` to the moved name. The fact that a content write
can move a name needs two pieces: Tessa's (this name is derived from my content) and Ulla's
(this name is chosen by scanning every device).

Illustrative only, to make the two dependencies concrete:

```sh
# ulla-blkid.oracle.sh — illustrative; no spelling here is proposed
sm_FsUuid__resolve() {
   local dev; dev=$(findfs "UUID=$1") || return 2
   printf 'yields sm.DevNum:%s\n' "$(stat -L -c '%t:%T' -- "$dev")" >>"${DREP_V1:-/dev/null}"
   # The answer depends on every block device's superblock, including devices that do not
   # carry this UUID now. Naming only "$dev" as the dependency and closing would claim more
   # than findfs knows.
}
```

The kernel's device number, by contrast, is the stdlib's primary naming of a block device. In
§ 3.1 nothing writes the kernel's device table, and `8:17` still names `sdb1` after line 1
(+SURE).

## § 5-hand-walks-against-311-as-written

Assumed modelling, which is one choice among several: the device number is the primary MScheme
of a partition MSort, identified in the boot. `UUID=`, `LABEL=`, by-path, and by-id are
secondary MSchemes that yield into it. Their catalog is the boot's block-device population.
Dora's `dd` may-writes the target device given whole and closes its set.

For § 3.1, line 1 against line 2's fact (~SUSPECT, hand walk):

- Line 2 reads through the natural key `U`, whose identity at probe is `8:17`.
- The writeset is `8:33` given whole. `compare(8:33, 8:17)` is DISJOINT, by two device numbers
  of one MScheme under unique-name.
- Token rule (`311:3.3-invalidation-three-mutator-species`): `8:33` is inside the store that
  `8:17` is scoped in, and `compare()` of a key against its own container is UNKNOWN. So
  `8:17`'s token is invalidated, although the kernel did not renumber anything.
- Routing rule: if Ulla's lookup emits no traversal, its traversal is the catalog given whole,
  and `8:33` is in it. `U`'s resolution is invalidated. If Ulla emits only `8:17` and closes,
  the prose meaning of the closing act (`311:1.7.1`: the backing is what can make the key stop
  reaching its object) makes that closure false. The mechanized truth of the closing act reads
  the static `passes` relation and cannot express it.
- Line 2 is not spared, for two separate reasons. Either reason alone suffices today.

For § 3.5, the rescan: line 2 writes a sysfs attribute of the adapter, a key in another
vocabulary. It compares KNOWN_UNSPOKEN against every device number and against Ulla's
catalog. Every fact through any of the names is invalidated (~SUSPECT, hand walk).

Not expressible today: the checker's world gives each MKey at most one MReferent for the whole
book (`reaches: lone`). "`U` reaches `sdb1` before line 1 and either device after it" cannot be
stated with every statement true. The case depends on time, which the fences do not yet hold.

## § 6-what-the-case-strains

Observations, not conclusions:

- `strain-content-write-moves-a-name` — The written device and the previously named device are
  different referents. Neither one's state is what changed for the reader; the name's referent
  did. No counting is involved.
- `strain-the-renamer-is-a-third-store` — The names live in udev's links, `blkid`'s cache,
  LVM's device cache, and the kernel's btrfs device registry. A daemon rewrites them in
  reaction to an event. That store is neither the writer's nor the written store.
- `strain-one-write-two-sensitivities` — On the same two partitions, a copy moves the
  content-derived names and not the kernel's numbers; a rescan moves the kernel's numbers and
  not the content-derived names. One store holds names from schemes with opposite
  dependencies.
- `strain-honest-dependency-is-the-whole-population` — A scanned name is correct only while no
  other device carries it. Its honest dependency is an absence over every device, so an honest
  closure is as wide as the population.
- `strain-winner-is-policy-not-content` — With duplicates, which device a name reaches is
  decided by order, cache, or configuration. That choice is nobody's content and no book line's
  speech.
- `strain-lands-after-return` — udev handles the change event after the writing command exits.
- `strain-renames-route-later-writes` — With duplicate physical volumes, and possibly with
  btrfs, a later write through the name can land on the copy.
- `strain-the-stable-name-is-the-cloneable-one` — The name that guidance calls stable is the
  one that a copy, a snapshot, or a template clone duplicates.

## § 7-candidate-readings-tested-against-the-case

- The two rules that the conductor proposed in `312f` and then withdrew: a write inside a store
  renames the store's members only when the write names the store itself, or while the store's
  describer has not finished its entailment. In § 3.1 the block device store's finished
  record ("a copy writes only its target and what it holds") is true, and `U` moved anyway.
  `U` is a secondary MScheme here, so the moved name is a routing question, not a token
  question. The case does not decide whether a narrowed token rule would be safe for device
  numbers; it shows that a rule decided per store cannot treat device numbers and UUIDs
  differently.
- The idea of the store owner naming its naming map as an ordinary referent (`312f` § 11.2):
  the maps here are udev's links and LVM's cache. They belong to a third party that rewrites
  them in reaction to writes elsewhere. Whose speech would describe them is open.
- Rooting the UUID (`:root`): `311:2.2-primary-of-and-identified-in` already says cloned
  identifiers make `:root` false. This case is that failure, caused by one book line.

## § 8-questions-the-case-raises

- Who may state that a content-derived name depends on the whole population, and what
  statement says it: the scanning layer's describer, the filesystem's describer, or both?
- Should a name's dependency be stated per MScheme rather than per store, given § 3.1 and
  § 3.5? If so, does that hold for primary MSchemes, secondary ones, or both?
- What does the model owe a name whose referent becomes plural mid-book: withdrawal only, or
  also an account of where later writes through it can land?
- Is a store that refuses duplicates (XFS) a seat for anything, or only a fact about its
  failure mode?
- How does the line-varying reach of the time slice express "one referent, then one of two"?
- What can `dorc why` say after § 3.1's line 2 ran on the copy: which device `U` named at
  probe, and which at apply?

## § 9-contrasts-any-answer-must-preserve

- A copy onto any device withdraws the facts read through the copied content-derived names.
- A label write onto another device withdraws the facts read through that label.
- A write to `sdb1` itself withdraws every fact about `sdb1`, by every name.
- A delete or rescan withdraws the facts read through kernel names and device numbers.
- The coarse floor stays available: withdraw every fact about block devices on any write to a
  block device.
- A fact read through `by-path` or a device number has no safety reason in this case to be
  withdrawn by a content write to another device. Whether that value is worth any speech is
  open.

## § 10-limits

Nothing here was run. The tie-break behaviour of `findfs`, udev, and LVM is graded and not
verified against a current source. The hand walks assume one modelling of the block-device
layer; other modellings are possible and are not explored. The checker cannot express the case
until a key's referent can change between lines.
