# 26C — The rerun-to-fixpoint engine: semantics worked through, the quiet-welding audit, and the revival implementation plan

AI-authored (Fable, solo design-rubber-duck pass at human direction, 2026-07-17) —
minted into the r26 series on the design lineage (`ai/main`), sibling to `notes/26B`
(the sitting record + human-typed rulings + obligations bank; its §1 rulings govern
here). This note is the deep working-through 26B's header anticipated: (a) the
reactive/rerun-to-fixpoint semantics made precise, including one CORRECTION to 26B §2's
confluence claim; (b) the audit the human asked for — invariants and quiet weldings in
the standing corpus/code that become unsound, wrong-direction, or silently misleading
under the reactive direction, each with a disposition; (c) a concrete implementation
plan slotting the reactive engine into the `262`/`260`/`261` stage ladders for the r26
revival. Verified against the r27 build tip (`ai/spike3-r27` @ `e16b0c8`, block-rebuild
closed + block-context lanes 1–3 landed, lane-integration in flight): the single-shot
pipeline shape, the single plan-mint choke point, `FactKey.context`, and the framed
records lane are all as this note assumes. Authority: root docs and 26B §1 outrank
this; everything below is conductor synthesis, confidence-marked. Companions:
`26B` · `262`/`260`/`261` + `26A` · `275`/`219` · `27H`/`27I` · `plans/27C` ·
`277` §5 · `22H`.

## §0 — Charge, and the findings in one screen

The human's concern (2026-07-17, near-verbatim): agents have been designing *without*
the reactive direction in mind; there may be invariants, weldings, or *quiet* weldings
in code and adjacent design that become unsound with this long-deferred machinery.

Findings, ranked:

- **`26C:finding-confluence-needs-conflict-carve`** (§1) — 26B §2's "confluence is THE
  correctness statement… byte-identical plan under every ordering" is +SURE only for
  *conflict-free* runs. Same-cell fact conflicts (the `22H` §1 ⊤→Value→⊤ retraction)
  make want-generation path-dependent, so the *gathered record-set itself* can vary
  with arrival order. Soundness survives unconditionally; byte-identity does not. The
  DST pin needs a carve, and the design owes a choice between accepting bounded
  superset-variance and a fact-GC discipline that restores full reproducibility.
- **`26C:finding-h1-rule-forecloses-iteration`** (§7 item 1) — the `26A`-amended
  `261` §2 h1 rule ("h1 edges resolve by connected-unit or controller-fold, **never by
  another shipped probe**") is a quiet welding minted in the single-wave frame; read
  literally it forbids the reactive engine's core move (iteration-N+1 probes whose
  argv embeds iteration-N captures). It needs a supersession annotation, not a repeal.
- **`26C:need-captured-bytes-ship-as-data`** (§5) — cross-iteration probe-minting
  ships *host-produced bytes back out* inside the next probe artifact's argv. This is
  a genuinely NEW security surface (the `275` §7 hostile-host hook widening from
  cell-naming into shipped-probe argv) that no standing law covers; a
  quoting/data-position law must be typed before the revival builds. This is the one
  item in this note I grade unsound-if-unaddressed rather than merely stale.
- **`26C:finding-classify-once-breaks`** (§7 item 3) — `260` s3-2 "classify once, fold
  per arrival" and `22H` §3's static-half host-independence silently break from
  iteration 2 onward: host-captured values enter the value plane, so analysis becomes
  per-host. Cheap (analysis is free), but the fleet kernel's shape must change from
  fold-per-arrival to analyze-per-iteration.
- Smaller but real: iteration-vs-attempt keying on the records lane (§4); the
  license-demotion order generalizing `262` §1's toward-run caveat (§3); economy-
  cancellation can land on guard-by-vouch instead of run, making it nearly
  plan-shape-free (§3); diagnostics must emit from final state only (§7 item 11);
  the sharpened binding-site-coherence menu, deliberately left un-ruled (§6).

What already holds and needs NO change (verified, +SURE): the sans-io fleet-kernel
shape (`260` §2) is reactive-ready as-is; `dorc-records/1` as landed (`27D` stage-5)
survives iteration with one additive key; the `27H`/`27I` representation is
reserve-shaped exactly as claimed (the foreclosure walk held); the plan-mint choke
point is still a single call site at tip; the `277` §5 universal meet is
fixpoint-robust by construction; `26B:rul-consent-cut-absolute` fences the apply
untouched.

## §1 — The semantic model, precisely (and the correction)

**State.** Per host: `R` = the accumulated record-set (grow-only by construction — a
record, once folded, is never removed; retraction happens in *derived* values, never
in `R`). The engine is a pure function `A(book, oracles, R) → (dispositions,
probe-wants, diags)`. The driver: fold arriving records into `R`, recompute `A`, ship
`wants(A) \ shipped`, repeat to quiescence, mint. Recompute-from-scratch per iteration
is the licensed implementation (`26B:rul-iteration-waste-acceptable`); the batch
pipeline is the degenerate schedule (26B §2, unchanged).

