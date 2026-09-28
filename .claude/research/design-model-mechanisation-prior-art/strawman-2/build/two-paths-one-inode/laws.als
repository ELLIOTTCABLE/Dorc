module laws
open two_paths_one_inode

pred allTrue[S: set MDecl] { S in True }

check neverWrongWhenAllTrue {
   all S: set MDecl, q: Query | allTrue[S] implies not wrong[S, q]
} for 6 but 8 MKey, 3 Site, 1 Query

check monotoneInSpeech {
   all S, S2: set MDecl, q: Query | S in S2 and allTrue[S2] implies answer[S, q] in answer[S2, q].*weaker
} for 6 but 8 MKey, 3 Site, 1 Query

check strangerSafe {
   all S: set MDecl, d: MDecl, q: Query | allTrue[S + d] and d.speaker not in S.speaker implies answer[S, q] in answer[S + d, q].*weaker
} for 6 but 8 MKey, 3 Site, 1 Query

check attributionHonest {
   all S: set MDecl, q: Query | wrong[S, q] implies some d: restsOn[S, q] | d not in True
} for 6 but 8 MKey, 3 Site, 1 Query

check attributionSufficient {
   all S: set MDecl, q: Query | answer[restsOn[S, q], q] = answer[S, q]
} for 6 but 8 MKey, 3 Site, 1 Query

check attributionMinimal {
   all S: set MDecl, q: Query, d: restsOn[S, q] | answer[S - d, q] != answer[S, q]
} for 6 but 8 MKey, 3 Site, 1 Query
