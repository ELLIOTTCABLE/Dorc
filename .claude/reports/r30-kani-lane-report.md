# r30 `lane-kani-harnesses` — builder report

Branch `ai/r30-lane-kani`, based on `5e6d6788`. Written to be read cold by a successor with
no context from the lane's session. Two environment deaths happened during this lane; §7
is the honest account of what caused them and what now prevents a third.

**Headline: 19 of 37 harnesses verify green; 18 are over-budget; ZERO counterexamples.** Nothing
in the algebra was refuted. The over-budget 18 are unjudged, not broken, and §1's shaping rule
says precisely what shape would judge them.

---

## §1 — The harness battery (law · bounds · result)

Home: `spike/verify/kani/`, a DETACHED crate (`[workspace]` empty, no `rust-version`) whose
`src/lib.rs` `#[path]`-includes `crates/core/src/lib.rs` and `crates/analysis/src/lattice.rs`.
Harnesses in `src/harness/{facade,lattice_laws,coordinate}.rs`.

Run under the mandated gates: 25 s wall-clock and 6 GiB address space per harness, reaped
between each. **OVER-BUDGET means UNJUDGED** — the harness proved nothing and refuted nothing.
Two gates can trip it, and they are not equivalent: a wall-clock trip might pass with a longer
budget, whereas an ADDRESS-SPACE trip is budget-independent (the three `compare_*` rows are
confirmed address-space trips — CBMC reported `Out of memory` / `CBMC failed with status 6`
inside 21 s, so no amount of waiting helps them).

### Green (19)

