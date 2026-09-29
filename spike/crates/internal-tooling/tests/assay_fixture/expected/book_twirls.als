module book_twirls
open claims
fact { some k: Knob | k.tag = knob_word }
one sig line_2, line_3, line_4 extends Line {}
one sig bob__this_spin_jiggles extends Jiggle {} { speaker = bob at = line_2 }
fact { line_2.cmd = spin line_2.argv = 0->sprocket_path + 1->append_spin_log + 2->stderr_to_stdout no line_2.above }
fact { line_3.cmd = twirl line_3.argv = 0->sprocket_path line_3.above = line_2 }
fact { line_4.cmd = frob line_4.argv = 0->w__dash_c + 1->w__sq_x_sp_y_sq_ + 2->w__slash_tmp_slash_gadget line_4.above = line_2 + line_3 }
fact { line_2.speech = alice_kit + bob__twirl_wobbles_on_the_sprocket_heap + bob__this_spin_jiggles }
fact { line_3.speech = line_2.speech }
fact { line_4.speech = line_3.speech }
check line_2 { (some j: Jiggle & line_2.speech | j.at = line_2) } for 5 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 3 Line, exactly 4 Claim
check line_3 { (some j: Jiggle & line_2.speech | j.at = line_2) implies (line_3.argv[0] in gizmo.~class) } for 4 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 3 Line, exactly 4 Claim
check line_4 { ((some j: Jiggle & line_2.speech | j.at = line_2) and (line_3.argv[0] in gizmo.~class)) implies (line_4.argv[2] in gizmo.~class) } for 5 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 3 Line, exactly 4 Claim
check every_line { { (some j: Jiggle & line_2.speech | j.at = line_2) } and { ((some j: Jiggle & line_2.speech | j.at = line_2) and (line_3.argv[0] in gizmo.~class)) implies (line_4.argv[2] in gizmo.~class) } } for 5 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 3 Line, exactly 4 Claim
run twirls { (some j: Jiggle & line_2.speech | j.at = line_2) and (line_3.argv[0] in gizmo.~class) and (line_4.argv[2] in gizmo.~class) } for 5 but 3 Int, 3 seq, exactly 14 Shword, exactly 1 Class, exactly 3 Line, exactly 4 Claim
