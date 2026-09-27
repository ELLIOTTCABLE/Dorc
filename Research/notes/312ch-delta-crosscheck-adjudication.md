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

[RESERVED. Written after all three lanes return: the convergence matrix over § 2 to § 5, what
survives ranked by consequence (a wrong SAME or DISJOINT with no false statement behind it; a
lost elision or an unreachable true answer; text a builder must guess at), what was raised and
died, and the decisions the human owns. Lone adversarial findings are re-walked before credit.]

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
- Changed text, all `fc56941e`: `311:2.5-may-read-the-readset`, "An mSort that declares no
  may-read set, or declares one and does not close it, is affected by every write: every mKey
  of that mSort is in every line's writeset." `311:2.6-may-write-the-writeset`, rule 4, last
  sentence: "Every mKey of an mSort with no closed may-read set is in every writeset."
  `311:1.10-vantage-route-placeholder-witness`: "Every mKey scoped in it [the mRoute] is in
  every line's writeset."
- The walk. USER_STORY stage 5's book with the stage-4 `foobar` oracle loaded. `org.foob.Certs`
  declares no `:identified-in` (so its mKeys are scoped in the mRoute, § 1.6) and no may-read
  set. The stdlib has rooted the boot, so the dpkg store's chain ends at the boot mRoot.

  ```sh
  apt-get update                                               # 5  drifted: runs; list inodes, closed; apt's record reached
  dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx    # 6  converged; readset: the nginx package key
  foobar sync-certs "$CERTS"                                   # 8  converged; org.foob.Certs:/etc/nginx/certs
  ```

  Line 5's writeset contains, by the § 1.10 sentence, every mKey scoped in the mRoute, so
  `org.foob.Certs:/etc/nginx/certs`. The sparing test for line 6 runs over every pair of a
  writeset member and a readset member "of one mSort or of two". `compare()` of the Certs key
  against the package key: one chain terminates at the mRoute, the other at a mRoot the first
  does not share, so `311:3.2-compare-one-chokepoint-four-answers` step 1 answers UNKNOWN.
  Not DISJOINT, so line 6 collides and guards. Every fact in the book collides with every
  line, whatever its describers declared, because the stranger's keys sit in every writeset
  and are UNKNOWN against every rooted key. The same holds for any mSort with no closed
  may-read set, by the § 2.5 sentence: a vendor's `org.vendor.Widget`, undeclared, sits in
  every writeset and is KNOWN_UNSPOKEN or UNKNOWN against every other sort's fact.
- Under the backward form (`c5e8f434`) the fact's readset held its own chain's entries and
  nothing of the stranger's; ⊤ appeared only at an undeclared member of that chain. Line 6
  spared. So the forward rewrite changed an answer, against the ledger's own summary of it
  (`312cg` § 28: "No object, relation, or answer changed"). It also breaks
  `26M:law-monotone-enhancement`: installing the stage-4 oracle removes stage 5's survivals.
- Through `311:3.3-invalidation-three-mutator-species` the same sentences make every traversal
  member whose mSort has no closed may-read set touched by every line, so every resolution
  through such a member reads unknown below every line.
- The narrow reading the sentences were reaching for (conductor's inference from `312cg`
  § 24 and § 28): a fact ABOUT a key of an undeclared sort, or about a key scoped in the mRoute,
  collides with every line. That reading puts ⊤ on the read side: a readset member whose mSort
  has no closed may-read set, or which is scoped in the mRoute, is ⊤, and ⊤ is DISJOINT from
  nothing. Rule 4's first sentence then still carries the chain: an undeclared ancestor's keys
  need not be injected, because a fact under it already collides through the ancestor's own
  membership when a write reaches it.
- Repair candidates, the human's call: move the three sentences to the read side as above; or
  restrict them to "every mKey of that mSort that appears in a readset".
