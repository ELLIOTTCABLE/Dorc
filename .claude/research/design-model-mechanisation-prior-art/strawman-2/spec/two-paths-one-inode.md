# Two paths, one inode

Two file paths that name one inode must never be told apart, and two that name two inodes in one filesystem, or in two filesystems of one boot, may be. This document holds the speech that decides it, the rules that consume the speech, and the books that pin the answers.

Tessa writes the file describer.
Simon writes the filesystem describer.
Carl writes the chmod describer.
The stdlib describes the boot.
Foob writes a certificate tool nobody here has heard of.
The host under test is the world; it speaks nothing, and measures show it.

## § 1 The question

```alloy
one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Ans {}
fact { weaker = UNKNOWN->SAME + UNKNOWN->DISJOINT + UNKNOWN->KNOWN_UNSPOKEN + KNOWN_UNSPOKEN->DISJOINT }
fact { Safe = UNKNOWN + KNOWN_UNSPOKEN }
fact { Spares = DISJOINT }
```

A converged line is elided only when every pair of a key an earlier running line wrote and a key its own check reads answers in `Spares`. Whatever is not in `Spares` collides.

## § 2 Ways of writing, and the things they reach

```alloy
sig MReferent {}
sig MValue {}
abstract sig MSort {}
abstract sig MScheme { yields: lone MScheme, primaryOf: lone MSort }
fact { all s: MScheme | no s.yields or no s.primaryOf }
sig MKey { scheme: one MScheme, value: one MValue, reaches: one MReferent, worldParent: lone MKey }
fact { no k: MKey | k in k.^worldParent }

sig Yields extends MDecl { from, to: one MScheme }
fact { all d: Yields & True | d.from.yields = d.to }

sig PrimaryOf extends MDecl { ofScheme: one MScheme, ofSort: one MSort }
fact { all d: PrimaryOf & True | d.ofScheme.primaryOf = d.ofSort }

sig IdentifiedIn extends MDecl { ofScheme: one MScheme, within: one MSort }
fact { all d: IdentifiedIn & True, k: MKey | k.scheme = d.ofScheme and some k.worldParent implies k.worldParent.scheme.primaryOf = d.within }

sig GuaranteesUniqueReferent extends MDecl { on: one MScheme }
fact { all d: GuaranteesUniqueReferent & True, a, b: MKey | a.scheme = d.on and b.scheme = d.on and a.worldParent = b.worldParent and a.value = b.value implies a.reaches = b.reaches }

sig GuaranteesUniqueName extends MDecl { on: one MScheme }
fact { all d: GuaranteesUniqueName & True, a, b: MKey | a.scheme = d.on and b.scheme = d.on and a.worldParent = b.worldParent and a.reaches = b.reaches implies a = b }

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
fact { all d: AliasesNothingElse & True, k, k2: MKey | k.worldParent.scheme.primaryOf = d.store and k2.reaches = k.reaches implies k2.worldParent = k.worldParent }

one sig simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem extends AliasesNothingElse {} { speaker = simon  store = Filesystem }
```

Simon can say this because he describes the filesystem. Tessa cannot, because an inode number carries nothing about the filesystem's kind.

## § 4 What a verb writes and what its check reads

```alloy
sig Verb {}
sig Site { verb: one Verb, arg: one MKey, before: set Site }
sig Converged in Site {}

sig MayWrite extends MDecl { verb: one Verb }
sig ChecksRead extends MDecl { verb: one Verb }
fun writesOf[S: set MDecl, s: Site]: set MKey { (some d: MayWrite & S | d.verb = s.verb) implies s.arg else MKey }
fun readsOf[S: set MDecl, s: Site]: set MKey { (some d: ChecksRead & S | d.verb = s.verb) implies s.arg else MKey }

one sig carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else extends MayWrite {} { speaker = carl  verb = chmod }
one sig carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode extends ChecksRead {} { speaker = carl  verb = chmod }
```

A verb nobody has described writes every key and reads every key.

## § 5 What the engine knows, and what is true

