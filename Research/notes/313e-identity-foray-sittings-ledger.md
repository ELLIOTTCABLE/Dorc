# 313e — The identity foray: the sittings ledger

> AI-authored (Fable, 2026-10-03/04, the human present). Notes-tier chronological ledger of a
> foray beside the mechanised `specs/311-identity.assay.md`, which continues in parallel
> **[TYPED]**. Successors append sections; they do not rewrite an earlier one. Nothing is ruled
> unless marked **[TYPED]** (the human typed it; the wording here is the conductor's) or cited as
> `docID:slug`. **[CONDUCTOR]** marks a claim the human has not acked. Grades: +SURE / ~SUSPECT /
> -GUESS / --WONDER. Every measurement is AI-run process evidence.

## § 1-the-first-pass-and-the-first-plan

The remit **[TYPED]**: judge whether something of 311's shape can be built from the ground up,
with rulings issued one at a time and the checker green at each step, with time and ordering,
and attribution as the output, right from the start. The end goal is design questions small
enough to rule on.

Typed across the sitting:

- A mutation-shaped lane in CI for kills that go quiet under redundancy. Ordering the rulings is
  the hard part. USER_STORY's stages are the wrong spine, and a mechanical value yardstick is
  nacked; the lean is outside-in, planned from what 311 yielded.
- The flag is one boolean; the same construction is built in both of its states; it turns one
  class of judgement from guard to elide where no concrete conflict is known. That class rests on
  a negative that closes over an inherently unclosed world (typed with a stated uncertainty). A
  thoroughly described toy world must reach elision past a line that ran without such a
  negative. In practice nearly every such elision will need the flag; USER_STORY and
  `KNOBS:kSURVIVAL` describe that practice, and the model wants the closed sense.
- A closure line such as `disturbs nothing-else` is assertion and consent, one object, revocable.
- Six cells must be buildable: positive statements all true or some false, against a closure
  absent, true, or false.
- Pure logic, typed as such: "these two do not touch" cannot be said without a negative, and
  flag-free elision in a world of many participants needs a matrix of concrete negatives.
- The flag class may sit in a second file checked in CI ("taking that as written").
- The redo would cover the referencing and aliasing half. The factoring must not force poor
  product behaviour; the negative answer's licence to survive a write is not to be dropped for a
  mechanical goal. "Answers are about one instant" drew hesitation and was never acked.

Found in the record: the recalled scoped negative appears once, as a lean
(`311q`, `disturbs no-other:sm.File`); a second, `kind__disjoint()`, is cited as ruled in
`plans/30W`. Nothing recorded says a scoped negative is trusted without the flag; every written
sparing rule says author plus flag.

The conductor's work **[CONDUCTOR]**:

- Scratch models (uncommitted, `.tmp/feasibility-probe/`): an apply trace with at-most claims;
  names that alias; names that move; named negatives beside open ones with the flag class; a
  two-world check that separates a bounded negative from an open one; three rules checked one
  step at a time against given answers. Each ran in milliseconds at four atoms.
- Measured on 311's generated modules: about twenty seconds of fixed cost per command; scoping
  unrelated signatures to zero gains five to ten percent; a three-line book's checks exceed
  150 s.
- Measured in assay: the shared halves work per directory; the spliced half may declare its own
  signatures and predicates, above the document's definitions; a document cannot open another.
- Withdrawn by the conductor after the human's objection: "nothing is licensed without a
  closure" (false where nothing ran above, and for named negatives); a negative answer meaning
  only "not one thing".
- The first plan cut the territory by product question (core, naming, extent, effect, order).
  Its text is at `c4b9761d`. Three clean-context reviews are `notes/313b` to `notes/313d`, and
  `notes/313a` adjudicates them: the per-rule step check and the induction over derivations
  stood; the cut does not divide the work.

## § 2-the-research-aside-and-the-ratchet

**[TYPED]** The split has little value; how do other projects keep a large checked specification
tractable?

- The research round `.claude/research/design-model-mechanisation-prior-art/` gained the front
  `front-tractable-decomposition-of-checked-specifications` in its `plan.md`, gathered by an Opus
  scout as `turn08-2026-10-04-notes.md`: 34 sources, graded by the scout. The conductor checked
  about twenty quoted excerpts against seven archived copies and read no source whole.
- What it established, in one line each: nobody checks a large interdependent object whole in
  the working loop; the commonest practice is small specifications never composed; mutual
  dependence is met with circular assume-guarantee, whose composition stays on paper; the unit of
  iteration is the obligation with only what it uses in view, which Alloy lacks; the large Alloy
  developments are smaller than 311 and were paid for in hours.

**[TYPED]**, the property most worth keeping: one committed model, claims only ever added, every
claim checked against everything committed. A later claim that an earlier one was wrong must
show as a red in that model, or as a change of foundation that visibly moves committed results.
The costliest waste is a walkback that was itself wrong.

- Withdrawn by the conductor on that objection: small throwaway models as the route.
- **[CONDUCTOR]** two leaks in that property today: a fact added to green a new claim cannot red
  an earlier check, so only a dead witness shows it; a committed law edited in place under its
  own name still passes if its result is unchanged (~SUSPECT, from `30Y`'s matching rule).

## § 3-the-assay-lane-and-the-rewritten-plan

**[TYPED]** The plan is rewritten for checking speed and for reading locality only, with
decomposition kept as a route not taken; an Opus builder does the mechanical assay work in its
own worktree.

- Measured: 311's fixed cost per command is one fact, the one that fills the walk's table for
  every pair of keys. With both table facts removed a law that never reads them fell from 20.9 s
  to 1.4 s; with the walk table's fact kept it stayed at 22 s. **[CONDUCTOR]** the proposal: a
  defining fact becomes a named premise that only the commands reading the table list. It is a
  specification edit and was not made.
- `notes/313` now carries that plan (commits `ebbf740c`, `a48a9e29`).
- The builder lane: branch `ai/assay-document-opens-document`, worktree
  `.tmp/trees/assay-document-opens-document`, base `404b0516`. The feature: a document may open
  a sibling document. Conductor rulings given to it: the opener sees the sibling's definitions
  only, emitted as their own module; opens of Alloy's bundled library pass through; the name is
  assay's existing munge of the stem.
- State when this section was written: the builder reports the feature built and tested, with
  311's compiled output and all of its lock keys unchanged. The cross-platform gate did not
  finish (a low-memory reaper stopped it). The builder was sent back to rebase, to correct the
  `--write` line in `spike/crates/internal-tooling/AGENTS.md`, and to finish the gate one leg at
  a time. The branch is not folded, and `LIVING_STATUS.md` carries no entry for the lane.
- Left out of the lane on purpose: the two leaks of § 2, a fully pinned example form, and a
  declared class per law.
- Afterwards: the builder reported both gate legs green, each run alone, on its rebased tip. The
  conductor confirmed the branch touches nothing under `specs/` and not the Java adapter, and
  folded it into `ai/main` at `067ef686` by rebase and fast-forward. That last rebase crossed
  one documentation-only commit made after the gate ran. The conductor did not read the
  builder's code. The worktree is left in place with the builder's logs.
