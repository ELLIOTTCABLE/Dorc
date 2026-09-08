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
> as a referent-agnostic example of a namespace or a transition.
>
> Naming discipline: where a prior-art term overlaps precisely, this document takes it and
> says which domain it comes from; where a nearby term is a false friend or is squatted by
> our own vocabulary, the heading or a blockquote under it says so.

## § 0-one-screen

Dorc removes a line only on a proof the line is unnecessary: a measurement taken before
anything runs, plus an author's vouch. The measurement can be destroyed by an earlier line
that really runs, so the engine must decide whether the piece of the world an earlier line
touches is the same piece a later line's proof depends on. That is identity, and it has two
consumers with opposite failure directions: SAME lets one fact stand for another and
under-executes when wrong; DISJOINT lets a proof survive a write and under-executes when
wrong. UNKNOWN is safe for both. The engine knows only syntax, authored speech, and probe
measurements; it never decodes a name. No single author knows the whole path from a tool's
name to a measurable referent, and viewpoint transitions authored by yet other people change
which referent a name reaches mid-book.

The model: every identity-bearing thing is an entity of a KIND. A kind's entities have a
NATURAL KEY inside a namespace (routing), and, where the kind's owner supplies a read, a
PRIMARY KEY inside another namespace (identity). Identity is the primary key scoped
recursively, a FULLY-QUALIFIED KEY, until a ROOT, a declared global namespace, or a ROUTE,
the last resort. State has PLACEMENTS, many-valued, which feed collisions only. Two things
compare by walking their fully-qualified keys to the first divergence inside a shared scope
and asking that scope's naming what inequality licenses; never diverging is sameness;
different roots are unrelated. Every positive answer rests on an explicitly typed,
long-named, absent-by-default warrant owned by one author; silence is unknown. Transitions
are of three species, each perishing a different fact. The context slot survives as a
VANTAGE, an address and a witness key, never as an identity input.

## § 1-model-objects

### § 1.1-referent

A piece of the world that has state: an inode, a database row, a package record, a kernel
parameter, a running process, a machine, a mount table. The engine never holds a referent.
It reaches referents only through keys, tokens, and derivations, and every identity
question is ultimately "do these two keys reach one referent". Referents are what GOTCHAS
are about.

### § 1.2-kind

> Not the PLT kind (the type of a type). A kind here is a *sort* in the many-sorted-logic
> sense: a named naming scheme over referents with one accountable owner, not a category of
> referents. Two kinds may name one referent set (a file by path and by descriptor; a host by
> name and by instance-id), each with exactly one natural key; their sameness is decided by a
> shared identifying scope (§3.2). The word is kept because the corpus is saturated with it;
> the PLT sense never arises in this domain.

A vocabulary with one accountable owner (reverse-DNS naming, no registry, owner-adjudication
as the social contract — unchanged from `24M` and `277` §6). A kind fixes, for its entities:
the natural-key naming (§2.1), the read and primary key if any (§2.2), the placements
(§2.3), the effect entailment (§2.4), the observer-dependence (§2.6), and whether it is a
hierarchical namespace when it serves as one (§2.8). A kind's owner speaks only about the
kind's own relations to its immediate neighbours; the engine composes. Kinds are the only
identity-bearing category; there is no separate entity type, aspect type, or context type in
the model (§1.8, §1.9).

### § 1.3-name-natural-key-primary-key

> RDBMS terms, taken whole with their culture: a natural key is domain-meaningful,
> user-typed, may alias, and is never trusted as identity; a primary key is what the store
> itself identifies rows by, and is unique only within its own table. Clones are two tables
> with colliding primary keys.

A name is a string an author wrote, addressing a referent of a kind FROM SOMEWHERE. Every
name is relative to a namespace instance, and a bare string means nothing. Each kind has:

- a NATURAL KEY: the strings tool authors and books use (`red/7`, `/etc/nginx.conf`,
  `nginx`, `beta`), interpreted in the kind's routing namespace (§2.1);
- a PRIMARY KEY: the strings the kind's own identity read returns (an inode number, a
  filesystem identifier, a canonical package name, a boot identifier), interpreted in the
  kind's identifying scope (§2.2).

When a kind has no read, the primary key IS the natural key. The distinction exists because
the natural key is what people write and is usually ambiguous (aliases, relative names),
while the primary key is what the store answers and is where the dangerous warrants can
honestly be placed.

### § 1.4-namespace-instance-scope-root

A namespace instance is an entity of some kind, used as the space in which another kind's
keys are interpreted: a database file for account keys, a directory for the next path
component, a mount table for paths, a filesystem for inodes, a resolver-plus-vantage for
hostnames, a process table for pids. "Scope" is the same object seen from the key: the scope
of `red/7` is a particular database. A kind's natural key has one scope (its routing
namespace) and its primary key has another (its identifying scope); they coincide when the
kind has no read. A ROOT is a kind that declares its primary keys need no scope, which is the
claim that they are globally comparable (the DNS root, whose keys are fully-qualified).
Rootness is a dangerous, long-named claim (§2.2); nothing is a root by default.
`GOTCHAS: identity-tokens-have-clone-horizons` is the standing witness against casual
rootness.

