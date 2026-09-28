module corpus
open claims

check seatCanKnow {
   all d: AliasesNothingElse | d.speaker in sortOwner[d.store]
   all d: IdentifiedIn | d.speaker in schemeOwner[d.ofScheme]
   all d: GuaranteesUniqueName + Root | d.speaker in schemeOwner[d.on]
   all d: Resolution + Placement | d.speaker in schemeOwner[d.of.scheme]
} for 12 but 4 Int, 7 seq, exactly 20 Shword, exactly 2 Class, exactly 17 Claim, exactly 0 Line
