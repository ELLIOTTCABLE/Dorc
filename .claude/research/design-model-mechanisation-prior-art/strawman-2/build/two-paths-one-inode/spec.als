module two_paths_one_inode
open generic

one sig tessa, simon, carl, stdlib, foob extends Speaker {}
one sig Path, Entry, Inode, DeviceNumber, BootId, BundlePath, SinglyLinkedPath extends MScheme {}
one sig File, Filesystem, Boot, CertificateBundle extends MSort {}
one sig chmod extends Verb {}

one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Ans {}
fact { weaker = UNKNOWN->SAME + UNKNOWN->DISJOINT + UNKNOWN->KNOWN_UNSPOKEN + KNOWN_UNSPOKEN->DISJOINT }
fact { Safe = UNKNOWN + KNOWN_UNSPOKEN }
fact { Spares = DISJOINT }

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

sig AliasesNothingElse extends MDecl { store: one MSort }
fact { all d: AliasesNothingElse & True, k, k2: MKey | k.parent.scheme.primaryOf = d.store and k2.reaches = k.reaches implies k2.parent = k.parent }

one sig simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem extends AliasesNothingElse {} { speaker = simon  store = Filesystem }

sig Verb {}
sig Site { verb: one Verb, arg: one MKey }

sig MayWrite extends MDecl { verb: one Verb }
sig ChecksRead extends MDecl { verb: one Verb }
fun writesOf[S: set MDecl, s: Site]: set MKey { (some d: MayWrite & S | d.verb = s.verb) implies s.arg else MKey }
fun readsOf[S: set MDecl, s: Site]: set MKey { (some d: ChecksRead & S | d.verb = s.verb) implies s.arg else MKey }

one sig carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else extends MayWrite {} { speaker = carl  verb = chmod }
one sig carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode extends ChecksRead {} { speaker = carl  verb = chmod }

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

fun sortOwner[s: MSort]: set Speaker { ((PrimaryOf & True) & ofSort.s).speaker }
fun schemeOwner[s: MScheme]: set Speaker { ((Yields & True) & from.s).speaker + ((PrimaryOf & True) & ofScheme.s).speaker }

check seatCanKnow {
   all d: AliasesNothingElse | d.speaker in sortOwner[d.store]
   all d: IdentifiedIn | d.speaker in schemeOwner[d.ofScheme]
   all d: GuaranteesUniqueName + GuaranteesUniqueReferent + Root | d.speaker in schemeOwner[d.on]
} for 12

one sig foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle extends PrimaryOf {} { speaker = foob  ofScheme = BundlePath  ofSort = CertificateBundle }

one sig tessa__a_singly_linked_path_names_one_entry_per_component_and_declines_a_path_whose_inode_has_another_entry extends Yields {} { speaker = tessa  from = SinglyLinkedPath  to = Entry }
one sig tessa__a_singly_linked_path_names_one_file extends GuaranteesUniqueName {} { speaker = tessa  on = SinglyLinkedPath }
