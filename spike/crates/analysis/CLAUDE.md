# spike/crates/analysis — CLAUDE.md

Role: the engine — CFG construction (`cfg.rs`), the monotone-dataflow worklist
(`lattice.rs` + `solve.rs`), effect/classify (`effect.rs`). The densest crate and
the entity-algebra's largest consumer. Read `spike/CLAUDE.md` first; deep design
rationale: `Research/notes/163`. The built machinery is code-visible — read the
code; this file carries only the dangerous, easy-to-miss parts. Registry
discipline: one rule per bullet, slugged; append to the matching section.

## Law — the dangers (each one is a latent wrong-elision or a hang)

- **lvalue-builtin-flags-are-spelled-not-guessed** (`30Nf` §3) — `value::transfer_lvalue_builtin`
  havocs EVERY tracked binding when it cannot say which variable an lvalue builtin wrote, and
  that floor stands. The one modelled exception is a LEADING `unset -f`, whose operands name
  FUNCTIONS by the builtin's specification, so the variable plane is untouched; only leading,
  because both floor shells stop option parsing at the first non-option word and `unset x -f`
  really does name a variable. Widening the exception set is licensure-relevant in the same
  species as a funcenv precision change (`28Q` §1's winner-shifting rider): a resolved variable
  resolves a load, a resolved load binds definitions, and bound definitions license. The
  measured cost of the un-narrowed form was a whole package never acquired.
- **spliced-internal-covers-detached-bodies** — `Cfg::is_spliced_internal` is true for a
  funcdef's OWN detached body lowering as well as for a call's spliced copy; both mean only
  "not a plan leaf". A consumer that needs "this is an execution" must ALSO check
  reachability-from-entry, or `ExecutionOwner::Leaf(call)` with `call != node`, which is the
  exact discriminator. Reading a detached body's vacuous-⊥ state as ambient is a wrong-elision
  (`vacuous-entry-fold`), and the flag alone does not stop you.
- **solve-termination-unenforceable** — the worklist's preconditions (monotone
  transfer · finite-height domain · semantic `Eq`) are caller-upheld and
  un-type-enforceable; a violation would HANG (empirically hundreds of
  CPU-seconds), so the worklist carries an iteration cap and stops instead. The
  cap flag (`Solution::converged`) is ADVISORY and is NOT the trust gate: a
  cap-tripped answer that still certifies is the least fixpoint and is USED.
  The gate is CERTIFICATION — every production answer comes from
  `solve_certified`, and `trusted()` throughout this crate means CERTIFIED, not
  converged (`solve-is-certified-only`). A consumer takes its named floor on
  `Inconsistent` (`trust_reach` is a per-consumer obligation, never ambient).
- **vacuous-entry-fold** — a detached/unreachable function body has a vacuous-⊥
  in-state; reading it as ambient is a wrong-elision. Fold to `MustRun`
  (`reachable_from_entry`).
- **opaque-poison-is-the-product** — `Opaque ⇒ Reach::Top` poisons all downstream
  ambient-ness. The poison wall is Dorc's honesty about unmodeled commands, not a
  bug: never "fix" it locally, and keep every refusal arm (splice ineligibility,
  over-budget, recursion) poison-preserving.
- **no-phase-no-fold-here** — this crate emits phase-/orientation-agnostic facts
  (consumed observables un-collapsed); never bake a phase default or fold
  `May`/`Must` here (`inv-superposition`) — a baked posture is a wrong-elision
  under the opposite phase's `kFAIL` direction.
- **must-lattice-by-type** — the merge is picked by the TYPE (`Must<L>` = `L`'s
  order-dual); a must-analysis over a bare `Powerset` is a compile error. Keep it
  that way: the union-where-you-meant-intersection bug must stay unrepresentable.
