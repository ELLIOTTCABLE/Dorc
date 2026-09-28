-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module laws
open species

check neverWrongWhenAllTrue { all S: set MDecl, w, r: Line | allTrue[S] implies not wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run neverWrongWhenAllTrue_premise { some S: set MDecl, w, r: Line | allTrue[S] and answer[S, w, r] = DISJOINT and some touches[w] and some backing[S, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check neverUnderExecutedWhenAllTrue { allTrue[Line.speech & MDecl] implies no l: Elided | underExecuted[l] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run neverUnderExecutedWhenAllTrue_premise { allTrue[Line.speech & MDecl] and some Survived } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check monotoneInSpeech { all S, S2: set MDecl, w, r: Line | S in S2 and allTrue[S2] implies answer[S, w, r] in answer[S2, w, r].*weaker } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run monotoneInSpeech_premise { some S, S2: set MDecl, w, r: Line | S in S2 and S != S2 and allTrue[S2] and answer[S2, w, r] = DISJOINT and answer[S, w, r] = UNKNOWN } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check strangerSafe { all S: set MDecl, d: MDecl, w, r: Line | allTrue[S + d] and d.speaker not in S.speaker implies answer[S, w, r] in answer[S + d, w, r].*weaker } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run strangerSafe_premise { some S: set MDecl, d: MDecl, w, r: Line | some S and allTrue[S + d] and d.speaker not in S.speaker } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check attributionHonest { all S: set MDecl, w, r: Line | wrong[S, w, r] implies some d: support[S, w, r] | d not in True } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run attributionHonest_premise { some S: set MDecl, w, r: Line | wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check attributionSufficient { all S: set MDecl, w, r: Line | answer[support[S, w, r], w, r] = answer[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run attributionSufficient_premise { some S: set MDecl, w, r: Line | some support[S, w, r] and answer[S, w, r] = DISJOINT } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check attributionByRemovalHonest { all S: set MDecl, w, r: Line | wrong[S, w, r] implies some d: restsOn[S, w, r] | d not in True } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run attributionByRemovalHonest_premise { some S: set MDecl, w, r: Line | wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check attributionByRemovalHonestWithOneVoice { all S: set MDecl, w, r: Line | oneVoicePerLine[S] and wrong[S, w, r] implies some d: restsOn[S, w, r] | d not in True } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run attributionByRemovalHonestWithOneVoice_premise { some S: set MDecl, w, r: Line | oneVoicePerLine[S] and wrong[S, w, r] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check flagGatesSurvival { no Typed implies no Survived } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run flagGatesSurvival_premise { some Typed and some Survived } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim

check survivalRestsOnFootprints { all l: Survived, w: l.above - Elided | some backing[said[l], l] implies some d: Disturbs & l.speech | matches[d.verb, d.sub, w] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
run survivalRestsOnFootprints_premise { some l: Survived | some backing[said[l], l] } for 5 but 4 Int, 4 seq, 3 Line, 6 Shword, 8 Claim
