# 280d working notes — review round, DeepSeek N

## Cross-check sources verified

Each bullet traced from a CLAUDE.md file to its cited source, read in full context.

1. spike/CLAUDE.md "ternary-compare-consumer-map (277 §2)"
   → 277 §2 consumer map table: same→transport, provably-disjoint→survival, unknown→safe bottom
   ✓ FAITHFUL (verbatim match on all three verdict consumers)

2. spike/CLAUDE.md "sparing-algebra (277 §3, as amended by 279f)"
   → 277 §3 amended comparison: "BOTH sides carry minted selectors AND claim-token ∈ dialect... AND claim-token ≠ backing-token. Everything else COLLIDES — a selector-less coordinate on EITHER side"
   ✓ FAITHFUL (amendment properly reflected; ⊤-either-side rule present)

3. spike/CLAUDE.md "context-entry-probing (plans/27C)"
   → 27C §0: measurement in denoted context · reuse-never-acquire · no privilege ordering
   ✓ FAITHFUL

4. spike/CLAUDE.md "value-predictions (275 · 271:rul-value-prediction-species)"
   → 275 §2: fields DERIVED, never declared · §9: authored surface = empty set
   ✓ FAITHFUL

5. spike/CLAUDE.md "set-lifting-universal-meet (277 §5)"
   → 277 §5: universal quantification, pin-no-outcome-as-generator, pin-set-meet-order-independence
   ✓ FAITHFUL (all three pins present and correctly summarized)

6. spike/CLAUDE.md "top-identifies-with-nothing"
   → 277 §6: "silence never identifies; ⊤ identifies with nothing (including itself)" + cross-kind same does not exist
   ✓ FAITHFUL

7. spike/CLAUDE.md "empty-world-byte-identical"
   → 277 §6: "Empty world ⇒ byte-identical to HEAD — the whole algebra is invisible until oracles mint it"
   ✓ FAITHFUL

8. spike/CLAUDE.md "fence-divergent-meaning"
   → 277 §6: "a claim-token is interpreted in the BACKING family's dialect... README-class constraint per 271:rul-net-quality-u-curve (documented and differential-tested, never lint-rescued)"
   ✓ FAITHFUL

9. spike/CLAUDE.md "rul-flag-is-razor-residue"
   → 271: "Claims own what lines can say; the flag owns what no line can say."
   ✓ FAITHFUL

10. spike/CLAUDE.md "stability-ledger (276:rul-verdicts-never-stable)"
    → 276: "syntax = marker-gated · __role names = permanent · verdicts = unstable-and-improving, disowned"
    ✓ FAITHFUL

11. spike/CLAUDE.md "two-binary-floor (276:rul-spec-two-binary-floor; KNOBS:kWHICHSH WELDED)"
    → 276: "A valid dorc-lang v0.1 base-dialect text is a stripped file that parses and runs identically under posh 0.14.1 and dash 0.5.12"
    ✓ FAITHFUL (version pins match)

12. spike/crates/analysis/CLAUDE.md "thread-the-flat-coordinate"
    → 277 §1: flat three-place coordinate + context slot
    ✓ FAITHFUL

13. spike/crates/core/CLAUDE.md "flat-three-place"
    → 271:rul-coordinate-shape-flat-three-place + 277 §1
    ✓ FAITHFUL

14. spike/crates/core/CLAUDE.md "pin-no-outcome-as-generator"
    → 277 §5: pin-no-outcome-as-generator
    ✓ FAITHFUL

15. spike/crates/core/CLAUDE.md "pin-set-meet-order-independence"
    → 277 §5: pin-set-meet-order-independence
    ✓ FAITHFUL

16. spike/crates/oracle/CLAUDE.md "tolerates-vouch (27C §2)"
    → 27C §2: per-function, per-dimension; "read-only by design, not by privilege-starvation"; gates context-shifted execution
    ✓ FAITHFUL

17. spike/crates/oracle/CLAUDE.md "rc-partition-here"
    → 271:rul-rc-partition-stands + rul-zero-one-inversion-pair
    ✓ FAITHFUL

18. spike/crates/plan/CLAUDE.md "survive-license"
    → 271:rul-flag-is-razor-residue + 271:rul-flag-named-risk-faultless-skips + 277 §5
    ✓ FAITHFUL

19. spike/crates/plan/CLAUDE.md "conditional-tails (27C §5)"
    → 27C §5: flag on fallback execute, tail lines conditioned, didn't-act branch keeps elision
    ✓ FAITHFUL

20. spike/crates/syntax/CLAUDE.md "charsets-posix-in-spirit"
    → 271:rul-posix-in-spirit-defaults + 277 §4b (selector = POSIX name in spirit; entity = portable-filename + /)
    ✓ FAITHFUL

21. spike/crates/syntax/CLAUDE.md "fence-rejection-rc"
    → 276:rul-spec-two-binary-floor fence-rejection-rc: "no dialect rule may ever depend on the exit code or error text of a rejected construct"
    ✓ FAITHFUL (bonus detail "dash exits 2 where posh exits 1" confirmed in 276)

