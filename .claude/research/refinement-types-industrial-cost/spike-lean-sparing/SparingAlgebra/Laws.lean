import SparingAlgebra.Sparing

/-!
# The laws (`notes/277` §2/§3/§5 pins, as theorem statements)

Numbered per the spike brief. Each theorem's doc-comment cites the spec sentence
it formalizes; REPORT.md carries the per-theorem ledger and every place the spec
underdetermined a choice made here.
-/

namespace SparingAlgebra

/-! ## 1 — `sparing_requires_every_pair_disjoint` (`277` §5 set-lifting) -/

/-- The fold the engine runs decides EXACTLY the spec's universal meet: flagged,
and every footprint×backing pair provably-disjoint — no stronger, no weaker.
(An accidental existential on either side would break the → direction.) -/
theorem sparesSet_iff_universal
      (E : EntityCmp) (D : Dialects) (flag : Bool) (F : List Coord) (S : BackingSet) :
      sparesSet E D flag F S = true ↔
         (flag = true ∧ ∀ f ∈ F, ∀ m ∈ S.members, compareM E D f m = .provablyDisjoint) := by
   simp [sparesSet, Bool.and_eq_true, List.all_eq_true, pairSpares, beq_iff_eq]

/-- "sparing over a backing-set requires EVERY footprint×backing pair
provably-disjoint". The fold the engine runs implies the ∀-form. -/
theorem sparing_requires_every_pair_disjoint
      (E : EntityCmp) (D : Dialects) (flag : Bool) (F : List Coord) (S : BackingSet)
      (h : sparesSet E D flag F S = true) :
      ∀ f ∈ F, ∀ m ∈ S.members, compareM E D f m = .provablyDisjoint := by
   intro f hf m hm
   simp only [sparesSet, Bool.and_eq_true, List.all_eq_true, pairSpares, beq_iff_eq] at h
   exact h.2 f hf m hm

/-- "any unknown member ⇒ collide": ONE pair failing provable-disjointness
defeats the whole set, whatever else it contains. -/
theorem one_unknown_member_collides
      (E : EntityCmp) (D : Dialects) (flag : Bool) (F : List Coord) (S : BackingSet)
      {f : Coord} {m : Member} (hf : f ∈ F) (hm : m ∈ S.members)
      (h : compareM E D f m ≠ .provablyDisjoint) :
      sparesSet E D flag F S = false := by
   cases hs : sparesSet E D flag F S with
   | false => rfl
   | true => exact absurd (sparing_requires_every_pair_disjoint E D flag F S hs f hf m hm) h

/-- The flag gates consumption: no sparing verdict exists without it
(`271:rul-flag-is-razor-residue` — per-invocation, never a default). -/
theorem no_sparing_without_flag (E : EntityCmp) (D : Dialects)
      (F : List Coord) (S : BackingSet) :
      sparesSet E D false F S = false := by
   simp [sparesSet]

/-! ### The two guard-invariants, structural (`277` §5) -/

/-- inv-backing-set-nonempty-by-construction: `BackingSet.members` is
`ownCell :: rest` — ∅ is unrepresentable. -/
theorem backing_set_members_nonempty (S : BackingSet) : S.members ≠ [] := by
   simp [BackingSet.members]

/-- ... and the minting line's own coordinate is always a member. -/
theorem own_cell_always_member (S : BackingSet) :
      Member.coord S.ownCell ∈ S.members := by
   simp [BackingSet.members]

/-- inv-top-never-encoded-as-empty, positively: the explicit ⊤ member COLLIDES —
it defeats sparing against every footprint (and, being `unknown`, licenses no
transport either). -/
theorem wall_member_collides
      (E : EntityCmp) (D : Dialects) (flag : Bool) (F : List Coord) (S : BackingSet)
      (hw : Member.wall ∈ S.members) {f : Coord} (hf : f ∈ F) :
      sparesSet E D flag F S = false :=
   one_unknown_member_collides E D flag F S hf hw (by simp [compareM])

