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
- PHASE-R GATHERING COMPLETE at this row. Wave totals (harness): R0' 135K · R1' 179K
  · R2' 178K · R3' 164K · R4' 173K+~463K subs · W1 212K ≈ 1.50M, plus the stopped
  first wave (partial turns, banked scratch). All five notes committed; corpus 98/98
  ×2 green at `5b58c5f`+notes; next = 22Z, then digestion (#9) → synthesis (#3) →
  GATE-2 (#4).
