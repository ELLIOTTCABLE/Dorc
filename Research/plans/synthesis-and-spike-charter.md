# Synthesis & corpus-spike charter

> **Status (2026-06-01): current top-of-stack synthesis for the planning round.** Ties the four
> research rounds (feasibility / analysis / userbase / performance) + the round-5 recovery into one
> cross-domain accord, and converts it into the **completionist question-set the corpus spike must
> answer**. Where this conflicts with an earlier per-domain plan, **this wins** (it's later and
> cross-cutting) — but it is *not* the final implementer-structured plan; that re-fold happens
> *after* the spike returns. Design tensions are referred to by their `KNOBS.md` slugs (`kFOO`).
> Confidence markers (`+SURE/~SUSPECT/-GUESS/--WONDER`) throughout.

## 0. Why this doc exists
The corpus spike (TODO line "skim shell-script corpii to validate design") is the **last gate before
first code**, and it is **expensive and slow** — so it must be *completionist*: one pass that feeds
*every* corpus-answerable design question at once, not a narrow skip-rate measurement. The four
research rounds each explored their own corner deeply and siloed; several of their open decisions turn
out to be answerable by the *same* corpus pass. This doc collates those into one question-set, parks
the premature concerns, names the doors to keep open, and flags where the older plans now actively
mislead.

## 1. State of the project, in one paragraph
Dorc = a strict-superset-of-POSIX-sh **language** + a static **effect-analyzer** that derives *how
hand-authored, in-shell oracles compose* across a script/function/fleet into a skip-safe, parallel,
read-only **probe**, then applies only the mutations each host still needs — "`terraform plan/apply`
for imperative shell." The thesis is strongly evidence-backed *for shell feasibility* (shell is
statically parseable; the engine+oracle split scales; real ops shell is short/linear/eval-rare) — but
the **engine-vs-oracle/trace investment split is genuinely open** and is the spike's job to settle (§2).

## 2. The real go/no-go — an investment calibration, not a design pivot (`kDEPS` + `effort-allocation`)
+SURE this is the most important unresolved thing in the corpus, but it is **not** a knob-flip: the
design axis `kDEPS` is *welded* to `kDEPS-accept-partial` (the anti-declarative thesis). What's open is
the **investment split** within that welded choice — how much of the accept-partial work the **static
engine** carries vs how much the **oracle-library + runtime-trace backstop** must. `static-derive` and
`runtime-trace` are complementary means to the same end (you want both); round-4 (perf) + round-5
(recovery) only demoted *how far static-derive alone reaches* — the `docker compose up foo.yaml` →
silently reads-`/mnt/blah` catch shows static analysis structurally cannot see daemon-mediated/hidden
deps. The corpus sizes the split, per the corpus-acquisition gate: *fat VALUE band → engine-heavy; thin
VALUE but fat HARD band → oracle-library/trace-heavy, engine secondary; high un-analyzable rate → rework
the thesis before any code.* **That measurement has not been run.** The mtime record shows the drift:
`analysis-architecture` + `phase-1` (5/30, the engine cathedral) *predate* `performance-architecture` +
`corpus-spike-seed-prompt` (5/31, the round that reined them in), so `analysis-architecture` describes
the firmest *engine design* but overstates its *primacy*. Until the spike returns, **treat
engine-primacy as provisional** — the engine *design* stands; its *weight* is TBD.

Reassuringly, the same first step is right under *all* outcomes: the corpus measurement is (a) the
go/no-go, (b) the empirically agent-safe category of work (MSR-2026: doc/CI/measurement PRs merge;
novel-engine/bugfix PRs fail), and (c) upstream of nearly every other open knob. The near-term work
below is deliberately **valuable whichever way the split lands** (corpus tooling + the ~40 bootstrap
oracles + Tier-A intraprocedural skip + a thin executor all pay off in the engine-heavy *and*
oracle/trace-heavy worlds).

## 3. THE SPIKE — the completionist question-set (primary deliverable)
**Two corpus *purposes*, not one corpus** (+SURE this distinction is load-bearing and the favorable
historical numbers conflate it):
- **Purpose-1 — shell corpora** (Debian maintainer-scripts, bash-in-the-wild, and broad public
  provisioning/ops shell): test parse-feasibility + the apply-cost×check-depth banding of the
  substrate Dorc actually *parses*. The "99.9% parse / shallow control-flow" numbers are
  Purpose-1 facts and say **nothing** about Purpose-2.
