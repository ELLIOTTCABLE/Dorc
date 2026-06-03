# Dorc research round — index

Autonomous research rounds grounding the Dorc design (a strict-superset of
POSIX-sh + static effect-analyzer; "the derivable part is how hand-authored
per-command oracles *compose*"). All stored *sources* herein discussed will be
human-authored, graded, and reproduced locally; but all non-resource content is
AI-generated, so take with a grain of salt - these are living notes from the
research and planning process; not verified, authoritative, nor necessarily
coherent.

All human-*authored* content lives at the repo-root; the absolute most
significant findings from this planning are usually reproduced by-hand in
DESIGN.md in the human's words.

## READ ME FIRST

Living index. Keep *this* block rewritten to the current state-of-art; append to the
per-round map below as new spikes land.

The work runs as N rounds (the NN in `NNx-slug.md`), each taking on a different
facet and ending in a **synthesis** under `plans/`. **The syntheses are the
on-ramp, and usually more-heavily human-reviewed; the `notes/` are their raw
material** — open a note only when a synthesis sends you there. Read the
syntheses by *question*, or by date (more recent rounds often incorporate at
least a little bit of prior rounds as context; older ones, although focused, are
often also full of superseded/incorrect info. peruse older rounds with care.)

- **What is it, and where's the hard part?** — `plans/021` (empty dir → CFG/effect engine) +
  `plans/041` (language / parser / orchestration decisions).
- **Can sh be analyzed soundly enough?** — `plans/055` (analysis architecture: sound+precise
  probe-reduction · reusable fact-structure · corpus-scale + incremental).
- **Who needs it, and what do we build?** — `plans/064` (per-feature integrate-vs-delineate
  matrix; userbase evidence in `notes/060`–`062`).
- **Fast enough?** — `plans/076` (performance architecture + the "decide-now, retrofit-hostile"
  list).
- **What must the core never optimize away?** — `plans/077` (the wrappable-leaf hook surface +
  seccomp network backstop — *a live constraint, not history*).
- **Cross-domain synthesis + the corpus charter** — `plans/083`.
- **How do we build it to fail fast?** — `plans/088` (falsification-first, build-to-kill —
  *advisory, not a phased plan*; reasoning in `notes/087`, kill-listing in `../DESIGN.md`).
- **Tracking shared state across hosts** — `plans/099` (latest round's conclusion: relational
  contracts over referent-agnostic symbols · MUST-vs-MAY · the IFDS decidable floor).

Through-line worth holding: the **soundness story keeps getting re-cut** — bias-inversion
(`051`) → perf demotes statically-derived deps (`076`) → trace-don't-derive recovery (`077`) →
relational MUST/MAY contracts (`099`). Later cuts supersede earlier framings of *how much Dorc
can know without running the host*; on that question, the later round wins.

> *Latest — round 9, 2026-06-02, state-tracking:* the `plans/099` conclusion above, now
> ground-tested against verbatim real-world scripts in the new `specimens/` corpus —
> `specimens/090` (a kernel-dev task-runner; the **bless-vs-abdicate** idiom ledger) and
> `specimens/091` (stack's installer; the **m×n abdication** motivator + meta-contract debt).
> Specimens are reproduced commit-pinned + byte-checked via `tools/inline-specimen.sh`.

## The per-round map (reference — the spine above is the curated reading order)
- `notes/000-source-manifest.md` — every source, graded (quality/relevance) + the **license contamination map**.
- `{notes,plans}/YYx-slug.md` where "x' is the highest number for each "YY" research-spike

### Foundations (round 1 — parse, engine architecture, positioning)
- `notes/010-parsing-shell.md` — statically parsing POSIX shell (Morbig); the trust argument.
- `notes/020-colis-architecture-and-coq-verdict.md` — the engine/oracle architecture that scales; **why Coq is not justified**.
- `notes/030-corpus-evidence-and-positioning.md` — real-world corpus evidence (28k + 1.35M scripts); the bootstrap-oracle list; positioning vs ShellCheck.
- `notes/040-parser-architectures-and-cribbability.md` — Morbig vs Oils vs mvdan; what we can legally crib.
- **`plans/021-phase-1-static-analysis-engine.md`** — **the hard part**: empty dir → CFG/effect engine.
- **`plans/041-phase-2-language-workload-orchestration.md`** — language / parser / orchestration (decisions to make).
- `learning-path/README.md` — curriculum for the human (anchor: the SPA textbook).

### Analysis round (round 2 — soundness/reachability/mutation, the user's real concern)
- `notes/050-analysis-prior-art-map.md` — the campaign map (reframe of "soundness" as over-approximation; bodies of work per Q1/Q2/Q3).
- `notes/051-mutation-core-and-compositional-scaling.md` — MOD/purity domain + compositional summaries; **the soundness-bias inversion** (now refined into the two-soundness standard — *probe-soundness* vs *elision-soundness*; see AGENTS §1).
- `notes/052-ifds-engine-and-datalog-bridge.md` — IFDS/IDE engine; side-effect & Datalog bridges.
- `notes/053-reusable-structure-and-scale-mechanisms.md` — PDG/SDG vs Datalog vs value-flow; scale levers.
- `notes/054-dynamic-language-soundness-tajs.md` — sound AI of a dynamic language (recency abstraction; eval→⊤).
- **`plans/055-analysis-architecture.md`** — the synthesis: answers Q1 (sound+precise probe reduction), Q2 (reusable fact/dependence structure), Q3 (corpus-scale + incremental).

### Userbase & problem-space round (round 3 — corpus, orchestration go/no-go, user-study)
- `notes/060-userbase-problemspace-map.md` — campaign map (3 spaces, grading).
- `notes/061-ansible-userstudy-synthesis.md` — Ansible empirical user-study (Carreira 2025: 59k posts + 20 interviews); validates the Dorc thesis + lists capability gaps.
- `notes/062-terraform-and-crosstool-userstudy.md` — Terraform + the ranked cross-tool synthesis (failures / costly choices / sacred cows).
- **`plans/063-corpus-acquisition-plan.md`** — test-data corpus plan (bootstrap from academic corpora → homelab-gitops → fuzzy-edge sampling) + `tools/corpus-survey.sh` (dry-run-validated `gh` spike).
- **`plans/064-orchestration-go-no-go.md`** — per-feature integrate-vs-delineate matrix.

### Performance round (round 4 — perf characteristics across the three phases)
- `notes/070-performance-map.md` — campaign map; the **perf reframe** (shortcuts are elision-safe → tune toward speed in the safe direction); the probe-optimizer-is-behaviour-dropping framing; the cost-model complication.
- `notes/071-analyzer-runtime-perf.md` — the complexity cliffs (cubic floor, IFDS O(ED³), **k-CFA EXPTIME** = the context-sensitivity redline) + the **memory wall** (recency as a memory lever).
- `notes/072-probing-parallelism-perf.md` — compile-the-probe sidesteps Ansible's N·H round-trips; the three ceilings (controller async-not-fork, sshd `MaxStartups`/`MaxSessions`, thundering herd); the **resumability dividend**.
- `notes/073-mutation-orchestration-perf.md` — derived-DAG scheduling (critical-path / RCPSP / Graham anomalies); the batching/blast-radius knobs; the **under-investment trap**.
- `notes/074-cost-model-and-tuning.md` — DB-optimizer + PGO model for **cost-without-annotation** (default-conservative + profile-guided from realtime-output); the tuning knobs at the duality-intersections.
- `notes/075-build-systems-as-prior-art.md` — **build systems = the skip-thesis in another domain**: the Scheduler×Rebuilder vocabulary; the **cross-host probe-memoization** perf lever; content-addressing seats the two modes; early cutoff; **hermeticity validates non-determinism-exclusion**.
- **`plans/076-performance-architecture.md`** — the synthesis: perf design principles + the **"decide-now retrofit-hostile" list** + flagged decisions. Includes the build-systems lens, the cross-host-memoization lever, and the two boundary findings (probe-vs-just-run; when Dorc *loses*, and how memoization narrows it).

### Recovery round (round 5 — trace-don't-derive; after the perf-critique demoted derived-dependency soundness)
- **`plans/077-pluggability-and-hook-surface.md`** — *live core constraint.* The wrappable leaf-execution seam (both phases), dual-use provenance, `--faithful` un-optimized mode, and the unprivileged **seccomp `socket(AF_INET)` backstop** that ships for all users (network cost-class + undeclared-network detection, fail-closed). What the core must not optimize away.
- **`plans/deferred/078-privileged-tracing-tool.md`** — *deferred privileged devtool.* Why per-process tracing (LD_PRELOAD/ptrace) structurally fails on daemon-mediated ops tooling (docker→dockerd out-of-process), so the real value needs **eBPF/auditd system-wide** (privileged → quarantined separate binary); the easy/hard security split; containerized fixtures as the reproducibility complement. Output is always proposed `sh` guards, never metadata.

### Corpus go/no-go round (round 6 — instrument + first tally, then the adversarial pass)
- `notes/080-corpus-spike-progress-and-first-tally.md` — *interim, not the final go/no-go.* The in-tree `tools/corpus` instrument built + validated (TS + tree-sitter; 10,350 files in 3.7s) and a first tally on a contrast-not-compound public sample. +SURE parse-feasibility is not the risk (0.0% hard parse-fail, eval 0.1%); ~SUSPECT idempotency is **module-native, not guard-native** (`creates:`/`removes:` ≈ 0.1%) → nudges the `kDEPS` split oracle-heavy (hold loosely — pre-bias). Load-bearing caveat: the raw tally was **~95% collection test-code**; scanner **v2** excludes test paths, splits mutating-vs-control, and **stratifies** (caught a live Simpson's paradox — guarded/mutating = role 45% / homelab 7%). The `Q-BAND` go/no-go stays open: it needs the §7 apply-cost×check-depth rules, parked for the user as "taste + ops-experience".
- `notes/081-blind-multimodel-study-critique.md` — *methodology note, not a measurement.* **Don't run the thesis-blind, three-model variant** (scaffolded out-of-tree at `~/shell-iac-corpus-study`) **for go/no-go numbers — it regresses on the standard `[80]` already sets.** Blinding doesn't remove the classification-rule bias; it *relocates* it (→ prompt-framing, each model's training prior, the thesis-aware final compare) where it is opaque + uncorrectable, and "compare the three runs" is uninterpretable both ways (divergence confounds corpus≠measure≠classifier; convergence = shared priors, not truth). The worry is real (the apply-cost/check-depth rules *are* researcher-DoF) — but the fix is the `086` recovery, not blinding.
- **`plans/086-corpus-classification-validation.md`** — *the recovery (supersedes the blind variant).* Return the `kDEPS` go/no-go without confirming-by-construction, on the instrument + sample you already have: **pre-register** the apply-cost/check-depth rules → **sensitivity-test** the verdict across conservative→liberal rule-sets (does taste decide it?) → **ground-truth** a stratified subsample on the planned calibration harness (container fixtures; a few dozen gold ops bias-correct the static band) → *optional* **models-as-raters of one fixed corpus** (measure the subjectivity via κ/α instead of hiding it) → **adversarial** worst-defensible rule-set (severe testing). Keep the instrument + SHA-pinned sample + contrast-not-compound; discard the blind protocol.

### Synthesis, charter & kill-criteria (rounds 7–8 — falsification-first)
- **`plans/083-synthesis-and-spike-charter.md`** — folds rounds 1–5 into one design picture and **charters the corpus go/no-go spike** that `086` then de-biases.
- `notes/087-kill-criteria-critique-and-scope-down.md` — the pivot to **build-to-kill over build-to-spec**; motivates the `A-VALUE` kill-listing reproduced by-hand in `../DESIGN.md` (Sensitivities).
- **`plans/088-implementation-strategy-advisory.md`** — *advisory, not a phased plan.* A falsification-first build order — a dogfood vertical slice meant to kill the thesis early. Reasoning: `087`. (Process scaffolding, not findings: `plans/084`–`085` are the spike's seed/session prompts.)

### State-tracking round (round 9 — tracking shared-state across the described remote systems; 2026-06-02)
- `plans/090-state-tracking-research-plan.md` — the reviewable charter: the problem reified (state-closure; **ambient vs transient** facts — transient = un-probeable); the governing frame (**unsolvable-by-analysis-alone → user-in-loop → floor / frontier / ceiling**); the **knob-vs-contract × weld-vs-adjust** quadrant; "shared state" promoted, convergence demoted.
- `notes/091-state-ops-theory-traugott.md` — ops-native theory: Traugott's **divergent / convergent / congruent** trichotomy (Dorc = *congruent outcomes from convergent inputs*); Burgess; **cross-camp** confirmation that "tracking state + understanding intent" is the unsolved core; the `kVOLATILES` time-sensitivity caveat.
- `notes/092-flow-typing-tainting-and-the-RAL.md` — the two formal spines: **occurrence typing** (latent-proposition narrowing) + **CQual** (flow-sensitive qualifiers; strong/weak update = the aliasing ceiling in operation); the **Puppet RAL** type/provider = the kind/oracle model. (ACSL/ShellCheck demoted to the `kOOB` floor.)
- `notes/093-impossibility-ceilings-and-floors.md` — the walls: **Rice** (semantic-undecidable → recognize-syntax + probe + delegate), **Ramalingam** (precise-footprint-undecidable, *even intraprocedurally*), **frame-problem** (non-effects unenumerable → closed-world frame axiom); the **IFDS** finite+distributive **decidable floor**.
- `notes/094-guard-carrier-specmining-and-grounding.md` — *conversation findings*: the **idempotency guard** (`if ! PROBE; then ESTABLISH`) is the sh-spelled spec-carrier (shared-arg = entity-link; polarity = probe/establish); consumer-guard ≡ oracle.
- `notes/095-grounding-symbol-grounding-and-the-probe.md` — grounding → **relational** (human-adjudicated): Dorc is **referent-agnostic**, keeps relational contracts; the chicken-and-egg is the symbol-grounding regress, resolved by *declaration*, not inference. (Harnad downgraded; the probe merely *executes the oracle's check*, it does not "ground".)
- `notes/096-spec-mining-and-the-must-may-boundary.md` — Engler's **MUST vs MAY** = the sound/unsound line (elision rides MUST only: idiomatic-implied or oracle-declared); spec-mining = *offline* oracle-bootstrap/linter, off the per-run path.

### Specimens (round 9 — verbatim real-world code, literate-annotated; design quarry, not a measurement)
Each specimen is a real script reproduced byte-exact + commit-pinned via `tools/inline-specimen.sh`, then annotated for how facts get *spelled* idiomatically — surfacing candidate idioms to **bless** (collapse to one analyzer-recognised form) vs **abdicate** (delegate the open-ended, higher-kinded zoos to community-named kinds, since baking them is an **m×n** registry of every-alias × every-concept).
- `specimens/090-literate-specimen-kernel-task-runner.md` — kernel-dev task-runner. The bless/abdicate ledger (`[ -f X ]`, `[ A -nt B ]`, `trap…EXIT/ERR`, `set -e`) **plus** the bake-into-core patterns: transient state (`trap`), provisioning-through-a-mount as *transport*, and atomic-publish licensing the probe (TOCTOU). The real-world grounding `099` §9 asked for.
- `specimens/091-specimen-stack-get-stack.md` — stack's `get-stack.sh`: the Puppet-RAL by hand, the **m×n abdication** motivator, and the **meta-contract debt** (how mutually-unaware oracles declare `provide`/equivalence/wrinkles in plain sh). The *last* abdicate-bucket specimen.
- Tooling: `tools/inline-specimen.sh OWNER/REPO PATH NOTE.md` — fetch verbatim + commit-pin + license + sha256, then edit around the single fenced block.
- *Forward:* specimen-hunting pivots from abdicate-bucket exemplars to **bake-into-core** correctness patterns (the control/state-flow the analyzer must model itself).
- **`plans/099-state-tracking-synthesis.md`** — **the conclusion**: the design-space map (§2 walls · §3 decidable floor · §4 contracts/q-floor · §5 knob deltas · §6 the probe's role · §7 spec-mining placement · §8 the two spines).

## Vendor/ (full-history clones)
CoLiS ecosystem (morbig, morsmall, colis-language, colis-constraints, shstats, lintshell, …), shellcheck, mvdan-sh, smoosh, oils, goblint-analyzer, tree-sitter-bash. See manifest for grades/licenses.
