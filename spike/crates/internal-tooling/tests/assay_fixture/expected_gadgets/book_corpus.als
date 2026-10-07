module book_corpus
open claims
one sig line_1 extends Line {}
fact { line_1.cmd = assay_colon and no line_1.argv and no line_1.above }
fact { line_1.speech = alice__twiddle_gizmos }
check theTwiddleGizmoIsHeard { alice__twiddle_gizmos in Heard } for 4 but 3 Int, 3 seq, exactly 3 Shword, exactly 0 Class, exactly 1 Line, exactly 1 Claim
run book_corpus {} for 4 but 3 Int, 3 seq, exactly 3 Shword, exactly 0 Class, exactly 1 Line, exactly 1 Claim
