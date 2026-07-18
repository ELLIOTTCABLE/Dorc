User-aid needs (AID-NEEDS)
==========================

The living registry of *user aid* — the one product-category covering errors, warnings,
hints, lints, `dorc why`, provenance display, and the whylog. Companion to
`ANALYZER-NEEDS.md` (which tracks what the engine must *compute*; this tracks what the
engine must *tell people*, and the mechanisms required to tell it correctly). LLM-generated,
human-directed (minted 2026-07-18, Fable design session with the human; the Law section's
rulings are human-typed or human-acked in that session unless cited elsewhere).

Why this registry exists: aid is deferred-by-nature ("lints and hints don't drive
core-engine-design-constraints") and therefore drops on the floor. Worse, the rationale
"it's fine, we can warn the user" must never wall off the question "should the core
mechanism *prevent* this instead?" — every row here is implicitly that question, kept
visible. Many rows look small from the user's side and need deep machinery (evidence
threading, provenance, replay) to produce *correctly*; the mechanism column is the point.

**Granularity law (human-ruled):** rows here are aid-*classes* (order ~10²), not error
codes. Codes (order ~10³ at production grade, close siblings, one full prose writeup each,
no generic fallback) live in the catalog and its defining cases — never here. Tests and
code cite rows as `AID-NEEDS:slug`. If this file starts enumerating messages, it has
failed; split the class instead.

**Columns** — `aid` (slug) · `what the user learns` · `demand` (evidence consumed:
`text` book/oracle text · `orcl` loaded oracle-set · `topo` CFG/wall topology · `facts`
probe facts (± = partial/streaming) · `apply` apply outcomes · `invoc` invocation record)
· `moments` (see axes; `▶`=push `◀`=pull) · `grade` (m measured · d derived · c
authored-claim · g conjecture) · `mech` (engine mechanisms, `an-*`/doc refs) · `st`
(B built · S specified · D designed-deferred · O open · W welded · X retired).


How to read the axes
--------------------

An aid-class is typed by three independent axes (human-acked schema, this session; the
flat input-rung ladder of `27R` §3 survives only as named corners of axis A):

- **A · demand** — the evidence product it consumes, plus a *stability* bit: in the
  reactive era (`26B`) facts arrive over time, and a push-fired aid that would retract
  under more arrivals churns trust; the plan-mint is the natural finality cut
  (`aid-plan-finality-discipline` below).
- **B · moment** — the user-situation it serves, and whether it is *pushed* (fires
  unprompted; relevance/fatigue dominates; selection must be ruthless) or *pulled* (the
  user asked; relevance is nearly free; derivability should run wide-open and the answer
  should be maximal). Moments: `olo` oracle-author hot-loop · `blo` book-author tuning
  loop · `rev` plan review ("this looks wrong") · `tui` mid-build curiosity query ·
  `post` post-apply annoyed · `ci` machine consumer.
- **C · grade** — the epistemic tier of the statement itself (measured / derived /
  authored-claim / conjecture). Grade is *rendered as syntax, not prose* (law below).

Surfaces (lint, plan render, apply console, `dorc why`, whylog, future TUI, CI mode) are
**selection policies over rows**, defined at the end — a row is never "owned" by a
surface. `kWARN`'s spike-era tune-high is a push-policy setting, not a row property; the
post-spike destination for push surfaces is precise-or-silent (`plans/111`), reached by
tuning selection, never by deleting detection.


Law — the bindings on every row (slugs; cite as `AID-NEEDS:law-…`)
------------------------------------------------------------------

- **law-two-planes-opposite-fail** (`26C` §5b, human hard-ack) — the license plane fails
  toward *unsureness*; the aid plane fails toward *narration with attributed
  confidence*. Aid may consume anything, including analyses vetoed for licensing
  (tool-aware text reasoning, host observation, oracle-contributed narration — the `26C`
  feeder classes), each attributed by class, forever display-tier.
- **law-collapse-mints-evidence** (human-acked, this session) — every point where the
  engine narrows for safety (meet-to-⊤, refuse, decline, wall, demote, cancel) mints a
  decision-inert evidence record carrying the collapse's *operands*, at the moment of
  collapse. Enforced at the *value* level (collapse constructors demand evidence — pure
  data, no arena in kernels; arena registration stays a post-pass backfill per `22D`);
  evidence is Eq-excluded (fixpoint termination, `22W` §2) and k-capped.
  `Evidence::Unexplained(site)` is constructible but renders as literally "unexplained" —
  self-advertising, never a silent allow-list.