- **errexit-couples-build-and-solve** — CFG construction is two-phase
  (structural walk, then precise failure→exit edge materialization). Loops are
  real cycles (back-edges); a `while`/`until` condition is `StatusIterated`
  (errexit-exempt, blocks unconditionally); `if`/`elif` conditions are
  `StatusRelaxable`; loop bodies are flagged `in_loop_body` (the per-SITE floor
  `plan` honors; the one thing that lifts it is a universally-quantified region edit
  over a closed member population — `plan/CLAUDE.md the-in-loop-floor-is-route-aware`).
- **member-population-has-one-enumerating-seat** (`30L` §7; built `30Qa`) —
  `cfg::loop_evaluations` is the ONE answer to "which members does this loop have":
  the region census takes the COUNT, the value plane the member TEXTS; a second
  enumerator is a licence surface that can disagree with itself. The member binding
  is VOID wherever the loop's EXTENT rebinds the iteration variable
  (`value::loop_extent_rebinds`; CFG-driven, because a spliced body is in-loop while
  its span lives in the definition, so an AST-subtree walk sees the call and nothing
  inside it) — measured: a `pkg=wombat` in the body made the probe measure cells the
  command never touched, then replace a live mutator. A member's class comes from its
  OWN cells (`classify_cells`); the node's union answers `MustRun`, the floor a
  refusal falls to.
- **pure-kernel** — ordered collections only; no clock/RNG/fs/net, directly or
  transitively; that purity is what lets DST run with no DI ceremony.
- **funcenv-reads-source-literal-plane-only** (`28K` §2) — the function-environment
  domain consumes resolved values ONLY through `funcenv::SourceLiteralPlane`, which
  admits a word solely when it is a literal graded `ValueGrade::ProgramText`. A
  probe-provenance value may never site a load, resolve a `.`/`source` target, or
  answer an env construct: which oracle answers a site would then depend on what a
  HOST said, making oracle loading world-dependent and the plan unreproducible from
  its inputs. Today the restriction is trivially total (every non-⊤ word is
  `ProgramText` at this stage) — which is precisely why the door is closed NOW,
  ahead of `core`'s `seam-re-bind` folding captured values back into the value plane.
  Never widen the accessor; never read `ValueFlow` directly from that domain.
  As of `30Na` the wall is a GATE, not a sentence: `funcenv::admits_a_load` is the one
  seat, and `variable_text` consults it through `VARIABLE_PLANE_GRADE` — the whole-hog
  grade the variable plane carries, since `ValueEnv` records none per value. When
  `seam-re-bind` lands, that constant is what stops being true and the gate then
  refuses rather than resolving off a host answer. Rider (r30 load lane): the
  load-head evaluator reads the AST for STRUCTURE only (`ParamOp` decoded) and routes
  every variable read through the plane; `$0` is a controller-held constant on
  `DefinitionTable` (`ScriptSpellings`, both live spellings), never a variable —
  fence `the_load_head_evaluator_names_no_value_plane_accessor`.
- **funcenv-is-a-pre-pass-not-a-round** (`28K` §2; `cli/CLAUDE.md`
  the-fixpoint-owns-the-rounds-and-builds-nothing-else) — env resolution is computed
  ONCE from the ORIGIN model and joins the frozen set; the validity fixpoint's ratchet
  erases EFFECTS and holds NO authority over BINDINGS. A records-proven-dead branch
  containing a funcdef must never re-run resolution, and a license once withheld is
  never regained by a later round. Enforced structurally at both ends, lexically:
  `funcenv`'s own `this_module_names_no_fixpoint_reachable_type` (it cannot import a
  records/effect/erasure/verdict type) and the cli's
  `the_fixpoint_loop_body_calls_no_funcenv_entry_point`. Sh-dead definitions
  (overridden unconditionals, unset names) are BINDING facts, value-flow-tier: they
  never enter the erasure ledger, never spell `CommandEffect::Pure`, and never become
  plan-line outcomes — a dead definition's book text ships verbatim and executes.
