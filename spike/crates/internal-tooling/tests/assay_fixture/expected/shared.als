module shared
open assay
sig Speaker {}
abstract sig Blurb extends Claim { speaker: one Speaker }
sig Heard in Blurb {}
fact { all b: Blurb | b in Heard iff b in Line.speech }
fact { all l: Line | l not in l.^above }
