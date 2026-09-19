# 311l — Identity crosscheck over `notes/311j`: neutral pass

Claude Fable 5.1 (Anthropic lineage), neutral pass, 2026-09-16.

> Notes-tier review, ahistorical; nothing here is ruled. Grades on my claims: +SURE / ~SUSPECT /
> -GUESS / --WONDER. Every finding is walked through `311j`'s own text on a plain-sh book of
> the kind people paste; section cites are `311j` unless prefixed. Where a suspected fault does
> not hold, § 5 says so. Rearrangement of existing machinery is named before anything new. Out
> of scope by brief: the two named horizons (noted only where the world hits them constantly),
> syntax, performance, omissions relative to the whole project.

## § 0-ranking

Class 1 (cannot represent): `fnd-sibling-cells-of-one-bearer-always-collide`;
`fnd-cross-transit-pairs-rest-on-horizoned-roots` (the fair "horizoned and constant" kind).
Class 2 (lost or inconsistent since 311): `fnd-disjoint-clause-over-demands-and-under-delivers`
(the strongest single item); `fnd-supply-modes-survive-their-own-retraction`;
`fnd-two-roots-forecloses-warranted-separation`; `fnd-indexical-clause-versus-singleton-spelling`;
`fnd-cross-sort-same-narrowed-by-design` (a price, not a fault);
`fnd-coherence-of-instance-derivations-unstated`. Class 3 (wrong speaker / over-knowing):
`fnd-sole-route-sited-on-the-child`; `fnd-warrants-claim-all-time-for-a-span`;
`fnd-shape-is-not-a-function-of-bytes`; `fnd-sentinel-rests-on-a-privileged-read`;
`fnd-observer-sorts-must-be-enumerated`. Class 4: three small text ambiguities. § 5: seven
suspected faults that do not hold.

## § 1-cannot-represent

### fnd-sibling-cells-of-one-bearer-always-collide

~SUSPECT on the reading of § 2.4 (two readings, argued below); +SURE on the consequence under
the reading the text supports.

```sh
#!/bin/sh
set -eu
KEY='ssh-ed25519 AAAA… ops@bastion'
chmod 0600 /root/.ssh/authorized_keys                                    # 4
grep -qF "$KEY" /root/.ssh/authorized_keys \
   || printf '%s\n' "$KEY" >>/root/.ssh/authorized_keys                  # 5
```

A day where a `cp` left the mode at 0644 and the key is present: line 4 really runs; line 5's
licence is the measured `contents` mCell of that inode. Walk. The chmod oracle's footprint
names the `mode` mCell (the part-level footprint `312b` § 8 held). § 1.9: `mode` and
`contents` are two mSorts, each `:identified-in` the bearer `I@F` with a singleton mKey. § 3.2
fourth bullet: different mSorts share no primary mScheme; the pair meets at `I@F`; at the leaf
it is KNOWN_UNSPOKEN; sparing rides the footprint side's finished definition, bounded by mPlacements
(§ 2.5). § 2.4: "exactly one mPlacement of an mSort sits on its identifying mFullyQualifiedKey
— the mParent-Store", and mPlacements are "mKeys of other mSorts", so a mCell's identifying
mPlacement is its bearer `I@F`, and both cells carry it. § 2.5's bound `compare()`s the two
sets; `I@F` against itself is SAME by one instance (§ 3.2 third bullet); an overlap; collide.
Line 5 guards on every day line 4 runs. The same walk fires for `systemctl enable nginx` then
`systemctl is-active nginx || systemctl start nginx` (`enabled@U` against `active@U`), for
`chown`/`touch` before any content-licensed line, for `usermod -s` before `usermod -aG`.

Nobody has a sentence for "mode and contents are separate state" — not Tessa, who owns both
cells. The § 3.2 route is closed because § 1.9 makes each mCell its own mSort (the legs' tops
are of different mSchemes, UNKNOWN); the § 2.5 route is closed because the identifying
mPlacement is the bearer. `312b` § 9's provisionally acked intent, "cells under one parent
partition by their placements, never by an owner's say-so", is not what the text delivers: the
placements that would partition them are dominated by the one placement they share.

