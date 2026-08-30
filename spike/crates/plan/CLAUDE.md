# spike/crates/plan — CLAUDE.md

Role: elision/replacement, the probe→apply compiler, and render — the crate where
irreversible verdicts are MINTED. Owns `PhasedVerdict`/`Bias`, the license
witnesses, `Disposition`/`Step`/`Plan`, `compile_probe`/`ProbePlan`, `build_plan`,
and the renders. Read `spike/CLAUDE.md` first; root `DESIGN.md` (plan/apply UX)
and `IMPLEMENTATION.md` ("To execute, or not to execute?") outrank `Research/`.
Registry discipline: one rule per bullet, slugged; append to the matching section.

## Law — the license mints (the sole minting sites for the two irreversible verbs)

- **sole-mint-witnesses** — `ReplaceLicense` (elide) and `GuardLicense` (guard)
  are private-field, sole-mint witnesses; BOTH demand a
  `core::claim::ByVouch<VerdictVouch>` **by value**. If a mint signature blocks
  your build, you likely lack the real vouch: obtain it (author the
  `is_converged()`) or let the command run — NEVER fabricate or convert a claim
  to satisfy the type. That conversion is the vouchless-elide hole the weld
  closes. Since the influence-carriage fold (r30, `30Qd`): all EIGHT licence mints
  (`prove_replaceable` · `prove_query_replaceable` · `prove_members_replaceable` ·
  `prove_shared_region_replaceable` · `prove_inline_replaceable` ·
  `prove_inline_query_replaceable` · `GuardLicense::mint` · `mint_for_shared_region`) take an
  `InfluenceAccount` beside the vouch and join it into the licence; `Plan::decided` carries
  the join of every record `project_plan` read; `certifier_trip`'s demotion is a RE-MINT that
  joins the trip witness into the record's own account, never a field poke
  (`core/CLAUDE.md the-influence-account-is-carried-never-stamped`). `SpineLoadDecision` is
  minted at the PRE-INTAKE seat (`mint_load_decisions`; `record_new_arm` only transcribes), so
  its authored account is a fact about where the seat stands, never an asserted label.
- **aggregate-mints-carry-the-same-demand** — the demand follows the MUTATION,
  not the node shape: an establish erased inside a member-loop or inline-call
  aggregate consumes its own reached vouch exactly as a standalone one does.
  `AllEstablishesVouched` is the private, non-empty (head+tail) proof; its mint
  takes the exact ORDERED `(site, fact)` list and rejects missing, extra,
  duplicate, reordered, wrong-site, and wrong-fact vouches — the whole aggregate,
  atomically. One private `AggregateEstablishes` value is the shared identity for that
  vouch proof AND universal effective freshness: every member independently crosses every
  external wall and passes reference re-derivation; `AggregateSurvivalWitness` must match the
  vouch receipts exactly before one atomic replacement mints. Any member failure rejects whole;
  the representative fact is display-only. The same exact population selects probe bodies:
  only an all-vouched mutation population ships reached verdict bodies as its ordered
  measurements; if any selected verdict cannot ship, the whole population is unresolvable.
  Predictions may still measure a population that cannot replace; query-only records remain
  independently predict-sourced (`30La`). Query-only bodies prove `ReadSubstitutionProof`
  separately and must NEVER manufacture a vouch to share an API. Keep both types and both provers
  private: the doctest in the crate doc pins that, and exposing either "for a
  test" is the regression it guards.
- **vouch-built-once** — vouches are built once (`build_vouches`) and threaded
  via the `Vouches` map; a vouch informs a license and never enters the
  fact-plane.
- **elision-predicate** — elide leaf L iff `probe(L.fact) = Converged ∧ ambient ∧
  Must ∧ no-consumed-unvouched-observable ∧ ¬⊤-contained`. ⊤-containment is a
  SEPARATE guard in `build_plan` — it guards a different failure (unmodeled
  execution context, not a stale fact); keep it separate.
- **cant-probe-cant-elide** — a kind with an effect but no shippable check is
  absent from the probe, so the apply runs it (`kFAIL-perform`). A missing or
  unknown verdict always folds to run.
- **no-rc0-vouch-exists** — "converged ⇒ rc 0" was bought false three times; a
  mutator's rc is ⊤. Never resurrect an establishes-contract rc-0 vouch.
