# 231 — the collapsed-gradient sweep (r23 §1 walk-back map)

This is the conductor synthesis of the r23 §1 collapsed-gradient sweep: 8 read-only
agents (`trust-source`, `coverage-completeness`, `channel-vouch`, `cardinality`,
`convergence-reachability`, `maymust-recovery`, `unseeded-hunt`, `exclusion-check`)
fanned across the spike source + planning corpus to find decisions that quietly
collapsed a *gradient* into a *boolean*, per the `plans/230` charter. Workflow
`wk4oe6wgv`; all 8 agents returned. It clusters their candidates, adjudicates each
against THE CRITERION (a gradient exists iff partial-benefit genuinely exists), and
ranks the real ones by lock-in. Everything here is AI-generated process-evidence, NOT a
correctness claim — the agents are fallible, citations are how you verify, and where I
disagree with an agent I say so. Confidence is marked per cluster (+SURE/~SUSPECT/-GUESS/
--WONDER). Trust the human docs (`IMPLEMENTATION.md`, `spike/CLAUDE.md`) and the welded
`inv-*` over any agent claim. One agent (`unseeded-hunt`) was size-capped by the harness:
its JSON array carried ONE of a claimed 8 candidates — the other 7 are LOST (in its
unread final message), a known gap flagged in §5.

## 0. The result in one paragraph

The sweep found a THIN decision-surface and a fat reporting-surface, exactly as the
charter §3 predicted ("do not pre-assume a large decision-surface"). I cluster the
candidates into **6 real gradient-clusters** plus a hard must-stay-boolean fence. Of the
6, only **two are coverage-completeness gradients that exist at HEAD and need no new
trust-type** (the multi-cell classify cliff; per-member/per-entity partial elision) —
these are the highest-value, lowest-novelty walk-backs. The trust/source-provenance
exemplar that `plans/230` led with turns out to have **ZERO live decision-edges today**:
the sole elision-minting site (`plan/lib.rs:301-335 prove_replaceable`) keys on nothing
source-tagged, so trust-taint at HEAD is purely a *reporting* concern (why-lens +
coverage `c2`) — its decision-surface must be *built* (a new decision-plane cell), it is
not a boolean waiting to be un-collapsed. Decision-surface sizing verdict: **THIN**.
Channel-vouch headline (23-c2): the strong claim that "mention-a-channel = vouch-it
ALREADY holds" is **REFUTED** at the spelling level (the `kind#channel#prop` annotation
form does not exist) but the human's *instinct* is right — vouch-by-mention is already an
idiomatic, shipping pattern (`oracle_effect … query …`), just never extended to the
channel-completeness axis; and `dc-elide-on-trusted-default` **IS** gated on a `dq-kOOB`
ruling (via `kTYANNOT`), NOT unblocked. The must-stay-boolean fence in one line: every
family splits the way THE CRITERION predicts — the precision/coverage half admits a
gradient (toward run-LESS, opt-in, fail-safe), the safety half (probe-inertness,
undeclared-mutation, solve-convergence, the Unknown-fold, Must-licensing) stays boolean,
and a gradient may only ever DEMOTE toward run, never PROMOTE toward elide.

## 1. The walk-back map — real gradients to un-collapse (ranked by lock-in, high→low)

