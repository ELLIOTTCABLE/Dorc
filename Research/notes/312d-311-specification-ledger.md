# 312d — The 311 specification ledger: turning the identity model into `specs/311`

> AI-authored (Fable, from 2026-09-28). Notes-tier LIVING ledger for the arc that mechanizes
> `notes/311` into `specs/311-identity.md` under the praxis of `plans/30Z`, checked by assay
> (`notes/30Y`). Nothing here is ruled unless it cites a ruling by `docID:slug` or carries
> **[TYPED]**; grades are +SURE / ~SUSPECT / -GUESS / --WONDER; **[HUMAN]** marks the human's
> framing, paraphrased from chat, never a ruling. Authority: the root docs, `spike/CLAUDE.md`,
> `specs/AGENTS.md`, `plans/30Z`, and the normative text of `notes/311` outrank this. Successors
> append sections at the tail and edit nothing in the middle.

## § 1-remit-as-typed

- **[TYPED]** 2026-09-28: there will be one 311. The note `notes/311` becomes `specs/311-identity.md`
  under the same docID; the note does not survive beside the spec.
- **[TYPED]** the sitting closes no design hole. Every known hole stays in the produced document
  as a `hole_` premise (`30Z:hole-fence-never-fill`); sections may stay red because of them, and a
  correctly-red result is a finding, never a defect to fix. The mechanization is expected to
  surface further holes, under the same rule. The document may reach no soundly-green output at
  all; that outcome is reported, never papered over. Eventual soundness is the goal, never
  greening.
- **[TYPED]** the current normative text of 311 is sacrosanct, where known wrong included, and its
  design-relevant underspecifications with it. A novel hole found during this work that no
  crosscheck item names becomes a held hole pending a design sitting; the conductor makes no
  design choice to close it, however small.
- **[TYPED]** clean cloth: the strawman directories under `notes/30Ya-*` are not read; the
  `30Ya`–`30Ye` build ledgers are avoided unless badly needed. The inputs are general Alloy
  usage, assay usage, and the r31-series text.
- **[TYPED]** non-vacuous green is to be reached for as much of 311 as possible, under the rules
  above.
- **[TYPED]** this is the first production Alloy in the codebase and sets the bar: production
  quality, best practices established and followed.
- **[TYPED]** Fable-tier and Astra-tier subagents are authorized at the value tier for this work
  where valuable; every dispatch still needs the human's typed ack. The human's expectation:
  main-context authorship first, worker churn and tooling later, an adversarial crosscheck last,
  under further instruction and possibly a successor conductor.
- **[TYPED]** the fixes and the crosschecks proceed on the platform this arc builds; neither is
  under attention here.

## § 2-scope-of-work

- In scope: the specification document itself, its lock, the shared halves under `specs/` where
  311 needs them, and whatever mise or hk plumbing the authoring loop proves to need.
- Out of scope: any edit to 311's meaning; any resolution of a `312cg` § 3 hold or a `312ch` § 1.7
  hold; the adversarial crosscheck over the result; any change to assay's own behaviour beyond
  what compiling 311 forces, which is reported upward first.

## § 3-strategy

Written before any authoring, 2026-09-28. Conductor's plan; the human may veto any line.

### § 3.1-the-shape-of-the-mechanization

- `str-two-strata-truth-and-speech` — the model has a WORLD stratum the engine never holds
  (mReferents; which mReferent an mKey reaches; which mReferent a store holds) and a SPEECH stratum
  (claim atoms with speakers: warrants, closures, declarations, may-read and may-write sets,
  sentinels), per `30Z:hab-claims-are-data-truth-is-computed`. Every relation of 311 § 2 becomes
  a claim species with one predicate, `trueOf`, that transcribes 311's own defining sentence into
  the world stratum. `compare()`, the writeset, the region test, and invalidation are computed
  from speech alone, as the engine computes them. The laws of 311 § 0 are then checks of the
  form: every claim in force is true of the world, and the engine answered SAME (or DISJOINT, or
  spared), therefore the world agrees. A wrong answer with every statement true is the
  counterexample 311 § 0 says refutes the model. This is the one encoding under which a green
  means what 311 means.
