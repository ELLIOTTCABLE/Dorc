# 150 — adversarial premise-review of the corpus: phase 1 (per-round)

> **Status (2026-06-04): round-15 artifact — phase-1 stamp (per-round adversaries).**
> An adversarial review-round. 14 clean-context "disowned-and-inverted" subagents (one
> per research round R1–R14, incl. the in-flight platform-compat `13x` and transport
> `14x` worktrees) were each tasked per the `adversarial-crosscheck` SKILL to surface the
> **load-bearing premises that break**. Framing handed to each: *"an org weighing whether
> to fund this; our best move is killing it to save resources."*
>
> **This is not truth-finding.** Adversarial prompting widens coverage; it does not
> de-bias. The trustworthy signal is **convergence across independent agents**; lone
> findings are **suspect-until-checked**; and the agents' Dorc-hostility is as much a bias
> as the corpus's Dorc-optimism. Notably, *no agent found a clean feasibility kill-shot* —
> several broke character and refused the kill-stance where unearned.
>
> **Human steer (mid-round, 2026-06-04):** go/no-go is YOLO-welded-GO, so the
> **self-killing / value-band** findings (`M1`/`M2` below) are **de-prioritized** —
> recorded for completeness, not actioned. The live target is **feasibility: "plans
> meta-poor in ways that will waste *engineering effort*"** — rework, wrong-substrate,
> mid-build walls, day-1 commitments that conflict, a contract built-toward that can't be
> expressed, a test needed-but-unscoped. Phase 2 (4 cross-cuts) lands in `151`+.
>
> Prose is AI-generated; per `README.md` trust the human-written root docs over it. The
> per-round agents' raw reports are not reproduced here — this is the lifted synthesis.

## Findings — FEASIBILITY / effort-waste (the live target; most-attended first)

- **fM3-ACCRETION — the corpus is accreting faster than it reconciles; later rounds
  silently overturn earlier "welded, day-1" decisions.** Three *independent* per-round
  agents each found a cross-round contradiction the corpus never flags. This is the
  single most effort-relevant pattern: the "decide-now / retrofit-hostile" list is
  internally inconsistent, so building to it means building to conflicting specs.
  - `fM3a` (R11 vs R5): `plans/111` bills rich provenance as recoverable from the
    *optimized* probe stream ("cost = marker bytes"), but `plans/077:20` already states the
    optimizer **"destroys the 1:1 leaf↔execution mapping … can't be attributed back to one
    source line."** ⇒ rich provenance is welded to the **slow `--faithful`** probe; "fast
    probe ∧ rich attribution" is not available. The round implies it is.
  - `fM3b` (R12): the "DST is cheap" verdict rests on **two mutually-exclusive** cost-dodges
    — "async-native dev ⇒ ~free" *and* "state-machine architecture sidesteps the
    transitive-dep wall." The cited state-machine sidestep (Polar Signals) works *precisely
    because it is **not** async* ("no `async` keyword … no runaway futures"). You can bank
    one, not both. Banking `L0` therefore forces an **irreversible async-vs-state-machine
    kernel decision now**, the opposite of the round's "reserve-it-decide-later" framing.
  - `fM3c` (R13): the transport note leans toward an **executor** that overturns the welded
    `kAGENTLESS` *and* `plans/111`'s `dac-C` "dumb-host" provenance carrier — and the note's
    own logic (live ∧ concurrent ⇒ executor; the probe is stated to be exactly both) implies
    "agentless ∧ dumb-host ∧ live-concurrent-probe" are **mutually inconsistent.** Fixable
    (scope the executor to the probe phase; re-grade `kAGENTLESS` to "no *persistent*
    daemon"), but currently walked-into unacknowledged.

- **fN-NOTATION — the soundness-critical contract has no spelling yet and may be
  unsatisfiable under its own constraint-conjunction** (R9, sharpened). The `q-floor`
  "≥1 MUST-grade relational anchor per kind" (`plans/099` §4) is what licenses *any* sound
  skip. To work it must be simultaneously: (a) pure runnable POSIX-sh, (b) a behavioural
  no-op (off-ramp), (c) machine-recognizable as a kind-anchor, (d) **cross-oracle coherent**
  (a third party declaring `neon ≡ ubuntu` to package-oracles that don't know it exists =
  type-class instance coherence), with `kOOB` forbidding the sidecar escape. The corpus
  concedes it "does not exist yet" (`090` §0.5). **Effort risk: you can build the whole
  analyzer toward an anchor notation that turns out unspellable** → everything degrades to
  ⊤/just-run, or the off-ramp dies (it becomes a DSL). *This is the highest-leverage
  unproven feasibility item.* The cheapest de-risk is a strawman, owed before engine work
  (→ phase-2 `X4`, the Xpbd worktree, attempts exactly this).

- **fN-ANALYZABILITY — the analyzer is being sized against a mis-measured substrate** (R1+R2,
  converged). "Shell parses 99.9%" is a *parse* fact doing illegitimate work for an
  *analyzability* claim; the corpus's own source reports translation success **77%** and
  scenario-completion **40%/12%** ([A-colis-installation-scenarios-tacas-2020]), failing on
  **common** features (parameter-expansion, globs). The real ⊤-surface for the IFDS
  finite-fact substrate is **dynamic *arguments* + command-substitution** — and
  command-substitution is the **#2 most-common feature, above `if`** ([A-bash-in-the-wild-tosem-2022]).
  The team's ⊤-instrument measures the wrong unit (dynamic command *names* + `eval`, **not**
  dynamic *arguments* or `source "$x"`/reads), and `080` admits the number is "a floor."
  **Effort risk: choosing/optimizing the engine substrate (IFDS-vs-Datalog, `kFACTS`,
  recency) on a ⊤-rate that's understated by the vector that actually bites.**

