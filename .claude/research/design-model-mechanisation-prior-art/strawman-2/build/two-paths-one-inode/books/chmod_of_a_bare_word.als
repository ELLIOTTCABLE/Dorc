module books/chmod_of_a_bare_word
open claims

one sig line_1, line_2 extends Line {}
fact { line_1.cmd = chmod  line_1.argv = 0->g_minus_w + 1->w_shared  no line_1.before }
fact { line_2.cmd = chmod  line_2.argv = 0->g_plus_w + 1->w_shared   line_2.before = line_1 }
fact { line_1.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod  line_2.speech = line_1.speech }

run line_1 { line_1 in Ran }
   for 4 but exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 16 Claim
run line_2 { line_1 in Ran and line_2 in Guarded and no by[line_2] }
   for 4 but exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 16 Claim