**Where 26B §2's monotonicity claim needs care.** The record-set is monotone; the
*derived cell-values are not*: `merge_observable` is a meet-toward-⊤, so a cell moves
⊤(unknown) → Value → ⊤(conflict) when two records genuinely disagree (`22H` §1 — the
retraction is reachable in principle: two establishers on one cell, an oracle adequacy
bug, or real drift mid-probe-phase). Conflict-⊤ is *sticky* (+SURE: the conflicting
records stay in `R`, so no later record un-conflicts the meet). Three consequences:

- **`26C:thm-fold-confluence-unconditional`** (+SURE) — for a FIXED record-set, the
  fold is order-free: the meet is commutative/idempotent/associative, so
  `plan = f(R)` regardless of arrival order. This is `262` §1's pin-fold-permutation
  and pin-terminal-determinism, unchanged — but note their quantifier: they range
  over orderings *of one record-set*, and say nothing about which record-set the
  process gathers.
- **`26C:thm-process-confluence-conflict-free`** (+SURE on the theory) — when no
  same-cell conflict occurs during a run, the knowledge system is genuinely monotone
  (every fold refines ⊤ toward value; killed arms only shrink walls), so by the
  classic chaotic-iteration result the want-set at quiescence is the unique least
  fixpoint: every ordering gathers the SAME final record-set ⇒ byte-identical plans
  under arrival shuffle. This is the honest form of 26B §2's advertised property.
- **`26C:finding-confluence-needs-conflict-carve`** (~SUSPECT on reachability,
  +SURE on the mechanism) — when a conflict DOES occur, a transient state (cell held
  Value V before the conflicting record arrived) can mint wants that the final state
  (cell ⊤) would not: probes ship, their records enter `R`, and those extra facts are
  *true measurements* that can enable outcomes the other ordering would not reach.
  <!-- /* corrected same day (§1b, prompted by the human's which-set-is-correct
  question): the original text here claimed extra facts license "extra elisions at
  sites outside the cone" via shared cells — wrong-as-stated, because
  inv-site-keyed-results means a VERDICT never travels between sites. The real
  cell-plane variance channel is the walls/patrol/fold-validity judgments (sound);
  the site-plane case is instead a genuine soundness hazard, closed by §1b's
  admissibility invariant. "Soundness is untouched" was likewise too glib — it is
  RECOVERABLE, via §1b, under either menu option. */ -->
  So the final record-set, and hence the plan, can vary with arrival order — and
  the variance is NOT confined to the toward-run direction. Plan reproducibility —
  a trust surface — is not free under conflict; soundness needs §1b's invariant.

**The menu for the carve** (revival's to choose; argue, don't drift — the same
discipline `26B:need-cancellation-finality-gate` demands):

- **`26C:opt-accept-superset-variance`** — accept it. Defense: a same-cell conflict is
  already adequacy-class territory (an oracle lied, or the world drifted mid-probe);
  the conflicted cell itself folds ⊤ ⇒ runs (safe); reproducibility-under-oracle-bugs
  was never promised. DST asserts byte-identity on conflict-free seeds only, and
  soundness + ⊤-on-the-conflicted-cell on conflict seeds. Cheapest; my lean (-GUESS)
  for v1.
