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
  an exception). The `24J` raw-ship debt is REPAIRED and machine-pinned: probe-render
  tests assert the raw book site cannot appear in emitted probe bytes, and their
  failure means the debt returned. Historical shapes in `24J` predate the repair.
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
- **rul-every-erased-establish-is-vouched** — the vouch requirement follows the
  MUTATION, not the plan node's shape: an establish erased inside an aggregate
  (member-loop, inline call) consumes its own reached `ByVouch<VerdictVouch>` exactly
  as a standalone one does. The aggregate proof is private, non-empty when mutation
  exists, and identity- and cardinality-matched to the exact ordered establish sites;
  missing, extra, duplicate, reordered, declined, dynamically-unresolved, or
  wrong-site/wrong-fact vouches reject the WHOLE aggregate, atomically. Convergence,
  self-reach, grade, consumption, and render-floor gates remain independently
  necessary — a vouch is an added condition, never a substitute. A genuinely
  query-only body proves its read-substitution separately and must never manufacture
  a mutation vouch to share an API. Never widen the aggregate mints to accept
  observation.
- **rul-probe-writes-only-what-it-owns** — controller-generated probe plumbing may
  open, create, truncate, read, or remove ONLY a resource it exclusively created and
  still holds; never a host pathname it merely named, was handed, or reconstructed
  from a string. Setup that cannot be made exclusive DEGRADES the affected lane
  (supply an inert sink; keep the static tiers) and never retries by name or removes
  an unowned object; the degradation is decision-inert evidence, and a
  degraded/failed lane never fails a plan or an apply. Host environment variables are
  hostile input and never site controller scratch — roots are controller-supplied
  literals (**rul-scratch-root-never-read-from-host**: no `TMPDIR`, `HOME`, or `XDG_*`
  expansion ever sites engine scratch; a host-chosen parent voids the exclusive create
  that the whole lane rests on, so host-configurability here is forbidden rather than
  unimplemented — an admin override, if one is ever wanted, is a controller-side value).
  This is a strictly narrower rule than the no-mutation contract: it binds
  ENGINE-generated constructs (`rul-probe-mutation-ownership-split`'s owned tier),
  where "we were only reading" is not a defense, because the write primitive is the
  scaffolding's, not the author's.

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
  every iteration, whatever the resolution order). Backing-sets are NON-EMPTY by
  construction (the minting line's own coordinate is always a member) and ⊤ is
  never encoded as ∅ — universal-over-∅ would vacuously spare/transport (`277` §5
  inv-backing-set-nonempty-by-construction · inv-top-never-encoded-as-empty).
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
  category). Ternary escalation dial, default = shifts only for `safe-across`-vouched
  functions (both-sides consent: author's per-function per-dimension mark × admin's
  dial). Wrapper entry forms are the ONE licensed seat for real context entry;
  predict closure bodies never escalate. Every failure (entry refused, impossible,
  rc 127, in-context decline) lands can't-say ⇒ guard/run. Without entry, a
  substrate-axis fact crosses a boundary only via **pure-predicate carry**
  (unflagged; see next bullet); an ingredient (identity) axis crosses only via the
  kind-owner's typed `undivided-by-transit-across <axis>` mark × the flag. Silence walls.
- **pure-predicate-carry** (`plans/27C` §4(a); `notes/27Xf` Tier-1; human-opted
  2026-07-17) — the ONLY unflagged cross-substrate-boundary carry, and the RETIREMENT
  of the old "engine-warranted carried-by row" (it leaned on tool-semantics the engine
  may not hold). A substrate-axis fact travels unflagged iff (A) every marked backing
  kind carries its owner's `undivided-by-transit-across <axis>` mark AND (B) the engine proves the
  verdict body READ-SET-CLOSED — everything influencing the verdict rc (data AND
  control-flow) traces to the site's argv or a marked read, with no unmarked external
  input. Referent-agnostic: the closure reads MARKS + sh-structure, never tool
  semantics (`inv-referent-agnostic`). The closure pass is DEFAULT-DISQUALIFY (any
  construct off the audited pure-construct safe-list disqualifies) and fails safe
  (missed-safe loses an elision, never carries a hidden read). Substrate axes ONLY —
  user/identity is excluded, because a shift there changes ACCESS to the body's own
  reads (EACCES flips the answer of a structurally-closed body). netns: `net.*` is
  per-netns, so the model must forbid `undivided-by-transit-across netns` on network kernel state. The
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

