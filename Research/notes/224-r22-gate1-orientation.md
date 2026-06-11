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