- **Purpose-2 — Ansible roles / Galaxy + IaC corpora** (Opdebeeck's 15k-script Ansible PDG; Galaxy API
  top collections; public `home-ops` / awesome-ansible trees): test *workload-shape + oracle-demand* —
  what work people express imperatively, which effect-classes/checks recur, how cleanly it maps onto
  sh-Dorc. This is the closer match to "what Dorc is for" and the source of the bootstrap-oracle ranking
  and the HARD-band sizing. Opdebeeck's Ansible PDG is a bonus: an existing CFG/dataflow model of ops
  code = a precedent for exactly what Dorc derives.

> **Method honesty (standing):** on a *static* corpus both cost-axes are *heuristic estimates*
> (command-name→cost-class; "guard exists ∧ is a known-shallow predicate"→shallow). State the heuristic
> + error bars; claim no false precision. Good enough to *size* bands, not measure exactly; a later
> container-fixture pass (the calibration harness) ground-truths a sample by *running* it. Shortcut:
> extend `shstats` (cloned, OCaml, already does corpus-wide shell-AST stats) rather than writing a
> walker from scratch. **No bulk download / no API hammering without explicit per-run leave**
> (`corpus-survey.sh` is dry-run by default); bootstrap from the curated academic corpora (labels
> included) *before* GitHub scraping.
>
> **Contrast, not compound (load-bearing):** sample corpora representative of the *broader world*,
> **deliberately NOT the user's own scripts** — those already encode the user's preferences (which also
> shape Dorc's design), so measuring them would *confirm the design by construction* instead of giving
> the independent reality-check the spike exists for. (No relevant user homelab tree exists anyway;
> ignore the `~/System` ansible and the user's GitHub contribs.)
>
> **Provenance — thread the hard data through (load-bearing):** treat this as a research study.
> Statistics is wide-open to noise, hallucination, and misinterpretation, so **every takeaway must
> travel with the raw numbers + the exact method that produced it** (sample N, the classification rule,
> the parse-failure rate, the exclusions) — never a dislocated summary claim. A reader must be able to
> trace any "X% of ops are Y" back to its counting procedure; methods are preserved at full fidelity
> even when results are summarized.

### A. Primary go/no-go — `kPROBING` (Purpose-1, the VALUE band)
- **Q-BAND** — classify each mutating op on two axes → band sizes. **apply-cost:** cheap-idempotent
  (`mkdir -p`, no-op `apt` on a current pkg) vs expensive/dangerous/slow (`apt install` fresh, service
  restart, build, migration, remote call). **check-depth:** shallow (a cheap read-only guard *fully*
  captures need — pkg@version, file-exists, port-open) vs deep (hidden/hard deps — `nginx -t`,
  `docker compose up`, daemon-mediated). → **VALUE** = expensive∧shallow (Dorc's whole win; **its size
  is the primary go/no-go**); **JUST-RUN** = cheap (probe is pure overhead — *subtract it out*, it
  inflates naive skip-rate); **HARD** = expensive∧deep (needs the oracle-library + tracer; feeds `kDEPS`).
- **Q-ANTICORR** — *the single most decision-relevant number*: among **expensive** ops, the
  shallow:deep ratio. Mostly-deep → VALUE band thin *by nature*, oracle-library/trace-heavy
  (`kDEPS` split → engine secondary). Many-shallow → thesis healthy (engine-heavy).

### B. Caps & architecture-shaping (Purpose-1, "also-answer" — one pass feeds all)
- **Q-UNANALYZABLE** `kDEPS` `kPRECISION` — the ⊤-bound rate: fraction of mutating leaves forced to
  must-probe/can't-skip by external/non-deterministic reads, no oracle, or dynamic constructs (`eval`,
  dynamic command names, `source "$dyn"`). **Caps everything downstream.** Corpus priors say ~rare
  (eval 9%/4%) but the ⊤-bound rate is the top unmeasured risk.
- **Q-PARSE** — clean-parse % under a strict POSIX core (not re-citing Debian's 99.9% — measure it on
  the sampled public corpus). Plus: which features hit the un-parseable/`unsafe` boundary, and at what
  frequency (the defined-or-`unsafe` decision list).
- **Q-CHEAPFRAC** `kPROBING` — cheap-mutation fraction: how much headline skip-rate is illusory
  JUST-RUN (for how many ops is just-running idempotently cheaper than probe(stat+round-trip)?).
- **Q-FLAT** `kCONTEXT` — does the abstract state stay **flat** — guards are flat system-state
  predicates, not closures recombining variables across calling contexts (the k-CFA-paradox fault
  line)? AND would context-*insensitive* analysis actually lose meaningful skips? (Decides whether the
  EXPTIME cliff is even in play, and whether per-host context is an affordable dial.)
- **Q-COSTVEC** `kFLATTEN` — which cost dimensions occur (local-stat / local-exec / network /
  remote-rpc / shared-resource) and at what frequency? Do checks call out to networks/daemons/remote
  services (→ keep-under-guard) or stay local (→ hoist-and-batch)?
- **Q-HOMOGENEITY** `kSTATE` — fleet/role homogeneity → equivalence-class size = the verdict-reuse
  rate. May say "memoization not worth it" (heterogeneous) or "it's the whole game" (homogeneous).
- **Q-WORKINGSET** `kFACTS` — corpus size / working-set estimate → does a materialized (Datalog-style)
  fact base fit commodity RAM, or is IFDS/demand forced? (RSS-relevant; the ops corpus is far smaller
  than OpenJDK, so likely fits — verify, don't assume.)
- **Q-MODULARITY** `kUNIT` — confirm the embarrassingly-modular R-roles×H-hosts structure
  (mostly-independent roles) that compositional summaries + finer unit-granularity exploit.
- **Q-INFER** `kBURDEN` `kOOB` — *the metadata-minimization question, made measurable*: how much of
  {effect-class, cost-class, memo-key} is **inferable** from sh-structure (e.g. a `curl`/`ssh`/`nc` in
  a check body ⇒ `network` cost-class; the presence/shape of a leading guard ⇒ shallow) or cheap
  observation, vs **must be declared**? This sizes the irreducible OOB floor — the smaller it is, the
  more we keep "all-sh, human-visible" and the less metadata sprawl we accrete.

### C. Niche & adoption (mixed corpus)
- **Q-FUZZYEDGE** — density of imperative-shoehorned-into-declarative signatures (`local-exec`,
  `null_resource`, `terraform_data` + `triggers=`, k8s `Job`/`initContainers`/`postStart`, Helm hooks,
  `cloud-init`/`runcmd`, Ansible `command`/`shell`/`raw`/`script` + `creates:`/`changed_when:`). Density
  = "this team is fighting their declarative tool" = prime Dorc candidates = the niche detector *and* an
  adoption-sizing signal.
- **Q-PROPERTY** `effort-allocation` — which **derived properties** have corpus support AND day-1
  "README-bullet" sell — skip, version-correct-check (MH2), undeclared-network flag (the seccomp
  backstop), drift-as-the-same-probe, diff-impact? Sizes which analyses-on-top to build first (high
  per-day marginal value) and which to advertise.

### D. Workload & oracle-demand (Purpose-2, Ansible/Galaxy)
- **Q-OPDIST** — distribution of operations/commands across real ops/Ansible work → the
  **bootstrap-oracle priority ranking** (extend bash-in-the-wild's builtin/coreutils ranking to the
  ops population; ~40-50 oracles cover the bulk). Directly the day-1 oracle set.
- **Q-CHECKPAT** — what idempotency/check patterns do Ansible modules + guarded shell encode
  (`creates:`/`changed_when:`/`failed_when:`/module check-mode) → what a Dorc `.check` oracle must
  actually *do* per command-class.
- **Q-THINWRAP** `kDEPS` — how much ops work is "thin idempotent CLI-wrapper" (maps cleanly to sh-Dorc,
  feeds VALUE) vs genuinely complex/daemon-mediated (HARD band)? Extends round-1's
  `community.general`-is-mostly-thin-wrappers finding to the top-N collections.
- **Q-EXPRESS** — which constructs resist clean translation into sh-Dorc? The corner-cases that resist
  *are* the language-design feedback (next round's input).

## 4. Defer-someday ledger — `<don't-build-now>` (parked; point future docs here)
+SURE several research efforts drifted org-heavy; these are real but premature against the homelab/
individual-dev target (DESIGN) and the unrun go/no-go. **Reserve the seam, document the redline, do
NOT carry as active MVP design weight.** Each is keyed to its knob/source.
- **Fan-out-tree relay** (`kSCHEDULE`; perf §6 #5) — flat single-controller caps at low-thousands;
  org-scale needs tiered SSH relays *with the Salt-syndic result-aggregation lesson baked in* (batched/
  streamed, never per-result ack). Homelab never hits this. Reserve hook only.
- **Async/backpressure fan-in machinery** (perf §6 #8) — the realtime-output-at-scale tension
  (thousands of producers → one consumer; priority-split verdicts/errors-never-dropped). Real at org
  scale; for ≤ low-thousands a structured-bounded-per-host result message suffices.
- **Cross-host memoization implementation** (`kSTATE`) — *reserve* the content-key+freshness in the
  verdict shape now (cheap, retrofit-hostile); *build* the reuse only if Q-HOMOGENEITY says it pays.
- **Tier-B interprocedural analysis** (IFDS/IDE summaries, backward slicing, the canonical-fact ontology,
  Datalog query layer) — `phase-1` Step 5 already defers this "only if the 10% earns it." The full
  `analysis-architecture` hybrid is the *reference design*, not MVP scope.
- **The privileged eBPF/auditd tracer** (`deferred/privileged-tracing-tool.md`) — already correctly
  quarantined (separate privileged binary, post-language). Core's only obligation now is the hook surface.
- **RCPSP / optimal scheduling, k8s-style rolling/canary/readiness machinery** (`kSCHEDULE`, perf §4) —
  executor SEAM; Dorc *derives the inputs*, but the executor builds a subset for v1.

## 5. Doors-to-keep-open ledger — retrofit-hostile seams to reserve NOW
+SURE these are the genuine "decide the *shape* now even though you build later" items (the small, real
subset of the perf §6 list). Reserve the minimal seam; don't build the machinery.
- **Compositional + incremental analysis from commit 1** (`kUNIT`; Facebook's lesson) — NOT
  whole-program batch. Retrofit = rewrite. *The* core analysis-architecture redline.
- **Versioned on-disk IR/summary format** (cheap; perf §6 #4) — needed for diff-time incrementality
  *and* team/oracle sharing. Version now; treat *persistence* as an optimization-with-cold-start-fallback
  (rust-analyzer punts it; Dorc's diff-primary smaller corpus tolerates cold-start in v1).
- **Analysis-plane ⊥ execution-plane** (the serialization seam; phase-2 §intro) — already core; keep the
  analyzer/executor separable over a JSON-ish IR/verdict contract so a heavy compile-the-probe never
  starves the live push, and the impl-language never reaches target hosts.
- **The leaf-execution seam** `kFIDELITY` `[H]` — every leaf executes through a process-level wrappable
  indirection (prefix + env + stable provenance id), in **both** probe and apply runners; never one
  opaque `sh -c`. One seam simultaneously serves: probe-soundness gating, the hoist/guard/drop optimizer
  (as provenance-preserving rewrites over the leaf-id set), the seccomp backstop, `--faithful` mode, and
  the future tracer. Design it once, here.
- **Async (never fork-per-host) executor *shape*** (`kSCHEDULE`; perf §6 #1) — even though the executor
  is SEAM and v1-thin, the *concurrency model* is retrofit-hostile (Ansible's literal lock-in). Pick
  async-event-loop shape; don't build the fan-out tree.
- **Context-insensitive *default*** (`kCONTEXT`) — a one-line discipline now that prevents the EXPTIME
  cliff; add context only on confirmed-flat-domain evidence.
- **Verdict shape carries `(verdict, content-key, freshness)`** (`kSTATE`) — even before memoization is
  built, so adding cross-host reuse later isn't a cache-read rewrite. *(But see `kSTATE`: whether to
  persist any of this at all is itself open — reserve the shape, don't commit to persistence.)*

## 6. Doors-to-keep-CLOSED ledger — do not accrete these
- **No central/persistent state store** (`kSTATE`) — Terraform's single biggest liability (contention,
  slow plans, secrets-in-state, drift). Dorc probes real host state; the host is the source of truth.
  *(Open caveat per `kSTATE`: the stateless-recompute stance is a lean, not yet interrogated — the
  verdict cache is where state could re-enter, and whether to allow it is an open question, not a
  settled "never".)*
- **No metadata sprawl** (`kOOB`) — minimize OOB; resist every temptation to add a YAML/JSON sidecar
  where inference, sh-expression, or runtime observation could carry it instead.
- **No `kDEPS-declare-world`** — if we ask the user to specify everything, they should just use
  Nix/Terraform.
- **No throwaway Ansible-transpile v1** — bootstrap oracles are cheap and enumerable; Ansible can't
  stream output nor frontload. (phase-2 §D; ratify the reversal.)
- **No fork-per-host executor; no `select()`-based event loop** (the 1024-fd wall).

## 7. Inconsistencies ledger — where the older corpus actively misleads (pointers, not fixes)
Strictly-additive supersession markers have been applied to the files marked ⟢; human-authored docs
(README/DESIGN/TODO) are **flagged for the user, not edited**.
- ⟢ `analysis-architecture.md` / `phase-1` — present engine-first as settled; **its *weight* is gated on
  the `kDEPS` investment split / the unrun go/no-go**, and the later perf/recovery rounds demoted how far
  static-derive alone reaches. (Marker added.)
- **the metadata tension** (`kOOB`) — `pluggability` + `corpus-spike-seed-prompt` assert "no metadata
  layer, all sh"; `cost-model` (note 74) + `performance-architecture` + AGENTS engineer-contract require
  effect-class / cost-class / `(verdict, content-key, freshness)`. Reconciled via `kOOB` (minimize,
  don't deny) + **Q-INFER** (measure the floor). *Substantially settled by the knob framing; floor is empirical.*
- **the "90% skip" / raw-skip-rate framing** (README, DESIGN, AGENTS MH1) — disavowed by the later
  banding (raw skip-rate is "a trap"; most skips may be worthless JUST-RUN). The honest metric is the
  **VALUE band size** (`kPROBING`), not raw skip-rate. (README/DESIGN flagged-for-user; AGENTS marker added.)
- **AGENTS "current focus: the analysis engine"** — imprecise; the immediate next action is *the corpus
  go/no-go spike that decides the engine-vs-oracle investment split.* (Marker added.)
- **"measure the user's own homelab corpus"** (`corpus-spike-seed-prompt`, `phase-1` Step −1, note 30) —
  **reversed** by *contrast-not-compound* (§3): sample *public* corpora, not the user's own. (Markers/notes added.)
- **README staleness** (user-acknowledged) — elevator pitch predates several design decisions; treat as
  tiebreaker-for-intent only, supplanted by later-mtime docs this round.

## 8. Language note (deferred, correctly) — `[not now]`
The impl-language `[USER DECISION]` is downstream of the `kFACTS` substrate spike, which is downstream of
the corpus (Q-WORKINGSET). **Do not force it now.** The user's live lean (per direction): **all-Rust**
(better single-binary distribution + a deliberate skill-stretch, but more LLM-dependence) **vs** a
**minimal OCaml analysis-engine via Melange + a TypeScript harness for the generic/UX bulk** (both
languages the user is expert in → faster iteration, less LLM-dependence; TS for
contributor-friendliness/adoption). Correction to `phase-2 §A`: the Melange option is **not**
"over-engineering for v1" — for this user it's an *adoption/contributor-friendliness* play, a legitimate
and live contender. Decide after the spike.

## 9. Next step — the corpus spike (runs in a SEPARATE agent session)
The spike executes in its own context window; kickoff prompt: `Research/plans/spike-session-prompt.md`.
This charter is the spike's **WHAT** (the §3 question-set); that prompt is its **HOW** (generic operating
rules). Shape:
1. **Gather a large, representative corpus — a real research subtask, not a footnote.** Where to get a
   big Ansible/ops collection is itself under-determined; **do not underweight it.** Start from the
   curated academic sets (Opdebeeck's 15k-script Ansible PDG, Rahman's labeled defects, GLITCH oracles —
   large, deduped, *labelled*) → then sample Ansible Galaxy + public GitHub (`home-ops`, awesome-ansible)
   for breadth. **Representative-of-the-world, NOT the user's own** (the contrast-not-compound rule, §3).
2. **Data lives out-of-tree; reproducibility lives in-tree.** The corpus (GBs) must **not** sit in the
   Syncthing-synced tree (that is exactly the `Vendor/` churn + process-lock pain). Acquire into an
   out-of-tree cache (`$XDG_CACHE_HOME/dorc/` or `/tmp` for a single session); keep a small **in-tree
   manifest** pinning every source (commit SHA / collection@version / Zenodo DOI + checksum + SPDX
   license) so the exact corpus re-acquires deterministically. **Separate `acquire` (network,
   rate-limited, resumable) from `analyze` (offline, pure, re-runnable).**
3. **Build the instrument to keep, not to throw away.** It gets checked into git and likely reused — so:
   typed, defensive (tree-sitter's error-tolerance is the asset for in-the-wild messes), tested,
   `mise`-pinned toolchain, lockfiled, rebuildable-from-first-principles (the `Vendor/` standard).
   Recommended: a TypeScript scanner (tree-sitter-bash for shell + a robust YAML/Ansible task-extractor).
   **Confirm the toolchain + get install-leave before installing anything (`mise`-managed, never
   system-global).**
4. **Run the §3 question-set**; report band-sizes + the anti-correlation ratio + every also-answer with
   **sample sizes, error bars, and explicit heuristic caveats — qualified, scoped, not overstating what a
   static pass can know.** Persist as a new `Research/notes` entry; **return the `kDEPS` investment-split
   go/no-go.**
5. *Then* the post-spike re-fold: "domains-of-research" → "phases-of-implementation" (per-dir
   `CLAUDE.md` + project skills + a master pointer-doc).
