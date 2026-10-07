# 312ch — The delta crosscheck over `notes/311`, 6b7108b4 → fc56941e: adjudication, the conductor's own review, and the three lanes

> AI-authored (Fable, 2026-09-26), notes-tier. The record of one focused crosscheck: the changes
> made to `notes/311-identity-and-relation-model.md` between the commit the last panel reviewed
> (`6b7108b4`, adjudicated in `notes/312c`) and HEAD at dispatch (`fc56941e`), and nothing else.
> Three lanes were dispatched in parallel: one Fable (in-lineage, adversarial stance) and two
> GPT-6-Astra (foreign lineage, one neutral, one adversarial), each under the human's six-step
> process (a closed read-in; a skim naming fronts; an empty skeleton; one reasoning turn and one
> write per front with no new reads; then a comparison pass with the tree open; then free rein).
> § 2 is the conductor's own review, written after dispatch and before any lane returned, so that
> it stands falsifiable beside theirs. § 1 and § 3 to § 5 are reserved and filled as the lanes
> return. Grades on the conductor's claims: +SURE / ~SUSPECT / -GUESS / --WONDER. **[HUMAN]** marks
> the human's framing, paraphrased from chat, never a ruling. Nothing here is ruled. No edit was
> made to 311 by this document's author during the crosscheck.

## § 0-conduct-and-scope

- Scope, as dispatched: the diff to 311 between `6b7108b4` and `fc56941e`; a finding points at
  changed text or at an interaction between changed text and the text it touches. Normative
  material is the focus; by 311's own conventions the blockquotes and § 5 are non-normative.
  The four open holds of `312cg` § 3 were neither named to the lanes nor fenced out.
- The lanes' closed read-in (step 1): root `README`, `DESIGN`, `IMPLEMENTATION`, `USER_STORY`,
  `KNOBS`, `AGENTS`, `spike/CLAUDE.md`, `Research/GOTCHAS.md`, then 311 whole, then the patch.
  No ledger, no prior panel report, no `311u` until step 5.
- Mechanics: the Fable lane ran natively over the real tree, read-only by instruction. The two
  Astra lanes ran `codex exec -s workspace-write` inside disposable scratch copies of the root
  docs, `spike/CLAUDE.md`, `Research/notes/`, and `Research/plans/`, so that each could write its
  report incrementally; the stock codex-reviewer shim's read-only sandbox cannot honour the
  incremental-write discipline. The repository was not written by any lane.
- The conductor's bias, stated: unlike the lanes, the conductor holds the ledgers (`312cg`,
  `311q`, `311t`) and the prior panel (`312c`) in context, and knows the four open holds. § 2
  was written from the patch and the current text, before any lane returned.

## § 1-synthesis-and-adjudication

Written after all three lanes returned, over § 2 to § 5. Every credited item was re-walked
against 311's text in § 2 to § 5; a lone finding is credited only where the re-walk holds.
Convergence across lineages weighs above eloquence, with one correction to that rule below:
the two Astra lanes are one lineage, and Fable's post-hoc agreements are not finds.

### § 1.1-the-answer-to-the-question-asked

**[HUMAN]** asked whether the tunes since the last panel broke anything new. They did, in five
places, and they surfaced one older hole the last panel missed. None of the six is a wrong answer
on the text's only reading; one is a wrong answer on the text's only reading and predates the
freeze; three are wrong answers on a reading the text permits and does not exclude; two are
losses of value, one of them large enough to be the round's headline. The four open holds of
`312cg` § 3 were not touched by any lane and stand as held.

### § 1.2-the-convergence-matrix

Rows are the threads of § 1.3 and § 1.4; a cell says what that view did. "found" is an
independent finding; "nit" a low-graded independent note; "agreed" a post-hoc agreement after
reading another view; "dead" a considered-and-killed entry; blank is untouched.

| thread | conductor § 2 | fable-a § 3 | astra-n § 4 | astra-a § 5 |
|---|---|---|---|---|
| exclusion keyed on the container drops interior effects | | found | | |
| stand-in drops P | nit (additive reading) | found (non-empty witness) | found (empty witness) | found (empty witness) |
| unmarked verdict readset closed-empty | found | agreed | | |
| open read set: "any routing mutation" undecidable | | found | | |
| injection sentences collide every fact | found | passed, rescinded on reading | found | found |
| cross-world entry collides its own world | found | agreed | | |
| entailment naming the parent leaks two levels | found | | | |
| closure count, § 1.5 against § 2.9 | | found | | |
| within-sort finished record against the register gloss | found | agreed | | |
| generated-key totality sentence | nit | agreed | found | found |
| § 2.9 "Danger: none" against the closing act | | agreed | found | found |
| least-set with a pair-relative exclusion | nit | nit | found (witness unconfirmed) | found (witness unconfirmed) |
| § 1.10 "vouches"; § 3.4 danger line omits the MRoute | | found | nit (MRoute) | |
| § 3.3 "is in the read set" literal | | found | found | |
| register and index cells | nit | found | found | |

### § 1.3-survivors-ranked

A wrong SAME or DISJOINT reachable with every statement true:

1. `srv-exclusion-keyed-on-the-container` (§ 3, Fable; lone; conductor CONFIRMED; +SURE of the
   text, ~SUSPECT of frequency) — the write-side exclusion in `311:2.6-may-write-the-writeset`
   discards a store owner's true entailment naming an interior sibling, for every pair inside
   that store, because it is keyed on the entailing container and not on the entailed member's
   position relative to the shared level. The quota-file witness spares a fact past a write
   that changed it with no false statement behind it. Not a reading: the sentence is
   unambiguous. Predates the freeze in substance; the last panel killed a neighbouring
   construction (`312cb` § 4, the free-space cell) and missed this shape. The same keying is
   the root of § 2.1's `fnd-entailment-naming-the-parent-leaks-two-levels-up` (over-collision two
   levels up), so one re-keying fixes a wrong spare and an over-collision at once.
2. `srv-stand-in-drops-p` (§ 2 nit, § 3, § 4, § 5; four views; CONFIRMED; +SURE the literal
   reading is unsafe) — `311:2.10-places-the-upward-lookup`'s "its emitted members stand in for
   P given whole" reads literally as replacing P, so P's own cells and aliases leave the sparing
   test. The Astra lanes' empty-enumeration witness and the conductor's and Fable's non-empty one
   (`dpkg -s nginx` spared past `apt-get remove nginx` with the dpkg store keyed as an inode)
   both reach a wrong spare with every statement true. The intended reading (the members stand
   in for the reach beneath P, rule 3; P stays) is what `312cg` § 15 recorded and the text does
   not say. New text.
