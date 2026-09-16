# 311e — Local review of System A's store-pointer refinement

> AI-authored review of `311d` (System A). This reviews that proposal on its own terms,
> not comparatively against another proposal and not as a recommendation to adopt the
> conversation's candidate design. Focus: the comparison algebra, expressive adequacy,
> and whether each required assertion belongs to a human equipped to make it.
> No implementation was inspected or executed. Distinguish defects in the written
> rules from missing contracts, conservative value losses, and inherited acknowledged
> risks. The examples below are reasoning specimens, not empirical test results.

## §0 — Overall assessment

The strongest move is mSite-specific store qualification: the tool oracle that actually
parses `--db` supplies the database instance, while a different mSort-owner answers the
file's identity question. Keeping the account name alongside that pointer avoids
collapsing every account merely because the accounts share a database. Making identity
answers perishable is also necessary and well motivated.

The unresolved core is WHAT a store pointer promises. The document moves between
physical storage, identity-determining inputs, and the dependencies of a mResolution.
Those are not interchangeable merely because they are represented as coordinates.
A union of pointers does not, by itself, retain the way their meanings compose.

The most concrete defects are the stated equality test for multiple stores and the
use of unequal IDENTIFYING mTokens to refute invariance. The most consequential design
holes concern role-sensitive store composition, completion authority across authors,
and the lifetime of name-mResolution relationships rather than just their result objects.

## §1 — `issue-multiple-store-equality-is-not-reflexive-as-written`

**Assessment: concrete defect or consequential ambiguity in A4's rule.**

A4 defines store sameness as equal cardinality and “every pointer pair same.” Under
its literal Cartesian-product reading, a complete two-store set does not `compare()` same
with itself. Let `S = {database-file, policy-file}`, with the two files known disjoint.
The cross pairs are disjoint, so `stores(S, S)` is not same. A mCell with perfectly
understood multiple stores loses even reflexive identity through that branch.

If “pointer pair” means some intended matching instead, that matching is not specified.
It cannot be supplied by vector order without making semantic equality depend on
emission order. Cardinality is also insufficiently grounded: two distinct coordinate
spellings in one set may resolve to one underlying store, while the other set names
that store once.

This is initially conservative value loss, not a wrong elision. Nevertheless it affects
exactly the multiple-pointer feature the proposal adds and must be resolved before the
comparison rule has a determinate meaning.

## §2 — `issue-store-sets-lose-the-roles-of-their-members`

**Assessment: repairing set equality does not, by itself, repair the identity model.**

Consider an ordinary layered-configuration interface:
`cfg --base A --overlay B get enabled` versus
`cfg --base B --overlay A get enabled`.
The source named as overlay wins conflicting values. Both questions concern the same
local setting name and read exactly the same two files. Their complete storage/input
SET is `{A, B}` in both cases; their answers can differ.

An unordered set knows that both files participate. It does not know which is the
base and which is the override. Site-specific pointers put the correct instances into
the set, but union removes the relationship between each instance and its role.

An mSort can avoid this by documenting a richer mKey name, introducing role-specific
parent mSorts, or otherwise preserving the composition recipe. Those are possible
repairs, not consequences of the current set rule. Encoding raw path spellings into
the mKey name also risks losing the cross-path identity value the pointer recursion
was supposed to recover.

If A instead defines authoring `stored-in` as explicitly promising that the UNORDERED
set, together with the canonical local name, is sufficient, these examples are outside
that stronger contract. That is a coherent restriction, but it must be stated and
priced: it is materially stronger than complete enumeration of storage locations.
The tool author knows the base/overlay roles and is the right person to supply them;
the two File owners cannot recover those roles by measuring file identity.

This is a general epistemic seam: individually correct descriptions of all inputs do
not establish the meaning of their composition.

## §3 — `issue-local-injectivity-is-used-without-its-locality-premise`

**Assessment: A4 needs a stronger stated theorem or a narrower first branch.**

A2 describes injectivity WITHIN one store. A4's first positive branch concludes
`different names + injective + total ⇒ disjoint` without consulting the relationship
between the stores.

This can be justified if every mReferent has a unique complete identity-bearing
mKey-PrimaryStore: if two references denoted the same thing, their mKey-PrimaryStores would necessarily be the
same, and local injectivity would then forbid different canonical names. In that
stronger model the missing explicit `s = same` check is not automatically a bug.

But A does not establish that unique-mKey-PrimaryStore premise. Ordinary regions or views can
contain the same state while assigning it different local names. For example, two
non-aliasing windows into an underlying block range can each have injective local
block numbers, while local block 100 in one is local block 0 in the other. Different
window identities do not settle separation of their contents. Each window is a complete
place to describe that block's state, not a unique exclusive enclosure of it.

