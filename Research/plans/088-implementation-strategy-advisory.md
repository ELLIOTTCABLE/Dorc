# Implementation strategy — advisory (falsification-first scope-down)

> **Status (2026-06-01): ADVISORY, not a phased plan.** This is *not* the implementer-structured
> phase breakdown — that re-fold ("domains-of-research → phases-of-implementation") is still a
> future job for a later synthesis agent (charter §9.5). This document is **input to that agent**:
> what the kill-criteria session learned about project-killers, about the limits of upfront
> analysis, and about how to scope the *first* code so it sets direction / risks killing the
> project cheaply — as opposed to "Making Quality Software the Normal Way." It **reframes the
> build-order** of `021-phase-1-static-analysis-engine.md` (which is engine-first, quality-engineering)
> and re-scopes the corpus go/no-go of `083-synthesis-and-spike-charter.md` §3. Reasoning roots:
> `notes/087-kill-criteria-critique-and-scope-down.md`. Confidence markers throughout; design
> tensions by `KNOBS.md` slug.

## 0. Read this before you phase the work
The existing plans describe a *good engine* (`analysis-architecture`, `phase-1`) and a *good corpus
study* (`charter` §3, `corpus-classification-validation`). Both are correct as designs and both are
the wrong *first move*. The pre-mortem + the author's pushback (note 82) converged on a different
opening: **the first implementation's job is to produce a go/no-go signal, not to be the foundation of
the eventual tool.** Everything below serves that. The live project-killers it targets, ranked
(note 82 §7): `A-ORACLE` (existential; supply×quality×writability), `A-VALUE` (now the canonical
`DESIGN.md` Sensitivities kill-listing; defended only by vibes), `A-WIN` (perf), `A-FLAT`
(discipline-managed, needs no spike).

## 1. The philosophy: build to kill, not to keep
+SURE the single most important reframe. Quality-software instinct says: pick the language, set up CI,
build the lossless IR, model the full hazard set, then layer analyses. That is *retrofit-correct and
direction-blind* — it spends weeks before the project can fail. Falsification-first inverts it: **find
the cheapest artifact whose observed behaviour would tell you "stop / redesign," and build only that.**
Concretely, three disciplines for the first code:
- **Disposable until proven.** Assume the first slice is a probe you throw away. (It may get kept — but
  *deciding to keep* is a later call, not a v1 constraint. Don't pay quality-tax on code whose job is
  to answer one question.)
- **Dogfood over population.** The early go/no-go is "is there value *for the author's profile*"
  (existence-proof on real personal ops), not "what is the value *in the world*" (a market statistic
  that is `N-MARKET` in disguise and empirically un-answerable by non-scientists — note 82 §5).
