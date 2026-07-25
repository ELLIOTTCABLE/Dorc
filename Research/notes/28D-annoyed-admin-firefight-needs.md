Annoyed-admin firefight survey — what a production incident asks of Dorc
========================================================================

> Seat & method (Fable session, 2026-07-25, banked for the r28 docs pass): prompted as an
> experienced admin at 03:40 — product down, expensive, `dorc apply` ran on cron at 02:00,
> and the deployment is presumed guilty until proven otherwise. Deliberately-thin context:
> the human-tier root docs plus the oracle-contract reference only; AID-NEEDS opened only
> after the enumeration already existed (its influence is confined to the coverage sketch
> at the end); the Research/ ocean unread. The harness force-loaded AGENTS.md and the
> spike CLAUDE registries, so "independent seat" is bounded by that. Scope is far-future:
> what Dorc might be ASKED to provide, unconstrained by build state. NOT folded into
> AID-NEEDS/ANALYZER-NEEDS — human directive, fold later; the coverage sketch below is the
> fold-guide.

Grading legend
--------------

The `grades:` field accumulates per-seat opinions; this note mints the `UX-want` seat
(experienced-admin-firefighter demand). Scale: A = "I will go looking for this in the
README before adopting, and will literally not risk the product without it — I have needed
it often enough to know"; B = reached-for during real incidents, absence materially
degrades trust; C = would be loved once discovered, but nobody arrives demanding it;
D = sounds nice to have. Other seats append their own `SEAT X` tokens inside the parens.

Extinguish — the 03:40 set
--------------------------

- `need-exact-input-identity` (grades: UX-want B): the full identity of what actually ran
  overnight: book bytes (rev/content-hash), the complete loaded oracle-set with
  versions/pins and where each came from, engine version, flags typed
  (`--risk-faultless-skips` on/off decides which whole trust-tiers were even in play),
  target list, and the trigger (cron / human / CI). Firefighting step zero is "diff
  against the last good night," and "the book didn't change" proves nothing — the oracle
  library or the engine may have moved beneath it. B rather than A only because git plus
  cron config reconstructs most of it by hand; the oracle-set identity is the part nothing
  else records (see the A-graded `need-dependency-provenance-audit`).

- `need-ground-truth-action-ledger` (grades: UX-want A): per host, per line: ran /
  guard-short-circuited / guard-fell-through / elided, with wall-clock, tool exit status,
  captured stdout/stderr, and the exact post-guard artifact bytes that executed. The first
  concrete question is "what changed on this box overnight," answered from the actor's own
  records in seconds rather than reconstructed from syslog and shell history.
  Fell-through guards count double: each is a line the plan implied probably-converged
  that really mutated. A: no experienced admin runs a mutating orchestrator that keeps no
  run-report; this gets checked in the README before first use.

