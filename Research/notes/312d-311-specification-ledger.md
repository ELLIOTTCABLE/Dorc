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
- `ask-rule-three-deferred` — RESOLVED the same sitting: rule 3 is mechanized
  (`act-traversals-and-regions`); a whole-marked entry seeds its finished enumeration or, absent
  one, every mKey its region covers.

## § 6-state-at-the-close-of-raw-authorship

2026-09-28, tip `e9de23bd`. The raw translation of `notes/311` into `specs/311-identity.assay.md`
is COMPLETE as an authored document and UNCOMPILED: no `--parse` and no solver run has touched it,
by the human's nack of `ask-parse-once-per-act`. Twenty-eight `alloy` fences, thirteen checks,
sixteen runs, no `sh` fence yet. The lock does not exist. Nothing below is green or red; every
result is owed to the builder phase, which needs the human's typed ack to dispatch.

### § 6.1-what-each-section-is-now

Mechanical (a fence with its translation): § 0.2, § 1.1, § 1.2, § 1.3, § 1.4, § 1.5, § 1.6,
§ 1.7, § 1.8, § 1.9, § 1.10, § 1.11, § 2.1 to § 2.11 (every one), § 3.1, § 3.2, § 3.3, § 3.4,
§ 3.5. Normative prose only (`<!-- normative -->`, the checker cannot reach it): § 0 (the two
sentences of the law that are its own terms of refutation), § 0.1 (the two strata), § 1.1.1
(state, value, and what an mReferent survives), § 1.2.1 (the strangers case), § 1.5.1 (the
token over time; the closure sentences the region test consumes are now ALSO mechanized in
§ 2.9, so § 1.5.1's copies of them are a duplication to strike at the accounting), § 1.6.1 (the
seats' attribution; further routes), § 1.7.1 (the lookup body's read set), § 1.8.1 (the mTopic
and mDerivations), § 1.10.1 (placeholder and witness), § 1.11.1, § 2.1.1 (the chain terminates;
the decline), § 2.6.1 (the finished definition and the worlds), § 2.9.1 (indexicals and the
floor), § 2.10.1 (invocation and refusal), § 3.1.1, § 3.2.1 (what the answers mean to their
consumers), § 3.3.1, § 3.4.1 (guest-dependent lends), § 3.5.1 (composites and attribution),
§ 4.1, § 4.2. Commentary: § 5, § 6, the Conventions, every arity/declared-by/default/consumer/
danger summary, every example, every false friend. The tri-partition accounting of 311's
sentences one by one (`act-tri-partition-accounting`) is NOT done; the section map above is its
first draft and the duplication in § 1.5.1 is its first finding.

### § 6.2-encodings-added-since-the-strategy

Each an encoding of 311's letter; where a reading was unavoidable it is flagged as an ask.

