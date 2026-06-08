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

### strain-5 — the poison wall is killed *specifically*, not *generally*: a realistic book needs broad oracle coverage to elide *anything* (K2, the headline e2e finding)

+SURE (built + tested + isolated by fragment, `plan::residual_poison_sources_isolated`): the keystone
re-key kills `apt-get update`'s poison **exactly** — proven at classify level
(`effect::poison_wall_dies_modeled_update_does_not_poison_install`) *and* now at plan level
(`plan::fixture_install_elides_when_update_is_the_only_neighbour`: on `update\ninstall`, the converged install
is `Disposition::Replace`). But on the **full** `fixtures/pi-webhost.book.sh`, the install **still runs** — and
so does `update` itself (both classify `EstablishWritten`). This is the honest outcome the charter asked for
(model enough to genuinely elide, OR document the precise residual; **not** a faked green): I documented the
residual rather than over-modeling, because the residual is the *interesting datum*.

The exclusion-check (isolated each upstream construct as a fragment, classifying `apt-get install`'s
`SkipClass`):
- `update\ninstall` (no poison) ⇒ both **Ambient** (the clean keystone win).
- `set -e\nupdate\ninstall` ⇒ both **Ambient** — `set -e` is a target-state-pure builtin (the fs-4 allowlist
  fix), correctly *not* a poison source. (Good: confirms the builtin-bless survived the re-key.)
- `case "$(hostname)" in …` upstream ⇒ both **Written**. The `$(hostname)` command-substitution is an
  un-oracled `Command` ⇒ `Opaque` ⇒ `Reach::Top`.
- `if ! command -v nginx …` guard upstream ⇒ both **Written**. The guard's `command -v nginx` is likewise
  un-oracled `Opaque` ⇒ `Top`.

So the real book carries **two independent residual poisons**, and modeling `update` was *necessary but not
sufficient*. The sharpest part of the finding (~SUSPECT this is the durable lesson): the `command -v nginx`
guard is the admin's **own idempotency check** — the very idiom Dorc exists to lift — and because it is
un-oracled it *poisons the block it guards*. That is the `seam-prov`/oracle-coverage reality: on a scrappy
book, *most* leaves are `Opaque`, and a single upstream `Opaque` re-tops `Reach` for everything after it, so
the per-cell precision the re-key buys is only *visible* once the upstream neighbours are also oracled (or
proven pure). The keystone removes a *specific* false-poison; it does not, alone, make a realistic book elide.
The `$(hostname)` and `command -v` cases point at the *next* coverage work: a host-identity oracle
(`an-host-identity-fact`) and a `command -v`/tool-existence oracle (the `R2-SHADOW` blessed form) would each
un-`Opaque` one of these. Neither is K2 scope; recorded as the measured oracle-coverage gap.

