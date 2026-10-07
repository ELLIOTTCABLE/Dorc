# 311w — Identity and relation: the tests

The tests of the identity specification, `311-identity.assay.md`: its books, the kills of its
laws, and the witnesses of its held holes. AI-authored; moved out of the specification by an Opus
builder at the human's direction (`notes/313` § 2.2). Specification-tier; the specification
outranks this document, and the root docs, `spike/AGENTS.md`, and the welds outrank both.

This document opens the specification and so sees every definition there; the specification sees
nothing here. The checker still loads the whole specification for every command below, so the
split serves the reader and saves no solving. Nothing here is a fact about the specification's
relations (`313:read-the-test-document-states-no-fact-about-the-specification`): a book's world
facts are scoped to that book's own generated module (`notes/30Y` § 2.3), and everything else in
this document is a command. The specification's conventions hold here. Each numbered section
carries the number of the specification section it tests, and a citation of a specification
section is written `311:<slug>`. The results of every command are in
`311w-identity.tests.lock.json` beside this file.

## The specification under test

```alloy
open w_311_dash_identity
```

<!-- prose-translation -->
> Every command of this document uses every signature, fact, function, and predicate of `311-identity.assay.md`.

## § 2-tests-of-the-model-relations

### § 2.1-tests-of-yields-into-another-scheme

```alloy
run kill_natural_same_is_sound_unique_referent {
   axiomaticByDifferentialTest
   some d: DeclaresUniqueReferent & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | naturalKeyAnswer[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 6 but 4 Int expect 1

run kill_natural_disjoint_is_sound_unique_name {
   axiomaticByDifferentialTest
   some d: DeclaresUniqueName & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | naturalKeyAnswer[x, y] = DISJOINT and some x.mRefersTo & y.mRefersTo
} for 6 but 4 Int expect 1
```

<!-- prose-translation -->
> Each of the two laws dies with the warrant it rests on.
> Each kill in this section asks with the engine's axioms holding.
> With one `:guarantees-unique-referent` false and every other statement in force true, a false SAME licensed before the primary MScheme is reachable.
> With one `:guarantees-unique-name` false and every other statement in force true, a false DISJOINT licensed before the primary MScheme is reachable.

### § 2.6-tests-of-may-write-the-writeset

```alloy
run hole_two_separated_things_reach_one_thing_beneath_witness {
   hole_two_separated_things_reach_one_thing_beneath and axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
} for 6 but 4 Int expect 1

run hole_a_write_affects_through_a_third_thing_witness {
   hole_a_write_affects_through_a_third_thing and axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
} for 6 but 4 Int expect 1

run kill_sparing_is_sound_closes_may_write {
   noStoreIsAmongItsOwnContents and outsideTheSparingHoles and axiomaticByDifferentialTest
   some d: ClosesMayWrite & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some l: Line, f: VerdictFact & InForce |
         spared[l, f] and some (World.lineWrites[l]).*affects & f.dependsOn
} for 4 but 4 Int, 10 Claim expect 1

run kill_sparing_is_sound_verdict_fact {
   noStoreIsAmongItsOwnContents and outsideTheSparingHoles and axiomaticByDifferentialTest
   some d: VerdictFact & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some l: Line | spared[l, d] and some (World.lineWrites[l]).*affects & d.dependsOn
} for 4 but 4 Int, 10 Claim expect 1

run kill_sparing_is_sound_closes_may_read {
   noStoreIsAmongItsOwnContents and outsideTheSparingHoles and axiomaticByDifferentialTest
   some d: ClosesMayRead & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some l: Line, f: VerdictFact & InForce |
         spared[l, f] and some (World.lineWrites[l]).*affects & f.dependsOn
} for 4 but 4 Int, 10 Claim expect 1

run kill_sparing_is_sound_finishes_entailment {
   noStoreIsAmongItsOwnContents and outsideTheSparingHoles and axiomaticByDifferentialTest
   some d: FinishesEntailment & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some l: Line, f: VerdictFact & InForce |
         spared[l, f] and some (World.lineWrites[l]).*affects & f.dependsOn
} for 4 but 4 Int, 10 Claim expect 1

run kill_sparing_is_sound_supplies_parent {
   noStoreIsAmongItsOwnContents and outsideTheSparingHoles and axiomaticByDifferentialTest
   some d: SuppliesParent & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some l: Line, f: VerdictFact & InForce |
         spared[l, f] and some (World.lineWrites[l]).*affects & f.dependsOn
} for 4 but 4 Int, 10 Claim, 5 MLevel expect 1
```

<!-- prose-translation -->
> The sparing law dies with a statement it rests on.
> Each kill of the sparing law asks outside the sparing holes, with the engine's axioms holding and no store among its own contents.
> With one completion record false and every other statement in force true, a false sparing is reachable.
> With one vouch false and every other statement in force true, a false sparing is reachable.
> Three more kills ask whether the law also dies with one closed may-read set false, one finished record false, or one supplied MParent instance false.
> An unsatisfiable kill says that no sparing rests on that statement alone.
> The kill by a supplied MParent instance asks over worlds of five levels.
> Its witness needs a store on the read MKey's chain beside the two MWorlds.

#### § 2.6.2-a-book-stage-five-the-index-given-whole

USER_STORY stage 5's stale-index morning, with the index given whole: Anna's apt describer
names the package index, an MKey of her own MSort identified in the boot, finishes its
entailment, and closes the line's at-most set; Tessa and the stdlib close every may-read set on
the status file's chain; the flag is set. So every reason for a collision but one is removed,
and the answer isolates that one: the index and the status file meet at the boot as MKeys of
two MSorts with no shared key space, the acked cost of `311t` § 14. The book is self-contained
and pinned as the books of 3.2.2-a-book-two-files-in-one-filesystem are; the book in which
Anna names a list file instead is 2.6.3-a-book-stage-five-the-list-file-named.

```alloy
run bookScope_stage_five_the_index_given_whole {} for 8 but 4 Int
```

```sh
# stage_five_the_index_given_whole.sh
   apt-get update
#} apt_get update
#= one sig stdlib, tessa, anna, deb extends Speaker {}
#= one sig sm_Boot extends MSort {} { sortOwner = stdlib }
#= one sig sm_Filesystem, sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_PkgIndex extends MSort {} { sortOwner = anna }
#= one sig sm_BootId extends MScheme {} { schemeOwner = stdlib }
#= one sig sm_FsId, sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig sm_AptIndex extends MScheme {} { schemeOwner = anna }
#= one sig boot_shape extends MShape {} { ofScheme = sm_BootId }
#= one sig fsid_shape extends MShape {} { ofScheme = sm_FsId }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig index_shape extends MShape {} { ofScheme = sm_AptIndex }
#= one sig the_boot, fs_1, status_inode, the_index extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig w_boot extends MRootWorld {} { rootShape = boot_shape }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and ambient = sm_Filesystem->k_fs_1 }
#= one sig k_boot extends MKey {} { mValue = boot_2f3a and scheme = sm_BootId and no cellSort and shape = boot_shape and no yielded and at = v0 and mRefersTo = the_boot }
#= one sig k_fs_1 extends MKey {} { mValue = dev_8_1 and scheme = sm_FsId and no cellSort and shape = fsid_shape and no yielded and at = v0 and mRefersTo = fs_1 }
#= one sig k_index extends MKey {} { mValue = apt_lists and scheme = sm_AptIndex and no cellSort and shape = index_shape and no yielded and at = v0 and mRefersTo = the_index }
#= one sig k_status_inode extends MKey {} { mValue = ino_9 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = status_inode }
#= one sig k_status_path extends MKey {} { mValue = var_lib_dpkg_status and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_status_inode and at = v0 and mRefersTo = status_inode }
#= Speaker = stdlib + tessa + anna + deb
#= MSort = sm_Boot + sm_Filesystem + sm_File + sm_PkgIndex
#= MScheme = sm_BootId + sm_FsId + sm_Inode + sm_Path + sm_AptIndex
#= MShape = boot_shape + fsid_shape + inode_shape + slash_path_shape + index_shape
#= MReferent = the_boot + fs_1 + status_inode + the_index
#= MKey = k_boot + k_fs_1 + k_index + k_status_inode + k_status_path
#= MVantage = v0 and MRoute = r0 and MRootWorld = w_boot
#= no Wrapper and no CompositeKey and no Role and some RiskFaultlessSkips
#= no Engine.lookupReadSetOpen
#= World.lineWrites = (anna__update_writes_the_index).writeLine->the_index
#= GivenWhole = anna__update_writes_the_index
#= holds = the_boot->fs_1 + the_boot->the_index + fs_1->status_inode
#= owns = holds
#= passes = fs_1->status_inode
#= no affects
#= one sig stdlib__the_boot_id_is_primary_of_the_boot extends DeclaresPrimaryOf {} { speaker = stdlib and primaryScheme = sm_BootId and ofSort = sm_Boot }
#= one sig stdlib__a_boot_id_is_a_root extends DeclaresRoot {} { speaker = stdlib and rootedShape = boot_shape }
#= one sig stdlib__the_boots_may_read_set_is_closed extends ClosesMayRead {} { speaker = stdlib and readSort = sm_Boot }
#= one sig tessa__the_filesystem_id_is_primary_of_the_filesystem extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_FsId and ofSort = sm_Filesystem }
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_filesystem_is_identified_in_its_boot extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = fsid_shape and inSort = sm_Boot }
#= one sig tessa__an_inode_is_identified_in_its_filesystem extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = inode_shape and inSort = sm_Filesystem }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__a_path_is_looked_up_in_a_filesystem extends DeclaresCatalogSort {} { speaker = tessa and forScheme = sm_Path and catalogSort = sm_Filesystem }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig tessa__an_inode_has_one_number extends DeclaresUniqueName {} { speaker = tessa and nameShape = inode_shape }
#= one sig tessa__fs_1_is_in_the_boot extends SuppliesParent {} { speaker = tessa and forKey = k_fs_1 and instance = k_boot and seat = DeclarationSeat }
#= one sig tessa__the_status_inode_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_status_inode and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig tessa__the_files_may_read_set_is_closed extends ClosesMayRead {} { speaker = tessa and readSort = sm_File }
#= one sig tessa__the_filesystems_may_read_set_is_closed extends ClosesMayRead {} { speaker = tessa and readSort = sm_Filesystem }
#= one sig anna__the_apt_index_is_primary_of_the_package_index extends DeclaresPrimaryOf {} { speaker = anna and primaryScheme = sm_AptIndex and ofSort = sm_PkgIndex }
#= one sig anna__the_index_is_identified_in_its_boot extends DeclaresIdentifiedIn {} { speaker = anna and onShape = index_shape and inSort = sm_Boot }
#= one sig anna__the_index_is_in_the_boot extends SuppliesParent {} { speaker = anna and forKey = k_index and instance = k_boot and seat = DeclarationSeat }
#= one sig anna__the_package_index_entails_nothing_else extends FinishesEntailment {} { speaker = anna and finishedSort = sm_PkgIndex and finishedShape = index_shape }
#= one sig anna__update_writes_the_index extends DeclaresMayWrite {} { speaker = anna and writeLine = this and writeEntry = k_index }
#= one sig anna__update_writes_nothing_else extends ClosesMayWrite {} { speaker = anna and closedLine = this }
#= axiomaticByContract and axiomaticByDifferentialTest and atMostClosed[this] and wholeWriteEntries[this] = k_index

   dpkg -s nginx
#} dpkg dash_s nginx
#= one sig deb__nginx_is_installed extends VerdictFact {} { speaker = deb and topic = k_status_path and atLine = this and markedReads = k_status_path and dependsOn = status_inode }
#= let f = atLine.this, l = (anna__update_writes_the_index).writeLine | tabledCompare[k_index, f.topic] = KNOWN_UNSPOKEN and not readsetIsTop[f] and not writesetIsTop[l, writesetsAgainst[l][f.topic]] and not spared[l, f]
```

<!-- prose-translation -->
> This book's ceiling is eight atoms of every kind the specification owns and integers of four bits.
> The stdlib roots the boot as in 3.2.2-a-book-two-files-in-one-filesystem and closes the boot's may-read set.
> Tessa speaks as in that book and closes the file's and the filesystem's may-read sets.
> Anna owns the package index.
> The apt index's MScheme is `:primary-of` the package index, and its shape is `:identified-in` the boot.
> Anna's declaration supplies the boot as the index's MParent.
> Anna declares the finished record for the index's shape.
> Anna's `apt-get update` may-writes the index, given whole, and writes nothing else.
> Deb's `dpkg -s nginx` measures the package as a read of the status file's path, which yields the status file's inode in the filesystem.
> Deb's fact marks the status file's path as its read and depends on the status file's inode.
> The book holds no world object besides those its lines name.
> The world holds one boot, one filesystem and the index in it, and the status file's inode in the filesystem.
> Each of these MReferents is owned by what holds it.
> A route through the filesystem passes to the inode.
> No write affects another MReferent.
> The first line writes the index.
> Every MKey is resolved from one MVantage on one MRoute, with the filesystem as its ambient instance, under no wrapper and with `--risk-faultless-skips` set.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> Line 1, `apt-get update`: every statement in force is true and the engine's axioms hold.
> At line 1, the line's at-most set is closed and its one entry, the index, is given whole.
> Line 2, `dpkg -s nginx`, against line 1: `compare()` answers KNOWN_UNSPOKEN for the index against the status file's path.
> At line 2, the fact's readset is not ⊤, and line 1's writeset against it is not ⊤.
> At line 2, the fact is not spared past line 1.

