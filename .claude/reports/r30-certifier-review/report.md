## Findings

1. `finding-missing-states-certify-clean` — Severity: Critical

+SURE: [`certify_solution`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/certify.rs:353>) silently `continue`s when a node’s state, an edge source, or an edge destination is absent. It then returns `Consistent` whenever no examined inequality failed at line 398.

Concrete failure: for any nonempty graph, pass `Solution { states: vec![], ... }`. The certifier executes zero checks and returns `Consistent`. A solver regression that truncates or drops its state vector—the exact “state-management” defect class this instrument claims to detect—therefore bypasses every floor and permits downstream licenses.

+SURE: a short `init` slice is similarly normalized to bottom at line 356, silently removing a boundary obligation. The production wrapper currently constructs an exact-length `init`, but the malformed-`states` fail-open directly undermines the checker’s stated role.

2. `finding-raw-worklist-bypasses-fence` — Severity: High

+SURE: raw `solve` is crate-private, but the actual worklist entry point, observer trait, and no-op observer are also crate-visible at [`solve.rs:83`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/solve.rs:83>) and [`solve.rs:99`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/solve.rs:99>).

Concrete failure: any production analysis module can call:

```rust
run(graph, direction, transfer, &mut Unobserved)
```

That returns an uncertified `Solution`. The lexical fence at [`certify.rs:1219`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/certify.rs:1219>) scans only calls named `solve`; it does not scan `run`. It also permits arbitrary raw-`solve` calls anywhere in `certify.rs`, rather than only the wrapper seat.

+SURE: no current production caller uses either bypass. However, the claimed compile/fence invariant is not enforced: a future bypass compiles and the fence remains green.

3. `finding-effect-failure-report-arrives-late` — Severity: High

+SURE: effect certification happens during initial classification at [`main.rs:982`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/cli/src/main.rs:982>), but its diagnostics are not reported until [`main.rs:1421`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/cli/src/main.rs:1421>).

Concrete failures:

- +SURE: `probe` mode returns at line 1234 before reaching the reporting call. A reaching-defs or self-reach inconsistency is floored but never disclosed.
- +SURE: host-backed operation calls `ship_probe` at line 1262 before reporting the inconsistency. A slow or failed transport therefore delays or entirely loses an engine-defect report that was known before the network call.

The license floor remains conservative, but this violates the specified pre-network, front-and-center failure posture.

4. `finding-self-reach-account-is-fabricated` — Severity: Medium

+SURE: [`self_reach_holds`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/effect.rs:1169>) reduces the complete `SolveConsistency` to a boolean. `self_reach_pass` then counts inconsistent solves, not failed checks. At [`effect.rs:1720`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/effect.rs:1720>), that solve count is reported as the failing-check count, while the narrative hard-codes `converged: true` and `rounds: 0`.

Concrete failure: one self-reach solve fails 17 inequalities after hitting its cap. The emitted account says one failure, zero retained indices, `converged=true`, and `rounds=0`. The actual indices, advisory values, by-value failures, and replay have already been discarded, so no pull surface can recover them.

+SURE: this does not grant an elision—the boolean floor is safe—but the self-report is materially false.

5. `finding-debug-build-skips-demotion` — Severity: Medium

+SURE: the main reaching-defs path executes a `debug_assert!` on consistency at [`effect.rs:1685`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/effect.rs:1685>) before applying `trust_reach=false` or minting the diagnostic.

Concrete failure: in a debug CLI or integration test, a genuine inconsistent answer panics immediately. It never reaches the whole-window `MustRun` product or reporting path. Release builds demote correctly; this is a debug-build violation of the “on any inconsistency, demote and report” contract rather than an under-execution route.

## Independent claim verification

1. Raw solve unreachable from production paths: Refuted as an enforcement claim.

+SURE: in the current snapshot, every checked-in production call uses `solve_certified`; raw `solve` is called only by the wrapper at [`certify.rs:432`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/certify.rs:432>).  
+SURE: the fence does not close every route because crate-visible `run` bypasses it, and the whole `certify.rs` file is exempted.

2. Funcenv floor cannot feed `never_live` subtraction or edge folding: Verified.

+SURE: [`fold_to_environment`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/funcenv.rs:604>) checks consistency before calling `dead`; a failing round therefore cannot contribute new folded edges. Any edges from prior certified rounds are discarded when [`funcenv_floor`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/funcenv.rs:643>) constructs an empty `folded_edges` set. Finally, [`never_live`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/analysis/src/funcenv.rs:957>) returns the empty set before reading states whenever the environment is untrusted. The CLI subtraction at [`main.rs:895`](<C:/Users/ec/Sync/Code/Dorc/.claude/worktrees/agent-a1dc580bcea6f51bb/spike/crates/cli/src/main.rs:895>) therefore receives no grant-shifting withdrawal.

+SURE: I could not run the prescribed suite because `mise tasks` crashed while accessing sandbox-denied configuration, and escalation was rejected. The findings above are from direct source and call-path inspection.