# 26H — the kernel fix plan: keying, validity, and loud degrades

AI-authored (Fable conductor, by-hand scout), 2026-07-27/28 overnight. THE plan-of-record for
fixing the four `26G` findings, minted under the human's kernel-fix protocol (typed this
session): (1) findings-only diagnosis (`notes/26G`, verified) → (2) this Fable in-window scout
and plan → (3) the human rewinds the conductor → (4) the rewound conductor dispatches a builder
to execute THIS plan and triple-checks the changes with fresh kernel reads. Correctness-critical;
never an aside. Authority: root docs > `spike/CLAUDE.md` + crate `CLAUDE.md` registries >
`notes/277`/`plans/281`/`notes/23O` > `26G` > this plan. Where this plan and law disagree, law
wins and the disagreement is a STOP-and-report.

## §0. Scout verification (what the conductor independently confirmed by reading, 2026-07-28)

Everything below was re-read at source, not taken from `26G`:

- v-auto-path — `analysis/src/effect.rs` `command_effect`: predict-set resolution → `idx.effect_of`
  cells; empty cells (`:366-374`) or no resolution (`:349-356`) → `auto_or_opaque` (`:235-248`),
  which mints `Establishes(auto_fact)` iff the provider is in the threaded `verdict_providers`
  set, else `Opaque`. The verdict BODY is never consulted for the coordinate. Doc-comment scope:
  the floor is for "a concrete-argv site that declared no marked effect" (`24L` §2/§3).
- v-auto-cell — `core/src/lib.rs:655-695`: `dorc-auto:<provider>` / `EntityRef::Singleton`
  ALWAYS / fixed `converged` selector. Its own doc prices the coarseness ("more forced runs,
  never fewer") and carries the two fences: `fence-unnameable` (the `:` in the kind makes the
  namespace unmintable by authors) and `fence-no-entity` (no bind ⇒ no operand promoted to
  referent). `is_auto_kind` is the load-bearing predicate for probe-ships-verdict-body and
  `fence-no-disjoint`.
- v-fact-fold — `cli/src/main.rs:5656-5761` `facts_from_sites`: records are read site-keyed
  (`RecordKey{site, member}`) then folded into `by_fact: BTreeMap<FactKey, Observable>` with
  `merge_observable` on collision; cross-site disagreement mints a `measured_merge_disagreement`
  narrative (decision-inert; largely unconsumed at render — `289:seam-narrative-render-unconsumed`).
- v-validity — `effect.rs:751-758` (contract) + `:1441-1446` (computation):
  `QueryResolvable.valid = reach.states[i].is_pristine()` — static, records-blind, computed in
  `analysis`; the caller withholds an invalid query's rc to ⊤ (`main.rs:5684-5694`). The
  doc-comment itself frames the collapse as caller-owned (`inv-superposition`).
- v-no-reprobe-needed — invalid-Query sites SHIP and are RECORDED: `plan/src/lib.rs:2834`/`:3085`
  emit `ProbeSiteKind::Query { valid }` sites into `probe.checks` regardless of the bit, and
  `facts_from_sites` reads their records (withholding only the FOLD). So a validity fixpoint
  (§3 W-C) needs NO second probe pass — the rcs are already measured, merely withheld.
- v-verdict-machinery — `oracle/src/verdict.rs`: `VerdictSet` WRAPS `PredictSet` (same
  role-parametrized parser, same dialect AST); the tracer reuses `resolve_word`/`eval_test`/
  `pattern_matches`; `evaluate_verdict`/`vouch_site` already answer reached-vouch vs decline per
  site argv. `verdict_providers()` (`verdict.rs:82-92`) is the existing `24L` §7 seam: the
  kernel is verdict-unaware BY DESIGN and the driver threads verdict-derived DATA in. The F1 fix
  extends this same seam with richer data; it does not breach the kernel's purity.
- v-topreason — `oracle/src/predict/eval.rs:106-131`: closed enum {EmptyArgv, NonConcreteWord,
  MissingAnnotation, NoProbeReached, UnresolvedAnnotationValue, BudgetExceeded, Pipeline}. No
  variant means "the body contained a statement I do not model"; `Pipeline` (`:125-130`) is the
  accept-degrade-and-say-so precedent to copy.
- v-fence-no-disjoint — `plan/src/survival.rs:1212+` (test `fence_no_disjoint_auto_backing_
  never_survives`): the auto-kind is registered (`add_auto_kind`, cli `main.rs:1213`) to force
  `MayAlias` ⇒ demote, so the synthetic singleton can never manufacture separation.

