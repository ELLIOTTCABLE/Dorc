# Source manifest

Grading: **Quality** = provenance (peer-reviewed/author-hosted human work = A; practitioner docs = B; anything AI-generated = rejected, not saved). **Relevance** to MH1 (static effect-analysis of shell) and/or Phase-2 (language/parser/orchestration). Recency deliberately *not* a penalty (research artifacts age but stay relevant; OCaml ages especially gracefully). All A-grade human sources are reproduced locally under `Research/` or cloned under `Vendor/`.

## Sources → `../sources.json`

The graded **document-sources** — the original ~40 papers + learning-path notes, grown by each round's web/practitioner sources to **~175 keys (rounds 1–14)** as of the round-16 spike — live in the machine-queryable [`../sources.json`](../sources.json): each `<grade>-<slug>-<year>` key maps to its `url`, local `file`, `sha256`, and graded relevance. To avoid a second source of truth this manifest no longer tables them — it keeps only what `sources.json` does *not*: the **Vendor** clones, the **license lever**, and the external / uncloned sources recorded per round below. **No new Vendor clones after round 5** — rounds 6–16 added document-sources (and in-tree instruments / byte-pinned specimens) only.

To-curate (links only so far; grade before saving): Clang "Data flow analysis: an informal introduction" (LLVM docs, B+); Ed Yang "Hoopl" blog series (A, functional dataflow lib); Wisconsin CS704 Horwitz notes (A). **Rejected: Grokipedia (AI-generated).**

## Vendor repos → `../vendor.json`

