# 311 — Identity and relation: the preparatory model for the r31 identity design

> AI-authored PREPARATORY SYNTHESIS (Fable, the `r31-prep-design-duck` sittings, 2026-09-07/08;
> the human present and adjudicating). Notes-tier. Nothing here is ruled; grades on the
> conductor's claims are +SURE / ~SUSPECT / -GUESS / --WONDER, and **[LEAN]** marks a human lean
> paraphrased from chat, never a ruling. Authority: root docs, `spike/CLAUDE.md`, and the welds
> outrank this; where this document disagrees with `plans/30W`, `notes/272`, `plans/30T` §6,
> `plans/30U` §7, `notes/26Ob` §10, or `plans/27C` §4, it is a deliberate proposal to
> re-litigate them, per the human's direction that nearly everything in this corner is open
> as long as the result is internally coherent.
>
> Purpose: enumerate the problem space of identity and relation across mutually-unknowing
> authors, fully enough that a fresh design agent can distill a correct, teachable design from
> it. §1–§3 are the MODEL and come first. §4 and §5 are deliberately empty and will be filled
> backwards from the model. §6 is the forcing-function inventory. §8 is historical.
>
> Scope discipline: abstract objects and relations only. No syntax, no strawman sh, no UX, no
> gradual-enhancement ladder, no implementation. The sole exception is `sh` vocabulary used
> as a referent-agnostic example of an mKey-CatalogStore or a transition.
>
> Naming discipline: where a prior-art term overlaps precisely, this document takes it and
> says which domain it comes from; where a nearby term is a false friend or is squatted by
> our own vocabulary, the heading or a blockquote under it says so. A hyphenated `mSpecies-Gloss` (mKey-CatalogStore, mSort-PrimaryStore, mKey-Natural) is a derived view of the species before the hyphen, never a declared species.

## § 0-one-screen

Dorc removes a line only on connected claims that the line is unnecessary: a measurement
claimed before anything runs, plus an author's vouch. The claim can be destroyed by an earlier
line that really runs, so the engine must decide whether the piece of the world an earlier line
touches is the same piece a later line's licence depends on. That is identity, and it has two
consumers with opposite failure directions: SAME lets one fact stand for another and
under-executes when wrong; DISJOINT lets a licence survive a write and under-executes when
wrong. UNKNOWN is safe for both. The engine knows only syntax, authored speech, and what
authored probes returned; it never decodes a key. No single author knows the whole path from a
tool's name to a measurable mReferent, and viewpoint transitions authored by yet other people
change which mReferent a key reaches mid-book.

The model: every identity-bearing thing is an mKey of an mSort. An mSort's mKeys have a
mKey-Natural inside an mKey-CatalogStore (routing), and, where the mSort's owner supplies a `resolve()`, a
mKey-Primary inside another mKey-PrimaryStore (identity). Identity is the mKey-Primary scoped
recursively, a mFullyQualifiedKey, until a mRoot, a declared global mKey-PrimaryStore, or a mRoute,
the last resort. State has mPlacements, many-valued, which feed collisions only. Two things
`compare()` by walking their mFullyQualifiedKeys to the first divergence inside a shared mKey-PrimaryStore
and asking what the `:identified-in` edge into that mKey-PrimaryStore licenses for inequality; never diverging is sameness;
different mRoots are unspoken (no generator applies to the pair). Every positive answer rests
on an explicitly typed,
long-named, absent-by-default warrant owned by one author; silence is unknown. Transitions
are of three species, each perishing a different fact. The context slot survives as a
mVantage, an address and a witness key, never as an identity input.

## § 1-model-objects

### § 1.1-referent

A piece of the world that has state: an inode, a database row, a package record, a kernel
parameter, a running process, a machine, a mount table. The engine never holds an mReferent.
It reaches mReferents only through mKeys, mTokens, and mDerivations, and every identity
question is ultimately "do these two mKeys reach one mReferent". mReferents are what GOTCHAS
are about.

### § 1.2-sort

> A *sort* in the many-sorted-logic sense: a carrier, one named naming scheme over mReferents
> with one accountable owner, not a category of mReferents. Never the pure-type-system sense (a
> universe) and never a PLT kind (the type of a type); the word names the carrier and nothing
> else. Two mSorts may name one mReferent set (a file by path and by descriptor; a host by
> name and by instance-id), each with exactly one mKey-Natural; their sameness is decided by a
> shared mKey-PrimaryStore (§3.2).

A vocabulary with one accountable owner (reverse-DNS naming, no registry, owner-adjudication
as the social contract — unchanged from `24M` and `277` §6). An mSort fixes, for its mKeys:
the `:named-in` edge (§2.1), the `resolve()` and mKey-Primary if any (§2.2), the mPlacements
(§2.3), the effect entailment (§2.4), the :observer-dependence (§2.6), and whether it is a
:hierarchical mSort when it serves as one (§2.8). An mSort's owner speaks only about the
mSort's own relations to its immediate neighbours; the engine composes. mSorts are the only
identity-bearing category; there is no separate mKey type, aspect type, or context type in
the model (§1.8, §1.9).

### § 1.3-key-natural-key-primary-key

> RDBMS terms, taken whole with their culture: a mKey-Natural is domain-meaningful,
> user-typed, may alias, and is never trusted as identity; a mKey-Primary is what the store
> itself identifies rows by, and is unique only within its own table. Clones are two tables
> with colliding mKey-Primaries.

A key is a string an author wrote, addressing an mReferent of an mSort FROM SOMEWHERE. Every
key is relative to an mKey-CatalogStore, and a bare string means nothing. Each mSort has:

- a mKey-Natural: the strings tool authors and books use (`red/7`, `/etc/nginx.conf`,
  `nginx`, `beta`), interpreted in the mSort's mKey-CatalogStore (§2.1);
- a mKey-Primary: the strings the mSort's own `resolve()` returns (an inode number, a
  filesystem identifier, a canonical package name, a boot identifier), interpreted in the
  mSort's mKey-PrimaryStore (§2.2).

When an mSort has no `resolve()`, the mKey-Primary IS the mKey-Natural. The distinction exists because
the mKey-Natural is what people write and is usually ambiguous (aliases, relative names),
while the mKey-Primary is what the store answers and is where the dangerous warrants can
honestly be placed.

