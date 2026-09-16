# 311j — Identity and relation: the two-species model (the current cut of 311)

> AI-authored PREPARATORY MODEL (Fable, the `r31-prep-design-duck` sittings; the human present
> and adjudicating). Notes-tier, ahistorical: the current cut of the `notes/311` model, kept as
> a separate document until it replaces 311. Nothing here is ruled; grades on the conductor's
> claims are +SURE / ~SUSPECT / -GUESS / --WONDER, and **[LEAN]** marks a human lean paraphrased
> from chat, never a ruling. Authority: root docs, `spike/CLAUDE.md`, and the welds outrank this;
> where this document disagrees with `plans/30W`, `notes/272`, `plans/30T` §6, `plans/30U` §7,
> `notes/26Ob` §10, or `plans/27C` §4, it is a deliberate proposal to re-litigate them.
>
> Purpose: the abstract objects, relations, and laws of identity across mutually-unknowing
> authors, stated fully enough that the concretization step (names, spellings, user-facing
> usage — the `312` series) can be built on it without re-modeling. §1–§3 are the MODEL. §4 is
> the forcing-function inventory.
>
> Scope discipline: abstract objects and relations only. No syntax, no strawman sh, no UX, no
> gradual-enhancement ladder, no implementation. The sole exception is `sh` vocabulary used as
> a referent-agnostic example of a catalog or a transition.
>
> Naming discipline: every model object is written mFixedTerm; every relation, attribute, or
> warrant is written `:fixed-term`, always preceded by a space; every abstract operation is
> written `op()`; every concrete authored member keeps the `__name()` form; a derived view of a
> species is written species-hyphen-gloss (mKey-Primary), never a declared species; an in-Dorc
> mSort or mScheme is always written with its prefix (`sm.File`, `sm.Path`). Where a prior-art
> term overlaps precisely, the document takes it and says which domain it comes from; where a
> nearby term is a false friend, the heading or a blockquote under it says so.

## § 0-one-screen

Dorc removes a line only on connected claims that the line is unnecessary: a measurement
claimed before anything runs, plus an author's vouch. The claim can be destroyed by an earlier
line that really runs, so the engine must decide whether the piece of the world an earlier line
touches is the same piece a later line's licence depends on. That is identity, and it has two
consumers with opposite failure directions: SAME lets one fact stand for another and
under-executes when wrong; DISJOINT lets a licence survive a write and under-executes when
wrong. UNKNOWN is safe for both. The engine knows only syntax, authored speech, and what
authored probes returned; it never decodes a key. No single author knows the whole path from a
tool's argument to a measurable mReferent, and viewpoint transitions authored by yet other
people change which mReferent a key reaches mid-book.

The model has two declared species, semantically separate and never substitutable. An mScheme
is a way of writing down which thing is meant (a path, an inode number, a package name, a unit
name, a hostname): it has a `resolve()`, and it is what every bind and every mark names. An
mSort is the thing meant (a file, a package, a unit, a host): nobody writes one down; it exists
so that several mSchemes can be told they name one referent class, and so that what is true of
the thing however it was named is declared once. Every mSort has exactly one primary mScheme,
whose keys are the addresses the mSort's own store answers with; every other mScheme of the
mSort yields the primary's key. Every mKey carries exactly one other mKey it was resolved
inside, its mParent: through the primary mScheme that edge carries identity, through a
secondary it carries routing. Identity is the primary key scoped in its mParent, recursively,
until a declared mRoot, the transit-free local mRoute, or unknown. Two things `compare()` by
walking their chains to the first divergence inside a shared mParent and asking what the arm
into it warrants. Every positive answer rests on an explicitly typed, long-named,
absent-by-default warrant owned by one author, sitting on one arm of one `resolve()`; silence
is unknown. Transitions are of three species, each perishing a different fact. The context
slot survives as a mVantage, an address and a witness key, never as an identity input.

## § 1-model-objects

### § 1.1-referent

A piece of the world that has state: an inode, a database row, a package record, a kernel
parameter, a running process, a machine, a mount table. The engine never holds an mReferent.
It reaches mReferents only through mKeys, mTokens, and mDerivations, and every identity
question is ultimately "do these two mKeys reach one mReferent". mReferents are what GOTCHAS
are about.

### § 1.2-sort

> A *sort* in the many-sorted-logic sense: a carrier, one referent class with one accountable
> owner. Never the pure-type-system sense (a universe) and never a PLT kind (the type of a
> type); the word names the carrier and nothing else. Under this model an mSort is not itself
> a naming scheme; the schemes are a second species (§1.3).

A referent class with one accountable owner (reverse-DNS naming, no registry,
owner-adjudication as the social contract). An mSort has no keys of its own and no
`resolve()`; it is never valid where an mScheme is. It fixes: which mScheme is its primary
(§1.3, §2.2); its mPlacements (§2.4); its effect entailment and finished definition (§2.5);
its :observer-dependence (§2.7); and its cells, which are mSorts under it (§1.9). An mSort's
owner speaks only about the mSort's own relations to its immediate neighbours; the engine
composes. An mSort is named only when something is declared about it as a whole; the floor
of §1.3 lets an mScheme exist before its mSort has a name.

### § 1.3-scheme

> A naming scheme: a term language over an mSort's referents, in the many-sorted-logic sense
> of terms. Never a sort.

A way of writing down which mReferent of some mSort is meant, with one accountable owner. An
mScheme fixes: its `resolve()`; what it yields, exactly one of (a) it is the `:primary-of` its
mSort and carries that mSort's identity arms (§1.6, §2.2), or (b) it `:yields` the key of
another mScheme of the same mSort (§2.1); where its keys are looked up when it is secondary
(§2.1); its lookup warrants, per arm (§1.5); and whether it is :hierarchical (§2.8). An mScheme
belongs to exactly one mSort: declaring one under two mSorts is a static contradiction,
refused; reuse of one scheme's lookup by another is delegation inside a `resolve()` body,
never shared membership.

