# 306c — Influence, refusal, and authority-free probing: suggested sequencing

> Tier: **LLM-authored, lower-reasoning agent** (Opus-class). This is a PROPOSAL to be tuned,
> not a spec to be executed. Its vocabulary and laws come from `notes/306b`, which is
> authoritative; where this document and that one disagree, that one wins. Some rules it
> inherits carry **[rationale withheld]** — the derivation lives at
> `Research/quarantine-DO-NOT-READ/306a` and is readable only by a reader authorized into that
> quarantine. Do not weaken a withheld-rationale rule to make sequencing easier; re-scope the
> work instead, or route through opaque review.
>
> Sizing here is guesswork. A reviewer with better judgment should re-price everything.

## §1 — Three items, and why they are ordered this way

### `item-influence-grade-and-its-seams` — the one that wants to move first

Build `notes/306b` §1: the grade type, its monotone/absorbing coercion discipline (the
lowering conversion must not exist), and the seams where it is carried.

**Why first, and it is a scheduling argument rather than a value one:** the kernel is under
active edit. A property that must flow through the analyzer's own types and control flow
(`306b:rul-influence-reaches-the-analyzer`) is dramatically cheaper to thread while those
seams are already open than to retrofit afterward. Retrofitting an absorbing dataflow
property across a settled kernel is the expensive shape.

Suggested v0 scope, deliberately small:

- The three-grade type with its one-way coercion, in `core`, following the existing
  `Must`/`May` and claim-tier precedents rather than inventing a new discipline.
- The positional flip at first host-byte ingestion — coarse, conservative, cheap.
- Carriage at the seams that already exist for other provenance (the intake edge, the fold,
  the decision record).

Explicitly **not** v0: the gradation axis (`306b` §1c is genuinely unsettled and building a
wrong axis is worse than building none), and computed-versus-declared reconciliation
(`306b` §1d).

This item is subtle engineering-tradeoff work more than design work, and is probably better
handled by a conductor reasoning over the real code than specified further from here.

### `item-authority-free-probing-mode` — small, general, and worth doing early

Build `306b:rul-authority-free-probing-mode`: a typed probing mode that structurally cannot
deploy licensure, credentials, or context escalation.

The human's framing is that this is **not** a narrow special case — it is the general shape
for any situation where the engine wants information without spending authority. Debugging
surfaces, explanation-time re-querying, and deliberate operation in a degraded state all want
it, and the affine-typed seam is cheap while the probing lanes are still malleable.

Sizing guess: small, if it lands as a type on the probe-dispatch seam rather than as a policy
check at each call site. A flag will not hold.

### `item-refusal-reach` — real, but the design question is open first

`306b` §4 states that refusal is an outcome orthogonal to the {elide, guard, run} ladder, and
that the product currently underweights it: the mechanism exists at the admission edge and
does not reach much further.

**This one should not be built until `306b` §4c is answered** — whether refusal is always
whole-target, or admits a narrower scope. That is an open design question, not an
implementation detail, and building the narrow version first would foreclose it.

The companion piece, `306b:rul-report-only-output-cannot-plan`, is buildable in principle
sooner: an analysis output type that structurally cannot yield a plan step, with the
containment at the analysis output rather than at plan emission. Note it **reverses** any
general prohibition on partial consumption of a malformed record stream — the prohibition
only ever applied to consumption that can reach a plan step.

## §2 — Sequencing, as a suggestion

1. `item-influence-grade-and-its-seams`, v0 scope only, while the kernel is open.
2. `item-authority-free-probing-mode`, independently and in parallel — it touches different
   seams.
3. `306b` §4c gets answered (human, or a design sitting).
4. `item-refusal-reach`, scoped by that answer.

Nothing here blocks work currently in flight. Item 1's *timing* argument is the only one with
a real clock on it, and the clock is "while the kernel seams are open," not a date.

## §3 — Deliberately not proposed

To keep this from becoming a program nobody asked for:

- No durable rebuild. `306b` §2's projection architecture is a design record; the artifact is
  a spike and the record is the deliverable.
- No gradation model (`306b` §1c). Unsettled; a wrong axis costs more than no axis.
- No render work (`306b` §6c). The display discriminator is open, and the dualistic-render
  direction is deferred by the human.
- No multi-round machinery. The corpus is silent there and there is prior ruling to reconcile
  first (`306b` §8).

## §4 — Two standing tripwires

These are the mechanism by which this work reaches whoever needs it, since none of it is
scheduled. Both are stated in the crate steering law; repeated here for the reader who
arrives at this document first.

- **Any enrichment of what the durable persists, or of what re-ingestion consumes**, stops
  for opaque review before design — not after build.
- **Any probing that is not one-shot** — concurrent, sequential, out-of-order, posthoc, or
  multi-target — stops for opaque review before design.

Both are "not building now"; the invariant is that the review has triggered and cleared before
the surface is touched. Neither is a request to build anything today.

## §5 — Open questions a reviewer should expect to answer

- Is refusal whole-target, or narrower? (`306b` §4c) — gates `item-refusal-reach`.
- Which admission conditions route to refusal, and which to ordinary conservative planning?
  This is a posture question rather than a correctness one, and is the human's.
- Where does the influence grade physically live — a field on existing provenance types, or
  its own carrier? An implementation call best made against the real code.
- Does the gradation axis need to exist before v0 ships, or can the three-grade floor stand
  alone for a while? My guess is it can stand alone, but I hold that weakly.
