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
crates: a pure `dorc-receipt` and a `dorc-receipt-crypto` holding only the Age/Ed25519
implementations of its capability traits. Stages 0–2 are complete; Stage 3 is substantively
complete; Stage 4 is partial; Stages 5–6 are unstarted.

## where the work is

| | |
|---|---|
| Build branch | `ai/r30-receipt` @ `5b73d56b`, worktree `.claude/worktrees/r30-receipt` |
| Conductor branch | `ai/r30-conduct`, worktree `.claude/worktrees/r30-conduct` (this file) |
| Base | `ai/main` @ `388e0249` — rebased onto it 2026-08-25, conflict-free. Re-check before folding; the sibling is still moving. |

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
| 4 intent / dispatch / outcome | **partial** — authority chain built, projection owed |
| 5 why / correlation / re-derivation | not started |
| 6 rip the old implementation | not started |

## what is owed

**Stage 4 exit:** multi-file topology carriage is DONE. Still owed: the intent/outcome
projection and publication; the DST apply route; and gating `ship_consented_apply`, which
today verifies nothing about the bytes it ships and is BLOCKED — see *open with the human*.

Three residues from the carriage lane: a diagnostic for a refused planned image (absence is
truthful but silent; the code is mintable, the defining case reads as diagnostics territory);
the binary's own `planned_image` wiring is unobservable from any test target, so the
projection half is covered at its own seat instead (same disclosure class as
`receipt_route.rs`'s existing cut); and `WhyWorld::final_presentation` still takes
`planned_image` and its one caller passes `None`, so a why report would name no image where
the run does — Stage 5's lane.

**Stage 3 residue, deliberate:** the old whylog writer still stands, and seven overlay slots
read `uncollected`. Both are explained under *rulings*; neither is an oversight.

**Arc close, hard:** see the final section. Nothing ships with it outstanding.

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
- **The apply route stays behind a configured bypass**, not a genuine standup. The transport
  layer refuses reusable multiplexing deliberately, with an attribution argument written at the
  seat: a shared master is a socket at a path the user's config chose, so an attempt could
  inherit a channel it never opened and the host it is attributed to stops being the
  controller's own fact. Building the standup means re-opening a governed surface. And the
  binary cannot sign anyway.
- **The crypto crate is a DEV-dependency of `cli`**, so the binary is structurally unable to
  sign rather than conditionally declining. It flips the day a key provider lands.
  Consequence: there is no second active writer, so the old whylog writer stays — Stage 3's
  literal exit clause is unmet by design, not by neglect.
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

**Windows relink lock, unresolved and the human's.** `cargo test --no-run` intermittently
cannot remove `spike/target/debug/dorc.exe` (`Access is denied`). Established: not the build —
it survives deleting the binary, and only that step trips it. Suspected but NOT proven: a sync
client, since Windows `CARGO_TARGET_DIR` resolves inside the synced tree at `mise.toml:118`
while the WSL arm points outside it and has never hit it. Candidate fixes: exclude `**/target`
from syncing, or extend that Windows arm to a per-worktree cache outside the tree. A
discriminator exists — point the target dir outside the synced tree for one run — and has not
been run.

## open with the human

**`open-permit-needs-a-standup-the-route-lacks` — BLOCKING Stage 4 item 4.** Both arms of
`IntentPublicationGate::permit` consume a `PreparedApplyIntent`, so even the configured
bypass needs an `ApplySessionReady` and therefore a `ResolvedApplyContext` with six
defaultless fields. `ship_consented_apply` can honestly answer one, and even that is the
invocation's spelling rather than a resolution. Both of `30Rb`'s sanctioned outcomes are
closed: supplying the rest is the named "fake target/session identity" stop condition, and
a genuine standup was ruled out above. Three candidates, none conductor-rulable:
`option-refuse-remote-apply-entirely` (honest, loud; breaks `spike/e2e/livetest.sh:253`,
which is never a gate — so silent to CI, loud to the human);
`option-sessionless-bypass-mint` (a governed change to a reviewed type, and it inverts
`sinv-controller-attribution`'s absence-means-no-authority); `option-bind-bytes-without-permit`
(image built and validated pre-dispatch, no affine boundary — less than
`30Rb:apply-dispatch-sequence` asks, and it leaves a mutative path beside an unused permit
chain, which is the shape that rots).

The four departures an outside-lineage spec-compliance pass graded as needing a human ruling
have all been RULED and BUILT — see *rulings that bind* above. Nothing from that pass is
outstanding. `30Rc1` holds the raw adjudication; it is foreign output, unadjudicated, and that
model over-flags severity.

One residue: **the acquired-source restructuring stands on an explicitly temporary, unwelded
ack** and wants grounding from outside this arc before it welds. It is the sole receipt-arc
entry in root `TODO-ADDTL.md`. Also with the human: the Windows relink lock above, and the
source vector's positional fallback.

## ARC-CLOSE OBLIGATIONS — nothing ships with these outstanding

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
