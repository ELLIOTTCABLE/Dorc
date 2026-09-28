-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module book_converged_host_everything_described
open claims

fact { no Typed }

one sig line_1, line_2, line_3, line_4, line_5, line_6 extends Line {}
one sig stdlib__the_package_index_is_fresh extends Verdict {} { speaker = stdlib  of = line_1 }
one sig stdlib__nginx_is_installed extends Verdict {} { speaker = stdlib  of = line_2 }
one sig tessa__the_nginx_config_matches extends Verdict {} { speaker = tessa  of = line_3 }
one sig foob__the_certs_are_current extends Verdict {} { speaker = foob  of = line_4 }
one sig debsvc__nginx_is_enabled_and_active extends Verdict {} { speaker = debsvc  of = line_5 }
one sig debsvc__the_rule_for_443_is_present extends Verdict {} { speaker = debsvc  of = line_6 }
fact { line_1.cmd = apt_get    line_1.argv = 0->update                                         no line_1.above }
fact { line_2.cmd = apt_get    line_2.argv = 0->install + 1->dash_y + 2->nginx                 line_2.above = line_1 }
fact { line_3.cmd = cp         line_3.argv = 0->local_nginx_conf + 1->nginx_conf               line_3.above = line_1 + line_2 }
fact { line_4.cmd = foobar     line_4.argv = 0->sync_certs + 1->certs_dir                      line_4.above = line_1 + line_2 + line_3 }
fact { line_5.cmd = systemctl  line_5.argv = 0->enable + 1->dash_dash_now + 2->nginx           line_5.above = line_1 + line_2 + line_3 + line_4 }
fact { line_6.cmd = ufw        line_6.argv = 0->allow + 1->port_443_tcp                        line_6.above = line_1 + line_2 + line_3 + line_4 + line_5 }
fact { line_1.speech = stdlib_apt + tessa_fs + debsvc_services + foob_certs + stdlib__the_package_index_is_fresh }
fact { line_2.speech = line_1.speech + stdlib__nginx_is_installed }
fact { line_3.speech = line_2.speech + tessa__the_nginx_config_matches }
fact { line_4.speech = line_3.speech + foob__the_certs_are_current }
fact { line_5.speech = line_4.speech + debsvc__nginx_is_enabled_and_active }
fact { line_6.speech = line_5.speech + debsvc__the_rule_for_443_is_present }

check line_1 { line_1 in Elided and no by[line_1] }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 6 Line, exactly 23 Claim
check line_2 { (line_1 in Elided and no by[line_1]) implies (line_2 in Elided and no by[line_2]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 6 Line, exactly 23 Claim
check line_3 { (line_1 in Elided and no by[line_1] and line_2 in Elided and no by[line_2]) implies (line_3 in Elided and no by[line_3]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 6 Line, exactly 23 Claim
check line_4 { (line_1 in Elided and no by[line_1] and line_2 in Elided and no by[line_2] and line_3 in Elided and no by[line_3]) implies (line_4 in Elided and no by[line_4]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 6 Line, exactly 23 Claim
check line_5 { (line_1 in Elided and no by[line_1] and line_2 in Elided and no by[line_2] and line_3 in Elided and no by[line_3] and line_4 in Elided and no by[line_4]) implies (line_5 in Elided and no by[line_5]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 6 Line, exactly 23 Claim
check line_6 { (line_1 in Elided and no by[line_1] and line_2 in Elided and no by[line_2] and line_3 in Elided and no by[line_3] and line_4 in Elided and no by[line_4] and line_5 in Elided and no by[line_5]) implies (line_6 in Elided and no by[line_6]) }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 6 Line, exactly 23 Claim
run converged_host_everything_described { Line in Elided and no by[line_1] and no by[line_2] and no by[line_3] and no by[line_4] and no by[line_5] and no by[line_6] }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 6 Line, exactly 23 Claim
