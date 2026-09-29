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