## Invariants — host evidence & controller attribution

The controller↔host boundary: how bytes a managed host produced become facts the
planner may act on. Everything here binds the INTAKE edge, never the kernel.

- **rul-host-bytes-bounded-before-admission** — every byte a managed host produced
  (probe results on stdin or from a file, a replayed durable, tool errors, any future
  transport) crosses explicit aggregate-stream, line, record-count, field,
  retained-byte, collection-cardinality, and numeric bounds BEFORE UTF-8 conversion,
  interning, or any large allocation — byte-first reading with a hard
  `take(limit + 1)`-shaped boundary, checked arithmetic, and typed refusal on
  overflow or truncation. Known records use a closed grammar and narrow parsers.
  Unknown or malformed material may be RETAINED as bounded raw material and may never
  become an interned coordinate, identifier, path, template, shell text, claim key, or
  license input. A writer's cap never proves a pre-existing file is bounded: replay
  re-bounds independently, and raw blocks nested inside a durable consume both the
  outer and the inner budget. Limits are injectable policy, not timeless truth.
- **rul-admission-is-a-closed-outcome** — intake answers `Admitted` /
  `NoObservation` / `Refused(<closed reason>)`, and the three are not
  interchangeable. `NoObservation` is a well-owned attempt that produced no usable
  fact: ordinary conservative planning, the authored command retained or guarded.
  `Refused` is framing, bounds, attribution, or integrity failure: no plan carrying
  mutation authority is emitted for that attempt, and refusal returns BEFORE plan
  construction, artifact rendering, or durable writing. Never collapse both into
  `Verdict::Unknown` and continue — that silently converts a broken channel into a
  measurement.
- **rul-attribution-is-controller-minted** — host, target, attempt, oracle-source-set,
  and generation identity come from immutable controller-owned invocation context and
  are attached to accepted records. A payload frame may be CHECKED against expected
  controller values; it never mints them. Absent or ambiguous scope means no
  authority. Scope must survive every conversion and cache key. Today's spike is
  width-one and its scope types are deliberately private and unshared — the
  re-entry trigger is any second scope becoming representable at all (real transport,
  concurrency, retry, cross-host reuse, saved approval), which is when carrying the
  scope has to become checking it.
- **rul-integrity-failure-withholds-mutation** — this does NOT contradict
  `two-phases-opposite-fail-directions`; it carves a different input. ANALYSIS
  uncertainty resolves toward running the authored command (unsure ⇒ run, always).
  Lost ATTEMPT, TRANSPORT, ATTRIBUTION, or EXECUTION integrity is not uncertainty
  about the world — it is not knowing whether we are still talking to the world we
  think we are — and it withholds further mutation rather than becoming a universal
  "run". Malformed, truncated, ambiguous, or stale authority material must never
  round up to either verdict. Type the distinction; test it in every
  phase × user × oracle-reliability cell.
- **rul-fixture-identity-never-production** — fixed identifiers, fixed nonces, and
  FNV-style digests are deterministic spike drift-detectors and satisfy fixture and
  harness surfaces ONLY. They must be structurally unable to reach a production
  boundary — remote transport, concurrency or retry, saved approval, multi-host
  caching, default persistence, or anything published. Keep ONE named substitution
  point rather than copies scattered per crate. Likewise headerless/legacy-tolerant
  parsing stays behind compile-time test exposure; environment presence alone never
  grants parser authority. Comments are not a fence — absence of a constructor is.
