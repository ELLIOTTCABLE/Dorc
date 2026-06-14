# 224 — r22 GATE-1: orientation record + synthesis claims

> Round-22 conductor, first note (2026-06-11). Append-only. Orientation per the 223
> priming prompt's ordered list; GATE-1 synthesis presented in-chat (this note is the
> crash-resilient record of its claims + flags, NOT the explainer). Awaiting human go
> for PHASE-R at time of writing.

## §1 Verified state

- Worktree `.claude/worktrees/spike3`, branch `ai/spike3`, HEAD `ada085d` ("Quarantine
  and rewrite the r22 prompt"), clean tree at session start.
- Full gate chain run by conductor on the inherited tree, real exit codes, BEFORE any
  edit: `cargo build --workspace` ok (warm) · `fmt --check` ok · `clippy -D warnings`
  ok · `cargo deny` bans/licenses/sources ok · `cargo test --workspace` **463 passed /
  0 failed / 1 pre-existing ignore** (re-run unpiped after the first pass tripped
  fb-17's own `| tail` masking trap — totals match 21W §10 exactly) · `sh e2e/run.sh`
  ×2 **96/96 all seven gates, both runs**, EXIT-asserted · `mise x -- typos spike`
  clean. Note: bare `cargo` resolved fine (global mise); spike/CLAUDE.md's
  `mise exec --` form remains the canonical invocation for builders.
- spike/CLAUDE.md drift noticed (not edited yet): says "43-case corpus" — corpus is 96
  dirs at HEAD; the file is conductor-updatable per the priming prompt and 19I's
  count-the-dirs rule already covers it. Low-priority fix when first touching it.
- SyncThing: per human memory-note 2026-06-11, the whole Sync folder is disabled on
  this PC — fb-9b ghost-husk risk paused, not solved; conflict cleanup stays human-owned.

## §2 Reading completed (order per priming prompt)

README · DESIGN · IMPLEMENTATION · KNOBS · TODO · TODO-ADDTL · STALENESS-AUDIT ·
AGENTS (in-context) · spike/CLAUDE.md · plans/21W · plans/21Z · plans/111 · notes/220
(full) · notes/222 (full) · notes/21G §§1–5 · notes/21K. Not read prospectively:
ANALYZER-NEEDS, notes/110/112/113, 21L/21N/218/218a/219, crate CLAUDE.mds — per-need.

## §3 GATE-1 synthesis claims (compact; chat carries the explainers)

- g1-1 one-way rule: licenses are Must-side facts mintable ONLY from oracle-claims
  (capability-style, vp-27); receipts are may-side metadata; `ProvId → License`
  non-constructible mirrors the existing Must→May one-way coercion. The "may REFUSE or
  EXPLAIN" phrasing states the tolerable DIRECTION of influence; the BUILT discipline
  is the strictest point on it — zero influence — and the erasability gate (strip
  receipts plane → re-run → verdict-identical) is what makes that testable. Any future
  receipt-consuming refusal must be re-derived as a fact first (a deliberate
  human-gated weld change), never read off the receipt. CI-cheap from commit one
  because the engine is a pure function (inv-determinism) — the gate is "run twice,
  with/without capture, diff dispositions" — and rq-2 already proved the
  trivial-now-load-bearing-later pattern.
- g1-2 in-engine-or-nowhere: Newt (external MySQL lineage store, per Titian) = 86×,
  DNF at 500 GB; one virtual call per tuple = >10× (Smoke); in-engine = 1.3–3×.
  Receipts live in the analyzer's own hot loop and structures (Smoke P1/P4) or nowhere;
  packaging predictor (vp-22): fork/sidecar dies, plane-inside-host survives.
- g1-3 no-size-cliff: rustc's 4-byte Span was SLOWER than 8-byte (interner overflow at
  10–20% miss); field widths measured, `lo` kept u32 so no cliff at big crates.
  Import the meta-lessons (measure first, common-case inline, graceful k-cap with
  truncation marker, licenses exempt), NOT the bit-packing — at Dorc scale (-GUESS,
  220 §6) the memory knob sits ~2 orders below biting.
- g1-4 formalism mapping: lineage (flat k-capped origin set, on every abstract value)
  → "who contributed to this ⊤" (dashboard blame, dedup). Stored-witness (why-unit,
  exactly the granted minimal conjunction, at licenses only) → "what licensed this
  substitution" (CHERI-intentionality: elide only on the cited witness). Alternatives
  structure → retraction survival — NOT stored; re-derivation covers it,
  over-invalidation is kFAIL-perform-safe. Full how (ℕ[X]) answers NO Dorc question;
  don't reserve. Where-provenance = the existing Span/SpanEdit/loc-* plane; separate
  plane because propagation laws differ (copying vs logical dependence); fusing
  recreates 111's coarsest-tier composition loss.
- g1-5 stress-test of "mapping complete" (220 §3): the one place multiplicity-shaped
  data could appear is per-iteration observables in modeled loops (task-L1 literal-list
  for/while) — and StatusIterated's unconditional block + the in-loop Query-probe
  exclusion are exactly what keep that question un-askable. So the mapping holds
  BECAUSE of the refusal postures: if in-loop elision is ever relaxed, the
  how-provenance question REOPENS (deferred-not-irrelevant; tripwire registered).
- g1-6 catalog inversion at HEAD: diag.rs catalog = 5 codes, all Note; the 17 scattered
  codes include every error-severity gate-3-tripping code. Layer-1's target population
  (give-up paths) is exactly the UNcatalogued one. Retrofit order: mechanical 17-code
  move FIRST (no behavior change), then the Pottier-direction gate
  (give-up-path ⇒ registered) — a completeness gate over the Note-island asserts
  nothing. Per-code DECLARED severity rides the retrofit (tc-fix3); the s-2
  classify-signature widening sequences EARLY (3 span-None catalog notes + arch-2
  seam-1 both gate on it); report() rendering spans = cheapest visible win (drop-A).
- g1-7 hostsim Finding: second free-text vocabulary (DST-judge products). Lean
  ~SUSPECT: formally OUT of the catalog for now; boundary rule = user-surface
  reachability decides membership; revisit when the oracle-author calibration harness
  (222 m-4 / DESIGN #5) productizes findings to authors. Human rules at GATE-2.
- g1-8 dac-B: receipts hang on the analyzer's OWN derivation graph (its CFG/dataflow/
  fold edges are the provenance edges); a second graph = two drifting sources of truth
  — explanations describing a derivation that isn't the one that produced the verdicts.

## §4 Flags raised at GATE-1 (chat carries full text)

- f-1 erasability-gate spec boundary: "verdict-identical" needs one decision — propose
  plan-artifact bytes + license/disposition ledger + error-class diagnostics identical;
  receipt-rendered explanation payloads exempt. Also the g1-1 REFUSE-vs-zero-influence
  reconciliation made explicit in the gate's doc.
- f-2 (+SURE hazard, ~SUSPECT first bite): `Top(cause)` must keep cause OUT of value
  Eq/Ord/hash — Top(a) ≡ Top(b) in the lattice — or the reshape itself makes receipts
  load-bearing via BTreeMap keying/join order (find-2's scar at the type level). Spec
  into arch-1's contract before any builder starts.
- f-3 ⊤-absorption choice is user-visible: first-cause ⇒ onion-peeling UX (fix one,
  re-run, next surfaces); lean store-k-capped-join + render-root-cause-only via
  suppression rules (store structure, render late).
- f-4 erasability ≠ trace-stability: arch-4's golden-TRACE fixtures additionally need
  receipt-plane determinism/churn economics (rq-C) — two different properties, two
  different gates.
- f-5 cosmetic: 220 vp-2's trailing "(vp-15)" cross-ref looks like it means §6's scale
  paragraph.
- f-6 kSTATE fence restated for arch-4: the d-1 dump is a write-only durable LOG
  (grep/`why` material); anything that re-INGESTS receipts across runs crosses the
  parked knob.
- rq additions proposed for PHASE-R: rq-F metadata-inertness prior art (debug-info-
  must-not-affect-codegen discipline, reproducible-builds; informs the erasability
  gate's spec); rq-G fleet error-grouping/fingerprinting (Sentry/WER/Socorro; serves
  the north star's fleet-aggregable clause + site-key stability at fleet scale).

## §5 Process log

- Dispatches: none yet (no subagents; gate chain run as a conductor background shell).
- Token log: n/a this note.
- Commits this note covers: this file only, pathspec-scoped.
- Chain green end-to-end at HEAD `ada085d`; fb-17 near-repeat logged: the conductor's
  own first chain piped `cargo test | tail` (masked rc/totals) — caught and re-run
  unmasked before any green claim. Keep the canonical chain literal, no pipes.

## §6 Queue at time of writing

GATE-1 presented, awaiting go → then: PHASE-R (interactive-research, rq-A..E ± F/G) ∥
warm-ups (d×d host-flip fixture; var-resolved redirect case) → 22x synthesis → GATE-2
→ ratified arcs. 22Z resumption prompt starts once PHASE-R holds state worth resuming.

---

## §7 GATE-1 rulings (human, in-session, 2026-06-11; appended at human direction —
## "write anything there… no reason to waste an entire slug")

- ru-1 (g1-3): premature-optimization caution CONFIRMED; exception named — optimize
  early only when likely AND its ergonomic/safety consequences make early spike-mapping
  design-direction-finding.
- ru-2: human wants an idiot-proof slow explainer of "no full why-provenance" —
  deferred deliverable, "when waiting"; queued as a task.
- ru-3 (g1-4): the why/where plane division RATIFIED — "definitely encode that clearly
  into the typings." Carries into arch-1's contract.
- ru-4 (g1-6): dedicate research to error-DISCIPLINE TOOLING ("how we keep ourselves
  disciplined"), Pottier-sense expanded across languages/domains; CI-triggered
  protections around error-handling/edge-cases/warnings/provenance. Registered as rq-H.
- ru-5 (g1-7): gentle human lean IN (one error system; DST tooling likely exposes to
  oracle-authors); not married. GATE-2 default flips to in-catalog unless evidence
  pushes back.
- ru-6 (f-3): ratified store-most-data + leave-UX-doors-open (k-capped join store,
  render-late). Pushback: both my framings were implementation-axes; the render should
  key on what's most useful to the admin — suspects a missing axis as direct proxy.
  Conductor candidate (chat): REMEDIATION-CLASS — classify origins by what user action
  clears them (author-oracle / add-declaration / fix-book / structural), rank renders
  by that; the dashboard's four-cause decomposition generalized per-site. Season-to-
  taste acknowledged.
- ru-7 (f-4): human leans AWAY from promising trace-stability upfront (~harness
  convenience, not user benefit); needs a concrete user-story to buy it. rq-C brief
  reframed to skepticism (evidence for OR against; default no-promise).
- ru-8 (rq-F): EXTREMELY hard yes — reproducible-builds literature gets a dedicated
  research agent and synthesis attention.
- ru-9 (rq-G): approved (o11y angle).
- ru-10: parallelization GO — warm-up fixtures + research fan-out (~4 Opus research
  agents primed with the interactive-research brief); wall-resilience required (commit
  durables early/often; no gigantic lose-it-all tasks). g1-1/f-1 explainer demanded
  slow-and-thorough, frontloaded (delivered in chat this turn). All rulings direct
  in-session (no [spike]/[product] marker needed).
- ru-11 (human, post-explainer; WELD): receipts are FULLY one-way — the "may REFUSE"
  allowance is OMITTED from the principle itself; neither allow nor reject may be
  affected by explanation-plane data, ever. Any receipt-prompted trigger must be
  re-derived as a first-class fact ("hard agree… welded"). Consequence: the
  erasability gate's decision assertion is strict equality permanently; any future
  receipt-influenced behavior is a WELD re-litigation, not a default loosening.
  Closes ask-zero-influence.
- ru-12 (human, post-explainer): the matching partition is AUTH'D with two
  corrections. (a) Naming honesty — stop saying "identical" when exemptions exist;
  naming delegated to conductor → ADOPTED: "identity plane" (byte-exact) vs "exempt
  plane" (closed enum of named reasons, per R4'/LLVM); the gate asserts
  IDENTITY-EXACT under strip+variance. (b) Artifact floor — comment text inside
  shipped .sh artifacts is NOT exempt: "shipped .sh artifacts are byte-identical" is
  the minimum floor; droppable stability promises live above it (separate --verbose
  output, temp .log files, why/dump channels). INVERSION AUTH'D: do not ship
  unstable tracing/receipt-derived data into finished artifacts in default modes.
  Last-resort sidecar shape if artifact-adjacent receipts ever become genuinely
  necessary (stable per-line ID-comment + <artifact>.sh.log mapping IDs→receipts) —
  human explicitly dislikes it; do not pursue unless necessary. Closes
  ask-partition + ask-comments.
- ru-13 (human, post-synthesis): full-engine-rerun-to-fixpoint is a likely
  attempted-if-not-fully-built path — wanted ALSO to avoid back-propagation /
  phase-fusing in the analyzer generally, not just for receipts-retraction. Effect:
  the no-full-why bet (lineage + witness-at-licenses, retraction-by-recompute) rides
  an engine mechanism that is planned anyway; the receipts plane stays forward-built/
  backward-queried with no feedback edges; and the erasability gate's run-twice shape
  matches the engine's native mode (synergy: the gate's marginal harness cost drops).
  Conductor watch-item (-GUESS): if rerun-to-fixpoint becomes the change-handling
  story, the per-tier epoch vector (220 vp-8) demotes from invalidation-machinery to
  a pure dashboard/why hint — don't build it load-bearing.
- ru-16 (human; GATE-2 PASSED): need-2 (hostsim Finding in-catalog), need-3 (retire
  arch-5 into arch-4's tail — approved "(sad)"; the projection-sidecar door stays
  reserved for the o11y future), need-4 (verdicts-everywhere, no trace-pinning),
  need-5 (third d×d fixture), need-6 (build GO, ordering arch-1 → s-2-early/arch-3 →
  arch-2 → arch-4-thin) all APPROVED. need-1 (severity): human leans severity-as-TYPE
  (capability-gated minting: "need an elision-site-license in-hand to mint a
  CRITICAL-tier error"), vibes-grade, defers to literature. Conductor resolution
  (proceeding under it; human veto window open): FACTOR the two axes — (a) treatment
  severity = registry-declared data + un-overridable floor tier (the surveyed
  consensus; no system gates TIERS by capability, and tier-gating polices severity
  INFLATION, the safe/noisy direction, while deflation — the dangerous one — is
  policed by completeness+expect, not constructors); (b) the human's evidence
  instinct lands structurally anyway: catalog codes are exhaustive-enum variants
  whose TYPED PAYLOADS demand exactly the objects the diagnostic cites (a
  probe-blaming error takes a ProbeRecord; license-adjacent errors take the
  site/license evidence) — make-bad-states-unrepresentable, near-zero cost at
  legitimate sites, impossible at fabricating sites. Builders flag-up any emit site
  where the natural evidence is NOT in hand (design smell, surfaced not threaded).
  ~SUSPECT on the asymmetry argument; revisit at retrofit time if payload-typing
  alone doesn't give the constraint-feel wanted.
- WAVE-1 DISPATCHED (post-GATE-2, ru-15-lean briefs): B1 arch-1 (arena + Top(cause) +
  erasability gate + unord-newtype + canary + digest) → worktree b1-arch1, branch
  ai/r22-arch1 @ 69c21ab. B2 third d×d fixture (outer-live × inner-diverged, 215 §5)
  → worktree b2-fixture3, branch ai/r22-fixture3 @ 69c21ab. Crosscheck x-1 (hostile
  pass on the gate, Fable) queued for post-B1-harvest per ru-14; x-2 queued at
  arch-2; x-3 demoted to B-brief hunt-list item.
- ru-17 (human; scope direction): add a first-draft of the BATTLEFIELD-BOUND
  structured diagnostic API to this spike — "could become The Product of the spike,
  and one of the first bits of actual code to write for the real codebase"; crib
  rustc (spans/labels/suggestions/applicability API shape) and Elm (rendered
  narrative philosophy) explicitly. Conductor integration: a design note FIRST
  (notes/22B, dispatched, parallel-safe to B1), then it becomes wave-2's spine —
  the 17-code retrofit doubles as the new API's proving ground. Guardrails carried
  into the design brief: the Fluent-regret friction test (adding a code = ~one
  edit, no DSL, no multi-file mandate), ru-16 typed payloads, ru-12 out-of-artifact
  receipt rendering. Tension stated (spike charter vs battlefield-bound): the spike
  PROVES the design; the real codebase extracts/reimplements — diag is a leaf
  module, extraction is cheap; we design-for-keeps without violating
  the-spike-is-disposable.
- ru-18 (human; process concern + direction): DST/re-runability just became
  user-facing, design-promise-fulfilling components ("help you be defensive +
  save pain when oracles go wrong" is foundational). Consequences adopted:
  (a) the probe-tape format is product surface, not test plumbing — design
  attention + versioning; (b) arch-4 gains a REPLAY GATE as its first test (run
  live recording the tape → re-run from tape → assert identity-plane identical;
  the erasability gate's sibling, proves the postmortem promise mechanically);
  (c) the deferred wish "figure out what happened with verbose OFF" returns as:
  capture-always-lean / render-on-demand — verbose becomes a DISPLAY knob, never a
  capture knob (~SUSPECT affordable at Dorc scale: probe outputs are small reads,
  O(sites×hosts) per run, human-timescale ops — NOT Bazel-scale; needs a cheap
  cost-measurement in arch-4 before welding); (d) retention = local rotated
  last-N-runs artifacts; Dorc is NEVER a postmortem-reliability-class log/trace
  ACCEPTOR — the OTel value-format/projection seam is the off-ramp for real
  retention (ship spans to YOUR collector), which upgrades that seam from
  tail-item to postmortem-story-adjacent (the need-3 "(sad)" gets its consolation).
  Secret-scrub-at-capture and durable-locally-first (fate-sharing) carry as
  constraints from 227.
- ru-19 (human, completing ru-18's thought): graduating DST/re-runability to
  product tier demands corresponding CI/testing ceremony — "the testing-value
  floors out at the runtime-value of the-thing-being-tested." Adopted, with the
  two distinctions that keep need-4 (no trace-pinning) standing: (dist-1)
  SELF-CONSISTENCY testing vs STORED-GOLDEN testing — the replay gate regenerates
  both sides every run (live ≡ replay-from-tape), so nothing is keyed to a stored
  artifact that can drift (dodges the entire plan-forcing rot class); (dist-2)
  same-inputs/same-binary trace DETERMINISM (needed by the runtime promise; given
  by inv-determinism; now TESTED) vs cross-code-change trace STABILITY (the
  expensive normalization-burdened promise — still correctly un-made; a postmortem
  replays the run as recorded, never across versions). The arch-4 ceremony set:
  cer-1 replay gate per fixture (ru-18b, the spine) · cer-2 tape format
  version-tag + binary-hash; replay REFUSES loudly on mismatch, refusal
  CI-pinned (cross-version replay explicitly not promised v1) · cer-3 secret-scrub
  sentinel test (planted token in probe output ⇒ asserted absent from durable) ·
  cer-4 tape round-trip (serialize→parse→identical) · cer-5 fold replay into the
  hostsim DST trials (each seeded trial records + replays + compares — the
  600-trial harness becomes a replay-fidelity fuzzer for free) · cer-6 a
  capture-cost budget assertion on fixtures (tape size bounded; guards accidental
  capture explosions). Stored-tape compat fixtures deferred until a format
  stability declaration (Bazel's experimental-across-7.x precedent).
- ru-14 (human, end-of-window economics): be judicious with adversarial-crosscheck /
  Fable-class spend for the remainder of the window — his intuition stands that one
  well-targeted pair at the right juncture is very high value; so fewer, sharper
  hostile passes at real junctures rather than the full inherited cadence. Conductor
  application: collapse the named crosscheck targets toward the highest-value (the
  gate itself, x-1, and over-suppression, x-2); x-3 (catalog-gate evasion) rides as
  a hunt-list item in the builder brief instead of a dedicated pass unless evidence
  demands one.
- ru-15 (human, prompt-style nit; also seeding-feedback material for round close):
  agent briefs are overly-constrained. Opus-class: drop constraints inferable from
  goals and context. Fable-class even more so: the verbatim safety block + ~a
  paragraph of prose on immediate goals + context of what's done/being-done
  elsewhere + reading pointers — not rule-lists. Adopted for all subsequent
  dispatches; candidate fb-18 at round close.
- ru-26 (human, 2026-06-12; code discipline, triggered by act-4's
  stderr-goldens-rejected lean — which he otherwise ACK'D): any
  implementation shaped by a "would churn unnecessarily" avoidance, at ANY
  point in the spike, MUST carry a nearby inline note that it was
  deliberately scoped-down for spike-specific churn reasons; such constraints
  must NEVER leak into greenfield work referencing the spike (defeats the
  spike's purpose). Live instance: anything not-handling-stderr says so
  locally and upfront. Written into spike/CLAUDE.md standing rulings.
- ru-25 (human, 2026-06-12; spike scope ADDITION): minimal-UI work added
  explicitly — his justification: the spike has not accumulated enough
  cruft/pain to make UI work unproductive or unrepresentative. ui-A: a
  fair-shape CLI over the core invocation modes discussed across the corpus
  (NOT flag-complete) such that every behavioral mode of the core is
  exercised; a vacuous multi-hostsim-in-DST is sanctioned ONLY as a driver —
  named rabbit-hole, avoid. ui-B: a very-focused STREAMING proof — the one
  feature threading every component with zero testing to date; maximally
  minimal acceptable (ANSI-sequence update emission or least-effort
  equivalent); human expects a DST timing/logical-clock dependency. Escape
  valve (human's words): if the spike grows heavy, this becomes r23.
  Continuity: ui-A ≈ ru-20's ui-3 phased-CLI shape; ui-B ≈ a minimal ui-2
  streaming slice — the contracts stay plane-based, these are consumers.
- ru-24 (human, 2026-06-12; token-economics normalized; lean wording corrected
  by the human immediately after first recording): the end-of-window burn is
  OVER; normal cost discipline applies. FABLE-tier dispatches now require
  ahead-of-time human approval with explicit per-dispatch justification.
  Human lean (gentle, not welded): reserve Fable FOR ADVERSARIAL-CROSSCHECK —
  the skill's structured pair protocol (neutral + disowned-adversarial,
  clean contexts), NOT ad-hoc lone-hostile dispatches; he has repeatedly seen
  unprompted conductors dispatch "just an adversarial" without the skill and
  considers that poor practice. Rare exceptions for extremely sensitive code
  regions. Refines ru-14 and the ru-23 three-tier discipline: the fable tier
  gains an ask-first gate, and adversarial spend defaults to the
  crosscheck-pair structure. The in-flight x-3 pair (pre-authorized, and
  already skill-shaped) is unaffected.
- fr-criticality ruling context (conductor answer, recorded): fr-1 (CACM WER paper)
  = verification garnish, drop-in-whenever; fr-2 (VMCAI'12) = drop-in-later but with
  a soft deadline — wanted before arch-2's over-suppression hostile pass (crosscheck
  x-2), since it's the formal grounding for the suppression-soundness posture (dc-7).
  Neither is go-to-the-machine-now load-bearing; the build arcs don't block on either.
- Context-practice directive (human, post-synthesis): heavy note re-reads risk
  auto-compaction (conductor at ~565k/1M at the time); prefer one-at-a-time reads
  feeding a single running synthesis. State when received: digestion already complete
  and plans/22A already written — 22A now serves as the canonical compressed
  artifact; FORWARD practice adopted: post-compaction and during the build phase,
  work from 22A + targeted Grep-slices of 225-229, never wholesale re-reads; builder
  briefs cite 22A conclusions + specific note sections, not full notes.
- Conductor analysis recorded with ru-12 (~SUSPECT, verify at arch-1 spec time): the
  EXISTING provenance_comment emitter is fact-plane-derived (edit/disposition
  ledger, not receipts), so current artifact comments sit safely INSIDE the
  byte-exact floor; what ru-12 forbids is the drift 21Z's wishlist pointed at
  (enriching artifact comments with receipt fragments). Receipt-derived explanation
  renders OUT-OF-ARTIFACT (CLI, why-query, dashboard); vp-12's
  evidence-in-the-user's-artifact is satisfied by the why-query SHOWING sh text,
  not by embedding it. 21Z note-C's dropped-disclosure hole now has a ruled
  direction: the carrier is the OOB lane/log, never fancier in-artifact comments.

## §8 PHASE-R launch (dispatch ledger v1)

- Research-dir mechanics — deviation-with-reason from the human's single-sources.json
  sketch (he marked it "idk"): per-agent dirs `.claude/research/r22-<front>/` while
  gathering (zero write-contention, per-dir validate.sh clean); conductor re-registers
  the adjudicated keep-set into a master `r22-errprov/` dir at digestion via
  new-source.sh — which doubles as the skill-mandated re-verification of
  `graded-by: subagent` grades. Agents commit their own dirs granularly.
- W1 fixtures builder (Opus, own worktree %TEMP%\dorc-r22\w1-fixtures, branch
  ai/r22-fixtures @ fa78935 verified): door1-door3-dead-block-folds (21W §4/§6 spec)
  + var-resolved redirect case (21H §9 residual-2). Zero-design pins; STOP-and-flag on
  any engine-change temptation; full unpiped gate chain before every commit; no BLESS.
- R0 (Opus): rq-A unpaywalled-primary hunt (Zdancewic–Myers CSFW'01; ACM Queue primer
  tail; Green–Tannen PODS'07; CACM-walled list) → fetch+register or hand human
  fetch-requests.
- R1 (Opus): rq-B catalog practice + rq-H discipline tooling.
- R2 (Opus): rq-C derivation-dump/why-query (f-4-skeptical) + rq-D minimal OTel.
- R3 (Opus): rq-E suppression/root-cause dedup + rq-G fleet fingerprinting.
- R4 (Opus): rq-F reproducible-builds / metadata-inertness (dedicated per ru-8).
- Token log: to be appended at harvest (agents report; fb-16).

## §9 Mid-launch re-site (two human directives, ~15min after first dispatch)

- directive-resite: research-front OUTCOMES go to `Research/notes/22x-slug.md`, one
  per front, rich ("don't compress away interesting context"); do NOT use the skill's
  `.claude/research/` siting — keep its mechanisms/practices only.
- directive-slugs: stop minting terse letter-code slugs; full-word slugs from now on
  (`finding-1`, `cost-1`, `front-1` style). Adopted conductor-wide, relayed in all
  relaunch briefs. (Prior notes keep their old slugs; append-only.)
- Execution: no agent-messaging tool exists in this session, so the five research
  agents were STOPPED (~1 turn in each) and RELAUNCHED with corrected contracts. The
  stopped agents' work is banked: four force-added commits (`acc6cbe` rqCD, `6b6dd23`
  rqBH, `85c4c3e`+`91d006c` rqEG — note `.claude/research` is GITIGNORED; those
  commits got in via -f) plus on-disk untracked scratch for rqA (all four
  previously-unreachable primaries DOWNLOADED: Zdancewic–Myers, Green–Tannen
  semirings, Livshits–Chong, Carata primer) and rqF. Relaunched agents read
  predecessor scratch as seed and write/commit ONLY their assigned corpus note.
- Assigned notes: 225 unreachable-primaries (R0') · 226 error-catalog-and-discipline-
  tooling (R1') · 227 derivation-dump-why-query-minimal-otel (R2') · 228 suppression-
  dedup-and-fleet-fingerprinting (R3') · 229 reproducible-builds-metadata-inertness
  (R4'). W1 (fixtures, own worktree) unaffected by the re-site.
- Conductor cleanup obligation at digestion: verify each front's scratch content is
  folded into its 22x note, then `git rm -r .claude/research` in one cleanup commit
  (history retains the force-added blobs; that is acceptable and append-only-safe).
- Kill-moment snapshots worth keeping (fb-15 bank): rqBH found rustc Fluent-migration
  tracking issue #132181 — first-party regret doc, deny-lints downgraded to allow,
  four named friction points. rqCD found the Bazel exec-log thread: 99GB→450MB compact
  format, 75min-vs-7min pre-optimization overhead, and the shipping maintainer "not
  sure it would ever be fine for this to always be collected by default" — direct
  caution against d-1's always-on durable. rqF mapped the full r-b.org leak-category
  page taxonomy. rqA read+graded Zdancewic–Myers and Green–Tannen before the stop.

## §10 Dispatch ledger (running; fb-16 — subagent-reported tokens)

- R0' rq-A primaries → notes/225 (final `32bac89`): Opus, ~115-135K tokens, 42 tool
  uses, ~9 min. All four primaries grade-A full-read; ZERO human fetch-requests
  blocking. Substantive corrections delivered: finding-zm-attribution — 220 vp-26's
  one-way slogan is the Sabelfeld–Sands GLOSS, not Zdancewic–Myers verbatim (ZM01's
  own Thm 4.2 carries a published self-correction, fn4: corrected bound weaker, "not
  tight") — the synthesis must cite the engineering rationale, not lean on ZM01 as a
  tight formal anchor. finding-carata-tail — SPADEv2 <10% overhead on production
  Apache; tail confirms vp-23 (noise/unbounded capture kill, not payload).
  finding-livshits — full-paper support for vp-27, devs "better off leaving out
  sanitizers entirely instead of trying to place them." Housekeeping: stale duplicate
  B-green key in rqA scratch sources.json (auto-resolves at the §9 cleanup).
- R3' rq-E+G → notes/228 (final `04606d7`): Opus, 164K harness-tokens (agent
  self-reported ~95K — harness counts are authoritative in this ledger), 59 tool uses,
  ~13 min, 18 sources, 743 lines. Design-shaping: finding-emit-at-origin — Clang's
  primary author deliberately moved AWAY from post-hoc visitor re-walking to NoteTags
  captured at transition-time ("generate the message where the info already exists");
  cuts against any emit-then-dedup design and matches the arena's
  capture-cause-at-creation shape. finding-min-suppression — 5-rule minimum set
  (carry-cause/prevent-cascade-at-origin · interestingness-from-sink pruning ·
  same-fact tie-break by speaker priority · observe-THAT-⊤-never-WHY ·
  flush-or-trip net). finding-site-key — CodeChecker's deployed stability ladder,
  sweet spot `(checker, file, enclosing-decl, whitespace-normalized line, range
  cols)`; Sentry hierarchical multi-hash (emit fine+coarse, match coarsest stable);
  WER condensing/expanding as the two failure directions. VMCAI'12 sound
  alarm-clustering = candidate formal cause-pointer prior art, capped ~SUSPECT
  pending fr-2.
- R2' rq-C+D → notes/227 (final `c526c87`): Opus, 178K harness-tokens, 56 tool uses,
  ~14 min, 780 lines. THE d-1 VERDICT (ru-7-shaped): d-1 SPLITS — the dump+`why`
  half has affirmative support (Buck2 `buck2 log` is the shipping architecture,
  ~15 lenses incl. a built-in golden-diff lens; thin-durable + recompute-on-demand is
  sound for Dorc because DST determinism lets the trace be reconstructed from
  seed+probe-tape); the golden-trace-PINNING half has only cost evidence plus a
  decade-scale regret analog — SQL plan-forcing (practitioner reversal "I do not
  recommend enabling"; key-drift rot; pins corrupting adjacent identity), Bazel
  keeping its dump format `experimental` across all of 7.x, and rustc UI-tests
  showing the real cost is the NORMALIZATION layer (and they keep human-written
  assertions so --bless can't rubber-stamp — trace-only pinning is anti-pattern even
  where pinning works). Best postmortem user-story found argues for RECEIPTS, not
  pinning (the silent-green-dashboard emergent-composition class). rq-D:
  `traceparent` trivially hand-emittable (import value-format, choose carrier);
  conduit-style edge-mapping keeps the SDK out-of-process; the OTel env-carriers
  spec (Beta) surfaced UNREAD — conductor must-read at digestion.
- W1 fixtures → branch ai/r22-fixtures, HARVESTED by cherry-pick onto ai/spike3 as
  `5da879c` (door1-door3-dead-block-folds) + `5b58c5f`
  (y1-var-resolved-target-invalidates-query): Opus, 212K harness-tokens (self-reported
  ~118K), 133 tool uses, ~27 min. Both zero-engine-change; goldens hand-derived then
  engine-confirmed; gate-6 needed NO exclusion marker on fixture-1 (the bare-only
  `dpkg -s` line is license-attributed to the guard's replace entry). Harvest
  verification: base proven code-unchanged since the verified-green `ada085d`
  (diff-empty), full chain re-run post-pick on the main tree — build/fmt/clippy/deny
  0 · tests 463/0/1-ignore (20 suites) · e2e **98/98 ×2** real exits · typos 0.
  fb-11 content-diff audit ai/spike3↔ai/r22-fixtures on spike/: EMPTY. Originals
  remain in %TEMP%\dorc-r22\w1-fixtures (human's look-don't-touch inventory).
  flag-dxd-third-cell (W1's, carried): 215 §5 labels the OPPOSITE cell (outer-live ×
  inner-DIVERGED-runs) "the d×d cell"; the corpus now brackets outer-dead-folds and
  outer-live-inner-converged-elides, but 215's labeled cell remains unauthored —
  candidate third fixture, decide at GATE-2.
- R1' rq-B+H → notes/226 (final `e2ab06b`): Opus, 179K harness-tokens, 85 tool uses,
  ~17 min, 1047 lines, 26 sources. Design-shaping: finding-gate-exists — rustc tidy's
  error_codes.rs is the registry↔emit-site cross-check to copy, but the cheapest
  Dorc spine is an exhaustive Rust enum catalog (compiler enforces handling) + a
  tidy-style grep for the reachability half; Dorc's give-up sites are nameable source
  points (structural advantage over Menhir's derived automaton states).
  finding-fluent-regret — rustc tracking #132181: deny-level authoring-mandate lints
  downgraded to allow; cheap structural gates endure, heavyweight authoring mandates
  earn hundreds of #[allow]s and die. finding-errorguaranteed-holes — ZST-minted-by-
  emit() transplants without proc-macros, BUT delayed-bugs can mint it and it carries
  no kind; type system ≈90%, end-of-run flush assertion covers the rest.
  finding-severity-fragments — every surveyed scheme drifts toward all-warnings
  unless an un-overridable tier exists (forbid/force-warn analogs); rustc `expect`
  level = a positive must-emit assertion, the severity-system form of the
  completeness wish. finding-elm-counterpole — world-class diagnostics with ZERO
  catalog machinery; a catalog buys regression-safety + multi-author consistency,
  never message quality. Menhir completeness kept green where wired into the BUILD
  GRAPH (Stan/dune; CompCert 5283-line database). Ratchet: env-side -D warnings,
  never #![deny(warnings)] (documented anti-pattern). cargo-mutants on error paths =
  adoptable-this-round.
- R4' rq-F → notes/229 (final `edf269d`): Opus, 173K harness-tokens for the top agent
  + four gathering sub-subagents (87/120/113/143K ≈ 463K; agent-reported ~520K
  all-in), 53 top-level tool uses, ~23 min, 1212 lines, 55 sources (all A/B). THE
  GATE BLUEPRINT, three upgrades to the GATE-1 f-1 proposal: (1) partition-language —
  adopt LLVM debugify's NAMED sanctioned-absence reasons over my severity-keyed
  boolean: closed `Exempt::{Explanation, ReceiptId, OriginOrdering, Timing}` enum
  assigned per-field AT THE DEFINITION SITE, gate FAILS on any field without an
  explicit assignment (new fields included-by-default = the safe direction); pair
  with canonicalize-don't-exempt for legitimately-varying compared fields.
  (2) adversarial-variance — run-B doesn't just strip receipts, it injects variance
  (reversed origin-set order, sentinel receipt IDs, varied DI'd hash seed — Debian's
  ~20-axis blueprint + sentinel canaries that make leaks self-identifying);
  DST-clean. (3) coverage-canary — the gate must prove it RAN
  (GCC_COMPARE_DEBUG=-fcompare-debug-not-overridden precedent); rot evidence says
  the failure mode is silent-no-op-while-green (the 80%-quarantine war story), and
  the gate passes the Meiklejohn two-question test (receipt-into-decision is
  invisible to decision-only tests — a class nothing else catches). Precedents: GCC
  -fcompare-debug shipped ~17 YEARS (253 tagged bugs); the Oliva doc states our
  invariant verbatim ("debug information isn't supposed to modify the executable
  code in any way whatsoever"); rustc's cautionary inverse — no in-tree gate, #75362
  open since 2020, regressed the moment it first succeeded (1.44.1→1.45.0).
  mechanism-unord-newtype (~2-4d): iteration-API-suppressed map newtype (rustc
  UnordMap precedent) turns the f-2 ordering-leak class into a COMPILE ERROR.
  Ordering = the most entangled leak category (LC_ALL=C sort). Caveat carried:
  sub-subagent-gathered B-rows are quote-trustworthy but interpretation ~SUSPECT
  pending conductor re-verification; six sources marked [self-read].
- D1 diag-structure design → notes/22B (final `29d3c78`): Opus, 146K harness-tokens,
  35 tool uses, ~8 min. Verified-against-live-source draft: exhaustive DiagCode enum
  with typed per-variant payloads (ru-16 realized); mandatory-primary SpanLabel
  (span-poverty killed by construction); Suggestion{applicability, remediation}
  (rustc Applicability verbatim); registry() with Floor{None, WarnOrDeny, Pinned};
  SiteId + GroupingKey fine/coarse SLOT; information-poor ProvId hook; small
  builder API. Refusals by name: Fluent/derive DSL, i18n, Menhir generate/compare,
  no-catalog pole, prose-goldens. Friction test answered honestly: three match-arms
  in ONE file, compiler-guided. Forks: fork-1 typed-vs-flat (disposed: typed, per
  standing ru-16; struct-count flagged at retrofit if it balloons) · fork-2
  coarse-key now-vs-stub (disposed: stub, per the design-the-slot brief) · fork-4 =
  floor membership (builder-proposes / human-disposes at the retrofit PR, as
  already slated). Process flag (correct call, noted): D1 followed spike/CLAUDE.md's
  no-Co-Authored-By rule over the generic harness instruction. Conductor reads 22B
  in full at wave-2 prep (context-practice: one-at-a-time, at need).
- PHASE-R GATHERING COMPLETE (line restored — a conductor edit accidentally consumed
  this bullet's lead-in; content unchanged). Wave totals (harness): R0' 135K ·
  R1' 179K · R2' 178K · R3' 164K · R4' 173K+~463K subs · W1 212K ≈ 1.50M, plus the
  stopped first wave (partial turns, banked scratch). All five notes committed;
  corpus 98/98 ×2 green at `5b58c5f`+notes; next = 22Z, then digestion (#9) →
  synthesis (#3) →
  GATE-2 (#4).
- B2 third d×d fixture → HARVESTED as `8421ecb` (door1-door3-inner-runs; outer-live
  × inner-diverged): Opus, 109K harness-tokens, 64 tool uses, ~17 min. Zero engine
  changes; golden hand-traced then engine-confirmed (`argv 1 replace / 2-4 run`);
  proves the doors compose without masking (door-1 keeps the block live; door-3
  still defers to Effect — Status-clear is not a license). Post-harvest chain: all
  gates rc=0, e2e 99/99 x2 unmasked, fb-11 audit EMPTY. The d-by-d bracket is now
  complete (three cells exec-pinned). Label nit: B2 used `(AI test)` vs the house
  `tests` label — preserved as-is (harvest keeps the builder series verbatim).
- CONFLICT-SWEEP WAVE (human-directed, post-ru-19): the r21/r22 ruling cascade
  (ru-11..ru-19 + GATE-2 dispositions) may have planned-in breakage against corpus
  assumptions compressed out of conductor inputs. Three Opus READ-ONLY reviewers
  dispatched on DISJOINT corpus regions, given the decision ledger but NOT the
  conductor's own conflict-candidates (held back for convergence checking):
  RV1 human-authority layer (DESIGN/KNOBS/STALENESS-AUDIT/TODO-pair/README —
  kOOB redline, dir-soundiness-ux, disclosure floor, contract-and-DX).
  RV2 transport/security/ops layer (plans/142, plans/102, 19B, 20V s5, 222 s7 —
  vs capture-always + tape + OTel-offramp). RV3 engine/charter layer (19H/19I,
  kFACTS/kSTATE, 219 forks, 21Z seams — vs rerun-to-fixpoint + receipts +
  diag-API-first). Conductor's HELD candidates: (held-1) ru-12 byte-exact/lean
  artifacts vs dir-soundiness-ux per-line IN-RENDER disclosure + 222 m-6
  render-the-assumption-where-the-human-reads; (held-2) ru-18 capture-always vs
  142's fast-lane/per-leaf-file split + 102's stderr-aggregation-as-fleet-target
  surface; (held-3, weaker) ru-13 rerun vs vp-8 epoch assumptions + bump-loop
  framing; (held-4, process) ru-17 vs spike-disposability charter —
  spike/CLAUDE.md gains the sanctioned exception at next touch.
- RV2 transport/security sweep → final-message deliverable (no files): Opus, 103K
  harness-tokens, 21 tool uses, ~3 min. Read 142/102/19B/20V/222 s5-7/21L/21N in
  full. VERDICT: no breaks-a-human-ruling; one breaks-a-recorded-assumption
  (reconciled); two friction-needs-a-sentence; rest reinforcement/false-alarm.
  CONVERGENCE with conductor's held candidates: finding-3 == held-1 (REAL: 20V s5 +
  222 m-6 want the counterfactual text in artifact comments; ru-12 evicts it) with
  the clean split = rec-1: the DISPOSITION/ATTRIBUTION line ("line 14 elided per
  package-oracle's converged-claim") is fact-plane and stays in-artifact inside the
  byte-exact floor; the COUNTERFACTUAL PAYLOAD ("assumes rc=0, 'already newest'")
  renders only on the OOB site-keyed lane + why-query (m-6's where-the-human-reads
  satisfied by the why-query). finding-1+2 == held-2 split in half: rec-2 cross-lock
  cer-6's controller tape-budget with 142's per-leaf remote size-cap residual (one
  bound, both ends; capture-always IS the workload that residual reserved against —
  re-pressures, does not break, the executor deferral); rec-3 the rotated durable
  inherits 102's crown-jewel posture (at-rest protections in-scope like key
  material; cer-3 scrub = floor not ceiling; no-exfiltrate-by-default welded; OTel
  off-ramp strictly opt-in — 102's omission rule is creds-scoped so not literally
  violated, but the at-rest aggregate is a NEW surface 102 never weighed).
  BONUS rec-4: ru-12 RESOLVES 21N's heredoc comment-drop disclosure hole by lane-
  routing (the one place artifact comments structurally can't carry disclosure is
  exactly where ru-12 says don't rely on them); m-5's muddies-plan/apply caveat
  answered the same way. watch-1: cer-2's binary-hash refusal is a hostsim-seam-tier
  property, NOT corpus-shell-exec tier (21L's documented lax-set) — deferred, track
  at arch-4. Gating-lane sizing and 19B collision-freedom: checked, untouched
  (capture rides per-leaf + verdict-triple lanes only). Reconciliation batch rec-1..4
  goes to the human after RV1/RV3 land.
- RV1 human-authority sweep → final-message deliverable (no files): Opus, 106K
  harness-tokens, 12 tool uses, ~3.5 min. Read README/DESIGN/IMPLEMENTATION/KNOBS/
  STALENESS-AUDIT/TODO-pair fresh. VERDICT: region overwhelmingly CONVERGENT; one
  breaks-a-recorded-assumption (== held-1, now CONFIRMED by two independent
  reviewers); one friction clause; one deferred-to-RV2; eight reinforcements; no
  breaks-a-human-ruling survived scrutiny. THE finding (rv1-finding-1, +SURE): the
  held-1 crux is sharper than the conductor had it — DESIGN approach-3 presents the
  PLAN "still as a simple shell-script", so if the plan-render counts as a
  "default-mode artifact" under ru-12's byte-floor, the human's dir-soundiness-ux
  per-line claimed-vs-proven disclosure is forbidden ON the surface he asked for it.
  Reconciliation rec-1-sharpened: TWO SURFACES, stated where builders read —
  the shipped/off-ramp .sh artifact is byte-floored and receipt-free; the
  PLAN-RENDER surface (TUI/CLI presentation, why-query) is NOT an artifact and is
  the sanctioned home for per-line disclosure, OVERLAID on artifact bytes, never
  embedded. rv1-finding-2 (~SUSPECT false-alarm, clause anyway): DESIGN ~180 "may
  short-term-persist probe results to reduce work on re-runs" = the kSTATE
  reuse-cache (still parked), NOT the ru-18 probe-TAPE (write-only postmortem
  durable, never re-ingested to elide work) — nominally distinct, builders could
  conflate; one clause = rec-5. Reinforcement highlights: kOOB's human
  clarification BLESSES ru-12's OOB routing; the byte-floor SERVES the off-ramp
  weld; the where/why two-plane split PRESERVES kFIDELITY's loc-DAG (watch-2:
  don't over-apply dac-B to the where-plane); owed-dst resolves
  best-effort-vs-hard-gate; IMPLEMENTATION's dictate carve-out covers the severity
  floor (residual noted: floor-tier diagnostics about USER code brush
  contract-over-dictate — concl-8 drift evidence justifies; future note).
- RV3 engine/charter sweep → final-message deliverable (no files): Opus, 132K
  harness-tokens, 31 tool uses, ~4 min. Read 19H/19I/219/21Z/21G/21K/111/220 +
  4 KNOBS entries fresh. VERDICT: no breaks-a-human-ruling; held-3 confirmed-mild
  (rv3-find-1: vp-8 epoch vector demoted by ru-13 — 22A already omits it; 220 §6
  now IB-annotated in place); TWO catches the conductor did not have:
  rv3-find-2 (~SUSPECT) fork-cmdsub-top-cause is being resolved BY BUILD-ORDERING
  toward cause-tagged (the arena makes the generic-floor/reshape split moot;
  219/21G's "cheap floor ships first" sequencing is superseded) — low-stakes,
  human confirmation wanted = rec-6. rv3-find-3 (+SURE, process): 219's fork-1..4
  and 22B's fork-1..4 are DISJOINT sets under IDENTICAL names — cross-note
  confusion hazard; fix at next 22B touch by note-scoped slugs (22B-fork-payload
  etc.) = housekeeping-3 (and a vindication of ru-15's full-word-slug nit).
  rv3-find-5 (~SUSPECT, latent for the future q-3 round): the ru-18 replay gate
  proves tape→replay determinism, NOT probe-records-match-real-execution — a
  wrong-but-self-consistent tape passes; it is a SIBLING of, not substitute for,
  219 fm-1 / 19I §3's probe-exec gate = rec-7, recorded not actioned. find-4/6/7/8
  false-alarms/reinforcements (21K d-1 was PROVISIONAL by design and cleanly
  reversed — 21Z clause IB-annotated; kFACTS/kSTATE/kPRECISION fences all honored;
  kFIDELITY needs one capture-what-vs-exec-granularity disambiguation line =
  housekeeping-4). Reinforcement highlight: ru-11 went STRICTER than 220 vp-26..29
  recommended and the base fully supports it.
- CONSOLIDATED RECONCILIATION BATCH (all three sweeps; to the human for
  ratification): rec-1 TWO-SURFACES (shipped/off-ramp artifact byte-floored
  receipt-free; plan-render surface NOT an artifact — sanctioned home for per-line
  claimed-vs-proven disclosure, overlaid never embedded) [held-1; RV1+RV2
  convergent; THE genuine collision — two human directions meeting at the
  plan-render]. rec-2 one-budget-both-ends (cer-6 = controller terminus of 142's
  per-leaf size-cap residual). rec-3 crown-jewel posture for the rotated durable
  (at-rest protections like key material; scrub=floor; no-exfiltrate-by-default
  WELDED; OTel off-ramp strictly opt-in). rec-4 lane-routing resolves 21N's
  heredoc disclosure hole (record; m-5 caveat answered). rec-5 tape ≠ kSTATE
  reuse-cache (write-only postmortem durable, never re-ingested to elide work;
  DESIGN ~180's may-short-term-persist = the PARKED cache, distinct). rec-6
  fork-cmdsub-top-cause resolved-by-ordering toward cause-tagged (confirm). rec-7
  replay-gate ≠ probe-exec-gate (recorded for the q-3 round). Housekeeping
  (conductor's, no ratification): 220+21Z IB annotations DONE this commit; 22B
  fork-slug rename at wave-2 touch; kFIDELITY disambiguation line; spike/CLAUDE.md
  gains held-4 sanctioned-exception + rec-1 two-surfaces + rec-5 at next touch.
- ru-20 (human; the four-UI enumeration — held-1/rec-1 dissolved properly): UI was
  deliberately punted ("baking it into any conversation seems dangerous — that
  leads to building correctness machinery targeting specific UI structures instead
  of vice versa"); correctness follows GRAND UX GOALS; UIs fall out. The quiet
  assumption, now enumerated (feature-parity-NOT-included): ui-1 actual-application
  mode (the "UI" is an SSH tunnel + rack lights; be very careful about mutation
  between input-script and on-wire bytes; DEFERRED-BUT-LEAN-NO on embedding much
  additional metadata as shell comments there — the existing fact-plane disposition
  comments stand, grow nothing). ui-2 TUI/pretty-mode (full ANSI UI; grey-out,
  rustc-arrow-style inserted not-really-script lines, scrollable cause lists;
  looks-like-shell-script is an aesthetic affectation THERE; realtime feedback =
  the wow-factor that converts to oracle-authors; needs significant architecture).
  ui-3 traditional phased CLI (`dorc plan` → file → `dorc apply` ships it) — THE
  home of the warnings-representation problem: sh-on-stdout dislocates
  stderr-warnings from code; human lean = direct-WRITE the script artifact, DOUBLY
  emit cited sections with their warnings/errors to the console. ui-4 mechanized
  (fixed formats, no fluff: scripting/automation/LSP/LLMs). Conductor mapping:
  still TWO PLANES (artifact vs render) with render having three modes; contracts
  stay PLANE-based per the human's own warning — the four UIs are consumers, never
  contract subjects. Happy convergence: 22B §4's one-value-four-projections maps
  ~1:1 onto ui-1..4 (artifact-comment / TUI-dashboard / CLI-narrative / OOB-lane).
  rec-1 RATIFIED in substance via this enumeration; the ui-3 cited-sections lean
  is the 22B narrative render. (rec-2/4/5/7 uncontested-recorded; rec-6 still
  awaiting an explicit nod.)
- ru-21 (human; crown-jewel counter-proposal, DIRECTION-grade — he is on the
  fence): lean = do NOT ingest output we don't need (security argument hard to
  overcome); instead take on the rotation-discipline pain of owning PER-HOST
  durables directories; postmortem reconstruction = reach out and slurp the
  relevant durables AT POSTMORTEM TIME, not prospectively; maybe with tooling.
  Conductor synthesis to propose (threading his lean against the fate-sharing
  counter): default host-side rich durables (142's per-leaf files already live
  host-side — simply don't drain by default) + NEED-DRIVEN eager fetch (on
  failure/refusal/divergence at a site, fetch THAT site's durables immediately
  while the connection lives) + `dorc postmortem <run>` slurp tooling. Controller
  keeps only the analyzer tape (verdict-lane records it consumed anyway + seed +
  digest) — RV2 finding-2's at-rest aggregate dissolves. COSTS flagged honestly:
  (cost-1, strongest) the dead-host window — the postmortem you most need (host
  went sideways/unreachable) is the one where slurping fails; need-driven fetch
  shrinks but does not close it. (cost-2) a persistent Dorc footprint on every
  host: rotation tooling + not-being-a-dick discipline = a new product commitment.
  (cost-3) scrub moves to INGEST (host-side sh can't scrub cheaply; the host
  already saw its own output, so aggregation-time scrubbing is the meaningful
  boundary) — cer-3 reframes accordingly. Settles at arch-4 spec time with the
  cost measurement; replay gate (cer-1) unaffected (replay needs only the
  controller-resident analyzer tape).
- ru-22 (human; the ingestion-as-declassification lens — hardens ru-21 a notch,
  still oven-time on the rest): host stdout/stderr quarantined per-host is the
  PRUDENT DEFAULT; the worry, in his words — any arbitrary subprocess of any
  process the book dispatches (package install-scripts, the whole heterogeneous
  meta-orchestrated mess) becoming an EXFILTRATION vector when not necessary; "we're
  already a massive security-hole by posture, in ways I haven't fully thought
  through." Conductor analysis adopted into the direction: (a) the
  failure-TRIGGERED auto-fetch from the prior synthesis is OUT — a host-influenced
  trigger (the host controls whether "failure" occurs) is an attacker-controllable
  declassification event; (b) the surviving shape: quarantine-by-default +
  NOTIFY-AND-OFFER (on failure/divergence Dorc prompts "evidence retained on host;
  run `dorc fetch-evidence <run> --site N.M`") + the FETCH itself is a human-held
  capability (the operator is the declassifier — Livshits-Chong few/explicit/owned
  permit-points, mirrored from release to INGESTION); same window as auto-fetch
  when the operator is present, and the unattended case — exactly where auto-fetch
  is attacker-shaped — stays closed; (c) the verdict lane is named as the ONE
  sanctioned ingestion point (fixed grammar, site-keyed, size-bounded,
  freeform-separated per 19B) — an orchestrator cannot ingest nothing, so the
  discipline is few/explicit/owned channels, not zero; (d) whatever ever gets
  ingested (lane today, slurped evidence later) is treated as hostile input at the
  controller: fixed-grammar parsing only, terminal-escape neutralization, size
  caps, scrub-at-ingest (102 E5 family). (e) flag-security-round-2: plans/102
  predates the r22 surfaces (durables, ingestion paths, OTel offramp) and the
  human's "haven't fully thought through" — a future-round security re-pass over
  the new surfaces is a recorded candidate, not scheduled. Dead-host window:
  accepted as a cost of the quarantine posture (the operator may fetch-on-notify
  while the tunnel lives, human-gated). All DIRECTION-grade pending the oven.
- ru-23 (human; two corrections to ru-22's shape + rec-6 closure):
  (a) CONSTRAINT caught from conductor contradiction: "fetch while the tunnel is
  warm" REQUIRES an in-run interactive y/n flow — Dorc must keep running and HOLD
  the SSH channel open through the offer; teardown ordering is the design point.
  The Ansible scar behind it (human, firsthand): a long book holding a channel
  open MASKED broken SSH access for half an hour — the warm tunnel hides
  access-breakage. Conductor candidate logged (-GUESS, unscheduled):
  fresh-connection canary at end-of-run — verify a NEW SSH connection succeeds
  BEFORE closing the warm one, with the warm tunnel as the recovery lifeline if
  it fails; converts the scar into a feature. (b) MODE-KEYED quarantine: the
  ru-22 posture applies to UNATTENDED/fan-out/converge-the-world mode only. In
  everyday INTERACTIVE mode (the default) the human wants realtime streaming
  output visibly — the bytes already cross to the controller, quarantine is
  already broken, so auto-RETAIN-on-error there has no marginal security cost
  and applies by default (scrub-at-ingest still applies to what's retained;
  ephemeral-terminal vs at-rest-file is the only delta). Unattended mode keeps
  quarantine + notify-and-offer + human-held fetch. (c) rec-6 CLOSED-RATIFIED:
  "no question, directly to richer metadata" — cause-tagged confirmed; B1 is
  mid-reshape on exactly this in arch-1. Scheduling note: the human suspects
  earlier-if-hitting-lots-of-sites and suggests sonnet-class agents for
  so-mechanical multi-site edits — ADOPTED as a dispatch heuristic (candidate
  for wave-2's 17-code catalog sweep: sonnet for the mechanical half, opus for
  the design half); also fb-candidate for round close (class-discipline gains a
  third tier: fable adversarial / opus build / sonnet mechanical).
- B1 arch-1 → HARVESTED as `54a4b84`+`38acbec`+`6b869a9` (arena+unord / Top(cause)+
  GATE / witness split): Opus, 479K harness-tokens (self-reported ~150K — largest
  delta yet; harness authoritative), 272 tool uses, ~70 min. Post-harvest chain:
  all gates rc=0 · 21 suites ok (481+ incl. the 3-test erasability gate) · e2e
  99/99 ×2 · typos 0 · fb-11 delta = exactly B2's fixture (expected). Build
  highlights: ProvId is !Ord BY DESIGN (can't key a decision BTreeMap — the weld's
  structural half); Reach Eq excludes the cause — found to be a TERMINATION
  requirement, not just contract (fixpoint converges on Eq); exempt-plane assigned
  via exhaustive destructuring with no `..` (new field = compile error until
  classified — include-by-default without proc-macros); canary asserts ran +
  nonzero arena + ≥1 Replace + ≥1 non-empty witness (non-vacuous exemption proof).
  PROOF-OF-BITE done: injected synthetic leak (cause-parity flipping effects) —
  gate caught it; reverted. strain-1 (load-bearing): first adversarial scheme
  (additive id offset) PRESERVED parity/residues — gate didn't bite; fixed with
  high-range odd-strided sentinels (concl-1's sentinel lesson re-learned in
  miniature). strain-2: first fixtures passed VACUOUSLY (no-establish oracle ⇒
  nothing elided) — caught, fixed, total_replaces>0 canary added (the 19I §3 trap
  in gate form). CONDUCTOR ADJUDICATIONS: tc-flag-1 ACCEPTED — cause lands on
  Reach::Top (21Z's literal causally-opaque complaint), ValueOf::Top cause
  DEFERRED-TO-ARCH-2 as a tracked item (vp-23-aligned: its consumer is the
  value-plane why-lens; rec-6's richer-metadata ratification carries scheduling
  latitude "wherever you prefer"; candidate sonnet-mechanical per ru-23).
  tc-flag-2 ACCEPTED — witness population threading is in-scope (the gate IS the
  consumer; witness-at-licenses is ratified vp-17 design, not speculative
  capture). Reach first-cause-wins join: accepted (decision-invariant; k-capped
  Join machinery exists in the arena for value-plane joins when needed; f-3's
  store-k-capped ruling was value-plane-scoped).
- x-1 DISPATCHED (the round's first Fable spend, per ru-14 one-sharp-pass; second
  only on divergence-need): hostile pass on the harvested gate, worktree
  x1-gate-attack, branch ai/r22-xcheck1 @ 6b869a9; may commit PoC pins; B1's
  hunt-1..8 handed with exceed-it instruction.
- WAVE-2 PREP (fresh-context conductor, clean-context resume per §11 process-1;
  2026-06-11): resumed from 22Z. Resume chain at `e6ea836`: all gates rc=0 ·
  test suites all green (1 ignored SPEC) · e2e 99/99 ×2 real exits · typos 0.
  Housekeeping landed: spike/CLAUDE.md gains the round-22 standing-rulings
  section (rec-1 two-surfaces / rec-5 tape≠kSTATE-cache / held-4
  battlefield-bound sanctioned exception) + corpus-count fix 43→99; 22B fork
  slugs renamed note-scoped `22B-fork-*` per rv3-find-3 (decoder note left in
  22B's header; ledger references above keep the old names). 22B read in full
  by the conductor. owed-1 explainers delivered in-chat this turn. Conductor
  catch at 22B read (~SUSPECT, handed to B3 to verify): the sketched
  `#[non_exhaustive]` on DiagCode inverts the stated intent — it forces
  DOWNSTREAM-crate matches to carry wildcard arms, defeating workspace-wide
  handle-every-code; B3 directed to omit it and push back if wrong.
- B3 arch-3-DESIGN → LAUNCHED (background; token/time figures appended at
  completion): worktree %TEMP%\dorc-r22\b3-arch3, branch ai/r22-arch3, base =
  `cb695a9` (the housekeeping tip; housekeeping commits `463c0b0` spike/CLAUDE.md
  + `cb695a9` 22B rename).
- B3 arch-3-DESIGN → HARVESTED as `894109c`+`9c4b621` (spine+3-migration / tidy
  gate): Opus, 390,910 harness-tokens (self-reported ~145K — fb-16 delta again;
  harness authoritative), 170 tool uses, ~50 min. Builder series `67d7f1d`/
  `b6c0b78` @ cb695a9; fb-11 content-diff EMPTY. Post-harvest conductor chain:
  all gates rc=0 · core 27 tests + diag_tidy 4 (new) · erasability 3/3 untouched
  · e2e 99/99 ×2 real exits · typos 0. ZERO goldens changed (coexistence: Diag
  lowers via to_legacy preserving (slug, span, severity); canon drops message ⇒
  digest-inert). Landed: exhaustive DiagCode (3 variants migrated:
  SiteUnresolvable / RenderHeredocRefused / CmdsubOperandTop; NO non_exhaustive —
  builder VERIFIED all consumers in-workspace, exhaustiveness is the feature),
  typed payloads, registry severity+Floor, mandatory primary SpanLabel, builder
  chain API, render_cli / render_artifact_comment / project_oob, diag::legacy
  submodule (3 dq-* survivors), LeafId promoted to core (dac-B), s-2 widening
  (real spans reach the ⊤-disclosures; `--> 20:45` proven in production CLI),
  report() renders spans (drop-A CLOSED), tidy gate with 20-code self-cleaning
  allow-list.
- CONDUCTOR ADJUDICATIONS (B3 tc-flags): tc-exempt-partition ACCEPTED-DEFERRED —
  structured Diag never reaches the canon at HEAD (only its to_legacy projection,
  already classified); proposed field classification BANKED for B4/arch-2:
  spans+code+payload-facts = identity / cause = Exempt::ReceiptId / prose
  children+suggestion = Exempt::Explanation. tc-wire-format ACCEPTED — slug() is
  the OOB wire token per the 22B-fork-wire-code disposition; consequence (rename
  = wire break) surfaced to the human. tc-gate3-evolution ACCEPTED — all-Note
  invariant replaced by per-code pin; render-refusal was already Error-declared
  in expected-diagnostics, no floor breach. tc-probe-site-ref ACCEPTED-HONEST —
  no first-class probe-record handle exists at HEAD; SiteId stands in; payload
  strengthens when probe records become first-class (ru-16 flag-up working as
  designed). tc-cmdsub-siteid ACCEPTED-WITH-DEBT — kernel-early code carries a
  CFG-node index in the LeafId-typed SiteId.leaf; safe today (render-plane-only)
  but two id-spaces in one type is NAMED DEBT: when diags grow site-keyed
  consumers (fleet rollup, OOB-lane keyed stores) the split must become typed;
  B4 told NOT to extend the standin where a real LeafId is in scope.
  tc-cmdsub-cause ACCEPTED-DEFERRED — cause hook present, None at the
  kernel-early site (mint_top_causes runs post-effects; inherent ordering);
  actual ProvId wiring lands in arch-2's emit-at-origin. PROPOSED floor column
  → to the human for PR-disposal: RenderHeredocRefused = Error+WarnOrDeny;
  SiteUnresolvable / CmdsubOperandTop = Note+None.
- B4 arch-3-MECHANICAL → LAUNCHED (background; first sonnet-tier dispatch this
  round, ru-23 third tier — class-discipline data point for round close):
  worktree %TEMP%\dorc-r22\b4-arch3m, branch ai/r22-arch3m, base = `fdb12af`
  (the B3-harvest ledger tip). Scope: migrate the 3 diag::legacy survivors then
  the 17 allow-list codes, behavior-preserving (verbatim message text, severity
  values unchanged into the registry, zero golden diffs expected, NEVER BLESS);
  per-code must-emit assertion found-or-added on existing harnesses; PROPOSED
  floor rows (Error give-ups → WarnOrDeny lean, disclosures → None); empty the
  allow-list + delete diag::legacy; the tc-cmdsub-siteid standin NOT to be
  extended where a real LeafId is in scope. Token/time at completion.
- B4 → FAILED-ABANDONED (human-observed): the sonnet did light investigation
  then RE-DELEGATED "the actual migration" to another sonnet with a
  near-identical prompt — recursively, FOUR deep; zero commits landed. The
  worktree's dirty tree (7 modified files, unverified multi-agent interleave)
  is ABANDONED in place in b4-arch3m for the human's inventory — not trusted,
  not reset, not harvested. fb-19 (candidate → adopted on the spot, human
  directive): sonnet-class agents must NEVER be given subagent-spawning
  latitude; every builder brief carries an explicit do-it-yourself /
  no-subagents clamp (spike/CLAUDE.md spawning section updated). Token cost:
  unrecovered (the stack was killed by the human).
- B4b arch-3-MECHANICAL re-dispatch: fresh worktree %TEMP%\dorc-r22\b4b-arch3m2,
  fresh branch ai/r22-arch3m2, base = the fb-19 ledger commit (this one); same
  scope as B4 verbatim + the EXECUTION-MODE clamp prepended (no subagents, no
  delegation, the failure mode named in-prompt). Sonnet again per ru-23 (the
  tier discipline stands; the clamp is the fix-attempt — if it recurs, the
  third tier demotes to opus-mechanical, fb-19 second data point).
- B4b → HARVESTED as `6f4862c`+`0e0a470` (20-code migration / tidy-gate empty):
  Sonnet, 148,680 harness-tokens, 248 tool uses, ~52 min — the fb-19 CLAMP
  HELD (no sub-spawning; mechanical tier ~2.6× cheaper than B3's design half,
  ru-23 economics confirmed). Builder series `27e6510`/`4085bd4` @ b217073;
  fb-11 content-diff EMPTY; "legacy deleted in prior context" report phrasing
  checked — all remaining diag::legacy strings are descriptive comments, the
  module is gone (builder likely self-compacted mid-run). Post-harvest
  conductor chain, unpiped: all gates rc=0 · core 25 tests + diag_tidy 4/4
  (allow-list EMPTY) · erasability 3/3 · e2e 99/99 ×2 real exits · typos 0 ·
  ZERO golden diffs (behavior-preservation held).
- CONDUCTOR ADJUDICATIONS (B4b flags): b4-cfg-top-severity ACCEPTED-FLAGGED —
  legacy emitted ONE code at two severities (Warning at the depth-limit site,
  Error at unsupported-construct); one-code-one-registry-row cannot represent
  that, builder unified at Error (the louder/kFAIL-safe direction; no fixture
  pinned the old Warning — e2e green proves the corpus never observes it).
  Human disposes at the PR; the split-into-two-codes alternative is the
  fallback if louder is unwanted. b4-spanless-codes ACCEPTED-AS-DEBT — six
  legacy span-None codes use slug-extraction (typed payload still constructed,
  evidence-demand enforced; legacy Diagnostic carries the None span so CLI
  output is identical); arch-3-residual-2: a `Diag::new_spanless` (or per-site
  span plumbing) cleans this up. b4-oracle-span-split consistent, no action.
  CONDUCTOR-FOUND residual: the promised per-code must-emit table was not
  delivered — tidy direction-A proves source-construction, not test-driven
  paths; arch-3-residual-1: cheap audit (map each of the 23 codes to a driving
  test, fill gaps) at arch-2 prep. FLOOR COLUMN state: B4b proposed
  Floor::None universally (conservative; brief's WarnOrDeny lean for Error
  give-ups NOT applied — builder chose flag-over-judgment, acceptable);
  full PROPOSED column = B3's three rows + B4b's twenty, human PR pass
  pending (gate2-ask-1's successor).
- Process self-log (fb-17 near-repeat, conductor, this turn): my first B4b
  post-harvest chain attempt piped `cargo test | grep` and `e2e | tail` to
  save tokens — the literal fb-17 scar (pipes mask rc; tail hides per-case
  output). Caught before any result was read; killed; re-run unpiped. The
  canonical chain stays literal, no pipes, even under token pressure.
- B5+B6 PARALLEL DISPATCH (human-directed end-of-window opus burn; both small,
  file-disjoint, arch-2-independent; token/time at completion): B5 e2e
  quiet-success knob (DORC_E2E_QUIET=1: ok-lines suppressed, failures verbatim,
  tally always, default byte-identical, POSIX sh; + one CLAUDE.md build-section
  sentence) → worktree b5-e2equiet, branch ai/r22-e2equiet @ c630726, with a
  perturb-restore failure-path proof required. B6 arch-3-residual-2 spanless
  mint path (visibly-second-class constructor; primary stays non-Option — no
  drop-B reopening; tidy test gains a hardcoded six-code spanless allowlist,
  self-cleaning; migrate the six slug-extraction sites; zero output change) →
  worktree b6-spanless, branch ai/r22-spanless @ c630726. Both opus, both
  carry the fb-19 clamp. Held back deliberately: must-emit audit (collides
  with B6 files; arch-2-prep item) and the x-1 test fold (needs
  post-arch-3-spine adaptation judgment; conductor-shaped).
- B5+B6 → HARVESTED as `606dc5c` (B5 quiet knob) + `bf3b4e3` (B6 spanless):
  B5 Opus 88,621 harness-tokens / 39 tools / ~43 min (self-rep ~70-85K — small
  fb-16 delta); B6 Opus 181,745 / 98 tools / ~28 min (self-rep ~95-105K).
  Builder commits `d14bfa2` @ c630726 / `3eb6283` @ c630726; per-branch fb-11
  diffs EMPTY. DEVIATION LOGGED: one combined post-harvest chain for both
  (independent smalls, disjoint files, human token-economy directive) instead
  of per-harvest chains. Chain green, e2e run QUIET (first conductor use of
  the knob — the two e2e sections totalled 8 lines): all gates rc=0 · core 26
  · diag_tidy 5/5 (new spanless_mint_allow_list_is_exact) · erasability 3/3 ·
  e2e 99/99 ×2 real exits · typos 0.
- CONDUCTOR ADJUDICATIONS: B6's representation ACCEPTED-COMMENDED — private
  `SpanSite{At,Spanless}` inside a now-private SpanLabel.span field +
  `span() -> Option<Span>` accessor; STRONGER than briefed (Spanless is
  unnameable outside core::diag — `new_spanless_site` is the literal only
  door; field-privatization verified zero-consumer-breakage). Allowlist gate
  is set-equal both directions over a production-only scan, both directions
  negative-controlled. b6-flag-1 ACCEPTED: the two check-* codes are
  span-OPTIONAL (real spans at 2 of 4 sites), correctly excluded from the six.
  b6-flag-3 atomic-commit ACCEPTED (constructor+migrations+gate genuinely
  co-dependent). B5 verified incl. the perturb-restore failure-path proof and
  default byte-identity. arch-3-residual-2 RESOLVED. PROCESS UPDATE: conductor
  chains now run e2e with DORC_E2E_QUIET=1 (failures print verbatim, so the
  fb-17 read-the-output discipline is intact; unquieted remains the default
  for builders/humans).
- x-3 CROSSCHECK PAIR dispatched (2026-06-12; human-ruled Fable-tier,
  adversarial-crosscheck skill protocol — neutral + disowned-adversarial in
  clean contexts; the round's second+third Fable spends): subject = the whole
  arch-3 diag family (B3 spine / B4b sweep / B6 spanless / diag_tidy gate) at
  base `6657a65`, all-legacy baseline `cb695a9`. Three test surfaces: (1)
  systematic legacy-vs-new observable-output equivalence for all ~23 codes
  (the comparison no builder produced; the corpus provably doesn't pin all
  codes and B4b rewrote its own verifying assertions); (2) empirical
  negative-control of every diag_tidy tripwire at HEAD (post-sweep grep-shape
  drift = the concl-3 green-while-asserting-nothing class); (3) hunt the
  remaining members of the cfg-top-node silent-change class. Exclusion set
  briefed gently as already-recorded context (known unification, spanless six,
  PROPOSED floors, abstract coverage-incompleteness), not as no-look zones.
  x3n-neutral (branch ai/r22-x3n) + x3a-attack (branch ai/r22-x3a, may commit
  PoC pins), both @ 6657a65, both fb-19-clamped, verdicts PRE-SANITIZED per
  process-1. Conductor presents BOTH to the human per the skill (convergence =
  signal; adversarial-only findings = suspect-until-checked); no single-verdict
  collapse. Token/time at completion. IF RESUMING COLD: check both branches +
  the two output notifications before any other action.
- x3a RETURNED (Fable, 227,070 harness-tokens, 119 tools, ~45 min; verdict
  sanitized-as-briefed; PoCs on ai/r22-x3a, each committed-then-reverted, tip
  tree byte-identical to base; NOT harvested). Verdict summary (UNADJUDICATED
  — held for x3n convergence): A/B executed both revisions over all 99 cases;
  stdout 0/99 differ, stderr 78/99 differ. Observable: x3a-1 region lines
  added universally (drop-A — deliberate, but goldens are stdout-only ⇒
  structurally blind; the conductor's "zero golden diffs = behavior held"
  framing was an overclaim); x3a-2 site-unresolvable message reshaped +
  enriched (the 22B worked-example enrichment — deliberate-but-uncertified);
  x3a-3 heredoc-refused gained a help line (same class); x3a-4 span
  None→Some on three Notes (s-2 — deliberate, unpinned). Latent: x3a-5
  check.rs lift_failure hardcodes Error via slug-extraction, BYPASSING
  registry() — "severity only from the registry" is false for check-*; a
  future registry edit is a silent no-op; no emit-vs-registry agreement test.
  Gate evasions (PoC hashes on the attack branch): x3a-B dead-catalog passes
  reachability (`e82b0c0` — the catalog's own match arms satisfy the grep);
  x3a-C unregistered legacy code via const-indirection (`8e6e224`); x3a-D
  spanless mint of a span-required code via binding-indirection (`8e6e224`;
  scanner also false-positives on comment prose); x3a-E retire-guard VACUOUS
  (`9db824d` — filter-then-assert-same-membership; cannot fire). ATTACKS THAT
  FAILED (machinery held): severity drift beyond known cfg-top-node — none of
  22; B4b message text character-identical across all 20 swept codes;
  span-presence preserved on all 14 corpus-unreached codes; stdout artifact
  byte-equal 0/99. Net shape: B4b's mechanical preservation is REAL; the
  certification language and the tidy gate's anti-drift power are what failed.
- x3n RETURNED (Fable, 307,926 harness-tokens, 125 tools, ~50 min; worktree
  left clean, scratch baseline removed). Verdict: none of the three areas
  fully clean. Area-1 fidelity: f-1 region lines (=x3a-1) · f-2
  site-unresolvable rewrite (=x3a-2) · f-3 cmdsub-operand-top span gain
  (=x3a-4) PLUS member-family emission 2→1 (x3n-ONLY; -GUESS reachability,
  parser may reject the triggering shape) · f-4/f-5 redir/inner-nonleaf span
  gains (=x3a-4) · f-6 heredoc help line (=x3a-3) · f-7 site-unresolvable
  no-matching-step defensive path now emits NOTHING + stale doc-comment
  (x3n-ONLY) · f-8 the known cfg-top-node depth-limit unification is pinned
  by NOTHING (no unit test, no corpus reach — enriches the known item).
  Verified-identical list covers the other 18 codes byte-level. Area-2 gate
  efficacy (all empirical, mutate→test→revert): t-1 "every variant
  constructed" VACUOUS for all 23 (diag.rs's own match arms satisfy the grep;
  deleted a sole production emit → green) = x3a-B, stronger proof · t-2
  retire-guard TAUTOLOGICAL (filter-then-assert same membership; full silent
  retirement → green) = x3a-E · t-3 working invariants enumerated (rename
  -drift, legacy A+B, spanless A; spanless B fails loud-by-staleness BUT its
  failure message instructs deleting the entry = institutionalizing the
  evasion) · t-4 SYSTEMIC: non-literal `DiagCode(expr)` emits bypass all five
  tests; check.rs lift_failure is the live instance (severity hardcoded,
  registry bypassed, span optionally None ⇒ structurally EIGHT codes can emit
  span-less, not six) = x3a-5 + x3a-C unified · t-5 gate-3 is one-directional
  error[-only (severity downgrades + vanished errors invisible; warning[
  patterns in 3 expected-diagnostics files are inert dead weight; x3n-ONLY).
  Area-3 coverage map (x3n-ONLY): e2e reaches 9 of 23 codes;
  PINNED-BY-NOTHING: effect-kind-disagreement, oracle-missing-kind,
  oracle-non-declaration, oracle-non-literal-kind, check-out-of-dialect,
  check-unterminated, syntax-malformed. UNPINNED∧CHANGED cross-cut: the three
  span-gain dq codes, site-unresolvable prose, heredoc help line, cfg-top-node
  depth site, the universal region lines.
- x-3 CONVERGENCE MAP (conductor): CONVERGENT-BOTH (highest trust): the four
  deliberate-but-uncertified observable change classes (f-1/2/3a/4/5/6 ≡
  x3a-1/2/3/4); t-1≡x3a-B and t-2≡x3a-E (both empirically proven twice,
  same root causes); t-4≡x3a-5+C (the indirection-bypass class, found from
  opposite directions); B4b's 20-code text+severity fidelity REAL; stdout
  0/99; no severity drift beyond the known one. NEUTRAL-ONLY (no hostile
  incentive — treat as solid leads): f-3b member-emission 2→1, f-7 silenced
  defensive path, t-5 gate-3 directionality + dead patterns, the coverage
  map, t-3's bad failure-message nuance. ADVERSARIAL-ONLY (suspect-until-
  checked per protocol, though all carry green PoC commits on ai/r22-x3a):
  x3a-D's specific binding-form NEW-mint evasion (x3n tested adjacent cells:
  literal-new caught, binding-existing caught-by-staleness; root cause
  converges). NO material contradictions between the passes. Pair cost:
  ~535K Fable harness-tokens combined.
- CONDUCTOR PROPOSED ACTIONS (pending human disposal; none executed): act-1
  fix check.rs lift_failure to route through the typed path (kills the
  registry-bypass severity fork, restores the six-code spanless boundary) ·
  act-2 rewrite the retire-guard with a real committed-source→list direction
  · act-3 make "every variant constructed" scan production emits only (the
  test's own comment already describes the unimplemented exclusion) — or
  re-doc it honestly · act-4 pin the UNPINNED∧CHANGED set with per-code unit
  assertions (absorbs into residual-1 must-emit work; option of stderr
  goldens REJECTED-lean: 78/99 churn) · act-5 process/22W: builders must
  separate "preserved" from "deliberately-changed-per-ruling" per surface;
  conductor must not equate stdout-golden-stability with behavior
  preservation (own overclaim recorded) · act-6 adjudicate f-3b/f-7 intent +
  fix the stale doc-comment (fold into act-1..3 PR). The four deliberate
  changes (drop-A lines, s-2 spans, 22B enrichments) are ratified work —
  action is CERTIFICATION (act-4 pins), not reversion.
- 22W-MATERIAL CORRECTION (human, 2026-06-12, on the conductor's in-chat
  spike review): the "context-compromise was a non-event / cold resume lost
  nothing" framing OVERSTATES. Demonstrated: the resume passed its gates and
  produced no DETECTED loss. Not demonstrated (unmeasurable): whether the
  round did a poorer job for the lost context — what the intact conductor
  would have caught, weighed, or steered differently. Lack of proof is not
  proof of absence; the effects at play are too subtle to settle empirically
  (as usual in engineering). 22W must carry the durability claim in this
  corrected, weaker form.
- B7 x3-FIX dispatched (human ack'd the action direction; conductor
  CONTINUING in-context per human lean — fresh-conductor note in 22Z was
  written under compromised-resumption conditions, no longer binding;
  handoff only at a clean post-harvest seam if compaction nears): worktree
  %TEMP%\dorc-r22\b7-x3fix, branch ai/r22-x3fix @ `2d09a9e`, Opus,
  fb-19-clamped. Scope: act-1 check.rs typed-path (registry bypass dies;
  spanless allowlist honestly 6→8 with documented WHY + emit-vs-registry
  agreement test) · act-2 retire-guard real source→list direction,
  negative-controlled · act-3 constructed-scan excludes diag.rs's own match
  arms, negative-controlled; needle-shape limit documented per ru-26 · act-6
  f-7 restore-or-document + doc-comment fix, f-3b reachability-check then
  restore-or-document. Behavior-preserving except explicit restorations;
  zero golden/stdout change expected. B8 (act-4 + residual-1 must-emit pins,
  the x3n unpinned∧changed list as worklist) dispatches at B7 harvest —
  serialized: file overlap on core tests. Token/time at completion. Scope: the
  22B §3/§4 spine in core + s-2 classify-signature widening EARLY + report()
  span rendering (drop-A) + gate-grep with seeded self-cleaning allow-list +
  the three §5 worked-example migrations as proving set. Opus tier (ru-23
  split); the mechanical ~14-site sweep is B4-Sonnet post-harvest. Conductor
  dispositions handed: use the EXISTING arch-1 arena ProvId (22B sketches a
  duplicate); 22B-fork dispositions (payload=typed per ru-16 · scope-key=stub
  coarse=fine · wire-code=string-slug, flagged at PR · floor-membership=
  builder-PROPOSES-only, human disposes · severity-help=no); goldens evolve
  hand-edited per-case with justification, NEVER BLESS (conductor-only);
  every new Diag field's exempt-partition classification = a tc-flag in the
  report; hostsim-Finding fold deferred beyond B3 (constraint noted: core
  cannot dep hostsim — Finding codes live in core's enum with core-expressible
  payloads, hostsim constructs); legacy Diagnostic coexists until B4 empties
  the allow-list.

- RESUMPTION (2026-06-12, fresh Fable conductor — third this round, after the
  security-vocab re-degradation recorded in the 22Z freeze): oriented from 22Z +
  this ledger + 22A + spike/CLAUDE.md. Harness TaskList did NOT survive the
  session boundary — reconstructed (12 tasks) from the 22Z queue; old task
  numbering is dead (old #13/#14 owed-explainers = new #1/#2). Frozen state
  verified exactly as recorded: ai/r22-x3fix tip `783894a` (fix-1 only), b7-x3fix
  worktree carrying the single uncommitted `M spike/crates/core/tests/diag_tidy.rs`
  edit — left IN PLACE for the human's inventory (freeze marked it discardable;
  fresh-worktree dispatch makes the discard unnecessary). Resume gate chain on
  the main tree at `44aa05d`, unpiped, e2e QUIET: ALL GREEN — build/fmt/clippy/
  deny rc=0 · cargo test **493** passed / 0 failed / 1 known-ignore (SPEC) across
  all suites incl. diag_tidy 5/5 + erasability 3/3 (an earlier draft of this entry
  said 488 — conductor arithmetic slip, corrected against the chain output at B7b
  review) · e2e **99/99 ×2** real exits · typos rc=0. One 22Z
  imprecision corrected there: code-HEAD equivalence class is `bf3b4e3` (B6), not
  `0e0a470` — B5/B6 code commits postdate 0e0a470; since bf3b4e3 only
  spike/CLAUDE.md gained the ru-26 doc lines (`2d09a9e`), rest notes-only.
- ledger-nit (recorded, no in-place fix — append-only): the "B7 x3-FIX dispatched"
  entry above carries a contaminated tail from the degraded window — everything
  from its second "Scope: the 22B §3/§4 spine…" sentence onward is B3's old scope
  text, pasted in error. B7's true scope ends at its first "Token/time at
  completion." The B7b entry below is authoritative for the fix-wave scope.
- B7b x3-FIX re-dispatch (2026-06-12, per the 22Z freeze plan "re-dispatch whole"):
  fresh worktree %TEMP%\dorc-r22\b7b-x3fix2, fresh branch ai/r22-x3fix2, base =
  `44aa05d` (current HEAD; rev-parse-verified at worktree creation, mise trusted),
  Opus, fb-19-clamped, ru-15-lean brief. Scope identical to B7: act-1 check.rs
  lift_failure onto the typed path, severity from registry, emit-vs-registry
  agreement test, spanless allowlist honestly 6→8 with documented WHY —
  implemented-but-FLAGGED, human disposes at harvest · act-2 retire-guard real
  committed-source→list direction, negative-controlled · act-3 constructed-scan
  excludes diag.rs's own match arms, negative-controlled, needle-shape limit
  documented per ru-26 · act-6 f-7 restore-or-document + stale doc-comment fix,
  f-3b reachability-check then restore-or-document. Behavior-preserving except
  explicit reported restorations; zero golden/stdout change expected; NEVER BLESS.
  CLEAN RE-DERIVATION: builder directed NOT to read ai/r22-x3fix / `783894a`; at
  harvest the conductor diffs the two independent fix-1 derivations as a free
  convergence cross-check, then 783894a stays unharvested (human inventory).
  B8 (act-4 + residual-1 must-emit pins) still serialized behind this harvest
  (file overlap on core tests). Token/time at completion.
- B7b RETURNED (2026-06-12): Opus, 286,776 harness-tokens (self-reported
  ~135-150K — the fb-16 delta again; harness authoritative), 134 tool uses,
  ~25 min. Builder series `817e050` (act-1/2/3) + `47d4e97` (act-6, doc-only)
  @ 44aa05d; worktree clean. GRANULARITY DEVIATION reported by builder,
  conductor-accepted-pending-human: act-1/2/3 in ONE commit — bidirectionally
  gate-coupled through the self-cleaning spanless allowlist (6 entries with 8
  minting fails one direction, 8 with 6 the other; b6-flag-3 atomic precedent).
- CONDUCTOR REVIEW (both diffs read in full): act-1 four-cell (terminated ×
  span) match, payloads spelled literal at each site (the grep-gates' eyes,
  needle-shape limit documented per ru-26); agreement test compares emitted
  severity to registry() SYMBOLICALLY (no hardcoded Error — survives a future
  human re-grade; better discipline than the stopped B7's hardcoded variant).
  act-2 extractor de-circularized (shape-scan bounded to the fn-slug body, the
  MIGRATED_SLUGS pre-filter deleted) PLUS a new anti-vacuity guard (empty
  extraction fails loud); two negative controls genuinely fire (catch_unwind
  asserting panic). act-3 scan basis switched to production_emit_source
  (excludes core); negative control is a PROPERTY-PIN (old basis contains a
  core-only marker, new basis doesn't — honest about why a true
  mutate-and-watch can't run in-test). act-6 both DOCUMENT, no observable
  change: f-7 ?-skip argued unreachable (every unresolvable site is a runnable
  leaf with a plan step; restoring would add a NINTH spanless mint for a dead
  branch); f-3b argued unreachable-for-⊤ STRONGER than x3n's parser -GUESS
  (member argvs concrete-by-construction — ⊤-carrying argvs never enter
  ValueFlow::member_argv). Conductor residuals (recorded, spike-acceptable,
  both needle-shape class): (residual-a, ~SUSPECT) the act-2 shape-scan reads
  only single-line `=> "…"` arms; an exotically-formatted arm is invisible —
  the non-empty guard catches catastrophic reshapes. (residual-b, +SURE,
  conductor-verified in the tip source): `production_emit_source()` excludes
  core but NOT non-core in-file `#[cfg(test)]` modules — so a test-only literal
  construction (e.g. the new agreement test's `registry(&Code::CheckUnterminated
  (…))` calls in oracle/src/check.rs) can satisfy the constructed-scan for a
  code whose production emit vanished. Strictly better than the old
  fully-vacuous basis (compile-coupling also softens the live instance), but
  the scan remains best-effort: B8's per-code must-emit pins (test-DRIVEN
  emission, not source grep) are the real liveness instrument — residual-b goes
  into B8's brief verbatim, with a one-line honesty note at the scan to land in
  B8's commit.
- x3fix CONVERGENCE CHECK (the clean-re-derivation dividend): B7b never saw
  `783894a`, yet the two act-1 derivations agree on EVERY substantive decision —
  identical four-cell match structure, identical literal-spelling choice (both
  with the needle-shape rationale written down), identical 6→8 growth, and both
  independently invented an emit-vs-registry agreement test. Divergences are
  cosmetic (interner param position/mutability; test naming) plus the
  symbolic-vs-hardcoded assertion nuance above. `783894a` confirmed a strict
  subset; stays UNHARVESTED (human inventory). Two independent derivations
  converging this hard is strong process evidence the fix shape is forced, not
  idiosyncratic.
- Builder test-count claim 497/0/1 VERIFIED-CONSISTENT: main at 44aa05d is 493
  (the corrected count above) + 3 negative controls + 1 agreement test = 497.
- fb-17 SELF-LOG (conductor, third near-repeat this round): my first B7b
  verify-chain script piped `cargo test | grep` and added a redirect-then-tail —
  the literal scar — caught at launch, task killed before any result was read,
  chain re-run literal/unpiped. The pattern recurs under token pressure;
  pre-commit self-check stays mandatory.
- B7b VERIFY CHAIN (conductor, on tip `47d4e97` in the builder worktree,
  literal/unpiped, e2e QUIET): ALL GREEN — build/fmt/clippy/deny rc=0 · cargo
  test **497/0/1-ignore** (diag_tidy 5→8: +3 negative controls; oracle lib
  20→21: +1 agreement test; arithmetic closes against main's 493 exactly) ·
  e2e **99/99 ×2** real exits · typos rc=0. Zero golden/stdout changes
  corroborated (e2e green both runs, no goldens touched in either commit).
- HARVEST GATED ON HUMAN (per the freeze's pending-decisions; presented
  in-chat this turn as 22-q1/22-q2/22-q3): spanless 6→8 amendment · f-7/f-3b
  document-vs-restore · granularity-deviation acceptance. Cherry-pick + fb-11
  content-diff + post-harvest main-tree chain follow the rulings; B8 dispatch
  follows the harvest.

- HUMAN RULINGS on B7b harvest (2026-06-13; the three freeze-reserved calls,
  recorded as 22-q1/q2/q3):
  - 22-hu-q1 (act-1 spanless): SYNTHESIZE an EOF span, do NOT mint span-less.
    Human reasoning: pointing the UI at end-of-file is genuinely right for a
    truncated/chopped file (the diagnostic lands at the real failure at least
    some of the time); a zero-width EOF caret is honest, not "pointing at
    nothing" (overrides the builder's + conductor's earlier
    spanless-is-more-honest lean). CONDUCTOR FINDING that sharpens it: BOTH
    check codes' only spanless source is `peek_span()==None`, which is EOF-only
    (pos≥toks.len()); there is NO non-EOF spanless case. So both get the
    synthesized span and the spanless allowlist reverts 6→8 → **6** — the
    amendment DISSOLVES ENTIRELY (a strictly better outcome than the freeze
    feared; act-1's registry fix lands with zero amendment to the stated
    spanless boundary). eof_span = zero-width at last-token `hi`.
  - 22-hu-q2 (act-6 silenced paths): an unreachable path should be an ASSERT,
    not a silent handled-skip. CONDUCTOR RECONCILIATION with inv-no-throw: both
    sites become `debug_assert!(false, <invariant>)` + retained safe fallback
    (loud in debug/test/DST, safe-degrading in release — rustc span_delayed_bug
    shape). NOT a release-panicking unreachable!(): f-3b is KERNEL (inv-no-throw
    forbids it outright); f-7 is CLI-edge but rides a reachability claim we
    decline to vouch hard (never-vouch). Both invariants conductor-verified
    construction-guaranteed + book-independent before authorizing the assert
    (f-7: unresolvable ⊆ plan.steps; f-3b: member argvs concrete-by-construction
    so no ⊤ on the None-site path).
  - 22-hu-q3 (commit granularity): act-1/2/3 coupled-commit ACCEPTED ("not my
    favourite, but let's not redo at this point").
- B7c REWORK dispatched (2026-06-13, Opus, fb-19-clamped): worktree
  %TEMP%\dorc-r22\b7c-x3fix3, branch ai/r22-x3fix3, base = `47d4e97` (B7b tip —
  keeps the accepted act-2/act-3 gate de-vacuuming underneath). Two surgical
  changes as separate commits: CHANGE-1 (22-hu-q1) add Parser::eof_span,
  `lift_failure` signature Option<Span>→Span, drop both new_spanless_site arms,
  allowlist back to 6, update agreement test, revert 6→8 narrative; CHANGE-2
  (22-hu-q2) both act-6 sites → debug_assert + safe fallback. Verified-facts
  handed as confirm-then-implement (caller set, EOF-only spanlessness, eof_span
  shape, both invariants' construction-guarantee). At completion: review +
  verify chain, then harvest the WHOLE series (817e050 + 47d4e97 + B7c commits)
  by cherry-pick; the honest history shows the human's ruling
  (spanless→eof-span) as a forward commit. Token/time at completion.

- B7c RETURNED (2026-06-13): Opus, 199,344 harness-tokens (self-reported
  ~95-105K — fb-16 delta), 85 tool uses, ~25 min. Builder commits `db6ca52`
  (CHANGE-1 eof-span) + `b562422` (CHANGE-2 f-7 assert) @ 47d4e97. CHANGE-1 and
  f-7 LANDED clean; f-3b HELD BACK with a verified-fact contradiction the
  builder correctly refused to code around (AGENTS exclusion-check working as
  designed).
- CONDUCTOR-FACT ERROR, owned (fb-candidate, process): my B7c brief asserted as
  VERIFIED that `emit_cmdsub_operand_top` is "called with site:None only from
  member_family's loop." FALSE. The unit test
  `command_effect_resolves_operand_singleton_and_top` (effect.rs ~1422) calls
  `command_effect(..., None)` DIRECTLY with a ⊤ operand (`apt-get install
  $PKG`), reaching the None-site arm WITH a ⊤ — which a `debug_assert!(false)`
  there trips. I traced the PRODUCTION call graph and never grepped for
  test/direct callers. Root finding: `site: Option<DiagSite>`'s `None` is
  OVERLOADED — (a) member-family path (⊤-impossible, production) AND (b)
  suppress-the-disclosure (the test's meaning, legitimately driving ⊤ to check
  classification return-values only). The f-3b path is therefore NOT a clean
  unreachable-path (it's a reached suppress-channel), so 22-hu-q2's
  "unreachable→assert" premise does not hold for it. fb-candidate: when
  "verifying" a reachability claim, grep ALL callers (tests + direct), not just
  the production graph — exactly the cell-coverage discipline AGENTS mandates,
  which I applied to the production cell only. The independent builder + the
  test suite caught it; never-vouch vindicated in both directions (don't trust
  the conductor's "VERIFIED" either).
- f-3b DISPOSITION → TO THE HUMAN (22-q4, not pre-decided): options —
  (opt-1) leave the None-branch a silent suppress, CORRECT its doc to state the
  overload (suppress-channel reached by tests, ⊤-unreachable in production), no
  assert [conductor lean: the premise dissolved, so docs-not-asserts is the
  honest 22-hu-q2 outcome for this site]; (opt-3) split `site` into typed
  states (`Suppress | At(site)`, or a member-family marker) so an assert can
  fire only on a genuine member-family ⊤ — honors the ruling but adds kernel
  machinery for a production-unreachable path (maintainability/simplicity cost
  vs low marginal validation value: a member ⊤ already collapses the family);
  (opt-4) assert the underlying invariant at its SOURCE — in member_family,
  that `value.member_argv(id)` yielded ⊤-free argvs — which the test does not
  reach, so it lands cleanly without touching the overloaded param [conductor
  second-lean if an assert is wanted]. f-7 (the clean unreachable path) got its
  assert; f-3b is the one that turned out not to be one.
- B7b+B7c HARVESTED as `<cherry-pick tip a798847>` (four-commit series
  817e050+47d4e97+db6ca52+b562422 cherry-picked onto ai/spike3; new hashes,
  builder series preserved on ai/r22-x3fix2/x3fix3). fb-11 content-diff
  ai/spike3↔ai/r22-x3fix3 on spike/: EMPTY. Net landed: registry-routed
  lift_failure (act-1) with EOF-span synthesis (22-hu-q1, allowlist stays 6) ·
  de-vacuumed diag_tidy gates + 3 negative controls (act-2/act-3) · f-7
  debug_assert (22-hu-q2 half) · act-6 docs. The harvested effect.rs f-3b
  doc is B7b's (production-accurate; due an overload-enrichment when 22-q4
  lands). `783894a` (dead B7) stays UNHARVESTED. Post-harvest conductor chain
  on the main tree at `a798847`, unpiped, e2e QUIET: ALL GREEN — build/fmt/
  clippy/deny rc=0 · cargo test **497/0/1-ignore** (diag_tidy 8: the 3 negative
  controls live; oracle 22: eof_give_up_carries_a_real_end_span +
  lift_failure_severity_agrees_with_registry; allowlist gate
  spanless_mint_allow_list_is_exact green at exactly 6) · e2e **99/99 ×2** real
  exits · typos rc=0. ZERO golden diffs (the two EOF-span codes are
  corpus-unreached, as predicted — the deliberate None→Some(eof) change reaches
  no golden).

- 22-q4 DEEP-DIVE (human pushed on the foundational assumption "members are
  ⊤-free"; conductor traced value.rs + effect.rs in full — the human's doubt was
  justified). FINDINGS:
  - fd-1 (+SURE, code-confirmed): `member_argv` is NOT ⊤-free. `record_member_sites`
    (value.rs ~782) resolves each member's argv via `resolve_site_words` and
    inserts it with NO ⊤-gate; `members_pass` does not post-filter; the only
    eligibility gates are on the FOR-LIST words + for-var reassignment, NOT on
    other body-command operands. So `for p in a b; do cmd "$p" $(date); done`
    produces member argvs `[cmd, <concrete>, ⊤]` — a ⊤-bearing entry.
  - fd-2 (+SURE): NO test exercises a loop body with a non-member ⊤ operand
    (grep `for…in…; do…$(` over value.rs/effect.rs tests = empty). That is why
    the false "⊤-free" claim was never falsified — the untested-invariant =
    vacuous-claim pattern, in miniature, in the very component the x-3 wave was
    auditing.
  - fd-3 (+SURE): the harvested f-3b doc ("None-site UNREACHABLE for a ⊤;
    belt-and-braces, NOT a live double-emit dedup") is BACKWARDS. effect.rs
    command_effect (195-217): a ⊤ operand emits CmdsubOperandTop + `return
    [Opaque]`. So a ⊤-bearing member argv → member_family's first ⊤ member →
    Opaque → `_ => return None` → family COLLAPSES → single-cell fallback
    discloses with the real span. The None-site emit IS reached in production
    (member-resolution scan) and the suppress is a LIVE dedup (prevents the
    member-scan emit from doubling the fallback emit). The value.rs field doc
    (84/138 "is ABSENT here") is also mechanically wrong: the entry is
    PRESENT-but-collapsed-by-the-consumer, not absent.
  - fd-4 (+SURE, the reassuring half): NO mis-elision. A ⊤ operand ALWAYS
    returns Opaque ⇒ MustRun ⇒ runs (kFAIL-perform holds); the ⊤ is disclosed
    exactly once (fallback). The bug is in the REASONING/DOCS, not behavior.
  - CONSEQUENCES: opt-4 (assert "members concrete" in member_family) is UNSOUND
    — it would fire on the valid `cmd "$p" $(date)` input; RULED OUT (caught
    before implementing — the value of the human's check + never-vouch). opt-1
    is the only sound disposition AND it is forced (no sound assert exists; the
    path is reachable). The corrected opt-1 doc must say: None = live suppress,
    reached when a loop-body command carries a non-member ⊤ operand; dedups
    against the single-cell fallback which discloses once with the real span;
    ⊤-operand⇒Opaque⇒runs so no mis-elision.
  - fb-candidate (process, stronger now): a false invariant survived B7b
    authoring + B7c + conductor review + would-have-been-"fixed"-wrong by opt-4;
    the human's "are you sure?" caught it. Untested invariants are where false
    claims hide — reinforces B8's must-emit/coverage remit and never-vouch.
- 22-q4 DISPOSITION: opt-1 (human pre-authorized "opt-1 is otherwise fine"),
  with the CORRECTED understanding above. Folded into B8 (its must-emit/coverage
  remit is the natural home for the pinning test): (i) rewrite the effect.rs
  f-3b doc + member_family call-site comment to the fd-3/fd-4 truth; (ii)
  correct the value.rs member_argv field doc (84/138) present-but-collapsed;
  (iii) ADD a pinning test — `for p in nginx curl; do apt-get install "$p"
  "$(date)"; done` ⇒ the apt site is MustRun (runs) AND exactly ONE
  dq-cmdsub-operand-top disclosure (proves the dedup; harness
  `classify_src_diags` + count by `dq-cmdsub-operand-top`). Until B8 lands, the
  harvested f-3b doc on ai/spike3 is KNOWN-FALSE (comment-only; flagged here).

- B8 DISPATCHED (2026-06-13, Opus, fb-19-clamped, human "Proceed"): worktree
  %TEMP%\dorc-r22\b8-mustemit, branch ai/r22-b8-mustemit, base = ai/spike3 HEAD
  `37703cb` (rev-parse-verified, mise trusted). Four parts, granular commits:
  PART A f-3b doc correction (effect.rs emit_cmdsub_operand_top + member_family
  comment + value.rs member_argv field doc → the 22-q4 truth; NO assert; the
  fd-1..fd-4 facts handed as confirm-then-correct) · PART B pinning test (loop
  body non-member ⊤ operand ⇒ MustRun + EXACTLY ONE dq-cmdsub-operand-top;
  count-not-presence, told to STOP if ≠1) · PART C must-emit audit — map all 23
  codes to driving tests, per-code pins for the unpinned set (x3n: the 7
  pinned-by-nothing + the UNPINNED∧CHANGED cross-cut), honest call-out for any
  un-pinnable code, stderr-goldens rejected · PART D the two needle-shape
  honesty notes at the diag_tidy scans (residual-a single-line-arm,
  residual-b non-core-cfg(test) basis). Token/time at completion. At harvest:
  review (esp. PART B count + PART C honesty), verify chain, cherry-pick, fb-11.

- B8 RETURNED + REVIEWED (2026-06-13): Opus, 224,828 harness-tokens
  (self-reported ~118K — fb-16 delta), 129 tool uses, ~32 min. Builder series
  `640b3c6` (PART A f-3b doc + PART B pin) + `e40d50d` (PART C 7 must-emit pins)
  + `d520f59` (PART D needle-shape notes) @ 37703cb. CONDUCTOR REVIEW (diffs read):
  PART A doc rewrite ACCURATE — states the live-dedup/reached/no-mis-elision/
  no-sound-assert truth precisely, no new false claim. PART B count==1 OBSERVED
  by builder (dedup model HELD; builder did not need to STOP); test asserts
  MustRun ∧ ¬EstablishMembers ∧ exactly-one dq-cmdsub-operand-top, with the
  count-not-presence rationale intact. PART C: the two check-* pins drive
  `lift_checks` over REAL sh (unterminated body; a `for` in the body) and assert
  the code identity — genuinely closing the x3a-B/t-1 direct-construction
  vacuity, not re-committing it; all 7 PINNED-BY-NOTHING codes matched x3n's
  list exactly, none faked, none needed disproportionate scaffolding; all 23
  codes now have ≥1 driving pin. PART D both ru-26 notes landed at the two
  scans. tc-flags: none new.
- B8 HARVESTED as `<cherry-pick tip d003e04>` (three-commit series cherry-picked
  onto ai/spike3; new hashes, builder series preserved on ai/r22-b8-mustemit).
  fb-11 content-diff ai/spike3↔ai/r22-b8-mustemit on spike/: EMPTY. The 22-q4
  f-3b docs are now CORRECTED on ai/spike3 (the known-false-pending-B8 flag
  cleared). x-3 FIX WAVE COMPLETE: act-1..6 all landed/dispositioned (act-5 was
  process-only, already in this ledger). Post-harvest conductor chain on the
  main tree at `d003e04`, unpiped, e2e QUIET: ALL GREEN — build/fmt/clippy/deny
  rc=0 · cargo test **505/0/1-ignore** (497 + B8's 8 new pins: the f-3b dedup
  pin, the 7 must-emit pins; analysis 133, oracle 26+check, syntax-parse 29) ·
  e2e **99/99 ×2** real exits · typos rc=0. ZERO golden diffs (PART A
  comment-only; B/C/D new tests + comments).

- PROCESS CHANGE (human, 2026-06-13): Fable-class DISABLED by Anthropic. Opus is
  now the tier ceiling for the conductor, validators, and all agents. ru-24's
  Fable ask-first gate is MOOT. Standing direction: (1) move slower, reason more;
  (2) dial UP adversarial-crosscheck use (less potent same-model, MORE necessary
  — no higher tier catches cross-cutting error); (3) surface foundational/
  cross-cutting design concerns to the human more often, don't resolve silently.
  Memory [[fable-ask-first]] updated to record the disablement.
- XC-1 TWO-PAIR ADVERSARIAL CROSSCHECK dispatched (2026-06-13, human-directed,
  adversarial-crosscheck skill protocol; first post-Fable, all Opus-vs-Opus):
  subject = the whole x-3 fix wave (44aa05d..d003e04 on spike/). Pair A
  TEST-QUALITY (are the must-emit pins / negative controls / f-3b dedup pin /
  EOF-span test genuine or vacuous-in-a-new-way): neutral `aa2e419e` + adversarial
  `a7cfaa6a`, each in an ISOLATED detached worktree (xc-a-neutral / xc-a-adversarial
  @ d003e04) so they can empirically mutate-then-revert to test whether a pin
  fires. Pair B DESIGN-COHERENCE (do the changes uphold/erode the inv-* + design
  tensions, run through the AGENTS exclusion-check four-by-two): neutral `ab79c262`
  + adversarial `a8c17b2f`, read-only on the main tree vs README/DESIGN/AGENTS/
  spike-CLAUDE/analysis-CLAUDE/core-CLAUDE. Clean contexts — none given the
  conductor's framing or this ledger's positive spin. RECONCILE on return:
  convergence (both passes) = highest trust; adversarial-only = suspect-until-
  conductor-verifies-in-source; present BOTH to the human uncollapsed (never
  vouch). Token/time at completion; xc-a-* worktrees cleaned after.

- XC-1 RECONCILED (2026-06-13; all four passes in, conductor-verified). HEADLINE
  LESSON (the round's most valuable same-model-crosscheck data point): the two
  most prominent Pair A findings were FALSE POSITIVES from a SHARED BLIND SPOT.
  Both test-quality agents (neutral + adversarial, independently) claimed
  `syntax-unsupported` (and, neutral-only, `render-heredoc-refused`) have "no
  behavioral pin / pass every gate" — because BOTH ran only `cargo test`, never
  `sh e2e/run.sh`. Conductor verification (one mutation push_unsupported→wrong
  code through the FULL chain + a grep of the corpus) overturned it: e2e gate-3
  (run.sh §684-707, the stderr-severity floor) matches every Error diagnostic
  against per-case `expected-diagnostics` patterns that EMBED the code
  (`error[syntax-unsupported]:`), so a wrong code = undeclared = case FAILS.
  Both codes are declared ×4 each (syntax-unsupported: background-amp-runs/
  top-eval/toprejected/while-read-file-rejects; render-heredoc-refused: 3×
  omitsafe21 + render21). B8's pin-map "×4" was RIGHT; the conductor's relayed
  "all 23 pinned" was RIGHT; the AGENTS under-counted by skipping e2e.
  CONVERGENCE OF TWO SAME-MODEL AGENTS ≠ INDEPENDENT CONFIRMATION when they
  share a method gap — exactly the human's "less valuable, same model" caution,
  and why verify-survivors is non-negotiable post-Fable. (Recorded as a 22W
  feedback item.)
- XC-1 REAL SURVIVORS (verified): (a) effect.rs:133 stale inline comment
  "member-family path: unreachable for ⊤ (concrete members)" CONTRADICTS the
  corrected f-3b fn-doc 20 lines above + the dedup test — B8 fixed the fn-doc +
  the member_family comment but missed this third co-located one
  (Pair-B-adversarial, conductor-verified in source). (b) value.rs:~753
  `record_member_sites` doc "each is a normal concrete argv" — same refuted
  framing, pre-existing (outside B8's briefed scope), conductor-verified. (c)
  Pair-A: `lift_failure_severity_agrees_with_registry`'s SEVERITY half is
  vacuous-at-HEAD — both sides read the registry, and both check codes are
  registry-Error, so a reversion to hardcoded `Diagnostic::error` would NOT trip
  it (its docstring oversells; the code/span/message halves are genuine). Real,
  low-severity, NO clean fix (no non-Error lift_failure code exists to
  distinguish). PUNCTURES the conductor's earlier "B7c symbolic test strictly
  better" praise — still better for the re-grade case, but it does NOT pin the
  registry-sourcing at HEAD. (d) Pair-A: `constructed_scan_negative_control`
  is a PROPERTY-pin (re-derives production_emit_source) — it does not guard the
  gate's basis WIRING, so a future revert of the gate to scanned_source() stays
  green (control + gate both). Real, low-severity. (e) minor: the f-3b dedup
  test comment's which-operand detail (conductor to verify-and-fix with the
  others). CONVERGENCE THAT HELD (trustworthy): Pair-B both passes — the 5
  design changes uphold the inv-* set (registry-routing, EOF-span determinism,
  debug_assert discriminant correct + inv-no-throw-respecting, f-3b substance
  TRUE, must-emit de-vacuuming) with no biting exclusion-check cell; Pair-A both
  — the 7 real must-emit pins + dedup-count + EOF-span all genuinely fire under
  mutation. CLEANUP (pending/surfaced to human): fix comments (a/b/e); judgment
  calls SURFACED — add unit code-pins for the 2 e2e-only codes? harden the
  negative control to guard the gate wiring? accept the severity-test limit? —
  all ru-26-flavored spike-scope calls. xc-a-* worktrees to clean.

- XC-1 CLEANUP HARVESTED as `d4b3826` (conductor-direct on main, human "clean up
  as you see fit" + per-finding rulings; chain green 506/0/1, e2e 99/99 ×2, typos
  0; zero golden diffs — comments + tests only). Four edits: (a) effect.rs:133
  inline comment corrected ("unreachable for ⊤" → "a ⊤ member IS reached + SUPPRESSED
  (dedup), disclosed once at the fallback"); (b) value.rs record_member_sites doc
  corrected ("each is a normal concrete argv" → for-var concrete but other operands
  MAY be ⊤, not ⊤-free); (c) NEW unit pin background_amp_emits_syntax_unsupported
  (parse.rs — `foo &` ⇒ push_unsupported ⇒ asserts code.0); (d) render-heredoc
  driver strengthened (observable_matrix.rs — added code.0 assertion to the
  existing count test). HUMAN RULINGS on the surfaced calls: add both unit pins
  (DONE c/d); negative-control hardening = LEAVE AS-IS; severity-test docstring =
  LEAVE AS-IS (explained, not changed). Finding (e) DROPPED — verified non-issue
  (the dedup test comment says "the ⊤" generically, never claims the fallback
  discloses $(date); adversarial-pass overstatement that did not survive source
  check — a verify-before-act win against overcorrection). KNOWN-ACCEPTED residuals
  (low-severity, disclosed, spike-scope): severity-test severity-half vacuous at
  HEAD (no non-Error lift_failure code to distinguish; registry-sourcing verified
  by code-read not test); constructed-scan negative-control is a property-pin (a
  basis-revert stays green). XC-1 COMPLETE; xc-a-* worktrees removed.
- ENV FLAG (2026-06-13, conductor noticed at cleanup commit): NEW SyncThing
  conflict husks materialized despite the folder being disabled 2026-06-11 —
  including `Research/notes/224-…sync-conflict-…030715-PHNHRER.md` (a husk of the
  ACTIVELY-EDITED ledger) + a second `plans/223-…233757` husk. Live files +
  git state are intact (husks are separate inert snapshots); NOT read/touched
  (human-owned per [[syncthing-device-identity]]). Surfaced to the human — may
  indicate sync re-enabled or misbehaving on this folder.

- ARCH-2 PREP (2026-06-13, human-directed: full design-doc read first, then prep,
  then an adversarial review-pair). FULL-CORPUS READ done in main context (human
  authorized the ~40k-token spend, 1M window): README/DESIGN/IMPLEMENTATION/KNOBS/
  ANALYZER-NEEDS + 22B + the TODO trio, all in full; plus the live seams
  (prov.rs arena, erasability.rs gate/canary, diag.rs cause hooks). KEY framing
  that grounds arch-2: the why-lens is the concrete realization of
  dir-soundiness-ux (STALENESS-AUDIT: per-line at-decision-point disclosure of
  the unsoundness), and the consumer that de-vacuums the erasability gate WITHOUT
  breaching ru-11 because it is a RENDERER (exempt-plane), never a decider.
  DELIVERABLE: notes/22C (`9b4943f`) — the arch-2 build contract: §2 build order
  (mvs-A emit-at-origin cause-wiring → mvs-B why-lens consumer → mvs-C suppression
  rules → mvs-D gate-obligation canary upgrade → mvs-E secondary-span → mvs-F
  ValueOf::Top deferred); §3 the welds (ru-11 one-way / rec-1 two-surfaces /
  dir-soundiness-ux / emit-at-origin / kFAIL); §5 the gate-obligation in detail
  (the load-bearing test: upgrade canary to "witness DIFFERS A/B yet identity
  plane identical" + proof-of-bite); §6 forks (origins / why-surface / value-⊤ /
  remediation-class). x-1 coverage-doc test (b68fc66): disposition = SUPERSEDE
  (it documented the vacuity arch-2 makes false), not fold.
- XC-2 ADVERSARIAL REVIEW-PAIR dispatched (2026-06-13, adversarial-crosscheck
  skill protocol, Opus-vs-Opus) on the 22C arch-2 PLAN (not code — a design
  critique): neutral `a5db7680` (assess soundness/scope, verify seam-claims vs
  code) + disowned-adversarial `a23ec87a` (5 named attacks: gate-obligation is a
  NEW vacuity · why-lens breaches the weld via remediation/tie-break · emit-at-
  origin contradicts the mint_top_causes-runs-after-effects ordering · one-origin
  scope leaves arch-1's passes-by-disuse hole · exclusion-check cells). Both
  read-only, clean contexts, told to verify against actual code + flag where a
  criticism does NOT hold. RECONCILE on return: convergence = trust; adversarial-
  only = conductor-verifies-in-source; present both to the human; THEN dispatch
  the arch-2 build only after the plan survives + the human rules the §6 forks.

- XC-2 RECONCILED (2026-06-13; both passes in, conductor-verified in source).
  VERDICT: the 22C arch-2 plan's DIRECTION is sound (why-lens as de-vacuuming
  consumer; the one-way weld HOLDS) but it has SEVERAL real, convergent errors
  that block dispatch — the review did exactly its job, catching them before a
  builder touched code. CONVERGENT (both passes, code-cited, conductor-confirmed):
  - fd-A (THE dispatch-blocker): mvs-A "mint the cause at ⊤-creation in the
    effect pass" is INFEASIBLE — `mint_top_causes` runs AFTER the effects pass
    (Opaqueness is the effects pass's OUTPUT; the ordering is inherent;
    effect.rs:120-125 + 819-842 + 224 §10 fd-3). My contract inverted the
    documented `tc-cmdsub-cause` resolution. FIX = option-(b): move the
    cause-bearing diag emission to a POST-`mint_top_causes` pass reading
    top_causes[node]; emit-at-origin is satisfied in SPIRIT (cause minted at the
    ⊤-origin node) but the diagnostic is assembled post-mint. NOT option-(a)
    (thread `&mut arena` into the kernel-early site — risks solve's pure-Fn
    determinism posture; both passes flag this). Wrinkle: the per-node cause is
    keyed on the CFG node's span (whole command), not the operand sub-span — so
    the why-lens cause is "this command went ⊤," not operand-level (fine for
    why-lens; worked-3's operand pairing is aspirational).
  - fd-B (the weld HOLDS — both passes, deflating attack-2): ProvId !Ord
    (prov.rs:54), Reach::Top cause excluded from Eq (effect.rs:471 — a
    termination requirement), RemediationClass/Suggestion render-only,
    render_artifact_comment/project_oob don't read exempt fields. No breach at
    HEAD/as-scoped. Forward-hazard to fence: a future mvs-C tie-break keying a
    disposition on which-cause-won WOULD breach (contract line needed).
  - fd-C (the gate-obligation de-vacuums WEAKLY/non-durably): the why-lens output
    is canon-EXEMPT by severity (CmdsubOperandTop=Note; canon_diag drops
    non-Error, conductor-verified erasability.rs:314 + the doc says "the gate
    would otherwise forbid the why-lens"). So "decisions identical under
    receipt-strip" stays trivially true. NEUTRAL: the de-vacuuming is
    consumer-REACHABILITY, not compared-plane-variance. ADVERSARIAL (deflating
    its own attack-1 strong form): mvs-D's canary upgrade IS a real strengthening
    BUT proof-of-bite as a one-off inject-and-REVERT pins nothing durable — the
    shipped gate stays vacuous against a FUTURE decision-reader = the XC-1
    finding-d property-pin-doesn't-guard-WIRING hole, again. CORRECTED
    gate-obligation = THREE durable assertions: (1) variance-bit: run-B receipts
    ≠ run-A (the perturbation reached the data — concl-3 coverage-canary); (2)
    consumer-saw-it: why-lens output DIFFERS A/B (a reader demonstrably saw the
    variance); (3) DURABLE proof-of-bite: a PERMANENT negative-control test where
    a synthetic decision-reader makes the gate FAIL — not a manual revert. My §0
    also mischaracterized the canary (it ALREADY asserts A/B byte-identity; the
    missing piece is asserting the variance bit + the reader).
  - fd-D (mvs-B underspecified + load-bearing): the why-lens (`why(site)->
    Explanation`) is the thinnest-specified step and the WHOLE de-vacuuming hinges
    on it; currently zero implementation (top_cause() zero callers). Needs real
    spec.
  - fd-E (fixture path): the de-vacuuming fixture MUST be a non-Members top-level
    ⊤-operand — the Members path SUPPRESSES the CmdsubOperandTop emit (f-3b
    dedup), so a `for p in…; do …"$(date)"; done` reads nothing.
  - fd-F (RedirTargetTop asymmetry): it has NO cause field at HEAD (payload
    {site} only) — wiring it is a PAYLOAD change, not a ride-along emit change;
    my contract wrongly presented the two as symmetric.
  - fd-G (reliability quadrant): the oracle-lifter ⊤ codes (OracleMissingProbe,
    CheckOutOfDialect) have no cause + site()==None → the why-lens reads NOTHING
    for the unreliable-oracle ⊤ class; the de-vacuuming covers only the
    reliable-oracle value-⊤ quadrant. PLUS the fallback-cause (site:None) render
    path is unaddressed (adversarial Area-5).
  ADVERSARIAL well-behaved (deflated its own attacks 1-strong + 2 honestly; no
  manufactured faults). DISPOSITION: REVISE 22C before any dispatch (fd-A..G);
  surface to human the gate-obligation reframe (fd-C three-part durable) + the
  mvs-A option-(b) re-architecture + the §6 forks. The plan SURVIVES in
  direction; it does not survive as a dispatch brief. XC-2 = high-value:
  same-model adversarial review caught a backwards pass-ordering and a
  non-durable gate that would have shipped the x-3 vacuity in a new coat.

- HUMAN RULING (2026-06-14): DROP the gate-obligation / durable-negative-control
  from arch-2 entirely. Reasoning (human-driven adversarial pushback, conductor
  conceded): the one-way weld is enforced by the TYPE SYSTEM (ProvId !Ord, cause
  out of lattice Eq, render-partition scope) — structural leaks are UNREPRESENTABLE
  (won't compile), so a runtime "prove it has teeth" test is redundant for that
  class AND a negative-control is impossible without weakening the types (= testing
  a different program); an intentionally-leaky/substituted engine is fragile and
  tests-little. The runtime erasability test's ONLY non-redundant job is catching
  type-VALID semantic content-reads (`if reach.top_cause().is_some() {…}`), a narrow
  class whose coverage is itself PARTIAL (the scrambler varies values, not presence).
  So the x-1 "vacuous-at-HEAD" alarm is largely a RED HERRING — the compiler does
  the real enforcement; the erasability test stays the cheap partial type-backstop
  it already is, NOT a load-bearing thing arch-2 must de-vacuum. CONSEQUENCE: the
  why-lens is arch-2's real deliverable, built for its OWN user-facing value
  (dir-soundiness-ux: per-line "why did this run"), not as a gate vehicle. 22C's
  §5 (gate-obligation) + mvs-D are VOID; 22C to be revised. fd-A (mvs-A ordering
  backwards → post-mint emission) + fd-E (non-Members fixture) + fd-F (RedirTargetTop
  no cause field) + fd-G (reliable-oracle quadrant only) still stand as real
  corrections for the why-lens build.

- WHY-LENS RETURNED + XC-3 RECONCILED + HARVESTED (2026-06-14). Builder
  (Opus, ~115K self-rep) landed 4 stages in 3 commits on ai/r22-whylens
  (`14c0166`/`2cf4ac5`/`e410537` @ 6c76a14): stage-1 cause-wiring via the
  deferred-typed-diag mechanism (a `&mut Vec<CmdsubTop>` collector through the
  effects pass; finalize the typed Diag with cause=top_causes[node] AFTER
  mint_top_causes — the corrected post-mint ordering, NO &mut arena in the pass);
  stage-2 `why(diag,&arena,src)->Option<Explanation>`; stage-3 cli disclosure to
  STDERR (`why: ran because …`); stage-4 dedup-by-cause-ProvId (Vec, !Ord). XC-3
  (human-directed crosscheck, can't-be-over-cautious; Opus pair, read-only): both
  passes + the conductor's own gate chain CONVERGE on SOUND-TO-HARVEST. Neutral:
  all 4 areas code-verified (pure/deterministic effects pass; ru-11 render-only;
  rec-1 artifact receipt-free; tests genuine). Adversarial: all 5 attacks
  (purity-broke / weld-breach / artifact-not-byte-identical / dedup-over-suppress /
  vacuous-tests) traced to source, NONE LANDS. Conductor chain on builder tip:
  GREEN (e2e 99/99 ×2 BYTE-IDENTICAL = artifact receipt-free + deterministic,
  independently confirmed; erasability 3/3 untouched). REAL FINDINGS (neither a
  blocker): (1) dormant fwd-hazard — CmdsubOperandTop derives Eq over `cause`
  (vs Reach which excludes it); harmless today, IS the 22D §2 named weld-breach
  watch-point (a future suppression tie-break comparing payloads would pull cause
  in) — no action, recorded. (2) COVERAGE GAP — the user-visible `why:` stderr
  render has NO e2e pin (render logic unit-pinned; artifact byte-identity
  e2e-pinned; the cli emission itself isn't — the e2e harness checks stdout +
  expected-diagnostics Error-floor, not the why: stderr line). Follow-up task,
  not a harvest blocker. HARVESTED as `<cherry-pick tip f40dded>` (3 commits
  onto ai/spike3; fb-11 EMPTY). REMEDIATION-CLASS disposition (tc-whylens-
  remediation): builder proposed CmdsubOperandTop⇒FixBookLine (Structural alt);
  conductor ACCEPTS FixBookLine — the render text is honest-CONDITIONAL ("to
  elide it, make the operand a literal Dorc can resolve+probe"), which doesn't
  lie (it says IF you can make it static); flagged for human override to
  Structural if the "don't imply it's the admin's fault when the dynamism may be
  essential" framing is preferred. Post-harvest chain results appended.
  METHOD NOTE: this is the first arc where the conductor's own design (22C) was
  caught wrong by XC-2 AND the corrected build (22D) sailed XC-3 clean — the
  crosscheck-on-design then crosscheck-on-build loop worked end to end post-Fable.

- fr-2 GRADED + STAMPED (2026-06-14): "Sound Non-Statistical Clustering of
  Static Analysis Alarms" (Lee/Lee/Yi, VMCAI'12) — at Desktop/vmcai12.pdf,
  CLEAN text-layer (pdftotext, 6460w, NO OCR needed — the Tesseract plan is
  moot). Grade **B** = `B-lee-lee-yi-sound-nonstatistical-alarm-clustering-
  vmcai-2012.{pdf,txt}` in Research/sources/ (GITIGNORED per Research/.gitignore
  — on-disk-only convention, NOT committed, durable through the context-clear;
  matches the 228/22A pre-grade `[B-vmcai-clustering-2012]`). THE FINDING (dc-7,
  grader digest): the paper grounds the POSTURE, not a drop-in mechanism — its
  soundness is manufactured by REFINEMENT-BY-REFUTATION (re-run the analyzer
  under "assume this alarm false"), which is exactly the backward re-derivation
  Dorc RULED OUT (ru-13). Dorc's actual dedup (collapse a ⊤-origin's pure
  poison-descendants by cause-ProvId) is the paper's TRIVIALLY-SOUND special
  case — "syntactic clustering" (§4.1, Example 4), needs no refutation. So sound
  clustering transfers to Dorc ONLY in the pure-propagation/syntactic-identity
  regime; the over-suppression risk is exactly where it might collapse
  CORRELATED-BUT-INDEPENDENT ⊤s. dc-7 cap lifts `~SUSPECT` → qualified `+SURE`.
  Relevant sections: §2.1 (alarm dependence), §2.2 (Def 3 + Thm 1), §4.1
  (syntactic clustering).
- x-2 OVER-SUPPRESSION CROSSCHECK dispatched (2026-06-14, adversarial-crosscheck
  skill, Opus pair) on the why-lens stage-4 dedup, GROUNDED in fr-2's
  sound-dependence-vs-correlation criterion: neutral `accb7687` + adversarial
  `aa2ef334`. Sharp attacks: two independent ⊤s sharing one cause-ProvId
  (hash-cons collision); a command with MULTIPLE independent ⊤ operands
  (`cmd "$(a)" "$(b)"` — command_effect returns Opaque on the FIRST ⊤, so the
  why-lens may disclose only one cause while a second independent ⊤ also forces
  the run — incomplete/misleading = the paper's over-suppression); non-descendant
  swept into a cluster; non-determinism. ON RETURN: reconcile, then write a
  notes/22x SYNTHESIZING the fr-2 finding + the x-2 verdict (human-directed —
  "stamp notes for the combination"), THEN a full durability sweep (ledger/22Z/
  tasks/harvest) because a CONTEXT-CLEAR follows. This is the last crosscheck
  before the clear.

- x-2 RECONCILED + SYNTHESIS STAMPED (2026-06-14; full writeup notes/22E). Both
  passes + conductor reconciliation: the why-lens cross-consumer dedup is SOUND
  in the straight-line/pure-propagation (paper's syntactic) regime + deterministic
  + the f-3b member-family suppress is sound. BUT two real OVER-SUPPRESSIONS
  (both DISCLOSURE-incompleteness only — ⊤⇒Opaque⇒runs / kFAIL-perform / stderr
  exempt-plane / NO mis-elision / NO weld-or-artifact-or-gate touched; low-sev,
  real vs fr-2): (x2-fd1, headline, ADVERSARIAL-found, neutral MISSED — the
  divergence) function INLINING gives two call-sites' spliced bodies the SAME body
  AstId (inv-leaf-seam) ⇒ same cause ProvId ⇒ the dedup collapses two GENUINELY
  INDEPENDENT dynamic operands (`apt_install "$(a)"; apt_install "$(b)"`),
  suppressing the 2nd why: line; reachable (1 literal-swap from passing e2e
  `inline21-wrapper-converged-elides`), unpinned; the exclusion-check the dedup
  skipped = the inlining cell (span-identity ⟺ cause-identity holds straight-line,
  BREAKS under inlining). (x2-fd2, both passes, upstream of the dedup) command_effect
  returns Opaque on the FIRST ⊤ operand → `cmd "$(a)" "$(b)"` discloses only
  operand 1; documented scope-cut (22D §1 "operand-level aspirational"). FIX
  (recorded 22E §3, NOT built): key the dedup on `(cause, site)` not cause-alone
  (cheap; cli/main.rs) + for fd2 disclose-all-⊤-operands or key cause on operand
  span; pin with the inlining + two-operand cases (folds into #16). Human
  disposition owed: fix-now vs accept-documented-cut (both disclosure-only, so
  deferrable; fd1 recommend-fix — cheap, contradicts "never hide an independent
  cause"). x-2 = the post-Fable crosscheck-grounded-in-research loop working:
  fr-2's "you'd need a refutation-proof (ru-13-banned)" is precisely why fd1
  can't be hand-waved. CONTEXT-CLEAR FOLLOWS — everything below is durable:
  why-lens harvested+green (f40dded), fr-2 graded+stamped, x-2 done+synthesized
  (22E), fixes recorded as #17. Round queue (post-clear): x2-fd1/fd2 fix (#17),
  remaining-ui-A/ui-B (#9, human), arch-4-thin (#10), round close (#12); carries
  #16 (why e2e-pin), the remediation-tag nod, the dormant Eq-cause hazard.

## §11 Post-gating self-audit (append-only; conductor, after a window where several turns produced no output)

> Written after several conductor turns produced nothing (model-gated on accumulated
> loaded vocabulary — the priming prompt's sec-gate warning, realized ONE LEVEL UP:
> banking a hostile crosscheck's full REPORT carried enough loaded vocabulary to gate
> the conductor itself). This section records, in neutral engineering terms, what the
> window established but never made durable, and audits the durable for gaps. The
> security-topic direction (ingestion/quarantine) is already ru-22/ru-23 and is NOT
> re-derived here per the human's omit-instruction.

### x1-outcome — the erasability gate is correctly built but VACUOUS-AT-HEAD (a test-coverage finding, not an arch-1 defect)
- An independent reader confirmed arch-1's structural defenses SOUND: ProvId is !Ord,
  the Reach ⊤-cause is excluded from Eq/Hash, the canon destructures exhaustively
  (no `..`), decision collections are BTreeMap/Vec (no shared-unordered iteration),
  and the mint/disposition paths take no arena/ProvId. No decision-divergence was
  producible. +SURE (two independent builders now agree the defenses hold).
- BUT the gate passes by DISUSE, not by exercised-inertness: at HEAD nothing in the
  decision pipeline READS a receipt. `top_cause()` has zero callers; the witness is
  populated yet the canon omits it; ValueOf::Top carries no cause at all. So run-A ≡
  run-B because the perturbed data is write-only/omitted — not because a live consumer
  was driven under variance. This is the 19I §3 trap ("passes because a fixture fed the
  right value") in GATE form. +SURE.
- Two machinery halves target not-yet-called code: the arena join path
  (OriginKind::Join + parent-reversal + k-cap truncation) is never invoked by the
  pipeline (only `arena.leaf` is), so the join-reversal half of the variance perturbs
  nothing; and the 6 gate fixtures exercise no Error-class diagnostic, errexit, door-3,
  heredoc, or multi-host fork — so the canon's diag path is dead and the
  most-likely-future-consumer constructs are uncovered.
- One latent consistency gap: the gate's canon is built from `classified.diags` only,
  while the cli digest uses `diags + refusals` — the gate underwrites a SMALLER
  identity plane than the digest. No fixture produces a refusal today ⇒ latent.
- Evidence: a documenting test at `b68fc66` on branch ai/r22-xcheck1 (records the
  vacuity; explicitly NOT a leak pin — none exists). NOT harvested (coverage-doc test
  on the attacker branch; fold-or-re-derive at arch-2 prep).
- DISPOSITION (conductor): arch-1 is NOT defective — gate and defenses are correctly
  built; the gate is simply not yet load-bearing because its subject (a receipt
  consumer) does not exist yet. arch2-gate-obligation (HARD, tracked): the instant
  arch-2 wires the first consumer (the why-lens reading `top_cause`, or any decision
  keyed on witness content), it MUST (a) land a fixture exercising that exact read with
  the variance targeting it, and (b) upgrade the canary from "witness non-empty" to
  "witness DIFFERS across A/B yet decisions are identical" — the actual non-vacuous
  inertness assertion. Until then, read the gate as "inertness STRUCTURALLY enforced;
  behaviourally UNEXERCISED," never "inertness proven end-to-end."
- never-vouch: this is an AI crosscheck's claim, and a DEFLATIONARY one (the gate
  proves LESS than its self-report implied) — so acting on it is conservative-safe; the
  human battle-tests the real thing.

### process-1 — model-gating reached the conductor (fb-candidate + resumption-relevant)
- Both dispatched Fable-class subagents this window, and then the conductor across
  several turns, gated on accumulated loaded vocabulary. fb-9's "cumulative, not any
  single bomb" is confirmed, one level up: a hostile crosscheck on an inertness/
  provenance component emits enough loaded vocabulary in its REPORT that banking the
  report poisons the conductor's own context.
- fb-candidate (round close): crosschecks on this component-family must return a
  PRE-SANITIZED verdict (disposition + neutral-engineering findings only; loaded detail
  stays siloed on the attacker branch, fetched only if a specific finding needs it).
  Bank the verdict, not the transcript. (This is the 21W sec-gate sub-agent guidance
  extended to the verdict-ingestion boundary.)
- Resumption: this context is compromised for further work in this topic-area; a
  successor conductor should resume from 22Z + this §11, clean-context. Round state is
  otherwise fully durable (arch-1 harvested + green; wave-2 next).

### owed-1 — explainers owed to the human, never delivered (gated mid-window)
- The human asked for slow explanations of: (a) why excluding the ⊤-cause from Reach's
  Eq is a TERMINATION requirement (the fixpoint's convergence test IS the `joined ==
  state[w]` Eq, so a cause-sensitive Eq never reaches a fixed point — confirmed by B1's
  build and the x-1 read); and (b) the two B1 scope judgments (cause on Reach::Top not
  ValueOf::Top, with the value-plane cause deferred-to-arch-2; and the witness threading
  through build_plan's call sites). All three explain ALREADY-RECORDED decisions — no
  new content — owed for the human's understanding. A clean-context conductor should
  deliver them.

### audit — already durable, no action
- ru-1..ru-23 all recorded (§7); rec-1..rec-7 batch recorded; RV1/RV2/RV3 sweeps banked
  (§10); B1 harvest + tc-flag adjudications recorded; B2 + D1 + the five research
  relaunches + the two warm-up fixtures recorded; 220 vp-8 and 21Z d-1 IB-annotated.
  The ONLY window-events missing from the durable were the three above (x1-outcome,
  process-1, owed-1) — now banked.
