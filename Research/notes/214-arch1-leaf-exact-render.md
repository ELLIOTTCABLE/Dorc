# 214 — arch-1: the leaf-exact (span-based) apply render. Strain + decisions.

> Round-21 w-2, builder note (slug 214 reserved per 211 §4). Charter: 211 §1 arch-1 + the
> d-1…d-10 design in the round-21 priming prompt. Append-only; confidence-marked
> (+SURE/~SUSPECT/-GUESS/--WONDER). Engine edits: `core/src/lib.rs` (Channel: retire
> `StatusRenderFloor`, add `StatusIterated`), `analysis/src/cfg.rs` (`lower_condition_region`
> channel-split), `plan/src/lib.rs` (the span-edit render replaces the line-render; `consumption_ok`
> + omit-safety + heredoc refusal), `plan/src/render.rs` (retire the line emitters, add the
> provenance comment), `cli/src/main.rs` (report the heredoc-refusal diagnostic). Tests:
> `analysis/tests/cfg.rs` (re-homed), `plan/tests/observable_matrix.rs` (re-homed), `plan/src/lib.rs`
> inline tests (re-homed), 5 new e2e cases (`render21-*`), door3-or-true-elides promoted (XFAIL deleted).
> HEAD before task: `f09ebd7`.

## §0 Headline (what shipped)

+SURE (traced + tested + e2e-exec-gated ×2): `render_apply` is now a **span-edit application** —
it collects `(Span, replacement)` edits over the original source bytes and splices them
right-to-left, line-by-line, appending one provenance comment per edited line. The round-21
carve-out family (T14 case-arm `inline_arm_subst`, F2 scaffolding `inline_scaffold_subst`,
whole-line `commented_line`, the `LineRender`/`classify_lines`/`emit_apply_lines` plumbing, the
`case_arm_oneliner_leaves`/`scaffolding_boundary_lines` detection) is **DELETED, not bypassed**
(d-7 grep: zero non-doc references). `Channel::StatusRenderFloor` is **deleted**; an if/elif guard
became an ordinary `StatusRelaxable` substitution site, and a `while`/`until` condition became a
NEW `Channel::StatusIterated` (the honest successor — unconditional block keyed on iteration, not
render capability). door-3's `door3-or-true-elides` payoff (XFAIL until now) renders `true || true`
with an EMPTY run-set and passes as an ordinary case. Gates green ×2: `cargo fmt --check` ·
`clippy --workspace --all-targets -D warnings` (no new expects) · `cargo test --workspace` (361
tests) · `sh e2e/run.sh` ×2 (75 cases, ZERO xfail, all six gates) · `typos spike`.

## §1 Design-as-built (the span render)

The render is three free functions + one method, replacing the `LineRender` state machine:

- **`Plan::collect_edits(src, ast) -> Vec<SpanEdit>`** — one edit per elided leaf. A `Replace`
  leaf ⇒ `(leaf-command-span, standin_sh(stand_in), original)`; a fold-dead `Omit` whose
  controller is neutralised ⇒ `(span, ":", original)` (the omit-safety gate); a `Run` leaf and a
  kept-controller `Omit` ⇒ no edit (verbatim by construction). REFUSE (d-6): a leaf carrying a
  heredoc redirect is dropped (runs verbatim) — its span covers `<<EOF`, not the body lines.
- **`normalise_edits(edits) -> Vec<SpanEdit>`** — sorts by `(lo, Reverse(hi))`, enforces the
  edit-model invariants: PARTIAL overlap is a `debug_assert` (leaf-seam violation) + release-drop;
  full containment ⇒ OUTER wins, inner dropped. (No current shape produces a containing
  construct-edit — only leaf commands are edited, and two leaf spans are disjoint — so the
  containment branch is defensive. The leaf-seam (`inv-leaf-seam`) is what guarantees disjointness.)
- **`emit_span_edits(src, edits) -> String`** — the splice. Walks SOURCE lines; for a line with
  edit(s), splices them right-to-left into the region `[line-start, last-consumed-line-end)`
  (covering any multi-line edit, which collapses its lines onto the start line), then appends the
  provenance comment if `comment_safe`. A multi-line edit's interior lines are CONSUMED (skipped).
