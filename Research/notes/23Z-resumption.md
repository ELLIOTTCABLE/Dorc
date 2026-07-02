# 23Z — r23 resumption / keep-alive (LIVING document — for the next conductor)

> **Living doc** (human instruction, 2026-07-01; cadence refined same day): update *judiciously*
> — direction-changes, discoveries, refutations, deferments; NOT per-turn conversational
> back-and-forth while the human firms up understanding. The bar: state must never be lost to a
> context-compression. It is the one sanctioned exception to the append-only-notes discipline. AI-authored; trust the
> root docs and the human-written `plans/233` over this where they conflict.

## Where the round is (2026-07-01): the oracle-contract design hole, and the ternary reshape

r23's gradient arc was interrupted by a design crisis, logged by the human in **`plans/233`**
(human-written, an exception under plans/): the oracle effect/poison contract is broken. With a
*binary* per-site verdict ({elide, run}, decided statically at plan-time), an oracle's *silence*
about a state-cell must mean either "trusted untouched" (unsound — a partial oracle becomes worse
than no oracle) or "poisoned" (safe — but then realistic books elide nothing). 233's four
approaches exhaust that axis; none is acceptable. This is the frame problem (non-effects can't be
enumerated; the round-9 walls note `093` named it), and no fifth trust-default escapes it. A
4-agent adversarial crosscheck agreed (**`notes/234`**, adjudicated; it also carries a
code-verified baseline correction: the spike poisons-all for *unmodeled* commands — the danger
lives in modeled-but-narrow oracles, not in a "dangerous floor").

**`233` is STAMPED (human, 2026-07-01) — do not edit it further**; work its consequences in
notes/conversation instead. Its frontloaded framing is binding: the proposal IS the trade —
*give up the attention-product to save the performance-product, wherever the world is
undescribed*; nothing in the guard tier repairs the attention-loss, and only the (open, may-be-no)
fork could buy it back.

The current working reshape (this session's design conversation, synthesized into **`233`
§"Update, 2026-07-01"** — that section is the ON-RAMP for the current state; read it first):
widen the verdict to ternary **{elide, guard, run}**. A `guard` = the oracle's own read-only
convergence predicate compiled *in-sequence* into the apply artifact (`check || original-cmd`).
An in-sequence check is frame-free — it consults the host *after* whatever interposed opaques
actually did — so its license is purely local self-knowledge, zero cross-oracle vocabulary.
Consequences: silence stops meaning anything (it merely fails to upgrade guard→elide); the safe
default becomes cheap (one host-local read instead of run-in-full); monotonicity restores (a
partial oracle helps its own sites, endangers nobody); enumeration/completeness-vouches demote to
opt-in upgrades gating the *elide* tier only. The human's own compression of the whole move
(2026-07-01, use it): we are pushing the given-but-incomplete fact-base *down into the apply
phase*, to heal from its own incompleteness.

