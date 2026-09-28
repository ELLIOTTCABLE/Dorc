-- STRAWMAN, non-normative: a conductor's exercise of the assay shape (notes/30Y) against a
-- made-up cut of Dorc's outcome algebra. Nothing here is Dorc's design; design decisions in
-- this directory were invented to reach a compiling model and bind nothing.
module book_pristine_host
open claims

fact { no Typed }

one sig line_1, line_2, line_3, line_4, line_5, line_6, line_7 extends Line {}
fact { line_1.cmd = apt_get    line_1.argv = 0->update                                                                                    no line_1.above }
fact { line_2.cmd = apt_get    line_2.argv = 0->install + 1->dash_y + 2->nginx                                                            line_2.above = line_1 }
fact { line_3.cmd = cp         line_3.argv = 0->local_nginx_conf + 1->nginx_conf                                                          line_3.above = line_1 + line_2 }
fact { line_4.cmd = foobar     line_4.argv = 0->sync_certs + 1->certs_dir                                                                 line_4.above = line_1 + line_2 + line_3 }
fact { line_5.cmd = systemctl  line_5.argv = 0->enable + 1->dash_dash_now + 2->nginx                                                      line_5.above = line_1 + line_2 + line_3 + line_4 }
fact { line_6.cmd = hork       line_6.argv = 0->tune + 1->dash_dash_profile + 2->web + 3->append_hork_log + 4->stderr_to_stdout           line_6.above = line_1 + line_2 + line_3 + line_4 + line_5 }
fact { line_7.cmd = ufw        line_7.argv = 0->allow + 1->port_443_tcp                                                                   line_7.above = line_1 + line_2 + line_3 + line_4 + line_5 + line_6 }
fact { all l: Line | l.speech = stdlib_apt + tessa_fs + debsvc_services + foob_certs }

check line_1 { line_1 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
check line_2 { line_1 in Ran implies line_2 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
check line_3 { (line_1 in Ran and line_2 in Ran) implies line_3 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
check line_4 { (line_1 in Ran and line_2 in Ran and line_3 in Ran) implies line_4 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
check line_5 { (line_1 in Ran and line_2 in Ran and line_3 in Ran and line_4 in Ran) implies line_5 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
check line_6 { (line_1 in Ran and line_2 in Ran and line_3 in Ran and line_4 in Ran and line_5 in Ran) implies line_6 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
check line_7 { (line_1 in Ran and line_2 in Ran and line_3 in Ran and line_4 in Ran and line_5 in Ran and line_6 in Ran) implies line_7 in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
run pristine_host { Line in Ran }
   for 12 but 4 Int, 6 seq, exactly 28 Shword, exactly 7 Line, exactly 17 Claim