- **`render::apply::provenance_comment(originals) -> String`** — the ONE provenance emitter (d-3).
  Appends `   # dorc: elided [<orig>; <orig>] (already converged / dead branch)`, disclosing each
  replaced command's ORIGINAL text (the old whole-line-comment form carried the original; the new
  form must not lose it). Interior newlines in an original are FLATTENED to spaces (else a
  multi-line operand's embedded `\n` splits the `#` comment into a stray unterminated-quote line —
  a real bug I hit and fixed, §4 strain-2).

Two helpers preserved/added: `is_neutralised` (omit-safety) was GENERALISED to resolve a COMPOUND
controller (an `if`-cond / `! pipeline` / `&&`/`||`) by walking its AST subtree
(`subtree_leaves_all`) and checking every `Simple` leaf is neutralised — needed because arch-1
makes a known-rc guard elide, so a dead body's controller can be a substituted compound (§4
strain-1). `comment_safe(rendered_line)` (d-3 SAFETY RULE): drops the comment when the rendered
line ends with `\` (backslash-continuation) or contains `<<` (heredoc operator).

**d-2(b) fold-outcome mapping** (how the existing fold renders today → span edits):
- `fold-oror-guard-omits` / `exec-query-guard-composition` / `exec-shimmed-query-fold`
  (`command -v/dpkg -s … || apt-get install`, guard holds): OLD whole-line collapse to `true`. NEW
  = guard span → `true` (its known rc), install span → `:` (Omit, controller neutralised) ⇒
  `true || :`. Same semantics (empty run-set, list rc 0), span-faithful text.
- A guard's Omit body keyed off the guard's leaf neutralisation — unchanged in spirit; the
  controller-resolution generalisation (§4 strain-1) is what makes the if-guard form work too.

## §2 d-4 CHECKPOINT-1 result (the StatusRenderFloor source enumeration)

+SURE, grepped + traced: `lower_condition_region(mark_status=…)` is the **ONLY**
`StatusRenderFloor` source (was `cfg.rs:1391`, the lone `singleton(Channel::StatusRenderFloor)`).
It is called from exactly two sites with the marking enabled: `lower_if_chain` (if/elif condition)
and `lower_while` (while/until condition). `lower_and_or` calls it with marking DISABLED (the
`&&`/`||` left operand marks `StatusRelaxable`/`StatusInvariant` itself). Nothing else carried
RenderFloor. The split (d-4): `mark_status` changed from `bool` to `Option<Channel>` — if/elif ⇒
`Some(StatusRelaxable)`, while/until ⇒ `Some(StatusIterated)`, `&&`/`||` ⇒ `None`.

**in_loop_body coverage of the while CONDITION (the d-4 checkpoint sub-question — answered, and the
prompt's guess was WRONG for this impl):** `lower_while` captures `loop_start = self.nodes.len()`
BEFORE lowering the condition (line ~949), then `mark_in_loop_range(loop_start, after)` covers the
condition nodes too. So a top-level `while` condition IS flagged `in_loop_body` here — it is NOT
"outside its own body's floor" as the prompt's d-4 note hypothesised. Consequence: a `while`
condition is floored to Run by the structural `in_loop_body` gate in `disposition_for` AND carries
`StatusIterated`. The `StatusIterated` mark is therefore defense-in-depth (the in_loop floor
already forces Run), but it stands independently so the block is honest about WHY (iteration), and
survives if that floor later lifts. (For an `if` nested INSIDE a loop body, the if-guard would be
in_loop too — floored — but a top-level `if` guard is not in_loop, so its new `StatusRelaxable`
substitution is what actually fires. +SURE.)

## §3 The d-5 audit — EXISTING corpus cases whose disposition flips (RESULT: NONE)

The charter's pre-registered expectation (bite-3, d-5) was that some converged known-rc Query
guard, formerly floored, now elides. **I traced every if/elif-guard case in the corpus; ZERO flip.**
+SURE. The if-guard cases are:
- `guarded` / `fi-shared-line`: guard is `if true` — `true` is a Pure builtin, no fact, never
  classified establish/query ⇒ never reaches `prove_replaceable` ⇒ unchanged (the body install,
  not the guard, was already eliding).
- `guard-status-blocks-elision` (`if ! command -v nginx; then apt-get install; fi`): the guard
  `command -v` is declared an **ESTABLISH** (`oracle_effect command '' establish present`), NOT a
  Query — so its rc is ⊤ (fork-mutator-rc), and `StatusRelaxable + ⊤` BLOCKS exactly as
  `StatusRenderFloor` did. The case STILL runs the install. The name "blocks-elision" stays
  accurate (it blocks, now via Relaxable+⊤ instead of RenderFloor). **This surprised me** — I
  initially mis-read it as a Query that would flip (it has the same `if ! …` shape as the fold
  cases), but the fold cases use a `query`-polarity tool oracle while this one uses `establish`.
  The distinguishing datum is the oracle's `oracle_effect … query/establish`, not the syntax.

So the d-5 "newly-expressible from the channel change" set on EXISTING cases is EMPTY. The if-guard
elision IS demonstrated, but by a NEW case (`render21-if-guard-query-elides`), not an existing flip.
~SUSPECT this is a fixture-coverage accident (no corpus case happened to put a `query`-polarity
guard in an `if`), not a design fact — the capability is real (the new case proves it).

## §4 What strained

- **strain-1 (the load-bearing one) — compound-controller omit-safety.** A fold-dead `Omit` body's
  controller is the **if-condition AST node** (the Pipeline `! dpkg -s nginx`), NOT the guard's
  `Simple` leaf. The old `is_neutralised(by_ast, leaf)` looked the controller up in `by_ast` (the
  per-leaf disposition map) — a compound node is not a leaf ⇒ `None` ⇒ "not neutralised" ⇒ the body
  rendered verbatim. At HEAD this was INVISIBLE (a floored guard never elided, so the body never
  wanted to elide via a neutralised compound). arch-1 makes a known-rc guard elide ⇒ the body's
  compound controller IS neutralised-in-fact, but the lookup couldn't see it. FIX: `is_neutralised`
  now takes the `&Ast` and, for a non-leaf controller, walks its subtree (`subtree_leaves_all`,
  mirroring the fold's `kill_rec` shape) asserting EVERY `Simple` leaf is neutralised — a guard
  whose every command is substituted reproduces the branch decision, so the dead body is safe to
  elide. Without this, `render21-if-guard-query-elides` rendered the dead install verbatim (it
  would have RUN — a wasted, but safe, execution). +SURE, demonstrated end-to-end.
- **strain-2 — the provenance comment's embedded newline.** A multi-line leaf's `original` text
  contains a literal `\n`; injected raw into a `#` comment it splits the comment, and the second
  line (`line"]`) is a stray command with an unterminated quote ⇒ `dash -n` FAIL. Caught by the
  re-derived `render_multiline_leaf_…` test. FIX: `provenance_comment` flattens interior whitespace
  in each original. (The comment is provenance prose; collapsing its whitespace loses nothing
  load-bearing.) +SURE.
