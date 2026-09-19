# 311p — Identity crosscheck over `notes/311j`: adjudication and synthesis

> AI-authored (Fable, 2026-09-17), notes-tier. Adjudicates the four crosscheck reports over
> `notes/311j` and its 311→311j diff: `311l` (Fable, neutral), `311m` (Fable, adversarial),
> `311n` (Astra, neutral), `311o` (Astra, adversarial). The dispatch bundle `311k` is quarantined
> and was not read here, by design. Every credited finding was re-walked through 311j's own text
> before crediting; convergence across lineages and stances weighed above eloquence; a finding
> raised by one pass alone was kept only where the walk checked out under my hand. Grades on my
> claims: +SURE / ~SUSPECT / -GUESS / --WONDER. Nothing here is ruled. One edit was made to 311j
> (§ 2) under the human's standing rule for changes needing no attention; everything else waits.

## § 0-verdict

Four passes, two lineages, converge on one thing: the DISJOINT bullet of §3.2 as committed on
2026-09-16 does not say what it was argued to say. Its "one leg is empty" disjunct is
unreachable, so the exercise's drifted-day survival of lines 7 and 8 does not follow from the
text, and it demands `:aliases-nothing-else` of the two legs' tops where `:guarantees-unique-name` alone
separates them, so two processes in one namespace can never be told apart. That is my error and
the first thing to repair. Three further threads have two-lineage support and re-walk cleanly:
the mCell fold of §1.9 dropped the per-cell store, so `enabled` and `active` cannot differ across
a reboot as the section claims and any two properties of one bearer always collide under §2.4
and §2.5; `:aliases-nothing-else` is declared by the child where the parent store's owner holds the
knowledge; and §1.10's engine vouch is stated for every floor spelling while justified only for
what the shell itself resolves. One false rule (§3.5's canary) was deleted. The rest is
text-level, or did not survive re-walking. Nothing here reaches 312.

## § 1-threads-that-survive, ranked

### 1. `thr-disjoint-bullet-as-written-is-defective` — 311l (b),(c) · 311m §1.2 · 311n 3 · 311o 4; +SURE

The text (§3.2, second bullet): a leg is "each chain's mKeys strictly below A", which includes
the leaf, so a leg is empty only when the mKey is A itself, and the preceding sentence already
sends that case to UNKNOWN. The disjunct is dead. Walk the exercise's line 5 against line 8: A
is the boot; legs `[B, net/ipv4/ip_forward]` and `[kernel/pid_max]`; tops an `sm.NetnsInode`
and an `sm.ProcSysPath`, different mSchemes; UNKNOWN. So the exercise's
`obs-a-leg-separates-by-aliases-nothing-else`, its drifted-day render, and `311q` §4's "recovers survival"
do not follow from the committed rule. Cause: the argument in chat used "nothing below A on G's
chain" to mean G's strict ancestors; the text wrote "leg" to include the leaf.

What the argument actually needs, stated: F's shape carries `:aliases-nothing-else`, so every route to
F's referent enters F's mParent; G's mParent is A, so G's route enters no store strictly below A;
F's mParent is strictly below A and is not A, because a store is never among its own contents;
hence F and G are two referents. That premise, the well-foundedness of `:identified-in`, is
what the walk has always assumed and the text never states; 311l (c) is right that the clause "a
key against its own container reads UNKNOWN" reads as a refusal of it. The two are compatible:
the UNKNOWN label is a consumer answer (a write to a container must collide with everything
inside it, and that is interference, not identity), while the structural premise is what the
walk uses. The text should say both.

Second defect, three passes (311l (a), 311m §1.2, 311n 3): the tops are over-demanded. Two
children of one shared A, of one mScheme carrying `:guarantees-unique-name`, with differing
values, are two referents by that warrant alone; `:aliases-nothing-else` does work only strictly below a
top, where it is what keeps guest pid 1 and host pid 4821 from reading DISJOINT (§4.2). The
demand bites exactly where a leaf shape honestly cannot carry `:aliases-nothing-else`: two pids in one
namespace; two files in a container's overlay root. Verified by the route argument: the tops'
`:aliases-nothing-else` is never used.

Proposed second bullet, for the human: "Otherwise, with each leg's top the child of A on its
side: DISJOINT iff either (a) the two tops are mKeys of one mScheme, each carrying
`:guarantees-unique-name`, with differing values, and every mKey strictly below its top, on both
legs, carries `:aliases-nothing-else` for its shape; or (b) one mKey's mParent is A, the other's is not,
and the other mKey's shape carries `:aliases-nothing-else`. Else UNKNOWN. A store is never among its own
contents, which (b) rests on; a key against its own container still reads UNKNOWN, since a write
to a container must collide with everything inside it." The exercise's render and `311q` §4
stand only once this lands, and the exercise carries a second defect that lands with it (§3,
`forwarding-alias`).

### 2. `thr-cell-fold-lost-the-per-cell-store` — 311m §1.3 · 311n 2; +SURE

§1.9 makes a mCell an mSort whose primary mScheme is `:identified-in` its bearer, with a
singleton key, and in the same paragraph says `enabled` and `active` have chains that "differ
exactly where one survives a reboot and the other does not, with no declaration about reboots by
anyone". One bearer has one chain; two cells whose identity is that chain plus a property differ
nowhere above the property. 311 §1.8 and §2.7 gave each aspect its own store (`enabled` in the
filesystem holding the symlink, `active` in the boot) and §3.3's lifecycle rule did the rest;
the fold kept the sentence and dropped the mechanism. The systemd owner must now pick one parent
for the bearer: the filesystem, and a post-reboot `systemctl start` elides on a pre-reboot
`active` measurement (wrong SAME across a lifecycle line, 311n's patch-day book); or the boot,
and `enabled` is re-probed for nothing. `312b` §9's own sketch (`sm.UnitEnablement` under
`sm.UnitFile:nginx`) quietly uses two bearers, which is 311's shape under another name.

Repair, minimal and agreed by both passes: a mCell's primary mScheme is `:identified-in` the
store its value lives in, and the bearer supplies only the natural key the cell borrows for its
own. That is 311's aspect-sort restored inside the two-species vocabulary: still no aspect
species, still one sort per cell, but per-cell stores. §4.2's refutation of the aspect species
stands; what it must not refute is the per-cell store.

### 3. `thr-identifying-store-makes-siblings-collide` — 311l §1 · 311n 4; +SURE on the text

§2.4 makes the identifying store one of a sort's mPlacements; §2.5 lets a finished definition
spare an KNOWN_UNSPOKEN pair only where no mPlacements overlap. Two cells of one bearer (`mode` and
`contents` of one file; `enabled` and `active` of one unit) are different sorts under §1.9, so
KNOWN_UNSPOKEN, and share the bearer as identifying placement: overlap, collide. `chmod` walls every
contents fact of the same file; `systemctl enable` walls `is-active`; and nobody has a sentence
against it, not even Tessa, who owns both cells. Repair, both passes: the identifying store
answers identity and leaves §2.5's overlap test; placements answer interference; a write to a
container is §3.3's routing or lifecycle perishing. With thread 2 the two cells' stores differ
anyway; the exclusion still matters for cells that share a store.

### 4. `thr-aliases-nothing-else-is-the-parents-knowledge` — 311l §3 · 311m §1.2; `312b` §7 found it against 311; +SURE on the seat, ~SUSPECT on frequency

§2.2 has the child's primary owner declare `:aliases-nothing-else` per shape of the child's bytes, and says
five sentences later that the child "never learns the mParent's types". An inode number carries
nothing about ext4, overlay-lower, nfs, or sshfs. Tessa must declare blanket, which is false on
every docker host (an overlay lower inode is reachable through the merged view), or not at all,
which forfeits every file-versus-file survival and kills stage 5 for the commonest line in every
book. Whether a store's contents are reachable from outside it is the store's nature, compartment
or view, and its owner classifies it per shape one level up already. Repair: `:aliases-nothing-else`
becomes the parent shape's claim about edges into it ("what is identified in me is reachable
only through me"; ext4 and a network namespace yes; overlay-lower, nfs-client, a pid namespace
no), and the child declares none. Rachel's `net/*` `:aliases-nothing-else` becomes Simon's on the
namespace. Thread 1's rule then reads the parent's declaration at each level. `312b` §7's
`fnd-two-epistemic-seats-mis-sited` said this and 311j did not take it.

