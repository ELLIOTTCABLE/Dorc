# System A: the narrow refinement (pointer-chain identity grafted onto the current design)

Scratchpad, conductor-only, never committed. Grades +SURE / ~SUSPECT / -GUESS / --WONDER.
Goal: keep the coordinate, the chokepoint, the consumer map, the flag, the finished-definition
gate, the index-kind context slot, and the placeholder/witness machinery; change WHO answers the
identity question and WHAT it is asked about. Syntax is strawman throughout.

## A0. The one change, stated

Today a cell's cross-context identity is answered by its KIND's owner filling a table against
index-kinds (invariant / keyed / T). Under System A a cell's identity is answered by following its
STORE POINTERS: each store pointer is a coordinate in another kind, declared at the site that
mints the cell (so it can carry the site's argv), and the store's kind owner answers identity by a
measured `resolve()` run in the site's world. Recursion bottoms out at a kind with a `resolve()`, or
at unknown. The index-kind slot survives as "which world the `resolve()` calls run in" and as the batching /
placeholder key, no longer as a set of axes the cell's owner must have opinions about.

## A1. New speech act: the store pointer mark (per site, per coordinate)

A mark verb `stored-in` usable on any coordinate-minting line (verdict lines in
`__is_converged`; emission lines in `__disturbs`; the fs binder's claims), naming a coordinate
in another kind whose identity DETERMINES this cell's identity. Multiple allowed. It is not a
read mark: `reads` widens staleness (kill-traffic); `stored-in` fixes identity. Normally
`stored-in` ⊆ `reads` and the read-vs-marks detector should nudge when a store is not also read.

```sh
acct__is_converged() {
   # argparse (verb, --db, --directory, number) ...
   db   : sm.dorc.File = "$dbpath"                      # bind: the operand is a File entity
   acct : org.acct.Account = "$dir/$num"                # bind: the owner's documented name form
   acct --db "$db" --directory "$dir" status "$num" \
      : org.acct.Account:"$acct"@enabled   stored-in sm.dorc.File:"$db"
}
acct__disturbs() {
   # same argparse ...
   printf '%s\n' "$dir/$num"   : disturbs org.acct.Account   stored-in sm.dorc.File:"$db"
}
```

Kind-level fixed stores stay where they are: `kind__state_stored_in` (per-arm, decomposed per
`30W` item 6) emits store pointers as KIND COORDINATES (`: stored-in sm.dorc.File`, entity on
stdout: `/var/lib/dpkg/status`). A cell's store set = kind-level pointers ∪ site-level pointers.
Substrate tokens (`fs`, `process`, `kernel`) are gone; they were kinds all along
(`sm.dorc.File`, `sm.dorc.Process:1`, `sm.dorc.Kernel`), each needing a `resolve()` for the
recursion to terminate (`311a` nit, taken to its end).

Totality: `stored nothing-else` (the ruled decomposition's sentinel) governs whether the store
set may license ANYTHING positive. Without it the store set is open-world and every
store-based comparison answers unknown (A4). This is stricter than `30W` §3's "without it an
unlisted store collides" and it has to be: with an undeclared shared store (a global index
file), two cells with disjoint declared stores are not disjoint either.

## A2. New kind member: the `resolve()`, generalized

`kind__identity()` (the `26Ob` §10c strawman for Host, generalized to every kind): invoked with
an entity, executed in the denoted world at the latest sound phase, prints ONE line the engine
holds as an opaque token; rc regime three (completion only); decline ⇒ unknown. Owner declares
alongside it whether the token is REFERENT-TRANSPARENT (equal ⇒ same; unequal ⇒ disjoint) or
merely IDENTIFYING (equal ⇒ same; unequal ⇒ unknown), and states the horizon in one sentence.

```sh
sm_dorc_File__identity() {                     # STRAWMAN; the stdlib File owner, Linux arm
   findmnt -no UUID,FSTYPE -T "$1" | ...        # a filesystem identity comparable across hosts
   stat -c '%i' -- "$1"                          # plus the inode; NFS: the server fsid + inode
   # printf one token line; decline (return 2) where no cross-host-comparable identity exists
}
: referent-transparent                          # STRAWMAN: unequal tokens separate
```

`kind__resolve` stays (name → canonical name, cheap, often static). `identity` is the stronger,
measured answer; where both exist, resolve runs first, then identity on the canonical name.

Injectivity: a kind's owner may declare its namespace NON-INJECTIVE (distinct canonical names
may denote one referent WITHIN one store — File, Host, anything path-like). Default remains
injective (today's disclosed-weak name floor, `300:rul-reference-entity-name-floor`); File's and
the index-kinds' carves become stdlib declarations instead of engine special cases. Inherited
debt, flagged: the default is a silence-licenses case. Not fixed in A.

## A3. The context slot's narrowed job

Unchanged shape: a finite map index-kind → index-value (entered / measured placeholder / T);
`ssh beta` lends Host=beta, `sudo -u alice` lends User=alice, `chroot /mnt` lends
MountNamespace=(entered). Its jobs under A: (i) name the world a `resolve()` executes in; (ii) the
batching / placeholder key `sk(E, K)`; (iii) for kinds with NO stores and no `resolve()`, the
wall (any context difference ⇒ unknown, as today). It is NOT consulted per-kind through an
owner's trichotomy table any more. Host need not be referent-transparent for Account's sake;
only Account's stores' tokens must be.

## A4. The relation, rewritten (same-kind branch only; cross-kind unchanged)

    compare(K:e1@c1, K:e2@c2):
       n ← names(K, e1, e2)               # via resolve: same / different / unknown
       S1, S2 ← store sets of the two cells (kind-level ∪ site-level), each member a coordinate
       s ← stores(S1@c1, S2@c2)          # recursive compare per pointer, then:
                                         #   same iff |S1|=|S2| and every pointer pair same
                                         #   disjoint iff store sets share no same-or-unknown pair
                                         #   else unknown
       total ← K's store set is finished (stored nothing-else reached)
       inj   ← K not declared non-injective

       if n = different ∧ inj ∧ total        ⇒ provably-disjoint   # the name floor, now conditioned
       if n = same ∧ s = same ∧ total        ⇒ same
       if s = disjoint ∧ total               ⇒ provably-disjoint
       else                                  ⇒ unknown

    base case: K has a `resolve()` ⇒ compare tokens read in c1 and c2 (equal ⇒ same;
    unequal ⇒ disjoint iff referent-transparent else unknown; unbound / declined ⇒ unknown).
    K has neither stores nor a `resolve()` ⇒ same iff c1 = c2 as maps and names same (today's
    within-context floor); unknown across any context difference.
    cycles in the pointer graph ⇒ unknown (depth-capped).

Consumers unchanged: same → transport; provably-disjoint → sparing under the flag; unknown /
unrelated safe for both. The universal meet over backing sets unchanged: transport needs every
backing member to transport; sparing needs every footprint×backing pair disjoint.

Consequence: sameness and disjointness now come out of ONE recursion. A store-level `same`
(one NFS file) is exactly what makes a cell-level `disjoint` unreachable for L3.

## A5. Invariance lines become derivable; keep them as static shortcuts

`: undivided-by-transit-across K'` for kind K says "my stores' identities are equal across
values of K'". Under A that is what the `resolve()` would find. Keep the line as a static
assertion that (a) saves a measurement where the owner is sure and (b) is CONTRADICTION-CHECKED
against a `resolve()` where both exist (tokens differ ⇒ declarations-genuinely-contradict ⇒ fail-fast,
pre-network where static, refuse-both otherwise). The trichotomy table (`30W` §4) is subsumed:
"keyed by K'" is derived from unequal store tokens across K' (Service@active stored-in
Process:1, whose identity is the boot token); "invariant" from equal ones (Package stored-in
File:/var/lib/dpkg/status). Patch day (`30W` §7) falls out with zero invariance lines.

## A6. Transitions perish identity through footprints on the store kind

An identity binding (token bound for store coordinate P in world w) is itself a FACT with
backing {P@w}. It dies by ordinary effective-world reach: any running mutator whose footprint
collides with P@w un-binds it (back to unknown) for everything downstream. Two granularities:

- coarse floor: File cells are stored-in `sm.dorc.MountNamespace:<entered>`; a mount oracle's
  `disturbs sm.dorc.MountNamespace:"$ns"` collides with EVERY File identity in that namespace
  (whole-entity ⇒ T-selector). Safe, over-broad: a mount under /mnt/team perishes /etc/passwd's
  identity too.
- region refinement: the mount oracle emits the mountpoint as a File REGION (`printf '%s\n'
  "$mp" : disturbs sm.dorc.File@subtree`, STRAWMAN selector) and the File owner's
  `kind__disjoint` decides containment of each File identity's entity under that region; only
  contained identities perish.

Neither author names the other: mount speaks File/MountNamespace vocabulary; Account speaks
File vocabulary; the engine chains. Perishing is withhold-shaped (composites may only withhold;
`28M:rul-composite-meets-toward-guard-run` satisfied). A dependent transport license (an
elision resting on a perished `same`) demotes to guard at settle; a dependent sparing (a
`disjoint` resting on a perished token) collides.

`30T` §6's v0 floor (entry-mutating verbs are total walls) can then relax exactly as far as
those verbs' oracles emit footprints in File vocabulary; until they do, still total walls.

## A7. The witness and the placeholder, unchanged in shape

A `resolve()` binds placeholders at probe (`sk(E, K)` per entry chain); re-run at apply standup
and after every fired transit disturbance; mismatch is integrity (withhold), never a verdict
input (`30W` §4; `26Ob:cor-standup-witness-licenses-bare-line-elision`). Two sites under one
entry chain share a placeholder pre-measurement as today.

## A8. The runbook under A

- Marks: `Account:"$dir/$num"@enabled stored-in File:"$db"` in the verdict; the same store
  pointer on the disturbs emission. File declared non-injective; File `resolve()` returns a
  cross-host-comparable (fsid, inode) or declines.
- L1@alpha, L3@alpha: File:/srv/people/accounts.db → token T. L2@beta, L4@beta:
  File:/mnt/team/people.db → token T (the shared export). Account's store set is total (one
  pointer + the sentinel).