- **rul-host-evidence-is-not-the-narrative-plane** — two unrelated planes both reach
  for the word "evidence", and merging them would be a genuine correctness loss. The
  DESCRIBE plane's collapse records are our OWN decision-inert narration of why the
  engine narrowed (`core::evidence`, renaming to `aid::Narrative` under `plans/288`).
  The INTAKE plane's host evidence is UNTRUSTED INPUT a managed host produced
  (`HostEvidenceLimits`, `read_host_evidence`, `ScopedHostEvidence`,
  `AdmittedUnscopedHostRecords`, all in `plan`/`cli`). One is what we say; the other
  is what we were told. Never alias, unify, or bulk-rename across the two — a
  workspace-wide rename must be module-driven, never grep-driven, and the intake
  family is out of its scope. Sanitizing or encoding intake bytes for display never
  makes them trusted.

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

## The authored surface (semantics digest; specs outrank: `plans/281` (mark grammar v0.2) · `277` · `273` · `274` · `275` · `plans/27C`; one-page dialect reference: `278`)

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
  per MATCHED invocation-shape, emitted `: disturbs KIND[@sel]`. `cmd__lend_map()`:
  the wrapper's dimension member — a bare `: lends DIM` (no value) = full lend; a
  valued `printf … : lends DIM` = mapped lend; a MISSING dimension = ⊤, walls (the
  enumerate-every-dimension law; absent-means-full-lend is REJECTED).
  `kind__resolve()`: entity canonicalization within its kind.
  `kind__disturbance_reaches_only()` (emits `: disturbs KIND`, the UNIFIED verb —
  `281` §5) · `kind__state_stored_only_in()` (`: stored-in <substrate>` emission
  lines + the `: undivided-by-transit-across <axis>` invariance mark; whole-member
  scope). Naming law: `only` in a role name = complete-by-contract, totalistic-
  survey-before-authoring; absence = arm-incremental.
- **coordinate-semantics** — the flat three-place `(kind, entity, selector)` +
  a context slot (recursive coordinate shapes DECLINED —
  `271:rul-coordinate-shape-flat-three-place`; structure lives in kind-owner
  functions BETWEEN coordinates). Kinds are reverse-DNS, ≥2 dots; the keystone
  disambiguator (`281:rul-verbs-dotless-kinds-dotted`) is that a verb is
  period-free and a coordinate's kind carries ≥2 periods. `@` introduces the
  selector, ATTACHED (`281` §R4, the highlight-safe respell of the old `#`;
  entity-less transitional `KIND:@SEL`). Polarity rides the verdict verb
  `asserts`/`refutes` (head sugar `:` / `:!`; observe = `reads`/`:?`), never the
  coordinate. The bare selector-less form permanently means whole-entity, and
  reads as ⊤-selector at consumers (collides with every cell). A coordinate names
  a CELL; names ≠ referents (aliasing is why resolve exists). Binds name entities,
  never cells (SOFT ruling). rc-arity (`281` §7): AT MOST ONE verdict per line;
  brace-alternation `@{a,b}` (attached) / `verb {a,b}` (whole payload) expands to
  N cells and is refused ONLY where it would forge a multi-cell verdict.
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

- **marker-gates-syntax-only** — `# dorc-lang/v0.2` (exact-match, stands alone,
  first ~10 lines) gates SYNTAX only — never `__role` name-recognition, never
  semantics. An unmarked file is plain sh; a marker naming an UNRECOGNIZED version
  fails loud, never a silent downgrade (recognized-set = {v0.2}, `281` §2j). TWO
  sanctioned comment-parses now exist (the `24M` one-parse limit LIFTED by the
  `281` §2j human ack): this version marker AND the `#:` mark carrier
  (immediate-colon, closed grammar, strip-if-valid) — neither is sidecar config
  (`KNOBS:kOOB`).
- **stability-ledger** (`276:rul-verdicts-never-stable`) — syntax = marker-gated ·
  `__role` names = permanent · verdicts = unstable-and-improving, disowned.
  **plan-as-API** is the named failure-mode: never promise cross-version
  plan-shape stability; anything like `--exit-code` gates on divergence-of-world,
  never plan shape.
