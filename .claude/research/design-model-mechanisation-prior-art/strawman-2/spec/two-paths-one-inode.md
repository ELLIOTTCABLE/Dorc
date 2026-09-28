# Two paths, one inode

Two file paths that name one inode must never be told apart, and two that name two inodes in one filesystem, or in two filesystems of one boot, may be. This document holds the speech that decides it, the rules that consume the speech, and the books that pin the answers.

Tessa writes the file describer.
Simon writes the filesystem describer.
Carl writes the chmod describer.
The stdlib describes the boot.
Foob writes a certificate tool nobody here has heard of.
The host under test is the world; it speaks nothing, and measures show it.

```alloy
one sig tessa, simon, carl, stdlib, foob extends Speaker {}
```

## § 1 Ways of writing, and the things they reach

```alloy
sig MReferent {}
abstract sig MSort {}
abstract sig MScheme { primaryOf: lone MSort }
sig Key { w: one Shword, scheme: one MScheme, reaches: one MReferent, worldParent: lone Key }
fact { no k: Key | k in k.^worldParent }
fact { all disj a, b: Key | not (a.w = b.w and a.scheme = b.scheme and a.worldParent = b.worldParent) }

sig Operand extends MDecl { verb: one Shword, at: one Int, under: one MScheme }
fact { all d: Operand & True, l: Line | d.verb = l.cmd implies some k: Key | k.w = l.argv[d.at] and k.scheme = d.under }

sig Yields extends MDecl { of: one Class, under: one MScheme, to: lone MScheme }

sig PrimaryOf extends MDecl { ofScheme: one MScheme, ofSort: one MSort }
fact { all d: PrimaryOf & True | d.ofScheme.primaryOf = d.ofSort }
fact { all s: MScheme | some s.primaryOf implies no { y: Yields & True | y.under = s and some y.to } }

sig IdentifiedIn extends MDecl { ofScheme: one MScheme, within: one MSort }
fact { all d: IdentifiedIn & True, k: Key | k.scheme = d.ofScheme and some k.worldParent implies k.worldParent.scheme.primaryOf = d.within }

sig GuaranteesUniqueName extends MDecl { on: one MScheme }
fact { all d: GuaranteesUniqueName & True, a, b: Key | a.scheme = d.on and b.scheme = d.on and a.worldParent = b.worldParent and a.reaches = b.reaches implies a = b }

sig GuaranteesUniqueReferent extends MDecl { on: one MScheme }
fact { all d: GuaranteesUniqueReferent & True, a, b: Key | a.scheme = d.on and b.scheme = d.on and a.worldParent = b.worldParent and a.w = b.w implies a.reaches = b.reaches }

sig Root extends MDecl { on: one MScheme }
fact { all d: Root & True, a, b: Key | a.scheme = d.on and b.scheme = d.on and a.w = b.w implies a.reaches = b.reaches }

one sig Path, Inode, DeviceNumber, BootId, BundlePath extends MScheme {}
one sig File, Filesystem, Boot, CertificateBundle extends MSort {}

one sig tessa__a_slash_separated_path_names_the_inode_of_its_last_entry extends Yields {} { speaker = tessa  of = slash_path  under = Path  to = Inode }
one sig tessa__a_bare_word_is_not_a_path extends Yields {} { speaker = tessa  of = bare_word  under = Path  no to }
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

A key is a word read under a scheme inside a parent; one word may be several keys. A line's word is a key only where a claim in force reads it as one. Tessa's lookup answers by the flavour of string it is handed: a slash-separated path names an inode, a bare word names nothing. Tessa says nothing about whether two paths may name one inode; the name warrant on `Path` is absent, and its absence is her statement.

## § 2 A store that exposes nothing else

```alloy
sig AliasesNothingElse extends MDecl { store: one MSort }
fact { all d: AliasesNothingElse & True, k, k2: Key | k.worldParent.scheme.primaryOf = d.store and k2.reaches = k.reaches implies k2.worldParent = k.worldParent }

one sig simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem extends AliasesNothingElse {} { speaker = simon  store = Filesystem }
```

Simon can say this because he describes the filesystem. Tessa cannot, because an inode number carries nothing about the filesystem's kind.

## § 3 What a verb writes, what its check reads, and whose check it is

```alloy
sig MayWrite extends MDecl { verb: one Shword }
sig ChecksRead extends MDecl { verb: one Shword }
fun keysOf[S: set MDecl, l: Line]: set Key { { k: Key | some d: Operand & S | d.verb = l.cmd and k.w = l.argv[d.at] and k.scheme = d.under } }
fun writesOf[S: set MDecl, l: Line]: set Key { (some d: MayWrite & S | d.verb = l.cmd) and some keysOf[S, l] implies keysOf[S, l] else Key }
fun readsOf[S: set MDecl, l: Line]: set Key { (some d: ChecksRead & S | d.verb = l.cmd) and some keysOf[S, l] implies keysOf[S, l] else Key }
fact { all v: Verdict | v.speaker in (ChecksRead & True & verb.(v.of.cmd)).speaker }

