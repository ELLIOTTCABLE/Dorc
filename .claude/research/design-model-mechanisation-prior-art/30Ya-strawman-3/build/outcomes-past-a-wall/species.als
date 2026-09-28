-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module species
open shared

one sig stdlib, tessa, debsvc, foob, identity extends Speaker {}

sig MReferent {}
sig Key { w: one Shword, reaches: one MReferent }
fact { all disj a, b: Key | a.w != b.w }

sig Touched { line: one Line, wrote: set Key }
fact { all l: Line | lone line.l }
fun touches[l: Line]: set Key { some line.l implies (line.l).wrote else Key }

sig Vouches extends MDecl { verb: one Shword }
sig Disturbs extends MDecl { verb: one Shword, sub: lone Shword, at: set Int, cells: set Shword }
sig Reads extends MDecl { verb: one Shword, sub: lone Shword, at: set Int, cells: set Shword }
sig Withholds extends MDecl { verb: one Shword }

pred matches[v: Shword, s: set Shword, l: Line] { v = l.cmd and (no s or s = l.argv[0]) }
fun keysAt[l: Line, at: set Int, cells: set Shword]: set Key { { k: Key | k.w in l.argv[at] + cells } }
fact { all d: Disturbs, l: Line | matches[d.verb, d.sub, l] implies d.at in (l.argv).Shword }
fact { all d: Reads, l: Line | matches[d.verb, d.sub, l] implies d.at in (l.argv).Shword }
fact { all d: Disturbs, c: d.cells | some k: Key | k.w = c }
fact { all d: Reads, c: d.cells | some k: Key | k.w = c }

fun footprint[S: set MDecl, l: Line]: set Key {
   (some d: Disturbs & S | matches[d.verb, d.sub, l])
      implies { k: Key | all d: Disturbs & S | matches[d.verb, d.sub, l] implies k in keysAt[l, d.at, d.cells] }
      else Key
}
fun backing[S: set MDecl, l: Line]: set Key {
   (some d: Reads & S | matches[d.verb, d.sub, l])
      implies { k: Key | all d: Reads & S | matches[d.verb, d.sub, l] implies k in keysAt[l, d.at, d.cells] }
      else Key
}

fact { all d: Disturbs | d in True iff all l: Line | matches[d.verb, d.sub, l] implies touches[l] in keysAt[l, d.at, d.cells] }
fact { all v: Verdict | some d: Vouches & True | d.verb = v.of.cmd and d.speaker = v.speaker }

sig Separate extends MDecl { a, b: one Key }
fact { all s: Separate | s.a != s.b }
fact { Computed = Disturbs + Separate }
fact { all s: Separate | s in True iff s.a.reaches != s.b.reaches }
pred separated[S: set MDecl, x, y: Key] { some s: Separate & S | (s.a = x and s.b = y) or (s.a = y and s.b = x) }

fun answer[S: set MDecl, w, r: Line]: one Ans {
   (all x: footprint[S, w], y: backing[S, r] | separated[S, x, y]) implies DISJOINT else UNKNOWN
}

fun said[l: Line]: set MDecl { l.speech & MDecl }
sig Withheld in Line {}
fact { Withheld = { l: Line | some d: Withholds & l.speech | d.verb = l.cmd } }
fact { Elided = { l: Converged - Withheld | all w: l.above - Elided | some Typed and answer[said[l], w, l] in Spares } }

pred wrong[S: set MDecl, w, r: Line] { answer[S, w, r] = DISJOINT and some x: touches[w], y: backing[S, r] | x.reaches = y.reaches }
pred underExecuted[l: Line] { l in Elided and some w: l.above - Elided, x: touches[w], y: backing[said[l], l] | x.reaches = y.reaches }

fun support[S: set MDecl, w, r: Line]: set MDecl {
   { d: Disturbs & S | matches[d.verb, d.sub, w] }
   + { d: Reads & S | matches[d.verb, d.sub, r] }
   + { s: Separate & S | some x: footprint[S, w], y: backing[S, r] | (s.a = x and s.b = y) or (s.a = y and s.b = x) }
}
fun restsOn[S: set MDecl, w, r: Line]: set MDecl { { d: S | answer[S - d, w, r] != answer[S, w, r] } }
fun by[l: Line]: set MDecl { { d: said[l] | some w: l.above - Elided | d in support[said[l], w, l] } }

pred allTrue[S: set MDecl] { S in True }
pred oneVoicePerLine[S: set MDecl] {
   all l: Line | lone { d: Disturbs & S | matches[d.verb, d.sub, l] } and lone { d: Reads & S | matches[d.verb, d.sub, l] }
   all disj s, t: Separate & S | not ((s.a = t.a and s.b = t.b) or (s.a = t.b and s.b = t.a))
}
