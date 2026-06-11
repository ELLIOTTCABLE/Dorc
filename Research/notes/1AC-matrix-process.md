# 1AC — D4 matrix process note (what was verified, what strained, what's flagged)

> **Disclosure:** LLM-generated (round-1A matrix agent); part of the intentionally
> quality-varied ARTIFICIAL N-of-1 testing-corpus effort (see 1A1
> rul-1A-llm-disclosure). Not real ops code; the corpus and seeds are frozen evidence,
> never executed; an artificial corpus cannot expose the truth of real-world ops-code.
> Working note behind `Research/plans/1AA-capability-matrix.md` (DRAFT, pending the
> hostile crosscheck). Append-only.

## §1 Engine claims: confirmed vs taken-on-word

CONFIRMED by direct source read (file:line in 1AA §4; all +SURE):
- c-1 value plane: Flat height-2 map domain, entry-⊤ seed, ⊤⇒run floor,
  non-convergence ⇒ all-⊤ (value.rs:1-37, 298-303, 509-520).
- c-2 cmdsub/arith/operator-expansion in a word ⇒ that word ⊤ (value.rs:976-1021); a
  var fed by cmdsub is ⊤ via the same recipe path.
- c-3 effect plane: argv threaded through the oracle's own check; Opaque on ⊤
  word/no-check/no-cell; the exact 15-builtin pure set incl. `printf`/`echo`/`true`
  (effect.rs:92-201, 285-304).
- c-4 NO redirection handling in effect.rs (grep: zero matches) — and SHARPER than the
  briefed claim: a `Redir` CFG node classifies **Pure** (effect.rs:532-539), so
  `printf … >> f` is an *invisible* mutation (no poison), not merely an un-modeled one.
  `cat > f` still poisons, but only because `cat` is an un-oracled external.
- c-5 CFG: ForLoop/WhileLoop lower to a real cyclic LoopHead+back-edge; Case lowers;
  FuncDef bodies detached with pass-through definition (cfg.rs:793-939, 844-868);
  in-loop render-floor (cfg.rs:198-206); `exit`/`return` route to program-exit with no
  fall-through (cfg.rs:626-630) — load-bearing: the L38 root-check's `exit 1` does NOT
  ⊤-poison the book (I checked specifically because `exit` is absent from the
  pure-builtin set).
- c-6 ⊤-trigger set as briefed (ast.rs:295-313; Loop variant 303-310), Loop residue
  incl. break/continue.
- c-7 the an-* statuses cited in the matrix, against ANALYZER-NEEDS.md's st column.
- c-8 census numbers used as given (mechanical, 1A7 self-test green); spot-consistent
  with my full read of harden.sh (e.g. `continue` ×2 at L573/591 — the lines that
  ⊤-reject both while-loops).

TAKEN ON WORD (not read this session; ~SUSPECT-confirmed):
- w-1 oracle crate internals: the check-evaluator's `Word`/`TestOp` sub-dialect,
  `resolve_probe`, and "annotation-reached-but-no-probe ⇒ Resolution::Top" — from 1A8's
  ground-truth-lift section + effect.rs's own CORPUS_CHECK_SRC test comment
  (effect.rs:655-690), which is engine-adjacent but not the crate itself.
- w-2 `plan::disposition_for`'s honoring of the in-loop floor and the
  consumed-observable gates — from cfg.rs doc-comments + spike/CLAUDE.md.
- w-3 `solve`'s iteration cap + converged flag semantics — from doc-comments.

## §2 imp-*/um-* review: no mis-scores found; three enrichments

Reviewed every 1A6 §3 imp-* and 1A8/1A9 um-* row against the engine read. None
mis-scored. Three places the matrix *sharpens* (not corrects) them:
- e-1 1A8 frames refusals as "→ run; never wrong-elide". Engine-precisely a refusal is
  `Opaque` ⇒ run AND ⊤-poison downstream reaching-defs (effect.rs:464). The refusal
  posture stays right; its *cost* (all downstream ambience) was unpriced. 1AA head-3.
- e-2 um-file-1's "no provider token" is, at HEAD, *stronger* than written: the
  redirect-write isn't just un-keyable — with `printf` blessed-pure it generates no
  effect at all, so a future confline-style Query downstream could read `valid=true`
  against a file the book itself just appended (the rule-query-validity bit,
  effect.rs:624-635, sees nothing). Latent today only because ambient Opaques already
  zero every validity bit in this corpus. Feeds tc-M2.
