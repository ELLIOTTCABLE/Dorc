import Generated.Funs
import Minispec.Vocabulary.TrustedBase

/-!
# JoinIsIdempotent

**The law, in English (authoritative):** joining a flat-lattice value with itself gives
back exactly that value, successfully. For every type `T` with lawful clone and
equality dictionaries and any `a : Flat T`, `a ⊔ a = a`.

This is the law the solver's convergence detection rests on: the fixpoint test asks
whether a join CHANGED anything, and idempotence is what makes "nothing changed" a
stable answer rather than an accident of representation. It is stated over the DERIVED
definitions — the translated bodies of the shipping Rust.

The hypotheses are the named trusted-base entries (`Minispec/Vocabulary/TrustedBase.lean`); the
element case genuinely needs both (the translated body consults equality, then clones).
The battery evaluates every boundary shape's self-join on concrete lawful dictionaries.
-/

namespace Minispec

open generated
open Aeneas.Std Result

/-- Self-join is the identity, for every lawful dictionary pair. -/
def JoinIsIdempotent : Prop :=
  ∀ (T : Type) [DecidableEq T]
    (cl : core.clone.Clone T) (eqi : core.cmp.Eq T),
    LawfulClone cl → LawfulEq eqi →
    ∀ a : lattice.Flat T,
      lattice.Flat.Insts.GeneratedLatticeLattice.join cl eqi a a = ok a

/-- Anti-vacuity: the element case is the non-trivial one — the translated body
    genuinely consults the equality dictionary and the clone (it is not a
    constructor-shape short-circuit), and still returns its argument. -/
theorem JoinIsIdempotent_nonvacuous :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      (lattice.Flat.Elem 7#u32) (lattice.Flat.Elem 7#u32)
    = ok (lattice.Flat.Elem 7#u32) := by rfl

/-- Coupling: the law's own `Prop`, applied at the battery ground. An edit that
    decouples the statement from the seat it names breaks this theorem, so the
    battery is mechanically about the law, never merely beside it. -/
theorem JoinIsIdempotent_specializes_at_u32 (h : JoinIsIdempotent) :
    ∀ a : lattice.Flat Aeneas.Std.U32,
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq a a = ok a :=
  h Aeneas.Std.U32 u32Clone u32Eq u32Clone_lawful u32Eq_lawful

/- Boundary battery: the two remaining shapes' self-joins. -/
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      lattice.Flat.Bottom lattice.Flat.Bottom
    = ok (lattice.Flat.Bottom : lattice.Flat Aeneas.Std.U32) := by rfl
example :
    lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
      lattice.Flat.Top lattice.Flat.Top
    = ok (lattice.Flat.Top : lattice.Flat Aeneas.Std.U32) := by rfl

end Minispec
