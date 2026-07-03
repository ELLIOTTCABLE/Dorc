# 23Ib — adversarial audit of the round-23 pivot (worklog)

> Commissioned external-skeptic pass over the r23 redirection (ternary verdict {elide, guard,
> run}); brief: find what the pivot silently gave up / papered over / dropped; self-kill every
> finding before presenting; state plainly where the work holds. Excluded by brief: corpus/
> market-fit (hard fence), spike-code-specific bugs, TBD-plans holes. This file: working notes,
> confidence-marked. Findings graded live; `23Ib-fdN` = candidate finding, `23Ib-wdN` =
> withdrawn-after-check.

## Read so far
README, DESIGN, IMPLEMENTATION, TODO, KNOBS, ANALYZER-NEEDS, USER_STORY, AGENTS,
Research/README, LIVING_STATUS, plans/233 (incl. end-annotation), spike/CLAUDE.md (all rulings).
Git: a92ad31 (human root-doc absorption, full diff), e57aa62/f333db7/88f03ed/8101df6 stats.

## Withdrawn so far (checked, died)
- 23Ib-wd1 (vouch-laundering through un-walling): worried foobar's vouch transitively licenses
  systemctl's elide one step removed. DIED: systemctl's elide depends on foobar *actually not
  running* (plan-guaranteed), not on foobar's vouch being *right*; a wrong vouch's blast radius
  stays at foobar's own site even through the un-walling chain. The 233 fence holds. +SURE.
- 23Ib-wd2 (guard breaks consumed observables): worried `check || cmd` corrupts `$(cmd)` /
  consumed-rc sites. DIED: 233 downsides enumerate refusal classes (consumed-stdout/cmdsub, rc
  consumed by admin control-flow, run-delta verbs, loops/multi-operand); e2e pins exist
  (redirect refuse-home XFAIL). Gate exists by design. +SURE it's designed; build pending.
