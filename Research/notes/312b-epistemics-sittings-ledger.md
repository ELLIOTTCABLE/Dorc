# 312b — Epistemics and assignment: the sittings ledger

> AI-authored (Fable, the `r31-prep-design-duck` sittings, from 2026-09-08). Notes-tier LIVING
> ledger for the phase that follows `notes/311`: sitting-by-sitting minutiae, experimental
> findings, and the takeaways extracted from `notes/312b-exercises/` (one record per exercise,
> not durable). The corpus import is FROZEN as `notes/312a`; the bare `312` is reserved for the
> phase's synthesis. Nothing here is ruled unless it cites a ruling by `docID:slug`; grades are
> +SURE / ~SUSPECT / -GUESS / --WONDER; **[HUMAN]** marks the human's framing, paraphrased from
> chat, never a ruling. Authority: root docs, `spike/CLAUDE.md`, the welds, `notes/311`, and
> `notes/312a` § 0 (the phase's conduct as typed) outrank this.

## § 1-typed-this-phase

Conduct (2026-09-08): plain book-sh grounds a case, never Dorc-spelled sh; the model's
response is prose in 311 vocabulary; seat spellings that fall out carry no weight; one record
per exercise in `312b-exercises/`; `GOTCHAS.md` authored freely now, chopped later into a
frontier and an extras file; the singular Fable-subagent authorization is the conductor's;
prior typed seat assignments (`312a` § 3) are re-verified on the 311 base, never carried as
bindings; short chat responses, nothing implied as ack.

Positions (2026-09-09): known-but-deferred effects stay in scope, gently, never labelled
horizon; unexpected churn (other users, host-owned cron, host-configured reactions) is
horizon, the price of play.

Conduct (2026-09-09): the sitting leans toward epistemics and the earliest shapes of UX, not
specific holes, while the clay is wet; exercises stay light and agile, a handful of
observations, not a page; each exercise builds in glossed STAGES like USER_STORY, usually two
or three, so the liminal middle of the enhancement curve is explored rather than only its ends.

## § 2-exercise-takeaways

From `312b-exercises/01` (placement parametric in an observer: pipx, `sudo git config
--global`, nvm):

- `lead-computed-namespace-instance` — a mNamespaceInstance or mPlacement can be a function of
  ρ and of another mKind's `resolve()` (`${PIPX_HOME:-$HOME/.local/pipx}`); `311` § 2.1's three supply
  modes have no room for it. A fourth, COMPUTED, of which SITE and AMBIENT may be special
  cases; unknown inputs make the instance unknown.
- `lead-ask-the-tool-for-its-store` — measure-in-context as the cheap rung: the tool prints
  its own store; the declared rule is the experienced rung.
- `lead-env-is-a-routing-namespace` — ρ routes command words, stores, and referents; `30S`
  covers pins and severs for verdict bodies only.
- `lead-local-route-perishes-on-rho-writes` — the engine's transit-free mRoute claim for
  command words is keyed by `PATH` and cwd; a `PATH` write is a routing mutation.
- `lead-policy-dependent-lends-split-two-ways` — wrapper author declares the default; the
  admin overrides; the oracle may read the policy and decline.

From `312b-exercises/02` (the wrapper seat: the crontab pair, sudoers by guest, wrapper order
and env scrubbing, `$SUDO` as a variable):

- `lead-lends-may-depend-on-the-guest` — sudoers matches the guest command, so the entered
  context can differ between the probe's entry and the apply's (`27Xf`); the wrapper author
  declares guest-insensitive by default and supplies a policy read that declines.
- `lead-policy-reads-decline-on-surprise` — the general move for policy-dependent tools and
  wrappers (sudoers, APT hooks, `env_keep`): read the policy in the denoted context, decline on
  departure from the declared default.
- `lead-context-sets-at-a-site` — a two-valued wrapper (`$SUDO`) gives a mSite a set of
  mEntryChains; `compare()` quantifies universally over the set; a fact whose mKey does not pass through
  User survives either way, one that does is unknown.
- `lead-narrowing-by-test-is-the-capture` — `FORFEITS:forfeit-value-narrowing-by-test` is what
  turns `$SUDO` from ⊤ into a two-element set; the idiom is dominant, so its priority rises.

From `312b-exercises/03` (store identity across hosts: NFS on two hosts, a socket on a shared
mount, the host key):

- `lead-identify-in-the-narrowest-store` — a mKind identifies in the narrowest mNamespaceInstance
  its state lives in, never a coarser one assumed to partition it; a file-backed mKind
  identified in Host is the highest-leverage wrong DISJOINT and a lint-able smell. The law
  routes the NFS question to the filesystem owner and the admin structurally.
- `lead-mount-lines-generate-correspondences` — the transition owner for a mount is the mount
  oracle; the mCorrespondence derives from the argv the admin wrote.
- `lead-cross-host-same-bottoms-out-in-the-admin` — every cross-host SAME chain ends at host
  sameness, the admin's seat by typed ruling; the model needs that seat.
- `lead-reads-decline-outside-their-ontology` — a mKind owner's `resolve()` declines on referents its
  mKind does not describe (File on a socket); the mechanical net for lazy coordinate borrowing.
- `lead-warrantless-tokens-are-witness-only` — endpoint witnesses (host keys, stamps) never
  license.

From `312b-exercises/04` (`a-host-is-not-a-partition`: one NFS export under two targets,
with and without a declared root):

- `hole-route-versus-root-reads-unspoken` (+SURE of the text) — `311` § 3.2 makes a
  mRoute-terminated chain against a rooted one unspoken, which a finished definition can spend;
  a chain with fewer measured links compares more decisively than the same chain complete.
  Latent today (no roots declared); live on the first true root.
- `hole-different-roots-read-unspoken` (+SURE of the text) — two mRoots are two mWorlds; the
  finished definition is a within-mWorld sentence; spending it across mWorlds names the wrong
  author in the why chain. Both holes reintroduce `a-host-is-not-a-partition` against
  `26Ob:ack-cross-world-wall-is-the-floor` (TYPED), which 311 otherwise reproduces exactly at
  the no-root floor.
- `hole-sole-route-orientation` (~SUSPECT; ambiguous text) — § 3.2's "every level above it"
  must mean every level between the divergence and the leaf, per § 2.2's definition; under
  either reading the NFS case rests on the File or Filesystem owner withholding :sole-route for
  network mounts, with no net for one who did not think of it.
- `law-partial-measurement-never-widens` (owed) — a mDerivation with an unmeasured or
  mRoute-terminated link yields at most what it yields measured.
- `lead-rootness-is-a-stdlib-only-seat` — a mRoot's blast radius is every mKind identifying into
  it, retroactively; declaring one is the highest-leverage act in the identity model and
  belongs with countability's few.
- `lead-cross-target-disjointness-is-the-admins-sole-route` — "these targets share no store"
  is :sole-route at the Target mScope, the admin's seat; it is the reserved posture option, and
  no rule change replaces it.
- `nit-same-is-or-across-derivations` — the mKey walk is "and" over levels; SAME overall is
  "or" across mDerivations (mKey walk; mCorrespondence; provider identifier); worth stating
  plainly in 311 § 3.2, where only the key-walk clause is written.

Cross-cutting, seen in all four: the admin seat is missing and every exercise hit it
(overrides; sudoers; host pairing; the cross-target posture). Convert-to-read recurs as the
newbie rung. Policy dependence recurs as the shape that splits a claim between an author's
default and an admin's deployment. A `resolve()` that declines outside its mKind's ontology recurs as
the mechanical net for lazy borrowing.

## § 3-time-sitting-banked

One sitting (2026-09-08/09), rewound over by the human; the conclusions, not the argument.

Frame (**[HUMAN]**): Dorc's value is not figuring out time; it is modeling precisely what the
admin already wrote to express time, so Dorc does not diverge from that intent. Every "time"
category examined landed in one of three bins: horizon; someday-punt; or bog-standard static
sh-analysis coverage that is engine work, never ops-world-state-model work. No new soundness
class was found.

- `time-logical-owed` — failure paths (`trap … EXIT`, errexit abort edges); the concurrent
  partial order (`&` to `wait`, pipeline stages); `exec` and detach within the book's extent;
  loops that establish their condition and bound their executions; containment (built); a
  cron or timer the book installs, treated as firing immediately and reaching to end of book.
  Arbitrarily complex static analysis, owed.
- `time-wall-knowable-punted` — a book-installed schedule firing during the apply (schedule
  text plus a remote clock read plus apply duration; unplaceable at a program point, hence the
  conservative reach above); one firing after disconnect (harmless to this apply under
  statelessness, the next probe re-measures; an aid hint at most); predicting async tool
  internals instead of waiting.
- `time-bridge-in-scope-gently` — return-is-not-done: `systemctl restart` before ready, a HUP,
  deferred triggers, DNS under TTL, replication lag. Wall by nature, logical by an authored
  `witness()`: one declaration bit (asynchronous with respect to a mCell) makes the footprint reach
  forward to the next wait on that mCell or to end of book; the tool author's wait in the
  verdict body or the admin's `until` and `sleep` closes it. No clock.
- `time-teaching-rules` — eventual consistency is :reaches, never :corresponds (SAME for
  collision is safe, SAME for transport is not); async tools put the wait in the verdict body;
  the admin's wait loop is first-class and `StatusIterated` already protects it; an
  unexplained `sleep` is a hint that a footprint omits a deferred consequence.
- `claim-guards-inherit-the-bare-book-exposure` (~SUSPECT; unattacked) — a guard and the
  command it fronts share a program point, so a deferred effect lands after both; the guarded
  book and the bare book reach the same state. If it holds, USER_STORY's "no staleness by
  construction" is overstated, not broken: no staleness relative to landed effects, the bare
  book's exposure otherwise (`236b` F1's July downgrade, never carried into the doctrine
  text). TOCTOU exposure is then a function of elision alone, bounded by walls and by plan
  age. Attack this before leaning on it.
