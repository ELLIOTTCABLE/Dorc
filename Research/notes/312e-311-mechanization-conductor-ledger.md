# 312e — The 311 mechanization conductor ledger: from the close of `312d`

> AI-authored (Fable, from 2026-09-29). Notes-tier LIVING ledger for the arc that mechanizes 311
> into `specs/311-identity.assay.md` under the praxis of `plans/30Z`, checked by assay
> (`notes/30Y`). It succeeds `notes/312d`, which is closed after its § 25 and is not edited.
> Nothing here is ruled unless it cites a ruling by `docID:slug` or carries **[TYPED]**; grades are
> +SURE / ~SUSPECT / -GUESS / --WONDER; **[HUMAN]** marks the human's framing, paraphrased from
> chat, never a ruling. Authority: the root docs, `spike/CLAUDE.md`, `specs/AGENTS.md`,
> `plans/30Z`, and the specification's own normative text outrank this. The ledger carries no
> question for the human (`312d` § 14). Successors append sections at the tail and edit nothing
> in the middle.

## § 1-remit-as-typed

- **[TYPED]** 2026-09-29: this conductor takes over the mechanization of the prose-only 311 into
  a mechanically-checked 311. The work is delicate. The ultimate priority is faithful
  translation: the translation introduces no new correctness hole and no new design hole.
- **[TYPED]** the predecessor's reading list governed the standup read. Reading beyond it waited
  for the human's ack during the standup turn only. After that turn the conductor uses its own
  judgement where a need appears.
- **[TYPED]** the conductor's ledger is a new note under the lowest unused `312` sub-ID, found by
  a search of the whole tree, the quarantine and `specs/` included.
- **[TYPED]** the conductor reads `specs/311-identity.assay.md` in full. The gate that the
  reading list recorded is lifted.
- **[TYPED]** the three worktrees of the `312d` arc, and their branches, were deleted on purpose.
  The conductor mints a new worktree for its own mutation.
- **[TYPED]** the official lock pass (`312d` § 25.3, `owed-the-official-write-from-scratch`) is
  dispatched now if the conductor believes it is time, and later if the conductor believes more
  work must land first. The human's lean: run it under an Opus, which handles a misfire,
  interprets the results, and fights minor breakage. The decision is the conductor's.

## § 2-standup-state

- The IDs taken in the `312` series, by file name over the whole tree: `312a`, `312b`, `312c`,
  `312ca`, `312cb`, `312cc`, `312cd`, `312cg`, `312ch`, `312cz` (in the quarantine), `312d`.
  This ledger takes `312e`.
- `ai/main` is at `595961ea`. The specification last changed at `ed70650f`. The committed lock
  is schema 1 and stale since `a212975c`.
- The conductor's worktree is `.tmp/trees/r31-mechanize-311-conductor`, on
  `ai/r31-mechanize-311-conductor`, minted from `ai/main` at `595961ea`. Every commit is by
  pathspec.

## § 3-the-official-pass-launched-by-the-human

- **[TYPED]** 2026-09-29: the human launched the official write in a terminal of their own, with
  the command the conductor had put in chat,
  `mise run assay -- specs/311-identity.assay.md --write --official`. The conductor dispatched no
  builder for it. The human reports how it goes.
- **[TYPED]** the conductor's estimate of the pass's length was an upper bound from the tier's
  caps, with no observed runtime behind it. The human acked the estimate as the safe reading.
- The pass reads `specs/` in the checkout it was launched from. That text stays untouched until
  the lock is written. The conductor's worktree has an output directory of its own on the
  Windows leg, so a parse or a commit there does not reach the pass's modules (~SUSPECT, read off
  `mise.toml` and the directories present, not tested).
- The sentence tables of `312d` § 12 and § 13 survive in the predecessor builder's session
  scratch directory, which is temporary. A copy is in the conductor's worktree under
  `.tmp/312d-accounting-tables/`, ignored by git.

## § 4-the-law-of-the-arc-as-glossed-and-the-goal

- **[TYPED]** 2026-09-29, the goal this arc reaches for: 311 is mechanized, and an adversarial
  review panel is dispatched to attack the fidelity of the mechanization, to find what the
  translation missed. Two earlier panels sit unfinished in consequence, gated on this proof
  platform and on the human's bandwidth, which is about one ruling at a time.
- **[TYPED]** spec edits: nack on anything that is not fidelity. No fixes, no improvements, no
  corrections, no narrowings. A sentence of 311-as-prose that cannot be represented mechanically
  becomes a hole, where it looks mechanical but is underspecified and the solver reds it in a way
  no fidelity edit closes, or normative non-mechanical text, where it does not look mechanizable.
  The human glossed the standing law of `312d` § 1, § 5.1, § 7, and § 21.1 and did not re-rule.
- **[TYPED]** the human has not read the conductor's findings of the standup read and has no room
  for them: they are unseen, neither acked nor nacked. Only what blocks the goal is put to them.
- **[TYPED]** the predecessor's assay asks are written as a handoff for the assay conductor at the
  project root, `_tmp-assay-needs-from-311-mechanization.md`: needs and clear chafes, brief, not a
  brief.

## § 5-fidelity-repairs-applied

Each repairs a transcription toward 311's letter under the law of § 4, on the conductor's own
authority, one commit each on the conductor's branch, translation moved with the fence. None was
put to the human. The lock is to show what moved; every one is expected to move nothing.

