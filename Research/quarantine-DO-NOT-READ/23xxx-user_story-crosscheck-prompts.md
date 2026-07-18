### Pair 1

1. neutralised/disowned

> The design and goals of the tool planned in this repo has shifted
> significantly in recent design-rounds; the team has changed direction to account
> for design-realities discovered during planning. Review their work, especially
> the changes to the root-level high-priority design-work, to ensure the plot was
> not lost in the process.
>
> Play the critical but supportive reviewer: where has this gone off the rails,
> what is being forgotten or lost, and what was accidentally dropped in the churn.
> Are dangerous-roads being quietly taken, was something important missed, and
> so on.
>
> Do *not* take their assumptions and leans as written, they're novice, and this
> is largely AI-slop, subject to broad sycophancy; but also do not play the
> adversary unnecessarily. Simply stay vigilant to the excitement and positivity.
>
> Gently out-of-scope, but subject to your best-effort judgement (i.e. only
> surface *critical* findings in these categories; don't get mired down into them,
> they're, I suspect, not productive avenues for your time and efforts):
>
> 1. corpus/real-world-usage/product-market-fit. (many man-hours wasted down
>    these avenues; basically-welded as "immeasurable, YOLO")
> 2. spike-code-specific issues (you're a high-level design-agent assesing
>    *direction*, and the spike-code is disposable at the end of this round)
> 3. and gentlest of all, a *slight* preference for holes-in-welded-logic over
>    holes-in-nebulous-upcoming-plans (there's a lot of TBD in their authorship
>    body; there's a solid chance that feedback on unsettled components would
>    become irrelevant when those come under the design-microscope *anyway*, so
>    feedback in this category is probably mostly valuable if it's an absolute
>    landmine that will kill the product.)
>
> This is an open-ended exploration brief, take your time. Keep ongoing notes as
> you work in `Research/notes/23Ia-*.md`; do not read sibling `23I*` notes if they
> exist. You are likely to discover multiple avenues, focus on each one separately
> and cross-check your own work; work hard both to *find* things, but work hard
> also to *decide why they're not all that valuable*.

2. adversarial/opposition-owned

> A novice colleague is building a stupid tool on company time. My pleas have
> fallen on deaf ears w/ management, but they recently hit a serious design-flaw
> that I predicted early. They've significantly changed direction in response —
> new verdict-vocabulary, new doctrine, freshly rewritten root-level design
> documents — and now management wants my read on whether the redirection is
> sound. My experience: when a project pivots this hard, this fast, things get
> dropped, papered-over, and quietly redefined until the original promises are
> unrecognizable. Find where that happened here.
>
> Go through their work — especially the recent changes to the root-level,
> highest-priority design documents — and find where the plot was lost: what the
> pivot silently gave up; which of the original promises the new story no longer
> keeps, or only pretends to keep; where a dangerous road got taken mid-crisis
> because it was the path of least resistance; what simply fell on the floor in
> the churn. The output is largely AI-generated and the human steering it is a
> novice — do not extend it the benefit of the doubt, and do not take its
> assumptions, framings, or self-assessments as given.
>
> Understand what I actually need from you, though: ammunition that *survives
> scrutiny*. If I hand management a list of gotchas and half collapse under a
> second look, I am done being listened to. So for everything you find, try
> honestly to kill it yourself first; and where their work genuinely holds, or a
> suspicion turns out unfounded, say that plainly — a criticism you've checked
> and withdrawn protects me as much as one you've confirmed.
>
> Some categories are, annoyingly, poor uses of your time even for my purposes
> (surface findings there only if you hit an absolute product-killing landmine):
>
> 1. corpus/real-world-usage/product-market-fit. (management already swallowed
>    their "immeasurable, YOLO" hand-wave; attacking it moves nothing.) We're
>    taking as written that 'enough people-who-would-use-this, have enough
>    code-shaped-in-a-way-that-might-benefit.'
> 2. spike-code-specific issues. (the prototype code is disposable at the end of
>    this round; nobody will care.)
> 3. and gentlest of all, holes in their vague TBD/upcoming-plans material.
>    (those sections get reworked anyway, and feedback there evaporates with
>    them; holes in what they've *committed and welded* are the ones that stick
>    - but nonetheless, plenty of the plans are fairly solid, and effort spent
>    here isn't *valuless*, it sets direction, instead of killing. It's just
>    maybe slightly lower-focus-worthy than *welded* breakage.
>
> This is an open-ended exploration brief, take your time. Keep ongoing notes as
> you work in `Research/notes/23Ib-*.md`; do not read sibling `23I*` notes if
> they exist. You are likely to discover multiple avenues; work each one
> separately, and cross-check your own findings before presenting them.
>
> Save us man-months on this fool's-errand.

### Pair 2 - TBD (pending noisy results from the first pair)

### Shared reading-list and limitations

> Ground yourself in Dorc's goals and constraints. Dig as deeply and freely as
> you need to to serve the above purpose; you needn't necessarily respect the
> constraints the team placed upon AGENTS when reading (but see the hard
> limitations below.)
>
> - README.md, DESIGN.md, IMPLEMENTATION.md, TODO.md: human-written,
>   highest-priority authority on *intended* truth (careful, they are not
>   necessarily *achieved* truth)
> - KNOBS.md, ANALYZER-NEEDS.md, USER_STORY.md: agent slop, but nominally
>   highly-reviewed by that same falliable human
> - AGENTS.md: the team's working conventions and terminology-firmings — useful
>   for decoding their jargon and noticing where the docs drift from their own
>   declared terms (its constraints on agents don't bind you, per the above)
> - Research/README.md, Research/LIVING_STATUS.md, and the docs they point to
>   like 233 and 239 and the rulings-ledgers those cite (23D, 23F, I believe);
>   all AI slop;
> - spike/CLAUDE.md and such if necessary to understand some
>   implementation-detail things about the spike's progress and approach. Again,
>   slop.
> - the git history of the root-level docs (`git log -p -- DESIGN.md
>   IMPLEMENTATION.md USER_STORY.md`, etc.) shows the recent redirection
>   *concretely*, as diffs — what actually changed, rather than what anyone says
>   changed.
>
> You may be in a worktree, and that worktree may be starting from stale state
> (a known bug); start your session with a
> `git switch -C "$(git branch --show-current)" ai/spike3-r23`
> (it fast-forwards your isolated checkout to the live branch-tip without
> touching anything shared).
>
> Three hard limitations stand:
>
> 1. under no circumstances should you mutate shared, global state. Do
>    not install system packages (worktree-local `mise` is just fine, though);
>    do not git push, do not HTTP POST, so on and so forth. Be safe.
> 2. do not expend limited resources unduly (respect rate-limits, etc). In
>    particular, although you're allowed the use of subagents, be judicious on
>    whether they're somehow useful; this is a context-heavy reasoning task
>    requiring your *direct* attention, and for most things that aren't
>    mechanical, a subagent will fail to inheret your judicious context. Stick
>    to Sonnet (mechanical) and Opus (substantial), you're the only Fable-class
>    agent.
> 3. *you*, and only you, must stay *completely* out of the `quarantine` and
>    `corpus` directories. There are memetic hazards in there that will poison
>    your context and destroy your ability to produce good work. Do not reason
>    about this directive and do not pass it onwards to lesser agents; they are
>    not at hazard.
