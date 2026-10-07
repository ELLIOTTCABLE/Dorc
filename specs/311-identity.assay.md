# 311 — Identity and relation: the specification

The identity model, mechanized. AI-authored (Fable, the `r31-prep-design-duck` sittings and the
`312d` mechanization arc, the human present and adjudicating). Specification-tier and
ahistorical; nothing here is ruled. The root docs, `spike/AGENTS.md`, and the welds outrank this
document. Where it disagrees with a prior document, it is a deliberate proposal to re-litigate
that document, and 4.2-supersessions-pending-in-prior-documents registers the disagreement until
the document is rewritten. Ledgers and reviews cite the note this document replaces as `311j`.

Purpose: the abstract objects, relations, and laws of identity across mutually-unknowing
authors, stated fully enough that the concretization step (names, spellings, user-facing usage:
the `312` series) can build on them without re-modeling. Scope: abstract objects and relations
only. No syntax, no strawman sh, no UX, no gradual-enhancement ladder, no implementation. No
spellings are proposed.

How to read this document (`plans/30Z`): the fences are the law. A section that holds a fence is
mechanical, and its `<!-- prose-translation -->` block says exactly what its fences say. A
section with no fence is normative only through a `<!-- normative -->` block, and such a block
is residue the checker cannot reach. Everything else, this running text included, is commentary:
the false friends from neighbouring fields, the names an object carried before this document,
the refuted shapes (`notes/311u`), and each section's examples, illustrative and not fully
worked, all run as plain prose, so that only the two headed forms ever render as quotes. The
results of every command are in `311-identity.lock.json` beside this file;
what runs them is `notes/30Y`; the arc's ledger is `notes/312d`. The books, the kills of the
laws, and the witnesses of the held holes are in `311w-identity.tests.assay.md`, which opens this
document.

## Conventions

- A model object is written MFixedTerm. The bare word never stands in for it.
- A relation, attribute, or warrant known to Dorc only from what Dorc is given is written
  `:fixed-term`, the default that silence leaves included (a default is a spelling too). These
  are neither built into the engine nor hidden from Dorc by design, and they are what product
  design has to spell. A relation in the world,
  which the engine never holds, is a model object, written MFixedTerm. A relation the engine
  derives takes neither.
- An abstract operation is written `op()`. A concrete authored member keeps the `__name()` form.
- A derived view of a species is written species-hyphen-gloss: MKey-Primary, MParent-Catalog,
  MParent-Store. It is never a declared species.
- "Cell" stays untagged. It means a singleton MSort under its MParent
  (1.9-cell-a-singleton-sort).
- "Lookup" stays untagged. It means the relation between an MKey and what it MRefers to.
- An MKey's bytes are its MValue. An MReferent's condition is its MState
  (1.1-referent-state-and-value).
- An in-Dorc MSort or MScheme is always written with its prefix: `sm.File`, `sm.Path`.
- The four answers of `compare()` are written SAME, DISJOINT, KNOWN_UNSPOKEN, and UNKNOWN
  (3.2-compare-one-chokepoint-four-answers).
- A section is cited by its slug, as 2.6-may-write-the-writeset. Within the same paragraph or
  list item, a second citation of that section uses its number alone, as §2.6.
- A blockquote is one of the two headed forms of `plans/30Z` and nothing else; a false friend,
  a prior name, a refuted shape (`notes/311u`), or an example is plain prose.
- § 5 is non-normative in its entirety (5-the-relations-indexed-two-ways). It indexes § 1 to
  § 3 and defines nothing.
- A strong preference, not a ban: no pronoun stands for a noun. Repeat the noun within about
  three lines, or point in parentheses to the line that introduces it. "A foo bars. The foo that
  bars is brown," rather than "A foo bars. It is brown."

Spelling inside the fences:

- In the fences, a model object keeps its prose name. A signature keeps the capital M: `MKey`,
  `MSort`. A field, or a function of one atom, starts with a lowercase m: `mParent`, `mRefersTo`,
  `mToken`. A shell word is assay's `Shword`; an MKey's MValue is one.
- A statement is a spoken foundation: a party states it, and the contract trusts it. A
  statement species is a signature under the shared `Spoken`, named for the act:
  `DeclaresIdentifiedIn`, `SuppliesParent`. Its fields name what it is about. One statement
  atom in a scenario is `speaker__what_it_says`.
- Every species declares one `isTrue[s: <Species>]`, which transcribes the sentence that says
  what a statement of that species means when true. The engine's own definitions never read the
  world stratum; only the `isTrue` predicates do.
- A field name is unique across the whole document, so no join is ever ambiguous.
- A law is `check law_<slug> … for N`; its premise twin is `run law_<slug>_premise {…}` with
  no scope of its own; a kill or a refuted shape is `run kill_<slug> {…} for N expect 1`; a
  held design question is `pred hole_<slug>` with `run hole_<slug>_witness {…} for N expect 1`.
  Every run carries a scope clause, since assay reads an unscoped run as a corpus outcome.
- The four answers are the atoms of `Answer`: `SAME`, `DISJOINT`, `KNOWN_UNSPOKEN`, `UNKNOWN`.

## § 0-the-problem-and-the-law

Dorc removes a line only on connected claims that the line is unnecessary: a measurement claimed
before anything runs, plus an author's vouch. An earlier line that really runs can destroy the
claim. So the engine must decide whether the piece of the world an earlier line touches is the
same piece a later line's license depends on. That decision is identity.

Identity has two consumers with opposite failure directions. SAME lets one fact stand for
another. DISJOINT lets a license survive a write. Each under-executes when wrong. UNKNOWN is
safe for both.

No single author knows the whole path from a tool's argument to a measurable MReferent.
Viewpoint transitions, authored by yet other people, change which MReferent an MKey MRefers to
mid-book. What the engine can and cannot hold is 0.1-the-two-strata.

Two of the law's four sentences are the checks of 3.2-compare-one-chokepoint-four-answers:
where nobody has spoken the walk declines, and the walk never reaches a false SAME or DISJOINT
while every statement in force is true and the engine's axioms hold. Who must say what is 3.5-committee-law-and-attribution.
The other two sentences are the law's own terms of refutation, and stay prose:

<!-- normative -->
> The true answer is reachable once those who can know have spoken.
> A wrong answer with no false statement behind it refutes the model.

### § 0.1-the-two-strata

The engine and the world are two strata, and the fences keep them apart: the world stratum (an
MReferent, what an MKey MRefers to, what a store holds) is what the `isTrue` predicates read; the
engine's definitions read only MKeys, their declared shapes, and the statements in force.
Nothing in the fences enforces the separation; the two sentences below are the rule, and a
reviewer reads the engine's definitions for a world relation by eye.

An answer rests on foundations of three kinds, kept apart so that a wrong answer is attributed
to the right party. A statement is what a party states, trusted by contract: every species of
§ 1 and § 2, under the shared `Spoken`. An engine's axiom is a rule about sh that differential
test discharges; nobody speaks it, and it is a premise of the laws and no atom
(1.10-vantage-route-placeholder-witness). A measurement the engine itself takes would be the
third kind; this document holds none, and the first one is never filed under `Spoken`.

<!-- normative -->
> The engine knows only syntax, authored speech, and what authored probes returned.
> It never decodes an MKey and never holds an MReferent.
> It knows MReferents only through MKeys, MTokens, and MDerivations.
> The engine never holds an MState, and no MKey names one.

### § 0.2-a-world-exists

The consistency probes of `plans/30Z` § 2.4: a facts-only contradiction makes every check
green, so these runs must stay satisfiable for the life of the document. The second asks for the
strongest ordinary world, two MKeys scoped in one MParent-Store with a known chain each.

```alloy
run bookScope {} for 6 but 4 Int

run world_exists {
   some k: MKey | knownChain[k] and some k.mRefersTo
} for 6 but 4 Int expect 1

run chains_meet_in_a_store {
   some disj a, b: MKey | isPrimaryKey[a] and isPrimaryKey[b] and no (a + b).cellSort
      and knownChain[a] and knownChain[b] and a.mParent = b.mParent and a.mParent in MKey
} for 6 but 4 Int expect 1
```

<!-- prose-translation -->
> A book's ceiling in this document is six atoms of every kind the specification owns, and integers of four bits.
> Some MKey has a known MFullyQualifiedKey and MRefers to an MReferent.
> Two MKeys can share one MParent-Store, each with a known MFullyQualifiedKey.

## § 1-the-model-objects

### § 1.1-referent-state-and-value

The world stratum of 0.1-the-two-strata: an MReferent, what a store holds, and what it holds by
its own construction against what it merely re-presents (the distinction 2.3-aliases-nothing-else-the-store-warrant
needs). The engine never reads these relations; the `isTrue` predicates do.

```alloy
sig MReferent { holds: set MReferent, owns: set MReferent, affects: set MReferent, passes: set MReferent }

fact { owns in holds }
```

<!-- prose-translation -->
> An MReferent is a persisting piece of the world.
> A store is an MReferent.
> What a store holds is identified in it.
> What a store owns, it holds by its own construction.
> A write to an MReferent affects the MState of the MReferents it affects (2.5-may-read-the-readset).
> A route to an MReferent passes through the MReferents that pass to it.
> What an MReferent passes to is what is reached beneath it (1.7-resolution-and-its-traversal, 2.9-the-traversal-and-the-region-test).
> An MReferent is not an MKey, and not an MSort.

UNACKED READING, temporary (`312d:ask-aliases-nothing-else-world-reading`,
`312d:ask-world-relations-for-effects`): the world relations `holds`, `owns`, `affects`, and
`passes` are the conductor's choice of world stratum, made so that the `isTrue` predicates of § 2
have something to transcribe into; 311 names none of them. In particular `owns` against `holds`
is one reading of § 2.3's "by the store's own construction". None of this is authoritative or
acked; it stands only until the document has run under Alloy and the reading is acked or
replaced, and it is not a pattern to extend (`notes/312d` § 7).

Every identity question is ultimately "do these two MKeys MRefer to one MReferent"; the MKey's
side of that question is 1.4-key-and-its-two-views. No other term names a part. An MReferent is
not an MTopic (1.8-fully-qualified-key-topic-and-derivation), and not the set an MKey given
whole denotes: the container is the MReferent, and the set is a set
(2.9-the-traversal-and-the-region-test).

Examples of an MReferent: an inode, a database row, a package record, a kernel parameter, a running process, a machine, a mount table.

#### § 1.1.1-state-and-value

State and time are not in the fences: no fence holds an MState, and the sentences that say what
an MReferent survives are read by the invalidation rules of 3.3-invalidation-three-mutator-species,
which withdraw authority and never compute a successor. The value plane (`notes/275`) is outside
this model. What is normative here is normative as prose. The first sentence below is about the
model's objects, and no fence draws a restriction from it: the checker considers worlds that hold
a piece nobody has keyed, since a description that names only some of the world is one Dorc must
be safe under (the ruling of `notes/312d` § 21 for the sentence of 1.3.1-a-primary-scheme-and-its-sort).

<!-- normative -->
> An MReferent has one or more MKeys.
> An MReferent survives changes to its MState.
> An MReferent has the MSort of the MKey that MRefers to it.
> Two MSorts over one piece of the world is the strangers case (1.2-sort-the-declared-carrier).
> An MReferent may have parts.
> Every part is an ordinary MReferent of an ordinary MSort identified in it (1.9-cell-a-singleton-sort).
> An MReferent does not survive destruction and recreation under its old MKey.
> An MReferent does not survive a lifecycle write to what it is scoped in (3.3-invalidation-three-mutator-species).
> Before a lookup binds an MKey, that MKey names a may-set of MReferents (1.5-token-and-the-two-warrants).
> At the probe, and at each MExecution, an MKey MRefers to no more than one MReferent.
> An MState is the condition of one MReferent at one instant: what a write changes and what a read observes.
> An MState is known only through a read, which yields an MValue.
> Two MReferents may have equal MStates and stay two.
> One MReferent's MState changes and it stays one.
> An MValue is bytes a shell holds or will hold, with an exit status where one is produced.
> There are three kinds of MValue.
> The first kind is an MKey's MValue: bytes that name an MReferent, bound at a bind.
> The second kind is a captured MValue: bytes a read copied out of an MState, graded by provenance (`notes/275`).
> The third kind is a verdict's exit status.
> An MValue is what a read yields from an MState, and it is never the MState.
> Two reads of one MReferent at two instants may yield two MValues.

### § 1.2-sort-the-declared-carrier

Many-sorted logic's carrier. Never a PLT kind. Never "the kind of thing". Pre-311 documents write _kind_.

A carrier in the logician's sense, a domain of discourse someone chose to speak in. It has
reverse-DNS naming, no registry, and owner-adjudication as its social contract. An MSort fixes
only what its owner declares under it, each a statement species of its own section: which
MScheme, if any, is its primary MScheme (2.2-primary-of-and-identified-in); its may-read set
(2.5-may-read-the-readset); its may-write entailment and finished definition
(2.6-may-write-the-writeset); its `:observer-dependence`
(2.8-observer-dependence-and-independence); its cells (1.9-cell-a-singleton-sort). Each of
those species carries the fact that its speaker is the MSort's owner. An MSort has no MKeys and
no `resolve()`: an MKey names an MScheme (1.4-key-and-its-two-views), and the floor of
1.3-scheme-a-way-of-writing lets an MScheme precede its MSort's name. An MSort is named only
when something is declared about it as a whole; naming is not in the fences.

```alloy
sig MSort { sortOwner: one Speaker }
```

<!-- prose-translation -->
> An MSort is one owner's declared vocabulary.

#### § 1.2.1-the-strangers-case

Several MSchemes into one MSort is the cooperative case: one owner admits several ways of
writing down what they describe. Several MSorts over one world-thing is the ordinary strangers
case. The sentences below are the laws of 3.2-compare-one-chokepoint-four-answers seen from the
MSort's side; they stay prose until that section's checks carry them.

<!-- normative -->
> An MSort is not a category of the world.
> It is never valid where an MScheme is.
> The engine never knows what an MSort denotes.
> It never assumes two MSorts denote disjoint MReferents.
> The strangers case is undetectable to the engine.
> It reads KNOWN_UNSPOKEN at the chokepoint (3.2-compare-one-chokepoint-four-answers).
> Only a human act merges it, by making one MSort's MSchemes yield into the other's.
> The owner speaks only about the MSort's relations to its immediate neighbours.

The strangers case: two vendors describe one tool, or two vocabularies name one cell under `/proc/sys`.

### § 1.3-scheme-a-way-of-writing

A term language over a carrier. Never itself an MSort.

An MScheme fixes its `resolve()` (3.1-identity-of-a-key), whether it is `:primary-of` an MSort
with its declarations per matched shape (2.2-primary-of-and-identified-in), what it `:yields`
per matched shape and where its MKeys are looked up when it is secondary
(2.1-yields-into-another-scheme), and its lookup warrants (1.5-token-and-the-two-warrants).
A shape is where every per-shape declaration
hangs, so it is declared here; what a shape is, a control-flow path of the owner's body, is
1.5-token-and-the-two-warrants's. The floor's "no warrants" is a fact of that section, and its
"identity `resolve()`" and "the MRoute as its only MParent" follow from 3.1-identity-of-a-key
and 1.6-parent-one-per-key. An MSort with no MScheme at all has singleton MKeys under MParents
(1.9-cell-a-singleton-sort), and a mark of the form `parent-key@sm.Sort` is that section's.

```alloy
sig MScheme { schemeOwner: one Speaker }

sig MShape { ofScheme: one MScheme }

sig DeclaresPrimaryOf extends Spoken { primaryScheme: one MScheme, ofSort: one MSort }

fact { all d: DeclaresPrimaryOf | d.speaker = d.ofSort.sortOwner }

fun primaryOf[s: MScheme]: lone MSort { (DeclaresPrimaryOf & InForce & primaryScheme.s).ofSort }

fact { all k: MSort | lone (DeclaresPrimaryOf & InForce & ofSort.k).primaryScheme }

fact { all s: MScheme | lone primaryOf[s] }

pred floor[s: MScheme] {
   no primaryOf[s]
   no (DeclaresYields & InForce).fromShape & ofScheme.s
}

pred isPrimary[s: MScheme] { some primaryOf[s] or floor[s] }

fact { all s: MShape | floor[s.ofScheme] implies no identifiedIn[s] and not isRoot[s] }

pred isTrue[d: DeclaresPrimaryOf] {}
```

<!-- prose-translation -->
> An MScheme is a way of writing down which MReferent is meant, with one accountable owner.
> Every shape belongs to one MScheme.
> The MSort's owner declares `:primary-of`, on the primary MScheme (2.2-primary-of-and-identified-in).
> An MSort has at most one primary MScheme.
> An MScheme is `:primary-of` at most one MSort.
> An MScheme that declares neither `:primary-of` nor `:yields` is the floor.
> The floor is the primary MScheme of an MSort that nobody named.
> No shape of the floor carries `:identified-in` or `:root`.
> A `:primary-of` declaration claims nothing about the world.

#### § 1.3.1-a-primary-scheme-and-its-sort

The first sentence below is about a well-formed description, and no fence draws a restriction
from it: the checker considers every world, those with a primary MScheme that never yields into
its MSort among them, since a description that fails a rule of form is still one Dorc must be
safe under. It is held for the design sitting with a lean to remove it (`notes/312d` § 21). The
second sentence says what a shape is; the fences hold a shape as an atom of its MScheme and
nothing of the body it is a path of.

<!-- normative -->
> Where an MSort has a primary MScheme, that MScheme yields into the MSort for at least one shape.
> A matched shape is a control-flow path of the owner's body.

### § 1.4-key-and-its-two-views

RDBMS primary key and natural key, with their culture: the natural key is user-typed, may alias, and is never identity. The primary key is what the store answers with. Pre-311 documents write _entity_.

An MKey is a plan-time object that models what a runtime string will denote. It is minted at a
bind, or at an emission point a `resolve()` declares (2.1-yields-into-another-scheme), before
any lookup runs. Every lookup is a measurement: what it decides late is which declared shape an
MValue matches, so which MSort an MKey resolves into, and which MParent MSort and warrants apply, is
known only once bytes arrive. The MPlaceholder, for an MValue or an MParent not yet measured, is
1.10-vantage-route-placeholder-witness's. An MLevel is what a MFullyQualifiedKey passes through
(1.8-fully-qualified-key-topic-and-derivation): an MKey, or one of the world atoms that end a
chain. The `mRefersTo` field is world stratum (0.1-the-two-strata) although it sits on the MKey;
no engine definition reads it. That an MKey MRefers to at most one MReferent is a definition and
not an assumption about the world (`notes/314a` § 1.2 and § 2.3). The two views coincide when an
MSort's only MScheme is its primary MScheme, and the primary view is where the dangerous
warrants can honestly sit.

```alloy
abstract sig MLevel {}

sig MKey extends MLevel {
   mValue: one Shword,
   scheme: lone MScheme,
   cellSort: lone MSort,
   shape: lone MShape,
   mParent: lone MLevel,
   yielded: lone MKey,
   at: one MVantage,
   mRefersTo: lone MReferent
}

fact { all k: MKey | some k.scheme iff no k.cellSort }

fact { all k: MKey | some k.cellSort implies no k.shape }

fact { all k: MKey | k.shape.ofScheme in k.scheme }

fact { all a, b: MKey | a.scheme = b.scheme and a.mValue = b.mValue implies a.shape = b.shape }

fact { all k: MKey | no yieldsTo[k.shape] implies no k.yielded }

fact { all k: MKey | some k.yielded implies k.yielded.scheme = yieldsTo[k.shape] }

fact { no k: MKey | k in k.^yielded }

fun keysOfShape[s: MShape]: set MKey { shape.s }

fun keysOfSort[k: MSort]: set MKey { {x: MKey | primaryOf[x.scheme] = k or x.cellSort = k} }

pred isPrimaryKey[k: MKey] { (isPrimary[k.scheme] and no yieldsTo[k.shape]) or some k.cellSort }

pred isNaturalKey[k: MKey] { not isPrimaryKey[k] }
```

<!-- prose-translation -->
> An MKey has three parts: an MValue, its MScheme, and its MParent (1.6-parent-one-per-key).
> The MScheme is always declared, with one exception.
> An MValue with no MScheme is nothing.
> A cell's MKey has its cell MSort in place of an MScheme and matches no shape (1.9-cell-a-singleton-sort).
> There is no default MScheme.
> A bind or a mark always names one.
> The MValue is a literal: the shell word bound at the bind.
> The MParent is the instance one of the seats of 1.6-parent-one-per-key supplies, or none, which leaves the MFullyQualifiedKey unknown from that level.
> A lookup chooses among the shapes its MScheme declares, never outside them: the shape an MKey matches is a shape of its own MScheme.
> Which shape an MValue matches is a function of the MKey's own bytes: two MKeys of one MScheme with equal MValues match one shape (1.6-parent-one-per-key).
> An MKey may carry the MKey its lookup emitted for it, minted at the emission point the `resolve()` declares.
> That emitted MKey is an MKey of the MScheme its shape `:yields` (2.1-yields-into-another-scheme).
> An MKey whose shape yields nothing carries no emitted MKey.
> Every MKey was resolved from one MVantage (1.10-vantage-route-placeholder-witness).
> No MKey is its own yield, directly or through others.
> An MKey MRefers to one MReferent, or to none (2.2-primary-of-and-identified-in).
> By definition, an MKey cannot MRefer to two MReferents.
> Everything that an MKey MRefers to is an MReferent.
> MKey-Primary is an MKey of the primary MScheme on a shape that yields nothing, or a cell's MKey (1.9-cell-a-singleton-sort).
> It is meaningful only relative to its MParent-Store.
> MKey-Natural is any other MKey, what tool authors and books write.