### § 1.5-token-and-the-two-warrants

> The two warrants are OWL's inverse-functional and functional properties, and OWL's
> inference consequences transfer exactly (inverse-functional is what licenses `sameAs`).
> OWL's names swap direction with the property's orientation, which is why ours are spelled
> out instead.

A token is a primary-key value: bytes the kind's read returned, compared for equality only,
never decoded (`inv-referent-agnostic`). A token is always scoped (§1.4). Any naming,
natural or primary, has two independent properties, and each is a separately declared,
absent-by-default warrant:

- ONE-NAME-ONE-REFERENT: within one scope instance, equal keys reach one referent. Licenses
  SAME from equality. Fails for round-robin resolution (`resolution-is-set-valued`) and for
  cloned identifiers presented as roots (`identity-tokens-have-clone-horizons`).
- ONE-REFERENT-ONE-NAME: within one scope instance, one referent has one key. Licenses
  DISJOINT from inequality. Fails for symlinks and hardlinks, package `provides`
  (`distinct-names-alias-within-a-kind`), route-qualified handles.

The one non-unknown default in the model, flagged for the human's eye: for a NATURAL key,
equal keys inside ONE scope instance across an UNWALLED span are taken to reach one
referent, because that is what a namespace is (a lookup), and `26Ob` §5 already accepted the
same default as placeholder sharing under one entry chain. A namespace whose lookup is not a
function opts out by declaring set-valued lookup. Everywhere else, including every primary
key compared across scope instances and every root, both warrants are absent until typed.

### § 1.6-resolution-and-traversal

A resolution is the fact that natural key N, interpreted in namespace instance S, at
program point p, reaches referent R. It is a FACT with a BACKING, and its backing is the
TRAVERSAL: the ordered chain of routing entities the lookup crossed (each directory entry and
symlink for a path; the resolver configuration and vantage for a hostname; the unit table for
a service name). A hierarchical namespace's structure fixes how a key decomposes into a
traversal (§2.8). A resolution perishes under ordinary effective-world reach from any mutator
whose footprint touches any traversal member. Its target object is NOT its backing: a key
can stop reaching an object without the object changing
(`renaming-a-parent-moves-every-child-name`, `a-path-is-not-a-referent`), and an
object can change without its key changing.

### § 1.7-fully-qualified-key-and-derivations

A fully-qualified key is the recursive identity of an entity: its primary key, scoped in its
identifying scope, whose identity is itself a fully-qualified key, terminating at a root, at
a route (§1.9), or at unknown. A fully-qualified key is one DERIVATION of identity. A topic
may carry several derivations with different generators: its fully-qualified key, a
provider-supplied identifier, a correspondence asserted by a transition owner (§2.5).
Derivations combine by coherence (§3.2), never by priority. Equality composes transitively
across derivations; separation is decided at one divergence and is never chained. ("Witness"
is reserved throughout for the standup re-measurement of §1.9.)

### § 1.8-cell-aspect-kind-value

A cell is the unit that has a value: what a probe measures and a mutator writes. In this
model a cell is simply an entity of a kind whose values are measurable. What earlier designs
called an aspect or selector (`Service:nginx@enabled`) is an ASPECT-KIND: a kind that borrows
another kind's natural-key naming (§2.7) but has its own read, identifying scope, placements,
and observer-dependence. `enabled` and `active` are two kinds sharing the unit name; one is
measured in a symlink's existence and the other in pid 1's memory, and their fully-qualified
keys differ, which is exactly why one survives a reboot and the other does not, with no
declaration about reboots by anyone. Three independent findings forced this fold (§6.2): one
identifying scope per identity-bearing thing, per-aspect placement, per-aspect
observer-dependence. The whole-entity claim (`disturbs Service:nginx`) reaches its
aspect-kinds by the owner's effect entailment (§2.4), and absent that entailment collides
with them as unrelated, which is the safe bottom. The selector position in the coordinate
and the selector dialect (`277` §3, `30J`) have no counterpart in this model.

### § 1.9-vantage-route-placeholder-witness

A vantage is the ADDRESS a probe reached a referent from: the entry chain, expressed as a
finite map from namespace kinds to the instances that wrappers lent (§3.4).
It is not part of any entity's identity. It has three jobs: it says where a read
executes; it supplies the ambient namespace instance for keys whose kind is named-in a lent
namespace; and, for a kind with neither read nor namespace, it is the ROUTE, the last-resort
scope under which two same-spelled keys in one unwalled span are one placeholder and anything
across vantages is unknown. Fully-qualified keys whose tokens are not yet measured are
placeholders keyed by (key, ambient instances, entry chain); the probe standup binds them;
the apply standup re-reads them through the same entry and compares (the WITNESS); mismatch
is integrity, never a verdict input (`26Ob:cor-standup-witness-licenses-bare-line-elision`).

