# Corpus go/no-go — classification validation plan

> **Status (2026-06-01): recovery plan.** Operationalizes the judgment-call parked in
> `notes/80-corpus-spike-progress-and-first-tally.md` §7 (`[80]`), and supersedes the
> blind multi-model variant rejected in `notes/81-blind-multimodel-study-critique.md`
> (`[81]`). Goal: return the `kDEPS` engine-vs-oracle go/no-go *without confirming the
> design by construction* — using the instrument, sample, and calibration-harness plan
> you already have, not a blind pipeline. Confidence markers throughout.

## The problem in one paragraph
The go/no-go is the VALUE-band size — the fraction of *mutating* ops that are
expensive-apply ∧ shallow-check (charter §A) — plus the anti-correlation ratio
(shallow:deep among expensive ops). Both are decided by two classification rules:
**apply-cost** (cheap-idempotent vs expensive/dangerous/slow) and **check-depth**
(shallow vs deep). `[80]` §7 correctly flags these as "taste + ops-experience" — they
are researcher-degrees-of-freedom, and a rule-set chosen to favour Dorc would manufacture
a fat VALUE band by construction. We need a verdict that is either *robust to* that
subjectivity or *anchored against ground truth* despite it.

## The recovery — five steps, in order
1. **Pre-register the rules** (`kPROBING` / `kPRECISION`). Write the per-command/
   per-module-class apply-cost table and the shallow/deep predicate, and commit them
   *before* computing any band size. This is what "blind to the thesis" was groping for —
   achieved by committing the method in public, not by emptying the analyst's head. It
   keeps operationalization (so the result stays interpretable and decision-relevant) while
   removing the freedom to fish thresholds toward a fat VALUE band.
2. **Sensitivity analysis.** Compute the go/no-go under ≥3 rule-sets spanning
   conservative → liberal. If the *verdict* (engine-heavy / oracle-heavy / rework-thesis)
   is stable across them, the subjectivity is immaterial — report that and proceed. If it
   *flips*, the classification is load-bearing → step 3 is mandatory, not optional. (This
   directly answers "does my taste decide the answer?" with a number.)
3. **Ground-truth a stratified random subsample** with the calibration harness already
   planned (charter §3 / `corpus-spike-seed-prompt.md` §Method): container fixtures — run
   the op, observe the state delta. This anchors *both* axes empirically: did the
   "expensive" op actually do expensive work, and did the "shallow" guard actually capture
   the need or did state still change behind a passing guard (an *elision-soundness* probe,
   AGENTS §1). A few dozen ground-truthed ops are enough to bias-correct the static band
   estimate and put a real confidence interval on it. *(Lead, ungraded — verify in the
   source-discipline pass: "prediction-powered inference" combines many cheap static labels
   + a few gold labels into an estimate with a valid CI, so the static heuristic buys scale
   and the gold subsample buys correctness.)*
4. **(Optional) Multiple models as raters of ONE fixed corpus** — the legitimate salvage
   of the multi-model idea. If you want the independent-perspective benefit that motivated
   `[81]`'s variant: give each model the *same* fixed corpus and the *same* pre-registered
   rubric, as independent *classifiers* of the same ops, then compute inter-rater agreement
   (Cohen/Fleiss κ or Krippendorff α). High agreement → the rubric is objective enough to
   trust; low agreement → the classification is intrinsically subjective and consensus can't
   rescue it (step 3 is the only arbiter). This *measures* the subjectivity you feared
   instead of hiding it. Blind the raters to the *thesis* if you like — never to the corpus
   or the question.
5. **Adversarial classification** (severe testing). Have an independent clean-context agent
   construct the most Dorc-*hostile* defensible rule-set (maximize "deep", minimize the
   VALUE band). If the go/no-go survives the worst defensible classification, it is robust;
   if only a favourable rule-set yields a fat VALUE band, *that* is the finding. This
   operationalizes the good adversarial instinct behind the blind variant, aimed squarely at
   the decision rather than scattered across whole open-ended studies.

## Keep / discard
- **Keep:** the instrument (`tools/corpus`; the variant's `harness.mjs` is the same
  machinery), the SHA-pinned sample + `resolved.lock`/`manifest.toml`, contrast-not-compound
  (charter §3), and the calibration-harness plan.
- **Discard:** the blind / autonomous / open-ended / per-agent-corpus protocol. Nothing in
  it is load-bearing that pre-registration + one fixed corpus + a ground-truth anchor don't
  do better and more cheaply.

## Scale & cost
`[80]` §7 already establishes that a stratified subset gives ±1–2% on the proportions at a
few thousand ops, and the instrument streams (10k files in ~3.7 s). The only
human-expensive part is the ground-truth subsample (dozens of ops in container fixtures) +
adjudicating the adversarial rule-set — bounded to a session, and the part that actually
buys the rigor.

## Cross-refs
`[80]` (the spike + parked rules), `[81]` (why not the blind variant), charter §2 (the
`kDEPS` split this feeds) + §3 (method honesty, contrast-not-compound), `corpus-spike-
seed-prompt.md` §Method (the calibration harness).
