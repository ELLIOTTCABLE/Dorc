# 28Rb — Adversarial review of `plans/28Q` (context-kernel unification)

> Tier: adversarial-crosscheck lane, Fable, clean context, 2026-08-01. Charter: "find
> where it breaks down"; TYPED/ACKED items primary targets; invented faults worse than
> no finding. Read base: 28Q · root README/DESIGN/IMPLEMENTATION/USER_STORY/KNOBS/TODO ·
> Research/README · 28M · 28K · 28P (full) · 27C · 26K · ANALYZER-NEEDS (cited rows) ·
> AID-NEEDS (cited laws) · spike/CLAUDE.md + crate CLAUDE.mds (cited slugs) ·
> `oracle/src/load_inert.rs` (verified in code). Certainty marks per the house
> convention. NOT committed; in-tree note only.
>
> Context fact that frames everything below: **stage-0 is already built** (verdict_lane
> in `plan/src/lib.rs`/`cli`, crate CLAUDE.mds updated, `pin28-split-family-lane-separation`
> re-blessed), and LIVING_STATUS marks stages i–ii "dispatchable." Findings 1 and 2 sit
> squarely in stage-i/ii's path.

## Verdict in one paragraph

The plan is not the incoherent mess the distrust-framing allows for. Its strongest
parts survive genuinely hostile reading: the stage-0 verdict-primacy re-cut is sound
and repairs a measured two-author license; the frame-relative fence reading is arguably
*more* correct than the file-keyed one; the day-N availability law survives the
four-by-two exclusion checks; the flat-domain reconciliation is real (frames are not
k-CFA). But I'm +SURE of two concrete, load-bearing holes — one semantic (P1's
helper-closure story is wrong under sh's own resolution rules), one structural (P2's
closure concept has *no legal spelling* under the current input rules, and §9's
"complete list" omits the amendment it needs) — plus one TYPED rule whose effect
quietly guts a ratified verdict it claims not to touch, and a set of overclaims that
will mislead stage briefs if ratified as written.

---

## F1 — P1's "helper closure" row is semantically wrong as written; the plural idioms' delivery is overclaimed  [severity: HIGH — stage-i correctness]

