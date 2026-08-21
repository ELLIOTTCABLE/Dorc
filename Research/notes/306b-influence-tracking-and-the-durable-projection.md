# 306b — Influence tracking, the durable projection, and refusal as an outcome

> Tier: **LLM-authored, lower-reasoning agent** (Opus-class, working from a long design
> sitting with the human). This document is the AUTHORITATIVE, conductor-facing statement of
> the vocabulary, laws, and architecture that sitting produced. It is a sanitized mirror.
>
> **Rationale availability.** Several rules below are stated without their full derivation,
> because the derivation does not survive restatement in ordinary engineering terms. Those
> are marked **[rationale withheld]**. The complete reasoning lives at
> `Research/quarantine-DO-NOT-READ/306a` and is readable only by a reader authorized into
> that quarantine. If you are not so authorized: **do not re-derive a withheld rationale, and
> do not weaken a rule because its justification looks thin from here.** Ask the
> human, or route through the /opaque-review SKILL if the human is unavailable.
>
> Confidence: the human's rulings below are firm. My own architectural suggestions are SOFT —
> offered to be tuned by a reviewer with better judgment and less context than I had. I
> reversed myself repeatedly during the sitting; treat unmarked reasoning accordingly.

## §0 — What this is, and what it is not

Dorc's analyzer consumes bytes produced by the machines it manages. Those bytes are
*unverified*: the engine has no independent means of confirming that they describe the state
they purport to describe. Everything the engine subsequently computes — verdicts,
subsequent probe sets, dispositions, narration — is downstream of them.

This document names the analysis that tracks that relationship, states what the engine owes
because of it, and describes the shape of the durable artifact under those constraints.

It settles very little. The sitting behind it produced far more tension than resolution, and
the honest headline is that **the resting points here are matters of tuning and taste rather
than mechanism.** What follows is vocabulary, a small number of firm laws, and a larger
number of suggestions explicitly marked as such.

## §1 — Influence: one taint-flow analysis among several

**The word is load-bearing and deliberately narrow.** This engine already performs several
taint-flow analyses over the *analyzed program* — value-grade provenance, read-set closure,
the who-am-I ingredient labelling, the emission-set non-interference derivation. "Taint"
alone is ambiguous across all of those. **`influence` names exactly one of them**, and no
other, and should never be used as a synonym for taint in general.

**`influence`** — a dataflow property over values, tracking derivation from host-reported
bytes.

Notably, this taint-flow doesn't just thread the *analyzed unit*, it threads upwards from the
analysis into the *analyzer* - when influenced, the analyzer itself must track,
at the type-level, its own influence. This is critical for aid-reporting: the
analyzer must always know what has influenced its decisions; influence is a
one-way gate, across all program state, including the analyzer.

Three grades at v0, with room reserved for refinement (§1c):

- **`authored-before-contact`** — computed only from controller-supplied invocation material
  and operator-authored source text, all of which exists before the first host exchange. The
  round-zero analysis lives entirely here.
- **`host-reported`** — the host-produced bytes themselves, as received.
- **`host-influenced`** — anything whose value, or whose control-flow path, depended on
  either of the above.

### §1a — `rul-influence-is-monotone-and-absorbing` [FIRM]

The grade moves in one direction only. Any computation touching a `host-influenced` operand
yields a `host-influenced` result; there is no operation that lowers a grade. This is the
same one-way shape the codebase already uses twice (the `Must → May` coercion, and the
`ByObservation`/`ByVouch`/`BySilence` claim tiers), and the same compile-direction discipline
should apply: the lowering conversion must not exist.

**At v0 the flip is global and positional**: it occurs at first host-byte ingestion, and
every code path invoked after that point is within its scope. That is deliberately coarse.
It is the conservative reading and it is cheap; refinement is §1c.

### §1b — `rul-influence-reaches-the-analyzer` [FIRM]

This is the part most likely to be missed, and it is the reason the analysis is worth
building at all.

