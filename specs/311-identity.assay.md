# 311 — Identity and relation: the specification

The identity model, mechanized. AI-authored (Fable, the `r31-prep-design-duck` sittings and the
`312d` mechanization arc, the human present and adjudicating). Specification-tier and
ahistorical; nothing here is ruled. The root docs, `spike/CLAUDE.md`, and the welds outrank this
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
what runs them is `notes/30Y`; the arc's ledger is `notes/312d`.

## Conventions

- A model object is written mFixedTerm. The bare word never stands in for it.
- A relation, attribute, or warrant is written `:fixed-term`.
- An abstract operation is written `op()`. A concrete authored member keeps the `__name()` form.
- A derived view of a species is written species-hyphen-gloss: mKey-Primary, mParent-Catalog,
  mParent-Store. It is never a declared species.
- "Cell" stays untagged. It means a singleton mSort under its mParent
  (1.9-cell-a-singleton-sort).
- "Lookup" stays untagged. It means the relation between an mKey and what it reaches.
- An mKey's bytes are its mValue. An mReferent's condition is its mState
  (1.1-referent-state-and-value).
- An in-Dorc mSort or mScheme is always written with its prefix: `sm.File`, `sm.Path`.
- The four answers of `compare()` are written SAME, DISJOINT, KNOWN_UNSPOKEN, and UNKNOWN
  (3.2-compare-one-chokepoint-four-answers).
- A section is cited by its slug, as 2.6-may-write-the-writeset. Within the same paragraph or
  list item, a second citation of that section uses its number alone, as §2.6.
- A blockquote is one of the two headed forms of `plans/30Z` and nothing else; a false friend,
  a prior name, a refuted shape (`notes/311u`), or an example is plain prose.
- § 5 is non-normative in its entirety (5-the-relations-indexed-two-ways). It indexes § 1 to
  § 3 and defines nothing.

Spelling inside the fences:

- A model object keeps its prose name as an Alloy signature: `mKey`, `mSort`. A shell word is
  assay's `Shword`; an mKey's mValue is one.
- A statement species is a signature under the shared `Statement`, named for the act:
  `DeclaresIdentifiedIn`, `SuppliesParent`. Its fields name what it is about. One statement
  atom in a scenario is `speaker__what_it_says`.
- Every species has one truth predicate, `true_<Species>[s]`, which transcribes the sentence
  that says what the statement means when true. The engine's own definitions never read the
  world stratum; the truth predicates alone do.
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

No single author knows the whole path from a tool's argument to a measurable mReferent.
Viewpoint transitions, authored by yet other people, change which mReferent an mKey reaches
mid-book. What the engine can and cannot hold is 0.1-the-two-strata.

Two of the law's four sentences are the checks of 3.2-compare-one-chokepoint-four-answers:
where nobody has spoken the walk declines, and the walk never reaches a false SAME or DISJOINT
while every statement in force is true. Who must say what is 3.5-committee-law-and-attribution.
The other two sentences are the law's own terms of refutation, and stay prose:

<!-- normative -->
> The true answer is reachable once those who can know have spoken.
> A wrong answer with no false statement behind it refutes the model.

### § 0.1-the-two-strata

The engine and the world are two strata, and the fences keep them apart: the world stratum (an
mReferent, what an mKey reaches, what a store holds) is what the truth predicates read; the
engine's definitions read only mKeys, their declared shapes, and the statements in force.
Nothing in the fences enforces the separation; the two sentences below are the rule, and a
reviewer reads the engine's definitions for a world relation by eye.

<!-- normative -->
> The engine knows only syntax, authored speech, and what authored probes returned.
> It never decodes an mKey and never holds an mReferent.
> The engine never holds an mState, and no mKey names one.

### § 0.2-a-world-exists

The consistency probes of `plans/30Z` § 2.4: a facts-only contradiction makes every check
green, so these runs must stay satisfiable for the life of the document. The second asks for the
strongest ordinary world, two mKeys scoped in one mParent-Store with a known chain each.

```alloy
run bookScope {} for 6 but 4 Int

run world_exists {
   some k: mKey | knownChain[k] and some k.reaches
} for 6 but 4 Int expect 1

run chains_meet_in_a_store {
   some disj a, b: mKey | isPrimaryKey[a] and isPrimaryKey[b] and no (a + b).cellSort
      and knownChain[a] and knownChain[b] and a.parent = b.parent and a.parent in mKey
} for 6 but 4 Int expect 1
```

<!-- prose-translation -->
> A book's ceiling in this document is six atoms of every kind the specification owns, and integers of four bits.
> Some mKey has a known mFullyQualifiedKey and reaches an mReferent.
> Two mKeys can share one mParent-Store, each with a known mFullyQualifiedKey.

## § 1-the-model-objects

### § 1.1-referent-state-and-value

The world stratum of 0.1-the-two-strata: an mReferent, what a store holds, and what it holds by
its own construction against what it merely re-presents (the distinction 2.3-aliases-nothing-else-the-store-warrant
needs). The engine never reads these relations; the truth predicates do.

```alloy
sig mReferent { holds: set mReferent, owns: set mReferent, affects: set mReferent, passes: set mReferent }

fact { owns in holds }

fact { all r: mReferent | some reaches.r }

fun sortsOf[r: mReferent]: set mSort { primaryOf[(reaches.r).scheme] }

fun partsOf[r: mReferent]: set mReferent { r.holds }
```

<!-- prose-translation -->
> An mReferent is a persisting piece of the world.
> A store is an mReferent; what a store holds is identified in it; what a store owns it holds by its own construction.
> A write to an mReferent affects the mState of the mReferents it affects (2.5-may-read-the-readset).
> A route to an mReferent passes through the mReferents that pass to it; what an mReferent passes to is what is reached beneath it (1.7-resolution-and-its-traversal, 2.9-the-traversal-and-the-region-test).
> An mReferent has the mSort of the mKey that reaches it, and two mSorts over one piece of the world is the strangers case (1.2-sort-the-declared-carrier).
> It has one or more mKeys.
> It may have parts, and every part is an ordinary mReferent of an ordinary mSort identified in it (1.9-cell-a-singleton-sort).
> It is not an mKey, and not an mSort.

UNACKED READING, temporary (`312d:ask-aliases-nothing-else-world-reading`,
`312d:ask-world-relations-for-effects`): the world relations `holds`, `owns`, `affects`, and
`passes` are the conductor's choice of world stratum, made so that the truth predicates of § 2
have something to transcribe into; 311 names none of them. In particular `owns` against `holds`
is one reading of § 2.3's "by the store's own construction". None of this is authoritative or
acked; it stands only until the document has run under Alloy and the reading is acked or
replaced, and it is not a pattern to extend (`notes/312d` § 7).

Every identity question is ultimately "do these two mKeys reach one mReferent"; the mKey's
side of that question is 1.4-key-and-its-two-views. No other term names a part. An mReferent is
not an mTopic (1.8-fully-qualified-key-topic-and-derivation), and not the set an mKey given
whole denotes: the container is the mReferent, and the set is a set
(2.9-the-traversal-and-the-region-test).

Examples of an mReferent: an inode, a database row, a package record, a kernel parameter, a running process, a machine, a mount table.

#### § 1.1.1-state-and-value

State and time are not in the fences: no fence holds an mState, and the sentences that say what
an mReferent survives are read by the invalidation rules of 3.3-invalidation-three-mutator-species,
which withdraw authority and never compute a successor. The value plane (`notes/275`) is outside
this model. What is normative here is normative as prose.

<!-- normative -->
> An mReferent survives changes to its mState.
> It does not survive destruction and recreation under its old mKey.
> It does not survive a lifecycle write to what it is scoped in (3.3-invalidation-three-mutator-species).
> Before a lookup binds an mKey, that mKey names a may-set of mReferents (1.5-token-and-the-two-warrants).
> An mState is the condition of one mReferent at one instant: what a write changes and what a read observes.
> An mState is known only through a read, which yields an mValue.
> Two mReferents may have equal mStates and stay two.
> One mReferent's mState changes and it stays one.
> An mValue is bytes a shell holds or will hold, with an exit status where one is produced.
> There are three kinds of mValue: an mKey's mValue, bytes that name an mReferent, bound at a bind; a captured mValue, bytes a read copied out of an mState, graded by provenance (`notes/275`); and a verdict's exit status.
> An mValue is what a read yields from an mState, and it is never the mState.
> Two reads of one mReferent at two instants may yield two mValues.

### § 1.2-sort-the-declared-carrier

Many-sorted logic's carrier. Never a PLT kind. Never "the kind of thing". Pre-311 documents write _kind_.

A carrier in the logician's sense, a domain of discourse someone chose to speak in. It has
reverse-DNS naming, no registry, and owner-adjudication as its social contract. An mSort fixes
only what its owner declares under it, each a statement species of its own section: which
mScheme, if any, is its primary mScheme (2.2-primary-of-and-identified-in); its may-read set
(2.5-may-read-the-readset); its may-write entailment and finished definition
(2.6-may-write-the-writeset); its `:observer-dependence`
(2.8-observer-dependence-and-independence); its cells (1.9-cell-a-singleton-sort). Each of
those species carries the fact that its speaker is the mSort's owner. An mSort has no mKeys and
no `resolve()`: an mKey names an mScheme (1.4-key-and-its-two-views), and the floor of
1.3-scheme-a-way-of-writing lets an mScheme precede its mSort's name. An mSort is named only
when something is declared about it as a whole; naming is not in the fences.

```alloy
sig mSort { sortOwner: one Speaker }
```

<!-- prose-translation -->
> An mSort is one owner's declared vocabulary.

#### § 1.2.1-the-strangers-case

Several mSchemes into one mSort is the cooperative case: one owner admits several ways of
writing down what they describe. Several mSorts over one world-thing is the ordinary strangers
case. The sentences below are the laws of 3.2-compare-one-chokepoint-four-answers seen from the
mSort's side; they stay prose until that section's checks carry them.

<!-- normative -->
> An mSort is not a category of the world.
> The engine never knows what an mSort denotes.
> It never assumes two mSorts denote disjoint mReferents.
> The strangers case is undetectable to the engine.
> It reads KNOWN_UNSPOKEN at the chokepoint (3.2-compare-one-chokepoint-four-answers).
> Only a human act merges it, by making one mSort's mSchemes yield into the other's.
> The owner speaks only about the mSort's relations to its immediate neighbours.

The strangers case: two vendors describe one tool, or two vocabularies reach one cell under `/proc/sys`.

### § 1.3-scheme-a-way-of-writing

A term language over a carrier. Never itself an mSort.

An mScheme fixes its `resolve()` (3.1-identity-of-a-key), whether it is `:primary-of` an mSort
with its declarations per matched shape (2.2-primary-of-and-identified-in), what it `:yields`
per matched shape and where its mKeys are looked up when it is secondary
(2.1-yields-into-another-scheme), and its lookup warrants (1.5-token-and-the-two-warrants).
Nothing else constrains where an mScheme yields. A shape is where every per-shape declaration
hangs, so it is declared here; what a shape is, a control-flow path of the owner's body, is
1.5-token-and-the-two-warrants's. The floor's "no warrants" is a fact of that section, and its
"identity `resolve()`" and "the mRoute as its only mParent" follow from 3.1-identity-of-a-key
and 1.6-parent-one-per-key. An mSort with no mScheme at all has singleton mKeys under mParents
(1.9-cell-a-singleton-sort), and a mark of the form `parent-key@sm.Sort` is that section's.

```alloy
sig mScheme { schemeOwner: one Speaker }

sig mShape { ofScheme: one mScheme }

sig DeclaresPrimaryOf extends Statement { primaryScheme: one mScheme, ofSort: one mSort }

fact { all d: DeclaresPrimaryOf | d.speaker = d.ofSort.sortOwner }

fun primaryOf[s: mScheme]: lone mSort { (DeclaresPrimaryOf & InForce & primaryScheme.s).ofSort }

fact { all k: mSort | lone (DeclaresPrimaryOf & InForce & ofSort.k).primaryScheme }

fact { all s: mScheme | lone primaryOf[s] }

pred floor[s: mScheme] {
   no primaryOf[s]
   no (DeclaresYields & InForce).fromShape & ofScheme.s
}

pred isPrimary[s: mScheme] { some primaryOf[s] or floor[s] }

fact { all s: mShape | floor[s.ofScheme] implies no identifiedIn[s] and not isRoot[s] }

fact {
   all d: DeclaresPrimaryOf & InForce |
      some s: ofScheme.(d.primaryScheme) | some identifiedIn[s] or isRoot[s]
}

pred true_DeclaresPrimaryOf[d: DeclaresPrimaryOf] {}
```

<!-- prose-translation -->
> An mScheme is a way of writing down which mReferent is meant, with one accountable owner.
> Every shape belongs to one mScheme.
> `:primary-of` is declared by the mSort's owner, on the primary mScheme (2.2-primary-of-and-identified-in).
> An mSort has at most one primary mScheme.
> An mScheme is `:primary-of` at most one mSort.
> The floor: an mScheme that declares neither `:primary-of` nor `:yields` is the primary mScheme of an mSort nobody has named, and no shape of it carries `:identified-in` or `:root`.
> Where an mSort has a primary mScheme, that mScheme yields into the mSort for at least one shape: some shape of it carries `:identified-in` or `:root`.
> A `:primary-of` declaration claims nothing about the world.

UNACKED READING, temporary (`312d:enc-primary-yields-into-its-sort`): 311 says a primary mScheme
"yields into the mSort for at least one shape"; the fence reads that as "some shape of it
carries `:identified-in` or `:root`". The conductor's reading, not acked, not authoritative,
held only until acked or replaced (`notes/312d` § 7).

### § 1.4-key-and-its-two-views

RDBMS primary key and natural key, with their culture: the natural key is user-typed, may alias, and is never identity. The primary key is what the store answers with. Pre-311 documents write _entity_.

An mKey is a plan-time object that models what a runtime string will denote. It is minted at a
bind, or at an emission point a `resolve()` declares (2.1-yields-into-another-scheme), before
any lookup runs. Every lookup is a measurement: what it decides late is which declared shape an
mValue matches, so which mSort an mKey reaches, and which mParent mSort and warrants apply, is
known only once bytes arrive. The mPlaceholder, for an mValue or an mParent not yet measured, is
1.10-vantage-route-placeholder-witness's. An mLevel is what a mFullyQualifiedKey passes through
(1.8-fully-qualified-key-topic-and-derivation): an mKey, or one of the world atoms that end a
chain. The `reaches` field is world stratum (0.1-the-two-strata) although it sits on the mKey;
no engine definition reads it. The two views coincide when an mSort's only mScheme is its primary
mScheme, and the primary view is where the dangerous warrants can honestly sit.

```alloy
abstract sig mLevel {}

sig mKey extends mLevel {
   value: one Shword,
   scheme: lone mScheme,
   cellSort: lone mSort,
   shape: lone mShape,
   parent: lone mLevel,
   yielded: lone mKey,
   at: one mVantage,
   reaches: lone mReferent
}

fact { all k: mKey | some k.scheme iff no k.cellSort }

fact { all k: mKey | some k.cellSort implies no k.shape }

fact { all k: mKey | k.shape.ofScheme in k.scheme }

fact { all a, b: mKey | a.scheme = b.scheme and a.value = b.value implies a.shape = b.shape }

fact { all k: mKey | no yieldsTo[k.shape] implies no k.yielded }

fact { all k: mKey | some k.yielded implies k.yielded.scheme = yieldsTo[k.shape] }

fact { no k: mKey | k in k.^yielded }

fun keysOfShape[s: mShape]: set mKey { shape.s }

fun keysOfSort[k: mSort]: set mKey { {x: mKey | primaryOf[x.scheme] = k or x.cellSort = k} }

pred isPrimaryKey[k: mKey] { (isPrimary[k.scheme] and no yieldsTo[k.shape]) or some k.cellSort }

pred isNaturalKey[k: mKey] { not isPrimaryKey[k] }
```

<!-- prose-translation -->
> An mKey has three parts: an mValue, its mScheme, and its mParent (1.6-parent-one-per-key).
> The mScheme is always declared, except that a cell's mKey has its cell mSort in place of an mScheme and matches no shape (1.9-cell-a-singleton-sort); there is no default mScheme, and a bind or a mark always names one.
> The mValue is a literal: the shell word bound at the bind.
> The mParent is the instance one of the seats of 1.6-parent-one-per-key supplies, or none, which leaves the mFullyQualifiedKey unknown from that level.
> A lookup chooses among the shapes its mScheme declares, never outside them: the shape an mKey matches is a shape of its own mScheme.
> Which shape an mValue matches is a function of the mKey's own bytes: two mKeys of one mScheme with equal mValues match one shape (1.6-parent-one-per-key).
> An mKey may carry the mKey its lookup emitted for it, minted at the emission point the `resolve()` declares: an mKey of the mScheme its shape `:yields` (2.1-yields-into-another-scheme), and none where the shape yields nothing.
> Every mKey was resolved from one mVantage (1.10-vantage-route-placeholder-witness).
> No mKey is its own yield, directly or through others.
> An mKey reaches one mReferent, or none (2.2-primary-of-and-identified-in).
> mKey-Primary is an mKey of the primary mScheme, on a shape that yields nothing, meaningful only relative to its mParent-Store; mKey-Natural is any other mKey, what tool authors and books write.

### § 1.5-token-and-the-two-warrants

OWL's inverse-functional and functional properties, spelled out by direction.

What each warrant licenses, SAME from equality and DISJOINT from inequality, is the walk of
3.2-compare-one-chokepoint-four-answers; here is what each means when true. "Within one
mParent" is read at the world stratum: two mParents that reach one mReferent, or one mWorld.
The instance of a warrant is per evaluation, and it may rest on what the path measured: since a
matched shape is a control-flow path, an evaluation that declines the warrant has matched a
shape that does not carry it.

