import SparingAlgebra.Compare

/-!
# Set-lifting and the universal meet (`notes/277` §5)

Consumers quantify UNIVERSALLY over backing-SETS: sparing requires EVERY
footprint×backing pair provably-disjoint; any unknown member ⇒ collide. An
accidental existential in either consumer is an under-execution path (279b-fd5).

The two guard-invariants are STRUCTURAL here:

- `inv-backing-set-nonempty-by-construction`: `BackingSet` carries the minting
  line's own coordinate as a distinguished field — ∅ is unrepresentable at the
  type. (`BackingSet.members` is always `ownCell :: rest`.)
- `inv-top-never-encoded-as-empty`: an unknown/failed derivation is an explicit
  `Member.wall` (or a selector-⊤ coordinate) — a MEMBER that collides — never
  an absence. Nothing in this module can express "empty backing".

The footprint side is a bare `List Coord`: the spec pins the ∅-hazard for
backing-sets only, and an authored-empty footprint ("disturbs at most nothing")
is arguably legitimate semantics — but the vacuity hazard is symmetric and
`277` §5 is silent about it; see REPORT.md gap-footprint-empty-set-unpinned.
-/

namespace SparingAlgebra

/-- A fact's backing: the minting line's own coordinate, plus derived members.
Non-empty by construction. -/
structure BackingSet where
   ownCell : Backing
   rest : List Member
deriving Repr

/-- The member list every consumer quantifies over. -/
def BackingSet.members (S : BackingSet) : List Member :=
   .coord S.ownCell :: S.rest

/-- Pair-level sparing license: the chokepoint answered provably-disjoint —
`unknown` fails this, which IS "any unknown member ⇒ collide" at the pair. -/
def pairSpares (E : EntityCmp) (D : Dialects) (f : Coord) (m : Member) : Bool :=
   compareM E D f m == .provablyDisjoint

/--
The survival-sparing verdict over a footprint and a backing-set: the admin flag
(`--risk-faultless-skips`; `271:rul-flag-is-razor-residue`) gates CONSUMPTION,
and the meet is universal over every footprint×backing pair. Written as the
fold the engine would run; `Laws.lean` proves it equal to the ∀-form and
order-independent.
-/
def sparesSet (E : EntityCmp) (D : Dialects) (flag : Bool)
      (F : List Coord) (S : BackingSet) : Bool :=
   flag && F.all fun f => S.members.all fun m => pairSpares E D f m

/--
The UNGUARDED encoding — a raw member list, no non-emptiness, ⊤-as-∅
expressible. Exists only so `Laws.lean` can state the vacuous-spare hazard
(`27Xf:cr-set-lifting-vacuous-at-empty`, the historically-shipped bug class)
against it. Never consume this for a verdict.
-/
def sparesRaw (E : EntityCmp) (D : Dialects) (flag : Bool)
      (F : List Coord) (members : List Member) : Bool :=
   flag && F.all fun f => members.all fun m => pairSpares E D f m

/-- The transport consumer's license, per verdict: `same` only (`277` §2).
Cross-context transport consumption is NOT modeled (routed through
`plans/27C` measure-in-context; the 275 ratifications were refused). -/
def transportLicensed : Verdict → Bool
   | .same => true
   | _ => false

/-- The sparing consumer's license, per verdict: flag-gated provably-disjoint
only. The flag permits acting on separation claims, never manufactures them. -/
def sparingLicensed (flag : Bool) : Verdict → Bool
   | .provablyDisjoint => flag
   | _ => false

end SparingAlgebra