- **strip-is-pure-erasure** — `dorc strip` erases binds + marks, erases the
  `dorc:` prefix, rewrites the shebang-runner; NO in-body name rewriting;
  `dorc-sh` row-three untouched. Per-carrier (`281` §9): a colon-form TRAILING
  mark region-erases to the block end (command survives); a colon-form STANDALONE
  mark line-deletes (region-deletes when it shares a line, e.g. a `case` arm); a
  `#:` block deletes ONLY when it parses as a valid mark-block, else stays a plain
  comment. Load-bearing subtlety: a bare-mark statement is an annotation-LINE
  (deleted whole, like a comment — NOT a POSIX `:` command); the author's last
  substantive command must remain the last status-affecting statement — marks
  erase to NOTHING (never a null-command), so no stripped-in `:` clobbers the
  tool-rc to 0.
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

## User-aid & diagnostics law (registry + laws: root `AID-NEEDS.md`; build phase: `27V`)

- **two-plane-aid-law** (`26C` §5b, human hard-ack) — the license plane fails toward
  unsureness; the aid/explanation plane fails toward narration with attributed
  confidence. Aid-evidence is decision-inert at the TYPE level (sealed; no conversion
  into any license-plane input compiles); license values flow into evidence freely,
  never back. Lint-clean licenses nothing.
- **collapse-mints-evidence** (`AID-NEEDS:law-collapse-mints-evidence`) — every
  safety-narrowing (meet-to-⊤, refuse, decline, wall, demote, cancel) mints
  decision-inert evidence carrying the collapse's OPERANDS, demanded by the collapse
  constructor at the VALUE level (pure data; kernels stay pure — arena registration is
  post-pass per `22D`). Evidence is Eq-EXCLUDED from lattice equality (fixpoint
  termination, `22W` §2) and k-capped. `Unexplained` is constructible but renders
  self-advertisingly.
- **trust-tier-is-syntax** — the epistemic tier of every rendered link (STRAWMAN
  spellings: measured / vouched / ran / claimed / derived / consented) is a typed
  evidence field rendered uniformly by arrangement code; prose fragments never
  hand-write epistemics. The tier SET and its typed rendering are the law; the words
  ride render-form-unwelded.