- `time-levers-unweighted` — plan age as an aid fact at apply; a verify posture (every elision
  a guard) as the admin's TOCTOU dial, nearly free; apply-standup freshness demotion
  (review-gated, `rul-repeated-probing-reviewed-before-design`); MH2's version window,
  deferred by choice, tractable in shape.

## § 4-naming-sitting-banked

One sitting (2026-09-10), interleaved with exercise 04's rabbit hole; conventions, each typed
unless marked lean. The rename procedure lives outside git, in a root `_tmp-` file.

Conventions, typed: every object the model defines is written mFixedTerm (mKey, mKind,
mCell), bare, plural by appending s; every relation, attribute, or warrant declared on the
model is written :fixed-term, always preceded by a space; every abstract operation is written
`op()` in backticks (`compare()`, `resolve()`); every concrete authored member keeps the
`__name()` form; referent and value are never tagged (the world and sh define them; Dorc only
mirrors them, though the lifts into Dorc are real players); no umbrella term for "things the
engine handles" is ever minted; an in-Dorc mKind is always written with its prefix (`sm.File`,
never File); RDBMS words appear only after an m once adopted or inside a parenthetical; "warrant"
is a flavour of attribute and is said to sit on a named edge type.

Acks typed: mKind as one naming scheme with one owner, with "multiple mPrimaryKeys per referent
across mKinds" as the stated consequence and the correctness of world-as-registry explicitly reserved;
an mKey is a value plus its mKind plus the index it was looked up in (partial ack: a coordinate
`sm.Package:nginx` is what an author writes, not yet an mKey); one natural-key scheme per
mKind, open as ergonomics until this sitting, now leaning forced (§ 5).

