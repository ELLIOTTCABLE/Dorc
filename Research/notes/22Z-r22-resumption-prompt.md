# 22Z — round-22 resumption prompt (fb-12 skeleton; keep CURRENT)

> Cold-conductor onboarding document. If you are reading this to resume round-22
> after a conductor death: this file + notes/224 (the round ledger) + TaskList are
> your state. Updated at: PHASE-R gathering complete, pre-digestion (2026-06-11).

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
   known stale: says 43-case corpus; it is 98 dirs at HEAD — count the dirs).
4. plans/21W (round-21 close) + plans/21Z (LIVING inventory). 5. The research base:
   plans/111 + notes/220 + notes/222 §5-§7. 6. THIS ROUND: notes/224 (GATE-1 record,
   rulings ledger §7, dispatch ledger §10) then the five PHASE-R notes 225-229.

## Verified state (at last update)

- HEAD lineage: `ada085d` (clean inherited base, full chain verified green) →
  `fa78935` notes/224 → research-note commits (225-229, interleaved) → `5da879c` +
  `5b58c5f` (W1's two e2e fixtures, cherry-pick harvest) → ledger commits.
- Gate chain at harvest: build/fmt/clippy/deny rc=0 · cargo test 463/0/1-ignore
  (20 suites) · `sh e2e/run.sh` ×2 = 98/98 SEVEN gates, real exits · typos rc=0.
  Canonical chain (fb-17): build FIRST, never pipe a gate, read e2e output, ×2.
- Corpus: 98 e2e cases (was 96; + door1-door3-dead-block-folds,
  y1-var-resolved-target-invalidates-query).
- Builder worktree %TEMP%\dorc-r22\w1-fixtures (branch ai/r22-fixtures) drained by
  cherry-pick, fb-11 content-diff verified EMPTY; originals left for the human.
- `.claude/research/` is GITIGNORED scratch from a superseded mechanism; four
  force-added commits hold parts of it; fold-check then `git rm -r` at digestion.
- SyncThing: whole Sync folder disabled on this PC (2026-06-11); ghost-husk risk
  paused; conflict cleanup is human-owned.

## Queue (mirror of TaskList at update time)

DONE since v1: digestion (all five notes full-read by conductor; honesty flags in
22A §8) · plans/22A synthesis COMMITTED (`61fa8e4` — 13 conclusions, arc re-scopes,
gate2-ask-1..5) · tracked .claude/research scratch REMOVED (`d4277e0`; ~3.2MB
untracked residue on disk left for the human, incl. rqA staging PDFs) · W1 fixtures
HARVESTED (corpus 98, chain green). NOW: GATE-2 presented in-chat, WAITING on human
adjudication of 22A §1 (re-scoped arcs) + §2 (gate2-ask-1..5). After go: arch-1
first (the gate lands with the arena's first commit), arch-3's s-2 widening early,
arch-2, arch-4-thin; crosscheck budget ~25-30%, first targets x-1..x-3 (22A §1).
#7 keep this file current.

## GATE state

GATE-1 PASSED (synthesis + rulings ru-1..ru-12, notes/224 §7). All three GATE-1
asks CLOSED: ask-zero-influence → ru-11 WELD (receipts fully one-way; no influence
on allow OR reject, ever; triggers re-derive as facts; gate equality permanent).
ask-partition + ask-comments → ru-12 (identity plane byte-exact vs exempt plane
with named closed-enum reasons; gate asserts IDENTITY-EXACT under strip+variance;
shipped .sh artifacts byte-identical INCLUDING comments = minimum floor; no
unstable tracing into default-mode artifacts; receipt-derived rendering lives
out-of-artifact; sidecar ID+`.sh.log` shape = disliked last resort). Open with the
human: fetch-requests fr-1 CACM "Debugging in the (Very) Large" (403-walled);
fr-2 VMCAI'12 sound-alarm-clustering PDF (no text layer) — load-bearing for rq-E,
capped ~SUSPECT.

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
.claude/research). Standing round-21 rulings in spike/CLAUDE.md hold (mutation-
analysis impossible; TOCTOU WONTFIX; no intra-host reordering; rc opaque; "skip"
banned; identity declared-never-inferred).

## Expected GATE-2 reshapes (conductor's working view, NOT ratified)

- arch-1 (ProvId arena + Top(cause) + erasability gate): strengthen per R4' —
  per-field Exempt-enum partition, adversarial variance run-B, coverage canary,
  UnordMap-style iteration-suppressed newtype (kills the f-2 Eq/Ord hazard at
  compile time); Top(a)≡Top(b) in the lattice stays the contract line.
- arch-2 (one consumer end-to-end): emit-at-origin reframe (R3': mint cause at
  ⊤-creation, suppress at render; never emit-N-then-dedup); minimum suppression
  set = R3's five rules; render ranked by remediation-class (ru-6).
- arch-3 (catalog retrofit): exhaustive-enum catalog spine + tidy-style grep for
  reachability (R1'); severity model needs an un-overridable floor tier; `expect`-
  style positive must-emit assertions; gate completeness/registration, NOT prose
  (fluent-regret lesson); hostsim Finding likely IN (ru-5).
- arch-4 (durable + why): reshape per d-1 split — dump format + why lens stand
  (Buck2 precedent; thin-durable + DST recompute); golden-trace PINNING demoted
  (plan-forcing regret; never trace-only if any).
- arch-5 (OTel seam): traceparent hand-emit on the verdict lane is sanctioned
  import-the-value-format; env-carriers spec (Beta) UNREAD — read before deciding.
- Warm-ups DONE (the two fixtures). Candidate third: 215 §5's labeled d×d cell
  (outer-live × inner-diverged-runs) — unauthored, decide at GATE-2.

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

flag-dxd-third-cell (above). flag-zm-attribution (225): vp-26's slogan is the
Sabelfeld–Sands gloss, NOT Zdancewic–Myers verbatim (ZM Thm 4.2 self-corrected,
"not tight") — synthesis must cite as engineering-precedent. flag-untracked-query-
information (229): rustc's lint is a direct gate analogue — candidate dylint-shaped
enforcement, machinery cost unknown. Carried round-21 flags live in 21W §6 (span-
bridge tier-2/3 recommended-together when plan is next touched; seam-1 ⊤-readout;
find-J reader-liveness = human's, parked; doors program parked behind 218/218a).
The 207/YOLO escape-hatch stays set aside. kSTATE parked (receipts per-run; the
d-1 dump is a write-only LOG, not read-back state). Capture-eagerness knob =
human's call once costs are real.

## Meta-goal

The project is partly the human's proof-to-himself that LLMs can do real, complex
engineering; the highest-level deliverable is demonstrated capability. Note where
better seeding would have prevented struggle; surface at round-close. Dispatch
heuristics: split by decision-surface, not size; pre-spelled contracts make big
builds mechanical; hostile crosschecks = highest value-per-token (~25-30% of build
spend); builders write their own hunt-lists, crosschecks told to exceed them.
