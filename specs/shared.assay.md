# Shared: the prepend half

The module assay opens beneath every specification in this directory (`notes/30Y` § 2.2). It
holds only what every specification talks about and none defines
(`30Z:form-shared-holds-only-the-stable`): who speaks, what a foundation is and when it is in
force, the order of a book's lines, the null outcome of a held step, and the default scope of a
book. Everything subtler is a specification's own. How a specification is written is
`plans/30Z`; what assay does with these fences is `notes/30Y`.

## § 1-speakers-and-foundations

A foundation is the genus of what an answer rests on. The one kind declared here is the
spoken foundation, which a specification's prose calls a statement: a party states it, and the
contract trusts it. An engine's axiom is a premise of a specification's laws and no atom. A
measurement that an engine itself takes would be a second kind of foundation, with no speaker;
no specification holds one yet, so none is declared.

```alloy
sig Speaker {}

abstract sig Foundation extends Claim {}

abstract sig Spoken extends Foundation { speaker: one Speaker }

sig InForce in Foundation {}

fact { all f: Foundation | f in InForce iff f in Line.speech }
```

<!-- prose-translation -->
> A speaker is a party who can be named.
> A foundation is a claim.
> A spoken foundation is a foundation with exactly one speaker.
> A foundation is in force exactly when some line of the book carries it in its speech.

## § 2-the-order-of-lines

```alloy
fact { all l: Line | l not in l.^above }

fact { all l: Line | l.above.above in l.above }
```

<!-- prose-translation -->
> No line is above itself, directly or through other lines.
> Every line above a line above a line is above that line.

## § 3-the-null-outcome-and-the-default-scope

```alloy
pred todo[l: Line] {}

run bookScope {} for 5 but 4 Int
```

<!-- prose-translation -->
> The outcome of a held step is true of every line.
> So the step keeps its place in the book and adds a true premise to the lines below it.
> A book's default ceiling is five atoms of every kind the specification owns, integers of four bits, and argv of at most seven words.
