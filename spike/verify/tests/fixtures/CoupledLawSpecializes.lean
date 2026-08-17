/-! # CoupledLawSpecializes

Fixture, not spec surface. `StatedLawHasAProp`'s shape PLUS the coupling theorem: the law's own
`Prop` applied at the battery's ground, which is what makes the battery an instance of the law
rather than a set of facts standing beside it.
-/

namespace Minispec

def CoupledLawSpecializes : Prop := ∀ n : Nat, n + 0 = n

theorem CoupledLawSpecializes_nonvacuous : (3 : Nat) + 0 = 3 := rfl

theorem CoupledLawSpecializes_specializes_at_nat (h : CoupledLawSpecializes) :
    (7 : Nat) + 0 = 7 := h 7

example : (0 : Nat) + 0 = 0 := rfl
#guard (1 : Nat) + 0 = 1

end Minispec
