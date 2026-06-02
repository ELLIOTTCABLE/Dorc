# Blind multi-model corpus variant — adversarial critique (don't run it for numbers)

> **Status (2026-06-01): methodology note, not a measurement.** Evaluates the
> *thesis-blind, three-model* corpus variant scaffolded out-of-tree at
> `~/shell-iac-corpus-study` — distinct from the in-tree `tools/corpus` instrument and
> the spike it drove (`notes/080-corpus-spike-progress-and-first-tally.md`, hereafter
> `[80]`). Verdict: **do not run it as a source of go/no-go numbers — it regresses on
> the standard `[80]` already sets.** The worry that produced it is real; the fix is
> `plans/086-corpus-classification-validation.md` (`[recovery]`), not blinding. Confidence
> markers throughout. External sources graded inline; some are highlights-only
> (flagged) pending the separate source-discipline pass — treat their grades as
> provisional. (This note was written *after* an initial critique made while
> deliberately blind to the Dorc thesis; the blind-phase conclusions survived
> unblinding, but several were sharpened once `[80]` was in view — see §3.)

## TL;DR
- ~SUSPECT the variant exists to dodge a real problem: the `Q-BAND` apply-cost ×
  check-depth rules are taste-laden researcher-judgment (`[80]` §7, parked "for the
  user… taste + ops-experience"), and they decide the VALUE-band size, hence the
  go/no-go. Measuring with rules you chose risks *confirming the design by
  construction*. That worry is legitimate and worth solving.
- +SURE the chosen fix — thesis-blind autonomous agents, each acquiring its own corpus
  and inventing its own measures — does **not** remove that bias. It *relocates* it
  (into the prompt framing, each model's training prior, and the final thesis-aware
  comparison), where it is opaque and uncorrectable, and it *discards* the rigor `[80]`
  already has.
- +SURE held to your *own* `[80]`/charter standard the variant loses on every axis: a
  defined estimand, one fixed reproducible corpus, the test-exclusion + per-stratum
  Simpson guard, and the planned container-fixture ground-truth anchor — all replaced
  by cross-model *consensus*, which is not validation.
- +SURE "compare the three runs afterward" yields an *uninterpretable* signal both
  ways: divergence confounds corpus ≠ measure ≠ classifier (three stacked confounds);
  convergence reflects shared training priors, not correspondence to reality.
- ~SUSPECT you already hold a *better-targeted* anti-bias tool than blinding —
  *contrast-not-compound* (charter §3: sample the world, never your own scripts). The
  blind variant is a cruder substitute that reintroduces sampling bias you engineered out.

## 1. What the variant is, and the problem it targets
`run-all.sh` runs the *same* open-ended prompt across three lineages (Claude/Codex/
Gemini), each cwd-isolated, with global config/skills/memory stripped (`--bare`,
relocated `CODEX_HOME`, no `AGENTS.md`/`GEMINI.md`) to prevent "thesis/identity leaks".
Each agent is told to "empirically characterize real-world shell + IaC at scale",
"decide for yourself what is worth measuring", "discover natural categories", acquire
its own corpus via `acquire()`, and emit a manifest + tables; then the runs are
"compared afterward, in a thesis-aware context". No run has executed (the `runs/<model>/`
dirs hold only seed files), so this is a pre-mortem, not an autopsy.

The motivating worry is correctly identified. `[80]` §7 parks the apply-cost and
check-depth rules *for the user* precisely because "the thresholds are taste +
ops-experience" — they are the researcher-degrees-of-freedom that set the VALUE-band
size. The pitch behind the variant is to obtain "facts unbiased by the author" by
removing the author from both *what to acquire* and *how to classify*.

## 2. Blinding doesn't remove the bias — it moves it somewhere worse
Bias is conserved across a measurement pipeline; you choose *where the human enters*,
not whether. Removing yourself from acquisition + classification pushes it into three
*less* inspectable places:
1. **The prompt.** "Characterize at scale", "natural categories", "popular and obscure
   / maintained and abandoned / polished and half-finished" is itself a planted axis-set.
   Your hand moves from *classifying* to *framing* — a more leveraged spot, not a
   vacated one.
2. **Each model's training prior.** What an LLM treats as "representative shell/IaC", a
   "natural category", or "notable" is a readout of its training distribution. You trade
   a *known, arguable, correctable* bias (yours) for an *opaque, uncorrectable* one (the
   model's) — strictly worse for the stated goal of author-independence.
3. **The final "compare in a thesis-aware context".** You re-enter at the point of
   maximum interpretive freedom, with the thesis in hand, to reconcile three non-identical
   piles — now wearing the authority of "three frontier models agreed".

Net: the taste you feared contaminating the classification is replaced by the *models'*
taste contaminating it — and unlike yours, theirs is never written down, graded, or
ground-truthed. +SURE: blinding the analyst to the *question* (not merely the desired
conclusion) is not what "blind study" means in method — real blinding conceals
condition-assignment *on top of* a fixed protocol; removing operationalization is the
opposite of rigor.

## 3. Held to the `[80]` standard, the variant regresses on every axis
This is the load-bearing point, and it is what unblinding sharpened: the critique is not
"you don't understand sampling" — `[80]` proves you do. It is "the variant throws away
what you already built."

| axis | your `[80]` / charter method | the blind variant |
| --- | --- | --- |
| estimand | one, decision-relevant (VALUE-band size; anti-corr ratio — charter §A) | "decide for yourself" → none shared |
| corpus | one SHA-pinned, manifest-tracked, out-of-tree sample, re-acquirable | each agent acquires a *different* convenience corpus |
| selection-bias control | caught ~95% test-code contamination; scanner v2 excludes test paths, splits mutating/control, **stratifies** (caught a live Simpson's paradox: guarded = role 45% / homelab 7% / other 55%) | nothing forces any of this; ad hoc per agent, if at all |
| ground truth | calibration harness *planned* (container fixtures: run the op, observe the state delta) | none — model consensus stands in |
| method honesty | charter §3: "every takeaway travels with raw numbers + exact method" | three undeclared methods, none pre-specified |

## 4. "Compare the three runs" is uninterpretable in both directions
- **Divergence** can't be read: it confounds *sampling* variance (different corpora),
  *measurement* variance (different measures), and *classifier* variance (different
  category schemes) into one inseparable blob.
- **Convergence** can't be read either: models trained on heavily-overlapping internet
  corpora share priors, so agreement is expected *even when wrong*. The annotation
  literature documents model-contingent-but-overlapping bias, self-preference (models
  favour low-perplexity/familiar outputs they'd generate), and explicitly warns of
  "circular validation… inflated metrics that do not reflect real-world performance"
  when there is no human gold standard `[C-llm-annotation-bias]`.

The contrast that proves the point: a 2026 AJPS study *did* get a valid, reproducible
measurement from a Claude+GPT+Gemini ensemble — correlating .87–.92 with expert-survey
benchmarks — but only because it used a **fixed corpus (235 manifestos), six
pre-specified dimensions, and an external human ground truth**, replicated across a
second model ensemble `[B-benoit-ajps2026]`. Those are exactly the three things the
blind variant lacks and `[80]` (mostly) has. The tooling can measure; *this protocol
around it* can't.

## 5. The sampling bias the variant reintroduces
Autonomous "characterize at scale" acquisition is convenience sampling of reachable
public GitHub. The canonical software-mining study finds public GitHub is ~71.6%
personal repositories, 67% single-committer, median 6 commits/project (90% < 50), ~46%
inactive over six months, and only ~63% of a hand-classified sample even *for software
development* `[A-kalliamvakou-msr2014]` (2014 data — directional, not current, but the
*kind* of skew is robust). You already route around exactly this: academic-labelled-
corpora-first, a SHA-pinned fixed sample, and *contrast-not-compound*. The variant's
"sample across the spectrum" instruction cannot fix a population most of which is private
and unreachable, and it undoes your own guardrail.

## 6. The kernel of truth — don't lose it
The classification rules *are* subjective researcher-DoF, and choosing rules that make
Dorc look good *would* be confirming-by-construction. Blinding is the wrong fix for the
right problem: it can't be checked, can't be ground-truthed, and isn't even the form of
blinding that controls bias. The right fixes — pre-register the rules, sensitivity-test
them, ground-truth them against the calibration harness, and (if you want the
independent-perspective benefit) use the models as *raters of one fixed corpus* so you
*measure* the subjectivity instead of hiding it — are in `[recovery]`.

## Sources
- `[A-dorc-corpus-tally]` +SURE — `notes/080-corpus-spike-progress-and-first-tally.md`:
  the measured first tally + v2 de-biasing. Graded A as *primary empirical* (your own
  instrument, reproducible from `tools/corpus` + `resolved.lock`); the surrounding
  planning prose is AI-generated per `Research/README.md`, so authoritative-for-intent,
  not verified.
- `[A-kalliamvakou-msr2014]` +SURE — Kalliamvakou et al., "The Promises and Perils of
  Mining GitHub", MSR 2014. Peer-reviewed; **full-read** this session via
  `chisel.cs.uvic.ca/pubs/kalliamvakou-MSR2014.pdf`. A.
- `[B-benoit-ajps2026]` ~SUSPECT — "Using LLMs to analyze political texts…", Am. J.
  Political Science 2026, `onlinelibrary.wiley.com/doi/10.1111/ajps.70050`.
  Peer-reviewed but **highlights-only (not full-read)** → graded B pending a full read;
  the ensemble-validity claim rests on it, so verify before load-bearing reuse.
- `[C-llm-annotation-bias]` -GUESS — cluster, **highlights-only**, graded C (mixed
  venue, provisional): model-contingent polarity bias (MDPI *Computers* 15(5):262);
  self-preference bias (arXiv 2508.06709); wisdom-of-LLM-crowds (arXiv 2307.12973).
  Supports "consensus ≠ ground truth; circular validation without a gold set".
- *Ungraded leads (asserted from model memory earlier this session; NOT used as
  evidence here, flagged for the source-discipline pass):* prediction-powered inference
  (Angelopoulos et al., Science 2023); Hubbard, *How to Measure Anything*; Evidence-Based
  Software Engineering (Kitchenham). Relevant to `[recovery]`; collect + grade before relying.