The other reading: "sits on its identifying mFullyQualifiedKey" as "the placement *is* the
mCell's own mFullyQualifiedKey". Under it sibling cells' identifying placements differ (two
sorts, known-unspoken, no overlap) and the finding vanishes — but the reading contradicts "mKeys of
other mSorts" in the same paragraph, and it makes the identifying placement of a file its own
identity rather than its filesystem, which is not what `dd` over a loop image (§ 2.4's example)
needs. Whichever is meant, the text should say it.

Rearrangement, smallest first. (a) The identifying mPlacement answers identity, not
interference: exclude it from § 2.5's overlap test, one clause. Then Tessa's per-cell finished
definitions (`mode` reaches `ctime`, nothing else; `contents` reaches `size`, `mtime`, `ctime`,
nothing else) partition the siblings, attributed to her, and whole-bearer footprints still
collide with every cell through the bearer's entailment (§ 1.9). (b) Alternatively make
`:lives-in` and `:observer-independence` per matched shape, as `:identified-in`, the warrants,
and `:reaches` already are (§ 2.2, § 2.10), and let a bearer's cells be shapes of one keyed
cell-mScheme with `:guarantees-unique-name`; siblings then separate at § 3.2 by inequality.
(b) also retires § 4.2's aspect-species refutation, whose argument ("per-aspect placement and
dependence make an aspect an mSort") does not survive per-shape declaration being the model's
standard variation mechanism. Nothing new is needed under either.

### fnd-cross-transit-pairs-rest-on-horizoned-roots

-GUESS on frequency: README and DESIGN have Dorc ship one book per host, so in-book transits
are the fan-out and jump-host minority. Recorded as the brief's fair "horizoned and the world
does it constantly" finding, not as a fault.

```sh
ssh db1 'dpkg -s postgresql-16 >/dev/null || apt-get install -y postgresql-16'      # 3
ssh db1 'grep -q "^max_connections = 500" /etc/postgresql/16/main/postgresql.conf \
   || …'                                                                           # 4
ssh db2 'dpkg -s postgresql-16 >/dev/null || apt-get install -y postgresql-16'      # 5
```

§ 1.10: across a transit the mRoute is never vouched; every mKey in lines 3–5 terminates in
its own mRoute. § 3.2 first bullet: two mRoutes across a transit read UNKNOWN. Line 3 runs on
a drifted day; line 4 (same machine, different mSorts — would be KNOWN_UNSPOKEN plus a finished
definition within one mWorld) and line 5 (a different machine) both collide, both guard. The
only door out is a stdlib mRoot (`sm.BootId` with `:rootness`), and every honest such token is
one of the two horizons (`identity-tokens-have-clone-horizons`,
`a-host-key-identifies-an-endpoint-not-a-machine`). With that root, 3-vs-4 meets at the boot
and works; 3-vs-5 hits `fnd-two-roots-forecloses-warranted-separation` below.
`a-jump-host-moves-the-vantage` makes the same book worse, not different.

## § 2-lost-or-inconsistent-since-311

### fnd-disjoint-clause-over-demands-and-under-delivers

+SURE on all four parts; they are readings of § 3.2's second bullet as written.

(a) The tops over-demand `:sole-route`. The clause requires it of "every mKey of both legs",
tops included. When two legs' tops are siblings under the shared level A, of one mScheme with
`:guarantees-unique-name`, differing values, they name two mReferents by unique-name alone: if
both reached one mReferent it would have two mKeys in A. `:sole-route` is load-bearing only
strictly below a top (it is what stops guest pid 1 and host pid 4821 from reading DISJOINT,
§ 4.2). The over-demand bites where the leaf shape honestly cannot carry `:sole-route`:

```sh
kill -HUP "$(cat /run/nginx.pid)"                                          # 3
kill -0 "$(cat /run/haproxy.pid)" 2>/dev/null || systemctl start haproxy   # 4
```

Both pids are in the inherited pid namespace (one instance). Legs `[P1]`, `[P2]`; tops of one
mScheme; the pid owner can declare `:guarantees-unique-name` (one process, one pid, in one
namespace) but § 4.2 forbids `:sole-route` on pids. Text: UNKNOWN; line 4 guards whenever
line 3 runs. Truth: two processes. The same defect closes every Docker-exec'd book: two files
in a container's overlay root are siblings under one overlay device, unique-name on inodes
holds, but the inode shape cannot carry `:sole-route` there (the file is also reachable through
the upper directory on the host), so `printf … >/etc/app/a.conf` never separates from a
`test -s /etc/app/b.conf` licence. Fix: demand `:sole-route` of every leg mKey strictly below
its top; keep unique-name and inequality on the tops. No new declaration.

(b) The "either one leg is empty" disjunct is unreachable. A leg is the chain strictly below
A; the sentence before ("if either mKey is A itself, the pair reads UNKNOWN") already removed
every case with an empty leg. Dead text, presumably a residue of an earlier cut.

(c) The exercise's lines 7 and 8 are not licensed by the text. In
`312b-exercises/net-sysctls-are-per-namespace`, line 5 against line 8: A is the boot; legs
`[nsB, net/ipv4/ip_forward]` and `[kernel/pid_max]`; the tops are an `sm.NetnsInode` and an
`sm.ProcSysPath`, different mSchemes; the only remaining disjunct is (b)'s dead one; UNKNOWN.
`311q` § 4 records the edit as recovering "survival across a differently-lengthed leg", and the
exercise's `obs-a-leg-separates-by-sole-route` and its drifted-day render (lines 7 and 8
"survive line 5") rest on it. Line 6 against 5 *is* licensed (tops `H` and `B`, both
`sm.NetnsInode`, unique-name, differing; `:sole-route` below). What the differently-lengthed
case actually needs is an axiom the model refuses: `:identified-in` irreflexive (a store is not
a member of itself), so that "reachable only through nsB" and "reachable only through the boot
directly" cannot name one mReferent. § 3.2's "an mKey against its own container reads UNKNOWN"
is precisely that refusal, and it is what keeps `contents@I` from reading DISJOINT against
`File:I`. Adopting irreflexivity for identity moves every whole-versus-part interference onto
the bearer's entailment in both directions (a part-write reaches the whole, declared by the
bearer owner), which today is free. The text is the conservative one. Either edit § 3.2 to
state the axiom and its cost, or fix the exercise's claim and render; --WONDER which the human
wants, and I lean to the exercise being wrong as written and the axiom being worth pricing
separately.

