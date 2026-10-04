# 313 — Cutting the identity territory into small checked documents: the plan

> AI-authored (Fable, the 2026-10-03/04 sitting, the human present). Notes-tier PROPOSAL: a plan
> for how the ground that `specs/311-identity.assay.md` covers is cut into several small
> specifications, and why the cut keeps the product's properties. Nothing here is ruled unless it
> is marked **[TYPED]** (the human typed it in the sitting) or cites a ruling by `docID:slug`;
> everything marked **[CONDUCTOR]** is the conductor's and unacked. The root docs,
> `spike/AGENTS.md`, the welds, and stamped `plans/` outrank this note. It is written to be
> attacked: an adversarial review follows it before any build.
> Every Alloy fragment below is illustration, never specification text
> (`30Z:form-strawmen-are-quarry-never-seed`).

## § 0-the-framing

**[TYPED]** The territory is cut into smaller documents, and the design is then grown inside
them one ruling at a time while every recorded check stays green.

The territory, in one sentence: a fact was measured before the apply; a line above the fact's
own line really ran; may the fact still be relied on, and on whose word.

### § 0.1-what-the-cut-must-yield-for-the-product

Each property below gets one home, small enough to be ruled on in one sitting and stated in
product terms. A **[TYPED]** tag here marks a requirement the human typed in the sitting; the
sentence that states it is the conductor's wording and is itself unacked.

- `prop-never-under-execute-on-true-speech` — an answer that lets an elision stand past a line
  that ran is right whenever every statement it rested on is true (IMPLEMENTATION, the ladder of
  sins).
- `prop-attribution-is-the-output` **[TYPED]** — every answer carries its chain, the statements
  it rested on. A wrong answer has a false statement in its own chain, never merely somewhere in
  what was loaded.
- `prop-silence-licenses-nothing` — no answer that separates two things exists without a
  negative statement that somebody made.
- `prop-the-negative-answer-is-the-survival-licence` — **[TYPED]** as a constraint: the
  factoring must not force poor product behaviour, and the human doubted that the licence to
  survive a write can be dropped from the negative answer to reach a mechanical factoring. The
  conductor's reading: the negative answer stays the static half of "this elision survives that
  write" (`30U:rul-cross-kind-sparing-needs-a-finished-definition`; `KNOBS:kSURVIVAL`).
- `prop-the-flag-has-one-closed-meaning` **[TYPED]**, the class's definition typed with a stated
  uncertainty — `--risk-faultless-skips` is one boolean. The same construction is built in both
  of its states. It turns exactly one class of judgement from guard into elide: the class that
  rests on a negative closing over an inherently unclosed world, where no concrete conflict is
  known. This is `271:rul-flag-is-razor-residue` given a modelable form.
- `prop-the-flag-free-floor-exists` **[TYPED]** — a small, unrealistically thoroughly described
  world reaches elision past a line that ran with no such open negative. In practice nearly every
  such elision will need the flag; the floor exists so that the flag's meaning is constrained.
- `prop-the-closure-is-assertion-and-consent` **[TYPED]** — a line such as
  `disturbs nothing-else` is one object: an assertion that can be false, and its author's
  revocable consent to carry that risk.
- `prop-acts-never-durations` — Dorc models the disturbing act and never how long something
  stays true (`312f:the-humans-responses`); change made by anything outside the book is the
  horizon (`ANALYZER-NEEDS` `an-toctou-window`).
- `prop-two-orders-stay-apart` — the order in which speech is loaded and the order in which
  lines mutate the host are different things and are never modelled as one axis.

Product risks the cut itself can introduce, each answered in § 3:

- `risk-a-statement-split-across-documents` — one authored line whose meaning is half in one
  document and half in another.
- `risk-an-answer-without-a-consumer` — a document whose answers mean nothing to the product
  alone, so that no ruling about them can be stated in product terms.
- `risk-the-factoring-forces-more-guarding` — a seam that makes the composed design more
  conservative than the uncut one.
