import Generated.Funs
import Minispec.Vocabulary.TrustedBase

/-!
# JoinIsAssociative

**The law, in English (authoritative):** grouping does not matter when joining three
flat-lattice values. For every type `T` with lawful clone and equality dictionaries and
any `a b c : Flat T`, joining `a` with `b` and then the result with `c` gives exactly
what joining `a` with the join of `b` and `c` gives.

Because the derived `join` returns in the translation's `Result`, the two groupings are
compared as monadic compositions: `(a ⊔ b) >>= (· ⊔ c)` against `(b ⊔ c) >>= (a ⊔ ·)`.
That is the honest translated form of the textbook law. The monadic SHAPE is kept —
nothing projects out of `Result` — but under the lawfulness hypotheses the
dictionaries cannot fail, so the error channel is unreachable here: this law asserts
nothing about failure behaviour.

Associativity is what lets a solver fold many incoming edges in any grouping and land on
one answer. Stated over the DERIVED definitions; hypotheses are the named trusted-base
entries (`Minispec/Vocabulary/TrustedBase.lean`). The battery below drives the one genuinely
subtle grouping family — element values under partial equality — plus the absorbing and
neutral shapes.
-/

namespace Minispec

open generated
open Aeneas.Std Result

/-- Grouping-independence of join, as monadic composition, for every lawful
    dictionary pair. -/
def JoinIsAssociative : Prop :=
  ∀ (T : Type) [DecidableEq T]
    (cl : core.clone.Clone T) (eqi : core.cmp.Eq T),
    LawfulClone cl → LawfulEq eqi →
    ∀ a b c : lattice.Flat T,
      (do
        let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join cl eqi a b
        lattice.Flat.Insts.GeneratedLatticeLattice.join cl eqi x c)
      = (do
        let y ← lattice.Flat.Insts.GeneratedLatticeLattice.join cl eqi b c
        lattice.Flat.Insts.GeneratedLatticeLattice.join cl eqi a y)

/-- Anti-vacuity: the subtle family — `p = q ≠ r` collapses differently through the two
    groupings' INTERMEDIATES (left passes through `Elem p`, right through ⊤) yet both
    land on ⊤; the law is doing real work, not following from shapes alone. -/
theorem JoinIsAssociative_nonvacuous :
    (do
      let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        (lattice.Flat.Elem 1#u32) (lattice.Flat.Elem 1#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        x (lattice.Flat.Elem 2#u32))
    = ok lattice.Flat.Top
    ∧ (do
      let y ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        (lattice.Flat.Elem 1#u32) (lattice.Flat.Elem 2#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        (lattice.Flat.Elem 1#u32) y)
    = ok lattice.Flat.Top := by
  constructor <;> rfl

/-- Coupling: the law's own `Prop`, applied at the battery ground. An edit that
    decouples the statement from the seat it names breaks this theorem, so the
    battery is mechanically about the law, never merely beside it. -/
theorem JoinIsAssociative_specializes_at_u32 (h : JoinIsAssociative) :
    ∀ a b c : lattice.Flat Aeneas.Std.U32,
      (do
        let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq a b
        lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq x c)
      = (do
        let y ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq b c
        lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq a y) :=
  h Aeneas.Std.U32 u32Clone u32Eq u32Clone_lawful u32Eq_lawful

/- Boundary battery: ⊥ neutral in any slot; ⊤ absorbing in any slot; distinct-element
   escalation stable under both groupings. -/
example :
    (do
      let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        lattice.Flat.Bottom (lattice.Flat.Elem 3#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        x lattice.Flat.Bottom)
    = ok (lattice.Flat.Elem 3#u32) := by rfl
example :
    (do
      let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        lattice.Flat.Top (lattice.Flat.Elem 3#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        x (lattice.Flat.Elem 3#u32))
    = ok lattice.Flat.Top := by rfl
example :
    (do
      let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        (lattice.Flat.Elem 1#u32) (lattice.Flat.Elem 2#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        x (lattice.Flat.Elem 3#u32))
    = ok lattice.Flat.Top := by rfl

end Minispec
