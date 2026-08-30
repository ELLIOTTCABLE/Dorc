User-aid needs (AID-NEEDS)
==========================

The living registry of *user aid* — the one product-category covering errors, warnings,
hints, lints, `dorc why`, provenance display, and durable receipts. Companion to
`ANALYZER-NEEDS.md`: that file tracks what the engine must *compute*; this one tracks
what the engine must *tell people*, and the mechanisms required to tell it correctly.
LLM-generated; add a row when a feature implies a new aid-class; keep rows current.

Why this registry exists: aid is deferred-by-nature ("lints and hints don't drive
core-engine-design-constraints") and therefore drops on the floor. Worse, the rationale
"it's fine, we can warn the user" must never wall off the question "should the core
mechanism *prevent* this instead?" — every row here is implicitly that question, kept
visible. Many rows look small from the user's side and need deep machinery (evidence
threading, provenance, replay) to produce *correctly*; the mechanism column is the point.

Engine-side build law for this category (narrative plane, receipt-backed why, the error catalog and
its authoring pipeline) lives in `Research/notes/27V`, `Research/plans/30R`, and `spike/CLAUDE.md`'s user-aid
block — not here. This file is the row-registry plus only the law needed to mint and
consume rows.

**Columns** — `aid` (slug) · `what the user learns` · `demand` (evidence consumed:
`text` book/oracle text · `orcl` loaded oracle-set · `topo` CFG/wall topology · `facts`
probe facts (± = partial/streaming) · `apply` apply outcomes · `invoc` invocation record
· `receipt` the posthoc receipt graph) · `moments` (see axes; `▶`=push `◀`=pull) · `grade`
(m measured · d derived · c authored-claim · g conjecture) · `mech` (engine mechanisms,
`an-*`/doc refs) · `st` (B built · S specified · D designed-deferred · O open · W welded
· X retired).


How to read the axes
--------------------

An aid-class is typed by three independent axes (the flat input-rung ladder of `27R` §3
survives only as named corners of axis A):

- **A · demand** — the evidence product it consumes, plus a *stability* bit: facts
  arrive over time in the reactive era (`26B`), and a push-fired aid that would retract
  under more arrivals churns trust; the plan-mint is the natural finality cut
  (`aid-plan-finality-discipline` below).
- **B · moment** — the user-situation it serves, and whether it is *pushed* (fires
  unprompted; relevance/fatigue dominates; selection must be ruthless) or *pulled* (the
  user asked; relevance is nearly free; derivability runs wide-open and the answer is
  maximal). Moments: `olo` oracle-author hot-loop · `blo` book-author tuning loop ·
  `rev` plan review ("this looks wrong") · `tui` mid-build curiosity query · `post`
  post-apply annoyed · `ci` machine consumer.
- **C · grade** — the epistemic tier of the statement itself (measured / derived /
  authored-claim / conjecture).

Surfaces (lint, plan render, apply console, `dorc why`, receipt-backed why, future TUI, CI mode) are
**selection policies over rows**, defined at the end — a row is never "owned" by a
surface. `kWARN`'s spike-era tune-high is a push-policy setting, not a row property; the
post-spike destination for push surfaces is precise-or-silent (`plans/111`), reached by
tuning selection, never by deleting detection.


Law — what a row-minter and row-consumer must know (cite as `AID-NEEDS:law-…`)
------------------------------------------------------------------------------

- **law-rows-are-classes** — rows are aid-*classes* (order ~10²), never error codes.
  Codes (order ~10³ at production grade; close siblings from world-state variants; one
  full prose writeup each, no generic fallback) live in the catalog and its defining
  cases — never here. Code prose is DEFINING-CASE-DERIVED (`282` errorloom generation
  flip): the case transcript is the authoring surface, and the committed catalog is
  regenerated from it by promote and fixpoint-protected against hand-edits. Tests and
  code cite rows as `AID-NEEDS:slug`. If this file starts enumerating messages, it has
  failed; split the class instead.