- `need-withheld-action-ledger` (grades: UX-want A): the negative space — every line that
  did NOT run, with its complete license chain (measured / vouched / claimed / derived /
  consented, per USER_STORY's why-render), filterable corpus-wide by trust tier ("only
  elisions resting on an unverified human claim"; "only ones that rode the risk flag").
  An orchestrator's signature failure is invisible at the host: a thing that didn't happen
  leaves no log line anywhere on the machine — if the fire is under-execution, this ledger
  is the only place the fire exists at all. A: for a product whose entire pitch is
  not-running things, "show me exactly why you skipped" is an adoption gate.

- `need-probe-phase-account` (grades: UX-want B): everything executed while "only
  looking": which oracle bodies ran on which hosts at plan time — including for plans
  nobody ever applied — each with file:line provenance and exit status; which contexts
  were entered (sudo-class) under which consents; known probe residue (the auth-log
  class). Probing is billed read-only, so nobody suspects it, which makes a mutating or
  mis-sited probe the perfect crime; and probes run parallel, fleet-wide, repeatedly —
  maximum blast radius at exactly the moment billed as zero-risk. The seat wants to rule
  the entire plan phase in or out with one query.

- `need-world-then-versus-now` (grades: UX-want B): timestamps on every measured fact; the
  plan's age at apply time (approved Friday 18:00, applied Sunday 02:00?); and one command
  for "re-measure now, diff against what you believed then." Splits the three root
  hypotheses — acted on a stale picture / acted correctly and the world moved after /
  innocent entirely — which demand completely different next moves.

- `need-exoneration-or-conviction` (grades: UX-want C): the symptom-indexed backward
  query: given the broken thing (a path, a service, a port), which executed lines COULD
  have touched it, which withheld lines SHOULD have touched it and didn't — plus the
  counterfactual: would a blind top-to-bottom run of the same book have left the host
  different? The working presumption is Dorc's guilt; the fastest good outcome is a
  credible alibi that redirects the hunt within minutes. Must be honestly bounded ("I ran
  `hork` at 02:03 and cannot speak for what `hork` did") — a confident wrong exoneration
  is the mis-attribution sin transplanted to the explain plane (IMPLEMENTATION's refined
  ladder). C overall because no admin has ever had this, so nobody arrives demanding it;
  the touched-my-path half alone grades B (it is what log-grepping approximates today),
  the counterfactual D-going-on-C.

- `need-fleet-blast-radius` (grades: UX-want B): the cross-host view: which verdicts fired
  identically fleet-wide versus only on the sick host; which hosts share the suspect
  oracle or claim; which had fell-through guards or divergent probe answers. Drives the
  two immediate calls — containment ("are 40 more boxes about to do this?") and diagnosis
  ("what is DIFFERENT about host 7?" — a per-host probe-fact diff is ~SUSPECT the
  highest-signal debugging artifact this tool could emit). B at the homelab target-market
  scale; becomes A the moment host-count clears a handful.

- `need-partial-apply-geometry` (grades: UX-want A): when the apply dies mid-book: exactly
  where; what completed before it; what proceed-and-flag flagged; and a classification of
  the resulting host state — converged / known-diverged / UNKNOWN. Recovery differs
  completely between "everything ran, one thing was wrong" and "died at line 41 of 210,
  box in a state nobody has named"; the unknown cell is the one that forbids naive
  re-running. A: "what happens when a step fails halfway" is a day-one README question
  for any orchestrator, and a mushy answer ends the evaluation.

- `need-suspicion-ranked-claims` (grades: UX-want C): incident triage ordered by the
  engine's own epistemology: every load-bearing human claim in the apply ranked by
  fragility (naked at-most claims above vouches above measurements), cross-ranked by load
  ("this one disturbs() arm licensed 60 elisions") and by recency-of-change. USER_STORY's
  why-chain does it for one line ("link 4 is the one unverified human claim in this
  chain"); this is the corpus-wide generalization as the incident landing page. C:
  transformative once experienced, but nobody knows to demand it in advance.

- `need-emergency-distrust-levers` (grades: UX-want A): restore-service controls that
  require zero understanding of the analysis: a full-distrust reconcile (run everything,
  elide nothing); per-oracle quarantine ("plan as if foobar's oracle doesn't exist");
  per-claim revocation; and the off-ramp in one command (emit the original un-Dorc'd book
  for hand-ssh). At 03:40 nobody debugs — they restore; the "it's just a shell script
  underneath" pitch is only real if the fallback is reachable while angry. ~SUSPECT the
  single most-demanded item on this list. A: the escape hatch is what an experienced
  admin checks for FIRST in any tool that makes decisions on their behalf.

- `need-raw-greppable-receipts` (grades: UX-want B): everything above also available as
  dumb raw data — the receipt as plain text/JSONL, executed artifacts retained as actual
  `.sh` files on disk — not solely through curated `why` renders. When the tool is the
  suspect, its explanations are suspect too; raw ground truth is the trust floor, and 3AM
  tooling is grep and jq, not a query language learned once at onboarding.

Prevent — the morning-after set
-------------------------------

- `need-root-cause-leverage-point` (grades: UX-want B): attribution terminating at a
  fixable file:line with a named owner, plus the repair-reach statement ("widening
  foobar.oracle.sh:31 heals every book downstream of it"). Prevention only sticks when
  the fix lands where the error CLASS lives — an oracle repair fixes the fleet; a
  defensive tweak in one book fixes one symptom and leaves the class armed.

- `need-incident-becomes-regression` (grades: UX-want C): freeze last night's wrong
  decision into a permanent deterministic test — replay the receipt through the engine
  and pin "under these facts, this line must run"; plus the engineer-side harness
  (DESIGN's containerized-TDD tooling cloud) so the fixed oracle carries the case
  forward. The postmortem action-item is always "add the test that would have caught
  this," and if that is not cheap it never happens. C from this seat; the harness half is
  engineer-seat material and may grade higher there.

- `need-dependency-provenance-audit` (grades: UX-want A): the supply-chain view of
  oracles: what the books depend on, at which claim tier (verdict-only / disturbs /
  kind-owner), published by whom, pinned how — and what changed between the last good run
  and the first bad one ("foobar's disturbs() gained the renew arm on Tuesday"). After
  weeks of quiet, an overnight break usually traces to a changed input; the book is in my
  git, the oracle library is in nobody's review flow. A for the inventory+pinning half:
  an experienced admin will not run unpinned community shell against prod as a matter of
  standing policy, and looks for the pin story pre-adoption; the cross-run what-changed
  diff half grades B.

- `need-granular-trust-repricing` (grades: UX-want B): post-burn controls finer than one
  global flag: scope `--risk-faultless-skips` per book / per oracle / per kind; standing
  distrust-lists ("guards, never elisions, for anything touching certs"); staleness
  limits ("refuse a plan older than N hours without re-probing"). The realistic
  post-incident reflex is switching the risk flag off wholesale and eating the
  drifted-day cost forever; granular repricing keeps the product's value while excluding
  exactly the burned party.

- `need-standing-drift-watch` (grades: UX-want B): the scheduled plan-as-monitor taken
  seriously: the stable exit-code contract, probe cost-classes, timeouts, per-host
  opt-outs (the shape USER_STORY's cooperation section already sketches), plus alerting
  when a long-elided line re-enters a plan. The overnight apply should not be the first
  thing to discover drift; the same diverged fact is a ticket at 14:00 and a fire at
  02:00.

- `need-near-miss-trend-surface` (grades: UX-want C): the shadows a fire casts before
  burning: guard fall-through counts across runs (a guard that keeps falling through is a
  vouched fact that keeps being wrong); accumulating decline/UNK breadcrumbs (unmodeled
  shapes silently blind-running, night after night); unmodeled lines now at 90
  consecutive re-runs against a possibly-non-idempotent tool. Each of these is
  this-incident, earlier and cheaper. C: real value, but it collides hardest with
  statelessness (tension list below) and nobody demands it pre-adoption.

- `need-consent-custody-trail` (grades: UX-want C): who approved which plan, when; the
  plan-at-approval versus the plan-at-apply; which flags were typed by whom; whether a
  stale approval was re-validated. The second morning question after "what broke" is "who
  signed off," asked by someone wearing a compliance hat — and it is the admin's honest
  self-defense ("the plan I approved did not contain this line"). C from the firefighter
  seat; -GUESS a change-management/compliance seat grades this B or A. The
  plan-age-at-apply sub-item does double duty under `need-world-then-versus-now`.

Tensions with standing posture (from the seat; observations, not rulings)
-------------------------------------------------------------------------

- `ten-statelessness-versus-forensics` — `need-fleet-blast-radius`, the diff half of
  `need-dependency-provenance-audit`, `need-standing-drift-watch`, and
  `need-near-miss-trend-surface` all want exactly what USER_STORY forswears ("no drift
  daemon, no fleet history, no stored suspicion") and rec-5 bans (the whylog is never
  re-ingested). -GUESS a thread-the-needle exists: receipts are already durable, so
  retaining every run's receipt and letting an EXTERNAL dumb tool diff them is
  trending-without-a-daemon — the ban is receipts feeding decisions, not receipts
  existing side by side.
- `ten-backward-query-inversion` — the symptom-to-lines query inverts the why machinery
  (today: line-to-explanation); a wrong "Dorc never touched X" is a mis-attributed
  exoneration, the worst sin-ladder cell transplanted to the explain plane. The recovery
  principles (explaining says more, with labels attached) cover the spirit; the inversion
  itself is new machinery.
- `ten-fleet-versus-single-host` — nearly everything read narrates one host; fires are
  fleet-shaped. Fan-in and correlation rows exist (ANALYZER-NEEDS §I/§J), but per-host
  verdict DIFFING as a first-class diagnostic surface does not.
- `ten-attention-inversion-under-fire` — the push/pull split already answers most of the
  apparent conflict between attention-rationing and forensic completeness: the
  firefighter is a pull consumer, and these entries demand coverage and rawness of the
  pull surface, not more push. +SURE this is consistent with
  `AID-NEEDS:law-pull-runs-wide-open` as written.

Coverage sketch vs the registries (fold-guide only; registries deliberately untouched)
--------------------------------------------------------------------------------------

Written AFTER the enumeration existed, as a check against AID-NEEDS/ANALYZER-NEEDS:

- Largely tabled already: `need-withheld-action-ledger` ≈ `AID-NEEDS:aid-why-license-chain`
  + `aid-survives-attribution` + `aid-plan-line-reason` (the corpus-wide trust-tier
  FILTER is the increment); `need-root-cause-leverage-point` ≈ the chain's
  leverage-point epilogue; `need-raw-greppable-receipts` ≈ the whylog + the 27W
  noise-tolerance posture (the raw-dump/JSONL stance is the increment);
  `need-standing-drift-watch` ≈ `AID-NEEDS:aid-error-exit-code-family` (st O);
  `need-partial-apply-geometry` ≈ `AID-NEEDS:aid-apply-divergence-report` (st S — the
  stop-point geometry plus the known-diverged/UNKNOWN state classification is the
  increment); `need-exact-input-identity` ≈ `AID-NEEDS:aid-loaded-oracle-inventory`
  (st O, build-state unverified) + the whylog invocation record.
- Partially tabled: `need-probe-phase-account` (`aid-escalation-consent-legibility` and
  the report lane cover slices; no full probe-execution inventory row exists);
  `need-ground-truth-action-ledger` (the whylog's apply report; ~SUSPECT per-line
  stdout/stderr capture of applied lines is nowhere specified);
  `need-dependency-provenance-audit` (`ANALYZER-NEEDS:an-oracle-ref-sha` st S covers
  pinning; inventory st O; the cross-run diff is absent);
  `need-incident-becomes-regression` (`an-replay-seed` and whylog replay are the seam;
  pin-as-test is absent); `need-suspicion-ranked-claims`
  (`AID-NEEDS:aid-why-problems-report` is the seed; fragility-ranking is absent).
- Absent as far as read: the counterfactual and symptom-indexed halves of
  `need-exoneration-or-conviction`; `need-fleet-blast-radius`;
  `need-near-miss-trend-surface`; `need-granular-trust-repricing` (the flag is
  per-invocation only); `need-consent-custody-trail`; and the plan-age-at-apply
  disclosure of `need-world-then-versus-now`.
