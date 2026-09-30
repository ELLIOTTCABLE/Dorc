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
