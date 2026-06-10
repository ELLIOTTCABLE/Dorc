# 20M — task-L1: loops parse + lower (brk-1 structure). Strain + decisions.

> Round-21 (take-3 continued). Charter: `209` brk-1 (the single highest-cost realism miss),
> executed as task-L1 = the STRUCTURE slice (parse + lower + sound-on-cycles + corpus). The
> PRECISION slice (Powerset loop-domain + member-elision render) is explicitly NOT here.
> AI-authored, confidence-marked. Companion to `209` (the breakdown map) and `20K` §4 (the
> inherited roadmap). HEAD before this task: `8c9f632`.

## §0 Headline

+SURE (traced + tested): `for NAME in WORD…`/`while`/`until` over an enumerable list now
PARSE to real AST nodes and LOWER to a genuine cyclic CFG with a back-edge — the first real
cycle the monotone worklist has ever been fed. All three analyses (value, effect/reaching-
defs, fold) stay sound and CONVERGE on the cycle. The brk-1 value-unlock is real end-to-end:
a converged install BELOW a *pure* loop now elides (`loop-post-elision-revives`). The
in-loop render-floor (a body leaf never mints a license) is the recorded floor the
member-elision slice later lifts. Gates all green (55 e2e, zero xfail; fmt/clippy-D/test/
typos).

## §1 Grammar coverage — what parses, what stays ⊤ (and why)

PARSES now (real `NodeKind::ForLoop`/`WhileLoop`, body + words + spans captured):
- `for NAME in WORD…; do LIST; done` — words captured losslessly; the iteration var is a name.
- `while LIST; do LIST; done` / `until LIST; do LIST; done` — `until` is `WhileLoop{until:true}`
  (continuation-sense flag; the CFG/value/effect planes are sense-agnostic, so one lowering
  serves both — the only thing `until` changes is which runtime branch the rc selects, which
  Dorc never interprets).
- Nested loops (two+ back-edges); `do`/`done` after `;` or newline; loops mid-sequence
  (neighbours survive).

STAYS ⊤-rejected (`UnsupportedReason::Loop`, loud `syntax-unsupported` + `cfg-top-node`):
- rl-1 **no-`in` `for NAME; do …`** — iterates runtime `"$@"` (positionals), not a static
  list. +SURE this is right: the list is runtime input, so the for-var and iteration count
  are both ⊤; modeling it gains nothing and the `"$@"`-expansion is the deferred dynamic-list
  surface (brk-3-adjacent). Pinned: `loop_shapes_outside_the_subset_stay_unsupported_loop`.