```alloy
fun token[k: mKey]: lone Shword { isPrimaryKey[k] implies k.value else none }

sig DeclaresUniqueReferent extends Statement { referentShape: one mShape }

sig DeclaresUniqueName extends Statement { nameShape: one mShape }

fact { all d: DeclaresUniqueReferent | d.speaker = d.referentShape.ofScheme.schemeOwner }

fact { all d: DeclaresUniqueName | d.speaker = d.nameShape.ofScheme.schemeOwner }

pred guaranteesUniqueReferent[s: mShape] { some DeclaresUniqueReferent & InForce & referentShape.s }

pred guaranteesUniqueName[s: mShape] { some DeclaresUniqueName & InForce & nameShape.s }

fact {
   all s: mShape | floor[s.ofScheme] implies
      not guaranteesUniqueReferent[s] and not guaranteesUniqueName[s]
}

pred withinOneParent[a, b: mKey] {
   some a.parent & b.parent
   or (a.parent + b.parent in mKey and some a.parent.reaches and a.parent.reaches = b.parent.reaches)
}

pred true_DeclaresUniqueReferent[d: DeclaresUniqueReferent] {
   all a, b: keysOfShape[d.referentShape] |
      withinOneParent[a, b] and a.value = b.value implies a.reaches = b.reaches
}

pred true_DeclaresUniqueName[d: DeclaresUniqueName] {
   all a: keysOfShape[d.nameShape], b: mKey |
      b.scheme = a.scheme and withinOneParent[a, b] and some a.reaches and a.reaches = b.reaches
         implies a.value = b.value
}
```

<!-- prose-translation -->
> A mToken is an mKey-Primary's mValue: bytes a `resolve()` returned.
> Any lookup, a secondary mScheme's or a primary mScheme's, may carry two warrants per matched shape; they are independent, separately declared, and absent by default.
> The lookup's owner declares each warrant per matched shape, and the floor of 1.3-scheme-a-way-of-writing carries neither.
> A warrant holds for every mKey of that shape inside any one mParent; a path that does not reach a warrant has not given it, so an mKey has a warrant when the shape it matched carries it.
> Two mKeys are within one mParent when they have one mParent, or when their mParents reach one mReferent.
> `:guarantees-unique-referent` is true when, within one mParent, equal mKeys of the shape reach one mReferent: the lookup is a function.
> `:guarantees-unique-name` is true when, within one mParent, one mReferent reached by an mKey of the shape has one mKey of the lookup.

UNACKED READING, temporary (`312d:ask-unique-name-ranges-over-the-scheme`): 311's "one mReferent
has one mKey" does not say whether the one mKey ranges over the warranted shape or over the
whole lookup; the fence takes the lookup, the reading under which the two-tops way of § 3.2
is sound, and "within one mParent" is read at the world stratum as mParents that reach one
mReferent. The conductor's readings, not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).

#### § 1.5.1-the-token-over-time-and-the-per-level-closure

A mToken is compared for equality only and never decoded (0.1-the-two-strata), and it is always
scoped in an mParent (1.6-parent-one-per-key). How long a warrant holds is
3.3-invalidation-three-mutator-species's; the engine's own vouch is
1.10-vantage-route-placeholder-witness's; the per-level closure is consumed by
2.9-the-traversal-and-the-region-test, and stays prose until that section is mechanized.

<!-- normative -->
> A warrant holds while that mKey's mResolution or mToken stands (3.3-invalidation-three-mutator-species).
> The engine vouches for one lookup itself: the local mRoute under no wrapper (1.10-vantage-route-placeholder-witness).
> Across a wrapper, the wrapper's author speaks (3.4-entry-and-lends), and everything else is measured and witnessed.
> Separately from the per-shape warrant, a lookup may emit a closure, `alias nothing-else`, for each level it resolved.
> The closure states that the mReferent at that level is reachable by exactly this one entry anywhere in the instance the lookup ran in, not only in the mParent-Catalog the entry was found in.
> It is a statement about one mKey, from the thing's end, made on the path that measured it.
> Where the lookup knows other entries, it emits them first, and a listed alias is checked as the first entry is.
> The closure and the per-shape warrant are two statements; the region test (2.9-the-traversal-and-the-region-test) consumes the closure.
> A grade governs every consumer of the answer it grades, corroboration and contradiction included.
> A lookup without `:guarantees-unique-name` cannot contradict anything by returning two different mTokens.

`:guarantees-unique-referent` fails for round-robin lookups, recycled mKeys, and cloned identifiers presented as mRoots. `:guarantees-unique-name` fails for symlinks and hardlinks, package `provides`, and route-qualified handles. For a path the closure's instance is the whole mount namespace. For a file, the evidence is a link count of one. For a directory, the evidence is that no other mount exposes it. A hardlink or a bind mount is where the closure is withheld.

### § 1.6-parent-one-per-key

What the mParent edge carries follows the mScheme (2.4-parent-as-a-relation): routing through a
secondary mScheme, identity through the primary one, and the walk of
3.2-compare-one-chokepoint-four-answers reads the identity edge. The declarations that fix the
mParent's mSort per shape are 2.2-primary-of-and-identified-in's; that a shape is a function of
the mKey's own bytes is 1.4-key-and-its-two-views's. The supplying seats, three of them, and
what a disagreement between them costs, are 1.6.1-the-three-seats. Nothing is a mRoot by
default, and cloned identifiers are the standing witness.

```alloy
abstract sig Seat {}

one sig BindSeat, YieldSeat, DeclarationSeat, EntryChainSeat extends Seat {}

sig SuppliesParent extends Statement { forKey: one mKey, instance: one mKey, seat: one Seat }

fact { all s: SuppliesParent | s.seat = YieldSeat implies s.speaker = (yielded.(s.forKey)).scheme.schemeOwner }

fact { all s: SuppliesParent | s.seat = DeclarationSeat implies s.speaker = s.forKey.scheme.schemeOwner }

fun supplies[k: mKey]: set mKey { (SuppliesParent & InForce & forKey.k).instance }

pred parentRefused[k: mKey] { some disj p, q: supplies[k] | p != q }

pred supplyFits[k: mKey] {
   one supplies[k]
   some identifiedIn[k.shape] implies primaryOf[supplies[k].scheme] = identifiedIn[k.shape]
   some k.cellSort implies sortOfKey[supplies[k]] = cellParentSort[k.cellSort]
}

fact {
   all k: mKey {
      (no k.shape and no k.cellSort) implies no k.parent
      some k.cellSort implies k.parent = (supplyFits[k] implies supplies[k] else none)
      isRoot[k.shape] implies k.parent = rootShape.(k.shape)
      (some k.shape and not isRoot[k.shape] and no identifiedIn[k.shape] and no yieldsTo[k.shape])
         implies k.parent = k.at.route
      some identifiedIn[k.shape]
         implies k.parent = (supplyFits[k] implies supplies[k] else none)
      some yieldsTo[k.shape]
         implies k.parent = (some supplies[k] implies (supplyFits[k] implies supplies[k] else none)
                             else k.at.ambient[catalogSortOf[k.scheme]])
   }
}

pred true_SuppliesParent[s: SuppliesParent] {
   isPrimaryKey[s.forKey] implies s.forKey.reaches in s.instance.reaches.holds
}
```

<!-- prose-translation -->
> Every mKey has at most one mParent: the mKey it was resolved inside, or the mWorld its chain ends at (1.8-fully-qualified-key-topic-and-derivation).
> An mKey matching no shape has no mParent.
> A cell's mKey has the mParent its mark supplied, an mKey of the mSort the cell is `:identified-in` (1.9-cell-a-singleton-sort).
> A shape declared `:root` is scoped in its own mWorld (2.2-primary-of-and-identified-in).
> A shape with neither `:identified-in` nor `:yields` is scoped in the mRoute of the mKey's mVantage (1.10-vantage-route-placeholder-witness).
> For a shape that yields, where no bind and no declaration supplied the mParent-Catalog instance, the mVantage supplies its ambient instance for the mScheme's catalog mSort (2.1-yields-into-another-scheme), or none where it holds none.
> The mParent instance is an mValue supplied by exactly one of three seats: the bind that minted the mKey (1.4-key-and-its-two-views); the lookup that yielded it (2.1-yields-into-another-scheme); the primary mScheme's declaration for the matched shape (2.2-primary-of-and-identified-in); for a secondary mScheme's mKey the third seat is the mEntryChain's instance (2.1-yields-into-another-scheme).
> A supply from the yield seat is the yielding lookup's owner's line; a supply from the declaration seat is the primary mScheme's owner's line.
> For a shape with `:identified-in`, the seat names the instance as an mKey of the mParent's mSort: an mKey of the primary mScheme of the mSort declared for the shape.
> Two seats that disagree are a contradiction.
> Where no seat supplied an instance, where two seats disagree, or where the supplied mKey is not of the declared mSort, the mKey has no mParent and its mFullyQualifiedKey is unknown from that level.
> A supplied mParent-Store instance is true when the mReferent the mKey-Primary reaches is held by the mReferent the instance reaches.

UNACKED READING, temporary (`312d:ask-route-for-any-path-without-identified-in`,
`312d:ask-mismatched-supply-reads-unknown`): the fence scopes every shape with neither
`:identified-in` nor `:yields` in the mRoute, warranted or not, on § 1.6's sentence alone; and it
gives a mKey whose supplied instance is not of the declared mSort no mParent (unknown from that
level), where 311 says only that the seat "names it as an mKey of one of the mParent's mSort's
mSchemes" and says nothing about a mismatch. The conductor's readings, not acked, not
authoritative, held only until acked or replaced (`notes/312d` § 7).

#### § 1.6.1-what-a-seat-answers-for

The refusal's attribution is 3.5-committee-law-and-attribution's; the routes that are not the
mParent are 2.10-places-the-upward-lookup's.

<!-- normative -->
> A disagreement between seats is refused and attributed to both.
> An emitter (a secondary mScheme, possibly a stranger's) can be wrong only about what it supplied: its lookup, and the mParent instance where it is the seat that supplied it.
> An mKey may carry further routes, one per other lookup that reached it (2.10-places-the-upward-lookup); none of them is its mParent.

Examples of a mParent-Catalog: a directory for a path's entry, a passwd database for a login name, a process table for a pid. Examples of a mParent-Store: a DNS zone for a record's owner name, a user namespace for a uid, a dpkg database for a canonical package name. Examples of a mRoot: the DNS mRoot, or a cloud instance-id whose issuer never repeats one.

### § 1.7-resolution-and-its-traversal

A mResolution is the fact that mKey N of mScheme S, resolved inside mParent P at program point
p, reaches mReferent R: in the fences it is the mKey's emitted mKey (1.4-key-and-its-two-views)
with the mTraversal beside it. The order the lookup crossed the members in is not held; the
consumers read membership. What touches a member and so invalidates the mResolution is
3.3-invalidation-three-mutator-species's; the read set of the lookup body is
1.7.1-the-lookup-body-and-its-reads.

```alloy
sig EmitsCrossed extends Statement { crossedFor: one mKey, crossedKey: one mKey }

sig ClosesTraversal extends Statement { closedFor: one mKey }

fact { all d: EmitsCrossed | d.speaker = d.crossedFor.scheme.schemeOwner }

fact { all d: ClosesTraversal | d.speaker = d.closedFor.scheme.schemeOwner }

fun crossed[k: mKey]: set mKey { (EmitsCrossed & InForce & crossedFor.k).crossedKey }

pred traversalClosed[k: mKey] { some ClosesTraversal & InForce & closedFor.k }

fun traversal[k: mKey]: set mLevel { crossed[k] + (traversalClosed[k] implies none else k.parent) }

pred true_EmitsCrossed[d: EmitsCrossed] {}

pred true_ClosesTraversal[d: ClosesTraversal] {
   let k = d.closedFor | passes.(k.reaches) in crossed[k].reaches
}
```

<!-- prose-translation -->
> A mResolution is a fact with a backing, and the backing is the mTraversal: the chain of routing mKeys the lookup crossed.
> The lookup emits that chain, one member per routing mKey (2.9-the-traversal-and-the-region-test), as the lookup owner's speech.
> The emission is an at-most set, and an emitted member licenses nothing alone.
> The lookup's owner closes it by an explicit act; the closing act is true when every mReferent the route to the mKey's mReferent in fact passes through is one an emitted member reaches.
> A lookup that emits no member has its mParent-Catalog, given whole, as its mTraversal.
> A lookup that emits members without the closing act has those members and its mParent-Catalog, given whole.

#### § 1.7.1-the-lookup-body-and-its-reads

The engine derives a lookup body's read set from the body, which the fences do not hold: shell
parity supplies the reads of sh constructs, the path mScheme the reads of a path, and the speech
that describes an external command the reads of that command.

<!-- normative -->
> Under ordinary effective-mWorld reach, any mutator whose writeset touches a mTraversal member invalidates the mResolution.
> Its target object is not its backing: an mKey can stop reaching an object without the object changing, and an object can change without its mKey changing.
> A mResolution also depends on the read set of the lookup body that produced it.
> The read set is closed only when the read set of every external command in the body is closed.
> The author of the speech that describes an external command closes that command's read set by an explicit act.
> Any write invalidates a mResolution whose read set is open (3.3-invalidation-three-mutator-species).

Examples of a mTraversal: each directory entry and symlink for a path, the resolver configuration and mVantage for a hostname, the unit table for a service name.

### § 1.8-fully-qualified-key-topic-and-derivation

The chain is over mKey-Primaries: `identity()` (3.1-identity-of-a-key) resolves a natural mKey
to the primary one first. The world atoms that end a chain are declared here; the mRoute is the
address of 1.10-vantage-route-placeholder-witness, and a mRoot shape's mWorld is the store of
2.2-primary-of-and-identified-in. A level's height is what the walk of
3.2-compare-one-chokepoint-four-answers aligns two chains by.

```alloy
abstract sig mWorld extends mLevel {}

sig mRoute extends mWorld {}

sig mRootWorld extends mWorld { rootShape: one mShape }

fact { all w: mRootWorld | isRoot[w.rootShape] }

fact { all s: mShape | isRoot[s] implies one rootShape.s }

fact { no l: mLevel | l in l.^parent }

fun fullyQualifiedKey[k: mKey]: set mLevel { k.*parent }

fun terminus[k: mKey]: lone mLevel { {l: k.*parent | no l.parent} }

pred knownChain[k: mKey] { terminus[k] in mWorld }

fun worldOf[k: mKey]: lone mWorld { terminus[k] & mWorld }

fun height[l: mLevel]: Int { #(l.^parent) }
```

<!-- prose-translation -->
> A mFullyQualifiedKey is the recursive identity of an mKey: the mKey scoped in its mParent, whose identity is itself a mFullyQualifiedKey, up through the mParents.
> The recursion terminates: no level is above itself.
> It terminates at a mRoot, at the mRoute (1.10-vantage-route-placeholder-witness), or at an unknown link, an mKey with no mParent.
> An mWorld is a terminus of a mFullyQualifiedKey: each mRoot shape is one mWorld, and each mRoute is one mWorld.
> A mFullyQualifiedKey is known when it terminates at an mWorld.
> The height of a level is the number of levels above it.

#### § 1.8.1-derivations-and-the-topic

A mFullyQualifiedKey is one mDerivation of identity; the others, and what a claim is about,
are 2.7-corresponds-across-a-transition's and 2.8-observer-dependence-and-independence's, and
their combination is 3.2-compare-one-chokepoint-four-answers's.

<!-- normative -->
> No mFullyQualifiedKey and no finished definition speaks across mWorlds.
> A mCorrespondence may speak across mWorlds (2.7-corresponds-across-a-transition, 3.2-compare-one-chokepoint-four-answers).
> A mTopic is what a claim is about: the mKey read, plus the observer instance when that mKey's mSort is observer-dependent (2.8-observer-dependence-and-independence).
> A mTopic may carry several mDerivations with different generators: its mFullyQualifiedKey, a provider-supplied identifier, or a mCorrespondence from a transition owner (2.7-corresponds-across-a-transition).
> mDerivations combine by coherence (3.2-compare-one-chokepoint-four-answers), never by priority.

### § 1.9-cell-a-singleton-sort

Pre-311 documents write _aspect_.

A cell's mKey is written `parent-key@sm.Sort`, with the mSort's name in full reverse-DNS, and is
minted at the mark that names it (1.4-key-and-its-two-views, 1.6-parent-one-per-key); the
mScheme left of `@` is the mParent's. A cell's mReferent may hold its mState elsewhere than in
its mParent, and the mState may be diffuse: the cell's may-read set (2.5-may-read-the-readset)
says where, and the freshness of a fact about the cell follows the writesets that reach the cell
through that set (2.6-may-write-the-writeset), together with any write that covers the mParent
(2.9-the-traversal-and-the-region-test, 3.3-invalidation-three-mutator-species). Two cells of
one mParent are two mSorts, with two may-read sets and two `:observer-dependence`s, and
3.2-compare-one-chokepoint-four-answers decides between them as between any two mSorts; a
writeset entry naming the mParent covers its cells (step 2 of the walk). The marked line that
answers a cell is a read of the cell's mKey, and the fact's identity is the mTopic
(1.8-fully-qualified-key-topic-and-derivation). An mSort with no mScheme at all has only such
mKeys (1.3-scheme-a-way-of-writing).

```alloy
sig DeclaresCell extends Statement { theCell: one mSort, cellParent: one mSort }

fact { all d: DeclaresCell | d.speaker = d.theCell.sortOwner }

fun cellParentSort[c: mSort]: lone mSort { (DeclaresCell & InForce & theCell.c).cellParent }

fact { all c: mSort | lone cellParentSort[c] }

fact { all k: mKey | some k.cellSort implies some cellParentSort[k.cellSort] }

fact { all disj a, b: mKey | some a.cellSort & b.cellSort and some a.parent & b.parent implies a = b }

pred true_DeclaresCell[d: DeclaresCell] {}
```

<!-- prose-translation -->
> A cell is a singleton mSort identified in its mParent: its owner declares it `:identified-in` the mParent's mSort (2.2-primary-of-and-identified-in), one mParent mSort per cell.
> The singleton mSort has no mScheme of its own; its mKeys carry the cell mSort in place of an mScheme (1.4-key-and-its-two-views).
> Under any one mParent instance it has exactly one mKey.
> A cell's identity is its mParent's plus its mSort (3.1-identity-of-a-key, 3.2-compare-one-chokepoint-four-answers).
> Declaring a cell claims nothing about the world.

Example: `active` is held in the service manager's memory in the boot, and a reboot reaches it through its may-read set. `enabled` is held in a symlink in a filesystem, and survives a reboot only where its mParent is not itself scoped in the boot (3.3-invalidation-three-mutator-species).

### § 1.10-vantage-route-placeholder-witness

A mVantage is the address a probe reached an mReferent from, and every mKey carries the one it
was resolved from. What a wrapper lends, and how a vantage entered through a wrapper inherits
the rest, is 3.4-entry-and-lends; the mRoute holds no mState and declares no may-read set, so a
readset member whose mFullyQualifiedKey ends at it is ⊤ (2.5-may-read-the-readset). "Resolved
once per mEntryChain and shared" is by construction: a vantage holds one ambient instance per
mParent-Catalog mSort, so every mKey the vantage scopes in that mSort has the one instance as
its mParent, and two same-spelled leaf mKeys are two atoms, SAME only by warrant. The engine is
a speaker for its one vouch. The placeholder and the standup `witness()` are two instants the
fences do not hold (1.10.1-placeholder-and-witness).

```alloy
one sig engine extends Speaker {}

sig mVantage {
   route: one mRoute,
   enteredFrom: lone mVantage,
   through: lone Wrapper,
   ambient: mSort -> lone mKey
}

fact { all v: mVantage | some v.through iff some v.enteredFrom }

fact { no v: mVantage | v in v.^enteredFrom }

fact { all v: mVantage, s: mSort | sortOfKey[v.ambient[s]] in s }

fact { all k: mKey | k.yielded.at in k.at }

fact { all f: VerdictFact | f.underObservers = mSort.(f.topic.at.ambient) }

pred engineVouchIsTrue {
   all v: mVantage | no v.through implies
      all k: mKey | k.at = v and isNaturalKey[k] and some k.parent & v.ambient[catalogSortOf[k.scheme]] implies
         k.reaches in k.parent.reaches.passes
}
```

<!-- prose-translation -->
> A mVantage is the mEntryChain: a finite map from mParent-Catalog mSorts to the instances in effect, each an mKey of that mSort, with its mRoute; it is not part of any mKey's identity.
> Every mKey is resolved from one mVantage, and an mKey a lookup emits is resolved from the vantage of the mKey it was emitted for.
> A mVantage says where a `resolve()` executes, supplies the ambient mParent for every mKey of a secondary mScheme looked up in a lent mParent-Catalog mSort, and is the mRoute, the last-resort mParent, for a shape with no `:identified-in` (1.6-parent-one-per-key).
> A vantage entered through a wrapper is entered from the caller's vantage, and no vantage is entered from itself.
> The observers a fact was measured under are the instances its mEntryChain holds (2.8-observer-dependence-and-independence).
> For execution under no wrapper, the engine itself vouches the ambient mParent instances: the engine's vouch is true when every mKey of a secondary mScheme it scoped in an ambient instance reaches what a route through that instance's mReferent passes to.

#### § 1.10.1-placeholder-and-witness

<!-- normative -->
> The mRoute is an address; it holds no mState and declares no may-read set.
> For execution under no wrapper, the engine itself vouches the mRoute and the ambient mParent instances within one unwalled span.
> Each is resolved once per mEntryChain and shared: one mPlaceholder.
> A mFullyQualifiedKey whose mTokens are not yet measured is a mPlaceholder keyed by (mKey, ambient mParents, mEntryChain); the probe standup binds it.
> The apply standup re-reads it through the same entry and `compare()`s the two; that re-read is the `witness()`.
> A mismatch is integrity, never a verdict input.
> The `witness()` cannot see a recycled mKey; that stays on the outside-churn horizon.

A recycled mKey the `witness()` cannot see: a reissued pid or inode.

### § 1.11-site-and-claim-species

A mSite is within a book line, assay's `Line`: it has an argv, a program point (the line's place
among the lines above it), and an mEntryChain (1.10-vantage-route-placeholder-witness). Every
species of speech is a subtype of the shared `Statement`, with one speaker: the writeset claim
and the entailment are 2.6-may-write-the-writeset's; the mCorrespondence is
2.7-corresponds-across-a-transition's; the per-mScheme declarations are
2.1-yields-into-another-scheme's, 2.2-primary-of-and-identified-in's, and
2.9-the-traversal-and-the-region-test's; the per-mSort declarations are
2.5-may-read-the-readset's, 2.8-observer-dependence-and-independence's,
1.9-cell-a-singleton-sort's, and 2.10-places-the-upward-lookup's; the wrapper's `:lends` and
their sentinel are 3.4-entry-and-lends's. The verdict fact is declared here, since it is the
thing every sparing is about. Its `dependsOn` field is world stratum (0.1-the-two-strata): the
mReferents whose mState the measured answer in fact depended on, which no engine definition
reads.

