# 30Ya — the model-mechanisation design ledger

> Tier: LLM-authored conductor ledger (Fable conductors, round 30; the 2026-09-27/28 sittings that
> produced `notes/30Y` and `plans/30Z`). Chronological: every human-typed lean and ruling of the
> design sittings, with the conductor findings the human read beside them; **[HUMAN]** marks what
> the human typed. The research phase that fed it (the question, the fronts, the per-front
> findings, the corpus scope map) is `.claude/research/design-model-mechanisation-prior-art/plan.md`.
> Grades on claims: +SURE / ~SUSPECT / -GUESS / --WONDER. Nothing here is ruled unless marked.

## The unit, as reasoned with the human (2026-09-27; read and not objected to; nothing ruled)

- Claims do not cross tiers; witnesses do. The model tier's claims are conditionals over
  speech ("given true declarations, `compare()` never lies"), whose truth-maker is
  consistency; the field tier's claims are about the truth of speech, whose truth-maker is
  correspondence with a host. § 0 and § 3.5 build the split in. So there is no consistent unit
  of claim across tiers, and there is a consistent unit of refutation for behavioural claims:
  the book, a few lines of sh with an expected verdict per line, which the panels already used
  against the abstract model and which is the product's own input at the field tier.
- Three kinds of claim: behavioural (share the book across tiers); structural (refag itself,
  complexity bounds; tier-local witnesses such as types, fences, counts; no book);
  correspondence (books, but only at the field tier). **[HUMAN]** ack that some concepts have
  no meaningful book and still deserve rigor.
- Vocabulary, the panels' own: book, rule (a slugged law restated per tier), walk (one tier's
  derivation of a verdict for a book), falsifier. Badges attach to rules; books never graduate,
  they accrue renderers. Engine nouns (mSort, mScheme) never appear in a book.
- The ratchet's content is the verdict-set over the book corpus, monotone except by human
  retirement; encodings churn beneath it. Seed corpus: `Research/GOTCHAS.md` (64), `311u`'s
  cases, the panels' witnesses. **[HUMAN]** hard ack on negative verification and mutation
  testing as a required guard.

## The proposed shape (conductor's synthesis, presented 2026-09-27; the human's reactions beside it)

One corpus of witnesses, one register of claims, one lock over both; every instrument reads
the corpus and writes to the register. Alignment of pieces that exist, not invention.

- Book corpus: a scenario as a speech record (declarations in force by owner and fingerprint;
  measurements with traversals and closures), sh lines, an expected verdict per line with the
  attribution set where the aid plane depends on it, and at least one falsifier. Books pin
  x1 and x2 only, never graduate, accrue renderers. Seed: `GOTCHAS.md`, `311u`, the panels'
  witnesses, `USER_STORY` stages as value books. A finding enters as a book that fails on the
  current model before its repair.
- Rule register: slug, quantifier prefix over (world, speech, design, implementation),
  dependency slugs fingerprinted as the x3 key, and discharges per instrument: `attributed`,
  `cased`, `model-checked(scope)`, `pinned`, `proved`, `demonstrated`, `kill-tested`, and
  `fenced` or `measured` for structural rules. Every ∃-witness travels with its ∀¬-fence; every
  fence with its un-fencing mutant. Ergonomics glosses that are checkable (add-one-true-
  statement-never-harms; gradual enhancement monotone) are two-speech-set rules, S ⊆ S′.
- Lock: computed-versus-committed mismatch fails in either direction; moves only by a human
  commit; the minispec catalogue lock extended downward, not a second lock. The residue report
  is one query: rules whose only discharge is `attributed`, by component.
  **[HUMAN] nack** on a generated index in the `SLUGS.md` style: "it's gotten large and
  unwieldy"; leaning to pull-generation (`internal-tooling md <something>` producing
  model-friendly, fully current output for a named form) or plain JSON, since models read
  JSON fine. So: the lock is a queryable artifact, its views generated on demand.
- Instruments: Alloy is the design tier's checker only (one module per component over a
  shared identity core; laws as `check`, books as exact-bound `run`, every `check` paired with
  a satisfiable `run`, a mutation lane, an equivalence check on every reword commit, pinned
  jar, small scope in CI and larger nightly). The speech record is the seam, schema from
  `311` § 5, hand-written and marked `hand` until the analyzer's front half produces it, then
  graduated by diff. `sparing-reference` and the `compare` chokepoint with Kani restate laws
  at bounds over the same records. minispec restates laws as `Prop`s with the records as the
  instance battery. DST renders books as simulated worlds; e2e runs them on hosts and is the
  only correspondence tier. Types and fences discharge structural rules. Panels are
  retargeted to attack encodings and mint falsifiers; their output is corpus entries.
- Container: prose stays in `Research/` with one discharge line per normative slug; models,
  books, and records under `spike/verify/` beside the harnesses. A weak lean, open.
- Flow: a 311 edit changes prose and module in one commit and runs the corpus; a flip is a
  finding, resolved by a human retiring a verdict in the lock commit or by repair. Graduation
  never rewrites a book.
- Front-3 corrections, conductor's read, presented 2026-09-27: `proved` is discharged over
  definitions the corpus validated (an executable extracted from the proof-side definitions
  runs the books; the theorems never see them); verdicts are stored intent, qualified in
  goblint's manner (`UNKNOWN!` soundness, `UNKNOWN` intended imprecision, `TODO` precision
  owed), never regenerated from implementation output; disagreement across instruments is
  triaged five ways (implementation, book, record, model, prose), named in the lock's failure
  output; SibylFS-style coverage (every matched shape times every answer has a book) is a
  candidate metric, cost unknown until the model's size is known.
