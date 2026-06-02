# 82 — Kill-criteria pre-mortem: the menu, the decomposition, the scope-down pivot

> **Status (2026-06-01): reasoning record, not a measurement.** A falsification-focused
> session: an external pre-mortem (`kill-prompt.md`) proposed a menu of implementation-spike
> *kill-criteria*; the author responded adversarially (`kill-prompt-2.md`); the exchange
> collapsed the threat list and pivoted the first-implementation strategy. The forward-looking
> output is `plans/088-implementation-strategy-advisory.md`; **this note is the *why* behind it**,
> for retrospectives and for an agent tracing the roots of a later decision. Confidence markers
> throughout. All synthesis is AI-generated per `Research/README.md`; the author's quoted
> positions are human and weighted as such (and were given deliberately adversarially — "treat
> them as opinion/bias unless they carry new information").

## 0. What happened, in one paragraph
A skeptical pre-mortem read the repo cold and produced (a) a provenance map, (b) a ranked list
of load-bearing unproven assumptions, and (c) a Pareto menu of spike-based kill-criteria. The
author then pushed back per-assumption and supplied **one decisive piece of new information**:
a real "statistics spike" to measure the corpus go/no-go had already been attempted and *failed*
— not for lack of tooling, but because turning corpus numbers into a trustworthy, direction-setting
verdict is research-grade and epistemically fragile (it surfaced the peer-reviewed reasons it
doesn't work; see `[81]`). The net effect was **subtractive**: three of the assumptions dissolved
or recategorized, one (`A-VALUE`) was conceded and promoted to a canonical DESIGN kill-listing,
and the recommended first-implementation shape shifted from "measure the population" to "build a
thin slice and dogfood it."

## 1. The assumption set (as proposed, pre-critique)
Slugs introduced this session; ranked by the pre-mortem's "if false, project sinks":
- `A-CEILING` (`kDEPS`/`Q-UNANALYZABLE`) — the ⊤-bound/unanalyzable rate (dynamic constructs,
  no-oracle, non-deterministic reads) is low enough to leave a useful skippable fraction.
- `A-VALUE` (`kPROBING`) — a non-trivial fraction of real mutating ops is `expensive-apply ∧
  shallow-check` (the VALUE band).
- `A-INERT` (`kFAIL-withhold`) — the read-only probe projection provably never mutates.
- `A-ORACLE` (`effort-allocation`) — the top ~40–50 command/module classes are cheaply, shallowly,
  hermetically oracle-authorable, and the community supplies the long tail.
- `A-HAZARD` (CFG correctness) — real-shell hazards (`set -e`/`trap`/redirection/subshell) are
  soundly modelable; "the engine is engineering, not research."
- `A-FLAT` (`kCONTEXT`) — guards are flat system-state predicates, so context-insensitive analysis
  both suffices and stays polynomial.
- `A-WIN` (perf boundary) — on its best cases Dorc actually beats `ansible --check`+tags and `pdsh`.
- `A-PARSE` (`Q-PARSE`) — a strict-superset parser cleanly covers ~all real scripts. *(Already
  de-risked: note 80 first-party 0% hard-fail / 1.5% ERROR-node.)*

Non-code-testable, flagged-and-set-aside: `N-MARKET` (niche exists/reachable), `N-MAGIC` (best-effort
UX is desirable not off-putting), `N-PRIORITY`, `N-CEDE` (orchestrator cedable — values half), `N-LANG`.

## 2. The kill-criteria menu (as proposed) — compressed
Each criterion specified BUILD / OBSERVE / KILLS / SURVIVES / EFFORT / DECISIVE and was placed on a
DECISIVENESS-vs-EFFORT frontier. The proposed non-dominated set was **KC-CEILING, KC-VALUE,
KC-INERT-a, KC-ORACLE, KC-INERT-b** (corpus-stat go/no-go + a probe-soundness vertical slice +
oracle-authorability), with KC-HAZARD/KC-FLAT/KC-WIN dominated. **This frontier did not survive the
author's input** — see §5 (the corpus-stat KCs were mis-priced) and §3 (three assumptions dissolved).

## 3. The decomposition — three threats dissolved or recategorized
The highest-value output of the session: the assumption list was **over-decomposed**, and the author's
pushback (mostly correct, with refinements) collapsed it.

### 3a. `A-CEILING` dissolves — it was the shadow of two other threats
The clean intuition (the author asked for this one slowly; it had "kept popping up" because it was
never cleanly separable). Every mutating step is in one of three epistemic buckets:
1. **knowable-skippable** — analysis + cheap probe *prove* "already-done / irrelevant" → skip. The
   only value-creating bucket.
2. **knowable-must-run** — proven to need running → run. Same behaviour as just running it.
3. **unknowable (= `A-CEILING`)** — analysis can't form an opinion → must conservatively run; can't
   safely probe.
Bucket-3 is load-bearing not just as dead-weight but because it **poisons downstream**: if step 5 is
unknowable and step 9's skippability depends on step 5's effect, step 9 inherits the ⊤. A modest
ceiling rate can have outsized reach when blind steps sit dependency-upstream. *That cascade* is what
"caps everything downstream" means.

But ⊤ has three disjoint *causes*, and each routes elsewhere:
- `ceiling-dynamic` (`eval`, `source "$dyn"`, dynamic names) → *total* poison (assume-touches-all),
  but **measured tiny** (note 80: eval 0.1%, dynamic-name 1.4%). Worst kind, empirically rare. Done.
- `ceiling-no-oracle` (unrecognized command) → *scoped* poison; its rate is `1 − oracle-coverage` =
  **this is `A-ORACLE`**, not an independent world-fact.
- `ceiling-nondet` (guard depends on clock/`$RANDOM`/network/unwritten-file) → a guard that depends on
  volatile state is *by definition not a shallow hermetic check*, so it lands on the **deep** side of
  `A-VALUE`'s check-depth axis = **the hardest sub-case of `A-VALUE`**, wearing a different hat. Its
  *dangerous* face (mis-classifying volatile as hermetic → wrong skip) is welded shut by `kVOLATILES`
  + `kFAIL-perform`, which route it back to "oracle authors must declare hermeticity" = `A-ORACLE` again.

+SURE conclusion: **`A-CEILING` is not a fourth threat.** It is `{dynamic: measured-tiny}` ∪
`{no-oracle = A-ORACLE}` ∪ `{nondet = the worst corner of A-VALUE, soundness-welded}`. Retire it as a
separate line item; it is a projection of the two real threats the author already ranks highest.

### 3b. `A-INERT` recategorizes — build-quality gate, not design-falsifier
The design *delegates* inertness to oracle authors ("…as long as your oracles don't mutate" — DESIGN
Priorities #1; Sensitivities #1). So a violation means "we shipped a bug," not "the design is wrong."
You cannot kill the *architecture* via `A-INERT`; you can only fail to implement it well. It is a
**calibration-harness gate** (`kVERIFY`), not a kill-criterion. Author concurs: "must-not-fuck-up, but
not a must-be-great-to-survive… contract-bounded by 'if given perfect oracles', hence low-priority."

### 3c. `A-HAZARD` dissolves — parse-reject (`=A-PARSE`) + a silent-mismodel residue
Author dismisses via the reserved knob: "Dorc is a shell-*lookalike*, not a dash instance; I reserve
the right to reject pathological scripts; minimize the cliff, don't eliminate the long tail." Correct —
that dissolves the *parse* side into `A-PARSE` (visible, graceful rejection). **The residue that does
NOT dissolve** (author's own refinement, → TODO): a script that *parses* but whose CFG silently
*under-models* a control-flow-altering construct (`set -e`/`trap`/`pipefail`, esp. conditional) is
*accepted* and yields a *wrong skip* — an elision-soundness bug that never announces itself. The teeth:
acceptance must gate on **modeling-completeness, not parse-success** (under-modeled ⇒ strongly reject /
⊤-poison, never silent best-effort), and the calibration harness must hammer `set -e`/`trap`/redirection
reachability against a dash/bash differential. So `A-HAZARD` has no independent content *provided* the
silent-mismodel residue is filed under elision-soundness (= the kernel quality of `A-INERT`).

### 3d. `A-FLAT` down-ranks — discipline-managed, not measurement-gated
The k-CFA cliff is detonated *only* by closures recombining captured variables across call-contexts
(Van Horn–Mairson EXPTIME ≡ encoding computation into closure-env nesting; note 71/54). Dorc's abstract
state is a **flat fact-map** (`pkg:nginx→installed@1.2`), read not captured-and-re-applied → the
polynomial regime (Might–Smaragdakis–Van Horn k-CFA-paradox). +SUSPECT `A-FLAT` is "valuably true"
(author's flat examples: 'docker installed?', 'in apt-cache?', 'online?', 'sudo ok?'). Crucially it
needs **no spike**: the `kCONTEXT` redline is *start at k=0* (insensitive → always polynomial, cliff
physically unreachable) and add context only on a *confirmed-flat* pattern. Flatness becomes a live
question only if shipping shows k=0 losing valuable skips (a precision-recovery question answerable
in-practice). So `A-FLAT` drops below the author's #3 ranking — and, importantly, is **immune to the
measurement-is-hard problem** that sank the stats spike (§5). The constructs that *break* flatness are
largely the same `eval`/dynamic ones that break `A-CEILING` → same root, same reassuring measurement.

### 3e. `A-PARSE` — de-risked, done. No change.

## 4. The `A-VALUE` crux (conceded; now the canonical DESIGN kill)
The one place the pre-mortem pushed *against* the author's down-ranking, using his own words. In his
nit he *disowned* "the UX falls out of the machinery anyway" as a tool-justification ("not as a
justification for the whole tool existing — hence nit, acked"); then in his threat-ranking he placed
`A-VALUE` at #4 *because of exactly that* ("aligned-incentives development… a great UX naturally falls
out"). Inconsistent: if UX-falls-out doesn't justify the tool (his ack), then `A-VALUE` being false is
*not* cushioned, and can't be #4-because-UX.

The sharp version (the **value-locus** question): is the value the **analyzer** (skip-value; DESIGN
component #2 "the thing") or the **UX** (provenance/plan-diff/greyed-out lines/good errors)? Most of the
UX needs *no effect oracles* and *no effect-lattice* — greying-out *by git-diff* (cheap, unsound)
delivers most of the *felt* "tool-confidence" win for ~5% of the engineering, with zero oracles. So the
real question `A-VALUE` poses is not "is there skip-value" but **"is *sound* elision (analyzer)
worth-more-than *cheap* elision (git-diff), enough to justify the cathedral and the `A-ORACLE`
existential risk?"** Consequence the author hadn't followed through: **resolving toward UX-locus largely
slays `A-ORACLE`** (his #1 dread) — he was carrying the dread of the expensive path while crediting the
value to the cheap path.

Author's response: "Acked on A-VALUE, you're right. My only solid argument against it is vibes; take
that into account — I *feel* it strongly, but acked that I *feel* it." He baked `A-VALUE` into
`DESIGN.md` as a **Sensitivities** entry (#2, "subtle traits of real-world ops habits") — now the
canonical kill-listing, framed correctly as a *bet* with a stated collapse-condition. (Pre-mortem
review nits, both spike-watches not design-edits: the DESIGN sensitivity hedges *analyzability* but
underweights *band-economics* — analyzable-but-all-cheap is still a value-failure it would score a
"win"; and it captures oracle *quality* not oracle *supply*, which lives in the network-effect para.)

## 5. Effort reassessment — the corpus go/no-go is not a cheap spike (new information)
The pre-mortem rated `KC-VALUE`/`KC-CEILING` as **S–M / "days"**. That measured the **instrument build**
(which note 80 confirms *was* cheap — 10k files/3.7s), not the **trustworthy direction-setting verdict**,
which is research-grade. The author reports empirically: a real attempt (the `~/shell-iac-corpus-study`
+ adversarial-prompting/clean-room/blind-multimodel work, `[81]`) cost a week+ and produced
science-theatre risk, not direction. Corrected: building the scanner is **S**; producing a band number
you'd bet the project on is **L and epistemically capped** — an order of magnitude past "just try it,"
for non-scientists.

The diagnosis (why it failed — not "science is hard"): ~SUSPECT **a precise, world-representative
VALUE-band is `N-MARKET` in disguise — a non-code-testable assumption tried as a code test.** "What
fraction of *the world's* ops is expensive∧shallow" is a question about a population most of which is
private and unreachable (note 81's Kalliamvakou: GitHub ~71% personal, ~46% inactive). The
classification-rule subjectivity (note 80 §7) was the *visible* failure; *representativeness* was the
*fatal* one, and no pre-registration/adversarial-rule-set fixes "this corpus isn't the world." The
honest move: measure the band on a corpus you *don't* claim is world-representative — your own dogfood
set — as an **existence-proof** ("value for my profile"), explicitly *not* a market estimate. The
`contrast-not-compound` rule (charter §3) was correctly scoped to an *unbiased market statistic* and is
*wrong* for a dogfood existence-proof, whose whole purpose is "is there value for someone like me" — the
actual early go/no-go (the target *is* the author-and-similar; DESIGN).

## 6. The `N-MARKET` refinement (author's sharpening — accepted)
The pre-mortem said "accept the target-market is you, stop measuring." Author: right conclusion, *wrong
reason*. For a **network-effect-bounded** tool, market-pattern awareness *is* engineering — it feeds the
oracle-bootstrap ranking, the contributor-incentive design (`kBURDEN`, `effort-allocation`, the
oracle-distribution SEAM, `kDEPS`), and "prepare for tools I'll hit someday / rope in donors for oracles
I don't yet use." "How-network-effect-bounded" is itself a knob the market-swipe tunes. So:
- **Kill:** the precise quantitative VALUE-band as a go/no-go number (representativeness-doomed).
- **Keep:** the qualitative market-pattern *swipe* (inference-feeding, ecosystem-design input).
+SURE this *converges* with the kept-cheap-counts recommendation: oracle-frequency ranking is exactly the
ecosystem-feeding work. The corrected *reason* matters for downstream agents: market reaches design via
the oracle-supply network-effect, **not** "the author is the only user that matters." `N-MARKET` is
therefore *not* cleanly "set aside, irrelevant to the build" — it is non-code-testable as a *number* but
a live, deep *design* input via the oracle ecosystem.

## 7. The live threats, after decomposition
What remains genuinely architecture-level (and what the first implementation should attack):
1. `A-ORACLE` (author's #1; existential) — supply (network-effect) × quality (contract-following) ×
   writability. Note 80's "idempotency is module-native, not guard-native" (`creates/removes`≈0.1%)
   already nudges `kDEPS` oracle-heavy. **Partly within control** via DX-tooling leverage (DESIGN
   component #5 — "the thing I have most personal leverage over"), which the author under-credits when
   ranking it "low-control."
2. `A-VALUE` (now DESIGN-canonical kill; defended only by vibes, acked) — and its sharper form, the
   sound-vs-cheap-elision value question.
3. `A-WIN` (perf; author's #2; "ouchie perf ouchie; no path forward but to try").
4. `A-FLAT` (discipline-managed; below the author's #3).
Dissolved/recategorized: `A-CEILING` (§3a), `A-HAZARD` (§3c), `A-INERT` (§3b → calibration gate),
`A-PARSE` (done). Set-aside, eyes-open: `N-MARKET` (conscious founder bet; §6).

## 8. The elision-soundness hazard (→ `TODO.md`)
Surfaced in §3c, dropped into TODO at the author's request. Recorded here so the root is traceable: the
*parsing-danger-not-reasoning-danger* framing — `set -e`/`trap`/`pipefail` are ordinary syntax that
parses fine, so the trap is *accept-and-under-model*, not *reject-weird-syntax*; acceptance must gate on
modeling-completeness. This is the subtlest correctness trap and the easiest to let slip.

## Cross-refs
`[80]` (first-party tally; the cheap-counts that *do* pay), `[81]` (why the stats spike fails;
representativeness), `KNOBS.md` (`kPROBING`/`kDEPS`/`kCONTEXT`/`kFAIL`/`kVOLATILES`/`effort-allocation`),
`DESIGN.md` Sensitivities (the `A-VALUE` kill-listing), `plans/088-implementation-strategy-advisory.md` (the
forward-looking output this note grounds), `plans/083-synthesis-and-spike-charter.md` §3 (the corpus
question-set this reassessment re-scopes).
