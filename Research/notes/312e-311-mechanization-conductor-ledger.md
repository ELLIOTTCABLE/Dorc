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
  satisfies `hole_composite_keys_with_same_parts_reach_differently`, which the sparing and
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