- **shadow-refusal-is-provable-at-both-ends** (`28K` §1 `rul-silent-shadowing-refuses`) —
  `funcenv::contests` complains ONLY where both halves are proven: the WRITE side, that
  the INNERMOST frame held a DIFFERENT unit's `Defined` (an outer-frame binding is a
  bounded subshell shadow — the sanctioned regional-preference idiom — and `Undefined` is
  the `unset -f` blessing); and the READ side, that the environment can name the winner at
  the unit's exit (a conditional definition joins ⊤ there and provably shadowed nothing).
  Never guard-SHAPE recognition around load sites (`28K` §2 `rul-conflict-pass-is-semantic`);
  a same-FILE redefinition belongs to the pre-existing `216` e-1 refusal, not here.
- **visibility-is-full-positional** (`28K` §2, ACKED spike-tier 2026-07-31; the mechanism re-cut at
  `28Q` §1.3) — every SITE-KEYED consuming act (verdict, predict-at-site, probe-ship, vouch, guard
  eligibility) answers only from the definition live AT the site, read through
  `funcenv::LiveDefinitions`. A definition below a site licenses NOTHING at it. The mechanism is now
  RESOLUTION, not agreement: `definition_before` names the definition and `dorc_core::answering_row`
  selects the row IT produced, so resolving a site's identity through one file's argparse while
  reading another file's cells — one cell measured, a different one keyed, pope-sin tier
  (`271:rul-sin-ordering`), invisible to every golden — cannot be SPELLED. The retired shape computed
  a whole-unit winner and withheld on disagreement; `KindIndex`/`VerdictIndex` carried `source_of`
  for that check and no longer do. A row CARRIES the `DefinitionId` its own lift minted (`28Q` §1.1),
  so the seat compares ids and joins nothing; the two parsers agreeing on a funcdef span is what
  holds that up, and `every_lifted_role_row_carries_its_parsed_definitions_span` is where it is
  measured. Two escapes exist
  and both are named: a name the `DefinitionTable` does not know yields `NoOpinion`, where a SOLE row
  answers and plural rows withhold (the environment holds no opinion and load order may not adjudicate
  — `28K` §6; containment is `28O:fnd-two-parsers-disagree-on-funcdefs`), and
  `LiveDefinitions::unsolved()` is the explicit no-environment posture for hand-built indices and the
  instrument/hint lanes. Never re-derive positionality from spans or argv — it is a fact about the
  CFG node.
- **vocabulary-acts-stay-ambient** (`28M` §5.3) — the kind-owner trio (`__resolve`,
  `__disturbance_reaches_only`, `__state_stored_only_in`) is the ONE exception: single-occupancy,
  loaded from the ambient prefix, never routed through the positional oracle. They canonicalize and
  type for OTHER authors' sites, so answering differently at different lines of somebody's book is
  incoherent. The species test is `oracle::reserved::is_vocabulary_role`, keyed on the SUFFIX
  because that IS the distinction (`271:rul-family`); an in-book one refuses with a notice.
- **top-licenses-nothing** (rider 1; `28O:res-polyfill-binding-tops-pending-fold`) —
  `funcenv::unprovable` names every role name whose exit binding is ⊤, and the driver
  withholds those families SILENTLY (⊤ never complains). Not decoration: it is the entire
  reason the refusal may under-fire soundly, since an uncaught shadow can then grant
  nothing either. Anything that lets a ⊤ binding license — a probe ship, a vouch, a guard,
  an elide — breaks the soundness argument, not merely a test.