### § 1.4-catalog-store-primary-store-root

An mKey-CatalogStore is an mKey of some mSort, used as the space in which another mSort's
mKeys are interpreted: a database file for account mKeys, a directory for the next path
component, a mount table for paths, a filesystem for inodes, a resolver-plus-vantage for
hostnames, a process table for pids. An mSort's mKey-Natural is looked up in one such mKey (its
mKey-CatalogStore) and its mKey-Primary is scoped in another (its mKey-PrimaryStore); they coincide when the
mSort has no `resolve()`. A mRoot is an mSort that declares its mKey-Primaries need no mSort-PrimaryStore, which is the
claim that they are globally comparable (the DNS mRoot, whose mKeys are fully-qualified).
:rootness is a dangerous, long-named claim (§2.2); nothing is a mRoot by default.
`GOTCHAS: identity-tokens-have-clone-horizons` is the standing witness against casual
:rootness.

### § 1.5-token-and-the-two-warrants

> The two warrants are OWL's inverse-functional and functional properties, and OWL's
> inference consequences transfer exactly (inverse-functional is what licenses `sameAs`).
> OWL's names swap direction with the property's orientation, which is why ours are spelled
> out instead.

A mToken is a mKey-Primary value: bytes the mSort's `resolve()` returned, compared for equality only,
never decoded (`inv-referent-agnostic`). A mToken is always scoped (§1.4). Any `:named-in` or `:identified-in` edge
has two independent properties, and each is a separately declared,
absent-by-default warrant:

- :guarantees-unique-referent: within one mKey-CatalogStore or mKey-PrimaryStore, equal mKeys reach one mReferent; the
  lookup is a function. Licenses SAME from equality. Fails for round-robin mResolution
  (`resolution-is-set-valued`), for recycled mKeys (`recycled-keys-outrun-the-unwalled-span`),
  and for cloned identifiers presented as mRoots (`identity-tokens-have-clone-horizons`).
- :guarantees-unique-name: within one mKey-CatalogStore or mKey-PrimaryStore, one mReferent has one mKey. Licenses
  DISJOINT from inequality. Fails for symlinks and hardlinks, package `provides`
  (`distinct-names-alias-within-a-kind`), route-qualified handles.

Both are properties of the `:named-in` or `:identified-in` edge, holding for K's mKeys and mReferents inside any one
mKey-CatalogStore or mKey-PrimaryStore, not of a single mKey and not of that mKey-CatalogStore or mKey-PrimaryStore: declared once per mSort by
the owner of the mKeys, holding for every mKey-CatalogStore or mKey-PrimaryStore they are looked up in. Neither holds by
default anywhere: not for mKey-Naturals, not for mKey-Primaries, not for mRoots. The one lookup
the engine vouches for itself is the transit-free local mRoute (§1.9). Every other claim that a
lookup is a function is the owner's opt-in (the dpkg database's owner makes it; a resolver's
owner and a pid table's owner never do), and across a transit it is never claimed, only
measured and witnessed.

### § 1.6-resolution-and-traversal

A mResolution is the fact that mKey-Natural N, interpreted in mKey-CatalogStore S, at
program point p, reaches mReferent R. It is a FACT with a BACKING, and its backing is the
mTraversal: the ordered chain of routing mKeys the lookup crossed (each directory entry and
symlink for a path; the resolver configuration and mVantage for a hostname; the unit table for
a service name). A :hierarchical mSort's structure fixes how a mKey decomposes into a
mTraversal (§2.8). A mResolution perishes under ordinary effective-mWorld reach from any mutator
whose footprint touches any mTraversal member. Its target object is NOT its backing: a mKey
can stop reaching an object without the object changing
(`renaming-a-parent-moves-every-child-name`, `a-path-is-not-a-referent`), and an
object can change without its mKey changing.

### § 1.7-fully-qualified-key-and-derivations

A mFullyQualifiedKey is the recursive identity of an mKey: its mKey-Primary, scoped in its
mKey-PrimaryStore, whose identity is itself a mFullyQualifiedKey, terminating at a mRoot, at
a mRoute (§1.9), or at unknown. A mFullyQualifiedKey is one mDerivation of identity. A mTopic, what a
claim is about (a mCell, together with the observer instance when the mCell's mSort is
observer-dependent, §2.6), may carry several mDerivations with different generators: its mFullyQualifiedKey, a
provider-supplied identifier, a mCorrespondence asserted by a transition owner (§2.5).
mDerivations combine by coherence (§3.2), never by priority. Equality composes transitively
across mDerivations; separation is decided at one divergence and is never chained. (`witness()`
is reserved throughout for the standup re-measurement of §1.9.)

### § 1.8-cell-aspect-sort-value

A mCell is the unit that has a value: what a probe measures and a mutator writes. In this
model a mCell is simply an mKey of an mSort whose values are measurable. What earlier designs
called an aspect or selector (`Service:nginx@enabled`) is an mAspectSort: an mSort that borrows
another mSort's `:named-in` edge (§2.7) but has its own `resolve()`, mKey-PrimaryStore, mPlacements,
and :observer-dependence. `enabled` and `active` are two mSorts sharing the unit name; one is
measured in a symlink's existence and the other in pid 1's memory, and their mFullyQualifiedKeys
differ, which is exactly why one survives a reboot and the other does not, with no
declaration about reboots by anyone. Three independent findings forced this fold (§6.2): one
mKey-PrimaryStore per identity-bearing thing, per-aspect mPlacement, per-aspect
:observer-dependence. The whole-mKey claim (`disturbs Service:nginx`) reaches its
mAspectSorts by the owner's effect entailment (§2.4), and absent that entailment collides
with them as unspoken, which is the safe bottom. The selector position in the coordinate
and the selector dialect (`277` §3, `30J`) have no counterpart in this model.

### § 1.9-vantage-route-placeholder-witness

