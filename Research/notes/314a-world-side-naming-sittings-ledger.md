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

- **[CONDUCTOR]** an Opus census of every "reach" in the specification at `63e08859`: 383 uses;
  310 are the ground-truth landing of a key,
  73 are five other senses (engine-side landing, control flow, route reachability, effect
  spread, "a false answer is reachable" by the checker), one of them in normative text
  (§ 1.7.1, "ordinary effective-mWorld reach"). Of the 310, no site means several mReferents at
  one time; three sentences say the landing changes over time (§ 0, § 1.7.1, § 3.3.1), and the
  invalidation rules rest on them; every fence reads one landing per key for the whole book (no
  `var` field), which § 3.3.2's commentary states outright for its reboot book.
- **[TYPED]** directive, for when the renaming comes: the normative collision is fixed to a
  different, locally correct word in a commit separate from the one that gives the (singular)
  meaning of "reach" its acked name.

### § 2.3-the-name-of-the-relation

- **[CONDUCTOR]** put: four combinations of one-or-a-set against one-instant-or-across-instants;
  the base unit is one mKey at one instant reaching at most one mReferent (the engine's record of
  it is the existing mResolution, singular); a set at one instant is never wanted; one value
  across instants is the case where the resolution stands; a set across instants is the derived
  plural, the existing normative word may-set. mResolution's "at program point p" needs retiming.
- **[TYPED]** asked to confirm: the minted word is world-side (the category of mReferent, never
  held by the engine), not a name for the last mKey in a chain. **[CONDUCTOR]** confirmed; the
  last mKey is the engine-side mKey-Primary that `identity()` finds; mResolution reads as the
  engine's record of a landing.
- **[TYPED]** read, not a ruling: this is a lattice over resolutions; a statement has one
  resolution across the whole analysis, or a lifted lattice of several possible resolutions that
  join and meet at points. **[CONDUCTOR]** agreed with one placement: the lattice is the
  analysis's, over engine-side resolutions; ground truth at one instant is one point. A menu for
  the world-side verb was put (refers to, designates, denotes, points to, resolves to), with a
  lean to "refers to" and "resolution" kept for the engine side.