#### § 2.6.3-a-book-stage-five-the-list-file-named

The same morning, with Anna's describer naming what `apt-get update` writes in Tessa's words:
one list file, by its path. The list file and the status file are two inodes of one
filesystem. The world, the stdlib, Tessa, and Deb are those of
2.6.2-a-book-stage-five-the-index-given-whole; Anna mints no MSort of her own. The answer below
is the fences' answer in this world, and it rests on a suspected hole that is held for the
design sitting: whether a write to one MKey of a store touches the store
(`notes/312d` § 22, `sus-a-contained-write-touches-its-store`).

```alloy
run bookScope_stage_five_the_list_file_named {} for 8 but 4 Int
```

```sh
# stage_five_the_list_file_named.sh
   apt-get update
#} apt_get update
#= one sig stdlib, tessa, anna, deb extends Speaker {}
#= one sig sm_Boot extends MSort {} { sortOwner = stdlib }
#= one sig sm_Filesystem, sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_BootId extends MScheme {} { schemeOwner = stdlib }
#= one sig sm_FsId, sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig boot_shape extends MShape {} { ofScheme = sm_BootId }
#= one sig fsid_shape extends MShape {} { ofScheme = sm_FsId }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig the_boot, fs_1, status_inode, list_inode extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig w_boot extends MRootWorld {} { rootShape = boot_shape }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and ambient = sm_Filesystem->k_fs_1 }
#= one sig k_boot extends MKey {} { mValue = boot_2f3a and scheme = sm_BootId and no cellSort and shape = boot_shape and no yielded and at = v0 and mRefersTo = the_boot }
#= one sig k_fs_1 extends MKey {} { mValue = dev_8_1 and scheme = sm_FsId and no cellSort and shape = fsid_shape and no yielded and at = v0 and mRefersTo = fs_1 }
#= one sig k_status_inode extends MKey {} { mValue = ino_9 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = status_inode }
#= one sig k_list_inode extends MKey {} { mValue = ino_31 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = list_inode }
#= one sig k_status_path extends MKey {} { mValue = var_lib_dpkg_status and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_status_inode and at = v0 and mRefersTo = status_inode }
#= one sig k_list_path extends MKey {} { mValue = var_lib_apt_lists_release and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_list_inode and at = v0 and mRefersTo = list_inode }
#= Speaker = stdlib + tessa + anna + deb
#= MSort = sm_Boot + sm_Filesystem + sm_File
#= MScheme = sm_BootId + sm_FsId + sm_Inode + sm_Path
#= MShape = boot_shape + fsid_shape + inode_shape + slash_path_shape
#= MReferent = the_boot + fs_1 + status_inode + list_inode
#= MKey = k_boot + k_fs_1 + k_status_inode + k_list_inode + k_status_path + k_list_path
#= MVantage = v0 and MRoute = r0 and MRootWorld = w_boot
#= no Wrapper and no CompositeKey and no Role and some RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no GivenWhole
#= World.lineWrites = (anna__update_writes_the_list_file).writeLine->list_inode
#= holds = the_boot->fs_1 + fs_1->status_inode + fs_1->list_inode
#= owns = holds
#= passes = fs_1->status_inode + fs_1->list_inode
#= no affects
#= one sig stdlib__the_boot_id_is_primary_of_the_boot extends DeclaresPrimaryOf {} { speaker = stdlib and primaryScheme = sm_BootId and ofSort = sm_Boot }
#= one sig stdlib__a_boot_id_is_a_root extends DeclaresRoot {} { speaker = stdlib and rootedShape = boot_shape }
#= one sig stdlib__the_boots_may_read_set_is_closed extends ClosesMayRead {} { speaker = stdlib and readSort = sm_Boot }
#= one sig tessa__the_filesystem_id_is_primary_of_the_filesystem extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_FsId and ofSort = sm_Filesystem }
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_filesystem_is_identified_in_its_boot extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = fsid_shape and inSort = sm_Boot }
#= one sig tessa__an_inode_is_identified_in_its_filesystem extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = inode_shape and inSort = sm_Filesystem }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__a_path_is_looked_up_in_a_filesystem extends DeclaresCatalogSort {} { speaker = tessa and forScheme = sm_Path and catalogSort = sm_Filesystem }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig tessa__an_inode_has_one_number extends DeclaresUniqueName {} { speaker = tessa and nameShape = inode_shape }
#= one sig tessa__fs_1_is_in_the_boot extends SuppliesParent {} { speaker = tessa and forKey = k_fs_1 and instance = k_boot and seat = DeclarationSeat }
#= one sig tessa__the_status_inode_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_status_inode and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig tessa__the_list_inode_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_list_inode and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig tessa__the_files_may_read_set_is_closed extends ClosesMayRead {} { speaker = tessa and readSort = sm_File }
#= one sig tessa__the_filesystems_may_read_set_is_closed extends ClosesMayRead {} { speaker = tessa and readSort = sm_Filesystem }
#= one sig tessa__writing_a_file_by_its_path_entails_nothing_else extends FinishesEntailment {} { speaker = tessa and finishedSort = sm_File and finishedShape = slash_path_shape }
#= one sig anna__update_writes_the_list_file extends DeclaresMayWrite {} { speaker = anna and writeLine = this and writeEntry = k_list_path }
#= one sig anna__update_writes_nothing_else extends ClosesMayWrite {} { speaker = anna and closedLine = this }
#= axiomaticByContract and axiomaticByDifferentialTest and atMostClosed[this] and atMostEntries[this] = k_list_path and entailmentFinished[k_list_path]

   dpkg -s nginx
#} dpkg dash_s nginx
#= one sig deb__nginx_is_installed extends VerdictFact {} { speaker = deb and topic = k_status_path and atLine = this and markedReads = k_status_path and dependsOn = status_inode }
#= let f = atLine.this, l = (anna__update_writes_the_list_file).writeLine | tabledCompare[k_list_path, f.topic] = DISJOINT and not readsetIsTop[f] and not writesetIsTop[l, writesetsAgainst[l][f.topic]] and routingInvalidatedBy[l, f.topic] and tokenInvalidatedBy[l, f.topic] and staleAt[this, f.topic] and not spared[l, f]
```

<!-- prose-translation -->
> This book's ceiling is eight atoms of every kind the specification owns and integers of four bits.
> The stdlib, Tessa, and Deb speak as in 2.6.2-a-book-stage-five-the-index-given-whole.
> Tessa's declaration also supplies the filesystem as the list file's inode's MParent.
> Tessa also declares the finished record for a file written by its path.
> Anna's `apt-get update` may-writes one list file, named by its slash path and not given whole, and writes nothing else.
> The book holds no world object besides those its lines name.
> The world holds one boot, one filesystem in it, and the status file's inode and the list file's inode in the filesystem.
> Each of these MReferents is owned by what holds it.
> A route through the filesystem passes to each inode.
> No write affects another MReferent.
> The first line writes the list file's inode.
> Every MKey is resolved from one MVantage on one MRoute, with the filesystem as its ambient instance, under no wrapper and with `--risk-faultless-skips` set.
> The book holds no composite MKey and no role.
> No lookup's read set is open, and no entry is given whole.
> Line 1, `apt-get update`: every statement in force is true and the engine's axioms hold.
> At line 1, the line's at-most set is closed, and its one entry is the list file's path.
> That entry's MSort and shape have a finished record.
> Line 2, `dpkg -s nginx`, against line 1: `compare()` answers DISJOINT for the list file's path against the status file's path.
> At line 2, the fact's readset is not ⊤, and line 1's writeset against it is not ⊤.
> Line 1 invalidates the status path's MResolution and its MToken.
> The status path is stale at the fact's site.
> At line 2, the fact is not spared past line 1.

#### § 2.6.4-a-book-stage-five-the-list-file-named-in-the-route

The thin sibling of 2.6.3-a-book-stage-five-the-list-file-named: Tessa describes files and
nothing above them, as in 3.2.5-a-book-two-files-scoped-in-the-route, so both inodes are scoped
in the MRoute and no MKey has an MKey for an MParent. The status inode's MFullyQualifiedKey
ends at the MRoute, which is why the fact's readset is ⊤.

```alloy
run bookScope_stage_five_the_list_file_named_in_the_route {} for 5 but 4 Int
```

```sh
# stage_five_the_list_file_named_in_the_route.sh
   apt-get update
#} apt_get update
#= one sig tessa, anna, deb extends Speaker {}
#= one sig sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig status_inode, list_inode extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig k_status_inode extends MKey {} { mValue = ino_9 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = status_inode }
#= one sig k_list_inode extends MKey {} { mValue = ino_31 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = list_inode }
#= one sig k_status_path extends MKey {} { mValue = var_lib_dpkg_status and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_status_inode and at = v0 and mRefersTo = status_inode }
#= one sig k_list_path extends MKey {} { mValue = var_lib_apt_lists_release and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_list_inode and at = v0 and mRefersTo = list_inode }
#= Speaker = tessa + anna + deb
#= MSort = sm_File
#= MScheme = sm_Inode + sm_Path
#= MShape = inode_shape + slash_path_shape
#= MReferent = status_inode + list_inode
#= MKey = k_status_inode + k_list_inode + k_status_path + k_list_path
#= MVantage = v0 and MRoute = r0 and no MRootWorld
#= no Wrapper and no CompositeKey and no Role and some RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no GivenWhole
#= World.lineWrites = (anna__update_writes_the_list_file).writeLine->list_inode
#= no holds and no owns and no passes and no affects
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__an_inode_has_one_number extends DeclaresUniqueName {} { speaker = tessa and nameShape = inode_shape }
#= one sig tessa__the_files_may_read_set_is_closed extends ClosesMayRead {} { speaker = tessa and readSort = sm_File }
#= one sig tessa__writing_a_file_by_its_path_entails_nothing_else extends FinishesEntailment {} { speaker = tessa and finishedSort = sm_File and finishedShape = slash_path_shape }
#= one sig anna__update_writes_the_list_file extends DeclaresMayWrite {} { speaker = anna and writeLine = this and writeEntry = k_list_path }
#= one sig anna__update_writes_nothing_else extends ClosesMayWrite {} { speaker = anna and closedLine = this }
#= axiomaticByContract and axiomaticByDifferentialTest and atMostClosed[this] and atMostEntries[this] = k_list_path and entailmentFinished[k_list_path]

   dpkg -s nginx
#} dpkg dash_s nginx
#= one sig deb__nginx_is_installed extends VerdictFact {} { speaker = deb and topic = k_status_path and atLine = this and markedReads = k_status_path and dependsOn = status_inode }
#= let f = atLine.this, l = (anna__update_writes_the_list_file).writeLine | tabledCompare[k_list_path, f.topic] = DISJOINT and not writesetIsTop[l, writesetsAgainst[l][f.topic]] and not staleAt[this, f.topic] and readsetIsTop[f] and not spared[l, f]
```

<!-- prose-translation -->
> This book's ceiling is five atoms of every kind the specification owns and integers of four bits.
> Tessa owns the file.
> The inode number's MScheme is `:primary-of` the file.
> Its shape carries `:guarantees-unique-name`, neither `:identified-in` nor `:root`, and no `:guarantees-unique-referent`.
> A slash path `:yields` an inode, and nothing says where a path is looked up.
> Tessa closes the file's may-read set and declares the finished record for a file written by its path.
> Nobody describes a filesystem or a boot.
> Anna and Deb speak as in 2.6.3-a-book-stage-five-the-list-file-named.
> The book holds no world object besides those its lines name.
> The world holds the status file's inode and the list file's inode.
> No store holds them, nothing passes, and no write affects another MReferent.
> The first line writes the list file's inode.
> Every MKey is resolved from one MVantage on one MRoute, which holds no ambient instance, under no wrapper and with `--risk-faultless-skips` set.
> Each inode's MKey is scoped in the MRoute, and each path's MKey has no MParent.
> The book holds no composite MKey and no role.
> No lookup's read set is open, and no entry is given whole.
> Line 1, `apt-get update`: every statement in force is true and the engine's axioms hold.
> At line 1, the line's at-most set is closed, and its one entry is the list file's path.
> That entry's MSort and shape have a finished record.
> Line 2, `dpkg -s nginx`, against line 1: `compare()` answers DISJOINT for the list file's path against the status file's path.
> At line 2, line 1's writeset against the fact is not ⊤, and the status path is not stale at the fact's site.
> At line 2, the fact's readset is ⊤.
> At line 2, the fact is not spared past line 1.

#### § 2.6.5-a-book-two-volumes-of-one-issuer

Two MKeys scoped directly in a MRoot's MWorld: volume identifiers that an issuer mints and does
not repeat. Petra describes the issuer's volumes; Ravi describes the tool. The book writes one
volume above a fact about the other.

```alloy
run bookScope_two_volumes_of_one_issuer {} for 4 but 4 Int
```

