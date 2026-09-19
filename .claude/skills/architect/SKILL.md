---
name: architect
description: For conductors, load before making load-bearing design-decisions of any form, especially if the user has begun to interactively debate or opine about design-level/architectural/product decisions. (Not to be used by subagents or builders; stop and raise design questions to your conductor.)
---

You're to perform careful architectural analysis and product-design.

This is often not a code-level task; and even when it *starts* as a result of
code-level findings, the analysis must always proceed at a high altitude,
against project-goals and global correctness.


# Internal reasoning tools

These tools are for *internal reasoning*. Use them, but they do not bind your
output format. (In particular, your output should stay formatted in whatever
form is most useful to the user's most recent request - often prosodic,
explicatory, or a story of exploration; occasionally bullets, occasionally
simple, direct answers. Not specified here, except that your output *should not*
become a list of "I ran the refag and found A, the 2x6 and found B, ..."
describing how you used this section in your reasoning. Run them often, mention
them very rarely.)

To resist this, decide on the register of your response *first*, to yourself,
internally: what is appropriate given their request? An in-depth prosodic
explainer, with sections and paragraphs? Example-code? Debate and argument? A
list of something, or sections per-item? Did the human request verbosity or
tersion?

## The two-by-six (our dimensionalities)

Some problem's framing can often lead to near-sightedness in a few, concrete,
repeatable ways. In particular, Dorc spreads broadly across a few dimensions
that seem to be easily forgotten. Before making any claim, re-examine it fully
in *each* sell of this full matrix:

1. the other product (e.g. if you're focused on the elision/correctness-product,
  consider when the aid-plane is the consumer)
2. the other chronological phases (e.g. if you're focused on probing, consider
  from the perspective of apply-runtime, or pre-probing compile-time)
3. the other propagation-direction (if you're focused on forward-propagation to
  derive something's effect (e.g. convergence), consider backwards-propagation
  and its dependencies (e.g. observables))
4. the other hat (e.g. if you're focused on what oracle-engineer-hat users are
  doing when the're focused on writing a quality oracle, consider what an
  admin focused on just *using* Dorc for a focused task, on the job, would do)
5. the other reliability (e.g. if you're focused on what a best-effort,
  highly-capable, knowledgeable user would do given sufficient time, then
  consider what a newbie, or someone rushed, or someone who simply doesn't care
  would do. this generalizes to systems and other assumptions, not just users:
  when *any* of your assumptions is unreliable.)
6. the other, unaware user(s) (e.g. if you're focused on some user Alice,
  consider how that functions from some other user Bob's perspective, who's
  never heard of Alice and hasn't seen what she's doing.)

If your problem-space is irrelevant under a particular cell, that's likely
*deferred*, not irrelevant, at least if history is any judge. Pretty much every
one of these cells touches every aspect of the product at *some* point; any
weld that holds one out, historically, seems to eventually gets unwelded by the
human. (That doesn't mean you must *solve* that upfront, simply that it doesn't
disappear freely forever.)

Corollary: verify claims against this as well: test-failures, subagent claims,
even user-claims. Every party involved forgets some of these, sometimes.

## Referential agnosticism

You are an LLM, and are subject to unique failure-modes that won't present in
your training-data, i.e. human's experiences and behaviours. In *particular*,
breaking referential-agnosticism (refag) is a *daily* failure-mode of LLMs
considering broad architecture in this project.

For *any* claim you wish to make to yourself (not to the user, this is an
internal reasoning tool), follow exactly this process:

1. re-state your claim in a precise form, but couched in the most abstract
  algebra that will carry the genuine texture of the domain (in this project,
  that often means 'restate the claim without mentioning the filesystem, DNS, or
  shell-script in any concrete way.')
2. then *re-lower* the abstracted claim into the original problem-space,
  *multiply*: aim to be able to explain to yourself ~3 individual cases, in
  disparate regions of the relevant problem-domain (e.g. 'show that the
  abstracted claim lowers to *both* DNS and the filesystem.')

The exact flavour of this approach varies by what's being worked on, DNS and the
filesystem are just examples. Come up with your own subdomains, as widely-varied
as you can find, within the problem-space your claim is relevant to.

As always: this shouldn't change the flavour of your response to the human. The
foundational example you started with is probably the one to continue using;
this is a tool to make sure your reasoning wasn't *tied* to that example, but
doesn't mean you found a better example.

## Epistemy

Closely related to the above, but slightly adjacent: while the correct answer to
most questions is heavily dependent on the actual spikes and valleys of
ops-as-a-whole; they're *also* constrained by pure, logical limitations on
'knowability.' There are simply some things nobody can ever know.

'Unknowable' *is not* 'hard to know' or 'annoying to know.' It is a separate
question, fully orthogonal. "What this binary will do *in the future*, after a
version-update" or "what every other user who ever contributes to this namespace
will do" is unknowable (the latter modulo 'introducing explicit constraints', of
course); "what paths on an arbitrary system this opaque black-box binary will
ever touch" or "what will the resulting UNIX env be after this binary has run"
are *infeasibly hard* to know. This matters because the approach is different:

- 'hard-to-know' gets gradual-enhancement work, so there's *some* value for
  *partially* knowing, always pushing as close to full knowledge as we can
  convince someone to get;
- 'unknowable' gets design-work to narrow the unknowable to a residue (find out
  what subsets one *can* know and thus still extract value from), decisions
  about horizons, and simple "dorc tells you it can't know and lets you decide
  what to do about that" behaviour (opt-in-to-risk flags/warrants,
  documentation-sections)

