# 30Rc — receipt build: conductor state

> Tier: quarantined conductor state. `30Ra` owns the design and its rationale; `30Rb` owns
> the build specification; `30Rc1` holds a foreign spec-compliance adjudication. This file
> duplicates none of them. It carries where the work is, what binds, what is owed, and the
> things that live nowhere else.
>
> Written as a RESUMPTION document, not a log. If something here is no longer true, rewrite
> it rather than appending a correction.

## the arc in one paragraph

Replace the single mutable whylog with three immutable receipt species — a plan receipt, an
apply intent written before the first mutating dispatch, and an apply outcome — correlated by
typed identities into a report-only graph. Everything read back is inert by type. Two new
crates: a pure `dorc-receipt`, a `dorc-receipt-crypto` holding the Age/Ed25519 implementations
of its capability traits, and — added by `30Rd` — a `dorc-receipt-local` owning the default
key/store I/O. Stages 0–4 are complete and Stage 5 is complete except its severance, which
`sched-severance-lands-in-d-four` places in D4. The format/identity repair pass and `30Rd` D0
are done. What remains is D1–D4 (the binary must be able to persist and read its own
replacement) and then Stage 6 (D5: the old durable goes). Read *the schedule* below before
dispatching.

## where the work is

| | |
|---|---|
| Build branch | `ai/r30-receipt` @ `b9a0be08`, worktree `.claude/worktrees/r30-receipt` |
| Conductor branch | `ai/r30-conduct`, worktree `.claude/worktrees/r30-conduct` (this file) |
| Base | `ai/main` @ `7693ac6f` — rebased onto it four times during the arc, conflict-free each time. Re-check before folding; the sibling is still moving. |
| Also standing | `ai/r30-hk-stash` @ `234d0da6`, worktree `.claude/worktrees/r30-hkstash` — a measurement lane's evidence; the human said drop it with the rest at cleanup. |

**This file's single home is `ai/r30-conduct`.** The copy in the build worktree is a dead
stub from the arc's first commit. Brief every builder to read it as
`git show ai/r30-conduct:Research/quarantine-DO-NOT-READ/30Rc-receipt-build-conduct.md`.

**A sibling conductor is actively working on `ai/main`** (env-identity, plan-diff,
bootstrap-graph). Rebase the build branch before folding, and re-read `LIVING_STATUS` before
editing it — it was rewritten by that sibling during this arc.

**Worktree/branch cleanup is BLOCKED and is the human's.** Five lane branches
(`ai/r30-receipt-{image,overlay,models,dfix}`, `ai/r30-gate-floor`) and their worktrees are
fully folded by CONTENT, but `git merge-base --is-ancestor` reports NOT contained, because the
build branch was rebased onto a moving `ai/main` mid-arc and their commits are now copies.
Containment cannot be proved, so `-d` refuses and `-D` is forbidden. Leave them; do not force.

| Stage | State |
|---|---|
| 0 laws, crate, deps, vectors | done |
| 1 identity + plain kernel | done |
| 2A apply image · 2B overlay+Age · 2C models+graph | done, folded in that order |
| 3 presented plan + PlanReceipt | substantive; see *owed* below |
| 4 intent / dispatch / outcome | done — gate green both legs @ `4177a589` |
| 5 why / correlation / re-derivation | done except its severance — see the blocker |
| repair pass (order line · identity table · framing) | done @ `b9a0be08` |
| 5A `30Rd` D0 crate, names, vectors, I/O model | done |
| 5A `30Rd` D1 key documents · D2 keyset state machine | done @ `366822be` |
| 5A `30Rd` D3 immutable local store | done @ `f564c7ac`; Windows gate OWED |
| 5A `30Rd` D4 production route | next, once the gate clears |
| 6 rip the old implementation (`30Rd` D5) | not started |

## what is owed

**Stage 4 is DONE. Stage 5 is DONE except its severance**, and Stage 6 is unstarted; both
remainders are the one blocker below and nothing else.

Landed in Stage 5: the from-nothing replay authority mint deleted and the witness re-derived
from its admission; the recorded-versus-recomputed comparison sealed so a live disposition
carrying an execution license can no longer occupy its current arm; source standing and
source material answered as two independent questions rather than one flat enum; a two-way
file-narrow fence over every seat naming the read-back wrapper, verified in both failing
directions; and `--last` confined to `dorc why`, refusing on plan/apply/probe/round-trip/
bundle at parse time, pinned through the binary with a `why --last` control.

Not built, deliberately: the why-phase witness. Until the why route reads receipts its mint
has no honest gate, and building it would put a from-nothing witness back in the lane that
removed one.

`tc-two-accounts-per-apply-document` is now SHAPED but not welded: both accounts sit side by
side on the apply request (`invocation` and `standup_account`), neither derived from the
other, so the caller that knows how thin its session is decides both and the projection seat
decides nothing. Under a thin session nothing is host-reported — the destination is argv, the
five axes unentered — so `authored-before-contact` is TRUE today. When the context grows the
answer moves at the caller, not in the projection.

**A user-facing surface awaits the human:** `--dispatch-without-receipt`, the explicit apply
bypass. `--no-whylog` was deliberately NOT reused: it is subtractive everywhere it appears,
while here the default is a refusal and the flag is what makes the run happen — one token
meaning "write less" on plan and "proceed anyway" on apply is a silent second meaning on a
flag that authorizes mutation. `livetest.sh` passes it beside `--no-whylog` (added, not
swapped); Stage 6's removal of the `--whylog` family will touch that line again. Name and
help text are the human's; the register is `None`, so a rename is free.

**Per-site apply detail does not exist.** The session outcome is whole-artifact, so the real
route records zero site rows and `ApplySiteReport` is exercised by the fixture battery alone.
Inventing rows would be the different-and-wrong record. The outcome's influence reads
`untracked` — truthful, and higher than host-influenced so it cannot under-claim.

One residual for whoever unblocks item 4: an invocation's spelled target is now a distinct
no-conversion type from a resolved destination, but the laundering path that still compiles
is `Spelled(context.destination().as_bytes().to_vec())`. Closing it means
`ResolvedApplyContext` no longer handing out a bare `&str` — that type's own lane. What the
split buys meanwhile is that the mistake requires writing `Spelled` over a resolved answer at
a named constructor: a false claim rather than an argument-position slip. A two-way lexical
census over that constructor is the tree's established alternative; minting one was
deliberately not done at a builder seat.

