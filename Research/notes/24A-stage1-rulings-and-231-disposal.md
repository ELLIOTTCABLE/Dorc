# 24A — round-24 Stage 1: session rulings, the 231 disposal, and the honest-baseline evidence

AI-authored (Fable conductor), 2026-07-03, round 24, **LIVE WORKING NOTE — IN PROGRESS**
(marked frozen at stage close; the 23M precedent). Charter: `plans/240`. This note is the
durable home for (a) the human's typed in-chat rulings at round-24 open, (b) the 231
disposal paragraph carried from the 23J repair-pass routing, and (c) Stage-1 build
evidence as it lands. Confidence-marked per the standing discipline.

## §1. Human rulings at round-24 open (typed in-chat, 2026-07-03; recorded per silence-is-not-ack)

- **rul24-wall-placement (ratifies the Stage-1b design).** Verdict-aware walls computed at
  plan time are correct, and not merely expedient: "re-ordering and re-verdicting is valid
  *right up until the moment of consent*" — the eventual design actively wants probe-results
  driving continuous, visible, in-UI plan-updates as they stream from multiple hosts. The
  wall *absolutely* depends on a probe-verdict. The first-order solution to
  allowing-stuff-after-yourself-to-elide is always "just elide *yourself*"; this round's
  charter is the *next*-order solution — how to let downstream elide when you yourself are
  diverged and must-run (Stage 2's footprint × backing × disjointness).
- **rul24-churn-on-green.** Golden churn from the fd10 flip is licensed, considered low-risk
  ("we're actively destroying and rewriting all that machinery"); execution delegated to
  Opus-tier builders.
- **rul24-conductor-role.** The conductor's deliverable #1 is "what strains"; #2 is
  really-clever briefs ∪ skeptical review of finished work against cross-cutting
  invariants. Opus agents are trusted for *local* correctness; they trip on subtle
  cross-cutting concerns — that is the conductor's review domain. High bar for
  conductor-written code. (Also: Opus builders MAY spawn sonnet subagents for mechanical
  sweeps; the no-subagents clamp propagates one tier down — each spawned subagent must be
  told not to spawn further.)
- **rul24-spelling-style (binding on the Stage-2 footprint strawman and all new spellings).**
  90% of solutions should work very hard to find *native sh spellings people already use*;
  if spelling-as-sh must break (as the typesystem did), break it *loudly* with first-class,
  high-quality language-design constructs. The middle ground is the worst approach.
  Strawman-tier stubs are exempt (explicitly NOT-DESIGN), the settled spelling is not.
- **rul24-yardstick-is-ui.** The plan-summary surface is a *feature strawman of the CLI
  pole* (the planned TUI/CLI interface split: TUI = ANSI + live-updating + live-editing;
  CLI = dirt-simple, flags, multiple invocations, stdin/stdout). Design the mock CLI, then
  test through it; never build the CLI *to* e2e-test it (that inverts the e2e into a janky
  indirected integration test). Low-importance to get exactly right; don't churn.
- **rul24-acceptance-shape (prospective, NOT a gate).** If the round goes to plan, the
  final "teaching" plausibly includes the human personally pointing an Opus at his real
  dotfiles — where idempotence sucks to establish, what needs oracles, first stdlib oracles
  for macOS idioms. Absent that, there is effectively no written acceptance-goal; the round
  is hard to "acceptance" in a vacuum. Bias the build toward
  actually-usable-at-the-CLI-in-anger.
- **Process fact (harness, recorded in conductor memory too):** subagent worktree isolation
  bases new worktrees on an ancient `main`, not the session branch — every builder brief
  now opens with an explicit step-zero `git reset --hard ai/spike3-r23` + tip-hash verify.

## §1a-addendum. rul24-mode-gate (typed in-chat, 2026-07-03, post-Stage-1-merge — BINDING on Stage 2+)

The elide-past-a-running-wall behaviour (the whole footprint/disjointness tier) **must be
mode-gated behind an explicit flag and must never be a (literal) default.** Non-flagged
behaviour stays Stage-1 ground truth: silence→run, no post-diverged-wall elision in any
meaningful way, fallback-to-guard-only (once the guard tier exists). Flag requirements:
clear name; SHORT enough that most users won't alias it away (an aliased opt-in is an
invisible opt-in); not actively recommended by hints or docs beyond noting availability.
Honest framing, never to be forgotten (human's words): the opt-in is likely to become a
cargo-culted effective-default ("everybody running --unsafe-enable-dangerous-auto-mode");
it is **marketing at best** (the user feels they chose the danger instead of instantly
attributing it to a Dorc bug) **and theatre at worst** (every user enables it — why are we
pretending). The last-ditch CYA "the user opted in" is demanded anyway, made as non-vacuous
as possible. Stage-2 e2e must assert BOTH sides: flagged = survival-elisions; non-flagged =
Stage-1 semantics unchanged.

## §1b. Footprint-spelling vibes session (typed in-chat, 2026-07-03; the USER_STORY stage-5 mock)

The footprint strawman was mocked into `USER_STORY.md` stage 5 (commit `2c6c0f3`) as a
third role-sibling `<provider>.touches()` — invoked with the site's argv, emits the
entity-coordinates the verb MUTATES one per line; emitting-anything-for-a-matched-verb
claims at-most-these; unmatched verb emits nothing = no claim = wall. Human vibes, typed:

- **rul24-threefunc-monotonic (shape RATIFIED).** The three-function oracle surface is
  fine — "as long as they always all stay optional and there's a clear monotonic
  gradual-enhancement path where every added component buys additional value, and no added
  component loses previously-present value." (Human confirmed the mock satisfies this.)
  NB this consciously supersedes rul-role-split's "exactly two invocation-contracts"
  sentence — the surface is now exactly THREE, all optional; the rc-partition rules apply
  to the two verdict-shaped members only (touches() emits coordinates on stdout, not a
  verdict rc).
- **rul24-claims-by-emission (ACCEPTED as strawman, reluctantly).** Opt-in-by-emission
  ("this is 233, strawmanned into being, and a living, breathing nightmare. it's also the
  entire brief of the round") stands for the build. OPEN question carried to build-evidence:
  does the sharpest claim in the design deserve its own explicit visible syllable, rather
  than riding an ordinary printf? If strawman-testing shows mis-authored footprints biting,
  the explicit syllable is the first lever.
- **Name: `touches` accepted** ("meh, don't care") — RESOLVED by the conductor's
  mutation-vs-observation analysis (below): footprint = write-set only, and `touches` reads
  write-flavored in sh culture (POSIX touch mutates), so the name matches the semantics.
  Pending-ack analysis, not yet human-ratified: reads don't kill facts; order-sacred removes
  the anti-dependency reason to track reads; the disjointness intersection is asymmetric
  (wall's WRITES × fact's probe's READS). The one reads-flavored worry (wall writes what the
  downstream COMMAND reads but its PROBE doesn't — update/install/pending-upgrade) is the
  converged≠no-op adequacy residue, already priced at the vouch — risk relocated, not new
  (23N §5 conserved-risk, again). Future consumer of read-sets = `dorc bump`
  backward-slicing; if it ever consumes footprints, re-run the scoping analysis.
- **Price: deferred to the round's evidence** (human: "we've ascertained this is dangerous.
  now, is it *valuable*?" — the yardstick's question, not a spelling-chat's).
- **Vocabulary fence (human catch, 2026-07-03): "verb" is author-plane shorthand, never an
  engine key.** The engine's only scoping concept for marks AND for `touches()` emissions
  is *the reached path under the site's constant-propagated argv* (rul-guard-license's
  "reached", verbatim). The author's `case $verb in install)…` is ordinary sh control-flow;
  per-verb judgment is expressed POSITIONALLY (decorate one arm, not another). A mark's
  textual fragments (`apt-get:install`) are opaque attribution payload — a consumer that
  token-matched them against argv would breach inv-referent-agnostic and duplicate
  reachability. Builder briefs for Stages 2/3 carry this fence explicitly (the vouch
  consumer and the touches-lift are both unbuilt — the drift would otherwise be easy).
- **Scoping analysis (conductor, pending-ack, answering the human's
  danger-chronology question):** the footprint's danger is CO-LOCATED (danger-sites ⊆
  benefit-sites, definitionally — only a disjointness-licensed elision can be wrongly
  licensed) and CO-TIMED (both fire only on runs where the wall actually runs; steady-state
  days never consult the footprint). Value is strictly monotone under
  rul24-threefunc-monotonic. Two honest asymmetries: SOCIAL (damage lands on the admin's
  line; blame on the footprint's author — the vouch's shape, one step sharper) and
  STALENESS-DRIFT (scope never widens with tool age; wrongness-probability climbs — the
  MH2 residue). The differentiator vs the pre-crisis dangerous-middle: same danger,
  UNSCOPED there (silence licensed everything, unsigned); positional/opt-in/signed here.
- **Sequencing flag (conductor, non-blocking):** post-crisis prose (23O §1.3) says the
  elide-license itself ultimately rides the converged-vouch; the spike's live license
  consumes no vouches yet. Stage-2 disjointness lands as an ADDITIONAL conjunct on the
  existing license; Stage-3 vouch-plumbing tightens the same license later — monotone
  composition, no conflict, tracked so neither stage silently "completes" the other's law.
- **rul24-divergence-is-the-game (human challenge, conductor-checked, CONCEDED + refined).**
  The human challenged the conductor's danger-scoping answer as too positive-leaning: "the
  'license-site diverges from elision-site' IS the whole game, and the whole danger, behind
  233alikes such as this feature." Checked; the logic HOLDS, with one refinement: TWO
  divergences compose. (i) **license-site ≠ elision-site** (all 233-alikes, vouch included):
  the completeness-claim escapes the context that could falsify it — authored once against
  the author's tool-version/platform/imagination, acting at unbounded later sites where
  nobody present can evaluate it (the admin can't — not knowing the tool is WHY they use an
  oracle — and the author isn't there). If claims could act only in their author's own books,
  233 would be nearly toothless (wrong claims bite their authors; closed feedback loop).
  (ii) **claim-subject ≠ blast-subject** (footprint-specific): the claim describes tool A's
  writes; the blast lands on tool B's wrongly-elided line. The vouch has (i) only; the
  footprint has (i)+(ii) — why it is the sharpest knife. The conductor's earlier co-location/
  co-timing answer stays mechanically true WITHIN one plan (danger-sites ⊆ benefit-sites;
  fires only when the wall runs) but answers where/when, not WHO-can-check — the epistemic
  divergence was wrongly footnoted as "social asymmetry". Confirmation: every existing
  defense maps onto the divergence (attribution = post-hoc re-tethering; opt-in = deliberate
  license-site; CONCENTRATE = shrink what travels; horizon = warn the elision-site;
  dangling-reference detection = the one mechanical elision-site check, existence-only).
  **Design-seed (human, delighted):** the missing defense — the elision-site mechanically
  validating the claim's CONTEXTUAL assumptions — is the long-wished version-tracking system
  (MH2's content-hash gating: "the binary you're eliding around is the binary I described"),
  now justified under the boxing-in-233 regime. NOT this spike. Counter-lean kept for
  honesty: the divergence is also the entire VALUE engine (author-pays-once, amortized — the
  library model); it cannot be removed, only tethered. Stage-2 consequence: attribution of
  survived elisions (why-lens naming whose footprint licensed what) is promoted from
  nice-to-have to the primary tether in the builder brief.

## §2. The 231 disposal (carried from the 23J repair-pass routing: map the pre-crisis sweep's clusters to survivor/dead-with-reason)

Context: `notes/231` is the *pre-crisis* collapsed-gradient sweep — round 23 §1 work,
intercepted by the `plans/233` crisis before any of it was acted on. The crisis rebuilt the
verdict architecture under it (ternary verdict; silence=wall; vouch never enters the
fact-plane; the atomic-command axiom). This disposal says what a future reader may still
build on. Per-cluster, against the post-`23O` settled law:

- **1a `decision-plane-trust-cell` — DEAD as designed; superseded in shape.** The crisis
  built the claimed-vs-proven distinction into the *verdict tier* instead of a per-value
  trust tag: fact-tier probe-observations license elide; judgment-tier converged-vouches
  license guard only and NEVER enter the fact-plane (rul-guard-license; rul-ternary-verdict's
  two nevers). A source-tagged fact-plane trust cell would now *duplicate* that architecture.
  What survives of 1a is its reporting half (why-lens provenance, decision-inert under
  ru-11 — unchanged) and the eventual "licensing tier / cross-site blast attribution" agenda
  (`23M` open-agenda), which is Stage-2+ work under the new framing. Do not resurrect 1a's
  shape.
- **1b `cardinality-strong-update` — SURVIVES-DEFERRED, re-keyed under Stage-5
  entity-identity.** The finding (no strong update exists; the uniqueness bit is absent,
  not boolean; `Kill` accumulates) is untouched by the crisis and still true. Its natural
  re-entry point is the round-24 Stage-5 synonym/aliasing cell (`23N` §6:
  must-not-alias-or-wall; dynamic points-to), where within-kind entity-identity becomes
  load-bearing. The 231 fence stands, now reinforced by silence=wall: a "probably unique"
  verdict may only DEMOTE toward weak/⊤, never PROMOTE to strong.
- **1c `coverage-vouch-default` — SURVIVES in principle; still spelling-blocked; not
  round-24 work.** The channel-completeness vouch is a species of the crisis's sanctioned
  shape (opt-in, author-explicit, judgment-tier claim; silence stays ⊤⇒run — exactly the
  anti-233 posture). Its spelling remains `dq-kOOB`/`kTYANNOT`-gated and is now further
  entangled with the parked "is authoring a verdict-function partly the vouching act?"
  question (`23O` §5). **[CORRECTED 2026-07-03, human catch — the original sentence here
  ("round-24 defers all vouch spelling") was overbroad and wrong.]** Three tiers, not one:
  (i) the *settled* vouch spelling is deferred, human-reserved (dq-kOOB) — not this round;
  (ii) *strawman* vouch spellings are REQUIRED by this round's method (charter: "built
  against strawman spellings"; rul-guard-license: "a stub explicitly marked strawman,
  trivially cheap to swap") — at HEAD the converged-vouch strawman half-exists (the
  predict-parser accepts the bare mark `: provider:verb~` as ConvergedVouch,
  derived-and-discarded; NO fixture carries the mark yet — the guard23 oracle headers
  narrate "PLUS the strawman vouch below" over bodies that contain none). Completing it
  (mark in fixtures + witness consumption) is Stage-3 work; the footprint strawman is
  Stage-2 work; (iii) only THIS cluster's channel-completeness vouch — which nothing on
  the six-stage ladder consumes — holds entirely, no strawman needed.
- **1d `multicell-establish-classify-cliff` — SURVIVES-LIVE; a yardstick-raiser.** Pure
  coverage/precision walk-back, orthogonal to the crisis, every piece built except the
  classify match. Becomes interesting exactly when the strawman family shows it binding
  (a multi-cell verb refusing to elide with all cells converged). Candidate rider for
  Stage 6 tuning; all-or-nothing aggregate stays the safe default (tc-multicell-aggregate-grain).
- **1e `partial-member-convergence` — SPLIT by the atomic-command axiom.** The
  across-ENTITIES half (partial elision of one multi-operand line, `install nginx curl jq`)
  is now DEAD — welded shut, not deferred: the atomic-command axiom (`23D` §3, `23O` §2)
  makes multi-operand lines whole-line; granularity comes from author-written loops. The
  across-MEMBERS half (loop members) SURVIVES-DEFERRED with 231's own hazard intact: the
  all-or-nothing license is a fixed-point self-consistency argument, and partial-member
  elision requires a per-member `self_reach` re-derivation (a separate analysis, not a
  softened knob).
- **1f `door3-recovery-dormant` — SURVIVES-LIVE; cheap yardstick-raisers.** The fold's
  three dormant recoveries (andor both-operands-agree; `node_rc` line-recovery; door-3's
  bare-`true` narrowing) are monotone, inv-kfail-safe precision wins, untouched by the
  crisis. Candidates for Stage 6 when the yardstick shows fold-blocked strawman lines.
- **§2 (the must-stay-boolean fence) — SURVIVES-WELDED, strengthened.** The fence's spine
  ("a gradient may only DEMOTE toward run, never PROMOTE toward elide") is now also the
  crisis's own law; `fence-coverage-effect-completeness-NOT-mutation` ("you can never
  confidence-soften 'this might mutate something I didn't model'") *is* 233 restated. The
  round-24 Stage-1b fd10 fix implements the fence's spirit at plan time.
- **§3 (decision-surface THIN) — SURVIVES with a delta.** Still no trust/source tag keyed
  by any live decision. Two deltas since: the Stage-1b wall walk adds *upstream will-run*
  as a plan-time decision input (verdict-derived, not trust-derived), and the unbuilt
  GuardLicense witness (Stage 3) will be the first decision-edge keyed on judgment-tier
  material — by design, quarantined from the fact-plane.
- **§4 (channel-vouch / mention=vouch) — LETTER DEAD, SPIRIT CONFIRMED.** Every spelling
  §4 grounds in (`oracle_effect` rows, `oracle_probe_*`, `oracle_kind=`) was retired by the
  23H realignment (the marker fiction deleted; effects derive from predict bodies). The
  mechanism it defended — mention-as-vouch, absence=safe-⊤ — is now *more* true: writing a
  probe idiom inside the predict body IS the mention, and deriving `ValueClaim`s from it IS
  the vouch. 231 §4's inline correction pointer (→ `232`) stands; add this note on top.
- **§5 tc-flags — carried, with two changes.** `tc-disagreement-rung` survives for the
  merge/reporting half; its *licensing* half is settled by `23L`'s LICENSE-SOURCE ruling
  (ONE convergence source at vouched sites; fact-plane ambience is never a second
  license-source). `tc-one-observable-build-vs-spec` is PARTIALLY DISSOLVED by the
  realignment (predict() now IS the oracle; the build moved toward the slug's text);
  residual gap = per-channel Stdout/Stderr prediction still has no authored surface —
  re-examine only if it binds. The `unseeded-hunt` agent's 7 lost candidates stay lost;
  a re-run is moot post-crisis (the sweep's framing predates the ternary verdict) —
  recorded as an accepted loss.

## §3. Stage-1 build evidence — LANDED 2026-07-03 (both builders merged; 131/131 green)

Two Opus builders, isolated worktrees, cherry-picked onto `ai/spike3-r23` (yardstick
`c78fba2`/`c70ca5e`/`44c7d1c`; fd10 wall `c1a9b2c`; XPASS promotion `ba805af`). Full gates +
`cargo test` (523) + e2e **131/131** (9 standing guard23 xfails) green on the merged tree.

**The honest-baseline yardstick reading (post-fd10; `sh e2e/yardstick.sh`):**

```
case                              sites  elide  omit  guard   run  elide-fr
strawman24-adequacy-seed              1      1     0      0     0      1.00
strawman24-all-converged-clean        3      3     0      0     0      1.00
strawman24-floor-no-oracle            3      0     0      0     3      0.00
strawman24-mixed-real                 6      2     1      0     3      0.33
strawman24-modeled-wall               2      0     0      0     2      0.00
strawman24-opaque-wall                3      1     0      0     2      0.33
strawman24-partial-oracle             3      1     0      0     2      0.33
FAMILY (7 cases)                     21      8     1      0    12      0.38
```

modeled-wall 0.50→0.00 was the fix landing as a visible metric change (pre-fix family 0.42).
**Post-wall elisions = 0 — the charter's Stage-1 baseline claim, now mechanically true.**
Stage 2 exists to move exactly these numbers back up, soundly.

**The fd10 mechanism (commit `c1a9b2c`):** an 8-line plan-time walk in `build_plan`, after
the span-sort (execution order — order-is-sacred), before leaf-id assignment. Wall predicate
= establish-bearing classes (`EstablishAmbient`/`Written`/`Members` + establishing
`InlineCall`), deliberately NOT `MustRun` (pure builtins never wall; opaques already ⊤-poison
statically — harmless redundancy). Demotion Replace→Run only (`inv-kfail`). Elided/omitted
mutators cast no shadow. A demoted mutator itself becomes a wall (it will run). Probes still
ship for post-wall sites (static classify untouched) — the strawman XPASS confirmed it
byte-identically against the pre-authored designed-future golden.

**Strain-ledger extracts (conductor-reviewed; confidence marks are the builders'):**
- **THE KILL GAP (A; flagged, open, routed to Stage 1c).** A running `Kills`-only command
  (`apt-get purge`, classifies `MustRun`) does NOT wall downstream different-cell converged
  establishes — the same under-execute shape as fd10. No corpus case exercises it (both kill
  cases are same-cell ⇒ `EstablishWritten` ⇒ run anyway). Closure needs `CommandEffect`
  threaded into `build_plan` (or a `SkipClass::Kill` variant) — localized, reversible.
- **Path-insensitivity of the walk (A; ~SUSPECT narrow).** Mutually-exclusive branches
  (`if q; then update; else install; fi`) can over-wall — but a plain `if [ -f x ]` condition
  lowers to ⊤ and poisons downstream anyway, so the loss needs a live non-poisoning,
  non-folding guard. Over-execution direction (safe); Stage-2 disjointness supersedes the
  total wall regardless. No CFG-aware walk warranted now.
- **hz-ambient-hole's lockstep-flip expectation RETIRED (A; conductor-verified).** The named
  sweep candidates (`exec-poison-wall-dead`, `guard23-vouch-inert-pair-a/b`) did NOT flip:
  their upstream mutators are CONVERGED ⇒ elided ⇒ no shadow — already honest at HEAD under
  the ratified law ("elision casts no poisoned shadow"). The corpus simply had NO
  diverged-upstream case until `exec-modeled-wall-runs`; that absence WAS the pin-open gap.
- **errexit defeats converged-mutator elision (B; KNOWN cost, newly MEASURED).** Under
  `set -e` a mutator's ⊤ rc is status-consumed ⇒ `consumption_ok` blocks ⇒ converged mutators
  run. NOT a new finding — it is the `206 §2` headline cost with the 20V doors as its named
  recovery program (door-3 landed) — but the yardstick now QUANTIFIES it (B's mixed-real
  collapsed to 1 elide until `set -e` was dropped), and the round-24 strawman-family axis had
  omitted it. Route: a dedicated `strawman24-errexit-defeats` case at Stage 6 (or 1c if
  convenient); NB the USER_STORY's flagship book opens `set -eu` — the story's stage-1 render
  depends on the doors program maturing. Family-axis lesson: book-IDIOM quality is an axis
  alongside oracle-coverage quality.
- **Gate-6 blindness bound (B; +SURE).** A wrong-but-LICENSED Replace is invisible to the
  dual-rail judge (it attributes, it does not judge licenses). The modeled-wall defect was
  pinned by the two-sided `head-expected.ran` + exec_check, not gate-6 — remember this when
  reading "differential-verified": license-correctness rides the hostsim differential's
  required-to-run judgment and the pins, not gate-6 alone.
- **Ratified surfaces:** summary grammar `dorc: plan-summary sites= elide= omit= guard= run=`
  (stderr, beneath gate-3's error-floor; parse target — grammar now stable); emitted in all
  plan-building modes incl. apply; `elide-fr = elide/sites` (Replace only; omit shown
  separately). `DispositionCounts`' exhaustive match forces Stage-3 to wire the guard bucket.

**Process notes:** worktree-isolated agents CANNOT `git reset --hard` (hook-blocked) — the
working step-zero is `git switch -C <worktree-branch> ai/spike3-r23` (both builders
independently found equivalents); future briefs say so. Conductor lean going forward
(human-typed): mechanical suite-running/verification rides Opus errands; the conductor keeps
only judgment moments (promotion diff-inspection, adjudication).