- **[HUMAN]** 2026-09-27: "we're headed in a productive direction." No code proceeds under
  this conductor; the shape, if pursued, gets a fresh context and a concrete plan. The phase's
  final output is one `30Y` document extending `notes/301` and kin to cover design-tier
  theorems and Alloy; no separate synthesis document.

## Meta-test cases for the 30Y system (conductor's set, presented 2026-09-27; read by the human)

Twelve elements, each chosen to stress a different mechanism; names from the tree, no code
read. Forward from design: `311:3.2` two-tops (the full chain, x3 fingerprinting; three panel
books exist); `311:3.5` committee law (attributed, with the checkable fragment that every
verdict's attribution set is nonempty and names only declared speakers); `311:1.1` "the engine
never holds an mReferent" with `30T:inv-no-world-facts-in-engine` (structural; fenced;
human-acked fences only); `311:3.3` lifecycle mutation (a two-phase book with a reboot; the
temporal module). Backward from code: the Kani harness
`the_consumer_map_is_exhaustive_and_exclusive` (pins a consumer map `311:4.2` supersedes;
-GUESS it is green against the old map: the seam failure made visible); the minispec units
`JoinIsCommutative` and kin with their Kani twins (two instruments, no link; `proved` resting
on unvalidated definitions); the `sparing-reference` unit test
`provably_disjoint_feeds_survival_sparing` (a code-tier book already; register one row citing
both; rename in place); the DST pin `permuting_edge_insertion_moves_no_set_and_no_summary`
(a family-witness law). Lateral and value: `spike/fixtures/pi-webhost.book.sh` with
`package.oracle.sh` (stage 5's line 9 as `collide, TODO: spare once the stdlib keys the boot's
children`); `GOTCHAS:a-host-is-not-a-partition` (∀ over x3, fence and mutant, an NFS fixture
at the field tier). Non-311: `28M`'s withhold-only wall (∀¬ over composites, multi-author
book); `30D`'s decision table and `ORACLE_PROVIDES:provides-finished-definition` (table
completeness; a cardinality rule that is also an oracle lint). Left out by choice:
`kBACKFLIPS` (a weld; breach discipline is a different meta-test) and
`inv-known-held-never-convert` (types suffice; the system must allow "no design-tier
discharge, by decision" without reading it as residue).

**[HUMAN]** 2026-09-27: "extraction" overstates it; no complex mechanism is aspired to beyond
collation and monotonicity assertion. Some existing tests may deserve moving or enrichment so
they sit alongside the Alloy spec rather than only as e2e.

## The narrowing to two concerns, and speech as the crux (2026-09-27, human-typed and reacted)

- **[HUMAN]** the remit is two concerns and most else is noise: (1) LLM-driven design
  regression, a thing stated imprecisely and read wrong by a later rewrite, composition, or
  assumption; (2) correspondence between firmed claims and the implementation: the *same*
  cases run against the product, one-to-one greening, a single-source input that cannot be
  rewritten between checkers (an sh book with outcomes). Loader facts, sh behaviour, and host
  faithfulness are not concerns. The tool is tracked-not-proved: a pre-commit or CI gate an
  LLM runs before showing a claim to a human, as an alternative to a full adversarial review.
- **[HUMAN]** "no ruling without a book" is fine but vastly insufficient: books are small and
  stay green across design churn; what churns is a *statement* of user speech, from its
  spelling to its fundamental objects. Two claim classes beyond book-with-outcomes: same book
  and outcomes but the speech path is gradual and risk-monotonic; same book, outcomes, and
  single-user experience but collaboration carries less risk. Both require the speech slice
  in the claim. How user speech is encoded into the Alloy, and how that reaches tests, Kani,
  and Lean, is the whole game.
- Conductor's reply, acked in part: three layers of speech. L0, the ops proposition with its
  speaker, stable across design churn (`GOTCHAS.md` minus the speaker); L1, its
  classification under a design coordinate (species, relation, grain, danger), churns with
  311, is the record; L2, the spelling, churns with 312. Speech is an input relation with
  `speaker`, `kind`, `danger` labels, never a fact; `compare` is a predicate of the
  declaration set; the generic laws (never wrong when all true; monotone in speech;
  stranger-safe; attributed to the speaker only) are species-free and survive rewrites
  verbatim; per-species checks are the churny layer. **[HUMAN] ack** the three classes as
  spelled (run over the full set; check over prefixes with danger order; check over
  multi-speaker unions with attribution; cross-coordinate risk reduction as a lock diff,
  tracked not proved) as goals of `30Y`.
- **[HUMAN]** two corrections: L2 must enter at later stages, since the chartered evolution is
  abstract model, then spelling, then implementation, each keeping the old checkable truths
  and gaining ones unrepresentable before; and the L0/L1/L2 strawman as first shown carried
  no identifiers or spans linking an L0 proposition to its L1 line to the Alloy, so its value
  was not visible. **[HUMAN]** an unexpected finding: speakers may be static world facts
  ("ops distributes unevenly over speakers"), which is not how ops realities had been treated.

## The strawman (2026-09-27; `strawman.md` in this directory holds it verbatim, written by the human from the conductor's output)

- Shape shown: `world/objects.json` (named grammatical objects, NGOs, each `forall` or
  `exists`); `speech/<speaker>.json` (L0: one speaker, one claim, one object); per-coordinate
  `vocab.json` (species, relations, answers, `weaker` order, safe set, dangers, consumer map, as
  data), `rules.als` (hand-written beside the prose), `file.json` (L1: every L0 claim filed or
  explicitly unfiled), `measure.json` (each book's world read into the coordinate's species);
  books as `book.sh` + `world.json` + `expect.json` (intent in L0 ids and product verdicts);
  `laws/generic.als` (tool-owned: `Decl`, `Speaker`, `Danger`, `Ans` with declared order,
  `Query`, `True`, `answer`, `wrong`, `restsOn`; six checks: never wrong when all true,
  monotone in speech, stranger-safe, attribution honest, sufficient, minimal); generated
  instances with atoms named by L1 id; `lock.json`; later tiers (`spell.json` spans and a
  one-syllable count, loader recovery by content, a Kani harness built from the same ids,
  hostsim and e2e runs).
- **[HUMAN]** objects are `exists` or `forall`, meaningless except as objects of speech; the
  shape is yielded by 311 itself; the world's behaviour enters only at the very end, on a
  host. No `world/same.json`: declared once, a thing is one thing; two things get two names.
  **[HUMAN] nack** any proposal form: a proposal is edits to the spec; the whole is one
  specification summed across files and vocabularies, mechanically checkable in its totality;
  checking is running it and seeing green; acceptance is merging the diff or a human ack; git
  is the substrate and never a first-class concept of the tooling. **[HUMAN]** "largely
  overengineered, but that's what I expected."

## The inversion and the human's leans (2026-09-27, human-typed)

- **[HUMAN]** the JSON strawman is backwards: it authors awkward forms of what Kani, Alloy,
  and the readable specification each need. The inverted shape: a literate-Markdown
  specification as the primary product, with inline Alloy fences and inline NGO/speaker
  identifier fences, and tools that strip or generate the checker inputs. Its cost is losing
  a generated one-to-one mapping of shared things across representations. Conductor's
  reconciliation, presented and not objected to: logic hand-written in the target language
  and tied by name lints; data (objects, speech, filings, books) kept structured inside the
  document and generated into every target; Alloy's Markdown mode runs the document directly.
  JSON was over-tuned: structured data and tool output only; never sentence-shaped content
  inside a tool artifact except an explicit `md` endpoint that builds prose from facts.
- **[HUMAN] lean, "by law":** mechanical trumps prose when the two disagree, in any analysis
  where both can be present; a disagreement is a design question to sit and burn down into
  more mechanical clarity. The burn-down: the two disagree (most often the prose implies
  something the model does not encode); a reviewer notices; additive constraints go into the
  checker and the prose is tuned; a wrong prose tune can only be walked back by further
  additive narrowing. The checker's job is to force explicitness over time.
- **[HUMAN] lean, firewall:** mechanizable and unmechanizable rules cannot coexist in prose.
  Only sections that are entirely unmechanizable may carry normative prose; anything
  mechanizable is extracted to its own section containing no normative prose, fully normative
  by mechanization, so that commentary beside mechanical law never smuggles in normative bits
  licensed by inexpressibility.
- **[HUMAN]** refinements to the firewall: no section tag; a section is mechanized iff it has
  an Alloy fence. Inverted convention: all prose outside backticks is non-normative;
  normative content always sits in a fence, ```alloy or ```normative or similar. 311's
  reshaping is not urgent.
- **[HUMAN] ack:** name lints are probably a large part of the actual build-out: `simon-1` is
  one identifier everywhere; its Alloy form exists; that form mentions Alloy `Store` iff the
  claim relates to NGO `Store`; and so on.
- **[HUMAN] lean:** instances and classes are global across the product, not per document;
  an id means one thing everywhere and must cohere where repeated; a local variant is
  reminted under a new id with its own associations, never modified in place for one consumer.
- **[HUMAN]** attribution stays inside the model and the system; attribution is the product.
  Refag will be broken slowly downhill from 311 for product value; things must stay true
  regardless. A GOTCHA in classic form is one or more speaker statements plus a world and an
  outcome, one step below a speaker column. The answer-lattice observation is held for later.

## Corpus, inlining, and the book form (2026-09-27, human-typed leans)

- **[HUMAN] lean:** a new corpus tier for normative documents, something like
  `spec/311-identity-and-relations.md` at the project root; `Research/` is not built to hold
  very churny extra-normative documents. Most design documents will not reach that tier soon;
  the tier must be ready when they do.
- **[HUMAN] lean, convincible:** everything inline, no intended-to-be-edited global registry
  (a generated lockfile at most); duplication across documents is acceptable so context sits
  beside its subject for LLM and human reading; exact reuse of a book or an NGO across
  documents is expected to be rare; speakers will duplicate most, and opening each spec
  document with a fixed speaker list is fine.
- **[HUMAN] strawman, books:** every word of a book is a literal entity (`exists`) or a
  `<class>` (`forall`): `<cmd-any> arbitrary-arg <file-path>`. The book appears twice, the
  normative class form and a non-normative instance rendering, `chmod arbitrary-arg
  /etc/file.conf`. The checker then checks the ```sh content against the Alloy claims, that
  the classes the book mentions are the ones the claims are about.

- **[HUMAN]** refinements, same day: nack bindings, the tool is a shell tool and can deal with
  shell syntax; `{class}` is the inline forall syntax (braces are the least-overloaded
  choice); nack free text, it leads LLMs down refag roads, so a book is pure (normative,
  mechanized) and concrete only; nack duplicating the book, use inline comments instead. The
  sketch: a ```claims fence describing the claims in terms of users (a pared-down form of
  `sm.Path`, `sm.Inode`), then one ```sh fence in which each normative line is a `#}` comment
  over names (`#} cmd-chmod chmod-recursive <a-path>`) followed by the concrete line that
  inhabits it (`chmod -R g-w /srv/a`). A global table holds the names (`cmd-chmod: "chmod"`,
  `chmod-recursive: ["-R","g-w"]`, `a-path: Instance<sm.Path, "/srv/a">`). The normative
  lines are checked against the Alloy; the concrete lines are run someday and discussed; the
  tool enforces that each concrete line inhabits its normative line. Names are global and
  fixed project-wide (`/srv/a` is always `a-path`); a new constraint mints a new name
  (`an-etc-path`, `a-path-under-bind-mount`).

## Rules for the spec form, from the first drafted specification (2026-09-27, human-typed; `strawman-2/spec/two-paths-one-inode.md` stands unrevised until the rewrite)

- A normative shell word is either bare, an `exists`, a literal name lifted into the corpus
  and required to inhabit the same value in that word position everywhere (`#} chmod` over
  `chmod` lifts `chmod: "chmod"`); or braced, a `forall`, a class whose agreement across the
  corpus is about who uses it and what instances it gathers. Class names and instance names
  are unrelated unless expressed otherwise; the name space is one slugged space, so `path`
  cannot appear bare where `{path}` exists. Unambiguous words are typed as themselves;
  disambiguation is needed only where Dorc-visible claims tie names together.
- No `names` list: names are lifted where used; the only check is a lazy net for
  incoherence. The exception is a thing with underivable attributes, a speaker defined once
  for prosodic reasons, which has no other metadata. No `describes` per speaker (who spoke
  is an attribute of the claim); no declared classes; no declared instances.
- No prosodic notes inside any fence; fences are purely mechanical and carry their own
  meaning entire; commentary is allowed only outside, explicitly non-normative. Rule, possibly
  linted: no comment syntax at all in `sh` or Alloy fences; `#` only for machine-checked
  siblings; pure code. The one intentional hole: names are implicit claims about the world,
  so a long explicit name is the correct carrier of intent the code cannot hold, and names
  are correctness-sensitive. Lean: Alloy names match the model's names exactly (`MReferent`).
- Normative Markdown: one sentence per line, no hard wraps, in `> blockquote` marked
  `<!-- normative: -->`, so renderers wrap it; inverting 311's blockquote convention.
- Lean: drop the `#>` directives and let books repeat; `loads` spelled as sh, stilted if need
  be, compiled later; ideally few loads; specifications will eventually use shell loading
  order because Dorc does.
- **[HUMAN]** the oracle/speech-claim corner is the least baked; the rest is coming together.
- **[HUMAN]** style nits after the rewrite (`020d25fc`), for the next mechanical pass: indent
  the concrete shell line four spaces so it aligns with the mapping line; no long padded
  lines; invert to a three-line cadence, shell, then `#}` map, then `#>=` outcome, wrappable
  because each has its own introducer; fold repeated `.` load lines into a named fake oracle
  set, a block headed `# tessa-fs.sh` that other blocks source; reintroduce `# filename.sh`
  at the top of every code block, optional but favoured, since explicit slugs and permanence
  have served the project.

## The collapse, the churn walk, and derived ownership (2026-09-27; conductor findings the human read and reacted to)

- A claim collapses to one Alloy atom: a speaker, a species sig, and world-name parameters,
  written directly under the sig whose `fact` says what it means when true. The `claims` and
  `filed` fences and the proposition prose are gone; the claim's long name carries the
  world-intent; commentary about why the speaker can say it sits outside the fence. Coherence
  is structural: parameters are typed by the sig's fields. **[HUMAN]** agreed that `filed`,
  `alloy`, and `claims` were one thing and that the proposition prose was a restatement that
  could drift.
- `danger` is never authored: it is the inverse of `restsOn` per claim, mapped through the
  consumer map to the wrong answer a false claim would cause; a lock column.
- Static versus derived is structural: class sigs as parameters versus instance atoms. A
  measure is one tuple of one model relation, witnessed by a `#}=` line in the book whose
  `#=>` carries the tuple; the Alloy fact is lifted from it, never hand-written. Loads are set
  expressions in a run at the design tier and `.` lines in sh at the spelling tier. Both
  "least-baked" corners dissolve into the model; the harness holds only the welded verdict
  vocabulary, `MDecl` with a speaker, an ordered answer set, `Query`, and three predicates.
  **[HUMAN]** any declaration the harness holds must be a universal truth that cannot range
  over values as the product evolves; fine to try measures entirely in the book; it may come to
  resemble errorloom. Conductor: a book is a transcript case; -GUESS the loom case format can
  be reused outright.
- The churn walk over four of 311's own changes (one species to two; the aspect, cell,
  per-cell-store, and parent-declares generations; the closure rewrite; the re-seated store
  closure): the document changes (sigs, meaning facts, one line per claim under a touched
  species), the lock changes (verdict rows move; the reboot book fails a law at the
  singleton-sort generation, which was `311p` thread 2; the start-then-enable row's elide,
  guard, guard, elide is `312c` thread 4 as a diff), and the harness does not. Cost of a
  rewrite is countable as diff lines per claim.
