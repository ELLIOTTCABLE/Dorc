# 22W — round-22 close report: ERRORS + PROVENANCE, what was settled

> Round-22 close / on-ramp. Synthesizes what r22 ESTABLISHED that a future agent
> (building the real Dorc, or running r23+) needs — the durable design-decisions,
> welds, and architecture, organized THEMATICALLY, not chronologically. The noisy
> play-by-play lives in `notes/224` §10 (the round ledger); this report extracts the
> load-bearing signal so it doesn't die buried there. AI-authored synthesis; process
> evidence, never proof (never-vouch — no claim here that the built thing is *good*,
> only what it *is* and *decided*). Confidence-marked +SURE/~SUSPECT/-GUESS/--WONDER.
> Cite the note-slug or file behind each claim; do not re-derive.
>
> Theme of the round: error-reporting + provenance — the "how we tell the user what
> we gave up on, and why" plane. Five build arcs landed (arch-1 provenance/erasability,
> arch-3 diagnostic-catalog, the x-3 catalog-discipline fix wave, the why-lens, ui-A
> mode surface) plus the engine-truth discoveries the work surfaced. Two FORWARD arcs
> were surfaced and DEFERRED — the analyzer's TIME axis (`plans/22H` live-plan /
> concurrent-incremental) and the CERTAINTY axis (`plans/230` best-effort / collapsed
> gradients); they are written up there and are explicitly OUT OF SCOPE for this
> report (what r22 *did*, not what's next).

Read order for a future agent: this report top-to-bottom is ~5 minutes and tells you
"what r22 settled." Then `spike/CLAUDE.md` for the welded `inv-*` invariants + the
standing round-22 rulings verbatim; the source seams cited inline to ground anything
load-bearing (`core/src/diag.rs`, `core/src/prov.rs`, `plan/src/erasability.rs`,
`cli/src/main.rs`); and `IMPLEMENTATION.md` "Correctness vs. best-effort: a band" for
the human frame the whole round hangs off.

---

## 1. The structured diagnostic API — the error-reporting spine (22B + arch-3)

WHAT IT IS. An exhaustive `DiagCode` enum (23 variants at HEAD, `core/src/diag.rs:58`)
with a **typed per-variant payload struct**, a **registry** (`registry()`,
`diag.rs:722`) that decides severity + an un-overridable **floor tier** keyed by code
(never at the construction site), and a **catalog-completeness gate** (`diag_tidy`)
that a give-up site must be a registered code to ship. +SURE — built, green, source-verified.

WHY IT MATTERS. This is the error-reporting spine the real Dorc inherits. ru-17 made
it BATTLEFIELD-BOUND: the human ruled it "could become The Product of the spike, and
one of the first bits of actual code to write for the real codebase" — a sanctioned
exception to the spike's disposability charter (`spike/CLAUDE.md` held-4). The spike
PROVES the shape; the real codebase extracts/reimplements it (diag is a leaf module,
extraction is cheap). Design-for-keeps applies to this module and NOTHING ELSE by analogy.

The durable design decisions, each load-bearing:

- **Typed payload per variant = the capability instinct made structural** (ru-16,
  `22B` type-sketch-1). A diagnostic cannot be constructed without the objects it
  cites: `SiteUnresolvable` demands a site + probe handle, `CmdsubOperandTop` demands
  a site + `OperandPosition` + an `Option<ProvId>` cause. You cannot author the
  diagnostic wrong because you cannot NAME the wrong objects — the stock Rust compiler
  is the enforcement engine, no DSL. +SURE.
- **Severity lives in the registry, never the call site** (`crib-4`), with an
  un-overridable **`Floor{None, WarnOrDeny, Pinned}`** (`crib-5`, `diag.rs:868`). This
  kills the surveyed all-warnings-drift failure mode (`226` finding-severity-fragments:
  every scheme drifts to all-warnings unless an un-overridable tier exists). The PROPOSED
  floor column (human disposes; `22B-fork-floor-membership` / `22A` gate2-ask-1):
  `RenderHeredocRefused` ⇒ Error + WarnOrDeny (a kFAIL-correctness give-up — silencing
  it hides a converged mutator running because render couldn't safely elide it); the
  pure disclosures (`SiteUnresolvable`, `CmdsubOperandTop`) ⇒ Note + None. +SURE on the
  shape; the column itself is builder-PROPOSES / human-disposes and still open.
- **Mandatory primary span** (`SpanLabel`, `diag.rs:589`): span-poverty is a compile
  error, not a habit. Span-lessness is reachable ONLY through `Diag::new_spanless_site`
  (`diag.rs:922`) whose `Spanless` discriminant is unnameable outside `core::diag` — a
  deliberately second-class door. The one genuinely-spanless case (a truncated file's
  EOF give-up) was ruled (22-hu-q1) to **synthesize a zero-width EOF span** rather than
  mint spanless — "pointing the UI at end-of-file is honest for a chopped file." +SURE.
- **`Suggestion{applicability, remediation}`** — `Applicability` cribbed verbatim from
  rustc (`MachineApplicable`/`MaybeIncorrect`/`HasPlaceholders`/`Unspecified`,
  `diag.rs:666`); a tool decides auto-apply from the applicability, not the prose. The
  remediation axis is its own theme (§3). +SURE.
- **One value, three+ render lanes, authored once** (`22B` §4): `render_cli` (the full
  render/overlay surface — Elm narrative tone over rustc's data model), `project_oob`
  (the fact-plane-only OOB lane projection, slug as the stable wire token —
  `tc-wire-format`, a rename = wire break), `render_artifact_comment` (fact-plane,
  receipt-stripped — see §2), and a future dashboard as a fourth pure consumer. +SURE.

THE FRICTION TEST (the central guardrail, `226` finding-fluent-regret). The single
load-bearing constraint is the Fluent-migration regret: rustc's deny-level authoring
mandate (a Fluent DSL + a derive + a mandatory multi-file edit) was walked back to
`allow` in Oct-2024 after metastasizing into hundreds of `#[allow]`s. So the test:
adding a code is ~one edit, no DSL, no derive machinery, no multi-file mandate, no
proc-macro (also forbidden by `inv-no-unsafe`). Honest answer (`22B` §7): it is THREE
match-arms in ONE file (`core/src/diag.rs`) — the enum variant + the `registry()` row
+ the render arm — all compiler-GUIDED (a missing arm won't compile on the exhaustive
match). That is categorically lighter than the Fluent mandate (four distinct
files/systems with cross-file consistency the compiler didn't check) and survives.
~SUSPECT the residual: ~20 payload structs is real upfront retrofit cost; the named
escape valve if friction ever bites is a single `misc(code_str, span, msg)`
free-text constructor (the rustc-`allow` analog), but the human/conductor lean
against shipping it pre-emptively (camel's nose).

WHAT WAS REFUSED BY NAME (`22B` §2, all +SURE): rustc's Fluent/derive DSL (`refuse-1`,
the named refusal); i18n entirely (`refuse-2` — one audience language; message text is
English literals at the construction site, decoupled from the stable `code`);
Menhir-style generate-the-complete-set + heavyweight compare CI (`refuse-3` — Dorc's
give-up sites are *nameable source points*, a structural advantage over derived
automaton states, so we take the cheap exhaustive-enum half); Elm's NO-catalog pole
(`refuse-4` — Elm is one-author, Dorc is multi-author infra where registration rots
silently); and golden-testing the rendered prose (`refuse-5` — snapshot the STRUCTURED
value, not the narrative text, which churns on every wording tweak).

CARRY-FORWARD KNOWN GAP: the diag-API's *self-report* overstates in one place — see §6
(the catalog-discipline fix wave) and §2's gate-vacuity finding. The migration was
behavior-preserving (legacy `Diagnostic` coexisted via `to_legacy` until B4b emptied
the allow-list and deleted `diag::legacy`); ZERO golden diffs across the whole arc.

---

## 2. The provenance plane + the ru-11 one-way weld (arch-1)

WHAT IT IS. A `ProvId` arena (`core/src/prov.rs`) of hash-consed derivation/origin
nodes. `ProvId` is **`!Ord`** (`prov.rs:55` — `NonZeroU32`, no `Ord` impl) by design:
it cannot key a decision `BTreeMap`, which is the *structural* half of the weld.
`OriginKind` (`prov.rs:76`) carries the source tiers: `BookSource` (loc-user-src),
`TopCause` (the analyzer's give-up points — "the single most load-bearing kind"),
`Join` (k-capped control-flow merge), and **`OracleClaim` / `ProbeResult` RESERVED —
declared but NOT minted** (the claim-vs-receipt trust axis, surfaces when a
probe-sourced observable carries provenance into the why-lens). +SURE, source-verified.

WHY IT MATTERS. This is the provenance/why architecture and the welds future work must
honor. The governing ruling:

- **ru-11 ONE-WAY WELD (load-bearing, `224` §7).** Receipts are FULLY one-way: receipts
  are DECISION-INERT — they may influence NO license, NO fold, NO disposition, NO
  Error-class diagnostic, ever. The "may REFUSE or EXPLAIN" allowance from the GATE-1
  framing was OMITTED from the principle itself at the human's direction ("hard agree…
  welded"). Any future receipt-prompted behavior is a WELD RE-LITIGATION (a deliberate
  human-gated change), never a default loosening — the receipt-content must be
  re-derived as a first-class FACT first. The why-lens (§3) is the only consumer, and
  it is a RENDERER (exempt-plane), never a decider.

- **THE TWO SURFACES (rec-1, ru-12 + ru-20; `spike/CLAUDE.md` standing rulings).** Two
  PLANES, contracts are plane-based:
  - the shipped/off-ramp `.sh` ARTIFACT is **byte-floored and receipt-free** —
    byte-identical under receipt-stripping, *including its comments* (the ru-12 floor);
    the existing fact-plane disposition/provenance comments stand and grow NOTHING;
  - the PLAN-RENDER surface (TUI/CLI presentation, `why`-query) is NOT an artifact: it
    is the sanctioned home for per-line claimed-vs-proven disclosure, OVERLAID on the
    artifact bytes, never embedded.
  This is enforced at render by `render_artifact_comment` (`diag.rs:1092`) whose match
  admits only fact-plane content (a render-refusal's site, never prose; everything else
  returns `None`) — the receipt fields are simply not in scope for the artifact renderer.
  +SURE. This collision (two genuine human directions meeting at the plan-render) was
  THE find of the conflict-sweep (held-1, confirmed by two independent reviewers RV1+RV2)
  and is the reason the four UIs are "consumers, never contract subjects" (§7).

- **THE ERASABILITY GATE + decision-digest** (`plan/src/erasability.rs`; the gate in
  `plan/tests/erasability.rs`). The identity plane (dispositions + license fields +
  artifact bytes incl. comments + Error-class diags keyed `(code, site, severity)`)
  must be byte-identical across a normal run and a receipts-stripped/VARIED run
  (adversarial variance — reversed origin order, sentinel ids, varied seed). The
  partition is enforced WITHOUT proc-macros by **exhaustive destructuring, no `..`**
  (`erasability.rs:18-28`): add a field and the canon stops compiling until the author
  classifies it identity (fold into bytes) or exempt (drop WITH a named reason). The
  exempt plane is a CLOSED enum `Exempt{Explanation, ReceiptId, OriginOrdering, Timing}`
  (`erasability.rs:51`) — include-by-default is the safe direction (a spurious
  identity-diff is loud-but-fixable; a wrongly-exempted leak is silent — LLVM debugify's
  bias). +SURE on the mechanism.

THE KEY INSIGHT — and the most important correction the round made about its own work
(x-1 / `224` §11 self-audit; ratified by the human 2026-06-14, `224` §10):

> The TYPE SYSTEM does the real enforcement. Structural leaks are UNREPRESENTABLE —
> they won't compile (`ProvId !Ord`, the ⊤-cause excluded from `Reach`'s Eq, the
> render-partition scope). So the runtime erasability gate is only a PARTIAL backstop
> for *type-valid SEMANTIC* leaks (an `if reach.top_cause().is_some() {…}` written into
> a decision path) — a narrow class whose coverage is itself partial (the scrambler
> varies values, not presence). AND the gate is **vacuous-at-HEAD**: it passes by
> DISUSE — nothing in the decision pipeline reads a receipt (`top_cause()` had zero
> callers when this was found; the witness is populated yet the canon omits it). So
> run-A ≡ run-B because the perturbed data is write-only, NOT because a live consumer
> was driven under variance — the 19I §3 "passes because a fixture fed the right value"
> trap, in GATE form. +SURE (two independent builders agree the *defenses* hold).

CONSEQUENCE — read the gate as "inertness STRUCTURALLY enforced, behaviourally
UNEXERCISED," never "inertness proven end-to-end." The corollary that a future agent
MUST honor: the original "gate-obligation" (a durable negative-control that the why-lens
de-vacuums the gate) was DROPPED entirely by human ruling — a negative control is
impossible without weakening the types (= testing a different program), and the compiler
already does the structural enforcement. So the x-1 "vacuous gate" alarm is largely a
RED HERRING; the gate stays the cheap partial type-backstop it is. The forward weld
HOLDS at HEAD (fd-B, XC-2, conductor-verified in source: render fns don't read exempt
fields, `Reach::Top` cause excluded from Eq). The ONE named forward hazard to fence: a
future suppression tie-break that orders dispositions by which-cause-won WOULD breach —
STOP and flag if that ever wants to be written (`22D` §2; and the dormant
`CmdsubOperandTop`-derives-Eq-over-cause watch-point, harmless today, XC-3 finding).

Subtlety worth keeping: excluding the ⊤-cause from `Reach`'s Eq is a TERMINATION
requirement, not merely a contract — the fixpoint's convergence test IS the `joined ==
state[w]` Eq, so a cause-sensitive Eq never reaches a fixed point (`224` §11 owed-1, B1
build-confirmed). The `loc`/where-provenance plane (Span/SpanEdit) stays SEPARATE from
the why/lineage plane because their propagation laws differ (copying vs logical
dependence); fusing them recreates 111's coarsest-tier composition loss (g1-4, ru-3 —
"encode that division clearly into the typings").

---

## 3. The why-lens — the first receipt-READER (arch-2, built as 22D)

WHAT IT IS. The first real receipt-CONSUMER, built for its OWN user-facing value (not as
a gate vehicle): a per-line "why did this command run (never elided)?" disclosure on the
render surface. A book with a top-level `$(…)`-forced command, run through the plan
render, shows on STDERR `why: ran because <cause>; <remediation hint>` with the cause
wired from the real arena. +SURE — harvested green (`f40dded`), XC-3 clean (5 attacks,
none land), e2e byte-identical artifact confirmed.

WHY IT MATTERS. It is the disclosure UX and the receipt-consumer pattern the real Dorc
follows. It realizes **`dir-soundiness-ux`**: frontload the unsoundness where the human
reads, AT the decision point (STALENESS-AUDIT). The durable shape:

- **Emit-at-origin, post-mint** (the corrected mvs-A, `22D` §1 stage-1; the XC-2
  dispatch-blocker fd-A). The ⊤-cause is minted in `mint_top_causes`, which runs AFTER
  the effects pass (a node's opaqueness is the effects pass's OUTPUT — the ordering is
  inherent). So the typed diag carrying the cause is assembled/finalized POST-mint,
  reading `top_causes[node]` — NOT at the kernel-early emit site, and the effects pass
  stays a pure `Fn` (no `&mut arena` threaded through `solve`). The mechanism: a
  `&mut Vec<CmdsubTop>` collector through the pass, finalize the typed `Diag` after the
  mint. +SURE. The per-node cause keys on the CFG node's span (the whole command), so
  the why is "this COMMAND went ⊤," not operand-level — correct for a why-lens;
  operand-level pairing is aspirational, not required.
- **Render-surface only, never the artifact** (rec-1 weld): the `why:` line is on
  STDERR, prefixed `why:` and never `error[`, so e2e gate-3's stderr-error-floor ignores
  it — the why-lens is ADDITIVE, never case-failing (`cli/src/main.rs:485`).
- **The reliability quadrant is honest** (fd-G): the why-lens covers the
  reliable-oracle value-⊤ case ONLY. The oracle-lifter give-up codes carry no cause +
  `site()==None`, so the why-lens reads NOTHING for them — they render their own
  message; the why-lens does not overclaim "every forced-run has a why."

REMEDIATION VOCABULARY — ru-27 (HOW-NOT-WHO; human 2026-06-14, LOW-PRIORITY /
SPIKE-DEFERS / greenfield direction only). The remediation vocabulary should be cut
along **HOW the fix is done, NOT who does it** — Dorc can never know who authored the
opaque sh (human / AI / curl'd-from-GitHub / coworker library). Corollary: do NOT encode
oracle-vs-book ANYWHERE (often the same file, same author — just shorthands). The current
`RemediationClass{AuthorOracle, AddDeclaration, FixBookLine, Structural}`
(`diag.rs:687`) LEANS WHO (`AuthorOracle`/`FixBookLine` bake the role). DISPOSITION:
`FixBookLine`/`Structural` are FINE for the spike (human OK'd twice); the how-axis re-cut
(resolve-dynamism / declare-identity / provide-model / structural-no-fix — note
`AddDeclaration` is already how-shaped) is greenfield work, recorded with an in-file
pointer only, surfacing at round-close as seeding-feedback. NO churn, NO half-rename.
The built why-lens disposed `CmdsubOperandTop ⇒ FixBookLine` because its text is
honest-CONDITIONAL ("to elide it, make the operand a literal Dorc can resolve+probe" —
it says IF you can make it static, doesn't lie); flagged for human override to
`Structural` if "don't imply it's the admin's fault when the dynamism may be essential"
is preferred. ~SUSPECT this is the right call; it is the one open remediation tag.

KNOWN COVERAGE GAP (carried, not a blocker): the user-visible `why:` stderr render had
no e2e pin at harvest — the render logic is unit-pinned and artifact byte-identity is
e2e-pinned, but the emission line itself wasn't (the e2e harness checks stdout +
expected-diagnostics, not the stderr `why:` line). Fixed alongside the #17 fix below.

---

## 4. Suppression / dedup soundness (22E + fr-2 = the VMCAI'12 sound-clustering paper)

WHAT IT GROUNDS. fr-2 = "Sound Non-Statistical Clustering of Static Analysis Alarms"
(Lee/Lee/Yi, VMCAI'12, graded B). The criterion: collapsing alarm A under alarm B is
SOUND iff a genuine dependence `B-false ⇒ A-false` holds. The trivially-sound special
case is **syntactic clustering** — A is ⊤ *solely because* B's ⊤ flowed into it;
dependence is structural, needs no refutation. OVER-SUPPRESSION = collapsing two alarms
that are merely CORRELATED but INDEPENDENT, hiding the second.

WHERE DORC SITS. Dorc's dedup lives ENTIRELY in the paper's trivially-sound SYNTACTIC
corner: collapse a ⊤-origin's pure poison-descendants by shared cause-`ProvId`, read
straight off the dataflow. So sound clustering transfers to Dorc ONLY in the
pure-propagation/syntactic regime — anything correlated-but-independent would need a
refutation-proof Dorc does not have. The design discipline: **STAY IN PURE PROPAGATION**
(dc-7, `22E` §1; lifts ~SUSPECT → qualified +SURE). The dedup is keyed
**`(cause, site)`, NOT cause alone** (`cli/src/main.rs:497`, the x2-fd1 fix) — two
inlined call-sites of the same wrapper get the SAME body AstId (`inv-leaf-seam`) hence
the same cause `ProvId`, so cause-alone would collapse two genuinely-independent forced
runs; the `site` half (the stable `site N.M` leaf) keeps them separately disclosed while
still deduping a true re-disclosure. +SURE.

THE REFUTATION CORRECTION (human, 2026-06-14, post-clear; CORRECTS 22E §1 — `notes/`
are not kept current, the ledger is the live record). 22E overclaimed that the paper's
refutation MECHANISM is "welded out." Wrong: **ru-13 bans IN-ENGINE back-propagation /
phase-fusing** (feedback edges WITHIN one analysis run), NOT re-running the whole pure-
FORWARD analyzer on a counterfactual input. Refinement-by-refutation IS achievable as
**N independent forward passes**: replace a ⊤-origin AST node with a known-valid node,
RE-RUN, drop the warnings that vanish — weld-compatible, and ALREADY the planned
mechanism for ru-13's own retraction-by-recompute + the run-twice erasability/replay
gates. +SURE that it's possible/compatible. ~SUSPECT (human correction — do NOT
overclaim) on COST: the rerun is NOT necessarily cheap. The "analysis ≪ network"
reasoning is the APPLY phase (network-dominated), NOT the why-phase (a local
analysis/render concern), so a rerun-to-fixpoint there is not free and not masked by
network latency. Default posture: correctness-over-perf for now, EXPLICITLY FLAGGED that
a why-phase refutation-rerun may bite. Forward-only stays chosen for simple/boring/
verifiable, not because backward is impossible.

THE TWO-DEDUP-MECHANISM DIRECTION (human point-3, ratified as design DIRECTION, NOT
this-spike work): mechanism-1 by-construction (collapse a ⊤-origin's pure poison-
descendants by shared cause, keyed `(cause, site)`; sound for the syntactic/propagation
class; errs toward OVER-disclosure = safe; BUILD NOW — done) + mechanism-2 by-refutation
(the rerun, for genuinely-dependent-but-NOT-shared-cause warnings only a re-run reveals;
DEFERRED, not weld-banned; build only if disclosure-noise ever demands tighter collapse).

The remaining real over-suppression (x2-fd2, `22E` §2, DEFERRED as a documented cut):
`command_effect` returns `Opaque` on the FIRST ⊤ operand and never inspects later ones,
so `cmd "$(a)" "$(b)"` discloses only operand 1 — an onion-peel UX (fix operand 1,
operand 2 surfaces; the f-3 onion-peel anticipated at GATE-1). Disclosure-only (the
command ALWAYS runs — kFAIL-perform), so deferrable; the bigger fix touches the
effects-pass + operand-span cause-keying. WHY IT MATTERS: this is the suppression
posture (stay-in-pure-propagation) and a recurring tool (whole-analyzer rerun) the real
Dorc inherits — and the retraction/change-handling story rides the same rerun mechanism.

---

## 5. The engine is CROSS-SITE + certainty rides value-flow (r22 findings about the EXISTING engine)

These are foundational TRUTHS about how the analyzer reasons, surfaced (not built) this
round when the live-plan crosscheck attacked a naive "facts are per-site independent"
premise. They are fair game as r22 discoveries (the r23 PLANS that consume them are not).

- **The fold is CROSS-SITE, value-dependent** (+SURE, source-confirmed `fold.rs`
  `eval_and_or` / `disposition_for`). The premise "facts are per-site independent" is
  FALSE — Dorc is a dataflow engine. A CONTROLLER leaf's Status can mark a *different*
  (body) leaf `Omit`, taking precedence over a convergence-`Replace`. Dispositions are
  cross-site and value-dependent; "ACC-4 per-host independence" is VACUOUS (`build_plan`
  is pure ⇒ independent by construction — tests the easy direction). The live-plan
  crosscheck's "monotonicity broken" headline was an artifact of an UNDER-SPECIFIED
  contract mis-aiming the adversary at a strawman — method lesson: state a model's
  assumptions up front or the adversary attacks a strawman.
- **A value's certainty rides VALUE-FLOW (information-flow), and you CAN recover
  certainty by computing** (+SURE). A consumed channel's `Predicted<T>` flips
  `Value → Top` when a DISAGREEING same-cell fact merges (`merge_observable` =
  meet-toward-⊤), needing NO TOCTOU. But certainty is RECOVERABLE: `cmd || true` is
  provably rc-0 over a ⊤ left operand (door-3 `StatusInvariant`, `inv-one-observable`) —
  the left's Status is consumed-in-form but dead-in-fact, so a ⊤ there never blocks. So
  "⊤ in" does not monotonically mean "⊤ out"; computation can re-establish a known value.
- **The certainty-tier has no clean type home today** (the lesson that closed the round,
  human 2026-06-14; `plans/22H` §1 reworked). Pure-CFG-structural folds ARE monotone
  (fully-trusted immutable input); the probe/oracle-tainted replacement-CONTENT is not.
  The taint type the human wanted certainty-claims to rest on EXISTS — `OriginKind`
  (`core/src/prov.rs`) — but is ru-11 DECISION-INERT (grounds EXPLANATION, never a
  decision; `OracleClaim`/`ProbeResult` RESERVED-not-minted). The decision-plane
  certainty type `Predicted<T>` (`Value`/`Top`) is SOURCE-BLIND. So a decision-driving
  "pure-CFG-can-only-downgrade vs tainted" certainty-tier has NO clean home — it's an r23
  design question (a NEW ru-11-compatible decision-plane source-tag, or stay best-effort
  + explanation-tiered). LESSON (carry it): do NOT write certainty-tier claims into
  durable docs without the type-level basis; downgrade when the basis is hand-waved.

WHY THESE MATTER. They are the ground truth a future agent needs before reasoning about
incremental re-analysis, certainty gradients, or any "can we tighten this" question —
the engine is a cross-site dataflow machine whose certainty is an information-flow
property, and the most foundational of the round's accidental discoveries.

---

## 6. Catalog discipline — the x-3 fix wave (keeping the catalog honest)

WHAT IT IS. The fix wave (act-1..6, `224` §10) that made the diagnostic catalog
GENUINELY-guarded after an adversarial crosscheck (x-3, two passes ~535K Fable-tokens)
found the gate machinery was largely VACUOUS while green. WHY IT MATTERS: this is the
test-discipline that keeps the catalog (§1) honest — and a standing demonstration that
"green" ≠ "guarded."

What the x-3 pair found and the wave fixed (all +SURE, source-landed, green at
`d003e04`, 505/0/1):

- **Registry-bypass removed** (act-1): `check.rs` `lift_failure` hardcoded `Error` via
  slug-extraction, BYPASSING `registry()` — "severity only from the registry" was FALSE
  for check-codes, and a future registry edit was a silent no-op. Routed onto the typed
  path; a symbolic emit-vs-registry agreement test added (compares emitted severity to
  `registry()` symbolically — survives a future re-grade). Note the residual: that
  agreement test's SEVERITY half is vacuous-at-HEAD (both check codes are registry-Error,
  no non-Error lift_failure code to distinguish) — known, low-severity, accepted (XC-1).
- **The "every variant constructed" gate was VACUOUS** (act-3): `diag.rs`'s OWN match
  arms satisfied the reachability grep (delete a sole production emit → still green). The
  scan basis switched to production-emit-source-only (excludes core), negative-controlled.
  Residual-b (carried into B8, +SURE): the scan still doesn't exclude non-core in-file
  `#[cfg(test)]` modules, so a test-only literal construction can satisfy it — the scan
  stays best-effort; the per-code must-emit PINS are the real liveness instrument.
- **The retire-guard was TAUTOLOGICAL** (act-2): filter-then-assert-same-membership —
  full silent retirement passed green. Rewritten with a real committed-source→list
  direction + an anti-vacuity guard (empty extraction fails loud) + two genuine negative
  controls (catch_unwind asserting panic).
- **Every code has a driving must-emit PIN** (B8 PART C): all 23 codes mapped to ≥1
  driving test; the 7 PINNED-BY-NOTHING (e.g. `effect-kind-disagreement`,
  `oracle-missing-kind`, the two check-codes) got per-code pins driving `lift_predicts` /
  the real give-up path over real sh and asserting code identity — genuinely closing the
  direct-construction vacuity. +SURE.
- **The f-3b member-⊤ dedup story was BACKWARDS** (B8 PART A + the 22-q4 deep-dive — the
  human pushed on the foundational "members are ⊤-free" assumption and was right). FACTS
  (all +SURE, code-confirmed): `member_argv` is NOT ⊤-free (`value.rs` `record_member_sites`
  has no ⊤-gate — `for p in a b; do cmd "$p" $(date); done` yields a ⊤-bearing member
  argv); the harvested doc claiming "None-site UNREACHABLE for a ⊤" was BACKWARDS — the
  None-site emit IS reached in production and the suppress is a LIVE dedup (a ⊤ member →
  Opaque → family collapses → single-cell fallback discloses once with the real span).
  CRUCIALLY: NO mis-elision (a ⊤ operand always returns Opaque ⇒ MustRun ⇒ runs;
  kFAIL-perform holds) — the bug was in the REASONING/DOCS, not behavior. The proposed
  "assert members concrete" fix (opt-4) was UNSOUND and caught BEFORE implementing (it
  would fire on the valid input) — the value of the human's check + never-vouch. The
  corrected doc + a count==1 pinning test landed.

THE META-LESSON (the round's most valuable crosscheck data point, XC-1): convergence of
two SAME-MODEL agents is NOT independent confirmation when they share a method gap. The
two most prominent Pair-A findings ("`syntax-unsupported` has no behavioral pin") were
FALSE POSITIVES from a SHARED BLIND SPOT — both agents ran only `cargo test`, never
`sh e2e/run.sh`, and missed that e2e gate-3 matches every Error diag against per-case
`expected-diagnostics` patterns that EMBED the code (a wrong code = case FAILS). Post-
Fable (no higher tier to catch cross-cutting error), VERIFY-SURVIVORS-IN-SOURCE is
non-negotiable; dial UP adversarial-crosscheck precisely because same-model passes are
less potent and more necessary.

ru-26 DISCIPLINE (welded, `spike/CLAUDE.md`): any implementation shaped by a "would
churn unnecessarily" scope-cut MUST carry a nearby inline note saying so, so the cut
can NEVER leak silently into greenfield work referencing the spike. Live instances: the
two needle-shape scan honesty notes (single-line-arm-only; non-core-cfg(test) basis) and
"anything not-handling-stderr says so locally." A leaked spike constraint defeats the
point of a spike.

---

## 7. ui-A's confirmation — UIs fall out of the plane split (22F)

WHAT IT IS. A minimal multi-mode CLI (`Mode{Probe, Plan, Apply, RoundTrip}`,
`cli/src/main.rs`) over ONE kernel call; mode routes ONLY stdout/stderr via a ~40-line
`advisory_filter` projection (`advisory = !matches!(mode, Apply)`). The legacy bare-flag
round-trip kept verbatim ⇒ the e2e harness untouched, 99/99 byte-identical. +SURE,
harvested green.

WHY IT MATTERS. It VALIDATES that contracts are plane-based, empirically. The key
finding (22F-fd1, +SURE): **rec-1's two surfaces ALREADY physically existed** in the
single-shot driver — the byte-floored artifact was already on stdout, the receipt/
disclosure plane already on stderr. ui-A only NAMED them; the four modes are three of
eight cells of three independent projection booleans. The UI fell OUT of the plane split
exactly as ru-20 predicted ("UIs are consumers, never contract subjects"). No
engine-design tension surfaced — every tension ui-A found is render-surface-contract.
The off-ramp `apply` console cut (ru-29-ratified): keep Error-floor + decision-digest
(identity, not receipt), DROP advisory — SAFE ONLY BECAUSE the severity registry floors
every must-not-silently-ship code as Error ("trust the registry"); standing guard: never
"clean up" apply to zero-stderr (reopens the silent-ship hole). ru-20's four-UI
enumeration maps ~1:1 onto 22B §4's one-value-four-projections (artifact-comment /
TUI-dashboard / CLI-narrative / OOB-lane).

---

## 8. Standing rulings + welded invariants a future agent MUST respect

THE ROUND-22 RULINGS (compact; full text `224` §7 + `spike/CLAUDE.md`):

- **ru-11** — receipts FULLY one-way / decision-inert; any receipt-prompted behavior is
  a weld re-litigation (§2).
- **ru-12 / rec-1** — two surfaces: byte-floored receipt-free `.sh` artifact (incl.
  comments) vs the render surface that carries disclosure overlaid (§2).
- **ru-13** — NO in-engine back-propagation / phase-fusing; a whole-analyzer FORWARD
  rerun is FINE (and is the planned retraction/refutation mechanism) (§4).
- **ru-17 / held-4** — the diag-API is BATTLEFIELD-BOUND (design-for-keeps; the sole
  sanctioned exception to spike-disposability) (§1).
- **ru-26** — churn-avoidance scope-cuts MUST be disclosed in-code; never leak into
  greenfield (§6).
- **ru-27** — remediation vocabulary is HOW-NOT-WHO; never encode oracle-vs-book;
  greenfield direction, spike-defers (§3).
- (supporting: **ru-16** typed payloads; **ru-18/19** DST/re-runability is product-tier
  with replay-gate ceremony; **ru-20** four UIs are consumers; **ru-22/23**
  ingestion-as-declassification — quarantine-by-default + human-held fetch, a recorded
  future security re-pass `flag-security-round-2`; **ru-28/29** the live-plan reframe →
  deferred to r23.)

THE WELDED `inv-*` INVARIANTS (do not violate; `spike/CLAUDE.md`): `inv-no-throw`
(every stage returns `Carrier<T>`, never panics — errors are data); `inv-determinism`
(the kernel is a pure function; ordered collections only, no `HashMap` iterated into
output, no async/clock/RNG/fs/net directly or transitively); `inv-kfail` (Probe → never
mutate / `kFAIL-withhold`; Apply → never elide a needed mutation / `kFAIL-perform`);
`inv-top-reject` (unmodeled ⇒ ⊤, rejected loudly never silently best-effort'd);
`inv-leaf-seam` (executable work is individually-wrappable leaves with a stable
`LeafId → AstId` back-map — NOTE the non-injective-under-inlining nuance that drove
x2-fd1); `inv-site-keyed-results` (probe results keyed by command-site, two
same-command sites must not collapse); `inv-probe-sourced-values` (a replacement
reproduces ONLY probe-provenance values — no fabricated rc=0/stdout; consumption-
coverage is the load-bearing precondition); `inv-one-observable` (exactly ONE concept of
a command's observable over `{Effect, Status, Stdout, Stderr}`; the consumed Status
splits Relaxable / Invariant / Iterated); plus `inv-must-may`, `inv-superposition`,
`inv-referent-agnostic`, `inv-no-unsafe`.

---

## 9. Deferred-forward (ONE line, do not elaborate)

Two forward arcs were surfaced this round and DEFERRED with seeds already written — the
analyzer's TIME axis (concurrent-probe streaming → incremental per-host re-analysis):
`plans/22H` (live-plan); and the CERTAINTY axis (converting accidentally-collapsed
booleans into proper gradients, trust as one exemplar): `plans/230` (best-effort /
collapsed-gradients). They are orthogonal, likely multi-round, and out of scope for this
report; do not re-plan them here.

---

## Process / seeding-feedback

The round's reusable process lessons — fb-candidates for the human's global prompt / AGENTS
/ future conductors. Process evidence, never a claim of goodness.

CROSSCHECK HYGIENE (the round leaned hard on adversarial-crosscheck post-Fable; these sharpen it):
- **fb-adv-exclusions** — frame an adversarial pass with EXCLUSIONS (what's settled, don't
  re-derive), never INCLUSIONS (your suspected weak-points). Priming the adversary with
  where-you-think-it-breaks collapses its breadth onto your priors — it digs into what you
  already know and surfaces nothing new. Strip self-flagged weak-points (`~SUSPECT` marks,
  "attack this hardest" lists) from the ARTIFACT handed over, too. Give context +
  problem-space + WHY, and latitude. (Saved to memory; candidate for the
  adversarial-crosscheck skill itself.)
- **fb-underspec** — an UNDER-specified artifact mis-aims the adversary at a strawman. The
  live-plan crosscheck's "monotonicity broken" headline was an artifact of the contract not
  stating its single-pass / no-correction model; both passes over-worried an out-of-scope
  case. State a model's load-bearing assumptions up front.
- **fb-same-model** (XC-1, §6) — same-model convergence is NOT independent confirmation when
  the two passes share a method gap (both ran only `cargo test`, missed the e2e pin).
  Verify-survivors-IN-SOURCE is non-negotiable; post-Fable (no higher tier), dial UP
  crosscheck precisely because same-model passes are less potent and more necessary.
- **fb-verdict-boundary** (process-1, `224` §11) — a crosscheck on an
  inertness/provenance/security-adjacent component must return a PRE-SANITIZED verdict
  (disposition + neutral-engineering findings only). Banking the full hostile TRANSCRIPT
  carries enough loaded vocabulary to model-gate the conductor itself — the priming-prompt's
  sec-gate realized one level up; it cost multiple conductor deaths this round. Bank the
  verdict, not the transcript.

CONDUCTOR DISCIPLINE:
- **fb-design-crosscheck** — the conductor's OWN design needs the crosscheck, not just
  builders' code. 22C (the conductor's first arch-2 plan) had two load-bearing errors XC-2
  caught (mvs-A ordering backwards; a non-durable gate-obligation). The design→crosscheck→
  build loop applies upward, to design.
- **fb-durable-certainty** — do NOT write certainty-tier / monotonicity claims into durable
  planning docs without a type-level basis; downgrade to best-effort when the basis is
  hand-waved (the run-count-monotone overclaim, walked back in `plans/22H` §1).
- **fb-human-voice** — never author in the human's `>`-blockquote voice; that format is the
  human's only (a conductor slip this round, in `TODO-ADDTL.md`, immediately corrected).
  Agents write plain prose; the human annotates in `>`.

CARRIED / REINFORCED (prior fb, still live): **fb-16** harness token counts are
authoritative over agent self-report in the ledger; **fb-19** sonnet recursively
self-delegates → every builder brief carries an explicit no-subagents clamp; **ru-15** Opus
briefs drop inferable constraints (safety block + goal + reading pointers, not rule-lists).

SEEDING FOR THE HUMAN (prompt/AGENTS-level):
- The clean-context-subagent-extraction + conductor-augment pattern produced THIS report
  well — a fresh reader of the noisy notes surfaces what a conversation-biased conductor
  glosses. A good template for round-close synthesis.
- The gradient CRITERION the human added to `IMPLEMENTATION.md` (best-effort = a *failure
  gradient*; a gradient exists iff partial-benefit exists; mutation = no-gradient, coverage =
  gradient) is the load-bearing seed for r23 (`plans/230`) — the sharpest framing of "which
  booleans become gradients."

RETROSPECTIVE (one line): r22 delivered the entire LIVE / reporting half of errors+provenance
(the diagnostic spine + the provenance one-way weld + the why-lens) and SURFACED — without
prematurely building — its two foundational forward halves (the certainty/best-effort axis,
the time/streaming axis), each left with a crosschecked seed. A coherent stopping line.