```sh
# two_volumes_of_one_issuer.sh
   aws ec2 modify-volume --volume-id vol-0b2 --iops 4000
#} aws ec2 modify_volume dash_dash_volume_id vol_0b2 dash_dash_iops iops_4000
#= one sig petra, ravi extends Speaker {}
#= one sig sm_Volume extends MSort {} { sortOwner = petra }
#= one sig sm_VolumeId extends MScheme {} { schemeOwner = petra }
#= one sig volume_id_shape extends MShape {} { ofScheme = sm_VolumeId }
#= one sig volume_a, volume_b extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig w_volumes extends MRootWorld {} { rootShape = volume_id_shape }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig k_vol_0a1 extends MKey {} { mValue = vol_0a1 and scheme = sm_VolumeId and no cellSort and shape = volume_id_shape and no yielded and at = v0 and mRefersTo = volume_a }
#= one sig k_vol_0b2 extends MKey {} { mValue = vol_0b2 and scheme = sm_VolumeId and no cellSort and shape = volume_id_shape and no yielded and at = v0 and mRefersTo = volume_b }
#= Speaker = petra + ravi
#= MSort = sm_Volume
#= MScheme = sm_VolumeId
#= MShape = volume_id_shape
#= MReferent = volume_a + volume_b
#= MKey = k_vol_0a1 + k_vol_0b2
#= MVantage = v0 and MRoute = r0 and MRootWorld = w_volumes
#= no Wrapper and no CompositeKey and no Role and some RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no GivenWhole
#= World.lineWrites = (ravi__modify_writes_the_volume).writeLine->volume_b
#= no holds and no owns and no passes and no affects
#= one sig petra__the_volume_id_is_primary_of_the_volume extends DeclaresPrimaryOf {} { speaker = petra and primaryScheme = sm_VolumeId and ofSort = sm_Volume }
#= one sig petra__a_volume_id_is_a_root extends DeclaresRoot {} { speaker = petra and rootedShape = volume_id_shape }
#= one sig petra__a_volume_has_one_id extends DeclaresUniqueName {} { speaker = petra and nameShape = volume_id_shape }
#= one sig petra__the_volumes_may_read_set_is_closed extends ClosesMayRead {} { speaker = petra and readSort = sm_Volume }
#= one sig petra__writing_a_volume_entails_nothing_else extends FinishesEntailment {} { speaker = petra and finishedSort = sm_Volume and finishedShape = volume_id_shape }
#= one sig ravi__modify_writes_the_volume extends DeclaresMayWrite {} { speaker = ravi and writeLine = this and writeEntry = k_vol_0b2 }
#= one sig ravi__modify_writes_nothing_else extends ClosesMayWrite {} { speaker = ravi and closedLine = this }
#= axiomaticByContract and axiomaticByDifferentialTest and atMostClosed[this] and atMostEntries[this] = k_vol_0b2 and entailmentFinished[k_vol_0b2] and worldOf[k_vol_0b2] = w_volumes

   aws ec2 describe-volumes --volume-ids vol-0a1
#} aws ec2 describe_volumes dash_dash_volume_ids vol_0a1
#= one sig ravi__the_volume_is_in_use extends VerdictFact {} { speaker = ravi and topic = k_vol_0a1 and atLine = this and markedReads = k_vol_0a1 and dependsOn = volume_a }
#= let f = atLine.this, l = (ravi__modify_writes_the_volume).writeLine | tabledCompare[k_vol_0b2, f.topic] = DISJOINT and not readsetIsTop[f] and not writesetIsTop[l, writesetsAgainst[l][f.topic]] and not staleAt[this, f.topic] and spared[l, f] and no (World.lineWrites[l]).*affects & f.dependsOn
```

<!-- prose-translation -->
> This book's ceiling is four atoms of every kind the specification owns and integers of four bits.
> The atoms include its two MKeys, one MRoute, and one MRoot MWorld.
> Petra owns the volume.
> The volume id's MScheme is `:primary-of` the volume, and its one shape is `:root` and carries `:guarantees-unique-name`.
> Petra closes the volume's may-read set and declares the finished record for a volume written by its id.
> Ravi's `modify-volume` may-writes the one volume it names, not given whole, and writes nothing else.
> Ravi's `describe-volumes` measures the other volume as a read of its id.
> Ravi's fact marks that id as its read and depends on the other volume.
> The book holds no world object besides those its lines name.
> The world holds two volumes.
> No store holds them, nothing passes, and no write affects another MReferent.
> The first line writes the second volume.
> Every MKey is resolved from one MVantage on one MRoute, which holds no ambient instance, under no wrapper and with `--risk-faultless-skips` set.
> Each volume's MKey is scoped in the volume id's MWorld.
> The book holds no composite MKey and no role.
> No lookup's read set is open, and no entry is given whole.
> Line 1, `modify-volume` on `vol-0b2`: every statement in force is true and the engine's axioms hold.
> At line 1, the line's at-most set is closed, and its one entry is that volume's id.
> That entry's MSort and shape have a finished record.
> At line 1, the entry's MFullyQualifiedKey ends at the volume id's MWorld.
> Line 2, `describe-volumes` on `vol-0a1`, against line 1: `compare()` answers DISJOINT for the two ids.
> At line 2, neither the fact's readset nor line 1's writeset against it is ⊤.
> At line 2, the read id is not stale at the fact's site.
> At line 2, the fact is spared past line 1.
> No MReferent that line 1 writes affects, directly or through others, an MReferent the fact's answer depended on.

### § 2.9-tests-of-the-traversal-and-the-region-test

```alloy
run hole_region_closure_with_unknown_leaf_pair_witness {
   hole_region_closure_with_unknown_leaf_pair and axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
} for 6 but 4 Int expect 1

run kill_region_disjoint_is_sound_closes_looked_up_in {
   noStoreIsAmongItsOwnContents
   not hole_region_closure_with_unknown_leaf_pair
   not hole_composite_keys_with_same_parts_refer_differently
   not hole_world_scoped_top_aliases_into_a_store
   axiomaticByDifferentialTest
   some d: ClosesLookedUpIn & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some D, x: MKey | regionTest[D, x] = DISJOINT and some x.mRefersTo & (D.mRefersTo + D.mRefersTo.passes)
} for 5 but 4 Int expect 1

run kill_region_disjoint_is_sound_alias_nothing_else {
   noStoreIsAmongItsOwnContents
   not hole_region_closure_with_unknown_leaf_pair
   not hole_composite_keys_with_same_parts_refer_differently
   not hole_world_scoped_top_aliases_into_a_store
   axiomaticByDifferentialTest
   some d: EmitsAliasNothingElse & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some D, x: MKey | regionTest[D, x] = DISJOINT and some x.mRefersTo & (D.mRefersTo + D.mRefersTo.passes)
} for 5 but 4 Int expect 1
```

<!-- prose-translation -->
> The region law dies with a statement it rests on.
> Each kill of the region law asks outside the three holes, with the engine's axioms holding and no store among its own contents.
> With one `looked-up-in nothing-else` false and every other statement in force true, a false DISJOINT of the region test is reachable.
> One more kill asks whether the law also dies with one `alias nothing-else` false.
> An unsatisfiable kill says that no DISJOINT of the region test rests on that closure alone.

### § 2.10-tests-of-places-the-upward-lookup

#### § 2.10.2-a-book-a-directory-removed-beside-a-file

A directory written whole, against a file the placing lookup says is in another directory. Dan
describes directories and that a directory `:places` a file; his placing lookup records the
file's route and closes it. Rick describes `rm -rf`. Tessa's world is thin: the filesystem is
scoped in the MRoute, so the file's readset is ⊤ and no sparing is reached. The book shows the
region test's DISJOINT through the placing route, the store-given-whole floor invalidating the
path's MResolution and the inode's MToken all the same, and that the world sits inside
`hole_region_closure_with_unknown_leaf_pair`: the file's inode and the directory's inode are
MKeys of two MSorts that the walk never separated.

```alloy
run bookScope_a_directory_removed_beside_a_file {} for 6 but 4 Int
```

```sh
# a_directory_removed_beside_a_file.sh
   rm -rf /srv/b
#} rm dash_rf srv_b
#= one sig tessa, dan, rick, carl extends Speaker {}
#= one sig sm_Filesystem, sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_Directory extends MSort {} { sortOwner = dan }
#= one sig sm_FsId, sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig sm_DirInode extends MScheme {} { schemeOwner = dan }
#= one sig fsid_shape extends MShape {} { ofScheme = sm_FsId }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig dir_shape extends MShape {} { ofScheme = sm_DirInode }
#= one sig fs_1, dir_a, dir_b, inode_a extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and ambient = sm_Filesystem->k_fs_1 }
#= one sig k_fs_1 extends MKey {} { mValue = dev_8_1 and scheme = sm_FsId and no cellSort and shape = fsid_shape and no yielded and at = v0 and mRefersTo = fs_1 }
#= one sig k_dir_a extends MKey {} { mValue = ino_2 and scheme = sm_DirInode and no cellSort and shape = dir_shape and no yielded and at = v0 and mRefersTo = dir_a }
#= one sig k_dir_b extends MKey {} { mValue = ino_3 and scheme = sm_DirInode and no cellSort and shape = dir_shape and no yielded and at = v0 and mRefersTo = dir_b }
#= one sig k_ino_a extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_a }
#= one sig k_srv_a_path extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_a and at = v0 and mRefersTo = inode_a }
#= Speaker = tessa + dan + rick + carl
#= MSort = sm_Filesystem + sm_File + sm_Directory
#= MScheme = sm_FsId + sm_Inode + sm_Path + sm_DirInode
#= MShape = fsid_shape + inode_shape + slash_path_shape + dir_shape
#= MReferent = fs_1 + dir_a + dir_b + inode_a
#= MKey = k_fs_1 + k_dir_a + k_dir_b + k_ino_a + k_srv_a_path
#= MVantage = v0 and MRoute = r0 and no MRootWorld
#= no Wrapper and no CompositeKey and no Role and some RiskFaultlessSkips
#= no Engine.lookupReadSetOpen
#= World.lineWrites = (rick__rm_writes_srv_b).writeLine->dir_b
#= GivenWhole = rick__rm_writes_srv_b
#= holds = fs_1->dir_a + fs_1->dir_b + fs_1->inode_a
#= owns = holds
#= passes = fs_1->inode_a + dir_a->inode_a
#= no affects
#= one sig tessa__the_filesystem_id_is_primary_of_the_filesystem extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_FsId and ofSort = sm_Filesystem }
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__an_inode_is_identified_in_its_filesystem extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = inode_shape and inSort = sm_Filesystem }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__a_path_is_looked_up_in_a_filesystem extends DeclaresCatalogSort {} { speaker = tessa and forScheme = sm_Path and catalogSort = sm_Filesystem }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig tessa__an_inode_has_one_number extends DeclaresUniqueName {} { speaker = tessa and nameShape = inode_shape }
#= one sig tessa__ino_a_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_ino_a and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig dan__the_directory_inode_is_primary_of_the_directory extends DeclaresPrimaryOf {} { speaker = dan and primaryScheme = sm_DirInode and ofSort = sm_Directory }
#= one sig dan__a_directory_is_identified_in_its_filesystem extends DeclaresIdentifiedIn {} { speaker = dan and onShape = dir_shape and inSort = sm_Filesystem }
#= one sig dan__a_directory_inode_reaches_one_directory extends DeclaresUniqueReferent {} { speaker = dan and referentShape = dir_shape }
#= one sig dan__a_directory_has_one_inode extends DeclaresUniqueName {} { speaker = dan and nameShape = dir_shape }
#= one sig dan__dir_a_is_in_fs_1 extends SuppliesParent {} { speaker = dan and forKey = k_dir_a and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig dan__dir_b_is_in_fs_1 extends SuppliesParent {} { speaker = dan and forKey = k_dir_b and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig dan__a_directory_places_a_file extends DeclaresPlaces {} { speaker = dan and placingSort = sm_Directory and placedSort = sm_File }
#= one sig dan__srv_a_app_conf_is_looked_up_in_dir_a extends RecordsLookedUpIn {} { speaker = dan and placedKey = k_srv_a_path and inKey = k_dir_a }
#= one sig dan__srv_a_app_conf_is_looked_up_in_no_other_directory extends ClosesLookedUpIn {} { speaker = dan and closedKey = k_srv_a_path and routeSort = sm_Directory }
#= one sig dan__writing_a_directory_entails_nothing_else extends FinishesEntailment {} { speaker = dan and finishedSort = sm_Directory and finishedShape = dir_shape }
#= one sig rick__rm_writes_srv_b extends DeclaresMayWrite {} { speaker = rick and writeLine = this and writeEntry = k_dir_b }
#= one sig rick__rm_writes_nothing_else extends ClosesMayWrite {} { speaker = rick and closedLine = this }
#= axiomaticByContract and axiomaticByDifferentialTest and atMostClosed[this] and wholeWriteEntries[this] = k_dir_b and entailmentFinished[k_dir_b]

   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig carl__srv_a_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_a_path and atLine = this and markedReads = k_srv_a_path and dependsOn = inode_a }
#= let f = atLine.this, l = (rick__rm_writes_srv_b).writeLine | regionTest[k_dir_b, f.topic] = DISJOINT and tabledCompare[f.topic, k_dir_b] = KNOWN_UNSPOKEN and hole_region_closure_with_unknown_leaf_pair and routingInvalidatedBy[l, f.topic] and tokenInvalidatedBy[l, f.topic] and staleAt[this, f.topic] and readsetIsTop[f] and not spared[l, f]
```