- **the-fold-decides-conditions-never-shapes** (`28M` §9) — `funcenv::analyze` is pessimistic
  conditional-constant-propagation over this domain: solve, decide the conditions the solved
  environment makes decidable, mask the arms those decisions prove dead (`cfg::Branch`,
  recorded by the lowering that wired the arm edges — never re-derived from adjacency), re-solve
  under a capped, MONOTONE mask. The decidable set is `dec-decidable-set-v0`, CLOSED and growing
  by NAME only: `command -v <literal name the unit DEFINES>` (the universe restriction is what
  keeps an ordinary host PATH probe out) and `[ -f <path the controller resolved as loadable> ]`,
  which decides TRUE only — and ONLY where the cwd at that line is determinate (no `cd` of a
  shape other than `/…`/`./…`/`../…` above it in the frame, no unknown `.`, no spliced body
  carrying either): the decision asserts a HOST fact from controller state and is sound solely
  under cwd-parity with the file in the shipped manifest (human-acked 2026-08-22; BUILT: the
  entry reads `LoadSites::clobbers` through `load_certainty` and refuses to decide under cwd-⊤ —
  `p-x-file-test-refuses-under-unknown-cwd` / `…-below-a-blind-load` promoted).
  Load-bearing because NOTHING ELSE catches it: `cd` is a blessed target-state-pure builtin
  (`value.rs`; `notes/199`) and forms no wall, so a wrong TRUE here masks the arm dead, can
  ERASE its bytes (`plan::erase`), and can hide a mutator or an unknown `.` from the wall
  computation — a wrong-elision route with no downstream repair.
  Keyed on condition DECIDABILITY, never guard SHAPE
  (`28K:rul-conflict-pass-is-semantic`). Because every environment answer now SELECTS A WINNER —
  each resolution seat directly, and `funcenv::never_live` for the dialect's minting scan — any
  widening of the decidable set is a WINNER-SHIFTING licensure surface: license-review-tier, never a
  convenience patch (`28P:adj-never-live-exactness-accepted`; `28Q` §1). Two riders carry the
  correctness: an UNREACHED node's
  transfer produces ⊥, not ⊤, or a masked-dead region still poisons the join it never reaches;
  and a decision, once taken, is stable under further masking (masking only removes paths, so a
  `Defined(d)` stays `d` or becomes unreached), which is what makes every intermediate state
  independently sound and running out of rounds a precision loss only.
- **never-live-feeds-the-dialect-fold-only** (`28M` §9, re-cut at `28Q` §1) — the fold reaches the
  BINDING; `funcenv::never_live` is what carries it to the ONE seat resolution does not cover. Its
  per-file WITHDRAWAL is RETIRED: every site-keyed act resolves through `LiveDefinitions`, and a
  never-live definition is named at no frame, so subtracting its rows bought nothing the lookup was
  not already doing. What survives is `dorc_oracle::build_dialect`'s whole-unit minting scan — which
  selector tokens the unit's authors minted AT ALL — a question with no frame to ask from, and one a
  dead define-if-absent body would otherwise answer by being last to DECLARE. The pairs travel to
  `lift_from_sets` as `binds_somewhere` data rather than as missing rows. Still EXACT, not
  conservative: it SHIFTS the minting winner rather than withholding, and a bigger or different
  dialect SPARES MORE, so it must be right — and it is, because a definition no program point
  binds is one no execution can call. Never widen it to "probably dead", and never let a seat
  read it for anything but the dialect. The scan itself is scheduled to go: the ruled dialect is
  per speaker closure, read at the backing's FRAME (`30J` §12), at which point `never_live` has
  no consumer left.
- **translation-fence-binds-lattice-too** (`304`) — `lattice.rs` is inside the TRANSLATED
  algebra tier (`spike/verify/aeneas/src/lib.rs` `#[path]`-includes this very file), so
  `core/CLAUDE.md`'s `keep-borrows-out-of-closure-returns` binds it verbatim: no
  Option-combinator whose closure RETURNS a borrow of its argument, no `mem::replace`
  inside `.map`, no `unwrap_or_else(<trait method>)` — spell the `match` cousin. A
  reintroduction breaks Aeneas translation SILENTLY (an ill-typed emission only
  `lake build` catches); nothing on the ordinary gate sees it, and only `verify:lean`
  does. The fence's own classes live in `spike/verify/aeneas/Cargo.toml`.

