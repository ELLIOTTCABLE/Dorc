/- Hand-written, NOT generated: the smallest real question — can the lattice laws
   the Rust doc-comment already states be stated and proved against the extraction?

   Every law here is about `Flat`, the ONE combinator whose Rust body is a plain
   match with no loops and no collections. If the laws are expensive here, they are
   worse everywhere else.

   Note the DOUBLED namespace: the crate is `lattice` and so is the module, so the
   extracted names are `lattice.lattice.Flat`. -/
import Lattice
open Aeneas Aeneas.Std Result
open lattice.lattice

namespace LatticeLaws

/- The generated `join` takes the two instance dictionaries the Rust bound
   `T: Clone + Eq` desugars to. Both are RECORDS OF OPAQUE FUNCTIONS: Aeneas's
   `core.cmp.PartialEq` is `structure ... where eq : Self → Rhs → Result Bool`,
   carrying no reflexivity, no symmetry, no transitivity. The Rust doc-comment's
   precondition "`L`'s `Eq` is semantic" has no counterpart to appeal to. -/

/- ⊥ ⊔ a = a — the first law in the Rust doc-comment, and UNPROVABLE as stated.
    The Rust arm is `(Flat::Bottom, x) | (x, Flat::Bottom) => x.clone()`, so the
    extraction returns `cl.clone a`, and `cl.clone` is a field of an opaque record:
    nothing says a clone returns its argument. Uncomment to watch it get stuck.

theorem join_bot_left_UNPROVABLE {T} (cl : core.clone.Clone T) (eq : core.cmp.Eq T)
    (a : Flat T) :
    Flat.Insts.LatticeLatticeLattice.join cl eq Flat.Bottom a = ok a := by
  unfold Flat.Insts.LatticeLatticeLattice.join Flat.Insts.CoreCloneClone.clone
  cases a <;> simp   -- residual goal mentions `cl.clone`, which is a free variable
-/

/-- The glue a Dorc proof effort would carry at EVERY generic seat: the assumption
    that the derived `Clone` is the identity. Not derivable from the extraction. -/
def LawfulClone {T} (cl : core.clone.Clone T) : Prop := ∀ x : T, cl.clone x = ok x

theorem join_bot_left {T} (cl : core.clone.Clone T) (eq : core.cmp.Eq T)
    (hcl : LawfulClone cl) (a : Flat T) :
    Flat.Insts.LatticeLatticeLattice.join cl eq Flat.Bottom a = ok a := by
  unfold Flat.Insts.LatticeLatticeLattice.join Flat.Insts.CoreCloneClone.clone
  cases a <;> simp [hcl _]

theorem join_bot_right {T} (cl : core.clone.Clone T) (eq : core.cmp.Eq T)
    (hcl : LawfulClone cl) (a : Flat T) :
    Flat.Insts.LatticeLatticeLattice.join cl eq a Flat.Bottom = ok a := by
  unfold Flat.Insts.LatticeLatticeLattice.join Flat.Insts.CoreCloneClone.clone
  cases a <;> simp [hcl _]

/-- The SECOND assumption the extraction cannot supply: that the `Eq` dictionary
    decides equality. With both, ⊔-idempotence goes through. -/
def LawfulEq {T} [DecidableEq T] (eq : core.cmp.Eq T) : Prop :=
  ∀ x y : T, eq.partialEqInst.eq x y = ok (decide (x = y))

theorem join_idem {T} [DecidableEq T] (cl : core.clone.Clone T) (eq : core.cmp.Eq T)
    (hcl : LawfulClone cl) (heq : LawfulEq eq) (a : Flat T) :
    Flat.Insts.LatticeLatticeLattice.join cl eq a a = ok a := by
  unfold Flat.Insts.LatticeLatticeLattice.join Flat.Insts.CoreCloneClone.clone
  cases a <;> simp [hcl _, heq _ _, core.cmp.impls.PartialEqShared.eq]

/- The honesty check the sibling Lean spike also runs: no `sorryAx`, and in
   particular none of the `axiom`s in the generated `FunsExternal`. -/
#print axioms join_bot_left
#print axioms join_bot_right
#print axioms join_idem

end LatticeLaws
