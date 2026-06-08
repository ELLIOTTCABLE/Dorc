# Knobs (design-tension registry)

Stable names for the **A-vs-B design-goal tensions** ("knobs") that recur across
Dorc's research and planning. Each knob is a single shared axis with two opposed
poles, where pursuing one pole *costs* the other. The purpose is *vocabulary*:
so multi-day, multi-agent research/planning/design can refer to the same tension
by the same slug instead of re-deriving it (badly, differently) in every
document.

This file is authoritative on *naming* (per me, the human). Synthesis notes
and plans should *reference* these slugs, not redefine the tensions. If a
document discovers a genuinely new tension, report it to the user for addition
here; if it discovers that two slugs are the same tension, report that similarly.

However, this is not *design*; don't mis-read content in here as advisory or
direction-setting. Prose is descriptive/identifying, not prescriptive/opining.

## How to read an entry
The `### kSLUG` in the header is canonical; re-use that term every time you recognize it.

First, `kSLUG-pole-a ↔ kSLUG-pole-b`: The axis and its two ends, each named to be unambiguous on its own.

- **Tension**: the two design-goals that pull apart (goal-served-by-pole-a *vs* goal-served-by-pole-b).
- **Status**:
  - `open` (a live choice),
  - `directional` (open but with a committed lean),
  - `mode` (resolution ceded to the user, intentionally, either through config, flags, or inference),
  - or `welded` (settled; do-not-relitigate — named only so we can still talk about it).
- **Owner**, who decides: `corpus` (the measurement spike), `user` (taste/values/runtime intent),
  `dominant-strategy` (prior-art-blessed, near-free), or a mix.
- **Lock-in**: how retrofit-hostile changes are down-the-road (`high` = decide
  the *shape* now even if you build later; `low` = reversible). See `kLOCKIN`
  (this is the meta-knob that tags all the others).

---

## Specification & knowledge-source — *where does per-command knowledge come from?*

### `kBURDEN`
Poles: `kBURDEN-we-infer ↔ kBURDEN-user-declares`

**Tension:** minimal user buy-in / invisibility / "magic" (DESIGN priorities 2 & 4) **vs** precision & soundness from explicit specification (priority 1). The deployer↔engineer audience gradient is this knob set per-human: a deployer sits towards `kBURDEN-we-infer`, an engineer who writes an oracle moves that one command towards `kBURDEN-user-declares`.
**Status:** open.
**Owner:** corpus (how-inferable real ops shell is) + user (designing the gradient).
**Lock-in:** med — the gradient must have no cliff (settled principle 5), so the *shape* matters early.

### `kOOB`
Poles: `kOOB-in-band ↔ kOOB-sidecar`