- rl-2 **`break`/`continue`** anywhere in a loop body — ⊤-rejects the WHOLE loop. HONEST
  REASON (the prompt asked for the real one, not a hand-wave): an un-modeled early-exit edge
  breaks **REACHING-uses soundness**, not just the back-edge. The fixpoint assumes every body
  path reaches the back-edge (so a body def reaches the next iteration's uses) AND that the
  loop-exit sees the join of all body paths. A `break` creates an exit path that SKIPS the
  rest of the body — so a def after the `break` does NOT reach the exit on that path, but the
  un-modeled CFG would still propagate it (a may-over-approximation that is unsound for a
  *must*-style elision downstream). A `continue` skips to the head, likewise dropping
  post-`continue` defs from that iteration's tail. Rather than model the two extra edges
  (doable, but their interaction with the errexit pass + the in-loop floor is unanalyzed),
  L1 ⊤-rejects. ~SUSPECT modeling them is a small, clean follow-up (two edges: `break`→merge,
  `continue`→head), but it wants its own exclusion-check pass. Binds to the INNERMOST loop
  (a nested `break` ⊤-rejects only the inner loop — `nested_break_continue_binds_to_inner_loop_only`).
- rl-3 **`for`-list word with `$(…)`/`$(())`** — an effect-bearing expansion in word
  position. HOLE#1's existing posture is that `$(…)` effects outside command position are a
  deferred surface; a for-list `$(cmd)` RUNS during expansion (it has effects) but L1 does not
  lower those word-position substitutions, so it ⊤-rejects rather than silently drop the
  effect. VERIFIED + STATED per the prompt. (A may-split unquoted `$pkgs` word does NOT
  ⊤-reject — no command runs during its expansion; the for-var just goes ⊤, body stays
  visible. So `for h in $hosts` is analyzable, body-visible, var-⊤ — the multiplicitous case
  analysis/CLAUDE.md flagged.)

The ⊤-trigger DID shrink (the preamble's warning — "that's you"): `spike/CLAUDE.md`
inv-top-reject + `syntax/CLAUDE.md`'s ⊤-trigger list both updated.

## §2 The errexit-in-loops rules implemented (with dash citations)

Extends the T9 condition-region pruning to loops. Verified against dash 0.5.x behaviour:
- er-1 **`while`/`until` CONDITION is errexit-EXEMPT.** A failing command in the condition
  region does NOT abort under `set -e`. dash: `set -e; while false; do :; done; echo ok`
  prints `ok` (the condition's non-zero rc is the loop-termination signal, not an error).
  Implemented by lowering the condition through `lower_condition_region(_, mark_status=true)`
  — the SAME path as an `if`/`elif` test, which already cleared fallibility across the whole
  region. So NO failure→exit edge on any condition command. Pinned:
  `while_condition_is_render_floor_and_errexit_exempt` (asserts `!has_exit_edge` on the cond).
- er-2 **A loop BODY command DOES abort under `set -e`.** dash: `set -e; for x in a b; do
  false; echo unreached; done; echo after` stops at the first `false` (prints neither
  `unreached` nor `after`). The body is NOT a condition region, so its commands keep their
  fallibility ⇒ phase-2 materialises a failure→exit edge, AND C-3 marks them
  `StatusRelaxable`-consumed. Pinned: same test asserts `has_exit_edge` on the body command +
  `StatusRelaxable` in its consumed set.
- er-3 **`for`-list words are pure expansion** — no failure-edge of their own (they mint no
  CFG node; a `$(…)` in them is rl-3 ⊤-rejected, so the "expansion command fails under set -e"
  case cannot arise for a parsed for). STATED per the prompt's verify-and-state ask.
- er-4 **The condition's status is consumed at the render FLOOR** (`StatusRenderFloor`, like
  an if-guard): a loop condition decides body-vs-exit, and the line-granular render cannot
  substitute it in-situ (it shares its line with `while`/`do`). So `consumption_ok` blocks it
  unconditionally — a `while CMD` condition is never elided. This is item-2(b).
- er-5 **errexit STATE flows across the back-edge.** A `set -e` inside a loop body persists to
  the next iteration (dash: same shell). The phase-2 forward errexit pass already handles the
  back-edge by fixpoint (height-2 lattice ⇒ converges); the body's `On` flows back to the head
  and joins. No special-casing needed — verified the existing `materialise_errexit_edges`
  worklist terminates with the cycle present.

## §3 The first-real-cycle worklist evidence

The prompt (and task-A's note) flagged "the worklist handles cycles by construction but has
NEVER been fed one" as an untested claim. It is now tested directly:
- `value::for_loop_body_reassignment_converges_via_back_edge` — a body var reassignment
  joins the pre-loop value across the back-edge; `converged == true` asserted explicitly.
- `value::while_loop_body_var_is_top_after_loop` — same for `while` (no loop var).
- `value::nested_loop_book_converges` — NESTED loops (two back-edges feeding each other);
  the worklist still reaches a fixed point.
- `effect::classify_converges_on_nested_loop_back_edges` — the reaching-defs fixpoint
  (`debug_assert!(reach.converged)` would trip otherwise) on a nested loop.
- `cfg::for_loop_lowers_a_back_edge_not_a_top_node` — the structural proof: the body and the
  `LoopHead` are MUTUALLY reachable (a real cycle), the head has an exit edge, succ/pred stay
  consistent over the cycle.

CONVERGENCE ARGUMENT (+SURE, the load-bearing one): the value transfer at a for-`LoopHead`
re-binds the iteration var to a CONSTANT (the Flat-JOIN of the list words, resolved against
incoming) every iteration — independent of the back-edge value for that var. That is a
monotone function (more ⊤ in ⇒ more ⊤ out), and the rest is the standard pointwise `MapL`
join over the height-2 `Flat` domain. Finite-height + monotone ⇒ terminates (the `solve` cap
+ loud non-convergence is the backstop, never hit in the corpus). The for-var being RESET
each iteration (not accumulated) is what keeps it from being a fresh ⊤-fountain that still
converges — it converges in ONE extra pass.

## §4 Every golden delta (corpus)

- gd-1 **`loop-degrades-safely` → DELETED; `loop-analyzed-body-runs` CREATED** (the re-pin).
  Book: `for x in a b; do apt-get install -y "$x"; done` + `apt-get install -y curl`. The
  for-var is ⊤ (2 distinct words) ⇒ body install Opaque ⇒ runs (⊤ operand AND in-loop floor,
  stacked). The Opaque propagates Reach::Top across the back-edge ⇒ curl below is Written ⇒
  runs. expected.ran = `a, b, curl` (the loop EXECUTES verbatim under mocks). The
  `expected-diagnostics` syntax-unsupported entry is GONE (the loop parses clean). No
  resolvable sites ⇒ empty probe-results.
- gd-2 **`loop-post-elision-revives` CREATED** (the brk-1 value-unlock, run-set-proven). Book:
  `for f in a b; do echo "$f"; done` (PURE body) + `apt-get install -y nginx`. The pure body
  gens nothing ⇒ the post-loop install is EstablishAmbient; the probe reports it `holds` ⇒
  it is `Replace`d (elided to `true`). The apply comments the install + substitutes `true`;
  expected.ran is empty (echo logs nothing, install elided). THIS is the value the round
  unlocks — a converged install below a loop, previously killed by ⊤-containment, now elides.
- gd-3 **`while-condition-floor` CREATED** (item-4b). Book: `set -e` + `while dpkg -s nginx;
  do echo installing nginx; done` + `apt-get install -y curl`. The condition `dpkg` is Opaque,
  in-loop, `StatusRenderFloor`-consumed ⇒ runs (never elided). The `dpkg` mock exits 1 so the
  loop runs ZERO iterations (terminating — see §5 cv-1). `set -e` does NOT abort on the false
  condition (er-1). curl is poisoned (Reach::Top out the back-edge) ⇒ runs. expected.ran =
  `dpkg -s nginx, apt-get install -y curl`. (The mutator-body errexit-region StatusRelaxable +
  failure-edge is pinned in the UNIT test, §2 er-2 — a stronger, isolated assertion than the
  e2e could give; see cv-1 for why the e2e body is a pure `echo`.)
- gd-4 **`loop-nested-converges` CREATED** (item-4c). Book: `for p in a b; do for q in c d;
  do apt-get install -y "$p$q"; done; done` + `echo all-done`. Nested back-edges; the inner
  install (`"$p$q"` ⊤) runs at depth 2; the loop expands to ac/ad/bc/bd. The load-bearing
  property is TERMINATION on nested cycles (dorc produces output, doesn't hang/cap). No
  elision.
- gd-5 **`toprejected` RE-PURPOSED** (a loop-as-⊤ pin that the prompt's item-1 says to flip,
  surfaced by the bless). Its old book `for i in 1 2; do apt-get install -y nginx; done` now
  PARSES — and (a finding!) the curl below it became EstablishAmbient/resolvable (the
  literal-list loop body establishes `package:nginx#installed`, a DIFFERENT cell from
  `package:curl#installed`, so it does NOT poison curl — only an Opaque would). Rather than
  let `toprejected` silently become a half-elision case, I re-pointed it at the RESIDUAL loop
  ⊤-reject: `for x in a b; do break; done` + curl. It still demonstrates a loud loop ⊤-reject
  (now via `break`, not "loop construct") with curl running on the ⊤-poison. expected-
  diagnostics updated to the `break`/`continue` message.

Non-loop goldens: UNCHANGED (verified via `git status` — only the five cases above + the
docs/source moved; the BLESS re-wrote 55 expected.out files byte-identically except these).

## §5 The in-loop render-floor shape

+SURE (the chosen mechanism): a per-node `Cfg::in_loop_body(id)` bit, set by an arena-range
pass in `lower_for`/`lower_while` (the same range idiom `expansion_internal` uses), covering
the body AND (for while/until) the condition. `plan::disposition_for` gates on it FIRST:
`if cfg.in_loop_body(node) { return Disposition::Run; }` — before the fold (`Omit`) and before
convergence-elision (`Replace`). So an in-loop leaf NEVER mints a license this round, for
EITHER substitution path. Kept it a SEPARATE structural guard (like `has_top_successor`),
NOT folded into `prove_replaceable` — it guards a render-capability fact (line-granular render
can't elide one iteration), not a value/phase fact, mirroring how the ⊤-containment guard is
kept separate. Pinned: `plan::in_loop_establish_runs_even_when_converged` (a single-word for,
so the body install RESOLVES to a real cell and a Converged host WOULD license it — but the
floor forces Run) + `plan::post_loop_install_elides_below_a_pure_loop` (the floor is
in-loop-SCOPED, not a blanket regression). This is the recorded floor `209` brk-1 (b)'s
member-elision render lifts: when the render can rewrite the iteration LIST to the diverged
members (`for pkg in postgresql`), an in-loop leaf becomes elidable per-member.

NOTE (a real subtlety worth flagging): even WITHOUT the structural floor, a self-establishing
loop body tends to become `EstablishWritten` via the back-edge (iteration 2 sees iteration
1's establish reach its own in-state). So the floor is belt-and-suspenders for establishes —
but it is LOAD-BEARING for in-loop QUERIES (a Query gens nothing, so `command -v` in a loop
body stays pristine/valid and WOULD be Replace-able without the floor). The floor is the only
thing stopping a per-iteration Query fold the line-render can't express.

## §6 Flags (tc-* — surfaced, not resolved)

- tc-loop-gate5 (HARNESS, ~SUSPECT worth a look): gate-5 (argv-echo differential) asserts
  every engine `run` site whose argv[0] is shimmed appears in the BARE book's executed argvs.
  An in-loop `run` body whose loop-guard short-circuits it (a `while` condition false ⇒ 0
  iterations) is `run`-disposed but ABSENT from the bare run — a FALSE gate-5 failure (I hit
  this building `while-condition-floor`). Worked around by making the e2e body a non-shimmed
  builtin (`echo`), so gate-5 skips it naturally. The principled fix (deferred, flagged):
  gate-5 should skip in-loop sites the way it already skips omit/replace sites (the
  `tc-gate5-omit` carve-out) — a loop-guarded body may legitimately be absent from the bare
  run. Needs an in-loop flag on the `argv` debug line (cli scope). Not done — out of L1's
  parser/analysis scope, and the workaround is sound.
- tc-loop-var-precision (DESIGN, expected): the for-var is a Flat JOIN ⇒ `for f in a b`
  gives `f = ⊤`, so the body install is Opaque and runs even though a Powerset {a,b} would
  let it elide per-member. This is `209` brk-1 (b), the NEXT slice — flagged as the recorded
  imprecision, not a bug.
- tc-while-cond-opaque (minor): a `while dpkg -s nginx` condition is Opaque (dpkg unoracled),
  so it both blocks (StatusRenderFloor) AND poisons downstream (Reach::Top). A FUTURE Query
  oracle for `dpkg -s` would make the condition a resolvable Query — but it would STILL be
  render-floored (a loop condition is not in-situ substitutable), so the floor, not the
  Opacity, is the operative block. Stated so the next round doesn't "fix" the Opacity expecting
  the condition to elide.

## §7 Exclusion-check (the four-by-two, abbreviated)

- reverse direction (backward/Must): the in-loop floor is orientation-agnostic (a structural
  Run, minted before any phase collapse). No backward analysis is instantiated yet, so this is
  vacuous — but the floor sits in `disposition_for`, which is apply-phase; a future backward
  caller would need its OWN in-loop handling (flagged, not built).
- other phase (probe): I worried an in-loop establish/query might be probed (the apply floor
  refuses to act, but the rendered PROBE keys off SkipClass, not the in-loop bit). VERIFIED
  benign: an in-loop self-establishing body becomes `EstablishWritten` via the back-edge (its
  own establish reaches its in-state on iteration 2), so `compile_probe` records it
  UNRESOLVABLE ⇒ it is never probed (traced: `for f in nginx; do apt-get install -y "$f";
  done` ⇒ `# site:0 skip-unresolvable`). So the back-edge + the floor agree, and no in-loop
  establish is probed. The one residual: an in-loop QUERY gens nothing ⇒ stays pristine/valid
  ⇒ WOULD be compiled into the probe (and folded) without the floor — the floor (apply-side)
  is what stops the fold; the probe would still carry the check (unused). ~SUSPECT harmless
  (a probed-but-unused in-loop Query check is wasted work, not a wrong-elision), but it is the
  one spot where the floor and the probe-compiler disagree. The clean fix (defer): teach
  `compile_probe` the in-loop bit too, so an in-loop site is unresolvable on BOTH sides. Not
  done (no corpus case probes an in-loop Query; the apply floor is the correctness-bearing
  half and it holds).
- other user (lazy admin): a scrappy book full of `for pkg in a b c; do apt-get install
  "$pkg"; done` gets the body run + the (multi-word) ⊤ var — exactly the safe degrade. The
  precision they'd want (elide the already-installed members) is brk-1 (b).
- other reliability (unreliable oracle): an in-loop Query with a flaky rc never folds (the
  floor runs it regardless) — strictly safer than the straight-line Query path.