- `rep-marks-out-of-the-blocks` — in § 2.1 and § 3.2 an UNACKED READING paragraph sat inside the
  translation block, so the sentences after it (both sections' law sentences) formed a blockquote
  with no header, which under `30Z:fw-normative-prose-is-a-headed-blockquote` is neither
  translation nor normative. The two paragraphs now follow their blocks. Commentary only.
- `rep-finished-record-truth-ranges-over-the-engines-keys` — `true_FinishesEntailment` ranged
  over `keysOfSort`, which holds primary and cell mKeys only, while `entailmentFinished` reads the
  written mKey's own shape; a record for a natural shape (a file written by its path) was
  therefore true in every world. The truth now ranges over every mKey whose `sortOfKey` and shape
  are the record's, the keys the engine treats as finished. The premise of the sparing law
  strengthens, so no green can move red; a twin or a kill that rested on the vacuous truth could
  go unsat, and that would be a finding.
- `rep-walk-reads-cell-sorts-as-sorts` — the walk's last step and `law_different_sorts_never_same`
  tested `x.scheme != y.scheme`, so two cell mKeys of two cell mSorts read UNKNOWN where 311 says
  mKeys of different mSorts read KNOWN_UNSPOKEN; the test is now over the mScheme or cell mSort an
  mKey carries. Both are safe bottoms; no law's answer moves.
- `rep-translations-say-what-the-fences-say` — § 1.4's definition of mKey-Primary now names a
  cell's mKey, which `isPrimaryKey` includes; the truths of `:guarantees-unique-referent` and
  `:root` now say "or both reach none", which their `=` on two `lone` sides says
  (`312d` § 9, the human's `=`-on-empty rule). Translation only.

## § 6-findings-of-the-read-unseen-by-the-human

Put in chat 2026-09-29 and, by the human's statement, unread; banked here so a successor does
not re-derive them. Each is a hand-walk; no solver has seen any. None is acted on beyond § 5.

- `fnd-sparing-law-seats-two-keys` (+SURE, scope arithmetic) — `mLevel` is one top-level
  signature over mKeys and both mWorld kinds; at `for 4` an mKey needs an mRoute and a non-⊤
  readset needs a mRoot mWorld, leaving two mKeys. The only spared world with two mKeys is two
  sibling mKeys of one `:root` shape, the shape of `two_volumes_of_one_issuer`. The sparing law's
  green, its two kills, and `law_exclusion_readings_agree` cover that shape. Rules 2 to 4 of the
  writeset, a store on a leg, a natural mKey, and a cell cannot appear in a spared world there.
  Widening `mLevel` alone (`for 4 but 5 mLevel, 10 Claim`) is the measurement to ask for after the
  official pass; a scope is part of the claim, so it is the human's.
- `sus-two-separated-leaves-share-a-held-part` (~SUSPECT) — a wrong sparing with every statement
  true, needing three mKeys: two sibling mKeys with unique names (DISJOINT) whose mReferents both
  hold a third mReferent; a write to one affects the shared part, which affects the other. The
  finished record ("affects only it and what it holds") and the may-read closure ("affected only
  by itself, what it holds, what holds it") are both true. `git -C wt-b gc` above `git -C wt-a
  fsck`, the object store shared. Not covered by `hole_world_scoped_top_aliases_into_a_store`.
  Sits on `312d` § 17.1's `ask-affects-and-the-chain`. Cheapest demonstration: a book of three
  mKeys at scope five.
- `fnd-every-referent-has-a-key-is-a-fact` (+SURE of the text) — `fact { all r: mReferent | some
  reaches.r }` transcribes "It has one or more mKeys" and boxes the checker out of worlds holding a
  thing nobody keyed; at the sparing law's scope it is what keeps the world above from fitting.
  Same class as the § 1.3.1 ruling (`312d` § 21.1): a checker boxing that is not documented
  horizon.
- `fnd-two-truths-are-never-the-sole-support` (~SUSPECT) — no answer rests on the truth of `:root`
  or `:identified-in` alone: `:root` is carried by construction as one mWorld per shape, and a
  true fitting supply implies `:identified-in`; their owed kills should come back unsat.
  `:observer-independence` has no kill because no fact-transport law consumes `sameTopic`.
- `sus-a-contained-write-touches-its-store` (`312d` § 22.3; +SURE of the text now) —
  `tokenInvalidatedBy` and `touchesTraversal` read every answer other than DISJOINT as a touch,
  and an mKey against its own container reads UNKNOWN, so within one store nothing spares; 311
  § 3.3's letter supports the fences.
- `fnd-self-flagged-weak-points-sit-in-the-artifact` — the thirteen UNACKED READING marks and the
  "suspected hole" commentary are self-flagged weak points inside the document a fidelity panel
  would read; stripping them before dispatch contradicts `312d` § 7's typed order that they sit
  inline. Put to the human as the first blocker; undecided.

## § 7-the-path-to-the-panel-as-typed

- **[TYPED]** 2026-09-29, walking back the "blockers only" framing of § 4 in one respect: the
  end-of-project correctness and soundness constraint is not softened. The human's lean: any
  significant work before the design decisions is running in place, since a design change on
  shifting sand opens holes as easily as it closes one. The hope: a one-to-one mechanization, a
  mechanical green, and an adversarial review that says the green means what the text looks like
  it says. Then a design sitting can know that a thing is not so, and a red counterexample says
  so too, and progress is monotone. The conductor gets from here to there, asks only when
  blocked, one question at a time, in simple language, and defers what it can to after the
  review.
- **[TYPED]** the official pass runs in the human's terminal for many hours, in parallel with all
  of the conductor's work.
- **[TYPED]** every line of normative specification text must be fully, strictly ASD-STE100
  compliant. The human added that line to `specs/AGENTS.md` in the primary checkout. Neither
  this conductor nor its predecessors had applied the `asd-ste100` skill to the specification's
  blockquotes.
- Conductor's reading, unasked: Opus builders are dispatched under `312d` § 24's typed
  authorization for this arc, without a per-dispatch ack; a Fable or Astra dispatch, the panel
  included, still takes one.

## § 8-work-toward-the-panel

- `act-accounting-close-dispatched` — an Opus in `.tmp/trees/r31-accounting-close` on
  `ai/r31-accounting-close` off `6d1ca99f`, seeded with the surviving tables: the tri-partition
  at the tip, the reverse account, a fence-against-sentence firewall audit, and the absent
  sentences; it edits nothing and runs no solver. Its brief is in the conductor's session
  scratchpad, not durable.
- `rep-seat-supplies-any-scheme-of-the-sort` — § 1.6: `supplyFits` demanded an instance of the
  mParent mSort's primary mScheme, where 311 says "one of the mParent's mSort's mSchemes" (the
  narrowing `312d` § 12 found without a mark); the fit is now by `sortOfKey`, and the child's
  mParent is the identity of the instance (3.1's "scoped in the identity of its mParent"), for
  the `:identified-in` and the cell cases alike. Widens the worlds; no answer can move green.
- `rep-referent-key-fact-becomes-prose` — § 1.1: `fact { all r: mReferent | some reaches.r }` is
  deleted; "An mReferent has one or more mKeys" moves to § 1.1.1's normative block with the
  reason beside it, on the § 1.3.1 precedent (`312d` § 21.1). The checker now considers worlds
  holding an unkeyed piece. Predicted consequence: the sparing law's premise twin can seat the
  shared-part world of § 6 at its own scope, so `law_sparing_is_sound` is expected RED at the
  next run; that red is the arc's protocol firing and becomes a hole for the sitting, not a fix.
- `act-ten-kills-asked` — kills for the species whose truths no kill exercised
  (`312d:owed-kills-for-ten-species`), each `expect 1`, placed by the law `311` § 5.2 names as
  the consumer: `:identified-in`, `:root`, a supplied mParent instance, and a wrapper's sentinel
  against the walk laws; a closed may-read set, a finished record, and a supplied instance
  against the sparing law; `alias nothing-else` against the region law. ~SUSPECT most come back
  unsat, which then says mechanically that no answer rests on that statement alone; the
  translation sentences say so. `:observer-independence` and `:lends` have no consuming law
  (no fact-transport law exists) and got none.
- `act-sparing-law-asked-at-five-levels` — the law's body is a named predicate; a second check
  asks it at `4 but 4 Int, 10 Claim, 5 mLevel`, whose twin demands a store on the read mKey's
  chain, the world the four-level scope cannot seat. Its result is the affordability
  measurement.
- `rep-sentinel-truth-presupposes-a-callers-instance` — `true_ClosesLends` demanded that every
  natural mKey of an unlent mSort under the wrapper reach what the caller's instance passes to,
  with no guard that the caller holds one, so a natural mKey of an mScheme with no declared
  catalog mSort made every sentinel false and hid every world with one. The truth now
  presupposes the instance, as its sentence does ("the caller's instance").
- `act-five-books-for-the-untouched-sections` — each a pinned world the conductor hand-walked,
  its translation STE-strict, its answers the fences' answers as walked: § 3.2.6 two cells of
  one unit (KNOWN_UNSPOKEN between cells, SAME for one cell read twice under two unit atoms);
  § 3.2.7 one configuration merged in two orders (a composite SAME by parts and UNKNOWN by the
  walk; swapped roles UNKNOWN); § 3.4.2 and § 3.4.3 one file through `sudo` with and without
  the flag (SAME with the sentinel in the support, else UNKNOWN across two mRoutes); § 3.3.2 a
  reboot between two reads (lifecycle invalidation withdrawing a timeless SAME); § 2.10.2 a
  directory removed beside a file (the region test DISJOINT through the placing route; the
  store-given-whole floor invalidating all the same; the world inside
  `hole_region_closure_with_unknown_leaf_pair`, a file and a directory being mKeys of two
  mSorts the walk never separated). Every book passed the commit hook's parse; none has met a
  solver.
- `fnd-invalidation-does-not-know-when-a-key-was-resolved` (+SURE of the text; a mechanization
  limit, not 311's) — `staleAt[s, k]` marks an mKey stale at every site below a line that
  touched its chain, whether the mKey was resolved above or below that line; the reboot book
  shows the mKey resolved after the reboot marked stale too. 311 § 3.3 says "an mKey whose
  mResolution ... a line above invalidated"; the fences hold no resolution time
  (`312d:str-static-over-lines-not-temporal`). For the panel and the temporal latitude.
- `acc-the-accounting-at-the-tip` — the Opus's account over `6d1ca99f` (tables copied to the
  conductor worktree's `.tmp/312e-accounting-tip/`, not durable): of the 770 baseline
  sentences, commentary 211, mechanical with a named carrier 263, structural 32, carried
  differently or by an inert definition 32, residue 223, duplicates 9, absent 0. Five rows moved
  category, each by a repair of § 5 or § 8 (the § 1.3.1 sentence to residue; the cells'
  KNOWN_UNSPOKEN sentence from structural to mechanical, its structural judgement having been
  wrong at `a212975c`; three § 2.8 sentences from inert to mechanical, the books now reading
  `sameTopic`). Of 436 blockquote sentences: 146 verbatim, 77 near-verbatim, 55 merges, 16
  moves, 6 rewords, 136 additions of the mechanization. Four clauses of the baseline appeared
  nowhere: the mTraversal being ordered, "or the filesystem binder", "leaf first", and the
  engine's vouch (replaced by the axiom under `43b8d4dd`); the first three are restored as
  normative prose or into their sentence.
- `acc-firewall-audit-disposition` — fifty fence-against-sentence findings. Applied as fidelity
  repairs: the writeset reads a written mKey's own entailment beside its identity chain's
  (F09: `contributingContainers` and `writesetUnexcluded` ranged over `levelKeysOf`, so the
  finished record's truth and the engine's read now cover one entailment); `parentCatalog` is
  the mParent for a primary mKey too (F10, 311's "one mKey"); the unused fourth seat atom
  deleted (F17); every hole has a sentence and every law sentence names the holes it is asked
  outside (F01 to F08); the guards the sentences dropped restored (F14 the cell, F15 the closing
  act, F16 the non-empty mTraversal); "by the walk" on the different-mSorts law (F12); a verdict
  fact reads an mKey, a cell's included (F13); the catalog supply claims nothing (F18);
  `sameTopic` in the fence's words (F20); the commentary slip at the index book. Left as
  recorded, no repair: `compareAt` has no consumer but the books (F11); `DeclaresPlaces` gates
  no record, invocation being prose (F21); no fact forbids a cell mSort a primary mScheme (F24,
  a wider universe); rule 3 applies at the seed only (F26, 311's "entry"). Rank 3 (kill and
  twin sentences, books' world facts, reasons the outcomes do not assert, two nits in the
  shared half) went to the STE pass as translation-only work.
- `act-ste-rewrite-dispatched` — an Opus in `.tmp/trees/r31-ste-rewrite` on `ai/r31-ste-rewrite`
  off `9b3662ee`: every headed-blockquote line to strict STE with the fence as the authority,
  one commit per top-level section, the linter as its gate, fidelity mismatches reported and
  not repaired, plus the rank-3 gaps above. The conductor reads every changed line before any
  fold. The accounting worktree and branch are removed (no commit of its own).
- **[TYPED]** 2026-09-29: the conductor runs no `--official`, and no second official pass runs
  before a pending tooling change that invalidates the official lock's keys has landed. The
  human's in-flight run lands tomorrow and is read for what it measures.
- **[TYPED]** 2026-09-29: a brief authorization for small, width-one solver work beside the
  official run was withdrawn the same hour: a sibling conductor is also working around or
  tuning the heavy-work lock, and three concurrent solver users is too ambitious. No solver
  work on either leg from this conductor while the official run holds the Windows lock.
  Measured before the withdrawal, for the record: the Windows lock held by the human's `assay`
  (pid 51604); the WSL leg unlocked, 19 GB free of 20; the host 12.5 GB free of 32 beside two
  JVMs. A measurement worktree was created and removed with no commit.
- **[TYPED]** a sibling conductor is working on the tooling and may fast-forward `ai/main` when
  done; this branch rebases over it at the fold.
- `pln-the-road-to-the-panel` (the conductor's plan, process only) — (1) the STE pass lands and
  is read line by line; (2) nothing touches a solver while the human's pass holds the
  heavy-work lock; afterward a builder measures the settled text at the gate tier, in a
  worktree of its own; (3) every red is triaged under `30Z` § 2.5: a transcription slip is
  repaired toward the letter, a design question becomes `hole_` with its witness and premise
  exclusion; the laws re-run at the gate tier until no new red appears; (4) the official pass
  and the lock's commit to `ai/main`, which accepts the rows, are scheduled by the human once
  the key change is in; (5) the panel, on the human's typed ack, after the human's word on the
  marks. The human's pass over `ed70650f` is the affordability measurement of the seven laws
  and the three unmeasured books, nothing more, since the text has moved under it.
- `act-ste-rewrite-folded` — the Opus's five per-section commits, squashed to one (a mechanical
  reflow; the builder's messages named sections) and rebased over this branch, then the
  conductor's own commit for the eight translation mismatches the builder's audit found and
  left: a cell's mParent in the two-cells book, the aligned-SAME range including the pair
  itself, `sameTopic`'s matching in each direction (not one to one), the vantage cycle through
  other vantages, "a matched shape is a control-flow path" moved to § 1.3.1's normative block
  (no fence holds it), "names a new mReferent afterward" moved to § 3.3.1's (`lifecycleInvalidatedBy`
  holds no new mReferent), the sudo-without-the-flag outcome in the fence's words, the shared
  half's derived argv clamp and its consequence sentence moved to commentary, and every book
  ceiling naming its bitwidth. At the tip: 975 normative lines, 13,330 words, zero hard
  violations, 111 passive advisories, 2 present-perfect kept for current relevance. The
  builder's other interpretations (a sentence per `#=` line, twins named by fence name, the
  `owns` line of the pid book completed to the fence) were read and stand.
- `fnd-the-commit-hook-parses-in-silence` (+SURE) — the builder reported that the `assay`
  pre-commit step "never ran" because no line for it appeared; under `HK_FIX=0` the hook runs
  `hk run -q`, which prints nothing for a passing step. The evidence that it ran is the staged
  compile output, `spike/target/alloy/311-identity.staged/`, whose files carry the timestamp of
  each commit in both worktrees. Every spec commit of this arc has therefore been compiled and
  parsed by Alloy at commit time.
- `msr-the-official-pass-over-ed70650f` — the human's `--write --official` pass (Windows, two
  children, 1800 s CPU and 4096 MB per command) over the text at `ed70650f`, read by an Opus
  from copies of `out.json` (a PowerShell transcript wrapping the report), the schema-2 lock
  (83 rows, matching the report row for row), and the stored instances. Affordability at the
  ceiling: everything is definite except the two `compare()` laws, which time out SOLVING at
  1800 s (translated in about 19 s, about 650k clauses, the size of siblings that solve in 20
  to 293 s; solve-bound, not translation-bound). Of the seven laws never measured above 120 s:
  `region` 76 s and `two_tops` 82 s (under the hot budget; the hot tier had deferred them on
  their old timeout rows), `exclusion` 124 s, `unstale` 168 s, `disjoint` 293 s
  (no-counterexample, twin sat), the two `compare()` laws unaffordable. The kill by
  `:aliases-nothing-else` is sat at 269 s. The three translation-bound books are definite:
  `two_files_one_filesystem` at 428 to 456 s per row (2.6M clauses, the largest in the
  document), the two stage-five books at 61 to 184 s. No definite result flipped against
  `312d` § 25.2; no construction disagreement; no unsat run. The lock the pass wrote sits
  uncommitted in the primary checkout beside `out.json`; it records the text at `ed70650f`,
  which `ai/main` still carries, and every key of it moves under this branch's repairs. Whether
  it is committed to `ai/main` as the measured baseline of that text is the human's; this
  branch cannot carry it.
- `red-the-two-exclusion-readings-disagree` — `law_exclusion_readings_agree` found a
  counterexample (124 s): a world of no mReferents, an entailment cycle between two shapeless
  primary mKeys, a natural at-most entry yielding the read mKey itself, a finished record on
  the natural shape (vacuously true under the old text), and a closed `looked-up-in` making the
  region test DISJOINT. Under the exclusion-as-built reading the fact is spared; under the
  exclusion-at-the-test reading the writeset is ⊤. `30Z` § 2.5 kind 4, and the answer to the
  question the law asks: the two readings of `311` § 2.6's exclusion sentence do disagree
  (`312ch` item 11; `hold-exclusion-keyed-on-the-container`). Nothing changed: the law stays
  as the question, its red is the honest form (`30Z:hole-three-kinds-and-their-idioms`, the
  expected-red idiom), and the lock's commit accepts it as residue. The world also sits inside
  `hole_region_closure_with_unknown_leaf_pair`, which this law's premise does not exclude on
  purpose, and shows `sus-finished-record-for-a-natural-shape-is-vacuous` live under the old
  text (repaired since, § 5).
- `red-region-reached-through-the-composite-hole` — `law_region_disjoint_is_sound` found a
  counterexample (76 s): a composite mKey whose one part is itself, SAME by parts with another
  composite that reaches a different mReferent, lifting the walk's UNKNOWN into `compare()`'s
  DISJOINT, which the region test then consumed. Every statement in force true. The world
  satisfies `hole_composite_keys_with_same_parts_refer_differently`, which the sparing and
  `compare()` laws exclude and the region law did not. Applied under the hole protocol at
  `ffea0bef`: the composite hole added to the region law's premise, its twin, and its two
  kills, the translation saying so. A composite whose part is itself is representable in the
  fences (`part: Role -> lone mKey` allows it); noted for the sitting, not narrowed.
- `red-a-thing-reachable-by-a-route-nobody-emitted` — `law_unstale_route_is_untouched` found a
  counterexample (168 s): a natural mKey with a closed-catalog mParent whose mReferent is
  passed to by two mReferents, the catalog's and another's; a line writes the other; the
  unclosed traversal is the catalog given whole, whose region test reads the written mKey
  DISJOINT; nothing is invalidated, and the write sat on a route to the thing. Every statement
  in force true; `hole_natural_key_catalog_off_the_route` is false in it (the catalog does pass
  to the thing). Kind 4: no unit says a thing is reached only through its catalog, the route
  side of the § 5.1 OPEN "no other home" cell (`held-shared-parts-across-separated-things` in
  the held-work file). NOT applied, by the human's stop: the owed act is a new hole, candidate
  `hole_a_route_off_the_catalog_reaches_the_thing` — `some k: mKey | not traversalClosed[k] and
  some passes.(k.reaches) - (k.parent & mKey).reaches - crossed[k].reaches` — with its witness
  run at six, added to the untouched-route law's premise, its twin, and its two kills, and a
  translation sentence for the hole. A successor authors it; nothing else is owed for this red.
- Reader chafe for the assay owner, beyond `312d` § 25.5's: the report and transcript carry no
  exit code; no row points to its instance file; the lock summary's `mismatches` merges new
  and moved; the two `compare()` timeouts printed no stderr diff line though their rows moved
  from schema 1 to 2; instance XML carries only the two tables, so a red's writeset and region
  answers are hand-evaluated with no evaluator short of a solve.
- `msr-ste-over-the-normative-lines` — the skill's linter over the 441 blockquote lines (9,405
  words) at `8a1be5a7`: 143 semicolons, 123 sentences over the cap, 81 passive advisories, 4
  present-perfect, 1 synonym rotation; 159 lines carry a hard flag. The day's own sentences are
  clean. The bulk rewrite is a translation-only pass (legal without ceremony under
  `30Z:fw-every-fence-change-changes-the-translation`), to run after the accounting audit lands
  so that the audit and the rewrite do not cross, by an Opus with the linter as its gate, the
  fence as the authority, and the conductor reading every changed line.

