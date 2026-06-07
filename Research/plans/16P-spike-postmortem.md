# Dorc — 16P spike postmortem

> **What this is.** The durable postmortem of the round-16 implementation spike. It replaces the 25
> quarantined round-16 notes (`notes/quarantine-DO-NOT-READ/160`…`16O`) and the disposable `spike/`
> Rust workspace (`notes/quarantine-DO-NOT-READ/spike/`) as the thing a future agent reads — reach
> last-mile evidence through the citations here; do **not** pull the quarantine back in wholesale.
>
> **Citations.** A quarantined note is cited by its non-resolving slug `notes/quarantine-DO-NOT-READ/16X`
> (no `Research/`, no `.md`), so the DO-NOT-READ directive is local to every cite and no auto-loader can
> follow it. The disposable spike's code is cited by **Rust symbol path** (`plan::prove_replaceable`,
> `cfg::build`) — one realization, not a contract; the type names it uses (`Carrier<T>`,
> `ReplaceLicense`) and the exact oracle sh spelling are illustrative, never committed decisions.

---

## 1. Frame

The round-16 work was a spike: a deliberately disposable Rust workspace whose stated job (per
`notes/quarantine-DO-NOT-READ/spike/CLAUDE.md`) was to *surface design problems by building*, not to
become the shipped tool. Round 16 ran `notes/quarantine-DO-NOT-READ/160`→`16O` (25 append-only notes);
notes `000`–`128` are the earlier research rounds and are background, not the subject here.

What the spike built, in one paragraph: a *pure* analysis pipeline — `syntax → analysis → plan`, a total
function of its inputs with no clock/RNG/IO reachable, directly or transitively — over a narrow,
demand-grown POSIX-sh subset. On top of a hand-rolled monotone-dataflow engine (lattice combinators + a
generic worklist solver) it realized: an oracle-driven *effect-map* keyed by a referent-agnostic *named
kind*; a reaching-definitions *ambient gate* that licenses eliding a command only when its target fact
is provably undisturbed upstream; an *observable/replace* elision model (replace a converged command
with the cheapest stand-in that reproduces the observables its consumers read); and a forward
*probe→apply* compiler. The whole chain was wired through a CLI round-trip (`dorc --book`), fuzzed under
deterministic-simulation tests against a seeded host model, and exercised by a shell-mechanized
end-to-end harness on real `.sh` files. It compiles a probe and an eliding apply; it *executes
neither* — there is no host executor, no networking, no apply over time.

**An implementer should read §3 (the built-vs-designed ledger) first.** It is the load-bearing section:
a large fraction of what the notes discuss was *designed, deferred, or fenced off as outside-spike*, and
must not be read as a shipped decision. The findings in §4 are tagged against that ledger throughout.

