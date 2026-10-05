# 314a — Naming the identity specification's world side: the sittings ledger

> AI-authored (Opus 5.5, 2026-10-05, the human present). Notes-tier chronological ledger of the
> sittings that name, explain, and ack the world side of `specs/311-identity.assay.md` before any
> sitting chooses a reading of it (`314:ord-name-a-relation-before-any-choice-that-reads-it`).
> Nothing is ruled unless marked **[TYPED]** (the human typed it; the wording here is the
> conductor's) or cited as `docID:slug`. **[CONDUCTOR]** marks a claim the human has not acked.
> Grades: +SURE / ~SUSPECT / -GUESS / --WONDER. Sections are written at discoveries and at the
> human's acks, not per turn. Successors append sections; they do not rewrite an earlier one.
> **[TYPED]** standing law: this ledger holds only what the human has seen and responded to, and
> never the conductor's reasoning from a turn the human has not yet answered.

## § 1-what-the-world-side-is

2026-10-05. The first item: the thing the seven world relations live in, which the specification
calls the world stratum, and the truth predicates that read it. Put in two rounds; the human's
responses follow each.

### § 1.1-the-first-round-as-put

- `ground-truth-is-the-world-side` **[CONDUCTOR]** — the world side is the specification's
  stand-in for reality: what is actually so in each world the checker considers. Nobody states
  it, and the engine's definitions never read it. Anchor: alias analysis, whose soundness is
  stated against the actual heap that the analysis never sees (SAME, DISJOINT, UNKNOWN as must-,
  no-, and may-alias).
- `truth-condition-is-contract-text` **[CONDUCTOR]** — a statement type's truth condition is the
  fine print of what its author asserts. The law assumes the truth condition of every statement
  in force, so a wrong answer is attributed to the statement whose truth condition fails.
  Consequences put: a truth condition stronger than the author's assertion hides countermodels
  and misattributes; 9 of the 28 are empty, which is right for entries and wrong where the engine
  relies on the statement (the cell and composite holes); attribution is not fault. Worked
  example: `:aliases-nothing-else` over an LDAP directory and a host's NSS view of it, where the
  `owns` reading puts the burden on the describer of the store that re-presents, who can know.
- Names put: "world stratum" → ground truth; "truth predicate" → truth condition; with
  alternatives, and with "world" already meaning three things in the specification (mWorld, the
  `World` signature, a whole scenario the checker considers).
- **[TYPED]** neither name is ruled yet.
- **[TYPED]** soft, conversational, not durable: the stronger reading of `:aliases-nothing-else`
  puts the epistemic burden in the wrong place.
- **[TYPED]** `holds` and `owns`, and the difference between them, read as imprecise glosses for
  something that likely has a precise, standard name in the field; to be dug when those
  relations are reached.
- **[TYPED]** the human's rephrase of the world side: the last link behind each claim, hidden from
  the engine — whether, in the world under examination, the claim is actually true or false.
  Tooling for checking fail-safety. "What actually happened" is a dangerous, temporal phrasing.
- **[TYPED]** a third category exists: world truths paired with no claim and specific to no book.
  The division may matter for writing the Alloy and for naming.

### § 1.2-the-second-round-as-put

- **[CONDUCTOR]** world facts vary from world to world, and a book pins some of them; a truth
  condition is fixed per statement type and is what its author signs. The universal premises
  paired with no claim, +SURE by grep: `owns in holds` (a `fact`); no store among its own
  contents and no route through itself (named premises of the laws); the engine's
  shell-resolution axiom (it reads `reaches` and `passes`); and `reaches: lone` (a multiplicity).
- **[CONDUCTOR]** the horizon is not inside ground truth; it selects which worlds the promise
  covers. "Take as ground truth" is an idiom for "assume", a cost to the name.
- `premise-kinds-by-who-answers` **[CONDUCTOR]** — every universal premise is true by definition,
  an engine axiom, or a horizon; truth conditions are the per-claim contract beside them; a hole
  is an item not yet assigned (`312f` § 12's three dispositions). ~SUSPECT nothing empirical about
  ops is universal, so "literally true" reduces to definitional plus engine. A horizon is
  described by its cause and by the user habit that keeps a user inside it, never by frequency.
- **[CONDUCTOR]** truth conditions ship as oracle-author documentation, horizons as admin
  documentation.
- **[TYPED]** RULING `rul-host-state-changes-only-at-book-actions`, for the encoding, for sanity
  and for checker performance: host state does not change outside book actions. The TOCTOU
  exclusion dictates the checker's state space, and is expected to be encoded as structure
  (timeless world facts for host state and for the truth behind claims) rather than as
  time-varying state. Time may be absent, and is not required to be: some ground truth is still
  about change (a line that was claimed to modify something did or did not).
- **[TYPED]** referential agnosticism is not an impregnable law for the specification: the engine
  holds some narrow ground truths by design (shell-loading semantics; perhaps basic host and
  network facts for orchestrator features).
- **[TYPED]** an attention tune, no new ruling: "a key reaches at most one mReferent" is
  definitional. An mReferent is defined as what an mKey can point to; round-robin DNS and other
  notions of identity, object, and target were weighed and distilled into it in the r31 ledgers.
- **[TYPED]** conversational ack: a clear, mandated distinction between ground truth and a
  horizoned truth. The human's description, put as a question: ground truth varies over no
  inputs; a horizoned truth varies over an input whose variance is discarded for a fixed output.
- **[TYPED]** ack: no frequency for horizons; add sibling coverage as a property of one. The table
  of horizons, their rationale, and their user documentation are not content of this
  specification and are set aside. What matters here is how "horizon" is defined and used in
  specification-class documents, and how the Alloy form of a horizon is set apart from other
  Alloy, if at all.
- **[TYPED]** NACK of horizons as admin documentation only. The horizon is equally oracle-author
  documentation: Dorc and the oracles a user installs share one horizon toward that user. It
  tells oracle engineers where not to spend effort and, more importantly, where they must (many
  extremely hard things in ops are not horizoned, and a battle-class oracle handles them all).

### § 1.3-the-third-round-as-put

- `time-is-book-line-time` **[CONDUCTOR]** — if time enters the model, its only clock is the
  book's order, and host state is a function of position in the book (the smallest slice of
  `312f` § 13.2).
- **[TYPED]** DURABLE ACK of `time-is-book-line-time` for this specification, corrected: time is
  logical time over externs, not over book lines, and it fully encodes sh control flow (loops,
  function calls, loads). The human's recollection is that this is what the corpus calls a site.
  A word that normative text cannot carry without a new term goes to the owed-terms list; nothing
  is minted. It may prove under-resolved; it is where to start.
- **[TYPED]** wallclock may not be wholly outside the model: wait loops, `sleep`, and shell
  concurrency (`&`), a first-class need for any meta-orchestration (create a machine through a
  provider's API, then loop in sh until it is up). The ruling restated as a narrowing of the
  TOCTOU weld: the model checks no externally initiated sub-step between one Dorc-visible action
  or claim and the next.
- **[TYPED]** the horizon discussion is tabled while time takes a turn of its own.
- **[TYPED]** directive, applied: § 1.4 of the specification states in its normative translation
  that, by definition, an mKey reaches at most one mReferent, with a commentary pointer to
  `311t:fnd-a-referent-is-an-aggregate-and-a-may-set`; the model is unchanged (commit "(AI dsn)
  State that a key reaching one referent is definitional"). The assay check of that commit at
  the gate tier: lock matches, exit 0.

### § 1.4-the-fourth-round-as-put

- **[CONDUCTOR]** the reading put for ack: (1) the clock counts runs of mSites, not mSites, and
  ticks at every simple command, builtins included; the shared half's "no line is above itself"
  fits only loop-free books, and `World.lineWrites` is indexed per line. (2) Deferred effects are
  in, as concurrency the book started (happens-before; a background job and its `wait` are the
  same shape); they land at any later run until the book observes them or ends, and one still in
  flight when the book ends is horizon for the next run. ~SUSPECT landing time matters to live
  checks inside the window and to names the effect re-points, not to the elision tier's walls.
  (3) `sleep` orders nothing, so wallclock is fully outside the model; this departs from `312b`
  § 3's banked "the admin's `until` and `sleep` closes it". Owed: a noun for one run of an mSite.
- **[TYPED]** loops and the other control-flow constructs must be handled by the specification;
  they cannot be punted.
- **[TYPED]** "extern" was a conversational gloss and is retracted. Meant: every command, shell
  builtins included, except one ruled (as ground truth) to have no side effects and so no step
  in logical time. Whether a command is implemented inside or outside the shell does not decide.
- **[TYPED]** gentle ack, not deeply investigated: the interesting cases are nearly always two
  worst cases for different consumers: an effect lands very late (last in logical time) or very
  early (first, immediately after dispatch).
- **[TYPED]** on `sleep`, not ruled: the strict reading is the near-certain ruling.
- **[TYPED]** concurrency was punted several times on purpose and is a round's worth of work; this
  sitting firms and reduces owed work and must not open it. Asked: can the local task go forward
  without locking in any concurrency decision at product or language-design tier? If not,
  concurrency is ruled out of this version of the specification entirely, at the cost of a new
  specification later. A question of Alloy capability, specification language, and ordering, not
  of product behaviour. The punt boundary may move to give a clean, stable specification;
  pushback is allowed if specifying 311 fully with concurrency is easier than expected.

### § 1.5-the-punt-boundary-as-put

- `concurrency-punt-boundary` **[CONDUCTOR]** — every concurrency design only adds order, so a
  soundness law that holds with nothing ordered beyond program order holds under every later
  design. In: time as program order over runs of mSites, as sh control flow sets it (loops,
  function calls, loads), a command ruled to have no side effects taking no place in it; every
  effect attributed to the run that started it, with no field for when it lands. Floor: a
  background job, pipeline stages, and a tool's deferred effects are ordered by nothing beyond
  program order. Out, nothing ruled: `wait`, observation as a join, `sleep`, describers'
  declarations of synchronous or asynchronous effects, and every consumer that reads state
  between dispatch and landing (today the guard tier's live checks and the apply standup's
  `witness()`, both outside the fences). Coupling put: the boundary holds while wrongness is
  judged by interference, not by end-state equality. Owed terms: a noun for one run of an
  mSite; the order relation over sites (today's textual `above`).
- **[TYPED]** that the specification does not cover landing time or ordering is normative truth
  and belongs in the specification (where and how not yet chosen). The "out" list is
  non-normative and conversational, kept so nobody is confused.
- **[TYPED]** Dorc concerns itself with the presence of runs, not with end states. Epistemics,
  not a ruling: Dorc cannot know end states, and nothing gives oracle authors or admins a
  vocabulary for what a state is; Dorc knows booleans about whether a run is wanted, and
  `__predict()` strings that no contract maps onto real state. Lean:
  `314:ord-the-wrongness-criterion-before-any-licence-result` is out by construction, not by
  ruling; open to being convinced otherwise.
- **[TYPED]** the punt boundary is accepted as understood; in accordance.

### § 1.6-a-noun-for-one-run-of-an-mSite

- **[CONDUCTOR]** menu put: call (paired with call site; the conductor's pick, with "an execution
  of the book" for the whole), invocation, dynamic instance, event, step; rejected outright: run,
  execution, occurrence, activation, iteration, span, task instance.
- **[TYPED]** call and invocation look risky: both assume that a line with effects involves a
  context switch (a function, a builtin, a shell function), which fails for variable
  definitions, and a line of pure variable definitions can have surprisingly complex effects.
  Asked: is "a call" 1:1 with a run of a site, guaranteed by POSIX? The clearest words are
  skipped only if they cause this precise mis-assumption.
- **[TYPED]** several words are acceptable at specification tier; the cost is a gloss evolving in
  other documents and conversation so that grep no longer finds every use, handled by declaring
  the gloss up front and banning it from the specification.
- **[TYPED]** NACK event (too overloaded); strong suspected nack of step. The name should have as
  much reach as "site" and will appear in other documents with a precise meaning.
- **[CONDUCTOR]** answer put, +SURE of the substance: no. POSIX (XCU 2.9.1) lets a simple command
  name no command; its assignments then change the current shell, its redirections are still
  performed, and the standard still calls it executed. Cases with effects and no callee: a
  `PATH` assignment (a routing change), a lone redirection that truncates a file, a command
  substitution inside an assignment, a function definition that re-routes a name, a redirection
  on a compound command. Call and invocation assume a callee, so both are out. Revised menu:
  execution (the pick: POSIX's word, under the rule that it always means one execution of an
  mSite, the whole book's run taking another word later), dynamic instance, site execution,
  occurrence; also rejected: evaluation, transition, action.
- **[TYPED]** DURABLE ACK `mExecution`; minted. Applied: § 1.11.2 of the specification defines it
  in a normative block, and § 1.10's translation and § 1.10.1's normative block no longer use the
  bare word (commit "(AI dsn new) Mint mExecution and drop the bare word elsewhere").

## § 2-what-reaches-means

### § 2.1-the-first-rounds-as-put

- `reach-is-per-execution` **[CONDUCTOR]** — an mKey is minted at a bind, so it is static, while
  loops, re-pointed names (a `current` symlink switched by `ln -sfn`), and round-robin lookups give
  one mKey different mReferents at different times. Proposed: at the probe and at each
  mExecution, an mKey reaches at most one mReferent; across its mExecutions, a may-set. § 1.4's
  committed "an mKey cannot reach two mReferents" is false once loops are encoded unless it means
  "at one mExecution". A summary reading (the referent is the pool) would make SAME unsound for
  round-robin names.
- **[CONDUCTOR]** explained on request: `reaches` is the ground-truth endpoint of a lookup, which
  the engine never holds; the word is booked three ways in the corpus (`30U`'s entailment,
  `28Q`'s effective reach, the specification's denotation) and means control-flow reachability
  to a compiler engineer. Beside the endpoint sit three chains with their own relations: the
  route (`passes`, the mTraversal), the re-keying (`:yields`), and the scoping (`holds`,
  mParent). Proposed: `reaches` names only the endpoint of one lookup at one instant, an instant
  being the probe or one mExecution.
- **[TYPED]** asked: what of a chain that reaches no mReferent but ends at one mKey that is itself
  unresolvable, a symbol that exists only inside the engine to decide that two resolutions end
  together? Expected to be very common, since the shell world will often be described only up
  to a local name.
- **[TYPED]** ack: some component here is falsifiable in an important way, which is why it needs
  a name and a distinction, so that false claims about it can be written down.
- **[TYPED]** asked whether the may-set is a lattice.
- **[TYPED]** ack: the grain is the mExecution, by design and dictum (the TOCTOU exclusion).
- **[TYPED]** the question as put cannot be answered: whether a site handles one settled value or a
  growing set is what decides whether a name is singular (an mResolution) or plural (a set of
  them), not the reverse. Two entangled questions instead: for each use of "reaches" in the
  specification, whether it was meant to be many-valued at that site (possibly with no single
  answer, if logical time was not in view when it was written); and, separately, minting fixed
  terms of fixed arity, each site then saying one or a set of them. Naming, plurality, and
  variation over time form a matrix. Not a terminology ruling.

### § 2.2-the-census-as-put

- **[CONDUCTOR]** an Opus census of every "reach" in the specification at `63e08859` (ephemeral
  table, root `_tmp-314a-reach-census.md`): 383 uses; 310 are the ground-truth landing of a key,
  73 are five other senses (engine-side landing, control flow, route reachability, effect
  spread, "a false answer is reachable" by the checker), one of them in normative text
  (§ 1.7.1, "ordinary effective-mWorld reach"). Of the 310, no site means several mReferents at
  one time; three sentences say the landing changes over time (§ 0, § 1.7.1, § 3.3.1), and the
  invalidation rules rest on them; every fence reads one landing per key for the whole book (no
  `var` field), which § 3.3.2's commentary states outright for its reboot book.
- **[TYPED]** directive, for when the renaming comes: the normative collision is fixed to a
  different, locally correct word in a commit separate from the one that gives the (singular)
  meaning of "reach" its acked name.