A mVantage is the ADDRESS a probe reached an mReferent from: the mEntryChain, expressed as a
finite map from mSorts to the mKey-CatalogStores that wrappers lent (§3.4).
It is not part of any mKey's identity. It has three jobs: it says where a `resolve()`
executes; it supplies the ambient mKey-CatalogStore for mKeys whose mSort is :named-in a lent
mKey-CatalogStore; and, for an mSort with neither `resolve()` nor `:named-in`, it is the mRoute, the last-resort
mKey-PrimaryStore. For transit-free local execution the engine itself vouches the mRoute: two same-spelled
mKeys in one unwalled span are one mPlaceholder, because one shell process resolves one
cwd-qualified mKey in one mount table. Across a transit the mRoute is never vouched, only
measured and witnessed; across mVantages it is unknown. mFullyQualifiedKeys whose mTokens are not yet measured are
mPlaceholders keyed by (mKey, ambient mKey-CatalogStores, mEntryChain); the probe standup binds them;
the apply standup re-reads them through the same entry and compares (the `witness()`); mismatch
is integrity, never a verdict input (`26Ob:cor-standup-witness-licenses-bare-line-elision`).
The `witness()` `compare()`s mTokens at one instant and cannot see a recycled mKey (a reissued pid or
inode); that stays on the outside-churn horizon (`recycled-keys-outrun-the-unwalled-span`).

### § 1.10-site-and-claim-species

A mSite is within a book line, with an argv, an mEntryChain, and a program point.
Speech at or about a mSite, each with one author:

- a VERDICT FACT: a mCell's measured value, backed by the marked read set
  (`30T:req-verdict-marks-every-read-cell`), vouched by the tool-oracle author;
- a FOOTPRINT CLAIM: an at-most write set per matched shape, with its completion witness
  (`30U`), by the tool-oracle author or the filesystem binder;
- an EFFECT ENTAILMENT: what disturbing an mKey of an mSort drags along, with the finished
  definition (`30U`), by the mSort owner;
- a mCorrespondence: mKey X in mKey-CatalogStore A is mKey Y in mKey-CatalogStore B, by the
  owner of the transition between A and B (§2.5);
- the mSort-level DECLARATIONS of §2: namings, warrants, mPlacements, :hierarchical,
  :observer-dependence, :rootness;
- the wrapper's :lends and their completion sentinel (§3.4).

## § 2-model-relations

Each relation below states: arity, who declares it, its default, which consumer reads it,
and which warrant makes it dangerous. The human's naming law applies throughout
(**[LEAN]**: dangerous operations rare and long-named; safe ones short) but no spellings are
proposed here.

### § 2.1-named-in (the natural key's catalog-store; routing)

mSort K's mKey-Naturals are interpreted in an mKey-CatalogStore of mSort N. Declared by K's owner, once
per mSort. The INSTANCE is supplied one of three ways, and the supply mode is part of the
declaration: FIXED (K's owner names it: the dpkg status file), SITE (the tool author's bind
fills it from argv: `--db`), AMBIENT (the mEntryChain's lent mKey-CatalogStore for N: the current
mount table for paths). Consumer: mResolution (§1.6). Default: absent means K's mKeys have no
mSort, so they resolve only under the mRoute (§1.9). Warrants: the two of §1.5, both
absent until the owner of the mKeys declares them; :guarantees-unique-referent is that owner's
claim that lookup in N is a function. This relation never licenses disjointness across mKey-CatalogStores by itself:
mKey-CatalogStore-disjointness is not referent-disjointness (`a-host-is-not-a-partition`,
`address-inequality-is-not-referent-inequality`, `containment-by-path-prefix-lies`).

### § 2.2-identified-in (the `resolve()`, the primary-store, sole-route, rootness)

> ER modeling's identifying relationship: a weak mKey's mKey-Primary includes its
> identifying mKey's mKey; a strong mKey has its own. Identifying is NOT containing.
> The containing half is :sole-route below (networking: single-homed), a separate,
> dangerous claim; conflating the two is the hardlink and NFS mistake.

K's `resolve()`, declared and executed by K's owner in the mVantage the mKey resolved from,
maps a mKey-Natural to a mKey-Primary scoped in an mKey of mSort M. That mKey is K's
mKey-PrimaryStore, the thing whose identity determines K's. The `resolve()` returns a scoped mToken,
and the mKey-PrimaryStore's identity is recursive (§1.7). Where no `resolve()` exists, M = N and the mKey-Primary
is the mKey-Natural. Consumer: identity (§3.1). Defaults: no `resolve()` means identity falls to
the mKey-Natural under its own defaults. Warrants, all absent by default and all declared by
K's owner about the mKey-Primary:

- :guarantees-unique-referent and :guarantees-unique-name (§1.5), governing what mToken equality and
  inequality license at K's level of a mFullyQualifiedKey;
- :sole-route: K's mReferents are reachable only through their mKey-PrimaryStore, so disjoint
  mKey-PrimaryStores imply disjoint K-referents. Required for a divergence at the mKey-PrimaryStore level
  to yield DISJOINT for K. Fails for replicated rows, for files reachable through several
  directories (a hardlink), for a record whose current state spans several files
  (`a-store-is-not-one-inode`);
- :rootness: K declares no M, and thereby claims its mKey-Primaries are globally comparable.
  Equivalent to declaring :guarantees-unique-referent over the whole world; fails for cloned
  identifiers (`identity-tokens-have-clone-horizons`). A mToken that can be duplicated across
  instances of its would-be mKey-PrimaryStore must be scoped in something smaller or left un-warranted.

A grade governs every consumer of the answer it grades, corroboration and contradiction
included: a `resolve()` without :guarantees-unique-name cannot contradict anything by returning two
different mTokens.

### § 2.3-lives-in ("placement", i.e. read footprint in separation logic)

> The correct name is footprint. `disturbs` squats it for the write footprint, and `backing`
> squats the per-fact read footprint (`23M`). mPlacement is the mSort-declared read footprint
> that a verdict fact's backing refines at probe time.

