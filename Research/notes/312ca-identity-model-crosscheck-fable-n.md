# 312ca — Identity-model crosscheck (neutral pass, Fable)

> AI-authored review of `notes/311-identity-and-relation-model.md`, read-only, static. HEAD at
> start: `7fe622830ae92a89a79386a59d1fa2c655a62cbf` (the model last touched at `6b7108b4`; no
> commit since has touched it). HEAD at end: `1d6288077c3e120bdb0a676c589b4c599e4c1357`; the
> two commits between (`a4257b2b`, `1d628807`) touched only the quarantined prompt kit,
> `SLUGS.md`, and the sibling `312cb` review — nothing this review read. Nothing here is ruled. Every
> claim carries `+SURE` / `~SUSPECT` / `-GUESS`; a finding against a `[LEAN]` line says so.
> Method: breadth-first stubs, then depth one at a time; each front is on disk before the next
> is opened.

## suspicions

(the breadth pass's stubs, kept as the trail; every one is dispositioned — the table says
where each went)

| stub | disposition |
|---|---|
| stale-index-morning-unreachable-under-walk | → fnd-in-memory-state-never-spares-past-a-file-write (folded with heterogeneous-stores-never-spare) |
| one-top-way-underspecified | → dead (human-acked; text nit) |
| container-subtraction-blinds-container-cells | → dead (safe; text nit) |
| placing-lookup-set-valued-disagreement | → fnd-placing-lookup-disagreement-undefined-over-sets |
| environment-is-catalog-or-shell-state | → dead (`30S` + measure-in-context) |
| link-count-is-not-closure-evidence | → fnd-link-count-is-not-closure-evidence |
| indexical-knowledge-is-not-the-path-owners | → dead (one stdlib seat) |
| absence-fact-topic-unspecified | → fnd-absence-fact-has-no-stated-topic |
| same-parent-cells-never-separate | → on record (`311p` thread 3, `311t:1192`); the novel residue is fnd-sparing-algebra-supersession-unregistered |
| nothing-across-mworlds-versus-correspondence | → fnd-mworld-fence-contradicts-the-correspondence-example |
| route-terminus-readset-never-closed | → fnd-readset-closure-undefined-at-the-terminus |
| resolve-readset-open-perishes-how | → fnd-perish-set-of-an-unclosed-resolve-unstated |
| shell-builtins-writeset-author-unnamed | → dead (`30S` engine-owned; completeness clause) |
| law-forbids-what-the-flag-buys | → dead (flag's object narrows; naming footnote) |
| heterogeneous-stores-never-spare | → folded into fnd-in-memory-state-never-spares-past-a-file-write |
| finished-writeset-needs-both-closures | → fnd-finished-writeset-needs-both-records-unstated |
| lookup-chains-force-second-round-trip | → dead (sequenced in one ship; integrity-only re-read) |

The original stubs follow, unedited.

- stale-index-morning-unreachable-under-walk · 3.2-compare-one-chokepoint-four-answers,
  2.6-may-write-the-writeset, 4.2 · USER_STORY stage 5 (human-reviewed) has `apt-get update`
  (footprint `sm.dorc.PkgIndex`, entity-less whole-kind) sparing `sm.dorc.Package:nginx`,
  `sm.dorc.File:/etc/nginx/nginx.conf`, service state, on the finished definition alone. 311
  says the finished definition generates no DISJOINT and a cross-mSort pair not separated by
  the walk reads KNOWN_UNSPOKEN. Can the walk separate an entity-less cell under the mRoute
  from a package identified in the dpkg database? If not, the headline render is lost and the
  4.2 register does not name USER_STORY.
- one-top-way-underspecified · 3.2-compare-one-chokepoint-four-answers step 3 · The "One top"
  separation way (x is its own top under A; x's primary mScheme declares, for SOME OTHER
  shape, `:identified-in` the mSort of y's top) is stated without an instance and I cannot
  build one where it is both needed and sound. "An omission is a distinction only inside the
  body that made it" reads as: because the author knew of mSort M and did not scope this shape
  in M, x is not inside any M-instance. That is a negative inference from silence about the
  OTHER side's container, which § 0 and silence-licenses-nothing forbid. Needs an instance or
  deletion.
- container-subtraction-blinds-container-cells · 2.6-may-write-the-writeset · "A container at
  or above the deepest level the written mKey shares with the read mKey contributes nothing."
  A write to inode 77 in dir 10 is thereby invisible to a cell OF dir 10 (`10@sm.Mtime`,
  `10@sm.Nlink`), which meets 77 at A=10 and then separates as two tops. Only the cell owner's
  may-read (the directory given whole) rescues it. Check whether the model states that this
  burden falls on the container-cell owner, and whether "two tops of one mScheme" can even be
  evaluated between a cell mKey and an inode mKey.
- placing-lookup-set-valued-disagreement · 2.10-places-the-upward-lookup · "Two answers that
  disagree are refused and attributed to G's owner." Under a bind mount one inode is placed in
  two mounts; invoked with `/mnt/a/f` and `/srv/a/f` a correct Mount lookup answers two
  different `looked-up-in` records. If "disagree" means unequal, a true lookup is refused and
  the refusal is attributed to an author who said nothing false (the pope-sin of
  `271:rul-sin-ordering`). Needs: disagreement defined over sets, or the closure alone.
- environment-is-catalog-or-shell-state · 3.4-entry-and-lends, 3.3, 1.10 · The environment is
  named as "shell state a resolve() read" (perishable) and must also be an mParent-Catalog mSort
  for `sudo`'s scrub (GOTCHAS a-wrapper-scrubs-what-an-outer-wrapper-lent) and
  `AWS_PROFILE=a aws` (an-env-variable-selects-the-referent) to be lend-able. The model does not
  say which, or both; if only the former, a wrapper that scrubs cannot be described truly.
- link-count-is-not-closure-evidence · 1.5-token-and-the-two-warrants, 2.9 step 3 · The model
  itself states the evidence for a file's `alias nothing-else` closure: "a link count of one".
  A magic link (`/proc/$pid/cwd/x`, `/proc/self/fd/3`, `/dev/fd/N`) reaches the inode without a
  directory entry and without passing through the containing directory, so the region test's
  every-level defence does not fire and step 3 can answer DISJOINT for D=`/a` whole vs x
  reached through the magic link while 77 sits inside `/a`. The false statement is the
  model's own evidence rule, not an author's. `-GUESS` on how the traversal records a magic
  link; verify.
- indexical-knowledge-is-not-the-path-owners · 2.9-hierarchical-and-the-region-test · "Only
  the mScheme owner can say which [components are indexical], in the lookup that meets them."
  That `/proc/sys/net` is per-netns (GOTCHAS net-sysctls-are-per-namespace) and `/proc/self`
  per-process is procfs knowledge, held by the store's describer, not by the path mScheme's
  owner. Under 3.5 each party speaks only of their own thing; the model here assigns the
  warrant to a party who cannot know it. Also: "an undeclared indexical component reads
  unknown" is unenforceable, since the engine cannot tell an undeclared indexical from a
  plain component.
- absence-fact-topic-unspecified · 1.9-cell-a-singleton-sort, 3.3, 1.11 · A verdict that
  asserts absence (`[ ! -e /x ]`, `dpkg -s foo` rc 1, `systemctl is-enabled foo` on no unit —
  GOTCHAS a-read-folds-absent-into-a-value) has no mResolution and so no mFullyQualifiedKey
  for its leaf. 3.3 names "its mParent-Catalog entry, its existence cell", but 1.9's cell is a
  SINGLETON under an mParent, whereas the entry `x` in directory 5 is one of many keyed by
  name. What is the mTopic of an absence fact, and who declares the existence cell's mSort?
- same-parent-cells-never-separate · 1.9-cell-a-singleton-sort, 3.2 step 3, 1.2, 4.2 ·
  `systemctl start nginx` (writeset `nginx@sm.Active`) above a measured `nginx@sm.Enabled`
  fact. 1.9 sends two cells of one mParent to 3.2 "as between any two mSorts"; 1.2 says the
  engine never assumes two mSorts disjoint; 3.2's two ways need "mKeys of one mScheme" or a
  self-top with an `:identified-in` distinction, and a cell has no mScheme. So the pair reads
  KNOWN_UNSPOKEN and `enabled` cannot survive `start`. The spike's `277` §3 sparing-algebra
  (spike/CLAUDE.md) spares exactly this pair by selector inequality within one dialect; 4.2
  does not register 277 §3. The owner who minted two cells under one parent is the one party
  who can say they are two things; the model gives them no way to say it, so this is a
  "no sane spelling possible" ergonomics fault plus an unregistered supersession.
- nothing-across-mworlds-versus-correspondence · 1.8, 3.2 step 1, 2.7 · "Nothing speaks across
  mWorlds" (twice) versus 2.7's mount-line mCorrespondence bridging a client mRoute to a
  server named in the DNS mRoot, and 3.2's "SAME is or across mDerivations". Either the
  mCorrespondence is an exception to the mWorld fence (then say so, and say the fence binds
  only the mFullyQualifiedKey derivation and the finished record) or the NFS example in 2.7
  is unreachable.
- route-terminus-readset-never-closed · 2.5-may-read-the-readset, 2.6 step 2, 1.3, 1.10 · A
  fact's readset "is closed only when every declared set is closed" yet the may-read default is
  ⊤. Every mFullyQualifiedKey terminates at the mRoute or a mRoot, whose mSort (the floor
  "mSort nobody has named", or the engine-vouched mRoute) has no owner and no may-read
  declaration. Read as "undeclared = ⊤", no readset ever closes and 2.6 step 2 never spares;
  read as "undeclared is skipped", the sentence "Default: ⊤, which collides with everything"
  is false at the terminus. The model must pick, and if the engine vouches the mRoute's
  may-read as closed-empty it must say so (it is a positive statement by the engine).
- resolve-readset-open-perishes-how · 3.3-perishing-three-mutator-species, 4.1 · A
  mResolution perishes "when its resolve() read the written state". The engine knows a
  resolve() body's reads only through the read-set closure (`27C` §4(a)(B)), which is
  default-disqualify. What is the perish set of a resolution whose resolve() body fails the
  closure: ⊤ (perishes on any write), or the body's marked reads only? Unstated; the former is
  safe and costs every resolution through an unclosed lookup, the latter is a silent channel.
- shell-builtins-writeset-author-unnamed · 3.3, 3.5, 1.5 · `cd`, `export`, `X=y`, `umask`,
  `exec 3>f` are routing writes to "shell state a resolve() read", but 3.5 says every positive
  step is one author's line and the engine only chains and meets. The engine is the party who
  knows these builtins' writesets (they are sh semantics, not tool claims); 1.5 licenses the
  engine to vouch only the transit-free mRoute. Either the engine's sh-semantics vouch is a
  second engine statement (name it) or builtin writes are ⊤ walls (state it).
- law-forbids-what-the-flag-buys · 0-the-problem-and-the-law, 2.6, 3.2, 4.2 ·
  IMPLEMENTATION ("Survival") defines `--risk-faultless-skips` as consent to exactly the
  class "nobody did something locally incorrect; but the net effect of several people's
  choices was incorrect elision" — the frame problem's "and nothing else" residue that no
  line can say (`spike/CLAUDE.md rul-flag-is-razor-residue`). § 0's law makes every DISJOINT
  rest on a warrant some party gave, and 2.6 strips the finished definition of any
  DISJOINT-generating power, so no answer the model returns can be wrong-with-no-false-
  statement. Then the flag licenses nothing: every survival already rests on named
  warrants (3.5). Either the model has silently retired the flag's object, contradicting the
  human-authored root docs (not in 4.2), or the strangers residue is meant to re-enter
  somewhere and the model does not say where.
- heterogeneous-stores-never-spare · 3.2 step 3, 2.6, USER_STORY stage 5 · Generalises
  stale-index-morning-unreachable-under-walk. Two facts whose chains pass through different
  store species (dpkg database vs unit table vs netfilter vs a filesystem) meet at the mRoute
  and their tops are keys of different mSchemes; neither of 3.2's two ways applies; the pair
  reads KNOWN_UNSPOKEN however completely everyone has spoken. So `foobar sync-certs` (a
  path-rooted sort) can never spare `sm.Service:nginx@active`, and `apt-get update` can never
  spare service or firewall facts. That is the whole stage-5 product on drifted days,
  `KNOBS:kSURVIVAL` "the whole stage-5–7 product", and `kHALVES` welded far toward elide.
  Not registered in 4.2 as a consequence.
- finished-writeset-needs-both-closures · 2.6-may-write-the-writeset · The at-most set is
  "closed by the completion record" (the verb author's) and the entailment is finished by
  "the reached completion record" (the mSort owner's); step 2 says "The writeset's definition
  is finished" as one condition. Say whether both records are required per entailed mSort
  (`30U` did), or one suffices; the difference is a silent channel.
- lookup-chains-force-second-round-trip · 1.4, 2.10, 1.10 · Every lookup is a measurement;
  a `:places` lookup is invoked with the mKeys the engine holds for a referent, which for a
  captured mValue are bound only after the first probe returns. The apply standup re-reads
  every placeholder. Both imply a second host exchange whose inputs depend on the first
  (`spike/CLAUDE.md rul-repeated-probing-reviewed-before-design`). Not catastrophic;
  flag only if a depth pass shows pairs × lookups is unbounded.

## findings

Reading order (kind first — self-consistency, then squaring with neighbours, then
revision-loss — and within kind by consequence; the bodies below sit in the order they were
written, which is the breadth-pass order):

1. fnd-link-count-is-not-closure-evidence — self-consistency; correctness (wrong DISJOINT,
   rare); `~SUSPECT`.
2. fnd-readset-closure-undefined-at-the-terminus — self-consistency; decides whether any
   sparing exists; `+SURE` ambiguous.
3. fnd-mworld-fence-contradicts-the-correspondence-example — self-consistency; two sentences
   opposite; `+SURE`.
4. fnd-finished-writeset-needs-both-records-unstated — self-consistency; silent channel if
   guessed; `+SURE`.
5. fnd-perish-set-of-an-unclosed-resolve-unstated — self-consistency; silent channel if
   guessed; `+SURE`.
6. fnd-absence-fact-has-no-stated-topic — self-consistency; gap; `+SURE`.
7. fnd-placing-lookup-disagreement-undefined-over-sets — self-consistency; aid
   mis-attribution only; `+SURE`.
8. fnd-in-memory-state-never-spares-past-a-file-write — squaring (USER_STORY, `KNOBS`);
   product, human decision; `+SURE` on the walk.
9. fnd-sparing-algebra-supersession-unregistered — squaring (`277` § 3, `30J`,
   `spike/CLAUDE.md`); register gap; `+SURE`.
10. fnd-revision-losses-footnote — revision-loss; footnote.

### fnd-in-memory-state-never-spares-past-a-file-write

Sections: 3.2-compare-one-chokepoint-four-answers (steps 1–4), 2.6-may-write-the-writeset,
1.9-cell-a-singleton-sort, 4.2. Kind: squaring (USER_STORY stage 5, `KNOBS:kSURVIVAL`,
`KNOBS:kHALVES`). Consequence: ergonomics/product, not correctness — every answer is safe.
Confidence: `+SURE` on the walk; `~SUSPECT` that the human has seen this particular
consequence stated.

Already on record, and only nodded at here: the posture itself. `311t` § 14 carries the
human-typed line "every sort walls everything around it until it is mapped into the
filesystem, spiritually, allowing for the few roots", the withdrawal of `311q` § 18's
parent-spoken partition, and "cross-sort sparing exists only where the walk answers
DISJOINT, which needs both sides in a shared key space". `311t:924` records that USER_STORY
stage 5 rests on the withdrawn partition, "`~SUSPECT` of how a stdlib chains a filesystem
and a service manager". That suspicion is what this finding resolves: it cannot.

The book, USER_STORY stage 5's own, stale-index morning, `--risk-faultless-skips` typed:

```sh
apt-get update                     # diverged; runs; writeset: the list files
...
systemctl enable --now nginx       # measured converged: enabled+active
```

The world: the list files are inodes in the root filesystem F. `enabled` is a symlink in F.
`active` is held in the service manager's memory, in this boot (1.9's own example).

The model's reading. apt's describer translates the bound into files, as `311t:1683`
instructs: writeset entries are inodes {l1, l2, …} in F, F in the mRoute. The site's facts
read two cells of the unit `nginx`: `nginx@sm.Enabled`, whose may-read is a symlink inode s
in F, and `nginx@sm.Active`, whose may-read is an activation in the manager, in the boot B,
B in the mRoute (or B a mRoot; either way below). 2.6 asks both questions for every
(writeset entry, read mKey) pair; the site is spared only if every pair passes (the universal
meet, `spike/CLAUDE.md set-lifting-universal-meet`).

- Pair (l1, s): chains meet at F; tops l1 and s are two inode mKeys of one mScheme with
  `:guarantees-unique-name`, differing; F `:aliases-nothing-else`; DISJOINT. Good.
- Pair (l1, activation): chains meet at the mRoute (or read UNKNOWN at step 1 if B is a
  mRoot the other side does not share). Tops: F (a filesystem mKey) and B (a boot mKey).
  Two tops: not one mScheme. One top: neither mKey is its own top. KNOWN_UNSPOKEN.
  Collides.

So line 9 guards on the stale-index morning, not elides; the render at USER_STORY:493–507 is
unreachable however completely apt's, systemd's, and the filesystem's describers speak. The
same walk kills every in-memory-versus-file pair in either direction: `systemctl start` (an
activation write) against any file fact; `sysctl -w` (a live kernel slot) against any file
fact; `nft add rule` against any file fact; `cp` against `@active`, live sysctls, live
rules, the mount table, running processes. Only file-versus-file, and same-mScheme keys at
different depths (the one-top way), ever spare under the model as written. That is a much
smaller product than `KNOBS:kSURVIVAL`'s "the whole stage-5–7 product" and than `kHALVES`
(welded far toward elide) describe; stage 5's headline sentence "the install's guard reads
the dpkg database, the `cp`'s fact lives in a config file's content, foobar's in its certs,
the service's in unit state — all disjoint, all survive" is three-quarters true and one
quarter false under 311.

Why the model cannot say otherwise from true statements: the two tops are children of one
container (the host) that no single definition distinguishes; 3.2 step 3 is deliberately
restricted to "a single definition's own distinctions" (`311t:1136`, human ack after
repeated attack). A statement "the boot's memory and the root filesystem are two things"
belongs to whoever describes the host's immediate children; the model has no seat for that
speaker, the partition being withdrawn.

Smallest repair, two honest options, either one a human decision: (a) accept, and let the
promotion carry one sentence stating the cost ("survival across a memory-held fact and a
file write, in either direction, does not exist; stage 5's line 9 guards") so the root docs
and `KNOBS` can be brought current rather than left silently stale; or (b) re-admit the
withdrawn partition in its narrowest form — a `:guarantees-unique-name`-class warrant on the
host's (or the boot's) *own* primary mScheme over its direct children, so that F and B become
"two tops of one mScheme", spoken once by the stdlib's host describer, which `311t:1043`'s
"speech is forced on whoever climbs out" already licenses in principle. Not (c): reading
KNOWN_UNSPOKEN as sparing, which `311t` and `311q` § 18 killed for cause.

Footnote (root docs are excluded from 4.2 by `311t:1161` "root docs excluded, known stale"):
the model's own 4.2 preamble does not say so; a reader of 311 alone cannot learn that
USER_STORY stage 5 and stage 7's "in any vocabulary, including ones I have never heard of"
are contradicted. One line in 4.2 would close that.

### fnd-link-count-is-not-closure-evidence

Sections: 1.5-token-and-the-two-warrants (the closure paragraph), 2.9 step 3. Kind:
self-consistency — the model supplies an evidence rule that does not support the statement it
licenses. Consequence: a wrong DISJOINT in the region test (cardinal class), at low frequency.
Confidence: `~SUSPECT` overall; `-GUESS` on the exact kernel path-walk behaviour, stated from
memory, unverifiable here.

The model says (1.5): the closure `alias nothing-else` states the referent "is reachable by
exactly this one entry anywhere in the instance the lookup ran in … For a file, the evidence
is a link count of one." `311t:1605` confirms this is the intended lookup body. 2.9 step 3
answers DISJOINT for D given whole against x when every level of x's mTraversal is DISJOINT
from D and emitted its closure, and defends against aliases above the leaf by asking every
level ("an alias may sit at any level … A leaf's own closure cannot see it").

An alias that sits at no level of the file's directory chain: a procfs magic link.

```sh
exec 3</srv/app/current/app.env             # holds the inode open
...
rm -rf /srv/app/current                     # writeset: Path:/srv/app/current given whole
...
cfg="$(cat /proc/self/fd/3)"                 # a read through the descriptor's name
```

The world: `app.env` is inode 77 in F, link count 1; `/proc/self/fd/3` resolves to inode 77
by jumping to the open file (a magic link; `-GUESS`: the walk lands on the file's dentry
without traversing `/srv/app/current`). The read's lookup records the traversal `/`, `proc`,
`self`, `fd`, `3` → 77 and, on the evidence rule, emits the closure for 77.

The model's reading of the pair (D=`/srv/app/current` given whole, x=the fact's mKey): step 1,
leaf 77 is not D; step 2, no level of x's traversal is SAME with D (dir 12 never appears);
step 3, every level DISJOINT from 12 with closures: DISJOINT. The fact survives `rm -rf` of
the directory that contains it. Whose statement is false? The closure on 77 — but the model
told its author that a link count of one is the evidence, and it is not: link count counts
directory entries, not open descriptions, and not the magic names procfs mints for them.

Where it bites in ops: rare. `/proc/$pid/fd/N`, `/dev/fd/N`, `/dev/stdin` as file arguments;
`/proc/$pid/cwd/…` and `/proc/$pid/root/…` are the commoner spellings and are probably safe
(the jump lands on a directory, which then appears as a level). Descriptor-held writes
(`>&3`) are the filesystem binder's business, not this. The model's failure is that it
states an evidence rule as if it were sufficient.

Smallest repair: strike the two evidence sentences from 1.5 (they are userspace, the path
describer's) or qualify them: the closure is withheld on any path containing a component the
lookup did not resolve by directory entry (magic links, indexicals), and a link count of one
is evidence only along a chain of ordinary entries. Either keeps the region test as written.

### fnd-placing-lookup-disagreement-undefined-over-sets

Sections: 2.10-places-the-upward-lookup, 1.5-token-and-the-two-warrants. Kind:
self-consistency (text). Consequence: aid-plane mis-attribution (a refusal naming an author
who said nothing false), never a wrong verdict. Confidence: `+SURE` on the text; `~SUSPECT`
on frequency.

```sh
mount --bind /mnt/a /srv/a
...
umount /srv/a                      # writeset: Mount:/srv/a given whole
...
cat /mnt/a/f                       # a fact through the other name
```

The world: one inode 77 in filesystem F, exposed by mount M1 (`/`) at `/mnt/a/f` and by the
bind mount M2 at `/srv/a/f`. Both paths are mKeys the engine holds for the referent (both
yield inode 77, SAME by `:guarantees-unique-referent`).

The model's reading: the writeset names a Mount mKey given whole; the fact's mKey has no Mount
route; Mount declares that it places Path; so the engine "invokes the lookup with every mKey
it holds for that mReferent". A per-key Mount lookup answers `looked-up-in Mount:M1` for
`/mnt/a/f` and `looked-up-in Mount:M2` for `/srv/a/f`. 2.10: "Two answers that disagree are
refused and attributed to G's owner." Both records are true; a referent under a bind mount is
placed in two mounts. The refusal is safe (walls), but its attribution names the mount
describer as contradicting themselves, which is the failure `311p` § 2 already struck once
from 3.5 ("refusal plus false attribution of two correct authors, the worst aid failure").

The model half-knows this: 1.5 says a lookup "where it knows other entries, emits them first,
and a listed alias is checked as the first entry is", i.e. answers are sets. 2.10 then speaks
of "two answers that disagree" as if answers were single. Smallest repair: define disagreement
over the union of records — positive `looked-up-in` records from several invocations
accumulate; only a `looked-up-in nothing-else` closure emitted on one invocation and
contradicted by another invocation's record is a contradiction, and that one is honestly the
owner's (their closure was false).

### fnd-absence-fact-has-no-stated-topic

Sections: 1.9-cell-a-singleton-sort, 1.6-parent-one-per-key, 3.3-perishing-three-mutator-
species, 1.11. Kind: self-consistency (a gap, not a contradiction). Consequence: none if
resolved the safe way; a builder choosing the other way mints a fact about nothing.
Confidence: `+SURE` the text is silent; `~SUSPECT` on the intended answer.

```sh
[ ! -e /etc/nginx/sites-enabled/default ] || rm /etc/nginx/sites-enabled/default
dpkg -s apache2 >/dev/null 2>&1 && apt-get remove -y apache2
```

Both guards assert absence. A `resolve()` of `/etc/nginx/sites-enabled/default` finds no
entry; there is no mResolution, no mReferent, no leaf mFullyQualifiedKey. 3.3 says creation,
deletion and rename are "routing writes to its mParent-Catalog entry, its existence cell", so
the model means the fact to be about the catalog entry. But 1.9 defines a cell as a
*singleton* mSort under an mParent, and the entries of directory 5 are many, keyed by name;
1.6 says the catalog edge "never carries identity". So an absence fact's mTopic is
`(entry named default, in directory 5)@sm.Exists`, an identity built on a routing-level mKey
the model says carries no identity. `311q` § 11–12 discussed existence as "the catalog
entry's facet" and then dropped the category; the final text kept the sentence in 3.3 and
lost the definition.

Why it matters: a later `apt-get install nginx-full` (writeset widened by `dpkg -L` to
`/etc/nginx/sites-enabled/default` as a *path*) must collide with the absence fact. That
collision needs the two to `compare()` not-DISJOINT, which needs the absence fact to have an
mKey the walk can place: the directory entry (dir 5, name `default`), which a directory does
map injectively (unique-referent and unique-name both hold for names in one directory).
Smallest repair: one paragraph in 1.9 or 1.6 stating that a catalog entry is an mReferent of
its own (a directory entry, a passwd line, a unit-table row), that its existence cell is
keyed by (mParent-Catalog instance, name), that both warrants hold for names within one
catalog instance by the catalog's construction, and that an absence fact's mTopic is that
cell. Without it, "silent channel" is the honest default: absence facts never transport and
never spare, which is safe but loses every hand-written `[ ! -e ]` guard's elision.

### fnd-sparing-algebra-supersession-unregistered

Sections: 4.1-boundary-of-this-model ("excludes a selector dialect"), 4.2, 1.9, 3.2. Kind:
squaring — an unregistered contradiction of a human-typed neighbour. Consequence: process;
the live steering law and the model disagree on a common book shape and nobody has been told.
Confidence: `+SURE`.

The neighbour: `277` § 3 "The selector dialect (the survival-license algebra)", human-acked
and typed (`277:180`), carried live as `spike/CLAUDE.md sparing-algebra` ("same-entity, a
claim SPARES a backing iff BOTH sides carry minted selectors AND claim-token ∈ dialect(…) AND
claim ≠ backing"), with dialect keying in `30J` § 12. Under it,

```sh
systemctl start nginx        # writeset: sm.Service:nginx@active
systemctl enable nginx       # measured: sm.Service:nginx@enabled
```

spares `enabled` past `start`: same entity, two minted selectors of one dialect, unequal.

The model: 4.1 excludes a selector dialect; 1.9 makes `active` and `enabled` two mSorts and
sends them to 3.2 "as between any two mSorts"; 1.2 forbids assuming two mSorts disjoint;
3.2's two ways need mKeys of one mScheme or a self-top distinction, and a cell has no mScheme.
Under the human's typed posture (`311t:1043`, `1057`) the pair collides unless both cells are
re-homed into one key space, and `active` (manager memory) and `enabled` (a symlink) never
share one. So the model reverses `277` § 3's answer for the commonest two-line service idiom
in books. That reversal is on record as a cost (`311p` thread 3; `311t:1023`–`1042`; "punted
to 312", `311t:1192`) and is not re-argued here.

What is not on record: 4.2 lists `30U`, `30W`, `30T`, `27C`, `271`, `272`, `26Ob`,
`ANALYZER-NEEDS` — and not `277` § 3, not `30J` § 12, not the `spike/CLAUDE.md`
sparing-algebra bullet. `311q` § 18 left `30J` "neither in nor out" (human), which is exactly
the state 4.2 exists to register. A builder reading the steering law and the model will
implement two different answers for the same pair. Smallest repair: one 4.2 entry naming
`277` § 3 / `30J` § 12 / the steering bullet, with the model's claim after "Here" (two cells
of one mParent separate only where their stores separate under 3.2; selector inequality
licenses nothing).

### fnd-mworld-fence-contradicts-the-correspondence-example

Sections: 1.8-fully-qualified-key-topic-and-derivation, 3.2 step 1, 2.7-corresponds-across-
a-transition, 3.2 "mDerivation sets". Kind: self-consistency (two sentences, opposite
readings). Consequence: a builder taking the fence literally refuses a SAME the model
elsewhere licenses (lost transport, safe); taking the example literally with no fence
statement admits cross-root SAME with no stated bound. Confidence: `+SURE` on the text.

1.8: "Nothing speaks across mWorlds." 3.2 step 1: "Nothing speaks across mWorlds, not even the
finished definition." 2.7: "A mount line's oracle knows mKeys under the mountpoint are mKeys
under the export on the named server, from this mVantage" — a mCorrespondence whose two ends
are a client path (terminating at this host's mRoute) and a server path (terminating at a
DNS mRoot plus a remote filesystem: another mWorld by 1.8's own definition). 3.2 then says
"SAME is 'or' across mDerivations (the mFullyQualifiedKey, a mCorrespondence, a
provider-supplied identifier)".

```sh
mount -t nfs files.corp:/export/www /srv/www
cp ./index.html /srv/www/index.html          # measured: content match, via the mount
```

Only the mount line's oracle can say the client key and the server key denote one inode;
that is 2.7's example and 3.5's committee law working as intended. But 1.8 and 3.2 step 1
say nothing speaks across mWorlds, full stop, and the server's inode lives in another one.
The two sentences cannot both be read as written. The pre-rewrite text (`21b93214:509`)
carried only the step-1 UNKNOWN for cross-route mFullyQualifiedKeys; the categorical "Nothing
speaks across mWorlds" arrived with the STE100 rewrite (`ca68b868`) and was not crosschecked.

Smallest repair: narrow the fence to what it is about — "no mFullyQualifiedKey derivation
and no finished record reaches across mWorlds; a mCorrespondence declared by the transition's
owner is the one derivation that may, vouch-tier, and it composes with DISJOINT only as
`311q` § 10 typed (a part, a view or a correlate does not)". `311t` § 6–§ 7 (Dana's zones,
"nothing on the web is aliases-nothing-else") is where the human's cross-root leans live;
the model's sentence should not out-run them.

### fnd-readset-closure-undefined-at-the-terminus

Sections: 2.5-may-read-the-readset ("closed only when every declared set is closed";
"Default: ⊤"), 2.6 step 2, 1.3 (the floor), 1.10 (the engine-vouched mRoute). Kind:
self-consistency — two sentences give opposite answers to the one question a builder must
ask first. Consequence: read one way, no sparing exists at all (every readset stays open);
read the other, an ancestor's silence is skipped and the "Default: ⊤" sentence is false.
Confidence: `+SURE` the text says both; `~SUSPECT` which was meant.

2.5: a fact's readset is the body's marked reads "together with the may-read entries declared
by every member of the mKey's mFullyQualifiedKey. It is closed only when every declared set
is closed." And: "Default: ⊤, which collides with everything." Every mFullyQualifiedKey ends
at the mRoute or a mRoot. The mRoute's mSort is the floor "mSort nobody has named" (1.3) or
the engine's own vouch (1.10); a mRoot shape's mSort is whatever a stdlib declared it as. None
of these has a may-read declaration in the model.

```sh
cp ./nginx.conf /etc/nginx/nginx.conf      # fact: content match; chain 77 → dir → F → mRoute
```

Reading A ("undeclared = ⊤"): the mRoute member's set is ⊤, the readset never closes, 2.6
step 2 never passes, no elision ever survives any write. Reading B ("only declared sets
count"): the chain's closure is decided by the leaf's and F's declarations alone, and an
ancestor whose describer said nothing is silently skipped — which is fine for the mRoute
(the engine can vouch its set closed-empty, but the model does not say it does) and
dangerous for a mid-chain store whose describer forgot (a loop-backed F whose image file was
never named: the very example 2.5 gives). Neither reading is stated, and the pre-rewrite text
(`21b93214:345`) carried the same sentence, so this was never crosschecked.

Smallest repair: state that (i) the leaf mSort's set defaults to ⊤; (ii) an intermediate
member's undeclared set is ⊤ too (the silent channel is the omission inside a declared set,
not the absence of the declaration); (iii) the mRoute's and each mRoot shape's may-read is
closed-empty by the engine's vouch (1.10) or the root describer's declaration respectively —
a positive engine statement that 1.5 should list beside the transit-free mRoute vouch. Under
that, (iii) is what makes any sparing possible, and it is one more thing the engine says.

### fnd-perish-set-of-an-unclosed-resolve-unstated

Sections: 3.3-perishing-three-mutator-species (routing: "when its `resolve()` read the written
state"), 4.1 (the read-set closure "as the falsification net for unmarked reads"). Kind:
self-consistency (gap). Consequence: a builder choosing the narrow reading opens a silent
channel (a resolution that should perish does not, and a SAME built on it keeps authority
past a write that changed what the name reaches). Confidence: `+SURE` the text is silent.

3.3 perishes a mResolution "when its `resolve()` read the written state". The engine knows a
body's reads only through `plans/27C` § 4(a)(B)'s read-set closure, which 4.1 adopts and
which is default-disqualify: an unaudited construct in the body fails the closure. The model
does not say what the perish set of a resolution through a body that *failed* the closure
is.

```sh
db="$(git config --global core.dbpath)"     # a resolve() that shells out
export GIT_CONFIG_GLOBAL=/srv/alt.gitconfig # routing write to shell state
...                                         # a later site keyed by "$db"
```

If the perish set is the marked reads only, the resolution survives the `export` and the
later site's SAME stands on a name that now reaches a different file. If the perish set is ⊤
(any routing write perishes it), the resolution is re-derived (lost transport, safe). `30S`
already prices this on the verdict-body side (pin-or-sever, withhold the probe); the model
should say the same for `resolve()` bodies: a lookup whose body is not read-set-closed has
perish set ⊤ under routing mutations, and its warrants hold within the span only until any
routing write. One sentence in 3.3.

### fnd-finished-writeset-needs-both-records-unstated

Sections: 2.6-may-write-the-writeset. Kind: self-consistency (one word, two records).
Consequence: a builder gating step 2 on one record where two are needed opens a silent
channel on the other side. Confidence: `+SURE` the text is ambiguous; `+SURE` of `30U`'s
intent.

2.6 says the writeset is "the may-write entries the verb's author declared per matched
shape … closed by the completion record, and widened by the may-write entailment that mSort
owners declare", and separately "The reached completion record finishes the definition" (the
mSort owner's). Step 2 then requires "The writeset's definition is finished" — singular.
`30U` § 5 is explicit that these are two records with two authors under one law: the verb
author's tail record is the mandatory completion witness of a *dynamic* `disturbs` body
(without it, exit-0 truncation under-claims and wrongly spares), and the kind owner's tail
record finishes the entailment. `30U` § 7's settle seat gates "each cross-kind footprint ×
backing pair on the footprint cell's origin kind" and "widened cells are part of the finished
statement, not separately gated".

```sh
apt-get install -y nginx       # dynamic disturbs: dpkg -L … | … ; tail record required
```

Under 311, for a pair (widened entry w of mSort K', read mKey r): is step 2 satisfied when
the verb author's record arrived but K's owner's `disturbance_reaches` for the origin cell
reached no record? Or when K's record arrived but the dynamic `disturbs` body died before
its tail? The model should say: both, per origin cell — the verb's at-most set closed by its
own record AND, for every mSort whose entailment widened the set, that mSort's finished
record reached for the origin cell's shape; and per `30U` § 7, widened cells inherit their
origin's finished-status rather than needing K'-side records. One sentence; `30U` has it.

### fnd-revision-losses-footnote

Kind: things a revision lost; git holds the text. Confidence: `+SURE` on what was dropped.

- The refuted-shapes register. `ac4bc430` ("five shapes this sitting killed") and `7724e30c`
  ("six shapes this arc refuted and what killed each") recorded, per shape, the killer case:
  the mParent as an implicit may-read entry (killed by the sidecar-files describer); skipping
  shared ancestors in the may-read test (same); a term for a referent's parts; a question
  species beside the mReferent; a mirror rule over may-read (killed by a resolver cache
  may-reading `/etc/hosts`); the upward lookup as a second mScheme of the placed mSort.
  `ca68b868` (the STE100 rewrite) dropped § 4.2-refuted-shapes whole. `311t:1159` records the
  human anticipating dropping it "for tightness", so this is by leave; but 4.2's surviving
  register lists only *other documents'* stale claims, and nothing in the tree now stops a
  successor from re-minting "the mParent is implicitly a may-read entry" — the shape that
  was killed twice (`311t:1194`–`1205`). A one-line pointer in 4.2 to the two commits would
  cost nothing.
- The four-gotcha litmus (`c7d48e1b`, `73e70d45`): an opening that named four GOTCHAS entries
  and, for each, "the answer owed and who must say what" — the § 0 law's own worked test.
  Dropped with § 4.1-slugged-gotchas in `ca68b868`. Its loss is why this review had to
  re-derive which gotchas the model handles; a successor reviewer will again.
- New sentences the rewrite introduced without a crosscheck: the categorical "Nothing speaks
  across mWorlds" (twice; see fnd-mworld-fence-contradicts-the-correspondence-example) and the
  arity/declared-by/default/consumer/danger tails now inlined per relation, replacing the
  pre-rewrite § 2.10 relation table — the latter is an improvement; the former is the one
  post-panel sentence that reads as law and was never attacked.

## considered-and-dead

- one-top-way-underspecified · 3.2 step 3 · Dead: the way is the same-mScheme
  different-depth case (`kernel.pid_max` in the boot against `net.ipv4.ip_forward` in a
  netns) and was human-acked in that form after repeated attack (`311t:1136`–`1154`). My
  worry that an unrelated other-shape `:identified-in` could yield an accidental DISJOINT
  dissolves: the real work is done by the deeper store's `:aliases-nothing-else` (if x were
  inside that store, the store would be giving its mKeys to its parent's thing); the "knows M"
  clause is only the guard against a floor-mScheme stranger separating from everything
  (`311q` § 15 `cliff-nothing-else-on-a-floor-sort`). Residual, text-level: the model states
  the rule without its instance or the ledger's clarifying sentence ("that body has some arm
  whose `:identified-in` names the store's mSort"); a reader cannot reconstruct it, as I
  could not until the ledger. One example line in 3.2 would fix it.
- container-subtraction-blinds-container-cells · 2.6 · Dead, safe direction. A write to
  inode 77 versus the cell `10@sm.Mtime` of its directory: the cell has no mScheme, so it is
  never a "top" that 3.2 step 3 can separate; the pair reads KNOWN_UNSPOKEN and collides. If the
  cell owner declares may-read = the directory given whole, 3.2 step 2 ("either mKey is A
  itself") collides too. The only way to a wrong DISJOINT is a closed may-read set that omits
  the directory, which is the fs describer's false statement about a thing they own. Text nit:
  2.6's "a write to an mKey is also a write to every container" is honoured only for the
  entailment, not for a container's own cells; one clause saying "a container's cells depend
  on their may-read sets, never on this sentence" would stop a builder reading it as coverage.
- environment-is-catalog-or-shell-state · 3.4, 3.3, 1.10 · Dead. `plans/30S` is the
  design-of-record: the environment is not a catalog; an exported delta is an index fence
  (`30S:rul-export-is-an-index-fence`), a prefix assignment is value-carried into the site's
  identity, shell-resolution variables are engine-owned, and a body below a delta is probeable
  only with pins or severance. In 311's terms all of that is "shell state a `resolve()` read"
  (routing), which is what 3.3 says. `sudo`'s scrub (GOTCHAS
  a-wrapper-scrubs-what-an-outer-wrapper-lent) is answered by measurement in the denoted
  context (`plans/27C`): the `resolve()` runs inside the wrapper and sees the scrubbed
  environment; nothing needs lending. The model could say in one clause that the environment
  is routing input, never an mParent-Catalog, and cite `30S`; not owed.
- indexical-knowledge-is-not-the-path-owners · 2.9 · Dead. That `/proc/sys/net` is per-netns
  is knowledge of whoever classifies filesystem types per shape "one level up" (2.2's own
  mechanism: the mount crossing yields a key in a procfs store whose describer says which
  subtrees are identified in which namespace), and in the stdlib that describer and the path
  describer are one seat. The second half ("an undeclared indexical reads unknown" is
  unenforceable) is just the ordinary knife: an undeclared indexical is a false yield by the
  lookup's owner, attributable. `311q` § 3's `sm.NetnsSelf:self` idiom shows the intended
  spelling. Nothing owed.
- shell-builtins-writeset-author-unnamed · 3.3, 3.5, 1.5 · Dead. `30S:rul-engine-owns-shell-
  resolution-vars` (human-typed) and `rul-export-is-an-index-fence` already make the engine
  the author of `cd`/`export`/assignment writes under the sh-parity law
  (`spike/CLAUDE.md rul-unsure-falls-toward-sh-parity`). The model's 1.5 lists only the
  transit-free mRoute as the engine's own vouch; adding "and the routing writes of shell
  builtins, by parity" is a one-clause completeness edit, not a fault.
- law-forbids-what-the-flag-buys · 0, 2.6, 3.2 · Dead. Under the model the flag still owns
  something no line can say: the at-most set's completeness (2.6 step 2), which is an
  open-world claim about a binary's future writes, exactly `30U` § 2's framing. What 311
  removes is only the cross-vocabulary sparing that rested on nobody's speech
  (`30U:inv-30U-no-sparing-on-nobodys-speech`), and the human typed the removal
  (`311t:1043` "separation resting on silence is a dealbreaker"). § 0's law and the flag are
  consistent. Footnote for the human, not the model: IMPLEMENTATION's definition of the
  flag's class ("nobody did something locally incorrect") and its name `--risk-faultless-
  skips` no longer describe any answer the model can give — every survival now names a
  fault-able author. The root docs are known stale on this by `311t:1161`; the name will
  eventually mislead an admin.
- lookup-chains-force-second-round-trip · 1.4, 2.10, 1.10 · Dead. A `:places` lookup's
  input, even a captured mValue, can be sequenced inside the one probe artifact the engine
  already ships (capture, then invoke, in authored sh scaffolding), so no dependent second
  exchange is forced by the model; the apply-standup `witness()` is on record (`311t:155`,
  `166`, `877`) and is integrity-only. Which pairs need a placing lookup is static (which
  writeset entries are given whole; which facts lack a route), so the count is bounded by
  sites × placing sorts, not by host state. Not catastrophic; the repeated-probing review
  (`spike/CLAUDE.md rul-repeated-probing-reviewed-before-design`) is the build's gate, not
  the model's.
## coverage

Read whole, before any synthesis: root `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`,
`USER_STORY.md`, `AGENTS.md`, `KNOBS.md`, `spike/CLAUDE.md`, `Research/GOTCHAS.md`; then the
model, every section, § 0 through § 4.2.

Model sections checked in breadth (a stub or a dead entry above traces to each): § 0 (the
law, against the flag); 1.1; 1.2 (strangers); 1.3 (the floor); 1.4; 1.5 (both warrants, the
closure); 1.6 (three seats); 1.7; 1.8 (mWorld); 1.9 (cells; absence); 1.10 (vouch, witness);
1.11; 2.1; 2.2 (rootness, one-level classification); 2.3; 2.4; 2.5 (closure, terminus); 2.6
(subtraction, two questions, records); 2.7; 2.8; 2.9 (region test, indexicals, magic links);
2.10 (disagreement); 2.11; 3.1; 3.2 (all four steps, both ways, derivation sets); 3.3 (all
three species); 3.4; 3.5; 4.1; 4.2 (every entry read against its neighbour where the
neighbour was read).

GOTCHAS walked against the model, by slug: a-path-is-not-a-referent ·
a-host-is-not-a-partition · same-name-different-referent-per-viewpoint ·
not-every-transit-changes-the-referent · address-inequality-is-not-referent-inequality ·
distinct-names-alias-within-a-kind · containment-by-path-prefix-lies ·
renaming-a-parent-moves-every-child-name · identity-tokens-have-clone-horizons ·
a-name-is-not-a-target-over-time · a-store-is-not-one-inode ·
an-omitted-store-breaks-invariance · composite-identity-is-structure-not-a-bag ·
the-subject-includes-the-observer · identity-tokens-perish-on-write-not-only-on-rename ·
a-recreated-name-is-a-new-referent · recycled-keys-outrun-the-unwalled-span ·
apt-get-update-can-install-packages · net-sysctls-are-per-namespace ·
one-state-reached-through-two-kinds · an-env-variable-selects-the-referent ·
sudo-picks-the-context-by-the-command · a-wrapper-scrubs-what-an-outer-wrapper-lent ·
a-login-shell-sources-profiles · a-cached-lookup-answers-for-the-past ·
an-open-descriptor-outlives-its-name · a-whole-write-replaces-every-part ·
a-read-folds-absent-into-a-value · backing-is-not-presenting ·
a-directory-can-have-two-parents · a-socket-on-a-shared-mount-is-not-shared ·
a-jump-host-moves-the-vantage. Each is handled by the model or by a named neighbour except
where a finding above says otherwise (magic links are not a GOTCHAS entry; they should be).

Neighbours read at depth: `plans/30U` whole; `plans/30S` whole; `notes/311p` whole;
`notes/311q` whole; `notes/311t` § 14–§ 15 and the § 6/§ 9/§ 15-adjacent passages the greps
led to; `notes/277` headings only; `Research/README.md`. Not read: `plans/30W`, `30T`, `27C`,
`notes/272` beyond what 4.2 and `spike/CLAUDE.md` quote — every 4.2 entry against them
re-reports a registered supersession, which the brief prices at nothing, and no stub needed
their text. History: `git log --follow` over all 53 revisions; `git show` of `0b66e890`,
`7724e30c`, `ca68b868`, `21b93214` (targeted greps), `-S mWorld`.

Deliberately not read: anything under `quarantine-DO-NOT-READ` or `corpora`, any `29*`
file, and this round's sibling passes `312cb`, `312cc`, `312cd` (independence). No scout was
spawned; nothing was executed beyond read-only git.

Not covered: `311t` § 3's killer set and the cross-root corner (DNS, cloud roots) — the
model's mRoot/mWorld sentences were checked for self-consistency only, not against that
ledger's leans; a reviewer with that ledger in context should own it. Performance was
checked only for the network-catastrophic case (none found).
## overall

The model is, by its own § 0 law, sound where I could walk it: I found no pair where it
returns SAME or DISJOINT from true statements alone, once the record's human-typed narrowing
of 3.2 (separation only from one definition's own distinctions) is read as intended. The
one correctness-class finding (fnd-link-count-is-not-closure-evidence) is the model
supplying an insufficient evidence rule for a closure it then consumes; it is rare in books
and repairs in two sentences. The rest of the self-consistency findings are gaps a builder
would have to resolve by guessing — the readset closure at the terminus (which decides
whether any sparing exists at all), the absence fact's topic, the unclosed `resolve()`
body's perish set, "finished" meaning one record or two, disagreement over set-valued
placing answers, and a fence sentence that contradicts the model's own NFS example. Each is
one or two sentences to fix and each fails toward silence or toward a false refusal if
guessed wrong, never toward a wrong elision.

The consequence the promotion should not carry silently is
fnd-in-memory-state-never-spares-past-a-file-write: the record accepts "every sort walls
everything around it until it is mapped into the filesystem" and leaves `~SUSPECT` how a
stdlib chains a filesystem and a service manager; it cannot, and so USER_STORY stage 5's own
line 9 guards on the stale-index morning, and every memory-held fact walls every file write
in both directions. That is a product statement about `kSURVIVAL` and `kHALVES`, human-owned,
and the choice between accepting it in writing and re-admitting a narrow host-level
partition is the one decision this review surfaces that is not a text fix. Beside it,
`277` § 3's sparing-algebra — still live steering law — is contradicted by the model and not
registered; that is the one register entry 4.2 owes before promotion.

Not attacked here: any `[LEAN]` line; the cross-root corner; the wider design's welds.
Everything else I suspected was either on record already (nodded, footnoted) or died under
the walk, and the dead entries say why.