# 19I — The round-19 corpus, extracted as a take-3 acceptance measuring-stick

> What this is. The round-19 test corpus, sorted by the behavior each case pins and tagged with its
> stand-in status — what a fixture fills in versus what is validated for real — so the take-3 rewrite knows
> what to cover and what it must re-ground on real inputs. The cruft (fixtures that exist only to make the
> round-19 stand-in build pass) is called out to strip. This is the grading rubric, not a transcription;
> read the cases themselves for detail. AI-authored, confidence-marked. Continues `19H`; trust the root
> docs and `19H`/`19A §5` over this.

## 0. How to read it

The corpus is two layers. The e2e layer is 43 sh-mechanized cases under `spike/e2e/cases/<name>/`
(`book.sh` + `probe-results.txt` on stdin + `expected.out`, optional `*.oracle.sh`, optional `mocks/`,
optional `XFAIL`), driven by `e2e/run.sh` through three gates: `dash -n` on every rendered probe and apply
(must parse), exec-under-mocks on the apply (cases with a `mocks/` dir run the rendered artifact under inert
shims and assert the exact run-set), and a content golden-diff (secondary). An `XFAIL` file pins the safe
behavior a known defect violates (expected-fail; a surprise pass is a loud XPASS-to-promote). The unit
layer is the Rust tests — `plan/tests/observable_matrix.rs` (the elision state-space), `analysis/tests/cfg.rs`
(CFG structure), `syntax/tests/parse.rs` (the parser), plus the inline `#[cfg(test)]` modules.

Four stand-in axes matter for de-crufting (what is not real in round-19, per `19H`):

- entity — resolved by the find-3 flag-strip. Works for literal operands, so the cases are valid
  behavior-acceptance, but take-3 must re-resolve every entity through the value analysis, not the strip.
- rc / observable — injected (stdin `rc=N` or a fixture). Take-3 sources it from running the read-only
  check (probe-projection), not a fixture.
- convergence — injected on stdin; there is no executor, so the host verdict is hand-fed. Take-3 sources it
  from running the probe.
- render-exec — the one genuinely-real axis: a `mocks/` case actually runs the rendered apply and checks
  which commands ran. Keep this gate as-is; take-3's renders must pass it.

## 1. The acceptance groups — what take-3 must reproduce

Grouped by behavior (exec-validated members noted; the rest are `dash -n` + golden only).

- A. Convergence elision (the core loop): `converged`, `diverged`, `exec-converged`, `exec-diverged`.
  Acceptance: a converged ambient establish is replaced by a no-op; a diverged one runs. Stand-in:
  convergence injected, entity flag-stripped.
- B. The cell-model / poison-wall (the keystone, the genuine round-19 win): `exec-poison-wall-dead`,
  `exec-same-cell-kill`, `exec-distinct-selectors`, `exec-singleton-update`, `kill-then-install`,
  `exec-opaque-neighbour`. Acceptance: per-selector cells (`#installed` ≠ `#fresh`, `#enabled` ≠ `#active`);
  a modeled `update` does not poison `install`; an upstream same-cell `purge` does; an un-oracled neighbour
  poisons. +SURE this group is validated for real at the cell level — only the entity coordinate rides the
  stand-in (the selector/kind discrimination does not).
- C. Observable-liveness (the stdout/stderr/effect gate): `consumed-output`, `exec-consumed-stdout`,
  `exec-devnull-exempt`, `enclosing-group-redir`, `exec-enclosing-pipe-subshell`, `redir-as-effect`.
  Acceptance: a consumed unvouched stdout/stderr blocks elision; `/dev/null` is exempt; an enclosing
  capture (`{ … } > f`, `( … ) | grep`) reaches the inner leaf; an output redirect is itself an effect.
- D. The fold + rc (the value-flow seed — the most stand-in-fed group): `fold-oror-guard-omits` (guard
  `rc=0`), `andor-rc-undeclared-runs` (no rc ⇒ runs, the safe default), `andor-rc-vouch-wrong` (injected
  `rc=9`), `render-multileaf-line-all-elide`, `exec-multileaf-line-mixed`. Acceptance: the fold omits a
  branch only from a known probed rc; an undeclared rc ⇒ run (the `kFAIL-perform` floor). This group is
  exactly what probe-projection must re-ground — see §2.
- E. Guards, `if`/`elif` (the F1 render floor): `guarded`, `guard-status-blocks-elision`. Acceptance: a
  converged `if`-guard must run, not elide (the line-granular render cannot substitute a guard in-situ).
  Note: under the settled model (`19A §5`) the guard is probed and its rc folds the branch — so this
  group's unconditional block is a round-19 render-floor stopgap that take-3's leaf-exact render +
  probe-projection supersedes (the guard becomes a normal probed value).
- F. errexit: `exec-errexit-elide-vouched`. Acceptance: a converged establish under `set -e` stays
  elidable (errexit-status is vouched by the establishes-contract; the engine does not mark it).
  <!-- /* superseded 2026-06-10 (round-20): this group baked the PRE-RULING shape — it directly
  contradicts the human's 19A §3 C-3 / §5 ("errexit is honored, not special-cased-as-vouched")
  and was caught by the round-20 adversarial crosscheck (notes/205 §2). The engine now marks
  errexit + `$?` as status-consumers (notes/206); the case is renamed
  `exec-errexit-top-status-runs` and asserts the converged establish RUNS. Do not re-derive a
  vouch from this entry. */ -->