## § 5-identity-model-findings-from-exercise-04

Findings from the rabbit hole under `312b-exercises/04`, superseding in part the 04 block in
§ 2; nothing ruled; grades on the conductor's claims.

- `fnd-the-store-mints-the-primary-key` (+SURE by definition) — the mPrimaryKey is the address
  the store answers with, so the parent named by :identified-in and the placement on the
  identifying chain are one object, the store holding the referent. Placement remains a set:
  that store plus the non-identifying placements.
- `fnd-the-identifying-parent-is-per-key` — for a mKind whose store can be local or remote
  (`sm.File`; the netns sysctls) the parent store's mKind varies per mKey; the owner declares a
  menu of admissible parent mKinds (`sm.LocalFilesystem`, `sm.NetworkExport`,
  `sm.ClusterFilesystem`, decline) and `resolve()` picks per mKey, returning the mPrimaryKey with
  the parent's mKey; each admissible parent carries its own :sole-route answer. 311 § 2.10's
  one identifying parent per mKind becomes one parent per mKey, one menu per mKind. The lazy resolve (statfs
  fsid in Boot) is true and unsafe; under the menu the lazy resolve has no arm and declines.
- `fnd-bound-the-finished-definition-by-stores` (ACKED) — "nothing else" means "nothing outside
  my declared stores and my reached mKinds' stores"; the engine compares stores by ordinary
  `compare()`; overlap or undeclared store ⇒ collide. Strictly narrower than today's sparing;
  handles blind parallel mKinds and cross-mWorld pairs by one rule; USER_STORY stages 5 and 7
  survive it unchanged (checked). Consequence: store declarations become knife-tier, the same
  tier as the finished definition they now constrain; `stored nothing-else` is the totality act.
  --WONDER: part of `:reaches` derives from store overlap (a package's placement includes its
  unit file), not all of it (postinst enabling a unit touches boot state no file lists).
