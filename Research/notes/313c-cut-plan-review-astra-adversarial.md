# 313c — Adversarial review of the identity cut

Section 0 is taken as given. Findings concern the arguments after it, not the feasibility of
ever making this cut. Web search worked; the Alloy language claim below was checked against
the primary reference. No solver result was obtained: sandboxed `mise tasks` and
`mise run alloy -- .tmp/313c-review/cyclic_support.als --command inhabited --procs 1
--timeout 30 --batch-timeout 40` failed on mise configuration access. Further solver attempts
were stopped. Counter-worlds below are checked by reading and direct evaluation, not Alloy.

## survival-does-not-frame-the-vantage

313 §3.5:

> If every write between the instant and the use is separate from all of those, then by § 3.4
> the thing's state and the route's state are unchanged, so each name still denotes the same
> thing in the same state.

The section's own axiom makes denotation a function of both route state and vantage. Its
preservation premise protects the referent and route, but does not require the vantage at use
to equal the vantage at measurement, or establish that changing it is irrelevant. Equality of
one argument of a two-argument function does not preserve its result.

Concrete book: `cd /b; ensure config`, with a prior measurement of relative `config` from `/a`.
The files and filesystem lookup state can remain unchanged while the same spelling selects a
different file. An instant label records when the answer held; it supplies no permission to
transport it between those contexts. Similarly, preserving the referent alone cannot transport
an observer-dependent verdict: 311 explicitly distinguishes two observers of one unchanged
file.

The seam needs an additional obligation: preserve or compare the vantage and relevant
observers, or include their determinants in the framed dependencies. Treating them as part of
the route can repair the argument, but that inclusion must be stated and checked. The two
conditions currently listed do not establish it.

Checked: `specs/311-identity.assay.md` §§1.10 and 2.8 (`sameTopic` compares observers);
`ANALYZER-NEEDS.md`'s cwd row (cwd is per-line and `cd` need not form a host-state wall);
scratch `c_renames.als`, which models repointing but has no vantage dimension.
Confidence: +SURE about the missing premise; this is not a claim that the seam is irreparable.

## naming-warrants-are-not-all-positive

313 §1:

> The seam between naming and extent is the razor's own seam: positive, line-sayable speech on
> one side, closures on the other (`271:rul-flag-is-razor-residue`).

Naming's example in §2.3 consumes `OneThingPerSpelling`. Under §3.2's definition of positive
speech, that warrant is not positive. In scratch `f_rules.als`, take names `p`, `a`, `b`:
`a` and `b` have equal spellings and distinct referents; only `a.parent = p`. The warrant for
`p` is true. Add just `b.parent = p`. No tuple is removed, all field multiplicities still
hold, but the warrant becomes false. This is exactly a world-relation extension under which
the plan says a positive statement must remain true.

Consequently, “produces SAME,” “line-sayable,” and “positive under world extension” cannot be
used interchangeably to justify this boundary. Naming already consumes a negative restriction
on possible referents. Whether that restriction is bounded is a further question; neither its
document nor the polarity of its answer settles it. Moving that warrant to extent would also
require specifying how naming receives its authority and support, rather than merely declaring
that naming owns positive speech.

Checked: direct evaluation of `inside` and `isTrue` in the supplied `f_rules.als`;
`specs/311-identity.assay.md` §1.5 independently gives the same uniqueness obligation in
`true_DeclaresUniqueReferent`. Confidence: +SURE that the example contradicts the claimed
positive/closure partition; no claim that every naming warrant requires the flag.

## signature-extension-can-change-lower-results

313 §2.1:

> An upper file can only add world-shapes, definitions, or facts. The first two cannot change a
> lower answer.

Counterexample: a lower module declares `abstract sig Statement {}` and
`sig Positive extends Statement {}`, and defines `only_positive` as `Statement in Positive`.
That predicate is necessarily true there. An upper module opens it and adds only
`sig Negative extends Statement {}`. Now one Positive atom and one Negative atom make the
unchanged lower predicate false. No explicit fact or redefinition was added. The lower witness
`some Positive` still exists, so rerunning witnesses alone does not expose the changed law.
Scoping Negative to zero also restores the old result and misses precisely the new worlds.

This matters for an extensible statement hierarchy: lexical inability to name an upper
signature is not semantic isolation from its instances. The proposed witness reruns and
zero-scope comparison do not justify preservation of lower checks in the assembled universe.
Check the relevant lower assertions there too, and distinguish an unchanged standalone module
from a conservative extension of its semantics.

Checked: Alloy's primary language reference, “Signature Declarations,” specifies that an
abstract signature contains the union of its extensions
([A-alloy-language-reference](https://alloytools.org/spec.html)); `30Y` §2.2 uses an abstract
claim hierarchy. Solver execution of this counterexample: not checked.
Confidence: +SURE about the language counterexample and the insufficiency of witness-only
testing; the actual future core could avoid this dependency.

## suspicions-dropped-after-checking

- **step-lemma-trusts-false-inputs** — dropped. `f_rules.als:parentsGivenSame` explicitly
  requires the consumed chains to be true. The abbreviated plan fragment omits that helper.
- **circular-derivations-refute-the-induction** — dropped. Induction over finite derivations
  is valid; the missing assembly is not evidence that it will admit unsupported cycles.
- **missing-import-support-is-concealed** — dropped. §2.2 explicitly reports assay's missing
  cross-document import support, and §2.3 assigns its need to assembly.
- **separation-is-merely-inequality** — dropped. §1 explicitly requires absence of shared
  parts; §3.4 correctly rejects unequal referents as sufficient for the frame premise.
