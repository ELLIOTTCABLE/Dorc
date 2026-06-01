# AGENTS.md — Dorc

- README.md, DESIGN.md, and TODO.md are human-written;
  - consider re-reading them first if they are not in-context (important context in those is *not* duplicated into this AGENTS.md, intentionally);
  - do not edit them, under any circumstances - suggest edits to the user if you see clear incorrectness; and
  - trust them over the ocean of unreviewed, LLM-generated planning-slop in the Research/ folder.

- try to use reference-slugs in documentation and conversation:
  - source-ID-with-grading (as per the interactive-research skill)
  - when generating 'lists' during conversation (a list of questions, a list of results, a list of nits ...), try and give them vaguely-unique slug-IDs (`nit-1. nit-2. nit-3.`) instead of bare Markdown lists (`1. 2. 3.`), to make it easier to refer-back (and help me see what *you're* referring-back to.)
  - similarly, reuse the named 'knobs' when referring to the shared-axis/"pair-in-tension" design-space components we're working with (see `KNOBS.md`)

# THE REST OF THIS DOCUMENT IS LLM-GENERATED; limited trust until I can review and trim it.

> Project charter. Global preferences — the confidence-marker convention, communication style, the
> maintainability › simplicity › validation › performance priority order, 3-space indent,
> comments-explain-why, research-in-main-context, and never-mutate-external-state-without-leave —
> live in the System `CLAUDE.md` and are **not** repeated here. This file is only what's specific to
> Dorc. The full research record and rationale live in `Research/` (start at `Research/README.md`);
> read it before researching, and don't re-derive what's already synthesized there.

## What Dorc is
A strict-superset-of-POSIX-sh **language** + a **static effect-analyzer**. You write ordinary
imperative shell; Dorc statically derives a read-only "check" projection of it — a cost-minimized
probe that collates independent state-queries, runs them across the fleet in parallel, and retains
only as much of the original control flow as earns its keep (an optimizer decision, tuned by
perf-testing — see *Genuinely open*). It ships that probe, reads back each host's *partial*
state-map, and applies only the minimal mutations each host actually needs. One line: **`terraform
plan`/`apply` for imperative shell — idempotency derived by analysis, not authored by hand.** Target:
the unkillable "Linux box someone SSH'd into" and "deploy a janky app from a non-ops dev (no
check-mode, no clean container, no clean API)" — **not** the clean world Nix/containers already own.

**Corrected framing — load-bearing.** The derivation is *bounded*: every command still needs a
human-authored, in-shell oracle (check / effect-class / version). What Dorc *derives* is how those
hand-authored oracles **compose** across a script, a function, and a fleet. The engine is the
composition machine; the oracle library is the knowledge. Never describe Dorc as "eliminating
hand-authoring."

## The contract matrix — two audiences · two modes · two soundnesses
Three orthogonal dualities generate most of Dorc's design tensions. State which one you're in before
arguing a tradeoff; conflating them is what repeatedly went wrong.

**Audiences** (often the *same human, same file, gradually* — design the gradient, not the poles). The
deployer contract is deliberately *lossy* — that **is** the product; a deployer forced to over-specify
just uses Nix/Terraform. We afford that looseness only by holding the engineer to a *stricter* contract:
their oracle honesty is the currency we spend to keep the deployer's buy-in near zero.

| | **Deployer** (ops/admin) | **Engineer** (oracle / role author) |
|---|---|---|
| Contract | lossy, inference-heavy, best-effort | strict, honest, precise |
| Buy-in | ~zero — paste shell, it runs | higher — authors checks / effect-classes / cost hints |
| We owe them | safety under under-specification; speed; no ceremony | leverage: their precision serves every downstream deployer |
| If we're too strict | they leave (the over-specification tax) | — |
| If we're too lax | — | deployer silently inherits unsound / expensive behaviour |
| Mindset | "explore this" · "does Dorc fit?" · "**fire, NOW**" | "fix the flaky role" · "make it team-consumable" · "publish a module" |

Touchstone: an inline `mycmd.check() { mycmd --dry-run "$@"; }` is a deployer becoming an engineer for
one line. Everything must permit that — minimal buy-in, no cliff between the modes.

**Modes** (the elision user-need; *same* soundness, *different* scope — a mode changes what's *in*
elision scope, never *whether* elision is sound).

