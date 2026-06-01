# Source manifest — Dorc research round

Grading: **Quality** = provenance (peer-reviewed/author-hosted human work = A; practitioner docs = B; anything AI-generated = rejected, not saved). **Relevance** to MH1 (static effect-analysis of shell) and/or Phase-2 (language/parser/orchestration). Recency deliberately *not* a penalty (research artifacts age but stay relevant; OCaml ages especially gracefully). All A-grade human sources are reproduced locally under `Research/` or cloned under `Vendor/`.

## Papers (`Research/papers/`, each with `.txt` extraction)

| File | Source / authors | Q | Relevance |
|---|---|---|---|
| `moller-schwartzbach-static-program-analysis` | Møller & Schwartzbach, Aarhus (free textbook, continuously updated) | A | **Anchor.** CFG, lattices, dataflow, fixpoints, interprocedural, abstract interpretation. The vocabulary + architecture of the whole field. |
| `morbig-sle2018` | Régis-Gianas, Jeannerod, Treinen — SLE 2018 (HAL) | A | **Keystone.** How to *statically* parse POSIX shell despite its eval-time grammar. Direct prior art, OCaml. |
| `jeannerod-phd-thesis-2021` | N. Jeannerod PhD (HAL tel-03917971) | A | **Master reference.** Full pipeline: parse → AST → feature-tree constraints → decision procedures → corpus analysis. 250pp OCaml. |
| `colis-installation-scenarios-tacas2020` | Becker, Jeannerod, Marché, Régis-Gianas, Sighireanu, Treinen — TACAS 2020 | A | Symbolic execution of 27k Debian maintainer scripts. **Feasibility evidence for MH1.** |
| `colis-platform-sttt2022` | same group — STTT journal 2022 (HAL hal-03737886) | A | Platform/engineering view of the toolchain end-to-end. The "STTT paper" the planning log cites. |
| `colis-specification-of-unix-utilities-2019` | Jeannerod, Régis-Gianas, Marché, Sighireanu, Treinen — Inria TR (hal-02321691) | A | **The ontology seed.** Enumerated FS-effect specs for coreutils — the Tier-B canonical-fact vocabulary, pre-built. |
| `verified-interpreter-shell-vstte2017` | Jeannerod, Marché, Treinen — VSTTE 2017 (hal-01534747) | A | Shell-like interpreter formally verified in **Why3** (not Coq). Direct input to "is a proof assistant justified?" |
| `smoosh-popl2020` | Greenberg & Blatt — POPL 2020 (author-hosted) | A | Executable formal POSIX semantics. Catalogues the *hazards* (set -e, expansion, fields) our analyzer must respect. |
| `biabduction-popl2009` | Calcagno, Distefano, O'Hearn, Yang — POPL 2009 (UCL, author-hosted) | A | Compositional per-procedure pre/post inference (Infer's basis). Maps to our requires/establishes dependency model. |
| `bash-in-the-wild-tosem2022` | Y. Dong et al. — TOSEM 2022 (author-hosted) | A | Large-scale empirical study of real bash: usage, smells, bugs. **Grounds the corpus-skip-rate spike.** |
| `dozer-icse-seip-2022` | Horton & Parnin, NCSU — ICSE-SEIP 2022 (arXiv 2203.12065) | A | Syscall-altitude insight: command effects observable at the kernel boundary. 2-page short paper (text complete; 121 lines is correct). **Code IS public**: github.com/config-migration/dozer (Python, dynamic strace — reference only, not cloned). |

## Learning path (`Research/learning-path/`)

| File | Source | Q | Note |
|---|---|---|---|
| `cmu-17355-dataflow-frameworks` | Aldrich, CMU 17-355 lecture notes | A | Dataflow framework over WHILE3ADDR — clean pedagogy. |
| `harvard-cs153-cfg-dataflow` | Harvard CS153 Lec17 | A | CFG construction + dataflow, lecture form. |

