# Prompt 1
You're in a worktree, during an in-flight implementation-spike. You're going to *orchestrate*, exploring a problem-space in my research-process, using subagents to keep your context-window clear and long-lasting for autonomous operation.

You'll take notes as you go; you're round 19, i.e. `notes/19[7-9A-Z]-descriptive-slug.md`. This is your *primary purpose*: rich logging of pain-points and the ways various aspects of the design described below will bite us, when we start *real* implementation.

(Again, your primary goal is *exploration*. Make choices that don't work and notate. Push frontiers and notate. You're to figure out *what's hard* about a particular approach, and why it does or doesn't work well in practice.)

As the orchestrator, your attention and context-window are *critical*. The following section applies to you, but should *also* be passed verbatim at the top of every subagent prompt (the subagents are capable models with large context-windows as well; they can handle reading deep context):

(Don't yet go digging further into the plans/ and notes/ and other resources; I will direct your onboarding directly.)

----
First, read @README.md, @DESIGN.md, @IMPLEMENTATION.md, and @KNOBS.md. These are human-written and are the final authority on design questions, with a couple key exceptions.

**An important note on safety:** this applies for the *entire process**, at all times; and must be passed onwards at the start
of every subagent prompt, as well:
- No git mutation outside your worktree (and never, ever push)
- Local commits are fine; try and work granularly (commit often.)
- Do not spend external resources (or exhaust rate-limits)
- Do not mutate global state (installing system packages, system config, etc - worktree-local `mise` installs and config are
fine)
- Make sure the shell-commands in your *runnable* test-fixtures are non-functional stubs like `hork` and `wombat` if you're
going to be executing it! Do not run system-mutative commands in the text-fixtures! Keep a clean separation between executable
(fake) fixtures; and real (dangerous) strawmen that must not be *executed* during development/testing.
- Copy this block, verbatim, into *all* subagent prompts.

Before continuing, synthesize for me what you've read, focusing on the priority order; the two phase-keyed soundnesses (kFAIL) and the probe/apply split; the two users (the admin who writes "books" vs the engineer who writes "oracles"); and the "everything is spelled in sh" principle (and its off-ramp).
----

Second, read @Research/notes/17x-strawmen/adversarial/compiled-probe.straw.sh. This supercedes previous design-direction, where appropriate, for this round - it's *one* approach to some of the KNOBS that we're spiking to surface implementation-problems behind.

Complete this section yourself, as the top-level agent, as well (you'll also synthesize the information below.)

Now the task itself.

Read the round-19 charter in full: @Research/plans/191-spike2-keystone-charter.md — it's the spike's on-ramp and names the locked decisions. Pay special attention to the 'tensions' that you must adjudicate; that's one of *your* primary goals, as the steering-agent; those cross-cutting concerns are likely to bite subagents.

Next, the handoff/re-seed note: **Research/notes/19F** — read it first (the round-19 failure-analysis + this iteration's priority; see also the Re-seed update at the end of Prompt 2), then work backwards through 19E…193 for context (`196` was the *prior* round's seed).

Finally, also read Research/plans/16P-spike-postmortem.md (its §3 built-vs-designed ledger first) and 16Q-next-spike-and-process.md.

Then tell me: what spike-2 is building (the keystone), what's already there versus the "poison-wall" gap, the locked ch-* fork-rulings from the charter, and the three leading-goal seams — and roughly what you expect to orchestrate subagents to do. Brief, confidence-marked. Flag anything in the charter that doesn't sit right with you; pushing back here is useful.

