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

sig AliasesNothingElse extends MDecl { store: one MSort }
fact { all d: AliasesNothingElse & True, k, k2: MKey | k.worldParent.scheme.primaryOf = d.store and k2.reaches = k.reaches implies k2.worldParent = k.worldParent }

one sig simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem extends AliasesNothingElse {} { speaker = simon  store = Filesystem }

sig Verb {}
sig Site { verb: one Verb, arg: one MKey, before: set Site }
sig Converged in Site {}

sig MayWrite extends MDecl { verb: one Verb }
sig ChecksRead extends MDecl { verb: one Verb }
fun writesOf[S: set MDecl, s: Site]: set MKey { (some d: MayWrite & S | d.verb = s.verb) implies s.arg else MKey }
fun readsOf[S: set MDecl, s: Site]: set MKey { (some d: ChecksRead & S | d.verb = s.verb) implies s.arg else MKey }

one sig carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else extends MayWrite {} { speaker = carl  verb = chmod }
one sig carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode extends ChecksRead {} { speaker = carl  verb = chmod }

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

fun sortOwner[s: MSort]: set Speaker { ((PrimaryOf & True) & ofSort.s).speaker }
fun schemeOwner[s: MScheme]: set Speaker { ((Yields & True) & from.s).speaker + ((PrimaryOf & True) & ofScheme.s).speaker }

check seatCanKnow {
   all d: AliasesNothingElse | d.speaker in sortOwner[d.store]
   all d: IdentifiedIn | d.speaker in schemeOwner[d.ofScheme]
   all d: GuaranteesUniqueName + GuaranteesUniqueReferent + Root | d.speaker in schemeOwner[d.on]
   all d: Resolution + Placement | d.speaker in schemeOwner[d.of.scheme]
} for 12

one sig foob__a_bundle_path_is_the_primary_key_of_a_certificate_bundle extends PrimaryOf {} { speaker = foob  ofScheme = BundlePath  ofSort = CertificateBundle }

one sig tessa__a_singly_linked_path_names_one_entry_per_component_and_declines_a_path_whose_inode_has_another_entry extends Yields {} { speaker = tessa  from = SinglyLinkedPath  to = Entry }
one sig tessa__a_singly_linked_path_names_one_file extends GuaranteesUniqueName {} { speaker = tessa  on = SinglyLinkedPath }