The floor: an mScheme that declares neither `:primary-of` nor `:yields` is the primary of an
mSort nobody has named, with an identity `resolve()`, no warrants, and the mRoute as its only
mParent; that mSort acquires a name the first time its owner declares something about it as a
whole. There is no default mScheme: a bind or a mark always names an mScheme, never an mSort.

### § 1.4-key

> RDBMS terms, taken whole with their culture: a primary key is what the store itself
> identifies rows by, and is unique only within its own table; a natural key is
> domain-meaningful, user-typed, may alias, and is never trusted as identity. Clones are two
> tables with colliding primary keys.

An mKey is a value (a string an author wrote or a `resolve()` returned), its mScheme, and its
mParent (§1.6). A bare string means nothing; a value with an mScheme but no mParent is not yet
an mKey. An mKey is typed by its mScheme at the bind or at the emission that produced it; a key
of the primary mScheme is thereby a key of the mSort. Two derived views: mKey-Natural, a key of
a secondary mScheme (what tool authors and books write; usually ambiguous — aliases, relative
names); mKey-Primary, a key of the primary mScheme (what the mSort's store answers with; where
the dangerous warrants can honestly be placed). When an mSort's only mScheme is its primary,
the two views coincide.

### § 1.5-token-and-the-two-warrants

> The two warrants are OWL's inverse-functional and functional properties, and OWL's inference
> consequences transfer exactly (inverse-functional is what licenses `sameAs`). OWL's names
> swap direction with the property's orientation, which is why ours are spelled out instead.

A mToken is an mKey-Primary's value: bytes the primary's `resolve()` returned, compared for
equality only, never decoded (`inv-referent-agnostic`), always scoped in an mParent. Any
lookup — a secondary mScheme's, or an arm of a primary's — has two independent properties,
and each is a separately declared, absent-by-default warrant on that lookup:

- :guarantees-unique-referent — within one mParent, equal keys reach one mReferent; the lookup
  is a function. Licenses SAME from equality. Fails for round-robin resolution
  (`resolution-is-set-valued`), for recycled keys (`recycled-keys-outrun-the-unwalled-span`),
  and for cloned identifiers presented as roots (`identity-tokens-have-clone-horizons`).
- :guarantees-unique-name — within one mParent, one mReferent has one key. Licenses DISJOINT
  from inequality. Fails for symlinks and hardlinks, package `provides`
  (`distinct-names-alias-within-a-kind`), route-qualified handles.

Both are properties of the lookup, holding for every key that takes it inside any one mParent,
not of a single key and not of that mParent: declared once, by the owner of the mScheme (for a
secondary) or of the arm (for a primary), holding wherever those keys are looked up. Neither
holds by default anywhere: not on secondary schemes, not on primaries, not on roots. The one
lookup the engine vouches for itself is the transit-free local mRoute (§1.10). Every other
claim that a lookup is a function is its owner's opt-in (the inode arm's owner makes it; a
resolver's owner and a pid table's owner never do), and across a transit it is never claimed,
only measured and witnessed.

### § 1.6-parent-and-arm

Every mKey has exactly one mParent, another mKey: the one it was resolved inside. What the
edge carries depends on the mScheme the key belongs to:

- through a secondary mScheme, the mParent is the catalog the key was looked up in (a mount
  namespace for a path; a passwd database for a login name; a process table for a pid), and
  the edge carries ROUTING: which instance, and perishing (§3.3). It never carries identity.
- through the primary mScheme, the mParent is the store the mSort's referents are identified
  in (a filesystem for an inode; a user namespace for a uid; a dpkg database for a canonical
  package name), and the edge carries IDENTITY: it is what `compare()` walks (§3.2). The
  primary's key is its mParent's own address for the referent, so catalog and store are one
  object at the primary.