3. `srv-unmarked-verdict-readset-closed-empty` (§ 2; Fable agreed; CONFIRMED on the text;
   ~SUSPECT it is 311's to close) — § 2.5's "A fact's readset is the marked reads of the body
   that answered it. For a verdict fact, the vouch closes them" gives an unmarked in-book
   verdict a closed empty readset, and "only when every pair is DISJOINT" is vacuous over zero
   pairs. The inherited pin (⊤ is never the empty set) forbids the reading from outside 311;
   § 1.11's definition of a verdict fact as a read of a cell arguably keeps unmarked bodies out
   of the model; the sentence a builder reads says neither. New text (`fc34936b`, `fc56941e`).
4. `srv-open-read-set-any-routing-mutation` (§ 3, Fable; lone; conductor CONFIRMED the
   ambiguity, PLAUSIBLE the wrong SAME) — `311:1.7-resolution-and-its-traversal`'s "Any routing
   mutation invalidates a MResolution whose read set is open" names a category the engine cannot
   decide for an open set (the third clause of what a routing mutation touches is knowable only
   through that set); under the decidable clauses a modelled content write to what an
   undescribed tool read leaves the MResolution standing and the warrant licenses a wrong SAME,
   flag-gated. The ledger's intent is ⊤ (`312cg` § 10); § 2.5's fact-side twin already says
   "every write". New text.

A lost elision or a monotonicity break:

5. `srv-injection-sentences-collide-every-fact` (§ 2, § 4, § 5 independently; Fable passed then
   rescinded; CONFIRMED; +SURE) — the round's headline by convergence and by scale. § 2.5's
   "every MKey of that MSort is in every line's writeset", rule 4's last sentence, and § 1.10's
   "Every MKey scoped in it is in every line's writeset" put every stranger's and every unrooted
   key into every line's writeset, where each compares UNKNOWN or KNOWN_UNSPOKEN against every
   rooted fact and fails every universal. Installing USER_STORY's own stage-4 `foobar` oracle
   kills stage 5's survivals. The backward form (`c5e8f434`) put ⊤ on the read side and did not
   do this; `312cg` § 28's "no object, relation, or answer changed" is contradicted;
   `26M:law-monotone-enhancement` is broken. Never a wrong spare. New text (`fc56941e`).
6. `srv-cross-world-entry-collides-its-own-world` (§ 2; Fable agreed; CONFIRMED; ~SUSPECT of
   weight) — § 2.5's new permission for a may-read entry keyed in another MWorld, under rule 4's
   non-DISJOINT trigger, pulls the declaring key into the writeset of every write in its own
   MWorld whenever the two MWorlds' MRoots are of different shapes; one honest additive entry
   ends intra-world sparing for its sort. Harmless where both worlds share one MRoot shape. New
   permission; the mechanism predates.
7. `srv-closure-count-one-level-against-every-level` (§ 3, Fable; lone; CONFIRMED textual) —
   § 1.5's "for the one level it resolved" against § 2.9's "every level emitted its closure"; the
   narrow count denies every multi-level path region its DISJOINT.
8. `srv-within-sort-finished-record-against-the-gloss` (§ 2; Fable agreed; CONFIRMED) —
   § 2.6 demands a reached finished record for every written shape, within a sort as well; the
   `30U` register entry says "across MSorts". The text is safe and deliberate (`312cg` § 15); the
   register misdescribes it.

Text a builder must guess at, and register:

9. `srv-generated-key-totality` (§ 4, § 5; conductor nit; Fable agreed; CONFIRMED) — § 2.2's
   "holds an MReferent under every MKey a book can name" is false for a deleted or stale
   generated key; the "or none" sentence is the rule.
10. `srv-danger-none-against-the-closing-act` (§ 4, § 5; Fable agreed; CONFIRMED) — § 2.9's
    retained "Danger: none" beside § 1.7's new authored closing act, whose falsity § 5.2 prices
    as stale.
11. `srv-least-set-with-a-pair-relative-exclusion` (all four views; ambiguity CONFIRMED; no
    witness in hand) — the line-global least set against the pair-relative carve; both Astra
    witnesses fail because rule 4 fires on the written key against the disk directly under either
    order; the conductor's free-space walk collides under both readings. A definitional
    clarification, safe either way.
12. `srv-tier-and-membership-wording` (§ 3, § 4) — § 1.10's "vouches" for the inherited instances
    (the engine named as voucher of a sentinel-and-flag claim); § 3.4's danger line naming catalog
    sorts and not the MRoute the inheritance sentence also inherits; § 3.3's "is in the read set"
    where the traversal clause says "other than DISJOINT".
13. `srv-register-and-index-cells` (all views) — § 4.2: the `plans/30W` index-kinds entry still
    says "nothing speaks across MWorlds"; the `30T` v0-floor entry still says "routing writes to
    its MParent-Catalog entry"; `rul-flag-is-razor-residue` and the human-opted
    `pure-predicate-carry` now read false and are unnamed; USER_STORY's stage 5 and stage 7
    renders are contradicted and unnamed. § 5.2: the finished record's "decl"; the three INVAL
    cells' "no"; § 5.1's sentinel row without a flag.

### § 1.4-raised-and-dead

Killed by at least one view and checked by the conductor: wrapper inheritance as an all-true
wrong SAME (§ 2.8's default observer dependence; every lane); the accepted floor that a chain
ending at the MRoute spares nothing (all lanes withdrew it as a defect after `311u` and `312cg`
§ 24); the empty traversal proving separation (the spoken-closure requirement blocks it); two
partial placing answers wrongly refused (records accumulate); cross-world correspondence
manufacturing equality or spurious contradiction (an explicit vouch-tier premise; the strongest
warranted answer stands); laundering a flagged SAME through an unflagged chain ("rests on" covers
the composed derivation); invalidation's DISJOINT step unflagged (no unflagged consumer can act on
a kept MResolution); missing lookup dependencies (the body-read closure and open-set invalidation
cover the `cat`-selected-target case); negative existence inexpressible (a directory fact or a
minted name-keyed referent); the example demotions as soundness loss (none; two lose guidance);
routing writes as the verb author's at-most claim (every honest combination collides); the
nested-backing case under the forward rules (collides through step 2, § 2.2's first check).

### § 1.5-candidate-edits-held-for-the-human

Per `312cg` § 1, no edit in this sitting; **[HUMAN]** "no new items taken". Listed by what each
does to the model, so the human can take them by class.

Clarifications that withhold no sparing (the `312cg` § 1 class, applicable on a word):

- `edit-stand-in-keeps-p` (item 2): § 2.10, "its emitted members stand in for the MKeys reached
  beneath P (§ 2.6, rule 3); P remains an entry of the test."
- `edit-any-write-invalidates-an-open-set` (item 4): § 1.7 and § 3.3, "Any write invalidates a
  MResolution whose read set is open"; § 3.3's read-set clause takes the traversal clause's
  "other than DISJOINT".
- `edit-closure-per-level` (item 7): § 1.5, "for each level it resolved".
- `edit-strike-the-totality-sentence` (item 9): § 2.2, strike "A store that generates its MKeys
  at creation holds an MReferent under every MKey a book can name", or read "under every MKey it
  has generated".
- `edit-danger-line-names-the-closing-act` (item 10): § 2.9, "Danger: a false closing act keeps a
  stale MResolution and every conclusion on it. An open or coarse emission is safe."
- `edit-tier-wording` (item 12): § 1.10 "supplies" for "vouches"; § 3.4 "over every
  MParent-Catalog MSort and the MRoute"; § 2.6 one sentence on whether the exclusion applies at
  construction or at the test (item 11).
- `edit-register-and-index` (items 8, 13): the six § 4.2 entries and three § 5 cells named above.

Changes to what the model answers (the human's, each a ruling):

- `rule-top-on-the-read-side` (item 5): drop the three injection sentences; state on the read
  side that a readset member is ⊤ when any MSort on its MFullyQualifiedKey, or on the
  MFullyQualifiedKey of any may-read entry rule 4 reached it through, has no closed may-read
  set, or when that chain ends at the MRoute. The leaf's sort alone is not enough: the
  backward form's "closed only when every set that joined is closed" is what the condition must
  reproduce, and rule 4's first sentence keeps the transitive propagation. Restores the
  backward form's answers and monotonicity; withholds nothing the backward form spared.
- `rule-exclude-by-the-entailed-member` (items 1 and § 2.1's two-level leak): § 2.6, "An
  entailed MKey at or above the deepest level that the written MKey shares with the read MKey
  contributes nothing to the test against that fact; an entailed MKey below that level is a
  member." Keeps the disk case the exclusion was written for; restores the quota collision;
  removes the two-level over-collision; makes the footer's parent-naming pattern harmless at
  any depth. Removes a sparing the text grants, so it is the human's under `312cg` § 1.
- `rule-unmarked-body-readset-is-top` (item 3): § 2.5, "A body that marks no read has readset
  ⊤", or an explicit sentence that an unmarked verdict is outside this model and the universal
  meet's pin governs it. Removes a sparing the text grants on one reading.
- `rule-cross-world-entry-cost` (item 6): state beside the permission that an entry keyed in
  another MWorld collides with every write in its own MWorld unless both MWorlds are one MRoot
  shape; or carry cross-world dependency as the transition owner's speech with its own consumer;
  or accept the cost in writing.

### § 1.7-applied-and-held

**[TYPED]** 2026-09-27, the human's rule: apply the clear and obvious fixes the human wants no say
in, where the edit licenses no new sparing (undiscussed sparing; restoring a sparing the ledger
made precise is not new) or was found by several lanes independently before step 5.

Applied to 311, one commit per theme (`312cg` § 29 carries the mapping): items 5 and 3
(`1a5fe87a`), 2 (`55697925`), 4 and the § 3.3 membership wording of 12 (`d2a548c5`), 7 and 10
(`b6253e4d`), 9 (`c20bbf4d`), the § 1.10 and § 3.4 wording of 12 (`47ca5ead`), the § 4.2
index-kinds and v0-floor entries and the § 5 cells of 13 (`64feb9f0`).

Held for the human: `rule-exclude-by-the-entailed-member` (item 1 and § 2.1's two-level leak; a
lone finding on a cut the human defended, `312cg` § 24 and § 25); `rule-cross-world-entry-cost`
(item 6); item 8 (the `30U` gloss against the within-sort demand; which is meant was not
discussed); item 11's construction-against-test clause (entangled with item 1); item 13's two
`spike/CLAUDE.md` register entries and the USER_STORY stage 5 and 7 lines (the latter pre-empt
`312cg:hold-stdlib-key-space-over-the-boots-children`).

### § 1.6-process-notes

- Mechanics: both Astra lanes ran `-s workspace-write` in scratch copies at the first attempt,
  no ACL repair needed, 16 and 21 minutes; the Fable lane took 57 minutes and about 485k tokens
  over 41 tool calls. The six-step process held in all three: every report shows its skeleton,
  its per-front conclusions before comparison, and its comparison-pass dispositions with
  rescissions kept.
- The Fable lane read this document at step 5, as the process allows. Its ten fronts are
  independent; its agreements are not. It found the round's three lone survivors that carry
  danger (items 1, 4, 7) and missed the round's headline (item 5) by checking only the cases the
  rewrite was built for; it says so itself. The calibration datum stands: a single lane confirming
  an equivalence is weak evidence; three views on one sentence is the signal.
- Astra over-flagged nothing this round; both lanes rescinded their route-floor variant on
  reading the register and graded their unsafe-reading findings ~SUSPECT with the safe reading
  named. Their least-set witnesses did not survive the conductor's re-walk; their ambiguity did.
- The conductor's § 2, written blind, agreed with the lanes on the headline and on two textual
  items, under-graded two items the lanes ranked higher (the stand-in, the totality sentence),
  and missed items 1, 4, 7, 10, and the § 3.3 membership wording.

## § 2-the-conductor-review

Written 2026-09-26 between dispatch and the first return. Each finding names the changed
sentences, the section, a walk, a graded conclusion, and what would refute it. Ordering: kind
first (a wrong answer reachable; a lost elision; text a builder must guess at), consequence
second.

### § 2.1-findings-ranked

#### fnd-open-sort-keys-in-every-writeset-collide-every-fact

- Kind: a changed answer; lost elision at product scale; a monotonicity break. Grade: +SURE of
  the text; +SURE that the backward form (`c5e8f434`) answered differently; ~SUSPECT the
  authors intended the narrow reading below.
- Changed text, all `fc56941e`: `311:2.5-may-read-the-readset`, "An MSort that declares no
  may-read set, or declares one and does not close it, is affected by every write: every MKey
  of that MSort is in every line's writeset." `311:2.6-may-write-the-writeset`, rule 4, last
  sentence: "Every MKey of an MSort with no closed may-read set is in every writeset."
  `311:1.10-vantage-route-placeholder-witness`: "Every MKey scoped in it [the MRoute] is in
  every line's writeset."
- The walk. USER_STORY stage 5's book with the stage-4 `foobar` oracle loaded. `org.foob.Certs`
  declares no `:identified-in` (so its MKeys are scoped in the MRoute, § 1.6) and no may-read
  set. The stdlib has rooted the boot, so the dpkg store's chain ends at the boot MRoot.

  ```sh
  apt-get update                                               # 5  drifted: runs; list inodes, closed; apt's record reached
  dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx    # 6  converged; readset: the nginx package key
  foobar sync-certs "$CERTS"                                   # 8  converged; org.foob.Certs:/etc/nginx/certs
  ```

  Line 5's writeset contains, by the § 1.10 sentence, every MKey scoped in the MRoute, so
  `org.foob.Certs:/etc/nginx/certs`. The sparing test for line 6 runs over every pair of a
  writeset member and a readset member "of one MSort or of two". `compare()` of the Certs key
  against the package key: one chain terminates at the MRoute, the other at a MRoot the first
  does not share, so `311:3.2-compare-one-chokepoint-four-answers` step 1 answers UNKNOWN.
  Not DISJOINT, so line 6 collides and guards. Every fact in the book collides with every
  line, whatever its describers declared, because the stranger's keys sit in every writeset
  and are UNKNOWN against every rooted key. The same holds for any MSort with no closed
  may-read set, by the § 2.5 sentence: a vendor's `org.vendor.Widget`, undeclared, sits in
  every writeset and is KNOWN_UNSPOKEN or UNKNOWN against every other sort's fact.
- Under the backward form (`c5e8f434`) the fact's readset held its own chain's entries and
  nothing of the stranger's; ⊤ appeared only at an undeclared member of that chain. Line 6
  spared. So the forward rewrite changed an answer, against the ledger's own summary of it
  (`312cg` § 28: "No object, relation, or answer changed"). It also breaks
  `26M:law-monotone-enhancement`: installing the stage-4 oracle removes stage 5's survivals.
- Through `311:3.3-invalidation-three-mutator-species` the same sentences make every traversal
  member whose MSort has no closed may-read set touched by every line, so every resolution
  through such a member reads unknown below every line.
- The narrow reading the sentences were reaching for (conductor's inference from `312cg`
  § 24 and § 28): a fact ABOUT a key of an undeclared sort, or about a key scoped in the MRoute,
  collides with every line. That reading puts ⊤ on the read side: a readset member whose MSort
  has no closed may-read set, or which is scoped in the MRoute, is ⊤, and ⊤ is DISJOINT from
  nothing. Rule 4's first sentence then still carries the chain: an undeclared ancestor's keys
  need not be injected, because a fact under it already collides through the ancestor's own
  membership when a write reaches it.
- Repair candidates, the human's call: move the three sentences to the read side as above; or
  restrict them to "every MKey of that MSort that appears in a readset".
- Falsifier: a sentence in 311 that keeps undeclared-sort keys out of the pairwise test against
  facts of other sorts, or that makes `compare()` of an unrooted stranger key against a rooted
  key DISJOINT. Neither is found: § 2.6 runs the test over pairs "of one MSort or of two", and
  § 3.2 step 1 reads unshared termini as UNKNOWN.

#### fnd-unmarked-verdict-readset-reads-closed-empty

- Kind: a wrong spare reachable on one reading of the text; the reading is a builder's, and an
  inherited pin outside 311 forbids it. Grade: +SURE the text admits the reading; ~SUSPECT it is
  311's to close (§ 4.1 hands unmarked reads to `plans/27C` and the universal meet).
- Changed text: `311:2.5-may-read-the-readset`, "A fact's readset is the marked reads of the
  body that answered it. For a verdict fact, the vouch closes them (`KNOBS:kCONTRACT-RUNGS`).
  May-read entries are not in a readset." The vouch sentence is `fc34936b`; the removal of the
  inherited entries is `fc56941e`. Consumer: `311:2.6-may-write-the-writeset`, "An elision is
  spared past a line only when `compare()` answers DISJOINT for every pair of a writeset member
  and a readset member".
- The walk. USER_STORY stage 3's in-book oracle carries no marks:

  ```sh
  foobar__is_converged() {
     [ "$1" = sync-certs ] && [ $# -eq 2 ] || return 2
     foobar status --certs-current -- "$2"
  }
  ```

  Its fact's marked reads are the empty set. The vouch closes the empty set. Below a running
  line with a finished writeset (stage 5's `apt-get update`), the universal over zero pairs
  holds. A builder who implements "spared iff every pair is DISJOINT", which is the only
  sufficiency anyone will implement from "only when", spares `foobar sync-certs` past every
  line on no statement by anybody. The inherited pin says otherwise: ⊤ is never encoded as the
  empty set (`notes/277` § 5; `spike/CLAUDE.md` set-lifting-universal-meet;
  `ANALYZER-NEEDS:an-backing-selfframing`, the own cell always a member). 311's § 1.11 defines
  a verdict fact as the answer to a read of a cell, so an unmarked body arguably yields no 311
  fact at all, and the verdict tier alone decides its own-site elision. The text a builder reads
  says neither. This is the shape the last panel found in the region test's empty universal
  (`312c` thread 6) and the human ruled on (`312cg` § 12: an empty set read as vacuously safe
  is never a default without a stated reason).
- Repair: one sentence in § 2.5. Either "A body that marks no read has readset ⊤", or "the
  readset of a verdict fact always contains the MKey of the cell its marked line answers; a body
  with no marked line answers no cell and has readset ⊤".
- Falsifier: a sentence in 311 that gives an unmarked body a non-empty or ⊤ readset. None is
  found. `311:1.9-cell-a-singleton-sort`'s "the marked line that answers a cell is a read of the
  cell's MKey" speaks of marked lines only.

#### fnd-cross-world-entry-collides-its-own-world

- Kind: lost elision, total for the declaring sort in its own MWorld, from one honest additive
  statement. Grade: +SURE of the walk; ~SUSPECT of weight (it needs the two worlds' MRoots to
  be of different shapes; the hypervisor case with two `sm.Boot` roots is safe).
- Changed text: `311:2.5-may-read-the-readset`, "An entry may name an MKey of another MWorld.
  `compare()` decides that pair as 3.2-compare-one-chokepoint-four-answers decides any pair."
  (`c5e8f434`), consumed by `311:2.6-may-write-the-writeset` rule 4 (`fc56941e`) and by § 3.2
  step 1, "MRoots of two shapes are two MWorlds ... No MFullyQualifiedKey speaks across MWorlds".
- The walk. A guest VM on a cloud host. The guest's filesystem `F_guest` is identified in its
  virtual disk, identified in the guest's `sm.Boot` MRoot. The hypervisor's describer, per
  `312cg` § 26, declares that the guest's disk may-read the host file `x.qcow2`, an inode in a
  host filesystem identified in a cloud `sm.Volume` MRoot. Any guest-side line that writes
  anything (`cp ./app.conf /etc/app/app.conf` inside the guest) has a writeset member
  `w_guest`. Rule 4: `compare(w_guest, host qcow2 inode)`: two MRoots of two shapes, so
  UNKNOWN, so not DISJOINT, so the guest disk's key joins the writeset. Then the test against
  any guest file fact `r`: `compare(guest disk, r)`: the disk is an ancestor of `r`, so § 3.2
  step 2 answers UNKNOWN. Collide. Every guest write collides every guest file fact from the
  moment the honest cross-world entry is declared. Where both worlds are one MRoot shape with
  `:guarantees-unique-name` (two `sm.Boot`s), the pair separates as siblings and the entry is
  harmless; the text does not say the entry's value depends on that.
- Not a regression of the rewrite: under the backward form the entry sat in the inherited
  readset and compared UNKNOWN against every guest write likewise. The permission to name a
  cross-world entry is what is new.
- Repair candidates: state the condition beside the permission (an entry keyed in another
  MWorld collides with every write in the entry's own MWorld unless both MWorlds are one MRoot
  shape); or carry cross-world dependency as the transition owner's speech, with its own
  consumer, rather than as a may-read entry.
- Falsifier: `compare()` answering DISJOINT for a pair whose chains end at MRoots of two shapes.
  § 3.2 step 1 forbids it.

#### fnd-entailment-naming-the-parent-leaks-two-levels-up

- Kind: lost elision; borderline scope (rule 2 restates semantics that predate the freeze; the
  footer example moved). Grade: +SURE of the walk; ~SUSPECT that stdlib describers would write
  the two-level pattern.
- Changed text: `311:2.6-may-write-the-writeset` rule 2, "Where an MKey of K is in the
  writeset, or an MKey identified beneath an MKey of K, every MKey that K's may-write entailment
  names is in the writeset", with the unchanged exclusion "A container at or above the deepest
  level that the written MKey shares with the read MKey contributes no entailment to the test
  against that fact", and the footer, "a filesystem's entailment, which names its disk".
- The walk. Under `312cg` § 26's rooting a filesystem is identified in its block device. Two
  filesystems `F` and `G` sit on two partitions `B` and `B2` of one disk `D`. Each describer
  follows the footer's pattern one level up: Filesystem's entailment names its partition;
  Partition's entailment names its disk. Write `w`: an inode in `F`. Fact `r`: an inode in `G`.
  Shared level: `D`. Rule 2: `w` is beneath `F`, so `B` joins; `B` is strictly below `D`, so its
  entailment contributes and `D` joins. The exclusion removes `D`'s own entailment (`D` is at
  the shared level) and not `D`'s membership. `compare(D, r)`: `D` is an ancestor of `r`, so
  step 2 answers UNKNOWN. Collide. Every file write on one filesystem collides every file fact
  on every other filesystem of the same disk. One level up alone (`B` in the writeset), `B` and
  `B2` separate as two partitions of one scheme, so the footer's pattern is safe at one level
  and leaky at two. § 2.5 tells a describer that the MParent is no may-read entry; nothing tells
  them that the MParent is no entailment member, and the footer names it.
- Repair: a sentence in § 2.6 mirroring § 2.5's, that an entailment need not name the written
  MKey's MParent, since the walk's step 2 already collides a write at or above it; the footer
  example then changes.
- Falsifier: text removing `D`'s membership. The exclusion removes an entailment, not a member.

#### fnd-finished-record-demanded-within-one-sort

- Kind: lost elision; a register gloss that misdescribes the text. Grade: +SURE of the
  mismatch; the demand itself predates the freeze and the human restored its strict form
  (`312cg` § 15).
- Changed text: `311:2.6-may-write-the-writeset`, "An unclosed at-most set or an unfinished
  entailment puts ⊤ in the writeset", and rule 2's "For each origin cell in the writeset, a
  reached finished record for that cell's MSort and shape finishes the entailment". The § 4.2
  entry for `30U` says "a finished definition stays necessary for sparing across MSorts".
- The walk. Two `cp` lines into one filesystem, the binder's at-most set closed, and `sm.File`'s
  owner has written no record for the inode shape. The entailment is unfinished, so ⊤ is in the
  first line's writeset, so the second line's fact collides although the pair is two inodes of
  one scheme under `:guarantees-unique-name`. `plans/30U` § 1 left within-kind comparison
  untouched by the finished-definition gate; 311 demands the record for every written shape.
  Safe, and deliberate per the ledger; the register entry reads narrower than the rule.
- Repair: either the register entry says "for sparing past any write, within a sort as well",
  or rule 2 exempts an origin cell whose sort declares no entailment. The human's call.
- Falsifier: text exempting an absent entailment from "unfinished". `30U`'s rung 1 makes the
  record's absence informative, hence unfinished.

#### nit-rule-three-and-the-finished-stand-in-both-apply

- Kind: text a builder must guess at; safe either way. Grade: -GUESS it matters.
- `311:2.6-may-write-the-writeset` rule 3 says every MKey reached beneath a whole-marked member
  is in the writeset; `311:2.10-places-the-upward-lookup` says a finished enumeration's members
  "stand in for P given whole in the test of §2.6". A builder applying both collides more (safe);
  applying only § 2.10 is the intent. One clause in rule 3 closes it.

#### nit-least-set-with-a-pair-relative-exclusion

- Kind: text a builder must guess at; safe either way. Grade: -GUESS.
- § 2.6 defines "the least set that four rules close" per line, then carves an entailment per
  pair ("to the test against that fact"). Whether the carve applies at construction (the excluded
  keys then seed neither rule 3 nor rule 4 for that fact) or only at the test is unstated. Both
  readings were walked on a filesystem's free-space cell that may-reads the block device: the
  cell collides under both, because the written inode itself compares UNKNOWN with the device.
  One sentence either way.

#### nit-generated-store-holds-every-key-a-book-can-name

- Kind: wording. Grade: -GUESS it bites.
- `311:2.2-primary-of-and-identified-in`, "A store that generates its MKeys at creation holds an
  MReferent under every MKey a book can name." A book can name a stale or mistyped generated key
  (an inode number captured before an `rm`). The honest sentence is "under every MKey it has
  generated".

#### nit-register-and-index-hygiene

- Kind: non-normative and register. Grade: +SURE of the omissions.
- `311:4.2-supersessions-pending-in-prior-documents` now admits root-document entries and carries
  one for USER_STORY's bought-unsoundness sentence, and none for the stage 5 render (a
  memory-held fact against a file write never separates under the current text, `312c` thread 3)
  nor the stage 7 sentence ("in any vocabulary, including ones I have never heard of").
- `311:5.2-by-what-a-false-statement-costs` lists the finished record as built "decl"; § 2.6
  says "The reached completion record finishes the definition", which is eval.

### § 2.2-checks-that-held

Each walked against the current text; recorded so the lanes' findings can be compared.

- `chk-nested-backing-collides-under-forward-rules` (+SURE) — the `312cg` § 25 counterexample
  under `fc56941e`'s rules: `dd` over the outer image `i1`; rule 4, `F1`'s entry `{i1}` is SAME,
  so `F1` joins; rule 4 again, `F2`'s entry `{i2}` with `i2` in `F1`: `compare(F1, i2)` is
  UNKNOWN (parent), so `F2` joins; the test against the inner fact `r` in `F2`: `compare(F2, r)`
  is UNKNOWN, collide. The forward form reproduces the transitive closure through § 3.2 step 2
  with no special rule.
- `chk-unclosed-ancestor-set-collides-its-descendants` (+SURE) — an ancestor filesystem `F`
  with an unclosed may-read set puts every Filesystem key in every writeset; `compare(F, r)` for
  `r` in `F` is UNKNOWN; collide. The backward form's "closed only when every set that joined is
  closed" is reproduced for chain members. Its over-reach beyond chain members is the first
  finding.
- `chk-sentinel-shared-mroute-spares-nothing` (+SURE) — under the sentinel and the flag two
  MVantages share the MRoute, so two-tops separation at the MRoute would license DISJOINT across
  a wrapper on a SAME-side claim; but every MRoute-terminated readset member is in every writeset
  (§ 1.10 as written) or ⊤ (under the first finding's repair), so no such fact spares and the
  danger line's "wrong SAME" is the sentinel's only consequence.
- `chk-region-test-second-form-uses-accumulated-records` (+SURE) — step 3's "with no
  `looked-up-in` record" reads over the union of invocations per § 2.10's accumulation sentence;
  a decline emits no closure and falls to step 4.
- `chk-unclosed-traversal-floor-member-blocks-disjoint` (+SURE) — a lookup that emits members
  without the closing act keeps its MParent-Catalog given whole as a level; that level carries
  no `alias nothing-else`; step 3 fails; UNKNOWN. A lookup cannot buy DISJOINT with per-level
  closures while withholding the traversal's closing act.
- `chk-top-in-writeset-invalidates-every-resolution` (+SURE) — ⊤ is DISJOINT from nothing, so
  an unmodelled line touches every traversal member and invalidates every resolution below it:
  the poison wall extended to identity, consistent with kill-reach.
- `chk-parent-write-covers-cells-by-step-two` (+SURE) — § 1.9's "A writeset entry naming the
  MParent covers its cells (§3.2, step 2)": the cell's chain is cell then parent; the writeset
  member is the parent, which is the deepest SAME level itself; UNKNOWN; collide. Dropping
  "whole" was right.
- `chk-fence-narrowing-is-consistent` (+SURE) — § 1.8, § 3.2 step 1, and § 2.6 agree that only a
  MCorrespondence crosses MWorlds; a cross-world may-read entry is a key compared by § 3.2 and
  never yields SAME across, so it does not breach the fence.

### § 2.3-accepted-costs-restated

Not holes. Stated so their scale is visible beside the holds.

- `cost-open-lookup-read-sets-invalidate-everywhere` (+SURE of the text; acked `312cg` § 11) —
  `311:1.7-resolution-and-its-traversal`: a lookup body's read set is closed only when every
  external command's read set is closed by that command's describer, and "Any routing mutation
  invalidates a MResolution whose read set is open." No describer has closed `stat`,
  `dpkg-query`, `readlink`, or `getent` today, so every routing mutation (an `export`, a `cd`, a
  `mount`, a `useradd`, any unmodelled line) invalidates every resolution below it, and no SAME
  or DISJOINT survives that line. Safe. Its scale: identity survives nothing past the first
  `export` in any book until the stdlib closes the read sets of the commands its lookups run.
- `cost-every-written-sort-needs-a-finished-record` — the fifth finding's demand, restated as
  a cost: within-sort sparing waits on the written sort's finished record for the shape written.
- `cost-unrooted-facts-spare-nothing` (+SURE; the § 24 litmus) — a fact whose chain ends at the
  MRoute spares past nothing; value begins where the stdlib declares roots and stores.

### § 2.4-out-of-scope-one-line-each

- `311:2.9-the-traversal-and-the-region-test`'s floor sentence, "reads UNKNOWN against every
  MKey that MScheme can yield in the same MParent-Catalog instance": for a path the catalog
  instance is a directory, so the sentence protects siblings and not descendants; the region
  test's step 2 covers descendants regardless, so the sentence is inert as written. Text that
  predates the freeze.
- `311:2.7-corresponds-across-a-transition`, "the model's only declared sameness generator
  besides MToken equality", beside § 3.2's "Two MSchemes yielding one MKey-Primary is the sole
  same-referent generator across ways of naming": two "only"s naming different generators,
  reconcilable if a yield into one primary counts as MToken equality. Predates the freeze.

## § 3-lane-fable-adversarial

Provenance: Claude Fable 5.1, in-lineage, adversarial stance; native, over the real tree,
read-only; ten fronts written and locked before its comparison pass. At step 5 it read `311u`,
`312cg`, `312c`, and this document as it then stood (§ 2, § 4, § 5), so every "agreed on
reading" below is post-hoc agreement, not an independent find; its ten fronts are independent.
It returned last. Findings restated by the conductor with re-walk and verdict.

Surviving in the lane's own ranking, with the conductor's verdict:

- `fa-stand-in-drops-p-non-empty-witness` (lane: ~SUSPECT leaning +SURE) — the § 2.10 stand-in
  sentence, reading (A) "the members replace the entry P given whole" against reading (B) "the
  members replace only the reach beneath P, P stays". Witness: `apt-get remove -y nginx` with
  at-most `{sm.Package:nginx given whole}` closed and the Package sort finished (files, unit,
  nothing else), above a converged `dpkg -s nginx` reading `nginx@installed`. Under (A) the
  members stand in; the cell is P's own and has no placing route; whether a member file and the
  cell collide then depends on how the dpkg store's chain is modelled. The lane's walk runs the
  dpkg store's identity through directory levels (`/usr` against `/var` as tops), which 311
  does not license: a directory is a routing catalog, not an identity parent. The conductor's
  variant reaches the same wrong spare cleanly: with the dpkg store keyed as its status file's
  inode in the root filesystem, a member inode and the cell's chain meet at the filesystem as
  two inodes of one MScheme under `:guarantees-unique-name`, DISJOINT, and the `dpkg -s` fact
  survives the removal of the package it measures. Under (B) the cell collides at once by
  § 3.2 step 2, as § 1.9 promises. Every statement is true under (A): the finished record is
  about other things by § 2.6's own words ("never no other MKey for the thing written"), so its
  silence about P's cells is no false closure. Verdict: CONFIRMED; four-way convergence on the
  ambiguity (§ 2.1 nit, § 4, § 5, here); the unsafe reading is the literal one; the witness
  stands with the conductor's stdlib shape in place of the lane's directory walk.
- `fa-open-read-set-any-routing-mutation-is-undecidable` (lane: ~SUSPECT; lone) —
  `311:1.7-resolution-and-its-traversal` and `311:3.3-invalidation-three-mutator-species`,
  "Any routing mutation invalidates a MResolution whose read set is open." A routing mutation
  is defined by what a write touches; its third clause, "shell state a `resolve()` read", is
  knowable only through the read set, which is open. A builder classifies by the two decidable
  clauses. Witness: `myhost__resolve() { mytool lookup "$1"; }` with `mytool` undescribed;
  `curl "http://$(myhost web)/health"`, then `cp new.conf /etc/mytool.conf` (at-most the content
  cell, closed, true), then a second `curl` through `myhost web`. The `cp` names no routing key
  and no catalog, and whether the config is state the lookup read is what the open set hides;
  under the decidable reading the MResolution of `web` stands, the warrant's condition ("while
  that MKey's MResolution or MToken stands") holds, the two sites read SAME by
  `:guarantees-unique-referent`, and under the flag the second site's elision survives a write
  that changed what `web` reaches. Conductor's re-walk: the ledger's stated intent is a ⊤
  perish set (`312cg` § 10, "open (⊤ perish set, fail-safe)"), and § 2.5's own fact-side
  sentence says "affected by every write"; the lookup-side sentence says "routing mutation"
  instead. Verdict: the ambiguity is CONFIRMED (+SURE the category is undecidable for an open
  set as written); the wrong SAME is PLAUSIBLE under the decidable reading, flag-gated. One
  word repairs it: any write.
- `fa-exclusion-keyed-on-the-container-drops-interior-effects` (lane: ~SUSPECT; lone;
  pre-existing in substance) — § 2.6's exclusion, "A container at or above the deepest level
  that the written MKey shares with the read MKey contributes no entailment to the test against
  that fact", is keyed on the entailing container, not on where the entailed MKey sits.
  Witness, every statement true: the filesystem's owner declares that writing any inode entails
  may-write of the quota accounting file's inode (an interior sibling); `cp payload
  /srv/data/blob` above a converged `repquota /srv | grep -q alice`, whose fact reads the quota
  inode. A is the filesystem; the filesystem is at A; its entire entailment is excluded; the
  quota inode never enters; `compare(blob inode, quota inode)` is two tops of one MScheme under
  `:guarantees-unique-name`, DISJOINT; the fact is spared past a write that changed it. The
  exclusion was written for entailed members above A (the disk), which step 2 collides anyway.
  Conductor's re-walk: the sentence is unambiguous, so this is not a reading; the declaration
  fits rule 2's shape (a write beneath an MKey of Filesystem fires the entailment, which names
  an MKey of another MSort); the prior panel tried and killed a neighbouring construction (the
  free-space cell, `312cb` § 4), and the separable-sibling shape is the one that survives. The
  substance predates the freeze ("contributes nothing" became "contributes no entailment"), so
  scope is borderline as with § 2.1's two-level leak, and the two are one defect from two
  sides: the exclusion keys on the container where it should key on the entailed member's
  position relative to A. Verdict: CONFIRMED, cardinal class on a narrow shape (store-level
  entailments naming separable interior siblings: a quota file, a journal exposed as an inode,
  an index beside its rows). One re-keying fixes both.
- `fa-closure-count-one-level-against-every-level` (lane: ~SUSPECT; lone) — § 1.5 says a lookup
  may emit `alias nothing-else` "for the one level it resolved"; § 2.9 step 3 needs "every level
  emitted its closure". Read narrowly a multi-level path lookup emits one closure and no path
  region ever reads DISJOINT; read per crossed level (as `311t` § 15 intended and § 5.1's grain
  "[key, level]" suggests) it works. Verdict: CONFIRMED textual, lost-elision direction; one
  phrase in § 1.5.
- `fa-engine-vouches-the-inherited-instances` (lane: ~SUSPECT) — § 1.10's "Under a wrapper, it
  vouches the inherited instances that 3.4-entry-and-lends admits" names the engine as voucher
  of something that rests on the sentinel and the flag, beside the genuinely engine-vouched
  local MRoute; § 3.5 and § 3.4's danger line have the tier right. Verdict: PLAUSIBLE textual;
  a builder can mint that SAME unflagged from § 1.10 alone; "supplies" or "chains" in place of
  "vouches". Two register gaps the lane adds (-GUESS): `spike/CLAUDE.md`
  `rul-flag-is-razor-residue` ("permits acting on separation claims") now also covers a
  sameness; the human-opted `pure-predicate-carry` as the one unflagged cross-boundary carry
  reads false under § 3.4. Verdict: CONFIRMED omissions, register.
- `fa-v0-floor-entry-keeps-the-catalog-entry` (lane: -GUESS) — the `30T` v0-floor entry of § 4.2
  still says "routing writes to its MParent-Catalog entry", a leftover of the retracted
  `dfc1f950` mandate; § 3.3 now leaves the written MKey to the verb author. Verdict: CONFIRMED,
  register.
- `fa-index-corrections` (lane: -GUESS) — § 5.2's three INVAL cells marked flag "no" (a stale
  MResolution reaches a decision only under the flag, since nothing outlives a running line
  unflagged); the finished record built "decl" where § 2.6 says "reached" (eval); § 5.1's
  sentinel row reading as an unflagged SAME for want of a flag column. Verdict: CONFIRMED,
  non-normative; the "decl" cell converges with § 2.1.

Raised and dead, the lane's own dispositions, each checked: the example demotions (no soundness
loss; two lose guidance, "a path prefix is not a store" and "state spanning several files is a
may-read matter"; the § 2.1 footer's rm/cp binding pair does normative-shaped work with no
normative home, which `312cg` § 23 judged acceptable); rule 4's forward propagation as a break
(the lane passed the injection sentences, then RESCINDED on reading this document; its
transitive-chain, leaf, and ancestor equivalences stand); invalidation's DISJOINT not
flag-gated (no unflagged consumer can act on a MResolution kept past a running line);
correspondence across MWorlds (sound, and the composition rules stay sound); routing writes as
the verb author's at-most claim (every honest combination collides; the register nit above is
what remains).

Not raised by this lane, from § 2.1: the two-level entailment leak (not considered; it shares a
root and a fix with the lane's quota finding, see § 1). Agreed on reading, post-hoc: the
unmarked verdict's closed-empty readset; the cross-world entry collapsing its own world; the
generated-key totality sentence; § 2.9's "Danger: none"; the `plans/30W` index-kinds entry; the
within-sort finished-record demand against the register gloss.

Process note the lane wrote against itself, kept because it is the round's calibration datum:
its rule-4 front confirmed the forward rewrite's equivalence by checking the cases the rewrite
was designed for (chain members) and asserted the rest harmless without walking a stranger's key
through the universal; three other views found the break on the same sentences it passed.

## § 4-lane-astra-neutral

Provenance: OpenAI Codex, GPT-6-Astra, `model_reasoning_effort="high"`, foreign lineage, neutral
stance; `-s workspace-write` in a scratch copy; first attempt, no setup errors; about 21 minutes.
Eight fronts, written one at a time before its comparison pass, each carrying its later
disposition. Findings below are restated by the conductor; each carries the conductor's re-walk
against 311's text and a verdict. The lane found no unconditional wrong SAME or DISJOINT.

Surviving in the lane's own ranking, with the conductor's verdict:

- `an-unrelated-floor-key-revokes-rooted-survival` (lane: +SURE) — the same sentences as
  `fnd-open-sort-keys-in-every-writeset-collide-every-fact` (§ 2.1), reached from two witnesses.
  First, the accepted one: a filesystem `F` scoped in the MRoute; a write to file `a` acquires
  `F` in its writeset by the § 1.10 sentence; `compare(F, b)` is UNKNOWN by step 2 for any `b`
  in `F`; the lane withdrew this as a defect after reading `311u:refuted-vantage-root-as-a-machine`
  and `312cg` § 24. Second, the one that survives: `a` and `b` fully described in one rooted
  store, `compare(a, b)` DISJOINT, and an unrelated key `u` elsewhere in the book, scoped in the
  MRoute; § 1.10 inserts `u` into every line's writeset; `compare(u, b)` is UNKNOWN by § 3.2
  step 1; the universal fails; the lane names the ledger's equivalence claim (`312cg` § 28) as
  contradicted and adds that unclosed unrelated sorts under rule 4 contaminate the same way.
  Verdict: CONFIRMED; converges with § 2.1's first finding in mechanism and in the same two
  sentences; the lane's separation of the accepted floor from the exported contamination is the
  right cut.
- `an-enumeration-stand-in-drops-the-container` (lane: ~SUSPECT) — `311:2.10-places-the-upward-lookup`'s
  "its emitted members stand in for P given whole in the test of §2.6", against § 2.9's "An entry
  names the MReferent of its MKey" plus what is reached beneath. Replacing the whole entry with
  the members can drop `P` itself; the lane's world is an empty collection with a label cell
  identified in `P`, a verb that writes `P` given whole and changes the label, and an owner whose
  finished enumeration is truthfully empty: the replacement is empty, the universal is vacuous,
  and the label fact spares. The lane grants the safe competing reading (retain `P`'s ordinary
  entry, replace only its reach beneath) and files the finding as an ambiguity with an unsafe
  permitted reading. Verdict: CONFIRMED, and sharpened by the conductor's re-walk in § 1: a
  non-empty enumeration does not rescue it wherever the members separate from `P`'s own cells
  under the stdlib's key space. The conductor's own § 2.1 nit walked only the additive reading.
- `an-generated-key-totality-is-false` (lane: +SURE) — `311:2.2-primary-of-and-identified-in`'s
  "A store that generates its MKeys at creation holds an MReferent under every MKey a book can
  name"; `id=$(widget create); widget delete "$id"; widget inspect "$id"` names a generated key
  the store no longer holds under it. Verdict: CONFIRMED as a text defect; the preceding "or
  none" sentence is the defensible rule. The lane's rider (~SUSPECT) that "equal MKeys reach one
  MReferent" needs saying whether it applies to two keys reaching none: PLAUSIBLE, textual.
- `an-closure-order-is-unspecified` (lane: ~SUSPECT) — the line-global "least set that four
  rules close" against the pair-relative exclusion of ancestor entailments; the lane's witness
  has `F`'s entailment name its disk `d` and `F` may-read `d`, so that saturating first adds `d`
  then `F` through rule 4 while excluding first adds neither. Verdict: the ambiguity is CONFIRMED
  and matches § 2.1's `nit-least-set-with-a-pair-relative-exclusion`; the witness is NOT
  confirmed: rule 4 fires on the written key `a` against `d` directly (UNKNOWN whether `d` is
  `a`'s ancestor or a stranger key at the MRoute), so `F` joins under either order and the two
  readings give one answer there. No witness in which the two readings differ is in hand.
- `an-danger-none-contradicts-the-closing-act` (lane: +SURE) — `311:2.9-the-traversal-and-the-region-test`'s
  retained "Danger: none. Finer buys sparing. Coarse is safe." beside § 1.7's new authored
  closing act: a falsely closed emission retains a stale resolution and every conclusion on it;
  § 5.2 names the knife, the normative line does not. Verdict: CONFIRMED; the conductor's § 2
  missed it.
- `an-index-kinds-entry-omits-the-correspondence-exception` (lane: +SURE) — the `plans/30W`
  index-kinds entry of § 4.2 still says "nothing speaks across MWorlds". Verdict: CONFIRMED,
  register hygiene.
- `an-read-set-membership-reads-literal` (lane: ~SUSPECT, withdrawn as substantive after the
  ledger) — § 3.3's "when the written MKey is in the read set of the lookup body" beside the
  traversal sentence's "other than DISJOINT". Verdict: CONFIRMED as a one-clause precision;
  the intended overlap reading is the ledger's (`312cg` § 10).
- `an-mroute-inheritance-exceeds-the-sentinel-claim` (lane: downgraded to --WONDER by its own
  comparison pass) — § 3.4's danger line calls the sentinel "an at-most claim over every
  MParent-Catalog MSort" while the inheritance sentence also inherits the MRoute; the lane's
  world (two databases, record `1` in each, a wrapper that lends the changed catalog and closes)
  was defeated by § 2.8's default observer dependence, as the lane itself found. Verdict: the
  wrong-SAME claim is DEAD; the textual point stands as a nit: the danger line should name the
  MRoute beside the catalog sorts, since § 1.10 and § 3.4 inherit it.

Raised and dead, the lane's own dispositions, each checked: wrapper inheritance as an all-true
wrong SAME (observer dependence; dead); no survival beneath the MRoute (the accepted floor);
literal read-set membership (withdrawn, see above); empty route families and aliases above the
leaf (the spoken-closure requirement and the every-level walk handle them); conflicting partial
placing answers (records accumulate; only a contradicted closure refuses); cross-world
correspondence against cross-world may-read entries (kept distinct and compatible).

Not raised by this lane, from § 2.1: the unmarked verdict's closed-empty readset; the cross-world
entry collapsing its own world; the two-level entailment leak; the within-sort finished-record
demand against the register's "across MSorts". The lane's cross-world front reads the entry as
propagating "from a write in y's world" without asking what the entry does against writes in
x's own world.

## § 5-lane-astra-adversarial

Provenance: OpenAI Codex, GPT-6-Astra, `model_reasoning_effort="high"`, foreign lineage,
adversarial stance; `-s workspace-write` in a scratch copy; first attempt, no setup errors
reported; about 16 minutes. Nine fronts, written before comparison, each carrying its later
disposition. The lane did not establish a new faultless wrong SAME or DISJOINT. Findings restated
by the conductor with re-walk and verdict.

Surviving in the lane's own ranking, with the conductor's verdict:

- `aa-ignorance-is-contagious-under-the-forward-rewrite` (lane: +SURE) — the same as § 4's
  first item and § 2.1's first finding: a described store with warranted distinct `a` and `b`, a
  third vocabulary `U` with a mentioned key `u` whose may-read set is open and whose relation to
  `b` is KNOWN_UNSPOKEN; rule 4's last sentence inserts `u` into the command's writeset; `(u, b)`
  cannot be DISJOINT; `b`'s survival dies. The lane states the repair's shape: preserve the
  distinction between "this unknown fact may depend on this write" and "this write may mutate
  this object's referent"; globalizing the former as the latter loses it. It rescinds its
  route-specific variant as the accepted floor after the comparison pass. Verdict: CONFIRMED;
  three-way convergence with § 2.1 and § 4 on the same two sentences; the lane's phrasing of the
  repair is the read-side placement § 2.1 proposes.
- `aa-stand-in-erases-the-originating-write` (lane: ~SUSPECT) — the § 2.10 stand-in sentence;
  the lane's set error is stated cleanly as `whole(P) = {P} ∪ members(P)` replaced by
  `members(P)`; its witness is the empty collection whose own state a verb changes. It grants
  that § 2.6's rule 1 may independently retain `P` and that an implementation may replace only
  the deep reach, and files an unsafe permitted reading. Verdict: CONFIRMED, and see § 1 for the
  conductor's non-empty witness.
- `aa-generated-key-totality-is-false` (lane: +SURE) — as § 4's third item; the lane adds that
  the "or none" sentence, the decline, and invalidation already protect absence, "which is
  precisely why the new universal existence sentence is unnecessary and misleading". Verdict:
  CONFIRMED, textual.
- `aa-least-set-and-target-relative-exclusion` (lane: ~SUSPECT) — as § 4's fourth item; the
  lane's witness likewise routes through `F` may-read `d`. Verdict: ambiguity CONFIRMED; witness
  NOT confirmed for the same reason as in § 4 (rule 4 fires on `a` against `d` directly under
  either order). The lane's narrowing after the comparison pass ("a single line-global least set
  and a target-relative least set are different objects") is the honest statement.
- `aa-danger-none-contradicts-the-closing-act` (lane: +SURE) — as § 4's fifth item; the lane
  supplies the witness: a lookup crossing `a` and `b` emits only `a` and closes; a write to `b`
  leaves the resolution alive; dependent SAME or DISJOINT keeps stale authority; attributable to
  the false closing act, so not a faultless case, but the "Danger: none" line is wrong. Verdict:
  CONFIRMED.

Raised and dead, the lane's own dispositions, each checked: the route-rooted survival loss (the
accepted floor); missing lookup dependencies (the new body-read closure and the open-set
invalidation cover the `selected=$(cat /etc/tool-target); tool "$selected"` case); the empty
traversal proving separation (the spoken-closure requirement blocks it); multiple placing answers
wrongly conflicting (records accumulate); cross-world correspondence manufacturing equality (an
explicit identity premise, observer qualification intact); wrapper inheritance bypassing observer
or consent checks (both remain; the lane also tested laundering a flagged SAME through an
unflagged transitive chain and found "rests on" covers the composed derivation); negative
existence becoming inexpressible (a directory fact or a describer-minted name-keyed referent
supplies it). One out-of-scope line, correctly filed: the region-against-region hole stays as
`312cg:hold-region-against-region-floor`.

Not raised by this lane, from § 2.1: the unmarked verdict's closed-empty readset; the cross-world
entry collapsing its own world (the lane's correspondence front says a remote backing "enters the
new write closure through comparison with that backing", without walking the entry against
writes in its own world); the two-level entailment leak; the within-sort finished-record demand
against the register's gloss.