**Location:** `28Q` §1 items 1–2 ("Every derived row — … a helper closure — is keyed by
the DefinitionId that produced it: … Computed once, whole-unit, exactly as today. No
index multiplication"; "The ONLY per-frame structure is the frame → live-definition
map") and consequence bullet 1 ("`28K` §1 `rul-scope-by-subshell-resource` … finally
delivered as written").

**The fault.** sh resolves a function body's calls against the environment **at
invocation, not at definition**. A helper binding is therefore a property of the
consuming site's *frame*, not of the role definition's DefinitionId. The plan's whole
point (P1) is to make same-name-different-bytes definitions answer positionally — but
the moment role members go positional, their helpers can be positionally plural too
(the blessed-override idiom: an admin `unset -f`s a member and supplies their own body
*and* a same-named helper; the regional-preference idiom: two competing files for one
tool sharing a helper name). Then "closure computed once, whole-unit" is exactly the
chimera the plan claims to make unrepresentable, one level down: site S answers from
the positionally-correct role definition D while D's pinned closure carries a helper
body no shell at S would resolve — a body executing under a binding its author never
saw. That is the pope-sin class (`271:rul-sin-ordering`) `28P:dec-the-gate-is-agreement-
not-re-resolution` built the veto to prevent, and P1 retires the veto.

**Why the obvious repair collides with existing law.** Making closures frame-keyed is
the semantically-correct fix, but its *emission* story breaks: two frames with two live
`_check` bodies in one artifact need helper-level hash-munge, and unlike role functions
(where the munged call comes from engine glue — `( yum__is_converged_h4x2 … ) || …`),
a helper's call sites live **inside authored role bodies**. Renaming them there
violates `28K` §4 `rul-pin-by-definition-bytes` (pinned material is "the
analysis-resolved definition's bytes (strip-applied, authored)") and the strip-law's
no-in-body-name-rewriting spirit. So frame-plural helpers are unrepresentable in the
pinned artifact without either byte-fidelity violation or whole-closure duplication
under munged role names with rewritten interiors. ~SUSPECT the same shape recurs for
constants (`28P:dec-constants-ride-per-contributing-file`: constants travel per
contributing file, whole-unit — a constant reassigned across frames re-opens it).

**The honest resting point the plan should state and doesn't:** keep
`helper-declaration-contested` (the existing differing-bytes withhold) as a standing
fence, i.e. the plural idioms deliver **only where the competing definitions' helper
namespaces don't collide on differing bytes**. That is a real, probably-acceptable
residual — but §1 instead claims the idioms "ANSWER from their positionally-live
definitions" and that `rul-scope-by-subshell-resource` is "finally delivered as
written," which a stage-i builder will read as a mandate to make the colliding case
work. Nothing anywhere in 28Q mentions helper plurality. (+SURE of the semantic gap;
~SUSPECT how often real oracle pairs collide on helper names — underscore-prefixed
names lean tool-specific, so the *frequency* may be low; the *hole in the plan text*
is independent of frequency.)

**Ask:** one ruled sentence in §1 — either "helpers stay whole-unit-contested
(plural-helper worlds withhold; disclosed value-loss)" or a designed frame-keyed
closure+emission story. The first is a one-liner and conservative; the second is real
design work the plan currently hides inside "No index multiplication."

## F2 — P2's entry-closure has no legal spelling; §9's "complete list" is incomplete; the proven package shape and the canonical story diverge  [severity: HIGH — stage-ii coherence]

**Location:** `28Q` §2 (the closure definition; "`price-multi-file-package` softens to
nothing — one entry sourcing its parts is one speaker; the bitem6-proven two-file
helper-package shape becomes the canonical packaging story"; "one CLOSURE mints a
kind's vocabulary members") and §9 ("the complete list; nothing else is open here").

**The fault, verified in code.** The entry-closure is "the transitive closure of
literal `.`-sourcing from an entry file." But there is currently no legal way for a
marked oracle file to source anything: `oracle/src/load_inert.rs::item_is_load_inert`
(lines 59–73) admits **only** funcdefs and static bare assignments — a top-level
`. helpers.sh` is a Simple command with words ⇒ `oracle-file-not-load-inert`, per
`28K:rul-marked-file-is-load-inert`'s own text ("never commands"). Book-side, a
top-level `.` walls (`28P:res-book-sourcing-wall-gates-this-item's-payoff`). So *every*
closure today is a singleton. 28Q lists the book-side blessing
(`res-dot-blessing-is-engine-side`, §9 pin 2) but **omits the oracle-side load-inert
amendment** — allowing literal `.`-of-a-provably-load-inert-file inside marked files —
without which §2's headline benefits are unbuildable. That amendment is a dialect-
surface widening of exactly the class the plan elsewhere routes to §10/human, and §9
claims completeness. +SURE.

**The divergence.** The two-file package shape bitem6 *proved*
(`pin28-helper-package-entrypoints-lift`) is CLI-**sibling-loaded** — no sourcing
anywhere; the closure resolves through the live environment. Under P2's definition
those are two closures. `28M` §7's tune (helpers ride under the *calling* entrypoint's
custody) papers over it for custody — but then custody is caller-keyed, not
closure-keyed, and §2's "custody flows to the closure" is ambiguous about which. Where
it stops being papering: **kind-owner single-occupancy** — "one CLOSURE mints a kind's
vocabulary members." A kind-owner package with `__resolve` in one CLI-loaded file and
`__disturbance_reaches_only` in another is legal today (occupancy is per-member-name)
and becomes two closures under P2 — a regression-by-re-keying for a shape nothing
currently forbids, fixable only by the sourcing that can't be spelled. ~SUSPECT this
exact cell was never exercised (no stdlib yet), but the re-key silently narrows a legal
surface while claiming `price-multi-file-package` "softens to nothing."

**Ask:** add the load-inert amendment to §9 as an owed ruling; state whether closure
membership is sourcing-only or sourcing-plus-CLI-co-naming (they give different
speakers); re-check the kind-owner occupancy sentence against the split-file owner
package.

## F3 — `rul-blessing-flows-from-best-caller` [TYPED] guts the ratified keep-verdict while §5 claims that verdict is "NOT ruled here"  [severity: MEDIUM-HIGH — trust-surface design]

**Location:** `28Q` §2 (the blessing paragraph) vs `28M` §11 (human lean at close:
"keep-for-now; re-open at spike end on the fires-often × bites-rarely instrument") and
`28M` §10 (`prn-vocabulary-is-output-side`; the same-day withdrawal of
`prop-observe-marks-feed-the-dialect`).

**The fault.** The keep-verdict's content was: verdict-body gen-marks mint facts but
their words do NOT enroll in the sparing dialect, because enrollment is a liberalizing
distinctness-warrant and the risk concentrates in judgment-tier proxy-checks. 28Q's
best-caller rule makes enrollment flow transitively from predict-reachability: "a
helper that a family's predict() ever reaches transitively … carr[ies] predict-tier
vocab-minting rights EVEN when invoked from is_converged()." Two consequences the plan
doesn't confront:

1. **The exclusion becomes a ceremony, not a fence.** A verdict-only author enrolls
   their words by factoring them into a helper and adding one thin delegating
   `predict`. The plan *advertises* this ("multi-arm verdict-only authors factor
   describing lines into helpers reached by one thin predict; the stage-4½ hill
   flattens"). But if the exclusion is bypassable by a one-line wrapper, its remaining
   protective content against the two-grades cell (undeclared-read proxy checks — the
   named risk that justified KEEP) is near-zero. The plan keeps the fence's *name* and
   deletes its *function*, while §5 says `28M` §11's keep/lift verdicts are
   "Made-visible but NOT ruled here." One of those two sentences is wrong.
2. **Enrollment becomes refactor-sensitive call-graph topology.** "Reachable from
   predict" is not a speech-act; it changes when an author refactors with zero
   semantic intent, silently widening the family's dialect and therefore what SPARES
   at the survival tier. That sits uneasily with the human's own articulated principle
   (`prn-vocabulary-is-output-side`: only an output *act* carries a distinctness
   warrant) and with the reason `prop-observe-marks-feed-the-dialect` was withdrawn
   (a monotonicity claim computed against the wrong baseline — enrollment's effect on
   the family's *other* words was ignored; best-caller has the same blind spot: it
   grades the helper's care-tier, not the vocabulary-growth's effect).

The inference itself ("reached-by-predict ⇒ demonstrably built under description-tier
care") is topology-to-intent and fails in the reverse-flow cell: a helper written FOR
`is_converged` (judgment-tier, "close-enough" checks) that predict later reaches
through a shared utility path gets elevated everywhere. `pin-blessing-keying` being
held open ("only bites when one closure hosts families of divergent care") does not
cover this — the bite is within one family, one closure, two members' care-tiers.

**This is TYPED substance, so it may already be welded.** If so, the honest follow-on
is to also mark the `28M` §11 keep-verdict as effectively superseded-in-function and
move the two-grades risk onto the spike-end instrument explicitly — not to carry both
texts as if compatible. ~SUSPECT the human typed the custody/grade *direction* (two
relations, two flow directions — which is sound) without the ceremony-bypass
consequence being priced; that consequence deserves a typed line of its own.

## F4 — P3's ssh story mis-seats the re-parse problem and "uniform entry" hides a necessary asymmetry  [severity: MEDIUM — stage-iii design edge, partially fenced by §10]

**Location:** `28Q` §3 bullet 1 (`rul-host-entry-is-ordinary-entry` [ACKED], the
re-parse rider) and §0's table (host as a context dimension).

**Three sub-faults:**

1. **The re-parse rider is a category error.** "ssh's genuine extra obligation is the
   remote-shell RE-PARSE round-trip, which is exactly its entry-siting vouch
   discharge." The entry-siting vouch (`27C:rul-entry-denoted-siting-vouch`) is
   discharged by authored body-code: structural insensitivity, tool interrogation, or
   a same-head tripwire. None of those techniques can verify **argv fidelity across
   ssh's join-and-re-parse** — that a site's argv survives `ssh host cmd 'a b'`
   becoming remote `cmd a b` is a *static analysis* question about what site the line
   even denotes, answered before any body runs. The corpus already owns this problem
   under a different name: payload decomposition (`plans/24T` — `sh -c` strings,
   heredoc books, the render ladder), which 28Q §3 never cites. `ssh host CMD…` is
   structurally closer to `sh -c` (a payload) than to `sudo` (a peeling wrapper whose
   remainder execs verbatim — the wrapper-law fence 28Q waves off as "mechanical, not
   categorical"). An honest ssh oracle can *decline* non-round-trippable argv, which
   is a fine floor — but the plan's one-liner tells the §10 sitting this is a vouch
   rider when it is the 24T problem wearing a network hat. -GUESS the §10 dig would
   rediscover this; the ACKED rider as written points it away from the right prior.
2. **"Entry is uniform across dimensions" cannot be literally true.** Probing the
   CLI-named target host IS entering a host context; under uniform entry +
   `27C`'s unchanged consent machinery, the default-dial/vouch requirements would
   apply to the product's baseline operation (every oracle body needing a
   host-tolerance mark before ordinary remote probing; `--no-probe-escalation`
   forbidding probing the named target at all — absurd). The actual design must carry
   an ambient-host asymmetry (consent-by-invocation for CLI-named targets;
   dial × vouch × entry only for in-book host shifts). The plan says "Local-exec: the
   controller is a context available-at-probe by definition" but never states the
   named-target rule. An ACKED "uniform" that must be non-uniform in the first cell
   anyone builds is exactly the kind of weld that later gets cited against the needed
   carve. +SURE the asymmetry is needed; ~SUSPECT it was simply assumed.
3. **Probe-time reach and wall-clock.** Under P3, an incidental in-book
   `ssh build-server make` (foreign host, one line) graduates from "unmodeled, walls,
   costs nothing at probe" to "a denoted context the probe phase may attempt to
   enter" — a new network dependency in the plan-construction critical path, where
   DESIGN says the user is sitting and waiting. An unreachable side-host = a connect
   timeout per plan, forever, for a line the admin never asked Dorc to understand.
   Licensing-wise everything degrades safely (can't-say ⇒ guard/run); *latency*-wise
   this is a regression the plan never mentions, and per the perf-doctrine
   (tunnels dominate; never let a network boundary participate in iteration) it is
   the one cost class the project has promised to watch. Needs a stated policy
   (opt-in side-host entry, lazy entry, or per-host timeout budget) in stage-iii's
   brief.

## F5 — The stage gates verify almost nothing the refactor changes; the plan inverts the project's own measure-first discipline  [severity: MEDIUM — process/verification]

**Location:** `28Q` §1 gate (`syn-single-frame-byte-identical`), §8 stage-i
("Unblocks: plural-idiom fixtures (expect new corpus cells, not churn)") and stage-iii
gate ("a book with no lifecycle events is byte-identical").

The corpus is single-definition-per-role and define-before-use (stage G's respell), so
byte-identity over it is *vacuously* satisfied by the new machinery — 28P says so
itself, repeatedly ("zero churn … would NOT have been [expected] for a plural one — so
this seat is under-covered by the corpus"). The lane's own best catches came from
commissioned fixtures built BEFORE ruling (bitem6 produced
`fnd-a-split-family-elides-on-two-authors`, the finding that forced stage-0; item0's
measure-then-fix caught the deriv-lane under-execute). 28Q reverses that order: the
frame conversion lands under a blind gate, and the fixtures that could catch a wrong
frame-lookup arrive afterward as "unblocks." Stage-i's brief should *commission the
plural-idiom fixtures first* (blessed-override answering above/below the unset;
subshell re-source in/out; the F1 helper-collision cell; munge-reachability) exactly as
bitem6 was commissioned. Also: stage-iii's gate as worded ("no lifecycle events ⇒
byte-identical") does not cover the ssh-line and local-exec behavior changes stage-iii
itself introduces — a book with zero lifecycle events but one ssh line is *supposed* to
change behavior under stage-iii, so the gate needs the carve spelled ("no lifecycle
events, no host-denoting lines, no local-exec").

## F6 — Overclaims and hygiene (each small; together they shape briefs wrongly)  [severity: LOW-MEDIUM]

- **`syn-zero-new-spellings` overclaims.** P3's load-bearing input — how an oracle
  says "`useradd` begins the user context" — has NO existing spelling; no current
  member (predict/disturbs/lend_map) carries begin/end semantics. §0 admits this only
  parenthetically while headlining "No new authored surface anywhere." The claim is
  true of the *sh* side and false of the *oracle-vocabulary* side, and it is the
  vocabulary side that `sent-language-is-becoming-crufty` watches. If ratified as a
  virtue-sentence, it will be cited against the very §10 additions stage-iii cannot
  proceed without.
- **"For free" contradicts the lane's own pricing.** §1: the whyworld/survival seat
  asymmetries are "oracle-only-vector coincidences that the one-lookup design
  deletes." 28P priced exactly that unification as "re-lifting that seat's whole
  world, which is a dispatch and not a rename." The one-lookup design *routes* the
  fix; it does not make re-lifting free. Stage-i sizing should carry 28P's price tag,
  not §1's.
- **Vocabulary retirement without a mapping.** "'Epoch', 'pivot', and 'transit' are
  retired … older documents using them read through this vocabulary" [ACKED] — but the
  old "transit" concept splits across TWO new rows (a lifecycle event when it's
  reboot-shaped; context entry/exit when it's wrapper-shaped), and 26K's
  transit-relative law reads correctly only under the first. A one-line mapping table
  (epoch→incarnation/availability-window; transit→lifecycle-event; pivot→host-arrival
  + scope-entry) would cost three lines and prevent the exact class of
  corpus-misreading the reading-rule creates.
- **The anti-piecemeal ledger misses a named adjacent item.** §5 sweeps bitem4/5,
  the registry, 26K §0b, the seat asymmetries — but not
  `res-survival-lanes-still-ship-closure-less` (`cli/CLAUDE.md
  one-helper-index-two-lanes`, "somebody's dispatch"), which is cheap, adjacent to
  stage-i's closure machinery, and exactly the kind of orphan the NO-MORE-PIECEMEAL
  banner exists to catch.
- **Ack-ledger blast radius.** "The three-pillar direction as a whole [ACKED]" is the
  kind of umbrella later agents cite to defend mechanics the ledger itself grades
  PROPOSED. The per-item grading is honest; the umbrella line invites lazy citation.
  Consider striking it or scoping it ("direction, not mechanics").

## Suspicions tested and dropped (so they aren't re-litigated)

- **Stage-0 verdict-primacy** — sound, and the strongest part of the plan. Checked
  against value-reproduction law (the ACKED `rul-erasure-license-splits-by-effect-class`
  keeps read-shapes on delegation-measured values, so byte-consuming sites lose
  nothing); against `inv-one-observable` (the verdict rc is the cell's measurement
  instrument, not a resurrected Verdict type; verdict-lane already existed as
  fallback); and against the 28P record (it repairs the *measured* two-author license
  `fnd-a-split-family-elides-on-two-authors`, and the fallback ordering it inverts was
  genuinely an unratified round-23 expedient). The gate's "an outcome move is a
  finding, never churn" is the right posture for verdict-argparse-narrower-than-predict
  sites.
- **The frame-relative committee fence** — initially suspicious, survives: sparing
  claims attach to the wall's frame, and the fence over frame-live members correctly
  catches the retroactive-liberalization cell (a blessed verdict override leaves the
  old author's `disturbs` live ⇒ two closures at that frame ⇒ sparing-inert). It is
  arguably the *more* correct reading of `28M` §4, not a weakening.
- **Superseding the `28P:amend-plural-value-loss-hold`** — legitimate: the hold's
  condition ("until the human replies") was discharged by the r28-megamerge dialogue
  this plan records; the fence stays UNRATIFIED-marked and the fence sitting stays
  open (§9 pin 5), matching `28M` §7's motion-authorized posture.
- **Day-N availability law** — survives the four-by-two exclusion checks: destroyer
  symmetric (departed ⇒ can't-say ⇒ run); build-day guards sound in-sequence; walls
  poison availability facts like any fact; unmodeled creators stay walls by their own
  running. The `useradd`/`sudo -u` generalization of `27C`'s pre-mount chroot story is
  a genuine unification, not a stretch.
- **kSTATE / hermeticity / rec-5** — respected: availability is per-run from plan
  structure + probe reachability; incarnation markers are within-run; unplanned churn
  routes to the integrity plane (consistent with
  `rul-integrity-failure-withholds-mutation` and the `an-toctou-window` WONTFIX).
- **`an-flat-domain`** — the reconciliation is real: frames are piecewise-constant
  intervals over one program order plus a static fork tree; no call-strings, no
  closure recombination. The owed ANALYZER-NEEDS paragraph can be written as claimed.
- **`rul-command-v-is-a-stdlib-oracle`** — coherent; the unit-definedness plane
  disambiguator matches `28M` §9's contract and the divergence cell stays pinned by
  `floor28-command-v-reads-fn-definedness`.
- **The softened incarnation definition** — the correlation door (`res-incarnation-
  correlation-door`) is honestly held open with both poles named; the "unconditional
  destroy-recreate can never converge downstream" motivation is correct as stated.

## Suggested disposition

Stage-0: done, keep. Stage-i: do not dispatch on the current §1 text — it needs the F1
ruling (one sentence, conservative option available) and F5's commissioned-fixtures
rider first. Stage-ii: needs F2's missing pin added to §9 and the closure-membership
question answered before the `DefinitionCustody` re-key means anything. F3 wants a
typed human line (either re-affirm the keep-verdict knowing it's now a ceremony, or
fold it into the spike-end instrument explicitly). Stage-iii/§10: carry F4 into the
sitting's agenda — cite 24T next to the ssh entry-form item, and write the
ambient-host asymmetry down before "uniform" hardens.
