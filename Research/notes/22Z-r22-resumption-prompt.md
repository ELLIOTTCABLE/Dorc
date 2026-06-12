# 22Z — round-22 resumption prompt (fb-12 skeleton; keep CURRENT)

> Cold-conductor onboarding document. If you are reading this to resume round-22
> after a conductor death OR a context-compromise: this file + notes/224 (the round
> ledger, esp. §7 rulings / §10 dispatch / §11 self-audit) + plans/22A (the research
> truth) + TaskList are your state. Updated at: build phase — WAVE-2 COMPLETE
> (arch-3 core done: B3 spine + B4b sweep both harvested, chain green, zero
> goldens; fb-19 logged); NEXT = arch-2 prep, fresh conductor turn (2026-06-11).

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

- Durable HEAD: the wave-2 ledger commit (this update's own). Code HEAD = B4b's
  harvested `0e0a470` (B3 spine `894109c`+`9c4b621`; B4b sweep `6f4862c`+
  `0e0a470`; both fb-11-verified). ARCH-3 CORE COMPLETE: all 23 diagnostic
  codes on the typed spine, allow-list EMPTY, diag::legacy deleted, chain
  green unpiped (e2e 99/99 ×2, ZERO golden diffs across both harvests).
  Earlier housekeeping: `463c0b0` + `cb695a9`; fb-19 process commit `b217073`. Lineage milestones: `ada085d`
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
  b3-arch3 (branch ai/r22-arch3, tip `b6c0b78`) HARVESTED by cherry-pick
  (`894109c`+`9c4b621`, fb-11 EMPTY). b4-arch3m: ABANDONED-DIRTY (fb-19
  recursive-self-delegation failure; 7 uncommitted files, human inventory;
  branch ai/r22-arch3m has zero commits past base). b4b-arch3m2 (branch
  ai/r22-arch3m2, tip `4085bd4`) HARVESTED by cherry-pick (`6f4862c`+`0e0a470`,
  fb-11 EMPTY). b5-e2equiet (tip `d14bfa2`) + b6-spanless (tip `3eb6283`) both
  HARVESTED by cherry-pick (`606dc5c`+`bf3b4e3`, fb-11 EMPTY; one combined
  chain, deviation logged in §10). x3n-neutral (ai/r22-x3n) + x3a-attack
  (ai/r22-x3a) possibly IN FLIGHT @ 6657a65: the x-3 Fable crosscheck PAIR on
  the arch-3 diag family (§10 tail entry) — a resuming successor checks those
  branches/results FIRST, banks verdicts PRE-SANITIZED (process-1), and
  presents BOTH passes to the human without collapsing them.
  PROCESS RULE (fb-19): every builder brief carries an explicit
  no-subagents/do-it-yourself clamp; sonnet-tier especially. PROCESS UPDATE:
  conductor gate chains run e2e with DORC_E2E_QUIET=1 (failures print
  verbatim; tally always; fb-17 read-the-output intact).
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

WAVE-2 PREP DONE (2026-06-11, clean-context successor): 22B read in full;
spike/CLAUDE.md gained the round-22 standing rulings + corpus-count fix
(`463c0b0`); 22B fork slugs renamed note-scoped (`cb695a9`); owed-1 explainers
delivered in-chat; resume chain verified green at `e6ea836` (all gates rc=0, e2e
99/99 ×2, typos 0). B3 (arch-3 design half, Opus) HARVESTED + ADJUDICATED — `894109c`+`9c4b621`,
chain green ×2, zero goldens; six tc-flags adjudicated in §10 (incl. the
verified #[non_exhaustive] omission); PROPOSED floor column awaiting the
human's PR-disposal (RenderHeredocRefused=Error+WarnOrDeny; the two
disclosures=Note+None). B4 FAILED (fb-19 recursive self-delegation; abandoned); B4b re-dispatch with
the no-subagents clamp SUCCEEDED and is HARVESTED — arch-3 core COMPLETE
(see arcs). NEXT = ARCH-2 PREP (fresh conductor turn): fold-or-re-derive
x-1's coverage-doc test from ai/r22-xcheck1 `b68fc66`; the arch-3-residual-1
must-emit audit rides along; ALSO: TODO.md carries an UNCOMMITTED human line
(2026-06-11, this worktree) — harness output too noisy at 2× e2e per commit;
candidate fix = a quiet-success knob in e2e/run.sh (failures verbatim + the
tally line; keeps fb-17's no-pipe/real-rc/read-the-output discipline, just
makes the output worth reading) — propose to the human at prep. Then arch-2
build dispatch (emit-at-origin, mvs-1..5, remediation-class render,
arch2-gate-obligation, rec-1 two-surfaces). x-2 (over-suppression) wants fr-2
(VMCAI PDF) first. WAVE-3: arch-2 (emit-at-origin,
mvs-1..5, remediation-class render, + arch2-gate-obligation; fold-or-re-derive
x-1's coverage-doc test at prep) then arch-4-thin (cer-1..6, host-side durables
per ru-21/22/23, traceparent tail — read OTel env-carriers spec first). x-2
(over-suppression) wants fr-2 (VMCAI PDF) first.

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
- arch-3 (catalog retrofit) — **CORE COMPLETE** (spine B3 `894109c`+`9c4b621`;
  sweep B4b `6f4862c`+`0e0a470`): all 23 codes are typed DiagCode variants
  with payloads + registry severity/Floor; allow-list EMPTY; diag::legacy
  deleted; s-2 widening + report() spans (drop-A closed); 3-lane render
  partition; tidy gate live. ZERO golden diffs across both harvests. TAIL
  ITEMS (not blocking arch-2): arch-3-residual-1 must-emit audit (map all 23
  codes to driving tests, fill gaps — at arch-2 prep); arch-3-residual-2
  Diag::new_spanless for the six slug-extraction span-less codes;
  hostsim-Finding fold (core-expressible payloads — core cannot dep hostsim);
  HUMAN PR PASS pending: the PROPOSED floor column (B3's 3 rows + B4b's 20,
  all-None except RenderHeredocRefused=WarnOrDeny) + the b4-cfg-top-severity
  Warning→Error unification (split-into-two-codes is the fallback). Debts:
  tc-cmdsub-siteid two-id-spaces (typed split when site-keyed consumers
  arrive), tc-cmdsub-cause (ProvId wiring lands in arch-2 emit-at-origin).
  RESIDUAL-2 RESOLVED (B6 `bf3b4e3`): spanless mint via private SpanSite +
  `new_spanless_site`, six-code allowlist gated in diag_tidy. residual-1
  (must-emit audit) still open, first item of arch-2 prep.
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
- **owed-1 (224 §11): DELIVERED** (in-chat, wave-2-prep turn, 2026-06-11).
- **housekeeping for wave-2 prep: RESOLVED** (`463c0b0` spike/CLAUDE.md +
  `cb695a9` 22B fork-rename).
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