- **law-evidence-is-decision-inert** (ru-11 + the `27L` sealed-room pattern, inverted) —
  no function from aid-evidence to any license-plane input exists, at the type level.
  License-plane values flow *into* evidence freely; never back. Lint-clean licenses
  nothing (`27R:dir-no-license-plane-contact`); silence-licenses-nothing runs in both
  directions.
- **law-trust-tier-is-syntax** (this session) — the epistemic tier of every rendered
  link (measured / vouched / ran / claimed / derived / consented) is a typed field on the
  evidence node, rendered uniformly by arrangement code. Prose fragments physically
  cannot overstate a claim into a measurement; mis-attribution (the pope-sin,
  `271:rul-sin-ordering`) is made structurally hard, not stylistically discouraged.
- **law-colocated-triple-render** (human-typed, this session) — a code's mechanical
  output, terse human line, and full prose registers are *colocated* in its defining
  case, making it difficult to slip mechanically-produced axes between them.
- **law-codes-vary-by-world-not-grammar** (human-typed, this session) — composing
  fragments into perfect English paragraphs is an explicit non-goal; output must be
  readable and flow, not case/plurality-matched. Sibling codes are yielded by actual
  variants in world-state or license structure, never by M×N combinations of what a
  fragment happens to compose with. Arrangement (ordering, indentation, list-joining,
  numbering) is code; templates are named-params-only (the Fluent fence, `22W` §1);
  interpolated values use engine-owned canonical formatters.
- **law-one-defining-case-per-code** (human-acked) — every code has exactly one defining
  case (trigger + world + params + the colocated triple render + the authoritative prose
  blocks); other cases assert structure and instance-of-template only. The prose flows
  through a *committed catalog intermediate* regenerated only by an explicit promote step
  (never auto-tracked by the build — the lag IS the assertion; Jest-snapshot/insta/BLESS
  pattern); build parses the catalog, no authored macros. Completeness gate: every code
  has a defining case; a defining case that stops triggering fails loud (the Menhir
  `.messages` property).
- **law-per-code-full-prose-no-fallback** (human-typed) — every code gets its own full
  writeup; no class-generic fallback exists (a fallback that renders plausibly is
  boilerplate-that-never-bites inverted: missing prose would produce zero signal).
  Friction is absorbed by tooling (scaffold generates the defining case + empty block),
  never by a semantic escape.
- **law-error-authorship-tier** (human-typed, this session; destined for
  `spike/CLAUDE.md`-tier law) — builders mint codes and defining-case structure and leave
  *explicitly-empty* prose blocks (rendering greppably as unwritten); a high-reasoning
  conductor or the human issues the prose from the builder's when/why/how report. Errors
  are too important to be left to lesser models.
- **law-pull-runs-wide-open** (this session) — on pull surfaces (`dorc why`, whylog) the
  user asked: derivability governs, answer maximally. On push surfaces, selection is
  ruthless and root-cause-only (AGENTS fail-fast; `(cause, site)`-keyed dedup;
  stay-in-pure-propagation — `22E`; the refutation-rerun is the deferred mechanism-2).
- **law-aid-adds-no-consent-moments** (`26B:rul-one-attention-moment`) — aid never adds
  interaction moments; everything front-loads into the one presented plan or waits to be
  pulled.
- **law-render-overlay-never-artifact** (rec-1, welded) — aid lives on render surfaces,
  overlaid; the byte-floored `.sh` artifact never carries it. The apply console keeps the
  error floor + decision digest only (`22F:advisory-vs-error-cut` (née 22F-fd6)).
- **law-lineno-identity** (`24H` ack-2ii) — one line-number space, the source file's,
  everywhere, round-trippable into `dorc why` addresses.
- **law-plain-language-surfaces** (`24H` ack-4) — no jargon on user-facing surfaces
  (no "⊤", no corpus vocabulary); quality, unambiguous English; semantically-correct
  characters fine, decoration not.