(d) Direction, for the record: `311` § 3.2 asked `:sole-route` of "every level above" the
divergence (root-ward), which protects nothing — the shared ancestor's other routes are
irrelevant to two keys that share it. `311j` moved it leaf-ward, correctly, and overshot at
the tops. The 311→311j change here is an improvement with one residual defect.

### fnd-supply-modes-survive-their-own-retraction

+SURE. § 2.1 still declares the mParent-Catalog "with a supply mode: FIXED … SITE … AMBIENT …
or COMPUTED" (311 had three; 311j *added* one), and the § 2.10 `:yields` row says "its supply
mode". `311q` § 3 lists "supply modes are not a concept" among the four findings acked in chat,
and the exercise's `~` section applies the three-seats reading to the catalog as well as the
store ("the three-seats finding applied to the catalog rather than the store"). `311q` § 4's
edit list touches § 2.1 only for the store seat. Either the catalog deliberately keeps modes
(then say why the store does not) or the enum goes and the catalog instance is "supplied by a
seat" like the store. Ahistorical edit owed either way.

### fnd-two-roots-forecloses-warranted-separation

~SUSPECT it is a loss; +SURE it is ambiguous. § 3.2 first bullet: "two mRoots" read UNKNOWN,
and "a second mRoot is a second mWorld". § 2.2: `:rootness` is "equivalent to
:guarantees-unique-referent over the whole world", and a root shape may also carry
`:guarantees-unique-name`. If "two mRoots" means two *keys* of one root shape, then boot X
against boot Y never separates, a root shape's unique-name is dead weight, and `:rootness`
buys SAME only — a narrowing against `311` § 1.4's "globally comparable" and `311` § 3.2's
known-unspoken-plus-finished (which at least spared). § 4.2's kill for the old rule (a partial key
compared more decisively than the complete one) is answered by UNKNOWN for mRoute-versus-mRoot
and for two root *shapes*; two measured, warranted keys of one root shape is not the widening
case. If "two mRoots" means two root shapes, say so; if keys, the cross-machine half of
`fnd-cross-transit-pairs-rest-on-horizoned-roots` is closed by text, not by horizon.