<!-- prose-translation -->
> This book's ceiling is six atoms of every kind the specification owns and integers of four bits.
> Tessa owns the filesystem and the file.
> The filesystem id's MScheme is `:primary-of` the filesystem, and its shape is scoped in the MRoute.
> The inode number's MScheme is `:primary-of` the file, and its shape is `:identified-in` the filesystem with both warrants.
> A slash path `:yields` an inode and is looked up in a filesystem.
> Dan owns the directory.
> The directory inode's MScheme is `:primary-of` the directory, and its shape is `:identified-in` the filesystem with both warrants.
> A directory `:places` a file, and writing a directory entails nothing else.
> Dan's placing lookup records that `/srv/a/app.conf` is looked up in the directory `/srv/a`, and closes the record.
> Tessa's declaration supplies the filesystem as the inode's MParent, and Dan's supplies it as each directory's MParent.
> Rick's `rm -rf` may-writes the directory `/srv/b`, given whole, and writes nothing else.
> Carl's `cmp` measures `/srv/a/app.conf` as a read of its path, marks that path as its read, and depends on the inode.
> The book holds no world object besides those its lines name.
> The world holds one filesystem with two directories and one inode in it, each owned by the filesystem.
> A route through the filesystem passes to the inode, and a route through the first directory passes to the inode.
> No write affects another MReferent, and the first line writes the second directory.
> Every MKey is resolved from one MVantage on one MRoute, with the filesystem as its ambient instance, under no wrapper and with `--risk-faultless-skips` set.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> Line 1, `rm -rf /srv/b`: every statement in force is true and the engine's axioms hold.
> The line's at-most set is closed, and its one entry is the second directory's MKey given whole.
> That entry's MSort and shape have a finished record.
> Line 2, `cmp` against `/srv/a/app.conf`, against line 1: the region test answers DISJOINT for the directory against the file's path.
> `compare()` answers KNOWN_UNSPOKEN for the two.
> This world is inside the held hole for the region test.
> Line 1 invalidates the path's MResolution and the inode's MToken.
> The path is stale at line 2, the fact's readset is ⊤, and the fact is not spared past line 1.

## § 3-tests-of-composition-and-laws

### § 3.2-tests-of-compare-one-chokepoint-four-answers

```alloy
run hole_cell_keys_under_same_parents_refer_differently_witness {
   hole_cell_keys_under_same_parents_refer_differently and axiomaticByContract and axiomaticByDifferentialTest
} for 6 but 4 Int expect 1

run hole_world_scoped_top_aliases_into_a_store_witness {
   hole_world_scoped_top_aliases_into_a_store and axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
} for 6 but 4 Int expect 1

run hole_composite_keys_with_same_parts_refer_differently_witness {
   hole_composite_keys_with_same_parts_refer_differently and axiomaticByContract and axiomaticByDifferentialTest
} for 6 but 4 Int expect 1

run kill_compare_same_is_sound_corresponds {
   not hole_cell_keys_under_same_parents_refer_differently
   not hole_composite_keys_with_same_parts_refer_differently
   axiomaticByDifferentialTest
   some d: DeclaresCorresponds & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | compare[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 5 but 4 Int expect 1

run kill_compare_disjoint_is_sound_corresponds {
   noStoreIsAmongItsOwnContents
   not hole_world_scoped_top_aliases_into_a_store
   not hole_composite_keys_with_same_parts_refer_differently
   axiomaticByDifferentialTest
   some d: DeclaresCorresponds & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | compare[x, y] = DISJOINT and walkOfKeys[x, y] != DISJOINT and some x.mRefersTo & y.mRefersTo
} for 5 but 4 Int expect 1

run kill_same_is_sound_unique_referent {
   not hole_cell_keys_under_same_parents_refer_differently
   axiomaticByDifferentialTest
   some d: DeclaresUniqueReferent & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 6 but 4 Int expect 1

run kill_same_is_sound_yields {
   not hole_cell_keys_under_same_parents_refer_differently
   axiomaticByDifferentialTest
   some d: DeclaresYields & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 5 but 4 Int expect 1

run kill_disjoint_is_sound_unique_name {
   noStoreIsAmongItsOwnContents
   not hole_world_scoped_top_aliases_into_a_store
   axiomaticByDifferentialTest
   some d: DeclaresUniqueName & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = DISJOINT and some x.mRefersTo & y.mRefersTo
} for 5 but 4 Int expect 1

run kill_disjoint_is_sound_aliases_nothing_else {
   noStoreIsAmongItsOwnContents
   not hole_world_scoped_top_aliases_into_a_store
   axiomaticByDifferentialTest
   some d: DeclaresAliasesNothingElse & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = DISJOINT and some x.mRefersTo & y.mRefersTo
} for 6 but 4 Int, 9 Claim expect 1

run kill_same_is_sound_identified_in {
   not hole_cell_keys_under_same_parents_refer_differently
   axiomaticByDifferentialTest
   some d: DeclaresIdentifiedIn & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 5 but 4 Int expect 1

run kill_disjoint_is_sound_identified_in {
   noStoreIsAmongItsOwnContents
   not hole_world_scoped_top_aliases_into_a_store
   axiomaticByDifferentialTest
   some d: DeclaresIdentifiedIn & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = DISJOINT and some x.mRefersTo & y.mRefersTo
} for 5 but 4 Int expect 1

run kill_same_is_sound_root {
   not hole_cell_keys_under_same_parents_refer_differently
   axiomaticByDifferentialTest
   some d: DeclaresRoot & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 5 but 4 Int expect 1

run kill_same_is_sound_supplies_parent {
   not hole_cell_keys_under_same_parents_refer_differently
   axiomaticByDifferentialTest
   some d: SuppliesParent & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 5 but 4 Int expect 1

run kill_disjoint_is_sound_supplies_parent {
   noStoreIsAmongItsOwnContents
   not hole_world_scoped_top_aliases_into_a_store
   axiomaticByDifferentialTest
   some d: SuppliesParent & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = DISJOINT and some x.mRefersTo & y.mRefersTo
} for 5 but 4 Int expect 1

run kill_same_is_sound_closes_lends {
   not hole_cell_keys_under_same_parents_refer_differently
   axiomaticByDifferentialTest
   some d: ClosesLends & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some x, y: MKey | walkOfKeys[x, y] = SAME and x.mRefersTo != y.mRefersTo
} for 5 but 4 Int expect 1

run kill_nobody_spoke_declines_unique_referent {
   no InForce & (DeclaresUniqueName + DeclaresRoot + DeclaresAliasesNothingElse)
   some DeclaresUniqueReferent & InForce
   some disj x, y: MKey | walk[x, y] = SAME
} for 6 but 4 Int expect 1
```

<!-- prose-translation -->
> Each hole has a witness that shows it inhabited.
> Each law dies with a statement it rests on.
> Each kill of a SAME law or a DISJOINT law asks with the engine's axioms holding.
> Such a kill also asks outside its law's holes, and under its law's store premise where the law has one.
> With one `:guarantees-unique-referent` false and every other statement in force true, a false SAME of the walk is reachable.
> With one `:yields` false and every other statement in force true, a false SAME of the walk is reachable.
> With one `:guarantees-unique-name` false and every other statement in force true, a false DISJOINT of the walk is reachable.
> With one `:aliases-nothing-else` false and every other statement in force true, a false DISJOINT of the walk is reachable.
> With one MCorrespondence false and every other statement in force true, a false SAME of `compare()` is reachable.
> With one MCorrespondence false and every other statement in force true, a false DISJOINT of `compare()` that the walk does not give is reachable.
> With a `:guarantees-unique-referent` in force and no other warrant, a SAME is reachable.
> The kill by `:aliases-nothing-else` asks over worlds of nine statements, since its witness holds eight in force at once.
> Six more kills ask whether the walk's laws also die with one more statement false.
> Two of these kills try one `:identified-in` false against the SAME law and the DISJOINT law of the walk.
> One kill tries one `:root` false against the SAME law of the walk.
> Two kills try one supplied MParent instance false against the SAME law and the DISJOINT law of the walk.
> One kill tries one wrapper's completion sentinel false against the SAME law of the walk.
> An unsatisfiable kill says that no SAME or DISJOINT of the walk rests on that statement alone.

#### § 3.2.2-a-book-two-files-in-one-filesystem

The books of this section and the next two are worlds pinned line by line (`plans/30Z` § 4,
`notes/30Y` § 2.4), each self-contained: every world object and every statement in force is
declared on the book's own lines, because a statement's fields name the world objects and a
load file can carry only statements. A book pins every signature to the atoms it names, so
each line's check evaluates the walk in that one world and the book's run is its own witness.
The expected answers are the conductor's hand-walk of the fences over the stated world
(`notes/312d` § 19); a red is triaged as that walk's slip, then a fence's, then a hole, and is
never a reason to restate the world. The actors: the stdlib, at the top of the curve, roots the
boot; Tessa describes filesystems and files; Carl describes `cmp`; Pia describes processes and
their namespaces; Dora describes `docker exec`. The path MScheme is looked up in the filesystem
in every book, a coarse describer's choice that keeps directories out of the world.

```alloy
run bookScope_two_files_one_filesystem {} for 10 but 4 Int
```

```sh
# two_files_one_filesystem.sh
   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig stdlib, tessa, carl extends Speaker {}
#= one sig sm_Boot extends MSort {} { sortOwner = stdlib }
#= one sig sm_Filesystem, sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_BootId extends MScheme {} { schemeOwner = stdlib }
#= one sig sm_FsId, sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig boot_shape extends MShape {} { ofScheme = sm_BootId }
#= one sig fsid_shape extends MShape {} { ofScheme = sm_FsId }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig the_boot, fs_1, inode_17, inode_42 extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig w_boot extends MRootWorld {} { rootShape = boot_shape }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and ambient = sm_Filesystem->k_fs_1 }
#= one sig k_boot extends MKey {} { mValue = boot_2f3a and scheme = sm_BootId and no cellSort and shape = boot_shape and no yielded and at = v0 and mRefersTo = the_boot }
#= one sig k_fs_1 extends MKey {} { mValue = dev_8_1 and scheme = sm_FsId and no cellSort and shape = fsid_shape and no yielded and at = v0 and mRefersTo = fs_1 }
#= one sig k_ino_17_at_line_1, k_ino_17_at_line_3 extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_17 }
#= one sig k_ino_42 extends MKey {} { mValue = ino_42 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_42 }
#= one sig k_srv_a_at_line_1 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_1 and at = v0 and mRefersTo = inode_17 }
#= one sig k_srv_b extends MKey {} { mValue = srv_b_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_42 and at = v0 and mRefersTo = inode_42 }
#= one sig k_srv_a_at_line_3 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_3 and at = v0 and mRefersTo = inode_17 }
#= Speaker = stdlib + tessa + carl
#= MSort = sm_Boot + sm_Filesystem + sm_File
#= MScheme = sm_BootId + sm_FsId + sm_Inode + sm_Path
#= MShape = boot_shape + fsid_shape + inode_shape + slash_path_shape
#= MReferent = the_boot + fs_1 + inode_17 + inode_42
#= MKey = k_boot + k_fs_1 + k_ino_17_at_line_1 + k_ino_17_at_line_3 + k_ino_42 + k_srv_a_at_line_1 + k_srv_b + k_srv_a_at_line_3
#= MVantage = v0 and MRoute = r0 and MRootWorld = w_boot
#= no Wrapper and no CompositeKey and no Role and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= holds = the_boot->fs_1 + fs_1->inode_17 + fs_1->inode_42
#= owns = holds
#= passes = fs_1->inode_17 + fs_1->inode_42
#= no affects
#= one sig stdlib__the_boot_id_is_primary_of_the_boot extends DeclaresPrimaryOf {} { speaker = stdlib and primaryScheme = sm_BootId and ofSort = sm_Boot }
#= one sig stdlib__a_boot_id_is_a_root extends DeclaresRoot {} { speaker = stdlib and rootedShape = boot_shape }
#= one sig tessa__the_filesystem_id_is_primary_of_the_filesystem extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_FsId and ofSort = sm_Filesystem }
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_filesystem_is_identified_in_its_boot extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = fsid_shape and inSort = sm_Boot }
#= one sig tessa__an_inode_is_identified_in_its_filesystem extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = inode_shape and inSort = sm_Filesystem }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__a_path_is_looked_up_in_a_filesystem extends DeclaresCatalogSort {} { speaker = tessa and forScheme = sm_Path and catalogSort = sm_Filesystem }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig tessa__an_inode_has_one_number extends DeclaresUniqueName {} { speaker = tessa and nameShape = inode_shape }
#= one sig tessa__fs_1_is_in_the_boot extends SuppliesParent {} { speaker = tessa and forKey = k_fs_1 and instance = k_boot and seat = DeclarationSeat }
#= one sig tessa__ino_17_at_line_1_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_ino_17_at_line_1 and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig tessa__ino_17_at_line_3_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_ino_17_at_line_3 and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig tessa__ino_42_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_ino_42 and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig carl__srv_a_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_1 and atLine = this and markedReads = k_srv_a_at_line_1 and dependsOn = inode_17 }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_ino_17_at_line_1 and worldOf[f.topic] = w_boot)

   cmp -s ./golden.conf /srv/b/app.conf
#} cmp dash_s golden_conf srv_b_app_conf
#= one sig carl__srv_b_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_b and atLine = this and markedReads = k_srv_b and dependsOn = inode_42 }
#= let f = atLine.this, g = carl__srv_a_matches_golden | tabledCompare[f.topic, g.topic] = DISJOINT and naturalKeyAnswer[f.topic, g.topic] = UNKNOWN and not sameTopic[f, g]

   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig carl__srv_a_matches_golden_again extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_3 and atLine = this and markedReads = k_srv_a_at_line_3 and dependsOn = inode_17 }
#= let f = atLine.this, g = carl__srv_a_matches_golden | tabledCompare[f.topic, g.topic] = SAME and naturalKeyAnswer[f.topic, g.topic] = UNKNOWN and sameTopic[f, g]
```

