# 16H — the observable/replace in-spike build landed (round-16 summary)

> **Status (2026-06-05): spike, round-16 implementation summary.** The bounded
> in-spike build scoped in 16G is done; the adversarial coverage-audit's gaps are
> closed (bar one deferred ⊤-containment case). Append-only (round 16: …16G → 16H).
> HEAD `eb55c95`. Confidence-marked.

## 0. What landed
- **`76e7806` builtin-pure (fix B):** `command_effect` treats a small blessed set of
  target-state-pure builtins (`set`, `cd`, `:`, `true`, `false`, `[`/`test`, `echo`,
  `printf`, `export`, `unset`, `shift`, `read`, `local`, `readonly`) as `Pure`, not
  `Opaque`, so they don't poison reaching-defs ambient-ness (fs-4 / the over-refuse
  direction). Sound: they touch shell-env/stdout, never an oracle-modeled fact.
  Un-ignored `spec_converged_set_e_does_not_poison_replacement`.
- **`eb55c95` observable-liveness gate (fix A) + skip→replace rename (16F):**
  - The gate is a new obligation in `prove_replaceable`: a leaf MustRun if an
    **unvouched output observable (stdout/stderr) is consumed**, by the conservative
    *structural* surrogate (16F §5; no value-plane, 16C brk-1) — a **non-last
    pipeline stage**, or a **Write/Append redirect of fd 1/2 to a non-`/dev/null`
    sink**. Status consumers (`&&`/`||`/`$?`/`if`) do NOT trigger it (establish-
    vouched). Computed plan-locally from the AST (`observable_use` /
    `non_last_pipeline_stages`) — no CFG change.
  - Strong-typed per the steer: `ObservableUse` (the consumed unvouched observables)
    is a required argument of `prove_replaceable`, so a `ReplaceLicense` is
    **unmintable** when an unvouched observable is consumed (prevent), and the
    `Observable` enum names the vouching model in the type (16D spotlight, without
    `Grounded<T>`).
  - Rename: `SkipLicense`→`ReplaceLicense`, `prove_skippable`→`prove_replaceable`,
    `Disposition::Skip`→`Replace`, `Resolved::Skippable`→`Replaceable`, render
    `# skip[`→`# replace[`; hostsim's DST test updated.

## 1. The 16E↔16F debate — settled for 16F, by the tests
The matrix adjudicates: `pins_converged_status_via_andand/captured/oror` stay green
(a consumed *status* is replaceable — establish-vouched), while every `spec_*stdout*`
and the new `spec_*stderr*` require run. So **one observable-liveness obligation,
status discharged by the establish-contract; no analyzer status/rc-value reasoning**
(16F §4). The converged-non-zero `mkdir`-style cell stays the documented
un-modellable cell (oracle-contract, 16D blast-radius-bounded), not an analyzer gate.

## 2. Coverage delta from the audit (note 16G)
Matrix now: 16 active + 1 ignored. New green specs: `stderr_to_file`,
`stderr_merged_piped` (2>&1), `redirect_is_an_effect` (the redirect itself is a
dropped FS mutation — `haz-redir-as-mutation`). New green pins (the scalpel guards):
`devnull_discard_replaced`, `status_via_oror_replaced`. Plus the 3 original stdout
specs un-ignored. A new `prove_replaceable` unit test pins the gate directly.

## 3. Deferred (documented), out-of-scope (unchanged)
- **DEFERRED — `spec_topcontext_background_leaf_must_run` (`#[ignore]`d):** hole-5,
  the ⊤-containment breach — `install &` (the `&` ⊤-rejects loudly) is still
  replaced because `build_plan` never consults diagnostics. An `inv-top-reject`
  breach at the plan layer; benign for a converged no-op, latently unsound. It is a
  distinct ⊤-handling/parser fix (a leaf whose own statement contains a ⊤ ⇒ MustRun),
  not the observable gate — a clean next item.
- **OUT OF SCOPE (unchanged):** the oracle gather/compute **bridge** that would
  *discharge* a consumed stdout; `Grounded<T>` (16D); cross-host; any rc/stdout
  *value* analysis (16F §4 smell); **fd-dup resolution** (`2>&1`, `>&3` beyond the
  pipeline/redirect floor — deliberately unresolved so `> /dev/null 2>&1` stays
  replaceable; a precision refinement). Corpus items parked.

## 4. State (network-free kernel; whole workspace green + clippy-clean)
`core` · `syntax` · `analysis(lattice,solve,cfg,effect)` · `oracle` · `plan` ·
`hostsim` · `cli`. Tests: analysis 18 · cfg 26 · core 5 · hostsim 6 · oracle 8 ·
plan 10 · matrix 16 (+1 deferred) · syntax 2+16.

## 5. Method (held — note 16G §6)
The gaps came from the `adversarial-crosscheck` skill (clean-context pair, un-seeded
by 16C–16F); every gap + the boundary pins were **verified by tracing the built
`dorc` binary**, not relayed. The exclusion-check (reverse / phase / user /
reliability) governed what was deferred vs fixed.

**NOTES INDEX:** …16E state/CFG · 16F observable/replace model · 16G coverage audit +
build scope · 16H (this — the gate landed).