- **one-load-account-many-projections** (`30I:rul-one-load-account-separate-projections`)
  — the loader resolves each supported load occurrence ONCE into `load::LoadAccount`,
  keeping sourcer, target, locus, positional context and nesting; a consumer takes a
  PROJECTION and never builds a second resolver. Three, none substituting for another:
  `occurrences()` (possible-load — undecided-guard fallbacks INCLUDED, because a bundle
  built without them omits a file the runtime `.` may load), `speaker_edges()` (authority
  — a `File` sourcer on a non-speculative route, nothing else), `selection_edges()`
  (narrative — what an author NAMED, aligned or not; decision-inert, never handed to a
  custody consumer). Absence from the speaker projection is never absence from the other
  two. A `(sourcer, target)` pair set does NOT substitute for occurrences: two textual
  load points naming one entrypoint are two occurrences, which bundle keying
  (`30I:rul-bundles-key-to-load-occurrences`) and locator composition both need kept
  apart. The sourcer SPECIES keeps a book `.` and a CLI pre-source out of both edge
  relations by type, never by a filter each consumer must remember.
- **rul-havoc-is-pointwise-never-the-stack** (`30P:law-no-unsoundness-below-a-blind-act` +
  `30P:principle-unknown-source-is-a-point-havoc`; built r30) — a BLIND ACT (a `.` whose
  target the controller does not HOLD: an unheld file, a dynamic operand, an acquired
  plain-sh inclusion alike) havocs every binding AT THAT LINE in the frame it sits in; a
  later unconditional definition re-binds its own NAME by last-wins — which names the BYTES
  Dorc pins, never a runtime binding it trusts. Never the old absorbing stack-wide ⊤. Frames
  are SUBSHELL scopes (`cfg::lower_scoped` is the only `ScopeEnter`/`ScopeExit` producer): a
  `.` inside a spliced function body binds the CALLER's frame and survives the return; one
  inside `( )` dies at the paren. The cwd domain has ONE seat: `LoadSites::clobbers` seeds
  cwd-⊤ from every blind act and every `cd` whose operand is not `/…`/`./…`/`../…`
  (`HavocCause::CwdUnknown { clobbered_at }`); `FuncEnv::load_certainty(node)` is the one
  composed answer (a cwd-havoc'd site sits in `havoc_causes` AND `resolved_loads`/`named_loads`;
  a reader of one map alone calls it exact); the decidable set's `[ -f ]` entry READS it and
  refuses to decide under cwd-⊤. Below a blind act the load plane CLAIMS NOTHING: a relative
  `.` is not EXACT ⇒ no authority, no rewrite, NOTHING SHIPPED for it (the
  `load-carriage-withheld-under-unknown-cwd` code names the clobbering line; human-typed
  2026-08-23: `. /etc/os-release; . ./relative.sh` is inherently unsound and the remedies are
  admin-sourced, `notes/30Pd`). Acquisition is KEPT — dropping it stalls the acquisition
  fixpoint, and `Withheld`-over-`NoOpinion` is the safe direction. The law names THREE blind
  acts and the engine models all three: `eval` is closed by refusal, the unheld `.` seeds the
  clobber, and a call Dorc cannot SPLICE seeds it through `Cfg::splice_refused` — recorded at
  the ONE seat that mints the refusal diagnostic (`Builder::refuse_splice`; EIGHT arms — seven
  `CFG_INLINE_REFUSED` plus the depth-2 positional Note; the silent `?`-returns stay ordinary
  `Opaque`, which is what keeps a call into a HELD oracle body from counting as blind). The
  tempting wrong seed — `call_body_sites`
  answering `None` — is one line away and WRONG: the splicer inlines same-file funcdefs only,
  so `None` also covers every call into an oracle body Dorc holds and models, and reading it
  as blind clobbers the cwd below every helper call (measured: two goldens). Built carve, owed
  a ruling before it moves: a `.` inside a LOAD PROGRAM (`run_control`'s nested-load arm)
  keeps the absorbing ⊤ — the `30Mg` R1 prelude floor.
- **rul-rewrite-needs-exact-and-explicit** (née rul-exact-is-not-explicit;
  `30P:rul-rewrite-permission-is-derived` + the blind-act law) — EXACT governs AUTHORITY and
  CARRIAGE (bindings below the line, vouches, shipping); EXPLICITNESS governs whether Dorc may
  touch the LINE at all. A rewrite needs BOTH: `cli::artifact::BookLoad::permits`
  (`LoadPermission`: `may_rewrite()` = explicit ∧ exact, `may_ship()` = exact) is the one seat
  every rewrite seat and both placement seats read; three cells and no fourth — EXACT∧explicit
  ⇒ re-point/paste + bundle · EXACT∧inexplicit ⇒ verbatim, mirrored at the authored path ·
  ¬EXACT ⇒ verbatim, nothing shipped. `Literal` is decided through
  `SourceLiteralPlane::literal_text`, which constant-folds — a literal-assigned book-set root
  (`OPS_LIB=.; . "$OPS_LIB/x.oracle.sh"`) IS explicit; an AST-literal test would be WRONG.
  `kept_in_place_reason` never answers "operand not explicit" for an explicit-but-non-EXACT
  line — a wrong repair is `271:rul-sin-ordering`'s top. Residue: a positional the value plane
  overlaid under splicing (`. "$1"` in an inlined body) reads `Literal` too — the ruling's own
  literal-assigned-root clause, not a widening to defend.
- **rul-acquiring-bytes-is-not-modelling-them** (`30P:principle-book-code-source-is-inclusion`,
  tier 2 only) — `load::Loadable::Included` is a file the controller READ (a book-sourced
  source carrying no dorc-lang marker) and models not at all: `program_at_key` answers
  `None`, so its `.` site havocs and walls exactly as an unread one does; the only thing
  acquisition buys is an occurrence for the artifact to mirror. Its declarations never
  enter `DefinitionTable::names()`, which is why its names answer `NoOpinion` rather than
  `Withheld`. The ONE selection seat deciding what a lift sees is
  `cli::snapshot::StaticLoadSnapshot::modelled_refs`, which reads an inclusion as the EMPTY
  source (positional, never filtered — the index IS the `SourceFileId`).
- **rul-guarded-source-compares-the-sentinel-value** (`30I:rul-load-semantics-stay-full-fidelity`)
  — recognized guarded-source compares the sentinel's VALUE against the guard's literal,
  not only whose names are bound: the structural arm alone modelled a package assigning
  `v1` under a guard testing `v2` as reused where a real shell sources it again
  (`SentinelArm::Reuse` with a mismatched value answers `Source`). The lossy speech
  projection (`30I:rul-guarded-source-speech-is-lossy`) still asks the NAME question and
  must never gain the value one.

## Direction — the re-key (entity-algebra-rebuild; spec = `277` §§1–3 + §7b)

- **splice-budgets-are-licensure-not-perf** (`30L:req-census-admits-the-wrapped-book`) — the
  `cfg::inline_budget` constants are a LICENCE surface, not a performance knob: an un-spliced
  call is `Opaque` ⇒ ⊤ ⇒ a poison wall, so raising a budget makes mutations visible that were
  hidden behind a wall and makes downstream elisions available. They were re-sized against
  measured corpus-shaped strawmen (a 15-command wrapped book is 63 AST nodes; the inherited
  per-call 64 admitted exactly one such book and nothing larger), and each carries its
  measurement in its own doc comment. Move one deliberately, with the corpus re-measured, never
  to make a fixture fit — and any change here carries the same winner-shifting review posture
  as a funcenv precision change (`28Q` §1): budgets decide which mutations exist to the model.
- **thread-the-flat-coordinate** — `(kind, entity, selector)` + context slot
  through `FactKey` → `Reach` → `command_effect` → `classify`/`SkipClass`.
  Per-selector CELLS are the poison-wall fix: `apt-get update` establishes the
  package-index cell, `install` establishes `…Package:nginx@installed` —
  different cells, no cross-poison.
- **compare-only-at-chokepoints** — dialect sets + backing provenance (minting
  family) enter comparison inside core's chokepoints (`selector_covers`; the
  relational whole-coordinate compare). Never inline token equality anywhere in
  this crate. ⊤-selector (selector-less, either side) collides with every cell of
  its entity.