/-- The hazard the guards exclude (`27Xf:cr-set-lifting-vacuous-at-empty`, the
historically-shipped bug class): over the UNGUARDED encoding, universal
quantification over an empty backing-set is vacuously true — silence-as-license
reached through an ABSENT claim rather than a wrong one. Cautionary: this is
the verdict `BackingSet` makes unrepresentable. -/
theorem vacuous_spare_over_empty_backing
      (E : EntityCmp) (D : Dialects) (F : List Coord) :
      sparesRaw E D true F ([] : List Member) = true := by
   simp [sparesRaw]

/-- The SYMMETRIC hazard the spec does not pin (REPORT.md
gap-footprint-empty-set-unpinned): the universal meet is vacuous over an empty
FOOTPRINT too — and this holds of the GUARDED encoding, whose guards protect
only the backing side. Legitimate iff an empty footprint can only arise from an
authored empty at-most claim — never as an encoding of unknown/⊤ disturbance.
`277` §5 states no such invariant for the footprint side. -/
theorem vacuous_spare_over_empty_footprint
      (E : EntityCmp) (D : Dialects) (S : BackingSet) :
      sparesSet E D true ([] : List Coord) S = true := by
   simp [sparesSet]

/-! ### The pre-amendment falsehood, checkably (`279f:fix-spare-top-backing`) -/

/-- `277` §3's PRE-amendment wording, reconstructed: only the claim side's ⊤ was
special-cased ("a ⊤ claim spares nothing"); a whole-entity (⊤) backing fell
through to the dialect test, sparable by any dialect-member claim. -/
def selectorTierPreAmendment (row : DialectRow) (k : Kind) : Selector → Selector → Verdict
   | .top, _ => .unknown
   | .tok c, .tok b =>
      if c = b then .same
      else if row k c && row k b then .provablyDisjoint
      else .unknown
   | .tok c, .top => if row k c then .provablyDisjoint else .unknown

/-- 279a-A5 as an inhabitant: under the pre-amendment wording a minted claim
SPARES a whole-entity backing — kill-traffic dismissed against the very cell
the fact sits on, the under-execution path the amendment closed. (Contrast
`top_never_spares` for the amended tier.) -/
theorem pre_amendment_spares_top_backing :
      ∃ (row : DialectRow) (k : Kind) (c : SelectorTok),
         selectorTierPreAmendment row k (.tok c) .top = .provablyDisjoint :=
   ⟨fun _ _ => true, ⟨0⟩, ⟨0⟩, rfl⟩

/-! ## 2 — `set_meet_order_independent` (`277` §5 pin-set-meet-order-independence) -/

/-- `List.all` is invariant under permutation (self-contained; no mathlib). -/
theorem all_perm {α : Type} (p : α → Bool) {l₁ l₂ : List α} (h : l₁.Perm l₂) :
      l₁.all p = l₂.all p := by
   induction h with
   | nil => rfl
   | cons x _ ih => simp [List.all_cons, ih]
   | swap x y l => simp [List.all_cons, Bool.and_left_comm]
   | trans _ _ ih₁ ih₂ => exact ih₁.trans ih₂

/-- "a set with any unknown member collides at every iteration, whatever the
member-resolution order": the fold's verdict is invariant under any permutation
of the member list (and of the footprint list). -/
theorem set_meet_order_independent
      (E : EntityCmp) (D : Dialects) (flag : Bool) (F : List Coord)
      (S₁ S₂ : BackingSet) (hp : S₁.members.Perm S₂.members) :
      sparesSet E D flag F S₁ = sparesSet E D flag F S₂ := by
   have hall : ∀ f : Coord,
         (S₁.members.all fun m => pairSpares E D f m)
            = S₂.members.all fun m => pairSpares E D f m :=
      fun _ => all_perm _ hp
   simp only [sparesSet, hall]

