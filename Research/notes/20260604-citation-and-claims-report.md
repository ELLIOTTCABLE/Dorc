# Citation & claims audit — report (2026-06-04)

Datestamped synthesis (not a round). Human-facing summary of the **source-claim adversarial audit**
of the `Research/` corpus, plus the `papers/`→`sources/` grading-migration that followed it. The
batch-by-batch evidence (every deciding quote + locator) is the durable artifact at
`_scratch/source-audit-register.md`; this report is the trust-level read and the concerns worth
knowing before you close out.

## TL;DR
- **Audited:** every load-bearing citation in `plans/` (the audit target) — all tier-A + tier-B
  sources, ranked by blast-radius. Six batches.
- **Result: 0 EGREGIOUS, 3 MILD.** Zero fabricated citations — every bracket-key resolves to a real,
  graded source, and every author-name cue resolves to a real work. Where checked, the factual
  attributions hold.
- **Hard caveat:** this method *cannot certify correctness* — it catches egregious, seconds-to-verify
  mismatches and nothing subtler. "0 EGREGIOUS" means *nothing blatantly, quotably wrong was found in
  the checked set*, not *the corpus is right*.
- **Newly grounded:** 36 seed papers graded + migrated into `sources/`; 4 A→B regrades; 2 hedge-lifts;
  3 in-place ahistorical annotations for the MILD findings.
- **Needs your eyes:** 2 grades I assigned without an audit (`tip-survey`, `reps-cfl`); plus the
  deliberately-unaudited generic-token source set.

## Method (brief)
Per cited source: pin the plan's asserting sentences → decompose to atomic propositions → tag
**factual-attributable** (number / named mechanism / "paper X proved Y") vs **interpretive-analogical**
("maps to" / "analog" / "lens") → deterministic precheck (grep the attributed tokens in the source
text; eliminate only verbatim-present purely-lexical claims) → for surviving factual atoms, dispatch a
**neutral** and an **adversarial** reader in clean context (no Dorc framing) → arbitrate on the
**quotes**, not the rhetoric. Two disciplines rode every step:
- **caveat-COST** — annotating a committed claim is invasive; reserved for egregious, human-adjudicable faults.
- **caveat-NOTRUTH** — the process is as poisonable as the original author; **"I couldn't read it" must
  never become "it doesn't support the claim."** Pursue only egregious mismatches; log the rest.

## Coverage — what was and wasn't checked
**Audited (full power):** the CoLiS / shell-front-end cluster · the `055` analysis-architecture PLT
spine (reps-* / IFDS / SDG-slicing / effects / purity / TAJS) · the security & threat-model sources ·
the build-systems + incremental-analysis sources (rattle, mokhov, souffle, arzt-bodden, biabduction) ·
the state-tracking lineage (Traugott, Burgess, Engler, tobin-hochstadt, foster). Every tier-A and
tier-B source.

**Not audited (deliberate, low marginal value — none carries a known correctness risk):**
- The **generic-token bracket set** (~45 keys: `ansible`/`terraform`/`docker`/`puppet`/`chef`/…) —
  tool/project-name cites whose sourcing-discipline you'd already accepted; their fan/reach counts are
  inflated by bare-word matches, so they were re-ranked down.
- The **gap-access / OCR set** — works cited but not locally readable (Opdebeeck, GLITCH, Rahman,
  Heintze–McAllester [paywalled, every mirror dead], cousot [image-only scan]) — to be fetched and
  adjudicated only if a dependent claim was reached. **None escalated** through the six batches.

## Findings — 3 MILD (none EGREGIOUS; all annotated in-place)
All three sit in `plans/021` (the "is Coq justified?" section). Each now carries an additive
`<!-- /* superseded 2026-06-03: … Source-claim audit → MILD. */ -->` aside *beside* the claim;
historical text untouched.

- **mild-1 · `021:48` · [A-colis-installation-scenarios-tacas-2020]** — the appositive "~28k scripts /
  **hundreds of commands**": "hundreds of commands" is the plan's own quantifier; the paper says only
  "*most of the UNIX commands called by the maintainer scripts*" (unquantified). The ~28k figure is
  verbatim; the engine/oracle-split substance holds. Only the count is unsourced.
- **mild-2 · `021:16` · [A-verified-interpreter-shell-vstte-2017]** — "the proof effort bought
  soundness of **a symbolic over-approximation**" is **mischaracterized**. VSTTE-2017 proved its CoLiS
  interpreter *sound **and** complete* w.r.t. the CoLiS semantics — an equivalence (Theorem 1) — **not**
  an over-approximation. "Symbolic over-approximation" is the TACAS-2020 *engine's* property (and
  Abash's, cited there as others' work). The "Dorc rejected over-approximation" conclusion still stands;
  it just belongs to the engine, not this proof. **This is the sharpest of the three** — a genuine
  wrong-characterization, not merely an unsourced number. It did *not* hit EGREGIOUS only because "the
  proof effort" is referent-ambiguous (could denote the broader CoLiS symbolic-exec project), which
  needs intent-adjudication and so fails the "verifies in seconds" bar.
