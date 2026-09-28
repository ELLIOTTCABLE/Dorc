module shared
open assay

sig Speaker {}
abstract sig MDecl extends Claim { speaker: one Speaker }
sig True in MDecl {}

abstract sig Ans { weaker: set Ans }
one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Ans {}
fact { weaker = UNKNOWN->SAME + UNKNOWN->DISJOINT + UNKNOWN->KNOWN_UNSPOKEN + KNOWN_UNSPOKEN->DISJOINT }
sig Safe in Ans {}
sig Spares in Ans {}
fact { Safe = UNKNOWN + KNOWN_UNSPOKEN  Spares = DISJOINT }

sig Converged in Line {}
sig Query { w, r: one Line, ans: one Ans }
fact { all q: Query | q.w not in Converged and q.r in Converged and q.w in q.r.before }
fact { all w, r: Line | w not in Converged and r in Converged and w in r.before implies some q: Query | q.w = w and q.r = r }

sig Ran, Elided, Guarded in Line {}
fact { Ran = Line - Converged }
fact { Elided = { l: Converged | all q: r.l | q.ans in Spares } }
fact { Guarded = Converged - Elided }
