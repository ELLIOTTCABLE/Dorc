> assay self-test fixture; it means nothing.

# Widgets

```alloy
one sig alice, bob extends Speaker {}

sig Wobble extends Blurb { of: one Shword }
sig Jiggle extends Blurb { at: one Line }
sig Knob { tag: one Shword }

pred wobbly[l: Line] { some w: Wobble & l.speech | w.of = l.cmd }
```

A law, its premise twin, and a corpus check.

```alloy
check everyWobbleIsHeardWhenSpoken { all l: Line, w: Wobble & l.speech | w in Heard } for 3 but 2 Int
run everyWobbleIsHeardWhenSpoken_premise { some l: Line | some Wobble & l.speech }

check everySpokenWobbleIsHeard { Wobble & Line.speech in Heard }
check everyHeardBlurbIsSpoken { Heard in Line.speech }

run bookScope_twirls {} for 5 but 3 Int, 3 seq
```

```alloy
one sig alice__frob_wobbles extends Wobble {} { speaker = alice  of = frob }
one sig bob__spin_wobbles extends Wobble {} { speaker = bob  of = spin }
one sig bob__twirl_wobbles_on_the_sprocket_heap extends Wobble {} { speaker = bob  of = sprocket_heap }
```

```sh
# alice_kit.sh
. ./alice__frob_wobbles.sh
. ./bob__spin_wobbles.sh
```

```sh
# twirls.sh
. ./alice_kit.sh
. ./bob__twirl_wobbles_on_the_sprocket_heap.sh

   frob --mode fast /tmp/sprocket
#} frob dash_dash_mode fast sprocket_path
#= some k: Knob | k.tag = knob_word

   spin /tmp/sprocket >>/tmp/spin.log 2>&1
#} spin {gizmo} append_spin_log stderr_to_stdout
#= one sig bob__this_spin_jiggles extends Jiggle {} { speaker = bob  at = this }
#= some j: Jiggle & this.speech | j.at = this

   twirl /tmp/sprocket
#} twirl {gizmo}
#= this.argv[0] in gizmo.~class for 4 but 3 Int

   frob -c 'x y' /tmp/gadget
#} frob -c 'x y' {gizmo}
#= this.argv[2] in gizmo.~class
```
