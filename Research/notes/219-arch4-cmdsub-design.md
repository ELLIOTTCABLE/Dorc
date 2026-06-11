# 219 — arch-4: command-substitution `$()`, design-FIRST note + recommendation

> Round-21 spike design note, append-only. arch-4 (note 211 §1): the command-substitution
> slice. Design-FIRST — this is a note + recommendation, NOTHING is built. AI-authored,
> confidence-marked, file:line-cited, strawman-sh-grounded. Trust README/DESIGN + the human
> rulings + 19H/19I over this. Builds on 209 brk-4 (the subject), 20E (the Query effect-class
> this would extend), 203 §1 (kFLATTEN/dir-timed-probe — the human's reserved CFG-into-probe
> aspiration), 211 §1 (the arch-4 charter). All line-citations are against the worktree at
> `92162f1` (`Temp/dorc-r21/cmdsub`).

---

## Executive summary (5 lines)

1. **Current `$()` handling is structurally complete but SILENTLY degrading**: the parser lowers `$()` bodies into real sub-ASTs; the CFG marks them `expansion_internal` (effect-bearing non-leaves); the value plane collapses any `$()`-bearing word to `⊤` (`recipe_of_word`→`Recipe::Top`); the command classifies `Opaque`. **No diagnostic fires anywhere** on this path — the find-3 no-silent-phantoms lesson is currently *violated* for `$()`.
2. **The minimum slice (q-2)** is one short build: emit an honest `Note`/`Warning` diagnostic at every site a `$()` forces `⊤` (the value-plane collapse, the Opaque classification, and the probe `skip-unresolvable`), each naming WHAT it blocked. This is the floor; it is pure-additive and risk-free.
3. **The prize (q-3) — the Query-shaped `$()`** (`v=$(getent group docker)`) is real but NOT free: the Query effect-class machinery (20E) is the right *template*, but it is built for **guard-position rc-consumers**, while `$()`-capture consumes **stdout**, which has **no producing wire** — the `OutClaim`/`stdout=` channel is reserved-but-inert end-to-end.
4. **The seams (q-4) BEND for keying but RESIST for value-carriage**: the site-keyed record lane + `stdout=` reserved key + `OutClaim` newtype mean a value-bearing record is *representationally* anticipated, but the inner-`$()`-command is a `expansion_internal` non-leaf with no `LeafId`, no probe invocation, and no record-grammar that survives multi-line/binary stdout — three concrete re-architecture costs.
5. **Recommendation (q-5): build q-2 only this round; design q-3, do not build it.** The prize needs a human ruling on a genuine fork (is a probe-captured stdout a NEW claim-type — door-2's counterfactual flavour — or an ordinary probe-observation?) plus the stdout-producing wire, the non-leaf→leaf promotion, and a record grammar that carries arbitrary text. That is a multi-wave program, not a short build task.

---

## q-1 — EXACT current `$()` handling, file:line-traced

I traced the full pipeline. The headline: **`$()` is handled coherently at the *structural* layer and collapses to `⊤` at the *value* layer, but every value-layer collapse is SILENT** (no `Diagnostic`). Below, by stage.

### q-1.a Parser — `$()` lowers to a real sub-AST (NOT a ⊤-trigger)

- `WordPart::CommandSubst(AstId)` — a word fragment whose body is a nested `List`/`Script` node (`crates/syntax/src/ast.rs:216-217`). +SURE.
- `Parser::parse_subst_body` re-lexes the inner text and recurses on the SAME builder into a nested `List` node, saving/restoring the outer token stream (`crates/syntax/src/parser.rs:1334-1343`). So the inner commands exist as a fully-parsed CFG-analyzable subtree. +SURE.
- `lower_part` maps `LexPart::CommandSubst(inner)` → `parse_subst_body` → `WordPart::CommandSubst(id)` (`parser.rs:1318-1321`). +SURE.
- **`$()` is NOT in the ⊤-trigger set.** The syntactic ⊤-triggers (`eval`, dynamic command-name, `. "$dyn"`, `$(( ))` in command position, lvalue-builtins, residual loop shapes, `&`) are enumerated at `crates/syntax/CLAUDE.md §K` and `ast.rs:296-313` (`UnsupportedReason`). A bare `$()` in a word is none of them — it parses clean. +SURE. The ONE `$()`-adjacent parser ⊤-reject is a `for`-LIST word containing a command-substitution (`parser.rs:803-812` `word_has_expansion_effect`, → `UnsupportedReason::Loop`, HOLE#1) — that is loud (an `Unsupported` node + `Error`), but it is the loop-list case, not the general `$()`.
- **Provenance debt** (`tn-coarse-subst-provenance`, `syntax/CLAUDE.md`): inner-parse diagnostic spans are relative to the inner text, not the outer source (`parser.rs:1330-1332`). Accepted-coarse for the spike. ~SUSPECT this bites the prize later (a probe-shipped inner command's diagnostics would mis-locate).

### q-1.b Value plane — a word containing `$()` resolves to `⊤`, UNCONDITIONALLY

This is the load-bearing answer to "a word containing `$()` resolves to…?": **`ValueOf::Top`**, always. The chain:

- `recipe_of_word` flattens a word's parts via `collect_frags` (`crates/analysis/src/value.rs:976-986`). +SURE.
- `collect_frags` calls `sem::classify_frag` per part; for a `WordPart::CommandSubst(_)` the class is `FragClass::OpaqueValue` (quoted) or `FragClass::SplitRisk` (unquoted) (`crates/syntax/src/sem.rs:180-188`). +SURE.
- In `collect_frags`, `OpaqueValue` returns `false` (whole word ⊤), and a `SplitRisk` that is not a plain-`$name` (`split_var_name` returns `None` for a command-subst — `value.rs:1054-1061`) ALSO returns `false` (`value.rs:1006-1013`). So **both quoted `"$(cmd)"` and unquoted `$(cmd)` collapse the word to `Recipe::Top`**. +SURE.
- `Recipe::Top` → `resolve_recipe`/`resolve_recipe_fields` → `Abstract::Top` → `ValueOf::Top` (`value.rs:883-901`, `923-965`, `intern_argv` at `183-191`). +SURE.
- The assignment case is identical: `v=$(cmd)` — the RHS word's recipe is `Recipe::Top`, so `apply_assigns` stores `Flat::Top` for `v` (`value.rs:857-863`, `recipe_of_word` via `Prep::new` at `267-275`). So **the captured variable `v` is `⊤` for all downstream uses** — exactly the `exec-opaque-var-runs` anchor (`PKG=$(cat /etc/pkg)` ⇒ `PKG=⊤` ⇒ `install "$PKG"` operand ⊤). +SURE.

Strawman, grounding the collapse:
```sh
PKG=$(cat /etc/pkg)        # value-plane: PKG ↦ Flat::Top
apt-get install -y "$PKG"  # argv: [Literal(apt-get), Literal(install), Literal(-y), Top]
```

### q-1.c CFG / effect classification — inner commands are effect-bearing NON-leaves

- `lower_simple` lowers each assignment-value AND each command word's `$()` bodies as scoped regions, *sequenced before* the `Command` node (`crates/analysis/src/cfg.rs:599-607`). +SURE.
- `lower_word_substs` emits, per `$()` body: `ScopeEnter` → body → `ScopeExit`, and marks every body node `expansion_internal = true` (`cfg.rs:1024-1044`, the marking at `1035-1037`). +SURE.
- `expansion_internal` semantics: the inner commands **stay in the effect dataflow** (they reach-def, so they poison/establish) but are **excluded from the plan/apply leaf set** (`cfg.rs:138-140`, `192-197`; `is_expansion_internal`). +SURE. Pinned by `crates/analysis/tests/cfg.rs:540` (`command_substitution_body_is_expansion_internal_subshell_body_is_not`) and `:877` (`find6_command_substitution_regions_and_assignment_fallibility`).
- A `$()` opens a subshell **scope**: env/var/cwd mutations inside DO NOT escape, FS mutations DO (`cfg.rs:82-88`, `ScopeEnter`/`ScopeExit`; `compute_scope_clobbers` forces every var assigned inside a scope to `⊤` at the exit — `value.rs:308-316`, `1063-1096`). +SURE. (Consequence: `x=1; y=$(x=2; echo hi); echo $x` — the inner `x=2` is clobbered to ⊤ at ScopeExit, so `$x` downstream is ⊤, not `1`. Sound, imprecise.)
- **Effect classification of the inner command**: it runs `command_effect` like any `Command` (`cfg.rs` keeps it a `Command` node). For the `exec-opaque-var-runs` case the inner `cat /etc/pkg` is un-oracled ⇒ `Opaque` (`effect.rs:152-157`, no check resolves) ⇒ `Reach::Top` ⇒ poisons all downstream ambient-ness (`effect.rs:463-464`). +SURE. For `exec-subst-body-nonleaf` the inner `apt-get install -y nginx` IS oracled ⇒ `Establishes(package:nginx#installed)` — it gens its cell into Reach, but because it is `expansion_internal` it is **dropped from the classify output leaf-set** (`effect.rs:574-579`, the `continue`). +SURE.
- **Outer command (the one whose argv contains `$()`)**: classifies on its OWN argv. `echo "...$(...)..."` — `echo` is a target-state-pure builtin (`effect.rs:285-304`, `is_target_state_pure_builtin` includes `echo`) ⇒ the OUTER is `Pure`. But `PKG=$(...)` then `apt-get install -y "$PKG"` — the install's operand is ⊤ ⇒ `Opaque` (`effect.rs:106-108`/`132`). +SURE.

### q-1.d Consumption — `v=$(cmd)` marks WHICH channels?

**Answer: NONE on the inner command, beyond the subshell scope.** The consumed-observable marking (`mark_consumed_range`, `cfg.rs:1419-1455`) marks an enclosing context's consumed channels onto inner leaves — but it is driven by *redirections* (`output_redir_observables`, the `( install ) > f` kill-shot at `cfg.rs:855-857`), not by `$()`-capture. There is **no code path that marks the inner `$()` command's `Stdout` as consumed-by-the-capture**. ~SUSPECT this is a latent gap but it is currently *harmless*: the inner command is `expansion_internal` ⇒ never a leaf ⇒ never reaches the consumption gate (`consumption_ok`, `plan/lib.rs:453-475`) anyway, because it never becomes a `Replace` candidate. So the question "which channels mark on `v=$(cmd)`" has the answer "the capture's stdout-consumption is NOT modeled, but it doesn't matter today because the inner command is unconditionally run-verbatim (non-leaf)." +SURE this is the current behavior; ~SUSPECT it becomes load-bearing the moment the prize tries to promote the inner command to a leaf (q-3).

### q-1.e Disposition / render today

- The inner `$()` command is **never** a `Step` (it has no `LeafId` — `compile_probe`/`build_plan` only walk classify's leaf output, which excluded it). So it appears in the apply artifact ONLY as the verbatim bytes of its enclosing line. +SURE.
- `exec-subst-body-nonleaf/expected.out`: the apply renders `echo "installed: $(apt-get install -y nginx)"` **verbatim** (the `echo` is Pure ⇒ `Run`), so the inner nginx install runs as a side effect of the `$()`. The `curl` install below is `Replace`d (`true`, converged). `ran:` = only `apt-get install -y nginx`. +SURE — this is the second current-behavior anchor (the first being `exec-opaque-var-runs`).
- The span-edit render (`render_apply`, `plan/lib.rs:1210-1213`, `collect_edits` at `1266-1300`) substitutes a leaf's command byte-span with a `StandIn`. Since the inner `$()` command is not a leaf, its span is never an edit target — it is carried inside the enclosing leaf's verbatim bytes (if the enclosing leaf runs) or inside the enclosing leaf's span (if the enclosing leaf is itself replaced, the whole `v=$(...)` line becomes `true`/`(exit n)` and the inner command vanishes with it). +SURE.

### q-1.f The ⊤-diagnostic story — EVERY silent degradation spot (find-3 no-silent-phantoms)

This is the most important q-1 finding and the direct motivation for q-2. **`$()` forces `⊤` at multiple points, and EVERY one is silent** (no `Error`, `Warning`, or `Note`):

- **silent-1 — value-plane collapse**: `recipe_of_word`→`Recipe::Top` for a `$()`-bearing word emits NO diagnostic. `value.rs` emits **zero** diagnostics anywhere (`grep`-verified: no `diags.push`/`Diagnostic::`/`Carrier` in `crates/analysis/src/value.rs`). +SURE.
- **silent-2 — Opaque classification**: `command_effect` returns `[Opaque]` for a ⊤ operand (`effect.rs:132`) or ⊤ command-word (`effect.rs:106-108`) with NO diagnostic. The ONLY diagnostic `effect.rs` emits is `KIND_DISAGREEMENT` (a warning, `effect.rs:74`, `257`), which fires only on an annotation/effect-map kind mismatch — never on a `$()`-induced Opaque. +SURE.
- **silent-3 — probe `skip-unresolvable`**: an Opaque site becomes `unresolvable` (`plan/lib.rs:857`) and renders as a `# site:N skip-unresolvable` COMMENT (`render.rs:204-207`). A comment in the artifact is *transparency*, but it is NOT a `Diagnostic` to stderr — the cli `report()` (`cli/main.rs:48`, stage reports at `105-133`) never sees it. So a human running `dorc` gets **no stderr signal** that a `$()` blocked an elision. +SURE.
- **silent-4 — the inner mutator vanishing under a replaced enclosing leaf**: if `v=$(apt-get install nginx)` had its enclosing assignment leaf replaced (it can't today — an assignment-only command with a ⊤ RHS classifies `Pure`, MustRun-ish — but the *shape* is the hazard), the inner install would silently disappear. Not currently reachable, but the prize must guard it. -GUESS (not reachable at HEAD; flagged as a shape-hazard for q-3).

Contrast — the LOUD ⊤ paths (so the asymmetry is clear): `eval`, dynamic command-name, `for`-list-`$()`, depth-bound all emit an `Unsupported` node + `Error`/`Warning` (`parser.rs` ⊤-rejects; `cfg.rs:457`, `995` `CFG_TOP`). **`$()`-in-a-word is the one common construct that degrades to ⊤ with no diagnostic at all.** +SURE. This is precisely the find-3 violation q-2 closes.

Other current-behavior anchors found (beyond `exec-opaque-var-runs`):
- **`exec-subst-body-nonleaf`** — oracled mutator inside `$()`, enclosing `echo` Pure ⇒ inner mutator runs verbatim, never independently elidable. (THE second anchor; see q-1.e.)
- **`case "$(hostname)"`** — the case scrutinee `$(hostname)` lowers to a scoped subst region (`cfg.rs` `lower_word_effects`→`lower_word_substs`; pinned `tests/cfg.rs:128`, `156-159`). The scrutinee value is ⊤, so case-arm matching cannot statically resolve — every arm stays live. 209 brk-4 calls this "the same eventual story" as the capture case (probe + collapse). +SURE structurally; the precision is the prize.
- **`exec-top-arith-in-arg-ok`** (`$((1+1))` sibling) — `echo "sum is $((1 + 1))"`: the `$(( ))` collapses the word to ⊤ identically, but `echo` is Pure so it doesn't matter; the install below elides. Confirms the ⊤-collapse is uniform across `$()`/`$(( ))`/`${x:-y}` (all `OpaqueValue`/`SplitRisk`, `sem.rs:182-188`). +SURE.

---

## q-2 — The minimum slice: honest, specific ⊤-diagnostics everywhere `$()` blocks something

This is the floor the round commits to. It is **pure-additive** (emit diagnostics; change no disposition), sized for one short build task. Concretely:

### q-2.a Where to emit (code sites)

- **site-A — the value-plane collapse** (`crates/analysis/src/value.rs`, `recipe_of_word`/`collect_frags`): `value.rs` currently threads no `Carrier`/diagnostics. The cleanest minimal home is NOT value.rs (it would need a diagnostics channel it doesn't have, a bigger change) but **`effect.rs::command_effect`**, which already takes `diags: &mut Vec<Diagnostic>` (`effect.rs:97`). At the two Opaque-return points where the cause is a ⊤ operand/word, emit a diagnostic. ~SUSPECT this is the right altitude: `command_effect` is where the ⊤ *becomes consequential* (Opaque ⇒ runs), and it already has the diagnostics sink. To name `$()` specifically (vs. an unassigned `$X` ⊤), `command_effect` would need to know WHY the word is ⊤ — which it does not today (it sees `ValueOf::Top`, cause-erased). So either (i) emit a generic "operand is runtime-dynamic ⇒ site runs" diagnostic (cheap, honest, but doesn't single out `$()`), or (ii) thread a ⊤-CAUSE tag from the value plane (`ValueOf::Top` → `ValueOf::Top { cause }`) so the message can say "command-substitution `$(…)`". **tc-fork (flagged, q-5)**: generic-⊤-diagnostic (cheap, ships this round) vs. cause-tagged-⊤ (precise, a `ValueOf` reshape touching value.rs + every consumer). For the *minimum* slice, (i) is the floor; (ii) is a stretch.
- **site-B — the probe unresolvable**: `compile_probe` already records `unresolvable` sites (`plan/lib.rs:855-857`). The cli could `report()` these to stderr (it has the `ProbePlan`, `cli/main.rs:139`). This is the most *visible* fix — one `eprintln!` loop over `probe.unresolvable` naming each site's source text — and needs no kernel change (it is a cli-edge readout, like `--debug-argv` at `cli/main.rs:170-176`). +SURE this is the cheapest high-value piece.

### q-2.b Diagnostic codes + what each names

Proposed (greppable, `ch-catalog` style — these are *suggestions*, the human owns the catalog):
- `dq-cmdsub-operand-top` (Note/Warning): "operand `<argv-text>` is a command-substitution `$(…)` / runtime-dynamic value ⇒ identity unresolved ⇒ site `<leafid>` runs (never elided)". Names the site + the blocking word. Emitted at site-A (or its generic form).
- `dq-cmdsub-inner-nonleaf` (Note): "command `<inner-text>` runs inside a `$(…)` substitution ⇒ effect-bearing but not independently elidable (runs whenever its enclosing line runs)". Names the inner mutator. This is the `exec-subst-body-nonleaf` disclosure — the inner nginx install is invisible today; this surfaces it. Emitted where classify drops the expansion-internal leaf (`effect.rs:577`) — it has the node + can resolve its text.
- `dq-site-unresolvable` (Note, cli-edge): the stderr echo of `probe.unresolvable`, naming each site's source command. Emitted in cli `report`-style.

### q-2.c Sizing

One short build: ~1 diagnostic code + emit-site in `effect.rs` (site-A generic form), ~1 in classify's leaf-drop (`dq-cmdsub-inner-nonleaf`), ~1 cli stderr loop (site-B). No disposition changes ⇒ **zero golden churn on `expected.out`** (the artifacts are unchanged; only stderr/diagnostics grow). The e2e harness diffs stdout artifacts (`spike/CLAUDE.md` "Build / test / run"), so the floor slice is invisible to the 66-case corpus's `expected.out` — it would want a new assertion mechanism (a `expected.diag`-style check) OR just unit tests at the classify layer (cheaper, and matches the "brutal integration tests" posture). ~SUSPECT unit-test-only is the right cost for the floor.

---

## q-3 — The prize: the Query-shaped `$()` (`v=$(getent group docker)`)

The dream (209 brk-4, +SURE it is the right *eventual* story): a `$()` whose VALUE arrives by probe — ship the inner read-only command in the probe, capture its stdout, fold the concrete value back into the apply so the captured variable becomes a known literal. This is genuinely valuable (it un-⊤s `case "$(hostname)"`, `pkg=$(detect-pkg-mgr)`, group-membership captures). But it is **far harder than the guard-Query (20E)** because it consumes **stdout**, not **rc**. Below, each sub-question.

### q-3.a What licenses the inner command into the probe?

The guard-Query template (20E) licenses a command into the probe iff it is a **declared Query** — the oracle's `oracle_effect <provider> <verb> query <selector>` (`20E §0`, `crates/oracle/...` `Polarity::Query`). The same gate is the only sound one here: **the inner `$(cmd)` is probe-eligible iff `cmd` resolves (through its oracle's `check()`) to a `CommandEffect::Queries(fact)`** — i.e. the oracle self-vouches it as read-only. +SURE this is the gate (it is the *exact* self-vouch carve-out the human welded: probe-inertness comes ONLY from structural vouching, `spike/CLAUDE.md` "Standing human rulings"; an oracle vouches its own command by existence, `DESIGN.md:505-512`).

- What gates it OUT: an un-oracled inner command (`cat /etc/pkg`) ⇒ Opaque ⇒ **NOT** probe-eligible ⇒ the site stays ⊤ (kFAIL-withhold: never ship a non-vouched command into the read-only probe). This is the `exec-opaque-var-runs` behavior, *preserved* — the prize does NOT rescue it. +SURE.
- The `oracle_probe_*` mechanism (`crates/oracle/src/lib.rs:84-94`, `resolve_probe` at `234`) is what supplies the actual read-only body. For a guard-Query, `command -v -- "$tool"` IS the probe (the check is the probe). For a value-capture, `getent group docker` IS the probe — the inner command itself, shipped verbatim. ~SUSPECT the cleanest model: a value-capture Query's "probe body" is the inner command's own resolved argv (it is already read-only-vouched), captured for stdout — distinct from a guard-Query, whose probe body is the `oracle_probe_*` declaration. **tc-fork (flagged): is the value-capture's probe the INNER COMMAND verbatim, or a separate `oracle_probe_*`-style declaration?** Verbatim is more faithful (it captures what the book actually does) but ships book-author code into the probe; a declaration is the established pattern but adds an authoring burden.

Strawman (the GOOD path):
```sh
# oracle declares getent-group as a read-only Query producing a value:
oracle_kind=group-membership
oracle_effect getent group query members      # rc-class today; the prize wants stdout-class
oracle_probe_group_membership() { getent group "$1"; }   # NB: captures STDOUT, not just rc

# book:
members=$(getent group docker)   # inner getent is a declared Query ⇒ probe-eligible
case "$members" in *:docker:*) echo "docker exists" ;; esac
```

### q-3.b The record-grammar extension (stdout is multi-line / arbitrary)

This is the **hardest concrete problem** and the reason the prize is not a short build. The current record grammar is single-line, token-delimited:
```text
site <leafid> effect=<holds|absent|cant-tell> rc=<n>
```
(`render.rs:118-125` header; `record_scaffold` at `186-195`; cli `parse_results` at `cli/main.rs:409-453`). It is **whitespace-and-newline-tokenized** (`it.next()`/`strip_prefix` over `line.split_whitespace`-style iteration, `cli/main.rs:431-441`). A captured stdout value is **multi-line and arbitrary-byte** (`getent group docker` ⇒ `docker:x:999:alice,bob`; `cat /etc/os-release` ⇒ many lines). The reserved `stdout=<text>` key (`cli/main.rs:436-437`, `render.rs:106-110`) is **token-shaped** — it breaks on the first space or newline in the value. +SURE this is a real wall.

Three options, ranked by my confidence they are sound:
- **opt-base64 (~SUSPECT best)**: the probe emits `stdout=<base64(value)>`; the cli `OutClaim` stores the decoded text. base64 is single-line, byte-safe, dash-expressible (`base64` is not POSIX but widely present; OR a pure-sh hex encoder — but that ships non-trivial code into the probe). Cost: the probe wrapper grows an encode step; the record stays one line. **fork: base64 (needs a `base64`/`openssl`/`xxd` on the host — a probe DEPENDENCY, breaks "ship-and-run-anywhere") vs. a pure-sh hex encode (heavier probe body, the `dq-reflexive-probe-inertness` closure-check must bless it).**
- **opt-length-prefix (-GUESS)**: a multi-line record framing `site <id> stdout-bytes=<n>` then `<n>` raw bytes. Robust but the cli `parse_results` is strictly line-oriented (`for line in input.lines()`, `cli/main.rs:411`) — this is a parser re-architecture, not an extension. +SURE it resists the current parser.
- **opt-refuse-non-text (the floor)**: ship the Query, but if the captured stdout is multi-line or non-UTF8, REFUSE the fold (the site stays ⊤ ⇒ runs). Only single-line ASCII captures (the `hostname`, the `detect-pkg-mgr` one-word case) fold. ~SUSPECT this is the right FIRST prize-slice — it handles the high-value one-line cases (`$(hostname)`, `$(uname -m)`) without a binary-safe wire, and degrades safely (kFAIL-perform) on everything else. The reserved `stdout=` token-key works as-is for single-token values.

### q-3.c inv-probe-sourced-values fit + freshness/TOCTOU posture

The captured value has **genuine probe provenance** (it is the actual stdout the read-only probe produced on the real host) — so it satisfies `inv-probe-sourced-values` (`spike/CLAUDE.md`: a replacement may reproduce ONLY values with probe-provenance; no fabricated defaults). +SURE this is the GOOD path — unlike a synthesized default, a probe-captured stdout traces to a concrete observable. The freshness/TOCTOU posture is **identical to every other probe fact**: the capture is as-of-probe-time, and probe→apply staleness is the standing WONTFIX (`spike/CLAUDE.md` "Standing human rulings": TOCTOU deferred-to-actively-WONTFIX; do not build re-probe-before-apply). So the prize inherits the same TOCTOU exposure as a converged-establish elision — no worse, no special handling. +SURE. (203 §1's dir-timed-probe is the reserved *future* refinement, not this.)

### q-3.d kFAIL-withhold fit

An unvouched inner command (no oracle, or the oracle does not declare it a Query) ⇒ the site stays `⊤`, the probe ships nothing for it, the apply runs the enclosing line verbatim. +SURE this is the existing behavior (`exec-opaque-var-runs`) and it is the correct kFAIL-withhold floor: the prize ADDS capability for vouched captures and changes NOTHING for unvouched ones. The asymmetry from the guard-Query is preserved: a Query mutates nothing, so withholding is always safe.

### q-3.e Where superposition collapses

Per `inv-superposition` (`spike/CLAUDE.md`; `analysis/CLAUDE.md`): the **kernel emits the site** (classify would emit a new `SkipClass::CaptureResolvable { fact, valid }`-shaped fact — the phase-/orientation-agnostic "this `$()` capture is probe-resolvable"), and the **phased caller decides** probe-inclusion (forward: `compile_probe` ships the capture-probe) vs. apply-fold (`build_plan`/`fold` substitutes the captured value). +SURE this is the established split (it mirrors `QueryResolvable`'s emit-in-classify / collapse-in-caller, `effect.rs:631-636` + `plan/lib.rs:305-310`). The new wrinkle: the captured value must flow from the probe-record (cli `facts_from_sites`, `cli/main.rs:281-310`) into the value plane's `ValueOf` for the *enclosing* uses — i.e. the fold needs to RE-RUN value-propagation with `v` bound to the captured literal. That is a NEW data dependency (value-plane ← probe-record) that does not exist today (value-flow runs BEFORE the probe, `analysis/value.rs`; the probe runs after). ~SUSPECT this is the deepest architectural cost — a second value-flow pass post-probe, or a fold-time substitution that bypasses the value plane.

### q-3.f What the apply RENDER does with a folded `v=$(cmd)`

Options:
- **render-substitute-assignment (~SUSPECT cleanest)**: substitute the assignment's RHS span — `v=$(getent group docker)` → `v='docker:x:999:alice,bob'` — quoting the captured value via `sem::single_quote` (`render.rs:167`, the lone quoting home). The span-edit machinery (`collect_edits`, `plan/lib.rs:1266-1300`) substitutes a command's byte-span, so editing the assignment leaf's span to `v='captured'` is *expressible* — BUT: the assignment is currently classified `Pure` (an assignment-only command, `effect.rs:101-103`), not a Replace candidate, so it never reaches `collect_edits` as an edit. The prize must promote a capture-assignment to a Replace-eligible leaf. +SURE this is a new disposition path.
- **multi-line values**: `sem::single_quote` of a multi-line value is dash-clean (single-quotes preserve newlines), so `v='line1
line2'` is valid sh — but the provenance-comment machinery flattens interior newlines (`render.rs:307-315`, `provenance_comment`), and the span-edit `emit_span_edits` collapses multi-line edits onto one line (`plan/lib.rs:1353-1378`). ~SUSPECT a multi-line captured VALUE in the substitution itself is fine (single-quoted), but the surrounding line-walk assumes single-line replacements — needs care. This reinforces opt-refuse-non-text (q-3.b) as the right first slice.
- **downstream word-splitting of the bound value (the 20N split machinery)**: once `v='docker:x:999:alice,bob'` is bound, a downstream unquoted `$v` field-splits under default IFS (`value.rs:903-965`, `resolve_recipe_fields`, the brk-3 split path). So the captured value re-enters the EXISTING split machinery — which is GOOD (it is already modeled) but means the captured value's split behavior must be reproduced faithfully (a glob char in the value ⇒ ⊤, `value.rs:959-964`). +SURE the split machinery exists; ~SUSPECT the capture→split interaction is an untested cell.

### q-3.g Failure modes ranked (highest-risk first)

- **fm-1 (highest) — capture-vs-mock divergence in e2e**: the probe-exec gate is STILL not a gate (20E §9: the probe is `dash -n`-checked, not executed under shims; records are hand-authored in `probe-results.txt`). So a capture-Query's stdout would be FIXTURE-authored, not produced by running the probe — the 19I §3 trap's residual. A wrong fixture stdout ⇒ a wrong fold, undetected. +SURE this is the worst exposure; the prize makes the probe-exec gate a hard prerequisite (a captured value that feeds the apply is far more load-bearing than a guard rc).
- **fm-2 — stale capture (TOCTOU)**: the captured value is as-of-probe-time; the apply binds it as a literal. Same WONTFIX posture as every probe fact (q-3.c), but the BLAST RADIUS is larger — a stale captured `$(hostname)` or `$(detect-pkg-mgr)` mis-routes the whole downstream `case`. +SURE; priced, accepted-as-WONTFIX, but worth a louder disclosure than a converged-establish.
- **fm-3 — downstream word-splitting** (q-3.f): an unquoted `$v` use re-splits the captured value; a glob char in the value globs against the apply-host fs. Handled by the existing split machinery (degrade-to-⊤ on glob), but an untested cell. ~SUSPECT.
- **fm-4 — multi-line / binary stdout breaks the record wire** (q-3.b): the token-shaped grammar mangles it ⇒ a silent wrong-parse unless refuse-non-text gates it. +SURE; the q-3.b option choice IS this mitigation.

Type sketches (NOT code):
```
// core: the value-capture observable channel already exists in shape:
//   Observable.stdout: Predicted<OutClaim>   (core/lib.rs:419 — RESERVED, always ⊤ today)
// classify would add:
SkipClass::CaptureResolvable { fact: FactKey, valid: bool }   // mirrors QueryResolvable
// the record would carry (already reserved):
//   site <leafid> stdout=<base64-or-single-token>            (cli/main.rs:436 — parsed, never produced)
// the fold/value re-bind:
//   ValueFlow needs a post-probe re-resolution: v ↦ Flat::Elem(captured)   (NEW dependency)
```

---

## q-4 — Seams: does the probe-record grammar / site-keying bend, or resist?

**Mixed: site-keying + the OutClaim representation BEND; the value-carriage wire + the non-leaf→leaf promotion RESIST.**

### q-4.a What bends (cite code)

- **Site-keying bends fully.** The record lane is keyed by command-SITE (`inv-site-keyed-results`, `plan/lib.rs:758-783` `site_order`; `RecordKey { site, member }` at `cli/main.rs:356-360`). It already carries a sub-key for in-loop members (`site N.M`, `render.rs:90-95` `site_key`, task-L2 item-4). A capture-site is just another `LeafId` — the keying needs no change. +SURE.
- **The OutClaim representation bends — it was BUILT for this.** `OutClaim(Symbol)` (`core/lib.rs:303-304`), `Observable.stdout: Predicted<OutClaim>` (`core/lib.rs:419`), and the cli `stdout=` parse-and-store (`cli/main.rs:436-437`) ALL exist as reserved-but-inert shape. The doc is explicit: "the newtype exists so a future stdout-producing probe is a value-plumbing change, not a representation change" (`core/lib.rs:300-302`); "a future stdout-producing probe is a value-plumbing change, not a grammar change" (`cli/main.rs:374-377`). +SURE — the representation seam is *deliberately* pre-opened for exactly the prize. This is the strongest evidence the prize was anticipated.
- **The `ProbeSiteKind` discriminant bends.** It already distinguishes `Establish` (rc-not-fold-usable) from `Query { valid }` (rc-fold-usable) (`plan/lib.rs:595-605`). A `Capture { valid }` variant slots in beside them — 20E §9 names `ProbeSiteKind` as "the seam the probe-exec gate + any future per-site policy keys on." +SURE.

### q-4.b What resists (where)

- **The record GRAMMAR resists multi-line/binary** (q-3.b): `parse_results` is line-oriented (`for line in input.lines()`, `cli/main.rs:411`) and token-delimited (`strip_prefix` over split tokens, `431-441`). A value-bearing record with arbitrary stdout needs base64 (a probe dependency) or a length-prefixed multi-line framing (a parser re-architecture). +SURE this resists — the `stdout=<text>` token-key is single-token-only.
- **The non-leaf→leaf promotion resists.** The inner `$()` command is `expansion_internal` ⇒ unconditionally excluded from the leaf set (`effect.rs:577`, `cfg.rs:1035-1037`). To ship it in the probe AND substitute its captured value, classify must EMIT it as a (new-class) leaf — reversing the find-cli-1 leaf-seam exclusion *for the vouched-Query case only*. +SURE this is a real classify change, not a config flag. The `member_argv`/Members machinery (`value.rs:541-695`) is precedent for "a non-trivial second pass emits extra fact-shapes off the converged solution" — the capture pass would be analogous.
- **The value-plane ← probe-record back-edge resists** (q-3.e): value-flow runs BEFORE the probe (`analysis/value.rs`, in the kernel); the captured value arrives AFTER (cli `facts_from_sites`). Binding `v` to the captured literal requires either a second value-flow pass post-probe or a fold-time substitution outside the value plane. +SURE this dependency is backwards relative to the current pipeline order.

### q-4.c Does anything track value-PROVENANCE today? Does the prize need it?

- **Today: partially.** `ProbeSiteKind` (`plan/lib.rs:595-605`) tracks WHICH observable a record's rc represents (the probe-command's vs. the guard's own) — this IS a provenance distinction (the wrong-concrete firewall, `cli/main.rs:255-297`). The `Derivation` (`plan/lib.rs:188-209`) records `LicenseVia` (which substitution path proved the license) + the fact + verdict. The `OutClaim` is interned (`core/lib.rs:303`), so a captured value has a stable identity. But there is NO tracking of "this `ValueOf::Literal` came from a probe-capture vs. a static literal" — the value plane's `ValueOf::Literal(Symbol)` (`value.rs:58-64`) is provenance-erased (a captured `nginx` and a source-literal `nginx` are indistinguishable). +SURE.
- **The prize NEEDS the missing piece.** `inv-probe-sourced-values` enforcement (the anti-masking discipline, `spike/CLAUDE.md`) requires that a substitution reproduces ONLY probe-provenance values. For the prize, the captured value's probe-provenance must be TRACKED so the fold can prove the substituted literal is probe-sourced (not fabricated). The `ValueOf::Literal` would need a provenance tag (`ValueOf::Literal { sym, source: Static | ProbeCapture(LeafId) }`), OR the capture-fold must route through a separate channel that carries provenance inherently (the `CaptureResolvable` SkipClass + the record's site-key). ~SUSPECT the latter (route-through-the-site-keyed-class) is cleaner than tagging every `ValueOf` — it confines the provenance to the capture path. This is the same fork as q-2's cause-tagged-⊤ (a `ValueOf` reshape vs. a side-channel).

---

## q-5 — Recommendation

### Recommendation: build q-2 ONLY this round; DESIGN q-3 (this note), do not build it.

**q-2 (the floor) is a go-now.** It is pure-additive (diagnostics, no disposition change), risk-free (zero golden churn on `expected.out`), closes a real find-3 violation (the `$()`→⊤ path is the one common construct that degrades silently — q-1.f), and is sized for one short build. It is the honest minimum the round committed to (note 211 §1: "at minimum honest, specific ⊤-diagnostics everywhere `$()` appears"). +SURE.

**q-3 (the prize) is design-more, not build-now.** It is genuinely valuable and the seams were *deliberately pre-opened* for it (q-4.a: `OutClaim`/`stdout=`/`ProbeSiteKind` all reserved). But building it this round is unwise because it requires, in dependency order: (1) the probe-exec gate as a hard prerequisite (fm-1: a captured value feeding the apply is far more load-bearing than the guard rc that 20E §9 already flagged the gate for — a fixture-authored capture is the 19I §3 trap with teeth); (2) a value-carriage wire that survives multi-line/binary stdout (q-3.b — base64-with-a-probe-dependency OR refuse-non-text-floor); (3) a classify change reversing the expansion-internal leaf exclusion for vouched captures (q-4.b); (4) a value-plane ← probe-record back-edge that runs against the current pipeline order (q-3.e/q-4.b); (5) provenance tracking on the captured value for `inv-probe-sourced-values` (q-4.c). That is a multi-wave program. ~SUSPECT the right shape: a FUTURE round does q-3 as its keystone, gated on the probe-exec gate landing first, starting with the **refuse-non-text single-line floor** (q-3.b opt-refuse-non-text — handles `$(hostname)`/`$(uname -m)` without a binary wire).

### Effort shape

- **q-2: ~1 short build task.** 2-3 diagnostic codes (`dq-cmdsub-operand-top`, `dq-cmdsub-inner-nonleaf`, `dq-site-unresolvable`), emit-sites in `effect.rs::command_effect` + classify's leaf-drop + a cli stderr loop; unit tests at the classify layer; no golden churn. Pairs with nothing (touches `effect.rs` + `cli`, disjoint from the door/render work).
- **q-3: a future round's keystone, ~3-4 waves**: (w-a) probe-exec gate; (w-b) the capture SkipClass + classify leaf-promotion + the `CaptureResolvable` emit; (w-c) the record wire (refuse-non-text floor first, then base64); (w-d) the post-probe value re-bind + render-substitute-assignment + a hostile pass (the 20T did-not-survive-list discipline). Each wave wants its own crosscheck.

### The forks the human must rule on (argue both sides; DO NOT settle)

- **fork-capture-claim-type (the q-5 headline fork): is a probe-captured stdout a NEW claim-type (like door-2's counterfactual), or an ordinary probe-observation?**
  - *Argument FOR new claim-type*: a captured stdout is qualitatively unlike a convergence verdict or a guard rc — it is an arbitrary VALUE the apply binds as program data, re-entering the value plane and the split machinery (q-3.f). It carries provenance obligations (`inv-probe-sourced-values`, q-4.c) that a boolean verdict does not (a verdict gates a license; a value BECOMES the program). door-2's counterfactual is precedent for "a probe observation that is not a simple holds/absent" needing its own claim-type. Treating it as ordinary risks the same representation-drift `19F` was built to prevent (a value smuggled through a verdict-shaped channel).
  - *Argument AGAINST (it is ordinary)*: the `OutClaim`/`stdout=` channel was DELIBERATELY built into the one-Observable tuple (`core/lib.rs:300-304`, `419`) precisely so a captured stdout is "a value-plumbing change, not a representation change." The infrastructure already models it as the fourth channel of the ONE Observable (`inv-one-observable`) — inventing a NEW claim-type would re-fragment the observable the round-19 unification fought to merge. A capture is just the Stdout channel finally producing a non-⊤ value, exactly as the reserved shape anticipated.
  - I do NOT settle this. It is the load-bearing design decision the prize turns on, and it determines whether the prize is "wire the reserved channel" (ordinary) or "design a new claim with its own provenance/precedence rules" (new type).
- **fork-cmdsub-top-cause (q-2.a)**: generic-⊤-diagnostic (cheap, ships the floor this round, doesn't single out `$()`) vs. cause-tagged-`ValueOf::Top { cause }` (precise messages naming `$(…)`, but a `ValueOf` reshape touching value.rs + every consumer). Same shape as the provenance fork (q-4.c). The floor wants generic; the prize wants the tag.
- **fork-capture-probe-body (q-3.a)**: the value-capture's probe is the INNER COMMAND verbatim (faithful — captures what the book does; but ships book-author code into the read-only probe, which the `dq-reflexive-probe-inertness` vouch-closure-check must then bless) vs. a separate `oracle_probe_*`-style declaration (the established pattern; but an authoring burden + a second place the read can drift from the book's actual `$()`).
- **fork-capture-wire (q-3.b)**: base64 (one-line, byte-safe, but a probe DEPENDENCY — breaks ship-and-run-anywhere) vs. pure-sh hex-encode (no dependency, but a heavier probe body the closure-check must bless) vs. refuse-non-text-floor (no wire change, handles only single-line ASCII captures, degrades safely on the rest). I lean refuse-non-text for the first slice, but the human owns the dependency-vs-coverage tradeoff.

---

## Confidence summary

- +SURE: the current `$()` handling (parser sub-AST, expansion_internal non-leaf, value-plane ⊤-collapse, Opaque classification) — all traced in source with tests. +SURE the degradation is SILENT (no diagnostic on any `$()`→⊤ path; `effect.rs`'s only diagnostic is `KIND_DISAGREEMENT`; `value.rs` emits none). +SURE the two current-behavior anchors (`exec-opaque-var-runs`, `exec-subst-body-nonleaf`).
- +SURE: q-2 is pure-additive, risk-free, the honest floor; q-3's seams were deliberately pre-opened (`OutClaim`/`stdout=`/`ProbeSiteKind` reserved-but-inert, doc-confirmed).
- +SURE: q-3's record-grammar wall (line-oriented, token-delimited parser) and the probe-exec-gate prerequisite (fm-1, fixture-authored captures = the 19I §3 trap).
- ~SUSPECT: the cleanest q-3 first-slice is refuse-non-text single-line; the value-plane ← probe-record back-edge is the deepest architectural cost; routing capture-provenance through the site-keyed class (not tagging every `ValueOf`) is cleaner.
- -GUESS: the silent-4 inner-mutator-vanishing shape-hazard is not reachable at HEAD (an assignment with a ⊤ RHS is Pure, not Replace-eligible), flagged for q-3.
- The fork-capture-claim-type is explicitly UNSETTLED — it is the design decision the prize turns on, argued both ways, handed up.
