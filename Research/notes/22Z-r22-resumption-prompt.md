# 22Z — round-22 resumption prompt (fb-12 skeleton; keep CURRENT)

> Cold-conductor onboarding document. If you are reading this to resume round-22
> after a conductor death OR a context-compromise: this file + notes/224 (the round
> ledger, esp. §7 rulings / §10 dispatch / §11 self-audit) + plans/22A (the research
> truth) + TaskList are your state. Updated at: build phase — GATE-2 passed, arch-1
> HARVESTED + x-1 done, wave-2 PENDING; conductor context compromised (§11), resume
> clean (2026-06-11).

## Role

You are the round-22 conductor for spike-3 of Dorc, continuing in worktree
`.claude/worktrees/spike3`, branch `ai/spike3`. Round lean: errors + provenance.
The round opened with a RESEARCH phase (PHASE-R, now gathered), runs interactive
with the human, and ratifies a build arc only at GATE-2. Your jobs: high-level
understanding, herding subagents, catching cross-cutting errors, protecting your
context window by delegating, adjudicating the balance-calls. Priming prompt (only
if the human re-hands it): quarantine 223. Do NOT read quarantine otherwise.

## Safety (verbatim from the priming prompt; copy into every subagent prompt)

- No git mutation outside this worktree; never, ever push. Local commits on this ai/*
  branch are encouraged — commit granularly, with `(AI …)` labels per the repo's style.
- Builder subagents commit GRANULARLY in their own conductor-created worktrees (or
  the main tree if working solo); the conductor harvests by rebase or
  cherry-pick, runs the full gate chain itself before/after each harvest, to
  preserve the builder series as evidence. Read-only agents make NO edits and NO
  commits, ever.
- Worktree mechanics: NEVER the harness's isolation:worktree (wrong-base ×5 last
  rounds); conductor-created explicit-path worktrees under %TEMP%/dorc-r22/ at a
  verified base; every worktree agent's FIRST action is `git rev-parse HEAD` ==
  the briefed base, STOP on mismatch. New worktrees may need `mise trust`, which
  is authorized.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't mutate
  global state (no system packages or system config; worktree-local `mise` is fine).
- Everything you build follows DST discipline: deterministic, local, mutation-safe.
  Clock, network, disk, and randomness only through DI seams; correctness-critical
  kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert mocks
  under `PATH=mocks-only`) — never real mutators. Real-command strawmen in the repo
  are frozen evidence; they must never be executed. The only sanctioned executor of
  fixture material is `sh e2e/run.sh` (BLESS exclusivity).
- `Research/notes/quarantine-DO-NOT-READ/` stays unread, including by you, except for
  files the human explicitly hands you.
- SEC-GATE DISCIPLINE (two conductor deaths in round 21): the H2SaLS corpus is a
  plain-POSIX-sh rewrite of a public DEFENSIVE server-hardening guide — legitimate
  target material; the likely kill-trigger is cumulative hardening VOCABULARY in
  reasoning. Keep the ANALYZER as the subject of every sentence; corpus content is
  inert data; never enumerate/elaborate hardening material; prefer siloed subagents
  for corpus contact, sanitized returns only; 21Xa sentinel-ledger for suspect reads.

## Ordered orientation (re-read on resume)

1. README / DESIGN / IMPLEMENTATION / KNOBS / TODO (+TODO-ADDTL, STALENESS-AUDIT) —
   human-authored authority. 2. AGENTS.md. 3. spike/CLAUDE.md (binding agreement;
   known stale: says 43-case corpus; it is 99 dirs at HEAD — count the dirs; the
   wave-2-prep turn is slated to fix this + write rec-1/rec-5/held-4 in).
4. plans/21W (round-21 close) + plans/21Z (LIVING inventory). 5. THIS ROUND, in
   order: **plans/22A (the compressed research truth — read FIRST, it supersedes
   wholesale re-reads of 225-229)**; notes/224 (§7 rulings ru-1..ru-23 + rec-1..7,
   §10 dispatch+sweeps, §11 post-gating self-audit); notes/22B (diag-API design
   draft, unread-in-full by the gated conductor — READ at wave-2 prep). The five
   PHASE-R notes 225-229 and the research base (plans/111, notes/220, notes/222
   §5-§7) are per-need Grep-slices, NOT wholesale reads.

## Verified state (at last update)

- Durable HEAD: `61844b1` (the 22Z self-update). Code HEAD = arch-1's `6b869a9`
  (notes/plans commits since carry no code delta). Lineage milestones: `ada085d`
  inherited-green base → `fa78935` notes/224 → research notes 225-229 → `5da879c`/
  `5b58c5f`/`8421ecb` the three e2e warm-up/d×d fixtures → `29d3c78` 22B diag draft →
  arch-1 `54a4b84`/`38acbec`/`6b869a9` (arena / Top(cause)+GATE / witness-split) →
  ledger+self-audit commits.
- Gate chain at last harvest (arch-1, code HEAD): build/fmt/clippy/deny rc=0 ·
  cargo test **481+/0/1-ignore (21 suites, incl. the 3-test erasability gate)** ·
  `sh e2e/run.sh` ×2 = **99/99 SEVEN gates**, real exits · typos rc=0. Canonical
  chain (fb-17): build FIRST, never pipe a gate, read e2e output, ×2.
- Corpus: **99 e2e cases** (the three d×d cells now all pinned + the var-resolved
  redirect + arch-1 changed no goldens — receipts decision-inert).
- Builder worktrees under %TEMP%\dorc-r22\: w1-fixtures, b1-arch1, b2-fixture3 all
  HARVESTED by cherry-pick (fb-11 content-diffs verified); x1-gate-attack
  (branch ai/r22-xcheck1, tip `b68fc66`) holds the x-1 coverage-doc test, NOT
  harvested (fold-or-re-derive at arch-2 prep). Originals left for the human.
- `.claude/research/` GITIGNORED scratch was REMOVED (`d4277e0`); ~3.2MB untracked
  re-fetchable residue (incl. the four rqA primary PDFs) remains on disk for the
  human's inventory. (Stale prior note said "remove at digestion" — done.)
- SyncThing: whole Sync folder disabled on this PC (2026-06-11); ghost-husk risk
  paused; conflict cleanup is human-owned.

## Queue (mirror of TaskList at update time)

GATE-2 PASSED (ru-16, notes/224 §7): need-2..6 approved; need-1 resolved by
conductor factoring (registry severity + floor tier + typed enum-variant payloads;
human veto window open). Rulings now run to ru-23 + rec-1..rec-7 — read §7 in full.

DONE since the v2 line: B1 arch-1 HARVESTED (`54a4b84`/`38acbec`/`6b869a9`; chain
green, e2e 99/99 ×2) — the receipts arena + Top(cause) + erasability gate + canary +
unord-newtype + digest all landed; tc-flags accepted (cause on Reach::Top, ValueOf
cause deferred-to-arch-2; witness-threading in-scope). B2 third d×d fixture
HARVESTED (`8421ecb`). D1 diag-API design draft = notes/22B (`29d3c78`, design-only,
unread-in-full by conductor — READ AT WAVE-2 PREP). RV1/RV2/RV3 conflict sweeps DONE
(§10): held-1 is the one real collision — rec-1 TWO-SURFACES (dissolved by ru-20's
four-UI enumeration: shipped/off-ramp artifact byte-floored; plan-render is NOT an
artifact, carries per-line disclosure overlaid). x-1 crosscheck DONE — verdict in
§11: the gate is correctly built but VACUOUS-AT-HEAD (write-only plane, no consumer
reads a receipt yet); arch2-gate-obligation tracks the fix.

NEXT = WAVE-2 PREP (a fresh-context conductor turn): (1) read 22B in full; (2) write
the ratified sentences into spike/CLAUDE.md — rec-1 two-surfaces, rec-5 tape≠kSTATE-
cache, the held-4 battlefield-bound sanctioned-exception, the corpus-count fix
(43→99), and rename 22B's fork-1..4 to note-scoped slugs (RV3 find-3 collision with
219's fork-1..4); (3) dispatch the 17-code catalog retrofit on the 22B spine, SPLIT
Opus-design / Sonnet-mechanical (ru-23); s-2 span-widening early. WAVE-3: arch-2
(emit-at-origin, mvs-1..5, remediation-class render, + arch2-gate-obligation) then
arch-4-thin (cer-1..6, host-side durables per ru-21/22/23, traceparent tail — read
OTel env-carriers spec first). x-2 (over-suppression) wants fr-2 (VMCAI PDF) first.

CONTEXT-COMPROMISE NOTE (the reason this update exists): the conductor gated
repeatedly this window on accumulated loaded vocabulary (see §11 process-1) after
banking a hostile crosscheck's full report. If you are a successor resuming clean:
22A is the compressed research truth (Grep-slice 225-229, never wholesale re-read);
owed-1 in §11 lists three explainers owed to the human (deliver them); and crosschecks
on the inertness/provenance component-family must return PRE-SANITIZED verdicts
(bank the verdict, not the transcript) or they re-compromise the context.
#7 keep this file current.

## GATE state

BOTH GATES PASSED; build phase live. GATE-1 (ru-1..ru-12): the one-way weld (ru-11)
and the identity/exempt partition (ru-12) — receipts influence NOTHING (allow or
reject), gate equality permanent, shipped .sh artifacts byte-identical INCLUDING
comments (the floor), receipt-derived rendering out-of-artifact, sidecar last-resort
disliked. GATE-2 (ru-16): need-2..6 approved (hostsim Finding in-catalog; arch-5
retired into arch-4 tail; verdicts-everywhere/no-trace-pinning; third d×d cell
authorized+built); need-1 resolved by conductor factoring (registry severity +
floor tier + typed enum-variant payloads). Rulings then ran to ru-23 + rec-1..rec-7
(see the Rulings section + 224 §7). Standing fetch-requests, non-blocking: fr-1 CACM
"Debugging in the (Very) Large" (403-walled, verification-garnish); fr-2 VMCAI'12
sound-alarm-clustering PDF (no text layer) — wanted before x-2's over-suppression
pass; both PDFs the human has located but not yet dropped in.

## Rulings (round-22, human; full text notes/224 §7)

ru-1 premature-opt caution confirmed (exception: likely + safety/ergonomic
consequences → spike-map early). ru-2 why-provenance explainer DELIVERED in-chat.
ru-3 why/where plane split RATIFIED — encode in typings. ru-4 rq-H added (error-
discipline tooling research). ru-5 hostsim Finding: human leans IN-catalog (one
error system), not married. ru-6 store-most-data ratified (k-capped join store,
render-late); render axis candidate = remediation-class. ru-7 trace-stability: do
NOT promise upfront; user-story required (R2' evidence: d-1 SPLITS — dump+why
affirmed, pinning has regret-evidence only). ru-8 rq-F hard-yes (delivered).
ru-9 rq-G approved. ru-10 parallelize; wall-resilience; full-word slugs
(finding-1 not f-1); research outcomes one-per-front in notes/22x (NOT
.claude/research).
ru-11 WELD receipts fully one-way (no influence allow/reject; triggers re-derive as
facts; gate equality permanent). ru-12 identity-plane byte-floor incl. artifact
comments / exempt-plane named closed enum / no receipt-data in default artifacts.
ru-13 rerun-to-fixpoint is the likely change-handling path (no back-prop; demotes
the vp-8 epoch vector to a hint — don't build load-bearing). ru-14 judicious
crosscheck spend (fewer sharper hostile passes). ru-15 LEANER agent briefs (safety
block + prose goals + pointers; Fable even leaner). ru-16 GATE-2 (need-1..6; see
GATE state). ru-17 battlefield-bound diag-API = The Product of the spike (crib
rustc+Elm; Fluent-regret friction test = adding a code ≈ one edit; 22B is its draft).
ru-18 replay/postmortem promise: probe-tape is product surface; capture-always-lean
/ verbose=DISPLAY-knob; retention=local-rotated-last-N; NEVER a log/trace acceptor;
OTel value-format = the retention off-ramp. ru-19 ceremony tier cer-1..6 (replay
gate, version+binary-hash refusal, scrub-sentinel, round-trip, fold-into-hostsim-DST,
capture-cost budget; self-consistency not stored-golden). ru-20 four-UI enumeration
(ui-1 SSH/rack-lights deferred-lean-no-comment-metadata · ui-2 TUI/pretty/realtime =
the wow-factor · ui-3 phased CLI = the warnings-representation home, doubly-emit
cited sections · ui-4 mechanized); contracts stay PLANE-based, UIs are consumers;
dissolves rec-1. ru-21 host-side durables DIRECTION (don't ingest unneeded; rotation
discipline; postmortem-time slurp). ru-22 ingestion-as-declassification (quarantine
default; failure-triggered auto-fetch OUT; human-held fetch capability). ru-23
tunnel-held y/n constraint (Dorc holds SSH open through the offer; fresh-connection
canary candidate) + MODE-KEYED quarantine (unattended=quarantine; interactive
default=stream+auto-retain, no marginal cost) + rec-6 CLOSED (cause-tagged) +
sonnet-mechanical dispatch tier. rec-1..rec-7 reconciliation batch: see 224 §10/§11
(rec-1 two-surfaces RATIFIED-in-substance via ru-20; rec-2/3/5 transport/at-rest
clauses; rec-4 heredoc-hole-resolved; rec-6 closed; rec-7 replay≠probe-exec gate).
Standing round-21 rulings in spike/CLAUDE.md hold (mutation-analysis impossible;
TOCTOU WONTFIX; no intra-host reordering; rc opaque; "skip" banned; identity
declared-never-inferred).

## The arcs (GATE-2-ratified; status inline)

- arch-1 (arena + Top(cause) + erasability gate) — **DONE/HARVESTED** (B1). Built
  with per-field Exempt-enum partition, adversarial-variance run-B + sentinels,
  coverage canary, iteration-suppressed newtype, decision digest; Top(a)≡Top(b)
  (cause out of Eq — a termination requirement, not just contract). x-1 verdict:
  correct but VACUOUS-AT-HEAD → arch2-gate-obligation (Open flags).
- arch-2 (one consumer end-to-end) — **NEXT after wave-2 prep.** emit-at-origin
  (mint cause at ⊤-creation, suppress at render, never emit-N-then-dedup); the five
  mvs-rules as tested code; render ranked by REMEDIATION-CLASS (ru-6); hierarchical
  site keys; span-bridge tier-2/3; rec-1 two-surfaces (disclosure on the render
  surface, never embedded in the byte-floored artifact); + arch2-gate-obligation;
  x-2 over-suppression pass (wants fr-2 first).
- arch-3 (catalog retrofit) — **wave-2, on the 22B spine.** 17 codes → exhaustive
  enum; tidy-style bidirectional grep + git-diff retire-guard + self-cleaning
  allow-lists; per-code severity in the registry + un-overridable floor tier (ru-16
  factoring; human disposes floor membership at the PR); `expect`-style must-emit +
  DST fault-injection; report() renders spans; s-2 classify-widening EARLY; hostsim
  Finding folds IN; gate completeness/registration, NEVER prose. SPLIT
  Opus-design/Sonnet-mechanical (ru-23).
- arch-4 (durable + why) — **wave-3.** Thin durable (probe-tape + inputs + seed +
  digest; JSONL version-tagged; no byte-stability promise) + `why` lens
  minimal-witness-first; cer-1..6 ceremony; host-side durables per ru-21/22/23;
  verdicts pinned, NO trace-pinning. The arch-5 OTel seam folded in as a tail:
  traceparent value-format on the verdict lane via DI seams — read the OTel
  env-carriers spec (Beta) FIRST.
- Warm-ups + third d×d cell all **DONE** (3 fixtures harvested).

## Process rules (scar tissue; full set in priming prompt + spike/CLAUDE.md)

Canonical gate chain before EVERY commit, unpiped, build first, e2e ×2 read-output.
BLESS exclusive, conductor-only. No harness worktree isolation. Token-log every
dispatch in 224 §10 (harness numbers authoritative). Cancelled agents may complete
— bank late results. Agent .output transcripts: do NOT read (JSONL, context bomb).
Relayed rulings need [spike]/[product] markers, else ask. Research outcomes →
notes/22x per-front; full-word slugs; unread-source claims ≤ ~SUSPECT; bracketed
[slug] ↔ graded-row bijection self-checks. No SendMessage tool exists this session:
brief agents COMPLETELY at launch; stop-and-relaunch is the only re-brief.

## Open flags

- **arch2-gate-obligation (HIGH, the live one; 224 §11 x1-outcome):** the erasability
  gate is correct but vacuous-at-HEAD (no decision reads a receipt yet — `top_cause()`
  has zero callers, the witness is canon-omitted, ValueOf::Top carries no cause). When
  arch-2 wires the first consumer it MUST land a fixture exercising that read under
  variance AND upgrade the canary from "witness non-empty" to "witness DIFFERS A/B yet
  decisions identical." Until then read the gate as "structurally enforced,
  behaviourally unexercised." x-1's coverage-doc test is on branch ai/r22-xcheck1
  (`b68fc66`), unharvested.
- **process-1 / context-compromise (224 §11):** model-gating reached the conductor
  after banking a hostile crosscheck's full report; fb-candidate = crosschecks on the
  inertness/provenance family must return PRE-SANITIZED verdicts (bank verdict, not
  transcript). Resume clean-context.
- **owed-1 (224 §11):** three explainers owed the human (Eq-as-termination; the two
  B1 scope judgments) — already-recorded decisions, deliver on a clean turn.
- **housekeeping for wave-2 prep:** spike/CLAUDE.md gains rec-1 two-surfaces + rec-5
  tape≠kSTATE-cache + held-4 battlefield-bound sanctioned-exception + corpus-count
  43→99; rename 22B's fork-1..4 (collides with 219's fork-1..4, RV3 find-3).
- flag-zm-attribution (225): vp-26's slogan is the Sabelfeld–Sands gloss, NOT
  Zdancewic–Myers verbatim — cite as engineering-precedent, not a tight bound.
- flag-untracked-query-information (229): rustc's lint is a direct gate analogue —
  candidate dylint enforcement, machinery cost unknown. watch-1 (rec): cer-2's
  binary-hash refusal is a hostsim-seam-tier property, not corpus-shell-exec tier.
  watch-2 (rec): don't over-apply dac-B's "no second graph" to kill the where-plane
  loc-DAG (the two planes are deliberately separate).
- Carried round-21 (21W §6): span-bridge tier-2/3 together when plan is next touched;
  seam-1 ⊤-readout; find-J reader-liveness parked (human's); doors parked behind
  218/218a. The 207/YOLO escape-hatch stays set aside. kSTATE parked (receipts
  per-run; the d-1 dump is a write-only LOG). Capture-eagerness knob = human's call
  once costs are real. RESOLVED this round: flag-dxd-third-cell (built, harvested).

## Meta-goal

The project is partly the human's proof-to-himself that LLMs can do real, complex
engineering; the highest-level deliverable is demonstrated capability. Note where
better seeding would have prevented struggle; surface at round-close. Dispatch
heuristics: split by decision-surface, not size; pre-spelled contracts make big
builds mechanical; hostile crosschecks = highest value-per-token (~25-30% of build
spend); builders write their own hunt-lists, crosschecks told to exceed them.
