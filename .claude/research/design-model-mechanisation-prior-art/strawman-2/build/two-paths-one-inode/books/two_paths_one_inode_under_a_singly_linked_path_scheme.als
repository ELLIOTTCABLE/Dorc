module books/two_paths_one_inode_under_a_singly_linked_path_scheme
open two_paths_one_inode

one sig v_srv_a_shared, v_srv_b_shared, v_inode_x, v_fs_1 extends MValue {}
one sig r_inode_x, r_fs_1 extends MReferent {}
one sig a_path, b_path, inode_x, fs_1 extends MKey {}
fact {
   a_path.scheme = SinglyLinkedPath  a_path.value = v_srv_a_shared  no a_path.worldParent
   b_path.scheme = SinglyLinkedPath  b_path.value = v_srv_b_shared  no b_path.worldParent
   inode_x.scheme = Inode  inode_x.value = v_inode_x  inode_x.reaches = r_inode_x
   fs_1.scheme = DeviceNumber  fs_1.value = v_fs_1  fs_1.reaches = r_fs_1  no fs_1.worldParent
}

fact { a_path.reaches = inode_x.reaches and b_path.reaches = inode_x.reaches and inode_x.worldParent = fs_1 }

one sig tessa__a_path_declines extends Resolution {} { speaker = tessa  of = a_path  no to }
one sig tessa__b_path_declines extends Resolution {} { speaker = tessa  of = b_path  no to }
one sig tessa__inode_x_is_placed_in_fs_1 extends Placement {} { speaker = tessa  of = inode_x  within = fs_1 }

one sig site_2, site_3 extends Site {}
fact { site_2.verb = chmod  site_2.arg = a_path  no site_2.before  site_2 not in Converged }
fact { site_3.verb = chmod  site_3.arg = b_path  site_3.before = site_2  site_3 in Converged }
one sig q_3 extends Query {} { w = site_2  r = site_3 }

fact { Loaded = (speaker.(tessa + simon + stdlib + carl) - tessa__a_path_names_one_entry_per_component) }

run site_2 { site_2 in Ran }
   for exactly 4 MKey, exactly 4 MValue, exactly 2 MReferent, exactly 2 Site, exactly 1 Query
run site_3 { site_3 in Guarded and not wrong[Loaded, q_3] and no by[site_3] }
   for exactly 4 MKey, exactly 4 MValue, exactly 2 MReferent, exactly 2 Site, exactly 1 Query
