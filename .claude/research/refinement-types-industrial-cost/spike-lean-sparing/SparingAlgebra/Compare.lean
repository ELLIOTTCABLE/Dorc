import SparingAlgebra.Dialect

/-!
# The ternary chokepoint (`notes/277` §2; `core:relational-compare-chokepoint`)

One comparison, everywhere:

    compare(cellA@ctxA, cellB@ctxB) ∈ { same | provably-disjoint | unknown }

Consumer map: `same` feeds transport only; `provably-disjoint` feeds flag-gated
survival sparing only; `unknown` is the safe bottom for BOTH consumers.

We build compare in the directional claim-vs-backing form the survival consumer
uses. `277` §2 writes a symmetric-looking relation, but the §3 dialect tier
consults the BACKING's minting family, and only backings carry one — see
REPORT.md gap-compare-symmetry-vs-directionality.
-/

namespace SparingAlgebra

/--
The entity-tier resolution outcome — deliberately a type DISTINCT from
`Verdict`, though isomorphic. `pin-no-outcome-as-generator`: a compare-verdict
must never re-enter the relation as evidence; keeping the evidence vocabulary a
different type means laundering a verdict back in requires an explicit,
greppable conversion function. (Lean cannot forbid a client writing one; the
guarantee is non-silence, not unrepresentability — see REPORT.md.)
-/
inductive EntityRes where
   | same
   | distinct
   | mayAlias
deriving DecidableEq, Repr

/--
Kind-owner entity canonicalization (`kind__resolve()`), abstracted: within a
kind and context, do two entity names denote one entity (`same`),
provably-distinct entities (`distinct` — the entity-granular baseline), or
possibly-aliasing names (`mayAlias` ⇒ demote; `core:canonical-coord-continuity`)?
A generator, vouch-tier: its soundness is the kind-owner's burden, hypothesized
where theorems need it, never assumed globally.
-/
abbrev EntityCmp := Kind → Ctx → Entity → Entity → EntityRes

/-- The ternary verdict (`277` §2). -/
inductive Verdict where
   | same
   | provablyDisjoint
   | unknown
deriving DecidableEq, Repr

/--
The selector tier (`selector_covers`-shaped; `277` §3 as amended by
`279f:fix-spare-top-backing`), reached only same-kind/same-entity/same-context.
Takes ONE family's row — the backing's minting family's — so other families'
vocabularies are invisible by type (see `Dialect.lean`).

- equal minted tokens ⇒ the same coordinate ⇒ `same` (structural Eq = semantic
  equality; canonical forms);
- two distinct tokens, BOTH in the backing family's dialect ⇒ `provablyDisjoint`
  (the claim narrows within a vocabulary whose minting family vouches its cells
  are distinct);
- everything else — ⊤ on either side, unminted tokens, cross-dialect tokens —
  `unknown`: never `same` (top-identifies-with-nothing), never disjoint
  (⊤-selector collides with every cell, either side).

Reading note (REPORT.md gap-both-sides-minted-redundancy): the spec's "BOTH
sides carry minted selectors" is rendered here as `row k c && row k b` — the
backing-side conjunct is true by construction in the engine (backing selectors
come from minting lines) but is checked anyway, so the algebra does not depend
on that engine invariant.
-/
def selectorTier (row : DialectRow) (k : Kind) : Selector → Selector → Verdict
   | .tok c, .tok b =>
      if c = b then .same
      else if row k c && row k b then .provablyDisjoint
      else .unknown
   | _, _ => .unknown

/--
THE chokepoint, layered:

1. context inequality ⇒ `unknown` — `272` §4 never-derive-separation: derivation
   yields keying, never separation; keying blocks transport and NEVER yields
   provably-disjoint. (v1 models no cross-context `same` generators: declared
   separation across context-values does not exist at all, and the invariance
   line / pure-predicate carry are out of scope here.)
2. kind inequality ⇒ `provablyDisjoint` — the kind-fence: cross-kind pairs
   short-circuit disjoint before canonicalization (v1; movable).
3. the entity tier (abstract generator; `mayAlias` demotes);
4. the selector tier, on the backing family's row only.
-/
def compare (E : EntityCmp) (D : Dialects) (claim : Coord) (b : Backing) : Verdict :=
   if claim.ctx = b.coord.ctx then
      if claim.kind = b.coord.kind then
         match E claim.kind claim.ctx claim.entity b.coord.entity with
         | .distinct => .provablyDisjoint
         | .mayAlias => .unknown
         | .same => selectorTier (D b.mintedBy) claim.kind claim.sel b.coord.sel
      else .provablyDisjoint
   else .unknown

/--
A backing-set member: a provenance-carrying coordinate, or the explicit full
wall. `277` §5 inv-top-never-encoded-as-empty: a failed/unknown/⊤ backing
derivation yields an explicit ⊤ MEMBER — "collides with every footprint, blocks
all transport" — never ∅. The spec's ⊤ member is stated UNQUALIFIED (every
footprint, even cross-kind), which a selector-⊤ under a known kind cannot
deliver (the kind-fence would spare cross-kind footprints against it); hence
this member-level wall constructor. See REPORT.md gap-top-member-granularity.
-/
inductive Member where
   | coord (b : Backing)
   | wall
deriving DecidableEq, Repr

/-- Member-level compare: the wall is `unknown` against everything. -/
def compareM (E : EntityCmp) (D : Dialects) (claim : Coord) : Member → Verdict
   | .coord b => compare E D claim b
   | .wall => .unknown

end SparingAlgebra