- **consumed-channel-blocking** — consumed `Stdout`/`Stderr` arrive un-collapsed
  and can only BLOCK (`inv-must-may`); consumed Status follows the trichotomy
  (`StatusRelaxable` substitutes an exact known rc, ⊤ blocks; `StatusInvariant`
  never blocks, still recorded, mark-union — any other blocking mark wins;
  `StatusIterated` blocks unconditionally).
- **ship-seam-reads-the-lane-not-the-kind** (`26H` §3.5; `28Q` §4
  rul-verdict-primacy-at-the-ship-seat, BUILT at stage-0, 2026-08-16) — `compile_probe` takes TWO ship
  closures, and the verdict-body one is gated on the caller's per-SITE verdict-lane set:
  never on the fact's kind, never on try-order. Every vouched, mutation-capable site is
  verdict-lane: the verdict body IS the probe check, its own reached answer is the
  convergence measurement, and it PRECEDES the predict lane — shipping a resolvable
  predict there would measure a different cell than the record keys, and prediction never
  licenses elision. A predict ships alone only where elision is statically unavailable;
  its cells feed the static concern topology whatever ships. The vouch gate stays on the
  verdict branch alone (a declined argv has nothing to measure and must ship no
  record — `guard23-refusepath-rc0-never-passes`).