### fnd-indexical-clause-versus-singleton-spelling

~SUSPECT, low. § 2.8 keeps "only the mScheme owner can say which [components are indexical],
and an undeclared indexical component reads unknown"; `311q` § 3 acks the singleton-spelling
convention (`sm.NetnsSelf:self`, `sm.HomePath:~`) as needing "no engine or model work", and the
exercise says § 2.8's sentence "is satisfied without a declaration". Both can stand — the
convention covers components a yield names; the clause covers book-literal components
(`/proc/self/…`, `/dev/fd/3`) that reach Tessa directly — but a reader of § 2.8 alone takes the
clause as the mechanism. One sentence in § 2.8 pointing at the yield-and-catalog composition
closes it. (`311` § 7's `open-local-route-claim-residue` was this residue; dropped by burndown,
still real.)

### fnd-cross-sort-same-narrowed-by-design

+SURE it changed; recorded as a price, not a fault. `311` § 3.2: "a shared mKey-PrimaryStore
with equal mKey-Primaries IS the cross-mSort SAME generator". `311j` § 3.2: "two mSchemes of
ONE mSort yielding one mKey-Primary is the sole same-referent generator … Dorc equates mKeys
and never merges mSorts". The cost is on the stranger: their glue line moves their mScheme
*into* the host mSort (§ 1.3, one mSort per mScheme; § 2.1, both of one mSort), so they cannot
keep a sort-level declaration of their own for those keys. `one-state-reached-through-two-kinds`
is UNKNOWN by design until a human merge (§ 1.2). `312b` § 7 argued this is the right seat; the
text should carry the price in one line so it is not re-litigated as a hole.

### fnd-coherence-of-instance-derivations-unstated

-GUESS, low. § 1.8: "mDerivations combine by coherence, never by priority"; § 3.2 gives the
"or across, and within" algebra and the contradiction rule for SAME against DISJOINT. What two
derivations of one *instance* do when they disagree under no `:guarantees-unique-name` is
unstated: the exercise's Nathan-lend and Simon-`self` both yield `B`; had they yielded `B` and
`B'` with unique-name absent, § 2.2's "cannot contradict anything" leaves both standing and the
ambient mParent is undetermined. `311` § 7's `open-derivation-algebra-formalization` named the
gap; burndown dropped the item, not the gap.

## § 3-wrong-speaker-or-over-knowing

### fnd-sole-route-sited-on-the-child

+SURE on the textual contradiction; ~SUSPECT on how often it bites. § 2.2 has the primary
mScheme's owner declare `:sole-route` "per matched shape of the mKey's value" and offers "an
inode in its ext4 table" as constitution; five sentences later, "the child mSort's owner never
learns the mParent's types". An inode number's bytes do not say ext4, nfs, overlay or sshfs.
Tessa can declare `:sole-route` on inodes only blanket (true for stores; false for views — an
overlay file is also reachable through its upper directory, an NFS or sshfs inode through the
server) or decline (losing every cross-filesystem DISJOINT). Blanket is safe only
conditionally: each view type's owner must decline `:sole-route` at *its* level so the walk
fails there first, and when one does not, § 3.5's attribution names Tessa's line for the fs
owner's error.

```sh
printf 'x' >/srv/data/a.conf                    # 3   /srv is sdb1
test -s /etc/app/b.conf || cp b.conf /etc/app/  # 4   / is sda1
```

A is the boot; legs `[F1, Ia]`, `[F2, Ib]`; tops differ under the fs owner's unique-name and
`:sole-route` by contract; below them `Ia` and `Ib` need `:sole-route`, Tessa's, which she can
only make blanket. `312b` § 7 `fnd-two-epistemic-seats-mis-sited` proposed the parent's
per-shape classification carry the children's `:sole-route`, the parent declaring once, children
inheriting; `311j` kept the child seat. Rearrangement: § 2.2 is already per shape one level up;
let the parent's shape declaration carry the children's `:sole-route`, and let the child owner
declare none. With `fnd-disjoint-clause-over-demands-and-under-delivers` (a), siblings never
need it at all.