- "Who can know" is not speaker metadata: 311 § 1.2 makes a sort one owner's vocabulary and
  § 3.5 lists per species whose line a statement must be. Ownership is derived: the speaker of
  a sort's identity claims owns it; a per-species owner-role table in the coordinate makes
  the seat rule a check, and `311p` thread 4 fails it. Strangers and cooperative cases fall
  out. A speaker needs a stable id and a prosodic line only. **[HUMAN]** the claim is the
  atomic unit of collaboration; a region of knowledge is the sort, principled because it is
  the engine's own unit; lean to name speakers by expertise and unification
  (`expert-docker-1`) with the mint-a-new-id praxis. Consequence: the identity claim is always
  the first line of any speaker's speech about a thing; an unowned sort's sort-level claims
  cannot pass the seat check, and its sites guard.

## The settled book form and the state at rewind (2026-09-27; human-typed rulings, conductor findings the human read)

- Two strata in the model. Truth: `reaches` and `worldParent` on `MKey`, named by no claim, read
  only by `wrong`. Knowledge: what the loaded speech says, including derived claims,
  `Resolution { of, to: lone }` and `Placement { of, within }`, whose speaker is derived from
  the owner of the key's scheme and whose truth is computed against the truth stratum, not
  assumed. A resolution with no target is a decline, always true. **[HUMAN]** the split of
  literal measurement from claimed measurement is the preferred approach; declining matters
  for loader, fail-through, and collaboration, since a declined answer can be re-answered, and
  that is unsettled by design, so the checker must be able to talk about it.
