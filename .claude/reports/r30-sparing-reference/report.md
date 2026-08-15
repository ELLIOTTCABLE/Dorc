# R30 sparing reference report

## Built reference model

Created the zero-dependency `dorc-sparing-reference` workspace library. Its
input vocabulary is independent of production types: opaque kind, entity,
selector, and family tokens; coordinates with top, unminted, or family-minted
selectors; claims; backing coordinates; non-empty backing sets; and explicit
dialects.

The library exposes three total, pure operations:

- `compare`: the closed ternary coordinate relation (`Same`,
  `ProvablyDisjoint`, or `Unknown`);
- `spare_pair`: the survival-sparing consumer for one claim/backing pair; and
- `spare_set`: the universal meet over every footprint-by-backing pair.

The implementation is a direct nested scan. It has no dependencies,
allocation, worklist, cache, recursion, outcome feedback, or hidden boolean
verdicts. `BackingSet::new(first, rest)` makes the empty set unrepresentable.
The workspace member was registered mechanically by Cargo without consulting
production Rust source.

Cargo regenerates a `dorc-sparing-reference` package stanza in
`spike/Cargo.lock` when the workspace is checked. The brief permits only
`spike/Cargo.toml` outside the new crate, so that generated lockfile change was
removed after every local check. The conductor must decide whether the
lockfile is an implicit exception before folding.

## Per-law example coverage

- Sparing algebra: positive minted/different/in-dialect pair; same-selector
  collision; selector-less claim and backing; unminted claim and backing;
  cross-dialect collision; and absent-dialect collision.
- Ternary consumer map: exact minted coordinate produces `Same`; a licensed
  differing selector produces `ProvablyDisjoint`; missing knowledge produces
  `Unknown`; only `ProvablyDisjoint` maps to pair sparing, while `Same` and
  `Unknown` collide.
- Universal set lifting: an all-disjoint 2-by-2 example spares; an unknown
  footprint member collides; an explicit top backing member collides; reversing
  known/unknown backing order still collides.
- No outcome as generator: an unknown comparison remains unknown before and
  after evaluating an unrelated provably-disjoint comparison.
- Non-empty backing construction: a singleton retains its mandatory first
  member and cannot report empty; top is represented as that explicit member
  rather than as emptiness.
- Top identifies with nothing: top compared with itself is `Unknown`.
- No cross-kind same: equal selector/entity tokens under different kinds are
  `Unknown`.
- Silence licenses nothing: no dialect evidence yields `Unknown` and collision.
- Conservative ambiguity pins: unequal entities and an empty footprint both
  collide.

The battery contains 22 ordinary unit tests. Each test name states the law or
instance it demonstrates.

## Flagged ambiguous readings

1. Unequal entities: the assigned selector-algebra excerpt says cross-entity
   disjointness is “unchanged,” but the assigned formal excerpts do not define
   the prior unequal-entity generator. `compare` therefore returns `Unknown`,
   and sparing collides. The site is marked `// FLAGGED:` and tested by
   `different_entities_collide_without_a_ratified_generator`.

2. Empty footprints: the law makes backing sets non-empty by construction but
   does not state whether footprints are non-empty or what universal lifting
   over an empty footprint means. `spare_set` conservatively returns
   `Collides`. The site is marked `// FLAGGED:` and tested by
   `an_empty_footprint_collides_conservatively`.

## Conductor routing

No ordinary implementation item required routing to the conductor from the
mandated first read.

## Verification record

- `cargo test -p dorc-sparing-reference --ignore-rust-version`: 22 passed,
  0 failed, with the sandbox-installed Rust 1.95 toolchain.
- `cargo clippy -p dorc-sparing-reference --all-targets
  --ignore-rust-version -- -D warnings`: clean.
- `cargo fmt -p dorc-sparing-reference -- --check`: clean.
- `mise run check`: blocked before task execution because the sandbox denied
  `mise` access to its configuration (`Access is denied`).
- `cargo test -p dorc-sparing-reference`: blocked before compilation because
  the ambient Rust 1.95 toolchain is below the workspace's Rust 1.96 minimum.
- `mise run gate:quick-quiet`: blocked before task execution by the same sandbox
  denial affecting `mise run check`.

Staging and commits were also blocked because the sandbox denied writes to the
worktree Git index outside the writable worktree root; the escalation request
was rejected. The implementation and this report therefore remain uncommitted
for conductor-side staging.