- **[TYPED]** lean: use the family of the acked mReferent, written with the model tag: "mRefers
  to", "its several mReferences", so that the relation is the one with an mReferent at its other
  end. Possible only if the relation is one-to-one with mReferents at its other end and if
  mReferents mean nothing in the model when no mKey refers to them.
  **[CONDUCTOR]** the first condition holds (the field's type; "or none" is "mRefers to no
  mReferent"); the second fails by design, since the checker considers pieces nobody keyed
  (`notes/312d` § 21, cited at § 1.1.1) and they carry the danger (a loop image nobody
  described); so mReferent keeps its own definition (§ 1.1), "mRefers to" is a relation into it,
  and an mReference is one instance (one mKey, one instant, one mReferent). Spelling: `312b`
  § 4 writes relations as `:fixed-term`.
- **[TYPED]** ACK of that family, with the correction.
- **[TYPED]** lean, open to pushback, not a ruling: the earlier description of the typographic
  conventions was imprecise. `:this` is reserved for claims, as a deliberate reference to the
  eventual syntax of claims; a relation that is not a claim, and will never have a spelling,
  keeps the mThisSyntax, the marker for a precisely defined term (a modelled relation is a
  modelled object in the higher-order sense). Taking it needs a review of existing `:` uses, so
  that none names an abstract relationship that can never be spelled.
- **[TYPED]** the usage sites are many: apply with great care; the adversarial reviewer may be
  woken afterwards. Offered, the choice left to the conductor: ledger without applying, and take
  the reviewer's earlier findings first, so that new edits do not rest on shaky ground.
- **[CONDUCTOR]** chosen order: the reviewer's findings, then the review of existing `:` uses,
  then the rename. Asked to confirm the base arity explicitly.
- **[TYPED]** ACK: an mKey reaches one mReferent; only one thing can be mReferred to at once; it
  is singular. Where reassignment or set-valuedness enters, new precise wording is appropriate,
  whether a new mTerm or careful phrasing (undecided).
- **[TYPED]** unconcerned with the lock: the fixes are mostly naming and normative text; Alloy
  confirms in CI in the background while the sitting's time goes to explanation, thinking, and
  ledgering.

### § 2.4-the-first-review-walked

A Fable reviewer in clean context attacked the two normative commits. The conductor put each
candidate change alone.

- **[TYPED]** one commit per semantic change, or at least per ack; no batches of reviewer nits.
  Adversarial review has produced churn in r31 and is not trusted: any change to normative text
  the conductor is not absolutely certain of is dropped.
- **[TYPED]** ack: the STE "may" fix; a pointer fix to the right heading of an external document.
- **[TYPED]** the POSIX citation stays broad (2.9 is fine; not 2.9.1.3).
- **[TYPED]** on deleting § 1.4's second added translation line: suspicious. The two lines bind
  opposite directions: "cannot reach two mReferents" excludes a second mReferent, and "all that
  one mKey reaches is one mReferent" excludes anything that is not an mReferent (no apples).
  The second half may go if the pair still says one-to-one, not one-to-one-plus. Applied: the
  line now reads "Everything that an mKey reaches is an mReferent", which also discharged the
  "may" fix (commit "(AI dsn) Translate the reach field's range plainly, without parts").
- **[TYPED]** ack, applied: § 1.4's commentary drops the aggregate reason and cites § 1.2 and
  § 2.3 here (commit "(AI dsn fix) Drop the aggregate reason and cite the typed definitional
  line").
- **[TYPED]** ack, applied: § 1.11.2's commentary credits POSIX with the verb, not the noun
  (commit "(AI dsn fix) Credit POSIX with the verb, not the noun").
- **[TYPED]** ACK of the wording "When the book runs again, its mSites have new mExecutions":
  the same mExecution an hour later on a re-run is nonsense; low value, kept, since a
  specification excludes the nonsensical too.
- **[TYPED]** "run" stays, to avoid churn, except that STE allows one meaning per word: a site
  that reads significantly differently is reworded. Applied: the plain reading (a command, a
  function, a body, the shell, or the book executing) keeps "run"; four normative lines in
  other senses take the specification's own words: two kills' scopes ("asks over worlds of"),
  a hole's witness, and the flag set "for one invocation of Dorc" (commit "(AI dsn) Keep run to
  one reading in normative text").
- **[TYPED]** the description of mSite against its executions, and an mExecution not being the
  tick of logical time, are held work, with no spec change yet.
- Not acted on, by the conductor's walk: rewording the other "run" sites; § 1.11's description;
  the per-instant normative sentences, deferred to the rename commit.

### § 2.5-the-colon-review-and-the-rename

- **[CONDUCTOR]** put: every `:`-term in the specification names a claim except `:parent` (the
  derived parent edge, already named mParent) and `:observer-dependence` (the default, which has
  no spelling); § 1.7.1's "effective-mWorld reach" is a mis-tag from the 2026-09-10 tagging pass
  over `30K`'s "effective world reach"; after the rename, the other uses of "reach" in normative
  text are reachability in the standard sense; the Alloy field needs a name.
- **[TYPED]** ack: `:parent` becomes mParent, written grammatically (its mParent, mParentage).
  Applied (commit "(AI dsn) Write the parent edge as mParent, not as a claim").
- **[TYPED]** `:observer-dependence` stays. The colon rule, restated: `:` marks a relation that is
  known only from what a user says, a default included (the user conveys it by taking no
  action, and a default is itself a spelling, open to change in the spelling work); a relation
  the engine never holds takes an mWord; a relation the engine derives takes neither (stated
  with uncertainty).
- **[TYPED]** ack: the Conventions line is rewritten to say so, short. Applied (commit "(AI dsn)
  Reserve the colon for relations only speech makes known").
- **[TYPED]** the mis-tag is undone first, in its own commit. Applied (commit "(AI dsn fix) Undo
  the tagging pass's mWorld in effective-world reach"); no other mWorld mis-conversion of that
  pass survives in the specification.
- **[TYPED]** ack: the § 1.7.1 replacement, which belongs in the next adversarial batch. Applied
  (commit "(AI dsn) Say where a mutator invalidates without borrowing effective reach").
- **[TYPED]** mRefers to names one specific thing; reachability in its algorithmic, compiler sense
  stays "reachability", an ordinary word that grated only as the name of an LLM-minted relation.
- **[TYPED]** ack: § 3.3's "reaches every mKey in its writeset … kill-reach" is reworded. Applied
  (commit "(AI dsn) Say a state mutation touches its writeset, not reaches it").
- **[TYPED]** ack: the m is carried into Alloy, in two commits: first the plain rename, then every
  Alloy spelling that should have followed the rule. Applied: the rename (60 prose lines, 199
  fence identifiers as `refersTo`, a § 6.1 row; commit "(AI dsn) Rename the key-to-referent
  relation to mRefers to"), then the tag (`mRefersTo`, `mParent`, `mValue`, `mToken`,
  `mTraversal`, `mFullyQualifiedKey`, `mParentStore`, `mParentCatalog`, and the Conventions'
  fence bullet; commit "(AI dsn) Carry the m into every Alloy spelling of a model term"). Alloy's
  parse lint is clean after each.

### § 2.6-the-renames-judgment-calls

- **[CONDUCTOR]** put after the rename: the fields `scheme`, `shape`, and `route` stay bare,
  since tagging them would clash with their signatures; `topic` stays, since the field holds the
  mKey alone while an mTopic adds the observer; the world relations stay untagged until named;
  two hole slugs keep "reach" in the landing sense, since renaming them touches historical
  ledgers. Asked whether the per-instant sentences go in.
- **[TYPED]** slugs are no reason to hold back: they exist so that one global grep-and-replace
  commit swaps them, historical documents included. Eyeball the results; keep a global replace
  in its own commit; check grammatical variants and other spellings; check git history for an
  earlier rename of the same slug.
- **[TYPED]** ack: the world relations that are not mWords stay as they are until their turn.
- **[TYPED]** ack: "At the probe, and at each mExecution, an mKey mRefers to no more than one
  mReferent."
- **[TYPED]** asked for the field-name choice explained and compared. Lean: no site in the
  document uses both `bleh` and `mBleh` with an intended division between them; the one
  conceivable exception is an Alloy reserved word that is also the right long-term word.
  Otherwise the non-m use is reworded or renamed.
- **[TYPED]** the reviewer was meant to wait for the next substantial change; its second report
  is read with extra skepticism.

### § 2.7-the-second-review-walked

The reviewer attacked the seven commits of § 2.5. The conductor put seven items.

- Applied first: the per-instant sentence in § 1.1.1 (commit "(AI dsn new) State that a key
  mRefers to at most one referent per instant").
- **[TYPED]** ack, applied: "An mKey has at most one mParent", as the fence says (commit "(AI dsn
  fix) Say at most one mParent, as the fence does").
- **[TYPED]** ack, applied: "No statement species declares an mKey's mParent", so the sentence no
  longer denies the Conventions (commit "(AI dsn fix) Say no statement species declares the
  mParent").
- **[TYPED]** ack, applied: § 3.3's line points to the authoritative section, "The sparing test of
  2.6-may-write-the-writeset decides what a state mutation invalidates" (commit "(AI dsn fix)
  Point a state mutation's invalidation at the sparing test").
- **[TYPED]** HARD ACK: language stays unambiguous; a pronoun is dangerous in a specification.
  Repeating the noun within about three lines, or a parenthetical pointer to the line that
  introduces it, is always reasonable ("a foo" then "the foo that bars", not "it"); a strong
  preference, not a ban, added to the Conventions. Applied: § 1.7.1 names the mutator outright
  (commit "(AI dsn fix) Name the mutator outright instead of its and it"); the Conventions bullet
  (commits "(AI dsn) Prefer a repeated noun to a pronoun" and "(- AI dsn fix) Drop a never that
  contradicted not-a-ban").
- **[TYPED]** ack, applied: "at each site that the shell can execute after the mutator" in place
  of the undefined "use" (commit "(AI dsn fix) Say site, not the undefined use").
- **[TYPED]** the execution order against the fence's text order is held with the line-varying
  reach work, so the line is not missed; no spec change yet.
- **[TYPED]** soft ack: the colon rule says what Dorc is given, not an author's speech. The
  category is likely larger than users: defined by negative space at both ends (not built into
  the engine, not hidden from Dorc by design), it is what product design must worry about:
  spelling, UX, users' expectations, defaults, configurability. Applied, with attributes and
  warrants named (commit "(AI dsn fix) Widen the colon rule from an author's speech to what Dorc
  is given").
- **[TYPED]** the Conventions' derived-relation clause against the m-tagged derived names, and the
  leftover uses of "reach": held for the next turn.
- Applied under § 2.6's slug ruling: the two hole slugs that kept "reach" in the landing sense are
  renamed `…_refer_differently` in the specification, its lock, `312d`, and `312e` (commit "(AI
  dsn) Rename the two refer-differently hole slugs everywhere").
- **[TYPED]** these items close before the rewind; no new large item is taken on.

### § 2.8-names-inside-the-fences

- **[TYPED]** the narrow dictum about names is walked back, then restated: the same rules for
  normative Alloy as for normative prose, even where the Alloy reads awkwardly to a programmer.
  Where the spec's word is precise, it is used; where a meaning differs, a different word is
  chosen and used consistently, as STE requires. A rule easy to review beats a fine
  delineation, since a fine one invites model drift. The conductor's "name each field for its
  role" is a probable NACK, as it mints a second name for a chosen word.
- **[TYPED]** ack: a field spelled exactly like a signature (`k.mScheme`) is one spelling with two
  meanings (the set of all mSchemes, a key's mScheme). "The" reads wrong, since `a.b` is "a's b".
  Floated, not ruled: `k.own_mScheme` as the default form, with room to choose a more exact name
  per item. The two laws put: use the m-term exactly where that concept is meant; never write
  `item.mTerm` where it collides in Alloy with a bare `mTerm`.
- **[TYPED]** ack: helpers follow the rule, awkward or not.
- **[TYPED]** NACK that the rename pass is large: the bytes changed do not matter when no
  semantics can change. A commit of only one-to-one renames (no split, no merge, no other
  normative text) is safe. Any case that needs a decision is ruled and reworded first, so that
  the broad mechanical commit always carries the intent "no semantic change"; one found later
  is a bug, not a question of intent.
- **[TYPED]** authorized: an Opus subagent inventories the fence names and sorts the clear
  one-to-one renames from those that need care.

### § 2.9-capitalized-model-terms

- **[CONDUCTOR]** put: a field that would be spelled like a signature needs a disambiguator; `has_`
  and `own_` were floated (`own_` collides with the world relation `owns` and with the
  `…Owner` fields), then a trailing underscore (`k.mScheme_`). Then two options: (A) one narrow
  exception, a bare lowercase base word only as a field whose declared type is exactly that
  term's signature; (B) signatures capitalized, `MScheme`, the lowercase form kept for a
  property of an instance. The inventory came back: 89 clear renames, 73 names needing care in
  17 groups, 7 deferred; among its findings, the prose never writes mShape but writes bare
  "shape" 146 times.
- **[TYPED]** `has_` reads as a boolean predicate: probable NACK. `owns` will very likely be
  killed with `reaches` as an LLM-minted term; its current use is acknowledged.
- **[TYPED]** lean B: it is the common form in other specifications. The lowercase m-form was
  chosen to avoid implying a one-to-one between a code name and the specification concept; the
  sitting has arrived at wanting that one-to-one on purpose, which reverses the reason.
- **[CONDUCTOR]** pushback put on B's application: the global replace uses an explicit list of
  model terms, never a pattern (mDNS, mTLS); the change reverses `312b` § 4's typed convention
  and re-cases quoted words in historical ledgers; nouns capitalized everywhere, the lowercase
  form only as a property of an instance; timing.
- **[TYPED]** RULING, minted: `MTerm` is used globally, everywhere, for everything, except where a
  language feature forces the lowercase form (an Alloy field, Rust). In Alloy: `sig MKey` against
  `k.mKey`. Applied by one global, project-wide, single-commit capitalization change. Nothing of
  it is applied in this sitting.
- **[TYPED]** the sitting ends here.

### § 2.10-the-capitalization-pass-handed-to-a-builder

2026-10-06, a successor conductor (Opus 5.5) after a handoff.

- **[TYPED]** ack of the mechanical capitalization pass of § 2.9, handed to an Opus builder. Scope:
  project-global. Nothing is exempt except this ledger, which records the rename itself.
  Historical documents and ledgers are included.
- **[TYPED]** the single mechanical pass renames only terms with no concern, conflict, merge,
  split, or other subtlety. Every other case gets attention first and its own pair of commits: a
  fix that renames nothing, then that one term's mechanical rename. The builder may yield to the
  conductor and the human for rulings before it takes such a case.
- **[TYPED]** the list of terms that rename freely is the conductor's and the builder's judgment.
  Suggested method: a wide match that catches every model term and each grammatical form of it;
  then a separate find-and-replace over each grammatical form of each accepted term only (never
  `mDNS`). The human is asked only where a distinction is meaningful.
- Landed: `ai/main` fast-forwarded over the builder's branch (tip "(AI dsn re) Capitalize model
  terms in sh comment prose"): about 6,100 tokens re-cased, byte-checked as m-to-M only; the
  parse clean; the gate-tier check matched every row it solved. **[CONDUCTOR]**, reported in chat
  with no reaction: the builder's four held occurrences ruled as "rename uses, never mentions"
  (a recorded search string, a quoted old spelling, and `312b` § 4's convention paragraph keep
  the old spelling; prose in sh comments is a use); a separate commit re-cased one file under
  `Research/quarantine-DO-NOT-READ/`; the builder ran `mise trust` on its worktree.

## § 3-the-truth-predicates

### § 3.1-the-predicates-name

- **[CONDUCTOR]** put: each statement species has one predicate, `true_<Species>[d]`, which holds
  when statement d is true in the world under examination; the law's premise
  (`axiomaticByContractExcept`) conjoins them over the statements in force, one line per
  species; the kills use their negation. Proposed for what each states: "truth condition"
  (formal semantics), untagged.
- **[TYPED]** the spelling reads strangely: "predicate: true, declares-aliases-nothing-else".
- **[CONDUCTOR]** put: a subset signature of true statements, defined per species like the shared
  half's `InForce`, with a one-line premise. **[TYPED]** declined: no restructuring in a naming
  round; a defining fact is assumed in every check, at a performance cost (the class `313` § 1.2
  measured) and likely a flexibility cost; naming only, structure untouched. Withdrawn.
- **[TYPED]** floated `DeclaresPrimaryOf_isTrueFor[x]`, reading d as what a claim is about.
  **[CONDUCTOR]** d is the statement itself; what it is about is reached through its fields.
- Measured (an Opus subagent, Alloy 6.2.0, a scratch model, a kill beside every check): one
  overloaded name per species, declared `pred isTrue[d: S]` or `pred S.isTrue` and called
  `d.isTrue` or `isTrue[d]`, resolves by the declared type of the argument; a call on the parent
  type is a hard "ambiguous" error; with one declaration visible, a parent-typed call is accepted
  silently and applies that body to every atom; overloads resolve across opened modules.
- **[TYPED]** lean, moot after the measurement: if overloading failed, a per-species accessor name
  (`fDeclaresPrimaryOf.isTrue[d]`).
- **[TYPED]** ACK: `pred isTrue[d: <Species>]`, the plain form with bodies unchanged, called
  `d.isTrue`. Applied only after the subagent's survey of how larger Alloy projects spell this
  pattern returns.
- **[TYPED]** an assay scout starts now, scout-only and no fix: the subagent found that the
  project's Alloy runner discards Alloy's type-check warnings (its adapter parses with a silent
  reporter); whether assay also never shows them is unchecked.
- Open: the family's prose name.

### § 3.2-claim-warrant-and-no-family-name

- Measured (an Opus subagent's survey of 445 Alloy files; partial, as GitHub code search was down
  for part of it): no project overloads one predicate name across sibling signatures; the common
  pattern puts truth on the parent as one relation or subset, each kind supplying its defining
  clause; overloading by argument type is documented (the Alloy language reference, alloydocs)
  and endorsed by Daniel Jackson on the Alloy forum.
- Measured (an Opus scout, read-only): no path in the repo collects, prints, or fails on an Alloy
  type-check warning; the specification yields 1,014 today, every one "implicit in-line
  conjunction", none of the vacuity class; surfacing them moves every lock key, since the adapter's
  digest is part of each.
- **[CONDUCTOR]** put for the family: "truth condition"; then "guarantees" in the docs, with no
  family noun in the spec.
- **[TYPED]** stay with "claim" and "warrant": they read well, and keeping them saves churn. A
  claim is anything Dorc carries as user speech: everything outside engine knowledge (the sh
  floor and its like), horizoned truth, and universal logical or provable truth. The set is very
  large and not usually admin-facing, but may surface as a tag on anything the engine carries and
  cannot prove. It sounds soft on purpose: treating a popular oracle as authoritative about its
  tool is among Dorc's gravest dangers, and everything Dorc touches is a messy, unreliable claim.
  It covers lifted code that an author does not feel as speech. A warrant is the subset that does
  feel like speech to an oracle author: the individually hand-authored line annotations, not what
  the engine's analysis and lifting imply; possibly also the few warrants implied by writing an
  entry point at all, even an empty one; nothing else.
- **[TYPED]** no word for the `isTrue` family, and not "guarantees": non-MTerm, locally accurate
  prose at each site, retiring any global name; "the `isTrue` predicates" in non-normative prose
  where needed. Any normative use is raised to the human, who doubts it will justify a normative
  name.
- **[CONDUCTOR]** put: the one normative use (the translation of `axiomaticByContractExcept`,
  "satisfies its species' truth predicate") can become product prose that defers to each
  species' own section, but that is exact only once the eight species whose truth test is empty,
  and whose sections never say so, each get one sentence; two of the eight are held holes.
  **[TYPED]** ack of the identifier rename; the line's Alloy flavour bothers the human, who
  asked for a product-level translation without weakening anything. No ruling on the packet yet.

### § 3.3-alloy-warnings-surfaced

- **[TYPED]** the warnings fix authorized: narrow, plain engineering, the minimal fix that closes
  the hole; then every implicit-conjunction warning, and no other kind, fixed in one commit. Done
  on a builder branch: assay refuses a spec with any Alloy type-check warning, and the runner
  prints them.
- **[CONDUCTOR]** found mid-fix: 149 of the warnings exist only because assay's emitter puts each
  paragraph on one output line; an explicit `and` there needs parentheses wherever `implies`, `or`,
  or a quantifier is involved, and a missing pair changes meaning with no type error, while
  juxtaposed formulas are conjoined whole whatever their operators.
- **[TYPED]** an `and` at the start of each later line reads fine to the human; the silent
  precedence danger is near a dealbreaker.
- **[TYPED]** ACK `fix-the-emitter-layout`: the emitter keeps the source's line breaks; the
  `and` rewrites of multi-line paragraphs are dropped. Lean: drop any normalization in the emitter
  that changes meaning at all; key churn from some formatting changes is acceptable, arbitrary
  re-wrapping is unlikely, and CI re-solves the lock.

## § 4-the-pure-logic-document

- **[TYPED]** the premise's translation reads like philosophical epistemics, not a product claim.
  "Every oracle author's claim must hold" is a product fact, and a subset of what the line says;
  "for a statement to hold, each composed sub-statement must hold" is no Dorc product fact and
  does not belong in 311. Dorc-unique epistemics do belong there.
- **[CONDUCTOR]** put, and acked by the human's next line: 311's normative prose states what is
  true of Dorc, not what is true of logic; a third kind, a modelling choice that looks like logic
  (no store among its own contents, which a bind mount can break), is product and stays.
- **[TYPED]** begin cutting 311 down: a document in `specs/` that is normative but not product
  surface, holding the bare-bones content Alloy forces and no interesting product choice, written
  as genuine epistemics and never as a description of iteration. Hard ack on a very strict entry
  rubric: the document must not absorb statements that are, subtly, about the product.
- **[TYPED]** no scout yet. Order: the rubric alone first, with no Alloy; then the one identified
  item (the premise's aggregation over the statements in force) and a tight chain of its
  dependencies, judged by the rubric; then the assay work that makes it run; more items later. How
  specifications see the document (an include mechanism, or opening) is the conductor's choice.
- The rubric was written (only a normative block, no Alloy) and attacked by a Fable reviewer in
  clean context. Upheld by the
  conductor: as written the rubric admits nothing ("meets every sentence" with the no-clear-answer
  fallback); the four "another choice" tests read literally exclude every definition a check uses;
  the naming rule excludes all Alloy-forced plumbing, since Alloy's typing makes it name product
  objects; the premise is product, the contract's trust in speakers. Refuted: that the "which
  worlds" ruling does not exist (`312d`, typed).
- **[TYPED]** the human suspected sycophancy and asked for an honest estimate. **[CONDUCTOR]** put,
  by section ranges of 311: tests about a third (books, kills, twins, witnesses); working lists
  (§ 4.2, § 6) and the index (§ 5) about 325 lines; Alloy-forced encoding of product statements 100
  to 200 lines; generic logic about 15 lines, which Alloy's standard library already holds. Real
  specifications separate these: conventions clauses, standard toolkits, normative and informative
  annexes, conformance suites kept apart.
- **[TYPED]** the same split is followed. Any pure logic is a third category: imported from a
  standard library, or a tiny non-normative mechanical file. The target here is an annex
  (`mechanical-annex.assay.md` floated) for shared product definitions that are noise for the
  specification's reader. Carved out as the floor the rubric must exclude: "this holds when users'
  speech holds" is a critical product statement that looks like annex material.
- **[TYPED]** the test-document split is made now, by a builder, beside the subtler split of
  product value. Asked: after the tests and the standard library, which of the remainder can leave
  the specification body without hiding an important normative translation of the prose; the
  human is on the fence about whether a mechanical annex is worth having.
- **[CONDUCTOR]** put, on a worked example, 311 § 3.2's premise block (the per-kind conjunction,
  the contract and engine premises, the two world-shape premises) with the walk's memo tables
  beside it. Of the block's 47 lines, about 30 (the per-kind conjunction, and its named-set slot,
  whose only use is the kills) can leave the body without hiding meaning, but only once every
  kind's section says when its claim is true: 19 sections do, 8 do not. About 17 lines stay as
  product: the contract premise, the engine axiom, and the two world-shape premises, which a bind
  mount can break. The tables (11 lines) could leave, at the cost of a jump for the reader of the
  walk. Estimated residue that could leave after the test split: 70 to 100 lines of a body of
  about 2,900. Recommended in place of an annex: assay generates the per-kind conjunction from
  the declared kinds, complete by construction; the premise's translation becomes the product
  floor sentence, with the eight missing per-kind sentences; the question is judged again after
  the split.
- **[TYPED]** novel assay generation is fine where it is sound and obvious, and for some of these
  items it is strictly better than a separate file. Not dispatched.
- **[TYPED]** the test split is dispatched to a builder, the `isTrue` rename first. The test
  document takes an unused 311 letter, not a new document ID (`316` nacked): `311w`.
- **[TYPED]** durables never mention temporary files. This ledger's pointers to review reports,
  inventories, and the held-work inventory were removed, and their findings kept.
- **[TYPED]** the remainder is tabled and the sitting closes. The next open design-moving item is
  taken after a rewind. State at the close: `specs/pure-logic.assay.md` holds only the rubric,
  which the review found admits nothing; the premise's translation line in 311 § 3.2 still says
  "truth predicate"; the eight per-kind truth sentences are unwritten.
- **[TYPED]** ack, applied: the pure-logic document and its lock are removed (commit "(AI dsn rm)
  Remove the pure-logic document and its lock").
- **[TYPED]** ack, applied: the three leftover uses of "reach". § 0.1's normative line now reads "It
  knows MReferents only through MKeys, MTokens, and MDerivations" (commit "(AI dsn fix) Say the
  engine knows referents through keys, not reaches them"); § 1.4's commentary says "resolves
  into", and § 1.10's says where a probe stood (commit "(- AI dsn fix) Say resolves into and where
  a probe stood, not reach").
- **[TYPED]** directive, applied: the root `AGENTS.md` terminology bridge names the parent views
  `MParent-Catalog` and `MParent-Store`, narrow renaming only; § 6.1's note that the bridge still
  named the old views is dropped.
- **[TYPED]** the Windows hk race: encode it in hk properly, a build that runs before the parallel
  steps only when the tooling changed, and not overengineered; a brief investigation of the
  necessary complexity first, and no build until the human acks.
- **[TYPED]** one word per meaning across the whole document, by plain STE consistency and with no
  new term; move away from "reaches", a product-facing word with an old, specific meaning.
  **[CONDUCTOR]** "knows" is already the document's one verb for having information, used for the
  engine, for parties, and loosely for tools; § 0.1's line joins that sense.
- **[TYPED]** ACK of the mapping for "reach": "reach" and "reachable" keep only graph
  reachability (the checker reaching an answer, a code path reaching a statement, the walk
  reaching a level); effect spread on the engine's side becomes "collides with", or "the
  writeset"; containment becomes "beneath", as the fence's own `beneath`; the one-offs take plain
  verbs. Applied after the test split folds, one commit per sense.
