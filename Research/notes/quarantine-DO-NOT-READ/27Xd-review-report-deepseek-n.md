# 27Xd — Review report: the 270-era design package after crosscheck adjudication

AI-authored (DeepSeek, single-session adversarial review, 2026-07-17). This is the
newest-increment review demanded by the round-28 task brief: assess the design package
(block-settle rulings + adjudication + amended specs + context-entry-probing spec +
steering-law CLAUDE.md files) for flaws, contradictions, and hard-to-undo decisions;
cross-check CLAUDE.md bullets against their cited rulings; produce one committed report.

Authority: this is one model's adversarial read. Findings are ranked by severity and
carry confidence markers. Root docs and human-TYPED rulings outrank everything here.

Package read: `279f` (adjudication) · `277` (entity algebra, amended) · `275` (value-
predictions, amended) · `271` (rulings ledger) · `276` (language sitting) · `278`
(language reference) · `27A`→`27B`→`27C` (context-entry trail) · `spike/CLAUDE.md` +
seven crate `CLAUDE.md` files. Grounded in root `README` / `DESIGN` / `IMPLEMENTATION` /
`USER_STORY` / `KNOBS` / `ANALYZER-NEEDS` / `TODO-ADDTL` / `AGENTS.md`.

---

## §1 — Overall assessment

The package is genuinely sound. It survived four independent adversarial crosscheck
lanes (`279f`), absorbed the one defective spec paragraph found (`277` §3's
⊤-backing-sparing bug, fixed same-day), formally refused ratification of an unready
transport tier (`275` §6), and redesigned the measurement lane for wrapped sites from
first principles (`27A`→`27B`→`27C`). Its internal citation discipline is the strongest
I have seen in an LLM-produced corpus: 21/21 traced CLAUDE.md bullets matched their
cited rulings verbatim or in faithful paraphrase, with zero divergences (§4). The
steering law is well-maintained.

The package's residual risk concentrates in a single priced tradeoff: the tolerance
vouch (`27C` §2) asks oracle authors to assert completeness about tool-internal
behaviour under privilege shifts — a judgment the frame problem makes structurally
unknowable — and ships it on the default dial. The design is honest about the risk and
the mitigation (attribution, stdlib CI, two-user differential testing), but the
completeness-burden anatomy is the same shape the rest of the design flag-gates. That
tension is the most significant finding below (finding-1).

There are no redesign-scale defects. The case for building now (`279f` §8) correctly
identifies diminishing returns from further design passes and high returns from
build-contact.

---

## §2 — Findings

### 1. finding-default-lane-completeness-tension — tolerance vouch ships default while carrying completeness-burden anatomy

**severity:** major
**confidence:** medium
**files:** `Research/plans/27C-context-entry-probing-design.md:95-134` · `Research/notes/27A-cross-context-transport.md:354-402` · `Research/notes/279f-crosscheck-adjudication.md:65-97`

The tolerance vouch (`27C` §2, `: tolerates:user`) requires the oracle author to
assert: "this body's effects are read-only by design, not by privilege-starvation." The
vouch ships on the DEFAULT escalation dial — no flag required (`27C` §1: `--probe-
escalation` is the default, and the tolerance vouch is the gate on shifted execution
under that dial).

The problem: for the dominant class of ops tools (package managers, language
toolchains, service CLIs), a `list`/`status`/`check` invocation may internally scaffold
directories, refresh caches, update lock files, or register state when it has write
permission — behaviour invisible to the oracle author who knows only the documented
API. This is the same "tool-internal" half of the frame problem `27A` §1 correctly
identifies as permanent. The author cannot know whether `pipx list` mutates as root on
any given host; the distinction "safe by design" vs "safe by permission-starvation" is
structurally unknowable.

