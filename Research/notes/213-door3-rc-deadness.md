# 213 — door-3 (rc-deadness): the `cmd || true` Status refinement

> Round-21 w-1, builder note (slug 213 reserved per 211 §4). Charter: `plans/20V` §4 door-3
> + 211 §1 arch-3(a). Append-only; confidence-marked (+SURE/~SUSPECT/-GUESS/--WONDER).
> Engine edits: `core/src/lib.rs` (Channel variant), `analysis/src/cfg.rs` (`lower_and_or`
> marking + `right_is_bare_true`), `plan/src/lib.rs` (`consumption_ok` + the Top→True
> stand-in comment). Tests: `analysis/tests/cfg.rs` (+7 classification pins),
> `plan/tests/observable_matrix.rs` (+5 disposition pins), 4 new e2e cases (`door3-*`).

## §0 What shipped (file:line, the actual delta)

- **d-1** `core::Channel::StatusInvariant` (`core/src/lib.rs` ~358) — a THIRD status
  channel beside `StatusRenderFloor`/`StatusRelaxable`. Semantics: consumed-in-form,
  dead-in-fact; never blocks a license, even at ⊤; still RECORDED in the consumed set
  (disclosure/provenance sees the read). No exhaustive `match` over `Channel` exists in
  non-test code (+SURE, grepped) — the closed enum is consumed only via `.contains()`, so
  the variant added with zero `match`-arm churn.
- **d-2** the marking rule (`analysis/src/cfg.rs` `lower_and_or` ~716 + `right_is_bare_true`
  ~1301). `door3 = (op == Or) && right_is_bare_true(right)`; when true the left operand's
  arena range is marked `StatusInvariant` instead of `StatusRelaxable`. `right_is_bare_true`
  = `NodeKind::Simple` with empty `assigns` + empty `redirs` + exactly one word literal
  `true` (the `word_literal` path already excludes command-subst). The `_op` param (unused
  at HEAD) is now used.
- **d-3** the mark-union (`plan/src/lib.rs` `consumption_ok` ~443) — `StatusInvariant` is
  absent from every block clause ⇒ a site carrying ONLY it passes even at ⊤; a site ALSO
  carrying a blocking mark (Relaxable/RenderFloor/Stdout) is still blocked by THAT mark.
- **d-6** stand-in: the `Predicted::Top => StandIn::True` arm in `disposition_for` (~1047)
  now also serves door-3; its comment names invariance (not a predicted rc-0) as the
  license, keeping weld-5 intact.

Acceptance (all green, run twice): `cargo fmt --check` · `clippy --workspace --all-targets
-D warnings` (no new expects) · `cargo test --workspace` (analysis cfg 47→54, plan matrix
24→29) · `sh e2e/run.sh` ×2 (66→70 cases, all six gates; the 4th door3 case is an EXPECTED
xfail — see §2) · `typos spike`. Zero churn on the existing 66 goldens (+SURE, `git diff
--stat` on existing `expected.*` empty).

## §1 The one thing that strained hard: the render wall (d-5 checkpoint-2 FIRED)

+SURE, this is the load-bearing finding and a **tc-flag** for the orchestrator. The
Status-channel refinement is correct and complete at the PLAN level: a converged
`cmd || true` mutator's disposition flips `Run → Replace(_, True)` — the charter's stated
behavioral delta ("sites whose only blocking mark was the ||-true consumption now mint").
Verified live (`--debug-argv`): `apt-get install -y nginx || true` (converged) →
`argv 1 replace`. **But the round-21 line-granular render cannot EXPRESS that mint**, so
the payoff case's empty-run-set is unreachable without a render extension d-5 explicitly
forbids door-3 from making.

The mechanism, exhaustively traced (four independent confirmations):
- The line `cmd || true` has TWO leaves: the mutator (left, now `Replace`) and `true`
  (right). `true` is a blessed target-state-pure builtin (`effect.rs::is_target_state_pure_builtin`
  lists `true`), so it classifies `Pure` ⇒ `disposition_for` final arm ⇒ `Disposition::Run`
  (+SURE).
- In `classify_lines`, a `Run` leaf puts the line in `run_lines`; in `emit_apply_lines`,
  `run_lines` WINS over `neutral_lines` (the whole line renders verbatim). So the mutator is
  NOT substituted — the artifact runs `apt-get`, run-set non-empty.