### § 1.10-site-and-claim-species

A site is within a book line, with an argv, an entry chain, and a program point.
Speech at or about a site, each with one author:

- a VERDICT FACT: a cell's measured value, backed by the marked read set
  (`30T:req-verdict-marks-every-read-cell`), vouched by the tool-oracle author;
- a FOOTPRINT CLAIM: an at-most write set per matched shape, with its completion witness
  (`30U`), by the tool-oracle author or the filesystem binder;
- an EFFECT ENTAILMENT: what disturbing an entity of a kind drags along, with the finished
  definition (`30U`), by the kind owner;
- a CORRESPONDENCE: key X in namespace instance A is key Y in namespace instance B, by the
  owner of the transition between A and B (§2.5);
- the kind-level DECLARATIONS of §2: namings, warrants, placements, hierarchy,
  observer-dependence, rootness;
- the wrapper's LENDS and their completion sentinel (§3.4).

## § 2-model-relations

Each relation below states: arity, who declares it, its default, which consumer reads it,
and which warrant makes it dangerous. The human's naming law applies throughout
(**[LEAN]**: dangerous operations rare and long-named; safe ones short) but no spellings are
proposed here.

### § 2.1-named-in (the natural key's namespace; routing)

Kind K's natural keys are interpreted in an instance of kind N. Declared by K's owner, once
per kind. The INSTANCE is supplied one of three ways, and the supply mode is part of the
declaration: FIXED (K's owner names it: the dpkg status file), SITE (the tool author's bind
fills it from argv: `--db`), AMBIENT (the entry chain's lent instance for N: the current
mount table for paths). Consumer: resolution (§1.6). Default: absent means K's keys have no
namespace, so they resolve only under the route (§1.9). Warrants: the two of §1.5, with
one-name-one-referent definitional within one instance and unwalled span, one-referent-one-
name absent. This relation never licenses disjointness across instances by itself:
namespace-disjointness is not referent-disjointness (`a-host-is-not-a-partition`,
`address-inequality-is-not-referent-inequality`, `containment-by-path-prefix-lies`).

### § 2.2-identified-in (the read, the identifying scope, sole-route, rootness)

> ER modeling's identifying relationship: a weak entity's primary key includes its
> identifying entity's key; a strong entity has its own. Identifying is NOT containing.
> The containing half is `sole-route` below (networking: single-homed), a separate,
> dangerous claim; conflating the two is the hardlink and NFS mistake.

K's identity read, declared and executed by K's owner in the vantage the key resolved from,
maps a natural key to a primary key scoped in an instance of kind M. That M-instance is K's
IDENTIFYING SCOPE, the thing whose identity determines K's. The read returns a scoped token,
and the scope's identity is recursive (§1.7). Where no read exists, M = N and the primary
key is the natural key. Consumer: identity (§3.1). Defaults: no read means identity falls to
the natural key under its own defaults. Warrants, all absent by default and all declared by
K's owner about the primary key:

- one-name-one-referent and one-referent-one-name (§1.5), governing what token equality and
  inequality license at K's level of a fully-qualified key;
- SOLE-ROUTE: K's referents are reachable only through their identifying scope, so disjoint
  scopes imply disjoint K-referents. Required for a divergence at the identifying-scope level
  to yield DISJOINT for K. Fails for replicated rows, for files reachable through several
  directories (a hardlink), for a record whose current state spans several files
  (`a-store-is-not-one-inode`);
- ROOTNESS: K declares no M, and thereby claims its primary keys are globally comparable.
  Equivalent to declaring one-name-one-referent over the whole world; fails for cloned
  identifiers (`identity-tokens-have-clone-horizons`). A token that can be duplicated across
  instances of its would-be scope must be scoped in something smaller or left un-warranted.

A grade governs every consumer of the answer it grades, corroboration and contradiction
included: a read without one-referent-one-name cannot contradict anything by returning two
different tokens.

### § 2.3-lives-in ("placement", i.e. read footprint in separation logic)

> The correct name is footprint. `disturbs` squats it for the write footprint, and `backing`
> squats the per-fact read footprint (`23M`). Placement is the kind-declared read footprint
> that a verdict fact's backing refines at probe time.

K's referents' state is affected by writes to these entities. Many-valued. Declared by K's
owner, per kind, with a completion sentinel closing the set. Consumer: collision only. A
footprint touching any placement collides with K's cells; an incomplete placement set loses
protection for sparing (an omitted placement is a silent channel:
`an-omitted-store-breaks-invariance`, `a-store-is-not-one-inode`) and never
licenses anything positive. Placement is
distinct from the identifying scope: the identifying scope is at most one and answers
identity; placement is many and answers interference. Two cells with different identifying
scopes can share a placement and therefore collide without being the same
(`the-subject-includes-the-observer`: two observers' writability cells share the file's
mode).