/-- The footprint side, same property. -/
theorem footprint_order_independent
      (E : EntityCmp) (D : Dialects) (flag : Bool) (F₁ F₂ : List Coord)
      (S : BackingSet) (hp : F₁.Perm F₂) :
      sparesSet E D flag F₁ S = sparesSet E D flag F₂ S := by
   simp only [sparesSet, all_perm _ hp]

/-- The pin, combined: an unknown member defeats the set under EVERY member
order — no resolution order can spare around it. -/
theorem unknown_member_collides_under_any_order
      (E : EntityCmp) (D : Dialects) (flag : Bool) (F : List Coord)
      (S₁ S₂ : BackingSet) (hp : S₁.members.Perm S₂.members)
      {f : Coord} {m : Member} (hf : f ∈ F) (hm : m ∈ S₁.members)
      (h : compareM E D f m ≠ .provablyDisjoint) :
      sparesSet E D flag F S₂ = false := by
   rw [← set_meet_order_independent E D flag F S₁ S₂ hp]
   exact one_unknown_member_collides E D flag F S₁ hf hm h

/-! ## 3 — `consumer_map_safety_inversion` (`277` §2; `273` §4's inversion) -/

/-- Ground truth, abstracted: which coordinate-pairs denote one cell, and which
referents share mutable state. Names are not referents — this is the plane the
verdicts are judged against. -/
structure World where
   sameCell : Coord → Coord → Prop
   overlap : Coord → Coord → Prop
   same_overlap : ∀ a b, sameCell a b → overlap a b

/-- A compare instance is sound for a world when its determined verdicts are
true of the world; `unknown` commits to nothing (generator-INCOMPLETENESS is
value-loss only — `277` §2 properties). -/
def SoundCompare (W : World) (E : EntityCmp) (D : Dialects) : Prop :=
   ∀ (claim : Coord) (b : Backing),
      (compare E D claim b = .same → W.sameCell claim b.coord) ∧
      (compare E D claim b = .provablyDisjoint → ¬ W.overlap claim b.coord)

/-- Under a sound compare, the transport consumer only ever acts on genuinely
same cells. -/
theorem sound_transport_never_misfires
      (W : World) (E : EntityCmp) (D : Dialects) (hs : SoundCompare W E D)
      (claim : Coord) (b : Backing)
      (h : transportLicensed (compare E D claim b) = true) :
      W.sameCell claim b.coord := by
   cases hv : compare E D claim b with
   | same => exact (hs claim b).1 hv
   | provablyDisjoint => rw [hv] at h; simp [transportLicensed] at h
   | unknown => rw [hv] at h; simp [transportLicensed] at h

/-- Under a sound compare, the sparing consumer only ever spares genuinely
non-overlapping referents — kill-traffic it dismisses truly misses. -/
theorem sound_sparing_never_misfires
      (W : World) (E : EntityCmp) (D : Dialects) (hs : SoundCompare W E D)
      (flag : Bool) (claim : Coord) (b : Backing)
      (h : sparingLicensed flag (compare E D claim b) = true) :
      ¬ W.overlap claim b.coord := by
   cases hv : compare E D claim b with
   | same => rw [hv] at h; simp [sparingLicensed] at h
   | provablyDisjoint => exact (hs claim b).2 hv
   | unknown => rw [hv] at h; simp [sparingLicensed] at h

/-- `unknown` licenses neither consumer — the safe bottom for BOTH. -/
theorem unknown_licenses_neither (flag : Bool) :
      transportLicensed .unknown = false ∧ sparingLicensed flag .unknown = false :=
   ⟨rfl, rfl⟩

/-- ... and (flag on) it is the UNIQUE verdict licensing neither: the two
determined verdicts each feed exactly their consumer. -/
theorem unknown_uniquely_idle (v : Verdict) :
      (transportLicensed v = false ∧ sparingLicensed true v = false) ↔ v = .unknown := by
   cases v <;> simp [transportLicensed, sparingLicensed]