- **erasure-demands-a-proof-and-a-rendered-death** (`26H` §4 — W-C; `erase.rs` is THE transform
  where this can be wrong, and its tests are the review's centre of gravity) — `prove_dead_branches`
  is the sole mint of a `DeadBranchProof`, and it demands FOUR things, not one: the fold proved the
  site unreachable from a KNOWN controlling status; the site actually gens into reach (the caller's
  invalidator set — never `Disposition`, because omitted-for-any-other-reason is not dead); the site
  is not floored into running anyway (in-loop, ⊤-successor); and the controller will really be
  SUBSTITUTED AWAY. That fourth is the wrong-yes fence — a fold-`Omit`ted leaf whose controller is
  not neutralised renders VERBATIM and runs behind the live guard (`is_neutralised`,
  `omitsafe21-heredoc-guard-keeps-body`), so erasing it would license downstream elisions off a
  mutator the artifact still executes. It is computed from analysis data and the fold input alone,
  through the shared `query_substitutes` seat, so no OUTCOME becomes a premise
  (`pin-no-outcome-as-generator`). Its refusal check is `leaf_has_heredoc` and ONLY that, matching
  `is_neutralised`'s `Replace` arm: `leaf_has_blocking_output_redirect` is the GUARD tier's refusal
  and including it would refuse `cmd >/dev/null 2>&1 || mutator`, the exact ladder idiom the
  fixpoint exists to fix.
- **erasure-is-records-grounded-only** — a statically-known controlling rc (empty list, bare
  assignment, funcdef — all rc 0 in the fold) is SOUND but is not a measurement, and the ledger's
  name promises records. Those branches keep today's behaviour; widening to static deadness is its
  own future design with its own name, never a quiet relaxation of this predicate.

## Law — guards

- **guard-shape** — `( <verdict-fn> <site argv> ) || <original bytes>`; the
  original bytes always survive verbatim; failure lands on run. Declared-dual
  glue is the engine's mechanical sense-flip
  (`( f args; [ $? -eq 1 ] ) || <bytes>`) — lossless inversion.
- **guards-mint-no-values** — a GuardInsert carries no StandIn, no Predicted, no
  Observable: on pass, the check's live rc is the line's rc; on fall-through the
  original runs genuinely. Licensed by the vouch, never by value-provenance.
- **never-synthesized-never-mutating** — never engine-synthesized sh; never
  declared/claimed output in guard position; a body that provably mutates lifts
  nowhere (`271:rul-no-mutating-guards`).
- **pinned-definitions-are-the-artifact's-binding** (`28K` §4
  `rul-runtime-resolution-never-load-bearing`) — `Plan::pinned_definitions` decides, for the whole
  artifact, which body each guard invokes and under what name; the render consults it and nothing
  re-derives a binding at runtime. A misalignment there could swap WHOSE judgment executes, which is
  pope-sin tier (`271:rul-sin-ordering`), so it is structural rather than three mechanisms agreeing.
  Since the emission stage: the SNAPSHOT (helper declarations) hoists ONCE, deduped by
  declaration site, ahead of the role bodies; a vouch carries `closure` and `body` separately
  and a body owns only its own bytes — which makes the munge a header-only edit by
  construction, and made ALREADY-IN-PLACE answerable (pre-split, the blob comparison never
  matched and a copy of the BOOK's own body hoisted above the book — corpus-reached, pinned
  twice). Whole-artifact DEFENSIVE emission (every emitted name munges) triggers on real
  definition vectors only (`oracle/CLAUDE.md a-top-reject-is-not-a-definition-vector`) —
  and per `spike/CLAUDE.md rul-happy-path-is-a-closed-set`, every idiomatic tier above the
  defensive floor is licensed only by PROVEN enumeration, never assumption.
  `PinnedDefinitions` is SPLIT along the line a second artifact form forces: `invoked`
  is the DECISION (which body a guard calls, under what name — what the Spine records,
  and where a misalignment is pope-sin tier), `definitions()` is the ordered form-neutral
  material, and `hoisted()` is one form's sh typesetting over it. A form that lays its
  dependencies out differently re-typesets from the same bindings rather than
  re-deriving them.
  Three rules in order: CONTENT-DEDUP (byte-identical bodies are one definition); ALREADY-IN-PLACE
  (a body the book's own text defines at top level with the same bytes is not copied — the
  EMITTED preamble never carries two same-named funcdefs, which is what dissolves the
  `reserved.rs` tension; a regional BOOK definition may share a preamble body's name, because
  book bytes are untouchable and sh scoping binds the region to its own body —
  `frame30-subshell-body-answers-inside-only` is the pinned case (`308:cr-artifact-two-funcdefs-letter`);
  the positional regime guarantees a book-sited definition PRECEDES its guards, so nothing
  is re-derived); BARE-IF-SINGLETON (over emitted material ∪ the book's top-level funcdef
  names); HASH-MUNGE (two distinct bodies under one name each emit once as
  `<name>_h<digest>`, digest over the definition BYTES). The retired dedup-by-funcname emitted the
  first body and let both sites invoke it. A munged name cannot parse as a `__role`, so a
  re-ingested artifact reads the guard as an opaque call ⇒ conservative run (`23A:P-reingest`).
  `Plan::render_sh` is the flat DST render and emits no preamble at all — never wire a second
  binding authority there. A definition's PLACEMENT is INHERITED from the source it was
  authored in, never assumed (`30Qb:rul-a-loaded-definitions-placement-is-its-load-position`):
  `Plan::decided` demands an `ArtifactEmission` — the settled form's carriage account
  (`PlacedSources`) plus its import edits, as ONE value — and `pin_definitions` files each
  closure declaration by its `ClosureDecl` key's source and each role body by its vouch's
  defining file. A `--pre-source` root is AMBIENT and hoisting it is faithful; a source a book
  `.` reaches is already carried at that `.` since the bundling
  (`30Ng:rul-bundle-at-dorc-lang-boundaries`), so the preamble adds nothing and a second copy
  would bind names at lines the authored program does not — for VARIABLES as much as funcdefs,
  since the closure track carries file-level constants (the landed double-carry defect). A
  source the form carries NOWHERE places nothing — placement, not the vouch: the guard, if
  any, resolves through the author's own `.`, which is `30P:rul-guard-resolves-like-its-mutation`
  exactly. `PinnedDefinitions::hoisted()` is retired for `typeset(&Placement)`: a form asks for
  the material AT a placement rather than assuming one. Every emitted name is minted through
  `placement::EmittedNames`, which holds every name the book names and lengthens the digest
  until the candidate is free — `<name>_h<digest>` is computable from the artifact, so it is
  squattable in plain sight rather than colliding by accident
  (`30P:rul-munged-name-lifts-over-opaque-load`).
- **the-pinned-unit-includes-the-closure** — what a guard's preamble ships is the stripped definition
  PLUS `dorc_oracle::closure`'s prefix (helpers + file-level constants). Two riders: a CONTESTED
  closure withholds the VOUCH (no guard, no elide, the site runs), and the vouch's `check_cmds` —
  the dual-rail `guardcmd` allowlist — must cover the closure's own commands, because once a helper
  travels with the definition the real check-command lives in the helper.
- **check-tax-awareness** — a guarded site pays its check on every apply,
  forever (`KNOBS:kPROBING`): an expensive check must earn its vouch or
  just-run. Planner economics, never a license question.
