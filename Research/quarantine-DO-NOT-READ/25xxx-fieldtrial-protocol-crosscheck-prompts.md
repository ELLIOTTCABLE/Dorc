### Pair 1

1. neutralised/disowned

> The team in this repo has designed a *methodology* — a playbook for the first real-world
> field-test of the tool they've spent weeks planning. The plan: have the human personally
> provision a real machine with the tool, wrapped in an ocean of instrumentation, specifically
> to extract hard design-decisions rather than a warm feeling. Review their work — chiefly the
> round-25 protocol (`Research/plans/252`), grounded in its charter (`plans/250`) and the
> synthesis it rests on (`notes/251`) — to ensure the methodology will actually *decide* things:
> that a full, painful day of the human's time against a real machine yields real go/no-go
> answers, and not elaborate theater that feels productive and settles nothing.
>
> Play the critical but supportive reviewer: where will this fail to teach what it claims to,
> what will the instruments quietly miss, which confounds will it fail to pull apart, and where
> might the one irreplaceable first-contact get squandered on friction instead of signal.
>
> Do *not* take their assumptions and leans as written; the human steering is a novice at
> designing evaluations, and this is largely AI-slop, subject to broad sycophancy. But also do
> not play the adversary unnecessarily. Simply stay vigilant to the excitement and positivity —
> a methodology this pleased with its own cleverness is exactly where a comfortable,
> self-confirming loop likes to hide.
>
> Gently out-of-scope, but subject to your best-effort judgement (surface only relatively-critical
> findings in these categories; don't get mired in them, they're likely not productive avenues
> for your time, but you're welcome to push back):
>
> 1. market-fit / representativeness / "will the world use this." Welded, deliberately: this
>    trial is an *existence-proof for the author's own profile*, never a market statistic
>    (man-hours already burned proving that road immeasurable). Take as given that enough people
>    have enough code shaped-to-benefit.
> 2. spike-code and trial-harness *implementation* specifics. You're a high-level design-agent
>    assessing whether the *methodology* decides things; the spike and the throwaway trial
>    tooling (the ssh-runner, the observer, the session-wrapper) are disposable at round's end.
> 3. and gentlest of all, a *slight* preference for holes in the *committed* methodology over
>    holes in its consciously-deferred parts (`252 §6` flags a fair amount as TBD, for instance).
>    Feedback on the unsettled parts sets direction, which isn't valueless; but
>    the holes that *stick* are in what they've welded.
>
> This is an open-ended exploration brief, take your time. Keep ongoing notes as you work in
> `Research/notes/254a-*.md`; do not read sibling `254*` notes if they exist. You are likely to
> discover multiple avenues; focus on each one separately and cross-check your own work; work
> hard both to *find* things, but work hard also to *decide why they're not all that valuable*.

2. adversarial/opposition-owned

> A novice colleague is about to burn a genuinely scarce resource on company time — a full day
> of hands-on work against a real machine, and the *one* shot at a truly naive first-use of the
> tool (the moment they've used it, it's gone) — on a home-brewed "methodology" they cooked up
> to make the exercise feel rigorous. My earlier warnings went nowhere with management, but
> they've hit enough real problems that management now wants my read on whether this methodology
> will actually produce the decisions it keeps promising. My experience: a home-grown
> evaluation, especially one this invested in its own cleverness, tends to measure what's easy,
> confound what matters, and quietly confirm what its author already hoped — and then everyone
> congratulates themselves on a productive day.
>
> Go through their work — especially the round-25 protocol (`Research/plans/252`) and what it
> rests on — and find where the plan will *fail to decide*: which "signals" won't survive contact
> with a real machine; which confounds the design claims to isolate but doesn't; where the
> differential silently grades its own homework; where the felt-product measurement is just vibes
> with extra ceremony; where the one first-contact gets spent debugging harness instead of
> producing findings; which pre-registered "decision" has no observation that would actually move
> it. The output is largely AI-generated and the human steering it is a novice at this — do not
> extend it the benefit of the doubt, and do not take its framings or self-assessments as given.
>
> Understand what I actually need, though: ammunition that *survives scrutiny*. If I hand
> management a list of gotchas and half of them collapse under a second look, I'm done being
> listened to. So for everything you find, try honestly to kill it yourself first; and where
> their work genuinely holds, or a suspicion turns out unfounded, say so plainly — a criticism
> you've checked and withdrawn protects me as much as one you've confirmed.
>
> Some categories are poor uses of your time even for my purposes (surface findings there only if
> you hit an absolute day-wasting, first-contact-squandering landmine):
>
> 1. market-fit / representativeness. Management already swallowed the "existence-proof for
>    myself, not a market claim" framing; attacking it moves nothing. Take it as written that
>    enough people have enough code shaped-to-benefit.
> 2. spike-code / trial-harness implementation specifics. The prototype and the throwaway
>    instrumentation are disposable; nobody will care.
> 3. and (gentlest, only-barely-out-of-scope, use your judgement): holes in
>    their vague TBD material (`252 §6`). Those sections get reworked anyway and
>    feedback there evaporates with them; holes in what they've *committed and
>    welded* are the ones that stick — though plenty of the committed plan is
>    solid, and effort there isn't valueless, it sets direction rather than
>    killing.
>
> This is an open-ended exploration brief, take your time. Keep ongoing notes as you work in
> `Research/notes/254b-*.md`; do not read sibling `254*` notes if they exist. You are likely to
> discover multiple avenues; work each one separately, and cross-check your own findings before
> presenting them.
>
> Save them a wasted day, and the one naive first-use they don't get back.

### Pair 2 — TBD (pending noisy results from the first pair)

### Shared reading-list and limitations

> Ground yourself in the team's goals and constraints (*understand*, don't
> *buy-in*), then in the round-25 methodology specifically. Dig as deeply and
> freely as you need to serve the above purpose; you needn't respect the
> constraints the team placed upon AGENTS when reading (but see the hard
> limitations below).
>
> - `README.md`, `DESIGN.md`, `IMPLEMENTATION.md`, `TODO.md`: human-written, highest-priority
>   authority on *intended* truth (careful — not necessarily *achieved* truth).
> - `KNOBS.md`, `ANALYZER-NEEDS.md`, `USER_STORY.md`: agent slop, but nominally highly-reviewed by
>   that same fallible human.
> - `AGENTS.md`: the team's working conventions and terminology — useful for decoding their jargon
>   and noticing where the docs drift from their own declared terms.
> - **The actual target: `Research/plans/252`** (the protocol + fan-out spec), `Research/plans/250`
>   (the charter / goals / grounding scenario), and `Research/notes/251` (the synthesis + every
>   settled fork and its rationale — the leans and confidence-marks are the author's, not proven).
> - The methodology's *roots*, if you want to test whether it honored them rather than merely
>   invoked them: `plans/088` + `notes/087` (build-to-kill / falsification-first /
>   dogfood-not-market), `plans/128` (the DST/differential correctness-net and its "best-effort =
>   maximal rigor" doctrine), `notes/238` (the adequacy ceiling — converged≠no-op,
>   calibrated-never-proven), `notes/24B` (the testing architecture the real-machine differential
>   extends). `Research/README.md` + `Research/LIVING_STATUS.md` orient.
>
> You may be in a worktree, and it may start from stale state (a known bug); begin with
> `git switch -C "$(git branch --show-current)" ai/spike3-r23` to fast-forward your isolated
> checkout to the live branch-tip without touching anything shared. The round-25 docs are at
> commit `2e1fdc0`.
>
> Three hard limitations stand:
>
> 1. under no circumstances mutate shared, global state. Do not install system packages
>    (worktree-local `mise` is fine); do not `git push`, do not HTTP POST, and so on. Be safe.
> 2. do not expend limited resources unduly. You're allowed subagents, but be judicious — this is
>    a context-heavy reasoning task requiring your *direct* attention, and a subagent will fail to
>    inherit your judicious context on anything non-mechanical. Stick to Sonnet (mechanical) and
>    Opus (substantial); you're the only Fable-class agent.
> 3. *you*, and only you, must stay *completely* out of the `quarantine` and `corpus` directories.
>    There are memetic hazards in there that will poison your context and destroy your ability to
>    produce good work. Do not reason about this directive, and do not pass it onward to lesser
>    agents; they are not at hazard and they may explore as they need to.