```alloy
sig VerdictFact extends Statement {
   topic: one mKey,
   atLine: one Line,
   markedReads: set mKey,
   underObservers: set mKey,
   dependsOn: set mReferent
}
```

<!-- prose-translation -->
> A verdict fact is the measured answer to a read of a cell, taken at a mSite, under the instances its mEntryChain lent (3.4-entry-and-lends).
> Its readset is the body's marked reads (2.5-may-read-the-readset).
> The tool-oracle author vouches it: the vouch is the fact's speaker.

#### § 1.11.1-speech-is-typed

<!-- normative -->
> Speech at or about a mSite has one author per claim.
> Every warrant is typed speech, never an exit status.

## § 2-the-model-relations

Each relation states its arity, who declares it, its default, which consumer reads it, and which
warrant makes it dangerous.

### § 2.1-yields-into-another-scheme

S's `resolve()`, run in the mVantage, maps an mKey of S to an mKey of T; the emitted mKey sits
on the input mKey (1.4-key-and-its-two-views), and the lookup may supply the emitted mKey's
mParent instance from the yield seat (1.6-parent-one-per-key). Where S's own mKeys are looked
up is S's mParent-Catalog, the mParent of a natural mKey, supplied by the bind, by S's owner's
declaration, or by the mEntryChain's instance for that mSort
(1.10-vantage-route-placeholder-witness). Arity: per matched shape of a secondary mScheme, into
any mSort. Declared by: S's owner. Default: none; an mScheme that declares neither `:yields` nor
`:primary-of` is a floor primary mScheme (1.3-scheme-a-way-of-writing). Consumer: mResolution
(1.7-resolution-and-its-traversal) and the mFullyQualifiedKey (3.1-identity-of-a-key). Danger:
the lookup warrants; a wrong yield or a wrong supplied instance is a wrong SAME or DISJOINT,
attributed to the yield.

```alloy
sig DeclaresYields extends Statement { fromShape: one mShape, intoScheme: one mScheme }

sig DeclaresCatalogSort extends Statement { forScheme: one mScheme, catalogSort: one mSort }

fact { all d: DeclaresYields | d.speaker = d.fromShape.ofScheme.schemeOwner }

fact { all d: DeclaresCatalogSort | d.speaker = d.forScheme.schemeOwner }

fun catalogSortOf[s: mScheme]: lone mSort { (DeclaresCatalogSort & InForce & forScheme.s).catalogSort }

fact { all s: mScheme | lone catalogSortOf[s] }

pred true_DeclaresCatalogSort[d: DeclaresCatalogSort] {}

fun yieldsTo[s: mShape]: lone mScheme { (DeclaresYields & InForce & fromShape.s).intoScheme }

fact { all s: mShape | lone yieldsTo[s] }

fact { all s: mShape | some yieldsTo[s] implies no identifiedIn[s] and not isRoot[s] }

pred true_DeclaresYields[d: DeclaresYields] {
   all k: keysOfShape[d.fromShape] | some k.yielded implies k.reaches = k.yielded.reaches
}

fun naturalKeyAnswer[x, y: mKey]: one Answer {
   (some x.scheme & y.scheme and some x.parent & y.parent) implies
      (sameAtOneLevel[x, y] implies SAME
       else twoTopsWay[x, y] implies DISJOINT
       else UNKNOWN)
   else UNKNOWN
}

check law_natural_same_is_sound {
   everyStatementInForceIsTrue implies
      all x, y: mKey | naturalKeyAnswer[x, y] = SAME implies x.reaches = y.reaches
} for 6 but 4 Int

run law_natural_same_is_sound_premise {
   everyStatementInForceIsTrue
   some disj x, y: mKey | isNaturalKey[x] and naturalKeyAnswer[x, y] = SAME and some x.reaches
}

check law_natural_disjoint_is_sound {
   everyStatementInForceIsTrue implies
      all x, y: mKey | naturalKeyAnswer[x, y] = DISJOINT implies no x.reaches & y.reaches
} for 6 but 4 Int

run law_natural_disjoint_is_sound_premise {
   everyStatementInForceIsTrue
   some x, y: mKey | isNaturalKey[x] and naturalKeyAnswer[x, y] = DISJOINT and some x.reaches and some y.reaches
}
```

<!-- prose-translation -->
> mScheme S `:yields` mScheme T, per matched shape, where T is of any mSort; S's owner declares it; one T per shape.
> Where S's own mKeys are looked up is S's mParent-Catalog, of one mSort, which S's owner declares; the declaration claims nothing about the world.

UNACKED READING, temporary (`312d:ask-catalog-sort-declaration`): 311 names three seats that
supply the mParent-Catalog INSTANCE and presupposes its mSort ("the mEntryChain's instance for
that mSort") without a declaration of it; `DeclaresCatalogSort` is the conductor's addition so
that the entry-chain seat can be read. Not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).
> A shape that yields carries no `:identified-in` and no `:root`.
> `:yields` is true when, for every mKey of the shape whose lookup emitted an mKey, the two reach one mReferent, or both reach none.
> S's lookup warrants (1.5-token-and-the-two-warrants) govern what equality and inequality of S's mKeys license before the primary mScheme is reached: within one mParent-Catalog, two mKeys of S read SAME by the one-level rule and DISJOINT by the two-tops way of 3.2-compare-one-chokepoint-four-answers, and UNKNOWN otherwise.
> They never license across mParent-Catalogs: two mKeys not in one mParent-Catalog read UNKNOWN.
> A SAME licensed before the primary mScheme is reached is never false while every statement in force is true.
> A DISJOINT licensed before the primary mScheme is reached is never false while every statement in force is true.

#### § 2.1.1-the-chain-and-the-decline

That the yield chain terminates at a primary mScheme is a property of the declarations, which
strangers write; the fences read a chain that reaches no primary mKey as an unknown identity
(3.1-identity-of-a-key). A decline is an evaluation that emits nothing.

<!-- normative -->
> T is a primary mScheme, or a secondary mScheme that in turn yields one; the chain always terminates at a primary mScheme.
> An unknown input makes the instance unknown.
> A `resolve()` declines on mReferents its mSort does not describe; this is the mechanical net against lazy borrowing.

A cache and the file it caches are two mParent-Catalogs. A decline: a path reaching a socket, under an mScheme into files. Two mSchemes over one spelling, into two mSorts: a path yields the directory entry, or the inode that the entry leads to. A tool that removes the entry binds under the first, and a tool that changes the file binds under the second.

### § 2.2-primary-of-and-identified-in

ER's identifying relationship. Identifying is not containing.

A second name is a second mScheme. P's mKeys mean something only relative to K's mParent-Store;
`:primary-of` itself is 1.3-scheme-a-way-of-writing's, P's `resolve()` being the identity is
3.1-identity-of-a-key's, "an mKey reaches one mReferent or none" is 1.4-key-and-its-two-views's,
and the two warrants are 1.5-token-and-the-two-warrants's. A store that generates its mKeys at
creation holds an mReferent under a generated mKey while that mReferent exists; a store that
admits mKeys as names may hold none under an mKey a book names. The mParent's type varies per
shape, so the child mSort's owner never learns the mParent's types: the mParent mSort's primary
mScheme classifies, one level up, and each owner speaks one level. A mToken duplicable across
instances of its would-be mParent must be scoped in something smaller, or left un-warranted,
since `:root` is false of it. That a grade governs every consumer of the answer it grades,
corroboration and contradiction included, is 1.5-token-and-the-two-warrants's.

Arity: at most one mScheme per mSort. Declared by: the mSort's owner, on the primary mScheme.
Default: none; the floor of 1.3-scheme-a-way-of-writing supplies an unwarranted identity
primary mScheme. Consumer: identity (3.1-identity-of-a-key). Danger:
`:guarantees-unique-referent`, `:guarantees-unique-name`, and `:root`, per matched shape.

```alloy
sig DeclaresIdentifiedIn extends Statement { onShape: one mShape, inSort: one mSort }

sig DeclaresRoot extends Statement { rootedShape: one mShape }

fact { all d: DeclaresIdentifiedIn | d.speaker = d.onShape.ofScheme.schemeOwner }

fact { all d: DeclaresRoot | d.speaker = d.rootedShape.ofScheme.schemeOwner }

fact { all s: mScheme, k: mSort | primaryOf[s] = k implies s.schemeOwner = k.sortOwner }

fun identifiedIn[s: mShape]: lone mSort { (DeclaresIdentifiedIn & InForce & onShape.s).inSort }

fact { all s: mShape | lone identifiedIn[s] }

pred isRoot[s: mShape] { some DeclaresRoot & InForce & rootedShape.s }

fact { all s: mShape | isRoot[s] implies no identifiedIn[s] }

pred true_DeclaresIdentifiedIn[d: DeclaresIdentifiedIn] {
   all k: keysOfShape[d.onShape] | some k.reaches implies
      some r: keysOfSort[d.inSort].reaches | k.reaches in r.holds
}

pred true_DeclaresRoot[d: DeclaresRoot] {
   all a, b: keysOfShape[d.rootedShape] | a.value = b.value implies a.reaches = b.reaches
}
```

<!-- prose-translation -->
> P's owner declares, per matched shape of the mKey's mValue, `:identified-in` mSort M: the mParent's mSort for mKeys of that shape; one M per shape.
> The primary mScheme's owner is the mSort's owner.
> `:identified-in` M is true when every mKey of the shape that reaches an mReferent reaches one held by an mReferent that some mKey of M reaches.
> P's owner declares `:root` per matched shape, absent by default: the shape declares no mParent, so it carries no `:identified-in`, and thereby claims global comparability.
> `:root` is true when it is `:guarantees-unique-referent` over the whole world: two mKeys of the shape with equal mValues reach one mReferent.
> For a `:root` shape, the world is the store: the shape's mWorld (1.8-fully-qualified-key-topic-and-derivation).

UNACKED READING, temporary (`312d:enc-primary-owner-is-sort-owner`): 311 has `:identified-in`
declared by "P's owner" (§ 1.6, § 2.2) and the section "declared by the mSort's owner, on the
primary mScheme"; the fence reconciles the two by a fact that a primary mScheme's owner is its
mSort's owner. The conductor's reading, not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).

`:root` fails for cloned identifiers. The mParent's type varies per shape: an ext4 filesystem in the mRoute, an NFS filesystem in a host, a tmpfs in a boot.

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
sig DeclaresAliasesNothingElse extends Statement { store: one mKey }

pred aliasesNothingElse[l: mLevel] { some DeclaresAliasesNothingElse & InForce & store.l }

