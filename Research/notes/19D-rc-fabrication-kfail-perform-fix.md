# 19D — Wave-4 fix: stop fabricating `rc=0`; gate branch-consumed elision on a declared rc

> AI-authored strain-log. Confidence-marked (+SURE / ~SUSPECT / -GUESS / --WONDER). Trust
> R/D/I/K and `19A §5`/`19B`/`19C` (the human's rulings) over this. Continues `19C` (the apply
> fold). This is a **welded-redline `kFAIL-perform` correctness fix** — a live priority-1
> under-execute that `19C` left masked by two hand-injected `rc=9`/`rc=0` literals.
>
> Landed (this worktree, `ai/round19-keystone`): see §6 for hashes. Baseline and result both
> GREEN: workspace 16/16 test-binaries pass, clippy clean (0 warnings), **43/43 e2e** (was 42;
> +1 new case; the one pre-existing `render-case-arm-oneliner-wrong` xfail unchanged).

## 1. The bug (proven live, by an adversarial trace, through the real CLI)

`cli::parse_results` and `hostsim::observe` **fabricated `rc=Some(Rc(0))` for any `converged`
fact with no explicit rc.** That fabricated 0 flowed into `19C`'s apply fold (`Known(0)` controller
short-circuits) and into convergence-elision (`StandIn::from_rc(0) = true`), both treating it as a
**known success**. For a **non-conforming establish** (a command that exits non-zero when its fact
already holds — `useradd` rc 9, `mkdir` w/o `-p`, `ln` w/o `-f`, `docker network create`) whose
status is consumed by a `&&`/`||`/guard, the real converged rc is non-zero, so the fallback MUST run
— but the fabricated 0 made the engine prune it.

Reproduced through the real CLI (`useradd deploy || mkdir /srv/app`, `user#present converged`, **no
rc**): the rendered apply collapsed the whole line to `true` — `mkdir` **silently dropped**. A
priority-1 under-execute (`inv-kfail`/`kFAIL-perform`). Two hand-injected `rc=9`/`rc=0` literals (the
`observable_matrix` promoted test + the `andor-rc-vouch-wrong` / `fold-oror-guard-omits` e2e
`probe-results.txt`) MASKED it — they declared the rc the default path never would.