**Result: 21/21 bullets verified. Zero factual divergences found.**

## Issues investigated and dropped

### 1. Tolerance vouch shares completeness-burden anatomy with flag-gated footprint tier
INVESTIGATED: The tolerance vouch asks authors to distinguish "read-only by design" from "read-only by privilege-starvation" — a tool-internal judgment structurally similar to footprint completeness.
DROPPED: The razor distinction holds — a tolerance vouch is a sayable positive claim (author says "safe in X"; when wrong, attributable), while a footprint omission is an unsayable negative. The design consistently gates unsayable claims behind --risk-faultless-skips and lets sayable claims ride vouch-tier. The blast radius also differs (mutation at plan time vs under-execution at apply time).

### 2. 279f dismissal of 279e-#5 relies on 273 §2 vocabulary that doesn't explicitly cover terminal rc
INVESTIGATED: The dismissal says "terminal delegation IS the all-channel claim, rc included." 273 §2 says "delegation = faithful claim" without explicitly naming rc.
DROPPED: "Faithful claim" in context means all channels — the delegation reproduces the subcommand's observables, which include rc. The 273 vocabulary partitions the claim space exhaustively (delegation / printf / explicit return / redirect-to-null / return 2), and delegation is the all-channel entry. Explicit return is a narrower rc-only claim.

### 3. 27A §5 self-correction has a rhetorical gap
INVESTIGATED: 27A §5's parenthetical says both reasons for a conclusion were wrong but the conclusion stands, without restating the correct reason at the point of original reasoning.
DROPPED: The 2026-07-16 annotation IS the correction; the corrected reason (measuring body's unsayable read-completeness) is stated. This is a documentation-narrative issue, not a design flaw. The 27C redesign renders the entire 27A §5 flagless-kind-invariance argument moot anyway.

### 4. 275 §6 annotation debt could confuse implementors
INVESTIGATED: 275 §6 carries original (incorrect) text interleaved with HTML-comment corrections and supersession banners.
DROPPED: The section banner states NOT-RATIFIED and points to 27C. A careful reader following the source trail lands in 27C. The human acked this as "noise." Documentation quality, not design quality.

### 5. Conditional tails multi-wall conjunction unspecified
INVESTIGATED: 27C §5 describes single-wall conditional tails but doesn't spec the multi-wall case (a line behind two walls, either of which may fire).
DROPPED: The detailed design is explicitly deferred to the placement-spectrum round. The STRAWMAN status in §10 makes this clear. Not a missing piece — correctly parked.

### 6. 27B's imp-1 provenance audit might overstate the case
INVESTIGATED: finding-imp-one-unwelded shows imp-1 was proposal-tier, never human-welded. This finding was critical to unlocking the 27C design.
DROPPED: The human explicitly adopted the rescope (27C:rule-reuse-never-acquire). The provenance audit was correct and productive. Not a flaw.

### 7. CLAUDE.md "structural-vouch-only" claims "exactly ONE source" for probe-inertness
INVESTIGATED: Checked against 271:rul-unprovable-rides-the-vouch which says unprovable regions ride the vouch; 271:rul-structural-vouch-only restated in spike/CLAUDE.md.
DROPPED: Consistent with the rulings. The "exactly ONE source" phrasing accurately captures the designed posture (no analysis-confidence threshold, no sandbox vouch, no second source).

### 8. Does 27C's tolerance-vouch mark grammar conflict with 277 §4c's brace-alternation scope?
INVESTIGATED: 27C §2 shows `: tolerates:{user,fs-view}` using brace alternation. 277 §4c restricts brace alternation to "claim-emission marks only" and says "Verdict and observe marks stay SINGLE-cell."
DROPPED: The tolerance mark is neither a verdict mark nor an observe mark — it's in the bare-mark family (colon-line marks, per 277 §4d). The brace alternation restriction is on verdict/observe marks specifically. The tolerance mark's brace expansion is a natural extension of the bare-mark grammar, consistent with 277 §4d's role-scoped vocabularies.

### 9. 27C never addresses whether --escalate-any-probe interacts with --risk-faultless-skips
INVESTIGATED: Under --escalate-any-probe, unvouched oracles shift contexts — measurements happen in-context, so no transport occurs, so --risk-faultless-skips is moot for those sites. The flags are orthogonal.
DROPPED: Correct by construction. The escalation dial controls measurement placement; the survival flag controls consumption of traveled claims. When measurement is in-context, nothing travels.

### 10. 279f §7 dismissal of "audit trail deleted" (279a-F4) — verified
INVESTIGATED: 271 header says dialogue compressed but git history retains full chronology. The dismissal says "the dialogue is in git history, not deleted."
DROPPED: Checked — 271's git history does contain the compressed dialogue. The claim "audit trail deleted" was rhetorical flourish. Dismissal correct.
