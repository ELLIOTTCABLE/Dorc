# Dorc source-claim adversarial audit — main adjudication loop (clean-context entry prompt)

Paste this into a **fresh context window** to run one phase of the audit. You are the top-level
adjudication-loop agent: you process one batch of sources, spawn adversarial subagent pairs, arbitrate
their output on evidence, and write all durable state to disk. When a phase ends or pauses for human
review, you re-emit the instruction to restart from this file in a fresh context. State lives on disk,
never in context — so each phase starts clean and cheap.

## Two disciplines you hold and the subagents must NEVER see
- **caveat-COST** — this is expensive *and* invasive. Annotating a committed claim is history-adjacent; never done lightly.
- **caveat-NOTRUTH** — this process cannot find truth. A subagent (or you) becoming sure "this was badly wrong!"
  is *as* poisonable as the original author's "this is so right!". Pursue only egregious, human-adjudicable findings;
  log the rest and move on. **"I couldn't read it" must NEVER become "it doesn't support the claim."**
  Restate both at pin / arbitrate / escalate. Putting either into a subagent prompt blunts its stance — don't.

## Read these first (all state is on disk)
1. `Research/_scratch/source-audit-register.md` — THE state file: danger-ranked queue, per-source `status:`,
   the deterministic `RESOLVED-DET` dispositions (skip those atoms — already settled), tiers, gap-access/OCR set,
   the handoff reconciliation. Read it in full; it tells you where the last phase stopped.
2. `_papers-handoff.md` (repo root) — existing quality-grades for 36 legacy papers. Grade for NON-audited sources;
   you SUPERSEDE it for audited ones.
3. Source text, resolved **location-agnostically**: try `Research/sources/<key>.txt`, then `Research/papers/<stem>.txt`,
   then the `.pdf` via the Read tool. (`papers/` may or may not have migrated to `sources/` — handle either.)
4. Claim-sites: `grep -rn "<key-or-surname>" Research/plans/`. **Plans are the audit target.** Use `notes/` only for
   downstream tracing, never as the source of what a claim "should" say (notes can poison).

Do NOT load project design docs (AGENTS/KNOBS/DESIGN) into any subagent — clean context is the mechanism.

## One phase = one batch (do exactly one, then stop)
1. Open the register. Take the next **≤6 PENDING** sources, top-down by danger-rank (tier-A before tier-B). Skip any
   source whose every atom is already `RESOLVED-DET`.
2. For each: `pin → deterministic-precheck → dispatch → arbitrate → log`.
3. If any source yields an **EGREGIOUS** finding: stop the phase *now*, write state, surface it to the human (below),
   re-emit the restart instruction.
4. At batch end — or the moment you sense the token budget running low — write state, re-emit the restart
   instruction, and STOP. Never start a second batch in one context.

### pin (you; restate caveats)
`grep` the source's claim-sites in `plans/`. Quote each asserting sentence verbatim; decompose into atomic
sub-propositions; drop atoms already `RESOLVED-DET`; tag each remaining atom **factual-attributable** (number /
named mechanism / "paper X did/proved Y") vs **interpretive-analogical** ("maps to" / "analog" / "lens") — only
factual atoms get full adjudication power; note inherited `+SURE` / `human-corrected`. If the source is
gap-access / OCR-blocked / unreadable → set `status: GAP`, do not dispatch, do not infer.

### deterministic-precheck (you; no LLM — shrink the queue for free)
For each factual atom, `grep -ai` its attributed tokens (quoted terms, named techniques, numbers) in the source text.
If a token is **purely lexical** ("the source uses/names/contains X") and present **verbatim**, mark that atom
`RESOLVED-DET` in the register with the matching quote and DO NOT dispatch for it. Reversed conservativity: eliminate
only when misalignment is essentially impossible; absence never proves a finding (only *keeps* the atom);
`pdftotext` line-splitting can lose a multi-word match — re-check whitespace-normalized before trusting an ABSENT.
Only atoms that survive this precheck go to subagents.

### dispatch (two subagents in parallel via the Agent tool — clean context, NO Dorc framing, NO caveats, NO meta)
Give each ONLY: the source path (+ "read it in full"), its framed atom(s), the output contract.

