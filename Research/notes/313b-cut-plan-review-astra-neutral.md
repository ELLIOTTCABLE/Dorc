# 313b — Neutral review of the identity cut

Reviewed `313` at `c4b9761d`; section 0 is the given brief. Confidence labels apply to the
identified failure, not to whether a repaired design will work.

## preservation-needs-more-than-soundness

Section 3.4: “This answers `risk-an-answer-without-a-consumer` and
`risk-the-factoring-forces-more-guarding`: the negative answer is the licence's static half,
as before, and the statements an author must make are the same ones; only the document that
reads them changes.”

The consumer argument holds; the preservation argument does not follow. Separation is a
sufficient premise for a frame rule. That does not establish that every previously available
survival still has a derivation from the same authored statements after the cut. Conditional
soundness tolerates arbitrarily many missing answers; the single flag-free witness excludes
only losing *all* such answers. Section 3.5 additionally sends an unknown route to “depends on
everything” without establishing which previously usable routes become unknown.

The plan needs a correspondence obligation for existing answers and books, including support
and flag eligibility, or an explicit accounting of intended movements. Its proposed CI assembly
can carry that obligation, but presently has no stated comparison with the uncut semantics.
Thus the cut is plausible, but its claim to meet section 0 is not established.

Checked: `311` sections 2.6 and 3.3, `313` sections 2.5 and 3.4–3.5, and USER_STORY stage 5.
No comparative solver run exists here. Confidence: +SURE about the missing implication.

## new-signatures-can-change-old-answers

Section 2.1: “An upper file can only add world-shapes, definitions, or facts. The first two
cannot change a lower answer.”

Counterexample: the lower module declares `abstract sig Statement {}` and
`sig Existing extends Statement {}`, and checks `Statement = Existing`. It passes. An upper
module opens it and adds only `sig Added extends Statement {}`. The same assertion now fails:
one `Added` atom suffices. Abstract coverage changes without any authored fact. Both versions
are consistent. New declarations can also change `univ`; lexical invisibility alone does not
establish semantic independence.

Checked: separate `mise run alloy -- .tmp/astra-neutral-review/lower.als` and `upper.als`
invocations, each with `--command lowerAnswerHolds --procs 1 --timeout 30`; results were
no-counterexample and counterexample at scope 3. The lower inhabitation run was SAT.
The [Alloy language reference][alloy-reference] specifies this abstract-signature behavior
[A-alloy-language-reference-2026]. Confidence: +SURE. The proposed zero-scope regression is
useful, but does not establish preservation when the new vocabulary is populated.

## local-greens-do-not-preclude-late-reds

Section 3.3: “Because these are corollaries, the flag class can live in a second file and run
in CI: a red there cannot undo a rule (`risk-a-late-red-undoes-earlier-rulings`).”

Induction works given universally sound steps and finite, grounded derivations whose support
is the union of the supports used. Scope-4 checks supply bounded evidence for those premises.
They do not guarantee the premises at larger CI scopes: a rule whose first counterexample
needs five names passes the smaller check. Nor does soundness establish that the classification
implements exactly section 0's flag policy. The three corollaries remain true for many choices
of which statements are called bounded, including a choice that gates the wrong product cases.

Checked: all six supplied feasibility models. `f_rules.als` checks local steps, not an assembled
derivation engine. `e_locality.als` varies touched cells while fixing sort membership;
`d_named.als` distinguishes `Spares` and `AtMost` directly. Those are useful experiments, but
do not discharge classification for the identity territory. Larger-scope and assembly tests
must remain capable of reopening a rule. Confidence: +SURE about that limit; the proposed
sort interpretation remains an explicitly unruled question, not a demonstrated defect.

## truth-does-not-establish-current-consent

Section 3.1: “Theorem: every derived answer is right whenever its chain is true.”

This law does not cover section 0's revocable consent requirement. Let a closure be true,
derive a survival from it, then remove the closure from the statements in force. Retaining
that answer with its original true chain still satisfies the theorem, although permission
has been withdrawn. Taking speech as a parameter does not alone forbid such retention.

Require every usable derivation's support to be in the applicable speech set, and check
withdrawal with and without an independent remaining derivation. That preserves the requested
single assertion-and-consent object; it needs no second author-facing declaration.

Checked: `313` sections 2.4–2.6 and 3.1–3.7; the scratch models have no withdrawal check.
This is a missing obligation, not an observed implementation failure. Confidence: +SURE.

## true-additions-can-activate-falsehood

Section 3.6: “So loading true speech never makes a right answer wrong.”

Not when previously loaded speech includes a falsehood. A writer falsely claims to touch
nothing. The engine initially cannot resolve name `n`, so answers unknown, which section 3.1
counts as right. Load a true binding from `n` to a cell the writer touches: the false closure
now licenses a wrong separation. Every wrong answer still has the false closure in its chain;
conditional soundness survives. Section 3.6's next bullet effectively admits this case.

Checked: a separate scratch `speech.als`; `mise run alloy --
.tmp/astra-neutral-review/speech.als --command trueSpeechEnablesWrongAnswer --instances
--procs 1 --timeout 30` found the witness at scope 4. Separate commands checked soundness under
all-true speech and inhabited its premise. Qualify the claim by truth of the complete support,
not merely the addition. Confidence: +SURE.

## checks-that-held

- The derivation induction and its attribution contrapositive are valid under the premises
  stated above. Unknown needs no affirmative soundness lemma.
- Inequality alone cannot license survival across overlapping state; separation has a concrete
  consumer. Separating speech sets from apply-time order is coherent.
- `assay.rs` confirms directory-local shared halves, appended laws above species, and separate
  book modules. Ordinary definitions go into `species.als`; literal claim atoms are an
  exception, extracted into `claims.als`. Cross-document assembly is indeed missing.
- `assay/key.rs` and `assay/drive.rs` support the lock/key and unmeasured-result claims.
  Command selection does not change a command's meaning. Existing timing claims were not
  reproduced; they do not establish that the queried formula cannot affect cost.

[alloy-reference]: https://alloytools.org/spec.html
