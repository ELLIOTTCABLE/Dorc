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

# Prompt 2
The round-19 keystone is already built and committed in a worktree at <root>/.claude/worktrees/round19/ (branch ai/round19-keystone; the Rust spike is under its spike/). Work there, not <root>/spike/ (that copy is pre-keystone). Confirm green from inside that spike/: mise exec -- cargo test --workspace, and sh e2e/run.sh for the corpus.
Before building, ground yourself in the theory the engine rests on. Read these chapters of Static Program Analysis (Møller & Schwartzbach) IN FULL into your context — a text-read of the PDF is on disk at Research/sources/B-moller-schwartzbach-static-program-analysis-2025.txt:
4 Lattice Theory · 5 Dataflow Analysis with Monotone Frameworks · 6 Widening · 8 Interprocedural Analysis · 9 Distributive Analysis Frameworks (IFDS/IDE) · 11 Pointer Analysis · 12 Abstract Interpretation
Chapters 4 and 5 are the engine you're extending. Chapter 9 is the "overpowered alternative" the charter deliberately does NOT take — read it so you can recognize when the hand-rolled worklist strains toward it (the charter's re-eval-trigger). 6 bears on termination under a recursive domain (seam-finite); 11 on the strong/weak-update + uniqueness mechanism behind the keystone. (3, 7, 10 are skippable for stated reasons — see the charter.)
Next, load up on the design the spike realizes. Read Research/plans/17N-named-kinds-discipline-and-cooperation.md — the named-kind discipline (how a kind is spelled, analyzed, and reconciled across oracles) — and the root ANALYZER-NEEDS.md (note, a fallible index, agent-compiled, only lightly human-reviewed), focusing on its §C (entity-algebra) and §M (engine substrate) need-clusters.
Go deeper into Research/ only as a specific question pulls you there — it's deep but noisy, and AGENTS.md's reading guide governs (syntheses over notes; later turns over earlier; trust the root human docs over all of it). Don't read it prospectively. notes/180 + notes/190 hold the substrate prior-art if you want to check the worklist decision; plans/055 is the reference engine design.
You now have the context to orchestrate. The keystone is built (notes/193); per notes/196 the work now is two-fold: (1) enrich the test-corpus to map the strain-frontier across the analyzer's full surface — mostly-failing elision-tests, exhaustive on the built surface, one-xfail-per-cluster on the unbuilt (notes/196 §3 has the coverage design); and (2) the probe-projection / guard-subsumption (Half B) — the now-promoted core work the keystone exposed as missing (notes/196 §2; the compiled-probe.straw.sh model): lift the book's own read-only guards into the probe instead of mis-eliding them (the F1 wrong-elision). Dispatch subagents per that design.
Each subagent gets: the SAFETY block at the very top; they'll automatically recieve the CLAUDE.md in the crate-root you invoke them into (they may need to be manually fed the spike-root spike/CLAUDE.md); and the specific charter / invariant slugs it must honor.
Give them the goal you wish to impart, and the invariants; but leave them room — they're Opus-Max-reasoning, and over-constraining wastes them.
Hold this framing (restated from above) the whole way: this is state-space exploration. The product is the record of what strained and where (log it under Research/notes/19*, continuing the numbering from previous iterations), not a green checkmark. Pursue correctness in order to find where pursuing it is hard. Spending real effort attacking a wall to map it is valuable; so is abandoning a direction when you hit a wall and going to explore a different part of the space — just take notes either way, and don't grind in a rabbit-hole.
(Repeat: Make sure the shell-commands are non-functional stubs like `hork` and `wombat` if you're going to be executing it! Do not run system-mutative commands in the text-fixtures!)
Apply /adversarial-crosscheck to your own work judiciously, at real junctures — a landed keystone, a substrate-seam result, "did we build the keystone or scaffold around it?" — and rotate its target (ap-3): aim passes at the harness and the three seams, not only core soundness. When you invoke it, pass your adversarial agents, in addition to the skill's own method: the SAFETY block; clean context, un-seeded from your own notes, convergence-across-passes as the trust signal, verify-by-tracing-the-code not by relaying a claim; don't over-constrain them; and carve out the unproductive dead-ends — market-fit, value-prop, corpus-matching, and anything else hard-to-measure and out of scope now.

# Re-seed update (round-19 → next iteration; added by the round-19 orchestrator — read notes/19F)

**This supersedes Prompt 2's "two-fold work" (corpus + Half-B) as the priority.** Round-19 built the keystone, the apply-fold, and a 43-case corpus, and surfaced a deeper, foundational failure: the codebase carries **three incoherent representations of a command's "observable"** — `cfg::Observable` (consumption-only channels), `core::Verdict{Converged}` (fact-cell state), and build-1's `Observed{verdict, rc}` (a bolted rc). That disconnect caused a live `kFAIL-perform` under-execute (`useradd … || mkdir` dropped) and kept re-confusing the design. **Read `Research/notes/19F` FIRST — it is this iteration's seed** (then `19E`→`193` for context; `196` was the *prior* round's seed). `spike/CLAUDE.md` `inv-one-observable` states the target.

**The job (rip-and-replace, NOT a half-fix — human-directed):** clean-rewrite the elision/observable layer so the codebase has exactly **ONE** coherent `Observable` — the command's output-tuple over channels `{Effect, Status, Stdout, Stderr}` that the oracle `.check()` *predicts* (a per-channel value, or a loud out-of-band ⊤ "can't-predict"), an enclosing context *consumes* (liveness — which channels it reads), and a substitution *reproduces*; "convergence" = the *derived* `Effect`-channel state (no-mutation, refined by the ambient gate), never a separate verdict and never the check's concern. The corpus/Half-B work is downstream of, and was defeated by, this disconnect — do it *after*.

- **Keep / wipe (19F §4):** KEEP — notes 193–19F (the deliverable; **preserve through any branch action**), the substrate (`syntax`/`lattice`/`solve`/`cfg`-construction/`core`/`hostsim`/`cli`/`e2e`), the 43-case corpus, and the keystone's **cell model** (`FactKey{kind,entity,selector}` + ambient gate — its `Verdict` *folds into* the `Effect` channel). RIP-AND-REPLACE — `cfg::Observable`, `Verdict`, `Observed`, `plan::fold`, `prove_replaceable`'s gate, the rc machinery, the `cli`/`hostsim` rc-sourcing. **Do NOT wipe to `main`** (it predates the keystone, corpus, and these notes). Fork from this branch, or cherry-pick the keep-set + notes.
- **DISTRUST in the current impl (19F §5):** the `Verdict`/`Observed` split; the `cli`/`hostsim` rc-*default* history (it fabricated `rc=0`); the **masking tests** (`andor-rc-vouch-wrong`'s injected `rc=9`, the promoted `…substitutes_exact_rc`, `fold-oror`'s `rc=0`) — they pass by *declaring* an rc the model should *predict*.
- **Acceptance = the anti-masking suite (19F §6):** right ONLY if observable values flow from the *check* end-to-end, with **no test hand-injecting an rc the check should predict** — the `useradd[rc9] || mkdir` redline, the conforming `&&`, the `command -v || install` idiom, the `set -e` case, and the OOB-⇒-run meta-test, all passing honestly.
- **Process keepers:** crosscheck with the **full neutral+adversarial pair** (the comparison is the signal — a single pass + a seeded self-review missed the under-execute until the adversarial pass drove it through the CLI); isolated worktrees here check out a STALE base and can't `git commit` (build agents run in the worktree; read-only crosscheck agents isolate fine); `SendMessage` is unavailable to course-correct a running agent.

# RESPONSES DO NOT COMMIT

[CORRECTED round-19 — the original framing below was wrong; see notes/19B + spike/CLAUDE.md inv-one-observable] Half-B / probe-projection: the probe is the UNION of (1) ordering+args from the book and (2) probe-bodies from oracles' declared `.check()`s — both oracle-anchored; nothing un-oracled is magically lifted. An oracled guard `command -v nginx` ships as `command__check -v nginx` (the book's FULL args) + the body of `command.check()` — NOT the book's raw guard, and the *oracle* (not Dorc) argparses. Per 19F, Half-B is downstream of the one-Observable rewrite.
~~"Half B (UNBUILT) = the probe projection — compile the probe from the book's own CFG, lift the book's own checks/guards (command -v, dpkg -s) as read-only interceptors,"~~
