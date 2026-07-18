User-aid needs (AID-NEEDS)
==========================

The living registry of *user aid* — the one product-category covering errors, warnings,
hints, lints, `dorc why`, provenance display, and the whylog. Companion to
`ANALYZER-NEEDS.md`: that file tracks what the engine must *compute*; this one tracks
what the engine must *tell people*, and the mechanisms required to tell it correctly.
LLM-generated; add a row when a feature implies a new aid-class; keep rows current.

Why this registry exists: aid is deferred-by-nature ("lints and hints don't drive
core-engine-design-constraints") and therefore drops on the floor. Worse, the rationale
"it's fine, we can warn the user" must never wall off the question "should the core
mechanism *prevent* this instead?" — every row here is implicitly that question, kept
visible. Many rows look small from the user's side and need deep machinery (evidence
threading, provenance, replay) to produce *correctly*; the mechanism column is the point.

Engine-side build law for this category (evidence plane, whylog, the error catalog and
its authoring pipeline) lives in `Research/notes/27V` and `spike/CLAUDE.md`'s user-aid
block — not here. This file is the row-registry plus only the law needed to mint and
consume rows.

**Columns** — `aid` (slug) · `what the user learns` · `demand` (evidence consumed:
`text` book/oracle text · `orcl` loaded oracle-set · `topo` CFG/wall topology · `facts`
probe facts (± = partial/streaming) · `apply` apply outcomes · `invoc` invocation record
· `whylog` the posthoc durable) · `moments` (see axes; `▶`=push `◀`=pull) · `grade`
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

Surfaces (lint, plan render, apply console, `dorc why`, whylog, future TUI, CI mode) are
**selection policies over rows**, defined at the end — a row is never "owned" by a
surface. `kWARN`'s spike-era tune-high is a push-policy setting, not a row property; the
post-spike destination for push surfaces is precise-or-silent (`plans/111`), reached by
tuning selection, never by deleting detection.


Law — what a row-minter and row-consumer must know (cite as `AID-NEEDS:law-…`)
------------------------------------------------------------------------------

- **law-rows-are-classes** — rows are aid-*classes* (order ~10²), never error codes.
  Codes (order ~10³ at production grade; close siblings from world-state variants; one
  full prose writeup each, no generic fallback) live in the catalog and its defining
  cases — never here. Tests and code cite rows as `AID-NEEDS:slug`. If this file starts
  enumerating messages, it has failed; split the class instead.
- **law-two-planes-opposite-fail** (`26C` §5b) — the license plane fails toward
  *unsureness*; the aid plane fails toward *narration with attributed confidence*. Aid
  may consume anything, including analyses vetoed for licensing (tool-aware text
  reasoning, host observation, oracle-contributed narration — the `26C` feeder classes),
  each attributed by class and forever display-tier. Aid-evidence is decision-inert at
  the type level: no path from any row's machinery into any license-plane input.
  Lint-clean licenses nothing; silence-licenses-nothing runs in both directions.
- **law-collapse-mints-evidence** — every point where the engine narrows for safety
  (meet-to-⊤, refuse, decline, wall, demote, cancel) mints a decision-inert evidence
  record carrying the collapse's *operands*, at the moment of collapse. A new
  safety-narrowing without its evidence mint (and, usually, its row here) is a defect.
  Mechanics: `27V` §1.
- **law-trust-tier-is-syntax** — the epistemic tier of every rendered link (strawman
  spellings: measured / vouched / ran / claimed / derived / consented) is a typed field
  rendered uniformly by arrangement code; prose never hand-writes epistemics, so a claim
  cannot be dressed as a measurement. The tier set and its typed rendering are the law;
  all render particulars stay unwelded pending implementation (`KNOBS:kFLOW`;
  `27V:rul-output-form-unwelded`). Mis-attribution is the worst aid failure
  (`271:rul-sin-ordering`); where certainty runs out, say so rather than rounding up.
- **law-pull-runs-wide-open** — on pull surfaces the user asked: derivability governs,
  answer maximally. On push surfaces selection is ruthless and root-cause-only
  (AGENTS fail-fast; `(cause, site)`-keyed dedup; stay-in-pure-propagation — `22E`).
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
- **law-whylog-is-sensitive** — whylog contents are host-metadata-sensitive even when
  secret-free; the secrets round owns the work. Fences: `an-diag-secret-taint`,
  `an-output-sanitization`, `26B:need-scrub-before-freeze`, `24R:repurp-finding12`. The
  whylog is never a verdict cache (rec-5: write-only, replay-driven, no re-ingest).
- **law-report-surfaces-speak-sh** (`27W:rul-report-surface-massaging`) — report/why
  surfaces show *code* by preference (an oracle's own arm, comments riding along as
  display, never parsed), and may massage what they show (slice to contributing lines,
  attach whole adjacent comments, mark elisions): attribution means implying authorship
  and directing repair, never byte-for-byte reproduction. The byte-floor laws bind the
  plan/artifact planes only; display-sh must never masquerade as a runnable artifact.


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
| aid-loaded-oracle-inventory | which oracles/dirs were actually loaded this run | invoc | rev▶ ci▶ | m | `24H` flow picks | O |
| aid-survives-attribution | whose at-most claim licensed each survival; the disjointness derivation; the resolver involved | facts+topo | rev▶ post◀ | m+c+d | SurvivalWitness; an-attribution-lanes | B |
| aid-guard-license-attribution | whose check guards each verify line, under whose vouch | facts+topo | rev▶ post◀ | c+d | GuardLicense lane | B |
| aid-carry-attribution | which cross-context carries happened, under which invariant lines + closure proof | facts+topo | rev▶ post◀ | c+d | `27C` §4(a); an-read-set-closure | B |
| aid-run-cause-disclosure | why a line can never elide (unknowable operand etc.), with remediation | text+topo | rev▶ blo▶ | d | why-lens; `(cause,site)` dedup | B |
| aid-escalation-consent-legibility | the escalation dial × capability × entry-capable wrappers in effect | invoc+orcl | rev▶ | m | `27C`; catalog re-home rides `27V` | B |
| aid-caret-span-precision | which exact section of a compound a diagnostic means | text | all▶ | d | `24H` ack-8; an-output-sanitization owed | O |
| aid-firehose-suppression | one honest aggregate instead of per-site unprobeable noise | text+topo | rev▶ | d | `24H` firehose fix | B |
| aid-plan-finality-discipline | push-hints fire only at stable/minted state (no retracting hints mid-build) | facts± | tui▶ rev▶ | — | `26B` finality; r26 seam | D |
| aid-sigpipe-flap-note | rc-141 sink-landings flagged "likely benign early-exit race" | facts | rev▶ | d | `279f` named class | B |

