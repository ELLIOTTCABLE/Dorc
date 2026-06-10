# 206 — task-E landed: C-3 honored; the headline-cost datum; flags adjudicated

> Round-20, follows 205 §2's rule-errexit-honored. Engine commit `72c445d`. AI-authored.

## §1 What landed

Consumption-marking now covers the two consumers 19A C-3 named: an errexit-region command's rc
(marked by riding `materialise_errexit_edges`' precise failure-edge set — so the mark inherits
the negated-pipeline/condition-region/`|| true` pruning for free), and a `$?`-reader's
CFG-predecessor command(s) (a conservative pred-walk; marking-more is the safe direction). Both
mark the value-relaxing channel, so probe-sourced rcs (future Query folds) still substitute
exactly; ⊤ rcs block. No plan-side change was needed — `inv-superposition` held, the existing
`AndOrStatus`+⊤ refusal is the single collapse point (the seam task-W must preserve: a
probe-sourced rc must arrive as `Predicted::Value`).

## §2 The headline-cost datum (the round's loudest strain; priced in 205 §2, now measured)

Both headline books open with `set -e`, and under C-3 × fork-mutator-rc every mutator's rc
there is consumed-with-⊤:
- `headline-pi-webhost` (all-converged): 6 mutator-elisions → **0**. Died: `apt-get update`,
  `apt-get install -y nginx`, `ufw allow 80/tcp`, `ufw allow 443/tcp`, `systemctl enable
  nginx`, `systemctl start nginx`.
- `headline-partial` (mixed): 3 → **0** (`apt-get install`, `systemctl enable`, `systemctl
  start`).
- The two headline run-sets are now IDENTICAL — under `set -e`, convergence no longer
  distinguishes anything for mutators. The keystone's "elides 6 mutations" demonstration is
  gone until Query-guard folds carry the value instead; this is the two human rulings composed,
  not an engine regression, and the recovery doors are the human's (a conformance declaration
  as a strictly-block-lifting May-grade signal, or richer probe evidence someday).

## §3 Flags from the build, adjudicated

- **lib.rs isolator masking residue** (task-E flag-2): `plan/src/lib.rs`'s `#[cfg(test)]`
  `plan_for` still injects declared `Rc(0)`; two `set -e` isolator tests now pass *via the
  relaxation* (true behavior for a known rc, but the injection is the 19I §2 masking pattern).
  Adjudication: task-W re-pins them on `verdict_only` (⇒ Run) — the unit layer should model
  fork-mutator-rc faithfully everywhere, not just in the matrix.
- **The channel rename is now evidence-backed** (task-E flag-4, resolving 19G's deferred
  `ch-wrong` bake-and-see): with four sources feeding the relaxable channel and one feeding the
  render-floor channel, the axis is provably *render-expressibility*, not construct-identity —
  `AndOrStatus` is now a misleading name. The rename (e.g. `StatusRelaxable` vs
  `StatusRenderFloor`) lands with the one-Observable completion pass; the two-channel
  *distinction* stays load-bearing until the leaf-exact render retires the floor.
- Brief-compliance nit for the dispatch ledger: the agent used `git mv` (index mutation)
  despite the no-git-staging instruction — harmless here (no commit, single agent), but the
  brief language should say "no index operations; rename via fs + let the orchestrator stage".

## §4 Suite state

136→145 cargo tests (55 analysis-lib incl. value, 41 cfg integration, matrix re-pinned), e2e
43/43 (42 + new `exec-dollarq-blocks-elision`; `exec-errexit-elide-vouched` renamed to
`exec-errexit-top-status-runs` so the name stops asserting the dead vouch), all four lint
gates green.