## § 9-the-rewound-conductor-and-the-hole-applied

2026-09-29, a rewound conductor over `578eaeea`, stood up on § 1 to § 8 and the specification
whole, then given the order to say what is next toward the end-state claim: that the mechanical
311 is as close to the prose 311 as it can be got, and an adversarial review searching for subtle
meaning-shifts is due.

- **[TYPED]** the official pass is complete, its lock is committed to `ai/main` (`49841963`, the
  human's own commit), and the predecessor surveyed it (§ 8, `msr-the-official-pass-over-ed70650f`).
- State: `ai/main` carries three commits this branch lacks: the sibling's LIVING_STATUS pointer
  (`083aceb7`), the lock, and the sat4j effort counters in the adapter (`034e5da1`). The lock
  records the text at `ed70650f` under the pre-counters adapter; every one of its keys moved twice
  since, under this branch's repairs and under the adapter's digest. It is evidence for that text
  and for nothing at the tip. The counters commit is the tooling change § 8's typed rule waited
  on before any second official pass (~SUSPECT it is the one meant; the LIVING_STATUS pointer
  says it "lands when the next official run is due anyway").
- `hole-a-route-off-the-catalog-applied` — `1aa60dd2`: `hole_a_route_off_the_catalog_reaches_the_thing`
  as § 8 named it, its witness run at six, the untouched-route law, its twin, and its two kills
  asked outside it, two translation sentences for the hole and the counts of holes retuned. The
  pre-commit step compiled and parsed it. Nothing else of § 8's three reds is owed.
- The road to the panel, as put to the human this sitting, stays in chat until reacted to.

## § 10-the-remit-bounded-the-panel-plan-typed-and-the-top-repair

2026-09-29, the same conductor, after the human's reaction to § 9's road and a notification from
the assay conductor's session.

### § 10.1-typed

- **[TYPED]** the fidelity gap `gap-top-writeset-invalidates-nothing` is acked unread, on the
  conductor's claim that the repair moves toward fidelity and gives reviewers less to catch.
- **[TYPED]** proceed as planned (§ 9's road, items 1 to 5).
- **[TYPED]** the panel is not within this lane. The remit is to bring the tree right up to the
  panel and stop: a tree that is fully, as far as the conductor knows, equivalent to the
  before-mechanization text.
- **[TYPED]** the plan for the panel, for the lane that runs it: the panel's first pass and its
  attack planning happen against ONE pure document, fully stripped, holding only the normative
  prose blocks and the mechanical blocks (no commentary of any kind, the UNACKED READING marks
  included), set against the full text of the original prose 311. The panel is instructed to
  produce Alloy, failing counterexamples, real-world shell books, and precise descriptions of how
  each attack proves disagreement or inauthenticity against the prose 311. Only after all of that
  is the panel told it may access other files of the project and its history, to collect
  further context for its items and report. All initial work is pure over the normative 311
  alone, unpoisoned by the non-normative commentary of the mechanical 311.

### § 10.2-the-assay-conductors-notification

A cross-session message from the assay conductor, 2026-09-29, a notification and no instruction;
its facts as relayed, each a peer's claim:

- `ai/main` moved to `0586ac1b` with the assay tooling; this branch's rebase (§ 9) landed on
  that tip, 32 ahead and 0 behind, with no conflict.
- The adapter's source digest moved (the sat4j effort counters), so every row of the committed
  lock now mismatches; regenerating it from scratch on the landed binary, together with this
  arc's spec changes, is this lane's. The human's official run took 1 h 34 m on two children;
  the official tier's batch cap is eight hours, a capped pass writes what it measured and leaves
  the rest owed, and is not resumable by ruling.
- `--write` refuses (exit 2) while the document or either shared half differs from `HEAD`, and
  records that commit in the lock header (`{"schema": 2, "commit": …}`): commit the text, then
  write.
- A solving pass writes its JSON report to `<repo>/.tmp/assay/<stem>-<UTC stamp>.json` and
  prints a summary ending in that path; progress goes to stderr as `tracing` events; `mise run
  assay-quiet` is the hook and agent spelling; `--help` lists every flag; deferral notes name
  their measurement; `expect` rides report rows. The shared-world module for books, the
  instance text form, and the two lints are the next tune, not started.
- Every tooling task carries `--profile tooling`; the tooling exe lives under `target/tooling/`,
  so the first build after the rebase is cold.
- Conductor's reading of "this lane owns the regeneration": consistent with § 1's typed lean (an
  Opus runs the official pass) and § 8's typed rule now that the key-moving change has landed; the
  write runs at the settled tip, after the slices loop, by an Opus, and its commit is the act that
  accepts the rows.

### § 10.3-the-top-repair-applied

- `rep-top-writeset-touches-everything` — `d15a630d`: `lineWritesetIsTop[l]` is
  `writesetIsTop[l, lineWriteset[l]]`; routing invalidation fires where the line's writeset is ⊤
  and the mKey has any mTraversal member or placing record; token invalidation where it is ⊤ and
  the identity chain has an mKey above; lifecycle invalidation where it is ⊤ and an mKey on that
  chain is mRoot-adjacent; and a ⊤ line counts as a write for an open read set. Three translation
  sentences say so. Direction: more stale, fewer spared, every existing book outcome unchanged by
  hand-walk (~SUSPECT until the slices run; the reboot line of § 3.3.2 was already ⊤, its boot
  having no finished record, and its outcome asserts only what still holds).
- Predicted consequence for the untouched-route twin: `not routingInvalidatedBy` now needs the
  line's writeset not ⊤, so every member's finished record must be in force too, eight
  statements at nine; ~SUSPECT it seats. If unsat, the count is the human's (a scope is part of
  the claim), never the conductor's.
- A consequence for book authorship, not a repair: a line with no completion record is a ⊤
  writer under the letter, a `cmp` line included, so a book that wants a read line inert below
  itself declares `ClosesMayWrite` on it with no entry. The existing books assert nothing that
  this changes; new books follow the convention.

### § 10.4-findings-of-the-read-unseen-by-the-human

Banked under the § 6 rule (the human has no room for findings; only blockers are put); a
hand-walk each; none posed, none acted on.

- `sus-passes-is-one-step-in-every-truth` (+SURE of the fences; ~SUSPECT of weight) — every
  truth predicate and law that reads a route reads `passes` one step (`passes.(r)`,
  `r.passes`), never `^passes`; the books pin `passes` transitively closed by hand
  (§ 2.10.2 states both `fs_1->inode_a` and `dir_a->inode_a`), and no fact says a book must.
  A book that pins one hop per edge makes the closing act's truth, the region law's
  conclusion, and the untouched-route law's conclusion read shallower than "a route through"
  in English. A world-modelling convention the panel may name; sits on
  `312d` § 17.1's `ask-passes`.

## § 11-the-gate-measurement-over-the-repaired-text-and-its-triage

2026-09-30. One Opus (worktree `.tmp/trees/r31-311-slices`, branch `ai/r31-311-slices` at
`95b5abff`, no edit, no commit, no `--write`) measured the document at the gate tier, 600 s of
CPU per command, as `--module` slices after a whole-document pass was cut at the harness's two-hour
background cap with no report written. Reports in its session scratchpad, not durable; the rows
are in its hand-back and are summarized here.

### § 11.1-what-was-measured

- Laws: 61 rows in 1 h 02 m. 42 green; the two accepted reds (`exclusion_readings_agree`,
  `region_disjoint`); `law_sparing_is_sound` moved from no-counterexample to counterexample;
  `law_compare_same_is_sound` moved from a 1800 s timeout to no-counterexample in 513 s;
  four unmeasured at 600 s of solving (`compare_disjoint`, `unstale_route` at 800k clauses,
  `kill_region_disjoint_is_sound_alias_nothing_else`, `kill_disjoint_is_sound_identified_in`);
  eleven rows new to the lock. All 13 twins sat but one; all 7 hole witnesses sat; 11 of 14
  measured kills sat.
- Books: every row of every book green as walked, the ⊤ repair having moved none; the six
  books authored since the lock are new rows. `two_files_one_filesystem` took 16 m 38 s at the
  gate tier and `stage_five_the_index_given_whole` 6 m 21 s.
- Prediction P2 (the untouched-route twin starving by one statement) did not occur: sat at nine.
  P3 (a book red) did not occur.

### § 11.2-the-reds-and-what-each-became

- `red-sparing-through-a-shared-part` — `law_sparing_is_sound` and
  `law_sparing_is_sound_with_a_store_on_the_chain`, both P1 exactly: two root-scoped sibling
  mKeys under `:guarantees-unique-name` with a finished record, whose mReferents both hold a third
  mReferent; the line writes one, its write affects the shared part, which affects the other; the
  fact depends on the other; DISJOINT by the two-tops way; spared; every statement true by hand
  (the finished record and both may-read closures hold because the part is held on each side).
  `30Z` § 2.5 kind 4, the thing's-end "no other home" cell of § 5.1. Applied under the hole
  protocol at `8d46d614`: `hole_two_separated_things_hold_one_part` (two mKeys `compare()`
  reads DISJOINT whose mReferents share a held part, directly or through others), its witness at
  six, the two sparing laws, their twins, and the five sparing kills asked outside it, the
  translation saying so. Held for the sitting; repaired by nobody.
- `red-region-through-the-world-scoped-top-hole` — `law_region_disjoint_is_sound`'s new
  counterexample (the lock's was the composite one, closed at `ffea0bef`): a route-scoped top
  with `:aliases-nothing-else` whose mReferent owns the mReferent a sibling top and a cell key
  both reach; the leaf pair DISJOINT and false by the two-tops way; the region test DISJOINT by
  the placing form, its closure vacuous with nothing passing. The world sits inside
  `hole_world_scoped_top_aliases_into_a_store`, which the DISJOINT laws exclude and the region
  law did not. Applied at `8d46d614`: that hole added to the region law's premise, its twin, and
  its two kills. The builder's scratch run of the law so premised timed out at 600 s (665k
  clauses); the official pass measures it.
- `red-with-a-store-twin-starved` — `law_sparing_is_sound_with_a_store_on_the_chain_premise`
  unsat at ten statements: a store on the read mKey's chain needs its own `:primary-of`, `:root`,
  `:identified-in`, a second supplied mParent, and a second closed may-read set, thirteen in force
  by the builder's hand count (~SUSPECT). The law had never been read. Applied at `8d46d614`: the
  second command and its twin run at fourteen, the least count that seats the witness plus one,
  the reason beside the scope paragraph (`30Z` § 2.8: raise a bound only when a twin is unsat
  for want of atoms). If fourteen still starves, the count is the sitting's.
- `red-exclusion-readings-a-new-shape` — the accepted red's instance moved shape (one mReferent;
  a shaped and a shapeless primary mKey in an entailment cycle; the natural at-most entry yields
  the read mKey itself). The question the law asks keeps its answer. Nothing applied.