### § 1.5-token-and-the-two-warrants

OWL's inverse-functional and functional properties, spelled out by direction.

What each warrant licenses, SAME from equality and DISJOINT from inequality, is the walk of
3.2-compare-one-chokepoint-four-answers; here is what each means when true. "Within one
MParent" is read at the world stratum: two MParents that MRefer to one MReferent, or one MWorld.
The instance of a warrant is per evaluation, and it may rest on what the path measured: since a
matched shape is a control-flow path, an evaluation that declines the warrant has matched a
shape that does not carry it.

```alloy
fun mToken[k: MKey]: lone Shword { isPrimaryKey[k] implies k.mValue else none }

sig DeclaresUniqueReferent extends Spoken { referentShape: one MShape }

sig DeclaresUniqueName extends Spoken { nameShape: one MShape }

fact { all d: DeclaresUniqueReferent | d.speaker = d.referentShape.ofScheme.schemeOwner }

fact { all d: DeclaresUniqueName | d.speaker = d.nameShape.ofScheme.schemeOwner }

pred guaranteesUniqueReferent[s: MShape] { some DeclaresUniqueReferent & InForce & referentShape.s }

pred guaranteesUniqueName[s: MShape] { some DeclaresUniqueName & InForce & nameShape.s }

fact {
   all s: MShape | floor[s.ofScheme] implies
      not guaranteesUniqueReferent[s] and not guaranteesUniqueName[s]
}

pred withinOneParent[a, b: MKey] {
   some a.mParent & b.mParent
   or (a.mParent + b.mParent in MKey and some a.mParent.mRefersTo and a.mParent.mRefersTo = b.mParent.mRefersTo)
}

pred isTrue[d: DeclaresUniqueReferent] {
   all a, b: keysOfShape[d.referentShape] |
      withinOneParent[a, b] and a.mValue = b.mValue implies a.mRefersTo = b.mRefersTo
}

pred isTrue[d: DeclaresUniqueName] {
   all a: keysOfShape[d.nameShape], b: MKey |
      b.scheme = a.scheme and withinOneParent[a, b] and some a.mRefersTo and a.mRefersTo = b.mRefersTo
         implies a.mValue = b.mValue
}
```

<!-- prose-translation -->
> A MToken is an MKey-Primary's MValue: bytes a `resolve()` returned.
> Any lookup, a secondary MScheme's or a primary MScheme's, may carry two warrants per matched shape.
> The two warrants are independent, separately declared, and absent by default.
> The lookup's owner declares each warrant per matched shape, and the floor of 1.3-scheme-a-way-of-writing carries neither.
> A warrant holds for every MKey of that shape inside any one MParent.
> A path that does not reach a warrant does not give it.
> So an MKey has a warrant when the shape it matched carries it.
> Two MKeys are within one MParent when they have one MParent, or when their MParents MRefer to one MReferent.
> `:guarantees-unique-referent` is true when, within one MParent, equal MKeys of the shape MRefer to one MReferent, or both MRefer to none: the lookup is a function.
> `:guarantees-unique-name` is true when, within one MParent, one MReferent that an MKey of the shape MRefers to has one MKey of the lookup.

UNACKED READING, temporary (`312d:ask-unique-name-ranges-over-the-scheme`): 311's "one MReferent
has one MKey" does not say whether the one MKey ranges over the warranted shape or over the
whole lookup; the fence takes the lookup, the reading under which the two-tops way of § 3.2
is sound, and "within one MParent" is read at the world stratum as MParents that MRefer to one
MReferent. The conductor's readings, not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).

#### § 1.5.1-the-token-over-time-and-the-per-level-closure

A MToken is compared for equality only and never decoded (0.1-the-two-strata), and it is always
scoped in an MParent (1.6-parent-one-per-key). How long a warrant holds is
3.3-invalidation-three-mutator-species's; the engine's own axiom is
1.10-vantage-route-placeholder-witness's; the per-level closure is consumed by
2.9-the-traversal-and-the-region-test, and stays prose until that section is mechanized.

<!-- normative -->
> A warrant holds while that MKey's MResolution or MToken stands (3.3-invalidation-three-mutator-species).
> One lookup rests on the engine's own axiom, discharged by differential test and never spoken: the local MRoute under no wrapper (1.10-vantage-route-placeholder-witness).
> Across a wrapper, the wrapper's author speaks (3.4-entry-and-lends), and everything else is measured and witnessed.
> It is a statement about one MKey, from the thing's end, made on the path that measured it.
> Any path the dialect admits may decline it.
> Where the lookup knows other entries, it emits them first, and a listed alias is checked as the first entry is.
> The closure and the per-shape warrant are two statements.
> A grade governs every consumer of the answer it grades, corroboration and contradiction included.
> A lookup without `:guarantees-unique-name` cannot contradict anything by returning two different MTokens.

`:guarantees-unique-referent` fails for round-robin lookups, recycled MKeys, and cloned identifiers presented as MRoots. `:guarantees-unique-name` fails for symlinks and hardlinks, package `provides`, and route-qualified handles. For a path the closure's instance is the whole mount namespace. For a file, the evidence is a link count of one. For a directory, the evidence is that no other mount exposes it. A hardlink or a bind mount is where the closure is withheld.

### § 1.6-parent-one-per-key

What the MParent edge carries follows the MScheme (2.4-parent-as-a-relation): routing through a
secondary MScheme, identity through the primary one, and the walk of
3.2-compare-one-chokepoint-four-answers reads the identity edge. The declarations that fix the
MParent's MSort per shape are 2.2-primary-of-and-identified-in's; that a shape is a function of
the MKey's own bytes is 1.4-key-and-its-two-views's. The supplying seats, three of them, and
what a disagreement between them costs, are 1.6.1-the-three-seats. Nothing is a MRoot by
default, and cloned identifiers are the standing witness.

```alloy
abstract sig Seat {}

one sig BindSeat, YieldSeat, DeclarationSeat extends Seat {}

sig SuppliesParent extends Spoken { forKey: one MKey, instance: one MKey, seat: one Seat }

fact { all s: SuppliesParent | s.seat = YieldSeat implies s.speaker = (yielded.(s.forKey)).scheme.schemeOwner }

fact { all s: SuppliesParent | s.seat = DeclarationSeat implies s.speaker = s.forKey.scheme.schemeOwner }

fun supplies[k: MKey]: set MKey { (SuppliesParent & InForce & forKey.k).instance }

pred parentRefused[k: MKey] { some disj p, q: supplies[k] | p != q }

pred supplyFits[k: MKey] {
   one supplies[k]
   some identifiedIn[k.shape] implies sortOfKey[supplies[k]] = identifiedIn[k.shape]
   some k.cellSort implies sortOfKey[supplies[k]] = cellParentSort[k.cellSort]
}

fact {
   all k: MKey {
      (no k.shape and no k.cellSort) implies no k.mParent
      some k.cellSort implies k.mParent = (supplyFits[k] implies identity[supplies[k]] else none)
      isRoot[k.shape] implies k.mParent = rootShape.(k.shape)
      (some k.shape and not isRoot[k.shape] and no identifiedIn[k.shape] and no yieldsTo[k.shape])
         implies k.mParent = k.at.route
      some identifiedIn[k.shape]
         implies k.mParent = (supplyFits[k] implies identity[supplies[k]] else none)
      some yieldsTo[k.shape]
         implies k.mParent = (let seats = supplies[k] + k.at.ambient[catalogSortOf[k.scheme]] |
                             one seats implies seats else none)
   }
}

pred isTrue[s: SuppliesParent] {
   isPrimaryKey[s.forKey] implies s.forKey.mRefersTo in s.instance.mRefersTo.holds
}
```

<!-- prose-translation -->
> Every MKey has at most one MParent: the MKey it was resolved inside, or the MWorld its chain ends at (1.8-fully-qualified-key-topic-and-derivation).
> An MKey that is not a cell's and matches no shape has no MParent.
> A cell's MKey has the MParent its mark supplied: an MKey of the MSort the cell is `:identified-in` (1.9-cell-a-singleton-sort).
> The cell's MKey has that MKey's identity as its MParent (3.1-identity-of-a-key).
> A shape declared `:root` is scoped in its own MWorld (2.2-primary-of-and-identified-in).
> A shape with neither `:identified-in` nor `:yields` is scoped in the MRoute of the MKey's MVantage (1.10-vantage-route-placeholder-witness).
> For a shape that yields, the MVantage's ambient instance for the MScheme's catalog MSort (2.1-yields-into-another-scheme) is one seat among those that supply the MParent-Catalog instance.
> For a shape that yields, the instance is the one the seats supply.
> For a shape that yields, there is no instance where the seats supply none or disagree.
> The MParent instance is an MKey that the seats supply, and the seats supply one instance.
> The first seat is the bind that minted the MKey (1.4-key-and-its-two-views).
> The second seat is the lookup that yielded the MKey (2.1-yields-into-another-scheme).
> The third seat is the primary MScheme's declaration for the matched shape (2.2-primary-of-and-identified-in).
> For a secondary MScheme's MKey, the third seat is the MEntryChain's instance (2.1-yields-into-another-scheme).
> A supply from the yield seat is the yielding lookup's owner's line.
> A supply from the declaration seat is the line of the owner of the MKey's own MScheme.
> For a shape with `:identified-in`, the seat names the instance as an MKey of the MParent's MSort, of any of that MSort's MSchemes.
> The MKey's MParent is the identity of that instance (3.1-identity-of-a-key).
> Where the instance has no identity, the MFullyQualifiedKey is unknown from that level.
> Two seats that disagree are a contradiction.
> Where no seat supplied an instance, the MKey has no MParent.
> Where two seats disagree, the MKey has no MParent.
> Where the supplied MKey is not of the declared MSort, the MKey has no MParent.
> In each of these three cases, the MKey's MFullyQualifiedKey is unknown from that level.
> A supplied MParent-Store instance is true when the MReferent the MKey-Primary MRefers to is held by the MReferent the instance MRefers to.
> A supplied MParent-Catalog instance claims nothing in this model.
> That the catalog is on the route to the MKey's MReferent is a held hole (3.3-invalidation-three-mutator-species).

UNACKED READING, temporary (`312d:ask-route-for-any-path-without-identified-in`,
`312d:ask-mismatched-supply-reads-unknown`): the fence scopes every shape with neither
`:identified-in` nor `:yields` in the MRoute, warranted or not, on § 1.6's sentence alone; and it
gives a MKey whose supplied instance is not of the declared MSort no MParent (unknown from that
level), where 311 says only that the seat "names it as an MKey of one of the MParent's MSort's
MSchemes" and says nothing about a mismatch. The conductor's readings, not acked, not
authoritative, held only until acked or replaced (`notes/312d` § 7).

#### § 1.6.1-what-a-seat-answers-for

The refusal's attribution is 3.5-committee-law-and-attribution's; the routes that are not the
MParent are 2.10-places-the-upward-lookup's.

<!-- normative -->
> A disagreement between seats is refused and attributed to both.
> An emitter is a secondary MScheme, possibly a stranger's.
> An emitter can be wrong only about what it supplied.
> What it supplied is its lookup, and the MParent instance where it is the seat that supplied that instance.
> An MKey may carry further routes, one per other lookup that reached it (2.10-places-the-upward-lookup).
> None of these routes is the MKey's MParent.

Examples of a MParent-Catalog: a directory for a path's entry, a passwd database for a login name, a process table for a pid. Examples of a MParent-Store: a DNS zone for a record's owner name, a user namespace for a uid, a dpkg database for a canonical package name. Examples of a MRoot: the DNS MRoot, or a cloud instance-id whose issuer never repeats one.

### § 1.7-resolution-and-its-traversal

A MResolution is the fact that MKey N of MScheme S, resolved inside MParent P at program point
p, MRefers to MReferent R: in the fences it is the MKey's emitted MKey (1.4-key-and-its-two-views)
with the MTraversal beside it. The order the lookup crossed the members in is not held; the
consumers read membership. What touches a member and so invalidates the MResolution is
3.3-invalidation-three-mutator-species's; the read set of the lookup body is
1.7.1-the-lookup-body-and-its-reads.

```alloy
sig EmitsCrossed extends Spoken { crossedFor: one MKey, crossedKey: one MKey }

sig ClosesTraversal extends Spoken { closedFor: one MKey }

fact { all d: EmitsCrossed | d.speaker = d.crossedFor.scheme.schemeOwner }

fact { all d: ClosesTraversal | d.speaker = d.closedFor.scheme.schemeOwner }

fun crossed[k: MKey]: set MKey { (EmitsCrossed & InForce & crossedFor.k).crossedKey }

pred traversalClosed[k: MKey] { some ClosesTraversal & InForce & closedFor.k }

fun mTraversal[k: MKey]: set MLevel { crossed[k] + (traversalClosed[k] implies none else k.mParent) }

pred isTrue[d: EmitsCrossed] {}

pred isTrue[d: ClosesTraversal] {
   let k = d.closedFor | passes.(k.mRefersTo) in crossed[k].mRefersTo
}
```

<!-- prose-translation -->
> A MResolution is a fact with a backing, and the backing is the MTraversal: the chain of routing MKeys the lookup crossed.
> The lookup emits that chain, one member per routing MKey (2.9-the-traversal-and-the-region-test), as the lookup owner's speech.
> The emission is an at-most set, and an emitted member licenses nothing alone.
> The lookup's owner closes the emission by an explicit act.
> The closing act is true when every MReferent the route to the MKey's MReferent in fact passes through is one that an emitted member MRefers to.
> A lookup that emits no member and no closing act has its MParent-Catalog, given whole, as its MTraversal.
> A lookup that emits members without the closing act has those members and its MParent-Catalog, given whole.
> A lookup that emits the closing act has exactly the members it emitted as its MTraversal.

#### § 1.7.1-the-lookup-body-and-its-reads

The read set of a lookup body is not in the fences.

<!-- normative -->
> The lookup emits the chain in the order it crossed the members, and the chain is ordered.
> A mutator whose writeset touches a MTraversal member invalidates the MResolution.
> The mutator invalidates the MResolution at each site that the shell can execute after the mutator.
> A MResolution's target object is not its backing.
> An MKey can cease to MRefer to an object without the object changing.
> An object can change without its MKey changing.
> A MResolution also depends on the read set of the lookup body that produced it.
> The engine derives that set from the body.
> Shell parity supplies the reads of sh constructs.
> The path MScheme supplies the reads of a path.
> The speech that describes an external command supplies the reads of that command.
> The read set is closed only when the read set of every external command in the body is closed.
> The author of the speech that describes an external command closes that command's read set by an explicit act.

Examples of a MTraversal: each directory entry and symlink for a path, the resolver configuration and MVantage for a hostname, the unit table for a service name.

### § 1.8-fully-qualified-key-topic-and-derivation

The chain is over MKey-Primaries: `identity()` (3.1-identity-of-a-key) resolves a natural MKey
to the primary one first. The world atoms that end a chain are declared here; the MRoute is the
address of 1.10-vantage-route-placeholder-witness, and a MRoot shape's MWorld is the store of
2.2-primary-of-and-identified-in. A level's height is what the walk of
3.2-compare-one-chokepoint-four-answers aligns two chains by.

```alloy
abstract sig MWorld extends MLevel {}

sig MRoute extends MWorld {}

sig MRootWorld extends MWorld { rootShape: one MShape }

fact { all w: MRootWorld | isRoot[w.rootShape] }

fact { all s: MShape | isRoot[s] implies one rootShape.s }

fact { no l: MLevel | l in l.^mParent }

fun mFullyQualifiedKey[k: MKey]: set MLevel { identity[k].*mParent }

fun terminus[k: MKey]: lone MLevel { {l: k.*mParent | no l.mParent} }

pred knownChain[k: MKey] { terminus[k] in MWorld }

fun worldOf[k: MKey]: lone MWorld { terminus[k] & MWorld }

fun height[l: MLevel]: Int { #(l.^mParent) }
```

<!-- prose-translation -->
> A MFullyQualifiedKey is the recursive identity of an MKey: the MKey scoped in its MParent, whose identity is itself a MFullyQualifiedKey, up through the MParents.
> The MFullyQualifiedKey of a natural MKey is its identity's (3.1-identity-of-a-key).
> The recursion terminates: no level is above itself.
> It terminates at a MRoot, at the MRoute (1.10-vantage-route-placeholder-witness), or at an unknown link, an MKey with no MParent.
> An MWorld is a terminus of a MFullyQualifiedKey: each MRoot shape is one MWorld, and each MRoute is one MWorld.
> A MFullyQualifiedKey is known when it terminates at an MWorld.
> The height of a level is the number of levels above it.

#### § 1.8.1-derivations-and-the-topic

A MFullyQualifiedKey is one MDerivation of identity; the others, and what a claim is about,
are 2.7-corresponds-across-a-transition's and 2.8-observer-dependence-and-independence's, and
their combination is 3.2-compare-one-chokepoint-four-answers's.

<!-- normative -->
> A MCorrespondence may speak across MWorlds (2.7-corresponds-across-a-transition, 3.2-compare-one-chokepoint-four-answers).
> A MTopic is what a claim is about: the MKey read, plus the observer instance when that MKey's MSort is observer-dependent (2.8-observer-dependence-and-independence).
> A MTopic may carry several MDerivations with different generators: its MFullyQualifiedKey, a provider-supplied identifier, or a MCorrespondence from a transition owner (2.7-corresponds-across-a-transition).
> MDerivations combine by coherence (3.2-compare-one-chokepoint-four-answers), never by priority.

### § 1.9-cell-a-singleton-sort

Pre-311 documents write _aspect_.

A cell's MKey is written `parent-key@sm.Sort`, with the MSort's name in full reverse-DNS, and is
minted at the mark that names it (1.4-key-and-its-two-views, 1.6-parent-one-per-key); the
MScheme left of `@` is the MParent's. A cell's MReferent may hold its MState elsewhere than in
its MParent, and the MState may be diffuse: the cell's may-read set (2.5-may-read-the-readset)
says where, and the freshness of a fact about the cell follows the writesets that reach the cell
through that set (2.6-may-write-the-writeset), together with any write that covers the MParent
(2.9-the-traversal-and-the-region-test, 3.3-invalidation-three-mutator-species). Two cells of
one MParent are two MSorts, with two may-read sets and two `:observer-dependence`s, and
3.2-compare-one-chokepoint-four-answers decides between them as between any two MSorts; a
writeset entry naming the MParent covers its cells (step 2 of the walk). The marked line that
answers a cell is a read of the cell's MKey, and the fact's identity is the MTopic
(1.8-fully-qualified-key-topic-and-derivation). An MSort with no MScheme at all has only such
MKeys (1.3-scheme-a-way-of-writing).

```alloy
sig DeclaresCell extends Spoken { theCell: one MSort, cellParent: one MSort }

fact { all d: DeclaresCell | d.speaker = d.theCell.sortOwner }

fun cellParentSort[c: MSort]: lone MSort { (DeclaresCell & InForce & theCell.c).cellParent }

fact { all c: MSort | lone cellParentSort[c] }

fact { all k: MKey | some k.cellSort implies some cellParentSort[k.cellSort] }

fact { all disj a, b: MKey | some a.cellSort & b.cellSort and some a.mParent & b.mParent implies a = b }

pred isTrue[d: DeclaresCell] {}
```

<!-- prose-translation -->
> A cell is a singleton MSort identified in its MParent: its owner declares it `:identified-in` the MParent's MSort (2.2-primary-of-and-identified-in), one MParent MSort per cell.
> Its MKeys carry the cell MSort in place of an MScheme (1.4-key-and-its-two-views).
> Under any one MParent instance it has exactly one MKey.
> A cell's identity is its MParent's plus its MSort (3.1-identity-of-a-key, 3.2-compare-one-chokepoint-four-answers).
> Declaring a cell claims nothing about the world.

Example: `active` is held in the service manager's memory in the boot, and a reboot reaches it through its may-read set. `enabled` is held in a symlink in a filesystem, and survives a reboot only where its MParent is not itself scoped in the boot (3.3-invalidation-three-mutator-species).

#### § 1.9.1-a-cell-sort-and-a-scheme

The sentence below is about a well-formed description, and no fence draws a restriction from
it: the checker considers a description that declares a primary MScheme on a cell MSort, on the
ruling of `notes/312d` § 21 for 1.3.1-a-primary-scheme-and-its-sort.

<!-- normative -->
> The singleton MSort has no MScheme of its own.

### § 1.10-vantage-route-placeholder-witness

A MVantage is the address where a probe stood when the probe measured an MReferent, and every MKey carries the one it
was resolved from. What a wrapper lends, and how a vantage entered through a wrapper inherits
the rest, is 3.4-entry-and-lends; the MRoute holds no MState and declares no may-read set, so a
readset member whose MFullyQualifiedKey ends at it is ⊤ (2.5-may-read-the-readset). "Resolved
once per MEntryChain and shared" is by construction: a vantage holds one ambient instance per
MParent-Catalog MSort, so every MKey the vantage scopes in that MSort has the one instance as
its MParent, and two same-spelled leaf MKeys are two atoms, SAME only by warrant. The engine does
not speak: its one axiom, about where the shell resolves, is a premise of every law
(3.2-compare-one-chokepoint-four-answers). The placeholder and the standup `witness()` are two instants the
fences do not hold (1.10.1-placeholder-and-witness).