(Process note, `ch-wrong`: the old `fixture_install_runs_despite_converged_probe` test *asserted* the poison;
I renamed it to `fixture_install_on_realistic_book_still_runs_residual_poison` and re-grounded its assertion in
the **new** cause — the test still asserts `Run`, but now for the residual-neighbour reason, with the
`update`-specific poison dead. The charter's "FLIP to Replace" was the *hoped* outcome; the honest outcome is
"flips on the isolated sequence, stays Run on the full book for a different correct reason." Both tests exist so
the distinction can't silently rot.)

### strain-6 — `ap-2`: the `sh -n` gate FIRED (as predicted); the honest fix is the `:` stand-in, *not* a paper-over (K2, charter deliverable)

+SURE (built + reproduced both directions): the `ap-2` runnability gate **fires on the `guarded` fixture**.
The committed `render_apply` comments an elided line *in place*; when the elided leaf is the lone body of a
`then`-clause (`if true; then\n  apt-get install …\nfi`), commenting it yields `if true; then\n# …\nfi` — an
empty clause, a POSIX syntax error. Both `dash -n` and `sh -n` reject it: `Syntax error: "fi" unexpected`
(exit 2). The old e2e *string-diffed* this and shipped it **green** — exactly the `ap-2` defect, live and
committed in `cases/guarded/expected.out`. **This firing IS the deliverable** (charter §4 / `ap-2`).

The fix (~SUSPECT this is the *honest* one, not a paper-over — argued, not reflexive): an elided line now emits
**two** lines — the provenance comment, then a bare `:` (POSIX null command) at the original indentation. Key
reframe: the `:` is **not** filler, it is the substitution **stand-in itself**. `Disposition::Replace`'s own
doc says "`true` is the degenerate stand-in," and the observable-matrix model *defines* replacement as
"substitute a `true`-stub that defaults every observable — status→rc-0 (vouched by convergence)." The
comment-only render was the actual bug: it **deleted** the command instead of substituting its stand-in. So
emitting `:` is *more* faithful to the model, not less — and `:` is valid in **every** context a command was
(top-level, `then`/`do`/`case`-arm), so the artifact is always `-n`-clean by construction. Cost: a top-level
lone elision gets a cosmetically-extra `:` line (harmless `true`-equivalent); the price of staying
*line-granular* without parsing clause structure. The leaf-exact alternative (only inject `:` when a clause
would actually empty) is the `seam-prov` render-fidelity work `plan/CLAUDE.md` flags — deferred, not needed for
correctness.

The gate itself (harness half, `cli` owns it — landed in `e2e/run.sh`): splits the cli's stdout into the two
emitted artifacts on their `#!/bin/sh` shebangs (probe = first block, apply = second) and `dash -n`/`sh -n`-
checks **each**, **always**, and **before** `BLESS` (blessing a non-runnable artifact is the trap). The text
golden-diff stays as a *secondary* check (it catches wrong-elision *content*, to which `-n` is blind — a render
that comments everything is `-n`-clean and useless; per `cli/CLAUDE.md`'s "needs both" tension). Verified
**non-vacuous**: the pre-fix comment-only artifact fed to the same checker fires (exit 2); the fixed `:`-stub
render passes (exit 0). `sh -n` never executes, so the fixtures' real-looking `apt-get`/`systemctl` lines are
safe (no `hork`/`wombat` stubbing needed — nothing runs).

### strain-7 — `fact_label`/cli-stdin format: `kind:entity#selector` (Operand) · `kind#selector` (Singleton) — resolves strain-4's --WONDER (K2)

+SURE (built + round-trips e2e): the strain-4 display-format question is **decided**. `fact_label` renders two
shapes, discriminated by the presence of a `:`-operand segment:
- **Operand** ⇒ `kind:entity#selector` — `package:nginx#installed`;
- **Singleton** ⇒ `kind#selector` — `package-index#fresh` (NO bare `:`, so the ugly `package-index:#fresh`
  strain-4 warned against never appears; `:` present ⟺ an operand exists).
The selector is **always** rendered (`#selector`). Dropping it would let an `is-active` probe-verdict discharge
an unmet `#enabled` cell — a wrong-elision under apply's `kFAIL` (`cli/CLAUDE.md` "stdin re-key gotcha"). The
cli's stdin grammar *is* this label: `parse_results` keys a `BTreeMap<String, Verdict>` on the **exact** label
string (it never decomposes kind/entity/selector — `inv-referent-agnostic`-clean, it's a string-equality match,
never a decode), so the round-trip is exact-string and the only requirement is **injectivity** over distinct
`FactKey`s. Injective modulo a `:`/`#` collision *inside* an interned name — a disposable-parser limitation
(`ch-scope`; book operands like `nginx` don't carry `:`/`#`; arch-qualified `nginx:amd64` would be the first
real collision, noted, not handled). Verified live: the probe emits `# probe: package:nginx#installed`, the
host echoes `package:nginx#installed converged`, and the verdict binds — the selector is carried end-to-end,
never widened to the whole entity. The `two-oracles` e2e case exercises both a `package:nginx#installed` and a
`service:nginx#enabled` label in one round-trip.

(Adjacent datum surfaced, NOT K2's to fix — strain-2 / `F-BLESSED` re-confirmed: the `service` oracle's probe
body is `systemctl is-active` (`#active`) while `enable` gates `#enabled` — the probe reads the **wrong
selector**. A real `service` oracle needs *two* probes (`is-enabled` AND `is-active`); the scrappy fixture
under-probes. The e2e only `-n`-checks (never executes) the body, so it doesn't bite here, but it is the live
≥enum-floor cliff: one `FactProbe` per *kind* can't answer per-*selector* facts. The probe-per-kind vs
fact-per-selector mismatch is a richness-phase item — flagged for K3/later, not threaded in K2.)
