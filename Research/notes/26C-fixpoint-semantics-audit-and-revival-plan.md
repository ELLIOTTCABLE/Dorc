# 26C — The rerun-to-fixpoint engine: semantics worked through, the quiet-welding audit, and the revival implementation plan

AI-authored (Fable, design-rubber-duck pass, 2026-07-17) — minted into the r26 series
on the design lineage (`ai/main`), sibling to `notes/26B` (the sitting record +
human-typed rulings + obligations bank; its §1 rulings govern here). **Human-REVIEWED
same day, in-session: the full plan is acked**; human-typed rulings and governing
formulations are marked inline, everything else is conductor synthesis,
confidence-marked. Contents: (a) the reactive/rerun-to-fixpoint semantics made
precise, including one CORRECTION to 26B §2's confluence claim; (b) the audit the
human asked for — invariants and quiet weldings in the standing corpus/code that
become unsound, wrong-direction, or silently misleading under the reactive
direction, each with a disposition; (c) the R0–R4 implementation plan slotting the
reactive engine into the `262`/`260`/`261` stage ladders; (d) the typing discipline
(§8b) for building it bad-states-unrepresentable. Verified against the r27 build tip
(`ai/spike3-r27` @ `e16b0c8`, block-rebuild closed + block-context lanes 1–3 landed,
lane-integration in flight): the single-shot pipeline shape, the single plan-mint
choke point, `FactKey.context`, and the framed records lane are all as this note
assumes. Authority: root docs and 26B §1 outrank this. Companions: `26B` ·
`262`/`260`/`261` + `26A` · `275`/`219` · `27H`/`27I` · `plans/27C` · `277` §5 ·
`22H`.

Vocabulary note (settled in review; rationale §8b): the interned identity of a
probe-question is a **Question** (`Query` vetoed — hard collision with the Queries
effect-class); the host's per-question result is an **Answer**; the engine's
world-model holds **Beliefs** about **cells** (coordinate-named slots of world
state, the established `277` sense); "record" stays wire-tier (`dorc-records/1`);
"want" survives only as the predicate *derivable-but-not-yet-asked*. Earlier drafts
and the `262` annotation say "want-identity" — read it as question-identity.

## §0 — Charge, and the findings in one screen

The human's concern (2026-07-17, near-verbatim): agents have been designing *without*
the reactive direction in mind; there may be invariants, weldings, or *quiet* weldings
in code and adjacent design that become unsound with this long-deferred machinery.

Findings, ranked:

- **`26C:inv-record-admissibility-by-want`** (§1) — the genuinely load-bearing NEW
  invariant, ACKED: under reactivity, probes are minted from beliefs, and a belief
  can retract after its probe shipped — leaving a true Answer to a question the site
  no longer asks. Consuming it elides on a stale premise: under-execution, the
  cardinal sin. Governing form (human-typed): **"our facts are inherently
  argv-keyed"** — an Answer is reachable only through re-derivation of its Question.
  The single-shot pipeline never needed this stated ("record exists ⇒ argv
  resolved" held by construction), making it exactly the quiet-assumption class
  this audit exists to surface.
- **`26C:finding-confluence-needs-conflict-carve`** (§1) — 26B §2's "byte-identical
  plan under every ordering" is +SURE only for *conflict-free* runs. Same-cell
  belief conflicts (the `22H` §1 ⊤→Value→⊤ retraction) make question-minting
  path-dependent, so the gathered evidence itself can vary with arrival order.
  Soundness survives (via the admissibility invariant); byte-identity does not.
  RESOLVED direction-tier (human-typed): default = maximal correct elision; a
  flagged stability mode restores reproducibility for the consumers that need it.
- **`26C:need-captured-bytes-ship-as-data`** (§5) — ACKED HARD: cross-iteration
  probe-minting ships *host-produced bytes back out* inside later probe artifacts'
  argv — a new injection surface no standing law covered — plus the human's
  corollary, `26C:law-host-boundary-severs-provenance`: the host boundary severs
  the engine's provenance chain, so composition must fail toward unsureness.
- **`26C:finding-h1-rule-forecloses-iteration`** (§7 item 1) — the sharpest quiet
  welding found: the `26A`-amended `261` §2 h1 rule ("…never by another shipped
  probe") read literally forbids the reactive engine's core move. Annotated in
  place, with its within-one-artifact scope preserved.
- **`26C:finding-classify-once-breaks`** (§7 item 3) — `260` s3-2 "classify once"
  and `22H` §3's static-half host-independence silently break from iteration 2
  (host-captured values enter the value plane ⇒ per-host, per-iteration analysis).
  Cheap, but the fleet kernel's shape changes. Annotated in place.
- Smaller but real: iteration-vs-attempt keying on the records lane (§4); the
  license-demotion order generalizing `262` §1's toward-run caveat (§3);
  final-state-only diagnostics (§7 item 11); the sharpened binding-site menu,
  deliberately un-ruled (§6); the why-explanation lane as a new obligation (§5b).

What already holds and needs NO change (verified, +SURE): the sans-io fleet-kernel
shape (`260` §2) is reactive-ready as-is; `dorc-records/1` as landed (`27D` stage-5)
survives iteration with additive keys; the `27H`/`27I` representation is
reserve-shaped exactly as claimed (the foreclosure walk held); the plan-mint choke
point is a single call site at tip; the `277` §5 universal meet is fixpoint-robust
by construction; `26B:rul-consent-cut-absolute` fences the apply untouched.

## §1 — The semantic model

**State and driver.** Per host: `R` = the accumulated evidence-set (grow-only by
construction — an Answer or Belief-establishing measurement, once folded, is never
removed from `R`; retraction happens in *derived* state, never in `R`). The engine
is a pure function `A(book, oracles, R) → (dispositions, questions, diags)`. The
driver: fold arriving results into `R`, recompute `A`, ship the derivable Questions
not yet asked, repeat to quiescence, mint. Recompute-from-scratch per iteration is
the licensed implementation (`26B:rul-iteration-waste-acceptable`); the batch
pipeline is the degenerate schedule (26B §2, unchanged).

**Monotonicity, stated carefully.** The evidence-set is monotone; *derived beliefs
are not*: the belief-merge is a meet, so a cell's belief moves Unknown → Held(v) →
Conflicted when two measurements genuinely disagree (`22H` §1 — reachable via an
oracle adequacy bug or real drift mid-probe-phase). Conflicted is *sticky* (+SURE:
the disagreeing measurements stay in `R`, so no later measurement un-conflicts the
meet) — a belief can move Held(v) → Conflicted but never Held(v) → Held(v′). Three
consequences:

- **`26C:thm-fold-confluence-unconditional`** (+SURE) — for a FIXED evidence-set,
  the fold is order-free (commutative/idempotent/associative meet):
  `plan = f(R)` regardless of arrival order. This is `262` §1's
  pin-fold-permutation and pin-terminal-determinism unchanged — noting their
  quantifier: they range over orderings *of one evidence-set*, and say nothing
  about which evidence-set the process gathers.
- **`26C:thm-process-confluence-conflict-free`** (+SURE on the theory) — when no
  belief-conflict occurs during a run, the knowledge system is genuinely monotone
  (folds refine Unknown toward Held; killed arms only shrink walls), so by the
  classic chaotic-iteration result the question-set at quiescence is the unique
  least fixpoint: every ordering gathers the SAME final evidence ⇒ byte-identical
  plans under arrival shuffle. This is the honest form of 26B §2's advertised
  property.
- **`26C:finding-confluence-needs-conflict-carve`** (~SUSPECT on real-world
  reachability, +SURE on the mechanism) — when a conflict DOES occur, a transient
  belief can mint Questions the final state would not. Worked instance: `W` (an
  interposer whose `disturbs` hits `/etc/pkg`) sits above `PKG=$(cat /etc/pkg)`;
  the capture folds only while W is proven-converged-and-elided (an elided command
  casts no wall). W's cell transiently reads converged → the fold validates →
  iteration 2 ships `dpkg__predict -s 'nginx'` for site S → S's Answer arrives.
  Then the conflicting measurement lands → W's belief goes Conflicted → W runs →
  the wall stands → the capture un-folds → S's argv is ⊤ again. The evidence
  gathered along the way splits into two planes with OPPOSITE dispositions:

**The Answer plane (question-keyed) — the soundness hazard, closed
unconditionally.** S's Answer says "converged" — but it answers "is
`dpkg -s nginx` converged," and the apply-time site is `dpkg -s "$PKG"` with PKG
unknown: a true answer to a question the site no longer asks. Consuming it elides
on a stale premise — under-execution. Hence **`26C:inv-record-admissibility-by-want`**
(soundness-tier; holds in BOTH stability modes; ACKED, the governing form
human-typed: *"our facts are inherently argv-keyed"*): the value-plane feeds the
question, so everything the value-plane fed into the question must be soundly
carried — *completely* — into any consumption of the answer. Operationally: an
Answer is admissible at plan-mint iff its Question — (site · context · the resolved
compiled form) — re-derives from the FINAL knowledge state; "completely" is
load-bearing (the identity covers everything value-plane-fed, context included,
since a wrapper chain's denoted context can itself be value-plane-derived). Build
shape (confirmed in review): **key, don't filter** — the Answer store is keyed by
the full Question identity, so a retracted premise makes the stale Answer
*unreachable* rather than found-and-rejected; the illegal state is unrepresentable
and no mint-time check exists to forget (§8b). Cheap: stickiness means a
resolution moves known→unknown, never known→different-known, so re-derivation
either reproduces the key or produces none.

**The Belief plane (world-keyed) — sound, order-variant, and the only variance
remaining once admissibility holds.** The stray probe's measurements also
establish beliefs about world-cells, and the consumers of beliefs — wall
judgments, the capture patrol, fold-validity — ask questions about the WORLD; a
true measurement answers them soundly *whenever and for whatever reason it was
gathered*. The license predicate consults measurement-truth plus static authored
claims, never the motivation for asking. So an extra belief can validate a
capture-fold (⇒ downstream elisions) the other ordering would have left walled —
and both outcomes are correct, because:

**"The correct set of elisions" is not a unique set.** Elision-licensing is a
predicate over (plan, evidence), not a function of the world: the engine never
promises the world-maximal elidable set, only that every printed elision is
individually licensed by true evidence. The licensed set was ALWAYS
evidence-contingent — a richer oracle library elides more; a probe deadline elides
less (`262` §1's own sanctioned weakening); a refused entry-form elides less — and
nobody calls the deadline-shrunk plan incorrect. Under-elision is the permanent
safe floor (unnecessary-execution is the LOWEST-ranked sin; "run everything" is
always a correct plan). What conflict-transients add is a *determinism* axis, not
a correctness one: two identical invocations on an identical world can gather
different evidence and print different, both-correct plans — a trust/UX regression
against the spirit of `262:spine-inv-order-free`, confined to runs that were
already anomalous (a conflict means an oracle lied or the world moved mid-probe).

**The resolution (human-typed; formal confirmation rides R2 entry):**

- **Default = maximal correct elision** (`26C:opt-accept-superset-variance`):
  capture the value; the conflicted cell itself always lands Conflicted ⇒ runs;
  reproducibility-under-anomaly was never promised. Plan-shape stability under the
  default is honest only *modulo conflict* — scheduled plan-comparison consumers
  (the `24R` cron story, `--exit-code`-adjacent tooling) are documented onto the
  flag below, never onto an implied determinism the default doesn't promise.
- **Flagged stability mode** (`26C:opt-justified-fact-gc`; spelling STRAWMAN, a
  `--stable-plan`-class flag) for reproducible-plan consumers (the dorc-ci shape).
  Mechanism — the replay argument (+SURE it is order-free): at quiescence, replay
  question-derivation from the final evidence-set in canonical batch-synchronous
  rounds (round 0 admits all Answers matching the no-facts question-set,
  re-derive; round 1 admits matches; … to fixpoint; discard the rest). In the
  replay, both conflicting measurements land in the SAME round, so the cell is
  Conflicted before any later round's questions derive — transient-motivated
  questions never re-derive, their Answers are discarded, and the kept subset is a
  pure function of the evidence's *content*, order-erased. Discarding true
  evidence only demotes sites toward run — the safe direction. Plans become
  byte-identical unconditionally under the flag.
- **Testing** (with the human's correction incorporated): DST exists to test the
  product, so the DEFAULT mode is the primary battery — no paradox, because the
  rig is seed-deterministic (`inv-determinism`: rerun any seed ⇒ bit-identical);
  variance lives ACROSS seeds, where the pins are per-seed reproducibility +
  cross-seed soundness envelope + admissibility + conflict-free byte-identity.
  The flagged mode contributes one ADDITIONAL pin family: unconditional
  byte-identity under shuffle, plus the cross-mode differential oracle
  *filtered-plan elisions ⊆ unfiltered-plan elisions* — a secondary net, never
  the primary surface.

## §2 — The line between scheduling and evidence (the `277` §5 re-read, delivered)

`27I` closed with: the fixpoint clause and `pin-no-outcome-as-generator` hold
trivially at HEAD because no probe-re-entrant back-edge exists; "the post-probe
re-bind re-reads this clause the day it is designed." This section is that re-read.
The precise line (+SURE; the human's banked `277` §5 concern was exactly
outcome-laundering under re-run-to-fixpoint):

- **Outcomes MAY schedule work.** Beliefs, resolved values, killed arms, and even
  dispositions may drive question-minting and cancellation — the reactive engine's
  entire point. Scheduling consumes knowledge and manufactures none: a shipped
  probe returns a *measurement*, and only the measurement enters the evidence.
- **Outcomes may NEVER re-enter as evidence.** A compare-verdict feeds only its
  licensed consumer and never becomes input to a later compare
  (`pin-no-outcome-as-generator`, unchanged); a *disposition* never feeds the
  sparing/transport relation, the belief plane, or a license mint. The universal
  meet stays the fixpoint-robust form: at every intermediate state a
  not-yet-resolved member reads unknown ⇒ collide, so partial iteration states
  can never spare more than the final state would (`277` §5, unchanged — and why
  recompute-per-iteration needs no compare-result invalidation machinery).
- One NEW fence this implies for the fallback-carry work landing in r27:
  **`26C:fence-capture-never-feeds-closure-pass`** — the `27C` §4(a)-(B)
  read-set-closure pass proves closure over *marks and sh structure*; a
  capture-resolved value must never be fed to it to "close" a body that reads an
  unmarked input. Closure is structural forever; captures resolve *argv*, never
  *closure* (~SUSPECT nobody intends otherwise, but the two mechanisms will
  coexist in one codebase and the laundering path is one careless refactor wide).

## §3 — Cancellation (the finality gate given mechanism; package ACKED)

Governing frame first (standing law restated — `kHALVES`, rul-attention-honesty):
**the attention-ack-wall is absolute.** Guards never buy attention; landing on a
guard is never a *solution* nor a licensed trade, and shifting work to guard-time
is never an answer to probe cost. Scope (human's): cancellation is exclusively
*shedding needed-then-disnecessitated work* — a probe minted because the plan
required it, later obviated by a parallel fact. Speculative add-then-cancel
architectures take no license from this section.

- **The license-demotion order** (`26C:rul-candidate-demotion-only-cancellation`):
  generalize `262` §1's deadline caveat ("timing policy moves content only toward
  run") to the named-mechanism ladder — **elide ≻ guard ≻ run**; any
  timing/cancellation policy may only DEMOTE a site along it, never promote. Same
  safety direction (kFAIL-perform: less license, more execution); the `262`
  annotation carries the generalization.
- **The finality gate, attention-first** — its real content: **economy-cancellation
  may never cost a possible elision.** A probe that could still remove a line from
  the plan is never cancelled, however slow. The cheaply-derivable qualifying
  class (`26C:mech-wall-standing-finality-class`, +SURE): a site downstream of a
  *confirmed-running unmodeled wall* — an unmodeled site always runs (no oracle ⇒
  no vouch), a running unmodeled wall casts ⊤ with no footprint under any flag, so
  every downstream site's ceiling is guard, permanently within the run. There, the
  line renders guard-or-run — VISIBLE — under every possible answer the probe
  could have returned: cancellation moves nothing the admin would ever have been
  spared. Finality must rest on retraction-proof grounds: unmodeledness (static)
  and Conflicted beliefs (sticky) qualify; a merely-Held divergence does not
  (-GUESS on the full class inventory, the revival's precision pass owns it).
- **Where a cancelled site rests** (`26C:mech-cancel-lands-on-guard-by-vouch`): not
  bare run — a cancelled measurement is a can't-say, and for a vouched site the
  guard tier licenses off the vouch alone
  (`26B:rul-reclassification-guards-are-floor`). So the site disposes GUARD: the
  check runs once at apply and does the right thing live — which is the motivating
  example's own economics ("the expensive check will run at apply-time anyway"),
  and makes cancellation nearly plan-shape-free (reason-text differences only).

## §4 — Iteration mechanics on the wire (the `262` §2 delta)

The landed `dorc-records/1` (`27D` stage-5) survives reactivity with additive
changes only (+SURE, verified against the deframer at tip):

- **`26C:need-per-artifact-identity`** — each shipped probe artifact gets its own
  wire identity: a fresh `nonce=` per artifact (already edge-minted and DI'd) plus
  an additive `iter=<n>` header key for render/why legibility. `attempt=` keeps
  its existing meaning — *retry of one artifact*, discard-prior-wholesale — and is
  never overloaded for iterations, whose results ACCUMULATE. The two semantics are
  opposites; conflating them would throw away the knowledge base.
- **`26C:need-per-artifact-leafid-namespace`** — LeafIds are per-compile;
  iteration N+1's artifact re-derives them with no stability guarantee against
  iteration N's numbering. Results therefore resolve through *their own
  artifact's* manifest — the engine-side map `(Nonce, LeafId) → QuestionId`,
  minted at compile — and cross-iteration accumulation happens in
  question/cell space, never leafid space. Consequence worth stating: **the host
  never names a Question** — it answers coordinates the engine minted, so a
  hostile stream cannot forge or replay a question-identity (composes with §5).
  The header's `sites=` census stays per-artifact; the lane-split truncation
  semantics apply per-artifact unchanged.
- **`26C:need-want-identity-diffing`** (name predates the vocabulary note; read
  question-identity) — "ship what's newly derivable" keys on the full interned
  Question, not the site: `derivable(state) − asked`, a set-difference over ids
  (§8b). Identity-keyed diffing is never wrong; site-keyed diffing is wrong the
  day two distinct resolutions of one site are both legitimate (+SURE the
  identity form is safe; ~SUSPECT the site-keyed form is even reachable-wrong,
  given stickiness — belt-and-suspenders is cheap here).
- **Fold semantics across iterations**: same-cell beliefs from different sites
  across iterations merge by the existing meet (that IS the legitimate conflict
  channel of §1). Same-leafid duplicates cannot occur across artifacts (namespaces
  disjoint). Nothing else changes.

## §5 — Captured bytes: the two laws (both ACKED)

From iteration 2 onward, the probe compiler embeds *host-produced bytes*
(iteration-1 captures) into a shipped artifact's argv: `PKG=$(cat /etc/pkg)`
captures `nginx`; iteration 2 ships `dpkg__predict '-s' 'nginx'`. The standing
byte-provenance law covers two parties (oracle bytes ship —
`271:rul-only-oracle-bytes-ship`; admin argv flows through the argparse —
`rul-argv-flows-bytes-do-not`; the admin's bytes never ship). Captures mint a
THIRD provenance — host-spoken bytes, the `275` §7 hostile-host hook widening from
cell-naming into shipped-executable context. A compromised or merely-weird tool
emitting `` nginx; rm -rf / `` must land in the next probe as an inert argv token,
never as sh.

**`26C:need-captured-bytes-ship-as-data`** (ACKED HARD, human-typed):

1. A captured value re-enters a shipped artifact ONLY in argv position of an
   oracle-authored function, single-quoted through the engine's one quoting home
   (`sem::single_quote`-class, `'`-escaping included) — never concatenated into
   command position, code position, redirect targets, or heredoc bodies.
2. The single-line wire floor (`275` §10) is load-bearing here, not just for the
   wire: multi-line captures refuse the fold *and* therefore never ship onward;
   newline rejection happens at capture, before anything downstream can consume.
3. The oracle's own argparse remains the type-checker (declines included) — a
   captured value the argparse rejects lands can't-say ⇒ run, exactly like a
   weird admin argv.
4. Injection-safety holds at EVERY iteration's ship, not only at mint —
   intermediate artifacts are just as host-visible as final ones.
5. DST must-covers: captured values containing `'`, spaces, `$`, backticks, glob
   chars, leading `-`; a multi-line capture refusing; a capture consumed in the
   same iteration's why-text rendering safely.
6. **`26C:law-host-boundary-severs-provenance`** (the human's corollary,
   near-verbatim): inside the engine a value carries an attribution/provenance
   type with metadata; the moment `'nginx'` is minted into a shipped script and
   fed as argv, **the provenance chain is severed** — the consuming command's
   output may be a modified form of the input, and which input components and
   what global state contributed is unknowable. Composition must be clean and
   must fail toward unsureness. Concretely, for chains (`A=$(f); B=$(g "$A")`):
   B's value is `g_host(A, world_g)`, engine-opaque — so (i) B's backing/grade
   derive from g's oracle marks and delegation structure (`275` §2, unchanged),
   never inherited from A; (ii) B's fold-VALIDITY composes transitively
   engine-side through the recipe: B folds only if every capture in its input
   recipe folds, and B's patrol-surface is the UNION of its producer's backing
   with its input captures' backings (frozen-B stands for apply-time-B only while
   frozen-A stands for apply-time-A — the `275` §5 question-axis, chain edition;
   a chain can never be part-frozen-part-live); (iii) any un-derivable link is ⊤
   and walls the fold. A host transformation never launders provenance.
   **Scope-fence: this clause governs the CLAIMS/LICENSE lanes only** — the
   explanation product carries the opposite obligation (§5b).

(~SUSPECT the clause set is complete — the revival's adversarial crosscheck should
treat this section as a target, exclusions-not-inclusions per standing practice.)
This law also evens out `26B:watch-dependent-chain-scheduling`'s asymmetry: the
on-host chain (executor-era) and controller-side iteration share the same
host-bytes-into-argv exposure; the mitigations differ, the quoting law binds both.

## §5b — The why-lens through the black box (human-typed direction; exploratory tier)

The severed-provenance law is correct for claims-driving and correctness, and
*cannot be allowed to be* the `dorc why` story: "your `dorc why` output only tells
the tale back to the most recent re-issue / value-fold" is — human's words — not
remotely good enough. Under rerun-to-fixpoint the why-chain must narrate a value
like `'nginx'` THROUGH iteration hops and host-side transformations, best-effort,
even though nothing in that narration may ever feed the license plane. The two
lanes fail in OPPOSITE directions by design: the license lane fails toward
unsureness (§5 clause 6); the explanation lane fails toward *narration with
attributed confidence*. This deepens `26B:need-provenance-through-rounds` into a
real sub-design: **`26C:need-why-explanation-lane`**.

What the engine gets FREE and sound (+SURE): the *structural* spine already spans
the boundary — recipes record that B was computed by `g "$A"`, Question identities
record which capture made which probe compilable, per-artifact nonces give the
iteration coordinate. The black box's inputs and outputs are engine-known even
where the transformation is not; the genuinely best-effort part is *contentful*
narration of what happened inside.

The candidate feeder classes (human-typed, near-verbatim; each display-plane-only,
each attributed BY CLASS in the why output):

- **`26C:feeder-non-referent-agnostic-text`** — static analysis vetoed everywhere
  else (`inv-referent-agnostic`): tool-aware textual reasoning ("`grep` filters
  its input's lines"; "the output contains the captured string") producing
  inferred-from-text-shape links.
- **`26C:feeder-host-observation-hot-path`** — powerful host machinery watching
  what *actually* happens, where available: the punted lint/mechanical-
  verification/tracer class (the `077` seccomp observe backstop; the
  `kFIDELITY-faithful` one-leaf-one-exec seam; the DX-tooling tracer ambitions)
  entering the probe HOT PATH — explicitly, specifically best-effort, producing
  provenance/logging/value-tracking that feeds `dorc why` only.
- **`26C:feeder-oracle-why-metadata`** — the dangerous one, meta-tier: oracles
  *contributing to* (never driving) the why-chain. Oracles are the very
  untrustworthy source `dorc why` exists to tattle on, so their contributions
  render as claimed-by-oracle-X, forever display-tier. OPEN CORNER, flagged not
  solved: what an authored why-contribution's *spelling* is under the `kOOB`
  redline (much of the need may already be served by existing marks + predicts;
  the residue is the question).

The fence that makes the backflips safe to attempt: the decision-inert plane
already EXISTS as typed precedent — ru-11's `OriginKind` ("grounds the why-lens
EXPLANATION, never a decision", `22H` §1) and the `27L` sealed `core::room` split
(`into_license_input()` exists only on Invited; compile-fail pinned). The
explanation lane's evidence type gets the same sealed treatment from day one: no
conversion into any license-plane input exists, at the type level — the vetoed
analyses stay vetoed where the veto matters, and
`26C:fence-capture-never-feeds-closure-pass` (§2) is one instance of the general
rule. Multiple feeders in concert, disagreeing gracefully, each tagged with class
and confidence, is the expected end-shape; the revival owns sizing it, and the r25
trial's `dorc why` output critique is the natural evidence for how much narration
depth is actually owed.

## §6 — The binding-site coherence gate, sharpened (deliberately NOT ruled)

`26B:gate-binding-site-coherence` stays THE revival's opening design-question and
stays un-ruled (human's choice). This section only sharpens the menu:

- **(a) freeze-in-artifact, reframed** — substituting the frozen value is not a
  new mechanism: it is the ordinary Replace applied to the *assignment leaf* —
  `PKG=$(cat /etc/pkg)` → `PKG='nginx'` — a StandIn reproducing the assignment's
  observable from probe-provenance bytes, exactly the `inv-probe-sourced-values`
  licensed shape (`219` q-3.f anticipated this as render-substitute-assignment).
  Under it the `279f` live-consumers hard gate *dissolves rather than binds*:
  apply-time consumers outside the folded region read the frozen binding,
  consistent with the killed arms by construction. Preconditions dragged in: the
  world-spoken floor (pre-pinned), `26B:need-scrub-before-freeze` (host bytes
  enter rendered artifacts for the first time — the secrets sequencing
  dependency), the §5 quoting law, and the render's single-line span-edit care.
  Divergence-*detection* (a live re-read compared report-only) composes cleanly.
- **(b′) structure-preserving folds** — the refinement of 26B's all-or-nothing
  option, and meaningfully more valuable: keep the binding LIVE and the case
  structure intact; per-site elisions *inside* arms remain sound without
  arm-removal, PROVIDED every modeled arm's sites got honest dispositions — which
  h2-speculation already provides (probes ship for all modeled sites regardless
  of arm liveness). A live re-capture that diverges dispatches into a *different
  arm whose sites also carry their own licenses* — no amputated structure, no
  silent fall-through. The capture's analysis value (un-⊤ the scrutinee, resolve
  downstream argv, reach vouches, kill walls) is fully banked; only the attention
  product of physically removing dead arms is forgone (display-tier dimming
  stands in). Needs neither substitution nor the scrub precondition. (-GUESS this
  is the v1 lane with (a) as the upgrade; the human owns it, at R2 entry.)
- Either way: the reclassification-guards floor is reachable under both, and the
  arm-kill DST must-covers (`26B:bank-deferred-lane-riders`) apply unchanged.

## §7 — The quiet-welding ledger (the audit)

Each: where written, why the reactive direction breaks or bends it, disposition.
Annotations marked APPLIED were made on `ai/main` alongside this note.

1. **`261` §2 h1, as amended by `26A:amend-h1-mechanism`** — "h1 edges resolve by
   exactly one of two mechanisms… (b) controller-fold consumption (…never by
   another shipped probe). Waves exist for width/pacing only." Written against
   the within-one-artifact wave mechanism; three crosscheck lanes hardened it
   into "the ONLY true edges." Under `26B:rul-plan-construction-is-reactive` a
   third mechanism exists by design: cross-iteration re-compilation (fold the
   capture, re-analyze, ship a NEW artifact whose argv embeds it) — subject to
   §5. Read literally, h1(b) forbids the engine 26B mandates. DISPOSITION:
   supersession annotation on `261` §2 (APPLIED); within one artifact the
   two-mechanism rule stands unchanged.
2. **`261` §2 h2(iii)** — "staged round-trips REJECTED as a relevance mechanism,
   permanently." Still correct — but one careless read from being cited against
   the reactive loop wholesale. The distinction: h2 is *relevance* (which arms
   are live — recoverable in-artifact or paid by bounded speculation); the
   reactive iteration crosses the network for *value-resolution and re-analysis*,
   which has no in-artifact alternative when the consumer is the controller's own
   analysis. DISPOSITION: clarifying annotation on `261` §2 (APPLIED).
3. **`260` §3 s3-2 + `22H` §3** — "classify once, fold per arrival"; "the static
   half is host-independent (+SURE, standing)." True at rung-0 and iteration 1;
   breaks the moment a host-captured value folds into the value plane: analysis
   states diverge per host, so classify/compile become per-(host × iteration).
   The fleet kernel's arrival-fold loop becomes an analyze-step loop
   (`A(book, oracles, R_host)`); `260` §0's "shipped once — ~O(hosts)
   round-trips" becomes O(hosts × chain-depth), bounded by the §4 termination
   argument. DISPOSITION: annotation on `260` §3/§0 (APPLIED); `22H` is
   historical with its own downgrade banner — no edit.
4. **`262` §2 `attempt=` discard-wholesale** — correct for retries, wrong if
   reused for iterations (§4). DISPOSITION: annotation on `262` §2 (APPLIED);
   `26C:need-per-artifact-identity` at the revival.
5. **`262` §1 spine-inv-order-free + pin-terminal-determinism** — the pins
   quantify over orderings of a fixed evidence-set; under reactivity the
   evidence-set is schedule-dependent through (i) the already-carved deadline
   weakening, (ii) cancellation (bounded by §3's demotion-only rule), and (iii)
   the §1 conflict carve. The toward-run caveat generalizes to the elide≻guard≻run
   demotion order. DISPOSITION: annotation on `262` §1 (APPLIED); the revival
   re-states the pins per §1's theorems.
6. **`27I`/`277` §5 "no probe-re-entrant back-edge at HEAD; plans mint once"** —
   the trivially-true pins stop being trivial; §2 is the owed re-read (outcomes
   may schedule, never evidence; recompute-per-iteration needs no invalidation
   machinery). DISPOSITION: no doc edit (the clause anticipated this); the
   revival's DST makes both pins non-trivially green.
7. **`rul-argv-flows-bytes-do-not` / `271:rul-only-oracle-bytes-ship`** — a third
   byte-provenance (host-spoken) enters argv position from iteration 2. Not a
   violation — shipped bytes are still oracle bytes and values still pass the
   argparse — but the two-party framing under-describes the surface.
   DISPOSITION: §5's laws (ACKED); spike/CLAUDE.md gains the bullet when the
   revival builds (steering docs stay current-truth).
8. **`inv-site-keyed-results` + gate-1 parity + the one-fixture e2e shape** — the
   harness serves exactly one `probe-results.txt` per case and the cli reads
   results once to EOF (verified at tip). DISPOSITION:
   `26B:need-per-round-harness` stands; §9 keeps reactive logic in the in-memory
   tier per the `24I` de-graduation doctrine, with ONE e2e multi-round exemplar
   family.
9. **`260` HostPhase linear ladder** — Probing becomes a Probing⇄Analyzing loop
   closed by the quiescence witness (`26B:need-quiescence-witness-at-mint`);
   `260` §6's per-host liveness print re-keys from first-fold to per-host
   quiescence. DISPOSITION: folded into the item-3 annotation (APPLIED);
   mechanism at the revival.
10. **`27C` batching "one entered segment per (host, context)"** — becomes per
    (host, context, iteration); entry-form self-effects (sudo auth-log lines)
    accrue per iteration. Ownership unchanged
    (`27C:rul-probe-mutation-ownership-split`); a UX/executor-era batching note,
    not correctness. Capture Questions inherit their site's *denoted context* and
    key by (site, context) — `26B:need-context-qualified-captures` stands,
    satisfied by `FactKey.context` at tip. DISPOSITION: none now.
11. **Diagnostics/why-lens under iteration** — a ⊤-cause disclosed at iteration k
    (e.g. `TopCause::WalledRead`) can RESOLVE by iteration k+2; the final plan
    must not carry stale wall-warnings for lifted walls, nor double-emit per
    iteration. Recompute-from-scratch gives the right behavior FREE (emit only
    the final state's diags — `26C:rul-diagnostics-final-state-only`,
    proposal-tier); an incremental implementation would need retraction
    machinery — one more reason recompute is the v1 semantics. Forward
    provenance composes via §5b's structural spine. DISPOSITION: revival brief.
12. **Checked and NOT in danger** (+SURE each, so nobody re-audits): the consent
    cut and everything apply-side (`rul-divergence-proceed`, `260` §4's apply
    lane, no-reorder-ever); `kSTATE`/rec-5 (all iteration intra-run; nothing
    persists; ids never serialized); freeze-at-binding's patrolled window
    (`275` §5 — unchanged; multi-iteration probing widens the probe→apply TOCTOU
    residual only as any slow probe already does; toctou-scope's fence against
    freshness-window machinery STANDS); S0-evaluability and the commutation axiom
    (probes stay read-only); `empty-world-byte-identical` (no oracles ⇒ no
    captures ⇒ one iteration — the reactive rung-0, pinned in §9); the `262` §4
    policy ports (the scheduler consumes each iteration's question-set unchanged).

## §8 — The revival implementation plan (re-cutting `262` S0/S1 + `260`/`261`)

Sequencing principle: the reactive semantics must be green *single-host,
in-memory, under DST* before transport/fleet fan-out consumes them — the batch
driver surfaces the semantic holes; the executor work surfaces the concurrency
holes (`26B:split-semantic-versus-concurrency-holes`); never interleave the two
hole classes in one stage.

- **R0 — skeleton, rig, ports** (= `262` S0, absorbed whole). One change of
  shape: the fleet kernel's command vocabulary includes, from day one,
  `ShipProbeArtifact{host, iter}` and (reserved, unimplemented) `CancelWorkItem`
  — reserving the loop in the event/command types costs nothing and avoids a v2
  vocabulary break. The h1-edge extraction pass lands as specced and also feeds
  Question identity (§4).
- **R1 — the records lane + emission locus residue** (= `262` S1 minus what
  `wire-records-v1-import` landed): subshell isolation, wave barriers, the width
  flag — PLUS the §4 iteration keys (fresh nonce per artifact; `iter=` additive;
  manifest-routed folding). Gate: the landed deframer pins stay green; one new
  pin — two artifacts for one book fold into one accumulated state with no
  leafid aliasing.
- **R2 — the reactive core, single-host, in-memory** (NEW; the deep-thought half
  alongside R0). The pure step `A(book, oracles, R) → (dispositions, questions,
  diags)`; the batch driver looping to quiescence; the §8b type substrate
  (Questions arena, Answers, Beliefs); question-identity diffing; chain-depth
  termination; the plan-mint choke point taking the (trivial) quiescence
  witness. Its first question-generator is the capture fold — the read-value
  slice as struck from r27, rebuilt against `275` §4's validity table + the
  reversible floor + `26B:bank-deferred-lane-riders` + §5's laws. **The
  binding-site gate (§6) is decided at R2 entry** (the first brief unwritable
  without it), and the §1 stability resolution is formally confirmed there. The
  reclassification-guards floor lands here. DST: §9's battery.
- **R3 — fleet fan-out of the reactive engine** (= `260` stages 26-1…26-3,
  re-read): per-host partitions iterate independently (the welded firewall as a
  partition, `26B:seam-per-host-partition`); transport drivers; per-host
  quiescence gating the per-host print + mint; apply fan-out + failure taxonomy
  + severed-apply sentinel + host-identity measures land UNCHANGED (the apply is
  fenced). The 142-degraded wire posture carries; iteration multiplies sessions
  per host, slightly strengthening `260` §5's ControlMaster case.
- **R4 — policy, telemetry, yardstick** (= `261` P2–P4 + `260` 26-4/26-5): cost
  classes, LPT, `ms=`, `--verify`, `dorc why --host`. Economy-cancellation lands
  HERE, last, as pure policy on green semantics: the §3 package (acked),
  DST-pinned demotion-only. The makespan yardstick gains an iteration axis
  (chain-depth × width); `26B:ask-trial-counts-capture-walls` sizes what R2's
  lane bought, and the trial's `dorc why` critique sizes §5b's narration depth.

Explicitly NOT this round (standing defers): executors discharging work on-host
(`26B:watch-dependent-chain-scheduling` — reserve, don't privilege); cross-host
synthesis; the kFACTS substrate flip (recompute keeps it substrate-agnostic;
any lean is an explicit decision — `26B:watch-kfacts-substrate-lean`); TUI/live
render beyond per-host lines; anything apply-side.

r27 meanwhile owes only the standing negatives (`26B` §5, re-verified at tip):
the choke point stays single, the reserved seams stay open, nothing new closes
them.

## §8b — The typing discipline (bad-states-unrepresentable in practice; shapes STRAWMAN, discipline firm)

How "facts are inherently argv-keyed" is spelled in Rust, and how DAG-sized keys
compare in O(1). Spike-idiom constraints honored: no unsafe, no macros, no
HashMap iteration, newtypes + module privacy as the enforcement mechanism.

- **`26C:disc-hashcons-want-identity`** — the key for a derivation DAG is its
  *hash-consed name* (Filliâtre/Conchon; salsa's interned queries; git's
  content-addressing as the anchor — a commit id names a whole history): every
  DAG node is a shallow struct whose children are ids; each node is interned
  (BTree-backed, deterministic order) in a per-host `Questions` arena; the whole
  question's identity is one `QuestionId(u32)`. Construction O(nodes) amortized;
  equality O(1); the ship-diff is `derivable − asked` over
  `BTreeSet<QuestionId>`. The arena persists across iterations SAFELY because it
  caches *names, not knowledge* — content-addressed names cannot go stale, so
  recompute-from-scratch re-derives the same ids for free. Sketch:

  ```rust
  pub struct QuestionId(u32);                    // private field; no from_raw, ever
  struct QuestionNode { site: SiteId, ctx: CtxNormId, form: FormId }
  enum FormNode { Predict { provider: ProviderId, argv: ArgvId },
                  Composed { stages: Box<[FormId]> },
                  Entered  { entry: ProviderId, inner: FormId } }
  enum ArgvNode { Lit(Symbol) }                  // post-resolution ONLY
  ```

  (~SUSPECT this node vocabulary is complete for R2's first slice; a starting
  shape, not a spec.)
- **`26C:disc-intern-normal-forms-only`** — intern denotations, never syntax:
  argv as post-resolution literals; contexts as the `27C` folded per-dimension
  normal form (`CtxNormId` — so `sudo -u postgres nice` and `nice sudo -u
  postgres` share one key by the existing ruling); compiled forms after
  composition. The normalization functions are the only producers of internable
  nodes (private constructors). Interning pre-normalization causes only spurious
  MISSES — the safe/wasteful direction — but the discipline is
  normal-forms-only.
- **`26C:disc-key-by-extension`** — the key holds the question's *extension*
  (resolved bytes: `argv=['-s','nginx']`), never its *intension* (which capture
  chain produced 'nginx'). Sound because Answers are world-facts, indifferent to
  why they were asked, and it maximizes sharing. Admissibility still bites
  because the extension is only REACHABLE through the intension:
  `Questions::derive(&Knowledge, SiteId) -> Option<QuestionId>` resolves argv
  and context through the live value-plane — a retracted premise ⇒ resolution
  fails ⇒ no key derives. Key by extension, narrate by intension (the
  premise-trace lives in the why/provenance plane).
- **`26C:disc-three-moats`** — unrepresentability is three private-constructor
  moats, each removing a *way of naming* rather than adding a check:
  1. `QuestionId` is unforgeable — `derive()` is the sole mint. Plan-mint reads
     `answers.get(arena.derive(&final_state, site)?)` — **the `?` IS the
     admissibility invariant**; no check exists to forget. (The signature-is-law
     style of the existing `claim-tier-gating`/`ByVouch`-by-value idiom.)
  2. The wire never speaks keys — the per-artifact manifest
     `(Nonce, LeafId) → QuestionId` is engine-minted at compile; hosts answer
     coordinates and can neither forge nor replay a question-name; no canonical
     key serialization ever crosses the boundary (§4).
  3. The Answer store has no other door — `Answers: BTreeMap<QuestionId,
     Answer>`, get-by-QuestionId only; no site-indexed view exists, so
     "whatever answer this site has" is not writable.
- **`26C:disc-two-stores-two-laws`** — the §1 planes as two containers with
  incompatible key types (itself the fence):

  ```rust
  pub struct Answers(BTreeMap<QuestionId, Answer>);     // admissibility-by-derivability
  pub struct Beliefs(BTreeMap<Coordinate, Belief<Claim>>); // meet-fold; walls/patrol consumers
  pub enum Belief<T> { Unknown, Held(T), Conflicted }
  ```

  The meet sends disagreeing `Held`s to `Conflicted`, which is absorbing —
  stickiness becomes STRUCTURAL (no function returns Conflicted→Held).
  `Conflicted ≠ Unknown` pays twice: the sticky meet, and finality (Conflicted
  is retraction-proof). Care: license consumers see both as ⊤
  (`inv-top-reject`: never branch on which-⊤) — enforce by giving `Belief` a
  single license-facing `fn held(&self) -> Option<&T>` and keeping the full
  enum crate-private; only the why-lane and the finality module distinguish.
- **`26C:disc-finality-witness-grade`** — the §3 gate as a witness pair
  mirroring the `inv-must-may` one-way coercion: `Final<T>` / `Provisional<T>`,
  constructors private to a finality module, each `Final` constructor consuming
  only retraction-proof inputs (static structure; unmodeledness;
  `Belief::Conflicted`). `Final<T> → T` free; `Provisional` never upgrades;
  `cancel(q: QuestionId, ceiling: Final<CeilingIsGuardOrRun>)` demands the
  witness *by value* — cancel-on-provisional-judgment does not compile.
- **`26C:disc-schedule-evidence-seal`** — §2's line at type level: `Command`
  (Ship/Cancel/Mint) is a SINK plane; no `From<Disposition>` or compare-verdict
  constructor exists on any evidence/belief/answer type; pinned with a
  compile-fail doctest per the `27L` `core::room` precedent.
- **Coherence "from elsewhere", four cases** — cross-iteration: same arena ⇒
  same ids, automatic. Wire: manifest association, never re-derivation from host
  bytes. Syntax variants: normal-forms-before-interning. Cross-host: one arena
  PER host inside the per-host state — `260` s3-1's no-cross-host-API discipline
  makes id confusion unrepresentable-by-absence; debug-tier generation stamps
  (a DI'd counter, never ambient — `inv-determinism`) as the belt. (Branded
  lifetimes considered and rejected: the theoretically-clean generativity trick
  fights stable Rust without unsafe/macros, both forbidden.)
- **Naming rationale, kept for future passes** — `Query` VETOED for the identity
  type (hard collision with the Queries effect-class; overloaded tokens mislead
  humans and agents alike); the ear-perking principle adopted: where a concept
  does something unusual, an idiomatic name is camouflage. `Question`/`Answer`
  make the acked invariant utterable in the type names ("an answer is readable
  only by re-deriving its question"); `Belief` states honesty
  (oracles-may-lie) and the stickiness law in English ("a conflicted belief
  never resolves").
- **Honest residue**: `derive()`'s `Option` deliberately erases the ⊤-cause
  (license paths never branch on which-⊤); a `derive_explain` sibling feeds the
  why-lane. Ids are within-run only — NEVER serialized (`kSTATE` parked;
  rec-5). Two sites asking the byte-identical question stay two keys (site is
  in `QuestionNode`) per `inv-site-keyed-results` — collapsing them is the
  standing kSTATE-coupled human decision, not a refactor. Named
  priority-tension: this trades some simplicity (arena-threading; an id layer)
  for validation on cardinal-sin-tier invariants — validation wins, and the
  analysis plane is perf-free territory.

## §9 — DST and harness plan

- **The battery (R2):** per-iteration record service from hostsim (a state
  oracle — it can answer any derived Question); seeded arrival-order shuffling
  ACROSS iterations, not just within one batch; `pin-process-confluence-
  conflict-free` (byte-identical plans under shuffle, conflict-free seeds);
  `pin-conflict-soundness` (injected same-cell conflicts ⇒ the belief lands
  Conflicted ⇒ its consumers guard/run; per-seed reruns stay bit-identical;
  byte-identity across seeds asserted only under the stability flag);
  `pin-empty-world-degenerate-loop` (no oracles ⇒ exactly one iteration,
  byte-identical to today); chain-depth termination (a depth-k capture chain
  quiesces in ≤ k+1 iterations); the §5 injection must-covers; the §4
  leafid-namespace pin; both `277` §5 pins re-proven non-trivially (a synthetic
  probe-re-entrant seed exists and the meet still collides on unknown members);
  under the stability flag: unconditional byte-identity + the containment
  differential (filtered-plan elisions ⊆ unfiltered-plan elisions).
- **Harness shape:** the in-memory tier carries the reactive logic (`24I`
  de-graduation doctrine; the e2e corpus does NOT balloon). e2e gains ONE
  multi-round exemplar family: a book with a capture-dependent guard,
  hostsim-served across two iterations, gate-1 parity per-artifact at width 1.
  The authored single-`probe-results.txt` case shape stays valid for every
  single-iteration case (the whole corpus, until an oracle vouches a capture).
- **Cancellation pins (R4):** cancelling inside the finality class never changes
  the elide-set (only reason-text / guard-vs-run within the demotion order); a
  seed that cancels outside the class is a rig ERROR (the gate is a
  precondition, not a policy preference).

## §9b — The prospective-invariant dictionary (law-form; the 26x-resident registry)

Fixed names, short statements of what must be held; the unrolled *why* lives in
the cited sections. These are NOT in `spike/CLAUDE.md` by design — the steering
files are current-truth-only, and this machinery is unbuilt; each entry migrates
there at the revival stage that makes it real (R2–R4), and until then THIS table
is the registry.

- **`26C:inv-record-admissibility-by-want`** (§1; R2) — an Answer is consumable
  only by re-deriving its full Question — (site · context · resolved compiled
  form) — from the current knowledge state. A retracted premise makes the Answer
  *unreachable*, never re-interpreted. Build as keying, not filtering.
- **`26C:need-captured-bytes-ship-as-data`** (§5; R2) — a captured value
  re-enters a shipped artifact only as a single-quoted argv token of an
  oracle-authored function, through its argparse; never command/code position,
  redirect targets, or heredoc bodies. Single-line enforced at capture. Holds at
  EVERY iteration's ship, not only at mint.
- **`26C:law-host-boundary-severs-provenance`** (§5 cl.6; R2; license lanes
  ONLY) — provenance/backing never inherit through a host-side transformation.
  A chained capture folds only if every input capture folds; its patrol-surface
  is the union; any un-derivable link is ⊤ and walls.
- **`26C:fence-capture-never-feeds-closure-pass`** (§2; binds from
  lane-fallback-carry onward) — read-set closure is proven over marks and sh
  structure only; a capture-resolved value never qualifies a body.
- **outcomes-schedule-never-evidence** (§2; R2) — dispositions and
  compare-verdicts may drive question-minting and cancellation; they never enter
  beliefs, backings, or license mints. (`277` §5's two pins re-proven
  non-trivially the day the back-edge exists.)
- **`26C:rul-candidate-demotion-only-cancellation`** (§3; R4) — any
  timing/cancellation policy may only DEMOTE a site along elide ≻ guard ≻ run.
- **the attention-first cancellation gate** (§3; R4) — economy-cancellation may
  never cost a possible elision; it fires only on retraction-proof finality
  (`26C:mech-wall-standing-finality-class`); a cancelled vouched site rests on
  guard-by-vouch (`26C:mech-cancel-lands-on-guard-by-vouch`), never bare run.
- **`26C:need-per-artifact-identity`** + **`26C:need-per-artifact-leafid-namespace`**
  (§4; R1) — every shipped artifact: fresh nonce, additive `iter=`, its own
  leafid space resolved through its engine-side manifest. `attempt=` stays
  retry-only (discard-wholesale); iteration results ACCUMULATE. Hosts answer
  coordinates; they never name Questions.
- **`26C:rul-diagnostics-final-state-only`** (§7 item 11; R2) — user-facing
  diagnostics emit from the quiesced final state only; no per-iteration
  emission, no stale wall-warnings for lifted walls.
- *Theorems (descriptive, DST-pinned, not obligations):*
  `26C:thm-fold-confluence-unconditional` (fixed evidence ⇒ order-free plan) ·
  `26C:thm-process-confluence-conflict-free` (conflict-free runs ⇒
  byte-identical plans under shuffle; conflicted runs stay sound, order-variant;
  the stability flag restores unconditional byte-identity).

## §10 — What remains open (everything else is acked or future-sited by design)

- **The binding-site gate** (§6) — deliberately un-ruled; decided at R2 entry.
- **The stability resolution** (§1; né `26C:ask-confluence-carve-choice`, the
  slug 26B's correction note cites) — direction-tier; formally confirmed at R2
  entry alongside the gate.
- **`26C:note-knob-candidate-plan-stability`** — the §1 resolution is
  KNOBS-shaped (≈ `kPLANSTABLE-maximal-elision ↔ kPLANSTABLE-reproducible-plan`,
  status mode, owner user, lock-in low); registry entry at the human's leisure.
- **Trial riders** — `26B:ask-trial-counts-capture-walls` + the §5b
  narration-depth sizing; both ride `270` §5's owed-on-revival list.
- **`26C:feeder-oracle-why-metadata` spelling** (§5b) — the kOOB question,
  revival-owned.

## §11 — Status table

| component | authority |
|---|---|
| semantic model; both confluence theorems; the two-plane split | conductor synthesis, +SURE (theory); spike-fit ~SUSPECT until R2 |
| record-admissibility / "facts are inherently argv-keyed" | HUMAN-ACKED; governing form human-typed; key-don't-filter confirmed |
| stability resolution (default maximal; flagged GC; DST framing) | HUMAN-TYPED lean + full-plan ack; formal confirmation at R2 entry |
| captured-bytes-as-data + host-boundary-severs-provenance | HUMAN-ACKED HARD; corollary human-typed; license-lane-scoped |
| cancellation package (attention-first gate · demotion order · guard-by-vouch) | HUMAN-ACKED (full-plan ack); mechanism at R4 |
| why-explanation lane (§5b) | HUMAN-TYPED direction, exploratory tier; sizing at revival |
| binding-site menu incl. (b′) | sharpened; deliberately UN-ruled (human's, at R2 entry) |
| quiet-welding ledger + APPLIED annotations | audit complete vs tip `e16b0c8`; annotations one-commit-revertible |
| R0–R4 ladder; §8b typing discipline | HUMAN-ACKED as plan; shapes STRAWMAN, discipline firm |
