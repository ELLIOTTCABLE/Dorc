# 313 — Keeping the identity specification fast to check and small to read: the plan

> AI-authored (Fable, the 2026-10-03/04 sitting, the human present). Notes-tier PROPOSAL. Nothing
> here is ruled unless it is marked **[TYPED]** (the human typed it in the sitting) or cites a
> ruling by `docID:slug`; everything else is the conductor's and unacked. Grades are +SURE /
> ~SUSPECT / -GUESS / --WONDER. Every measurement is AI-run process evidence. The root docs,
> `spike/AGENTS.md`, the welds, and stamped `plans/` outrank this note. This text replaces the
> plan that `notes/313a` adjudicates; that text is at `c4b9761d`.

## § 0-the-framing

**[TYPED]** Two goals, for the specification at `specs/311-identity.assay.md` and for any
specification written like it:

- `goal-checking-is-fast` — a design ruling is checked in seconds or minutes, so that rulings can
  be made one at a time with every recorded result staying green.
- `goal-reading-is-local` — a person or a model works on one part of the design while loading
  only that part.

**[TYPED]** One constraint that both goals serve and neither may weaken: there is one committed
model; claims are only ever added to it; every claim is checked against everything committed. A
later claim that an earlier one was wrong has to appear as a red in that model, or as a named
change to its foundation that visibly moves committed results.

What this plan is not for: making the checker's work smaller by cutting the specification into
separately checked parts. That hope is set aside; § 4 keeps what was learned about it.

What the plan must not cost the product: a performance change that silently changes what a law
means; a split for reading that lets two parts drift apart; a result that reads green because it
was not run.

## § 1-performance

### § 1.1-the-measured-cost-and-its-cause

Measured 2026-10-04 on the specification's generated modules (runner defaults; the build
directory may be stale against the document).

| command | as committed | both table facts removed | only the compare table's fact removed |
| --- | --- | --- | --- |
| `law_nobody_spoke_declines` | 20.9 s | 1.4 s | 22.0 s |
| `law_nobody_spoke_declines_premise` | 18.4 s | 0.6 s | not run |
| `world_exists` | about 18 s | 1.0 s | 18.7 s |
| `law_compare_disjoint_is_sound` | timeout at 1800 s | (reads the table; result meaningless) | timeout at 240 s |

The fixed cost of about twenty seconds that every command pays is one fact: the one that fills a
table with the walk's answer for every pair of keys. A command that never reads the table pays
for it all the same. Scoping three unrelated signatures to zero, by contrast, moved one law from
20.9 s to 19.0 s.

### § 1.2-definitions-as-named-premises

`perf-a-defining-fact-becomes-a-named-premise` — a fact whose only work is to define a derived
relation (the walk table, the compare table) becomes a named predicate. A command whose formula
reads that relation lists the predicate as a premise. A command that does not read it lists
nothing.

- Why no meaning changes, +SURE as logic: such a fact is a definition. In every world exactly one
  value of the table satisfies it, so it removes no world as seen by a formula that never
  mentions the table.
- `perf-a-run-that-reads-a-table-states-it` — a `check` that omits a premise is asked about more
  worlds and can only go red. A `run` that omits one can find a world in which the table is
  garbage. So every `run` that reads a table states the premise, and a lint or a review step
  holds that.
- How it is verified: every key moves and no result moves (`30Z:loop-a-reword-moves-nothing`).
  That costs one full pass.
- What it buys: by a crude text count, 37 of the 63 law-module commands never mention a notion
  that reads a table. -GUESS, since indirect use is not counted.
- What it does not buy: the laws that read the tables, the three that do not finish among them.

### § 1.3-a-slow-command-is-fixed-before-the-next-claim

