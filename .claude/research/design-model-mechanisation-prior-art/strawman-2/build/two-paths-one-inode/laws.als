module laws
open species

check neverWrongWhenAllTrue { all S: set MDecl, q: Query | allTrue[S] implies not wrong[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
run neverWrongWhenAllTrue_premise { some S: set MDecl, q: Query | allTrue[S] and some writesOf[S, q.w] } for 6 but 8 Shword, 3 Line, 1 Query

check monotoneInSpeech { all S, S2: set MDecl, q: Query | S in S2 and allTrue[S2] implies answer[S, q] in answer[S2, q].*weaker } for 6 but 8 Shword, 3 Line, 1 Query
run monotoneInSpeech_premise { some S, S2: set MDecl, q: Query | S in S2 and S != S2 and allTrue[S2] } for 6 but 8 Shword, 3 Line, 1 Query

check strangerSafe { all S: set MDecl, d: MDecl, q: Query | allTrue[S + d] and d.speaker not in S.speaker implies answer[S, q] in answer[S + d, q].*weaker } for 6 but 8 Shword, 3 Line, 1 Query
run strangerSafe_premise { some S: set MDecl, d: MDecl, q: Query | allTrue[S + d] and d.speaker not in S.speaker } for 6 but 8 Shword, 3 Line, 1 Query

check attributionHonest { all S: set MDecl, q: Query | wrong[S, q] implies some d: restsOn[S, q] | d not in True } for 6 but 8 Shword, 3 Line, 1 Query
run attributionHonest_premise { some S: set MDecl, q: Query | wrong[S, q] } for 6 but 8 Shword, 3 Line, 1 Query

check attributionSufficient { all S: set MDecl, q: Query | answer[restsOn[S, q], q] = answer[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
run attributionSufficient_premise { some S: set MDecl, q: Query | some restsOn[S, q] } for 6 but 8 Shword, 3 Line, 1 Query

check attributionMinimal { all S: set MDecl, q: Query, d: restsOn[S, q] | answer[S - d, q] != answer[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
run attributionMinimal_premise { some S: set MDecl, q: Query | some restsOn[S, q] } for 6 but 8 Shword, 3 Line, 1 Query
