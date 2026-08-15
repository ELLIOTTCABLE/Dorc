/-! # StatedLawHasAProp

Fixture, not spec surface. The written shape: a `Prop`, an anti-vacuity probe, and a battery.
-/

namespace Minispec

def StatedLawHasAProp : Prop := ∀ n : Nat, n + 0 = n

theorem StatedLawHasAProp_nonvacuous : (3 : Nat) + 0 = 3 := rfl

example : (0 : Nat) + 0 = 0 := rfl
#guard (1 : Nat) + 0 = 1

end Minispec
