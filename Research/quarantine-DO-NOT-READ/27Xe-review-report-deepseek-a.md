# 27Xe — Red-team review report (DeepSeek-A)

AI-authored (DeepSeek-V4-Pro, 2026-07-17). Scope: attack the 270-era response
package for internal contradictions, claims that don't hold under their own rules,
amendments that rename problems instead of fixing them, and dismissals whose stated
reasons fail when the cited text is checked. Working notes at `27Xd`.

This report was produced in a single session by one model reading the entire
package. No subagents were spawned. No cited text was taken on trust — every
finding was verified against the cited source before inclusion. This reviewer is
foreign to the project's corpus family (not Fable/Opus); this is the first
out-of-model-family review of the response package as a whole, including `27C`.

---

## §1 — Findings (most severe first)

### F1: The trilemma "dissolution" is false — 27C re-enters the claims corner it claims to escape

**Severity:** Major. **Confidence:** High.

27B `Research/notes/27B-measurement-placement-rescue.md:35-36` claims that
context-entry probing "dissolves the trilemma in every measurable cell (sound +
minimal-oracle + unflagged, simultaneously)." The trilemma from 27A
`Research/notes/27A-cross-context-transport.md:137-139` was: {sound-by-default ·
minimal-oracle-stays-minimal · unflagged product A under wrappers} — any
claims-based design picks two. 27B's argument for escape was that measurement in
context is "not claims-based" and therefore outside the trilemma's quantifier
(`27B:94-101`, finding-trilemma-scope).

27C then introduced the tolerance vouch (`27C:96-102`): a per-function,
per-dimension authored claim that "this body's effects are read-only by design,
not by privilege-starvation — executing it in a context shifted along the named
dimensions will not mutate." This IS an authored claim — a new surface the oracle
author must write to unlock product A under wrappers. Without it, wrapped sites
guard/run. The minimal oracle that previously needed only `is_converged()` now
needs `: tolerates:user` as well.

27C's own supersession banner (`27B:9-19`) partially corrects 27B: "the
oracle-side tolerance vouch REQUIRED at the default dial." But 27B's body text
(§0, §2, §6) still carries the uncorrected "dissolves the trilemma" claim. A
reader who trusts 27B's narrative without reading 27C's banner will believe
context-entry probing achieves all three trilemma goals simultaneously. It does
not: the tolerance vouch is new ceremony, exactly the kind of author-facing
burden the trilemma's "minimal-oracle-stays-minimal" pole exists to name. 27C
picks {sound, unflagged} at the cost of {minimal-oracle} — a legitimate trade,
but a trade nonetheless. The response never states in its own voice that a
trilemma corner was chosen; it lets 27B's overclaim stand while quietly
correcting it in a supersession banner.

- `27B:35-36` — "Route 1 dissolves the trilemma in every measurable cell (sound + minimal-oracle + unflagged, simultaneously)"
- `27A:137-139` — trilemma definition
- `27C:96-102` — tolerance vouch = authored claim
- `27C:125-128` — "Per-function always ... a two-member oracle marks each member it wants shiftable"
- `27C:334-335` — "the tolerance vouch exists, per-function, per-dimension" (RULED)

### F2: The response never draws the connection between its central refutation and its central fix — the "different approach" is the same approach applied to a different observable

**Severity:** Major. **Confidence:** High.

The crosscheck adjudication's most important result was that the backing mark
carries no completeness burden (`279f:39-40`, 279b-fd1/279a-A2). `24D`'s
selfframing correction explicitly states that backing is declaration-scope with no
completeness burden — consuming it as completeness was the transport chain's
fatal error. This finding was credited, the transport ratifications were refused,
and the question was routed to block-context implementation-planning.

The fix, delivered by 27B→27C, was context-entry probing: measure in the site's
own context, no transport needed. The completeness gap is avoided by never
transporting.

