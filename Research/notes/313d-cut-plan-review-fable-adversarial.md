# 313d — Cut plan review (Fable, adversarial)

> Adversarial review of `notes/313-cut-plan-for-the-identity-territory.md` at `c4b9761d`.
> AI-authored; nothing ruled. The plan's § 0 is taken as given. Every Alloy result is process
> evidence: AI-run, at the stated bound, on small variants of the plan author's own uncommitted
> scratch models; the variants were deleted after the run, so each delta is stated in words.
> `+SURE` / `~SUSPECT` mark confidence. Findings are ordered most damaging first; dropped
> suspicions follow.

## § 1-findings

### fnd-separation-is-a-held-hole-not-todays-speech

- Sentence (§ 3.4): "the negative answer is the licence's static half, as before, and the
  statements an author must make are the same ones; only the document that reads them changes."
- Failure: the specification's DISJOINT is checked to mean "no common MReferent"
  (`law_disjoint_is_sound`), which is the plan's own too-weak "not one thing". The shared-part case
  is a held hole there, `hole_two_separated_things_reach_one_thing_beneath`, fenced out of
  `law_sparing_is_sound`. `313:cut-the-negative-answer-is-separation` answers that hole by
  definition, and no existing statement derives it: `:guarantees-unique-name` yields two things,
  never two things with no common part. Counter-world: `/dev/sda` and `/dev/sda1`, one store, two
  unique names, one a part of the other; `dd of=/dev/sda` above a fact about `sda1`.
- Consequence: a new per-store statement ("members share no part", a closure over a membership), or
  the sibling rule is dropped. The first is more speech, plausibly flag-class for every separation;
  the second is `313:risk-the-factoring-forces-more-guarding`.
- Check: the author's `f_rules` plus a `parts` relation and an acyclic-parts premise. The
  different-spelling rule concluding not-one-thing: no counterexample at 4. The same rule
  concluding separation: counterexample at 4.
- Confidence: +SURE of the red and of the hole's text; ~SUSPECT of which consequence follows.

### fnd-the-instant-seam-forgets-the-writers-names

- Sentence (§ 3.5): "If every write between the instant and the use is separate from all of those,
  then ... each name still denotes the same thing in the same state", under "nothing static
  promises persistence".
- Failure: the argument carries only the fact's names forward. A write's names resolve when the
  write runs, and the separation the engine holds is about the probe instant. An earlier line can
  repoint a later line's name onto the fact's thing while staying separate from the fact's thing
  and route: `ln -sfn /var/lib/dpkg /srv/x; : >/srv/x/status`, above an elided install whose fact
  read `/var/lib/dpkg/status`. Every statement is true at its own instant.
- Check: the author's `c_renames` with the unique-name warrant true at instant zero only. A run
  with every at-most true when its line runs, the warrant true at resolution, and an elided line
  under-executed: sat at 4, 8 steps; the instance is that shape. The control, with the warrant
  true for the whole run: no counterexample at the same bound. So the only scratch with renames is
  green through a warrant that promises persistence, and in the witness that warrant is falsified
  by the book's own act alone, against `313:prop-acts-never-durations`.
- A repair exists (a write's names are facts with routes; a disturbed route makes the write's
  footprint top). The plan does not state it, and it costs guarding.
- Confidence: +SURE of the world in that model; ~SUSPECT of the repair's price. The plan does not
  cite `c_renames`; § 3.5 is declared paper.

### fnd-the-bounded-class-moves-with-what-is-called-named

- Sentence (§ 3.3): "Checked in a scratch model: ... 'touches no member of this sort but these'
  are bounded; 'touches nothing but these' is not."
- Failure, first leg: the two "worlds" of `e_locality` differ only in what the writer touches;
  sort membership is shared. A sort's membership is the unclosed world of
  `313:prop-the-flag-has-one-closed-meaning`. With membership differing between the worlds, the
  sort-scoped negative is not local: counterexample at 4. The everyday at-most claim was placed
  outside the flag by a frozen relation.
- Second leg: the class is decided by `named`, which is authored data. A sort holding every cell
  gives the sort-scoped negative the truth condition of "nothing but these" (no counterexample at
  4), yet one is bounded and the other is not (sat at 4). A class an author moves by declaring a
  wider named set is kind-gating by another route (`271:rul-no-claim-type-gating`).