### fnd-warrants-claim-all-time-for-a-span

+SURE on the wording; ~SUSPECT on the rearrangement being enough. § 1.5: each warrant is
"declared once … holding for every mKey that takes it inside any one mParent", with no temporal
bound, and recycled mKeys are listed as a failure. Honest owners of reissued keys therefore
decline (the exercise's Simon on nsfs inodes; Tessa declares nothing on inodes) or over-claim
and own a horizon (the exercise's Simon on `boot_id`, by the human's lean) — two stances in one
strawman. What an owner can honestly say is span-scoped: equal mKeys in one mParent reach one
mReferent unless a perishing write intervened (§ 3.3: a routing write to the entry, a state
write to the store), which § 3.3 already computes; the residue — reissue by an actor outside
the book — is the named horizon. Books that lose under decline:

```sh
mount --bind /srv/app/etc /etc/app                    # earlier, or standing
printf 'x' >/srv/app/etc/a.conf                       # 5
grep -q x /etc/app/a.conf || …                        # 6   one inode, two spellings
```

and `ip netns exec blue sysctl -w …` beside `nsenter --net=/var/run/netns/blue sysctl -w …`
(two wrappers, one nsfs inode). Equal values, no warrant, UNKNOWN: safe, and the fact never
stands in. Rearrangement: read both warrants as holding within an unwalled span absent a
perishing write, say so in § 1.5, and the two stances become one honest declaration.

### fnd-shape-is-not-a-function-of-bytes

+SURE on the contradiction; low practical exposure. § 1.6: the matched shape is "a function of
the mKey's own bytes"; § 2.2: the primary mScheme's `resolve()` "is the identity on the mKey".
A device number's parent (ext4 in the boot; NFS in a server; overlay, a view) is not in its
bytes; the classifying arm must read `mountinfo` or `statfs`. `312b` § 8 accepts world-reading
arms; the exercise says a `resolve()` body's reads are the owner's to mark and unmarked reads
back the resolution as ⊤. `311j` says neither, and since a primary's `resolve()` has no
mTraversal (§ 1.7 backs lookups), a classification read cannot perish. Exposure is small
because a path re-resolves through its own traversal, which does perish; but the text
contradicts its own practice. Either say a primary's arm may read the world and those reads
back its classification, or keep shapes bytes-only and route fs types through a secondary
mScheme that reads `mountinfo` and yields a typed filesystem key.

### fnd-sentinel-rests-on-a-privileged-read

~SUSPECT. Right seat, undisclosed cost. § 3.4: a lend may depend on the guest; "the wrapper
author declares the guest-insensitive default and supplies a policy read that declines on
departure". The exercise's `obs-the-sentinel-is-the-keystone` shows every SAME and DISJOINT
passing through inherited instances that only `lends nothing-else` makes inherited. For `sudo`
(`sudo-is-a-variable`: every mutating line), the honest sentinel is conditional on sudoers
(GOTCHAS `sudo-picks-the-context-by-the-command`, `a-wrapper-scrubs-what-an-outer-wrapper-lent`,
`a-login-shell-sources-profiles`), and the policy read (`sudo -l`) is itself privileged and may
be refused at probe. When it declines, § 3.4 leaves every unlent mParent-Catalog ⊤ for that
line, and no pair of `$SUDO` lines — nor a `$SUDO` line against a bare one — is ever SAME or
DISJOINT. The model should say that a declined policy read walls identity for the whole book,
not just the line; today only the exercise's observation records that the keystone exists.

### fnd-observer-sorts-must-be-enumerated

-GUESS, low; the § 4.2 "cannot know the axes" kill at reduced severity. § 2.7: independence is
declared by K's owner per (mSort, O), default dependent. Tessa must name every O-sort (user,
group, SELinux role, capability set, …) before a `sudo grep -q x /etc/foo` fact stands for a
bare `grep -q x /etc/foo` one; an O-sort minted later by a wrapper author is unknown to her, so
facts never carry across that wrapper until she learns of it. Silence is no-carry, not a wall,
so this is safe; recorded because the seat is the same one the model rejected for axes.

