# Shared

> STRAWMAN, non-normative. A conductor's exercise of the assay shape (`notes/30Y`) against a made-up cut of Dorc's outcome algebra, written to find where Alloy and assay fight an author. Nothing in this directory is Dorc's design; every design decision here was invented to reach a compiling model, binds nothing, and is superseded by any ruled document on the same topic.

The tree-global module. assay opens it into every module it generates, after its own harness. It holds what every Dorc specification talks about and none defines: who speaks, what a claim is, when a claim is true, the order of lines, the answers to the sparing question and which of them spare, the measurement a line's verdict rests on, the admin's flag, and the four outcomes a line can have. Which converged lines elide is each specification's own law; this module names the outcomes and leaves their definition to the document that owns the sparing question.

```alloy
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
```

A line is above another when sh reaches it first; the relation is transitive and never reaches back to its own line. A claim whose truth nothing computes is true exactly where some line has it in force; a specification names the claims whose truth it computes against the world. A line is converged when a verdict claim about it is in force and true; the verdict is a measurement, so it is speech, with a speaker and the possibility of being false. A line that is not converged runs. A converged line that does not elide is guarded. An elided line with a line above it that may run has survived that line. The flag is typed for the whole book or not at all. The world a book's commands are checked in holds at most twelve of any kind of thing assay does not size itself, integers of four bits, and argument vectors of at most six words, because a sequence can be no longer than the largest integer the bitwidth admits; a document overrides that for one of its books with `bookScope_<book>`, and an outcome line overrides it for one command with its own `for`.
