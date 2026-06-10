# 20B — Pair reconciliation: propagation-correctness + acceptance-apparatus (4 agents)

> Round-20. Both planned crosscheck targets ran as full neutral+adversarial pairs
> (Fable-class, clean contexts). Convergence between passes = the trusted signal;
> everything adopted was orchestrator-traced first. Fixes referenced here landed in
> 6fb2345 + 184632a + the commit carrying this note.

## §1 Propagation pair (the no-floor obligation) — verdict: spine sound, precision paths bled

- CONVERGENT (independently found by both): **prefix-env argv visibility** — POSIX §2.9.1
  expands argv before prefix assignments; the engine had it backwards, driven end-to-end to a
  wrong elision. Fixed (argv resolves against incoming env only; pinning test inverted).
- Adversarial-only, traced, fixed: **globby/`##` prefix-strips** evaluated literally where
  dash globs — plus the latent sibling my fix exposed (unmodeled `${…}`→Literal would
  self-compare in test-position) — all now `Word::Unmodeled` ⇒ Top in every position.
- Neutral-only, traced, fixed: **the lvalue-builtin family** (`read`/`export NAME=`/
  `readonly NAME=`/`getopts`) — variable mutation unmodeled while effect-layer-Pure, so stale
  concretes survived both planes (F-read demonstrated to a wrong elision of a
  runtime-determined install). Fixed: clobber-to-⊤ family handling (never value-modeling),
  with tests; bare `export NAME` (no `=`) correctly preserves the binding.
- Held under attack (both passes): subshell-vs-brace-group leak semantics, branch joins +
  entry-⊤-seed, ⊤-region havoc completeness, concatenation, empty-vs-unset in tests, the
  conservative ⊤ family. Pattern confirmed: every break was in precision-ADDING paths;
  the degrade spine held everywhere.

## §2 Apparatus pair — verdict: goldens honest, gates process-blind, flagship lane vacuous

- CONVERGENT, demonstrated by both: **the harness never observed the engine process** —
  exit status piped away, stderr discarded, empty artifacts `dash -n`-clean; under BLESS a
  dead engine would have blessed ~43 empty goldens with reassuring messages. FIXED this
  round: crash/empty hard-fail guard before the xfail lens and before bless (verified both
  directions: 44/44 green normally; crash-stub now 44/44 loud FAIL), plus
  missing-`expected.ran` ⇒ loud fail for mocks cases. Residual (task-D): a
  stderr-severity floor (error-class diagnostics should fail a case unless declared) — can't
  be a blanket stderr-empty assert since ⊤-reject warnings are legitimate.
- CONVERGENT: **the headline lane is vacuous** — 23/44 cases stdin-dead; the two headline
  books are byte-identical triplets asserting render+exec fidelity only; 19I group J
  described a guarded realistic e2e book that never existed in the corpus (the honest
  residual lives at the UNIT layer — `fixture_install_on_realistic_book_still_runs_…` — where
  it IS pinned). Annotated: 19I group J, STALENESS-AUDIT's "elides 6 mutations" line.
  task-D restores the value-story measurably: a guarded realistic e2e book + partial
  convergence, asserted via Query-guard folds.
- Adversarial-only, traced, accepted-as-true: **9 empty-probe cases carry unvalidated
  checks** (garbage check ⇒ still green — their green never depended on resolution);
  group-B's poison-direction members are indistinguishable from blanket over-conservatism.
  The principled fix is cm-2 (20A): the argv-echo/check-eval differential validates checks by
  EXECUTION rather than by more goldens — task-D's gate work, not more case-authoring.
- Adversarial-only, accepted: **the exec lane's blind spots** — all-exit-0 mocks make
  rc-class regressions invisible (the "secondary" text golden is what actually catches them
  — re-weight the gates' billing accordingly); the sorted run-set cannot see ORDER
  regressions, though "the book's order is sacred" is a welded ruling (task-D: assert the
  unsorted log too — sequential sh is deterministic, sorting discards the assertion).
- cov-5 (adversarial): the declared-rc lane is mechanism-broader than its one sanctioned
  fixture (any establish-site rc would fold; only fixture discipline prevents it), and the
  priced recovery path — set -e + known-rc ⇒ still elides — is composed-but-never-tested.
  task-D: pin that composition with a genuinely probe-sourced Query rc under `set -e`.
- lie-2: the standing render-xfail has NO goldens (its pin is only "dash -n fails") — its
  hand-authored safe-behavior goldens land with task-D's render fix, which converts it to
  must-pass anyway.
- Record nits: 208's case-count + rename misdescription annotated in place; the stale
  find-3-era oracle header and the one stale matrix comment fixed in code. ok-1..ok-6: all
  six re-blessed goldens hand-rederived clean; renames lost no assertions; mock discipline
  holds; the C-3 citation chain verified genuine.

## §3 task-D inheritance (consolidated, final — supersedes scattered earlier lists)

rule-probe-exec-gate + the results-lane producer (the wire bridge — find-2, load-bearing);
rule-anno-render; rule-query-validity (with the st-3 query-doesn't-invalidate-query
refinement); Query effect-class + fold-oror re-grounding + stdin `rc=N` lane removal
(stage-2); vouch-closure check; render xfail→pass + leaf-exact case-arm fix + xfail goldens;
Exit(n) coverage via a nonzero Query-guard rc; set-e+probe-sourced-rc composition case;
guarded realistic e2e book (the group-J restoration); floor-boundary strawmen (loops/
sourcing/partial-⊤); stderr-severity floor; unsorted run-set assertion; CORPUS_CHECK_SRC
dedup; probe-source reconciliation per st-2 (probes stay declarations; placeholder check
probe-bodies must not ship — pkgindex's tautological `test -n fresh` is the named hazard);
cm-2 argv-echo differential gate (the 20A countermeasure, which subsumes the
check-validation gap).

## §4 Dispatch ledger (meta-goal)

Four Fable agents this wave: 167k–283k tokens each (~900k total). Yield: two priority-1
wrong-elision classes fixed pre-task-D, one demonstrated apparatus hole closed, the
flagship-vacuity discovery, and a fully-written task-D gate agenda. Highest value-per-token
spend of the round, again — consistent with the durables pair. Both pairs produced their
sharpest results where aimed AT THE APPARATUS or AT fresh precision code; neither found
anything in the degrade spine. The 20A reframe (move soundness to harness-time) is the
structural answer to needing these passes less.