- Default resolutions are derived by the harness from the static scheme claims composed with
  the world facts, under the hypothesis that the lookup is correct, and generated into the
  book module; a human writes only deviations, a decline or a wrong lookup. **[HUMAN]** the
  written resolution lines felt like claims belonging with the oracle content; they were the
  hand-written default, and the errorloom analogy holds: setup is the world, the oracle's
  output is derived unless the test is about the oracle misbehaving.
- Two comment forms and nothing else. `#}` binds shell words to names, shell-lexed, `{}` a
  word kind, underscores so one spelling serves shell and Alloy. `#=` is line-scoped Alloy,
  lifted verbatim across consecutive `#=` lines, `this` bound to the site the line above
  generates, linted so every free name is that line's own: its map names, instance names it
  introduces, `Loaded`, claim atoms, `this`. A line whose `#=` holds no verdict form is a
  fixture line and binds no `this`. Cross-line statements go in an Alloy fence; the seven
  books have none. **[HUMAN]** collocation by scope is the principle; freeform Alloy beside a
  shell line is valid only where it is mechanically checked to hold of that line, and
  file-line references are a non-starter. The introducer characters are free to grow; the
  syntax serves only this project, keeps information local for an editing LLM, and is
  skimmable by the human.
- Verdicts are Alloy: `this in Ran`, `this in Elided`, `this in Guarded`, three subset sigs of
  `Site` defined once from the consumer map; the e2e compiler recognises exactly these forms
  to emit the expected transcript. **[HUMAN]** inverted from sugar: a fixed set of Alloy forms
  is the source and the expected-run files are compiled from them. Attribution is never
  authored: `restsOn` is computed and locked; an attribution intent is a scoped `#=` check
  beside the outcome it constrains (`… not in by[this]`).
