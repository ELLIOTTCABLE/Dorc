# 30Z — specification praxis

> Tier: LLM-authored plan (Fable conductor, round 30, from the 2026-09-27/28 sittings and the two
> strawman runs). Ahistorical: rewritten in place as the praxis firms; history is `notes/30Ya` and
> git. Subordinate to the root docs. Siblings: `notes/30Y` (assay, the tooling: what it reads,
> builds, runs, and reports; nothing about how to write is there) and `notes/301` (minispec, the
> code-tier instrument whose posture this inherits). Grades: **[TYPED]** the human typed it ·
> **[ACKED]** confirmed in dialogue · **[CONDUCTOR]** conductor-derived, unratified. Generic to any
> specification checked by a bounded model finder toward our goals: Alloy's own words are used for
> Alloy's things, and the checker's own words are confined to 5-assay-vocabulary-and-forms. "An
> agent" below means a language model working the document; everything not so marked applies to
> any author. A section of this document is cited by its slug, as 2.4-inhabit-before-you-believe-green.

## § 0-what-this-is

- A specification here is a Markdown document whose normative content is Alloy the checker runs,
  with prose bound to that Alloy by rule (1-the-posture), maintained by one loop (2-the-loop) and a
  set of habits (3-writing-so-the-adversary-cannot-cheat), in one form
  (4-the-shape-of-a-specification). This document is how to write and maintain one. It is not a
  tutorial in Alloy: the language, its authoring loop at the model level, its vacuity catalogue and
  semantic traps are the `using-alloy` skill's, assumed here and not repeated. It names no shell
  command.
- Why it exists: the three review rounds over the identity model found two defect populations.
  Genuine model defects narrowed with every round. Text-drift under folds and rewrites, a unit
  stated precisely once and read wrong later, did not narrow, and scaled with edit volume. The
  adversary finds the first population; the firewall (1.2-the-firewall) and the recorded results
  (6-results-locks-and-what-an-author-owes) catch the second.
- Vocabulary, kept minimal: a *unit* is the atom of concretization (2.1-choose-a-unit). A *law* is
  a `check` over a free universe. A *scenario* is a pinned world, named atoms at exact bounds,
  checked one step at a time. A *hole* is a named, held-open design question
  (2.6-hold-a-question-open). Everything else is Alloy's word for it, or the checker's, in
  5-assay-vocabulary-and-forms.

## § 1-the-posture

### § 1.1-checked-never-confirmed

- **`pos-checked-never-confirmed`** [TYPED] — every statement a specification makes is a `check`:
  the solver is set against it, fills every gap the specification left with the world that breaks
  it, and hands that world back. A `run` is used for one thing only, to show a set of facts has a
  world, so that a check cannot pass by contradiction (2.4-inhabit-before-you-believe-green). The
  counterexample is the unit the author forgot to write.
- **`pos-bounded-is-never-proved`** — "no counterexample at scope six" is a fact about scope six.
  Every result is recorded with its scope; no report says proved; the posture is tracked, not
  proved.
- **`pos-mechanical-trumps-prose`** [TYPED lean, "by law"] — where the mechanical text and any prose
  disagree, the mechanical text is authoritative, and the disagreement is a design question to sit
  and burn down into more mechanical clarity: additive constraints go into the Alloy, the prose is
  retuned, and a wrong prose tune is walked back only by further additive narrowing. The checker's
  job is to force explicitness over time.
- **`pos-halo-is-the-hazard`** (inherited from `301`) — the failure this whole posture exists to
  prevent is a reader inferring "there is Alloy, so it is covered". Every mechanism below either
  earns confidence mechanically or renders its absence by name.

### § 1.2-the-firewall

The rule that keeps normativity from smuggling itself into commentary [TYPED 2026-09-28].

- **`fw-a-section-is-mechanical-iff-it-holds-a-fence`** — a section is mechanical when it contains
  any fence the checker reads; otherwise it is not. No other tag decides this.
- **`fw-normative-prose-is-a-headed-blockquote`** — normative prose, wherever it appears, is a `>`
  blockquote, one sentence per line, no hard wraps, headed by an HTML comment on the line above
  it. Running text outside a blockquote is never normative (1.3-what-plain-prose-is-for).
- **`fw-normative-prose-alone-only-outside-mechanics`** — a non-mechanical section either carries a
  block headed `<!-- normative -->` or is commentary. That block is the only place normative
  content may exist as prose alone; such a section is residue the checker cannot reach, and is
  counted as such (6-results-locks-and-what-an-author-owes).