The distinction between logical block occurrences and shared physical state must also
be explicit: choosing separate logical subjects does not alone make their writes
noninterfering. The dependent-state comparison still has to account for their backing.

The proposed disjoint-store branch has the related obligation: “all stores listed”
is a completeness claim, not necessarily exclusive ownership of state. If A intends
stores to be unique identity-bearing enclosures, say so and provide a place for ordinary
coarse/shared storage descriptions that cannot make that stronger assertion.

This concern is separate from A's openly acknowledged default-injectivity debt. Even
an explicit TRUE local-injectivity declaration does not supply an unspecified global
mKey-PrimaryStore theorem.

## §4 — `issue-totality-has-no-defined-author-over-the-union`

**Assessment: an incomplete authority/composition contract.**

A1 forms a mCell's store set by unioning KIND-level pointers and SITE-level pointers,
then uses `stored nothing-else` to authorize positive comparisons. It does not specify
which author closes which set:

- Does the mSort-owner close only their own output, or the eventual union with mSite
  pointers authored in other files?
- Can a tool's reached mSite-level sentinel close the mSort-owner's output?
- If both close their own contributions, who has promised that the two contributions
  collectively contain every identity-relevant input?
- How do verdict-side and disturbance-side descriptions of nominally the same mKey
  retain the same interpretation without implicitly composing their authors' judgments?

None is answered by set union alone. A generic mSort-owner may not know the invocation
shape or provider-specific input; the tool oracle knows the argv but may not know the
mSort's implementation-independent semantics. Explicit delegation can make such
collaboration legitimate, but its boundary is the missing contract, not something a
completion sentinel supplies automatically.

A1 also says `stored-in` normally belongs within `reads`, with a detector nudge when it
does not. That suggests useful shared information, but does not make a read-dependency
annotation an identity-sufficiency claim. The latter grants power; the former can remain
an honest conservative disclosure. Requiring more description before permitting a
comparison is sensible. Reinterpreting an existing weak description as that permission
would not be.

The physical execution witness is another independent requirement: successful record
arrival cannot decide whose semantic completeness promise covered the union.

## §5 — `issue-token-inequality-does-not-refute-identifying-equality`

**Assessment: concrete incorrect inference in A5.**

A2 deliberately permits an IDENTIFYING `resolve()`: equal mTokens prove sameness, unequal
mTokens say nothing. Such a reader can legally return different route-qualified handles
for one shared object. For example, two mounts can provide distinct identifying handles
for the same file; equality of either complete handle remains a valid identification
within its warranted mKey-PrimaryStore.

A5 nevertheless treats differing mTokens as a contradiction of an invariance statement
and derives keying from unequal store mTokens. For an identifying-only reader, inequality
is not a refutation of mReferent equality. A true invariance line and two unequal mTokens
can coexist without any author being wrong.

The grade must govern the corroboration path as well as the primary comparison path.
A contradiction requires an actual warranted separation result about the same subjects
and circumstances, not raw mToken inequality. Otherwise the supposedly defensive check
wrongly withdraws valid authority and directs the user toward a false disagreement.

Separately, invariance of mCell identity and identity of every physical backing store
are not automatically the same claim. A5 redefines the old line in terms of stores.
That may be an intended new contract, but it needs an explicit semantic argument rather
than assuming a measurement is simply the old declaration made empirical.

## §6 — `issue-terminal-reads-concentrate-comparison-scope-responsibility`

**Assessment: acknowledged risk whose proposed allocation still needs scrutiny.**

A does explicitly require cross-host-comparable mTokens and allows decline. That is an
important safeguard in its contract, and the review should not pretend the requirement
is absent. However, A2's File example places filesystem identity construction inside
the generic File identity body, including NFS-specific knowledge. A4 makes possession
of an `resolve()` the recursion's base case.

The available knowledgeable speaker for an identifier's mKey-PrimaryStore may be the filesystem or
storage-provider owner, not the File owner. If a File `resolve()` returns a local object key,
there must be a way for that mKey to remain qualified by its owning filesystem rather
than either becoming a terminal global mToken or forcing the File author to assemble
one. “There is an identity member” is not itself evidence that its answer can replace
all store qualification.

The precedence also matters for an mSort with both a `resolve()` and useful stores:
if the `resolve()` declines, does A4 return unknown immediately, or attempt the store recursion?
The written base case appears to choose the former. That loses value the other
information may still support. If both routes answer, reconciliation is unspecified;
resolve-then-identity is a sequence, not a model of alternative witnesses.

Cloning illustrates the distinct grades precisely: equal cloned UUIDs threaten SAME.
Multiple different identifiers for one shared object threaten DISJOINT if the `resolve()` was
wrongly declared transparent. A horizon sentence must describe which promise is really
being made; it is not a substitute for the appropriate knowledgeable contributor.

