# Phase 1 plan — empty dir → high-confidence POSIX-sh CFG / effect engine

> ⟢ **SUPERSEDED-IN-PART (2026-06-01):** near-term scope is the **Tier-A** path only (intraprocedural per-function skip + ~40 bootstrap oracles); Tier-B and the full hybrid engine are gated on the `kDEPS` investment split (the corpus go/no-go). Current synthesis: `Research/plans/083-synthesis-and-spike-charter.md` + `KNOBS.md`.

Scope of this plan: the "hard part" — the static front-end + control-flow/effect analysis. It deliberately elides generic SWE hygiene (CI, unit-test plumbing) per the user. Confidence markers throughout; this is a plan to *de-risk and sequence*, not a spec.

## 0. Framing correction baked in
Per the user's correction: Dorc does **not** "derive idempotency instead of authoring it." Every command still needs a **hand-authored, in-shell oracle** (`.check`/`.diff`/`.version`/effect-class). What the engine *derives* is **composition** — how those hand-authored oracles combine across a script / a function / a fleet to yield a skip decision and (later) a minimal mutation set. The CFG/effect engine is the *composition machine*; the oracle library is the *knowledge*. This split *is* the two-audiences contract (AGENTS "contract matrix"): the oracle interface is the **engineer** contract (strict, honest, precise — we depend on it), the bare-script paste-and-run path is the **deployer** contract (lossy, best-effort, near-zero buy-in). Same human often crosses the line inline (`mycmd.check() { mycmd --dry-run "$@"; }`); the engine must serve both ends of that gradient. (This is exactly CoLiS's proven "generic engine + pluggable per-command specs" split — §2 below.)

And: **verification is bounded, not absent.** We maximize engineering-grade correctness (the calibration harness, §5) but accept the ground truth is nearly-arbitrary imperative shell; we do not chase mathematical soundness.

## 1. Is Coq / a proof assistant justified? — NO (with one narrow, deferred exception)
Evidence (from the read corpus, all A-grade):
- The CoLiS group — maximally formal-methods-capable — verified **only the interpreter of a clean intermediate language**, in **Why3 + automated SMT** (deliberately *not* Coq/Isabelle, judged too heavy), and even that needed a novel "ghost-skeleton" proof technique for one simplified-language interpreter. (VSTTE 2017.)
- They **could not** formally verify the two parts Dorc lives on: the **parser** ("POSIX spec is informal → impossible to prove correct; we don't even claim absence of bugs") and the **shell→IL translation** (no formal shell semantics). Both are trusted via review + **differential testing vs dash/bash**. (Morbig paper; TACAS 2020.)
- The proof effort bought **soundness of a symbolic over-approximation** — the exact goal Dorc rejected.