- **`fw-translation-matches-exclusively-and-precisely`** — a mechanical section may carry a block
  headed `<!-- prose-translation -->`, optional at birth and strongly recommended [TYPED]. It says
  exactly what the fences say: every statement in a fence has its sentence, and no sentence says
  anything the fences do not. Where the two disagree the fence is authoritative, and the
  disagreement is a finding worked under 2.7-bank-the-answer. The translation is the reviewer's
  diff surface: a rewrite that drifts the prose is visible against a fence that did not move, and
  a rewrite that drifts the fence moves the checks.
- **`fw-every-fence-change-changes-the-translation`** [TYPED 2026-09-28] — every change to a
  mechanical section's fences includes a change to that section's translation block, and where
  the section has none the change adds one. This cannot prove the two match; it fights drift by
  making the author read the prose every time the law moves, and it makes the review diff show
  both readings side by side. A change to the translation alone is legal (the prose tune of
  1.1-checked-never-confirmed). A change to a fence with its translation untouched is committed
  only on an active, direct human ack for that commit. The rule is law and is not mechanically
  checked: no sound check of it exists that is not fragile [TYPED].
- **`fw-fences-are-purely-mechanical`** [TYPED] — no comment syntax inside a fence except the
  checker's own comment forms (5-assay-vocabulary-and-forms); no prosodic note, no rationale.
  Intent rides names: a long, explicit name is the one carrier of meaning the code cannot hold,
  which makes names correctness-sensitive and worth their length.
- [CONDUCTOR, to tune in use] the translation block immediately follows the fences it translates,
  one block per section; where a fence is a list of atoms whose names are already sentences, the
  block says what the names cannot say (who is speaking, what the list is for) and does not
  restate each name.

### § 1.3-what-plain-prose-is-for

- **`pro-plain-text-is-non-normative`** [TYPED 2026-09-28] — all running text outside blockquotes
  and fences is rationale and commentary. It never describes the product's behaviour, in any way.
  It may describe the world the specification is about, briefly. It mostly points elsewhere.
- **`pro-sparse-and-pointing`** [TYPED] — a specification is not the home for detail, examples,
  alternatives, or history. Most authorship at any moment has a sibling document of active note: a
  design document under `plans/` for that region of the work, or at least a ledger under `notes/`.
  Findings, examples, and the long-winded go there, and the specification carries a brief pointer
  for elaboration. Prose beside law is where normativity leaks, so the less of it the better.

### § 1.4-who-edits

- **`edit-specification-content-is-a-design-act`** [TYPED] — content is written in dialogue with
  the human, attacked adversarially and by eye, and firmed slowly into something exhaustive. An
  agent building tooling never edits it; the tier is do-not-touch during unattended builds. This is
  near `301:law-spec-touch-frontier-human-only` and not identical to it: the drift this posture
  fights is a later rewrite reading an earlier unit wrong, and the lock
  (6-results-locks-and-what-an-author-owes), not access control, is what makes that visible.

## § 2-the-loop

One unit at a time. At every step the standing question is the same: what did the design not say?

### § 2.1-choose-a-unit

- **`loop-unit-not-sentence`** [TYPED lean] — the unit is the smallest thing that can go red on its
  own, not an English sentence. English maps poorly onto the conceptual atoms of the work: one
  sentence often holds several facts, and one concept often needs several sentences. The author
  decides the unit; the sizing habits below are a starting point, to be tuned in use [CONDUCTOR].
- Too small: a `sig` with no fact about it is not checkable and is not a unit; pair a declaration
  with the fact that gives it meaning, or with the check that consumes it.
- Too large: a `fact` that conjoins several rules hides which one a counterexample refutes; a check
  that asserts three things reports one world for all three. Split conjunctions into separate facts
  and checks so that a red names one thing.
- A unit is done when it has its Alloy, its translation sentence or sentences, and at least one
  check or scenario that would go red if the unit were deleted. That deletion, done by hand, is the
  cheapest kill test there is; a unit nothing misses is either decoration or proof that the checks
  around it are too weak.

### § 2.2-write-it-and-place-it

- What kind of statement it is decides where it goes: an object and its relations are a `sig` with
  multiplicities; a rule about every world is a `fact`; a way of speaking about the world is a sig
  under the claim base with the fact that says what it means when true; a law is a `check` with its
  satisfiable twin (2.4-inhabit-before-you-believe-green); a concrete situation with expected
  answers is a scenario.