```alloy
sig Resolution extends MDecl { of: one MKey, to: lone MKey }
sig Placement extends MDecl { of: one MKey, within: one MKey }
fun Static: set MDecl { MDecl - Resolution - Placement }

sig Loaded in MDecl {}
fact { Resolution + Placement in Loaded }
fact { True & Static = Loaded & Static }
fact { all x: Resolution | x in True iff (no x.to or x.of.reaches = x.to.reaches) }
fact { all p: Placement | p in True iff p.within = p.of.worldParent }

fun resolvedIn[S: set MDecl]: MKey -> MKey { { a, b: MKey | some x: Resolution & S | x.of = a and x.to = b } }
fun parentIn[S: set MDecl]: MKey -> MKey { { a, b: MKey | some p: Placement & S | p.of = a and p.within = b } }
fun ident[S: set MDecl, k: MKey]: lone MKey { k.*(resolvedIn[S]) & scheme.(primaryOf.MSort) }
fun chain[S: set MDecl, k: MKey]: set MKey { k.*(parentIn[S]) }
pred rooted[S: set MDecl, k: MKey] { all t: chain[S, k] | some t.(parentIn[S]) or some d: Root & S | d.on = t.scheme }
fun top[S: set MDecl, x, A: MKey]: one MKey { chain[S, x] & (parentIn[S]).A }

pred separated[S: set MDecl, x, y: MKey] {
   some A: x.^(parentIn[S]) & y.^(parentIn[S]) | no (x.^(parentIn[S]) & y.^(parentIn[S]) - A.*(parentIn[S])) and
   let tx = top[S, x, A], ty = top[S, y, A] |
      tx.scheme = ty.scheme and tx.value != ty.value
      and (some d: GuaranteesUniqueName & S | d.on = tx.scheme)
      and (all s: (x.^(parentIn[S]) + y.^(parentIn[S])) - A.*(parentIn[S]) | some d: AliasesNothingElse & S | d.store = s.scheme.primaryOf)
}

fun answerKeys[S: set MDecl, x, y: MKey]: one Ans {
   (no ident[S, x] or no ident[S, y]) implies UNKNOWN
   else (not rooted[S, ident[S, x]] or not rooted[S, ident[S, y]]) implies UNKNOWN
   else separated[S, ident[S, x], ident[S, y]] implies DISJOINT
   else UNKNOWN
}

sig Query { w, r: one Site }
fact { all q: Query | q.w not in Converged and q.r in Converged and q.w in q.r.before }
fun answer[S: set MDecl, q: Query]: one Ans {
   (all x: writesOf[S, q.w], y: readsOf[S, q.r] | answerKeys[S, x, y] = DISJOINT) implies DISJOINT else UNKNOWN
}
pred wrong[S: set MDecl, q: Query] { answer[S, q] = DISJOINT and some x: writesOf[S, q.w], y: readsOf[S, q.r] | x.reaches = y.reaches }
fun restsOn[S: set MDecl, q: Query]: set MDecl { { d: S | answer[S - d, q] != answer[S, q] } }
fun by[s: Site]: set MDecl { { d: Loaded | some q: r.s | d in restsOn[Loaded, q] } }

sig Ran, Elided, Guarded in Site {}
fact { Ran = Site - Converged }
fact { Elided = { s: Converged | all q: r.s | answer[Loaded, q] in Spares } }
fact { Guarded = Converged - Elided }
```

A resolution is what a lookup told the engine; `reaches` is what was so. A resolution with no target is a decline, and a decline is never false. A chain that ends at a key nobody has declared a root spares nothing.

## § 6 Who may say what

<!-- normative: -->
> A claim is made by the seat that can know it.
> Whether a filesystem exposes its inodes through another filesystem is the filesystem describer's knowledge.
> Whether a file has one inode is the file describer's knowledge.
> What a lookup returned is the lookup owner's statement, witnessed by the host.
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
   all d: Resolution + Placement | d.speaker in schemeOwner[d.of.scheme]
} for 12
```

## § 7 The oracle sets

```sh
# tessa_fs.sh
. ./tessa__a_path_names_one_entry_per_component.sh
. ./tessa__an_entry_names_one_inode.sh
. ./tessa__an_inode_is_the_primary_key_of_a_file.sh
. ./tessa__a_file_is_identified_in_its_filesystem.sh
. ./tessa__equal_inodes_in_one_filesystem_reach_one_file.sh
. ./tessa__a_file_has_one_inode_in_its_filesystem.sh
```

```sh
# simon_fs.sh
. ./simon__a_device_number_is_the_primary_key_of_a_filesystem.sh
. ./simon__a_filesystem_is_identified_in_the_boot_that_mounted_it.sh
. ./simon__a_filesystem_has_one_device_number_in_its_boot.sh
. ./simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem.sh
```

```sh
# stdlib_boot.sh
. ./stdlib__a_boot_id_is_the_primary_key_of_a_boot.sh
. ./stdlib__a_boot_id_is_a_root.sh
```

```sh
# carl_chmod.sh
. ./carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else.sh
. ./carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode.sh
```

## § 8 The books

```sh
# two_paths_one_inode.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d' /srv/a/shared /srv/b/shared
#}  stat -c '%i %d' a_path b_path
#=  a_path.reaches = inode_x.reaches and b_path.reaches = inode_x.reaches and inode_x.worldParent = fs_1

    cat /proc/sys/kernel/random/boot_id
#}  cat /proc/sys/kernel/random/boot_id
#=  fs_1.worldParent = boot_1

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

    chmod g+w /srv/b/shared
#}  chmod g+w b_path
#=  this in Guarded
```

```sh
# siblings_in_one_filesystem.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d' /srv/a/shared /srv/a/other
#}  stat -c '%i %d' a_path c_path
#=  a_path.reaches = inode_x.reaches and c_path.reaches = inode_y.reaches
#=  inode_x.worldParent = fs_1 and inode_y.worldParent = fs_1

    cat /proc/sys/kernel/random/boot_id
