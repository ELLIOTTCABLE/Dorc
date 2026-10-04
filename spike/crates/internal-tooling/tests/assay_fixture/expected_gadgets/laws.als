module laws
open species
check everyGizmoSpokenIsHeard { all l: Line | Gizmo & l.speech in Heard } for 3 but 2 Int
run everyGizmoSpokenIsHeard_premise { some l: Line | some Gizmo & l.speech } for 3 but 2 Int
check wobblyNeedsAWobble { all l: Line | wobbly[l] implies some Wobble & l.speech } for 3 but 2 Int
