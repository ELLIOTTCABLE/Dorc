# Shared

The tree-global module. assay opens it into every module it generates, after its own harness. It holds what every Dorc specification talks about and none defines: who speaks, what a claim is, when a claim is true, the four answers and their order, the measurement a line's verdict rests on, the sparing question, and the three outcomes.

```alloy
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
```

The world a book's commands are checked in holds at most twelve of any kind of thing assay does not size itself; a document overrides that for one of its books with `bookScope_<book>`, and an outcome line overrides it for one command with its own `for`. A claim whose truth nothing computes is true exactly where some line has it in force; a specification names the claims whose truth it computes against the world. A line is converged when a verdict claim about it is in force and true; the verdict is a measurement, so it is speech, with a speaker and the possibility of being false, and its truth is computed once a specification models the state the check reads. A converged line is elided only when every question about it, one per earlier line that ran, answers in `Spares`. What answers a question is each specification's own law over the speech in force at the converged line.
