# 23Ia — directional review of the round-23 redirection (external reviewer)

Reviewer notes, 2026-07-02/03. Brief: critical-but-supportive review of the r23 design-shift
(ternary verdict {elide, guard, run}); hunt for lost plot, dropped threads, dangerous roads
quietly taken. Out-of-scope per brief: corpus/market-fit; spike-code-specific issues; holes in
explicitly-TBD design (unless landmine-grade). Certainty marks per convention:
+SURE / ~SUSPECT / -GUESS / --WONDER. This note is process-evidence, not a correctness claim
(never-vouch discipline applies to it as to everything else AI-written).

Read: README/DESIGN/IMPLEMENTATION/TODO/TODO-ADDTL/KNOBS/AGENTS/USER_STORY/ANALYZER-NEEDS(head)/
STALENESS-AUDIT(head) · Research/README + LIVING_STATUS · plans/233 (incl. end-annotation) ·
plans/230 · plans/22H(head) · notes/231 (whole) · 232(head) · 237 · 238 · 239 · 23A · 23D ·
23F (both addenda) · 23G (targeted) · 23H §7 · spike/CLAUDE.md · 218a (targeted) · the
guard23-ternary-flagship fixture + goldens + git history · `git show a92ad31` (the human's
root-doc absorption). Quarantine and corpus dirs untouched.

## §0 Verdict on the redirection itself

+SURE on the shape: the plot is NOT lost. 233 documents a genuine, load-bearing incoherence in
the pre-r23 oracle effect/poison contract (silence-as-vouch unsound — a partial oracle worse
than none; silence-as-poison valueless — nothing elides in real books; no default escapes the
frame problem), discovered and written up by the human personally. The ternary verdict is a
coherent escape that restores monotonicity ("silence licenses nothing") and the trade is
frontloaded honestly in the stamped doc ("give up the attention-product to save the
performance-product, wherever the world is undescribed"). The founding priorities survived
re-cut, not diluted: the human's own absorption commit (a92ad31) SPLITS the old priority-2 into
authorship-effort vs attention-budget and writes the poison-wall/guard story into
IMPLEMENTATION in his voice, including the "this is a different product" warning. The two-halves
doctrine + rul-attention-honesty are stronger anti-drift fences than the corpus had before the
crisis. Process quality was visibly high: 3-agent crosscheck with skeptical adjudication, four
human post-adjudication corrections (two REVERSING agent/conductor conclusions —
anti-sycophancy machinery demonstrably firing both directions), xfail-first pins, a hostile
pin-review that found real mechanism bugs (23C adv-1 variable-clobbering; adv-2 nounset) which
were then ruled and repaired (23F h1–h4), and an honest builder STOP (23H §7) rather than a
masking co-authoring pass.

Dangerous roads that were taken were mostly taken with eyes open: the door-4 reversal is
re-welded with the original deferral's REASONS engaged (239 delta-1 rationale); the TOCTOU
amendment draws a principled identified-cause/open-world line; the fact-indirection collapse
(probe-code onto the execution path) is named as a new hazard-class with containment; kSILO's
new shove is recorded with its mitigation; the round-9 closed-world revocation is flagged as a
loose end. The exceptions are below.

## §1 Findings (23Ia-fdN, strongest first)

### 23Ia-fd1 — the guard's rc-channel is semantically different from the probe's mark-channel, and no ruling names the gap (~SUSPECT design-gap, +SURE on each instance)

The probe lane and the guard lane run the same authored bytes but READ DIFFERENT OUTPUTS:
the probe harvests PER-MARK node rc's via engine scaffolding into the OOB record lane
("rc is opaque to Dorc; verdicts travel out-of-band"; the flagship probe golden derives
holds/absent/cant-tell engine-side); the guard `check || command` necessarily consumes the
body's single AGGREGATE in-band sh rc — there is no Dorc runtime at apply. "Plan-prediction
and apply-guard run the same code, so plan-vs-apply divergence can only be world-drift, never
model-disagreement" (233 §guard-license) is therefore true only under an UNSTATED structural
precondition: on the vouched path, the body's aggregate rc must coincide with the
mark-conjunction the plan verdict was computed from. Three instances of its violation are
already in the corpus, treated piecemeal, without the unifying condition being named:

- hz-refusepath (23A §6): refuse paths exit 0 (`case` with no matching arm; a failed `if`
  with no else) — covered by the witness's reached-path component;
- adv-3 (23F): runtime path-drift onto unvouched rc-0 paths — hazard-registered as
  "oracle-contract lint territory";
- NEW, sharpest — the inverted-claim path (+SURE, traced concretely): the R4a re-spelling
  gives every package oracle a `purge` arm `dpkg-query -W "$pkg" ... : package:"$pkg".installed!`.
  Strip loses the `!` (annotation-plane); the shipped body's rc then means the OPPOSITE of
  purge-convergence. A converged-vouch on that path + the ruled `||`-form mints a guard that
  RUNS the purge when converged (harmless, useless, check-tax) and SKIPS it exactly when the
  world drifted to needing it — wrong-elision, the one sin, in the safe-sounding mechanism.
  Today this is masked only by (a) the cov-q4 interim freeze (inverted-claim arms ⇒ MustRun),
  whose in-code note says it "dissolves into the uniform no-vouch-no-elide license when the
  guard/vouch tier lands" — a dissolution instruction that, followed naively, mints the
  backwards guard; and (b) the strawman vouches never covering purge. No pin covers a
  vouched-inverted arm (P-rundelta covers unvouched restart; 23C hunted refuse-paths, not
  inversion-under-`||`).

Also related, one strawman-idiom consequence worth stating for the vouch-spelling round:
233's own systemctl exemplar (the most-copied oracle text in the corpus) reports facts through
if/else marks in CONDITION position, establishing enabled=true or =false without the body ever
failing — its stripped aggregate rc is vacuously 0 on the modeled path. Fact-reporting-style
check bodies (which the elide-half NEEDS: establishes-both-ways feed the fact-plane) are
structurally rc-unsound as guards; tail-position single-probe bodies (the fixture style) are
rc-sound. The two-halves doctrine's own rule ("no guard-half decision may quietly discard a
constraint the elision-goal needs") cuts here in both directions and nobody has written the
tension down.

RECOMMENDATION: promote rc-soundness to a structural component of the witness — a guard mints
only where the analyzer can verify, on the vouched reached path, that aggregate-rc-0 implies
the vouched establish-set holds (tail-position probe, non-inverted claim, no trailing
rc-clobbering statement, refuse-paths nonzero-or-unreachable) — refusing to guard otherwise,
loudly, exactly like the other refuse-homes. That single condition subsumes hz-refusepath,
adv-3, and the inversion case, and it belongs to the vouch-spelling family's charter (dq-kOOB
cluster) plus one new pin (vouched-inverted-arm never guards). Cheap now; a silent
wrong-elision generator later.