## § 4-lower-value

- `amb-perish-touch-of-a-store` (~SUSPECT). § 3.3: "a state mutation whose footprint touches a
  mParent-Store perishes the mTokens scoped in it". If a footprint on `contents@I@F` touches
  `F` (an mKey against its own container, UNKNOWN, the safe bottom), every write on a
  filesystem perishes every inode token in it; if only SAME touches, overlay copy-up
  (`identity-tokens-perish-on-write-not-only-on-rename`) is not perished at plan time and rides
  on the witness alone. The honest answer is per store type — the overlay shape's owner
  declares that a state write reaches the store — which is per-shape `:reaches` on the store
  mSort, existing machinery; the text should say which reading holds meanwhile.
- `amb-route-claim-broader-than-its-warrant` (~SUSPECT). § 1.10's engine-vouched claim is
  justified path-specifically ("one shell process resolves one cwd-qualified mKey in one mount
  table") but stated for every mScheme, including floor mSchemes whose lookup is the tool's and
  invisible (`resolution-is-set-valued`, `an-env-variable-selects-the-referent`,
  `a-binary-reads-environment-you-cannot-see`); § 4.2's own kill list for the definitional
  default (round-robin, cached lookups, `:latest`) passes at the floor. `312b` § 8 says ρ
  writes and entry forms are covered by existing rules; name them in § 1.10 (a ρ write between
  two spellings re-keys the mPlaceholder; an unmodeled entry form is ⊤) so the claim's scope is
  visible and the floor's generosity is seen to be about the span, not the lookup.
- `amb-key-against-own-container-is-asymmetric` (-GUESS). The UNKNOWN for a mKey against its
  container protects the write-to-container direction (a container write must collide with
  every fact inside) and over-protects the write-inside direction (a knob write collides with
  the namespace's existence fact). § 1.9 already routes whole-to-part through entailment; the
  asymmetry is where `fnd-disjoint-clause-over-demands-and-under-delivers` (c)'s axiom would
  be priced.

## § 5-suspected-faults-that-do-not-hold

- Observer-dependence default direction: a write under a lent user against a fact under
  another observer collides on the mCell (SAME at the chokepoint); the O-instance qualifies
  only the SAME-for-carry consumer (§ 2.7). Safe in both directions. Holds.
- `cd`, relative paths, `~`: the catalog-beside-key ABI and the sh model (exercise, `~`
  section); nothing for identity to add. Holds.
- `useradd alice; chown alice /x`: a routing write to the passwd entry (§ 3.3) and a fresh
  mPlaceholder for the later lookup. Holds.
- Guest pid 1 against host pid 4821: absent `:sole-route` on pids, UNKNOWN (§ 4.2). Holds —
  and is why (a) above must keep `:sole-route` on every leg mKey below a top.
- Hardlinks, bind mounts, symlinked config dirs read SAME: representable by the file owner's
  `:guarantees-unique-referent` on inodes; the difficulty is only that § 1.5's wording makes
  the honest form unsayable (`fnd-warrants-claim-all-time-for-a-span`), not a modeling gap.
- Round-robin names at the floor: the cell is the set-valued name as a whole; which member
  answered is the verdict lane's question, not identity's. Holds.
- Two seats supplying different mParent instances (a bind's trailer against a yield's record):
  refused and attributed to both (§ 1.6). Holds, and is the right fail-fast.

## § 6-edits-in-value-order

`fnd-disjoint-clause-over-demands-and-under-delivers` (a) and (b) are text-only and safe;
(c) needs a human call between the axiom and the exercise. Then
`fnd-sibling-cells-of-one-bearer-always-collide` (a), one clause in § 2.5 plus one sentence in
§ 2.4 fixing the reading. Then `fnd-supply-modes-survive-their-own-retraction`,
`fnd-two-roots-forecloses-warranted-separation`, and `fnd-warrants-claim-all-time-for-a-span`,
each a wording decision. `fnd-sole-route-sited-on-the-child` is the one seat move.