```alloy
sig MVantage {
   route: one MRoute,
   enteredFrom: lone MVantage,
   through: lone Wrapper,
   ambient: MSort -> lone MKey
}

fact { all v: MVantage | some v.through iff some v.enteredFrom }

fact { no v: MVantage | v in v.^enteredFrom }

fact { all v: MVantage, s: MSort | sortOfKey[v.ambient[s]] in s }

fact { all k: MKey | k.yielded.at in k.at }

fact { all f: VerdictFact | f.underObservers = MSort.(f.topic.at.ambient) }

pred shellResolvesInTheAmbientInstance {
   all v: MVantage | no v.through implies
      all k: MKey | k.at = v and isNaturalKey[k] and some k.mParent & v.ambient[catalogSortOf[k.scheme]] implies
         k.mRefersTo in k.mParent.mRefersTo.passes
}
```

<!-- prose-translation -->
> A MVantage is the MEntryChain: a finite map from MParent-Catalog MSorts to the instances in effect, each an MKey of that MSort, with its MRoute.
> A MVantage is not part of any MKey's identity.
> Every MKey is resolved from one MVantage, and an MKey a lookup emits is resolved from the vantage of the MKey it was emitted for.
> A MVantage says where a `resolve()` runs.
> A MVantage supplies the ambient MParent for every MKey of a secondary MScheme looked up in a lent MParent-Catalog MSort.
> For a shape with no `:identified-in`, a MVantage is the MRoute, the last-resort MParent (1.6-parent-one-per-key).
> A vantage entered through a wrapper is entered from the caller's vantage.
> No vantage is entered from itself, directly or through other vantages.
> The observers a fact was measured under are the instances its MEntryChain holds (2.8-observer-dependence-and-independence).
> When the shell runs under no wrapper, the ambient MParent instances rest on the engine's axiom about where the shell resolves.
> Differential test discharges that axiom, and nobody speaks it.
> The axiom concerns every MKey of a secondary MScheme that the engine scoped in an ambient instance under no wrapper.
> The axiom holds when every such MKey MRefers to what a route through that instance's MReferent passes to.

#### § 1.10.1-placeholder-and-witness

<!-- normative -->
> The MRoute is an address.
> The MRoute holds no MState and declares no may-read set.
> When the shell runs under no wrapper, the MRoute and the ambient MParent instances within one unwalled span rest on the engine's axiom.
> Each of the MRoute and those instances is resolved once per MEntryChain and shared: one MPlaceholder.
> The MValue is a literal, or a MPlaceholder for a captured MValue.
> The MParent is an instance one of the seats of 1.6-parent-one-per-key supplies, or a MPlaceholder.
> A MFullyQualifiedKey whose MTokens are not yet measured is a MPlaceholder keyed by (MKey, ambient MParents, MEntryChain).
> The probe standup binds it.
> The apply standup re-reads it through the same entry and `compare()`s the two.
> That re-read is the `witness()`.
> A mismatch is integrity, never a verdict input.
> The `witness()` cannot see a recycled MKey.
> That stays on the outside-churn horizon.

A recycled MKey the `witness()` cannot see: a reissued pid or inode.

### § 1.11-site-and-claim-species

A MSite is within a book line, assay's `Line`: it has an argv, a program point (the line's place
among the lines above it), and an MEntryChain (1.10-vantage-route-placeholder-witness). Every
species of speech is a subtype of the shared `Spoken`, with one speaker: the writeset claim
and the entailment are 2.6-may-write-the-writeset's; the MCorrespondence is
2.7-corresponds-across-a-transition's; the per-MScheme declarations are
2.1-yields-into-another-scheme's, 2.2-primary-of-and-identified-in's, and
2.9-the-traversal-and-the-region-test's; the per-MSort declarations are
2.5-may-read-the-readset's, 2.8-observer-dependence-and-independence's,
1.9-cell-a-singleton-sort's, and 2.10-places-the-upward-lookup's; the wrapper's `:lends` and
their sentinel are 3.4-entry-and-lends's. The verdict fact is declared here, since it is the
thing every sparing is about. Its `dependsOn` field is world stratum (0.1-the-two-strata): the
MReferents whose MState the measured answer in fact depended on, which no engine definition
reads.

```alloy
sig VerdictFact extends Spoken {
   topic: one MKey,
   atLine: one Line,
   markedReads: set MKey,
   underObservers: set MKey,
   dependsOn: set MReferent
}
```

<!-- prose-translation -->
> A verdict fact is the measured answer to a read of an MKey, a cell's MKey included.
> A verdict fact is taken at a MSite, under the instances its MEntryChain lent (3.4-entry-and-lends).
> Its readset is the body's marked reads (2.5-may-read-the-readset).
> The tool-oracle author vouches it: the vouch is the fact's speaker.

#### § 1.11.1-speech-is-typed

<!-- normative -->
> Speech at or about a MSite has one author per claim.
> Every warrant is typed speech, never an exit status.

#### § 1.11.2-executions-of-a-site

A compiler's program point against its dynamic instances: an MSite is a place in the book's text,
and an MExecution is one time the shell executes it. The verb is POSIX's, for which a simple
command is executed even when it names no command (POSIX.1-2024, XCU 2.9.1, Simple Commands). A word built on
"call" or "invocation" assumes a callee, which an assignment or a lone redirection lacks.

<!-- normative -->
> An MExecution is one time that the shell executes an MSite.
> One MSite can have more than one MExecution.
> In a loop, an MSite can have one MExecution in each iteration.
> In a function, an MSite can have one MExecution each time the function runs.
> In a loaded file, an MSite can have one MExecution each time the shell loads the file.
> When the book runs again, its MSites have new MExecutions.

## § 2-the-model-relations

Each relation states its arity, who declares it, its default, which consumer reads it, and which
warrant makes it dangerous.

### § 2.1-yields-into-another-scheme

S's `resolve()`, run in the MVantage, maps an MKey of S to an MKey of T; the emitted MKey sits
on the input MKey (1.4-key-and-its-two-views), and the lookup may supply the emitted MKey's
MParent instance from the yield seat (1.6-parent-one-per-key). Where S's own MKeys are looked
up is S's MParent-Catalog, the MParent of a natural MKey, supplied by the bind, by S's owner's
declaration, or by the MEntryChain's instance for that MSort
(1.10-vantage-route-placeholder-witness). Arity: per matched shape of a secondary MScheme, into
any MSort. Declared by: S's owner. Default: none; an MScheme that declares neither `:yields` nor
`:primary-of` is a floor primary MScheme (1.3-scheme-a-way-of-writing). Consumer: MResolution
(1.7-resolution-and-its-traversal) and the MFullyQualifiedKey (3.1-identity-of-a-key). Danger:
the lookup warrants; a wrong yield or a wrong supplied instance is a wrong SAME or DISJOINT,
attributed to the yield.

```alloy
sig DeclaresYields extends Spoken { fromShape: one MShape, intoScheme: one MScheme }

sig DeclaresCatalogSort extends Spoken { forScheme: one MScheme, catalogSort: one MSort }

fact { all d: DeclaresYields | d.speaker = d.fromShape.ofScheme.schemeOwner }

fact { all d: DeclaresCatalogSort | d.speaker = d.forScheme.schemeOwner }

fun catalogSortOf[s: MScheme]: lone MSort { (DeclaresCatalogSort & InForce & forScheme.s).catalogSort }

fact { all s: MScheme | lone catalogSortOf[s] }

pred isTrue[d: DeclaresCatalogSort] {}

fun yieldsTo[s: MShape]: lone MScheme { (DeclaresYields & InForce & fromShape.s).intoScheme }

fact { all s: MShape | lone yieldsTo[s] }

fact { all s: MShape | some yieldsTo[s] implies no identifiedIn[s] and not isRoot[s] }

pred isTrue[d: DeclaresYields] {
   all k: keysOfShape[d.fromShape] | some k.yielded implies k.mRefersTo = k.yielded.mRefersTo
}

fun naturalKeyAnswer[x, y: MKey]: one Answer {
   (some x.scheme & y.scheme and some x.mParent & y.mParent) implies
      (sameAtOneLevel[x, y] implies SAME
       else twoTopsWay[x, y] implies DISJOINT
       else UNKNOWN)
   else UNKNOWN
}

check law_natural_same_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest implies
      all x, y: MKey | naturalKeyAnswer[x, y] = SAME implies x.mRefersTo = y.mRefersTo
} for 6 but 4 Int

run law_natural_same_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest
   some disj x, y: MKey | isNaturalKey[x] and naturalKeyAnswer[x, y] = SAME and some x.mRefersTo
}

check law_natural_disjoint_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest implies
      all x, y: MKey | naturalKeyAnswer[x, y] = DISJOINT implies no x.mRefersTo & y.mRefersTo
} for 6 but 4 Int

run law_natural_disjoint_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest
   some x, y: MKey | isNaturalKey[x] and naturalKeyAnswer[x, y] = DISJOINT and some x.mRefersTo and some y.mRefersTo
}
```

<!-- prose-translation -->
> MScheme S `:yields` MScheme T, per matched shape, where T is of any MSort.
> S's owner declares `:yields`.
> A shape yields into at most one T.
> Where S's own MKeys are looked up is S's MParent-Catalog, of one MSort, which S's owner declares.
> That declaration claims nothing about the world.
> A shape that yields carries no `:identified-in` and no `:root`.
> `:yields` is true when, for every MKey of the shape whose lookup emitted an MKey, the two MRefer to one MReferent, or both MRefer to none.
> S's lookup warrants (1.5-token-and-the-two-warrants) govern what equality and inequality of S's MKeys license before the primary MScheme is reached.
> Within one MParent-Catalog, two MKeys of S read SAME by the one-level rule (3.2-compare-one-chokepoint-four-answers).
> Within one MParent-Catalog, two MKeys of S read DISJOINT by the two-tops way (3.2-compare-one-chokepoint-four-answers).
> Within one MParent-Catalog, two MKeys of S read UNKNOWN otherwise.
> S's lookup warrants never license across MParent-Catalogs: two MKeys not in one MParent-Catalog read UNKNOWN.
> A SAME licensed before the primary MScheme is reached is never false while every statement in force is true and the engine's axioms hold.
> Each premise twin in this section asks for a world where every statement in force is true and the engine's axioms hold.
> The premise twin of `law_natural_same_is_sound` asks for a world where a natural MKey reads SAME with another MKey by the natural-key license and MRefers to an MReferent.
> A DISJOINT licensed before the primary MScheme is reached is never false while every statement in force is true and the engine's axioms hold.
> The premise twin of `law_natural_disjoint_is_sound` asks for a world where the natural-key license reads a natural MKey DISJOINT with an MKey, and each MRefers to an MReferent.

UNACKED READING, temporary (`312d:ask-catalog-sort-declaration`): 311 names three seats that
supply the MParent-Catalog INSTANCE and presupposes its MSort ("the MEntryChain's instance for
that MSort") without a declaration of it; `DeclaresCatalogSort` is the conductor's addition so
that the entry-chain seat can be read. Not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).

#### § 2.1.1-the-chain-and-the-decline

That the yield chain terminates at a primary MScheme is a property of the declarations, which
strangers write; the fences read a chain that reaches no primary MKey as an unknown identity
(3.1-identity-of-a-key). A decline is an evaluation that emits nothing.

<!-- normative -->
> T is a primary MScheme, or a secondary MScheme that in turn yields one.
> The chain always terminates at a primary MScheme.
> Nothing else constrains where an MScheme yields.
> An unknown input makes the instance unknown.
> A `resolve()` declines on MReferents its MSort does not describe.
> This decline is the mechanical net against lazy borrowing.

A cache and the file it caches are two MParent-Catalogs. A decline: a path that MRefers to a socket, under an MScheme into files. Two MSchemes over one spelling, into two MSorts: a path yields the directory entry, or the inode that the entry leads to. A tool that removes the entry binds under the first, and a tool that changes the file binds under the second.

### § 2.2-primary-of-and-identified-in

ER's identifying relationship. Identifying is not containing.

A second name is a second MScheme. P's MKeys mean something only relative to K's MParent-Store;
`:primary-of` itself is 1.3-scheme-a-way-of-writing's, P's `resolve()` being the identity is
3.1-identity-of-a-key's, "an MKey MRefers to one MReferent or to none" is 1.4-key-and-its-two-views's,
and the two warrants are 1.5-token-and-the-two-warrants's. A store that generates its MKeys at
creation holds an MReferent under a generated MKey while that MReferent exists; a store that
admits MKeys as names may hold none under an MKey a book names. The MParent's type varies per
shape, so the child MSort's owner never learns the MParent's types: the MParent MSort's primary
MScheme classifies, one level up, and each owner speaks one level. A MToken duplicable across
instances of its would-be MParent must be scoped in something smaller, or left un-warranted,
since `:root` is false of it. That a grade governs every consumer of the answer it grades,
corroboration and contradiction included, is 1.5-token-and-the-two-warrants's.

Arity: at most one MScheme per MSort. Declared by: the MSort's owner, on the primary MScheme.
Default: none; the floor of 1.3-scheme-a-way-of-writing supplies an unwarranted identity
primary MScheme. Consumer: identity (3.1-identity-of-a-key). Danger:
`:guarantees-unique-referent`, `:guarantees-unique-name`, and `:root`, per matched shape.

```alloy
sig DeclaresIdentifiedIn extends Spoken { onShape: one MShape, inSort: one MSort }

sig DeclaresRoot extends Spoken { rootedShape: one MShape }

fact { all d: DeclaresIdentifiedIn | d.speaker = d.onShape.ofScheme.schemeOwner }

fact { all d: DeclaresRoot | d.speaker = d.rootedShape.ofScheme.schemeOwner }

fact { all s: MScheme, k: MSort | primaryOf[s] = k implies s.schemeOwner = k.sortOwner }

fun identifiedIn[s: MShape]: lone MSort { (DeclaresIdentifiedIn & InForce & onShape.s).inSort }

fact { all s: MShape | lone identifiedIn[s] }

pred isRoot[s: MShape] { some DeclaresRoot & InForce & rootedShape.s }

fact { all s: MShape | isRoot[s] implies no identifiedIn[s] }

pred isTrue[d: DeclaresIdentifiedIn] {
   all k: keysOfShape[d.onShape] | some k.mRefersTo implies
      some r: keysOfSort[d.inSort].mRefersTo | k.mRefersTo in r.holds
}

pred isTrue[d: DeclaresRoot] {
   all a, b: keysOfShape[d.rootedShape] | a.mValue = b.mValue implies a.mRefersTo = b.mRefersTo
}
```

<!-- prose-translation -->
> P's owner declares, per matched shape of the MKey's MValue, `:identified-in` MSort M: the MParent's MSort for MKeys of that shape.
> A shape has at most one M.
> The primary MScheme's owner is the MSort's owner.
> `:identified-in` M is true when every MKey of the shape that MRefers to an MReferent MRefers to one held by an MReferent that some MKey of M MRefers to.
> P's owner declares `:root` per matched shape, absent by default: the shape declares no MParent, so it carries no `:identified-in`, and thereby claims global comparability.
> `:root` is true when it is `:guarantees-unique-referent` over the whole world: two MKeys of the shape with equal MValues MRefer to one MReferent, or both MRefer to none.
> For a `:root` shape, the world is the store: the shape's MWorld (1.8-fully-qualified-key-topic-and-derivation).

UNACKED READING, temporary (`312d:enc-primary-owner-is-sort-owner`): 311 has `:identified-in`
declared by "P's owner" (§ 1.6, § 2.2) and the section "declared by the MSort's owner, on the
primary MScheme"; the fence reconciles the two by a fact that a primary MScheme's owner is its
MSort's owner. The conductor's reading, not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).

`:root` fails for cloned identifiers. The MParent's type varies per shape: an ext4 filesystem in the MRoute, an NFS filesystem in a host, a tmpfs in a boot.

### § 2.3-aliases-nothing-else-the-store-warrant

The warrant is self-knowledge: whether another store aliases this one is not claimed, since
nobody can know it, and the world stratum of 1.1-referent-state-and-value keeps what a store
owns apart from what it merely holds so that a store re-presenting another's things is the one
whose claim is false. 3.2-compare-one-chokepoint-four-answers asks the warrant of every store on
both legs, so an aliasing store blocks separation by its own silence. Arity: per store.
Declared by: whoever describes the store. Default: absent; nothing inside that store then
separates from anything outside it. Consumer: DISJOINT. Danger: an aliasing store declared
`:aliases-nothing-else` is a wrong DISJOINT.

```alloy
sig DeclaresAliasesNothingElse extends Spoken { store: one MKey }

pred aliasesNothingElse[l: MLevel] { some DeclaresAliasesNothingElse & InForce & store.l }

pred isTrue[d: DeclaresAliasesNothingElse] {
   let s = d.store.mRefersTo |
      all r: s.holds | s in owns.r and owns.r in s
}
```

<!-- prose-translation -->
> A store is `:aliases-nothing-else` when nothing identified in it is, by the store's own construction, also identified in another store.
> Every MReferent the store holds, the store owns, and no other store owns.
> The store gives its own MKeys to no other store's MReferents.
> The warrant is declared per store, on the MKey that names it, and is absent by default.

Examples where it holds: a DNS zone for its records, a dpkg database for its packages, a network namespace for its `net/*` knobs, a disk filesystem for its inodes. Examples where it never holds: a client NFS mount, an NSS view of LDAP, a chroot, a nested pid namespace, an overlay, a front over another daemon, a relabelling of part of its MParent's state. State spanning several files is a may-read matter (2.5-may-read-the-readset). A hardlink is a `:guarantees-unique-name` failure on the path MScheme. The path MScheme yielding the inode dissolves it.

### § 2.4-parent-as-a-relation

Its MSort is the primary MScheme's declaration for the matched shape, and its instance is
whichever seat supplied it (1.6-parent-one-per-key). A MVantage supplies instances and is never
an MParent (1.10-vantage-route-placeholder-witness). Arity: one per MKey. Declared by: derived
from the MScheme's declarations and the supplying seat. Default: not applicable. Consumer:
routing, through a secondary MScheme; identity, through the primary MScheme. Danger: none of its
own.

```alloy
fun mParentStore[k: MKey]: lone MLevel { isPrimaryKey[k] implies k.mParent else none }

fun mParentCatalog[k: MKey]: lone MLevel { k.mParent }
```

<!-- prose-translation -->
> An MKey has at most one MParent.
> No statement species declares an MKey's MParent.
> Through the primary MScheme, an MKey's MParent is the MParent-Store, which carries identity.
> Through a secondary MScheme, an MKey's MParent is the MParent-Catalog, which carries routing and never identity.
> At the primary MScheme, MParent-Catalog and MParent-Store are one MKey.
> The far end is an ordinary MKey with an identity of its own, or an MWorld, never a string the engine composes.

### § 2.5-may-read-the-readset

Separation logic's footprint, at the read side: the MReferents an answer may depend on.

The relation is many-valued: an edge type from an MSort to MKeys of other MSorts, a template
the MSite or environment fills; the fences hold the filled form, one entry per MKey of the
MSort, spoken by the MSort's owner, with the sentinel per MSort. Every MSort in a
MFullyQualifiedKey may declare may-read entries, not only leaves, and an entry may name an MKey
of another MWorld, which `compare()` decides as 3.2-compare-one-chokepoint-four-answers decides
any pair. May-read is distinct from MParent: the MParent is one and answers identity; may-read
entries are many and answer interference, so two cells with different MParents can share a
may-read entry and so collide without being the same. An MKey's MParent instance is no may-read
entry and needs no declaration, since the walk collides a write at or above it; an entry naming a
store says more, that every write to an MKey relative to that store may change K, nobody having
said otherwise, which is 2.9-the-traversal-and-the-region-test's whole-mark. A closed may-read
set is knife-tier, one of the two closures every sparing rests on. The MRoute declares no set
(1.10-vantage-route-placeholder-witness). Arity: many per MSort, with a sentinel. Declared by:
the MSort owner. Default: ⊤. Consumer: the writeset of a line. Danger: none positive; omission
is the silent channel.

```alloy
sig DeclaresMayRead extends Spoken { ofKey: one MKey, readEntry: one MKey }

sig ClosesMayRead extends Spoken { readSort: one MSort }

fact { all d: DeclaresMayRead | d.speaker = sortOfKey[d.ofKey].sortOwner }

fact { all d: ClosesMayRead | d.speaker = d.readSort.sortOwner }

fun mayReadEntries[k: MKey]: set MKey { (DeclaresMayRead & InForce & ofKey.k).readEntry }

fun mayReadEdge: MKey -> MKey { {k, e: MKey | e in compositeMayRead[k]} }

pred mayReadClosed[s: MSort] { some ClosesMayRead & InForce & readSort.s }

pred sortClosed[k: MKey] { mayReadClosed[sortOfKey[k]] }

pred readsetMemberIsTop[k: MKey] {
   some m: k.*mayReadEdge |
      (not sortClosed[m]
       or (some l: identity[m].*mParent & MKey | not sortClosed[l])
       or terminus[identity[m]] in MRoute)
}

fun readset[f: VerdictFact]: set MKey { f.markedReads }

pred readsetIsTop[f: VerdictFact] { no f.markedReads or some k: f.markedReads | readsetMemberIsTop[k] }

pred isTrue[d: DeclaresMayRead] {}

pred isTrue[d: ClosesMayRead] {
   all k: keysOfSort[d.readSort] | some k.mRefersTo implies
      affects.(k.mRefersTo) in k.mRefersTo.*holds + (^holds).(k.mRefersTo) + compositeMayRead[k].mRefersTo
}

pred isTrue[f: VerdictFact] {
   some f.markedReads implies f.dependsOn in f.markedReads.mRefersTo
}
```

