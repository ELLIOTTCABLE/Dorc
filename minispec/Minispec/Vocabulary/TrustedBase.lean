import Generated.Funs

/-!
# TrustedBase — the named hypotheses every generic statement carries

The Rust→Lean translation models a generic bound like `T: Clone + Eq` as explicit
dictionary parameters — records of OPAQUE functions. Nothing in the translation says a
clone returns its argument, or that equality means equality: the real guarantees live in
the Rust compiler and its derive machinery, which keep enforcing them over the shipping
code. A Lean statement over a generic seat therefore SAYS what it assumes of those
dictionaries, by carrying one of the predicates below as an explicit hypothesis.

Each hypothesis is a NAMED TRUSTED-BASE ENTRY (`28T` ev-aeneas-experiment: the derived
`Clone`/`Eq` models are lawless). A law quantifying over these is honest — "for every
lawful dictionary" — and a law that dropped them would be unprovable, not stronger.
Concrete dictionaries used by the instance batteries prove these predicates outright,
so nothing in a battery rests on an unproven assumption.

These are translation-scaffolding vocabulary, not Dorc law (`301` §1: generic
scaffolding lives outside the units); they are governed shared vocabulary all the same —
changing what "lawful" means here changes what every unit claims, so edits are ceremony.
-/

namespace Minispec

open generated
open Aeneas.Std Result

/-- The derived `Clone` is the identity: cloning returns its argument, successfully.
    (The Rust `#[derive(Clone)]` on the modeled types guarantees exactly this; the
    translation cannot see that, so statements assume it by name.) -/
def LawfulClone {T : Type} (cl : core.clone.Clone T) : Prop :=
  ∀ x : T, cl.clone x = ok x

/-- The derived `Eq` decides propositional equality: the modeled `eq` returns
    successfully, answering `true` exactly when its arguments are equal. (The Rust
    doc-comment's "semantic `Eq`" precondition, stated over the dictionary the
    translated code actually calls.) -/
def LawfulEq {T : Type} [DecidableEq T] (eqi : core.cmp.Eq T) : Prop :=
  ∀ x y : T,
    core.cmp.impls.PartialEqShared.eq eqi.partialEqInst x y = ok (decide (x = y))

/-- The batteries' concrete ground: `U32` with the identity clone and decidable
    equality. Shared by every unit's instance battery; lawful by construction, proven
    outright below — a worked example of what the predicates mean. -/
def u32Clone : core.clone.Clone Aeneas.Std.U32 := { clone := fun x => ok x }
def u32Eq : core.cmp.Eq Aeneas.Std.U32 :=
  { partialEqInst := { eq := fun x y => ok (decide (x = y)) } }

theorem u32Clone_lawful : LawfulClone u32Clone := fun _ => rfl
theorem u32Eq_lawful : LawfulEq u32Eq := fun _ _ => rfl

end Minispec