**NEUTRAL:**
> Read the source at `<path>`. For each statement, decide whether the source supports it and return verbatim
> quote(s) with locators that bear on it — or "no bearing text found". Statements: `<atoms>`.
> Then grade the source for a citation DB: `grade` (A peer-reviewed/primary-authoritative · B practitioner/secondary ·
> C low-provenance), one-line `grading-reasoning`, `relevance-description` (to "static effect-analysis of shell /
> ops-orchestration"), `grading-certainty` + `relevance-certainty` each ∈ {`+1:SURE`,`-0:SUSPECT`,`-1:GUESS`,`-2:WONDER`},
> a resolving `url` if you can confirm one, and `published` (YYYY[-MM[-DD]]). Return one JSON object:
> `{verdicts:[{atom, supported:"yes"|"partial"|"no", quotes:[...]}], grade, grading-reasoning, relevance-description, grading-certainty, relevance-certainty, url, published}`.

**ADVERSARIAL** (disown the artifact + own the doubt, per the `adversarial-crosscheck` skill; carry its anti-fabrication guard):
> A colleague produced the claims below and attributed them to the source at `<path>`; I distrust the attribution and
> suspect they overstated or misread it. Read the source in full and find where each attribution breaks down — wrong
> term, wrong quantity, a different subject, an inverted conclusion. Quote the text. **But say so plainly where an
> attribution genuinely holds; do not manufacture faults.** Claims: `<atoms>`.
> Then grade the source [same grading contract + JSON shape as above].

### arbitrate (you; restate caveats)
Decide on the **quotes**, not the agents' rhetoric. **EGREGIOUS** only when there's a quotable mismatch a human verifies
in seconds: wrong number · different subject · inverted conclusion · attribution to a work that lacks it. Interpretive /
mild / divergent → **MILD**, log only. A fault only the adversarial pass raised, uncorroborated → read the disputed
quotes yourself; escalate only if you independently confirm it. Finalize the grade by reconciling the two gradings
(usually agree; on conflict take the more conservative); this SUPERSEDES the handoff grade for this source.

### log (always; on disk; you write ONLY under `Research/_scratch/`)
- Update the source's `status:` in the register (`PASS` / `MILD` / `EGREGIOUS` / `GAP`) with the deciding quotes.
- Append to `Research/_scratch/grading-staging.jsonl` one line: the final slug `"<grade>-<stem>"` →
  `{url, grading-certainty, grading-reasoning, relevance-certainty, relevance-description, graded-by:"top-level-agent", published, via:"source-claim-audit"}`
  (the exact `_adopt-source.sh` stdin schema; the deferred migration consumes this — you do NOT run the migration).

## On an EGREGIOUS finding (human-gated; you stop here)
Write it to the register, then present to the human: the atom · the plan claim-site (`file:line`) · the source quote
that contradicts it · a **trace** of propagation (`grep` the source across all `Research/` + `DESIGN.md`/`KNOBS.md`/`TODO.md`;
name any knob / wall / verdict the broken atom feeds; flag later-round reliance). Propose — do NOT apply — an additive,
**ahistorical** annotation placed local to the claim. Substantive edits and deletions are the human's alone.

## Hard constraints
- WRITE ONLY under `Research/_scratch/` (the register + `grading-staging.jsonl`). Never edit the live `sources.json`,
  `plans/`, `notes/`, `DESIGN.md`/`KNOBS.md`/`TODO.md`, and never move `papers/` files. The corpus is concurrently
  edited and human-owned; the migration is staged, not run. No git operations.
- ≤6 sources per phase (fewer if claim-sites are dense). One phase per context.

## Restart instruction (re-emit this verbatim when a phase ends or pauses)
> **Phase complete / paused for review.** All state is on disk in `Research/_scratch/source-audit-register.md`
> (+ `grading-staging.jsonl`). I edited no corpus file. To continue: open a **fresh context window** and paste
> `Research/_scratch/adjudication-loop-prompt.md` again — it will resume from the register's next PENDING sources.
