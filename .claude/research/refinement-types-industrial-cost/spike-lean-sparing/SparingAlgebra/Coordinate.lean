/-!
# The coordinate (`notes/277` §1; `271:rul-coordinate-shape-flat-three-place`)

The flat three-place `(kind, entity, selector)` plus the context slot. Kinds,
entities, selector tokens, contexts, and families are opaque interned tokens —
modeled as bare `Nat` ids, the engine's interner — compared only by id
(`core:inv-referent-agnostic`: never decode text for meaning).

We formalize the ALGEBRA, not the engine: no parsing, no canonicalization
machinery; entity resolution is abstracted in `Compare.lean`.
-/

namespace SparingAlgebra

structure Kind where
   id : Nat
deriving DecidableEq, Repr

structure Entity where
   id : Nat
deriving DecidableEq, Repr

structure SelectorTok where
   id : Nat
deriving DecidableEq, Repr

structure Ctx where
   id : Nat
deriving DecidableEq, Repr

structure Family where
   id : Nat
deriving DecidableEq, Repr

/--
The selector position. `top` is the bare selector-less form — permanently
"whole-entity / ⊤-selector at consumers" (`277` §1): it collides with every cell
of the entity, on either side, and — per `top-identifies-with-nothing` — never
yields `same`, even against itself.

NOTE the conflation this encoding inherits from the spec: the *definite*
whole-entity name ("true / occupied / whole-entity") and a *failed/unknown*
⊤ derivation (`277` §5 inv-top-never-encoded-as-empty) share this constructor,
which forces the chokepoint to be conservative about `top`/`top` sameness.
See REPORT.md gap-top-selector-self-sameness.
-/
inductive Selector where
   | top
   | tok (t : SelectorTok)
deriving DecidableEq, Repr

/-- Flat three-place + context slot. Deeper structure lives in kind-owner
functions BETWEEN coordinates, never in the coordinate. -/
structure Coord where
   kind : Kind
   entity : Entity
   sel : Selector
   ctx : Ctx
deriving DecidableEq, Repr

end SparingAlgebra
