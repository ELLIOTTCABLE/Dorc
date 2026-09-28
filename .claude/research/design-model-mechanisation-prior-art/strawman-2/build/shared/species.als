module shared
open assay

sig Speaker {}
abstract sig MDecl extends Claim { speaker: one Speaker }
sig True in MDecl {}
sig Computed in MDecl {}
fact { all c: MDecl - Computed | c in True iff c in Line.speech }

abstract sig Ans { weaker: set Ans }
one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Ans {}
fact { weaker = UNKNOWN->SAME + UNKNOWN->DISJOINT + UNKNOWN->KNOWN_UNSPOKEN + KNOWN_UNSPOKEN->DISJOINT }
sig Safe in Ans {}
sig Spares in Ans {}
fact { Safe = UNKNOWN + KNOWN_UNSPOKEN  Spares = DISJOINT }

sig Verdict extends MDecl { of: one Line }
sig Converged in Line {}
fact { Converged = (Verdict & True).of }

sig Query { writer, reader: one Line, ans: one Ans }
fact { all q: Query | q.writer not in Converged and q.reader in Converged and q.writer in q.reader.before }
fact { all w, r: Line | w not in Converged and r in Converged and w in r.before implies some q: Query | q.writer = w and q.reader = r }

sig Ran, Elided, Guarded in Line {}
fact { Ran = Line - Converged }
fact { Elided = { l: Converged | all q: reader.l | q.ans in Spares } }
fact { Guarded = Converged - Elided }

run bookScope {} for 12 but 4 Int