**Tension:** dogfooding / human-visibility / no-cliff / trivial off-ramp (everything is shell you read and run) **vs** engine expressiveness for what shell genuinely cannot carry (effect-class, provenance/leaf-id, cost-class, memo-key+freshness).
**Status:** directional — lean `kOOB-in-band`; minimize the sidecar. **Owner:** user (the value) + corpus (`Q-INFER` sizes the irreducible floor). **Lock-in:** med.
Entangled with `kBURDEN` (that's *how much* is specified; this is *what form*).
> *Clarification (human, 2026-06-03):* this knob's redline is **user-configuration form**, not metadata transport — out-of-band *metadata* (provenance/leaf-id, effect/cost-class, memo-key, network-transport framing) is fine; what is verboten (at least for now) is sidecar *configuration* — no YAML, no frontmatter, no pragma, no comment-parsing — all config is spelled in `sh` / library-code.
False friend of `kCOMMS` (Execution & modes): the same in-band/out-of-band axis but for Dorc's own transport, which carries no config redline.

### `kTYANNOT`
Poles: `kTYANNOT-inline ↔ kTYANNOT-eol-comment`

**Tension:** *iff* Dorc ends up needing a non-sh-native **type-annotation** at all (itself unsettled — `dq-kOOB`; see `Research/plans/17N` top paragraph), how is it spelled in the script? `kTYANNOT-inline` annotates a type directly on a command argument — ergonomic, intuitive, significant-meaning-in-place (DX-forward) **vs** `kTYANNOT-eol-comment` carries it on an end-of-line `# …` comment (shellcheck-style; verbose, forces pulling typed values out of argument-position). The forcing tension is the **off-ramp**: sh has no inline-comment form, so an inline annotation is *not inert* under stock `dash`/`bash` — it aborts (`local: :: bad variable name`) or silently corrupts (verified, `Research/plans/17O` F-OFFRAMP) — breaking `kLANG`'s "absolutely trivial" off-ramp and demanding a correctness-critical strip/transpile pass; an eol comment is inert under any shell (off-ramp-free) but **re-opens `kOOB`'s `no-comment-parsing` redline**. So *both poles sacrifice a "spelled in sh" principle* — inline sacrifices the off-ramp (`kLANG`), eol-comment sacrifices no-comment-config (`kOOB`); the knob is *which*.
**Status:** open, **gated by a prior open question** (`dq-kOOB`: whether a type-system exists at all). **Owner:** user (DX/ergonomics vs off-ramp purity). **Lock-in:** med (the spelling threads the parser + every typed oracle; the off-ramp guarantee is high-lock).
> Entangled with `kOOB` (the eol-comment pole's cost) and `kLANG` (the inline pole's cost). Source: `Research/plans/17N` top paragraph + `Research/plans/17O` F-OFFRAMP. *(Added 2026-06-08 with human auth; the synthesis docs reference this slug, they do not redefine it.)*

---

## The probe optimizer — *per-leaf economics of checking vs acting*

### `kPROBING`
Poles: `kPROBING-probe-first ↔ kPROBING-just-run`

**Tension:** avoid expensive/dangerous redundant *work* (check before acting) **vs** avoid redundant *checking* overhead (for a cheap idempotent op like `mkdir -p`, the probe's stat can cost more than just doing it). The apply-cost×check-depth banding (VALUE / JUST-RUN / HARD) lives on this axis.
**Status:** open — half decided-now, half runtime-dynamic. The per-leaf call is hard to tune and probably dynamic: this is where Dorc starts to resemble a query-planner and eventually wants Executor Smarts. The part *we* set is the meta-knob — **when** to graduate into Executor Smarts. **Owner:** corpus (sizes the bands) + runtime. **Lock-in:** low, but the decision-point must exist in the planner.

### `kFLATTEN`
Poles: `kFLATTEN-hoist ↔ kFLATTEN-maintain-cfg`

**Tension:** `kFLATTEN-hoist` lifts cheap independent checks into one flat parallel probe (desirable, but work) **vs** `kFLATTEN-maintain-cfg` keeps the 'apply'-phase control-flow in the shipped probe, leaving probe-checks under (probing-versions-of-) their original guards (cheap and safe — a local guard elides its expensive check).
**Status:** open; spike-responsive (`Q-COSTVEC`); plausibly low-value, and possibly near-free depending on the analysis-transformation architecture. **Owner:** corpus + cost-model. **Lock-in:** low.

---

## The analysis engine — *how hard does the static analysis think?*

### `kPRECISION`
Poles: `kPRECISION-precise ↔ kPRECISION-cheap`

**Tension:** fewer wasted probes + more apply-concurrency (precision unlocks parallelism) **vs** a fast, low-memory, maintainable engine. Safe to trade — cutting precision costs probes/runs, never correctness, while `kFAIL` holds.
**Status:** open. **Owner:** corpus + user (engine-depth is partly a learning/taste lever). **Lock-in:** low per-mechanism, except `kCONTEXT`.

### `kCONTEXT`
Poles: `kCONTEXT-sensitive ↔ kCONTEXT-insensitive`

**Tension:** precision on cross-call / per-host facts **vs** staying polynomial. A safety boundary, not a tuning dial: k-CFA (k≥1) is EXPTIME unless the abstract domain stays flat (k-CFA paradox; `Q-FLAT`).
**Status:** open, redline — default `kCONTEXT-insensitive`; add context only where flat-domain is confirmed. **Owner:** corpus. **Lock-in:** high (baking in global context-sensitivity is fatal).

### `kUNIT`
Poles: `kUNIT-fine ↔ kUNIT-coarse`

**Tension:** precise per-function skip + precise diff-recompute (fine) **vs** lower summary-composition overhead + fewer cross-unit deps to track (coarse). (Terraform's state-split tension on the analysis unit — but Dorc *derives* cross-unit deps, so finer costs less than Terraform's manual wiring.)
**Status:** open. **Owner:** corpus (`Q-MODULARITY`). **Lock-in:** med.

### `kFACTS`
Poles: `kFACTS-materialized ↔ kFACTS-on-demand`

**Tension:** extensibility + provenance + query-speed (Datalog/Soufflé materializes all facts) **vs** low memory (IFDS/demand computes only what's queried — the memory wall). This *is* the engine-substrate decision.
**Status:** open. **Owner:** corpus (`Q-WORKINGSET` / RSS). **Lock-in:** high (substrate is expensive to swap; a hybrid — demand core + bounded relational layer — is one resolution).

---

## State, reuse & freshness

### `kSTATE`
Poles: `kSTATE-persist ↔ kSTATE-recompute`

**Tension:** persisted state — a verdict cache, cross-host memoization, any central record — buys speed and reuse **vs** stateless recompute from the one known ground truth (host reality; on-disk code) buys correctness and dodges staleness/contention.
**Status:** open, **and genuinely unsettled.** Prior rounds treated central state as a near-killer (Terraform contention / stale / secrets-in-state); the build-systems prior-art offers the stateless counter-model (rust-analyzer: no persisted cache, recompute from on-disk truth). Neither has been interrogated; resolution may end up `mode` (floated to the user via config or inference). **Owner:** user + corpus (`Q-HOMOGENEITY` sizes the reuse upside). **Lock-in:** high to *reserve* (the verdict shape / content-key), low to *use*.

---

## Execution & modes

### `kELISION`
Poles: `kELISION-scoped ↔ kELISION-full`

**Tension:** elide genuine checks / expressed desired-state *for now, at user request* outside a declared scope — hot-loop speed, accepting staleness (`dorc some-smart-ish-diffing-update-from-git`) **vs** elide nothing un-proven — completeness / no drift (`dorc some-reconcile-all-state-completely`). *(Elision = deliberately not-checking-right-this-second something unknown/possibly-bad; distinct from skipping-because-known-good, which is just applying-for-free.)*
**Status:** mode (user picks via update/reconcile; changes elision *scope*, never elision *soundness*). **Owner:** user (runtime). **Lock-in:** low.

### `kOBJECTIVE`
Poles: `kOBJECTIVE-latency ↔ kOBJECTIVE-throughput`

**Tension:** minimize time-to-first-action (deployer "server's on fire, NOW") **vs** maximize whole-fleet makespan (engineer's full reconcile) — different objective functions, hence different optimizer defaults.
**Status:** open (derive from mode + a coarse urgency intent). **Owner:** user-intent. Coupled to `kELISION`. **Lock-in:** low.

### `kFIDELITY`
Poles: `kFIDELITY-optimized ↔ kFIDELITY-faithful`

**Tension:** performance (the minimized, batched, opaque production probe) **vs** debuggability / attribution (`--faithful`: one-leaf-one-exec, 1:1 source mapping — the seam the realtime-output requirement *and* the future tracer both need).
**Status:** open — both ship (`kFIDELITY-optimized` default, `kFIDELITY-faithful` reserved). **Owner:** dominant-strategy. **Lock-in:** high — the leaf-execution seam must be wrappable + provenance-preserving from day 1.
> *Round-11 (error/provenance prior-art, 2026-06-03):* the "provenance-preserving" clause is heavier than a 1:1 source map — the faithful seam must preserve an N-tier, per-host-forking, host-qualified multi-locator derivation DAG (loc-host / loc-user-src / loc-probe / loc-surface); see `notes/110` (+ forthcoming `plans/` synthesis).

### `kCOMMS`
Poles: `kCOMMS-executor-OOB ↔ kCOMMS-transpilation-inband`

The form of Dorc's own controller↔host metadata: a bootstrapped probe-executor reporting out-of-band (Ansible-python-style), or transpiled markers in-band in a real-shell stream. A false friend of `kOOB` (same in-band/out-of-band axis, but Dorc's implementation I/O, not user-written config); open, and either pole rides the one `kFIDELITY` session seam.
> *Round-14 (transport prior-art, 2026-06-04):* the two poles conflate two *orthogonal* axes — in-band↔OOB and executor↔pure-sh. The directional lean is the **executorless-OOB** quadrant the poles omit: tool I/O full-fidelity on native SSH channels; Dorc-signalling out-of-band, split by size/urgency — short gating verdicts (`(verdict, content-key, freshness)`) on a shared atomic fast-lane, large diagnostics in per-leaf files demuxed by filename. The executor pole is re-pinned to {no-writable-fs, hard backpressure}, *not* concurrency/attribution (pure-sh covers those). Security is structural (signalling never shares a lane with freeform). Residual: writable-fs on stripped/Windows targets. See `plans/142`.

### `kSCHEDULE`
Poles: `kSCHEDULE-wide ↔ kSCHEDULE-ordered`

**Tension:** raw parallelism width **vs** schedule quality (critical-path-first; resource-aware). The Graham anomaly: more workers can *increase* makespan, so the schedule matters more than the width.
**Status:** open, org-scale → defer-but-reserve. **Owner:** dominant-strategy (list-scheduling heuristics). **Lock-in:** low.

---

## The meta-knob

### `kLOCKIN`
Poles: `kLOCKIN-commit ↔ kLOCKIN-reversible`

**Tension:** ship velocity + design coherence (decide it, build it) **vs** avoid premature foreclosure (reserve a seam, keep the door open). The organizing lens: every other knob carries a "lock-in tag" for how costly getting-it-wrong-later is.
**Status:** open (per-decision). **Owner:** user + the synthesis.

---

## Platform reach — *how far do we bend for odd hosts/targets?*

### `kTPLATFORMS`
Poles: `kTPLATFORMS-mainstream ↔ kTPLATFORMS-wide`

**Tension:** bend engineering toward mainstream Linux/macOS targets (fewer features broken for odd hosts) **vs** reach the long-tail — Windows, ARM, RasPi, RISC-V, Synology/busybox — the heterogeneous fleets the initial userbase runs.
**Status:** open, lean `kTPLATFORMS-wide`, but gated by `kLANG`: "wide" is tractable only as "any target that can already evaluate POSIX sh" (an sh-precondition + per-platform oracles); Windows-without-sh forces the transpile/foreign-input options `kLANG` welds out. **Owner:** user + corpus. **Lock-in:** med (entangled with `kLANG` + oracle-library portability).

### `kWINLOCAL`
Poles: `kWINLOCAL-nix-only-controller ↔ kWINLOCAL-windows-supported`

**Tension:** *nix-only orchestrator host (every push-tool analog does this — Ansible/Salt forbid a Windows controller; fork/local-exec UNIX-isms stay free) **vs** a native-Windows controller for the Windows-daily-driver homelabber. **Status:** directional, mild-lean `kWINLOCAL-nix-only-controller` (WSL is the prior-art escape hatch; the analyzer is platform-free text, so low lock-in). **Owner:** user. **Lock-in:** low.

---

## Welded — settled; do not relitigate (named only so we can refer to them)

### `kLANG`
Poles: `kLANG-sh-is-the-product ↔ kLANG-pluggable-language`

**Tension:** sh as the *sole authored/analyzed language* — one analyzer, one parser, one oracle-contract idiom, and a probe-compiler whose ceiling (what a sanitized probe can determine about a host, the network, cross-host truth) is shaped by sh's semantics **vs** a 2-to-N *input*-language backend (e.g. PowerShell) for broader native reach.
**Welded to `kLANG-sh-is-the-product`**: a second *input* language is not a backend but a second product — new analyzer/parser/language-design/oracle-library; the only shared remnant is the name + the thin pluggable orchestrator — and it is not separable, since sh's shape governs what the subset-probes can prove. NB: binds the *authored* language only; a target running native *commands* inside sh control-flow does not breach it (that variance lives in oracles — see `kTPLATFORMS`). **Owner:** welded. **Lock-in:** absolute (day-one).

### `kFAIL`
Poles: `kFAIL-withhold ↔ kFAIL-perform`

**Tension:** probe-soundness — never mutate in a read-only pass (`kFAIL-withhold`) **vs** elision-soundness — never skip a needed mutation (`kFAIL-perform`).
**Welded, phase-keyed**: the probe phase fails `kFAIL-withhold`, the apply phase `kFAIL-perform` — opposite safe directions, not a dial. The one thing never traded for performance. **Owner:** welded. **Lock-in:** absolute.

### `kVOLATILES`
Poles: `kVOLATILES-exclude ↔ kVOLATILES-model`

**Tension:** kVOLATILES-exclude for a sound skip-cache (demand/correctness-precondition-contract the canonicalization/striping of volatile state — "hermetic oracles") **vs** kVOLATILES-model to achieve fidelity to nondeterministic reality.
**Welded to `kVOLATILES-exclude`**: non-determinism breaks any sound skip system (the build-systems world reached the identical conclusion — hermeticity is a *precondition* for caching, not a Dorc shortcut).
**Owner:** welded (settled principle 3).

### `kVERIFY`
Poles: `kVERIFY-calibrate ↔ kVERIFY-prove`

**Tension:** engineering-grade confidence that ships (differential + property + container-fixture tests — the calibration harness) **vs** mathematical soundness (proof assistant).
**Welded to `kVERIFY-calibrate`**: "TypeScript, not Coq" — end-to-end proof is unattainable (the un-provable parser/translation gates everything) and serves the disclaimed 5%; even CoLiS fell back to differential testing. **Owner:** welded.

### `kDEPS`
Poles: `kDEPS-declare-world ↔ kDEPS-accept-partial`

**Tension:** total upfront dependency specification (Nix/Ansible/Terraform — high buy-in, complete knowledge) **vs** accepting that dependency knowledge is non-total and filling it best-effort. *(static-derive and runtime-trace both serve `kDEPS-accept-partial` — complementary means, not opposed poles; you want both, trace as a backstop to derive.)*
**Status:** welded → `kDEPS-accept-partial` (the anti-declarative thesis; DESIGN "rejected: declarative resource graph"). **Owner:** welded.
The *open* question is not this axis but the **investment split** within it — how much `static-derive` carries vs how much the oracle-library + runtime-trace backstop must (the `Q-BAND`/`Q-ANTICORR` spike → `effort-allocation`).

### `kAGENTLESS`
Poles: `kAGENTLESS-push ↔ kAGENTLESS-host-autonomy`

**Tension:** central push authority — one operator node drives the fleet, no per-host daemon to own or secure (DESIGN "push, not pull"; the ergonomic + no-listening-daemon win) **vs** host autonomy — each host applies only what it fetches and verifies itself (CFEngine's "no one except root@localhost can force cfengine to do anything"), which *bounds blast-radius* but reintroduces a pull/agent surface.
**Welded to `kAGENTLESS-push`** (ergonomic, per DESIGN) — named only to keep the *security cost* in view: push concentrates the crown-jewel in the operator workstation (fleet-wide SSH keys — SaltStack's listening-master RCE blast-radius is relocated, not removed), and real m→n→o bastion-hopping reintroduces multi-hop trust. Push is *ergonomic, not a security claim*. **Owner:** welded. **Lock-in:** high (architectural).

---

## Not a knob (a prioritization principle, parked here so it isn't mistaken for one)
**`effort-allocation`** — engine-core vs oracle-long-tail vs analyses-on-top. *Not* an A-vs-B design tension; a resourcing call. Lean (user's): highest per-day marginal value is the **core extensible engine** + **analyses-on-top that promote correctness/UX/perf properties**, even though the oracle *corpus* has the larger total eventual reach (community-grown, long-tail). Bootstrap only the ~40-50 highest-frequency oracles; let the community grow the tail.