- `str-truth-predicates-are-transcriptions` — the largest way this work could mint a hole is a
  `trueOf` that says more or less than 311's sentence, since the laws are exactly as strong as
  those predicates. Rule: each `trueOf` is a transcription of one quoted sentence of 311, cited in
  the translation block; each is exercised by a scenario where the claim is false and the law
  catches it; where 311's sentence admits two readings the relation is left uninterpreted with
  the sentence's consequences as axioms, so the adversary chooses the reading, and the hole is
  named (`30Z:hole-three-kinds-and-their-idioms`, the missing-definition idiom).
- `str-lower-bounds-not-definitions-for-closures` — the writeset (311 § 2.6) is "the least set
  that four rules close". For a soundness law the adversary wants the set small, so axiomatizing
  "closed under the four rules" without minimality is exactly the lower bound a soundness check
  needs, and it leaves the construction-against-test ambiguity of `312ch` item 11 to the
  adversary rather than choosing it. Minimality matters only to value laws, which are red until
  the design says which reading holds.
- `str-static-over-lines-not-temporal` — the book's line order (assay's `above`) is the only
  chronology 311 needs at v1: invalidation is "some line above touched a member", a span is a
  run of lines with no wall. No `var`. Temporal encoding stays latitude for the lifecycle
  species, per `30Y` § 4.
- `str-refuted-shapes-are-inhabited-runs` — each `311u` entry that names a candidate rule becomes
  a predicate for that rule and a `run` that the rule's wrong answer is reachable with every
  statement true, `expect 1`, so the killer world stays inhabited on every edit
  (`30Z:loop-refuted-shapes-stay-as-checks`).
- `str-flag-is-a-world-fact` — `--risk-faultless-skips` is a `lone sig` present or absent per
  world; consumers that 311 gates on it demand it; one law states that nothing flag-priced is
  consumed without it.

### § 3.2-order-of-authoring

One unit at a time under `30Z` § 2, each unit with its fence, its translation, and one check or
scenario that dies if the unit is deleted. Order, chosen so the first non-vacuous greens are the
model's core:

1. Standup: the toolchain as found (`mise run assay` on a skeleton; `--parse` before any solve);
   `git mv` of the note to `specs/311-identity.assay.md` so history follows; the shared halves
   (`specs/shared.assay.md`: Speaker, the claim base, in-force, `bookScope`, `todo`;
   `specs/shared-laws.assay.md`: empty until a law proves shared).
2. The chain: mReferent, mSort, mScheme, mKey with value, scheme, parent; `:primary-of` and
   `:identified-in` per shape; `:root`; the mRoute terminus; `identity()` (311 § 1.1, § 1.4, § 1.6,
   § 1.8, § 2.2, § 2.4, § 3.1). Inhabit.
3. Tokens and the warrants: `:guarantees-unique-referent`, `:guarantees-unique-name`,
   `:aliases-nothing-else` (§ 1.5, § 2.3).
4. `compare()`: the one-level rule, the walk, the four answers; the laws `same-is-sound`,
   `disjoint-is-sound`, `nobody-spoke-declines`, `partial-measurement-never-widens`,
   `separation-from-one-definition` (§ 3.2, § 0). Twins, kills, and the first scenarios (hardlink;
   two files one filesystem; nested pid namespaces; NFS across two routes).
5. `:yields` and the seats (§ 2.1, § 1.6 disagreement refusal).
6. may-read, the writeset, the sparing test (§ 2.5, § 2.6); scenarios from USER_STORY stage 5 and
   the loop image.
7. Traversals, the region test, `:places` (§ 1.7, § 2.9, § 2.10).
8. Invalidation (§ 3.3).
9. Observer, correspondence, cells, composite, vantage, entry and lends (§ 2.7, § 2.8, § 1.9,
   § 2.11, § 1.10, § 3.4).
10. Committee law and attribution (§ 3.5): the support of every answer is one speaker per step.
11. The residue: every 311 sentence not mechanized re-homed under `<!-- normative -->`; the
    register § 4.2 and the boundary § 4.1 as normative prose residue; § 5 and § 6 as commentary
    verbatim; the sentence accounting; the lock written and committed.

