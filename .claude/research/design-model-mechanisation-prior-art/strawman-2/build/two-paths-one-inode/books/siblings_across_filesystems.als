module books/siblings_across_filesystems
open two_paths_one_inode

one sig v_srv_a_shared, v_var_lib_other, v_inode_x, v_inode_z, v_fs_1, v_fs_2, v_boot_1 extends MValue {}
one sig r_inode_x, r_inode_z, r_fs_1, r_fs_2, r_boot_1 extends MReferent {}
one sig a_path, d_path, inode_x, inode_z, fs_1, fs_2, boot_1 extends MKey {}
fact {
   a_path.scheme = Path  a_path.value = v_srv_a_shared  no a_path.worldParent
   d_path.scheme = Path  d_path.value = v_var_lib_other  no d_path.worldParent
   inode_x.scheme = Inode  inode_x.value = v_inode_x  inode_x.reaches = r_inode_x
   inode_z.scheme = Inode  inode_z.value = v_inode_z  inode_z.reaches = r_inode_z
   fs_1.scheme = DeviceNumber  fs_1.value = v_fs_1  fs_1.reaches = r_fs_1
   fs_2.scheme = DeviceNumber  fs_2.value = v_fs_2  fs_2.reaches = r_fs_2
   boot_1.scheme = BootId  boot_1.value = v_boot_1  boot_1.reaches = r_boot_1  no boot_1.worldParent
}

fact { a_path.reaches = inode_x.reaches and d_path.reaches = inode_z.reaches }
fact { inode_x.worldParent = fs_1 and inode_z.worldParent = fs_2 }
fact { fs_1.worldParent = boot_1 and fs_2.worldParent = boot_1 }

one sig tessa__a_path_resolves_to_inode_x extends Resolution {} { speaker = tessa  of = a_path  to = inode_x }
one sig tessa__d_path_resolves_to_inode_z extends Resolution {} { speaker = tessa  of = d_path  to = inode_z }
one sig tessa__inode_x_is_placed_in_fs_1 extends Placement {} { speaker = tessa  of = inode_x  within = fs_1 }
one sig tessa__inode_z_is_placed_in_fs_2 extends Placement {} { speaker = tessa  of = inode_z  within = fs_2 }
one sig simon__fs_1_is_placed_in_boot_1 extends Placement {} { speaker = simon  of = fs_1  within = boot_1 }
one sig simon__fs_2_is_placed_in_boot_1 extends Placement {} { speaker = simon  of = fs_2  within = boot_1 }

one sig site_3, site_4 extends Site {}
fact { site_3.verb = chmod  site_3.arg = a_path  no site_3.before  site_3 not in Converged }
fact { site_4.verb = chmod  site_4.arg = d_path  site_4.before = site_3  site_4 in Converged }
one sig q_4 extends Query {} { w = site_3  r = site_4 }

fact { Loaded = speaker.(tessa + simon + stdlib + carl) }

run site_3 { site_3 in Ran }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_4 { site_4 in Elided and simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem in by[site_4] and not wrong[Loaded, q_4] }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query

run site_4_without_simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem { answer[Loaded - simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem, q_4] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_simon__a_filesystem_has_one_device_number_in_its_boot { answer[Loaded - simon__a_filesystem_has_one_device_number_in_its_boot, q_4] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_simon__a_device_number_is_the_primary_key_of_a_filesystem { answer[Loaded - simon__a_device_number_is_the_primary_key_of_a_filesystem, q_4] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_stdlib__a_boot_id_is_a_root { answer[Loaded - stdlib__a_boot_id_is_a_root, q_4] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_simon__fs_1_is_placed_in_boot_1 { answer[Loaded - simon__fs_1_is_placed_in_boot_1, q_4] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_tessa__a_path_resolves_to_inode_x { answer[Loaded - tessa__a_path_resolves_to_inode_x, q_4] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else { answer[Loaded - carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else, q_4] not in Spares }
   for exactly 7 MKey, exactly 7 MValue, exactly 5 MReferent, exactly 2 Site, exactly 1 Query
