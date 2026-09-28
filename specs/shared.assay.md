# Shared: the prepend half

The module assay opens beneath every specification in this directory (`notes/30Y` § 2.2). It
holds only what every specification talks about and none defines
(`30Z:form-shared-holds-only-the-stable`): who speaks, what a statement is and when it is in
force, the order of a book's lines, the null outcome of a held step, and the default scope of a
book. Everything subtler is a specification's own. How a specification is written is
`plans/30Z`; what assay does with these fences is `notes/30Y`.

## § 1-speakers-and-statements

```alloy
sig Speaker {}

abstract sig Statement extends Claim { speaker: one Speaker }

sig InForce in Statement {}

fact { all s: Statement | s in InForce iff s in Line.speech }
```

<!-- prose-translation -->
> A speaker is a party who can be named.
> A statement is a claim with exactly one speaker.
> A statement is in force exactly when some line of the book carries it in its speech.

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
> The outcome of a held step is true of every line, so the step keeps its place in the book and adds a true premise to the lines below it.
> A book's default ceiling is five atoms of every kind the specification owns, integers of four bits, and argv of at most seven words.