### 5. `thr-strangers-spare-through-a-finished-definition` — 311m §1.1; one pass, verified; +SURE of the walk, ~SUSPECT of frequency

`echo 0 | sudo tee /proc/sys/net/ipv4/ip_forward`: the path is argv, so 30T's redirect binder
never sees it; Tessa's path lookup stats it (procfs regular files stat), and tee's footprint is a
File cell. Line 10's knob is another sort, so KNOWN_UNSPOKEN, and sparing rides File's finished
definition, which the stdlib must ship or every drifted `cp` walls the book, bounded by §2.5's
placements: File's (a procfs device) and KernelParam's (a namespace) are different sorts and
never overlap by any reading. Spared; the host stays dark. Two consequences. §1.2's "KNOWN_UNSPOKEN
at the chokepoint (collides for sparing)" holds only until the footprint side finishes its
definition; this is 30U's knife, a finished definition false for one shape (writing a procfs
file is not "nothing else"), attributed to Tessa, and the userspace mitigation is 30T's
allowlist reasoning applied to her finished definition per filesystem type. The text should say
so rather than "collides". And §2.5's bound is unstated for non-SAME placement pairs: if only
SAME collides, strangers in different vocabularies never collide (this walk); if any non-DISJOINT
collides, cross-sort placements always collide and the finished definition can never spare
stage 5's `systemctl` past `apt-get update`. Neither is what was meant. Both fable passes also
note that 311j removed 311's cross-sort SAME generator (a shared store with equal primaries), so
one referent under two sorts has no positive route but a human merge; 311m proposes the store's
owner declaring that two primaries share one key space in it. A thread for 311 at a higher bar,
not adopted here.

