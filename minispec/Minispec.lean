/-!
# minispec

The law corpus's root module. Importing every unit is what makes `lake build` a whole-corpus
check rather than a per-file one, so a unit that stops elaborating cannot hide behind a
neighbour that still does.

A new unit is added here in the same commit that mints it.
-/

import Minispec.JoinIsCommutative
import Minispec.JoinIsIdempotent
import Minispec.LeqIsReflexive
