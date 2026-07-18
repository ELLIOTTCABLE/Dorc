# 16L — test-suite audit (lean + enrich): per-file → global → adversarial

> **Status (2026-06-06): spike, round-16 — test-suite curation.** A 3-phase audit
> of the whole suite, goal: lean (rip redundant/trivial) AND enrich (add only crucial
> gaps), to a MEDIUM-MINIMAL, low-noise, high-value level. Append-only (round 16: …16K
> → 16L). HEAD `502acf6`. Confidence-marked.

## 0. Method (the human's 3-phase design)
1. **Phase A — per-file (me, as units):** read each test file alone, rip the clear
   trivia/redundancy, generalize where it covers more. (`a3d00e6`)
2. **Phase B — global cross-cutting (one normal subagent):** load ALL tests at once,
   find cross-FILE / cross-LAYER redundancy + gaps invisible per-file. Reported; I
   adjudicated + applied. (`0cd53bc`)
3. **Phase C — adversarial (`adversarial-crosscheck` skill):** clean-context pair
   (neutralised + disowned-inverted), un-seeded by the 16x notes, calibration bracketed
   as a hard constraint. (`502acf6`)
Calibration given to every subagent: low-noise > 100% coverage; crucial state-spaces;
high value not high quantity; each test earns a reasoned argument. No tests for unbuilt
features (apply, multi-host, gather/compute bridge, fd-dup, forward-must boundary-seed).

## 1. What changed (net −14 tests, +3 enrichments; suite 119→105 active +1 ignored)
- **Phase A (−8, +2):** rip `core::span_to` (trivial helper); cfg `fixture_entry_reaches_set_e`
  (implied by builds_consistently), `fixture_heredoc…` (under-delivered), `swallow…`
  (folded into `find3` + a `||` case); matrix `…piped_to_tee` (≡ grep), `…redirected_then_read`
  (≡ redirect_is_an_effect), `…status_captured` ($? ≡ &&/||); both syntax/src lib smoke-tests
  (≡ tests/parse.rs). Fixed the stale matrix module-doc (it still called the specs
  `#[ignore]`d / "currently FAIL" — untrue since the gate landed).
- **Phase B (−6, +1):** the matrix end-to-end specs duplicated the (this-session-added)
  cfg `consumed_*` fact-layer + the plan-unit collapse-layer. Removed the pure
  fact-duplicates (`stderr_to_file`, `stderr_merged_piped`, `enclosing_subshell_pipe`,
  `enclosing_subshell_devnull`) and the cross-file dupes (`pins_subst_internal`,
  `pins_poisoned_install_runs` — kept the classify-layer + fixture e2e). FIRST enriched
  plan-unit `no_license_when_unvouched_output_consumed` to cover the **Stderr** branch
  (was Stdout-only) so dropping the matrix stderr cell lost no coverage.
- **Phase C (+2, −2):** ADD `effect::pure_builtin_upstream_does_not_poison_ambientness`
  and `parse::reject_over_deep_nesting_is_loud`; REMOVE matrix `pins_unconverged_install_runs`
  + `pins_converged_lone_install_replaced`.

## 2. The adversarial pass — what held, what didn't (the skill's value)
- **CONVERGENT (both passes, +SURE) ⇒ trusted + applied:** the `is_target_state_pure_builtin`
  allowlist (14 entries) was pinned only for `set` (one e2e); the other 13 + the
  Ambient-vs-Written line were unguarded. A mis-edit dropping `:`/`echo` is a silent
  wrong-skip. Added a classify-layer unit test. (The single highest-value finding.)
- **ADVERSARIAL-ONLY ADD, traced ⇒ HELD:** the parser depth cap (MAX_DEPTH=256) emits
  `Unmodeled("nesting too deep")` + an Error, but it was the ONE ⊤-trigger whose reason
  no test asserted (totality only checks no-panic). Traced to `parser.rs:288`; added a
  loud-reject assertion. inv-top-reject completeness.
- **NEUTRAL-ONLY REMOVE, traced ⇒ HELD:** the matrix verdict-axis baselines
  (diverged⇒run, converged⇒replace for a lone install) are subsumed by the plan-unit
  e2e (`diverged_install_runs`, `converged_ambient_install_is_replaced_rest_runs`). The
  matrix isolates the OBSERVABLE dimension; the baselines belong in plan-unit.
- **ADVERSARIAL-ONLY REMOVEs, traced ⇒ did NOT hold (dropped, the manufactured ones):**
  `carrier_reports` (uniquely pins error⇒has_errors; the threads-test is warning-only);
  `unsupported_in_sequence` (uniquely pins CFG-layer neighbour-survival; parser test is
  AST-level, loop test has no neighbours); `unknown_folds_in_both_phases` (the ONLY test
  of **Apply-phase** Bias — the catastrophic wrong-skip direction; the adversarial itself
  hedged it); the lattice `May`-half trim (cohesive dual-lattice law-test, ~3-line saving).
  Exactly the "hostile pass over-reaches" the skill warns of — caught by tracing each.
- **DECLINED (neutral -GUESS, not cheap):** a real-CFG non-convergence test for the
  `classify` `trust_reach` guard — `Reach` is finite-height so it can't be forced
  non-convergent without a synthetic seam; the `debug_assert` + the solver-level cap test
  are the floor.

## 3. State (network-free kernel; whole workspace green + clippy-clean)
Tests: core 4 · analysis 19 (lattice 5 · solve 8 · effect 6) · cfg 31 · oracle 8 ·
plan-lib 11 · matrix 9 (+1 ignored) · hostsim 6 · parse 17 · syntax-lib 0 = **105 active
+ 1 ignored**. The matrix is now purely the observable-dimension cells + the status A/B
vouching + the HOLE#1 spec; the fact layer lives in cfg `consumed_*`, the verdict
baseline in plan-unit.

**NOTES INDEX:** …16J superposition spec · 16K superposition rewrite landed · 16L (this
— 3-phase test-suite audit: lean + enrich).