- **strain-3 — the multi-line-edit line collapse.** A span edit may cover multiple SOURCE lines
  (a literal-newline operand), so the rendered line count differs from the source. Handled by
  emitting in ONE line-walk over the source, computing each multi-line edit's `consumed_through`
  line and skipping the absorbed lines — the rendered line that holds an edit corresponds 1:1 to
  the edit's START source line (replacements never ADD lines). +SURE the bookkeeping is correct
  (the `render21-multiline-leaf-substitutes` + `render_multiline_leaf_…` pins exercise it).
- **strain-4 — the heredoc diagnostic plumbing.** d-6 wants a diagnostic, but `render_apply`
  returns `String` (pure), and threading a `Carrier` would ripple to every caller. I added
  `Plan::render_refusal_diagnostics(ast) -> Vec<Diagnostic>` (a separate method) that the cli
  `report()`s on stderr — keeping the render pure and the diagnostic at the I/O edge. ~SUSPECT this
  is the right boundary (the render stays a pure String fn; the diagnostic is cli-edge prose), but
  a stricter reading might want the refusals folded into the render's return — flagged.

## §5 Churn table (per case → bucket → justification)

Buckets: **text-only** (run-set + exec-gate UNCHANGED, only the golden comment format moved);
**newly-expressible** (run-set legitimately changed — a leaf that shares a line with a Run leaf now
elides); **payoff** (door-3's deferred unlock).

| case | bucket | one-line justification |
|---|---|---|
| door3-or-true-elides | **payoff** | XFAIL deleted; `true \|\| true`, EMPTY run-set — the mutator span substituted (Replace, StatusInvariant license), `true` rhs verbatim (NOT a spurious fold). The d-8 hard acceptance. |
| exec-multileaf-line-mixed | **newly-expressible** | `apt-get install -y nginx; systemctl reload` (nginx converged): OLD ran BOTH (run_lines-wins on a mixed line); NEW substitutes the install span → `true`, systemctl verbatim ⇒ run-set drops `apt-get install -y nginx`. |
| headline-guarded-realistic | **newly-expressible** | `dpkg -s nginx \|\| apt-get install` (guard valid known-rc, install live): OLD ran the guard `dpkg -s nginx` (run_lines-wins, the install is a Run leaf on the line); NEW substitutes the guard span → `false` ⇒ run-set drops `dpkg -s nginx`. Mutators unchanged. |
| converged, exec-converged, exec-devnull-exempt, exec-distinct-selectors, exec-enabled-not-active-host, exec-enclosing-pipe-subshell, exec-literal-unset-pure, exec-poison-wall-dead, exec-pure-builtin, exec-query-guard-composition, exec-resolved-var-elides, exec-shimmed-query-fold, exec-singleton-update, exec-subshell-establish, exec-subst-body-nonleaf, exec-top-arith-in-arg-ok, enclosing-group-redir, fi-shared-line, fold-oror-guard-omits, guarded, loop-members-all-converged-elides, loop-post-elision-revives, partial-top-argv-runs, post-loop-shared-done-line, pre-loop-shared-for-line, redir-as-effect, render-case-arm-oneliner, render-multileaf-line-all-elide, seam-two-providers-one-kind, split-single-elides, two-oracles | **text-only** (×31) | The OLD whole-line-comment form (`# <cmd>   # dorc: elided\n<standin>`, 2 lines) became the in-situ span form (`<standin>   # dorc: elided [<cmd>]`, 1 line). Exec gate PASSED for every one ⇒ run-set identical. (run.sh fails on the FIRST gate, so a `[content diff]` failure means the exec gate already passed.) |
| render21-if-guard-query-elides, render21-if-guard-toprc-runs, render21-while-guard-floored, render21-heredoc-refusal, render21-multiline-leaf-substitutes | NEW | d-9 cases (see §6). |

**Zero semantic golden churn outside {payoff, newly-expressible}.** The two newly-expressible
cases are the leaf-exact render's core capability: substituting a converged/known-rc leaf that
SHARES A LINE with a Run leaf — unreachable under the line render (`run_lines`-wins forced the whole
line verbatim). Both run-set shrinks are sound (the substituted leaves are read-only/converged;
their observables are reproduced — a `dpkg -s` guard → `false` reproduces its probed rc; a
converged install → `true`).

## §6 The d-9 new corpus cases (all exec-gated, mocks-only, hand-derived)

- **render21-if-guard-query-elides** (if-guard pole A): `if ! dpkg -s nginx >/dev/null 2>&1; then
  apt-get install -y nginx; fi`, nginx holds. The guard (valid pkgstate Query, rc 0) substitutes
  to `true`; `! true` ⇒ if-false ⇒ the body install Omits (controller neutralised) → `:`. Renders
  `if ! true; then :; fi`. EMPTY run-set. The if-guard analogue of the `||`-fold, unlocked by arch-1.
- **render21-if-guard-toprc-runs** (anti-pole B): `if apt-get install -y nginx; then echo started;
  fi`, converged. The guard is a MUTATOR (⊤ rc) ⇒ `StatusRelaxable + ⊤` BLOCKS ⇒ runs verbatim.
  run-set `[apt-get install]`. Proves arch-1's if-guard unlock is VALUE-keyed (known rc elides; ⊤
  runs), not a blanket "guards now elide".
