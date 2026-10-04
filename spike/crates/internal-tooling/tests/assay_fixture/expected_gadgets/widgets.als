module widgets
open assay
open shared
one sig alice, bob extends Speaker {}
sig Wobble extends Blurb { of: one Shword }
sig Jiggle extends Blurb { at: one Line }
sig Knob { tag: one Shword }
pred wobbly[l: Line] { some w: Wobble & l.speech | w.of = l.cmd }
