module books/siblings_across_filesystems
open claims

fact { word.inode_x.scheme = Inode and word.inode_z.scheme = Inode }
fact { word.a_path.reaches = word.inode_x.reaches and word.d_path.reaches = word.inode_z.reaches }
fact { word.inode_x.worldParent = word.fs_1 and word.inode_z.worldParent = word.fs_2 }
fact { word.fs_1.worldParent = word.boot_1 and word.fs_2.worldParent = word.boot_1 }

one sig line_3, line_4 extends Line {}
one sig carl__the_file_at_d_path_has_the_mode extends Verdict {} { of = line_4 }
fact { line_3.cmd = chmod  line_3.argv = 0->g_minus_w + 1->a_path  no line_3.before }
fact { line_4.cmd = chmod  line_4.argv = 0->g_plus_w + 1->d_path   line_4.before = line_3 }
fact { line_3.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod }
fact { line_4.speech = line_3.speech + carl__the_file_at_d_path_has_the_mode }

check line_3 { line_3 in Ran }
   for 8 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
check line_4 { line_3 in Ran implies line_4 in Elided and simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem in by[line_4] }
   for 8 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
run siblings_across_filesystems { line_3 in Ran and line_4 in Elided and simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem in by[line_4] }
   for 8 but 4 Int, exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 18 Claim
