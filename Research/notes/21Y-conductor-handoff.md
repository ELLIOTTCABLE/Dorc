# 21Y — Conductor handoff prompt (defensive; for a replacement orchestrator-Fable)

> WHY THIS EXISTS: the round-21 orchestrator (or a subagent) hit Fable's cybersecurity
> safety-gate. If the conductor cannot be resurrected, a fresh high-capability Fable is
> started FROM THIS FILE to resume the round. The H2SaLS corpus is a DEFENSIVE
> server-hardening rewrite — legitimate target material; if a sec-gate fires, the
> trigger is most likely the corpus's hardening vocabulary in reasoning, not a real
> security task. Keep the analyzer (not the workload's domain) as the subject in all
> reasoning and subagent briefs.
>
> Maintained by the lower-capability caretaker while in-flight agents land. Last
> updated mid-#13. Append/refresh as work lands.

## SAFETY (propagate verbatim to every subagent prompt)
- No git mutation outside this worktree; never push. Local commits on `ai/spike3`
  encouraged, granular, `(AI …)`-labelled.
- Don't mutate global state (no system packages/config; worktree-local mise fine).
- DST discipline: deterministic, local, mutation-safe; clock/net/disk/rng via DI only;
  kernel stays dependency-clean.
- Executable fixtures use inert stubs under PATH=mocks-only; real-command strawmen are
  frozen evidence, never executed; only `sh e2e/run.sh` executes fixture material;
  BLESS is orchestrator-only, never with build-agents in flight.
- `Research/notes/quarantine-DO-NOT-READ/` stays unread (incl. the priming prompts).

## ORIENTATION (read in this order)
1. README.md, DESIGN.md, IMPLEMENTATION.md, KNOBS.md, TODO.md (root, human-authored).
2. STALENESS-AUDIT.md if present (human voice; rulings beat prose).
3. AGENTS.md, spike/CLAUDE.md (working agreement + every inv-* invariant), and
   crates/*/CLAUDE.md.
4. Round-20 close: Research/plans/20K, 20U, 20V (the errexit-doors charter).
5. This round's notes: 211 (plan-of-attack) → 212 (mid-round rulings) → 21F (r1A
   report) → 21G (spike-4/error-tooling direction) → 21K (direction batch 3,
   PROVISIONAL) → per-arch build notes (213/214/215/216/219/21B/21D/21E/21H) →
   220/221/222 (round-22 research, filed under 22x convention).

## ROUND-21 STATE AT HANDOFF (HEAD = 0c48e07)
Landed/committed on `ai/spike3`:
- door-3 (StatusInvariant `|| true`): note 213.
- arch-1 (span-edit render; StatusRenderFloor retired → StatusRelaxable if-guard +
  StatusIterated while/until; heredoc refuse-class): note 214; P1 fix (adjacent
  multi-line splice) note 21E.
- arch-2 (budget-bounded function inlining; SkipClass::InlineCall; all-or-nothing CALL
  license; site N.M sub-records): note 216.
- door-1 (cascade folds at base — zero engine edits, 7 corpus cases): note 215.
- arch-6 (dorc-coverage dashboard; per-door attribution; H2SaLS = 0% full-elision,
  decomposed): note 21B.
- arch-7 (seeded differential harness in hostsim; 500/500 clean): note 21D.
- y-1 (redirect-write cells, closes the imp-1 stale-guard hole) + q-2 (cmdsub
  ⊤-diagnostics floor + core::diag catalog): note 21H. THIS IS HEAD (0c48e07).
- Research notes 220/221/222 filed (round-22 material).

## IN-FLIGHT AT HANDOFF — NOW DRAINED (updated post-#13-landing)
- NO agents remain in flight. TASK #13 (wave-2 fix slice) LANDED: 5 files modified,
  +329 lines, UNCOMMITTED at HEAD 0c48e07 (cfg.rs, tests/cfg.rs, effect.rs,
  core/src/diag.rs, coverage/src/lib.rs). Its full completion report + diff snapshot:
  %TEMP%/dorc-r21/handoff/{13-report.md, inflight-13-uncommitted.diff}.
  Agent-claimed gates green ×2 — RE-VERIFY before committing (verify-don't-relay).
  Two NEW flags it raised, neither decided: tc-fix3-severity (the depth-2-positional
  refusal is a Note because the diag catalog is Note-only this round; sibling inline
  refusals are warnings — make louder only by moving off-catalog or changing the
  catalog severity invariant) and the detached-funcdef-copy asymmetry (the unreachable
  definition copy still splices a positional body harmlessly; only the reachable copy
  refuses; lower_funcdef deliberately untouched).
- ALSO UNCOMMITTED: notes 21K (direction batch 3) and this 21Y file — commit at first
  quiesce.

## WAVE-2 CROSSCHECK VERDICT (already delivered; drives #13)
- arch-2 single-level inlining: SOUND under attack (per AI crosscheck — process
  evidence, not proof).
- P1 (m-2): the imp-1 stale-guard hole confirmed LIVE — CLOSED by y-1 (HEAD 0c48e07).
  Orchestrator verified `: > f` and bare `> f` shapes also now keep the guarded
  install live.
- P2 (m-6): dashboard heredoc over-count → #13 fix-1.
- P2 (m-8): depth-2 transitive inlining broken-but-SAFE (positional non-thread → runs;
  record double-count → redundant) → #13 fix-2/fix-3. NOTE: this REFUTES note 216 §6
  hunt-1's self-claim that depth-2 works — the correction belongs in note 217 (the
  orchestrator's reconciliation note, append-only; do NOT edit 216).
- flake (m-7): render21-adjacent-multiline-elides exec-gate flake = STALE shared-target
  binary, not a harness race. Closed.

## REMAINING ROUND-21 WORK (orchestrator queue)
- HARVEST #13 when it lands (gates ×2, commit).
- NOTE 217: reconcile wave-1 + wave-2 crosschecks (verdict tables; arch-1 P1→21E;
  wave-2 P1→y-1; depth-2 correction to 216; flake closed). Orchestrator writes it.
- TASK #7 arch-5 (partial-member list-rewriting): ENTRY GATE verified — self_reach_holds
  (effect.rs ~620) IS global-pristine (empty suppressed in-state). Human AUTHORIZED the
  cell-family re-scope under EXPANDED obligations (21K d-6 + task #7 description):
  deep-preamble pole mints soundly · sibling-writer-via-back-edge still refuses ·
  TWO-LEAF body floor pinned explicitly · value-plane pole · multi-effect future-hazard
  note · dedicated hostile pass (load-bearing, since 20T validated the STRICT form).
- TASK #5 door-4 + door-2 + precedence seam: BUILD LAST, behind a default-OFF CLI flag
  (human ruling, 212): apply stays pure elision-only; the `Never` seam position must
  provably produce zero transforms; precedence seam keeps oracle-default/engine-global/
  admin-per-book ALL live (dq-errexit-2 genuinely open); hostile crosscheck mandatory
  (four-world trace + correlated-failure/boundary-3 attack).
- TASK #12 harness pass: newline-safe mock-log protocol; EXIT_RC=<n> case marker
  (convert door1-and-form to exec-asserted); the dual-rail corpus harness + confound
  battery (human's DST direction); DST-position note. Orchestrator-scope.
- ROUND-CLOSE: the durable close report (20K analogue); plans/21Z living spike-4
  inventory (NOT a wrap-report section — a living doc); seeding feedback; dispatch-
  heuristic refinements.

## STANDING HUMAN RULINGS THIS ROUND (do not relitigate)
- NEVER vouch for AI output as good/sound/proven. Internal AI-run gates/crosschecks are
  PROCESS EVIDENCE and a triage signal for human-review priority — never proof. The
  codebase is slop until externally human-battle-tested. (Hard limit; memorized.)
- dq-errexit-1/2/3 + declaration spelling = the human's; surface, never settle.
- door-4 is product-hard-deferred; spike builds it behind a flag, last.
- Parallel builders: orchestrator-created explicit-path worktrees at a verified base
  (NEVER harness `isolation:worktree` — it cut wrong-base 5×; every worktree agent
  verifies `git rev-parse HEAD` == briefed base as FIRST action). Build worktrees live
  under %TEMP%/dorc-r21/ (keep target/ out of SyncThing).
- Research dives are normally his interactive-research skill (main-context, adjudicated);
  the 3 background dives this round are gathering-only — read cited raw sources before
  trusting (fb-6: unread-source claims cap at ~SUSPECT).
- Explain corpus slugs to the human in conversation; design over implementation;
  he tracks via the chat, not the corpus.

## TEMP ARTIFACTS (not in git; under %TEMP%/dorc-r21/)
- handoff/ — 13-report.md (the #13 completion report) + inflight-13-uncommitted.diff
  (its full 329-line diff) + status.txt. THE RESUMER'S FIRST READ after this file.
- worktrees: cmdsub, coverage, door1, hostsim, p1fix, xcheck2 (disposable; all
  harvested); shapecheck (orchestrator's y-1 shape-verification scratch).
- Harness note: agent `.output` transcript files under %TEMP%/claude/.../tasks/ are
  EMPTY on disk (results arrive via notifications only) — do not infer liveness from
  their mtimes; the caretaker got burned by this once.
