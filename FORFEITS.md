# Forfeited-value register (⊤-narrowing TODOs)

The living registry of *deliberately forfeited, known-capturable value*: every place the
design takes the easy conservative route — collapse to ⊤/unknown, withhold, collide,
wall — while knowing that more work could capture more known-correct value. Minted at
human direction (2026-08-13): the project has repeatedly bought tractability with
quiet value-forfeits, and each such purchase must be RECORDED with its capture path and
its back-out cost, so the easy route never silently ossifies into the design.

LLM-authored, human-reviewed in place; sibling of `ANALYZER-NEEDS.md` (what the engine
must know) and `AID-NEEDS.md` (how it talks to people) — this file is what the engine
*declines to capture, for now*. Discipline: ahistorical and current-state only; add a
row WHENEVER a conservative collapse is chosen for tractability (a brief that takes the
easy route without a row here is incomplete); a captured forfeit's row is rewritten or
removed, never annotated as history. SCOPE, sharpened [human-typed 2026-08-16]: a row
is a DESIGN-DISCUSSION opt-out — an option talked through and explicitly declined
("no, not now; too expensive or out-of-scope"), forfeiting a whole CATEGORY of value —
design-level, generally large. Sharpened again [human-typed 2026-08-21]: the ONE
category a row may hold is an ANALYSIS limitation — hard to implement, high-machinery —
that would specifically yield better or more-correct ELISIONS if we bothered to write
it. Not a grab bag of every punt, deferral, or someday: emission/placement/artifact
limitations, harness gaps, and prose debt are never rows here. NOT a tracker of
designed-and-planned work that simply is not built yet, and never implementation-level
nits: those live in the ledgers, the law files, and the xfail census. Entries carry: the RULE (what conservatism, ruled
where) · FORFEITS (who loses what, when) · CAPTURE (the known path to the value) ·
BACK-OUT (how retrofit-hostile waiting is) · REVISIT (the trigger).

