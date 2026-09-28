-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module corpus
open claims

check oneDescriberPerVerb { all disj a, b: Vouches | a.verb != b.verb }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 17 Claim, exactly 0 Line
check describerSpeaksAlone {
   all d: Disturbs | some v: Vouches | v.verb = d.verb and v.speaker = d.speaker
   all d: Reads | some v: Vouches | v.verb = d.verb and v.speaker = d.speaker
   all d: Withholds | some v: Vouches | v.verb = d.verb and v.speaker = d.speaker
}
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 17 Claim, exactly 0 Line
check oneFootprintPerShape { all disj d, e: Disturbs | d.verb != e.verb or d.sub != e.sub }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 17 Claim, exactly 0 Line
check oneBackingPerShape { all disj d, e: Reads | d.verb != e.verb or d.sub != e.sub }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 17 Claim, exactly 0 Line
