-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module assay

sig Shword { class: set Class }
sig Class {}
sig Claim {}
sig Line { above: set Line, speech: set Claim, cmd: one Shword, argv: seq Shword }
