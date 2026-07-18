  ---
  Pair 1 — is our conception of the problem complete?

1. neutralised

> Attached is a working map of a design problem: how a best-effort, shell-script-based static analyzer should decide which
> commands invalidate ("poison") which items of global system-state, so it can safely skip already-converged work without a real
> mutation-analysis. It states hard constraints, goals, and several candidate approaches. Assess whether the problem-space as
> framed is complete: independently enumerate the framings and design-angles a solution could take, and identify any the
> document does not consider, or that its framing quietly forecloses.

1. adversarial

> A colleague mapped the attached design problem (best-effort "poison-scoping" for a shell-script analyzer) and is satisfied it
> covers the available approaches. I don't buy that — maps like this almost always bake in a framing that writes off whole
> regions of the solution-space before anyone notices the door closed. Roam freely and find the options, framings, and angles
> they never put on the table; show me where the setup itself is what blinds them. Where you judge the map genuinely complete on
> some axis, say so plainly rather than inventing a gap.

  ---
  Pair 2 — internal holes in the fourth design ("kind-scoped poisoning")

2. neutralised

> Attached is a design problem with several candidate approaches; the fourth and last ("kind-scoped poisoning") is the one under
> consideration. Assess that fourth approach strictly on its own terms: where it fails or silently mishandles inputs, where its
> soundness or usability claims do not hold, and what it costs that the document may underweight. Report without presuming the
> approach is right or wrong.

2. adversarial

> A colleague is quietly leaning toward the fourth/last design in the attached doc ("kind-scoped poisoning") and would like it
> to hold up. I distrust it — it reads as too convenient, the kind of thing that looks clean until a real case walks through it.
> Roam freely and tear into it: the internal holes, the inputs it silently mis-handles, the soundness or usability reasons not
> to pursue it. Assume it's flawed and your job is to prove where — but if a line of attack genuinely doesn't land, say so
> instead of forcing it.

  ---
  Shared reading-list — 23x-excluded (append to all four, ahead of the 233 doc)

> Before the attached 233 problem-doc, ground yourself in Dorc's goals and
> constraints. Dig as deeply and freely as you need to to serve the above purpose;
> you needn't necessarily respect the constraints the team placed upon AGENTS when
> reading (but you are still not to mutate shared resources, and so on.)
>
> - README.md, DESIGN.md, IMPLEMENTATION.md (especially "Correctness vs. best-effort: a band") — what Dorc is, and the
> best-effort / "soundiness" thesis that defines what a good answer even means here.
> - KNOBS.md, ANALYZER-NEEDS.md — the design-tension and engine-needs vocabulary; reuse these slugs, don't coin parallel ones.
> - spike/CLAUDE.md — the welded invariants and standing rulings; in particular rul-mutation-impossible (mutation is
> fundamentally unanalyzable; probe-safety is structural self-vouch only) and the phase-keyed kFAIL (never wrongly elide).
> - Research/plans/055 + Research/plans/099 — the two phase-keyed soundnesses and the MUST-vs-MAY decidable floor (the soundness
> frame the problem sits on).
> - Research/README.md — as a map only, if you need to navigate.
>
> Do NOT read Research/notes/231, notes/232, notes/23Z, or any other 23x-series note — they carry the team's in-progress conclusions
> and leanings, and would prime you toward answers we want you to reach independently. Read only 233 and pre-round-23 resources.
>
> You are working in parallel with other agents, either avoid mutation
> (compilation included), or, if necessary, create yourself a new worktree (ensure
> it splits off of the codebase in .claude/worktrees/spike3-r23) in which to do
> mutative/experimental work. (I suspect this won't be necessary.)