- **polarity-becomes-transitions** — the binary `Establish`/`Kill` bit becomes a
  typestate transition-table (install vs purge vs update are different
  transitions on one kind), still ⊤-conservative on lookup miss.
- **no-strong-update-v1** — `Kill` accumulates; "probably unique" may only DEMOTE
  (the 231 fence); the uniqueness bit is a reserved seam, never inferred hot.
- **erasure-is-applied-once-never-consulted** (`26H` §4 — W-C) — `classify_with_why_diags` takes
  an `erase::ErasedSites` overlay and applies it at ONE seam: the effect vector, before `solve`.
  Everything downstream (reach, `valid`, classes, kills, backings, ⊤-causes) derives from that
  residual, so an erased site is indistinguishable from one that never mutated and NO consumer is
  handed the overlay to consult. That uniformity IS the safety property, and it is why the
  rejected mask-parameter shape — a flag every present and future consumer must remember — is not
  coming back. Spelling is `CommandEffect::Pure`; an `Erased` variant would recreate exactly the
  must-remember surface. The same pass returns the INVALIDATOR set (Establish/Kill/Opaque, taken
  POST-erasure) because `SkipClass` cannot answer "does this gen into reach": a kill, an opaque,
  and a blessed pure builtin all classify `MustRun`.
- **origin-reach-is-probe-only** (`30K`, BUILT) — `Reach`, `Reach::is_pristine`, and
  `SkipClass::EstablishProbe{Ambient,Written}` answer ONE question: which check may ship,
  and which cell the authored model names. They carry no apply-time authority and their
  names say so. Apply-time freshness, effective Query validity, total walls, and
  footprint survival read `plan::world::ReachingWalls` and nothing else. Never make the
  two species generic behind one trait, and never convert effective reach back into probe
  eligibility.
