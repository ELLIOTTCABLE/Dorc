# spike/ — Dorc implementation spike: working agreement

This `spike/` tree is a **disposable Rust implementation spike** of Dorc (the
spec-mining, static-analysis sh-orchestrator the repo-root docs describe). Its job is
to **surface design problems by building the hard parts** — never to become the
shipped tool. Goal-shape: academic-grade static-analysis work (CFG, monotone dataflow,
abstract interpretation, lattices), built **boring, defensive, careful, judicious** —
correctness (types + tests) over features; less code over more, never at the cost of
readability.

This file is *steering law*: the invariants, licenses, and rulings a task-focused
agent must not break while looking elsewhere. It is current-truth only, not a history
document (history lives in `Research/` + git); superseded rules are rewritten in
place, not bracket-annotated.

**Reading order** (do not skip): root `README.md` / `DESIGN.md` / `IMPLEMENTATION.md`
(human-authored ground truth — they outrank everything, including this file) →
`Research/LIVING_STATUS.md` (the live arc: charter pointers, dispatch state, current
gates) → this file → `spike/crates/<c>/CLAUDE.md` for the crate you touch. Root
`KNOBS.md` = the named design-tension registry (reuse its slugs; never re-derive a
tension under a new name); root `AGENTS.md` = repo-wide agent law (terminology
firming, exclusion-check discipline).

**Registry discipline** (this file and all seven crate files): one rule per bullet,
each with a greppable slug; grouped under standing section headers; APPEND new
entries to the matching section rather than restructuring; cite outside sources as
`docID:slug` (e.g. `271:rul-lend-map`). Dense beats prose.

## Safety — autonomous run (frontload; propagate verbatim)

Put these at the top of your reasoning, and at the very top of *every* subagent
prompt you write:
- No git mutation outside this worktree; never, ever push. Local commits on
  this `ai/*` branch are encouraged — granular, `(AI …)`-labelled.
- Don't spend external resources or exhaust rate-limits beyond tokens; don't
  mutate global state (no system packages or system config; worktree-local
  `mise` installs/config are fine).
- Everything you build follows DST discipline: deterministic, local,
  mutation-safe. Clock, network, disk, and randomness only through DI seams;
  correctness-critical kernels stay dependency-clean.
- Executable test-fixtures use non-functional stubs (`hork`, `wombat`, inert
  mocks under `PATH=mocks-only`) — never real mutators. Real-command strawmen
  in the repo are frozen evidence; they must never be executed. The only
  sanctioned executor of fixture material is `sh e2e/run.sh` (syntax-checks,
  and execs only under inert mocks).
- Perpetuate this block, verbatim, to the top of every subagent prompt.

## The product, distilled (dense root-doc duplication — read even if you read the roots)

- **what-dorc-is** — Dorc plans and applies host convergence from plain POSIX-ish sh:
  **books** (runbooks; admin-authored, chaotic, imperative) + **oracles** (per-tool
  description packages; engineer-authored, our dialect). All user knowledge is
  *spelled in sh* — function bodies, argv, control flow — never YAML / comments /
  sidecar config (`KNOBS:kOOB`).
- **two-phases-opposite-fail-directions** (inv-kfail, welded) — **probe** = read-only,
  parallel, never mutates (unsure ⇒ don't probe); **apply** = never skips a needed
  mutation (unsure ⇒ run). Phase-keyed via `core::Phase`. The one thing performance
  never trades.
- **execution-priority-order** (IMPLEMENTATION.md) — 1. NEVER under-execute (eliding a
  needed command is the cardinal sin) > 2. avoid over-execute (repeat runs) >
  3. avoid unnecessary-execute (the value-prop; lowest). Every ambiguity, at every
  layer, resolves toward *run*.
- **verdict-vocabulary** — per-line outcomes {elide, guard, run} plus
  descope / omit / survive (`KNOBS` named-mechanisms). Licenses differ per mechanism;
  never borrow a neighbor's name. Elision is observable-preserving REPLACEMENT (the
  degenerate full-skip case); a guard is an insertion in front of untouched bytes.
- **rul-attention-honesty** (welded) — attention is saved ONLY by provable elision.
  The plan render is the whole book, original order, byte-honest; a line that may
  execute is never hidden or folded (at most dimmed, warily). Never hide risk.
- **rul-divergence-proceed** — apply-time divergence from plan prediction:
  proceed-and-flag; no abort, no strict mode. All decisions front-load into the single
  approval; late events are report-items only. No second-guess layer above guards.
- **two-users-never-conflated** — the **admin** (book author; lazy; firefighting) and
  the **engineer** (oracle author; correctness-focused; publishing). Design each
  toward the other; check every design against both (and against unreliable oracles,
  and against the other phase — AGENTS.md exclusion-check).
- **correctness-is-contract-bounded** — we are exactly as correct as user-authored
  code lets us be; the engine's job is keeping failures ATTRIBUTED (a pointable line)
  and LOCAL. `271:rul-sin-ordering`: mis-attributed error (pope-sin) > unattributed
  (cardinal-sin) > attributed-but-could-have-helped-more (mild-sin) > human-failed-
  against-our-genuine-best (not-sin). Slot every failure-mode analysis into that scale.