### Ask-surfaces (◀ pull; `dorc why` and the whylog)

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-why-problems-report | zero-arg: the problematic subset of the current analysis (refusals, walls, guards, can't-tells) | text..facts | rev◀ blo◀ | m+d | `24H` ack-2i | B |
| aid-why-line-address | per-line interrogation by `book.sh:N` or content match | same | rev◀ tui◀ post◀ | m+d | law-lineno-identity | B |
| aid-why-license-chain | the full numbered chain behind any disposition, tier-worded per link, naked-trust links identified by construction, with re-measure + leverage-point epilogue | facts+topo+invoc | post◀ rev◀ | m+c+d | evidence plane + witnesses + minting-line threading (`27V`) | S |
| aid-why-wall-narration | why a wall formed: which participant, which channel/coverage failure, what it costs downstream | text+orcl+topo | blo◀ olo◀ rev◀ | d | collapse evidence (walls) | S |
| aid-why-decline-narration | which oracle arm declined this shape and why — the arm itself inlined (show-the-code, massaged per law) | orcl+facts | olo◀ blo◀ post◀ | c+d | collapse evidence (declines) + `27W` classes + arm-inlining | S |
| aid-why-disagreement-narration | two establishers disagreed on one cell: who, where, which values | facts | rev◀ post◀ | m | collapse evidence (merge operands; `22H` §1) | S |
| aid-why-value-chain-narration | a captured value narrated through iteration hops and host transformations, best-effort, feeder-attributed | facts±+history | post◀ | m+c+g | `26C:need-why-explanation-lane` + feeders; r26 | D |
| aid-whylog-posthoc-why | all of the above, after everything is apparently complete, zero setup: `dorc why --last` | whylog | post◀ | m+c+d | thin durable + replay (`27V`); `22A:concl-10` | S |

### Authoring-time classes (`dorc lint`; ▶ push, hot-loop-safe, never probes)

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
| aid-lint-oracle-solo-mode | oracle files linted with no book present | orcl | olo▶ | d | `27S:seam-oracle-validate-factoring` | O |
| aid-authored-decline-classes | which shapes an oracle deliberately declines, and why (closed class set: unsound/unmodeled/interactive/hazard); routes the enhancement-nags honestly | text+orcl (facts± at tier-3) | olo▶ blo▶ rev▶ post◀ ci▶ | c | `27W` (versioned report-lane emission; static-first three tiers; decline evidence field) | S |
| aid-coverage-instrument | analyzer-coverage dashboard over a corpus (instrument, never a gate) | text+orcl | ci▶ | d | `dorc-coverage` | B |

### Error/report classes

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-error-catalog-explainers | per-code colocated triple render: machine line, terse line, full prose registers (terse/deep/first-encounter) | — | all | — | defining cases + committed catalog (`27V` §3) | S |
| aid-error-exit-code-family | semantic fast-fail exit codes (10+ range); `--exit-code` divergence-of-world contract for cron | invoc | ci▶ | d | `24H` ack-1 (B); `--exit-code` (S; never sink-landings — `279f`) | O |
| aid-apply-divergence-report | apply-time divergence from prediction: proceed-and-flag report items, never questions | apply | post◀ rev▶ | m | rul-divergence-proceed; whylog feeds | S |
| aid-refusal-breadcrumbs | an oracle's loud refusals surfaced with the site that ran anyway | facts | rev▶ olo▶ | c | the versioned report lane (`27W` §2: `DREP_V1`-strawman sink, `<verb> <class> <tail>` grammar, degrade-generic) | S |


Unowned (rows above whose mechanism no round owns; watch, don't lose)
---------------------------------------------------------------------

- `aid-lint-kind-adjudicability-bar` (`24S:A4`) — hard-gates community-shared kinds.
- `aid-refusal-breadcrumbs` — the report-stream spelling is still unsettled.
- Why-surface output sanitization (`an-output-sanitization`) + whylog sensitivity —
  security round.
- `aid-caret-span-precision` / `aid-loaded-oracle-inventory` — build-state unverified;
  confirm against the code before citing as built.


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
  answer is the flagship product.
- **whylog** — the same reader over the replayed thin durable; the *most*-informed mode,
  facing the user at their most annoyed. Never a cache; sensitivity-fenced.
- **TUI (future)** — sugar over the same rows (`26B:rul-one-attention-moment`); the
  greyed-row curiosity query is pull embedded in push chrome.
- **CI mode** — machine render only; versioned additive-only envelopes; gates on
  divergence-of-world and severity thresholds, never on finding-set stability
  (plan-as-API is the named failure-mode).
