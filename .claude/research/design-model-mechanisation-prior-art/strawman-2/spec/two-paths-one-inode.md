# Two paths, one inode

Two file paths that name one inode must never be told apart, and two that name two inodes in one filesystem, or in two filesystems of one boot, may be. This document holds the speech that decides it, the rules that consume the speech, and the books that pin the answers.

Tessa writes the file describer.
Simon writes the filesystem describer.
Carl writes the chmod describer.
The stdlib describes the boot.
Foob writes a certificate tool nobody here has heard of.
The world is the host under test and speaks only through measures.

## § 1 The question

```alloy
one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Ans {}
fact { weaker = UNKNOWN->SAME + UNKNOWN->DISJOINT + UNKNOWN->KNOWN_UNSPOKEN + KNOWN_UNSPOKEN->DISJOINT }
fact { Safe = UNKNOWN + KNOWN_UNSPOKEN }
fact { Spares = DISJOINT }
```

A line is spared past an earlier line only when every pair of a written key and a read key answers in `Spares`. Whatever is not in `Spares` collides.

## § 2 Ways of writing, and the things they reach

```alloy
sig MReferent {}
sig MValue {}
abstract sig MSort {}
abstract sig MScheme { yields: lone MScheme, primaryOf: lone MSort }
sig MKey { scheme: one MScheme, value: one MValue, parent: lone MKey, resolvesTo: lone MKey, reaches: one MReferent }
fact { no k: MKey | k in k.^parent }
fact { all k: MKey | some k.resolvesTo implies k.reaches = k.resolvesTo.reaches }

sig Yields extends MDecl { from, to: one MScheme }
fact { all d: Yields & True | d.from.yields = d.to }

sig PrimaryOf extends MDecl { ofScheme: one MScheme, ofSort: one MSort }
fact { all d: PrimaryOf & True | d.ofScheme.primaryOf = d.ofSort }

sig IdentifiedIn extends MDecl { ofScheme: one MScheme, within: one MSort }
fact { all d: IdentifiedIn & True, k: MKey | k.scheme = d.ofScheme and some k.parent implies k.parent.scheme.primaryOf = d.within }

sig GuaranteesUniqueReferent extends MDecl { on: one MScheme }
fact { all d: GuaranteesUniqueReferent & True, a, b: MKey | a.scheme = d.on and b.scheme = d.on and a.parent = b.parent and a.value = b.value implies a.reaches = b.reaches }

sig GuaranteesUniqueName extends MDecl { on: one MScheme }
fact { all d: GuaranteesUniqueName & True, a, b: MKey | a.scheme = d.on and b.scheme = d.on and a.parent = b.parent and a.reaches = b.reaches implies a = b }

sig Root extends MDecl { on: one MScheme }
fact { all d: Root & True, a, b: MKey | a.scheme = d.on and b.scheme = d.on and a.value = b.value implies a.reaches = b.reaches }

one sig tessa__a_path_names_one_entry_per_component extends Yields {} { speaker = tessa  from = Path  to = Entry }
one sig tessa__an_entry_names_one_inode extends Yields {} { speaker = tessa  from = Entry  to = Inode }
one sig tessa__an_inode_is_the_primary_key_of_a_file extends PrimaryOf {} { speaker = tessa  ofScheme = Inode  ofSort = File }
one sig tessa__a_file_is_identified_in_its_filesystem extends IdentifiedIn {} { speaker = tessa  ofScheme = Inode  within = Filesystem }
one sig tessa__equal_inodes_in_one_filesystem_reach_one_file extends GuaranteesUniqueReferent {} { speaker = tessa  on = Inode }
one sig tessa__a_file_has_one_inode_in_its_filesystem extends GuaranteesUniqueName {} { speaker = tessa  on = Inode }

one sig simon__a_device_number_is_the_primary_key_of_a_filesystem extends PrimaryOf {} { speaker = simon  ofScheme = DeviceNumber  ofSort = Filesystem }
one sig simon__a_filesystem_is_identified_in_the_boot_that_mounted_it extends IdentifiedIn {} { speaker = simon  ofScheme = DeviceNumber  within = Boot }
one sig simon__a_filesystem_has_one_device_number_in_its_boot extends GuaranteesUniqueName {} { speaker = simon  on = DeviceNumber }

one sig stdlib__a_boot_id_is_the_primary_key_of_a_boot extends PrimaryOf {} { speaker = stdlib  ofScheme = BootId  ofSort = Boot }
one sig stdlib__a_boot_id_is_a_root extends Root {} { speaker = stdlib  on = BootId }
```

