import Generated.Funs
import Minispec.Vocabulary.TrustedBase

/-!
# JoinIsCommutative

**The law, in English (authoritative):** joining two flat-lattice values gives the same
answer in either order. For every type `T` with lawful clone and equality dictionaries,
and any two values `a b : Flat T`, `a ⊔ b = b ⊔ a`. The statement keeps the
translation's monadic shape (nothing projects out of `Result`), but under the
lawfulness hypotheses the dictionaries cannot fail, so the error channel is
unreachable here — this law asserts nothing about failure behaviour.

`Flat T` (⊥ · a single element · ⊤) is the translation of the engine's simplest lattice
combinator, and `join` is the solver's merge for it. Commutativity is stated over the
DERIVED definitions — the translated bodies of the shipping Rust — so the claim is about
the code that runs, not a transcription of it.

The hypotheses are the named trusted-base entries (`Minispec/Vocabulary/TrustedBase.lean`): the
translation's generic dictionaries are lawless records, so the statement says what it
assumes of them. The instance battery below evaluates the law on concrete lawful
dictionaries at every boundary shape (⊥ · element · ⊤), both orders.
-/

namespace Minispec

open generated
open Aeneas.Std Result

/-- Joining flat-lattice values commutes, for every lawful dictionary pair. -/
def JoinIsCommutative : Prop :=
  ∀ (T : Type) [DecidableEq T]
    (cl : core.clone.Clone T) (eqi : core.cmp.Eq T),
    LawfulClone cl → LawfulEq eqi →
    ∀ a b : lattice.Flat T,
      lattice.Flat.Insts.GeneratedLatticeLattice.join cl eqi a b
        = lattice.Flat.Insts.GeneratedLatticeLattice.join cl eqi b a

/-- Anti-vacuity: the law does non-trivial work — two DIFFERENT elements join to ⊤ in
    both orders (not merely the degenerate self-join). -/
theorem JoinIsCommutative_nonvacuous :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        (lattice.Flat.Elem 1#u32) (lattice.Flat.Elem 2#u32)
      = ok lattice.Flat.Top
    ∧ lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        (lattice.Flat.Elem 2#u32) (lattice.Flat.Elem 1#u32)
      = ok lattice.Flat.Top := by
  constructor <;> rfl

/-- Coupling: the law's own `Prop`, applied at the battery ground. An edit that
    decouples the statement from the seat it names breaks this theorem, so the
    battery is mechanically about the law, never merely beside it. -/
theorem JoinIsCommutative_specializes_at_u32 (h : JoinIsCommutative) :
    ∀ a b : lattice.Flat Aeneas.Std.U32,
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq a b
        = lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq b a :=
  h Aeneas.Std.U32 u32Clone u32Eq u32Clone_lawful u32Eq_lawful

/- Boundary battery: every shape pair, both orders, exact answers. -/
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      lattice.Flat.Bottom (lattice.Flat.Elem 1#u32)
    = ok (lattice.Flat.Elem 1#u32) := by rfl
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      (lattice.Flat.Elem 1#u32) lattice.Flat.Bottom
    = ok (lattice.Flat.Elem 1#u32) := by rfl
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      lattice.Flat.Bottom lattice.Flat.Top
    = ok (lattice.Flat.Top : lattice.Flat Aeneas.Std.U32) := by rfl
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      lattice.Flat.Top lattice.Flat.Bottom
    = ok (lattice.Flat.Top : lattice.Flat Aeneas.Std.U32) := by rfl
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      (lattice.Flat.Elem 1#u32) lattice.Flat.Top
    = ok lattice.Flat.Top := by rfl
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      lattice.Flat.Top (lattice.Flat.Elem 1#u32)
    = ok lattice.Flat.Top := by rfl

end Minispec