Influence is **not** merely a label on the engine's model of the managed world. It is a
property of the engine's *own execution*. If a host-reported value determines which branch
our own code takes, then everything computed inside that branch is influenced — including
values that never touch a coordinate, a fact, or a verdict. The property flows *upward* out
of the analyzed picture and *into the analyzer*, and it propagates through our own types,
our own control flow, and our own derived data structures.

Motivating example: *discarding the entire analysis* is an influenced result;
and is a perfect counter-exmpale to 'influence is within-dataflow.'

Practical consequence, and the one to design against: a conclusion can be influenced without
mentioning anything host-shaped. Scheduling decisions, iteration counts, which passes ran,
which diagnostics were reachable, what a fixpoint converged to — all of these can be
influenced by *when and whether* bytes arrived, without any host-reported value appearing in
the result.

### §1c — Gradation is owed [SOFT — shape unsettled]

Three grades is a floor, not a model. The human has stated that this wants **degrees** —
influence has intensity, and treating a value whose *shape* was determined by host-reported
bytes identically to one that merely *mentions* such a value will over-collapse and cost real
value.

I do not know the right axis and am not proposing one. Candidates worth a reviewer's
attention, none endorsed:

- by **channel** — did the influence enter through a value, a control-flow branch, or a
  timing/arrival property?
- by **distance** — how many derivation steps from ingestion, with a saturating cap.
- by **contingency** — would perturbing the host-reported input actually change *this*
  result? (Note this one is *computable* rather than declarable, given a pure kernel: it is
  a differential, and the same machinery the sparing re-derivation already runs.)

The contingency axis is the one I would investigate first, because it distinguishes
"influenced" from "influenced in a way that matters," and that distinction is what keeps the
grade from saturating to useless (§6b).

### §1d — Declared versus computed [SOFT]

Because the kernel is pure, a value's influence grade is in principle **derivable** rather
than merely assertable: a result is influenced exactly when it depends on host-reported
input. If that holds, the engineering shape is a declared grade as the cheap always-on
approximation, with computation available as the ground truth to audit it against, and
disagreement between them treated as a defect signal rather than resolved silently.

## §2 — The conceptual structure and its durable projection

### §2a — `rul-durable-is-a-projection` [FIRM, human-stated]

The engine's single output is a complete in-memory structure — every decision, its inputs,
its grade, its narration. Every apparent product is a *function over* that structure: the
executable artifact, the plan render, the connections the orchestrator opens, and **the
`.whylog` file itself**.

The durable is therefore not "the log." It is one lossy projection among several, and
"what do we persist" is the question *what does that projection keep*.

**The durable is not permitted to be poor; it may be forced to be poor** [human]. Loss is a
high cost and must not be bought cheaply. Where the projection does drop something, the
drop must be recorded as such — a projection that silently omits is indistinguishable from
one that had nothing to omit.

### §2b — What the projection may not keep [FIRM, human-stated]

Freeform host output is **not** persisted. The engine is referentially agnostic: it cannot
determine what any given byte-sequence means or whether it is sensitive, so no filtering rule
of the form "detect the sensitive thing and omit it" is available to it, and none should be
attempted. The resolution is structural rather than selective — host text that
*isn't* filtered through a contracted oracle stays on the host, and is pulled at
debugging time if it is wanted then, accepting that it may be gone.

However, that's a cold comfort - the structured record stream the engine must
accept is bounded by **contract only**. Nothing in construction, testing, or
proof prevents an oracle body from emitting arbitrary content into it, and the
engine must not claim otherwise anywhere in its documentation or output. Our
host-text-on-the-host rule is defense-in-depth, but not exhaustive; it *does
not* prevent host-text from reaching the controller. It exists only as a
backstop against imperfect oracle-authorship against admin debugging-needs,
where the oracle-author made the wrong selection about what to report or return
or filter-out, effectively.

### §2c — Content, as it stands [MIXED]

Persisted today and correct to keep: the invocation record, the record stream as received,
the report lane, the decision digest, the apply report, the run nonce.