### 23Ia-fd2 — the flagship golden went stale through the XFAIL blind spot, and now contradicts the ruling it pins (+SURE, verified via git)

At pin-authoring (e5bdbf9) the flagship check body was the simple tail-probe form and the
golden's preamble lawfully equaled strip(body). R4a (9a3faef, same day) rewrote the fixture
oracle to the case-with-marks form and did NOT touch the golden — its commit message says
"Golden-stable", which is true only because XFAIL content-diffs are inactive (exactly the
conv-1 build-window blindness 23B/23C flagged). At HEAD the pinned artifact bytes are neither
the whole stripped body nor a byte-identical substring — the flagship golden now violates
rul-ternary-verdict's sourcing law, and the conv-1 repair (the grep-floor asserts only the
`<check> || <original bytes>` line-shape) does not police preamble≡strip(body). Whoever
promotes the flagship will hit un-flagged churn; worse, a builder could treat the golden's
case-collapsed preamble as the intended emitter output (it reads like the "lifted" form,
which the strip-only reversal made an optional edge-case). Fix: re-author the preamble bytes
now or annotate the case dir; consider extending the grep-floor to preamble-vs-oracle-file
equality while XFAIL persists.

### 23Ia-fd3 — the 230/231 walk-back map was orphaned by the crisis pivot without triage (+SURE on the absence; ~SUSPECT on how much it matters)