### § 2.4-reaches (effect entailment), and the finished definition

Unchanged from `plans/30U`: disturbing an entity of K entails disturbing these entities of
other kinds; arm-incremental, collide-adding; the reached completion record finishes the
definition and is the sole licensor of sparing across UNRELATED keys (§3.2). Declared by
K's owner. This relation is about effects, not identity, and it is what carries a package's
postinst enabling its unit, a restart killing a main process, and every other cross-kind
consequence that no fully-qualified key expresses.

### § 2.5-corresponds (declared sameness across a transition)

> A scoped `sameAs`. `23M` rejected `owl:sameAs` for admin-declared cross-kind co-reference;
> this is the other case, same-kind across two namespace instances, asserted by the one
> author who owns the transition between them (the NAT table, the pid-namespace map).

Natural key X in namespace instance A denotes the same referent as key Y in namespace
instance B. Declared by the owner of the TRANSITION between A and B, which is neither key's
kind owner: the container manager knows guest pid 1 is host pid 4821
(`correspondence-is-known-only-to-the-transition-owner`); `sudo -u alice` knows inner "me"
is outer "alice", which is what a mapped lend has been asserting all along. Consumer: a
SAME derivation (§1.7), vouch-tier, attributed to the transition author. Default: absent, so
keys across a transition compare unknown unless a read binds tokens on both sides. Danger: a
wrong correspondence is a wrong SAME. This generalizes the mapped lend of `273` and is the
model's only declared sameness generator besides token equality.

### § 2.6-observer-dependence

K's cells' VALUES depend on which instance of namespace kind O the measurement was taken
under. Declared by K's owner per kind as its complement, observer-independence of O.
Default: a cell measured under a lent instance of O is assumed to depend on it, so its fact
is about (referent, O-instance) and never stands for the same referent under another
O-instance. Consumer: the SAME consumer, as a qualifier on the claim's topic. This is the
surviving half of the old invariance line (`271:rul-invariance-speech-act`): its store half
is measured away by §2.2, its observer half cannot be measured by any identity read because
the object is the same and the answer differs (`the-subject-includes-the-observer`), and it
must remain speech. Measurement in the denoted context (`plans/27C`) stays the default lane;
carrying a fact across an O-shift requires this declaration, and the human's hoped-for
static no-transit path costs exactly one such line per kind in the stdlib.

### § 2.7-named-like (aspect-kinds)

Aspect-kind A borrows kind K's natural-key naming: A's entities are addressed by K's keys in
K's namespace, and A has its own read, identifying scope, placements, and
observer-dependence. Declared by A's owner, who is normally K's owner. Consumer: resolution
of A's keys via K's §2.1. The whole-entity relationship between K:x and A:x is carried by
K's `reaches` (§2.4); without it they are unrelated and collide, which is safe.

### § 2.8-hierarchical-namespace (versus flat; traversal structure)

