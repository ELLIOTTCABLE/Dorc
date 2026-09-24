# 312cb — Identity model crosscheck (Fable, lane A)

> Adversarial crosscheck of `notes/311-identity-and-relation-model.md` at
> `6b7108b4ccabc3d961508d98a563ebc06616623b`, read-only, nothing ruled. The test is the
> document's own § 0 law: a world and a book where every party's statement is true and the
> design answers wrong, or the true answer is unreachable however many people speak, or
> reaching it costs what no admin or engineer would pay. Every finding is spelled in plain
> ops-sh with real tools; every claim carries `+SURE` / `~SUSPECT` / `-GUESS`. Findings are
> ordered by kind (design-as-written · squares-with-the-project · what-a-revision-lost), then
> by consequence (wrong removal · unreachable truth · forced cost). Considered-and-dead
> entries and already-on-record entries stay in the file so nobody re-walks them.
>
> Read before this: root `README`/`DESIGN`/`IMPLEMENTATION`/`USER_STORY`/`AGENTS`/`KNOBS`,
> `spike/CLAUDE.md`, `Research/GOTCHAS.md`; then the document whole; then `Research/README`,
> the prior crosscheck record (`311p` adjudication, `311q` firming ledger, `311t` cross-root
> ledger, the four reports `311l`–`311o` by targeted search), `plans/30U`/`27C`/`30S` at the
> cited seats, and the document's history (`git log --follow`, the pre-STE100 text at
> `ca68b868^`, the q2 change at `b8aeb9eb`). Sibling reports `312cc`/`312cd` were not read.

## § 0-verdict

The model is better than its predecessors at the thing they got wrong — it never lets
structural inability to find overlap stand in for a declaration, and the human's
2026-09-19 acks (separation from one definition's own distinctions; collide widely) are
sound and worth keeping. It is not stable in the sense the word implies. Four things stand
against it as written, in order of consequence:

- The wrapper sentinel (`3.4`) mints SAME across a wrapper for every catalog sort the
  wrapper's author never heard of, and SAME feeds transport unflagged. The prior record
  found this (`311p` thread 7) and then dismissed it as "already harmless because § 2.7's
  default catches what it would carry" (`311t` § 14); that dismissal does not hold, because
  the sentinel makes the two instances *one* and observer-dependence only bites across
  *different* instances. This is the one place the document licenses a wrong answer with
  every statement true and no flag in front of it.
- The flat-scheme perishing sentence (`3.3`) admits a reading under which any
  `apt-get install`, `useradd`, or `systemctl enable` perishes every same-catalog
  resolution in the book — the package vocabulary, the user vocabulary, the unit
  vocabulary — and the document gives flat schemes no refinement. The narrow reading
  rescues the product and makes the sentence vacuous; the text does not say which.
- The region test (`2.9`) quantifies universally over "every mTraversal of D's mSort" with
  no non-empty guard, so a key with *no* traversal of that sort answers DISJOINT where
  `3.2`'s own unknown-link rule answers UNKNOWN. The pin that closes it (`277` § 5,
  inherited by `4.1`) is not restated at the seat that needs it.
- `2.7`'s mount-line example declares a correspondence across two hosts that `1.8` and
  `3.2` step 1 say cannot exist and that no party in `3.5`'s list can honestly make; the
  recorded reading (`311m` `held-cross-route-nfs`: UNKNOWN) contradicts the example that
  is still in the text.

Beyond those: `4.2` omits the as-built same-entity sparing algebra (`277` § 3,
`spike/CLAUDE.md` `sparing-algebra`, `30J`) that the model retires; the acked route back to
USER_STORY stage 5's `active`-against-files survival ("a stdlib key space for what sits
directly in a boot") is foreclosed by `2.2`'s own examples unless the stdlib keys the
machine's children in one scheme, which the text nowhere says; and the STE100 rewrite
dropped the only record of *why* the text chose "whatever is not DISJOINT collides" — the
refuted-shapes list now lives only in git, and a successor will re-derive the partition
that list killed. Everything else I suspected either died on re-walking or is already on
the record with a human ack; those are kept below, marked.

Blunt reading: promote to a plan only after the sentinel is priced like the finished
record, the flat-perishing sentence is disambiguated in the narrow direction, the region
test gets its non-empty guard, and the mount example is struck. None of the four is a
large edit; all four are the kind of thing that ships if nobody outside the room reads the
sentences literally.

## § 1-findings-design-as-written

### fnd-lend-sentinel-mints-same-past-a-stranger-and-the-net-does-not-catch-it

