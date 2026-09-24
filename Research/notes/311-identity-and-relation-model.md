# 311 — Identity and relation: the model

> AI-authored PREPARATORY MODEL (Fable, the `r31-prep-design-duck` sittings; the human present
> and adjudicating). Notes-tier, ahistorical; the ledgers and reviews around it cite it as
> `311j`, the id it carried until 2026-09-19. Nothing here is ruled; grades on the conductor's
> claims are +SURE / ~SUSPECT / -GUESS / --WONDER, and **[LEAN]** marks a human lean paraphrased
> from chat, never a ruling. Authority: root docs, `spike/CLAUDE.md`, and the welds outrank this;
> where this document disagrees with `plans/30W`, `notes/272`, `plans/30T` §6, `plans/30U` §7,
> `notes/26Ob` §10, or `plans/27C` §4, it is a deliberate proposal to re-litigate them.
>
> Purpose: the abstract objects, relations, and laws of identity across mutually-unknowing
> authors, stated fully enough that the concretization step (names, spellings, user-facing
> usage — the `312` series) can be built on it without re-modeling. §1–§3 are the MODEL; §4 is
> the litmus and the forcing-function inventory.
>
> Scope discipline: abstract objects and relations only. No syntax, no strawman sh, no UX, no
> gradual-enhancement ladder, no implementation.
>
> Naming discipline: every model object is written mFixedTerm, and the bare word never stands
> in for it; every relation, attribute, or warrant is written `:fixed-term`, always preceded by
> a space; every abstract operation is written `op()`; every concrete authored member keeps the
> `__name()` form; a derived view of a species is written species-hyphen-gloss (mKey-Primary,
> mParent-Catalog, mParent-Store), never a declared species; "cell" stays untagged and means a
> singleton mSort under its mParent (§1.9); an in-Dorc mSort or mScheme is
> always written with its prefix (`sm.File`, `sm.Path`). "Lookup" (the relation between an
> mKey and what it reaches) stays untagged; an mKey's bytes are its mValue and an mReferent's
> condition is its mState (§1.1). A blockquote under a heading names a false friend, and
> nothing else.

## § 0-one-screen

Dorc removes a line only on connected claims that the line is unnecessary: a measurement
claimed before anything runs, plus an author's vouch. The claim can be destroyed by an earlier
line that really runs, so the engine must decide whether the piece of the world an earlier line
touches is the same piece a later line's licence depends on. That is identity, and it has two
consumers with opposite failure directions: SAME lets one fact stand for another and
under-executes when wrong; DISJOINT lets a licence survive a write and under-executes when
wrong. UNKNOWN is safe for both. The engine knows only syntax, authored speech, and what
authored probes returned; it never decodes an mKey and never holds an mReferent. No single
author knows the whole path from a tool's argument to a measurable mReferent, and viewpoint
transitions authored by yet other people change which mReferent an mKey reaches mid-book.

The model has two declared species, never substitutable: the mScheme (a way of writing a
thing down; what every bind and mark names; owns a `resolve()`) and the mSort (one owner's
declared carrier, under which that owner's speech coheres; owns nothing a bind can name). At
most one mScheme per mSort is primary; every mKey has one mParent; identity is the mFullyQualifiedKey,
the chain of mParents through primary mSchemes; `compare()` decides at the first divergence;
every positive answer rests on a typed, absent-by-default warrant one owner declared for one
shape of mKey-Primary, and silence is unknown.

## § 1-model-objects

### § 1.1-referent

An mReferent is a persisting piece of the world that has state: an inode, a database row, a
package record, a kernel parameter, a running process, a machine, a mount table. It has the
mSort of the mKey that reaches it (two mSorts over one piece of the world is the strangers case
of §1.2) and one or more mKeys. It may have parts, and every part is an ordinary mReferent of an
ordinary mSort identified in it (§1.9); no other term names a part. The engine never holds an
mReferent. It reaches mReferents only through mKeys, mTokens, and mDerivations, and every
identity question is ultimately "do these two mKeys reach one mReferent"; before a lookup binds
an mKey, that mKey names a may-set of mReferents (§1.5). An mReferent survives changes to its
mState; it does not survive destruction and recreation under its old mKey
(`a-recreated-name-is-a-new-referent`), nor a lifecycle write to what it is scoped in (§3.3).
An mReferent is not an mKey (an mKey names it), not an mState (it has one), not an mValue (a
read yields one from it), not a mTopic (§1.8, an mKey read with its observer instance), and not
the set an mKey given whole denotes (§2.8: the container is the mReferent, the set is a set).
mReferents are what GOTCHAS are about.

An mState is the condition of one mReferent at one instant: what a write changes and what a
read observes. The engine never holds an mState and no mKey names one; an mState is known only
through a read, which yields an mValue. Two mReferents may have equal mStates and stay two; one
mReferent's mState changes and it stays one.

An mValue is bytes a shell holds or will hold, with an exit status where one is produced: an
mKey's mValue (bytes that name an mReferent, bound at a bind), a captured mValue (bytes a read
copied out of an mState, graded by provenance, `notes/275`), a verdict's exit status. An mValue
is what a read yields from an mState, never the mState: two reads of one mReferent at two
instants may yield two mValues.

### § 1.2-sort

> Many-sorted logic's carrier. Never a PLT kind; never "the kind of thing".

An mSort is one owner's declared vocabulary: a carrier in the logician's sense, a domain of
discourse someone chose to speak in, with reverse-DNS naming, no registry, and
owner-adjudication as the social contract. It is not a category of the world. The engine never
knows what an mSort denotes and never assumes two mSorts denote disjoint mReferents: what an
mSort fixes is only what its owner declares under it — which mScheme, if any, is its primary
mScheme (§2.2), its may-read set (§2.4), its may-write entailment and finished definition (§2.5), its
:observer-dependence (§2.7), and its cells (§1.9) — and the owner speaks only about the
mSort's relations to its immediate neighbours. Several mSchemes into one mSort is the
cooperative case: one owner admitting several ways of writing down what they describe. Several
mSorts over one world-thing is the ordinary strangers case (two vendors describing one tool;
two vocabularies reaching one cell under `/proc/sys`): undetectable to the engine,
KNOWN_UNSPOKEN at the chokepoint (§3.2), and merged only by a human act, one mSort's mSchemes
made to yield into the other's. An mSort has no mKeys and no `resolve()`, is never valid where an
mScheme is, and is named only when something is declared about it as a whole; the floor of
§1.3 lets an mScheme precede its mSort's name.

### § 1.3-scheme

> A term language over a carrier. Never itself an mSort.

