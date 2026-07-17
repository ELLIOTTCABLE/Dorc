# spike/crates/core — CLAUDE.md

Role: the shared vocabulary every crate agrees on FIRST (dac-B: agree the types
before consumers build, or two incompatible graphs grow). Read `spike/CLAUDE.md`
first — its invariant clusters are this crate's law; this file carries only the
core-local sharpenings. Registry discipline: one rule per bullet, slugged; append
new entries to the matching section.

## Law — the claim-tier trust algebra (the soundness boundary lives here)

- **claim-tier-shape** — `Claim<T: Tier, P>` with act/source aliases
  `ByObservation<P>` / `ByVouch<P>` / `BySilence<P>`; tiers sealed — no fourth tier,
  ever. Any code that GATES on a claim's tier must read the `core::claim`
  doc-comments first (they carry the unrepresentability properties).
- **when-blocked-rule** (repeated from root because it is the single most important
  line in this crate): if one of these types blocks your build, you likely hold the
  WRONG claim — obtain the real vouch (author the `is_converged()`), or let the
  command run; NEVER convert a claim to satisfy the signature. That conversion IS
  the soundness hole the boundary exists to stop. A blocked "expected `ByVouch`,
  found `ByObservation`" is a measurement being laundered into a mutation-license.
- **vouch-never-a-fact** — a vouch informs a license and never enters the
  fact-plane; fact-plane exits ride `ByObservation`. Inadmissible in any other
  site's elide/poison reasoning.

## Law — the coordinate (`notes/277` is THE spec; `plans/271` the rulings)

- **flat-three-place** — the coordinate, everywhere it appears (facts, backings,
  footprints, disjointness, probe keying), is flat `(kind, entity, selector)` in a
  representation that also carries a **context slot** (default = the host-default
  world; its name is deliberately unminted). Recursive/nested coordinate shapes
  were DECLINED (`271:rul-coordinate-shape-flat-three-place`); deeper structure
  lives in kind-owner functions BETWEEN coordinates, never in the coordinate.
- **names-are-not-referents** — a coordinate names a CELL; two coordinates may name
  one cell (aliasing — why `kind__resolve()` exists). Never read
  coordinate-inequality as cell-disjointness (`272:never-derive-separation`).
- **selector-chokepoint** — `SelectorId` stays opaque/interned; EVERY selector
  comparison lives behind ONE `selector_covers`-shaped function. No caller compares
  tokens inline. The bare selector-less form permanently means
  whole-entity / ⊤-selector at consumers (collides with every cell, either side).
- **relational-compare-chokepoint** — ALL whole-coordinate comparison sits behind
  one chokepoint that MAY answer relationally; per-axis pointwise decomposition is
  never baked into the API (`271:rul-seam-context-slot-and-relational-chokepoint`).
  Verdicts are ternary {same | provably-disjoint | unknown}: same → transport only;
  provably-disjoint → flag-gated sparing only; unknown → the safe bottom for both.
- **pin-no-outcome-as-generator** — a compare-verdict feeds only its licensed
  consumer; it never re-enters the relation as evidence for a later verdict.
- **pin-set-meet-order-independence** — a coordinate-SET with any unknown member
  collides, at every iteration, whatever the member-resolution order (universal
  meet over backing-SETS; `277` §5).
- **canonical-coord-continuity** — `CanonicalCoord` stays a private mint;
  `kind__resolve()` canonicalizes the ENTITY within its kind; selectors do NOT
  canonicalize at v1; `Resolution::MayAlias` ⇒ demote. A raw coordinate cannot
  reach the intersection in a resolver-bearing kind.
- **kind-fence-movable** — cross-kind pairs short-circuit disjoint BEFORE
  canonicalization at v1, but the fence must stay MOVABLE (the parked co-reference
  mechanism lands against it; keep `CanonicalCoord` extensible toward a
  kind-carrying canonical).

## Seams — reserve representation room, build NOTHING (`277` §5)

- **seam-uniqueness-bit** — no strong update exists at v1 (`Kill` accumulates); the
  standing 231 fence: "probably unique" may only DEMOTE, never license. Room in the
  coordinate/comparison representation only.
- **seam-backing-sets** — a fact's backing is a coordinate SET, derived per-channel
  through recipe dataflow; an observe mark widens the enclosing fact's backing
  (safe direction — kill-surface only grows).
- **seam-re-bind** — (pipeline-order) the value plane runs strictly BEFORE the
  probe; folding a captured literal back needs a second value-flow pass or a
  fold-time substitution channel; (literal-provenance) keep a slot open for
  source-literal vs probe-captured distinction on values.

## Law — vocabulary discipline

- **inv-referent-agnostic** — never decode a token's/kind's text for meaning;
  compare for co-reference, resolve for display only.
- **inv-superposition-here** — `Grade`/`Verdict`/`Phase` stay phase-/orientation-
  agnostic; the phased caller collapses, never a `core` type default.
- **inv-determinism-here** — deterministic `Ord`/`Hash` for anything used as a map
  key; the `Interner` is order-of-interning; keep canonical forms so structural
  `Eq` = semantic equality.
- **inv-no-throw-here** — `core` is the no-throw spine; constructors return data,
  never panic.