r23's chartered deliverable (plans/230 §1: the collapsed-gradient walk-back map, "ranked by
lock-in") landed as notes/231 and then vanished from the arc: no 23x note, ledger, or
LIVING_STATUS item carries 1a (decision-plane trust-cell, HIGH lock), 1b (cardinality /
strong-update-is-ABSENT, HIGH lock), 1d (multicell classify cliff — the sweep's nominated
"shovel-ready" coverage gradient, silent and untested at classify), 1f (door3 recovery trio),
or the tc-flags (tc-disagreement-rung, tc-cardinality-strong-update-rung,
tc-multicell-aggregate-grain), or the unseeded-hunt agent's 7 LOST candidates that 231 §5
explicitly flags as "the one place the sweep is materially incomplete." Research/README's
claim that "the surviving un-collapse work lives on in the elide-half design arc (seeds: 238 +
23D §5)" is traceable only for the vouch thread (231-1c → 232 → the parked dq-kOOB family) and
the consciously-parked 1e (atomic axiom, 23D §3); the rest is silently gone. Partial defenses
exist — the ternary itself IS the headline un-collapse; the vouch's plane-separation
(fact-plane vs judgment-tier) arguably obsoletes a graded trust cell for now; 1b's
problem-area partially re-emerges as 23D §5's entity-aliasing fence — but none of that is
written down as a triage. Cheap fix: one paragraph in the rulings ledger disposing each
cluster (subsumed-by-ternary / parked / carried-into-elide-half / re-run-the-lost-agent), so
the next best-effort round doesn't re-pay the sweep.

### 23Ia-fd4 — USER_STORY (audited-tier) carries four wrinkles that survived review (+SURE on each text-level contradiction; severity low-to-medium)

- (a) Stage-1's "Dorc knows enough to *not even show those first lines* to the user most of
  the time" contradicts the doc's own header ("the plan is the whole book ... elided lines
  are present-but-commented-out") and the typed plan-surface ruling (23D §2: whole runbook,
  original order, elided lines present-but-greyed, NO tier-sorted views). Both claims sit on
  the load-bearing attention-honesty weld; one of them is wrong, or the artifact/render
  distinction needs to be said out loud there.
- (b) Stage-2's monotonicity pitch — "installing this library could NOT ... hurt anything" —
  overclaims claim-monotonicity into code-monotonicity. Adding an oracle cannot endanger
  other sites' VERDICTS (true, and the design's real achievement), but it ships that
  author's code onto every host in probe and (vouched) guard position — the round-10
  supply-chain surface ("Dorc is a package manager; users don't read scripts"). One
  qualifying clause fixes it.
- (c) The very first elision shown (stage-1, `apt-get update` ⇒ "converged: package index
  fresh") rides the volatile/freshness class that 237 convergence-4 lists among the excluded
  big-ticket classes (update/pull/restart) and that kVOLATILES/run-delta treats as the
  hardest convergence to state. Writable as an author's judgment-priced vouch, maybe — but
  the walkthrough's poster-child eliding on a freshness singleton, unremarked, will steer
  early oracle-authors straight at the muddiest class. A footnote or a different first
  example (the dpkg guard alone) avoids teaching the exception first.
- (d) Stage-4's published-oracle exemplar uses bare `"$2"`/`"$1"` arity probing — the exact
  nounset-fatal idiom 23C adv-2 demonstrated and 23F h3 ruled against (lean into `local` +
  `${n:-}` hygiene) — in a story whose book sets `-eu`. The subshell-wrap contains the
  blast (check dies ⇒ falls through ⇒ run: safe-but-silently-useless guard + check-tax),
  but the audited tutorial is teaching the anti-pattern the same week it was ruled against.

### 23Ia-fd5 — lane-privilege is the one unnamed cell in the guard posture-shift (~SUSPECT, seam-reservation grade)

The door-4 re-weld rationale answers drift ("same trust-object both lanes, same bytes") but
not privilege: probe-lane oracle code runs under the probe's execution context (and the 077
seccomp classifier backstop); guard-position code runs under the apply lane's — and
sudo/become first-classing is a deferred-surface item (TODO-ADDTL / 17O R2-CONTEXT). Today
the lanes plausibly share a user; when become lands, a vouch would silently promote oracle
code from the read-lane context into the book's elevated context. Nothing needs building
now; the become design just needs a one-line reservation ("guard invocations inherit the
probe-lane's privilege contract, or get their own ruling") so the cell can't be skipped when
that round happens. (Exclusion-check pedigree: other-phase × other-user × privilege — the
IMPLEMENTATION "two axes of trust: competence AND security-privilege" line's second axis
currently has no r23 home at all.)

### 23Ia-fd6 — root-doc absorption residuals (+SURE, small, listed so the next rewrite pass doesn't miss them)

a92ad31 absorbed the big pieces (priorities re-cut; poison-wall; guarding section;
skip→guard/replace/elide terminology). Still owed, now-contradictory rather than merely
missing:
- DESIGN "Contract & DX": "high-quality oracles should *not* change behaviour in the
  happy-case ... they should always strive to be a functional no-op" — a vouch-licensed
  guard is now a SANCTIONED behaviour delta (suppressing a converged command). The paragraph
  needs the third license written in, or the guard carved out of "behaviour."
- IMPLEMENTATION "two similar-sounding questions we ask the oracle-writer" — the
  converged-vouch is a THIRD question (a fallible skippability judgment, distinct from both
  mutation-safety and modeling-completeness), and it is the only one that changes apply
  behaviour. Absent from the root docs entirely.
- rul-attention-honesty's welded sentence (delta-5's candidate line) is still not verbatim
  anywhere human-authored; the new priority-3 paragraph carries the reasoning but not the
  rule.
