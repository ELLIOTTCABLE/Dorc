# 27O — lane-fallback-carry landing (pure-predicate carry, `27C` §4(a))

AI-authored (Opus builder, r27 lane-fallback-carry session, 2026-07-17). Records what landed for
the `27C` §4(a) **pure-predicate carry** — the engine-proved read-set-closure pass, "the spike's
obligation to discharge and prove in practice" (`27C` §10). Authority: root docs + `spike/CLAUDE.md`
rulings + `271`/`272`/`277`/`27C`/`27D`/`27N`/`27Xf` outrank this. Companions: `plans/27C` §0.2/§4(a)/§9
(THE spec) · `27D` (the block-context ledger; Tier-1 adjudication) · `27N` (the lane-integration
surfaces this builds on) · `272`/`277` §4e (the `invariant:<axis>` line surface (A) consumes).

## Branch / fold state (READ FIRST — the conductor must reconcile)

- Branch `ai/r27-fallback-carry`, based on `ai/spike3-r27` @ **`1aecaa3`** (verified at step-zero; the
  worktree's stale base `4b9e8aa` was on a divergent r26 doc lineage — switched off it per step-zero).
- Tip = the commit carrying this note. Commits (oldest→newest): `a80d8f4` (carry core module + 18
  unit tests) · `fa6487b` (WrappedProbe::Carry + degrade-path wiring) · `0bd07b6` (e2e fixtures) ·
  this note.
- NOT rebased (hook-reserved to the human). HOLD at tip; the conductor+human fold. Behind-count vs the
  lineage tip (`ai/spike3-r27` @ `1aecaa3`) at hand-off: **0** (verify before fold).

## What landed

A degraded wrapped site whose crossed boundary is a **substrate axis** (fs-view / netns) now CARRIES
its ambient measurement across the boundary, UNFLAGGED, iff BOTH (A) authored axis-invariance AND
(B) engine-proved read-set closure hold. The carry is a DISTINCT, explicitly-licensed path: the cli
keys a carried fact `Context::HostDefault` (measure ambient), so `core::coord::compare` is UNTOUCHED
— cross-context compare stays `Unknown` for every OTHER consumer (`pin-no-outcome-as-generator`).

- **`oracle::carry`** (new module, `a80d8f4`) — the whole (A)+(B)+decision core:
  - `InvarianceIndex::lift` — the (A) index, per marked backing kind → the substrate axes its owner
    declared invariant, lifted from `state_stored_only_in()` `invariant:<axis>` colon-lines
    (`277` §4e). Enforces the **netns caveat** (`27C` §4(a)): a `net-kernel` store claiming
    `invariant:netns` is dropped + a loud `carry-netns-on-net-kernel-forbidden` diagnostic.
  - `read_set_closed(&Predict)` — the (B) conservative sh-taint pass over a verdict body,
    DEFAULT-DISQUALIFY over the audited safe-list (below). Returns `Closed { read_kinds }` or
    `Open(ClosureReject)`. Reads marks + sh structure only (`inv-referent-agnostic`); fails safe.
  - `decide_carry(crossed, closure, inv)` — the combined decision: substrate-scope (user excluded),
    then (B), then (A) per crossed dim × read kind (universal meet — any gap walls).
- **`plan::WrappedProbe::Carry`** (`fa6487b`) — a distinct disposition alongside `Enter`/`Degrade`;
  `compile_probe` ships it identically to Enter (the fact's HostDefault context steers the readback
  to ambient), and `build_wrapped_vouches` mints its elide/guard vouch (empty `enter_defs` ⇒ the
  guard shape is the AMBIENT inner check guarding the book bytes).
- **cli `build_wrapped_analysis`** (`fa6487b`) — builds the `InvarianceIndex` once; on the `Degrade`
  arm, `try_carry` runs (B) over the inner verdict body + `decide_carry` over the crossed dims; a
  Carry keys the `PeeledSite` context `HostDefault` and inserts `WrappedProbe::Carry`.

## The read-set-closure safe-list (my OWNED decision — the audit IS the artifact)

**Home:** `oracle::carry` (a pure static read of oracle-authored `Predict` bodies + marks, sibling to
`oracle::entry`'s decision machinery; the engine `analysis` stays book-focused; `inv-referent-agnostic`).

**DEFAULT-DISQUALIFY.** A verdict body is CLOSED iff every construct is on the safe-list below; ANY
other construct ⇒ `Open` ⇒ carry walls (fail safe: a missed-safe body loses an elision, never carries
a hidden read).

Admitted statements: `Assign` (clean RHS ⇒ binds the name clean; a DIRTY RHS disqualifies outright,
so no dirty-bound var ever exists) · `Annotation` (clean value, binds) · `Shift` · `While`/`If` (test
operands must be clean — a dirty branch condition disqualifies) · `Case` (clean scrutinee) · `Command`
(see below).

Admitted commands: (1) a MARKED command (verdict `:`/`:!` or observe `:?`) with clean operands — a
DECLARED read; its mark's dotted kind is recorded in the read-set (the (A) inputs). (2) an UNMARKED
command whose command word is a **pure builtin** with clean operands. The pure-builtin list — the
WHOLE unmarked-command safe-list — is exactly:

| builtin | rationale |
|---|---|
| `:` / `true` / `false` | constant-rc no-ops (no system read) |
| `return` | sets the verdict rc from a clean literal (the `return 2` decline sink) |
| `[` / `test` | string test over clean argv operands (branch/verdict conditions) |
| `printf` | emits clean argv to stdout; touches no system state |

Every OTHER command word (`cat`, `sysctl`, `id`, any tool) is an external read/effect, admissible
ONLY when MARKED. A pipeline (`cmd | cmd`) disqualifies.

Admitted words (clean values): `Literal` · `SingleQuotedLiteral` · `Positional` · `PositionalArgs`
(`"$@"`) · `PositionalStripPrefix` · `PositionalDefault` · `Var` bound clean earlier. DIRTY: a `Var`
referencing an unbound (ambient ENV) variable · `Word::Unmodeled` (command substitution `$(…)`,
`${x:-y}` env-defaults, arithmetic, process-substitution artifacts — the catch-all for opaque
external input).

**Deliberate spike-scope narrowing (disclosed, `churn-avoidance-disclosure`):** the spike-dialect
closable idiom is the MARKED-COMMAND (rc-partition) form — the tool reads the marked cell and its rc
IS the verdict. The `27C` STRAWMAN cmdsub-comparison spelling (`[ "$(sysctl -n "$1")" = "$2" ]`) is
NOT closable here: command substitution lifts to `Word::Unmodeled`, so a cmdsub whose OUTPUT is
consumed disqualifies (safe direction — walls, never a hidden read). Realizing that spelling as
closed needs cmdsub-VALUE modeling (the pass would see the inner read + its mark) — a named future
safe-list extension, not built. (B)-gated to the VERDICT body: carry applies only when the shipped
inner check IS the auto-cell verdict body (`inner_fn` ends `__is_converged`), so the closed body ==
the measured body; the predict-inner carry path is deferred (mirrors `27N`'s predict-inner deferral).

## DST battery coverage map (`27C` §9 row → test)

| `27C` §9 battery row | test(s) |
|---|---|
| closed body carries across fs-view | `closure_marked_command_argv_only_is_closed`, `decide_carries_closed_invariant_across_fsview`; e2e `carry-fsview-elides` |
| straddler (unmarked `$(cat …)`) walls | `closure_straddler_cmdsub_walls`, `decide_walls_straddler_via_closure`; e2e `carry-fsview-hidden-read-walls` |
| marked read of a NON-invariant kind walls | `decide_walls_marked_read_of_non_invariant_kind` |
| env-read disqualifies | `closure_ambient_env_var_walls` |
| opaque-call disqualifies | `closure_unmarked_external_command_walls` |
| process-substitution / clock disqualify | `closure_straddler_cmdsub_walls` (the `UnmodeledWord` class — `<(…)`, `$(date)` lift to `Word::Unmodeled` identically); `closure_pipeline_walls` (composition) |
| branch condition reading a varying store disqualifies | `closure_dirty_branch_condition_walls` |
| `net.*` not carried across netns | `invariance_netns_on_net_kernel_is_dropped_and_diagnosed`, `decide_walls_net_kernel_across_netns` |
| empty-oracle world byte-identical | `invariance_empty_world_licenses_nothing`; e2e 88 pre-existing cases byte-stable (rung-0) |
| (scope) user dimension excluded | `decide_walls_user_dimension_crossing` |
| (positive) non-net kernel carries across netns | `invariance_netns_on_plain_kernel_is_honored`, `decide_carries_plain_kernel_across_netns` |
| (degenerate) no marked read walls | `closure_no_marked_read_walls` |
| (positive) case-dispatch verdict closes | `closure_case_dispatch_with_marked_read_is_closed` |

e2e acceptance: **`carry-fsview-elides`** (a wrapped fs-view site elides via carry — both (A)+(B) hold)
and **`carry-fsview-hidden-read-walls`** (a hidden ambient read correctly walls; (A) holds, (B) fails,
isolating the closure pass as the decider).

## Rider — rider-wrapped-query-guard-fixture (bounded, DONE)

**`context-entry-wrapped-guard`** (new e2e): an unmodeled `horkwall` wall poisons the downstream
wrapped `sudo hork install frob` establish to `EstablishWritten` (can no longer elide); a reached
wrapped vouch + a converged probe mint the entry-composed GUARD, rendering
`( sudo__enter hork__is_converged install frob ) || sudo hork install frob   # dorc: guard [...]`.
This exercises the `27N` `tc-wrapped-guard-shape-unexercised` scaffold end-to-end. **Fixture ONLY —
no wiring needed**: the guard tier (`disposition_for`'s `EstablishWritten` arm) already consults the
wrapped vouch's entry-composed invocation; it simply had no fixture that landed a wrapped site past a
wall with a converged probe. (`27N`/`27D` say "query site"; the mechanism is `EstablishWritten` whose
guard-check is a read-only query — same shape.)

## Rework-vs-conformant accounting

- CONFORMANT (new): the entire `oracle::carry` module; the `WrappedProbe::Carry` variant; the cli
  `try_carry` + degrade-path integration; the three e2e fixtures. No existing surface was reworked.
- CONFORMANT (threading): `compile_probe` + `build_wrapped_vouches` extended their existing
  `Enter`-matching arms to also match `Carry` (both ship the entry-composed shape; the fact's context
  is the sole difference). No second plan/probe mint exit (`26B` single-choke-point held — data
  threaded into `build_plan_walled` + `compile_probe`→`ProbePlan`).
- The landed `compare` cross-context-Unknown chokepoint is UNTOUCHED and relied upon; the `FactKey`
  no-collision/no-transport pins stay green.

## Golden delta classes

- 88 pre-existing e2e: **NO delta** (byte-stable; `empty-world-byte-identical` — no fixture carries an
  `invariant:<axis>` line + a wrapper + a closed verdict, so carry never fires in the old corpus).
- 3 NEW cases (delta class = **new-fixture**, no pre-existing golden churned).

## Comment budget (`24P` §8 rider) — numbers + commands

- Denominator (added lines, code + authored fixtures, EXCLUDING generated goldens/copied logs/mocks):
  **920** (854 rust + 66 authored fixture). Command: `git diff ai/spike3-r27...HEAD --numstat -- '*.rs'`
  summed (854) + authored fixture `wc -l` (66).
- Numerator (Rust `//` non-doc + authored sh `#`, EXCLUDING `///` and generated fixture content):
  **92 raw ⇒ 10.0%**; **84 discretionary ⇒ 9.1%** (excluding the 8 required non-discretionary
  `#!/bin/sh` shebangs + `# dorc-lang/v0.1` markers). Commands:
  `git diff ai/spike3-r27...HEAD -- '*.rs' | grep -cE '^\+\s*//[^/]'` = 71 (rust inline);
  `grep -hcE '^\s*#'` over authored `book.sh`/`*.oracle.sh` (excluding copied hork/sudo oracles,
  expected.out, mocks) = 21 (of which 8 are shebang/marker).
- `///` doc-comments (mandated on public items, why-tier, slug-citing) are excluded from the numerator
  per the `27N` disposition-comment-budget-overage correction; checked by eye at checkpoint.

## tc-flags (flagged UP, NOT settled)

- **`tc-carry-verdict-body-only`** — carry runs (B) over the inner VERDICT body and is gated on the
  shipped inner check BEING that verdict body (auto-cell shape). The predict-inner carry path (a
  separate `__predict` ships) is deferred — closing the verdict but measuring the predict would be a
  body mismatch. Mirrors `27N`'s own predict-inner-entry-composed deferral. Whether to unify (close
  the SHIPPED body's AST, whichever it is) is a conductor judgment.
- **`tc-carry-keys-fact-hostdefault`** — carry keys the carried fact `Context::HostDefault` (measure
  ambient) rather than keeping the Wrapped context + an explicit cross-context answer-lookup. This is
  sound BECAUSE (A) certifies the cell is substrate-invariant (the ambient cell genuinely equals the
  in-context cell), and it keeps `compare` untouched. The alternative (keep Wrapped, add a per-site
  carry lookup) is more machinery for no soundness gain under (A)+(B). Flagged as the load-bearing
  representation choice.
- **`tc-carry-empty-read-set-conservative`** — a closed body with NO marked read is treated
  `NoCarry(NoMarkedRead)` (conservative — an argv-only tautology establishes no host fact worth
  carrying). Carrying it would be vacuously sound; the conservative choice costs nothing (such bodies
  are degenerate). Flagged.

## Disclosed gaps / deferred pieces (`ru-26`)

- **Cmdsub-value modeling** — the `27C` STRAWMAN `[ "$(sysctl -n "$1")" = "$2" ]` closable spelling
  walls here (cmdsub ⇒ `Word::Unmodeled` ⇒ disqualify). The spike closable idiom is the
  marked-command form; the cmdsub-comparison form needs the pass to see the inner read + its mark (a
  dialect + safe-list extension), not built. Safe (walls, never a hidden read).
- **Predict-inner carry** — carry applies only to auto-cell (verdict-body) inners. A marked oracle
  shipping a separate `__predict` does not carry (the shipped predict ≠ the closed verdict). Deferred.
- **netns positive path end-to-end** — the netns caveat + positive netns invariance are unit-pinned;
  no netns e2e fixture (netns entry-form details are root-only, `27C` §9 open corner). fs-view carries
  the e2e demonstration.

## What block-stdlib must maintain

- The `invariant:<axis>` line inside `state_stored_only_in()` is the (A) authoring surface: a
  substrate kind (`kernel`/`net-kernel`/`fs`) whose state is genuinely axis-invariant adds one
  `:   : invariant:<axis>` colon-line. A `net-kernel` store must NEVER claim `invariant:netns` (the
  caveat diagnoses it). The stdlib quality bar should lint an `invariant:` line against a who-am-I
  emission (the `271:rul-invariance-speech-act` contradiction-checker — a separate, existing surface).
- Stdlib verdict bodies that want to carry across a substrate boundary must be read-set-closed:
  every external read MARKED, no unmarked cmdsub/env/pipeline, argv-driven. The marked-command
  (rc-partition) idiom is the closable template.
- Whoever migrates the `FactKey`-widening into `command_effect` (the `27N` `tc-factkey-widening`
  flag): the carry path keys the `PeeledSite` context `HostDefault` on the Degrade arm — that keying
  decision must survive the migration (it is the carry license's manifestation).