A way of writing down which mReferent is meant, with one accountable owner. An mScheme fixes:
its `resolve()`; whether it is `:primary-of` an mSort, with its declarations per matched
shape (§2.2); what it `:yields`, per matched shape, each an mKey of another mScheme of any
mSort (§2.1); where its mKeys are looked up, when it is a secondary mScheme (§2.1); its lookup
warrants (§1.5); and whether it is :hierarchical (§2.8). An mSort has at most one primary
mScheme; where it has one, that mScheme yields into the mSort for at least one shape, and
nothing else constrains where an mScheme yields. An mSort may have no mScheme at all: its mKeys
are then singletons under mParents (§1.9). The floor: an mScheme that declares neither
`:primary-of` nor `:yields` is the primary mScheme of an mSort nobody has named, with an
identity `resolve()`, no warrants, and the mRoute as its only mParent; that mSort acquires a
name the first time its owner declares something about it as a whole. There is no default
mScheme: a bind or a mark always names an mScheme, and a mark of the form `parent-key@sm.Sort`
names the mParent's mScheme and the singleton's mSort (§1.9).

### § 1.4-key

> RDBMS primary key and natural key, with their culture: the natural key is user-typed, may
> alias, is never identity; the primary key is what the store answers with.

An mKey is a plan-time object: an mValue, its mScheme, and its mParent (§1.6), modeling what a
runtime string will denote. It is minted at a bind, or at an emission point a `resolve()`
declares (§2.1), and before any lookup runs. The mScheme is always declared. The mValue is a
literal, or a mPlaceholder for a captured mValue; the mParent is an instance one of the seats
of §1.6 supplies, or a mPlaceholder. Every lookup is a measurement that binds mPlaceholders
and chooses among the structures of the mFullyQualifiedKey the mSchemes declare, never outside
them; what measurement decides late is which declared shape an mValue matches (§2.1, §2.2), so
which mSort an mKey reaches, and which mParent mSort and warrants apply, is known only once
bytes arrive. An mValue with no mScheme is nothing; an
mScheme with no supplyable mParent instance leaves the mFullyQualifiedKey unknown from that
level. Derived views: mKey-Natural, an mKey of a secondary mScheme (what tool authors and
books write); mKey-Primary, an mKey of the primary mScheme (meaningful only relative to its
mParent-Store; where the dangerous warrants can honestly sit). They coincide when an mSort's only
mScheme is its primary mScheme.

### § 1.5-token-and-the-two-warrants

> OWL's inverse-functional and functional properties, spelled out by direction.

A mToken is an mKey-Primary's mValue: bytes a `resolve()` returned, compared for equality only,
never decoded (`inv-referent-agnostic`), always scoped in an mParent. Any lookup, a secondary
mScheme's or a primary mScheme's, may carry per matched shape two independent, separately
declared, absent-by-default warrants:

- :guarantees-unique-referent — within one mParent, equal mKeys reach one mReferent; the
  lookup is a function. Licenses SAME from equality. Fails for round-robin lookups
  (`resolution-is-set-valued`), recycled mKeys (`recycled-keys-outrun-the-unwalled-span`),
  and cloned identifiers presented as mRoots (`identity-tokens-have-clone-horizons`).
- :guarantees-unique-name — within one mParent, one mReferent has one mKey. Licenses DISJOINT
  from inequality. Fails for symlinks and hardlinks, package `provides`
  (`distinct-names-alias-within-a-kind`), route-qualified handles.

Declared per matched shape, by the lookup's owner, holding for every mKey of that shape inside
any one mParent while that mKey's mResolution or mToken stands (§3.3). A matched shape is a
control-flow path of the owner's body: a warrant is constructed when that body is evaluated for
the mKey in hand, may rest on what the path measured, and may be declined by any path the
dialect admits; a path that does not reach a warrant has not given it. The one lookup the engine
vouches for itself is the transit-free local mRoute (§1.10); across a transit nothing is
claimed, only measured and witnessed.

### § 1.6-parent

Every mKey has exactly one mParent, the mKey it was resolved inside. What the edge carries
follows the mScheme:

- through a secondary mScheme the mParent is the mParent-Catalog, the thing the mKey was
  looked up in (a mount namespace for a path; a passwd database for a login name; a process
  table for a pid), and the edge carries ROUTING: which instance, and perishing (§3.3). Never
  identity.
- through the primary mScheme the mParent is the mParent-Store, the mKey relative to which
  alone the mKey-Primary means anything (a DNS zone for a record's owner name; a user namespace
  for a uid; a dpkg database for a canonical package name), and the edge carries IDENTITY: it
  is what `compare()` walks
  (§3.2). At the primary mScheme, mParent-Catalog and mParent-Store are one mKey.