one sig carl__chmod_reads_its_operand_as_a_path extends Operand {} { speaker = carl  verb = chmod  at = 1  under = Path }
one sig carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else extends MayWrite {} { speaker = carl  verb = chmod }
one sig carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode extends ChecksRead {} { speaker = carl  verb = chmod }
```

A verb nobody has described writes every key and reads every key. A described verb handed a word no claim reads as a key writes and reads every key too; the empty write set spares nothing. A verdict about a line is the word of whoever described that verb's check.

## § 4 What the engine knows, and what is true

```alloy
sig Resolution extends MDecl { of: one Key, to: lone Key }
sig Placement extends MDecl { of: one Key, within: one Key }
fact { Computed = Resolution + Placement }
fact { all x: Resolution | x in True iff (no x.to or x.of.reaches = x.to.reaches) }
fact { all p: Placement | p in True iff p.within = p.of.worldParent }

fun resolvedIn[S: set MDecl]: Key -> Key {
   { a, b: Key | some x: Resolution & S | x.of = a and x.to = b }
   + { a: Key - (Resolution & S).of, b: Key | some y: Yields & S | y.of in a.w.class and y.under = a.scheme and b.scheme = y.to and b.reaches = a.reaches }
}
fun parentIn[S: set MDecl]: Key -> Key {
   { a, b: Key | some p: Placement & S | p.of = a and p.within = b }
   + { a: Key - (Placement & S).of, b: Key | b = a.worldParent and some d: IdentifiedIn & S | d.ofScheme = a.scheme }
}
fun ident[S: set MDecl, k: Key]: lone Key { k.*(resolvedIn[S]) & scheme.(primaryOf.MSort) }
fun chain[S: set MDecl, k: Key]: set Key { k.*(parentIn[S]) }
pred rooted[S: set MDecl, k: Key] { all t: chain[S, k] | some t.(parentIn[S]) or some d: Root & S | d.on = t.scheme }
fun top[S: set MDecl, x, A: Key]: one Key { chain[S, x] & (parentIn[S]).A }

pred separated[S: set MDecl, x, y: Key] {
   some A: x.^(parentIn[S]) & y.^(parentIn[S]) | no (x.^(parentIn[S]) & y.^(parentIn[S]) - A.*(parentIn[S])) and
   let tx = top[S, x, A], ty = top[S, y, A] |
      tx.scheme = ty.scheme and tx.w != ty.w
      and (some d: GuaranteesUniqueName & S | d.on = tx.scheme)
      and (all s: (x.^(parentIn[S]) + y.^(parentIn[S])) - A.*(parentIn[S]) | some d: AliasesNothingElse & S | d.store = s.scheme.primaryOf)
}

fun answerKeys[S: set MDecl, x, y: Key]: one Ans {
   (no ident[S, x] or no ident[S, y]) implies UNKNOWN
   else (not rooted[S, ident[S, x]] or not rooted[S, ident[S, y]]) implies UNKNOWN
   else separated[S, ident[S, x], ident[S, y]] implies DISJOINT
   else UNKNOWN
}

fun answer[S: set MDecl, q: Query]: one Ans {
   (all x: writesOf[S, q.writer], y: readsOf[S, q.reader] | answerKeys[S, x, y] = DISJOINT) implies DISJOINT else UNKNOWN
}
fact { all q: Query | q.ans = answer[q.reader.speech, q] }
pred wrong[S: set MDecl, q: Query] { answer[S, q] = DISJOINT and some x: writesOf[S, q.writer], y: readsOf[S, q.reader] | x.reaches = y.reaches }
fun restsOn[S: set MDecl, q: Query]: set MDecl { { d: S | answer[S - d, q] != answer[S, q] } }
fun by[l: Line]: set MDecl { { d: l.speech | some q: reader.l | d in restsOn[l.speech, q] } }
```

A resolution is what a lookup told the engine; `reaches` is what was so. Where nobody wrote one, the resolution is derived from the yield claim that matches the key's flavour and scheme, under the hypothesis that the lookup was correct; a written one stands in its place, and a written one with no target is a decline, which is never false. A placement is derived the same way from the scheme's identified-in claim. A chain that ends at a key nobody has declared a root spares nothing. What a line's verdict rests on names the claims in force at that line, so a derived resolution is attributed to the yield claim it came from. A book's fixture says which scheme each token the host printed belongs to, as it says which filesystem each inode is in: the adversary is otherwise free to read the printed inode number as a boot id.

## § 5 The laws

```alloy
pred allTrue[S: set MDecl] { S in True }