- **One slice, several threats.** Prefer the artifact that simultaneously pressures `A-VALUE`, `A-WIN`,
  and `A-ORACLE` (that's `do-4`) over three separate studies.

## 2. `do-1` (free, upstream, NOT code): resolve the value-locus
Before any spike, settle: **is the value the analyzer (sound skip-value) or the UX (provenance /
plan-diff / greyed-out lines / good errors / single-language no-cliff)?** This costs an afternoon of
honest thought and re-scopes everything downstream:
- **UX-locus** → most UX needs *no effect oracles and no effect-lattice* (greying-out by git-diff is
  cheap and unsound but delivers most of the *felt* win). Under this reading **`A-ORACLE` — the #1
  dread — mostly evaporates**, and the tool you're building is smaller, cheaper, lower-risk.
- **Analyzer-locus** (DESIGN component #2, "the thing") → then `A-VALUE` is load-bearing and **may not
  be down-ranked behind a UX argument** (the author disowned that cushion; note 82 §4). The cathedral
  and the `A-ORACLE` risk are real and must be earned.
The author's live position is analyzer-ish-but-eyes-open (he baked `A-VALUE` as a conscious *bet*). That
is a fine place to stand — *if* the bet is then tested cheaply, which is `do-2`.

## 3. `do-2` (the build-order inversion): cheap-elision first, let the bite earn the analyzer
~SUSPECT the highest-leverage de-risking on the board, and the way to *test the `A-VALUE` bet without
building the cathedral*. The sound vs cheap elision distinction (note 75's input-addressed vs
content-addressed; note 82 §4):
- **Cheap elision** = grey-out / skip *by git-diff + syntactic dependency* — no probe, no effect
  oracles, unsound (might grey a line that actually needed to run).
- **Sound elision** = grey-out / skip *by probe-proven already-done* — the expensive analyzer + oracle
  machinery.
The sequence: **build cheap-elision first, dogfood it, and watch whether its unsoundness ever actually
bites** (a greyed line that needed to run, on your real ops). If it bites, *that pain is the earned,
empirical justification for the analyzer.* If it never bites, you have shipped something useful and
dodged the cathedral *and* the `A-ORACLE` existential risk. This inverts `phase-1`'s engine-first order:
the analyzer should be *pulled into existence by observed need*, not pushed as foundation. (Gate this on
`do-1`: under pure analyzer-locus you may skip straight to `do-4`; under UX-locus or hybrid, `do-2`
first is +SURE the right order.)

## 4. `do-4` (the one real spike), spelled out
The thin vertical slice — the minimal end-to-end Dorc that produces a direction-setting signal. Built to
the §1 disciplines (disposable-until-proven, dogfood, multi-threat).

**BUILD — smallest end-to-end path, no more:**
- *Parser:* reuse the already-vendored `tools/corpus` tree-sitter-bash (wasm, committed). Do **not**
  open the parser-strategy/language lock-in question (`phase-2 §A`) — that's not what this spike tests;
  tree-sitter is throwaway-fine here.
- *CFG:* build over the parse, but only model the control-flow constructs your dogfood scripts *actually
  use*; **⊤-reject everything else** (the elision-soundness discipline, `do-5`/TODO — acceptance gates
  on modeling-completeness, not parse-success). Do not build the full hazard set up front.
- *Oracles:* hand-author ~10 by note-80 normalized frequency (`apt`/`package`, `file`, `service`,
  `template`, `copy`, `get_url`, `systemd`, `command`, `lineinfile`, `stat`) as hermetic effect-class +
  shallow check (`kVOLATILES`).
- *Probe/apply:* emit the read-only probe projection (`kFAIL-withhold`), run it, emit the still-needed
  mutation set as a Terraform-style plan/diff.
- *Executor:* thin/mocked — local containers or ssh-to-one-box. **Do not** build async fan-out / the
  scheduler / the rich knobs (all SEAM/reserved; `orchestration-go-no-go`). The under-investment trap
  (note 73) is real but it is *not* what this spike tests.
- *Harness:* container fixtures for three scenarios — cold host, converged host (re-run), one-line-change
  re-apply. (This is the `kVERIFY` calibration harness in seed form; keep it.)

**OBSERVE / MEASURE — three threats at once:**
- `A-VALUE` (existence-proof): on your *own* real ops + ~a dozen curated public roles
  (geerlingguy / `home-ops`, already acquired in `resolved.lock`), does Dorc skip a *meaningful,
  expensive* fraction on the converged re-run and the one-line-change? Watch the *felt* value too (does
  greying-out unrelated lines actually help). **Honest scope: existence-proof for the author's profile,
  NOT a market band.**
- `A-WIN` (in-practice): wallclock vs `ansible-playbook --check`+tags and vs `pdsh 'script.sh'` on the
  same three scenarios. Does Dorc win where it claims (re-apply / one-change)? Is probe overhead
  net-positive vs `pdsh` on *expensive* ops (and net-negative on cheap ones, as expected)?
- `A-ORACLE` (difficulty-floor): how hard was each oracle to write *correctly and hermetically*? Which
  classes resisted a shallow check (forced deep/daemon-mediated)? Treat as a **floor** on difficulty —
  early oracles are noisily-hard (author: writing-them-extra-early is poor signal for mid-term ease, which
  *tooling* is meant to improve) — but if even the top-10 are brutal, that *is* signal.

**KILLS (any one ⇒ stop-and-rethink before scaling):** the skip feels like theatre on your *own* real ops
even at its best case (`A-VALUE`); OR Dorc loses wallclock to `pdsh` on the converged re-run (`A-WIN`);
OR the top-10 oracles can't be made shallow+hermetic without daemon-mediated depth (`A-ORACLE` →
`kDEPS` is oracle/trace-heavy and the engine is secondary).

**SURVIVES:** meaningful expensive skip on your real ops; clear win on re-apply/one-change vs Ansible;
top-10 oracles writable shallow. → proceed to phase the real engine, *now* with an earned mandate.

**EFFORT:** M (days–couple weeks), *bounded by design.* The point is direction-setting, not
foundation-quality. If it's sprawling past two weeks, you are building the cathedral — stop.

## 5. `do-3`: keep the cheap corpus counts, kill the precise band
- **Keep (cheap, trustworthy, ecosystem-feeding):** the mechanical `tools/corpus` counts — oracle/command
  frequency ranking (the `A-ORACLE` bootstrap order) and the dynamic-construct rate (done: eval 0.1%).
  +SURE these *do* pay, and per the `N-MARKET` refinement (note 82 §6) they are the *qualitative
  market-pattern swipe* that feeds oracle-ecosystem engineering for a network-effect tool — keep them for
  that reason, not just bootstrap-ordering.
- **Kill (expensive, epistemically capped):** the precise, world-representative VALUE-band statistic as a
  go/no-go number. It is `N-MARKET` in disguise (representativeness, not classification, is the fatal
  problem); a non-scientist team cannot make it direction-setting without science-theatre (note 82 §5;
  `[81]`). Do **not** re-run it. The band question is answered instead by `do-4`'s dogfood existence-proof.

## 6. `do-5`: discipline, not spikes (no measurement, immune to the §5 problem)
- `A-FLAT` (`kCONTEXT`): **default context-insensitive (k=0)** — always polynomial, the EXPTIME cliff is
  physically unreachable. Add context *only* on a confirmed-flat pattern, and only if shipping shows k=0
  losing valuable skips. No upfront flatness measurement.
- `A-INERT` (`kFAIL-withhold`): a **calibration-harness gate**, not a go/no-go. The probe-projection
  must be differential-tested to never mutate — but its failure is a *bug to fix*, not a *design to
  abandon* (the design delegates inertness to oracle authors; note 82 §3b).
- **elision-soundness / the `set -e` acceptance gate** (→ `TODO.md`): control-flow-altering constructs are
  a *parsing/acceptance* danger, not just a reasoning one. Acceptance gates on modeling-completeness;
  under-modeled ⇒ strongly reject / ⊤-poison, never silent best-effort. Hammer `set -e`/`trap`/redirection
  reachability in the harness from the start.

## 7. What NOT to do (the anti-goals)
- ✗ Re-run the precise corpus band statistic (proven expensive + science-theatre-prone; `do-3`).
- ✗ Build the engine cathedral (`phase-1` empty-dir→engine) as quality-software *before* the `do-4`
  go/no-go. Earn the mandate first.
- ✗ Let "the UX falls out anyway" justify the analyzer (the author disowned this; note 82 §4). If the
  value is the UX, build the UX tool (`do-2`) and discover whether the analyzer is even wanted.
- ✗ Build the executor's rich knobs / async fan-out / scheduler in v1 (SEAM/reserved; the under-investment
  trap is a *later* fight, not a *first* one).