## §7 — `issue-result-object-is-not-the-whole-resolution-dependency`

**Assessment: the coarse intent is sound, but A6's finer invalidation claim is not yet
supported by the stated backing model.**

A6 backs an identity binding for coordinate P with `{P@w}`. Resolving a path can depend
on directory entries, symlinks, mount bindings, and other interpretation inputs, not
merely on the final object denoted by P.

For example, `/config/current/app.db` may resolve through a symlink to an object under
`/volumes/team`. Replacing the symlink can change what the book's next access reaches
without mutating the previously identified database object. A correct object-separation
answer between the symlink entry and the database does not prove that the replacement
spares the mResolution of the book's name.

The all-MountNamespace coarse dependency may conservatively contain this if its meaning
covers all relevant name-mResolution state and the collision path is actually consumed.
But relaxing to region footprints needs the dependency of mResolution on the affected
region, not just region separation from the final inode. The File/mSort describer
can know the traversal; the `mount` or `ln` author should not have to enumerate every
future name elsewhere that happens to route through it.

If `kind__disjoint` is intended to answer this resolution-dependency question too, that
is a distinct consumer contract that must be explicit. A true statement that two objects
are separate cannot be reused as a claim that changing one cannot retarget a name for
the other.

A4 also says cross-mSort comparison is unchanged, while A6 relies on mKey-CatalogStore writes
colliding with File identity dependencies. Existing store-collision machinery may
provide that path, so this is not by itself a contradiction. The path and its transitive
consumption must nevertheless be stated; storing an mKey-CatalogStore pointer in a side record
would not make `{P@w}` sufficient automatically.

## §8 — `issue-storage-equality-does-not-close-observation-context`

**Assessment: A9's re-opening of the transport argument is not discharged merely by
measuring the pointers instead of declaring invariance.**

`sudo -u alice test -w /shared/accounts.db` and
`sudo -u root test -w /shared/accounts.db` address one object and can return different
answers. Measuring the file identity more accurately does not resolve that difference.
The proposal must identify the complete subject of the protected claim, including
whatever observer-related state is relevant.

A4's universal transport requirement over backing members is useful. If all relevant
inputs are represented there and compared under the appropriate semantics, this class
can be handled. But neither matching physical stores nor a detector for some unmarked
reads proves that all such inputs are represented. A1's identity-determining pointer
could be a stronger positive speech act, but then its mKey-PrimaryStore and the author's competence
to warrant it must be explained. Physical-storage completeness alone is insufficient.

This is not a request for special authority machinery. User, mSort, and other
contextual contributions can use the same algebra. The review concern is loss of
those contributions when the per-index reasoning is replaced, and assigning all of
them implicitly to an mSort-owner who may not know the surrounding tools or policies.

## §9 — `issue-the-demonstration-relies-on-a-contradiction-canary`

**Assessment: A8's bad-bind recovery is a property of that specimen, not validation
of the identity rule.**

A8 says a bare `7` bind is rescued because the two mSites independently produce contrary
measurements, which meet to unknown. That recovery requires both independent answers
to exist and be consumed; it cannot be the general safety argument for an identity
conclusion that may itself justify omitting redundant measurement. Nor does absence
of a disagreement prove identity correct.

The record should also not imply that a disagreement uniquely identifies the bad bind.
A wrong `resolve()`, mistaken siting, faulty check, or wrong binding can produce
conflicting observations. The why-chain can expose the relevant collaborating lines;
without further evidence it cannot diagnose the bind as the sole culprit.

The properly qualified red/blue specimen remains a good positive illustration of A:
retaining the local account name AND the database instance avoids equating every row
of a shared database.

## §10 — Positive results and scope of this review

A gives useful explicit places to express:

- an argv-selected database instance without forcing the mSort-owner to know the book;
- the difference between local account names within a shared store;
- cross-host store identity without relying on host equality;
- conservative loss of authority when identity answers expire or decline;
- mSite-local provenance for an identity-determining declaration.

The review is not a rejection of measured store-pointer recursion. Its main requirement
is to make explicit the claims currently hidden in the set/comparison operations:
which roles the pointers occupy, who closes their combined meaning, when a mKey-PrimaryStore is
exclusive, and which dependencies make a measured relationship valid.

The default name floor and the added per-mSite repetition are already acknowledged
costs in A10. They should be priced, but they are not the novel findings of this review.
A7's statement about re-witnessing after every transit also needs reconciliation with
later witness-policy decisions before implementation; that interface issue is not
central to the abstract findings above.

No code defect, performance measurement, or final project direction is asserted here.