check neverWrongWhenAllTrue { all S: set MDecl, q: Query | allTrue[S] implies not wrong[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
run neverWrongWhenAllTrue_premise { some S: set MDecl, q: Query | allTrue[S] and some writesOf[S, q.writer] } for 6 but 8 Shword, 3 Line, 1 Query

check monotoneInSpeech { all S, S2: set MDecl, q: Query | S in S2 and allTrue[S2] implies answer[S, q] in answer[S2, q].*weaker } for 6 but 8 Shword, 3 Line, 1 Query
run monotoneInSpeech_premise { some S, S2: set MDecl, q: Query | S in S2 and S != S2 and allTrue[S2] } for 6 but 8 Shword, 3 Line, 1 Query

check strangerSafe { all S: set MDecl, d: MDecl, q: Query | allTrue[S + d] and d.speaker not in S.speaker implies answer[S, q] in answer[S + d, q].*weaker } for 6 but 8 Shword, 3 Line, 1 Query
run strangerSafe_premise { some S: set MDecl, d: MDecl, q: Query | allTrue[S + d] and d.speaker not in S.speaker } for 6 but 8 Shword, 3 Line, 1 Query

check attributionHonest { all S: set MDecl, q: Query | wrong[S, q] implies some d: restsOn[S, q] | d not in True } for 6 but 8 Shword, 3 Line, 1 Query
run attributionHonest_premise { some S: set MDecl, q: Query | wrong[S, q] } for 6 but 8 Shword, 3 Line, 1 Query

check attributionSufficient { all S: set MDecl, q: Query | answer[restsOn[S, q], q] = answer[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
run attributionSufficient_premise { some S: set MDecl, q: Query | some restsOn[S, q] } for 6 but 8 Shword, 3 Line, 1 Query

check attributionMinimal { all S: set MDecl, q: Query, d: restsOn[S, q] | answer[S - d, q] != answer[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
run attributionMinimal_premise { some S: set MDecl, q: Query | some restsOn[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
```

## § 6 Who may say what

> A claim is made by the seat that can know it.
> Whether a filesystem exposes its inodes through another filesystem is the filesystem describer's knowledge.
> Whether a file has one inode is the file describer's knowledge.
> What a lookup returned is the lookup owner's statement, witnessed by the host.
> Whether a line is converged is the statement of whoever described that verb's check.
> The engine chains and meets; it originates no claim.
> Every survival names the claims it rested on and no others.

```alloy
fun sortOwner[s: MSort]: set Speaker { ((PrimaryOf & True) & ofSort.s).speaker }
fun schemeOwner[s: MScheme]: set Speaker { ((Yields & True) & under.s).speaker + ((PrimaryOf & True) & ofScheme.s).speaker }

check seatCanKnow {
   all d: AliasesNothingElse | d.speaker in sortOwner[d.store]
   all d: IdentifiedIn | d.speaker in schemeOwner[d.ofScheme]
   all d: GuaranteesUniqueName + GuaranteesUniqueReferent + Root | d.speaker in schemeOwner[d.on]
   all d: Resolution + Placement | d.speaker in schemeOwner[d.of.scheme]
}
```

## § 7 The load files

```sh
# tessa_fs.sh
. ./tessa__a_slash_separated_path_names_the_inode_of_its_last_entry.sh
. ./tessa__a_bare_word_is_not_a_path.sh
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
. ./carl__chmod_reads_its_operand_as_a_path.sh
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
#} stat -c '%i %d' a_path b_path
#= w.inode_x.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.b_path.reaches = w.inode_x.reaches and w.inode_x.worldParent = w.fs_1

   cat /proc/sys/kernel/random/boot_id
#} cat /proc/sys/kernel/random/boot_id
#= w.fs_1.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /srv/b/shared
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_b_path_has_the_mode extends Verdict {} { of = this }
#= this in Guarded
```

```sh
# siblings_in_one_filesystem.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

   stat -c '%i %d' /srv/a/shared /srv/a/other
#} stat -c '%i %d' a_path c_path
#= w.inode_x.scheme = Inode and w.inode_y.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.c_path.reaches = w.inode_y.reaches
#= w.inode_x.worldParent = w.fs_1 and w.inode_y.worldParent = w.fs_1

   cat /proc/sys/kernel/random/boot_id
#} cat /proc/sys/kernel/random/boot_id
#= w.fs_1.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /srv/a/other
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_c_path_has_the_mode extends Verdict {} { of = this }
#= this in Elided
#= simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem not in by[this]
```

```sh
# siblings_across_filesystems.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

   stat -c '%i %d' /srv/a/shared /var/lib/other
#} stat -c '%i %d' a_path d_path
#= w.inode_x.scheme = Inode and w.inode_z.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.d_path.reaches = w.inode_z.reaches
#= w.inode_x.worldParent = w.fs_1 and w.inode_z.worldParent = w.fs_2

   cat /proc/sys/kernel/random/boot_id
#} cat /proc/sys/kernel/random/boot_id
#= w.fs_1.worldParent = w.boot_1 and w.fs_2.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /var/lib/other
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_d_path_has_the_mode extends Verdict {} { of = this }
#= this in Elided
#= simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem in by[this]
```

```sh
# chmod_of_a_bare_word.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

   chmod g-w shared
#} chmod g-w {bare_word}
#= this in Ran

   chmod g+w shared
#} chmod g+w {bare_word}
#= one sig carl__the_file_at_shared_has_the_mode extends Verdict {} { of = this }
#= this in Guarded
#= no by[this]
```

Carl reads the operand under `Path` whatever it looks like; Tessa's lookup declines a bare word; the key has no identity and the site guards with nothing attributed.

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
#} stat -c '%i %d' a_path d_path
#= w.inode_x.scheme = Inode and w.inode_z.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.d_path.reaches = w.inode_z.reaches
#= w.inode_x.worldParent = w.fs_1 and w.inode_z.worldParent = w.fs_2

   cat /proc/sys/kernel/random/boot_id
#} cat /proc/sys/kernel/random/boot_id
#= w.fs_1.worldParent = w.boot_1 and w.fs_2.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /var/lib/other
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_d_path_has_the_mode extends Verdict {} { of = this }
#= this in Guarded
```

Without Simon's closure, two files in two filesystems collide. Nothing was false; nobody said the filesystems were compartments.

```sh
# siblings_in_one_filesystem_without_the_root.sh
. ./tessa_fs.sh
. ./simon_fs.sh
. ./stdlib__a_boot_id_is_the_primary_key_of_a_boot.sh
. ./carl_chmod.sh

   stat -c '%i %d' /srv/a/shared /srv/a/other
#} stat -c '%i %d' a_path c_path
#= w.inode_x.scheme = Inode and w.inode_y.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.c_path.reaches = w.inode_y.reaches
#= w.inode_x.worldParent = w.fs_1 and w.inode_y.worldParent = w.fs_1

   cat /proc/sys/kernel/random/boot_id
#} cat /proc/sys/kernel/random/boot_id
#= w.fs_1.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /srv/a/other
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_c_path_has_the_mode extends Verdict {} { of = this }
#= this in Guarded
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
#} stat -c '%i %d' a_path d_path
#= w.inode_x.scheme = Inode and w.inode_z.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.d_path.reaches = w.inode_z.reaches
#= w.inode_x.worldParent = w.fs_1 and w.inode_z.worldParent = w.fs_2

   cat /proc/sys/kernel/random/boot_id
#} cat /proc/sys/kernel/random/boot_id
#= w.fs_1.worldParent = w.boot_1 and w.fs_2.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /var/lib/other
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_d_path_has_the_mode extends Verdict {} { of = this }
#= this in Elided
#= no (speaker.foob & by[this])
```

Foob's speech about a sort nobody else uses changes nothing, and the survival names none of it.

```alloy
one sig tessa__a_singly_linked_path_names_one_file extends GuaranteesUniqueName {} { speaker = tessa  on = Path }
```

```sh
# tessa_singly_linked.sh
. ./tessa_fs.sh
. ./tessa__a_singly_linked_path_names_one_file.sh
```

```sh
# two_paths_one_inode_under_a_singly_linked_path_scheme.sh
. ./tessa_singly_linked.sh
. ./simon_fs.sh
. ./stdlib_boot.sh
. ./carl_chmod.sh

   stat -c '%i %d %h' /srv/a/shared /srv/b/shared
#} stat -c '%i %d %h' a_path b_path
#= w.inode_x.scheme = Inode
#= w.a_path.reaches = w.inode_x.reaches and w.b_path.reaches = w.inode_x.reaches and w.inode_x.worldParent = w.fs_1
#= one sig tessa__a_path_declines extends Resolution {} { speaker = tessa  of = w.a_path  no to }
#= one sig tessa__b_path_declines extends Resolution {} { speaker = tessa  of = w.b_path  no to }

   cat /proc/sys/kernel/random/boot_id
#} cat /proc/sys/kernel/random/boot_id
#= w.fs_1.worldParent = w.boot_1

   chmod g-w /srv/a/shared
#} chmod g-w {slash_path}
#= this in Ran

   chmod g+w /srv/b/shared
#} chmod g+w {slash_path}
#= one sig carl__the_file_at_b_path_has_the_mode extends Verdict {} { of = this }
#= this in Guarded
```

Under a describer that warrants one file per path, a path whose inode has a second entry is one the lookup declines rather than answers. The host still names one inode from two paths, the written declines stand in for the derived resolutions, and the site guards with nothing attributed. The scheme said less than the world needed, and said so.