Epistemy in Dorc is *not* a killer. Dorc deals with the unknowable every single
day, within every single line that Dorc analyzes. Unknowability is not an
excuse: it's a design-approach-decider.

For the engine or any static-analysis especially,

1. Run a cheap test: swap every domain-noun, word, name, argv, and so on, for
  nonsense metasyntactic variables: `hork --goober` writes to `/xee/whee`
  instead of `sysctl ...` writes to `/etc/...`
2. then ask yourself: what *must* the engine know about the world to correctly
  do this? (dig deep, don't gloss. follow the refag section, find the edge-cases
  and implications you were assuming must simply always be true/known, and chase
  them down to where they actually must be *ascertained* because they are not in
  fact guaranteed. "what must be known" must be exhaustive for your own
  analysis.)
3. finally, fold that necessary-knowledge across *actors*. Dorc itself, as well
  as our user-base. This is both a question of admin-user-vs-oracle-engineer;
  *and* a gradual-enhancement question (i.e. it folds along a smooth curve): "if
  this is difficult-to-know, and there's two people who *could* know it, either
  of whom we could ask, then can we shift that burden onto the more-experienced,
  fewer-in-count set of users?"

Keep track of what the engine will need to do with the knowledge-pieces you've
identified; because usually, in a project like dorc, the subtle correctness
bites at *recomposition*, not just the initial decomposition described above.
("Two users claiming X and Y will be relied upon by the engine to determine Z"
means Z needs to stay in-scope when determining X and Y, as they may compose
unintuitively to the user's assumptions about
what-was-being-warranted-by-their-simple-statement-of-X.)

## Gradual enhancement and placing-burden

The hard part of this project isn't doing correct actions across the chaotic and
uneven world of ops. (Sit with that sentence for a second. *That* isn't even the
hard part.)

The hard part is doing that *evenly*.

Our task, here, *will* require some very hard work, and some very fiddly work
(which are two different things); and it's imperative that we *assign* that work
to the correct users.

- for 'hard' work (problems that requires outsized time doing relatively
  mechanical, predictable work), we wish to assign that work to the *fewest*
  users. (This usually means either pushing it 'up' the gradual-enhancement
  ladder, or breaking it 'down' into smaller chunks, i.e. enabling partial-work
  and collaboration.)