- **law-two-planes-opposite-fail** (`26C` §5b) — the license plane fails toward
  *unsureness*; the aid plane fails toward *narration with attributed confidence*. Aid
  may consume anything, including analyses vetoed for licensing (tool-aware text
  reasoning, host observation, oracle-contributed narration — the `26C` feeder classes),
  each attributed by class and forever display-tier. The aid-narrative plane is
  decision-inert at the type level: no path from any row's machinery into any
  license-plane input.
  Lint-clean licenses nothing; silence-licenses-nothing runs in both directions.
- **law-collapse-mints-narrative** (née law-collapse-mints-evidence) — every point where
  the engine narrows for safety (meet-to-⊤, refuse, decline, wall, demote, cancel) mints a
  decision-inert *narrative* record (`aid::CollapseNarrative`) carrying the collapse's
  *operands*, at the moment of collapse. A new safety-narrowing without its narrative mint
  (and, usually, its row here) is a defect. "Evidence" is deliberately reserved for
  possible future correctness-plane use (`288:rul-narrative-layer-naming`).
  Mechanics: `27V` §1. AS-BUILT correction (`30Kb`): the class census is gate-held, but
  `WallFormation` is still minted only on the risk-accepted leaf path; honest walls and
  always-at-node/non-leaf invalidators lack a truthful participant because the current operand
  requires `LeafId`. The mint half is therefore NOT complete. The broader consume gap remains
  `289:seam-narrative-render-unconsumed`, owned by the arrangement-home round; fixing the mint
  requires an effective-wall participant/owner operand, never fabricated leaf attribution.
- **law-trust-tier-is-syntax** — the epistemic tier of every rendered link (strawman
  spellings: measured / vouched / ran / claimed / derived / consented) is a typed field
  rendered uniformly by arrangement code; prose never hand-writes epistemics, so a claim
  cannot be dressed as a measurement. The tier set and its typed rendering are the law;
  all render particulars stay unwelded pending implementation (`KNOBS:kFLOW`;
  `27V:rul-output-form-unwelded`). Mis-attribution is the worst aid failure
  (`271:rul-sin-ordering`); where certainty runs out, say so rather than rounding up.
- **law-pull-runs-wide-open** — on pull surfaces the user asked: DERIVABILITY runs
  wide-open (everything the engine holds is reachable; the exhaustive tier is one
  LABELED step away, `--all`), and the REGISTER is maximal — any pull answer is
  generous next to the jealously-meted, in-your-way push surfaces; that
  pull-vs-push contrast is this law's original content and stands. WITHIN a pull
  surface, the default is curated per law-selection-is-goal-derived — and curation
  is a TUNING, not a minimalism mandate: "carefully" does not mean "minimally",
  curating maximally is legal; curating WRONG is the one forbidden thing. On push
  surfaces selection stays ruthless and root-cause-only (AGENTS fail-fast;
  `(cause, site)`-keyed dedup; stay-in-pure-propagation — `22E`).