- 23Ib-wd3 (book-order pin as pivot casualty): "guards pin apply to book-order" looked like the
  pivot killing within-host apply parallelism (an-schedule-dag etc). DIED as a *pivot* finding:
  the pin is a ROUND-20 standing ruling ("No intra-host apply parallelization or reordering,
  ever"), pre-crisis; guards merely ride it. Residue: ANALYZER-NEEDS §I staleness + DESIGN.md
  retained "elide or rearrange" wording — doc-drift nit, not a giveaway. +SURE.
- 23Ib-wd4 (rul-divergence-proceed vs AGENTS fail-fast): "proceed-and-flag, no abort ever"
  looked like it contradicts "absolutely fail-fast once state is unknown". MOSTLY DIED:
  within identified-cause scope, in-sequence guards are stale-free by construction and elided
  lines sit above the first will-run site; the incoherent case (mid-apply evidence that the
  probe snapshot is globally stale ⇒ elisions suspect) is *unattributed drift* = the standing
  round-20 TOCTOU WONTFIX, re-scoped consciously by 239 delta-2 (identified-cause vs open-world).
  Residue: AGENTS.md fail-fast bullet not updated to cite the reconciliation — doc-drift line.
  ~SUSPECT residue only.
- 23Ib-wd5 (attention-honesty vs hiding elided lines): rul-attention-honesty "whole book, never
  hide" vs USER_STORY "not even show those first [elided] lines". DIED: the protective clause
  covers *lines that will execute*; elided lines are the product. rec-1 two-surfaces separates
  artifact (whole book) from render. Consistent.

## Live candidates (to verify further)
- 23Ib-fd1 (DESIGN.md keeps the dead promise): DESIGN.md (rewritten a92ad31, same day as the
  closure) still says an operation with no oracle "to declare its global-data-dependencies" can
  never be elided/rearranged — implying declaration buys elision past a running command — and
  defines poison-walls as coming from *unmodeled* commands. The welded design refutes both:
  authored completeness-vouches ruled unwritable (233 end-annotation corr-3; 237 conv-2; 238
  claim-4); vouch never enters fact-plane (rul-guard-license); ANY will-run command walls the
  elide-tier below it, modeled or not (USER_STORY drifted-day: modeled foobar walls systemctl);
  recovery = derived-at-probe-time footprints, a future ~SUSPECT arc (238), not authorship.
  Top-authority doc materially over-promises the thing the crisis just killed. ~SUSPECT→verify
  237/238 exact text.
- 23Ib-fd2 (errexit-implicit rc-consumption OPEN under the flagship walkthrough): 23D §3 interim
  posture "guards mint only where NO explicit status reader exists; errexit-implicit is OPEN,
  unpinned both ways" — while USER_STORY (KNOBS-tier) depicts guards minting in a `set -eu`
  book as settled ("survived by design"). If errexit-consumption resolves to blocking, guard
  minting in strict-mode books (≈ all diligent books) dies and stage-2's value story with it.
  Verify 23D exact text + whether USER_STORY carries a caveat.
- 23Ib-fd3 (enforcement-parity on the apply path): guard tier moves oracle check-bodies onto
  the mutative path; probe-lane enforcement (an-withhold-sandbox / an-withhold-monitor /
  vouch-closure) cannot follow into a plain-sh artifact. 233 names the hazard-class
  ("collapses the fact-indirection"; "body-trust machinery must be inherited onto the execution
  path") but the inheritance mechanism appears undefined; composition XFAILs (set-u crash,
  variable-capture) pinned open at weld time. Priced posture-shift, yes — residual: what IS
  the body-trust machinery on the execution path? Verify 23A/23D.
- 23Ib-fd4 (ANALYZER-NEEDS not absorbed): the living registry gained no guard-tier rows;
  an-elision-predicate (st B) misdescribes the welded predicate (no vouch term, no wall term).
  Acknowledged as owed in LIVING_STATUS arc-2 ("ANALYZER-NEEDS rows" during build slice) —
  so tracked-not-silent; still a currency gap in a root-level doc. Verify git log.
- 23Ib-fd5 (USER_STORY monotonicity overclaim): "Note what installing this library could NOT
  do: hurt anything." Unconditional; actually conditional on third-party checks honoring
  read-only + being sane (slow/crashy checks cost probe latency, apply check-tax, guard
  fallthrough noise; mutating check = probe-phase mutation). DESIGN carries the conditionality;
  USER_STORY (audited, KNOBS-tier) states it bare. Small but real overclaim in the newest
  root doc.
- 23Ib-fd6 (vouch semantics welded while its meaning is TODO): rul-guard-license welds
  converged-vouch as THE license while the human's own TODO (f333db7, same day) records
  converged-vs-no-op author-intent as an open question, and the vouch *spelling* is OPEN
  (strawman stub). Weld-of-license before settle-of-semantics — check 23F h1-h5 for whether
  this is squared.
- 23Ib-fd7 (admin recourse parked): machine inserts suppression (guard) of commands the admin
  wrote; the admin's "always run this, ignore the vouch" idiom is PARKED (233; 235 hatches
  parked, un-park signal = admin-recourse pressure). Interim recourse = plan-edit per apply.
  Two-users check: cost lands on admin, judgment lands on engineer. Priced-but-parked; verify
  235/23D.

## Round-2 updates (after 23D, 237, 239, 23F)

- fd2 (errexit-implicit) CONFIRMED: 23D §3 "human suspects painful breakage under EITHER
  default"; interim posture = mint-when-no-explicit-reader; "no pin encodes either default."
  USER_STORY depicts the interim posture as product, no caveat; stage-2 ledger rests on it.
  STANDS (medium-high).
- fd6 (vouch semantics) MOSTLY DIES: converged-vouch meaning IS ruled (23D §7 three-way
  convergence terminology; vacuous-universal test); only the spelling + first-class
  converged-vs-noop modeling are open, both disclosed. Keep as one line.
- fd3 (apply-path trust) NARROWS: 23C demonstrated adv-1 (var-clobber: book `pkg` overwritten
  by predict body → vim never installs; +SURE, executed) + adv-2 (set-u kill); human h3 ruling:
  subshell-wrap sanctioned + local-hygiene encouraged, explicitly "never as sandboxing" (rm-rf
  reigns). Behaviour-pins landed. RESIDUAL that stands: adv-3 runtime path-drift FAIL-OPEN
  (host-conditional branches land on unvouched rc-0 paths ⇒ guard suppresses with no live
  verification; both fixture bodies rc-0 their refuse paths, verified) — disposed to
  "oracle-contract lint territory; not pin-now" while the mechanism it rides is welded. Plus:
  one-body-two-lanes (path-exercise identity between lanes) flagged NOT-part-of-GO, task #2.
  The 239 delta-1 GO-rationale ("same bytes, both lanes") is byte-true at the default pole,
  path-open at the sanctioned lifted pole. STANDS (medium), framed precisely.
