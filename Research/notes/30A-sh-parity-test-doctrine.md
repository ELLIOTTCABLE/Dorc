# 30A — The sh-parity test doctrine and the pin battery

> Tier: conductor-authored methodology spec (Fable, 2026-08-16), encoding three
> human-typed rulings from the morning sittings: `rul-unsure-falls-toward-sh-parity`
> (spike/CLAUDE.md; the load-bearing one), `rul-happy-path-is-a-closed-set`
> (spike/CLAUDE.md), and the testing mandate ("a lot more pins/tests against sh
> behaviour… TDD first… some will be FAILING tests with named greening dates; lean
> DST/unit over expensive e2e, except a small-ish set of differential posh/dash
> tests"). This note is BOTH the doctrine and the work order for
> `lane-sh-parity-pin-battery`. Authority: the human's typed rulings > this note;
> `spike/CLAUDE.md` invariants bind every test authored under it.

## §1 — The doctrine

- **d1-tdd-first-for-linguistic-behavior** — any engine work touching a core sh
  semantic (load-order, name binding, scoping, subshell lifetime, conditional
  definition) starts from tests stating sh's answer, written BEFORE or WITH the
  mechanism, in the paired form: an our-intent unit test (what we believe sh does,
  asserted against our model) beside a differential manifest (what posh∩dash
  actually do, measured once, committed as ground truth — the floor30 pattern).
  Not exhaustive; not a coverage shop; the pairs cover the behaviors that FORCE
  engine-implementation choices.
- **d2-tier-lean** — unit/DST first, always: unit tests live in the owning crate;
  world-dependent behavior goes through hostsim's seeded DST, not e2e. New e2e
  cells only where the property is genuinely whole-product (an artifact's rendered
  shape; a gate interaction). The differential family is deliberately SMALL and
  rides the opt-in floor lane (`mise run test:floor`, `DORC_E2E_FLOOR_SHELLS`),
  so the default suite's wall-clock does not grow with it.
- **d3-xfail-with-named-greening** — target behavior the engine does not yet
  implement is pinned as a FAILING test wearing its greening trigger, via a small
  unit-tier helper (build it in this lane, once, shared):
  `xfail_until(trigger, horizon, || { …assert TARGET… })` — the suite
  PASSES while the inner assertion fails, and goes LOUDLY RED (XPASS-to-promote,
  the e2e lens's semantics) the moment the target behavior arrives, so a pin can
  never green silently. `trigger` is a named lane/stage slug (semantic names, never
  bare stage-N); `horizon` is a ROUND marker, never a calendar date [human-nacked
  2026-08-16 — LLM date-estimates are worthless; the round cadence is the
  project's real clock]: `end-of-r30`, `r31`, or a stage-within-round
  (`r31:closure-custody`). ENFORCEMENT: the census seat carries a one-line
  `CURRENT_ROUND` const, bumped by conductors at round-open; a pin whose horizon
  round has PASSED goes red in the census until explicitly re-horizoned with a
  reason string (a reviewed edit) or greened — the "wait, why isn't this done"
  question, forced mechanically at every round boundary. A census test
  enumerates every live xfail pin with its trigger and horizon so
  `mise run test -- xfail_census` answers "what is owed, and by when" in one
  screen. Never express an xfail by asserting the WRONG behavior as if desired —
  the interim behavior may additionally be pinned, but only in a test whose name
  says interim.
- **d4-parity-over-shape** — when a pin author is unsure what to assert, the
  assertion is sh's behavior (run the strawman under dash if unsure what sh does),
  per `rul-unsure-falls-toward-sh-parity`. Deny-shaped assertions are legal only
  for named members of enumerable sets, and the test says which set.
- **d5-quality-is-a-ratchet** — output-idiomatic-quality is pinned mechanically,
  not by taste: assertions that the happy-path corpus emits ZERO munged names and
  ZERO defensive-mode artifacts, so any future regression into defensive-ugly
  output on clean books goes red instead of shipping quietly
  (`rul-happy-path-is-a-closed-set`'s executable half).

## §2 — The pin inventory (the battery; grouped by state)

**P-green — assert current-and-correct behavior now (unit/DST unless marked):**

- p-last-wins-helper-binding — plural helper across two oracle files, differing
  bytes: resolution names the last declaration; the custody composite gates the
  license (both suspend arms + the licensed singular cross-file reach).
- p-helper-unset-f — `unset -f _helper` at oracle top level: the name resolves to
  nothing after it; a body reaching it withholds/declines rather than borrowing an
  earlier declaration.
- p-subshell-helper-death — a helper defined in a book subshell dies at the paren
  (currently: never enters the index at all; assert the conservative outcome and
  cross-reference p-x-regional-helper below for the target).
- p-book-collision-forces-non-idiomatic — a book top-level funcdef sharing an
  engine-emitted name forces munge/scope for the ENGINE's definition; the book's
  own binding and its own later calls are untouched (two cells: book-defines-only
  vs book-defines-and-calls).
- p-defensive-forced-fallback — a real definition vector (`alias`,
  unresolvable load) munges every emitted name; a wrapper's command-position
  `"$@"` does NOT trigger it (extends the landed pin on
  `a-top-reject-is-not-a-definition-vector`).
- p-zero-munge-happy-corpus — the ratchet (d5): over every corpus case that is
  not deliberately a collision/defensive witness, emitted artifacts contain zero
  munged names; the witness set is an explicit allow-list, so growth is a
  reviewed act.
- p-blessed-toplevel-conditional — `command -v foo || foo() { … }` at oracle top
  level: the definition is MAY-bound; nothing licenses on it; the frame answer is
  can't-say (this pin lands GREEN only if current behavior already refuses —
  verify first; if the construct is currently rejected wholesale, pin the
  rejection as interim and add the target under P-x).

**P-x — xfail pins with named greening triggers:**

- p-x-definition-grade-keying — a file defining one role twice across frames
  (top-level + `unset -f` + redefine, one file) answers per-DEFINITION at each
  site. Trigger: the definition-grade keying repair (closure-custody's re-key;
  the `28Q` §1 item-1 tripwire's executable half).
- p-x-regional-helper — a helper redefined in a book region serves in-region
  sites with the regional body, post-region sites with the ambient one. Trigger:
  the funcenv table-widening (+ book-region indexing).
- p-x-intra-compound-plurality — a composed probe compound whose two participants
  need same-named, differing-bytes helpers gets per-segment environments
  (explicit per-segment subshells or alpha-rename — whichever
  `pin-emission-planner-universal` lands). Trigger: the emission planner. FIRST:
  measure and pin CURRENT behavior of a cross-custody plural helper under a
  composed predict ship (the ~SUSPECT license-gating gap from the morning
  sitting) as its own green pin, whatever it turns out to be — if it licenses,
  that is a FINDING to report before pinning.
- p-x-placement-tuning-pair — the A/B worlds from the morning sitting (top-lift +
  munge for the many-use helper; in-paren colocation for the once-used collider)
  render as their idiomatic forms. Trigger: the emission planner.
- p-x-blessed-toplevel-source — a `.`-sourced oracle file's definitions
  participate in resolution as sh would bind them. Trigger:
  `pin-oracle-side-sourcing-amendment` (`28Q` §4; since built and promoted).

**P-diff — SUPERSEDED BY INVENTORY (measured at the battery's checkpoint): all
three buildable cells were ALREADY covered by the existing ELEVEN committed floor
manifests** — `d-helper-binding-order` by `floor28-load-order-last-definition-wins`
+ `floor28-unset-f-and-redefinition`; `d-subshell-definition-death` by
`floor28-subshell-scoped-re-source` + `floor30-subshell-nesting-and-removal-scope`;
`d-conditional-definition` by `floor28-funcdef-as-or-operand` +
`floor28-define-if-absent-polyfill` + `floor28-command-v-reads-fn-definedness`
(which measures tool-present-vs-absent via PATH). The battery therefore minted
ZERO manifests, and the original ≤8 budget is retired as written-without-the-
inventory-in-view. `d-alpha-rename-equivalence` stays RESERVED in the xfail census
(a Reserved pin with a call site reddens). Standing rule unchanged: a future
manifest exists only where an engine choice depends on it, measured once through
`bless:floor`, never hand-computed.

## §3 — Placement, budget, and gates

Unit pins live in the owning crate's `src` tests or `tests/` per existing
convention; the xfail helper and census live where the first consumer is (share —
one named seat, no copies). Hostsim/DST for anything world-shaped. New e2e cells:
only p-x-placement-tuning-pair and the intra-compound witness plausibly need
whole-product form; everything else stays unit-tier. The suite's wall-clock is a
constraint: report the before/after `gate:full-quiet` timings per leg at the fold;
if the delta exceeds ~30s on either leg, say so and propose what moves to the
opt-in tier. Standard gates bind: both-legs green, `bless:dry` clean, comment
budget, granular commits.
