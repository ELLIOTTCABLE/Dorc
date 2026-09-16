# 312b-exercises/04 — a-host-is-not-a-partition

Exercise record; ahistorical; takeaways extracted to `notes/312b` § 2. Book lines are real sh;
the `#` glosses are prose in `notes/311` vocabulary, never spellings.

## Strawmen

S1, the floor: two targets, one NFS export, no mRoots declared anywhere.

```sh
ssh beta  'apt-get install -y foo'     # runs; postinst writes /srv/shared/foo.conf
                                       # footprint: Package:foo, widened by the Package owner's
                                       # finished definition to foo's File keys, each a chain
                                       # inode → fsid-beta → boot-beta → machine-beta → ROUTE(beta)
ssh alpha 'cmp -s ./foo.conf /mnt/shared/foo.conf || cp ./foo.conf /mnt/shared/foo.conf'
                                       # probed converged; backing File:/mnt/shared/foo.conf,
                                       # chain inode → fsid-alpha → boot-alpha → machine-alpha → ROUTE(alpha)
```

`compare()`: two mRoutes; § 1.9 says unknown. Collide; the guard stays; at apply it re-reads and
runs. Correct, coarse. This is `26Ob:ack-cross-world-wall-is-the-floor` reproduced by the model
with no mRoots.

S2, a true mRoot arrives: the cloud mSort's owner declares instance-ids a mRoot (honestly; no
clone horizon), Machine identifies in Instance, both probes ran every `resolve()`.

```sh
#   … → machine-beta  → i-beta   (root)
#   … → machine-alpha → i-alpha  (root)
```

Same mRoot mSort; first divergence at the top; instance-ids carry :guarantees-unique-name. § 3.2:
DISJOINT if "every level above it carries :sole-route." Read mRoot-ward, nothing is above:
DISJOINT, :sole-route never consulted. Read leaf-ward, which is what § 2.2 defines :sole-route
for: Machine-in-Instance, Boot-in-Machine, Filesystem-in-Boot, File-in-Filesystem must all hold,
and File-in-Filesystem is false on NFS (one export, two client fsids), so an owner who declared
it lazily yields DISJOINT and one who withheld it yields UNKNOWN. Either way alpha's guard is
spared past beta's postinst unless the File or Filesystem owner thought of NFS.

S3, partial measurement: as S2, but alpha's probe did not run the instance `resolve()`.

```sh
#   … → machine-beta  → i-beta   (root)
#   … → machine-alpha → ROUTE(alpha)
```

§ 3.2: one side a mRoute, the other not: UNSPOKEN. Sparing rides the footprint side's finished
definition; Package's is written; alpha's guard is spared. Fewer measured links produced more
license than S2's complete chain under the honest reading.

S4, the SAME direction, the mWorld of S1 plus the admin's own mount line:

```sh
ssh alpha 'mount beta:/srv/shared /mnt/shared'
                                       # a transition the admin wrote; the mount owner derives a
                                       # correspondence: keys under /mnt/shared at alpha ≡ keys under
                                       # /srv/shared on the server named "beta" from alpha's vantage
```

Two mDerivations now exist for the pair: the mKey walk (unknown) and the mCorrespondence (SAME,
once the admin says the server "beta" is the `ssh beta` target,
`26M:ack-authored-host-sameness-parallel`). Coherence: SAME stands; the footprint collides with
the backing; the guard stays. Correct, and precise rather than coarse.

S5, neighbours, one host and the export:

```sh
mount --bind /srv/shared /mnt/shared   # one host: two paths, one fsid, one inode → SAME by the key walk
ls -l /mnt/shared/docker.sock          # a socket on the export: the File `resolve()` returns equal (fsid, inode)
                                       # on both hosts → SAME → yet a connect from alpha fails
```

The bind mount is the case the mKey walk was built for. The socket is a File mKey reaching a
kernel object; the File owner's `resolve()` must decline on it, else a fact about beta's daemon
transports to alpha.

S6, the admin's posture, in behaviour only: "alpha and beta share no store."

```sh
# not a spelling: the admin asserts sole-route at the Target store for every sort whose chain
# passes through a Filesystem; S2's divergence then yields DISJOINT honestly, attributed to the
# admin's line; S1 stays unknown without it
```

## Observations

- `obs-unspoken-is-sort-level-only` — § 3.2's route-versus-root and roots-differ clauses read
  unspoken; both must read unknown. Unspoken means two vocabularies that never met inside one
  mWorld, which is what 30U's finished definition was written to spend; a mRoute or a second mRoot
  is a second mWorld, and no finished definition speaks to it.
- `obs-partial-measurement-must-never-widen` — S3 against S2: a chain with fewer measured links
  compared no less decisively. A monotonicity law is owed: a mDerivation with an unmeasured or
  mRoute-terminated link yields at most what it yields with the link measured. `16P` DP-8's
  vacuous-bottom is the same law from the other side.
- `obs-sole-route-runs-leaf-ward` — § 2.2 defines :sole-route as what lets a divergence at a
  store level license disjointness beneath it; § 3.2's "every level above it", after "walk from
  the mRoots", reads the other way. State the orientation: every level between the divergence
  and the leaf.
- `obs-rootness-is-retroactive` — a true mRoot anywhere up a chain converts every lazy store
  beneath it, fleet-wide, from unknown to decisive. The blast radius is not the mRoot's own
  mReferents; it is every mSort that identifies into it. Countability: mRoots are stdlib-only and
  few, and the first one is the trigger for every hole in this record.
- `obs-the-finished-definition-is-within-world` — "nothing else, in any vocabulary" is not "in
  any mWorld"; the engine must not spend it across mRoutes or mRoots; when it does, the Package
  author is the wrong name in the why chain (pope-sin).
- `obs-the-filesystem-owner-must-know-network-filesystems` — `311` § 6.2's "the File owner
  should never learn NFS" holds only if the Filesystem owner does: identify a network mount in its
  server (an unknown link, through the mVantage) or decline. A statfs fsid scoped in Boot is
  true and unsafe.
- `obs-cross-target-disjointness-is-the-admins-sole-route` — S6: "these targets share no store"
  is :sole-route at the Target store, the admin's seat under the posture option
  `26Ob:ack-cross-world-wall-is-the-floor` reserved. The model needs the seat; no rule change
  replaces it.
- `obs-same-is-or-across-derivations` — S4: the mKey walk is "and" over its levels; SAME overall
  is "or" across mDerivations; the mount line is the mCorrespondence generator and the admin's
  host pairing is its last link.
- `obs-reads-decline-outside-their-ontology` — S5: the File `resolve()` declining on non-regular files
  is the net that keeps a borrowed path from minting SAME for a kernel object.
- `obs-attribution-tests-the-rule` — for any fix: when the case fires, the why chain must name
  a line that could be wrong (the mRoot declaration; a missing :sole-route; the admin's posture),
  never one that could not (the Package author's within-mWorld sentence).