- `risk-the-engine-reads-the-flag` — the flag changing an answer instead of a consumer's use of
  it.
- `risk-two-documents-drift-at-their-interface` — one document assuming what another only
  checked.

### § 0.2-what-the-cut-must-yield-for-the-process

- Small units of analysis: one noun or one statement kind per sitting.
- Every red is an answerable finding inside what has been ruled so far: it involves the core's
  nouns and one statement, or it names two rules that disagree.
- Expensive and whole-design checks run in CI, beside the sittings, never inside them
  **[TYPED]** (the flag class "taking that as written"; a mutation-shaped lane for redundancy).

Process risks:

- `risk-an-unrun-check-reads-as-green`.
- `risk-a-late-red-undoes-earlier-rulings`.
- `risk-the-composition-rests-on-paper` — Alloy checks each piece at a bound; the argument that
  the pieces compose is not itself checked by Alloy.
- `risk-a-core-noun-is-wrong-early` — every document rests on the core, so a wrong noun
  re-founds all of them.
- `risk-tooling-assumed-and-absent`.

### § 0.3-the-goal

**[TYPED]** Mostly monotone refinement that stays green: each sitting adds one thing; every
recorded result stays where it was unless the sitting meant to move it; a moved result is a
finding (`30Z:loop-a-reword-moves-nothing`).

## § 1-the-cut

**[CONDUCTOR]**, unacked. Documents are cut by the product question each one answers. Names are
strawman.

| Document | Its one question | Answers it adds | Kind |
|---|---|---|---|
| core | What is a thing, a part, a name, an instant, a statement, an answer, a chain? | none | nouns, and the one law |
| naming | What does this name mean here? | same | static; positive speech |
| extent | Do these two things overlap? | separate, overlap | static; the closures live here |
| effect | What can this command touch; what does this fact depend on? | cannot-change | static, per write and fact |
| order | What ran before this line? | still-holds | the only temporal document |
| assembly, books, goals | Does it compose; does it meet the whole-design goals? | none | CI |

The first build is the core, naming, and extent: the half the human named
"referencing/aliasing" **[TYPED]**. Effect and order are the rest of the territory and come after.

Two decisions carry the cut.

- `cut-answers-are-about-one-instant` — every answer of naming and extent is about one instant
  of resolution and says so. Nothing static promises persistence. What changes a name's meaning
  is an act, and acts belong to the effect and order documents (§ 3.5).
- `cut-the-negative-answer-is-separation` — the negative answer means "no part of one is a part
  of the other". "One is inside the other, or they share a part" is its own answer, overlap.
  "Not one thing" alone is never an answer: two different things can share a part, so it could
  license nothing (§ 3.4).

Why four specification documents and not two, against the human's four criteria **[TYPED]**
(fewest dangerous seams first; then even work; clear focus; good dependencies):

- Seams. Each seam is a small, closed answer set with a chain. The seam between naming and
  extent is the razor's own seam: positive, line-sayable speech on one side, closures on the
  other (`271:rul-flag-is-razor-residue`). The instant seam is safe under the two conditions in
  § 3.5. A separate "mutable identity" document is worse: it would split one warrant's truth
  into a now-half and a later-half (`risk-a-statement-split-across-documents`).
- Work. One cut alone gives an estimated third against two thirds. Four gives documents of
  comparable estimated size.
- Focus. Each document has one product question.
- Dependencies. A star: every specification document depends on the core alone (§ 2.3).

## § 2-alloy-mechanics

### § 2.1-what-alloy-allows

These are the language's own rules.

- Opening a file is all-or-nothing: its signatures, definitions, and facts all arrive. There is
  no partial import, and no way to leave a fact behind.
- Opening is one-way. The lower file cannot see the upper one.
- Nothing below can be changed from above: no field added to a lower signature, no lower
  definition redefined. New things attach by pointing down at old things.
- A `check` or `run` is a query over its own file and everything below it. It adds no
  constraint another command inherits. Its cost follows that same closure.
- Signatures that extend one parent share the parent's scope.

What follows:

- An upper file can only add world-shapes, definitions, or facts. The first two cannot change a
  lower answer. A fact can remove worlds: a lower green stays green, a lower witness can die.
- What must not be influenced goes at the bottom; what must not be read goes at the top.
- Choosing which commands run when changes nothing about what any of them means. A command that
  did not run is unmeasured, never green; assay's lock already says so
  (`30Z:lock-unmeasured-is-not-a-pass`).

### § 2.2-what-assay-does-today

Measured 2026-10-03/04 with scratch documents in a scratch directory, compile and parse only
(process evidence; AI-run).

- The two shared halves are per directory, and both work in a new directory.
- The spliced half (`shared-laws.assay.md`) is appended after each document's commands, and a law
  in it resolves names the document defines. It may declare its own signatures and predicates;
  they land in the laws module, above the document's definitions and invisible to them.
- A document's non-command Alloy all lands in one module; its books sit above that.
- A document cannot open another document: `open <other stem>` fails with "File cannot be
  found". Assay passes the line through verbatim and does not place the other document's module
  in the output directory.

Measured on the existing identity specification's generated modules (runner defaults; the build
directory may be stale): a satisfiable `run` costs about 18 s and a law that reads little about
21 s, so translation of the closure is a fixed cost per command; each line check of one
three-line book timed out at 150 s; scoping three unrelated signatures to zero moved a law from
20.9 s to 19.0 s. Cost is set by how much is loaded, not by which question is asked.

### § 2.3-the-layout

A new directory under `specs/` with its own shared halves, so that no existing lock moves.

- The core is that directory's `shared.assay.md`: the nouns, what each answer means in the
  world, and a table of given answers (below). Declarations and definitions only.
- The one law lives once, in the spliced half, stated over names every document defines.
- Naming, extent, effect, and order are one document each. None opens another.
- The assembly, the books, and the goal documents sit above them and need the one missing
  feature (a document opening another). They are CI-tier, so the sittings do not wait for it.

The star works because of how a rule is checked. An identity answer is derived by rules whose
inputs are other answers ("same" needs "same container", up the chain). A rule is checked
against answers it is merely *given*:

```alloy
one sig Given { ans: Name -> Name -> lone Answer, why: Name -> Name -> Statement }

pred givenIsSound {
   all a, b: Name, x: Given.ans[a][b] | (all s: Given.why[a][b] | isTrue[s]) implies right[a, b, x]
}

check rule_same_spelling_is_sound {
   givenIsSound implies all a, b: Name, s: OneThingPerSpelling |
      (a.spelling = b.spelling and parentsGivenSame[a, b, s.scopeOfThings] and isTrue[s])
         implies right[a, b, SAME]
} for 4
```

Naming's rules and extent's rules use each other's answers through the given table, so neither
document needs the other's definitions. The check never computes the whole walk.

### § 2.4-the-shape-of-every-unit

One rule is one statement kind with:

- what the statement means when true, reading the world only;
- what it names, as data on the statement;
- what the engine concludes from it and from given answers;
- what enters the chain;
- the step check above, with its premise twin;
- two witnesses: a world where the statement is true and the rule fires, and a world where the
  statement is false and the answer is wrong.

Over the kinds of statement, the witnesses make six cells buildable **[TYPED]**: positive
statements all true or some false, against a closure absent, present and true, or present and
false.

### § 2.5-what-runs-when

- On every edit: parse; the document has a world; each cell is buildable.
- In the sitting: the step check of the rule being touched, and its twin.
- In CI: the assembly (all rules chained to a fixpoint), the books, the goal documents, the
  mutation lane, the larger scopes.

### § 2.6-the-rules-of-the-build

- `build-no-facts-beyond-structure` — the core and the rule documents state definitions. A rule
  about the world is a named predicate that a law lists as a premise. A premise subtracts
  obligation from one check; a fact narrows every check (`30Z:hole-fence-never-fill`).