- STALENESS-AUDIT (the designated rewrite input-sheet) is rev-3, pruned 2026-06-09 — it
  predates r23 and carries none of this; anyone rewriting from it inherits the gap.

### 23Ia-fd7 — the guard-tier's standalone value-measurement intent has no arc slot (-GUESS, soft; category-1-adjacent so stated once and dropped)

237 convergence-4 (3-way: retained-value needs measurement; payoff population
anti-correlated with guardability) was routed to "spike-internal dashboard instrumentation"
— which appears in no LIVING_STATUS arc item; check-cost banding (task #8) is parked with no
sanctioned data source. If the sizing intent is dead, better to say so than to let
convergence-4 read as handled.

## §2 Checked and found adequately handled (so the negative results are on record)

- Variable-namespace clobbering + nounset in guard position: found by 23C (adv-1/adv-2),
  ruled (23F h3: subshell-wrap sanctioned now, local-hygiene encouraged), behaviour-pinned
  (23G). The crosscheck machinery caught a real mechanism bug pre-build.
- Guard stdout leaking onto the apply transcript: jc-silencing, deliberately unpinned and
  flagged with the 218a redirect as the known fix.
- Book-order pinning vs the 22H live-plan arc: intra-host order was already welded in
  round-20 ("no intra-host apply parallelization or reordering, ever"); guards change
  nothing there. The probe-stream/re-fold composition risk is flagged twice (Research/README,
  LIVING_STATUS #5) and gates the placement-spectrum round. Watched, not dropped.
- kSILO shove from machine-inserted guards: named in KNOBS with the no-double-guard
  mitigation (P-handguard pins it).
- Escape hatches, partial-member elision, errexit-implicit consumption, one-body-two-lanes,
  wave/hoisted re-verification, multi-host plan surface: each consciously parked/deferred
  with owners and un-park signals (the np-* register in 23A is exemplary practice).
- The atomic re-spelling build honored anti-masking (23H's STOP rather than co-authoring
  mocks and goldens in one pass; 23F ask-probe-divergence ruled with mocks-first sequencing).
- rul-divergence-proceed vs AGENTS' cross-network fail-fast: bare-sh parity holds (errexit
  books keep errexit; un-errexit books continue as bare sh would); the residual
  guard-skips-a-command-that-would-have-crashed cell is inside the deferred errexit-implicit
  experiment, where it belongs.

## §3 One paragraph on what I did NOT do

Per brief: no corpus/market/value-sizing analysis (beyond fd7's one line); no spike-code
review beyond what design questions forced (the flagship fixture archaeology served fd1/fd2);
elide-half seeds (238's horizon/derivation, 23D §5's namespace-ownership and grounding
bridges) read but not adversarially worked — they are ~SUSPECT-marked with their own hostile
pass owed, and nothing in them read as a product-killing landmine to me at this altitude
(--WONDER stands on whether "coherence gates disjointness" is solvable without a de-facto
registry, but that is precisely the question their own pass is chartered to answer).
