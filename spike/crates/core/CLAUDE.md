# spike/crates/core — CLAUDE.md

Role: the shared vocabulary every crate agrees on FIRST (dac-B: agree the types
before consumers build, or two incompatible graphs grow) — the DECIDE plane. The
DESCRIBE plane (diagnostics, catalog, render, narrative records, `Carrier`) is
`crates/aid`, which deps this crate and is never depended upon BY it (`288` §2a).
Read `spike/CLAUDE.md` first — its invariant clusters are this crate's law; this file
carries only the core-local sharpenings. Registry discipline: one rule per bullet, slugged; append
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
- **custody-is-one-newtype-and-one-crossing** (`28M` §8; `28P` bitem3) — `DefinitionCustody`
  names WHOSE utterance a license rests on, and `LicenseCustody` names which of the three
  single-author shapes a `ReplaceLicense` is (`Vouched` / `VouchedSeverally` / `MeasuredSelf`).
  Construction is ONE seat and consumers only ever COMPARE custodies — never read the file id to
  decide anything — because `28M` §10 `dir-ownership-is-transitive-inclusion` (UNRULED) may re-key
  custody from the defining file to an entry file's transitive sourcing-closure, and that re-key
  must stay a change to this type's internals. **Never key a NEW decision off a raw
  `SourceFileId`.** `defining_file()` is provenance and display only. The variant set is the fence:
  a widening that reproduced ANOTHER author's measured value under this author's license fits none
  of the three, so it cannot compile without adding a variant here — which is the point, and the
  only place that decision should ever be visible.
- **contested-is-write-once** (`28K` §1 `rul-silent-shadowing-refuses`) —
  `ContestedFamilies` is the license-plane fact naming the role families whose licenses
  are WITHHELD for a run: built once from the function environment's own answer, keyed by
  the MUNGED family base (so `apt-get`/`apt_get` are one family and every `__role` member
  is covered by one key), and read-only thereafter. There is no `un_contest`, no `remove`,
  no `&mut` accessor, and there must never be one — the refusal is sound because it can
  under-fire but never un-withhold, and that property is what let it ship ahead of the
  decidable-condition fold (`28M` §9, since landed) and what must survive every later
  sharpening of what the environment can prove. The same type now also carries the cli's
  per-file never-live withdrawal, which is why "write-once" is a property of the VALUE and
  not of how many are built. The DiagCode derives FROM the fact; licensure never reads a
  diagnostic (`two-plane-aid-law`).

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
- **auto-cell-is-the-markless-floor** — `auto_fact` mints the typeless floor's
  per-provider singleton (`24L` §2/§3) and `is_auto_kind` recognizes it. Since `26H` §3 a
  verdict body that AUTHORS a coordinate keys that cell instead, so the auto-cell is the
  RESIDUAL markless case, not every verdict-only site. Two consumers, and only one is
  about the kind: `fence-no-disjoint` (the synthetic singleton must never manufacture
  separation) reads it and must keep doing so; which-body-ships does NOT — an authored
  verdict cell is an ordinary kind, indistinguishable from a predict-minted one, so that
  discriminator is site-keyed in `analysis`. Never re-derive a lane from a kind test.

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
- **inv-no-throw-here** — constructors return data, never panic. (`Carrier<T>`, the
  no-throw spine type itself, lives in `crates/aid`; `core` returns bare values and
  never accumulates diagnostics.)
- **site-identity-is-decide-plane** — `SiteId` (`leaf` + optional in-loop `member`) lives
  here beside `LeafId`, not in the describe plane: it is the identity two same-command
  sites must not collapse across (`inv-site-keyed-results`), shared by the probe-records
  lane, the apply plan's steps, and every diagnostic. `aid` re-exports it; it is never
  re-minted.

## Law — the verified-core substrate (r30; `notes/300` §2/§2a, `28T` heritage)

- **sorted-facade-law** — the algebra tier's ordered storage is
  `core::sorted::{SortedSet, SortedMap}` (private-backed sorted `Vec`s); raw
  `BTreeMap`/`BTreeSet` never appear in verified-core code (checker and reference
  implementations included — the `verified-core-discipline` skill's code-shape rule).
  Canonical form (strict ascent: `∀i: get_at(i) < get_at(i+1)`) is HONOUR-SYSTEM, not
  type-carried: each invariant has ONE named seat (the private `position` scan feeding
  `insert`/`remove`) plus its seat tests, and the Kani lane's exhaustive pins are the
  closing net. `#[cfg(kani)] Arbitrary` impls home in this module and construct via an
  arbitrary `Vec` + `kani::assume(canonical)` — NEVER via repeated `insert`, which
  makes the insert harnesses circular.
- **keep-borrows-out-of-closure-returns** (`304`; the reshape) — in the TRANSLATED
  algebra tier (`core::sorted`, `analysis::lattice`): no Option-combinator whose
  closure RETURNS a borrow of its argument, no `mem::replace` inside `.map`, no
  `unwrap_or_else(<trait method>)` — spell the `match` cousin. The Aeneas pipeline
  breaks on these, once SILENTLY (an ill-typed emission only `lake build` catches).
  The fence and its classes live in `spike/verify/aeneas/Cargo.toml`; a `.map`
  reintroduction re-breaks translation invisibly unless `verify:lean` runs.
