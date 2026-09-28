module books/siblings_in_one_filesystem
open two_paths_one_inode

one sig v_srv_a_shared, v_srv_a_other, v_inode_x, v_inode_y, v_fs_1, v_boot_1 extends MValue {}
one sig r_inode_x, r_inode_y, r_fs_1, r_boot_1 extends MReferent {}
one sig a_path, c_path, inode_x, inode_y, fs_1, boot_1 extends MKey {}
fact {
   a_path.scheme = Path  a_path.value = v_srv_a_shared  no a_path.worldParent
   c_path.scheme = Path  c_path.value = v_srv_a_other  no c_path.worldParent
   inode_x.scheme = Inode  inode_x.value = v_inode_x  inode_x.reaches = r_inode_x
   inode_y.scheme = Inode  inode_y.value = v_inode_y  inode_y.reaches = r_inode_y
   fs_1.scheme = DeviceNumber  fs_1.value = v_fs_1  fs_1.reaches = r_fs_1  no fs_1.worldParent or fs_1.worldParent = boot_1
   boot_1.scheme = BootId  boot_1.value = v_boot_1  boot_1.reaches = r_boot_1  no boot_1.worldParent
}

fact { a_path.reaches = inode_x.reaches and c_path.reaches = inode_y.reaches }
fact { inode_x.worldParent = fs_1 and inode_y.worldParent = fs_1 }
fact { fs_1.worldParent = boot_1 }

one sig tessa__a_path_resolves_to_inode_x extends Resolution {} { speaker = tessa  of = a_path  to = inode_x }
one sig tessa__c_path_resolves_to_inode_y extends Resolution {} { speaker = tessa  of = c_path  to = inode_y }
one sig tessa__inode_x_is_placed_in_fs_1 extends Placement {} { speaker = tessa  of = inode_x  within = fs_1 }
one sig tessa__inode_y_is_placed_in_fs_1 extends Placement {} { speaker = tessa  of = inode_y  within = fs_1 }
one sig simon__fs_1_is_placed_in_boot_1 extends Placement {} { speaker = simon  of = fs_1  within = boot_1 }

one sig site_3, site_4 extends Site {}
fact { site_3.verb = chmod  site_3.arg = a_path  no site_3.before  site_3 not in Converged }
fact { site_4.verb = chmod  site_4.arg = c_path  site_4.before = site_3  site_4 in Converged }
one sig q_4 extends Query {} { w = site_3  r = site_4 }

fact { Loaded = speaker.(tessa + simon + stdlib + carl) }

run site_3 { site_3 in Ran }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4 { site_4 in Elided and simon__an_ext4_filesystem_exposes_its_inodes_through_no_other_filesystem not in by[site_4] and not wrong[Loaded, q_4] }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query

run site_4_without_tessa__an_inode_is_the_primary_key_of_a_file { answer[Loaded - tessa__an_inode_is_the_primary_key_of_a_file, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_tessa__a_file_has_one_inode_in_its_filesystem { answer[Loaded - tessa__a_file_has_one_inode_in_its_filesystem, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_tessa__a_path_resolves_to_inode_x { answer[Loaded - tessa__a_path_resolves_to_inode_x, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_tessa__c_path_resolves_to_inode_y { answer[Loaded - tessa__c_path_resolves_to_inode_y, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_tessa__inode_x_is_placed_in_fs_1 { answer[Loaded - tessa__inode_x_is_placed_in_fs_1, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_tessa__inode_y_is_placed_in_fs_1 { answer[Loaded - tessa__inode_y_is_placed_in_fs_1, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_simon__fs_1_is_placed_in_boot_1 { answer[Loaded - simon__fs_1_is_placed_in_boot_1, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_stdlib__a_boot_id_is_a_root { answer[Loaded - stdlib__a_boot_id_is_a_root, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else { answer[Loaded - carl__chmod_changes_the_mode_of_the_file_at_its_path_and_nothing_else, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
run site_4_without_carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode { answer[Loaded - carl__chmod_is_converged_when_the_file_at_its_path_has_the_mode, q_4] not in Spares }
   for exactly 6 MKey, exactly 6 MValue, exactly 4 MReferent, exactly 2 Site, exactly 1 Query
