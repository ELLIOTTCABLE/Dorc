module books/two_paths_one_inode
open claims

-- world facts, verbatim from the fixture lines
fact { word.inode_x.scheme = Inode }
fact { word.a_path.reaches = word.inode_x.reaches and word.b_path.reaches = word.inode_x.reaches and word.inode_x.worldParent = word.fs_1 }
fact { word.fs_1.worldParent = word.boot_1 }

one sig line_3, line_4 extends Line {}
-- the claim declared on line 4, in force there and after
one sig carl__the_file_at_b_path_has_the_mode extends Verdict {} { of = line_4 }
fact { line_3.cmd = chmod  line_3.argv = 0->g_minus_w + 1->a_path  no line_3.before }
fact { line_4.cmd = chmod  line_4.argv = 0->g_plus_w + 1->b_path   line_4.before = line_3 }
fact { line_3.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod }
fact { line_4.speech = line_3.speech + carl__the_file_at_b_path_has_the_mode }

-- each line's outcome is checked with the outcomes above it as premises; the book's run proves the facts consistent
check line_3 { line_3 in Ran }
   for 8 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
check line_4 { line_3 in Ran implies line_4 in Guarded }
   for 8 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
run two_paths_one_inode { line_3 in Ran and line_4 in Guarded }
   for 8 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