- G. Leaf-seam / lowering / classify boundary: `exec-subst-body-nonleaf`, `exec-subshell-establish`,
  `exec-detached-fn`, `exec-top-arith-in-arg-ok`, `background-amp-runs`, `exec-literal-unset-pure`,
  `exec-pure-builtin`, `exec-multi-entity`. Acceptance: a `$(…)`-internal command is effect-bearing but not
  a leaf; a subshell body is a leaf; a detached function body ⇒ MustRun; pure builtins do not poison;
  `&`/multi-entity/dynamic ⇒ ⊤ ⇒ run. These pin the CFG-lowering and classify boundaries — mostly
  structural, least stand-in-dependent.
- H. Degrade-to-run / weirdos (the ⊤ ⇒ run safety floor): `garbage-stdin`, `top-eval`, `toprejected`,
  `exec-opaque-var`, `no-oracle`. Acceptance: unmodeled / opaque / garbage input ⇒ run, no crash. The
  apply-direction floor `19H §1.3` leans on — validated here.
- I. Probe render / oracle contract: `probe-operand-quoting` (F-QUOTE — single-quote the bound operand),
  `seam-two-providers-one-kind`, `two-oracles`. Acceptance: the probe renders safely; two providers share
  one named kind. Note: the round-19 probe ships the oracle body with the operand Dorc-extracted and bound
  (Half-B.1) — take-3's command-keyed full-args `check()` (`19H §2`) supersedes the extraction, so these
  re-express against the `check()` rather than the `oracle_*` markers.
- J. Flagship real-book: `headline-pi-webhost` (executed under mocks), `headline-partial`. Acceptance: on a
  scrappy realistic book, elide what is converged and run the rest. +SURE honest residual: on the full
  `pi-webhost` book most still runs (two un-oracled neighbours — `$(hostname)`, `command -v nginx` — each
  poison), which is the real measure of how much oracle coverage a real book needs to elide anything.
- K. Render-fidelity (deferred — the one xfail): `render-case-arm-oneliner-wrong`. Pins the safe behavior
  (a one-line `case`-arm body must render `dash -n`-clean). A take-3 must-fix, on the leaf-exact /
  structural render (`C-5`).

## 2. The cruft to strip (fixtures that only fill in for the stand-in build)

- The rc-injection mechanism. The cli stdin `rc=N` and the fixtures that use it: `andor-rc-vouch-wrong`
  (`rc=9`) and `fold-oror-guard-omits` (`rc=0`). In take-3 the rc comes from running the read-only check,
  not stdin. Keep the behavior (the fold omits from a known rc; an undeclared rc runs — `andor-rc-undeclared-runs`
  is the keeper that already asserts the safe default); strip the injection and re-ground it through the
  probe. `andor-rc-vouch-wrong` is the canonical masking: `useradd`'s converged rc is un-probeable, so the
  fixture hand-feeds `rc=9` — under the `19H §2.3` lean (`fork-mutator-rc` ⇒ ⊤ ⇒ run) this case's premise
  goes away (the mutator just runs), so it is a cut, not a re-ground.
- The Rust masking tests in `observable_matrix.rs`:
  `nonconforming_establish_andor_left_operand_substitutes_exact_rc` (hand-injected `rc=9` plus a `useradd`
  oracle that bakes the username `deploy` in as the verb) and its helper `plan_for_user_oror`. They assert
  behavior that needs the value analysis plus probe-projection; strip the injection and the baked verb, and
  either cut (per the mutator-rc lean) or re-express against the real mechanism.
- The find-3 entity stand-in (pervasive). Every case's entity is flag-stripped; it happens to resolve the
  literal operands, so the cases stay valid behavior-acceptance, but take-3 re-validates every entity
  through the value analysis. The `useradd` no-verb fixtures (baked verb) are the explicit wart — those
  cases only pass because the fixture fakes the verb the engine cannot derive.
- SyncThing artifacts. `fold-oror-guard-omits/book.sync-conflict-*.sh` and `…/probe-results.sync-conflict-*.txt`
  are sync droppings, not corpus — delete (the human owns that deletion; flagged, not done here).

## 3. The measuring-stick — how take-3 is graded against this

- The 43 e2e cases plus `observable_matrix.rs` are the behavior spec. Take-3 must reproduce the green
  behavior with the stand-ins replaced: entity from the value analysis (not the flag-strip), rc/observable
  from probe-projection (not stdin/fixture injection), convergence from running the probe (not a hand-fed
  verdict). A case that only passes because a stand-in happened to feed the right value is not yet a pass
  for take-3.
- Keep the exec-under-mocks gate verbatim — it is the genuinely-real axis (the render actually runs), and
  it is the cheapest guard against the round-16 `ap-2` trap (a non-runnable artifact shipping green). Take-3
  renders must pass it.
- The xfail (`render-case-arm-oneliner-wrong`) converts from a pin to a requirement: take-3's render must
  make it pass.
- Net: this corpus is a sound behavior-acceptance stick. Its value to take-3 is the behaviors plus the
  explicit stand-in tags — they are the list of what take-3 must source for real rather than fake, and the
  cases in group D (and the matching matrix tests) are where "for real" is the whole point.

---

*Companion to `19H` (the value analysis + `check()` shape). Together they are the round-19 hand-forward:
`19H` says what take-3 must build, `19I` says what take-3 is graded against. The spike code itself is the
disposable record (`16P`-style) — reach behavior through these two docs and the `notes/19*` strain-log, not
by reviving the spike.*
