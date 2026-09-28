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
is residue the checker cannot reach. Everything else, this running text included, is commentary.
An un-headed blockquote is commentary: under a heading it names a false friend from a
neighbouring field, a name the object carried before this document, or a refuted shape
(`notes/311u`); at the end of a section it holds the section's examples, illustrative and not
fully worked. The results of every command are in `311-identity.lock.json` beside this file;
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
- A blockquote under a heading is non-normative. It names a false friend from a neighbouring
  field, a name the object carried before this document, or a refuted shape (`notes/311u`).
- A blockquote at the end of a section is non-normative. It holds the section's examples.
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

The laws that state this for `compare()` are the checks of 3.2-compare-one-chokepoint-four-answers;
who must say what is 3.5-committee-law-and-attribution. The law itself:

<!-- normative -->
> The true answer is reachable once those who can know have spoken.
> Where nobody has spoken, the model declines to answer.
> The model never reaches a false answer while every statement behind it is true.
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

## § 1-the-model-objects

### § 1.1-referent-state-and-value

The world stratum of 0.1-the-two-strata: an mReferent, what a store holds, and what it holds by
its own construction against what it merely re-presents (the distinction 2.3-aliases-nothing-else-the-store-warrant
needs). The engine never reads these relations; the truth predicates do.

```alloy
sig mReferent { holds: set mReferent, owns: set mReferent }

fact { owns in holds }

fact { all r: mReferent | some reaches.r }

fun sortsOf[r: mReferent]: set mSort { primaryOf[(reaches.r).scheme] }

fun partsOf[r: mReferent]: set mReferent { r.holds }
```

<!-- prose-translation -->
> An mReferent is a persisting piece of the world.
> A store is an mReferent; what a store holds is identified in it; what a store owns it holds by its own construction.
> An mReferent has the mSort of the mKey that reaches it, and two mSorts over one piece of the world is the strangers case (1.2-sort-the-declared-carrier).
> It has one or more mKeys.
> It may have parts, and every part is an ordinary mReferent of an ordinary mSort identified in it (1.9-cell-a-singleton-sort).
> It is not an mKey, and not an mSort.

Every identity question is ultimately "do these two mKeys reach one mReferent"; the mKey's
side of that question is 1.4-key-and-its-two-views. No other term names a part. An mReferent is
not an mTopic (1.8-fully-qualified-key-topic-and-derivation), and not the set an mKey given
whole denotes: the container is the mReferent, and the set is a set
(2.9-the-traversal-and-the-region-test).

> Examples of an mReferent: an inode, a database row, a package record, a kernel parameter, a
> running process, a machine, a mount table.

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

> Many-sorted logic's carrier. Never a PLT kind. Never "the kind of thing". Pre-311 documents
> write _kind_.

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

> The strangers case: two vendors describe one tool, or two vocabularies reach one cell under
> `/proc/sys`.

### § 1.3-scheme-a-way-of-writing

> A term language over a carrier. Never itself an mSort.

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

### § 1.4-key-and-its-two-views

> RDBMS primary key and natural key, with their culture: the natural key is user-typed, may
> alias, and is never identity. The primary key is what the store answers with. Pre-311
> documents write _entity_.

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
   scheme: one mScheme,
   shape: lone mShape,
   parent: lone mLevel,
   reaches: lone mReferent
}

fact { all k: mKey | k.shape.ofScheme in k.scheme }

fact { all a, b: mKey | a.scheme = b.scheme and a.value = b.value implies a.shape = b.shape }

fun keysOfShape[s: mShape]: set mKey { shape.s }

fun keysOfSort[k: mSort]: set mKey { {x: mKey | primaryOf[x.scheme] = k} }

pred isNaturalKey[k: mKey] { not isPrimary[k.scheme] }

