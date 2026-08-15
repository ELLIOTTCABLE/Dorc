# 306c — Influence seams: a scoped arc for the kernel round

> Tier: **LLM-authored, lower-reasoning agent** (Opus-class). A PROPOSAL for the active
> conductor to review, re-price, and dispatch — not a spec. Vocabulary and laws come from
> `notes/306b`, which is authoritative. Rules marked **[rationale withheld]** there have
> derivations at `Research/quarantine-DO-NOT-READ/306a`, readable only by a reader authorized
> into that quarantine; do not weaken one to make sequencing easier — ask the human, or route
> through the `/opaque-review` SKILL if the human is unavailable.
>
> Sizing is guesswork. Everything here is one builder's arc by intent.

## §1 — Scope, and what this is not

**One item is owed and closes a standing open question — §3b, the record-lane codes.** That
decision has been open long enough to become the catalog census's only long-term exception, it
is what this design round was convened to resolve, and it is now RULED at `306b` §6e. It should
land.

The rest of the arc is admitted on a weaker criterion: **it is cheaper now than later, and one
item is a design prerequisite for a kernel stage that has not started yet.** No lane currently
in flight is wrong without that remainder. Everything that failed both criteria is in §5.

**Honest value statement, because a conductor is deciding how much builder to spend:** apart
from §3b, v0 buys *structure*, not behaviour — a type that cannot be un-widened, one named seat
where the property is minted, and an output variant that later work extends instead of
retrofits. The engine does nothing observably different at the end of it. If that trade is not
worth a builder right now, the honest minimum is **§3 (both halves) alone**; §2 and §4 can wait
for the stage that needs them.

## §2 — `item-influence-grade` — the type and its one seat

Build `306b` §1's grade as a type in `core`, following the shape the codebase already uses
twice (`Must`/`May` coercion; the `ByObservation`/`ByVouch`/`BySilence` claim tiers):

- Three variants at v0 — `authored-before-contact`, `host-reported`, `host-influenced`.
- **The lowering conversion does not exist.** Widening is free, narrowing does not compile.
  `core::claim`'s `compile_fail` doctests are the precedent for sealing this, and are cheap to
  mirror.
- **One minting seat**, at the intake edge where host-produced bytes become anything else.
  The seat should be singular and named, in the manner of the existing one-named-seat-per-
  invariant discipline; a second minting site is the regression to guard against.
- **The v0 flip is positional and global** — post-ingestion means influenced. This is coarse on
  purpose. It requires no per-value dataflow, which is what keeps this item small: it is a
  phase property carried by construction, not an analysis.

Carriage at v0 is **in-memory only**, terminating at the decision plane.

> **Scope fence, and it is load-bearing:** persisting the grade into the durable is enrichment
> of what the durable holds, which trips this material's own standing tripwire (§4). v0 stays
> in-memory precisely so this arc does not have to fire it. `306b` §3a's rehydration rule is
> therefore *not* built here and is not owed until the durable carries grades.

**Why now rather than later** — and this is narrower than I first argued, so it is worth stating
precisely. The *definition-factoring* work currently in flight is load-plane work, and the load
plane reads source-literal material only (`funcenv-reads-source-literal-plane-only`); it is
structurally uninfluenced, so threading a grade through it would be vacuous. The real argument
is one stage further out: **`28Q`'s world-scopes stage computes availability from probe results,
which is influenced by construction.** Designing that stage with the grade already in hand is
materially different from retrofitting it afterward, and retrofitting an absorbing property
across settled kernel code is the expensive shape. So the deadline is *before that stage's
design*, not this week.

Sizing guess: small. A type, a seal, one mint, and carriage through structures that already
thread provenance.

## §3 — The report-only output, and the consumer that closes a standing question

Two halves, and they are one piece of work: the type, and the thing that has been waiting for
it. **§3b is the owed item; §3a is what makes §3b cheap rather than a rewrite.** A conductor
short on budget should take this section and skip the rest of the arc.

### §3a — `item-report-only-output` — extend the demotion machinery that exists

Build `306b:rul-report-only-output-cannot-plan`: an analysis output that is structurally
incapable of yielding a plan step, so that a target whose intake integrity is lost still gets a
complete analysis and a full report, and cannot get mutation authority.

**Site it as an extension of the solve-certifier's existing consumer floors** rather than as a
new mechanism. `plans/302` already built the shape — a closed outcome whose consumers each
supply a named floor, and a whole-window demotion when the outcome is bad. Intake-integrity loss
wants the same treatment reached for a different reason, and framing it as "a second trigger
into machinery that exists" is both cheaper and more likely to stay coherent than a parallel
path.

Two implementation notes from `306b` §4b, both worth holding:

- **A type, not a flag.** A boolean eventually goes unchecked; the whole value here is that the
  plan-producing conversion is absent rather than guarded.
