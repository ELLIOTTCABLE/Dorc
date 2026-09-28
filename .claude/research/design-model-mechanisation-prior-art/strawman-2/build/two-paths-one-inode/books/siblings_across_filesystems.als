module books/siblings_across_filesystems
open claims

fact { inode_x.scheme = Inode and inode_z.scheme = Inode }
fact { a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches }
fact { inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2 }
fact { fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1 }

one sig line_3, line_4 extends Line {}
fact { line_3.cmd = chmod  line_3.argv = 0->g_minus_w + 1->a_path  no line_3.before }
fact { line_4.cmd = chmod  line_4.argv = 0->g_plus_w + 1->d_path   line_4.before = line_3 }
fact { line_3.speech = tessa_fs + simon_fs + stdlib_boot + carl_chmod  line_4.speech = line_3.speech }

run line_3 { line_3 in Ran }
   for 4 but exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 16 Claim
run line_4 { line_3 in Ran and line_4 in Elided and simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem in by[line_4] }
   for 4 but exactly 20 Shword, exactly 2 Class, exactly 2 Line, exactly 16 Claim
