# 24A — round-24 Stage 1: session rulings, the 231 disposal, and the honest-baseline evidence

AI-authored (Fable conductor), 2026-07-03, round 24, **LIVE WORKING NOTE — IN PROGRESS**
(marked frozen at stage close; the 23M precedent). Charter: `plans/240`. This note is the
durable home for (a) the human's typed in-chat rulings at round-24 open, (b) the 231
disposal paragraph carried from the 23J repair-pass routing, and (c) Stage-1 build
evidence as it lands. Confidence-marked per the standing discipline.

## §1. Human rulings at round-24 open (typed in-chat, 2026-07-03; recorded per silence-is-not-ack)

- **rul24-wall-placement (ratifies the Stage-1b design).** Verdict-aware walls computed at
  plan time are correct, and not merely expedient: "re-ordering and re-verdicting is valid
  *right up until the moment of consent*" — the eventual design actively wants probe-results
  driving continuous, visible, in-UI plan-updates as they stream from multiple hosts. The
  wall *absolutely* depends on a probe-verdict. The first-order solution to
  allowing-stuff-after-yourself-to-elide is always "just elide *yourself*"; this round's
  charter is the *next*-order solution — how to let downstream elide when you yourself are
  diverged and must-run (Stage 2's footprint × backing × disjointness).
- **rul24-churn-on-green.** Golden churn from the fd10 flip is licensed, considered low-risk
  ("we're actively destroying and rewriting all that machinery"); execution delegated to
  Opus-tier builders.
- **rul24-conductor-role.** The conductor's deliverable #1 is "what strains"; #2 is
  really-clever briefs ∪ skeptical review of finished work against cross-cutting
  invariants. Opus agents are trusted for *local* correctness; they trip on subtle
  cross-cutting concerns — that is the conductor's review domain. High bar for
  conductor-written code. (Also: Opus builders MAY spawn sonnet subagents for mechanical
  sweeps; the no-subagents clamp propagates one tier down — each spawned subagent must be
  told not to spawn further.)
- **rul24-spelling-style (binding on the Stage-2 footprint strawman and all new spellings).**
  90% of solutions should work very hard to find *native sh spellings people already use*;
  if spelling-as-sh must break (as the typesystem did), break it *loudly* with first-class,
  high-quality language-design constructs. The middle ground is the worst approach.
  Strawman-tier stubs are exempt (explicitly NOT-DESIGN), the settled spelling is not.
- **rul24-yardstick-is-ui.** The plan-summary surface is a *feature strawman of the CLI
  pole* (the planned TUI/CLI interface split: TUI = ANSI + live-updating + live-editing;
  CLI = dirt-simple, flags, multiple invocations, stdin/stdout). Design the mock CLI, then
  test through it; never build the CLI *to* e2e-test it (that inverts the e2e into a janky
  indirected integration test). Low-importance to get exactly right; don't churn.
- **rul24-acceptance-shape (prospective, NOT a gate).** If the round goes to plan, the
  final "teaching" plausibly includes the human personally pointing an Opus at his real
  dotfiles — where idempotence sucks to establish, what needs oracles, first stdlib oracles
  for macOS idioms. Absent that, there is effectively no written acceptance-goal; the round
  is hard to "acceptance" in a vacuum. Bias the build toward
  actually-usable-at-the-CLI-in-anger.
- **Process fact (harness, recorded in conductor memory too):** subagent worktree isolation
  bases new worktrees on an ancient `main`, not the session branch — every builder brief
  now opens with an explicit step-zero `git reset --hard ai/spike3-r23` + tip-hash verify.

## §2. The 231 disposal (carried from the 23J repair-pass routing: map the pre-crisis sweep's clusters to survivor/dead-with-reason)

Context: `notes/231` is the *pre-crisis* collapsed-gradient sweep — round 23 §1 work,
intercepted by the `plans/233` crisis before any of it was acted on. The crisis rebuilt the
verdict architecture under it (ternary verdict; silence=wall; vouch never enters the
fact-plane; the atomic-command axiom). This disposal says what a future reader may still
build on. Per-cluster, against the post-`23O` settled law:

- **1a `decision-plane-trust-cell` — DEAD as designed; superseded in shape.** The crisis
  built the claimed-vs-proven distinction into the *verdict tier* instead of a per-value
  trust tag: fact-tier probe-observations license elide; judgment-tier converged-vouches
  license guard only and NEVER enter the fact-plane (rul-guard-license; rul-ternary-verdict's
  two nevers). A source-tagged fact-plane trust cell would now *duplicate* that architecture.
  What survives of 1a is its reporting half (why-lens provenance, decision-inert under
  ru-11 — unchanged) and the eventual "licensing tier / cross-site blast attribution" agenda
  (`23M` open-agenda), which is Stage-2+ work under the new framing. Do not resurrect 1a's
  shape.
- **1b `cardinality-strong-update` — SURVIVES-DEFERRED, re-keyed under Stage-5
  entity-identity.** The finding (no strong update exists; the uniqueness bit is absent,
  not boolean; `Kill` accumulates) is untouched by the crisis and still true. Its natural
  re-entry point is the round-24 Stage-5 synonym/aliasing cell (`23N` §6:
  must-not-alias-or-wall; dynamic points-to), where within-kind entity-identity becomes
  load-bearing. The 231 fence stands, now reinforced by silence=wall: a "probably unique"
  verdict may only DEMOTE toward weak/⊤, never PROMOTE to strong.
- **1c `coverage-vouch-default` — SURVIVES in principle; still spelling-blocked; not
  round-24 work.** The channel-completeness vouch is a species of the crisis's sanctioned
  shape (opt-in, author-explicit, judgment-tier claim; silence stays ⊤⇒run — exactly the
  anti-233 posture). Its spelling remains `dq-kOOB`/`kTYANNOT`-gated and is now further
  entangled with the parked "is authoring a verdict-function partly the vouching act?"
  question (`23O` §5). Round-24 boundaries explicitly defer all vouch spelling; hold.
- **1d `multicell-establish-classify-cliff` — SURVIVES-LIVE; a yardstick-raiser.** Pure
  coverage/precision walk-back, orthogonal to the crisis, every piece built except the
  classify match. Becomes interesting exactly when the strawman family shows it binding
  (a multi-cell verb refusing to elide with all cells converged). Candidate rider for
  Stage 6 tuning; all-or-nothing aggregate stays the safe default (tc-multicell-aggregate-grain).
- **1e `partial-member-convergence` — SPLIT by the atomic-command axiom.** The
  across-ENTITIES half (partial elision of one multi-operand line, `install nginx curl jq`)
  is now DEAD — welded shut, not deferred: the atomic-command axiom (`23D` §3, `23O` §2)
  makes multi-operand lines whole-line; granularity comes from author-written loops. The
  across-MEMBERS half (loop members) SURVIVES-DEFERRED with 231's own hazard intact: the
  all-or-nothing license is a fixed-point self-consistency argument, and partial-member
  elision requires a per-member `self_reach` re-derivation (a separate analysis, not a
  softened knob).
- **1f `door3-recovery-dormant` — SURVIVES-LIVE; cheap yardstick-raisers.** The fold's
  three dormant recoveries (andor both-operands-agree; `node_rc` line-recovery; door-3's
  bare-`true` narrowing) are monotone, inv-kfail-safe precision wins, untouched by the
  crisis. Candidates for Stage 6 when the yardstick shows fold-blocked strawman lines.
- **§2 (the must-stay-boolean fence) — SURVIVES-WELDED, strengthened.** The fence's spine
  ("a gradient may only DEMOTE toward run, never PROMOTE toward elide") is now also the
  crisis's own law; `fence-coverage-effect-completeness-NOT-mutation` ("you can never
  confidence-soften 'this might mutate something I didn't model'") *is* 233 restated. The
  round-24 Stage-1b fd10 fix implements the fence's spirit at plan time.
- **§3 (decision-surface THIN) — SURVIVES with a delta.** Still no trust/source tag keyed
  by any live decision. Two deltas since: the Stage-1b wall walk adds *upstream will-run*
  as a plan-time decision input (verdict-derived, not trust-derived), and the unbuilt
  GuardLicense witness (Stage 3) will be the first decision-edge keyed on judgment-tier
  material — by design, quarantined from the fact-plane.
- **§4 (channel-vouch / mention=vouch) — LETTER DEAD, SPIRIT CONFIRMED.** Every spelling
  §4 grounds in (`oracle_effect` rows, `oracle_probe_*`, `oracle_kind=`) was retired by the
  23H realignment (the marker fiction deleted; effects derive from predict bodies). The
  mechanism it defended — mention-as-vouch, absence=safe-⊤ — is now *more* true: writing a
  probe idiom inside the predict body IS the mention, and deriving `ValueClaim`s from it IS
  the vouch. 231 §4's inline correction pointer (→ `232`) stands; add this note on top.
- **§5 tc-flags — carried, with two changes.** `tc-disagreement-rung` survives for the
  merge/reporting half; its *licensing* half is settled by `23L`'s LICENSE-SOURCE ruling
  (ONE convergence source at vouched sites; fact-plane ambience is never a second
  license-source). `tc-one-observable-build-vs-spec` is PARTIALLY DISSOLVED by the
  realignment (predict() now IS the oracle; the build moved toward the slug's text);
  residual gap = per-channel Stdout/Stderr prediction still has no authored surface —
  re-examine only if it binds. The `unseeded-hunt` agent's 7 lost candidates stay lost;
  a re-run is moot post-crisis (the sweep's framing predates the ternary verdict) —
  recorded as an accepted loss.

## §3. Stage-1 build evidence — PENDING (builders in flight at time of writing)

Two Opus builders dispatched in isolated worktrees (fd10/silence=wall; yardstick + strawman
family). Their strain-ledgers, the honest-baseline yardstick reading, and the merge record
land here when reviewed.