- **forfeit-two-position-sparing-collide** — RULE (as-built through stage-i;
  `307c:dec-dialect-keeps-a-whole-unit-fold`): the sparing dialect stays ONE
  whole-unit minting fold per family (`dialect_minting_source`) — NO
  position-aware collide exists in code, and a frame-dead regional mint can
  therefore still sit in the global dialect a flag-gated meet consults
  (`308c` find-2's construction; census-invisible today). The ACKED floor
  (extremely-soft, human 2026-08-13: collide unless both positions agree on the
  backing family's closure and dialect) is CAPTURE-tier, not yet rule. FORFEITS:
  drifted-day survivals in plural-definition worlds under the risk flag once the
  floor lands — and until it lands, the exposure runs the OTHER way (a too-large
  dialect spares more), bounded by the census's enumerated blessed idioms.
  CAPTURE: position-aware dialect resolution with a per-token agreement proof;
  the `28T` sparing mini-model is the proof home, and it GATES promoting
  plural-idiom books beyond the census set. BACK-OUT: low mechanically; med
  socially (published claims calibrate against whichever floor ships). REVISIT:
  the mini-model's formalization; first field evidence of plural-idiom books.
- **forfeit-helper-plurality-withhold** — RULE (as-built under
  `rul-vouch-reaches-own-custody-only`): resolution stays sh's last-wins, and custody is
  now the closure — a file plus everything its top-level `.` lines pull in, transitively
  (`core::CustodyClosures`). What stays withheld is four shapes: (a) a resolved reach
  landing outside the voucher's closure, plurality irrelevant — CO-LOADING is ingestion
  and composes nothing (`emit30-cross-custody-plural-helper-suspends`); (b) the book
  reaching into a vouched composition, both arms — a hazard closed rather than value
  lost (these silently shipped before); (c) plural declarations of one name INSIDE one
  closure with differing bytes, because a flat load-order vector cannot express how a
  file's own declarations interleave with the ones its `.` pulls in, and no licence may
  rest on an order the engine cannot promise is sh's; (d) the unenumerable tier,
  ruled-permanent, reachable only through a literal `alias` in a shipped body.
  FORFEITS: cross-file helper reaches spelled by co-loading alone, and same-name
  collisions within one package. The two-file package shape itself is NO LONGER
  forfeited — one `.` line takes custody and it lifts, pinned end-to-end by
  `pin28-helper-package-entrypoints-lift` (the vendor's package) and
  `pin30-swapped-entrypoints-source-the-helpers` (an admin's own entrypoints over the
  vendor's helpers). CAPTURE for what remains: (c) wants a load model that carries the
  interleaving rather than a flat vector — the emission planner's territory
  (`28Q:pin-emission-planner-universal`); (a) and (b) are ruled, not owed. BACK-OUT: low
  (every suspension is additive). REVISIT: (c) with the emission planner.
- **forfeit-ambient-dependency-vouch-composition** — RULE (`30I` §3.4,
  human-typed 2026-08-19): ambient callback, plugin, patch, logging, and
  caller-provided function dependencies remain ordinary legal sh and never
  fail-fast merely because Dorc can see their definition. They do not compose a
  cross-custody vouch: exact sourced/guarded custody is still required, and every
  non-exact reach suspends while the book site runs. FORFEITS: probe/guard/elision
  value from intentionally ambient helper compositions, not the compositions
  themselves. CAPTURE: a future attributable mechanism may admit specific
  ambient dependency classes without letting co-loading or name equality mint
  authority (`30I:pin-ambient-dependency-vouch-composition`). BACK-OUT: low
  (additive authority only). REVISIT: real-oracle pressure for injected
  implementations or a broader guard/load-order design.
- **forfeit-divergence-collapse-to-unknown** — RULE (`28Q` §3, the human-carved v0
  floor): conditional/looped lifecycle events land unknown ⇒ guard/run. FORFEITS:
  every availability-derived license below an `if`-guarded creator — and defensive
  guarding is the corpus's dominant idiom, so this bites the best-written books
  hardest. CAPTURE: richer divergence tracking in the fact-lattice (per-branch
  world-states); the §3 carve reserves the seat and forbids dependants on the
  collapse. BACK-OUT: med — the carve is the only fence between the floor and
  ossification. REVISIT: near/mid-term, per the human's typed expectation
  (2026-08-13).
- **forfeit-no-host-merging** — RULE (`28Q` §3): host identity never merges at v0
  (aliased spellings of one host stay distinct). FORFEITS: duplicate probing per alias
  — wall-clock and auth-log noise only, never correctness. CAPTURE:
  controller-authenticated host identity (never resolver-tier — the failure direction
  inverts for hosts). BACK-OUT: low. REVISIT: the multi-host revival.
- **forfeit-committee-fence-sparing-inert** — RULE (`28M` §4, built-as-spiked,
  UNRATIFIED): a family whose live role members span source units spares nothing.
  FORFEITS: all survival value for multi-unit families — the overlay/patch-author
  archetype, the largest sympathetic screwed population (`28M` §6). CAPTURE: the
  composite-license admissibility ruling (the fence sitting, unscheduled). BACK-OUT:
  low pre-publication (`rul-strawman-formats-no-compat`); med after real oracles
  publish. REVISIT: the fence sitting; field evidence.
- **forfeit-command-v-poison-wall** — RULE (as-built; `28P`): a polyfill guard's own
  `command -v` line is an unmodeled running command ⇒ walls; the delivered polyfill
  cell guards instead of eliding. FORFEITS: elision in every polyfilled book.
  CAPTURE: a blessed target-state-pure builtin table in book position — a licensure
  widening, routed to the human. REVISIT: the stdlib revival (polyfills become
  common).
- **forfeit-wrapped-case-bodied-book-verdict** — RULE (as-built, measured through
  stage-i AND the crosscheck burndown; `28P` · `307c:fnd-pin30-did-not-flip`): the
  canonical case-bodied verdict shape, sited in a BOOK, answers nothing at wrapped
  sites (the same bytes in an oracle file ship fine). The stage-i EXPECTED-TO-FLIP
  cell did NOT flip — the oracle-only-vector hypothesis is REFUTED by measurement
  at both candidate seats, and it survived the wrapper-lane conversion too. The
  surviving lead (both crosscheck lineages, independently): the discriminator is
  the verdict body's SHAPE — pin28's straight-line delegation body ships and
  elides at a wrapped site; pin30's case-with-decline-arm drops — recorded in the
  cell's own header. FORFEITS: wrapped-site coverage for the most-taught authored
  shape. Stage-0 CHECKED (2026-08-16): still no flip, measured three times across
  the re-cut; SIX candidate causes now eliminated by measurement (ship-seat
  ordering · vector membership · book-lift of case bodies · arm-scoped consent ·
  the wrapped verdict-first preference · top-level-mark equivalence with pin28).
  The two surviving candidates, verbatim: `evaluate_verdict`'s tracer over a case
  body at the PEELED argv; and `peel_book_chain`. CAPTURE: chase those two seats
  from the body-shape lead, then widen the lane. REVISIT: the next wrapped-lane
  touch; the chase is now two-seats-narrow and cheap.
- **forfeit-survival-lanes-closure-less** — RULE (as-built; `28P`):
  `disturbs`/`resolve`/`reaches` bodies ship without their helper closures; a body
  death walls the footprint total. FORFEITS: survival sparing whenever a kind-owner
  factors helpers (walls, never corruption). CAPTURE: the `HelperIndex` extension —
  staged as an emission-stage rider. REVISIT: that stage's fold.
- **forfeit-loop-populations-open-until-propagation** — RULE (`30L` §7, staged by
  design): loop-generated invocation populations are typed `Open` today — a
  literal `for pkg in nginx curl; do install_pkg "$pkg"; done` contributes an
  Open census and its regions Run — even though the member identity, the
  `IterationSlot` axis, and the settlement's ordered member machinery are all
  representation-ready. FORFEITS: every loop-member elision/guard until the
  propagation lane lands; the r21 `20S` build proved the value real. CAPTURE:
  the loop-propagation lane turns finite fully-enumerated literal populations
  `Closed(members)` as a value-plane change (`p-x-loop-population-closes-over-
  literal-members` is the standing red pin; identity/witness shapes may not be
  re-keyed — `30L:pin-loop-types-need-no-rekey`). BACK-OUT: low by construction
  (that immutability is what the staging bought). REVISIT: the propagation lane,
  wanted soon per the `30L` charter.
- **forfeit-certifier-trip-evicts-elisions** — RULE (`302` §3
  rul-certifier-trip-guard-only, TYPED 2026-08-15): any solve-certifier
  `Inconsistent` evicts every elision-family outcome (elide / omit / survive)
  across the (host, plan) scope — including elisions whose own windows certified
  clean, and the measurement-only converged prefix (straight-line root,
  all-above-elided, literal argv, census-unique verdict body — derivable with no
  fixpoint anywhere). FORFEITS: the whole attention product on engine-defect days.
  CAPTURE: a second mini-analyzer, super-dumb by charter — CFG-walk and
  dataflow-staticness ONLY, pursuing straight-line elision up to the first mutator,
  sharing no solver and no certifier substrate; for the survive tier,
  measured interference-freedom via the parked generation-probe revival (`27C` §5
  fired-branch fast path — re-adjudicate against rul-attention-honesty first: it
  converges toward guard-shaped machinery). BACK-OUT: trivial — every capture is
  additive against one boolean. REVISIT: trips observed non-vanishingly in the
  field, or either capture's machinery arriving for its own reasons.
