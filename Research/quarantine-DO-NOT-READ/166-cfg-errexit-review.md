# 166 — CFG review: the coarse `set -e` modeling is buggy both ways

> **Status (2026-06-05): spike, CFG-review finding + fix-spec.** A web-grounded
> critic ran `dash` 0.5 + `bash` 5.3 against IEEE 1003.1-2017 §2 and the bash
> `set` manual, and audited `analysis/cfg.rs`'s errexit modeling. Result: the
> coarse "add a failure→exit edge when unsure, more edges = safe" approach
> (cfg.rs ~184-185) is **wrong in both directions** — it both *invents* abort
> edges the shell never takes and *misses* ones it does — and the human's note-164
> flag ("argues for precise `set -e` modeling, not coarse") is vindicated.
> Empirical (`[RAN]`) findings are trustworthy; this note is the fix-spec.

## The dangerous direction — MISSING real abort edges (unsound forward skip)
A missing abort edge ⇒ the CFG thinks a command always ran ⇒ downstream a fact
looks established when it isn't ⇒ a **wrong skip** (`kFAIL-perform` violation).
- **find-5 redir-failure [RAN]:** a failing *redirection* aborts under `set -e`
  regardless of the command word (`: > /nonexistent/file` → abort). cfg.rs gates
  the failure-edge on `CfgNodeKind::Command` only (`materialise_errexit_edges`
  ~691-701); `Redir` nodes (sequenced *before* their command, ~409-413) never get
  one. → also add failure-edges on `Redir` nodes.
- **find-4 subshell-scope-leak [RAN]:** `set -e`/`set +e` inside `( )`/`$( )` must
  NOT change errexit outside it ("applies to each subshell separately"). The
  errexit forward pass (`materialise_errexit_edges`/`errexit_inflow`/
  `errexit_after` ~659-734) treats `ScopeEnter`/`ScopeExit` as pass-through → a
  toggle inside a subshell leaks out (drops a real outer abort = UNDER; or adds a
  spurious one = OVER). → the errexit lattice flow must save/restore at
  `ScopeEnter`/`ScopeExit`.
- **find-6 cmd-subst-failure [RAN+reasoned]:** `x=$(false)` aborts under `set -e`
  (assignment takes the subst's status). The host assignment-`Simple` node's
  edge doesn't reflect the inner subst failure. → messy; the host command's
  fallibility must consider its command-subst children.
- **find-7 function-body [RAN+STRUCT]:** a detached `FuncDef` body computes
  errexit `Off` (its fresh `Entry` is pred-less; ~588-599) so its commands get no
  abort edges. Low severity *today* (calls aren't modeled → body is dead), but
  wrong when call edges land. → note as a known gap; don't seed func-Entry `Off`.

## The other direction — SPURIOUS abort edges (safe forward, UNSOUND backward)
**find-8 [reasoning]:** cfg.rs:184 claims "more edges = sound for both kFAIL
phases." True forward; **false backward.** A spurious `cmd→exit` edge adds `cmd`
to `pred(exit)`; the backward apply-minimization slice (the module advertises it,
~8-9) then sees the post-`cmd` continuation as conditionally-bypassed, and can
conclude a downstream mutation is *skippable* when it's always reached — under
`Phase::Apply` (`kFAIL-perform`) that's the **unsafe** direction. So the spurious
edges below are not harmless padding; fix them (they're mismodelings, not
deliberate conservatism):
- **find-1 negation [RAN]:** `! cmd` never aborts under `set -e` (POSIX `!`
  exemption), even `! true`. `lower_pipeline` (~441-455) ignores `negated`. → a
  negated pipeline is never fallible-for-errexit. (This is Dorc's own guard idiom.)
- **find-2 non-tail-condition [RAN]:** the WHOLE `if`/`while`/`until` test is
  exempt, not just its last command. `mark_condition_context` (~803-807) clears
  only the region-exit node. → clear fallibility across the whole condition region.
- **find-3 compound-condition [RAN]:** when the condition / `&&`/`||`-left is itself
  a chain or pipeline, its exit is a `Merge`, so `mark_condition_context` clears
  nothing and inner non-final operands keep edges. `lower_and_or` (~463-481) marks
  only its immediate left. → propagate condition-context through the whole region.

## Confirmed CORRECT (do not re-touch)
pipeline last-stage-only governs errexit (~447-452); `cmd || true` swallow (`||`-
left is condition-context); top-level `&&`/`||` final-operand edge kept; `exit`/
`return` severs fall-through; brace `{ }` does NOT scope (matches real leak);
`for`/`while`/`until`→`Top`; `succ`/`pred` consistency (`add_edge` ~294-302).

## Conclusion / fix-spec
Make errexit **precise**, eliminating both spurious and missing edges (fixes
find-1..6; note find-7). Then "more edges = safe" is moot (edges are precise) and
both the forward skip-consumer and the backward apply-slice are sound. Each fix
gets a regression test = the dash-verified script from the finding. This is the
direct payoff of the human's note-164 scrutiny flag.