To-curate (links only so far; grade before saving): Clang "Data flow analysis: an informal introduction" (LLVM docs, B+); Ed Yang "Hoopl" blog series (A, functional dataflow lib); Wisconsin CS704 Horwitz notes (A). **Rejected: Grokipedia (AI-generated).**

## Vendor repos (`Vendor/`, full history)

| Repo | Lang | Q | Relevance |
|---|---|---|---|
| `colis-anr/morbig` | OCaml | A | The static POSIX parser. menhir-based. Cribbable front-end candidate. License: GPL3 (⚠ contamination if reused). |
| `colis-anr/morsmall` | OCaml | A | Concise POSIX-sh AST atop Morbig. Pushed 2025 — most-alive CoLiS piece. |
| `colis-anr/colis-language` | OCaml | A | The symbolic analyser/engine. (We rejected symbolic-exec, but parse→IR pipeline is instructive.) |
| `colis-anr/colis-constraints` | OCaml | A | Feature-tree constraint backend. |
| `colis-anr/shstats` | OCaml | A | **Statistical analyzer for shell-script corpora — directly reusable for the skip-rate spike.** |
| `colis-anr/lintshell` | OCaml | A | User-extensible POSIX-sh lint — conceptually near our oracle model. |
| `shellcheck` | Haskell | A | Most mature *real-world* shell static analyzer. Pragmatic parser + checks over messy bash. |
| `mvdan-sh` | Go | A | Production hand-rolled shell parser/AST/formatter. Best "hand-roll a robust parser" reference. License: **BSD-3** (permissive). |
| `smoosh` | OCaml/Lem | A | Reference implementation of the executable POSIX semantics. |
| `oils` (OSH/YSH) | Python/C++ | B→ | Andy Chu's statically-parseable shell. Per user: *investigate*, not assumed-useful. |
| `goblint-analyzer` | OCaml | A | Modular abstract-interpretation framework for C. Best OCaml AI-architecture reference. |
| `tree-sitter-bash` | C/JS grammar | B | Incremental GLR grammar for bash — parser-generator-path reference. |

Infer codebase deliberately **not** cloned (huge; the bi-abduction *concept* is captured by the POPL09 paper; Goblint covers the OCaml-AI-architecture need better for us).

## License contamination map (Phase-2-critical)
- **GPL-3** (linking forces Dorc → GPL-3): morbig, morsmall, colis-language/constraints, shstats, **ShellCheck**.
- **Permissive**: Goblint (MIT), Smoosh (MIT), tree-sitter-bash (MIT), Oils (Apache-2.0), mvdan/sh (BSD-3).
- Consequence: the best-fit OCaml shell parser (Morbig) is GPL-3; reuse-as-code contaminates. Techniques are free. See `notes/40`.

## Synthesis notes & plans produced this round
- `notes/10-parsing-shell.md` — Morbig recipe, the trust argument, ShellCheck contrast, Dozer.
- `notes/20-colis-architecture-and-coq-verdict.md` — CoLiS engine/oracle split, corpus evidence, "no Coq" verdict.
- `notes/30-corpus-evidence-and-positioning.md` — 1.35M-script feature data, bootstrap-oracle ranking, vs-ShellCheck positioning.
- `notes/40-parser-architectures-and-cribbability.md` — Morbig vs Oils vs mvdan, license-aware cribbability, Oils verdict.
- `learning-path/README.md` — graded curriculum (anchor: Møller-Schwartzbach SPA) for the human.
- `plans/phase-1-static-analysis-engine.md` — the engine plan (Coq verdict, architecture, crib map, build sequence).
- `plans/phase-2-language-workload-orchestration.md` — language/parser/orchestration decisions (several `[USER DECISION]`).

