# 22D — the why-lens end-to-end (corrected arch-2, merged with the ui-A why-slice)

> Round-22 conductor, 2026-06-14. SUPERSEDES 22C's §2/§5 (the gate-obligation +
> mvs-D are VOID per the human ruling: the type system enforces the weld;
> structural leaks are unrepresentable; the x-1 "vacuous gate" is a red herring —
> 224 §10). 22C's design survey (cribbing rustc/Elm, the render model) and 22B
> still stand for the DESIGN; this note is the CORRECTED BUILD CONTRACT and the
> dispatch basis. Human-approved (2026-06-14): arch-2 and the ui-A why-slice have
> become one body of work — build the why-lens END-TO-END, consumer-first, because
> every remaining piece of "arch-2" only pays off through the why-UI that shows
> it, and building it speculatively ahead of its only consumer is the
> design-the-slot-before-a-consumer anti-pattern. AI-authored; +SURE/~SUSPECT/
> -GUESS/--WONDER.

## §0 What this arc IS (and is NOT)

IS: the first real receipt-READER, built for its OWN user-facing value — the
per-line "why did this command run (never elided)?" disclosure on the plan-render
surface (`dir-soundiness-ux`: frontload the unsoundness where the human reads,
at-decision-point). A user runs the plan render on a book with a ⊤-forced command
and SEES "line N runs because `$(…)` is unknowable [author-oracle]," with the
cause wired from the real arena, on the render surface, never embedded in the
byte-floored artifact.

IS NOT: a gate-de-vacuuming exercise (DROPPED — the erasability test stays the
cheap type-backstop it already is; NO durable-negative-control, NO leaky/forked
engine). NOT the full ui-A (all invocation modes — that is later/separate); this
is the why-SLICE only. NOT the postmortem why (that is arch-4, reading a durable
tape; this is the LIVE why, reading the live arena).

## §1 The build, in stages (granular commits; each harvestable)

