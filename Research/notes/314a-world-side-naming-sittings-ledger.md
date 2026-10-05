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
