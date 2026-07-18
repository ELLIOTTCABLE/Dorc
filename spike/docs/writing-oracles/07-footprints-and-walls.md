# Footprints: describing what a command disturbs

This page teaches the third role a command's oracle can carry, and it opens with a
warning label: this is the one part of the system where a mistake of yours can hurt
a line that is not about your tool. Everything before this page was safe to get
wrong in the cheap direction - a bad verdict endangers your own tool's lines, a
timid decline just runs a command. Footprints are sharper, the feature they feed is
off by default, and the discipline they demand is real. Read this page fully before
writing your first one.

## The problem: honest walls on drifted days

By now your book is well described. On a converged morning everything elides, elided
lines cast no walls, and the plan is two lines long. Then comes a drifted morning:
the package index has gone stale overnight - nothing else - and `apt-get update`
really is going to run. An honest wall is a wall: that one line, actually running,
means every downstream fact was probed before it and might be stale after it. The
whole tail of the book degrades to guards, on account of one line that everyone
watching knows perfectly well touches nothing but the package index.

Everyone except Dorc - because "touches nothing but the package index" is a claim
about a black-box binary, silence licenses nothing, and nobody has said it. It is,
note, a different *kind* of claim than anything you have written so far. A verdict
says "this state holds". This claim says "this command disturbs these things, and
nothing else" - and that "and nothing else" is the part no machine can check. It
can only be said by a person, about a tool they know, with their name on it.

## The disturbs member

```sh
# dorc-lang/v0.1
apt_get__disturbs() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb="$1"; shift
   case "$verb" in
   update) :   : sm.dorc.PkgIndex ;;
   esac
}
```

The member is invoked with a site's arguments, like its siblings. For an
invocation shape it matches, the body emits the entities that shape disturbs - one
per line on stdout, with the kind riding a trailing mark. Here the `update` arm
emits a single whole-kind claim (the package index is one thing; there is no
entity to name), using `:` - the shell's do-nothing command - as the emission
line. A tool whose disturbance depends on its operand emits it:

```sh
foobar__disturbs() {
   verb="$1"; shift
   case "$verb" in
   sync-certs|renew) printf '%s\n' "$1"   : org.foob.Certs ;;
   esac
}
```

The semantics of matching versus not matching carry all the weight, so here they
are slowly. A shape the body does not match emits nothing, which is silence: no
claim, the wall stands, exactly as safe as before you wrote the member. A shape
the body does match makes a complete claim about that shape: "this invocation
disturbs at most what I emitted - whatever else it might touch is residue I answer
for." There is no partial matching. You cannot say "it touches this, among other
things I have not thought about"; matching the shape *is* saying "and nothing
else". Which yields the two rules of footprint discipline:

When unsure whether a matched shape disturbs some cell, include it. Claiming too
much only walls too much - downstream facts collide with your claim and fall back
to guards, which is the safe, merely-less-valuable direction.

When unsure whether you have enumerated everything a shape disturbs, do not match
the shape. An incomplete enumeration is not a smaller claim; it is a wrong one,
and it fails in the dangerous direction.

Grow the member arm by arm, each arm only after a genuine survey of that verb -
its documentation, its file activity on a scratch machine, its source if you have
it. A disturbs body is also a probe body (the full page-three contract applies),
and it may ask the tool read-only questions at probe time when the honest answer
lives on the host - a package manager's file payload, for instance, is knowable
only where the package is.

Precision pays. Claims can carry selectors (`printf '%s\n' "$1"   :
sm.dorc.Service#{enabled,active}` claims two cells of a service), and the finer
your claim, the fewer innocent downstream facts it collides with - a whole-entity
claim collides with every fact about that entity. But precision is a refinement of
a complete survey, never a substitute for one.

## What footprints buy, and what the buyer pays

With footprints in place, the engine can do something it never could before: let a
proven fact survive a command that actually runs. Every probed fact already knows
where its own truth lives - the cells its check read, which your marks and observe
disclosures made explicit. When a wall with a footprint runs, the engine intersects
the footprint against each downstream fact's backing. No overlap, provably: the
fact survives, and its line stays elided even though something upstream really
executed. Any overlap, or any unknown: exactly the old world, guard or run. On the
stale-index morning, the index refresh runs alone and the rest of the book keeps
its shape - the entire point.

Now the price, stated as plainly as the design states it to admins. An elision
that survives a running wall has no runtime net under it. Nothing re-checks it at
apply; that is what surviving means - a re-check would be a guard, and the
attention cost would be back. It rests entirely on your "and nothing else" being
true. If you forgot that `sync-certs` also rewrites a service unit, the elision
your claim wrongly spared belongs to a different tool, a different author, a
different file - and that line silently fails to run. Skipping needed work is the
single failure the whole system is designed never to commit, and this is the one
feature that can commit it on a human's mistaken say-so.

Accordingly, the feature is fenced. It is off by default; admins opt in per
invocation with a flag whose name says what it risks (`--risk-faultless-skips` in
the current spelling), and the design is honest that the consent is the point -
nothing about the flag makes a wrong claim safer. Both ends must act: your clean
claim and the admin's typed flag. Every survived elision is attributed - the plan
records whose footprint it rested on, and the why-machinery names you. And the
bite, when it comes, is short and narrow: the very next plan re-probes reality,
finds the fact genuinely diverged, and the line comes back; nothing compounds.

Weigh that soberly and it comes out worth doing, for the tools you truly know.
Churn-heavy, early-in-book commands with well-understood state - index refreshes,
log rotations, cache warms - are precisely the ones whose honest walls cost books
the most, and precisely the ones whose authors can survey completely. That is the
work: say only what you surveyed, match only what you enumerated, and the drifted
mornings stop costing everyone the rest of the book.

<!-- quoted: USER_STORY.md stage 5 + bought-unsoundness; 277 sections 3, 4c;
     spike/CLAUDE.md rul-flag-is-razor-residue, sparing-algebra,
     set-lifting-universal-meet; 271 rul-touches-becomes-disturbs,
     rul-at-most-family-names -->
