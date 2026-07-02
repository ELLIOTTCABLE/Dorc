# 235 — post-233 state: the fork's resolution + the escape-hatch taxonomy

AI-authored synthesis of the same-day design conversation *after* `plans/233` was stamped
(2026-07-01); human-corrected in-flight. Exists to give the next step — the adversarial
crosscheck, whose prompts the human dictates — a clean state to stand beside. Trust stamped-233
over this where they conflict. Confidence-marked. **Every spelling below is a strawman** — most
are comments, and comments-as-spelling is to be avoided "at nearly all costs" (the kOOB
no-comment-config redline); the real spelling question is deferred with the whole vouch-surface
family.

## §1 The fork (233's closing question): resolved-in-direction

- **Oracle-side global-shaped claims stay dead in every form** (human, firm): truth-testimony
  ("touches only X and Y" — the word *only* is the same vacuous universal over unattended
  observables as "does nothing"; the hork/private-cache refutation applies a fortiori) and
  blanket-vouch alike.
- **Admin-side consent-priced *attention*-product: gently no** (human). The psychological
  argument: the consent-act itself — typing `# abandon_hope_all_ye_who_statically_analyze_here`,
  every time — taps an exhaustible well of trust and undercuts precisely the relax-into-safety
  the attention-product sells. In normal/reconcile modes the honest product statement stands:
  *Dorc narrows your attention only where the world is described; elsewhere it makes your book
  fast and safe, but not shorter.* Display-compression ("expected: 1 change, 96 verify-no-op")
  is the only attention-story past a poison-wall.
- What remains live is not attention-*rescue* but *escape-hatches from poison-cost* — §2.

## §2 The escape-hatch taxonomy (three distinct things; do not re-conflate)

- **hatch-isolate** *(main-mode; the germane one; ON-FENCE — provisional slug)*: a per-call-site,
  admin-side annotation on the line invoking an opaque — "don't worry about this line affecting
  other lines, I think it's isolated" (human strawmen: `# isolate`, `# dorc-ignore`) — where the
  command still **executes every run**; only its *poison* is suppressed, re-opening downstream
  `guard`→`elide` upgrades. Honest risk-shape: this is consent-to-trust-a-**running**-command —
  it genuinely can invalidate downstream facts, and the consent accepts that wrong-elisions of
  *downstream* commands may result. "The only form of wrong-elision Dorc may ever knowingly
  introduce" (human). Fences sketched (human): **book-exclusive** (writing it inside an oracle is
  an error — consent cannot ship); **unsharable**; required **at every call-site, every time**
  (no global blessing of `hork`; the friction is the anti-cargo-cult mechanism); and subject to
  the **strongest correctness machinery we've considered** — containment, attribution back to the
  annotated line, and distinct disclosure of every downstream elision that *rides* the consent.
- **hatch-bump-exclude** *(bump-mode-only; sketched)*: exclude *this* opaque from
  dependency/dataflow-tracking in `bump` mode specifically (strawman: `# dorc-skip-if-unsound`).
  Motivation: frame-poisoning guts bump's narrowing — ~SUSPECT reading of the human's example:
  "you changed line 70 ⟹ 67 of 100 lines, all of 3..70, must re-run," because the opaque at
  line 3 poisons the fact-base bump needs to trust that 4..69's effects still stand — while
  dependency-tracking can't be abandoned wholesale (it *is* bump). Character: a
  committed-to-git-but-temporary tool — "I'm still working on this *this week*; not the
  intended/long-term approach" — riding inside bump's already-sanctioned user-dictum unsoundness
  class. "Maybe the best form of a still-fairly-bad footgun" (human). One structural note: in
  bump mode the excluded opaque typically does *not run* (it leaves the affected set), so the
  can't-invalidate-what-didn't-run argument partially applies; its residual risk is **missed
  edges in both directions** (something in the affected set wanted the opaque's fresh run; or the
  opaque actually depended on the changed line and should have re-run).
- **hatch-dont-run** *(dismissed)*: consent-to-skip-the-command-itself needs no feature — the
  admin comments the line out. (An AI misread of the human's first `# yolo` sketch argued this
  variant's safety at length in-conversation; recorded here so it doesn't resurface as a design
  object.)

## §3 Cross-cutting notes

- **The second book/oracle divergence** (human observation; applies to both live hatches): until
  now the book/oracle division carried semantic constraints on *one side only* — the
  intentionally-shell-invalid `foo.bar()` naming as opt-in to "not-arbitrary-sh: more warnings,
  more demands." The hatches introduce the opposite: a feature *allowed only book-side, an error
  oracle-side*. Not a showstopper — a gentle weld-snap, principled by the social-globality
  asymmetry (`233` line-315): a dictum is first-person risk-allocation for one book; oracles are
  written to travel.
- **Spelling contact:** all strawmen here are comments; the kOOB redline stands; the historical
  gradual-enhancement flow deliberately moved annotations comment → one-line function →
  (currently, ugh) couple-line function. Nothing here decides spelling.
- **Deferability:** both live hatches ruled additive (human + conductor): the license/witness
  architecture takes new species; the plan-render's claimed-vs-proven split takes new disclosure
  tiers (reserved in task #3: elisions *riding* an isolation-consent must never render like
  proven elisions); the mode machinery hosts hatch-bump-exclude. Parked as task #10.
- **Crosscheck hygiene:** nothing in §1–§2 is in stamped-233. Keep it out of every crosscheck
  brief and artifact-copy; independent convergence by the save-the-attention agent on any of
  these shapes is evidence for the human's fence-sitting.

## §4 Where this leaves the round

Settled + stamped: `plans/233` (the ternary reshape; the frontloaded trade; the guard-license).
Resolved-in-direction: the fork (§1). Parked: the hatches (§2; task #10), spelling (the
vouch-surface family), tasks #2–#9. Next: the 3-agent adversarial crosscheck of stamped-233
(structure + prep in `23Z`); prompts dictated by the human.