Lock-in = how hard to un-bake later ("decide the shape now" = high; "reversible local
change" = low). High-lock first.

### 1a. `decision-plane-trust-cell` — the source-but-inert / decision-but-blind split (lock-in: HIGH)

Converged by **3 agents** (`trust-source` cands 1+2, `exclusion-check`
`fence-trust-receipts-decision-inert`, `convergence-reachability`
`host-as-adversary`). This is the `plans/230` §2 worked exemplar, pinned: `OriginKind`
(`core/prov.rs:76-94`) HAS the source taxonomy (`BookSource`/`OracleClaim`/`ProbeResult`/
`TopCause`/`Join`) but is welded decision-INERT (ru-11, `prov.rs:10-27`; `ProvId` is
`!Ord` at `prov.rs:54-55`). `Predicted<T>`/`Observable` (`core/lib.rs:361-367`, `449-472`)
HAVE the decision power but are source-BLIND. Neither is a decision-plane trust cell.
- **boolean-today:** `Predicted<T> = Value(T) | Top` — a 2-state cliff with ZERO record of
  *why* it is Top (undeclared-gap vs probe-couldn't-tell vs oracle-claimed-but-distrusted).
- **partial-benefit (CRITERION):** GENUINE *as a structure to build*, but note the honest
  asymmetry — an oracle-CLAIMED converged-rc and a probe-OBSERVED rc are both `Value(N)`
  today, indistinguishable; a graded source-tag (`claimed < proven`) lets merge prefer the
  higher-trust source instead of both collapsing to Top⇒run. **My adjudication: this is
  the *home* the trust exemplar needs, not itself a live collapsed decision** — there is no
  decision keyed on it yet to un-collapse (see §3). It is high-lock because where the cell
  attaches (on `ValueOf`/the Observable, referenced-BY the receipts, never stored-IN them)
  is the "decide the shape now" call.
- **safe-direction (inv-kfail):** refuse-only. A `Value(claimed)` already licenses today;
  a trust-tag can only ADD a refuse (distrust ⇒ block ⇒ run). A tag that UP-trusts past
  today's floor breaches kfail. The breach mechanism (ru-11): storing the tag IN the
  receipts and reading `OriginKind` back to drive a license — that also makes `Reach::Top`
  cause-sensitive and breaks fixpoint termination (`effect.rs:505-509`,521-532).
- **citations:** `core/prov.rs:76-94`, `:10-27`, `:54-55`; `core/lib.rs:361-367`,`449-472`;
  `analysis/value.rs:58-63` (`ValueOf = Literal(Symbol) | Top`, the attach point);
  `plans/230:81-88`.
- **confidence:** +SURE on the structural split; ~SUSPECT on `ValueOf` being the right
  attach point (the `trust-source` agent marked `valueof-no-source-tag` ~SUSPECT).

### 1b. `cardinality-strong-update` — no strong update exists; the uniqueness bit is unbuilt (lock-in: HIGH)

Converged by **2 agents** (`cardinality` cands 1+4, `exclusion-check`
`fence-cardinality-strongupdate-uniqueness`). The `cardinality` agent's headline is a
SURPRISE worth foregrounding: the strong/weak-update gate is not boolean — **it is
ABSENT**. `Reach::with` is `s.insert(fact)` (pure monotone may-set); BOTH `Establishes(f)`
and `Kills(f)` route through `state.with(*f)` (`effect.rs:739-757`), so a Kill ADDS a
write-marker and never removes the cell. `fnd-4`'s `{singleton,multiple}` cardinality bit
(`notes/180`) is reserved-not-minted; `EntityRef::Singleton` (`core/lib.rs:517-524`) is a
NAMING device for nullary mutators (`apt-get update`), NOT a cardinality-1 tag.
- **boolean-today:** uniqueness is *implicitly* boolean via the value-flow ⊤ cliff — a
  concrete literal operand ⇒ `EntityRef::Operand` (unique-by-coref); anything non-literal
  ⇒ ⊤/Opaque before the entity is even constructed.
- **partial-benefit (CRITERION):** the agents correctly split this. Strong-update ITSELF is
  binary (cardinality=1 ⇒ overwrite, else accumulate) — NOT a gradient. The genuine
  gradient is downstream: a *resolved-but-maybe-aliased* operand (`for h in $hosts; install
  package:$h`) reaching a WEAK-establish rung instead of cliffing to Opaque-poison.
- **safe-direction (inv-kfail) — DANGER:** this is the one place softening flips to the
  UNSAFE direction. Strong-update OVERWRITES (kills the old fact); a wrongly-confident
  "unique" verdict on an aliased entity means a kill/establish on one alias is treated as
  authoritative for all ⇒ stale resting probe trusted ⇒ wrong-elision (kFAIL-perform). The
  fence: strong-update stays gated by a BOOLEAN provably-unique (=1) test; a "probably
  unique" confidence may only ever DEMOTE toward weak/⊤, never PROMOTE to strong.
- **lock-in HIGH + a representational trap (`singleton-overload-naming-vs-cardinality`,
  +SURE):** the cardinality bit MUST be a SEPARATE field, never folded into the existing
  `Singleton` variant — a nullary mutator is unique (one reason), a resolved `package:nginx`
  is unique (a different reason), an aliased `package:$h` is NOT, and all three need
  distinguishing. `an-*` rows are `st=O` (retrofit-hostile, `16Q §1`).
- **citations:** `effect.rs:563-573`,`739-757`; `core/lib.rs:510-524`; `notes/180:45-54`
  (fnd-4),`:218-223`; `core/CLAUDE.md` + `ANALYZER-NEEDS §C`.
- **confidence:** +SURE the gate is absent (not boolean); ~SUSPECT on the weak-establish
  rung being realizable (`maybe-aliased-is-binary-via-top` is ~SUSPECT — most of this case
  is currently masked by the value-flow ⊤ cliff, a different dimension).

### 1c. `coverage-vouch-default` — the trusted-default completeness vouch (`dc-elide-on-trusted-default`) (lock-in: MED)

Converged by **3 agents** (`channel-vouch` `trusted-default-vouch-absence-is-gap`,
`coverage-completeness` `coverage-oracled-bool-reporting-collapse`, `exclusion-check`
`fence-coverage-default-direction`). This is the `plans/230` §3/§4 HEADLINE gradient,
restated correctly by the `channel-vouch` agent (its real target, after refuting the
spelling claim — see §4). `an-vouch-default` (`ANALYZER-NEEDS:47`) hard-codes "a stub
default is sound iff dead-or-vouched: effect←convergence, status←establishes-contract,
stdout/stderr←NOTHING"; `consumption_ok` (`plan/lib.rs:570-592`, verified) returns `false`
on ANY consumed `Stdout`/`Stderr` unconditionally (`:572-573`), and on `StatusRelaxable`
at ⊤ (`:582`). There is NO author surface to vouch "I reviewed this output; the default is
complete."
- **boolean-today:** a lazy gap and a reviewed-complete default are INDISTINGUISHABLE —
  both cliff to block/run. "Absence is ambiguous" (`plans/230 §4`).
- **partial-benefit (CRITERION):** GENUINE. A consumed `Stdout`/`Stderr` on an otherwise
  converged-ambient-Must leaf forces run today; a diligent author who confirmed the output
  is reproducible/irrelevant could unlock elision of THAT leaf (more commands elided) while
  lazy authors keep the safe block. The canonical coverage gradient.
- **safe-direction (inv-kfail):** SAFE only if the vouch is author-explicit AND the default
  stays block-on-absence (absence ⇒ ⊤ ⇒ run). A vouch that defaulted-ON (absence=complete)
  breaches kfail toward under-execute — the R2-MULTIOP/find-1 class. The reporting shadow
  (`coverage/lib.rs:274-275` `oracled: bool`) is decision-inert (a read-only dashboard) and
  cannot make any apply less safe — grading `c2` from bool to `{unmodeled, partial,
  vouched-complete}` is reporting-only.
- **lock-in MED, but entangled HIGH:** the *spelling* threads the parser + every typed
  oracle and is gated on `dq-kOOB`/`kTYANNOT` (see §4) — that entanglement is the high-lock
  part. **tc-flag (do not resolve):** whether the vouch reuses the `kTYANNOT`-inline surface
  (good DX, off-ramp-hostile) or the `oracle_effect`-row surface (off-ramp-clean, a new
  polarity dimension `R2-CHANGEDELTA` warns against).
- **citations:** `ANALYZER-NEEDS:47`; `plan/lib.rs:570-592`,`:255-281`; `plans/230:117-130`;
  `coverage/lib.rs:274-275`,`:9-13`.
- **confidence:** +SURE (the `channel-vouch` and `exclusion-check` agents independently
  landed the same gap with the same `consumption_ok` citation).

### 1d. `multicell-establish-classify-cliff` — a verb's N≥2 effect-cells cliff to MustRun (lock-in: MED)

Surfaced by **1 agent** (`coverage-completeness`, +SURE, UNSEEDED) and adjacent-cited by
the verdict agent. This is the `coverage-completeness` agent's nominated canonical r23
coverage gradient, and the most "shovel-ready" find: every piece is built EXCEPT the final
classify match. `command_effect` already returns `Vec<CommandEffect>` (multi-cell is legal,
`us-effectmap`); `reach_transfer` already gens EVERY cell; `resolve_probe` already resolves
per-`(kind,selector)`. But `classify_site` (`effect.rs:1077-1092`) matches ONLY single-
element slices (`[Establishes(f)]` / `[Queries(f)]`), and every `Vec` of len≥2 falls to
`_ => MustRun` — silently, and UNTESTED at classify (the agent checked: `multi_cell_verb_*`
tests only pin the oracle INDEX, never this cliff).
- **boolean-today:** a `(provider,verb)` with ≥2 declared establish cells (e.g. `purge` kills
  `#installed` AND dirties `#config`) classifies to `MustRun` outright — even when BOTH cells
  are converged+probeable.
- **partial-benefit (CRITERION):** met head-on. A verb establishing `#installed` AND `#config`
  could elide when BOTH probe-converged. `EstablishMembers`/`InlineCall` already show how to
  aggregate N cells all-or-nothing — the substrate exists.
- **safe-direction (inv-kfail):** SAFE — floor is MustRun(run); a multi-cell aggregate license
  (every cell ambient + converged + consumption-gated, ALL-OR-NOTHING) only moves toward
  Replace when STRICTLY MORE is proven. **Constraint (`mixed-written-ambient-multicell`,
  ~SUSPECT):** the aggregate MUST fold written-ness conservatively — any cell that is
  `EstablishWritten` (upstream-mutated) ⇒ run; the per-cell ambient/written split has no
  representation today (`EstablishAmbient`/`Written` are single-FactKey variants), so cand-1's
  license must require EVERY cell ambient.
- **lock-in MED. tc-flag (do not resolve):** whether the multi-cell aggregate stays strict
  all-or-nothing or eventually goes partial-cell (like partial-member, 1e) is a soundness call
  for adversarial design; safe default is all-or-nothing.
- **citations:** `effect.rs:1077-1092`,`:204-209`,`:304-338`,`:739-740`,`:605-620`;
  `oracle/lib.rs:100-110`,`:1037-1061`.
- **confidence:** +SURE the cliff exists and is untested at classify; ~SUSPECT on the
  written-cell composition detail.

### 1e. `partial-member-convergence` — one diverged member runs the WHOLE family/set (lock-in: MED)

The single broadest convergence across the sweep: **5 agents** (`cardinality`
`loop-member-family-all-or-nothing`, `coverage-completeness` `an-partial-convergence-
multientity`, `convergence-reachability` `per-channel-effect-verdict-not-multi-entity`,
`unseeded-hunt` `partial-member-and-family-license`, `exclusion-check` `fence-maymust-self-
reach-allornothing`). All point at `an-partial-convergence` (`ANALYZER-NEEDS:82`, `st=D`
DEFERRED). Two shapes of the same gradient: across-ENTITIES (`apt install nginx curl jq`,
one verb many targets) and across-MEMBERS (`for p in nginx curl jq; do install $p`). Today
`member_family` (`effect.rs:352-387`) is ALL-OR-NOTHING (`_ => return None` on any non-
establish member) and the multi-operand argv degrades to ⊤ at the `[ "$2" = "" ]` R2-MULTIOP
guard ⇒ run-all. `prove_members_replaceable` carries a per-member `&[Verdict]` vector — the
SHAPE exists for loops, just not for straight-line operand-sets.
- **boolean-today:** N-1 cleanly-converged members buy NOTHING if member N is messy.
- **partial-benefit (CRITERION):** the textbook case — `IMPLEMENTATION.md:147-152`'s own
  "elide many … or fewer-but-still-some" example. Elide the converged subset, run the rest.
- **safe-direction (inv-kfail) — a real hazard the exclusion-check agent caught (~SUSPECT):**
  this is **NOT a safe in-place softening of the existing boolean.** The all-or-nothing
  `self_reached`/all-converged conjunction (`effect.rs:639-641`, `plan/lib.rs:394`,`:420`) is
  a FIXED-POINT self-consistency argument — elide-ALL is sound because the elision's own
  effect removes the body's writes; a PARTIAL member elision leaves the run-members' writes
  reaching the elided ones, so their resting probe is NO LONGER authoritative ⇒ wrong-elision.
  The gradient must be a SEPARATE analysis (per-entity reaching with the partial-elision
  effect modeled), per-member re-derived `self_reach`, NOT a knob on the boolean. Bounded by
  sh process-atomicity (you cannot elide PART of one `exec`) — realizable benefit is
  per-command decomposition, an oracle-contract question.
- **lock-in MED. tc-flag (do not resolve):** the per-member `self_reach` re-derivation is the
  soundness-critical design (`tc-l2-member-list-not-rewritten` is cited; `21W` authorized the
  re-scope).
- **citations:** `effect.rs:352-387`,`:639-641`; `value.rs:476-479`;
  `plan/lib.rs:394`,`:411-440`,`:464`; `ANALYZER-NEEDS:82`; `20U:132-133`.
- **confidence:** +SURE the collapse exists; ~SUSPECT on realizability (the self-reach
  fixed-point breakage makes this materially harder than it looks).

### 1f. `door3-recovery-dormant` — certainty-by-computation the fold half-built then dropped (lock-in: LOW)

Converged by **1 agent** (`maymust-recovery`, three candidates). The `plans/230 §2`
counter-example refinement ("you CAN gain certainty by computing"; `flaky-⊤ || true` has a
provably-0 rc) is ALREADY the working exemplar — `door3-already-recovers-the-exemplar`
(+SURE): `StatusInvariant` never blocks even at ⊤ because the `||` read is dead-in-fact
(`plan/lib.rs:585`, verified). Trust there is already a meet-over-SEMANTIC-dependencies, not
over-inputs. The collapse is in three dormant/narrowed siblings that follow the same template:
- `andor-both-operands-agree-dropped` (+SURE, low-lock): `eval_and_or`'s None branch sets the
  construct status to ⊤ unconditionally, even though the doc names the recoverable case (both
  operands same known rc ⇒ that rc regardless of which ran). `fold.rs:267-272` — recovery
  stated in the comment, then `AbstractRc::Top` returned flat.
- `node-rc-line-value-recovery-dormant` (~SUSPECT, med-lock): `FoldResult.node_rc`/`rc_of` is
  built + documented to let the renderer compute a fully-folded LINE's stand-in, but `render.rs`
  NEVER reads it (grep-confirmed) — only `dead_controller` is consumed (`lib.rs:1240`). A
  compound line whose whole rc is structurally computable could be Replaced as a unit.
- `door3-narrowed-to-bare-true-keyword` (+SURE, low-lock): door-3 is gated to a bare `true`
  via `right_is_bare_true` (`cfg.rs:1748`); `|| :`, `|| true >/dev/null`, `|| { :; }` all keep
  `StatusRelaxable` and block at ⊤, though `:` is equally observable-free. A deliberate-
  conservatism cut (`20V §4 d-2`), flagged as a sizing question not a bug; `|| false` MUST
  stay excluded (rc 1 is load-bearing).
- **partial-benefit (CRITERION):** GENUINE and monotone — each recovered known rc lets a
  downstream consumer fold/relax where it currently sees ⊤ and blocks ⇒ more elisions. None
  is all-or-nothing.
- **safe-direction (inv-kfail):** all three are SAFE — a recovered value is provably-correct
  (both branches yield it), so recovery only adds precision toward run from an over-conservative
  floor. The `node-rc` line-collapse carries a watch (~SUSPECT): a line-level stand-in must
  reproduce EVERY consumed channel of every leaf on the line, not just rc — must route through
  `consumption_ok` per-leaf before any collapse.
- **lock-in LOW** (`andor` and the door-3 widening are local fold/cfg edits); the `node-rc`
  wiring is MED.
- **confidence:** +SURE on `andor` and the narrowing; ~SUSPECT on `node-rc` realizability.

> Cross-cluster note on `trust_reach`: `trust-reach-global-meet-over-inputs`
> (`maymust-recovery`, ~SUSPECT) and `convergence-trust-boolean-converged-flag`
> (`convergence-reachability`, +SURE must-stay) DISAGREE. The first calls the single global
> `trust_reach = reach.converged` (`effect.rs:1058`) the §2 meet-over-inputs anti-pattern and
> proposes a per-consumer/per-region convergence credit; the second says solve-convergence has
> no sound partial-benefit (a capped solve may have missed an upstream kill anywhere — you
> cannot say "80% converged") and is correctly boolean. **My adjudication: the second is right
> for the DECISION, and the disagreement is a real tc-flag.** Whether convergence (a solver-
> completeness property) can be sliced per-value the way information-flow taint can is a genuine
> judgment call the `maymust-recovery` agent itself flagged as tc-shaped. In release `Reach` is
> finite-height + monotone so the cap never trips (debug_assert-guarded) — the boolean is
> defensive and effectively dead, which weakens the case for the credit. **Flagged
> `tc-convergence-per-value`, NOT placed in the walk-back map** (it is at best a deferred,
> soundness-fragile precision lever, not a safe un-collapse).

## 2. The must-stay-boolean fence — do NOT gradient here

The `exclusion-check` agent's cross-cutting finding (its `notes`, +SURE) is the spine: every
family splits the same way, and three breach-mechanisms recur. A gradient breaches when it
(1) PROMOTES toward less-safe (strong-update on "probably unique"; elide on "probably
converged"; Must on "high May") — always an inv-kfail-perform under-execute; gradients may
only DEMOTE/add-precision-toward-run. (2) Leaks a trust/coverage gradient into a CATEGORY
boundary that is not a confidence question (the Establish-rc firewall; probe-op inertness) —
the dc-probe-NOT class. (3) Reads a receipt/`OriginKind` to drive the gradient instead of a
separate decision cell — ru-11, and it breaks `Reach::Top` termination.

The fence rows (all `must_stay_boolean: true`, all +SURE unless noted):

- **`fence-trust-probe-shipping` / `fence-cardinality-probe-op-safety` (dc-probe-NOT).** Probe
  SHIPPING is a structural boolean self-vouch — `hostsim Host::run` (`lib.rs:64`) refuses any
  Establish/Kill in `Phase::Probe` regardless of confidence; `spike/CLAUDE.md:72` "no analysis-
  confidence threshold ever makes a probe safe." No "partially inert" probe exists. Mechanism:
  a trust score that let a higher-confidence un-vouched probe SHIP moves toward probe (less
  safe) — a dc-probe-NOT breach AND transitively an inv-kfail-perform breach
  (`IMPLEMENTATION.md:194-195`: under-estimating probe-safety "leads to relying on idempotence
  anyway"). Cardinality must NOT leak in either ("this probe over a unique singleton is safe to
  mutate"). This is the `rul-mutation-impossible` / `dc-probe-NOT` principled exclusion, not an
  arbitrary weld.
- **`fence-coverage-effect-completeness-NOT-mutation` (rul-mutation-impossible).** SPLIT and
  load-bearing: a coverage gradient may grade WHICH DECLARED channels are modeled (toward
  run-less), but a gradient over whether an UNDECLARED mutation exists has NONE — a missed
  mutation is the mutation-class. The breach: a completeness-confidence score that downgraded
  the `Opaque`⇒⊤⇒poison floor (let a partially-modeled command NOT poison downstream ambient-
  ness on "looks mostly complete") is an inv-kfail-perform breach. You can never confidence-
  soften "this might mutate something I didn't model" (`IMPLEMENTATION.md:140-145`).
- **`fence-channelvouch-establish-rc-firewall` (inv-probe-sourced-values).** An Establish-class
  site's record-rc is the PROBE command's rc, never the mutator's, so it feeds the fold's
  Status NOTHING (status ⇒ `Predicted::Top` always — `cli/main.rs:657-664`). The establish-rc
  and mutator-rc are DIFFERENT OBSERVABLES (a category error, not a confidence question). The
  breach: a confidence gradient letting a high-confidence Establish oracle's probe-rc feed
  Status fabricates the value a downstream `&&`/`||` fallback reads — the round-19 under-execute.
- **`grade-must-may-hardcoded` / `may-must-grade-2-level` / `fence-maymust-grade-direction`
  (inv-must-may).** `Grade{Must,May}` is a one-way 2-level boolean (`Must→May` legal, `May→Must`
  a compile-error, `lattice.rs:267-302`); at every live mint site `Grade::Must` is HARDCODED
  (`plan/lib.rs:1260`,381,435,515). Only an under-approximate Must may license. The breach:
  collapsing the `May→Must` compile-error into a runtime threshold ("May at 0.9 ⇒ treat as
  Must") inverts the soundness line. A May confidence may feed reporting/nudges
  (`an-enrichment-nudge`) or the toward-run direction only. **Flag (not a gradient): the spike
  never EXERCISES the May path in production — an evidence gap, not a collapsed gradient**
  (`grade-must-may-hardcoded`, ~SUSPECT).
- **`fence-convergence-trust-reach-boolean` / `fence-convergence-unknown-fold` /
  `host-as-adversary-forged-verdict` (inv-kfail-perform, an-host-as-adversary).** The host
  verdict IS legitimately 3-valued (`Converged`/`Diverged`/`Unknown`), but `Unknown` folds to a
  boolean Run in BOTH phases (`plan/lib.rs:90-135`; "No code path folds Unknown to a skip",
  `:71`). The sharpest live hole: a managed host can FORGE a single `Converged` on a Must fact
  and silently elide a needed apply — defended ONLY structurally (merge-on-disagreement +
  Must-only). The principled fix (`an-verdict-failsafe-default`, "failed/unknown until
  finalized") and per-host reachability (`an-host-reachability`, a dark/timed-out host ⇒ Unknown)
  are **UNBUILT in hostsim** (it never returns Unknown) — these are missing *coverage
  mechanisms that add precision toward run*, NOT gradients to soften the trust bit.
- **`reachability-overapprox-boolean-cliff` (an-reachable).** "Unreachable from entry" is a
  vacuous-⊥ indistinguishable from a clean ambient; reading it as ambient is the wrong-elision
  the boolean prevents. The genuine partial-benefit that DOES exist is already a SEPARATE
  mechanism — `arch-2` inlining (`try_inline_call`) makes eligible bodies reachable with a loud
  `CfgInlineRefused`. So the "gradient" lives as coverage-expansion (inline budgets), not a
  softened reachability bit.

Two SURPRISES the exclusion agent flagged for the design phase: two SEEDED "gradients" —
`fence-maymust-self-reach-allornothing` (the partial-member case, 1e) and
`fence-channelvouch-disagreement-merge` (`dc-disagreement`) — are **NOT safe to soften in
place**; they rest on fixed-point / category arguments, and the gradient there must be a
SEPARATE analysis, not a knob on the existing boolean. These two (the `dc-disagreement` rung
ordering, `plans/230 §5`, and `fence-cardinality-strongupdate`) are the tc-*-shaped soundness-
rung judgment calls flagged for the human.

## 3. Decision-surface honest-sizing (§3)

**Verdict: THIN.** The `trust-source` agent MEASURED it (not assumed) and the finding is the
single most important sizing result: it traced the sole elision-minting site, `prove_replaceable`
(`plan/lib.rs:301-335`), and it keys on exactly `{SkipClass, Grade, PhasedVerdict, consumed-
Channels, Predicted<Rc> status}` — **NONE of which carries a source or trust tag.** There is
currently ZERO analyzer decision keyed on trust/source-provenance. So the `plans/230` trust-
taint exemplar at HEAD is purely a REPORTING concern (why-lens + coverage `c2`), confirming the
charter's "if this list stays SHORT, thin decision-edge" hypothesis verbatim.

Count of places a trust/certainty level ACTUALLY changes an analyzer decision today: **0 live;
2 latent.** The two latent decision-edges are both already flagged by `plans/230 §3` as human-
blessed EDGES, not defaults: `merge_observable`'s same-cell tie-break (`dc-disagreement`,
`cli/main.rs:692-715`) and `dc-elide-on-trusted-default` (1c). Everything else the sweep surfaced
on the trust/certainty axis (claimed-vs-proven, Diverged-vs-Unknown, coverage `c2` bool, the
why-lens source split) feeds REPORTING only — decision-inert under ru-11.

Implication for r23 scope: the trust-taint lattice is **not a boolean waiting to be un-collapsed;
its decision-surface has to be BUILT** (the 1a cell, then wired into exactly the 1c vouch + the
1d/1e coverage aggregates). The genuinely-shovel-ready walk-backs are the COVERAGE-completeness
ones (1d, 1e, 1f) that exist at HEAD and need no new trust-type. r23 should right-size accordingly:
lead with coverage (thin trust decision-edge confirmed), treat the trust-cell as a structural
investment whose payoff is the 1c vouch and the `dc-disagreement` edge, and not over-invest in a
trust lattice whose live decision-surface is two human-blessed edges.

## 4. Channel-vouch (23-c2) — does mention=vouch already hold?

<!-- /* CORRECTED → notes/232 (2026-06-15, human-directed design dialogue). Take 232 over this
section on three points a future agent MUST honor: (1) `oracle_effect` is an OPEN spelling strawman
(193: "the exact sh spelling is oracle's to choose … a strain if awkward, not a blocker"), NOT a
settled vouch-mechanism and NOT a vetoed-and-dropped one; the inline `: Kind#selector` form is the
live alternative pole, gated by dq-kOOB — do NOT build on the marker form as if welded. (2)
`dc-elide-on-trusted-default` SPLITS by channel-nativeness: stdout/stderr/rc/fds/files get
off-ramp-clean *native-sh-idiom* contracts (e.g. the body-redirect `} >/dev/null`), UNBLOCKED; only
Dorc-modeled effect-cells need the invented `!`/annotation and are the dq-kOOB-blocked half — so this
section's "IS dq-kOOB-gated, NOT unblocked" headline is too uniform. (3) oracle-spelling ⊥
book-spelling: the oracle declares the modeled command's observables CENTRALLY (the lazy book needs no
guard), which is the feature — not a cross-actor gap. */ -->

**Verdict: the strong claim is REFUTED at the spelling level; the human's instinct is right
about the mechanism; and `dc-elide-on-trusted-default` IS `dq-kOOB`-gated, NOT unblocked.**
(`channel-vouch` agent, +SURE, with the two other agents corroborating the gap at 1c.)

The hypothesis was: "mention/predict-a-channel = vouch-it ALREADY holds; absence-of-mention =
can't-predict = Top = run; so `dc-elide-on-trusted-default` is NOT blocked on a `dq-kOOB` ruling."
Three findings refute the first clause:

1. **The proposed `kind#channel#prop` annotation form DOES NOT EXIST.** The inline annotation
   (`oracle/src/check/ast.rs:88-111`, verified) carries `name : kind = value` — IDENTITY only
   (kind + entity), three fields, no channel. `#installed`/`#enabled`/`#fresh` are fact-CELL
   selectors that appear ONLY in `oracle_effect` rows, never in the annotation. The `kind` field
   is "an opaque coordination handle (`inv-referent-agnostic`); never decoded for meaning"
   (`ast.rs:99-103`). Real fixtures, verified verbatim:
   - `pkg : package = "$1"` (`converged/package.oracle.sh:14`)
   - `svc : service = "$1"` (`exec-distinct-selectors/service.oracle.sh:17`)
   There is no channel-mention surface from which to read a vouch.
2. **`check()` resolves IDENTITY, not per-channel observables.** `check()`
   (`oracle/src/check.rs:30-32`, `eval.rs:16-18`) yields `(kind, entity, verb, probe_body)`. The
   Effect channel is derived DOWNSTREAM by `command_effect` from the `(kind,verb)→effect-map`
   polarity (`effect.rs:304-310`); Status comes from probe rc; Stdout/Stderr are never vouched
   (`cfg.rs:123-131`: "Effect is vouched by convergence and never enters the consumed set").
   `inv-one-observable`'s "check() PREDICTS per-channel values" is **aspirational wording** — the
   built reality splits the prediction across three sources. No author "predicts a channel inside
   `check()`" anywhere. (My note: this is the one place an agent contradicts an `inv-*` slug's
   literal text; it is a build-vs-spec gap the agent flags, not a weld breach — worth surfacing to
   the human since `inv-one-observable` reads as if `check()` does this.)
3. **The Stdout/Stderr completeness vouch the headline needs is hard-blocked with NO author
   surface.** `consumption_ok` (`plan/lib.rs:572-573`, verified) returns `false` on any consumed
   Stdout/Stderr unconditionally; `prove_replaceable` conjunct-4 doc (`:255-281`) "a declared rc
   does NOT vouch output content." `plans/230 §4` itself calls the spelling "unsolved and hard."

**Where the human's instinct IS right** — vouch-by-mention is an idiomatic, already-shipping
pattern, just on a different axis. The closest real analogue (`query-polarity-is-the-real-vouch-
by-mention`, +SURE):
```sh
oracle_kind=pkgstate
oracle_probe_pkgstate() { dpkg -s "$1" >/dev/null 2>&1; }
oracle_effect dpkg '' query installed          # ← MENTIONING `query` vouches read-only
```
(`headline-guarded-realistic/pkgstate.oracle.sh:9`, verified verbatim.) MENTIONING a verb with
`query` polarity vouches it read-only ⇒ its `check()` becomes a Query-guard whose rc can
substitute (`prove_query_replaceable`, `plan/lib.rs:337-356`, confirmed present at `:358`);
ABSENCE of any effect-map row ⇒ `Opaque` ⇒ runs (`effect.rs:305-310`). That IS mention=vouch /
absence=safe-Top — but spelled in `oracle_effect` ROWS, not the inline annotation, and it vouches
read-only-ness (Effect channel), not Stdout/Stderr completeness. The value-less Singleton
annotation (`absence-of-annotation-IS-already-top-run`, +SURE) is a second instance: a present
value-less annotation is the EXPLICIT Singleton opt-in, a wholly-missing one degrades to
`Top(MissingAnnotation)` "the safe direction" (`parser.rs:492-505`) — presence=opt-in /
absence=safe-Top, but for the entity axis only.

**So the MECHANISM is proven viable in-tree** (two shipping instances); **the GAP is that it has
never been extended to the channel-completeness axis** — that extension is the one true r23
gradient here (1c, `trusted-default-vouch-absence-is-gap`).

**On `dq-kOOB`-blocked-or-not:** confirmed BLOCKED, not unblocked. `KNOBS.md:56-61` (verified):
`kTYANNOT` (poles `kTYANNOT-inline ↔ kTYANNOT-eol-comment` — the completeness-vouch spelling
surface) is "open, **gated by a prior open question (`dq-kOOB`: whether a type-system exists at
all)**." The agent's note lands it precisely: the completeness vouch's *spelling* is gated by
`dq-kOOB`/`kTYANNOT`; the *mechanism* (mention-as-vouch) is unblocked and shipping on the
read-only-polarity and Singleton axes. The precise gap: **a channel-completeness vouch surface
does not exist and cannot be designed without first settling whether the vouch rides the
`kTYANNOT`-inline annotation (DX-good, off-ramp-hostile, gated by `dq-kOOB`) or a new
`oracle_effect`-row polarity (off-ramp-clean, but `R2-CHANGEDELTA` warns against adding effect-map
dimensions).** That is the `tc-`/design question, not a HEAD boolean.

## 5. What r23's design phase inherits

**Open questions / tc-flags (flagged, NOT resolved):**
- `tc-disagreement-rung`: the probe-observed-vs-oracle-claimed lattice ordering for a trust-aware
  `merge_observable` (`dc-disagreement`, `plans/230 §5`) — a SOUNDNESS call, design adversarially;
  wrong rung = soundness error, not churn. `fence-channelvouch-disagreement-merge` and
  `merge-observable-source-blind-collapse` both land it; the latter is the riskiest gradient in
  the sweep (preferring a source moves AWAY from Top/run, the inv-kfail-forbidden direction unless
  the preferred source provably dominates).
- `tc-cardinality-strong-update-rung`: the unique>summary rung for strong-update
  (`an-strong-weak-update`) — the one place a gradient could flip to the UNSAFE direction; must
  DEMOTE on any uncertainty, never PROMOTE on a threshold (1b).
- `tc-vouch-surface`: whether the channel-completeness vouch rides `kTYANNOT`-inline (gated by
  `dq-kOOB`) or an `oracle_effect`-row polarity (`R2-CHANGEDELTA`-adjacent) — §4's precise gap.
- `tc-multicell-aggregate-grain`: strict all-or-nothing vs partial-cell for the multi-cell verb
  license (1d) — safe default all-or-nothing.
- `tc-partial-member-self-reach`: the per-member `self_reach` re-derivation that partial-member
  elision REQUIRES (1e) — the fixed-point self-consistency that breaks under naive partial
  elision; the gradient is a separate analysis, not a softening.
- `tc-convergence-per-value`: can solver-convergence be sliced per-value the way taint can? The
  `maymust-recovery` (pro) and `convergence-reachability` (con) agents DISAGREE; con is right for
  the decision; flagged, not in the walk-back map (§1f cross-note).
- `tc-one-observable-build-vs-spec`: `inv-one-observable` says `check()` predicts per-channel
  values; the build splits prediction across effect-map / probe-rc / never-vouched
  (`channel-vouch` finding 2). Surface to the human — the slug text and the code diverge.

**Genuinely-novel unseeded finds worth keeping** (outside the seeded families, +SURE unless
noted):
- `multicell-establish-classify-cliff` (1d) — the `coverage-completeness` agent's nominated
  canonical r23 coverage gradient; every piece built except the classify match; SILENT and
  UNTESTED at classify. The highest-value low-novelty walk-back.
- The `door3` dormant-recovery trio (1f) — `eval_and_or` both-operands-agree (commented then
  dropped to ⊤); `node_rc`/`rc_of` line-recovery (built, render never reads it); door-3 narrowed
  to the bare `true` keyword. Three monotone, inv-kfail-safe precision wins the fold already
  half-built.
- The `cardinality` SURPRISE (1b) — there is NO strong update at all; the gate is ABSENT, not
  boolean; `Kill` accumulates rather than removes. Re-frames the whole `an-strong-weak-update`
  family as build-not-soften.

**Process/coverage caveats for the conductor (do NOT smooth over):**
- The `unseeded-hunt` agent was harness size-capped: its JSON `candidates` array held 1 of a
  claimed 8 (`partial-member-and-family-license`, which duplicates 1e); the other 7 are in its
  unread final assistant message and are **LOST to this synthesis.** If r23's design phase wants
  the full unseeded set, that agent must be re-run with a smaller per-candidate budget or its
  message retrieved. This is the one place the sweep is materially incomplete.
- Two referenced artifacts could not be ground by the agents and I did not re-verify: a literal
  `Research/notes/16Q` file (cited as `16Q §1` / `dq-entity-algebra` throughout but evidently
  consolidated — the retrofit-hostility flag is carried authoritatively in `core/CLAUDE.md` +
  `ANALYZER-NEEDS §C`, which suffices); and the precise line ranges in a few `effect.rs` cites
  drift slightly between agents (e.g. `classify_site` cited as both `:1077-1092` and `:1068-1085`)
  — the function is real and the cliff is real, but a builder should re-confirm exact spans before
  editing.
- I did NOT execute anything (frozen-evidence discipline); all citations are from static reads.
  The agents' line numbers are their claims — verified by me only for the channel-vouch §4
  examples, `consumption_ok`, the annotation AST, and the `dq-kOOB`/`kTYANNOT` KNOBS rows.