`perf-a-slow-command-is-fixed-first` — a per-command time limit is a build rule. A command over
the limit is restructured before any further claim is added. The limit's value is unruled.
Precedent: a verified storage system that held every unit under twenty seconds by this rule
(the research round's front `front-tractable-decomposition-of-checked-specifications`).

### § 1.4-for-the-laws-that-do-not-finish

Three tactics, in the order to try them; each has precedent in that front, and none is measured
here.

- `perf-step-form-for-walk-laws` — a law about the whole recursive walk is restated per rule:
  given answers that are right whenever their chains are true, this rule's conclusion is right
  whenever its own statement is true. The check reads one rule and never computes the walk. The
  review found this form valid for well-founded derivations (`313a`, what stood). That the steps
  compose is then an argument on paper beside a bounded check of the whole.
- `perf-a-human-case-split-with-a-coverage-check` — the law becomes several checks, each with a
  case condition in its premise, and one further check that the cases cover.
- `perf-explicit-witnesses-for-existentials` — a law of the form "for every … some …" names the
  witness by a function.

### § 1.5-what-was-tried-or-found-and-does-not-pay

- Scoping unrelated signatures to zero: five to ten percent, measured.
- The research tools for incremental Alloy analysis: all rely on parts that do not depend on one
  another.
- A faster tier beside a full one: assay's tiers already are that. It moves the hours and does
  not remove them.

## § 2-reading-and-locality

### § 2.1-what-the-document-is-made-of

Counted 2026-10-04 over the document's 4,351 lines: 1,276 lines inside Alloy fences (about 280
definitions and 63 commands), 694 inside book fences, 979 of translation and normative sentences,
929 of commentary. The definitions, which are what a reader must hold to work on the design, are
the smallest part.

### § 2.2-the-specification-holds-the-design-a-second-document-holds-its-tests

`read-the-specification-holds-definitions-and-their-laws` — the specification document keeps
each definition, its translation, and the law and witness that give it meaning
(`30Z:loop-unit-not-sentence`). A second document, which opens the first, holds everything that elaborates and constrains: the
books, the kills, the refuted shapes, the hole witnesses, the variants at other scopes.

- Nothing the second document declares is visible to the first. Alloy's opening is one-way.
- The checker still loads the specification for every command of the second document. This is a
  split for the reader only.
- `read-the-test-document-states-no-fact-about-the-specification` — a fact in the upper
  document could remove worlds from under a law it re-asks. It states none about the
  specification's relations.
- It needs one feature assay lacks, a document opening a sibling. A builder lane on branch
  `ai/assay-document-opens-document` is building that and nothing else.
- -GUESS the specification document would be about a third of today's length.

### § 2.3-a-red-names-what-it-rests-on

`read-a-law-lists-its-premises` — with § 1.2 and the step form of § 1.4, a law's text names the
rules it rests on. A red then involves that list and nothing else, which is what makes one unit
analysable without the rest. A shorter premise list is a stronger check (`313a`, what stood).

## § 3-what-this-plan-leaves-alone

- The design questions the specification holds open. Building it again from smaller pieces would
  meet the same questions; the first attempt at that re-found one of them within a day.
- The two known leaks in the constraint of § 0: a fact added to make a new claim green cannot
  turn an earlier check red, so only a dead witness shows it; and a committed law edited in place
  under its own name still passes if its result is unchanged (~SUSPECT, from `30Y`'s matching
  rule). Both have small mechanical answers and neither is in flight.

## § 4-routes-not-yet-taken

Cutting the specification into separately checked documents, so that each check loads less. Kept
here because a later specification may decompose where this one does not.

What was tried: documents cut by product question (naming, extent, effect, order) over a shared
core, each rule checked against answers it is merely given.

What the review found (`notes/313a`): the per-rule check and the induction over derivations
stand. The cut does not divide the work. The answers are mutually recursive; each new answer's
meaning lands in the shared core; and a check made in a lower document has to be asked again in
the assembled one, because adding a signature above can change a result below.

What other projects do (the research round's `plan.md`, the same front):

- Keep each specification small, one per concern, and never compose them. The commonest practice.
  Global coherence is argued and not checked, which is at odds with the constraint of § 0.
- Write a chain of specifications by level of abstraction, each tied to the one above by a
  refinement mapping. It needs dependence in one direction only.
- Check each part against stand-ins for the others (circular assume-guarantee). It admits mutual
  dependence. Its composition theorem is proved on paper, and its obligations are tested or
  checked by hand.
- It pays where a part has internals to hide behind a narrow interface: ten to several hundred
  times in one protocol, about three times in another, where one module was merged back.

When to look again: a specification whose parts depend on one another in one direction, or meet
only through a narrow interface. The measure to take first is how much of each part's detail the
others never see.