pred true_DeclaresAliasesNothingElse[d: DeclaresAliasesNothingElse] {
   let s = d.store.reaches |
      all r: s.holds | s in owns.r and owns.r in s
}
```

<!-- prose-translation -->
> A store is `:aliases-nothing-else` when nothing identified in it is, by the store's own construction, also identified in another store: every mReferent the store holds, it owns, and no other store owns.
> The store gives its own mKeys to no other store's mReferents.
> The warrant is declared per store, on the mKey that names it, and is absent by default.

Examples where it holds: a DNS zone for its records, a dpkg database for its packages, a network namespace for its `net/*` knobs, a disk filesystem for its inodes. Examples where it never holds: a client NFS mount, an NSS view of LDAP, a chroot, a nested pid namespace, an overlay, a front over another daemon, a relabelling of part of its mParent's state. State spanning several files is a may-read matter (2.5-may-read-the-readset). A hardlink is a `:guarantees-unique-name` failure on the path mScheme. The path mScheme yielding the inode dissolves it.

### § 2.4-parent-as-a-relation

Its mSort is the primary mScheme's declaration for the matched shape, and its instance is
whichever seat supplied it (1.6-parent-one-per-key). A mVantage supplies instances and is never
an mParent (1.10-vantage-route-placeholder-witness). Arity: one per mKey. Declared by: derived
from the mScheme's declarations and the supplying seat. Default: not applicable. Consumer:
routing, through a secondary mScheme; identity, through the primary mScheme. Danger: none of its
own.

```alloy
fun parentStore[k: mKey]: lone mLevel { isPrimaryKey[k] implies k.parent else none }

fun parentCatalog[k: mKey]: lone mLevel { isNaturalKey[k] implies k.parent else none }
```

<!-- prose-translation -->
> `:parent` is one per mKey, never plural, never a species of its own.
> Through the primary mScheme it is the mParent-Store, which carries identity; through a secondary mScheme it is the mParent-Catalog, which carries routing and never identity.
> At the primary mScheme, mParent-Catalog and mParent-Store are one mKey.
> The far end is an ordinary mKey with an identity of its own, or an mWorld, never a string the engine composes.

### § 2.5-may-read-the-readset

Separation logic's footprint, at the read side: the mReferents an answer may depend on.

The relation is many-valued: an edge type from an mSort to mKeys of other mSorts, a template
the mSite or environment fills; the fences hold the filled form, one entry per mKey of the
mSort, spoken by the mSort's owner, with the sentinel per mSort. Every mSort in a
mFullyQualifiedKey may declare may-read entries, not only leaves, and an entry may name an mKey
of another mWorld, which `compare()` decides as 3.2-compare-one-chokepoint-four-answers decides
any pair. May-read is distinct from mParent: the mParent is one and answers identity; may-read
entries are many and answer interference, so two cells with different mParents can share a
may-read entry and so collide without being the same. An mKey's mParent instance is no may-read
entry and needs no declaration, since the walk collides a write at or above it; an entry naming a
store says more, that every write to an mKey relative to that store may change K, nobody having
said otherwise, which is 2.9-the-traversal-and-the-region-test's whole-mark. A closed may-read
set is knife-tier, one of the two closures every sparing rests on. The mRoute declares no set
(1.10-vantage-route-placeholder-witness). Arity: many per mSort, with a sentinel. Declared by:
the mSort owner. Default: ⊤. Consumer: the writeset of a line. Danger: none positive; omission
is the silent channel.

```alloy
sig DeclaresMayRead extends Statement { ofKey: one mKey, readEntry: one mKey }

sig ClosesMayRead extends Statement { readSort: one mSort }

fact { all d: DeclaresMayRead | d.speaker = sortOfKey[d.ofKey].sortOwner }

fact { all d: ClosesMayRead | d.speaker = d.readSort.sortOwner }

fun mayReadEntries[k: mKey]: set mKey { (DeclaresMayRead & InForce & ofKey.k).readEntry }

fun mayReadEdge: mKey -> mKey { {k, e: mKey | e in mayReadEntries[k]} }

pred mayReadClosed[s: mSort] { some ClosesMayRead & InForce & readSort.s }

pred sortClosed[k: mKey] { mayReadClosed[sortOfKey[k]] }

pred readsetMemberIsTop[k: mKey] {
   some m: k.*mayReadEdge |
      (some l: m.*parent & mKey | not sortClosed[l]) or terminus[m] in mRoute
}

fun readset[f: VerdictFact]: set mKey { f.markedReads }

pred readsetIsTop[f: VerdictFact] { no f.markedReads or some k: f.markedReads | readsetMemberIsTop[k] }

pred true_DeclaresMayRead[d: DeclaresMayRead] {}

pred true_ClosesMayRead[d: ClosesMayRead] {
   all k: keysOfSort[d.readSort] | some k.reaches implies
      affects.(k.reaches) in k.reaches.*holds + (^holds).(k.reaches) + mayReadEntries[k].reaches
}

pred true_VerdictFact[f: VerdictFact] {
   some f.markedReads implies f.dependsOn in f.markedReads.reaches
}
```

<!-- prose-translation -->
> K `:may-read` these mKeys: K's mState is affected by writes to them; K's owner declares an entry per mKey of K, and closes the set per mSort with a completion sentinel.
> An entry licenses nothing positive.
> A closed may-read set is true when, for every mKey of K that reaches an mReferent, every mReferent whose write affects that mReferent is the mReferent itself, something it holds, something that holds it, or an mReferent one of the mKey's entries reaches.
> A fact's readset is the marked reads of the body that answered it; for a verdict fact, the vouch closes them (`KNOBS:kCONTRACT-RUNGS`): the vouch is true when the measured answer depended on no mReferent outside what the marked reads reach.
> A body that marks no read has readset ⊤.
> May-read entries are not in a readset; a write reaches K through them (2.6-may-write-the-writeset, rule 4).
> A readset member is ⊤ where its mSort has no closed may-read set, where an mSort on its mFullyQualifiedKey has no closed may-read set, where a may-read entry through which rule 4 reaches it fails either test, transitively, or where its mFullyQualifiedKey, or that of such an entry, ends at the mRoute.
> A readset is ⊤ where the body marked no read or where any member is ⊤.

UNACKED READING, temporary (`312d:ask-may-read-is-declared-per-key`): 311's may-read is "a
template the mSite or environment fills"; the fence holds only the filled form, one entry per
mKey, and no template. The truth predicates of the closure and of the vouch transcribe into the
world relations of 1.1-referent-state-and-value, which are themselves an unacked reading. The
conductor's reading, not acked, not authoritative, held only until acked or replaced
(`notes/312d` § 7).

A may-read entry above the leaf: a loop-backed filesystem's state lives in a file of the outer filesystem, and `dd` over the image rewrites every inner fact. A shared entry: two observers' writability cells share the file's mode.

### § 2.6-may-write-the-writeset

A refuted shape: two entries overlap only when they are one place (`311u:refuted-only-same-entries-overlap`). The model collides whatever is not DISJOINT.

The at-most set is the footprint of `plans/30U`, declared by the verb's author per matched
shape and closed by the completion record; where the body emits at runtime, the verb author's
completion record closes them. The entailment is arm-incremental and collide-adding, declared
by K's owner, about effects and not identity; it carries every cross-mSort consequence no
mFullyQualifiedKey expresses, and the reached finished record witnesses that the write set,
after entailment, is complete. The fences hold the filled form of the entailment, an entry per
written mKey, and the record per shape. The sentence excluding containers at or above the level
the written mKey shares with the read mKey admits two readings (`notes/312ch`, item 11): the
exclusion applied as the writeset is built, or only at the test; both are mechanized, the law is
stated over the first, which spares the more, and a check asks whether they ever disagree. Rule
3 reads what an entry given whole covers from 2.9-the-traversal-and-the-region-test and
2.10-places-the-upward-lookup. Arity: per matched shape of
the verb, for the at-most set; many per matched shape on the mSort, for the entailment; plus
the finished record. Declared by: the verb's author, for the at-most set; the mSort owner, for
the entailment. Default: unfinished, which collides. Consumer: the sparing test, within one
mWorld; never a generator of DISJOINT. Danger: the premature finished record.

```alloy
one sig World { lineWrites: Line -> mReferent }

sig DeclaresMayWrite extends Statement { writeLine: one Line, writeEntry: one mKey }

sig ClosesMayWrite extends Statement { closedLine: one Line }

sig DeclaresEntails extends Statement { fromKey: one mKey, entailedEntry: one mKey }

sig FinishesEntailment extends Statement { finishedSort: one mSort, finishedShape: lone mShape }

fact { all d: DeclaresEntails | d.speaker = sortOfKey[d.fromKey].sortOwner }

fact { all d: FinishesEntailment | d.speaker = d.finishedSort.sortOwner }

fun atMostEntries[l: Line]: set mKey { (DeclaresMayWrite & InForce & writeLine.l).writeEntry }

pred atMostClosed[l: Line] { some ClosesMayWrite & InForce & closedLine.l }

fun entailed[k: mKey]: set mKey { (DeclaresEntails & InForce & fromKey.k).entailedEntry }

pred entailmentFinished[k: mKey] {
   some d: FinishesEntailment & InForce | d.finishedSort = sortOfKey[k] and d.finishedShape = k.shape
}

fun wholeWriteEntries[l: Line]: set mKey { (DeclaresMayWrite & GivenWhole & InForce & writeLine.l).writeEntry }

fun wholeReadEntries: set mKey { (DeclaresMayRead & GivenWhole & InForce).readEntry }

fun entryAnswer[m, e: mKey]: one Answer { e in wholeReadEntries implies regionTest[e, m] else tabledCompare[m, e] }

fun rule4[m: mKey]: set mKey { {k: mKey | some e: mayReadEntries[k] | entryAnswer[m, e] != DISJOINT} }

fun seed[l: Line]: set mKey { atMostEntries[l] + beneathFor[wholeWriteEntries[l]] }

fun contributingContainers[m, r: mKey]: set mKey {
   (identity[m].*parent & mKey) - (meet[identity[m], identity[r]].mLevel).*parent
}

fun spreadsTo[m, r: mKey]: set mKey { entailed[contributingContainers[m, r]] + rule4[m] }

fun writesetAgainst[l: Line, r: mKey]: set mKey {
   seed[l].*({m, k: mKey | k in spreadsTo[m, r]})
}

fun writesetUnexcluded[l: Line]: set mKey {
   seed[l].*({m, k: mKey | k in entailed[identity[m].*parent & mKey] + rule4[m]})
}

fun writesetAtTest[l: Line, r: mKey]: set mKey {
   seed[l] + {k: mKey | some m: writesetUnexcluded[l] | k in spreadsTo[m, r]}
}

pred writesetIsTop[l: Line, ws: set mKey] {
   not atMostClosed[l] or some m: ws | not entailmentFinished[m]
}

fun memberAnswer[l: Line, w, r: mKey]: one Answer {
   w in wholeWriteEntries[l] implies regionTest[w, r] else tabledCompare[w, r]
}

pred sparedBy[l: Line, f: VerdictFact, ws: mKey -> mKey] {
   flagged
   l in f.atLine.above
   not readsetIsTop[f]
   no r: readset[f] | staleAt[f.atLine, r]
   all r: readset[f] {
      not writesetIsTop[l, ws[r]]
      all w: ws[r] | memberAnswer[l, w, r] = DISJOINT
   }
}

fun writesetsAgainst[l: Line]: mKey -> mKey { {r, w: mKey | w in writesetAgainst[l, r]} }

fun writesetsAtTest[l: Line]: mKey -> mKey { {r, w: mKey | w in writesetAtTest[l, r]} }

pred spared[l: Line, f: VerdictFact] { sparedBy[l, f, writesetsAgainst[l]] }

pred sparedAtTest[l: Line, f: VerdictFact] { sparedBy[l, f, writesetsAtTest[l]] }

pred true_DeclaresMayWrite[d: DeclaresMayWrite] {}

pred true_ClosesMayWrite[d: ClosesMayWrite] {
   let l = d.closedLine |
      World.lineWrites[l] in atMostEntries[l].reaches + wholeWriteEntries[l].reaches.passes
}

pred true_DeclaresEntails[d: DeclaresEntails] {}

pred true_FinishesEntailment[d: FinishesEntailment] {
   all k: keysOfSort[d.finishedSort] | k.shape = d.finishedShape and some k.reaches implies
      (k.reaches).affects in k.reaches.*holds + entailed[k].reaches
}

check law_sparing_is_sound {
   everyStatementInForceIsTrue and storesAreWellFounded and not hole_world_scoped_top_aliases_into_a_store implies
      all l: Line, f: VerdictFact & InForce | spared[l, f] implies
         no (World.lineWrites[l]).*affects & f.dependsOn
} for 6 but 4 Int

run law_sparing_is_sound_premise {
   everyStatementInForceIsTrue and storesAreWellFounded
   not hole_world_scoped_top_aliases_into_a_store
   some l: Line, f: VerdictFact & InForce |
      spared[l, f] and some World.lineWrites[l] and some f.dependsOn and some atMostEntries[l]
}

check law_exclusion_readings_agree {
   all l: Line, f: VerdictFact & InForce | spared[l, f] iff sparedAtTest[l, f]
} for 6 but 4 Int

run law_exclusion_readings_agree_premise {
   some l: Line, f: VerdictFact & InForce, m: atMostEntries[l], r: readset[f] |
      some contributingContainers[m, r] and some entailed[identity[m].*parent & mKey]
}
```

<!-- prose-translation -->
> A line's writeset against a read mKey is the set of mKeys the line may write or may change: the least set that four rules close.
> Rule 1: every may-write entry the verb's author declared per matched shape is in the writeset; the completion record closes those entries.
> Rule 2: where an mKey of K is in the writeset, or an mKey identified beneath an mKey of K, every mKey that K's may-write entailment names is in the writeset; a container at or above the deepest level that the written mKey shares with the read mKey contributes no entailment.
> Rule 3: where an mKey given whole is in the writeset, every mKey reached beneath it is in the writeset (2.9-the-traversal-and-the-region-test): its finished enumeration's members where it has one (2.10-places-the-upward-lookup), and every mKey its region covers besides where it has none.
> Rule 4: where an mKey in the writeset `compare()`s other than DISJOINT with a may-read entry declared for an mKey k, k is in the writeset (2.5-may-read-the-readset, 3.2-compare-one-chokepoint-four-answers); a may-read entry given whole is compared by the region test; may-read entries feed rule 4 and no other rule.
> Under the second reading the exclusion applies only at the test: the writeset is built with every container contributing, and an mKey excluded only by the last step is dropped there.
> An unclosed at-most set puts ⊤ in the writeset, and so does a member whose mSort and shape have no reached finished record.
> ⊤ is DISJOINT from nothing: an elision is spared past a line only under `--risk-faultless-skips` (3.2-compare-one-chokepoint-four-answers), and only when the line is above the fact's site, neither the readset nor the writeset against any readset member is ⊤, no readset member is stale at the site (3.3-invalidation-three-mutator-species), and `compare()` answers DISJOINT for every pair of a writeset member and a readset member, of one mSort or of two, by the region test where the member is an entry given whole (2.9-the-traversal-and-the-region-test).
> A may-write entry and an entailment entry license nothing alone.
> A completion record is true when every mReferent the line writes is one an at-most entry reaches, or one a route through a whole-marked entry's mReferent passes to.
> A finished record is true when, for every mKey of the shape that reaches an mReferent, writing that mReferent affects only it, what it holds, and the mReferents its entailment names.
> A sparing is never false while every statement in force is true and no store is among its own contents: no mReferent the line writes affects, directly or through others, an mReferent the fact's answer depended on.
> Whether the two readings of the exclusion ever disagree on a sparing is asked, and either answer is a finding.

UNACKED READING, temporary (`312d:ask-both-exclusion-readings-are-mechanized`): 311's exclusion
sentence admits two readings (`notes/312ch` item 11), and the fence mechanizes both rather than
choosing, but the sparing LAW is stated over one of them, the one that spares more, and "spared
only when" is read as the engine's decision rather than a necessary condition. Those are the
conductor's readings, not acked, not authoritative, held only until acked or replaced
(`notes/312d` § 7).

#### § 2.6.1-the-finished-definition-and-the-worlds

<!-- normative -->
> The test spares narrowly and collides widely: whatever is not DISJOINT collides.
> The entailment generates no DISJOINT: "nothing else" is no other thing, never no other mKey for the thing written.
> The finished definition is a within-mWorld sentence; it never speaks across mRoutes or mRoots (3.2-compare-one-chokepoint-four-answers).
> A may-read entry that names an mKey of another mWorld enters rule 4 where a write in that mWorld reaches it (2.5-may-read-the-readset).

Without the exclusion of containers at or above the shared level, a filesystem's entailment, which names its disk, would make two files in one filesystem collide through it. Examples of the entailment: a package's postinst enabling its unit, a restart killing a main process.

### § 2.7-corresponds-across-a-transition

A scoped `sameAs`. Not "corresponds to" loosely: a part, a view, or a correlate of a thing is not it.

Absent a mCorrespondence, mKeys across a transition `compare()` UNKNOWN unless a
mFullyQualifiedKey binds mTokens on both sides, which is the walk's step 1; the mCorrespondence
is the model's only declared sameness generator besides mToken equality, and it is consumed as
one SAME mDerivation by `compare()` (3.2-compare-one-chokepoint-four-answers), vouch-tier,
attributed to the transition author. Arity: per transition pair. Declared by: the transition
owner. Default: absent, so UNKNOWN. Danger: a wrong mCorrespondence is a wrong SAME.

```alloy
sig DeclaresCorresponds extends Statement { keyX: one mKey, keyY: one mKey }

fact { all d: DeclaresCorresponds | d.speaker not in d.keyX.scheme.schemeOwner + d.keyY.scheme.schemeOwner }

pred corresponds[x, y: mKey] {
   some d: DeclaresCorresponds & InForce | (d.keyX = x and d.keyY = y) or (d.keyX = y and d.keyY = x)
}

pred true_DeclaresCorresponds[d: DeclaresCorresponds] { d.keyX.reaches = d.keyY.reaches }
```

<!-- prose-translation -->
> mKey X inside mParent A `:corresponds` to mKey Y inside mParent B: they denote the same mReferent.
> The owner of the transition between A and B declares it, and that owner is neither mKey's mScheme owner.
> A mCorrespondence is true when the two mKeys reach one mReferent, or both reach none.

Examples: the container manager knows guest pid 1 is host pid 4821. `sudo -u alice` knows inner "me" is outer "alice". A mount line's oracle knows mKeys under the mountpoint are mKeys under the export on the named server, from this mVantage.

### § 2.8-observer-dependence-and-independence

No `resolve()` can measure observer-dependence: the object is the same and the answer differs,
so it must remain speech, and measurement in the denoted context (`plans/27C`) stays the default
lane. The observers a read was taken under are the lent instances of its mEntryChain
(3.4-entry-and-lends), held on the verdict fact. Arity: per (mSort, O). Declared by: the mSort
owner. Default: dependent, so no carry across O-instances. Consumer: the SAME consumer, as a
qualifier on the claim's mTopic. Danger: a false independence.

```alloy
sig DeclaresObserverIndependence extends Statement { independentSort: one mSort, ofObserver: one mSort }

fact { all d: DeclaresObserverIndependence | d.speaker = d.independentSort.sortOwner }

pred observerIndependent[k: mSort, o: mSort] {
   some DeclaresObserverIndependence & InForce & independentSort.k & ofObserver.o
}

fun topicObservers[f: VerdictFact]: set mKey {
   {o: f.underObservers | not observerIndependent[sortOfKey[f.topic], sortOfKey[o]]}
}

pred sameTopic[f, g: VerdictFact] {
   tabledCompare[f.topic, g.topic] = SAME
   all o: topicObservers[f] | some p: topicObservers[g] | tabledCompare[o, p] = SAME
   all p: topicObservers[g] | some o: topicObservers[f] | tabledCompare[o, p] = SAME
}

pred true_DeclaresObserverIndependence[d: DeclaresObserverIndependence] {
   all f: VerdictFact | sortOfKey[f.topic] = d.independentSort implies
      no f.dependsOn & {o: f.underObservers | sortOfKey[o] = d.ofObserver}.reaches
}
```

<!-- prose-translation -->
> The mValues that reads of K's cells yield depend on which mKey of mSort O the read was taken under; K's owner declares the complement, `:observer-independence` of O, per mSort.
> By default, a cell measured under a lent mKey of O is assumed to depend on it: its fact is then about (mReferent, O-instance), and it stands for another fact only when the two are about one mReferent under O-instances that are SAME.
> `:observer-independence` of O is true when no answer about a K-cell depended on the O-instance it was taken under.

UNACKED READING, temporary (`312d:enc-observers-are-the-vantage-ambients`): 311 says a cell is
"measured under a lent mKey of O"; the fence takes every ambient instance of the topic's
mVantage as an observer and the answer's dependence on the observer's mReferent as the truth
of independence. The conductor's readings, not acked, not authoritative, held only until acked
or replaced (`notes/312d` § 7).

### § 2.9-the-traversal-and-the-region-test

A lookup emits its mTraversal (1.7-resolution-and-its-traversal); it may cross several levels,
each looked up in a catalog that the previous level named. The engine never reads an mKey's
syntax: whatever splitting an mKey needs happens inside a lookup's body. A lookup that emits no
member has its mParent-Catalog, given whole, as its mTraversal, and any touch on that catalog
then invalidates every mResolution through it: the coarse, safe floor. The walk of
3.2-compare-one-chokepoint-four-answers collides a write to an entry's mReferent with
everything identified in it; what an entry given whole names beyond that is the region test.
Only x's mTraversals are walked, and D needs no closure of its own; every level is asked, never
only the leaf, because an alias may sit at any level and a leaf's own closure cannot see it.
`compare()` compares a level by the identity of the mReferent that the level resolved to
(`walkOfKeys`, § 3.2). Unequal mTokens say only that the thing is not the routing mKey itself.
Arity: per lookup, per matched shape. Declared by: the lookup's owner, through what the lookup
emits. Default: no emission, so the mTraversal is the mParent-Catalog given whole. Consumer:
mResolution backings, hence invalidation (3.3-invalidation-three-mutator-species), and the
region test. Danger: a false closing act keeps a stale mResolution and every conclusion built on
it; a false `alias nothing-else` is a wrong DISJOINT; an open or coarse emission is safe.

```alloy
sig GivenWhole in DeclaresMayWrite + DeclaresMayRead {}

sig EmitsAliasNothingElse extends Statement { atLevel: one mKey }

fact { all d: EmitsAliasNothingElse | d.speaker = d.atLevel.scheme.schemeOwner }

pred aliasClosed[m: mKey] { some EmitsAliasNothingElse & InForce & atLevel.m }

pred true_EmitsAliasNothingElse[d: EmitsAliasNothingElse] {
   no c: mKey - d.atLevel | c.scheme = d.atLevel.scheme and some c.reaches and c.reaches = d.atLevel.reaches
}

fun sortOfKey[k: mKey]: lone mSort { primaryOf[identity[k].scheme] + identity[k].cellSort }

fun traversalMembers[k: mKey]: set mKey {
   crossed[k] + (traversalClosed[k] implies none else k.parent & mKey)
}

fun levelsOf[x: mKey]: set mKey { x.*yielded + (identity[x].^parent & mKey) }

pred coveredBy[D, x: mKey] {
   some m: traversalMembers[levelsOf[x]] + placedIn[x, sortOfKey[D]] | tabledCompare[m, D] = SAME
}

pred lookupTraversalOfSort[l: mKey, G: mSort] {
   some traversalMembers[l] and all m: traversalMembers[l] | some sortOfKey[m] & G
}

pred outsideByTraversals[D, x: mKey] {
   let G = sortOfKey[D] {
      (some l: levelsOf[x] | lookupTraversalOfSort[l, G]) or some placedIn[x, G]
      all l: levelsOf[x] | lookupTraversalOfSort[l, G] implies
         traversalClosed[l] and all m: traversalMembers[l] | tabledCompare[m, D] = DISJOINT and aliasClosed[m]
      some placedIn[x, G] implies
         lookedUpInClosed[x, G] and all g: placedIn[x, G] | tabledCompare[g, D] = DISJOINT
   }
}

pred outsideByPlacing[D, x: mKey] {
   let G = sortOfKey[D] | lookedUpInClosed[x, G] and no placedIn[x, G]
}

fun regionTest[D, x: mKey]: one Answer {
   tabledCompare[x, D] = SAME implies SAME
   else coveredBy[D, x] implies UNKNOWN
   else (outsideByTraversals[D, x] or outsideByPlacing[D, x]) implies DISJOINT
   else UNKNOWN
}

fun beneath[D: mKey]: set mKey { {x: mKey | coveredBy[D, x]} }

check law_region_disjoint_is_sound {
   everyStatementInForceIsTrue and storesAreWellFounded implies
      all D, x: mKey | regionTest[D, x] = DISJOINT implies
         no x.reaches & (D.reaches + D.reaches.passes)
} for 6 but 4 Int

run law_region_disjoint_is_sound_premise {
   everyStatementInForceIsTrue and storesAreWellFounded
   some D, x: mKey | regionTest[D, x] = DISJOINT and some x.reaches and some D.reaches.passes
      and some traversalMembers[levelsOf[x]]
}
```

<!-- prose-translation -->
> An entry's author marks it given whole.
> A lookup may emit a closure, `alias nothing-else`, for a level it resolved (1.5-token-and-the-two-warrants); it is true when no other mKey of that level's mScheme reaches the level's mReferent.
> A lookup's mTraversal members are the routing mKeys it emitted, and its mParent-Catalog where the emission is not closed.
> The mTraversals of x are those `identity(x)` produced, at every lookup on the way to the primary mKey and at every level of x's mFullyQualifiedKey (1.7-resolution-and-its-traversal, 3.1-identity-of-a-key), and the routes of 2.10-places-the-upward-lookup.
> A mTraversal is of D's mSort when every member of it is of that mSort, or when it is a placing route of that mSort.
> For an mKey D given whole against an mKey x, step 1: if x's leaf compares SAME with D, SAME.
> Step 2: else if any level of a mTraversal compares SAME with D, D's region covers x, and the pair reads UNKNOWN.
> Step 3: else if x carries a closure for D's mSort, DISJOINT, in one of two forms.
> First form: x has at least one mTraversal of D's mSort, and on every such mTraversal every level compares DISJOINT with D and every level emitted its closure, `alias nothing-else` for a lookup's level or `looked-up-in nothing-else` for a placing route (2.10-places-the-upward-lookup), the lookup's emission being closed.
> Second form: the placing lookup of D's mSort emitted `looked-up-in nothing-else` for x with no `looked-up-in` record; then x is in no region of that mSort.
> Step 4: otherwise UNKNOWN.
> An entry given whole names, beyond the mReferent of its mKey, every mReferent reached beneath that mKey through the mScheme's lookups or through a placing route: every mKey the region covers.
> A DISJOINT of the region test is never false while every statement in force is true and no store is among its own contents: x reaches neither D's mReferent nor an mReferent a route through D's mReferent passes to.

UNACKED READING, temporary (`312d:ask-alias-closure-instance-scope`): 311 scopes `alias
nothing-else` to "the instance the lookup ran in"; the fence's truth ranges over every mKey of
the level's mScheme in the world, which is STRONGER than the sentence and so the direction that
can hide a counterexample. "Of D's mSort" for a mTraversal is read as every member being of
that mSort. The conductor's readings, not acked, not authoritative, held only until acked or
replaced (`notes/312d` § 7).

#### § 2.9.1-indexicals-and-the-floor

<!-- normative -->
> A lookup may cross an indexical routing mKey, whose mResolution depends on the observing process; only the lookup's owner can say which, in the body that meets it, and an undeclared indexical routing mKey reads unknown.
> Containment is membership in a mTraversal.
> A routing mKey named whole, in a writeset or as a may-read entry, stands for whatever its mScheme reaches beneath it; in the test of 2.6-may-write-the-writeset it reads UNKNOWN against every mKey that mScheme can yield in the same mParent-Catalog instance, whatever 3.2-compare-one-chokepoint-four-answers answers of the two as siblings; that is the floor, which the region test refines.

Lookups that cross several levels: a path yields a directory entry looked up in a shorter path. A hostname yields a resolver step from a mVantage. A dotted unit name yields an entry in its instance table. Lookups that cross one level: an inode number, a uid. An indexical routing mKey: `/proc/self`. A path prefix is not a store. A mutator that touches a directory needs to know nothing about files. An alias above the leaf: a bind mount of a directory above a file, or an alias entry above a leaf. A positional catalog: the resolution of position N depends on every position at or before it, and `ufw insert 1` gives every later rule a new number. The lookup emits positions 1 to N as its mTraversal, one line per member. A rowid table renumbered after ten thousand rows makes that emission large. A single emission that describes the set, spelled as sh, is work for the 312 series.

### § 2.10-places-the-upward-lookup

G's owner publishes a lookup that is invoked with the mValue of an mKey of T, its matched
shapes deciding which spellings of T it answers; membership is a relation between two
mReferents, never a spelling of one, so the placing lookup is not an mScheme of T. The route
so recorded is a mTraversal of the mKey for the region test
(2.9-the-traversal-and-the-region-test), invalidated as any mResolution is
(3.3-invalidation-three-mutator-species), and never the mKey's mParent, which is the route the
mKey's own lookup supplied (1.6-parent-one-per-key). When the engine invokes the lookup, and
what it refuses when two invocations disagree, are 2.10.1-invocation-and-refusal. Arity: per
(G, T). Declared by: G's owner. Default: absent; T's mKeys then have no route of mSort G, and
the pair reads as 3.2-compare-one-chokepoint-four-answers decides it. Consumer: the region
test, and invalidation. Danger: a false `looked-up-in nothing-else` is G's owner's wrong
DISJOINT.

```alloy
sig DeclaresPlaces extends Statement { placingSort: one mSort, placedSort: one mSort }

sig RecordsLookedUpIn extends Statement { placedKey: one mKey, inKey: one mKey }

sig ClosesLookedUpIn extends Statement { closedKey: one mKey, routeSort: one mSort }

fact { all d: DeclaresPlaces | d.speaker = d.placingSort.sortOwner }

fact { all d: RecordsLookedUpIn | d.speaker = sortOfKey[d.inKey].sortOwner }

fact { all d: ClosesLookedUpIn | d.speaker = d.routeSort.sortOwner }

pred places[G, T: mSort] { some DeclaresPlaces & InForce & placingSort.G & placedSort.T }

fun placedIn[x: mKey, G: mSort]: set mKey {
   {g: (RecordsLookedUpIn & InForce & placedKey.x).inKey | some sortOfKey[g] & G}
}

pred lookedUpInClosed[x: mKey, G: mSort] { some ClosesLookedUpIn & InForce & closedKey.x & routeSort.G }

fun beneathFor[P: mKey]: set mKey { entailmentFinished[P] implies entailed[P] else entailed[P] + beneath[P] }

pred true_DeclaresPlaces[d: DeclaresPlaces] {}

pred true_RecordsLookedUpIn[d: RecordsLookedUpIn] {
   d.placedKey.reaches in d.inKey.reaches.passes
}

pred true_ClosesLookedUpIn[d: ClosesLookedUpIn] {
   all g: keysOfSort[d.routeSort] | d.closedKey.reaches in g.reaches.passes implies
      some r: placedIn[d.closedKey, d.routeSort] | r.reaches = g.reaches
}
```

<!-- prose-translation -->
> An mSort G may declare that it `:places` another mSort T; G's owner declares it.
> For an mKey of T the placing lookup emits `looked-up-in G:key`, a record G's owner speaks, true when the mKey's mReferent is one that a route through G:key's mReferent passes to.
> It emits a closure `looked-up-in nothing-else`, scoped to routes of mSort G, true when every G-thing a route to the mKey's mReferent passes through is one a record names.
> The `looked-up-in` records of every invocation accumulate: x's routes of mSort G are every G:key recorded for x.
> The store's end of the same relation is G's enumeration of its members, which the may-write entailment of 2.6-may-write-the-writeset carries as write reach: when that entailment is finished for P, its emitted members stand in, in the test of 2.6-may-write-the-writeset, for the mKeys reached beneath P (rule 3); an unfinished entailment widens the writeset only.

#### § 2.10.1-invocation-and-refusal

Invocations are not in the fences; a record either is in force or is not.

<!-- normative -->
> The engine invokes G's lookup only when all three hold: a writeset or readset entry names an mKey of G given whole; the mKey of T on the other side of that pair, a writeset entry or a readset entry, has no route of mSort G; G declares that it places T.
> It invokes the lookup with every mKey it holds for that mReferent.
> A closure `looked-up-in nothing-else` from one invocation can contradict a record from another invocation; the engine then refuses both answers and attributes the refusal to G's owner.
> Each member of a finished enumeration is an entry of the test of 2.6-may-write-the-writeset, and P itself stays an entry of that test.
> Where a placing route places x in P and a finished enumeration of P has no member SAME with x, the pair reads UNKNOWN.

Matched shapes of a placing lookup: a path-shaped mValue answered, an inode number declined.

### § 2.11-composite-sorts-and-roles

The author who knows the roles mints the mCompositeSort, normally the tool author. Plurality of
inputs is an mSort with structure, never a set of mParents: a composite mKey has one mParent as
any mKey does, and its parts beside it. Arity: per composite. Declared by: the author holding
the roles. Default: not applicable. Consumer: identity (3.1-identity-of-a-key), through
`compare()`. Danger: as any mSort.

```alloy
sig Role {}

sig DeclaresComposite extends Statement { compositeSort: one mSort }

fact { all d: DeclaresComposite | d.speaker = d.compositeSort.sortOwner }

pred isComposite[s: mSort] { some DeclaresComposite & InForce & compositeSort.s }

sig CompositeKey in mKey { part: Role -> lone mKey }

fact { all k: CompositeKey | isComposite[sortOfKey[k]] and some k.part }

fact { all k: mKey - CompositeKey | no k.part }

pred compositeSame[x, y: mKey] {
   x + y in CompositeKey
   sortOfKey[x] = sortOfKey[y]
   x.part.mKey = y.part.mKey
   all r: x.part.mKey | tabledWalk[x.part[r], y.part[r]] = SAME
}

fun compositeMayRead[k: mKey]: set mKey { mayReadEntries[k] + mayReadEntries[Role.(k.part)] }

pred true_DeclaresComposite[d: DeclaresComposite] {}
```

<!-- prose-translation -->
> A mTopic whose mReferent's mState depends on several inputs in roles is an mKey of a mCompositeSort: a composite mKey names one part per role.
> That mSort's identity is its owner's function of its named parts: two composite mKeys of one mCompositeSort are SAME when they name the same roles and their parts are SAME role by role.
> The may-read set of a mCompositeSort is the union of its parts' may-read sets.
> Declaring a mCompositeSort claims nothing about the world.

UNACKED READING, temporary (`312d:ask-composite-parts-by-walk`): 311 says the identity "is its
owner's function of its named parts"; the fence fixes that function as parts SAME role by role
by the walk alone, since `compare()` reads this predicate and Alloy refuses the recursion. The
conductor's reading, not acked, not authoritative, held only until acked or replaced
(`notes/312d` § 7).

Examples: a base and an overlay, or a primary and its replica set.

## § 3-composition-and-laws

### § 3.1-identity-of-a-key

`identity(k)` runs the lookups that take a natural mKey to a primary one
(2.1-yields-into-another-scheme) and then reads the chain of
1.8-fully-qualified-key-topic-and-derivation. Each level carries the warrants declared for the
shape its mKey matched (1.5-token-and-the-two-warrants).

```alloy
fun identity[k: mKey]: lone mKey { {p: k.*yielded | isPrimaryKey[p]} }
```

<!-- prose-translation -->
> For k an mKey of mScheme S, follow S's `resolve()`'s emission, and each yielded mScheme's emission in turn, until an mKey of a primary mScheme is in hand: that mKey is the identity of k, and there is none where no emission reaches a primary mScheme.
> The identity of an mKey of a primary mScheme is that mKey: the primary mScheme's `resolve()` is the identity on the mKey (2.2-primary-of-and-identified-in).
> The result is that mKey-Primary scoped in the identity of its mParent, recursively through each level's primary mScheme, until a mRoot, the mRoute, or an unknown link: its mFullyQualifiedKey (1.8-fully-qualified-key-topic-and-derivation).

#### § 3.1.1-what-identity-reads-beyond-the-chain

Where a `resolve()` runs is 1.10-vantage-route-placeholder-witness's; composites, cells, and
observers are 2.11-composite-sorts-and-roles's, 1.9-cell-a-singleton-sort's, and
2.8-observer-dependence-and-independence's.

<!-- normative -->
> Each `resolve()` runs from k's mVantage.
> Each emission supplies the mParent instance for the mKey it yields.
> A mCompositeSort's identity is its owner's function of its parts' identities.
> A cell's identity is its mParent's plus its mSort (1.9-cell-a-singleton-sort).
> An mKey of an observer-dependent mSort carries the O-instance in its mTopic.
> The mVantage is consulted only to know where to run `resolve()` calls and which ambient mParents to bind.

### § 3.2-compare-one-chokepoint-four-answers

Alias analysis's may/must trichotomy, plus KNOWN_UNSPOKEN for "no generator applies".

A refuted shape: a parent partitions its children's mSorts (`311u:refuted-parent-partitions-its-children`). Separation comes from one definition's own distinctions.

The walk below is the mFullyQualifiedKey mDerivation; `compare(x, y)` combines it with the
others, and the laws of § 0 are stated over both here; what each answer licenses is
3.2.1-what-the-answers-mean-to-their-consumers. Levels
are numbered from the leaf, level 0, upward through mParents; the walk aligns two chains by
height from the terminus (1.8-fully-qualified-key-topic-and-derivation). "One instance" is one
atom until 1.10-vantage-route-placeholder-witness is mechanized: the mPlaceholder inherited
through a wrapper's sentinel under `--risk-faultless-skips` (3.4-entry-and-lends), or resolved
once in one unwalled span under no wrapper, will widen `oneInstance` and nothing else. Step 1's
reasons (a mRoot shape is one mWorld, SAME by `:root`; two mKeys of that shape meet there and
compare as siblings; mRoots of two shapes are two mWorlds, and a mRoute is another) are how
1.8-fully-qualified-key-topic-and-derivation declares the termini. mKeys of different mSorts
share no primary mScheme, so their mFullyQualifiedKeys meet, if at all, only at a common
ancestor, and that meeting is not a claim about the leaves; Dorc equates mKeys and never merges
mSorts. The laws take every statement in force as true, since the support of one answer is
3.5-committee-law-and-attribution's; "a store is never among its own contents", which the
one-top way rests on, is a premise of the DISJOINT law and never a fact.

```alloy
abstract sig Answer {}

one sig SAME, DISJOINT, KNOWN_UNSPOKEN, UNKNOWN extends Answer {}

pred oneInstance[a, b: mLevel] { a = b }

pred sameAtOneLevel[a, b: mLevel] {
   oneInstance[a, b]
   or (a + b in mKey and a.value = b.value and a.shape = b.shape and guaranteesUniqueReferent[a.shape])
   or (a + b in mKey and some a.cellSort and a.cellSort = b.cellSort)
}

pred alignedSame[a, b: mLevel] {
   height[a] = height[b]
   all a2: a.*parent, b2: b.*parent | height[a2] = height[b2] implies sameAtOneLevel[a2, b2]
}

pred sameChains[x, y: mKey] { alignedSame[x, y] }

fun meet[x, y: mKey]: mLevel -> mLevel {
   {a: x.*parent, b: y.*parent |
      alignedSame[a, b] and
      no a2: x.*parent, b2: y.*parent | alignedSame[a2, b2] and a in a2.^parent}
}

fun topBelow[x: mKey, a: mLevel]: lone mLevel { {t: x.*parent | t.parent = a} }

fun legStores[x: mKey, a: mLevel]: set mLevel { (x.^parent & mKey) - a.*parent }

pred twoTopsWay[tx, ty: mLevel] {
   tx + ty in mKey
   tx.scheme = ty.scheme
   guaranteesUniqueName[tx.shape] and guaranteesUniqueName[ty.shape]
   tx.value != ty.value
}

pred oneTopWay[x, y: mKey, tx, ty: mLevel] {
   x = tx and y != ty
   some primaryOf[ty.scheme]
   some s: ofScheme.(x.scheme) - x.shape | identifiedIn[s] = primaryOf[ty.scheme]
}

pred separatedAt[x, y: mKey, a, b: mLevel] {
   x != a and y != b
   let tx = topBelow[x, a], ty = topBelow[y, b] |
      twoTopsWay[tx, ty] or oneTopWay[x, y, tx, ty] or oneTopWay[y, x, ty, tx]
   all s: legStores[x, a] + legStores[y, b] | aliasesNothingElse[s]
}

fun walk[x, y: mKey]: one Answer {
   (not knownChain[x] or not knownChain[y] or worldOf[x] != worldOf[y]) implies UNKNOWN
   else sameChains[x, y] implies SAME
   else (some a: meet[x, y].mLevel, b: mLevel.(meet[x, y]) | x = a or y = b) implies UNKNOWN
   else (some a: meet[x, y].mLevel, b: mLevel.(meet[x, y]) | separatedAt[x, y, a, b]) implies DISJOINT
   else x.scheme != y.scheme implies KNOWN_UNSPOKEN
   else UNKNOWN
}

fun walkOfKeys[x, y: mKey]: one Answer {
   (some identity[x] and some identity[y]) implies walk[identity[x], identity[y]] else UNKNOWN
}

pred sameBy[x, y: mKey] { tabledWalk[x, y] = SAME or corresponds[x, y] or compositeSame[x, y] }

fun sameClosure: mKey -> mKey { *{x, y: mKey | sameBy[x, y] or sameBy[y, x]} }

pred contradicted[x, y: mKey] {
   y in x.sameClosure
   some x2: x.sameClosure, y2: y.sameClosure | tabledWalk[x2, y2] = DISJOINT
}

fun compare[x, y: mKey]: one Answer {
   contradicted[x, y] implies UNKNOWN
   else y in x.sameClosure implies SAME
   else (some x2: x.sameClosure, y2: y.sameClosure | tabledWalk[x2, y2] = DISJOINT) implies DISJOINT
   else tabledWalk[x, y]
}

one sig Tables { walkTable: mKey -> mKey -> Answer, compareTable: mKey -> mKey -> Answer }

fact { all x, y: mKey | Tables.walkTable[x][y] = walkOfKeys[x, y] }

fact { all x, y: mKey | Tables.compareTable[x][y] = compare[x, y] }

fun tabledWalk[x, y: mKey]: one Answer { Tables.walkTable[x][y] }

fun tabledCompare[x, y: mKey]: one Answer { Tables.compareTable[x][y] }

pred hole_cell_keys_under_same_parents_reach_differently {
   some disj a, b: mKey | some a.cellSort & b.cellSort and a.parent != b.parent
      and some a.parent.reaches & b.parent.reaches and a.reaches != b.reaches
}

run hole_cell_keys_under_same_parents_reach_differently_witness {
   hole_cell_keys_under_same_parents_reach_differently and everyStatementInForceIsTrue
} for 6 but 4 Int expect 1

pred hole_world_scoped_top_aliases_into_a_store {
   some x, s: mKey | some x.parent & mWorld and some s.parent & x.parent and s != x
      and some x.reaches and x.reaches in s.reaches.^holds
}

run hole_world_scoped_top_aliases_into_a_store_witness {
   hole_world_scoped_top_aliases_into_a_store and everyStatementInForceIsTrue and storesAreWellFounded
} for 6 but 4 Int expect 1

pred hole_composite_keys_with_same_parts_reach_differently {
   some disj a, b: CompositeKey | compositeSame[a, b] and a.reaches != b.reaches
}

run hole_composite_keys_with_same_parts_reach_differently_witness {
   hole_composite_keys_with_same_parts_reach_differently and everyStatementInForceIsTrue
} for 6 but 4 Int expect 1

check law_compare_same_is_sound {
   everyStatementInForceIsTrue
      and not hole_cell_keys_under_same_parents_reach_differently
      and not hole_composite_keys_with_same_parts_reach_differently implies
      all x, y: mKey | compare[x, y] = SAME implies x.reaches = y.reaches
} for 6 but 4 Int

run law_compare_same_is_sound_premise {
   everyStatementInForceIsTrue
   not hole_cell_keys_under_same_parents_reach_differently
   not hole_composite_keys_with_same_parts_reach_differently
   some disj x, y: mKey | compare[x, y] = SAME and walkOfKeys[x, y] != SAME and some x.reaches
}

check law_compare_disjoint_is_sound {
   everyStatementInForceIsTrue and storesAreWellFounded
      and not hole_world_scoped_top_aliases_into_a_store
      and not hole_composite_keys_with_same_parts_reach_differently implies
      all x, y: mKey | compare[x, y] = DISJOINT implies no x.reaches & y.reaches
} for 6 but 4 Int

run law_compare_disjoint_is_sound_premise {
   everyStatementInForceIsTrue and storesAreWellFounded
   not hole_world_scoped_top_aliases_into_a_store
   not hole_composite_keys_with_same_parts_reach_differently
   some x, y: mKey | compare[x, y] = DISJOINT and walkOfKeys[x, y] != DISJOINT and some x.reaches and some y.reaches
}

pred allInForceTrueExcept[except: set Statement] {
   engineVouchIsTrue
   all d: DeclaresPrimaryOf & InForce - except | true_DeclaresPrimaryOf[d]
   all d: DeclaresYields & InForce - except | true_DeclaresYields[d]
   all d: DeclaresIdentifiedIn & InForce - except | true_DeclaresIdentifiedIn[d]
   all d: DeclaresRoot & InForce - except | true_DeclaresRoot[d]
   all d: SuppliesParent & InForce - except | true_SuppliesParent[d]
   all d: DeclaresUniqueReferent & InForce - except | true_DeclaresUniqueReferent[d]
   all d: DeclaresUniqueName & InForce - except | true_DeclaresUniqueName[d]
   all d: DeclaresAliasesNothingElse & InForce - except | true_DeclaresAliasesNothingElse[d]
   all d: DeclaresMayRead & InForce - except | true_DeclaresMayRead[d]
   all d: ClosesMayRead & InForce - except | true_ClosesMayRead[d]
   all d: VerdictFact & InForce - except | true_VerdictFact[d]
   all d: DeclaresMayWrite & InForce - except | true_DeclaresMayWrite[d]
   all d: ClosesMayWrite & InForce - except | true_ClosesMayWrite[d]
   all d: DeclaresEntails & InForce - except | true_DeclaresEntails[d]
   all d: FinishesEntailment & InForce - except | true_FinishesEntailment[d]
   all d: EmitsCrossed & InForce - except | true_EmitsCrossed[d]
   all d: ClosesTraversal & InForce - except | true_ClosesTraversal[d]
   all d: EmitsAliasNothingElse & InForce - except | true_EmitsAliasNothingElse[d]
   all d: DeclaresPlaces & InForce - except | true_DeclaresPlaces[d]
   all d: RecordsLookedUpIn & InForce - except | true_RecordsLookedUpIn[d]
   all d: ClosesLookedUpIn & InForce - except | true_ClosesLookedUpIn[d]
   all d: DeclaresCell & InForce - except | true_DeclaresCell[d]
   all d: DeclaresCorresponds & InForce - except | true_DeclaresCorresponds[d]
   all d: DeclaresObserverIndependence & InForce - except | true_DeclaresObserverIndependence[d]
   all d: DeclaresComposite & InForce - except | true_DeclaresComposite[d]
   all d: DeclaresCatalogSort & InForce - except | true_DeclaresCatalogSort[d]
   all d: DeclaresLends & InForce - except | true_DeclaresLends[d]
   all d: ClosesLends & InForce - except | true_ClosesLends[d]
}

pred everyStatementInForceIsTrue { allInForceTrueExcept[none] }

pred storesAreWellFounded { no r: mReferent | r in r.^holds }

check law_same_is_sound {
   everyStatementInForceIsTrue and not hole_cell_keys_under_same_parents_reach_differently implies
      all x, y: mKey | walkOfKeys[x, y] = SAME implies x.reaches = y.reaches
} for 6 but 4 Int

run law_same_is_sound_premise {
   everyStatementInForceIsTrue
   not hole_cell_keys_under_same_parents_reach_differently
   some disj x, y: mKey | walkOfKeys[x, y] = SAME and some x.reaches
}

check law_disjoint_is_sound {
   everyStatementInForceIsTrue and storesAreWellFounded and not hole_world_scoped_top_aliases_into_a_store implies
      all x, y: mKey | walkOfKeys[x, y] = DISJOINT implies no x.reaches & y.reaches
} for 6 but 4 Int

run law_disjoint_is_sound_premise {
   everyStatementInForceIsTrue and storesAreWellFounded
   not hole_world_scoped_top_aliases_into_a_store
   some x, y: mKey | walkOfKeys[x, y] = DISJOINT and some x.reaches and some y.reaches
      and some (x.^parent + y.^parent) & mKey
}

check law_nobody_spoke_declines {
   no InForce & (DeclaresUniqueReferent + DeclaresUniqueName + DeclaresRoot + DeclaresAliasesNothingElse)
      implies all disj x, y: mKey | walk[x, y] not in SAME + DISJOINT
} for 6 but 4 Int

run law_nobody_spoke_declines_premise {
   no InForce & (DeclaresUniqueReferent + DeclaresUniqueName + DeclaresRoot + DeclaresAliasesNothingElse)
   some disj x, y: mKey | knownChain[x] and knownChain[y] and worldOf[x] = worldOf[y]
}

check law_different_sorts_never_same {
   all x, y: mKey | x.scheme != y.scheme implies walk[x, y] != SAME
} for 6 but 4 Int

run law_different_sorts_never_same_premise {
   some x, y: mKey | x.scheme != y.scheme and knownChain[x] and knownChain[y] and worldOf[x] = worldOf[y]
}
```

<!-- prose-translation -->
> The walk answers one of SAME, DISJOINT, KNOWN_UNSPOKEN, or UNKNOWN.
> One level: two levels are SAME iff they are one instance, or they are two mKeys with equal mValues whose shape carries `:guarantees-unique-referent`, or they are two mKeys of one cell mSort (1.9-cell-a-singleton-sort).
> Two chains are aligned-SAME at a pair of levels when the two levels have one height and every pair of levels above them of one height is SAME by the one-level rule.
> Two mFullyQualifiedKeys are SAME iff they are SAME at every level down to the leaf.
> Step 1: if either mFullyQualifiedKey contains an unknown link, the pair reads UNKNOWN; if one terminates at an mWorld the other does not share, the pair reads UNKNOWN.
> Step 2: otherwise walk downward from the top to the deepest level at which the two chains are SAME by the one-level rule, and call that level A; if either mKey is A itself, the pair reads UNKNOWN, since a write to a container collides with everything inside it.
> Step 3: otherwise call the child of A on each side that side's top, the top being the mKey itself when its mParent is A; separation is concluded in one of two ways.
> Two tops: both tops are mKeys of one mScheme, each carrying `:guarantees-unique-name`, with differing mValues.
> One top: exactly one mKey is its own top, and the `resolve()` body of its primary mScheme declares, for some other shape, `:identified-in` the mSort of the other side's top.
> Step 4: the pair reads DISJOINT iff one of the two ways holds and every store strictly below A, down to either leaf's mParent, is `:aliases-nothing-else` (2.3-aliases-nothing-else-the-store-warrant); separation is decided once, at A.
> Otherwise, mKeys of different mSorts read KNOWN_UNSPOKEN, and mKeys of one mSort read UNKNOWN.
> Two mKeys of any mScheme are walked by their identities (3.1-identity-of-a-key); an mKey with no identity reads UNKNOWN.
> `compare(x, y)`: SAME is "or" across mDerivations, the mFullyQualifiedKey walk, a mCorrespondence (2.7-corresponds-across-a-transition), and a mCompositeSort's function of its parts (2.11-composite-sorts-and-roles), and "and" within one mFullyQualifiedKey; SAME composes transitively.
> A warranted SAME and a warranted DISJOINT on one pair is a contradiction: the pair reads UNKNOWN, and the refusal with its attribution is 3.5-committee-law-and-attribution's.
> Otherwise the strongest warranted answer stands: SAME, else DISJOINT, else what the walk answers.
> The two tables hold the engine's walk answer and `compare()` answer over every pair of mKeys and equal those functions pair by pair; they are plumbing the other definitions read, not objects of the model.
> SAME then DISJOINT composes to DISJOINT; DISJOINT then DISJOINT never chains.
> `compare()` never reaches a false SAME while every statement in force is true.
> `compare()` never reaches a false DISJOINT while every statement in force is true and no store is among its own contents.

UNACKED READING, temporary (`312d:ask-contradiction-reads-unknown`, `312d:enc-one-instance-is-one-atom-for-now`,
and the laws' premise): 311 says a contradiction is "refuse both and attribute both authors",
which is not one of the four answers, so the fence answers UNKNOWN there; "one instance" is one
atom, the sharing of 1.10-vantage-route-placeholder-witness being by construction; and every
law premises EVERY statement in force true where 311 says "every statement behind it", the
support of 3.5-committee-law-and-attribution being unused in the premise. The conductor's
readings, not acked, not authoritative, held only until acked or replaced (`notes/312d` § 7).
> Every statement in force is true when each statement in force satisfies its species' truth predicate.
> Every statement in force outside a named set is true when each statement in force outside that set satisfies its species' truth predicate, and the engine's vouch holds; with the empty set named, this is every statement in force being true.
> A store is never among its own contents when no mReferent holds itself, directly or through others.
> The model never reaches a false SAME while every statement in force is true: two mKeys the walk reads SAME reach one mReferent, or both reach none (no counterexample at scope 6 is the claim, never a proof).
> The model never reaches a false DISJOINT while every statement in force is true and no store is among its own contents: two mKeys the walk reads DISJOINT reach no common mReferent.
> Where nobody has spoken, the model declines to answer: with no warrant of any kind in force, two distinct mKeys never read SAME or DISJOINT.
> mKeys of different mSorts never read SAME.

#### § 3.2.1-what-the-answers-mean-to-their-consumers

The consumer map is the sparing test's and the fact-transport's; the flag's gate on a
sentinel-inherited SAME is by construction (3.4-entry-and-lends), and its gate on sparing is in
the test (2.6-may-write-the-writeset). Partial measurement never widening is a statement about
two measurements of one chain, which the fences do not hold. A provider-supplied identifier is
an mKey of a `:root` shape in this model.

<!-- normative -->
> SAME means the fact is about this mKey.
> The engine consumes a SAME that rests on a wrapper's sentinel under `--risk-faultless-skips` (3.4-entry-and-lends).
> DISJOINT licenses sparing under the same flag.
> UNKNOWN and KNOWN_UNSPOKEN are the safe bottoms.
> An omission is a distinction only inside the body that made it.
> No mFullyQualifiedKey speaks across mWorlds; the finished definition does not speak across mWorlds, because its sentence is within-mWorld (2.6-may-write-the-writeset); a mCorrespondence is the one mDerivation that may speak across mWorlds (2.7-corresponds-across-a-transition).
> KNOWN_UNSPOKEN never spares and never transports, whatever either side has declared finished (2.6-may-write-the-writeset).
> Two mSchemes yielding one mKey-Primary is the sole same-referent generator across ways of naming.
> Partial measurement never widens: a mDerivation with an unmeasured or mRoute-terminated link yields at most what it would yield with the link measured.
> A contradiction is refused, and both authors are attributed.
> The universal meet over backing sets is unchanged.
> Genuinely different mKey-Primaries for one mReferent are two mDerivations for one mTopic, reconciled by coherence, never a second mKey inside one mParent.

Two mSorts meeting at a common ancestor: a package status file and a unit file share a filesystem. Genuinely different mKey-Primaries for one mReferent: an NFS filehandle and the server's inode, or a machine-id and a cloud instance-id.

### § 3.3-invalidation-three-mutator-species

Three mutator species invalidate three kinds of fact, and in all three the engine withdraws
authority and never computes the successor identity: below a line, an invalidated
mResolution's mFullyQualifiedKey reads unknown, so SAME loses authority, elisions demote to
guards, and DISJOINT collides. The lines above a site are the mutators that ran before it
(the shared order of lines). The read set of the lookup body is derived from the body, which
the fences do not hold, so whether it is open is an uninterpreted relation with 311's sentence
as its only axiom (`plans/30Z` § 2.6). The state mutation's kill-reach is the sparing test of
2.6-may-write-the-writeset itself, and a lifecycle write to a mRoot-adjacent mKey is caught by
that test as a write to a container, since every mKey scoped in it meets it at itself; what the
fences add is the token that a state mutation to the mParent-Store invalidates.

```alloy
one sig Engine { lookupReadSetOpen: set mScheme }

fun lineWriteset[l: Line]: set mKey { writesetUnexcluded[l] }

pred touchesTraversal[w: mKey, k: mKey] {
   (some l: levelsOf[k] |
      (some m: crossed[l] | tabledCompare[w, m] != DISJOINT)
      or (not traversalClosed[l] and some p: l.parent & mKey | regionTest[p, w] != DISJOINT))
   or some g: (RecordsLookedUpIn & InForce & placedKey.k).inKey | tabledCompare[w, g] != DISJOINT
}

pred routingInvalidatedBy[l: Line, k: mKey] {
   some w: lineWriteset[l] | touchesTraversal[w, k]
   or (some lineWriteset[l] and some (levelsOf[k] + k).scheme & Engine.lookupReadSetOpen)
}

pred tokenInvalidatedBy[l: Line, k: mKey] {
   some w: lineWriteset[l], p: identity[k].^parent & mKey | tabledCompare[w, p] != DISJOINT
}

pred lifecycleInvalidatedBy[l: Line, k: mKey] {
   some w: lineWriteset[l] | some w.parent & mWorld and w in identity[k].^parent
}

pred staleAt[s: Line, k: mKey] {
   some l: s.above | routingInvalidatedBy[l, k] or tokenInvalidatedBy[l, k] or lifecycleInvalidatedBy[l, k]
}

fun compareAt[s: Line, x, y: mKey]: one Answer {
   (staleAt[s, x] or staleAt[s, y]) implies UNKNOWN else tabledCompare[x, y]
}

pred hole_unclosed_traversal_without_a_key_catalog {
   some k: mKey, l: levelsOf[k] | not traversalClosed[l] and no l.parent & mKey
}

run hole_unclosed_traversal_without_a_key_catalog_witness {
   hole_unclosed_traversal_without_a_key_catalog and everyStatementInForceIsTrue and storesAreWellFounded
} for 6 but 4 Int expect 1

pred hole_natural_key_catalog_off_the_route {
   some k: mKey | isNaturalKey[k] and some k.parent & mKey and some k.reaches
      and k.reaches not in k.parent.reaches.passes
}

run hole_natural_key_catalog_off_the_route_witness {
   hole_natural_key_catalog_off_the_route and everyStatementInForceIsTrue and storesAreWellFounded
} for 6 but 4 Int expect 1

check law_unstale_route_is_untouched {
   everyStatementInForceIsTrue and storesAreWellFounded
      and not hole_unclosed_traversal_without_a_key_catalog
      and not hole_natural_key_catalog_off_the_route implies
      all s: Line, k: mKey, l: s.above | not routingInvalidatedBy[l, k] and atMostClosed[l] implies
         no World.lineWrites[l] & passes.(levelsOf[k].reaches + k.reaches)
} for 6 but 4 Int

run law_unstale_route_is_untouched_premise {
   everyStatementInForceIsTrue and storesAreWellFounded
   not hole_unclosed_traversal_without_a_key_catalog
   not hole_natural_key_catalog_off_the_route
   some s: Line, k: mKey, l: s.above |
      not routingInvalidatedBy[l, k] and atMostClosed[l] and some World.lineWrites[l] and some crossed[levelsOf[k]]
}
```

<!-- prose-translation -->
> A routing mutation touches routing mKeys or a mParent-Catalog: a writeset entry touches a mTraversal member when `compare()` answers other than DISJOINT for the pair (3.2-compare-one-chokepoint-four-answers), by the region test where the member is a mParent-Catalog given whole.
> A routing mutation invalidates a mResolution when its mTraversal includes a touched mKey.
> Any write invalidates a mResolution whose read set is open.
> A state mutation reaches every mKey in its writeset (2.6-may-write-the-writeset): ordinary kill-reach.
> A first write can also change an mKey-Primary, so a state mutation whose writeset touches a mParent-Store invalidates the mTokens scoped in it.
> A lifecycle mutation writes a mRoot-adjacent mKey; every mKey-Primary scoped in it names a new mReferent afterward.
> Below a site, an mKey whose mResolution, mToken, or mWorld a line above invalidated is stale, and every mFullyQualifiedKey built on it reads unknown there.
> While every statement in force is true and no store is among its own contents, a line that invalidates no mResolution of an mKey and closed its at-most set writes nothing that a route to that mKey's mReferents passes through.

#### § 3.3.1-what-invalidation-withdraws

<!-- normative -->
> In all three species the engine withdraws authority; it never computes the successor identity.
> A routing mutation also touches shell state a `resolve()` read: a write invalidates the mResolution when a writeset entry `compare()`s other than DISJOINT with a member of the read set of the lookup body that produced it (1.7-resolution-and-its-traversal).
> Dependent SAME conclusions lose authority, dependent elisions demote to guards, and dependent DISJOINT conclusions collide.
> The touched object itself is untouched.
> Creation, deletion, and rename of an mKey are routing writes; they change what the mKey reaches; the mKeys they write are the verb author's at-most claim, and a writeset that omits them is the ordinary at-most omission knife, now visibly covering routing mKeys.
> Cells whose mFullyQualifiedKeys pass through a lifecycle-written mKey are new and unmeasured; cells whose mFullyQualifiedKeys do not are untouched.
> "Keyed by Boot" and "invariant across Boot" are the shape of the mFullyQualifiedKey, not declarations.

Routing mutations: a mount, a symlink replacement, a rename, a user added, a hostname change, a write to any environment variable, cwd, or configuration a lookup reads. `userdel alice; useradd alice` invalidates every mResolution of the old mKey. Lifecycle mutations: a reboot, a re-provision.

### § 3.4-entry-and-lends

Dynamic binding: `parameterize`, `fluid-let`.

The lent instance becomes the ambient mParent for every mKey of a secondary mScheme looked up
in that mParent-Catalog mSort (1.10-vantage-route-placeholder-witness, 1.6-parent-one-per-key),
and leaf mKeys then inherit transitively through their mFullyQualifiedKeys with no further
speech. A wrapper may declare mCorrespondences across the mParent-Catalogs it lends
(2.7-corresponds-across-a-transition). A lend that depends on the guest is
3.4.1-guest-dependent-lends. Arity: per wrapper, per mParent-Catalog mSort, plus the sentinel.
Declared by: the wrapper owner. Default: ⊤, which walls. Consumer: ambient mParent supply.
Danger: a wrong lend measures the wrong mVantage; a wrong sentinel is a wrong SAME, and the
flag prices it.

```alloy
lone sig RiskFaultlessSkips {}

pred flagged { some RiskFaultlessSkips }

sig Wrapper { wrapperOwner: one Speaker }

sig DeclaresLends extends Statement { lendingWrapper: one Wrapper, lentSort: one mSort, lentInstance: one mKey }

sig ClosesLends extends Statement { closedWrapper: one Wrapper }

fact { all d: DeclaresLends | d.speaker = d.lendingWrapper.wrapperOwner }

fact { all d: ClosesLends | d.speaker = d.closedWrapper.wrapperOwner }

fun lent[w: Wrapper, s: mSort]: lone mKey {
   (DeclaresLends & InForce & lendingWrapper.w & lentSort.s).lentInstance
}

fact { all w: Wrapper, s: mSort | lone lent[w, s] }

pred lendsClosed[w: Wrapper] { some ClosesLends & InForce & closedWrapper.w }

pred inherits[v: mVantage] { some v.through and lendsClosed[v.through] and flagged }

fact {
   all v: mVantage, s: mSort | some v.through implies
      v.ambient[s] = (some lent[v.through, s] implies lent[v.through, s]
                      else inherits[v] implies v.enteredFrom.ambient[s]
                      else none)
}

fact {
   all v: mVantage | some v.through implies
      (inherits[v] implies v.route = v.enteredFrom.route else v.route != v.enteredFrom.route)
}

fun keysUnder[w: Wrapper]: set mKey { {k: mKey | k.at.through = w} }

pred true_DeclaresLends[d: DeclaresLends] {
   all k: keysUnder[d.lendingWrapper] |
      isNaturalKey[k] and catalogSortOf[k.scheme] = d.lentSort implies
         k.reaches in d.lentInstance.reaches.passes
}

pred true_ClosesLends[d: ClosesLends] {
   let w = d.closedWrapper {
      all k: keysUnder[w] | isNaturalKey[k] and no lent[w, catalogSortOf[k.scheme]] implies
         k.reaches in k.at.enteredFrom.ambient[catalogSortOf[k.scheme]].reaches.passes
      all k: keysUnder[w], j: mKey | j.at = k.at.enteredFrom and k.scheme = j.scheme and k.value = j.value
         and k.parent = k.at.route and j.parent = j.at.route implies k.reaches = j.reaches
   }
}
```

<!-- prose-translation -->
> `--risk-faultless-skips` is set for a run, or it is not.
> A wrapper's entry `:lends` mParent-Catalog instances for the mParent-Catalog mSorts it perturbs, and nothing else; the wrapper owner declares each lend, one instance per lent mSort, and the completion sentinel.
> An unlent mParent-Catalog mSort is ⊤ under the wrapper: a vantage entered through the wrapper holds no instance for it.
> After the wrapper's completion sentinel, and under `--risk-faultless-skips`, the unlent mSorts and the mRoute inherit the caller's instances instead; otherwise the mRoute is unknown across the two vantages, another mRoute.
> A lend is true when every mKey of a secondary mScheme looked up in the lent mSort under the wrapper reaches what a route through the lent instance's mReferent passes to.
> The sentinel is an at-most claim over every mParent-Catalog mSort and the mRoute: it is true when every such mKey of an unlent mSort under the wrapper reaches what a route through the caller's instance passes to, and when an mKey scoped in the mRoute under the wrapper reaches what its same-spelled twin scoped in the caller's mRoute reaches.

UNACKED READING, temporary (`312d:enc-lends-truth-is-routing`, `312d:enc-vantage-is-the-entry-chain`):
311 says a wrapper lends instances "for the mParent-Catalog mSorts it perturbs, and nothing
else"; the fence reads "perturbs" through the world relation `passes` and reads the mRoute's
inheritance as sameness of what same-spelled route-scoped mKeys reach, and it makes a vantage
not inheriting hold a DIFFERENT mRoute atom. The conductor's readings, not acked, not
authoritative, held only until acked or replaced (`notes/312d` § 7).

#### § 3.4.1-guest-dependent-lends

<!-- normative -->
> A lend may depend on the guest; the wrapper author then declares the guest-insensitive default and supplies a policy read that declines on departure.

Examples: a chroot lends a mount namespace. `sudo -u` lends a user. `ip netns exec` lends a network namespace. A lend that depends on the guest: sudoers matches the guest command.

### § 3.5-committee-law-and-attribution

Every statement species in this document carries one speaker (the shared `Statement`), and
each species' speaker fact names the party who can know it about their own tool, store, or
machine: the engine only chains and meets. What the fences add here is the support of an
answer, the statements it rested on, so that the two shapes of composition are checkable: a
granting composite (a SAME or a DISJOINT) rests on more than one author only where each author
spoke about their own lookup; a withholding composite (an UNKNOWN, a KNOWN_UNSPOKEN, a
collision) names nobody. Attribution renders the support; the aid plane's rendering is
`AID-NEEDS.md`'s. The two-tops way's support is one mScheme's owner by construction; the
one-top way's is one mScheme's owner and the stores' describers on both legs; a SAME's support
is the warrants of every level, the mCorrespondences, and the sentinels that made instances one.

```alloy
fun warrantsOn[k: mLevel]: set Statement {
   (DeclaresUniqueReferent & InForce & referentShape.(k.shape))
   + (DeclaresUniqueName & InForce & nameShape.(k.shape))
   + (DeclaresRoot & InForce & rootedShape.(k.shape))
   + (DeclaresIdentifiedIn & InForce & onShape.(k.shape))
   + (SuppliesParent & InForce & forKey.k)
   + (DeclaresAliasesNothingElse & InForce & store.k)
}

fun chainSupport[x: mKey]: set Statement {
   warrantsOn[fullyQualifiedKey[identity[x]]]
   + (DeclaresYields & InForce & fromShape.((x.*yielded).shape))
   + (DeclaresLends & InForce & lentInstance.(mSort.(x.at.ambient)))
   + (ClosesLends & InForce & closedWrapper.(x.at.*enteredFrom.through))
}

fun sameSupport[x, y: mKey]: set Statement {
   chainSupport[x.sameClosure] + (DeclaresCorresponds & InForce & (keyX + keyY).(x.sameClosure))
}

fun disjointSupport[x, y: mKey]: set Statement { chainSupport[x] + chainSupport[y] }

fun speakersOf[s: set Statement]: set Speaker { s.speaker }

check law_disjoint_by_two_tops_rests_on_one_scheme_owner {
   all x, y: mKey, a, b: mLevel |
      meet[identity[x], identity[y]] = a -> b and separatedAt[identity[x], identity[y], a, b]
         and twoTopsWay[topBelow[identity[x], a], topBelow[identity[y], b]] implies
            one speakersOf[DeclaresUniqueName & InForce & nameShape.((topBelow[identity[x], a] + topBelow[identity[y], b]).shape)]
} for 6 but 4 Int

run law_disjoint_by_two_tops_rests_on_one_scheme_owner_premise {
   some x, y: mKey, a, b: mLevel |
      meet[identity[x], identity[y]] = a -> b and separatedAt[identity[x], identity[y], a, b]
         and twoTopsWay[topBelow[identity[x], a], topBelow[identity[y], b]]
}

```

<!-- prose-translation -->
> The support of a SAME is the warrants at every level of the chains it rests on, the yields that reached them, the lends and sentinels that made instances one, and the mCorrespondences.
> The support of a DISJOINT is the warrants at every level of the two chains.
> A DISJOINT by the two-tops way rests on one mScheme's owner's `:guarantees-unique-name` declarations.

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
> Every positive step is one author's line: a `:yields` and its lookup warrants, a shape's `:identified-in` and its warrants, a may-read set and its sentinel, a may-write entailment and its finished record, a mCorrespondence, an `:observer-independence`, a `:lends`.
> So the speech needed grows with the number of authors, never with the number of pairs of them.
> The engine only chains and meets.
> Every statement an answer rests on is something one party can know about their own tool, store, or machine; that party says it alone, describes nothing they cannot see, and names no other author.
> A granting composite is entailed jointly by one mScheme's `:yields` and the primary mScheme's declaration for that shape; each author speaks about their own lookup.
> A withholding composite names nobody and needs nobody's consent.
> Every survival names the `:aliases-nothing-else` and `:guarantees-unique-name` declarations it rested on, the route closures it rested on (1.5-token-and-the-two-warrants, 2.10-places-the-upward-lookup), and the closed may-read sets its writeset rested on.
> Every SAME names the `resolve()` calls, the declarations, the sentinels and route claims that made instances one, and the mCorrespondences.
> Every invalidated conclusion names the writeset that invalidated it.

A granting composite: "these two accounts are one". A withholding composite: a mount invalidating an account's mResolution.

## § 4-relation-to-other-documents

### § 4.1-boundary-of-this-model

The boundary is prose by nature: it names what other documents own and what this one refuses,
and no fence can hold an absence.

<!-- normative -->
> This model uses, and does not redefine: the verdict, vouch, and guard tier; the authored may-write entries, the completion record as the witness of a finished definition, and `--risk-faultless-skips`; the four-answer chokepoint and its consumer map, except that a sentinel-inherited SAME rides the flag (3.4-entry-and-lends); the universal meet; measure-in-context, entry forms, siting vouches, the escalation dial, and `safe-across` (`plans/27C`); the read-set closure as the falsification net for unmarked reads (`plans/27C` §4(a)(B)); binds as the mKey-minting act; the mPlaceholder and the standup `witness()`; the integrity plane; the committee law.
> The model excludes a selector dialect, an aspect species, an authored region predicate, an engine-side name floor, and an engine table that generates SAME.
> The context slot is a mVantage and nothing else.

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

- `30U:rul-cross-kind-sparing-needs-a-finished-definition`, `30U:the-record` "As a generator",
  and `30U:constraints-on-other-components` "The comparison", with
  `30T:rul-binder-claims-are-ordinary`: a finished definition generates cross-kind
  provably-disjoint verdicts, and a footprint cell is found disjoint from another kind's backing
  cell through it. Here: a finished definition stays necessary for sparing across mSorts and
  generates no DISJOINT (2.6-may-write-the-writeset). `compare()` decides every pair. A
  cross-mSort pair the walk does not separate reads KNOWN_UNSPOKEN, whatever is finished
  (3.2-compare-one-chokepoint-four-answers).
- `ANALYZER-NEEDS:an-kind-reach`, `ANALYZER-NEEDS:an-compare-chokepoint`, and
  `ANALYZER-NEEDS:an-disjointness`: the `unrelated` answer is the cross-kind answer only absent
  the claimed kind's finished definition, and the record licenses cross-kind sparing. Here:
  `unrelated` is KNOWN_UNSPOKEN, and `provably-disjoint` is DISJOINT. KNOWN_UNSPOKEN never
  spares, whatever is finished (3.2-compare-one-chokepoint-four-answers).
- `plans/30W` §1 and §5, with `26Ob:res-per-index-relation-table`: a kind's owner declares the
  kind referent-transparent, one grade under which token equality gives same and token
  inequality gives disjoint. Here: a lookup carries two independent warrants per matched shape,
  `:guarantees-unique-referent` and `:guarantees-unique-name`, each absent by default
  (1.5-token-and-the-two-warrants, 2.2-primary-of-and-identified-in). `:root` is the
  separate per-shape claim of global comparability (§2.2).
- `plans/30W` §1 index-kinds and §10 build item 1, with
  `26Ob:res-worlds-compare-through-the-chokepoint`: the context slot is a product over
  index-kinds, and a world is a coordinate in a cell's key. Here: the context slot is a mVantage
  and nothing else, an address that is part of no mKey's identity
  (1.10-vantage-route-placeholder-witness, 4.1-boundary-of-this-model). Identity is the
  mFullyQualifiedKey (1.8-fully-qualified-key-topic-and-derivation). A world is an mWorld, a
  terminus of a mFullyQualifiedKey. No mFullyQualifiedKey and no finished definition speaks
  across mWorlds. A mCorrespondence may (§1.8, 3.2-compare-one-chokepoint-four-answers).
- `plans/30W` §2 and §3 `kind__disjoint()`, its `30W:rul-disjoint-is-an-rc-predicate` [TYPED],
  and `30T:file-identity` on the region predicate: an owner-authored region predicate generates
  disjointness between regions. Here: there is no authored region predicate
  (4.1-boundary-of-this-model). Containment is membership in a mTraversal. The region test over
  emitted mTraversals (2.9-the-traversal-and-the-region-test) and the `:places` lookup
  (2.10-places-the-upward-lookup) decide it.
- `plans/30W` §2 to §4, `26Ob:res-cell-level-relation-is-the-filtered-meet` and
  `26Ob:10f-the-target-pin`, `plans/27C` §4(A), `ANALYZER-NEEDS:an-invariance-speech-act`, and
  `271:rul-invariance-speech-act` [TYPED]: the kind owner's invariance line
  (`undivided-by-transit-across`, `invariant:<axis>`, `: user-invariant`) licenses transport
  across an index or an axis, and the store member yields invariant, keyed, or ⊤ per (kind,
  selector, index-kind). Here: there is no invariance line and no per-kind table against axes
  (4.1-boundary-of-this-model). Whether a lifecycle write or a lent instance reaches a cell is
  the shape of its mFullyQualifiedKey, not a declaration (3.3-invalidation-three-mutator-species,
  3.4-entry-and-lends). Leaf mKeys inherit across a wrapper with no further speech, under the flag (§3.4).
  The observer half of the line is `:observer-independence` of O, declared per mSort and absent
  by default (2.8-observer-dependence-and-independence). The store half is displaced by
  `:aliases-nothing-else` and measured mTokens (2.3-aliases-nothing-else-the-store-warrant,
  3.2-compare-one-chokepoint-four-answers).
- `notes/272` §3 (the carried-by table and emission-set non-interference) and `plans/27C` §4
  (the who-am-I derivation as contradiction-checker): an engine-owned substrate-by-axis table
  and a taint over who-am-I ingredients derive keying and check declarations. Here: the engine
  holds no table that generates SAME (4.1-boundary-of-this-model). Keying is the
  mFullyQualifiedKey's shape (3.3-invalidation-three-mutator-species). No `resolve()` can measure
  observer-dependence, so it remains speech (2.8-observer-dependence-and-independence). The
  contradictions the engine refuses are two seats disagreeing on an mParent instance
  (1.6-parent-one-per-key), a warranted SAME against a warranted DISJOINT
  (3.2-compare-one-chokepoint-four-answers), and two disagreeing answers from one placing lookup
  (2.10-places-the-upward-lookup).
- `notes/272` §5 the fence: emitted locators feed only the dependence bit and the keying recipe,
  and are never compared against File facts. Here: a may-read entry is an mKey that `compare()`s
  against every writeset entry, and an entry naming a store reaches every mKey relative to that
  store (2.5-may-read-the-readset, 2.6-may-write-the-writeset).
- `30T:file-identity` per-aspect identity: "same file" is one relation per aspect, and the
  identity tier carries an authored per-aspect relation mapping. Here: there is no aspect
  species (4.1-boundary-of-this-model). Each aspect is a cell, a singleton mSort with its own
  may-read set and its own `:observer-dependence` (1.9-cell-a-singleton-sort).
  Same-for-existence and same-for-contents are two facts in the filesystem describer's
  vocabulary. Contents is a fact about the inode the path mScheme yields
  (2.3-aliases-nothing-else-the-store-warrant). Existence is a fact about the directory, or
  about an mReferent keyed by name in it where the describer mints one. Creation, deletion, and
  rename write the one the describer named (3.3-invalidation-three-mutator-species).
- `30T:file-identity` the v0 floor: entry-mutating verbs make no at-most claims, and same-kind
  path-distinct comparisons answer unknown. Here: creation, deletion, and rename of an mKey are
  routing writes, and the mKeys they write are the verb author's at-most claim
  (3.3-invalidation-three-mutator-species). Path-distinct mKeys separate under
  `:guarantees-unique-name` and `:aliases-nothing-else`
  (3.2-compare-one-chokepoint-four-answers).
- `plans/30W` §4 "Containment among index-kinds": containment among stores is a `reaches`
  relation their owners declare. Here: containment among stores is the mParent chain
  (1.6-parent-one-per-key, 1.8-fully-qualified-key-topic-and-derivation). The may-write
  entailment carries effects, never identity (2.6-may-write-the-writeset).
- `notes/277` §3 (the selector dialect, the survival-license algebra), `plans/30J` §12 (dialect
  keying), and the `spike/CLAUDE.md` sparing-algebra invariant: a same-entity claim spares a
  backing only when both carry minted selectors of one dialect and the two selectors differ.
  Here: the model excludes a selector dialect (4.1-boundary-of-this-model). Two cells
  of one mParent are two mSorts. They separate only as 3.2-compare-one-chokepoint-four-answers
  separates any two mSorts (1.9-cell-a-singleton-sort).
- `spike/CLAUDE.md` compare-consumer-map and
  `311a:note-transport-single-consented-sparing-double`: every SAME is vouch-tier and unflagged.
  Here: the engine consumes a SAME that rests on a wrapper's sentinel under
  `--risk-faultless-skips` (3.4-entry-and-lends, 3.2-compare-one-chokepoint-four-answers). Every
  other SAME generator is vouch-tier.
- `KNOBS:kSURVIVAL` and `ANALYZER-NEEDS:an-mode-gate`: the flag gates the survival tier's
  sparing. Here: the flag also gates a SAME that rests on a wrapper's sentinel
  (3.4-entry-and-lends).
- `USER_STORY.md`, the bought-unsoundness section: past the flag the admin trusts named authors'
  at-most claims, and everywhere else only measurements. Here: past the flag the admin also
  trusts wrappers' sentinels (3.4-entry-and-lends).

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

- ADDR: T's mKey means something only relative to P.
- WRITE: a write to P can change T, and P is not on T's chain.
- ROUTE: T is reachable through P.
- VANT: an answer about T depends on where the read ran.
- ATTEST: an answer about T is A's word.

The levels of ADDR and ROUTE:

- KEY: one mReferent, one mKey, inside one store.
- STORE: one mReferent, one store, across stores.
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

OPEN marks a cell with no statement in the model. The root naming file holds its candidate
names. TABLED marks a row that the ledgers set aside (`311t` § 13). A dash marks a cell with no
statement, where the default does the work.

| row    | level | end  | entry                                                            | closure                                                                  | licenses     | §             |
| ------ | ----- | ---- | ---------------------------------------------------------------- | ------------------------------------------------------------------------ | ------------ | ------------- |
| ADDR   | KEY   | P    | the lookup yields the canonical mKey [shape]                     | `:guarantees-unique-name` [shape] · `:guarantees-unique-referent` [shape] | DISJ · SAME  | 1.5, 2.1, 3.2 |
| ADDR   | KEY   | T    | `alias k'` [key]                                                 | `alias nothing-else` [key, level]                                        | DISJ         | 1.5, 2.9      |
| ADDR   | STORE | P    | `:corresponds`[^corr]                                            | `:aliases-nothing-else` [store]                                          | DISJ         | 2.3, 2.7, 3.2 |
| ADDR   | STORE | T    | `:identified-in` [shape] · `:corresponds`[^corr]                 | OPEN[^open] · `:root` [shape]                                            | — · SAME     | 1.6, 2.2      |
| WRITE  | —     | T    | `may-read` [sort] · the marked reads [body]                      | `may-read nothing-else` [sort] · the vouch [body]                        | SPARE        | 2.5           |
| WRITE  | —     | P    | the entailment [sort, shape] · the at-most set [verb, shape]     | the finished record [sort, shape] · the completion record [verb, path]   | SPARE        | 2.6           |
| ROUTE  | CAT   | T    | the emitted mTraversal [lookup, shape]                           | the closing act[^act] · `alias nothing-else` [level]                     | INVAL · DISJ | 1.7, 2.9      |
| ROUTE  | CAT   | P    | —                                                                | —[^floor]                                                                | —            | 2.9           |
| ROUTE  | XCAT  | T    | `looked-up-in G:key` [key][^placing]                             | `looked-up-in nothing-else` [key, route-sort]                            | DISJ · INVAL | 2.10          |
| ROUTE  | XCAT  | P    | `:places` [G, T] · the finished entailment's members[^shared]    | the finished record[^shared]                                             | SPARE        | 2.10          |
| VANT   | —     | T    | —[^dep]                                                          | `:observer-independence` of O [sort, O]                                  | SAME         | 2.8           |
| VANT   | —     | W    | `:lends` [wrapper, sort] · `:corresponds` [lent catalogs]        | the completion sentinel [wrapper]                                        | SAME         | 3.4           |
| VANT   | —     | E    | —                                                                | the local-route vouch [span]                                             | SAME         | 1.5, 1.10     |
| ATTEST | —     | T, A | TABLED                                                           | TABLED                                                                   | —            | `311t` § 12   |

[^corr]:
    `:corresponds` is the transition owner's statement (2.7-corresponds-across-a-transition).
    The table files it at both ends of the STORE level, as the positive twin of each closure
    (`311t` § 15). It generates SAME, so this entry is itself a knife (5.2).

[^open]:
    The thing's end has no closure at the STORE level: "this mReferent has no other home".
    `311t` § 15 argues that the cell may stay empty, because the cases that need it decline and
    read UNKNOWN.

[^act]:
    The model gives the closing act no spelling. The `looked-up-in` records of
    2.10-places-the-upward-lookup are its natural form (`312cg` § 18).

[^floor]:
    A catalog makes no statement about its own entries. A writeset entry that names the catalog
    whole covers every mResolution through it (2.9-the-traversal-and-the-region-test).

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
entries. Each picks which mReferent or which instance is meant, so a false one keys a fact to
the wrong thing.

The kinds:

- route: picks a thing or an instance.
- warrant: a per-shape grade on a lookup.
- closure: a per-key statement, made on the path that measured the mKey.
- sentinel: the closing act on a declared set.
- record: the closing act on an emitted set, at a body's tail.
- vouch: one party's statement that stands for a set of measurements.

The built column says when the statement comes into being:

- decl: declared, before any lookup runs.
- eval: constructed on the path that measured the mKey.

The if-false column names the wrong answer:

- wSAME: one fact stands for another thing's fact.
- wDISJ: a license survives a write that destroyed it.
- wSPARE: the same survival, reached through the sparing test.
- stale: a mResolution stands after a write that should have invalidated it, and every
  conclusion built on it stands with it.
- vantage: the engine keys a fact at the wrong instance.

The flag column says whether the engine consumes the answer only under `--risk-faultless-skips`
(3.2-compare-one-chokepoint-four-answers, 3.4-entry-and-lends). An INVAL cell marked "no"
inherits the flag of the consumer that a kept mResolution feeds, since a stale mResolution
reaches a decision only past a running line. The consumer column orders the rows. Within one
consumer, the unflagged rows come first.

| statement                              | kind     | speaker              | grain           | built      | consumer     | if-false        | flag     | §             |
| -------------------------------------- | -------- | -------------------- | --------------- | ---------- | ------------ | --------------- | -------- | ------------- |
| `:guarantees-unique-referent`          | warrant  | the lookup's owner   | shape           | decl, eval | SAME         | wSAME           | no       | 1.5, 2.2      |
| `:root`                                | warrant  | T's owner            | shape           | decl       | SAME         | wSAME           | no       | 2.2, 3.2      |
| `:corresponds`                         | route    | the transition owner | pair            | decl       | SAME         | wSAME           | no       | 2.7           |
| `:observer-independence` of O          | warrant  | T's owner            | sort, O         | decl       | SAME         | wSAME           | no       | 2.8           |
| the local-route vouch                  | vouch    | the engine           | span            | —          | SAME         | wSAME           | no       | 1.5, 1.10     |
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
documents, `plans/`, `spike/` code and its `CLAUDE.md` files) get one commit that makes them
current: a mechanical replacement, or a Sonnet pass. Historical documents (`notes/`) will likely
keep the dead name. A slug is the exception: when a slug is renamed in a new or important
document, every reference to that slug is renamed everywhere, historical documents included.

The r31 ledgers are frozen and historical. They record many of these renames as they happened,
so they carry the old words unchanged. Non-ledger content will probably be updated.

| in the r31 ledgers | here |
|---|---|
| mKey-CatalogStore / mKey-PrimaryStore | mParent: one edge per mKey; derived views mParent-Catalog (through a secondary mScheme; routing) and mParent-Store (through the primary mScheme; identity) |
| mSort-CatalogStore / mSort-PrimaryStore | none: the mParent's mSort is declared per matched shape on the primary mScheme (`:identified-in`) |
| naming system (described, never named) | mScheme (owns the `resolve()`; what every bind and mark names) |
| `:named-in` | `:yields` (per matched shape, into any mSort) |
| `:identified-in` (one per mSort) | `:primary-of` on the mScheme, with `:identified-in` and the warrants per matched shape |
| mAspectSort, `:named-like` | none: a cell is a singleton mSort under its mParent (1.9-cell-a-singleton-sort) |
| mNaturalKey / mPrimaryKey | mKey-Natural / mKey-Primary (the ledgers and exercises keep the old order as an acceptable gloss) |

Outside r31, the last bullet of `AGENTS.md`'s "Terminology firming" still names mKey-CatalogStore,
mKey-PrimaryStore, mSort-CatalogStore, and mSort-PrimaryStore as this model's words.

Plain-English "read" that must NOT be tagged or renamed (a host read, not the operation):
312b-exercises/01 ~line 54; 312b-exercises/02 ~lines 52 and 65.

| dead name | name here | where the dead name remains |
|---|---|---|
| mPlacement · `:lives-in` · "placement"; and "backing" (`an-backing-selfframing`, the probe's marked read set): the declared and the measured halves of one side | `may-read` (the record verb / relation); Readset (the set: the declared `may-read` entries, the probe's marked reads, and the Readsets of every container on the chain). Sentinel `may-read nothing-else` | `ANALYZER-NEEDS` rows, including the slug `ANALYZER-NEEDS:an-backing-selfframing` (also cited from `plans/30T`); `USER_STORY`; `KNOBS` |
| footprint · `disturbs` · "at-most claim"; and `:reaches` / `disturbance_reaches` (the entailment, computed into the same set) | `may-write` (the record verb / relation); Writeset (the set: the declared `may-write` entries, the entailment, and the may-write sets of the written thing's containers strictly below the level shared with the fact's key). Sentinel `may-write nothing-else` (the finished definition / completion record) | `USER_STORY` stages 5–7; `KNOBS:kBURDEN`; `ANALYZER-NEEDS`; `FORFEITS`; the spike members `cmd__disturbs()` and `kind__disturbance_reaches()`, which follow the verb |
| "perishing" and its forms, used as jargon | no term: the jargon retires. Plain English and standard compiler-engineering terminology, usually but not always a phrase with the word "invalidation". Never a capitalised or tagged form (no "Invalidation", no mInvalidation). Where it becomes "invalidation", it nearly always needs a precise subject; for how "perishing" was used, the human believes that is probably "routing invalidation", unchecked | not yet investigated |
| `unrelated` | KNOWN_UNSPOKEN | `30U` § 7, `compare-consumer-map`, `ANALYZER-NEEDS:an-compare-chokepoint`. The design-of-record documents keep `unrelated` until the model is ruled (4.2-supersessions-pending-in-prior-documents names them) |

### § 6.2-undecided-whether-a-name-is-dead

§ 1 to § 5 still use these words. The table in 6.1-applied-here-not-yet-applied-elsewhere lists
each as a dead name. Whether that retirement covers these uses is undecided.

- "backing" for a mResolution's mTraversal (1.7-resolution-and-its-traversal, and the consumer
  line of 2.9-the-traversal-and-the-region-test), and "backing sets" in
  3.2-compare-one-chokepoint-four-answers. The dead "backing" is the probe's marked read set.
- "at-most claim" for the mKeys that a verb's author declares it writes
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
| the identifying-store object (the parent named by `:identified-in`) | "store" is provisional; this model writes mParent-Store. One word is owed; whether mParent-Store discharges it is unacked |
| the spike's `kind__` API prefix | follows mSort, or the mScheme as the exercise strawmen have it (`sm_Path__resolve()`); untouched in code and pre-311 documents; strawman-tier |
| `:root` | its name is unacked; where it sits is undecided (`311p` thread 7) |