- **certifier-trip-cleanup-runs-in-every-driver** (`302:rul-certifier-trip-guard-only`)
  — `certifier_trip::demote_on_trip` runs immediately after `build_plan_walled` in
  EVERY plan-producing driver; the censusless producers reach it through the one seat
  `certifier_trip::project_censusless`, and `every_plan_producer_spends_its_certifier_trip`
  is the two-way lexical roster that makes a FIFTH producer a diff rather than a silent
  omission (four had already forgotten — `30Md:fnd-discarded-trip-retains-elisions`).
  The must-remember surface is DISSOLVED: `project_plan` demands a
  `certifier_trip::TripSpent`, whose one mint is `spend_certifier_trip`, which cannot be
  reached without a `CertifierTrip` in hand — so a producer that never spent its latch has
  no projection to call. The lexical roster stays as belt-and-braces, because it binds a
  different thing: a producer that builds a walled plan and hands the Spine somewhere else
  never reaches the projection seat. On a tripped run,
  Replace and Omit demote to run (`DemoteTag::CertifierTripped`, a reason-enum arm,
  never a sibling code); guards STAND only on the syntactic occupancy-1 census over
  `DefinitionTable` — the census consults NO solve, and that independence IS its
  admissibility (a trip disqualifies solver and certifier together; nothing either
  touched may testify). Runs run; the mid-pipeline §3 floors are untouched and
  still fire in place; no recovery, no carves, no re-planning.

## Law — the survival tier (the design's ONE naked-trust cell)

- **survive-license** — an elision kept past a RUNNING wall requires: the vouch
  + footprint×backing provably-disjoint (through the core chokepoints only) +
  the admin's `--risk-faultless-skips` (per-invocation, never a default).
  Keying and hint-lane values NEVER feed it; the flag permits acting on
  separation claims, never manufactures them.
- **universal-meet-here** — sparing over backing-SETS quantifies universally:
  any unknown member ⇒ collide, at every iteration, order-independent.
- **attribution-rendered** — every survival renders its full attribution chain
  (whose claim licensed it, line-level first link); the why-lens names them.
- **empty-world-byte-identical** — no oracles loaded ⇒ output byte-identical;
  rung-0 pin in every brief touching this crate.
- **an-at-most-claim-has-two-atomicities** (`28P:dec-whole-body-atomic-refusal`) — a derived
  footprint must prove BOTH that its record stream arrived whole AND that the body which wrote
  it finished. They are independent and only the first reads like a gate: `deriv-end n=<K>` is
  counted by the SCAFFOLD from lines received, so a body that emits three coordinates and dies
  on an unbound helper closes at `n=3` and agrees with itself — transport intact, survey false,
  at-most claim wrongly NARROW, and narrow SPARES MORE (measured wrong-elision). So the
  scaffold captures the emitting body's status BEFORE the record pipe (a pipeline's status is
  its RHS's, which is why the body's was previously unreachable) and carries it as
  `body-rc=<R>`; non-zero refuses the WHOLE family ⇒ the site walls total. Never file a
  body-death under the transport code `deriv-family-incomplete` — the stream was perfect, and
  saying otherwise mis-attributes (`271:rul-sin-ordering`). Two things this is NOT: a verdict
  rc (`rul-rc-partition` binds verdict functions; this is a binary did-the-body-finish, spelled
  `body-rc=` so it can never be read as the site record's `rc=`), and a completion signal —
  a body that truncates and exits 0 stays invisible to `body-rc`. That witness is now RULED
  (`plans/30U` §5; `ANALYZER-NEEDS:an-atmost-completion-signal`): the authored tail-position
  `disturbs nothing-else` report-lane record — mandatory where licensure rides runtime
  emission (dynamic `disturbs` bodies; the finished-definition act in
  `disturbance_reaches`), absent by design for collide-only emissions. Build rides the
  `30U` §10 sketch, unscheduled.
- **rederivation-is-demote-only** (r30; `notes/300` §2b) — before a plan ships, every
  standalone OR aggregate-member survival re-derives through the naive reference model
  (`dorc-sparing-reference`) inside effective freshness, before any replacement witness
  mints (a post-pass demote would let a now-running site cast no wall downstream). The minted
  `SurvivalWitness` goes in BY VALUE and comes back `Confirmed(witness) | Demoted(..)`
  — the re-check cannot fabricate a witness, agreement licenses nothing new, and the
  adapter never touches the production compare path (zero shared helpers; lexically
  fenced both ways). The differential's one disclosed coverage limit: the backing-side
  dialect-membership conjunct is adapter-computed, not model-re-derived
  (`sparing_differential.rs` header).

## Law — durable replay

- **inv-reingested-material-never-authorizes-action** — every durable read-back value is a
  `Recorded*`/report value with no conversion to live claims, influence accounts, vouches,
  licenses, `PlanAuthority`, probing, artifacts, or apply. Re-ingestion describes a past
  world-moment and never drives action (`30R:standing-invariants`).
- **inv-recorded-and-rederived-remain-distinct** — durable consumers preserve four states:
  recorded-only, rederived-only, both-agreeing, and both-disagreeing. Never substitute one for
  the other or silently resolve disagreement (`30R:recorded-versus-rederived`).

## Law — render

- **the-render-decides-nothing** (the `30E` render-decision audit, closed by `30Nd`) — every
  render-time answer is taken ONCE, at `Plan::decided`, from the settled dispositions: which
  body a guard invokes, which licensed edits the span render refuses, which `Omit`s have a
  neutralised controller, which regions are still live, and the whole-artifact
  defensive-emission regime. `Plan::decided` is the only constructor and `render` is private,
  so a plan whose render is undecided is unrepresentable; `render_apply` and the three
  disclosure surfaces READ the plane and decide nothing. A choice stays render-side only when
  neither `dorc why` nor a second artifact form could ever need to account for it — the
  elided line's commented-original wrapping is the exemplar (one `Replace`, two byte-shapes,
  same observables). `project_plan` records what it decided in the same act, so a projection
  whose render decisions nothing wrote down cannot exist. A refused edit carries BOTH
  identities and exactly one is populated: `RefusedEdit.leaf` for an execution,
  `RefusedEdit.region` for the one authored edit many executions share
  (`30N:rul-region-refusal-discloses-region-keyed`). The diagnostic surface reads
  `render-heredoc-refused` only for leaf refusals; shared-region refusals remain region-keyed
  narrative records and are NEVER smeared across their contributing invocations, which would
  report N refusals for one edit and point N readers at calls that did nothing wrong
  (`271:rul-sin-ordering`). An IMPORT EDIT (`30Ng:rul-bundle-at-dorc-lang-boundaries`) is
  decided like every other render answer: `plan::ImportEdit` is an INPUT to `Plan::decided`,
  settled before the plan exists from authored-before-contact inputs, and it reaches the
  artifact bytes, the plan surface (`plan-import-rewritten`) and the decision plane
  (`RenderDecision::ImportRewritten`) from that one decision. There is no emission-time
  substitution, and there must never be one: the whole reason the rewrite is a narrow
  grant is that no use of it is silent.