Suggested additions [SOFT]:

- **Per-site decision records** rather than a single scalar digest. One digest answers only
  "does this reproduce," which is a wall; per-site records make the answer a *diff*. The
  `SiteId` keying this needs is already owed for unrelated reasons.
- **Round structure**, once probing is ever more than one-shot (§3).
- **Timings**, when they arrive — with the note that per-leaf durations are `host-influenced`
  by construction (the host determines them) and additionally disclose state that no record
  stated.

**Deferred, not settled**: storing operator-authored source text *by value* rather than by
path-and-digest. I argued for it (self-containment; replay survives ordinary editing;
shareability), the human's counter-arguments stand, and the disposition is a deferral rather
than a decision. Do not treat either direction as ruled.

## §3 — Re-ingestion

### §3a — `rul-influence-rehydrates-on-reingestion` [FIRM]

A round trip through a durable must not lower an influence grade. Material re-read from a
durable carries the grade it had when written, and everything derived from it on re-ingestion
carries that grade forward under §1a.

### §3b — `rul-missing-influence-grade-reads-highest` [FIRM]

A grade that is absent, unverifiable, or fails its recomputation check rehydrates as the
**highest** grade, never the lowest. This is the same posture as the engine's existing
silence-licenses-nothing discipline, applied to this analysis: an unreadable grade is not an
absent constraint.

The property this buys is worth stating for whoever implements it: under this rule, removing
grade metadata from a durable cannot lower anything, so metadata loss degrades conservatively
rather than permissively.

### §3c — `rul-reingestion-drives-no-action` [FIRM, human-stated]

Nothing re-read from a durable may produce an action. This is **not** redundant with §3a,
and should not be removed on the argument that §3a subsumes it: not all persisted material is
influenced — `authored-before-contact` material stays uninfluenced forever — and the hazard
it guards is *temporal scope*, not derivation. A durable's uninfluenced content still
describes a world-moment that has passed. Two rules, two distinct failures.

(Both are modulo `KNOBS:kSTATE`, which remains the human's knob; this document is
kSTATE-agnostic. Even if the kSTATE door were ever intentionally opened, it
would be a *different* durable with *different* architecture and constraints;
not the whylog, and must be designed with its own care and goals.)

### §3d — Replay shape [SOFT]

Because the kernel is pure, re-ingestion can *re-derive* rather than believe. The suggested
shape:

- Re-derive from the persisted inputs; use persisted conclusions to **align and compare**,
  never to substitute.
- **`rul-recorded-and-rederived-stay-distinguishable`** [FIRM] — a rendered link is in
  exactly one of four states, and the state is always visible: re-derived now · recorded then
  · both, agreeing · both, disagreeing. A persisted conclusion must never render in a
  re-derivation's clothing. (This is the existing trust-tier-is-syntax discipline extended
  along a replay axis.)
- Disagreement is a **finding**, not an error to swallow: it detects durable corruption,
  version skew, and engine nondeterminism with one mechanism.
- A cheap cryptographic hash over the durable is worth carrying as an independent integrity
  signal [human]. It competes with nothing above.

> human: i need to look into equivocation literature & decentralized
>        witness-cosigning; can untrusted hosts keep the controller honest about
>        the whylog?

### §3e — Additional acts past re-ingestion [SOFT, human-framed]

For material the projection could not keep, an opt-in door exists: freeform host-side
residue, and structured re-querying of the host. Two constraints on both:

- Both answer about **now**, never about **then**. Host-side residue has been on host-managed
  storage since the run; re-querying interrogates a world that has moved. Rendering either as
  contemporaneous with the decisions it explains is a provenance error of the kind the
  engine's attribution discipline exists to prevent.
- Re-querying is a probing act outside the plan/apply cycle and is governed by §5c.

## §4 — Refusal

### §4a — `rul-refusal-is-not-a-ladder-position` [FIRM] [rationale withheld]