- Books are named `# <name>.sh`; repeated loads fold into named oracle sets, one block per
  speaker, sourced by books. The site's argument is the last key-typed name on its map line,
  a generator convention, soft.
- 311 § 1.3 added as a fact: a scheme yields or is primary, never both; without it the laws
  module admits an instance where a stranger's primary claim on `Path` breaks monotonicity.
- State: `notes/30Ya-strawman-2/spec/two-paths-one-inode.md` and its `build/` at the commit that
  follows this entry hold the final shape. Everything is unrun; every result in `report.json`
  is expected, not observed. Soft spots an implementer inherits: braced class words appear in
  no book, so `{class}` may be unnecessary in books; the `#=` verdict forms are the only
  vocabulary the e2e compiler knows; sites are single-argument, redirects and pipes are outside
  this slice; the claim name's speaker prefix duplicates the `speaker` field, one should give;
  re-answering a declined key by another scheme is representable and unmodelled; the temporal
  module (a resolution leaving `True` after a write, § 3.3) is the next missing species;
  `attributionSufficient` may fail on redundant speech, since `restsOn` is single-removal;
  scope five or six is the likely ceiling for the subset-quantified laws.
- **[HUMAN]** closing: no code under this conductor; the next step is a clean-context attempt at
  implementation from this ledger and the strawman, and the phase's durable output is one
  `30Y` document extending `notes/301` to design-tier theorems and Alloy.

## At the rewind (2026-09-27; the successor conductor's read-in, and the human's typed corrections to it)

- Conductor's observation, **[HUMAN]** "a true observation": the generic laws call `Query`,
  `answer`, `wrong`, `restsOn`, `by`, `Site`, and the three verdict sigs, all defined by the
  spec and none by the harness, so the tool's contract with a spec is a by-name signature that
  nothing yet checks. **[HUMAN]** a meaningful spec document must specify what it is talking
  about as part of its specification; the division between the harness and the part of a spec
  shared by most or all specs is a meaningful distinction, not noise. Where it lies is not ruled.