- `build-checks-list-few-runs-state-all` — a check under fewer premises is asked about more
  worlds, so leaving a rule off a check can only turn it red. A `run` under fewer rules can find
  a world that breaks the missing rule, so every `run` states all of them.
- `build-one-engine-in-the-assembly` — the assembly checks the whole engine, never a part of it.
- `build-the-core-never-reads-the-flag` — the flag is declared in the spliced half or a goal
  document, where the definitions cannot see it.
- `build-speech-is-a-parameter` — every engine definition takes the set of statements in force
  as a parameter. Comparing two sets of speech is then a static check. Alloy's time is used only
  in the order document, for the order in which lines mutate the host.
- `build-truth-never-reads-speech` — a truth predicate takes no speech parameter. A closure
  closes over the entries of its own unit (`30Y:books-lines-speech-and-outcomes`: a load's
  claims are atomic).
- `build-new-vocabulary-points-backward` — forced by Alloy; it is what makes § 3.7's test
  possible.
- `build-upper-files-state-no-facts-about-lower-relations`, and the lower document's witnesses
  are re-run from the top.

## § 3-why-the-properties-hold

Each argument is marked as checked in a scratch model, or as a paper argument.

### § 3.1-soundness-by-induction-over-derivations

Step lemma, one per rule: if every given answer is right whenever its chain is true, and the
rule's own statement is true, then the rule's conclusion is right. Checked per rule by Alloy, at
a bound.

Theorem: every derived answer is right whenever its chain is true. Paper argument, by induction
on the depth of the derivation: the chain of a conclusion is the rule's statement together with
the chains of the answers it used, so a true chain makes every used answer right, and the step
lemma gives the conclusion. A rule that only withholds needs no lemma: "unknown" is always right.

This gives `prop-never-under-execute-on-true-speech` and `prop-attribution-is-the-output`
together: by the contrapositive, a wrong answer has a false statement in its own chain. It
holds for each derivation separately, which § 3.3 uses.

The assembly check in CI is the cross-check of the paper step, at a bound
(`risk-the-composition-rests-on-paper`).

Checked in a scratch model (three rules, two ways of combining given answers, two deliberately
wrong variants; scope 4; twelve commands as expected, the wrong variants red).

### § 3.2-no-separating-answer-from-positive-speech

Paper argument. Call a statement positive when it stays true as more is added to the
world's relations. The world in which every thing is a part of every other satisfies every
positive statement. So no set of positive statements entails that two things are separate.

So `prop-silence-licenses-nothing` is not a rule the engine must be made to obey: any engine
that satisfies § 3.1 and ever answers "separate" has a negative statement in that answer's chain.

### § 3.3-the-flag-class

Definition, by a property and not by a list of statement kinds
(`271:rul-no-claim-type-gating`): a negative statement is *bounded* when its truth is a function
of the things it names alone. Two worlds that agree on the named things agree on its truth.

```alloy
pred localToWhatItNames[n: Negative] {
   (Writer.touchesA & named[n]) = (Writer.touchesB & named[n]) implies
      (trueIn[n, Writer.touchesA] iff trueIn[n, Writer.touchesB])
}
```

Checked in a scratch model: "does not touch this cell" and "touches no member of this sort but
these" are bounded; "touches nothing but these" is not. What naming a sort names is the one
interior choice, left unruled **[TYPED]**.

A judgement is flag-class when every derivation that suffices for it uses an unbounded negative.
Three consequences, each a corollary of § 3.1 (paper argument; the three were also checked
directly in a scratch model with an apply trace):

- A wrong answer outside the flag class has a false bounded statement in its chain: it has a
  derivation of bounded statements only, and § 3.1 applies to that derivation.
- A wrong answer whose bounded statements are all true is flag-class, with a false unbounded
  negative in its chain: were there a bounded-only derivation, § 3.1 would make the answer
  right.
- With the flag off, no flag-class judgement elides, so no wrong elision exists with every
  bounded statement true.

Because these are corollaries, the flag class can live in a second file and run in CI: a red
there cannot undo a rule (`risk-a-late-red-undoes-earlier-rulings`). Because the flag is declared
above the definitions, `risk-the-engine-reads-the-flag` is closed by the module order.