### § 3.3-document-form-decisions

Each is a form call within the praxis, not a design call; the human may veto any.

- `form-section-slugs-are-kept` — every section slug of 311 is kept verbatim, since the corpus
  cites them as `311:slug`; new subsections only beneath them.
- `form-residue-lives-in-a-sub-section` — a 311 sentence that stays prose lands under a
  non-mechanical sub-heading `residue` beneath its section, as a `<!-- normative -->` blockquote,
  so no normative sentence is silently demoted to commentary by
  `30Z:fw-a-section-is-mechanical-iff-it-holds-a-fence`. The accounting of every normative
  sentence of 311 into fence-plus-translation or residue is owed at the close.
- `form-translation-reuses-311-sentences` — a translation block reuses 311's own sentence where
  the fence says exactly that, so the diff against the note is minimal and the drift surface
  small.
- `form-alloy-names-keep-the-m-prefix` — model objects keep 311's spelling in Alloy (`sig mKey`,
  `sig mSort`), so one grep finds prose and code; relations and predicates are lowerCamel of the
  prose slug (`identifiedIn`, `guaranteesUniqueName`); claim atoms follow assay's
  `speaker__sentence` convention; long names over comments, since fences carry no comments
  (`30Z:fw-fences-are-purely-mechanical`).
- `form-scenarios-are-plain-sh` — every scenario is a book of real commands from the cases 311
  and `311u` already cite, with the describers' claims as load files, per
  `30Z:form-scenarios-are-one-world-each`.

### § 3.4-priorities-in-tension

- Fidelity to 311's letter against mechanizability: resolved toward the letter by
  `str-truth-predicates-are-transcriptions`; an unmechanizable sentence is residue, never
  approximated.
- Coverage against vacuity: more mechanized means more encoding surface; mitigated by the kill
  step on every law, the premise twin on every check, and a scenario where each claim species is
  false.
- Readability against solver cost: names are free and long; closures over derived relations are
  not, so the chain is one `parent` field and every walk closes over it alone.
- Value against safety in the encoding of closures: soundness laws take lower bounds
  (`str-lower-bounds-not-definitions-for-closures`); value laws stay honestly red where the text
  underspecifies.

### § 3.5-questions-for-the-human-non-design

Answered by default as stated; a word from the human overrides.

- `ask-heading-level-is-the-section` — the firewall's "section" is read at every heading level,
  so a `residue` sub-heading with no fence is non-mechanical although its parent holds fences.
- `ask-register-is-normative-residue` — § 4.1 and § 4.2 are carried as `<!-- normative -->`
  blockquotes (they state what the model excludes and contradicts), counted as residue.
- `ask-index-and-renames-stay-as-commentary` — § 5 and § 6 stay verbatim as commentary; both
  already declare themselves non-normative.
- `ask-commit-granularly-on-ai-main` — one commit per unit on `ai/main`, the lock committed with
  the fences it records.

## § 4-standup-findings

- **[TYPED]** 2026-09-28, on sharing: `specs/` may hold more than the one document; primitives
  likely to be reused by other components' specifications may be divided out. Correctness is the
  only guiding star, and sharing that harms it is not attempted. Any shared content that is in
  any way normative is in the `30Z` format, visible and coherent with every other
  specification, never buried in a supportive preload file.