### 6. `thr-floor-vouch-broader-than-its-justification` — 311o 1 · 311n 5 · 311l `amb-route-claim`; two lineages; +SURE

§1.10 vouches that "two same-spelled mKeys of one mScheme in one unwalled span are one
mPlaceholder, because one shell process resolves one cwd-qualified mKey in one mount table", and
§3.2 reads one placeholder as SAME with no warrant. The justification is about paths and command
words; a floor key whose lookup lives inside a binary (a TCP endpoint behind a local proxy; a
round-robin name; a tool's store chosen by `$KUBECONFIG`) is resolved by nothing the shell holds.
311o's redis-behind-a-proxy case, as a single line, is the author's vouch exposed to GOTCHA 11,
not identity's business; with two same-spelled lines, or an `export` between them (311m's
`kubectl` case, which `plans/30S` already fences in the world), the rule mints a SAME the engine
cannot justify. Repair, safe direction: the vouch covers ambient instances within a span (one
placeholder per sort per entry chain, which is what the exercise used) and inherited instances;
same-spelled leaf keys of a floor spelling are separate placeholders and SAME only by warrant.
Cost: one deduplicated probe at steady state, which the exercise never valued. With it, widen
§3.3's routing species from `PATH` to any environment or cwd a lookup reads (`312b` §2's lead;
`30S`).

### 7. `thr-parentless-warrants-and-the-sentinel-are-open-world-claims` — 311m §1.4 · 311n 6; two lineages; ~SUSPECT on how the route inherits

§1.10 says the mRoute is never vouched across a transit; the exercise's walk said "the boot and
the route inherited through Nathan's sentinel". If keys minted inside a wrapper share the outer
route, a warrant on a parentless shape (311m's crontab: `guarantees-unique-referent` on a
singleton key) makes `sudo -u deploy crontab …` and `crontab …` SAME across sudo: a true local
sentence read as a claim about every wrapper. Under §1.10's reading a key minted inside a transit
has its own unvouched route and the pair is UNKNOWN. The text should state that the route never
inherits (instances inherit as objects carrying their own chains), that a warrant on a parentless
shape speaks within one route only, and that `:rootness` is the sole way to speak across routes.
The sentinel's "nothing else" is a finished-definition-class claim: a stranger's
`vendor.NetNamespace` over the same namespaces inherits falsely (311n 6), and §3.4 should price
it as 30U prices the record, the wrapper author's line, mitigated by strangers yielding into the
stdlib sort rather than minting beside it.