Through the primary mScheme the mParent's mSort (`:identified-in`) and the warrants (§1.5;
:rootness, §2.2) are declared by the primary mScheme's owner per matched shape of
the mKey's mValue, a function of the mKey's own bytes, so an emitter (a secondary mScheme,
possibly a stranger's) can be wrong only about what it supplied: its lookup, and the mParent
instance where it is the seat that supplied it. The mParent instance is an mValue supplied by
exactly one of three seats, each naming it as an mKey of one of the mParent's mSort's
mSchemes: the bind that minted the mKey (§1.4), the lookup that yielded it (§2.1), or the
primary mScheme's declaration for the matched shape (§2.2); two seats that disagree are a
contradiction, refused and attributed to both. A shape with no `:identified-in` is scoped in
the mRoute (§1.10). A mRoot is a shape declared
:rootness: its mKeys need no mParent, which claims they are globally comparable (the DNS
mRoot; a cloud instance-id whose issuer never repeats one). Nothing is a mRoot by default;
`identity-tokens-have-clone-horizons` is the standing witness.

### § 1.7-resolution-and-traversal

A mResolution is the fact that mKey N of mScheme S, resolved inside mParent P, at program point
p, reaches mReferent R: a FACT with a BACKING, the mTraversal — the ordered chain of routing
mKeys the lookup crossed (each directory entry and symlink for a path; the resolver
configuration and mVantage for a hostname; the unit table for a service name). A
:hierarchical mScheme's structure fixes how an mKey decomposes into a mTraversal (§2.8). A
mResolution perishes under ordinary effective-mWorld reach from any mutator whose Writeset
touches a mTraversal member. Its target object is not its backing: an mKey can stop reaching
an object without the object changing (`renaming-a-parent-moves-every-child-name`,
`a-path-is-not-a-referent`), and an object can change without its mKey changing.

### § 1.8-fully-qualified-key-and-derivations

A mFullyQualifiedKey is the recursive identity of an mKey: its mKey-Primary scoped in its
mParent, whose identity is itself a mFullyQualifiedKey through the mParent's own primary
mScheme, terminating at a mRoot, the mRoute (§1.10), or unknown. It is one mDerivation of
identity. A mTopic — what a claim is about: the mKey read, plus the observer instance when
that mKey's mSort is observer-dependent (§2.7) — may carry several mDerivations with different
generators: its mFullyQualifiedKey, a provider-supplied identifier, a mCorrespondence from a
transition owner (§2.6). mDerivations combine by coherence (§3.2), never by priority.
(`witness()` is reserved for the standup re-measurement of §1.10.)

### § 1.9-cell

A cell is a singleton mSort identified in its mParent: an mSort whose owner declares
`:identified-in` the mParent's mSort (§2.2) and which has, under any one mParent instance,
exactly one mKey, written `parent-key@sm.Sort` with the mSort's name in full reverse-DNS. That
mKey is minted at the mark that names it (§1.4, §1.6): the singleton mSort has no mScheme of
its own, and the mScheme left of `@` is the mParent's. A cell's identity is its mParent's plus
its mSort (§3.1). Its mReferent's mState may be held elsewhere than its mParent, and may be
diffuse: the cell's may-read set (§2.4) says where that mState is held, and the freshness of a
fact about the cell follows that set, never the mParent (`enabled`, held in a symlink in a
filesystem, survives a reboot; `active`, held in the service manager's memory in the boot, does
not; both under one mParent). Two cells of one mParent are two mSorts with two may-read sets
and two :observer-dependences, and §3.2 decides between them as between any two mSorts. A
Writeset entry naming the mParent whole covers its cells (§2.8, §3.2). The marked line that
answers a cell is a read of the cell's mKey, and the fact's identity is the mTopic (§1.8).
There is no aspect species.

### § 1.10-vantage-route-placeholder-witness

A mVantage is the ADDRESS a probe reached an mReferent from: the mEntryChain, a finite map from
mParent-Catalog mSorts to the instances wrappers lent (§3.4). It is not part of any mKey's
identity. It says where a `resolve()` executes; it supplies the ambient mParent for every mKey
of a secondary mScheme looked up in a lent mParent-Catalog mSort; and for a shape with no
`:identified-in` it is the mRoute, the last-resort mParent. For transit-free local execution
the engine itself vouches the mRoute, and the ambient and inherited mParent instances within
one unwalled span: each is resolved once per mEntryChain and shared, one mPlaceholder. Two
same-spelled leaf mKeys are two mPlaceholders, SAME only by warrant (§3.2). Across a transit
the mRoute is never vouched; across mVantages it is unknown.
mFullyQualifiedKeys whose mTokens are not yet measured are mPlaceholders keyed by (mKey,
ambient mParents, mEntryChain); the probe standup binds them; the apply standup re-reads them
through the same entry and `compare()`s (the `witness()`); mismatch is integrity, never a
verdict input (`26Ob:cor-standup-witness-licenses-bare-line-elision`). The `witness()` cannot
see a recycled mKey (a reissued pid or inode); that stays on the outside-churn horizon.

### § 1.11-site-and-claim-species

A mSite is within a book line, with an argv, an mEntryChain, and a program point. Speech at or
about a mSite, each with one author: a VERDICT FACT (the measured answer to a read of a cell,
its Readset the body's marked reads, `30T:req-verdict-marks-every-read-cell`, vouched by the
tool-oracle author); a WRITESET CLAIM (an at-most may-write set per matched shape with its
completion witness, `30U`, by the tool-oracle author or the filesystem binder); a MAY-WRITE
ENTAILMENT (§2.5, by the mSort owner); a mCorrespondence (§2.6, by the transition owner); the
per-mScheme declarations (§2.1, §2.2, §2.8); the per-mSort declarations (§2.4, §2.7, §1.9); the
wrapper's :lends and their sentinel (§3.4).

## § 2-model-relations

Each relation states: arity, who declares it, its default, which consumer reads it, and which
warrant makes it dangerous. The human's naming law applies throughout (**[LEAN]**: dangerous
operations rare and long-named; safe ones short) but no spellings are proposed here.

### § 2.1-yields (a secondary mScheme, into the primary mScheme; the mParent-Catalog)

mScheme S `:yields` mScheme T, per matched shape, T of any mSort: S's `resolve()`, run in the
mVantage, maps an mKey of S to an mKey of T, and may supply that mKey's mParent instance
(§1.6). T is a primary mScheme or a secondary mScheme that in turn yields one; the chain
always terminates at a primary mScheme. Declared by S's owner, per matched shape. Where S's own mKeys
are looked up — S's mParent-Catalog — is an instance supplied by one seat, as a store is
(§1.6): the bind that minted the mKey, S's owner's declaration, or the mEntryChain's instance
for that mSort (§1.10), each naming it as an mKey of one of that mSort's mSchemes; an unknown
input makes the instance unknown.
Consumer: mResolution (§1.7) and the mFullyQualifiedKey (§3.1). Default: neither `:yields`
nor `:primary-of` is the floor of §1.3. S's lookup warrants (§1.5) govern what equality and
inequality of S's mKeys license before the primary mScheme is reached, and never license
across mParent-Catalogs (`a-host-is-not-a-partition`,
`address-inequality-is-not-referent-inequality`). A `resolve()` declines on mReferents its
mSort does not describe (a path reaching a socket, under an mScheme into files): the
mechanical net against lazy borrowing.

### § 2.2-primary-of (:identified-in, :aliases-nothing-else, :rootness)

> ER's identifying relationship. Identifying is not containing.

mScheme P is `:primary-of` mSort K, at most once per mSort, and a second name is a second mScheme: P's
mKeys mean something only relative to K's mParent-Store, and P's `resolve()` is the identity
on the mKey. P's owner declares, per matched shape of the mKey's mValue, `:identified-in` mSort M
(the mParent's mSort for mKeys of that shape) and, absent by default:

- :guarantees-unique-referent and :guarantees-unique-name (§1.5), governing what mToken
  equality and inequality license at this level of a mFullyQualifiedKey;
- :rootness — the shape declares no mParent and thereby claims global comparability: equivalent
  to :guarantees-unique-referent over the whole world; fails for cloned identifiers. A mToken
  duplicable across instances of its would-be mParent must be scoped in something smaller or
  left un-warranted.

The mParent's TYPE varies per shape (an ext4 filesystem in the mRoute; an NFS filesystem in a
host; a tmpfs in a boot), so the child mSort's owner never learns the mParent's types: the
mParent mSort's primary mScheme classifies, one level up, each owner speaking one level. An
mValue matching no declared shape reads unknown from this level. A grade governs every consumer of the answer it grades, corroboration and contradiction
included: a lookup without :guarantees-unique-name cannot contradict anything by returning
two different mTokens.

