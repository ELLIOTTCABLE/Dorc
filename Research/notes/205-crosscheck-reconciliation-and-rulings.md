# 205 — Crosscheck reconciliation: the pair's findings, my traces, and the resulting rulings

> Round-20. A full neutral+adversarial pair (Fable-class, clean contexts, identity/values
> frontloaded per the human's direction) ran against the round's durables (201/202/203,
> spike/CLAUDE.md, commit 808931f) at HEAD=f957656, mid-round — pulled forward from the planned
> post-keystone slot at the human's suggestion. This note reconciles them; convergence between
> the passes is the trusted signal, adversarial-only claims were re-traced by the orchestrator
> before adoption (trace-don't-relay). AI-authored; the C-3 citation below is the human's ruling
> verbatim from 19A.

## §0 Outcome in one paragraph

The test-surgery and process artifacts survived both passes (the adversarial: "the most
trustworthy artifact in the set"). The damage concentrates in MY design prose (202/203/CLAUDE.md
§rulings): three paths by which a wrong *concrete* value enters the fold — corrupted-probe rc
(annotation shipped verbatim), stale cross-kind guard rc (the Query gate is same-cell-only), and
fabricated rc-0 through unmarked status-consumers (errexit/`$?`) — all three invisible to the
harness for one shared reason: **no gate ever executes a probe body under a real shell**. One of
the three additionally contradicts a standing human ruling (19A C-3). All are now ruled (below)
and scheduled; none had reached built code except the third (inherited from spike-2, live since
`set` was blessed Pure).

## §1 Convergent findings (both passes independently) — adopted as rulings

- **find-annotation-ships** (= neutral gap-1): the inline annotation inside a *shipped* check()
  body is not inert under dash — `pkg : Kind = "$1"` PATH-executes `pkg`, never binds `$pkg`,
  and the probe then produces wrong *concrete* observables (the no-floor class), while `dash
  -n`, the apply-only exec gate, and the fact-level hostsim are all structurally blind.
  **rule-anno-render**: the probe-artifact *emitter* renders the annotation node as a plain
  assignment (`pkg="$1"`). This is not the banned off-ramp strip/transpile pass — ch-shape-anno
  governs oracle *files* run without Dorc; the probe artifact is Dorc's own rendering, and
  emitting executable sh from the dialect AST is the emitter's job. 202 §4's "no strip pass"
  sentence is corrected by this note to carry that distinction. Lands in task-D; its test is
  the probe-exec gate below.
- **the probe-execution blindspot** (= neutral gap-2 + the adversarial's unifying observation):
  nothing specifies what *executes* a probe in the acceptance loop, so stage-2 could re-key the
  lane without re-grounding it (the 19I §3 trap in better clothes).
  **rule-probe-exec-gate**: `e2e/run.sh` gains a probe-side exec-under-mocks gate — the
  rendered probe runs under `PATH=mocks-only` with check-command shims (`dpkg-query`,
  `getent`, …) that emit scripted outputs; the harness asserts the produced results-lane
  records (and the run-set, kFAIL-withhold-style: a probe artifact invoking an un-shimmed
  command fails loud). This also gives us a real (if coarse) dialect-vs-dash differential for
  the shipped bodies (us-dialect-fidelity's practical mitigation). Lands in task-D, before any
  case is re-grounded.

## §2 Adversarial-only, orchestrator-traced and CONFIRMED

- **find-stale-crosskind** (+SURE after my own trace): 202 §2's staleness gate is sound only
  same-cell. Its own motivating example fails as written: `apt-get purge nginx` kills
  `package:nginx#installed`; the guard's Query cell is `tool:nginx#present` — different kind, no
  edge (and `inv-referent-agnostic` forbids bridging on the shared token). An *oracled* upstream
  mutator therefore leaves a cross-kind-dependent guard's probe-time rc "valid", and the fold
  elides the install the purge just made necessary — under-execute in the flagship idiom, opened
  precisely by oracle coverage (the 196 §2 law: every elision-power increment surfaces
  wrong-elisions the poison wall was masking). My 202 example even used a kind-cell vocabulary
  (`#exists`) that matches no fixture — written, not traced. Mea culpa; the +SURE was wrong.
  **rule-query-validity** (interim, conservative): a Query's probed rc is fold-valid only when
  NO effect-bearing command (oracled mutator on ANY cell, or Opaque) reaches it from entry —
  the "pristine-prefix" rule, implementable as one reaching-defs bit. This keeps the dominant
  idiom (top-of-book guards before any mutation) and surrenders guard-folds after any mutator
  (incl. after `apt-get update`, which costs real elisions in update-first books — recorded,
  priced, accepted for the round). The precise fix is cross-kind dependency knowledge — an
  oracle-declared inter-kind edge (`tool:X#present` depends-on `package:X#installed`), which is
  the recursive-algebra / kind-embedding direction and ALSO what dir-timed-probe's
  entanglement-ordering presupposes; deferred with that linkage recorded.
- **find-errexit-fabrication** (+SURE; verified against the source ruling): 19A §3 C-3 (human):
  "errexit is honored, not special-cased-as-vouched … gw-1's 'errexit-status stays vouched,
  still elides' carries the identical rc-vouch unsoundness"; 19A §5 (human-ruled corrected
  model): "errexit isn't special either". The committed engine special-cases it (consumption
  marking deliberately skips errexit; `f1_status_consumed_by_errexit_stays_vouched` +
  `exec-errexit-elide-vouched` pin the vouch), and 19I group F wrote the pre-ruling shape into
  "acceptance" — an AI-authored doc the trust-order subordinates to 19A. C-3 called the error
  *latent* (set -e then poisoned everything); the committed engine blesses `set` Pure, so it is
  *live*: a converged NON-conforming establish under `set -e` is replaced by `true`, hiding the
  abort a real run raises; `$?` is a second unmarked consumer with priority-1 exposure
  (`mkdir x; [ $? -ne 0 ] && recover` — fold suppresses `recover`).
  **rule-errexit-honored** (executing C-3 + C-4-as-refined; task-E): consumption marking gains
  (a) errexit-region commands' status marked with the value-relaxing channel (the AndOrStatus
  semantics: known/probe-sourced rc ⇒ exact-substitution OK; ⊤ rc ⇒ run) — NOT the
  unconditional-block if-guard channel; (b) a `$?`-bearing command marks its CFG-predecessor
  command(s)' status the same way. Costs, priced honestly: under fork-mutator-rc a mutator's
  status is always ⊤, so **mutator-elision inside `set -e` regions dies** — and both headline
  books open with `set -e`, so the keystone's "elides 6 mutations" headline regresses until
  Query-guard folds (probe-sourced rcs, unaffected) carry the value instead. That arithmetic is
  the two human rulings composed (C-3 × no-declared-mutator-values); the recovery doors are the
  human's, not mine: a conformance *declaration* used strictly as a block-lifter would be
  May-grade-shaped (it could only restore an elision the Effect channel already licenses —
  but it IS a declared value; his call), or richer probe-sourced evidence someday. The stage-1
  module-doc sentence I wrote ("the vouch survives ONLY for the errexit consumer") gets
  corrected in task-E's commit, along with the two stale matrix comments the adversarial found
  ("set -e poisons / never reaches the status question" — refuted by the file's own tests).

## §3 Adversarial-only — adopted with adjustments

- **dev-reflexive**: real. My CLAUDE.md mutation-impossibility paragraph read as banning even
  the cheap vouch-closure check (refuse to ship a probe body containing a call that is neither
  the self-vouched command, a declared Query, nor blessed-pure). Carved back in CLAUDE.md;
  owed in task-D (task-C's strain-6 independently flagged `eval`-as-command-word inside check
  bodies as exactly this gate's job — convergence noted).
- **dev-groupE-render**: real inconsistency. The if-guard unconditional block cannot be
  superseded without the leaf-exact render, which no task owned, while the xfail
  (`render-case-arm-oneliner-wrong`) was promised must-pass. Resolution: the render-fidelity
  fix (case-arm leaf-exact commenting) joins task-D's scope; if-guard supersession is
  explicitly DEFERRED past this round (Query folds arrive for `&&`/`||` and errexit consumers;
  `if`-guards keep the safe block) — 202 §2's "supersedes group E" is corrected by this note
  to "supersedes the `&&`/`||` half; the if-half waits on render".
- **dev-floor-undershoot**: acknowledged as an aggregate: book-`while`, `. /path` sourcing, and
  partial-⊤ argv are each recorded-deferred but together under-shoot 19H §1.4's floor, and the
  corpus cannot see it (no loop/sourcing cases). Disposition: stands as recorded debt; one
  strawman e2e case per waiver (asserting the SAFE degrade, xfail-style where value is absent)
  goes into task-D's corpus additions so the stick at least measures the boundary. (19H §1.4's
  own "while … already in the apply fold's reach" was charter-internal error — `while` never
  was; noted for the human's eventual 19H errata.)
- **us-effectmap**: real footgun — `KindIndex.effects` silently clobbers a second
  `oracle_effect` for the same (provider, verb), and one-cell-per-verb blocks multi-cell verbs.
  task-W: duplicate ⇒ loud diagnostic; value becomes a small Vec (multi-cell verbs are real:
  `purge` kills `#installed` and dirties `#config`-ish cells eventually).
- **us-sure-drift / Exit(n) coverage**: post-surgery, `StandIn::Exit(n)`'s path has zero test
  coverage; it revives with Query rcs (`command -v` absent ⇒ 126/127). task-D acceptance adds a
  nonzero-guard-rc case. The +SURE-calibration datum joins §5.
- **us-standing-rule**: softened in CLAUDE.md — site-keyed stays the default shape (it is
  independently right for C-1/full-argv semantics), but it is not a weld, and `kSTATE`'s
  fact-keyed reuse pole is named as the counterweight; the human's same-day confidence
  downgrade governs.

## §4 Corrections-by-reference (append-only discipline; the cited notes stay as written)

- 201 §0: "one doc-fix (§2)" should read §1. — 201 §3's "a fixture literally cannot inject one"
  overclaims: the provenance type guards *engine-internal* fabrication; lane-feeding fixtures
  mint provenance by construction (test-discipline covers that half).
- 202 §2: the staleness example's cell vocabulary (`tool:nginx#exists`) should be `#present`,
  and the example itself is cross-kind-invalid per §2 above (rule-query-validity replaces the
  same-cell-ambient rule). 202 §3's closing "(check-id, channel, value)" should read
  **site-id** (the bullet above it is correct; flaw-3) — now also an inv in CLAUDE.md.
- 203 §2 was superseded ~minutes later by the repo-wide hook uninstall (5a5978e).
- Fixture comments citing "201 §4" for the vouch-wrong cut should cite 203 §3 (queued code-fix,
  rides task-E's re-bless).
- 19I group F is hereby marked stale-against-19A-C-3 (the trust-order already subordinates it;
  this note is the greppable pointer).

## §5 Calibration (the meta-goal deliverable)

The pair earned its cost twice over (two class-1 finds + one human-ruling conflict, all
pre-build). The specific lesson: my +SUREs over *design prose I authored the same day* ran hot
— 202 §2's staleness-sufficiency, 203 §1's "today's sound floor", 201 §5's "+SURE on scope" —
while +SUREs over *traced code* held. Standing correction for the rest of the round: a +SURE on
an untested design mechanism requires either a strawman traced against the actual fixture
vocabulary or a downgrade to ~SUSPECT. Also adopted: the orchestrator reserves note-slugs at
dispatch time (task-C took 204 in good faith; collision was luck-free but the protocol was
underspecified).

## §6 Sequencing out of this note

task-E (errexit/`$?` consumption + re-pins + headline re-bless + stale-comment fixes) →
task-W (wiring: find-3 removal, classify on value-plane + check-resolution, effect-map
dup/multi-cell, 19H §2.1-corrected fixture oracles, kind-agreement lint per task-C's open seam)
→ crosscheck #1 (propagation-correctness, now incl. opaque-region clobbering & the dialect↔dash
divergence list) → task-D (probe-projection + rule-anno-render + rule-probe-exec-gate +
rule-query-validity + vouch-closure + render-fidelity xfail + Exit(n) coverage + stage-2
de-cruft + floor-boundary strawmen).
