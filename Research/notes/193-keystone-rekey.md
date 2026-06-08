# 193 — Spike-2 keystone: the entity-algebra re-key (design-intent + strain-log)

> Round-19's first notation doc (192 is quarantined). Append-only — new `## strain-N`
> subsections as the build surfaces them; do not edit prior content in place. The
> *deliverable* is the strain-log (charter §0/§7), not green tests. AI-authored,
> confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust root
> `DESIGN`/`KNOBS`/`README`/`IMPLEMENTATION`/`AGENTS` and `plans/191` over this.

## 0. Working location (round-19 setup)

`.claude/worktrees/round19` on branch `ai/round19-keystone`, forked off `main` @ `357efdd`
(which carries the round-19 fork + the 191 charter; verified green there: `mise exec --
cargo test --workspace`, 0 failed). `main` is untouched; autonomous-commit on the branch;
**never pushed.** Root human docs (`DESIGN`/`KNOBS`/`AGENTS`/`TODO`/`IMPLEMENTATION`/root
`CLAUDE.md`) are watched at the *root* worktree, edited only by the human. Per-crate
`spike/crates/*/CLAUDE.md` are agent-authored and tunable toward the keystone (human auth,
2026-06-08) — I'll true them up to the landed reality as I go.

## 1. The keystone, restated (what K1 builds)

+SURE (traced in source): nothing elides on `fixtures/pi-webhost.book.sh` because the flat
`effect::FactKey { kind, entity }` is one bit per pair, and `apt-get update` is **doubly**
un-keyable — no oracle verb *and* no operand — so it is `Opaque ⇒ Reach::Top` and poisons the
`apt-get install nginx` below it to `EstablishWritten` (the passing test
`fixture_install_runs_despite_converged_probe`). SPA §12.5 names the class precisely: **join is
always sound *and* complete** — merge-points never lose precision — so this is a *lattice-shape*
defect (too few cells), not a join defect. The fix is more cells.

The minimal re-key that kills it:
- a **selector** facet on the key (`17N inc-S` ≥enum; `an-per-entity-selector`): `service#enabled`
  and `#active` are *independently* mutation-gating (`enable --now` writes both; an `is-active`
  probe must not discharge an unmet `#enabled`), which a flat key cannot hold;
- an **entity that admits a singleton** (`an-host-identity-fact`-adjacent): `apt-get update` is a
  *nullary* mutator on the one package index — no operand — so the key must carry `Singleton`, not
  require an `OpaqueToken`.

Result: `update → (package-index, Singleton, fresh)`, `install nginx → (package, Operand(nginx),
installed)`, `systemctl enable nginx → (service, Operand(nginx), enabled)`, `systemctl start nginx
→ (service, Operand(nginx), active)`. Distinct cells ⇒ no cross-poison and no false discharge.

## 2. Decisions baked (charter `ch-wrong` owns being wrong; record where they hurt)

