# 311g — Review of System B's identity model

> AI-authored review of `311f` (System B), originally delivered in conversation before
> the source was moved into the repository. The proposal is comparative exploration,
> not accepted project direction. This review examines its core expressivity and
> allocation of knowledge between authors, not compatibility with existing code.
> Findings distinguish rules that are incorrect as written, underspecified contracts,
> and conservative losses of value. No implementation was inspected or executed.
> System A has not been read by this reviewer.

## §0 — Assessment

System B's valuable central move is to distinguish the object reached from the route
used to reach it. That accommodates shared volumes, aliases, and retargeting without
making host identity a universal partition of state.

Its weakest foundations are unscoped terminal measurements, unconditional separation
through storage, and the suggestion that identifying an object makes observation
context disposable. Its largest missing composition feature is a place for another
knowledgeable speaker to contribute an alternative identification or correspondence.
Several apparent simplifications leave substantive questions inside “mReferent,”
“store,” and “measured token.” These are not objections based on implementation churn.

## §1 — `issue-measured-tokens-escape-their-store`

**Assessment: a strong equality contract is missing or insufficiently distinguished
from ordinary local canonicalization.**

`311f` B2 makes equal `Measured` mTokens establish sameness unconditionally. B3 permits
an identity member to return answers ranging from a canonical package name to
`fsid:inode`. Those answers do not have uniform comparison store:

- a package name is meaningful within a package database;
- an inode number is meaningful within a filesystem;
- many filesystem identifiers are only locally meaningful;
- some apparently global identifiers are duplicated by cloning.

`Located` is the right representational ingredient for scoped answers, but B3 does not
clearly distinguish “I obtained a local canonical name” from “I obtained a mToken
sufficient to terminate the chain.” B5's direct `Measured(File, fsid:inode)` is exposed
to precisely that distinction.

B4 also misidentifies the cloneable-mToken failure direction. Different machines with
the same cloned machine-id threaten SAME. The `transparent` grade controls whether
DIFFERENT mTokens establish separation; it cannot protect the unconditional equality
rule. Wrong separation arises when one mReferent can produce different mTokens, not
from equal mTokens on distinct clones.

The generic File author should not have to know the store of every filesystem's
identifiers. The filesystem/provider author often knows better. Preserve that
contribution explicitly rather than hide it in a supposedly terminal mToken. This is
not a claim that trustworthy terminal mTokens cannot exist; it is a requirement to
state the strength and store of the contract admitting them.

## §2 — `issue-storage-is-not-necessarily-exclusive`

**Assessment: the unconditional implication from storage separation to subject
separation needs a stronger relation than ordinary physical storage.**

B0's “a row cannot live in two database files” is not a general foundation for B2's
`stores disjoint ⇒ mReferents disjoint`. SQLite's current database state can span its
main file and write-ahead log. A logical record can have several replicas. Overlay
copy-up can change physical placement while an application retains one logical name.
These examples concern potentially different choices of mReferent, and the model must
say which choice it requires rather than silently identify logical objects with
physical occurrences.

One repair is to choose an abstract Database or ReplicationGroup as the store, instead
of a File. That can be sensible, but the load-bearing claim then becomes exclusive
identity-bearing enclosure, not merely “state is physically stored here.” Not every
`lives-in` declaration warrants that stronger meaning.

There is a related representational omission: B discusses STORE SETS, but the `Located`
constructor has one `in: Ref`. The proposal does not specify how a subject spanning
several independently identified stores is represented, or how their comparisons
combine for sameness and separation.

The database/storage implementation author is often the competent speaker for WAL,
sharding, or replication. The account author should not have to survey that machinery.
The model needs to compose those contributions without interpreting every storage
edge as exclusive containment. Recursive enclosure itself is not refuted; the
unconditional interpretation of ordinary storage is.

## §3 — `issue-object-identity-does-not-identify-every-observation`

**Assessment: B2's dissolution of transport omits a necessary account of the full
subject of an observation.**

B2 says: “If two mSites' Refs `compare()` same, there is one mCell; whichever probe read it,
read it.” Consider `sudo -u alice test -w /shared/accounts.db` versus
`sudo -u root test -w /shared/accounts.db`. Both address the same filesystem object;
their answers can differ.

B could give this question a richer subject involving the file and observing
credentials. That is a possible solution, not an existing consequence of file identity.
The system must represent the combination and assign its parts to competent authors.
Listing additional physical stores does not obviously supply the missing qualification.