But 27C then replaces the refused completeness claim with a different one: the
tolerance vouch. The tolerance vouch (`27C:96-102`) claims the body "will not
mutate" in shifted contexts — a negative-universal assertion about the body's
complete behavior across all shifted executions. Structurally, this claim has the
same completeness shape as the one that was refused: an author asserting a
property about their tool that requires knowledge of the tool's complete behavior.
The original transport claim asked "does your answer depend on anything beyond
what you marked?" The tolerance vouch asks "does your body ever write, in any
shifted context?" Both are frame-problem questions about tool behavior the author
may not fully know.

The response never explicitly argues why one completeness claim is acceptable and
the other is not. The argument IS available — the tolerance vouch is empirically
falsifiable (two-user CI catches the naive case, tracers catch more), while
backing-completeness is not — but neither 27B nor 27C make this argument. The
connection between "we refused transport because backing ≠ completeness" and "we
now require tolerance vouching for context entry" is a missing paragraph in the
design trail. Without it, the response reads as: the transport completeness gap
was fatal, so we built a mechanism with a different completeness gap and called
it a new approach.

- `279f:39-40` — backing ≠ completeness; 279b-fd1/279a-A2 credited
- `24D:rul24-selfframing-correction` (cited at `279f:39`) — backing carries no completeness burden
- `27C:96-102` — tolerance vouch is a negative-universal about body mutation behavior
- `27C:127-133` — two-user differential CI as mitigation

### F3: The adjudication is structurally incapable of finding "kill" flaws in its own corpus