- **`26C:opt-justified-fact-gc`** — at quiescence, recompute the want-fixpoint from
  the final state and mint the plan from only the *justified* subset of `R` (facts
  whose gathering want derives from final knowledge). Restores full byte-identity
  (plan = f(justified-R), and the justified set is order-free by the fixpoint
  argument). Discarding true facts moves affected sites toward run — the safe
  direction — and the waste is licensed by `26B:rul-iteration-waste-acceptable`.
  Costs: a second analysis pass and a subtler mental model ("the engine knows X but
  won't use it").

Either way, hostsim gains conflict-injection seeds; see §9.

## §1b — Addendum (same day; from the human's "the correct set of elisions is the correct set" question)

The human's push-back — *either direction feels like a failure-mode* — was half
right, and forced a split §1 blurred. There are TWO kinds of order-varying evidence,
with opposite dispositions:

- **Site-plane records under a retracted want — a genuine soundness hazard,
  closed unconditionally.** Worked instance: `W` (an interposer whose `disturbs`
  hits `/etc/pkg`) sits above `PKG=$(cat /etc/pkg)`; the capture's fold is valid
  only while `W` is proven-converged-and-elided (an elided command casts no wall).
  Cell `K_W` transiently reads converged → the fold validates → iteration 2 ships
  `dpkg__predict -s 'nginx'` for site S → S's record arrives. Then the conflicting
  `K_W` record lands → `K_W` → ⊤ → `W` runs → the wall stands → the fold retracts →
  S's argv is ⊤ again. S now HAS a verdict-record — but it answers "is
  `dpkg -s nginx` converged," and the apply-time site is `dpkg -s "$PKG"` with PKG
  unknown: the record answers a question the site no longer asks. Consuming it
  would elide on a stale premise — under-execution, the cardinal sin. Hence
  **`26C:inv-record-admissibility-by-want`** (soundness-tier, MANDATORY under
  either §1 menu option — this promotes §4's want-identity keying from bookkeeping
  to invariant): *a verdict-record is admissible at mint iff its carried
  want-identity `(site, context, resolved-argv)` re-derives from the final
  knowledge state; a record whose want does not re-derive is inadmissible ⇒ the
  site takes its final-state disposition (⊤ ⇒ run/guard).* Cheap to check: sticky
  conflict-⊤ means a resolution can move Value→⊤ but never Value→Value′ (+SURE),
  so admissibility is an equality test, not a search. Note the single-shot build
  never needed this invariant — "record exists ⇒ argv resolved" holds trivially
  when nothing retracts — which makes it exactly the class of quiet assumption
  this note exists to surface.
- **Cell-plane facts consumed by walls / the capture patrol / fold-validity —
  sound, order-variant, and the only variance that remains once admissibility
  holds.** These consumers ask questions about the WORLD ("did anything disturb
  this backing in the window," "is this interposer converged"), and a true
  measurement answers them soundly *whenever and for whatever reason it was
  gathered* — the license predicate consults measurement-truth plus static
  authored claims, never the motivation for asking. An extra cell-fact can
  validate a capture-fold (⇒ downstream elisions) that the other ordering would
  have left walled. Both outcomes are correct, because —

**— "the correct set of elisions" is not a unique set.** Elision-licensing is a
predicate over (plan, evidence), not a function of the world: the engine never
promises the world-maximal elidable set, only that every elision it prints is
individually licensed by true evidence. The licensed set was ALWAYS
evidence-contingent — a richer oracle library elides more; a probe deadline elides
less (`262` §1's own sanctioned weakening); a refused entry-form elides less — and
nobody calls the deadline-shrunk plan incorrect. Under-elision is the permanent
safe floor (unnecessary-execution is the LOWEST-ranked sin; "run everything" is
always a correct plan). What conflict-transients add is not a new correctness
axis but a new *determinism* axis: two identical invocations on an identical
world could gather different evidence and print different (both-correct) plans —
a trust/UX regression against the spirit of `262` spine-inv-order-free, not a
soundness one.

**Why `26C:opt-justified-fact-gc` restores determinism** (the replay argument,
+SURE): justification is computed by REPLAYING want-derivation from the final
record-set in canonical batch-synchronous rounds — round 0 admits all records
matching the no-facts want-set, re-derives, round 1 admits matches, and so on to
fixpoint. In the replay, both conflicting `K_W` records land in the SAME round, so
the cell is ⊤ before any later round's wants derive — the transient-motivated want
never re-derives, its answers are discarded, and the kept subset is a pure
function of the record-set's content, not its arrival order. Since (absent
conflicts) the gathered record-set is itself order-free (`thm-process-confluence-
conflict-free`), and (under conflicts) the replay discards exactly the
order-varying tail, plans become byte-identical unconditionally — the DST
byte-identity pin needs no conflict carve at all under this option. That testing
story is the strongest argument FOR the GC; the arguments against remain §1's
(a second analysis pass; "the engine measured X but refuses to look at it"). The
§10 ask stands; admissibility (above) is NOT part of the menu — it holds either
way.

## §2 — The line between scheduling and evidence (the `277` §5 re-read, delivered)

`27I` closed with: the fixpoint clause and `pin-no-outcome-as-generator` hold
trivially at HEAD because no probe-re-entrant back-edge exists; "the post-probe
re-bind re-reads this clause the day it is designed." This section is that re-read.

The precise line (+SURE this is the right cut; the human's banked concern in `277` §5
was exactly outcome-laundering under re-run-to-fixpoint):

- **Outcomes MAY schedule work.** Facts, resolved values, killed arms, and even
  dispositions may legitimately drive want-minting and cancellation — that is the
  reactive engine's entire point. Scheduling consumes knowledge; it manufactures
  none: a shipped probe returns a *measurement*, and the measurement is the only
  thing that enters the fact plane.
- **Outcomes may NEVER re-enter as evidence.** A compare-verdict (spare / collide /
  transport-same) feeds only its licensed consumer and never becomes an input to a
  later compare (`pin-no-outcome-as-generator`, unchanged); a *disposition* never
  feeds the sparing/transport relation, the fact plane, or a license mint. The
  universal meet stays the fixpoint-robust form: at every intermediate state a
  not-yet-resolved member reads unknown ⇒ collide, so partial iteration states can
  never spare more than the final state would (`277` §5, unchanged — and this is why
  recompute-per-iteration needs no compare-result invalidation machinery).
- One NEW fence this distinction implies, for the fallback-carry work landing in r27:
  **`26C:fence-capture-never-feeds-closure-pass`** — the `27C` §4(a)-(B) read-set-
  closure pass proves closure over *marks and sh structure*; a capture-resolved value
  must never be fed to it to "close" a body that reads an unmarked input. Closure is
  structural forever; captures resolve *argv*, never *closure* (~SUSPECT nobody
  intends otherwise, but the two mechanisms will coexist in one codebase and the
  laundering path is one careless refactor wide).

## §3 — Cancellation, worked through (the finality gate given mechanism)

26B's motivating example: cancel a slow read when a parallel fact disnecessitates it.
26B banked the hazard (cancel-on-non-final ⇒ order-dependent plans) and owed the
gate. Working it through yields something better than a bare gate:

**The license-demotion order.** Generalize `262` §1's deadline caveat ("timing policy
may move content only toward run") to the named-mechanism ladder: **elide ≻ guard ≻
run** — any timing/cancellation policy may only DEMOTE a site along that order,
never promote (`26C:rul-candidate-demotion-only-cancellation`; proposal-tier). The
`262` §1 wording predates the guard tier's subtleties and should be annotated to say
run-set-⊇ *or guard-set-⊇ where the vouch reaches* — the direction of safety is
identical (kFAIL-perform: less license, more execution).

**Where a cancelled probe's site lands.** Not bare run: a cancelled measurement is a
can't-say, and for a *vouched* site can't-say supports the guard tier with no probe
measurement at all — `26B:rul-reclassification-guards-are-floor` says exactly that
guards license off the vouch alone. So the policy
(`26C:mech-cancel-lands-on-guard-by-vouch`, ~SUSPECT the right default): an
economy-cancelled site with a reachable converged-vouch disposes GUARD; an unvouched
site was never probed anyway. Consequences, both pleasant: (a) the plan-shape cost of
cancellation collapses to reason-text — the line renders "verify" either way the
probe would have resolved short of elision, so cancellation is nearly
plan-reproducibility-free; (b) the check-tax moves to apply-time exactly once, which
is the human's example's own economics (the expensive check "will run at apply-time
anyway").

**The finality gate, made concrete.** Economy-cancellation may fire only when the
site's disposition CEILING is at-or-below where cancellation lands it, in every
extension of current knowledge. One class is cheaply derivable (+SURE):
**`26C:mech-wall-standing-finality-class`** — a site downstream of a
*confirmed-running unmodeled wall* can never elide: an unmodeled site always runs (no
oracle ⇒ no vouch ⇒ run — nothing can ever elide it), a running unmodeled wall casts
⊤ (no footprint ⇒ no survival under any flag), so every downstream site's ceiling is
guard, permanently, within this run. Cancel freely there; land on guard-by-vouch.
Everything outside a derivable-finality class keeps its probe (`26B`'s "alternatives
must be argued, not adopted by drift" stands). Note the monotonicity caveat from §1
applies here too: "confirmed-running" must mean *diverged-measured or unmodeled*, not
"currently believed diverged off a cell that could still conflict-retract" — under
conflict the wall-judgment itself can revert to unknown, which is why the finality
predicate must be derived from retraction-proof grounds (unmodeledness is
retraction-proof; a measured divergence is not, -GUESS pending the revival's
precision pass).

## §4 — Iteration mechanics on the wire (the `262` §2 delta)

The landed `dorc-records/1` (`27D` stage-5) survives reactivity with additive
changes only (+SURE, verified against the deframer at tip):

- **`26C:need-per-artifact-identity`** — each shipped probe artifact gets its own
  identity on the wire. Cheapest concrete form: mint a fresh `nonce=` per artifact
  (the nonce is already edge-minted and DI'd), and add an additive `iter=<n>` key to
  the header for render/why-lens legibility. `attempt=` keeps its existing meaning —
  *retry of one artifact*, discard-prior-wholesale — and must NOT be overloaded for
  iterations, whose records ACCUMULATE across artifacts. The two keys' semantics are
  opposites; conflating them is the obvious wrong shortcut (`262` §2's
  discard-wholesale rule applied to an iteration would throw away the accumulated
  knowledge base).
- **`26C:need-per-artifact-leafid-namespace`** — LeafIds are per-compile; iteration
  N+1's artifact re-derives them from a re-run analysis and there is no guarantee of
  stability against iteration N's numbering. Records therefore resolve through
  *their own artifact's* LeafId→AstId back-map (routed by nonce), and cross-iteration
  accumulation happens in site/cell space, never in leafid space. The header's
  `sites=<N>` census is already per-artifact — consistent.
- **`26C:need-want-identity-diffing`** — `26B:need-probe-want-diffing`'s
  `wants(N+1) \ shipped(≤N)` must key on the full want identity
  `(site, context, resolved-argv/recipe)` rather than the site alone: a site whose
  probe becomes compilable only at iteration 2 was never shipped, and a site-keyed
  diff would be vacuously right today but silently wrong the day two distinct
  resolutions of one site are both legitimate. (+SURE the identity-keyed form is
  never wrong; ~SUSPECT the site-keyed form is actually reachable-wrong, given
  conflict-⊤ is sticky — belt-and-suspenders is cheap here.)
- **Fold semantics across iterations**: same-cell facts from *different sites*
  across iterations merge by the existing meet (that IS the legitimate conflict
  channel of §1). Same-leafid duplicates cannot occur across artifacts (leafid
  spaces are disjoint by the namespace rule). Nothing else changes; the lane-split
  truncation semantics (fact-toward-run, deriv-toward-wall) apply per-artifact
  unchanged.

## §5 — The new law this note asks for: captured bytes ship as data

The one place I judge the standing law inadequate rather than merely stale
(`26C:need-captured-bytes-ship-as-data`; wants a typed human ack at revival, and the
ask is listed in §10):

From iteration 2 onward, the probe compiler embeds *host-produced bytes* (iteration-1
captures) into a shipped artifact's argv: `PKG=$(cat /etc/pkg)` captures `nginx`;
iteration 2 ships `dpkg__predict '-s' 'nginx'`. Today's byte-provenance law covers
two parties — oracle bytes ship (`271:rul-only-oracle-bytes-ship`), admin argv flows
through the oracle's argparse (`rul-argv-flows-bytes-do-not`), and *the admin's bytes
never ship*. The capture lane mints a THIRD provenance: host-spoken bytes, authored
by whatever ran on the host at probe time — the `275` §7 hostile-host hook ("stdout =
the host AUTHORING content"), which that note priced for cell-naming and future
artifacts, but which now reaches *shipped executable context* one iteration later. A
compromised or merely-weird tool emitting `` nginx; rm -rf / `` or `` `reboot` ``
must land in the next probe as an inert argv token, never as sh.

The law, proposal-tier:

1. A captured value re-enters a shipped artifact ONLY in argv position of an
   oracle-authored function, single-quoted through the engine's one quoting home
   (`sem::single_quote`-class, `'`-escaping included), never concatenated into
   command position, code position, redirect targets, or heredoc bodies.
2. The single-line wire floor (`275` §10) is load-bearing for this, not just for the
   wire: multi-line captures refuse the fold *and* therefore never ship onward.
   Newline rejection happens at capture, before anything downstream can consume.
3. The oracle's own argparse remains the type-checker (declines included) — a
   captured value that the oracle's argparse rejects lands can't-say ⇒ run,
   exactly like a weird admin argv.
4. Injection-safety holds at EVERY iteration's ship, not only at mint — intermediate
   artifacts are just as host-visible as final ones.
5. DST must-covers: captured values containing `'`, spaces, `$`, backticks, glob
   chars, leading `-`; a multi-line capture refusing; a capture consumed in the same
   iteration's why-text rendering safely.

(+SURE the surface is real; ~SUSPECT the five clauses are complete — the revival's
adversarial crosscheck should treat this section as a target, exclusions-not-
inclusions per standing practice.)

This also softens `26B:watch-dependent-chain-scheduling`'s asymmetry: the on-host
chain (executor-era) and the controller-side iteration have the SAME
host-bytes-into-argv exposure; the mitigation differs (controller-side cross-check vs
quote-as-data), but neither lane is the "safe" one. The quoting law applies to both.

## §6 — The binding-site coherence gate, sharpened (deliberately NOT ruled)

`26B:gate-binding-site-coherence` stays THE revival's opening design-question and
stays un-ruled (human's choice). This section only sharpens the menu it will be
answered from:

- **(a) freeze-in-artifact, reframed** — substituting the frozen value into the
  artifact is not a new mechanism: it is the ordinary Replace applied to the
  *assignment leaf* — `PKG=$(cat /etc/pkg)` → `PKG='nginx'` — a StandIn reproducing
  the assignment's observable from probe-provenance bytes, exactly the
  `inv-probe-sourced-values` licensed shape (`219` q-3.f render-substitute-assignment
  anticipated this). Under it the `279f` live-consumers hard gate *dissolves rather
  than binds*: apply-time consumers outside the folded region read the frozen
  binding, consistent with the killed arms by construction. Preconditions it drags
  in: the world-spoken floor (pre-pinned), `26B:need-scrub-before-freeze` (host
  bytes enter rendered artifacts for the first time — the secrets sequencing
  dependency), the §5 quoting law, and the render's multi-line/span-edit care
  (`219` q-3.f's single-line-replacement assumption). Divergence-*detection* (a live
  re-read compared report-only) composes cleanly.
- **(b) all-or-nothing folding, refined into (b′) structure-preserving folds** —
  the original (b) folds a case only when the entire construct's fate is decided.
  The sharper observation: with the binding kept LIVE, per-site elisions *inside*
  arms remain sound even without arm-removal, PROVIDED every modeled arm's sites got
  honest dispositions (which h2-speculation already provides: probes ship for all
  modeled sites regardless of arm liveness). A live re-capture that diverges from
  the frozen value dispatches into a *different arm whose sites also carry their own
  licenses* — no amputated structure, no silent fall-through. Under (b′) the
  capture's analysis value (un-⊤ the scrutinee, resolve downstream argv, reach
  vouches, kill walls) is fully banked; only the *attention* product of physically
  removing dead arms is forgone (display-tier dimming stands in). This is
  meaningfully more than 26B's (b) as written, and it needs neither substitution nor
  the scrub precondition. (-GUESS this becomes the v1 lane with (a) as the
  flag-or-later upgrade; the human owns it.)
- Either way: the reclassification-guards floor (`26B:rul-reclassification-guards-
  are-floor`) is reachable under both, and the arm-kill DST must-cover list
  (`26B:bank-deferred-lane-riders`) applies unchanged.

## §7 — The quiet-welding ledger (the audit the human asked for)

Each: where it is written, why the reactive direction breaks or bends it, and the
disposition. Annotations marked APPLIED were made on `ai/main` alongside this note;
the rest are revival-owned.

1. **`261` §2 h1, as amended by `26A:amend-h1-mechanism`** — "h1 edges resolve by
   exactly one of two mechanisms… (b) controller-fold consumption (…never by another
   shipped probe). Waves exist for width/pacing only." Written against the
   within-one-artifact wave mechanism; three crosscheck lanes hardened it into "the
   ONLY true edges" phrasing. Under `26B:rul-plan-construction-is-reactive` a third
   mechanism exists by design: cross-iteration re-compilation (fold the capture,
   re-analyze, ship a NEW artifact whose argv embeds it) — subject to §5's law.
   Reading h1(b) literally forbids the engine 26B mandates. DISPOSITION: supersession
   annotation on `261` §2 (APPLIED); within one artifact the two-mechanism rule
   stands unchanged.
2. **`261` §2 h2(iii)** — "staged round-trips REJECTED as a relevance mechanism,
   permanently." Still correct — but one careless read away from being cited against
   the reactive loop wholesale. The distinction: h2 is about *relevance* (which arms
   are live — recoverable in-artifact or paid for by bounded speculation waste); the
   reactive iteration crosses the network for *value-resolution and re-analysis*,
   which has no in-artifact alternative when the consumer is the controller's own
   analysis. DISPOSITION: clarifying annotation on `261` §2 (APPLIED).
3. **`260` §3 s3-2 + `22H` §3** — "classify once, fold per arrival"; "the static
   half is host-independent (+SURE, standing)." True at rung-0 and iteration 1;
   breaks the moment a host-captured value folds into the value plane: analysis
   states diverge per host, so classify/compile become per-(host × iteration). The
   fleet kernel's arrival-fold loop becomes an analyze-step loop (26B §2's kernel
   step); `build_plan`-per-batch generalizes to `A(book, oracles, R_host)`. Also:
   `260` §0's "shipped once — ~O(hosts) round-trips" becomes O(hosts × chain-depth),
   bounded by `26B:need-probe-want-diffing`'s termination argument. DISPOSITION:
   annotation on `260` §3/§0 (APPLIED); `22H` is historical and already carries its
   own downgrade banner — no edit.
4. **`262` §2 `attempt=` discard-wholesale** — correct for retries, wrong if reused
   for iterations (§4). DISPOSITION: annotation on `262` §2 (APPLIED);
   `26C:need-per-artifact-identity` + `iter=` additive key at the revival.
5. **`262` §1 spine-inv-order-free + pin-terminal-determinism** — the pins quantify
   over orderings of a fixed record-set; under reactivity the record-set itself is
   schedule-dependent through (i) the already-carved deadline weakening, (ii)
   cancellation (now bounded by §3's demotion-only rule), and (iii) the §1 conflict
   carve. The toward-run caveat generalizes to the elide≻guard≻run demotion order.
   DISPOSITION: annotation on `262` §1 (APPLIED); the revival re-states the pins as:
   fold-confluence unconditional; process-confluence conflict-free; demotion-only
   for every content-changing schedule policy.
6. **`27I`/`277` §5 "no probe-re-entrant back-edge at HEAD; plans mint once"** — the
   trivially-true pins stop being trivial. §2 above is the owed re-read: outcomes may
   schedule, never evidence; the universal meet needs no invalidation machinery
   under recompute-per-iteration. DISPOSITION: no doc edit (the clause anticipated
   exactly this); the revival's DST makes both pins non-trivially green.
7. **`rul-argv-flows-bytes-do-not` / `271:rul-only-oracle-bytes-ship`** — a third
   byte-provenance (host-spoken) enters argv position from iteration 2. Not a
   violation — the shipped bytes are still oracle bytes and the values still pass
   the argparse — but the law's two-party framing silently under-describes the
   surface. DISPOSITION: §5's law; wants a typed ack; spike/CLAUDE.md gains the
   bullet only when the revival actually builds (steering docs stay current-truth).
8. **`inv-site-keyed-results` + gate-1 parity + the one-fixture e2e shape** — the
   harness serves exactly one `probe-results.txt` per case and the cli reads results
   once to EOF (verified at tip). DISPOSITION: `26B:need-per-round-harness` stands;
   §9 recommends the in-memory tier carry reactive logic per the `24I` de-graduation
   doctrine, with ONE e2e multi-round exemplar family, not a corpus-wide reshape.
9. **`260` HostPhase linear ladder** (Pending→Connecting→Probing→Planned→…) —
   Probing becomes a Probing⇄Analyzing loop closed by the quiescence witness;
   `Planned` is only reachable through the witness
   (`26B:need-quiescence-witness-at-mint`). Also `260` §6's per-host liveness
   ("summary prints as that host's fold completes") re-keys from first-fold to
   per-host quiescence. DISPOSITION: annotation on `260` (APPLIED, folded into the
   item-3 note); mechanism at the revival.
10. **`27C` batching "one entered segment per (host, context)"** — becomes per
    (host, context, iteration); entry-form self-effects (sudo auth-log lines) accrue
    per iteration. Authored-residue ownership is unchanged
    (`27C:rul-probe-mutation-ownership-split`); the accrual is a UX/executor-era
    batching note, not a correctness item. DISPOSITION: none now; noted for the
    executor era. Capture wants inherit the *denoted context* of their site and key
    by (site, context) — `26B:need-context-qualified-captures` stands, satisfied by
    `FactKey.context` at tip.
11. **Diagnostics/why-lens under iteration** — a ⊤-cause disclosed at iteration k
    (e.g. `TopCause::WalledRead`, dynamic-value causes) can RESOLVE by iteration
    k+2; the final plan must not carry stale wall-warnings for lifted walls, and the
    kWARN-rich surface must not double-emit per iteration. Recompute-from-scratch
    gives the right behavior FREE (emit only the final state's diags); an
    incremental implementation would need retraction machinery — one more reason the
    recompute form is the v1 semantics (`26C:rul-diagnostics-final-state-only`,
    proposal-tier). Provenance composes the other direction: a fact learned at
    iteration k cites the iteration-(k−1) capture that made its probe compilable
    (`26B:need-provenance-through-rounds`), and the per-artifact nonce (§4) is the
    natural iteration coordinate for the why-lens chain. DISPOSITION: revival brief.
12. **Things checked and NOT in danger** (+SURE each, so nobody re-audits): the
    consent cut and everything apply-side (`rul-divergence-proceed`, the apply lane
    of `260` §4, no-reorder-ever); `kSTATE`/rec-5 (all iteration is intra-run;
    nothing persists); freeze-at-binding's patrolled window (`275` §5 — unchanged;
    multi-iteration probing widens the probe→apply TOCTOU residual only in the sense
    any slow probe already does, and toctou-scope's fence against freshness-window
    machinery STANDS); S0-evaluability and the commutation axiom (probes stay
    read-only; the world doesn't move from our side); `empty-world-byte-identical`
    (no oracles ⇒ no captures ⇒ no wants ⇒ the loop degenerates to today's single
    pass — worth pinning as the reactive rung-0); the `262` §4 policy ports (the
    scheduler consumes each iteration's want-set unchanged; LPT/cost tiers
    indifferent to which iteration minted a task).

## §8 — The revival implementation plan (re-cutting `262` S0/S1 + `260`/`261`)

Sequencing principle: the reactive semantics must be green *single-host, in-memory,
under DST* before transport/fleet fan-out consumes them — the batch driver surfaces
the semantic holes; the executor work surfaces the concurrency holes
(`26B:split-semantic-versus-concurrency-holes`); do not interleave the two hole
classes in one stage. The ladder (stage names new; absorbed stages cited):

- **R0 — skeleton, rig, ports** (= `262` S0, absorbed whole). One change of shape:
  the fleet kernel's step vocabulary includes, from day one,
  `ShipProbeArtifact{host, iter}` and (reserved, unimplemented) `CancelWorkItem`
  commands — reserving the loop in the event/command types costs nothing and avoids
  a v2 vocabulary break. The determinism rig, ports, and h1-edge extraction pass
  land exactly as specced (the extraction pass now ALSO feeds want-identity, §4).
- **R1 — the records lane + emission locus residue** (= `262` S1 minus what
  `wire-records-v1-import` landed): subshell isolation, wave barriers, the width
  flag — PLUS the §4 iteration keys (fresh nonce per artifact; `iter=` additive;
  per-artifact back-map routing at the fold). Gate: the landed deframer pins stay
  green; one new pin — records from two artifacts for one book fold into one
  accumulated state with no leafid aliasing.
- **R2 — the reactive core, single-host, in-memory** (NEW; the round's deep-thought
  half, alongside R0). The pure step `A(book, oracles, R) → (dispositions, wants,
  diags)`; the batch driver looping to quiescence; want-identity diffing;
  chain-depth termination; the plan-mint choke point taking the (trivial) quiescence
  witness. Its first want-generator is the capture fold — the read-value slice as
  struck from r27, rebuilt against `275` §4's validity table + the reversible floor
  + the `26B:bank-deferred-lane-riders` gates + §5's quoting law. **The binding-site
  gate (§6) is decided at R2 entry** — it is the first brief that cannot be written
  without it. The reclassification-guards floor lands here. DST: §9's battery.
- **R3 — fleet fan-out of the reactive engine** (= `260` stages 26-1…26-3,
  re-read): per-host partitions iterate independently (the welded firewall as a
  partition, `26B:seam-per-host-partition`); transport drivers; per-host quiescence
  gating the per-host print + mint; apply fan-out + failure taxonomy + severed-apply
  sentinel + host-identity measures land UNCHANGED (the apply is fenced; nothing
  reactive touches it). The 142-degraded wire posture (`26A:amend-wire-honesty`)
  carries; iteration multiplies sessions per host, which slightly strengthens the
  case for the ControlMaster reuse `260` §5 already specs.
- **R4 — policy, telemetry, and the yardstick** (= `261` P2–P4 + `260` 26-4/26-5):
  cost classes, LPT, `ms=`, `--verify`, `dorc why --host`. Economy-cancellation
  lands HERE, last, as pure policy on the already-green semantics: the
  wall-standing finality class + guard-by-vouch landing (§3), DST-pinned demotion-
  only. The makespan yardstick gains an iteration axis (chain-depth × width), and
  the `26B:ask-trial-counts-capture-walls` numbers (the trial having run by then,
  per `270` §5 ordering) size how much R2's lane actually bought.

Explicitly NOT this round (unchanged from the standing defers): executors
discharging work on-host (`26B:watch-dependent-chain-scheduling` — reserve, don't
privilege); cross-host synthesis; the kFACTS substrate flip (recompute-per-iteration
keeps it substrate-agnostic; any lean is an explicit decision —
`26B:watch-kfacts-substrate-lean`); TUI/live-render beyond per-host lines; anything
apply-side.

r27 meanwhile owes only the standing negatives (`26B` §5, re-verified at tip): the
choke point stays single (lane-integration must not fork it), the reserved seams
stay open, nothing new closes them.

## §9 — DST and harness plan

- **The battery (R2):** per-iteration record service from hostsim (it is a state
  oracle — it can answer any derived want); seeded arrival-order shuffling ACROSS
  iterations, not just within one batch; `pin-process-confluence-conflict-free`
  (byte-identical plans under shuffle, conflict-free seeds);
  `pin-conflict-soundness` (injected same-cell conflicts ⇒ conflicted cell ⊤ ⇒ its
  consumers guard/run; no assertion of byte-identity unless
  `26C:opt-justified-fact-gc` is chosen, in which case byte-identity returns and is
  asserted); `pin-empty-world-degenerate-loop` (no oracles ⇒ exactly one iteration,
  byte-identical to today); chain-depth termination (a depth-k capture chain
  quiesces in ≤ k+1 iterations); the §5 injection must-covers; the §4
  leafid-namespace pin; both `277` §5 pins re-proven non-trivially (a synthetic
  probe-re-entrant seed exists and the meet still collides on unknown members).
- **Harness shape:** the in-memory tier carries the reactive logic (`24I`
  de-graduation doctrine; the e2e corpus does NOT balloon). e2e gains ONE
  multi-round exemplar family: a book with a capture-dependent guard, hostsim-served
  across two iterations, gate-1 parity per-artifact at width 1. The authored
  single-`probe-results.txt` case shape stays valid for every single-iteration case
  (which is the whole corpus until an oracle vouches a capture).
- **Cancellation pins (R4):** cancelling inside the finality class never changes the
  elide-set (only reason-text/guard-vs-run within the demotion order); a seed that
  cancels outside the class is a rig ERROR (the gate is a precondition, not a
  policy preference).

## §10 — Asks for the human (slugged; none block r27)

1. **`26C:ask-ack-captured-bytes-law`** — §5's captured-bytes-ship-as-data law wants
   a typed ack (or correction) before the revival's R2 builds. It extends
   `rul-argv-flows-bytes-do-not` to a third byte-provenance and I judge it the one
   genuinely load-bearing NEW law in this note.
2. **`26C:ask-confluence-carve-choice`** — §1's menu: accept superset-variance under
   conflicts (my lean) vs justified-fact-GC for full reproducibility. Cheap to defer
   to R2 entry; recorded so it is chosen, not drifted into.
3. **`26C:ask-cancellation-posture`** — §3's package (demotion-only rule +
   guard-by-vouch landing + wall-standing finality class) as the standing answer to
   `26B:need-cancellation-finality-gate` — sanity-ack wanted, R4-timed.
4. **`26C:ask-annotation-review`** — the §7 APPLIED annotations touch `260`/`261`/
   `262` (plans are ahistorical; they were annotated, not rewritten, because the
   single-artifact readings remain correct for r27-era consumers). Skim-review at
   leisure; revert is one commit each.
5. *(Standing, restated for the round-close merge: `26B:ask-trial-counts-capture-walls`
   rides the trial revival; the binding-site gate stays deliberately un-ruled until
   R2 entry.)*

## §11 — Status table

| component | status |
|---|---|
| semantic model; fold-confluence; conflict-free process-confluence | +SURE (theory); spike-fit ~SUSPECT until R2 |
| confluence conflict-carve (the 26B §2 correction) | +SURE mechanism; ~SUSPECT real-world reachability; menu un-chosen (`ask-confluence-carve-choice`) |
| outcomes-schedule-never-evidence line; closure-pass fence | conductor synthesis; consistent with `277` §5's typed clause |
| cancellation package (demotion order · guard-by-vouch · finality class) | proposal-tier (`ask-cancellation-posture`) |
| wire iteration keys; want-identity; leafid namespaces | +SURE needed; shapes proposal-tier |
| captured-bytes-as-data law | NEW LAW, proposal-tier, ack wanted (`ask-ack-captured-bytes-law`) |
| binding-site gate menu incl. (b′) structure-preserving folds | sharpened, deliberately UN-ruled (human's, at R2 entry) |
| quiet-welding ledger + APPLIED annotations | audit complete against tip `e16b0c8`; annotations one-commit-revertible |
| R0–R4 revival ladder | proposal-tier; composes `262` S0/S1 + `260`/`261` without re-deciding their settled content |