- **fN-SILENT — the subtlest correctness-trap is named in prose but absent from the actual
  test harness** (R1+R9+R7-8, converged). A script that *parses* but whose CFG silently
  under-models `set -e`/conditional-`trap`/`pipefail`/redirection is **accepted and yields a
  wrong skip — "an elision-soundness bug that never announces itself"** (`087` §3c). The
  stated fix ("acceptance gates on modeling-completeness, not parse-success") **collides
  head-on** with the "99% of scripts just parse / gradual-enhancement" pitch (accept-rate
  governed by the *hardest* construct in a script, not the easiest). And the concrete spike
  harness (`088`: cold/converged/one-line-change fixtures) **does not include** the dash/bash
  `set -e`/`trap` differential `087` §3c said was the required teeth. Bash-specific; most
  real ops is bash. **Effort risk: building the analyzer + harness without the one
  differential test that guards the only-never-trade property (`kFAIL`).**

- **fN-PROBEPREDICT — the perf architecture optimizes a denominator it declines to measure,
  and routes around its own refutation** (R4). N·H→H, cross-host memoization, and skip-rate
  are all denominated in *probe-decidable leaves*; but inter-resource guard dependency (the
  canonical `install X → only_if X-present → configure X` idiom) makes a large fraction of
  leaves **un-probe-decidable** (decidable only *after* an upstream same-run mutation). The
  graded-A first-party refutation — Chef "why-run considered harmful" ([A-chef-whyrun-harmful-2018],
  *"no-op modes … can only observe resources in isolation"*) — sits in Dorc's corpus but is
  **quarantined in the security track**, demoted to a one-line "soundness rider," never
  allowed to touch the perf math. **Effort risk: building memoization/scheduling machinery
  whose payoff is gated by an unmeasured (plausibly small) fraction.**

- **fMVP-ENTANGLE — the value-prop plausibly needs the analyzer *and* a non-naive executor
  *and* a bootstrap-oracle set *and* the transport, together** (R4+R12+R13+R7-8). The perf
  "auto-tune what they hand-tune" pitch lives entirely in the executor the author calls
  "shitty"/cedeable; the Graham anomaly means a **naive** scheduler can be *worse than
  serial* (not a safe partial); DST/transport want irreversible day-1 kernel commitments.
  This is the time-to-first-value question and it's live: several rounds independently emit
  "must exist day-1 / retrofit-hostile" items that may collectively *be* the mega-build.
  (→ phase-2 `X2` synthesizes the dependency graph.)

## Findings — per-round `THE-ONE` (compressed; feasibility-framed)

- `R1 foundations` — feasibility *holds* (parseable, engine-buildable, no-Coq correct); it
  just proves the *necessary*, not the *sufficient*, condition. Sharp residual = `fN-ANALYZABILITY`.