- **stage-1 — cause-wiring (the corrected mvs-A; the subtle/risky foundation).**
  Wire `CmdsubOperandTop`'s `cause: Option<ProvId>` to the REAL minted cause.
  CORRECTED ORDERING (the XC-2 fd-A dispatch-blocker): the ⊤-cause is minted in
  `mint_top_causes`, which runs AFTER the effects pass (a node's opaqueness is the
  effects pass's OUTPUT — the ordering is inherent; `effect.rs:120-125`,
  819-842). So the TYPED diag carrying the cause must be assembled/finalized
  AFTER `mint_top_causes`, reading `top_causes[node]` — NOT minted at the
  kernel-early `command_effect` emit site (and DO NOT thread `&mut arena` into the
  effects pass — that risks `solve`'s pure-`Fn` determinism posture). Cleanest
  mechanism is the builder's call (post-mint assembly of the cmdsub diags; or
  keep the typed diags un-lowered until a post-mint cause-backfill, since the
  legacy `Diagnostic` drops the cause in `to_legacy`). Constraint to honor:
  effects pass stays a pure function; the arena/cause work lives at the `classify`
  level where the arena already is. Wrinkle (state it, don't fight it): the
  per-node cause is keyed on the CFG node's span (the whole command), so the cause
  is "this COMMAND went ⊤," not operand-level — correct for a why-lens ("why did
  this command run"); 22B worked-3's operand-level pairing is aspirational, not
  required.
- **stage-2 — the why-lens render (the consumer).** A function reading a diag's
  cause + the arena → the per-line "why" explanation, remediation-classed.
  `render_cli` already exists (arch-3); EXTEND it (or add `why(site)->Explanation`
  beside it) to surface the cause-derived "ran because <cause>; <remediation
  hint>." Minimal-witness-first (228): show the cause-site once, the smallest
  honest explanation.
- **stage-3 — the minimal CLI disclosure (the ui-A why-slice).** Make it
  USER-VISIBLE: the plan render shows, per forced-run (non-elided) command, the
  why-lens disclosure inline (at-decision-point, `dir-soundiness-ux`). Extend the
  existing CLI plan render; do NOT build a new `dorc plan`/`dorc why` subcommand
  this arc (fork-surface disposition). rec-1 WELD: the disclosure renders on the
  plan-render SURFACE, OVERLAID, never in the byte-floored `.sh` artifact
  (`render_artifact_comment` stays fact-plane).
- **stage-4 — suppression scoping (mvs-C, only what the why-lens needs).**
  emit-at-origin (the ⊤-origin carries the cause; pure-propagation consumers
  inherit silently — f-3b is already this shape). Dedup in RENDERING (the why-lens
  shows the cause-site once across N poisoned consumers), never destroyed at
  capture. Do NOT over-build the full mvs-1..5 subsystem — scope to the dedup the
  render actually exercises; flag anything that wants more.

## §2 The welds this arc MUST honor

- **ru-11 one-way (load-bearing).** The why-lens EXPLAINS, never DECIDES. It reads
  the cause to render text; no disposition/artifact/Error-diag may key on receipt
  content. (The types already make the structural form of this unrepresentable —
  ProvId !Ord, cause out of `Reach`'s Eq; the discipline is: don't write a
  type-valid `if cause.is_some()` into a decision path. If a suppression tie-break
  ever wants to order dispositions by which-cause-won, STOP and flag — that is the
  one forward weld-breach hazard.)
- **rec-1 two-surfaces.** Render surface (plan presentation / CLI) carries the why;
  the byte-floored `.sh` artifact never does.
- **emit-at-origin (concl-5).** Cause minted at the ⊤-origin node; consumers
  inherit silently.
- **dir-soundiness-ux.** Per-line, at-decision-point; blame lands on the specific
  cause; apply-side education (the why-lens teaches where the forced-run came
  from). Two users: remediation-class already splits AuthorOracle (engineer) /
  FixBookLine (admin) / Structural (Dorc's fault) — keep that honest.
- **inv-determinism / inv-no-throw / kFAIL-perform.** Ordered collections only;
  the why-lens returns data, never panics; a ⊤ runs (the why-lens explains the
  run, never licenses a skip).

## §3 Scope cuts (conductor fork dispositions; human delegated "as you see fit")

- **origins: `CmdsubOperandTop` ONLY** this arc (the worked example; de-risk).
  `RedirTargetTop` DEFERRED — it has NO cause field at HEAD (payload `{site}`
  only, fd-F), so wiring it is a separate payload change, not a ride-along; do it
  later if the consumer wants it.
- **surface: inline plan-render + the `why(site)` primitive**; defer a standalone
  `dorc why` subcommand.
- **`ValueOf::Top` cause: DEFER** (fork-valuetop) — `Reach::Top` cause is what the
  why-lens reads; the value-plane ⊤ is a second plane for a later sweep.
- **remediation-class per code: builder PROPOSES, conductor disposes at harvest**
  (fork-remediation) — a small column like the floor column.
- **gate-obligation: DROPPED.** Erasability test untouched. x-1 coverage-doc test
  (`b68fc66`): now MOOT (it documented a vacuity ruled a red herring) — abandon,
  do not fold.
- **reliability quadrant (fd-G): the why-lens covers the reliable-oracle value-⊤
  case only.** The oracle-lifter give-up codes carry no cause + `site()==None`, so
  the why-lens reads nothing for them — state this honestly in the render (those
  codes already render their own message; the why-lens is additive for caused
  ⊤s), do NOT overclaim "every forced-run has a why."

## §4 Dispatch shape

ONE Opus builder, fb-19-clamped, conductor-created worktree at a verified base
(current ai/spike3 tip). Brief = §1 (staged build, stage-1 ordering called out as
the must-get-right foundation) + §2 (welds) + §3 (scope). Granular commits per
stage; stage-1 (cause-wiring) lands + verifies green FIRST (it is the risky
restructure — isolate it). Acceptance: a book with a top-level `$(…)`-forced
command, run through the plan render, shows the per-line why with a real
arena-sourced cause, on the render surface, artifact byte-unchanged
(receipt-free). NO BLESS. Full gate chain per commit. Crosscheck after harvest:
x-2 (over-suppression — does the render-dedup hide a second independent cause?),
which tests stage-4 where it is real (wants fr-2 first; Opus skill-pair).
