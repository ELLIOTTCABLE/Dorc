# Dorc research round — index

> ➤ **START HERE (2026-06-01):** the current cross-domain synthesis + the corpus-spike question-set is `plans/synthesis-and-spike-charter.md`; the design-tension vocabulary is `../KNOBS.md`. Read those first; the per-round notes/plans below are the chronological substrate they synthesize.

Autonomous research rounds grounding the Dorc design (a strict-superset-of-POSIX-sh + static effect-analyzer; "the derivable part is how hand-authored per-command oracles *compose*"). All stored references herein should be human-authored, graded, and reproduced locally; but all non-resource content is AI-generated, so take with a grain of salt - these are living notes from the research and planning process; not verified, authoritative, nor necessarily coherent.

## Read in this order
1. `notes/00-source-manifest.md` — every source, graded (quality/relevance) + the **license contamination map**.
2. `notes/10-parsing-shell.md` — how to statically parse POSIX shell (Morbig); the trust argument.
3. `notes/20-colis-architecture-and-coq-verdict.md` — the engine/oracle architecture that scales; **why Coq is not justified**.
4. `notes/30-corpus-evidence-and-positioning.md` — real-world corpus evidence (28k + 1.35M scripts); the bootstrap-oracle list; positioning vs ShellCheck.
5. `notes/40-parser-architectures-and-cribbability.md` — Morbig vs Oils vs mvdan; what we can legally crib.
6. `plans/phase-1-static-analysis-engine.md` — **the hard part**: empty dir → CFG/effect engine.
7. `plans/phase-2-language-workload-orchestration.md` — language / parser / orchestration (decisions to make).
8. `learning-path/README.md` — curriculum for the human (anchor: the SPA textbook).

### Analysis round (round 2 — soundness/reachability/mutation, the user's real concern)
9. `notes/50-analysis-prior-art-map.md` — the campaign map (reframe of "soundness" as over-approximation; bodies of work per Q1/Q2/Q3).
10. `notes/51-mutation-core-and-compositional-scaling.md` — MOD/purity domain + compositional summaries; **the soundness-bias inversion** (now refined into the two-soundness standard — *probe-soundness* vs *elision-soundness*; see AGENTS §1).
11. `notes/52-ifds-engine-and-datalog-bridge.md` — IFDS/IDE engine; side-effect & Datalog bridges.
12. `notes/53-reusable-structure-and-scale-mechanisms.md` — PDG/SDG vs Datalog vs value-flow; scale levers.
13. `notes/54-dynamic-language-soundness-tajs.md` — sound AI of a dynamic language (recency abstraction; eval→⊤).
14. **`plans/analysis-architecture.md`** — the synthesis: answers Q1 (sound+precise probe reduction), Q2 (reusable fact/dependence structure), Q3 (corpus-scale + incremental).

### Userbase & problem-space round (round 3 — corpus, orchestration go/no-go, user-study)
15. `notes/60-userbase-problemspace-map.md` — campaign map (3 spaces, grading).
16. `notes/61-ansible-userstudy-synthesis.md` — Ansible empirical user-study (Carreira 2025: 59k posts + 20 interviews); validates the Dorc thesis + lists capability gaps.
17. `notes/62-terraform-and-crosstool-userstudy.md` — Terraform + the ranked cross-tool synthesis (failures / costly choices / sacred cows).
18. **`plans/corpus-acquisition-plan.md`** — test-data corpus plan (bootstrap from academic corpora → homelab-gitops → fuzzy-edge sampling) + `tools/corpus-survey.sh` (dry-run-validated `gh` spike).
19. **`plans/orchestration-go-no-go.md`** — per-feature integrate-vs-delineate matrix.

### Performance round (round 4 — perf characteristics across the three phases)
20. `notes/70-performance-map.md` — campaign map; the **perf reframe** (shortcuts are elision-safe → tune toward speed in the safe direction); the probe-optimizer-is-behaviour-dropping framing; the cost-model complication.
21. `notes/71-analyzer-runtime-perf.md` — the complexity cliffs (cubic floor, IFDS O(ED³), **k-CFA EXPTIME** = the context-sensitivity redline) + the **memory wall** (recency as a memory lever).
22. `notes/72-probing-parallelism-perf.md` — compile-the-probe sidesteps Ansible's N·H round-trips; the three ceilings (controller async-not-fork, sshd `MaxStartups`/`MaxSessions`, thundering herd); the **resumability dividend**.
23. `notes/73-mutation-orchestration-perf.md` — derived-DAG scheduling (critical-path / RCPSP / Graham anomalies); the batching/blast-radius knobs; the **under-investment trap**.
24. `notes/74-cost-model-and-tuning.md` — DB-optimizer + PGO model for **cost-without-annotation** (default-conservative + profile-guided from realtime-output); the tuning knobs at the duality-intersections.
25. `notes/75-build-systems-as-prior-art.md` — **build systems = the skip-thesis in another domain**: the Scheduler×Rebuilder vocabulary; the **cross-host probe-memoization** perf lever; content-addressing seats the two modes; early cutoff; **hermeticity validates non-determinism-exclusion**.
26. **`plans/performance-architecture.md`** — the synthesis: perf design principles + the **"decide-now retrofit-hostile" list** + flagged decisions. Includes the build-systems lens, the cross-host-memoization lever, and the two boundary findings (probe-vs-just-run; when Dorc *loses*, and how memoization narrows it).