- fd4 (ANALYZER-NEEDS staleness) STRENGTHENS: cov-q4 retired the polarity/kill class from the
  lifted representation entirely ("NO polarity/kill class survives, ever"), and cov-q3
  acknowledges "the globally-enumerable one-to-one probe table is gone" — yet §A/§D rows
  (an-kill, an-effect-polarity, an-elision-predicate, an-fact-probe...) stand un-annotated at
  status B. Registry stale in BOTH directions (missing guard rows; invalidated old rows).
  Tracked-owed caveat stands (LIVING_STATUS arc-2 one-liner).
- fd7 (admin recourse) CONFIRMED priced: 237 convergence-5 (both adversaries + human's own
  parked idea converge on the one-line lever), still parked, task #10 un-park signal recorded.
  Report as conscious-park with 3-way want-signal.
- NEW fd8 (guard-value anti-correlation, their own finding): 237 convergence-4 (3-way): the
  expensive classes (update/pull/restart) are volatile/run-delta/excluded = un-guardable;
  check-tax forever; 233's "shadow" line corrected. Their own crosscheck says the guard-half's
  retained-value claim needs measurement; route = spike dashboards. Report as: the welded
  half's value-shape is open BY THEIR OWN ADJUDICATION; the docs (USER_STORY ledger) present
  the guard-half's value as illustrative-but-positive.