Three residues from the carriage lane: a diagnostic for a refused planned image (absence is
truthful but silent; the code is mintable, the defining case reads as diagnostics territory);
the binary's own `planned_image` wiring is unobservable from any test target, so the
projection half is covered at its own seat instead (same disclosure class as
`receipt_route.rs`'s existing cut); and `WhyWorld::final_presentation` still takes
`planned_image` and its one caller passes `None`, so a why report would name no image where
the run does — Stage 5's lane.

**Stage 3 residue, deliberate:** the old whylog writer still stands, and seven overlay slots
read `uncollected`. Both are explained under *rulings*; neither is an oversight.

**OWED AND BLOCKING NOTHING BUT HONESTY: a Windows `mise run both gate:full-quiet` at
`f564c7ac`.** D3's WSL leg is green at the tip; its Windows leg is green two commits earlier
at `7f4a4106`, and at the tip Windows has `build` + `check-quiet` + `test` (3089 trials) +
whole-workspace clippy green — which is NOT the completion gate and was not claimed as one.
The delta is the path-routed checks the gate adds. Blocked on the box: 0.93 GiB free of 31.69,
preflight wants 4. Not our mess to reap — `vmmemWSL` (5.8 GiB, drains gradually) is the largest
single holder but the remainder is the human's own workload. Never `DORC_PREFLIGHT=skip`.

**Four `tc-*` from D3, all D4-or-later's:** `dorc_receipt::capability::ReceiptSource` has ZERO
implementors workspace-wide and is exactly the weak shape `30Rd` says to replace — deleting it
touches a `30Rb`-reviewed table, so it stands as a trap a D4 builder would otherwise reach for;
`SignedReceipt::publish(name: &str, …)` likewise survives at two `receipt_edge` seats and D4
must route production through the typed publication instead; a document over BOTH the aggregate
and per-document bounds reports the aggregate one (honest — that is the bound that fired — but
a caller wanting the distinction must re-read); and on macOS both roles resolve to one path
while the spec gives keys the strict permission rule and the store a weaker one, so a
group-readable shared root blocks keys but not receipts. That last is the spec's own
consequence, and macOS validation is deferred regardless.

**D3's `--last` is STRUCTURAL, not behavioural** — worth knowing before anyone "improves" it.
The maximum-order cohort is the ONLY selection the store offers: there is no newest-complete
and no next-one-down, so falling back past a damaged newest candidate is not a call any caller
can make from anything in the API. Every member at the top order is retained and ambiguity is
reported rather than broken — deliberately no receipt-ID tie-break, since that would pick a
document by the value least related to when it was written.

**D4 riders for `ELLIOTTCABLE/Dorc#1`** (plan trailer naming its receipt; human-pointed, LOW
priority, must not redirect D4). Split deliberately:

- **TAKE: retrieval by receipt ID.** The ID already satisfies the ask — controller-minted,
  opaque (random, so it leaks no hostname, path, or content), stable, and already carried in
  both the filename and the signed header. Only the lookup is missing, and D4 builds store
  reads anyway. It does NOT reopen D3's "one selection only" property: that property forbids
  a second RANKING (a newest-complete, a next-one-down) so partial-newest fallback stays
  unrepresentable. An exact-match retrieval is not a ranking.
- **LEAVE A SEAM, BUILD NOTHING: the trailer itself.** Two plumbing constraints only, no
  surface and no decision: the durable is published BEFORE the artifact is emitted (the issue
  requires it — a trailer must never name a path that was never made durable), and the
  publication result carries its own path and ID rather than dropping them on the floor.
  That reduces the feature to a later render-side lane.
- **DO NOT decide, and do not let a builder decide:** whether a pointer-trailer is compatible
  with `spike/CLAUDE.md two-surfaces` (the artifact is "byte-floored and receipt-free"; a
  trailer is receipt material embedded in it — this is the real design question and it is the
  human's); the opaque-ID-versus-literal-path config axis (the human has explicitly not
  decided, and the path leaks homedir/hostname into an artifact that may be committed or
  attached to a report); and whether a failed durable write refuses the plan or emits it
  untrailered (also explicitly undecided). The issue's `dorc why --diff` re-read is a NEW
  reingestion path and sits under `rul-durable-contents-reviewed-before-design`.
  Byte-identity is not at risk either way: the trailer is excluded from it by the human's own
  ruling, and the emitted sh is mutable by the user regardless.

**BLOCKING THE MERGE COMMIT, and the human's: `apply-plan-not-dispatchable` has no honest
trigger under 16 MiB.** The case was ours, and it only ever ran because of the
canonical-payload echo main deleted — so it was never honestly driven. Measured, not assumed:
`dorc apply --host --plan FILE` hands one opaque stream to `image_of_external_stream`, which
builds the entry itself with `path: None`, so every path-shaped refusal is structurally
unreachable rather than merely unused; the other two reason arms are fixed by the
single-assignment shape and by a binary that links no signer. **The only caller-influenced
refusals on the whole route are the two size bounds** (16 and 24 MiB).

**THE STRUCTURAL FINDING, which outlives this one case:** a deterministic pre-network apply
refusal has no honest driver at all. The loom's apply arm reads a plan only to confirm it
exists, discards it, and renders from a declared `EdgeFault` — which is documented for
NONDETERMINISTIC edge outcomes, and is therefore the wrong instrument for a decision pure
production code makes. And the replay harness resolves every case-relative file through one
bounded reader capped at **64 KiB**, so no case can hand any consumer a file 256× larger than
that ceiling — generated at run time or committed makes no difference, because the refusal is
on the READ. The honest route the defect registry's own rule demands is unreachable here not
because the scenario is hard to construct but because **the harness that would observe it
refuses inputs three orders of magnitude smaller than the bound under test.**

Two harness changes would close it and neither was made: raising a bounded reader over
untrusted input by 256×, and teaching the loom's apply arm to drive the real dispatch prelude.
The second is a smaller step than it sounds — the sibling case was repaired by exactly that
move at one lib seat — but the first is a governed surface in its own right.

**Note for D4:** `intent-not-published`, one of this code's other reason words, is unreachable
today only because the binary links no signer. D4 links one. So this diagnostic becomes
honestly drivable at D4 without any harness change at all.

So the trade is exactly two options and both are bad: a >16 MiB committed fixture, or delete
the case — which orphans its code, since every code owes exactly one, and deleting the code
takes a production diagnostic with it. A third option was considered and does NOT survive: a
tighter route-specific bound on the external stream (16 MiB is mis-sized for a reviewed shell
script) collapses because any bound small enough for a comfortable fixture is plausibly too
tight for a real generated plan. Explicitly refused as a route: a new frontmatter key or an
injected bound — that is the testing-into-production bypass main just spent 82 commits
removing, and taking it here would make this merge dishonour the thing it merges.

**Consequence: the merge sits UNCOMMITTED** in `.claude/worktrees/r30-receipt`, everything
staged, pre-commit refusing on that one case. Fragile, and holding it was the deliberate
choice over laundering a structurally-invalid case into a removal at a conductor seat.

**A defect of ours the bypass was hiding:** our arc put the dispatch gate in front of the
transport path and never updated `transport-crlf-refused` / `transport-apply-failed`, whose
invocations can no longer reach a transport diagnostic at all. Repaired in the same lane; the
loom now models production's ordering. Nothing caught it because the loom's apply driver was
skipping the gate — the same bypass, hiding our own regression.

**A fence that now guards less than it says — OWED to whichever stage rewires that seat.**
`the_driver_takes_its_authority_from_its_admission` asserts over `cli/src/main.rs`, and its
own doc calls that "the one file that answers a live intake". After main's refactor it is not:
`engine.rs` answers it, and legitimately spells the no-intake mint for its fixture arm. **The
fence still passes and covers nothing.** Moving or widening it is a governed act and was
correctly not done mid-merge. Written down rather than remembered, because a passing guard
that covers nothing is this arc's most recurrent hazard.

**Tooling gap, recorded:** `dorc-loom publish` cannot distinguish an in-progress merge from an
uncommitted publish and refuses, so a merge that legitimately moves transcripts has to go
through the dump route instead. The workaround was reported rather than swallowed, and its
result was made sound by mechanically confirming every refreshed transcript differed in
exactly one line and that line was the digest. The publisher should learn to recognise a merge
state; a later tooling lane.

**Owed to D4:** refuse to EMIT an undated document at the production composition root, sited
for trivial removal when stable-format output becomes supported. It cannot live at a lib seam
— a refusal there would refuse the very runs that want an undated artifact. Also D4's: the
publication-gate atomicity change, and the `results::replayed_records` severance.

**Two `tc-*` awaiting the human, one root.** `30Rd` requires the Unix landing be owned by the
effective user, and the effective uid is **not reachable in safe Rust** from this crate's
dependency set — `libc::geteuid` is `unsafe fn` and the workspace `forbid`s unsafe, so it
needs `rustix`/`nix` as a new PRODUCTION dependency of a security-relevant crate, unscouted by
`30Rd`. `tc-non-following-open-needs-a-platform-call` has the same root: opening an existing
member is `symlink_metadata`-then-open, so a swap between them stays possible (creation is
unaffected — exclusive create is one act; and inspect-and-read are on the retained handle, so
that ordering is real). Interim, and it fails safe: ownership is a typed explicitly-
unestablished fact (`OwnerCheck::NotEstablished`), so a reopened keyset can never read as
owner-verified. The mode half is fully implemented. **Does not block D3.**

**Zeroization is partial, and will be mis-summarized if not recorded**: `ed25519` 2.2.3
declares no `zeroize` feature, so its scrubs of the PKCS#8 intermediate buffer and of
`KeypairBytes` are unreachable whatever we configure downstream. `30Rd` already disclaims
memory-erasure guarantees, so nothing is broken — but nobody may later write that the private
key material is scrubbed.

**A fence gap worth closing when something touches it:** a malformed `DORC_FIXTURE_CLOCK_MS`
is read at the process edge and is NOT structurally test-only, so a production run can be
handed a fixture clock value. Only the clock, not identity — but it is the shape
`rul-fixture-identity-never-production` warns about ("environment presence alone never grants
parser authority").

**Watch, not chased:** one gate run reddened at `pin28-helper-package-entrypoints-discarded`
(`ap-2-exec`, rendered apply rc=1) inside the real-tools lane, then did not reproduce across an
isolated re-run, a full real-tools pass (222/222), and two clean full gates. Reported as
cause-unestablished rather than unrelated — the builder did not prove its change uninvolved.
Plausible mechanism: the shared `target/` under a multi-task gate with a sibling conductor
live. If a lane sees this shape again, that recurrence is the signal.

**Arc close, hard:** see the final section. Nothing ships with it outstanding.

## the schedule (conductor-chosen 2026-08-25, human-delegated; HONOR IT)

`30Rd` arrived after Stage 5 and made a minimal production durable edge REQUIRED — the
binary must be able to persist and read its replacement before the old whylog goes. The
human delegated scheduling and asked that ambiguous landings be chosen ahead of time rather
than drifted into. These are those choices:

- **`sched-repair-pass-runs-first`** — the sitting changed things Stages 0–4 already built.
  The format half repairs BEFORE `30Rd` work starts: the new top-level `order` line and its
  token type, every committed conformance vector and fixture re-cut, and the exhaustive
  identity table for the two plan identities (assigned to Stage 0, reviewed before Stage 1,
  and both IDs already minted in Stage 3 — so the table is written now and the existing mints
  validated against it). Doing format work after more code depends on the format is strictly
  worse, and the human leaned this way too.
- **`sched-gate-atomicity-rides-d-four`** — the publication gate must consume ONE private
  value binding intent, image witness, policy, and `30Rd`'s publication proof, with the permit
  minted atomically from it. That proof does not exist until D4, so this cannot repair early.
- **`sched-severance-lands-in-d-four`** — Stage 5's unmet exit (`results::replayed_records`
  laundering a durable into live evidence) closes in D4, beside the why-route rewiring, which
  is the first moment `why` can answer from a receipt and that seat loses its caller. Stage 5
  is formally complete at D4, not before. **Chosen in advance; do not re-open it later for
  convenience.**
- **`sched-five-a-is-d-zero-through-d-four`** — Stage 5A ≡ D0–D4; Stage 6 ≡ D5. `30Rb` reads
  both ways (5A says "every stage in `30Rd`", which would swallow D5 and leave Stage 6 empty);
  D5's own "Only after D4 exit" header settles it.
- **`sched-the-bypass-flag-dies-in-d-four`** — `--dispatch-without-receipt` is kept only while
  it is load-bearing: it is what keeps `livetest` running until the real route can publish.
  D4 requires the default real apply path to traverse the concrete gate and says a configured
  bypass cannot satisfy that exit, so the flag and its `livetest.sh` use are removed there,
  not carried into Stage 6.

## rulings that bind

### Human — by spec amendment (`30Rb` trailer) and direct ruling

- **Source ordinal = deterministic acquired-source table order**, never dynamic
  load-occurrence order. `SourceRole` is V1 table classification only. Repeated/multi-role/
  multi-target semantics stay with `30I`'s occurrence account. NACKED: creating a private
  construction boundary for the source vector.
- **`PlanPlane::PresentedPlanIdentity` is exactly `PresentedPlanId`.** A private
  final-presentation witness carries three identities atomically into projection but is not an
  identity and cannot satisfy identity APIs; minted only after both views and the artifact
  bytes are final; cross-plan field substitution pinned; no generic identity-bundle framework.
- **`PlanningInputId` membership** (sibling conductor, human-skimmed): it identifies the
  complete decision-relevant VALUE presented to the planner including authority scope — two
  invocations with genuinely identical scoped inputs MAY share it, and `ReceiptId` is what
  distinguishes events. Includes authored state (ordered source table with digests, roles,
  paths, resolution context; load-occurrence account; oracle provenance; controller semantics;
  target/context/generation; every parsed policy value affecting analysis or settlement) AND
  admitted world state (admission outcome plus every bounded typed controller-attributed
  record, in the planner's own order, with attempt/generation/host/source-set attribution).
  Excludes dispositions, render decisions, narratives, storage policy, the presented-plan
  identity, artifact bytes. **Without the admitted world state a converged host and a drifted
  host mint the same identity.**
- **Both in-process and e2e testing are eventually owed.** Delaying e2e while supporting
  components land across later stages is reasonable — but only where the risk is "a bug e2e
  would have caught is caught by a later builder instead". NOT where it risks eventual
  incorrectness. Late detection is affordable; undetectable is not.
- **Test architecture must not drive build order** — exclusive categories, and not a licence
  to disregard tests. For tests non-productively blocking build order, over the precise
  predicted range: leave-red (costly — successor churn), disable (if tooling reaches), remove
  (extremis). **Restored verbatim at arc close.**
- **Builder-authored steering prose stays out of the tree this arc**, reviewed at close. No
  builder writes or edits a `CLAUDE.md`; invariant prose is reported to the conductor and
  collected here. This arc's edits were removed at `e55a7fe9`.

### Conductor — settled, do not re-litigate

- **Two crates, not one** (against `30Rb:result-and-exit` item 1): `age` pulls `rand`, and root
  `AGENTS.md` forbids nondeterministic deps in the kernel's graph. Every authority mint stays
  in the pure crate, so the split strengthens verify-before-interpret rather than relocating
  it. `sha2` is the one edge reaching the kernel. The foreign adjudication graded this a spec
  oversight, not a departure.
- **The grammar table was amended four times**: `invocation.attempt`, `admission.stream`,
  `render-decision.member`, and `apply-context` at tag 16. The governing principle: *"record
  more" is the tiebreaker where the spec is SILENT, never a licence to grow a REQUIRED field
  list whenever something is droppable.* Applied in the negative to narrative's dropped site.
- **A rich-to-plain remint mints its OWN identity.** An earlier ruling that it keeps the
  original's was reverted at `cd05d080`: the spec calls the output a new document, identities
  are controller-minted per document, and the graph is required to keep
  same-identity-different-bytes distinguishable. A remint has no recorded back-link to its
  origin; adding one is a grammar change. Deliberate gap, not an omission.
- **Plain intents carry the image identity and `withheld-plain` only.** `30Rb` promises
  topology summaries in prose that the reviewed row cannot hold. *Widening a reviewed wire
  table to satisfy a sentence of prose inverts the authority order* — the table was the
  reviewed artifact.
- **Seven overlay slots read `uncollected`**, not `captured`: argv, record stream, load
  custody, probe-ship source, survival poison, render detail, licensor locus. Each holds
  something, but writing it means deciding a durable RENDERING, which is a governed decision at
  the wrong seat. `uncollected` differs from `unavailable` (which would claim the run held
  nothing), so the loss is recorded in-band. A successor funding these ADDS rather than
  corrects.
- **The apply route mints a THIN session, and grows it** (human-typed 2026-08-25, superseding
  this arc's earlier bypass-not-standup ruling). The stop condition forbids FABRICATING a
  target identity; it does not forbid an honest one that establishes little. "No context was
  entered — nothing escalated, nothing chrooted, running as whatever the destination
  resolves to" is a true, controller-known statement, and the six defaultless fields were a
  gap in the type rather than a fact about the world. What matters and is preserved: ONE mint
  seat, chronologically fixed, consumed once per host, growing richer as machinery arrives.
  Widen only in the honest direction; never add a default.
  The transport layer still refuses reusable multiplexing deliberately, with an attribution
  argument written at the seat: a shared master is a socket at a path the user's config chose,
  so an attempt could inherit a channel it never opened and the host it is attributed to stops
  being the controller's own fact. A thin session needs no multiplexing and does not reopen it.
- **Ordinary `dorc apply --host` REFUSES; live acceptance rides an explicit harness-injected
  `ConfiguredReceiptBypass`** — never a default, never TTY inference, never satisfied by an
  ordinary receipt write succeeding (conveyed ruling, 2026-08-25). **No minimal key provider
  ships now**; that reopens the deliberately deferred production key-custody surface. The
  binary therefore still cannot sign, so the refusal is what the required arm becomes until a
  provider lands; the deterministic route takes the published arm with injected fixtures.
- **The crypto crate is a DEV-dependency of `cli`**, so the binary is structurally unable to
  sign rather than conditionally declining. It flips the day a key provider lands.
  Consequence: there is no second active writer, so the old whylog writer stays — Stage 3's
  literal exit clause is unmet by design, not by neglect.
- **`dorc-apply-context/1`**, a nested exact container inside one opaque field carrying the
  five non-destination context axes, on the `ApplyArtifactImage` precedent; the destination
  rides the sibling `target-name` slot so neither repeats the other. A strawman wire name
  like every other pre-user one — rename in place, never adapt.
- **`30Rb`'s prose promises more than its reviewed tables hold in at least two places**
  (plain-intent topology summaries; apply-intent controller semantics). The tables were the
  reviewed artifact and they win; a third instance gets the same answer.
- **The writer-side aggregate bound fails per species**: the intent's required arm REFUSES
  (it binds exact bytes, so omission cannot mint the permit), the outcome OMITS with
  `omitted-limit` (a bounded report is still a true one).
- **`PresentedPlanId`'s sections are LENGTH-FRAMED**, matching the sibling planner-input
  encoding. Measured 2026-08-25: the header-delimited form was not injective by construction —
  two of its sections carry bytes that may spell a section header, and the apply render is the
  book verbatim, so a book containing a bare `== diags ==` line puts two exact header lines in
  the canon and the content/next-section split is unrecoverable. The builder found this,
  declined to repair a governed surface, and pinned it; the deferral was OVERRULED. Ground: the
  identity never authorizes, but it IS recompute-and-compared, so a collision produces a
  confident wrong explanation — the top of `271:rul-sin-ordering`. Cost of repair is lowest
  before anything external exists; pre-user reshape-in-place governs.
- **The opacity line is about US, not about "why" comments.** A comment naming *what goes
  wrong otherwise* is forbidden when the harm it describes is a fact about our construction,
  our threat model, or what someone could do to us. A comment naming a documented property of
  a public primitive — the OS, a library — is ordinary engineering and stays, and stays
  ESPECIALLY when its job is stopping a future agent from making a change that is on a stop-
  condition list. Ruled over the sync-never-retried comment in the keyset sequence: the harm
  it names is a durability misreport by the kernel, which is public lore, and the change it
  prevents is a named stop condition. Four sibling comments that named OUR partial states were
  trimmed in the same pass; that asymmetry is the rule.
- **Clocklessness is a REQUIRED capability, not a test artifact** (human-typed 2026-08-25):
  stable tests need it now, and CI users eventually want a stable-format artifact they can diff
  to ask "did the whylog change". So clockless support is built right out to the edge, and the
  refusal to EMIT an undated receipt sits AT the production composition root, sited to be
  trivially swapped out when stable-format output becomes a supported mode. Because production
  never emits one, `--last` never sorts one, so the ruled principle holds by construction: an
  undated run never silently causes older history to be selected. Do not "simplify" this by
  making the all-zeroes token unrepresentable — that reading was floated by this conductor and
  corrected by the human. Low priority; no live exposure until the edge emits.
- **The order token**: a 20-digit value above `u64::MAX` is ADMITTED (the grammar fixes width,
  not range; refusing would make valid-but-large indistinguishable from malformed at the seat
  least able to tell). A REMINT takes the caller's order, never its origin's — a second document
  at a second moment, and inheriting would seat two documents at one store position.
- **`roots` = `plan.sh` at 0, bundle roots appended.** The alternative makes a `root` line
  assert one thing in the one-file case and another once a dependency appears; a durable
  field whose meaning changes shape under a non-trivial population is the drift the
  ordinals carry-in describes. Pinned by a test that the two cases agree on root 0.
- **The final-presentation witness mints AFTER the artifact bytes**, retiring the interim
  `planned_image: None`. Not a new position — it is the already-ruled one, which the
  plan-time absence of an image had deferred. The fence at that seat moves with it and is
  verified in its failing direction.
- `age` 0.12.x with `armor` explicit (not a default feature). `core::spine::InvocationMode`
  untouched. `survival.outcome` and `site-classification.class` split to eight tokens each
  rather than merging variants with differing repair meaning. `SessionOutcome::LostAfterSend`
  maps to `unknown`, never `transport-failed`. `RecordedSurvival::wall()` stays **withheld** —
  its field is typed as a leaf, its only source is an ordinal.

## carry-ins — what the typesystem does not hold

*The deliverable for whoever synthesizes steering prose at arc close. Themed, not
chronological. Every item was found by a builder while building.*

### Identity and authority are licensed by position, not by type

The identity mints take bare bytes; nothing in their signatures can tell a settled surface from
a fragment hashed early. What makes them honest is that their one caller sits downstream of a
plan only its single constructor can produce, after settlement quiesced and the certifier latch
was spent. **Move such a call earlier, or add a second one somewhere convenient, and the type
system will not notice — only the lexical fences will, and only if nobody widens one to make a
build pass.** The same is true of the presentation witness: every input it takes exists long
before settlement quiesces, so only its call site makes it honest.

The witness's cross-plan check is deliberately weaker than it looks. It compares the one
identity the witness and the Spine both hold, refusing on disagreement and on absence — which
catches a whole-witness swap, the realistic accident. It cannot catch a witness whose other two
identities were wrong from birth, because nothing downstream holds a copy to disagree with.
That is why the constructor COMPUTES both rather than accepting either; a parameter accepting
one would dissolve the guarantee while every test stayed green.

### Fences, and how they fail

**A fence firing is answered by adding a TRUE entry, or by fixing the code — never by loosening
the fence.** Adding a true entry maintains the guarantee; widening a matcher or removing an
entry weakens it.

There are two influence censuses and neither is a formality. The authored one is an assertion
about the world and deliberately lists fixture seats, because a lexical walk cannot tell them
from production code. The untracked one is an **inventory of production seams staging real
holes**, and its value comes entirely from counting only those. **Escaping the first by
entering the second passes the gate and quietly destroys what the inventory measures.**

The rehydration fence watches the influence-floor VARIANT, so it catches deciding a grade at
projection time as well as reading one back — deliberately over-broad, and it has already
caught its own author's new code. Thread the account through; widening is a decision, not a
list edit.

A fence that substring-matches a crate name also matches every identifier ending in those
bytes. Match whole identifiers, and verify a repaired fence in the FAILING direction — one
verified only against false positives is half-verified.

### Ordinals, positions, and things range-checked but never sense-checked

Region rows define an ordinal space by position and render rows reference it by a bare integer;
a detail entry is keyed by its record's position. **A position is range-checked and never
sense-checked**, so a wrong one enriches whichever row shares the integer while the document
still validates cleanly. Number and emit in ONE walk. The source ordinal means table position,
and the tree's older phrase "load order" coincides with it only because an earlier ruling made
identifier order and acquisition order agree — decouple those and every recorded ordinal
silently changes what it claims, without touching receipt code.

A row's atoms write positionally into the grammar table's field order and nothing relates the
two; the round-trip test bites ONLY because every fixture row uses distinct values in
same-typed fields. **The numbers are the mechanism, not decoration.** What converts a
cross-lane grammar widening from corruption into a test failure is that every row goes out
through the writer's own emitter, which refuses a row it could not read back — a projection
hand-building a record bypasses the one thing that caught it.

### A capability bundle that quietly grew a decision

`ApplyPublishingCapabilities` (né `ApplyPublication`) was described by its builder as a plain
bundle of injected capabilities, and was — except for one field that became the recorded
influence account on every intent row the route writes. That is state a decision rests on,
behind public fields, under a name that reads like *a publication happened*. The rules on
publication types did not formally bite a bundle; they would have bitten whatever a later
agent grew it into. **The name is the hazard as much as the fields are** — a type whose name
asserts an outcome invites being read for that outcome. The repair took both halves: the
account moved onto the request beside the invocation's own, and the bundle was renamed to say
what it is and privatised behind a constructor.

### Promises the writer must fund

`captured` is now a promise: a slot marked captured with no entry produces a document its own
reader refuses, and the writer's account check is the only thing turning that from a
reader-side mystery into a refusal at the seat that caused it. The temptation is widening the
held-set over a slot convenient to flag but not designed to write — a one-word change that
breaks the document.

The recorded file-mode type has **no unknown arm**, deliberately, so "I don't know whether this
is an execution input" is unrepresentable and the obligation to refuse lands on the conversion
seat and nowhere else. Today `unused` is a MEASUREMENT, not a default: everything published is
created user-only and non-executable, and the artifact is invoked as an argument to an
interpreter rather than through its own exec bit, with dependencies reached by sourcing. If
either fact changes, `unused` becomes a lie and the type has no arm to say so.

### Positional correctness in the format

A plain document's signed body and its skeleton span are byte-identical, so every plain test
passes whichever you hand the parser; they diverge only for rich. The skeleton span and the
armor span must both be slices of the ONE located body, taken in a single pass — locate the
armor separately and the bytes verified and the bytes opened could differ with nothing
complaining.

Line-ending normalization is safe only because it happens before serialization, hence before
signing, and because nothing on the read path normalizes. **It is correct because of where it
sits, not what it does.** Its paired trim-and-append spans a crate boundary with nothing
enforcing the pairing, and the append is legitimate only because it happens after verification.

The region terminator search rests on another project's alphabet — base64 excludes the hyphen,
so no armor line can spell the terminator. Now asserted locally against a real sealed region,
so an upstream change fails one loud test instead of every rich document mysteriously failing
to locate.

### The apply lane

`dorc apply --host` returns before any book is read: no Spine, no plan, no presentation, and
today no durable at all. **The receipt graph's many-to-many shape follows from this**, not from
taste — the admin owns the mapping, and the apply invocation genuinely does not know which plan
produced the bytes it was handed unless told. `OriginatingPlans::Unavailable` is a true answer,
not a missing feature. An agent assuming the intent seat extends the plan-side write will reach
for a Spine that is not there and be tempted to synthesize the link.

The two routes to a dispatch permit must stay unreachable from one another, and the asymmetry
in the post-dispatch failure set is load-bearing: six of seven arms narrow to nothing and only
the durable one reaches the continuation, which is what stops a future agent widening a match
arm until a lost host is handled like a logging problem.

The file↔published-path correspondence exists only at the PLACING act and nowhere after, so it
is recorded by the same call that chooses each destination — which is what makes a forgotten
siting a refusal rather than a silently short edge set. Two consequences a successor will meet:
an absorbed dependency sites at the SAME path as the bytes that swallowed it, so
`parent == child ⇒ no edge` covers absorption with no special-casing (give absorbed files their
own destinations and edges appear naming files the artifact does not publish); and every edge
today's four forms can state is `loads` — `Contains` needs both ends published and none does,
so its absence is a fact about the forms, not an omission. A planned image that cannot be built
reads as absence, not failure; `presented-plan` already spells absence, so no wire table moved.

The conversion originally refused a set carrying dependency files rather than recording an
obvious-looking topology. "The plan loads everything beside it" is true for the bundled form and false for the
mirrored one the moment one dependency sources another — and that falsehood would sit inside a
container whose entire promise is reproducing exactly what the apply uses. The information
exists (the bundle projection knows which entry each root materializes; the load account knows
which occurrence encloses which) but the selection discards both before the artifact set is
built. **Restoring that carriage is the work; inventing the edges is a different and wrong
record.**

### Tests that prove less than they appear to

Recurring, found independently in four guises: a negative test passing for a reason other than
the one it claims; a round-trip test made vacuous by an encoder handing back stored bytes;
tests written downstream of a decision, which encode that decision and cannot falsify it; and a
test that re-implements the seat it means to exercise, demonstrating a capability it does not
observe.

A fifth guise, found by falsification in the `--last` lane and the sharpest of them: **an
exit-status assertion on a refusal is vacuous whenever a DIFFERENT refusal fires anyway.**
With the new guard disabled, `dorc plan --last book.sh` still exits non-zero — on
file-not-found, because the book is absent from the cwd — so a test checking only "it
refused" passes with the guard removed. Only the assertion naming the exact slug catches the
regression. The tidying instinct that replaces a slug check with a plain rc check is
therefore a silent un-testing, and the case now carries that measurement in its own doc so a
successor knows what they would be deleting.

**Swept, 2026-08-25, mechanically rather than by eye** — every `compile_fail` in the arc's
crates was flipped to a plain example and its actual compiler error read, because that error
is the only evidence of what a pin proves. 33 pins across `receipt`, `receipt-crypto`,
`receipt-local`, and `plan`. Reference-receiver degradation: no instances beyond the two
already repaired. Everything else fails on its intended property. Two residues, neither
repaired: two `receipt` pins fail on *"no method named X"*, which proves the absence of those
NAMES rather than the property — sound only because each is paired with a structural pin
carrying the real seal; and one **pre-arc** pin in `plan/src/certifier_trip.rs` fails on
ARITY (`E0061`), so it proves the function takes more than two arguments, not that the trip
witness is required — remove the witness, add two other parameters, and it still passes. The
stronger form is constructing the witness directly, whose private field is immune to arity
drift. It arrived via `ai/main` and is the sibling conductor's surface, so it was routed, not
touched.

A seventh, and the most generalizable of them: **a `compile_fail` test proves only that
SOMETHING failed to compile.** Unless it pins the exact error it passes for any reason. Two
mechanisms measured in one lane: a clone-pin whose receiver is a REFERENCE degrades the hard
`E0599` into a mere lint, which rustdoc scores as a successful compile; and a pin carrying a
trait signature that has since moved passes on the signature mismatch rather than on the seal
it claims to test. Demand the property by value; pin the error; verify in the failing
direction.

A sixth: **a structural check spelled lexically is satisfiable by a comment.** The identity
table's opener/terminator check ran `contains` over encoder source text; it now asserts
equality against the encoders' own `pub const`s, so a rename moves both or fails to compile.
And when the presented-plan ambiguity was repaired, its pin was REMOVED rather than flipped —
its assertion ("exactly one section-header line") becomes meaningless once no header lines
exist, so it would have passed for the wrong reason. What replaced it is positive: a walker
that takes a canon apart by its declared lengths, because **recoverability IS injectivity**,
plus the spoofing case a book can actually construct. Negatives that go vacuous when the
defect is fixed want replacing with the positive property, not re-pointing.

Countermeasures now in use: **pin every negative to its exact refusal in a committed table** —
the overlay family fails closed in ways that look alike, so "it was rejected" is satisfied by
several different bugs; keep one seat driven by both the binary and the test; and verify a
repaired guard in its failing direction.

**Measure premises rather than assuming them.** Builders repeatedly found their own
just-written tests asserting behaviour the tree does not have — an intake fold nobody knew
about, a header counting site facts rather than records, a fixture target that was not what was
assumed, and a reader refusing structurally rather than at the signature. Reasoning from
plausible line numbers has failed under measurement more than once in this arc; measure.

### Byte-exact fixtures

**Two fences guard them and neither knows about the other.** `.gitattributes` closes git's
check-in filter; it does not reach hk's fixers, and a spell-checker's fixer once rewrote base64
inside a committed vector after an unrelated format run. The exclusion key does not help,
because the tool honours ignore rules for walked paths but not for explicitly-passed ones and
the hook passes the file list. **A frozen vector is protected by whatever the LAST tool to
touch it does, not by the attribute you gave it** — and the way you discover the open fence is
that a test goes red for a reason that reads like a parser bug. Never trust the worktree as
evidence of what was committed; verify with `check-attr` plus `cat-file -p :<path>`.

### Other seams a successor will meet

The why-side world never populates the Spine's durable arm — no invocation, presented plan,
record stream, or admission — so projecting from it refuses today. Whether that driver should
populate its own arm is a real design question, not an oversight to patch.

There is no apply executor, and the session outcome is whole-artifact rather than per-site.
Per-site rows come from the deterministic route; the real route honestly records zero, because
the field counts rows recorded rather than sites that existed.

The source vector's book row is derived from its declared ROLE, never position. A positional
fallback beside it answers with the last element when no book is found — a real but separable
defect the human is holding.

## the completion gate

`mise run both gate:full-quiet` now opens with a **discovery floor** (`gate:floor`) that
refuses a run which would check nothing. Built during this arc after a measured false green:
the gate returned success having executed zero checks. **Check the floor's line, not the exit
code** — recent lanes measure ~10 checks over ~150 files per leg; a short pass is unambiguously
wrong.

Three vacuity causes were found, and only one reproduces: an unreadable default-branch setting
(the tool then guesses a remote head, fails, falls back, prints git fatals and **exits 0**); a
non-reproducing leg-local instance, most likely per-worktree state caching; and — **the one
that matters for Stage 6** — **a pure-deletion changeset selects ZERO checks**, because every
glob is matched against paths no longer on disk. Stage 6 is a deletion stage. Without the floor
its gate would have been vacuous by construction. Accepted cost: a deletion-only commit now
refuses.

**A pathspec commit CAN sweep, but not in an agent's environment** (measured 2026-08-25;
evidence and a re-runnable harness on `ai/r30-hk-stash` @ `234d0da6`, worktree
`.claude/worktrees/r30-hkstash`, left standing for the human). `hk.pkl`'s `cargo_fmt` carries
`stage = "spike/**/*.rs"`, and in FIX mode hk stages every dirty match of that glob, not the
files the step was handed — proven by a run that committed files `cargo fmt` cannot have
touched. It reaches a pathspec commit because git re-reads its partial-commit false index
*after* the hook, so a hook-side `git add` lands. `HK_FIX=0` prevents it outright and is
verified arriving inside the hook process itself, so no agent can hit it; the builder report
that prompted this is better explained by a broader-than-believed pathspec.

Two things a successor should carry anyway. **The human's cell is not fully protected by
stashing**: `cargo fmt --all` is workspace-global, so a file clean in the worktree but
unformatted in HEAD is rewritten for the first time — a stash cannot hide a change that does
not exist yet — and the glob then commits it. And **"pre-commit is check-only under agents"
now rests on `HK_FIX=0` alone**; `HK_STASH=none` removed the second independent guard, so it
is a single point of failure with a silent failure mode. Proposed fix, measured, NOT landed
(the human's to route): drop the `stage` keys from `cargo_fmt` and its detached twin, so hk
falls back to staging the step's own files as `typos` already does. Cost: a collateral
reformat is left visibly unstaged rather than silently committed.

Plain-git rider worth knowing regardless: `git commit -- <dir>` commits everything dirty
under that directory, and `git commit -- <paths>` commits the WORKTREE content of those
paths, ignoring what was staged.

**Brief every lane on the gate's SHAPE, not just its name** — two lanes stranded on it. The
completion gate is ONE `mise run both gate:full-quiet` at the FINAL tip, foreground, Windows
leg first. Per-leg runs during a lane are fine as private feedback and are NOT the gate: a
Windows leg green at one commit and a WSL leg green at another verifies neither tip. And the
Windows-first rule only helps the first pairing — the WSL leg's build cache is what inflates
the pressure the Windows probe reads, so a *second* Windows run after a WSL run is the one
that gets refused. When preflight refuses, **retry inside the turn** (it refuses in seconds,
so a poll is nearly free); ending the turn to wait strands the lane, because the wake-up goes
to the conductor and nobody is watching the thing being waited on. Never `DORC_PREFLIGHT=skip`
— that bound exists because this box OOM'd twice.

**Windows relink lock, unresolved and the human's.** `cargo test --no-run` intermittently
cannot remove `spike/target/debug/dorc.exe` (`Access is denied`). Established: not the build —
it survives deleting the binary, and only that step trips it. Suspected but NOT proven: a sync
client, since Windows `CARGO_TARGET_DIR` resolves inside the synced tree at `mise.toml:118`
while the WSL arm points outside it and has never hit it. Candidate fixes: exclude `**/target`
from syncing, or extend that Windows arm to a per-worktree cache outside the tree. A
discriminator exists — point the target dir outside the synced tree for one run — and has not
been run.

## open with the human

**The laundering seat is `results::replayed_records`, not the authority mint** (measured
2026-08-25). `of_admitted_replay`'s from-nothing mint is deleted and the witness now derives
from the admission that earned it — but that was a re-attribution, not the severance. On the
`dorc why` route, `replayed_records` converts a durable's record stream into the SAME
admitted-evidence types a live probe intake produces, and below that seat replayed bytes are
indistinguishable from live measurement: real replace/guard licenses are minted, a real plan
is projected and decided, a presented-plan identity is minted. `why` returns before printing,
so nothing reaches stdout — the licenses are still genuinely constructed. The precondition for
removing it is exactly the blocker below: the binary must be able to READ a receipt, so that
`why` answers from recorded content and `replayed_records` has no caller.

**`open-the-arc-cannot-exit-without-a-key-provider` — BLOCKING Stages 5 and 6.** The exit
criteria and the no-key-provider ruling are mutually inconsistent, and this is the second
place this arc has found both sanctioned outcomes closed.

Every valid receipt carries a signature. The crypto crate is a dev-dependency of `cli`, so
the shipped binary can neither sign nor verify — it writes and reads no receipts at all.
Flipping the dependency does not help: without a key there is nothing to sign with, so the
binary would decline at runtime instead of being unable, which is strictly worse and equally
non-writing. So today the receipt family is exercised only under test with injected fixture
capabilities, and the old whylog remains the only working durable.

That makes three exit clauses unreachable as written: a plan route writing/reading/explaining
a receipt from the *real* pipeline; both product routes ending in `dorc why`; and the old
format, writer, and reader being deleted. Deleting the old writer while the replacement
cannot run in the binary leaves the product with no durable and `dorc why --last` with
nothing to read.

Three ways out, all the human's: relax the exit so the old whylog survives until a provider
lands (the arc closes architecturally complete, product-incomplete); accept a binary that
writes no durable and delete the old one anyway (spike-honest, product-regressive); or lift
the no-provider ruling. Nothing here is conductor-rulable and no lane may work around it.

The four departures an outside-lineage spec-compliance pass graded as needing a human ruling
have all been RULED and BUILT — see *rulings that bind* above. Nothing from that pass is
outstanding. `30Rc1` holds the raw adjudication; it is foreign output, unadjudicated, and that
model over-flags severity.

One residue: **the acquired-source restructuring stands on an explicitly temporary, unwelded
ack** and wants grounding from outside this arc before it welds. It is the sole receipt-arc
entry in root `TODO-ADDTL.md`. Also with the human: the Windows relink lock above, and the
source vector's positional fallback.

## ARC-CLOSE OBLIGATIONS — nothing ships with these outstanding

0. **Gate-8 asserts far less than it reads as asserting** (measured 2026-08-25, acted on by
   nobody). Across all six protected cases the only live needle is `=== OUTCOME ===`; every
   other line of `survivebite27`'s `expect-why-chain` — the block that reads as pinning the
   tier-worded links, the two loci, and the naked-trust epilogue — begins with `#` and is
   filtered before comparison. **Verify this independently before letting it lower the price
   of losing that arm**; it is a finding that happens to reduce the cost of a thing that
   blocks work, which is the shape that most deserves a second measurement.

1. **Gate-8 restoration.** Disabling the whylog writer empties the replay arm of **six** loom
   cases (`survivebite27-naked-trust-chain` and five `whygallery-*`) through
   `cli/tests/e2e.rs` gate-8, which drives the binary three times to compare a live render
   against a replayed one. Moving that arm onto the receipt reader is Stage 5 work.
   **Currently NO restoration debt is outstanding — nothing has been disabled, removed, or left
   red.** Keep that true or know precisely when it stops being. Note: **neither "disable"
   instrument reaches these cases** — the loom frontmatter vocabulary is closed with no xfail
   key and an unrecognised key is a refusal, and the xfail mechanism is panic-based while e2e
   gates report by pushing a string and returning normally, so wrapping a gate arm would read
   passing unconditionally and fire its own XPASS. Only leave-red or remove.
2. **Steering prose.** Every invariant above is owed a synthesis pass into the crate and spike
   steering files, by an authorized author, reviewed by the human. Nothing was written to the
   tree. Carries one edit outside this arc: `.claude/skills/conductor/fable.md`'s single
   ~800k subagent-token figure wants splitting into two, human-typed 2026-08-25 —
   **~750k, stop taking new ground** (finish the item, land its tests, report, commit; a
   planning boundary with comfortable room, explicitly not drop-plates) and **~850k, write
   down what matters** (the ledger, and any in-flight lie such as a disabled or
   deliberately-red test that would be catastrophic to forget — NOT "commit bad state";
   worktrees are durable and code survives, knowledge is what does not).
3. **Branch and worktree cleanup**, blocked on the containment problem described at the top.
4. **`plans/30R` reconciliation** against `30Rb` as amended, in public vocabulary.
5. **`gate:arc`** from the populated branch before folding into `ai/main`.