## Analysis round (round 2) — soundness/reachability/mutation prior art
Papers (`Research/papers/`, all grade A, free author/HAL/arXiv/DROPS copies):
- `reps-horwitz-sagiv-ifds-popl1995` — IFDS: precise interprocedural dataflow as realizable-path reachability + tabulation summaries. **Engine candidate.**
- `reps-cfl-reachability-survey-1998` — CFL-reachability unifying framework (dataflow/slicing/points-to all = L-reachability).
- `salcianu-rinard-purity-vmcai2005` — purity/side-effect (MOD) analysis; prestate-mutation; compositional per-method summaries. **Mutation-domain template.**
- `lucassen-gifford-effect-systems-popl1988` — type-and-effect systems (the oracle effect-class lineage).
- `cousot-abstract-interpretation-popl1977` — AI foundation (⚠ image-only scan; theory via SPA textbook).
- `distefano-scaling-static-analyses-facebook-cacm2019` — Infer/Zoncolan: compositionality = scale; diff-time deployment (0%→70%). **Scale + deployment template.**
- `horwitz-reps-binkley-sdg-slicing-toplas1990` — PDG/SDG + interprocedural slicing = reachability. **Reusable-structure template (Q2).** (`tip-program-slicing-survey-1995` also saved but ⚠ image-only.)
- `scholz-souffle-datalog-cc2016` — Datalog→native compiled analysis at scale. **Queryable-fact-base (Q2/Q3).**
- `bravenboer-smaragdakis-doop-oopsla2009` — declarative points-to in Datalog (Doop).
- `avgustinov-ql-codeql-ecoop2016` — QL/CodeQL: OO queries over a relational program-fact DB.
- `sui-xue-svf-cc2016` — sparse value-flow analysis; Graph/Rules/Solver factoring; mem-region partitioning. **Sparsity + factoring (Q3).**
- `arzt-bodden-reviser-incremental-ifds-icse2014` — incremental IFDS/IDE via clear-and-propagate. **Incremental (Q3).**
- `szabo-inca-*` (oopsla2018, pldi2021) — incremental Datalog with lattices (IncA/DRedL).
- `abadi-mcsherry-foundations-differential-dataflow-esop2015` — incremental iterative dataflow at scale.
- `heintze-tardieu-demand-pointer-analysis-pldi2001`, `reps-demand-interprocedural-cc1994` — demand-driven analysis (Q1B/Q3).
- `jensen-moller-tajs-type-analysis-javascript-sas2009` — **sound AI of a messy dynamic language** (our predicament); recency abstraction; eval→⊤. **Most direct methodological precedent.**

Frameworks cloned (`Vendor/`, full history): `flow` (OCaml, MIT-ish), `infer` (OCaml, MIT — compositional/bi-abduction), `TAJS` (Java — sound AI of JS), `WALA` (Java — IFDS/slicing framework), `souffle` (C++, UPL — Datalog→native), `doop` (Datalog), `SVF` (C++ — sparse value-flow), `salsa` (Rust — incremental query engine), `codeql` (QL lib/docs — queryable fact base).

Analysis notes: `50-analysis-prior-art-map`, `51-mutation-core-and-compositional-scaling`, `52-ifds-engine-and-datalog-bridge`, `53-reusable-structure-and-scale-mechanisms`, `54-dynamic-language-soundness-tajs`. Synthesis: **`plans/analysis-architecture.md`** (answers Q1/Q2/Q3).

