# 23J — adjudication of the direction-review pair (23Ia neutral · 23Ib adversarial)

Conductor adjudication, 2026-07-02, under the standing calibration (convergence = signal;
adversarial-only = suspect-until-checked; demonstrated > argued; never credulous toward
hostility — nor toward praise). Sources harvested: `notes/23Ia-directional-review.md`,
`notes/23Ib-pivot-audit-worklog.md`. AI process-evidence, never a correctness claim.

## The verdict, cross-compared

Both passes, from opposite stances, land the same headline: **the pivot itself is sound —
the crisis was real and self-caught, the ternary escape is genuine (not a rename), the trade
was priced in the open, and the tempting paper-overs were rejected on the record.** The
adversarial pass's own kill-list is the strongest evidence: it tried "the guard is unsound,"
"the vouch launders global claims," "the attention-loss was hidden," and withdrew each with
reasons. Where BOTH passes bite is the same seam: **the newly-rewritten root-level docs and
USER_STORY narrate ahead of the pins/build** — end-state design presented as present
mechanics, plus a handful of overclaims. That convergent seam is the actionable product of
this crosscheck.

## Convergences (credited at full weight)

- **conv-userstory-hurt** (23Ia-fd4b ≡ 23Ib-fd5, same sentence): stage-2's "installing a
  library could NOT hurt anything" extends claim-monotonicity (welded, true) into
  code-monotonicity (false — third-party checks execute in probe AND now guard position;
  mutation/crash/slowness all hurt). Fix: one conditional clause, human's hand (root doc).
- **conv-root-residue** (23Ia-fd6 ≈ 23Ib-fd1, overlapping cluster): the a92ad31 rewrite left
  drift standing in DESIGN — (a) "an operation with no oracle to declare its
  global-data-dependencies… can never elide **or rearrange**": the declare-and-we-elide
  implicature is the refuted authored-completeness mechanism (233 corr-3, 237 conv-2, 238
  claim-4; the surviving license is derived footprints, future-tier), and "rearrange"
  contradicts the round-20 order-sacred ruling; (b) "oracles should always strive to be a
  functional no-op" is false as written under vouch-licensed guards (a sanctioned behaviour
  delta); (c) walls described as coming only from *unmodeled* commands, where the welded
  design walls the elide-tier below any *retained* command. All human-doc fixes.
- **conv-rc-soundness** (23Ia-fd1 ≡ 23Ib-fd3 residue, facet-convergence — the pair's most
  valuable product): the guard consumes the predict body's AGGREGATE IN-BAND rc, while the
  design's claims ride per-mark, engine-interpreted semantics. Three facets, one gap:
  (i) the `!`-inverted claim's inversion lives in the engine's interpretation and is
  STRIPPED from the shipped bytes — a vouched inverted arm would mint a guard that skips
  precisely when the world drifted to needing the command (conductor re-traced the
  mechanism: +SURE; today masked ONLY by the cov-q4 transitional freeze, and the freeze's
  own dissolution note, followed naively, mints it); (ii) corpus predict bodies rc-0 their
  refuse paths (hz-refusepath / 23F adv-3) — runtime host-conditional branches can land on
  unvouched rc-0 paths and silently suppress a mutator; (iii) fact-reporting bodies
  (233's own systemctl exemplar) are rc-vacuous, so the check-shape the ELIDE-half wants is
  structurally guard-unsound — a genuine two-halves tension the doctrine says must be
  flagged. DISPOSITION: promote **rc-soundness** (aggregate-rc-0 ⇔ the vouched establish-set
  holds on the runtime-reached path) to a NAMED structural component of the guard-witness —
  refuse-to-guard, loudly, where it can't be established — owned by the vouch-spelling
  round's charter; plus two new xfail pins (vouched-inverted-arm never-mints-backwards;
  refuse-path-rc0 never-passes-a-guard).
- **conv-process** (both verdicts): the round's discipline (crosschecks, discount ledgers,
  honest STOPs, xfail-first) held under hostile reading. Recorded; never-vouch applies.

## Adversarial-only, VERIFIED (conductor-checked before credit)

- **23Ib-fd10 — the crisis's own unsoundness is design-closed, pin-open, repair-unscheduled.**
  Verified verbatim: `23A` hz-ambient-hole says the P-pair goldens ENCODE the 233 §0 ambient
  elision at HEAD ("documented, not endorsed"); no xfail anywhere pins
  modeled-will-run-walls-downstream; the fix rides the elide-half arc ("when the human wants
  it"). The sequencing has a real rationale (naive poison-everything zeroes the elide tier;
  guards are the prerequisite that makes the fix AFFORDABLE) — but signed-law-without-a-pin
  is exactly what this project's method exists to prevent. DISPOSITION: author the missing
  xfail (modeled-diverged mutator walls a downstream converged site) in the next repair
  pass; add an explicit repair-scheduling line to LIVING_STATUS arc-4.
