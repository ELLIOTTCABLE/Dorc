# 280d — Working notes for the deepseek-a red-team pass

Working scratch for 280e review report. Not part of the deliverable; committed for
transparency. Each claim tracked to resolution.

## Claim inventory

### C1: The trilemma "dissolution" overclaim (27B §0, §2)
- 27A §1: trilemma = {sound · minimal-oracle · unflagged}, any claims-based picks 2
- 27B §2 finding-trilemma-scope: "A design that never consumes a cross-context claim
  — because the measurement executes in the site's own context — is outside the
  quantifier"
- 27B §0: "Route 1 dissolves the trilemma in every measurable cell (sound +
  minimal-oracle + unflagged, simultaneously)"
- 27C §2: tolerance vouch IS a claim (about body mutation behavior in shifted contexts)
- 27C supersession banner on 27B: "the oracle-side tolerance vouch REQUIRED at the
  default dial (27C §§2–3)"
- VERDICT: 27B's claim is materially wrong. 27C re-introduces claims-dependence.
  27C's banner partially corrects. Trilemma shape: 27C picks {sound, unflagged} at
  cost of {minimal-oracle}. Confirmed reading of 27C §2 + §10.
- Severity: MEDIUM (27C corrects it; 27B's body text remains misleading for readers)

### C2: The completeness migration (279f §3 → 27C §2)
- 279f §3: transport's `:?` backing consumed as completeness → refuse
- 27C §2: tolerance vouch = "this body's effects are read-only BY DESIGN, not by
  privilege-starvation"
- Both are author-asserted negative-universal claims about tool behavior
- Key difference: backing-completeness is NOT empirically testable (can't enumerate
  all answer-dependencies); tolerance is empirically testable (two-user CI, tracer)
- 27C never explicitly argues this testability difference as the justification
- 27C §7: "27C:hole-bad-oracle-blast" acknowledges the blast-radius tradeoff
- VERDICT: The design DOES improve the situation (testable claim vs untestable claim),
  but the response narrative frames this as "different approach" rather than "same
  approach, different observable, better testability." The connection between the
  refusal in 279f and the acceptance in 27C is never drawn explicitly.
- Severity: MEDIUM (design is correct; narrative is misleading in its framing of
  novelty)

### C3: Adjudication structural bias (279f)
- 279f is Fable-adjudicated (same model that wrote corpus)
- Reviews were stance-engineered, manufactured-fault discount applied
- Coverage gap disclosed: sol-adversarial lane produced nothing
- Foreign-adversarial coverage is DeepSeek-only (shallower substrate reads)
- Credit pattern: findings that converge on spec bugs → credited + fixed
- Dismissal pattern: findings about deeper design → parked, deferred, or argued away
- VERDICT: Not a single-document flaw but a systemic epistemic problem. The
  adjudication cannot credibly claim "no kill" when it is structurally the same
  model that produced the corpus, adjudicating reviews of which only one was
  out-of-model-family (DeepSeek) and was described as having "shallower substrate
  reads."
- Severity: HIGH (process finding) but LOW weight per task instructions (not a
  documentable contradiction)

### C4: The tolerance-vouch completeness gap unconnected
- 279f §3: `24D` ruling — backing carries NO completeness burden
- 27C §2: tolerance vouch carries a completeness-shaped claim (body never mutates
  in shifted contexts)
- The tolerance vouch has a completeness gap of the SAME shape as the transport claim
- Difference: tolerance vouch failure → mutation (attributed); transport failure →
  under-execution (unattributed). Sin ordering makes tolerance strictly better.
- CI bar partially mitigates but is not a proof
- 27C §7 explicitly acknowledges `27C:hole-bad-oracle-blast` as a "priced trade"
- VERDICT: The hole is acknowledged. There is no hidden gap. The design honestly
  prices the risk. NOT A FINDING (withdrawn).

### C5: Conditional tails and plan determinism
- 27C §5: tail lines conditioned on guard flags → two possible states per line
- Render under attention law: at most dimmed, annotated
- The plan is no longer "these lines execute, these don't" — it's now
  "these lines execute, these don't unless line 8 runs"
- Attention is PARTIALLY saved (better than unconditional guard, worse than
  unconditional elision)
- VERDICT: Not a design flaw — a legitimate improvement over current state.
  Honesty in rendering is maintained. NOT A FINDING (withdrawn).

### C6: The "four independent angles" claim (279f §1)
- 279a = Fable (in-lineage, same model as author)
- 279b = GPT-5.6-Sol (foreign, neutral)
- 279d/279e = DeepSeek (foreign, neutral/adversarial)
- Sol-adversarial lane produced nothing
- "Independent" requires different methodologies or threat models; same-model-family
  (Fable) and neutral (GPT) reviews are not genuinely independent in the
  adversarial sense
- VERDICT: The "four independent angles" claim overstates independence. Only
  DeepSeek provided genuinely foreign-adversarial coverage. But this is a process
  claim, not a design claim. MINOR.

### C7: 27B §6 "transport dies as a mechanism" vs 27C §4 fallback lane
- 27B §6: "transport dies as a mechanism"
- 27C §4: fallback lane EXISTS (cross-dimension consumption without entry) under
  --risk-faultless-skips
- 27C supersession banner: "transport is DEMOTED, NOT DEAD"
- VERDICT: 27B overclaimed; 27C corrected it. Explicit in supersession banner.
  MINOR (documentation, not design).

### C8: Authoring cost cliff under 27C
- 27C requires: wrapper oracle with entry form + inner oracle with tolerance vouch
  + admin default dial
- Three actors, three authored surfaces, two configuration points
- Missing any link → silent degrade to guard/run
- Hints drive the ladder, so degradation is visible and fixable
- VERDICT: The chain IS long (3+ actors) but degradation is fail-safe (guard/run)
  and hints guide repair. This is consistent with gradual enhancement. NOT A
  FINDING (withdrawn).

### C9: 271 compression loss
- 271 was "compressed 2026-07-12 at the human's direction (turn-by-turn noise
  removed)"
- The adjudication (§7) dismisses the "audit trail deleted" charge as "recoverable,
  not destroyed" (in git history)
- But git history for an AI-human dialogue requires reading raw transcripts, which
  is effectively lost to casual review
- VERDICT: Genuine information loss from compression, but the human directed it.
  MINOR.

### C10: The `analysis/CLAUDE.md` per-selector poison-wall claim
- analysis/CLAUDE.md: "per-selector CELLS are the poison-wall fix: `apt-get update`
  establishes the package-index cell, `install` establishes
  `…Package:nginx#installed` — different cells, no cross-poison"
- This assumes oracles correctly classify what selectors their commands affect
- If an oracle under-claims (update affects more than PkgIndex), different cells
  can still have cross-poison
- This is the frame problem again — just at finer granularity
- The claim "different cells, no cross-poison" is only as good as the oracles'
  accuracy about which cells commands affect
- VERDICT: Not a bug — it's the same caveat that applies everywhere in Dorc
  (oracle correctness is a precondition). The text is slightly overconfident.
  MINOR.

## Disposition
Going into the report with:
- C1 (MEDIUM, HIGH confidence) — trilemma overclaim
- C2 (MEDIUM, HIGH confidence) — completeness migration narrative
- C3 (HIGH process, LOW weight per instructions) — adjudication structural bias
- C6 (MINOR) — "four independent angles" overstates
- C7 (MINOR) — transport "dies" vs "demoted"
- C9 (MINOR) — 271 compression loss

Withdrawn: C4, C5, C8, C10 — all investigated and found either acknowledged, benign,
or consistent with design principles.
