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

**Start here (current state):** the standing design accord is the human-authored `../DESIGN.md` +
`../KNOBS.md`; the most recent *event* is the **round-16 implementation spike** — read `plans/16P`
(what it built / deliberately deferred — its §3 ledger first) then `plans/16Q` (what's owed next). The
per-facet conclusions below (`055`/`099`/`102`/`111`/`128`/`139`/`142`) are the durable answers each
round settled.

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
- **Where the project is *now*** — `plans/16P` + `plans/16Q` (the round-16 impl-spike record and what it
  leaves owed). *(The rounds-1–5 cross-domain synthesis + corpus-spike charter, `plans/083`, is
  **historical**: it self-labels "current top-of-stack / last gate before first code" but predates rounds
  9–16 and the spike — read it for the early accord, not as current status.)*
- **How do we build it to fail fast?** — `plans/088` (falsification-first, build-to-kill —
  *advisory, not a phased plan*; reasoning in `notes/087`, kill-listing in `../DESIGN.md`).
- **Tracking shared state across hosts** — `plans/099` (the round-9 conclusion: relational
  contracts over referent-agnostic symbols · MUST-vs-MAY · the IFDS decidable floor).
- **Real-world grounding of that round (specimens)** — `plans/09A` (bless/abdicate × bake-into-core · the Tier-A/B canonical-forms ledger · the rarity ≠ effect / contract-not-detector correction).
- **Does it run on Windows / odd targets?** — `plans/139` (platform-compat conclusion: `kLANG` *sh-is-the-product*
  weld; controller = platform-free text + ssh; targets are sh-precondition-gated into tier-A/tier-B, never executor/transpile).
- **How do errors / provenance / "why" flow, end-to-end?** — `plans/111` (one PROV-shaped derivation-DAG, built-forward/
  queried-backward; `(result × diagnostics)` never-throw; N-tier + per-host-forking; controller-side, hosts stay dumb).
- **How do we test a network appliance without a network?** — `plans/128` (deterministic-simulation testing; the one
  all-nondeterminism seam is the controller↔host *transport*; "best-effort" = maximal rigor, ceilings the edge not the kernel).
- **How does the controller talk to hosts?** — `plans/142` (the *executorless-OOB* transport: tool I/O on native ssh
  channels, Dorc-signalling out-of-band split by size/urgency; `kCOMMS`).
- **Where do the plans break / what wastes effort?** — `notes/151` (adversarial premise-review; the convergent finding —
  the *named-kind oracle contract is unspelled and four rounds defer it* — is the hinge `16` stubbed and `16Q` flags as next).

Through-line worth holding: the **soundness story keeps getting re-cut** — bias-inversion
(`051`) → perf demotes statically-derived deps (`076`) → trace-don't-derive recovery (`077`) →
relational MUST/MAY contracts (`099`). Later cuts supersede earlier framings of *how much Dorc
can know without running the host*; on that question, the later round wins.

> *Latest — round 16, 2026-06-05, the implementation spike (`do-4`):* a deliberately disposable Rust
> workspace that **built the cheapest tier of the `055` engine** — a pure, deterministic
> `syntax→analysis→plan` kernel (hand-rolled monotone dataflow, reaching-defs **ambient gate**,
> **observable/replace** elision, phase-keyed `kFAIL`, a probe→apply compiler) — and proved the
> **apply-2 chain runs end-to-end** on real `.sh` under DST + an sh e2e harness. Record: `plans/16P`
> (neutral; durable threads `T1`–`T17`; **read its §3 built-vs-designed ledger first**) + `plans/16Q`
> (forward-look). **Deliberately *not* built — do not read as settled:** the **named-kind oracle contract**
> (the sh idiom an author writes) was a *held strawman, deferred pending `dq-kOOB` — the next decision up*;
> the **precision/recency keystone** (`16Q §1`: without it nothing elides on a realistic book), the apply
> executor, and apply-3 are all `NOT BUILT`.
>
> *Recent rounds (newest first; detail in the per-round map below):* **15** adversarial premise-review
> (`notes/150`+`151`; 2026-06-04; no plan) · **14** controller↔host transport / `kCOMMS` (`plans/142`;
> 2026-06-04) · **13** platform-compat / `kLANG` weld (`plans/139`; 2026-06-03/04) · **12** cross-network
> DST/TDD (`plans/128`; 2026-06-03) · **11** error/provenance spine (`plans/111`; 2026-06-03) · **10**
> security threat-model (`plans/101`+`102`; 2026-06-02/03; Chef why-run refutation, Salt-CVE blast-radius,
> `kAGENTLESS` welded, version-drift spike parked) · **9** state-tracking (`plans/099`+`09A`; specimens;
> 2026-06-02).

## The per-round map (reference — the spine above is the curated reading order)
- `notes/000-source-manifest.md` — every source, graded (quality/relevance) + the **license contamination map**.
- `{notes,plans}/YYx-slug.md` where "x' is the highest number for each "YY" research-spike

### Foundations (round 1 — parse, engine architecture, positioning)
- `notes/010-parsing-shell.md` — statically parsing POSIX shell (Morbig); the trust argument.
- `notes/020-colis-architecture-and-coq-verdict.md` — the engine/oracle architecture that scales; **why Coq is not justified**.
- `notes/030-corpus-evidence-and-positioning.md` — real-world corpus evidence (28k + 1.35M scripts); the bootstrap-oracle list; positioning vs ShellCheck.
- `notes/040-parser-architectures-and-cribbability.md` — Morbig vs Oils vs mvdan; what we can legally crib.
- **`plans/021-static-analysis-engine.md`** — **the hard part**: empty dir → CFG/effect engine.
- **`plans/041-language-workload-orchestration.md`** — language / parser / orchestration (decisions to make).
- `learning-path/README.md` — curriculum for the human (anchor: the SPA textbook).

### Analysis round (round 2 — soundness/reachability/mutation, the user's real concern)
- `notes/050-analysis-prior-art-map.md` — the campaign map (reframe of "soundness" as over-approximation; bodies of work per Q1/Q2/Q3).
- `notes/051-mutation-core-and-compositional-scaling.md` — MOD/purity domain + compositional summaries; **the soundness-bias inversion** (now refined into the two-soundness standard — *probe-soundness* vs *elision-soundness*; see `kFAIL`).
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
- **`plans/083-synthesis-and-spike-charter.md`** — folds rounds 1–5 into one design picture and **charters the corpus go/no-go spike** that `086` then de-biases. *(Historical: the rounds-1–5 accord — superseded as "current status" by rounds 9–16 and the spike; do not read its "last gate before first code" framing as live.)*
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

**Synthesis / on-ramp: `plans/09A-specimen-grounding-synthesis.md`** — bless/abdicate × bake-into-core, the Tier-A/B canonical-forms ledger, and the **rarity ≠ effect / contract-not-detector** correction. Companion to `plans/099` (does not edit it).
- `specimens/090-literate-specimen-kernel-task-runner.md` — kernel-dev task-runner. The bless/abdicate ledger (`[ -f X ]`, `[ A -nt B ]`, `trap…EXIT/ERR`, `set -e`) **plus** the bake-into-core patterns: transient state (`trap`), provisioning-through-a-mount as *transport*, and atomic-publish licensing the probe (TOCTOU). The real-world grounding `099` §9 asked for.
- `specimens/091-specimen-stack-get-stack.md` — stack's `get-stack.sh`: the Puppet-RAL by hand, the **m×n abdication** motivator, and the **meta-contract debt** (how mutually-unaware oracles declare `provide`/equivalence/wrinkles in plain sh). The *last* abdicate-bucket specimen.
- `specimens/092-bc-cfg-shell-env-state.md` — **first bake-into-core** specimen (an *excerpt class*, commit-pinned snippets): the **shell-execution-environment** state the analyzer must model — options (`set -euo pipefail`; `$-`-conditional `set +e` toggles), cwd (subshell-scoped `cd`), traps (canonical + *conditional*), `|| true` best-effort. Carries an early **bc-crossCFG** lead: system-state save/restore looks genuinely rare (the common bounded mutate-then-restore sections are all shell-internal).
- `specimens/093-bc-crosscfg-system-state-rarity.md` — **bc-crossCFG** (task #2): system-state save/restore brackets (do_x;…;undo_x over host state, the W5 wrong-skip) are **rare** — the obvious tool-pairs (iptables-save/restore, setenforce, modprobe/rmmod) are dominated by *persistence / permanent-disable*, not transient revert. One clean counterexample (leifliddy mkosi build: `getenforce → setenforce 0 → trap 'setenforce 1' EXIT`); its undo is a `trap` — but (corrected) the `trap` is a **contract, not a detector**: the identical trap-free do/undo is more common and invisible, opaque mutators leave no trace (W3), and transient-ness is undecidable (Rice/W1). Rarity bounds the *cost* of the conservative withhold-default, never buys detection.
- Tooling: `tools/inline-specimen.sh OWNER/REPO PATH NOTE.md` — fetch verbatim + commit-pin + license + sha256, then edit around the single fenced block.
- *Forward:* specimen-hunting pivots from abdicate-bucket exemplars to **bake-into-core** correctness patterns (the control/state-flow the analyzer must model itself).
- **`plans/099-state-tracking-synthesis.md`** — **the conclusion**: the design-space map (§2 walls · §3 decidable floor · §4 contracts/q-floor · §5 knob deltas · §6 the probe's role · §7 spec-mining placement · §8 the two spines).

### Security prior-art & threat-modeling round (round 10 — the TODO security dive; 2026-06-02)
- `notes/100-security-prior-art-and-threat-modeling.md` — 17 practitioner-weighted sources graded (13 round-10 + ProxyJump/shellcheck for (b) + Matrix-AI/seal for the version survey); findings + verbatim citations + the human's gate-adjudication. **Probe-non-mutation has a first-party refutation** (Chef why-run; "read-only ≠ side-effect-free") *and* **stops at the oracle-grounding boundary** (transfer-to-contract, never eliminate); **control-node = whole-fleet RCE** (Salt CVE); **seccomp = classifier not sandbox**; **Dorc is a package manager** (supply-chain lever; users don't read scripts → defensive-lint backstop); methodology = *democratized* STRIDE.
- **`plans/101-security-threat-modeling-map.md`** — the map + **fronts 1–6** + the gate-adjudicated knobs **`kAGENTLESS`** (was `kBLAST`; push blast-radius — *added to KNOBS, welded*) and **`kTRUST`** (oracle-distribution integrity — *parked/out-of-scope*; cede to git). Gap-answers: oracle-trust → no code-fetching, defensive-lint backstop; **probe-contract → not a decision** (read-only welded-forced, cost best-effort); push → *ergonomic not a security claim*.
- **`plans/102-dorc-threat-model.md`** — the deliverable: STRIDE over 5 trust-elements (operator-node · ssh-hops · probe · oracle · plan-output) + the **soundness-boundary** doctrine (eliminate only in Dorc's *own* code; oracle-behaviour = transfer+mitigate) + premortem + per-oracle template + **7 banked footgun-avoidance items**. Plus **Cross-cutting · version-drift** (the concrete grounding-breaker; content-hash-gating as the no-registry defence — *parked spike*, see `../TODO.md`).

### Error / provenance / reporting round (round 11 — the cross-cutting "why" spine; 2026-06-03)
- `notes/110` (+ `112` ops, `113` query-planner) — 31 graded sources across five prior-art domains (parsing/recovery, program-repair, static-transform provenance, distributed provenance, ops orchestration, RDBMS query-planners); the recurring node-shape + per-tier toolkit.
- **`plans/111-error-provenance-reporting-synthesis.md`** — **the round conclusion**: error/provenance is *one* PROV-shaped derivation-DAG, not a pile of subsystems — `(best-effort result × accumulated diagnostics)` never-throw; compact origin-handles resolved lazily; built-forward/queried-backward; **N-tier + per-host-forking** (exceeds the single-machine prior-art); **the spine *is* the analyzer's own graph** (agree graph-types first, or build two incompatible graphs); **controller-side, hosts stay dumb** (`kAGENTLESS` preserved); ≥3 never-conflated edge-types. Heaviest knob touch: `kFIDELITY` (the N-tier locator-DAG; see KNOBS round-11 marker).

### Cross-network TDD / CI round (round 12 — how to test a network appliance without a network; 2026-06-03)
- `notes/120` (broad sweep) · `122` (DST explainer) · `123` (Rust DST ecosystem + the transitive-dep wall) · `124` (DST↔Dorc seam + the `axis-dst-cost` ladder) · `125` (containerizability quadrant; infer-vs-annotate) · `126` (transient-fault / Jepsen `:ok/:fail/:info`) · `127` (synthesis handoff). 33 new sources.
- `plans/121-cross-network-tdd-ci-map.md` — the **frozen** round map (axes, 3 fronts, `concl-*` callouts); not rewritten by the conclusion.
- **`plans/128-cross-network-tdd-ci-conclusion.md`** — **the conclusion**: deterministic-simulation testing is unusually *cheap* for Dorc; **the one all-nondeterminism seam is the controller↔host *transport* (`ship(host,unit)→results`)** — reserving it (rung L0) + a clock/rand/IO-injecting kernel is the only retrofit-hostile day-1 item; the **tier boundary everyone hand-annotates and nobody infers**; "best-effort" = *maximal* rigor, ceilings the edge never the kernel. A leaf is a *compile-time* `(host,leaf)` coordinate, **not** an RPC.

### Platform-compatibility round (round 13 — orchestrator + target platform-compat; 2026-06-03/04)
- `plans/130-platform-compatibility-research-plan.md` — the reviewable charter (interactive-research `plan.md`):
  framing (Windows in ops), the orchestrator/target decomposition, proposed knobs + fronts. **Gate-revised.**
- `notes/131-platform-compat-prior-art-survey.md` — wide-net prior-art: Ansible/Salt/pyinfra controller=*nix-only;
  Windows-target=executor pattern; Win32-OpenSSH has no ControlMaster (#1328); MSYS path-mangling; Rust build matrix.
- `notes/132-sh-as-product-and-target-precondition.md` — the human's steer: **`kLANG`** (*sh-is-the-product*) as a
  new foundational weld; Windows/odd *targets* are **sh-precondition-gated** (git-bash/WSL/busybox-w32 set once, the
  Ansible⟷Python analogy), never executor/transpile; `kTPLATFORMS`/`kWINLOCAL` proposed, `kTRANSPORT` dropped.
- `notes/133-sh-precondition-and-busybox-viability.md` — **F-PRECOND** (done): Dorc's target precondition (SSH +
  POSIX sh, *no Python*) is a strict *subset* of Ansible's (SSH + POSIX shell + Python); interpreter-as-prerequisite
  is canonical prior art; busybox-w32 runs sh-*syntax* but a non-POSIX *env* (perms bogus) ⇒ `kTPLATFORMS-wide`
  splits into tier-A real-POSIX vs tier-B sh-syntax-only targets.
- `notes/134-crlf-line-ending-hazard.md` — **F-CRLF** (done): Windows-authored CRLF breaks *below* the shell
  (shebang-`\r` = kernel exec failure, un-guardable from sh) ⇒ Dorc normalize-on-ship or detect-and-fail-clear.
- **`plans/139-platform-compatibility-synthesis.md`** — **the conclusion**: `kLANG` welds the pluggable-language
  question shut; orchestrator = platform-free text + ssh (Rust dodges `fork()`); targets = sh-precondition (lighter
  than Ansible's) split **tier-A real-POSIX / tier-B sh-syntax-only**; `kWINLOCAL`/`kTPLATFORMS`; CRLF policy.
  **Reconciled with out-of-band rounds 10–12** — `kCOMMS` disambiguation, `kAGENTLESS` align, F-RUSTCI/F-SSHPOOL
  subsumed by round 12. (F-RUSTCI dropped, F-SSHPOOL folded.)
- `notes/135-win32-bootstrap-mechanics.md` + **`plans/deferred/13A-busybox-win32-bootstrap.md`** — *post-conclusion
  addendum (F-BOOTSTRAP).* Corrects the overstated "first-sh = human-only" boundary: a mechanized `raw`-equivalent
  (scp/`curl.exe` a static busybox.exe → invoke by path) onboards a bare Win32-OpenSSH box to "runs any sh" without
  breaching `kLANG`; robust pattern = scp-then-invoke-by-path (the `sh -s` stdin-pipe is Win32-OpenSSH's buggy zone).

### Controller↔host transport round (round 14 — the `kCOMMS` substrate; 2026-06-04)
- `notes/140` (broad sweep) · `141` (7 graded primaries: apt `APT::Status-Fd`, bats fd-3, pdsh second-connection-for-stderr, debconf line-protocol, FIFO multi-writer atomicity, ssh-fds-don't-transit) · `143` (env/toolchain handoff).
- **`plans/142-controller-host-transport-plan.md`** — variance-map **and** its round-close resolution (read the Resolution, it supersedes the `## My read` lean): the **executorless-OOB design** — tool I/O rides native ssh *batch* channels (channels = batches ≤ `MaxSessions`, internal `&` concurrency) at full fidelity; **Dorc-signalling is out-of-band, split by size/urgency** (short gating `(verdict, content-key, freshness)` on a shared atomic fast-lane; large diagnostics in per-leaf files demuxed by filename); **security is structural** (signalling never shares a lane with freeform). The executor pole shrinks to a narrow corner {no-writable-fs, hard backpressure}. Residual: writable-fs on stripped/Windows targets. (`kCOMMS`; see KNOBS round-14 marker.)

### Adversarial premise-review round (round 15 — red-team the corpus for effort-waste; 2026-06-04)
*No `plans/` synthesis (per human). Adversarial-only — **convergence is the signal, lone findings suspect-until-checked**; target = "meta-poor planning that wastes engineering effort" (go/no-go is YOLO-GO, so the self-kill / value-band findings are recorded-but-de-prioritized).*
- `notes/150-adversarial-premise-review-phase1.md` — 14 clean-context per-round adversaries (R1–R14); *no clean feasibility kill-shot found.* Frontloads **`fM3-ACCRETION`** (later rounds silently overturn earlier "welded/day-1" calls) + **`M4` citation-integrity** faults.
- `notes/151-adversarial-premise-review-phase2-conclusion.md` — 4 cross-cutting agents. **THE CONVERGENCE: the whole analyzer hinges on one unspelled artifact — the sh idiom by which an oracle NAMES its kind / anchors a skip / reports a verdict — reached and deferred by four rounds.** De-risk (X3): an analyzer-internal, `kOOB`-legal kind-index *is* buildable (a lifted index of user-authored declarations ≠ the maintainer-arbitrated registry the design rejects). Also the **`kCOMMS` knot** (five day-1 seams land on the leaf-execution session) and empirically-run oracle bugs (ufw `.`-as-regex; apt-get `-o` leak; oracles fail `dash -n`).
- `notes/20260604-citation-and-claims-{register,report}.md` — the round's citation-integrity audit (flagged e.g. an unverifiable "verbatim" rattle quote anchoring `plans/deferred/078`).

### The implementation spike (round 16 — `do-4`: build to surface design problems; 2026-06-05)
*A deliberately disposable Rust workspace. The 25 round-16 notes + the spike code are **quarantined** (`notes/quarantine-DO-NOT-READ/`) — reach last-mile evidence through the two postmortems' citations; do **not** pull the quarantine back in wholesale.*
- **`plans/16P-spike-postmortem.md`** — the neutral record: the cheapest `055` tier built (a pure deterministic kernel; reaching-defs **ambient gate**; **observable/replace** elision; the witness/license; a probe→apply compiler; DST + sh-e2e), as durable threads `T1`–`T17`, **each tagged against the built-vs-designed ledger (§3 — read first)**. The kind-index *mechanism* exists; the **oracle contract an author writes was a held strawman, not built**.
- **`plans/16Q-next-spike-and-process.md`** — the forward-look (opinion, marked): the **precision/recency layer is the keystone the spike skipped** (`§1`: without it nothing elides on a realistic book); spike-2's `q1-*` build-list (instantiate a *backward* analysis + apply-3; recency/selectors; probe-projection); the retrofit-hostile decisions to **settle on paper first** (`dq-entity-algebra`, `dq-substrate`, `dq-kOOB`); enshrine only the fact-centric anchor's *shape, not spelling*. Plus process lessons for the next throwaway spike.

## Vendor/ (full-history clones)
CoLiS ecosystem (morbig, morsmall, colis-language, colis-constraints, shstats, lintshell, …), shellcheck, mvdan-sh, smoosh, oils, goblint-analyzer, tree-sitter-bash. See manifest for grades/licenses.