- **render21-while-guard-floored** (while anti-pole): `while dpkg -s nginx >/dev/null 2>&1; do echo
  checking; done`. The condition is a Query but in a while ⇒ `StatusIterated` ⇒ NEVER substituted
  (verbatim), even though the SAME guard in an `if` elides. The mock dpkg exits 1 so the loop runs
  ZERO times and TERMINATES (a holds would loop forever — the disaster StatusIterated prevents).
  run-set `[dpkg -s nginx]` (the condition runs once). The in-loop Query is also probe-excluded
  (item-6b), so the probe ships no resolvable site.
- **render21-heredoc-refusal** (d-6 refusal): `apt-get install -y nginx <<EOF\nsome config\nEOF`,
  converged. The disposition LICENSES a Replace, but the heredoc leaf is REFUSED (its span covers
  `<<EOF`, not the body) ⇒ runs verbatim + emits `error[render-heredoc-refused]` (declared in
  `expected-diagnostics`, gate-3). run-set `[apt-get install]`.
- **render21-multiline-leaf-substitutes** (newly-expressible pin): `apt-get install -y
  "multi\nline"` (operand spans 2 lines, converged) + `systemctl reload nginx`. The multi-line span
  collapses to `true` (the line-render's old multi-line refusal retired); systemctl runs. The
  provenance comment flattens the operand's newline. run-set `[systemctl reload nginx]`.

## §7 Test re-homing table (d-7 — every deleted test gets a successor or a reason)

| deleted/changed test | successor / disposition |
|---|---|
| `render_floor_status_blocks_unconditionally` (plan inline) | → `iterated_status_blocks_unconditionally` — `StatusIterated` is the new unconditional-block channel (the successor channel); same shape, all-rc block. |
| `consumed_if_guard_marks_render_floor` (cfg) | → `consumed_if_guard_marks_relaxable` — if-guard now marks `StatusRelaxable`. |
| `consumed_negated_if_guard_marks_render_floor` (cfg) | → `consumed_negated_if_guard_marks_relaxable`. |
| `while_condition_is_render_floor_and_errexit_exempt` (cfg) | → `while_condition_is_iterated_and_errexit_exempt` — while-cond now `StatusIterated`. |
| `consumed_errexit_mark_respects_precise_edge_pruning` (cfg) | re-homed IN PLACE: the channel no longer distinguishes if-guard (was RenderFloor) from errexit (Relaxable) — both are Relaxable now — so the precise-edge property is pinned by the FAILURE-EDGE (`!has_exit_edge` on the errexit-exempt guard), the real property. |
| `render_multi_line_case_arm_body_keeps_whole_line_comment_form` (matrix) | → `render_multi_line_case_arm_body_substitutes_span_in_situ` — own-line arm bodies now substitute in-situ (the whole-line form retired). |
| `render_own_line_then_body_keeps_whole_line_comment_form` (matrix) | → `render_own_line_then_body_substitutes_span_in_situ`. |
| `render_multiline_leaf_on_scaffolding_line_refuses_license_and_runs_verbatim` (matrix) | → `render_multiline_leaf_on_scaffolding_line_substitutes_cleanly` (d-6: re-derive to the NEW behavior — multi-line is newly expressible). |
| `f1_status_consumed_by_if_guard_blocks_replacement` (matrix) | UNCHANGED assertion (still passes): the guard `apt-get install` is a mutator (⊤ rc) ⇒ Relaxable+⊤ blocks, same Run disposition. Comment updated to name Relaxable. |
| `render_one_liner_case_arm_body_substitutes_in_situ_keeping_arm_structure` (matrix) | UNCHANGED (still passes — `nginx) true ;;` is exactly the span-edit form); comment carries forward. |
| `consumed_andand_left_operand_marks_relaxable_status` + the `!StatusRenderFloor` assertions across cfg | the distinguishing assertion flipped from `!StatusRenderFloor` to `!StatusIterated` (a `&&`/`||` operand is never a loop condition). |
| The F2 scaffolding pins (`render_post_loop_install_sharing_done_line_…` etc., matrix) | UNCHANGED (still pass) — they assert `done; true   # dorc: elided` which is a prefix-substring of the new comment, and the scaffolding-kept behavior is preserved by the span edit. |

## §8 d-10 verdicts (20O find-6 latents + 20S §10 hunt-4 frozen shapes vs the new render)

**20O find-6 latents** (the member-elision preconditions; were "floor-masked"):
- post-`while` `$?` marks the body (item-6a): UNCHANGED — the cfg marking is `StatusRelaxable` on
  the body, untouched; the while CONDITION's mark moved RenderFloor→StatusIterated but stays inert
  (the body-mark is what matters). `consumed_post_while_dollar_question_marks_body…` re-homed
  (asserts StatusIterated on the cond now) and PASSES. **unfrozen-and-correct.**
- `done > file` body-stdout consumption unmarked: UNCHANGED — this is a cfg consumption gap, not a
  render concern; the render doesn't touch it. **still-blocked-why: orthogonal to arch-1** (a cfg
  marking gap, deferred).