- ✗ Open the language/parser lock-in (`phase-2`) during the spike — orthogonal to what it tests.

## 8. Controllability footnote (for the phasing agent)
The author ranks `A-ORACLE` #1 partly because it feels *low-control* (community uptake). That conflates
dread with probability. Per DESIGN component #5, oracle-ease is precisely **where the author has the most
leverage** (DX tooling: oracle-author containers, test-harness, CI, linters/LSP). So `A-ORACLE` is more
controllable than the dread implies — *if* the DX-tooling leverage is invested. The phasing should treat
"make oracles easy to write well" as a first-class workstream, not a long-tail afterthought; it is the
lever on the #1 existential risk.

## 9. Relationship to existing plans (supersession pointers, additive)
- ⟢ `021-phase-1-static-analysis-engine.md` — its *build-order* (engine-first, corpus-Step-−1-gate) is
  reframed: the go/no-go gate is `do-4` (dogfood vertical slice), not a corpus statistic, and the order is
  `do-2` (cheap-elision-first) where the value-locus isn't pure-analyzer. The engine *design* stands; its
  *primacy as the first build* does not (consistent with the charter's own "engine-primacy provisional").
- ⟢ `083-synthesis-and-spike-charter.md` §3 — the corpus question-set remains valid *as questions*, but the
  precise VALUE-band as a go/no-go *number* is downgraded (`do-3`/note 82 §5); the band is answered by
  `do-4` dogfood instead. `contrast-not-compound` (§3) is correctly scoped to a market statistic and does
  **not** forbid dogfooding the author's own ops for an existence-proof.
- `086-corpus-classification-validation.md` — its pre-register/sensitivity/adversarial machinery is the right
  rigor *if* a corpus band is ever computed; this advisory says don't compute it as the go/no-go. Keep the
  instrument, shelve the precise-band ambition.
