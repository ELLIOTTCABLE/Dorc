module books/b7_3
open two_paths_one_inode

one sig v_srv_a_shared, v_var_lib_other, v_inode_x, v_inode_z, v_fs_1, v_fs_2, v_boot_1 extends MValue {}
one sig r_inode_x, r_inode_z, r_fs_1, r_fs_2, r_boot_1 extends MReferent {}
one sig a_path, d_path, inode_x, inode_z, fs_1, fs_2, boot_1 extends MKey {}
fact {
   a_path.scheme = Path  a_path.value = v_srv_a_shared  no a_path.parent  a_path.resolvesTo = inode_x
   d_path.scheme = Path  d_path.value = v_var_lib_other  no d_path.parent  d_path.resolvesTo = inode_z
   inode_x.scheme = Inode  inode_x.value = v_inode_x  inode_x.parent = fs_1  no inode_x.resolvesTo  inode_x.reaches = r_inode_x
   inode_z.scheme = Inode  inode_z.value = v_inode_z  inode_z.parent = fs_2  no inode_z.resolvesTo  inode_z.reaches = r_inode_z
   fs_1.scheme = DeviceNumber  fs_1.value = v_fs_1  fs_1.parent = boot_1  no fs_1.resolvesTo  fs_1.reaches = r_fs_1
   fs_2.scheme = DeviceNumber  fs_2.value = v_fs_2  fs_2.parent = boot_1  no fs_2.resolvesTo  fs_2.reaches = r_fs_2
   boot_1.scheme = BootId  boot_1.value = v_boot_1  no boot_1.parent  no boot_1.resolvesTo  boot_1.reaches = r_boot_1
}

one sig site_1, site_2 extends Site {}
fact { site_1.verb = chmod  site_1.arg = a_path  site_2.verb = chmod  site_2.arg = d_path }
one sig q_2 extends Query {} { w = site_1  r = site_2 }

fun loaded: set MDecl { speaker.(tessa + simon + stdlib + carl) }
fact { True = loaded }

fun by: set MDecl {
   tessa__an_inode_is_the_primary_key_of_a_file
   + simon__a_device_number_is_the_primary_key_of_a_filesystem
   + simon__a_filesystem_has_one_device_number_in_its_boot
   + simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem
   + stdlib__a_boot_id_is_a_root
   + carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else
   + carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode
}

run site_2_elide { answer[loaded, q_2] in Spares and not wrong[loaded, q_2] and restsOn[loaded, q_2] = by }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query

run site_2_without_tessa__an_inode_is_the_primary_key_of_a_file { answer[loaded - tessa__an_inode_is_the_primary_key_of_a_file, q_2] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_2_without_simon__a_device_number_is_the_primary_key_of_a_filesystem { answer[loaded - simon__a_device_number_is_the_primary_key_of_a_filesystem, q_2] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_2_without_simon__a_filesystem_has_one_device_number_in_its_boot { answer[loaded - simon__a_filesystem_has_one_device_number_in_its_boot, q_2] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_2_without_simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem { answer[loaded - simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem, q_2] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_2_without_stdlib__a_boot_id_is_a_root { answer[loaded - stdlib__a_boot_id_is_a_root, q_2] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_2_without_carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else { answer[loaded - carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else, q_2] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_2_without_carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode { answer[loaded - carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode, q_2] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