An arm is one edge type of a primary mScheme's `resolve()`. It declares the mParent's mSort
(`:identified-in`) and the warrants that hold for keys taking that arm (§1.5, :sole-route,
:rootness; §2.2). A primary mScheme has one or more arms, and which arm a key takes is decided
from the key's own bytes, never from anything the emitter of the key could supply — so an
emitter (a secondary mScheme, possibly a stranger's) can be wrong only about its own lookup,
never about the warrants. The mParent INSTANCE for a primary key is supplied by whichever
`resolve()` yielded the key (§2.1), or, for a key bound directly under the primary, by the
mVantage. An arm with no `:identified-in` scopes its keys in the mRoute (§1.10). A mRoot is
an arm that declares `:rootness`: its keys need no mParent, which is the claim that they are
globally comparable (the DNS root, whose keys are fully-qualified; a cloud instance-id honestly
minted unique). Nothing is a root by default; `identity-tokens-have-clone-horizons` is the
standing witness against casual :rootness.

### § 1.7-resolution-and-traversal

A mResolution is the fact that key N of mScheme S, resolved inside mParent P, at program point
p, reaches mReferent R. It is a FACT with a BACKING, and its backing is the mTraversal: the
ordered chain of routing keys the lookup crossed (each directory entry and symlink for a path;
the resolver configuration and mVantage for a hostname; the unit table for a service name). A
:hierarchical mScheme's structure fixes how a key decomposes into a mTraversal (§2.8). A
mResolution perishes under ordinary effective-mWorld reach from any mutator whose footprint
touches any mTraversal member. Its target object is NOT its backing: a key can stop reaching
an object without the object changing (`renaming-a-parent-moves-every-child-name`,
`a-path-is-not-a-referent`), and an object can change without its key changing.

### § 1.8-fully-qualified-key-and-derivations

A mFullyQualifiedKey is the recursive identity of an mKey: its mKey-Primary, scoped in its
mParent, whose identity is itself a mFullyQualifiedKey through the mParent's own primary arm,
terminating at a mRoot arm, at the mRoute (§1.10), or at unknown. A mFullyQualifiedKey is one
mDerivation of identity. A mTopic — what a claim is about: a mCell, together with the observer
instance when the mCell's mSort is observer-dependent (§2.7) — may carry several mDerivations
with different generators: its mFullyQualifiedKey, a provider-supplied identifier, a
mCorrespondence asserted by a transition owner (§2.6). mDerivations combine by coherence
(§3.2), never by priority. Equality composes transitively across mDerivations; separation is
decided at one divergence and is never chained. (`witness()` is reserved throughout for the
standup re-measurement of §1.10.)

### § 1.9-cell

A mCell is the unit that has a value: what a probe measures and a mutator writes. A mCell is an
mKey of an mSort whose values are measurable, and such an mSort (`enabled`, `active`,
`contents`, `mode`) is declared like any other, with one difference of shape: its primary
mScheme's only arm is `:identified-in` the mSort whose referents it is a property of (its
bearer), and its own key is a singleton, so a cell's identity is its bearer's identity plus
which property. Two cells of one bearer (`enabled`, measured in a symlink's existence;
`active`, measured in pid 1's memory) are two mSorts with two mPlacements and two
:observer-dependences, and their mFullyQualifiedKeys differ exactly where one survives a
reboot and the other does not, with no declaration about reboots by anyone. The whole-bearer
claim (`disturbs` a service) reaches its cells by the bearer's effect entailment (§2.5), and
absent that entailment collides with them as unspoken, which is the safe bottom. There is no
aspect species and no selector position; a bind names keys, and a cell is only ever named by
its own mScheme.

### § 1.10-vantage-route-placeholder-witness

A mVantage is the ADDRESS a probe reached an mReferent from: the mEntryChain, expressed as a
finite map from catalog mSorts to the instances that wrappers lent (§3.4). It is not part of
any mKey's identity. It has three jobs: it says where a `resolve()` executes; it supplies the
ambient mParent for every secondary key whose mScheme is looked up in a lent catalog mSort;
and, for an arm with no `:identified-in`, it is the mRoute, the last-resort mParent. For
transit-free local execution the engine itself vouches the mRoute: two same-spelled keys of
one mScheme in one unwalled span are one mPlaceholder, because one shell process resolves one
cwd-qualified key in one mount table. Across a transit the mRoute is never vouched, only
measured and witnessed; across mVantages it is unknown. mFullyQualifiedKeys whose mTokens are
not yet measured are mPlaceholders keyed by (key, ambient mParents, mEntryChain); the probe
standup binds them; the apply standup re-reads them through the same entry and compares (the
`witness()`); mismatch is integrity, never a verdict input
(`26Ob:cor-standup-witness-licenses-bare-line-elision`). The `witness()` `compare()`s mTokens
at one instant and cannot see a recycled key (a reissued pid or inode); that stays on the
outside-churn horizon (`recycled-keys-outrun-the-unwalled-span`).

### § 1.11-site-and-claim-species

A mSite is within a book line, with an argv, an mEntryChain, and a program point. Speech at or
about a mSite, each with one author:

- a VERDICT FACT: a mCell's measured value, backed by the marked read set
  (`30T:req-verdict-marks-every-read-cell`), vouched by the tool-oracle author;
- a FOOTPRINT CLAIM: an at-most write set per matched shape, with its completion witness
  (`30U`), by the tool-oracle author or the filesystem binder;
- an EFFECT ENTAILMENT: what disturbing a key of an mSort drags along, with the finished
  definition (`30U`), by the mSort owner;
- a mCorrespondence: key X inside mParent A is key Y inside mParent B, by the owner of the
  transition between A and B (§2.6);
- the per-mScheme DECLARATIONS: `:yields` or `:primary-of`, the arms, the warrants, the catalog
  and its supply mode, :hierarchical (§2.1, §2.2, §2.8);
- the per-mSort DECLARATIONS: mPlacements, :observer-dependence, cells (§2.4, §2.7, §1.9);
- the wrapper's :lends and their completion sentinel (§3.4).

## § 2-model-relations

Each relation below states: arity, who declares it, its default, which consumer reads it, and
which warrant makes it dangerous. The human's naming law applies throughout (**[LEAN]**:
dangerous operations rare and long-named; safe ones short) but no spellings are proposed here.

### § 2.1-yields (a secondary scheme, into the primary; the catalog)