Trial-claim corrections adopted from `26G` (independently spot-checked where cheap): the
`context-entry-babby-{elides,diverges}` fixtures ARE the existing two-same-command-sites pins;
the `kp` mark was a read-set declaration, not effect-coordinate evidence; sibling-`cant-tell`
(not just conflict) collapses a shared cell.

## §1. Scope and sequencing (four workstreams, strict order; W-D minted 2026-07-28)

W-A (loud degrades; LANDED whole — tip `f9e7f4a7`, both halves, zero disposition movement) →
**W-D (and-or lists: the R-5 safety fix + the closed-form support)** → W-B (verdict-mark
keying) → W-C (validity fixpoint). W-D precedes W-B because more/fewer resolving bodies is
upstream input to the keying work (same coupling logic as
`26G:haz-fixing-keying-changes-fold-inputs`); every workstream lands separately, re-blesses
separately, never measured while another is in flight. The safety-direction law now carries
ONE exception: `26G:§FINDING-andand-resolves-a-wrong-coordinate` (found by W-A2's bounded
investigation) IS a wrong-yes-capable soundness defect — the only one known — and W-D
phase 1 exists to close it. Everywhere else the standing rule holds: any diff that makes
MORE elide without a pinned law argument is presumptively wrong.

## §1½. W-D — and-or lists in oracle bodies (minted 2026-07-28; provenance: the human's
in-chat demand that `|| return` is critical-path to oracle code and the dialect cut
contradicts the corpus's own mandated idioms (R2-MULTIOP's arity gate, the oracle-contract §3
standard gate, USER_STORY stage 3's own oracle) — direction human-demanded; scope ACKED
(human-typed 2026-07-28: "Acked, reasonable… I was just making sure `return 2` was handled,
that one is critical. Can't be typing a three-line gate around every single command when a
dozen+ possible paths might need active refusals in a new/careful/defensive oracle." —
phase 2 form (b) `cmd || return N` is therefore REQUIRED-tier, the workstream's center))

Phase 1 — SAFETY (closes R-5; resolutions may only DISAPPEAR): lex `||`, `&&` (and lone `&`)
as real tokens in the predict/verdict dialect lexer; and-or shapes parse structurally; every
shape not supported by phase 2 degrades to loud-⊤ with the and-or/or-list reason (W-A's
threading renders it). Kills the wrong-coordinate hole: `a && b` can never again lex as three
words with everything right of `&&` invisible-but-shipped. Golden movement expected ONLY in
the fail-safe direction (wrong-Resolved → ⊤/run); every diff inspected at bless.

Phase 2 — SUPPORT (the closed forms; restores the critical-path idioms): static support for
exactly (a) `[ … ] || return N` — left side statically evaluable by the existing eval_test
machinery; semantically identical to the blessed `if ! …`-free spelling (`if [ … ]; then
return N; fi` equivalence, differential-tested); and (b) `cmd || return N` with LITERAL N —
decline-or-continue: the left rc is unknowable but the right branch is a decline, and a
maybe-decline is always safe under the rc-partition (≥2 ⇒ can't-say ⇒ run), so the walk
resolves down the fall-through path. `&&` closed forms (`[ … ] && return N` etc.) ONLY if
the map shows them equally clean (the fall-through path there is left-FAILED, whose tracing
state is murkier) — else they stay loud-⊤ and are named in the landing report. General
`A || B` with a non-return right side (fallback probing — coordinate ambiguity, the real
footgun) stays loud-⊤, permanently until separately designed.

Fences: no keying changes, no validity changes (W-B/W-C territory); differential tests vs
the floor shells for every supported form (the tracer must agree with dash/posh on which
path runs, incl. the `||`-left errexit-exemption agreeing with book-side law); the contract
doc is NOT edited (phase 2 makes it TRUE again); empty-world byte-identical; map-then-execute
with a conductor checkpoint between map and code, and between phase 1 and phase 2.

## §2. W-A — every silent ⊤-degrade gets a span and a reason (fnd-three-constructs-void-marks +
fnd-existence-gate-darkens-oracle(a) + the shared-cell silence)

Law: `inv-top-reject` ("bias every parser ambiguity toward ⊤-reject-with-diagnostic");
`TopReason::Pipeline` is the shape (accept byte-exact, degrade conservatively, SAY SO).

1. `TopReason::UnmodeledStatement` (name at builder's discretion, one variant) carrying the
   offending statement's span; minted where the tracer abandons a body on a statement it does
   not model. Surface it (a) from `dorc lint` and (b) in the probe-side `site-unresolvable`
   note, which today names the site but not the cause. Prose explicitly-empty per
   error-authorship-tier; DiagCodes get defining cases.
2. The cause-agnostic backstop, sequenced FIRST inside W-A (`26G` recommends; hours, catches
   constructs nobody has found yet): a `# dorc-lang/v0.2`-marked file whose binds/marks do not
   survive to the lifted representation warns from `dorc lint`, naming the voided marks'
   line(s). Mechanically: compare marks-recognized-at-parse against marks-in-lift (or
   strip-delta, as the r26 trial did by hand); negative pin — a legitimately markless oracle
   must NOT warn.
3. Pin the three known voiding constructs (bracket-test statement, `case` glob arm,
   continuation-into-redirection) as cases; pin bare `*)` staying quiet; pin a voided file with
   no post-bind reference (proves independence from the incidental shellcheck SC2154). The
   parse-side drop site(s) are `-GUESS` territory (`26G` §fnd-three… §1): the builder pins the
   actual site(s) by stepping the three inputs (plausible region: `oracle/src/predict/parser.rs`
   `_ => return None` arms `:255`/`:1436`/`:1456`, `mark_grammar.rs:127`) and decides
   one-guard-vs-three from evidence, not guesswork. These constructs stay ACCEPTED (legal POSIX;
   parse-permissively-trace-conservatively) — the diagnostic is a warning naming voided marks,
   never a refusal; `strip` round-trip must stay byte-exact.
4. The shared-cell silence: when `facts_from_sites` merges a sibling `cant-tell` (or conflict)
   into a cell and thereby de-licenses sites, that existing `measured_merge_disagreement` /
   merge event must reach a PUSH surface as one deduped line (plan stderr advisory tier,
   kWARN-tune-high era) — today it exists as an unconsumed narrative. Smallest honest render;
   do not build the arrangement-home round here.
5. Explicitly OUT of W-A: modeling `command -v` (or `test`/`[`) in tracer bodies — that is
   ruling-gated (§5 R-1). W-A's diagnostics will name these statements loudly, which both
   stops the authoring bleeding and accumulates the evidence the ruling wants.

## §3. W-B — verdict-mark coordinate keying (fnd-shared-auto-cell-collides)

Law: oracle-contract §4 "attach facts to the one line that measured them" + §5a (the vouch
never becomes a fact — NB the fix keys the establish CELL; the vouch stays vouch);
`inv-site-keyed-results` restored fact-ward; minting-law (verdict `:`/`:!` and observe `:?`
marks on runnable lines MINT — the algebra already intends verdict marks as real coordinates);
`24L` §2/§3 (the auto-cell's priced scope is the MARKLESS verdict-only body — read the
`auto_or_opaque` doc-comment; a body that authored a coordinate is outside that pricing).

Mechanism (the seam is `command_effect`'s inputs — extend the existing `24L` §7 threading):

1. The driver lifts verdict sets ONCE (today: `verdict_providers()` set + `build_vouches`'s own
   re-lift) and threads an evaluable verdict structure into `classify`/`command_effect` — either
   the `VerdictSet`s themselves or a distilled per-provider evaluator. Replace the bare
   `verdict_providers: &BTreeSet<ProviderId>` parameter rather than adding beside it (one seam,
   no drift between set and sets).
2. In `command_effect`, at BOTH `auto_or_opaque` call sites: before falling to the floor,
   evaluate the provider's verdict body against the site's fully-concrete argv (the same
   `evaluate` walk — `VerdictSet` wraps `PredictSet`). Selection rule, hard: the coordinate
   comes ONLY from a VERDICT mark (`asserts`/`refutes`) on the reached path — never an observe
   (`:?`), never a bind alone; at most one verdict per line is already grammar law (rc-arity,
   `281` §7). Polarity (`:!`) does NOT change the cell (sense lives in the vouch/glue, not the
   key).
3. Outcome table (exhaustive; anything not listed = auto-cell):
   - reached path + verdict mark + fully-resolved coordinate (kind + entity from argv-bound
     positions + selector) ⇒ `Establishes(authored cell)`.
   - reached path, marked, but the mark's value-position does not resolve on this argv
     (`UnresolvedAnnotationValue`-shape) ⇒ auto-cell (never a garbage key, never Opaque — the
     provider still bears a verdict fn). W-A's diagnostics say so.
   - declined argv (return≥2 path, unmatched arm, non-literal return) ⇒ auto-cell. Rationale:
     a decline is the author refusing to speak for this shape; their coordinate must not key a
     shape they refused (the membership floor is all that remains).
   - markless verdict body ⇒ auto-cell, byte-identical to today (`24L` §2 regression pin).
4. FORBIDDEN cheap alternative, stated so nobody reaches for it under pressure: per-site
   synthetic cells (`dorc-auto:<provider>#<site>`). The same-cell collision is LOAD-BEARING
   conservatism — `an-written-stale` (`purge X; … install X` ⇒ resting probe not authoritative)
   depends on same-state sites COLLIDING. Only an AUTHORED coordinate may split cells, because
   only the author knows two sites touch different state. Two sites with the SAME authored
   entity must still share a cell and still ⊤-merge on disagreement (that collapse is correct).
5. Downstream consequences the builder must handle (not optional):
   - `facts_from_sites` needs no change (distinct keys stop the lossy merge structurally); the
     cross-site merge for genuinely-shared cells STAYS, with W-A's surfacing.
   - Survival/sparing: an authored verdict coordinate is a REAL kind and enters the ruled
     algebra with the same powers predict-derived coordinates already have — that is
     as-designed (minting-law), NOT a new license class. The builder must verify: (a) the
     verdict-minted selector tokens register in dialect(minting family, kind) exactly as
     predict-minted ones do (sparing-algebra: unminted/cross-dialect tokens COLLIDE); (b) the
     fact's backing-SET includes the verdict body's own observe-marked reads
     (observe-widening; kill-surface only grows); (c) residual auto-cells keep
     `fence-no-disjoint` (the `add_auto_kind` registration and its test stand un-weakened).
     If any of (a)/(b) turns out unbuilt for verdict-minted tokens, BUILDING it is in-scope
     for W-B; leaving verdict coordinates outside the sparing dialect (⇒ collide) is the
     acceptable conservative fallback, disclosed.
   - `EstablishWritten`/reach now computes over authored cells: a `cp` establish can collide
     with ANOTHER provider's same-cell fact. That is a precision AND conservatism improvement
     (real cross-provider staleness becomes visible); expect golden churn; inspect each.
   - probe-ships-verdict-body routing keys off `is_auto_kind` in places (v-auto-cell): audit
     every `is_auto_kind` consumer — a verdict-marked site now carries an authored kind but
     must STILL ship its verdict body as its probe (it has no predict). The discriminator
     "which body ships" must become "verdict-provider with no predict-resolution", not
     "kind is auto". This is the likeliest silent-breakage point of the whole fix; enumerate
     the consumers (`grep is_auto_kind`) in the builder's first commit.
6. Tests (from `26G` §4, consolidated): two same-command sites, distinct authored entities —
   distinct cells, independent licensing, ORDER-INDEPENDENT (kills the F1 tie-break mystery
   behaviorally; its mechanism stays unquoted); same-command SAME-entity — shared cell, ⊤ on
   disagreement; sibling-`cant-tell` — de-licenses only true shared cells, WITH the W-A line;
   unresolvable mark value — auto-cell fallback; markless — byte-identical (empty-world pin
   too); re-bless `context-entry-babby-elides`/`-diverges` (dispositions become per-site).

## §4. W-C — the validity fixpoint (fnd-dead-branch-still-invalidates)

Law: USER_STORY st.3 ("a command that will not run cannot invalidate anything") — implemented
in the wall predicate, violated by the validity bit (`26G:haz-two-poisoning-mechanisms-one-law`);
`26B:rul-plan-construction-is-reactive` (human-typed): plan-construction is EVENTUALLY reactive —
incoming facts re-analyze and mutate ongoing analysis. The bounded, same-records,
deterministic iteration below is a degenerate special case of that ruled direction, NOT the r26
reactive machinery (no new probing is minted; nothing streams).

Shape — RE-CUT 2026-07-28 at human direction, superseding the earlier mask-parameter design
(a `proven_dead` set threaded into classify was REJECTED: a flag every present and future
consumer must remember to consult is the composition footgun; absence cannot be forgotten).
The ruled shape is **analyzer-model erasure**, per the human-ACKED overlay model (typed
2026-07-28, verbatim-tier):

> 1. the analyzer handles a MODEL of the code, mapped back to the authoritative
>    storage-for-all-global-non-analysis-purposes (span-map etc.);
> 2. the analyzer-model can SHRINK, to omit things PERMANENTLY that we explicitly disallow
>    to future analysis passes;
> 3. shrinking elements in the analyzer-model type-provably MUST mint information in the
>    global model showing what shrank and why.

Mechanism:

1. **The frozen origin.** The input model — book bytes, CFG, spans, leaf identity, the
   as-lifted effects, oracle lifts, admitted records, vouches — is NEVER mutated by the loop.
   Rebuild-from-origin is always possible (§4¾).
2. **The erasure ledger** — the loop's SOLE cross-round accumulator. Grow-only within a
   record-world; entries are `(site, erased-effect, proof, round)` where `proof` is a
   records-proven-dead derivation and nothing else: a measured, valid, non-conflicted query
   rc (`Predicted::Value`), folded through sh control-flow semantics (the errexit/`||`
   doors), showing the branch containing the site cannot be reached. NEVER derived from
   `Disposition` — omitted-for-any-reason is not dead (`26G`'s naive-fix hazard). Type-gated
   both ways: the entry constructor DEMANDS the proof (no entry without derivation), and the
   erasure function CONSUMES an entry (no shrink without a ledger record) — this is the
   overlay model's point 3 made structural, and it discharges §4.6's mint half by
   construction. Round-tagged for the why-chain.
3. **The residual model** — derived each round from origin + ledger. Identity planes
   (CfgNodeId, LeafId, spans, render presence, record keying) are IDENTICAL to the origin —
   `ref-erase-effects-never-identity`: erasure operates on the effect/invalidator plane
   ONLY; a renumbered or dropped site would shear record keying (`inv-site-keyed-results`),
   the byte-honest render, and `dorc why` addresses. What is absent from the residual is the
   erased sites' MUTATOR-HOOD: to every analysis consumer, an erased site is
   indistinguishable from one that never mutated (uniformity is the safety property — no
   consumer can mishandle a flag that does not exist), while the ledger keeps the
   distinction fully recoverable for provenance. The concrete spelling (an erased-typed
   effect variant vs Pure-with-ledger-backing) is the builder's map-then-execute proposal;
   the reviewer holds it to the uniformity + recoverability pair above.
4. **The loop**, driver-owned: round k derives the residual from origin + ledger → solve +
   classify the residual (validity recomputed naturally — erased mutators simply do not
   exist to invalidate) → fold with the frozen records → new proofs append to the ledger →
   round k+1; terminate when a round appends nothing. Monotone (erasure only removes
   invalidators ⇒ more queries valid ⇒ more folds ⇒ more proofs; pinned by test, not
   argument alone), bound = site count, hard iteration cap + LOUD diagnostic (never silent
   partial). Deterministic (BTree order throughout); the final artifacts are a pure function
   of (origin, records) — byte-identical across runs and under record-arrival shuffle.
5. **No re-probe** (`§0:v-no-reprobe-needed`): invalid-Query checks ship and their rcs are
   measured, only withheld; the loop consumes measurements already in hand. Probe EMISSION
   is untouched at v1 — a scope cut, not an invariant (`seam-wc-fixpoint-meets-reactive-
   probing`, §4¾).
6. **Concentrated danger, by design**: representation-level un-undo-ability protects against
   consumer drift but cannot protect against a bug in the ONE transform (fold-proof →
   ledger entry → erasure). That function is where W-C can be wrong, and concentrating the
   risk there is intentional — the conductor's triple-check reads it line-by-line against
   the proof definition above, and its tests are the review's center of gravity.
7. Tests: N-ladder (N≥3), unmodeled RHS, all-holds ⇒ all N fold (today 1); middle guard
   `absent` ⇒ cascade stops exactly there (above folds, it and below run/guard); modeled-RHS
   ladder unchanged (today's passing case — regression); a live (non-dead) Opaque still
   invalidates everything below — THE case that keeps it honest; iteration determinism
   (same inputs ⇒ byte-identical, shuffle record arrival order); a cyclic/loop shape pinning
   termination + the in-loop-body structural floor untouched; empty-world byte-identical.
   PLUS the erasure-model pins: no ledger entry constructible without a proof (compile-time
   where achievable); no shrink without an entry; identity planes byte-stable across rounds
   (spans/ids/render of an erased site unchanged); ledger round-tags reach the why-chain;
   the rebuild-from-origin contract exercised as a unit test of the API (a mutated
   record-set input ⇒ ledger discarded, full recompute — unreachable in v1 production paths,
   pinned so the contract survives until the reactive round makes it reachable).
8. ATTRIBUTION IS A HARD REQUIREMENT, not polish (added at human challenge, 2026-07-28;
   mint-half now discharged structurally by the type-gated ledger — the RENDER half below
   remains an explicit deliverable the types do not discharge):
   every proven-dead site and every round-2+ validity flip mints its witness/narrative link,
   and the why-chain for a cascaded elision renders the full derivation ("trusted because
   line N's mutator was proven dead by line N's measured rc, round k"). An unattributable
   cascaded elision is the second-worst sin (`271:rul-sin-ordering`) built into the flagship
   fix — a W-C build that passes §4.5's behavior tests but cannot answer `dorc why` with the
   chain is INCOMPLETE and does not land.

## §4¾. W-C cross-round state law + the reactive-era bridge (human-driven sharpening,
2026-07-28 — the composition-across-reruns footgun, named and closed)

Different quantities have DIFFERENT re-run semantics, and mixing them is the catastrophe
class (human-typed: verbatim-carried vs actively-maintained vs meet-to-⊤ vs never-survives
compose unintuitively). The loop's law, enforced structurally by where data lives:

- FROZEN INPUTS (carried verbatim, never re-derived, never re-admitted): book/CFG/spans ·
  oracle lifts · the ADMITTED records (admission runs ONCE; a Refused stream gets no second
  chance in round 2; no re-measure, no re-merge) · vouches (records-independent; frozen to
  prevent dependence creep).
- PER-ROUND PURE DERIVATIONS (recomputed from scratch each round; NEVER incrementally
  patched): reach states · classifications · `valid` bits · fact views + their meets ·
  statuses · dispositions · the plan. Round k = F(frozen inputs, dead-set_k). Meets stay
  honest BY recomputation — they can neither linger past their justification nor be
  forgotten while it stands. Any cross-round incremental mutation of these is a defect.
- THE ONE ACCUMULATOR (actively maintained, grow-only): the ERASURE LEDGER (§4.2 — né the
  proven-dead set; same content, now typed with its proof and mint-gated). Its monotonicity
  is CONDITIONAL — scoped to a fixed record-set (below). The residual model derived from
  origin + ledger is NOT state: it is a per-round pure derivation and is rebuilt, never
  patched.
- NEVER-SURVIVES (discarded intermediate outputs): every non-final round's plan, render,
  narratives, origins. Only the FINAL round's artifacts reach any surface (user, whylog,
  why-lens). A round-1 "substitution refused" narrative is FALSE in round 2 and must not
  reach the whylog. Sole deliberate exception: the §4.6 round-tagged derivation links,
  durable by design so the why-chain can render the cascade.

### seam-wc-fixpoint-meets-reactive-probing (NAMED SEAM — the reactive round MUST consume
and discharge this before wiring the two loops together; human correction adopted: "probe
emission decided once" is TEMPORARILY true — current-pipeline happenstance, not a documented
invariant; `26B` owes a reactive architecture with many parallel probing waves before the
single consent moment that fixes and freezes the plan)

What W-C GUARANTEES (conditional, local, documented): erasure-ledger monotonicity within a
FIXED record-set; rebuild-from-origin on any record-set change; intermediate rounds
unobservable (never rendered, never whylogged); final artifacts a pure function of the final
record-set.

- brg-ledger-resets-on-record-world-change — grow-only holds only while the record-set is
  FIXED. Any record-set change (new arrival, retry/supersession, conflict resolution)
  invalidates the ledger ENTIRELY: discard, rebuild the residual from the frozen origin,
  recompute the whole fixpoint against the new record-world. Compute-wasteful and correct
  (analysis is cheap; the network is not); makes the `26B` confluence target (final answer =
  pure function of the FINAL record-set, wave-structure-independent) hold trivially.
  Carrying the ledger across record-worlds is the exact composition mistake §4¾ exists to
  forbid — an erasure "permanent" within its record-world is NOT permanent across them.
  HUMAN-DIRECTED (2026-07-28): whether the record-set can even shift in the validating
  direction is UNSETTLED — this concern stays LIVE, never settled/closed/assumed-safe, until
  the reactive round rules on it.
- brg-emission-exclusion-is-v1-scoping — "no feedback into probe emission" is a v1 SCOPE
  CUT, not an invariant: the reactive round may legitimately rule that proven-deadness
  informs probe minting (e.g. no probes into dead branches). Must not be documented as
  permanent anywhere user- or builder-facing.
- brg-half-remembered-rules-not-relied-upon — the human half-recalls a foreseen rule
  ("reactive probing only ever adds probes/information, never rescinds" and/or "late
  information cannot add elisions, only remove them"). The corpus digests suggest
  (~SUSPECT, unread-in-full) these echo `26C`'s cancellation package (demotion-only ·
  cancellation-finality · quiescence-witness). PER HUMAN DIRECTION: none of this is relied
  upon here — it may need to change; W-C's correctness stands on its own conditional
  guarantees alone.
- Handed-forward questions the reactive round must answer at this seam: does
  rebuild-from-origin (which can produce FEWER elisions under more records) compose with a
  demotion-only DISPLAY law once intermediate states become observable? (trivially safe at
  v1 solely because intermediates are unobservable) · does record supersession even exist
  under the eventual attempt model, or are record-worlds append-only? · termination and
  monotonicity under interleaved probe-waves and fixpoint re-runs (the standing open
  questions the human names) · whether erasure may inform wave-k+1 probe minting.

## §5. Rulings owed the human (collect at plan review or at triple-check; none blocks W-A)

- R-1-model-command-v (from `26G` fnd-existence-gate §2, ambiguous-needs-human): should the
  tracer model `command -v` (and how far does the modeled-statement set go — `test`, `[`,
  `case $?`)? WRONGLY POSED — `command -v` already models; the real question is OR-LISTS in
  oracle bodies (`26G:§CORRECTION-orlist-not-command-v`), weighed against the oracle-contract
  teaching an existence gate that is currently unusable. RECOMMEND still defer: let W-A's
  diagnostics accumulate real-author evidence first (each addition is a deliberate
  `inv-top-reject` ⊤-shrink, its own purity argument). The gate idiom is now broken-but-LOUD
  (`TopReason::OrList` reaches the `site-unresolvable` note); a doc-side interim note is the
  human's call (docs are their voice).
- R-5-andlist-resolves-wrong — NOT a precision question like its siblings: `&` is not a
  predict-lexer metacharacter, so `a && b` lexes as three WORDS and every statement right of
  `&&` is invisible to the tracer while the byte-exact shipped probe still runs it. A swallowed
  `shift` therefore resolves a coordinate off the UNSHIFTED argv while the host measures the
  shifted one — a wrong `Resolved`, the disaster class, not a ⊤. Full repro + the per-statement
  severity breakdown: `26G:§FINDING-andand-resolves-a-wrong-coordinate`. The fix (lex `&&` so
  the and-list degrades like the or-list) GROWS ⊤ and moves dispositions, so it needs its own
  dispatch and re-bless — it is not W-A's, which changes no elision anywhere. Ranks ABOVE
  R-1/R-3 in the queue: it is the only open item that can produce a wrong yes.
- R-2-dead-set-kernel-input — DISCHARGED BY ACK (human-typed 2026-07-28, the three-point
  overlay-model restatement in §4): the kernel analyzes a MODEL; the model may be residual;
  shrinkage mint-gates provenance into the global model. The old mask-parameter question it
  answered is moot (that design is rejected). Residual: the builder's concrete spelling of
  erasure still lands under conductor review per §6.
- R-3-fact-keyed-reservation (W-B): `inv-site-keyed-results`'s parenthetical reserves
  fact-keyed verdict shapes to "a conscious orchestrator+human decision". `26G` §fnd-shared…
  §2 reads the auto-cell collapse as that reserved decision taken for the MARKLESS floor and
  not obviously for authored marks. W-B restores site-distinctness exactly where an author
  spoke. Ratify that reading.
- R-4-exit-code-collision (carried from the executor lane, unrelated to the kernel: exit 11
  wrapper-incoherent vs `260`'s unreachable-host reservation) — listed here only so the
  morning sweep sees one queue.

## §6. Execution protocol (RE-CUT 2026-07-28, human-typed: NO pre-execution rewind — the
seated conductor executes W-A→W-B→W-C with its full standing context and triple-checks each
landing; the rewind happens AFTER the correctness-critical work is done and reviewed)

- One builder per workstream (W-A may be the resumed findings-diagnostician — it holds the
  deepest root-cause context; W-B and W-C MUST be separate dispatches with a conductor
  checkpoint between). Worktrees off the current r26
  integration tip (rebase `ai/r26-analyzer-findings` forward or branch fresh; conductor's
  call at dispatch time). Map-then-execute-split applies: each builder proposes its concrete
  diff-plan (files, signatures, test list) BEFORE writing code; conductor reviews against
  this note and `26G`, then releases execution.
- Conductor triple-check per workstream (the human's stated reason for the rewind): fresh
  reads of every touched invariant site — minimum: `effect.rs` classify/auto seam ·
  `core` auto_fact/fences · `facts_from_sites` · survival's fence test + dialect registration ·
  every `is_auto_kind` consumer (W-B) · the fold/door code W-C hooks. Verify the outcome
  tables (§3.3) and the never-lists (§3.4, §4.2) line by line against the diff.
- Gates: `mise run both gate:full-quiet` per workstream; goldens re-blessed per workstream in
  SEPARATE commits, each diff inspected case-by-case (bless cannot prove an elision right);
  the empty-world and markless pins run before any bless.
- The comment budget (15 bare `//` per lane) and error-authorship-tier (builders mint
  `[unwritten:]` prose only) bind as always.
- Size honesty (from `26G`, scout-concurred): W-A hours-to-days; W-B days, cross-cutting
  (four crates); W-C days, heaviest (pass structure). Do not compress W-B/W-C into one
  overnight push; the protocol exists because rushed kernel work here converts precision
  bugs into soundness bugs.

## §7½. The feedback-risk inventory (human-demanded, 2026-07-28 — W-C is results-fed-back-in,
the class the project has long treated as maximally dangerous; the rewound conductor re-checks
the build against EVERY row)

- risk-license-laundering — outputs-as-premises chains. Account: the only cross-round carrier
  is records-proven-branch-deadness — now the typed, proof-demanding erasure ledger (§4.2);
  dispositions/verdicts/licenses are recomputed outputs and NEVER re-enter
  (`pin-no-outcome-as-generator`). The ledger's constructor is the enforcement point.
- risk-self-consistent-fantasy — a fixpoint settling on an internally-consistent elide-all
  fiction (cf. the self_reach doc-comment's own argument). Account: well-founded layering —
  round-1 deadness uses only unconditionally-pristine queries; each layer rests on strictly
  earlier layers + one more measurement; nothing proves itself.
- risk-error-amplification — the honest cost: one wrong/stale rc today under-executes one
  line; cascaded, it can un-run N, and depth-N elisions rest on an N-conjunction of
  measurements. Claim (~SUSPECT, triple-check target): no new CLASS of exposure — line 1 of
  every ladder already elides on this logic; probe→apply staleness is the priced stage-3
  bargain; `toctou-scope` excludes unattributed-drift machinery. Quantity changes, kind does
  not; say so in any user-facing account.
- risk-nontermination — monotone growing set, bound = site count, hard cap + LOUD diagnostic
  (never silent partial); monotonicity pinned by test, not argument alone.
- risk-nondeterminism — degenerate confluence (all records pre-loop, BTree order, pure
  function of inputs); arrival-shuffle byte-identity pinned (`26B` confluence target).
- risk-superposition-breach — kernel stays records-blind: one opaque phase-agnostic input
  ("sites proven not to execute"), caller-derived, `verdict_providers`-shaped; ruling R-2.
- risk-by-exclusion (three adjacent feedbacks NOT built, and the build must not drift into
  them): no feedback into probe EMISSION (nothing new ever executes on a host because of the
  loop — §4.4); no cross-RUN re-ingestion (rec-5; within-run, same-records only); no
  aid-plane leakage (two-plane law; narratives stay decision-inert).
- risk-untrusted-reach — a host-forged rc gains cross-site reach within its own plan.
  Account: bounded admission + conflicted-rc-withheld stand; a host could already lie its
  own lines into elision (self-harm-only at N=1); any second host / saved result triggers
  the standing kSTATE/host-as-adversary riders BEFORE reuse.
- risk-unattributed-cascade — closed by §4.6 (hard requirement; found at the human's
  challenge — the original plan pinned behavior but not attributability).

## §7. Cold trails carried (do not silently forget)

- F1 tie-break mechanism (why the LATER of an all-holds same-cell pair guards) — unverified
  hypothesis in `26G` §fnd-shared… §1; moot after W-B but the order-independence test is the
  behavioral pin. If W-B's build DISCOVERS the mechanism, one line in its landing note.
- F3 parse-drop site(s) — `-GUESS` until the builder steps the three inputs (§2.3).
- The verdict-body existence-gate asymmetry (`26G` fnd-existence-gate §1: the gate did not
  darken a VERDICT body's auto-keying; vouch-survival unmeasured) — W-A's case list includes
  it; verify while pinning.
- D9/whylog status from the executor lane is UNKNOWN (its report skipped the item) — check at
  round close; unrelated to this plan.
