# 24I — e2e de-graduation audit (read-only; the migration's execution spec)

AI-authored (Opus audit agent, conductor-condensed), 2026-07-05, round 24. Human-commissioned:
the e2e suite is an expensive tier for small precise logic-tests; classify all 152 cases and
spec the move. Process-evidence throughout — every REDUNDANT names its covering test and the
migrator (or conductor) eyeballs the twin before any deletion. ~SUSPECT marks preserved.

## Orienting facts (measured/verified)
- 130 cases execute under mocks; 22 are analysis-only (`dash -n` + golden + gate-3 only).
  Per-case cost (unloaded): mocks ≈0.9s, analysis-only ≈0.2s; msys PROCESS-SPAWN count is the
  tax (engine 27ms, dash -n 21ms). One XFAIL at HEAD (pipe-guard); guard23 "XFAIL until tier
  lands" comments are STALE (the tier landed — clean them up in passing).
- **The integration tier already exists in embryo:** `plan/tests/observable_matrix.rs` drives
  parse→plan→`render_apply` in-process with injected verdicts (`holds`) + vouches
  (`build_vouches`) + footprints (the touches lift), asserting render STRINGS and structural
  `Disposition`/licenses. The plan-crate units (48) already twin many cases 1:1 by name.
- The sweep (flavour C) generates all NINE `TopologyClass`es × honesty at 3000 seeds — the
  survival-tier e2e variations have generated analogues (assertion-depth per topology is the
  one thing NOT verified — batch-5's per-row check).

## Counts + the win
STAY ≈48 · DEGRADE ≈50 · REDUNDANT ≈54 (movable ≈100–104; ~55–70 land cleanly first-pass —
the rest need their twin authored). Wall-clock: moving ~55 mocks cases ≈ −40% of the ~2min
baseline; the migrated coverage itself runs <1s in-memory. The ratio is the deliverable.

## THE design-flag (carry loudly into the migration brief)
**The in-memory twins ARE the ap-2 trap at scale**: `observable_matrix.rs` render-asserts via
`.contains()` with NO `dash -n` — the text-diff blindness that shipped non-runnable sh green
twice. Latent only while e2e twins still run dash. The new tier MUST add the one-shot
`dash -n` per rendered artifact (21ms, no mock machinery) — the cheap 90% of ap-2.

## What STAYS e2e (≈48; the thin floor — real-shell-ness IS the property)
The guard23 sh-semantics set (ternary-flagship, var-namespace-isolated, nounset-book-survives,
fallthrough-drift/canttell, mutator-fails-continues, reingest-collision, already-hand-guarded,
heredoc-refuses, multioperand-atomic, refusepath-rc0) · errexit/subshell/pipe runtime cases
(exec-errexit-top-status, strawman24-errexit-defeats, exec-subshell-establish,
exec-enclosing-pipe-subshell, exec-consumed-stdout, exec-subst-body-nonleaf) · gate-5
value-flow↔dash argv-agreement set (split-single/multi, glob-for-word, probe-operand-quoting,
exec-resolved-var, exec-multi-entity, exec-top-arith, exec-opaque-var, top-eval,
loop-nested-converges) · rc-faithfulness (door1-and-form) · render-runnability shapes
(render21-heredoc-refusal, render21-adjacent-multiline, render21-multiline-leaf,
omitsafe21-heredoc-guard-keeps/flipped, omitsafe21-heredoc-and-flipped) · gate-1 real
probe-parity (exec-shimmed-query-fold) · the yardstick handful (headline-guarded-realistic,
headline-pi-webhost, headline-partial, strawman24-mixed-real, -all-converged-clean,
-floor-no-oracle, -partial-oracle, -adequacy-seed) · flavour-C tie-downs (tiedown-survive/
killwall-statebearing) · strawman24-derived-survive (real probe-lane shipping) · the three
pipe-guard cases (incl. the XFAIL promotion pin).

## Migration ORDER (each batch independently landable; migrator eyeballs every twin)
1. **18 analysis-only REDUNDANTs, twins confirmed by name:** converged, diverged,
   consumed-output, enclosing-group-redir, redir-as-effect, background-amp-runs,
   guard-status-blocks-elision, andor-rc-undeclared-runs, y1-devnull-exempt,
   y1-var-resolved-target-invalidates-query, y1-redirect-write-invalidates-query,
   y1-top-target-poisons, no-oracle, toprejected, while-read-file-rejects,
   inline21-recursion-rejects, inline21-redirect-body-refuses, inline21-overbudget-degrades.
   Pure deletions (an::/plan::/om:: twins named in the full audit).
2. **door-3/query-guard REDUNDANTs:** door3-or-true-elides/-diverged-runs,
   door3-and-true-blocks, exec-dollarq-blocks-elision, exec-query-guard-composition,
   exec-query-after-mutator-runs, door1-guard-below-mutators-invalid, fold-oror-guard-omits,
   exec-converged, exec-diverged (om::/plan:: twins). KEEP exec-shimmed-query-fold.
3. **DEGRADE render/loop/inline logic → a new `plan/tests/render_corpus.rs`** (+ analysis/tests
   for ⊤-reject diags; a new cli/tests for why-lens/plan-summary/multi-o/garbage-stdin):
   door1-cascade-*, door1-door3-*, door3-or-handler-blocks, kill-then-install +
   exec-same-cell-kill (byte-identical twin books — collapse), exec-devnull-exempt,
   exec-pure-builtin, exec-literal-unset-pure, exec-opaque-neighbour(~SUSPECT twin-depth),
   exec-detached-fn, exec-multileaf-line-mixed, exec-distinct-selectors(~SUSPECT),
   exec-enabled-not-active-host(~SUSPECT), exec-singleton-update(~SUSPECT), loop-* (6),
   render-case-arm-oneliner, render-multileaf-line-all-elide, *-shared-line (3),
   render21-if-guard-* (2), render21-while-guard-floored, inline21-wrapper-* (2),
   inline21-errexit-call-composes, inline21-in-loop-call-floored, seam-two-providers-one-kind,
   two-oracles, garbage-stdin, guarded, guard23-why-attribution. String-assert render shapes;
   structural asserts for dispositions; ONE dash -n per rendered artifact (the flag above).
4. **guard23 no-mint floors → GuardLicense-absence structural asserts** (tighter than run-set
   proxies): guard23-background-not-guarded, -canttell-plan-runs, -no-vouch-runs,
   -vouch-gates-elision, -cross-oracle-vouch-scoped, -rundelta-never-guards,
   -inverted-vouch-never-backwards, -top-argv-runs, -cmdsub-position-runs,
   -consumed-stdout-runs, -redirect-line-runs, -explicit-rc-consumers-run, -inloop-unchanged.
   Named plan:: twins exist for several (guard_mints_only_on…, no_license_for_ambient…,
   diverged_sense_glue…, substitution_internal…, no_license_when_unvouched…).
5. **LAST + most care — survival tier vs sweep (all ~SUSPECT until per-topology assertion-depth
   verified):** strawman24-survive-simple/-unflagged, -modeled-wall, -opaque-wall,
   -survive-killwall, -survive-multiwall, -nonsurvive-bare, -nonsurvive-hit,
   -crosskind-residue(diag may be the real point — maybe DEGRADE-to-diag), -incoherent-refused
   (ditto), -alias-provides, -alias-symlink, -reach-crossauthor, -reach-static-service.
   Per row: confirm the `TopologyClass` scenario asserts the same end-state; the real-emit
   half stays anchored by the STAY yardstick+tie-downs+derived-survive.

## Also surfaced
- In-corpus duplication independent of the move: byte-identical twin books
  (converged↔exec-converged, diverged↔exec-diverged, kill-then-install↔exec-same-cell-kill,
  consumed-output↔exec-consumed-stdout) — collapse free.
- run.sh's three self-test batteries are fixed per-invocation cost the in-memory tier never
  pays; fine, just accounting.
- strawman24-adequacy-seed's POINT is documenting what the differential canNOT see
  (converged≠no-op) — never migrate it; it is the honesty pin.