The per-line outcomes {elide, guard, run} form an ordering the engine uses under
*uncertainty*: less certainty demotes toward run. **Refusal is not the bottom of that
ladder.** It is an orthogonal outcome, reached for a different reason, and demoting a site
along the ladder is not a substitute for it.

The engine already holds the mechanism (`rul-integrity-failure-withholds-mutation`, and
`rul-admission-is-a-closed-outcome`'s `Refused`, which returns before plan construction). What
is underbuilt is its *reach*: refusal is currently a narrow admission-edge outcome, and the
sitting concluded the product underweights it generally, because for a gradually-enhanced
tool refusal is almost never the right answer — right up until it is.

**The rationale for preferring refusal over ladder-demotion in the relevant cases is
withheld.** Do not reconstruct it from the mechanism, and do not conclude from its absence
that demotion is equivalent.

### §4b — `rul-report-only-output-cannot-plan` [FIRM, human-stated]

When ingestion integrity is lost, the engine does **not** fail fast and does **not** stop
analyzing. It continues: probing proceeds, folding proceeds, analysis proceeds to completion,
and the product is a full analytic report with root-cause material, graded throughout by §1
so that its claims can be correctly qualified.

What changes is the **output type**. The analysis for that target produces a value that is
structurally incapable of yielding a plan step. Implementation notes [SOFT]:

- This wants to be a **type**, not a flag. A boolean eventually goes unchecked.
- The containment belongs at the **analysis output**, not at plan emission. Facts are
  cross-cutting — survival, wall-walk, and the decision record all read them — so a fold that
  produces ordinary facts leaks sideways even when no plan is emitted for that target.
- This **reverses** any general prohibition on partial consumption of a malformed record
  stream. The prohibition was only ever justified for consumption that can reach a plan step;
  consumption into a report-only output carries none of that weight.

### §4c — Whole-target or narrower? [OPEN]

Whether refusal is always whole-target, or admits a narrower scope, is **unresolved**. It was
raised and not settled. A reviewer should treat both as live.

## §5 — Probing modes

### §5a — `rul-authority-free-probing-mode` [FIRM, human-stated]

Probing that occurs outside the ordinary plan/apply cycle — debugging surfaces, re-querying
for explanation (§3e), operating deliberately in a degraded state — runs in a mode that
deploys **no licensure, no credentials, and no context escalation.**

This is not a special-case rule. It is the general shape for any situation where the engine
wants information without spending authority, and the human's framing is that the affine-typed
seam for it is worth building early precisely because it generalizes.

### §5b — Continued probing under lost integrity [FIRM]

§4b's continue-and-analyze includes continued probing. One bound: continued probing must not
**escalate context**. Gathering read-only material is one act; entering a shifted context
(the wrapper/context-entry machinery) on a target whose integrity has already failed is
another, and the escalation dial refuses it. [rationale withheld]

### §5c — Consent [SOFT]

Re-querying at explanation time is not covered by the plan/apply consent moment — the operator
asked a question, they did not authorize a run — and it happens when no plan is on screen to
disclose anything. It needs its own gate. Shape unsettled. (This is slightly
covered in the re-probing-auth/consent discussion on dorc explain/why surfaces.)

## §6 — Diagnostics and rendering

### §6a — `rul-diagnostic-names-the-observation` [FIRM]

A diagnostic over host-reported material names **what was observed**, never an inferred cause.
"Seven lines carried no terminal token" is the observation; "your pipe is tearing" is an
inference, and stating one inference forecloses the others in the reader's mind.

This is not a stylistic preference. In a controlled study of compiler error messages, a message
that named a subclass rather than its parent sent 49 of 50 participants to the wrong fix and
one to the right one [A-barik-compiler-error-messages-2017]. A *truthful* message produced
the wrong repair, at that rate, because of which level it named. Where several causes are
consistent with an observation, name the set or name none.

### §6b — `rul-influenced-values-never-gate-engine-control-flow` [FIRM]

Counts, ratios, and other scalars derived from host-reported material may be **reported** and
must not **decide**: they may not rank attention, sort output, cross a threshold, or select a
code path in the engine. (Note the interaction with §1b — if such a scalar gates a branch,
everything in that branch becomes influenced.)

### §6c — Marking, and its saturation problem [SOFT, and this is the sharpest open tension]

Values carrying a non-lowest influence grade should render wearing it, so a reader can tell
measurement from derivation. Two findings pull hard against a naive implementation of that:

- A marker present on nearly everything conveys nothing and becomes chrome. This is a general
  information-design result, and it is why "grade every link" fails.
- Per §1a, once influence flips it covers nearly the whole post-ingestion picture. **So the
  influence grade is the right discriminator for what the engine may *do*, and the wrong one
  for what the renderer should *mark*.**

The suggested resolution [SOFT] is that these are two different discriminators for two
different jobs: the grade governs authority; something sparser governs display. The
contingency axis (§1c) is the natural candidate, because it is rare where the grade is
universal.

The human's own direction for the render layer, recorded and deferred: the taint frontier is
**computable**, so a partially-graded render is dualistic — a trusted region, a boundary
marker, and a derived region rooted at a named node — which then informs sorting and graph
disposition. That is render-layer work, not core-truth work.

### §6d — Sibling diagnostic codes over one code with a hidden enum [FIRM as applied]

Where distinct observable conditions imply **distinct repairs**, they earn distinct codes.
The catalog law's own test is different-world-states-with-different-repairs, and conditions
like "the stream's framing never began," "two writes interleaved," "material arrived after the
close marker," and "the header's identity fields do not match this invocation" pass it — the
last especially, since it means the engine may not be addressing the target it believes it is.

Collapsing them into one code with an internal reason enum destroys the reader's ability to
discriminate without buying anything: the conditions still occur, and still occur at the same
rates — the reader simply loses the material with which to tell them apart, and with it the
ability to reach the right repair. [rationale withheld for the fuller argument.]

### §6e — The nine record-lane codes: RULED, keep them [FIRM — closes a standing question]

The engine's catalog carries nine `records-*` codes describing distinct conditions of the
structured record stream. Eight of them have no defining case and no production emitter, and
have stood as the catalog census's only long-term exception. **The question of what to do with
them is closed here: keep all nine.**

Reasoning, in order of weight:

1. **They pass the catalog law's own test.** Applied honestly, each names a different
   world-state with a different repair: material truncating mid-write (channel, buffering,
   host memory pressure) · two writes interleaved (concurrency on the channel) · material that
   is not ours, or is from a previous attempt (concurrent runs, retry hygiene) · material after
   the close marker (a wrong `wait`, an inherited descriptor) · framing that never began
   (transport or artifact shipping) · identity fields that do not match this invocation
   (**the engine may not be addressing the target it believes it is** — categorically unlike
   the others). These are siblings by the law's definition, not by grammar-fit.
2. **Deletion buys nothing.** The conditions are not made rarer by having one name; only the
   reader is made poorer. §6d.
3. **The developer-experience evidence points the same way.** An effective false positive is a
   finding the reader does not understand and does not act on (§7); nine conditions each naming
   a distinct physical event with a distinct repair are more actionable, not less, than one
   code with the discrimination hidden inside it.

**Where they are computed, however, changes.** The forgiving parser that originally produced
them returns partially-parsed material, and that behaviour is a property of *where its output
went* — a path that could reach a plan step — not of its vocabulary. So:

- **The strict admission path computes the discrimination.** It already classifies; this is a
  refinement of an existing classification, and it inherits refuse-the-whole-attempt unchanged.
- **The forgiving parser is re-homed, not deleted.** A report-only consumer (§4b) is a
  legitimate destination for forgiving parsing. Its output type must be one the plan-producing
  path cannot consume, and — because no type can privilege one crate over another here — the
  remaining half is a lexical gate asserting a non-empty walk, in the manner of the gates that
  already exist for comparable seats. Adding an entry to such a gate's allow-list is a governed
  act, not a local edit.
- **The nine carry §6a's and §6b's disciplines**: each names its observation and not an inferred
  cause, and none of their counts may gate engine control flow.
- **They remain diagnostics attached to an outcome, never outcomes themselves.** The closed
  admission outcome set does not grow, and no per-code behavioural branch may appear.

**Routing — dispositioned, softly.** Which of the nine refuse (§4) and which proceed under
ordinary conservative planning is settled well enough to build against, and the per-condition
table lives with the work at `plans/306c` §3b rather than here. The line they were sorted along
is worth carrying in law, because it extrapolates: **refuse where the loss is unbounded or the
frame's identity is in question; proceed conservatively where the loss is bounded, detected, and
self-accounting.** Only one condition's behaviour changes from what the code does today.

Graded [human, SOFT — spike-tier]: a disposition to ship against, not welded law, and a builder
who finds a case wrong on contact should report it rather than contort.

## §7 — What the DX evidence supports

A graded source base was gathered during the sitting. The findings below are the ones that
survive restatement as ordinary developer-experience guidance; the remainder, and the full
graded manifest, are pointed at in §9.

- **Expertise raises trust in an analyzer rather than lowering it.** Practitioners with more
  program-analysis background report *higher* use and *higher* belief that the tool catches
  real faults [A-christakis-bird-program-analysis-2016]. The assumption that an expert
  audience will independently catch the engine's mistakes is unsupported.
- **The tolerance envelope is tight and quantified.** Only 24% of developers tolerate a 20%
  false-positive rate; a large-scale industrial deployment holds its compile-time bar under
  10% *effective* false positives and auto-disables an analyzer whose not-useful ratio exceeds
  10% [A-sadowski-static-analysis-google-2018]. The reframing that matters: an **effective
  false positive** is a true finding the reader does not understand and does not act on — the
  reader sets the rate, not the author.
- **Per-user output customization damaged adoption** in that same deployment, because no
  finding could be relied upon to have been seen; configuration moved to project scope
  [A-sadowski-static-analysis-google-2018]. Relevant to any per-operator verbosity dial.
- **Diagnostics are read harder than code** — mean fixation durations were higher on error
  text than on source (419ms vs 394ms), attributed to prose↔code modality switching
  [A-barik-compiler-error-messages-2017].
- **Detail behind a click is, empirically, detail withheld.** Follow-through rates on
  "more information" affordances are very low even in operator populations. Nothing
  load-bearing should live only on a pull surface.
- **Repeated identical diagnostics lose adherence over days**, at rates far below firehose
  volume; varying presentation measurably slows the decay, and readers knowing about the
  variation does not defeat it.
- **Verbose beat terse** in at least one operator-facing study, and additionally *reduced*
  follow-up questions — which cuts against the intuition that brevity respects expert time.
- **Followability is an engine constraint, not a renderer constraint.** The classic
  automation result is that a machine should decide "using methods and criteria, and at a
  rate, which the operator can follow, even when this may not be the most efficient method
  technically" [A-bainbridge-ironies-of-automation-1983]. This sits in genuine tension with
  the project's spend-analysis-freely posture and bites hardest where derivation chains are
  deepest.

These do not reconcile into a single setting. The sitting's conclusion was that the resting
point is a matter of ongoing tuning, and that anything presenting itself as a resolution here
is probably an artifact of wanting one.

## §8 — Open

- The gradation axis for influence (§1c). Unsettled, and the most consequential open item.
- Whether refusal is whole-target (§4c).
- The consent gate for explanation-time re-querying (§5c).
- The marking discriminator (§6c) — the display half specifically.
- Source text by-value versus by-reference in the durable (§2c). Deferred, both directions live.
- Multi-round replay: the corpus is **silent** on how re-derivation works once probing is more
  than one-shot. Rounds must be recorded with their correspondence (which returned material
  answers which dispatch), and arrival ordering, deadline firings, and partial-arrival
  decisions are inputs that do not re-derive. There is prior ruling in this area
  (`26C`) that a reviewer must reconcile against before designing.

## §9 — Pointers

- Full derivation, and every withheld rationale: `Research/quarantine-DO-NOT-READ/306a`
  (authorized readers only).
- Graded source base:
  `.claude/research/quarantine-DO-NOT-READ-patronizing-aid/` — `sources.json` and the turn
  notes. Sources cited above by slug are drawn from it.
- Suggested build sequencing and its open scheduling questions: `Research/plans/306c`.
- Existing law this builds on, unchanged: `rul-host-bytes-bounded-before-admission` ·
  `rul-admission-is-a-closed-outcome` · `rul-integrity-failure-withholds-mutation` ·
  `rul-attribution-is-controller-minted` · `two-plane-aid-law` · `rul-whylog-is-the-spine`
  (whose boundary-3 wording was narrowed during this sitting).

## §10 — Influence carriage across semantic entities [HUMAN-RULED 2026-08-21]

This closes `30M:ask-spine-grade-boundary` in design, not implementation. It does not
settle §1c's eventual gradation, require complete project-wide threading now, or make
influence an authority policy. It rules the carriage shape that later work refines.

- **`rul-influence-carried-by-entities`** — a stable semantic object whose meaning must
  inherit influence carries a private, immutable, non-optional influence account itself.
  This includes analyzer conclusions, decisions, licenses, Spine events, projection
  selections, arrangements, routing choices, and rendered outputs. A parallel side table
  that callers must remember to populate is not an acceptable enforcement mechanism.
- **`rul-semantic-mints-join-influence`** — the constructor that turns inputs into a
  stable semantic object also computes that object's maximal inherited influence from
  every contributing data and control input. Constructors accept influencing objects or
  restricted dependency accounts, never a caller-selected grade. No public field,
  default, `None = authored`, generic downgrade, or deserialization route may manufacture
  lesser influence. An interned/graph representation may back the account later, but
  never replaces type-carriage.
- **`rul-transient-wrappers-are-plumbing-only`** — a generic influenced-value wrapper may
  carry influence through temporary generic calculations. A stable domain object whose
  invariant is "always influence-bearing" stores the account directly; the wrapper is
  not its persistent model.
- **`rul-consequential-sinks-require-influence`** — authority mints, finalized analysis
  decisions, Spine event construction, projection/filter decisions, sink selection, and
  output construction accept only semantic objects whose type guarantees influence
  carriage. A sealed common contract plus exhaustive species/consumer censuses should
  make a new consequential type fail until it joins the discipline.
- **`rul-spine-preserves-never-stamps`** — Spine is a relatively frozen output product
  assembled from stabilized analyzer results. It neither originates nor computes
  influence, never applies an object-global grade, and never fills an absent record field.
  Each record arrives with the maximal influence established by its own semantic mint;
  Spine stores it unchanged. Spine and its views remain structurally unable to feed any
  conclusion back into analysis.
- **`rul-projections-continue-influence-flow`** — Spine finalization does not terminate
  propagation. Filters/views make later selection, ordering, arrangement, routing, sink,
  and output decisions; each stable result is another influence-bearing semantic object
  joining its Spine inputs and other influencing inputs. Such decisions never mutate
  Spine. Where their account is owed for explanation, it lives in an explicit projection
  result/trace rather than a hidden renderer branch or retroactive Spine append.
- **`rul-untracked-is-not-authored`** — full threading is deliberately not owed yet.
  Every unconverted seam is explicit `untracked`/unknown and reads maximally influenced at
  consequential consumers; absence never means authored-before-contact. This is how the
  implementation may remain staged without laundering its missing region.

Influence remains causal accounting, orthogonal to authority: it answers what affected a
result, while claim tiers, vouches, admin policy, and authority-bearing projection types
answer what that result may license. Implementation should remove the landed one-grade-per-
Spine stamp, convert obvious semantic boundaries first, and leave explicit conservative
adapters elsewhere. The deadline is before the `309` boundary close, influence-aware render
work, or any durable-grade lift relies on the current record fields.
