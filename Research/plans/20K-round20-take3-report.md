# 20K — Round-20 (take-3) report: the value-plane spike, delivered and graded

> What this is. The durable round-close synthesis (the 16P/16Q analogue for round-20),
> written by the orchestrating agent at the human's direction. Evidence basis: the full
> notes series (201–20J), and — unusually — TWO independent hostile audits of the delivered
> state (the charter-adherence pair, reconciled in 20I), each of which reran the suites and
> drove its own scratch books through the real binary. Trust the root
> README/DESIGN/IMPLEMENTATION/KNOBS and the human rulings over this; confidence-marked.
> HEAD at writing: abfbddf. Suite: 52/52 e2e (zero xfail, six gates), 250+ unit/integration
> tests, all four lint gates clean.

## §1 Outcome against the charter (19H built / 19I graded)

**19H §1 (the value analysis) — REAL.** `analysis::value` is a genuine flow-sensitive
constant/parameter propagation over the existing worklist (entry-⊤-seed; joins; scope
containment; lvalue-builtin clobbers; POSIX §2.9.1 prefix-env ordering; ⊤-region havoc;
non-convergence⇒all-⊤), and `oracle::check` is a concrete evaluator tracing resolved argv
through the oracle's OWN argparse to its inline kind-annotation. The two are joined at the
argv boundary (the recorded shape-deviation from "one machinery": dataflow for books,
concrete evaluation for the constrained dialect — 202 §1/203 §4; both audits graded it
within charter latitude). find-3 (engine-side argparse) is deleted; both audits
independently confirmed no fallback path exists: **"identity is declared, never inferred"
is true in code.** Floor under-shoots are recorded + strawman-pinned (no book loops, no
sourcing, no partial-⊤ argv, book-function params deferred — 209 maps the road).

**19H §2 (check() contract-lifting) — entity half real; probe-body half a recorded
retreat.** Entity/verb/kind resolution through the dialect evaluator is wired and
e2e-witnessed. The chartered "check body IS the shipped probe" unification was retreated
from under st-2 (probes remain separate per-(kind,selector) declarations; the check
resolves identity only; `Resolved.probe_body` computed-and-unused) — revisitable at
dq-kOOB, where the oracle-spelling ruling lives. The retreat's consequences were themselves
chased down: the per-selector probe gap it left became 20I find-1 (a live under-execute,
since FIXED structurally — see §4), and rule-anno-render is discharged-as-moot until any
emitter ships check-body spans.

**19H §3 / one-Observable — complete in shape, honestly labeled.** One `Observable` tuple
over {Effect, Status, Stdout, Stderr}; stdout/stderr are `Predicted<OutClaim>` with nothing
producing values yet; the channel rename (`StatusRelaxable`/`StatusRenderFloor`) executed
the evidence-backed resolution of 19G's deferred bake-and-see (four sources vs one ⇒ the
axis is render-expressibility). 19H §1.5's fold *domain-widening* (rc → named values) is
explicitly NOT delivered — deferred via 209 brk-4 (the probe, not the engine, is the future
source of value-bearing observables).

**The 19I axes, as graded by the hostile audits:**
- **entity — re-grounded, real.** Scratch-verified twice over; the corpus's resolution path
  is the genuine mechanism end to end.
- **rc/observable — Query-only-real by construction.** The injection lanes (stdin `rc=N`,
  `declared-rc`) are dead at the grammar level; the ONLY production mint of a fold-usable
  status is a valid Query site's record (firewall pinned in three directions; establish-rc
  withheld unconditionally — the probe's rc is never the mutator's). Honest caveats: the
  fold-demonstration cases ride builtin guards (permanently parity-opted-out) plus the new
  shimmed-fold case; the one enforced bare-Query elision is the unruled
  tc-query-bare-elision behavior (awaiting the human).
- **convergence — 29/38→(now 31/40-ish post-task-P) enforced parity; second-order
  grounding.** The probe artifact is runnable and self-reporting; gate-1 executes it under
  mocks and asserts record parity + vouch-closure; shims are entity-branched host-state
  (non-tautological, corruption-proven). Honest framing both audits insisted on: the engine
  still consumes the fixture; parity proves the probe WOULD produce it — the live wire
  (executed-probe output feeding the apply directly) is cm-1's job, deferred-with-reason.
- **render-exec — strengthened beyond any prior round.** Six gates (dash -n both artifacts,
  apply-exec, probe-exec, redirect sandbox, stderr floor, ordered run-set, argv-echo
  differential), each demonstrated to fail on a planted perturbation; the T14 render xfail
  converted to a passing requirement via leaf-exact case-arm substitution; zero xfail at
  HEAD.