- What is shared and what is the document's: the shared tier holds only what is stable across every
  specification (who speaks, what a claim is and when it is true, the order of steps, the answers
  and their order, the outcomes' names); everything a document could reasonably define differently
  is the document's, and the two strawmen differed exactly at the definition of which converged
  steps are spared. Where a name is used in more than one document it means one thing everywhere;
  a local variant is minted under a new name, never modified in place (4-the-shape-of-a-specification).

### § 2.3-make-it-well-formed

- Alloy accepts the unit or refuses it, loudly and at the token. Most refusals are self-correcting
  and are not documented here. The one worth reading as design: an unknown name is an object the
  prose assumed and the model does not have. Ask whether the object is missing or the prose was
  wrong to assume it before adding the declaration.
- The checker's own refusals (5-assay-vocabulary-and-forms) are structural, about the form of the
  document, and never about its meaning.

### § 2.4-inhabit-before-you-believe-green

- **`loop-green-needs-a-witness`** [ACKED] — a `check` whose premise has no world passes without
  saying anything. Every check is paired with a `run` of its premise that must be satisfiable, and
  a check with no twin is recorded as unwitnessed, never as green. Keep one run that asks only
  whether the model has any world at all at the working scope; when it goes unsat, every green in
  the document is a lie.
- **`loop-twin-is-the-strongest-witness`** [ACKED] — the twin asks for the strongest world the law
  is meant to cover, not a merely satisfiable one. The monotonicity law of the whole programme was
  written backwards, copied into a second document unread, and green in both while its premise was
  unsat at the default scope; the moment its twin asked for a smaller speech set answering UNKNOWN
  beside a larger one answering DISJOINT, the law went red. A law whose premise run is unsat has
  never been read by anyone, machine or author (`fal-vacuous-green`).

### § 2.5-read-the-counterexample

The failure discipline: assume the design, not the tool [TYPED]. Triage a red in this order
[CONDUCTOR, from the strawman fights]:

1. **An admissible world you forgot to pin.** The adversary dropped an object the scenario relied on,
   or invented one the species never forbade. The fix is a fact in the scenario or a rule in the
   species, never a wider scope (`fal-wider-scope-as-a-fix`). **`rul-two-instances-lift-to-the-species`**:
   the second scenario that needs the same pin is the signal that the rule belongs to the species
   ("a cell a describer names is a key by the naming" was pinned twice before it was found).
2. **An inverted direction or order.** An order spelled with the stronger element on the wrong side;
   a universal where an existential was meant; a reflexive closure where a strict one was meant.
   Read the instance's skolems against the prose sentence: the instance is usually the sentence with
   its direction flipped.
3. **A false unit.** The fence says something the design does not mean. Fix the fence; retune the
   translation; the recorded result moves and that movement is the finding.
4. **Silence.** No unit speaks to the shape the world exhibits. That is a design question, and
   2.6-hold-a-question-open is how it is held without stopping the sitting.
5. **The tool.** Last, and rarely. A timeout is a result carrying its translation size, not a
   refutation and not a reason to retry with more time (2.8-bounds-and-cost). Before blaming the
   solver, rebuild the counterexample by hand from the instance it printed; the instance is always
   readable.

### § 2.6-hold-a-question-open

What to do when a red is a design question and the sitting must continue [CONDUCTOR, 2026-09-28].

- **`hole-what-kills-and-what-weakens`** — a *contradiction* in the facts does not weaken the search;
  it kills it silently, every check vacuously green, and the runs of
  2.4-inhabit-before-you-believe-green are what catch it. A *hole*, a shape the specification is
  silent about, weakens the search exactly as feared: the adversary finds the cheapest world and
  hands it back for every check that touches the region, the same boring world each time, and
  nothing else about those checks is learned.
- **`hole-fence-never-fill`** — when a red is silence, write one predicate naming the shape of the
  counterexample, in the vocabulary the model already has, and add its negation to the *premise* of
  the affected check and of the check's twin. Touch nothing else: no fact, no body, no meaning fact,
  no scope. A premise subtracts obligation from one check; a fact adds a claim about every world.
  Only the first is a deferral. The facts, the thing every check is measured against, are
  byte-identical before and after; what changed is the coverage one check claims, and that change
  is named in the check's own text.
- **`hole-a-free-marker-excludes-nothing`** (`fal-free-marker`) — an unconstrained indicator sig
  used as a guard fences no world: the adversary sets it empty and attacks everywhere. A guard bites
  only when written as the structure of the counterexample itself.
- **`hole-carries-its-witness-pair`** — beside the predicate, a `run` that the hole is inhabited (it
  is real, not a misspelling) and the check's twin with the guard added (the remainder is not empty).
  Together they bound the guard from both sides; its tightness beyond that is the author's judgment
  and is what a human reads at adjudication.
- **`hole-three-kinds-and-their-idioms`** — a *representable region* the model can build takes the
  guard above. A *missing definition* whose consumers already exist becomes an uninterpreted
  relation (a field on a singleton sig) with only the ruled sentences about it as axioms; its
  dependents stay honestly red until the body arrives, and never take a stub body returning a
  constant (`fal-stub-body`). *Missing vocabulary*, a phenomenon the model cannot represent, cannot
  be fenced because the adversary cannot build it either; it is a scope note in the residue
  (6-results-locks-and-what-an-author-owes), not a guard. Two more shapes: a scenario step whose
  expected answer is not yet known takes a named tautology as its outcome, so the step keeps its
  place in the world and contributes a true premise (a weaker premise can only make later steps
  redder, never greener); and a law that is not yet true as a whole stays stated in full and is
  recorded as an expected red, which is the most truthful form but answers "this whole law is not
  yet true" rather than "check everything except this corner".
- **`hole-is-a-question-not-an-answer`** — the deliverable of a sitting is the set of hole predicates
  with their witnesses, each a design question with the world that raised it. The human adjudicates
  questions, one sentence each, never fixes; each answer becomes an ordinary edit and the hole is
  deleted. An author who instead answers a question to unblock the sitting (`fal-answer-to-unblock`)
  writes a fact, and that fact silently changes what every other check is measured against; this is
  the drift the whole protocol exists to prevent. *An agent* in a sitting not licensed to design may
  add hole predicates and check premises and nothing else; that license is one diff wide and is
  reviewable as such.

### § 2.7-bank-the-answer

- **`loop-fix-where-the-unit-lives`** — a rule about all worlds goes in the species; a fact about
  this world goes in the scenario; a scope is never the fix (2.5-read-the-counterexample, item 1).
- **`loop-refuted-shapes-stay-as-checks`** — a finding enters as a check or a scenario that fails
  before its repair and passes after, and it stays. The register of refuted shapes is the regression
  suite; nothing about it is ever deleted because it now passes.
- **`loop-a-reword-moves-nothing`** [ACKED] — rewriting prose or Alloy for clarity must leave every
  recorded result unmoved. A moved result under a reword is a finding, never churn: it is the
  text-drift population made visible, and "no answer changed" becomes a diff of zero rather than a
  claim. Names and forms rename in place; nothing keeps a historical spelling.

### § 2.8-bounds-and-cost

Low priority, and to be tuned in use [CONDUCTOR].

- A bound is the adversary's headroom above what a scenario names, not a budget to maximize.
  Unnamed kinds are what a bound caps; named atoms are exact and free.
- Raise a bound only when a twin is unsat for want of atoms, to the least count that seats the
  witness plus a little headroom, and say why beside it. Lower a bound only for cost, only on the
  command or scenario that does not finish, never on the shared ceiling, and if lowering makes a twin
  go unsat you went one too far.
- Cost lives in the free-universe laws: subset-quantified statements and transitive closures taken
  per pair. Prefer an encoding that walks a chain once over one that closes it per pair; where the
  cost is inherent, cap that document's scenarios below the shared ceiling and record the reason. A
  timeout is a result to read, with its translation size, and never a reason to wait longer.
- In the hot loop, run the one law or the one scenario you are fighting, not the document: every
  scenario is its own module and every command has a name, so the smallest run is always available
  (5-assay-vocabulary-and-forms).

## § 3-writing-so-the-adversary-cannot-cheat

Habits, each learned from a world the solver handed back.

- **`hab-name-what-exists`** [ACKED] — what the solver would otherwise choose, the specification
  must state. An unnamed object the model owns that a scenario relies on is the adversary's to drop
  or invent. Where a describer names a thing, the species should say the thing exists by that
  naming, so that no scenario has to pin it.
- **`hab-direction-and-order`** — spell every order once, in the shared tier, and read every law over
  it in both directions before believing it; a law over an order that is green while vacuous has
  not been read (2.4-inhabit-before-you-believe-green). Separation and other two-place relations
  are stated about two things: say `a != b` where the design means two, or the model will let a
  thing be separate from itself.
- **`hab-defaults-fall-against-you`** [ACKED] — an unrecorded truth defaults to the worst reading (a
  step nobody recorded wrote everything); silence licenses nothing; an empty set is never ⊤, and a
  universal over an empty set is vacuously true, so a claim that names nothing must be given a
  meaning on purpose (is "marked and empty" a legal claim, or does it read as ⊤?) rather than left
  to the quantifier.
- **`hab-claims-are-data-truth-is-computed`** [TYPED] — two strata: what is so, which no claim names
  and only the wrongness test reads; and what was said, which is data the scenario states and the
  adversary may not flip once in force. A measurement is a claim, with a speaker and the possibility
  of being false; only abstract world facts exist outside claims. At-most claims for one thing
  intersect, since each bounds the same thing from above; a union widens as speech grows and breaks
  monotonicity. Entries add collisions; closures remove them; the closure is the knife.
- **`hab-attribution-and-checks-that-cannot-fail`** — attribution by single removal is honest under
  one voice and masked under redundancy; the structural reading (the derivation's support) needs no
  premise. And a check that restates a definition cannot fail and is not a check
  (`fal-restated-definition`); no mechanism catches it, so kill every law by hand once: delete a
  unit it rests on and watch it go red.
- **`hab-alloys-quiet-traps`** — only the traps that make work unsound *silently*; a loud refusal
  needs no entry here. A sequence's length is clamped to the largest integer the bitwidth admits
  without a word, so an unspelled `seq` bound is a hidden cap. Integers wrap at the bitwidth, so a
  count past it is a wrong number, not an error. A kind the scope clause does not name inherits the
  small default, which is how a twin goes unsat and a law green for want of atoms. A subset sig
  (`in`) may overlap its siblings where `extends` would not, so two subsets meant to be disjoint must
  say so. `*` includes the identity where `^` does not. A `lone` or `set` field is the adversary's to
  leave empty. And a fact written to make one check pass narrows the universe for every other check,
  which is why `fal-fact-for-now` is a habit and not a tool question.

## § 4-the-shape-of-a-specification

The document form the sittings settled [TYPED leans unless marked].

- One document per component; sections by the design's own joints; the firewall (1.2-the-firewall)
  in every section; plain prose brief and pointing (1.3-what-plain-prose-is-for).
- **`form-everything-inline-no-registry`** — no registry anybody is meant to edit; duplication across
  documents is acceptable so that context sits beside its subject; exact reuse of a scenario across
  documents is expected to be rare. Speakers duplicate most, and a document opens with its fixed
  list of them.
- **`form-ids-are-global`** — an identifier means one thing across the whole tree and must cohere
  where repeated; a local variant is a new identifier with its own associations, never the old one
  modified for one consumer.
- **`form-shared-holds-only-the-stable`** — the shared tier is opened beneath every document and
  holds only what every specification talks about and none defines; anything subtler is the
  document's, or opened explicitly by the document that wants it.
- **`form-scenarios-are-one-world-each`** — a scenario pins one world and its expected answers, step
  by step; it never graduates, and it accrues consumers rather than being rewritten for them. No
  engine or implementation noun appears in a scenario; it is written in the words of the world and
  the speech about it.
- **`form-cadence-and-names`** — a scenario reads as the concrete step, then its mapping to names,
  then its outcome, each on its own introducer so any of the three can wrap; every fenced block is
  headed by its file name; names are long and explicit because they are the one place intent lives
  (1.2-the-firewall). Loads are spelled as the shell spells them and folded into named sets rather
  than repeated.
- **`form-strawmen-are-quarry-never-seed`** [TYPED] — exploration documents that invented answers to
  reach a compiling model are frozen once their arc closes, carry a non-normative header, and are
  never promoted into a specification; a specification is written clean from the ruled prose.

## § 5-assay-vocabulary-and-forms

The words assay (`notes/30Y`) uses for the things above, so that this document and that one agree.
Mechanics, flags, and layout are `30Y`'s and are not repeated.

- The *harness* is the shell-side structure assay owns and opens beneath everything: words, classes,
  claims, and lines. The *prepend half* (`shared.md`) is the shared tier of 2.2-write-it-and-place-it,
  opened beneath every document; the *append half* (`shared-laws.md`) is what every document must
  satisfy, spliced after the document's own definitions, so a shared law may name a function each
  document defines. Both live beside the documents and are found by name.
- A *species* is a sig under the claim base with the fact that says what its claims mean when true;
  a *claim atom* is a `one sig` under a species; a *load file* is an `sh` fence of `.` lines whose
  name a scenario sources.
- A *book* is the scenario of 0-what-this-is: an `sh` fence of concrete shell lines, each followed
  by a `#}` line mapping its words to names and by `#=` lines of Alloy scoped to that line. On the
  map line a bare component is the name of the literal beneath it, the literal itself when the two
  are equal; a braced component `{class}` puts the literal in the class and leaves it named after
  itself. Any name that is not already an Alloy identifier becomes one under a deterministic,
  injective munge [TYPED 2026-09-28: a word defaults to itself, and no ceremony forces a name].
  Names are global: one literal, one name, and one name, one literal, across the document. A
  `#=` line is a *declaration* (a claim or a world object, `this` bound to the line), a *fact* (no
  `this`, hoisted), or the line's *outcome* (mentions `this`; the statement the adversary attacks,
  with the outcomes of the lines above it as premises). A trailing `for` on an outcome is that
  command's scope.
- A *law* is a `check` carrying a scope clause and runs over a free claim universe; a *corpus check*
  is a `check` carrying none and runs over exactly the document's claims, all in force, as an
  outcome of the *corpus book*, a generated book of one null-command line. A check's *premise twin*
  is the `run` named `<check>_premise`; a check without one is recorded `premise: absent`.
- Scope is the specification's: `run bookScope {}` in the prepend half is every book's ceiling, a
  document's `run bookScope {}` overrides it for that document, `run bookScope_<book> {}` for one
  book, and a trailing `for` for one command; assay appends the exact bounds of the four kinds it
  owns and nothing else.
- The smallest run: every book is its own generated module and every law and twin has a name, so
  the hot loop of 2.8-bounds-and-cost runs one module, or one command by name, never the document.
- Where assay departs from the `using-alloy` skill's idioms, on purpose: a book's atoms are named
  `one sig`s at exact bounds in a generated module of their own, where the skill's some/disj
  instance idiom serves hand-written tests inside one shared model; and `run bookScope {}` and a
  book's `run book_<name> {}` are the skill's own inhabitation probe, not the empty-block `check`
  it warns against.
- Conventions this document adds on top of `30Y`, all [CONDUCTOR]: a hole predicate is named
  `hole_<slug>` and its inhabitation run `hole_<slug>_witness`; the null outcome for a held step is
  `todo[this]`, a predicate with an empty body that the prepend half defines; a claim's long name
  is its sentence and the speaker prefix on it is a convention, not a check.
- What refuses where: Alloy refuses what is not well-formed Alloy; assay refuses what is not
  well-formed *document* (a map line that does not pair with its command word for word, a name
  covering two literals or a literal under two names, a load that resolves to nothing, a `this` on
  a line with no outcome); nothing refuses a check that restates its own definition, which is why
  3-writing-so-the-adversary-cannot-cheat keeps the kill-by-hand habit.

## § 6-results-locks-and-what-an-author-owes

- **`lock-results-beside-the-spec`** — every command's result is recorded in `<stem>.lock.json`
  beside its document, one row per command: module, name, kind, scope, result, the premise
  twin's result, and a hash of the command's text; never a timing. `--write` records the run;
  `--check` recomputes and compares in both directions.
- **`lock-is-the-ratchet`** [TYPED 2026-09-28] — `--check` passes when the run matches the lock row
  for row, reds included: a set of reds the committed lock already records, with no new one, is a
  pass. A new red, a moved result, a row present on one side only, is a mismatch. Committing the
  lock is the act that accepts a result, and the report lists the accepted reds as the document's
  residue, so nobody has to open the lock to see them.
- **`lock-a-reword-shows-as-nothing`** — a rewording that moves no result leaves the lock
  untouched; a moved row is the text-drift signal of 2.7-bank-the-answer made mechanical.
- **`gate-commit-lints`** — committing a document under `specs/` runs assay's compile lints,
  solver-free, in about a second; a refusal names the document and the lint.
- **`gate-completion-checks-the-lock`** — builder completion runs `--check` over the changed
  documents and their locks: parse errors refuse first, then the run, bounded by the runner's caps
  on wall-clock, CPU time, heap, and the whole batch.
- **`gate-one-heavy-task-per-machine`** — every solver run and every heavy gate holds one
  machine-global lock; a second heavy task is refused with the holder's name (exit 75). That is
  contention, not a failure: nothing was checked and nothing is broken; do other work and retry.
- On demand, outside any gate: `--parse` alone, one module, or one command by name
  (5-assay-vocabulary-and-forms), and the runner's `--instances` for the counterexample.