- **classify-answers-with-its-invalidators** (`30K` §3.7) — `classify` returns
  `Classification { value, diags, invalidators }` because the invalidator set is NOT
  derivable from the classes: a `$( … )` body command, a write-shaped redirection, and an
  unmodeled construct all gen into the world without being leaves. A caller that drops it
  elides past a mutation nothing in its inputs can see. `cfg::ExecutionOwner` says whose
  decision governs each of them, recorded at lowering and never re-derived.
- **erasure-licence-is-a-fence-not-a-guarantee** — `erase::ErasureLicense::for_site` is PUBLIC and
  therefore forgeable: this crate cannot depend on `plan`, so the type system cannot prove a
  licence traces to a records-proven derivation. Do not read that seal as a type guarantee. The
  real seal is one layer up (`plan::erase`: a ledger entry demands a `DeadBranchProof`, which
  demands a `FoldResult` no foreign crate can populate); here the fence is LEXICAL —
  `licence_mint_has_exactly_one_caller` in `dorc-plan` fails if a second caller appears anywhere
  in the workspace. A new caller is not a refactor; it is a second, unproven route to shrinking
  the analyzer model.
- **verdict-lane-is-site-keyed** (`26H` §3; `28Q` §4 rul-verdict-primacy-at-the-ship-seat,
  BUILT at stage-0, 2026-08-16; the ratified reading is `307:rul-primacy-moves-the-body-never-the-cell`:
  primacy moves the measuring BODY and never the CELL — the predict author's declared
  coordinate remains the site's establish, so no invalidation, backing, or why-coordinate
  moves; "the cell the shipped body measures" means the cell whose CONVERGENCE the rc
  asserts, which is the site's cell. Member and inline-call aggregates retain an exact
  ordered measurement subject beside their predict-derived topology; only an exact
  all-vouched population may ship reached verdict bodies, and an incomplete population
  stays unable to replace (`30La`). The lane is a per-SITE out-param, never derived from
  the fact's kind and never by try-order over the ship closures (the shipped body's
  measurement must be the cell the record keys). A vouched, mutation-capable site is verdict-lane: the verdict
  body measures, keyed by the author's coordinate when the reached path carries exactly
  one fully-resolved verdict mark, else the `24L` §2 auto-cell, else `Opaque`. Prediction
  never licenses elision; predict cells feed the static concern topology whatever ships.
  Selection stays narrow: kind+entity from the reached BIND (never the mark's own entity
  text — `identity-declared-never-inferred`), selector from the mark, verdict marks only
  (an observe widens, never keys), and TWO marks on one path key NOTHING (one rc
  witnesses one cell — `281` §7 rc-arity).