**Severity:** Process (HIGH severity as epistemic failure, but LOW weight per the
task's stated weighting). **Confidence:** High.

The adjudication (`279f`) was performed by Fable — the same model family that
authored the corpus under review. Four "independent" review lanes break down as:
279a (Fable, in-lineage — same model that produced the corpus), 279b (GPT-5.6,
neutral), 279d/279e (DeepSeek, adversarial but with "shallower substrate reads,"
their own metadata disclosing they did not read `24C`/`24S`/`24T`/`219` bodies).
The sol-adversarial lane (GPT-based) produced nothing. Only DeepSeek provided
genuinely foreign-adversarial coverage.

The adjudication's verdict: "No kill" (`279f:21`). The pattern: every credited
finding maps to a specific, amendable spec paragraph (sparing predicate bug,
set-lifting laws, divergent-meaning fence). Every dismissal maps to a finding
that challenges the design frame rather than the spec text (flag boundary,
ordering lock-in, completeness gap → parked for later). The adjudicator credits
findings it can fix with a text edit and dismisses or defers findings that would
require redesign.

This is not a claim that any specific credited or dismissed finding is wrong.
It is an observation about the structure of the review process: no review
performed in this package was capable of finding a kill-level flaw in the design
frame itself, because (a) the in-lineage reviewers share the same design
assumptions, (b) the neutral foreign reviewer was not adversarial, and (c) the
adversarial foreign reviewers had incomplete substrate reads. The response
package's claim to have survived adversarial review (`279f:228-237`, "four
independent lanes with a kill mandate produced: zero kills") should be read with
this structural limitation in mind.

This finding is HIGH severity as an epistemic problem (the corpus has not been
genuinely adversarially tested) but the task instructions weight it LOW ("LOW
value: repeating findings the earlier review round already made"). While not a
repeat of an earlier finding — the earlier review round never audited the
response — the process concern is inherently lower-value than a design flaw.

- `279f:12-17` — lane composition + coverage gap disclosure
- `279f:21` — "No kill" verdict
- `279f:228-237` — "four independent lanes with a kill mandate produced: zero kills"
- `279f:217-222` — DeepSeek review metadata (shallower substrate reads)

### F4: The response's "ratification" claim covers a design that never received outside review

**Severity:** Medium. **Confidence:** High.

The block-settle package ratified two major designs: the entity algebra (`277`)
and the context-entry probing mechanism (`27C`). The entity algebra underwent a
multi-review adjudication (`279f`). The context-entry probing mechanism did not.

27C was produced in a single day (2026-07-16) from 27B's review-through-dialogue
process. 27B is labeled "Outside review" but was authored by the same Fable model
that wrote the corpus. The human "ruled" on specific items (`27C:334-343`) based
on AI-presented summaries of AI-written analysis. The crosscheck adjudication
(`279f`) predates 27C and could not have reviewed it. No adversarial pass has
touched 27C — not even the limited adversarial coverage that touched the rest of
the package.

27C's §10 status ledger marks items as RULED (human-typed) or STRAWMAN. The RULED
items include the core mechanism: "the tolerance vouch exists, per-function,
per-dimension" and "vouch-required-at-default." These rulings were made in
dialogue between the human and the same AI that designed 27C, with no outside
input. This does not make them wrong — but it means the response's claim to
represent a reviewed, ratified design is misleading for its centerpiece. The
mechanism that replaced the refused transport chain was designed, "corrected in
dialogue," and ruled on — all within the same model family — in approximately one
working day, and has never been reviewed by anyone or anything outside that loop.

- `27C:334-335` — "the tolerance vouch exists ... RULED (human-typed)"
- `27B:9-11` — "corrected in dialogue and 27C is the resulting spec"
- `27A` and `27B` authorship banners — both Fable
- `279f` date (2026-07-13) vs `27C` date (2026-07-16) — adjudication predates 27C

### F5: The `analysis/CLAUDE.md` "no cross-poison" claim assumes oracle correctness

**Severity:** Minor. **Confidence:** High.

`spike/crates/analysis/CLAUDE.md:45-47` states: "Per-selector CELLS are the
poison-wall fix: `apt-get update` establishes the package-index cell, `install`
establishes `…Package:nginx#installed` — different cells, no cross-poison."

This claim is only true if the oracle for `apt-get update` correctly declares
that `update` touches only the package-index cell. If `update` in fact touches
other state (and the oracle under-claims), the cells ARE different — but
cross-poison still occurs because the real-world command touched state the oracle
didn't declare. The frame problem applies to per-selector cells exactly as it
applied to per-entity coordinates: the granularity improvement raises the bar for
oracle correctness but does not eliminate the dependence on that correctness.

The caveat is that this is consistent with Dorc's foundational contract: oracles
must be correct for the engine to be correct. The CLAUDE.md text is mildly
overconfident ("no cross-poison") where it should say "no cross-poison when the
oracle correctly declares what each verb disturbs." The difference is
presentational — a reader of CLAUDE.md might believe the selector granularity
eliminates the poison-wall problem, when it actually just makes the problem finer-
grained and still oracle-dependent.

- `spike/crates/analysis/CLAUDE.md:45-47` — "different cells, no cross-poison"
- `DESIGN.md` — oracle correctness as precondition (throughout)

### F6: The "four independent angles" figure is inflated

**Severity:** Minor. **Confidence:** High.

`279f:228-230` states "Four independent lanes with a kill mandate produced: zero
kills." The lanes were: one in-lineage model (Fable), one neutral foreign model
(GPT), and two adversarial foreign models (DeepSeek), with the sol-adversarial
lane producing nothing. Three of four "lanes" are from two model families, and
one of those families (Fable) also wrote the corpus and performed the
adjudication. Counting the GPT neutral reviewer (which is not adversarial) and
the Fable reviewer (which shares the author's design frame) as "independent" in
the adversarial sense inflates the claimed independence.

- `279f:228-230` — "four independent lanes"
- `279f:12-17` — actual lane composition

### F7: 27B claims transport "dies as a mechanism"; 27C preserves it

**Severity:** Minor. **Confidence:** High.

`27B:330-331` claims "transport dies as a mechanism." `27C:39-42` preserves a
flag-tier fallback lane for cross-dimension consumption. 27C's supersession
banner (`27B:11-12`) explicitly corrects this: "transport is DEMOTED, NOT DEAD."
27B's §6 is materially wrong and has not been corrected in place — the
overclaimed "dies as a mechanism" remains in 27B's body text.

- `27B:330-331` — "transport dies as a mechanism"
- `27C:39-42` — fallback lane preserved
- `27B:11-12` — supersession correction

---

## §2 — Withdrawn attacks

### W1: The tolerance vouch has a hidden completeness gap

Initially flagged as a potential analogue to the transport completeness gap
(refused in 279f §3). On review, the tolerance vouch and the backing mark differ
in a crucial respect: the tolerance vouch is empirically falsifiable (two-user CI,
tracer), while the backing-completeness claim is not (you cannot enumerate every
possible answer-dependency). 27C §7 explicitly acknowledges the blast-radius
tradeoff as `27C:hole-bad-oracle-blast`. The design honestly prices the risk.
**Withdrawn**: the hole exists but is acknowledged and correctly classified as a
different risk category from the transport gap.

### W2: Conditional tails violate plan determinism

27C §5 introduces tail lines whose fate depends on apply-time guard outcomes.
This makes the plan probabilistic rather than deterministic — a line may or may
not execute depending on whether an upstream guard's fallback fired. On review,
27C §5 explicitly states the render stays under attention law ("may-execute lines
never hidden — at most dimmed, annotated"), and the rendering contract is
consistent with the welded `rul-attention-honesty`. The conditional tail is a
legitimate improvement over unconditional guarding (which always shows the
guard). **Withdrawn**: the mechanism is correctly bounded by existing law.

### W3: The context-entry chain is too long

27C requires a modeled wrapper (entry form) + inner oracle tolerance vouch +
admin default dial — three actors, two configuration points. Missing any link
degrades silently to guard/run. On review, this is consistent with Dorc's
gradual-enhancement model: degradation is fail-safe (guard/run), hints drive the
repair ladder, and each rung on the ladder buys value without breaking the rungs
below. **Withdrawn**: the chain's length is a cost, not a bug, and gradual
enhancement handles it correctly.

### W4: The adjudication's dismissal of 279e-#5 (implicit terminal rc) is unverified

I attempted to verify whether `273` §2's per-channel vocabulary actually answers
the claimed gap (the implicit terminal rc of a predict delegation being
unspecified). Without reading `273`'s full body, I cannot confirm or refute the
adjudication's claim. Rather than claim a problem I cannot verify, **withdrawn**
with the note that this reviewer was unable to fully verify this dismissal.

### W5: 271's dialogue compression loses material information

The compression of `271` from turn-by-turn dialogue into bullet-point rulings
(performed at the human's direction) lost detail that is only recoverable from
git history. On review, this is a process choice the human explicitly directed,
not an error in the package. **Withdrawn**: the human owns the compression
decision.

---

## §3 — Summary

The package contains one materially false claim (F1: 27B's trilemma "dissolution"
overclaim, partially corrected in 27C's supersession banner but uncorrected in
27B's body), one missing analytical connection (F2: the completeness gap was
moved, not solved, and the response never argues why the new gap is acceptable),
several inflated process claims (F3, F4, F6), and minor documentation issues (F5,
F7). No single finding is a kill — the response's core mechanisms (context-entry
probing, conditional tails, the entity algebra's amendments) are sound on their
own terms. But the package presents its logic as cleaner than it is: a trilemma
was traded for a different trilemma corner, a completeness gap was migrated to a
more testable observable without acknowledging the migration, and the adjudication
claimed broader adversarial coverage than it achieved.

The weakest link is F4: the centerpiece mechanism (`27C`) was designed in one day
by the same model family that wrote everything else, has never been externally
reviewed, and was ruled on in dialogue between the human and that same model. The
design may be correct — no finding here claims it is not — but its claim to
ratification status rests on thinner ground than the entity algebra's, and the
package does not distinguish the two.