- `enc-cells-are-scheme-less-keys` — `mKey.scheme` is `lone`, `mKey.cellSort` is `lone`, exactly
  one of them is set; a cell key has no shape, one mKey per (cell mSort, mParent instance), and
  the one-level rule reads two cell keys of one mSort as SAME (§ 1.9 "a cell's identity is its
  mParent's plus its mSort").
- `enc-catalog-sort-is-declared` — `DeclaresCatalogSort` (S's owner) says which mSort a secondary
  mScheme's mKeys are looked up in; 311 § 2.1 presupposes it ("the mEntryChain's instance for
  that mSort") without spelling it. `ask-catalog-sort-declaration`.
- `enc-vantage-is-the-entry-chain` — `sig mVantage { route, enteredFrom, through, ambient:
  mSort -> lone mKey }`, every mKey `at: one mVantage`, an emitted mKey at its input's vantage; a
  vantage entered through a wrapper holds the lent instance, else the caller's instance under
  the sentinel and the flag, else none; its mRoute is the caller's under the sentinel and the
  flag, else another. "One mPlaceholder" is one atom per vantage per mSort by construction.
- `enc-lends-truth-is-routing` — a lend is true when every natural mKey of the lent mSort under
  the wrapper reaches what a route through the lent instance passes to; the sentinel's truth is
  the same for every unlent mSort against the caller's instance, plus equal-spelled route-scoped
  mKeys inside and outside reaching one mReferent.
- `enc-engine-is-a-speaker` — `one sig engine extends Speaker`; its one standing vouch
  (`engineVouchIsTrue`, § 1.10) is a conjunct of `everyStatementInForceIsTrue`.
- `enc-flag-is-a-lone-sig` — `lone sig RiskFaultlessSkips`; `flagged` gates `sparedBy` and the
  sentinel's inheritance; nothing else reads it.
- `enc-compare-over-derivations` — `sameBy` is the walk, a mCorrespondence, or a composite's
  parts SAME by the walk; `sameClosure` is its reflexive-transitive closure; DISJOINT composes
  through SAME; a contradiction (SAME by closure and DISJOINT between closure members) reads
  UNKNOWN. `ask-contradiction-reads-unknown`: 311 says "refuse both"; a refusal is not one of the
  four answers, so the safe bottom stands in, and the refusal itself stays normative prose in
  § 3.2.1 and § 3.5.1.
- `enc-composite-parts-walk-only` — `compositeSame` reads `walkOfKeys` on the parts, not
  `compare`, because `compare` reads `compositeSame` and Alloy refuses the recursion; a part SAME
  only by a mCorrespondence does not make the composites SAME. `ask-composite-parts-by-walk`.
- `enc-alias-closure-instance-is-every-key` — `alias nothing-else` at a level is true when no
  other mKey of the level's mScheme reaches its mReferent, over every mKey in the world; 311
  scopes it to "the instance the lookup ran in". The encoding is stronger than the sentence
  (fewer worlds satisfy the claim), which is the direction that can HIDE a counterexample.
  `ask-alias-closure-instance-scope`.
- `enc-observers-are-the-vantage-ambients` — a fact's `underObservers` is every ambient instance
  of its topic's vantage; `topicObservers` drops those the sort declared independent of.
- `enc-support-functions` — § 3.5's attribution is `chainSupport`, `sameSupport`,
  `disjointSupport` (sets of statements) and one law, that a two-tops DISJOINT rests on one
  mScheme owner's `:guarantees-unique-name` declarations.
- `enc-finished-record-per-sort-and-shape` — `FinishesEntailment { finishedSort, finishedShape:
  lone }`, since a cell has no shape; 311 says "that cell's mSort and shape".
- `enc-open-read-set-is-uninterpreted` — `one sig Engine { lookupReadSetOpen: set mScheme }`
  (`30Z:hole-three-kinds-and-their-idioms`, the missing-definition idiom); "any write
  invalidates a mResolution whose read set is open" is its only axiom.
- `enc-traversal-members-are-keys` — an unclosed lookup's mParent-Catalog joins its traversal
  only where it is an mKey; a mRoute or mWorld catalog contributes nothing and the unclosed
  traversal blocks DISJOINT anyway.
- `enc-levels-include-the-yield-chain` — `levelsOf[x]` is every mKey on x's yield chain plus
  every level of its identity's mFullyQualifiedKey, since a natural mKey's own lookup is what
  crossed the routing mKeys.

### § 6.3-suspicions-to-let-the-solver-settle

Unwalked by the solver; each is a red the builder should expect and report, not repair.

- `sus-view-at-the-shared-level` — under `enc-world-stratum`, a store A that holds a sub-store Y
  and also holds, as a borrowed re-presentation, a thing Y owns, with A's own mScheme keying both
  under `:guarantees-unique-name`, may let the two-tops way read the direct mKey and the mKey
  through Y as DISJOINT with every statement true, since § 3.2 asks `:aliases-nothing-else` only
  of stores strictly below A. If `law_disjoint_is_sound` is red with that shape, it is a novel
  hole to hold, not a truth predicate to tighten.
- `sus-unique-name-across-shapes` — if the scheme-wide reading of
  `ask-unique-name-ranges-over-the-scheme` is nacked, `law_disjoint_is_sound` goes red on two
  shapes of one mScheme.
- `sus-exclusion-readings-differ` — `law_exclusion_readings_agree` is expected red; a red there
  is `312ch` item 11 made concrete and is a finding for the design sitting.
- `sus-cross-world-may-read-entry` — `312ch` item 6's collapse is reproducible through rule 4
  with an entry keyed in another mWorld; `law_sparing_is_sound` stays sound but the premise twin
  may find no world where such an entry coexists with a sparing.
- `sus-natural-disjoint-needs-the-catalog` — `naturalKeyAnswer` reads DISJOINT for two natural
  mKeys under one mParent-Catalog with `:guarantees-unique-name`; the truth predicate's "within
  one mParent" for natural keys is the catalog, so this should be sound, and the twin tells
  whether the world seats it.

### § 6.4-what-the-builder-may-and-may-not-do

The Fable-tier builder, on the human's typed ack, in its own worktree off `ai/main`:

- Reads first, in order: root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`; `specs/AGENTS.md`;
  `plans/30Z`; `notes/30Y`; the `using-alloy` skill and `examples.md`; `specs/311-identity.assay.md`
  whole; `specs/shared.assay.md`; this ledger's § 5 and § 6. Nothing under `notes/30Ya-*`.
- May: fix syntax and type errors in the fences without changing meaning; rename a field that
  Alloy finds ambiguous; add a `_premise` twin or a scope where a twin is unsat for want of atoms
  and say why; add `expect` marks that record what the run showed; write the first `sh` books
  from the cases 311's examples name (two files in one filesystem; a hardlink; nested pid
  namespaces; USER_STORY stage 5), with load files of claim atoms; run `--write` and commit the
  lock; report every red with its counterexample rebuilt by hand.
- May not: add, remove, or reword a fact, a truth predicate, a law's body, or a translation
  sentence to make anything green; widen a scope as a fix; choose between two readings; close a
  hole. A red that is a design question becomes `pred hole_<slug>` plus its witness run and the
  premise exclusion of the affected check, and nothing else (`30Z:hole-fence-never-fill`); a
  moved translation is a finding reported upward.
- Reports: the lock as written; each red with its shape and which of § 6.3 it matches or does
  not; each edit it made beyond syntax, with the sentence it believes unchanged.

### § 6.5-next-acts

`act-tri-partition-accounting` (the sentence-by-sentence account of `notes/311` at `7a63bae6`
against the specification; § 6.1 is its draft; the § 1.5.1 duplication is its first strike);
the builder phase on the human's ack; the books; then the adversarial crosscheck under the
human's further instruction.

## § 7-the-unacked-readings-are-a-failure-mode

**[TYPED]** 2026-09-28: every place where a soft or ambiguous sentence of 311 was turned into a
hard mechanization is marked INLINE in the specification, adjacent to the fence-and-translation
pair it softens, as a commentary paragraph beginning `UNACKED READING, temporary` and naming
its slug here. Each such mark says the reading is the conductor's, has not been acked, is not
authoritative, and stands only until 311 has been completed, committed, and run under Alloy and
the reading is acked or replaced. This is a FAILURE MODE, recorded so it is not repeated: the
correct move on an underspecified sentence is a held hole, `pred hole_<slug>` with its witness
run and the premise exclusion of the affected checks (`30Z:hole-fence-never-fill`), or residue
as normative prose, never a reading hardened into a fence with the prose walked back beside it.
No successor and no builder mints another such mark; the ones below are the whole set, and each
is deleted when its reading is acked (the mark goes, the fence stays) or replaced (the fence
moves and the lock shows it).

The sites, each `312d:` slug being the mark's citation:

- § 1.1: `ask-aliases-nothing-else-world-reading`, `ask-world-relations-for-effects` (the world
  stratum itself).
- § 1.3: `enc-primary-yields-into-its-sort`.
- § 1.5: `ask-unique-name-ranges-over-the-scheme`.
- § 1.6: `ask-route-for-any-path-without-identified-in`, `ask-mismatched-supply-reads-unknown`
  (a supplied instance not of the declared mSort leaves no mParent; 311 does not say what a
  mismatch does).
- § 2.1: `ask-catalog-sort-declaration`.
- § 2.2: `enc-primary-owner-is-sort-owner` (a fact that the primary mScheme's owner is the
  mSort's owner, reconciling § 1.6's "P's owner" with § 2.2's "the mSort's owner").
- § 2.5: `ask-may-read-is-declared-per-key`.
- § 2.6: `ask-both-exclusion-readings-are-mechanized` (and "spared only when" read as the
  decision).
- § 2.8: `enc-observers-are-the-vantage-ambients`.
- § 2.9: `ask-alias-closure-instance-scope` (and "of D's mSort" read as every member's mSort).
- § 2.11: `ask-composite-parts-by-walk`.
- § 3.2: `ask-contradiction-reads-unknown`, `enc-one-instance-is-one-atom-for-now`, and the
  laws' premise (every statement in force, not the support).
- § 3.4: `enc-lends-truth-is-routing`, `enc-vantage-is-the-entry-chain` (a non-inheriting
  vantage holds a different mRoute atom).
- § 3.5: `enc-support-functions`.

## § 8-the-builder-dispatched

2026-09-28, over `bbe7fe72` and the ledger commit that carries this section, the human present
and the dispatch typed. One Fable-tier builder, in its own worktree on `ai/312d-mechanize-311`,
holding the § 6.4 remit sharpened into a brief (inline in the dispatch, not durable): parse;
inhabit; the twins; every law, each red rebuilt by hand and classified under `30Z` § 2.5 and
matched against § 6.3; holes only under `30Z:hole-fence-never-fill`; one kill per green law,
through an `allInForceTrueExcept` restructuring of the premise checked `old iff new`; the lock
written and committed; books last, and only after each book's world is put to the conductor. The
builder runs every parser and solver run; the conductor runs none and takes no adjacent work
while it runs. Nobody mints another UNACKED READING mark.

## § 9-the-builder-phase-first-two-turns

State at `3d697bd6` on `ai/312d-mechanize-311` (fourteen commits over `3a5c81f7`; worktree
`C:\Users\ec\Sync\Code\Dorc\.claude\worktrees\agent-a03007ade2e0378bf`), 2026-09-28: the document
parses; the lock exists (38 rows, eight accepted reds) and reproduces under `--check` with the
runner's caps raised (`--timeout 120 --batch-timeout 4000`, about 25 minutes per pass); the
completion gate's `assay-lock` step fails only because its default 540 s batch cap reaches half
the commands. Not folded. The conductor's turn-1 rulings were given in chat and applied by the
builder; the turn-2 rulings are in chat and not yet banked.

Findings of turn 1, every one a transcription defect of the conductor's raw pass and none a
change to 311's meaning:

- `fnd-none-equality-class` — four fences compared two possibly-empty `lone` expressions with
  `=`, which is true when both are empty: the § 1.9 cell fact (forbidding any two ordinary mKeys
  one mParent, which starved every soundness twin and made four greens vacuous),
  `withinOneParent`, `lifecycleInvalidatedBy`, `naturalKeyAnswer`. **[TYPED]** 2026-09-28, the
  human recalling the skill's own trap entry: the spelling is the overlap, `some a & b`; `=`
  between two possibly-empty expressions only where the translation says "or both empty". The
  builder swept every fence under that rule and found seven sites in all.
- `fnd-walk-laws-quantified-natural-keys` — the two walk laws ranged `walk` over every mKey
  where § 3.2 walks by identities; restated over `walkOfKeys`, which covers the primary case as
  the identity of a primary key and the natural case through `true_DeclaresYields`.
- `fnd-touches-traversal-mis-scoped` — a block-bodied quantifier scoped the catalog-given-whole
  disjunct inside `some m: crossed[l]`; re-parenthesized to the translation's reading.
- `fnd-placed-in-whole-sort-dead` — `placedIn[k, mSort]` compared one sort against the whole
  set and never fired; now every `looked-up-in` record of k.
- `fnd-compare-translation-cost` — every `compare`-routed command failed to translate at scope
  6, about ×5 per atom; memoized as `Tables` (`walkTable`, `compareTable`, no multiplicity on
  the answer column, one defining fact each, agreement with the functions checked at scope 4
  because the un-tabled form does not solve at 5); the tables tax every command about 17 s.

Holes surfaced by the solver, each minted `hole_<slug>` with a sat witness and premise
exclusions on the laws that showed it, each a design question for the human's sitting, none
closed:

- `hole_cell_keys_under_same_parents_reach_differently` — § 1.9 gives a cell's identity as
  (mParent, mSort) engine-side; no sentence ties a cell key's mReferent to its mParent's
  mReferent and its mSort. Excludes the SAME laws.
- `hole_world_scoped_top_aliases_into_a_store` — the two-tops and one-top ways with a leaf top
  scoped in an mWorld and no store on that leg: an mReferent inside a closed store is also
  reached directly from the world. The § 5.1 OPEN thing's-end cell's first non-declining case.
  The mRoute variant rests on `ask-route-for-any-path-without-identified-in`; the mRoot variant
  also on `ask-aliases-nothing-else-world-reading`, since an mWorld owns nothing in this encoding
  where § 2.2 says the world is the store. Excludes the DISJOINT laws and, by evidence, sparing.
- `hole_composite_keys_with_same_parts_reach_differently` — § 2.11's twin of the cell hole.
  Excludes the compare laws.
- `hole_unclosed_traversal_without_a_key_catalog` — a route-scoped lookup with no emitted member
  has an empty mTraversal in the fences; 311 covers it by the engine's vouch of the mRoute within
  a span (§ 1.10.1), which is prose. Excludes the unstale law.
- `hole_natural_key_catalog_off_the_route` — no world sentence says a natural mKey's supplied
  mParent-Catalog instance is on the route to its mReferent (`true_SuppliesParent` speaks for
  mKey-Primaries only). Excludes the unstale law. Conductor's candidate sentence for the sitting,
  unacked: the catalog seat is true when the mKey's mReferent is one a route through the
  instance's mReferent passes to.
- Left red at the close of turn 2: § 2.9 step 3's second form concludes DISJOINT for a leaf pair
  whose `compare()` is UNKNOWN (two parentless cell keys of one mSort reaching one mReferent; a
  `looked-up-in nothing-else` vacuously true), shown by the region and sparing laws; a hole is
  the turn-3 ruling's holding pattern, and the sentence 311 lacks (step 3 presupposes step 1
  settled that x is not D) is the sitting's.

Kills: four sat (`natural_same` by unique-referent, `natural_disjoint` by unique-name, `same` by
unique-referent, `nobody_spoke` by unique-referent); `two_tops` killable only by deleting its
speaker fact; `different_sorts_never_same` unkillable by construction and kept as a tripwire.

The builder reports one direct message from the human, verbatim in its supplement, asking for a
tooling-chafe section as a first-class product; § 10 is that section. The conductor did not see
the message and lists it here as reported.

## § 10-tooling-chafe-the-builders-register

The builder's observations, as delivered (its grades; the conductor's notes marked), for the
assay owner to lift into `30Yc`'s chafe register. Nothing semantic was weakened for any of these.

- Coverage that narrowed, so each is visible: `law_sparing_is_sound` at `4 but 4 Int, 10 Claim`
  because its writeset closure does not translate at 6; the tables' agreement check at 4, since
  at 5 it translated (67 s, 974k clauses) and did not solve in 400 s; `disjoint`, `compare_same`,
  `compare_disjoint`, `two_tops` translate in about 18 s and do not solve in 120 s CPU at 6 (all
  no-counterexample at 5; `two_tops` no-counterexample at 6 in 95 s alone); five holes narrow
  seven laws, the praxis as designed.
- Runner, lock, gate: 38 commands need about 25 minutes under the 120 s per-command cap, so the
  540 s batch default leaves the tail `not-run`, the lock mismatches, and the gate's `assay-lock`
  step has no override (suggested: a per-document cap spelled beside `bookScope` and copied by
  assay, or a batch cap derived from row count times the per-command cap); a `timeout` row is
  time-dependent, so the ratchet is not deterministic across machines or loads (`two_tops`
  no-counterexample alone and `timeout` in the batch; the unstale twin 82–108 s; `law_same`
  53–60 s; suggested: record the phase, still translating against translated with clause counts,
  in the lock, and a deterministic budget beside seconds); write-then-check is two full passes for
  one lock; `--parse` launches a JVM under preflight and the machine lock, about 30 s, some
  twenty-three times this arc.
- Assay's compilation shape against Alloy's inlining: Alloy inlines every call and the only
  sharing device is a table field defined by a fact, and every law shares one `laws.als`, so
  every command pays every table's fact (a per-law module would let cheap laws stay cheap and
  allow per-law caps; -GUESS on assay's cost); a table on a `one sig` adds a column, so a `meet`
  table is arity 5 and refused at 107 atoms, and arity-4 tables for `regionTest` and the writesets
  made translation worse; an unscoped `run` becomes a corpus outcome silently, only the `_premise`
  naming keeping twins out (suggested: refuse an unscoped run not named `<existing check>_premise`);
  the corpus book with zero claims still runs, 14 s per pass.
- Instances and the reading loop: every instance is one JSON line with every relation, empty ones
  included, about 8 kB with escaped newlines; the harness's Grep omits long lines and its sed
  splitting was refused, so rebuilding by hand from that text was the costliest step of the loop
  (suggested: `--instances-dir` with one plain-text instance per red, empty relations omitted,
  one relation per line); scratch bisecting needs module copies beside a probe (suggested:
  `--probe <file>` against the current out directory); rows need grep patterns (suggested:
  `--summary` printing name, result, wall per line).
- Alloy traps the document contained that no tool caught: the empty-equality trap at seven sites
  (an advisory compile lint on `=` between two possibly-empty expressions outside a `some` guard
  would have flagged all seven; conductor's note: an instrument that reports, never a fence,
  and the human's to want); a branching function applied to a set argument, accepted silently
  (a line for the skill's trap list); a declared `fun` result multiplicity is documentation only,
  which is why the tables stay unbounded; three syntax shapes the parser refuses loudly (a
  block-bodied quantifier followed by `or`; juxtaposed formulas in a comprehension body;
  `some disj p, q: S` with no body).
- The harness around the tools: a `for` loop over a helper script, a variable-heavy path
  assignment, and `wsl --cd … -- mise trust` were refused as too complex to verify, so every
  solver call became literal-path lines and the WSL gate leg never ran; every solver run, parse
  included, holds the one machine lock, so with eleven deliberate 120 s timeouts per full pass
  most of the wall-clock was waiting in series.
- The builder's overall grade: the tooling did its job, every refusal loud, every row
  reproducible under a raised cap, no verdict dependent on an option; the chafe is cost and
  ergonomics, with the one correctness-adjacent exception that time-based `timeout` rows are not
  deterministic across machines.

## § 11-turn-three-and-the-handover

Tip `50e53398` on `ai/312d-mechanize-311` (twenty-one commits over `3a5c81f7`; only the
specification and the lock touched), 2026-09-29. Turn 3 applied the fourteen rulings the
conductor gave in chat after turn 2: `coveredBy` and `seed` per element (a branching function had
been applied to a set argument, a second mechanical slip class beside `fnd-none-equality-class`);
`true_ClosesLookedUpIn` guarded inside its antecedent; `routesAreStrict` as a named premise on the
untouched-route law, never a fact; the exclusion-readings law at four atoms and ten statements
beside the sparing law; the sixth hole. The lock: 39 rows, no counterexample row. Witnessed and
killed greens: `natural_same`, `natural_disjoint`, `same`, `nobody_spoke`. Witnessed greens:
`sparing` (at `4 but 4 Int, 10 Claim`) and `different_sorts_never_same`. Seven checks translate
and do not solve within 120 s CPU at their scopes (`disjoint`, `compare_same`, `compare_disjoint`,
`two_tops`, `region`, `unstale` at nine statements, `exclusion_agree` at four and ten), each
no-counterexample at five where measured; they are the coverage frontier and stay as recorded.
Holes held: § 9's five and `hole_region_closure_with_unknown_leaf_pair` (§ 2.9 step 3 concludes
DISJOINT for a leaf pair `compare()` reads UNKNOWN or KNOWN_UNSPOKEN, step 1 never having settled
that x is not D; on the region, sparing, and untouched-route laws). The builder's three judgement
calls beyond the rulings' letter, accepted by the conductor: the guard inside the antecedent (a key
reaching nothing keeps a vacuously true closure, as the sentence says), one translation sentence
for `routesAreStrict`, and the hole's widening to KNOWN_UNSPOKEN. The gate's `assay-lock` step
stays red at its 540 s batch cap. Not folded; the fold is the human's.

Correction to § 6.5: the baseline for the tri-partition accounting is the note's content at the
parent of the rename commit, `1af7e0d9^:Research/notes/311-identity-and-relation-model.md` (the
note at `8b7ad973`), not `7a63bae6`; edits landed after that tip (`312cg` § 24 to § 30).

**[TYPED]** 2026-09-29: the first builder is wound down at its context cap. A new clean-context
Fable builder is authorized; it is allowed an Opus under it, at its option, for churn-y,
low-logic mechanics, under the standing order that an Opus never edits Alloy text in any
meaningful way, only mechanics and general mess that is not thinking about correct
specification behaviour. Every prior instruction stands. The successor is dispatched from
`50e53398` rebased over `ai/main`, as `ai/312d-mechanize-311-b`, with the order: the
tri-partition accounting against the corrected baseline; the books, each world put to the
conductor first; the final lock and report, with its own tooling-chafe section.

## § 12-the-accounting-first-pass

The successor's turn 1 (branch `ai/312d-mechanize-311-b`, the predecessor's twenty-one commits
rebased over `ai/main` at `1995fa4a`; no edit of its own yet), 2026-09-29. Baseline: the note's
content at `1af7e0d9^` (`a08eb53c`; last changed at `8b7ad973`), 1339 lines; § 5 and § 6
byte-identical between baseline and specification. Sentence-level over § 0 to § 4: 770
sentences. First-pass categories, the successor's judgement over every row: commentary 207;
plain prose judged normative 66 (57 of them the § 4.2 register entries, which sit outside the
blockquote); mechanical with a named carrier 237; carried structurally by typing or absence 33;
in a translation block but carried differently or by a definition nothing consumes 53;
normative residue 152; residue duplicating a mechanized sentence 18; absent 4. The reverse
account over 304 headed sentences: 169 verbatim, 57 merges, 11 moves, 2 rewords, and the
additions of the mechanization (13 law sentences; 17 truth-predicate sentences; 3
world-stratum sentences under the § 1.1 marks; 15 structural facts the encoding needs; 4
premise definitions; 2 probes; the scope and plumbing sentences; 8 marked readings). The full
table is the successor's scratchpad `tri-partition.tsv` with `additions.tsv`, uncommitted.

Readings the raw pass hardened WITHOUT a mark, found by the accounting (no mark is minted for
them, per § 7; they are the sitting's, listed here so they are not lost): a `:primary-of`
mScheme may carry a yielding shape whose mKeys are then natural (`isPrimaryKey` demands a
non-yielding shape; 311 § 2.1 has `:yields` on a secondary mScheme's shapes) · a supplied
mParent instance must be an mKey of the parent mSort's PRIMARY mScheme, where 311 § 1.6 says
"of one of the mParent's mSort's mSchemes" (narrows the worlds; the books' path-in-directory
case hits it) · the composite replaced "a provider-supplied identifier" in § 3.2's derivations
and a verdict fact's topic is one mKey, so a topic with two mKeys is representable only through
a mCorrespondence · invalidation reads the writeset with every container contributing, there
being no read mKey to exclude against · the region test's first form demands the lookup's
closing act (stronger than 311's first form; differs only where the unclosed traversal's catalog
is an mWorld) · a mTraversal's order is not held (no consumer in 311 reads it). Two more were
repaired toward the letter on the conductor's ruling: the entry chain is a seat that refuses on
disagreement (§ 1.6), and a lifecycle write is to a mRoot-adjacent mKey, not any mWorld-adjacent
one (§ 3.3). Disagreements recorded, no edit: § 1.9's "exactly one mKey" against the fence's "at
most one" (an existence fact would be a generator the scopes cannot seat; the prose is not
softened); § 2.1's "may supply" against § 3.1's "supplies" in the baseline itself; the may-write
translation naming "the verb's author", a role the fences lack. Definitions 311 states whose
consuming law is a coverage gap: `sameTopic` (no fact-transport law), `compareAt`, `places`,
the two mParent views, the support functions. Two false units of the `placedIn` class repaired
on ruling: `readsetMemberIsTop` walked the catalog chain where 311 says the mFullyQualifiedKey;
`compositeMayRead` was defined and read by nothing where 311 says the composite's set is the
union. The conductor's rulings on the fifty-one items were given in chat and are being applied
on the successor's branch; the tri-partition's final counts follow its turn 2.

## § 13-the-close-of-the-mechanization-arc

**[TYPED]** 2026-09-29: the arc closes here, both conductor and builder near their context caps.
No fast-forward of `ai/main`, no ceremony, both builder worktrees left in place; a rewound
conductor takes the fold and the ceremony before the next dispatch. Nothing new is taken on.

State a successor inherits. The work is on `ai/312d-mechanize-311-b` at `a212975c`, twenty-nine
commits over `ai/main` (twenty-one the first builder's, eight the successor's), tree clean,
worktree `C:\Users\ec\Sync\Code\Dorc\.claude\worktrees\agent-a70159e5c19f3608a`; the first
builder's branch `ai/312d-mechanize-311` at `50e53398` and its worktree
`...\agent-a03007ade2e0378bf` are superseded by the rebase and kept only as evidence. `ai/main`
itself carries only this ledger's commits above `3a5c81f7`. The fold is a rebase of the successor's
branch over `ai/main` (expected clean) and a fast-forward; the completion gate's `assay-lock` step
is red for the batch-cap reason alone.

The successor's eight commits: four repairs toward 311's letter (the entry chain as a refusing seat,
§ 1.6, the one edit that changed a translation sentence and is ruled; `readsetMemberIsTop` over the
identity chain; `compositeMayRead` consumed by rule 4 and the closure truth; lifecycle as
mRoot-adjacent), four translation sentences added, thirteen baseline sentences restored verbatim
into normative blocks with `sortsOf` and `partsOf` deleted, nine residue duplicates struck under the
one-copy rule, and the fifty-seven register entries moved inside § 4.2's blockquote byte for byte.
No red appeared; every touched command reproduced its lock row. The lock (39 rows) is the first
builder's, untouched; `--check` at `a212975c` mismatches on exactly one row,
`law_disjoint_by_two_tops_rests_on_one_scheme_owner`, recorded `timeout` and now solving in 95 to
105 s against the 120 s cap, its hash unchanged: the time-dependence of § 10, not an effect of any
edit. Left as is on the conductor's ruling; a re-lock waits on a deterministic budget in the runner.

The tri-partition at the tip, over the 770 baseline sentences: commentary 211; mechanical with a
named carrier 259; structural 33; residue 222; carried differently or by an inert definition 36,
every one a recorded item (the marks, § 12's found readings, the inert definitions); residue copies
kept whole for an unmechanized half 9; plain prose judged normative 0; absent 0. The tables are the
successor's scratchpad `tri-partition-v3.tsv` and `additions-v2.tsv`, uncommitted; whether they
become a durable is the human's.

Not started, banked as the next arc's first step: the books, four worlds drafted by the successor
and confirmed by the conductor, no load file written. B1 `two-files-one-filesystem`: tessa owns
`sm.Path`, `sm.File`, `sm.Filesystem`; the inode mScheme `:primary-of sm.File`, its shape
`:identified-in sm.Filesystem` with both warrants; the path shape `:yields` it; the filesystem's
shape scoped in the mRoute and `:aliases-nothing-else`; the declaration seat supplies `fs_1`, a
primary key; `/srv/a` and `/srv/b` yield two inodes, DISJOINT; a second path to the first inode,
SAME. B2 `a-hardlink`: as B1 with the path shape carrying neither warrant (the honest world under
§ 2.3); two paths yield one inode, SAME; the region test against a sibling directory UNKNOWN; the
book with a false unique-name in force is the attributed wrong-DISJOINT world, a second book.
B3 `nested-pid-namespaces`: pia's pid mScheme `:primary-of sm.Process`, `:identified-in
sm.PidNamespace` with both warrants; the namespace mScheme's two shapes, `nested` and `initial`;
no `:aliases-nothing-else` on the inner namespace; pid 1 inside and pid 4821 outside reach one
process: UNKNOWN without the container manager's `:corresponds`, SAME with it. B4
`user-story-stage-five`: as USER_STORY spells it, the index cell given whole against the four facts
meets each at `host` with no shared key space and reads KNOWN_UNSPOKEN, which never spares; the
honest outcomes are `todo[this]` until the sitting, the acked cost of `311t` § 14 and the § 4.2
register's first entry; the book that spares is the one whose apt describer names the list files.

Owed to nobody, listed so a successor sees the whole field: the human's sitting over the six holes
(§ 9, § 11), the found readings and recorded disagreements (§ 12), and the seven UNACKED READING
marks (§ 7); the tooling items of § 10 for the assay owner, the deterministic budget first; the
adversarial crosscheck over the platform, under the human's instruction; the `AGENTS.md`
opaque-review gate, which binds a Fable conductor at the end of this work and needs the human's
typed ack to dispatch.

## § 14-the-rewound-conductors-standup-and-the-fold

A rewound conductor, 2026-09-29, holding the ledger, the specification at `a212975c` whole, both
shared halves, the lock, and the `using-alloy` skill with its examples. Typed this sitting and
banked; the conductor's proposals of the same sitting stay in chat until the human reacts.

- **[TYPED]** no opaque-review this arc: nothing in it touches memetic-hazard material. § 13's
  last line is superseded on that point.
- **[TYPED]** the fold: acked and done. `ai/312d-mechanize-311-b` rebased clean over `ai/main`'s
  two ledger commits and `ai/main` fast-forwarded to `7177558a`; the tree is clean. Both builder
  worktrees (`agent-a03007ade2e0378bf`, `agent-a70159e5c19f3608a`) remain in place, unpruned.
- **[TYPED]** the ledger carries no question for the human, ever: it is for historical surgery and
  failure investigation and is not read by a human. A question is put in chat or nowhere.
- **[TYPED]** the books are acked as the arc's main destination.
- **[TYPED]** the human is on the fence about dispatching a builder with a remit to make the
  solving faster (the fear: quietly vacuous results) and leaves the call to the conductor.
- **[TYPED]** the human is tempted to switch builders to Opus, three serial Fable builders being
  a first for the project, and to lean on a Fable-tier adversarial review at the end.
- **[TYPED]** the world stratum has not been explained to the human and is owed in chat, next
  turn, on the human's ack; nothing about it was ever "put to" them.
- Correction to § 13: "the seven UNACKED READING marks" is wrong. The specification carries
  fourteen marks (§ 1.1, § 1.3, § 1.5, § 1.6, § 2.1, § 2.2, § 2.5, § 2.6, § 2.8, § 2.9, § 2.11,
  § 3.2, § 3.4, § 3.5), about nineteen readings between them; § 7's site list is the accurate one.
- The successor's accounting tables survive, uncommitted, at the session scratchpad
  `…\Temp\claude\C--Users-ec-Sync-Code-Dorc\542b14fc-6170-4a07-bdec-8925ebc76c29\scratchpad\b\`
  (`tri-partition.tsv`, `-v2`, `-v3`, `additions*.tsv`); the directory is ephemeral.
- Coverage read off the lock at the fold: of thirteen checks, six no-counterexample and seven
  `timeout`; kills exist for four laws only (`natural_same`, `natural_disjoint`, `same`,
  `nobody_spoke`); the other nine checks, `sparing` and `different_sorts_never_same` included,
  carry no kill. No `sh` fence exists.

## § 15-the-planning-sitting-before-the-next-dispatch

2026-09-29, the same rewound conductor, design and planning only; the human present and
reading. The human's word at the close: "I see no logical holes, pseudo-ack." That is not a
ruling on any item below; every item stays at the grade it carries. The conductor is to be
rewound before any dispatch, so this section is written at the resolution a successor needs to
resume without re-deriving. Two turns are QUEUED by the human and were deliberately not spent
here: (1) an explanation, in chat, of what the conductor calls "the world stratum" (the human
does not know what the term means; it was never put to them); (2) a full turn theorizing how to
speed the hot loop mechanically, with the incremental-lock idea vetted before any of it is handed
to the assay builder.

### § 15.1-the-standup-as-put-and-the-humans-corrections

- The conductor's standup separated the "hole count" into three populations: genuine 311 holes
  (the six `hole_` predicates; +SURE two are 311's own, the cell key's mReferent untied from its
  mParent's and its mSort, and § 2.9 step 3 presupposing step 1 settled x ≠ D; two are twins of
  the first; ~SUSPECT two are artifacts of the conductor's readings,
  `hole_world_scoped_top_aliases_into_a_store` and
  `hole_unclosed_traversal_without_a_key_catalog`); the invented world stratum (the fourteen
  marks nearly all hang on the four world relations plus `reaches`, which 311 never held and the
  § 0 law cannot be checked without; the process defect is that this one decision was made
  piecemeal and marked fourteen times instead of put once); and process slips (the `=`-on-empty
  class, a branching function on a set, six readings hardened with no mark, the LLM Alloy error
  rate amplified by authoring 2,500 lines with no parse).
- Two platform defects named: the encoding is expensive (`meet` closes `^parent` per pair,
  `height` is an integer, `sameClosure` is a closure over a comprehension, the `Tables` memo
  taxes every command ~17 s) so the DISJOINT and compare laws time out at six and the burndown
  cannot re-run them; and no book exists, though books are the stronger instrument and would have
  caught the slip classes in minutes.
- **[TYPED]** "~1,200 words" was a typo for lines; the human's mental model of this corner is
  effectively the two tables of § 5, and that simplicity may be why the mechanization is full of
  holes.
- **[TYPED]** the ledger carries no question for the human (banked § 14); the world stratum is
  explained in chat, next, on the human's ack.

### § 15.2-scope-down-proposed-and-retracted

- The conductor first proposed lowering the scope of the seven timing-out commands to where each
  solves (`30Z` § 2.8), with the twin kept satisfiable as the tripwire.
- **[TYPED]** the human's objection, accepted in full: a timeout is not a weaker measurement, it
  is the absence of one; the corpus should not be deterministic under a short hot-loop cap; there
  will be a CI or full-gate budget that is the "official" one, and hot-loop caps must be shorter,
  so per-item scope caps cannot be hardcoded to speed the hot loop.
- RETRACTED (conductor): scope-down. The corrected model: SCOPE is part of the CLAIM ("no
  counterexample in worlds of size N"); the CAP is a BUDGET for finding out. The lock holds the
  official-budget result; the hot loop runs what it can and records "not measured here"; a
  per-item scope drops only if a command cannot finish at the official budget either, which
  nobody has measured. The praxis's "lower a bound only for cost" presumed one budget.
- First act on the seven, therefore: measure once at a generous cap (~900 s per command, about
  two hours worst case, unattended), report only, no lock write; learn which are unaffordable at
  six against merely slow. The lock-format consequence (a budget column; `timeout` never equal
  to a result; the one row that already flipped between machines) is the queued hot-loop turn's.

### § 15.3-the-builder-tier-decision

- **[TYPED]** the human's framing: a builder that only reports reds to the conductor is no
  better than a Fable, because the conductor's context is the bottleneck (rewinds start near
  700–900k of 900k); the three viable cells are "Opus does it all and asks occasional
  questions", "Fable does it all and asks occasional questions", and "a Fable pushes an Opus";
  NOT "the conductor pushes an Opus". The human's hope: Opus can churn against reds semantically
  now that the document is mostly written. On typing it out the human leaned to Opus, wanting
  tokens for a substantial adversarial review, and asked the conductor to be very sure and argue
  both sides.
- Both sides as argued. FOR Opus: the remaining work enumerated (measuring the seven; kills by a
  fixed pattern with the species chosen from the law's premise; the four books whose worlds and
  answers § 13 already drafts, leaving transcription plus small-instance diagnosis, with a
  self-contradictory world caught by assay as an unsat book run; the split if taken, mechanical
  against a spelled seam) is about three quarters churn; the `30Z` § 2.5 triage is a procedure;
  the failure that bit this arc (readings hardened, prose walked back) was a Fable-tier discipline
  failure, not a capacity one; the Fable budget buys most at the adversarial review. FOR Fable:
  a new counterexample on `disjoint` or `compare_disjoint` at the official budget lands in the
  subtlest part of the model (two-tops, one-top, `legStores`, the store closure) and a mis-triage
  either mints a spurious hole or reports a real hole as a slip and stalls; an Opus inventing a
  book world from the spec alone must be fluent in a world stratum documented only in truth
  predicates and marks, and a subtly wrong world yields a GREEN book that means nothing, which no
  twin catches because a book's run is its own twin; each Opus question costs conductor context.
- The call (conductor; the human's pseudo-ack): ONE Opus, with two clamps and one valve. Clamp
  one: it never edits a fence body, a world fact, an outcome line, or a translation; its only
  permitted edits are a `hole_` predicate with its witness run and premise exclusion, a kill run,
  a `Scope:` paragraph, and the books' plumbing lines (`#}` map lines, load files). Clamp two: a
  red is reported with its instance rebuilt by hand and its `30Z` § 2.5 class named, never
  repaired. The valve: exactly one class escalates to the conductor, a NEW counterexample on a law
  that today reads `timeout`; that is the arc's purpose firing and the right place to spend
  conductor context. Against the green-book risk: the brief carries the four worlds and every
  expected answer as § 13 drafts them, so the load-bearing lines are the conductor's in the brief
  and the Opus transcribes; a book whose world it would have to invent is one it may not write.
  Grade: sure of the direction; the residual risk sits at the valve, and the valve's cost is
  bounded by how often the arc does its job. "What remains is mechanical" as first put was
  overstated; "mostly mechanical with one rare deep-triage class" is the accurate form.

### § 15.4-the-311-313-seam-argued-and-deferred

- **[TYPED]** the human, on the document's length, leans toward splitting the semantic material
  into two specifications, 311 and 313, and asked for a clear, simple, internally consistent
  seam and for a PRODUCT-SIDE or EPISTEMIC reason the dependency can only ever run one way, so a
  mechanical split never forces an incorrect design-level division later. Two concerns typed:
  (1) correctness, whether properties weaken by not being executed as a unit every time, and
  lesser, an LLM reading only one document before editing (mitigable by the mechanical net and
  `AGENTS.md`); (2) wallclock, likely better when editing one, with algorithmic duplication when
  both must build.
- The seam (conductor): `compare()`. 311 keeps everything `compare()` needs and nothing it does
  not: the objects (§ 1.1 to § 1.6, § 1.8, § 1.9, § 1.10), the identity relations (`:yields`,
  `:primary-of`, `:identified-in`, `:root`, `:aliases-nothing-else`, `:parent`, `:corresponds`,
  composites, lends and the vantage), `identity()`, `compare()` with its tables, the walk laws,
  and the committee law over compare's support; its one export is the four-answer table. 313 takes
  the consumers: the verdict fact and its topic (§ 1.11; § 2.8's observer qualifier), may-read and
  the readset, may-write and the writeset, the entailment and finished record, the traversal and
  region test (§ 1.7's traversal half; § 2.9), `:places` (§ 2.10), invalidation (§ 3.3), sparing
  and its laws. 313 opens 311. +SURE (every fence read for it) no identity-side definition reads
  an interference species, so the dependency is strictly one-way and Alloy enforces it after the
  split (311 never opens 313, so a stray reference fails to parse). Four definitions are mis-homed
  today and move with the cut: `sortOfKey` (defined in § 2.9, used identity-side everywhere);
  `traversal`, `EmitsCrossed`, `ClosesTraversal` (§ 1.7, whose resolution half is identity and
  traversal half routing, so the section splits); `VerdictFact`; `flagged` (read by both; declared
  in 311). Books exercising `compare()` alone belong in 311; books exercising sparing in 313.
- The product-side argument (conductor; the human saw no logical hole): the direction is the
  human's own typed formulation, `311t` § 15, "identity is minting; freshness is holding".
  Identity asks who hands out a name and what it reaches, answered from speech about lookups and
  stores, and must be answerable before any line runs because both its consumers need it at plan
  time. Interference asks where state is kept and what a write reaches, and is DEFINED OVER minted
  names (every may-read and may-write entry is an mKey), so it presupposes identity, while a
  `resolve()` never consults a may-read set (§ 2.4 and § 2.5 in words: the mParent answers
  identity, entries answer interference, a parent instance is no entry). The refuted shapes fence
  it: `311u:refuted-identity-as-a-table-against-axes` and `311u:refuted-stored-in-as-one-relation`
  are the designs where holding leaked into minting, both killed on ops cases. The temporal half is
  § 3.3's charter sentence, "the engine withdraws authority and never computes the successor
  identity": book order enters only to WITHDRAW an identity answer, never to produce one. In the
  fences this already holds: `compare()` reads no `Line`, no `above`, no fact, no writeset;
  `compareAt` (313's) is `compare()` met with staleness. The one future test of the seam is
  § 1.10.1's "one placeholder per unwalled span" (a span names lines), but a span is "no
  invalidating line between", which is staleness, so it lands on 313's side of the meet as
  `compareAt` does; mechanizing it as an input to timeless `compare()` would violate § 3.3's
  sentence and the seam would refuse it loudly. Grade: ~SUSPECT sufficient forever; +SURE it is
  311's own principle and not a mechanical convenience. Restated: 311 is the timeless identity
  algebra with no book order in it; 313 is what the book's lines do to facts.
- Correctness under the split (conductor; +SURE of the direction): 311's checks are STRENGTHENED,
  since every statement species shares one `Claim` scope (at `for 6` at most six statements exist
  across twenty-eight species today) and removing the twelve interference species from 311's
  universe seats more identity statements at the same scope; 313's laws lose nothing, seeing all of
  311's definitions and facts through `open`, and the one genuinely joint property, sparing, lives
  there. Costs: `cost-cross-spec-open` (-GUESS assay does not yet implement one document opening
  another's species and claims modules; the fixture shows only `open shared`; if so, a small assay
  item and a hard prerequisite); `cost-slug-rename` (every `311:2.5-`, `2.6-`, `2.9-`, `2.10-`,
  `3.3-` citation across the corpus becomes `313:`, per the spec's own § 6.1 rule, a mechanical but
  real pass over the crosscheck ledgers especially); `cost-own-probes` (313 needs its own
  inhabitation runs); the § 4.2 register and § 5 tables split by which document each "Here" cites.
- **[TYPED]** the human's second thought: two documents are almost guaranteed slower or worse,
  since under one net cap each region gets half (one region exhausted while the other finishes
  early, the slack no longer shared) or under a cap each the runtime doubles; lean to one file and
  one shared cap until fully firmed.
- Conductor's answer: the dichotomy does not hold at the command level (per-command caps decide
  `timeout`; a split moves no row's result; 313's commands cost what they cost today and 311's
  get cheaper; total work is the same or less; the batch cap is a ceiling not a cost, and "two
  batches, two caps" is a spelling problem for the tooling), but the conclusion holds for two
  other reasons, so the split is DEFERRED: the world stratum is unacked and the split would assign
  its relations across two documents (`holds`, `owns`, `passes` to 311; `affects` to 313), so a
  re-cut of the stratum could re-cut the split; and the corpus-wide slug rename is churn to pay
  once, after firming. The seam is recorded here and cuts later at no extra cost.

### § 15.5-wallclock-observations-banked-unvetted

Proposed by the conductor in chat, NOT vetted, and the human queued a full turn to vet the
incremental-lock idea before it reaches the assay builder; nothing here is to be handed to a
builder until that turn.

- The human's observed 45-minute single tool call was almost certainly `--write` followed by
  `--check`, two full ~25-minute passes for one lock, under the machine lock. A pass is
  thirty-nine commands serial, one JVM each; seven burn the full 120 s cap by design (fourteen
  minutes of nothing); every command pays the two memo tables' defining facts (~17 s).
- Candidate assay items, none touching Alloy semantics: an incremental lock, each row keyed by a
  hash of the command's text plus its module's text plus every module it transitively opens, so
  `--check` and `--write` re-run only changed rows (with books in their own modules a book edit
  re-runs one module); `--write` reporting the diff it would have checked so write-then-check is
  one pass; the runner owning K parallel children within one run under a shared RAM budget (the
  machine-global lock stays; preflight bounds RAM); a batch cap derived from row count times the
  per-command cap; a per-command passthrough (`mise run alloy -- --command <name>` exists, assay
  does not expose it, the first builder asked for exactly that as `--probe`). ~SUSPECT the
  incremental lock alone turns the book loop from ~25 minutes to under two.

### § 15.6-the-first-dispatch-as-sketched

Pending the human's typed ack after the two queued turns; a rewound successor writes the brief.
One Opus, its own worktree off `ai/main` (`cba61fdb` at this writing), brief carrying: the safety
block; step-zero and step-one per `spike/CLAUDE.md`; the two clamps and the valve of § 15.3; the
four worlds of § 13 with every expected answer; the no-subagent clamp. Order of work: the
official-budget measurement of the seven timing-out commands, report only, no lock write; the
books, each transcribed from the brief's world, run, and any red reported with its instance and
its § 2.5 class; kills for the unkilled green laws (`sparing` first). Excluded from the brief:
any scope edit; the split; any lock-format change; anything the two queued turns decide.

## § 16-the-hot-loop-sitting-and-the-assay-handoff

2026-09-29, the same conductor, the first of the two queued turns. **[TYPED]** the ask, sent
mid-turn: dig deeply into the performance-against-correctness mechanics; theorize how to speed
the hot loop, which fixes belong in the tooling (assay and the runner), and what hot, gate, and
CI should each contain. CI is not stood up; GitHub's free runners were floated (the human
recalled a ~10-minute limit; the conductor's +SURE-ish correction: six hours per job, but ~7 GB
RAM), with a cheap Vultr runner, the human's dedicated server, or the sibling Mac as
alternatives; the how of CI is set aside as standup work. **[TYPED]** a tool call to read
assay's CLI surface was NACKED: theorize from context. So every claim below about assay's
current flags is from `notes/30Y`, the § 10 chafe register, the lock read at `a212975c`, and the
`using-alloy` skill, and is graded accordingly. **[TYPED]** at the close: ledger; then a HANDOFF
for the assay builder, written as a peer document and not a brief (the human's goals first,
verbatim; the conductor's analysis included whole but softened, its goals never encoded as
demands; the builder is Fable-class and knows Alloy better; the conductor's uniquely valuable
content is the Dorc, project, and spec context); the assay builder runs in parallel in its own
worktree under the human's separate prompting; from here on the conductor and the human move
forward on the CURRENT tooling. The handoff is the uncommitted root file
`_tmp-assay-hot-loop-handoff.md`.

### § 16.1-result-against-non-result-and-the-locks-blind-spot

- A command yields a DEFINITE result (`sat`, `unsat`, `counterexample`, `no-counterexample`) or a
  NON-RESULT (`timeout`, in translation or in solving; `not-run` under the batch cap). A definite
  result is a fact about the model at that scope and is budget-independent. A non-result is the
  absence of a measurement at that budget.
- The lock at `a212975c` treats `timeout` as a result: row-for-row equality, so `timeout` matches
  `timeout`. Cosmetic consequence: rows flip with machine speed (one already has). Unsound
  consequence (+SURE): a `timeout` row can hide a red indefinitely: a later edit that gives
  `law_disjoint_is_sound` a real counterexample at six, on a machine that still times out at
  120 s, matches the lock and passes the gate. The seven timeouts ARE the DISJOINT and compare
  core, so this blind spot sits on the arc's purpose.
- Schema the conductor proposes per row: `scope` (exists), `result`, `budget` (the per-command
  cap the result was measured under, as a tier name or seconds), `size` (Alloy's primary
  variables and clauses after translation, deterministic given the jar), and for a timeout
  `phase` (`translating` or `solving`); `premise` derived from the twin row at report time, not
  stored.
- Asymmetric matching, the whole fix, no scope touched: `rule-definite-beats-nonresult` (a
  definite result found at any budget may be written; a non-result never overwrites one and never
  matches one; it is reported "unmeasured at this tier"); `rule-counterexample-always-mismatches`
  (a counterexample at any budget against a locked `no-counterexample` is a mismatch);
  `rule-size-tells-machine-from-encoding` (a timeout against a locked definite result at the same
  or lower budget is machine noise when the clause count is unchanged, warn and pass, and an
  encoding regression when it grew, mismatch, a finding); `rule-timeout-rows-are-owed` (the lock
  may carry a timeout so the document is not blocked, listed as residue of a different kind from
  an accepted red).

### § 16.2-the-incremental-lock-and-its-attack

- The claim: a row is skipped on `--check` or `--write` when it has a definite result and its KEY
  matches. Key = hash over every input the child JVM receives: the text of every generated module
  the command's module transitively opens (`assay.als`, `shared.als`, `species.als`; for a book
  also `words.als`, `claims.als`, the book module), each with its command blocks stripped; the
  command's own generated text; the row's scope string; the Alloy jar's digest (mise-pinned); the
  runner's option set (solver, skolem depth, symmetry breaking, whatever the Java passes to
  `A4Options`). Computable by assay without parsing Alloy: it generated the modules, wrote the
  `open` lines, and emitted the command blocks so it knows their extents.
- Why laws and books decouple: `laws.als` opens `species` only (`30Y` § 2.3); `words.als` and
  `claims.als` are opened only by book modules and the corpus book. A book edit changes a book
  module, `words.als`, and `claims.als`, and no law's key. A hole predicate or fence edit changes
  `species.als`, hence every key, correctly. A new kill run changes its own row only. This is what
  turns the book loop from a full pass into one module (~SUSPECT from ~25 minutes to under two).
- Why skipping is sound: same module texts, command, scope, jar, and options give the same
  Kodkod translation (deterministic) and, under SAT4J, a deterministic sequential solver, the
  same verdict; a parallel solver could vary the INSTANCE shown, never the verdict, and the lock
  records verdicts. Definite results only; timeouts are always re-attempted at their tier.
- The attack (under-invalidation is the only failure that matters): JVM inputs enumerated as
  module files, command, scope, jar with Alloy's `util/*` library inside it, runner options; the
  runner reads no environment for semantics; hole predicates, `Tables` facts, and shared halves
  are module text; an assay codegen change alters module text so every key changes and one full
  pass follows, self-correcting; a stale module in the out directory is inert unless opened, and
  assay should clean the directory on compile; a hand-edited lock row is trusted until the
  from-scratch tier, which ignores keys (the belt to this brace); the current sixteen-hex hash is
  fine for change detection, a truncated SHA-256 costs nothing. Residuals: -GUESS whether any
  runner option varies by platform (the WSL leg choosing a different solver would make the lock
  per-platform; the option set must be pinned in one place and hashed); the runner must select a
  command by NAME not position (the lock's row identity is module plus name); assay must refuse
  two commands of one name in one module.
- **[TYPED]** "defend": stripping commands rests on "a command never constrains the model". The
  defense as given: a command is a query (`run {P} for S` asks for facts ∧ P; `check {P} for S`
  for facts ∧ ¬P), and nothing in the grammar lets a command introduce a usable name, add a
  constraint another command inherits, declare an atom, or open a module; facts alone remove
  worlds and cannot sit inside a command block. Two channels by which one command's text DOES
  reach another row, both covered: (1) a malformed command makes the whole module graph fail to
  compile, so every row would be `error`; therefore keys are consulted only AFTER the graph
  compiles, a precondition, and a compile failure trusts nothing; (2) assay-generated commands
  embed authored formulas (a book line's check carries every line above as premises; a book's run
  conjoins every outcome; the corpus book's run conjoins every unscoped authored check; `run
  bookScope {}` is copied onto other rows as their scope), covered because the key hashes the
  GENERATED command text and the row's scope string, not the authored `#=` line. Checked also:
  `expect 1` disables symmetry breaking per command, which prunes isomorphic instances and never
  changes a verdict; unused signatures consume scope in every command regardless. Residual:
  -GUESS Alloy 6.2's grammar has no command-level partial-instance or bounds annotation (Forge's
  `inst`; Kodkod's API); were there one it would still be local to its command.
- Superseded: the § 10 per-law-modules suggestion buys nothing once commands are stripped from
  the module hash, and would not touch the `Tables` tax (species facts are global to every module
  that opens species).

### § 16.3-the-tiers

- Pre-commit unchanged: assay's solver-free lints, no JVM, under three seconds.
- `tier-hot`: `--only <command>` and `--module <book>` by name (the praxis's smallest run, `30Z`
  § 5; -GUESS assay does not expose it though `mise run alloy -- --command` does); incremental
  `--check` over changed rows only; cheap-first ordering by recorded `size`; a deterministic size
  threshold above which a row is `deferred`, never attempted, so the hot loop never produces a
  timeout; ~120 s per command; the 540 s batch cap kept for the harness's foreground window;
  exit 0 when every re-measured row matches, deferred rows listed.
- `tier-gate` (builder completion): incremental over changed keys; ~600 s per command; a batch
  cap derived from rows-to-run times the cap; background under the machine lock; rows still over
  the cap recorded `timeout` with `budget: gate` and `size`; exit under the asymmetric rules (a
  size-grown timeout fails as a cost regression; same-size warns).
- `tier-official` (nightly or CI): from scratch, keys ignored; 900 to 3600 s per command; K
  parallel children with K times the heap cap under preflight's RAM bound (the machine lock
  protects against two tasks, not one task's children; K of four plausible on this box; ~7 GB on
  GitHub caps K at one or two); writes a candidate lock and prints the diff against the committed
  one; a human commits it; larger scopes as separate commands belong here (`30Y` § 3.1), which
  needs assay to accept an alternate scope clause per tier, a later design item.
- The invariant across tiers: a lower tier never makes the lock worse (it writes only definite
  results and never overwrites one with a non-result); only the official tier makes it whole.

### § 16.4-what-belongs-to-whom

- Tooling candidates, none touching Alloy semantics, in the conductor's value order for the
  mechanization side's loop: T3 `--only`/`--module`, cheap-first ordering, deterministic deferral
  · T1 the incremental key and skip · T2 the schema and asymmetric match · T6 `--write` printing
  its diff so write-then-check is one pass (the 45-minute call) · T7 a plain-text instance per red
  in a directory (the builder's costliest loop step was 8 kB JSON lines) · T5 parallel children
  under the RAM budget (official tier) · T4 the derived batch cap with `deferred` never mismatching
  · T8 refusing an unscoped `run` not named `_premise` (today silently a corpus outcome;
  correctness-adjacent) · T9 cleaning the out directory on compile.
- Spec-side, the conductor's and the human's, each a reword the lock must show moves nothing:
  S1 the two `Tables` defining facts become a predicate `tablesAgree` used as a premise by exactly
  the commands that read a table, so the ~20 commands that never read one stop paying ~17 s each
  (~6 minutes per full pass); meaning unchanged for readers and non-readers alike. S2, NOT
  proposed now: genuine cost restructurings of the walk (integers out of `height`; `meet` as a
  table rather than per pair), only after the official-budget measurement says which laws are
  unaffordable at six, each under `check { old iff new }`, each the human's call individually.
- Unsettled by the sitting: whether the seven are slow or unaffordable at six (the next
  dispatch's first item); the platform-option question; alternate scopes per tier.

## § 17-the-world-stratum-explained-and-the-law-defended-with-the-humans-corrections

2026-09-29, the same conductor, the second queued turn and the turn after it. The conductor's
two explanations are banked as delivered; the human's corrections sit beside them as **[TYPED]**
and bind everything after. Nothing here is ruled beyond those corrections.

### § 17.1-the-world-stratum-as-explained

- The one-sentence form: 311 is written from the engine's side and says what may be concluded
  from declarations; its § 0 law says those conclusions are never wrong while the declarations
  are true; checking that needs a subject for "true" and "wrong", which 311 deliberately lacks;
  the world stratum is that subject, invented by the conductor in the raw pass so the laws would
  have one, and every UNACKED READING mark is a place the invention forced a choice 311's English
  left open.
- The anchor given: `compare()` as a proof system (axioms the statements in force; rules the
  walk; theorems the four answers), the § 0 law as its soundness against a semantics, an Alloy
  `check` as a bounded search for a countermodel (premises hold, conclusion fails). Type-system
  soundness and alias analysis as the familiar instances.
- What was built: `sig mReferent { holds, owns, affects, passes }` (held = identified in, many
  stores per referent; owns ⊆ holds = by the store's own construction; affects = a write to one
  changes another's mState, directional; passes = a route passes through), `mKey.reaches: lone`,
  `VerdictFact.dependsOn`, `World.lineWrites`. Each truth predicate transcribes one 311 sentence
  into these; each law's conclusion reads them. No mState values, no time, no mValue content.
- The discipline: the engine's definitions never read a world relation; Alloy cannot enforce it
  (the fields sit on `mKey` beside fields the engine reads); § 0.1 states it as prose; a reviewer
  greps the engine's definitions for the seven world names.
- The direction of error: a truth predicate stronger than 311's sentence hides countermodels (the
  dangerous direction; the alias-closure mark is the instance); weaker yields noise reds; leaving
  `affects` and `passes` uninterpreted is the conservative posture and is the current state.
- The fourteen marks collapse to five questions on the vocabulary: `ask-holds` (identified-in as
  many-to-many between referents); `ask-owns` (own ⊆ hold; `:aliases-nothing-else` as "all I hold
  I own and nobody else owns", chosen over the strong reading because § 2.3 says aliasing by
  another store is not claimed); `ask-affects-and-the-chain` (directional; the may-read closure's
  exemption of the holds-chain, a reading without which every honest closure is false);
  `ask-passes` (one relation for traversals, places, and lends); `ask-the-actual-bits` (reaches
  one-or-none; dependsOn; lineWrites; no state, no time). The remaining marks
  (`ask-contradiction-reads-unknown`, `enc-support-functions`, the exclusion readings) stay their
  own questions.
- Consequence for books: `#=` world facts are written in this vocabulary and are the load-bearing
  lines, which is why the four worlds were drafted in advance.

### § 17.2-the-section-zero-law-defended-in-product-terms

- The claim as put: Dorc never removes a line on its own authority; every removal rests on
  statements named people made about their own tools, stores, or machines; the engine composes
  and adds no belief of its own; a wrong removal is therefore somebody's false sentence, and the
  receipt can point at it. Four grounds: agnosticism leaves no stronger promise available; the
  recovery story (`dorc why`) requires every wrong removal to have a wrong link, else
  IMPLEMENTATION's second sin has a non-empty class; each hat's liability is bounded only if no
  anonymous step sits between their sentences and the decision; it is the one soundness the frame
  problem permits, because the open-world residue is located entirely in authored "nothing else"
  sentences. Objections met: garbage-in (Dorc names which garbage and adds none; `311u` is the
  register of refused own-inferences); the flag (no skip is faultless: every survival rests on a
  named negative existential, false when wrong, its author unable to have known); the engine's own
  vouch (about sh, differentially tested); measurement (`271:rul-measurement-is-authorship`);
  "never" (applies to the composition, never the outcome); truism (`30U`'s free rider was a live
  violation; the six holes are candidate ones). Not claimed: correctness, safety, completeness,
  the removal of the frame problem, integrity, adequacy.

### § 17.3-the-humans-corrections-typed-and-binding

- **[TYPED]** NACK of "Dorc knows nothing about the world by design": Dorc DOES know things
  about the world by design, and the dividing line between engine-known and spoken is very
  precise because it yields work. Its current gloss, not its forever-totality: sh semantics
  (engine-known) against system, binary, and internet truths (spoken). The engine-known side owes
  differential tests, rich floors, cross-systems testing; the spoken side owes obsessive refag
  analysis, language design, speech and contract work. The line can move: the engine could one
  day model `sudo` in-engine, paying with differential and cross-platform tests on `sudo`.
- **[TYPED]** NACK of "the engine speaks": speech is reserved for what a user does under the
  contract. The engine computes. The engine's axioms are the complement of speech-axioms, the
  differentially-tested ones; in the spec they are axioms, never speech.
- **[TYPED]** careful with "no measurements": the point of separating measurement-that-is-speech
  is to RESERVE a slot for someday engine-measurements, Dorc-generated, no-refag, locked-down,
  referentially-tested probe components (some already on the table around the r26 orchestration
  work, where the engine and executor do actual things to actual systems). Such a measurement
  must never be mixed with user speech nor attributed to a user's fault.
- **[TYPED]** phrase the split as "the contract" against "the engine", the project's actual split
  of obligation: speech, from the perspective of owed work, is contract-precision work; the known
  world is engine-plus-tests work. Ack asked.
- **[TYPED]** "and we know which" is effectively the whole product: with a fully accurate map of
  claim to line to narrative, knowing which lines a net set of boolean conclusions disnecessitates
  is a strict subset of the whole narrative (the human's logic, to be checked); lean toward
  expanding the precise terminology to "contract + engine → narrative", ish. Logic to be checked;
  naming matters deeply.
- **[TYPED]** horizon: from a specification standpoint the horizon is mostly not thought or
  talked about elsewhere precisely BECAUSE the specification draws and maintains the line; the
  Alloy is suspected to be one of the only places it is explicitly drawn; the human has no
  picture of how, and asked for a strawman: how the spec mechanically says "TOCTOU is out of
  scope" to the adversary.
- **[TYPED]** the flag's wording is precise: what the engine computes, owes, and works over is
  ATTRIBUTION; the flag prices FAULT, in the who-took-a-wrong-action-that-could-have-gone-
  otherwise sense, not the moral one. A faultless skip has no wrong action because of epistemics:
  no world exists in which the attributable persons' set contains a solution that the contract
  owed them. Contract-net-epistemics: the contract decides what "reasonably knowable" means and
  horizons out the technically-knowable-in-forty-five-years; that contract shapes the
  specification as the precise line between attribution and fault; things horizoned out as not
  reasonably knowable are nobody's fault and are the domain of the faultless skip. "Faultless" is
  a precise mechanical mapping over attributions: `attributions.filter(within contracted
  knowability class)`, ish.
- **[TYPED]** extremely hard ack on refusing "never" for outcomes, and a HARD NACK on "usually" or
  any frequency terminology about outcomes at all: Dorc will usually be wrong; the value model is
  that OPS is usually wrong, most of ops is burning down wrongness and then gasping for air at a
  fragile steady state, and Dorc wants to burn down that wrongness faster and more precisely.
- **[TYPED]** otherwise ack. Ledger; opine in chat; a rewind toward dispatch follows.

## § 18-the-naming-sitting-the-vouch-correction-and-the-state-before-dispatch

2026-09-29, the same conductor, the last sitting before the rewind toward dispatch.

### § 18.1-typed-this-sitting

- **[TYPED]** a sibling conductor is live, with a builder applying performance enhancements to
  the tooling in its own worktree; the root `_tmp-assay-hot-loop-handoff.md` is its seed.
- **[TYPED]** the genus word for what an answer rests on is **Foundation**, not Ground; not to
  be bikeshedded.
- **[TYPED]** a design ruling, issued in chat despite the no-rulings posture of this arc: the
  engine is not "vouch". The spec text that said "the engine itself vouches" (and 311's own
  § 1.10 sentence it came from) was wrong, in the usual pattern of a precisely-defined contract
  term mis-reused under one of its fuzzy English false-friend meanings. The deviation from
  prose-311 is authorized; it is a one-to-one map and believed semantically a no-op; it is to be
  one standalone, complete commit, because it IS a normativity modification of 311's prose.
- **[TYPED]** both classes of rename are ledgered for the builder to handle; there is not enough
  to justify a separate rename round in-flow. This supersedes, for these renames, § 11's standing
  order that an Opus never edits Alloy text meaningfully.
- **[TYPED]** if any pure rename (no merge, no split, no semantics) is one the conductor is 100%
  sure conveys intent and truth better in a direct reading of the specification AS A
  SPECIFICATION (mechanical prose, not "better Alloy"), it may simply be made, no ceremony.
- **[TYPED]** the "and we know which" law belongs in the spec; the conductor's construction is
  not acked (the human is an Alloy novice) and is the builder's to implement, attack, and fight,
  IF necessary for 311's mechanization, else punted.
- **[TYPED]** a pass over the product core (README, DESIGN, IMPLEMENTATION) for product axioms
  with mechanical Alloy encodings that 311 could be built on top of is wanted LATER; first a
  no-semantic-change 311 rebuild; for now only product laws necessary to make 311's laws solvable
  are attacked.
- **[TYPED]** "statement" is nacked as the genus: the word must cover spoken-and-trusted-by-
  contract, measured-or-tested-at-runtime, and measured-or-tested-in-advance.

### § 18.2-the-vouch-correction-applied

Commit `43b8d4dd` on `ai/main`, the spec alone, standalone. One-to-one: `engineVouchIsTrue` →
`shellResolvesInTheAmbientInstance` (§ 1.10 fence; the first conjunct of `allInForceTrueExcept`
in § 3.2, left in place, the split being the builder's); § 1.5.1's commentary "the engine's own
vouch" and normative "The engine vouches for one lookup itself" → the engine's own axiom,
discharged by differential test and never spoken; § 1.10's commentary "The engine is a speaker
for its one vouch" → the engine does not speak, its one axiom is a premise of every law; § 1.10's
translation and § 1.10.1's normative sentence likewise; § 3.2's translation "and the engine's
vouch holds" → "the engine's axiom about where the shell resolves holds"; § 5.1's and § 5.2's
"the local-route vouch" → "the local-route axiom", with a kind `axiom` added to § 5.2's legend
(an engine rule about where the shell resolves, discharged by differential test; never a party's
statement). Also in the same commit, the conductor's judgment: `one sig engine extends Speaker {}`
REMOVED, as the reification of the same error (nothing named it as a speaker). That one piece is
NOT one-to-one: a `one sig` consumed a Speaker slot in every command's universe, so removing it
can only WIDEN worlds; the lock may move in the safe direction (a starved twin becoming sat; in
principle a check finding a counterexample a Speaker-starved universe hid, which would be a
finding). The builder's first `--check` shows whether anything moved. The author's vouch (the
verdict fact's speaker; `KNOBS:kCONTRACT-RUNGS`; "vouch-tier" in § 4.2) is untouched: that word is
the contract's and stays. The pre-commit lints ran quiet; no Alloy parse has been run on the
result, per the standing nack on parsing in the conductor's context.

### § 18.3-the-renames-for-the-builder

Both classes, for the next builder. Every reword is verified by the lock moving nothing (the
Speaker-slot widening above excepted, already landed).

- Pure renames the conductor is sure of as specification prose (the premise then carries 311's
  own normative sentence as its name): `storesAreWellFounded` → `noStoreIsAmongItsOwnContents`;
  `routesAreStrict` → `noRoutePassesThroughItself`.
- Reword, the axiomatic pair: extract `shellResolvesInTheAmbientInstance` out of
  `allInForceTrueExcept`; name the remainder `axiomaticByContract` (every spoken foundation in
  force holds; `everyStatementInForceIsTrue` retires); name the engine side
  `axiomaticByDifferentialTest` (today its one conjunct is the shell-resolution axiom; when the
  engine-known line moves, a new conjunct joins it, paid for by a differential test); every law
  and every kill premises BOTH by name, so each law's text shows the two obligation classes
  (whose work outside the spec discharges it). The conjunction is identical to today's, so
  nothing moves.
- Reword, the genus: `Statement` → `Foundation` (abstract, under assay's `Claim`), with
  `Spoken extends Foundation { speaker: one Speaker }` carrying every current species; `InForce`
  ranges over `Foundation`. A future `Measured extends Foundation` (engine measurements: per-world
  facts the engine established, nobody's speaker, Dorc's fault when false, trust discharged by the
  component's test) is NAMED ONLY WHEN INHABITED; § 0.1's prose gains the three categories
  (spoken foundations by contract; the engine's axioms by differential test; engine measurements,
  currently uninhabited) so nobody files the first engine measurement under `Spoken`. The
  shared half `specs/shared.assay.md` declares `Statement`/`Speaker`/`InForce`, so the genus rename
  touches it too. Engine axioms are not foundations-as-atoms (reifying them would spend `Claim`
  scope); "foundation" is the prose genus for all three.
- The support-form law (§ 17's "and we know which", mechanical): `(all s: support[answer] |
  holds[s]) implies answer is true`, with `holds` a disjunction over species since Alloy has no
  dispatch; strictly stronger than the in-force form (weaker premise) and the fidelity 311's § 0
  "behind it" asks for. Owed for fidelity, not solvability; sequenced after the world-stratum
  sitting and the books; the construction is the builder's to attack.
- `horizon_` as a first-class prefix beside `hole_`: same mechanism (a named premise exclusion
  with an inhabited `_witness` run), opposite lifecycle (a hole is deleted when answered; a horizon
  stays as the product's stated boundary). First instance: `sig OutsideWrite { hits: set mReferent
  }`, `pred horizon_writes_by_no_line { no OutsideWrite }` premised on the sparing law, with a
  witness run in which an outside write hits a spared fact's dependency, `expect 1`; translation
  "Dorc does not account for a write no line of the book performs". Additive; the conductor's
  authorship; after the stratum sitting. The recycled-key horizon waits on the temporal latitude.

### § 18.4-state-before-the-rewind

`ai/main` at `43b8d4dd` plus this ledger commit; tree clean; the fold done; both builder
worktrees in place; the sibling perf-builder live in its own worktree. Not blocking dispatch:
the world-stratum sitting (§ 17.1's five questions), the six holes (§ 9, § 11), the
found-without-a-mark readings (§ 12), and the fourteen marks (§ 7). The first dispatch is as
§ 15.6 sketched, plus § 18.3's two rename classes and the reword-verification discipline
(`check { old iff new }` where a body moves; lock diff zero otherwise), minus anything the two
queued turns have since decided (both are now spent: § 16 and § 17). The conductor sees no open
thread before dispatch.

## § 19-the-opus-sitting-strategy-and-the-books-authored

2026-09-29, a rewound conductor over `baff88de`, holding the ledger, the specification whole,
both shared halves, the lock's row names, and the `using-alloy` skill with its examples. Written
before any authoring, per the human's order; § 3, § 5, § 15, and § 18 stand except where a line
below supersedes one by slug.

### § 19.1-typed-this-sitting

- **[TYPED]** the conductor is the primary bulk-author, conductor, and adjudicator, with
  everything critical in context; it reasons, holds the source documents in sight, and writes
  the `.md`, `.assay.md`, and `.als` text; churny tool-calls and fiddling go to subagents.
- **[TYPED]** builders are Opus from here (the belief: the residue is mostly mechanical). The
  conductor is more critical of an Opus's results and claims than the ledger's history of
  Fable-tier builders warrants, especially where a builder tries to contradict or reverse a
  prior result.
- **[TYPED]** the order stands: main-context authorship first, worker churn and tooling later,
  the adversarial crosscheck last under further instruction.
- **[TYPED]** Fable-tier and Astra-tier subagents stay authorized at the value tier; every
  dispatch still needs the human's typed ack.

### § 19.2-standup-findings

- `fnd-hot-loop-lane-not-on-main` — `ai/r30-assay-hot-loop` carries four commits over `ai/main`
  (the Rust-side runner over one adapter child, comment-free one-item-per-line modules with a
  line-map sidecar, the whole-book conjunction, the stray-run and duplicate-label refusals); none
  is on `ai/main`. Assay on `ai/main` exposes `--parse`, `--parse-only`, `--check`, `--write`,
  `--out`; the runner exposes `--command`. The builder works on the current tooling and neither
  waits for nor merges the lane.
- `fnd-book-drafts-contradict-the-primary-yields-fact` (+SURE, by hand) — § 13's B1 scopes the
  filesystem's shape in the mRoute, and its B4 has every chain end at the mRoute; the fact of
  § 1.3 (`enc-primary-yields-into-its-sort`: a `:primary-of` scheme has some shape carrying
  `:identified-in` or `:root`) forbids a route-scoped `:primary-of` scheme, and an inode
  `:identified-in sm.Filesystem` needs the filesystem's sort to have a primary scheme
  (`supplyFits`). The honest world under the current text roots the boot (`:root` on the boot
  id's shape) and identifies the filesystem in it, which is `312cg` § 24 (a) and § 26. The
  authored books below take that world; the § 13 drafts are superseded.
- `fnd-in-force-is-book-global` (+SURE of the text) — the shared half defines `InForce` as
  `Line.speech` over every line, so a claim declared on a later line is in force for every
  line's outcome; per-line speech growth is invisible to 311's definitions, which read `InForce`
  only. Consequence for the books: a variant that adds one claim is a separate book, never a
  later line. A 311-side limitation to record for the design sitting, not a tooling item.
- `fnd-shared-world-objects-have-no-home` (+SURE of `30Y` § 2.3) — a book's world objects
  (speakers, sorts, schemes, shapes, keys, referents, the vantage) can be declared only in that
  book's `#=` lines: an `alloy` fence sends a non-claim `one sig` to `species.als`, which every
  law opens, so shared world objects would consume every law's scope and change every law's
  incremental key; and a load file carries claim atoms only, whose fields must name world
  objects. So every book is self-contained and the stdlib's world is written out in each,
  byte-identical where it is the same world. Recorded for the assay owner as the one tooling
  want this sitting: a module of world objects that book modules open and laws never do.

### § 19.3-book-form-decisions

Each is a form call within `plans/30Z` § 4 and `notes/30Y` § 2.4; none reads 311's design.

- `bk-self-contained-by-hash-equals` — every book declares its own world objects and its own
  claims as `#=` declarations; no load file at this stage (`fnd-shared-world-objects-have-no-home`).
- `bk-every-signature-pinned` — a book pins every signature to the union of its declared atoms
  and every world relation (`holds`, `owns`, `affects`, `passes`, `World.lineWrites`,
  `Engine.lookupReadSetOpen`, the flag, `no Wrapper` and `no CompositeKey` where none exists),
  the skill's some/disj idiom in the `one sig` form that assay's own departure prescribes
  (`30Z` § 5). A book is one world; its checks evaluate the model there, and its run is its
  witness.
- `bk-outcomes-reach-this-through-the-lines-own-speech` — an outcome names its line through the
  statements declared on it (`atLine.this` for a fact, `writeLine.this` for a may-write entry),
  never a line number; a sparing outcome names the writing line as `(the may-write atom).writeLine`.
- `bk-claims-declared-where-they-first-bear` — the stdlib's and each describer's declarations
  sit on the first line; a fact sits on the line that measures it; a may-write entry on the line
  that writes.
- `bk-expected-answers-are-the-conductors-hand-walk` — every expected answer in a book is the
  conductor's walk of the fences over the stated world, written before any solver run; a red
  is triaged under `30Z` § 2.5 as the walk's slip, then the fence's, and is never a reason to
  restate the world. The first line of an honest book asserts `everyStatementInForceIsTrue`, so
  the book's answers are answers about an honest world; a book with a false claim asserts
  `allInForceTrueExcept[that claim]` and names it, which is the attribution the § 0 law promises.
- `bk-the-path-catalog-is-the-filesystem` — the path scheme's mParent-Catalog sort is the
  filesystem in every book (a coarse describer's choice inside the book; a directory sort would
  add referents the books do not need). The engine's axiom then requires the filesystem to pass
  to its inodes in the world stratum, which the books state.
- `bk-one-book-per-claim-set` — a variant that changes what is in force is its own fence
  (`fnd-in-force-is-book-global`).
- `bk-books-sit-beside-what-they-exercise` — the walk's books are a sub-section of § 3.2; the
  sparing test's books a sub-section of § 2.6; each book's section carries a translation block
  saying the world and each line's expected answer, in the words of the world.
- `bk-scope-per-book` — each book carries `run bookScope_<book> {} for N but 4 Int` in an
  `alloy` fence beside it, N the book's mLevel count plus headroom for nothing (a pinned world
  needs none).

### § 19.4-the-four-books-and-their-expected-answers

Authored this sitting into `specs/311-identity.assay.md`; the worlds, in words, are in the
specification's translation blocks. Expected answers, as the conductor walked them:

- `two_files_one_filesystem` (§ 3.2): the stdlib roots the boot; Tessa's filesystem is
  identified in the boot, her inodes in the filesystem, her paths yield inodes and are looked
  up in the filesystem; the inode shape carries both warrants. Three `cmp` lines: `/srv/a`,
  `/srv/b`, `/srv/a` again. Line 1: an honest world; the path's identity is its inode; the chain
  ends at the boot's mWorld. Line 2 against line 1: DISJOINT by the two-tops way at the
  filesystem, no store on either leg; the natural-key license UNKNOWN (paths carry no warrant);
  not the same topic. Line 3 against line 1: SAME by `:guarantees-unique-referent` on two inode
  atoms of one value; the natural-key license UNKNOWN; the same topic (the filesystem is the
  one observer and compares SAME with itself).
- `a_hardlink_under_a_false_unique_name` (§ 3.2): Tessa's world with `/srv/a/app.conf` and
  `/srv/mirror/app.conf` two paths to one inode, and Tessa's FALSE `:guarantees-unique-name` on
  the path shape in force. Line 2 against line 1: the walk by identities SAME and the same
  topic; the natural-key license DISJOINT (a wrong DISJOINT); every statement in force true
  except the one named, which is false: the attributed wrong answer.
- `nested_pid_namespaces` (§ 3.2): Pia's pids identified in pid namespaces, namespaces nested
  by shape, the initial namespace in the boot; Dora's `docker exec` lends the container's
  namespace and declares that guest pid 1 corresponds to host pid 4821. Line 1 (`kill -0 4821`,
  the host vantage): honest; identity and world. Line 2 (`docker exec web kill -0 1`, a vantage
  entered through the wrapper): the walk UNKNOWN (one-top fails: the pid scheme has no other
  shape identified in a namespace), `compare()` SAME through the mCorrespondence, and NOT the
  same topic, since a process fact is observer-dependent on its namespace by default and the
  two namespaces compare UNKNOWN (one is the other's container). The last is the informative
  answer: the correspondence makes the referents one and § 2.8's default keeps the facts apart.
- `stage_five_the_index_given_whole` (§ 2.6): USER_STORY stage 5's morning with Anna's apt
  describer naming the package index given whole, identified in the boot, its entailment
  finished; Tessa's and the stdlib's may-read sets closed; Deb's `dpkg -s` reading the status
  file's path; the flag set. Line 2 against line 1: `compare()` KNOWN_UNSPOKEN (two sorts under
  the boot; neither the two-tops nor the one-top way fires); the readset is not ⊤ and the
  writeset is not ⊤; not spared. The acked cost of `311t` § 14 with the identity reason
  isolated from every other reason for a collision.

### § 19.5-suspicions-banked-for-a-fifth-book

- `sus-primary-level-traversal-is-the-store-given-whole` (~SUSPECT; a hand-walk, unrun) — under
  § 1.7's fence a level with no emitted member and no closing act has its mParent given whole as
  a traversal member, primary levels included, since at the primary the catalog and the store
  are one mKey (§ 2.4). A write to any inode in a filesystem then touches the inode level of
  every other key in that filesystem (`touchesTraversal`: the region test reads the written
  inode as covered by the filesystem), so every same-filesystem write routing-invalidates every
  resolution in the filesystem, and the closing act cannot honestly be given at the primary
  level while the world routes anything to the inode (`true_ClosesTraversal` needs nothing to
  pass to it). If that walk holds, USER_STORY stage 5's file survivals never spare within one
  filesystem under the current text, whatever Anna names. To be shown by a book
  (`stage_five_the_list_files_named`, Anna naming the list inodes) after the four run, and held
  as a hole or as the acked coarse floor at the sitting; never repaired here. It sits on
  `ask-passes` (§ 17.1).

### § 19.6-the-opus-builders-remit

One Opus, its own worktree off `ai/main`, on the human's typed ack. The two clamps and the
valve of § 15.3 stand, sharpened for a lower-reasoning builder:

- It never edits a fence body, a world fact, an outcome line, or a translation, with the one
  ledgered exception of § 18.3's renames, each verified by `check { old iff new }` where a body
  moves and by the lock moving nothing otherwise.
- A red is reported with its instance rebuilt by hand and its `30Z` § 2.5 class named, never
  repaired; a book red is FIRST the conductor's hand-walk being wrong, then a fence slip, then
  a hole, and the builder says which it believes and why, and touches nothing.
- A lock row that flips against the committed lock, and any claim that a prior ledger result
  was wrong, is escalated with the evidence, never applied. The valve stays: a new
  counterexample on a law that today reads `timeout` is the arc's purpose firing.
- **[TYPED]** 2026-09-29, mid-sitting: the sibling's tooling work will invalidate the existing
  lock, which will be regenerated in full later; no lock regeneration, churn, or worry during
  the first parts of this arc, until the human says otherwise. Consequences: the builder never
  runs `--write`; a `--check` mismatch against the committed lock is not a finding and is not
  reported as one; a reword is verified by `check { old iff new }` where a body moves and, for
  a rename, by the runner reporting the same result on the affected commands before and after,
  in its report, never by the lock. § 15.2's "no lock write" for the measurement now covers the
  whole sitting.
- Order of work: (1) compile and run the four books (each its own module; `--parse` first);
  report every row; (2) the official-budget measurement of the seven timing-out commands, report
  only (§ 15.2); (3) § 18.3's renames, one commit each; (4) kills for the unkilled green laws,
  `sparing` first, by the `allInForceTrueExcept` pattern; (5) a tooling-chafe section in its
  report. No lock is written.
- No subagents under it. No scope edit except a `bookScope_<book>` a book needs to seat its own
  atoms, and then only upward to the count the world names.

## § 20-the-worlds-sitting-and-the-punt

2026-09-29, the same conductor, after the four books landed (`d8d7da50`) and before any
dispatch. The human's words are banked as typed; the conductor's opinions of the sitting were
read and neither acked nor nacked, the human saying they did not follow them, so they are
chat-tier thoughts and bind nothing.

### § 20.1-typed-this-sitting

- **[TYPED]** a lean, not a global hard rule, and not for the one-to-one mechanization itself:
  where a world of claims seems wrong, the repair is not necessarily to edit the structure of
  the claims. Often both worlds are kept as test cases, since net they specify Dorc's behaviour
  more fully. The human's instance: the conductor's switch to a rooted-boot world is a different
  way of writing the stdlib, and that cannot express the correctness of 311. Possible stdlibs
  are useful for discussing how 311 should operate, but 311 cannot be specified by THE stdlib,
  because the stdlib is specified in terms of 311. A thinner stdlib that cannot license an
  answer is itself a test case that narrows the specification.
- **[TYPED]** "never replace" is too strong: it is one rule of thumb among many. The human
  reached for "only remove when the replacement strictly dominates" and asked what foundation
  that has; the conductor's answer is § 20.3's first item.
- **[TYPED]** the human wants the specification documents readable and short-ish, the Alloy and
  the sh sections reading as specification text and necessary examples; a document that accrues
  every narrowing example grows long fast.
- **[TYPED]** the human's picture of a book: like an e2e, a global thing about the world, whose
  outcome is a set of judgements about it (the narrative and the projection derived from it into
  outcomes), dependent on the whole product and not on one specification document; and no
  mechanical way to share books is visible that does not make the whole specification one
  document.
- **[TYPED]** the human suspects specification documents will not map cleanly, many to many,
  onto seams that align with fundamental units of outcome-truth, and does not know what such
  units are.
- **[TYPED]** the punt: most of the sharing question waits until more specification is written
  and the needed shape can be decided. For now nasty duplication is mildly fine, with an eye to
  reducing it soundly later.
- **[TYPED]** the performance lane is fully landed and `ai/main` forwarded over it.

### § 20.2-findings-posed-and-unanswered

- `fnd-primary-yields-reading-forbids-thin-stdlibs` (+SURE of the texts) — the marked reading
  `enc-primary-yields-into-its-sort` (§ 1.3's fact: some shape of a primary mScheme carries
  `:identified-in` or `:root`) is stronger than 311's sentence, that a primary mScheme yields
  into its mSort for at least one shape. It refuses every world in which a named mSort's primary
  mKeys are scoped in the mRoute, which 311's letter permits (§ 1.6; § 1.5's warrants on any
  lookup). A fact that removes legal worlds is the direction that hides counterexamples. The
  conductor switched the books' worlds to a rooted boot where it should have reported this; the
  human's lean caught it. Posed in chat: replace the clause with "some shape of it declares no
  `:yields`". Unanswered; nothing is edited. The thin twins of the four books (two files scoped
  in the mRoute; a stage-five twin) cannot be written until it is answered, since their run has
  no instance under the fact as it stands.
- `fnd-thin-worlds-withhold-sparing-not-disjoint` (+SURE of the text; ~SUSPECT of the intent) —
  a mRoute is one mWorld, so two chains ending at one mRoute pass step 1 of the walk and may
  read DISJOINT; what a rooted stdlib buys is sparing (a readset member whose chain ends at the
  mRoute is ⊤, § 2.5) and comparison across two mVantages.

### § 20.3-the-conductors-opinions-unacked-and-unfollowed

Listed by slug so a successor does not re-derive them; each is an opinion, none a plan.

- `opn-domination-is-provable-only-syntactically` — one test case strictly dominates another,
  over every future edit, only where the world is the same, the premises no stronger, and the
  dominated conclusion a syntactic conjunct of the dominating one; a mutation battery shows
  redundancy as evidence over the mutants tried, never as proof; a richer world never dominates
  a thinner one, and the pair tests monotonicity (`26M:law-monotone-enhancement`), which neither
  tests alone.
- `opn-exemplary-books-here-regression-books-beside` — a sibling document for the regression
  body, its rows reporting under the specification's own lock so that a red there is the
  specification's red (the answer to `30Z:pos-halo-is-the-hazard`); exemplary books stay.
- `opn-share-what-is-known-keep-conclusions-private` — three roles (a vocabulary of signatures;
  libraries and scenarios of atoms; specifications holding meaning, laws, and short assertions
  about named scenarios), composed by `open`, with textual splice kept for upward obligation
  alone; a scenario silent in a specification's vocabulary is a legal input to it.

### § 20.4-the-tooling-as-landed-and-what-it-changes

`notes/30Y` and `plans/30Z` are rewritten for the landed checker; the conductor re-read
`30Y` § 2 and the praxis's diff. What bears on this arc: every book gets an `every_line`
conjunction; a targeted run (`--only`, `--module`) runs a slice and never writes; the tiers
are `--hot`, `--gate`, and `--official` (from scratch, the ceiling budget); results are definite
or unmeasured, and exit 4 asks for the official tier; the specification's committed lock is
schema 1, so every row re-solves and nothing is cached from it; pre-commit's `assay` step
compiles and parses the staged document. The four books were committed before the lane landed
and have met no parser. § 19.6's order of work stands with two substitutions: the books run as
`--module book_<name>` slices at the hot tier, and the measurement of the seven timing-out
commands is an `--official --check`, which writes nothing.

## § 21-the-primary-coherence-ruling-and-the-state-at-the-rewind

2026-09-29, the same conductor; the last section before the rewind. The dispatch of the builder
is the rewound successor's, on the human's typed ack then.

### § 21.1-typed-this-sitting

- **[TYPED]** for now every relevant book stays in the specification document, with
  duplication where it is needed; the specifics of sharing are punted. Owed work, named by the
  human: moving excessive or less important examples, those that do not flow for a human
  reader, out of the document.
- **[TYPED]** "which worlds the checker considers" IS design: it stays exactly equivalent to
  which worlds Dorc promises to work in. The only worlds the checker may be boxed out of are
  the worlds chosen, and documented, as horizon.
- **[TYPED]** a nit that cannot be acted on now: the coherence sentence of § 1.3 (a primary
  mScheme yields into its mSort for at least one shape) may not be sane as a modelling
  constraint. In any given run the arms are singular, because argv is concrete and a fixed set
  of paths is taken, so whether the spelling carries a word on some branch not taken cannot be
  meaningful to the model. It was intended more as a lint than as a constraint on the truth of
  the model. No design change is ruled in this sitting; 311 as written is what is wanted.
- **[TYPED]** the ruling on `fnd-primary-yields-reading-forbids-thin-stdlibs`: the third option,
  temporarily. The checker stops drawing any restriction from the sentence; the sentence stays
  in the specification word for word; the item joins the burn-down list with an explicit lean,
  to remove the sentence from the specification entirely. For now the one-to-one mapping wins.

### § 21.2-applied

- The specification, one commit: the fact of § 1.3 that demanded `:identified-in` or `:root` on
  some shape of a primary mScheme is deleted; its sentence leaves § 1.3's translation block; the
  mark `enc-primary-yields-into-its-sort` is deleted, its reading being replaced and not acked;
  a new non-mechanical § 1.3.1 carries 311's sentence verbatim from the baseline
  (`1af7e0d9^`, line 153) under `<!-- normative -->`. § 7's list of marks is one site shorter:
  thirteen sites remain. The change widens every law's universe, so rows may move, and a red
  that appears is a finding the narrower universe hid.
- `burn-primary-coherence-sentence` — the burn-down item: the sentence of § 1.3.1, held for the
  design sitting, lean to remove. Its ground is the human's nit above.

### § 21.3-the-one-tooling-need-as-answered

Asked by the human what, of the sharing discussion, is universal whatever shape is later
chosen; the conductor's answer, which drew no reaction and binds nothing: atoms that every book
of a document sees and no law does, the mirror of what the claims module already is for claim
atoms. Today the only place outside a book for a shared world atom is an `alloy` fence, which
lands in the module every law opens and changes every law's universe and every row's key.
Urgency low while books are self-contained.

### § 21.4-what-a-rewound-successor-does-first

- Reads this ledger's § 19 to § 21, the specification whole, `notes/30Y` § 2 and § 3 as
  rewritten, `plans/30Z`, and the `using-alloy` skill.
- Dispatches one Opus builder off `ai/main`, on the human's typed ack, under § 19.6 with
  § 20.4's substitutions: parse; the four books as slices; the official-tier check of the seven
  timing-out laws, reported, never written; § 18.3's renames; kills, `sparing` first; no lock
  written until the human says otherwise. The document has met no parser since the four books
  and the § 1.3 edit, unless the pre-commit step of the commit that carries this section ran one.
- Authors, in its own context and before or beside that dispatch: the thin twins the ruling
  makes writable (`two_files_scoped_in_the_route`: DISJOINT by `:guarantees-unique-name` with
  the inodes scoped in the mRoute, no SAME, and a writing line whose fact is never spared, the
  readset being ⊤ at the mRoute; a stage-five twin of the same shape), and the fifth book of
  § 19.5 (`stage_five_the_list_files_named`), each beside its rooted sibling, none replacing it.
- Holds for the sitting, untouched: the six holes, the thirteen marked sites, § 12's found
  readings, `burn-primary-coherence-sentence`, `fnd-in-force-is-book-global`, and § 19.5's
  suspicion once a book shows it.

## § 22-the-second-opus-sitting-standup-and-strategy

2026-09-29, a rewound conductor over `09a5a85d`. The prompt is the one § 19 answered, reproduced
for reinforcement; the typed items of § 19.1 stand and are not repeated. Written before any
authoring, per the prompt's order.

### § 22.1-standup-findings

- The tree is clean at the tip. The four books of § 19.4 have met no solver. The committed lock
  is schema 1 and stale by design (§ 19.6); nothing writes a lock this sitting.
- `fnd-the-commit-hook-parses-staged-specification-bytes` (+SURE of `hk.pkl`, the `assay` step) —
  committing specification text compiles and parses the staged bytes, so a commit is the first
  parse a new book meets, and a refusal arrives as the commit's own. The conductor still runs no
  parse and no solve by hand (§ 5.5).
- `fnd-opus-dispatch-is-read-as-typed` (the conductor's reading of the prompt) — "we're going to
  try to proceed with Opus builders" is read as the typed authorization for Opus builder
  dispatches in this arc. Fable-tier and Astra-tier dispatches stay gated on a typed ack each.

### § 22.2-strategy

Each line is process, never design. § 19.3's book forms and § 19.6's clamps stand.

- `str-author-in-context-solve-in-the-builder` — the conductor writes every book, translation,
  and ledger line; one Opus builder runs every parse and every solve, reports rows, and edits
  only inside § 19.6's clamps.
- `str-legs-with-a-report-between` — the builder's remit is cut into legs, each ending in a
  report the conductor reads before the next leg is sent: the parse and the books as hot-tier
  module slices; § 18.3's renames; kills, `sparing` first; the official-tier check of the laws
  that read `timeout`, last because it holds the machine for hours. One agent, its context kept
  across legs.
- `str-evidence-over-summary` — a builder's claim counts only with its evidence in the report: the
  row as the tool printed it, the instance rebuilt by hand for a red, the diff for an edit. The
  conductor reads the branch's diff itself before any fold. A claim that contradicts a result
  this ledger records is escalated with its evidence and applied by nobody.
- `str-a-red-book-is-first-the-hand-walk` — § 19.3's order of suspicion stands. To find the false
  conjunct of a red outcome the builder checks each conjunct alone, in a scratch module beside
  the generated ones, never in the specification.
- `str-syntax-repairs-are-lexical-and-shown` — the builder may repair what stops a module
  parsing only where the repair is lexical (a token's spelling, a parenthesis Alloy demands),
  each shown as the line before and the line after; a repair that needs a choice between two
  meanings is reported and made by nobody.
- `str-books-are-appended-never-renumbered` — a new book takes the next free number in its
  family, so no cited slug moves.
- `str-both-worlds-stay` — a thin world sits beside its rooted sibling and replaces nothing
  (§ 20.1). A book's answer is the fences' answer in that world; where that answer rests on a
  suspected hole, the book's commentary says so in one sentence and the hole is held.

### § 22.3-the-books-authored-this-sitting-and-their-expected-answers

Authored into `specs/311-identity.assay.md` before the dispatch; each expected answer is the
conductor's hand-walk of the fences over the stated world, unrun. The worlds, in words, are in
the specification's translation blocks.

- `two_files_scoped_in_the_route` (§ 3.2.5), the thin sibling of `two_files_one_filesystem`:
  Tessa describes files and nothing above them; the inode number's shape carries
  `:guarantees-unique-name` alone, no `:identified-in`, no `:root`, no
  `:guarantees-unique-referent` (an inode has one number; a number alone does not say which
  filesystem it is of); nothing says where a path is looked up, so a path's mKey has no mParent.
  Line 2 against line 1: DISJOINT by the two-tops way at the mRoute; the natural-key license
  UNKNOWN; not one mTopic. Line 3 against line 1, the same path again: the walk and `compare()`
  UNKNOWN, where the rooted world reads SAME; not one mTopic.
- `stage_five_the_list_file_named` (§ 2.6.3), § 19.5's fifth book, the rooted world of § 2.6.2
  with Anna naming one list file by its path: `compare()` DISJOINT (two inodes of one
  filesystem); the readset not ⊤; the writeset not ⊤; the status path stale at line 2, by the
  routing rule (its unclosed lookup has the filesystem given whole as its mTraversal, and the
  region test reads the written path as covered) and by the token rule (the written path
  compares UNKNOWN with the filesystem, the status inode's mParent-Store); not spared.
- `stage_five_the_list_file_named_in_the_route` (§ 2.6.4), its thin sibling: `compare()` DISJOINT
  at the mRoute; the writeset not ⊤; nothing stale, since no level has an mKey for an mParent;
  the readset ⊤, since the status inode's mFullyQualifiedKey ends at the mRoute; not spared.
- `two_volumes_of_one_issuer` (§ 2.6.5): Petra roots the volume id; Ravi's tool writes one
  volume above a fact on another. `compare()` DISJOINT by the two-tops way at the volume id's
  mWorld; neither set ⊤; nothing stale; spared, and in the world the write reaches nothing the
  fact depended on. The one book in which the sparing test is reached.
- `sus-a-contained-write-touches-its-store` (~SUSPECT; a hand-walk, unrun; it sharpens § 19.5 and
  is posed in chat, with no reaction yet) — the walk reads an mKey against its own container
  UNKNOWN (§ 3.2 step 2), and § 3.3's fences read every answer other than DISJOINT as a touch,
  for a mTraversal member and for a mParent-Store alike. So a write to any mKey inside a store
  invalidates the mToken of every mKey scoped in that store and in every store above it, and
  the sparing test is reached only by read mKeys scoped directly in a mRoot's mWorld. § 2.6.3
  shows the shape and § 2.6.5 the exception. Held; repaired by nobody.