pred isPrimaryKey[k: mKey] { isPrimary[k.scheme] }
```

<!-- prose-translation -->
> An mKey has three parts: an mValue, its mScheme, and its mParent (1.6-parent-one-per-key).
> The mScheme is always declared; there is no default mScheme, and a bind or a mark always names one.
> The mValue is a literal: the shell word bound at the bind.
> The mParent is the instance one of the seats of 1.6-parent-one-per-key supplies, or none, which leaves the mFullyQualifiedKey unknown from that level.
> A lookup chooses among the shapes its mScheme declares, never outside them: the shape an mKey matches is a shape of its own mScheme.
> Which shape an mValue matches is a function of the mKey's own bytes: two mKeys of one mScheme with equal mValues match one shape (1.6-parent-one-per-key).
> An mKey reaches one mReferent, or none (2.2-primary-of-and-identified-in).
> mKey-Natural is an mKey of a secondary mScheme, what tool authors and books write; mKey-Primary is an mKey of the primary mScheme, meaningful only relative to its mParent-Store.

### § 1.5-token-and-the-two-warrants

> OWL's inverse-functional and functional properties, spelled out by direction.

A mToken is an mKey-Primary's mValue: bytes a `resolve()` returned. It is compared for equality
only and never decoded. It is always scoped in an mParent.

Any lookup, a secondary mScheme's or a primary mScheme's, may carry two warrants per matched
shape. They are independent, separately declared, and absent by default.

- `:guarantees-unique-referent`: within one mParent, equal mKeys reach one mReferent. The lookup
  is a function. It licenses SAME from equality.
- `:guarantees-unique-name`: within one mParent, one mReferent has one mKey. It licenses
  DISJOINT from inequality.

The lookup's owner declares each warrant per matched shape. The warrant holds for every mKey of
that shape inside any one mParent, while that mKey's mResolution or mToken stands
(3.3-invalidation-three-mutator-species). A matched shape is a control-flow path of the owner's
body. A warrant is constructed when that body is evaluated for the mKey in hand. The declaration
is per shape. The instance is per evaluation. It may rest on what the path measured. Any path
the dialect admits may decline it. A path that does not reach a warrant has not given it.

The engine vouches for one lookup itself: the local mRoute under no wrapper
(1.10-vantage-route-placeholder-witness). Across a wrapper, the wrapper's author speaks
(3.4-entry-and-lends), and everything else is measured and witnessed.

Separately from the per-shape warrant, a lookup may emit a closure, `alias nothing-else`, for
each level it resolved. The closure states that the mReferent at that level is reachable by
exactly this one entry anywhere in the instance the lookup ran in, not only in the
mParent-Catalog the entry was found in. It is a statement about one mKey, from the thing's end,
made on the path that measured it. Where the lookup knows other entries, it emits them first,
and a listed alias is checked as the first entry is. The closure and the per-shape warrant are
two statements. The region test (2.9-the-traversal-and-the-region-test) consumes the closure.

> `:guarantees-unique-referent` fails for round-robin lookups, recycled mKeys, and cloned
> identifiers presented as mRoots. `:guarantees-unique-name` fails for symlinks and hardlinks,
> package `provides`, and route-qualified handles. For a path the closure's instance is the
> whole mount namespace. For a file, the evidence is a link count of one. For a directory, the
> evidence is that no other mount exposes it. A hardlink or a bind mount is where the closure is
> withheld.

### § 1.6-parent-one-per-key

Every mKey has exactly one mParent: the mKey it was resolved inside. An mKey may carry further
routes, one per other lookup that reached it (2.10-places-the-upward-lookup). None of them is
its mParent. What the mParent edge carries follows the mScheme.

- Through a secondary mScheme, the mParent is the mParent-Catalog: the thing the mKey was looked
  up in. The edge carries routing: which instance, and invalidation
  (3.3-invalidation-three-mutator-species). It never carries identity.
- Through the primary mScheme, the mParent is the mParent-Store: the mKey relative to which
  alone the mKey-Primary means anything. The edge carries identity.
  It is what `compare()` walks (3.2-compare-one-chokepoint-four-answers). At the primary
  mScheme, mParent-Catalog and mParent-Store are one mKey.

Through the primary mScheme, that mScheme's owner declares the mParent's mSort
(`:identified-in`) and the warrants (1.5-token-and-the-two-warrants, and `:root`,
2.2-primary-of-and-identified-in). The declaration is per matched shape of the mKey's mValue, a
function of the mKey's own bytes. So an emitter (a secondary mScheme, possibly a stranger's) can
be wrong only about what it supplied: its lookup, and the mParent instance where it is the seat
that supplied it.

The mParent instance is an mValue supplied by exactly one of three seats. Each seat names it as
an mKey of one of the mParent's mSort's mSchemes:

- the bind that minted the mKey (1.4-key-and-its-two-views)
- the lookup that yielded it (2.1-yields-into-another-scheme)
- the primary mScheme's declaration for the matched shape (2.2-primary-of-and-identified-in)

Two seats that disagree are a contradiction. It is refused and attributed to both.

A shape with no `:identified-in` is scoped in the mRoute
(1.10-vantage-route-placeholder-witness). A mRoot is a shape declared `:root`. Its mKeys
need no mParent, which claims they are globally comparable. Nothing is a mRoot by default.
Cloned identifiers are the standing witness.

> Examples of a mParent-Catalog: a directory for a path's entry, a passwd database for a login
> name, a process table for a pid. Examples of a mParent-Store: a DNS zone for a record's owner
> name, a user namespace for a uid, a dpkg database for a canonical package name. Examples of a
> mRoot: the DNS mRoot, or a cloud instance-id whose issuer never repeats one.

### § 1.7-resolution-and-its-traversal

A mResolution is the fact that mKey N of mScheme S, resolved inside mParent P at program point
p, reaches mReferent R. It is a fact with a backing. The backing is the mTraversal: the ordered
chain of routing mKeys the lookup crossed. The lookup emits that chain, one member per routing
mKey, in the order it crossed them (2.9-the-traversal-and-the-region-test). The emission is an
at-most set. The lookup's owner closes it by an explicit act. A lookup that emits no member has
its mParent-Catalog, given whole, as its mTraversal. A lookup that emits members without the
closing act has those members and its mParent-Catalog, given whole.

Under ordinary effective-mWorld reach, any mutator whose writeset touches a mTraversal member
invalidates the mResolution. Its target object is not its backing. An mKey can stop reaching an
object without the object changing. An object can change without its mKey changing.

A mResolution also depends on the read set of the lookup body that produced it. The engine
derives that set from the body. Shell parity supplies the reads of sh constructs. The path
mScheme supplies the reads of a path. The speech that describes an external command supplies
the reads of that command.

The read set is closed only when the read set of every external command in the body is closed.
The author of the speech that describes an external command closes that command's read set by
an explicit act. Any write invalidates a mResolution whose read set is open
(3.3-invalidation-three-mutator-species).

> Examples of a mTraversal: each directory entry and symlink for a path, the resolver
> configuration and mVantage for a hostname, the unit table for a service name.

### § 1.8-fully-qualified-key-topic-and-derivation

A mFullyQualifiedKey is the recursive identity of an mKey: its mKey-Primary scoped in its
mParent, whose identity is itself a mFullyQualifiedKey through the mParent's own primary
mScheme. The recursion terminates at a mRoot, at the mRoute
(1.10-vantage-route-placeholder-witness), or at an unknown link. A mFullyQualifiedKey is one
mDerivation of identity.

An mWorld is a terminus of a mFullyQualifiedKey. Each mRoot shape is one mWorld. Each mRoute is
one mWorld. No mFullyQualifiedKey and no finished definition speaks across mWorlds. A
mCorrespondence may speak across mWorlds (2.7-corresponds-across-a-transition,
3.2-compare-one-chokepoint-four-answers).

A mTopic is what a claim is about: the mKey read, plus the observer instance when that mKey's
mSort is observer-dependent (2.8-observer-dependence-and-independence). A mTopic may carry
several mDerivations with different generators: its mFullyQualifiedKey, a provider-supplied
identifier, or a mCorrespondence from a transition owner (2.7-corresponds-across-a-transition).
mDerivations combine by coherence (3.2-compare-one-chokepoint-four-answers), never by priority.

### § 1.9-cell-a-singleton-sort

> Pre-311 documents write _aspect_.

A cell is a singleton mSort identified in its mParent. Its owner declares it `:identified-in`
the mParent's mSort (2.2-primary-of-and-identified-in). Under any one mParent instance it has
exactly one mKey, written `parent-key@sm.Sort`, with the mSort's name in full reverse-DNS. That
mKey is minted at the mark that names it (1.4-key-and-its-two-views, 1.6-parent-one-per-key).
The singleton mSort has no mScheme of its own. The mScheme left of `@` is the mParent's. A
cell's identity is its mParent's plus its mSort (3.1-identity-of-a-key).

A cell's mReferent may hold its mState elsewhere than in its mParent, and the mState may be
diffuse. The cell's may-read set (2.5-may-read-the-readset) says where that mState is held. The
freshness of a fact about the cell follows the writesets that reach the cell through that set
(2.6-may-write-the-writeset), together with any write that covers the mParent
(2.9-the-traversal-and-the-region-test, 3.3-invalidation-three-mutator-species).

Two cells of one mParent are two mSorts, with two may-read sets and two `:observer-dependence`s.
3.2-compare-one-chokepoint-four-answers decides between them as between any two mSorts. A
writeset entry naming the mParent covers its cells (§3.2, step 2). The marked line that answers
a cell is a read of the cell's mKey. The fact's identity is the mTopic
(1.8-fully-qualified-key-topic-and-derivation).

> Example: `active` is held in the service manager's memory in the boot, and a reboot reaches it
> through its may-read set. `enabled` is held in a symlink in a filesystem, and survives a reboot
> only where its mParent is not itself scoped in the boot (3.3-invalidation-three-mutator-species).

### § 1.10-vantage-route-placeholder-witness

A mVantage is the address a probe reached an mReferent from: the mEntryChain, a finite map from
mParent-Catalog mSorts to the instances wrappers lent (3.4-entry-and-lends). It is not part of
any mKey's identity. It does three things:

- It says where a `resolve()` executes.
- It supplies the ambient mParent for every mKey of a secondary mScheme looked up in a lent
  mParent-Catalog mSort.
- For a shape with no `:identified-in`, it is the mRoute: the last-resort mParent.

The mRoute is an address. It holds no mState and declares no may-read set. A readset member
whose mFullyQualifiedKey ends at it is ⊤ (2.5-may-read-the-readset).

For execution under no wrapper, the engine itself vouches the mRoute and the ambient mParent
instances within one unwalled span. Under a wrapper, the wrapper's sentinel supplies the
inherited instances that 3.4-entry-and-lends admits, under `--risk-faultless-skips`. Each is
resolved once per mEntryChain and shared: one mPlaceholder.
Two same-spelled leaf mKeys are two mPlaceholders, SAME only by warrant
(3.2-compare-one-chokepoint-four-answers). Two mVantages share the mRoute and the ambient
instances through a wrapper's sentinel under `--risk-faultless-skips` (3.4-entry-and-lends).
Otherwise the mRoute and the ambient instances are unknown across mVantages.

A mFullyQualifiedKey whose mTokens are not yet measured is a mPlaceholder keyed by (mKey,
ambient mParents, mEntryChain). The probe standup binds it. The apply standup re-reads it
through the same entry and `compare()`s the two. That re-read is the `witness()`. A mismatch is
integrity, never a verdict input. The `witness()` cannot see a recycled mKey. That stays on the
outside-churn horizon.

> A recycled mKey the `witness()` cannot see: a reissued pid or inode.

### § 1.11-site-and-claim-species

A mSite is within a book line. It has an argv, an mEntryChain, and a program point. Speech at or
about a mSite has one author per claim. The claim species are:

- a verdict fact: the measured answer to a read of a cell. Its readset is the body's marked
  reads. The tool-oracle author vouches it.
- a writeset claim: an at-most may-write set per matched shape, with its completion witness. The
  tool-oracle author or the filesystem binder makes it.
- a may-write entailment (2.6-may-write-the-writeset), by the mSort owner.
- a mCorrespondence (2.7-corresponds-across-a-transition), by the transition owner.
- the per-mScheme declarations (2.1-yields-into-another-scheme,
  2.2-primary-of-and-identified-in, 2.9-the-traversal-and-the-region-test).
- the per-mSort declarations (2.5-may-read-the-readset,
  2.8-observer-dependence-and-independence, 1.9-cell-a-singleton-sort, and `:places` with its
  lookup, 2.10-places-the-upward-lookup).
- the wrapper's `:lends` and their sentinel (3.4-entry-and-lends).

Every warrant is typed speech, never an exit status.

## § 2-the-model-relations

Each relation states its arity, who declares it, its default, which consumer reads it, and which
warrant makes it dangerous.

### § 2.1-yields-into-another-scheme

mScheme S `:yields` mScheme T, per matched shape, where T is of any mSort. S's `resolve()`, run
in the mVantage, maps an mKey of S to an mKey of T. It may supply that mKey's mParent instance
(1.6-parent-one-per-key). T is a primary mScheme, or a secondary mScheme that in turn yields
one. The chain always terminates at a primary mScheme.

Where S's own mKeys are looked up is S's mParent-Catalog. It is an instance supplied by exactly
one of three seats, as a store's is (1.6-parent-one-per-key). Each seat names it as an mKey of
one of that mSort's mSchemes:

- the bind that minted the mKey
- S's owner's declaration
- the mEntryChain's instance for that mSort (1.10-vantage-route-placeholder-witness)

An unknown input makes the instance unknown.

S's lookup warrants (1.5-token-and-the-two-warrants) govern what equality and inequality of S's
mKeys license before the primary mScheme is reached. They never license across mParent-Catalogs.
A `resolve()` declines on mReferents its mSort does not describe. This is the mechanical net
against lazy borrowing.

- Arity: per matched shape of a secondary mScheme, into any mSort.
- Declared by: S's owner.
- Default: none. An mScheme that declares neither `:yields` nor `:primary-of` is a floor primary
  mScheme (1.3-scheme-a-way-of-writing).
- Consumer: mResolution (1.7-resolution-and-its-traversal) and the mFullyQualifiedKey
  (3.1-identity-of-a-key).
- Danger: the lookup warrants. A wrong yield or a wrong supplied instance is a wrong SAME or
  DISJOINT, attributed to the yield.

> A cache and the file it caches are two mParent-Catalogs. A decline: a path reaching a socket,
> under an mScheme into files. Two mSchemes over one spelling, into two mSorts: a path yields the
> directory entry, or the inode that the entry leads to. A tool that removes the entry binds under
> the first, and a tool that changes the file binds under the second.

### § 2.2-primary-of-and-identified-in

> ER's identifying relationship. Identifying is not containing.

mScheme P is `:primary-of` mSort K, at most once per mSort. A second name is a second mScheme.
P's mKeys mean something only relative to K's mParent-Store. P's `resolve()` is the identity on
the mKey. An mKey of P names the mReferent that K's mParent-Store holds under it, or none. For a
`:root` shape, the world is the store. The mKey is a level of a mFullyQualifiedKey in both cases
(1.8-fully-qualified-key-topic-and-derivation). A store that generates its mKeys at creation
holds an mReferent under a generated mKey while that mReferent exists. A store that admits mKeys
as names may hold none under an mKey a book names.

P's owner declares, per matched shape of the mKey's mValue:

- `:identified-in` mSort M: the mParent's mSort for mKeys of that shape.
- `:guarantees-unique-referent` and `:guarantees-unique-name` (1.5-token-and-the-two-warrants),
  absent by default. They govern what mToken equality and inequality license at this level of a
  mFullyQualifiedKey.
- `:root`, absent by default. The shape declares no mParent and thereby claims global
  comparability. It is equivalent to `:guarantees-unique-referent` over the whole world. A
  mToken duplicable across instances of its would-be mParent must be scoped in something
  smaller, or left un-warranted.

The mParent's type varies per shape. So the child mSort's owner never learns the mParent's
types. The mParent mSort's primary mScheme classifies, one level up. Each owner speaks one
level. An mValue matching no declared shape reads unknown from this level.

A grade governs every consumer of the answer it grades, corroboration and contradiction
included. A lookup without `:guarantees-unique-name` cannot contradict anything by returning two
different mTokens.

- Arity: at most one mScheme per mSort.
- Declared by: the mSort's owner, on the primary mScheme.
- Default: none. The floor of 1.3-scheme-a-way-of-writing supplies an unwarranted identity
  primary mScheme.
- Consumer: identity (3.1-identity-of-a-key).
- Danger: `:guarantees-unique-referent`, `:guarantees-unique-name`, and `:root`, per matched
  shape.

> `:root` fails for cloned identifiers. The mParent's type varies per shape: an ext4
> filesystem in the mRoute, an NFS filesystem in a host, a tmpfs in a boot.

### § 2.3-aliases-nothing-else-the-store-warrant

A store is `:aliases-nothing-else` when nothing identified in it is, by the store's own
construction, also identified in another store. The store gives its own mKeys to no other
store's mReferents.

The warrant is self-knowledge. Whether another store aliases this one is not claimed, since
nobody can know it. 3.2-compare-one-chokepoint-four-answers asks the warrant of every store on
both legs, so an aliasing store blocks separation by its own silence.

- Arity: per store.
- Declared by: whoever describes the store.
- Default: absent. Nothing inside that store then separates from anything outside it.
- Consumer: DISJOINT (3.2-compare-one-chokepoint-four-answers).
- Danger: an aliasing store declared `:aliases-nothing-else` is a wrong DISJOINT.

> Examples where it holds: a DNS zone for its records, a dpkg database for its packages, a
> network namespace for its `net/*` knobs, a disk filesystem for its inodes. Examples where it
> never holds: a client NFS mount, an NSS view of LDAP, a chroot, a nested pid namespace, an
> overlay, a front over another daemon, a relabelling of part of its mParent's state. State
> spanning several files is a may-read matter (2.5-may-read-the-readset). A hardlink is a
> `:guarantees-unique-name` failure on the path mScheme. The path mScheme yielding the inode
> dissolves it.

### § 2.4-parent-as-a-relation

`:parent` is one per mKey (1.6-parent-one-per-key): never plural, never a species of its own.
Its mSort is the primary mScheme's declaration for the matched shape. Its instance is whichever
seat supplied it. The far end is an ordinary mKey with an identity of its own, never a string
the engine composes. A mVantage supplies instances and is never an mParent.

- Arity: one per mKey.
- Declared by: derived from the mScheme's declarations and the supplying seat.
- Default: not applicable.
- Consumer: routing, through a secondary mScheme. Identity, through the primary mScheme.
- Danger: none of its own.

### § 2.5-may-read-the-readset

> Separation logic's footprint, at the read side: the mReferents an answer may depend on.

K `:may-read` these mKeys: K's mState is affected by writes to them. The relation is
many-valued. It is an edge type from an mSort to mKeys of other mSorts, a template the mSite or
environment fills. K's owner declares it, with a completion sentinel closing the set. Every
mSort in a mFullyQualifiedKey may declare may-read entries, not only leaves. An entry may name
an mKey of another mWorld. `compare()` decides that pair as
3.2-compare-one-chokepoint-four-answers decides any pair.

A fact's readset is the marked reads of the body that answered it. For a verdict fact, the
vouch closes them (`KNOBS:kCONTRACT-RUNGS`). A body that marks no read has readset ⊤. May-read
entries are not in a readset. A write reaches K through them (2.6-may-write-the-writeset,
rule 4). An mSort that declares no may-read set, or declares one and does not close it, is
affected by every write. A readset member is ⊤ where any of these holds:

- its mSort has no closed may-read set
- an mSort on its mFullyQualifiedKey has no closed may-read set
- a may-read entry through which rule 4 reaches it fails either test above, or an entry
  through which rule 4 reaches that entry, transitively
- its mFullyQualifiedKey, or the mFullyQualifiedKey of such an entry, ends at the mRoute

⊤ is DISJOINT from nothing. The mRoute declares no set (1.10-vantage-route-placeholder-witness).

An mKey's mParent instance is no may-read entry and needs no declaration. The walk of
3.2-compare-one-chokepoint-four-answers collides a write at or above it. A may-read entry naming
a store says more: every write to an mKey relative to that store may change K, nobody having
said otherwise.

An omitted entry is a silent channel. It licenses nothing positive.

May-read is distinct from mParent. The mParent is one and answers identity. May-read entries are
many and answer interference. Two cells with different mParents can share a may-read entry and
so collide without being the same.

A closed may-read set is knife-tier. It is one of the two closures every sparing rests on
(2.6-may-write-the-writeset).

- Arity: many per mSort, with a sentinel.
- Declared by: the mSort owner.
- Default: ⊤. A readset member of the mSort, or one reached through it, is then ⊤.
- Consumer: the writeset of a line (2.6-may-write-the-writeset).
- Danger: none positive. Omission is the silent channel. The closed set is one of sparing's two
  closures.

> A may-read entry above the leaf: a loop-backed filesystem's state lives in a file of the outer
> filesystem, and `dd` over the image rewrites every inner fact. A shared entry: two observers'
> writability cells share the file's mode.

### § 2.6-may-write-the-writeset

> A refuted shape: two entries overlap only when they are one place
> (`311u:refuted-only-same-entries-overlap`). The model collides whatever is not DISJOINT.

A line's writeset is the set of mKeys the line may write or may change. It is the least set
that four rules close:

1. Every may-write entry the verb's author declared per matched shape is in the writeset (the
   footprint of `plans/30U`). The completion record closes those entries. Where the body emits
   at runtime, the verb author's completion record closes them.
2. Where an mKey of K is in the writeset, or an mKey identified beneath an mKey of K, every mKey
   that K's may-write entailment names is in the writeset. For each origin cell in the
   writeset, a reached finished record for that cell's mSort and shape finishes the entailment
   (`plans/30U`).
3. Where an mKey given whole is in the writeset, every mKey reached beneath it is in the
   writeset (2.9-the-traversal-and-the-region-test).
4. Where an mKey in the writeset `compare()`s other than DISJOINT with a may-read entry declared
   for an mKey k of mSort M, k is in the writeset (2.5-may-read-the-readset,
   3.2-compare-one-chokepoint-four-answers).

An unclosed at-most set or an unfinished entailment puts ⊤ in the writeset. An unclosed may-read
set puts ⊤ in the readset (2.5-may-read-the-readset). ⊤ is DISJOINT from nothing. A container at or above the deepest level that the written mKey shares with the read
mKey contributes no entailment to the test against that fact. May-read entries feed rule 4 and
no other rule.

The entailment: writing an mKey of K entails may-write of these mKeys of other mSorts. It is
arm-incremental and collide-adding. The reached completion record finishes the definition. It
witnesses that the write set, after entailment, is complete. K's owner declares it. The
entailment is about effects, not identity. It carries every cross-mSort consequence no
mFullyQualifiedKey expresses. It generates no DISJOINT: "nothing else" is no other thing, never
no other mKey for the thing written.

An elision is spared past a line only when `compare()` answers DISJOINT for every pair of a
writeset member and a readset member (2.5-may-read-the-readset), of one mSort or of two
(3.2-compare-one-chokepoint-four-answers, or 2.9-the-traversal-and-the-region-test where an
mKey is given whole).

The test spares narrowly and collides widely. Whatever is not DISJOINT collides. The finished
definition is a within-mWorld sentence. It never speaks across mRoutes or mRoots (§3.2). A
may-read entry that names an mKey of another mWorld enters rule 4 where a write in that mWorld
reaches it (§2.5).

- Arity: per matched shape of the verb, for the at-most set. Many per matched shape on the
  mSort, for the entailment. Plus the finished record.
- Declared by: the verb's author, for the at-most set. The mSort owner, for the entailment.
- Default: unfinished, which collides.
- Consumer: the sparing test above, within one mWorld. Never a generator of DISJOINT.
- Danger: the premature finished record.

> Without the exclusion of containers at or above the shared level, a filesystem's entailment,
> which names its disk, would make two files in one filesystem collide through it. Examples of
> the entailment: a package's postinst enabling its unit, a restart killing a main process.

### § 2.7-corresponds-across-a-transition

> A scoped `sameAs`. Not "corresponds to" loosely: a part, a view, or a correlate of a thing is
> not it.

mKey X inside mParent A `:corresponds` to mKey Y inside mParent B: they denote the same
mReferent. The owner of the transition between A and B declares it. That owner is neither mKey's
mScheme owner.

Absent a mCorrespondence, mKeys across a transition `compare()` UNKNOWN unless a
mFullyQualifiedKey binds mTokens on both sides. The mCorrespondence is the model's only declared
sameness generator besides mToken equality.

- Arity: per transition pair.
- Declared by: the transition owner.
- Default: absent, so UNKNOWN.
- Consumer: a SAME mDerivation (1.8-fully-qualified-key-topic-and-derivation), vouch-tier,
  attributed to the transition author.
- Danger: a wrong mCorrespondence is a wrong SAME.

> Examples: the container manager knows guest pid 1 is host pid 4821. `sudo -u alice` knows
> inner "me" is outer "alice". A mount line's oracle knows mKeys under the mountpoint are mKeys
> under the export on the named server, from this mVantage.

### § 2.8-observer-dependence-and-independence

The mValues that reads of K's cells yield depend on which mKey of mSort O the read was taken
under. K's owner declares the complement, `:observer-independence` of O, per mSort. By default,
a cell measured under a lent mKey of O is assumed to depend on it. Its fact is then about
(mReferent, O-instance). It never stands for the same mReferent under another O-instance.

No `resolve()` can measure observer-dependence: the object is the same and the answer differs.
It must remain speech. Measurement in the denoted context (`plans/27C`) stays the default lane.

- Arity: per (mSort, O).
- Declared by: the mSort owner.
- Default: dependent, so no carry across O-instances.
- Consumer: the SAME consumer, as a qualifier on the claim's mTopic.
- Danger: a false independence.

### § 2.9-the-traversal-and-the-region-test

A lookup emits its mTraversal, one member per routing mKey it crossed
(1.7-resolution-and-its-traversal). A lookup may cross several levels, each looked up in a
catalog that the previous level named. The engine never reads an mKey's syntax. Whatever
splitting an mKey needs happens inside a lookup's body. A lookup may cross an indexical routing
mKey, whose mResolution depends on the observing process. Only the lookup's owner can say which,
in the body that meets it. An undeclared indexical routing mKey reads unknown.

A lookup that emits no member has its mParent-Catalog, given whole, as its mTraversal
(1.7-resolution-and-its-traversal). Any touch on that catalog then invalidates every mResolution
through it. That is the coarse, safe floor. Containment is membership in a mTraversal.

An entry names the mReferent of its mKey. The walk of 3.2-compare-one-chokepoint-four-answers
collides a write to that mReferent with everything identified in it. An entry given whole also
names every mReferent reached beneath that mKey, through the mScheme's lookups or through a
placing route (2.10-places-the-upward-lookup). The entry's author marks it given whole.

A routing mKey named whole, in a writeset or as a may-read entry, stands for whatever its
mScheme reaches beneath it. In the test of 2.6-may-write-the-writeset it reads UNKNOWN against
every mKey that mScheme can yield in the same mParent-Catalog instance, whatever
3.2-compare-one-chokepoint-four-answers answers of the two as siblings. Unequal mTokens say only
that the thing is not the routing mKey itself. That is the floor.

The region test refines the floor. For an mKey D given whole against an mKey x, walk each
mTraversal that `identity(x)` produced at every level of x's mFullyQualifiedKey
(1.7-resolution-and-its-traversal, 3.1-identity-of-a-key), including the routes of
2.10-places-the-upward-lookup, leaf first:

1. If x's leaf compares SAME with D: SAME.
2. Else if any level of a mTraversal compares SAME with D: D's region covers x, and the pair
   reads UNKNOWN.
3. Else if x carries a closure for D's mSort, in one of two forms: DISJOINT.
   - x has at least one mTraversal of D's mSort. On every such mTraversal, every level compares
     DISJOINT with D and every level emitted its closure, `alias nothing-else`
     (1.5-token-and-the-two-warrants) or `looked-up-in nothing-else`
     (2.10-places-the-upward-lookup).
   - The placing lookup of D's mSort emitted `looked-up-in nothing-else` for x with no
     `looked-up-in` record. Then x is in no region of that mSort.
4. Otherwise: UNKNOWN.

Only x's mTraversals are walked. D needs no closure of its own. Every level is asked, never only
the leaf, because an alias may sit at any level. A leaf's own closure cannot see it. `compare()`
(3.2-compare-one-chokepoint-four-answers) compares a level by the identity of the mReferent that
the level resolved to.

- Arity: per lookup, per matched shape.
- Declared by: the lookup's owner, through what the lookup emits.
- Default: no emission, so the mTraversal is the mParent-Catalog given whole.
- Consumer: mResolution backings (1.7-resolution-and-its-traversal), hence invalidation
  (3.3-invalidation-three-mutator-species), and the region test.
- Danger: a false closing act keeps a stale mResolution and every conclusion built on it. A
  false `alias nothing-else` is a wrong DISJOINT. An open or coarse emission is safe.

> Lookups that cross several levels: a path yields a directory entry looked up in a shorter
> path. A hostname yields a resolver step from a mVantage. A dotted unit name yields an entry in
> its instance table. Lookups that cross one level: an inode number, a uid. An indexical routing
> mKey: `/proc/self`. A path prefix is not a store. A mutator that touches a directory needs to know
> nothing about files. An alias above the leaf: a bind mount of a directory above a file, or an
> alias entry above a leaf. A positional catalog: the resolution of position N depends on every
> position at or before it, and `ufw insert 1` gives every later rule a new number. The lookup
> emits positions 1 to N as its mTraversal, one line per member. A rowid table renumbered after
> ten thousand rows makes that emission large. A single emission that describes the set, spelled
> as sh, is work for the 312 series.

### § 2.10-places-the-upward-lookup

An mSort G may declare that it `:places` another mSort T. G's owner publishes a lookup that is
invoked with the mValue of an mKey of T. Its matched shapes decide which spellings of T it
answers. For that mKey it emits `looked-up-in G:key` and a closure `looked-up-in nothing-else`,
scoped to routes of mSort G. These are the records any lookup emits. Membership is a relation
between two mReferents, never a spelling of one, so the placing lookup is not an mScheme of T.

The route so recorded is a mTraversal of the mKey for the region test
(2.9-the-traversal-and-the-region-test). A mutator invalidates it as it invalidates any
mResolution (3.3-invalidation-three-mutator-species). It is never the mKey's mParent, which is the route the
mKey's own lookup supplied (1.6-parent-one-per-key).

The engine invokes G's lookup only when all three hold:

- a writeset or readset entry names an mKey of G given whole
- the mKey of T on the other side of that pair, a writeset entry or a readset entry, has no
  route of mSort G
- G declares that it places T

It invokes the lookup with every mKey it holds for that mReferent. The `looked-up-in` records of
every invocation accumulate. A closure `looked-up-in nothing-else` from one invocation can
contradict a record from another invocation. The engine then refuses both answers and
attributes the refusal to G's owner.

The store's end of the same relation is G's enumeration of its members. The may-write entailment
of 2.6-may-write-the-writeset already carries that as write reach. When that entailment is
finished for P (§2.6), its emitted members stand in, in the test of §2.6, for the mKeys reached
beneath P (§2.6, rule 3). Each member is an entry of that test. P itself stays an entry of that
test. An unfinished entailment widens the writeset only. Where a placing route places x in P and
a finished enumeration of P has no member SAME with x, the pair reads UNKNOWN.

- Arity: per (G, T).
- Declared by: G's owner.
- Default: absent. T's mKeys then have no route of mSort G, and the pair reads as
  3.2-compare-one-chokepoint-four-answers decides it.
- Consumer: the region test (2.9-the-traversal-and-the-region-test), and invalidation.
- Danger: a false `looked-up-in nothing-else` is G's owner's wrong DISJOINT.

> Matched shapes of a placing lookup: a path-shaped mValue answered, an inode number declined.

### § 2.11-composite-sorts-and-roles

A mTopic whose mReferent's mState depends on several inputs in roles is an mKey of a
mCompositeSort. The author who knows the roles mints it, normally the tool author. That mSort's
identity is its owner's function of its named parts. Plurality of inputs is an mSort with
structure, never a set of mParents. The may-read set of a mCompositeSort is the union of its
parts' may-read sets.

- Arity: per composite.
- Declared by: the author holding the roles.
- Default: not applicable.
- Consumer: identity (3.1-identity-of-a-key).
- Danger: as any mSort.

> Examples: a base and an overlay, or a primary and its replica set.

## § 3-composition-and-laws

### § 3.1-identity-of-a-key

`identity(k)`, for k an mKey of mScheme S:

1. Run S's `resolve()` from k's mVantage, and each yielded mScheme's `resolve()` in turn, until
   an mKey of a primary mScheme is in hand. Each emission supplies the mParent instance for the
   mKey it yields.
2. The result is that mKey-Primary scoped in `identity(mParent)`, recursively through each
   level's primary mScheme, until a mRoot, the mRoute, or an unknown link.

Each level carries the warrants declared for the shape its mKey matched. A mCompositeSort's
identity is its owner's function of its parts' identities. A cell's identity is its mParent's
plus its mSort (1.9-cell-a-singleton-sort). An mKey of an observer-dependent mSort carries the
O-instance in its mTopic. The mVantage is consulted only to know where to run `resolve()` calls
and which ambient mParents to bind.

### § 3.2-compare-one-chokepoint-four-answers

> Alias analysis's may/must trichotomy, plus KNOWN_UNSPOKEN for "no generator applies".

> A refuted shape: a parent partitions its children's mSorts
> (`311u:refuted-parent-partitions-its-children`). Separation comes from one definition's own
> distinctions.

`compare(x, y)` answers one of SAME, DISJOINT, KNOWN_UNSPOKEN, or UNKNOWN. SAME means the fact
is about this mKey. The engine consumes a SAME that rests on a wrapper's sentinel under
`--risk-faultless-skips` (3.4-entry-and-lends). DISJOINT licenses sparing under the same flag.
UNKNOWN and KNOWN_UNSPOKEN are the safe bottoms.

One level. Two mKeys at one level are SAME iff they are one instance, or they are equal mValues
whose shape carries `:guarantees-unique-referent`. One instance means one mPlaceholder:
inherited through a wrapper's sentinel under `--risk-faultless-skips` (3.4-entry-and-lends), or
one ambient instance resolved once in one unwalled span under no wrapper
(1.10-vantage-route-placeholder-witness).

Two mFullyQualifiedKeys. Levels are numbered from the leaf, level 0, upward through mParents.

1. If either mFullyQualifiedKey contains an unknown link, the pair reads UNKNOWN. If one
   terminates at a mRoute the other does not share, the pair reads UNKNOWN. That covers one at
   the mRoute and one at a mRoot, and two mRoutes across a wrapper. A mRoot shape is one mWorld,
   SAME by `:root`. Two mKeys of that shape meet there and compare as siblings. mRoots of
   two shapes are two mWorlds. A mRoute is another. No mFullyQualifiedKey speaks across mWorlds.
   The finished definition does not speak across mWorlds, because its sentence is within-mWorld
   (2.6-may-write-the-writeset). A mCorrespondence is the one mDerivation that may speak across
   mWorlds (2.7-corresponds-across-a-transition).
2. Otherwise walk downward from the top to the deepest level at which the two chains are SAME by
   the one-level rule. Call that level A. If either mKey is A itself, the pair reads UNKNOWN: a
   write to a container collides with everything inside it.
3. Otherwise call the child of A on each side that side's top. The top is the mKey itself when
   its mParent is A. Separation is only ever concluded from a single definition's own
   distinctions, in one of two ways.
   - Two tops: both tops are mKeys of one mScheme, each carrying `:guarantees-unique-name`, with
     differing mValues.
   - One top: exactly one mKey is its own top, and the `resolve()` body of its primary mScheme
     declares, for some other shape, `:identified-in` the mSort of the other side's top. An
     omission is a distinction only inside the body that made it. This way rests also on a store
     never being among its own contents.
4. The pair reads DISJOINT iff one of the two ways holds and every store strictly below A, down
   to either leaf's mParent, is `:aliases-nothing-else`
   (2.3-aliases-nothing-else-the-store-warrant). Otherwise it reads UNKNOWN. Separation is
   decided once, at A.

Two mFullyQualifiedKeys are SAME iff they are SAME at every level down to the leaf.

Different mSorts. mKeys of different mSorts share no primary mScheme. Their mFullyQualifiedKeys
meet, if at all, only at a common ancestor, and that meeting is not a claim about the leaves.
The walk decides such a pair as it decides any other: DISJOINT where it separates them, and
otherwise KNOWN_UNSPOKEN. KNOWN_UNSPOKEN never spares and never transports, whatever either
side has declared finished (2.6-may-write-the-writeset). Two mSchemes yielding one mKey-Primary
is the sole same-referent generator across ways of naming. Dorc equates mKeys and never merges
mSorts.

Partial measurement never widens. A mDerivation with an unmeasured or mRoute-terminated link
yields at most what it would yield with the link measured.

mDerivation sets. SAME is "or" across mDerivations (the mFullyQualifiedKey, a mCorrespondence, a
provider-supplied identifier) and "and" within one mFullyQualifiedKey. A warranted SAME and a
warranted DISJOINT on one pair is a contradiction: refuse both and attribute both authors.
Otherwise the strongest warranted answer stands. SAME composes transitively. SAME then DISJOINT
composes to DISJOINT. DISJOINT then DISJOINT never chains. The universal meet over backing sets
is unchanged. Genuinely different mKey-Primaries for one mReferent are two mDerivations for one
mTopic, reconciled by coherence, never a second mKey inside one mParent.

> Two mSorts meeting at a common ancestor: a package status file and a unit file share a
> filesystem. Genuinely different mKey-Primaries for one mReferent: an NFS filehandle and the
> server's inode, or a machine-id and a cloud instance-id.

### § 3.3-invalidation-three-mutator-species

Three mutator species invalidate three kinds of fact. In all three, the engine withdraws
authority. It never computes the successor identity.

- A routing mutation touches routing mKeys, a mParent-Catalog, or shell state a `resolve()`
  read. A routing mutation invalidates a mResolution when its mTraversal includes a touched
  mKey. A writeset entry touches a mTraversal member when `compare()` answers other than
  DISJOINT for the pair (3.2-compare-one-chokepoint-four-answers). A write also invalidates the
  mResolution when a writeset entry `compare()`s other than DISJOINT with a member of the read
  set of the lookup body that produced it (1.7-resolution-and-its-traversal). Any write
  invalidates a mResolution whose read set is open. Every mFullyQualifiedKey built on an
  invalidated mResolution reads unknown below the line. Dependent
  SAME conclusions lose authority, and dependent elisions demote to guards. Dependent DISJOINT
  conclusions collide. The touched object itself is untouched. Creation, deletion, and rename of
  an mKey are routing writes. They change what the mKey reaches. The mKeys they write are the
  verb author's at-most claim. A writeset that omits them is the ordinary at-most omission
  knife, now visibly covering routing mKeys.
- A state mutation reaches every mKey in its writeset (2.6-may-write-the-writeset): ordinary
  kill-reach. A
  first write can also change an mKey-Primary. So a state mutation whose writeset touches a
  mParent-Store invalidates the mTokens scoped in it.
- A lifecycle mutation writes a mRoot-adjacent mKey, such as a boot or a tenure. Every
  mKey-Primary scoped in it names a new mReferent afterward. Cells whose mFullyQualifiedKeys
  pass through it are new and unmeasured. Cells whose mFullyQualifiedKeys do not are untouched.
  "Keyed by Boot" and "invariant across Boot" are the shape of the mFullyQualifiedKey, not
  declarations.

> Routing mutations: a mount, a symlink replacement, a rename, a user added, a hostname change,
> a write to any environment variable, cwd, or configuration a lookup reads.
> `userdel alice; useradd alice` invalidates every mResolution of the old mKey. Lifecycle
> mutations: a reboot, a re-provision.

### § 3.4-entry-and-lends

> Dynamic binding: `parameterize`, `fluid-let`.

A wrapper's entry `:lends` mParent-Catalog instances for the mParent-Catalog mSorts it perturbs,
and nothing else. The lent instance becomes the ambient mParent for every mKey of a secondary
mScheme looked up in that mParent-Catalog mSort.

An unlent mParent-Catalog mSort is ⊤ under the wrapper. After the wrapper's completion
sentinel, and under `--risk-faultless-skips`, the unlent mSorts and the mRoute inherit the
caller's instances instead. Leaf mKeys then inherit transitively through their
mFullyQualifiedKeys with no further speech. A wrapper may declare mCorrespondences across
the mParent-Catalogs it lends (2.7-corresponds-across-a-transition). A lend may depend on the
guest. The wrapper author then declares the guest-insensitive default and supplies a policy read
that declines on departure.

- Arity: per wrapper, per mParent-Catalog mSort, plus the sentinel.
- Declared by: the wrapper owner.
- Default: ⊤, which walls.
- Consumer: ambient mParent supply.
- Danger: a wrong lend measures the wrong mVantage. The sentinel is an at-most claim over every
  mParent-Catalog mSort and the mRoute. A wrong sentinel is a wrong SAME. The flag prices it.

> Examples: a chroot lends a mount namespace. `sudo -u` lends a user. `ip netns exec` lends a
> network namespace. A lend that depends on the guest: sudoers matches the guest command.

### § 3.5-committee-law-and-attribution

Every positive step is one author's line:

- a `:yields` and its lookup warrants
- a shape's `:identified-in` and its warrants
- a may-read set and its sentinel
- a may-write entailment and its finished record
- a mCorrespondence
- an `:observer-independence`
- a `:lends`

The engine only chains and meets. Every statement an answer rests on is something one party can
know about their own tool, store, or machine. That party says it alone, describes nothing they
cannot see, and names no other author. So the speech needed grows with the number of authors,
never with the number of pairs of them.

A granting composite is entailed jointly by one mScheme's `:yields` and the primary mScheme's
declaration for that shape. Each author speaks about their own lookup. A withholding composite
names nobody and needs nobody's consent.

Attribution:

- Every survival names the `:aliases-nothing-else` and `:guarantees-unique-name` declarations it
  rested on, the route closures it rested on (1.5-token-and-the-two-warrants,
  2.10-places-the-upward-lookup), and the closed may-read sets its writeset rested on.
- Every SAME names the `resolve()` calls, the declarations, the sentinels and route claims that
  made instances one, and the mCorrespondences.
- Every invalidated conclusion names the writeset that invalidated it.

> A granting composite: "these two accounts are one". A withholding composite: a mount
> invalidating an account's mResolution.

## § 4-relation-to-other-documents

### § 4.1-boundary-of-this-model

This model uses, and does not redefine:

- the verdict, vouch, and guard tier
- the authored may-write entries, the completion record as the witness of a finished definition,
  and `--risk-faultless-skips`
- the four-answer chokepoint and its consumer map, except that a sentinel-inherited SAME rides
  the flag (3.4-entry-and-lends)
- the universal meet
- measure-in-context, entry forms, siting vouches, the escalation dial, and `safe-across`
  (`plans/27C`)
- the read-set closure as the falsification net for unmarked reads (`plans/27C` §4(a)(B))
- binds as the mKey-minting act
- the mPlaceholder and the standup `witness()`
- the integrity plane
- the committee law

The model excludes a selector dialect, an aspect species, an authored region predicate, an
engine-side name floor, and an engine table that generates SAME. The context slot is a mVantage
and nothing else.

### § 4.2-supersessions-pending-in-prior-documents

A living register of statements in prior documents that this model contradicts. An entry stays
until its document is rewritten. Remove an entry when its document catches up. Add an entry when
another prior document is found to disagree. Each entry names the passage, gives its claim, then
gives this model's claim after "Here". A root document gets one brief entry where a passage
became untrue. The human refreshes root documents.

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
