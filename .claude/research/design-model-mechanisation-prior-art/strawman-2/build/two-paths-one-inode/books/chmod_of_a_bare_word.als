module books/chmod_of_a_bare_word
open claims

one sig line_1, line_2 extends Line {}
one sig carl__the_file_at_shared_has_the_mode extends Verdict {} { of = line_2 }
fact { line_1.cmd = chmod  line_1.argv = 0->g_minus_w + 1->w_shared  no line_1.above }
fact { line_2.cmd = chmod  line_2.argv = 0->g_plus_w + 1->w_shared   line_2.above = line_1 }
fact { line_1.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod }
fact { line_2.speech = line_1.speech + carl__the_file_at_shared_has_the_mode }

check line_1 { line_1 in Ran }
   for 12 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
check line_2 { line_1 in Ran implies line_2 in Guarded and no by[line_2] }
   for 12 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
run chmod_of_a_bare_word { line_1 in Ran and line_2 in Guarded and no by[line_2] }
   for 12 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