- `fnd-delete-the-route-and-root-clauses` — 311 § 3.2's route-versus-root and roots-differ
  clauses read unspoken, spendable by a within-mWorld finished definition; both must read
  unknown, and with per-mKey parents cross-mWorld comparison runs through the ordinary mKey
  walk. `26Ob:ack-cross-world-wall-is-the-floor` is reproduced at the no-root floor. The
  clause was buying pivot-book cross-mWorld sparing; that value returns through the menu
  (local-exclusive parents carry :sole-route) and through the admin's cross-target posture,
  never through the finished definition.
- `law-partial-measurement-never-widens` (owed) — a mDerivation with an unmeasured or
  mRoute-terminated link yields at most what it yields measured.
- `fnd-sole-route-is-provable-by-constitution-or-contract` — never for views. A process in its
  kernel, a package in its dpkg file, an inode in its ext4 table are :sole-route by what they
  are; ext4 in its boot by the filesystem's own contract (concurrent mounting unsupported); a
  client NFS mount, an NSS view of LDAP, a chroot's view of a bind mount are views and get no
  :sole-route. Parent classification and :sole-route are one declaration. Clones are
  :guarantees-unique-referent failures on a token, never :sole-route failures. 311 § 2.2's
  hardlink example is a :guarantees-unique-name failure on paths, handled by `resolve()`; a
  one-word fix owed.