The ground-truth design documents are the first-party `README.md` / `DESIGN.md` / `KNOBS.md`; trust them
over this postmortem where they conflict, and over the quarantined notes (and
`notes/quarantine-DO-NOT-READ/spike/CLAUDE.md`'s `inv-*` slugs, which are AI-authored convention)
absolutely — a trust order the spike itself had to relearn mid-stream (T11, §5).

---

## 2. Arc

A compact, neutralized timeline; each entry is a quarantined round-16 note (cited by its non-resolving
slug). "Built" = landed in spike code at that note.

- `notes/quarantine-DO-NOT-READ/160` — enumerated the analyzer design surface from the research base;
  most items build-deferred. The research→spike bridge. *Predicted a structure that later diverged:* a
  separate `probe` crate (folded into `plan`) and a "day-1" verdict-memo
  (`{verdict, content_key, freshness}`) that was never built.
- `notes/quarantine-DO-NOT-READ/161` — first oracle-contract strawman, *command-centric* (probe = a
  dry-run of the mutator, `apt-get --simulate | grep`). Superseded by
  `notes/quarantine-DO-NOT-READ/162`.
- `notes/quarantine-DO-NOT-READ/162` — pivot to a *fact-centric* contract (the one actually built): an
  oracle declares a named **kind**, a read-only **fact-probe** (three-outcome), and an **effect-map**
  `(provider, verb) → (kind, establish|kill)`. Open problems O-1..O-6 logged.
- `notes/quarantine-DO-NOT-READ/163` — engine design: a monotone dataflow framework (lattice / solve /
  cfg / effect); reaching-definitions named as the ambient gate; a direction-generic solver set as a
  requirement.
- `notes/quarantine-DO-NOT-READ/164` — first build checkpoint: core / syntax / lattice / solve /
  coarse-errexit cfg landed. The solver had no iteration cap and could **loop forever** on a
  non-monotone transfer — reproduced empirically (two runs reached 435 and 783 CPU-seconds before being
  killed) — fixed with a convergence flag + cap. The effect pass was written but dormant (uncompiled).
- `notes/quarantine-DO-NOT-READ/165` — the orientation-soundness hazard: a correct value consumed in the
  wrong soundness orientation is *silently* unsound. Introduced `May`/`Must` (order-dual),
  `PhasedVerdict`/`Bias`, the witness, and `BoundedLattice`. Human ruling: *calibrate up* — lean
  deliberately into type-locks as state-space exploration, varying lock-style across modules.
- `notes/quarantine-DO-NOT-READ/166` — the coarse "more failure-edges = safe" errexit model is unsound
  **both ways** (a missing edge is unsound forward → wrong skip; a spurious edge is unsound backward →
  wrong apply-slice). Built a *precise* errexit pass.
- `notes/quarantine-DO-NOT-READ/167` — adversarial review of the effect pass found two wrong-skip paths:
  a detached function-body establish read as ambient, and a release path trusting a non-converged solve.
  Both fixed (entry-reachability gate; a converged-trust gate).
- `notes/quarantine-DO-NOT-READ/168` — build summary: the effect→plan elision path complete end-to-end,
  network-free. Engine-wide `meet` + `May`/`Must` + the plan locks (`PhasedVerdict`/`Bias`/witness)
  landed. Human chose the *maximal* orientation machinery (engine-wide meet) as exploration.
- `notes/quarantine-DO-NOT-READ/169` — first CLI capstone; surfaced several leaf-rendering / encoding
  defects (subst-internal commands leaking as plan leaves; a UTF-8 BOM; a heredoc body excluded from a
  leaf span; an errexit ⊤ on the fixture).
- `notes/quarantine-DO-NOT-READ/16A` — direction note (planning only): the apply phase (executor,
  backward slice, re-probe / TOCTOU), multi-host (fleet, CAP failure modes), and unreliable/mutative
  oracles. §0 fixed the `notes/quarantine-DO-NOT-READ/169` errexit defect (merges now seed ⊥, not
  `Off`) — a prerequisite for a sound *backward* apply.
- `notes/quarantine-DO-NOT-READ/16B` — leaf-seam scope: `$(…)` command-substitution bodies are
  *effect-bearing non-leaves*; subshell `( )` and group `{ }` bodies *are* leaves. Built.
- `notes/quarantine-DO-NOT-READ/16C` — an adversarial pair refuted analyzer-side value-synthesis (the
  kernel has no value plane; every lock is fact-shaped). The rescue was reframed as an oracle-declared
  *bridge* — **outside-spike**. In-spike floor: an output-consumed leaf is MustRun.
- `notes/quarantine-DO-NOT-READ/16D` — the best-effort-under-degradation lens (deferred as a build, kept
  as a criterion): the heavy types prove a *conditional* and a *floor*, not ground truth. Standing
  review criterion for later design.
- `notes/quarantine-DO-NOT-READ/16E` — unified read-write model (a *living* note, edited in place): a
  sound elision needs *both* forward value-validity (reaching same-fact writes) and backward
  result-liveness (is the output consumed?). Contains an internal recant→over-correction→re-validation,
  and names a recurring anti-pattern (§5).
- `notes/quarantine-DO-NOT-READ/16F` — observable/replace model (human-ratified): bans the word "skip" →
  **replace**, "channel" → **observable**. A converged leaf is replaced by a `true`-stub that defaults
  every observable; a default is sound iff *dead or vouched*. In-spike floor: one observable-liveness
  gate. Corrects `notes/quarantine-DO-NOT-READ/16C` / `notes/quarantine-DO-NOT-READ/16E` (status is
  *not* an analyzer obligation; only
  stdout/stderr-liveness is).
- `notes/quarantine-DO-NOT-READ/16G` — observable-coverage audit; scoped the in-spike build (the
  liveness gate, blessing target-state-pure builtins, the skip→replace rename, a ⊤-containment spec).
- `notes/quarantine-DO-NOT-READ/16H` — observable gate landed (then leaf-local) + builtin-pure blessing
  + the rename (`SkipLicense`→`ReplaceLicense`, `# skip[`→`# replace[`).
- `notes/quarantine-DO-NOT-READ/16I` — an adversarial pass found the just-landed gate's leaf-local
  design bypassed by enclosing scope (`{ install; } > f`, `( install ) | grep`); fixed with a top-down
  consumption walk. ⊤-containment fixed. HOLE#1 surfaced + deferred.
- `notes/quarantine-DO-NOT-READ/16J` — superposition spec: *collapse at the caller, not the engine*.
  Method correction: a context compaction had dropped `DESIGN`/`KNOBS`, and the agent was reasoning from
  the notes + an analogy; re-reading the primary docs answered the supposed open question directly.
- `notes/quarantine-DO-NOT-READ/16K` — superposition landed: consumption computed *during CFG lowering*,
  stored per-node, emitted un-collapsed; the phased caller collapses it. A backward-`solve` fixpoint was
  considered and *rejected* (the CFG flattens pipelines, so a dataflow pass could not see
  pipe-consumption).
- `notes/quarantine-DO-NOT-READ/16L` — test-suite audit (per-file / global / adversarial). The
  highest-value finding: a target-state-pure-builtin allowlist was pinned only for `set`; a mis-edit
  dropping `:`/`echo` would be a silent wrong-skip → added a classify-layer guard.
- `notes/quarantine-DO-NOT-READ/16M` — apply-2 compiler: the three-applies taxonomy; built the forward
  `compile_probe`→`ProbePlan` and the **can't-probe ⇒ can't-elide** link; the assembled chain DST-tested
  over 64 seeds. DESIGN holes 1-3 surfaced.
- `notes/quarantine-DO-NOT-READ/16N` — CLI round-trip + sh e2e: built the `dorc --book=<f> [-o
  <oracle>]…` round-trip (read-only probe on stdout → results on stdin → eliding apply, line-granular).
  A `sh`-mechanized harness (not cargo), 3 baseline cases.
- `notes/quarantine-DO-NOT-READ/16O` — e2e "weirdos": 6 adversarial cases. Finding: every weirdo
  degrades to *run, never wrongly elide*, with no crash; the apply floor is apply-1. 9/9 e2e cases green.

---

## 3. Built-vs-designed ledger — the generality spine

A future *implementing* agent must not read a deferred or illustrative idea as a shipped decision. This
ledger is the spine; every finding in §4 is tagged against it.

### 3.1 Built (present in spike code, exercised by tests)

- **`core`** — newtype ids, `Span`, `Carrier<T>` (value + accumulated diagnostics), a deterministic
  `Interner` (order-of-interning assignment, never hashed), `OpaqueToken`/`KindId`/`ProviderId`,
  `Phase`/`Verdict`/`Grade`/`FactDomain`/`Fact`.
- **`syntax`** — a hand-rolled byte lexer + recursive-descent parser for the narrow subset; lossless
  quoting; first-class redirections; heredoc body capture; `Unsupported`=⊤ with a fixed reason set; a
  depth bound; total / no-throw (a backtick lexer infinite-loop and an unbounded recursion were caught
  and fixed during the build).
- **`analysis::lattice`** — `Lattice`/`BoundedLattice` + the combinators `Powerset`/`Flat`/`Product`/
  `MapL`/`May`/`Must`; lattice-law property tests; `MapL` kept in canonical no-⊥ form so structural `Eq`
  coincides with semantic equality.
- **`analysis::solve`** — a propagation worklist generic over `Graph` + `Lattice` +
  `Direction{Forward, Backward}`; returns `Solution{states, converged, rounds}` with an iteration cap.
- **`analysis::cfg`** — `Ast→Cfg`; node kinds Entry/Exit/Command/Redir/Merge/ScopeEnter/ScopeExit/Top; a
  **precise** errexit forward pass with subshell scope save/restore; per-node `expansion_internal`
  (command-substitution bodies); per-node `consumed` observables computed during lowering; `Top` for
  `Unsupported`.
- **`analysis::effect`** — `command_effect` (oracle lookup, ⊤-conservative); the reaching-defs ambient
  gate (a `Reach` lattice via `solve`); an `is_target_state_pure_builtin` allowlist; an
  entry-reachability gate; a converged-trust (`trust_reach`) gate; `classify` →
  MustRun / EstablishAmbient / EstablishWritten.
- **`oracle`** — static `lift` of the fact-centric sh idiom → a `KindIndex` (probe-by-kind, effects
  keyed `(provider, verb)`); diagnostics on non-literal anchor / missing probe / top-level mutator /
  malformed effect; fail-soft.
- **`plan`** — `PhasedVerdict<P>` + `Bias`; `ReplaceLicense` minted only by `prove_replaceable`, which
  checks four conditions (class is an ambient establish ∧ `Must` ∧ verdict-resolves-Converged ∧ no
  consumed-unvouched stdout/stderr); ⊤-containment is a *separate* guard in `build_plan` (via
  `has_top_successor`: any leaf adjacent to a `Top` node folds to Run before the mint is attempted), so
  the net elision predicate is the conjunction of the two; a `Derivation` audit trail; `compile_probe`→
  `ProbePlan` (+ can't-probe⇒can't-elide); `build_plan`→`Plan{Run|Replace}`; `render_sh` (flat) +
  `render_apply` (line-granular).
- **`hostsim`** — a seeded LCG; a `Host` fact-store; a `verdict`; a `run(phase, op)` carrying a
  `kFAIL-withhold` violation monitor; DST chain tests (64-seed) + an unprobeable-fact test.
- **`cli`** — the `dorc --book= [-o]…` round-trip (probe → stdin results → eliding apply); a lenient
  `parse_results` (unknown ⇒ `Unknown`).
- **`spike/e2e/run.sh`** — a `sh`-mechanized e2e (not cargo); 9 cases (3 baseline + 6 weirdos),
  golden-diffed.

### 3.2 Designed but not built — *do not present any of these as decisions*

These are the contamination risks. Each was discussed, sketched, or reserved; **none is in the code as a
working capability.**

- **The apply executor (DESIGN's "Option C").** Actual host mutation over time, TOCTOU
  re-probe-before-apply, idempotence-by-execution. `hostsim` models the *effect* side, but nothing
  drives a plan against a host. The spike compiles a probe and an apply and executes neither.
- **Multi-host / fleet / CAP** failure modes — direction only (`notes/quarantine-DO-NOT-READ/16A`).
- **apply-3** (the targeted "desired-set" mode, `dorc bump`) and its **backward relevance-reduction** —
  the *solver* supports `Direction::Backward`, but **no backward analysis is instantiated**; only
  forward analyses exist. `Must` and the dual machinery are exercised only by their own unit tests.
- **The oracle `Bridge{gather, compute}`** (the discharge for an output-consumed leaf) —
  **outside-spike**; the in-spike posture is the MustRun floor.
- **`Grounded<T>` / `OracleConditional<T>`** (typed best-effort-under-degradation) — a deferred sketch;
  the only realized fragment is the `Derivation` record naming which oracle claim a replacement rests
  on.
- **Value-flow / `Owes` lattice / `Discharge` witness** — refuted analyzer-side, then deferred.
- **A verdict-memo / cross-host memoization** (`{verdict, content_key, freshness}`) —
  `notes/quarantine-DO-NOT-READ/160` called it "day-1"; **never built** (no content-keys, no freshness,
  no cache).
- **Tier-B interprocedural** (IFDS/supergraph, call/return/handler edges, the backward apply-slice) —
  reserved; a known latent gap is function bodies seeded errexit-`Off` pending real call-edges.
- **The precision / recency layer** — *not built*: no recency strong/weak update, no per-entity
  qualifier/selector (`installed` vs `version`; `svc#enabled` vs `#active`), no uniqueness/aliasing
  gate. The fact domain is a flat boolean set of mutated `(kind, entity)` keys (reaching-defs gen/kill
  only). `core::FactDomain`/`core::Fact` exist in the vocabulary but the analysis keys on the leaner
  flat key; the partition/selector representation is reserved-but-unwired. (The `Research/plans/055`
  reference design and `notes/quarantine-DO-NOT-READ/160` flag this as load-bearing — TAJS-style
  precision collapses without it.)
- **A general shell-env-state analysis** — only **errexit** was modeled, as a bespoke forward pass in
  `cfg::build`, *not* the generic `solve` over a `Product` lattice that
  `notes/quarantine-DO-NOT-READ/163` sketched. `cwd`/`trap`/`ifs`/`fds`/`pipefail` are unmodeled;
  `trap`/`alias`/`source`-following are unmodeled (a literal `. /path` parses as a plain command and its
  effects are not followed).
- **Real `kFAIL-withhold` enforcement** (seccomp / sandbox) — the `hostsim` DST monitor is the
  stand-in; the contract frame provably *cannot* enforce probe-inertness on its own (§5 DP-4).
- **Probe flakiness / unreliable-oracle modeling** (Unknown-with-probability) — deferred; the seeded
  PRNG drives only *initial host state*.
- **HOLE#1** — lowering command-substitutions in redirect-targets / case-patterns into the CFG (so their
  effects poison) — deferred (the principled "CFG-lowering completeness" item).
- **fd-dup resolution** (`2>&1`, `>&3` beyond the structural floor) — deliberately unresolved (keeps
  `> /dev/null 2>&1` replaceable).
- **Per-provider flag grammars** (entity-extraction precision) — only a coarse single-literal-non-flag
  operand rule; pre-verb flags / multi-entity / non-literal / double-quoted-literal all ⇒ MustRun.
- **The probe *projection* mechanism** — what the probe phase actually does with the consumption fact,
  and whether it reads this exact fact, is the unbuilt open question `inv-superposition` exists to force
  a conscious per-phase answer for.
- **A faithful control-flow plan rewrite** — `render_sh` flattens; `render_apply` only comments lines.
  Also unbuilt: heredoc-body-complete leaf text, pipeline-as-one-leaf, precise subst-body source spans.

### 3.3 Relation to the `055` analysis-architecture reference design

`Research/plans/055-analysis-architecture.md` is the firm engine reference design. The clearest framing
for what the spike *is*: **the spike built `055`'s cheapest tier and none of its heavy machinery.**
Concretely it realized `055`'s Tier-A fast path — plain intraprocedural monotone dataflow — and the
*shape* `055` names (oracle effect-class as the pluggable transfer; ⊤-on-unknown = un-probeable =
can't-elide; the two phase-keyed soundnesses). It took **neither** of `055`'s heavier substrates: no
IFDS/IDE realizable-path summaries, no Datalog/Soufflé fact-base — the substrate decision
`055`/`notes/quarantine-DO-NOT-READ/160` flagged *stays open*; the spike hand-rolled a generic
monotone-dataflow worklist instead. Reserved-but-unbuilt from the reference design: the interprocedural
supergraph + summaries; the recency strong/weak keystone + selectors (§3.2); per-role compositional
summaries + per-host instantiation; incremental/diff-time deployment; backward slicing from a dirty set;
the queryable Datalog fact-base + provenance why-trees + the N-tier locator DAG. The spike is the
cheapest-tier skeleton of `055`, **not a down-payment on its heavy machinery.**

---

## 4. Findings by thread

Durable findings, organized by queryable topic (the slugs `T1`…`T17` are stable handles). Each carries
the *finding* in prose, plus a symbol/note anchor for the evidence.

### T1 — the pure, deterministic kernel

The whole analysis pipeline (`syntax → analysis → plan`) was implemented as a *total function* of its
inputs: no clock, RNG, filesystem, or network reachable, directly or transitively. Nondeterminism was
confined to exactly two edges — a seeded, injected PRNG in the host model, and the CLI's real I/O.
Ordered collections were used wherever iteration order is observable in output; no hash-ordered
iteration reaches output. Every stage returns a *value + accumulated diagnostics* carrier and never
panics on malformed input: errors are data, not control flow, so a degraded value plus `Error`
diagnostics flows downstream and *unrelated* problems still surface in a single pass.

The durable finding: this purity is what let the *entire pipeline* run under deterministic-simulation
tests with no dependency-injection ceremony — the kernel itself is the unit under test, fed seeded host
answers. Stated as design guidance, a pure / finite / synchronous kernel *sidesteps the
async-vs-state-machine kernel fork entirely* (async enters only at the mocked executor), and made the
kernel's reachable-flow soundness checkable by independent review agents (T17). Errors-as-data is what
enables the batch best-effort posture DESIGN asks for.

*Anchor:* `core::Carrier`, `core::Interner`; the determinism integration tests. *Spike realization
(illustrative):* the `Carrier<T>` writer-monad shape and its name are one way to spell errors-as-data,
not a requirement.

### T2 — the modeled subset and ⊤-rejection

The parser models only the narrow sh subset the analyzer exercises, grown *demand-driven* (seeded by one
real "book" fixture plus the oracle idioms). Everything outside collapses to an explicit `Unsupported`
node carrying a reason, and downstream to an absorbing ⊤ CFG node — **rejected loudly, never silently
best-effort'd**. The framing that matters: under-modeling is a *correctness boundary*
(elision-soundness), not a TODO — an unmodeled construct must be both un-probeable and un-elidable,
because a half-understood construct could hide a mutation that invalidates an elision. The ⊤-trigger set
is split by locus: *syntactic* triggers caught at the parser (`eval`, dynamic command names, arithmetic
command position, lvalue-taking builtins, loops, background `&`, over-deep nesting) versus *semantic* ⊤
at the dataflow (no oracle entry, dynamic word). Totality is the load-bearing invariant the pipeline
rests on, because the parser runs on untrusted scripts: a table of hostile inputs (NUL bytes,
unterminated quotes/heredocs, garbage operators, deep nesting) must all return without panic, every
arena id resolving.

*Anchor:* `syntax::ast::NodeKind::Unsupported`, the parser trigger checks and `MAX_DEPTH`,
`cfg::CfgNodeKind::Top`; the totality tests. *Spike realization (illustrative):* the *exact* current
subset (no loops, etc.) is minimal-to-what-works — "a weirdo that hits a syntax limit is fixed by the
fixture, not the grammar" (`notes/quarantine-DO-NOT-READ/16O`). The boundary discipline is the durable
part; the subset's contents are not.

### T3 — the monotone-dataflow substrate

The engine is the classical monotone-dataflow framework: a finite-height lattice + monotone per-node
transfer functions, solved to least fixed point by a propagation worklist, generic over `Graph` +
`Lattice` + `Direction`. The framework knows nothing about shell; sh-specific modeling sits on top.
Lattices are built compositionally from combinators (a powerset, a height-2 flat lattice, a
componentwise product, a pointwise map kept canonical so structural `Eq` is semantic equality) rather
than hand-rolled per analysis.

The surfaced design problem (carried as §5 DP-2): the framework's termination and correctness rest on
preconditions the type system *cannot* express — transfer monotonicity, finite height *for the values
actually produced*, and semantic `Eq`. A violating transfer **loops forever**; this was reproduced
empirically (two non-monotone `solve` runs reached 435 and 783 CPU-seconds before being killed). The
mitigation built: `solve` returns a `converged` flag plus a generous iteration cap, converting an
infinite climb into a loud non-convergence; a malformed graph edge is a debug-assert + release-skip, not
a panic. The deeper finding ("TypeScript, not Coq", `KNOBS kVERIFY`): un-provable invariants need DST +
loud-fail backstops, not doc-comments alone.

*Anchor:* `analysis::lattice`, `analysis::solve` (`Solution{states, converged, rounds}`); the
non-convergence and lattice-law tests. *Generality:* durable as method; the network-context caveat
(`notes/quarantine-DO-NOT-READ/163`) bounds it — even an "expensive" analysis is dwarfed by the SSH
round-trips that follow, so the engine's big-O is not the thing to optimize (`KNOBS kPRECISION`).

### T4 — orientation discipline (`May`/`Must`)

The *same* one-direction solver runs both a may-analysis (over-approximate: ⊥-start, ⊔-merge) and a
must-analysis (under-approximate) by wrapping the lattice: `May<L>` is the identity wrapper; `Must<L>`
is the **order-dual** of `L`. So "which merge" is picked by the *type*, eliminating the
union-where-you-needed-intersection bug class. The one-way coercion `Must → May` exists; `May → Must`
does not — a degraded belief can never be re-promoted to license an elision. A second boundary was
lifted into the type system: a must-analysis needs a *representable* ⊤ to seed interior nodes, but a
powerset over an unbounded element type has no finite universal set; `BoundedLattice` is implemented by
the flat lattice but **not** by a bare `Powerset`/`MapL`, so "a must-analysis over a bare powerset" is a
*compile error*. This is the antidote to the central hazard of `notes/quarantine-DO-NOT-READ/165`: a
correct value consumed in the wrong soundness orientation, *silently* — no crash, no wrong number, just
an answer sound under assumptions the reader does not hold.

**Generality flag (NOT BUILT).** Despite the full `Must` machinery, the spike instantiates *no*
must-analysis and *no* backward analysis; the apparatus is reserved for apply-3 (T13). The
engine-wide-meet choice was explicitly *state-space exploration* (the
`notes/quarantine-DO-NOT-READ/168` "calibrate-up" ruling), not a load-bearing requirement of what was
built. Do not read the maximal orientation machinery as a settled architectural decision.

*Anchor:* `analysis::lattice::{May, Must, BoundedLattice}`; the duality and dual-analysis tests.

### T5 — phase-keyed soundness (`kFAIL`)

Two soundnesses with opposite fail-directions, *welded and phase-keyed* (`KNOBS kFAIL`): the probe phase
never mutates (when unsure, don't probe); the apply phase never skips a needed mutation (when unsure,
act). A three-valued verdict (`Converged`/`Diverged`/`Unknown`) folds `Unknown` conservatively *per
phase* via a `Bias` trait with one impl per phase; there is no writeable code path that folds `Unknown`
to an elision. The phase is carried in the type (`PhasedVerdict<Probe>` cannot be read as
`PhasedVerdict<Apply>`). Observed in the e2e weirdo pass: under every tested degradation
(unmodeled / un-oracled / poisoned / output-consumed / garbage input), the apply phase produced the
conservative (run) disposition, with no crash — i.e. the apply floor is apply-1.

*Anchor:* `core::Phase`, `core::Verdict`, `plan::{PhasedVerdict, Bias}`; the `unknown_folds_to_run`
tests and the e2e cases. *Spike realization (illustrative):* both `Bias` impls currently return `Run` on
`Unknown` (the probe-phase plan-builder is unbuilt), so the probe-vs-apply *distinction* in `Bias` is
latent — designed, not yet exercised divergently.

### T6 — the witness pattern

The single irreversible verb (elide a command) takes a *witness with private fields*, not a `bool`. The
only constructor is one reviewed proving function that checks every precondition; the plan emitter
accepts the witness type, so "elide" is unspellable without the proof. The witness carries its
derivation (the audit trail, the greyed-out "why" for a future plan UI). The durable pattern:
concentrate the catastrophic decision in one reviewable place; combined with the T4/T5 types, the
wrong-elision is made *structurally* hard rather than convention-enforced.

*Anchor:* `plan::{ReplaceLicense, Derivation}`, `plan::ReplaceLicense::prove_replaceable` (the sole
mint); the `license_minted_*` / `no_license_*` battery. *Spike realization (illustrative):* the
`ReplaceLicense` name (renamed mid-spike from `SkipLicense` — T10) is one realization of the
private-field-witness pattern.

### T7 — the referent-agnostic kind-index (dn-1, the oracle contract)

Cross-oracle identity must bind to a *named kind*, never a shared argument token — token-equality
collides across oracles. The engine never decodes a token's text to infer meaning: it compares tokens
for intra-script co-reference and resolves them for display/provenance only. The kind-index is a
**3-place relation** `(kind, provider, verb) → effect`, *not* a 1-place naming convention (which
clobbers when two providers touch the same kind). The contract is **fact-centric, not command-centric**
— the spike's first major correction (`notes/quarantine-DO-NOT-READ/161` → `notes/quarantine-DO-NOT-READ/162`, §5 DP-1). An oracle
declares a named kind, a read-only fact-probe that observes whether `kind:entity` holds (three-outcome:
holds / absent / can't-tell), and an effect-map `(provider, verb) → (kind, establish|kill)`. A book's
bare mutator resolves through the effect-map to a fact, and an elision is licensed by *the fact already
holding* plus the ambient gate (T8) — never by re-running the mutator in dry-run. The first strawman's
command-centric dry-run probe was refuted because the named-kind index was *decorative* in that elision
path. The oracle file is plain sh that the analyzer *lifts statically* (never sources or runs):
assignments, plainly-named functions, accumulating marker calls; it stays dash-clean and inert if Dorc
vanishes (a *weak* off-ramp — inert, not independently useful).

*Anchor:* `core::{OpaqueToken, KindId, ProviderId}`, `oracle::{KindIndex, FactProbe, lift}`,
`effect::command_effect` (⊤-conservative on dynamic word/verb, no oracle entry, or not-exactly-one
literal-non-flag operand). *Spike realization (illustrative):* the exact sh spelling (`oracle_kind=`,
`oracle_probe_<kind>()`, `oracle_effect <prov> <verb> <pol>`) is a strawman held for the human's `kOOB`
ruling (§5), **not a committed syntax.** *Surfaced problems:* DP-3/4/5 and the `kOOB` ruling (§5).

### T8 — the ambient gate (reaching-definitions over the effect-map)

The cardinal wrong-elision guard. A converged establish is elidable only if its fact is *ambient*: no
upstream in-script command mutated it, so the host's resting probe is authoritative. This is
reaching-definitions over the oracle effect-map (oracle gen/kill = the transfer functions). It catches
the canonical wrong-elision `apt-get purge nginx; … apt-get install nginx`: the purge's kill *reaches*
the install, so the install is classified `EstablishWritten` (resting probe stale) and must run — even
on a host where nginx is present at probe-time.

Two adversarially-surfaced refinements, both durable:
- **(DP-8) Half-modeling a construct is more dangerous than ⊤-rejecting it.** An establish in a
  *detached region* (a function body with no modeled call-edge) had a vacuous-⊥ reaching in-state,
  indistinguishable from a clean "nothing upstream mutated this" — read as ambient, a latent
  wrong-elision found by the adversarial pair. Fix: an *entry-reachability* gate — a command unreachable
  from `entry` folds to MustRun. Generalizes to any future detached region (trap handlers, sourced
  files, backgrounded `&`).
- **(DP-9) A capped / non-converged solve is a partial under-approximation** (an upstream kill may not
  have propagated). Trusting it would be a silent wrong-elision. Fix: `classify` reads ambient state
  only behind a `trust_reach = reach.converged` gate; `!converged ⇒ MustRun`. The producer cannot
  enforce this — it is a *per-consumer* obligation.

The precision cost, made legible: any un-oracled command is `Opaque` ⇒ ⊤ ⇒ it poisons *all* downstream
ambient-ness. Because ubiquitous builtins (`set`, `:`, `echo`, `cd`, `[`) touch shell-env/stdout but no
oracle-modeled fact, a blessed *target-state-pure-builtin* allowlist treats them as `Pure` so they don't
poison; anything off the list stays the safe over-refusing `Opaque`. (Without it, a `set -e` atop nearly
every defensive book would block all elision — a value cost, not a correctness bug.) The
adversarially-validated *negative* result, stated at its true strength: across straight-line / branch /
`&&`/`||` / subshell / `$()` / pipeline / `set -e` / heredoc / `case` flow, **two independent agents
could not find a wrong-elision in the reachable gen/kill core** — a confidence signal, *not* a proof of
soundness.

*Anchor:* `effect::{Reach, classify, SkipClass, reachable_from_entry, is_target_state_pure_builtin}`;
the `*_is_ambient` / `*_poisons_ambientness` / `detached_function_body_*` tests.

### T9 — CFG hazards (construction/dataflow coupling)

`set -e`/errexit *alters the CFG itself* (a conditional failure→exit edge after a fallible command), and
errexit is itself a forward dataflow fact (toggled by `set -e`/`set +e`). So there is **no clean
build-then-solve split**: the CFG builder runs a small errexit forward pass and materializes the edges.
The `notes/quarantine-DO-NOT-READ/166` finding: a *coarse* "add a failure-edge when unsure; more edges =
safe" model is unsound **both ways** — a *missing* edge is unsound forward (a wrong elision), a
*spurious* edge is unsound backward (the future apply-minimization slice sees an always-reached mutation
as conditionally bypassed). The fix makes errexit *precise*: failure-edges are pruned where the shell
never aborts (a `!`-negated pipeline; the whole `if`/`while`/`until` condition region; `&&`/`||` left
operands; `|| true`) and *extended* where it does (a failing **redirection** aborts under `set -e`
too). The one remaining conservative direction is `set "$dyn"` ⇒ ⊤ ⇒ add the edge, with a diagnostic.
Each fixed case has a regression test pinned to a `dash`-verified script.

Two more durable structural points: subshell `( )`/`$( )` scope — env/var/cwd mutations don't escape, FS
mutations do; the errexit pass saves/restores at scope boundaries so a toggle inside a subshell doesn't
leak out (a brace group `{ }` *does* leak, both modeled, both `dash`-verified). And **leaf-scope and
effect-scope are different and tracked separately** (`notes/quarantine-DO-NOT-READ/16B`): a command
inside a `$(…)` body is *effect-bearing* (its mutations still poison/establish) but is *not* a runnable
plan/apply leaf (it runs during word expansion) — marked `expansion_internal` during lowering, kept in
the dataflow, excluded from the leaf set; subshell/group bodies remain leaves.

*Anchor:* `cfg::build` (the two-phase construction; the `ErrExit` lattice; `materialise_errexit_edges`;
`pair_scope`; `mark_consumed_range`; `expansion_internal`); the `find1..find6` and
`command_substitution_body_is_expansion_internal` tests. *NOT BUILT:* only errexit is modeled among
shell options (§3.2); the errexit pass is a bespoke forward fixpoint, not the generic `solve` over a
`Product` lattice that `notes/quarantine-DO-NOT-READ/163` sketched.

### T10 — the observable / replace model

"Skip" was rejected as a word: it connoted *omit the line*, framing the decision as unary and hiding the
real operation. The operation is **replace** — substitute the cheapest *stand-in* that reproduces the
*observables* a leaf's consumers read; omission is only the degenerate replace (stand-in = `true`,
nothing reads any observable). The reframe forces the question "replace with *what*?" that "skip"
suppressed. The vouching model: the trivial stub defaults every observable (effect→none, status→0,
stdout/stderr→empty), and a default is sound iff *dead or vouched* — **effect ← convergence** (the
forward ambient gate); **status ← the establishes-contract** (declaring "(provider,verb) establishes F"
*is* the claim "when F is converged this is a successful no-op", rc 0 — free, and load-bearing because
under `set -e` every status is consumed); **stdout/stderr ← nothing**. So a value-bearing consumed
stdout/stderr makes the stub unsound ⇒ run. This is one uniform *observable-liveness* obligation with
status discharged by the establish-contract — not separate status/stdout analyses, and *no* analyzer
reasoning about rc/stdout *values* (undecidable, over-approximated by the decidable structural surrogate
"the observable is consumed in a value-bearing position").

The adversarial correction: the first gate was *leaf-local* (inspecting only a leaf's own redirs /
pipeline membership), and an adversarial pass found it bypassed — output-consumption is a property of
the *enclosing* construct (`{ install; } > f`, `( install ) | grep`). The fix propagates enclosing
consumption down to inner leaves (a top-down structural mark). The `/dev/null` discard sink is exempt
throughout — the precision "scalpel": the gate must *shrink* replacements, not over-run and erode the
feature. Separately, ⊤-containment was enforced at the plan layer: a leaf whose CFG node has a `Top`
successor folds to Run (slightly over-refusing — adjacency, not strict containment — but sound), because
`build_plan` never consults diagnostics.

*Anchor:* `cfg::{Observable, Cfg::consumed_observables, output_redir_observables}` (the `/dev/null`
exemption); `plan::{prove_replaceable, has_top_successor}`; the observable state-space matrix test. *NOT
BUILT:* the oracle *bridge* that would discharge a consumed stdout (so a value-consumed leaf could be
replaced anyway), the `Owes` lattice, the `Discharge` witness, and any rc/stdout *value* analysis —
explicitly outside-spike (§3.2). fd-dup resolution is a deliberately-unbuilt precision floor.

### T11 — superposition (engine emits, caller collapses)

The principle (`inv-superposition`): the analyzer kernel emits *phase- and orientation-agnostic* lattice
facts; only the phased *caller* collapses them, by arguing the phase and orientation. The engine must
never fold `May`/`Must` or bake a phase default — a baked posture is a wrong-elision under the opposite
phase's `kFAIL`. This generalizes the verdict's phase-typing (T5) to *every* phase-sensitive fact
(DESIGN: "same analysis, different fail-safe posture"). The consumption fact is emitted un-collapsed;
`prove_replaceable<P: Bias>` is generic over the phase and receives consumption as a `May<_>` that — per
the one-way coercion — can only *block* a license, never grant one.

The implementation finding: consumption is computed *during CFG lowering* (a single exhaustive
structural traversal with no catch-all arm, so a new node kind is a compile error), stored per-node as a
*total* vector — so "empty" means examined-and-quiet, never un-examined; the "absent leaf" that slipped
an earlier hand-traversal is structurally impossible. A backward `solve` fixpoint was considered and
*rejected*: the CFG flattens pipelines (carries no pipe-edges), so a dataflow pass literally could not
see pipe-consumption, whereas the lowering still has the `Pipeline` AST in hand. A deliberate
non-decision: a per-phase `Bias::on_consumed_output` method was floated and *not added* — the result
enum is apply-shaped (`{Replaceable, Run}`, no `Withhold`), so a per-phase consumed-direction method
would force the unbuilt probe impl to return the wrong (apply-shaped) answer, baking exactly the posture
the principle forbids. The honest forcing-point for the probe phase is a *separate* probe plan-builder
(deferred, §3.2).

This thread also carries a *method* lesson (expanded in §5): the note exists because a context
compaction dropped the human-authored `DESIGN`/`KNOBS`, and the agent was reasoning from the
(AI-authored) notes plus an analogy; re-reading the primary docs answered the supposed open question
outright.

*Anchor:* `cfg::{Cfg::consumed_observables, Builder::mark_consumed_range}`, `plan::prove_replaceable<P>`;
`notes/quarantine-DO-NOT-READ/16J` (spec) + `notes/quarantine-DO-NOT-READ/16K` (landed). *Note:*
`inv-superposition` was cited throughout the `cfg`/`plan` code but absent from
`notes/quarantine-DO-NOT-READ/spike/CLAUDE.md`'s invariant registry
(`notes/quarantine-DO-NOT-READ/16K` left it proposed-not-pasted); it was retrofitted into the registry (human-authorized) so an agent reading the
list finds what the code is written against.

### T12 — best-effort-under-degradation (the meta-lens) — *largely NOT BUILT*

Dorc's soundness is *capped by construction*: the oracle-grounding boundary is unverifiable (a
frame-clean oracle can ship a wrong fact-probe, a mutating probe, a lying verdict, a garbled lift). The
heavy types therefore do **not** prove ground truth; they prove a *conditional* ("*if* the oracle's
unverifiable claims hold, the elision is safe") and a *floor* ("*when* they don't, the failure direction
is the phase-safe one"). The footgun this lens names: an agent or tired human reading the witnesses as
proofs-of-reality and over-trusting exactly where the design is weakest. What the types *can* defensibly
do, and the standing review criterion they imply: (a) make oracle-trust dependency *type-visible* (no
conflation of oracle-conditional with analyzer-proven); (b) guarantee every degradation folds
conservatively; (c) bound the blast radius (one lying oracle corrupts only its own elision). Existing
locks (T2's ⊤-reject, T5's `Bias` fold, T4's one-way `Must → May`, T1's no-throw,
oracle-lift-degrades-to-⊤) are *ad-hoc instances* of this stance.

*NOT BUILT:* the proposed *uniform* mechanism — a `Grounded<T>` / `OracleConditional<T>` wrapper on every
oracle-derived value (taint-style, discharged only by a `kFAIL`-fold or a witnessed runtime backstop) —
was sketched and *deferred*. The only realized fragment is the `Derivation` record naming which oracle
claim a replacement is conditional on. The note stands as a standing review-criterion for all later
design: *where does oracle-trust enter, is that entry type-visible, and does every degradation fold
conservatively with bounded blast radius?*

### T13 — apply taxonomy + the probe→apply contract

Three applies, sharper than DESIGN's prose: **apply-1** full unconditional (run everything — the trivial
fallback floor); **apply-2** converge + safe-elide (probe the host, elide what is provably already
converged — *the default*, forward-only); **apply-3** targeted desired-set (apply the user's set, eliding
what can't be proven *relevant* — deferred). The key relation: **apply-3 ⊃ apply-2** — it is apply-2 plus
a *backward* relevance-reduction, so it needs the backward engine; a strict superset of effort, not a
separate path (`KNOBS kELISION`). The spike pins a contract DESIGN left implicit (surfaced as holes 1-3,
§5): **elide leaf L iff probe(L.fact)=Converged ∧ ambient ∧ Must ∧ no-consumed-unvouched-observable ∧
not-⊤-contained** — where the first four conjuncts are checked by the license mint `prove_replaceable`
and ⊤-containment is a separate guard in `build_plan` (T6, T10). And the **can't-probe ⇒ can't-elide**
link — a kind with an effect but no declared probe is un-checkable, so it is absent from the probe and
the apply runs it, even on a host that holds the fact. The forward compiler half (`compile_probe`) emits,
per probeable ambient fact, the oracle's verbatim read-only probe body as a renderable read-only script;
the apply half (`build_plan`) is driven by the (simulated or real) probe answers.

*Anchor:* `plan::{compile_probe, ProbePlan, build_plan, Plan}`; the `compile_probe_includes_*` and
`dst_apply2_chain_*` tests. *NOT BUILT:* the apply *executor* (Option C) — host mutation over time,
TOCTOU re-probe-before-apply, idempotence-by-execution — and apply-3 / the backward relevance-reduction.
The compiler compiles a probe and an apply; it executes neither.

### T14 — the leaf-seam (dn-3)

Executable work is a list of *individually wrappable* leaves, each with a stable `LeafId → AstId`
back-map — never one opaque `sh -c "$bigscript"`. The back-map is dual-use (plan provenance now; a future
per-leaf tracer/wrapper later, the seam `KNOBS kFIDELITY-faithful` reserves). "What is a leaf" is an
explicitly *unsettled* part of dn-3 (top-level / branch-body / subshell-body / group-body are leaves;
`$()`-internal commands are not). Two render modes were built and stand as a concrete instance of the
optimized-vs-faithful tension: a *flat* source-ordered list (`render_sh`) that does **not** reproduce
enclosing `if`/`case` guards (it shows mutator dispositions, not a runnable control-flow rewrite), and a
*line-granular* book-faithful render (`render_apply`) that comments already-converged lines in place,
preserving control-flow structure and indentation.

*Anchor:* `plan::{LeafId, Step, Disposition, Plan::render_sh, Plan::render_apply}`; the e2e `guarded`
case (an `if … fi` preserved with indentation). *NOT BUILT:* a *faithful* control-flow plan rewrite
beyond line-commenting; heredoc-body-complete leaf text; pipeline-as-one-leaf; precise subst-body source
spans.

### T15 — DST / host-model methodology

The host is a *seeded, deterministic state-machine* (an injected `u64` seed → an LCG → a random subset of
candidate facts) — the one sanctioned home for nondeterminism. It does two jobs: answer fact-probes
against a modeled system-state (the concrete stand-in for the injected verdict seam), and **detect a
probe attempting a modeled mutation** — the `kFAIL-withhold` monitor (a recorded-and-refused violation),
the DST stand-in for the real seccomp/sandbox the contract frame provably cannot supply (§5 DP-4).
Looping seeds fuzzes the analyzer/plan over many host states, reproducibly, with no network. Because the
kernel is pure/finite/synchronous (T1), DST needs no DI ceremony — the kernel itself is the unit under
test — and the same property made the kernel's soundness reviewable by independent adversarial agents in
one session.

*Anchor:* `hostsim::{Host, HostOp, Violation, Lcg}`; the `probe_phase_mutation_is_a_withhold_violation`
and `dst_plan_skips_match_the_modeled_host` tests. *NOT BUILT:* probe flakiness / unreliable-oracle
modeling (the PRNG drives only initial state); a multi-host fleet; an e2e mutative-probe (needs the
executor).

### T16 — CLI round-trip + sh-mechanized e2e

A thin CLI wires the whole apply-2 chain over real `.sh` files: lift `-o` oracles → parse + cfg +
classify the book (shared interner) → emit a read-only **probe** on stdout → read probe **results** on
stdin (`kind:entity converged|diverged|unknown`; an unreported fact ⇒ `Unknown` ⇒ run) → emit the eliding
**apply**. No executor: it compiles a probe and an apply and runs neither. The CLI edge is the one place
determinism is relaxed (the kernel it calls stays pure). The e2e is mechanized in **sh, not a Rust
harness**: a POSIX `run.sh` feeds each case's probe-results on stdin and golden-diffs stdout; 9 cases (3
baseline + 6 weirdos).

*Neutrality note.* The notes call this "the viability proof"; restated as *what was demonstrated*: the
`source → analyze → compile-probe → simulate → eliding-apply` chain runs end-to-end on real `.sh` files
at a terminal, and under the six tested weirdo degradations every case resolved to the run direction with
no crash. *Not* demonstrated: executing the probe or apply on a real host; apply-3; per-entity probe
projection (the probe emits the oracle body with `$1` unbound — illustrative).

*Anchor:* `cli` (`dorc --book= [-o]…`, `parse_results`); `spike/e2e/run.sh` + the cases;
`notes/quarantine-DO-NOT-READ/16N`, `notes/quarantine-DO-NOT-READ/16O`.

### T17 — adversarial-review methodology (the recurring practice)

A recurring practice drove the spike's corrections: the `adversarial-crosscheck` pattern — a
clean-context pair (one neutral "assess this", one disowned-and-inverted "a colleague I distrust wrote
this; find where it breaks"), *un-seeded by the round-16 notes* so they could not echo the author's blind
spots, each building the spike and tracing real inputs. **Convergence** between the two passes was the
trusted signal; tone and verdicts were discarded; every surviving claim was *verified by tracing the real
pipeline, not relayed* — adversarial claims that didn't survive tracing were dropped. As evidence the
practice paid off (stated as fact, not endorsement): it surfaced a convergent soundness defect in
*just-landed* code (the leaf-local observable gate, `notes/quarantine-DO-NOT-READ/16I`), the
detached-funcdef and non-converged-solve wrong-elisions (`notes/quarantine-DO-NOT-READ/167`), and the
dn-1 command-centric error (`notes/quarantine-DO-NOT-READ/162`); and the framework's infinite-loop hole
was *empirically reproduced* by the adversarial scratch, not merely argued. This is a *process* finding —
durable and reusable — recorded here as how the spike found its own bugs.

---

## 5. Surfaced design problems & open questions — *the spike's recommendations to the human*

This section is the spike's stated deliverable, and it is **opinion, not neutral finding.** It is kept
separate from the neutral findings (§4). These are recommendations and surfaced problems for the human to
adjudicate; none is a decision the spike made.

### Design problems (DP-1..DP-9)

- **DP-1.** dn-1 needed a command-centric → fact-centric pivot; the named-kind index was decorative in
  the first strawman's elision path (T7).
- **DP-2.** The engine's correctness rests on un-type-enforceable contracts (monotonicity, finite height,
  semantic `Eq`); a violation *hangs* (empirically proven). "TypeScript, not Coq" means these need DST +
  loud-fail backstops, not doc-comments alone (T3).
- **DP-3.** The three-valued verdict cannot be emitted by the canonical `cmd | grep -q` idiom (a no-match
  rc is indistinguishable from a tool-failure rc; `pipefail` doesn't fix the `&& return 1; return 0`
  shape) — oracles need a probe shape that captures the tool's own rc.
- **DP-4.** `kFAIL-withhold` (the probe never mutates) is **not enforceable by the contract frame**; it
  needs a separate sandbox/sancheck mechanism, built apart from the verdict channel. (The `hostsim`
  monitor is a DST stand-in only.)
- **DP-5.** Entity-extraction is *sound-XOR-useful*: a generic flag-stripper that ⊤s on any unknown flag
  is safe but ⊤s common idiomatic lines; a per-provider flag grammar fixes precision but re-arms the
  `-o`-class mutation hazard and is a real authoring burden (`KNOBS kBURDEN`).
- **DP-6** *(positive).* The pure synchronous kernel sidesteps the async-vs-state-machine kernel fork
  entirely (async only at the mocked executor), is DST-friendly with no DI ceremony, and made
  reachable-flow soundness checkable by independent agents (T1, T15).
- **DP-7.** The ambient ∧ invariant gate is the *analyzer's* job (reaching-defs over the effect-map), not
  the oracle's (T8).
- **DP-8.** Half-modeling a construct (its definition but not its invocation/handler edges) is *more
  dangerous* than ⊤-rejecting it, because the half-model *looks* analyzable — the reachability instance
  of the `notes/quarantine-DO-NOT-READ/165` orientation minefield (T8).
- **DP-9.** Solve-convergence is a *per-consumer* obligation (every analysis must fold `!converged` to its
  phase-safe ⊤), not just a producer return value (T8).

### DESIGN holes (surfaced in `notes/quarantine-DO-NOT-READ/16M`, for the human — *not* edited into DESIGN)

- **hole-1/2/3.** The probe→apply elision contract is *implicit* in DESIGN; "no probe ⇒ no elision" is
  unstated; DESIGN's elision paragraph blurs the two elision kinds (already-correct vs not-relevant-now)
  and does not enumerate the three applies (`KNOBS kELISION` only half-distinguishes). Recommendation:
  make the contract and the taxonomy (T13) explicit.

### Method findings

- **The recurring anti-pattern (`notes/quarantine-DO-NOT-READ/16E`):** excluding an edge or quadrant
  because it is irrelevant in *one* cell — then it returns via the reverse direction, the other phase,
  the other user, or the reliable-oracle case. Implies a standing *four-way exclusion check* before
  dropping any case, plus the rule "verify a claimed failure by tracing it; don't relay it."
- **The compaction-sourcing lesson (`notes/quarantine-DO-NOT-READ/16J`):** re-read the primary
  `README`/`DESIGN`/`KNOBS`/`TODO` after any context compaction. The round-16 notes are AI-authored
  working notes (trust less than primary docs), and even
  `notes/quarantine-DO-NOT-READ/spike/CLAUDE.md`'s `inv-*` slugs are AI-authored convention, not
  first-party ground truth.

### Open human ruling

- **The `kOOB` redline (dn-1).** Is an oracle's `oracle_kind=package` declaration "config-in-disguise" (a
  sidecar configuration form the `kOOB-in-band` lean forbids), or legitimate in-band sh? The spike held
  the exact oracle sh spelling (T7) as a strawman *pending this ruling*; it is an explicit open human
  decision, not resolved by the spike.

---

## 6. Inaccuracy index

No adversarial correctness pass was run on the notes (a separate whole-spike adversarial run is planned).
A prior review found the notes accurate as of their moment; nearly all apparent "wrongness" is
*supersession the append-only trail already encodes* (the CLI `--has` flag → stdin round-trip; the `skip`
→ `replace` rename; the predicted separate `probe` crate folding into `plan`; the "day-1" verdict-memo
never built; drifting test-counts) — evolution, not error, and not annotated.

- The `notes/quarantine-DO-NOT-READ/16A` header was mislabeled `# 170 —`; fixed in place to `16A` (a
  trivial mechanical typo).
- One mild, below-the-bar discrepancy: `notes/quarantine-DO-NOT-READ/16E`'s notes-index footer reads
  "ACTIVE, pending validation" while its body header reads "VALIDATED" (the note was edited in place).
  Noted for completeness; not a will-make-an-agent-do-wrong-things error.
