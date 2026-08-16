# Authoring Kani harnesses (the measured shaping discipline)

Read this BEFORE writing or modifying any `#[kani::proof]` harness. Every rule here
is measured, not theorized — the evidence base is the r30 battery (107/107 green,
zero counterexamples, zero over-budget after reshaping; `Research/notes/300a` §1
carries the raw measurements). Harnesses written without this discipline reliably
either OOM the solver or verify vacuously.

## The economics (what CBMC actually pays for)

CBMC turns the harness into one logic formula and solves it. Cost is dominated by
SYMBOLIC UNKNOWNS carried through mutation: a collection of unknown length forces
the solver to model every possibility simultaneously, and two things multiply that
into unaffordability — a reallocation under symbolic length (the allocator's move
modeled under every case), and the cross-product of two symbolic walks. Anchor
numbers, same law both rows: concrete length 2, full backing → green in 2 s;
symbolic length ≤3 with a growing insert → 21 min, 3.6 GB, OOM. A spare-capacity
assumption alone moved a harness four orders of magnitude.

## Rule 1 — concrete, declared sizes

One harness per length (or length-pair), the size in the harness NAME
(`set_insert_preserves_canonical_form_at_length_2`) and as a const generic — never
a number in doc prose (measured drift: five doc-comments understated their own
bounds; const generics removed the failure mode by construction). N harnesses per
law is honest, not bloat: each declares exactly the universe it verified. A
gestured-at bound that was never actually covered is worse than a small one that
was.

## Rule 2 — the two unaffordable input shapes

Never write: (a) a growing mutation applied to a symbolic-length collection;
(b) two symbolic-length collections in one harness, even read-only (every walk of
one multiplies every walk of the other). Shrinking the element domain does NOT
help (measured: a four-value domain moved nothing); concretizing length does.

## Rule 3 — count the inserts INSIDE the operators

Concrete inputs are necessary, not sufficient. An operator that grows a collection
element-by-element goes symbolic after its own first insert — `union` clones the
left side then inserts the right per-element, so its second insert lands on an
already-symbolic length whatever the input sizes were. Consequences, measured:
a merge is affordable only when it performs ≤1 insert (lopsided length-pairs:
`union` at (0–2, 0–1) green, (·,2) unaffordable); and a law that COMPOSES one
merge into another (absorption, ⊔/⊓-are-bounds, associativity over collection
combinators) has NO affordable shape at ANY size. Route those laws to the
property-test/seat-test tier, and say so in the harness file where the harness
would have been — never chase them with budget.

## Rule 4 — generators

- Never build values by the mutation under test: generating sets via repeated
  `insert` makes every `insert` harness assume what it proves.
- The pattern: draw an arbitrary backing, `kani::assume(<canonical/invariant>)`.
- Every assumed invariant is PAIRED with a closing harness proving the real
  producer maintains it, inductively (base: the empty value satisfies it; step:
  an arbitrary producer-call over an arbitrary satisfying value still satisfies
  it). An unpaired assume is a hole — the assumed class may not contain what
  production actually builds. Worked example: `mint_maintains_the_dialect_invariant`
  closing `Dialect`'s `every_key_has_a_token`.
- Guard vacuity with an in-generator `assert!` on satisfiability: Kani turns it
  into a proof obligation, so an unsatisfiable assume FAILS loudly instead of
  greening every downstream harness on an empty universe.
- Spare-capacity assumptions (`capacity > len`) are legitimate when reallocation
  is NOT the property under test — cover the reallocating path once, separately,
  at a concrete full-backing size (`…_when_the_backing_moves`).

## Rule 5 — subsumption hygiene

A harness whose generator and assertions are a strict subset of another's at the
same size is redundant; prefer deleting it in the same change that lands the
superset (flag if unsure). Shared law-helpers beat duplicated law text — but a
change to a shared helper changes every caller's checked statement; treat that as
statement-review-tier and disclose it.

## Rule 6 — when NOT Kani

Escalate for reach, never prestige. Large or genuinely-two-sided universes →
property tests. Ordering/permutation/multiset stability → DST pins. Types still
on raw `BTree*` backings → exhaustive-small tests beside the type (Kani cannot
reach them; migrate when they sit on the facade). Types whose dependency closure
cannot enter the detached `#[path]`-include unit → exhaustive-small in their own
crate, with statements written to move unchanged later.

## Mechanics

The harness crate `spike/verify/kani` is a DETACHED `#[path]`-include unit — it
judges the shipping source bytes, not a copy, and exists outside the workspace
because of the Kani-toolchain MSRV wall. `#[cfg(kani)]` support (generators,
`Arbitrary` impls) lives crate-local beside the types it serves, never in the
harness crate — that is what reaches private backings without production
widening. The lane: `mise run verify:kani` (Linux/WSL only; the driver runs one
harness at a time under wall-clock + address-space gates, reaps CBMC by exact
name between harnesses, and classifies green / FAILED / OVER-BUDGET with the
resource gates checked BEFORE any verdict text is believed). A no-Kani rot check
(`cargo check` of the detached unit) runs even where the toolchain is absent.
`DORC_KANI_HARNESS_BUDGET_SECS` narrows the wall-clock gate for calibration runs;
narrowing can only move a harness judged→unjudged, never the reverse.
