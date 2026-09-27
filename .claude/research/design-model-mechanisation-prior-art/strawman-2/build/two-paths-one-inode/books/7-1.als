module books/b7_1
open two_paths_one_inode

one sig v_srv_a_shared, v_srv_b_shared, v_inode_x, v_fs_1, v_boot_1 extends MValue {}
one sig r_inode_x, r_fs_1, r_boot_1 extends MReferent {}
one sig a_path, b_path, inode_x, fs_1, boot_1 extends MKey {}
fact {
   a_path.scheme = Path  a_path.value = v_srv_a_shared  no a_path.parent  a_path.resolvesTo = inode_x
   b_path.scheme = Path  b_path.value = v_srv_b_shared  no b_path.parent  b_path.resolvesTo = inode_x
   inode_x.scheme = Inode  inode_x.value = v_inode_x  inode_x.parent = fs_1  no inode_x.resolvesTo  inode_x.reaches = r_inode_x
   fs_1.scheme = DeviceNumber  fs_1.value = v_fs_1  fs_1.parent = boot_1  no fs_1.resolvesTo  fs_1.reaches = r_fs_1
   boot_1.scheme = BootId  boot_1.value = v_boot_1  no boot_1.parent  no boot_1.resolvesTo  boot_1.reaches = r_boot_1
}

one sig site_1, site_2 extends Site {}
fact { site_1.verb = chmod  site_1.arg = a_path  site_2.verb = chmod  site_2.arg = b_path }
one sig q_2 extends Query {} { w = site_1  r = site_2 }

fun loaded: set MDecl { speaker.(tessa + simon + stdlib + carl) }
fact { True = loaded }

run site_2_guard { answer[loaded, q_2] not in Spares and not wrong[loaded, q_2] and no restsOn[loaded, q_2] }
   for exactly 5 MKey, exactly 5 MValue, exactly 3 MReferent, exactly 2 Site, exactly 1 Query