<!-- prose-translation -->
> This book's ceiling is ten atoms of every kind the specification owns and integers of four bits.
> The atoms include its eight MKeys, one MRoute, and one MRoot MWorld.
> The stdlib roots the boot: the boot id's MScheme is `:primary-of` the boot's MSort, and its one shape is `:root`.
> Tessa owns the filesystem and the file.
> The filesystem id's MScheme is `:primary-of` the filesystem, and its shape is `:identified-in` the boot.
> The inode number's MScheme is `:primary-of` the file, and its shape is `:identified-in` the filesystem.
> The inode number's shape carries `:guarantees-unique-referent` and `:guarantees-unique-name`.
> A slash path `:yields` an inode and is looked up in a filesystem.
> Carl's fact at each line marks the path that line reads as its read and depends on the inode that path MRefers to.
> The book holds no world object besides those its lines name.
> The world holds one boot, one filesystem in it, and two inodes in the filesystem.
> Each of these MReferents is owned by what holds it.
> A route through the filesystem passes to each inode.
> No write affects another MReferent, and no line writes.
> Every MKey is resolved from one MVantage on one MRoute, whose ambient filesystem is the one filesystem, under no wrapper and without the flag.
> The seat that supplies each inode's and the filesystem's MParent is the primary MScheme's declaration.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> Line 1, `cmp` against `/srv/a/app.conf`: every statement in force is true and the engine's axioms hold.
> At line 1, the path's identity is the inode key its lookup emitted, and its MFullyQualifiedKey ends at the boot's MWorld.
> Line 2, `cmp` against `/srv/b/app.conf`, against line 1: `compare()` answers DISJOINT, the natural-key license of 2.1-yields-into-another-scheme answers UNKNOWN, and the two facts are not about one MTopic.
> Line 3, `cmp` against `/srv/a/app.conf` again, against line 1: `compare()` answers SAME, the natural-key license answers UNKNOWN, and the two facts are about one MTopic.

#### § 3.2.3-a-book-a-hardlink-under-a-false-unique-name

Tessa's world with one inode under two paths, and Tessa's `:guarantees-unique-name` on the
path shape in force although it is false in that world (a hardlink is that warrant's failure,
2.3-aliases-nothing-else-the-store-warrant). The book asks the walk by identities and the
natural-key license the same question, and asks which statement in force is the false one.
The natural-key license's DISJOINT is the wrong answer that the one false statement licenses.

```alloy
run bookScope_a_hardlink_under_a_false_unique_name {} for 8 but 4 Int
```

```sh
# a_hardlink_under_a_false_unique_name.sh
   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig stdlib, tessa, carl extends Speaker {}
#= one sig sm_Boot extends MSort {} { sortOwner = stdlib }
#= one sig sm_Filesystem, sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_BootId extends MScheme {} { schemeOwner = stdlib }
#= one sig sm_FsId, sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig boot_shape extends MShape {} { ofScheme = sm_BootId }
#= one sig fsid_shape extends MShape {} { ofScheme = sm_FsId }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig the_boot, fs_1, inode_17 extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig w_boot extends MRootWorld {} { rootShape = boot_shape }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and ambient = sm_Filesystem->k_fs_1 }
#= one sig k_boot extends MKey {} { mValue = boot_2f3a and scheme = sm_BootId and no cellSort and shape = boot_shape and no yielded and at = v0 and mRefersTo = the_boot }
#= one sig k_fs_1 extends MKey {} { mValue = dev_8_1 and scheme = sm_FsId and no cellSort and shape = fsid_shape and no yielded and at = v0 and mRefersTo = fs_1 }
#= one sig k_ino_17_at_line_1, k_ino_17_at_line_2 extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_17 }
#= one sig k_srv_a extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_1 and at = v0 and mRefersTo = inode_17 }
#= one sig k_srv_mirror extends MKey {} { mValue = srv_mirror_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_2 and at = v0 and mRefersTo = inode_17 }
#= Speaker = stdlib + tessa + carl
#= MSort = sm_Boot + sm_Filesystem + sm_File
#= MScheme = sm_BootId + sm_FsId + sm_Inode + sm_Path
#= MShape = boot_shape + fsid_shape + inode_shape + slash_path_shape
#= MReferent = the_boot + fs_1 + inode_17
#= MKey = k_boot + k_fs_1 + k_ino_17_at_line_1 + k_ino_17_at_line_2 + k_srv_a + k_srv_mirror
#= MVantage = v0 and MRoute = r0 and MRootWorld = w_boot
#= no Wrapper and no CompositeKey and no Role and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= holds = the_boot->fs_1 + fs_1->inode_17
#= owns = holds
#= passes = fs_1->inode_17
#= no affects
#= one sig stdlib__the_boot_id_is_primary_of_the_boot extends DeclaresPrimaryOf {} { speaker = stdlib and primaryScheme = sm_BootId and ofSort = sm_Boot }
#= one sig stdlib__a_boot_id_is_a_root extends DeclaresRoot {} { speaker = stdlib and rootedShape = boot_shape }
#= one sig tessa__the_filesystem_id_is_primary_of_the_filesystem extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_FsId and ofSort = sm_Filesystem }
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_filesystem_is_identified_in_its_boot extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = fsid_shape and inSort = sm_Boot }
#= one sig tessa__an_inode_is_identified_in_its_filesystem extends DeclaresIdentifiedIn {} { speaker = tessa and onShape = inode_shape and inSort = sm_Filesystem }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__a_path_is_looked_up_in_a_filesystem extends DeclaresCatalogSort {} { speaker = tessa and forScheme = sm_Path and catalogSort = sm_Filesystem }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig tessa__an_inode_has_one_number extends DeclaresUniqueName {} { speaker = tessa and nameShape = inode_shape }
#= one sig tessa__a_path_names_one_inode extends DeclaresUniqueName {} { speaker = tessa and nameShape = slash_path_shape }
#= one sig tessa__fs_1_is_in_the_boot extends SuppliesParent {} { speaker = tessa and forKey = k_fs_1 and instance = k_boot and seat = DeclarationSeat }
#= one sig tessa__ino_17_at_line_1_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_ino_17_at_line_1 and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig tessa__ino_17_at_line_2_is_in_fs_1 extends SuppliesParent {} { speaker = tessa and forKey = k_ino_17_at_line_2 and instance = k_fs_1 and seat = DeclarationSeat }
#= one sig carl__srv_a_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_a and atLine = this and markedReads = k_srv_a and dependsOn = inode_17 }
#= axiomaticByContractExcept[tessa__a_path_names_one_inode] and axiomaticByDifferentialTest and not tessa__a_path_names_one_inode.isTrue and (let f = atLine.this | identity[f.topic] = k_ino_17_at_line_1)

   cmp -s ./golden.conf /srv/mirror/app.conf
#} cmp dash_s golden_conf srv_mirror_app_conf
#= one sig carl__srv_mirror_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_mirror and atLine = this and markedReads = k_srv_mirror and dependsOn = inode_17 }
#= let f = atLine.this, g = carl__srv_a_matches_golden | tabledCompare[f.topic, g.topic] = SAME and sameTopic[f, g] and naturalKeyAnswer[f.topic, g.topic] = DISJOINT
```

<!-- prose-translation -->
> This book's ceiling is eight atoms of every kind the specification owns and integers of four bits.
> The stdlib, Tessa, and Carl speak as in 3.2.2-a-book-two-files-in-one-filesystem, and Tessa also declares `:guarantees-unique-name` on the slash path shape.
> The book holds no world object besides those its lines name.
> The world holds one boot, one filesystem in it, and one inode in the filesystem.
> Each of these MReferents is owned by what holds it.
> A route through the filesystem passes to the inode.
> No write affects another MReferent, and no line writes.
> Two paths, `/srv/a/app.conf` and `/srv/mirror/app.conf`, each yield an inode key of the one inode's number, and both MRefer to the one inode.
> Every MKey is resolved from one MVantage on one MRoute, whose ambient filesystem is the one filesystem, under no wrapper and without the flag.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> Line 1, `cmp` against `/srv/a/app.conf`: every statement in force is true except Tessa's `:guarantees-unique-name` on the path shape.
> That one statement is false, and the engine's axioms hold.
> At line 1, the path's identity is the inode key its lookup emitted.
> Line 2, `cmp` against `/srv/mirror/app.conf`, against line 1: `compare()` answers SAME and the two facts are about one MTopic.
> At line 2, the natural-key license answers DISJOINT.

#### § 3.2.4-a-book-nested-pid-namespaces

