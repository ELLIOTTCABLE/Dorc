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