- **dec-seam-ownership → `core`.** The structured algebra is *defined in `core`* as the shared
  vocabulary every crate agrees on first (`dac-B`); `analysis::effect::FactKey` *becomes* this
  `core` type (re-export / `pub use`), not a parallel key. Closes the open question in
  `core/CLAUDE.md`'s tension section — the current decorative-`core::Fact` / canonical-flat-
  `effect::FactKey` split *is* the two-diverging-graphs failure `dac-B` warns against; collapse it.
  (`core::Fact`/`FactDomain` are unused-decorative today — replace, don't preserve.)
- **dec-shape → minimal selector re-key first; recursion is first-to-give.** Build
  `FactKey{kind, entity:EntityRef, selector}` (no recursion, no typestate transition table, no
  cardinality) → kill the poison wall → executable acceptance, *before* any richer machinery
  (`ap-1`). Typestate transitions (`inc-7`), occurrence-typing (`inc-6`), strong/weak
  (`an-strong-weak-update`), and the recursive kind-embedding (`ch-entity-algebra`) layer on top as
  separate phases.
- **dec-cardinality-deferred → T5.** The `{Singleton,Multiple}` strong-update gate is *not* in the
  K1 key — strong/weak is a later phase. ~SUSPECT it lands as an effect/transfer annotation, **not**
  a `FactKey` field (cardinality is a property of the *occurrence*, not cell *identity* — two
  effects on the same cell with different cardinality are still the same cell). Recorded so K1
  doesn't bake it into the key.
- **flag-1 → an early recursion smoke-test (T6).** `seam-finite`'s recursion-*height* hang-risk
  lives only in `ch-entity-algebra` (the first-to-give richness), so a minimal depth-bounded nesting
  case rides adjacent to the keystone — `seam-finite` gets height-termination evidence even if the
  richness later gives. (The strong-update *monotonicity* half of `seam-finite` is separate and
  cheap — see §5.)

## 3. The exact `core` type (K1 implements this verbatim; the shape is the TL-agent's, `§5b`)

```rust
/// One independently-mutation-gating facet of a kind's ≥enum state-model
/// (17N inc-S / an-per-entity-selector). An interned name; never decoded
/// (inv-referent-agnostic) — compared for co-reference, resolved for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SelectorId(pub Symbol);

/// The cell a fact is about: an operand-named cell, or the kind's implicit
/// singleton. `apt-get update` is a nullary mutator on the one package index
/// (no operand) — the old flat key required an OpaqueToken, so a no-operand
/// mutator fell through to Opaque ⇒ Reach::Top ⇒ the poison wall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityRef {
    Operand(OpaqueToken),
    Singleton,
}

/// A system-state fact-key, re-keyed for spike-2 (charter §3 / 16Q §1). The flat
/// (kind, entity) pair gains a selector — the cell coordinate the whole engine
/// reaches over. Carries NO source span (provenance is the node's). Two keys are
/// equal iff kind+entity+selector match.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FactKey {
    pub kind: KindId,
    pub entity: EntityRef,
    pub selector: SelectorId,
}
```

`Copy`/`Ord`/`Hash` preserved (current `FactKey` is `Copy`; `Reach`'s `BTreeSet<FactKey>` needs
`Ord`) — `EntityRef`/`SelectorId` are `Copy`+`Ord`, so the bound holds. `inv-determinism`: any
map/set keyed on `FactKey` stays `BTree*`, never hashed-into-output.

## 4. Propagation contract (what each crate threads; K1 = core+oracle+analysis)

- **`oracle`** — `KindIndex::effect_of(provider, verb)` returns `(KindId, SelectorId, Polarity)`
  (was `(KindId, Polarity)`). The `oracle_effect` lift grows a selector. **The exact sh spelling is
  oracle's to choose** (it owns the `kTYANNOT`/`ch-shape-anno` experiment) — a 4th token
  (`oracle_effect systemctl enable establish enabled`) is the obvious strawman; if it reads
  awkward, that friction is a `notes/193` strain, not a blocker. Whether `Singleton`-ness is
  oracle-declared or analyzer-inferred-on-zero-operands (see below) is also oracle+analysis's call —
  flag the choice.
- **`analysis::effect`** — `command_effect` builds the richer key: `EntityRef::Operand(tok)` from a
  single literal operand (as today), `EntityRef::Singleton` when a *modeled* `(provider,verb)` has
  **zero** operands (the `update` case); `selector` from the effect-map. `Reach`/`Reach::with`/
  `Reach::mutated`/`classify`/`SkipClass` are *mostly mechanical* (richer key, same set-shaped
  dataflow) — `Reach` stays a `BTreeSet<FactKey>`-over-⊤ for K1 (the `MapL<entity,StateLattice>`
  typestate move is T4, not now). The poison dies because `update`'s key ≠ `install`'s key.
- **`plan`** (K2) — `prove_replaceable` destructures `SkipClass::EstablishAmbient(fact)` and the
  predicate is per-fact; it only *refines* per-selector. ~SUSPECT the witness shape survives
  untouched (keys on `FactKey`+grade+verdict). `fact_label` resolves the richer key for display.
- **`hostsim`/`cli`** (K2) — the host fact-store keys on the richer `FactKey`; `cli`'s
  `parse_results` matches probe lines by `fact_label`.

## 5. `tc-*` guardrails — subagents FLAG these UP, never resolve locally (`§5b`, `inv-superposition`)

A single-crate worker lacks the phase/user/orientation context to settle these; a wrong orientation
is a *silent* wrong-elision. Emit the fact, default to the safe side, leave a `// TODO(tc-…): …`,
and report it up to the TL-agent:
- `tc-uniqueness` — strong (overwrite) vs weak (accumulate) update. **Not in K1** (no strong-update
  yet); when T5 builds it, the literal⇒singleton heuristic is *mine* to bless.
- `tc-collapse` / `tc-mint` / `tc-phase` — any `May`/`Must` collapse, `ReplaceLicense` mint, or phase
  default. K1 changes none of these (it re-keys; the locks stay where they are).
- `tc-taint` — proof-level vs oracle/human-tainted facts (`Grounded<T>`, unbuilt) — not K1.
- The standing **exclusion-check** (AGENTS): before excluding any case, re-test under reverse-
  direction · other-phase · other-user · other-reliability.

## 6. Theory anchors (why this shape — SPA, read in full this round)

- **§12.5** join is sound+complete ⇒ the poison is lattice-shape (cells), fixable by the re-key, not
  a merge problem. **§4.3 / Ex 4.26** map-update on a *singleton* var is monotone ⇒ the eventual
  strong-update-on-singleton transfer is monotone on a fixed abstraction (so it won't hang — the
  `seam-finite` monotonicity half is cheap; `notes/180 fnd-3` concurs the non-monotonicity scare is
  *meta*-level). **§11.5** strong-update sound only on a singleton cell ⇒ `tc-uniqueness`/`an-entity-
  uniqueness`. **§9.4** IFDS needs finite-`D` + *distributive* transfers; strong-update + Andersen
  (Ex 11.6) are non-distributive ⇒ `an-distributive-split` is non-distributive ⇒ the hand-rolled
  worklist is *correct*, not a compromise (the `re-eval-trigger`: IFDS reopens only if a seam forces
  transfers distributive over finite-`D`). **§6** widening = the `seam-finite` recursion-height tool
  (depth-bound = simple-widening to a finite-height image).

## 7. Acceptance for K1 (executable, not paper)

1. Workspace green (`mise exec -- cargo test --workspace`).
2. `fixture_install_runs_despite_converged_probe` is **updated** to assert the install is now
   `Disposition::Replace` on a `Converged` host (the poison is dead) — and the oracle now models
   `apt-get update` on a distinct cell. Keep a *new* regression that a genuine upstream same-cell
   kill (`purge nginx; install nginx`) still forces `EstablishWritten`/Run (the ambient gate must
   not be over-loosened — exclusion-check the fix).
3. A selector regression: `systemctl enable nginx; systemctl start nginx` keeps both as distinct
   establishes (neither discharges the other) — the `#enabled`≠`#active` distinction live.
4. Runnable fixtures (if any are *executed*) use non-functional stubs (`hork`/`wombat`); the rendered
   apply artifact is `sh -n`-checked (K2's harness), never string-diffed.

## strain-log
*(grows as the build surfaces friction — the round-19 product)*

### strain-1 — singleton-ness: inferred-on-zero-operands, and where it over-loosens (`tc-uniqueness`-adjacent, `ch-wrong`)

+SURE (built + tested, `effect::command_effect`): I made `Singleton`-ness **analyzer-inferred
on zero operands**, *not* oracle-declared. A modeled `(provider,verb)` with zero literal
non-flag operands ⇒ `EntityRef::Singleton`; one operand ⇒ `Operand(tok)`; >1 ⇒ ⊤ (unchanged).
This is the minimal `dec-shape` (no cardinality token in the K1 key).

The strain (where it bakes wrong, `ch-wrong`): the inference can't distinguish "nullary mutator
on a genuine singleton" (`apt-get update` → `package-index#fresh`, correct) from "operand-verb
mis-called with no operand" (`apt-get install` alone → now mis-modeled `Singleton(package#installed)`
instead of `Opaque`). ~SUSPECT this is a *latent over-loosening*: if that bogus singleton cell
were probed Converged, the empty-arg install could be wrongly elided. It is low-severity in
practice (an arg-less `install` is itself a book error, and rare), but it is a real soundness
seam, not cosmetic. The principled fix is an oracle-declared singleton/cardinality bit
(`dec-cardinality-deferred` / `an-entity-uniqueness`), explicitly deferred past K1. **`tc-uniqueness`
flagged up, NOT resolved** — I defaulted to the shape `notes/193` §4 specified (infer) and recorded
the cost here rather than minting a cardinality mechanism (which would be building T5 early).

Exclusion-check done: the over-loosening only manifests in `Phase::Apply` (a wrong-elide); under
`Phase::Probe` a spurious singleton just means an extra probe (harmless, `kFAIL-withhold`). Other-user:
an oracle author *could* defend this by declaring cardinality, but the lazy-admin book can't — so the
inference is the admin-facing default and the cost lands on the admin's side. That asymmetry is the
argument *for* an oracle-declared bit later (the author can pay to remove the admin's footgun).

### strain-2 — `ch-shape-anno`: the 4th-token selector spelling reads clean; the *real* friction is the absent-vs-`#installed` default (`kTYANNOT`)

+SURE (built + tested, `oracle::lift_command` + `fixtures/package.oracle.sh`): the selector sh-spelling
I chose is the obvious strawman — a 4th positional token on the marker:
`oracle_effect apt-get install establish installed`. It lifts cleanly, the fixture stays `dash`-parseable
(it's still just a marker call; the marker shim `oracle_effect(){ :; }` ignores extra args), and arity/
polarity error-handling extends mechanically (now "exactly four literal args"). **No meaningful friction
in the spelling itself** — contrary to the `ch-shape-anno`-will-strain prior. The 4-token form is *less*
off-ramp-hostile than the inline-`local w : T` annotation `ch-shape-anno` warned about, because it lives
in the oracle file (author surface), not the book, and degrades to a no-op shim.

The friction that *did* surface (~SUSPECT, design-level, hand to `kTYANNOT`): every effect now MUST name
a selector, so `install` and `purge` both spell `installed` and a single-state kind (`file:exists`?) must
invent a selector name with nothing to contrast it against. That's verbose boilerplate for the common
single-selector case. The natural ergonomic relief — "omit the selector ⇒ a default `#default`/`#_` cell"
— silently re-introduces the flat-key collision the re-key exists to kill (every un-selector'd verb shares
one cell). So the spelling is fine but the *default* is a genuine `kTYANNOT` tension: explicit-everywhere
(boilerplate) vs implicit-default (re-collides). K1 took explicit-everywhere; flag the default question up.

### strain-3 — `dec-seam-ownership` → `core` was mechanical; the re-key is *not* "most of the diff" in analysis (16Q §1 over-predicted)

+SURE (traced): defining `FactKey`/`EntityRef`/`SelectorId` in `core` and `pub use`-ing it from
`analysis::effect` was clean — `core::Fact`/`FactDomain` were genuinely decorative (zero non-test
references outside `core`), so "replace, don't preserve" cost nothing. The `dac-B` two-diverging-graphs
risk the `core`/`effect::FactKey` split warned about was real but cheap to collapse *now* (it would have
compounded had `plan`/`hostsim` grown more on the flat key first — `ap-1` vindicated).

Mild correction to `16Q §1`'s "re-keying `FactKey` propagates through nearly the whole engine / most of
the diff": in `analysis` the propagation was **almost entirely mechanical and small**. `Reach`/`Reach::with`/
`Reach::mutated`/`classify`/`SkipClass` needed *zero* body changes — they're generic over `FactKey` as an
opaque `Ord` key, so the richer key threaded for free. The *only* real logic change was the ~10-line
entity-resolution in `command_effect` (the singleton branch). The bulk of the analysis diff is **tests**
(new regressions + re-spelling fact literals), not engine logic. So the keystone's "nearly the whole engine"
weight is real but lands in the *consumers* (`plan`/`hostsim`/`cli` — all the `FactKey { kind, entity }`
literal construction sites), not in the dataflow core. That's a precision win for the substrate decision:
the set-shaped `Reach` over an opaque key absorbed a domain-refinement with no transfer-function rewrite.

### strain-4 — K1/K2 boundary: one display-fn (`fact_label`) gates the entire downstream workspace

+SURE (built): with `core`+`oracle`+`analysis` green, the workspace's *only* lib-level compile break is
`plan::fact_label` reading `fact.entity.0` (now an `EntityRef` enum, no `.0`). Everything else that breaks
is **test-side** `FactKey { kind, entity }` literal construction (mapped in the report). I deliberately left
`fact_label` broken with a `TODO(K2)` rather than fix it, because its *format* (`kind:entity#selector`?
how to render `Singleton`'s empty entity — `package-index:#fresh` is ugly) couples to `cli::parse_results`'
stdin grammar and `hostsim`'s key (per `cli/CLAUDE.md` "stdin re-key gotcha"). --WONDER whether the honest
K2 move is to make `fact_label` emit a round-trippable `kind/selector` for `Singleton` (no bare `:#`). That
display-format choice is a small but real `seam-prov`-adjacent decision; recording it so K2 doesn't bake an
ugly/ambiguous label by reflex.