- L3 vs L2: names same (red/7), stores same (T=T) ⇒ SAME ⇒ collide ⇒ L3 guards. Correct; and
  it would stay correct with Host referent-transparent and alpha≠beta, because Host is not
  consulted.
- L1 vs L4: names different (red/7 vs blue/7), injective, total ⇒ disjoint ⇒ no transport; L4's
  own probe is diverged ⇒ runs. Correct. Bare-`7` binding: names same, stores same ⇒ same ⇒ L1's
  fact transports and meets L4's own contrary measurement ⇒ disagreement ⇒ unknown ⇒ run, with
  the why-chain naming the acct author's bind. Knife lands on the right author.
- File `resolve()` declines cross-host: s = unknown ⇒ L3 collides (safe), L1 does not transport (safe).
- A `mount` under /mnt/team between L1 and L2: beta's File identity perishes below it; L2's
  footprint store is unknown ⇒ compare with L3's T ⇒ unknown ⇒ collide. Safe. L4's store is
  unknown ⇒ nothing transports into it. Safe.
- sudo/crontab (`272` §0): Cron cell stored-in File:/var/spool/cron/crontabs/$(id -un); `resolve()`
  as alice vs root ⇒ different inodes ⇒ disjoint. dpkg: same inode ⇒ same ⇒ the alice-measured
  fact answers the root site. Zero invariance lines.