Tessa says nothing about whether two paths may name one inode. The name warrant on `Path` is absent, and its absence is her statement.

## § 3 A store that exposes nothing else

```alloy
sig AliasesNothingElse extends MDecl { store: one MSort }
fact { all d: AliasesNothingElse & True, k, k2: MKey | k.parent.scheme.primaryOf = d.store and k2.reaches = k.reaches implies k2.parent = k.parent }

one sig simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem extends AliasesNothingElse {} { speaker = simon  store = Filesystem }
```

Simon can say this because he describes the filesystem. Tessa cannot, because an inode number carries nothing about the filesystem's kind.

## § 4 What a verb writes and what its check reads

```alloy
sig Verb {}
sig Site { verb: one Verb, arg: one MKey }

sig MayWrite extends MDecl { verb: one Verb }
sig ChecksRead extends MDecl { verb: one Verb }
fun writesOf[S: set MDecl, s: Site]: set MKey { (some d: MayWrite & S | d.verb = s.verb) implies s.arg else MKey }
fun readsOf[S: set MDecl, s: Site]: set MKey { (some d: ChecksRead & S | d.verb = s.verb) implies s.arg else MKey }

one sig carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else extends MayWrite {} { speaker = carl  verb = chmod }
one sig carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode extends ChecksRead {} { speaker = carl  verb = chmod }
```

A verb nobody has described writes every key and reads every key.

## § 5 Compare

```alloy
sig Query { w, r: one Site }

fun ident[k: MKey]: lone MKey { k.*resolvesTo & scheme.(primaryOf.MSort) }
fun chain[k: MKey]: set MKey { k.*parent }
pred rooted[S: set MDecl, k: MKey] { all t: chain[k] | some t.parent or some d: Root & S | d.on = t.scheme }
fun top[x, A: MKey]: one MKey { chain[x] & parent.A }

pred separated[S: set MDecl, x, y: MKey] {
   some A: x.^parent & y.^parent | no (x.^parent & y.^parent - A.*parent) and
   let tx = top[x, A], ty = top[y, A] |
      tx.scheme = ty.scheme and tx.value != ty.value
      and (some d: GuaranteesUniqueName & S | d.on = tx.scheme)
      and (all s: (x.^parent + y.^parent) - A.*parent | some d: AliasesNothingElse & S | d.store = s.scheme.primaryOf)
}

fun answerKeys[S: set MDecl, x, y: MKey]: one Ans {
   (no ident[x] or no ident[y]) implies UNKNOWN
   else (not rooted[S, ident[x]] or not rooted[S, ident[y]]) implies UNKNOWN
   else separated[S, ident[x], ident[y]] implies DISJOINT
   else UNKNOWN
}
fun answer[S: set MDecl, q: Query]: one Ans {
   (all x: writesOf[S, q.w], y: readsOf[S, q.r] | answerKeys[S, x, y] = DISJOINT) implies DISJOINT else UNKNOWN
}
pred wrong[S: set MDecl, q: Query] { answer[S, q] = DISJOINT and some x: writesOf[S, q.w], y: readsOf[S, q.r] | x.reaches = y.reaches }
fun restsOn[S: set MDecl, q: Query]: set MDecl { { d: S | answer[S - d, q] != answer[S, q] } }
```

A chain that ends at a key nobody has declared a root spares nothing.

## § 6 Who may say what

<!-- normative: -->
> A claim is made by the seat that can know it.
> Whether a filesystem exposes its inodes through another filesystem is the filesystem describer's knowledge.
> Whether a file has one inode is the file describer's knowledge.
> The engine chains and meets; it originates no claim.
> Every survival names the claims it rested on and no others.

### § 6.1 The seat, derived

```alloy
fun sortOwner[s: MSort]: set Speaker { ((PrimaryOf & True) & ofSort.s).speaker }
fun schemeOwner[s: MScheme]: set Speaker { ((Yields & True) & from.s).speaker + ((PrimaryOf & True) & ofScheme.s).speaker }

check seatCanKnow {
   all d: AliasesNothingElse | d.speaker in sortOwner[d.store]
   all d: IdentifiedIn | d.speaker in schemeOwner[d.ofScheme]
   all d: GuaranteesUniqueName + GuaranteesUniqueReferent + Root | d.speaker in schemeOwner[d.on]
} for 12
```

## § 7 The books

```sh
#}= stat -c '%i %d' a-path b-path                                  #=> a-path.resolvesTo = inode-x, b-path.resolvesTo = inode-x, inode-x.parent = fs-1
stat -c '%i %d' /srv/a/shared /srv/b/shared
#}= cat /proc/sys/kernel/random/boot_id                            #=> fs-1.parent = boot-1
cat /proc/sys/kernel/random/boot_id

#} chmod g-w a-path                                                #=> run
chmod g-w /srv/a/shared
#} chmod g+w b-path                                                #=> guard
chmod g+w /srv/b/shared
```

