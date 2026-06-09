# Prompt 1

You're going to orchestrate an implementation spike, exploring a problem-space in my research-process. You'll take notes as you go; you're round 19, i.e. `notes/19[2-9A-Z]-descriptive-slug.md`.

Again, your primary goal is *exploration*. Make choices that don't work and notate. Push frontiers and notate. You're to figure out *what's hard* about a particular approach, and why it does or doesn't work well in practice.

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

For build-state, also read Research/plans/16P-spike-postmortem.md (its §3 built-vs-designed ledger first) and 16Q-next-spike-and-process.md.

Then tell me: what spike-2 is building (the keystone), what's already there versus the "poison-wall" gap, the locked ch-* fork-rulings from the charter, and the three leading-goal seams — and roughly what you expect to orchestrate subagents to do. Brief, confidence-marked. Flag anything in the charter that doesn't sit right with you; pushing back here is useful.


# Prompt 2
The previous spike's code is already forked, builds green, and is waiting at <root>/spike/. Confirm it for yourself: `mise exec -- cargo test --workspace`. Skim the analyzer crates (spike/crates/{core,analysis,oracle,plan}) enough to see what exists.

Before building, ground yourself in the theory the engine rests on. Read these chapters of Static Program Analysis (Møller & Schwartzbach) IN FULL into your context — a text-read of the PDF is on disk at Research/sources/B-moller-schwartzbach-static-program-analysis-2025.txt:

4 Lattice Theory · 5 Dataflow Analysis with Monotone Frameworks · 6 Widening · 8 Interprocedural Analysis · 9 Distributive Analysis Frameworks (IFDS/IDE) · 11 Pointer Analysis · 12 Abstract Interpretation

Chapters 4 and 5 are the engine you're extending. Chapter 9 is the "overpowered alternative" the charter deliberately does NOT take — read it so you can recognize when the hand-rolled worklist strains toward it (the charter's re-eval-trigger). 6 bears on termination under a recursive domain (seam-finite); 11 on the strong/weak-update + uniqueness mechanism behind the keystone. (3, 7, 10 are skippable for stated reasons — see the charter.)

Next, load up on the design the spike realizes. Read Research/plans/17N-named-kinds-discipline-and-cooperation.md — the named-kind discipline (how a kind is spelled, analyzed, and reconciled across oracles) — and the root ANALYZER-NEEDS.md, focusing on its §C (entity-algebra) and §M (engine substrate) need-clusters.

Go deeper into Research/ only as a specific question pulls you there — it's deep but noisy, and AGENTS.md's reading guide governs (syntheses over notes; later turns over earlier; trust the root human docs over all of it). Don't read it prospectively. notes/180 + notes/190 hold the substrate prior-art if you want to check the worklist decision; plans/055 is the reference engine design.

You now have the context to orchestrate. Build the keystone per plans/191, dispatching subagents per-component (presumably the spike crates, although make your own decisions about how to structure subagent effort).

Each subagent gets: the SAFETY block at the very top; they'll automatically recieve the CLAUDE.md in the crate-root you invoke them into (they may need to be manually fed the spike-root spike/CLAUDE.md); and the specific charter / invariant slugs it must honor.

Give them the goal you wish to impart, and the invariants; but leave them room — they're Opus-Max-reasoning, and over-constraining wastes them.

Hold this framing (restated from above) the whole way: this is state-space exploration. The product is the record of what strained and where (log it under Research/notes/19*, continuing the numbering from previous iterations), not a green checkmark. Pursue correctness in order to find where pursuing it is hard. Spending real effort attacking a wall to map it is valuable; so is abandoning a direction when you hit a wall and going to explore a different part of the space — just take notes either way, and don't grind in a rabbit-hole.

Two things the charter makes non-negotiable, because the last spike got them wrong:
- Build the keystone (the analysis-engine/entity-algebra re-key) before more type-machinery (ap-1). The re-key invalidates anything built on the old flat fact key.
- The acceptance harness must execute or `sh -n`-check the rendered artifact, never string-diff it (ap-2).

(Repeat: Make sure the shell-commands are non-functional stubs like `hork` and `wombat` if you're going to be executing it! Do not run system-mutative commands in the text-fixtures!)

Apply /adversarial-crosscheck to your own work judiciously, at real junctures — a landed keystone, a substrate-seam result, "did we build the keystone or scaffold around it?" — and rotate its target (ap-3): aim passes at the harness and the three seams, not only core soundness. When you invoke it, pass your adversarial agents, in addition to the skill's own method: the SAFETY block; clean context, un-seeded from your own notes, convergence-across-passes as the trust signal, verify-by-tracing-the-code not by relaying a claim; don't over-constrain them; and carve out the unproductive dead-ends — market-fit, value-prop, corpus-matching, and anything else hard-to-measure and out of scope now.
