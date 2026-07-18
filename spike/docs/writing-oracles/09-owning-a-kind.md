# Owning a kind

Most oracle authors never write anything on this page, and that is by design. The
members here belong not to a command but to a kind - the vocabulary itself - and
they exist for the handful of people who own one: whoever maintains the base
library's package kind, whoever minted `org.foob.Certs` back on page four. If you
minted a kind that only your own oracle uses, you are technically its owner and
can defer all of this. The page matters when a kind is shared: the moment two
authors' oracles name the same kind, questions arise that neither of them can
answer, because the answers belong to the vocabulary, not to any tool.

Kind members are spelled with the munged kind name in place of a command word:
`sm.dorc.Package` yields functions named `sm_dorc_Package__resolve` and so on.
Same mechanical munge as always - dots become underscores.

## Two names, one thing: the resolver

The comparison machinery you met on the footprints page works on names, and names
lie. On a real Debian host, `nginx` and `nginx-full` can be one package wearing
two names; two paths can be one file through a symlink. Watch the failure: a
wall's footprint says it disturbs `sm.dorc.Package:nginx`, a downstream fact is
backed by `sm.dorc.Package:nginx-full`, the strings differ, "disjoint", and a line
elides past a wall that really touched its state. Every other gap in the footprint
machinery fails toward running too much; aliasing is the one that fails silent and
dangerous. It is the single place a name must be more than a string.

The fix is one function, written by the kind's owner, because what "the same
entity" means is exactly what a kind's owner holds:

```sh
# dorc-lang/v0.1
sm_dorc_Package__resolve() {
   dpkg-query -W -f '${Package}\n' -- "$1" 2>/dev/null || printf '%s\n' "$1"
}
```

Called with an entity name, it prints the canonical name; a name it cannot answer
for falls through to itself. The engine then canonicalizes both sides of every
comparison in this kind, so aliases collide (that line above correctly runs) while
genuinely different entities stay distinct. There is exactly one resolver per
kind; a second declaration anywhere in the loaded world is refused loudly. A kind
with no resolver keeps plain name-comparison - today's floor, nothing revoked.

The judgment asymmetry to hold in your head while writing one: wrongly merging two
entities only causes over-verification - collisions that did not need to happen,
guards instead of elisions, value lost, nobody hurt. Wrongly splitting one entity
re-opens the silent skip the resolver exists to close. So resolve conservatively:
when the canonical form is uncertain, mapping a name to itself (and letting it
collide as a distinct name would) errs safe, and delegating to the tool's own
authoritative query - as above - beats reimplementing its aliasing rules. A
resolver body is a probe body; the page-three contract applies whole.

## What touching an entity drags along: reach

The footprint machinery compares within kinds. Here is the gap that leaves,
replayed from the admin's seat: some colleague's oracle honestly declares that
`hork tune` disturbs `sm.dorc.Package:nginx` - meaning, in their head, the whole
package, files included. But `sm.dorc.Package:nginx` and
`sm.dorc.File:/etc/nginx/nginx.conf` are different kinds, so a downstream
file-fact sails over that wall untouched. And the colleague cannot fix it: which
files a package owns is the package system's knowledge, not theirs. The owner
says it once, for everyone:

```sh
sm_dorc_Package__disturbance_reaches_only() {
   printf '%s\n' "$1"   : sm.dorc.Service
   dpkg -L "$1" 2>/dev/null   : sm.dorc.File
}
```

Read: a disturbance to a package, of any provenance, reaches at most the
same-named service, and exactly the files the package installed. The engine
applies this to every footprint coordinate of the kind, whoever emitted it - the
colleague's claim now covers nginx's files without the colleague learning
anything. Emission lines follow the footprint grammar: entities on stdout, the
implied kind riding the mark. The first line is static knowledge, resolved at
plan time; the second is a host question, run read-only at probe time, because
the true payload lives only on the host. Both shapes live in one body, and a line
can migrate between them as the kind's needs change.

Reach only ever widens claims - it makes footprints touch more, never less - so
its failure direction is the safe one: an over-broad reach walls too much; it
never licenses a skip. That is also why it applies to footprints only. A fact's
backing stays exactly what its probe read; nothing here inflates what a fact
claims for itself.

The member's long name is deliberate, and it introduces a naming convention that
now governs your judgment: `only` in a role name means complete-by-contract. A
member with `only` in its name is a survey, not a contribution - authoring it
asserts you have enumerated the whole answer, and consumers will act on its
negative space (everything you did not emit is thereby declared unreachable).
Survey totalistically before authoring one; members without `only` grow arm by
arm, members with it are all-or-nothing per invocation.

## Where state lives: the store member

The last member answers the humblest-sounding question with the widest
consequences: where does this kind's state physically live, and what does its
location not depend on?

```sh
sm_dorc_KernelParam__state_stored_only_in() {
   printf 'kernel-sysctls\n'   : kernel
   :   : invariant:fs-view
}
```

The emission lines place the state on a substrate - kernel memory here; a
filesystem path for most kinds (`printf '/var/lib/dpkg\n'   : fs` for the package
kind). The `only` contract applies with full force: you are declaring the state
lives in these places and nowhere else.

The bare colon-lines carry the interesting part: invariance declarations, one per
context dimension the state does not vary along. This is where kind ownership
meets the previous page. Kernel parameters are one store machine-wide, so a fact
about them measured outside a chroot is still true inside it: `invariant:fs-view`
says so, and licenses the engine to carry such facts across that boundary - after
it has independently verified, structurally, that the measuring check read nothing
else. A package's installed-ness does not depend on who asks: `invariant:user`.
What you must not do is flatter the kind: a network-parameters kind must not claim
`invariant:netns` (network state is precisely what varies per network namespace -
the model refuses that combination loudly), and the package kind must not claim
`invariant:fs-view` (every chroot has its own package database). Declare only the
invariances that are true of the substrate itself, and the engine turns them into
exactly the cross-context answers they justify; declare none, and facts simply do
not cross, which is never wrong, only quieter. A declared invariance that
contradicts the member's own emissions - a "user-invariant" store whose emitted
paths contain `$HOME` - is caught and refused at plan time.

## The stewardship

All three members share a social shape worth naming once, plainly. A kind, once
published and adopted, is a vocabulary other people build on: their marks mint
selectors in it, their footprints claim in it, their books benefit or bleed
through it. Minting one is cheap; owning one is a standing role. Document what
the kind means - what counts as an entity, what each selector token asserts, what
the canonical name form is - because strangers will use it exactly as carefully
as you explained it, and misuse of a kind against its owner's documentation is a
contract violation on the misuser. Collaboration inside your namespace will
happen whether you invite it or not; what the ecosystem owes you is the
attribution to adjudicate - your kinds carry your domain name - and what you owe
it is answers when adjudication is needed. There is no registry and no mechanical
enforcement behind any of this, deliberately: reverse-DNS naming makes every
vocabulary's accountable party legible, and the rest is the same human protocol
that keeps package names and Java packages coherent.

<!-- quoted: USER_STORY.md stages 6-7; 272 store member + invariance; 277
     sections 4e, 6 divergent-meaning ownership refinement; 271
     rul-at-most-family-names, rul-invariance-speech-act; plans/27C section 4;
     e2e carry-fsview fixtures -->