| | `dorc update` (goal-framed) | `dorc reconcile` (state-framed) |
|---|---|---|
| Intent | correctness for what I changed; speed elsewhere | reconcile all state; elide only probe-proven |
| Elision scope | user-declared (e.g. the git-diff) + probe | probe only |
| Trade | completeness → speed (accepts staleness outside scope) | speed → completeness |
| Drift outside scope | not checked — the user's explicit choice | fully checked |
| Serves | the dev/debug/explore **hot-loop** (the Ansible pain we target) | the final full production push |

**Soundnesses** — Principle 1: *probe-soundness* (read-only pass never mutates; fail-safe = withhold)
and *elision-soundness* (never skip a needed mutation; fail-safe = perform). Phase-keyed, opposite fail
directions. The modes ride on top of elision-soundness; the audiences set how much inference we owe.

## Phase & priorities
Build order, in sequence: **analysis engine → language/parser → orchestration.** **Current focus:
the analysis engine** (update this line as phases complete). Stay focused on the current phase; the
north star is *better imperative / non-declarative orchestration*, so don't make analysis or language
choices that foreclose it. The two must-haves are the thesis —
everything else is replaceable:
- **MH1** — deep CFG analysis → a skip-safe parallel check-program. 90% of value = skipping
  hosts/functions with no work, zero mutation risk. Per-*function* skip is the differentiator (it
  relieves Ansible's most-hated trait: re-running the whole fleet to change one role).
- **MH2** — versioning as an invisible layer over existing version managers (from `npm install
  foo@3.5.5`, derive the version-correct `foo.check`).

## Settled principles — do not relitigate (the corrections that took repeating)
1. **Two soundnesses — never conflated; always say which.** *Probe-soundness*: the read-only pass
   must never perform a prestate mutation — uncertain effect ⇒ treat as may-mutate (oracle-check it,
   or defer it to apply); never run a raw, possibly-mutating command in a promised-read-only pass.
   Fail-safe action = *withhold execution*; idempotence does **not** rescue this. *Elision-soundness*:
   the apply pass must never skip a still-needed mutation — uncertain necessity ⇒ **run it**. Fail-safe
   action = *perform execution*; over-running is safe (the user owes idempotence). A wrong skip is
   outage-grade. They look opposite only because in probing *executing* is the hazard and in applying
   *eliding* is; both are sound over-approximations of their *own* hazard. **Verification is bounded,
   not a non-goal**: maximize engineering-grade correctness, but anything unmodeled (`eval`, dynamic
   command names, non-determinism, a command with no oracle) collapses to the conservative end of
   *both* — un-probeable (defer to apply) **and** can't-skip (must-run). Soundness is **relative to the
   oracle contract**: we own our *derivations* (if we conclude a leaf is inert, it must be); a leaf's
   *declared* effect-class is demanded of the contributor, assumed in design, and best-effort-checked —
   a lying oracle is GIGO, outside the bound, not something to over-defend against. Shared kernel (a bug
   here breaks *both*): `inert-classified ⇒ provably no mutation`; elision additionally needs *skip only
   on positive proof of convergence*. "TypeScript, not Coq."
2. **Parsing is a non-concern** — a boring, pluggable, owned recipe. Do not over-invest effort or
   research here; the hard, novel work is downstream (effect-analysis + composition).
3. **Non-determinism is excluded, not modeled.** Random/time-dependent state is not a truth worth
   establishing; oracles attain accuracy by *canonicalizing* (strip timestamps, mask volatile fields).
4. **Keep check and mutate SEPARATE.** Frontloading, plan-time skip, and producer-agnostic deps all
   rest on this; reject anything that fuses them.
5. **Gradual enhancement is the point.** Pasting naive upstream-doc shell must Just Run as v1;
   idempotency / checks / effects / deps layer on incrementally. Day-1 floor: no worse than a shell runner.
6. **Target imperfect-state machines, not convergence to immutable truth.** Declarative /
   promise-theory / NixOS is the wrong model and we must not drift toward it.

## Engineering guardrails (Dorc-specific)
- **The analysis architecture is decided at the shape level** (`Research/plans/analysis-architecture.md`):
  a compositional, over-approximating may-mutate (MOD) abstract interpretation; per-command
  effect-class supplied by oracles; IFDS/IDE-style summaries; recency strong/weak abstraction for
  precision; a dependence/value-flow graph exposed as a queryable fact base; per-role summaries +
  sparsity + demand + incremental for scale. It is **Infer's architecture running an
  over-approximating MOD analysis, not an under-approximating bug-finder.** It is *engineering, not
  research* — referenceable current codebases exist for every component.
- **Genuinely open — flag, don't silently resolve:** engine substrate (hand-rolled IFDS/IDE vs
  Datalog/Soufflé vs hybrid; needs a spike); implementation language (OCaml vs Rust vs OCaml-core +
  TS-harness — *low-regret*, downstream of the substrate choice; OCaml "ages well" but is **not** a
  selection criterion); recency granularity; context-sensitivity dial; closed-vs-open predicate
  vocabulary; **probe CFG-retention policy** (a *cost-based optimizer* choice per check/subtree —
  *hoist-and-batch* cheap, independent, read-only checks into the parallel flat probe, vs
  *keep-under-guard* expensive checks so a cheap local guard can elide them, e.g. don't `curl` a
  remote service when `[ -f /etc/flag ]` is false. Over-probing is correctness-*safe* but
  economically central — it's the cost the tool exists to cut — so this is **not** a "flatten
  everything" punt; driven by a cost model + perf-testing, not asserted now); **guard purity**
  (effectful `if`-guards like `if mkdir …; then` — classify-and-defer vs require declared guard-purity).
- **No Coq/Isabelle/Why3 as methodology.** Calibration replaces proof: differential testing (analysis
  prediction vs actually running the mutate on container fixtures) + property tests of **both** kernels —
  `inert-classified ⇒ provably no mutation` (the *probe-soundness* kernel: pure-marked leaves really are
  read-only) and *skip fires only on positive proof of convergence* (the *elision-soundness* kernel).
  Even the CoLiS group fell back to differential-testing for the un-provable parts.
- **License-awareness is real.** The best-fit OCaml prior art (Morbig, the CoLiS stack, ShellCheck) is
  **GPL-3** — reusing it as code contaminates Dorc. Permissive: Goblint/Smoosh/tree-sitter-bash (MIT),
  Oils (Apache-2.0), mvdan/sh (BSD). Crib techniques freely; don't *link* GPL-3 without a deliberate
  decision.
- **The orchestration boundary is decided** (`Research/plans/orchestration-go-no-go.md`). INTEGRATE:
  {engine + sh-superset language + planner + oracle library + realtime/provenance output}. CEDE:
  secrets, privilege-escalation, **persistent state-backend (deliberately avoided — it is Terraform's
  single biggest liability)**, declarative resource graphs, control-plane/GUI/scheduler, templating
  engine. SEAM: a thin, swappable ssh-streaming executor reading existing inventory. Keep the
  Ansible-loved sacred cows — agentless-ssh, idempotence, low barrier-to-entry, heterogeneity.
- **Realtime streamed output + precise errors are a hard requirement**, not a nicety — debugging
  opacity (the frozen buffer, the escape-to-a-shell-script-to-debug) is the most-cited real-world pain.
  Corollary: the probe is an **optimization artifact** (an opaque, reshaped minimization of the user's
  script) — users get **no** control over its shape, but every check and result must stay
  **provenance-attributable** to the source line it came from. Opaque-to-control, transparent-to-explanation.

## Research guardrails (Dorc-specific)
- The prior art is **already mapped and synthesized** across three rounds (shell/parsing,
  analysis/soundness, userbase/problem-space) under `Research/`. Read it before researching; don't
  re-derive settled findings.
- **Grade every source.** This domain is thick with SEO/AI "X vs Y" listicle slop — weight
  peer-reviewed / primary / practitioner sources, down-grade marketing and AI-generated content, and
  reproduce genuinely-good human sources locally (`Research/papers`, `Research/notes`).
- **Don't bulk-download corpora or hammer APIs without explicit leave; ask per-tool before installing
  anything.** Full git clones are welcome (history aids the "why") while the total footprint stays
  under ~50 GB. `gh` is authenticated; `tools/corpus-survey.sh` is dry-run by default.
- **Test-data bootstraps from curated academic corpora** (Opdebeeck's 15k-script Ansible PDG, GLITCH's
  annotated oracles, Rahman's labeled defects incl. idempotency) *before* any GitHub scraping; the
  homelab-GitOps community is the realistic ops corpus and the actual target demographic.

## Rejected — do not propose
Rollback / `.undo` / transactionality as a core feature; symbolic execution or proven-soundness as a
*goal*; convergence / promise-theory / CFEngine; Nix or any own-the-world model; a central
state-backend; an authored declarative resource graph; a control-plane / GUI / scheduler (be the thing
CI calls, not the platform); a templating engine (defer to `envsubst`/`jq`/`m4`, provide only the
convergence-predicate + plan-time diff around them); consumer-side guard-lifting as the *primary*
oracle mechanism; an Ansible-transpile throwaway-v1 (bootstrap oracles are cheap, and Ansible can
neither stream output nor frontload).