## A9. What A touches in the existing design (churn inventory)

- ADD: the `stored-in` mark verb on coordinate-minting lines; `kind__identity` as a general
  member with a referent-transparent / identifying declaration and a horizon sentence; a
  per-kind non-injective declaration.
- KEEP: coordinate + slot; chokepoint + consumer map; flag; finished definition; universal
  meet; `resolve`; `disjoint`; `disturbance_reaches`; placeholder + witness; measure-in-context
  default (`27C`) with transport as the fallback lane.
- DEMOTE: the invariance line to a static shortcut with contradiction-check; the trichotomy
  table and the per-index filtered meet (`26Ob` §10b) to a special case of store recursion
  (they remain the semantics for kinds whose stores are index-kinds directly).
- RETIRE: substrate tokens; the engine-side File / index-kind name-floor carves (become
  declarations).
- BREACH, deliberately: `272` §5 addresses-are-not-coordinates (store locators ARE coordinates
  in the store's kind; the all-Package-cells-collapse fear does not materialize because store
  sameness is necessary for cell sameness, never sufficient — names still separate rows);
  `279f` §3's refused transport chain (re-litigated: this is the accepted invariance-line
  mechanism with measurement replacing declaration, under the same totality sentinel).
- UNCHANGED knives: a wrong store list (pipx-in-~/.local) still transports a user-dependent
  fact; a wrong read horizon (device numbers compared across hosts) is a wrong-same; a wrong
  non-injective omission on a path-like kind is a wrong-disjoint. Each is one author's line.

## A10. Holes A does not close

- Default injectivity is silence-licenses (the name floor's original sin), kept for stability.
- Store pointers must be repeated on every coordinate-minting line of a family (verdict AND
  disturbs); cargo-cult pressure; an engine rule "a family's disturbs cells inherit the verdict's
  store pointers for the same argparse arm" is tempting and probably wrong (different arms).
- Kinds whose store is an instance of themselves (symlink → target; bind mount → source) need
  the recursion to terminate at a measured token, and the token's horizon carries the aliasing.
- Cross-host token comparability rests entirely on each store kind owner's horizon sentence;
  nothing mechanical checks that a token is host-stable.
- The `resolve()` runs per (pointer × world) at probe: one more authored body shipped per
  store kind per world. Network-cheap; authorship-costly for the stdlib.
- Still two vocabularies for "where my state is": the kind-level member and the site-level
  mark. Coherent, but two.