- **[HUMAN]** the spec tier is no-builder-edits, a do-not-touch tier during unattended build
  runs, near minispec's posture though not identical (minispec stays LLM-built). In practice
  the pattern is the same as every design corner: a frontier model and the human, attacked by
  adversarials and by eye, firmed slowly into something exhaustive. Far out; the present work
  is the Alloy harness only.
- **[HUMAN]** ack the shape: a minimal harness first, then back to 311 work to use it. **[HUMAN]
  nack** promoting the strawman into `spec/`: turning 311 into a spec is delicate, clean-context,
  product-focused frontier work, guaranteed to surface a dozen underspecifications at once.
- Conductor's holds, unreacted (chat-only until reacted): the laws module opens the claim
  atoms; the correspondence half stays out of the first cut.

## The agnostic recut of assay (2026-09-27; human-typed leans, and the consequences the human read and acked; the plan is `notes/30Y`)

- **[HUMAN]** 30Y is tooling only: what assay reads, builds, runs, reports. No praxis in it
  (where normativity sits, fence rules, name-correctness lints, which are not mechanically
  checkable). The tool's name is assay.
- **[HUMAN]** assay stays agnostic of anything 311-shaped, and as far as is reasonable of
  anything Dorc-shaped: a generic solver over sh-spelled lines mapped to abstract claims about
  those lines. No model-hygiene checks. It will eventually own the outcome vocabulary, for
  mapping between tools, and nothing else Dorc-shaped; punted.
- **[HUMAN]** Markdown in, Alloy out; JSON in, JSON out; a nonzero exit is how it speaks. No
  prose, opinions, suspicions, or hints, in text or smuggled into JSON. Lean on the user
  invoking Alloy directly wherever possible. Fancy reporting nacked for now; listed as later.
- **[HUMAN]** a class is an assay-specific opaque category of strings: arbitrary, never parsed,
  never meaningful, not a scheme; its correspondence with mScheme is accidental. It exists so two
  claims can agree about two different literals as one kind of text; where they can share the
  literal, they just do. Braced words are join nodes, not quantifiers (the prior conductor
  doubted a book forall exists); a "for every inhabitant seen so far" check may come later.
- **[HUMAN]** `#}` binds only the immediately preceding line, naming or classing each of its
  components; never a general truth over other lines. Any component may be classed, the
  command word included. A bounded word-set form (`{foo} rest... a_bar`, both ends bound) maybe
  later. Conductor's join rule, acked: the literal is the join key; one literal named on one line
  and classed on another is one atom carrying both.
- **[HUMAN]** the unit is the logical line; assay owns shell truths and exposes decomposed
  structure to Alloy (command and argv now; parts, redirects, substitution trees later), never
  literals. Nack on refusal machinery: absolute MVP, least work, hand the map line to the syntax
  crate's lexer.
- **[HUMAN]** statements of what flavour or scheme a thing is are claims, not fixture lines;
  `inode_x.class = inode` belongs to the oracle that mints inodes. Consequence, acked in the
  strawman: a class on a map line and a claim about the command's argument position type a named
  word; the fixture states world truth. Residue the strawman keeps: a book pins the scheme of
  each token the host printed (see the open-world finding below).
- **[HUMAN]** no declared literals in a spec: every word is minted by assay. Conductor's fix: a
  `words.als` module per document, below claims, holding every literal, class, and introduced
  name, with class memberships closed.
- **[HUMAN]** assay opens the tree-global `spec/shared` into every generated module; anything
  subtler is opened explicitly. Integers stay (argv positions; positional catalogs). Three-space
  indentation. Claims are atomic; a load is a set of them; no partial override; the wrapping
  concern is withdrawn; "oracle sets" are load files, since claims may later ride lines of
  oracle sh.
- Conductor findings the human read, unreacted beyond the ack to proceed: a line's run carries
  the lines above it (prefix runs), else a later run can dissolve an earlier query by choosing
  its convergence; a `#=` that is a declaration is emitted at module level; an argv word is a key
  only where a claim in force reads it, else a described verb's empty write set spares
  vacuously; a `run` over the open world a fixture leaves passes when any admitted world gives
  the verdict, so the book must pin what the solver would otherwise choose, and a `check` with
  convergence taken as given is the alternative to price in the first experiment; `Key in
  Shword` makes one value under two parents unrepresentable, a modelling choice for the 311 work.
- State: `notes/30Ya-strawman-2/` rewritten on this harness (`harness/assay.als`, `spec/shared.md`, the
  spec, `build/`); everything unrun; `report.json` is expected, not observed. Next: rewind and
  dispatch the implementation of `30Y`.

## Checker as adversary, and convergence as a claim (2026-09-27; human-typed leans, and the consequences the human acked)

- **[HUMAN]** the first purpose of assay, ahead of the two payoffs `30Y` listed, is to enforce
  rigor: make it hard for the design process to write a wishy-washy line into the spec. Strong
  lean: checker as adversary. Conductor: that is `check` semantics. A `run` passes when any
  world the fixture admits gives the verdict (the solver fills every gap in your favour); a
  `check` passes only when the verdict is forced (the solver fills every gap against you), and
  its counterexample is the sentence you forgot to write. The laws and corpus checks already
  were checks; only book outcomes were runs, inherited from a strawman whose generator pinned
  the world tightly enough that a run was as good as a check.