/-! ### The inversion witness

Why the relation must be ternary: one SOUND situation — verdict `unknown`,
referents genuinely overlapping, cells not the same — indicts both binary
defaults at once. Collapse `unknown` to believed-no-overlap and the sparing
consumer spares a pair whose kill-traffic actually hits (silent
under-execution, the cardinal sin); collapse it to believed-overlap/same and
the transport consumer transports across distinct cells. `273` §4:
believed-no-overlap and believed-overlap are each dangerous to one consumer;
only unknown is safe for both. -/

/-- Every same-kind pair genuinely overlaps; no pair is the same cell. -/
def strawWorld : World where
   sameCell _ _ := False
   overlap a b := a.kind = b.kind
   same_overlap _ _ h := h.elim

/-- The resolver that honestly knows nothing (`MayAlias` everywhere). -/
def strawE : EntityCmp := fun _ _ _ _ => .mayAlias

/-- The empty world: no oracles loaded, no dialects minted. -/
def strawD : Dialects := fun _ _ _ => false

def strawCoord : Coord := ⟨⟨0⟩, ⟨0⟩, .top, ⟨0⟩⟩

def strawBacking : Backing := ⟨strawCoord, ⟨0⟩⟩

theorem straw_sound : SoundCompare strawWorld strawE strawD := by
   intro claim b
   constructor
   · intro h
     simp only [compare, strawE] at h
     split at h
     · split at h <;> simp_all
     · simp_all
   · intro h hov
     simp only [compare, strawE] at h
     split at h
     next hctx =>
        split at h
        next hkind => simp_all
        next hkind => exact hkind hov
     next hctx => simp_all

theorem straw_verdict_unknown :
      compare strawE strawD strawCoord strawBacking = .unknown := by
   decide

/-- The witness: a sound situation where acting on either binary reading of
`unknown` misfires its consumer. -/
theorem safety_inversion_witness :
      ∃ (W : World) (E : EntityCmp) (D : Dialects) (c : Coord) (b : Backing),
         SoundCompare W E D ∧ compare E D c b = .unknown ∧
         W.overlap c b.coord ∧ ¬ W.sameCell c b.coord :=
   ⟨strawWorld, strawE, strawD, strawCoord, strawBacking,
    straw_sound, straw_verdict_unknown, rfl, fun h => h⟩

/-- The named bundle the brief asks for: sound consumers never misfire; unknown
is the unique verdict idle for both; and the witness shows each binary default
dangerous for exactly one consumer — the asymmetry that forces ternary. -/
theorem consumer_map_safety_inversion :
      (∀ W E D, SoundCompare W E D → ∀ claim b,
         (transportLicensed (compare E D claim b) = true → W.sameCell claim b.coord) ∧
         (∀ flag, sparingLicensed flag (compare E D claim b) = true → ¬ W.overlap claim b.coord)) ∧
      (∀ v, (transportLicensed v = false ∧ sparingLicensed true v = false) ↔ v = .unknown) ∧
      (∃ W E D c b, SoundCompare W E D ∧ compare E D c b = .unknown ∧
         W.overlap c b.coord ∧ ¬ W.sameCell c b.coord) :=
   ⟨fun W E D hs claim b =>
      ⟨sound_transport_never_misfires W E D hs claim b,
       fun flag => sound_sparing_never_misfires W E D hs flag claim b⟩,
    unknown_uniquely_idle,
    safety_inversion_witness⟩

/-! ## 4 — `no_outcome_as_generator` (`277` §5 pin)

This pin lands as an API-SHAPE property, not a theorem (as the spec's fixpoint
clause anticipates): the evidence plane — `EntityCmp` (returning `EntityRes`)
and `Dialects` — is Verdict-free BY TYPE, so a compare outcome cannot re-enter
the relation as evidence without an explicit, greppable conversion. Lean cannot
forbid a client WRITING such a conversion; the guarantee is that laundering is
never silent. The example below is a compile-time tripwire: it stops compiling
the day anyone adds a `Coe Verdict EntityRes`. -/