**Both audits' shared verdict: the round did not repeat 16Q's ap-1/ap-2 — it inverted
them.** Keystone first (landed in the round's first third), harness afterward, and the
strain-record honest enough that the adversarial audit's planned findings kept turning out
to already be in the notes.

## §2 Rulings ledger (made or executed this round; authority noted)

- **rul-mutation-impossible** (human): mutation-analysis of arbitrary commands is
  impossible permanently; probe-safety is structural vouching only; PLT totalism vocabulary
  is harmful here. (201 §1.)
- **rul-toctou** (human): probe→apply staleness is deferred-to-actively-WONTFIX. (201 §1.)
- **C-3 executed** (human ruling from 19A, found violated by the inherited engine via the
  adversarial durables-crosscheck): errexit and `$?` are ordinary status-consumers. Engine
  now marks both (the value-relaxing channel); the pre-ruling vouch is purged from code and
  docs. (205 §2, 206, task-E.)
- **fork-mutator-rc adopted** (human's lean, executed): a mutator's rc has no sanctioned
  source; ⊤ ⇒ runs. Composed with C-3: **mutator-elision inside `set -e` books is gone**
  (the headline 6→0), with the value-story relocated to Query-guard folds. Recovery doors
  remain the human's (a conformance declaration as a block-lifter-only signal, or richer
  probes). (201/205/206.)
- **st-1** (orchestrator, delegated): short-form kinds ARE the kind this spike; reverse-DNS
  is the deferred namespacing design. **st-2** (orchestrator, delegated): probes stay
  separate declarations; the dialect does not grow pipelines; check() resolves identity
  only — revisit at dq-kOOB. **st-3** (orchestrator, delegated): pristine-prefix Query
  validity, refined: upstream queries/pure-builtins don't invalidate; only an oracled
  mutator or Opaque does. (20A §4.)
- **rule-query-validity / rule-anno-render / rule-probe-exec-gate** (orchestrator, from the
  crosscheck reconciliation): the first two live in 205 §1–2 (anno-render currently moot at
  HEAD); the third is built (gate-1). (205, 20F.)
- **kELISION naming-caution** applied at root KNOBS.md (uncommitted there, pending human
  review); kUNIT/kVOLATILES "skip"-wording residuals flagged, untouched. (201 §1.)
- **Flagged, awaiting the human:** tc-query-bare-elision (substitute an unconsumed valid
  read — sound, possibly over-eager); tc-perselector-wrapper-scheme (uniform
  `<kind>_<selector>__check` naming); tc-pipe-ran-order (pipe-stage concurrency vs the
  ordered run-set gate — latent flake, design fix sketched in 20J); the errexit/YOLO-mode
  product tension (207 — his own reflection, with the incentive-gradient finding and the
  inline-oracle/prefix-env-pun directions).

## §3 The soundness story (the round's center of gravity)

Five disaster-class (wrong-elision-licensing) defects were found and fixed this round, every
one by a crosscheck pair, none by tests or review:
1. **Errexit-vouch** (inherited; violated C-3): converged non-conforming establishes under
   `set -e` elided to `true`, hiding aborts. Fixed by marking errexit/`$?` consumption.
2. **Prefix-env argv visibility** (new code, accepted in review): POSIX §2.9.1 backwards —
   driven end-to-end to a wrong elision. Fixed; pinned with the citation.
3. **`${N#pat}` glob-vs-literal** (+ the latent unmodeled-`${…}`-as-literal sibling): wrong
   concretes vs dash semantics. Fixed via `Word::Unmodeled` (fails every position).
4. **The lvalue-builtin family** (`read`/`export=`/`readonly=`): variable mutation unmodeled
   while effect-Pure — stale concretes elided runtime-determined installs. Fixed via
   clobber-to-⊤ family handling.
5. **Per-selector under-probe** (st-2 consequence; 20I find-1): `#enabled` convergence
   minted by an `is-active` probe — a live under-execute inside the enforced-parity set.
   Fixed structurally (per-(kind,selector) probe declarations; multi-selector kinds without
   them are un-probeable ⇒ run — F-BLESSED enforced by `resolve_probe`).
The pattern (20A's diagnosis): every break was in precision-ADDING code or confident prose;
the degrade spine (⊤⇒run) held under every attack. The countermeasure program is partly
landed: **cm-2** (the argv-echo differential, gate-5 — dash adjudicates ~60 value-plane
resolutions per run) and **cm-3** (the single shell-semantics module `syntax::sem`,
human-frontloaded; its extraction surfaced four latent representational divergences) are
built; **cm-1** (the differential product-gate, bare-vs-apply run-set against the license
ledger, exhaustive over modeled host-states) is THE deferred item — deferred-with-reason
(the human's isolation-tier pricing), urgency raised by the gate-5 disposition carve-out,
and the top harness item for the next round. The measuring stick itself was caught lying
twice (dead-engine-blesses-green; the vacuous headline lane) and fixed.

## §4 What the next round inherits (the take-4 charter material)

- **cm-1**, msys-tier (no new infra needed for the corpus population): the one gate that
  observes elided sites; everything else now exists for it (runnable probes, ordered
  run-sets, dispositions in `--debug-argv`).
- **The 209 enrichment roadmap**, value-ordered: book `for`-loops over literal lists
  (parser+CFG back-edges + a Powerset loop-domain + the elide-list-MEMBERS render
  direction), deliberate word-splitting, budget-bounded function inlining (which the
  inline-oracle/wrapper-pun direction in 207 §4 requires), partial-⊤ argv (operand-only
  holes), parameter-expansion operators on knowns — all enrichments of the same substrate,
  to be built on `syntax::sem`, with gate-5/cm-1 as the safety net the roadmap previously
  lacked.
- **The fold domain-widening** (19H §1.5, deferred): value-bearing observables arrive via
  the probe (Query stdout), not engine synthesis.
- **dq-kOOB** (human): the oracle-spelling ruling, which now also owns the
  one-vs-two-declarations question (st-2's retreat), the annotation form, and the
  prefix-env-pun (`DORC_DRY=--dry-run cmd`) and wrapper-function inline-oracle directions.
- **Cross-kind dependency edges**: the precise fix for pristine-prefix's coarseness, the
  recursive-algebra direction, and the shared prerequisite of the human's (downgraded)
  timed-probe reservation.
- **Residual honest gaps**: convergence grounding is parity-mediated until cm-1; the Query
  fold's e2e evidence rides the new shimmed-fold case plus opted-out builtin-guard cases;
  the if-guard render floor stands until a guard-capable leaf-exact render.

## §5 Process & dispatch heuristics (the meta-goal deliverables)

- **Opus held on every build task** (zero Fable graduations needed): E/W/D1/S/D2/D3a/D3b/O/P,
  sizes 240k–490k tokens. The 490k task-W datum taught the split-lesson (wire/Query/gates
  ran as three slices thereafter, each cleaner). Briefs that worked: SAFETY verbatim at
  top, a read-list with note-slugs, the API contract pinned, golden-discipline stated as
  stop-and-flag, tc-* flag-up-don't-resolve, and a RESERVED strain-note slug (the slug
  collision happened twice before reservation became protocol).
- **Fable crosscheck pairs were the round's highest value-per-token spend** (~6 pairs,
  ~170k–330k per agent): every disaster-class find came from one; the neutral+adversarial
  COMPARISON repeatedly mattered (the neutral relayed the very example the adversarial
  broke; convergent finds were the most trustworthy). Values/identity-frontloaded light
  briefs (the human's guidance) outperformed checklist briefs.
- **Verify-don't-relay paid off** repeatedly: agent reports were honest but RA diagnostics
  flagged false alarms twice, and my own review caught the ⊤-region havoc hole in task-A's
  delivery before the crosscheck did.
- **Calibration correction adopted mid-round** (205 §5): my +SUREs over same-day design
  prose ran hot (three were refuted); +SUREs over traced code held. Standing rule: +SURE on
  an untested mechanism requires a strawman traced against real fixture vocabulary.
- **Minor brief-compliance drift to tighten**: task-E used `git mv`; task-P used
  `git checkout --` to restore a race-flipped golden (correct outcome, wrong tool) — the
  no-index-ops language now needs "no working-tree-restore ops either; report and let the
  orchestrator restore".

## §6 Seeding feedback (for the human's next priming prompt)

- fb-1 (201): the copy-vs-cleanroom ambiguity in the seeding instructions cost a real
  adjudication cycle; one sentence would have fixed it.
- fb-2 (201): the hk/relativeWorktrees hook breakage cost a diagnostic loop; pre-stating
  known-broken tooling (or fixing repo-side) saves every future round the same loop.
- fb-3 (new): the round's two biggest mid-flight redirections — C-3 having been silently
  violated by the inherited engine, and the headline books' set-e composition — were both
  DISCOVERABLE from the corpus + rulings at seed time; a one-line "known tensions to check
  first" list in the priming prompt (e.g. "19I group F vs 19A C-3 disagree — resolve
  early") would have moved task-E to the round's start and avoided building the matrix
  re-pins twice.
- fb-4: the delegated-judgment pattern (st-1/2/3: "make the call, record it, I overrule
  cheaply") worked well and is worth making explicit earlier; the judgments held under both
  audits.

## §7 Note index (the round's full strain corpus)

201 gate+rulings+seeding · 202 input-side design (+supersessions) · 203 timed-probe
reservation + stage-1 (+addendum) · 204 check-dialect build · 205 crosscheck rulings ·
206 C-3/headline cost · 207 errexit incentive + inline-oracle directions · 208 task-W
wiring (+corrections) · 209 value-plane breakdown map · 20A whack-a-mole + st-judgments ·
20B pairs reconciliation · 20C probe wire · 20D semantics module · 20E Query class ·
20F harness gates · 20G corpus+render · 20H one-Observable · 20I charter reconciliation ·
20J per-selector repairs · 20L round-close pointer.