- The two render paths that DO collapse a `cmd_a || cmd_b` line to a single `true`
  (`fold-oror-guard-omits`, `exec-query-guard-composition`) work ONLY because the LEFT's rc
  is KNOWN (a probe-sourced Query rc 0), letting the fold `Omit` the right operand as dead.
  door-3's left rc is ⊤ (mutator) ⇒ the fold can't omit the `true` ⇒ it stays `Run`. This is
  the precise asymmetry (+SURE).
- The honest target artifact is `true || true` (splice the mutator's span in-situ, keep
  `|| true`). That needs the in-situ splice machinery (`scaffold_subst`/`inline_subst`),
  which is GATED on scaffold-keyword lines or case-arm one-liners (`scaffolding_boundary_lines`
  / `case_arm_oneliner_leaves`); a bare `||` line is neither (+SURE, read both helpers).

Empirical proof (a throwaway experiment, reverted): neutralising the `StatusRelaxable`
block in `consumption_ok` (simulating door-3) flipped `argv 1` to `replace` while
`argv 2 true` stayed `run`, and the rendered apply was `apt-get install -y nginx || true`
VERBATIM. So even with the license minting, the line renders unchanged.

**Disposition taken**: built the full mechanism + all unit pins + the 3 render-INDEPENDENT
e2e cases; authored the payoff case `door3-or-true-elides` as an **XFAIL** that asserts the
CORRECT behavior (empty run-set) and is expected to fail until arch-1 (leaf-exact render,
wave-2) lands. It will XPASS (loud, "promote") the moment the render can collapse the line.
This respects d-5 (no render extension) AND the deliverable (the named payoff case exists,
asserting the right thing). **Flag for orchestrator**: confirm the XFAIL choice vs. cutting
the case; and confirm door-3's value-realisation is correctly deferred to arch-1 (the wave
plan §3 orders door-3 BEFORE arch-1, so this deferral looks intended, but the charter d-5
text says "STOP and report", which I read as "don't extend render", not "abandon the
mechanism" — the mechanism is the durable spike product and is inert-but-correct until
arch-1).

## §2 d-4 checkpoint-1 result: PASSED (errexit exempts the `||`-left)

+SURE. Traced + empirically confirmed: in `set -e; cmd || true`, the mutator (left of `||`)
is NOT errexit-marked. `lower_and_or` lowers the left via `lower_condition_region(left, _,
false)`, which calls `clear_fallible_range` ⇒ the left's nodes have `fallible=false` ⇒
`materialise_errexit_edges` (gated on `self.fallible[v]`) neither adds a failure-edge nor a
`StatusRelaxable` mark to them. So the mutator's ONLY status mark is the one `lower_and_or`
itself places — which door-3 changes to `StatusInvariant`. Empirical: the d-4 experiment
(neutralise the single `StatusRelaxable` gate) flipped the converged mutator to `replace`,
proving that mark was the sole blocker. No separate semantic change was needed; the e2e
payoff is blocked by render alone, not by an errexit double-mark.

## §3 Residuals (ranked by how much they'd bite)

- **res-1 (the big one) — the render wall**, §1. The payoff is XFAIL'd. Structural argument
  for unlock: arch-1's leaf-exact render substitutes the mutator's byte-span directly (no
  whole-line/`run_lines`-wins coupling), yielding `true || true` — dash-clean, empty
  run-set. +SURE this is exactly the arch-1 capability; ~SUSPECT arch-1 will need a pole-pin
  for "a `Replace` leaf sharing a line with a `Run` leaf" (the door-3 line is the canonical
  instance — feed it to arch-1's crosscheck).
- **res-2 — `cmd || true; echo $?` stays blocked** (pinned: `door3_oror_true_then_dollar_question_runs_residual`).
  The `$?`-predecessor pred-walk marks BOTH `cmd` and `true` `StatusRelaxable`; mark-union ⇒
  `cmd` = {StatusInvariant, StatusRelaxable} ⇒ the Relaxable+⊤ blocks ⇒ `cmd` runs.
  Acceptable-conservative (kFAIL-perform). Structural argument it COULD later unlock (+SURE
  the rc is invariant, ~SUSPECT the analysis to prove it is non-trivial): `$?` after
  `cmd || true` is ALWAYS 0 — the LIST rc is invariant regardless of `cmd`. A future refinement
  could mark a `$?`-read whose nearest enclosing construct is a `|| true` (or any
  identical-rejoin construct) as reading an INVARIANT value, and then the predecessor mark
  could itself be Invariant. Not this slice; the conservative block is sound.
- **res-3 — the chain over-mark on `a`** (verified: `a = {StatusRelaxable, StatusInvariant}`
  in `a || b || true`). The outer `|| true`'s Invariant lands on the WHOLE outer-left range,
  so it also marks `a` (the inner-||'s left). INERT (a's Relaxable still blocks), but
  IMPRECISE for disclosure — a's provenance would spuriously show "||-true-consumed". Not
  pinned as a contract (a future precision pass could mark only the direct left operand). +SURE
  harmless; ~SUSPECT worth a precise sub-range mark someday if disclosure fidelity matters.
- **res-4 — `||` chains where the bare-`true` is NOT the outermost.** e.g. `cmd || true || other`
  parses `(cmd || true) || other`: the INNER `|| true` marks `cmd` Invariant, but the OUTER
  `|| other` marks the whole `(cmd || true)` range `StatusRelaxable` (over `cmd` AND the inner
  `true` node). So `cmd` = {Invariant, Relaxable} ⇒ blocks. --WONDER whether that's the
  RIGHT answer: `cmd`'s rc IS read by the outer `|| other` (if `cmd||true` "fails"... but it
  never fails — list rc is always 0, so `other` never runs). So `other` is actually dead and
  `cmd`'s rc is still invariant. The current engine conservatively blocks (Relaxable wins) —
  sound, but a missed door-3 opportunity. NOT in the charter's slice; flagged as a latent. A
  hostile crosscheck should probe whether this conservative block is the intended boundary.