example : True := by
   fail_if_success have : EntityRes := (Verdict.unknown : EntityRes)
   trivial

/-! ## 5 — `cross_family_monotone` (`277` §3 properties, post-`279f` wording) -/

/-- "a newly loaded family never alters comparisons against OTHER families'
backings" — the QUALIFIED monotonicity (the absolute form was falsified:
within-family growth may flip collide→spare against that family's own backings,
and that is declared kill-surface control). Structural root: `selectorTier`
receives only the backing family's row, so other rows are invisible by type. -/
theorem cross_family_monotone
      (E : EntityCmp) (D D' : Dialects) (f₀ : Family) (h : LoadsOnly D D' f₀)
      (claim : Coord) (b : Backing) (hb : b.mintedBy ≠ f₀) :
      compare E D' claim b = compare E D claim b := by
   have hrow : D' b.mintedBy = D b.mintedBy := h b.mintedBy hb
   simp [compare, hrow]

/-- Lifted over members (the wall is evidence-free on both sides). -/
theorem cross_family_monotone_member
      (E : EntityCmp) (D D' : Dialects) (f₀ : Family) (h : LoadsOnly D D' f₀)
      (claim : Coord) (m : Member)
      (hm : ∀ b : Backing, m = .coord b → b.mintedBy ≠ f₀) :
      compareM E D' claim m = compareM E D claim m := by
   cases m with
   | wall => rfl
   | coord b => exact cross_family_monotone E D D' f₀ h claim b (hm b rfl)

/-! ## Riders — cheap pins the spec names as test obligations -/

/-- `279f:fix-spare-top-backing` (closed under-execution path 279a-A5): a ⊤
claim spares nothing within the entity, AND a whole-entity backing is spared by
nothing — BOTH sides, not just the claim side the pre-amendment wording
special-cased. -/
theorem top_never_spares (row : DialectRow) (k : Kind) (s : Selector) :
      selectorTier row k .top s ≠ .provablyDisjoint ∧
      selectorTier row k s .top ≠ .provablyDisjoint := by
   cases s <;> simp [selectorTier]

/-- `top-identifies-with-nothing`, selector tier: ⊤ never yields `same`, even
against itself — a ⊤ member must block transport, because the encoding cannot
distinguish the definite whole-entity name from a failed derivation. -/
theorem top_never_same (row : DialectRow) (k : Kind) (s : Selector) :
      selectorTier row k .top s ≠ .same ∧ selectorTier row k s .top ≠ .same := by
   cases s <;> simp [selectorTier]

/-- Empty world (no oracles loaded, no dialect rows): the selector tier never
manufactures disjointness — the algebra is invisible, entity-granular behavior
survives byte-identical (`277` §6). -/
theorem empty_world_no_selector_sparing (k : Kind) (cs bs : Selector) :
      selectorTier (fun _ _ => false) k cs bs ≠ .provablyDisjoint := by
   cases cs <;> cases bs <;> simp [selectorTier] <;> split <;> simp

/-- No self-licensing (`277` §3 properties): a claim never spares ITS OWN
coordinate, whatever family utters it — provided the entity resolver does not
lie an entity apart from itself (a generator-soundness side-condition; the
algebra cannot repair a lying `kind__resolve`). -/
theorem no_self_licensing (E : EntityCmp) (D : Dialects) (c : Coord) (fam : Family)
      (hE : E c.kind c.ctx c.entity c.entity ≠ .distinct) :
      compare E D c ⟨c, fam⟩ ≠ .provablyDisjoint := by
   simp only [compare, if_true]
   cases hEv : E c.kind c.ctx c.entity c.entity with
   | distinct => exact absurd hEv hE
   | mayAlias => simp
   | same => cases c.sel <;> simp [selectorTier]

end SparingAlgebra