The design acknowledges this risk honestly (`27C` §7, `27C:hole-bad-oracle-blast`: "a
false tolerance vouch mutates in-context at plan time; attributed, bounded"). The blast
radius is narrower than the transport lane's cardinal sin (under-execution), and the
failure is attributable (the vouch is a sayable positive claim). But the anatomy —
asking an author to make a completeness judgment about tool-internal behaviour — is the
same shape the rest of the design flag-gates behind `--risk-faultless-skips`. The
disturbs footprint is flag-gated because omission is unsayable (the frame problem). The
tolerance vouch defaults because the vouch is a sayable positive; but the vouch's
*criterion* (knowledge of tool-internal behaviour) is no more author-accessible than
the footprint's.

This is not a contradiction — the razor distinction (sayable vs unsayable claim) holds.
But the razor is cutting on claim *form*, not on claim *epistemic accessibility*, and
the tolerance vouch's epistemic demands are the sharpest in the default lane. The
design would benefit from stating this tension explicitly in `27C` §7 rather than
burying it as one of five residual holes.

### 2. finding-27a-self-correction-rhetorical-gap — `27A` §5 correction buries the correct reason inside a parenthetical

**severity:** minor
**confidence:** high
**files:** `Research/notes/27A-cross-context-transport.md:428-435`

The 2026-07-16 parenthetical in `27A` §5 corrects a refuted-list entry: the conclusion
"flagless composed transport stays refused" originally had two stated reasons (non-
local blast ⇒ flag-tier; kind-owner's invariance is mis-located). The parenthetical
admits BOTH reasons were wrong (the razor is sayability-centric, not locality-centric;
the kind-owner's invariance line IS a sayable claim, typed same date as ruling
`271:rul-invariance-speech-act`). The correct reason IS supplied — the measuring body's
unsayable read-completeness — but it arrives as an inline correction to a refuted-list
entry, not as a restated conclusion at the point of original reasoning.

An implementor reading `27A` §5 in isolation (without tracing the `27C` redesign)
might land on the wrong mental model of *why* flagless composed transport is refused.
The `27C` redesign renders the question moot for the default lane, but the correction's
placement remains structurally weak — it corrects a refuted entry in a §5 list rather
than the §1 analysis that fed the conclusion.

The 279f adjudication corrected the substantive issue (`279f:fix-275-license-source`)
but didn't catch that the same license-source correction needs to propagate to `27A`
§5's parenthetical with equal clarity. This is documentation debt, not design debt.

### 3. finding-275-annotation-hazard — `275` §6 carries original incorrect text ahead of corrections

**severity:** minor
**confidence:** high
**files:** `Research/notes/275-value-predictions-and-the-capture-lane.md:147-169` · `Research/notes/279f-crosscheck-adjudication.md:114-115`

`275` §6 retains its original (incorrect) transport chain text, with HTML-comment
corrections interleaved. A builder who reaches `275` §6 via a citation (e.g., from
`275` §8's customer routing) sees the original premise-1 first: "The value is a pure
function of the state its backing names" — the very sentence the crosscheck
adjudication credited as the package's most important finding (`279b-fd1`/`279a-A2`),
noting it violates `24D:rul24-selfframing-correction`. The correction follows in an
HTML comment, then another supersession comment. The section's NOT-RATIFIED banner
points to `27C`, but a citation-following reader who trusts the text they see may not
read the banner as a repudiation.

The human acked this as "noise" (`279f` §6: "Acked, softly... this all reads to me as
noise"). The implementor guard is the `27C` pointer. Hazard remains: citation trails
can fork before reaching `27C`, and the stale text in `275` §6 is the highest-stakes
stale text in the corpus (it contradicts a typed ruling).

### 4. finding-conditional-tails-multi-wall-unspecified — multi-wall conjunction for conditional tails is parked without a seam

**severity:** minor
**confidence:** medium
**files:** `Research/plans/27C-context-entry-probing-design.md:221-236`

`27C` §5 describes single-wall conditional tails: wall at line N sets a flag; tail
lines condition on it. The multi-wall case (a line behind two walls, either of which
may fire independently) needs conjunction (`dorc_wall_5 || dorc_wall_9`), and at three
walls the expression grows combinatorially. The detailed design is deferred to "the
placement-spectrum round," and nothing in `27C` §5 reserves the conjunction-algebra
seam.

The single-wall case is the common one (early-book guards are rare; two-fired-walls in
one tail even rarer), so the deferral is low-cost. But the render format and the
conditional-compilation machinery will need to know the disjunction shape. A one-
sentence seam reservation ("a tail line behind N walls conditions on their
disjunction; representation shape TBD") would prevent the single-wall assumption from
baking into the probe emitter.

### 5. finding-27c-dial-interaction-unspecified — interaction between the escalation dial and --risk-faultless-skips is implicit

**severity:** minor
**confidence:** low
**files:** `Research/plans/27C-context-entry-probing-design.md:35-41` · `Research/plans/27C-context-entry-probing-design.md:187-220`

The `27C` design introduces two admin flags that govern this territory: the escalation
dial (`--no-probe-escalation` / `--probe-escalation` / `--escalate-any-probe`) and the
existing `--risk-faultless-skips`. Their interaction is implicit rather than stated:

- Under `--no-probe-escalation`: no measurement-in-context occurs, so the fallback
  lane (§4) is the only route. `--risk-faultless-skips` applies to the fallback lane's
  composed outcomes — this IS stated in §4.

- Under `--escalate-any-probe`: all oracles shift contexts, so measurement-in-context
  moots transport for those sites. `--risk-faultless-skips` has nothing to gate for
  entered sites — this is correct by construction, not stated.

- Under the default `--probe-escalation`: vouched oracles enter contexts; unvouched
  oracles don't. For the unvouched residue, the fallback lane applies, and
  `--risk-faultless-skips` gates it per §4. Correct by construction.

The interaction is mechanically sound — the two flags govern orthogonal concerns
(measurement placement vs claim consumption) — but the design never states this
orthogonality explicitly. An implementor reasoning about flag combinatorics will derive
it correctly, but the derivation shouldn't be necessary.

---

## §3 — Strengths (genuinely sound parts)

The following components of the package deserve explicit acknowledgment as sound:

- **The ternary relation and its consumer map** (`277` §2). The safety inversion
  (believed-no-overlap is safe for transport, dangerous for kill-traffic, and vice
  versa) is a crisp PLT insight correctly routed into the design. The unknown-is-safe-
  bottom property means every partial or incorrect generator degrades safely.

- **The `27A`→`27B`→`27C` design trail.** `27A` correctly identified the transport
  completeness gap and its consequences. `27B` correctly identified the unstated
  premise (imp-1 was never welded) and proposed measurement-in-context. `27C` is the
  resulting spec — reuse-never-acquire, the ternary escalation dial, both-sides consent
  via tolerance vouch, the fallback lane. The trail demonstrates method: a false
  premise produced a design dead-end; finding the premise dissolved the dead-end. The
  `279f` adjudication's transport-cluster disposition (refuse `275` §6 ratifications;
  route to block-context planning) was correct, and `27C` is the answer.

- **The selector-dialect algebra** (`277` §3, as amended). The minting rules (verdict/
  observe marks mint; claims/disturbs never mint), the dialect-per-family scoping, the
  ⊤-either-side collides rule, and the within-family dialect-growth semantics are
  precise, test-pinnable, and correctly flag-gated on the survival side.

- **The steering-law discipline.** Every CLAUDE.md bullet I traced verified against its
  cited source (§4). The self-correction cycle (crosscheck → adjudication → amendment →
  CLAUDE.md update) worked: the `277` §3 `279f:fix-spare-top-backing` amendment
  propagated correctly into `spike/CLAUDE.md`'s sparing-algebra bullet, through `277`
  §3's post-amendment text, and into the crate-level CLAUDE.md files' chokepoint
  descriptions. Zero stale citations.

- **The `279f` adjudication itself.** The maximum-skepticism discipline (verified
  every credited finding against package texts; dismissals carry reasons) is the right
  pattern for LLM-produced review batches. The lane-weighting is disclosed and the
  adversarial-manufactured-fault discount is stated. The one speculative element
  (`27A`§5's "both reasons were wrong" — finding-2 above) is a documentation issue,
  not an adjudication error.

---

## §4 — Mechanical cross-check: CLAUDE.md bullets vs cited sources

21 bullets traced. Method: read the CLAUDE.md bullet, locate the cited
ruling/document at the cited location, read the surrounding context, compare.

| # | CLAUDE.md file | bullet/topic | cited source | verdict |
|---|---|---|---|---|
| 1 | spike/CLAUDE.md | ternary-compare-consumer-map | `277` §2 consumer map table | ✓ FAITHFUL |
| 2 | spike/CLAUDE.md | sparing-algebra (as amended by `279f`) | `277` §3 amended comparison | ✓ FAITHFUL |
| 3 | spike/CLAUDE.md | context-entry-probing | `plans/27C` §0 | ✓ FAITHFUL |
| 4 | spike/CLAUDE.md | value-predictions | `275` §2 + §9 | ✓ FAITHFUL |
| 5 | spike/CLAUDE.md | set-lifting-universal-meet | `277` §5 (all three pins) | ✓ FAITHFUL |
| 6 | spike/CLAUDE.md | top-identifies-with-nothing | `277` §6 | ✓ FAITHFUL |
| 7 | spike/CLAUDE.md | empty-world-byte-identical | `277` §6 | ✓ FAITHFUL |
| 8 | spike/CLAUDE.md | fence-divergent-meaning | `277` §6 | ✓ FAITHFUL |
| 9 | spike/CLAUDE.md | stability-ledger | `276:rul-verdicts-never-stable` | ✓ FAITHFUL |
| 10 | spike/CLAUDE.md | two-binary-floor | `276:rul-spec-two-binary-floor` | ✓ FAITHFUL |
| 11 | spike/CLAUDE.md | rul-only-oracle-bytes-ship (RATIFIED) | `271:rul-only-oracle-bytes-ship` (ratified 2026-07-16) | ✓ FAITHFUL |
| 12 | analysis/CLAUDE.md | thread-the-flat-coordinate | `277` §1 | ✓ FAITHFUL |
| 13 | core/CLAUDE.md | flat-three-place | `271:rul-coordinate-shape-flat-three-place` + `277` §1 | ✓ FAITHFUL |
| 14 | core/CLAUDE.md | relational-compare-chokepoint | `277` §2 + `271:rul-seam-context-slot-and-relational-chokepoint` | ✓ FAITHFUL |
| 15 | core/CLAUDE.md | pin-no-outcome-as-generator | `277` §5 | ✓ FAITHFUL |
| 16 | core/CLAUDE.md | pin-set-meet-order-independence | `277` §5 | ✓ FAITHFUL |
| 17 | oracle/CLAUDE.md | tolerates-vouch | `27C` §2 | ✓ FAITHFUL |
| 18 | oracle/CLAUDE.md | rc-partition-here | `271:rul-rc-partition-stands` + `rul-zero-one-inversion-pair` | ✓ FAITHFUL |
| 19 | plan/CLAUDE.md | survive-license | `271:rul-flag-is-razor-residue` + `271:rul-flag-named-risk-faultless-skips` | ✓ FAITHFUL |
| 20 | plan/CLAUDE.md | conditional-tails | `27C` §5 | ✓ FAITHFUL |
| 21 | syntax/CLAUDE.md | fence-rejection-rc | `276:rul-spec-two-binary-floor` (fence-rejection-rc) | ✓ FAITHFUL |

**Result: 21/21 bullets verified. Zero factual divergences found.**

The fence-rejection-rc bullet (`syntax/CLAUDE.md`) includes the detail "dash exits 2
where posh exits 1" — confirmed in the `276` source text. The tolerates-vouch bullet
(`oracle/CLAUDE.md`) correctly captures per-function, per-dimension scope and the
"read-only by design" wording — confirmed verbatim in `27C` §2.

---

## §5 — Suspicions investigated and dropped

Each entry states what was investigated, what was found, and why it was dropped.

### 5.1 Tolerance vouch is structurally equivalent to flag-gated footprint

**Investigated:** Whether the tolerance vouch's completeness-burden anatomy (author
must know tool-internal behaviour) is materially the same as the disturbs footprint's,
such that both should be flag-gated.

**Found:** The razor distinction holds: a wrong tolerance vouch is a sayable positive
claim (author said "safe in X"; attribution works when it's wrong), while a wrong
footprint is an unsayable omission (author didn't list Y in `disturbs()` because they
didn't know about Y). The blast radius also differs — mutation at plan time vs under-
execution at apply time.

**Dropped because:** The design is applying its own razor consistently. The tension is
real (see finding-1) but is a pricing decision, not a contradiction. The design's
honest residue statement (`27C` §7) acknowledges the risk.

### 5.2 279f dismissal of 279e-#5 (terminal delegation covers rc) relies on implicit claim

**Investigated:** The dismissal says `273` §2's vocabulary makes delegation an all-
channel claim including rc. `273` §2 says "delegation = faithful claim" but doesn't
explicitly enumerate rc.

**Found:** `273` §2's five-entry vocabulary (delegation / printf / explicit return /
redirect-to-null / return 2) partitions the claim space exhaustively. Delegation is the
all-channel entry (reproduces all observables); explicit return is a narrower rc-only
claim. A terminal `"$@"` that doesn't capture rc differently IS faithfully reproducing
the subcommand's rc.

**Dropped because:** The vocabulary exhaustiveness makes the rc-inclusion derivable.
One extra word in `273` §2 ("delegation = faithful all-channel claim") would close
this, but the dismissal is correct as-is.

### 5.3 `27A` §5's self-correction has a rhetorical gap

**Investigated:** The 2026-07-16 parenthetical correcting `27A` §5 says both reasons
for a conclusion were wrong but the conclusion stands.

**Found:** The correct reason IS supplied (measuring body's unsayable read-
completeness), but it's buried in a refuted-list entry rather than restated at the §1
point of original reasoning.

**Dropped because:** Promoted to finding-2 (minor, documentation-tier). Not a design
flaw — the conclusion is correct and `27C` renders the question moot for the default
lane.

### 5.4 279d-F5's complaint about `272` §3 body reading pre-amendment was correctly dismissed

**Investigated:** The adjudication says "the amendment block sits prominently inside
§3 itself, ahead of the outcomes' consumers."

**Found:** Verified. `272` §3 carries banner-level amendment text at its top. The
annotate-don't-rewrite convention is the correct corpus discipline for an LLM-produced
design corpus (rewriting would destroy amendment provenance).

**Dropped because:** The dismissal holds. The rebuild brief cites `277` §2/§4e (post-
amendment) anyway, so the citation trail is clean.

### 5.5 `27C` conditional tails multi-wall conjunction unowned

**Investigated:** Whether the multi-wall case needs a seam reservation in `27C` §5.

**Found:** Single-wall is the common case. Multi-wall conjunction is combinatorially
messy but genuinely deferred to the placement-spectrum round.

**Dropped because:** Substantively correct. Promoted to finding-4 (minor, seam
reservation).

### 5.6 279f dismissal of "audit trail deleted" (279a-F4's rhetorical flourish)

**Investigated:** Whether the `271` compression destroyed design dialogue.

**Found:** `271`'s git history retains the full chronology. The compression removed
turn-by-turn noise from the living document; the dialogue is recoverable.

**Dropped because:** The dismissal is correct. The 279a-F4 ordering critique survives
without the flourish.

### 5.7 CLAUDE.md "structural-vouch-only" claims "exactly ONE source"

**Investigated:** Whether the claim overstates by ignoring other sources of probe-
inertness assurance.

**Found:** `271:rul-unprovable-rides-the-vouch` says unprovable regions ride the
authored vouch. `271:rul-proven-mutation-fails-fast` makes proven-mutation fail-fast.
The "exactly ONE source" claim is about vouch-based license to probe, not about the
totality of safety mechanisms. The effect-check is falsification, not a license-source.

**Dropped because:** The bullet's wording is accurate in context (it lives in the
invariants cluster about vouch and trust). No other source mints a probe-execution
license.

### 5.8 Brace-alternation grammar scope for tolerance marks

**Investigated:** Whether `: tolerates:{user,fs-view}` conflicts with `277` §4c's
restriction of brace alternation to "claim-emission marks only."

**Found:** `277` §4c restricts brace-alternation to "claim-emission marks only" AND
explicitly excludes verdict/observe marks. The tolerance mark is in the bare-mark
family (colon-line marks, `277` §4d) — it is neither a verdict mark nor an observe
mark. It's a claim-emission surface (it claims something: tolerance).

**Dropped because:** The grammar compartments are correctly separated by role. The
tolerance mark is a claim-emission mark; brace-alternation applies naturally.

### 5.9 `--escalate-any-probe` × `--risk-faultless-skips` interaction unspecified

**Investigated:** Whether the two flags interact in non-obvious ways.

**Found:** Under `--escalate-any-probe`, all oracles enter contexts, so measurement-in-
context moots transport. `--risk-faultless-skips` gates traveled claims; nothing
travels when measurement is in-context.

**Dropped because:** Orthogonal by construction. Promoted to finding-5 (minor,
documentation).

### 5.10 The `275` §6 stale-text hazard

**Investigated:** Whether the original incorrect text in `275` §6 could mislead a
builder.

**Found:** The NOT-RATIFIED banner points to `27C`. A careful reader gets the right
answer. A citation-following reader who stops at `275` §6 could be misled.

**Dropped because:** The human acked softly ("noise"). The implementor guard exists.
Promoted to finding-3 (minor, documentation hazard).

---

## §6 — No-action items (things I did not find)

- **No internal contradictions.** The package is self-consistent. Where documents
  disagree (e.g., `27A` vs `27B` vs `27C`), the supersession banners correctly route
  to the newest ruling.

- **No hard-to-undo decisions shipped as default.** The welded items (kWHICHSH floor,
  flat coordinate, `#` selector, rc partition, kLANG) are genuinely settled. The
  provisional items (selector dialect, STRAWMAN spellings, conditional tail mechanics)
  are correctly marked as provisional.

- **No amendment that fails to fix the finding it answers.** All five `279f` §4
  amendments correct the specific defects credited in the adjudication table. Verified:
  `fix-spare-top-backing` (⊤ on backing side now collides), `fix-set-lifting`
  (universal quantification stated), `fence-divergent-meaning` (frontloaded limitation
  with owner-adjudication clause), `fix-dialect-properties` (reworded to what is true
  and pinnable), `fix-275-license-source` (step-4 rewritten).

- **No dismissal whose reason does not hold.** Verified the five dismissals in `279f`
  §7 that I could independently check (279e-#5, 279e-#3, 279d-F5, 279a-F4's "audit
  trail," 279e-#6). All hold.

- **No CLAUDE.md bullet that diverges from its cited ruling.** 21/21 verified faithful
  (§4).
