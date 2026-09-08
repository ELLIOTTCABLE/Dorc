# System B: identity from whole cloth (referents, routes, and stores)

Scratchpad, conductor-only, never committed. Grades +SURE / ~SUSPECT / -GUESS / --WONDER.
Goal: the simplest internally-coherent semantic for "are these two things the same thing",
composed from mutually-unknowing authors' single-link statements, with no obligation to the
current coordinate, slot, trichotomy, invariance line, or name floor. Syntax strawman.

## B0. Four primitives

REFERENT — a piece of the world that has state: an inode, a db row, a package record, a
kernel parameter, a running process, a machine. Cells are (referent, aspect); aspects are the
owner-minted selectors, unchanged.

NAME — how a tool addresses a referent FROM SOMEWHERE. A name is always relative to a
namespace: `red/7` relative to a database file; `/mnt/team/people.db` relative to a mount
table; `alice` relative to a passwd database; `beta` relative to a resolver configuration as
seen from a vantage. Names are strings authors wrote; the engine never decodes them.

NAMESPACE (routing) — the thing a name is interpreted IN. Itself a referent of some kind
(a File; a MountNamespace; a Host-as-vantage). Namespaces are what viewpoint transitions
perturb: `chroot` swaps the mount table; `ssh` swaps every ambient namespace for the far
host's; `sudo -u` swaps the identity user-relative names resolve against; `mount` and `ln -s`
and `mv` MUTATE a namespace in place for everyone after the line.

STORE (containment) — the referent a referent's state physically lives in. An Account row
lives in its db file. A File's contents live in a Filesystem (identified by fsid) at an inode;
a File's existence lives in its parent Directory's entry table. A Service's enabled-ness lives
in a unit-file symlink (a File); its active-ness lives in pid 1 (a Process, whose store is a
Boot). A Package record lives in the dpkg status File.

Routing and containment are DIFFERENT relations and the design must keep them apart: two
names in two different mount tables can route to ONE inode (bind mounts, NFS), so
namespace-disjointness never implies referent-disjointness; but a row cannot live in two
database files, so store-disjointness does imply referent-disjointness. Which of the two a
kind exhibits is the kind owner's to say, and the current design (and System A) conflate them
under one "stored-in".

## B1. The identity of a referent

    identity(R) = (identity(store(R)), canonical name of R within store(R))

recursively, terminating at a kind whose owner supplies a MEASURED token (an identity read run
in the world the name was resolved in), or at Top. The namespace a name was resolved THROUGH
is not part of the identity; it is provenance, and it is what the witness re-verifies.

So a Ref is a self-contained value once resolved:

    Ref ::= Located(kind, canonical-name, in: Ref)      # a named referent inside a store
          | Measured(kind, token, grade, horizon)        # a referent the owner can measure
          | Route(entry-chain)                           # the fallback store: "wherever this
                                                         #   route reaches"; same iff same
                                                         #   unwalled chain, else unknown
          | Top

`Route` is the honest floor for kinds that declare neither store nor read: their referents are
identified only by how the probe got to them. This is the old context slot, demoted to the
bottom of the chain and used only when nothing better exists.

