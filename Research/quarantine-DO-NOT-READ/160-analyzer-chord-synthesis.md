# 160 — analyzer-chord synthesis (build-2 functional surface)

> **Status (2026-06-05): spike working-note, round-16.** Synthesis of four
> clean-context corpus sweeps (engine+hazards · fact-model+grounding ·
> probe-compiler+cost+hooks · provenance+verdict+specimens) read across
> `021 040 041 052 055 071 073 074 076 077 090 092 093 094 095 096 099 09A 110 111`.
> Purpose: enumerate **every analyzer chord** so the Rust spike's `analysis`
> crate (build-2) explores the full state-space and doesn't under-scope. The
> sweeps' *overviews* are distrusted; their *chord enumerations + doc anchors*
> are the value and are reproduced/condensed here. Confidence-marked. Chord
> slugs are stable handles for later reference.

## 0. The result in one paragraph
The analyzer is a **composition machine over oracle-supplied knowledge**, not a
knower. It builds a whole-program CFG that must model a small **hazard set** or
be unsound; runs a **finite + distributive** dataflow (IFDS-shaped, decidable
floor) over a **flat fact domain** (k=0 context-insensitive — the EXPTIME
redline); anchors facts via an **oracle-declared, analyzer-internal kind-index**
(the dn-1 hinge, a 3-place kind/provider/equivalence relation, *not* a naming
convention); narrows them with **occurrence-typing** and **recency strong/weak
update gated on uniqueness** (the SF-1 keystone); grades every belief **MUST vs
MAY** (only MUST licenses a skip); and lowers the result into a **leaf-id-
preserving probe projection** whose every shortcut must **fail the conservative
way for its phase** (`kFAIL`). Everything unmodeled is **⊤ → reject/just-run**.

---

## 1. Seam map — which chords live in which crate

| crate | owns | key chords |
| --- | --- | --- |
| `core` (built) | shared vocabulary | Carrier(dn-7), ids, Span, OpaqueToken(W4), Phase, Verdict, Grade, Fact |
| `syntax` | lexer + recursive-descent parser → AST; ⊤-reject | lossless-enough AST (quoting-context + redirections first-class), ⊤-trigger classifier |
| `oracle` | the dn-1 hinge | kind-index, Tier-A blessed-forms recogniser, effect-class registry, latent-prop lifting |
| `analysis` | CFG + ShellEnvState + dataflow | the hazard set, IFDS engine, narrowing, strong/weak update, MUST/MAY licensing, the two soundnesses |
| `probe` | probe-compiler | leaf-seam, projection (faithful+optimized), VerdictMemo, content-key, probe-vs-just-run |
| `plan` | apply projection + plan/diff | minimized mutation set as sh, three-valued verdict + provenance UI feed |
| `hostsim` | DST host model | seeded state-machine host; synthesizes verdicts; the test substrate (no network) |
| `cli` | thin driver/UI | run the pipeline, print plan as sh + why-provenance |

CFG-construction and dataflow are **coupled** (errexit edges are an analysis
*output*, see haz-seterr) → `analysis` owns both as internal modules, not two
crates.

---

## 2. The hazard set — the CFG MUST model these or be unsound (021 §2 + 040)

- **haz-seterr** (subtlest; +SURE load-bearing) — `set -e`/`errexit` and
  `pipefail` *alter the CFG itself* (implicit exit-edge after ~every simple cmd),
  and can be toggled conditionally (`set +e … set -e`, even via `$-`
  introspection). ⇒ option-state is a forward dataflow fact
  (`errexit ∈ {on,off,⊤}`); each simple-cmd node conditionally sprouts an
  exit-edge gated on it. **No clean "build CFG then solve" split** — edge
  existence is partly an analysis output. (lockin: high — forces the coupled
  architecture.) *Gap:* the irregular real `errexit` semantics (suppressed inside
  `if`/`&&`/`||` conditions; command-substitution inheritance) are not spelled in
  the corpus — model carefully, verify against `dash`/`bash`.
