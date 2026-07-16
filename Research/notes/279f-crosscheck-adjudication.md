# 279f — Adjudication of the 270-era crosscheck (reports 279a/b/d/e)

AI-authored (Fable adjudicator, 2026-07-13). The batched adjudication of the four
crosscheck reports (branch `ai/spike3-r23-crosscheck-reports` @ `5ce1757`) against the
block-settle package at review-point `9431ccb`. Authority: root docs and human-TYPED
rulings outrank this; this note records adjudication *verdicts* and *proposals* — the §6
ask-list closes only on typed acks. Adjudication ran under the standing
maximum-skepticism discipline: every credited finding was re-verified against the package
texts; dismissals carry their reasons (§7) so the human can spot-check the skepticism
itself.

Lane weighting applied: 279a (Fable, in-lineage, adversarial, ran its own sub-lanes
including a Codex/GPT-5.6 outside-lineage review of `277`) > 279b (GPT-5.6-Sol, foreign,
neutral) > 279d/279e (DeepSeek, foreign, neutral/adversarial). Coverage gap disclosed:
the sol-adversarial lane produced nothing; foreign-adversarial coverage is DeepSeek-only,
partially compensated by 279a's internal Codex sub-lane. Adversarial lanes were
stance-engineered; manufactured-fault discount applied throughout.

## §1 — Verdict in one paragraph

