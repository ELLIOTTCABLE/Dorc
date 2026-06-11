# 212 — Mid-round rulings, batch 1 (dq-errexit ledger + r1A arrival; 2026-06-10)

> Orchestrator note. The human relayed rulings from a parallel conversation with the
> round-20 agent (two messages, same day), mid-round as the charter anticipated. Human
> voice paraphrased; quoted fragments verbatim. Supersedes the corresponding 20V §8
> statuses. (Slug 212 was reserved for recon digests; repurposed — the recon reports
> were absorbed into build briefs and need no durable copy.)

## dq-errexit-1 (canary the only cost-species?) — stays OPEN; adjudication is evidence-driven

Process ruling (human): dq-1 gets decided against constructed evidence. Candidate
second cost-species must arrive as concrete strawmen / corpus-derived shapes with
reasoning attached — never as assertions offered for sign-off. The
run-evidence/audit-trail candidate (211 §7: a converged mutator run still writes apt
history, auditd trails, atime — elision removes that evidence) stands as the first
ledger entry; the H2SALS dashboard walk should actively collect more.

Standing style rule (human, this round): notes and subagent briefs justify claims by
evidence and artifacts only — no characterizations of people in agent-readable
material.

## dq-errexit-2 (oracle-author owns the bare-middle default?) — LEANING-yes DOWNGRADED to genuinely-open

Human read the lean, understands it, is "not yet sold": oracle-default ownership
"sharpens the cliff between 'my code is kinda book-y' and 'my code is kinda oracle-y'…
The more correctness we hand off to the oracle-author, the more we turn the Oracle
Author into an Important Thing that somebody has to Try Hard To Be." (kSILO stated in
ownership terms.) He acks the other two owners (engine-global; admin-per-book) and
their tradeoffs; this third tradeoff joins the balance. Possible terminal outcome, his
words: "if we can't reduce the pareto frontier here and provide better behaviour
overall, then we've named a new KNOBS entry."

Build consequences (live):
- The precedence seam keeps ALL THREE ownership models genuinely live; nothing built
  may assume oracle-ownership of the bare middle — in mechanism shape, naming, or
  tests.
- Anti-cliff direction to keep measurable: arch-2 + door-1-via-wrappers (rungs
  r-2/r-4) move elision value to "provable from sh anyone can read" rather than
  "declarations only oracle-authors mint." The dashboard reports the
  r-2/r-4-reachable population separately from r-3-only, so the ownership question is
  decided against numbers. (Division of labor, confirmed with the human: r1A produced
  the corpus + census + oracle seeds; arch-6 builds the analyzer-report that RUNS OVER
  them — consumer, not producer; no duplication.)

## dq-errexit-3 / door-4 (guard-insertion trust) — directional RULING + the correlated-failure frame

Ruling (product-grade): door-4 "would have to be a CLI flag or similar, absolutely
for-sure. Some users may prefer the apply phase be a *pure*, direct, elision-only
transform of their code." Trust-boundary taxonomy (human's; adopt as the dq-3 frame):
a bad oracle today can (1) do bad things during probing — very bad; (2) cause extra
work during apply — less bad; but can NEVER (3) "cause new, novel bad things to happen
during apply-phase." Door-4 breaks boundary (3).

Second relay, the sharper frame: door-4 introduces a *stacked, correlated* failure
mode — one bad oracle now produces bad behaviour at plan AND THEN at apply, two
user-experienceable failures from the same known-imperfect truth-source. Plus the
industrial-psychology multiplier: "clearly-correlated negative experiences in quick
succession solidify memory and shape the flavour of that memory hard… the amount it
makes using-a-tool-suck can be outsized in relation to how much it actually measurably
sucked." Hence the hesitation toward any phased separation that rides one imperfect
truth. Door-taxonomy connection: doors 1/3 add zero new correlated surface; door-2
adds declaration-trust but no novel apply-phase actions; door-4 adds both — which is
why it sits behind the flag, last.

Spike disposition CONFIRMED by the human: exploring "exactly this sort of dangerous
state-space in a bounded, throwaway manner" is "perfectly reasonable — ideal, even."
So: the spike builds door-4, LAST (after doors 1–3 value is measured), behind a seam
whose default is `Never` (apply = pure elision-only transform), opt-in modeled as a
CLI flag, the `Never` position provably producing zero transforms, hostile crosscheck
mandatory (four-world trace + a boundary-(3)/correlated-failure attack). The product
hard-defers it regardless of spike results.

Priority reshuffle (stands): arch-6 dashboard lands BEFORE door-4 work — "how much do
the other three doors help out before we risk it" is now an explicitly human-wanted
number. Revised wave tail: arch-1 crosscheck ∥ arch-2 → door-1 cascade cases + arch-2
crosscheck → arch-6 + arch-4 note → arch-5 → arch-3c (flagged, default-off) → stretch.

## r1A arrival (side-quest complete)

`ai/r1A-H2SALS` is complete and committed; worktree live at
`.claude/worktrees/ai-r1A-H2SALS`. Corpus: `Research/corpora/H2SaLS/` (harden.sh,
census/{commands,constructs}.tsv + summary, 11 oracle seeds, README) +
`Research/corpora/tools/census*.sh`.

Orchestrator decisions:
- NO merge into ai/spike3 now: the dashboard's adapter seam reads the sibling worktree
  path read-only, which buys everything a merge would while risking nothing
  (gate-interaction risk on a merged tree is nonzero, reversibility favors waiting).
  If the dashboard later needs committed fixtures, selectively copy the specific files
  with attribution — not a branch merge.
- Quality caveat (human-flagged): the work was handed down to a lower-capability agent
  partway through (the remainder believed mostly-mechanical). The dashboard must
  VERIFY census numbers against the corpus itself (spot-check command extraction,
  counts) before weighting anything by them.
- Reader grounding: the corpus is a plain-POSIX-sh rewrite of a public, defensive
  server-hardening guide — the project's legitimate target material. Brief any agent
  reading it with that context up front, and prefer the 1AE summary note as the entry
  point.

## Meta (for round-close seeding feedback)

The relayed-rulings channel worked well; the one ambiguity in batch-1a (spike-builds-
door-4 vs not) was resolved by the second relay same-day. Logged for the fb-* list: a
one-line spike/product disposition marker in relays removes the ambiguity class
entirely.
