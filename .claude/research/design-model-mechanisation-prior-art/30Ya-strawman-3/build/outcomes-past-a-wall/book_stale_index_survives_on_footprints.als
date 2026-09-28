-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module book_stale_index_survives_on_footprints
open claims

fact { all disj a, b: Key | a.reaches != b.reaches }
fact { Typed = risk_faultless_skips }

one sig line_1, line_2, line_3, line_4, line_5, line_6, line_7 extends Line {}
one sig identity__the_package_index_is_not_the_package_database extends Separate {} { speaker = identity  a = w.pkg_index  b = w.dpkg_db }
one sig identity__the_package_index_is_not_the_local_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.local_nginx_conf }
one sig identity__the_package_index_is_not_the_nginx_config extends Separate {} { speaker = identity  a = w.pkg_index  b = w.nginx_conf }
one sig identity__the_package_index_is_not_the_certs extends Separate {} { speaker = identity  a = w.pkg_index  b = w.certs_dir }
one sig identity__the_package_index_is_not_the_unit_state extends Separate {} { speaker = identity  a = w.pkg_index  b = w.unit_state }
one sig world__update_wrote_the_package_index extends Touched {} { line = line_1  wrote = w.pkg_index }
one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = line_2 }
one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = line_3 }
one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = line_4 }
one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = line_5 }
one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = line_7 }
fact { line_1.cmd = apt_get    line_1.argv = 0->update                                                                                    no line_1.above }
fact { line_2.cmd = apt_get    line_2.argv = 0->install + 1->dash_y + 2->nginx                                                            line_2.above = line_1 }
fact { line_3.cmd = cp         line_3.argv = 0->local_nginx_conf + 1->nginx_conf                                                          line_3.above = line_1 + line_2 }
fact { line_4.cmd = foobar     line_4.argv = 0->sync_certs + 1->certs_dir                                                                 line_4.above = line_1 + line_2 + line_3 }
fact { line_5.cmd = systemctl  line_5.argv = 0->enable + 1->dash_dash_now + 2->nginx                                                      line_5.above = line_1 + line_2 + line_3 + line_4 }
fact { line_6.cmd = hork       line_6.argv = 0->tune + 1->dash_dash_profile + 2->web + 3->append_hork_log + 4->stderr_to_stdout           line_6.above = line_1 + line_2 + line_3 + line_4 + line_5 }
fact { line_7.cmd = ufw        line_7.argv = 0->allow + 1->port_443_tcp                                                                   line_7.above = line_1 + line_2 + line_3 + line_4 + line_5 + line_6 }
fact { line_1.speech = stdlib_apt + tessa_fs + debsvc_services + foob_certs
   + identity__the_package_index_is_not_the_package_database + identity__the_package_index_is_not_the_local_config
   + identity__the_package_index_is_not_the_nginx_config + identity__the_package_index_is_not_the_certs
   + identity__the_package_index_is_not_the_unit_state }
fact { line_2.speech = line_1.speech + stdlib__nginx_is_installed }
fact { line_3.speech = line_2.speech + tessa__the_nginx_config_matches }
fact { line_4.speech = line_3.speech + foob__the_certs_are_current }
fact { line_5.speech = line_4.speech + debsvc__nginx_is_enabled_and_active }
fact { line_6.speech = line_5.speech }
fact { line_7.speech = line_6.speech + debsvc__the_rule_for_443_is_present }

check line_1 { line_1 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
check line_2 { line_1 in Ran implies (line_2 in Survived and not underExecuted[line_2] and stdlib__apt_get_update_disturbs_only_the_package_index in by[line_2]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
check line_3 { (line_1 in Ran and line_2 in Survived and not underExecuted[line_2] and stdlib__apt_get_update_disturbs_only_the_package_index in by[line_2]) implies (line_3 in Survived and not underExecuted[line_3]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
check line_4 { (line_1 in Ran and line_2 in Survived and not underExecuted[line_2] and stdlib__apt_get_update_disturbs_only_the_package_index in by[line_2] and line_3 in Survived and not underExecuted[line_3]) implies (line_4 in Survived and not underExecuted[line_4]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
check line_5 { (line_1 in Ran and line_2 in Survived and not underExecuted[line_2] and stdlib__apt_get_update_disturbs_only_the_package_index in by[line_2] and line_3 in Survived and not underExecuted[line_3] and line_4 in Survived and not underExecuted[line_4]) implies (line_5 in Survived and not underExecuted[line_5] and no (speaker.foob & by[line_5])) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
check line_6 { (line_1 in Ran and line_2 in Survived and not underExecuted[line_2] and stdlib__apt_get_update_disturbs_only_the_package_index in by[line_2] and line_3 in Survived and not underExecuted[line_3] and line_4 in Survived and not underExecuted[line_4] and line_5 in Survived and not underExecuted[line_5] and no (speaker.foob & by[line_5])) implies line_6 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
check line_7 { (line_1 in Ran and line_2 in Survived and not underExecuted[line_2] and stdlib__apt_get_update_disturbs_only_the_package_index in by[line_2] and line_3 in Survived and not underExecuted[line_3] and line_4 in Survived and not underExecuted[line_4] and line_5 in Survived and not underExecuted[line_5] and no (speaker.foob & by[line_5]) and line_6 in Ran) implies line_7 in Guarded }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
run stale_index_survives_on_footprints { line_1 in Ran and line_2 in Survived and not underExecuted[line_2] and stdlib__apt_get_update_disturbs_only_the_package_index in by[line_2] and line_3 in Survived and not underExecuted[line_3] and line_4 in Survived and not underExecuted[line_4] and line_5 in Survived and not underExecuted[line_5] and no (speaker.foob & by[line_5]) and line_6 in Ran and line_7 in Guarded }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 27 Claim