No kill. The core came back clean from four independent angles: the ternary relation's
routing, the guard shape, never-derive-separation (docker-validated), the kWHICHSH floor
weld, the coordinate grammar, and the razor discipline all held. The genuine soft spots
concentrate in exactly the tier the package itself queued as un-ratified — **the
cross-context transport chain** (`275` §6 and its fact-side sibling) — plus **one
defective spec paragraph** (`277` §3's sparing predicate and its properties list) and a
handful of brief-level gaps. Disposition: **fix the spec paragraph, refuse the transport
ratifications as posed, fold the riders into the dispatch package, and dispatch
block-rebuild.** Nothing found warrants redesign-scale delay; the only redesign-scale
question (backing-completeness, §3 below) attaches to a tier that is not in
block-rebuild and can be deferred without touching it.

## §2 — Adjudication table

Finding keys: F/A = 279a's own numbering; fd = 279b; dsN/dsA = 279d/279e.

| finding | claim | verdict | consequence |
|---|---|---|---|
| 279b-fd1 / 279a-A2 | `:?` backing is positive disclosure, no completeness burden; `275` §6 premise-1 consumes it as completeness ("value is a pure function of its backing") | **CREDITED — the crosscheck's most important result** | transport ratifications refused as posed; §3 |
| 279a-A1 (Codex fd4) | `275` §6 step-4 still cites the r2 derivation as license source, revoked same-day by `271:rul-invariance-speech-act` | **CREDITED as stale-text hazard**, not live contradiction — `277` §2's generator registry and `272`'s amendment carry the correct semantics; newest-wins resolves it. Danger = a builder citing `275` in isolation | `275` §6 amended in place (§4) |
| 279a-A5 (Codex fd1) | `277` §3 sparing predicate spares selector-less (⊤) *backings* | **CREDITED — real spec-text bug** (+SURE; `277` §1 states the ⊤-collide intent, §3's predicate fails to encode the backing side) | `277` §3 amended (§4); regression pin in rebuild brief |
| 279a-F6 / 279b-fd2 (partial) | §3 properties ("monotone", "noise fails safe", "attributable") false as written; dialect growth flips collide→spare | **CREDITED, narrowed**: within-family dialect growth flips comparisons against that family's *own* backings only — the family's declared kill-surface control, flag-gated. The property *wording* is wrong; the mechanism survives | `277` §3 properties reworded (§4) |
| 279b-fd2 / 279a-A6 | cross-family claim-tokens interpreted in the backing family's dialect can spare on divergent meanings; "adjudicability-tier" parking mis-classes a correctness path | **CREDITED, scoped**: all consequences flag-gated (sparing) or safe (collide) at v1; but the limitation is README-class per `271:rul-net-quality-u-curve`, not lint-deferred | fence added to `277` §6 (§4) |
| 279b-fd5 | no set-lifting laws for backing-SETS (existential vs universal quantification) | **CREDITED** — cheap, retrofit-hostile if wrong | one sentence in `277` §5; DST pins (§5) |
| 279a-F1 / 279e-#1 | flag boundary (pinnability) mis-drawn — from opposite directions: F1 "un-flagged transport shares flagged anatomy"; dsA "the flagged at-most claim is line-attributable too" | **CREDITED as a real philosophy-tier challenge**; together they show pinnability under-determines the boundary. Consequence-light at v1 IF transport defers; USER_STORY's stays-local sentence is false only if transport ships un-flagged | ruling-candidate queued (§6 ask-flag-boundary-recut); not a rebuild gate |
| 279a-A3 | value-freeze patrol misses poison-walls (claim-based patrol) | **PARTIALLY CREDITED**: `275` §3 says folds are "patrolled by the walls machinery," which defaults unmodeled⇒total-wall; ~SUSPECT a misreading — but the co-valuation prose reads claim-based, so the ambiguity is real | one-sentence brief rider (§5) |
| 279a-A4 | substrate-token choice is authored, yet carried-by invariance is branded "engine-warranted" — trust-tier mislabel, silently live when netns/fs-view land | **CREDITED, latent** (v1 reserves the slot; human already graded substrate marks SOFT) | wording note rides the fs-view/netns re-entry; recorded here |
| 279a-F2 | pipefail + `\| grep -q` SIGPIPE race ⇒ run-to-run verdict flap; collides with the `--exit-code` divergence-of-world contract | **CREDITED, scoped** (empirically verified by 279a's lane; safe-direction, attention/trust hit) | riders (§5): flap = named nondeterminism class; `--exit-code` never sources from sink-landings; hostsim race injection |
| 279b-fd3 | read-value slice under-specified; imported wire fixes `coord=` fields only — `stdout=` still `split_whitespace`-truncates: "single-line" promise delivers single-token | **CREDITED** (mechanical half taken as written from a high-capability lane; code cites checked against `262` §2's own scope note) | wire-import brief rider + acceptance pin (§5) |
| 279b-fd4 | two-binary floor ≠ parser membership; bare `set -o pipefail` is dialect-legal but survives strip and fails the floor ⇒ off-ramp guarantee cracked | **CREDITED on the pipefail crack only**; the parser-membership half is discounted — the engine parser + `inv-top-reject` IS the membership authority; the floor is a conformance gate, and `278` already separates the three guarantees | ask-pipefail-emit-never |
| 279d-F1 | block-context's headline lanes (W2 whole-ρ replication, transform lane) depend on task-14, deferred with understated scope | **CREDITED** — W1/bare-lane/single-command captures are independent; W2 is not | ask-promote-task-14 (run it during block-rebuild) |
| 279d-F2 | generator registry conductor-proposed, rebuild ships against it | **RESOLVED BY THIS PASS** — the registry survives with the §4 amendments; its three cross-generator test cases adopted as DST pins (§5) |
| 279a-F4 | ordering bakes highest-lock-in design before outside contact; lock-in rationale partially circular | **CREDITED as process observation**, discounted where rhetorical ("audit trail deleted" — the dialogue is in git history, not deleted). Actionable residue = a cheaper reality-contact point | ask-thin-reality-checkpoint |
| 279a-A7 (Codex fd6) | nested-wrapper (`sudo nice cmd`) lend/ρ composition has no stated algebra | **CREDITED, brief-tier**: `273` defines single-peel semantics; composition across a peel chain is unstated | block-context planning owes the composition rule (pointwise lend composition; ⊤ propagates) (§5) |
| 279a-A8 | the two-binary floor never reaches `sh -c` payload interiors; post-strip, `dorc:sh` payloads shift to host-sh semantics | **CREDITED, narrow**: a floor-disciplined payload runs anywhere, so the shift is priced by the same portability discipline; residual = payloads are not floor-TESTED | DX-tooling note (strip could floor-test extracted payloads); quality-bar line (§5) |
| 279a-A9 | zsh "IN via discipline" but the discipline set is open-ended (u-curve dip risk) | **CREDITED, minor** — the set is explicitly accreting; quality-bar owns it | quality-bar accretion continues (§5) |
| 279a-A10 / Codex fd8 | binding-site elision unbinds runtime consumers | credited, **already acknowledged-open** (`275` §5) | hard-gate rider (§5) |
| 279a-A11/A12, 279b-fd4 (row-3) | row-3 dangle and stale spellings vs root-doc promises | credited, disclosed-and-ruled; reconciliation is the human's root-doc queue | ask-root-doc-queue |
| 279d-F3 / 279e-#2/#4 | `state_stored_only_in` blast-radius wider than family peers; `only` mechanically unenforceable; invariance line invisible in the name | credited as **documentation/quality-bar tier** (consistent with the u-curve ruling: docs over imperfect nets) | quality-bar item (§5) |
| 279d-F4 | "invariant" names two mechanisms (authored line vs carried-by outcome) with different provenance | **CREDITED, minor** — the docs distinguish them when read carefully; a grep-able wording split ("structural-invariant" for carried-by) is cheap | rides the same fs-view/netns re-entry note as 279a-A4 |
| 279d-F6 | "incorrectness-inexpressible" is a strong claim for a module-boundary discipline | **CREDITED, minor** — the recommended compile-fail pin is cheap and converts prose to machine-check | rider: a compile-failure test lands with the invited-rooms types (§5) |
| 279a-F5 | attention anti-correlates with need (drifted days) | credited as a **known structural truth** (USER_STORY stage 5 owns it); lens, not finding |
| 279a-F3 | changed-detection fold vs byte-for-byte plan-honesty | credited **narrowed** (unstamped tier; render-honesty reconciliation owed when the replace-tier scope call is made) | recorded; rides the changed-detection unpark rider already in `271` |

## §3 — The transport cluster (what actually matters)

Three lanes independently converged on the same joint: **the cross-context transport
chain consumes the backing field as a completeness claim that no authored mark makes.**
The `:?` mark asserts "this read reads X" (one thing, per the orthogonality doctrine);
`24D` (and the landed type) explicitly give backing *no completeness burden*; yet `275`
§6 premise-1 ("the value is a pure function of the state its backing names") requires
exactly completeness-plus-determinism. A body that honestly marks one read while its
output also depends on an unmarked input (ρ, locale, a cache, a second file) transports
a wrong value with no wrong line anywhere — the cardinal-sin shape the razor arc was
built to kill, re-entering through the value plane. The fact-side transport (the `24S`
§2 sudo headline) shares the gap one level up: the `invariant:` line speaks to the
*store*; nothing audits the verdict-function's actual read-set against it.

Also inside this cluster: `275` §6 step-4's license-source text predates the task-8
razor-conversion (279a-A1) — fixed in place per §4.

**Disposition (proposed, ask-transport-disposition):** the `275` ratifications queued for this pass
(three-regimes · backing-inheritance · transport chain) are **refused as posed**.
Register-backed transport is analytic and stands. World-cell value transport and the
fact-side probe-outside license move to **block-context implementation-planning**, which
must resolve the completeness gap by explicit choice among: (a) v1-defer cross-context
transport entirely (honest-walls-for-worlds, the fallback `271` already recorded); (b) an
authored completeness speech-act (an `only`-flavored read-disclosure — kills the
authored-surface-empty headline; new ceremony); (c) restrict transport to bodies whose
read-set the effect-analysis can fully close (narrow but free of new vocabulary).
**[ANSWERED — 2026-07-16, `27C`, human-ruled: by the option none of a/b/c stated —
(d) measure in the site's own context (reuse-never-acquire, escalation dial ×
tolerance vouch); transport demotes to a flag-tier fallback lane. The refusal of the
`275` ratifications stands.]** None of
this touches block-rebuild: the rebuild builds coordinates, chokepoints, dialect sets,
seams, and fences — no transport license is minted by anything in its build list.

## §4 — Spec amendments applied in place (2026-07-13; each annotated; awaiting ack)

- **279f:fix-spare-top-backing** — `277` §3 comparison: sparing now requires selectors
  on BOTH sides; a ⊤/selector-less coordinate on either side collides. (279a-A5.)
- **279f:fix-dialect-properties** — `277` §3 properties reworded to what is true and
  pinnable: cross-family monotonicity; within-family dialect growth = that family's own
  flag-gated kill-surface control; noise-safety scoped to unmatched tokens.
- **279f:fix-set-lifting** — `277` §5 backing-SETS seam: universal quantification stated
  (sparing needs EVERY footprint×backing pair provably-disjoint; any unknown member
  collides; transport needs every member to transport). (279b-fd5.)
- **279f:fence-divergent-meaning** — `277` §6 gains the frontloaded README-class
  limitation: cross-family claim-tokens are interpreted in the backing family's dialect;
  same-spelled tokens with divergent meanings are the priced residue of shared kinds.
- **279f:fix-275-license-source** — `275` §6: step-4 rewritten to the post-amendment
  license source (typed `invariant:` line + engine carried-by; derivation
  contradiction-checks only), and the section bannered NOT-RATIFIED with a pointer here.

## §5 — Dispatch-package deltas (brief riders; fold into `LIVING_STATUS`'s package)

- **entity-algebra-rebuild brief:** pin the amended §3 predicate (⊤-either-side
  regression test); pin the corrected properties; adopt 279d-F2's three cross-generator
  DST cases (mapped-lend × keyed kind; full-lend × invariant kind; disturbs ×
  dialect-selector sparing); set-lifting universal-quantifier pins.
- **wire-records-v1-import brief:** the value-record stdout field must carry arbitrary
  single-line bytes (last-to-token or length-framed); acceptance pin: embedded spaces
  survive round-trip. (279b-fd3.)
- **value-recipe-reshape brief:** paper-walk `219`'s six-step capture chain against the
  reserved seams as a foreclosure check before freezing representations; per-channel
  backing sets; OutClaim rename.
- **read-value / block-context planning:** hard gate — never elide a capture-binding
  whose variable has live apply-time consumers outside the folded region (279a-A10);
  one-sentence clarification that the value-freeze patrol IS the walls machinery
  (unmodeled interposers wall the fold by default; claims narrow only under the flag)
  (279a-A3); the nested-wrapper lend/ρ composition rule (279a-A7); DST must-covers:
  spaces, empty output, nonzero rc, merged stderr, hidden walls, probe/apply value
  disagreement (279b-fd3; the salvageable residue of 279e-#6).
- **connected-probe / plan surfaces:** the pipefail-SIGPIPE flap is a named
  nondeterminism class — why-lane note on 141-sink landings ("likely benign early-exit
  race; consider full-read form"); `dorc plan --exit-code` computes from
  divergence-of-world facts, never raw sink-landings (sharpens the already-acked wording
  rider); hostsim gains SIGPIPE-race injection so goldens cannot flap. (279a-F2.)
- **typeless-floor / invited-rooms:** a compile-failure test pins the license-plane type
  split when it lands (hint-lane fact refused by license-consuming signatures).
  (279d-F6.)
- **stdlib quality-bar:** store-survey audit item for `kind__state_stored_only_in()`
  ("audited every store this kind's tools reach, from every context?"); prefer full-read
  consumer forms over early-exit `-q` where producers mind SIGPIPE; zsh-discipline list
  stays accreting (279a-A9); payload-text floor-testing noted as a DX-tooling wish
  (279a-A8).

## §6 — Ask-list (typed acks wanted; nothing here closes by silence)

- **ask-amendment-acks:** ack (or revert — each is one commit) the §4 in-place
  amendments.
- **ask-transport-disposition:** the disposition of §3 — refuse the `275`
  ratifications as posed; route the completeness-gap decision (a/b/c) to
  block-context implementation-planning. This is the only ask that touches design
  substance. **[DISCHARGED 2026-07-16 via `27A`→`27C`: option (d),
  measure-in-context; see the §3 annotation.]**
- **ask-flag-boundary-recut:** queue a fresh-session re-examination of `rul-flag-is-razor-residue`
  before wrapper-sudo (W2) dispatches: 279a-F1 and 279e-#1 jointly show pinnability
  under-determines the flag boundary; candidate re-cut = the outcome-centric framing of
  TODO.md:21 (non-local under-execution with no runtime net), which is where the flag
  name already points. Not a rebuild gate. **[SUBSTANTIALLY DISCHARGED 2026-07-16:
  `27C`'s escalation dial is the recut — outcome-centric, both-sides consent
  (dial × vouch); residual flag questions live in `27C` §5's fenced fallback lane.]**
- **ask-pipefail-emit-never:** bare `set -o pipefail` joins the emit-never class
  (analyzer accepts and models it; stdlib/quality-bar require the gate idiom) —
  preserves "strip output is floor-legal" without a strip transform. One-line
  annotations to `276`/`278` on ack. **[ACKED 2026-07-16, typed, with four clarifying
  clauses (handshake-presupposition · lint-hint · the nondurable-emission carve —
  ephemeral post-handshake wire-bytes emit bare, no idiom · accept-don't-modify the
  authored idiom); ruling minted at `276:rul-pipefail-emit-never`; `278` §1/§3/§5
  annotated. One dispatch gate remains: ask-amendment-acks.]**
- **ask-promote-task-14:** run task-14 (the fresh-session structural-vouch
  re-derivation, Opus-conductable) DURING block-rebuild, so its ruling exists before
  block-context implementation-planning. (279d-F1: W2 and the transform lane genuinely
  wait on it.) **[MOOT 2026-07-16: task-14 DISSOLVED — the human clarified
  "triple-check" never meant a clean-room pass; the law + composed-predict repair
  RATIFIED by explicit typed ack after an in-context re-derivation.
  `271:rul-only-oracle-bytes-ship` carries the ruling + riders.]**
- **ask-thin-reality-checkpoint (optional, human-taste):** a thin reality-contact
  checkpoint after block-rebuild — the dotfiles/dogfood book or a stages-1–4
  mini-trial on a real host, wrappers accepted as honest walls — before
  block-context. Addresses the one non-rhetorical residue of 279a-F4 and the human's
  own time-to-tool pressure.
- **ask-root-doc-queue:** root-doc queue adds (human-owned): USER_STORY's stays-local sentence
  gains a transport caveat if ask-transport-disposition resolves toward shipping
  transport un-flagged; the row-3 documented-dangle joins the off-ramp prose.

## §7 — Dismissed / discounted (with reasons)

- **279e-#5** (implicit terminal rc unspecified): answered by `273` §2's per-channel
  vocabulary — a terminal delegation IS the all-channel claim, rc included. A clarifying
  clause could be added but nothing is unspecified.
- **279e-#6** (freeze under `set -e`): the errexit claim is confused (`v=$(false)` DOES
  trip errexit; the report asserts exemption); the salvageable residue is the nonzero-rc
  DST pin, kept in §5.
- **279e-#3** (knife-tier circularity): the v1 consumer set is enumerated and the
  future-consumer question is already carried by `271`'s
  rider-floor-vs-changed-detection-coupling. Wording-tier at most.
- **279e-#7** (permanent pipefail gate-idiom ceremony): a deliberate, priced,
  human-acked choice — the report itself concedes this; recorded, no action.
- **Codex fd3** (carried-by "generates invariance" contradiction): dissolved by 279a
  itself; `277` §2 defers semantics to `272` §3 r1, which is precise.
- **279b-fd4, parser-membership half**: the floor was never posed as the parser spec;
  the engine grammar + ⊤-reject posture is the membership authority. The pipefail crack
  is kept (ask-4).
- **279d-F5** (`272` §3 body reads pre-amendment): the amendment block sits prominently
  inside §3 itself, ahead of the outcomes' consumers; rewriting the body would breach
  the annotate-don't-rewrite convention the corpus runs on. The rebuild brief cites
  `277` §2/§4e (post-amendment) anyway. No action.
- **279a-F4's "audit trail deleted"**: the `271` compression moved dialogue to git
  history per its own header; recoverable, not destroyed. The ordering critique survives
  without the flourish.
- **279e withdrawn attacks A–G**: all seven withdrawals check out against the texts;
  notably B (disturbs claims are harm-dormant without the flag) is correct and
  independently confirms the flag-gating of the dialect algebra's sharp edges.
- **279d's overall "substantively sound" grade**: taken with its stated confidence
  (medium on the two majors) and the lane's shallower substrate reads (its own metadata
  discloses not reading `24C`/`24S`/`24T`/`219` bodies).

## §8 — The code-now weighing (requested counterbalance)

The reviews strengthen, not weaken, the case for building now. Four independent lanes
with a kill mandate produced: zero kills, one wrong spec paragraph (fixed same-day for
the cost of three edits), one un-ready tier that the package had already refused to
self-ratify (now formally refused), and a pile of brief riders. Every remaining open
question of consequence — the completeness gap, the flag boundary, the composition law —
is of the kind that further whole-package design passes have demonstrably NOT settled
(the transport chain survived five sittings and was caught by adversaries reading built
artifacts, i.e., by contact, not by more synthesis). The marginal value of another
design round is low and falling; the marginal value of build-contact (block-rebuild) and
reality-contact (ask-thin-reality-checkpoint) is high. Dispatch, with the §5 riders
and §6 asks.