- e-3 um-file-restart-1's "the engine's value-flow following the flag variable" is
  half-true at HEAD in an unexpected way: the flags it would follow are ⊤ at all four
  guards — A's via honest branch-joins, but B's welded-1 flags (which WOULD resolve to
  a concrete `1`) are erased first by the §9 ⊤-loop havoc (value.rs:325-331). The
  blast-radius coupling between B6 and B10 was in neither seed note; it is 1AA's y-7.

## §3 What strained

- s-1 the matrix wanted an elision-rate column; refused it (N-of-1 + head-2's
  zero-or-near floor at HEAD makes any % misleading). Scope-guarded in 1AA §3.
- s-2 "criticality" needed a working definition before scoring (execution/ordering
  breakage under engine mis-modeling, never security); chose one and flagged it (tc-M5)
  rather than scoring under an ambiguity.
- s-3 the A9/B5 cell forced reading the task-L2 license mechanics closely; the result
  (self-reach pristine = empty-set, effect.rs:478-483) is so strict it can never fire
  on this corpus — see tc-M1. Scoring "modeled" next to "never licensable here" felt
  contradictory; resolved by grading the *machinery* modeled and flagging the license.
- s-4 value-plane call-transparency (value.rs:386-399): a function call havocs nothing,
  so a body-written variable read downstream resolves to a stale pre-call constant.
  Strawman: `c=0; f(){ c=1; }; f; [ "$c" -eq 1 ] && service x restart` — `$c` resolves
  `0`. At HEAD no consumer folds guard values, so it is latent, but it is a
  wrong-CONCRETE channel (the no-floor class), not mere imprecision. In this corpus the
  one exposed variable (`sshd_changed`, written at L177 inside `set_sshd_line`) happens
  to be masked by top-level branch-joins to ⊤. Folded into y-2's scoring.

## §4 tc-flags — cross-cutting calls NOT settled here (flag up, don't resolve)

- **tc-M1 (self-reach strictness).** `self_reach_holds` requires the suppressed-solve
  in-state be EMPTY (effect.rs:478-483): ANY upstream establish — even an unrelated
  cell like `package:sudo#installed` — refuses every Members license book-wide. The
  in-code rationale only needs "no writer of MY cells reaches me"; the implementation
  tests "no writer of ANY cell reaches me". Possibly deliberate round-narrowing
  ("bias every ambiguity to ineligible"), but as-is the L2 license is unreachable on
  any realistic book (corpus evidence: both for-loops sit under §1-2 mutators).
  Engine-owner call: per-family-cell foreign-writer test vs the global pristine.
- **tc-M2 (fix-ordering weld).** an-redirection-effect must land BEFORE (or with)
  (a) brk-2 inlining and (b) any confline-style grep Query provider. Either alone arms
  e-2's stale-validity channel: inlined grep-guards + invisible printf-appends ⇒
  `QueryResolvable { valid: true }` over book-mutated files. Sequencing is an
  orchestrator decision; the matrix only welds the dependency direction.
- **tc-M3 (provider collision is now live).** effect.rs:139-151 takes the FIRST check
  that resolves, commented "no corpus case is ambiguous". The D3 seed-set breaks that
  premise: `crond.oracle.sh` and `fetched.oracle.sh` both key provider `test`
  (1A9 tc-F2). Loading both makes `[ -e X ]` routing file-order-dependent. Same
  question as tc-F2/F3 (should builtins/read-commands be provider-keyable at all);
  needs a ruling before the seeds are ever loaded together.
- **tc-M4 (unregistered machinery).** Three cheap-looking precision moves have no an-*
  row: (i) unquoted-heredoc BODY expansion through the value plane (would make 4/6
  `cat >` overwrite payloads statically known — all interpolants are top-of-file
  literals); (ii) constant-folding `$(cat <<'EOF'…)` (L196 — the value IS the quoted
  body); (iii) static-heredoc-table read-loop enumeration (B6's `done <<EOF` k/v
  tables — loop-unrolling against literal data). Registry additions to propose, not
  engine work to start.
- **tc-M5 (criticality definition).** Scored as "engine mis-modeling breaks the book's
  orchestrated end-to-end execution/ordering", NOT "blocks elision of this row" (else
  every row reads max under head-2's poison economy). A reviewer preferring the other
  definition should re-read A3/A13 first (they move the most).
- **tc-M6 (generality column).** Pure gut-feel by charter; the B8 zeros
  (no braced-operator params at all) are the cell most likely to mislead a later
  reader — real corpora will not be this clean. Marked in-table; repeated here.
