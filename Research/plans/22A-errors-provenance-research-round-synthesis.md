# plans/22A — round-22 PHASE-R synthesis: errors + provenance, researched

> The research round's conclusion (2026-06-11), in the 111/220 mold. Evidence basis:
> the five PHASE-R notes — 225 (unreachable primaries, all four now full-read),
> 226 (error-catalog practice + discipline tooling), 227 (derivation-dump/why +
> minimal OTel), 228 (suppression/dedup + fleet fingerprinting), 229
> (reproducible-builds / metadata-inertness) — ~130 graded sources total, grades
> assigned by gathering subagents with conductor re-verification status in §8; plus
> the GATE-1 rulings ru-1..ru-12 (notes/224 §7). AI-gathered and AI-synthesized;
> process evidence, never proof (the never-vouch hard limit). Confidence marks
> +SURE / ~SUSPECT / -GUESS / --WONDER. Greppable slugs `concl-N` here; per-front
> findings keep their home-note slugs (cite as `228:finding-grouping-key-design`).

## §0 Conclusions up front

- **concl-1 (the erasability gate is mainstream, and ours should be adversarial).**
  +SURE. Run-twice-with/without-metadata-and-compare has shipped in production
  compilers for ~17 years (GCC `-fcompare-debug`; LLVM debugify; Go's release-gating
  cross-OS double-build) and the mature systems all inject *variance* between the two
  runs rather than running identically — Debian varies ~20 axes with SENTINEL values
  so a leak is self-identifying in the diff. Ours: run-B strips receipts AND
  adversarially varies what remains (reversed origin-set order, sentinel ProvIds,
  varied DI'd seed). 229 §0/§1/§6.
- **concl-2 (the partition language: identity plane / exempt plane, per-field, named
  reasons, include-by-default).** +SURE, and human-ratified in shape (ru-12): the
  decision output splits into an identity plane (byte-exact: per-site dispositions,
  licenses, shipped .sh artifacts INCLUDING comments — the ru-12 floor) and an exempt
  plane defined by a CLOSED enum of named reasons (`Exempt::Explanation/ReceiptId/
  OriginOrdering/Timing`), assigned at each field's definition site, gate FAILING on
  any unassigned field. LLVM's named DebugLoc absence-reasons are the precedent, with
  the governing bias: when unsure, NOT exempt ("an absent location can be detected
  and fixed; an incorrectly annotated instruction is much harder"). Canonicalize-
  don't-exempt (timestamp-clamping) covers fields that legitimately vary but must
  still compare. 229 §0-partition-language; ru-11/ru-12.
- **concl-3 (gates rot by silent no-op; ship the canary).** +SURE. The documented
  failure mode is the gate going green-while-asserting-nothing (the 80%-quarantined
  e2e suite; the killed proof-gate that caught nothing a later job wouldn't). GCC
  pre-empted it with a coverage canary (`-fcompare-debug-not-overridden` errors iff
  the gate didn't run). Mandates: no auto-retry/quarantine on the gate; a canary
  asserting the gate ran and compared ≥1 thing; and the gate survives Meiklejohn's
  two-question test because receipt-into-decision leaks are invisible to
  decision-only tests — it catches a class nothing else can. 229 §0 finding-3, §9.
- **concl-4 (ordering is the most entangled leak; enforce it in types).** +SURE.
  Ordering nondeterminism is the central leak category for an analyzer, and the
  strong projects enforce determinism structurally, not by convention: rustc's
  `UnordMap` (iteration API removed — the leak won't compile) and
  `potential_query_instability` / `untracked_query_information` lints; LLVM's
  `-reverse-iterate` + ASLR-seeded hashing so order-dependence breaks immediately.
  Dorc: an iteration-suppressed newtype around decision-internal maps (~2-4d) turns
  the f-2 hazard class (Top(cause) or any receipt data perturbing BTreeMap/join
  order) into a compile error; `untracked_query_information` is the named direct
  analogue of the whole gate. 229 §4/§5/§7; rustc's no-in-tree-gate regression
  (reproducible at 1.44.1, broken by 1.45.0, issue open since 2020) is the argument
  for shipping ours in-tree from commit one.
- **concl-5 (cascade-dedup: emit at origin; suppression is five rules, not a
  subsystem).** +SURE on direction (~SUSPECT pending fr-2 on the formal grounding).
  Clang's own author moved AWAY from emit-then-rewalk-then-dedup to capturing the
  explanation where the fact becomes known (NoteTag at addTransition); rustc's
  poisoned values carry proof-of-emission so downstream stays silent by construction.
  Dorc's ⊤-cause-pointer IS this pattern: mint the cause at ⊤-creation; pure-
  propagation consumers inherit silently and never have standing to emit. The
  residual machinery is small: 228's mvs-rule-1..5 (carry-cause · interestingness-
  from-sink pruning · same-fact tie-break by speaker priority · observe-THAT-⊤-never-
  WHY · flush-or-trip net). Dedup happens in RENDERING; receipts are never destroyed
  at capture (merge/split is lossy — Sentry's scar). 228 §0/§1/§5.
- **concl-6 (site identity: structural, whitespace-normalized, hierarchical,
  config-tested).** +SURE. The deployed stability ladder (CodeChecker context-free-v2)
  lands on `(checker, file, enclosing-decl, whitespace-normalized line, range cols)`;
  Sentry's stability trick is emitting fine+coarse key hierarchies and matching the
  coarsest stable; WER names the two failure directions (expanding = volatile dims in
  the key; condensing = keying on symptom not cause). Dorc key candidate:
  `(analysis-rule-id, enclosing-structural-scope, whitespace-normalized command
  text)`, emitted fine (`+ call-site`) and coarse (`oracle-decl-identity`) for fleet
  rollup. Dorc's structural edge: we key on the analyzer-KNOWN ⊤-origin/claim-site —
  the callee-identity Sentry must guess — valid only while we never key on the
  consumer. Grouping/suppression rules, where configurable, are first-class tested
  config replayable in DST (Socorro: ~a change a week, testability drove their
  redesign). 228 §2/§5.
- **concl-7 (catalog: exhaustive-enum spine + tidy-grep reachability + retire-guard;
  gate completeness, never prose).** +SURE. rustc's tidy error_codes.rs is the gate to
  copy (bidirectional registry↔emit-site cross-check, git-diff retire-guard,
  self-cleaning hardcoded allow-lists); the cheapest Dorc spine is the catalog as one
  exhaustive Rust enum (the compiler enforces handle-every-code) plus a tidy-style
  grep test for the reachability/docs half. Dorc's give-up sites are nameable source
  points — a structural advantage over Menhir's derived automaton states, whose
  manual still proves completeness mechanically enforceable (kept green where wired
  into the BUILD GRAPH — Stan/dune; CompCert's 5283-line database). The counter-
  thesis is load-bearing: rustc's deny-level Fluent authoring mandate was downgraded
  to allow after earning hundreds of #[allow]s ("cheap structural gates endure;
  heavyweight authoring mandates die"), and Elm ships world-class diagnostics with
  ZERO catalog machinery — so the gate buys regression-safety and multi-author
  consistency, never message quality, and must not creep toward prose enforcement.
  226 §1/§2/§5/§7/§12.
- **concl-8 (severity: declared in the registry, with an un-overridable floor tier
  and expect-style positive assertions).** +SURE. Every mature scheme is consumer-
  overridable and drifts toward all-warnings unless an explicit un-overridable tier
  exists (rustc forbid/force-warn; future-incompat as a severity FLOOR — warn-or-
  deny, never off). Severity lives in the registry (TS `category`, Clang tablegen
  class), never at call sites (ESLint's per-site config = the maximally fragmented
  end). The sleeper: rustc's `expect` level — a positive "this site MUST emit X"
  assertion that fails CI when it stops — is the severity-system expression of the
  completeness wish, and composes with DST fault-injection (force each probe/oracle
  failure seam, assert the registered code fires). 226 §6/§10.
- **concl-9 (d-1 splits: durable dump + why-lens affirmed; trace-PINNING demoted).**
  +SURE on the split. The dump half has shipping precedent (Buck2's always-on
  `buck2 log`, ~15 lenses, built-in divergence-diff) and a cost story that is
  favorable for Dorc specifically (one run per orchestration, network-dominated —
  the 99GB/75-min Bazel scars come from per-action build workloads we don't have).
  The pinning half has a decade-scale regret analog: SQL plan-forcing (expert
  reversal; silent key-drift no-op "waiting for a query_id that never comes"; failed
  pins actively harmful and unmonitored; pinning corrupting adjacent identity) and
  Bazel deferring format-stability across all of 7.x. rustc UI tests prove pinned
  output works ONLY with a large normalization layer and redundant human-written
  verdict assertions so a re-bless can't rubber-stamp. Disposition (matches ru-7):
  build the dump + `why`; pin VERDICTS everywhere; trace-pinning only if a critical
  tier earns it later, never trace-only, with a staleness-janitor designed in. 227
  §0/§2/§4/§6.
- **concl-10 (the durable can be THIN: seed + probe-tape; recompute the rest).**
  +SURE on the mechanism (it is our own DST determinism, independently confirmed by
  rr/Pernosco's store-minimal-recompute-everything economics): given the same inputs
  and injected seed, the analyzer reproduces the identical derivation, so the
  per-run durable need only carry what cannot be recomputed (the probe-response
  tape, the input identities, the seed) plus the decision digest. The full trace is
  a RENDERING of a re-run, not a stored artifact. This dissolves most of the
  always-on cost worry and honors kSTATE (the durable is a write-only log;
  re-ingesting receipts across runs stays forbidden — ru-12/f-6). Dump format:
  JSON-lines, version-tagged, NO byte-stability promise (OTel's own file-exporter
  spec disclaims ordering; Bazel's experimental-across-7.x precedent). 227 §7/§8.
- **concl-11 (why-lens UX: the most-diagnostic slice is the default first view).**
  +SURE. The postmortem evidence says dumps pay off only when the first view is the
  correlated, minimal, causal slice (the 47-minute incident where the events existed
  from minute zero and were looked at at minute 27); the dump-diff practitioners
  want transitive-vs-non-transitive filtering and normalization of uninteresting
  field-differences. Maps onto 220's minimal-witness-first habits unchanged; the
  silent-green-dashboard failure class (emergent composition that "survives every
  test you have") is the best affirmative case for per-value receipts — and it
  argues for receipts/why, not pinning. 227 §2 query-ux-1, §9.
- **concl-12 (minimal OTel: neutral events, project at the edge, value-format only).**
  +SURE. The working shape is bazel_conduit's: the tool emits a neutral event
  stream; OTel is a downstream projection (sidecar), never an in-process SDK
  dependency; the machinery you skip reappears at the EXPORT layer where it belongs.
  `traceparent` is a fixed hex value-format trivially emitted without any library —
  adopt the value-format, choose our carrier (verdict lane / env), with IDs and
  times routed through DI seams (every documented hand-rolling pitfall — stale
  clocks, epoch-zero, negative durations, queue drops — is a clock/RNG/ordering leak
  our seams already fence). Two cautions: scrub secrets AT CAPTURE before any
  attribute is set (command-lines carry tokens); and the observability channel must
  not share fate with what it observes (durable-locally-first, then exfiltrate).
  OTel env-carriers spec (Beta) surfaced but UNREAD — read before the arch-5 call.
  227 §3/§8/§9.
- **concl-13 (formalism corrections from the primaries).** +SURE. (a) The one-way
  slogan is the Sabelfeld–Sands survey's gloss; Zdancewic–Myers itself supplies the
  semantic property (active attacker learns no more than passive observer), an
  attacker model whose fair-environment assumption sharpens our rule ("receipts must
  not mint authority they did not already hold"), and a blame construction
  (`glb(D)`) that needs a DISTRIBUTIVE lattice — a caveat to carry on ⊤-blame; its
  Thm 4.2 carries a published self-correction, so cite the property, never a tight
  bound, and cite the survey for the slogan. (b) Green–Tannen's Prop 3.5 makes our
  coarsening PRINCIPLED (a coarse view is safe exactly when the map is a semiring
  homomorphism — flat lineage on values is a sanctioned image, not accidental loss),
  the why-vs-how passage is the verbatim ground for witness-at-licenses, and Thm 9.2
  (distributive-lattice containment collapse) corroborates refusing how-polynomials.
  (c) Livshits–Chong full-read: humans misplace permit-points; placement correctness
  is ORTHOGONAL to analyzer precision (our best-effort anchor); claim no optimality.
  (d) Carata tail: overhead never killed a provenance system (SPADEv2 <10% on
  production Apache); noise and capture-without-consumer did; disclosed-vs-observed
  is the trust axis that independently re-derives claim-vs-receipt. 225 §0.

## §1 What this changes in the arcs (the GATE-2 re-scope proposal)

- **arch-1 (ProvId arena + Top(cause) + THE GATE) — SURVIVES, strengthened.**
  Adds, from research: the per-field `Exempt` closed-enum partition with
  include-by-default (concl-2, honoring ru-12's artifact byte-floor); adversarial
  variance in run-B + sentinel receipt values (concl-1); the coverage canary
  (concl-3, ~0.5d); the iteration-suppressed newtype for decision-internal maps
  (concl-4, ~2-4d — subsumes GATE-1 flag f-2 structurally); a per-run decision
  digest line (cheap always-on signal, ~0.5d); per-stage gate localization DEFERRED
  until a leak is hard to localize (LLVM's `-debugify-each` precedent, with its
  cost/sampling caveat). Contract lines unchanged from GATE-1: Top(a)≡Top(b) in the
  lattice; licenses exempt from k-caps; site-keyed receipts. Erasability assertion
  is permanent strict equality on the identity plane (ru-11 WELD).
- **arch-2 (one consumer end-to-end) — SURVIVES, reshaped by emit-at-origin.**
  The ⊤-cause is captured at creation and carried (concl-5); downstream consumers
  never emit; the dashboard why-not renders minimal-witness-first with the
  suppression rule-set as code (mvs-1..5), ranked by REMEDIATION-CLASS (ru-6's
  missing axis: group origins by which user action clears them — author-oracle /
  add-declaration / fix-book-line / structural). Receipt-derived rendering stays
  OUT of shipped artifacts (ru-12 inversion); the span-bridge tier-2/tier-3 work
  rides along as planned. Site identity per concl-6 (hierarchical keys) — this also
  pre-builds the fleet-aggregation seam without committing to fleet UI.
- **arch-3 (catalog retrofit) — SURVIVES, sharpened.** The 17-code retrofit into an
  exhaustive-enum catalog; the tidy-style bidirectional gate + git-diff retire-guard
  + self-cleaning allow-lists (concl-7); per-code declared severity in the registry
  with an un-overridable floor tier marked per-code (concl-8; resolves tc-fix3's
  shape — the human ratifies WHICH codes pin to the floor); `expect`-style positive
  must-emit assertions on give-up sites, composed with DST fault-injection scenarios
  (force each oracle/probe failure seam → assert the registered code fires); spans
  rendered in report() + the s-2 classify widening EARLY (unchanged from GATE-1);
  hostsim Finding folds IN-catalog per ru-5's lean unless the build finds friction
  (flag-up if so). Explicitly NOT built: any authoring-mandate lint on message
  phrasing, any prose-quality gate (concl-7's regret evidence).
- **arch-4 (durable + why) — RESHAPED per concl-9/10/11.** Build: the thin durable
  (probe-tape + inputs + seed + decision digest, JSONL, version-tagged, no
  byte-stability promise) and `why` as the first lens with minimal-witness-first +
  transitive-filtering. Verdicts pinned everywhere in fixtures (status quo). Do NOT
  build: golden-TRACE pinning this round (the regret evidence + ru-7; revisit only
  with a concrete user-story and a designed staleness-janitor). The OOB record
  grammar grows its provenance field on the site-keyed anchor as planned.
- **arch-5 (OTel seam) — SHRINKS to a value-format decision.** Adopt the
  traceparent value-format on our existing lanes via DI seams when arch-4's durable
  lands; no SDK, no collector, projection-at-the-edge reserved as a future sidecar
  (concl-12). Gate on reading the env-carriers spec (one fetch) — if it offers a
  cleaner env-var carrier convention, prefer matching it. Effectively merges into
  arch-4's tail; I propose retiring arch-5 as a separate arc.
- **Crosscheck budget:** hold the inherited heuristic — ~25-30% of build spend on
  hostile passes; the named first targets are (x-1) the gate itself (can a hostile
  builder construct a receipt-into-decision leak the gate misses — partition holes,
  canonicalization bugs), (x-2) the suppression rule-set (over-suppression: construct
  two independent causes where root-cause-only hides the second), (x-3) the catalog
  gate (can a give-up path evade registration — the comment-line exclusion class).

## §2 Open items at GATE-2 (the human's)

- **gate2-ask-1 severity floor:** ratify per-code severity declarations + which codes
  are floor-pinned un-overridable (concl-8; tc-fix3's successor).
- **gate2-ask-2 hostsim Finding:** confirm IN-catalog (ru-5 lean) as the build
  instruction.
- **gate2-ask-3 arch-5 retirement:** confirm folding the OTel seam into arch-4's
  tail as a value-format-only item.
- **gate2-ask-4 trace-pinning:** confirm "verdicts-everywhere, no trace-pinning this
  round" as the d-1 disposition (concl-9; aligns with your ru-7 lean).
- **gate2-ask-5 the third d×d cell:** authorize (or defer) the 215-labeled
  outer-live × inner-diverged-runs fixture (cheap, W1-shaped).
- Standing fetch-requests, non-blocking: fr-1 (CACM Debugging-in-the-Very-Large),
  fr-2 (VMCAI'12 text layer — would lift concl-5's formal-grounding cap), 226's
  fetch-1 (TS stability-policy verbatim). The doors program, kSTATE, find-J,
  capture-eagerness: all stay parked per the priming prompt; nothing in the research
  moved them.

## §8 Registry + re-verification status (honesty section)

The graded source registry lives distributed in the five notes' Graded-sources
sections (225: 4 sources · 226: 26 · 227: 20 · 228: 17 · 229: 55; all
`graded-by: subagent` except 225's two predecessor-banked reads, re-verified by its
own author). Conductor re-verification at synthesis time: I fully read all five
NOTES (the curated evidence: every load-bearing claim above traces to a verbatim
excerpt I read in its note context), but I re-read only a handful of SOURCES
directly; the following load-bearing claims rest on subagent grading I have NOT yet
independently re-verified against primaries and are flagged accordingly:
- the Clang BugReporter mechanics (228 §1 code excerpts) — high-trust (verbatim
  code), unverified provenance of line numbers;
- the Debian variance table (229 §6) — verbatim-derived, unverified against the live
  page;
- the SQL plan-pinning regret cluster (227 §4) — several sources read via Exa
  highlights only, honestly capped by the agent; the CONCLUSION (concl-9) would
  survive any one of them falling, not all;
- VMCAI'12 sound clustering — capped ~SUSPECT by everyone, pending fr-2.
Per the never-vouch limit: this synthesis is the conductor's best reading of
AI-gathered evidence; the design consequences are arguments, not verified facts,
and the build arcs carry their own gates.