<!-- prose-translation -->
> K `:may-read` these MKeys: writes to them affect K's MState.
> K's owner declares an entry per MKey of K.
> K's owner closes the set per MSort with a completion sentinel.
> An entry licenses nothing positive.
> A closed may-read set is true when every MKey of K that MRefers to an MReferent meets one condition.
> The condition is that every MReferent whose write affects the MKey's MReferent is of one of four kinds.
> The four kinds are the MKey's MReferent itself, something it holds, something that holds it, and an MReferent that one of the MKey's entries MRefers to.
> A fact's readset is the marked reads of the body that answered it.
> For a verdict fact, the vouch closes the marked reads (`KNOBS:kCONTRACT-RUNGS`).
> The vouch is true when the measured answer depended on no MReferent outside what the marked reads MRefer to.
> A body that marks no read has readset ⊤.
> May-read entries are not in a readset.
> A write reaches K through the may-read entries (2.6-may-write-the-writeset, rule 4).
> A readset member is ⊤ where its MSort has no closed may-read set.
> A readset member is ⊤ where an MSort on its MFullyQualifiedKey has no closed may-read set.
> A readset member is ⊤ where a may-read entry through which rule 4 reaches it fails either test, transitively.
> A readset member is ⊤ where its MFullyQualifiedKey, or that of a may-read entry through which rule 4 reaches it, ends at the MRoute.
> A readset is ⊤ where the body marked no read or where any member is ⊤.

UNACKED READING, temporary (`312d:ask-may-read-is-declared-per-key`): 311's may-read is "a
template the MSite or environment fills"; the fence holds only the filled form, one entry per
MKey, and no template. The `isTrue` predicates of the closure and of the vouch transcribe into the
world relations of 1.1-referent-state-and-value, which are themselves an unacked reading. The
conductor's reading, not acked, not authoritative, held only until acked or replaced
(`notes/312d` § 7).

A may-read entry above the leaf: a loop-backed filesystem's state lives in a file of the outer filesystem, and `dd` over the image rewrites every inner fact. A shared entry: two observers' writability cells share the file's mode.

### § 2.6-may-write-the-writeset

A refuted shape: two entries overlap only when they are one place (`311u:refuted-only-same-entries-overlap`). The model collides whatever is not DISJOINT.

The at-most set is the footprint of `plans/30U`, declared by the verb's author per matched
shape and closed by the completion record; where the body emits at runtime, the verb author's
completion record closes them. The entailment is arm-incremental and collide-adding, declared
by K's owner, about effects and not identity; it carries every cross-MSort consequence no
MFullyQualifiedKey expresses, and the reached finished record witnesses that the write set,
after entailment, is complete. The fences hold the filled form of the entailment, an entry per
written MKey, and the record per shape. The sentence excluding containers at or above the level
the written MKey shares with the read MKey admits two readings (`notes/312ch`, item 11): the
exclusion applied as the writeset is built, or only at the test; both are mechanized, the law is
stated over the first, which spares the more, and a check asks whether they ever disagree. Rule
3 reads what an entry given whole covers from 2.9-the-traversal-and-the-region-test and
2.10-places-the-upward-lookup. Arity: per matched shape of
the verb, for the at-most set; many per matched shape on the MSort, for the entailment; plus
the finished record. Declared by: the verb's author, for the at-most set; the MSort owner, for
the entailment. Default: unfinished, which collides. Consumer: the sparing test, within one
MWorld; never a generator of DISJOINT. Danger: the premature finished record.

```alloy
one sig World { lineWrites: Line -> MReferent }

sig DeclaresMayWrite extends Spoken { writeLine: one Line, writeEntry: one MKey }

sig ClosesMayWrite extends Spoken { closedLine: one Line }

sig DeclaresEntails extends Spoken { fromKey: one MKey, entailedEntry: one MKey }

sig FinishesEntailment extends Spoken { finishedSort: one MSort, finishedShape: lone MShape }

fact { all d: DeclaresEntails | d.speaker = sortOfKey[d.fromKey].sortOwner }

fact { all d: FinishesEntailment | d.speaker = d.finishedSort.sortOwner }

fun atMostEntries[l: Line]: set MKey { (DeclaresMayWrite & InForce & writeLine.l).writeEntry }

pred atMostClosed[l: Line] { some ClosesMayWrite & InForce & closedLine.l }

fun entailed[k: MKey]: set MKey { (DeclaresEntails & InForce & fromKey.k).entailedEntry }

pred entailmentFinished[k: MKey] {
   some d: FinishesEntailment & InForce | d.finishedSort = sortOfKey[k] and d.finishedShape = k.shape
}

fun wholeWriteEntries[l: Line]: set MKey { (DeclaresMayWrite & GivenWhole & InForce & writeLine.l).writeEntry }

fun wholeReadEntries: set MKey { (DeclaresMayRead & GivenWhole & InForce).readEntry }

fun entryAnswer[m, e: MKey]: one Answer { e in wholeReadEntries implies regionTest[e, m] else tabledCompare[m, e] }

fun rule4[m: MKey]: set MKey { {k: MKey | some e: compositeMayRead[k] | entryAnswer[m, e] != DISJOINT} }

fun seed[l: Line]: set MKey { atMostEntries[l] + {k: MKey | some P: wholeWriteEntries[l] | k in beneathFor[P]} }

fun levelKeysOf[m: MKey]: set MKey { m + (identity[m].*mParent & MKey) }

fun contributingContainers[m, r: MKey]: set MKey {
   let excluded = (meet[identity[m], identity[r]].MLevel).*mParent |
      levelKeysOf[m] - excluded - (some identity[m] & excluded implies m else none)
}

fun spreadsTo[m, r: MKey]: set MKey { entailed[contributingContainers[m, r]] + rule4[m] }

fun writesetAgainst[l: Line, r: MKey]: set MKey {
   seed[l].*({m, k: MKey | k in spreadsTo[m, r]})
}

fun writesetUnexcluded[l: Line]: set MKey {
   seed[l].*({m, k: MKey | k in entailed[levelKeysOf[m]] + rule4[m]})
}

fun writesetAtTest[l: Line, r: MKey]: set MKey {
   seed[l] + {k: MKey | some m: writesetUnexcluded[l] | k in spreadsTo[m, r]}
}

pred writesetIsTop[l: Line, ws: set MKey] {
   not atMostClosed[l] or some m: ws | not entailmentFinished[m]
}

fun memberAnswer[l: Line, w, r: MKey]: one Answer {
   w in wholeWriteEntries[l] implies regionTest[w, r] else tabledCompare[w, r]
}

pred sparedBy[l: Line, f: VerdictFact, ws: MKey -> MKey] {
   flagged
   l in f.atLine.above
   not readsetIsTop[f]
   no r: readset[f] | staleAt[f.atLine, r]
   all r: readset[f] {
      not writesetIsTop[l, ws[r]]
      all w: ws[r] | memberAnswer[l, w, r] = DISJOINT
   }
}

fun writesetsAgainst[l: Line]: MKey -> MKey { {r, w: MKey | w in writesetAgainst[l, r]} }

fun writesetsAtTest[l: Line]: MKey -> MKey { {r, w: MKey | w in writesetAtTest[l, r]} }

pred spared[l: Line, f: VerdictFact] { sparedBy[l, f, writesetsAgainst[l]] }

pred sparedAtTest[l: Line, f: VerdictFact] { sparedBy[l, f, writesetsAtTest[l]] }

pred isTrue[d: DeclaresMayWrite] {}

pred isTrue[d: ClosesMayWrite] {
   let l = d.closedLine |
      World.lineWrites[l] in atMostEntries[l].mRefersTo + wholeWriteEntries[l].mRefersTo.passes
}

pred isTrue[d: DeclaresEntails] {}

pred isTrue[d: FinishesEntailment] {
   all k: MKey | sortOfKey[k] = d.finishedSort and k.shape = d.finishedShape and some k.mRefersTo implies
      (k.mRefersTo).affects in k.mRefersTo.*holds + entailed[k].mRefersTo
}

pred sparingIsSound {
   all l: Line, f: VerdictFact & InForce | spared[l, f] implies
      no (World.lineWrites[l]).*affects & f.dependsOn
}

pred hole_two_separated_things_reach_one_thing_beneath {
   some disj x, y: MKey | tabledCompare[x, y] = DISJOINT
      and some x.mRefersTo.*(holds + passes) & y.mRefersTo.*(holds + passes)
}

pred hole_a_write_affects_through_a_third_thing {
   some disj a, b, c: MReferent | b in a.affects and c in b.affects and c not in a.affects
}

pred outsideTheSparingHoles {
   not hole_world_scoped_top_aliases_into_a_store
   not hole_region_closure_with_unknown_leaf_pair
   not hole_composite_keys_with_same_parts_refer_differently
   not hole_two_separated_things_reach_one_thing_beneath
   not hole_a_write_affects_through_a_third_thing
}

check law_sparing_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
      and outsideTheSparingHoles implies
      sparingIsSound
} for 4 but 4 Int, 10 Claim

run law_sparing_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
   outsideTheSparingHoles
   some l: Line, f: VerdictFact & InForce |
      spared[l, f] and some World.lineWrites[l] and some f.dependsOn and some atMostEntries[l]
}

check law_sparing_is_sound_with_a_store_on_the_chain {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
      and outsideTheSparingHoles implies
      sparingIsSound
} for 4 but 4 Int, 14 Claim, 5 MLevel

run law_sparing_is_sound_with_a_store_on_the_chain_premise {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
   outsideTheSparingHoles
   some l: Line, f: VerdictFact & InForce, r: readset[f] |
      spared[l, f] and some World.lineWrites[l] and some f.dependsOn and some atMostEntries[l]
         and some identity[r].^mParent & MKey
}

check law_exclusion_readings_agree {
   all l: Line, f: VerdictFact & InForce | spared[l, f] iff sparedAtTest[l, f]
} for 4 but 4 Int, 10 Claim

run law_exclusion_readings_agree_premise {
   some l: Line, f: VerdictFact & InForce, m: atMostEntries[l], r: readset[f] |
      some contributingContainers[m, r] and some entailed[levelKeysOf[m]]
}
```

<!-- prose-translation -->
> A line's writeset against a read MKey is the set of MKeys the line may write or may change.
> The writeset is the least set that four rules close.
> Rule 1: every may-write entry declared for the line is in the writeset.
> The completion record closes those entries.
> Rule 2 applies where an MKey of K is in the writeset, or an MKey identified beneath an MKey of K.
> Under rule 2, every MKey that K's may-write entailment names is in the writeset.
> Under rule 2, a container at or above the deepest level that the written MKey shares with the read MKey contributes no entailment.
> The entailment read for a written MKey is the one declared on that MKey and on each container on its identity's chain.
> The written MKey is excluded with its identity.
> Rule 3: where an MKey given whole is in the writeset, every MKey reached beneath it is in the writeset (2.9-the-traversal-and-the-region-test).
> Where the MKey given whole has a finished enumeration, the MKeys reached beneath it are that enumeration's members (2.10-places-the-upward-lookup).
> Where the MKey given whole has no finished enumeration, the MKeys reached beneath it are its enumeration's members and every MKey its region covers besides.
> P itself stays an entry of that test.
> Rule 4 applies where an MKey in the writeset `compare()`s other than DISJOINT with a may-read entry declared for an MKey k.
> Under rule 4, k is in the writeset (2.5-may-read-the-readset, 3.2-compare-one-chokepoint-four-answers).
> Rule 4 compares a may-read entry given whole by the region test.
> May-read entries feed rule 4 and no other rule.
> Under the second reading, the exclusion applies only at the test.
> Under the second reading, the writeset is built with every container contributing.
> Under the second reading, an MKey that only the last step excludes is dropped there.
> An unclosed at-most set puts ⊤ in the writeset, and so does a member whose MSort and shape have no reached finished record.
> ⊤ is DISJOINT from nothing.
> An elision is spared past a line only under `--risk-faultless-skips` (3.2-compare-one-chokepoint-four-answers).
> An elision is spared past a line only when the line is above the fact's site.
> An elision is spared past a line only when neither the readset nor the writeset against any readset member is ⊤.
> An elision is spared past a line only when no readset member is stale at the site (3.3-invalidation-three-mutator-species).
> An elision is spared past a line only when `compare()` answers DISJOINT for every pair of a writeset member and a readset member.
> The pairs include pairs of one MSort and pairs of two MSorts.
> Where the writeset member is an entry given whole, the region test gives that answer in place of `compare()` (2.9-the-traversal-and-the-region-test).
> A may-write entry and an entailment entry license nothing alone.
> K's owner declares the entailment.
> The written MSort's owner declares the finished record.
> A completion record is true when every MReferent the line writes is of one of two kinds.
> The first kind is an MReferent that an at-most entry MRefers to.
> The second kind is an MReferent that a route through a whole-marked entry's MReferent passes to.
> A finished record is true when writing each covered MReferent affects only it, what it holds, and the MReferents the entailment names.
> The covered MReferents are those an MKey MRefers to whose MSort and shape are the record's.
> A held hole: two MKeys that `compare()` reads DISJOINT MRefer to two MReferents from which one MReferent is reached beneath both, by holding or by route, directly or through others.
> A held hole: a write to one MReferent affects a third MReferent through a second, and does not affect the third directly.
> The sparing holes are five: the world-scoped-top hole, the region hole, the composite hole, the one-thing-beneath hole, and the third-thing hole (3.2-compare-one-chokepoint-four-answers, 2.9-the-traversal-and-the-region-test).
> A world is outside the sparing holes when it is outside each of the five.
> A sparing is never false while every statement in force is true and the engine's axioms hold and no store is among its own contents.
> Under those premises, for every sparing, no MReferent the line writes affects, directly or through others, an MReferent the fact's answer depended on.
> The sparing law is asked outside the sparing holes.
> Two commands ask the same law.
> The first command asks it over worlds of four levels.
> The second command asks it over worlds of five levels and fourteen statements, where a spared fact's read MKey can have a store on its chain.
> The premise twins of the two sparing commands also ask for the sparing law's premises and ask outside the sparing holes.
> The premise twin of `law_sparing_is_sound` asks for a world where a fact that depended on something is spared past a writing line with an at-most entry.
> The premise twin of `law_sparing_is_sound_with_a_store_on_the_chain` asks for a world where the first twin's spared fact has a readset member with a store on its chain.
> Whether the two readings of the exclusion ever disagree on a sparing is asked, and either answer is a finding.
> The premise twin of `law_exclusion_readings_agree` asks for a world where an at-most entry has an entailing level and a contributing container against a read MKey.