- **law-selection-is-goal-derived** (human-typed 2026-07-26; banked `28H`) — there
  is ONE global fact/derivation model; user-facing surfaces drive only CONSENT and
  GOAL (`why` = understanding · `why --probe` = understanding + consent-to-live-
  probe · `plan --why` = preparing-to-reconcile, tuned toward understanding; the
  `28E` §8 goals/consent matrix is thereby RULED, no longer vibe). What a default
  render shows is derived BACKWARDS from the derived user-goal under the
  receipt/address/world state — dynamic, never a static dump of what is at hand.
  Curation is either DEEPLY EFFECTIVE or ENTIRELY ABSENT (the dag-explanation-ux
  round: verbosity tastes good and hurts task success; mis-curation was the worst
  failure mode): `--all` is the absent-curation tier, labeled; every curated
  default must be conscious of what it shows and why. The dichotomy governs
  curation CORRECTNESS, never density — density is tunable within the goal.
  ERA POSTURE (human-typed, same sitting): during the spike the concern is THE
  ARCHITECTURE TO TUNE prosody/verbosity, far more than where it is tuned —
  generate more, better, extra, noisy output data, because that forces the
  architecture to track it; tuning down later is cheap, tuning up is very hard
  (the kWARN tune-high posture, generalized). The goal-derivation FORM is the
  requirement; its present setting leans generous. The typings and
  output-construction follow the form "<goal> leaves-user-wanting-of <info-piece>
  when world is in <state>" — as architecture shape and internal narration (the
  derived goal is inspectable DATA the render can name), explicitly NOT a
  prose-style catalog/register (refused as overengineering). Worked consequence:
  an authored decline is noise at default verbosity unless the asked question
  implicates it — and the calculus flips with receipt state (in an
  otherwise-quiet receipt, the quiet classes may BE the interesting thing).
- **law-aid-adds-no-consent-moments** (`26B:rul-one-attention-moment`) — aid never adds
  interaction moments; everything front-loads into the one presented plan or waits to be
  pulled.
- **law-render-overlay-never-artifact** (rec-1, welded) — aid lives on render surfaces,
  overlaid; the byte-floored `.sh` artifact never carries it. The apply console keeps
  the error floor + decision digest only (`22F:advisory-vs-error-cut` (née 22F-fd6)).
- **law-lineno-identity** (`24H` ack-2ii) — one line-number space, the source file's,
  everywhere, round-trippable into `dorc why` addresses.
- **law-plain-language-surfaces** (`24H` ack-4) — no jargon on user-facing surfaces (no
  "⊤", no corpus vocabulary); quality, unambiguous English.
- **law-whylog-is-sensitive** — receipt contents are sensitive without promising secret
  recognition. Rich receipts protect opaque detail; plain projection and structural metadata
  remain bounded shape claims, never “safe to share.” Fences: `an-diag-secret-taint`,
  `an-output-sanitization`, `26B:need-scrub-before-freeze`, `24R:repurp-finding12`. Receipt
  read-back is report-only and never a verdict/reuse cache; no recorded value converts to action.
- **law-report-surfaces-speak-sh** (`27W:rul-report-surface-massaging`) — report/why
  surfaces show *code* by preference (an oracle's own arm, comments riding along as
  display, never parsed), and may massage what they show (slice to contributing lines,
  attach whole adjacent comments, mark elisions): 'attribution' in the context
  of show-the-code means implying authorship and directing repair, not
  necessarily byte-for-byte reproduction. The byte-floor laws bind the
  plan/artifact planes only; display-sh needn't masquerade as a runnable
  artifact.


Registry
--------

