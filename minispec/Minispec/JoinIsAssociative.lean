import Generated.Funs
import Minispec.TrustedBase

/-!
# JoinIsAssociative

**The law, in English (authoritative):** grouping does not matter when joining three
flat-lattice values. For every type `T` with lawful clone and equality dictionaries and
any `a b c : Flat T`, joining `a` with `b` and then the result with `c` gives exactly
what joining `a` with the join of `b` and `c` gives — including the error behaviour:
neither grouping can fail where the other succeeds.

Because the derived `join` returns in the translation's `Result`, the two groupings are
compared as monadic compositions: `(a ⊔ b) >>= (· ⊔ c)` against `(b ⊔ c) >>= (a ⊔ ·)`.
That is the honest translated form of the textbook law; nothing about `Result` is
assumed away.

Associativity is what lets a solver fold many incoming edges in any grouping and land on
one answer. Stated over the DERIVED definitions; hypotheses are the named trusted-base
entries (`Minispec/TrustedBase.lean`). The battery below drives the one genuinely
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
  constructor <;> native_decide

/- Boundary battery: ⊥ neutral in any slot; ⊤ absorbing in any slot; distinct-element
   escalation stable under both groupings. -/
example :
    (do
      let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        lattice.Flat.Bottom (lattice.Flat.Elem 3#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        x lattice.Flat.Bottom)
    = ok (lattice.Flat.Elem 3#u32) := by native_decide
example :
    (do
      let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        lattice.Flat.Top (lattice.Flat.Elem 3#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        x (lattice.Flat.Elem 3#u32))
    = ok lattice.Flat.Top := by native_decide
example :
    (do
      let x ← lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        (lattice.Flat.Elem 1#u32) (lattice.Flat.Elem 2#u32)
      lattice.Flat.Insts.GeneratedLatticeLattice.join u32Clone u32Eq
        x (lattice.Flat.Elem 3#u32))
    = ok lattice.Flat.Top := by native_decide

end Minispec