- Falsifier: a sentence in 311 that keeps undeclared-sort keys out of the pairwise test against
  facts of other sorts, or that makes `compare()` of an unrooted stranger key against a rooted
  key DISJOINT. Neither is found: § 2.6 runs the test over pairs "of one mSort or of two", and
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
  readset of a verdict fact always contains the mKey of the cell its marked line answers; a body
  with no marked line answers no cell and has readset ⊤".
- Falsifier: a sentence in 311 that gives an unmarked body a non-empty or ⊤ readset. None is
  found. `311:1.9-cell-a-singleton-sort`'s "the marked line that answers a cell is a read of the
  cell's mKey" speaks of marked lines only.

#### fnd-cross-world-entry-collides-its-own-world

- Kind: lost elision, total for the declaring sort in its own mWorld, from one honest additive
  statement. Grade: +SURE of the walk; ~SUSPECT of weight (it needs the two worlds' mRoots to
  be of different shapes; the hypervisor case with two `sm.Boot` roots is safe).
- Changed text: `311:2.5-may-read-the-readset`, "An entry may name an mKey of another mWorld.
  `compare()` decides that pair as 3.2-compare-one-chokepoint-four-answers decides any pair."
  (`c5e8f434`), consumed by `311:2.6-may-write-the-writeset` rule 4 (`fc56941e`) and by § 3.2
  step 1, "mRoots of two shapes are two mWorlds ... No mFullyQualifiedKey speaks across mWorlds".
- The walk. A guest VM on a cloud host. The guest's filesystem `F_guest` is identified in its
  virtual disk, identified in the guest's `sm.Boot` mRoot. The hypervisor's describer, per
  `312cg` § 26, declares that the guest's disk may-read the host file `x.qcow2`, an inode in a
  host filesystem identified in a cloud `sm.Volume` mRoot. Any guest-side line that writes
  anything (`cp ./app.conf /etc/app/app.conf` inside the guest) has a writeset member
  `w_guest`. Rule 4: `compare(w_guest, host qcow2 inode)`: two mRoots of two shapes, so
  UNKNOWN, so not DISJOINT, so the guest disk's key joins the writeset. Then the test against
  any guest file fact `r`: `compare(guest disk, r)`: the disk is an ancestor of `r`, so § 3.2
  step 2 answers UNKNOWN. Collide. Every guest write collides every guest file fact from the
  moment the honest cross-world entry is declared. Where both worlds are one mRoot shape with
  `:guarantees-unique-name` (two `sm.Boot`s), the pair separates as siblings and the entry is
  harmless; the text does not say the entry's value depends on that.
- Not a regression of the rewrite: under the backward form the entry sat in the inherited
  readset and compared UNKNOWN against every guest write likewise. The permission to name a
  cross-world entry is what is new.
- Repair candidates: state the condition beside the permission (an entry keyed in another
  mWorld collides with every write in the entry's own mWorld unless both mWorlds are one mRoot
  shape); or carry cross-world dependency as the transition owner's speech, with its own
  consumer, rather than as a may-read entry.
- Falsifier: `compare()` answering DISJOINT for a pair whose chains end at mRoots of two shapes.
  § 3.2 step 1 forbids it.

#### fnd-entailment-naming-the-parent-leaks-two-levels-up

- Kind: lost elision; borderline scope (rule 2 restates semantics that predate the freeze; the
  footer example moved). Grade: +SURE of the walk; ~SUSPECT that stdlib describers would write
  the two-level pattern.
- Changed text: `311:2.6-may-write-the-writeset` rule 2, "Where an mKey of K is in the
  writeset, or an mKey identified beneath an mKey of K, every mKey that K's may-write entailment
  names is in the writeset", with the unchanged exclusion "A container at or above the deepest
  level that the written mKey shares with the read mKey contributes no entailment to the test
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
  and leaky at two. § 2.5 tells a describer that the mParent is no may-read entry; nothing tells
  them that the mParent is no entailment member, and the footer names it.