Scope: the sparing law runs at four atoms of the model's kinds and ten statements because its writeset closes a comprehension over every pair of MKeys and does not finish translating at six, and because its twin's witness (the flag, a verdict fact, a closed may-read set on every level of a read key's chain, a closed at-most set, a finished record for every writeset member, and the separation they rest on) is unsat at six statements and seats at ten. Four levels seat an MRoute, a MRoot MWorld, and two MKeys, so no spared world at that scope holds a store on a read MKey's chain; the second command asks the same law at five levels, where its twin demands such a store, and its result is the measurement of whether that claim is affordable. That twin is unsat at ten statements: a store on the chain adds its own `:primary-of`, `:root`, `:identified-in`, a second supplied MParent, and a second closed may-read set, thirteen in force by hand count, so the second command runs at fourteen, the least count that seats its witness plus one.

Scope: the exclusion-readings law runs at the sparing law's four atoms and ten statements for the same reason, since both of its sides are the sparing test over a writeset, and it does not finish translating at six.

UNACKED READING, temporary (`312d:ask-both-exclusion-readings-are-mechanized`): 311's exclusion
sentence admits two readings (`notes/312ch` item 11), and the fence mechanizes both rather than
choosing, but the sparing LAW is stated over one of them, the one that spares more, and "spared
only when" is read as the engine's decision rather than a necessary condition. Those are the
conductor's readings, not acked, not authoritative, held only until acked or replaced
(`notes/312d` § 7).

#### § 2.6.1-the-finished-definition-and-the-worlds

<!-- normative -->
> The test spares narrowly and collides widely: whatever is not DISJOINT collides.
> A may-write entry is declared by the verb's author, or the filesystem binder, per matched shape.
> The entailment generates no DISJOINT: "nothing else" is no other thing, never no other MKey for the thing written.
> The finished definition is a within-MWorld sentence.
> The finished definition never speaks across MRoutes or MRoots (3.2-compare-one-chokepoint-four-answers).
> A may-read entry that names an MKey of another MWorld enters rule 4 where a write in that MWorld reaches it (2.5-may-read-the-readset).

Without the exclusion of containers at or above the shared level, a filesystem's entailment, which names its disk, would make two files in one filesystem collide through it. Examples of the entailment: a package's postinst enabling its unit, a restart killing a main process.

### § 2.7-corresponds-across-a-transition

A scoped `sameAs`. Not "corresponds to" loosely: a part, a view, or a correlate of a thing is not it.

Absent a MCorrespondence, MKeys across a transition `compare()` UNKNOWN unless a
MFullyQualifiedKey binds MTokens on both sides, which is the walk's step 1; the MCorrespondence
is the model's only declared sameness generator besides MToken equality, and it is consumed as
one SAME MDerivation by `compare()` (3.2-compare-one-chokepoint-four-answers), vouch-tier,
attributed to the transition author. Arity: per transition pair. Declared by: the transition
owner. Default: absent, so UNKNOWN. Danger: a wrong MCorrespondence is a wrong SAME.

```alloy
sig DeclaresCorresponds extends Spoken { keyX: one MKey, keyY: one MKey }

fact { all d: DeclaresCorresponds | d.speaker not in d.keyX.scheme.schemeOwner + d.keyY.scheme.schemeOwner }

pred corresponds[x, y: MKey] {
   some d: DeclaresCorresponds & InForce | (d.keyX = x and d.keyY = y) or (d.keyX = y and d.keyY = x)
}

pred isTrue[d: DeclaresCorresponds] { d.keyX.mRefersTo = d.keyY.mRefersTo }
```

<!-- prose-translation -->
> MKey X inside MParent A `:corresponds` to MKey Y inside MParent B: they denote the same MReferent.
> The owner of the transition between A and B declares it, and that owner is neither MKey's MScheme owner.
> A MCorrespondence is true when the two MKeys MRefer to one MReferent, or both MRefer to none.

Examples: the container manager knows guest pid 1 is host pid 4821. `sudo -u alice` knows inner "me" is outer "alice". A mount line's oracle knows MKeys under the mountpoint are MKeys under the export on the named server, from this MVantage.

### § 2.8-observer-dependence-and-independence

No `resolve()` can measure observer-dependence: the object is the same and the answer differs,
so it must remain speech, and measurement in the denoted context (`plans/27C`) stays the default
lane. The observers a read was taken under are the lent instances of its MEntryChain
(3.4-entry-and-lends), held on the verdict fact. Arity: per (MSort, O). Declared by: the MSort
owner. Default: dependent, so no carry across O-instances. Consumer: the SAME consumer, as a
qualifier on the claim's MTopic. Danger: a false independence.

```alloy
sig DeclaresObserverIndependence extends Spoken { independentSort: one MSort, ofObserver: one MSort }

fact { all d: DeclaresObserverIndependence | d.speaker = d.independentSort.sortOwner }

pred observerIndependent[k: MSort, o: MSort] {
   some DeclaresObserverIndependence & InForce & independentSort.k & ofObserver.o
}

fun topicObservers[f: VerdictFact]: set MKey {
   {o: f.underObservers | not observerIndependent[sortOfKey[f.topic], sortOfKey[o]]}
}

pred sameTopic[f, g: VerdictFact] {
   tabledCompare[f.topic, g.topic] = SAME
   all o: topicObservers[f] | some p: topicObservers[g] | tabledCompare[o, p] = SAME
   all p: topicObservers[g] | some o: topicObservers[f] | tabledCompare[o, p] = SAME
}

pred isTrue[d: DeclaresObserverIndependence] {
   all f: VerdictFact | sortOfKey[f.topic] = d.independentSort implies
      no f.dependsOn & {o: f.underObservers | sortOfKey[o] = d.ofObserver}.mRefersTo
}
```

<!-- prose-translation -->
> The MValues that reads of K's cells yield depend on which MKey of MSort O the read was taken under.
> K's owner declares the complement, `:observer-independence` of O, per MSort.
> By default, a cell measured under a lent MKey of O is assumed to depend on it.
> Its fact is then about (MReferent, O-instance).
> That fact stands for another fact only when `compare()` answers SAME for the two MKeys.
> That fact stands for another fact only when every observer instance of either fact has an observer instance of the other that `compare()` answers SAME for.
> `:observer-independence` of O is true when no answer about a K-cell depended on the O-instance it was taken under.

UNACKED READING, temporary (`312d:enc-observers-are-the-vantage-ambients`): 311 says a cell is
"measured under a lent MKey of O"; the fence takes every ambient instance of the topic's
MVantage as an observer and the answer's dependence on the observer's MReferent as the truth
of independence. The conductor's readings, not acked, not authoritative, held only until acked
or replaced (`notes/312d` § 7).

### § 2.9-the-traversal-and-the-region-test

A lookup emits its MTraversal (1.7-resolution-and-its-traversal); it may cross several levels,
each looked up in a catalog that the previous level named. The engine never reads an MKey's
syntax: whatever splitting an MKey needs happens inside a lookup's body. A lookup that emits no
member has its MParent-Catalog, given whole, as its MTraversal, and any touch on that catalog
then invalidates every MResolution through it: the coarse, safe floor. The walk of
3.2-compare-one-chokepoint-four-answers collides a write to an entry's MReferent with
everything identified in it; what an entry given whole names beyond that is the region test.
Only x's MTraversals are walked, and D needs no closure of its own; every level is asked, never
only the leaf, because an alias may sit at any level and a leaf's own closure cannot see it.
`compare()` compares a level by the identity of the MReferent that the level resolved to
(`walkOfKeys`, § 3.2). Unequal MTokens say only that the thing is not the routing MKey itself.
Arity: per lookup, per matched shape. Declared by: the lookup's owner, through what the lookup
emits. Default: no emission, so the MTraversal is the MParent-Catalog given whole. Consumer:
MResolution backings, hence invalidation (3.3-invalidation-three-mutator-species), and the
region test. Danger: a false closing act keeps a stale MResolution and every conclusion built on
it; a false `alias nothing-else` is a wrong DISJOINT; an open or coarse emission is safe.

```alloy
sig GivenWhole in DeclaresMayWrite + DeclaresMayRead {}

sig EmitsAliasNothingElse extends Spoken { atLevel: one MKey }

fact { all d: EmitsAliasNothingElse | d.speaker = d.atLevel.scheme.schemeOwner }

pred aliasClosed[m: MKey] { some EmitsAliasNothingElse & InForce & atLevel.m }

pred isTrue[d: EmitsAliasNothingElse] {
   no c: MKey - d.atLevel | c.scheme = d.atLevel.scheme and some c.mRefersTo and c.mRefersTo = d.atLevel.mRefersTo
}

fun sortOfKey[k: MKey]: lone MSort { primaryOf[identity[k].scheme] + identity[k].cellSort }

fun traversalMembers[k: MKey]: set MKey {
   crossed[k] + (traversalClosed[k] implies none else k.mParent & MKey)
}

fun levelsOf[x: MKey]: set MKey { x.*yielded + (identity[x].^mParent & MKey) }

pred coveredBy[D, x: MKey] {
   (some l: levelsOf[x], m: traversalMembers[l] | tabledCompare[m, D] = SAME)
   or (some m: placedIn[x, sortOfKey[D]] | tabledCompare[m, D] = SAME)
}

pred lookupTraversalOfSort[l: MKey, G: MSort] {
   some traversalMembers[l] and all m: traversalMembers[l] | some sortOfKey[m] & G
}

pred outsideByTraversals[D, x: MKey] {
   let G = sortOfKey[D] {
      (some l: levelsOf[x] | lookupTraversalOfSort[l, G]) or some placedIn[x, G]
      all l: levelsOf[x] | lookupTraversalOfSort[l, G] implies
         traversalClosed[l] and all m: traversalMembers[l] | tabledCompare[m, D] = DISJOINT and aliasClosed[m]
      some placedIn[x, G] implies
         lookedUpInClosed[x, G] and all g: placedIn[x, G] | tabledCompare[g, D] = DISJOINT
   }
}

pred outsideByPlacing[D, x: MKey] {
   let G = sortOfKey[D] | lookedUpInClosed[x, G] and no placedIn[x, G]
}

fun regionTest[D, x: MKey]: one Answer {
   tabledCompare[x, D] = SAME implies SAME
   else coveredBy[D, x] implies UNKNOWN
   else (outsideByTraversals[D, x] or outsideByPlacing[D, x]) implies DISJOINT
   else UNKNOWN
}

fun beneath[D: MKey]: set MKey { {x: MKey | coveredBy[D, x]} }

pred hole_region_closure_with_unknown_leaf_pair {
   some D, x: MKey | tabledCompare[x, D] in UNKNOWN + KNOWN_UNSPOKEN and regionTest[D, x] = DISJOINT
}

check law_region_disjoint_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
      and not hole_region_closure_with_unknown_leaf_pair
      and not hole_composite_keys_with_same_parts_refer_differently
      and not hole_world_scoped_top_aliases_into_a_store implies
      all D, x: MKey | regionTest[D, x] = DISJOINT implies
         no x.mRefersTo & (D.mRefersTo + D.mRefersTo.passes)
} for 6 but 4 Int

run law_region_disjoint_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
   not hole_region_closure_with_unknown_leaf_pair
   not hole_composite_keys_with_same_parts_refer_differently
   not hole_world_scoped_top_aliases_into_a_store
   some D, x: MKey | regionTest[D, x] = DISJOINT and some x.mRefersTo and some D.mRefersTo.passes
      and (some l: levelsOf[x] | some traversalMembers[l])
}
```

<!-- prose-translation -->
> An entry's author marks it given whole.
> A lookup may emit a closure, `alias nothing-else`, for a level it resolved (1.5-token-and-the-two-warrants).
> That closure is true when no other MKey of that level's MScheme MRefers to the level's MReferent.
> A lookup's MTraversal members are the routing MKeys it emitted, and its MParent-Catalog where the emission is not closed.
> The MTraversals of x are those `identity(x)` produced, at every lookup toward the primary MKey and at every level of x's MFullyQualifiedKey (1.7-resolution-and-its-traversal, 3.1-identity-of-a-key).
> The routes of 2.10-places-the-upward-lookup are also MTraversals of x.
> A MTraversal is of D's MSort when it has a member and every member of it is of that MSort.
> A MTraversal is also of D's MSort when it is a placing route of that MSort.
> For an MKey D given whole against an MKey x, step 1: if x's leaf compares SAME with D, SAME.
> Step 2: else if any level of a MTraversal compares SAME with D, D's region covers x, and the pair reads UNKNOWN.
> Where a placing route places x in an MKey that compares SAME with D, D's region covers x, whatever D's enumeration holds.
> Step 3: else if x carries a closure for D's MSort, DISJOINT, in one of two forms.
> First form: x has at least one MTraversal of D's MSort.
> In the first form, every level on every such MTraversal compares DISJOINT with D.
> In the first form, every level on every such MTraversal emitted its closure.
> That closure is `alias nothing-else` for a lookup's level, or `looked-up-in nothing-else` for a placing route (2.10-places-the-upward-lookup).
> In the first form, the lookup's emission is closed.
> Second form: the placing lookup of D's MSort emitted `looked-up-in nothing-else` for x with no `looked-up-in` record.
> In the second form, x is in no region of that MSort.
> Step 4: otherwise UNKNOWN.
> An entry given whole names more than the MReferent of its MKey.
> The entry given whole also names every MReferent reached beneath that MKey, through the MScheme's lookups or through a placing route.
> An entry given whole names every MKey the region covers.
> A held hole: some MKey D and some MKey x whose `compare()` reads UNKNOWN or KNOWN_UNSPOKEN, and whose region test reads DISJOINT.
> The region law has three premises: every statement in force is true, the engine's axioms hold, and no store is among its own contents.
> Under those premises, a DISJOINT of the region test is never false.
> Such a DISJOINT for D and x is false when x MRefers to D's MReferent, or to an MReferent that a route through D's MReferent passes to.
> The region law is asked outside that hole, outside the composite hole, and outside the world-scoped-top hole (3.2-compare-one-chokepoint-four-answers).
> The premise twin of `law_region_disjoint_is_sound` asks for a world where the region test reads DISJOINT for D and x.
> In that world, x MRefers to an MReferent, D's MReferent passes to an MReferent, and a level of x has MTraversal members.
> The twin also asks for the region law's premises, outside the three holes.

UNACKED READING, temporary (`312d:ask-alias-closure-instance-scope`): 311 scopes `alias
nothing-else` to "the instance the lookup ran in"; the fence's truth ranges over every MKey of
the level's MScheme in the world, which is STRONGER than the sentence and so the direction that
can hide a counterexample. "Of D's MSort" for a MTraversal is read as every member being of
that MSort. The conductor's readings, not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).

#### § 2.9.1-indexicals-and-the-floor

<!-- normative -->
> The MTraversals of x are walked leaf first.
> A lookup may cross an indexical routing MKey, whose MResolution depends on the observing process.
> Only the lookup's owner can say which, in the body that meets it.
> An undeclared indexical routing MKey reads unknown.
> Containment is membership in a MTraversal.
> In the test of 2.6-may-write-the-writeset, a routing MKey named whole reads UNKNOWN against every MKey that its MScheme can yield in the same MParent-Catalog instance.
> That answer holds whatever 3.2-compare-one-chokepoint-four-answers answers of the two as siblings.
> That UNKNOWN is the floor, which the region test refines.

Lookups that cross several levels: a path yields a directory entry looked up in a shorter path. A hostname yields a resolver step from a MVantage. A dotted unit name yields an entry in its instance table. Lookups that cross one level: an inode number, a uid. An indexical routing MKey: `/proc/self`. A path prefix is not a store. A mutator that touches a directory needs to know nothing about files. An alias above the leaf: a bind mount of a directory above a file, or an alias entry above a leaf. A positional catalog: the resolution of position N depends on every position at or before it, and `ufw insert 1` gives every later rule a new number. The lookup emits positions 1 to N as its MTraversal, one line per member. A rowid table renumbered after ten thousand rows makes that emission large. A single emission that describes the set, spelled as sh, is work for the 312 series.

### § 2.10-places-the-upward-lookup

G's owner publishes a lookup that is invoked with the MValue of an MKey of T, its matched
shapes deciding which spellings of T it answers; membership is a relation between two
MReferents, never a spelling of one, so the placing lookup is not an MScheme of T. The route
so recorded is a MTraversal of the MKey for the region test
(2.9-the-traversal-and-the-region-test), invalidated as any MResolution is
(3.3-invalidation-three-mutator-species), and never the MKey's MParent, which is the route the
MKey's own lookup supplied (1.6-parent-one-per-key). When the engine invokes the lookup, and
what it refuses when two invocations disagree, are 2.10.1-invocation-and-refusal. Arity: per
(G, T). Declared by: G's owner. Default: absent; T's MKeys then have no route of MSort G, and
the pair reads as 3.2-compare-one-chokepoint-four-answers decides it. Consumer: the region
test, and invalidation. Danger: a false `looked-up-in nothing-else` is G's owner's wrong
DISJOINT.

```alloy
sig DeclaresPlaces extends Spoken { placingSort: one MSort, placedSort: one MSort }

sig RecordsLookedUpIn extends Spoken { placedKey: one MKey, inKey: one MKey }

sig ClosesLookedUpIn extends Spoken { closedKey: one MKey, routeSort: one MSort }

fact { all d: DeclaresPlaces | d.speaker = d.placingSort.sortOwner }

fact { all d: RecordsLookedUpIn | d.speaker = sortOfKey[d.inKey].sortOwner }

fact { all d: ClosesLookedUpIn | d.speaker = d.routeSort.sortOwner }

pred places[G, T: MSort] { some DeclaresPlaces & InForce & placingSort.G & placedSort.T }

fun placedIn[x: MKey, G: MSort]: set MKey {
   {g: (RecordsLookedUpIn & InForce & placedKey.x).inKey | some sortOfKey[g] & G}
}

pred lookedUpInClosed[x: MKey, G: MSort] { some ClosesLookedUpIn & InForce & closedKey.x & routeSort.G }

fun beneathFor[P: MKey]: set MKey { entailmentFinished[P] implies entailed[P] else entailed[P] + beneath[P] }

pred isTrue[d: DeclaresPlaces] {}

pred isTrue[d: RecordsLookedUpIn] {
   d.placedKey.mRefersTo in d.inKey.mRefersTo.passes
}

pred isTrue[d: ClosesLookedUpIn] {
   all g: keysOfSort[d.routeSort] | some d.closedKey.mRefersTo and d.closedKey.mRefersTo in g.mRefersTo.passes implies
      some r: placedIn[d.closedKey, d.routeSort] | r.mRefersTo = g.mRefersTo
}
```

<!-- prose-translation -->
> An MSort G may declare that it `:places` another MSort T.
> G's owner declares `:places`.
> For an MKey, the placing lookup emits `looked-up-in G:key`, a record G's owner speaks.
> The record is true when the MKey's MReferent is one that a route through G:key's MReferent passes to.
> The placing lookup emits a closure `looked-up-in nothing-else`, scoped to routes of MSort G.
> The closure is true when every G-thing a route to the MKey's MReferent passes through is one a record names.
> The `looked-up-in` records of every invocation accumulate: x's routes of MSort G are every G:key recorded for x.
> The store's end of the same relation is G's enumeration of its members.
> The may-write entailment of 2.6-may-write-the-writeset carries that enumeration as write reach.
> In the test of 2.6-may-write-the-writeset, when that entailment is finished for P, its emitted members stand in for the MKeys reached beneath P (rule 3).
> An unfinished entailment widens the writeset only.

#### § 2.10.1-invocation-and-refusal

Invocations are not in the fences; a record either is in force or is not.

<!-- normative -->
> The engine invokes G's lookup only when three conditions hold.
> First, a writeset or readset entry names an MKey of G given whole.
> Second, the MKey of T on the other side of that pair, a writeset entry or a readset entry, has no route of MSort G.
> Third, G declares that it places T.
> The engine invokes the lookup with every MKey it holds for that MReferent.
> A closure `looked-up-in nothing-else` from one invocation can contradict a record from another invocation.
> The engine then refuses both answers and attributes the refusal to G's owner.

Matched shapes of a placing lookup: a path-shaped MValue answered, an inode number declined.

### § 2.11-composite-sorts-and-roles

The author who knows the roles mints the MCompositeSort, normally the tool author. Plurality of
inputs is an MSort with structure, never a set of MParents: a composite MKey has one MParent as
any MKey does, and its parts beside it. Arity: per composite. Declared by: the author holding
the roles. Default: not applicable. Consumer: identity (3.1-identity-of-a-key), through
`compare()`. Danger: as any MSort.

```alloy
sig Role {}

sig DeclaresComposite extends Spoken { compositeSort: one MSort }

fact { all d: DeclaresComposite | d.speaker = d.compositeSort.sortOwner }

pred isComposite[s: MSort] { some DeclaresComposite & InForce & compositeSort.s }

sig CompositeKey in MKey { part: Role -> lone MKey }

fact { all k: CompositeKey | isComposite[sortOfKey[k]] and some k.part }

fact { all k: MKey - CompositeKey | no k.part }

pred compositeSame[x, y: MKey] {
   x + y in CompositeKey
   sortOfKey[x] = sortOfKey[y]
   x.part.MKey = y.part.MKey
   all r: x.part.MKey | tabledWalk[x.part[r], y.part[r]] = SAME
}

fun compositeMayRead[k: MKey]: set MKey { mayReadEntries[k] + mayReadEntries[Role.(k.part)] }

pred isTrue[d: DeclaresComposite] {}
```

<!-- prose-translation -->
> A MTopic whose MReferent's MState depends on several inputs in roles is an MKey of a MCompositeSort: a composite MKey names one part per role.
> That MSort's identity is its owner's function of its named parts.
> Two composite MKeys of one MCompositeSort are SAME when they name the same roles and their parts are SAME role by role.
> The may-read set of a MCompositeSort is the union of its parts' may-read sets.
> The MCompositeSort's owner, the author holding the roles, declares it.
> Declaring a MCompositeSort claims nothing about the world.

UNACKED READING, temporary (`312d:ask-composite-parts-by-walk`): 311 says the identity "is its
owner's function of its named parts"; the fence fixes that function as parts SAME role by role
by the walk alone, since `compare()` reads this predicate and Alloy refuses the recursion. The
conductor's reading, not acked, not authoritative, held only until acked or replaced
(`notes/312d` § 7).

Examples: a base and an overlay, or a primary and its replica set.

## § 3-composition-and-laws

### § 3.1-identity-of-a-key

`identity(k)` runs the lookups that take a natural MKey to a primary one
(2.1-yields-into-another-scheme) and then reads the chain of
1.8-fully-qualified-key-topic-and-derivation. Each level carries the warrants declared for the
shape its MKey matched (1.5-token-and-the-two-warrants).

```alloy
fun identity[k: MKey]: lone MKey { {p: k.*yielded | isPrimaryKey[p]} }
```

<!-- prose-translation -->
> For k an MKey of MScheme S, follow S's `resolve()`'s emission, and each yielded MScheme's emission in turn.
> Follow the emissions until an MKey of a primary MScheme is in hand.
> That MKey is the identity of k.
> There is no identity of k where no emission reaches a primary MScheme.
> The identity of an MKey of a primary MScheme is that MKey: the primary MScheme's `resolve()` is the identity on the MKey (2.2-primary-of-and-identified-in).
> The result is that MKey-Primary scoped in the identity of its MParent, recursively through each level's primary MScheme.
> The recursion terminates at a MRoot, the MRoute, or an unknown link.
> The result is the MFullyQualifiedKey of k (1.8-fully-qualified-key-topic-and-derivation).

#### § 3.1.1-what-identity-reads-beyond-the-chain

Where a `resolve()` runs is 1.10-vantage-route-placeholder-witness's; composites, cells, and
observers are 2.11-composite-sorts-and-roles's, 1.9-cell-a-singleton-sort's, and
2.8-observer-dependence-and-independence's.

<!-- normative -->
> Each `resolve()` runs from k's MVantage.
> Each emission supplies the MParent instance for the MKey it yields.
> An MKey of an observer-dependent MSort carries the O-instance in its MTopic.
> The MVantage is consulted only to know where to run `resolve()` calls and which ambient MParents to bind.

### § 3.2-compare-one-chokepoint-four-answers

Alias analysis's may/must trichotomy, plus KNOWN_UNSPOKEN for "no generator applies".

A refuted shape: a parent partitions its children's MSorts (`311u:refuted-parent-partitions-its-children`). Separation comes from one definition's own distinctions.

The walk below is the MFullyQualifiedKey MDerivation; `compare(x, y)` combines it with the
others, and the laws of § 0 are stated over both here; what each answer licenses is
3.2.1-what-the-answers-mean-to-their-consumers. Levels
are numbered from the leaf, level 0, upward through MParents; the walk aligns two chains by
height from the terminus (1.8-fully-qualified-key-topic-and-derivation). "One instance" is one
atom until 1.10-vantage-route-placeholder-witness is mechanized: the MPlaceholder inherited
through a wrapper's sentinel under `--risk-faultless-skips` (3.4-entry-and-lends), or resolved
once in one unwalled span under no wrapper, will widen `oneInstance` and nothing else. Step 1's
reasons (a MRoot shape is one MWorld, SAME by `:root`; two MKeys of that shape meet there and
compare as siblings; MRoots of two shapes are two MWorlds, and a MRoute is another) are how
1.8-fully-qualified-key-topic-and-derivation declares the termini. MKeys of different MSorts
share no primary MScheme, so their MFullyQualifiedKeys meet, if at all, only at a common
ancestor, and that meeting is not a claim about the leaves; Dorc equates MKeys and never merges
MSorts. The laws take every statement in force as true and the engine's axioms as holding, two
premises named apart because different work outside this document discharges each; they take
every statement and not an answer's own, since the support of one answer is
3.5-committee-law-and-attribution's; "a store is never among its own contents", which the
one-top way rests on, is a premise of the DISJOINT law and never a fact.

```alloy
abstract sig Answer {}

one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Answer {}

pred oneInstance[a, b: MLevel] { a = b }

pred sameAtOneLevel[a, b: MLevel] {
   oneInstance[a, b]
   or (a + b in MKey and a.mValue = b.mValue and a.shape = b.shape and guaranteesUniqueReferent[a.shape])
   or (a + b in MKey and some a.cellSort and a.cellSort = b.cellSort)
}

pred alignedSame[a, b: MLevel] {
   height[a] = height[b]
   all a2: a.*mParent, b2: b.*mParent | height[a2] = height[b2] implies sameAtOneLevel[a2, b2]
}

pred sameChains[x, y: MKey] { alignedSame[x, y] }

fun meet[x, y: MKey]: MLevel -> MLevel {
   {a: x.*mParent, b: y.*mParent |
      alignedSame[a, b] and
      no a2: x.*mParent, b2: y.*mParent | alignedSame[a2, b2] and a in a2.^mParent}
}

fun topBelow[x: MKey, a: MLevel]: lone MLevel { {t: x.*mParent | t.mParent = a} }

fun legStores[x: MKey, a: MLevel]: set MLevel { (x.^mParent & MKey) - a.*mParent }

pred twoTopsWay[tx, ty: MLevel] {
   tx + ty in MKey
   tx.scheme = ty.scheme
   guaranteesUniqueName[tx.shape] and guaranteesUniqueName[ty.shape]
   tx.mValue != ty.mValue
}

pred oneTopWay[x, y: MKey, tx, ty: MLevel] {
   x = tx and y != ty
   some primaryOf[ty.scheme]
   some s: ofScheme.(x.scheme) - x.shape | identifiedIn[s] = primaryOf[ty.scheme]
}

pred separatedAt[x, y: MKey, a, b: MLevel] {
   x != a and y != b
   let tx = topBelow[x, a], ty = topBelow[y, b] |
      twoTopsWay[tx, ty] or oneTopWay[x, y, tx, ty] or oneTopWay[y, x, ty, tx]
   all s: legStores[x, a] + legStores[y, b] | aliasesNothingElse[s]
}

fun walk[x, y: MKey]: one Answer {
   (not knownChain[x] or not knownChain[y] or worldOf[x] != worldOf[y]) implies UNKNOWN
   else sameChains[x, y] implies SAME
   else (some a: meet[x, y].MLevel, b: MLevel.(meet[x, y]) | x = a or y = b) implies UNKNOWN
   else (some a: meet[x, y].MLevel, b: MLevel.(meet[x, y]) | separatedAt[x, y, a, b]) implies DISJOINT
   else (x.scheme + x.cellSort) != (y.scheme + y.cellSort) implies KNOWN_UNSPOKEN
   else UNKNOWN
}

fun walkOfKeys[x, y: MKey]: one Answer {
   (some identity[x] and some identity[y]) implies walk[identity[x], identity[y]] else UNKNOWN
}

pred sameBy[x, y: MKey] { tabledWalk[x, y] = SAME or corresponds[x, y] or compositeSame[x, y] }

fun sameClosure: MKey -> MKey { *{x, y: MKey | sameBy[x, y] or sameBy[y, x]} }

pred contradicted[x, y: MKey] {
   y in x.sameClosure
   some x2: x.sameClosure, y2: y.sameClosure | tabledWalk[x2, y2] = DISJOINT
}

fun compare[x, y: MKey]: one Answer {
   contradicted[x, y] implies UNKNOWN
   else y in x.sameClosure implies SAME
   else (some x2: x.sameClosure, y2: y.sameClosure | tabledWalk[x2, y2] = DISJOINT) implies DISJOINT
   else tabledWalk[x, y]
}

one sig Tables { walkTable: MKey -> MKey -> Answer, compareTable: MKey -> MKey -> Answer }

fact { all x, y: MKey | Tables.walkTable[x][y] = walkOfKeys[x, y] }

fact { all x, y: MKey | Tables.compareTable[x][y] = compare[x, y] }

fun tabledWalk[x, y: MKey]: one Answer { Tables.walkTable[x][y] }

fun tabledCompare[x, y: MKey]: one Answer { Tables.compareTable[x][y] }

pred hole_cell_keys_under_same_parents_refer_differently {
   some disj a, b: MKey | some a.cellSort & b.cellSort and a.mParent != b.mParent
      and some a.mParent.mRefersTo & b.mParent.mRefersTo and a.mRefersTo != b.mRefersTo
}

pred hole_world_scoped_top_aliases_into_a_store {
   some x, s: MKey | some x.mParent & MWorld and some s.mParent & x.mParent and s != x
      and some x.mRefersTo and x.mRefersTo in s.mRefersTo.^holds
}

pred hole_composite_keys_with_same_parts_refer_differently {
   some disj a, b: CompositeKey | compositeSame[a, b] and a.mRefersTo != b.mRefersTo
}

check law_compare_same_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest
      and not hole_cell_keys_under_same_parents_refer_differently
      and not hole_composite_keys_with_same_parts_refer_differently implies
      all x, y: MKey | compare[x, y] = SAME implies x.mRefersTo = y.mRefersTo
} for 6 but 4 Int

run law_compare_same_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest
   not hole_cell_keys_under_same_parents_refer_differently
   not hole_composite_keys_with_same_parts_refer_differently
   some disj x, y: MKey | compare[x, y] = SAME and walkOfKeys[x, y] != SAME and some x.mRefersTo
}

check law_compare_disjoint_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
      and not hole_world_scoped_top_aliases_into_a_store
      and not hole_composite_keys_with_same_parts_refer_differently implies
      all x, y: MKey | compare[x, y] = DISJOINT implies no x.mRefersTo & y.mRefersTo
} for 6 but 4 Int

run law_compare_disjoint_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
   not hole_world_scoped_top_aliases_into_a_store
   not hole_composite_keys_with_same_parts_refer_differently
   some x, y: MKey | compare[x, y] = DISJOINT and walkOfKeys[x, y] != DISJOINT and some x.mRefersTo and some y.mRefersTo
}

pred axiomaticByContractExcept[except: set Spoken] {
   all d: DeclaresPrimaryOf & InForce - except | d.isTrue
   all d: DeclaresYields & InForce - except | d.isTrue
   all d: DeclaresIdentifiedIn & InForce - except | d.isTrue
   all d: DeclaresRoot & InForce - except | d.isTrue
   all d: SuppliesParent & InForce - except | d.isTrue
   all d: DeclaresUniqueReferent & InForce - except | d.isTrue
   all d: DeclaresUniqueName & InForce - except | d.isTrue
   all d: DeclaresAliasesNothingElse & InForce - except | d.isTrue
   all d: DeclaresMayRead & InForce - except | d.isTrue
   all d: ClosesMayRead & InForce - except | d.isTrue
   all d: VerdictFact & InForce - except | d.isTrue
   all d: DeclaresMayWrite & InForce - except | d.isTrue
   all d: ClosesMayWrite & InForce - except | d.isTrue
   all d: DeclaresEntails & InForce - except | d.isTrue
   all d: FinishesEntailment & InForce - except | d.isTrue
   all d: EmitsCrossed & InForce - except | d.isTrue
   all d: ClosesTraversal & InForce - except | d.isTrue
   all d: EmitsAliasNothingElse & InForce - except | d.isTrue
   all d: DeclaresPlaces & InForce - except | d.isTrue
   all d: RecordsLookedUpIn & InForce - except | d.isTrue
   all d: ClosesLookedUpIn & InForce - except | d.isTrue
   all d: DeclaresCell & InForce - except | d.isTrue
   all d: DeclaresCorresponds & InForce - except | d.isTrue
   all d: DeclaresObserverIndependence & InForce - except | d.isTrue
   all d: DeclaresComposite & InForce - except | d.isTrue
   all d: DeclaresCatalogSort & InForce - except | d.isTrue
   all d: DeclaresLends & InForce - except | d.isTrue
   all d: ClosesLends & InForce - except | d.isTrue
}

pred axiomaticByContract { axiomaticByContractExcept[none] }

pred axiomaticByDifferentialTest { shellResolvesInTheAmbientInstance }

pred noStoreIsAmongItsOwnContents { no r: MReferent | r in r.^holds }

pred noRoutePassesThroughItself { no r: MReferent | r in r.^passes }

check law_same_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest and not hole_cell_keys_under_same_parents_refer_differently implies
      all x, y: MKey | walkOfKeys[x, y] = SAME implies x.mRefersTo = y.mRefersTo
} for 6 but 4 Int

run law_same_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest
   not hole_cell_keys_under_same_parents_refer_differently
   some disj x, y: MKey | walkOfKeys[x, y] = SAME and some x.mRefersTo
}

check law_disjoint_is_sound {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents and not hole_world_scoped_top_aliases_into_a_store implies
      all x, y: MKey | walkOfKeys[x, y] = DISJOINT implies no x.mRefersTo & y.mRefersTo
} for 6 but 4 Int

run law_disjoint_is_sound_premise {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents
   not hole_world_scoped_top_aliases_into_a_store
   some x, y: MKey | walkOfKeys[x, y] = DISJOINT and some x.mRefersTo and some y.mRefersTo
      and some (x.^mParent + y.^mParent) & MKey
}

check law_nobody_spoke_declines {
   no InForce & (DeclaresUniqueReferent + DeclaresUniqueName + DeclaresRoot + DeclaresAliasesNothingElse)
      implies all disj x, y: MKey | walk[x, y] not in SAME + DISJOINT
} for 6 but 4 Int

run law_nobody_spoke_declines_premise {
   no InForce & (DeclaresUniqueReferent + DeclaresUniqueName + DeclaresRoot + DeclaresAliasesNothingElse)
   some disj x, y: MKey | knownChain[x] and knownChain[y] and worldOf[x] = worldOf[y]
}

check law_different_sorts_never_same {
   all x, y: MKey | (x.scheme + x.cellSort) != (y.scheme + y.cellSort) implies walk[x, y] != SAME
} for 6 but 4 Int

run law_different_sorts_never_same_premise {
   some x, y: MKey | (x.scheme + x.cellSort) != (y.scheme + y.cellSort) and knownChain[x] and knownChain[y] and worldOf[x] = worldOf[y]
}
```

<!-- prose-translation -->
> The walk answers one of SAME, DISJOINT, KNOWN_UNSPOKEN, or UNKNOWN.
> One level: two levels are SAME iff one of three cases holds.
> In the first case, they are one instance.
> In the second case, they are two MKeys with equal MValues whose shape carries `:guarantees-unique-referent`.
> In the third case, they are two MKeys of one cell MSort (1.9-cell-a-singleton-sort).
> Two chains are aligned-SAME at a pair of levels when two conditions hold.
> The two levels have one height.
> The two levels, and every pair of levels above them of one height, are SAME by the one-level rule.
> Two MFullyQualifiedKeys are SAME iff they are SAME at every level down to the leaf.
> Step 1: if either MFullyQualifiedKey contains an unknown link, the pair reads UNKNOWN.
> Step 1: if one MFullyQualifiedKey terminates at an MWorld the other does not share, the pair reads UNKNOWN.
> Step 2: otherwise walk downward from the top to the deepest level at which the two chains are SAME by the one-level rule.
> Call that level A.
> If either MKey is A itself, the pair reads UNKNOWN, since a write to a container collides with everything inside it.
> Step 3: otherwise call the child of A on each side that side's top.
> The top is the MKey itself when its MParent is A.
> Separation is concluded in one of two ways.
> Two tops: both tops are MKeys of one MScheme, each carrying `:guarantees-unique-name`, with differing MValues.
> One top: exactly one MKey is its own top.
> In the one-top way, the `resolve()` body of that MKey's primary MScheme declares `:identified-in` the MSort of the other side's top, for some other shape.
> Step 4: the pair reads DISJOINT iff one of the two ways holds and every store strictly below A is `:aliases-nothing-else` (2.3-aliases-nothing-else-the-store-warrant).
> The stores strictly below A are those down to either leaf's MParent.
> Separation is decided once, at A.
> Otherwise, MKeys of different MSorts read KNOWN_UNSPOKEN, and MKeys of one MSort read UNKNOWN.
> Here, two MKeys are of different MSorts in three cases (1.4-key-and-its-two-views).
> Their MSchemes differ.
> Their cell MSorts differ.
> One carries an MScheme and the other carries a cell MSort.
> Two MKeys of any MScheme are walked by their identities (3.1-identity-of-a-key).
> An MKey with no identity reads UNKNOWN.
> `compare(x, y)`: SAME is "or" across MDerivations, and "and" within one MFullyQualifiedKey.
> The MDerivations are the MFullyQualifiedKey walk, a MCorrespondence (2.7-corresponds-across-a-transition), and a MCompositeSort's function of its parts (2.11-composite-sorts-and-roles).
> SAME composes transitively.
> A warranted SAME and a warranted DISJOINT on one pair is a contradiction: the pair reads UNKNOWN, and the refusal with its attribution is 3.5-committee-law-and-attribution's.
> Otherwise the strongest warranted answer stands: SAME, else DISJOINT, else what the walk answers.
> The two tables hold the engine's walk answer and `compare()` answer over every pair of MKeys, and equal those functions pair by pair.
> The tables are plumbing the other definitions read, not objects of the model.
> SAME then DISJOINT composes to DISJOINT.
> DISJOINT then DISJOINT never chains.
> `compare()` never reaches a false SAME while every statement in force is true and the engine's axioms hold.
> `compare()` never reaches a false DISJOINT while every statement in force is true and the engine's axioms hold and no store is among its own contents.
> Every statement in force outside a named set is true when each statement in force outside that set satisfies its species' truth predicate.
> Every statement in force is true when that holds with the empty set named.
> The contract makes it axiomatic.
> The engine's axioms hold when the engine's axiom about where the shell resolves holds (1.10-vantage-route-placeholder-witness).
> Differential test makes them axiomatic, and nobody speaks them.
> A store is never among its own contents when no MReferent holds itself, directly or through others.
> No route passes through itself when no MReferent passes to itself, directly or through others.
> A held hole: two cell MKeys of one cell MSort, under two MParents that MRefer to one MReferent, MRefer to two MReferents.
> A held hole: an MKey scoped in an MWorld MRefers to an MReferent that a sibling MKey's MReferent holds, directly or through others.
> A held hole: two composite MKeys that are SAME by their parts MRefer to two MReferents.
> Each law below is asked only outside the holes its sentence names.
> The model never reaches a false SAME while every statement in force is true and the engine's axioms hold.
> Under those premises, two MKeys the walk reads SAME MRefer to one MReferent, or both MRefer to none.
> No counterexample at scope 6 is the claim, never a proof.
> The SAME law of the walk is asked outside the cell hole.
> Each premise twin of a law with premises also asks for those premises, outside that law's holes.
> The premise twin of `law_same_is_sound` asks for a world where the walk reads two distinct MKeys SAME and the first MRefers to an MReferent.
> The model never reaches a false DISJOINT under three premises.
> The premises are that every statement in force is true, the engine's axioms hold, and no store is among its own contents.
> Under those premises, two MKeys the walk reads DISJOINT MRefer to no common MReferent.
> The DISJOINT law of the walk is asked outside the world-scoped-top hole.
> The premise twin of `law_disjoint_is_sound` asks for a world where the walk reads a pair of MKeys DISJOINT and each MRefers to an MReferent.
> In that world, one of the pair has an MKey above it.
> The SAME law of `compare()` is asked outside the cell hole and the composite hole.
> The premise twin of `law_compare_same_is_sound` asks for a world where `compare()` reads two distinct MKeys SAME, the walk does not, and the first MRefers to an MReferent.
> The DISJOINT law of `compare()` is asked while no store is among its own contents, outside the world-scoped-top hole and the composite hole.
> The premise twin of `law_compare_disjoint_is_sound` asks for a world where `compare()` reads a pair of MKeys DISJOINT, the walk does not, and each MRefers to an MReferent.
> Where nobody has spoken, the model declines to answer: with no warrant of any kind in force, two distinct MKeys never read SAME or DISJOINT.
> The premise twin of `law_nobody_spoke_declines` asks for a world where no warrant is in force and two distinct MKeys have known chains in one MWorld.
> MKeys of different MSorts never read SAME by the walk.
> The premise twin of `law_different_sorts_never_same` asks for a world where two MKeys of different MSorts have known chains in one MWorld.

UNACKED READING, temporary (`312d:ask-contradiction-reads-unknown`, `312d:enc-one-instance-is-one-atom-for-now`,
and the laws' premise): 311 says a contradiction is "refuse both and attribute both authors",
which is not one of the four answers, so the fence answers UNKNOWN there; "one instance" is one
atom, the sharing of 1.10-vantage-route-placeholder-witness being by construction; and every
law premises EVERY statement in force true where 311 says "every statement behind it", the
support of 3.5-committee-law-and-attribution being unused in the premise. The conductor's
readings, not acked, not authoritative, held only until acked or replaced (`notes/312d` § 7).

#### § 3.2.1-what-the-answers-mean-to-their-consumers

The consumer map is the sparing test's and the fact-transport's; the flag's gate on a
sentinel-inherited SAME is by construction (3.4-entry-and-lends), and its gate on sparing is in
the test (2.6-may-write-the-writeset). Partial measurement never widening is a statement about
two measurements of one chain, which the fences do not hold. A provider-supplied identifier is
an MKey of a `:root` shape in this model.

<!-- normative -->
> SAME means the fact is about this MKey.
> The engine consumes a SAME that rests on a wrapper's sentinel under `--risk-faultless-skips` (3.4-entry-and-lends).
> DISJOINT licenses sparing under the same flag.
> UNKNOWN and KNOWN_UNSPOKEN are the safe bottoms.
> An omission is a distinction only inside the body that made it.
> KNOWN_UNSPOKEN never spares and never transports, whatever either side declared finished (2.6-may-write-the-writeset).
> Two MSchemes yielding one MKey-Primary is the sole same-referent generator across ways of naming.
> Partial measurement never widens: a MDerivation with an unmeasured or MRoute-terminated link yields at most what it would yield with the link measured.
> A contradiction is refused, and both authors are attributed.
> The universal meet over backing sets is unchanged.
> Genuinely different MKey-Primaries for one MReferent are two MDerivations for one MTopic, reconciled by coherence, never a second MKey inside one MParent.

Two MSorts meeting at a common ancestor: a package status file and a unit file share a filesystem. Genuinely different MKey-Primaries for one MReferent: an NFS filehandle and the server's inode, or a machine-id and a cloud instance-id.

### § 3.3-invalidation-three-mutator-species

Three mutator species invalidate three kinds of fact, and in all three the engine withdraws
authority and never computes the successor identity: below a line, an invalidated
MResolution's MFullyQualifiedKey reads unknown, so SAME loses authority, elisions demote to
guards, and DISJOINT collides. The lines above a site are the mutators that ran before it
(the shared order of lines). The read set of the lookup body is derived from the body, which
the fences do not hold, so whether it is open is an uninterpreted relation with 311's sentence
as its only axiom (`plans/30Z` § 2.6). The state mutation's kill-reach is the sparing test of
2.6-may-write-the-writeset itself, and a lifecycle write to a MRoot-adjacent MKey is caught by
that test as a write to a container, since every MKey scoped in it meets it at itself; what the
fences add is the token that a state mutation to the MParent-Store invalidates.

```alloy
one sig Engine { lookupReadSetOpen: set MScheme }

fun lineWriteset[l: Line]: set MKey { writesetUnexcluded[l] }

pred lineWritesetIsTop[l: Line] { writesetIsTop[l, lineWriteset[l]] }

pred hasTraversalMembers[k: MKey] {
   (some l: levelsOf[k] | some crossed[l] or (not traversalClosed[l] and some l.mParent & MKey))
   or some (RecordsLookedUpIn & InForce & placedKey.k).inKey
}

pred touchesTraversal[w: MKey, k: MKey] {
   (some l: levelsOf[k] |
      (some m: crossed[l] | tabledCompare[w, m] != DISJOINT)
      or (not traversalClosed[l] and some p: l.mParent & MKey | regionTest[p, w] != DISJOINT))
   or some g: (RecordsLookedUpIn & InForce & placedKey.k).inKey | tabledCompare[w, g] != DISJOINT
}

pred routingInvalidatedBy[l: Line, k: MKey] {
   (lineWritesetIsTop[l] and hasTraversalMembers[k])
   or (some w: lineWriteset[l] | touchesTraversal[w, k])
   or ((lineWritesetIsTop[l] or some lineWriteset[l]) and some (levelsOf[k] + k).scheme & Engine.lookupReadSetOpen)
}

pred tokenInvalidatedBy[l: Line, k: MKey] {
   (lineWritesetIsTop[l] and some identity[k].^mParent & MKey)
   or (some w: lineWriteset[l], p: identity[k].^mParent & MKey | tabledCompare[w, p] != DISJOINT)
}

pred lifecycleInvalidatedBy[l: Line, k: MKey] {
   (lineWritesetIsTop[l] and some w: identity[k].^mParent & MKey | some w.mParent & MRootWorld)
   or (some w: lineWriteset[l] | some w.mParent & MRootWorld and w in identity[k].^mParent)
}

pred staleAt[s: Line, k: MKey] {
   some l: s.above | routingInvalidatedBy[l, k] or tokenInvalidatedBy[l, k] or lifecycleInvalidatedBy[l, k]
}

fun compareAt[s: Line, x, y: MKey]: one Answer {
   (staleAt[s, x] or staleAt[s, y]) implies UNKNOWN else tabledCompare[x, y]
}

pred hole_unclosed_traversal_without_a_key_catalog {
   some k: MKey, l: levelsOf[k] | not traversalClosed[l] and no l.mParent & MKey
}

pred hole_natural_key_catalog_off_the_route {
   some k: MKey | isNaturalKey[k] and some k.mParent & MKey and some k.mRefersTo
      and k.mRefersTo not in k.mParent.mRefersTo.passes
}

pred hole_a_route_off_the_catalog_reaches_the_thing {
   some k: MKey | not traversalClosed[k]
      and some passes.(k.mRefersTo) - (k.mParent & MKey).mRefersTo - crossed[k].mRefersTo
}

check law_unstale_route_is_untouched {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents and noRoutePassesThroughItself
      and not hole_unclosed_traversal_without_a_key_catalog
      and not hole_natural_key_catalog_off_the_route
      and not hole_a_route_off_the_catalog_reaches_the_thing
      and not hole_region_closure_with_unknown_leaf_pair implies
      all s: Line, k: MKey, l: s.above | not routingInvalidatedBy[l, k] and atMostClosed[l] implies
         no World.lineWrites[l] & passes.(levelsOf[k].mRefersTo + k.mRefersTo)
} for 6 but 4 Int, 9 Claim

run law_unstale_route_is_untouched_premise {
   axiomaticByContract and axiomaticByDifferentialTest and noStoreIsAmongItsOwnContents and noRoutePassesThroughItself
   not hole_unclosed_traversal_without_a_key_catalog
   not hole_natural_key_catalog_off_the_route
   not hole_a_route_off_the_catalog_reaches_the_thing
   not hole_region_closure_with_unknown_leaf_pair
   some s: Line, k: MKey, l: s.above |
      not routingInvalidatedBy[l, k] and atMostClosed[l] and some World.lineWrites[l] and some crossed[levelsOf[k]]
}
```

<!-- prose-translation -->
> A routing mutation touches routing MKeys or a MParent-Catalog.
> A writeset entry touches a MTraversal member when `compare()` answers other than DISJOINT for the pair (3.2-compare-one-chokepoint-four-answers).
> Where the member is a MParent-Catalog given whole, the region test gives that answer.
> A routing mutation invalidates a MResolution when its MTraversal includes a touched MKey.
> Any write invalidates a MResolution whose read set is open.
> The sparing test of 2.6-may-write-the-writeset decides what a state mutation invalidates.
> Invalidation reads the line's writeset with every container contributing, there being no read MKey to exclude against.
> For invalidation as for the sparing test, a line's writeset is ⊤ where its at-most set is unclosed or a member has no reached finished record (2.6-may-write-the-writeset).
> ⊤ touches every MTraversal member, every MParent-Store, and every MRoot-adjacent MKey, since ⊤ is DISJOINT from nothing.
> A line whose writeset is ⊤ is a write for a MResolution whose read set is open.
> A first write can also change an MKey-Primary, so a state mutation whose writeset touches a MParent-Store invalidates the MTokens scoped in it.
> A lifecycle mutation writes a MRoot-adjacent MKey, and invalidates every MKey whose identity's chain passes through that MKey.
> Below a site, an MKey whose MResolution, MToken, or MWorld a line above invalidated is stale, and every MFullyQualifiedKey built on it reads unknown there.
> A held hole: some level of an MKey has no closing act and no MKey for its MParent-Catalog.
> A held hole: a natural MKey with an MKey for its MParent-Catalog MRefers to an MReferent that no route through the catalog's MReferent passes to.
> A held hole: an MKey with no closing act MRefers to an MReferent that a third MReferent passes to.
> That third MReferent is neither the MReferent of the MKey's MParent-Catalog nor an MReferent that an emitted member MRefers to.
> The untouched-route law has four premises.
> Every statement in force is true, and the engine's axioms hold.
> No store is among its own contents, and no route passes through itself.
> Under those premises, the law concerns a line that closed its at-most set and invalidates no MResolution of an MKey.
> Such a line writes nothing that a route to that MKey's MReferents passes through.
> The untouched-route law is asked outside those three holes and the region hole (2.9-the-traversal-and-the-region-test).
> The premise twin of `law_unstale_route_is_untouched` asks for a world where a line that closed its at-most set writes something and invalidates no MResolution of an MKey.
> In that world, the line is above a site, and a level of that MKey has an emitted member.
> The twin also asks for the law's premises, outside the law's four holes.

Scope: the untouched-route law runs at nine statements because its twin's witness needs seven in force at once (three traversal closures, one emitted member, the line's at-most entry and its completion, and the name warrant that separates the written key from the crossed one) and is unsat at the six every other command shares.

#### § 3.3.1-what-invalidation-withdraws

<!-- normative -->
> In all three species the engine withdraws authority.
> The engine never computes the successor identity.
> Every MKey-Primary scoped in a lifecycle-written MKey names a new MReferent afterward.
> A routing mutation also touches shell state a `resolve()` read.
> A write invalidates a MResolution when a writeset entry `compare()`s other than DISJOINT with a member of the lookup body's read set.
> That lookup body is the one that produced the MResolution (1.7-resolution-and-its-traversal).
> Dependent SAME conclusions lose authority, dependent elisions demote to guards, and dependent DISJOINT conclusions collide.
> The touched object itself is untouched.
> Creation, deletion, and rename of an MKey are routing writes.
> They change what the MKey MRefers to.
> The MKeys they write are the verb author's at-most claim.
> A writeset that omits them is the ordinary at-most omission knife, which now visibly covers routing MKeys.
> Cells whose MFullyQualifiedKeys pass through a lifecycle-written MKey are new and unmeasured.
> Cells whose MFullyQualifiedKeys do not pass through it are untouched.
> "Keyed by Boot" and "invariant across Boot" are the shape of the MFullyQualifiedKey, not declarations.

Routing mutations: a mount, a symlink replacement, a rename, a user added, a hostname change, a write to any environment variable, cwd, or configuration a lookup reads. `userdel alice; useradd alice` invalidates every MResolution of the old MKey. Lifecycle mutations: a reboot, a re-provision; the MRoot-adjacent MKey they write is a boot's or a tenure's.

### § 3.4-entry-and-lends

Dynamic binding: `parameterize`, `fluid-let`.

The lent instance becomes the ambient MParent for every MKey of a secondary MScheme looked up
in that MParent-Catalog MSort (1.10-vantage-route-placeholder-witness, 1.6-parent-one-per-key),
and leaf MKeys then inherit transitively through their MFullyQualifiedKeys with no further
speech. A wrapper may declare MCorrespondences across the MParent-Catalogs it lends
(2.7-corresponds-across-a-transition). A lend that depends on the guest is
3.4.1-guest-dependent-lends. Arity: per wrapper, per MParent-Catalog MSort, plus the sentinel.
Declared by: the wrapper owner. Default: ⊤, which walls. Consumer: ambient MParent supply.
Danger: a wrong lend measures the wrong MVantage; a wrong sentinel is a wrong SAME, and the
flag prices it.

```alloy
lone sig RiskFaultlessSkips {}

pred flagged { some RiskFaultlessSkips }

sig Wrapper { wrapperOwner: one Speaker }

sig DeclaresLends extends Spoken { lendingWrapper: one Wrapper, lentSort: one MSort, lentInstance: one MKey }

sig ClosesLends extends Spoken { closedWrapper: one Wrapper }

fact { all d: DeclaresLends | d.speaker = d.lendingWrapper.wrapperOwner }

fact { all d: ClosesLends | d.speaker = d.closedWrapper.wrapperOwner }

fun lent[w: Wrapper, s: MSort]: lone MKey {
   (DeclaresLends & InForce & lendingWrapper.w & lentSort.s).lentInstance
}

fact { all w: Wrapper, s: MSort | lone lent[w, s] }

pred lendsClosed[w: Wrapper] { some ClosesLends & InForce & closedWrapper.w }

pred inherits[v: MVantage] { some v.through and lendsClosed[v.through] and flagged }

fact {
   all v: MVantage, s: MSort | some v.through implies
      v.ambient[s] = (some lent[v.through, s] implies lent[v.through, s]
                      else inherits[v] implies v.enteredFrom.ambient[s]
                      else none)
}

fact {
   all v: MVantage | some v.through implies
      (inherits[v] implies v.route = v.enteredFrom.route else v.route != v.enteredFrom.route)
}

fun keysUnder[w: Wrapper]: set MKey { {k: MKey | k.at.through = w} }

pred isTrue[d: DeclaresLends] {
   all k: keysUnder[d.lendingWrapper] |
      isNaturalKey[k] and catalogSortOf[k.scheme] = d.lentSort implies
         k.mRefersTo in d.lentInstance.mRefersTo.passes
}

pred isTrue[d: ClosesLends] {
   let w = d.closedWrapper {
      all k: keysUnder[w] | isNaturalKey[k] and no lent[w, catalogSortOf[k.scheme]]
            and some k.at.enteredFrom.ambient[catalogSortOf[k.scheme]] implies
         k.mRefersTo in k.at.enteredFrom.ambient[catalogSortOf[k.scheme]].mRefersTo.passes
      all k: keysUnder[w], j: MKey | j.at = k.at.enteredFrom and k.scheme = j.scheme and k.mValue = j.mValue
         and k.mParent = k.at.route and j.mParent = j.at.route implies k.mRefersTo = j.mRefersTo
   }
}
```

<!-- prose-translation -->
> `--risk-faultless-skips` is set for one invocation of Dorc, or it is not.
> A wrapper's entry `:lends` MParent-Catalog instances for the MParent-Catalog MSorts it perturbs, and nothing else.
> The wrapper owner declares each lend, one instance per lent MSort, and the completion sentinel.
> An unlent MParent-Catalog MSort is ⊤ under the wrapper: a vantage entered through the wrapper holds no instance for it.
> After the wrapper's completion sentinel, and under `--risk-faultless-skips`, the unlent MSorts and the MRoute inherit the caller's instances instead.
> Otherwise the MRoute is unknown across the two vantages, another MRoute.
> A lend concerns every MKey of a secondary MScheme looked up in the lent MSort under the wrapper.
> A lend is true when every such MKey MRefers to what a route through the lent instance's MReferent passes to.
> The sentinel is an at-most claim over every MParent-Catalog MSort and the MRoute.
> The sentinel is true when two conditions hold.
> The first condition concerns each MKey of a secondary MScheme looked up in an unlent MSort under the wrapper.
> The first condition applies where the caller holds an instance for that MSort.
> Under the first condition, each such MKey MRefers to what a route through the caller's instance passes to.
> The second condition is that an MKey scoped in the MRoute under the wrapper MRefers to what its same-spelled twin scoped in the caller's MRoute MRefers to.

UNACKED READING, temporary (`312d:enc-lends-truth-is-routing`, `312d:enc-vantage-is-the-entry-chain`):
311 says a wrapper lends instances "for the MParent-Catalog MSorts it perturbs, and nothing
else"; the fence reads "perturbs" through the world relation `passes` and reads the MRoute's
inheritance as sameness of what same-spelled route-scoped MKeys MRefer to, and it makes a vantage
not inheriting hold a DIFFERENT MRoute atom. The conductor's readings, not acked, not
authoritative, held only until acked or replaced (`notes/312d` § 7).

#### § 3.4.1-guest-dependent-lends

<!-- normative -->
> A lend may depend on the guest.
> The wrapper author then declares the guest-insensitive default and supplies a policy read that declines on departure.

Examples: a chroot lends a mount namespace. `sudo -u` lends a user. `ip netns exec` lends a network namespace. A lend that depends on the guest: sudoers matches the guest command.

### § 3.5-committee-law-and-attribution

Every statement species in this document carries one speaker (the shared `Spoken`), and
each species' speaker fact names the party who can know it about their own tool, store, or
machine: the engine only chains and meets. What the fences add here is the support of an
answer, the statements it rested on, so that the two shapes of composition are checkable: a
granting composite (a SAME or a DISJOINT) rests on more than one author only where each author
spoke about their own lookup; a withholding composite (an UNKNOWN, a KNOWN_UNSPOKEN, a
collision) names nobody. Attribution renders the support; the aid plane's rendering is
`AID-NEEDS.md`'s. The two-tops way's support is one MScheme's owner by construction; the
one-top way's is one MScheme's owner and the stores' describers on both legs; a SAME's support
is the warrants of every level, the MCorrespondences, and the sentinels that made instances one.

```alloy
fun warrantsOn[k: MLevel]: set Spoken {
   (DeclaresUniqueReferent & InForce & referentShape.(k.shape))
   + (DeclaresUniqueName & InForce & nameShape.(k.shape))
   + (DeclaresRoot & InForce & rootedShape.(k.shape))
   + (DeclaresIdentifiedIn & InForce & onShape.(k.shape))
   + (SuppliesParent & InForce & forKey.k)
   + (DeclaresAliasesNothingElse & InForce & store.k)
}

fun chainSupport[x: MKey]: set Spoken {
   warrantsOn[mFullyQualifiedKey[identity[x]]]
   + (DeclaresYields & InForce & fromShape.((x.*yielded).shape))
   + (DeclaresLends & InForce & lentInstance.(MSort.(x.at.ambient)))
   + (ClosesLends & InForce & closedWrapper.(x.at.*enteredFrom.through))
}

fun sameSupport[x, y: MKey]: set Spoken {
   chainSupport[x.sameClosure] + (DeclaresCorresponds & InForce & (keyX + keyY).(x.sameClosure))
}

fun disjointSupport[x, y: MKey]: set Spoken { chainSupport[x] + chainSupport[y] }

fun speakersOf[s: set Spoken]: set Speaker { s.speaker }

check law_disjoint_by_two_tops_rests_on_one_scheme_owner {
   all x, y: MKey, a, b: MLevel |
      meet[identity[x], identity[y]] = a -> b and separatedAt[identity[x], identity[y], a, b]
         and twoTopsWay[topBelow[identity[x], a], topBelow[identity[y], b]] implies
            one speakersOf[DeclaresUniqueName & InForce & nameShape.((topBelow[identity[x], a] + topBelow[identity[y], b]).shape)]
} for 6 but 4 Int

run law_disjoint_by_two_tops_rests_on_one_scheme_owner_premise {
   some x, y: MKey, a, b: MLevel |
      meet[identity[x], identity[y]] = a -> b and separatedAt[identity[x], identity[y], a, b]
         and twoTopsWay[topBelow[identity[x], a], topBelow[identity[y], b]]
}

```

<!-- prose-translation -->
> The support of a SAME has four parts.
> The first part is the warrants at every level of the chains it rests on.
> The second part is the yields that reached them.
> The third part is the lends and sentinels that made instances one.
> The fourth part is the MCorrespondences.
> The support of a DISJOINT is the warrants at every level of the two chains.
> A DISJOINT by the two-tops way rests on one MScheme's owner's `:guarantees-unique-name` declarations.
> The premise twin of `law_disjoint_by_two_tops_rests_on_one_scheme_owner` asks for a world where the walk separates two MKeys by the two-tops way.

UNACKED READING, temporary (`312d:enc-support-functions`): 311 lists what attribution names;
the support functions above are the conductor's construction of "the statements an answer
rested on", and 311 does not define such a set. Not acked, not authoritative, held only until
acked or replaced (`notes/312d` § 7).

#### § 3.5.1-composites-and-attribution

That every positive step is one author's line is carried, species by species, by the speaker
fact beside each species and its translation; that a withholding answer names nobody is the
absence of a support function for UNKNOWN and KNOWN_UNSPOKEN, which a reviewer reads off the
definitions and no check can state.

<!-- normative -->
> Every positive step is one author's line.
> The positive steps are these: a `:yields` and its lookup warrants, a shape's `:identified-in` and its warrants, and a may-read set and its sentinel.
> The positive steps are also a may-write entailment and its finished record, a MCorrespondence, an `:observer-independence`, and a `:lends`.
> So the speech needed grows with the number of authors, never with the number of pairs of them.
> The engine only chains and meets.
> Every statement an answer rests on is something one party can know about their own tool, store, or machine.
> That party says it alone, describes nothing they cannot see, and names no other author.
> One MScheme's `:yields` and the primary MScheme's declaration for that shape jointly entail a granting composite.
> Each author speaks about their own lookup.
> A withholding composite names nobody and needs nobody's consent.
> Every survival names the `:aliases-nothing-else` and `:guarantees-unique-name` declarations it rested on.
> Every survival names the route closures it rested on (1.5-token-and-the-two-warrants, 2.10-places-the-upward-lookup).
> Every survival names the closed may-read sets its writeset rested on.
> Every SAME names the `resolve()` calls, the declarations, the sentinels and route claims that made instances one, and the MCorrespondences.
> Every invalidated conclusion names the writeset that invalidated it.

A granting composite: "these two accounts are one". A withholding composite: a mount invalidating an account's MResolution.

## § 4-relation-to-other-documents

### § 4.1-boundary-of-this-model

The boundary is prose by nature: it names what other documents own and what this one refuses,
and no fence can hold an absence.

<!-- normative -->
> This model uses, and does not redefine, the verdict, vouch, and guard tier.
> This model uses, and does not redefine, the authored may-write entries, the completion record as the witness of a finished definition, and `--risk-faultless-skips`.
> This model uses, and does not redefine, the four-answer chokepoint and its consumer map.
> The one exception in that consumer map is that a sentinel-inherited SAME rides the flag (3.4-entry-and-lends).
> This model uses, and does not redefine, the universal meet.
> This model uses, and does not redefine, measure-in-context, entry forms, siting vouches, the escalation dial, and `safe-across` (`plans/27C`).
> This model uses, and does not redefine, the read-set closure as the falsification net for unmarked reads (`plans/27C` §4(a)(B)).
> This model uses, and does not redefine, binds as the MKey-minting act.
> This model uses, and does not redefine, the MPlaceholder and the standup `witness()`.
> This model uses, and does not redefine, the integrity plane and the committee law.
> The model excludes a selector dialect, an aspect species, an authored region predicate, an engine-side name floor, and an engine table that generates SAME.
> The context slot is a MVantage and nothing else.

### § 4.2-supersessions-pending-in-prior-documents

A living register of statements in prior documents that this model contradicts. An entry stays
until its document is rewritten. Remove an entry when its document catches up. Add an entry when
another prior document is found to disagree. Each entry names the passage, gives its claim, then
gives this model's claim after "Here". A root document gets one brief entry where a passage
became untrue. The human refreshes root documents. The register is normative as prose: each
entry's "Here" sentence is a claim this model makes against a named passage, and the fences
that carry the claim are the ones the entry cites.

<!-- normative -->
> Each entry below stands until its document is rewritten, and each "Here" is this model's claim against the passage the entry names.
> `30U:rul-cross-kind-sparing-needs-a-finished-definition`, `30U:the-record` "As a generator", and `30U:constraints-on-other-components` "The comparison", with `30T:rul-binder-claims-are-ordinary`: a finished definition generates cross-kind provably-disjoint verdicts.
> The same passages say that a footprint cell is found disjoint from another kind's backing cell through a finished definition.
> Here: a finished definition stays necessary for sparing across MSorts and generates no DISJOINT (2.6-may-write-the-writeset).
> `compare()` decides every pair.
> A cross-MSort pair the walk does not separate reads KNOWN_UNSPOKEN, whatever is finished (3.2-compare-one-chokepoint-four-answers).
> `ANALYZER-NEEDS:an-kind-reach`, `ANALYZER-NEEDS:an-compare-chokepoint`, and `ANALYZER-NEEDS:an-disjointness`: the `unrelated` answer is the cross-kind answer only absent the claimed kind's finished definition, and the record licenses cross-kind sparing.
> Here: `unrelated` is KNOWN_UNSPOKEN, and `provably-disjoint` is DISJOINT.
> KNOWN_UNSPOKEN never spares, whatever is finished (3.2-compare-one-chokepoint-four-answers).
> `plans/30W` §1 and §5, with `26Ob:res-per-index-relation-table`: a kind's owner declares the kind referent-transparent, one grade under which token equality gives same and token inequality gives disjoint.
> Here: a lookup carries two independent warrants per matched shape, `:guarantees-unique-referent` and `:guarantees-unique-name`, each absent by default (1.5-token-and-the-two-warrants, 2.2-primary-of-and-identified-in).
> `:root` is the separate per-shape claim of global comparability (§2.2).
> `plans/30W` §1 index-kinds and §10 build item 1, with `26Ob:res-worlds-compare-through-the-chokepoint`: the context slot is a product over index-kinds.
> The same passages say that a world is a coordinate in a cell's key.
> Here: the context slot is a MVantage and nothing else, an address that is part of no MKey's identity (1.10-vantage-route-placeholder-witness, 4.1-boundary-of-this-model).
> Identity is the MFullyQualifiedKey (1.8-fully-qualified-key-topic-and-derivation).
> A world is an MWorld, a terminus of a MFullyQualifiedKey.
> No MFullyQualifiedKey and no finished definition speaks across MWorlds.
> A MCorrespondence may (§1.8, 3.2-compare-one-chokepoint-four-answers).
> `plans/30W` §2 and §3 `kind__disjoint()`, its `30W:rul-disjoint-is-an-rc-predicate` [TYPED], and `30T:file-identity` on the region predicate: an owner-authored region predicate generates disjointness between regions.
> Here: there is no authored region predicate (4.1-boundary-of-this-model).
> Containment is membership in a MTraversal.
> The region test over emitted MTraversals (2.9-the-traversal-and-the-region-test) and the `:places` lookup (2.10-places-the-upward-lookup) decide it.
> `plans/30W` §2 to §4, `26Ob:res-cell-level-relation-is-the-filtered-meet` and `26Ob:10f-the-target-pin`, `plans/27C` §4(A), `ANALYZER-NEEDS:an-invariance-speech-act`, and `271:rul-invariance-speech-act` [TYPED]: the kind owner's invariance line (`undivided-by-transit-across`, `invariant:<axis>`, `: user-invariant`) licenses transport across an index or an axis.
> The same passages say that the store member yields invariant, keyed, or ⊤ per (kind, selector, index-kind).
> Here: there is no invariance line and no per-kind table against axes (4.1-boundary-of-this-model).
> Whether a lifecycle write or a lent instance reaches a cell is the shape of its MFullyQualifiedKey, not a declaration (3.3-invalidation-three-mutator-species, 3.4-entry-and-lends).
> Leaf MKeys inherit across a wrapper with no further speech, under the flag (§3.4).
> The observer half of the line is `:observer-independence` of O, declared per MSort and absent by default (2.8-observer-dependence-and-independence).
> The store half is displaced by `:aliases-nothing-else` and measured MTokens (2.3-aliases-nothing-else-the-store-warrant, 3.2-compare-one-chokepoint-four-answers).
> `notes/272` §3 (the carried-by table and emission-set non-interference) and `plans/27C` §4 (the who-am-I derivation as contradiction-checker) make one claim.
> An engine-owned substrate-by-axis table and a taint over who-am-I ingredients derive keying and check declarations.
> Here: the engine holds no table that generates SAME (4.1-boundary-of-this-model).
> Keying is the MFullyQualifiedKey's shape (3.3-invalidation-three-mutator-species).
> No `resolve()` can measure observer-dependence, so it remains speech (2.8-observer-dependence-and-independence).
> The contradictions the engine refuses are three.
> The first is two seats that disagree on an MParent instance (1.6-parent-one-per-key).
> The second is a warranted SAME against a warranted DISJOINT (3.2-compare-one-chokepoint-four-answers).
> The third is two disagreeing answers from one placing lookup (2.10-places-the-upward-lookup).
> `notes/272` §5 the fence: emitted locators feed only the dependence bit and the keying recipe, and are never compared against File facts.
> Here: a may-read entry is an MKey that `compare()`s against every writeset entry.
> An entry naming a store reaches every MKey relative to that store (2.5-may-read-the-readset, 2.6-may-write-the-writeset).
> `30T:file-identity` per-aspect identity: "same file" is one relation per aspect, and the identity tier carries an authored per-aspect relation mapping.
> Here: there is no aspect species (4.1-boundary-of-this-model).
> Each aspect is a cell, a singleton MSort with its own may-read set and its own `:observer-dependence` (1.9-cell-a-singleton-sort).
> Same-for-existence and same-for-contents are two facts in the filesystem describer's vocabulary.
> Contents is a fact about the inode the path MScheme yields (2.3-aliases-nothing-else-the-store-warrant).
> Existence is a fact about the directory, or about an MReferent keyed by name in it where the describer mints one.
> Creation, deletion, and rename write the one the describer named (3.3-invalidation-three-mutator-species).
> `30T:file-identity` the v0 floor: entry-mutating verbs make no at-most claims, and same-kind path-distinct comparisons answer unknown.
> Here: creation, deletion, and rename of an MKey are routing writes, and the MKeys they write are the verb author's at-most claim (3.3-invalidation-three-mutator-species).
> Path-distinct MKeys separate under `:guarantees-unique-name` and `:aliases-nothing-else` (3.2-compare-one-chokepoint-four-answers).
> `plans/30W` §4 "Containment among index-kinds": containment among stores is a `reaches` relation their owners declare.
> Here: containment among stores is the MParent chain (1.6-parent-one-per-key, 1.8-fully-qualified-key-topic-and-derivation).
> The may-write entailment carries effects, never identity (2.6-may-write-the-writeset).
> `notes/277` §3 (the selector dialect, the survival-license algebra), `plans/30J` §12 (dialect keying), and the `spike/AGENTS.md` sparing-algebra invariant make one claim.
> A same-entity claim spares a backing only when both carry minted selectors of one dialect and the two selectors differ.
> Here: the model excludes a selector dialect (4.1-boundary-of-this-model).
> Two cells of one MParent are two MSorts.
> They separate only as 3.2-compare-one-chokepoint-four-answers separates any two MSorts (1.9-cell-a-singleton-sort).
> `spike/AGENTS.md` compare-consumer-map and `311a:note-transport-single-consented-sparing-double`: every SAME is vouch-tier and unflagged.
> Here: the engine consumes a SAME that rests on a wrapper's sentinel under `--risk-faultless-skips` (3.4-entry-and-lends, 3.2-compare-one-chokepoint-four-answers).
> Every other SAME generator is vouch-tier.
> `KNOBS:kSURVIVAL` and `ANALYZER-NEEDS:an-mode-gate`: the flag gates the survival tier's sparing.
> Here: the flag also gates a SAME that rests on a wrapper's sentinel (3.4-entry-and-lends).
> `USER_STORY.md`, the bought-unsoundness section: past the flag the admin trusts named authors' at-most claims, and everywhere else only measurements.
> Here: past the flag the admin also trusts wrappers' sentinels (3.4-entry-and-lends).

## § 5-the-relations-indexed-two-ways

This section is non-normative. It indexes the statements that § 1 to § 3 define, and it
defines nothing of its own. Where this section and § 1 to § 3 disagree, § 1 to § 3 govern.
The first index (5.1-by-what-is-true-in-the-world) orders the statements by the fact about the
world that each statement is about. It is the table of `311t` § 11, with the route row of
`311t` § 14 and the two levels of `311t` § 15. The second index
(5.2-by-what-a-false-statement-costs) orders the statements by the wrong answer that a false
statement yields. It collects the Danger lines of § 2 in one place, with the vouches and the
routing statements beside them.

### § 5.1-by-what-is-true-in-the-world

Each row is one fact about the world, about a thing T and a container P. Each fact has two
ends: the party who describes T, and the party who describes P. Each end can make two
statements. An entry adds collisions. A closure removes them. The closure is the knife.

The rows:

- ADDR: T's MKey means something only relative to P.
- WRITE: a write to P can change T, and P is not on T's chain.
- ROUTE: T is reachable through P.
- VANT: an answer about T depends on where the read ran.
- ATTEST: an answer about T is A's word.

The levels of ADDR and ROUTE:

- KEY: one MReferent, one MKey, inside one store.
- STORE: one MReferent, one store, across stores.
- CAT: inside one catalog instance.
- XCAT: across catalogs.

The ends:

- T: the describer of the thing.
- P: the describer of the container, or of the written thing.
- W: the author of the wrapper.
- X: the owner of the transition.
- E: the engine.

A grain in brackets says what one statement covers. The licenses column names the consumer that
reads the closure:

- SAME.
- DISJ: DISJOINT.
- SPARE: the sparing test of 2.6-may-write-the-writeset.
- INVAL: invalidation (3.3-invalidation-three-mutator-species).

OPEN marks a cell with no statement in the model. TABLED marks a row that the ledgers set aside
(`311t` § 13). A dash marks a cell with no statement, where the default does the work.

| row    | level | end  | entry                                                            | closure                                                                  | licenses     | §             |
| ------ | ----- | ---- | ---------------------------------------------------------------- | ------------------------------------------------------------------------ | ------------ | ------------- |
| ADDR   | KEY   | P    | the lookup yields the canonical MKey [shape]                     | `:guarantees-unique-name` [shape] · `:guarantees-unique-referent` [shape] | DISJ · SAME  | 1.5, 2.1, 3.2 |
| ADDR   | KEY   | T    | `alias k'` [key]                                                 | `alias nothing-else` [key, level]                                        | DISJ         | 1.5, 2.9      |
| ADDR   | STORE | P    | `:corresponds`[^corr]                                            | `:aliases-nothing-else` [store]                                          | DISJ         | 2.3, 2.7, 3.2 |
| ADDR   | STORE | T    | `:identified-in` [shape] · `:corresponds`[^corr]                 | OPEN[^open] · `:root` [shape]                                            | — · SAME     | 1.6, 2.2      |
| WRITE  | —     | T    | `may-read` [sort] · the marked reads [body]                      | `may-read nothing-else` [sort] · the vouch [body]                        | SPARE        | 2.5           |
| WRITE  | —     | P    | the entailment [sort, shape] · the at-most set [verb, shape]     | the finished record [sort, shape] · the completion record [verb, path]   | SPARE        | 2.6           |
| ROUTE  | CAT   | T    | the emitted MTraversal [lookup, shape]                           | the closing act[^act] · `alias nothing-else` [level]                     | INVAL · DISJ | 1.7, 2.9      |
| ROUTE  | CAT   | P    | —                                                                | —[^floor]                                                                | —            | 2.9           |
| ROUTE  | XCAT  | T    | `looked-up-in G:key` [key][^placing]                             | `looked-up-in nothing-else` [key, route-sort]                            | DISJ · INVAL | 2.10          |
| ROUTE  | XCAT  | P    | `:places` [G, T] · the finished entailment's members[^shared]    | the finished record[^shared]                                             | SPARE        | 2.10          |
| VANT   | —     | T    | —[^dep]                                                          | `:observer-independence` of O [sort, O]                                  | SAME         | 2.8           |
| VANT   | —     | W    | `:lends` [wrapper, sort] · `:corresponds` [lent catalogs]        | the completion sentinel [wrapper]                                        | SAME         | 3.4           |
| VANT   | —     | E    | —                                                                | the local-route axiom [span]                                             | SAME         | 1.5, 1.10     |
| ATTEST | —     | T, A | TABLED                                                           | TABLED                                                                   | —            | `311t` § 12   |

[^corr]:
    `:corresponds` is the transition owner's statement (2.7-corresponds-across-a-transition).
    The table files it at both ends of the STORE level, as the positive twin of each closure
    (`311t` § 15). It generates SAME, so this entry is itself a knife (5.2).

[^open]:
    The thing's end has no closure at the STORE level: "this MReferent has no other home".
    `311t` § 15 argues that the cell may stay empty, because the cases that need it decline and
    read UNKNOWN.

[^act]:
    The model gives the closing act no spelling. The `looked-up-in` records of
    2.10-places-the-upward-lookup are its natural form (`312cg` § 18).

[^floor]:
    A catalog makes no statement about its own entries. A writeset entry that names the catalog
    whole covers every MResolution through it (2.9-the-traversal-and-the-region-test).

[^placing]:
    G's owner publishes the placing lookup (2.10-places-the-upward-lookup). The record it emits
    is about T, so the table files it at T's end.

[^shared]:
    One statement in two rows. The finished entailment is the store's end of WRITE and of ROUTE
    (2.10-places-the-upward-lookup).

[^dep]: Dependence is the default and has no spelling (2.8-observer-dependence-and-independence).

### § 5.2-by-what-a-false-statement-costs

Each row is one statement whose falsity yields a wrong answer. A false entry adds a collision
and costs sparing only, so entries are absent from this table. An omitted entry is a false
closure, and the closure's row prices it. Four routing statements are present although they are
entries. Each picks which MReferent or which instance is meant, so a false one keys a fact to
the wrong thing.

The kinds:

- route: picks a thing or an instance.
- warrant: a per-shape grade on a lookup.
- closure: a per-key statement, made on the path that measured the MKey.
- sentinel: the closing act on a declared set.
- record: the closing act on an emitted set, at a body's tail.
- vouch: one party's statement that stands for a set of measurements.
- axiom: an engine rule about where the shell resolves, discharged by differential test; never a
  party's statement.

The built column says when the statement comes into being:

- decl: declared, before any lookup runs.
- eval: constructed on the path that measured the MKey.

The if-false column names the wrong answer:

- wSAME: one fact stands for another thing's fact.
- wDISJ: a license survives a write that destroyed it.
- wSPARE: the same survival, reached through the sparing test.
- stale: a MResolution stands after a write that should have invalidated it, and every
  conclusion built on it stands with it.
- vantage: the engine keys a fact at the wrong instance.

The flag column says whether the engine consumes the answer only under `--risk-faultless-skips`
(3.2-compare-one-chokepoint-four-answers, 3.4-entry-and-lends). An INVAL cell marked "no"
inherits the flag of the consumer that a kept MResolution feeds, since a stale MResolution
reaches a decision only past a running line. The consumer column orders the rows. Within one
consumer, the unflagged rows come first.

| statement                              | kind     | speaker              | grain           | built      | consumer     | if-false        | flag     | §             |
| -------------------------------------- | -------- | -------------------- | --------------- | ---------- | ------------ | --------------- | -------- | ------------- |
| `:guarantees-unique-referent`          | warrant  | the lookup's owner   | shape           | decl, eval | SAME         | wSAME           | no       | 1.5, 2.2      |
| `:root`                                | warrant  | T's owner            | shape           | decl       | SAME         | wSAME           | no       | 2.2, 3.2      |
| `:corresponds`                         | route    | the transition owner | pair            | decl       | SAME         | wSAME           | no       | 2.7           |
| `:observer-independence` of O          | warrant  | T's owner            | sort, O         | decl       | SAME         | wSAME           | no       | 2.8           |
| the local-route axiom                  | axiom    | the engine           | span            | —          | SAME         | wSAME           | no       | 1.5, 1.10     |
| `:lends`                               | route    | the wrapper's author | wrapper, sort   | decl       | SAME         | vantage         | no       | 3.4           |
| `:yields` with the supplied instance   | route    | the lookup's owner   | shape           | decl, eval | SAME · DISJ  | wSAME · wDISJ   | no · yes | 1.6, 2.1      |
| `:identified-in`                       | route    | T's owner            | shape           | decl       | SAME · DISJ  | wSAME · wDISJ   | no · yes | 1.6, 2.2, 3.2 |
| the completion sentinel                | sentinel | the wrapper's author | wrapper         | eval       | SAME         | wSAME           | yes      | 3.4           |
| `:guarantees-unique-name`              | warrant  | the lookup's owner   | shape           | decl, eval | DISJ         | wDISJ           | yes      | 1.5, 3.2      |
| `:aliases-nothing-else`                | warrant  | the store's describer | store          | decl       | DISJ         | wDISJ           | yes      | 2.3, 3.2      |
| `alias nothing-else`                   | closure  | the lookup           | key, level      | eval       | DISJ         | wDISJ           | yes      | 1.5, 2.9      |
| `looked-up-in nothing-else`            | closure  | G's placing lookup   | key, route-sort | eval       | DISJ · INVAL | wDISJ · stale   | yes · no | 2.10          |
| `may-read nothing-else`                | sentinel | T's owner            | sort            | decl       | SPARE        | wSPARE          | yes      | 2.5           |
| the vouch over the marked reads        | vouch    | the oracle's author  | body            | decl       | SPARE[^vouch] | wSPARE         | yes      | 2.5           |
| the completion record                  | record   | the verb's author    | path            | eval       | SPARE        | wSPARE          | yes      | 2.6           |
| the finished record                    | record   | the written sort's owner | sort, shape | decl, eval | SPARE        | wSPARE          | yes      | 2.6, 2.10     |
| the deep mark on an entry              | —        | the entry's author   | entry           | decl       | SPARE        | omitted: wSPARE | yes      | 2.9           |
| the traversal's closing act            | closure  | the lookup's owner   | lookup, shape   | eval       | INVAL · DISJ | stale · wDISJ   | no · yes | 1.7, 2.9      |
| a command's read-set closure           | closure  | the command's describer | command      | decl       | INVAL        | stale           | no       | 1.7, 3.3      |

[^vouch]:
    The vouch licenses the vouched line's own elision at the verdict tier, with no flag
    (`KNOBS:kCONTRACT-RUNGS`). Its closure of the readset feeds the sparing test, under
    the flag.

## § 6-pending-renames

This section is temporary and non-normative. Delete it when every entry is applied or dropped.

Renaming is in progress, at the same time as the model changes. Outside this section, one
concept may have at most two names anywhere in git: its name before r31, and its name in § 1 to
§ 5 of this document. A split or a merge is allowed. A third name for one concept is never
allowed. Thus every new write to a durable document uses the name that § 1 to § 5 use now. This
is true also where this section marks that name as about to change. A later single pass (grep,
then replace) catches those new uses together with the old ones. Names that are never to be
used are in `311u:names-never-to-use`.

This section says only which name to use when. It keeps no status and no history.

### § 6.1-applied-here-not-yet-applied-elsewhere

Use the name of § 1 to § 5. Never write the dead name in new text.

The application will likely be partial. Core, critical, and living documents (the root
documents, `plans/`, `spike/` code and its `AGENTS.md` files) get one commit that makes them
current: a mechanical replacement, or a Sonnet pass. Historical documents (`notes/`) will likely
keep the dead name. A slug is the exception: when a slug is renamed in a new or important
document, every reference to that slug is renamed everywhere, historical documents included.

The r31 ledgers are frozen and historical. They record many of these renames as they happened,
so they carry the old words unchanged. Non-ledger content will probably be updated.

| in the r31 ledgers | here |
|---|---|
| MKey-CatalogStore / MKey-PrimaryStore | MParent: one edge per MKey; derived views MParent-Catalog (through a secondary MScheme; routing) and MParent-Store (through the primary MScheme; identity) |
| MSort-CatalogStore / MSort-PrimaryStore | none: the MParent's MSort is declared per matched shape on the primary MScheme (`:identified-in`) |
| naming system (described, never named) | MScheme (owns the `resolve()`; what every bind and mark names) |
| `:named-in` | `:yields` (per matched shape, into any MSort) |
| `:identified-in` (one per MSort) | `:primary-of` on the MScheme, with `:identified-in` and the warrants per matched shape |
| MAspectSort, `:named-like` | none: a cell is a singleton MSort under its MParent (1.9-cell-a-singleton-sort) |
| MNaturalKey / MPrimaryKey | MKey-Natural / MKey-Primary (the ledgers and exercises keep the old order as an acceptable gloss) |

Plain-English "read" that must NOT be tagged or renamed (a host read, not the operation):
312b-exercises/01 ~line 54; 312b-exercises/02 ~lines 52 and 65.

| dead name | name here | where the dead name remains |
|---|---|---|
| MPlacement · `:lives-in` · "placement"; and "backing" (`an-backing-selfframing`, the probe's marked read set): the declared and the measured halves of one side | `may-read` (the record verb / relation); Readset (the set: the declared `may-read` entries, the probe's marked reads, and the Readsets of every container on the chain). Sentinel `may-read nothing-else` | `ANALYZER-NEEDS` rows, including the slug `ANALYZER-NEEDS:an-backing-selfframing` (also cited from `plans/30T`); `USER_STORY`; `KNOBS` |
| footprint · `disturbs` · "at-most claim"; and `:reaches` / `disturbance_reaches` (the entailment, computed into the same set) | `may-write` (the record verb / relation); Writeset (the set: the declared `may-write` entries, the entailment, and the may-write sets of the written thing's containers strictly below the level shared with the fact's key). Sentinel `may-write nothing-else` (the finished definition / completion record) | `USER_STORY` stages 5–7; `KNOBS:kBURDEN`; `ANALYZER-NEEDS`; `FORFEITS`; the spike members `cmd__disturbs()` and `kind__disturbance_reaches()`, which follow the verb |
| "perishing" and its forms, used as jargon | no term: the jargon retires. Plain English and standard compiler-engineering terminology, usually but not always a phrase with the word "invalidation". Never a capitalised or tagged form (no "Invalidation", no MInvalidation). Where it becomes "invalidation", it nearly always needs a precise subject; for how "perishing" was used, the human believes that is probably "routing invalidation", unchecked | not yet investigated |
| `reaches`, the world relation from an MKey to an MReferent | MRefers to (one instance is an MReference); the fence field is `mRefersTo` | the r31 ledgers from `312d` on |
| `unrelated` | KNOWN_UNSPOKEN | `30U` § 7, `compare-consumer-map`, `ANALYZER-NEEDS:an-compare-chokepoint`. The design-of-record documents keep `unrelated` until the model is ruled (4.2-supersessions-pending-in-prior-documents names them) |
| "truth predicate" (and the identifiers `true_<Species>`) | no term: written "the `isTrue` predicates" or plain local prose; the identifier is `isTrue` (cited elsewhere as `<Kind>.isTrue`) | the r31 ledgers from `312d` on, and one translation line of § 3.2 until its rewrite |

### § 6.2-undecided-whether-a-name-is-dead

§ 1 to § 5 still use these words. The table in 6.1-applied-here-not-yet-applied-elsewhere lists
each as a dead name. Whether that retirement covers these uses is undecided.

- "backing" for a MResolution's MTraversal (1.7-resolution-and-its-traversal, and the consumer
  line of 2.9-the-traversal-and-the-region-test), and "backing sets" in
  3.2-compare-one-chokepoint-four-answers. The dead "backing" is the probe's marked read set.
