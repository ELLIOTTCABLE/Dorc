module assay

sig Shword { class: set Class }
sig Class {}
sig Claim {}
sig Line { before: set Line, speech: set Claim, cmd: one Shword, argv: seq Shword }