The human's load-bearing correction to the first draft of that idea: real books keep a permanent
opaque tail (his priors: ~10% of commands, in ~90% of books), so the static-elide region ("before
the first *opaque*", not first mutator) is small, ~always — **guards carry nearly all realized
skip-value**, and the *attention* product (lines literally removed from the plan) is NOT bought by
guards. A 100-command book with one unmodeled third command is a ~97-guard artifact forever.

Prior art that MUST be read before designing anything here: **`notes/218a`** — round-21 already
designed this mechanism ("door-4"/GuardInsert, errexit-motivated): the `||`-form errexit
exemption, preamble-function shipping, already-guarded refusal (never double-guard an admin's
hand-written guard), the claim-noop conflation (`dpkg -s nginx` passes while `apt-get install
nginx` would still *upgrade* an outdated package), mint-policies m-a/b/c. It was flag-gated and
human-ruled build-last / product-hard-defers; the reshape *reverses* that posture — a deliberate
re-weld to be made consciously (parking-lot #5), never drifted into. Also **`notes/232`**
(channel-nativeness split; the body-redirect stdout-vouch is now a guard-eligibility enabler) and
**`notes/231`** (walk-back map; its 1b/1d/1e items survive, re-anchored).

## The fork — RESOLVED-IN-DIRECTION (2026-07-01); ACTIVE = crosscheck dispatch (awaiting say-so)

Fork outcome (human, typed): oracle-side global claims stay DEAD in every form (truth-testimony
and blanket-vouch alike — the vacuous-universal refutation covers both). Admin-side
consent-priced *attention*-product: **gently NO** — the human's psychological argument: the act
of typing the consent ("abandon_hope_all_ye…") itself taps an exhaustible trust-well and poisons
the relax-into-safety that the attention-product sells. So in normal/reconcile modes the product
statement stands: *Dorc narrows your attention only where the world is described; elsewhere it
makes your book fast and safe, but not shorter* — display-compression is the only
attention-story past opaques. Residual: the ESCAPE-HATCH TAXONOMY,
synthesized in **`notes/235`** — read it; it CORRECTS an AI misread that briefly lived here as a
"`# yolo` skip-consent" (the human never proposed skipping; commenting-out exists). The corrected
taxonomy: **hatch-isolate** (main-mode, per-call-site-every-time, book-exclusive, EXECUTED but
poison-suppressed — the germane one, ON-FENCE, consent-to-trust-a-RUNNING-command) ·
**hatch-bump-exclude** (bump-mode-only dependency-exclusion; temporary-but-committable) ·
**hatch-dont-run** (dismissed). Parked as task #10; ruled additive; task #3 carries the
disclosure-tier reserve (elisions RIDING an isolation-consent never render like proven).

**Crosscheck (the active item):** timing endorsed by conductor — stamped-233 + pre-build +
fork-open makes this the optimal moment; iterating further would prime agent-3. Human's
structure: THREE agents — (1) neutral/disowned; (2) adversarial-owned, kill-direction ("the plan
is dead, these idiots think they've found a way around it"); (3) adversarial-owned,
save-the-attention direction ("too pessimistic; a way to save the attention-featureset exists").
Prep recs (conductor, in-conversation): context = stamped-233 + corpus MINUS all 23x-notes, no
sight of the conversation, siblings firewalled; agent-3 gets a copy with the fork-paragraph
neutralized to one line (de-prime its search space); known-open items listed in briefs as
EXCLUSIONS (never inclusions); expect re-found downsides dressed as kills — de-bias at
adjudication like `234` did; agent-3's brief must distinguish design-search from product-
apologetics. The conductor's consent-reframe and the entire `235` escape-hatch taxonomy stay OUT of every
brief and artifact-copy — independent convergence on any of it is data. NEVER read
`Research/notes/quarantine-DO-NOT-READ/`.

**DISPATCHED 2026-07-01:** three background Fable-tier agents, prompts DICTATED BY THE HUMAN
verbatim (spelling-fixes only: H2SaLS, measurement, acceptable, "if you need",
counter-arguments, conceived); the adversarial-crosscheck skill was invoked for process-shape
only (its prompt-writing guidance explicitly overridden by the human). Durable outputs land in
`Research/notes/236a-*` (neutral/disowned) / `236b-*` (adversarial: solution-too-pessimistic,
find-better) / `236c-*` (adversarial: solution-too-optimistic, kill-direction) + each agent's
final message. Contamination cleanups pre-dispatch: `.tmp-234-crosscheck/` DELETED (human order;
was flagged relocate-or-bin in 234); a stray root file containing the human's full
prompt-dictation message relocated to the session scratchpad (`stray-prompt-draft-relocated.md`)
— return it to the human if wanted.

**COMPLETE 2026-07-02:** all three returned (`236a`/`236b`/`236c` notes in Research/notes);
conductor adjudication = **`notes/237`** (convergence-ledger, collision-ledger, discount-ledger,
recommendations — word-slugs per the human's style ruling: `convergence-3`, never `C3`).
Headlines: mechanism validated 3-way; vouch/elide tier de-centered 3-way; cross-pair convergence
on RE-OBSERVATION — but read 237's §"Post-adjudication corrections": the human REFUTED its
attention-recovery claim (approval precedes application; contingency must be shown in full
either way), re-priced it to execution-shape + artifact-length with a wall-density caveat, and
re-framed it as a re-verification PLACEMENT SPECTRUM (per-site guard ↔ hoisted post-wall wave;
one mechanism, a dial), contained by single-approval + contingency-policy, no mid-apply
re-planning. Admin one-liner convergence = fence-sitting data for task #10.

**ADJUDICATION CALIBRATION (human, binding):** this is state-space exploration — agents are
"extremely unlikely to come up with a genuine kill-shot, nor a genuine new-direction." Be VERY
skeptical of all results; the exercise intentionally exercises sycophancy in both directions.
Historical failure-mode to avoid: conductors being "meta-sycophantic"/credulous toward
adversarial responses (treating hostile findings as automatically more honest). Surface findings
to the human only where they materially affect direction. Skill-process still applies:
convergence-across-passes = the trustworthy signal; adversarial-only findings =
suspect-until-checked; present passes side-by-side, human judges; no single-verdict collapse.

## Conversation-conduct fences (human feedback, this session — binding on any successor)

- Spell slugs out in conversation; explain first, cite parenthetically. Break complex things down
  slowly; the human actively flags density.
- Slug STYLE: word-based (`convergence-3`, `collision-1`), NEVER opaque letter-number ids
  (`C3`, `X1`, `w-7`). The drift pressure comes from reading the corpus's fixed artifacts —
  resist it; the human has corrected this repeatedly (AGENTS.md has the rule).
- Use `233`'s inline-annotation vocabulary in conversation and strawmen. The spike's
  `oracle_effect` marker rows are an open strawman the human says should NOT exist — never present
  them as the design (`232` §1 has the provenance).
- **HARD QUARANTINE: corpus / H2SaLS topics.** "Landmines that will kill this entire
  design-session" (human, verbatim). No sizing or measurement proposals routed through them; hard
  defer; do not ask why. (Also in auto-memory.)
- Keep THIS file updated proactively. Echo the TaskList to the human every round; it may not
  survive a context clear — the parking-lot below is the durable copy.
- The human left himself a TODO (his, not ours): model "converged" vs "noop" as distinct
  first-class concepts — gently deferred; don't focus on it unless it becomes load-bearing.
- Terminology RULED (human, 2026-07-01): **"convergence" = the state-you-want, with mutation
  tolerated as noise** — a converged system may still mutate; what makes it converged is that the
  mutation is irrelevant to its function. Never use "converged" to mean "re-run is a literal
  no-op"; the three-way is: mutation-known-but-not-cared-about (converged) / mutation-unknown
  (the residue no human can testify about) / no-mutation-legitimately (the degenerate class).
- Design principle banked (human, 2026-07-01): **tooling never rescues a contract** — "if
  something is only 'correct' when a future build-tool maximally guards against getting it
  wrong, you've just described it-being-incorrect." Argue correctness from the contract alone;
  tooling only shifts error-rates.

## Parking-lot (mirrors the session TaskList #1–#9; all pending, most gated on the fork)

1. **Guard-license rule — SETTLED 2026-07-01, folded into `233` §"The guard-license".** Read
   that subsection as authoritative. One-line: license = the explicit, sh-spelled
   **converged-vouch** (fallible judgment, claimed-tier, disclosed, mark-attributed; floor =
   hand-written-guard-idiom + attribution); universally-quantified "does nothing" licenses are
   REFUTED-dead (vacuous testimony — the hork/private-cache class); "per-verb" corrected to
   *per-reached-path* (engine vocabulary = control-flow + constprop only; vouch scope =
   reachability; predicate = the oracle's own check invoked with the site's argv — whole
   stripped body per the ORACLE GROUND-TRUTH block, the reached path's establish-probes
   defining what convergence means); fence = vouch
   licenses own-command sites only, never enters the fact-plane (witness-type + plane-absence);
   the fact-indirection-collapse hazard is named there. Intent-smuggling: human ruled
   attribute-don't-prevent (wide oracle design-space, attribution is the lever). Residual opens
   live in the 233 subsection: spelling (vouch-surface family); admin per-site force-run idiom.
2. **Guard-form gradient: apply-guard-fat ↔ apply-guard-thin (+ render obligation)** — a
   GRADIENT, not a fork (human correction 2026-07-01, encoded in `233`): fat = ship a
   (pared-down) check()-body as a function + call it (no new machinery; the DESIGN BASELINE);
   thin = partial-evaluate down to an inlined residue (aspirational ceiling; the single-command
   rendering is "dangerously simple-seeming" — most real guards have genuine control-flow, and
   whether any realistic oracle reaches thin is open). Design must work at fat; thinning is a
   progressive upgrade. Either way: guards must render as ONE uniform, visually-inert, foldable
   construct or the mindshare cost is total.
3. **Plan-surface contract** — display-compression ("1 change, 96 verify-no-op") over a fat
   artifact: acceptable substitute for artifact-shrinkage? A strict-on-divergence mode?
4. **Guards-can't-serve walkthrough** (slowly) — consumed stdout/cmdsub; admin-consumed rc
   (`&&`/`||`/`$?`/`if` — admin intent wins); run-delta verbs (oracle must be able to DECLINE a
   predicate — a state-guard on restart is the forbidden wrong-skip); loops/multi-operand.
   Interacts with `232`'s body-redirect vouch.
5. **Re-weld pass** — door-4's product-hard-defers reversal; TOCTOU-WONTFIX spirit (letter
   survives — guards defend no probe-fact, they decide fresh); `inv-probe-sourced-values`
   carve-out (a guard *reproduces nothing*; `218a` inv-g1 keeps values from crossing); every
   elision-is-probe-fact-licensed phrasing gains the third verdict. Probe-side welds untouched —
   state the fence: probe-shipping stays structural self-vouch regardless of any
   completeness/ACK claim.
6. **`234` triage** — demand/consumer-anchored poisoning (now an elide-tier burden-relief lever);
   story-2 via provide-equivalence + runtime-traced footprint; the probe-fence one-liner into
   233/DESIGN; whether the positive-frame salvage becomes the fork's spelling (same claim-family).
7. **xfail re-derivation** — the pre-crisis set re-anchors (vouch cases → elide-upgrade tier); new
   guard pins wanted (insert at converged-past-wall; fall through on divergence; no double-guard;
   never guard run-delta verbs). Gated on the fork + #1.
8. **Check-cost banding (PARKED)** — `kPROBING` governs (expensive checks earn a vouch or
   just-run); sizing has NO corpus route (quarantined); needs a sanctioned data source before
   un-parking.
9. **Density backlog** — slow walk-throughs on request: the self-reach/fixed-point argument (why
   an all-converged region licenses eliding itself, and why partial elision breaks it); the
   prefix rule's exact statement; `218a` mechanics vs what the reshape changes.
10. **The escape-hatches (PARKED, human on-fence)** — authoritative shape in **`notes/235`** §2:
   hatch-isolate (main-mode, executed-but-poison-suppressed, per-call-site-every-time,
   book-exclusive, strongest machinery — the only knowingly-introduced wrong-elision ever) ·
   hatch-bump-exclude (bump-only dep-exclusion) · hatch-dont-run (dismissed). Second book/oracle
   divergence; kOOB comment-spelling contact; kept out of 233 + crosscheck briefs.

## Post-crosscheck rulings (human, 2026-07-02)

- **adj-q1 COMMISSIONED** → task #11 (re-verification placement-spectrum design round; gates
  tasks #5/#7; full constraint-set in the task description, incl. the two below).
- **ATTENTION-CHRONOLOGY doctrine (named, binding):** user attention is cheap in the
  right-after-hitting-return epoch, expensive in the thirty-five-minutes-later epoch; part of
  Dorc's value-prop is spending CPU-heavy analyzer work to SHIFT the user's attention-work into
  the cheap epoch. Late attention-demands are possible but very expensive and narrow the
  value-add vs just-running-the-script. Consequence: no mid-apply prompts/stops ever ("how much
  it would suck to come back 20 minutes later and find Dorc had stopped after the first three
  commands" — and an *expected* stop only removes surprise, not the cost); all decisions
  front-load into the one approval; late events are report-items.
- **adj-q3 RESOLVED 2026-07-02 — the TWO-HALVES doctrine** (human; supersedes both the
  crosscheck's "de-center" and the conductor's "ceiling, not critical path"): true full elision
  is THE GOAL — the golden hill: the apt line commented out in the plan along with 90% of the
  book because it's true on the system — and the guard-half is its sister with EQUAL design
  attention (sooner value before quality oracles exist; permanent fallback for un-oracled
  tools/lazy days/un-oracled platforms). ANTI-CREEP RULE, binding on all future planning
  writing: no "someday/aspirational/hopefully" tier-language about the elision-goal; no
  guard-half decision quietly discards a constraint the elision-goal needs — flag tensions to
  the human. (History of the resolution: his four counter-arguments — ~80% built; the 90/10
  prior is an unfounded guess; cool-factor; the wish-E value-ceiling — plus 237's second
  correction: authored-static vouches stay dead for every command class, but probe-derived
  per-run footprints — apt-simulation closure + payload file-lists + host trigger/hook registry
  + stereotyped-maintainer-script recognition, ⊤ on residue — are constructible-in-principle.)
  FURTHER LEVELED by the human (same day): the arbitrary-payload property is NOT
  package-manager-specific — EVERY command hides a potential ocean of global state (`cp` can
  trip inotify handlers, FUSE, quotas); class-scoped solutions are no solution; the mechanism
  must be uniform for cp and apt alike. Conductor's uniform reframe (~SUSPECT, seed for the
  vouch-ceiling design, needs its own adversarial pass): (1) ALL effect-claims are
  HORIZON-BOUNDED — claims cover first-order tool-contract effects only; host-configured
  reactions (watchers/triggers/hooks) are a named, uniform residue class no per-command claim
  covers (the kVOLATILES move again: name the exclusion as contract, don't model it); an
  un-horizoned authored universal was ALWAYS unsignable, for cp too — the horizon is what makes
  any authored claim honest. (2) Claims may be DERIVED-AT-PROBE-TIME, one mechanism with an
  effort/depth gradient: cp derives trivially (argv→path), apt elaborately (simulate→pin→read
  payload), curl|sh not at all (⊤, wall) — mechanism-uniform, value-graded. (3) The residue is
  handled ONCE, globally: host reaction-registries are themselves probe-able host-state
  (host-level oracles), or a single disclosed exclusion — never per-command testimony.
  Authored-universal-without-horizon stays dead for every command class. Note the kill-agent's own accounting cuts FOR
  the ceiling: without elide-ambitions the fact-plane is over-built and Dorc converges to
  "guard-compiler with hint-probe."

## BACK-TO-EARTH PLAN (acked by human 2026-07-02; the current spine)

1. DONE — ceiling stamped: **`notes/238`** (horizon-bounded claims, derivation-gradient, the
   composed GFY boundary, pinning TABLED with its fail-loud cousin, mountain-as-requirements-
   catalog, the five-claims decomposition). Seed for the parked vouch-ceiling round; needs its
   own adversarial pass when un-parked.
2. DONE — crisis-closure package: **`notes/239`** — human ruled **GO 2026-07-02** ("239 looks
   good to me"). Deltas 1–3 APPLIED to spike/CLAUDE.md (the round-23 rulings block:
   rul-ternary-verdict / rul-guard-license / rul-attention-honesty / rul-divergence-proceed;
   the TOCTOU identified-cause clarifier; the inv-probe-sourced-values guard carve-out);
   delta-4 APPLIED to KNOBS (kELISION second naming-caution, with the 233/239 pointers);
   delta-5 deferred (human, DESIGN's rewrite is his); delta-6 was already applied (233's
   end-annotation). **THE CRISIS IS FORMALLY CLOSED; the build resumes.**
3. IN FLIGHT — guard-tier xfail set (task #7's unblocked half): human directed a FABLE-class
   subagent for the derivation (tier explicitly authorized — "the written specification is
   important enough"), prompt in the human's 236-style (general, goals-focused, no
   over-instruction), notes-slug `Research/notes/23A-*`; prompt drafted, AWAITING HUMAN REVIEW
   before dispatch. Then THE ONE PLANNED CROSSCHECK of this sequence: a neutral+adversarial
   PAIR over the pin-set ("find the pin that licenses a wrong-elision; find the licensing hole
   no pin covers"). Crosscheck applies NOWHERE else in the sequence (human asked; conductor's per-step
   reasoning 2026-07-02: ceiling-note = non-binding, closure-deltas = human hand-applies each,
   build = DST/gates/harness are the instrument). Task #11 keeps its own baked-in crosscheck.
4. Then — build the slice: round-21 guard mechanics at the fat pole, behind the existing flag,
   default flipped only after the delta-1 re-weld is signed. Task #11's cost-model memo proceeds
   as parallel desk-work.

Also memory'd this session: the Fable firewall-breaking tendency (three instances; lead with the
breach, price containment first, offer the non-breaching cousin, expect tabling).

**ORACLE GROUND-TRUTH (human, 2026-07-02 — overrides stale spike-layer framings; bind on all
future writing):** (1) oracles are JUST SH, often in the same file as the book; the added syntax
is STRIP-ONLY — the strip removes type-annotations and rewrites `name.check()` → `name_check()`,
nothing else, and its output is runnable sh (period-names = a semaphore opting into extra
Dorc lint/warnings, NOT a different language). (2) The argparse-deconstruction is an ANALYZER
TRICK, not a language constraint: an oracle may contain arbitrary sh — `rm -rf /` ships and
wipes root; oracles are constrained in what we ASSUME from them, never in what they contain.
(3) Therefore the check IS the oracle, and the stripped whole body is what ships in BOTH lanes
(probe under structural self-vouch; apply as guard); lifting an invocation-relevant subset
(what old rounds dangerously called "verbs") is an optional edge-case gated on the author's sh
matching the abstract-interpretation constraints, and any lifted form must be BYTE-IDENTICAL TO
A SUBSTRING of the oracle body — maybe not even worth building ("maybe just ship the entire
oracle during both probe and apply", human, unsure). The spike's st-2 check/probe split is
spike-INTERNAL implementation, not design truth — a build-vs-design divergence to reconcile
(alongside the existing inv-one-observable text-vs-code flag). Agreements preserved through the
uncrossing: never engine-synthesized guards; never declared output in guard-position (declared/
probed output is used ONLY in the inverse, full-elision stand-in case, per
inv-probe-sourced-values).

**Step-2 review feedback (human, 2026-07-02 — pre-GO; all applied):** rul-ternary-verdict's
supersession language corrected (signing 239 IS the door-4-deferral reversal — no prior decision
existed) and its sourcing principle REWRITTEN per the oracle ground-truth above (whole stripped
oracle body, both lanes; substring rule for lifted forms; two nevers).
delta-2 rephrased: the line is IDENTIFIED-CAUSE vs OPEN-WORLD drift ("TOCTOU stays out,
hork-catching is in"). delta-4: KNOBS is conductor-editable-WITH-REVIEW this session (human
ruling — not human-write-only; the kELISION note must slug-point to plans/233). delta-5
DEFERRED (human away from machine; NB: he says DESIGN needs a thorough rewrite eventually and
IMPLEMENTATION is ¼-written — human-owned, low-spoons, do not nag). delta-6 APPLIED: the
sanctioned `<!-- /* … */ -->` frozen-doc annotation (precedent 231 §4) now sits at 233's end
with four reader-must-honor corrections. NEW candidate invariant, flagged not welded:
**one-body-two-lanes** (byte-exact probe/guard bodies; probe runs exercise guard code; tensions
with apply-guard-thin; routed to task #2). AWAITING FINAL HUMAN ACK before step 3 (xfail
derivation) begins.

## Task-3 rulings (human, 2026-07-02 — the plan-surface contract; task #3 CLOSED)

- **The plan is the code; the code is the plan.** Original vision was `dorc plan >theplan.sh` — an
  *editable file* of what actually needs running. Even as the UX grows richer (TUI/CLI), the
  spirit binds: the plan-render is the WHOLE runbook, original order (order sacred in display
  too), line-numbered, syntax-highlighted; **elided lines stay present but greyed** (in file form:
  comment-lines, consistent with the r22 byte-floored-artifact ruling); **guards appear inline as
  real code** with postfixed reason-comments (`hork nginx  # unmodeled`); a predicted effort/time
  summary at the end. NO tier-sorted report views.
- **Attention is saved ONLY by provable elision — WELDED 2026-07-02** (upgraded from hard-lean;
  human, verbatim: "consciously rejected and welded. my tool may be 'scrappy', but it is
  *correct*. I will not hide risk from the user." — a candidate DESIGN.md line, his to place): no fold-by-default,
  no hiding lines that will execute on the user's server. The guards exist because we're in a
  dangerous position; hiding that from the user is exactly wrong — in ops the edge-cases DESERVE
  attention, and a young tool mutating users' control-flow is the last thing to hide. TUI dimming
  of guards: maybe, warily. (COLLISION pending adjudication: 236a's headline redirect recommends
  render-compression AS the attention-product — the human's fresh typed doctrine contradicts it;
  present the collision, don't silently drop either.)
- **Divergence policy: proceed-and-flag, welded-in-direction; NO strict/abort mode.** Doctrine:
  guards shipped to apply must be COMPLETE as far as the oracle is concerned — detecting/surfacing
  broken or uninterpretable world-state is the ORACLE's job (defensively, on its own channels),
  never a second-guess layer of the engine's. Un-flagged divergence = "diverged and fixable";
  the user chose `apply` over `plan` = intent to fix. "Oracle says fixable ∪ the command knows how
  ∪ the guard knows it's needed ∪ the user said go ⟹ just goddamn fix it." Mechanically: guard
  falls through → mutator runs → the mutator's own failure is the natural loud stop (errexit);
  the apply-report leads with divergences-from-prediction.
- **Approval semantics: byte-identity holds for the plain-file FALLBACK path only** (softened by
  the human, same day): `plan >theplan.sh; edit; execute` stays a supported, deliberately
  plain-unix-y fallback where edits run as-is and re-analysis is a fresh plan. But Dorc is an
  orchestrator: multi-host specialization (20 hosts may need 20 different plans; nobody reviews
  20 .sh files) puts SOME "present → modify in-UI → re-analyze" loop on the table — how much is
  an OPEN question, deferred. The richer surface must match the fallback's *vibe*: present the
  script, multiplexed over hosts in some undetermined way, as plain line-numbered sh; "apply"
  applies the multiplexing into per-host actual executables. Candidate generalized invariant
  (conductor, unconfirmed): each per-host executable is a deterministic, inspectable function of
  (the reviewed surface × that host's facts); any edit yields a fresh reviewable surface. The
  BINDING part is the vibe-doctrine either way: don't hide danger; don't buy attention where it
  can't be bought safely; spell and present things as plain sh.
- Conversation fence (temporary, phone sessions): explain any prior-art inline — the human cannot
  open files at will; do not lean on slugs.

## The pre-crisis xfail arc (retained; gated on the reshape ruling)

Charter §6 method (xfail-first → design → adversarial-crosscheck → build) and the harness contract
stand (`spike/e2e/run.sh`: a case = `book.sh` + `*.oracle.sh` + `probe-results.txt` through 8
gates; an `XFAIL` file pins desired behaviour; zero xfail at HEAD). The previously-designed set —
`pin-consumed-stdout-runs` · `xf-vouch-stdout-bodyredirect` · `pin-ortrue-toplift-recovers` ·
`xf-ortrue-widen` · `xf-andor-both-agree` · `pin-probe-safety-boolean` (the dc-probe-NOT red-XPASS
tripwire; must-stay-boolean) · `xf-multicell-elide` · `xf-partial-member-elide` · the
`cardinality` pair · `xf-vouch-effectcell` (spelling-gated) · `xf-disagreement-prefer-probe`
(riskiest rung; design adversarially before pinning) — survives mechanically but re-anchors under
the ternary (task #7). Pending tc-flags from `231` §5 / `232` §8 remain surfaced-not-resolved:
`tc-vouch-surface`, `tc-disagreement-rung`, `tc-cardinality-strong-update-rung`,
`tc-multicell-aggregate-grain`, `tc-partial-member-self-reach`, `tc-one-observable-build-vs-spec`,
and the `unseeded-hunt` recovery (7 of 8 candidates lost to a size-cap; re-run smaller if wanted).

## Method reminder

Design → **adversarial-crosscheck the design** (exclusions-not-inclusions framing; structured
neutral + disowned-adversarial pair; convergence is the signal, lone findings suspect) → build,
against the welds: `inv-kfail` (a gradient adds precision only toward run), `ru-11`
(decision-taint is a separate cell, never the receipts), probe-safety stays boolean (no
confidence threshold ever ships a probe). NB Fable-tier is available again as of 2026-07-01 (this
session ran on it) — the ru-24 ask-first gate re-applies to any Fable-tier agent dispatch.

## Pointers

human problem-log + reshape synthesis **`plans/233`** (§"Update, 2026-07-01" = the on-ramp) ·
crosscheck `notes/234` · door-4 prior art `notes/218a` · r23 arc `plans/230` → `notes/231` →
`notes/232` · welded invariants `spike/CLAUDE.md` · build reality `plans/16P` §3 + `16Q` + the
spike-3 closes `20K`/`21W`/`22W` · harness `spike/e2e/run.sh`.