K's mReferents' state is affected by writes to these mKeys. Many-valued. Declared by K's
owner, per mSort, with a completion sentinel closing the set. Consumer: collision only. A
footprint touching any mPlacement collides with K's mCells; an incomplete mPlacement set loses
protection for sparing (an omitted mPlacement is a silent channel:
`an-omitted-store-breaks-invariance`, `a-store-is-not-one-inode`) and never
licenses anything positive. mPlacement is
distinct from the mSort-PrimaryStore: the mSort-PrimaryStore is at most one and answers
identity; mPlacement is many and answers interference. Two mCells with different mKey-PrimaryStores
can share a mPlacement and therefore collide without being the same
(`the-subject-includes-the-observer`: two observers' writability mCells share the file's
mode).

### § 2.4-reaches (effect entailment), and the finished definition

Unchanged from `plans/30U`: disturbing an mKey of K entails disturbing these mKeys of
other mSorts; arm-incremental, collide-adding; the reached completion record finishes the
definition and is the sole licensor of sparing across UNSPOKEN mKeys (§3.2). Declared by
K's owner. This relation is about effects, not identity, and it is what carries a package's
postinst enabling its unit, a restart killing a main process, and every other cross-mSort
consequence that no mFullyQualifiedKey expresses.

### § 2.5-corresponds (declared sameness across a transition)

> A scoped `sameAs`. `23M` rejected `owl:sameAs` for admin-declared cross-mSort co-reference;
> this is the other case, same-mSort across two mKey-CatalogStores, asserted by the one
> author who owns the transition between them (the NAT table, the pid-namespace map).

mKey-Natural X in mKey-CatalogStore A denotes the same mReferent as mKey Y in mKey-CatalogStore
B. Declared by the owner of the TRANSITION between A and B, which is neither mKey's
mSort owner: the container manager knows guest pid 1 is host pid 4821
(`correspondence-is-known-only-to-the-transition-owner`); `sudo -u alice` knows inner "me"
is outer "alice", which is what a mapped lend has been asserting all along. Consumer: a
SAME mDerivation (§1.7), vouch-tier, attributed to the transition author. Default: absent, so
mKeys across a transition `compare()` unknown unless a `resolve()` binds mTokens on both sides. Danger: a
wrong mCorrespondence is a wrong SAME. This generalizes the mapped lend of `273` and is the
model's only declared sameness generator besides mToken equality.

### § 2.6-observer-dependence

K's mCells' VALUES depend on which mKey of mSort O the measurement was taken
under. Declared by K's owner per mSort as its complement, :observer-independence of O.
Default: a mCell measured under a lent mKey of O is assumed to depend on it, so its fact
is about (mReferent, O-instance) and never stands for the same mReferent under another
O-instance. Consumer: the SAME consumer, as a qualifier on the claim's mTopic. This is the
surviving half of the old invariance line (`271:rul-invariance-speech-act`): its store half
is measured away by §2.2, its observer half cannot be measured by any `resolve()` because
the object is the same and the answer differs (`the-subject-includes-the-observer`), and it
must remain speech. Measurement in the denoted context (`plans/27C`) stays the default lane;
carrying a fact across an O-shift requires this declaration, and the human's hoped-for
static no-transit path costs exactly one such line per mSort in the stdlib.

### § 2.7-named-like (aspect-sorts)

mAspectSort A borrows mSort K's `:named-in` edge: A's mKeys are addressed by K's mKeys in
K's mKey-CatalogStore, and A has its own `resolve()`, mKey-PrimaryStore, mPlacements, and
:observer-dependence. Declared by A's owner, who is normally K's owner. Consumer: mResolution
of A's mKeys via K's §2.1. The whole-mKey relationship between K:x and A:x is carried by
K's :reaches (§2.4); without it they are unspoken and collide, which is safe.

### § 2.8-hierarchical-catalog-store (versus flat; traversal structure)