- **haz-trap** — `trap` installs handler edges (EXIT/ERR/signal) from many nodes;
  conditional registration ⇒ path-dependent; `trap "$dyn" EXIT` ⇒ ⊤. EXIT (always)
  vs ERR (failure-only) distinction matters. v1 may conservatively join handler
  effects into the function effect-set rather than precise edge placement
  (elision-safe). **`trap` is a CONTRACT not a detector** (09A §4): its presence
  declares transient-cleanup; its absence licenses nothing.
- **haz-redir-as-mutation** — redirections are mutation sites *independent of the
  command word* (`: > /etc/x` truncates; here-docs write files). ⇒ node effect =
  `command-word ⊔ each-redirection-target`; redirection target is its own
  effect-source. (lockin: high — IR must make redirections first-class, not
  cosmetic children.)
- **haz-unquoted** (pervasive — ~80% of scripts) — unquoted expansion / word-
  splitting / glob changes arity & targets. ⇒ the word IR node must carry
  quoting-context losslessly (`quoted-literal | quoted-expansion |
  unquoted-expansion(may-split) | glob`); unquoted feeding an effect-target
  degrades it toward ⊤. (lockin: high — parser must preserve quoting; **likely
  dominates the ⊤-bound rate** → a `kPRECISION` knob to watch.)
- **haz-concurrency** — `&`, subshells `( )`, pipeline stages run in subshell
  envs: **env/var mutations don't escape, FS mutations do.** ⇒ effect facts carry
  an escape-class `{env-scoped, escaping}`; subshell-exit transfer projects out
  env mutations. Pipeline last-stage status is what `pipefail` keys on.
- **haz-whole-program-cfg** — CFG spans functions/`source`/aliases (supergraph),
  not per-file. Static `source` = known call edge; dynamic `. "$dyn"` = ⊤; alias
  = lexical macro resolved pre-CFG. (Required by design; *build-deferred* —
  intra-function first, but the IR must be cross-file-addressable from day 1.)

### The ⊤-trigger set (canonical; exhaustiveness here *is* probe-soundness)
`eval` · `alias` · dynamic `source`/`.` (`. "$dyn"`) · dynamic command names
(`"$cmd" args`) · recursive arithmetic (`$((b))` where `b` names another expr) ·
`[[ ]]` operands · lvalue-taking builtins (`unset "$expr"`, `printf -v`,
`${!ref}`, `test -v`) · unrecognized command with no oracle · non-distributive
blow-up · external/non-det read (clock/`$RANDOM`/network). **Fork (040:21 vs
021):** exclude at the *grammar* (parser rejects) vs model as a dataflow `⊤`. For
the spike: parser ⊤-rejects the syntactic members; dataflow ⊤ for the semantic
ones (no-oracle, non-det). The `Effect` lattice needs an **absorbing `Top`**.

---

## 3. The engine substrate (052/055/071) — decidable floor + the forks

- **eng-decidable-floor** (+SURE) — keep facts **finite** + transfer functions
  **distributive** (IFDS: finite+distributive ⇒ precise, polynomial, decidable).
  The establish/require/conflict dependency layer *is* gen/kill ⇒ distributive ⇒
  fits. Step off (infinite domain / non-distributive) ⇒ ⊤.
- **eng-ifds-supergraph** — exploded supergraph = CFG-node × `(D ∪ {0})`; transfer
  fns are micro-graphs (≤`(D+1)²` edges); composition = graph join; `0` = the
  always-reachable seed. "may-mutate" *is literally* Callahan's `may-modify`
  side-effect problem, solved by Tabulation in ~`O(E·D)` on h-sparse inputs
  (true for shell). **Build-deferred** to Tier-B; Tier-A (intraprocedural, ~90%)
  needs only plain monotone dataflow — *build that first*, reserve the supergraph
  seam.
- **eng-context-insensitive-redline** (high lockin; SAFETY boundary not a dial) —
  default **k=0**. k-CFA (k≥1) is EXPTIME *iff* the abstraction lets contexts
  combine closure-style; a **flat fact-map** stays polynomial (the k-CFA paradox).
  -GUESS-verify-in-spike that Dorc's domain stays flat; never add context until a
  flat pattern is confirmed. Baking in global context-sensitivity is fatal.