- **verdict-minted-facts-thread-their-family** — a verdict-lane fact carries
  `family = Some(provider)` EXACTLY. `build_dialect` mints only from predict-derived cells,
  so a verdict-minted selector sits outside the sparing dialect and COLLIDES (the disclosed
  conservative fallback; registering them is its own future dispatch). Leaving the family
  to `sole_family`'s reverse lookup would hand this fact some PREDICT family's dialect and
  spare a cell no verdict mark ever minted a token for — the one sparing leak this lane
  must not open.

## Seams — watch and report, don't resolve

- **seam-interproc** — same-file eligible calls SPLICE (per-call-site body clone);
  body regions take REGION-level decisions through `plan::region`'s universal meet
  (`plans/30L` — the old all-or-nothing CALL license is retired; the call stays its
  own leaf and elides only derived, per `pin-whole-helper-derived-only`); cross-file
  `. /path` sourcing stays ⊤; the ineligible population stays Opaque. Watch: does
  the worklist scale, or beg for IFDS realizable-path summaries?
- **seam-prov** — the dependency graph is this crate's dataflow output; the
  hand-built derivation-DAG vs growing taint is the strongest later case for a
  relational layer.
- **substrate-decision-stands** — keep and extend the hand-rolled worklist (not
  IFDS-the-algorithm, not Datalog/Soufflé). Graduate to Ascent ONLY if all three
  coincide: why-trees heavily weighted ∧ structured lattice domain needed ∧ DST
  tolerates `default-features=false` (Ascent has no provenance — the exact lever
  it would be wanted for).

## Law — the solve-certifier (r30; spec `plans/302`)

- **solve-is-certified-only** — `solve` and the observer-bearing `run` are
  `pub(crate)`; every production answer routes through `solve_certified`, whose
  `SolveConsistency` is consumed at a NAMED floor (value · funcenv · reach ·
  self-reach). The whole-file lexical fence covers BOTH needles (`solve(`, `run(`)
  with a non-empty-walk assertion. `trusted()` means CERTIFIED — never re-read the
  advisory `converged` flag as a trust gate (consistent-at-cap is the lfp and is
  used).
- **certifier-trip-is-a-monotone-latch** (`302:rul-certifier-trip-guard-only`) —
  `certify::CertifierTrip` is a private-field monotone boolean; its ONLY mutator
  takes a real `SolveConsistency` (inheriting the checker's lexical fence), and
  nothing clears it. It threads as an OUT-PARAM through the fixpoint drivers so
  unobserved intermediate rounds still latch — a round-2 failure must not be
  invisible to readers of the settled round. All four consumer floors latch it;
  the cross-window CLEANUP consumer is plan-crate law (`plan/CLAUDE.md`).
- **floors-are-whole-window-and-demote-only** — an `Inconsistent` demotes the ENTIRE
  analysis window; summaries (first-break, unstable components) explain, never scope.
  The funcenv floor BREAKS the fold at the failing round with `folded_edges = ∅` — every
  environment answer is winner-shifting under true resolution and `never_live` subtracts exactly
  besides, so edges folded from uncertified states would GRANT
  (`303:fnd-never-live-is-the-grant-shifting-consumer`, generalized at `28Q` §1; pinned
  independently by `the_fold_breaks_to_its_floor_at_the_failing_round`). No recovery mechanism, ever
  (`302` §9: recovery failures correlate with the trigger's own causes).
