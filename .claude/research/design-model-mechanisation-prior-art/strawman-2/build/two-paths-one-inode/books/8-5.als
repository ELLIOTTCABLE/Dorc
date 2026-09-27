module books/b8_5
open two_paths_one_inode

one sig v_srv_a_shared, v_srv_b_shared extends MValue {}
one sig r_a_path, r_b_path extends MReferent {}
one sig a_path, b_path extends MKey {}
fact {
   a_path.scheme = SinglyLinkedPath  a_path.value = v_srv_a_shared  no a_path.parent  no a_path.resolvesTo  a_path.reaches = r_a_path
   b_path.scheme = SinglyLinkedPath  b_path.value = v_srv_b_shared  no b_path.parent  no b_path.resolvesTo  b_path.reaches = r_b_path
}

one sig site_1, site_2 extends Site {}
fact { site_1.verb = chmod  site_1.arg = a_path  site_2.verb = chmod  site_2.arg = b_path }
one sig q_2 extends Query {} { w = site_1  r = site_2 }

fun loaded: set MDecl {
   speaker.(tessa + simon + stdlib + carl) - tessa__a_path_names_one_entry_per_component
}
fact { True = loaded }

run site_2_guard { answer[loaded, q_2] not in Spares and not wrong[loaded, q_2] and no restsOn[loaded, q_2] }
   for exactly 2 MKey, exactly 2 MValue, exactly 2 MReferent, exactly 2 Site, exactly 1 Query