```sh
#}= stat -c '%i %d' a-path c-path                                  #=> a-path.resolvesTo = inode-x, c-path.resolvesTo = inode-y, inode-x.parent = fs-1, inode-y.parent = fs-1
stat -c '%i %d' /srv/a/shared /srv/a/other
#}= cat /proc/sys/kernel/random/boot_id                            #=> fs-1.parent = boot-1
cat /proc/sys/kernel/random/boot_id

#} chmod g-w a-path                                                #=> run
chmod g-w /srv/a/shared
#} chmod g+w c-path                                                #=> elide by tessa__an_inode_is_the_primary_key_of_a_file tessa__a_file_has_one_inode_in_its_filesystem stdlib__a_boot_id_is_a_root carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode
chmod g+w /srv/a/other
```

```sh
#}= stat -c '%i %d' a-path d-path                                  #=> a-path.resolvesTo = inode-x, d-path.resolvesTo = inode-z, inode-x.parent = fs-1, inode-z.parent = fs-2
stat -c '%i %d' /srv/a/shared /var/lib/other
#}= cat /proc/sys/kernel/random/boot_id                            #=> fs-1.parent = boot-1, fs-2.parent = boot-1
cat /proc/sys/kernel/random/boot_id

#} chmod g-w a-path                                                #=> run
chmod g-w /srv/a/shared
#} chmod g+w d-path                                                #=> elide by tessa__an_inode_is_the_primary_key_of_a_file simon__a_device_number_is_the_primary_key_of_a_filesystem simon__a_filesystem_has_one_device_number_in_its_boot simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem stdlib__a_boot_id_is_a_root carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode
chmod g+w /var/lib/other
```

## § 8 The same lines under less speech, a stranger's speech, and other speech

```sh
. ./tessa__a_path_names_one_entry_per_component.sh
. ./tessa__an_entry_names_one_inode.sh
. ./tessa__an_inode_is_the_primary_key_of_a_file.sh
. ./tessa__a_file_is_identified_in_its_filesystem.sh
. ./tessa__equal_inodes_in_one_filesystem_reach_one_file.sh
. ./tessa__a_file_has_one_inode_in_its_filesystem.sh
. ./simon__a_device_number_is_the_primary_key_of_a_filesystem.sh
. ./simon__a_filesystem_is_identified_in_the_boot_that_mounted_it.sh
. ./simon__a_filesystem_has_one_device_number_in_its_boot.sh
. ./stdlib__a_boot_id_is_the_primary_key_of_a_boot.sh
. ./stdlib__a_boot_id_is_a_root.sh
. ./carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else.sh
. ./carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode.sh

#}= stat -c '%i %d' a-path d-path                                  #=> a-path.resolvesTo = inode-x, d-path.resolvesTo = inode-z, inode-x.parent = fs-1, inode-z.parent = fs-2
stat -c '%i %d' /srv/a/shared /var/lib/other
#}= cat /proc/sys/kernel/random/boot_id                            #=> fs-1.parent = boot-1, fs-2.parent = boot-1
cat /proc/sys/kernel/random/boot_id

#} chmod g-w a-path                                                #=> run
chmod g-w /srv/a/shared
#} chmod g+w d-path                                                #=> guard
chmod g+w /var/lib/other
```

Without Simon's closure, two files in two filesystems collide. Nothing was false; nobody said the filesystems were compartments.

```sh
. ./tessa__a_path_names_one_entry_per_component.sh
. ./tessa__an_entry_names_one_inode.sh
. ./tessa__an_inode_is_the_primary_key_of_a_file.sh
. ./tessa__a_file_is_identified_in_its_filesystem.sh
. ./tessa__equal_inodes_in_one_filesystem_reach_one_file.sh
. ./tessa__a_file_has_one_inode_in_its_filesystem.sh
. ./simon__a_device_number_is_the_primary_key_of_a_filesystem.sh
. ./simon__a_filesystem_is_identified_in_the_boot_that_mounted_it.sh
. ./simon__a_filesystem_has_one_device_number_in_its_boot.sh
. ./simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem.sh
. ./stdlib__a_boot_id_is_the_primary_key_of_a_boot.sh
. ./carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else.sh
. ./carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode.sh

#}= stat -c '%i %d' a-path c-path                                  #=> a-path.resolvesTo = inode-x, c-path.resolvesTo = inode-y, inode-x.parent = fs-1, inode-y.parent = fs-1
stat -c '%i %d' /srv/a/shared /srv/a/other
#}= cat /proc/sys/kernel/random/boot_id                            #=> fs-1.parent = boot-1
cat /proc/sys/kernel/random/boot_id

#} chmod g-w a-path                                                #=> run
chmod g-w /srv/a/shared
#} chmod g+w c-path                                                #=> guard
chmod g+w /srv/a/other
```