- **no-specialized-shell** (`30L:rul-edit-authored-definition-once` ·
  `pin-no-generated-specialization`) — a shared region's edit lands ONCE, at the authored
  function-body span; definitions stay in place and calls stay calls. Never a per-call
  specialized body, a cloned or renamed helper, or generated argument dispatch — there must
  always be one authored line, by one human, answerable for anything that runs. A shared GUARD
  therefore carries the region's SOURCE-level argv (`install -y "$1"`), never a site's resolved
  operands: positionals re-bind per invocation inside sh, and a per-call literal installed into
  shared source is a check about the wrong operand at every other invocation. Whole-helper
  elision stays DERIVED: when every invocation is itself neutralised the body executes on no
  route, so `Plan::live_regions` drops the edit and the inert definition keeps its authored
  bytes — conservatively, since a capped route account cannot answer "every invocation".

- **ap-2-runnable** — `render_apply` must emit runnable, `sh -n`-clean POSIX;
  acceptance executes or `-n`-checks the artifact, never text-diffs alone (the
  historical trap: a non-runnable empty `then`-clause shipped green, twice).
- **attention-honesty-here** — the render is the whole book, original order; a
  may-execute line is never hidden (at most dimmed). Span-exact substitution
  (the leaf's byte-span is the edit unit); a leaf the span render cannot safely
  edit (heredoc-carrying) REFUSES its license at render time, loudly.
- **display-only-resolution** — resolving interned tokens to text here is for
  display/provenance only; never branch on the resolved text.

## Direction

- **re-key** — the elision predicate goes per-selector/per-cell at the
  entity-algebra-rebuild; ALL coordinate/selector comparison through core's
  chokepoints, never inline.
- **one-settlement-one-world** (`30K`, BUILT) — every apply-time answer derives from one
  grow-only settlement (`plan::settle`) over one fact: which mutations may ACTUALLY
  execute. A round applies the ledger, re-derives the model, solves `ReachingWalls`, folds
  the frozen records through the validity that reach implies, decides every site, and
  proves what cannot execute; a growing round discards every provisional product. Only the
  quiescent round — sealed by a `Quiescence` the ledger alone mints — writes Spine, and a
  `ProvisionalEffectiveRound` has no Spine API to reach for. Never add a second settlement.
- **acts-and-dispositions-mint-together** — `decide_site` returns BOTH the `Disposition`
  and the private `EffectiveAct`, from one pass over one set of conditions. There is no
  `From<Disposition> for EffectiveAct` and there must never be one: the act is the other
  half of the conclusion, not a reading of the outcome (`pin-no-outcome-as-generator`).
- **only-a-proof-retires-a-wall** — a `Guard` walls exactly like a `Run` (its untouched
  fallback is the authored mutation), and a `Replace` the RENDER will refuse walls too. A
  wall is retired ONLY by a `NoMutationProof` in the ledger, and the two species reach
  different consumers on purpose: a DeadBranch shrinks the analyzer's effect model, a
  Replaced one must not (that spelling would also destroy the site's own class).
- **no-visible-wall-bookkeeping** (`27C` §5, HUMAN-TYPED 2026-08-19) — Guard is
  conservatively a possible mutator downstream. Never emit wall flags,
  conditional-tail blocks, or controller bookkeeping into reviewed `plan.sh` to
  recover precision; downstream lines use only ordinary guard/run forms. Probe
  artifacts may use private machinery because they are not the approval surface.
- **region-decisions-meet-universally** (`plans/30L` §5) — `plan::region` groups per-instance
  answers by the authored span they would edit and meets them: Replace → Omit → Guard → Run,
  universally quantified, semantic (never tag-level) equivalence, biased to Run. Nothing
  branches on route-set cardinality; an `Open` population runs without consulting a proof; and
  a proof list that does not correspond exactly to the census's population runs. The decisions
  are consumed by `plan::settle`, which is where the shared license mints. Guard economics ride
  the POPULATION, never one route: candidates drop before the meet unless some route measured
  Converged (a guard whose check fails at every invocation is pure check-tax — `KNOBS:kPROBING`),
  and a route's guard admission demands a DEFINITE verdict — Unknown refuses as the unsure
  direction.
- **shared-edit-before-erasure** (`plans/30L` §6) — a region's per-instance no-execution proofs
  are minted at ONE seat (`settle::lower_shared_decision`'s `Replace` arm) and only after the
  universal meet agreed. Nothing may mint one per instance ahead of that: the ledger is
  grow-only and has no retraction, so a later Run meet would have to re-introduce a wall it had
  already retired for a mutation the artifact still executes. The witness the license carries is
  the exact ORDERED union of every contributing instance's establish
  (`pin-shared-witness-spans-instances`); a per-call witness never substitutes, and
  `AllEstablishesVouched::mint`'s identity/cardinality match is what makes that unspellable.
  Region freshness reads a SELF-SUPPRESSED solve over the whole population — the sibling
  instances of one region wall each other, and the region's own atomic replacement is what
  removes them (`effect::self_reach_holds`'s argument, one level up) — and that second answer is
  read only beside its OWN certification.
- **wire-records** — probe results move to the `262` §2 records lane at
  block-rebuild (partial deriv-family ⇒ wall-total; additive keys).
- **the-in-loop-floor-is-route-aware** (`30L` §7; built `30Qa`) — `floored_in_loop`
  still binds every per-SITE leaf decision; it lifts ONLY for a route carrying a member
  ORDINAL from a closed region population, because that edit lands once at the authored
  definition, universally quantified over every member. The ordinal IS the fence: a
  route the census gave no ordinal keeps the floor. Members are OVERLAYS, never clones —
  one lowered `cfg_node`, N route instances, N `InlineSite` entries MEMBER-MAJOR, and
  `site N.M` sub-indices that only ever append to member 0's non-loop numbering (in-loop
  inline calls shipped `site 0.0` before this lane; that identity is pinned). The census
  counts members independently of argv; duplicate `(site, fact)` establishes keep
  refusing (ruled, `30Q`); `ship_auto`'s subject vector is the whole node population
  (`node_subjects`), byte-identical to the single fact outside a loop.

## Determinism + precision tests

- **verdict-injected** — `build_plan`/`compile_probe` are pure; the host verdict
  is injected (`verdict_of`); output order is span-sorted, ordered collections
  only.
- **R2-CHANGEDELTA** — "do B because A changed": the author's `changed=1` flag
  is a consumed observable the discipline must PRESERVE, never synthesize.
  Never elide a delta-gated effect via a *state*-probe; never synthesize the
  cross-kind `file:`→`service:` edge. Encode the precision test; don't add
  effect-map dimensions.