- `R2 analysis-arch` — IFDS finite-fact substrate vs the dominant feature (command-subst /
  dynamic args); `IFDS ≡ Datalog "one substrate for free"` overstates a one-way encoding of
  the *distributive* layer only; recency imported from TAJS is a mis-transfer (allocation-site
  multiplicity Dorc lacks; the right tool is CQual uniqueness-gated update, cited elsewhere).
- `R3 userbase` — evidence base = **enterprise CS-pros** (authors disclaim representativeness);
  product target = **homelabbers**; never reconciled. Several user-study cites read *against*
  source (AWX-pain→cede inverts; "drop to a shell script" is a debugging workaround the paper
  files under *Debugging*, pointing to Pulumi/Nix — not shell-as-substrate).
- `R4 performance` — `fN-PROBEPREDICT` (above).
- `R5 recovery` — the tier that *ships* (unprivileged seccomp) is **blind to the
  daemon-mediated case that motivates the whole component**; the value-recovering tier
  (privileged eBPF) is deferred/unbuilt ⇒ "relabel a hole as deferred." Seccomp net itself
  bypassable (socketcall/x32/inherited-fd), corroborated by a *live* event (Docker
  CVE-2026-31431, see citations). **Fabricated load-bearing quote** (→ `fM4`).
- `R6 corpus-method` — the validation method can't move the denominator from corpus→world
  and has **no pre-committed fat/thin threshold**; the container-fixture "ground-truth"
  **systematically under-samples the deep/nondeterministic tail** (`kVOLATILES` can't be
  reproduced hermetically) ⇒ biased toward GO. *(Largely go/no-go; de-prioritized.)*
- `R7-8 kill-rigor` — *meta*: the self-critique is **mostly rigorous** (A-CEILING/HAZARD/INERT/FLAT
  dissolutions are sound; A-VALUE concession is honest) — the one motivated move is the
  go/no-go substitution (`M1`, de-prioritized). Feasibility residue = `fN-SILENT` filed in
  TODO-research-phase while the harness ships without its test.
- `R9 state-tracking` — the correctness *theory* is the **good part** (impossibility-ceilings
  used correctly; relational/MUST-MAY coherent). The feasibility hole is `fN-NOTATION` (the
  contract has no spelling) + specimens are wrong-quadrant local glue (eval/heredoc-heavy).
- `R10 security` — the threat model **omits the managed host as an adversary** while probe
  results flow *back* into the controller's elision decisions ⇒ a hostile host can forge
  convergence verdicts to **silently suppress a security-relevant apply** (`kFAIL-perform`'s
  exact failure direction). SHA-pin prescribed as the supply-chain fix by a source
  ([B-nesbitt-quacks-package-manager-2026]) that says SHA-pin doesn't cover the transitive
  case. Host-blast-radius mitigations (SSH CA, hardware tokens) **evaporate for the homelab
  audience**.
- `R11 error/provenance` — `fM3a` (above). Held: the `result × [diagnostics]` data-shape
  thesis is the round's strongest, genuinely-corroborated contribution.
- `R12 cross-network DST` — `fM3b` + the verdict-correctness tests are **circular** (the
  simulator that *produces* the truncation is the oracle for *where* it truncated → validates
  parser self-consistency, **not** that marker boundaries match real-host leaf-execution
  boundaries — which is the *differentiating* correctness claim, and the part DST can least
  validate). The differential-test fallback ("our evaluator ≡ dash") is asserted "easy,"
  unanalyzed, and is a research-grade problem.
- `R13 transport` — `fM3c` (above). FIFO `PIPE_BUF` atomicity holds only for short status
  lines, not freeform/binary tool output; note conflates FIFO atomicity with regular-file
  `O_APPEND` (a latent concurrency-corruption bug). Native-SSH (russh) for channels vs
  OpenSSH-config for security (`plans/102` ProxyJump/host-keys) = two transports, seam unowned.