- **[HUMAN]** "the key whose word is `a_path`" is good, not a cost: a key is a first-class thing
  a line takes, and the spec should be explicit about it. What stays out of a spec is assay and
  Alloy minutiae (scopes, bounds, munged names). `Key` becomes its own sig carrying a word.
- **[HUMAN]** the engine's final decision, which lines elide, guard, and run, is as a whole what
  the model concludes; the outcome is always the checked statement, `Ran` included.
- **[HUMAN]** two characters at most for any new introducer, and suspicious that one is needed.
  It is not. The given the adversary must not flip is "this line converged", and **[HUMAN]**
  convergence is not a measurement-as-given: a measurement is a claim; only abstract world facts
  exist outside claims. Consequence, **[HUMAN]** "nearly ideal": a verdict claim
  `Verdict { of: one Line }` is declared on the site line (`one sig carl__… extends Verdict {}
  { of = this }`), its speaker derived from the verb's check owner, true by the static default
  until a spec models the state it measures; `Converged` is derived as the lines whose verdict
  claim is in force and true; a claim cannot be flipped. The static lift: on a site line a
  declaration is a claim and a formula is the outcome; world facts never need `this`. assay
  emits a `check` per line with the earlier lines' outcomes as premises and one `run` per book
  that all outcomes hold together. A line that runs writes only its outcome. **[HUMAN]** this
  makes "a convergence claim is fallible human speech" visible and atomic in the spec.
- Conductor finding, posed after the fact and corrected: with a first-class `Key`, Alloy's
  default scope of three no longer fits a book. **[HUMAN]** significant findings are posed
  before the patch, not resolved inside one; and assay cannot fix this, since it cannot know the
  sizing bounds of modelling objects.

## Scope is the spec's (2026-09-27; human-typed, from the first-principles explainer)

- **[HUMAN]** the ceiling must be configurable by the spec author and shareable across spec
  files ("a key is a thing across the entire spec"). Conductor, acked: Alloy has no
  document-wide scope, only a clause per command, so the spec spells it as a command with an
  agreed name and an empty body, `run bookScope {} for 12 but 4 Int` in `spec/shared`, which
  assay copies onto every command it generates; `bookScope_<book>` in the owning document
  overrides one book; a trailing `for` on an outcome line overrides one command. The ceiling
  caps one world, not a total across books; the headroom above what a fixture names is where
  the adversary builds counterexamples; the shared number is the common case plus headroom.
- **[HUMAN]** "eight named things" felt absurdly small, with the counter-thesis that the scope
  also sizes the exploration space, and a worry that scope pressure pushes toward not modelling
  interim things like keys, missing exactly where precision was needed. Conductor, acked:
  names are exact and free; the scope caps only unnamed kinds, so first-class keys added no
  atoms to any book, only a kind that needs a ceiling; eight was the strawman's largest fixture
  plus one and is a weak adversary; twelve for this strawman. The laws explore the free
  universe at scope six regardless of how books are sized.
- **[HUMAN]** a per-book escape is required: a gnarly book of eight commands and ten to fifteen
  keys may need a cap of twenty without the forty two-line exemplars paying for it. Conductor,
  acked: a scope is paid per command, so a large book's ceiling costs only its own commands;
  the three-tier convention above is the escape. `30Y` § 2.4 carries it as
  `mech-scope-is-spelled-in-alloy`; the strawman's shared module carries `bookScope`.
- Also this sitting: `Key.word` is `w` (so `w.inode_x`) and `Query`'s ends are `writer` and
  `reader`, a mechanical rename the human delegated.

## The pre-step: Alloy in anger before assay exists (2026-09-27/28; human-typed framing, conductor's actions)

- **[HUMAN]** not the `30Y` build: a last proving-out, deploying the upstream tool against a real
  Dorc problem without the constraints of perfect specification text. An Opus builder stands up
  the minimum that runs `.als` files (mise-managed JDK and Alloy jar, one runner task, one
  platform; no compiler, no scaffolding); the conductor hand-writes a second strawman, spec and
  build both, targeting completeness and compileability, on something tractable that is not
  311 so the state-space coverage grows; then the builder merges it and fights it toward green.
  Fable tokens go to the authorship, not the tooling.
- **[HUMAN]** the primary output of the series is the record of where Alloy fights the author,
  divided carefully at the end into good correctness-chafe and things the tooling should
  absorb. Where Alloy fights because the design is genuinely not firm, that is the tool doing
  its job: log it as a surfaced softness, then the conductor has full latitude to invent an
  answer for the strawman and pursue green, for this session only. The builder has no such
  latitude: it repairs mechanically and returns design questions to the conductor.
- **[HUMAN]** every file in a strawman directory carries a strawman, non-normative header, so
  invented decisions can never be read as Dorc's design. Also from this sitting: three
  backticks inline break the human's renderer; name a fence kind in words.
- Conductor's choice of subject: the per-line outcome algebra past a wall (elide, guard, run,
  survive under the flag; footprints, backings, withholds; the identity tier's answers as
  opaque claims), the vocabulary assay is meant to own one day and the region `USER_STORY`
  stages 2 and 5 narrate. `notes/30Ya-strawman-3/`: nine books, ten laws, four
  corpus checks; `FINDINGS.md` there is the fight record and carries the final division.
