# Seed prompt — corpus-measurement spike (for a fresh context window)

> ➤ **EXTENDED (2026-06-01):** folded into and broadened by `Research/plans/083-synthesis-and-spike-charter.md` §3 (the completionist two-corpus question-set, each question tagged with its `KNOBS.md` slug). Use the charter as the spike's question-set; this remains a valid condensed launch-prompt. **Correction:** ignore this doc's "measure the user's own homelab corpus" — reversed by *contrast-not-compound* (sample *public* corpora; charter §3). Generic operating rules for the spike session: `Research/plans/085-spike-session-prompt.md`.

*Paste/condense this to launch the clean context. It orients, states the go/no-go, and lists every
corpus-answerable open question so one pass feeds them all. **It is not authoritative** — write your
own scripts, challenge this framing, prefer the human-authored docs where they conflict.*

## Mission
Execute the corpus spike (TODO.md line: "skim shell-script corpii to validate design"). Produce the
**MH1 go/no-go evidence base**: measure whether real ops shell has the structure Dorc's value depends
on. This is a *measurement + light tooling* round — **not** bulk acquisition, **not** building the engine.

## Read first (authority order)
1. **Human-authored, authoritative, do-not-edit:** `README.md`, `DESIGN.md`, `TODO.md`. Internalize
   DESIGN's priorities + "Project components" + the gradual-enhancement thesis.
2. **`AGENTS.md`** — the contract matrix (two audiences / two modes / two soundnesses), settled
   principles, the genuinely-open list. Note line 1's warning: `Research/` is unreviewed LLM-generated
   planning-slop — useful, not gospel.
3. **`Research/plans/063-corpus-acquisition-plan.md`** — where to get data (academic corpora *before* GitHub
   scraping), sampling/dedup/licensing, **and the "What the spike must MEASURE" section** (the depth-split
   below, in full). **`tools/corpus-survey.sh`** is the dry-run-by-default `gh` survey spike.