- Third leg (reading only): the floor witness in `d_named` exists because `needs: one Cell` makes
  the fact's dependence structural. In the specification it is speech under `ClosesMayRead`, a
  "nothing but these". Every survival through separation then rests on an unbounded negative.
- The plan says what a sort names is unruled; the fault is "checked", and three corollaries
  resting on a class with no fixed extension.
- Confidence: +SURE of the two mechanical legs; ~SUSPECT of the third.

### fnd-every-answers-meaning-lands-in-the-core

- Sentence (§ 3.7): "Adding a rule document changes no other document's text or key."
- Failure: effect and order add answers (§ 1's table). Order uses effect's answer only as a given
  (§ 2.3), so that answer's world meaning, and its nouns (write, fact, instant), must sit in the
  core. The core is `shared.assay.md`, in every row's loaded closure, which the key hashes. So the
  two later documents each re-key and re-measure every row, and a changed rightness definition is
  not something "the new signature scoped to zero" can test. The alternative, each document
  holding its own copy of the meaning, is `313:risk-two-documents-drift-at-their-interface`.
- "Documents of comparable estimated size" is also doubtful: the hard nouns pool in the core.
- Check: reading (§ 2.3, § 3.7, the `assay` task's key definition). No run.
- Confidence: +SURE of the re-key; ~SUSPECT of the size claim.

### fnd-overlap-is-not-exclusive-or-not-positive

- Sentences: "Overlap is a positive answer" (§ 3.4); "under true speech both would be right, which
  is impossible, so under true speech old answers stand" (§ 3.7).
- Failure: as defined ("one is inside the other, or they share a part"), a same pair is an overlap
  pair. Two different definite answers are then both right, the given table holds one answer per
  pair, and a sound new naming rule moves a recorded overlap to same under true speech. If overlap
  is narrowed to exclude same, it asserts two things, and by § 3.2's own argument it needs a
  negative statement.
- Check: logic; one check that same and overlap exclude each other, counterexample at 4.
- Confidence: +SURE of the dilemma; ~SUSPECT that rows move in practice.

### fnd-true-speech-arms-a-loaded-false-statement

- Sentences (§ 3.6): "loading true speech never makes a right answer wrong"; "That world is the
  false-closure cell, and it is flag-class."
- Failure: § 3.1 protects an answer by its own chain. A true statement can complete a derivation
  whose other premise was already loaded and false: a false one-thing-per-spelling waits for a
  true "the parents are the same", and a withheld answer becomes a wrong same. The false statement
  there is not a closure, so the world is not confined to the flag class. What survives is
  attribution: the chain holds the false statement.
- Check: argument on the author's `f_rules` rule shape. No run.
- Confidence: +SURE.

### fnd-the-whole-design-tier-has-no-measured-path

- Sentence (§ 3.1): "The assembly check in CI is the cross-check of the paper step, at a bound."
- Failure: § 2.2 measures cost as following the loaded closure. The assembly, the books, and the
  goal documents load all of it, the closure whose line checks timed out at 150 s, and need a
  feature assay lacks. Every § 0.1 property is whole-design, so the sittings measure step lemmas
  over a free given table and nothing else. The plan also does not say how a fixpoint is encoded:
  a table merely closed under the rules admits self-supporting cycles, which induction on depth
  does not cover, and naming and extent consume each other's answers.
- Check: reading only.
- Confidence: ~SUSPECT.

## § 2-suspicions-dropped

- The induction of § 3.1 holds for well-founded derivations. The given-table hypothesis reduces
  to "the used answers are right", and other documents' statement kinds do not disturb it.
- "The chain is assumed, never checked": dropped. The step check's premise list is the chain, and
  a shorter one is a check under fewer premises. One gap remains, minor: with two derivations, the
  first corollary of § 3.3 needs the answer to carry the bounded-only one.
- § 3.3's three corollaries follow from § 3.1 once the class is fixed.
- § 3.2: the all-overlapping world is inadmissible under the specification's own premise that no
  store is among its contents, so the proof as written fails there. A single-chain world appears
  to repair it, and no separating counter-world was found. Dropped as a fault; the property should
  stay a checked law (`law_nobody_spoke_declines`), not a paper theorem.
- § 2.1's account of Alloy and § 2.6's premise-versus-fact rules: read against the praxis, no
  fault found, untested.
- Step-check cost: every scratch command above solved in under three seconds.