## Userbase & problem-space round (round 3)
Papers: `ansible-challenges-mixed-methods-study-2025` (Carreira et al., arXiv 2504.08678v2 — **grade A**, the empirical user-study anchor: 59,157 posts + 20 interviews).
Key external sources (links + quotes captured in notes; not all saved as files):
- Academic IaC corpora (both test-data AND validation criteria): **Opdebeeck PhD 2024** (15k+ Ansible scripts + a Program Dependence Graph for Ansible; `soft.vub.ac.be`/`ropdeb.ee`); **GLITCH** (polyglot IaC, annotated oracle datasets; arXiv 2205.14371/2308.09458); **Rahman "Gang of Eight"** defect taxonomy incl. idempotency (arXiv 2505.01568, `chrisparnin.me/pdf/GangOfEight.pdf`); Begoug IaC-SO; **InfraFix** (arXiv 2503.17220).
- Ops-corpus sources: homelab-gitops community (`k8s-at-home`/`homelab`/`home-ops` GitHub topics; `onedr0p/cluster-template` lineage).
- Provider-corpus: Ansible Galaxy API (top collections by download); Chef Supermarket / Puppet Forge / TF Registry.
- Sentiment: HashiCorp's own "provisioners are a last resort" docs (authoritative on the fuzzy edges); HN Terraform threads; practitioner blogs (graded B, opinion). **Rejected/down-graded**: the pile of SEO/AI "X vs Y" listicles (spacelift/attuneops/kestra/guru99/oneuptime/etc.).
Notes: `60-userbase-problemspace-map`, `61-ansible-userstudy-synthesis`, `62-terraform-and-crosstool-userstudy`. Deliverables: **`plans/corpus-acquisition-plan.md`** (+ `tools/corpus-survey.sh`), **`plans/orchestration-go-no-go.md`**.

