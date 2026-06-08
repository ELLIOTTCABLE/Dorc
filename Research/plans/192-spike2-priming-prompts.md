# 191 — Round-19: priming-prompt series for the spike-2 orchestrator

This is a launch sequence for the top-level orchestrator agent (the "TL-agent") that will drive spike-2.
Feed the prompts in order to a fresh agent; the early ones load context and end in a synthesis checkpoint
(have it write back what it learned) before the next — the read→synthesize cadence is what keeps a long
autonomous run anchored to Dorc's actual goals. The ordering is deliberate: core tenets, then the task, then
the static-analysis theory, then the design corpus, then the work.

The SAFETY block below prefixes the first prompt and is marked for propagation; everything else is plain
prompts to copy or adapt. (Written to leave the agents room — they're all Opus-Max — rather than over-direct
them.)

---

## SAFETY (put at the very top of the first prompt, and tell the agent to carry it to the top of every subagent prompt)

```
SAFETY — read first, obey throughout, and reproduce verbatim at the top of every prompt you give a subagent:
- No git mutation outside this worktree. Local commits on the worktree branch are fine; never push, never
  touch git state in any other repo/worktree. Use read-only git elsewhere.
- Do not exhaust rate-limits on external resources. Space out network fetches; respect robots.txt.
- No global system-state mutation (no system package installs, no system config). Worktree-local mise
  installs and config — including `mise trust` — are fine.
- Perpetuate this SAFETY block, verbatim, to the very top of every subagent prompt you write.
```

---

## Prompt 1 — core tenets (comprehension check)

```
[SAFETY block here]

You're going to orchestrate an implementation spike for a project called Dorc. Before anything else, build
your own understanding from the human-authored ground truth. Read these at the worktree root: README.md,
DESIGN.md, IMPLEMENTATION.md, KNOBS.md, AGENTS.md. Don't open Research/ plans or notes yet.

Then tell me, in your own words and reasonably briefly, what Dorc is and its core tenets — at least: the
priority order; the two phase-keyed soundnesses (kFAIL) and the probe/apply split; the two users (the admin
who writes "books" vs the engineer who writes "oracles"); and the "everything is spelled in sh" principle
(and its off-ramp). Mark your confidence on claims you're unsure of (+SURE / ~SUSPECT / -GUESS / --WONDER).
This synthesis is the checkpoint — we don't move on until it reads solid to you.
```

## Prompt 2 — the task overall (build-state + charter)

```
Now the task itself. Read the round-19 charter in full: Research/plans/190-spike2-keystone-charter.md — it's
the spike's on-ramp and names the locked decisions. For build-state, also read
Research/plans/16P-spike-postmortem.md (its §3 built-vs-designed ledger first) and 16Q-next-spike-and-process.md.

The previous spike's code is already forked, builds green, and is waiting at <root>/spike/. Confirm it for
yourself: `mise trust mise.toml; mise trust spike/mise.toml` once (the fork relocated the path-keyed trust),
then from spike/ run `mise exec -- cargo test --workspace`. Skim the analyzer crates
(spike/crates/{core,analysis,oracle,plan}) enough to see what exists.

Then tell me: what spike-2 is building (the keystone), what's already there versus the "poison-wall" gap, the
locked ch-* fork-rulings from the charter, and the three leading-goal seams — and roughly what you expect to
orchestrate subagents to do. Brief, confidence-marked. Flag anything in the charter that doesn't sit right
with you; pushing back here is useful.
```

## Prompt 3 — the static-analysis grounding (read it whole)

```
Before building, ground yourself in the theory the engine rests on. Read these chapters of Static Program
Analysis (Møller & Schwartzbach) IN FULL into your context — the PDF is on disk at
Research/sources/B-moller-schwartzbach-static-program-analysis-2025.pdf (or https://cs.au.dk/~amoeller/spa/spa.pdf):

  4 Lattice Theory · 5 Dataflow Analysis with Monotone Frameworks · 6 Widening ·
  8 Interprocedural Analysis · 9 Distributive Analysis Frameworks (IFDS/IDE) ·
  11 Pointer Analysis · 12 Abstract Interpretation

Chapters 4 and 5 are the engine you're extending. Chapter 9 is the "overpowered alternative" the charter
deliberately does NOT take — read it so you can recognize when the hand-rolled worklist strains toward it
(the charter's re-eval-trigger). 6 bears on termination under a recursive domain (seam-finite); 11 on the
strong/weak-update + uniqueness mechanism behind the keystone. (3, 7, 10 are skippable for stated reasons —
see the charter.)

Tell me the load-bearing ideas from each chapter that bear on extending a monotone-worklist analyzer with a
recursive, kind-typed fact-lattice and strong/weak update. This is the last load step before the design corpus.
```