- `R14 platform` — Dorc is **stricter than Ansible on the *authored* language** (rejects the
  bashisms the audience writes; Ansible just ships your bash to the target's bash). The
  sh-precondition "fixes the wrong half" (target evaluator, not authored input). The
  Ansible-Python analogy breaks at the bootstrap leg (`raw` rides the target's *native*
  shell; `kLANG` forbids that). busybox-w32's bogus perms/path model may corrupt the
  **probe's derived truth**, not just oracle coverage ⇒ threatens `kFAIL-withhold` on tier-B.

## Findings — go/no-go meta (DE-PRIORITIZED per steer; recorded only)

- `M1-UNFALSIFIABLE` (R6 + R7-8 converged) — the population value-band measurement (which
  *could* return NO-GO) was retired and **swapped for a self-confirming dogfood existence-proof**
  on the author's own ops (the exact population `contrast-not-compound` banned); kill-triggers
  carry escape hatches; **no pre-committed numeric threshold.** "Build-to-kill cannot kill."
  *Not actioned (YOLO-GO), but worth one human glance: it means the central bet will never be
  forced to resolve, which is a strategy choice, not a bug.*
- `M2-VALUE` (backdrop of all of phase A) — the `expensive∧shallow∧analyzable` band is
  unmeasured; supporting signals lean negative (homelab guard-rate 7%; idempotency
  module-native not guard-native; specimens wrong-quadrant). *De-prioritized.*

## What HELD UP (do not waste effort re-attacking)

- **Impossibility-ceilings used correctly** (Rice / Ramalingam / frame-problem / IFDS
  finite+distributive floor) — verified against primary sources; the "refuse the semantic
  question, recognize-syntax + probe + delegate" posture is sound. The `kFAIL` phase-keyed
  weld is coherent and is genuinely *why perf is safe to chase*.
- **No soundness proof is claimed** (`kVERIFY-calibrate`); the corpus concedes it's capped
  and is broadly self-correcting/hedged. "They think they proved it sound" is a strawman.
- **Analyzer scalability** (cubic-floor / IFDS h-sparse O(ED) / Facebook-Infer
  compositional-incremental / memory-wall instinct) is the most solid part of the perf round.
- **The fan-out/network-ceiling engineering** (MaxStartups/MaxSessions, async-not-fork,
  ControlMaster, thundering-herd, Salt-syndic aggregation lesson) is correct and well-sourced.
- **`kLANG` "a second input language = a new product, not a backend"** is sound; controller-
  *nix-only is dominant-strategy-correct (Ansible/Salt/pyinfra all agree).
- **The `result × [diagnostics]` / error-node-kind data-shape** (R11) is well-corroborated.

## Citation-integrity flags (feasibility-relevant: building on a bad premise wastes effort)

`fM4` — load-bearing citation faults the 2026-06-04 self-audit structurally missed (it
deprioritized interpretive/analogical claims + several source tiers, exactly where these sit):
- **Fabricated quote (sharpest):** `plans/deferred/078:9` attributes "the mechanism finding
  that **shapes everything**" to a verbatim rattle README quote (*"if `fsatrace` runs a binary
  that goes through a server process not spawned by the build system, it won't be visible"*)
  that **does not exist** in rattle's or fsatrace's README ([A-spall-mitchell-rattle-perfect-dependencies-2020]
  discusses Go-static/SIP coverage gaps, a *different* axis). The *conclusion* is independently
  true; the *evidentiary anchor* is invented and dressed with "confirmed verbatim."
- R3: the Ansible user-study is read 180° against its authors' interpretation in ≥2 places.
- R2: k-CFA-paradox cite is a term-equivocation (flat *environment* ≠ flat *fact-map*;
  polynomiality bought *by* destroying the precision the "dial" promised) ([A-might-smaragdakis-vanhorn-kcfa-paradox-pldi-2010]).
- R10: SHA-pin prescribed by a source that says SHA-pin is insufficient for transitive deps.
- R11: PlanBouquet "provable bounds" cited for an architecture whose discovery mechanism
  (re-execute the query) is exactly what `kFAIL-withhold` forbids — bound non-transferable.
- R7-8: `do-2` ships a v1 (git-diff elision) that contradicts its own cited source (`075`:
  "the git-diff should *never* be the skip authority").

## Human reframes (post-phase-1, 2026-06-04)

- **For the phase-2 conclusion (do NOT lose):** frontload `fM3-ACCRETION` and `fM4`
  (citation-integrity) with **verbatim adversarial-agent quotations** — the human wants
  fidelity + bandwidth on these two specifically.
- **The oracle concern is a *triangle*, not a kill (human correction).** The docs
  over-weighted the oracle-supply *kill*. Reality: one vertex is "best-effort engine, low
  value," where *both* community/network-effect *and* admin author-skill/effort are low (the
  valley); but *either* lever alone lifts Dorc out toward runtime/perf/correctness/sanity
  value. ⇒ a productive lens: **Dorc as an engine that bootstraps homelabbers out of bad
  scripting habits and *into* oracle-authoring.** Consequence for this round: the oracle
  *cold-start / ecosystem-value* question is **de-prioritized** (go/no-go-adjacent, now
  triangle-addressed); the *live* oracle concern is purely **mechanism-feasibility** —
  `fN-NOTATION` (is the contract spellable?) + triple-authorability. (Human will lean on the
  pipeline framing in `DESIGN`; phase-2 `X3` is retuned to mechanism-only accordingly.)