## B2. compare(R1, R2) — one recursion, four answers

    Measured(k, t1) vs Measured(k, t2):
       t1 = t2                       ⇒ same
       t1 ≠ t2 ∧ grade = transparent ⇒ disjoint
       else                          ⇒ unknown
    Located(k, n1, in: S1) vs Located(k, n2, in: S2):
       s ← compare(S1, S2)
       s = same:     n1 = n2 ⇒ same · n1 ≠ n2 ∧ k injective ⇒ disjoint · else unknown
       s = disjoint: disjoint                       (a row cannot be in two files)
       s = unknown:  unknown
    Route(E1) vs Route(E2): same iff E1 = E2 and the route is unwalled between the two
                            sites (`26Ob` §10a's derived batching key) · else unknown
    different kinds: compare the store chains; disjoint iff both chains are TOTAL and share
                     no same-or-unknown store pair; same never (cross-kind co-reference stays
                     an owner-declared act, not derived); else unrelated
    Top vs anything, itself included: unknown
    any positive answer additionally requires TOTALITY of every store set it rests on
    (the `located nothing-else` sentinel, B4); an open-world store set yields unknown.

Consumers as today: same → transport; disjoint → sparing under the flag; unknown /
unrelated safe for both. The universal meet over backing sets stands. `disjoint` for
sparing still also needs the FOOTPRINT side widened and finished (`disturbance_reaches` +
`disturbs nothing-else`, unchanged): reach is effect entailment, not identity, and B keeps it.

Note what dissolved: "transport" as a distinct mechanism. If two sites' Refs compare `same`,
there is one cell; whichever probe read it, read it. The world a fact was measured through
is provenance, and the witness's job is to confirm the route still reaches that referent at
apply. `--risk-faultless-skips` still gates sparing; `same` rides the owners' reads and
totality sentinels, vouch-tier, exactly as invariance lines did.

## B3. Who says what (the authored surface)

Kind owner, per kind:
- `kind__identity()` — the ONE identity member. Input: a canonical name (or the store's
  handle for store-relative kinds); runs in the denoted world; prints one token line; rc is
  completion-only; decline ⇒ unknown. Two grades declared beside it: `transparent` (unequal
  tokens separate) or `identifying` (equal tokens unite, unequal say nothing); one horizon
  sentence. `resolve` is FOLDED into this: for a symbolic kind the token is the canonical
  name (`dpkg-query -W -f '${Package}'`), graded identifying-or-injective; for a measurable
  kind it is the measurement (`fsid:inode`). One member, one question: "what is this, as far
  as you can tell, from here?"
- `kind__located_in()` — per selector: `: lives-in KIND[:fixed-entity]` (the store; where a
  site cannot supply the instance from argv, a fixed one is named here) and `: named-in KIND`
  (the routing namespace, where names are ambient-relative). One line `: located-in KIND`
  when they coincide (Account: both the db File). Totality: `located nothing-else` per arm.
  Injectivity of the kind's names within one store: `: names-are-distinct` (default absent ⇒
  unknown on name inequality; silence licenses nothing, no disclosed-weak floor).
- `kind__disjoint()` — region containment for path-like kinds (unchanged ruling); used for
  perishing under a disturbed region and for region-vs-region sparing.
- `kind__disturbance_reaches()` + `disturbs nothing-else` — unchanged.

Tool oracle, per site:
- binds carry BOTH the entity and, where argv supplies it, the store instance:
  `db : sm.dorc.File = "$dbpath"` then `acct : org.acct.Account = "$dir/$num" in "$db"`
  (STRAWMAN `in`: the bind's second clause names the store instance for a kind whose
  `located_in` says the store is a File). Marks and disturbs emissions then carry the bound
  entity, whose Ref already includes its store; no per-line `stored-in` repetition.
- verdict marks, observe marks, `disturbs` emissions, `predicts` — unchanged.

Wrapper oracle:
- `cmd__lend_map()` lends ROUTING NAMESPACE INSTANCES for the kinds it perturbs, and nothing
  else: `sudo -u X` lends the User namespace; `chroot D` lends MountNamespace :=
  Located(MountRoot, D, in: ambient); `ssh H` lends Host := Located(Host, H, in: vantage) and
  RESETS every ambient namespace to the far world's (the reset sentinel); `nice` lends
  nothing. Absent kinds inherit after a `lends nothing-else` sentinel, else Top. The map is an
  analysis-time ENVIRONMENT consulted when a name is resolved to its store instance; it is
  not part of any fact's key.
- entry forms and `safe-across` unchanged (measure-in-context stays the default lane).

Mutator oracles:
- a routing mutator (`mount`, `ln -s`, `mv`, `useradd`, `hostname`) `disturbs` the namespace
  cell (`sm.dorc.MountNamespace:"$ns"@table`, or a File region via `disjoint`). A RESOLUTION
  (name → store instance, made under that namespace) is a fact whose backing is the namespace
  cell; it perishes below the mutator; every Ref built on it re-reads as Top downstream.
- a store mutator writes cells as today; identity untouched.
- a lifecycle mutator (`reboot`) disturbs the Boot referent; stores that live in Process or
  Kernel get new tokens on re-read; their cells are new cells (the old "keyed" outcome,
  derived).

## B4. Totality, injectivity, and the two knives

Positive answers rest on exactly three authored completeness acts, each one author's line:
- `located nothing-else` (kind owner): "my referents live in these stores and no other".
  Omitted store = a hidden shared channel = the pipx-in-`~/.local` knife, wrong SAME or wrong
  DISJOINT alike. Without the sentinel, every store-based answer is unknown.
