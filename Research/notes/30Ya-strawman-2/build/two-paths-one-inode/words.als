module words
open shared

-- one atom per distinct literal on any map line in the document, per braced class, and per name a #= introduces
one sig stat, dash_c, fmt_i_d, fmt_i_d_h, cat, proc_boot_id, chmod, g_minus_w, g_plus_w,
   a_path, b_path, c_path, d_path, w_shared,
   inode_x, inode_y, inode_z, fs_1, fs_2, boot_1 extends Shword {}
one sig slash_path, bare_word extends Class {}

-- the class memberships the map lines state, and no others
fact { class = a_path->slash_path + b_path->slash_path + c_path->slash_path + d_path->slash_path + w_shared->bare_word }