Guest pid 1 and host pid 4821 are one process (`311u:refuted-deriving-the-store-warrant-from-chain-shape`;
2.7-corresponds-across-a-transition's first example). Pia's pids are identified in pid
namespaces, which nest by shape, the initial one identified in the boot; the container's
namespace declares no `:aliases-nothing-else`. The book's second line runs inside the container
through Dora's `docker exec`, which lends the container's namespace; Dora declares the
MCorrespondence, as the transition's owner. Nothing inherits without the sentinel and the flag,
which is why the entered vantage has a MRoute of its own. The SAME comes through the
MCorrespondence, and the two facts are not about one MTopic because a process is
observer-dependent on its namespace by default and the two namespaces compare UNKNOWN.

```alloy
run bookScope_nested_pid_namespaces {} for 8 but 4 Int
```

```sh
# nested_pid_namespaces.sh
   kill -0 4821
#} kill dash_0 pid_4821
#= one sig stdlib, pia, dora extends Speaker {}
#= one sig sm_Boot extends MSort {} { sortOwner = stdlib }
#= one sig sm_PidNamespace, sm_Process extends MSort {} { sortOwner = pia }
#= one sig sm_BootId extends MScheme {} { schemeOwner = stdlib }
#= one sig sm_PidNsId, sm_Pid extends MScheme {} { schemeOwner = pia }
#= one sig boot_shape extends MShape {} { ofScheme = sm_BootId }
#= one sig initial_namespace_shape, nested_namespace_shape extends MShape {} { ofScheme = sm_PidNsId }
#= one sig pid_shape extends MShape {} { ofScheme = sm_Pid }
#= one sig the_boot, ns_0, ns_1, proc_web extends MReferent {}
#= one sig r0, r1 extends MRoute {}
#= one sig w_boot extends MRootWorld {} { rootShape = boot_shape }
#= one sig docker_exec extends Wrapper {} { wrapperOwner = dora }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and ambient = sm_PidNamespace->k_ns_0 }
#= one sig v1 extends MVantage {} { route = r1 and enteredFrom = v0 and through = docker_exec and ambient = sm_PidNamespace->k_ns_1 }
#= one sig k_boot extends MKey {} { mValue = boot_2f3a and scheme = sm_BootId and no cellSort and shape = boot_shape and no yielded and at = v0 and mRefersTo = the_boot }
#= one sig k_ns_0 extends MKey {} { mValue = ns_4026531836 and scheme = sm_PidNsId and no cellSort and shape = initial_namespace_shape and no yielded and at = v0 and mRefersTo = ns_0 }
#= one sig k_ns_1 extends MKey {} { mValue = ns_4026532201 and scheme = sm_PidNsId and no cellSort and shape = nested_namespace_shape and no yielded and at = v0 and mRefersTo = ns_1 }
#= one sig k_pid_4821 extends MKey {} { mValue = pid_4821 and scheme = sm_Pid and no cellSort and shape = pid_shape and no yielded and at = v0 and mRefersTo = proc_web }
#= one sig k_pid_1 extends MKey {} { mValue = pid_1 and scheme = sm_Pid and no cellSort and shape = pid_shape and no yielded and at = v1 and mRefersTo = proc_web }
#= Speaker = stdlib + pia + dora
#= MSort = sm_Boot + sm_PidNamespace + sm_Process
#= MScheme = sm_BootId + sm_PidNsId + sm_Pid
#= MShape = boot_shape + initial_namespace_shape + nested_namespace_shape + pid_shape
#= MReferent = the_boot + ns_0 + ns_1 + proc_web
#= MKey = k_boot + k_ns_0 + k_ns_1 + k_pid_4821 + k_pid_1
#= MVantage = v0 + v1 and MRoute = r0 + r1 and MRootWorld = w_boot and Wrapper = docker_exec
#= no CompositeKey and no Role and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= holds = the_boot->ns_0 + ns_0->ns_1 + ns_0->proc_web + ns_1->proc_web
#= owns = the_boot->ns_0 + ns_0->ns_1 + ns_0->proc_web
#= no passes and no affects
#= one sig stdlib__the_boot_id_is_primary_of_the_boot extends DeclaresPrimaryOf {} { speaker = stdlib and primaryScheme = sm_BootId and ofSort = sm_Boot }
#= one sig stdlib__a_boot_id_is_a_root extends DeclaresRoot {} { speaker = stdlib and rootedShape = boot_shape }
#= one sig pia__the_namespace_id_is_primary_of_the_pid_namespace extends DeclaresPrimaryOf {} { speaker = pia and primaryScheme = sm_PidNsId and ofSort = sm_PidNamespace }
#= one sig pia__the_pid_is_primary_of_the_process extends DeclaresPrimaryOf {} { speaker = pia and primaryScheme = sm_Pid and ofSort = sm_Process }
#= one sig pia__the_initial_namespace_is_identified_in_its_boot extends DeclaresIdentifiedIn {} { speaker = pia and onShape = initial_namespace_shape and inSort = sm_Boot }
#= one sig pia__a_nested_namespace_is_identified_in_its_parent_namespace extends DeclaresIdentifiedIn {} { speaker = pia and onShape = nested_namespace_shape and inSort = sm_PidNamespace }
#= one sig pia__a_pid_is_identified_in_its_namespace extends DeclaresIdentifiedIn {} { speaker = pia and onShape = pid_shape and inSort = sm_PidNamespace }
#= one sig pia__a_pid_reaches_one_process_in_its_namespace extends DeclaresUniqueReferent {} { speaker = pia and referentShape = pid_shape }
#= one sig pia__a_process_has_one_pid_in_a_namespace extends DeclaresUniqueName {} { speaker = pia and nameShape = pid_shape }
#= one sig pia__ns_0_is_in_the_boot extends SuppliesParent {} { speaker = pia and forKey = k_ns_0 and instance = k_boot and seat = DeclarationSeat }
#= one sig pia__ns_1_is_in_ns_0 extends SuppliesParent {} { speaker = pia and forKey = k_ns_1 and instance = k_ns_0 and seat = DeclarationSeat }
#= one sig pia__pid_4821_is_in_ns_0 extends SuppliesParent {} { speaker = pia and forKey = k_pid_4821 and instance = k_ns_0 and seat = DeclarationSeat }
#= one sig pia__pid_1_is_in_ns_1 extends SuppliesParent {} { speaker = pia and forKey = k_pid_1 and instance = k_ns_1 and seat = DeclarationSeat }
#= one sig dora__docker_exec_lends_the_containers_pid_namespace extends DeclaresLends {} { speaker = dora and lendingWrapper = docker_exec and lentSort = sm_PidNamespace and lentInstance = k_ns_1 }
#= one sig dora__guest_pid_1_is_host_pid_4821 extends DeclaresCorresponds {} { speaker = dora and keyX = k_pid_1 and keyY = k_pid_4821 }
#= one sig pia__web_is_running extends VerdictFact {} { speaker = pia and topic = k_pid_4821 and atLine = this and markedReads = k_pid_4821 and dependsOn = proc_web }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_pid_4821 and worldOf[f.topic] = w_boot)

   docker exec web kill -0 1
#} docker exec web kill dash_0 pid_1
#= one sig pia__init_is_running_inside extends VerdictFact {} { speaker = pia and topic = k_pid_1 and atLine = this and markedReads = k_pid_1 and dependsOn = proc_web }
#= let f = atLine.this, g = pia__web_is_running | tabledWalk[f.topic, g.topic] = UNKNOWN and tabledCompare[f.topic, g.topic] = SAME and not sameTopic[f, g]
```

<!-- prose-translation -->
> This book's ceiling is eight atoms of every kind the specification owns and integers of four bits.
> The stdlib roots the boot as in 3.2.2-a-book-two-files-in-one-filesystem.
> Pia owns the pid namespace and the process.
> The namespace id's MScheme is `:primary-of` the pid namespace.
> Its initial shape is `:identified-in` the boot, and its nested shape is `:identified-in` the pid namespace.
> The pid's MScheme is `:primary-of` the process, and its shape is `:identified-in` the pid namespace and carries both warrants.
> Pia's declaration supplies the boot as the initial namespace's MParent, and the initial namespace as the container namespace's MParent.
> Pia's declaration supplies the initial namespace as host pid 4821's MParent, and the container's namespace as guest pid 1's MParent.
> No store declares `:aliases-nothing-else`.
> Dora owns the wrapper `docker exec`.
> The wrapper `:lends` the container's namespace as the pid-namespace instance, and Dora declares no sentinel for it.
> Dora declares that guest pid 1 `:corresponds` to host pid 4821.
> Each of Pia's two facts marks the pid it reads as its read and depends on the process.
> The book holds no world object besides those its lines name.
> The world holds one boot and the initial namespace in it.
> The initial namespace holds the container's namespace and one process.
> The container's namespace holds that process too.
> The boot owns the initial namespace, and the initial namespace owns the container's namespace and the process.
> The container's namespace owns nothing.
> Nothing passes, no write affects another MReferent, and no line writes.
> The first line runs from the host's MVantage, whose ambient namespace is the initial one.
> The second line runs from a MVantage entered through the wrapper.
> That MVantage's ambient namespace is the lent one, and its MRoute is another MRoute.
> The book holds no composite MKey and no role, and the flag is not set.
> No lookup's read set is open.
> Line 1, `kill -0 4821`: every statement in force is true and the engine's axioms hold.
> At line 1, the pid's identity is itself, and its MFullyQualifiedKey ends at the boot's MWorld.
> Line 2, `kill -0 1` inside the container, against line 1: the MFullyQualifiedKey walk answers UNKNOWN.
> At line 2, `compare()` answers SAME.
> At line 2, the two facts are not about one MTopic.

#### § 3.2.5-a-book-two-files-scoped-in-the-route

The thin sibling of 3.2.2-a-book-two-files-in-one-filesystem: the same three lines, where Tessa
describes files and nothing above them. An inode has one number, so she declares
`:guarantees-unique-name` on the inode number. A number alone does not say which filesystem it
is of, so she withholds `:guarantees-unique-referent`. Nobody describes a filesystem, so
nothing says where a path is looked up. Both books stay: the thin world is a description Dorc
must be safe under, and it is no correction of the rooted one.

```alloy
run bookScope_two_files_scoped_in_the_route {} for 7 but 4 Int
```

```sh
# two_files_scoped_in_the_route.sh
   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig tessa, carl extends Speaker {}
#= one sig sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig inode_17, inode_42 extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig k_ino_17_at_line_1, k_ino_17_at_line_3 extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_17 }
#= one sig k_ino_42 extends MKey {} { mValue = ino_42 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_42 }
#= one sig k_srv_a_at_line_1 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_1 and at = v0 and mRefersTo = inode_17 }
#= one sig k_srv_b extends MKey {} { mValue = srv_b_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_42 and at = v0 and mRefersTo = inode_42 }
#= one sig k_srv_a_at_line_3 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_3 and at = v0 and mRefersTo = inode_17 }
#= Speaker = tessa + carl
#= MSort = sm_File
#= MScheme = sm_Inode + sm_Path
#= MShape = inode_shape + slash_path_shape
#= MReferent = inode_17 + inode_42
#= MKey = k_ino_17_at_line_1 + k_ino_17_at_line_3 + k_ino_42 + k_srv_a_at_line_1 + k_srv_b + k_srv_a_at_line_3
#= MVantage = v0 and MRoute = r0 and no MRootWorld
#= no Wrapper and no CompositeKey and no Role and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= no holds and no owns and no passes and no affects
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__an_inode_has_one_number extends DeclaresUniqueName {} { speaker = tessa and nameShape = inode_shape }
#= one sig carl__srv_a_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_1 and atLine = this and markedReads = k_srv_a_at_line_1 and dependsOn = inode_17 }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_ino_17_at_line_1 and worldOf[identity[f.topic]] = r0 and no f.topic.mParent)

   cmp -s ./golden.conf /srv/b/app.conf
#} cmp dash_s golden_conf srv_b_app_conf
#= one sig carl__srv_b_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_b and atLine = this and markedReads = k_srv_b and dependsOn = inode_42 }
#= let f = atLine.this, g = carl__srv_a_matches_golden | tabledCompare[f.topic, g.topic] = DISJOINT and naturalKeyAnswer[f.topic, g.topic] = UNKNOWN and not sameTopic[f, g]

   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig carl__srv_a_matches_golden_again extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_3 and atLine = this and markedReads = k_srv_a_at_line_3 and dependsOn = inode_17 }
#= let f = atLine.this, g = carl__srv_a_matches_golden | tabledWalk[f.topic, g.topic] = UNKNOWN and tabledCompare[f.topic, g.topic] = UNKNOWN and naturalKeyAnswer[f.topic, g.topic] = UNKNOWN and not sameTopic[f, g]
```

<!-- prose-translation -->
> This book's ceiling is seven atoms of every kind the specification owns and integers of four bits.
> The atoms include its six MKeys and one MRoute.
> Tessa owns the file.
> The inode number's MScheme is `:primary-of` the file.
> Its shape carries `:guarantees-unique-name`, neither `:identified-in` nor `:root`, and no `:guarantees-unique-referent`.
> A slash path `:yields` an inode, and nothing says where a path is looked up.
> Nobody describes a filesystem or a boot.
> Carl's fact at each line marks the path that line reads as its read and depends on the inode that path MRefers to.
> The book holds no world object besides those its lines name.
> The world holds two inodes.
> No store holds them, nothing passes, no write affects another MReferent, and no line writes.
> Every MKey is resolved from one MVantage on one MRoute, which holds no ambient instance, under no wrapper and without the flag.
> Each inode's MKey is scoped in the MRoute, and each path's MKey has no MParent.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> Line 1, `cmp` against `/srv/a/app.conf`: every statement in force is true and the engine's axioms hold.
> At line 1, the path's identity is the inode key its lookup emitted, whose MFullyQualifiedKey ends at the MRoute.
> At line 1, the path's own MKey has no MParent.
> Line 2, `cmp` against `/srv/b/app.conf`, against line 1: `compare()` answers DISJOINT, the natural-key license of 2.1-yields-into-another-scheme answers UNKNOWN, and the two facts are not about one MTopic.
> Line 3, `cmp` against `/srv/a/app.conf` again, against line 1: the MFullyQualifiedKey walk and `compare()` answer UNKNOWN.
> At line 3, the natural-key license answers UNKNOWN, and the two facts are not about one MTopic.

#### § 3.2.6-a-book-two-cells-of-one-unit

Two cells of one MParent are two MSorts (1.9-cell-a-singleton-sort), and the walk decides
between them as between any two MSorts. Sven describes units and two of their cells; the unit
name is scoped in the MRoute, so the world is thin. The book reads one cell twice, under two
MKey atoms for the unit, and reads the other cell once.

```alloy
run bookScope_two_cells_of_one_unit {} for 6 but 4 Int
```

```sh
# two_cells_of_one_unit.sh
   systemctl is-active nginx.service
#} systemctl is_active nginx_service
#= one sig sven extends Speaker {}
#= one sig sm_Unit, sm_UnitActive, sm_UnitEnabled extends MSort {} { sortOwner = sven }
#= one sig sm_UnitName extends MScheme {} { schemeOwner = sven }
#= one sig unit_name_shape extends MShape {} { ofScheme = sm_UnitName }
#= one sig unit_nginx, active_state, enabled_state extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig k_nginx_at_line_1, k_nginx_at_line_3 extends MKey {} { mValue = nginx_service and scheme = sm_UnitName and no cellSort and shape = unit_name_shape and no yielded and at = v0 and mRefersTo = unit_nginx }
#= one sig k_active_at_line_1, k_active_at_line_3 extends MKey {} { mValue = nginx_at_active and no scheme and cellSort = sm_UnitActive and no shape and no yielded and at = v0 and mRefersTo = active_state }
#= one sig k_enabled extends MKey {} { mValue = nginx_at_enabled and no scheme and cellSort = sm_UnitEnabled and no shape and no yielded and at = v0 and mRefersTo = enabled_state }
#= Speaker = sven
#= MSort = sm_Unit + sm_UnitActive + sm_UnitEnabled
#= MScheme = sm_UnitName
#= MShape = unit_name_shape
#= MReferent = unit_nginx + active_state + enabled_state
#= MKey = k_nginx_at_line_1 + k_nginx_at_line_3 + k_active_at_line_1 + k_active_at_line_3 + k_enabled
#= MVantage = v0 and MRoute = r0 and no MRootWorld
#= no Wrapper and no CompositeKey and no Role and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= holds = unit_nginx->active_state + unit_nginx->enabled_state
#= owns = holds
#= no passes and no affects
#= one sig sven__the_unit_name_is_primary_of_the_unit extends DeclaresPrimaryOf {} { speaker = sven and primaryScheme = sm_UnitName and ofSort = sm_Unit }
#= one sig sven__a_unit_name_reaches_one_unit extends DeclaresUniqueReferent {} { speaker = sven and referentShape = unit_name_shape }
#= one sig sven__active_is_a_cell_of_a_unit extends DeclaresCell {} { speaker = sven and theCell = sm_UnitActive and cellParent = sm_Unit }
#= one sig sven__enabled_is_a_cell_of_a_unit extends DeclaresCell {} { speaker = sven and theCell = sm_UnitEnabled and cellParent = sm_Unit }
#= one sig sven__the_active_cell_at_line_1_is_of_nginx extends SuppliesParent {} { speaker = sven and forKey = k_active_at_line_1 and instance = k_nginx_at_line_1 and seat = BindSeat }
#= one sig sven__the_active_cell_at_line_3_is_of_nginx extends SuppliesParent {} { speaker = sven and forKey = k_active_at_line_3 and instance = k_nginx_at_line_3 and seat = BindSeat }
#= one sig sven__the_enabled_cell_is_of_nginx extends SuppliesParent {} { speaker = sven and forKey = k_enabled and instance = k_nginx_at_line_1 and seat = BindSeat }
#= one sig sven__nginx_is_active extends VerdictFact {} { speaker = sven and topic = k_active_at_line_1 and atLine = this and markedReads = k_active_at_line_1 and dependsOn = active_state }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_active_at_line_1 and f.topic.mParent = k_nginx_at_line_1 and worldOf[f.topic] = r0)

   systemctl is-enabled nginx.service
#} systemctl is_enabled nginx_service
#= one sig sven__nginx_is_enabled extends VerdictFact {} { speaker = sven and topic = k_enabled and atLine = this and markedReads = k_enabled and dependsOn = enabled_state }
#= let f = atLine.this, g = sven__nginx_is_active | tabledCompare[f.topic, g.topic] = KNOWN_UNSPOKEN and not sameTopic[f, g]

   systemctl is-active nginx.service
#} systemctl is_active nginx_service
#= one sig sven__nginx_is_active_again extends VerdictFact {} { speaker = sven and topic = k_active_at_line_3 and atLine = this and markedReads = k_active_at_line_3 and dependsOn = active_state }
#= let f = atLine.this, g = sven__nginx_is_active | tabledCompare[f.topic, g.topic] = SAME and sameTopic[f, g]
```

<!-- prose-translation -->
> This book's ceiling is six atoms of every kind the specification owns and integers of four bits.
> Sven owns the unit and two of its cells.
> The unit name's MScheme is `:primary-of` the unit.
> Its one shape carries `:guarantees-unique-referent` and neither `:identified-in` nor `:root`.
> The active cell and the enabled cell are each `:identified-in` the unit.
> Sven's fact at each line marks the cell MKey that line reads as its read and depends on that cell's state.
> The book holds no world object besides those its lines name.
> The world holds one unit with its active state and its enabled state, each owned by the unit.
> Nothing passes, no write affects another MReferent, and no line writes.
> Every MKey is resolved from one MVantage on one MRoute, which holds no ambient instance, under no wrapper and without the flag.
> The unit's MKey is scoped in the MRoute.
> Each active cell's MKey has the unit's MKey of its own line as its MParent, supplied by the mark that named it.
> The enabled cell's MKey has the unit's MKey of line 1 as its MParent, supplied by the mark that named it.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> Line 1, `is-active`: every statement in force is true and the engine's axioms hold.
> The active cell's identity is its own MKey, its MParent is the unit's MKey, and its chain ends at the MRoute.
> Line 2, `is-enabled`, against line 1: `compare()` answers KNOWN_UNSPOKEN, and the two facts are not about one MTopic.
> Line 3, `is-active` again, against line 1: `compare()` answers SAME, and the two facts are about one MTopic.

#### § 3.2.7-a-book-one-configuration-from-two-files-in-two-orders

A composite MKey names one part per role (2.11-composite-sorts-and-roles), and the same two
parts in swapped roles are another MKey (`composite-identity-is-structure-not-a-bag`). Cora
describes a tool that merges a base file with an overlay; Tessa's files are thin, scoped in the
MRoute. The walk over the composites answers UNKNOWN because the merge shape carries no warrant.

```alloy
run bookScope_one_configuration_from_two_files_in_two_orders {} for 6 but 4 Int
```

```sh
# one_configuration_from_two_files_in_two_orders.sh
   cfg --base ./a.toml --overlay ./b.toml
#} cfg dash_dash_base a_toml dash_dash_overlay b_toml
#= one sig tessa, cora extends Speaker {}
#= one sig sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_MergedConfig extends MSort {} { sortOwner = cora }
#= one sig sm_Inode extends MScheme {} { schemeOwner = tessa }
#= one sig sm_MergeKey extends MScheme {} { schemeOwner = cora }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig merge_shape extends MShape {} { ofScheme = sm_MergeKey }
#= one sig base_role, overlay_role extends Role {}
#= one sig inode_a, inode_b, merged_a_over_b, merged_b_over_a extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig k_ino_a extends MKey {} { mValue = ino_7 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_a }
#= one sig k_ino_b extends MKey {} { mValue = ino_9 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_b }
#= one sig k_a_over_b_at_line_1, k_a_over_b_at_line_3 extends MKey {} { mValue = merge_a_b and scheme = sm_MergeKey and no cellSort and shape = merge_shape and no yielded and at = v0 and mRefersTo = merged_a_over_b }
#= one sig k_b_over_a extends MKey {} { mValue = merge_b_a and scheme = sm_MergeKey and no cellSort and shape = merge_shape and no yielded and at = v0 and mRefersTo = merged_b_over_a }
#= part = k_a_over_b_at_line_1->base_role->k_ino_a + k_a_over_b_at_line_1->overlay_role->k_ino_b + k_a_over_b_at_line_3->base_role->k_ino_a + k_a_over_b_at_line_3->overlay_role->k_ino_b + k_b_over_a->base_role->k_ino_b + k_b_over_a->overlay_role->k_ino_a
#= Speaker = tessa + cora
#= MSort = sm_File + sm_MergedConfig
#= MScheme = sm_Inode + sm_MergeKey
#= MShape = inode_shape + merge_shape
#= Role = base_role + overlay_role
#= MReferent = inode_a + inode_b + merged_a_over_b + merged_b_over_a
#= MKey = k_ino_a + k_ino_b + k_a_over_b_at_line_1 + k_a_over_b_at_line_3 + k_b_over_a
#= CompositeKey = k_a_over_b_at_line_1 + k_a_over_b_at_line_3 + k_b_over_a
#= MVantage = v0 and MRoute = r0 and no MRootWorld
#= no Wrapper and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= no holds and no owns and no passes and no affects
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig cora__the_merge_key_is_primary_of_the_merged_config extends DeclaresPrimaryOf {} { speaker = cora and primaryScheme = sm_MergeKey and ofSort = sm_MergedConfig }
#= one sig cora__a_merged_config_is_a_composite extends DeclaresComposite {} { speaker = cora and compositeSort = sm_MergedConfig }
#= one sig cora__a_over_b_is_valid extends VerdictFact {} { speaker = cora and topic = k_a_over_b_at_line_1 and atLine = this and markedReads = k_a_over_b_at_line_1 and dependsOn = merged_a_over_b }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_a_over_b_at_line_1 and worldOf[f.topic] = r0)

   cfg --base ./b.toml --overlay ./a.toml
#} cfg dash_dash_base b_toml dash_dash_overlay a_toml
#= one sig cora__b_over_a_is_valid extends VerdictFact {} { speaker = cora and topic = k_b_over_a and atLine = this and markedReads = k_b_over_a and dependsOn = merged_b_over_a }
#= let f = atLine.this, g = cora__a_over_b_is_valid | not compositeSame[f.topic, g.topic] and tabledCompare[f.topic, g.topic] = UNKNOWN and not sameTopic[f, g]

   cfg --base ./a.toml --overlay ./b.toml
#} cfg dash_dash_base a_toml dash_dash_overlay b_toml
#= one sig cora__a_over_b_is_valid_again extends VerdictFact {} { speaker = cora and topic = k_a_over_b_at_line_3 and atLine = this and markedReads = k_a_over_b_at_line_3 and dependsOn = merged_a_over_b }
#= let f = atLine.this, g = cora__a_over_b_is_valid | tabledWalk[f.topic, g.topic] = UNKNOWN and compositeSame[f.topic, g.topic] and tabledCompare[f.topic, g.topic] = SAME and sameTopic[f, g]
```

<!-- prose-translation -->
> This book's ceiling is six atoms of every kind the specification owns and integers of four bits.
> Tessa owns the file.
> The inode number's MScheme is `:primary-of` the file, and its shape carries `:guarantees-unique-referent` and nothing else.
> Cora owns the merged configuration, a MCompositeSort with a base role and an overlay role.
> The merge key's MScheme is `:primary-of` it, and its shape carries no warrant.
> The world holds two inodes and two merged configurations.
> No store holds them, nothing passes, no write affects another MReferent, and no line writes.
> Every MKey is resolved from one MVantage on one MRoute, which holds no ambient instance, under no wrapper and without the flag.
> Every MKey is scoped in the MRoute.
> Each composite MKey names one inode per role, and the second line's composite MKey names the two inodes in swapped roles.
> Cora's fact at each line marks the composite MKey that line reads as its read and depends on that merged configuration.
> The book holds no world object besides those its lines name.
> No lookup's read set is open.
> Line 1, `a.toml` under `b.toml`: every statement in force is true and the engine's axioms hold.
> The composite MKey's identity is itself, and its chain ends at the MRoute.
> Line 2, `b.toml` under `a.toml`, against line 1: the composites are not SAME by their parts.
> `compare()` answers UNKNOWN, and the two facts are not about one MTopic.
> Line 3, `a.toml` under `b.toml` again, against line 1: the walk over the composite MKeys answers UNKNOWN.
> The composites are SAME by their parts, role by role.
> `compare()` answers SAME, and the two facts are about one MTopic.

### § 3.3-tests-of-invalidation-three-mutator-species

```alloy
run hole_unclosed_traversal_without_a_key_catalog_witness {
   hole_unclosed_traversal_without_a_key_catalog and axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
} for 6 but 4 Int expect 1

run hole_natural_key_catalog_off_the_route_witness {
   hole_natural_key_catalog_off_the_route and axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
} for 6 but 4 Int expect 1

run hole_a_route_off_the_catalog_reaches_the_thing_witness {
   hole_a_route_off_the_catalog_reaches_the_thing and axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
} for 6 but 4 Int expect 1

run kill_unstale_route_is_untouched_closes_traversal {
   noStoreIsAmongItsOwnContents and noRoutePassesThroughItself
   not hole_unclosed_traversal_without_a_key_catalog
   not hole_natural_key_catalog_off_the_route
   not hole_a_route_off_the_catalog_reaches_the_thing
   not hole_region_closure_with_unknown_leaf_pair
   axiomaticByDifferentialTest
   some d: ClosesTraversal & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some s: Line, k: MKey, l: s.above | not routingInvalidatedBy[l, k] and atMostClosed[l]
         and some World.lineWrites[l] & passes.(levelsOf[k].mRefersTo + k.mRefersTo)
} for 6 but 4 Int, 9 Claim expect 1

run kill_unstale_route_is_untouched_closes_may_write {
   noStoreIsAmongItsOwnContents and noRoutePassesThroughItself
   not hole_unclosed_traversal_without_a_key_catalog
   not hole_natural_key_catalog_off_the_route
   not hole_a_route_off_the_catalog_reaches_the_thing
   not hole_region_closure_with_unknown_leaf_pair
   axiomaticByDifferentialTest
   some d: ClosesMayWrite & InForce | axiomaticByContractExcept[d] and not d.isTrue
      and some s: Line, k: MKey, l: s.above | not routingInvalidatedBy[l, k] and atMostClosed[l]
         and some World.lineWrites[l] & passes.(levelsOf[k].mRefersTo + k.mRefersTo)
} for 6 but 4 Int, 9 Claim expect 1
```

<!-- prose-translation -->
> The untouched-route law dies with a statement it rests on.
> Each kill of the untouched-route law asks outside the law's four holes, with the engine's axioms holding.
> Each such kill also asks while no store is among its own contents and no route passes through itself.
> The law dies with one traversal's closing act false and every other statement in force true.
> The law dies with one completion record false and every other statement in force true.
> In each such world, a line that invalidates no MResolution of an MKey writes something that a route to that MKey's MReferents passes through.

#### § 3.3.2-a-book-a-reboot-between-two-reads

A lifecycle write to a MRoot-adjacent MKey: Rob describes `reboot` as writing the boot's own
MKey. Pia's pids are identified in the boot for this book. The fences hold no instant, so the
world is one world and both pid MKeys MRefer to one process; what the book shows is the engine
withdrawing authority below the line, and that it withdraws it from the MKey resolved after the
reboot as well, since no fence holds when an MKey was resolved.

```alloy
run bookScope_a_reboot_between_two_reads {} for 5 but 4 Int
```

```sh
# a_reboot_between_two_reads.sh
   kill -0 4821
#} kill dash_0 pid_4821
#= one sig stdlib, pia, rob extends Speaker {}
#= one sig sm_Boot extends MSort {} { sortOwner = stdlib }
#= one sig sm_Process extends MSort {} { sortOwner = pia }
#= one sig sm_BootId extends MScheme {} { schemeOwner = stdlib }
#= one sig sm_Pid extends MScheme {} { schemeOwner = pia }
#= one sig boot_shape extends MShape {} { ofScheme = sm_BootId }
#= one sig pid_shape extends MShape {} { ofScheme = sm_Pid }
#= one sig the_boot, proc_web extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig w_boot extends MRootWorld {} { rootShape = boot_shape }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig k_boot extends MKey {} { mValue = boot_2f3a and scheme = sm_BootId and no cellSort and shape = boot_shape and no yielded and at = v0 and mRefersTo = the_boot }
#= one sig k_pid_at_line_1, k_pid_at_line_3 extends MKey {} { mValue = pid_4821 and scheme = sm_Pid and no cellSort and shape = pid_shape and no yielded and at = v0 and mRefersTo = proc_web }
#= Speaker = stdlib + pia + rob
#= MSort = sm_Boot + sm_Process
#= MScheme = sm_BootId + sm_Pid
#= MShape = boot_shape + pid_shape
#= MReferent = the_boot + proc_web
#= MKey = k_boot + k_pid_at_line_1 + k_pid_at_line_3
#= MVantage = v0 and MRoute = r0 and MRootWorld = w_boot
#= no Wrapper and no CompositeKey and no Role and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no GivenWhole
#= World.lineWrites = (rob__reboot_writes_the_boot).writeLine->the_boot
#= holds = the_boot->proc_web
#= owns = holds
#= no passes and no affects
#= one sig stdlib__the_boot_id_is_primary_of_the_boot extends DeclaresPrimaryOf {} { speaker = stdlib and primaryScheme = sm_BootId and ofSort = sm_Boot }
#= one sig stdlib__a_boot_id_is_a_root extends DeclaresRoot {} { speaker = stdlib and rootedShape = boot_shape }
#= one sig pia__the_pid_is_primary_of_the_process extends DeclaresPrimaryOf {} { speaker = pia and primaryScheme = sm_Pid and ofSort = sm_Process }
#= one sig pia__a_pid_is_identified_in_its_boot extends DeclaresIdentifiedIn {} { speaker = pia and onShape = pid_shape and inSort = sm_Boot }
#= one sig pia__a_pid_reaches_one_process_in_its_boot extends DeclaresUniqueReferent {} { speaker = pia and referentShape = pid_shape }
#= one sig pia__pid_4821_at_line_1_is_in_the_boot extends SuppliesParent {} { speaker = pia and forKey = k_pid_at_line_1 and instance = k_boot and seat = DeclarationSeat }
#= one sig pia__pid_4821_at_line_3_is_in_the_boot extends SuppliesParent {} { speaker = pia and forKey = k_pid_at_line_3 and instance = k_boot and seat = DeclarationSeat }
#= one sig pia__web_is_running extends VerdictFact {} { speaker = pia and topic = k_pid_at_line_1 and atLine = this and markedReads = k_pid_at_line_1 and dependsOn = proc_web }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_pid_at_line_1 and f.topic.mParent = k_boot and worldOf[f.topic] = w_boot)

   reboot
#} reboot
#= one sig rob__reboot_writes_the_boot extends DeclaresMayWrite {} { speaker = rob and writeLine = this and writeEntry = k_boot }
#= one sig rob__reboot_writes_nothing_else extends ClosesMayWrite {} { speaker = rob and closedLine = this }
#= atMostClosed[this] and atMostEntries[this] = k_boot and some k_boot.mParent & MRootWorld

   kill -0 4821
#} kill dash_0 pid_4821
#= one sig pia__web_is_running_again extends VerdictFact {} { speaker = pia and topic = k_pid_at_line_3 and atLine = this and markedReads = k_pid_at_line_3 and dependsOn = proc_web }
#= let f = atLine.this, g = pia__web_is_running, l = (rob__reboot_writes_the_boot).writeLine | tabledCompare[f.topic, g.topic] = SAME and lifecycleInvalidatedBy[l, g.topic] and lifecycleInvalidatedBy[l, f.topic] and staleAt[this, g.topic] and staleAt[this, f.topic] and compareAt[this, f.topic, g.topic] = UNKNOWN
```

<!-- prose-translation -->
> This book's ceiling is five atoms of every kind the specification owns and integers of four bits.
> The stdlib roots the boot as in 3.2.2-a-book-two-files-in-one-filesystem.
> Pia owns the process.
> The pid's MScheme is `:primary-of` the process, and its shape is `:identified-in` the boot and carries `:guarantees-unique-referent`.
> Pia's declaration supplies the boot as each pid MKey's MParent.
> Rob's `reboot` may-writes the boot's MKey, not given whole, and writes nothing else.
> Pia's fact at each `kill -0` line marks the pid MKey that line reads as its read and depends on the process.
> The book holds no world object besides those its lines name.
> The world holds one boot and one process in it, owned by the boot.
> Nothing passes, no write affects another MReferent, and the second line writes the boot.
> Every MKey is resolved from one MVantage on one MRoute, which holds no ambient instance, under no wrapper and without the flag.
> The book holds no composite MKey and no role.
> No lookup's read set is open, and no entry is given whole.
> Line 1, `kill -0 4821`: every statement in force is true and the engine's axioms hold.
> The pid's identity is itself, its MParent is the boot's MKey, and its chain ends at the boot's MWorld.
> Line 2, `reboot`: the line's at-most set is closed, its one entry is the boot's MKey, and that MKey is scoped in a MRoot MWorld.
> Line 3, `kill -0 4821` again, against line 1: the timeless `compare()` answers SAME.
> The reboot is a lifecycle write to an MKey on both pids' chains.
> Both pid MKeys are stale at line 3.
> `compare()` at line 3 answers UNKNOWN.

### § 3.4-tests-of-entry-and-lends

#### § 3.4.2-a-book-one-file-across-sudo-under-the-sentinel

Tessa's thin world of 3.2.5-a-book-two-files-scoped-in-the-route, read once from the host and
once through `sudo`. Wanda describes `sudo` as a wrapper that lends nothing and declares its
completion sentinel; the flag is set, so the vantage entered through the wrapper inherits the
caller's MRoute, and the two inode MKeys are scoped in one MRoute. The book asks whether the
sentinel is in the SAME's support. The sibling book 3.4.3-a-book-one-file-across-sudo-without-the-flag
withholds the flag.

```alloy
run bookScope_one_file_across_sudo_under_the_sentinel {} for 5 but 4 Int
```

```sh
# one_file_across_sudo_under_the_sentinel.sh
   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig tessa, carl, wanda extends Speaker {}
#= one sig sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig inode_17 extends MReferent {}
#= one sig r0 extends MRoute {}
#= one sig sudo extends Wrapper {} { wrapperOwner = wanda }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig v1 extends MVantage {} { enteredFrom = v0 and through = sudo }
#= one sig k_ino_17_at_line_1 extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_17 }
#= one sig k_ino_17_at_line_2 extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v1 and mRefersTo = inode_17 }
#= one sig k_srv_a_at_line_1 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_1 and at = v0 and mRefersTo = inode_17 }
#= one sig k_srv_a_at_line_2 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_2 and at = v1 and mRefersTo = inode_17 }
#= Speaker = tessa + carl + wanda
#= MSort = sm_File
#= MScheme = sm_Inode + sm_Path
#= MShape = inode_shape + slash_path_shape
#= MReferent = inode_17
#= MKey = k_ino_17_at_line_1 + k_ino_17_at_line_2 + k_srv_a_at_line_1 + k_srv_a_at_line_2
#= MVantage = v0 + v1 and MRoute = r0 and no MRootWorld and Wrapper = sudo
#= no CompositeKey and no Role and some RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= no holds and no owns and no passes and no affects
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig wanda__sudo_lends_nothing_else extends ClosesLends {} { speaker = wanda and closedWrapper = sudo }
#= one sig carl__srv_a_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_1 and atLine = this and markedReads = k_srv_a_at_line_1 and dependsOn = inode_17 }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_ino_17_at_line_1 and worldOf[identity[f.topic]] = r0)

   sudo cmp -s ./golden.conf /srv/a/app.conf
#} sudo cmp dash_s golden_conf srv_a_app_conf
#= one sig carl__srv_a_matches_golden_as_root extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_2 and atLine = this and markedReads = k_srv_a_at_line_2 and dependsOn = inode_17 }
#= let f = atLine.this, g = carl__srv_a_matches_golden | inherits[v1] and v1.route = r0 and tabledCompare[f.topic, g.topic] = SAME and sameTopic[f, g] and wanda__sudo_lends_nothing_else in sameSupport[f.topic, g.topic]
```

<!-- prose-translation -->
> This book's ceiling is five atoms of every kind the specification owns and integers of four bits.
> Tessa owns the file, as in 3.2.5-a-book-two-files-scoped-in-the-route, with `:guarantees-unique-referent` on the inode number's shape and no other warrant.
> Wanda owns the wrapper `sudo`.
> It lends nothing, and Wanda declares its completion sentinel.
> Carl's fact at each line marks the path that line reads as its read and depends on the inode.
> The book holds no world object besides those its lines name.
> The world holds one inode.
> No store holds it, nothing passes, no write affects another MReferent, and no line writes.
> The first line runs from the host's MVantage, which holds no ambient instance.
> The second line runs from a MVantage entered through the wrapper.
> The flag is set.
> Both inode MKeys are scoped in the one MRoute.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> Line 1, `cmp` from the host: every statement in force is true and the engine's axioms hold.
> The path's identity is the inode MKey its lookup emitted, whose chain ends at the MRoute.
> Line 2, `cmp` through `sudo`, against line 1: the entered MVantage inherits, and its MRoute is the caller's.
> `compare()` answers SAME, the two facts are about one MTopic, and Wanda's sentinel is in the SAME's support.

#### § 3.4.3-a-book-one-file-across-sudo-without-the-flag

The world of 3.4.2-a-book-one-file-across-sudo-under-the-sentinel with the flag withheld.
Nothing inherits, so the vantage entered through the wrapper has a MRoute of its own, and the
two inode MKeys are scoped in two MRoutes.

```alloy
run bookScope_one_file_across_sudo_without_the_flag {} for 6 but 4 Int
```

```sh
# one_file_across_sudo_without_the_flag.sh
   cmp -s ./golden.conf /srv/a/app.conf
#} cmp dash_s golden_conf srv_a_app_conf
#= one sig tessa, carl, wanda extends Speaker {}
#= one sig sm_File extends MSort {} { sortOwner = tessa }
#= one sig sm_Inode, sm_Path extends MScheme {} { schemeOwner = tessa }
#= one sig inode_shape extends MShape {} { ofScheme = sm_Inode }
#= one sig slash_path_shape extends MShape {} { ofScheme = sm_Path }
#= one sig inode_17 extends MReferent {}
#= one sig r0, r1 extends MRoute {}
#= one sig sudo extends Wrapper {} { wrapperOwner = wanda }
#= one sig v0 extends MVantage {} { route = r0 and no enteredFrom and no through and no ambient }
#= one sig v1 extends MVantage {} { route = r1 and enteredFrom = v0 and through = sudo }
#= one sig k_ino_17_at_line_1 extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v0 and mRefersTo = inode_17 }
#= one sig k_ino_17_at_line_2 extends MKey {} { mValue = ino_17 and scheme = sm_Inode and no cellSort and shape = inode_shape and no yielded and at = v1 and mRefersTo = inode_17 }
#= one sig k_srv_a_at_line_1 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_1 and at = v0 and mRefersTo = inode_17 }
#= one sig k_srv_a_at_line_2 extends MKey {} { mValue = srv_a_app_conf and scheme = sm_Path and no cellSort and shape = slash_path_shape and yielded = k_ino_17_at_line_2 and at = v1 and mRefersTo = inode_17 }
#= Speaker = tessa + carl + wanda
#= MSort = sm_File
#= MScheme = sm_Inode + sm_Path
#= MShape = inode_shape + slash_path_shape
#= MReferent = inode_17
#= MKey = k_ino_17_at_line_1 + k_ino_17_at_line_2 + k_srv_a_at_line_1 + k_srv_a_at_line_2
#= MVantage = v0 + v1 and MRoute = r0 + r1 and no MRootWorld and Wrapper = sudo
#= no CompositeKey and no Role and no RiskFaultlessSkips
#= no Engine.lookupReadSetOpen and no World.lineWrites
#= no holds and no owns and no passes and no affects
#= one sig tessa__the_inode_number_is_primary_of_the_file extends DeclaresPrimaryOf {} { speaker = tessa and primaryScheme = sm_Inode and ofSort = sm_File }
#= one sig tessa__a_path_yields_an_inode extends DeclaresYields {} { speaker = tessa and fromShape = slash_path_shape and intoScheme = sm_Inode }
#= one sig tessa__an_inode_number_reaches_one_inode extends DeclaresUniqueReferent {} { speaker = tessa and referentShape = inode_shape }
#= one sig wanda__sudo_lends_nothing_else extends ClosesLends {} { speaker = wanda and closedWrapper = sudo }
#= one sig carl__srv_a_matches_golden extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_1 and atLine = this and markedReads = k_srv_a_at_line_1 and dependsOn = inode_17 }
#= axiomaticByContract and axiomaticByDifferentialTest and (let f = atLine.this | identity[f.topic] = k_ino_17_at_line_1 and worldOf[identity[f.topic]] = r0)

   sudo cmp -s ./golden.conf /srv/a/app.conf
#} sudo cmp dash_s golden_conf srv_a_app_conf
#= one sig carl__srv_a_matches_golden_as_root extends VerdictFact {} { speaker = carl and topic = k_srv_a_at_line_2 and atLine = this and markedReads = k_srv_a_at_line_2 and dependsOn = inode_17 }
#= let f = atLine.this, g = carl__srv_a_matches_golden | not inherits[v1] and v1.route = r1 and tabledCompare[f.topic, g.topic] = UNKNOWN and not sameTopic[f, g]
```

<!-- prose-translation -->
> This book's ceiling is six atoms of every kind the specification owns and integers of four bits.
> Tessa, Wanda, and Carl speak as in 3.4.2-a-book-one-file-across-sudo-under-the-sentinel.
> The book holds no world object besides those its lines name.
> The world holds one inode.
> No store holds it, nothing passes, no write affects another MReferent, and no line writes.
> The book holds no composite MKey and no role.
> No lookup's read set is open.
> The flag is not set.
> The MVantage entered through the wrapper inherits nothing, and its MRoute is another MRoute.
> Line 1, `cmp` from the host: every statement in force is true and the engine's axioms hold.
> The path's identity is the inode MKey its lookup emitted, whose chain ends at the host's MRoute.
> Line 2, `cmp` through `sudo`, against line 1: the entered MVantage does not inherit, and its MRoute is the second MRoute.
> `compare()` answers UNKNOWN, and the two facts are not about one MTopic.
