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
It is also a quickref, not a changelog: entries describe the *current* state of
each tension; history lives in the cited `Research/` docs.

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

**Tension:** minimal user buy-in / invisibility / "magic" (DESIGN priorities 2 & 4) **vs** precision & soundness from explicit specification (priority 1). The deployer↔engineer audience gradient is this knob set per-human: a deployer sits towards `kBURDEN-we-infer`, an engineer who writes an oracle moves that one command towards `kBURDEN-user-declares`. The gradient's authored surface is the two optional function-families (per-TOOL `predict`/`is_converged`/`touches`; per-KIND `resolve`/`reaches`), each silence degrading to a named floor, ratified monotonic — every added member buys value, none removes prior value (rul24-threefunc-monotonic; `Research/notes/24A` §1b, `24G` §2) — and the deployer end stays free (the admin's own hand-written guard is first-class lifted material).
**Status:** open.
**Owner:** corpus (how-inferable real ops shell is) + user (designing the gradient).
**Lock-in:** med — the gradient must have no cliff (settled principle 5), so the *shape* matters early.

### `kOOB`
Poles: `kOOB-in-band ↔ kOOB-sidecar`

**Tension:** dogfooding / human-visibility / no-cliff / trivial off-ramp (everything is shell you read and run) **vs** engine expressiveness for what shell genuinely cannot carry (effect-class, provenance/leaf-id, cost-class, memo-key+freshness).
**Status:** directional — lean `kOOB-in-band`; minimize the sidecar. In practice all authored knowledge rides sh function-bodies + inline binds + trailing marks (strip-only; effects derive from the bodies), while the sanctioned OOB *metadata* lanes (the `UNK`/refusal report; the probe-readback lanes) carry no configuration. **Owner:** user (the value) + corpus (`Q-INFER` sizes the irreducible floor). **Lock-in:** med.
Entangled with `kBURDEN` (that's *how much* is specified; this is *what form*).
> *Redline (human):* the redline is **user-configuration form**, not metadata transport — out-of-band *metadata* (provenance/leaf-id, effect/cost-class, memo-key, network-transport framing) is fine; what is verboten (at least for now) is sidecar *configuration* — no YAML, no frontmatter, no pragma, no comment-parsing — all config is spelled in `sh` / library-code.
> *Sanctioned exception:* exactly ONE comment-parse exists — the dialect-version marker (`# dorc-lang/v0.1`, first ~10 lines), a closed set of one; it gates syntax only, never `__role` name-recognition (a permanent, unversionable surface). Eol-comment annotations stay rejected; a second comment-parse takes a fresh human ruling. (rul24M-version-comment, `Research/notes/24M` §1.)
False friend of `kCOMMS` (Execution & modes): the same in-band/out-of-band axis but for Dorc's own transport, which carries no config redline.

### `kTYANNOT`
Poles: `kTYANNOT-inline ↔ kTYANNOT-eol-comment`

**Tension:** how a non-sh-native **type-annotation** is spelled in the script. `kTYANNOT-inline` annotates directly on a command argument — ergonomic, intuitive, significant-meaning-in-place — but is *not inert* under stock shells (aborts or silently corrupts; verified, `Research/plans/17O` F-OFFRAMP), so it demands a correctness-critical strip pass, taxing `kLANG`'s "absolutely trivial" off-ramp. `kTYANNOT-eol-comment` carries it on an end-of-line `# …` comment — inert under any shell (off-ramp-free) but a breach of `kOOB`'s no-comment-parsing redline, and it forces typed values out of argument-position. *Both poles sacrifice a "spelled in sh" principle*; the knob is *which*.
**Status:** directional — **de-facto `kTYANNOT-inline`; the formal weld is human-reserved.** The upstream gate (`dq-kOOB`: whether a type-surface exists at all) resolved by construction: the inline dialect (bare `__role`-named functions, inline binds, trailing marks) is stamped and implemented, with the off-ramp cost paid by the strip pass (bare marks delete whole-statement; the author's last substantive command stays the last status-affecting statement). Two containments cheapen the trade: annotations are **marker-gated per-file** (an unmarked book is plain sh; a marked file — including a book hosting oracles — erases to plain sh via `dorc strip`), and function names are bare munged POSIX NAMEs, so the strip is pure bind/mark erasure and rewrites no names (rul24M-bare-dorcism-names · rul24-totalistic-munge, `Research/notes/24C`). The eol pole was never actually clean (the `kOOB` breach). Residual `dq-kOOB` scope: the formal stamp (human's); entity/selector grammar + per-position charsets (human-deferred); kind evolution beyond the within-unit duplicate-resolver refusal (rul24M-kind-unify-owed). Bootstrap-kind curation: resolved — `sm.dorc.*`, mandatory reverse-DNS ≥2 dots.
**Owner:** user (DX/ergonomics vs off-ramp purity). **Lock-in:** med (the spelling threads the parser + every typed oracle; the off-ramp guarantee is high-lock).
> Entangled with `kOOB` (the eol pole's cost) and `kLANG` (the inline pole's cost). Sources: `Research/plans/17N` top paragraph · `17O` F-OFFRAMP · `spike/CLAUDE.md` strip-fidelity.

### `kCONTRACT-RUNGS`
Poles: `kCONTRACT-RUNGS-single ↔ kCONTRACT-RUNGS-ladder`

**Tension:** authoring a verdict-function is ONE act carrying the full license (rul24-vouch-is-verdict-authoring: write `is_converged()` and Dorc may guard AND elide on its yes — minimal ceremony, no new syllable, maximally sh-native) **vs** a separable license-ladder (ORACLE_PROVIDES provides-license: rung-0 display-only / rung-1 guard-in-position / rung-2 carried-elide) letting a wary engineer hand over *answers* while withholding *permission* — the engineer-side hatch, sibling of the admin's `kSURVIVAL-trusted` flag. Single risks over-licensing by authors who only meant to inform (there is no way to provide a verdict without licensing skips); the ladder is the least sh-native thing in the design (POSIX has no idiom for "blame me") and would have to be a loud first-class construct under the native-or-break-loudly law. Rung-selection must stay per-path or the monotonic-degradation story breaks.
**Status:** open, **default pinned** (rul24M-rungs-default, `Research/notes/24M` §1): an unmarked verdict-function reads as full-license — guard AND elide at its own sites — permanently; future rung machinery arrives only as opt-down spellings on the withholding side, never re-reading unmarked functions. ORACLE_PROVIDES's ladder remains the standing pressure, buildable only on that side; the field trial's adequacy-bite data is the first evidence either way. **Owner:** user (liability-surface taste). **Lock-in:** med (the pinned default caps later churn at additive opt-down spellings).

---

## The probe optimizer — *per-leaf economics of checking vs acting*

### `kPROBING`
Poles: `kPROBING-probe-first ↔ kPROBING-just-run`

**Tension:** avoid expensive/dangerous redundant *work* (check before acting) **vs** avoid redundant *checking* overhead (for a cheap idempotent op like `mkdir -p`, the probe's stat can cost more than just doing it). The apply-cost×check-depth banding (VALUE / JUST-RUN / HARD) lives on this axis — and the guard tier added a third consumer, the **check-tax**: a guarded site pays its oracle-check on every apply, forever, so an expensive check must either earn its vouch or just-run (`Research/plans/233`).
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

**Critical note:** (human): if this is ever unparked, it is *critical* that is unparked *alongside some hostile-host security work*. The moment we store state from one host and affect the other with it, we're a security vector.

Standing fences while parked: the probe-TAPE is a write-only postmortem durable, never this knob's reuse-cache — nothing re-ingests receipts across runs (rec-5, welded; `spike/CLAUDE.md`); and the wall-clock-keyed verdict classes (package-index freshness, cert expiry) are the first hermeticity test-cases any content-key design must survive — they are inherently volatile-keyed and break naive keying first.

---

## Execution & modes

### `kELISION` — DEPRECATED → `kSCOPE` + `kSURVIVAL`
Retired (human, 2026-07-05); header kept here, in reading order, for reference. One slug carried two jobs and drowned once "elision" firmed as the replacement-mechanism's name: its actual axis (what is in scope to be checked at all) is now **`kSCOPE`**, directly below; the trust-dial its disambiguation-footnote grew is now **`kSURVIVAL`**. Mechanism vocabulary: the Named-mechanisms block. Reading rule for the existing corpus: a `kELISION` citation means `kSCOPE`, unless the surrounding text says "replacement-elision" (→ **elide**) or "the survival tier" (→ `kSURVIVAL`).

### `kSCOPE`
Poles: `kSCOPE-asked ↔ kSCOPE-whole-book`

**Tension:** deliberately *not checking, right now*, state the user hasn't asked about — hot-loop speed inside the asked scope, staleness accepted (and disclosed) outside it (`dorc bump`-style partial update) **vs** checking everything the book expresses — completeness, no drift (`dorc reconcile`-style full convergence). The knob is *what is in scope to be checked at all*; it never touches soundness inside that scope. A line outside the asked scope is **descoped** (see Named mechanisms) — never "elided": nothing was proven about it.
**Status:** mode (user picks via update/reconcile; changes checking *scope*, never *soundness*). **Owner:** user (runtime). **Lock-in:** low.

### `kSURVIVAL`
Poles: `kSURVIVAL-trusted ↔ kSURVIVAL-honest-walls`

**Tension:** keep proven-converged elisions *past a command that really runs*, on the strength of authors' at-most footprint claims — attention preserved on drifted days, the whole stage-5–7 product (USER_STORY "the bought unsoundness") **vs** honest walls — a running un-footprinted command demotes everything downstream to guard/run, nothing ever rests on traveled human claims, and the check-tax plus the attention-lines come back. The trusted pole is the design's one *naked* trust: a wrong at-most claim silently under-executes *someone else's* line with no runtime net, which is why it takes a double opt-in (the author's clean claim AND the admin's explicit flag) and full per-elision attribution.
**Status:** mode at the admin level (the explicit flag — never a default, short enough not to alias away, honestly "marketing at best, theatre at worst" and demanded anyway; rul24-mode-gate, `Research/notes/24A` §1a). The design-level posture it sits atop is `kHALVES` (welded); whether this outermost tier earns its keep at all is the field-trial's decomposed question, fires-often × bites-rarely vs the felt magic (`Research/plans/252` B4). **Owner:** user (the admin per-invocation; the author per-claim). **Lock-in:** low mechanically (a flag), med socially (published at-most claims accrete against whatever the gate promises).

### `kOBJECTIVE`
Poles: `kOBJECTIVE-latency ↔ kOBJECTIVE-throughput`

**Tension:** minimize time-to-first-action (deployer "server's on fire, NOW") **vs** maximize whole-fleet makespan (engineer's full reconcile) — different objective functions, hence different optimizer defaults.
**Status:** open (derive from mode + a coarse urgency intent). **Owner:** user-intent. Coupled to `kSCOPE`. **Lock-in:** low.

### `kFIDELITY`
Poles: `kFIDELITY-optimized ↔ kFIDELITY-faithful`

**Tension:** performance (the minimized, batched, opaque production probe) **vs** debuggability / attribution (`--faithful`: one-leaf-one-exec, 1:1 source mapping — the seam the realtime-output requirement *and* the future tracer both need). The "provenance-preserving" obligation is heavier than a 1:1 source map: the faithful seam must preserve an N-tier, per-host-forking, host-qualified multi-locator derivation DAG (loc-host / loc-user-src / loc-probe / loc-surface; `Research/plans/111`).
**Status:** open — both ship (`kFIDELITY-optimized` default, `kFIDELITY-faithful` reserved). **Owner:** dominant-strategy. **Lock-in:** high — the leaf-execution seam must be wrappable + provenance-preserving from day 1.

### `kCOMMS`
Poles: `kCOMMS-executor-OOB ↔ kCOMMS-transpilation-inband`

The form of Dorc's own controller↔host metadata: a bootstrapped probe-executor reporting out-of-band (Ansible-python-style), or transpiled markers in-band in a real-shell stream. A false friend of `kOOB` (same in-band/out-of-band axis, but Dorc's implementation I/O, not user-written config); either pole rides the one `kFIDELITY` session seam.
**Status:** open, directional. The two poles conflate two *orthogonal* axes — in-band↔OOB and executor↔pure-sh — and the lean is the **executorless-OOB** quadrant the poles omit: tool I/O full-fidelity on native SSH channels; Dorc-signalling out-of-band, split by size/urgency (short gating verdicts on a shared atomic fast-lane; large diagnostics in per-leaf files demuxed by filename); the executor pole re-pinned to {no-writable-fs, hard backpressure}, *not* concurrency/attribution; security structural (signalling never shares a lane with freeform). Residual: writable-fs on stripped/Windows targets. Full resolution: `Research/plans/142`. What *forces* the decision (this knob + the marker-protocol/backpressure/async-vs-statemachine cluster) is the live-plan/reactivity work (`plans/22H`) — live ∧ concurrent is what demands an executor; single-shot whole-script shipping (the spike/trial floor) commits nothing.

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

## Pseudo-knobs — consequence-gradients (named to watch, not to dial)

A pseudo-knob is not an orthogonal dimension we can *choose* a position on, like the knobs above; it is a synthesis of other knobs plus what turns out to be decidable/provable in the analyzer — a gradient the design gets *shoved along* as those resolve. Named so the shove stays visible and attributable, and so we can reason about which real-knob moves shift it.

### `kSILO`
Poles: `kSILO-full-parity ↔ kSILO-different-limitations`

**Tension:** where correctness-code (guards, checks, convergence predicates) accumulates over time, between its two possible homes. Under `kSILO-full-parity`, correctness-code is analyzed/lifted/rewarded identically wherever it lives — book or oracle — so admins keep writing guards in their books, books stay defensively rich, and the standalone off-ramp stays strong (the book still self-guards when run without Dorc). Under `kSILO-different-limitations`, the oracle-contract dialect is more analyzable than arbitrary book-sh (the kind-annotation, the constrained argparse, declared cardinality exist only there), so every capability gated behind it is a pull to migrate correctness out of books into oracle-libraries: books lean out, the world's published shell gets less defensive, and the book-alone off-ramp degrades to "runs, but re-runs blind."
**Status:** pseudo — not directly settable; the position is an *output* of `kBURDEN` (who declares), `kTYANNOT`/`kOOB` (where annotation may live at all), `kLANG` (the off-ramp weld), the analyzer's per-file complexity dial (one machinery for books and oracles, thresholds may differ — 19H §1.3), and — the biggest shover — which half of the probe model the UX visibly *rewards*: oracle-fact elision (Half A: in-book guards redundant ⇒ silo-pull) vs guard-subsumption (Half B: the admin's own guard is what buys them speed ⇒ anti-silo). The machine-inserted guard tier is the strongest live pull toward `kSILO-different-limitations` (admins' incentive to hand-write guards drops when the machine writes them — independently flagged by all three of its design-review crosscheck agents); both directions are now real in-build, so user habit is the thing to watch.
**Owner:** emergent (analyzer decidability) + user (which gradient we *want*, expressed through the constituent knobs). **Lock-in:** low as a mechanism (it has none of its own); high as a watch-item — by the time the gradient shows up in user habit, the constituent decisions that caused it are baked.
> Mitigations, so the gradient isn't over-feared: an admin's guard and an oracle's check are usually not duplicates (deployment-specific intent vs tool-state truth; the genuinely-siloable overlap is the narrow middle cohort); in-book guards retain selfish value under any oracle regime (safe bare re-runs after partial failure, defense against engine misprediction, the unreliable-oracle cell) — surfaceable as a lint-nudge; the machine never duplicates an admin's hand-written guard (the no-double-guard rule — their guard *is* the mechanism, machine-recognized, pinned); and the Half-B reward is demonstrated (the admin's own `dpkg -s … || install` lifts and elides). Origin: the human's silo concern, human-named poles; 19H §1.1/§1.3 · 196 §2 · `notes/237` rec-5.

### `kWHICHSH`
Poles: `kWHICHSH-minimum-lcd ↔ kWHICHSH-maximum-gcd`

**Tension:** one small, pinnable dialect target — the *minimum* superset of classic POSIX that defensive ops-sh needs (dash-ish; a single parser; shippable *as* an executor; "testing we do what we promised" collapses to differential-testing against a named `dash` version) **vs** the *maximum* common subset of the shells users actually daily-drive (bash/zsh/ash — meet existing artifacts and habits where they are; but then no single target exists, we become the arbiter of "what do the major shells sufficiently-similarly implement?", and fidelity/correctness is a moving target). Human-voice source: DESIGN's *"POSIX" sh* section (the two poles, the three constraints, and the lean: "a very mild superset of POSIX; or maybe a slight subset of POSIX2024", pinned to a specific shipped artifact, `dash >= 0.5.13`). This knob is the surviving *dialect* sub-axis inside the `kLANG` weld — kLANG settled "sh, not a second language"; kWHICHSH is "which sh."
**Status:** **WELDED (human, typed 2026-07-12) → `kWHICHSH-minimum-lcd`, as an *executable* floor — do not relitigate:** a valid dorc-lang v0.1 base-dialect text is a stripped file that parses and runs identically under pinned `posh` and pinned `dash`; where the two disagree, the construct is outside the dialect. (Pins, ruled 2026-07-12: **`posh 0.14.1` ∩ `dash 0.5.12`** — dash 0.5.12 is the newest release lacking pipefail (it lands in 0.5.13), so the pair *agrees* on rejecting bare `set -o pipefail`; posh 0.14.1 is current-Debian-stable. Real-binary battery empirics: `.claude/research/kwhichsh-gcd/turn02-*`; acked ruling list + fences: `Research/notes/276`.) The weld inherits Debian Policy §10.4's institution: policy text = the citable human-readable spec, posh = the enforcement binary, `checkbashisms` = a free linter. pipefail lives *above* the floor by design (analyzer-modeled; per-host handshake in the apply lane; non-pipefail executors an explicitly unsupported class; the paste spelling is the self-gating `(set -o pipefail 2>/dev/null) && set -o pipefail` idiom; authored bare-form text is accepted-never-modified, so strip-output floor-legality holds for *lint-clean* text — `276:rul-pipefail-emit-never`). The *why* of the oracle-side strictness is shareability (human, 2026-07-12): oracles are the cross-shell carrier — a stripped oracle must run on any POSIX box, whichever shell its consumer daily-drives, so an author's hard-won work reaches every consumer, and a consumer's shell taste never costs them access to the library. **The scope carve stands: the weld binds ORACLE/marked-dialect text only; *book*-acceptance is a separate, open question** (TODO.md:13–14 lean: dorc-lang strict in marked files; bash/zsh possibly best-effort-tolerated in unmarked runbooks) — a value-ladder (parse-for-sites / probe-verdicts / guard-lift / guard-insertion), not a parse-bit, with the far pole admitting even non-sh books someday provided the book can itself invoke stripped-POSIX oracles. Evidence bearing on that open half: the famous dotfiles artifacts are bash/zsh while the portability-minded core deliberately writes POSIX (`Research/plans/24R` §0b, repurp-finding16/38). Dialect-lean history: r23-h3 (`Research/notes/23F`), subsumed by the weld.
**Owner:** welded (human) on the floor; user + corpus on the open book-acceptance half. **Lock-in:** absolute on the floor (welded; and `__role` name-recognition was already a permanent, unversionable surface, `Research/notes/24M`); med on the book-tolerance tier, concentrating in the parser's error-tolerance posture (decided at the rebuild, cheap never again).
Entangled: `kLANG` (parent weld) · `kTYANNOT`/`kOOB` (what the *marked* dialect adds is settling in the `271` sittings) · `kTPLATFORMS` (sh-precondition targets, below).

## Platform reach — *how far do we bend for odd hosts/targets?*

### `kTPLATFORMS`
Poles: `kTPLATFORMS-mainstream ↔ kTPLATFORMS-wide`

**Tension:** bend engineering toward mainstream Linux/macOS targets (fewer features broken for odd hosts) **vs** reach the long-tail — Windows, ARM, RasPi, RISC-V, Synology/busybox — the heterogeneous fleets the initial userbase runs.
**Status:** open, lean `kTPLATFORMS-wide`, but gated by `kLANG`: "wide" is tractable only as "any target that can already evaluate POSIX sh" (an sh-precondition + per-platform oracles); Windows-without-sh forces the transpile/foreign-input options `kLANG` welds out. **Owner:** user + corpus. **Lock-in:** med (entangled with `kLANG` + oracle-library portability).

### `kWINLOCAL`
Poles: `kWINLOCAL-nix-only-controller ↔ kWINLOCAL-windows-supported`

**Tension:** *nix-only orchestrator host (every push-tool analog does this — Ansible/Salt forbid a Windows controller; fork/local-exec UNIX-isms stay free) **vs** a native-Windows controller for the Windows-daily-driver homelabber.
**Status:** directional, mild-lean `kWINLOCAL-nix-only-controller` (WSL is the prior-art escape hatch; the analyzer is platform-free text, so low lock-in) — and the field evidence to date is uniformly friction-ward for a Windows controller: no `op` session-caching across processes, git-bash `ssh` breaking on foreign ssh-configs, CRLF-on-ship hazards for locally-authored books, no ControlMaster in Win32-OpenSSH (`Research/plans/139` §4). **Owner:** user. **Lock-in:** low.

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

### `kHALVES`
Poles: `kHALVES-guard-half ↔ kHALVES-elide-half`

**Tension:** the guard-half — runtime re-verification (`( check ) || cmd`), sound, zero naked trust, the book fast and safe *but not shorter*, attention unpaid **vs** the elide-half — lines removed from the plan on static proof modulo attributed human claims: the attention product Dorc exists for, unsound exactly where a claim is wrong. The split is driven by a technical cap more than by design taste: the frame problem makes sound past-wall elision impossible (`plans/233`, permanent), so every step past the guard-half's ceiling is *bought* with bounded, attributed unsoundness — the converged-vouch's adequacy gap (converged≠no-op) at the plain-elision tier; the at-most footprint trust at the survival tier.
**Welded far toward `kHALVES-elide-half`** (human): the design deliberately admitted that bounded unsoundness to retrieve the extra elision — "removal by proof, or honesty about the lack of one" (USER_STORY, the bought-unsoundness section) — on kHALVES doctrine's terms (`plans/239` §1: full elision stays THE goal; the guard-half is sister and *permanent* fallback, owed equal design attention; no guard-half decision may silently discard an elide-half constraint). The one sanctioned re-opening is evidence, not argument: the field trial's fires-often × bites-rarely numbers (`plans/252` B4), deliberately mechanical as the check on re-litigation impulses in *both* directions. **Owner:** welded (human). **Lock-in:** high (the vouch/footprint machinery, the `kSURVIVAL` gate, and the published-oracle contract all assume the lean).

### `kWARN`
Poles: `kWARN-rich ↔ kWARN-precise-or-silent`

**Tension:** emit every distinct root-cause detection the engine can make — warnings/hints/attribution as a primary product surface, the detection machinery kept alive and exercised **vs** the classical fatigue doctrine: a warning channel that cries wolf gets tuned out and dies (the SQL-Server plan-warnings lesson — "suboptimal warnings must be precise or silent"; `plans/111`).
**Welded to `kWARN-rich`, scoped to the spike era** (human; rul24-warnings-tune-high, `Research/notes/24G` §8). Three grounds: (1) *mechanism-keepalive* — a seemingly-noisy warning keeps detection, spans, provenance-threading, and routing alive for later, better, more-critical warnings; muting/removing is always cheap while adding high-quality detection late is expensive (the detectable-moment is during construction, when the builder holds the analysis); (2) *LLM-feedback* — the spike is LLM-built, and warnings surfacing in agents' tool-output re-teach invariants at exactly the moment of relevance; (3) *warnings ARE the product* — the hint/attribution/why channel drives the entire gradual-enhancement curve, so both the training-corpus prior and the senior-engineer noise-distrust prior are miscalibrated here. Scope of the weld: DISTINCT root-cause detections only — correlated-cascade suppression stands unchanged (AGENTS fail-fast: only root-cause is reported); end-user attention-economy pressure routes to the late, cheap knobs (tiering, curation, muting), never to not-building the detection. **Owner:** welded (human). **Lock-in:** low — removal is always the cheap direction; that asymmetry *is* the rationale.

---

## Named mechanisms — firm vocabulary, not knobs (licenses differ; never conflate)

The five per-line outcomes that recur across planning, one name each. (AGENTS.md's terminology-firming is the human-voice source for elide/replace/guard; this table registers the set so nothing borrows a neighbor's name again.)

- **descope** — a line outside the user's asked scope (`kSCOPE-asked`) is not checked at all this run; staleness accepted and disclosed. License: the user's mode choice. Claims nothing about the world.
- **omit** — a branch proven *dead* by value-flow (a folded guard's untaken arm); it could not run. License: probe-sourced values through pure dataflow. (Engine-firm already; listed for completeness — the yardstick counts it separately from elision.)
- **elide / replace** — an observable-preserving replacement of a proven-converged command (elide = the degenerate full-skip case, per AGENTS.md). License: probe-facts PLUS a reached converged-vouch (the elide-weld), under `kFAIL-perform`.
- **guard** — the oracle's own verdict-function inserted ahead of the command's untouched bytes (`( check ) || original`); reproduces nothing; fails toward run. License: the converged-vouch (judgment-tier, attributed).
- **survive** — an elision kept past a RUNNING wall. License: the vouch PLUS footprint × backing disjointness PLUS the admin's `kSURVIVAL-trusted` flag. The only naked-trust cell; fully attributed.

## Not a knob (a prioritization principle, parked here so it isn't mistaken for one)
**`effort-allocation`** — engine-core vs oracle-long-tail vs analyses-on-top. *Not* an A-vs-B design tension; a resourcing call. Lean (user's): highest per-day marginal value is the **core extensible engine** + **analyses-on-top that promote correctness/UX/perf properties**, even though the oracle *corpus* has the larger total eventual reach (community-grown, long-tail). Bootstrap only the ~40-50 highest-frequency oracles (the field-trial stdlib is that slice; `Research/plans/252` P5); let the community grow the tail.