The core authors KNEW `rc=None` must be ⊤ (`core::Observed` doc: "A converged fact whose rc is
un-injected falls back to a conforming `Rc(0)` only at the *caller's* explicit choice, never
silently here") — but the CLI/hostsim callers *took* that fallback, defeating the ⊤.

## 2. The fix (option B, applied uniformly — exit-status is observable-liveness-gated)

Two layers (the fabricated 0 poisoned BOTH; closing it needs both):

- **(1) Stop fabricating `rc=0`** (`cli::parse_results`, `hostsim::observe`). A converged fact with
  no explicitly-declared rc now carries `rc=None` (⊤), NOT `Some(0)`. An explicit `rc=N` in stdin /
  a test's injected `Observed` still yields `Some(N)`. **+SURE this alone fixes the fold side:** the
  fold reads `Top` ⇒ `is_success()==None` ⇒ no short-circuit ⇒ the fallback stays live (verified: the
  default-path repro now keeps `useradd … || mkdir` verbatim, both run).

- **(2) Gate branch-consumed `Replace` on a declared rc** (the residual the fold doesn't cover —
  the `Replace` *disposition*). After (1), a converged non-conforming establish on a `||`-left still
  got `Replace(license, True)` (a latent rc-0 lie, masked only by the line-render keeping a mixed
  line verbatim). Closed by:
  - the engine now marks a `&&`/`||` left operand's status `Observable::AndOrStatus`
    (`cfg::lower_and_or`; generalising the F1 `if`/`elif`-only `mark_status`), emitted un-collapsed
    (`inv-superposition`);
  - `prove_replaceable` gained an `observed_rc: Option<Rc>` and refuses the license for a consumed
    `AndOrStatus` **when the rc is `None`** (⊤ ⇒ run, the floor) — and **relaxes** it when the rc is
    `Some(N)` (the value-preserving stand-in `StandIn::from_rc(N)` reproduces the exact status, so the
    branch decides identically — `19A §5`).

- **(3) Status NOT consumed ⇒ rc dead ⇒ elide freely** — unchanged. A bare converged establish with
  no branch consumer still elides (its stand-in's rc is never read). Verified: `apt-get install`
  (converged, no rc) still renders to `true`.

The net: undeclared-rc + branch-consumed ⇒ run (safe); declared-rc ⇒ fold/substitute with the exact
value (value recovered when the oracle declares the rc — the C-4 gradual-enhancement bargain).

### The keystone judgment-call (why two `Status` variants, not one uniform relaxation)

+SURE (proven empirically — see strain-A): a *uniform* rc-relaxation of `if`/`elif` BREAKS `dash -n`
(`ap-2`). The line-granular `render_apply` cannot substitute a guard sharing its line with the
`if`/`then`/`fi` scaffolding: a multi-line `if cmd\nthen\n…\nfi` with a declared rc renders
`# if cmd` / `true` / `then` / … — an orphaned `then` (the 19C strain-D break, reproduced). So the
prompt's "generalises … uniformly, relaxes when rc declared" is right for `&&`/`||` but **render-unsafe
for `if`/`elif`**. Resolution (type-honest, `inv` "illegal-states-unrepresentable"): TWO observable
variants by *render-expressibility*, not semantics —
- `Observable::Status` (`if`/`elif` guard) — blocks **unconditionally** (the render floor; full
  retirement waits on the leaf-exact / structural render `C-5`, exactly as `19C` strain-D ruled);
- `Observable::AndOrStatus` (`&&`/`||` left operand) — blocks **only when rc undeclared** (the render
  CAN express it: operand+operator on one line, the fold + the `render_apply` omit-safety gate handle
  it).
Both are rc-relaxable at the *disposition* layer; they differ only in whether the line-render can
express the substitution. I did NOT disturb the `if`/`elif` behaviour (kept its `Status`, its
emission, and its cfg tests byte-identical) — 19C strain-D's human-reviewed "keep mark_status for
if/elif" stands.

## 3. The value-delta (what no longer elides without a declared rc — the intended trade)

`kFAIL-perform` priority: never under-execute > avoid over-execute > avoid unnecessary-execute. The
fix trades *unnecessary-execution* (lowest) to remove an *under-execute* (highest) — the correct
direction. Concretely, a converged establish whose status is **branch-consumed** no longer elides
UNLESS its rc is declared:

- **`&&`/`||` left operand, converged, undeclared rc** ⇒ now **runs** (was wrongly `Replace`d/folded).
  e.g. `apt-get install && systemctl start` (converged, no rc) keeps the line verbatim — `install`
  re-runs. *This is a value-loss for a conforming post-condition that doesn't declare rc-0.* It is
  recovered the moment the oracle declares `rc=0` (then it relaxes and elides). +SURE this is the
  intended trade; the `&&` post-condition `pins_*`/`f1_andand…` tests kept eliding *because they
  declare rc=0*.
- **`command -v guard || install`, guard converged, undeclared rc** ⇒ now the line runs verbatim
  (was: fold omits the install, line collapses to `true`). The e2e `fold-oror-guard-omits` case had to
  add `rc=0` to its `probe-results.txt` to keep demonstrating the fold-omit — now explicitly the
  *declared-rc opt-in* path. ~SUSPECT this is the most visible real-world delta: the canonical
  `dpkg -s || install` idiom's guard (`command -v`, `dpkg -s`) is read-only and conformingly-rc-0, but
  Dorc no longer *assumes* that — build-2's read-only `.check()` must declare/produce the rc (which it
  trivially does: it runs the check and reads `$?`, `19B §3`). So the idiom keeps eliding once oracled,
  which is the whole gradual-enhancement bargain.
- **bare converged establish (no branch consumer)** ⇒ **unchanged**, still elides (status dead).
- **errexit-consumed status** ⇒ **unchanged**, still vouched/elidable (see §5 `tc` note).

The delta is bounded to *branch-consumed status with an undeclared rc* — exactly the cell where a
fabricated rc-0 was a confident wrong value.

## 4. Masking tests corrected (the default path is now asserted, not hidden)

- `plan/tests/observable_matrix.rs`:
  - KEPT `nonconforming_establish_andor_left_operand_substitutes_exact_rc` (the declared-`rc=9` opt-in:
    `Replace` by `(exit 9)`, `mkdir` `Run`).
  - ADDED `andor_left_operand_undeclared_rc_runs_kfail_perform` — **the default-path assertion the
    prompt required**: converged `install || start` with **no declared rc** ⇒ the leaf is `Run` (not
    `Replace`/`Omit`). This is the honest pin the old `should_panic`-"not replaced" used to be.
  - REWROTE `f1_andand_left_operand_stays_replaced_tc_mint_gap` →
    `andand_left_operand_declared_rc0_relaxes_and_replaces` (the `tc-mint` gap is RESOLVED, not a gap;
    the declared rc-0 relaxes — renamed + recommented so the green doesn't hide a stale "deferred gap"
    framing).
  - `prove_replaceable` unit tests: added `andor_status_blocks_only_when_rc_undeclared` (the
    rc-conditional gate) + `if_guard_status_blocks_unconditionally` (the render floor); strengthened
    `no_license_when_unvouched_output_consumed` to pass `Some(Rc(0))` (proving stdout/stderr block
    *regardless* of declared rc); threaded the new arg through the 5 existing call sites.
  - Updated the module-doc A/B contrast + the NON-CONFORMING comment block (both described the
    pre-fix "status is fine" / "unmarked &&/|| / NOT fixed here" model).
- `analysis/tests/cfg.rs`: REWROTE `consumed_andand_left_operand_does_not_mark_status` →
  `consumed_andand_left_operand_marks_andor_status` (it now asserts the engine marks `AndOrStatus`,
  NOT the stale "left unmarked"); ADDED `consumed_oror_left_operand_marks_andor_status` (the `||` dual).
- e2e: ADDED `andor-rc-undeclared-runs` (the no-`rc=9` sibling of `andor-rc-vouch-wrong`) — drives the
  real CLI under inert mocks; `expected.ran` asserts **both** `useradd` and `mkdir` ran (the safe
  default, line verbatim). Updated `fold-oror-guard-omits` to declare `rc=0` (the opt-in path).
  `BLESS=1` perturbed ONLY these two cases' goldens (git-diff-verified) — no other case's output
  changed, confirming the fix did not silently widen/narrow elsewhere (the no-mask check).

## 5. `tc-*` flags (never settled in isolation — flagged up)

- **`tc-reliability` / `tc-mint` (the fix IS a `tc-reliability` resolution the orchestrator ruled =
  option B):** an undeclared rc on a non-conforming establish is now treated as ⊤ (run), not a
  confident 0. A *declared* rc is trusted (build-2's contract); a *lying* declared rc is the
  forged-observable exposure (`19C` strain-F / `an-host-as-adversary`) — **no worse** than the existing
  convergence-elision's trust in the verdict, and `inv-must-may` (Must) + the `kFAIL-withhold` sandbox
  remain the defenses. Not a new exposure.
- **`tc-collapse`:** the rc-relaxation is the *apply caller's* collapse (`prove_replaceable`), reading
  the engine's un-collapsed `AndOrStatus` (`inv-superposition`). The engine bakes no rc-policy. +SURE
  right home; flagging it.
- **errexit (a `tc` I scoped OUT, flag for the orchestrator):** the prompt's fix-2 listed "or errexit"
  as needing a declared rc. I **left errexit untouched** (it still never marks a status variant ⇒ stays
  vouched/elidable). Rationale (~SUSPECT, please adjudicate): errexit-consumed status on a *converged*
  establish is genuinely vouched — convergence ⟹ the desired state holds ⟹ the establish "succeeded",
  so `set -e` correctly does not abort, and eliding it (not running an already-done establish) is right.
  Unlike a `||`-guard, no *different* command's reachability turns on the rc; errexit's only decision
  is "abort-or-continue", which is "continue" for a converged establish. AND `19A §5` superseded `19C`-
  C-3's "errexit is special" with "abstract-interpretation gets it for free", AND `set -e` is `Opaque`-
  poisoning in practice so an ambient establish rarely survives under it anyway. Marking errexit-status
  risked re-breaking the deliberately-restored `set -e`-doesn't-poison elision (`spec_converged_set_e_
  does_not_poison_replacement`). So I judged errexit out-of-scope for *this* under-execute (which has a
  proven `&&`/`||` trace; errexit does not). If the orchestrator disagrees, it is a *separate* gate.
- **NOT raised / out of scope:** `tc-uniqueness`, `tc-taint`, `tc-user` (build-2's), `tc-phase` (the
  fold/`prove_replaceable` stay apply-phase-agnostic, unchanged from `19C`).

## 6. Exclusion-check (AGENTS four axes) — run, recorded

- **other phase (probe vs apply):** +SURE the fix is apply-side. `parse_results`/`observe` are I/O
  edges (cli) / the DST seam (hostsim) — `inv-determinism` exempt; neither leaks a phase inward. The
  `AndOrStatus` mark is engine-emitted un-collapsed; the rc-relaxation is the apply caller's collapse.
  A probe does not run the apply fold nor `prove_replaceable`. ✓
- **other reliability:** see §5 `tc-reliability`. Undeclared rc ⇒ ⊤ ⇒ run (the new floor *reduces*
  exposure). A forged declared-rc is build-2's contract exposure, no worse than today. ✓
- **reverse / backward:** untouched (the backward/`Must`/apply-3 tower stays un-instantiated, `ch-scope`).
  The `AndOrStatus`/rc-gate is forward (apply-2). ✓
- **other user (admin vs engineer):** the fix moves rc-production burden *entirely* to the engineer's
  oracle (build-2 opt-B). The admin's default (write idiomatic sh, declare no rc) is now safe-by-default
  (run). The engineer recovers elision by declaring the rc. ✓ — and this is precisely the C-4 gradual-
  enhancement contract.

## 7. What build-2 (the real oracle-contract side) MUST maintain (hand-forward)

- **+SURE, the load-bearing one:** build-2 MUST produce a **non-conforming establish's exact converged
  rc** (`useradd`→9, `mkdir` w/o `-p`→1, `ln` w/o `-f`, `docker network create`, …) via opt-B (`19B §1`)
  on the OOB lane (`$DORC_VERDICT` as structured `rc=N` data, `19B §2`), never an exit code. The
  injection point is exactly `observe: Fn(FactKey)->Observed`'s `rc` field — which now defaults to
  `None` (run) instead of a fabricated 0. If build-2 omits a non-conforming rc, the establish simply
  **runs** (safe) — no longer silently under-executes. So the failure mode shifted from
  *silent-under-execute* (catastrophic) to *unnecessary-execute* (benign) — the correct `kFAIL-perform`
  posture. The value (elision) is recovered iff build-2 declares the rc.
- A read-only `.check()` (`command -v`, `dpkg -s`) trivially produces its rc (run the check, read `$?`,
  `19B §3`) — so the canonical `guard || install` idiom keeps eliding once the guard is oracled. The
  un-oracled / undeclared-rc case is the (safe) floor.
- **`Observable::AndOrStatus` vs `Observable::Status`** is a render-expressibility split, NOT a
  semantic one. When the **leaf-exact / structural render (`C-5`)** lands, the `if`/`elif` `Status`
  unconditional block can ALSO become rc-relaxable (substitute the guard in-situ, keep `then`/`fi`),
  collapsing the two variants back to one. Until then, `Status` stays the render floor.
- The cli stdin `rc=N` grammar is the build-1 stand-in for the OOB lane; build-2 replaces it with
  `$DORC_VERDICT` records. The `rc=None`-default semantics must survive that swap (an absent OOB rc is
  ⊤, never a fabricated 0).

## 8. Honest scope notes / what I did NOT do

- Did NOT touch errexit-status handling (§5) — flagged for orchestrator adjudication; the proven
  under-execute is `&&`/`||`, not errexit.
- Did NOT retire the `if`/`elif` `Status` block (render floor, 19C strain-D) — kept byte-identical;
  it waits on `C-5`.
- Did NOT build the oracle-contract side (build-2) — the rc is still *injected* (cli `rc=N` / test
  `Observed`), now defaulting to `None`.
- Did NOT touch the backward/`Must`/apply-3 tower (`ch-scope`), `case` string-folding (`19C` strain-C),
  or the `render-case-arm-oneliner-wrong` xfail (unchanged render-fidelity defer).
- `while`/`until` branch-status is vacuous (loops are ⊤-rejected today); a bare top-level `!`-pipeline
  marks no status (its status gates nothing) — both correctly left unmarked.
