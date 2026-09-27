module generic

sig Speaker {}
abstract sig MDecl { speaker: one Speaker }
sig True in MDecl {}
abstract sig Ans { weaker: set Ans }
sig Safe in Ans {}
sig Spares in Ans {}
