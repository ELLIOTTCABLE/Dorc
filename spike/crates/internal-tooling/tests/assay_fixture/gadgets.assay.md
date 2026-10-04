> assay self-test fixture; it means nothing.

Gadgets opens widgets, and names its sigs and predicates.

```alloy
open widgets
sig Gizmo extends Wobble {}
```

A library open written after a paragraph, a claim atom under a sig the opened document declares, a
law, its premise twin, and a corpus check.

```alloy
open util/boolean
one sig alice__twiddle_gizmos extends Gizmo {} { speaker = alice  of = twiddle }

check everyGizmoSpokenIsHeard { all l: Line | Gizmo & l.speech in Heard } for 3 but 2 Int
run everyGizmoSpokenIsHeard_premise { some l: Line | some Gizmo & l.speech } for 3 but 2 Int

check theTwiddleGizmoIsHeard { alice__twiddle_gizmos in Heard }
```

```sh
# fiddles.sh
. ./alice__twiddle_gizmos.sh

   twiddle /tmp/knob
#} twiddle knob_path
#= wobbly[this]
```