- **Contain at the analysis output, not at plan emission.** Facts are cross-cutting — survival,
  the wall walk, and the decision record all read them — so a fold that yields ordinary facts
  leaks sideways even when no plan is emitted for that target.

And one consequence worth flagging to whoever builds it, because it reverses a rule they may
have internalised: **this legitimises partial consumption of a malformed record stream.** The
prohibition was only ever justified for consumption that can reach a plan step. Consumption into
a report-only output carries none of that weight, and `306b` §4b makes continuing-to-analyse the
required behaviour rather than a tolerated one.

Sizing guess: medium, dominated by finding the right seat rather than by the code.

### §3b — `item-record-lane-codes` — the owed decision, now ruled

This is the item the round was convened for. The catalog's nine `records-*` codes are **kept**
(`306b` §6e — read it, the reasoning is short and the conclusion reverses an earlier lean that
may still be in circulation). What changes is where they are computed and what may reach them.

The work, in dependency order:

1. **Move the discrimination into the strict admission path.** It already classifies; this is a
   refinement of an existing classification, not new machinery, and it keeps
   refuse-the-whole-attempt unchanged. This is what gives the nine a production emitter and
   retires the census exception.
2. **Re-home the forgiving parser rather than deleting it**, onto §3a's report-only output.
   Its defect was that partially-parsed material could reach a plan-producing path — a property
   of its destination, not its vocabulary — and §3a removes that destination.
3. **Fence it.** No type can privilege one crate over another at this seat, so the remaining
   half is a lexical gate asserting a non-empty walk, matching the gates that already exist for
   comparable seats. Note for the builder: adding an entry to such an allow-list is a governed
   act requiring review, never a local edit — including the entry this work itself adds.
4. **Give the eight their defining cases**, which is what actually closes the census exception,
   and apply `306b` §6a/§6b to them: each names its observation rather than an inferred cause,
   and none of their counts may gate engine control flow. Prose is authored at the conductor or
   human tier as usual — builders mint the structure with the prose explicitly empty.

**Blocked on one human ruling, and only for part 4's disposition:** which of the nine route to
refusal and which to ordinary conservative planning (`306b` §6e's closing paragraph). Parts 1–3
do not wait on it; part 4's cases will need re-blessing if the routing lands differently, which
is cheap.

Sizing guess: small-to-medium, and mostly mechanical once §3a exists.

## §4 — Optional rider: reserve the probing-mode seam

`306b:rul-authority-free-probing-mode` wants a probing mode that structurally deploys no
licensure, credentials, or context escalation. Its consumers — debugging surfaces,
explanation-time re-querying, deliberate degraded operation — do not exist yet.

The cheap move, if the conductor wants it: **mint the mode type with exactly one variant today**
and thread it at the probe-dispatch seam. Additional variants then arrive additively instead of
as a signature change across every call site. If it does not fit the arc, drop it — the human
called it reasonable rather than ruled.

## §5 — Deliberately out of this arc

Each of these failed both admission criteria, and naming them is how this stays one builder's
work rather than a programme.

- **The gradation axis** (`306b` §1c). Genuinely unsettled; a wrong axis costs more than none.
- **Computed-versus-declared grade reconciliation** (`306b` §1d).
- **Persisting the grade, and rehydration** (`306b` §3a/§3b) — durable enrichment, trips §6.
- **Refusal's wider reach** (`306b` §4a) — gated on the open whole-target question at §4c;
  building the narrow version first would foreclose it.
- **Render and marking work** (`306b` §6c) — the display discriminator is open, and the
  dualistic-render direction is deferred by the human. This is also the churniest surface here
  and the least suited to a seam-focused arc.
- **Multi-round machinery** (`306b` §8) — trips §6, and there is prior ruling to reconcile first.
(The legacy record parser is **not** in this list — it moved into the arc at §3b, where it
belongs, since §3a is what makes re-homing it cheap.)

## §6 — The two standing tripwires

Stated in the crate steering law; repeated here for whoever reads this document first. Neither
asks for anything to be built.

- **Any enrichment of what the durable persists, or of what re-ingestion consumes** — stop for
  opaque review before design, not after build.
- **Any probing that is not one-shot** — concurrent, sequential, out-of-order, posthoc, or
  multi-target — stop for opaque review before design.

## §7 — What a reviewer should decide

- How much of the arc is worth a builder, given §1's honest value statement. §3 is the part
  that is owed; §2 and §4 are the part that is merely well-timed.
- Where the grade physically lives — a field on existing provenance types, or its own carrier.
  Best decided against the real code; I have no view worth acting on.
- Whether §3 genuinely extends the certifier's floors or only resembles them. If it is a
  resemblance rather than a shared mechanism, §3 should be dropped from this arc and re-planned;
  a parallel demotion path is worse than no second trigger.
- Whether the §4 rider fits, or is noise this round.