- **law-whylog-is-sensitive** (human-flagged, this session; NOTE only, parked) — the
  whylog carries host metadata that is sensitive for some hosts even when secret-free.
  Fences when the secrets round arrives: `an-diag-secret-taint`,
  `an-output-sanitization` (hostile bytes → TTY), `26B:need-scrub-before-freeze`,
  `24R:repurp-finding12`. The whylog is also never a verdict cache (rec-5 untouched;
  write-only, replay-driven, no cross-run re-ingest).


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
| aid-escalation-consent-legibility | the escalation dial × capability × entry-capable wrappers in effect | invoc+orcl | rev▶ | m | `27C`; legacy lane → catalog (27T) | B |
| aid-caret-span-precision | which exact section of a compound a diagnostic means | text | all▶ | d | `24H` ack-8; an-output-sanitization owed | O |
| aid-firehose-suppression | one honest aggregate instead of per-site unprobeable noise | text+topo | rev▶ | d | `24H` firehose fix | B |
| aid-plan-finality-discipline | push-hints fire only at stable/minted state (no retracting hints mid-build) | facts± | tui▶ rev▶ | — | `26B` finality; r26 seam | D |
| aid-sigpipe-flap-note | rc-141 sink-landings flagged "likely benign early-exit race" | facts | rev▶ | d | `279f` named class | B |