A namespace kind is HIERARCHICAL when its keys decompose into an ordered chain of its own
entities, each a routing entity the lookup crosses: a path into directory entries, a hostname
into resolver steps from a vantage, a dotted unit name into its instance table. Otherwise it
is FLAT. The engine derives the decomposition from key SYNTAX, which is language, plus this
declaration (`30T` §5's syntax-versus-semantics line, unchanged). Consumer: resolution
backings (§1.6) and therefore perishing (§3.3). Default: flat, so the traversal is the
namespace instance as a whole, and any touch on the instance perishes every resolution
through it, the coarse and safe floor. This is what replaces authored region predicates:
containment is membership in a traversal, and a mutator that touches a directory needs to
know nothing about files (`renaming-a-parent-moves-every-child-name`,
`namespace-composition-is-not-concatenation`).

### § 2.9-composite-kinds (roles)

A topic whose value depends on several inputs IN ROLES (a base and an overlay; a primary and
its replica set) is an entity of a composite kind minted by the author who knows the roles,
normally the tool author, and that kind's identity is defined by its owner from its named
parts (ER modeling's associative entity with role names). There is no set of identifying
scopes anywhere in the model: the identifying scope is at most one (§2.2), and plurality of
inputs is a kind with structure (`composite-identity-is-structure-not-a-bag`). Placement of a
composite is the union of its parts' placements.

### § 2.10-relation-table

| relation | arity | declared by | default | consumer | danger |
|---|---|---|---|---|---|
| named-in | one per kind, instance per key | kind owner (instance: fixed / site / ambient) | no namespace ⇒ route only | resolution | one-referent-one-name (disjoint from inequality) |
| identified-in (the read) | one per kind | kind owner | none ⇒ primary key = natural key | identity | one-name-one-referent · one-referent-one-name · sole-route · rootness |
| lives-in | many per kind, sentinel | kind owner | ⊤ ⇒ collides with everything of the kind | collision | none positive; omission is the silent channel |
| reaches + finished | many, per arm | kind owner | unrelated ⇒ collide | cross-root sparing | the premature finished record |
| corresponds | per transition pair | transition owner | unknown | SAME derivation | a wrong correspondence |
| observer-independence | per (kind, O) | kind owner | dependent ⇒ no carry | SAME qualifier | a false independence |
| named-like | one per aspect-kind | aspect owner | none | resolution reuse | none |
| hierarchical | per namespace kind | namespace owner | flat ⇒ whole-instance traversal | perishing | none (finer is value, coarse is safe) |
| lends (+ sentinel) | per wrapper, per namespace kind | wrapper owner | ⊤ ⇒ walls | ambient instance supply | a wrong lend measures the wrong vantage |
| composite kind | per composite | the author holding the roles | n/a | identity | as any kind |

## § 3-composition

### § 3.1-identity-of-an-entity

identity(e of kind K) is the primary key of e (via K's read run from e's vantage, or the
natural key if K has no read), scoped in identity(identifying-scope instance), recursively,
until a root, a route, or an unknown link. Each level carries the warrants K's owner declared
for that naming. A composite kind's identity is its owner's function of its parts'
identities. An entity of an observer-dependent kind carries the O-instance as part of its
topic. Nothing about identity consults the vantage map except to know where to run reads
and which ambient instances to bind.

### § 3.2-compare (one chokepoint, four answers)

> Alias analysis's trichotomy: `same` is must-alias, `disjoint` is must-not-alias, `unknown`
> is may-alias, and `unrelated` is may-alias with the extra fact that no generator ever
> spoke. `24F`'s `MayAlias` was this vocabulary before the corpus renamed it.

compare(x, y) ∈ {same, disjoint, unrelated, unknown}, consumers exactly as today
(`compare-consumer-map`: same → the fact is about this cell; disjoint → sparing under
`--risk-faultless-skips`; unknown and unrelated → the safe bottoms). For two fully-qualified
keys:

- walk from the roots. If the roots differ, or one side is a route and the other is not, the
  answer is UNRELATED, and sparing across the pair rides only the footprint side's finished
  definition (§2.4), as `30U` has it.
- at the first level where the two keys differ inside a shared scope S: DISJOINT iff S's
  naming for that kind carries one-referent-one-name and every level above it carries
  sole-route; else UNKNOWN. Deeper levels are not consulted; separation is decided once.
- if no level differs: SAME iff every level's equality is warranted (definitional for natural
  keys within one instance and unwalled span; one-name-one-referent for primary keys and
  roots); else UNKNOWN.
- any unknown link on either key: UNKNOWN.

For derivation sets: a warranted SAME and a warranted DISJOINT on one pair is a
contradiction, refuse both and attribute both authors; otherwise the strongest warranted
answer stands. SAME is transitive across derivations; DISJOINT is never chained. Universal
meet over backing sets is unchanged (`set-lifting-universal-meet`): sparing needs every
footprint-by-backing pair disjoint. Cross-kind comparison needs no special rule: two
fully-qualified keys either share a scope at some level or they do not. A shared identifying
scope with equal primary keys IS the cross-kind SAME generator: two kinds whose owners
identify into one scope name one referent set through two natural keys, one thing in two
tables joined on a shared key, and this is the one place the corpus's "cross-kind same does
not exist" is superseded. Genuinely different primary keys for one referent (an NFS
filehandle and the server's inode; a machine-id and a cloud instance-id) are two derivations
for one topic, reconciled by §1.7's coherence, never a second key inside one scope.

### § 3.3-perishing (three mutator species, three invalidated facts)

- A ROUTING mutation (a mount, a symlink replacement, a rename, a user added, a hostname
  change) touches routing entities. Every RESOLUTION whose traversal includes a touched
  entity perishes; every fully-qualified key built on that resolution reads unknown below the
  line; dependent SAME conclusions lose authority and dependent elisions demote to guards;
  dependent DISJOINT conclusions collide. The touched object itself is untouched
  (`a-path-is-not-a-referent`, `renaming-a-parent-moves-every-child-name`).
- A STATE mutation writes cells through placements: ordinary kill-reach, unchanged. A first
  write can also change a primary key
  (`identity-tokens-perish-on-write-not-only-on-rename`), so a state mutation
  whose footprint touches an identifying scope perishes the tokens scoped in it.
- A LIFECYCLE mutation (a reboot, a re-provision) disturbs a root-adjacent entity (a boot, a
  tenure); every primary key scoped in it names a new referent afterward; cells whose
  fully-qualified keys pass through it are new and unmeasured; cells whose keys do not are
  untouched. What earlier designs declared as "keyed by Boot" or "invariant across Boot" is
  the shape of the key, not a declaration (`plans/30W` §7's patch day, derived).

In all three the engine withdraws authority; it never computes the successor identity.

### § 3.4-entry-and-lends

> A lend is dynamic binding: the wrapper rebinds a namespace parameter for the guest's
> dynamic extent, exactly `parameterize` or `fluid-let`. Nesting, shadowing, and
> innermost-wins all follow from that frame; nothing about it is Dorc-specific.

A wrapper's entry lends NAMESPACE INSTANCES for the namespace kinds it perturbs and nothing
else; the lent instances become the ambient supply for keys whose kinds are named-in those
namespace kinds. Namespace kinds not lent inherit the caller's instance only after the
wrapper's completion sentinel over namespace kinds; before it they are ⊤. Leaf kinds inherit
transitively through their fully-qualified keys with no speech from anyone
(`not-every-transit-changes-the-referent`). A wrapper may additionally declare
correspondences across the namespaces it lends (§2.5). Entry forms, siting vouches, the
escalation dial, and measure-in-context remain `plans/27C`'s; only the fallback lane changes
shape (§3.6).

### § 3.5-committee-law-satisfied

Every positive step is one author's line: a naming and its warrants, a read, a sole-route
flag, a placement set and its sentinel, an entailment and its finished record, a
correspondence, an observer-independence, a lend. The engine only chains and meets. A
granting composite ("these two accounts are one") is entailed jointly by the account
owner's identified-in declaration and the database owner's read, each speaking about their
own kind (`28M:rul-composite-meets-toward-guard-run`). A withholding composite (a mount
perishing an account's resolution) names nobody and needs nobody's consent. Attribution:
every survival names the sole-route and one-referent-one-name lines it rested on; every SAME
names the reads and correspondences; every perished conclusion names the footprint that
perished it.

### § 3.6-dissolved-kept-relitigated

Dissolved (no counterpart): the context slot as an identity input and the per-index-kind
trichotomy with its filtered meet (`plans/30W` §4, `26Ob` §10b); the invariance line as one
thing (split: store half measured, observer half §2.6); substrate tokens; `kind__disjoint` as
an authored region predicate (containment is traversal membership); `resolve` as a member
distinct from the read; the selector position and the selector dialect (`277` §3, `30J`);
the engine-side name-floor carves for File and index-kinds (they are the absent
one-referent-one-name warrant); the disclosed-weak default name floor
(`300:rul-reference-entity-name-floor`) **[LEAN: default safe even when painful]**;
"transport" as a lane for observer-independent cells (one cell, one fact); the store member's
argv blindness (the site fills the instance).

Kept unchanged: the verdict, vouch, and guard tier; footprints, `reaches`, finished
definitions, and `--risk-faultless-skips`; the four-answer chokepoint and consumer map; the
universal meet; measure-in-context, entry forms, `safe-across`; the read-set closure as the
falsification net for unmarked reads (`27C` §4(a)(B)); binds as the naming act; the
placeholder and standup witness; the integrity plane; the committee law.

Re-litigated on the merits: `272` §5 addresses-are-not-coordinates (a placement IS a
coordinate in another kind, and identity is a fully-qualified key so no store-level collapse
follows); `279f` §3's refused transport chain (re-opened as fully-qualified keys of
warranted reads, not as backing completeness); `271:rul-invariance-speech-act` (re-read:
textual derivation never licenses; an authored read under a typed warrant does; the observer
half stays speech).

## § 4-epistemics

*(deliberately empty; to be filled backwards from §1–§3)*

## § 5-ux-gradual-enhancement-teaching

*(deliberately empty; to be filled backwards from §1–§3)*

## § 6-forcing-functions

### § 6.1-slugged-gotchas-and-what-each-forces

Referenced by slug (`Research/GOTCHAS.md`); the sentence there is the forcing function, the
line here is the model element it forces.

- `a-path-is-not-a-referent` — resolutions are facts with traversal backings; a remount is a
  routing mutation (§1.6, §3.3).
- `a-host-is-not-a-partition` — the vantage is an address, never an identity input;
  identity is a fully-qualified key through the file's identifying scope (§1.9, §2.2).
- `same-name-different-referent-per-viewpoint` — the routing namespace instance can be
  ambient and lent; the fully-qualified key differs per instance (§2.1, §3.4).
- `not-every-transit-changes-the-referent` — leaf kinds inherit through their keys; a
  transit that lends no namespace on a key leaves it untouched (§3.4).
- `address-inequality-is-not-referent-inequality` — named-in licenses no disjointness across
  instances; sole-route is a separate, dangerous flag (§2.1, §2.2).
- `distinct-names-alias-within-a-kind` — one-referent-one-name is absent by default; the
  read supplies the primary key (§1.5, §2.2).
- `containment-by-path-prefix-lies` — the File natural key's namespace is not sole-route;
  containment is traversal membership on the primary-key side (§2.2, §2.8).
- `namespace-composition-is-not-concatenation` — namespace instances are entities with
  identities, never strings composed by the engine (§1.4, §3.4).
- `renaming-a-parent-moves-every-child-name` — hierarchical namespaces decompose into
  traversals; perishing by traversal membership (§2.8, §3.3).
- `a-name-resolves-from-a-vantage` — the network vantage is part of the address and of a
  hostname's traversal (§1.9, §2.8).
- `resolution-is-set-valued` — the one definitional default has an owner opt-out; a
  set-valued lookup yields unknown from equal keys (§1.5).
- `identity-tokens-have-clone-horizons` — rootness is a dangerous claim; one-name-one-
  referent is absent by default on primary keys (§1.4, §2.2).
- `a-name-is-not-a-target-over-time` — placeholders, the standup witness, integrity
  withhold; sameness of a target is continuity witnessed, never a spelling (§1.9).
- `a-store-is-not-one-inode` — placement is many-valued and distinct from the identifying
  scope; sole-route is not implied by "lives in" (§2.2, §2.3).
- `an-omitted-store-breaks-invariance` — placement totality; the completion sentinel;
  omission is the silent channel (§2.3).
- `nonzero-status-is-not-speech` — every warrant is typed speech, never an exit status; the
  rc regimes of `311a` §6 stand (§1.10).
- `composite-identity-is-structure-not-a-bag` — at most one identifying scope; roles are a
  composite kind minted by the author who holds them (§2.9).
- `the-subject-includes-the-observer` — observer-dependence as the surviving half of the
  invariance line; two observers' cells share placement but not identity (§2.3, §2.6).
- `correspondence-is-known-only-to-the-transition-owner` — corresponds as a transition-owner
  generator; mapped lends are correspondences (§2.5).
- `identity-tokens-perish-on-write-not-only-on-rename` — a state mutation on an identifying
  scope perishes the tokens scoped in it (§3.3).

### § 6.2-conceptual-dead-ends-and-what-killed-each

Recorded as what-killed-it, so the dead end is not re-walked.

- IDENTITY AS A PER-KIND TABLE AGAINST AXES (the trichotomy invariant/keyed/⊤ per
  index-kind, `30W` §4; the filtered meet, `26Ob` §10b). Killed by: the kind owner cannot know
  the axes; silence walls forever and a guess ("keyed by Host") plus a referent-transparent
  Host yields a wrong DISJOINT on a shared volume. The truth: a cell's identity is a property
  of what its key denotes, not of the kind the key is written in.
- THE CONTEXT AS PART OF THE FACT KEY, argued as the root cause of the above. Killed by the
  review: a qualified key is an address of a question and asserts no partition; the
  dangerous inference lived in the meet's generators, not in the key's existence. Surviving
  form: the vantage is an address and a witness key (§1.9). This dissolved the A-versus-B
  binary of `311h`.
- STORE SETS, unioned and compared as bags (`311d` A1/A4; `311f`'s single `in:` for
  multi-store topics). Killed by base-and-overlay: same set, different roles, different
  answers. Also killed by the reflexivity defect of pairwise set equality. Surviving form:
  one identifying scope, composite kinds for roles (§2.9).
- "STORED-IN" AS ONE RELATION conflating routing and containment (`311d`; and the current
  design's keyed-by versus stored-in). Killed by hardlinks, bind mounts, NFS: namespace
  disjointness is not referent disjointness. Surviving form: named-in, identified-in with
  sole-route, and placement as three relations (§2.1–§2.3).
- TERMINAL TOKENS (`311f`'s `Measured(File, fsid:inode)`). Killed by NFS and by the scope
  question: an inode is a key in a filesystem, a filesystem identifier is a key in whatever
  minted it; the File owner should never learn NFS. Surviving form: rootness as an explicit
  dangerous claim; every token scoped (§1.4, §2.2).
- ONE GRADE ON A READ (`311f` B3's transparent/identifying). Killed by clones: equal machine-
  ids on two machines threaten SAME, which the disjointness grade cannot protect. Surviving
  form: two independent warrants per naming (§1.5).
- OBJECT IDENTITY CARRIES EVERY OBSERVATION ("one cell, one fact, whichever probe read it";
  `311f` B2). Killed by `test -w` under two users on one file. Surviving form: observer-
  dependence per kind, absent means dependent (§2.6); the invariance line's observer half
  lives.
- AUTHORED REGION PREDICATES (`kind__disjoint`, `30W:rul-disjoint-is-an-rc-predicate`; the
  `@subtree` selector idea). Killed by the symlink-retarget case: separation of two objects
  says nothing about whether changing one retargets a key for the other; and by the human's
  granule observation that the natural description unit is the atomically-disturbable one.
  Surviving form: hierarchical namespaces and traversals; containment as traversal
  membership (§2.8).
- IDENTITY BINDINGS BACKED BY THEIR TARGET OBJECT (`311d` A6). Killed by the same symlink
  case. Surviving form: resolution backed by traversal (§1.6).
- UNION TOTALITY WITH NO AUTHOR (`311d` A1's kind-level ∪ site-level sentinel). Killed by
  asking who closed the union. Surviving form: the owner declares and closes; sites fill
  instances (§2.1).
- CONTRADICTION-CHECKING AN INVARIANCE LINE WITH IDENTIFYING-GRADE INEQUALITY (`311d` A5).
  Killed by the grade itself. Surviving law: a grade governs every consumer (§2.2).
- SHARED ANCESTORS AS COLLISIONS (`311f` B2's cross-kind rule). Killed by the package status
  file and the unit file sharing a filesystem. Surviving form: ancestors are scopes;
  compare at the divergence (§3.2).
- THE DISCLOSED-WEAK NAME FLOOR (`300:rul-reference-entity-name-floor`; kept in `311d`).
  Killed by the human's default-safe lean and by its own per-kind carves. Surviving form:
  one-referent-one-name is typed, never assumed; the reviewer's window specimen also showed
  the floor was applied without its locality premise.
- MEASUREMENT MAKES DECLARATIONS REDUNDANT (`311h`'s framing). Killed by `311i` §0: a read
  establishes a token, not its scope, topic, warrant, applicability, or sufficiency.
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
  identifying scope, per-aspect placement, per-aspect observer-dependence). Surviving form:
  aspect-kinds (§1.8, §2.7). Recorded as forced, not chosen for tidiness.

## § 7-open-questions (carried into the next sitting; not rulings)

- `open-definitional-equality-default` — §1.5's one non-unknown default (equal natural keys
  in one instance across an unwalled span reach one referent) is inherited from `26Ob` §5's
  placeholder sharing; the human has not looked at it under this model. If it falls, nothing
  ever transports even within one vantage without a read.
- `open-injectivity-derivation` — the human asked for a narrow, stable derivation of
  one-referent-one-name that does not depend on an attentive author; none was found; the
  typed warrant on the read is the current answer, held as a dangerous corner.
- `open-no-transit-path-cost` — the static reconstruction of a no-transit probe path for
  observer-independent cells (§2.6) is priced at one declaration per kind; whether the
  stdlib will genuinely pay it for every kind, and what the access-refusal fallback looks
  like before reactive probing exists, is undesigned.
- `open-derivation-algebra-formalization` — §3.2's derivation meet (coherence, transitivity
  of SAME only) is stated, not specified; the per-property meet direction registry `28M` §8
  wanted is the same object.
- `open-composite-kind-authoring` — who mints role-bearing composite kinds in practice, and
  whether a tool author minting one per tool is the cargo-cult shape the boilerplate razor
  forbids.
- `open-aspect-kind-verbosity` — aspect-kinds are forced; their authored surface is not
  designed, and sugar is explicitly deferred to the UX pass.
- `open-observer-namespace-inventory` — which namespace kinds parameterize VALUES (User) as
  opposed to only routing keys; the model treats it per (kind, O) but the stdlib will want a
  short list.
- `open-context-slot-build-shape` — `plans/30W` item 1 builds the slot as a product over
  index-kinds; under this model the slot is a vantage (address and witness key) only, and
  the trichotomy meet must not be built; the as-built audit the human deferred decides how
  much of the existing slot is reusable.
- `open-security-and-hostile-host` — every read here is host-produced bytes crossing the
  intake boundary; nothing in this model widens what a host may mint (tokens are compared,
  never decoded), but the standing review gates apply before any of it becomes design.

## § 8-ledger (historical; pointers, not content)

The r31-prep sittings this document synthesizes, in order:

- `notes/311a` — the kernel-shape and rc-law sitting (2026-09-05/06): the context slot as a
  map over kinds; the rc regimes; `kind__overlaps` brought into the kernel and then re-ruled
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
- `Research/GOTCHAS.md` — minted 2026-09-08 from these sittings; twenty items at this
  writing; referenced by slug.

Human leans typed across the sittings and treated here as leans, never rulings: default safe
even when painful; no aliases or shortcuts for subsumed members; no perf-motivated holes;
dangerous operations rare and long-named; store as a flagged namespace rather than a
separate type; one relation with opt-in flags rather than three; aspects in scope once
forced; nearly everything in this corner open to re-litigation given internal coherence.

Design-of-record documents this model proposes to supersede in part once ruled: `plans/30W`
(§1 rootness, §2 generators, §4 trichotomy, §5 worlds), `notes/272` (the member and §5's
fence), `plans/30T` §6 (the per-aspect relation mapping and the v0 floor), `plans/30U` §7
(the store-collide consumer), `notes/26Ob` §10b (the filtered meet; the per-index relation
table), `plans/27C` §4 (the fallback lane's shape, not its measure-in-context default), `277`
§3 and `plans/30J` (the selector dialect), `plans/281`'s selector position. Everything on the
footprint side and the verdict side is untouched.
