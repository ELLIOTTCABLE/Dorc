module laws
open species
check everyWobbleIsHeardWhenSpoken { all l: Line, w: Wobble & l.speech | w in Heard } for 3 but 2 Int
run everyWobbleIsHeardWhenSpoken_premise { some l: Line | some Wobble & l.speech } for 3 but 2 Int
check wobblyNeedsAWobble { all l: Line | wobbly[l] implies some Wobble & l.speech } for 3 but 2 Int