An mSort is :hierarchical when its mKeys decompose into an ordered chain of its own
mKeys, each a routing mKey the lookup crosses: a path into directory entries, a hostname
into resolver steps from a mVantage, a dotted unit name into its instance table. Otherwise it
is FLAT. A :hierarchical mSort may also contain INDEXICAL components whose mResolution
depends on the observing process (`/proc/self`); only the mSort owner can say which, and
an undeclared indexical component reads unknown. The engine derives the decomposition from
mKey SYNTAX, which is language, plus this
declaration (`30T` §5's syntax-versus-semantics line, unchanged). Consumer: mResolution
backings (§1.6) and therefore perishing (§3.3). Default: flat, so the mTraversal is the
mKey-CatalogStore as a whole, and any touch on the mKey-CatalogStore perishes every mResolution
through it, the coarse and safe floor. This is what replaces authored region predicates:
containment is membership in a mTraversal, and a mutator that touches a directory needs to
know nothing about files (`renaming-a-parent-moves-every-child-name`,
`namespace-composition-is-not-concatenation`).

### § 2.9-composite-sorts (roles)

A mTopic whose value depends on several inputs IN ROLES (a base and an overlay; a primary and
its replica set) is an mKey of a mCompositeSort minted by the author who knows the roles,
normally the tool author, and that mSort's identity is defined by its owner from its named
parts (ER modeling's associative mKey with role names). There is no set of identifying
mSort-PrimaryStores anywhere in the model: the mSort-PrimaryStore is at most one (§2.2), and plurality of
inputs is an mSort with structure (`composite-identity-is-structure-not-a-bag`). mPlacement of a
composite is the union of its parts' mPlacements.

### § 2.10-relation-table

| relation | arity | declared by | default | consumer | danger |
|---|---|---|---|---|---|
| :named-in | one per mSort, mKey-CatalogStore per mKey | mSort owner (mKey-CatalogStore: fixed / site / ambient) | no mKey-CatalogStore ⇒ mRoute only | mResolution | :guarantees-unique-referent (same from equality) · :guarantees-unique-name (disjoint from inequality) |
| :identified-in (the `resolve()`) | one per mSort | mSort owner | none ⇒ mKey-Primary = mKey-Natural | identity | :guarantees-unique-referent · :guarantees-unique-name · :sole-route · :rootness |
| :lives-in | many per mSort, sentinel | mSort owner | ⊤ ⇒ collides with everything of the mSort | collision | none positive; omission is the silent channel |
| :reaches + finished | many, per arm | mSort owner | unspoken ⇒ collide | cross-mRoot sparing | the premature finished record |
| :corresponds | per transition pair | transition owner | unknown | SAME mDerivation | a wrong mCorrespondence |
| :observer-independence | per (mSort, O) | mSort owner | dependent ⇒ no carry | SAME qualifier | a false independence |
| :named-like | one per mAspectSort | aspect owner | none | mResolution reuse | none |
| :hierarchical | per mSort | mSort owner | flat ⇒ whole-mKey-CatalogStore mTraversal | perishing | none (finer is value, coarse is safe) |
| :lends (+ sentinel) | per wrapper, per mSort | wrapper owner | ⊤ ⇒ walls | ambient mKey-CatalogStore supply | a wrong lend measures the wrong mVantage |
| mCompositeSort | per composite | the author holding the roles | n/a | identity | as any mSort |

## § 3-composition

### § 3.1-identity-of-a-key

identity(e of mSort K) is the mKey-Primary of e (via K's `resolve()` run from e's mVantage, or the
mKey-Natural if K has no `resolve()`), scoped in identity(mKey-PrimaryStore), recursively,
until a mRoot, a mRoute, or an unknown link. Each level carries the warrants K's owner declared
for that level's `:identified-in` edge. A mCompositeSort's identity is its owner's function of its parts'
identities. An mKey of an observer-dependent mSort carries the O-instance as part of its
mTopic. Nothing about identity consults the mVantage map except to know where to run `resolve()` calls
and which ambient mKey-CatalogStores to bind.

### § 3.2-compare (one chokepoint, four answers)

> Alias analysis's trichotomy: `same` is must-alias, `disjoint` is must-not-alias, `unknown`
> is may-alias, and `unspoken` is may-alias with the extra fact that no generator applies to
> the pair at all (no shared mKey-PrimaryStore, no mCorrespondence, no finished definition). It becomes
> decidable only when one arrives, and it is named for silence so that it never reads as
> separation. `24F`'s `MayAlias` was this vocabulary before the corpus renamed it.

compare(x, y) ∈ {same, disjoint, unspoken, unknown}, consumers exactly as today
(`compare-consumer-map`: same → the fact is about this mCell; disjoint → sparing under
`--risk-faultless-skips`; unknown and unspoken → the safe bottoms). For two mFullyQualifiedKeys:

- walk from the mRoots. If the mRoots differ, or one side is a mRoute and the other is not, the
  answer is UNSPOKEN, and sparing across the pair rides only the footprint side's finished
  definition (§2.4), as `30U` has it.
- at the first level where the two mKeys differ inside a shared mKey-PrimaryStore S: DISJOINT iff S's
  `:identified-in` edge for that mSort carries :guarantees-unique-name and every level above it carries
  :sole-route; else UNKNOWN. Deeper levels are not consulted; separation is decided once.
- if no level differs: SAME iff every level's equality is warranted (:guarantees-unique-
  mReferent on that level's `:identified-in` edge, or the engine's own transit-free local-mRoute claim of §1.9
  at the bottom); else UNKNOWN.
- any unknown link on either mKey: UNKNOWN.

For mDerivation sets: a warranted SAME and a warranted DISJOINT on one pair is a
contradiction, refuse both and attribute both authors; otherwise the strongest warranted
answer stands. SAME is transitive across mDerivations; DISJOINT is never chained. Universal
meet over backing sets is unchanged (`set-lifting-universal-meet`): sparing needs every
footprint-by-backing pair disjoint. Cross-mSort comparison needs no special rule: two
mFullyQualifiedKeys either share an mKey-PrimaryStore at some level or they do not. A shared mKey-PrimaryStore
with equal mKey-Primaries IS the cross-mSort SAME generator: two mSorts whose owners
identify into one mKey-PrimaryStore name one mReferent set through two mKey-Naturals, one thing in two
tables joined on a shared mKey, and this is the one place the corpus's "cross-mSort same does
not exist" is superseded. Genuinely different mKey-Primaries for one mReferent (an NFS
filehandle and the server's inode; a machine-id and a cloud instance-id) are two mDerivations
for one mTopic, reconciled by §1.7's coherence, never a second mKey inside one mKey-PrimaryStore.

### § 3.3-perishing (three mutator species, three invalidated facts)

- A ROUTING mutation (a mount, a symlink replacement, a rename, a user added, a hostname
  change) touches routing mKeys. Every mResolution whose mTraversal includes a touched
  mKey perishes; every mFullyQualifiedKey built on that mResolution reads unknown below the
  line; dependent SAME conclusions lose authority and dependent elisions demote to guards;
  dependent DISJOINT conclusions collide. The touched object itself is untouched
  (`a-path-is-not-a-referent`, `renaming-a-parent-moves-every-child-name`). Creation,
  deletion, and rename of an mKey are routing writes to its mKey-CatalogStore entry (its existence
  mCell), so `userdel alice; useradd alice` perishes every mResolution of the old mKey
  (`a-recreated-name-is-a-new-referent`); a footprint that omits the entry is the ordinary
  at-most omission knife, now visibly covering routing mKeys.
- A STATE mutation writes mCells through mPlacements: ordinary kill-reach, unchanged. A first
  write can also change a mKey-Primary
  (`identity-tokens-perish-on-write-not-only-on-rename`), so a state mutation
  whose footprint touches an mKey-PrimaryStore perishes the mTokens scoped in it.
- A LIFECYCLE mutation (a reboot, a re-provision) disturbs a mRoot-adjacent mKey (a boot, a
  tenure); every mKey-Primary scoped in it names a new mReferent afterward; mCells whose
  mFullyQualifiedKeys pass through it are new and unmeasured; mCells whose mKeys do not are
  untouched. What earlier designs declared as "keyed by Boot" or "invariant across Boot" is
  the shape of the mKey, not a declaration (`plans/30W` §7's patch day, derived).

In all three the engine withdraws authority; it never computes the successor identity.

### § 3.4-entry-and-lends

> A lend is dynamic binding: the wrapper rebinds an mKey-CatalogStore parameter for the guest's
> dynamic extent, exactly `parameterize` or `fluid-let`. Nesting, shadowing, and
> innermost-wins all follow from that frame; nothing about it is Dorc-specific.

A wrapper's entry :lends INDEXES for the mSorts it perturbs and nothing
else; the lent mKey-CatalogStores become the ambient supply for mKeys whose mSorts are :named-in those
mSorts. mSorts not lent inherit the caller's mKey-CatalogStore only after the
wrapper's completion sentinel over mSorts; before it they are ⊤. Leaf mSorts inherit
transitively through their mFullyQualifiedKeys with no speech from anyone
(`not-every-transit-changes-the-referent`). A wrapper may additionally declare
mCorrespondences across the mKey-CatalogStores it :lends (§2.5). Entry forms, siting vouches, the
escalation dial, and measure-in-context remain `plans/27C`'s; only the fallback lane changes
shape (§3.6).

### § 3.5-committee-law-satisfied

Every positive step is one author's line: a `:named-in` or `:identified-in` edge and its warrants, a `resolve()`, a :sole-route
flag, a mPlacement set and its sentinel, an entailment and its finished record, a
mCorrespondence, a :observer-independence, a :lends. The engine only chains and meets. A
granting composite ("these two accounts are one") is entailed jointly by the account
owner's :identified-in declaration and the database owner's `resolve()`, each speaking about their
own mSort (`28M:rul-composite-meets-toward-guard-run`). A withholding composite (a mount
perishing an account's mResolution) names nobody and needs nobody's consent. Attribution:
every survival names the :sole-route and :guarantees-unique-name lines it rested on; every SAME
names the `resolve()` calls and mCorrespondences; every perished conclusion names the footprint that
perished it.

### § 3.6-dissolved-kept-relitigated

Dissolved (no counterpart): the context slot as an identity input and the per-index-kind
trichotomy with its filtered meet (`plans/30W` §4, `26Ob` §10b); the invariance line as one
thing (split: store half measured, observer half §2.6); substrate mTokens; `kind__disjoint` as
an authored region predicate (containment is mTraversal membership); the canonicalizing `__resolve()` as a member
distinct from the `resolve()` (they are one member); the selector position and the selector dialect (`277` §3, `30J`);
the engine-side name-floor carves for File and index-kinds (they are the absent
:guarantees-unique-name warrant); the disclosed-weak default name floor
(`300:rul-reference-entity-name-floor`) **[LEAN: default safe even when painful]**;
"transport" as a lane for observer-independent mCells (one mCell, one fact); the store member's
argv blindness (the mSite fills the mKey-CatalogStore).

Kept unchanged: the verdict, vouch, and guard tier; footprints, :reaches, finished
definitions, and `--risk-faultless-skips`; the four-answer chokepoint and consumer map; the
universal meet; measure-in-context, entry forms, `safe-across`; the read-set closure as the
falsification net for unmarked reads (`27C` §4(a)(B)); binds as the key-minting act; the
mPlaceholder and standup `witness()`; the integrity plane; the committee law.

Re-litigated on the merits: `272` §5 addresses-are-not-coordinates (a mPlacement IS a
coordinate in another mSort, and identity is a mFullyQualifiedKey so no store-level collapse
follows); `279f` §3's refused transport chain (re-opened as mFullyQualifiedKeys of
a warranted `resolve()`, not as backing completeness); `271:rul-invariance-speech-act` (re-read:
textual mDerivation never licenses; an authored `resolve()` under a typed warrant does; the observer
half stays speech).

## § 4-epistemics

*(deliberately empty; to be filled backwards from §1–§3)*

## § 5-ux-gradual-enhancement-teaching

*(deliberately empty; to be filled backwards from §1–§3)*

## § 6-forcing-functions

### § 6.1-slugged-gotchas-and-what-each-forces

Referenced by slug (`Research/GOTCHAS.md`); the sentence there is the forcing function, the
line here is the model element it forces.

- `a-path-is-not-a-referent` — mResolutions are facts with mTraversal backings; a remount is a
  routing mutation (§1.6, §3.3).
- `a-host-is-not-a-partition` — the mVantage is an address, never an identity input;
  identity is a mFullyQualifiedKey through the file's mKey-PrimaryStore (§1.9, §2.2).
- `same-name-different-referent-per-viewpoint` — the mKey-CatalogStore can be
  ambient and lent; the mFullyQualifiedKey differs per mKey-CatalogStore (§2.1, §3.4).
- `not-every-transit-changes-the-referent` — leaf mSorts inherit through their mKeys; a
  transit that lends no mKey-CatalogStore on a mKey leaves it untouched (§3.4).
- `address-inequality-is-not-referent-inequality` — :named-in licenses no disjointness across
  mKey-CatalogStores; :sole-route is a separate, dangerous flag (§2.1, §2.2).
- `distinct-names-alias-within-a-kind` — :guarantees-unique-name is absent by default; the
  `resolve()` supplies the mKey-Primary (§1.5, §2.2).
- `containment-by-path-prefix-lies` — the File mKey-Natural's mKey-CatalogStore is not :sole-route;
  containment is mTraversal membership on the mKey-Primary side (§2.2, §2.8).
- `namespace-composition-is-not-concatenation` — mKey-CatalogStores are mKeys with
  identities, never strings composed by the engine (§1.4, §3.4).
- `renaming-a-parent-moves-every-child-name` — :hierarchical mSorts decompose into
  mTraversals; perishing by mTraversal membership (§2.8, §3.3).
- `a-name-resolves-from-a-vantage` — the network mVantage is part of the address and of a
  hostname's mTraversal (§1.9, §2.8).
- `resolution-is-set-valued` — :guarantees-unique-referent is the owner's opt-in per `:named-in` edge; a
  resolver's owner never makes it, so equal hostnames yield unknown and the landing is
  measured (§1.5, §1.9).
- `identity-tokens-have-clone-horizons` — :rootness is a dangerous claim; :guarantees-unique-
  mReferent is absent by default on mKey-Primaries (§1.4, §2.2).
- `a-name-is-not-a-target-over-time` — mPlaceholders, the standup `witness()`, integrity
  withhold; sameness of a target is continuity witnessed, never a spelling (§1.9).
- `a-store-is-not-one-inode` — mPlacement is many-valued and distinct from the mSort-PrimaryStore
  ; :sole-route is not implied by :lives-in (§2.2, §2.3).
- `an-omitted-store-breaks-invariance` — mPlacement totality; the completion sentinel;
  omission is the silent channel (§2.3).
- `nonzero-status-is-not-speech` — every warrant is typed speech, never an exit status; the
  rc regimes of `311a` §6 stand (§1.10).
- `composite-identity-is-structure-not-a-bag` — at most one mKey-PrimaryStore; roles are a
  mCompositeSort minted by the author who holds them (§2.9).
- `the-subject-includes-the-observer` — :observer-dependence as the surviving half of the
  invariance line; two observers' mCells share mPlacement but not identity (§2.3, §2.6).
- `correspondence-is-known-only-to-the-transition-owner` — :corresponds as a transition-owner
  generator; mapped :lends are mCorrespondences (§2.5).
- `identity-tokens-perish-on-write-not-only-on-rename` — a state mutation on an mKey-PrimaryStore
  perishes the mTokens scoped in it (§3.3).
- `a-recreated-name-is-a-new-referent` — creation and deletion are routing writes to the
  mKey-CatalogStore entry; a stable mKey over a recreate is a perished mResolution (§3.3).
- `recycled-keys-outrun-the-unwalled-span` — no lookup is a function by default; a
  mKey-CatalogStore that reissues mKeys never earns :guarantees-unique-referent, and the `witness()` cannot
  see a recycled mKey (§1.5, §1.9).

### § 6.2-conceptual-dead-ends-and-what-killed-each

Recorded as what-killed-it, so the dead end is not re-walked.

- IDENTITY AS A PER-KIND TABLE AGAINST AXES (the trichotomy invariant/keyed/⊤ per
  index-kind, `30W` §4; the filtered meet, `26Ob` §10b). Killed by: the mSort owner cannot know
  the axes; silence walls forever and a guess ("keyed by Host") plus a referent-transparent
  Host yields a wrong DISJOINT on a shared volume. The truth: a mCell's identity is a property
  of what its mKey denotes, not of the mSort the mKey is written in.
- THE CONTEXT AS PART OF THE FACT KEY, argued as the root cause of the above. Killed by the
  review: a qualified mKey is an address of a question and asserts no partition; the
  dangerous inference lived in the meet's generators, not in the mKey's existence. Surviving
  form: the mVantage is an address and a witness key (§1.9). This dissolved the A-versus-B
  binary of `311h`.
- STORE SETS, unioned and compared as bags (`311d` A1/A4; `311f`'s single `in:` for
  multi-store mTopics). Killed by base-and-overlay: same set, different roles, different
  answers. Also killed by the reflexivity defect of pairwise set equality. Surviving form:
  one mSort-PrimaryStore, mCompositeSorts for roles (§2.9).
- "STORED-IN" AS ONE RELATION conflating routing and containment (`311d`; and the current
  design's keyed-by versus stored-in). Killed by hardlinks, bind mounts, NFS: mKey-CatalogStore
  disjointness is not mReferent disjointness. Surviving form: :named-in, :identified-in with
  :sole-route, and mPlacement as three relations (§2.1–§2.3).
- TERMINAL TOKENS (`311f`'s `Measured(File, fsid:inode)`). Killed by NFS and by the mKey-PrimaryStore
  question: an inode is a mKey in a filesystem, a filesystem identifier is a mKey in whatever
  minted it; the File owner should never learn NFS. Surviving form: :rootness as an explicit
  dangerous claim; every mToken scoped (§1.4, §2.2).
- ONE GRADE ON A READ (`311f` B3's transparent/identifying). Killed by clones: equal machine-
  ids on two machines threaten SAME, which the disjointness grade cannot protect. Surviving
  form: two independent warrants per `:named-in` or `:identified-in` edge (§1.5).
- OBJECT IDENTITY CARRIES EVERY OBSERVATION ("one cell, one fact, whichever probe read it";
  `311f` B2). Killed by `test -w` under two users on one file. Surviving form: observer-
  dependence per mSort, absent means dependent (§2.6); the invariance line's observer half
  lives.
- AUTHORED REGION PREDICATES (`kind__disjoint`, `30W:rul-disjoint-is-an-rc-predicate`; the
  `@subtree` selector idea). Killed by the symlink-retarget case: separation of two objects
  says nothing about whether changing one retargets a mKey for the other; and by the human's
  granule observation that the natural description unit is the atomically-disturbable one.
  Surviving form: :hierarchical mSorts and mTraversals; containment as mTraversal
  membership (§2.8).
- IDENTITY BINDINGS BACKED BY THEIR TARGET OBJECT (`311d` A6). Killed by the same symlink
  case. Surviving form: mResolution backed by mTraversal (§1.6).
- UNION TOTALITY WITH NO AUTHOR (`311d` A1's mSort-level ∪ mSite-level sentinel). Killed by
  asking who closed the union. Surviving form: the owner declares and closes; mSites fill
  mKey-CatalogStores (§2.1).
- CONTRADICTION-CHECKING AN INVARIANCE LINE WITH IDENTIFYING-GRADE INEQUALITY (`311d` A5).
  Killed by the grade itself. Surviving law: a grade governs every consumer (§2.2).
- SHARED ANCESTORS AS COLLISIONS (`311f` B2's cross-mSort rule). Killed by the package status
  file and the unit file sharing a filesystem. Surviving form: ancestors are mKey-PrimaryStores;
  `compare()` at the divergence (§3.2).
- THE DISCLOSED-WEAK NAME FLOOR (`300:rul-reference-entity-name-floor`; kept in `311d`).
  Killed by the human's default-safe lean and by its own per-mSort carves. Surviving form:
  :guarantees-unique-name is typed, never assumed; the reviewer's window specimen also showed
  the floor was applied without its locality premise.
- A DEFINITIONAL EQUAL-KEYS DEFAULT ("equal mKey-Naturals in one mKey-CatalogStore across an unwalled
  span reach one mReferent, because that is what an mSort is"). Killed by: it feeds only
  the dangerous consumer (kill-reach already collides on unknown, so SAME buys it nothing);
  recreation under a stable name, pid reuse, round-robin and cached lookups, and `:latest`
  tags are ordinary ops, not exotica; and the claim's owner is different at every level.
  Surviving form: the engine vouches only the transit-free local mRoute; every mKey-CatalogStore's
  lookup-functionality is its owner's :guarantees-unique-referent, opt-in (§1.5, §1.9).
- MEASUREMENT MAKES DECLARATIONS REDUNDANT (`311h`'s framing). Killed by `311i` §0: a `resolve()`
  establishes a mToken, not its mKey-PrimaryStore, mTopic, warrant, applicability, or sufficiency.
  Surviving framing: measurement relocates speech to questions the owner can answer.
- RENAMING RESOLVE INTO IDENTITY FIXES THE FALLTHROUGH IDIOM. Killed by the observation that
  a renamed member can still echo a fallback and exit 0; the repair is the completion-only
  rc regime plus a taught decline, not the name.
- THE DISAGREEMENT CANARY AS A SAFETY ARGUMENT (`311d` A8's bare-`7` rescue). Killed by
  noting the second probe exists only because nothing was elided on the first; absence of
  disagreement is not evidence, and a wrong bind is a wrong SAME that lands, attributed.
- CONVERGENCE OF TWO SYSTEMS FROM ONE WINDOW AS EVIDENCE (`311h`). Killed by the human's
  first bullet. Surviving discipline: clean-context adversarial review before any design-of-
  record.
- THE ASPECT AS A THIRD COORDINATE POSITION. Killed by three independent pushes (one
  mKey-PrimaryStore, per-aspect mPlacement, per-aspect :observer-dependence). Surviving form:
  mAspectSorts (§1.8, §2.7). Recorded as forced, not chosen for tidiness.

## § 7-open-questions (carried into the next sitting; not rulings)

- `open-local-route-claim-residue` — the engine's own transit-free local-mRoute claim (§1.9) is
  believed sound modulo indexical components an mSort owner declines (`/proc/self` on
  procfs); no further hole was found, and none was hunted adversarially.
- `open-injectivity-derivation` — the human asked for a narrow, stable mDerivation of
  :guarantees-unique-name that does not depend on an attentive author; none was found; the
  typed warrant on the `resolve()` is the current answer, held as a dangerous corner.
- `open-no-transit-path-cost` — the static reconstruction of a no-transit probe path for
  observer-independent mCells (§2.6) is priced at one declaration per mSort; whether the
  stdlib will genuinely pay it for every mSort, and what the access-refusal fallback looks
  like before reactive probing exists, is undesigned.
- `open-derivation-algebra-formalization` — §3.2's mDerivation meet (coherence, transitivity
  of SAME only) is stated, not specified; the per-property meet direction registry `28M` §8
  wanted is the same object.
- `open-composite-sort-authoring` — who mints role-bearing mCompositeSorts in practice, and
  whether a tool author minting one per tool is the cargo-cult shape the boilerplate razor
  forbids.
- `open-aspect-sort-verbosity` — mAspectSorts are forced; their authored surface is not
  designed, and sugar is explicitly deferred to the UX pass.
- `open-observer-sort-inventory` — which mSorts parameterize VALUES (User) as
  opposed to only routing mKeys; the model treats it per (mSort, O) but the stdlib will want a
  short list.
- `open-context-slot-build-shape` — `plans/30W` item 1 builds the slot as a product over
  index-kinds; under this model the slot is a mVantage (address and witness key) only, and
  the trichotomy meet must not be built; the as-built audit the human deferred decides how
  much of the existing slot is reusable.
- `open-security-and-hostile-host` — every `resolve()` here is host-produced bytes crossing the
  intake boundary; nothing in this model widens what a host may mint (mTokens are compared,
  never decoded), but the standing review gates apply before any of it becomes design.

## § 8-ledger (historical; pointers, not content)

The r31-prep sittings this document synthesizes, in order:

- `notes/311a` — the kernel-shape and rc-law sitting (2026-09-05/06): the context slot as a
  map over mSorts; the rc regimes; `kind__overlaps` brought into the kernel and then re-ruled
  as `kind__disjoint` (`30W:rul-disjoint-is-an-rc-predicate`). Its §7 cleanup pass is still
  owed and now largely superseded in intent by this document.
- `notes/311b` — the sibling conductor's index-identity and committee-speech ledger; unread
  by this document's author at the human's direction, so this document may duplicate or
  contradict it.
- `notes/311c` — the regrounding digest written before the rewind: what to read, the
  footguns in the reading, the human's framing of the sitting, and the then-current system.
- `notes/311d` / `311f` — the two strawman systems (narrow refinement; whole cloth), authored
  in one window under the containment lens; `311h` their comparison. Both contain rules now
  known wrong (§6.2); read them as specimens.
- `notes/311e` / `311g` / `311i` — the foreign-lineage reviews of each system and their
  synthesis; mined for paths not taken, and the source of §6.2's last nine entries.
- `Research/GOTCHAS.md` — minted 2026-09-08 from these sittings; twenty-two items at this
  writing; referenced by slug.
- Vocabulary minted here and used ahistorically above: `unspoken` (née `unrelated`, `30U` §7
  and `compare-consumer-map`).

Human leans typed across the sittings and treated here as leans, never rulings: default safe
even when painful; no aliases or shortcuts for subsumed members; no perf-motivated holes;
dangerous operations rare and long-named; store as a flagged mSort rather than a
separate type; one relation with opt-in flags rather than three; aspects in scope once
forced; nearly everything in this corner open to re-litigation given internal coherence.

Design-of-record documents this model proposes to supersede in part once ruled: `plans/30W`
(§1 :rootness, §2 generators, §4 trichotomy, §5 mWorlds), `notes/272` (the member and §5's
fence), `plans/30T` §6 (the per-aspect relation mapping and the v0 floor), `plans/30U` §7
(the store-collide consumer), `notes/26Ob` §10b (the filtered meet; the per-index relation
table), `plans/27C` §4 (the fallback lane's shape, not its measure-in-context default), `277`
§3 and `plans/30J` (the selector dialect), `plans/281`'s selector position. Everything on the
footprint side and the verdict side is untouched.
