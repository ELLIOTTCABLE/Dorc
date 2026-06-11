# 1AE — round-1A close-out: synthesis, seeding feedback, human-surface items

> **Disclosure:** LLM-generated (round-1A closing agent). The round's corpus
> (`Research/corpora/H2SaLS/`) is an intentionally quality-varied ARTIFICIAL N-of-1
> testing corpus (1A1 rul-1A-llm-disclosure): frozen evidence, never executed
> (`dash -n` only), and unable to expose the truth of real-world ops-code. Subject of
> all analysis = THE ANALYZER. Append-only.

## §1 Round synthesis

All four deliverables shipped, the last crosschecked-and-finalized this session:

- **D1** `harden.sh` (696 lines, dash-clean; three personas at varied defensiveness;
  its own crosscheck found+fixed two systematic mapping-error classes — 1A2–1A6).
- **D2** `census.sh` + fixture/selftest + the H2SaLS census output (1A7).
- **D3** eleven oracle seeds spanning the core families and the file-state spectrum
  (1A8/1A9), one provider-collision adjudicated (tc-F2: crond's resolver inert).
- **D4** the capability matrix `Research/plans/1AA-capability-matrix.md` — now FINAL:
  built by a fresh compiler-framed subagent (1AC), audited by a hostile clean-context
  pair, reconciled in 1AD, corrections folded.

What the matrix says, post-crosscheck (the round's product, N-of-1 caveats standing):

- **The engine's gap on this corpus is in the effect/identity planes, not the
  grammar** (head-1, twice-audited): the only fired ⊤-trigger is `Loop` (two
  `continue`s); what floors the book is no-oracle Opaques, cmdsub-⊤, detached
  functions, and invisible redirect-writes.
- **The poison economy dominates the elision economy** (head-2): one Opaque ⊤-poisons
  all downstream ambience and one ⊤-region havocs all values; this book triggers both
  early — at L38 (the wall's true origin, 1AD brk-1) and L572. Sequencing beats
  per-row difficulty: cheap poison levers (probe-ful pkgindex + an id companion) →
  redirect cells (y-1) → inlining (y-2, welded after y-1 per tc-M2) → havoc-narrowing
  (y-7, welded with/after y-2 per 1AD nit-3adv) → run-delta last (y-5).
- **Refusals are sound but not free** (head-3): every honest oracle refusal lands
  Opaque and poisons; a sanctioned poison-stop-without-license cell shape does not
  exist at HEAD (1AD conv-2) and is the round's clearest new machinery candidate.
- The top two yikes rows — **invisible redirect-writes** (Redir⇒Pure + printf
  blessed-pure) and **detached helper functions** (24 Opaque calls + call-transparent
  value plane ⇒ latent wrong-concretes) — survived both hostile passes unmoved.

Crosscheck disposition in one line: the citation mass, all 19 `an-*` statuses, census
numbers, and ranks 1–2 held; what fell was y-3's geography/"first domino" rationale,
the declared-effect-no-probe remediation story, tc-F2's claimed liveness, and four
attributions — every substantive break traceable to the matrix author's self-disclosed
unread zone (or one same-line oversight). The 1AC disclosure discipline predicted the
fault-lines; the charter's hostile-pair requirement earned its cost (1AD §5).

## §2 Seeding feedback (charter-requested; 1AB §5 carried, two added)

- **fb-1 (the window — the big one; carried).** A round whose deliverable is an N-of-1
  probe of a *hardening* runbook runs the orchestrator's D4 synthesis into the
  security window. The mitigation that worked all round (matrix build AND both
  crosscheck audits AND this close-out step): dispatch security-domain-adjacent
  synthesis to FRESH subagents briefed with the ENGINE as subject and the workload's
  domain explicitly out of scope; keep the driver on dispatch/reconcile/commit. Seed
  it as a one-liner: "D4-style synthesis is window-prone; plan to dispatch it to clean
  compiler-framed subagents."
- **fb-2 (carried).** State at seed time that the runbook's natural framing ("what
  each command secures") is the trip-wire and the analyzer framing ("how the engine
  models this sh shape") is the safe isomorph — same matrix, two framings, one clears.
- **fb-3 (carried).** The clean-context multi-author corpus build (dec-5) validated:
  the D1 crosscheck pair independently re-derived nearly every builder-flagged
  divergence. Keep persona-variance + clean-context discipline.
- **fb-4 (carried).** Future rewrite briefs must carry the one-snippet-rc rule
  ("Ansible `shell:` = last-command-rc; render tolerated failure as `|| true`, never
  bare under `set -e`") — removes 1A6 §1's worst error class at the source.
- **fb-5 (carried, still live).** Exclude `.claude/worktrees/` from SyncThing or
  expect conflict-file resurrection (round-20 flag; a stale synced copy of this
  worktree path existed on another device).
- **fb-6 (NEW — inherit the hedge).** Every substantive matrix break (1AD brk-1,
  conv-1, conv-2) was a +SURE-marked claim whose derivation passed through a
  disclosed-as-unread source (1AC §1's w-*) — the disclosure was honest but the
  downstream claims didn't inherit it. Seed a rule: a claim derived from a
  taken-on-word fact is capped at ~SUSPECT until the source is read, however confident
  the chain feels. (Cheap to state; the matrix would have self-flagged its three
  weakest spots.)
- **fb-7 (NEW — reconcile by source, not by vote).** The crosscheck pair produced one
  direct conflict (the in-loop floor) where the neutral reading was literally correct
  and wrong by omission (1AD §4); convergence-counting alone would have mis-resolved
  it, and three single-audit findings (incl. the headline brk-1) needed independent
  re-verification before applying. Keep "reconciler re-verifies every applied
  correction against the cited source" as protocol — it changed the outcome three
  times this round.

## §3 Items to surface to the human (flag, NOT fix; consolidated from 1AB §6 + 1AD §6)

- **h-1 (engine-owner call): tc-M1 self-reach strictness.** `self_reach_holds`
  demands an EMPTY suppressed in-state (effect.rs:478-483) — ANY upstream establish
  book-wide refuses every Members license. Corpus evidence: unreachable under any
  realistic preamble. Per-family-cell foreign-writer test vs global pristine is the
  open question.
- **h-2 (dialect ruling): read-providers.** tc-F2/tc-F3/tc-M3 + 1AD f-1AD-2 as one
  family: should read-builtins (`test`/`[`/`grep`/`cmp`) be provider-keyable at all;
  first-resolves-wins once two claimants exist; and the kind-default fallback's
  silent-wrong-probe hazard when a one-selector kind gains a second cell. Note
  tc-M3's premise is corrected by 1AD conv-1 (NOT live at HEAD — the blessed-pure
  gate; prospective once grep/cmp providers collide).
- **h-3 (registry candidates, no `an-*` rows today):** the poison-stop-without-
  license cell shape (1AD f-1AD-1 — head-3's economics want it; um-pkg-3 is its
  upstream prescription) + tc-M4's three (heredoc-body expansion, `$(cat <<'EOF')`
  folding, static-heredoc-table loop enumeration).
- **h-4 (welds, sequencing):** tc-M2 (redirect cells before/with inlining) and 1AD
  nit-3adv (havoc-narrowing with/after inlining) — both are fix-ORDER constraints,
  not fixes.
- **h-5 (KNOBS candidates — human-auth-only, not edited):** 1A9's file-state spectrum
  verdict bears on `kSILO`/`kBURDEN`; um-file-restart-1 makes TODO's 2026-06-08
  `run-delta` entry concrete against a real corpus shape.
- **h-6 (trivial engine-side comment fix, out of round scope):** cfg.rs:140-145 +
  198-202 still call the member-elision lift future; plan/lib.rs:176-183 landed it
  (1AD f-1AD-3).
- **h-7 (N-of-1 guard, carried verbatim):** the matrix's "generality" column is
  gut-feel, not evidence; B8's zeros (no braced-operator params) will NOT generalize.
  Do not let the matrix steer roadmap beyond "the shapes ONE real runbook needed."
- **h-8 (round-21 pickup):** re-derive elision rates only AFTER the cheap poison
  levers + y-1 land (1AA §3's guard); the matrix is the capability baseline to diff
  against, not a target list.

## §4 Final state

Branch `ai/r1A-H2SALS`, never pushed. Chain (this session's tail): `7299362` 1AD
reconciliation → `dc5c715` 1AA finalized → this note. Durable deliverables:
`Research/corpora/H2SaLS/` (harden.sh, census/, oracles/ — frozen evidence),
`Research/corpora/tools/census.sh`, `Research/plans/1AA-capability-matrix.md` (FINAL),
notes 1A1–1AE. Reference material + build intermediates remain outside the synced tree
(`%TEMP%/dorc-1A-sources`, `%TEMP%/dorc-1A-build`; re-fetch is mechanical from the
corpus README pins). Nothing was ever executed; `dash -n` remains the only validation
ever applied to corpus material.