### 8. Text-level, lower

- `txt-supply-modes-vestigial` (311l, 311m): §2.1 still enumerates FIXED, SITE, AMBIENT, COMPUTED
  for the catalog while the store is "any seat"; align.
- `txt-two-roots-ambiguous` (311l): "two mRoots" should read "mRoots of two shapes"; two keys of
  one root shape compare by `:guarantees-unique-name`.
- `txt-warrants-span-scoped` (311l): read both warrants as holding within an unwalled span absent
  a perishing write, so Simon can warrant nsfs inodes honestly and a bind mount reads SAME.
- `txt-shape-is-bytes-only` (311l, 311m): a device number's type is not in its bytes; carried by
  rearrangement (a device number as a secondary spelling of Filesystem whose yield reads the
  mount table and yields a typed primary); cost, unstated: every `mount` line then perishes every
  file resolution below it under §2.8's flat default.
- `txt-ext4-in-boot-and-in-route` (311m): §2.2's two ext4 examples disagree; pick one.
- `txt-same-then-disjoint-attribution` (311m): §3.5's survival attribution must name the
  mCorrespondence when SAME-then-DISJOINT was used.
- `txt-placement-may-need-a-read` (311m): directory members as placements need a read.
- `txt-indexical-clause` (311l): §2.8's sentence reads as a mechanism; one sentence pointing at
  composition by yield closes it.

## § 2-applied-to-311j

One deletion, §3.5: "Two mSchemes of one mSort that resolve one string, in one mParent, from one
mVantage, to two different mKey-Primaries, where the primary mScheme's shape carries
`:guarantees-unique-referent`: at most one is right, so both are withheld and narrated." Raised by
311n 1 and 311o 3; verified: an mKey is a value plus its mScheme plus its mParent (§1.4), so one
string under two mSchemes is two questions (`sudo -u 1000` against `sudo -u '#1000'`; a branch
and a tag both named `release`), and §2.2 already forbids reading unequal tokens as contradiction
without `:guarantees-unique-name`, which the sentence did not even cite. Its failure direction
was refusal plus false attribution of two correct authors, the worst aid failure
(`271:rul-sin-ordering`). Removed, ahistorically.

## § 3-did-not-survive, or was already the horizon

- 311l `fnd-cross-transit-pairs-rest-on-horizoned-roots`: cross-host comparison needs a stdlib
  root, and every honest root token is one of the two horizons; recorded by its author as not a
  fault.
- 311l `fnd-observer-sorts-must-be-enumerated`, `fnd-sentinel-rests-on-a-privileged-read`,
  `amb-perish-touch-of-a-store`, `amb-key-against-own-container-is-asymmetric`: safe directions,
  noted, no action.
- 311o `local-route-promotes-lookup-identity` as a single-line catastrophe: a line elided on its
  own probe rests on the author's vouch, not on `compare()`; the identity half is thread 6.
- 311o `forwarding-alias-gets-separated`: `net.ipv4.ip_forward` and
  `net.ipv4.conf.all.forwarding` are one kernel field (the reviewer checked `devinet.c`), so
  Rachel's `:guarantees-unique-name` on `net/*` in the exercise is false, a strawman defect and a
  clean specimen of `distinct-names-alias-within-a-kind`; the model carries it by canonicalizing
  or withholding. The exercise record is wrong there and waits for thread 1, since both touch its
  render.
- Attacks that held, listed once by the reports and not re-walked: bind mounts and the netns
  `resolv.conf` overlay; hardlinks; recreate-under-a-name; a floor key through any wrapper;
  cross-route NFS; nested pids under 311j's rule; descriptors outliving path replacement; base
  and overlay as a mCompositeSort.

## § 4-what-this-redirects

In 311: §3.2 rewritten as thread 1; cells regain their own stores (thread 2) and the identifying
store leaves the placement test (thread 3); `:aliases-nothing-else` re-seated on the parent (thread 4);
§1.2 and §2.5 made honest about strangers and the finished definition (thread 5); §1.10 narrowed
(thread 6); the route and the sentinel priced (thread 7). Every thread is a statement about the
model; the one authoring-surface remark (a per-filesystem-type finished definition for files) is
userspace. Nothing reaches 312.