## Prompt 4 — the design corpus (judicious, need-led)

```
Now the design the spike realizes. Read Research/plans/17N-named-kinds-discipline-and-cooperation.md — the
named-kind discipline (how a kind is spelled, analyzed, and reconciled across oracles) — and the root
ANALYZER-NEEDS.md, focusing on its §C (entity-algebra) and §M (engine substrate) need-clusters.

Go deeper into Research/ only as a specific question pulls you there — it's deep but noisy, and AGENTS.md's
reading guide governs (syntheses over notes; later turns over earlier; trust the root human docs over all of
it). Don't read it prospectively. notes/180 + notes/190 hold the substrate prior-art if you want to check the
worklist decision; plans/055 is the reference engine design.

Then synthesize, briefly: the entity-algebra shape you'll build (recursive, fields typed by the kind
namespace itself — kinds embed kinds), and how re-keying the fact domain to it kills the poison wall.
```

## Prompt 5 — the work, and how to run it

```
You now have the context to orchestrate. Build the keystone per plans/190, dispatching subagents
per-component (the spike crates). Each subagent gets: the SAFETY block at the very top; the per-component
CLAUDE.md in spike/crates/<crate>/ (and the spike-root spike/CLAUDE.md); and the specific charter / invariant
slugs it must honor. Give them the goal and the invariants, then leave them room — they're Opus-Max, and
over-constraining wastes them.

Hold the frame the whole way: this is state-space exploration. The product is the record of what strained and
where (write it to Research/notes/19*), not a green checkmark. Pursue correctness in order to find where
pursuing it is hard. Spending real effort attacking a wall to map it is valuable; so is abandoning a
direction when you hit a wall and going to explore a different part of the space — just take notes either
way, and don't grind in a rabbit-hole.

Two things the charter makes non-negotiable, because the last spike got them wrong:
- Build the keystone (the entity-algebra re-key) before more type-machinery (ap-1). The re-key invalidates
  anything built on the old flat fact key.
- The acceptance harness must execute or `sh -n`-check the rendered artifact, never string-diff it (ap-2).

You own the cross-cutting tension adjudications (the `tc-*` set in plans/190 §5b — collapsing May/Must,
minting an elision-voucher, proof-vs-tainted, strong/weak-update, which-phase, which-user,
holds-under-a-lying-oracle). A per-crate subagent flags these up to you; you discharge them with the
phase/user/orientation context it lacks, running the exclusion-check (reverse-direction · other-phase ·
other-user · other-reliability) before excluding any case. Type-encode them restrictively where you can; keep
them honest by hand where the type can't.

Apply /adversarial-crosscheck to your own work judiciously, at real junctures — a landed keystone, a
substrate-seam result, "did we build the keystone or scaffold around it?" — and rotate its target (ap-3):
aim passes at the harness and the three seams, not only core soundness. When you invoke it, pass your
adversarial agents, in addition to the skill's own method: the SAFETY block; clean context, un-seeded from
your own notes, convergence-across-passes as the trust signal, verify-by-tracing-the-code not by relaying a
claim; don't over-constrain them; and carve out the unproductive dead-ends — market-fit, value-prop,
corpus-matching, and anything else hard-to-measure and out of scope now.
```

---

## Orchestration notes (for whoever launches the TL-agent — not part of the prompts)

- The per-component division maps to the crates: `core` (vocabulary — the fact-key re-key starts here),
  `syntax` (the disposable test-front-end parser), `analysis` (cfg + the worklist + effect/classify — the
  keystone's home), `oracle` (the lift + the kTYANNOT-inline experiment + the stdlib-oracle quality bar),
  `plan` (elision/replacement + the backward/bump skeleton + the executable render), `hostsim`/`cli` (DST +
  the round-trip + the executable acceptance harness). Each gets a CLAUDE.md (see the spike-root CLAUDE.md
  and `spike/crates/<crate>/CLAUDE.md`).
- ap-1 implies a sequencing the TL-agent should respect: `core`'s fact-key re-key lands before the crates
  that depend on it (`analysis`, `plan`) build much on top — otherwise that work is thrown away.
- The charter's three seams (seam-prov, seam-interproc, seam-finite) are the highest-value targets for both
  the build and the adversarial passes; they're where the deliberately-underpowered substrate is most likely
  to strain, which is the whole point.