- **23Ib-fd2 — USER_STORY narrates guard-minting under `set -eu` as settled while the ruling
  is open-with-suspected-breakage-either-way** (23D §3 defers errexit-implicit; 23A
  np-errexit deliberately authors no `set -e` book — verified). The guard anatomy's
  "||-left is errexit-exempt" sentence is settled sh fact; the MINTING question is what's
  open, and the story carries no caveat. Fix: one caveat clause (or drop `set -eu` from the
  book) — human's hand; the conductor drafted the sentence and flags its own authorship of
  the conflation.
- **23Ib-fd4 — ANALYZER-NEEDS.md stale across five rounds** incl. rows contradicting the
  polarity-retirement ruling. Already tracked (arc-2); bumped to the build-slice's entry
  gate rather than during.

## Neutral-only, VERIFIED

- **23Ia-fd2 — the flagship XFAIL golden went stale through the XFAIL blind spot**: verified
  by git — `9a3faef` (R4a) rewrote the flagship's fixture oracle after the pin round
  (`d8081fc`); xfail content-diffs are inactive, so the hand-authored desired-golden's
  pinned preamble no longer equals strip(new-body). The exact conv-1 blindness, instantiated
  in the window between pin and closeout. DISPOSITION: hand-re-derive the flagship (and
  pair-b) expected-preamble from the converted oracle body in the next repair pass; consider
  a preamble≡strip(body) floor while there.
- **23Ia-fd3 — 231's walk-back map orphaned without triage** (clusters 1a/1b/1d/1f, three
  tc-flags, 7 unseeded-hunt candidates unaccounted). DISPOSITION: a disposal paragraph in
  `23D` — small dedicated task; some drops are likely defensible, and saying so is the fix.
- **23Ia-fd5 — lane-PRIVILEGE unnamed** (sudo/become: a vouch promotes oracle code into the
  book's elevated context; the re-weld rationale answers drift, not privilege).
  DISPOSITION: seam-row in the ledger + ANALYZER-NEEDS row when refreshed; design lands with
  the orchestrator/become work, not now.
- **23Ia-fd4a — USER_STORY's stage-1 "not even show those lines" paragraph** (human-authored)
  sits in tension with 23D §2's whole-book render ruling and the doc's own header. NOT
  adjudicable by the conductor — this is the human's word against the human's weld; routed
  as an ask (below). Note rul-attention-honesty's LETTER protects only will-execute lines,
  so a view-mode collapse of elided lines is plausibly sanctionable — but then 23D §2's
  absolutism and the USER_STORY header want amending, not ignoring.

## Discounted / de-biased (with reasons)

- 23Ib-fd9 ("a mandated no-hedging zone") — largely REFUTED: the two-halves language rule
  bans tier-DEMOTION of the elision goal ("someday, hopefully"), not honesty about walls —
  the corpus demonstrably hedges where warranted (USER_STORY's "forever", the residue
  section). Kept: nothing. The doctrine reads fine as written.
- 23Ib-fd8 (guard-half value-case open) — mostly ALREADY-ON-RECORD (conv-4 check-tax
  honesty; folding consciously banned; banding parked). New information ≈ zero; useful only
  as a reminder that the placement-spectrum round (task #11) carries the wall-density
  cost-model as its FIRST item.
- 23Ib-fd7 (hatch sequencing) — accurate and consciously parked with a named un-park
  trigger (task #10); reported as sequencing-risk, which is where it already lives.
- 23Ib doc-drift nit "DESIGN reintroduces 'skip' against AGENTS' ban" — the reviewers
  cannot know the human LICENSED the DESIGN-only umbrella in-conversation (2026-07-02); the
  finding is correct against the WRITTEN corpus, which means the license needs a durable
  home (the AGENTS.md carve-out line) — routed as an ask, and a nice demonstration of
  silence-is-not-ack biting in reverse.
- 23Ia-fd7 (measurement-route dangling) + 23Ib nit (093 annotation owed) — already on the
  loose-ends list; no new action.

## Routing

REPAIR PASS (agent, after the R3/P5 closeout lands; one brief): the two rc-soundness pins ·
the fd10 modeled-wall xfail · flagship/pair-b preamble re-derivation (+ optional
preamble≡strip floor) · the 231 disposal paragraph (reads 231, maps clusters to
survivor/dead-with-reason). CHARTER ADDITIONS: rc-soundness → the vouch-spelling round;
wall-density cost-model already first in task #11. HUMAN'S HAND (root docs): the
conv-userstory-hurt clause · the set-eu caveat (or drop it) · conv-root-residue (a)(b)(c) ·
the AGENTS skip-carve-out line · the fd4a render-view ruling · optionally stage-4's bare
`"$2"` → `${2:-}` and the first-taught-elision swap (23Ia-fd4c/d). LEDGER: lane-privilege
seam-row (this note is its record until the refresh).