- **no-reorder-ever** — the book's order is sacred: no intra-host apply
  parallelization or reordering, ever. Apply speed comes from elision only;
  probe-phase parallelism is where wall-clock is won.
- **perf-doctrine** — network round-trips dominate; slow remote commands dominate the
  network. Analysis-side big-O is almost never the constraint: spend analysis freely;
  never let a network boundary participate in iteration.
- **oracles-may-lie** — judgments, not facts: the system has coherence checks,
  calibration, and attribution — never a truth/lie axis. An oracle's product is a
  JUDGMENT (the deliberate helpful lie is the stdlib's founding transaction).

## Invariants — license & trust (the wrong-elision tripwires)

- **inv-must-may** — only a `Must`-grade fact may license a replacement; `May` blocks
  or hints, never authorizes. The one-way coercion is `Must → May`; the reverse does
  not compile.
- **claim-tier-gating** — license mints demand `ByVouch<VerdictVouch>` by value
  (`core::claim`; sealed tiers ByObservation / ByVouch / BySilence). If a mint
  signature blocks your build, you hold the WRONG claim: obtain the real vouch (author
  the verdict-function) or let the command run — NEVER convert or fabricate a claim to
  satisfy a signature. That conversion IS the soundness hole the boundary exists to
  stop ("expected ByVouch, found ByObservation" = a measurement being laundered into a
  mutation-license). A vouch informs a license and never becomes a fact.
- **rul-vouch-is-verdict-authoring** — authoring `cmd__is_converged()` / IS the
  vouching act; a reached path answering 0-in-the-named-sense is the license.
  Declining is ordinary control-flow (`return 2` / unauthored path); run-delta verbs
  decline natively. No vouch ⇒ run. The vouch never enters the fact-plane and is
  inadmissible in any other site's reasoning.
- **rul-ternary-verdict** — the per-site verdict set is {elide, guard, run}. A guard
  is `( <verdict-fn> <site argv> ) || <original bytes>`: the original command's bytes
  always survive verbatim (no code path removes them); the inserted check is the
  oracle's own authored body, shipped strip-only. Two nevers: never
  engine-synthesized sh (no author ⇒ no self-vouch, no attribution, and a second
  source of convergence-truth); never declared/claimed output in guard position.
- **rul-probe-mutation-ownership-split** (WELDED, human-typed 2026-07-17;
  `27C` §3 carries the full text) — the probe-never-mutates law allocates by
  OWNERSHIP: authored/oracle code (stdlib included) sits under the loud
  frontloaded no-mutation contract (the uncontrolled ocean; an entry-form's
  self-effects are the AUTHOR's vouched residue, attributed to their line);
  ENGINE-generated constructs are OWNED — no hard line, judgment/UX tier,
  careful-and-necessary against the flag-derived user-story. Never justify
  either tier by "state the user cares about" (struck: referent-agnostic +
  phantom user-instruction).
- **rul-no-mutating-guards** — a verdict-function body that fails the non-mutation
  proof lifts NOWHERE: not as a probe and not as an apply-time guard. We do not
  insert proven mutation, out-of-order, in not-user-spelling, into apply bodies.
- **rul-proven-mutation-fails-fast** — proven-mutates ⇒ genuine plan-time,
  pre-network, loud fail-fast (the declarations-genuinely-contradict category),
  whatever the provability source. Fail-fast governs until first network-transit;
  best-effort begins where static analysis is exhausted.
- **rul-unprovable-rides-the-vouch** — the unprovable region of a verdict-function
  ships on the authored vouch, probe AND guard. The static effect-check is
  falsification-first, NEVER a completeness gate.
- **structural-vouch-only** — probe-inertness has exactly ONE source: the structural
  self-vouch (a command inside its own oracle's authored body). Mutation-analysis of
  arbitrary commands is impossible, permanently; no analysis-confidence threshold
  ever makes a probe "safe". PLT soundness-totalism actively harms design thinking
  here — Dorc pipes declared claims around and attributes failures best-effort.
- **rul-only-oracle-bytes-ship** (RATIFIED 2026-07-16) — the probe lane ships ONLY
  oracle-authored bytes. A probed compound ships with every participant replaced by
  its oracle's predict (composed: `a__predict -f -f | b__predict`); wrapper contexts
  are entered only through the wrapper-oracle's authored entry form. Build riders:
  per-channel coverage (a participant may be model-substituted iff every channel the
  compound consumes from it is covered; an rc-only body never sits upstream of a
  byte-consumer) · stream-fidelity of substituted bodies on consumed channels ·
  capture-ships-real-bytes (byte-consumption demands real execution — same rule, not
  an exception). ⚠ HEAD DEBT: the landed `24J` connected-probe raw-ships book bytes —
  confirmed standing-law debt; the repair is the block-rebuild probe-emission
  touch-point. NEVER imitate the landed shape; read `24J`'s header correction before
  touching probe emission.
- **rul-argv-flows-bytes-do-not** — the admin's argv flows into the shipped probe as
  the predict's *arguments*, passing THROUGH the oracle author's own argparse (the
  argparse is the type-checker of the vouch — declines included); the admin's bytes
  never ship. Engine shape-matching over book bytes is an unchecked cast and may
  never mint a probe-execution license. (Lifted in-book guards contribute
  *recognition* only: `dpkg -s nginx` lifts as `dpkg__predict '-s' 'nginx'`.)
- **probe-composition-walls** — probes are synthesized CFG scaffolding + oracle
  bytes, never book bytes (a probe never inherits the book's `trap`s); inversely,
  oracle code never transports into book code — only synthesized scaffolding does.
- **vouch-scope-is-the-body-never-the-tool** — a body-vouch is a claim about a REGION
  OF SH, never about the command families it invokes; it mints no command-family
  fact. An unmodeled command carried in one oracle's body stays ⊤ everywhere else.
  Exactly one vouch reaches out of its file: a tool's own oracle reaching that tool's
  book-sites, argv-keyed.

## Invariants — separation, worlds & survival

- **silence-licenses-nothing** — an unmodeled command / wrapper / dimension / kind is
  ⊤ (a wall), never a permissive default. Hints drive gradual enhancement; silence
  never licenses anything, anywhere, at any tier.
- **never-derive-separation** (`272` §4) — derivation yields *keying*, never
  separation: address-inequality is not referent-inequality. Vocabulary: **keyed**
  (derived; safe; license-free re-indexing that can only block transport) vs
  **partitioned** (declared-only; knife-tier). Declared separation across
  context-values does not exist at all at v1.
- **ternary-compare-consumer-map** (`277` §2) — `compare(cellA@ctx, cellB@ctx)` ∈
  {same | provably-disjoint | unknown}. *same* feeds transport only;
  *provably-disjoint* feeds survival-sparing only (flag-gated); *unknown* is the safe
  bottom for BOTH consumers. It is ternary because of the safety inversion:
  believed-no-overlap is safe for the transport consumer and dangerous for the
  kill-traffic consumer, and vice versa — no binary default is safe for both.
- **rul-flag-is-razor-residue** — claims own what lines can say; the admin flag
  (`--risk-faultless-skips`) owns what no line can say (the open-world at-most
  residue: the frame problem's "and nothing else"). Per-invocation, never a default;
  never keyed to claim-types. Keying never feeds survival; hint-lane values never
  feed survival; the flag permits acting on separation claims, never manufactures
  them.
- **sparing-algebra** (`277` §3, as amended by `279f`) — same-entity, a claim SPARES
  a backing iff BOTH sides carry minted selectors AND claim-token ∈ dialect(the
  backing's minting family, kind) AND claim ≠ backing. Everything else COLLIDES: a
  selector-less/⊤ coordinate on EITHER side, unminted tokens, cross-dialect tokens.
  Marked runnable lines mint (verdict `:`/`:!` and observe `:?`); claim/disturbs
  emissions never mint.
- **set-lifting-universal-meet** (`277` §5) — consumers quantify UNIVERSALLY over
  backing-SETS: sparing requires EVERY footprint×backing pair provably-disjoint (any
  unknown member ⇒ collide); transport requires every member to transport. Pins:
  **pin-no-outcome-as-generator** (a compare-verdict feeds only its licensed consumer
  and never re-enters the relation as evidence for a later verdict);
  **pin-set-meet-order-independence** (a set with any unknown member collides at
  every iteration, whatever the resolution order).
- **top-identifies-with-nothing** — ⊤ identifies with nothing, including itself;
  cross-kind *same* does not exist (no generator; the co-reference mechanism is
  parked behind a movable kind-fence).
- **fence-divergent-meaning** — a claim-token is interpreted in the BACKING family's
  dialect; same-spelled tokens with divergent meanings across families are a
  frontloaded README-class limitation — documented and differential-tested, never
  lint-rescued (`271:rul-net-quality-u-curve`: imperfect mechanical nets are the
  design footgun; docs beat them).
- **empty-world-byte-identical** — with no oracles loaded, the entire algebra is
  invisible: rung-0 regression, byte-identical to entity-granular behavior. Every
  brief pins it.
- **context-entry-probing** (`plans/27C` — THE wrapper/context spec) — a wrapped site
  is answered by measurement in the site's *denoted* context. Reuse-never-acquire:
  the probe lane exercises only authority the connection already holds (no prompting,
  no credential handling); there is NO privilege ordering ("demotion" is not a safety
  category). Ternary escalation dial, default = shifts only for `tolerates:`-vouched
  functions (both-sides consent: author's per-function per-dimension mark × admin's
  dial). Wrapper entry forms are the ONE licensed seat for real context entry;
  predict closure bodies never escalate. Every failure (entry refused, impossible,
  rc 127, in-context decline) lands can't-say ⇒ guard/run. Without entry, a
  substrate-axis fact crosses a boundary only via **pure-predicate carry**
  (unflagged; see next bullet); an ingredient (identity) axis crosses only via the
  kind-owner's typed `invariant:<axis>` line × the flag. Silence walls.
- **pure-predicate-carry** (`plans/27C` §4(a); `notes/27Xf` Tier-1; human-opted
  2026-07-17) — the ONLY unflagged cross-substrate-boundary carry, and the RETIREMENT
  of the old "engine-warranted carried-by row" (it leaned on tool-semantics the engine
  may not hold). A substrate-axis fact travels unflagged iff (A) every marked backing
  kind carries its owner's `invariant:<axis>` line AND (B) the engine proves the
  verdict body READ-SET-CLOSED — everything influencing the verdict rc (data AND
  control-flow) traces to the site's argv or a marked read, with no unmarked external
  input. Referent-agnostic: the closure reads MARKS + sh-structure, never tool
  semantics (`inv-referent-agnostic`). The closure pass is DEFAULT-DISQUALIFY (any
  construct off the audited pure-construct safe-list disqualifies) and fails safe
  (missed-safe loses an elision, never carries a hidden read). Substrate axes ONLY —
  user/identity is excluded, because a shift there changes ACCESS to the body's own
  reads (EACCES flips the answer of a structurally-closed body). netns: `net.*` is
  per-netns, so the model must forbid `invariant:netns` on network kernel state. The
  pass is a correctness surface the spike must build and prove; empty-oracle world
  stays byte-identical (no closure attempted).
- **value-predictions** (`275` · `271:rul-value-prediction-species`) — every
  byte-shaped belief beyond program text (captured stdout, stored rcs, composed
  outputs, register-resolved values) is a value-prediction; its provenance and
  backing are DERIVED, never declared — the authored surface is the empty set.
  Freeze-at-binding; the patrol IS the walls machinery. Backing carries NO
  completeness burden — which is why cross-context *value* transport was refused as
  posed (`279f` §3) and the answer is measure-in-context (`27C`), never a
  completeness claim.

## Invariants — observables & rc

- **inv-one-observable** — exactly ONE concept of a command's observable: its
  output-tuple over channels {Effect, Status, Stdout, Stderr} (extensible).
  `predict()` predicts per-channel values (or a loud OOB can't-predict); an enclosing
  context CONSUMES channels; a substitution REPRODUCES the consumed channels'
  predicted values; elision is licensed only when Effect predicts no-mutation.
  Convergence is the DERIVED state of the Effect channel — never a separate
  probe-reported verdict. Do not re-introduce a standalone `Verdict`, a bolted
  `Observed{rc}`, or a consumption-only observable enum.
- **status-consumption-trichotomy** — consumed Status splits three ways, keyed on
  real consumption semantics: `StatusRelaxable` (a known rc reproduces the consumer's
  decision — `&&`/`||` operands, errexit-region commands, `$?`-readers'
  predecessors, `if`/`elif` guards; a probe-sourced rc substitutes exactly, ⊤
  blocks) · `StatusInvariant` (the bare `cmd || true` left operand:
  consumed-in-form, dead-in-fact; NEVER blocks, even at ⊤; still RECORDED in the
  consumed set; mark-union composes — any other blocking mark on the site wins) ·
  `StatusIterated` (a `while`/`until` condition: a per-iteration SEQUENCE no single
  rc reproduces; blocks unconditionally).
- **inv-probe-sourced-values** — a replacement stand-in may reproduce ONLY values
  with probe-provenance: every emitted channel value traces to a concrete observable
  the probe actually produced. No fabricated defaults, no rc=0 assumptions, no
  synthesized stdout; a *consumed* channel whose prediction is ⊤ forbids the mint.
  Consumption-coverage is the load-bearing precondition: never argue a channel
  "dead" without tracing who could read it. Knife-tier "probe-provenance" / "real
  bytes" is pinned to the world-spoken (delegation-produced) reading
  (`271:rul-composed-bytes-defer-and-floor`).
- **guards-mint-no-values** — a GuardInsert carries no StandIn, no Predicted, no
  Observable: on pass, the check's own live rc is the line's rc; on fall-through,
  the original command runs and its observables are genuine. Guards are licensed by
  the vouch, never by probe-provenance of values.
- **anti-masking-tests** — no test may hand-inject an observable the check itself
  should predict; a check returning can't-predict must flip its dependent case to
  *run*.
- **rul-rc-partition** + **rul-zero-one-inversion-pair** — a verdict-function's exit
  status reads one fixed table: 0 = the named sense holds; 1 = its complement;
  **≥2 = flat sink** (confused ⇒ run), semantically flat FOREVER — never inverted,
  never licensing, in any current or future decision table. Direct glue =
  `( f args ) || <bytes>`; declared-dual glue = engine-emitted mechanical sense-flip.
  Authors never collapse statuses out of a verdict-function (no `!`, no `|| true`,
  mind pipeline tails); tools with non-test exit vocabularies get explicit
  `case $? in` remap arms.
- **rc-naming-discipline** — bare "rc" is banned in design text: qualify as tool-rc /
  predicted-rc / apply-rc. Verdicts are never rc's; Dorc verdicts travel out-of-band
  (`$DORC_VERDICT` lane); no exit code can mean "unknown".
- **sigpipe-flap-class** (`279f`) — pipefail + early-exit consumers (`| grep -q`)
  race SIGPIPE into rc-141 sink-landings that flap run-to-run: a NAMED
  nondeterminism class. Why-lane note on 141-landings ("likely benign early-exit
  race; consider full-read form"); `dorc plan --exit-code` computes from
  divergence-of-world facts, never raw sink-landings; hostsim injects the race.

## Invariants — analysis boundaries

- **inv-top-reject** — anything unmodeled collapses to ⊤ and is rejected loudly
  (un-probeable ∧ un-elidable), never silently best-effort'd. Under-modeling is a
  correctness boundary, not a TODO. Shrinking a ⊤-trigger is a deliberate design
  act, never an accident; bias every parser ambiguity toward
  ⊤-reject-with-diagnostic.
- **inv-referent-agnostic** — the engine never decodes an `OpaqueToken`'s or kind's
  text to infer meaning; cross-oracle identity binds to a named `KindId`, never a
  shared token. Resolve interned text for display/provenance only; never branch on
  it.
- **identity-declared-never-inferred** — the engine parses NO tool argv; the
  oracle's own argparse is the sole entity-resolver. Never re-introduce engine-side
  argparse or flag-strip stand-ins.
- **inv-superposition** — the kernel emits phase-/orientation-agnostic facts; only
  the phased caller collapses them. Never bake a phase default. A cross-cutting
  `tc-*` judgment call is flagged UP to the orchestrator, never settled inside a
  component or a single-crate subagent.
- **inv-site-keyed-results** — the probe-results lane keys by command-site (the
  stable LeafId→AstId back-map), never by fact / kind:entity / command-family: two
  same-command sites must not collapse. (Fact-keyed verdict shapes are a conscious
  orchestrator+human decision, not a local refactor — kSTATE-coupled.)
- **inv-leaf-seam** — executable work is a list of individually wrappable leaves
  with a stable LeafId→AstId back-map; never one opaque `sh -c "$bigscript"`.
  (Under function inlining the map is non-injective AstId-ward; the Step-level map
  stays injective; the CALL leaf is the render unit.)
- **toctou-scope** — IDENTIFIED-CAUSE re-verification is in (a guard re-verifies a
  fact whose invalidation has a named, in-book cause — hork-catching);
  UNATTRIBUTED-drift machinery is out (no freshness windows, no systematic
  re-probe, no third-party/wallclock drift accounting). Do not build toward the
  latter.

## Invariants — engineering substrate

- **inv-determinism** — the analyzer kernel (`syntax → analysis → probe → plan`) is
  a pure function of its inputs: no clock, RNG, filesystem, or network, directly or
  transitively. Nondeterminism lives ONLY in `hostsim` (seeded, injected PRNG) and
  the `cli` edges (real I/O). Never iterate a `HashMap`/`HashSet` where order is
  observable — `BTreeMap`/sorted vecs. No `async` in the kernel. DST over seeds is
  the regression backbone.
- **inv-no-throw** — every pipeline stage returns `Carrier<T>` (value + accumulated
  diagnostics) and never panics on malformed input. Errors are data.
  `unwrap`/`expect` never on untrusted-input paths (tests may).
- **inv-no-unsafe** — `unsafe` is `forbid`-den workspace-wide. No FFI. No authored
  macros (`macro_rules!`/proc-macros); standard `#[derive(...)]`s encouraged.
- **hermeticity-precondition** (`KNOBS:kVOLATILES`, welded) — read-only ≠ hermetic:
  a probe that routes through live DNS / warm caches / wallclock cannot license
  (the `getent hosts` class). Hermeticity is a *precondition* for any sound
  skip-system, never an optimization.
- **skip-banned** — "skip" is a banned word in design/code layers: elision is
  observable-preserving REPLACEMENT, and the hard parts are observables /
  replacement / insertion, not rc-checking (the ban protects LLM reasoning from
  that misread — `271:rul-skip-ban-is-llm-facing`; the word is fine in human-facing
  UI/doc text).
- **diag-api-design-for-keeps** — the structured diagnostic API is the ONE
  sanctioned design-for-keeps exception to spike disposability (the real codebase
  extracts it). Nothing else gains that status by analogy.
- **churn-avoidance-disclosure** (ru-26) — any implementation shaped by a
  spike-specific scope-cut MUST carry a nearby inline note saying so, so the cut
  never leaks silently into greenfield work referencing the spike. (Live instance:
  anything not-handling-stderr must say so, locally and upfront.)
- **two-surfaces** (rec-1) — the shipped/off-ramp `.sh` artifact is byte-floored and
  receipt-free (byte-identical under receipt-stripping, comments included); the
  PLAN-RENDER surface (TUI/CLI, `why`-lens) is the sanctioned home for per-line
  claimed-vs-proven disclosure — overlaid on the artifact bytes, never embedded in
  them.
- **probe-tape-not-a-cache** (rec-5) — the probe-TAPE is a write-only postmortem
  durable; NOTHING re-ingests receipts across runs. The kSTATE reuse-cache stays
  parked; unparking it is a human act hard-coupled to hostile-host security work.

## The authored surface (semantics digest; specs outrank: `277` · `273` · `274` · `275` · `plans/27C`; one-page dialect reference: `278`)

- **families-and-roles** — a *family* is the name-derived set of `__role` functions
  describing one description-target; two species: COMMAND (`systemctl__*`) and KIND
  (`sm_dorc_Package__*`). Membership is by name-construction only — never file,
  never author. The role vocabulary is engine-owned, closed-at-a-version, extends
  BY NEW NAME ONLY. Names are bare munged POSIX NAMEs (dots are dead) and a
  PERMANENT, unversionable compat surface — recognized even in unmarked files.
- **role-menu** — `cmd__predict()`: the one read-only model of a command;
  wrapper-ness is DETECTED, never declared (a body whose command-position `"$@"`
  runs its argument-slot is a peeling wrapper by tautology); per-channel
  claim/decline vocabulary: delegation line = faithful all-channel claim · printf =
  asserted output claim · explicit return = rc claim · redirect-to-null =
  per-channel DECLINE (that channel ⊤ for consumers) · `return 2` = whole-shape
  decline. `cmd__is_converged()`: the verdict member (see
  rul-vouch-is-verdict-authoring). `cmd__disturbs()` (né touches): at-most claims
  per MATCHED invocation-shape. `cmd__lend_map()`: the wrapper's dimension member —
  empty entry for a present key = full lend; contents = mapped lend; a MISSING key
  = ⊤, walls (the enumerate-every-dimension law; absent-key-means-full-lend is
  REJECTED). `kind__resolve()`: entity canonicalization within its kind.
  `kind__disturbance_reaches_only()` · `kind__state_stored_only_in()` (substrate
  emission lines + the `invariant:<axis>` colon-line; whole-member scope). Naming
  law: `only` in a role name = complete-by-contract, totalistic-survey-before-
  authoring; absence = arm-incremental.
- **coordinate-semantics** — the flat three-place `(kind, entity, selector)` +
  a context slot (recursive coordinate shapes DECLINED —
  `271:rul-coordinate-shape-flat-three-place`; structure lives in kind-owner
  functions BETWEEN coordinates). Kinds are reverse-DNS, ≥2 dots. `#` introduces
  the selector, ATTACHED (valid coordinate char immediately before it). Polarity
  rides the mark sigil family `:` / `:!` / `:?`, never the coordinate. The bare
  selector-less form permanently means whole-entity, and reads as ⊤-selector at
  consumers (collides with every cell). A coordinate names a CELL; names ≠
  referents (aliasing is why resolve exists). Binds name entities, never cells
  (SOFT ruling). A verdict/observe mark asserts exactly ONE thing (multi-
  consequence readings are derivations); brace-alternation `#{a,b}` is
  claim-emission-marks only.
- **rho-claim-ladder** (`271:rul-env-claim-inversion`) — bare `"$@"` claims NOTHING
  (⊤; never "claims isolation") · `VAR=x "$@"` = per-variable claim, rest ⊤ ·
  `env "$@"` = full ambient passthrough (the `env` syllable IS the claim) ·
  `env -i VAR=x … "$@"` = exactly-these. Silence = floor everywhere; every
  believable claim is a typed, pointable line.
- **wrapper-law** (`273`) — a peeling wrapper parses a prefix of its own argv, then
  execs the REMAINDER verbatim, once, locally (escapes belong to other families:
  eval'ers, runtime-data, out-of-family declines). Dual-peel coherence: predict and
  lend_map argparse are independent, but where both answer one invocation their
  `"$@"` must reach the same tail position — disagreement is static incoherence ⇒
  plan-time fail-fast. No tool oracle ever mentions a wrapper; no wrapper oracle
  ever mentions a kind.
- **dorc-sh-trio** (`274` · `271:rul-dorc-prefix-head-synthesis`) — bare `sh -c '…'`
  = THE escape hatch: analysis DESCENDS for hints only and licenses NOTHING (this
  IS the long-owed `unsafe`; no second construct will ever exist —
  `276:rul-unsafe-is-bare-sh`). `dorc:sh -c '…'` = full analysis license;
  probe-ship rewrites to `dorc-sh` (per-run PATH shim); strip = prefix-erasure.
  `dorc-sh` typed directly = the runtime object: no analysis license;
  strip-untouched (documented-dangle). No nested annotation inside opaque blobs
  (plan-time parse-failure tier). Descend-don't-license is enforced at the TYPE
  level: invited rooms may mint licenses, hint-only rooms cannot.

## Language & off-ramp law

- **marker-gates-syntax-only** — `# dorc-lang/v0.1` (exact-match, stands alone,
  first ~10 lines; the sole sanctioned comment-parse in the product, a closed set
  of one) gates SYNTAX only — never `__role` name-recognition, never semantics. An
  unmarked file is plain sh.
- **stability-ledger** (`276:rul-verdicts-never-stable`) — syntax = marker-gated ·
  `__role` names = permanent · verdicts = unstable-and-improving, disowned.
  **plan-as-API** is the named failure-mode: never promise cross-version
  plan-shape stability; anything like `--exit-code` gates on divergence-of-world,
  never plan shape.
- **strip-is-pure-erasure** — `dorc strip` erases binds + marks whole-statement,
  erases the `dorc:` prefix, rewrites the shebang-runner; NO in-body name
  rewriting; `dorc-sh` row-three untouched. Load-bearing subtlety: a bare-mark
  statement is an annotation-LINE (deleted whole, like a comment — NOT a POSIX `:`
  command); the author's last substantive command must remain the last
  status-affecting statement in the stripped body — a stripped-in trailing `:`
  clobbers the body's tool-rc to 0 = an always-skip guard.
- **two-binary-floor** (`276:rul-spec-two-binary-floor`; `KNOBS:kWHICHSH` WELDED) —
  a valid stripped base-dialect text parses and runs identically under
  `posh 0.14.1` and `dash 0.5.12`; where they disagree, the construct is outside
  the dialect. Strip-then-run-under-both IS the executable off-ramp test. Scope:
  oracle/marked text only (book-acceptance is a separate, open question).
  **fence-rejection-rc**: no dialect rule may ever depend on the exit code or
  error text of a REJECTED construct (the binaries diverge there).
- **dialect-quality-law** (`276:rul-base-dialect-ruling-list`) — the dialect is
  "POSIX + `local`"; quote-as-law; printf-doctrine (never `echo` with
  flags/escapes); `f()` only; the bash-family ban (`${x/…}` `${x^^}`
  `${x:off:len}` `[[ ]]` `==` `<<<` `&>` `|&`); `$'…'` out-for-now; authored
  `eval` never (delegation is an ACTUAL command; `eval` may reappear only as
  engine-lowering vocabulary).
- **emit-never-class** — `test -a`/`-o`, and bare `set -o pipefail` on
  durable/paste-facing surfaces: accepted and modeled in input, never written by
  us. The blessed pipefail spelling is the self-gating
  `(set -o pipefail 2>/dev/null) && set -o pipefail`; ephemeral post-handshake
  wire-bytes emit bare (`276:rul-pipefail-emit-never`). Pipeline-rc is
  verdict-load-bearing (pipefail off ⇒ wrong verdicts, unsafe; on ⇒ lost
  elisions, safe); apply-lane availability is a per-host handshake fact, never a
  version database; non-pipefail executors are an explicitly unsupported class.
- **posix-in-spirit-default** (`271:rul-posix-in-spirit-defaults`, standing) — for
  any grammar minutiae the dialect must mint (charsets, quoting): find the
  corresponding POSIX rule, simplify it for our purposes, match it in spirit.
  Conservative for the spike; characters once granted can never be clawed back.

## Where the build stands (dated 2026-07-17 — the one drift-expected section; `Research/LIVING_STATUS.md` is the live view)

- LANDED: the round-24 ladder Stages 1–5 + `270:block-rebuild` COMPLETE (eight
  dispatches, both bless checkpoints stamped; ledger `notes/27D`, landing notes
  `27E`–`27I`): the dorc-lang v0.1 corpus respell (`#`-selectors, sigil family,
  disturbs-family names, markers+shebangs) · the typeless floor (auto-cell;
  verdict-provider kernel seam; marker-gate at the cli edge; totalistic
  forward-munge) · composed-predict probe emission (the `24J` raw-ship debt
  REPAIRED — rul-only-oracle-bytes-ship is machine-pinned) · the `277` entity
  algebra (chokepoints in `core::coord`; `Relation::Overlaps`; dialect sets;
  context slot reserved) · cause-tagged fragment recipes + `ValueGrade` +
  backing-SETS with observe-widening producing · `dorc-records/1` framing
  (production deframer; strict path; `@@dorc@@` terminal token) · e2e
  de-graduated 154→76 with the `render_corpus.rs` twin tier + the mandatory
  dash-n net · whole-file `dorc strip` (parser-backed; erases the marker) +
  `dorc-sh` thin bin.
- The five old XFAIL specimens are RESOLVED (respell landed); no XFAILs at HEAD.
- The `24J` raw-ship law-debt is CLEARED — the invariant block above remains
  binding law; the "HEAD DEBT" annotation inside rul-only-oracle-bytes-ship is
  historical.
- NEXT: `270:block-context` (implementation-planning first; consumes
  `plans/27C` whole + the `27D` forward seam-list; payload rulings
  `27D:rul-payload-pins-near-weld` + `27D:rul-synthesized-payload-render-stays-
  unwelded` govern payload-v1), then `270:block-stdlib`.

## Build / test / run

No per-dir toolchain pin; the global mise config supplies stable. **Always invoke
cargo through mise, from inside `spike/`:**

```
mise exec -- cargo build --workspace
mise exec -- cargo test --workspace
mise exec -- cargo clippy --workspace --all-targets
sh e2e/run.sh        # the e2e corpus (case-count drifts — count the dirs): dash -n gate + exec-under-mocks
```

- `DORC_E2E_QUIET=1` suppresses per-case `ok` lines (final tally only; failures
  still print in full).
- Pre-commit gate set (nothing runs automatically — there is NO git hook; run all
  four yourself before every commit; never `--no-verify`): `cargo fmt --check` ·
  `clippy -D warnings` · `cargo deny check licenses bans sources` · `typos`
  (`mise x -- typos spike` from the worktree root).
- Before trusting e2e results, force a fresh `cargo build --workspace`; run the
  final e2e FOREGROUND with a generous timeout.
- **BLESS is EXCLUSIVE** — `BLESS=1` re-blesses ALL cases from whatever
  `target/debug/dorc` exists at that instant; concurrent agents share one
  `target/`. Never run BLESS while any build-agent is in flight;
  orchestrator-only, on a freshly-verified binary, resulting diff inspected
  case-by-case. Bless cannot prove an elision RIGHT — review by eye.
- Lint posture: the workspace lint table in `spike/Cargo.toml` is policy for new
  code — do not weaken it. Legacy crate-root `#![expect(..., reason)]`s
  self-ratchet; remove as layers are replaced; never add new ones to fresh code.

## Code style

- Newtypes over bare integers/strings; make illegal states unrepresentable.
- Doc-comment every public type/fn with *why*, citing the research slug it
  implements. Avoid what/how comments on self-evident code (~10% comment budget,
  brutally brief).
- Rust convention is 4-space and `rustfmt` enforces it (project convention beats
  the human's global 3-space preference).
- Tests: brutal, adversarial integration tests + DST systems-tests over exhaustive
  unit coverage; each test carries a reasoned argument for the invariant it pins;
  repetition in tests is fine. Honor anti-masking-tests (above).

## Boundaries

- **Never edit** the worktree-root human docs: `README.md`, `DESIGN.md`,
  `IMPLEMENTATION.md`, `USER_STORY.md`, `TODO.md`, `AGENTS.md`, root `CLAUDE.md`.
  `KNOBS.md` is conductor-editable only with edits left UNCOMMITTED for human ack.
  Surface problems upward instead.
- **Never read** `Research/notes/quarantine-DO-NOT-READ/` (including spike2 code)
  or `Research/corpora/` unless the orchestrator explicitly hands you a pointer.
- Spike design notes (what strained, where, confidence-marked) go in
  `Research/notes/` as numbered notes: append-only, new note per chunk, never edit
  a prior note; check the tree before minting an ID.
- Commits: small + granular + frequent. `(AI <labels>) terse one-line message` per
  `.gitlabels`; the `AI` label is mandatory; no `Co-Authored-By` trailer; never
  push. Run the four gates first.

## Spawning subagents (supervisor law — mandatory)

- Every subagent prompt begins with the **Safety block** (verbatim, above), then:
  - **step-zero** (worktree agents only — `isolation: worktree` bases agents on a
    possibly-stale `main`): `git switch -C <task-branch> <current-lineage-branch>`
    (today: `ai/spike3-r23`), verify the tip hash matches what the conductor
    stated, verify `pwd`; **step-0.5**: `mise trust`.
  - **step-one**: an EXPLICIT read of root `README.md` + `DESIGN.md`, this
    `spike/CLAUDE.md`, and the crate's `CLAUDE.md` — before any task material.
    Then exactly the note-slugs the orchestrator hands it. Pass absolute paths.
- Hand it the specific invariant slugs it must honor; require it to flag (never
  resolve) any `tc-*`-shaped judgment call; require it to report back context
  other subagents must maintain.
- Every builder brief MUST forbid sub-spawning in so many words: "do the work
  yourself; you MUST NOT spawn subagents." (Sonnet-class agents have an observed
  recursive-self-delegation failure mode; never hand sonnet-tier a brief without
  the clamp.)
- Propagate the naming discipline (`270` §1) into every brief: hyphenated
  full-word slugs; `docID:slug` for outside references; subscript old opaque
  labels once ("né P5").
- Briefs that churn tests/fixtures carry the comment budget rider (`24P` §8:
  rip-don't-update + a hard numeric budget + the counting command).

## Confidence + reference discipline

- Mark uncertain claims: `+SURE` / `~SUSPECT` / `-GUESS` / `--WONDER`.
- Slugs: hyphenated, full English words, HARD MINIMUM three words
  (`finding-guard-bytes-travel`, never `q-2`/`W3`); outside-document references
  carry the minting doc's ID (`docID:slug`).
- Reuse `KNOBS.md` and existing corpus slugs rather than re-deriving a tension or
  concept under a new name; unroll opaque legacy labels at point of use.
