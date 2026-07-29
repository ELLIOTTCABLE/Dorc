# spike/crates/analysis — CLAUDE.md

Role: the engine — CFG construction (`cfg.rs`), the monotone-dataflow worklist
(`lattice.rs` + `solve.rs`), effect/classify (`effect.rs`). The densest crate and
the entity-algebra's largest consumer. Read `spike/CLAUDE.md` first; deep design
rationale: `Research/notes/163`. The built machinery is code-visible — read the
code; this file carries only the dangerous, easy-to-miss parts. Registry
discipline: one rule per bullet, slugged; append to the matching section.

## Law — the dangers (each one is a latent wrong-elision or a hang)

- **solve-termination-unenforceable** — the worklist's preconditions (monotone
  transfer · finite-height domain · semantic `Eq`) are caller-upheld and
  un-type-enforceable; a violation HANGS (empirically hundreds of CPU-seconds).
  `solve` carries an iteration cap and returns `converged == false` instead —
  every correctness-critical caller MUST check it (`trust_reach` is a
  per-consumer obligation, never ambient).
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
  `StatusRelaxable`; loop bodies are flagged `in_loop_body` (the structural floor
  `plan` honors — no in-loop license).
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
- **top-licenses-nothing** (rider 1; `294:res-polyfill-binding-tops-pending-fold`) —
  `funcenv::unprovable` names every role name whose exit binding is ⊤, and the driver
  withholds those families SILENTLY (⊤ never complains). Not decoration: it is the entire
  reason the refusal may under-fire soundly, since an uncaught shadow can then grant
  nothing either. Anything that lets a ⊤ binding license — a probe ship, a vouch, a guard,
  an elide — breaks the soundness argument, not merely a test.

## Direction — the re-key (entity-algebra-rebuild; spec = `277` §§1–3 + §7b)

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
- **erasure-licence-is-a-fence-not-a-guarantee** — `erase::ErasureLicense::for_site` is PUBLIC and
  therefore forgeable: this crate cannot depend on `plan`, so the type system cannot prove a
  licence traces to a records-proven derivation. Do not read that seal as a type guarantee. The
  real seal is one layer up (`plan::erase`: a ledger entry demands a `DeadBranchProof`, which
  demands a `FoldResult` no foreign crate can populate); here the fence is LEXICAL —
  `licence_mint_has_exactly_one_caller` in `dorc-plan` fails if a second caller appears anywhere
  in the workspace. A new caller is not a refactor; it is a second, unproven route to shrinking
  the analyzer model.
- **verdict-lane-is-site-keyed** (`26H` §3) — `command_effect` reaches the verdict lane
  from TWO fallbacks (nothing resolved this argv; something RESOLVED but declared no cells
  for its verb) and answers with the author's coordinate when the reached path carries
  exactly one fully-resolved verdict mark, else the `24L` §2 auto-cell, else `Opaque`. It
  reports the lane as a per-SITE out-param, because the second fallback leaves a shippable
  predict on a site whose cell the VERDICT body owns — try-order over the ship closures
  would measure the wrong cell. Selection is narrow on purpose: kind+entity from the
  reached BIND (never the mark's own entity text — `identity-declared-never-inferred`),
  selector from the mark, verdict marks only (an observe widens, never keys), and TWO
  marks on one path key NOTHING (one rc witnesses one cell — `281` §7 rc-arity).
- **verdict-minted-facts-thread-their-family** — a verdict-lane fact carries
  `family = Some(provider)` EXACTLY. `build_dialect` mints only from predict-derived cells,
  so a verdict-minted selector sits outside the sparing dialect and COLLIDES (the disclosed
  conservative fallback; registering them is its own future dispatch). Leaving the family
  to `sole_family`'s reverse lookup would hand this fact some PREDICT family's dialect and
  spare a cell no verdict mark ever minted a token for — the one sparing leak this lane
  must not open.

## Seams — watch and report, don't resolve

- **seam-interproc** — same-file eligible calls SPLICE (per-call-site body clone;
  all-or-nothing CALL license); cross-file `. /path` sourcing stays ⊤; the
  ineligible population stays Opaque. Watch: does the worklist scale, or beg for
  IFDS realizable-path summaries?
- **seam-prov** — the dependency graph is this crate's dataflow output; the
  hand-built derivation-DAG vs growing taint is the strongest later case for a
  relational layer.
- **substrate-decision-stands** — keep and extend the hand-rolled worklist (not
  IFDS-the-algorithm, not Datalog/Soufflé). Graduate to Ascent ONLY if all three
  coincide: why-trees heavily weighted ∧ structured lattice domain needed ∧ DST
  tolerates `default-features=false` (Ascent has no provenance — the exact lever
  it would be wanted for).
