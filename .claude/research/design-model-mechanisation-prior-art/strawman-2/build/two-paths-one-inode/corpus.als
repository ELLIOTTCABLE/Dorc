module corpus
open claims

check seatCanKnow {
   all d: AliasesNothingElse | d.speaker in sortOwner[d.store]
   all d: IdentifiedIn | d.speaker in schemeOwner[d.ofScheme]
   all d: GuaranteesUniqueName + Root | d.speaker in schemeOwner[d.on]
   all d: Resolution + Placement | d.speaker in schemeOwner[d.of.scheme]
} for 4 but exactly 20 Shword, exactly 2 Class, exactly 16 Claim, exactly 0 Line