- Repair: a sentence in § 2.6 mirroring § 2.5's, that an entailment need not name the written
  mKey's mParent, since the walk's step 2 already collides a write at or above it; the footer
  example then changes.
- Falsifier: text removing `D`'s membership. The exclusion removes an entailment, not a member.

#### fnd-finished-record-demanded-within-one-sort

- Kind: lost elision; a register gloss that misdescribes the text. Grade: +SURE of the
  mismatch; the demand itself predates the freeze and the human restored its strict form
  (`312cg` § 15).
- Changed text: `311:2.6-may-write-the-writeset`, "An unclosed at-most set or an unfinished
  entailment puts ⊤ in the writeset", and rule 2's "For each origin cell in the writeset, a
  reached finished record for that cell's mSort and shape finishes the entailment". The § 4.2
  entry for `30U` says "a finished definition stays necessary for sparing across mSorts".
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
- `311:2.6-may-write-the-writeset` rule 3 says every mKey reached beneath a whole-marked member
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
- `311:2.2-primary-of-and-identified-in`, "A store that generates its mKeys at creation holds an
  mReferent under every mKey a book can name." A book can name a stale or mistyped generated key
  (an inode number captured before an `rm`). The honest sentence is "under every mKey it has
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
  mVantages share the mRoute, so two-tops separation at the mRoute would license DISJOINT across
  a wrapper on a SAME-side claim; but every mRoute-terminated readset member is in every writeset
  (§ 1.10 as written) or ⊤ (under the first finding's repair), so no such fact spares and the
  danger line's "wrong SAME" is the sentinel's only consequence.
- `chk-region-test-second-form-uses-accumulated-records` (+SURE) — step 3's "with no
  `looked-up-in` record" reads over the union of invocations per § 2.10's accumulation sentence;
  a decline emits no closure and falls to step 4.
- `chk-unclosed-traversal-floor-member-blocks-disjoint` (+SURE) — a lookup that emits members
  without the closing act keeps its mParent-Catalog given whole as a level; that level carries
  no `alias nothing-else`; step 3 fails; UNKNOWN. A lookup cannot buy DISJOINT with per-level
  closures while withholding the traversal's closing act.
- `chk-top-in-writeset-invalidates-every-resolution` (+SURE) — ⊤ is DISJOINT from nothing, so
  an unmodelled line touches every traversal member and invalidates every resolution below it:
  the poison wall extended to identity, consistent with kill-reach.
- `chk-parent-write-covers-cells-by-step-two` (+SURE) — § 1.9's "A writeset entry naming the
  mParent covers its cells (§3.2, step 2)": the cell's chain is cell then parent; the writeset
  member is the parent, which is the deepest SAME level itself; UNKNOWN; collide. Dropping
  "whole" was right.
- `chk-fence-narrowing-is-consistent` (+SURE) — § 1.8, § 3.2 step 1, and § 2.6 agree that only a
  mCorrespondence crosses mWorlds; a cross-world may-read entry is a key compared by § 3.2 and
  never yields SAME across, so it does not breach the fence.

### § 2.3-accepted-costs-restated

Not holes. Stated so their scale is visible beside the holds.

- `cost-open-lookup-read-sets-invalidate-everywhere` (+SURE of the text; acked `312cg` § 11) —
  `311:1.7-resolution-and-its-traversal`: a lookup body's read set is closed only when every
  external command's read set is closed by that command's describer, and "Any routing mutation
  invalidates a mResolution whose read set is open." No describer has closed `stat`,
  `dpkg-query`, `readlink`, or `getent` today, so every routing mutation (an `export`, a `cd`, a
  `mount`, a `useradd`, any unmodelled line) invalidates every resolution below it, and no SAME
  or DISJOINT survives that line. Safe. Its scale: identity survives nothing past the first
  `export` in any book until the stdlib closes the read sets of the commands its lookups run.
- `cost-every-written-sort-needs-a-finished-record` — the fifth finding's demand, restated as
  a cost: within-sort sparing waits on the written sort's finished record for the shape written.
- `cost-unrooted-facts-spare-nothing` (+SURE; the § 24 litmus) — a fact whose chain ends at the
  mRoute spares past nothing; value begins where the stdlib declares roots and stores.

### § 2.4-out-of-scope-one-line-each

- `311:2.9-the-traversal-and-the-region-test`'s floor sentence, "reads UNKNOWN against every
  mKey that mScheme can yield in the same mParent-Catalog instance": for a path the catalog
  instance is a directory, so the sentence protects siblings and not descendants; the region
  test's step 2 covers descendants regardless, so the sentence is inert as written. Text that
  predates the freeze.
- `311:2.7-corresponds-across-a-transition`, "the model's only declared sameness generator
  besides mToken equality", beside § 3.2's "Two mSchemes yielding one mKey-Primary is the sole
  same-referent generator across ways of naming": two "only"s naming different generators,
  reconcilable if a yield into one primary counts as mToken equality. Predates the freeze.

## § 3-lane-fable-adversarial

[RESERVED. Provenance line (Claude Fable 5.1, in-lineage, adversarial stance), then each of the
lane's findings restated by the conductor with its re-walk against 311's text and a verdict, then
the lane's raised-and-dead items in one line each. Raw output is not pasted.]

## § 4-lane-astra-neutral

Provenance: OpenAI Codex, GPT-6-Astra, `model_reasoning_effort="high"`, foreign lineage, neutral
stance; `-s workspace-write` in a scratch copy; first attempt, no setup errors; about 21 minutes.
Eight fronts, written one at a time before its comparison pass, each carrying its later
disposition. Findings below are restated by the conductor; each carries the conductor's re-walk
against 311's text and a verdict. The lane found no unconditional wrong SAME or DISJOINT.

Surviving in the lane's own ranking, with the conductor's verdict:

- `an-unrelated-floor-key-revokes-rooted-survival` (lane: +SURE) — the same sentences as
  `fnd-open-sort-keys-in-every-writeset-collide-every-fact` (§ 2.1), reached from two witnesses.
  First, the accepted one: a filesystem `F` scoped in the mRoute; a write to file `a` acquires
  `F` in its writeset by the § 1.10 sentence; `compare(F, b)` is UNKNOWN by step 2 for any `b`
  in `F`; the lane withdrew this as a defect after reading `311u:refuted-vantage-root-as-a-machine`
  and `312cg` § 24. Second, the one that survives: `a` and `b` fully described in one rooted
  store, `compare(a, b)` DISJOINT, and an unrelated key `u` elsewhere in the book, scoped in the
  mRoute; § 1.10 inserts `u` into every line's writeset; `compare(u, b)` is UNKNOWN by § 3.2
  step 1; the universal fails; the lane names the ledger's equivalence claim (`312cg` § 28) as
  contradicted and adds that unclosed unrelated sorts under rule 4 contaminate the same way.
  Verdict: CONFIRMED; converges with § 2.1's first finding in mechanism and in the same two
  sentences; the lane's separation of the accepted floor from the exported contamination is the
  right cut.
- `an-enumeration-stand-in-drops-the-container` (lane: ~SUSPECT) — `311:2.10-places-the-upward-lookup`'s
  "its emitted members stand in for P given whole in the test of §2.6", against § 2.9's "An entry
  names the mReferent of its mKey" plus what is reached beneath. Replacing the whole entry with
  the members can drop `P` itself; the lane's world is an empty collection with a label cell
  identified in `P`, a verb that writes `P` given whole and changes the label, and an owner whose
  finished enumeration is truthfully empty: the replacement is empty, the universal is vacuous,
  and the label fact spares. The lane grants the safe competing reading (retain `P`'s ordinary
  entry, replace only its reach beneath) and files the finding as an ambiguity with an unsafe
  permitted reading. Verdict: CONFIRMED, and sharpened by the conductor's re-walk in § 1: a
  non-empty enumeration does not rescue it wherever the members separate from `P`'s own cells
  under the stdlib's key space. The conductor's own § 2.1 nit walked only the additive reading.
- `an-generated-key-totality-is-false` (lane: +SURE) — `311:2.2-primary-of-and-identified-in`'s
  "A store that generates its mKeys at creation holds an mReferent under every mKey a book can
  name"; `id=$(widget create); widget delete "$id"; widget inspect "$id"` names a generated key
  the store no longer holds under it. Verdict: CONFIRMED as a text defect; the preceding "or
  none" sentence is the defensible rule. The lane's rider (~SUSPECT) that "equal mKeys reach one
  mReferent" needs saying whether it applies to two keys reaching none: PLAUSIBLE, textual.
- `an-closure-order-is-unspecified` (lane: ~SUSPECT) — the line-global "least set that four
  rules close" against the pair-relative exclusion of ancestor entailments; the lane's witness
  has `F`'s entailment name its disk `d` and `F` may-read `d`, so that saturating first adds `d`
  then `F` through rule 4 while excluding first adds neither. Verdict: the ambiguity is CONFIRMED
  and matches § 2.1's `nit-least-set-with-a-pair-relative-exclusion`; the witness is NOT
  confirmed: rule 4 fires on the written key `a` against `d` directly (UNKNOWN whether `d` is
  `a`'s ancestor or a stranger key at the mRoute), so `F` joins under either order and the two
  readings give one answer there. No witness in which the two readings differ is in hand.
- `an-danger-none-contradicts-the-closing-act` (lane: +SURE) — `311:2.9-the-traversal-and-the-region-test`'s
  retained "Danger: none. Finer buys sparing. Coarse is safe." beside § 1.7's new authored
  closing act: a falsely closed emission retains a stale resolution and every conclusion on it;
  § 5.2 names the knife, the normative line does not. Verdict: CONFIRMED; the conductor's § 2
  missed it.
- `an-index-kinds-entry-omits-the-correspondence-exception` (lane: +SURE) — the `plans/30W`
  index-kinds entry of § 4.2 still says "nothing speaks across mWorlds". Verdict: CONFIRMED,
  register hygiene.
- `an-read-set-membership-reads-literal` (lane: ~SUSPECT, withdrawn as substantive after the
  ledger) — § 3.3's "when the written mKey is in the read set of the lookup body" beside the
  traversal sentence's "other than DISJOINT". Verdict: CONFIRMED as a one-clause precision;
  the intended overlap reading is the ledger's (`312cg` § 10).
- `an-mroute-inheritance-exceeds-the-sentinel-claim` (lane: downgraded to --WONDER by its own
  comparison pass) — § 3.4's danger line calls the sentinel "an at-most claim over every
  mParent-Catalog mSort" while the inheritance sentence also inherits the mRoute; the lane's
  world (two databases, record `1` in each, a wrapper that lends the changed catalog and closes)
  was defeated by § 2.8's default observer dependence, as the lane itself found. Verdict: the
  wrong-SAME claim is DEAD; the textual point stands as a nit: the danger line should name the
  mRoute beside the catalog sorts, since § 1.10 and § 3.4 inherit it.

Raised and dead, the lane's own dispositions, each checked: wrapper inheritance as an all-true
wrong SAME (observer dependence; dead); no survival beneath the mRoute (the accepted floor);
literal read-set membership (withdrawn, see above); empty route families and aliases above the
leaf (the spoken-closure requirement and the every-level walk handle them); conflicting partial
placing answers (records accumulate; only a contradicted closure refuses); cross-world
correspondence against cross-world may-read entries (kept distinct and compatible).

Not raised by this lane, from § 2.1: the unmarked verdict's closed-empty readset; the cross-world
entry collapsing its own world; the two-level entailment leak; the within-sort finished-record
demand against the register's "across mSorts". The lane's cross-world front reads the entry as
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