4. **`Research/plans/076-performance-architecture.md` §4b/§5/§6** — the depth-split rationale ("Blow 3"
   anti-correlation), the boundary findings (probe-vs-just-run; when Dorc loses), and the §6 decisions
   tagged `[corpus-sensitive]` (this prompt's "also-answer" list).
5. **`Research/notes/030-corpus-evidence-and-positioning.md`** + **`notes/071-analyzer-runtime-perf.md`** —
   prior corpus numbers (Debian 28k, GitHub 1.35M) and the complexity cliffs the data must speak to.

## THE go/no-go measurement (primary deliverable)
**Raw skip-rate is a trap** — "90% skippable!" is worthless if those are cheap-to-just-run ops where the
probe costs more than the work saved. Classify each *mutating operation* on **two axes**:
- **apply-cost**: *cheap-idempotent* (just-run it; probe is pure overhead — `mkdir -p`, no-op `apt` on a
  current pkg) vs *expensive/dangerous/slow* (`apt install` fresh, service restart, build, migration,
  large pull, remote call).
- **check-depth**: *shallow* (a cheap read-only guard **completely** captures need — pkg@version,
  file-exists, port-open) vs *deep* (hidden/hard-to-specify deps — `nginx -t`, `docker compose up`,
  daemon-mediated; the `/mnt/blah` catch).

**Three bands + verdicts:**
1. **VALUE = expensive-apply ∧ shallow-check** — Dorc's whole win. **Its size is the primary go/no-go.**
   Fat → build the engine; thin → thesis in trouble.
2. **JUST-RUN = cheap-apply (any check)** — probe is overhead; just run. **Subtract it out** — it inflates
   naive skip-rate and is not Dorc's value-add.
3. **HARD = expensive-apply ∧ deep-check** — unservable without M4 (lifted guards + oracle library +
   the deferred tracing tool turning deep→shallow). Fat here → **the oracle library is load-bearing,
   not optional.**

**Single most decision-relevant number: the anti-correlation ratio** — among *expensive* ops, the
shallow:deep check ratio. Mostly-deep → value band thin by nature, M4 is the only lever (Blow 3 confirmed).
Many-shallow → thesis healthy.

**Supporting (necessary, not sufficient):** the **un-analyzable rate** (leaves forced to
must-probe/can't-skip by external/non-det reads, no oracle, dynamic constructs — caps everything) and the
**cheap-mutation fraction** (how much headline skip-rate is illusory JUST-RUN).

## ALSO answer (one corpus pass should feed every corpus-sensitive perf decision)
From `076-performance-architecture.md` §6 `[corpus-sensitive]` + §7 + analysis-architecture:
- **(perf §6 #3 — context-sensitivity)** Does the abstract state stay **flat** in real scripts — guards
  are flat system-state predicates, not closures recombining variables across calling contexts (the
  k-CFA-paradox fault line)? AND would context-*insensitive* analysis actually lose meaningful skips, or
  is per-host/per-role context unnecessary? (Decides whether the EXPTIME cliff is even in play.)
- **(perf §6 #7 — cost vector)** Which cost dimensions actually occur (local-stat / local-exec / network /
  remote-rpc / shared-resource) and at what frequency? Do checks call out to networks/daemons/remote
  services (→ keep-under-guard) or stay local (→ hoist-and-batch)?
- **(perf §6 #9 — cross-host memoization)** How homogeneous are hosts/roles across a fleet — i.e. the
  equivalence-class size that sets the verdict-reuse rate? (May say "memoization not worth it.")
- **(perf §7 #3 — probe-vs-just-run)** The cheap-mutation fraction again, framed as: for how many ops is
  just-running idempotently cheaper than probe(stat+roundtrip)?
- **(perf §7 #4 — substrate memory)** Corpus size / working-set estimate → does a materialized
  (Datalog-style) fact base fit commodity RAM, or is IFDS/demand forced? (RSS-relevant.)
- **(analysis #2 — modularity)** Confirm the embarrassingly-modular **R roles × H hosts** structure
  (mostly-independent roles) that compositional summaries exploit.
- **(fuzzy-edge detector, DESIGN's niche)** Density of imperative-shoehorned-into-declarative signatures
  (`local-exec`/`null_resource`/`creates:`/`changed_when:`/k8s `Job`/`cloud-init runcmd`) = "this team is
  fighting their declarative tool" = prime Dorc candidates.

## Method
- **Static-corpus heuristics**: command-name → apply-cost-class and cost-vector; "a guard exists ∧ is a
  known-shallow predicate" → shallow-check. **State the heuristic and its error bars; claim no false
  precision.** Good enough to *size* bands, not to measure exactly.
- **Extend `shstats`** (cloned, `Vendor/colis-anr/shstats`, OCaml) for the two-axis tally rather than
  writing a corpus walker from scratch — it already does corpus-wide shell-AST statistics.
- **Ground-truth a sample later** via the calibration harness (container fixtures: run the op, observe the
  state delta) to validate the static heuristics on a subset. Not required this round; note it.

## Hard constraints (standing, non-negotiable)
- **No bulk download / no API hammering without explicit per-run leave.** `corpus-survey.sh` is dry-run by
  default; the user runs the materializing pass. Honor `gh` rate limits (preflight is built in).
- **Bootstrap from curated academic corpora** (Opdebeeck 15k Ansible PDG; GLITCH annotated oracles;
  Rahman labeled defects incl. idempotency) **before** any GitHub scraping — they come *with* labels.
- **Ask per-tool before installing anything.** Footprint < ~50 GB; full clones welcome (history aids "why").
- **Grade every source** (peer-reviewed/primary A; practitioner B; SEO/AI-slop rejected). Reproduce good
  human sources locally.
- **No external mutation**: read-only git only; no push; no DB/system changes. Collate, present, let the
  user execute.

## Deliverable
A measurements writeup (band sizes + the anti-correlation ratio + the also-answer findings, each with
error bars and method) that returns the go/no-go: **fat VALUE band → build the engine; thin VALUE but fat
HARD band → the oracle-library/tracing strategy *is* the project, engine secondary; high un-analyzable
rate → rework the thesis before any code.** Persist as a new `Research/notes/` entry; update the corpus
notes + manifest.

## Anti-mispigeonhole reminders (things that took repeating; don't regress)
- **Two soundnesses, never conflated** (`kFAIL`): probe-soundness (read-only pass never mutates;
  fail-safe = withhold) vs elision-soundness (never skip needed work; fail-safe = perform). Opposite
  fail-directions.
- **No metadata layer — it's all sh.** Findings reduce to proposed `if …; then` guards; there is no
  declared-effect/cost sidecar (Reading A: inference + shell structure, annotation held latent).
  <!-- /* superseded 2026-06-03: overstated. Reconciled to `kOOB` — *minimize* the out-of-band layer, don't deny it: an irreducible OOB floor (effect-class · provenance/leaf-id · cost-class · memo-key+freshness) is real; its size is the open `Q-INFER`. See `KNOBS.md` `kOOB` + `083-synthesis-and-spike-charter.md` §7. */ -->
- **Perf is cost-and-latency, not correctness** — every shortcut is elision-safe; tune toward speed in the
  safe direction. The spike measures *value headroom*, not a correctness gate.
- The engine is the point (DESIGN component 2); orchestrator/executor is to be **ceded/pluggable**; the
  oracle **library** is the existential network-effect risk. Weight findings accordingly.