Distinguish identity of the object from identity of the complete subject of the claim
about it. These coincide for some claims, not all. Credentials, client-local caches,
mSort-relative visibility, and authorization mechanisms can contribute without
changing the object reached.

This review does NOT propose a special privilege algebra. User-related inputs should
use the same abstract machinery as other inputs. The concern is erasing contextual
information before showing how all relevant inputs became part of the claim. If each
mSort owner must recover every influence through `located_in`, responsibility may land
on the wrong author: a file author cannot reasonably know every mechanism affecting
a particular caller's access.

## §4 — `issue-common-ancestors-defeat-cross-sort-precision`

**Assessment: conservative value hole in the stated comparison rule, not unsafe
elision.**

B2 permits cross-mSort disjointness only if the total store chains share no
same-or-unknown store pair. Consider:

- Package nginx → package-status File → Filesystem F;
- Service web → unit-state File → Filesystem F.

The relevant files may be demonstrably disjoint, but the full chains share Filesystem
F. The written rule therefore blocks separation. An unrelated file write can similarly
collide with persistent state rooted in that filesystem despite complete authorship.

An ancestor establishing where an object is located is not necessarily the extent of
state being protected. Flattening an identity/location chain into collision evidence
loses this distinction. The proposal needs a rule for discharging comparison at a
sufficiently precise level. Its region predicate might help, but B does not specify
how that result satisfies or overrides the all-ancestor test.

The terminal case also needs specification: “no further stores listed” must not
vacuously prove two arbitrary terminal mSorts disjoint. This is a question for the
formalized rule, not a claim about an implementation that does not yet exist.

## §5 — `issue-knowledge-can-live-outside-the-sort-owner`

**Assessment: an important missing contribution surface, with a clearly better-informed
speaker available.**

One process can be PID 1 inside a container and PID 4821 outside it. The generic Process
oracle, executing inside the container, may lack the information or privileges to
identify the outer process. The container manager's author can know the correspondence
from its control interface.

B assigns identity production to one mSort-owner member running in the denoted mWorld,
and wrapper authorship to lending routing indexes. Where can the container author
contribute the concrete correspondence between these two process names?

The index declaration alone does not provide it. Namespace disjointness cannot
separate the processes either: they are the same process seen through different PID
indexes. Requiring the Process owner to learn every container manager's semantics
would put the responsibility on the wrong speaker.

The model could be extended, or explicit helper composition could carry this knowledge.
As written, that contribution has no clear primitive or contract. This is not a proof
that the knowledge is unrepresentable under every extension. It identifies a choice
between losing available value and centralizing implementation-specific knowledge in
the generic mSort owner—precisely the epistemic seam the review was asked to examine.

## §6 — `issue-alternative-identities-are-not-represented`

**Assessment: expressive gap for mixed-provider and optional-identifier cases.**

The `Ref` sum chooses among `Located`, `Measured`, and `Route`; it does not express
that one subject has an ordinary scoped identification AND a provider-specific
identification. B states no comparison for `Located` against `Measured`, or rules for
reconciling several descriptions of one subject.

This matters when one provider supplies a stronger identifier and another only the
ordinary enclosure, or both descriptions exist and corroborate or contradict one
another. Choosing one representation loses information. Mixing untagged mToken schemes
in the single identity member can be worse: two encodings of the same object could be
mistaken for disjoint identifiers.

Repair is plausible, but it must retain dependencies and validity conditions rather
than merely collect additional strings or establish a provider priority.

## §7 — What the model handles well, and review limits

With suitable identifier contracts and the qualifications above, B handles several
important cases naturally:

- **Red versus Blue:** complete store qualification plus author-warranted local-name
  distinctions prevents the original mistaken account merge.
- **Shared volumes:** host identity need not partition the referenced state.
- **Remounts and symlink changes:** making name resolution itself an invalidatable fact
  is the right shape.
- **Partial knowledge:** withholding unsupported positive conclusions avoids guessing.
- **Incarnation changes:** relevant identity-bearing state can invalidate old conclusions
  without the engine computing successor state.

The shorthand “chroot swaps the mount table” is technically imprecise: root directory,
mount namespace, and other path-resolution inputs differ. That alone does not refute
the abstract model; its arbitrary mSort mReferents could represent the corrected
inputs. The findings above concern stronger issues than such mechanical shorthand.

No claim here depends on preserving the current implementation or the conversation's
other candidate designs. The source's routing/object distinction is worth retaining
as an exploratory insight even if the representation and authoring contracts change.