mScheme S `:yields` mScheme T, both of one mSort: S's `resolve()`, run in the mVantage, maps a
key of S to a key of T together with the value that names T's key's mParent, and S declares
which mScheme of the mParent's mSort that value is read in (`:parent-key-of`). T is the primary
or a secondary that in turn yields the primary; the chain always terminates at the primary.
Declared by S's owner, once per secondary scheme. Where S's keys are themselves looked up — S's
own mParent, the catalog — is declared once per secondary scheme with a supply mode: FIXED (S's
owner names it: the dpkg status file), SITE (the tool author's bind fills it from argv: `--db`),
AMBIENT (the mEntryChain's lent instance for that catalog mSort: the current mount namespace
for paths). The catalog may also be COMPUTED from ρ and from another mScheme's `resolve()`
(`${PIPX_HOME:-$HOME/.local/pipx}`); an unknown input makes the instance unknown. Consumer:
mResolution (§1.7) and the identity chain (§3.1). Default: an mScheme with no `:yields` and no
`:primary-of` is the floor of §1.3. Warrants (§1.5) on S's lookup govern what equality and
inequality of S's keys license before the primary is reached; they never license across
catalogs: catalog-disjointness is not referent-disjointness (`a-host-is-not-a-partition`,
`address-inequality-is-not-referent-inequality`, `containment-by-path-prefix-lies`). A
`resolve()` declines on referents its mSort does not describe (a path that reaches a socket,
under a scheme into files): the mechanical net against lazy borrowing.

### § 2.2-primary-of and the arms (the identity edge; :identified-in, :sole-route, :rootness)

> ER modeling's identifying relationship: a weak entity's primary key includes its identifying
> entity's key; a strong entity has its own. Identifying is NOT containing. The containing half
> is :sole-route below (networking: single-homed), a separate, dangerous claim; conflating the
> two is the NFS mistake.

mScheme P is `:primary-of` mSort K, once per mSort: P's keys are K's store's own addresses, and
P's `resolve()` is the identity on the key, existing to carry the arms. Each arm declares
`:identified-in` mSort M — the mParent's mSort, the thing whose identity determines K's for
keys taking that arm — and the warrants for those keys, all absent by default, all declared by
P's owner:

- :guarantees-unique-referent and :guarantees-unique-name (§1.5), governing what mToken
  equality and inequality license at K's level of a mFullyQualifiedKey;
- :sole-route — K-referents taking this arm are reachable only through their mParent, so
  disjoint mParents imply disjoint K-referents. Required at every level between a divergence
  and the leaf for the divergence to yield DISJOINT (§3.2). Provable by constitution (a process
  in its kernel; an inode in its ext4 table; a package in its dpkg file) or by contract (ext4
  in its boot: concurrent mounting unsupported), and never for views (a client NFS mount; an
  NSS view of LDAP; a chroot's view of a bind mount), which get no :sole-route. Parent
  classification and :sole-route are one declaration, on the arm. A record whose state spans
  several files (`a-store-is-not-one-inode`) is a mPlacement matter (§2.4), not a :sole-route
  failure; a hardlink is a :guarantees-unique-name failure on the path scheme, dissolved by the
  path scheme yielding the inode;
- :rootness — the arm declares no mParent and thereby claims its keys are globally comparable;
  equivalent to declaring :guarantees-unique-referent over the whole world; fails for cloned
  identifiers. A mToken that can be duplicated across instances of its would-be mParent must be
  scoped in something smaller or left un-warranted.

Which arm a key takes is a function of the key's bytes alone (§1.6); the mParent's TYPE
therefore varies per arm (an ext4 filesystem in the mRoute; an NFS filesystem in a host; a
tmpfs in a boot), and the child mSort's owner never learns the parent's types — the parent
mSort's primary classifies, one level up, each owner speaking one level. A grade governs every
consumer of the answer it grades, corroboration and contradiction included: a lookup without
:guarantees-unique-name cannot contradict anything by returning two different mTokens.

### § 2.3-parent (the one edge per key)

One per mKey, derived from §2.1 and §2.2: for a key of a secondary mScheme, the catalog it was
looked up in; for a key of the primary, the store named by the arm it took. Never declared
separately, never plural, never a species of its own — the far end is an ordinary mKey of the
mSort the scheme or arm named. Consumer: routing and perishing through a secondary; identity
through the primary. The context slot has no counterpart here: a mVantage supplies instances
and is never an mParent.

### § 2.4-lives-in (placement; the read footprint)

> The correct name is footprint. `disturbs` squats it for the write footprint, and `backing`
> squats the per-fact read footprint (`23M`). mPlacement is the mSort-declared read footprint
> that a verdict fact's backing refines at probe time.

K's referents' state is affected by writes to these keys. Many-valued: an edge type from an
mSort (a template the site or environment fills) to mKeys of other mSorts. Declared by K's
owner, per mSort, with a completion sentinel closing the set. Every mSort in a chain may
declare mPlacements, not only leaves (a loop-backed filesystem's state lives in a file of the
outer filesystem; `dd` over the image rewrites every inner fact). Exactly one mPlacement of a
bearer sits on its identifying chain — the mParent through the primary — and the rest are
non-identifying, followed for interference only. Consumer: collision, and the bound of §2.5. A
footprint touching any mPlacement collides with K's mCells; an incomplete mPlacement set loses
protection for sparing (an omitted mPlacement is a silent channel:
`an-omitted-store-breaks-invariance`, `a-store-is-not-one-inode`) and never licenses anything
positive. mPlacement is distinct from the mParent: the mParent is exactly one and answers
identity; mPlacement is many and answers interference. Two mCells with different mParents can
share a mPlacement and therefore collide without being the same
(`the-subject-includes-the-observer`: two observers' writability mCells share the file's mode).
Under §2.5 a declared mPlacement set is knife-tier: it bounds what the finished definition may
spare.

### § 2.5-reaches (effect entailment), and the finished definition

As `plans/30U`: disturbing a key of K entails disturbing these keys of other mSorts;
arm-incremental, collide-adding; the reached completion record finishes the definition and is
the sole licensor of sparing across UNSPOKEN pairs (§3.2). Declared by K's owner. This relation
is about effects, not identity, and it is what carries a package's postinst enabling its unit,
a restart killing a main process, and every other cross-mSort consequence that no
mFullyQualifiedKey expresses. One bound, on the licensor: "nothing else" means nothing outside
K's declared mPlacements and the declared mPlacements of the mSorts K reaches; the engine
`compare()`s mPlacements by the ordinary chokepoint, and an overlap, or an undeclared
mPlacement on either side, collides. The finished definition is a within-mWorld sentence; it
never speaks across mRoutes or mRoots (§3.2).

### § 2.6-corresponds (declared sameness across a transition)

> A scoped `sameAs`. `23M` rejected `owl:sameAs` for admin-declared cross-mSort co-reference;
> this is the other case, same-mScheme keys across two catalogs, asserted by the one author who
> owns the transition between them (the NAT table, the pid-namespace map, the mount line).

Key X inside mParent A denotes the same mReferent as key Y inside mParent B. Declared by the
owner of the TRANSITION between A and B, which is neither key's mScheme owner: the container
manager knows guest pid 1 is host pid 4821
(`correspondence-is-known-only-to-the-transition-owner`); `sudo -u alice` knows inner "me" is
outer "alice"; a mount line's oracle knows keys under the mountpoint are keys under the export
on the named server, from this mVantage. Consumer: a SAME mDerivation (§1.8), vouch-tier,
attributed to the transition author. Default: absent, so keys across a transition `compare()`
unknown unless a chain binds mTokens on both sides. Danger: a wrong mCorrespondence is a wrong
SAME. This generalizes the mapped lend of `273` and is the model's only declared sameness
generator besides mToken equality.

### § 2.7-observer-dependence

K's mCells' VALUES depend on which key of mSort O the measurement was taken under. Declared by
K's owner per mSort as its complement, :observer-independence of O. Default: a mCell measured
under a lent key of O is assumed to depend on it, so its fact is about (mReferent, O-instance)
and never stands for the same mReferent under another O-instance. Consumer: the SAME consumer,
as a qualifier on the claim's mTopic. This is the surviving half of the old invariance line
(`271:rul-invariance-speech-act`): its store half is measured away by §2.2, its observer half
cannot be measured by any `resolve()` because the object is the same and the answer differs
(`the-subject-includes-the-observer`), and it must remain speech. Measurement in the denoted
context (`plans/27C`) stays the default lane; carrying a fact across an O-shift requires this
declaration.

### § 2.8-hierarchical (traversal structure; on the scheme)

An mScheme is :hierarchical when its keys decompose into an ordered chain of its own keys,
each a routing key the lookup crosses: a path into directory entries, a hostname into resolver
steps from a mVantage, a dotted unit name into its instance table. Otherwise it is FLAT (an
inode number; a uid). A :hierarchical mScheme may also contain INDEXICAL components whose
mResolution depends on the observing process (`/proc/self`); only the mScheme owner can say
which, and an undeclared indexical component reads unknown. The engine derives the
decomposition from key SYNTAX, which is language, plus this declaration (`30T` §5's
syntax-versus-semantics line, unchanged). Consumer: mResolution backings (§1.7) and therefore
perishing (§3.3). Default: flat, so the mTraversal is the catalog as a whole, and any touch on
the catalog perishes every mResolution through it — the coarse and safe floor. This is what
replaces authored region predicates: containment is membership in a mTraversal, and a mutator
that touches a directory needs to know nothing about files
(`renaming-a-parent-moves-every-child-name`, `namespace-composition-is-not-concatenation`).

### § 2.9-composite-sorts (roles)

A mTopic whose value depends on several inputs IN ROLES (a base and an overlay; a primary and
its replica set) is a key of a mCompositeSort minted by the author who knows the roles,
normally the tool author, and that mSort's identity is defined by its owner from its named
parts (ER modeling's associative entity with role names). Plurality of inputs is an mSort with
structure, never a set of mParents (`composite-identity-is-structure-not-a-bag`). mPlacement of
a composite is the union of its parts' mPlacements.

### § 2.10-relation-table

| relation | arity | declared by | default | consumer | danger |
|---|---|---|---|---|---|
| `:primary-of` + arms (`:identified-in` per arm) | one scheme per mSort; one or more arms | the mSort's owner, on the primary | none — the floor of §1.3 supplies an unwarranted identity primary | identity (§3.1) | :guarantees-unique-referent · :guarantees-unique-name · :sole-route · :rootness, per arm |
| `:yields` (+ `:parent-key-of`; the catalog and its supply mode) | one per secondary scheme | the scheme's owner | none ⇒ the scheme is a floor primary | mResolution; the chain to the primary | :guarantees-unique-referent · :guarantees-unique-name on the lookup; a wrong yield is a wrong SAME, attributed to the yield |
| `:parent` | one per key | derived (§2.3) | n/a | routing (secondary) · identity (primary) | none of its own |
| `:lives-in` | many per mSort, sentinel | mSort owner | ⊤ ⇒ collides with everything of the mSort | collision; the bound on the finished definition | none positive; omission is the silent channel; the set bounds what may be spared |
| `:reaches` + finished | many, per arm | mSort owner | unspoken ⇒ collide | cross-mSort sparing, within one mWorld, bounded by mPlacements | the premature finished record |
| `:corresponds` | per transition pair | transition owner | unknown | SAME mDerivation | a wrong mCorrespondence |
| `:observer-independence` | per (mSort, O) | mSort owner | dependent ⇒ no carry | SAME qualifier | a false independence |
| `:hierarchical` | per mScheme | mScheme owner | flat ⇒ whole-catalog mTraversal | perishing | none (finer is value, coarse is safe) |
| `:lends` (+ sentinel) | per wrapper, per catalog mSort | wrapper owner | ⊤ ⇒ walls | ambient mParent supply | a wrong lend measures the wrong mVantage |
| mCompositeSort | per composite | the author holding the roles | n/a | identity | as any mSort |

## § 3-composition

### § 3.1-identity-of-a-key

identity(k), for k a key of mScheme S of mSort K: run S's `resolve()` from k's mVantage, and
each yielded mScheme's in turn, until a key of K's primary is in hand, each emission supplying
the mParent instance for the key it yields; then the primary key, scoped in identity(mParent)
through the arm the key took, recursively through each level's primary arm, until a mRoot arm,
the mRoute, or an unknown link. Each level carries the warrants of the arm the key took there.
A mCompositeSort's identity is its owner's function of its parts' identities. A cell's
identity is its bearer's plus which cell (§1.9). A key of an observer-dependent mSort carries
the O-instance as part of its mTopic. Nothing about identity consults the mVantage except to
know where to run `resolve()` calls and which ambient mParents to bind.

### § 3.2-compare (one chokepoint, four answers)

> Alias analysis's trichotomy: `same` is must-alias, `disjoint` is must-not-alias, `unknown`
> is may-alias, and `unspoken` is may-alias with the extra fact that no generator applies to
> the pair at all (no shared mParent at any level, no mCorrespondence, no finished
> definition). It becomes decidable only when one arrives, and it is named for silence so that
> it never reads as separation.

compare(x, y) ∈ {same, disjoint, unspoken, unknown}, consumers exactly as today
(`compare-consumer-map`: same → the fact is about this mCell; disjoint → sparing under
`--risk-faultless-skips`; unknown and unspoken → the safe bottoms). For two
mFullyQualifiedKeys, levels numbered from the leaf (level 0) upward through mParents:

- if either chain contains an unknown link, or the two chains terminate differently (one at
  the mRoute and one at a mRoot; two mRoots; two mRoutes across a transit): UNKNOWN. A mRoute
  or a second mRoot is a second mWorld, and nothing speaks across mWorlds — not the finished
  definition, whose sentence is within-mWorld (§2.5).
- otherwise the chains terminate together, and their levels are compared downward from the
  top. At the first level n whose keys differ inside a shared mParent (the level-(n+1) keys
  being SAME): DISJOINT iff the arm those two keys took carries :guarantees-unique-name and
  every arm from level n down to level 0 carries :sole-route; else UNKNOWN. Deeper levels are
  not consulted; separation is decided once.
- if no level differs down to the leaf: SAME iff every arm on the chain carries
  :guarantees-unique-referent, with the engine's own transit-free local mRoute claim standing
  in at the top (§1.10); else UNKNOWN.
- two keys of different mSorts share no primary (an mScheme belongs to one mSort), so their
  chains meet, if at all, only at a common ancestor level; that meeting is not a claim about
  the leaves (a package status file and a unit file share a filesystem). At the leaf the pair
  is UNSPOKEN, and sparing across it rides only the footprint side's finished definition
  (§2.5), bounded by mPlacements. Two mSchemes of ONE mSort yielding one primary key IS the
  same-referent generator across ways of naming — one thing in two term languages, joined on
  the mSort's own address — and it is the only one; Dorc equates keys and never merges mSorts,
  which stays a human act.
- partial measurement never widens: a mDerivation with an unmeasured or mRoute-terminated link
  yields at most what it would yield with the link measured.

For mDerivation sets: SAME is "or" across mDerivations (the chain; a mCorrespondence; a
provider-supplied identifier) and "and" within one chain. A warranted SAME and a warranted
DISJOINT on one pair is a contradiction: refuse both and attribute both authors; otherwise the
strongest warranted answer stands. SAME composes transitively; SAME then DISJOINT composes to
DISJOINT; DISJOINT then DISJOINT never chains. Universal meet over backing sets is unchanged
(`set-lifting-universal-meet`): sparing needs every footprint-by-backing pair disjoint.
Genuinely different mKey-Primaries for one mReferent (an NFS filehandle and the server's
inode; a machine-id and a cloud instance-id) are two mDerivations for one mTopic, reconciled
by coherence, never a second key inside one mParent.

### § 3.3-perishing (three mutator species, three invalidated facts)

- A ROUTING mutation (a mount, a symlink replacement, a rename, a user added, a hostname
  change, a write to `PATH` for command-word resolution) touches routing keys or a catalog.
  Every mResolution whose mTraversal includes a touched key — or whose catalog was touched at
  all, under the flat default — perishes; every mFullyQualifiedKey built on that mResolution
  reads unknown below the line; dependent SAME conclusions lose authority and dependent
  elisions demote to guards; dependent DISJOINT conclusions collide. The touched object itself
  is untouched (`a-path-is-not-a-referent`, `renaming-a-parent-moves-every-child-name`).
  Creation, deletion, and rename of a key are routing writes to its catalog entry (its
  existence mCell), so `userdel alice; useradd alice` perishes every mResolution of the old
  key (`a-recreated-name-is-a-new-referent`); a footprint that omits the entry is the ordinary
  at-most omission knife, now visibly covering routing keys.
- A STATE mutation writes mCells through mPlacements: ordinary kill-reach, unchanged. A first
  write can also change an mKey-Primary (`identity-tokens-perish-on-write-not-only-on-rename`),
  so a state mutation whose footprint touches a store that is some key's mParent perishes the
  mTokens scoped in it.
- A LIFECYCLE mutation (a reboot, a re-provision) disturbs a root-adjacent key (a boot, a
  tenure); every mKey-Primary scoped in it names a new mReferent afterward; mCells whose
  mFullyQualifiedKeys pass through it are new and unmeasured; mCells whose chains do not are
  untouched. What earlier designs declared as "keyed by Boot" or "invariant across Boot" is the
  shape of the chain, not a declaration.

In all three the engine withdraws authority; it never computes the successor identity.

### § 3.4-entry-and-lends

> A lend is dynamic binding: the wrapper rebinds a catalog parameter for the guest's dynamic
> extent, exactly `parameterize` or `fluid-let`. Nesting, shadowing, and innermost-wins all
> follow from that frame; nothing about it is Dorc-specific.

A wrapper's entry :lends catalog INSTANCES for the catalog mSorts it perturbs (a chroot lends
a mount namespace; `sudo -u` lends a user; `ip netns exec` lends a network namespace) and
nothing else; the lent instance becomes the ambient mParent for every secondary key whose
mScheme is looked up in that catalog mSort. Catalog mSorts not lent inherit the caller's
instance only after the wrapper's completion sentinel over catalog mSorts; before it they are
⊤. Leaf keys inherit transitively through their chains with no speech from anyone
(`not-every-transit-changes-the-referent`). A wrapper may additionally declare
mCorrespondences across the catalogs it :lends (§2.6). A lend may depend on the guest
(sudoers matches the guest command): the wrapper author declares the guest-insensitive
default and supplies a policy read that declines on departure from it. Entry forms, siting
vouches, the escalation dial, and measure-in-context remain `plans/27C`'s.

### § 3.5-committee-law-satisfied

Every positive step is one author's line: a `:yields` and its lookup warrants; an arm and its
warrants; a mPlacement set and its sentinel; an entailment and its finished record; a
mCorrespondence; a :observer-independence; a :lends. The engine only chains and meets. A
granting composite ("these two accounts are one") is entailed jointly by one mScheme's
`:yields` and the primary's arm, each author speaking about their own lookup
(`28M:rul-composite-meets-toward-guard-run`). A withholding composite (a mount perishing an
account's mResolution) names nobody and needs nobody's consent. Two mSchemes of one mSort that
resolve one string, in one mParent, from one mVantage, to two different primary keys, where
the primary's arm carries :guarantees-unique-referent: at most one is right, so both are
withheld and narrated (`28M:rul-conflict-between-totals-is-falsification`). Attribution: every
survival names the :sole-route and :guarantees-unique-name arms it rested on and the
mPlacement sets that bounded it; every SAME names the `resolve()` calls, the arms, and the
mCorrespondences; every perished conclusion names the footprint that perished it.

### § 3.6-scope (what this model leaves untouched)

The verdict, vouch, and guard tier; footprints, `:reaches`, finished definitions, and
`--risk-faultless-skips`; the four-answer chokepoint and consumer map; the universal meet;
measure-in-context, entry forms, `safe-across`; the read-set closure as the falsification net
for unmarked reads (`27C` §4(a)(B)); binds as the key-minting act; the mPlaceholder and the
standup `witness()`; the integrity plane; the committee law. The invariance line's store half
is measured away by §2.2 and its observer half lives as §2.7; the context slot is a mVantage
(an address and a witness key) and nothing else; there is no selector position, no selector
dialect, no aspect species, no authored region predicate, no engine-side name floor, no
engine table that generates `same`.

## § 4-forcing-functions

### § 4.1-slugged-gotchas-and-what-each-forces

Referenced by slug (`Research/GOTCHAS.md`); the sentence there is the forcing function, the
line here is the model element it forces.

- `a-path-is-not-a-referent` — mResolutions are facts with mTraversal backings; a remount is a
  routing mutation (§1.7, §3.3).
- `a-host-is-not-a-partition` — the mVantage is an address, never an identity input; identity
  is a chain through the file's mParent, and two chains terminating in two mRoutes read
  unknown (§1.10, §3.2).
- `same-name-different-referent-per-viewpoint` — a secondary key's mParent is ambient and
  lent; the chain differs per catalog instance (§2.1, §3.4).
- `not-every-transit-changes-the-referent` — leaf keys inherit through their chains; a transit
  that lends no catalog on a key's chain leaves it untouched (§3.4).
- `address-inequality-is-not-referent-inequality` — a secondary lookup licenses no
  disjointness across catalogs; :sole-route is a separate, dangerous, per-arm flag (§2.1,
  §2.2).
- `distinct-names-alias-within-a-kind` — :guarantees-unique-name is absent by default; the
  primary's key comes from the `:yields` chain (§1.5, §2.1).
- `containment-by-path-prefix-lies` — the path scheme carries no :sole-route; containment is
  mTraversal membership (§2.2, §2.8).
- `namespace-composition-is-not-concatenation` — catalogs are mKeys with identities, never
  strings composed by the engine (§1.6, §3.4).
- `renaming-a-parent-moves-every-child-name` — :hierarchical schemes decompose into
  mTraversals; perishing by mTraversal membership (§2.8, §3.3).
- `a-name-resolves-from-a-vantage` — the network mVantage is part of the address and of a
  hostname's mTraversal (§1.10, §2.8).
- `resolution-is-set-valued` — :guarantees-unique-referent is the lookup owner's opt-in; a
  resolver's owner never makes it, so equal hostnames yield unknown and the landing is
  measured (§1.5, §1.10).
- `identity-tokens-have-clone-horizons` — :rootness is a dangerous per-arm claim;
  :guarantees-unique-referent is absent by default on primaries (§1.6, §2.2).
- `a-name-is-not-a-target-over-time` — mPlaceholders, the standup `witness()`, integrity
  withhold; sameness of a target is continuity witnessed, never a spelling (§1.10).
- `a-store-is-not-one-inode` — mPlacement is many-valued and distinct from the mParent;
  :sole-route is not implied by :lives-in (§2.2, §2.4).
- `an-omitted-store-breaks-invariance` — mPlacement totality; the completion sentinel;
  omission is the silent channel; the set bounds the finished definition (§2.4, §2.5).
- `nonzero-status-is-not-speech` — every warrant is typed speech, never an exit status; the rc
  regimes of `311a` §6 stand (§1.11).
- `composite-identity-is-structure-not-a-bag` — exactly one mParent; roles are a
  mCompositeSort minted by the author who holds them (§2.3, §2.9).
- `the-subject-includes-the-observer` — :observer-dependence as the surviving half of the
  invariance line; two observers' mCells share mPlacement but not identity (§2.4, §2.7).
- `correspondence-is-known-only-to-the-transition-owner` — :corresponds as a transition-owner
  generator; mapped :lends are mCorrespondences (§2.6).
- `identity-tokens-perish-on-write-not-only-on-rename` — a state mutation on a store that is
  an mParent perishes the mTokens scoped in it (§3.3).
- `a-recreated-name-is-a-new-referent` — creation and deletion are routing writes to the
  catalog entry; a stable key over a recreate is a perished mResolution (§3.3).
- `recycled-keys-outrun-the-unwalled-span` — no lookup is a function by default; a catalog
  that reissues keys never earns :guarantees-unique-referent, and the `witness()` cannot see a
  recycled key (§1.5, §1.10).
- `a-cached-lookup-answers-for-the-past` — a secondary scheme's `resolve()` reads one catalog
  instance and declares which; a cache and the file it caches are two catalogs (§2.1).
- `one-state-reached-through-two-kinds` — two mSchemes of one mSort (`sysctl -w` and a write
  under `/proc/sys`) yield one primary key; the same-referent generator across ways of naming
  (§3.2).

### § 4.2-refuted-shapes-and-what-killed-each

Recorded as what-killed-it, so the shape is not re-walked.

- ONE DECLARED SPECIES (an mSort is a naming scheme; a second way of naming is a second
  mSort). Killed by: every referent-level attribute (mPlacements, entailment and the finished
  sentence, :observer-dependence, cells) is then written once per naming scheme by the same
  author with no seat to say the copies describe one thing; two finished sentences for one
  referent class land as withhold-and-narrate, never the fail-fast a contradiction deserves;
  and a stranger who knows only a new lookup must mint a whole referent class or leave it
  empty. Surviving form: two species, the mScheme hanging off the mSort (§1.2, §1.3).
- A DEFAULT mScheme PER mSort (a bare mSort in a bind meaning "its usual scheme"). Killed by:
  it makes an mSort valid where an mScheme belongs, so the two species are no longer
  semantically separate, and the bind stops saying what the author actually holds. Surviving
  form: a bind or mark always names an mScheme (§1.3).
- TWO ROLES PER KEY (a catalog it was looked up in AND a store it is identified in, as two
  edges). Killed by: at the primary they are one object (the primary's key is the store's own
  address), and a secondary key's identity is never its own but its yield's. Surviving form:
  one mParent per key, whose meaning follows the scheme (§1.6, §2.3).
- THE mParent DECLARED PER mScheme (one parent mSort for all of a primary's keys). Killed by:
  what a filesystem identifier is scoped in varies by the filesystem's TYPE (ext4 in a boot;
  NFS in a server; sshfs synthesising inodes per client), never by the child mSort or its
  schemes. Surviving form: the parent and the warrants ride the ARM, and a key's bytes select
  the arm (§1.6, §2.2). A subsort or type menu was proposed and retracted: the per-type
  variation is arms, and no new term is minted.
- DERIVING :sole-route FROM CHAIN SHAPE. Killed by nested pid namespaces: guest pid 1 and host
  pid 4821 are one mReferent and both chains resolve cleanly to one boot. Surviving form:
  :sole-route stays a declared, absent-by-default warrant on the arm (§2.2).
- AN ASPECT SPECIES (a selector position; an mAspectSort borrowing another mSort's lookup).
  Killed by: three independent pushes (one mParent per identity-bearing thing; per-aspect
  mPlacement; per-aspect :observer-dependence) already make an aspect an mSort in all but
  name, and the borrowed lookup is just an arm `:identified-in` the bearer. Surviving form: a
  cell is an mSort with a singleton key under its bearer (§1.9).
- IDENTITY AS A PER-KIND TABLE AGAINST AXES (the trichotomy invariant/keyed/⊤ per index-kind,
  `30W` §4; the filtered meet, `26Ob` §10b). Killed by: the mSort owner cannot know the axes;
  silence walls forever and a guess ("keyed by Host") plus a referent-transparent Host yields
  a wrong DISJOINT on a shared volume. The truth: a mCell's identity is a property of what its
  key denotes, not of the scheme the key is written in.
- THE CONTEXT AS PART OF THE FACT KEY. Killed by: a qualified key is an address of a question
  and asserts no partition; the dangerous inference lived in the meet's generators, not in
  the key's existence. Surviving form: the mVantage is an address and a witness key (§1.10).
- STORE SETS, unioned and compared as bags. Killed by base-and-overlay: same set, different
  roles, different answers; and by the reflexivity defect of pairwise set equality. Surviving
  form: one mParent, mCompositeSorts for roles (§2.9).
- "STORED-IN" AS ONE RELATION conflating routing and containment. Killed by hardlinks, bind
  mounts, NFS: catalog-disjointness is not referent-disjointness. Surviving form: `:yields`
  with its catalog, the primary's arms with :sole-route, and mPlacement as three relations
  (§2.1, §2.2, §2.4).
- TERMINAL TOKENS (`Measured(File, fsid:inode)`). Killed by NFS and by the parent question: an
  inode is a key in a filesystem, a filesystem identifier is a key in whatever minted it; the
  File owner should never learn NFS — and under this model does not, because the parent
  mSort's primary classifies one level up (§2.2). Surviving form: :rootness as an explicit
  dangerous claim; every mToken scoped.
- ONE GRADE ON A LOOKUP (transparent/identifying). Killed by clones: equal machine-ids on two
  machines threaten SAME, which a disjointness grade cannot protect. Surviving form: two
  independent warrants per lookup (§1.5).
- OBJECT IDENTITY CARRIES EVERY OBSERVATION ("one cell, one fact, whichever probe read it").
  Killed by `test -w` under two users on one file. Surviving form: :observer-dependence per
  mSort, absent means dependent (§2.7).
- AUTHORED REGION PREDICATES (`kind__disjoint`; a subtree selector). Killed by the
  symlink-retarget case: separation of two objects says nothing about whether changing one
  retargets a key for the other; and by the granule observation that the natural description
  unit is the atomically-disturbable one. Surviving form: :hierarchical mSchemes and
  mTraversals; containment as mTraversal membership (§2.8).
- IDENTITY BINDINGS BACKED BY THEIR TARGET OBJECT. Killed by the same symlink case. Surviving
  form: mResolution backed by mTraversal (§1.7).
- UNION TOTALITY WITH NO AUTHOR (an mSort-level ∪ mSite-level sentinel). Killed by asking who
  closed the union. Surviving form: the owner declares and closes; mSites fill catalogs
  (§2.1, §2.4).
- SHARED ANCESTORS AS COLLISIONS. Killed by the package status file and the unit file sharing
  a filesystem. Surviving form: ancestors are mParents; `compare()` at the divergence (§3.2).
- ROUTE-VERSUS-ROOT AND ROOTS-DIFFER AS UNSPOKEN. Killed by the finished definition spending
  across mWorlds: a chain with fewer measured links compared more decisively than the same
  chain complete, and the Package author became the wrong name in the why chain. Surviving
  form: both read UNKNOWN; partial measurement never widens (§3.2).
- THE DISCLOSED-WEAK NAME FLOOR (within one mSort, unequal names answer disjoint). Killed by
  the human's default-safe lean and by its own per-mSort carves. Surviving form:
  :guarantees-unique-name is typed, never assumed (§1.5).
- A DEFINITIONAL EQUAL-KEYS DEFAULT ("equal keys in one catalog across an unwalled span reach
  one mReferent, because that is what a scheme is"). Killed by: it feeds only the dangerous
  consumer (kill-reach already collides on unknown); recreation under a stable name, pid
  reuse, round-robin and cached lookups, and `:latest` tags are ordinary ops; and the claim's
  owner differs at every level. Surviving form: the engine vouches only the transit-free local
  mRoute; every lookup's functionality is its owner's opt-in (§1.5, §1.10).
- MEASUREMENT MAKES DECLARATIONS REDUNDANT. Killed by: a `resolve()` establishes a mToken, not
  its mParent, mTopic, warrant, applicability, or sufficiency. Surviving framing: measurement
  relocates speech to questions the owner can answer.
- RENAMING RESOLVE INTO IDENTITY FIXES THE FALLTHROUGH IDIOM. Killed by the observation that a
  renamed member can still echo a fallback and exit 0; the repair is the completion-only rc
  regime plus a taught decline, not the name.
- THE DISAGREEMENT CANARY AS A SAFETY ARGUMENT. Killed by noting the second probe exists only
  because nothing was elided on the first; absence of disagreement is not evidence, and a
  wrong bind is a wrong SAME that lands, attributed.
- CONVERGENCE OF TWO SYSTEMS FROM ONE WINDOW AS EVIDENCE. Killed by the human's first bullet.
  Surviving discipline: clean-context adversarial review before any design-of-record.