### Ask-surfaces (◀ pull; `dorc why` and the whylog)

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-why-problems-report | zero-arg: the problematic subset of the current analysis (refusals, walls, guards, can't-tells) | text..facts | rev◀ blo◀ | m+d | `24H` ack-2i | B |
| aid-why-line-address | per-line interrogation by `book.sh:N` or content match | same | rev◀ tui◀ post◀ | m+d | law-lineno-identity | B |
| aid-why-license-chain | the full numbered chain behind any disposition, tier-worded per link, naked-trust links identified by construction, with re-measure + leverage-point epilogue | facts+topo+invoc | post◀ rev◀ | m+c+d | evidence plane + witnesses + minting-line threading (`27T`) | S |
| aid-why-wall-narration | why a wall formed: which participant, which channel/coverage failure, what it costs downstream | text+orcl+topo | blo◀ olo◀ rev◀ | d | collapse evidence (walls) | S |
| aid-why-decline-narration | which oracle arm declined this shape, and what would have answered | orcl+facts | olo◀ blo◀ | c+d | collapse evidence (declines); refusal breadcrumb lane (spelling settling) | S |
| aid-why-disagreement-narration | two establishers disagreed on one cell: who, where, which values | facts | rev◀ post◀ | m | collapse evidence (merge operands; `22H` §1) | S |
| aid-why-value-chain-narration | a captured value narrated through iteration hops and host transformations, best-effort, feeder-attributed | facts±+history | post◀ | m+c+g | `26C:need-why-explanation-lane` + feeders; r26 | D |
| aid-whylog-posthoc-why | all of the above, after everything is apparently complete, zero setup: `dorc why --last` | whylog | post◀ | m+c+d | thin durable + replay (`27T`); `22A:concl-10` | S |

### Authoring-time classes (`dorc lint`; ▶ push, hot-loop-safe, never probes)

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-lint-analysis-diagnostics | the no-world pipeline's diagnostics over given files | text+orcl | olo▶ blo▶ ci▶ | d | `27R` source-1 | B |
| aid-lint-unmodeled-inventory | per book: unmodeled families, first-wall position, degradation counts | text+orcl+topo | blo▶ ci▶ | d | `27R` source-2 | B |
| aid-lint-verdict-body-mechanicals | status-flattening hazards in verdict bodies (pipeline tails; the rest caught upstream as dialect) | text | olo▶ ci▶ | d | `27R` source-3; `279f` constant-rc candidate seamed | B |
| aid-lint-external-tool-relay | shellcheck/checkbashisms findings, strip-line-remapped, tolerant adapters, coverage-asserted envelope | text | olo▶ ci▶ | d | `27R` §4/§8b | B |
| aid-lint-reserved-name-checks | munge charclass/collision; book squats on `__role` namespace | text | olo▶ blo▶ | d | `oracle/reserved.rs`; → catalog (27T) | B |
| aid-lint-strip-floor-check | strip output parses/runs identically under both pinned floor shells | text | olo▶ ci▶ | d | two-binary floor; fence-rejection-rc; `27R` seam | S |
| aid-lint-wrapper-oracle-bar | wrapper-family quality: peel cross-check, argparse lints, self-vouch/footprint | text+orcl | olo▶ ci▶ | d | `24S:A6` | S |
| aid-lint-carrier-payload-bar | carrier/payload quality: which-arg-is-code gates, reconstruction differential, dorcism-in-payload | text+orcl | olo▶ ci▶ | d | `24T:P-A4` | S |
| aid-lint-kind-adjudicability-bar | kind-topology clauses machine-readable; binding smells; differential discharge — REQUIRED before kinds go community-shared | text+orcl | olo▶ ci▶ | d | `24S:A4`; unowned | S |
| aid-lint-oracle-solo-mode | oracle files linted with no book present | orcl | olo▶ | d | `27S:seam-oracle-validate-factoring` | O |
| aid-coverage-instrument | analyzer-coverage dashboard over a corpus (instrument, never a gate) | text+orcl | ci▶ | d | `dorc-coverage` | B |

### Error/report classes

| aid | what the user learns | demand | moments | grade | mech | st |
|---|---|---|---|---|---|---|
| aid-error-catalog-explainers | per-code colocated triple render: machine line, terse line, full prose registers (terse/deep/first-encounter) | — | all | — | defining cases + committed catalog + promote (`27T`) | S |
| aid-error-exit-code-family | semantic fast-fail exit codes (10+ range); `--exit-code` divergence-of-world contract for cron | invoc | ci▶ | d | `24H` ack-1 (B); `--exit-code` (S, never sink-landings — `279f`) | O |
| aid-apply-divergence-report | apply-time divergence from prediction: proceed-and-flag report items, never questions | apply | post◀ rev▶ | m | rul-divergence-proceed; whylog feeds | S |
| aid-refusal-breadcrumbs | an oracle's loud UNK refusals surfaced with the site that ran anyway | facts | rev▶ olo▶ | c | report stream, spelling settling (oracle-contract §6) | O |


Gap ledger (built-vs-designed; owners)
--------------------------------------

Numbered per the 2026-07-18 inventory session; `27T` = the user-aid build-phase note.

1. **gap-no-durable-why** — no whylog, no `--last`, no durable reader; USER_STORY's
   headless story is fiction at HEAD. → `27T` (human-ruled build-now).
2. **gap-claim-vs-receipt-unminted** — `OriginKind::{OracleClaim, ProbeResult}` reserved
   since r22, never minted; the arena cannot ground claim-vs-measurement. → `27T`.
3. **gap-two-diag-systems** — the legacy string-slug `Diagnostic` coexists with the
   battlefield-bound catalog; the newest lanes (escalation, wrapped, munge) went legacy.
   → `27T` (human-ruled: rip out the legacy, one mechanism, kill don't deprecate).
4. **gap-suggestion-unwired** — `Suggestion`/`Applicability` no production emits;
   `RemediationClass` not a registry column; floors never human-ratified;
   `Floor::Pinned` unused. → `27T` (rides the catalog re-cut).
5. **gap-ack6-sibling-hint-absent** — ruled hint, no emitter. → `27T` rider.
6. **gap-hints-unpinned** — zero `hint:` e2e expectations; the nag-loop is the product
   and has no coverage. → `27T` rider (kWARN keepalive applies to pins too).
7. **gap-minting-line-threading** — claims/vouches carry no source line; blocks
   stdlib-era attribution (`27Q` §2 precondition, previously unowned). → `27T`.
8. **gap-why-surface-sanitization** — `an-output-sanitization` unbuilt while why/hint
   lanes print host-derived text. → security round; fence noted in law-whylog-is-sensitive.
9. **gap-smalls** — `--risk-faultless-skips` ruled name unparsed (code: `--trust-footprints`);
   `--exit-code` unbuilt; a diag.rs header claims 20 codes over an enum of 15. → `27T` smalls / root-doc queue.


Surfaces (selection policies over the registry)
-----------------------------------------------

- **`dorc lint`** — rows with demand ⊆ {text, orcl, topo}, push-selected, hot-loop-merciful
  (`--fail-on` leans error-only); never probes, never licenses; rung-probe is the plan
  pipeline's advisory surface wearing a lint hat, never a second probe path (`27R` §8b).
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