- **mild-3 · `021:15`** — the parenthetical "(Morbig paper; TACAS 2020.)" was a terse dual-citation
  gluing two distinct works; **now slug-fixed** to `[A-morbig-sle-2018]; [A-colis-installation-scenarios-tacas-2020]`.

**Everything else PASSed** — including the highest-risk EGREGIOUS-class number-claims, each
verbatim-grounded in-source and re-read in the skeptical direction: distefano **0%→70% fix-rate**,
arzt-bodden **~80% saved**, souffle **35 s**, TAJS **87%→<2% precision / 512 MB OOM**, the k-CFA
**EXPTIME-for-closures / polynomial-for-flat** direction, rattle's **CPU-pipeline hazard taxonomy**.

## Grading + migration (the housekeeping that followed)
- All **36** seed papers under `papers/` were graded and migrated into `sources/` (gitignored binaries +
  tracked `sources.json` metadata); `papers/` was removed.
- Grade provenance: **20** from this audit (these *supersede* any prior grade), **14** from an earlier
  *non-adversarial* "grade-to-merge" handoff pass, **2** assigned here (see conc-grades-2).
- **4 A→B regrades**, now propagated across `notes/`+`plans/`: `colis-specification` (self-labeled tech
  report, no venue) · `moller-schwartzbach` (lecture-notes textbook, secondary) · `dozer` (2-page
  ICSE-SEIP short paper) · `ansible-challenges` (arXiv preprint, no confirmed venue).
- **2 hedge-lifts** in `sources.json`: `foster` and `tobin-hochstadt` carried "abstract-read only"
  hedges; the owed deep reads were done and *confirmed* (did not diverge), so the hedges were lifted.

## Concerns / questionables — read before closing
- **conc-grades-2 (action: review).** `tip-survey` and `reps-cfl` were graded by *neither* the audit nor
  the handoff. To let `papers/` fully empty I assigned both **A / -0:SUSPECT** grade, relevance
  **-1:GUESS** — *provenance grades, not content-audited* (flagged in-entry via
  `graded-by: top-level-agent` + `via: "papers-migration (provenance grade…)"`). `reps-cfl` the audit had
  explicitly marked **out-of-scope** (no plan claim-site; the CFL-reachability cubic-floor at `076:15` is
  attributed to Heintze–McAllester, not this survey). ~SUSPECT — your call; override freely.
- **conc-power (structural).** Interpretive / analogical claims ("maps to", "analog", "lens" — e.g.
  Traugott's convergent/congruent trichotomy, the SPA textbook framing, Harnad symbol-grounding) were
  labelled **low-power** and **not audited symmetrically** — they're near-unfalsifiable, so the audit
  spent its power on factual-attributable atoms instead. The *framing* claims therefore carry materially
  less assurance than the *factual* ones. +SURE that's the right allocation; ~SUSPECT a few framing
  claims would wobble under harder scrutiny (several already self-hedge, e.g. `099:25`).
- **conc-mybias (structural).** The arbiter (me) is Dorc-aligned — a standing bias toward believing the
  plans. Mitigated by resting every verdict on verbatim quotes that a neutral *and* an adversarial reader
  independently surfaced, and by re-reading each number-claim skeptically. It is still a real residual:
  the missing check is a genuinely hostile external reviewer, which this audit did not have.
- **conc-notruth (structural).** Restating the cap: the method cannot find truth. A subagent (or I)
  becoming sure "this is badly wrong!" is as poisonable as the author's "this is so right!". The audit is
  a floor against blatant error, not a correctness proof.
- **conc-unread (coverage).** A few sources were graded on provenance without a full content read:
  `cousot` (image-only scan — theory cross-covered by the SPA textbook + the co-cited TAJS) and
  `heintze-tardieu` (body OCR-garbled — title + dblp provenance carry the low-power analogical cite). Per
  caveat-NOTRUTH their unreadability was **not** taken as non-support.
- **conc-unaudited (coverage).** The generic-token set (~45 keys) is ungraded-by-this-audit. If any
  becomes load-bearing later, audit it then.
- **conc-warnings (cosmetic).** `validate.sh` is **error-clean**, but reports ~101 "unbracketed"
  warnings — pre-existing bare-stem mentions in the `notes/000` round-lists and the audit register's
  tables (a handful are the now-canonical `sources/A-…` paths, which *look* like bare keys to the
  linter). Warning-level only; a separate bracket-canonicalization pass would clear them. I did not — they
  predate the migration and aren't load-bearing.

## Open items
- Review the 2 assigned grades (**conc-grades-2**) — the only substantive judgment I made that the audit
  didn't back.
- Optional, low-value: a generic-token audit; the bare-stem→bracket hygiene (**conc-warnings**).
- Full evidence — quotes, locators, per-batch reasoning, the deterministic precheck — lives in
  `_scratch/source-audit-register.md`.