Sections: `3.4-entry-and-lends` ("mParent-Catalog mSorts not lent inherit the caller's
instance only after the wrapper's completion sentinel"; "The sentinel is an at-most claim
over every mParent-Catalog mSort"), `3.2-compare-one-chokepoint-four-answers` ("One instance
means one mPlaceholder: inherited through a wrapper's sentinel"), `2.8-observer-dependence-and-independence`
(the default), `3.5-committee-law-and-attribution` ("describes nothing they cannot see").
Confidence: `+SURE` of the walk under the text; `+SURE` that the record's dismissal
misreads `2.8`; `~SUSPECT` of frequency (the stranger must have minted a catalog sort the
wrapper genuinely perturbs — capabilities, seccomp, cgroup, SELinux context, umask are the
ordinary candidates).

Prior record: `311n` item 6 (`wrapper-closure-exceeds-knowledge`, hedged: "§2.7's default
observer-dependence *can* independently prevent transporting"), `311p` thread 7 ("the
sentinel's 'nothing else' is a finished-definition-class claim ... §3.4 should price it as
30U prices the record"), then `311q` § 10 ("the rest were punted or declined and are not
owed") and `311t` § 14 ("The wrapper's sentinel has the same form and is already harmless
because § 2.7's default catches what it would carry"). This finding is that last sentence
being wrong, and the consequence being a wrong SAME with no flag.

The book, the ordinary sudo-threaded one (`GOTCHAS` sudo-is-a-variable), with one
third-party oracle installed:

```sh
SUDO=sudo
cap-audit --report /etc/cap-audit/baseline.json || cap-audit --fix   # line 3: a fact from
                                                                     #   a vendor oracle
$SUDO cap-audit --report /etc/cap-audit/baseline.json || $SUDO cap-audit --fix   # line 7
```

World: `cap-audit` compares the *caller's* effective capability set against a baseline
(the vendor's tool; it reads `/proc/self/status`). Statements, every one true: sudo's oracle
lends `sm.User` and `sm.Group`, closes its sentinel — sudo genuinely changes nothing about
mounts, pids, or the network. The vendor's oracle mints `org.vendor.CapSet` as the catalog
sort its fact is looked up in (the fact is "baseline holds for *this* capability set") and
declares nothing about `sm.User`, because the vendor never met sudo's author.

The design's reading of line 7. `org.vendor.CapSet` is not lent by sudo; under `3.4` it
"inherits the caller's instance" after the sentinel. Line 3's fact was measured under the
ambient `org.vendor.CapSet` instance; line 7's site is under the inherited one — one
mPlaceholder — so `3.2`'s one-level rule reads them SAME with no warrant. Line 3 measured
converged; line 7 elides on line 3's fact. Root's effective capability set is not the
caller's; line 7 should have run.

Why `2.8`'s default does not catch it. The default says "a cell measured under a lent mKey
of O is assumed to depend on it" and then "never stands for the same mReferent under
another O-instance". Both halves are about *different* instances of O. Here the sentinel
has made the inside and outside `org.vendor.CapSet` instances *the same* placeholder, so
there is no other O-instance for the dependence to separate; the mTopic on both sides is
(key, the-one-instance). Observer-dependence guards the case where the wrapper *lends* O;
the sentinel's hole is exactly the case where it does not. `311n` saw this and hedged;
`311t` § 14 hardened the hedge into a dismissal.

Why it is a design fault and not an oracle fault. Sudo's author made a true statement about
everything they can see. The sentinel is defined (`3.4`) as an at-most claim over "every
mParent-Catalog mSort", an open set other authors extend after the sentence is written; by
`3.5`'s own law that is a statement about what its speaker cannot see. The document prices
the same shape on the write side (`2.6`: "Danger: the premature finished record", under
`--risk-faultless-skips`) and on the sameness side prices it nowhere: SAME is the transport
consumer and rides no flag (`spike/CLAUDE.md` compare-consumer-map; `KNOBS:kSURVIVAL` covers
DISJOINT only). `plans/30S` § 2 `rul-positive-speech-only` refuses exactly this shape on the
environment plane ("Open-world sensitivity lists are refused permanently: the one
enumeration nobody can complete is done by the shell instead"); `27C` (`spike/CLAUDE.md`
role-menu: "a MISSING dimension = ⊤, walls; absent-means-full-lend is REJECTED") refused it
for lends. `4.2` registers the `27C` invariance-line supersession and not this one.

Smallest repair: the sentinel closes only the catalog sorts the wrapper's owner *names* as
inherited (positive speech, `30S`'s posture); an unnamed catalog sort stays ⊤ across the
entry and walls facts looked up in it, which is `27C`'s law restated in the model's terms.
If the open-world inheritance is wanted for value, it rides the flag beside the finished
record, and `4.2` registers the `27C` flip.

### fnd-flat-perishing-eats-the-package-vocabulary

Sections: `3.3-perishing-three-mutator-species` (routing bullet: "under the flat default,
when its mParent-Catalog was touched at all"; "Creation, deletion, and rename of an mKey are
routing writes to its mParent-Catalog entry"), `2.9-hierarchical-and-the-region-test` ("By
default an mScheme is flat. The mTraversal is then the mParent-Catalog as a whole, and any
touch on it perishes every mResolution through it"), `2.6-may-write-the-writeset` ("A write
to an mKey is also a write to every container on that mKey's mFullyQualifiedKey"),
`1.7-resolution-and-its-traversal`. Confidence: `+SURE` the text admits the reading and
gives flat schemes no refinement; `~SUSPECT` the author intends the narrow reading. Prior
record: `311l` `amb-perish-touch-of-a-store` (the state-mutation bullet's "touches a
mParent-Store", filed "safe direction, no action") and `311p` § 8 `txt-shape-is-bytes-only`
("every `mount` line then perishes every file resolution below it under §2.8's flat
default", a cost noted in passing). The package/user/unit consequence is not on record.

The book (USER_STORY stage 6's own):

```sh
apt-get install -y nginx                                  # line 3: runs (diverged); a
                                                          #   routing write to the dpkg-db
                                                          #   entry `nginx`
dpkg -s ca-certificates >/dev/null 2>&1 || apt-get install -y ca-certificates   # line 8:
                                                          #   probed converged
```

World: nginx absent, ca-certificates present. Statements: dpkg's owner says canonical
package names are a flat scheme over the dpkg database (a package name is not
hierarchical); `install` creates the entry `nginx`; the finished record lists the package's
files and unit. All true.

Two sentences, two answers:

- The sparing test (`2.6` q1): chains meet at the dpkg store; two tops of one scheme under
  `:guarantees-unique-name`, differing; the store is `:aliases-nothing-else`. DISJOINT.
  Line 8 survives. Right.
- Perishing (`3.3`): line 8's mResolution of `ca-certificates` is through a flat scheme, so
  its mTraversal "is the mParent-Catalog as a whole"; the write to entry `nginx` "is also a
  write to every container on that mKey's mFullyQualifiedKey" (`2.6`) — the catalog — so the
  catalog "was touched at all", the resolution perishes, "every mFullyQualifiedKey built on a
  perished mResolution reads unknown below the line", and "dependent elisions demote to
  guards". Line 8 guards. Wrong.

Perishing is stated as unconditional and prior to the test, so on this reading it wins. The
same reading makes `useradd deploy` demote every user-keyed fact below it, `systemctl enable
foo` (a unit-table entry) every service fact, and `apt-get install anything` every `dpkg -s`
guard after it. The base library's vocabulary is flat, and `2.9`'s only refinement is
`:hierarchical`, which a package name, a login name, or a unit name cannot honestly be.
Stage 6's render and stage 5's `dpkg -s nginx` guard surviving `apt-get update` are
unreachable on it.

The narrow reading — an entry write touches only the entry, and "touched at all" means a
write *naming the catalog whole* — rescues the product and makes the flat-default sentence
vacuous (a traversal that *is* the catalog is already covered by "includes a touched mKey"),
and it needs `2.6`'s first sentence read as sparing-test-only (its own exclusion clause
suggests so: "contributes nothing to the *test* against that fact"). `3.3` and `1.7` never
cite that exclusion; they say "touches a mTraversal member" and "touched at all". The text
under-specifies in the direction that kills the product.

Smallest repair: one sentence in `3.3` — an entry write perishes only mResolutions whose
mTraversal includes *that entry* (hierarchical) or *that catalog named whole* (flat); the
container-write of `2.6` feeds entailment, never perishing. And say which sentence wins when
the two disagree.

### fnd-region-test-quantifies-over-an-absent-traversal

Sections: `2.9-hierarchical-and-the-region-test` (step 3: "on every mTraversal of D's
mSort, every level compares DISJOINT with D and every level emitted its closure"; "Only x's
mTraversals are walked"), `2.10-places-the-upward-lookup` (the three invocation conditions),
`3.2` ("Partial measurement never widens"; "If either mFullyQualifiedKey contains an unknown
link, the pair reads UNKNOWN"), `4.1` ("uses, and does not redefine ... the universal
meet"). Confidence: `~SUSPECT` — the wrong answer needs q2 to pass on a false finished
record, so it lands on the registered knife; the finding is that q1 contributes a false
DISJOINT of its own where it should decline, and the inherited pin that forbids that
(`277` § 5 inv-top-never-encoded-as-empty, quoted in `spike/CLAUDE.md`
set-lifting-universal-meet) is not restated at the seat. Prior record: the same lesson at a
different seat — `311q` § 15 `cliff-nothing-else-on-a-floor-sort` ("a finished definition
over an empty placement set spares every cross-sort pair; wrong DISJOINT ... refusing one
over zero placements and no store is mechanical"); `311t` § 15 `rul-the-outside-rule-with-the-alias-closure`
states the four outcomes with "every level compares DISJOINT with D and every level is
closed" and never asks whether there is a level.

The book:

```sh
rm -rf /var/cache/app/*                                   # line 4: runs; the fs binder's
                                                          #   writeset = /var/cache/app given
                                                          #   whole
app status --json | jq -e '.warm' >/dev/null || app warm  # line 9: probed converged; the
                                                          #   app oracle marks a cell of
                                                          #   org.app.Instance:default
```

World: the app's warm-state is held in `/var/cache/app/index`; the app oracle's author
closed the cell's may-read set and listed the app's config file but not the cache (the
registered knife, `2.6` "the premature finished record"). Every other statement true.

`2.6` asks q1 then q2. q1 is the region test: x = the cell, D's sort is path; x has no
path-traversal because no lookup of x ever passed through a path — not because a lookup
found none. `2.10` fires the placing lookup only when the placing sort "declares that it
places T"; nobody declared that paths place app-instances. Step 3's universal is vacuously
true over zero traversals: DISJOINT. q2: the readset is closed and the cache is absent from
it; every writeset entry compares DISJOINT with every readset entry (config file inode vs
the cache directory: two-tops, closed). Line 9 survives line 4 and should not.

What is the author's fault and what is the model's. The omitted cache is the author's
false closure, priced and attributed. But `3.2` would read the *pair itself* UNKNOWN (x's
chain never reaches a path level, so against a path key the pair has an unknown link), and
`2.6` sends a key-given-whole to `2.9` instead, where the same pair reads DISJOINT because
the universal is over an empty set. The text does not distinguish "x has no route of sort G
because none was computed" from "x has a closed route of sort G with zero entries" — only
the second may satisfy step 3, and `2.10`'s trigger conflates them ("has no route of mSort
G"). The engine has the information (was a G-lookup run on x, and did it close?) and the
rule does not consult it.

Smallest repair: step 3 requires at least one mTraversal of D's mSort on x, and that it be
closed; zero traversals reads UNKNOWN — the inherited pin, restated where it is consumed.

### fnd-correspondence-speaks-across-worlds-from-one-vantage

Sections: `2.7-corresponds-across-a-transition` (the mount-line example; "the model's only
declared sameness generator besides mToken equality"), `3.2` (step 1 "two mRoutes across a
transit" read UNKNOWN; "SAME is 'or' across mDerivations"), `1.8` ("Nothing speaks across
mWorlds"), `1.10` ("Across mVantages it is unknown"), `3.5` (the committee list).
Confidence: `+SURE` the three sentences contradict; `+SURE` the mount example hands the
transition owner a statement they cannot make; `~SUSPECT` on how much product rides on it.
Prior record: `311m` `held-cross-route-nfs` ("Two hosts, one export, one book: two mRoutes,
UNKNOWN, nothing spared across worlds. Correct and blunt") — the recorded reading is
UNKNOWN; the mount example was already in the text (pre-STE100 `2.6-corresponds`) and no
pass flagged that it says otherwise.

The book, two targets in one fleet run:

```sh
# on nfs1 (the server), earlier in the run:
install -m 0644 ./site.conf /srv/export/site.conf                       # line S: runs
# on web1 (a client), later:
[ -f /mnt/site/site.conf ] || install -m 0644 ./site.conf /mnt/site/site.conf   # line C
```

World: `web1:/mnt/site` is `nfs1.corp:/srv/export`. The mount oracle on `web1` reads
`/proc/mounts` and declares, as `2.7`'s example instructs, that mKeys under `/mnt/site`
`:correspond` to mKeys under `/srv/export` on `nfs1.corp` from this mVantage. True.

Three readings the text licenses at once:

- `1.8` and `3.2` step 1: two mRoutes across a transit; UNKNOWN; nothing speaks across
  mWorlds. Line C guards. Safe, and what `311m` recorded.
- `3.2` "SAME is 'or' across mDerivations" with `2.7`: the mCorrespondence is a second
  mDerivation and yields SAME whatever the mFullyQualifiedKey yields. Then a fact measured on
  `nfs1` stands for the same file on `web1` — transport, the dangerous direction — on a
  vouch-tier claim.
- The statement itself. The mount oracle sees `nfs1.corp` in a mount line and can say
  "served by whatever `nfs1.corp` resolves to *from web1*". It cannot say that this is the
  fleet target Dorc calls `nfs1`: that is one name compared across two mVantages (`GOTCHAS`
  a-name-resolves-from-a-vantage, a-jump-host-moves-the-vantage), which `1.10` calls
  unknown. No party in `3.5`'s list owns the completing statement, and an engine that makes
  it is `4.1`'s excluded engine-generated SAME. The example hands the transition owner a
  warrant they cannot honestly issue.

Smallest repair: strike the mount example from `2.7` (the container-pid and `sudo -u`
examples are within one mRoute and stand), and add to `3.2` that a mCorrespondence whose two
mParents terminate at different mRoutes is refused at declaration.

### fnd-route-examples-foreclose-the-acked-repair

Sections: `2.2-primary-of-and-identified-in` ("an ext4 filesystem in the mRoute, an NFS
filesystem in a host, a tmpfs in a boot"), `1.6` ("A shape with no `:identified-in` is
scoped in the mRoute"), `3.2` step 3 (the two ways), `3.3` (the boot as a mRoot-adjacent
mKey). Confidence: `+SURE` of the walk; `~SUSPECT` that it is a gap rather than a deliberate
deferral. Prior record, heavy: `311t` § 14 `fnd-address-first-dissolves-the-two-readings`
("Cost: USER_STORY stage 5 keeps its file-backed survivals through inodes and loses `active`
against files until a stdlib key space exists for what sits directly in a boot. Not small;
touches `30U`") with the human's ack of the route and hard ack that "an author who mints an
mSort and hangs it on nothing comparable stays guard-only"; `311q` § 18 (the human: "every
sort walls everything around it until it is mapped into the filesystem, spiritually,
allowing for the few roots"); the pre-STE100 `4.2` kill of "A PARENT PARTITIONING ITS
CHILDREN'S mSorts ... Surviving form: both sides identify into one mScheme whose body
warrants `:guarantees-unique-name`". So the loss is acked and the repair direction is
recorded. What is not recorded: that the document's own examples put the repair out of
reach.

The book (USER_STORY stage 5):

```sh
apt-get update                                  # line 5: runs (index stale); writeset =
                                                #   the apt lists directory, given whole
systemctl enable --now nginx                    # line 9: probed converged; needs
                                                #   nginx@enabled AND nginx@active to survive
```

`enabled` is a symlink inode in the root filesystem: two-tops at the filesystem; survives.
`active` is held in the service manager's memory in the boot (`1.9`'s own example). Its
chain: cell → `sm.Service:nginx` → manager → boot → …; the lists directory's: inode → root
filesystem → …. For a way to hold at their meeting level, the two tops must be "mKeys of
one mScheme" with `:guarantees-unique-name` — the recorded repair, "a stdlib key space for
what sits directly in a boot". Under `2.2`'s examples the root filesystem's shape is
`:identified-in` nothing (scoped in the mRoute) and the boot is a separate mRoot-adjacent
key; they are keys of two schemes meeting at a terminus nobody owns, and no later
declaration by the stdlib can make a disk filesystem and a boot two shapes of one scheme
without contradicting the examples. The one-top way is closed too (neither leaf is its own
top). KNOWN_UNSPOKEN, by the text's own placement of its top-level objects.

Smallest repair: one sentence saying the stdlib keys the machine's immediate children (its
boots, disk filesystems, the machine's own namespaces) as shapes of *one* primary scheme
with `:guarantees-unique-name`, and correct `2.2`'s example to "an ext4 filesystem in the
machine". Without it, the acked route back to stage 5 is not in the model that is being
promoted.

## § 2-findings-squaring-with-the-project

### fnd-register-omits-the-as-built-sparing-algebra

Sections: `4.1` ("The model excludes a selector dialect, an aspect species"),
`4.2-supersessions-pending-in-prior-documents` (no entry names `277`, `30J`, or
`spike/CLAUDE.md`). Confidence: `+SURE` of the omission (`grep` for `277`, `30J`,
`selector` in the document: two hits, both `4.1`/`4.2` prose, neither a register entry).

`spike/CLAUDE.md` `sparing-algebra` is steering law: "same-entity, a claim SPARES a backing
iff BOTH sides carry minted selectors AND claim-token ∈ dialect(...) AND claim ≠ backing."
Under the model, two cells of one mParent are two singleton sorts, both their own tops at
the parent, neither of `3.2`'s two ways applies, and they read KNOWN_UNSPOKEN — `311t` § 15
records the retraction plainly ("two singletons of two sorts under one parent read
known-unspoken under § 3.2, and only the parent's owner keying them in one scheme with
`:guarantees-unique-name` separates them"; residue punted to 312). That is a supersession of
`277` § 3 as amended by `279f`, of `30J` § 12's dialect keying, and of the as-built
`oracle::build_dialect`. `4.2` registers `272` § 3 and § 5, `30W`, `30T`, `30U`, `27C`, and
`ANALYZER-NEEDS`, and not this — the one supersession that has code behind it. `311q` § 18
says "30J neither in nor out". A register that names every superseded document but the one
with a built implementation is the register a builder will not find.

Repair: one `4.2` entry naming `277` § 3 / `spike/CLAUDE.md` sparing-algebra / `30J` § 12,
stating "Here: cells of one mParent separate only where the parent's owner keys them in one
scheme under `:guarantees-unique-name`; no selector dialect exists."

### fnd-sentinel-posture-inverts-thirty-s

Sections: `3.4`; `plans/30S` § 2 `rul-positive-speech-only`, `rul-transport-wall-alone-is-banned`.
Confidence: `+SURE` of the two texts; a squaring note that sharpens
`fnd-lend-sentinel-mints-same-past-a-stranger-and-the-net-does-not-catch-it` rather than a
separate fault. `30S` (ruled 2026-08-24) refuses open-world at-most enumerations on the
environment plane — "authors owe POSITIVE speech only ... the at-most complement is
performed by the CONSTRUCT, never enumerated by a person" — and `30S` is the document `311p`
thread 6 leans on for the `kubectl`/`export` case. `3.4`'s sentinel is an author-enumerated
at-most complement over an open set of sorts. The two documents hold opposite postures on
the same shape of claim, one ruled, one about to be promoted; `4.2` registers no relation
between them.

## § 3-findings-what-a-revision-lost

### fnd-refuted-shapes-live-only-in-git

Confidence: `+SURE` of the history; a footnote by the brief's weighting. Commit `ca68b868`
("Rewrite the whole model in plain STE100, dropping the supersedes list and the
forcing-function sections") removed the pre-rewrite `§ 4-forcing-functions`: the
four-gotcha litmus (now folded into § 0), the twenty-four-line gotcha→element map
(`4.1-slugged-gotchas-and-what-each-forces`), and the forty-odd-line refuted-shapes list
(`4.2-refuted-shapes-and-what-killed-each`). `311t` § 15 records the human's leave to drop
it "for tightness". Two entries on that list are the only statement anywhere of *why* the
promoted text reads as it does: "ONLY SAME may-read entries OVERLAP. Killed by ... Surviving
form: whatever is not DISJOINT collides" (the reading `311q` § 8's cell-sparing argument
assumed and the promoted `2.6` abandons), and "A PARENT PARTITIONING ITS CHILDREN'S mSorts.
Killed by: children minted by other definitions could never spare" (the repair `311q` § 18
proposes and `fnd-route-examples-foreclose-the-acked-repair` needs a cousin of). A
successor reading the promoted text and `311q` will re-derive the partition and re-argue
the overlap reading. The `plans/` convention is ahistorical rewriting, so the loss is by
design; the cost is that the two kills most likely to be re-litigated are now findable only
by `git show ca68b868^:Research/notes/311-identity-and-relation-model.md`. Repair, if any:
carry those two kills as one-line blockquotes under `2.6` and `3.2`.

## § 4-considered-and-dead

Each was walked as a candidate wrong answer and died; kept so nobody re-walks it.

- `dead-aliasing-stores-forfeit-containers-and-nfs` — I first read `3.2` step 4 as
  demanding `:aliases-nothing-else` of the filesystem itself for two files in one overlay or
  NFS mount, which would forfeit every same-filesystem survival on a container root. Wrong
  walk: the filesystem *is* A, and step 4 asks stores "strictly below A, down to either
  leaf's mParent" — for two inodes directly in one filesystem that set is empty, and the
  two-tops way under the filesystem's own `:guarantees-unique-name` separates them. `311p`
  thread 1 found and repaired exactly this in the pre-`0b66e890` text. Dead.
- `dead-rootness-has-no-smaller-scope` — `2.2` says a duplicable token "must be scoped in
  something smaller, or left un-warranted", and every smaller scope for a cloud account id
  (endpoint, TLS peer, resolved address) is a name from a vantage. I read that as an empty
  retreat. It is not: scoping under an *unwarranted* endpoint makes the chain carry an
  unwarranted level, and "SAME is 'and' within one mFullyQualifiedKey" then yields UNKNOWN —
  the honest answer for localstack-presents-the-real-id. `311t` § 6 walked this
  ("an emulator presenting the real id can never mint SAME, and nothing mints DISJOINT
  there"). Dead; the sentence works by withholding, not granting.
- `dead-held-in-is-said-twice` — a cell's may-read set says "my state is held in `status`";
  the entailment must say it again on the write side, and a finished record omitting it is
  a wrong DISJOINT the model could have caught from the may-read. `311t` § 15
  `fnd-the-mirror-rule-is-not-derivable-from-may-read` and the human's objection ("a user
  typed may-read; you are describing something that may-writes") settle it: may-read is
  influence toward the cell (a resolver cache may-reads `/etc/hosts` and a query that fills
  the cache writes nothing to the file), so no coherence check can be derived from it; the
  write-side sentence is the sort-level may-write, same owner, one line. Dead, human-ruled.
- `dead-cells-separate-on-a-warrant-nobody-holds` — the second horn of my cell suspicion:
  if two cells of one parent separate by the two-tops way on the parent scheme's
  `:guarantees-unique-name`, a stranger's cell for the same in-memory state (`@org.other.Running`
  beside `@sm.Active`) would be wrongly spared on a warrant the parent's owner cannot hold.
  Dead: a stranger's cell is a key of a *different* scheme, no way applies, KNOWN_UNSPOKEN.
  The first horn (cells never separate as `1.9` defines them) stands and is on record — § 5.
- `dead-region-test-walks-only-one-side` — `2.9` walks only x's traversals and asks D for no
  closure; a bind mount *from* an ancestor of x *into* D's subtree looked like an escape.
  It is not: the mount's target being elsewhere still makes x's ancestor "exposed by another
  mount", so its closure is honestly withheld under `1.5`'s mount-table evidence
  (`311t` § 15 corrected the evidence to `mountinfo`). Dead; `311m` lists bind mounts and
  hardlinks among the attacks that held.
- `dead-container-exclusion-licenses-a-wrong-disjoint` — `2.6`'s "a container at or above the
  deepest shared level contributes nothing" looked like it could drop an entailment that
  reached the fact (a directory's mtime; a filesystem's free-space cell; the loop image).
  Every construction dies on the walk: a cell of the container is the container's own top,
  so against a child of a different scheme neither way holds and the pair reads UNKNOWN; a
  loop image is either A itself (UNKNOWN) or strictly below the shared level (its
  entailment joins). Dead.
- `dead-same-at-the-leaf-without-parent-same` — the one-level rule reads equal tokens under
  `:guarantees-unique-referent` as SAME; two filesystems each holding inode 1234 would be
  SAME at the leaf with parents not SAME. The warrant is "within one mParent" (`1.5`), and
  even misapplied the walk's "deepest SAME level" would be the leaf, where "either mKey is A
  itself" reads UNKNOWN. Dead; a clarity nit at most.
- `dead-observer-independence-is-quadratic` — `2.8`'s arity per (mSort, O) contradicts
  `3.5`'s "grows with the number of authors, never with the number of pairs of them". True
  as a textual contradiction, safe in direction, `-GUESS`-grade on record (`311l`
  `fnd-observer-sorts-must-be-enumerated`: "safe, recorded because the seat is the same one
  the model rejected for axes"). Demoted to a nit: strike the linearity sentence or scope
  it to positive generators.
- `dead-transit-ambiguity-makes-remote-hosts-inert` — "Across a transit the mRoute is never
  vouched" read as if every ssh-reached host had an unvouched mRoute, making the model inert
  for the push product. The intended reading (transit-free = local to the probe's own
  execution on that host; the standup `witness()` bridges probe and apply sessions) is
  recoverable from `1.10`, and `311p` thread 7's repair states it. Dead; a wording nit.
- `dead-perishing-of-a-may-read-entry` — a routing write to a may-read entry's traversal
  (`mv /srv /srv2` under a loop image) leaves the fact true and the model collides it. Safe
  direction, over-conservative, by design. Dead.
- `dead-declined-yields-wall-sockets` — a path reaching a socket declines under the file
  scheme (`2.1`), so `rm -f /run/x.sock` walls everything below it until a socket sort
  exists. Cost, not wrongness; reachable by binder work. Dead.

## § 5-already-on-record

Objections I raised in step 2 that the corpus had already found; nodded, footnoted, not
re-argued.

- `rec-cells-of-one-parent-never-separate` — `311p` threads 2 and 3; `311t` § 15's
  retraction ("two singletons of two sorts under one parent read known-unspoken under § 3.2,
  and only the parent's owner keying them in one scheme with `:guarantees-unique-name`
  separates them"); residue punted to 312. Residual nuance: `1.9` gives a cell no scheme, so
  the recorded escape is available only by *not* using a cell — the text should say that
  `chmod`-past-contents and `enable`-past-`active` need the owner's sub-sorts under one
  scheme, not cells. Folded into `fnd-register-omits-the-as-built-sparing-algebra`.
- `rec-separation-dies-at-the-route` — `311t` § 14, the human's ack of address-first and of
  "a sort hung on nothing comparable walls"; `311q` § 18. The loss of USER_STORY stage 5's
  `active`-against-files is acked. Not registered in `4.2` because root docs are excluded by
  policy ("root docs excluded, known stale", `311t` § 14). The surviving residual is
  `fnd-route-examples-foreclose-the-acked-repair`. One tension the record names and does not
  resolve: `KNOBS:kSURVIVAL` ("never unshipped short of product death") and
  `28T:post-survival-is-the-product` were typed about a tier whose drifted-morning headline
  this model gives up until the stdlib key space exists.
- `rec-survival-attribution-omits-correspondence` — `311p` § 8 `txt-same-then-disjoint-attribution`
  (from `311m`); declined by the human at `311q` § 10 ("attribution of every link is
  standing implementation law and is not restated per relation"). Noted only that
  `a368430d` then *did* restate route closures in `3.5`'s survival list, so the list is
  now neither complete nor absent; either finish it (add SAMEs consumed by composition) or
  delete it to match the ruling.
- `rec-lend-sentinel-is-an-open-world-claim` — `311n` 6, `311p` thread 7, punted at
  `311q` § 10. The novel part is the dismissal being wrong; see § 1's first finding.
- `rec-flat-default-perishing-cost` — `311p` § 8 `txt-shape-is-bytes-only` notes the flat
  default's cost for `mount` lines; `311l` `amb-perish-touch-of-a-store` the state-mutation
  bullet. The routing-bullet reading and its package/user/unit consequence are § 1's second
  finding.
- `rec-emulated-authority` — `311t` § 6–§ 8 (the exercise; GOTCHA 59 minted at `311q` § 18).
  Handled by arrangement; see the dead entry above.
- `rec-two-schemes-canary-deleted` — `115438b5`, `311p` § 2; not re-raised.

## § 6-coverage

Read whole: the document; root `README`, `DESIGN`, `IMPLEMENTATION`, `USER_STORY`,
`AGENTS`, `KNOBS`; `spike/CLAUDE.md`; `Research/GOTCHAS.md`; `Research/README`; `311p`;
`311q`; the pre-STE100 text's `§ 3.5`–`§ 4.2`. Read at the cited seats: `311t` § 6–§ 7,
§ 14–§ 15; `311l`/`311m`/`311n` by search (perish, corresponds/NFS, sentinel, observer);
`plans/30U` § 4–§ 9; `plans/27C` (the enumerate-everything/flag passage); `plans/30S` § 2;
the `b8aeb9eb` and `5ec215db` diffs. Not read: `311l`–`311o` whole; `30W`, `30T`, `272`,
`277` whole (their supersessions are registered in `4.2` and re-adjudicated in `311p`/`311t`,
and my findings do not turn on their text beyond what `spike/CLAUDE.md` quotes); sibling
reports `312cc`/`312cd` (concurrent, deliberately unread); anything under
`quarantine-DO-NOT-READ`, `corpora`, or `29*`; `LIVING_STATUS`. No scout was spawned; no
web search was used; no repository script, fixture, or task was executed.

Sections of the document walked against at least one book: `1.1`–`1.11`, `2.1`–`2.11`,
`3.1`–`3.5`, `4.1`–`4.2`. Sections where I found nothing and looked hardest: `2.3` (the
self-knowledge reading holds once `3.2` step 4 is read correctly), `2.11` (composite sorts;
base-and-overlay ordering behaves), `3.1`. `GOTCHAS` entries checked against the model
beyond the ones the document maps: 3, 4, 5, 6, 8, 12, 13, 17, 18, 20, 21, 22, 23, 29, 30,
31, 34, 44, 48, 51, 59, 60, 62, 63 — none mishandled beyond what § 1 states; 23/24/25
(hooks, postinst, late effects) are at-most-claim truth, not identity.

What this pass could not do: build the sentinel case against a real third-party oracle
(none exists; the capability-set sort is my invention, marked as such), or test the flat
perishing reading against the spike (the spike implements the predecessor design and is
not the target). Both are `~SUSPECT` on frequency and `+SURE` on the text.

## § 7-head-at-start-and-end

- Start: `6b7108b4ccabc3d961508d98a563ebc06616623b`.
- End: `7fe622830ae92a89a79386a59d1fa2c655a62cbf` — three commits landed during the review
  (`b164565a`, `b9a5ca3d`: the sibling astra reports filed; `7fe62283`: the quarantined
  prompt kit). `git diff --stat 6b7108b4..7fe62283 -- Research/notes/311-identity-and-relation-model.md`
  is empty: the reviewed text is byte-identical at both.
