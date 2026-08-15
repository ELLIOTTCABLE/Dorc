/-! # HoledLawIsVacuous

Fixture, not spec surface. A hole in a STATEMENT is worse than one in a proof: it typechecks,
so nothing downstream complains, and every law resting on it is vacuous.
-/

namespace Minispec

def HoledLawIsVacuous : Prop := ∀ n : Nat, n + 0 = n

theorem HoledLawIsVacuous_nonvacuous : (3 : Nat) + 0 = 3 := by
  sorry

end Minispec