## §4 doc-delta for `inv-one-observable` (orchestrator to apply — human-auth files, I don't touch)

The `inv-one-observable` paragraph in `spike/CLAUDE.md` (and the mirrored prose at
`core::Channel` and `plan/CLAUDE.md`) currently says the *consumed* Status splits **two**
ways by render-expressibility: `StatusRenderFloor` (the lone if/elif guard) vs
`StatusRelaxable` (the four readers a known rc reproduces). door-3 adds a **third** variant
on a **different axis**. Proposed insertion (after the StatusRelaxable sentence):

> A third consumed-Status variant, `Channel::StatusInvariant` (door-3, `20V` §4 / note 213),
> splits on a DIFFERENT axis than render-expressibility: whether the consumer's branches
> rejoin with identical observables. The `cmd || true` left operand is consumed-in-form
> (the `||` reads the rc) but dead-in-fact (both continuations yield list-rc 0, no
> observable, `$?`=0, errexit sees 0) ⇒ it NEVER blocks a license, even at ⊤, and needs no
> known rc (unlike Relaxable). It is still RECORDED in the consumed set (disclosure sees the
> read). So the consumed-Status axis is now {render-floor | value-relaxable | dead-invariant},
> the first two keyed on render/value capability, the third on continuation-identity.

~SUSPECT the cleanest framing for the human: Relaxable asks "can a known rc reproduce the
consumer's DECISION?"; Invariant asks "does the consumer DECIDE anything observable at all?".
A `|| true` decides nothing (both arms identical), so no rc — known OR ⊤ — matters.

## §5 Adversarial hunt-list (WRITE-IT-YOURSELF — ranked; a hostile crosscheck must EXCEED this)

Hostile-identity framing: "a builder I distrust widened a license surface (the one place
disaster-class bugs live) by adding a 'never blocks' channel. Find where the never-block is
WRONG — a `|| true` whose continuations are NOT actually identical, or a mark that should
have blocked and didn't." Construct every probe against dash (the semantic oracle), not the
engine's self-report.