- NEW fd9 (golden-hill vs scattered-opaques): 239 §1 golden hill = "90% of the book" commented
  out; the ternary preserves that ONLY at full per-command vouch coverage + convergence (elided
  commands cast no wall — elegant salvage, credit it); but an UNMODELED command always runs ⇒
  permanent wall ⇒ under the human's own ~90/10 scattered-opaques prior (237 correction-2
  block), real books cap elision at the first unmodeled tool forever, pending the unbuilt
  derivation arc (238). USER_STORY admits it plainly ("ufw ... verifies rather than elides,
  forever"). Meanwhile 239 §1 MANDATES no-aspirational-language about the elide-goal
  corpus-wide. Vocabulary-policing vs their own priors: rhetorical finding, one paragraph.
- Fairness ledger (say plainly): crisis honestly declared by the human ("the design is broken,
  right now"); trade frontloaded in 233's update; render-compression paper-over REJECTED
  (collision-1) despite two agents recommending it; corrections layered honestly (233
  end-annotation, 237 post-adjudication incl. human antagonistic challenges); xfail-first
  discipline; TOCTOU re-scope reasoned not waived; monotonicity restoration is real.

## Round-3 updates (after 238, 23A, git checks)

- NEW fd10 (THE strongest): the crisis-opening §0 unsoundness is design-closed but neither
  pinned nor scheduled. Chain: 239 §1 welds "silence means NOTHING" design-wide (signed GO);
  its elide-tier consequence (a will-run modeled command cannot be cleared by its oracle's
  silence ⇒ walls its downstream) is depicted as current behaviour in USER_STORY (stage-3
  drifted day: modeled foobar walls systemctl) — but: 23A hz-ambient-hole states the P-pair
  goldens ENCODE the §0 ambient elision at HEAD ("documented, not endorsed"; "when its fix
  lands" — unscheduled); no xfail pins modeled-will-run-walls-downstream (X-flagship's wall is
  opaque hork; flagship site-3, the modeled diverged site, has nothing downstream of it); the
  fix lives in arc-4 elide-half ("when the human wants it", seeds ~SUSPECT, hostile pass owed).
  So the unsound behaviour the human declared "broken, right now" in 233 remains live at HEAD,
  un-xfail'd, with the guard tier (new value) scheduled ahead of it (old soundness). Fairness:
  hz-ambient-hole + P-pair book-comments disclose it loudly; the naive immediate fix
  (poison-everything) would collapse the elide tier — sequencing has a rationale. But the gap
  between signed-law and pin-lattice is real and exactly one un-authored case wide. +SURE of
  the components; ~SUSPECT of "unscheduled" (arc-4 has no date, that's the basis).
- fd2 SHARPENED: 23A np-errexit: "NO book in the set uses `set -e`; no pin asserts either
  answer... both defaults suspected painful" + hz-setu: "The corpus-standard predict-body idiom
  is itself set-u-unsafe... unpinned, unresolved". The pinned e2e corpus deliberately avoids
  set -e/-u because open; USER_STORY's flagship book leads with `set -eu` (deliberately added
  in b941508 "real shell constructs") and narrates guard-minting there as settled ("survived
  by design"). The careful corpus and the confident walkthrough disagree about the same regime.
  (set -u BEHAVIOUR pin landed post-23A via 23G repairs as xfail — desired, unbuilt; errexit
  remains unpinned both ways.)
- fd3 final shape: adv-1/adv-2 demonstrated → h3 ruling (subshell sanctioned, local-hygiene
  encouraged-never-enforced, "Dorc is not and never will be a sandbox") → behaviour pins.
  Residue that stands: hz-refusepath/adv-3 fail-open class (rc-0 refuse paths suppress
  mutators when reached at runtime off the vouched path) — tripwires exist (P-multiop,
  P-rundelta) but the runtime-path-drift channel itself = "lint territory, not pin-now";
  np-onebody (path-exercise identity between lanes) open task #2. jc-silencing: check stdout
  leaks into apply transcript, deliberately unpinned.
- fd5 upgraded to human-owned: 13170c3 shows the human's final pass touched adjacent lines
  (FIXME marker etc); the unconditional "could NOT hurt anything" survived line-level audit.
- fd4 precision: ANALYZER-NEEDS last substantive touch = d186d81 (round-18). Stale across
  rounds 19-23 entirely (two spikes + crisis), both directions (no guard/vouch/witness rows;
  invalidated rows an-kill/an-effect-polarity/an-elision-predicate still st=B; cov-q4 "NO
  polarity/kill class survives, ever"; cov-q3 "the one-to-one probe table is gone").
  LIVING_STATUS arc-2 owes "ANALYZER-NEEDS rows" — tracked, unpaid, and not pivot-specific.
- 093 round-9 closed-world revocation annotation: confirmed absent (owed per LIVING_STATUS §6
  loose-ends). Minor, tracked.

## FINAL LEDGER — what I hand back (ordered by survivability under scrutiny)

Confirmed, strongest first: fd10, fd2, fd1, fd8 (their-own-finding, incl. the foldable-guards
mitigation 233 leaned on being banned by rul-attention-honesty — conscious, collision-1),
fd3-residue, fd9 (rhetorical tier), fd4, fd5, fd7, doc-drift nits (AGENTS fail-fast
unreconciled-in-place; DESIGN "rearrange" vs round-20 ruling; DESIGN reintroducing "skip"
against its own AGENTS ban; 093 annotation owed).

Withdrawn after honest kill-attempts: wd1 (vouch-laundering — fence holds through the
un-walling chain), wd2 (consumed observables — refusal classes designed+pinned), wd3
(book-order pin — round-20, not the pivot), wd4 (divergence-proceed vs fail-fast — coherent
under the identified-cause/TOCTOU split), wd5 (attention-honesty vs hidden-elided-lines —
two-surfaces), plus: guard mechanism unsound (survived a genuine kill-brief, 236c), the
door-4 reversal as drift (consciously re-welded with rationale engaged, 239 delta-1), the
attention-loss as hidden (frontloaded in 233's update in the human's own voice).

Where the work genuinely holds (state plainly): the crisis was self-caught and honestly
declared; the ternary is a real escape (monotonicity restored; silence de-weaponized; the
un-walling-by-elision logic is blast-radius-sound); fail-directions are right throughout
(converged-only mint, can't-tell runs, fall-through-to-run); the render paper-over (fold the
verify-tail, Ansible ok:96) was PROPOSED BY TWO CROSSCHECK AGENTS AND REJECTED by the human's
welded honesty doctrine — the exact opposite of quiet redefinition; process quality
(three-agent crosschecks under skepticism calibration, discount ledgers, xfail-first,
anti-masking, honest STOPs) is far above what the "novice + slop" prior predicts. Per the
project's own never-vouch discipline: that is process-evidence, not proof of correctness.
