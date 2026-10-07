# Pure logic: what the checks need and Dorc does not decide

Alloy that the specifications in this directory need, so that their checks mean what their
authors intend, and that says nothing about Dorc. A reader who wants to know what Dorc does, what
a user must say, or who answers for a wrong answer never needs this document. AI-authored (Opus,
the `notes/314a` sittings, the human present). Specification-tier; the root docs,
`spike/AGENTS.md`, and the welds outrank it.

1-the-admission-rubric decides what may enter, and it is strict on purpose. Fewer people read
this document than read any specification, so a product decision placed here is a product
decision that nobody reviews. The cost of the strictness falls the other way and is small: an
item of pure logic that stays in a product specification costs that specification some length,
and costs nothing in correctness.

How a specification sees this document is assay's business (`notes/30Y`), not the rubric's.

## § 1-the-admission-rubric

<!-- normative -->
> An item is one definition, premise, law, or check of this document, with its prose.
> An item enters this document only when the item meets every sentence of this section.
> An item is a truth of logic or a conservative definition.
> A truth of logic holds in every world, whatever the world contains.
> A conservative definition gives a new name to an expression that uses other names.
> A conservative definition removes no world.
> A check that does not use the new name gets the same result with the definition and without the definition.
> No fact about Dorc, about its users, or about the systems that Dorc acts on is a reason for an item.
> An item names no object that a product specification declares, and no object that `shared.assay.md` declares.
> A product specification applies an item to the objects that the product specification declares.
> An assumption about the shape of the world is never an item, even when the assumption looks like logic.
> An assumption about the shape of the world is a statement about the product, and its home is a product specification.
> An item stays out when another choice for the item could change what Dorc does.
> An item stays out when another choice for the item could change what a user must say.
> An item stays out when another choice for the item could change who answers for a wrong answer.
> An item stays out when another choice for the item could change which worlds a check considers.
> When a sentence of this section gives no clear answer for an item, the item stays out.
> The prose of an item says what the item means as logic.
> The prose of an item never says how a solver or a reader evaluates the item.
> Each item carries one sentence that says why the item meets this section.

A conservative definition is logic's conservative extension: adding the definition removes no
model of what came before, so nothing that does not mention the new name can tell that the
definition is there. A `pred` or a `fun` is always conservative. A `fact` is conservative only
when it fixes exactly one value for a new name in every world.

The naming sentence keeps this document generic, so that every specification can include it, and
keeps the product out of reach: an item that cannot name a claim cannot say anything about a
claim. Its cost is that plumbing which must name a product specification's own objects (the
per-kind conjunction in 311's premise; the tables 311's walk reads) cannot enter. Such plumbing
stays in its specification, or assay generates it.

Assumptions about the shape of the world that look like logic, and are not: no store is among its
own contents, which a bind mount that shows a directory inside itself breaks; no route passes
through itself; an MKey MRefers to at most one MReferent at one instant, which is definitional
only because a typed ruling defines what an MReferent is (`notes/314a` § 1.2), and so is a
statement about the product's terms. The last of the four tests holds because which worlds a
check considers is design (`notes/312d`, typed).