### Recovery round (round 5 — trace-don't-derive; after the perf-critique demoted derived-dependency soundness)
27. **`plans/pluggability-and-hook-surface.md`** — *live core constraint.* The wrappable leaf-execution seam (both phases), dual-use provenance, `--faithful` un-optimized mode, and the unprivileged **seccomp `socket(AF_INET)` backstop** that ships for all users (network cost-class + undeclared-network detection, fail-closed). What the core must not optimize away.
28. **`plans/deferred/privileged-tracing-tool.md`** — *deferred privileged devtool.* Why per-process tracing (LD_PRELOAD/ptrace) structurally fails on daemon-mediated ops tooling (docker→dockerd out-of-process), so the real value needs **eBPF/auditd system-wide** (privileged → quarantined separate binary); the easy/hard security split; containerized fixtures as the reproducibility complement. Output is always proposed `sh` guards, never metadata.

### Corpus go/no-go round (round 6 — instrument + first tally, then the adversarial pass)
29. `notes/80-corpus-spike-progress-and-first-tally.md` — *interim, not the final go/no-go.* The in-tree `tools/corpus` instrument built + validated (TS + tree-sitter; 10,350 files in 3.7s) and a first tally on a contrast-not-compound public sample. +SURE parse-feasibility is not the risk (0.0% hard parse-fail, eval 0.1%); ~SUSPECT idempotency is **module-native, not guard-native** (`creates:`/`removes:` ≈ 0.1%) → nudges the `kDEPS` split oracle-heavy (hold loosely — pre-bias). Load-bearing caveat: the raw tally was **~95% collection test-code**; scanner **v2** excludes test paths, splits mutating-vs-control, and **stratifies** (caught a live Simpson's paradox — guarded/mutating = role 45% / homelab 7%). The `Q-BAND` go/no-go stays open: it needs the §7 apply-cost×check-depth rules, parked for the user as "taste + ops-experience".
30. `notes/81-blind-multimodel-study-critique.md` — *methodology note, not a measurement.* **Don't run the thesis-blind, three-model variant** (scaffolded out-of-tree at `~/shell-iac-corpus-study`) **for go/no-go numbers — it regresses on the standard `[80]` already sets.** Blinding doesn't remove the classification-rule bias; it *relocates* it (→ prompt-framing, each model's training prior, the thesis-aware final compare) where it is opaque + uncorrectable, and "compare the three runs" is uninterpretable both ways (divergence confounds corpus≠measure≠classifier; convergence = shared priors, not truth). The worry is real (the apply-cost/check-depth rules *are* researcher-DoF) — but the fix is item 31, not blinding.
31. **`plans/corpus-classification-validation.md`** — *the recovery (supersedes the blind variant).* Return the `kDEPS` go/no-go without confirming-by-construction, on the instrument + sample you already have: **pre-register** the apply-cost/check-depth rules → **sensitivity-test** the verdict across conservative→liberal rule-sets (does taste decide it?) → **ground-truth** a stratified subsample on the planned calibration harness (container fixtures; a few dozen gold ops bias-correct the static band) → *optional* **models-as-raters of one fixed corpus** (measure the subjectivity via κ/α instead of hiding it) → **adversarial** worst-defensible rule-set (severe testing). Keep the instrument + SHA-pinned sample + contrast-not-compound; discard the blind protocol.

## Headline findings
- **Feasibility (MH1) is strongly supported, and the engine is engineering not research.** Shell *is* statically parseable (Morbig + Oils, two independent proofs); the engine+oracle split *scales to 28k scripts* (CoLiS); real provisioning shell is short/linear/eval-rare (Debian 28k: avg 15 lines, 99.9% parse; GitHub 1.35M: `if` 70%, `for`<40%, eval 9%). Referenceable current codebases exist for every engine component (Morbig, Goblint, Smoosh, mvdan, tree-sitter-bash) plus the canonical theory (SPA textbook).
- **The novel/unreferenced work is downstream** (oracle *composition* + the version layer), not the CFG engine.
- **No Coq/proof-assistant.** Even CoLiS fell back to differential-testing for parser + translation; they proved only a clean IL's interpreter (in Why3, not Coq), buying soundness we reject. Replace with the calibration harness (differential + property + container fixtures — the user's "test-container toolkit").
- **Positioning:** Dorc sits *above* ShellCheck (the de-facto linter, which by literature "does not check resource existence" — the #2 real-world bug class and exactly Dorc's altitude). The analysis-level niche is unoccupied.
- **License lever:** the best-fit OCaml parser (Morbig) is GPL-3 → clean-room its recipe, or hand-roll (recommended), which also *decouples the parser from the language choice*.

## Decisions awaiting the user (see plan 2 §F)
1. **Implementation language** (OCaml vs Rust vs OCaml-core+TS-harness) — low-regret due to the serialization seam, but sets the velocity/distribution/growth tradeoff.
2. **Drop the Ansible-transpile throwaway-v1?** (contradicts the planning log; I lean drop — bootstrap oracles are cheap and Ansible can't stream/frontload.)
3. Parser strategy (lean hand-rolled recursive-descent + lexer-modes); executor (lean thin built-in ssh-streamer).

## Vendor/ (full-history clones)
CoLiS ecosystem (morbig, morsmall, colis-language, colis-constraints, shstats, lintshell, …), shellcheck, mvdan-sh, smoosh, oils, goblint-analyzer, tree-sitter-bash. See manifest for grades/licenses.
