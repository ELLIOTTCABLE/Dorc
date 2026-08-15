# 303 — the derived-definitions lane, as built: what translates, what does not, and why

> Tier: LLM-authored builder note (Opus, round-30 wave-1, `lane-derived-definitions-pipeline`
> + `lane-minispec-verify-standup`). Subordinate to root docs, `spike/CLAUDE.md`, `notes/301`
> (THE spec) and `notes/300` (the arc ledger). Append-only; a later round mints a new note
> rather than editing this one. Confidence marks per the house discipline.

## §1 — What stands

- `spike/verify/aeneas/` — the translation lane. `src/lib.rs` is a translation UNIT, not a
  copy: it `#[path]`-includes `crates/core/src/sorted.rs` and `crates/analysis/src/lattice.rs`
  verbatim and re-aliases the crate with `extern crate self as dorc_core` so `lattice.rs`'s own
  import resolves unedited. A copy would drift, and a drifted derived definition makes every
  law stated over it a statement about code that no longer exists.
- `minispec/` — the lake package: `Generated/` (committed, `@generated`), `Minispec/` (three
  unwritten unit stubs), `Minispec/Proofs/`, `CLAUDE.md` (conductor stub), `REPORT.md`
  (generated).
- `spike/verify/` — the `dorc-verify` binder: catalogue lock, computed badge set, generated
  report, the cheap gate, and the pipeline's non-charon halves.

## §2 — Findings: three aeneas limitations, one charon limitation

All four are +SURE — each was isolated with a minimal probe outside the repo tree, and each has
a proved-translating cousin where one is claimed.

- **`fnd-closure-return-borrow-aborts-aeneas`** — `Option::map` where the closure's RETURN
  borrows the closure's own argument aborts aeneas ("Can't end abstraction N as it is set as
  non-endable"). The `match`-spelled cousin of the identical operation translates cleanly;
  probed in both directions, for shared, field, tuple and `&mut` reborrows. Hits
  `SortedMap::{get, get_mut, get_at}`.
- **`fnd-mut-closure-emits-ill-typed-lean`** — the SHARPEST of the four, because it is silent.
  `SortedMap::insert`'s `.map(|entry| mem::replace(&mut entry.1, value))` translates with NO
  error from charon or aeneas, and the emitted Lean does not typecheck: the generated
  `call_once` returns `Result (V × (K × V))` (the `&mut` lens back-channel widening the return)
  while the `FnOnce` dictionary it populates declares `closure → (K × V) → Result V`. Only
  `lake build` catches it. This generalizes turn08's sorry-census law: **a green translate
  proves nothing without a green lake build, and a green lake build proves nothing without a
  hole census.** Both are now wired.
- **`fnd-unwrap-or-else-trait-fn-item-unimplemented`** — `Option::unwrap_or_else` with a
  TRAIT-METHOD fn item (`V::bottom`) is "Unimplemented" in aeneas
  (`SymbolicToPureTypes.ml:426`). A plain `fn` item is fine; `Option::<&V>::cloned()` alone is
  fine; the `match { Some(v) => v.clone(), None => V::bottom() }` cousin is fine. Hits
  `MapL::get`.
- **`fnd-charon-cannot-name-an-inherent-impl`** — charon 0.1.232's name-matcher cannot address
  an inherent impl block. `{impl Type<_, _>}`, `{Type<_,_>}`, `{impl Type}` and the `@`-typevar
  forms all PARSE and match nothing; only `_` matches an impl. `SortedSet` and `SortedMap` share
  the `sorted` module, so fencing the map's accessors fences the set's too.

## §3 — The reshaping this asks the human for

`300` §2a's `fnd-iterator-exits-may-not-translate` anticipated the exits. It did not anticipate
the ALGEBRA PROPER choking, and the brief's rule for that case is report-don't-patch, so
nothing in `crates/` was touched. But the anticipated cause did NOT occur: the
`while let Some(x) = v.get(i)` walk shape translates perfectly, and `SortedSet::{position,
insert, remove, contains, union, intersection, get_at}` and `SortedMap::{position, remove}` all
came through clean. The facade's SHAPE is fine.