#}  cat /proc/sys/kernel/random/boot_id
#=  fs_1.worldParent = boot_1

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

    chmod g+w /srv/a/other
#}  chmod g+w c_path
#=  this in Elided
#=  simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem not in by[this]
```

```sh
# siblings_across_filesystems.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d' /srv/a/shared /var/lib/other
#}  stat -c '%i %d' a_path d_path
#=  a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches
#=  inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2

    cat /proc/sys/kernel/random/boot_id
#}  cat /proc/sys/kernel/random/boot_id
#=  fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

    chmod g+w /var/lib/other
#}  chmod g+w d_path
#=  this in Elided
#=  simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem in by[this]
```

## § 9 The same lines under less speech, a stranger's speech, and other speech

```sh
# siblings_across_filesystems_without_the_closure.sh
. ./tessa_fs.sh
. ./simon__a_device_number_is_the_primary_key_of_a_filesystem.sh
. ./simon__a_filesystem_is_identified_in_the_boot_that_mounted_it.sh
. ./simon__a_filesystem_has_one_device_number_in_its_boot.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d' /srv/a/shared /var/lib/other
#}  stat -c '%i %d' a_path d_path
#=  a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches
#=  inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2

    cat /proc/sys/kernel/random/boot_id
#}  cat /proc/sys/kernel/random/boot_id
#=  fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

    chmod g+w /var/lib/other
#}  chmod g+w d_path
#=  this in Guarded
```

Without Simon's closure, two files in two filesystems collide. Nothing was false; nobody said the filesystems were compartments.

```sh
# siblings_in_one_filesystem_without_the_root.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib__a_boot_id_is_the_primary_key_of_a_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d' /srv/a/shared /srv/a/other
#}  stat -c '%i %d' a_path c_path
#=  a_path.reaches = inode_x.reaches and c_path.reaches = inode_y.reaches
#=  inode_x.worldParent = fs_1 and inode_y.worldParent = fs_1

    cat /proc/sys/kernel/random/boot_id
#}  cat /proc/sys/kernel/random/boot_id
#=  fs_1.worldParent = boot_1

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

    chmod g+w /srv/a/other
#}  chmod g+w c_path
#=  this in Guarded
```

Without the boot declared a root, every chain ends at a key nobody vouched for, and two inodes in one filesystem collide.

```alloy
one sig foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle extends PrimaryOf {} { speaker = foob  ofScheme = BundlePath  ofSort = CertificateBundle }
```

```sh
# foob_certs.sh
. ./foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle.sh
```

```sh
# siblings_across_filesystems_with_a_stranger.sh
. ./foob_certs.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d' /srv/a/shared /var/lib/other
#}  stat -c '%i %d' a_path d_path
#=  a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches
#=  inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2

    cat /proc/sys/kernel/random/boot_id
#}  cat /proc/sys/kernel/random/boot_id
#=  fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

    chmod g+w /var/lib/other
#}  chmod g+w d_path
#=  this in Elided
#=  no (speaker.foob & by[this])
```

Foob's speech about a sort nobody else uses changes nothing, and the survival names none of it.

```alloy
one sig tessa__a_singly_linked_path_names_one_entry_per_component_and_declines_a_path_whose_inode_has_another_entry extends Yields {} { speaker = tessa  from = SinglyLinkedPath  to = Entry }
one sig tessa__a_singly_linked_path_names_one_file extends GuaranteesUniqueName {} { speaker = tessa  on = SinglyLinkedPath }
```

```sh
# tessa_singly_linked.sh
. ./tessa__a_singly_linked_path_names_one_entry_per_component_and_declines_a_path_whose_inode_has_another_entry.sh
. ./tessa__a_singly_linked_path_names_one_file.sh
. ./tessa__an_entry_names_one_inode.sh
. ./tessa__an_inode_is_the_primary_key_of_a_file.sh
. ./tessa__a_file_is_identified_in_its_filesystem.sh
. ./tessa__equal_inodes_in_one_filesystem_reach_one_file.sh
. ./tessa__a_file_has_one_inode_in_its_filesystem.sh
```

```sh
# two_paths_one_inode_under_a_singly_linked_path_scheme.sh
. ./tessa_singly_linked.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

    stat -c '%i %d %h' /srv/a/shared /srv/b/shared
#}  stat -c '%i %d %h' a_path b_path
#=  a_path.reaches = inode_x.reaches and b_path.reaches = inode_x.reaches and inode_x.worldParent = fs_1
#=  one sig tessa__a_path_declines extends Resolution {} { of = a_path  no to }
#=  one sig tessa__b_path_declines extends Resolution {} { of = b_path  no to }

    chmod g-w /srv/a/shared
#}  chmod g-w a_path
#=  this in Ran

    chmod g+w /srv/b/shared
#}  chmod g+w b_path
#=  this in Guarded
```

Under a path scheme that declines any path whose inode has a second entry, the host still names one inode from two paths, the scheme returns nothing for either, and the site guards with nothing attributed. The scheme said less than the world needed, and said so.
