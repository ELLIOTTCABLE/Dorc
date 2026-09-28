-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module book_guarded_writer_casts_a_wall
open claims

fact { all disj a, b: Key | a.reaches != b.reaches }
fact { some w.local_nginx_conf }
fact { Typed = risk_faultless_skips }

one sig line_1, line_2, line_3, line_4 extends Line {}
one sig identity__the_package_index_is_not_the_nginx_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.nginx_conf }
one sig identity__the_package_index_is_not_the_backup extends Separate {} { speaker = identity  a = w.pkg_index  b = w.conf_backup }
one sig identity__the_package_index_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.pkg_index  b = w.unit_state }
one sig identity__the_nginx_config_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.nginx_conf  b = w.unit_state }
one sig identity__the_backup_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.conf_backup  b = w.unit_state }
one sig world__update_wrote_the_package_index extends Touched {} { line = line_1  wrote = w.pkg_index }
one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = line_2 }
one sig world__the_copy_wrote_the_nginx_config extends Touched {} { line = line_2  wrote = w.nginx_conf }
one sig tessa__the_backup_matches extends Verdict {} { speaker = tessa  of = line_3 }
one sig world__the_backup_wrote_the_backup extends Touched {} { line = line_3  wrote = w.conf_backup }
one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = line_4 }
fact { line_1.cmd = apt_get    line_1.argv = 0->update                                         no line_1.above }
fact { line_2.cmd = cp         line_2.argv = 0->local_nginx_conf + 1->nginx_conf               line_2.above = line_1 }
fact { line_3.cmd = cp         line_3.argv = 0->nginx_conf + 1->conf_backup                    line_3.above = line_1 + line_2 }
fact { line_4.cmd = systemctl  line_4.argv = 0->enable + 1->dash_dash_now + 2->nginx           line_4.above = line_1 + line_2 + line_3 }
fact { line_1.speech = stdlib_apt + tessa_fs + debsvc_services
   + identity__the_package_index_is_not_the_nginx_config + identity__the_package_index_is_not_the_backup
   + identity__the_package_index_is_not_the_unit_state + identity__the_nginx_config_is_not_the_unit_state
   + identity__the_backup_is_not_the_unit_state }
fact { line_2.speech = line_1.speech + tessa__the_nginx_config_matches }
fact { line_3.speech = line_2.speech + tessa__the_backup_matches }
fact { line_4.speech = line_3.speech + debsvc__nginx_is_enabled_and_active }

check line_1 { line_1 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 4 Line, exactly 25 Claim
check line_2 { line_1 in Ran implies line_2 in Guarded }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 4 Line, exactly 25 Claim
check line_3 { (line_1 in Ran and line_2 in Guarded) implies line_3 in Guarded }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 4 Line, exactly 25 Claim
check line_4 { (line_1 in Ran and line_2 in Guarded and line_3 in Guarded) implies (line_4 in Survived and not underExecuted[line_4] and tessa__cp_disturbs_only_its_destination in by[line_4]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 4 Line, exactly 25 Claim
run guarded_writer_casts_a_wall { line_1 in Ran and line_2 in Guarded and line_3 in Guarded and line_4 in Survived and not underExecuted[line_4] and tessa__cp_disturbs_only_its_destination in by[line_4] }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 4 Line, exactly 25 Claim