- `red-three-kills-unsat` — `kill_same_is_sound_identified_in`, `kill_same_is_sound_root`,
  `kill_same_is_sound_closes_lends` unsat at five, each carrying `expect 1`; they land as
  accepted reds. The first two are § 6's `fnd-two-truths-are-never-the-sole-support` measured: the
  walk reads `:identified-in` only as a sort match and `:root` only as the world atom the shape
  is scoped in, so a SAME still needs `:guarantees-unique-referent`, whose truth over one parent
  forces equal reaches. The third is evidence about a marked reading, not about 311: under
  `312d:enc-one-instance-is-one-atom-for-now` an inheriting vantage holds the caller's mRoute
  ATOM, so a false sentinel and a true `:guarantees-unique-referent` over that atom contradict,
  and no world exists in which the sentinel alone is the false statement behind a SAME. Under the
  letter the sentinel is what makes two instances one, and its falsity should be expressible
  alone. The marks in § 3.2 and § 3.4 already hold this; the unsat kill is its measurement.
  The `expect 1` stays: the document says these kills ask whether the law dies, and the residue
  records that it does not.
- `msr-compare-same-now-definite` — `law_compare_same_is_sound` no-counterexample at scope six in
  513 s, where the lock's row was a 1800 s timeout: a definite result replacing an unmeasurement,
  legal under `30Yf:lock-asymmetric-match`; ~SUSPECT the repairs since `ed70650f` shrank its
  problem. `law_compare_disjoint_is_sound` still times out.