- `dsp-shared-tier-is-minimal-and-in-form` (conductor's disposition) — the shared halves
  `specs/shared.assay.md` and `specs/shared-laws.assay.md` are written as `30Z` documents (fences,
  translation blocks, the firewall), and hold only what no specification defines: the speaker,
  the claim base and its speaker field, the in-force rule, the null outcome, and the default
  book scope. Everything 311 defines stays in 311. With one specification in the tier, what a
  second would share is a guess; lifting a unit into the shared tier later is a reword the lock
  must show moves nothing (`30Z:loop-a-reword-moves-nothing`), which is the correctness-preserving
  direction. Alloy composes modules only downward through `open`, so a shared definition can
  never depend on a document's own; that constraint is what keeps a shared unit from becoming a
  buried rule.
- `fnd-corpus-walk-did-not-know-specs` — the slug index and the docID dangle lint anchored their
  walk at `Research/`, the steering files, and the root docs, so a note moved to `specs/` would
  have dangled every `311:` citation and dropped 311's section slugs from `SLUGS.md`, against
  `Research/README.md`'s rule that a mechanised note moves under its own ID. Fixed in the shared
  corpus walk (`spike/crates/internal-tooling/src/corpus.rs`, `docids.rs`), two lines, committed
  with the move. The lint is silent over the whole tree afterward.
- **[TYPED]** 2026-09-28, tooling settled and working: every `30Z` document is spelled
  `<stem>.assay.md` (the shared halves included; assay refuses any other name); the lock is
  `<stem>.lock.json` and the out directory `target/alloy/<stem>/`; the docID lint knows
  `specs/` and walks all of `Research/` by filename, the quarantine included.
  `mise run assay -- specs/311-identity.assay.md --parse` is green on the fence-less document.

## § 5-strategy-amended-after-the-rewind

Written 2026-09-28 by the rewound conductor, before any fence was authored. § 3 stands except
where a line below supersedes it; each superseding line names the § 3 slug it moves.

### § 5.1-tunes-typed-this-sitting

- **[TYPED]** the `using-alloy` skill now prefers a rejection run, `run { valuation and not rule }
  expect 1`, over the `expect 0` form, which scope starvation fakes. Supersedes the `expect 0`
  reading of `str-refuted-shapes-are-inhabited-runs`.
- **[TYPED]** `311u` and the crosscheck commentaries may wait for a later pass; not a ruling.
- **[TYPED]** readability and factoring are wanted where they cost no correctness; the human read
  `str-truth-predicates-are-transcriptions` as duplicating each English sentence twice.
- **[TYPED]** `git mv` of the note to `specs/311-identity.assay.md` is done (§ 3.2 item 1).
- **[TYPED]** `form-residue-lives-in-a-sub-section` is NACKED as literally put, kept in spirit:
  incompletely translated lines mid-arc are expected; the close owes that every part of 311 is
  exactly one of (1) non-normative, (2) normative and mechanical, or (3) normative and genuinely
  unmechanizable. That tri-partition is the whole arc's work order.
- **[TYPED]** an unmechanizable sentence: lean toward SPLITTING the mechanizable content out, per
  `plans/30Z`, only where both resultant meanings are carried faithfully and the input prose is
  unambiguous; otherwise residue, hopefully temporary.
- **[TYPED]** the human's hope is that mechanizable and unmechanizable CONCEPTS are already
  separate in 311: the algebra, the objects, and the relations are mechanical; the unmechanizable
  is a small set of outlier concepts that belong in sections of their own. Firewall the two by
  section wherever the text allows; a residue sub-heading is a fallback, not the plan.
- **[TYPED]** the two tables of 311 § 5 are non-normative.
- **[TYPED]** nit: the ledger's order items and every reference use slugs.

### § 5.2-dispositions

- `dsp-translation-is-the-one-copy` (supersedes the reading of
  `str-truth-predicates-are-transcriptions`) — a mechanized sentence of 311 lives ONCE, as the
  translation block of the fence that carries it (`form-translation-reuses-311-sentences`); a
  truth predicate's translation IS that sentence. No second quotation anywhere. The only
  duplication the praxis forces is fence beside translation, which is the firewall's diff surface
  and stays.
- `dsp-runs-always-carry-a-scope` — assay routes any command with a scope clause to `laws.als`,
  a `_premise` twin inherits its check's scope, and an UNSCOPED `run` is emitted as a corpus
  outcome (`assay.rs`, the `corpus.push` arm), which would invert its meaning. Every run here
  carries a scope; twins omit it and inherit.
- `dsp-rejection-runs-expect-one` — a kill, a refuted shape, and a hole's witness are each a
  `run { … } expect 1` at a scope that seats the witness, never an `expect 0`.
- `dsp-register-and-panels-in-a-later-pass` — the `311u` entries as inhabited runs and the § 4.2
  register's mechanical twins come after the tri-partition, unless a kill of a law needs one.
- `dsp-firewall-by-section` — expected mechanical: § 0's law, § 1's objects, § 2's relations,
  § 3.1 to § 3.3. Expected normative-unmechanizable, to be homed in non-mechanical sections under
  `<!-- normative -->`: the boundary (§ 4.1), the register (§ 4.2), and the sentences that name
  acts outside the model (the vouch tier, the standup `witness()`, the integrity plane, the
  flag's consumption policy, the aid attribution of § 3.5). A sentence found unmechanizable
  inside a mechanical section moves only where its meaning is unambiguous; else it stays and is
  counted at the close.
- `dsp-parse-sparingly-in-main-context` — the conductor runs `--parse` at most once per act and
  fixes only what is obvious; every solver fight goes to the builder, after the raw translation.

### § 5.3-encoding-decisions

Each is an encoding of 311's letter, not a reading of its design; where a choice of reading was
unavoidable it is flagged `ask-` in § 5.5 and taken by default as stated.

- `enc-levels-keys-and-worlds` — `abstract sig mLevel {}`; `sig mKey extends mLevel { value: one
  Shword, scheme: one mScheme, shape: lone mShape, parent: lone mLevel, reaches: lone mReferent
  }`; `abstract sig mWorld extends mLevel {}`; `sig mRoute extends mWorld {}`; one world atom per
  `:root` shape (311 § 2.2 "for a :root shape, the world is the store"; § 3.2 "two mKeys of that
  shape meet there and compare as siblings"), so a root key's chain ends at its shape's world and
  the walk needs no special case. An mKey's mValue is the shell word assay minted.
- `enc-world-stratum` — `sig mReferent { holds: set mReferent, owns: set mReferent }` with
  `owns in holds`. "Identified in a store" is `holds`; "by the store's own construction" is
  `owns` (a store re-presenting another's things holds without owning). `reaches` is one or none
  (§ 2.2 "or none"). No mState, no mValue beyond the key's word, at this stage.
- `enc-statements-are-declarations` — every `:relation`, warrant, closure, and supply of § 1 and
  § 2 is a subtype of the shared `Statement`, carrying its subject as fields; each has one truth
  predicate, `true_<Species>`, transcribing its defining sentence into the world stratum; the
  engine reads only `InForce` statements; the laws premise `everyStatementInForceIsTrue`.
- `enc-shape-is-the-matched-path` — `sig mShape { scheme: one mScheme }`; a key with no shape
  matched no path and reads unknown from that level; equal mValues of one mScheme match one
  shape (§ 1.6, "a function of the mKey's own bytes"); a shape with no in-force
  `:identified-in` is scoped in the mRoute (§ 1.6).
- `enc-parent-is-a-field-fed-by-supplies` — `SuppliesParent` statements are the seats; a key's
  `parent` is the one in-force supply where exactly one exists, the mRoute of its vantage where
  its shape has no `:identified-in`, its shape's world where the shape is `:root`, and none
  otherwise (an unknown link). Two supplies that disagree leave no parent; the refusal law is
  `act-yields-and-seats`'s.
- `enc-height-by-cardinality` — a level's height is `#(l.^parent)`; two levels align when
  their heights agree; the lockstep of § 3.2 walks aligned pairs from the terminus. Integers are
  the shared `bookScope`'s bitwidth; a chain longer than the bitwidth admits is impossible at the
  scopes used.
- `enc-walk-is-the-fqk-derivation` — `fun walk[x, y: mKey]: one Answer` is § 3.2's
  mFullyQualifiedKey mDerivation with its four answers; `compare()`, the coherence over
  mDerivations, is defined in `act-vantage-observer-cells-composite-lends` and reads `walk`.
- `enc-one-instance-is-one-atom-for-now` — until § 1.10 is mechanized, "one instance" is one
  atom; the sentinel's and the ambient instance's sharing widens it later, and the lock shows
  what that moves.
- `enc-stores-well-founded-is-a-premise` — "a store is never among its own contents" (§ 3.2, the
  one-top way) is a named predicate in the premise of the laws that rest on it, never a fact.

### § 5.4-order-of-acts

Supersedes § 3.2's numbered list; item 1 is done. One act at a time, each with its fences, its
translations, its twins, and one kill.

- `act-chain-and-worlds` — § 1.1, § 1.3, § 1.4, § 1.6, § 1.8, § 2.2, § 2.4, § 3.1: the objects,
  the chain, the termini, `identity()` as the chain. Inhabit; the chain-terminates fact.
- `act-warrants-and-store-closure` — § 1.5's two warrants and `:root`, § 2.3.
- `act-the-walk-and-its-laws` — § 3.2 and § 0: `walk`, the four answers,
  `law_same_is_sound`, `law_disjoint_is_sound`, `law_nobody_spoke_declines`; twins; kills; the
  first books (a hardlink; two files in one filesystem; nested pid namespaces).
- `act-yields-and-seats` — § 2.1, § 1.6's three seats and the disagreement refusal.
- `act-readset-writeset-sparing` — § 2.5, § 2.6; USER_STORY stage 5 and the loop image as books.
- `act-traversals-and-regions` — § 1.7, § 2.9, § 2.10.
- `act-invalidation` — § 3.3.
- `act-vantage-observer-cells-composite-lends` — § 1.10, § 2.7, § 2.8, § 1.9, § 2.11, § 3.4,
  and `compare()` over mDerivations.
- `act-committee-and-attribution` — § 3.5.
- `act-tri-partition-accounting` — § 4 and every remaining sentence homed as (1), (2), or (3);
  the lock written; the builder dispatched on the human's typed ack.

### § 5.5-questions-for-the-human-non-design

Taken by default as stated; a word overrides.

- `ask-aliases-nothing-else-world-reading` — § 2.3's "by the store's own construction" is
  encoded as ownership (`enc-world-stratum`), since the strong reading (nothing held here is
  held anywhere) contradicts § 2.3's "whether another store aliases this one is not claimed".
  The encoding decides which worlds a red can show; it is a world-modelling choice, not a
  change to any rule.
- `ask-route-for-any-path-without-identified-in` — § 1.6's "A shape with no `:identified-in` is
  scoped in the mRoute" is applied to every matched path, warranted or not. The narrower
  reading (an undeclared path reads unknown) is noted and not taken; § 1.6's sentence is
  explicit.
- `ask-parse-once-per-act` — NACKED **[TYPED]** 2026-09-28: no `--parse` in the conductor's
  context at all; the builder owns every parser and solver run. `dsp-parse-sparingly-in-main-context`
  is superseded.
- `ask-unique-name-ranges-over-the-scheme` — § 1.5's "within one mParent, one mReferent has one
  mKey" is encoded with the one mKey ranging over the lookup (every mKey of the scheme), not over
  the warranted shape alone. Under the shape-only reading, two shapes of one mScheme could name
  one mReferent under two mValues and the two-tops way of § 3.2 would read them DISJOINT with
  every statement true. The scheme-wide reading is the one under which § 3.2's own way is sound;
  the shape-only reading is noted and not taken.
- `ask-may-read-is-declared-per-key` — § 2.5's may-read is "a template the mSite or environment
  fills"; the fences hold the filled form, one `DeclaresMayRead` per (mKey, entry), spoken by
  the mSort's owner, with `ClosesMayRead` per mSort. A template mechanism is not modelled.
- `ask-both-exclusion-readings-are-mechanized` — § 2.6's container exclusion (`312ch` item 11)
  is encoded twice, `writesetAgainst` (excluded as built) and `writesetAtTest` (excluded at the
  last step only); `law_sparing_is_sound` is stated over the first, which spares more, and
  `law_exclusion_readings_agree` asks whether they ever differ. No reading is chosen.
- `ask-world-relations-for-effects` — the sparing law's world side is three uninterpreted
  relations: `mReferent.affects` (a write to one changes the mState of the other),
  `World.lineWrites` (what a line in fact writes), and `VerdictFact.dependsOn` (what the
  measured answer in fact depended on). Each truth predicate of § 2.5 and § 2.6 is stated over
  them, transcribing its sentence; nothing else reads them.
- `ask-rule-three-deferred` — an entry given whole (§ 2.6 rule 3) waits for the traversal act;
  until then every entry names its mReferent and nothing beneath, stated in § 2.6's commentary.