- in-loop Queries still compiled into the probe: UNCHANGED — task-L2 item-6b excludes them;
  `render21-while-guard-floored` confirms the in-loop Query condition is `skip-unresolvable`.
  **unfrozen-and-correct.**

**20S §10 hunt-4 frozen shapes** (the Members render hunt — "a Members body NOT a one-liner, or
sharing its line with a SIBLING, or a member whose elision leaves an empty arm; does the in-situ
splice stay dash-n-clean?"): Under the span render, a Members body leaf edits its OWN span to
`true` regardless of line layout (no scaffolding-line special-case needed) ⇒ the multi-line /
sibling-sharing concerns DISSOLVE (the span edit is layout-agnostic). I verified
`loop-members-all-converged-elides` renders `for pkg in nginx curl; do true; done` (span-edit), and
the other 3 loop-members cases keep IDENTICAL run-sets (they RUN the body — no elision — so no
render change). **newly-correct: the frozen multi-line/sibling Members shapes are no longer a
render risk; an empty arm cannot arise (the span becomes `true`/`:`, never empty).** A `do apt-get
install "$pkg"; echo done` (sibling on the body line) would now edit only the install span and keep
`echo done` — +SURE clean (the multi-edit-per-line splice is pinned by the 3-replace hunt, §9).

**The 20S Members run-sets:** loop-members-partial-runs, loop-member-external-writer-runs,
loop-var-body-reassign-tops — all IDENTICAL run-sets (git shows no golden change for them);
loop-members-all-converged-elides churned textually only (justified above). +SURE.

## §9 Adversarial hunt-list (WRITE-IT-YOURSELF — ranked; a hostile crosscheck must EXCEED this)

Hostile-identity framing: "a builder I distrust rewrote the ENTIRE apply render from line-granular
to byte-span splicing, and widened two licenses (if-guard → substitutable, multi-line → expressible)
— the one place disaster-class bugs (broken `dash -n` artifacts shipped green, or a wrong-elision)
live. Find where a span edit corrupts the artifact, or elides a leaf whose observable a downstream
context still reads." Construct every probe against dash (the semantic oracle), not the engine.

- **hunt-1 (HIGHEST) — span-edit overlap / containment under nested constructs.** `normalise_edits`
  asserts disjoint-or-contained; I claim no current shape produces a containing construct-edit
  (only leaf commands edit). ATTACK: a leaf whose span is reported by the parser to OVERLAP a
  sibling's (a span bug — the parser is the highest-risk surface). Candidates: a command whose span
  includes a trailing redirect that the NEXT command's span also claims; `a; b` where the spans
  abut (lo==hi) — does the `e.lo < prev.hi` guard correctly NOT treat abutment as overlap? Verify
  `for x in a; do install; done; install2` (two installs, spans must be disjoint). The
  `debug_assert` fires in debug — run the hostile corpus under a debug build and watch for it.
- **hunt-2 — comment-safety on weird line shapes.** `comment_safe` drops on trailing `\` or any
  `<<`. ATTACK the BOUNDARY: a line that ENDS an edit but whose post-edit content has a `#` already
  (does appending ` # dorc:` after an existing `#` matter? — no, it's all one comment, but verify);
  a line with `<<` that is NOT a heredoc (`echo "a << b"` — a quoted `<<` ⇒ `comment_safe` returns
  false CONSERVATIVELY, dropping the comment even though it'd be safe — a missed disclosure, not a
  corruption, but confirm it's a drop not a break); a CRLF line (`\r` before `\n`) — does the
  `trim_end` + `<<` check still hold? a line ending in `&` (background — though `&` ⊤-rejects); a
  `case` arm `;;` line where the comment lands after `;;` (verified clean, but a `;&` fallthrough?).
- **hunt-3 — the if-guard substitution channel gating.** The new `StatusRelaxable` if-guard elides
  ONLY with a known rc + Query (no mutation). ATTACK: an if-guard that is an establish with a
  DECLARED rc (none exist — fork-mutator-rc — but if a future oracle declares one, does it wrongly
  elide a mutator-guard?); an if-guard reading consumed Stdout (`if out=$(command -v x); then` — the
  `$()` capture); an if-guard with a `/dev/null` redirect vs a real-sink redirect (the real sink
  must still block — `if dpkg -s x > log; then`); a MULTI-command if-condition (`if a; b; then` —
  the deciding leaf is `b`, but `subtree_leaves_all` requires BOTH a and b neutralised for the body
  to elide — is that too strict, or correct? I claim correct: the body is dead only if the WHOLE
  cond's decision is reproduced, which needs every cond leaf substituted).
- **hunt-4 — the while/until anti-pole (StatusIterated).** Verify `until` (not just `while`) marks
  StatusIterated; a `while` with a MULTI-command condition (every cond command StatusIterated?); a
  `while` whose condition is a known-rc Query that SHOULD still block (it does — StatusIterated is
  unconditional); a `while true; do …; done` (infinite by design — does anything try to elide the
  `true`? the body floor + StatusIterated should both refuse). The disaster shape: confirm NO path
  substitutes a loop condition with a constant (the infinite/zero-iteration bug). Construct
  `while CMD; do …; done` under dash with CMD's rc varying and confirm the engine NEVER produces a
  constant-condition artifact.
- **hunt-5 — door-3 interaction with the span render (note 213 §5 hunt-6, re-attacked).** Confirm
  `door3-or-true-elides` passes for the RIGHT reason (mutator span → `true`, `true` rhs verbatim,
  NOT a spurious fold) — I verified `argv 1 replace, argv 2 run true`. ATTACK the chain shapes
  (res-4): `cmd || true || other` (the install stays Run — I verified verbatim); `cmd || true;
  echo $?` (res-2, the `$?` over-mark blocks — verify the install runs); `(cmd && x) || true`
  (hunt-1 from 213). And the dual: does any `|| true` shape now WRONGLY elide because the span
  render can express what the line render couldn't? (door-3's MARK is unchanged; only the render
  changed — so a door-3 mint that was inert-at-HEAD is now expressed. Confirm each expressed mint is
  sound.)
- **hunt-6 — the omit-safety compound-controller walk (strain-1).** `subtree_leaves_all` is the
  riskiest new code. ATTACK: a controller whose subtree has a leaf the walk MISSES (a funcdef call
  in the cond — functions are detached, the call IS a Simple leaf, but its BODY isn't walked — is
  that right? the call-leaf's own disposition is what matters, +SURE); a deeply-nested cond
  (`if ! { a && b; }; then`); a cond with a ⊤/Unsupported leaf (does `subtree_leaves_all` return
  true for the `_ => true` arm on an Unsupported node, wrongly marking the body neutralised? — an
  Unsupported leaf is NOT a Step, so it hits `_ => true`... ATTACK: `if eval x; then install; fi`
  where `eval` ⊤-rejects — does the install wrongly elide? It shouldn't: the fold won't mark the
  body dead under a ⊤ cond (the fold's `eval_if` ⊤-stops), so there's no Omit to mis-resolve. But
  verify the COMPOSITION — a ⊤ cond leaf + a known-rc cond leaf in the same `&&`).
- **hunt-7 — multi-line + multi-edit splice arithmetic.** The `region_lo..region_hi` +
  right-to-left splice is index arithmetic over bytes. ATTACK: a multi-line edit whose region
  contains ANOTHER edit (two multi-line leaves overlapping lines); a multi-line edit at EOF (no
  trailing newline — does `line_start.get(last+1)` fall back to `src.len()` correctly?); a CRLF
  source (the `\n`-counting `line_of` vs the `\r` bytes — the splice offsets are absolute byte
  positions, so `\r` is just a byte in the region, preserved verbatim — but confirm the comment
  lands AFTER the `\r`); UTF-8 multi-byte operands (the span offsets are byte positions —
  `replace_range` on a non-char-boundary PANICS; do any spans land mid-char? the parser's spans
  should be char-aligned, but a hostile UTF-8 operand is worth a probe).
- **hunt-8 (lower) — provenance disclosure fidelity.** The flattened original collapses ALL
  whitespace (`split_whitespace().join(" ")`) — a command with intentional multiple spaces or tabs
  is mis-disclosed (cosmetic, not a correctness bug, but confirm it never changes the ARTIFACT, only
  the comment). And a command whose text contains `]` (the comment's bracket delimiter) — does it
  confuse a reader/parser? (It's inside a `#` comment ⇒ inert; cosmetic only.)

## §10 Flags (tc-*/doc-deltas — surfaced, not resolved)

- **doc-delta-1 (orchestrator/human to apply — I don't touch CLAUDE.md):** `spike/CLAUDE.md`
  `inv-one-observable` paragraph, `crates/analysis/CLAUDE.md` (the `cfg.rs` bullet:
  "`StatusRenderFloor`-consumed"), and `crates/plan/CLAUDE.md` (the consumed-Status paragraph) all
  describe `StatusRenderFloor` as a LIVE channel. It is DELETED. The consumed-Status axis is now
  {StatusRelaxable | StatusInvariant | StatusIterated}: Relaxable (a KNOWN rc reproduces the
  decision — `&&`/`||` operands, errexit commands, `$?`-predecessors, AND if/elif guards),
  Invariant (the consumer decides nothing — `cmd || true`), Iterated (a `while`/`until` condition —
  the per-pass sequence, unconditional block). The "render-expressibility" framing is retired
  (the leaf-exact render removed render capability as an axis); the three live channels are keyed on
  REAL reasons (value / continuation-identity / iteration). Proposed: replace every
  `StatusRenderFloor` mention with the StatusRelaxable(if-guard)/StatusIterated(loop) split.
- **doc-delta-2:** `crates/plan/CLAUDE.md`'s "Tension to flag" § (the `an-render-modes` line-granular
  vs leaf-exact tension, and the `--WONDER whether a faithful control-flow rewrite … reconciles
  them`) is RESOLVED by this round: the leaf-exact render IS that faithful rewrite, and it
  reconciles render-fidelity with leaf-exact provenance (the back-map is now byte-exact, not
  line-blurred). The "comment the elided leaf in situ with a `:`-stub" hypothesis is exactly what
  shipped (a `:` for dead Omits, the stand-in for Replaces). Worth recording the resolution.
- **tc-heredoc-diagnostic-boundary (strain-4):** the heredoc-refusal diagnostic rides a SEPARATE
  `Plan` method the cli calls, not the render's return. ~SUSPECT right (keeps render pure); a
  stricter "render returns its own diagnostics" reading is a judgment call — flagged.
- **tc-multiline-original-flatten (strain-2):** the provenance comment flattens a multi-line
  original's whitespace. Cosmetic, but it's a disclosure-fidelity choice (the human sees
  `[apt-get install -y "multi line"]`, not the literal newline). Flagged in case disclosure
  fidelity is later weighted.
- **NOT a tc, but worth the orchestrator's eye:** the two newly-expressible run-set shrinks
  (exec-multileaf-line-mixed, headline-guarded-realistic) are SEMANTIC golden changes. They are
  sound (a converged/known-rc leaf sharing a line with a Run leaf now elides — the leaf render's
  core win), but they DO change what the apply runs on a real host. The orchestrator's bless-verify
  must inspect these two `expected.ran` diffs by eye (done here; re-confirm).

## §11 Confidence summary

- +SURE: the span render is behavior-preserving on run-sets except the documented {payoff,
  newly-expressible} set; all 75 e2e ×2 green zero-xfail; 361 workspace tests; fmt/clippy-D/typos
  clean; the carve-out family + StatusRenderFloor variant DELETED (zero non-doc refs).
- +SURE: door3-or-true-elides passes for the right reason (Replace span-substituted, `true` rhs
  verbatim); the if-guard pole pair + while anti-pole + heredoc refusal + multi-line pin all
  exec-gate the intended quadrants.
- +SURE: the d-5 audit found ZERO existing-case disposition flips (the if-guard cases are Pure or
  establish-guards, not known-rc Queries); the capability is proven by the NEW case.
- ~SUSPECT: the compound-controller `subtree_leaves_all` walk (strain-1) is the riskiest new code —
  it's where a hostile crosscheck should dig (hunt-6); I believe it's sound (a body elides only if
  every cond leaf is neutralised) but the ⊤-leaf-in-cond composition (hunt-6) is the corner I'm
  least sure of.
- ~SUSPECT: the heredoc-diagnostic boundary (tc-heredoc-diagnostic-boundary) and the multi-line
  original flatten (tc-multiline-original-flatten) are the two judgment calls I made locally that a
  reviewer might re-decide.
- --WONDER: whether the in_loop_body covering the while CONDITION (§2) is the cleanest design — the
  StatusIterated mark is defense-in-depth given the floor, which is slightly redundant; a future
  round that lifts the in-loop floor (member-elision for loop conditions?) would rely on
  StatusIterated alone, so keeping it honest now is correct, but the redundancy is worth a note.