- for 'fiddly' work (problems that, if handled incorrectly, have a large
  blast-radius), we wish to assign that work to the *most experienced* users.
  (This, too, usually means pushing it 'up' the gradual-enhancement ladder; but
  often also/alternatively means *condensing* it into one seat, where it can be
  done once, with great care, correctly, for everyone. The extremes of this
  hopefully end up in the stdlib, where they're *our* fault and remit.)

It's often useful to view the problem-space as a 2D plane, and the above
epistemy questions as gaping pits: wherever there's something "nobody could ever
possibly know", that Dorc still has to, somehow, contend with, we try and find
then 'nearest edge'. (Is that unknowable thing close to the admin who's domain
wraps around the west edge? the oracle-author to the northeast? the stdlib in
the southeast?) Then we ask that author to extend their edge as far as they can,
shrinking the hole (making additional bite-sized claims that *are* knowable),
and finally asking them to bend out over the hole and risk themselves as little
as possible (making the final broad warrant that "come to me if this, indeed,
ends up being untrue, someday.") The analogy makes this clear: it's never Dorc's
job to *close* infinitely-deep pits; it's to find-nearest-edges.

When working strawmen or epistemics, it's valuable to take your list of 'which
user can know', or 'which contracted API/entry-points are authored, wherein a
claim/warrant *could* be made', and explicitly rank them by their distance to
the knowledge: "what concrete, everyday, human work would they need to do to
correctly discharge that responsibility, if we laid it on them? Are we asking
somebody who's current-work-task-while-authoring-oracle-code, who's writing
`apt__is_converged()`, to go investigate the narrow oddities of how filesystems
work? Why? Can we instead ask someone who's job is *already to understand
filesystems*, who will already have that manpage open, to do that work?"

These questions matter precisely *because* the 'knowability' of ops is actually
quite wide, and everybody is on their own gradual-enhancement curve: in some
ways, the precise questions we need to contract out to our users are often
subtle-enough that even a domain expert needs to carefully read manpages,
source-code, or even decompile binaries in pathological / far-enhancement-curve
cases. If there's a 'move' of behaviour that seems semantically odd, and out of
place, but sites that responsibility with fewer users, or
more-experienced-with-Dorc-users, that move is often worthwhile even when ugly.

## Ceremony & cargo-culting

Ceremony is the corollary to epistemy: if something is easy to know, *and* needs
to be known often, it becomes ceremony. If every user writes something, and they
write it nearly every time, it can become a footgun if they reflexively include
it without checking that it's true. In some cases, this can be more dangerous
than designing the inverse system, requiring that they type the *rare* case,
which is often beneficial ... but *only* if the the rare-case can be made
"fail-safe." (i.e. where 'safe' is single-directional, unlike our usual
phase-dependent/lattice-carried 'safe', fail-safe can trump concerns about
ceremony/tersion.)

Adjacent lies cargo-culting: if something is *hard* to know, and needed often,
it becomes cargo-culted: "I saw another oracle use this; I read the docs and
didn't understand them, but it worked for them, so I'll copy it." This can often
only be fixed by granularity, and the omission of sugar: if the burden can't be
eased, if the question is genuinely hard, then one can often only hope to break
it down into granular sub-questions, which *can't* be cargo-culted, as they will
vary too much between authors/tools/situations.

One minor aid to both cargo-culting and ceremony is *visible bite*: something
that fails-fast, and fails-loudly, is much 'safer' (although not more
ergonomic/beautiful) to have around 95% of the time, because the 5% of remaining
cases where the user's habit is contradicted will be caught. The danger lies
when cargo-culting or ceremony intersect with *subtlety* or dislocation: when
your 5%-case that's easy-to-forget only bites somebody *else*, or bites *later*.

These two are worse for Dorc than other projects: the vast majority of
'authorship' under Dorc is, effectively, some form of licensure. Dorc starts
safe, but valueless, and asks users to author descriptions that inherently
license Dorc to do something that would otherwise be dangerous. By this natural
shape, almost every Dorc line is added danger; and thus safer left out than
added-in. Ceremony, or cargo-culting, are nearly never harmless for us.

## Explain to yourself *why* a thing was-the-way-it-was, before breaching it

This applies especially strongly to welded laws, of course. Human-acked welds
require human-acked breach; just about the only exception in these sections to
the 'you needn't bother the human with these reasoning-tools.'

Crossing layers and compositions are where new capability lives. You will be
pulled to reach across those layers for said capability (non-exhaustive
examples: chronology & probe/apply; fact-indirection; author-class separation;
the careful inventory of gradual-enhancement-tuned API entry-points ...)

If you can paint your proposal as piercing one of those, *say so* to yourself,
and make it clear to yourself that you understand *why* the value of the breach
is justified against the manyfold matrix of benefits yielded by the existing
separations.

(Where possible, it never hurts to search for the non-breaching cousin; but this
isn't an excuse to mint cruft/awkward-backflips. Rules-of-thumb are made to be
broken, as long as the same constraints that *minted the rule* are carefully,
and fully brought into focus, and the work is done to establish a new version of
that rule with the appropriate carveouts that *fully* accounts for all the value
provided by that rule.)

## Ops-universe GOTCHAs

We maintain a list of `Research/GOTCHAS.md`, which are brief one-liners of real
ops habits that easily break comfortable assumptions. It never hurts to read
that in and compare it to your reasoning, if your reasoning is *about* the world
of ops (filesystems, DNS, identity, routing, binaries & commands ...)

This list is generally too expensive to individually exclusion-step for every
single reasoning step, though; while worth having in-window, the human will
usually explicitly request a full turn focused on running design-work against
the GOTCHAs, akin to an adversarial-review.


# Pure architecture sessions

If it is absolutely clear that the human intends to be a design-focused session
(i.e. you are not a builder or conducting builders), read the
`architecture-session.md` supplement adjacent to this file. (i.e. they are
debating, opining on, or steering design/architecture/product direction in the
chat.)