The 28 Vendor clones — local path, GitHub repo, **pinned commit**, language, grade, license, and Dorc-relevance — now live machine-queryable in [`../vendor.json`](../vendor.json), mirroring the `sources.json` delegation above so there is a single source of truth. Rebuild the checkouts from it with [`../clone-vendor.sh`](../clone-vendor.sh) (gh-auth'd, commit-pinned, self-healing, exponential-backoff, per-repo partial-clone for the heavy ones); regenerate the old table on demand with `../clone-vendor.sh --table`. (`infer` *is* among the 28, despite a round-1 "not cloned" aside that the round-2 clone superseded.)

## License contamination map (Phase-2-critical)
Per-repo `license`/`licenseNote` live in `vendor.json`; the lever:
- **GPL / copyleft** (reusing their *code* forces Dorc → GPL): the CoLiS set (morbig, morsmall, colis-language/constraints, lintshell, shstats), **ShellCheck** (GPL-3), **tup** (GPL-2); **WALA** is EPL-2 (weaker, file-level).
- **Permissive**: MIT (Goblint, Smoosh, codeql, flow, infer, tree-sitter-bash), Apache-2 (TAJS, salsa, Oils), UPL (souffle), BSD-3 (mvdan/sh), ISC (fsatrace).
- **Verify before reuse** (GitHub returned no SPDX id): SVF, doop, rattle, fabricate.
- Consequence: the best-fit OCaml shell parser (Morbig) is GPL-3; reuse-as-code contaminates. Techniques are free. See `notes/040`.

## Analysis round (round 2) — soundness/reachability/mutation prior art
Papers (now under `Research/sources/`, graded in `sources.json`; all grade A, free author/HAL/arXiv/DROPS copies):
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

Analysis notes: `050-analysis-prior-art-map`, `051-mutation-core-and-compositional-scaling`, `052-ifds-engine-and-datalog-bridge`, `053-reusable-structure-and-scale-mechanisms`, `054-dynamic-language-soundness-tajs`. Synthesis: **`plans/055-analysis-architecture.md`** (answers Q1/Q2/Q3).

## Userbase & problem-space round (round 3)
Papers: `ansible-challenges-mixed-methods-study-2025` (Carreira et al., arXiv 2504.08678v2 — **grade A**, the empirical user-study anchor: 59,157 posts + 20 interviews).
Key external sources (links + quotes captured in notes; not all saved as files):
- Academic IaC corpora (both test-data AND validation criteria): **Opdebeeck PhD 2024** (15k+ Ansible scripts + a Program Dependence Graph for Ansible; `soft.vub.ac.be`/`ropdeb.ee`); **GLITCH** (polyglot IaC, annotated oracle datasets; arXiv 2205.14371/2308.09458); **Rahman "Gang of Eight"** defect taxonomy incl. idempotency (arXiv 2505.01568, `chrisparnin.me/pdf/GangOfEight.pdf`); Begoug IaC-SO; **InfraFix** (arXiv 2503.17220).
- Ops-corpus sources: homelab-gitops community (`k8s-at-home`/`homelab`/`home-ops` GitHub topics; `onedr0p/cluster-template` lineage).
- Provider-corpus: Ansible Galaxy API (top collections by download); Chef Supermarket / Puppet Forge / TF Registry.
- Sentiment: HashiCorp's own "provisioners are a last resort" docs (authoritative on the fuzzy edges); HN Terraform threads; practitioner blogs (graded B, opinion). **Rejected/down-graded**: the pile of SEO/AI "X vs Y" listicles (spacelift/attuneops/kestra/guru99/oneuptime/etc.).
Notes: `060-userbase-problemspace-map`, `061-ansible-userstudy-synthesis`, `062-terraform-and-crosstool-userstudy`. Deliverables: **`plans/063-corpus-acquisition-plan.md`** (+ `tools/corpus-survey.sh`), **`plans/064-orchestration-go-no-go.md`**.

## Performance round (round 4) — perf characteristics across the three phases
Mostly *mining the existing on-disk corpus for cost* + targeted external search for ops/distsys prior art (the phase-2/3 gap). New external sources, graded:
- **Heintze–McAllester "On the Cubic Bottleneck in Subtyping and Flow Analysis"** (LICS 1997) — A. The cubic floor for CFL-reachability/points-to (2NPDA-complete). **Paywalled (IEEE); every open mirror is dead (Cornell/Kozen 404).** Result captured in note 71 + the open companion below.
- **Sridharan–Fink "The Complexity of Andersen's Analysis in Practice"** (SAS 2009) — A, open. The cubic-bottleneck companion: Andersen points-to is cubic worst-case but near-linear in practice on real programs (the "cubic floor but small-in-practice" point Dorc relies on). **On disk: `sources/A-sridharan-andersen-complexity-practice-sas-2009.{pdf,txt}`** (1042 lines; manu.sridharan.net).
- **Van Horn–Mairson "Deciding k-CFA is complete for EXPTIME"** (ICFP 2008) — A. Context-sensitivity is provably exponential; the redline for Dorc's context dial. **On disk: `sources/A-vanhorn-mairson-kcfa-exptime-icfp-2008.{pdf,txt}`** (495 lines; brandeis.edu).
- **Mitogen for Ansible docs** (`mitogen.networkgenomics.com/ansible_detailed.html`) — A (primary, author). Orchestration-overhead-dominates evidence; 1.25–7× numbers; `local_action` serialization footgun; connection delegation. Now registered: [A-mitogen-ansible-2024].
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
- **k-CFA paradox** (Might–Smaragdakis–Van Horn, PLDI 2010) — A. The crucial refinement: k-CFA is EXPTIME for *closures*, polynomial for *flat/OO data* — so Dorc's flat fact-map may afford context-sensitivity. **On disk: `sources/A-might-smaragdakis-vanhorn-kcfa-paradox-pldi-2010.{pdf,txt}`** (1365 lines; yanniss.github.io).
- **strace/ptrace vs eBPF overhead** (SysTutorials + practitioner benchmarks, B) — ptrace 2–10× (up to 102×) vs eBPF <1–2%; the effect-observation altitude for calibration/derivation.
- **rust-analyzer durable incrementality** (rust-analyzer blog + issue #4712, A/B) — the incremental poster child *punts* on-disk cache persistence (too subtle); tempers Dorc's "versioned summary format" optimism with a cold-start fallback.
- **SaltStack syndic GitHub issues** (#55564/#43003/#19864, B, maintainer-primary) — the fan-out-tree result-aggregation lock-in ("does not scale", won't-fix).
- **Terraform state-splitting retrospectives** (Medium/Terragrunt, B) — 20–30 min monolith plans; split trades slow-plans for cross-state-dependency complexity (the granularity tension Dorc transposes onto the analysis unit).
- **Puppet/Chef splay + thundering herd** (O'Reilly, Puppet docs/forums, B) — `splay`/`fqdn_rand` jitter; server 503+Retry-After backpressure; the "flocking" re-convergence problem.
- **distsys log-aggregation / backpressure** (Better Stack, design-guru, B) — the fan-in dual of fan-out; backpressure + bounded-buffer + hierarchical aggregation for the realtime-output scale cost.

## Recovery round (round 5) — trace-don't-derive; reintroducing lost values
After the perf-critique demoted derived-dependency soundness (the `docker compose up foo.yaml`→reads-`/mnt/blah` catch), this round grounds the **third option** for deps: observe them at runtime (trace), not declare (specify-the-world) or statically-derive (unreliable). Synthesis split into **`plans/077-pluggability-and-hook-surface.md`** (live core constraints: wrappable leaf-execution seam, dual-use provenance, `--faithful` mode, the unprivileged seccomp network-class backstop) + **`plans/deferred/078-privileged-tracing-tool.md`** (the deferred privileged eBPF/auditd tool; why per-process tracing fails on daemon-mediated tooling; easy/hard security split). *(Earlier `recovery-design.md` was the first cut — reframed by the user as devtool-not-core and removed.)* Key sub-findings: M2 trace-detects-hidden-deps = honest reliability flagging; M4 lifted-guards+oracle-library grows the value band; dep-free UX survivors (plan-as-shell, streaming, modes) live in `076-performance-architecture.md`; the rattle hazard mechanism settles hoist-safety.
- **Spall–Mitchell–Tobin-Hochstadt "Build Scripts with Perfect Dependencies"** (OOPSLA 2020, rattle) — **A, the load-bearing recovery find.** Forward build system: straight-line script, NO declared deps, *trace* file accesses → "perfect dependencies, no way to get them wrong"; speculation + hazards. The mechanism that recovers "invisibility" (priority #4). **On disk: `sources/A-spall-mitchell-rattle-perfect-dependencies-2020.{pdf,txt}`** (728 L; ndmitchell.com).
- **Vendor clones (trace-based dependency inference, full history):** `Vendor/rattle` (Haskell — forward build, speculation, fsatrace-based), `Vendor/tup` (C — FUSE-traced deps + undeclared-access *detection*, the M2 model), `Vendor/fabricate` (Python — strace/access-time traced, memoize.py lineage), `Vendor/fsatrace` (C — the LD_PRELOAD/file-access tracer rattle uses; the pragmatic janky-box-friendly trace mechanism, §4 default). Grades A/B (working primary implementations).
- **tup / fabricate / memoize.py** (gittup.org, brushtechnology, McCloskey; A/B docs) — three independent implementations confirming trace-based dep-inference is old, robust, cross-ecosystem. tup's "error on undeclared access" = M2's hidden-dep detection.
- **Build Systems à la Carte** (Mokhov–Mitchell–Peyton Jones, ICFP 2018, MSR) — **A, the round's highest-value late find.** Scheduler×Rebuilder factoring; verifying-vs-constructive traces; minimality/early-cutoff. The skip-thesis unifying theory. **On disk: `sources/A-mokhov-build-systems-a-la-carte-icfp-2018.{pdf,txt}`** (943 lines; microsoft.com).
- **Bazel remote caching / remote-execution-API / "build without the bytes"** (bazel.build, Aspect/Tweag blogs) — A/B. Content-addressed action cache (CAS); hermeticity requirement; cross-machine reuse = the cross-host-memoization model.
- **Nix content-addressed derivations + early cutoff** (NixOS wiki, Tweag, nix.dev) — A/B. Input-addressed vs content-addressed = the two-modes seating; determinism precondition.
- **Buck2 / DICE / Watchman** (buck2.build, facebook/buck2) — A/B. Incremental-computation-engine convergence (≈ Salsa/Adapton); event-driven change-detection.
**These five build-systems sources → `notes/075-build-systems-as-prior-art.md`** (the skip-thesis prior art + the cross-host-memoization lever).
- **Prometheus** (pull-model scrape/federation/sharding + **staleness** semantics; prometheus.io docs A, Robust Perception A, Grafana/Last9 B) — the probe phase in another domain: pull-knows-inventory (`up`/unreachable=unknown), staleness bounds memoization, hierarchical-federation = fan-out-tree (third domain confirming it).
- **osquery / Fleet** (distributed SQL fleet-state collection; osquery.readthedocs.io A, fleetdm/kolide B) — **target-side watchdog** (don't degrade the monitored host), **differential mode** (report only changed rows), schedule-staggering. → folded into `notes/072`.
- **REJECTED**: the `oneuptime.com/blog/post/2026-*` cluster (AI content-farm: templated titles, future-dated, no author); assorted SEO "X vs Y" listicles; a dead `devseccon.com` Puppet-retrospective URL (301→snyk.io marketing, content gone — not cited).

Notes: `070-performance-map` (campaign map + the perf reframe + the contract-matrix-driven cost complication), `071-analyzer-runtime-perf` (the complexity cliffs + memory wall), `072-probing-parallelism-perf` (fan-out ceilings + thundering herd + resumability), `073-mutation-orchestration-perf` (DAG scheduling + batching + the under-investment trap), `074-cost-model-and-tuning` (DB-optimizer + PGO model for cost-without-annotation). Synthesis: **`plans/076-performance-architecture.md`** (the perf design principles + the "decide-now retrofit-hostile" list + flagged decisions).

## Corpus go/no-go round (round 6 — 2026-06-02) — methodology imports
No new domain sources; the round imported *methodology* (pre-registration, severe testing, inter-rater κ/α) graded in `sources.json`, and built the in-tree `tools/corpus` instrument (TS + tree-sitter — an instrument, not a Vendor clone). Notes `080`/`081`; recovery plan `086`. (Rounds 7–8 = synthesis/charter/kill-criteria; no new sources.)

## State-tracking round (round 9 — 2026-06-02) — shared-state across hosts
Ops-theory + PLT sources, all graded in `sources.json` (grep `traugott|burgess|engler|foster|harnad|ramalingam`): Traugott "Why Order Matters" [A-traugott-order-matters-2002] + Burgess/CFEngine [B-burgess-cfengine-2010] (the divergent/convergent/congruent trichotomy); Engler "deviant behavior" spec-mining [A-engler-deviant-behavior-2001] (the MUST-vs-MAY line); Foster flow-sensitive qualifiers / CQual [A-foster-flow-sensitive-qualifiers-2002] + occurrence-typing (the two formal spines); Harnad symbol-grounding [A-harnad-symbol-grounding-1990] (downgraded — Dorc keeps a referent-agnostic *relational* anchor); Ramalingam [A-ramalingam-undecidability-aliasing-1994] (precise-footprint-undecidable, even intraprocedurally). Notes `091`–`096`; conclusion **`plans/099`** + specimen synthesis `plans/09A`. Specimens are byte-exact real scripts pinned via `tools/inline-specimen.sh` (in-tree, not Vendor clones). No new Vendor clones.

## Security prior-art & threat-modeling round (round 10 — 2026-06-02)
Practitioner/industry-weighted security dive (the TODO security item; threat-modeling-first). All graded web sources reproduced under `sources/` (full skill schema). New sources, graded:
- **Chef "Why-Run Mode Is Considered Harmful"** (`chef.io/blog`, Julian Dunn) — **A**. First-party refutation of the dry-run/no-op contract: guards create inter-resource deps a no-op can't evaluate; "no-op modes are not side-effect-free" (systemd-lockup). Direct prior-art for `kFAIL-withhold`/`kELISION`/the TODO elision-soundness hazard.
- **F-Secure SaltStack authorization bypass advisory** (CVE 2020-11651/52; archived f-secure.com) — **A**. The defining config-mgmt disaster: unauth → root on master AND all minions; 6000+ exposed. The control-node-blast-radius lesson.
- **Schneier "Attack Trees"** (Dr Dobbs 1999, schneier.com) — **A**. Canonical methodology; the reusable-subtree/root-node structure is an *architectural analog* of oracle-composition.
- **David A. Wheeler "Filenames and Pathnames in Shell"** (dwheeler.com, 2010→2025) — **A**. Terminal-escape injection via Dorc's plan-as-shell CLI output; shell-codegen quoting; adversarial remote filenames. Maintained, reference-heavy.
- **CFEngine security / trust model** (gnu.org cfengine docs, Burgess, ~CFEngine-2 era) — **A**. The pull/voluntary-cooperation counter-thesis to push; trust checklist (integrity-over-secrecy, host-identity spoofing, encryption≠trust). Cite for trust model only — convergence is the rejected part.
- **Docker seccomp security profiles docs** (docs.docker.com) — **B**. Default blocks ~44/300+ syscalls ("compat not containment"); ptrace bypasses seccomp. Tests the TODO seccomp-skepticism: classifier, not sandbox.
- **Jessie Frazelle "How to use the new Docker Seccomp profiles"** (blog.jessfraz.com 2016) — **B**. Core-author; strace-derived filters are incomplete by construction (missed 6 syscalls). Caution for `plans/078` trace-to-derive.
- **Shostack "Ultimate Beginner's Guide to Threat Modeling"** (shostack.org) — **B**. Four-Question Framework + STRIDE; *deprecates attack-trees* (expert-heavy) — the democratized-methodology answer for Dorc's two-user gradient.
- **Threat Modeling Manifesto** (threatmodelingmanifesto.org, 15-expert consensus) — **B**. Values/anti-patterns; *Hero-Threat-Modeler* anti-pattern ("everyone can and should") resolves the attack-tree expertise tension for non-expert oracle authors.
- **Andrew Nesbitt "If It Quacks Like a Package Manager"** (nesbitt.io 2026) — **B**. "If it has transitive execution, it's a package manager"; Ansible Galaxy (the analog) shipped opt-in/off integrity, overwritable versions, become-escalating exec, no lockfile. The supply-chain retrofit-hostile checklist.
- **Terraform issue #12489 "data/external executes on plan"** (github hashicorp/terraform) — **B**. Maintainer: plan's "no actions" guarantee doesn't apply to data sources. Plan-time code execution = the probe-non-mutation analog.
- **"TTP Diaries: SSH Agent Hijacking"** (embracethered.com, Rehberger 2022) — **B**. Conditional push-model risk: agent-forwarding through a bastion lets root on any hop pivot to the fleet.
- **Klein "Premortem"** (gary-klein.com author page; canonical HBR 2007 paywalled) — **C**. The methodological framing the human named; author-primary but a thin stub.
- **heipei "SSH Agent Forwarding considered harmful"** (heipei.github.io 2015) — **B** *(b)*. Canonical ProxyJump-not-ForwardAgent piece: a compromised hop both impersonates you fleet-wide *and* eavesdrops/modifies the session; ProxyJump terminates it on the operator workstation. Dorc's secure m→n→o default.
- **ShellCheck README** (github koalaman/shellcheck) — **B** *(b)*. The defensive-lint backstop: catches injection/quoting *patterns* (Gallery: Quoting/Robustness) but is an AST pattern-filter, not a malice-detector — can't certify an unread oracle.
- **Matrix AI "Reproducible Builds vs Semantic Versioning"** (matrix.ai, Qiu 2016) — **B** *(b+ version survey)*. Version strings aren't identity ("1.2.3 only enforces the package *says* it's 1.2.3"); content-hash is; git-commit-hash is the no-NixOS route; semver survives as *intent metadata*, not the soundness gate.
- **seal.security "The Versioning Ghost"** (seal.security 2026) — **C** *(b+; commercial, primary measurement)*. >10k Alpine same-version-string/different-hash collisions (musl 1.2.5-r9 ± the CVE fix); the empirical backbone for version-drift. PURL distro-coordinate as partial fix.
- **REJECTED/not-saved**: the "threat modeling is theater" counter-search (only low-grade SEO/commercial — itself a finding: practitioner consensus is pro-TM, debate is lightweight-vs-heavy); SEO agent-vs-agentless listicles (Palo Alto/Wiz/Orca/Darktrace).

Notes: `100-security-prior-art-and-threat-modeling` (+ gate-adjudication & (b)/(b+) addenda). Map: **`plans/101-security-threat-modeling-map.md`** (trust-boundary map · fronts 1–6 · gate-adjudicated knobs `kAGENTLESS`-welded / `kTRUST`-parked · gap-answers). Deliverable: **`plans/102-dorc-threat-model.md`** (STRIDE-per-element · soundness-boundary doctrine · premortem · per-oracle template · 7 banked items · the parked version-drift / content-hash spike).

## Error / provenance / reporting round (round 11 — 2026-06-03)
31 sources, all `top-level-agent`-graded on a full read, across five prior-art domains; full slug list in **`plans/111`** §5. Anchors by domain — parsing/recovery: [A-bour-merlin-2018] (`result × diagnostics`, never-throw) [A-pottier-reachability-2016] [B-rustc-diagnostics-2024] [B-gcc-libcpp-location-2024]; distributed provenance: [B-prov-primer-2013] [B-otel-spec-overview-2024] [A-wittner-distributed-prov-2022] [B-slsa-provenance-2023]; static-transform provenance: [B-mozilla-sourcemap-2024] [B-llvm-dilocation-2024]; ops: [B-puppet-transaction-report-2024] [B-k8s-api-conventions-conditions-2024] [B-ansible-error-handling-2026]; query-planner: [A-leis-query-optimizers-2015] [B-haritsa-robust-qp-2020] [B-postgres-explain-2024]. Conclusion **`plans/111`**. No new Vendor clones.

## Cross-network TDD / CI round (round 12 — 2026-06-03)
33 new sources (148 total at round close), DST / Jepsen / IaC-self-test / tier-theory clusters; grep `sources.json` `madsim|turmoil|shuttle|jepsen|tigerbeetle|antithesis|fdb-sigmod|sled|polarsignals|s2-dst|moonpool|testcontainers|molecule|test-kitchen|beaker|hughes|eatonphil|warpstream|etcd|tc-netem|scylladb|foundationdb`. The Rust DST crates (`madsim`/`turmoil`/`shuttle`) are *referenced*, **not cloned**. Map `plans/121` (frozen); conclusion **`plans/128`**. No new Vendor clones.

## Platform-compatibility round (round 13 — 2026-06-03/04)
Prior-art graded in `sources.json` (grep `busybox|win32openssh|pyinfra|msys2|mitogen`): busybox-w32 [A-busybox-w32-readme-2024]; Win32-OpenSSH no-ControlMaster #1328 [A-win32openssh-controlmaster-1328-2019] + the `sh -s` stdin-pipe bug #1545 [A-win32openssh-pipe-1545-2020]; pyinfra/Salt controller=*nix-only [A-pyinfra-compatibility-2024]; MSYS path-mangling [B-msys2-filesystem-paths-2024]. Notes `131`–`135`; conclusion **`plans/139`** (+ deferred `plans/deferred/13A`). No new Vendor clones.

## Controller↔host transport round (round 14 — 2026-06-04)
7 graded primaries (`notes/141`), in `sources.json`: apt `APT::Status-Fd` [A-debian-apt-progress-reporting-2022]; bats fd-3 [A-bats-core-writing-tests-2024]; pdsh second-connection-for-stderr [A-pdsh-readme-2024]; FIFO multi-writer atomicity [A-man7-pipe-2024]; ssh-fds-don't-transit [B-unix-se-ssh-fds-not-transportable-2025]; no-pty stream-mangling [B-kamil-ssh-separate-streams-tty-2021]; debconf line-protocol-over-standard-streams. `mitogen` [A-mitogen-ansible-2024] re-mined for the minimal-bootstrap cost. Plan + round-close resolution **`plans/142`**. No new Vendor clones.

## Adversarial premise-review round (round 15 — 2026-06-04) — citation-integrity audit
No new domain sources (a red-team *over* the corpus). Manifest-relevant output: the round's **citation-integrity audit** (`notes/20260604-citation-and-claims-register.md` + `-report.md`) re-checked the corpus's own citations and flagged load-bearing faults the 2026-06-04 self-audit missed — notably an **unverifiable "verbatim" rattle quote** anchoring `plans/deferred/078` (the *conclusion* — daemons defeat per-process tracing — is independently true; the *evidentiary anchor* is invented), plus interpretive over-reads (R3 user-study, R2 k-CFA term-equivocation). **Treat `078`'s tracing rationale as unsourced until re-grounded;** a second citation pass over the interpretive + tracing/transport tiers is banked.

## Implementation spike (round 16 — 2026-06-05)
No new external sources — a build spike (quarantined; record in `plans/16P`/`16Q`). Listed only so the manifest's round coverage is complete.
