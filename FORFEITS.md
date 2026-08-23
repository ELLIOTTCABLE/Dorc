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
BACK-OUT (how retrofit-hostile waiting is) · REVISIT (the trigger) · REDS (the xfail pins
and/or e2e cases that keep the forfeited value ENCODED IN SH and red until captured —
`30P:rul-forfeits-carry-reds`, human-typed 2026-08-22: a row without reds is incomplete).

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
- **forfeit-book-dynamic-load-analysis** — RULE (`plans/30P:the-load-principles`,
  soundness-first re-cut 2026-08-22): a `.` operand resolves only over controller-known
  inputs through shell semantics (`principle-load-operands-evaluate-over-controller-known-inputs`;
  `rul-no-tool-modelling-in-the-load-plane`); a load head is EXACT or a point havoc, nothing
  between (`30P:rul-load-head-is-exact-or-havoc` — the snapshot-suffix set, its POSSIBLE
  singleton, and the runtime-verified candidate are all struck); `$0` is symbolic with two
  live spellings, a dead spelling is not unsound (`30P:model-symbolic-dollar-zero`); a
  command substitution in a load operand resolves only through a statically-evaluable
  stdlib predict whose stdout is claimed (`30P:rul-static-predict-sites-loads`); an unknown
  source is a point havoc (r30). FORFEITS: elision below any dynamic-headed load
  (`$OPS_LIB`, `${LIB:-./lib}`, `$(find_config)`) until an authored EXACT spelling replaces
  it; `$(dirname "$0")`- and `$(cd … && pwd)`-headed loads until the static-predict tier and
  the `dirname`/`cd`/`pwd` stdlib predicts exist; glob loads (order-unknown, universal meet
  over member effects) until after `lane-loop-propagation`; slashless operands (a PATH
  search) permanently. CAPTURE: EXACT witnesses are authored text only (a literal, a
  book-set root, `$0` under the symbolic model, a static predict's claimed stdout); the
  decidable set grows by name, license-review-tier. BACK-OUT: low (every rule is additive;
  no engine selection can launder into EXACT). REVISIT: the `r31:book-load-acceptance`
  attention-call. REDS: `p-x-load-operand-case-over-dollar-zero` ·
  `p-x-load-operand-dirname-of-dollar-zero`
  · `p-x-load-operand-cd-pwd-of-dollar-zero` · `p-x-glob-load-acquires-members` ·
  `p-x-glob-load-members-are-order-unknown` · `p-x-glob-load-no-match-aborts` ·
  `load31-punted-load-shapes`.
- **forfeit-plain-sh-inclusion-analysis** — RULE (`30P:principle-book-code-source-is-inclusion`,
  tiers 1 and 3 punted by human ruling 2026-08-22): a resolvable `.` of an ORDINARY sh file
  is acquired and shipped beside the plan (`mech-acquire-and-ship-plain-sh`; landing evidence
  `load30-plain-sh-inclusion-ships`, which is NOT a red of this row) but its
  contents are NOT analyzed — no splice, its definitions unknown, its sites unplanned, the
  `.` site walls — and it is never pasted into a single-stream plan. FORFEITS: every
  elision and guard inside plain-sh helper files (the most common multi-file book shape),
  and the single-stream form for such books. CAPTURE: the splice-as-body-called-once with a
  first-class source frame (`30Pb:fnd-dot-source-remains-an-execution-frame`), then the
  byte-verbatim paste under the unwelded exclusion set (top-level `return` excluded by
  ruling; the `floor30-atlas-*` manifests are the evidence base). BACK-OUT: med — the
  splice touches the CFG and the frame model; nothing in r30 forecloses it. REVISIT: the
  next language-surface round; this is its obvious entry point. REDS:
  `p-x-book-code-source-is-inclusion` · `load31-punted-load-shapes`.
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
- **forfeit-cell-blind-self-reach-walls-loop-siblings** — RULE (as-built; r21's
  `self_reached = Reach::is_pristine`, measured by the loop-propagation lane): self-reach
  is CELL-BLIND — "nothing reached me at all" — so inside a closed member loop a region's
  own member establishes reach a sibling DIRECT mutator back over the loop edge, and
  cell-disjointness between the two buys nothing: the direct mutator runs, walls the
  region, and the region takes GUARD where a cell-aware self-reach would let it REPLACE.
  FORFEITS: full elision for the mixed loop body (a direct in-loop mutator beside a
  called one); `loop30-direct-and-called-mutators-share-a-loop-body` greens as Guard.
  CAPTURE: widen `is_pristine` to cell-aware self-reach — a licence widening, its own
  lane (`tc-self-reach-cell-blind-widening`, loop lane report). BACK-OUT: low
  (additive). REVISIT: the next settlement touch. REDS: OWED — a Replace-asserting
  variant of that case, to mint at the next dispatch (the loop lane landed under a
  dispatch hold). Residue that is NOT row-shaped, recorded in `30Q`: nested calls under a
  member loop stay unbound (⊤ ⇒ run); duplicate `(site, fact)` establishes refuse (ruled);
  a loop extent that rebinds its iteration variable refuses the member binding wholesale.
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