- `fnd-warrants-sit-on-edge-types` — :guarantees-unique-referent and :guarantees-unique-name on
  a :named-in edge type (a kind's natural keys into an index kind), declared by the child kind's
  owner; :sole-route on an :identified-in edge type (a kind into its parent kind). Location of
  :sole-route is contested: 311 puts it on the child; the person who knows whether the parent is
  a store or a view is the parent's owner; exercise 04 leaned parent.
- `fnd-directionality` — :named-in runs inward from a value to a store (the lift; `resolve()`
  performs it and is the only relation with a value on its left); :identified-in and :lives-in
  run outward from an mKey to other mKinds' mKeys; one `resolve()` per mKind returns the
  mPrimaryKey with its store's mKey, and the chain is built by calling each mKind's `resolve()`
  in turn. No separate parent-lookup operation exists.
- `fnd-placement-is-an-edge-set` — :lives-in is an edge type from an mKind (a template the site
  or environment fills) to mKeys of other mKinds, many-valued because state is spread across
  stores; exactly one member sits on the identifying chain; the rest are non-identifying
  placements, followed for interference only.
- `fnd-index-and-store-are-two-roles` — an mKey of another kind can play two roles for my
  keys: the index (`:named-in`'s far end; where a natural key is looked up: mount table, dpkg
  database, resolver from a vantage) and the store (`:identified-in`'s far end). Both are
  ordinary mKeys, never a node type of their own. The index is implicit (ambient, from the
  vantage and entry chain) and is part of the mKey: chroot is the example (`sm.File:/etc/passwd`
  inside and outside `chroot /mnt`, one value, two indexes, two inodes). Crontab is NOT that
  example: its natural key is the ambient user, supplied by the entry chain, so under sudo the
  VALUE differs (`sm.Crontab:root` versus `sm.Crontab:alice`).
- `fnd-a-referent-has-many-primary-keys-across-kinds` — one per mKind, one mKind per referent is
  unenforceable without a registry; the model detects (store overlap collides) and equates
  (`resolve()` into one store gives SAME); Dorc equates mKeys, never merges mKinds
  (`24M:rul24M-kind-unify-owed` stays a human act). Coherence of the conceptual merged thing is
  by meet: placements union, `:reaches` union, vouches stay per author; a granting composite
  never arises. UX obligation, not a model hole: the merged thing is never presented as one.
- `fnd-store-is-the-registry` (**[HUMAN]** naming acked, correctness reserved) — every store
  is the registry for its own referents; what lacks a registry is which mKinds describe which
  referents; the contract asks each author to point at the physical minting authority via
  `resolve()` and declare placements, never to coordinate with strangers; the community
  negotiates precision only, through the hint "two mKinds share a store".
- `fnd-shared-natural-key-class-is-a-canary` — two mKinds sharing a natural-key class cannot
  corrupt each other at the lookup (custody: a mKind's mKeys go only through its own `resolve()`);
  they meet only in the store. Same string, same index, same mVantage, resolved by two mKinds to
  different mKeys of a :guarantees-unique-referent store ⇒ at most one is right ⇒ withhold both
  and narrate (`28M:rul-conflict-between-totals-is-falsification`). Not named in 311.
- `fnd-aspects-are-inhabited` — an aspect-kind (`sm.ServiceActive:nginx`) has a natural key
  borrowed through :named-like, a referent, its own store, and a value; only the selector
  position vanished. The `@` sugar (`sm.Service:"nginx"@sm.Active`) is the same graph iff the
  aspect keeps its own store, and is a good surface-syntax candidate; it must expand to a
  per-parent aspect-kind and never let one aspect name span parents (the pgbouncer case).
- `fnd-one-natural-key-scheme-per-kind-is-forced` — a bind is the lift from a value to an mKey
  and must know which lookup to run; N schemes per mKind would put a scheme selector on every
  bind, which is a second mKind by another name; users by name and by uid share one index and
  are disambiguable only by two mKinds resolving into one store. Minting a mKind is the honest
  spelling of a second naming system.
- `fnd-layers` — referent (the world) · value (sh strings; lifted from the world by any read)
  · mKey (lifted from a value by a bind or a `resolve()`); mKinds are the type layer of mKeys,
  not a fourth level. :guarantees-unique-name is injective (no aliases), :guarantees-unique-
  referent is functional; both together are one-to-one.

## § 7-shape-sitting-banked

One sitting (2026-09-10), the second half of the naming turn; the comparison between 311 as
written ("A") and the human's alternative ("B(b)"). Nothing ruled; the human is dissatisfied
with A and leaning B(b); the second node species' NAME is owed (candidates live in the root
`_tmp-` file, never here). Written in current corpus vocabulary; where B(b) needs a word the
corpus lacks, it is described, not named.

The two shapes:

- A (311 as written): one declared node species. An mKind is one naming scheme with one
  `resolve()`. A second way of naming the same referents is a second mKind; the two meet in
  the store (`resolve()` into one store ⇒ SAME).
- B(b): two declared node species. The referent-class node (311's mKind, kept as the
  umbrella) and, hanging off it, one or more naming-system nodes, each carrying an index to
  query (`:named-in`), a `resolve()` yielding the mKind's mPrimaryKey, and the two lookup
  warrants. Exactly one naming system is PRIMARY (its index is the store, because its
  mNaturalKey is the store's own address); one is DEFAULT (what a bare mKind in a bind means;
  absent, the primary). The floor is the primary before anyone authors it: identity
  `resolve()`, no warrants, index the declared store. A bind is typed by a naming system; after
  `resolve()` the mPrimaryKey is typed by the mKind.

Findings:

- `fnd-a-duplicates-without-a-coherence-seat` — under A every referent-class attribute
  (mPlacements, `:reaches` and the finished sentence, `:observer-dependence`, aspects, the
  parent menu) is written once per naming system by the same author, and the engine has no
  place to be told the copies describe one thing; two finished sentences for one referent class
  from one author land as withhold-and-narrate, never the fail-fast the human acked. Under B(b)
  each is written once and a naming system resolving into any other store is a static
  contradiction, refused pre-network. This is the ergonomic argument and it is epistemic: the
  author who knows the store says it once.
- `fnd-the-glue-seat-is-free-under-b` — a stranger publishes a naming system into an existing
  mKind (a lookup plus two warrants) and inherits its mPlacements, entailments, and aspects;
  the only thing they can get wrong is their own lookup, attributed to it. Under A they mint a
  whole mKind and either duplicate speech they do not own or leave it empty, in which case
  their mKind collides with everything.
- `fnd-the-store-is-the-primary-lookups-index` — the mKind declares no store and has no key
  concept of its own; `:sole-route` and `:rootness` sit on the primary naming system's index
  relation; a secondary naming system's index may be a different object from the store
  (`sm.File` by path: the mount table, not the filesystem). The polymorphic parent
  (local / network / cluster filesystem) is decided by the PARENT mKind's own primary
  `resolve()`, one level up: `sm.File`'s path lookup returns an inode and a filesystem key and
  stops; `sm.Filesystem`'s lookup classifies. Each owner speaks one level. "View versus
  store" needs no attribute: a client NFS mount's primary lookup resolves its index to the
  server's export, an unresolvable link from here.
- `fnd-one-naming-scheme-per-kind-is-forced-under-a-only` — the bind argument (a bind must
  know which lookup to run) forces DISAMBIGUATION, not mKind-minting; under B(b) the
  disambiguator is the naming-system node named at the bind. Qualifier-as-index does not
  generalise (users by name and by uid share the passwd index); qualifier-as-naming-system
  does.
- `fnd-two-epistemic-seats-mis-sited` — the lookup warrants are the index owner's knowledge
  (how the passwd database behaves under duplicate names) yet both shapes have the
  naming-system author write them; a default warrant declared on the index's mKind,
  inherited and only narrowable, seats it correctly. `:sole-route` is the parent mKind's
  knowledge (store or view); the parent declares once, children inherit; 311 puts it on the
  child.
- `fnd-sort-names-the-carrier-only` (**[HUMAN]**, hard nack) — 311 § 1.2's "sort" is
  many-sorted logic's carrier; pure-type-system sorts (universes) are the false friend. Under A
  each naming scheme is a degenerate sort; under B(b) only the referent-class node is a genuine
  sort, and a naming system is a term language, never a sort. If the word appears it names the
  carrier and nothing else. Subsort as carrier inclusion is importable without retracts or
  regularity; `resolve()` deciding a parent is a dynamic retract.
- `fnd-two-species-not-orders` — one lifting chain: a sh-level read (`cat`, `stat`, a
  command substitution) makes a value from the world; a bind or `resolve()` makes an mKey from
  a value, but an mKey is still an order-1 string with associated data, not a new order. Above
  that, two species, not strata: things that travel (referents, values, mKeys) and things that
  are declared (mKinds, naming systems, relations, warrants); the declared species is not
  stratified, since an mKind mentions mKeys as content while an mKey mentions its mKind only as
  a tag. They are not types.
- `fnd-key-is-value-kind-index` — an mKey is a value, its mKind, and the index it was looked
  up in; a coordinate is what an author writes, not yet an mKey (partial ack). Chroot is the
  example (one path, two mount tables, two inodes); crontab is not, since its mNaturalKey is
  the ambient user and the VALUE differs under sudo.
- `fnd-aspects-are-inhabited` — an mAspectKind borrows only the lookup (`:named-like`) and
  has its own store, referent, and value. The `@` sugar (`sm.Service:"nginx"@sm.Active`) is
  the same graph iff the aspect keeps its own store, a surface-syntax candidate; it must expand
  per parent and never let one aspect name span parents.
- `fnd-many-primary-keys-per-referent` — unenforceable without a registry; Dorc equates
  mKeys and never merges mKinds (`24M:rul24M-kind-unify-owed` stays a human act); coherence of
  the conceptual merged thing is by meet; the merged thing is never presented as one (a UX
  obligation, not a hole). Store-as-registry: naming acked, correctness reserved.
- `fnd-shared-natural-key-class-is-a-canary` — two mKinds sharing a natural-key class cannot
  corrupt each other at the lookup (custody); same string, same index, same mVantage, resolved
  to different keys of a `:guarantees-unique-referent` store ⇒ at most one is right ⇒ withhold
  both and narrate.

## § 8-counterexample-hunt-banked

One sitting (2026-09-10), after the shape sitting: every constriction in the B(b) gloss
attacked with sh. Nothing ruled.

- Held: one parent per mCell (a lazy bind is saved by declaring both placements); the engine's
  route claim under `cd`, ρ writes, `ln -sfn`, unmodeled retargets, and `&` (each by an existing
  rule); strangers naming one store by different mKinds (unspoken, precision only); whole-to-part
  `:reaches` against `enable --now` (a part-level footprint names what it touches).
- `fnd-schemes-belong-to-one-kind` — the ABC case: a naming system declared under two mKinds is
  the shadowing refusal; reuse is delegation inside a `resolve()` body (sanctioned composition,
  custody by sourcing); two naming systems on one mKind can disagree on a mPrimaryKey and the
  store's :guarantees-unique-name then licenses a wrong DISJOINT, attributed to both (the canary).
- `fnd-store-type-varies-the-chain` — sshfs synthesises inode numbers per client mount, so two
  clients hold two mPrimaryKeys for one file; the variation is by the TYPE of the store instance
  (ext4, nfs, sshfs, overlay, loop, proc), never by the child mKind or its naming system. What
  varies per type: how the store instance itself is identified (its own parent), its placements
  (loop: the backing file), and whether its content addressing is unique (proc: no). Whether the
  store is a view is DERIVABLE from its own chain reaching an unknown link, so :sole-route needs
  no attribute. Proposal B(c): store types as first-class SUBSORTS of the store mKind
  (order-sorted, carrier inclusion), the umbrella's `resolve()` performing the retract on the
  host; children key into the umbrella and inherit the subsort's answers. "Index-kinds" (`30W`,
  context axes) is a different concept; never reuse the name.
- `fnd-parent-kinds-carry-placements` — a loop-backed filesystem's state lives in a file of the
  outer filesystem; `dd` over the image rewrites every inner fact; every mKind in a chain may
  declare `:lives-in`, not only leaves.
- `fnd-schemes-take-site-parameters` — `acct --db PATH`: the index arrives in argv; a naming
  system's index may be site-supplied (the R1-into-R1 case).
- Nit: SAME then DISJOINT composes to DISJOINT; only DISJOINT then DISJOINT never chains.
- Punted to its own exercise, not dropped: overlayfs (GOTCHAS
  `identity-tokens-perish-on-write-not-only-on-rename`): a merged view over two stores; the
  type row needs placements {lower, upper} and no warrants, else a write to the lower spares
  wrongly through the merged path.

## § 6-state-for-a-rewound-successor

Commits on `ai/main` this phase, oldest first: `9850d3cb` GOTCHAS 23–56 · `b4d80fd4` 312a ·
`1bb710a0` exercises 01–03 · `c0e29b54` exercise 04 + this ledger · `660aae7f` conduct ·
`866bedbd` the `resolve()` naming across 311x/312x. The tagging pass (mTerm, :relation, `op()`)
over the same sixteen files is landed by the commit carrying this section; its report noted
that `perish()` has no operation-sense occurrence anywhere, that "world" as external reality
and "witness" in its completion and evidentiary senses are never tagged, and twenty-one
borderline calls, all recorded in the scratchpad report and reviewable in the diff. Three lines describe a host read in plain English, not the `resolve()`
operation, and stay untagged: exercises/01 ~54, exercises/02 ~52 and ~65.

Exercise 04 is still OPEN: its file is the ten-observation version and owes a rewrite into the
light staged form (the no-root floor; a root arrives; the store-bound world); its § 2 block
above predates § 5 and is superseded where they differ. Owed on design-of-record documents,
each waiting on the human's word: 311 § 2.2 (the parent menu; the hardlink example), 311 § 3.2
(delete the route and root clauses; state :sole-route's orientation leaf-ward; state that SAME
is "or" across mDerivations and "and" within the mKey walk), 30U (the finished definition's
licensor bounded by stores). Punted by the human: the flag's re-derivation; where custody and
marks sit; whether the admin's cross-target line is a book line, a lint, or a posture flag.

Of the conductor's long naming turn (2026-09-10), the human had reached and answered: the
stops, the retractions, and the naming conventions. Not yet reached when the rewind was
called: directionality, placement as an edge set, the polymorphic identifying parent, where
warrants sit, the layers check, and the store intuition; all are banked in § 5 above.