Without the boot declared a root, every chain ends at a key nobody vouched for, and two inodes in one filesystem collide.

```alloy
one sig foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle extends PrimaryOf {} { speaker = foob  ofScheme = BundlePath  ofSort = CertificateBundle }
```

```sh
. ./foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle.sh
. ./tessa__a_path_names_one_entry_per_component.sh
. ./tessa__an_entry_names_one_inode.sh
. ./tessa__an_inode_is_the_primary_key_of_a_file.sh
. ./tessa__a_file_is_identified_in_its_filesystem.sh
. ./tessa__equal_inodes_in_one_filesystem_reach_one_file.sh
. ./tessa__a_file_has_one_inode_in_its_filesystem.sh
. ./simon__a_device_number_is_the_primary_key_of_a_filesystem.sh
. ./simon__a_filesystem_is_identified_in_the_boot_that_mounted_it.sh
. ./simon__a_filesystem_has_one_device_number_in_its_boot.sh
. ./simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem.sh
. ./stdlib__a_boot_id_is_the_primary_key_of_a_boot.sh
. ./stdlib__a_boot_id_is_a_root.sh
. ./carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else.sh
. ./carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode.sh

#}= stat -c '%i %d' a-path d-path                                  #=> a-path.resolvesTo = inode-x, d-path.resolvesTo = inode-z, inode-x.parent = fs-1, inode-z.parent = fs-2
stat -c '%i %d' /srv/a/shared /var/lib/other
#}= cat /proc/sys/kernel/random/boot_id                            #=> fs-1.parent = boot-1, fs-2.parent = boot-1
cat /proc/sys/kernel/random/boot_id

#} chmod g-w a-path                                                #=> run
chmod g-w /srv/a/shared
#} chmod g+w d-path                                                #=> elide by tessa__an_inode_is_the_primary_key_of_a_file simon__a_device_number_is_the_primary_key_of_a_filesystem simon__a_filesystem_has_one_device_number_in_its_boot simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem stdlib__a_boot_id_is_a_root carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode
chmod g+w /var/lib/other
```

Foob's speech about a sort nobody else uses changes nothing, and the survival names none of it.

```alloy
one sig tessa__a_singly_linked_path_names_one_entry_per_component_and_declines_a_path_whose_inode_has_another_entry extends Yields {} { speaker = tessa  from = SinglyLinkedPath  to = Entry }
one sig tessa__a_singly_linked_path_names_one_file extends GuaranteesUniqueName {} { speaker = tessa  on = SinglyLinkedPath }
```

```sh
. ./tessa__a_singly_linked_path_names_one_entry_per_component_and_declines_a_path_whose_inode_has_another_entry.sh
. ./tessa__a_singly_linked_path_names_one_file.sh
. ./tessa__an_entry_names_one_inode.sh
. ./tessa__an_inode_is_the_primary_key_of_a_file.sh
. ./tessa__a_file_is_identified_in_its_filesystem.sh
. ./tessa__equal_inodes_in_one_filesystem_reach_one_file.sh
. ./tessa__a_file_has_one_inode_in_its_filesystem.sh
. ./simon__a_device_number_is_the_primary_key_of_a_filesystem.sh
. ./simon__a_filesystem_is_identified_in_the_boot_that_mounted_it.sh
. ./simon__a_filesystem_has_one_device_number_in_its_boot.sh
. ./simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem.sh
. ./stdlib__a_boot_id_is_the_primary_key_of_a_boot.sh
. ./stdlib__a_boot_id_is_a_root.sh
. ./carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else.sh
. ./carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode.sh

#}= stat -c '%i %d %h' a-path b-path                               #=> a-path.resolvesTo = none, b-path.resolvesTo = none
stat -c '%i %d %h' /srv/a/shared /srv/b/shared

#} chmod g-w a-path                                                #=> run
chmod g-w /srv/a/shared
#} chmod g+w b-path                                                #=> guard
chmod g+w /srv/b/shared
```

Under a path scheme that declines any path whose inode has a second entry, neither path resolves, and the site guards with nothing attributed. The scheme said less than the world needed, and said so.
