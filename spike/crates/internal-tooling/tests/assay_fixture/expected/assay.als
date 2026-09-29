module assay
sig Shword { class: set Class }
sig Class {}
abstract sig Claim {}
sig Line { above: set Line, speech: set Claim, cmd: one Shword, argv: seq Shword }