A store is :aliases-nothing-else when nothing identified in it is, by the store's own
construction, also identified in another store: it gives its own mKeys to no other store's
mReferents (a DNS zone for its records; a dpkg database for its packages; a network namespace
for its `net/*` knobs; a disk filesystem for its inodes), and never where it does (a client
NFS mount; an NSS view of LDAP; a chroot; a nested pid namespace; an overlay; a front over
another daemon; a relabelling of part of its mParent's state). It is self-knowledge: whether
another store aliases this one is not claimed, since nobody can know it, and §3.2 asks the
warrant of every store on both legs, so that an aliasing store blocks separation by its own
silence. Whoever describes the store holds it. State spanning several files
(`a-store-is-not-one-inode`) is a may-read matter (§2.4); a hardlink is a
:guarantees-unique-name failure on the path mScheme, dissolved by the path mScheme yielding
the inode.

### § 2.3-parent

One per mKey (§1.6): never plural, never a species of its own. Its mSort is the primary
mScheme's declaration for the matched shape; its instance is whichever seat supplied it; the
far end is an ordinary mKey. A mVantage supplies instances and is never an mParent.

### § 2.4-may-read (the Readset)

> Separation logic's footprint, at the read side: the mReferents an answer may depend on.

K `:may-read` these mKeys: K's mState is affected by writes to them. Many-valued: an edge
type from an mSort (a template the mSite or environment fills) to mKeys of other mSorts.
Declared by K's owner, with a completion sentinel closing the set. Every mSort in a
mFullyQualifiedKey may declare may-read entries, not only leaves (a loop-backed filesystem's
state lives in a file of the outer filesystem; `dd` over the image rewrites every inner fact).
A fact's Readset is the marked reads of the body that answered it together with the may-read
entries declared by every member of the mKey's mFullyQualifiedKey; it is closed only when every
declared set is closed. An mKey's mParent instance is no may-read entry and needs no
declaration: §3.2's walk collides a write at or above it. A may-read entry naming a store says
more: every write to an mKey relative to that store may change K, nobody having said
otherwise. Consumer: collision, and the write-path question of §2.5. A Writeset entry not
DISJOINT from a may-read entry collides with K's cells and with every fact identified beneath
K; an omitted entry is a silent channel (`an-omitted-store-breaks-invariance`,
`a-store-is-not-one-inode`) and licenses nothing positive. May-read is distinct from mParent:
the mParent is one and answers identity; may-read entries are many and answer interference.
Two cells with different mParents can share a may-read entry and so collide without being the
same (`the-subject-includes-the-observer`: two observers' writability cells share the file's
mode). Under §2.5 a closed may-read set is knife-tier: it is one of the two closures every
sparing rests on.

### § 2.5-may-write (the Writeset, its entailment, and the finished definition)

A line's Writeset is the at-most set of mKeys it may write: the may-write entries its verb's
author declared per matched shape (`plans/30U`'s footprint), closed by the completion record,
widened by the may-write entailment that mSort owners declare. The entailment, as `plans/30U`:
writing an mKey of K entails may-write of these mKeys of other mSorts; arm-incremental,
collide-adding; the reached completion record finishes the definition: it witnesses that the
write set, after entailment, is complete. Declared by K's owner. About effects, not identity:
it carries a package's postinst enabling its unit, a restart killing a main process, every
cross-mSort consequence no mFullyQualifiedKey expresses, and it generates no DISJOINT
("nothing else" is no other thing, never no other mKey for the thing written). An elision is
spared past a write only when two questions are answered, in order, for every pair of a
Writeset entry and the mKey a fact reads, of one mSort or of two. First, `compare()` answers
DISJOINT (§3.2). Second, no write path joins them: the Writeset's definition is finished; the
may-read sets declared by every member of the fact's mFullyQualifiedKey are closed; and every
Writeset entry `compare()`s DISJOINT with every one of their entries. It spares narrowly and
collides widely: whatever is not DISJOINT collides, and so does an undeclared may-read entry.
The finished definition is a within-mWorld sentence; it never speaks across mRoutes or mRoots
(§3.2).

### § 2.6-corresponds (declared sameness across a transition)

> A scoped `sameAs`; `23M` rejected the unscoped one. Not "corresponds to" loosely: a part, a
> view, or a correlate of a thing is not it.

mKey X inside mParent A denotes the same mReferent as mKey Y inside mParent B. Declared by the
owner of the TRANSITION between A and B, which is neither mKey's mScheme owner: the container
manager knows guest pid 1 is host pid 4821
(`correspondence-is-known-only-to-the-transition-owner`); `sudo -u alice` knows inner "me" is
outer "alice"; a mount line's oracle knows mKeys under the mountpoint are mKeys under the
export on the named server, from this mVantage. Consumer: a SAME mDerivation (§1.8),
vouch-tier, attributed to the transition author. Default: absent, so mKeys across a
transition `compare()` unknown unless a mFullyQualifiedKey binds mTokens on both sides.
Danger: a wrong mCorrespondence is a wrong SAME. This generalizes the mapped lend of `273` and
is the model's only declared sameness generator besides mToken equality.

### § 2.7-observer-dependence

The mValues that reads of K's cells yield depend on which mKey of mSort O the read was taken
under. Declared by K's owner per mSort as its complement, :observer-independence of O. Default:
a cell measured
under a lent mKey of O is assumed to depend on it, so its fact is about (mReferent,
O-instance) and never stands for the same mReferent under another O-instance. Consumer: the
SAME consumer, as a qualifier on the claim's mTopic. This is the surviving half of the old
invariance line (`271:rul-invariance-speech-act`): its store half is measured away by §2.2,
its observer half cannot be measured by any `resolve()` because the object is the same and
the answer differs (`the-subject-includes-the-observer`), and it must remain speech.
Measurement in the denoted context (`plans/27C`) stays the default lane.

### § 2.8-hierarchical (mTraversal structure; on the mScheme)

An mScheme is :hierarchical when its mKeys decompose into an ordered chain of its own mKeys,
each a routing mKey the lookup crosses: a path into directory entries, a hostname into
resolver steps from a mVantage, a dotted unit name into its instance table. Otherwise FLAT (an
inode number; a uid). A :hierarchical mScheme may contain INDEXICAL components whose
mResolution depends on the observing process (`/proc/self`); only the mScheme owner can say
which, and an undeclared indexical component reads unknown. The engine derives the
decomposition from mKey SYNTAX, which is language, plus this declaration (`30T` §5's
syntax-versus-semantics line). Consumer: mResolution backings (§1.7), hence perishing (§3.3).
Default: flat, so the mTraversal is the mParent-Catalog as a whole and any touch on it
perishes every mResolution through it — the coarse, safe floor. This replaces authored region
predicates: containment is membership in a mTraversal, and a mutator that touches a directory
needs to know nothing about files (`renaming-a-parent-moves-every-child-name`,
`namespace-composition-is-not-concatenation`). A routing mKey named whole, in a Writeset or as
a may-read entry, stands for whatever its mScheme reaches beneath it: in §2.5's test it reads
UNKNOWN against every mKey that mScheme can yield in the same mParent-Catalog instance, whatever
§3.2 answers of the two as siblings (unequal mTokens say only that the thing is not the routing
mKey itself).

### § 2.9-composite-sorts (roles)

A mTopic whose mReferent's mState depends on several inputs IN ROLES (a base and an overlay; a primary and
its replica set) is an mKey of a mCompositeSort minted by the author who knows the roles,
normally the tool author, and that mSort's identity is its owner's function of its named
parts. Plurality of inputs is an mSort with structure, never a set of mParents
(`composite-identity-is-structure-not-a-bag`). The may-read set of a mCompositeSort is the
union of its parts' may-read sets.

### § 2.10-relation-table

| relation | arity | declared by | default | consumer | danger |
|---|---|---|---|---|---|
| `:primary-of` (`:identified-in` per matched shape) | at most one mScheme per mSort | the mSort's owner, on the primary mScheme | none — the floor of §1.3 supplies an unwarranted identity primary mScheme | identity (§3.1) | :guarantees-unique-referent · :guarantees-unique-name · :rootness, per matched shape |
| `:yields` (the mParent-Catalog, supplied by one seat; the yielded mKey's mParent instance where the yield supplies it) | per matched shape of a secondary mScheme, into any mSort | the mScheme's owner | none ⇒ the mScheme is a floor primary mScheme | mResolution; the mFullyQualifiedKey | the lookup warrants; a wrong yield or a wrong supplied instance is a wrong SAME or DISJOINT, attributed to the yield |
| `:aliases-nothing-else` | per store | the store's describer | absent ⇒ nothing inside that store separates from anything outside it | DISJOINT (§3.2) | an aliasing store declared :aliases-nothing-else is a wrong DISJOINT |
| `:parent` | one per mKey | derived (§2.3) | n/a | routing (secondary mScheme) · identity (primary mScheme) | none of its own |
| `:may-read` | many per mSort, sentinel | mSort owner | ⊤ ⇒ collides with everything | collision; the write-path question (§2.5) | none positive; omission is the silent channel; the closed set is one of sparing's two closures |
| `:may-write` (the verb's at-most set; the mSort's entailment) + finished | per matched shape of the verb; many, per matched shape, on the mSort | the verb's author; the mSort owner | unfinished ⇒ collide | the write-path question (§2.5), within one mWorld; never a generator of DISJOINT | the premature finished record |
| `:corresponds` | per transition pair | transition owner | unknown | SAME mDerivation | a wrong mCorrespondence |
| `:observer-independence` | per (mSort, O) | mSort owner | dependent ⇒ no carry | SAME qualifier | a false independence |
| `:hierarchical` | per mScheme | mScheme owner | flat ⇒ whole-mParent-Catalog mTraversal | perishing | none (finer buys sparing, coarse is safe) |
| `:lends` (+ sentinel) | per wrapper, per mParent-Catalog mSort | wrapper owner | ⊤ ⇒ walls | ambient mParent supply | a wrong lend measures the wrong mVantage; the sentinel is an at-most claim over every mParent-Catalog mSort |
| mCompositeSort | per composite | the author holding the roles | n/a | identity | as any mSort |

## § 3-composition

### § 3.1-identity-of-a-key

identity(k), for k an mKey of mScheme S: run S's `resolve()` from k's mVantage, and each
yielded mScheme's in turn, until an mKey of a primary mScheme is in hand, each emission
supplying the mParent instance for the mKey it yields; then the mKey-Primary scoped in
identity(mParent), recursively through each level's primary mScheme, until a mRoot, the
mRoute, or an unknown link. Each level carries the warrants declared for the shape its mKey
matched. A mCompositeSort's identity is its owner's function of its parts'
identities; a cell's is its mParent's plus its mSort (§1.9); an mKey of an
observer-dependent mSort carries the O-instance in its mTopic. The mVantage is consulted only
to know where to run `resolve()` calls and which ambient mParents to bind.

### § 3.2-compare (one chokepoint, four answers)

> Alias analysis's may/must trichotomy, plus `known-unspoken` for "no generator applies".

compare(x, y) ∈ {same, disjoint, known-unspoken, unknown}, consumers as today
(`compare-consumer-map`: same → the fact is about this mKey; disjoint → sparing under
`--risk-faultless-skips`; unknown and known-unspoken → the safe bottoms). For two
mFullyQualifiedKeys, levels numbered from the leaf (level 0) upward through mParents:

- if either mFullyQualifiedKey contains an unknown link, or one terminates at a mRoute the
  other does not share (one at the mRoute and one at a mRoot; two mRoutes across a transit):
  UNKNOWN. A mRoot shape is one mWorld, SAME by :rootness, where two mKeys of that shape meet
  and compare as siblings; mRoots of two shapes are two mWorlds, a mRoute is another, and
  nothing speaks across mWorlds — not the finished definition, whose sentence is
  within-mWorld (§2.5).
- otherwise walk downward from the top to the deepest level at which the two chains are SAME
  by the rule below; call it A. If either mKey is A itself, the pair reads UNKNOWN: a write to
  a container collides with everything inside it. Otherwise call each mKey's top the child of
  A on its side (the mKey itself when its mParent is A). Separation is only ever concluded
  from a single definition's own distinctions, in one of two ways. Two tops: both are mKeys of
  one mScheme, each carrying :guarantees-unique-name, with differing mValues. One top: exactly
  one mKey is its own top, and the `resolve()` body of its primary mScheme declares, for some
  other shape, `:identified-in` the mSort of the other side's top; an omission is a
  distinction only inside the body that made it. DISJOINT iff one of the two holds and every
  store strictly below A down to either leaf's mParent is :aliases-nothing-else (§2.2); else
  UNKNOWN. The one-top case rests also on a store never being among its own contents.
  Separation is decided once, at A.
- two mKeys at one level are SAME iff they are one instance (one mPlaceholder: inherited
  through a wrapper's sentinel, §3.4, or one ambient instance resolved once in a transit-free
  unwalled span, §1.10) or are equal mValues whose shape carries :guarantees-unique-referent.
  Two mFullyQualifiedKeys are SAME iff they are SAME at every level down to the leaf.
- mKeys of different mSorts share no primary mScheme, so their mFullyQualifiedKeys meet, if at
  all, only at a common ancestor, and that meeting is not a claim about the leaves (a package
  status file and a unit file share a filesystem). The walk above decides such a pair as it
  decides any other: DISJOINT where it separates them, and otherwise KNOWN_UNSPOKEN, which
  never spares and never transports, whatever either side has declared finished (§2.5). Two
  mSchemes yielding one mKey-Primary is the sole same-referent generator across ways of
  naming; Dorc equates mKeys and never merges mSorts.
- partial measurement never widens: a mDerivation with an unmeasured or mRoute-terminated link
  yields at most what it would yield with the link measured.

For mDerivation sets: SAME is "or" across mDerivations (the mFullyQualifiedKey; a
mCorrespondence; a provider-supplied identifier) and "and" within one mFullyQualifiedKey. A
warranted SAME and a warranted DISJOINT on one pair is a contradiction: refuse both, attribute
both authors; otherwise the strongest warranted answer stands. SAME composes transitively;
SAME then DISJOINT composes to DISJOINT; DISJOINT then DISJOINT never chains. Universal meet
over backing sets is unchanged (`set-lifting-universal-meet`). Genuinely different
mKey-Primaries for one mReferent (an NFS filehandle and the server's inode; a machine-id and a
cloud instance-id) are two mDerivations for one mTopic, reconciled by coherence, never a
second mKey inside one mParent.

### § 3.3-perishing (three mutator species, three invalidated facts)

- A ROUTING mutation (a mount, a symlink replacement, a rename, a user added, a hostname
  change, a write to any environment variable, cwd, or configuration a lookup reads) touches
  routing mKeys, a mParent-Catalog, or shell state a `resolve()` read. Every mResolution whose
  mTraversal includes a touched mKey, or whose `resolve()` read the written state — or whose
  mParent-Catalog was touched at all, under the flat default — perishes; every
  mFullyQualifiedKey built on it reads unknown below the line; dependent SAME conclusions lose
  authority and dependent elisions demote to guards; dependent DISJOINT conclusions collide.
  The touched object itself is untouched. Creation, deletion, and rename of an mKey are
  routing writes to its mParent-Catalog entry (its existence cell), so `userdel alice;
  useradd alice` perishes every mResolution of the old mKey
  (`a-recreated-name-is-a-new-referent`); a Writeset that omits the entry is the ordinary
  at-most omission knife, now visibly covering routing mKeys.
- A STATE mutation reaches a cell through the cell's may-read entries: ordinary kill-reach. A
  first write can also change an mKey-Primary
  (`identity-tokens-perish-on-write-not-only-on-rename`), so a state mutation whose Writeset
  touches a mParent-Store perishes the mTokens scoped in it.
- A LIFECYCLE mutation (a reboot, a re-provision) disturbs a mRoot-adjacent mKey (a boot, a
  tenure); every mKey-Primary scoped in it names a new mReferent afterward; cells whose
  mFullyQualifiedKeys pass through it are new and unmeasured; cells whose
  mFullyQualifiedKeys do not are untouched. "Keyed by Boot" and "invariant across Boot" are
  the shape of the mFullyQualifiedKey, not declarations.

In all three the engine withdraws authority; it never computes the successor identity.

### § 3.4-entry-and-lends

> Dynamic binding: `parameterize`, `fluid-let`.

A wrapper's entry :lends mParent-Catalog INSTANCES for the mParent-Catalog mSorts it perturbs
(a chroot lends a mount namespace; `sudo -u` a user; `ip netns exec` a network namespace) and
nothing else; the lent instance becomes the ambient mParent for every mKey of a secondary
mScheme looked up in that mParent-Catalog mSort. mParent-Catalog mSorts not lent inherit the
caller's instance only after the wrapper's completion sentinel; before it they are ⊤. Leaf
mKeys inherit transitively through their mFullyQualifiedKeys with no speech from anyone
(`not-every-transit-changes-the-referent`). A wrapper may declare mCorrespondences across the
mParent-Catalogs it lends (§2.6). A lend may depend on the guest (sudoers matches the guest
command): the wrapper author declares the guest-insensitive default and supplies a policy read
that declines on departure. Entry forms, siting vouches, the escalation dial, and
measure-in-context remain `plans/27C`'s.

### § 3.5-committee-law-satisfied

Every positive step is one author's line: a `:yields` and its lookup warrants; a shape's `:identified-in` and
its warrants; a may-read set and its sentinel; a may-write entailment and its finished record; a
mCorrespondence; a :observer-independence; a :lends. The engine only chains and meets. A
granting composite ("these two accounts are one") is entailed jointly by one mScheme's
`:yields` and the primary mScheme's declaration for that shape, each author speaking about their own lookup
(`28M:rul-composite-meets-toward-guard-run`); a withholding composite (a mount perishing an
account's mResolution) names nobody and needs nobody's consent. Attribution:
every survival names the :aliases-nothing-else and :guarantees-unique-name declarations it rested on and
the may-read sets that bounded it; every SAME names the `resolve()` calls, the declarations,
the sentinels and route claims that made instances one, and the mCorrespondences; every
perished conclusion names the Writeset that perished it.

### § 3.6-scope (what this model leaves untouched)

The verdict, vouch, and guard tier; the authored may-write entries, the completion record as the
witness of a finished definition, and `--risk-faultless-skips`; the four-answer chokepoint
and consumer map; the universal meet; measure-in-context, entry forms, `safe-across`; the read-set closure as the falsification net
for unmarked reads (`27C` §4(a)(B)); binds as the mKey-minting act; the mPlaceholder and the
standup `witness()`; the integrity plane; the committee law. The invariance line's store half
is measured away by §2.2 and its observer half lives as §2.7; the context slot is a mVantage
and nothing else; there is no selector dialect, no aspect species, no
authored region predicate, no engine-side name floor, no engine table that generates `same`.

### § 3.7-supersedes (as currently written; terse by design)

- `plans/30U` § 1 `rul-cross-kind-sparing-needs-a-finished-definition` and § 4 "as a
  generator": a finished definition stays necessary for sparing across mSorts and generates no
  disjointness (§2.5).
- `ANALYZER-NEEDS:an-kind-reach` and `ANALYZER-NEEDS:an-compare-chokepoint`: the `unrelated`
  answer never spares, whatever is finished (§3.2).

## § 4-forcing-functions

A cut of the model is tested against four situations (from `Research/GOTCHAS.md`; more live
there). Half of what is owed is the answer: whether two lines of a book touch one piece of the
world or two. The true answer must be reachable once those who can know have spoken; where
nobody has, the cut must decline to say; and it may never reach the false answer while
everything said is true, since a wrong answer with no false statement behind it refutes the
cut. The other half is who must say what. Every statement the answer rests on is something one
party can know about their own tool, store, or machine, said by that party alone, describing
nothing they cannot see and naming no other author, so that the speech needed grows with the
authors and never with the pairs of them. A cut that is right only by asking someone to know
what they cannot has failed as surely as one that is wrong. The first two situations are the
classic pair, under a kernel that can be asked, answers truly, and cannot be impersonated; the
second two are the same shapes where nothing plays that part, and who answered must be settled
before what they said counts.

- `not-every-transit-changes-the-referent` — owes "one thing": the sameness must carry through
  a wrapper whose author never heard of dpkg.
- `same-name-different-referent-per-viewpoint` — owes "two things", never one: identical argv;
  what separates them is ambient, set by the wrapper.
- `an-emulated-authority-presents-the-real-id` — the first shape with nothing to ask: equal ids
  do not make one thing, and different addresses do not make two, until who answered is known.
- `a-private-name-resolves-only-inside` — the second shape with nothing to ask: owes "cannot
  say" until someone speaks, and an explanation naming where the name was resolved from.

### § 4.1-slugged-gotchas-and-what-each-forces

Referenced by slug (`Research/GOTCHAS.md`); the sentence there is the forcing function, the
line here is the model element it forces.

- `a-path-is-not-a-referent` — mResolutions are facts with mTraversal backings; a remount is a
  routing mutation (§1.7, §3.3).
- `a-host-is-not-a-partition` — the mVantage is an address, never an identity input; identity
  is a mFullyQualifiedKey through the file's mParent, and two mFullyQualifiedKeys terminating
  in two mRoutes read unknown (§1.10, §3.2).
- `same-name-different-referent-per-viewpoint` — a secondary mScheme's mParent-Catalog is
  ambient and lent; the mFullyQualifiedKey differs per instance (§2.1, §3.4).
- `not-every-transit-changes-the-referent` — leaf mKeys inherit through their
  mFullyQualifiedKeys; a transit that lends no mParent-Catalog on an mKey's mFullyQualifiedKey
  leaves it untouched (§3.4).
- `address-inequality-is-not-referent-inequality` — a secondary mScheme's lookup licenses no
  disjointness across mParent-Catalogs; :aliases-nothing-else is a separate property of a store (§2.1, §2.2).
- `distinct-names-alias-within-a-kind` — :guarantees-unique-name is absent by default; the
  mKey-Primary comes from the `:yields` chain (§1.5, §2.1).
- `containment-by-path-prefix-lies` — a path prefix is no store; containment is mTraversal
  membership (§2.2, §2.8).
- `namespace-composition-is-not-concatenation` — mParent-Catalogs are mKeys with identities,
  never strings composed by the engine (§1.6, §3.4).
- `renaming-a-parent-moves-every-child-name` — :hierarchical mSchemes decompose into
  mTraversals; perishing by mTraversal membership (§2.8, §3.3).
- `a-name-resolves-from-a-vantage` — the network mVantage is part of the address and of a
  hostname's mTraversal (§1.10, §2.8).
- `resolution-is-set-valued` — :guarantees-unique-referent is the lookup owner's opt-in; a
  resolver's owner never makes it, so equal hostnames yield unknown and the landing is
  measured (§1.5, §1.10).
- `identity-tokens-have-clone-horizons` — :rootness is a dangerous per-shape claim;
  :guarantees-unique-referent is absent by default on primary mSchemes (§1.6, §2.2).
- `a-name-is-not-a-target-over-time` — mPlaceholders, the standup `witness()`, integrity
  withhold; sameness of a target is continuity witnessed, never a spelling (§1.10).
- `a-store-is-not-one-inode` — may-read is many-valued and distinct from mParent;
  :aliases-nothing-else is not implied by `:may-read` (§2.2, §2.4).
- `an-omitted-store-breaks-invariance` — may-read totality; the completion sentinel;
  omission is the silent channel; the closed set is one of sparing's two closures (§2.4,
  §2.5).
- `nonzero-status-is-not-speech` — every warrant is typed speech, never an exit status; the rc
  regimes of `311a` §6 stand (§1.11).
- `composite-identity-is-structure-not-a-bag` — exactly one mParent; roles are a
  mCompositeSort minted by the author who holds them (§2.3, §2.9).
- `the-subject-includes-the-observer` — :observer-dependence as the surviving half of the
  invariance line; two observers' cells share a may-read entry but not identity (§2.4, §2.7).
- `correspondence-is-known-only-to-the-transition-owner` — `:corresponds` as a
  transition-owner generator; mapped :lends are mCorrespondences (§2.6).
- `identity-tokens-perish-on-write-not-only-on-rename` — a state mutation on a mParent-Store
  perishes the mTokens scoped in it (§3.3).
- `a-recreated-name-is-a-new-referent` — creation and deletion are routing writes to the
  mParent-Catalog entry; a stable mKey over a recreate is a perished mResolution (§3.3).
- `recycled-keys-outrun-the-unwalled-span` — no lookup is a function by default; a
  mParent-Catalog that reissues mKeys never earns :guarantees-unique-referent, and the
  `witness()` cannot see a recycled mKey (§1.5, §1.10).
- `a-cached-lookup-answers-for-the-past` — a secondary mScheme's `resolve()` reads one
  mParent-Catalog instance and declares which; a cache and the file it caches are two
  mParent-Catalogs (§2.1).
- `one-state-reached-through-two-kinds` — the strangers case: two mSorts over one cell,
  minted by authors who never met; KNOWN_UNSPOKEN at the chokepoint, which never spares;
  merged only by a human act (§1.2, §3.2).

### § 4.2-refuted-shapes-and-what-killed-each

Recorded as what-killed-it, so the shape is not re-walked.

- ONE DECLARED SPECIES (an mSort is itself an mScheme; a second way of naming is a second mSort).
  Killed by: every mSort-level attribute (the may-read set, entailment and the finished
  sentence, :observer-dependence, cells) is then written once per mScheme by the same author with
  no seat to say the copies describe one thing; two finished sentences for one carrier land as
  withhold-and-narrate, never the fail-fast a contradiction deserves; a stranger who knows
  only a new lookup must mint a whole mSort or leave it empty. Surviving form: two species
  (§1.2, §1.3).
- AN mSort AS A CATEGORY OF THE WORLD ("the thing meant"). Killed by: the engine never holds
  an mReferent, and strangers mint overlapping vocabularies constantly; reading an mSort as
  the world's category makes two mSorts' disjointness an assumption, which is the
  wrong-DISJOINT knife. Surviving form: an mSort is a declared carrier; cross-mSort pairs are
  known-unspoken (§1.2).
- A DEFAULT mScheme PER mSort. Killed by: it makes an mSort valid where an mScheme belongs, and
  the bind stops saying what the author actually holds. Surviving form: a bind or mark always
  names an mScheme (§1.3).
- TWO ROLES PER mKey (an mParent-Catalog AND an mParent-Store, as two edges). Killed by: at the
  primary mScheme they are one mKey, and a secondary mScheme's mKey has no identity of its own
  but its yield's. Surviving form: one mParent per mKey, its meaning following the mScheme
  (§1.6).
- THE mParent DECLARED PER mScheme (one mParent mSort for all of a primary mScheme's mKeys).
  Killed by: what a filesystem identifier is scoped in varies by the filesystem's TYPE (ext4
  in a boot; NFS in a server; sshfs synthesising inodes per client), never by the child mSort
  or its mSchemes. Surviving form: mParent and warrants are declared per matched shape of the
  mKey's mValue (§2.2).
- SUBSORTS, A TYPE MENU, OR ONE MEMBER PER TYPE. Killed by: a second name is a second mScheme,
  and the variation is a partial function of one mScheme's mValues. Surviving form: per-shape
  declarations on one `resolve()` (§2.2).
- DERIVING :aliases-nothing-else FROM THE STRUCTURE OF THE mFullyQualifiedKey. Killed by nested pid
  namespaces: guest pid 1 and host pid 4821 are one mReferent and both mFullyQualifiedKeys
  resolve cleanly to one boot. Surviving form: :aliases-nothing-else stays declared, absent by default,
  per store (§2.2).
- AN ASPECT SPECIES (a selector position; an aspect borrowing another mSort's lookup). Killed
  by: one mParent per identity-bearing thing, a per-aspect may-read set, and per-aspect
  :observer-dependence already make an aspect an mSort in all but name, and the borrowed
  lookup is a primary mScheme `:identified-in` the mParent's mSort. Surviving form: a cell is a
  singleton mSort under its mParent (§1.9).
- IDENTITY AS A PER-KIND TABLE AGAINST AXES (the trichotomy invariant/keyed/⊤ per index-kind,
  `30W` §4; the filtered meet, `26Ob` §10b). Killed by: the mSort owner cannot know the axes;
  silence walls forever and a guess ("keyed by Host") plus a referent-transparent Host yields
  a wrong DISJOINT on a shared volume. The truth: a cell's identity is a property of what its
  mKey denotes, not of the mScheme the mKey is written in.
- THE CONTEXT AS PART OF THE `FactKey`. Killed by: a qualified mKey is an address of a question
  and asserts no partition; the dangerous inference lived in the meet's generators. Surviving
  form: the mVantage is an address and what keys the `witness()` (§1.10).
- STORE SETS, unioned and compared as bags. Killed by base-and-overlay: same set, different
  roles, different answers; and by the reflexivity defect of pairwise set equality. Surviving
  form: one mParent, mCompositeSorts for roles (§2.9).
- "STORED-IN" AS ONE RELATION conflating routing and containment. Killed by hardlinks, bind
  mounts, NFS: mParent-Catalog disjointness is not mReferent disjointness. Surviving form:
  `:yields` with its mParent-Catalog, the store's :aliases-nothing-else, and
  may-read as three relations.
- TERMINAL mTokens (`Measured(File, fsid:inode)`). Killed by NFS and by the mParent question:
  an inode is an mKey in a filesystem, a filesystem identifier is an mKey in whatever minted
  it; the File owner should never learn NFS, and under this model does not, because the
  mParent mSort's primary mScheme classifies one level up (§2.2). Surviving form: :rootness as
  an explicit dangerous claim; every mToken scoped.
- ONE GRADE ON A LOOKUP (transparent/identifying). Killed by clones: equal machine-ids on two
  machines threaten SAME, which a disjointness grade cannot protect. Surviving form: two
  independent warrants per lookup (§1.5).
- OBJECT IDENTITY CARRIES EVERY OBSERVATION ("one cell, one fact, whichever probe read it").
  Killed by `test -w` under two users on one file. Surviving form: :observer-dependence per
  mSort, absent means dependent (§2.7).
- AUTHORED REGION PREDICATES (`kind__disjoint`; a subtree selector). Killed by the
  symlink-retarget case: separation of two objects says nothing about whether changing one
  retargets an mKey for the other; and by the granule observation that the natural
  description unit is the atomically-disturbable one. Surviving form: :hierarchical mSchemes
  and mTraversals (§2.8).
- IDENTITY BINDINGS BACKED BY THEIR TARGET OBJECT. Killed by the same symlink case. Surviving
  form: mResolution backed by mTraversal (§1.7).
- UNION TOTALITY WITH NO AUTHOR (an mSort-level ∪ mSite-level sentinel). Killed by asking who
  closed the union. Surviving form: the owner declares and closes; mSites fill
  mParent-Catalogs.
- SHARED ANCESTORS AS COLLISIONS. Killed by the package status file and the unit file sharing
  a filesystem. Surviving form: ancestors are mParents; `compare()` at the divergence (§3.2).
- mRoute-VERSUS-mRoot AND mRoots-DIFFER AS KNOWN_UNSPOKEN. Killed by the finished definition spending
  across mWorlds: a mFullyQualifiedKey with fewer measured links compared more decisively
  than the same one complete, and the Package author became the wrong name in the why chain.
  Surviving form: both read UNKNOWN; partial measurement never widens (§3.2).
- THE DISCLOSED-WEAK NAME FLOOR (within one mSort, unequal names answer disjoint). Killed by
  the human's default-safe lean and by its own per-mSort carves. Surviving form:
  :guarantees-unique-name is typed, never assumed (§1.5).
- A DEFINITIONAL EQUAL-KEYS DEFAULT ("equal mKeys in one mParent-Catalog across an unwalled
  span reach one mReferent, because that is what an mScheme is"). Killed by: it feeds only the
  dangerous consumer (kill-reach already collides on unknown); recreation under a stable
  name, pid reuse, round-robin and cached lookups, and `:latest` tags are ordinary ops; and
  the claim's owner differs at every level. Surviving form: the engine vouches only the
  transit-free local mRoute; every lookup's functionality is its owner's opt-in (§1.5, §1.10).
- MEASUREMENT MAKES DECLARATIONS REDUNDANT. Killed by: a `resolve()` establishes a mToken, not
  its mParent, mTopic, warrant, applicability, or sufficiency. Surviving framing: measurement
  relocates speech to questions the owner can answer.
- RENAMING RESOLVE INTO IDENTITY FIXES THE FALLTHROUGH IDIOM. Killed by: a renamed member can
  still echo a fallback and exit 0; the repair is the completion-only rc regime plus a taught
  decline, not the name.
- THE DISAGREEMENT CANARY AS A SAFETY ARGUMENT. Killed by: the second probe exists only
  because nothing was elided on the first; absence of disagreement is not evidence, and a
  wrong bind is a wrong SAME that lands, attributed.
- CONVERGENCE OF TWO SYSTEMS FROM ONE WINDOW AS EVIDENCE. Killed by the human's first bullet.
  Surviving discipline: clean-context adversarial review before any design-of-record.
- A FINISHED DEFINITION AS A GENERATOR OF DISJOINT ACROSS mSorts. Killed by `ctr task kill`
  against a converged `docker start` of that same container: "nothing else" is no other thing,
  never no other mKey for the thing written. Surviving form: `compare()` decides every pair;
  the completion record witnesses a complete write set (§2.5, §3.2).
- ONLY SAME may-read entries OVERLAP. Killed by: unequal names, nobody's speech, sparing, the
  name floor moved onto may-read entries. Surviving form: whatever is not DISJOINT collides (§2.5).
- A STORE'S CLOSURE AS "REACHABLE ONLY THROUGH IT". Killed by: nobody can know what else
  aliases their store. Surviving form: :aliases-nothing-else, self-knowledge, asked of both
  legs (§2.2).
- SEPARATION DRAWN FROM TWO DEFINITIONS AT ONCE. Killed by one definition's store inside a
  shared mParent against another definition's mKey directly in it, one slot under two
  vocabularies, every positive sentence true. Surviving form: an omission is a distinction
  only inside the body that made it (§3.2).
- A PARENT PARTITIONING ITS CHILDREN'S mSorts. Killed by: children minted by other definitions
  could never spare. Surviving form: both sides identify into one mScheme whose body warrants
  :guarantees-unique-name (§3.2).
