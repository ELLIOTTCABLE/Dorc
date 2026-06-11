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