### Plan-surface classes (mostly ▶ push)

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-plan-line-reason | every surviving line's reason (run/verify/converged), every elision's cause, in the rendered plan | text+orcl+topo+facts | rev▶ blo▶ | m+d | an-narrowed-plan, rul-attention-honesty | B |
| aid-plan-summary-tally | the "N to run, M to verify (K skipped)" contract line | same | rev▶ ci▶ | d | plan-summary grammar (stable slugs) | B |
| aid-first-wall-nudge | which unmodeled command forms the first wall; how many sites an oracle for it would recover | text+orcl+topo | rev▶ blo▶ olo▶ | d | first-wall walk (`cli`); USER_STORY st.2–3 | B |
| aid-oracle-coverage-nudge | "this looks like a guard — an oracle would lift it" (company-it-keeps enrichment) | text+topo | blo▶ olo▶ | g | an-enrichment-nudge | S |
| aid-unloaded-sibling-oracle | sibling `*.oracle.sh` files exist but are not loaded (suggest, never auto-load) | invoc+text | blo▶ rev▶ | d | `24H` ack-6 | S |
| aid-loaded-oracle-inventory | which `dorc-lang` sources/dirs were actually loaded this run | invoc | rev▶ ci▶ | m | ordered receipt source identities with source class + digest; full valid-`dorc-lang` bytes withheld | B |
| aid-survives-attribution | whose at-most claim licensed each survival; the disjointness derivation; the resolver involved | facts+topo | rev▶ post◀ | m+c+d | SurvivalWitness; an-attribution-lanes | B |
| aid-guard-license-attribution | whose check guards each verify line, under whose vouch | facts+topo | rev▶ post◀ | c+d | GuardLicense lane | B |
| aid-region-edit-disclosure | a shared function-body edit (elision region) shown ONCE at the authored definition, marked universal over its contributing invocations with the route count; `why` walks definition→every licensing invocation and call→the shared edits it executes. The two why components render `[unwritten:]` | facts+topo | rev▶ post◀ | m+c+d | `plans/30L` render/why laws; `SpineRegionDecision.routes` complete by directive | B |
| aid-import-rewrite-disclosure | a generated plan's re-pointed or in-place-inlined import (the one edit Dorc reserves over a plan it generated) disclosed at the authored `.` (`plan-import-rewritten`, advisory-routed) and recorded as `RenderDecision::ImportRewritten`; the artifact bytes self-comment the edit. `why` does not yet carry import edits (residue) | text+invoc | rev▶ post◀ | d | `30Ng:rul-bundle-at-dorc-lang-boundaries`; `plan::ImportEdit` as a `Plan::decided` input | B |
| aid-artifact-form-explanation | which artifact form `auto` settled on and why it fell back (`artifact-form-fallback`); explicit-intent or stream-posture refusals BEFORE network naming the claimants/cause (`artifact-form-refused`, `artifact-publish-refused`); the lifted-section boundary in the artifact itself separates low-attention material from the book. All prose `[unwritten:]`; `--artifact-dir`/`--form` absent from the help page | invoc+text | rev▶ blo▶ | d | `30Ng` §§4–6 (stream posture; two-section review surface); `cli::artifact::Selection` | B |
| aid-cross-custody-dependency-narration | why a role's ambient helper cannot join its vouch: distinguish source-act-present-but-unaligned (selected dependency vs live binding/guard mismatch) from ambient-or-untraceable (ordinary shell resolution with no attributable selection); name call + live definition + lost capability, and present source/guard as value recovery rather than admission. Both preserve the runnable plan; whole-unit inventory is lint/pull, plan-push only when a reached site loses composition | text+orcl+topo | olo▶ blo▶ rev▶ post◀ | d | `30I:rul-cross-custody-distinction-is-narrative`; vouch-custody suspension | B |
| aid-carry-attribution | which cross-context carries happened, under which invariant lines + closure proof | facts+topo | rev▶ post◀ | c+d | `27C` §4(a); an-read-set-closure | B |
| aid-bundle-origin-chain | for generated plan/bundle code: every compilation locus through to the deepest content-matching original source, while retaining generated loci; absent/changed originals fall back honestly. Re-ingested markers mint aid-only `BundleOriginClaim`, never analytic identity | text+orcl+invoc | rev▶ olo▶ post◀ | c+d | an-locator-dag; an-graft-provenance; `30I` §9 | B |
| aid-run-cause-disclosure | why a line can never elide (unknowable operand etc.), with remediation | text+topo | rev▶ blo▶ | d | why-lens; `(cause,site)` dedup | B |
| aid-escalation-consent-legibility | the escalation dial × capability × entry-capable wrappers in effect | invoc+orcl | rev▶ | m | `27C`; catalog re-home rides `27V` | B |
| aid-caret-span-precision | which exact section of a compound a diagnostic means | text | all▶ | d | `24H` ack-8; `27U` caret dispatch (7 sites plumbed, multi-line frames, survey committed; two named deferrals) | B |
| aid-firehose-suppression | one honest aggregate instead of per-site unprobeable noise | text+topo | rev▶ | d | `24H` firehose fix | B |
| aid-plan-finality-discipline | push-hints fire only at stable/minted state (no retracting hints mid-build) | facts± | tui▶ rev▶ | — | `26B` finality; r26 seam | D |
| aid-sigpipe-flap-note | rc-141 sink-landings flagged "likely benign early-exit race" | facts | rev▶ | d | `279f` named class | B |