- **hunt-1 (HIGHEST) — the marks-union edge / does Invariant ever WRONGLY suppress a block?**
  The whole safety rests on `consumption_ok` blocking when ANY blocking mark is present, with
  Invariant merely declining to block. Attack: find a site that SHOULD block (a live rc reader)
  but ends up with ONLY Invariant. Candidates: a `|| true` whose left is itself a compound
  whose interior has a live reader the range-mark mis-covers; a lowering order where the
  outer Invariant mark is applied to a node that a LATER pass expected to be Relaxable.
  Verify `a||b||true` (pinned), `cmd||true||other` (res-4, NOT pinned — attack it), and
  `(cmd && x) || true` (the left is an `&&` — does `cmd`'s `&&`-Relaxable survive the outer
  Invariant over-mark? it must, since `&&`'s left rc is live). +SURE the gate logic is
  block-wins; ~SUSPECT the range-marking granularity is where a hole would hide.
- **hunt-2 — errexit-exemption interaction.** d-4 says the `||`-left is errexit-exempt so the
  mutator carries no errexit mark. Attack the COMPOSITION: `set -e; cmd1; cmd2 || true` — is
  `cmd1` (a bare errexit-region command, NOT a `||` left) still correctly `StatusRelaxable`
  -blocked? (It must — door-3 only touches the `||`-left.) And `set -e; { cmd || true; }` —
  does the group boundary leak the exemption? And the negated form `set -e; ! cmd || true`.
  Construct each under dash and confirm the abort-vs-no-abort matches the engine's
  block-vs-unlock.
- **hunt-3 — the chain shape (res-4) and right-nesting.** `cmd || true || other`,
  `cmd || other || true`, `a || b || c || true`. Trace which operands get Invariant vs
  Relaxable and whether each matches dash's actual list-rc invariance. res-4 is the prime
  suspect (conservative block that MIGHT be a missed unlock, or MIGHT be hiding a real
  blocker — decide which).
- **hunt-4 — the bare-`true` predicate's boundary.** Attack `right_is_bare_true` for
  false-POSITIVES (a non-inert `true` that slips through): `true` as a FUNCTION (`true() {
  rm -rf /; }; cmd || true`) — the predicate matches the literal word, but a redefined `true`
  is NOT inert. +SURE this is a real hole shared with EVERY `true`-stand-in in the engine
  (the whole codebase assumes `true` is the builtin); --WONDER if door-3 widens the blast
  radius (it mints a Replace where HEAD ran). Also: `\true`, `'true'` (quoted — `word_literal`
  matches SingleQuoted, so `|| 'true'` WOULD qualify — is that right? a quoted `true` is the
  same builtin, so yes, but confirm), `${x:-true}` (param expansion — `word_literal` returns
  None ⇒ correctly excluded), `true;` vs `true` (trailing-sep — does the parser produce a
  bare Simple or a List? if List, the predicate returns false ⇒ conservative miss, safe).
- **hunt-5 — false-NEGATIVES that prove the slice is too narrow (lower priority, not a safety
  bug).** `|| :` (pinned-deferred), `|| true >/dev/null` (pinned-refused), `cmd ||  true `
  (extra whitespace — should still match), `cmd|| true` (no space). Confirm the deferred ones
  are deferred by CHOICE, not by accident, and that whitespace doesn't break the match.
- **hunt-6 — the render wall, re-attacked from the OTHER phase.** The XFAIL asserts empty
  run-set. Attack: is there a render path TODAY that would make it pass for the WRONG reason
  (e.g. the mutator gets `Omit`'d via some fold, not `Replace`'d via door-3)? Confirm the
  xfail fails for the RIGHT reason (run_lines-wins verbatim), so when it XPASSes under arch-1
  it's the render fix, not a spurious fold. And the dual: does any EXISTING case's render
  change because door-3 now marks something Invariant? (No existing `|| true` in the corpus —
  +SURE, d-7 grep — but confirm no `&&`/`||` test regressed: the suite says no.)

## §6 What surprised me

- ~SUSPECT-turned-+SURE: I expected the mechanism to be the hard part and the e2e to be
  mechanical; it INVERTED. The Channel/marking/gate work was ~40 lines and clean (the
  `inv-superposition` design — engine emits, plan collapses — absorbed door-3 with one new
  variant and one gate clause). The e2e payoff is what hit the wall, exactly at the
  render/leaf-seam boundary the round is concurrently rebuilding. The charter's d-5
  checkpoint PRE-REGISTERED this; it fired precisely as written.
- +SURE: door-3's mechanism is INERT on the entire existing corpus (zero `|| true` shapes,
  d-7) — its only activation site is the one case that can't render. So the mechanism build,
  end-to-end, is currently testable ONLY via unit pins (which inspect disposition/consumed
  directly), not via the e2e exec gate. That's an honest consequence, not a gap: the unit
  pins ARE the verification, and they're strong (they pin the consumed-set classification AND
  the disposition flip AND every negative pole).
- -GUESS: the `||`-left errexit-exemption (d-4) being already-correct meant door-3 needed NO
  errexit change. The comment at `cfg.rs:1117` ("pruned ... + `|| true`") had already
  anticipated `|| true` in the errexit pass — but that pruning is about the `true` node's own
  fallibility, orthogonal to the LEFT operand's exemption (which comes from
  `lower_condition_region`'s `clear_fallible_range`). Two unrelated mechanisms, both
  pre-existing, both pointing the same way.
