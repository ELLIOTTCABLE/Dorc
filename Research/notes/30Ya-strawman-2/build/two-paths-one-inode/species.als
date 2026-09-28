module species
open shared

one sig tessa, simon, carl, stdlib, foob extends Speaker {}

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

sig AliasesNothingElse extends MDecl { store: one MSort }
fact { all d: AliasesNothingElse & True, k, k2: Key | k.worldParent.scheme.primaryOf = d.store and k2.reaches = k.reaches implies k2.worldParent = k.worldParent }

sig MayWrite extends MDecl { verb: one Shword }
sig ChecksRead extends MDecl { verb: one Shword }
fun keysOf[S: set MDecl, l: Line]: set Key { { k: Key | some d: Operand & S | d.verb = l.cmd and k.w = l.argv[d.at] and k.scheme = d.under } }
fun writesOf[S: set MDecl, l: Line]: set Key { (some d: MayWrite & S | d.verb = l.cmd) and some keysOf[S, l] implies keysOf[S, l] else Key }
fun readsOf[S: set MDecl, l: Line]: set Key { (some d: ChecksRead & S | d.verb = l.cmd) and some keysOf[S, l] implies keysOf[S, l] else Key }
fact { all v: Verdict | v.speaker in (ChecksRead & True & verb.(v.of.cmd)).speaker }

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

pred allTrue[S: set MDecl] { S in True }

fun sortOwner[s: MSort]: set Speaker { ((PrimaryOf & True) & ofSort.s).speaker }
fun schemeOwner[s: MScheme]: set Speaker { ((Yields & True) & under.s).speaker + ((PrimaryOf & True) & ofScheme.s).speaker }