- The runs (`notes/30Yb`; the lane folded into `ai/main` 2026-09-28, worktree and branch
  removed). Strawman-2: five mechanical fights; three law premises vacuous at the default
  claim scope; the seat check red for want of a universe; every per-line check with a query
  timed out (two million variables; the `separated` closures); at `8 Claim` four laws red,
  which the strawman-3 findings now put under suspicion of the inverted-law bug below rather
  than of the identity model. Strawman-3, first run: seventy-six of eighty-one commands as
  expected, none over twelve seconds at the ceiling of twelve; five reds, three design catches
  ruled and applied, marked strawman: `monotoneInSpeech` and `strangerSafe` read backwards in
  both strawmen (`weaker` puts the stronger answer on the right), separation is irreflexive, a
  cell a describer names is a key by the naming. Second run: ninety-one of ninety-two as
  expected; the one-voice removal-attribution law went green, correcting the conductor's own
  hand-reasoning (a sole footprint removed widens to every key, which includes the backing's
  own, and nothing separates a key from itself). The division of every fight into
  correctness-chafe, tooling-tune, and encoding cost is `FINDINGS.md` "The division"; the
  short form: fights found while writing were mostly tooling, fights found by running were
  mostly correctness, and six of the eight correctness items were invisible until the solver
  handed back a world.

## The `30Y` firming sitting (2026-09-28; human-typed rulings on what the runs taught)

- **[HUMAN] ack**, applied to `30Y`: harness names (`Claim` abstract, `above` for `before`, no
  field name shared across sigs, a lint on Alloy's reserved words); one flat directory per
  document with the harness and the shared module written in; a declaration on a line binds
  `this` whether claim or world object; a timeout is a result with its translation size, not a
  runner failure (acked "ish"); assay sets `seq` wherever it sets `Int`.
- **[HUMAN] nack** on "section three is stale" as framed: `30Y` is a living document that
  describes what is, never what is built against what is to come, and must read equally current
  when the build is done. Applied: § 3 describes the runner and the two fixtures in the present
  tense, with no built-or-unbuilt language.
- **[HUMAN]** praxis lives in this ledger for now; `30Y` stays tooling only.
- **[HUMAN]** no lean on the corpus universe and neither first option liked; the alternative,
  a generated corpus book of one null-command line whose speech is every declared claim, so
  corpus checks are ordinary outcomes with truth-in-force defined, acked as marginally better.
  Applied to `30Y` § 2.3, § 2.5, § 2.8; `corpus.als` is gone from the layout.
- **[HUMAN]** the shared tier is a prepend half and an append half, both authored in `spec/`;
  assay stays a thin preprocessor that opens the first beneath every document and splices the
  second, as text, after each document's definitions into its laws module (Alloy resolves
  names downward through `open` only, so a shared law over a document-defined name can be
  shared no other way). On the human's question, clarified and **[HUMAN] acked**: no
  assay-flavoured content lives in the append half; the names its laws use (`answer`, `wrong`,
  `support`) are the spec tier's own convention that every document must satisfy, Alloy refuses
  a document that leaves one undefined, and names in one part of `spec/` constraining names
  elsewhere in it is fine as long as they all appear in `spec/`. Applied to `30Y` § 2.2.
- Resolved by the above, no longer open: where the generic laws live (the append half; the
  inverted law would have been written once), the corpus universe (the null-line book), and the
  scope convention for laws (the append author sizes each generic law once).

## Praxis for writing a specification (from the strawman fights; **[HUMAN]** this ledger is the home for praxis for now, since `30Y` is tooling only)

- What the solver would otherwise choose, the book must state. An unnamed object the spec
  owns (a key, a referent, a world record) that a book relies on is the adversary's to drop or
  invent; the counterexample names it, and the fix is a fact in the book or a rule in the
  species, never a wider scope. Two instances in one strawman before the species-level rule
  (a named cell is a key) replaced the per-book pins.
- The premise twin of a law is written as the strongest witness the law is meant to cover, not
  the trivial one. `monotoneInSpeech_premise` asking for a smaller set answering UNKNOWN and a
  larger one answering DISJOINT is what let the inverted law fail on the next run; a twin that
  is merely satisfiable would have passed it.
- A check that restates a definition cannot fail and is not a check (`attributionMinimal` was
  the definition of `restsOn`); the emilia specimen. No mechanism catches it.
- The shared tier holds only what is stable across specifications: speakers, the claim base
  and the truth default, the order of lines, the answers and their order, the verdict and
  convergence, the flag, and the names of the outcomes. The sparing law, and so the definition
  of which converged lines elide, is each document's; the two strawmen's shared modules
  differed exactly there.
- At-most claims for one line intersect; a union widens a footprint as speech grows and breaks
  monotonicity.
- Every name a specification mints avoids Alloy 6's reserved words, the temporal set included,
  and never reuses a field name across sigs where a join could go through it.
- A corpus check is an outcome of the document's corpus book, one null-command line whose
  speech is every declared claim, so truth-in-force is defined there and a check may read it
  (`30Y` § 2.3, **[HUMAN]** acked 2026-09-28). The generic laws live in the spec tier's append
  half, spliced after each document's definitions, so they are written once and every document
  must define the names they use (`30Y` § 2.2, **[HUMAN]** typed 2026-09-28).
- The counterfactual reading of attribution (single removal) is honest under one voice per
  line and dishonest under redundant footprints; the structural reading (the derivation's
  support) needs no premise and is what `311` § 3.5's sentences describe.