## Method & caveats

- **Process:** clean-context, "disowned-and-inverted" framing per `adversarial-crosscheck`;
  3-agent calibration batch (R1/R4/R9) → tuned (added the converged value-band to OOS so
  later agents found fresh territory) → 2 fan-out batches. Each agent pointed at its round's
  synthesis `plan(s)` + notes; allowed bounded external/`sources/`/`Vendor/` checks.
- **Trust model:** convergent findings (`fM3`, `fN-ANALYZABILITY`, `fN-SILENT`, `M1`) are the
  signal; singletons (host-trust-boundary R10, circular-DST-validation R12, busybox-corrupts-
  probe R14) are sharp but **suspect-until-checked**. The arbiter (me) is Dorc-curious; the
  agents were paid to be hostile — neither is truth.
- **`N-REPO` (checked, benign):** `Vendor/colis-anr/` holds only `colis-batch`+`morbig`, but
  `clone-vendor.sh` rebuilds Vendor/ on-demand and the techniques live in the cited papers;
  not a real gap.

## Citations

Corpus documents referenced (by path, this tree): `plans/021`, `041`, `055`, `063`, `064`,
`076`, `077`, `deferred/078`, `083`, `086`, `088`, `099`, `09A`, `101`, `102`, `111`, `121`,
`128`; `notes/075`, `080`, `081`, `087`, `090`, `093`, `131`–`133`, `140`–`142`. Human-root:
`DESIGN.md`, `KNOBS.md`, `README.md`, `TODO.md`.

Graded sources the findings turn on (already in `sources.json`):

> [A-chef-whyrun-harmful-2018] (relevance: +1:SURE)
> first-party refutation of predict-without-applying; load-bearing for `fN-PROBEPREDICT`.

> [A-colis-installation-scenarios-tacas-2020] (relevance: +1:SURE)
> the 77% translation / 40%+12% scenario figures under-quoted by the foundations docs; `fN-ANALYZABILITY`.

> [A-bash-in-the-wild-tosem-2022] (relevance: +1:SURE)
> command-substitution as the #2 feature (above `if`) — the real ⊤-surface; `fN-ANALYZABILITY`.

> [A-might-smaragdakis-vanhorn-kcfa-paradox-pldi-2010] (relevance: -0:SUSPECT)
> the k-CFA-paradox cite mis-applied (flat-env ≠ flat-fact-map); `fM4`.

> [A-spall-mitchell-rattle-perfect-dependencies-2020] (relevance: +1:SURE)
> the rattle paper whose README is *mis-quoted* in `078:9` (the fabricated anchor); `fM4`.

> [B-nesbitt-quacks-package-manager-2026] (relevance: -0:SUSPECT)
> "transitive execution ⇒ package manager"; says SHA-pin doesn't cover transitive deps — the R10 mis-prescription; `fM4`.

External corroboration surfaced by a subagent (UNREGISTERED this turn — second-hand read;
register via `new-source.sh` if it becomes load-bearing): **Docker CVE-2026-31431 "Copy
Fail"** — Docker tried to block a syscall family via seccomp, *could not* (pointer-behind-
socketcall), and shipped an LSM fallback; independently corroborates the R5 seccomp-bypass
finding with a weeks-old production event. `docker.com/blog/mitigating-cve-2026-31431-copy-fail-in-docker-engine/`,
`github.com/moby/moby/pull/52537`.