### § 11.3-tooling-chafe-from-the-builder

- A whole-document gate pass (laws about an hour, books about half an hour more) outruns the
  harness's two-hour background cap, and a killed pass writes no partial report; only the stored
  instances survive it. The builder fell back to per-module slices, each `--json`'d, and the
  replay of those instances made the slices cheap.
- `assay-quiet` prints nothing for two hours; the only progress signal was new files under
  `instances/`.
- `--module` takes one module per invocation; a sweep needs a wrapper loop.
- The runner's caps go after a SECOND `--` (`mise run assay-quiet -- --check <spec> -- --batch-timeout N`); § 10.2's spelling with one `--` was wrong.

## § 12-leg-two-and-the-third-thing-hole

2026-09-30. The same Opus re-measured the eleven commands `8d46d614` touched, at `697a91ae`, as
`--only` slices at the gate tier. The shared-part witness sat; the fourteen-statement twin sat
(the starvation count held); the five sparing kills and the `looked-up-in` region kill sat; the
region law and its `alias nothing-else` kill timed out at 600 s and are the official tier's.
Both sparing laws were red again, through one new shape outside the shared-part hole.

- `red-sparing-through-a-third-thing` — the written mReferent holds a part that no mKey reaches;
  the write affects that part; the part affects the mReferent that holds the read thing; that
  holder affects the read thing. Every closure in force is true, because each speaks one step of
  `affects` (the finished record allows the held part; the may-read closure allows the holder),
  and the law asks the transitive chain. Applied under the hole protocol at `c0487fbe`:
  `hole_a_write_affects_through_a_third_thing` (some a, b, c with b in a's effects, c in b's,
  and c not in a's), its witness at six, both sparing laws, their twins, and the five kills asked
  outside it. Chosen over the narrower "an unkeyed thing carries the effect" because the same
  chain runs through keyed things whose owner's record is not in force and whose mKey is not a
  writeset member, and the narrow hole would miss that world.
- Conductor's reading for the sitting, unposed (the human has no room): this is
  `312d` § 17.1's `ask-affects-and-the-chain` made concrete. Under the reading that `affects`
  is the TOTAL effect of a write (the relation closed under itself, which § 1.1's sentence "A
  write to an mReferent affects the mState of the mReferents it affects" can carry), the written
  thing's finished record is FALSE in these worlds (its write affects the read thing, which it
  neither holds nor entails), so the record's owner is attributable, as 311 intends the knife to
  cut. Under the reading that `affects` is one step, nobody is at fault and § 0's own terms
  refute the model. The hole holds the choice; a ruling for the total reading is one world-stratum
  fact (`^affects in affects`) and the hole's deletion, never the conductor's to write.
- At `c0487fbe` the sparing laws are unmeasured under the new premise; the kills may starve under
  one more conjunct. Leg three measures them and, if quiet, runs the official write at that tip.

## § 13-leg-three-and-the-two-world-shape-holes

2026-09-30. Leg three at `57fbe4ee`: the third-thing witness sat, both twins sat, all five kills
sat; both sparing laws red once more, one shape, outside the four holes; the builder stopped
before the official write, as briefed.

- `red-sparing-through-a-held-thing-off-the-route` — a whole-marked entry's region reaches R0
  one `passes` step down; the line writes R0, which the completion record allows; R0 HOLDS the
  read thing R2 but no route passes to R2, so the placing closure is vacuously true over the
  keyed things of the sort and the region test reads DISJOINT by its second form; R0 affects R2,
  which the read closure allows because R0 holds R2. Every statement true. The world stratum's
  `holds` and `passes` are unlinked, and the region is one `passes` step.
- Applied at `cfd0f37b`, under the hole protocol: `hole_a_held_thing_is_not_reached_beneath`
  (some b holds c and does not pass to c) and `hole_a_route_reaches_beyond_one_step` (a passes
  to b, b to c, a not to c), each with a witness at six; both premised out of the sparing laws,
  their twins, and the five kills. The second is added on a hand-walk, +SURE: with `holds`
  pinned inside `passes` the same world returns as a two-step route the one-step region misses,
  so excluding the first alone buys one round. The seven sparing exclusions are factored into
  one named predicate, `outsideTheSparingHoles`, a pure inlining of the same conjuncts.
- For the sitting, unposed: with the affects hole (§ 12) these are the three world-stratum
  closures the laws presuppose and 311's prose leaves to English: `affects` transitive, `passes`
  transitive, `holds` inside `passes`. Each is a one-line world fact if ruled, and a hole
  deleted; each is `312d` § 17.1's `ask-affects-and-the-chain`, `ask-passes`, and `ask-holds`.
- Leg four re-measures the sparing commands under the seven holes and, if quiet, runs the
  official write at `cfd0f37b` plus this ledger's commit.

## § 14-the-accounting-close-and-the-repairs-it-licensed

2026-09-30. One Opus (worktree `.tmp/trees/r31-accounting-close`, no solver, no edit) closed the
tri-partition at `57fbe4ee`, over the baseline note at `1af7e0d9^` and the whole span of
commits since the last accounting (`a993d795` to `c0487fbe`, nineteen). Tables and scripts:
`tri-partition-close.tsv`, `reverse-close.tsv`, `firewall-close.tsv`, `absent-close.txt`,
`duplicates-close.txt`, copied to the conductor worktree's `.tmp/312e-accounting-close/` and the
session scratchpad, neither durable; whether they become one is the human's (`312d` § 13).

### § 14.1-the-counts

- Of the 770 baseline sentences (§ 0 to § 4): commentary 211; mechanical with a named carrier
  261; structural 32; carried differently or by an inert definition 31; residue 225; residue
  duplicating a mechanized sentence 10; absent 0. § 6 is byte-identical to the baseline; § 5
  differs only by `43b8d4dd`'s vouch-to-axiom rename and the `axiom` kind it added.
- Of the 974 blockquote sentences of the specification (plus the shared half's 8): verbatim 190,
  near-verbatim 147, reword 53, merge 29, move 21; additions of the mechanization 542, of which
  book world 203, book answer 83, kill or twin 61, structural fact 57, truth predicate 34, law
  31, scope or plumbing 24, marked reading 19, hole 11, premise 11, world stratum 6, probe 2.
- Firewall: of the fifty carried findings 29 closed and 21 open (rank 1: 1 open, `compareAt`
  the sole carrier of "reads unknown there" and read by one book only, a coverage gap already
  recorded at `312d` § 12); 23 new (3 at rank 2, 20 at rank 3, the latter mostly commands and
  hole witnesses with no sentence, and glosses with no fence).
- Inert definitions the fences hold and nothing reads: `token`, `parentStore`, `parentCatalog`,
  `parentRefused`, `places`, `disjointSupport`, `traversal` (§ 1.7).

### § 14.2-applied

`67052e63`, one commit, every item a translation-side act or a definitional no-op, none a
change to what any law answers:

- Duplicates struck under the one-copy rule: § 1.10.1's copy of "Differential test discharges
  that axiom, and nobody speaks it"; § 3.1.1's composite-identity sentence (§ 2.11's
  translation carries it); § 2.9.1's "a routing mKey named whole ... stands for whatever its
  mScheme reaches beneath it" (§ 2.9's translation carries it). Three tier-A pairs left as
  residue on purpose: the consumer map's "DISJOINT licenses sparing under the same flag", the
  framing "spares narrowly and collides widely", and the "under ordinary effective-mWorld reach"
  sentence whose qualifier is unmechanized.
- Absent clauses restored: "Any path the dialect admits may decline it" (§ 1.5.1, normative);
  "such as a boot or a tenure" (§ 3.3's examples); "An mValue with no mScheme is nothing"
  (§ 1.4, the fact at `some k.scheme iff no k.cellSort`); "P itself stays an entry of that test"
  (§ 2.6, the seed); the placing-route case of the region test's step 2 (§ 2.9, `coveredBy`'s
  `placedIn` arm, stated as the fence has it: covered whatever the enumeration holds).
- Sentences retuned to their fences: the mParent instance is an mKey and the seats supply one
  instance (F17); a declaration-seat supply is the line of the owner of the mKey's own mScheme
  (N01; see § 14.3); the identity of a primary mScheme's mKey is that mKey on a shape that yields
  nothing (F19); rule 1 is every entry declared for the line, the author clause moved to § 2.6.1's
  residue (F25); a record is emitted for an mKey, not "of T" (F21; see § 14.3); the
  correspondence kill of `compare()`'s DISJOINT names its extra conjunct (N02).
- `fullyQualifiedKey[k]` is `identity[k].*parent` (N03), its one consumer already passing an
  identity, so nothing moves; § 1.8 says a natural mKey's mFullyQualifiedKey is its identity's.
- § 1.9's "The singleton mSort has no mScheme of its own" moved from the translation to a new
  normative § 1.9.1 on the § 1.3.1 precedent (F24): no fence forbids `:primary-of` on a cell
  mSort, and boxing the checker out of that description is design.

### § 14.3-recorded-for-the-sitting-not-fenced

Each is a place where the fences admit a world the prose says does not arise; each could be one
structural fact; none was added, since which worlds the checker considers is design
(`312d` § 21.1) and a fact is the direction that hides counterexamples.

- A `looked-up-in` record is admitted for an mKey of any mSort, with no `:places` declaration
  gating it; `places` is read by nothing (F21). The invocation prose of § 2.10.1 is where the
  gate lives.
- A declaration-seat supply is admitted for a natural mKey, spoken by its secondary mScheme's
  owner, where § 1.6 says a secondary mScheme's third seat is the mEntryChain (N01).
- "Under any one mParent instance it has exactly one mKey" against the fence's at-most-one
  (F23, recorded since `312d` § 12); rule 3 applied at the seed only (F26, likewise); composite
  parts SAME by the walk only (F27, the mark `ask-composite-parts-by-walk`).

## § 15-leg-four-and-the-family-stated-once

2026-09-30. Leg four at `d1e5c5c4` (the builder held before the official write on the
conductor's message; both sparing laws were red anyway): both new witnesses sat, both twins sat,
the five kills sat; the sparing laws red through a fourth shape.

- `red-sparing-through-a-part-in-the-region` — the whole entry's mReferent passes to R0; the
  read thing holds R0 and passes to it; R0 affects the read thing; the line writes R0. Outside all
  seven holes: the shared part is reached by route on one side and held on the other, and both
  world-shape holes are satisfied (R0 is passed to as well as held; the two-step route has its
  direct edge). Every statement true, the region test DISJOINT by the vacuous placing form.
- The family, seen whole after three rounds: a write lands on an mReferent that both the written
  thing and the read thing reach beneath them, by holding or by route, and the engine's speech
  (keys, entries, regions, one-step closures) never names that shared thing. Applied at
  `c9fe5210` under `30Z` § 2.5's rule that the second pin lifts to the species:
  `hole_two_separated_things_reach_one_thing_beneath` (two mKeys `compare()` reads DISJOINT whose
  mReferents' reflexive-transitive `holds + passes` images intersect) REPLACES the shared-part,
  held-thing, and one-step holes; the third-thing (affects) hole stays; `outsideTheSparingHoles`
  is five. By hand, +SURE the new hole covers every world the three did that the sparing law
  needs, and ~SUSPECT it is tied to the counterexample's structure where the two world-shape
  holes excluded any such pair of mReferents anywhere in the world. It is § 5.1's OPEN cell,
  "this mReferent has no other home", stated from the thing's end for the sparing test.
- For the sitting, unposed: the two remaining world-stratum questions under the sparing law are
  `ask-affects-and-the-chain` (the third-thing hole) and the no-other-home cell (this hole); the
  `passes`-transitivity and `holds`-in-`passes` questions of § 13 are no longer load-bearing for
  any law and stay `312d` § 17.1's.
- Leg five re-measures the sparing commands and, if quiet, writes the lock at the tip.

## § 16-leg-five-the-sparing-law-green-and-the-record-that-never-bears

2026-09-30, at `2f6f4350`. `law_sparing_is_sound` no-counterexample at four levels and ten
statements, twin sat, 60 s; the family hole's witness sat; the sudo book green (the
`fullyQualifiedKey` change moved nothing); `law_sparing_is_sound_with_a_store_on_the_chain` a
timeout at 600 s (translated in 273 s, twin sat) and `kill_sparing_is_sound_supplies_parent` a
timeout (translation alone 207 s), both owed to the official ceiling; four kills sat.

- `fnd-the-finished-records-truth-never-bears` — `kill_sparing_is_sound_finishes_entailment`
  unsat at four and ten under the five-hole premise, where it was sat under fewer holes. The
  builder's hand argument, re-derived by the conductor, +SURE at one step: a false sparing needs
  the written mReferent to affect the depended-on one, which the vouch makes the read mKey's own
  mReferent; the read closure, true, then makes the written thing a part of it, a holder of it, or
  an entry's mReferent; part and holder are the one-thing-beneath hole, and an entry sends the read
  mKey into the writeset by rule 4, where it reads SAME with itself and nothing spares. So the
  record's truth is implied by the read closure's outside the hole; its presence still gates
  (⊤ when absent, § 2.6). 311 prices "the premature finished record" as the sparing knife
  (§ 2.6's danger line, § 5.2's row); as mechanized, the knife is the may-read closure, and the
  record's knife lives exactly in the worlds the hole holds open. For the sitting, beside the
  no-other-home cell: the two are one question. The kill keeps `expect 1` and lands as residue,
  the translation already saying what an unsatisfiable kill says.
- No edit follows; the text at `2f6f4350` is the settled text. Leg six is the official write.

## § 17-the-official-lock-and-the-close-of-the-arc

2026-09-30. The from-scratch official write over the settled text ran 3 h 50 m on two children
at 1800 s of CPU per command and exited 0; the builder committed the lock by pathspec. The
conductor's branch fast-forwarded to it and was rebased over `ai/main` (ten sibling commits,
clean), so the lock's header named the pre-rebase tip and a hot-tier `--write` re-pointed it,
solving nothing. This is the last section; the arc stops here, right up to the panel, which is
another lane's (§ 10.1).

### § 17.1-what-the-lock-records

125 rows. No construction disagreement, nothing gone, nothing owed.

- Laws, 66 rows: thirteen checks. Green with a sat twin: `natural_same`, `natural_disjoint`,
  `sparing` (four levels, ten statements), `compare_same`, `same`, `disjoint`, `nobody_spoke`,
  `different_sorts_never_same`, `unstale_route` (six levels, nine statements, no counterexample
  under its four holes and the ⊤ repair, where the old lock had a counterexample),
  `disjoint_by_two_tops_rests_on_one_scheme_owner`. Accepted red: `exclusion_readings_agree`,
  the question whose answer is that the two readings of § 2.6's exclusion disagree. Unmeasured
  at the ceiling, solving: `sparing_with_a_store_on_the_chain` (fourteen statements, five
  levels, twin sat), `region_disjoint` (its premise grew by two holes since it solved in 76 s),
  `compare_disjoint` (never affordable). Every twin sat. Every hole witness sat, nine holes.
  Kills: fourteen sat; five unsat with `expect 1`, landing as residue: the finished record
  (§ 16), `:identified-in` against SAME and against DISJOINT, `:root` against SAME, the sentinel
  against SAME (§ 11.2); two timeouts, `sparing_supplies_parent` (five levels) and
  `region_alias_nothing_else`.
- Books, 59 rows: fourteen books and the corpus, every line, every conjunction, every run
  green, each answer as the conductor hand-walked it.

### § 17.2-the-state-a-successor-inherits

- `ai/main` carries the specification, the shared half, the lock, and this ledger, folded by
  fast-forward; the conductor's and the builder's worktrees and branches are removed.
- The specification at the tip: 4,351 lines, 42 `alloy` fences, 14 books, nine `hole_`
  predicates, thirteen UNACKED READING sites. Every normative sentence is STE-strict at the
  linter (§ 8). The accounting at `57fbe4ee` (§ 14) plus the two commits after it (the
  family hole, the audit repairs) is the tri-partition of record; its tables are not durable.
- The claim this arc hands the panel's lane: the mechanical 311 is, as far as this conductor
  knows, one-to-one with the prose 311 at `1af7e0d9^`, with every departure in one of four
  named places: the thirteen marks (readings hardened by the first pass, `312d` § 7); the nine
  holes (design silences the solver found, each a premise exclusion with an inhabited witness);
  the accounted gaps of § 14.3 and `312d` § 12 (worlds the fences admit that the prose says do
  not arise, left admitted on purpose); and the residue (normative prose the checker cannot
  reach, 225 baseline sentences). Nothing outside those four is a known meaning-shift.

### § 17.3-held-for-the-design-sitting

Nothing here is owed by this ledger's author; it is the field.

- The nine holes, and the two questions they reduce to for the sparing law: the no-other-home
  cell (§ 15) and whether a record's "affects" is the write's total effect (§ 12), with § 16's
  consequence that as encoded the finished record's truth never bears.
- The thirteen marks (`312d` § 7) and the five world-stratum questions (`312d` § 17.1).
- The unsat kills as evidence: `:identified-in`, `:root`, and the sentinel never stand alone
  behind an answer (§ 11.2); the sentinel's is a measurement of the one-atom reading.
- The accounted gaps (§ 14.3): the `:places` gate, the declaration seat for a natural mKey,
  exactly-one against at-most-one for a cell's mKey, rule 3 at the seed, composite parts by the
  walk; and the inert definitions of § 14.1.
- `burn-primary-coherence-sentence` (`312d` § 21.2); `fnd-in-force-is-book-global`
  (`312d` § 19.2); `sus-passes-is-one-step-in-every-truth` (§ 10.4), now load-bearing for no
  law; `sus-a-contained-write-touches-its-store` (`312d` § 22.3), shown by
  2.6.3-a-book-stage-five-the-list-file-named.
- The three ceiling timeouts, each a cost question for the assay owner or a restructuring
  the human calls singly (`312d` § 16.4 S2).