### Ask-surfaces (◀ pull; live and receipt-backed `dorc why`)

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-why-problems-report | zero-arg: the problematic subset of the current analysis (refusals, walls, guards, can't-tells) | text..facts | rev◀ blo◀ | m+d | `24H` ack-2i | B |
| aid-why-line-address | per-line interrogation by `book.sh:N` or content match | same | rev◀ tui◀ post◀ | m+d | law-lineno-identity | B |
| aid-why-license-chain | the full numbered chain behind any disposition, tier-worded per link, naked-trust links identified by construction, with re-measure + leverage-point epilogue | facts+topo+invoc | post◀ rev◀ | m+c+d | evidence plane + witnesses + minting-line threading (`27V`); the walker + flagship (`27U`) | B |
| aid-why-wall-narration | why a wall formed: which participant, which channel/coverage failure, what it costs downstream | text+orcl+topo | blo◀ olo◀ rev◀ | d | collapse evidence (walls); the truthful region/instance operand is threaded (`30L`) but `None`-populated until the wall-narration mint ruling widens the mint population (`30M`, the one remaining ruling) | S |
| aid-why-decline-narration | which oracle arm declined this shape and why — the arm itself inlined (show-the-code, massaged per law) | orcl+facts | olo◀ blo◀ post◀ | c+d | collapse evidence (declines) + `27W` classes + arm-inlining | B |
| aid-why-disagreement-narration | two establishers disagreed on one cell: who, where, which values | facts | rev◀ post◀ | m | collapse evidence (merge operands; `22H` §1) | S |
| aid-why-value-chain-narration | a captured value narrated through iteration hops and host transformations, best-effort, feeder-attributed | facts±+history | post◀ | m+c+g | `26C:need-why-explanation-lane` + feeders; r26 | D |
| aid-whylog-posthoc-why | receipt-rooted posthoc why after everything is apparently complete: selected root + causal graph closure, exact historical general-sh source, durable locators, current-source comparison, and explicit partial/authentication states | receipt graph | post◀ | m+c+d | `dorc-receipt::report::RecordedWhyFacts` is BUILT and sealed; CLI aid/weft arrangement is rehomed; report-only kernel re-derivation deferred | S (facts B; presentation/re-derivation pending) |

### Authoring-time classes (`dorc lint`; ▶ push, hot-loop-safe, never probes)

Lint's lane-local code namespace is RETIRED (`288` §5, landed): every dorc-MINTED lint finding is an
ordinary registry `DiagCode` with a defining case, so floors, severity policy, dedup keying,
`dorc why`-addressability, and the mint guarantee cover lint uniformly. FOREIGN relays
(`shellcheck:SC2086`, and the tolerant adapter's `external-text` location marker) stay source-tagged
relay strings forever — they are another tool's vocabulary, not ours. The human render's framed-vs-
compact split is a named SELECTION POLICY (`289:rul-lint-render-split-is-policy`) with a
`--terse`/`--verbose` dial, not a side effect of which findings happen to carry provenance.

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-lint-analysis-diagnostics | the no-world pipeline's diagnostics over given files | text+orcl | olo▶ blo▶ ci▶ | d | `27R` source-1 | B |
| aid-lint-unmodeled-inventory | per book: unmodeled families, first-wall position, degradation counts | text+orcl+topo | blo▶ ci▶ | d | `27R` source-2 | B |
| aid-lint-verdict-body-mechanicals | status-flattening hazards in verdict bodies (pipeline tails; the rest caught upstream as dialect) | text | olo▶ ci▶ | d | `27R` source-3; `279f` constant-rc candidate seamed | B |
| aid-lint-external-tool-relay | shellcheck/checkbashisms findings, strip-line-remapped, tolerant adapters, coverage-asserted envelope | text | olo▶ ci▶ | d | `27R` §4/§8b | B |
| aid-lint-reserved-name-checks | munge charclass/collision; book squats on `__role` namespace | text | olo▶ blo▶ | d | `oracle/reserved.rs`; catalog re-home rides `27V` | B |
| aid-lint-strip-floor-check | strip output parses/runs identically under both pinned floor shells | text | olo▶ ci▶ | d | two-binary floor; fence-rejection-rc; `27R` seam | S |
| aid-lint-wrapper-oracle-bar | wrapper-family quality: peel cross-check, argparse lints, self-vouch/footprint | text+orcl | olo▶ ci▶ | d | `24S:A6` | S |
| aid-lint-carrier-payload-bar | carrier/payload quality: which-arg-is-code gates, reconstruction differential, dorcism-in-payload | text+orcl | olo▶ ci▶ | d | `24T:P-A4` | S |
| aid-lint-kind-adjudicability-bar | kind-topology clauses machine-readable; binding smells; differential discharge — REQUIRED before kinds go community-shared | text+orcl | olo▶ ci▶ | d | `24S:A4`; unowned | S |
| aid-lint-oracle-solo-mode | oracle files linted with no book present | orcl | olo▶ | d | `dorc_oracle::validate` factored book-free (`27U` d4b); + the decline-inventory source | B |
| aid-authored-decline-classes | which shapes an oracle deliberately declines, and why (closed class set: unsound/unmodeled/interactive/hazard); routes the enhancement-nags honestly | text+orcl (facts± at tier-3) | olo▶ blo▶ rev▶ post◀ ci▶ | c | `27W` all three tiers live (`27U`: sink recognition, per-arm inventory, per-site classing, runtime drain + pairing); the drain runs on a controller-owned per-attempt scratch directory and degrades to an inert sink if that cannot be created (`spike/CLAUDE.md` decline-class-emission). No e2e case renders a drained probe yet — gate-2's redirect scan refuses the sink spelling | B |
| aid-coverage-instrument | analyzer-coverage dashboard over a corpus (instrument, never a gate) | text+orcl | ci▶ | d | `dorc-coverage` | B |

### Error/report classes

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-error-catalog-explainers | per-code colocated triple render: machine line, terse line, full prose registers (terse/deep/first-encounter) | — | all | — | defining-case-transcript-authored prose + fixpoint-protected committed catalog (`27V` §3; `282` generation flip) | S |
| aid-error-exit-code-family | semantic fast-fail exit codes (10+ range); `--exit-code` divergence-of-world contract for cron | invoc | ci▶ | d | `24H` ack-1 (B); `--exit-code` (S; never sink-landings — `279f`) | O |
| aid-apply-divergence-report | apply-time divergence from prediction: proceed-and-flag report items, never questions | apply | post◀ rev▶ | m | rul-divergence-proceed; ApplyOutcome receipt feeds | S |
| aid-refusal-breadcrumbs | an oracle's loud refusals surfaced with the site that ran anyway | facts | rev▶ olo▶ | c | the versioned report lane BUILT end-to-end (`27U`: recognition + noise-tolerant ingestion; the runtime drain on a controller-owned scratch directory, degrading to an inert sink when it cannot be created — `spike/CLAUDE.md` decline-class-emission) | B |


Unowned (rows above whose mechanism no round owns; watch, don't lose)
---------------------------------------------------------------------

- `aid-lint-kind-adjudicability-bar` (`24S:A4`) — hard-gates community-shared kinds.
- Receipt-backed why output sanitization (`an-output-sanitization`) and encoded
  `RecordedWhyFacts` consumption — the following why-surface conductor.
- The prose-register schema (terse/deep/first-encounter) + a catalog home for the
  class-level remediation-hint prose — human/conductor design sitting (`27U` §7).


### Invocation-time classes (the `dorc: {msg}` family + `dorc-sh`; ▶ push, pre-analysis)

Every INVOCATION error is an ordinary registry code with a defining case (`288` §6, landed): the
`dorc: ` / `dorc-sh: ` / `dorc: lint: ` prefixes and the usage synopsis are print-seat CHROME, not
catalog prose, so 21 codes do not each carry a copy of the usage text. The cut follows
`law-codes-vary-by-world-not-grammar`: a value-taking flag given no value is ONE code with a
`{flag}` hole however many flags share the shape, while `humane_read_error`'s `io::ErrorKind`
branches are THREE siblings (missing / unreadable / unclassed — three worlds, three repairs).
Severity is uniformly Error with no floor; exit codes are unchanged and never read severity.

Their cases are HONEST-triggered where the world IS the argv (`289:rul-worldless-route-honest-
trigger`): `dorc-loom` runs the real parser over the case's own replay command through an INTERNAL
`dorc-cli` lib target and refuses if the declared slug does not fire. Source-, record-, and
analysis-world cases run the shared production engine over materialized books, oracles, and record
streams. Nonportable I/O outcomes (the read-error triple, shim publication, lint tool failures,
transport faults, and `dorc-sh`'s file/exec failures) come from a closed typed loom-only edge-state
controller, never an arbitrary diagnostic payload. Three internal invariant failures alone use the
human-authorized `$ dorc-loom --this defect` route; its exact scenario set is fenced in
`dorc-loom/src/defect.rs`, and production code cannot name that authority.

Surfaces (selection policies over the registry)
-----------------------------------------------

- **`dorc lint`** — rows with demand ⊆ {text, orcl, topo}, push-selected, hot-loop-merciful
  (`--fail-on` leans error-only); never probes, never licenses; probe-inclusive lint is
  the plan pipeline's advisory surface wearing a lint hat, never a second probe path
  (`27R` §8b).
- **plan render** — the advisory plane: full aid; push rows at their finality cut; every
  pull row reachable from it by address. Spike-era policy: tune high (kWARN weld).
- **apply console** — error floor + decision digest only; receipt-free
  (`22F:advisory-vs-error-cut`). Never "cleaned up" to zero-stderr (silent-ship hole).
- **`dorc why`** — pull; wide-open derivability; problems-report default; the chain
  answer is the flagship product. At its most verbose it prints everything the report
  lane received, noise included — sanitized, attributed, never silently dropped
  (`27W:rul-report-noise-tolerant`).
- **receipt-backed why** — a rooted receipt graph rendered from sealed `RecordedWhyFacts`,
  facing the user at their most annoyed. Never a cache; final arrangement remains pending.
- **TUI (future)** — sugar over the same rows (`26B:rul-one-attention-moment`); the
  greyed-row curiosity query is pull embedded in push chrome.
- **LSP (future)** — another selection policy over the same rows: publishDiagnostics =
  the push side under a precise-or-silent-leaning default (the kWARN weld's
  late-cheap-knobs route); hover/code-lens = pull, wide-open. Finality-cut gating
  applies (no retracting squiggles); span precision is the felt-quality dependency.
  Probing tier: an editor EXTENSION drives consent over these same lanes — never a
  second probe path (`27R` §8b's rung-probe fence), one design class with scheduled/
  cron probing; tabled with the half-typed-argv hazard (`27U` §6).
- **MCP (future)** — the CI-mode posture is the contract (versioned additive
  envelopes; gate on codes/severity, never finding-set identity); lint/why/explain
  tools put the engine in an authoring agent's loop; a container-target probe tool
  sidesteps the standing-consent hazard. Punted, recorded (`27U` §6).
- **CI mode** — machine render only; versioned additive-only envelopes; gates on
  divergence-of-world and severity thresholds, never on finding-set stability
  (plan-as-API is the named failure-mode).