- **render-form-unwelded** (`27V:rul-output-form-unwelded`) — the particulars of
  rendered aid output (wording, numbering, connectives, arrangement shape) are
  unstable-and-improving pending real-world generated output; `KNOBS:kFLOW` governs
  the mechanism-vs-polish resting point. Goldens pin content + structure and re-bless
  freely on arrangement churn; never treat a current render as contract (the
  plan-as-API failure-mode's aid-surface cousin).
- **one-catalog-no-legacy** (`27V:rul-kill-legacy-diagnostic`) — the structured
  `DiagCode` catalog is the ONLY diagnostics mechanism; the legacy string-slug
  `Diagnostic` is being removed. Never add new emissions to it.
- **replay-editability-is-provenance** (`282:rul-replay-editability-is-provenance`)
  — every errorloom replay is an arbitrary command plus exact result bytes. Prose is
  editable ONLY when the embedding consumer's driver returns typed `EditableRender`
  provenance for that exact invocation and result. Command names, flags, output
  formats, JSON/prose shape, prefixes, skeleton similarity, and `{{...}}` bytes grant
  no edit authority. Driving an invocation and exposing editable prose are separate
  capabilities.
- **replay-executor-ownership** (`282:rul-generic-executor-consumer-dispatch`) —
  generic errorloom owns the consumer-neutral driver/result API and reusable
  controlled shell/process executor; it knows no Dorc names, formats, or template
  policy. The embedding consumer owns exact-shape in-process dispatch and explicitly
  chooses whether a decline reaches generic execution. Generic fallback returns
  testable bytes only unless an explicit transformation-aware driver preserves
  provenance. A pipeline such as `dorc plan --format=jsonl | jq --pretty` therefore
  carries no edit authority by default.
- **defining-case-catalog** (post-`282`-flip) — every code has exactly ONE defining
  case; the **committed transcript CASE is the authoring surface** and the compiled
  generated catalog lock/table is DERIVED from it (`282:rul-transcript-is-the-
  authoring-surface`). Cases live at `crates/dorc-loom/cases/<slug>.loom` (txtar +
  flat-YAML frontmatter). The phase-two compiler is split by ownership: errorloom
  transports renderer-stamped editable sections and opaque variable identities;
  dorc-loom alone parses strict whole-token `{{name}}`, resolves it against the
  current typed payload, and compiles catalog fields. The old tagged-region/
  param-table/re-holing/prose-bless path is dead; never restore it. Durable promotion
  is LANDED (phase three, `notes/287`): `dorc-loom compile/promote` over a
  content-bound receipt, publishing the wholly generated `core/src/catalog_lock.rs`
  plus affected cases. All candidate bytes and both fixpoints precede publication;
  final per-target temp-file replacements are not a promised crash transaction
  (interruption is loud in git, repaired by rerun). The legacy `DORC_CATALOG_PROMOTE`
  writer is retired; hand-edits to the lock are refused or caught. The build parses
  the catalog and never auto-tracks case files (the lag IS the assertion).
  `message`/`help` are
  `Option<&'static str>` (`None` renders the `[unwritten:]` placeholder at render
  time, never a stored string); interpolated values use engine-owned canonical
  formatters; sibling codes come from world-state/license variants ONLY, never
  grammar-fit (`AID-NEEDS:law-codes-vary-by-world-not-grammar`). Both fixpoint gates
  are live: errorloom render-level + Dorc's generated-lock byte-identity gate
  (`dorc-loom --test fixpoint`).
- **error-authorship-tier** (human-typed 2026-07-18) — builders mint codes and
  defining-case structure with EXPLICITLY-EMPTY prose blocks (rendering greppably as
  unwritten); prose is a conductor/human act issued from the builder's when/why/how
  report. Never ship builder-authored error prose.
- **whylog-write-only-replay** — the whylog thin durable (invocation record · records
  stream incl. the report lane · decision digest · apply report · seed) is write-only
  and replay-driven (`dorc why --last` re-derives through the same kernel); never a
  cache (rec-5); contents are host-metadata-sensitive — the secrets round owns that
  work; do not widen contents casually.
- **decline-class-emission** (`27W`) — an oracle classes a deliberate decline by a
  plain-sh emission ON the declining path: `printf '<verb> <class> <tail>\n'
  >>"${DREP_V1:-/dev/null}"` (sink name STRAWMAN; the `:-/dev/null` default makes the
  idiom total off-Dorc). Verb + class vocabularies are engine-owned, append-only
  (v1: verb `decline`; classes {unsound, unmodeled, interactive, hazard} — starter set
  pending typed ack); unknown verb/class degrades to a generic note, never an error.
  Static-first: per-arm inventory always; per-site class when argv threads; the
  runtime emission is the only-opportunity fallback, deduped (site, arm, class).
  Runtime capture is LIVE, on a channel the controller exclusively owns: an artifact with any
  emitting check opens ONE per-attempt scratch directory with `mkdir -m 700` (rooted at a
  controller literal — never a host environment expansion), binds `DREP_V1` to a per-site file
  inside it, drains that file into `report` records, and unlinks per-file before an empty-only
  `rmdir`. A failed create EMPTIES the guard variable and the lane degrades to `/dev/null`,
  never retrying, never removing what is already there, and never failing a plan or an apply.
  The static tiers (per-arm inventory, per-site classing) are unaffected either way, and the
  authored idiom never changes because the sink VALUE is engine-supplied. Never restore a
  pathname-based capture protocol: the probe may not create, truncate, read, or unlink a host
  pathname it merely named (`rul-probe-writes-only-what-it-owns`).
  Classes route AID only — the rc-partition stays a flat sink; the license plane
  never reads a class. Silent declines stay legal; classing is enhancement.
  Noise-tolerant (`27W:rul-report-noise-tolerant`): ingestion never silently drops
  author emissions — unrecognized/free-form lines are retained (sanitized, capped,
  attributed) and print in full at max verbosity; default surfaces stay selected.
- **report-lane-versioned-entry** (`27W:rul-versioned-entry`) — the report sink's env
  NAME carries its format version; a new format mints a new name; recognized sink
  names are permanent once published (the `__role`-name posture).
- **report-surface-massaging-carve** (`27W:rul-report-surface-massaging`) — report/why
  surfaces may re-emit massaged code excerpts (contributing-lines slice, whole-comment
  attachment, marked elisions): authorship-implying, repair-directing, never
  byte-obligated. Binds ONLY the report/why render plane — the artifact byte-floor
  (two-surfaces) and the executable-plane never-synthesized-sh law are untouched;
  display-sh must never masquerade as runnable.
- **error-prose-conductor-flow** (`27U` §4/§5; `282`-flip retired the roster,
  `28A` §2p) — catalog prose is three-state: `sm `-prefixed migrated-verbatim
  builder text (awaiting human rewrite) · `[unwritten: <slug>]` (`message: None`,
  awaiting conductor/human prose) · unprefixed prose for a **case-owned** code (a
  `dorc-loom` case file exists for its slug). The `CONDUCTOR_AUTHORED` roster is
  GONE; enforcement is `message_registers_are_sm_or_unwritten` re-keyed to
  `is_case_owned(slug)` + the two fixpoint gates. Builders author ZERO user-facing
  strings, ever; prose is a conductor/human act (`27V:rul-error-authorship-tier`),
  authored at the transcript surface (looking at the rendered case) or, still
  sanctioned, by direct catalog edit from the structured metadata — promote-v2
  carries it behind the fixpoint gate, orchestrator-only. Prose burn-down is
  LAZY-by-design (`282:lean-machinery-now-prose-lazy`); `[unwritten:]` is a legal
  resting state (the prose-quality sprint is human-owned).
- **rul-chain-is-pull-only** (`27U` d4a) — the full numbered why-chain renders only
  on pull surfaces (`dorc why N` live / `--last`); plan stderr keeps compact
  attribution lines. Push stays ruthlessly selected even under the spike's
  kWARN tune-high.

## Where the build stands (dated 2026-07-21 — the one drift-expected section; `Research/LIVING_STATUS.md` is the live view)

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
- LANDED (2026-07-19, the user-aid build phase — `notes/27U` is the ledger;
  branch `ai/r27-aid` awaiting the human fold): legacy `Diagnostic` dead; the
  ONE catalog (const-table + three-state prose + defining-cases 17/52 with the
  shrink-only ratchet + the promote fixpoint tool) · the sealed evidence plane
  minted at every collapse class · the whylog durable + `dorc why --last`
  replay · the `27W` report lane's static tiers + ingestion + the runtime drain, the
  last re-landed on an exclusively-created scratch directory (decline-class-emission
  above; no e2e case renders a drained probe yet) ·
  minting-line/file:line attribution end-to-end · the arrangement walker + THE
  FLAGSHIP GREEN (the naked-trust chain, live and replayed) · lint absorbed
  (one severity vocabulary; `dorc_oracle::validate` book-free;
  rung-oracle-solo) · caret plumbing + multi-line frames · `conduct-bless.sh`.
- ERRORLOOM FOLLOW-ON: phase-two editable transport + strict Dorc template compiler
  are accepted on `ai/r28-errorloom-phase2`. Phase-three Unit 1 is in REWORK on
  `ai/r28-phase3-unit1-cli`: its command surface/inventories remain useful, but
  content-skeleton replay selection is rejected. NEXT is the consumer-neutral
  replay-driver/result seam + Dorc exact-shape dispatch, then receipts/promotion;
  durable handoff `28A`, design authority `282`.
- HORIZON AFTER the errorloom follow-on: `270:block-stdlib` under the human-led
  conductor (`27Q` on-ramp; the minting-line precondition is now DISCHARGED), then
  the field-trial revival.

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
- **real-tools-lane-opt-in** (human-authorized 2026-07-18) — the ONE sanctioned
  real-external-invocation lane: `DORC_E2E_REAL_TOOLS=<comma-list>` (default
  UNSET ⇒ zero external invocations, zero real-tool PATH probes) runs the lint
  adapters against REAL, mise-pinned, read-only linters (shellcheck,
  checkbashisms) over repo-local fixtures only. A listed-but-absent tool FAILS
  the lane loudly (opt-in implies require-tools). Never default-on; never
  golden-pins upstream message text (structural + stable-code assertions only);
  never touches files outside the worktree. Real mutators remain forbidden
  everywhere.

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
  `IMPLEMENTATION.md`, `TODO.md`, `AGENTS.md`, root `CLAUDE.md` — surface problems
  upward instead. The LLM-maintained root docs (`USER_STORY.md`, `KNOBS.md`,
  `ANALYZER-NEEDS.md`, `AID-NEEDS.md`, `TODO-ADDTL.md`) are edit-and-commit for
  in-place human review (the human deletes what they disagree with); keep them
  living, task-focused, and free of session chronology ("updated by X during Y"
  never appears in them — conductor-tier edits only, not casual subagent edits).
- **Never read** `Research/notes/quarantine-DO-NOT-READ/` (including spike2 code)
  or `Research/corpora/` unless the orchestrator explicitly hands you a pointer.
- Spike design notes (what strained, where, confidence-marked) go in
  `Research/notes/` as numbered notes: append-only, new note per chunk, never edit
  a prior note; check the tree before minting an ID.
- Commits: small + granular + frequent. `(AI <labels>) terse one-line message` per
  `.gitlabels`; the `AI` label is mandatory; no `Co-Authored-By` trailer; never
  push. Run the four gates first.

- `rul-strawman-formats-no-compat` — pre-user, EVERY versioned wire/format/env
  name (`dorc-lint-format/1`, `DREP_V1`, `dorc-whylog/1`, `dorc-records/1`, …) is
  strawman: rename/reshape in place, all sites in one commit; never an adapter, alias, or
  mapping from a historical spelling. "Permanent once published" clauses activate at
  publication, not before. Applies generally; *ask* the human if you suspect
  they want to pay the prices of backwards-compatibility over velocity/simplicity.

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
  rip-don't-update + a hard numeric budget + the counting command; structural
  banners in large data tables are noted separately, not billed).
- **worktree-file-access-law** (`27U` §2 — two incidents, one root cause) — a
  worktree agent's every Read/Grep/Edit/cite lives under its OWN worktree; the
  primary checkout is radioactive for ANY access, read-only included; re-verify
  `pwd` + branch before every commit.
- **map-then-execute-split** (`27U` §4) — big-bang dispatches split map-and-rule
  (proposal + conductor rulings + a mechanical spec) from execution (fresh
  budget, zero re-derivation); the checkpoint between them is where conductor
  review catches grounding errors and license bugs cheaply.
- **foreground-final-verification** (`27U` §2) — a builder's FINAL verification
  runs foreground; ending a turn awaiting your own backgrounded task strands
  the lane (the wake-up goes to the conductor, not you).

## Confidence + reference discipline

- Mark uncertain claims: `+SURE` / `~SUSPECT` / `-GUESS` / `--WONDER`.
- Slugs: hyphenated, full English words, HARD MINIMUM three words
  (`finding-guard-bytes-travel`, never `q-2`/`W3`); outside-document references
  carry the minting doc's ID (`docID:slug`).
- Reuse `KNOBS.md` and existing corpus slugs rather than re-deriving a tension or
  concept under a new name; unroll opaque legacy labels at point of use.