| harness | law · seat | bounds | time |
|---|---|---|---|
| `selector_covers_never_spares_a_top_or_a_self` | `sparing-algebra` + the `279f:fix-spare-top-backing` regression · `coord::selector_covers` | dialects ≤3 tokens, 2-identity symbols | 3.50 s |
| `selector_covers_needs_both_tokens_minted` | unminted ⇒ collide; empty dialect ⇒ never spares (`empty-world-byte-identical`) | dialects ≤3 | 4.25 s |
| `selector_identifies_only_two_concrete_equal_tokens` | `top-identifies-with-nothing` · `coord::selector_identifies` | 2-identity symbols | 0.75 s |
| `the_consumer_map_is_exhaustive_and_exclusive` | `ternary-compare-consumer-map` — no verdict feeds both consumers; `Unknown` feeds neither | whole `Relation` domain × both identity answers | 0.50 s |
| `the_universal_meet_is_order_independent` | `pin-set-meet-order-independence` + `pin-no-outcome-as-generator` | all 27 member-triples, all 6 orders | 0.50 s |
| `an_empty_backing_set_would_spare_vacuously` | `inv-backing-set-nonempty-by-construction` · `inv-top-never-encoded-as-empty` | ∅ and all singletons | 0.50 s |
| `set_membership_agrees_with_the_walk` | `position`'s sole scan agrees with a naive walk · `SortedSet::position` | `SortedSet<u8>` ≤3 | 3.25 s |
| `set_remove_preserves_canonical_form` | strict ascent survives removal; removal reported iff it happened | `SortedSet<u8>` ≤3 | 5.26 s |
| `set_insert_preserves_canonical_form_when_the_backing_moves` | THE canonical-form seat, across a reallocation · `SortedSet::insert` | `SortedSet<u8>` exactly 2, full backing | 2.00 s |
| `map_get_at_walks_key_order` | the walk and the lookup see one map · `SortedMap::get_at` | `SortedMap<u8,u8>` ≤3 | 3.25 s |
| `map_remove_keeps_key_order` | key order survives removal; the unbound value returns | `SortedMap<u8,u8>` ≤3 | 6.00 s |
| `flat_obeys_the_binary_laws` | ⊥/idempotence/commutativity/absorption/both ⊑ readings · `Lattice` | `Flat<u8>`, WHOLE domain | 1.00 s |
| `flat_is_associative` | ⊔ and ⊓ associative | `Flat<u8>`, whole domain | 1.00 s |
| `flat_obeys_the_top_laws` | ⊤ identity of ⊓, absorbing for ⊔ · `BoundedLattice::top` | `Flat<u8>`, whole domain | 0.75 s |
| `product_obeys_the_binary_laws` | componentwise — both components' laws survive pairing | `Product<Flat<u8>,Flat<u8>>`, whole domain | 2.25 s |
| `product_obeys_the_top_laws` | bounded only when BOTH components are | `Product<Flat<u8>,Flat<u8>>`, whole domain | 1.00 s |
| `may_obeys_the_binary_and_top_laws` | `May<L>` is the identity wrapper | `May<Flat<u8>>`, whole domain | 5.51 s |
| `must_obeys_the_binary_and_top_laws` | `Must<L>` is a lawful lattice — **this passing IS the proof the dual is right** | `Must<Flat<u8>>`, whole domain | 5.00 s |
| `must_is_the_order_dual_of_its_inner_lattice` | ⊔≡⊓, ⊥≡⊤ at the operation level (`inv-must-may`'s substrate) | `Flat<u8>`, whole domain | 1.00 s |

The `Must`/`May` pair is the one I would flag to a reviewer as most worth its cost: the whole
one-engine-both-orientations design rests on `Must<L>` being the order-dual, nothing else picks
the merge, and a silent mis-choice there is a wrong-elision under the opposite phase.

### Over-budget (18) — unjudged, grouped by which clause of the shaping rule they trip

| harness | trip |
|---|---|
| `compare_answers_unknown_across_a_context_gap` | ADDRESS SPACE (`Dialect::any_minted` mints via real `insert`) |
| `compare_answers_unknown_on_a_resolver_gap` | ADDRESS SPACE, same generator |
| `compare_derives_separation_only_from_its_three_sources` | ADDRESS SPACE, same generator |
| `selector_covers_is_monotone_in_the_dialect` | growing `insert` at symbolic length |
| `set_insert_preserves_canonical_form` | growing `insert`, symbolic ≤3 |
| `set_insert_touches_only_its_own_member` | growing `insert` + clone, symbolic ≤3 |
| `set_union_is_canonical_and_semantic` | `union` inserts per element, two collections |
| `set_intersection_is_canonical_and_semantic` | `intersection` inserts per element, two collections |
| `set_structural_eq_is_set_eq` | TWO symbolic-length collections, cross-lookups |
| `map_insert_keeps_keys_ascending_and_rebinds` | growing `insert`, symbolic ≤3 |
| `map_structural_eq_is_binding_eq` | TWO symbolic-length collections, cross-lookups |
| `powerset_obeys_the_binary_laws` | `Powerset` ⊔/⊓ insert per element |
| `powerset_is_associative` | as above, three values |
| `mixed_product_is_a_lattice_without_a_top` | `Powerset` component, as above |
| `maplattice_obeys_the_binary_laws` | `MapL` ⊔/⊓ insert per key |
| `maplattice_is_associative` | as above, three values |
| `maplattice_keeps_its_canonical_form` | `MapL::insert` + two maps |
| `maplattice_merges_pointwise` | two maps, per-key equations |

Every one of the 18 draws either a growing mutation at symbolic length or two symbolic-length
collections — the two clauses measured below. None is a statement about the algebra being wrong.

**What this leaves genuinely unverified at this tier**, stated plainly rather than buried: the
canonical-form seat under a symbolic-length backing, both set operators, both structural-equality
laws, and every lattice law over the two collection-shaped combinators (`Powerset`, `MapL`).
Those all keep their existing seat tests (`300` §2a's bank), which is what they had before this
lane; the Kani tier adds nothing to them yet.

## §2 — Placements flagged (what did NOT go to Kani, and why)

Three targets from the brief are at a lower rung than the brief's default, each flagged here
rather than silently decided.

- **`analysis::effect::Reach`'s cause-excluding equality** (`303:fnd-reach-equality-excludes-its-cause`)
  → **exhaustive-small tests, in `analysis::effect`'s own test module.** `Reach` is still
  raw-`BTreeSet`-backed (`300:fnd-reach-lattice-outside-scope` defers the eviction), and a
  `BTreeSet` is out of this tier's reach. Three tests landed: the equivalence-relation laws
  plus cause-blindness, the CONGRUENCE property (equal-modulo-cause inputs give
  equal-modulo-cause outputs under `join`/`meet`/`leq` — the one that actually makes the
  fixpoint terminate), and the lattice laws read through that equality. The statements are
  written so they move to `spike/verify/kani` UNCHANGED when `Reach` moves onto the facade.
- **`plan`'s `normalise_edits` span non-overlap** → **exhaustive-small tests, in `plan`'s own
  test module.** `SpanEdit` lives inside `dorc-plan`, whose dependency closure cannot enter a
  dependency-free `#[path]`-include unit, and its `String` fields are the shape a bounded model
  checker pays most for. The quantifier is a loop over every interval collection a five-byte
  source can express — exhaustive over that universe rather than over a bound. Pins: survivors
  are sorted and share no byte (what `emit_span_edits`'s right-to-left splice rests on), the
  OUTER edit wins a nested pair, duplicates collapse.
- **The narrative-fold permutation pins** (on the 28T target list) → **not built by this lane,
  and property/DST-tier by shape.** They are about output-multiset stability across orderings,
  which is DST's quantifier, not Kani's. Flagged, not attempted.

One more placement worth stating because it is NOT a retreat: the **universal meet over
backing-SETS** (target d) IS in the Kani battery, but its fold is the harness's own, exactly as
`coord.rs`'s existing test writes it. Backing-sets are a reserved seam
(`core/CLAUDE.md:seam-backing-sets` — singletons at v1), so there is no production fold to
verify yet. What is pinned is the LAW the seam must be built to, before there is anything to get
wrong. `selector_covers` and `compare` — the real seats — are verified directly.

## §3 — The `pinned` badge: `seam-kani-pairing-unbuilt` is CLOSED

Built in `spike/verify/src/kani.rs` (the lane driver) plus `evidence.rs`/`report.rs` wiring.

- **Resolution is toolchain-resolved, never string-matched against source.** `cargo kani list`
  enumerates the harnesses that EXIST; a `#[kani::proof]` commented out is exactly the case a
  grep cannot see. The list's table is parsed structurally, and an EMPTY list is a refusal
  rather than "nothing resolves" — a rotted parser would otherwise answer "that harness does
  not exist" for every law in the catalogue, which reads exactly like a deleted harness.
- **Citations resolve by FUNCTION name**, projected from the qualified path, so moving a harness
  between modules is a refactor rather than a silent unpinning. Kani's `--exact` filter needs
  the qualified spelling, so both live in the driver.
- **`evidence::Tier` grew from `WithLean{bool}` to `WithEngines{lean_built: Option<bool>, kani:
  Option<&Report>}`.** The `Option`s are load-bearing: the lanes are independently opt-in, so a
  Lean-only run must answer `NotAtThisTier` for `pinned` rather than `absent`, or every law's
  Kani pin would read as missing.
- **Three outcomes, kept apart** (`badge::Evidence`): no paired harness · the harness does not
  resolve (a citation pointing at nothing — rot) · the harness resolved and did not verify (a
  finding about the code). Collapsing the last two would make a deleted harness read exactly
  like a broken law.
- **No catalogue row cites a harness yet**, so no `pinned` expectation changed and the cheap
  gate is unaffected. All three laws are unwritten stubs; `evidence::one` short-circuits on that
  before reaching any badge. The machinery is pinned by unit tests instead
  (`kani.rs::tests`) — list parsing against Kani 0.67's real bytes, name projection, and the
  over-budget/green/failed trichotomy. The first real pairing lands when a law is authored, and
  that is a spec-side promote (`301:law-spec-touch-frontier-human-only`).

`report --with-kani` recomputes the badge and refuses a mismatch in either direction; the
committed `REPORT.md` stays the CHEAP-tier render (`304:tc-report-is-cheap-tier-only` unchanged).

## §4 — The census double-count, fixed

`304:fnd-axiom-census-double-counts`. `pipeline::census` walked every `*.lean` under
`minispec/Generated/`, which is both `FunsExternal_Template.lean` and the byte-identical
`FunsExternal.lean` that `materialize` copies from it — so every axiom was counted twice.

Fix: the census skips the external templates, keyed off the same `EXTERNAL_TEMPLATES` table
`materialize` copies from, so a renamed template stops being skipped and stops being copied in
one edit rather than one without the other. The Lean build imports the materialized copy alone,
so the materialized tree is what the trusted base IS.

**`minispec/REPORT.md`: external axioms 26 → 13**, exactly the real unique count `300` §2 named.
Proof holes stay 0 (unaffected — that count is per-file-with-a-hole and there are none). The
re-baseline went through the sanctioned cheap-tier path, `mise run verify:report -- --write`;
it is a one-line diff and the only deliberate golden change in this lane. Pinned by
`pipeline::tests::the_census_counts_the_materialized_tree_and_not_its_templates`.

## §5 — Counterexamples: NONE — and the near-miss that would have invented three

**No harness produced a counterexample.** Nothing in `core::sorted`, `core::coord` or
`analysis::lattice` was refuted at any bound that finished.

That sentence was nearly wrong, and how it was nearly wrong is the most useful thing in this
report. The first full battery reported **three FAILED harnesses** — the three `compare_*` rows
— and I was one step from writing them up as findings against the coordinate algebra. They are
not. Running one by hand with full output showed:

```
Runtime Convert SSA: 4.6953s
Out of memory

CBMC failed with status 6
VERIFICATION:- FAILED
```

**CBMC prints `VERIFICATION:- FAILED` on its own way down after exhausting the address-space
cap.** My driver read the verdict string before checking the gates, so every memory trip was
being classified as a refuted law. Fixed (`49dd1bca`): the gate check now runs FIRST, because an
out-of-memory run has no verdict whatever it printed; `CBMC failed with status` joined the
recognized vocabulary; and a regression test carries the real bytes above so the ordering cannot
silently invert again. Re-running the three under the fixed driver reclassifies all three to
OVER-BUDGET, which is why the headline reads 0 failed.

Worth stating as a lesson rather than a fix: a verification lane whose failure classifier is
wrong in the pessimistic direction is not "conservative" — it manufactures findings, and a
manufactured counterexample against a correctness-critical algebra is exactly the kind of thing
that gets acted on. The law is `law-never-weaken-the-question`; the symmetric obligation is
never to *invent* the question either.

## §6 — Toolchain shape, and every global-state disclosure

- **Pin:** `"cargo:kani-verifier" = { version = "0.67.0", os = ["linux"] }` in the ROOT
  `mise.toml`, beside the elan pin and for the same reason: it is ADDITIVE (a small downloader
  shim providing `cargo-kani`) and shadows no toolchain, so it roots rather than nesting the way
  `spike/verify/aeneas/mise.toml`'s rustc-shadowing pin must. `os` gates it off Windows.
  **Conductor: this is the one placement judgment worth a second look** — the brief said
  "additive pins may root", and I read this as additive; nesting it would also work.
- **Tasks:** `verify:kani` (the lane) and `verify:kani-setup` (the one-time engine fetch). Both
  route through `dorc-verify`, so the Windows refusal is one polite line from Rust rather than a
  `run_windows` branch, and `task-bodies-are-shell-free` is untouched.
- **Kani's own homes, the pre-authorized exception, used:** `~/.kani/kani-0.67.0/` (~500 MB:
  CBMC, kani-compiler, kissat). No env redirect is offered.
- **DISCLOSURE, the one that most deserves a ruling:** Kani's first-time setup ran IMPLICITLY on
  the first `cargo-kani --help` and, as its step 3/5, ran `rustup toolchain install
  nightly-2025-11-21-x86_64-unknown-linux-gnu` into `~/.rustup`. I did not invoke rustup and did
  not get to decide before it happened. That is user-global state outside the worktree and
  outside mise, and mise's `core:rust` plugin manages the same rustup home. Additive and
  reversible: `rustup toolchain uninstall nightly-2025-11-21-x86_64-unknown-linux-gnu`. I read
  it as inside "the `cargo kani setup` artifacts"; confirm rather than take it from me.
- **Also mutated:** `mise install cargo:kani-verifier@0.67.0` → `~/.local/share/mise/installs/`;
  `mise trust` on the worktree and on `spike/verify/aeneas` from the WSL side; build caches
  `~/.cache/dorc-kani-target` and `~/.cache/dorc-wsl-target-kani`. Three probe caches created
  during calibration (`dorc-kani-probe`, `-probe2`, `-probe3`) were DELETED.
- **rustup itself was NOT installed** — mise's `core:rust` had already put it at
  `~/.cargo/bin/rustup`.
- Nothing touched the shared checkout; every git operation ran in this worktree; no pushes; no
  system packages, no system config.

### The MSRV wall — why the harness crate is detached rather than an ordinary member

Kani 0.67 compiles with rustc **1.93-nightly**; the workspace declares `rust-version = "1.96"`;
cargo refuses to build any package whose MSRV exceeds the active compiler. Every arrangement
that keeps the algebra crates as DEPENDENCIES hits this — their `rust-version.workspace = true`
resolves against `spike/Cargo.toml` however the dependent manifest is written. Measured, not
guessed: `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=allow` does NOT lift it; `cargo build
--ignore-rust-version` DOES, but `cargo-kani` exposes no cargo-flag passthrough.

The two ways through are lowering a product MSRV declaration to suit a tool, or having no
package in the build graph but the harness crate. I took the second, which is also the shape the
derived-definitions lane independently arrived at — so the repo now has ONE verification-unit
pattern rather than two. `extern crate self as dorc_core` / `as dorc_analysis` plus
`pub use core_included::*` is what lets the included modules' own `crate::` paths resolve
unedited.

**Costs, stated because they are real:** nothing in `mise run check` or `cargo build
--workspace` compiles the harness crate, so only the opt-in lane can catch it rotting against a
signature change; and the lane rebuilds the included modules rather than reusing `spike/target/`.

### `#[cfg(kani)]` support code, and where it lives

Crate-local beside the types it generates (`301` §3), never in the harness crate — that is what
lets it reach private backings with no production widening. Four `kani_support` modules:
`core::sorted`, `core::coord`, `core::lib`, `analysis::lattice`. `spike/Cargo.toml`'s workspace
lints gained `unexpected_cfgs = { check-cfg = ['cfg(kani)'] }`, without which `-D warnings`
refuses the ordinary build at every one of those items.

**THE ARBITRARY LAW held** (`300` §2a): every facade value is drawn as an arbitrary `Vec` and
`kani::assume`d canonical — never built by repeated `insert`, which would make the `insert`
harnesses assume what they prove. The canonical predicate is strict ascent
(`∀i: get_at(i) < get_at(i+1)`, maps over `.0`), expressed in the facade's own walk vocabulary.

### The tractability finding — the single most useful thing this lane learned

A `Vec` whose LENGTH is symbolic and whose backing is FULL makes `insert` reallocate at a
symbolic size, and reading the result back afterwards is what a bounded model checker cannot do.
Measured on `SortedSet<u8>` at a bound of TWO members:

| shape | result |
|---|---|
| build only | 0.56 s |
| build + walk (no mutation) | 0.80 s |
| `insert`, no read-back | 1.42 s |
| raw `std::Vec::insert` + read-back | 1.10 s |
| **symbolic length + full backing + `insert` + read-back** | **21 min, 3.6 GB, "CBMC ran out of memory"** |
| symbolic length + spare capacity assumed | **2.21 s** |
| concrete length + full backing | **0.66 s** |

Neither a smaller element domain nor a tighter unwind bound nor `kissat` nor `--no-slice-formula`
moved it; `kani::assume(items.capacity() > items.len())` moved it by four orders of magnitude.

So the generators are: `any_canonical<N>()` (symbolic length ≤ N, spare capacity — the
workhorse) and `any_canonical_at_capacity<LEN>()` (exactly LEN, full backing — the value a
growing mutation must reallocate away from, covered by its own harness so the growth path is not
skipped). Capacity is invisible through the facade — no accessor exposes it, equality compares
contents — so which generator a harness draws from cannot change the answer to any question
asked here; it changes only whether `alloc` moves the buffer on the way. That reasoning is
written into `core::sorted`'s `kani_support` doc so the next person does not re-derive it.

### The shaping rule the battery then measured — read this before writing another harness

The battery's own results sharpen the finding into a rule, and the cleanest evidence is a
matched pair of harnesses over the SAME law:

| harness | value shape | result |
|---|---|---|
| `set_insert_preserves_canonical_form` | symbolic length ≤ **3**, spare capacity | **over-budget** |
| `set_insert_preserves_canonical_form_when_the_backing_moves` | **concrete** length 2, full backing | **green, 2.0 s** |

Symbolic length is what costs — but NOT uniformly, and the battery's spread says exactly where.
Sorting §1's results by shape gives a two-clause rule, and both clauses have clean counter-
examples on the other side:

- **A GROWING mutation at symbolic length is unaffordable.** `insert` blows the budget on both
  facades at bound 3; `remove` at the same bound and the same symbolic length is green
  (`set_remove_preserves_canonical_form` 5.3 s, `map_remove_keeps_key_order` 6.0 s). Shrinking
  memmoves down over a known-smaller range and never reallocates; growing does neither.
- **TWO symbolic-length collections in one harness is unaffordable, even read-only.**
  `set_structural_eq_is_set_eq` and `map_structural_eq_is_binding_eq` mutate nothing and still
  blow the budget, because each cross-lookup walks one symbolic-length backing per element of
  another. One symbolic-length collection, read-only, is comfortable at bound 3
  (`set_membership_agrees_with_the_walk` 3.3 s).

Everything green is on the affordable side of both clauses; everything over-budget trips at
least one. So the shaping a successor should apply is **concrete lengths, one harness per
length (or per length-pair)** — not a bigger budget, and not a smaller element domain (measured:
a four-value element domain moved nothing). Concrete-length harnesses are also a HONESTER
statement — each declares the exact size it verifies rather than a bound — at the cost of N+1
harnesses per unary law and more for binary ones. That trade is real and belongs to whoever
picks it up; this lane records the measurement rather than pre-empting the choice.

## §7 — Two environment deaths, and the gates that now prevent a third

Both were the same failure: an unattended CBMC eating the WSL VM. The first was measured at
3.6 GB / 21 min; the second took a 15 GiB VM down. **My lane caused them** — I was running
Kani battery work both times.

Two aggravating facts worth carrying forward:

- **Stopping a background task does not kill CBMC.** It is a grandchild of `cargo-kani`; killing
  the task leaves it running, and a survivor competes with the next harness for the same memory.
  I confirmed this directly once, via `pgrep -af cbmc` after a stop.
- **`pkill -f cbmc` is wrong.** `-f` matches whole command lines, including the driver's own. I
  used it once early, over-broadly; every cleanup since is `pkill -9 -x cbmc`, exact-name only.

The discipline is now IN THE DRIVER rather than in anyone's habits (`spike/verify/src/kani.rs`):

1. **One harness at a time.** `verify:kani` never issues a bare battery invocation.
2. **Two gates per harness**, applied by a shell wrapper because an address-space cap must be set
   on the process that allocates and CBMC is a grandchild — capping the driver would cap the
   wrong process. The wrapper is
   `sh -c 'ulimit -v 6000000; exec timeout -k 10 120 cargo-kani --harness … --exact …'`.
3. **A reaper between every harness**, success or kill: `pkill -9 -x cbmc`, exact-name only.
4. **A belt-and-braces poll** past the shell's deadline, so a missing or wedged `timeout` still
   ends the harness rather than the machine.
5. **Over-budget is a THIRD outcome**, never folded into green or failed. A harness killed at a
   gate has proved nothing AND refuted nothing: calling it failed announces a counterexample
   nobody found, calling it green pins a law on a run that never finished. It is a FINDING — the
   formula needs a shape the checker can afford — and the driver says so in those words.

Gate trips are recognized by exit 124/137 (timeout and its follow-up KILL) or by allocation-
failure vocabulary, because CBMC, the allocator and Kani's driver each word an out-of-memory
differently and none of them agree on an exit code. A no-verdict run that tripped NEITHER gate
is reported as a broken run, not as over-budget — hiding a real breakage behind a resource
excuse is the failure mode that would make this whole lane untrustworthy.

Exit codes are a trichotomy: 0 all green · 1 a finding (counterexample or over-budget) · 2 the
lane could not run (wrong platform, toolchain absent). An absent toolchain is never a silent
pass.

## §8 — Gate evidence (exactly what was run)

- **`mise run gate:full-quiet`, Windows leg, foreground, GREEN — re-run at the FINAL commit.**
  `1951 tests run: 1951 passed (1 slow), 1 skipped`, plus the four lint gates, `clippy:clean`
  from a wiped target dir, a fresh `cargo build --workspace`, and all doctests. This is the
  load-bearing one: it proves the DEFAULT suite is green with NO Kani present, which is the
  lane's opt-in claim. (An earlier green run at 1950 tests predated the last two commits; this
  is the one that covers the whole branch.)
- **`mise run check`** run before every commit; the pre-commit hook (check-only, `HK_FIX=0`)
  ran on all of them and refused nothing.
- **The WSL leg of `gate:full-quiet` was NOT run**, under the conductor's explicit allowance.
  Justification, stated so it can be checked: everything this lane adds to compiled product
  code is either `#[cfg(kani)]` (invisible to any ordinary build on either platform), test-only
  (`analysis::effect`, `plan`), or inside `dorc-verify`, whose only platform-conditional code is
  a `cfg!(windows)` refusal — and the Linux side of that refusal is exercised every time the
  lane runs, which it did, repeatedly. No `#[cfg(unix)]`/`#[cfg(windows)]` region was added,
  so `one-platform-green-is-not-cross-platform-green`'s specific hazard does not apply here.
- **`mise run verify:kani` on Windows**: refuses in one line — `the Kani lane is Linux/WSL only
  (upstream publishes no Windows asset); run it from the WSL leg` — exit 2. Verified by running
  it, not by reading the code.
- **`mise run bless:dry` was NOT run.** The one golden this lane touches is
  `minispec/REPORT.md`, which is not a bless-path artifact at all: it is republished by
  `verify:report -- --write` and gate-checked by `verify:check`, both of which ran green inside
  `gate:full-quiet` (the hk `minispec` step and the `corpus_is_coherent` tests). No
  `crates/*/tests/` golden changed, so there is nothing for a bless to re-derive.

## §9 — Commits, in order

Branch `ai/r30-lane-kani`, based on `5e6d6788`. Left as-based — the conductor rebases at fold.

```
d2fc198b (AI fix) Count the trusted base once, not once per emitted copy
b9d66002 (AI new ana) Give the algebra tier hand-written harness generators
505860bc (AI new tool) Stand up the bounded-verification lane and its harness battery
9b86e7c4 (AI new tool) Close the harness-pairing seam with real toolchain evidence
a7630500 (AI fix tool) Drive Kani by its own fully-qualified harness names
a915cf11 (AI new tool) Gate every harness on address space and wall-clock, in the driver
237c9f23 (AI new test ana) Pin the cause-excluding Reach equality and its lattice laws
8a952dcf (AI new test) Pin span-edit survivors as sorted and byte-disjoint
60b4b33f (AI doc) Bank the kani-lane report against another harness death
56fa8d44 (AI new tool) Print each harness verdict as it lands, not at the end
49dd1bca (AI fix tool) Read an out-of-memory kill as a gate trip, never as a counterexample
```

The last two are worth a reviewer's eye: `56fa8d44` exists because a battery killed partway
through used to lose every verdict it had already earned, and `49dd1bca` is §5's near-miss.

## §10 — For the successor: what is NOT done

- **No catalogue row cites a harness.** The `pinned` badge machinery is closed and unit-pinned,
  but no law is authored yet, so nothing exercises it end-to-end against the real catalogue. The
  first authored law is where that closes, and promoting the citation is a spec-side act.
- **`Reach` and `normalise_edits` are at the lower rung by necessity, not by preference.** Both
  carry the flag in-source. `Reach`'s move is unblocked the moment it sits on the facade.
- **The harness crate is invisible to `mise run check`.** A signature change in `core`/`analysis`
  breaks it only under the opt-in lane. If that bites, the cheapest fix is a `cargo check` of the
  detached manifest in the lane task — it does NOT need Kani, only the 1.93 toolchain, so it
  would still be Linux-only.
- **`spike/CLAUDE.md`'s Build/test/run block does not list `verify:kani*`.** `300` §2 already
  owes it the `verify:*` rows at discipline-close; these two join that list.
- **18 of 37 harnesses are unjudged.** §1 says which and why, and the shaping rule says what to
  do (concrete lengths, one harness per length or length-pair). This is the largest single piece
  of remaining work, and it is mechanical rather than design-shaped.
- **`Dialect::any_minted` is the worst-behaved generator in the lane.** It builds through real
  `mint` calls — deliberately, because a dialect IS what minting produced, and an arbitrary
  `SortedMap` would admit dialects no run can reach. But `mint` goes `SortedSet::singleton`
  (capacity 1, FULL) then `insert`, which is exactly the pathological shape, and it takes all
  three `compare_*` harnesses down with it. Whoever reshapes should weigh the faithfulness the
  current generator buys against a hand-built canonical dialect with an assumed shape; I would
  not trade it away silently, which is why it is here rather than already changed.
- **The 25 s budget used for the recorded run is not the code's default.** `DEFAULT_BUDGET_SECS`
  is 120; the run set `DORC_KANI_HARNESS_BUDGET_SECS=25` because every harness that verifies at
  all does so inside 6 s, so 25 s is four times the slowest green. That is a resource gate, not
  a proof bound — lowering it can only move a harness from judged to unjudged, never the
  reverse — but a successor re-running at 120 s may find a wall-clock trip or two flip green.
  The three address-space trips will not.
