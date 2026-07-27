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

## §1. Scope and sequencing (three workstreams, strict order)

W-A (loud degrades) → W-B (verdict-mark keying) → W-C (validity fixpoint). W-B and W-C are
COUPLED in the dangerous direction (`26G:haz-fixing-keying-changes-fold-inputs`: distinct
coordinates change the invalidating set W-C computes from): land separately, re-bless
separately, never measure one while the other is in flight. W-A first because it is
semantics-free, needs no ruling, and its diagnostics then witness W-B/W-C's effects during
their own builds. Nothing here changes the fail-toward-run bias anywhere; any diff that makes
MORE elide without a pinned law argument is presumptively wrong (`26G:haz-safety-direction-
holds-everywhere` — no finding was a soundness bug; do not create one fixing them).

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

Shape — the caller owns the loop; `analysis` stays records-blind (`inv-superposition`
preserved: the kernel gains one phase-agnostic INPUT, "sites proven not-to-execute", as data;
it never learns why):

1. `classify` (and the reach solve feeding it) accepts `proven_dead: &BTreeSet<CfgNodeId>`
   (default empty = today, byte-identical). A proven-dead site's effects are treated as `Pure`
   in the reach transfer and excluded from the pristine/invalidating set. It does NOT change
   reachability/CFG (the site still exists, still renders, still walls NOTHING because it will
   not run).
2. `build_plan` (or the fold layer) exposes the RECORDS-PROVEN-DEAD set: exactly the sites
   whose enclosing branch a folded, valid, non-conflicted query rc proved dead
   (`Predicted::Value` substituted into the errexit/`||` doors). NEVER derived from
   `Disposition` (omitted-for-any-reason is NOT dead — `26G`'s naive-fix hazard; a site is
   removed from the invalidating set only on positive records-proof).
3. The cli driver iterates: solve+classify(proven_dead) → compile the fold with the records in
   hand → collect newly-proven-dead → loop while the set GREW. Monotone (masking effects only
   ever makes more queries valid ⇒ more folds ⇒ more dead), bound = site count, hard cap +
   loud diagnostic if hit (never silent partial). Deterministic iteration (BTree order); the
   final plan is a pure function of (book, oracles, records) — byte-identical across runs.
   No re-probe: v-no-reprobe-needed (invalid queries' rcs are already measured, only withheld).
4. Probe-compile is NOT in the loop at v1: the probe ships once with validity as computed at
   ship time; the fixpoint runs at PLAN construction where records exist. (A future refinement
   — recomputing validity for probe-emission itself — is the r26 reactive lane's business.)
5. Tests: N-ladder (N≥3), unmodeled RHS, all-holds ⇒ all N fold (today 1); middle guard
   `absent` ⇒ cascade stops exactly there (above folds, it and below run/guard); modeled-RHS
   ladder unchanged (today's passing case — regression); a live (non-dead) Opaque still
   invalidates everything below — THE case that keeps it honest; iteration determinism
   (same inputs ⇒ byte-identical, shuffle record arrival order); a cyclic/loop shape pinning
   termination + the in-loop-body structural floor untouched; empty-world byte-identical.

## §5. Rulings owed the human (collect at plan review or at triple-check; none blocks W-A)

- R-1-model-command-v (from `26G` fnd-existence-gate §2, ambiguous-needs-human): should the
  tracer model `command -v` (and how far does the modeled-statement set go — `test`, `[`,
  `case $?`)? RECOMMEND: defer; let W-A's diagnostics accumulate real-author evidence first
  (each addition is a deliberate `inv-top-reject` ⊤-shrink, its own purity argument). The
  oracle-contract's own §3 gate idiom stays broken-but-LOUD until then; a doc-side interim
  note is the human's call (docs are their voice).
- R-2-dead-set-kernel-input (W-C): ratify that `proven_dead` as a classify input preserves
  `inv-superposition`/records-blindness (phase-agnostic data, caller-derived). The scout's
  argument: the kernel already takes caller-threaded world-knowledge (`verdict_providers`,
  `24L` §7); this is the same shape.
- R-3-fact-keyed-reservation (W-B): `inv-site-keyed-results`'s parenthetical reserves
  fact-keyed verdict shapes to "a conscious orchestrator+human decision". `26G` §fnd-shared…
  §2 reads the auto-cell collapse as that reserved decision taken for the MARKLESS floor and
  not obviously for authored marks. W-B restores site-distinctness exactly where an author
  spoke. Ratify that reading.
- R-4-exit-code-collision (carried from the executor lane, unrelated to the kernel: exit 11
  wrapper-incoherent vs `260`'s unreachable-host reservation) — listed here only so the
  morning sweep sees one queue.

## §6. Execution protocol (binds the rewound conductor)

- One builder per workstream (fresh Opus each; W-A may be one builder; W-B and W-C MUST be
  separate dispatches with a conductor checkpoint between). Worktrees off the current r26
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
