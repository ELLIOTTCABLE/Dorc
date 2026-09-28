-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module shared
open assay

sig Speaker {}
abstract sig MDecl extends Claim { speaker: one Speaker }
sig True in MDecl {}
sig Computed in MDecl {}
fact { all c: MDecl - Computed | c in True iff c in Line.speech }

fact { all l: Line | l not in l.^above and l.above.above in l.above }

abstract sig Ans { weaker: set Ans }
one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Ans {}
fact { weaker = UNKNOWN->SAME + UNKNOWN->DISJOINT + UNKNOWN->KNOWN_UNSPOKEN + KNOWN_UNSPOKEN->DISJOINT }
sig Safe in Ans {}
sig Spares in Ans {}
fact { Safe = UNKNOWN + KNOWN_UNSPOKEN  Spares = DISJOINT }

sig Verdict extends MDecl { of: one Line }
sig Converged in Line {}
fact { Converged = (Verdict & True).of }

abstract sig Flag {}
one sig risk_faultless_skips extends Flag {}
sig Typed in Flag {}

sig Ran, Elided, Guarded in Line {}
sig Survived in Elided {}
fact { Ran = Line - Converged }
fact { Guarded = Converged - Elided }
fact { Survived = { l: Elided | some l.above - Elided } }

run bookScope {} for 12 but 4 Int, 6 seq