Verdict (SURE): No Coq/Isabelle; and not even CoLiS-style Why3+SMT as a *methodology*. Rationale: Dorc's domain is precisely the regime where CoLiS itself fell back to trust+testing; end-to-end soundness is unattainable regardless (the un-provable front-end gates everything), so proving the middle buys nothing the user wants. **Replace formal verification with the calibration harness (§5)** — differential testing + property tests + the user's own test-container oracle-fixture idea. 
- Narrow, *deferred* exception (GUESS): the algorithmic kernels where a bug is **silent and dangerous** — the probe-soundness invariant "⊥ (inert-classified) ⇒ provably no mutation" (plus elision's "skip ⇒ proven-converged") — could merit a small property-test suite or even a Why3 lemma. That's a v2 unit-confidence tool for one function, not a project methodology. Do not adopt up front.

## 2. Fundamental architecture
A pipeline of well-understood stages; each maps to referenceable prior art. (This is essentially the CoLiS front-end shape minus the symbolic back-end, plus our oracle/probe back-end.)

```
source (strict-superset sh)
  → [Lexer]      mode-sensitive, lossless          (Morbig prelexer / Oils lexer-modes)
  → [Parser]     → CST/IR (schema-defined, visitors) (Morbig incremental-LR / Oils recursive-descent+ASDL)
  → [CFG build]  over the IR, hazards modeled        (SPA §2.5; hazards below)
  → [Effect analysis]  forward may-analysis on a lattice  (SPA §4–5; transfer fn per command via its oracle)
        ⊤ = may-mutate / unknown ;  ⊥ = provably no mutation
  → [Skip decision]  per-host & per-function (Tier A); ship read-only probe, run on host
  → [(deferred Tier B) interprocedural facts + slicing → minimal mutation set]  (SPA §8–9, IFDS/IDE)
        ▲
        └── [Oracle interface]  pluggable per-command contracts; PROBE real host state (not symbolic)
Cross-cutting: [Calibration harness] differential + property + container fixtures (§5)
```

**Hazards the CFG/effect stage MUST model (else unsound in the dangerous direction)** — confirmed by Morbig/Oils/Smoosh and the planning log:
- **Redirections are mutation sites independent of commands** (`: > /etc/x`, here-docs write files).
- **`set -e` / `pipefail` / `trap` alter the CFG itself** (implicit exit/handler edges; can be set conditionally) — getting reachability wrong here is the subtlest unsoundness.
- **Whole-program CFG** across functions / `source` / aliases (not per-file).
- **Concurrency**: `&`, subshells, pipeline stages — subshell *env* mutations don't escape, *FS* mutations do.
- **The dynamic-construct boundary** (`eval`, dynamic command names, `. "$dyn"`, recursive `$((…))`, LValue-taking builtins) → `⊤`/`unsafe` by construction (enumerated in `notes/040`).
- **Quoting/word-splitting fragility** is pervasive (80% of real scripts have ≥1 smell) → unquoted expansion must be a first-class hazard, not an edge case.

**The effect lattice (the heart):** orientation is the planning-log's and it's a textbook **forward, may** analysis (SPA §5.8). ⊤ = may-mutate/unknown is the conservative default; skip only on ⊥. Each command's transfer function comes from its oracle's declared **effect class** (pure-query / mutating / unknown). Unknown command, eval, unrecognized guard → ⊤ (un-probeable at probe-time + can't-skip at apply). The cost asymmetry is **phase-dependent** (apply: false-skip dangerous, false-run cheap; probe: a mis-run mutation is the catastrophe) — encoded by the per-phase lattice orientation, not by a proof. Two soundnesses; see AGENTS §1.

**Engine/oracle separation (the load-bearing CoLiS lesson):** the analysis engine is *generic over command knowledge*; each command's check/effect is a plug-in. CoLiS proved this scales to ~28k scripts / hundreds of commands. Dorc keeps the split but swaps CoLiS's *symbolic FS-relation spec* for an *executable check-oracle shipped to and run on the host* (probe real state instead of symbolically modelling it).

## 3. How much can we crib? (per component, license-aware — see `notes/040`)
- **Parser:** techniques are free (non-copyrightable). Code reuse is license-gated: Morbig (the ideal fit) is **GPL-3** → either clean-room reimplement its incremental-LR + speculative-parsing recipe (~2–5k LoC; the SLE'18 paper is a near-complete spec) **or** bind **tree-sitter-bash** (MIT, C) if its tree suffices **or** fork **mvdan/sh** (BSD) iff Dorc is Go. (Decided in Phase 2.)
- **CFG / lattice / dataflow / fixpoint solver:** theory from the SPA textbook; *structural factoring* from **Goblint** (MIT): separate `domain` (lattice lib) / `solver` (fixpoint engines) / `framework` (analysis glue) / `analyses` (plug-ins) / `lifters` (sensitivity transformers). But Goblint is 84k LoC, C-coupled, and *sound* — Dorc needs a **far smaller** engine (≈one may-mutate analysis + slicing, not 84 sound domains). Crib the factoring, not the code.
- **Engine/oracle separation:** crib from CoLiS (concept, not GPL-3 code).
- **Dynamic-construct exclusion set + lossless-IR + schema→visitors:** adopt Morbig/Oils conventions.
- **Explicitly do NOT crib:** symbolic execution, feature-tree FS-relation logic, the formal-proof apparatus. These serve soundness we rejected.

## 4. Is the relevant work overview-level, or referenceable 2026 codebases? — BOTH; the engine is engineering, not research
SURE: for *every* engine component there is a working, current, referenceable codebase **and** canonical theory:
- Parsers: Morbig + morsmall (OCaml), mvdan/sh (Go), tree-sitter-bash (C), Oils (Py→C++) — all live.
- AI engine: Goblint (OCaml, actively developed — `vmcai25` tag).
- Semantics reference: Smoosh (OCaml).
- Theory: Møller-Schwartzbach SPA (updated 2025).
→ Phase 1 is an **engineering** problem with strong references, not open research. The genuinely novel/unreferenced work is **downstream** (the oracle-composition model + the version layer, MH1/MH2) — *not* the CFG/effect engine, which is well-trodden. This is the reassuring finding: we are not inventing static analysis; we are assembling a known engine and pointing it at a known-hard but *parseable* language (Morbig+Oils both prove shell is statically parseable; CoLiS proved the engine+oracle split scales).

## 5. The calibration harness (what replaces "soundness")
This is the user's "test-container toolkit," elevated to the project's correctness mechanism:
- **Differential testing**: analysis predictions vs. real shell (dash/bash) behavior and vs. actually-running-the-mutate on container fixtures. (CoLiS's parser-trust model.)
- **Property tests**: parse∘pretty-print round-trip = identity (the lossless invariant); `oracle.check` vs. ground-truth "did running the mutate change anything" on pinned fixtures.
- **Container fixtures**: pin `foo.check@version` against known filesystem states; catch oracle regressions on version bumps. (The user's idea; this is CI, not proof.)
- **Corpus differential**: re-parse + re-analyze a standing corpus on every change; track clean-parse% and skip-rate drift.

## 6. Build sequence (empty dir → engine)
- **Step −1 (gate the whole project): the MH1 de-risking spike.** Before writing any engine, run an existing parser (tree-sitter-bash MIT, or Morbig) + an `shstats`-style pass over the **user's own homelab corpus** and measure (a) clean-parse % under a strict core, (b) % of mutating leaves that are ⊤-bound by external/non-deterministic reads. Two corpora already say parse% will be ~99% and control flow is shallow/linear (CoLiS 28k; bash-in-the-wild 1.35M) — but the ⊤-bound rate on the *user's* scripts is the unmeasured risk. Decision gate. *(Needs `opam`/OCaml or tree-sitter toolchain — ASK before installing.)*
- **Step 0:** Phase-2 decisions (language; parser strategy).
- **Step 1: IR + parser.** Schema-defined lossless IR (visitors generated). Parser for the chosen strict-POSIX core (clean-room Morbig recipe, or tree-sitter binding). Acceptance: round-trip + differential-vs-dash on the spike corpus.
- **Step 2: CFG construction** over the IR with the full hazard set (§2) modeled. Acceptance: hand-built CFG fixtures incl. `set -e`/`trap`/redirection/subshell cases.
- **Step 3: effect lattice + oracle interface + per-host skip.** Forward may-analysis; ⊤/⊥; ship the **~40–50 bootstrap oracles** (the bash-in-the-wild priority list: `echo [ test cd export source read printf` + `rm mkdir cat cp mv ln chmod chown touch mkdir test ...`). Acceptance: "clean host → skip; any dirty leaf → no-skip" on container fixtures, zero false-skips.
- **Step 4: per-function skip** (intra-function leading-guard reasoning) — *the differentiator* (relieves Ansible's "re-run whole fleet for one role"). Intra-function syntactic fact-matching only; no canonical-predicate vocabulary yet.
- **Step 5 (defer): Tier-B** interprocedural facts + backward slicing (SPA §8–9 / IFDS) → sub-host minimal mutation set; the ~dozen canonical high-fanout facts. Build only if the 10% earns it.
- **Cross-cutting from Step 1:** the calibration harness (§5).

## 7. Phase-1-specific risks / open calls
- **The ⊤-bound rate on real homelab scripts** (Step −1) is the top unmeasured risk; everything downstream assumes a useful fraction is skippable.
- **Oracle-coverage bootstrap** (planning-log's existential risk): day-1 zero coverage = zero skip. Mitigation = ship the bootstrap set; the bash-in-the-wild ranking makes this finite and ordered.
- **Quoting/word-splitting** must be modeled as a hazard from Step 2 (it's everywhere), not bolted on.
- **`set -e`/`trap` reachability** is the subtlest correctness trap — prioritize fixtures here.
- **Leading-guard-only hoisting** assumption (planning log) — validate against the spike corpus, not asserted.
- **License**: if we link Morbig/ShellCheck we inherit GPL-3 — decide deliberately in Phase 2 (see plan 2).
