module book_corpus
open claims
one sig line_1 extends Line {}
fact { line_1.cmd = assay_colon no line_1.argv no line_1.above }
fact { line_1.speech = alice__frob_wobbles + bob__spin_wobbles + bob__twirl_wobbles_on_the_sprocket_heap }
check everySpokenWobbleIsHeard { Wobble & Line.speech in Heard } for 4 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 1 Line, exactly 3 Claim
check everyHeardBlurbIsSpoken { Heard in Line.speech } for 4 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 1 Line, exactly 3 Claim
check every_line { (Wobble & Line.speech in Heard) and (Heard in Line.speech) } for 4 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 1 Line, exactly 3 Claim
run book_corpus {} for 4 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 1 Line, exactly 3 Claim