### FORK F-substrate (kFACTS; high lockin) — settle during spike
`kFACTS-materialized` (Datalog/Soufflé: extensibility + provenance + query-speed
"for free", 052) **vs** `kFACTS-on-demand` (IFDS/demand: compute only what's
queried — low memory, 071). The decisive tension: **052 sells Datalog's upside,
071 prices it** (materialization = RSS wall; recency abstraction non-optional —
TAJS 87%→<2% without it; slicing exponential). Resolution candidate:
**eng-hybrid-two-layer** = demand IFDS reachable-fact core + a bounded, coarse,
finite effect/MOD characterization layer (points-to-like, ⊤-on-overflow), sharing
one CFG/supergraph. 052:17 explicitly flags the hybrid "a real design decision for
synthesis." **Spike stance:** model facts as relations (so analyses are queries)
but hand-roll the solver (no Datalog dep — `inv-determinism` + no-dep core);
demand-driven; measure peak RSS if it ever matters (it won't at spike scale).

---

## 4. Fact model + grounding + narrowing (055/092/094/095/096) — the dn-1 theory

- **fact-pair** (dn-4) — `Fact = (opaque-token, source-expr)` + `FactDomain`
  (pkg/file/svc/user/port/mount) + optional **selector** (`svc:foo#enabled` — a
  package has `installed` AND `version`; structured per occurrence-typing's
  `car(p)` selector). Strong/weak update operates per-selector.
- **effect-class** — per-command transfer = its oracle's effect class:
  `PureQuery` (⊥; Salcianu–Rinard "mutating only new objects is pure") /
  `Mutating(FactSet gen/kill)` / `Unknown` (⊤). Dynamic command-word ⇒ can't look
  up ⇒ ⊤.
- **occurrence-triple** (the narrowing algorithm) — `(latent-proposition, object,
  substitution)`; narrowing = substitute the accessed object into the latent
  prop; polarity (`!`)/`&&`/`||` compose by negate/conj/disj. A command with **no
  latent proposition cannot narrow ⇒ ⊤ ⇒ run.** The **latent-prop is where the
  oracle anchor attaches** (`when_true`/`when_false` fact predicates).
- **SF-1 keystone = recency-lever ∧ cqual-strong-weak** (high lockin; build into
  the fact domain from the start):
  - *recency* (precision payoff) — strong-update the fresh entity a command
    touches ("pkg foo present@X now"), weak-update summarized ones. Without it,
    maybe-mutated proliferates → nothing skippable (87%→<2%).
  - *uniqueness gate* (the soundness half) — strong-update is legal **only on a
    provably-unique entity** (CQual linearity = Ramalingam's aliasing ceiling in
    operation); else weak-update (join, ⊤-ward). For sh: literal arg → likely
    unique; `$var`/glob/loop → maybe-aliased → weak. **This is the keystone whose
    misfire makes the floor unsound** (151 SF-1) — and it collapses to the same
    dn-1 strawman (what licenses "same entity").
- **qualifier-flow** — state-facts are flow-sensitive qualifiers on a *fixed*
  base (script structure frozen; only facts flow): `FlowState = Map<Entity,
  Qualifier>` propagated per program point. Qualifier lattice is **oracle-supplied
  per kind** (declare-anchor + infer-propagation = `kBURDEN`).
- **must-may** (the sound/unsound line) — `Grade::Must` (implied-by-structure via
  guard/occurrence-typing **or** oracle-declared) licenses a skip; `Grade::May`
  (mined/distributional) is a hint only, never elision. Mining is **offline
  oracle-bootstrap ranking**, never in the per-run elision path.
- **guard-carrier** (~SUSPECT, hypothesis-grade) — the idempotency guard
  `if ! PROBE; then ESTABLISH; fi` ≡ `PROBE || ESTABLISH` is the spec-carrier:
  shared *literal* arg = entity-link (intra-script only!), guard polarity =
  probe-vs-establisher. Normalize both forms to one `GuardedEstablish{probe,
  establish}` node.

### FORK F-kindindex (dn-1, THE hinge; high lockin) — settle FIRST, before engine
`cross-oracle-named-kind` + `kind-index`: cross-oracle identity must bind to a
**named kind**, never a shared token (token-equality is name-collision-prone
across oracles). No sh **naming convention** works — `pkg__probe` is a 1-place
namespace but the relation is **3-place (kind, provider, equivalence)**, and
sourcing apt's then brew's oracle would clobber. ⇒ an **analyzer-internal,
`kOOB`-legal index** lifted statically from author-written sh declarations:
`ProviderDecl { kind: KindId, provider: ProviderId, probe: …, establish: …,
entity_extraction: …, equivalence: … }`. 151's de-risk: lifting provide/
equivalence facts from oracle ASTs into an internal map keyed by leaf-id does NOT
violate `kOOB` (the redline is user-*config* form, not metadata transport). **The
open question is the sh idiom an oracle writes that we lift** — that is ph-1, the
dn-1 strawman. (See note 161.)

---

## 5. Probe-compiler (076/077) — projection rules + the leaf-seam

- **pj-elision-safe** (the discipline gating all rewrites) — a shortcut is legal
  iff it **fails toward the conservative end for its phase** (`Phase::Probe` →
  withhold; `Phase::Apply` → perform). Every optimizer pass carries this
  proof-obligation.
- **ls-wrappable-seam** (dn-3; high lockin) — output is `Vec<Leaf>` where
  `Leaf { id: LeafId, argv, env (carries DORC_LEAF_ID), wrapper: Option<…> }` —
  **never one opaque `sh -c`**. Stable `LeafId → AstId` back-map, dual-use
  (plan-provenance + future tracer). Both probe & apply runners share the seam.
- **ls-id-preservation** (the wo-1 resolution I'm adopting = **R1**) — projection
  is a CFG→CFG rewrite where every output node carries `stands_for:
  SmallVec<LeafId>` (1 un-batched, N batched) and elided leaves go in a side
  `elided` table — provenance **unconditionally recoverable**, cheap (a Vec/node).
  (R2 = best-effort + `--faithful` fallback; R1 dissolves wo-1, matches 111.)
- **pj-faithful-vs-optimized** — two projections from one CFG: `Faithful`
  (one-leaf-one-exec, guards preserved — the correctness oracle, **build first**)
  and `Optimized` (hoist/batch/drop). Differential-test Optimized ⊆ Faithful.
- **pj-probe-vs-just-run** (kPROBING; reserve the slot, defer the threshold) —
  per-leaf `ProbeDecision { Probe, JustRun, MustProbe }` from `(CostClass,
  Idempotent, Danger, Reach)`. Cheap idempotent (`mkdir -p`) → JustRun;
  dangerous/irreversible → MustProbe.
- **pj-guard-purity-precondition** (easy to miss; correctness gate) — a retained
  guard may gate an expensive check **only if** it is read-only AND evaluable
  against *initial* host state (no dependence on an unapplied upstream mutation).
  Else can't-keep-guarded.
- **vm-verdict-memo** (high lockin; the single most load-bearing data type) —
  `VerdictMemo { verdict: Verdict, content_key, freshness }`; `content_key` is
  **dependence-derived** (hash of the fact-slice the verdict reads — over-include
  is safe, under-key is an elision-soundness violation); `freshness` TTL gates
  reuse. Even with a mocked host, the compiler emits the key-derivation + verdict
  slot. **Cross-host memo reuse is deferred; the key+memo shape is day-1.**
- **vm-hermetic-oracle** (kVOLATILES, welded) — verdict = pure fn of
  *canonicalized* host-state; volatiles (timestamps/PIDs/ordering) excluded from
  the content-key. Determinism is the precondition for soundness AND memoization.

---

## 6. Result / provenance / verdict model (110/111) — graph-types-first (dac-B)

- **ch-graph-first** (dac-B; highest-leverage) — the provenance/error spine **IS**
  the analyzer's own dependency graph with payloads hung on `NodeId`; not a second
  graph. Agree node/edge types before building either layer.
- **ch-edge-types** — ≥3 never-conflated edge kinds: `DerivedFrom` (static
  transform), `RanOn` (distribution), `DependsOn` (dataflow). (PROV/OTel converge.)
- **ch-verdict** (dn-5) — three-valued `Verdict {Converged/Ok, Diverged/Fail,
  Unknown}` + staleness, **distinct from the diagnostic stream** (K8s Conditions ≠
  Events). `Unknown` folds conservatively.
- **ch-errnode + ch-cascade-suppress** — one explicit error/⊤ node-kind carrying
  {attempted-kind, salvaged children, best-guess}; it gets an absorbing `Unknown`
  type so downstream conclusions become unknown — **cascade-suppress in the
  lattice**, not by diagnostic-ranking.
- **ch-locator-list** (defer depth; reserve shape) — a surfaced annotation
  co-resolves a *variable-length list* of typed locators
  (`loc-host/loc-user-src/loc-probe/loc-surface`), never pre-flattened (loses
  precision to the coarsest tier). For the spike (no real hosts): UserSrc + Probe
  locators suffice; keep it a `Vec<Locator>`, not a fixed struct.
- **ch-controller-side + ch-fanin** — provenance reconstructs controller-side from
  delimited host output (hosts stay dumb, `kAGENTLESS` preserved); fan-in is a
  single aggregator that *links* sources — **never distributed consensus/merge.**

---

## 7. State model + shell-env (090/092/093/099) — the elision-soundness core

- **ambient-vs-transient** (W5; the core state typing) — `Stability {Ambient,
  Transient}`: ambient = stable resting value → probeable; transient =
  created+destroyed within a run → **un-probeable** (probing would mutate).
  Misclassifying transient-as-ambient = *the* wrong skip.
- **hoist-predicate** — hoist a guard to the probe **iff** the fact it reads is
  `ambient ∧ invariant` (no in-script gen/kill reaches it AND hermetic/no-TOCTOU).
  = reaching-definitions over the system-state store, oracle gen/kill = transfer
  fns. Conservative default: don't-hoist unless *positively* established.
- **undecidable-floor** — "is fact F stable T₀→guard?" is behavioural ⇒
  Rice-undecidable ⇒ **no sound transient-detector exists.** Default ⊤; rely on
  author contracts (`trap`) + positive-license, never detection.
- **shellenv** (the third state-kind, neither system-state nor program-vars) —
  `ShellEnvState { errexit, pipefail, noglob, ifs, cwd, fds, traps }` threaded as
  abstract state per program point. **subshell-scope**: `( )`/`$( )` push/pop a
  frame — `cd`/`set`/assignment inside doesn't escape (the inverse-transient).
  **swallow**: `cmd || true` suppresses the errexit exit-edge + flags best-effort.
- **floor-ceiling** (design invariant) — Floor = plain linear sh (never worse than
  not using Dorc); Ceiling = single minimal correct execution. The minimizer is a
  separable layer that *removes* work from the floor plan; `min_plan ⊑ linear`.
  Always keep a working linear fallback.

### Tier-A blessed forms (09A §3c) — analyzer recognises for everyone, no oracle
`PROBE || ESTABLISH` / `if ! PROBE; then ESTABLISH; fi` · `[ -f X ]`
(file-existence) · `<cheap> | grep -q <literal>` (artifact-has-property — a 2nd
existence form) · `[ A -nt B ]` (freshness) · `command -v X` (tool presence;
`which` rejected) · `set -euo pipefail` (entry-state init) · `( cd X && … )` /
`"$(cd … && pwd)"` (scoped) · `trap '…' EXIT` (transient-cleanup *contract*) ·
`cmd || true` (best-effort). Tier-B = the oracle-declared probe/establish commands
inside (`dpkg -s`, `apt install`, …) — no canonical form, by design.

---

## 8. Concrete sh test-inputs (from the specimens — real, commit-pinned)
Use as parser/analyzer fixtures, grouped by what they exercise:
- **strict-mode/env**: `set -euo pipefail`; `set +e … set -e` bracket; `$-`
  introspection toggle; `(cd "$d" && …)`; `REPO="$(cd … && pwd)"`.
- **traps**: `trap 'rm -f "$tmp"' EXIT INT`; conditional `if cond; then trap
  cleanup EXIT; fi`; EXIT-trap vs ERR-trap.
- **swallow**: `docker rm -f "$c" || true`.
- **guards (VALUE band)**: `if [ ! -f "$IMAGE" ]; then …`; `if ! readelf -S x |
  grep -q .gdb_index; then …`; `[ "$BUILT" -nt a.c ] && … && exit 0`;
  `command -v nginx || apt-get install -y nginx`; `case $(hostname) in pi-web*) …`.
- **transient brackets**: `[ "$(getenforce)" = Enforcing ] && setenforce 0; work;
  setenforce 1` (+ its trap-wrapped twin — analyzer must treat them **identically**,
  proving trap≠detector).
- **TOCTOU/heredoc**: atomic `build-temp + mv`; `cat <<EOF > img/sbin/init … EOF`
  (heredoc writes code → recognise the edge and STOP; generated code is data).
- **the strawmen** (`Research/strawmen/`, worktree 01Xpbd): `pi-webhost.straw.sh`
  (the do-4 book), `apt-get`/`nginx`/`systemctl`/`ufw` oracles. NB X4 found the
  dotted verb-ladder (`apt-get.check`) **fails `dash -n`** — a dn-1 finding.

---

## 9. The forks to settle DURING the spike (don't silently build past these)
- **fork-dn1** (ph-1, FIRST) — the oracle kind-naming/anchor/verdict sh idiom →
  the analyzer-internal kind-index shape. Note 161.
- **fork-substrate** (§3) — hybrid two-layer; spike = hand-rolled demand solver +
  relational fact modeling, no Datalog dep.
- **fork-wo1** (§5) — provenance through the optimizer; adopting **R1**
  (always-recoverable `stands_for` Vec).
- **fork-strongupdate** (§4, SF-1) — what licenses "same unique entity"; collapses
  to fork-dn1.
- **fork-errexit-semantics** (§2) — the irregular `set -e` rules; pin against
  `dash`/`bash` differentially.
- **fork-toptrigger-placement** (§2) — grammar-reject vs dataflow-⊤; spike does
  both (syntactic at parser, semantic at dataflow).

## 10. Build sequencing (demand-driven; Tier-A first)
`core`(done) → **ph-1 dn-1 strawman (note 161)** → `syntax` (~10% sh: the
strawman book's constructs) → `analysis` intraprocedural monotone dataflow +
ShellEnvState + hazard set (Tier-A) → `oracle` kind-index + Tier-A forms +
effect-class → narrowing/strong-weak/MUST-MAY → `probe` faithful-then-optimized +
leaf-seam + VerdictMemo → `plan` + `cli` → `hostsim` DST woven throughout (built
early as the test substrate). **Deferred/executor-only** (mock or skip): async
executor, fan-out tree, cross-host memo *reuse* (keys are day-1), scheduler,
seccomp backstop, tier-3 profile-guided cost, the IFDS interprocedural supergraph
(Tier-B), persistence of the verdict cache.

## 11. Load-bearing raw-read anchors (if I need depth later)
`021:39-45` hazard set · `040:14-21` ⊤-set · `052:17` hybrid fork · `071:11-12`
k-CFA redline · `071:16-18` datalog RAM wall · `055:14-26` two-soundnesses ·
`055:33`+`092:124-126` SF-1 · `092:89-113` occurrence-triple · `096:19-31`
MUST/MAY · `076:§4b` content-key/hermetic · `077:9-23` leaf-seam + wo-1 ·
`111:§0` dac-A/B/C/D · `090:§0-0.5` ambient/transient + floor/ceiling · `092` whole
(shellenv) · `09A §3c` Tier-A ledger.