- "at-most claim" for the MKeys that a verb's author declares it writes
  (3.3-invalidation-three-mutator-species, 3.4-entry-and-lends,
  4.2-supersessions-pending-in-prior-documents). If it is a name, it is a third name for the
  may-write concept. If it only describes the strength of the claim, it stays.

### § 6.3-open-names

Nothing here is applied anywhere. Do not write a candidate. Where § 1 to § 5 hold a provisional
name, use that name.

| concept | candidates / notes |
|---|---|
| the route row's spellings (all owed; the relation is not new) | (a) a sort's declaration that it places another sort (strawman `: : places "sm.File"` on the placing sort's declaration member); (b) the upward body on the placing sort, invoked with a placed key's value, its arms deciding which spellings it consumes (strawman `sm_Package__member_of()`); (c) the downward members body (today only as write reach, USER_STORY stage 7's `dpkg -L : disturbs sm.File`; as membership, a member owed or that body reused); the record both ends feed is `looked-up-in P:key` with a closure scoped per route-sort, which exists. The relation name `:places` is provisional here; its final name is 312's |
| the two ends of the alias row | `:guarantees-unique-name` declared per shape at the store's end; `alias nothing-else` emitted per key at the thing's end: two RELATED BUT SEPARATE statements here. Whether licensing needs both ends in agreement, either end, or a lattice, and what "a warrant" is, are 312's. "One warrant, two spellings" is retracted |
| a sort's key that some store hands out names for | "individual": reads well (human); "sort" and "scheme-for-naming-a-sort" read well alongside it |
| `sm.Service:nginx@active` | the `@` coordinate as facet syntax, expanding to a facet-sort key formed from the parent's resolved primary key (`311q` § 8–§ 9); sugar and defaults are 312 material |
| the THING'S-end twin of `:aliases-nothing-else` (this thing has no other parent) | human: `:sole-parent` (beside `:sole` · `:sole-view` for the store's end, now spent) · conductor: the sentinel of `:identified-in` (`identified-in nothing-else`) · warrant form: `:guarantees-unique-parent` · verb menu considered for the store's end `:<verb>-nothing-else`: views · re-keys · aliases (taken) · re-exports (collides with sh `export`) · fronts · factors-through. Human: many names churn if the table shape of `311t` § 11 is taken, the "guarantees" being better spelled as closure statements |
| the identifying-store object (the parent named by `:identified-in`) | "store" is provisional; this model writes MParent-Store. One word is owed; whether MParent-Store discharges it is unacked |
| the spike's `kind__` API prefix | follows MSort, or the MScheme as the exercise strawmen have it (`sm_Path__resolve()`); untouched in code and pre-311 documents; strawman-tier |
| `:root` | its name is unacked; where it sits is undecided (`311p` thread 7) |
