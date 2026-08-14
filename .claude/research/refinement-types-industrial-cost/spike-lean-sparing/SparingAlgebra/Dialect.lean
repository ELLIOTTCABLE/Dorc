import SparingAlgebra.Coordinate

/-!
# The selector dialect (`notes/277` §3)

`dialect(family, kind)` = the selector tokens that FAMILY's verdict/observe marks
carry for that kind. No global per-kind vocabulary exists. Dialects grow only by
oracle-load (authored marks parsed at oracle-read); a host can never mint a
selector at runtime.

Structural choice, load-bearing for `cross_family_monotone`: the selector-tier
comparison (`Compare.lean`) receives a single family's `DialectRow`, not the whole
`Dialects` map — the comparison CANNOT see other families' rows, by type. That is
the qualified monotonicity, made structural.

Also load-bearing for `pin-no-outcome-as-generator`: every evidence type here is
Verdict-free by construction.
-/

namespace SparingAlgebra

/-- One family's minted vocabulary, per kind. Decidable membership. -/
abbrev DialectRow := Kind → SelectorTok → Bool

/-- The full evidence surface: per-family rows. -/
abbrev Dialects := Family → DialectRow

/--
A backing coordinate carries its minting provenance (`277` §3: "backing
provenance (minting family) carried into the comparison"). Only backings have
one — footprint/claim coordinates come from claim/disturbs emissions, which
never mint. This one-sidedness is why the sparing-tier comparison is
irreducibly directional; see REPORT.md gap-compare-symmetry-vs-directionality.
-/
structure Backing where
   coord : Coord
   mintedBy : Family
deriving DecidableEq, Repr

/--
"Loading a new family f₀" — the only growth operation on evidence: it may write
f₀'s rows and must leave every OTHER family's row untouched. (`277` §3
properties, as reworded per `279f:fix-dialect-properties`: the ABSOLUTE
monotonicity was falsified — within-family growth may flip collide→spare against
that family's own backings, and that is declared kill-surface control.)
-/
def LoadsOnly (D D' : Dialects) (f₀ : Family) : Prop :=
   ∀ f, f ≠ f₀ → D' f = D f

end SparingAlgebra