What chokes is five one-line idioms, all `Option`-combinator spellings with a
proved-translating `match` cousin — turn08's `disc-keep-borrows-out-of-closure-returns-and-
option-holds` discipline, which that note already recommended adopting as house style:

| site | today | the cousin that translates |
|---|---|---|
| `SortedMap::get_at` | `self.entries.get(i).map(\|(k, v)\| (k, v))` | `match … { Some((k, v)) => Some((k, v)), None => None }` |
| `SortedMap::get` | `…get(at).map(\|(_, v)\| v)` | the same `match` |
| `SortedMap::get_mut` | `…get_mut(at).map(\|(_, v)\| v)` | the same `match` |
| `SortedMap::insert` | `…get_mut(at).map(\|e\| mem::replace(&mut e.1, value))` | the same `match` |
| `MapL::get` | `self.0.get(k).cloned().unwrap_or_else(V::bottom)` | `match … { Some(v) => v.clone(), None => V::bottom() }` |

~SUSPECT (not +SURE) that the five reshaped bodies then translate AND typecheck end to end:
each cousin was proved to translate, but the reshaped facade has not been through a lake build
as a set. Cheap to settle — one `verify:translate` + `verify:lean` after the reshape lands.

The reshape is a `crates/core` + `crates/analysis` edit inside the verified core, so it is a
human/conductor act. Closing it deletes the whole `(b)` and `(c)` half of the fence.

## §4 — The fence, and what it costs today

`spike/verify/aeneas/Cargo.toml`'s `[package.metadata.charon]` holds one `opaque` list, each
entry classed. An opaque item keeps its signature and loses its body, landing as a named axiom
the report counts — loud, never silent.

- (a) EXITS, permanent: `iter` / `into_iter` / `from_iter` on both facades and on
  `Powerset`/`MapL`. The algebra proper never calls them.
- (b) BLOCKED, the §2 findings: `SortedMap::{get, get_mut, get_at, insert}`, `MapL::get`.
- (c) COLLATERAL of `fnd-charon-cannot-name-an-inherent-impl`: `SortedSet::{get_at, insert}`,
  which translate perfectly and are axiomatized anyway because the pattern cannot discriminate.
  **`SortedSet::insert` is the canonical-form seat** (`300` §2a), so this is the fence's real
  cost: no Lean law about set insertion is provable until (b) closes. The v0 remit claims are
  over `Flat`, which is untouched, so nothing currently planned is blocked.

`analysis::effect::Reach` is absent rather than fenced — it is not in the translation unit at
all (`300:fnd-reach-lattice-outside-scope`), and the config says so.

## §5 — Measurements

- Strict translation: 0 errors, 0 warnings, **0 proof holes**, 40 external axioms, ~1s.
- `lake build` over the whole package: green, 1707 jobs, ~2.5 min warm.
- Aeneas's OWN Lean library carries holes (`Aeneas/Std/Slice.lean`, `StringIter.lean`) — a
  trusted-base entry the `Generated/` census structurally cannot see. `verify:lean` now counts
  every `declaration uses 'sorry'` across the build and reports it, because Lean says it in one
  line of a 1700-job build, which is how a trusted-base entry becomes invisible.
- Byte-budget tripwire: **8 KB**, calibrated against the sparing-algebra spike's
  statement-bearing files (1.9 / 2.0 / 3.3 / 5.2 KB) and deliberately below its 17.5 KB
  many-theorems-in-one-file shape. Our stubs are ~1.5 KB of prose before a law exists.

## §6 — `301` §7 answers

1. **Verso: NOT adopted; the named fallback taken.** The `301` §1 fallback (structured
   doc-comments + include-by-generation) is what stands: units carry Lean module docstrings
   (`/-! … -/`), and the transclusion half is unbuilt. Evidence and reasoning in §7 below.
2. Reach certification — named seam, unbuilt (`seam-kani-pairing-unbuilt`'s sibling).
3. **2a. The decision-record surface: NOT trivially consumable; named seam
   `seam-decision-record-read-mode`.** The whylog carries the right shape —
   `plan::whylog::ApplyLine { leaf, disposition, predicted }`, and `parse` is public and total
   — but three things block it. (i) It records `leaf: u32`, NOT the full `SiteId` (leaf +
   optional in-loop `member`), so two in-loop member sites COLLAPSE, and `301` §2 names `SiteId`
   as the identity. (ii) It is a durable in the controller's state dir under
   `whylog-write-only-replay` / `probe-tape-not-a-cache`; a loom run does not surface one.
   (iii) The plan route has no machine dump-mode at all — `--format=jsonl` is `dorc lint`'s,
   and `expected.out` is the rendered artifact, i.e. the render-plane coupling `301` §2
   explicitly refuses. The minimal product feature that closes it: a plan-route decision dump
   emitting `(SiteId, decision)` pairs, `SiteId`-keyed rather than leaf-keyed. Real feature,
   useful to any consumer, and a dispatch of its own.
4. Tripwire threshold — §5.
5. Remit claims — three PLACEHOLDER slugs stand (`JoinIsCommutative`, `JoinIsIdempotent`,
   `LeqIsReflexive`), expanded from `301` §4's two-word candidates to satisfy the ≥3-word slug
   law. The human's pick replaces them.

## §7 — Why Verso was not taken

Three costs, none individually fatal, jointly decisive at v0:

- **A toolchain the pin cannot absorb.** `minispec/lean-toolchain` is dictated by aeneas's own
  `backends/lean/lean-toolchain` (v4.31.0) — not chosen, and not movable without moving the
  aeneas pin. Verso is a separate lake dependency with its own Lean-version compatibility
  window, so adopting it adds a second pin that must agree with a version we do not control.
  ~SUSPECT the current Verso would resolve against v4.31.0; NOT verified, and verifying it
  costs another multi-gigabyte dependency store.
- **The transclusion it was wanted for is the half that is furthest away.** `301` §1's
  "one rendered page" wants the law's bound demonstration TRANSCRIPTS transcluded
  cross-directory. There are no bindings yet (§6 item 2a is a seam), so the feature would be
  built against nothing.
- **It buys nothing the stubs need.** A Verso genre's value is rendered output; what a unit
  needs today is that an LLM opening the file reads prose first, which a module docstring
  already does, and that the file elaborates, which it does.

The fallback is not a dead end: units are ordinary Lean with docstrings, so adopting Verso
later is a per-file change with no data migration. Re-open it when the first binding exists and
a rendered page has something to render.

## §8 — Flagged, not resolved

- `tc-fence-collateral-costs-the-canonical-seat` — `SortedSet::insert` axiomatized as
  collateral (§4c). Accepted at v0 because the remit claims are over `Flat`; it is a real
  narrowing of what the Lean tier can say, and it evaporates when §3 lands.
- `tc-report-is-cheap-tier-only` — `minispec/REPORT.md` publishes the CHEAP-tier render, and
  `report --write --with-lean` is refused, so the committed artifact is never a cache of
  evidence the ordinary gate cannot recompute. The with-lean render prints and gates, and is
  deliberately never published. Judgement call, cleanly reversible.
- `tc-unit-declaration-contract-is-strawman` — the names the binder looks for
  (`def <Slug> : Prop`, `theorem <Slug>_nonvacuous`, `theorem <Slug>_holds`) are STRAWMAN and
  rename freely. They are the first thing an authoring lane will want to argue with.