`prop-the-flag-free-floor-exists` is one standing witness: a line that ran, no unbounded
negative, no flag, an elision below. It is expected red until the core has a bounded negative
statement; adding one is an addition, not a re-founding.

### § 3.4-why-the-negative-answer-is-separation

Paper argument. The survival question is a frame rule (USER_STORY stage 5): if a write is
confined to a set of things, a fact depends on a set of things, and no part of anything in the
first set is a part of anything in the second, the fact is unchanged.

- "Not one thing" is too weak for that premise: a file and its filesystem are two things, and a
  write to one changes the other.
- Separation is exactly the premise. So the static documents own every question of containment,
  and the effect document never reasons about it.
- Overlap is a positive answer: it makes a collision known, which is the "known concrete
  conflict" of `prop-the-flag-has-one-closed-meaning`.

This answers `risk-an-answer-without-a-consumer` and `risk-the-factoring-forces-more-guarding`:
the negative answer is the licence's static half, as before, and the statements an author must
make are the same ones; only the document that reads them changes.

The algebra of these answers (same with same is same; a part of a thing separate from another
is separate from it; overlap does not chain) is the core's, and is checked there. Only the
same/different half of it has been checked in a scratch model.

### § 3.5-why-one-instant-is-a-safe-seam

Paper argument.

Conditions: every answer names the instant it is about; nothing static promises persistence
(`prop-acts-never-durations`).

Claim: a fact depends on the thing it is about and on the things its names were resolved
through, its route. If every write between the instant and the use is separate from all of
those, then by § 3.4 the thing's state and the route's state are unchanged, so each name still
denotes the same thing in the same state.

- This rests on one modelling axiom, owed a ruling: what a name denotes is determined by the
  state of its route and the vantage it is resolved from.
- An unknown route is "depends on everything", so any write invalidates: the conservative floor.
- It makes naming change the effect document's ordinary business, with no theory of its own, and
  so avoids `risk-a-statement-split-across-documents`: a warrant is true or false at an instant,
  whole, in one document.
- Not checked: `GOTCHAS:a-path-is-not-a-referent`, `GOTCHAS:an-insert-renumbers-every-later-key`,
  and `GOTCHAS:recycled-keys-outrun-the-unwalled-span` are believed to fit; a name derived from
  content (`GOTCHAS:a-copy-takes-the-originals-name`) has a route that is the whole population,
  and falls to the floor.

### § 3.6-speech

- Truth never reads speech (`build-truth-never-reads-speech`), so loading another statement
  never changes whether a statement is true. This holds by construction, with no check.
- § 3.1 holds for every set of speech, since the engine takes the set as a parameter. So loading
  true speech never makes a right answer wrong.
- Harm is not monotone in speech and cannot be made so: loading a second oracle can give a false
  closure its first victim. That world is the false-closure cell, and it is flag-class.
- `prop-two-orders-stay-apart`: speech order is a comparison between two sets in one static
  world; apply order is a sequence of instants in one run. Alloy has one timeline per model and
  its temporal operators cannot compare two runs, so only apply order uses it.

### § 3.7-why-the-growth-is-mostly-monotone

- Adding a rule document changes no other document's text or key.
- Adding a sound rule keeps the whole sound: the induction of § 3.1 gains one case.
- A new rule only adds derivations. An old definite answer can change only when the new rule
  derives the opposite; under true speech both would be right, which is impossible, so under
  true speech old answers stand. Under false speech the disagreement is a known conflict, and
  the engine withholds.
- Adding a noun to the core re-keys every row. The test that nothing moved: with the new
  signature scoped to zero, every earlier result is unchanged. Unmeasured.
- A reword moves no row (`30Z:lock-a-reword-shows-as-nothing`).

"Mostly" is honest: a wrong core noun is not monotone to repair
(`risk-a-core-noun-is-wrong-early`), which is why the first sittings are the core's nouns.