- `names-are-distinct` (kind owner): distinct canonical names in one store are distinct
  referents. Wrong for a path-like kind = the synonym knife (`23M`'s dangerous cell).
- `transparent` grade on a read (kind owner): unequal tokens are different referents. Wrong
  for a cloneable token (machine-id on cloned images) = wrong disjoint across clones; but
  note the read's EQUALITY side is unaffected by that grade.

And one on the footprint side, unchanged: `disturbs nothing-else`.

Silence anywhere ⇒ unknown ⇒ walls. There is no default positive answer left in the system.

## B5. Transitions, walked

- `ssh beta acct --db /mnt/team/people.db ...`: ssh lends Host=beta and resets ambient
  namespaces. `--db` is bound as a File in beta's MountNamespace; File's `identity` runs on
  beta and returns fsid:inode (or declines). The Account Ref is Located(Account, red/7, in:
  Measured(File, T)). Alpha's line yields Located(Account, red/7, in: Measured(File, T)).
  Same T ⇒ one cell. Host never enters the identity.
- `chroot /mnt apt-get install x`: MountNamespace lent; Package's store File
  /var/lib/dpkg/status resolves in the chroot's table to a different inode ⇒ Measured(File,
  T') ⇒ Package cells inside the chroot are different cells from the host's. The old
  "package kind must not claim fs-view invariance" becomes a measurement, not a rule.
- `sudo -u alice crontab -l` vs root: Cron located-in File:/var/spool/cron/crontabs/$(id -un);
  the who-am-I ingredient resolves under the lent User namespace to two names, two inodes ⇒
  disjoint. `sudo dpkg -s nginx`: same inode ⇒ same cell as the unprivileged site's.
- `mount -t nfs srv:/team /mnt/team` mid-book: disturbs MountNamespace@table (coarse) or File
  region /mnt/team (fine, via File's `disjoint`); every resolution through it perishes; Refs
  below re-read Top; their comparisons go unknown; dependent transports demote to guard,
  dependent sparings collide.
- `reboot`: Boot disturbed; Process:1's token changes; Service@active (lives-in Process:1) is
  a new cell ⇒ guard; Package@installed (lives-in File on a persistent Filesystem) has the
  same fsid:inode ⇒ same cell ⇒ elide. Patch day, zero declarations beyond `located_in`.
- A kind with nothing declared (the two-minute oracle, the auto-cell): Route(E) at the
  bottom; same site under one chain answers itself; anything across worlds is unknown.

## B6. What B abandons, plainly

- The context slot as a component of a fact's key. Worlds are ROUTES: an analysis-time
  environment of routing-namespace instances, consulted at resolution, recorded as
  provenance, verified by the witness. Only `Route(E)` survives as the last-resort store.
- The per-index-kind trichotomy and the filtered meet over index-kinds.
- Invariance lines (`undivided-by-transit-across`). Derived from reads. (Could return as a
  static hint that skips a read, contradiction-checked; not needed for soundness.)
- Substrate tokens. Kinds.
- The name floor as a default. Injectivity is declared per kind; silence ⇒ unknown.
- `resolve` as a separate member. Folded into `identity` with a grade.
- The engine-side File / index-kind carves. Declarations.
- The one-sentence volume identity of `30W` §5. Replaced by File's read plus the store
  chain; a Volume/Filesystem kind's read is where NFS fsid versus local UUID is decided, and
  the File owner's `located_in` points at it.

## B7. What B keeps unchanged

The (kind, entity, selector) core with owner-minted selectors and the selector dialect; the
four-valued chokepoint and its consumer map; `--risk-faultless-skips` on sparing; `reaches`
+ finished definitions on the footprint side; `disjoint` as a region predicate; the universal
meet over backing sets; measure-in-context as the default lane with entry forms and
`safe-across`; the read-set closure as the falsification net for unmarked reads; the
placeholder/witness shape (placeholders now keyed by resolution, not by index-kind); binds
as the naming act; the committee law (every positive link one author's).

## B8. Prices

- Value regression for lazy kind-minters: without `names-are-distinct`, same-kind
  different-name pairs no longer spare. The stdlib declares it day one; third parties must
  opt in. This is "silence licenses nothing" applied to the one place it was not.
- More authored reads: every store kind in a chain needs an `identity` for the chain to
  license anything; until the stdlib has File, Filesystem, Process, Boot, Host reads,
  everything above them is Route-keyed (today's behaviour).
- A representational rewrite: facts carry Refs, not (coordinate, context). `30W` item 1's
  context-slot product is NOT built; something narrower (the ambient namespace environment)
  is built at the analysis seat instead.
- The routing/containment split is one more distinction for kind owners to learn. It earns
  its place: conflating them is a wrong-disjoint on every path-like kind.
- Two ways to name a store instance: fixed (in `located_in`) and per-site (the bind's `in`
  clause). Coherent, but two.