## Performance round (round 4) — perf characteristics across the three phases
Mostly *mining the existing on-disk corpus for cost* + targeted external search for ops/distsys prior art (the phase-2/3 gap). New external sources, graded:
- **Heintze–McAllester "On the Cubic Bottleneck in Subtyping and Flow Analysis"** (LICS 1997) — A. The cubic floor for CFL-reachability/points-to (2NPDA-complete). **Paywalled (IEEE); every open mirror is dead (Cornell/Kozen 404).** Result captured in note 71 + the open companion below.
- **Sridharan–Fink "The Complexity of Andersen's Analysis in Practice"** (SAS 2009) — A, open. The cubic-bottleneck companion: Andersen points-to is cubic worst-case but near-linear in practice on real programs (the "cubic floor but small-in-practice" point Dorc relies on). **On disk: `papers/sridharan-andersen-complexity-practice-sas2009.{pdf,txt}`** (1042 lines; manu.sridharan.net).
- **Van Horn–Mairson "Deciding k-CFA is complete for EXPTIME"** (ICFP 2008) — A. Context-sensitivity is provably exponential; the redline for Dorc's context dial. **On disk: `papers/vanhorn-mairson-kcfa-exptime-icfp2008.{pdf,txt}`** (495 lines; brandeis.edu).
- **Mitogen for Ansible docs** (`mitogen.networkgenomics.com/ansible_detailed.html`) — A (primary, author). Orchestration-overhead-dominates evidence; 1.25–7× numbers; `local_action` serialization footgun; connection delegation. *Saved: `_scratch/mitogen-ansible.rst`.*
- **pyinfra performance docs** (`docs.pyinfra.com`) — A (primary). "Network is the bottleneck"; validates the two-phase (gather→execute) model. 
- **OpenSSH multiplexing cookbook + sshd MaxStartups/MaxSessions** (OpenSSH wikibooks; Microsoft/OpenSSH docs) — A. The connection ceilings.
- **Salt architecture/syndic docs** (`docs.saltproject.io`) — B. Fan-out-tree horizontal scaling; agent-pull contrast.
- **Terraform graph internals** (`stategraph.com/blog/terraform-dag-internals`, `terraform-parallelism`; HashiCorp `/internals/graph`) — B+/A. DAG semaphore walk; "concurrency = graph width, not the setting."
- **Ansible strategies/serial/throttle/max_fail_percentage docs** (`docs.ansible.com`) — A. The batching/blast-radius knobs.
- **Kubernetes Deployment rollout docs** (`kubernetes.io`) — A. maxSurge/maxUnavailable/readiness-gate.
- **Scheduling theory** (List scheduling / Critical-path method / RCPSP — Graham's bound + anomalies) — A (underlying theorems) via B (Wikipedia summaries). 
- **Twitter "Murder" (BitTorrent for deploys)** (`github.com/lg/murder`) — B. Artifact-distribution-is-the-bottleneck-at-scale; O(N)→O(log N) P2P.
- **DB query-optimization** (Adaptive Cost Model, arXiv 2409.17136; Oracle optimizer docs; query-optimization survey) — A/B. Cost model + cardinality-estimation-dominates + adaptive re-planning (the cost-model template).
- **PGO / AutoFDO** (Google Research, CGO 2016) — A. Sampling-based profile-guided optimization with no annotation — the model for harvesting probe-cost from realtime-output.
- **Reviser** (Arzt–Bodden, on disk) — A. Re-mined for the **80% incremental-savings** number.
- **Soufflé** (on disk) — A. Re-mined for the **128 GB-RAM / billions-of-tuples** memory-wall quantification.
- **k-CFA paradox** (Might–Smaragdakis–Van Horn, PLDI 2010) — A. The crucial refinement: k-CFA is EXPTIME for *closures*, polynomial for *flat/OO data* — so Dorc's flat fact-map may afford context-sensitivity. **On disk: `papers/might-smaragdakis-vanhorn-kcfa-paradox-pldi2010.{pdf,txt}`** (1365 lines; yanniss.github.io).
- **strace/ptrace vs eBPF overhead** (SysTutorials + practitioner benchmarks, B) — ptrace 2–10× (up to 102×) vs eBPF <1–2%; the effect-observation altitude for calibration/derivation.
- **rust-analyzer durable incrementality** (rust-analyzer blog + issue #4712, A/B) — the incremental poster child *punts* on-disk cache persistence (too subtle); tempers Dorc's "versioned summary format" optimism with a cold-start fallback.
- **SaltStack syndic GitHub issues** (#55564/#43003/#19864, B, maintainer-primary) — the fan-out-tree result-aggregation lock-in ("does not scale", won't-fix).
- **Terraform state-splitting retrospectives** (Medium/Terragrunt, B) — 20–30 min monolith plans; split trades slow-plans for cross-state-dependency complexity (the granularity tension Dorc transposes onto the analysis unit).
- **Puppet/Chef splay + thundering herd** (O'Reilly, Puppet docs/forums, B) — `splay`/`fqdn_rand` jitter; server 503+Retry-After backpressure; the "flocking" re-convergence problem.
- **distsys log-aggregation / backpressure** (Better Stack, design-guru, B) — the fan-in dual of fan-out; backpressure + bounded-buffer + hierarchical aggregation for the realtime-output scale cost.

## Recovery round (round 5) — trace-don't-derive; reintroducing lost values
After the perf-critique demoted derived-dependency soundness (the `docker compose up foo.yaml`→reads-`/mnt/blah` catch), this round grounds the **third option** for deps: observe them at runtime (trace), not declare (specify-the-world) or statically-derive (unreliable). Synthesis split into **`plans/pluggability-and-hook-surface.md`** (live core constraints: wrappable leaf-execution seam, dual-use provenance, `--faithful` mode, the unprivileged seccomp network-class backstop) + **`plans/deferred/privileged-tracing-tool.md`** (the deferred privileged eBPF/auditd tool; why per-process tracing fails on daemon-mediated tooling; easy/hard security split). *(Earlier `recovery-design.md` was the first cut — reframed by the user as devtool-not-core and removed.)* Key sub-findings: M2 trace-detects-hidden-deps = honest reliability flagging; M4 lifted-guards+oracle-library grows the value band; dep-free UX survivors (plan-as-shell, streaming, modes) live in `performance-architecture.md`; the rattle hazard mechanism settles hoist-safety.
- **Spall–Mitchell–Tobin-Hochstadt "Build Scripts with Perfect Dependencies"** (OOPSLA 2020, rattle) — **A, the load-bearing recovery find.** Forward build system: straight-line script, NO declared deps, *trace* file accesses → "perfect dependencies, no way to get them wrong"; speculation + hazards. The mechanism that recovers "invisibility" (priority #4). **On disk: `papers/spall-mitchell-rattle-perfect-dependencies-2020.{pdf,txt}`** (728 L; ndmitchell.com).
- **Vendor clones (trace-based dependency inference, full history):** `Vendor/rattle` (Haskell — forward build, speculation, fsatrace-based), `Vendor/tup` (C — FUSE-traced deps + undeclared-access *detection*, the M2 model), `Vendor/fabricate` (Python — strace/access-time traced, memoize.py lineage), `Vendor/fsatrace` (C — the LD_PRELOAD/file-access tracer rattle uses; the pragmatic janky-box-friendly trace mechanism, §4 default). Grades A/B (working primary implementations).
- **tup / fabricate / memoize.py** (gittup.org, brushtechnology, McCloskey; A/B docs) — three independent implementations confirming trace-based dep-inference is old, robust, cross-ecosystem. tup's "error on undeclared access" = M2's hidden-dep detection.
- **Build Systems à la Carte** (Mokhov–Mitchell–Peyton Jones, ICFP 2018, MSR) — **A, the round's highest-value late find.** Scheduler×Rebuilder factoring; verifying-vs-constructive traces; minimality/early-cutoff. The skip-thesis unifying theory. **On disk: `papers/mokhov-build-systems-a-la-carte-icfp2018.{pdf,txt}`** (943 lines; microsoft.com).
- **Bazel remote caching / remote-execution-API / "build without the bytes"** (bazel.build, Aspect/Tweag blogs) — A/B. Content-addressed action cache (CAS); hermeticity requirement; cross-machine reuse = the cross-host-memoization model.
- **Nix content-addressed derivations + early cutoff** (NixOS wiki, Tweag, nix.dev) — A/B. Input-addressed vs content-addressed = the two-modes seating; determinism precondition.
- **Buck2 / DICE / Watchman** (buck2.build, facebook/buck2) — A/B. Incremental-computation-engine convergence (≈ Salsa/Adapton); event-driven change-detection.
**These five build-systems sources → `notes/75-build-systems-as-prior-art.md`** (the skip-thesis prior art + the cross-host-memoization lever).
- **Prometheus** (pull-model scrape/federation/sharding + **staleness** semantics; prometheus.io docs A, Robust Perception A, Grafana/Last9 B) — the probe phase in another domain: pull-knows-inventory (`up`/unreachable=unknown), staleness bounds memoization, hierarchical-federation = fan-out-tree (third domain confirming it).
- **osquery / Fleet** (distributed SQL fleet-state collection; osquery.readthedocs.io A, fleetdm/kolide B) — **target-side watchdog** (don't degrade the monitored host), **differential mode** (report only changed rows), schedule-staggering. → folded into `notes/72`.
- **REJECTED**: the `oneuptime.com/blog/post/2026-*` cluster (AI content-farm: templated titles, future-dated, no author); assorted SEO "X vs Y" listicles; a dead `devseccon.com` Puppet-retrospective URL (301→snyk.io marketing, content gone — not cited).

Notes: `70-performance-map` (campaign map + the perf reframe + the contract-matrix-driven cost complication), `71-analyzer-runtime-perf` (the complexity cliffs + memory wall), `72-probing-parallelism-perf` (fan-out ceilings + thundering herd + resumability), `73-mutation-orchestration-perf` (DAG scheduling + batching + the under-investment trap), `74-cost-model-and-tuning` (DB-optimizer + PGO model for cost-without-annotation). Synthesis: **`plans/performance-architecture.md`** (the perf design principles + the "decide-now retrofit-hostile" list + flagged decisions).
