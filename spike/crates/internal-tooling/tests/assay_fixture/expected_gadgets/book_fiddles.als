module book_fiddles
open claims
one sig line_1 extends Line {}
fact { line_1.cmd = twiddle line_1.argv = 0->knob_path no line_1.above }
fact { line_1.speech = alice__twiddle_gizmos }
check line_1 { (wobbly[line_1]) } for 4 but 3 Int, 3 seq, exactly 3 Shword, exactly 0 Class, exactly 1 Line, exactly 1 Claim
run fiddles { (wobbly[line_1]) } for 4 but 3 Int, 3 seq, exactly 3 Shword, exactly 0 Class, exactly 1 Line, exactly 1 Claim
