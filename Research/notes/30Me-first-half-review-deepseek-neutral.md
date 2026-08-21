# 30Me first-half review — deepseek-neutral

Scope covered: `spike/crates/plan` (settle, world, lib, survival, rederive, certifier_trip, erase, fold), the crate-local `CLAUDE.md` laws, `spike/CLAUDE.md`, `Research/plans/302-solve-certifier-spec.md`, `Research/plans/309-spine-reification-and-projections.md`, `Research/plans/30I-static-loading-and-bundle-emission.md`, and `Research/notes/30K-effective-world-reach-work-order.md`. Out of scope by instruction: `minispec/`, `spike/verify/`, `Research/corpora/`. Nothing was executed; this is read-only. Findings are ordered by design-importance, all LOW.

---

## Finding 1 — wall-formation account dropped on the honest path (`finding-wall-formation-account-honest-policy-dropped`)

Severity: **LOW**. Confidence: **+SURE** that the code does what it says; **+SURE** it disagrees with the quoted law text; **~SUSPECT** the severity is as low as I rate it (the consumer does not exist yet, see below).

**`spike/crates/plan/src/settle.rs:512-519`** (verbatim):

```rust
// `30K` §7 asks for a wall-formation account per effective mutation act; it is minted only
// under the risk-accepted policy, exactly where it was minted before. DEVIATION, reported:
// nothing consumes the record yet (`289:seam-narrative-render-unconsumed`), so widening it
// to the honest path buys no account and costs every why-transcript an `[unnarrated: …]`
// line. It widens with its consumer, not ahead of one.
if accounts_survival && matches!(decision.act, EffectiveAct::MayMutate(_)) {
    walls.push(*leaf);
}
```

where `accounts_survival = matches!(inputs.policy, WallPolicy::RiskAccepted { .. })` (`settle.rs:458`).

The design text it rests on — `Research/notes/30K-effective-world-reach-work-order.md` §7 ("Spine, narratives, and rendering"), the "Required final accounts" list:

> - each final disposition, as today;
> - each clean/survived/demoted/re-derivation survival result, as today;
> - **wall formation for each effective mutation act that reaches a dependent decision;**
> - Query invalidation and effective-reach collapse operands, through existing narrative species where truthful;
> - the certifier result and terminal cleanup, as today.

Reasoning: the §7 list carries **no policy qualifier** on the wall-formation account — it is unconditional. The implementation mints `CollapseKind::WallFormation` only under `WallPolicy::RiskAccepted`. The `Honest` policy (the default) produces no wall-formation account for an effective mutation act, so a dependent decision reached through a wall on the honest path has no narrative record. The code comment is a self-disclosure of the deviation, so this is not a hidden divergence — it is an *acknowledged* one.

Why LOW rather than higher: the record's only consumer is the why-chain render, and that seam is named-unconsumed (`289:seam-narrative-render-unconsumed`, per `spike/CLAUDE.md`: "only `VerdictDecline` carrying an `authored_reason` is CONSUMED by a render … a missing narrative omits SILENTLY"). So today no visible output differs, and the freshness/disposition decisions are unaffected — the wall itself is still computed and still floors the dependent decision; only the *account* is missing. The finding is a design-fidelity gap, not a wrong-elision or a wrong-render. It becomes consequential exactly when the why-lens starts consuming wall-formation records, which is when the comment itself promises to widen.

---

## Finding 2 — the records-grounded fence is incidental, not typed (`finding-records-grounded-fence-incidental-not-typed`)

Severity: **LOW**. Confidence: **+SURE** about every code fact below (all read directly); **~SUSPECT** that this is worth flagging as a divergence rather than a nit; **-GUESS** whether a live input can actually slip through (I could not execute; see the residual-cell note).

**`spike/crates/plan/src/fold.rs:34-41`** — the `AbstractRc` type and its doc:

```rust
/// A node's abstract exit status: a concrete value, or ⊤ (unknown). The fold's
/// lattice is the flat one (`Flat<i32>`-shaped) — two distinct knowns never need to
/// join here (a node has one status), so ⊤ is the only non-concrete element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbstractRc {
    /// A statically-known exit status (from a probed observable, replayed through the
    /// shell's own operator semantics).
    Known(Rc),
    /// Unknown / not-probed / unmodeled ⇒ ⊤. No fold through this (`inv-kfail`).
    Top,
}
```

The law it rests on — `spike/crates/plan/CLAUDE.md`:

> **erasure-is-records-grounded-only** — a statically-known controlling rc (empty list, bare assignment, funcdef — all rc 0 in the fold) is SOUND but is not a measurement, and the ledger's name promises records. Those branches keep today's behaviour; widening to static deadness is its own future design with its own name, never a quiet relaxation of this predicate.

Reasoning: `AbstractRc::Known` is minted from **two** disjoint sources. Measured: `fold.rs:153` (`AbstractRc::Known(rc)` from `observe(id).status = Predicted::Value(rc)`). Static shell-semantics rc-0: empty `List` (`fold.rs:132`), empty `Pipeline` (`fold.rs:161`), `FuncDef` (`fold.rs:206`), `Word`/`Assign`/`Redir` (`fold.rs:234`), and a false-cond-`if`-with-no-`else` (`fold.rs:307`). The enum does **not** distinguish the two — the doc comment's parenthetical "from a probed observable" is therefore inaccurate for the static mints.

`prove_dead_branches` accepts either: `spike/crates/plan/src/erase.rs:169` is

```rust
let AbstractRc::Known(controller_rc) = fold.rc_of(controller) else { continue; };
```

with no "was this rc measured?" check. The `DeadBranchProof::controller_rc` accessor (`erase.rs:101-103`) likewise documents "The controller's **measured** rc — the number the whole derivation rests on," which the type does not guarantee.

What actually keeps the named static-rc-0 controllers out of erasure is an *emergent* property of `controller_substitutes_away`: it returns `any_leaf && all` (`erase.rs:243`), and `subtree_leaves_all` returns `_ => true` **without setting `any`** for `FuncDef`/`Word`/`Assign`/`Redir`/`Unsupported` (`lib.rs:5397-5398`, "funcdef body is detached; word/assign/redir/unsupported carry no command leaf"). So an empty list, bare assignment, or funcdef controller yields `any_leaf = false` ⇒ `false` ⇒ no proof. I am +SURE those three are blocked — I traced each through the code.

The divergence is not behavioral for the law's three enumerated constructs; it is that the boundary the law names as load-bearing ("never a quiet relaxation of this predicate") is not represented in the type. It lives in a traversal that happens to return `any=false` for those constructs. A future widening of `subtree_leaves_all` to descend into funcdef bodies (for any unrelated reason) would silently convert `AbstractRc::Known(0)` static controllers into erasure licenses with no check tripping, and the wrong doc comment ("from a probed observable") actively invites that misreading.

Residual cell I could not close by reading: a false-cond-`if`-with-no-`else` nested as a *controller* (`if if q; then probe; fi; then a; else mutator; fi`, with `q` and `probe` both `QueryResolvable`) would mint `controller_rc = 0` from the fold's `if`-no-else rule rather than a measurement of the inner `if`. That is pathological and I could not verify reachability through classify/CFG without executing, so I do not assert it as a live bug — I note it as the cell the current incidental fence does *not* obviously cover.

---

## Finding 3 — synthetic consumer-map test cannot fail (`finding-synthetic-consumer-map-test-cannot-fail`)

Severity: **LOW**. Confidence: **+SURE** (the test body was read in full).

**`spike/crates/plan/src/survival.rs:2004-2054`** — `synthetic_cross_generator_consumer_map_holds`.

The "consumer map" assertions the test's name promises are over closures **defined inside the test**:

```rust
let survival_spares = |r: Relation| matches!(r, Relation::ProvablyDisjoint);
let transport_licensed_by_relation = |_r: Relation| false;
assert!(!survival_spares(Relation::Unknown), ...);
assert!(!transport_licensed_by_relation(Relation::Unknown), ...);
assert!(!transport_licensed_by_relation(Relation::Overlaps), ...);
assert!(!survival_spares(Relation::Overlaps), ...);
```

These four assert the hardcoded behavior of local closures, not the production `compare` / `Relation` / `disjoint` path. `survival_spares(Relation::Unknown)` is `false` because the closure literally matches only `ProvablyDisjoint`; it cannot fail even if production `compare` started returning `ProvablyDisjoint` for everything. Only two assertions exercise production code:

```rust
assert!(dorc_core::coord::selector_identifies(Some(sel), Some(sel)), ...);
assert!(!dorc_core::coord::selector_identifies(None, None), ...);
```

This falls squarely under the review remit's third clause ("a landed test that cannot fail even if the behavior it names broke"). It is LOW because the comment self-labels it "SYNTHETIC", and the real ternary-compare consumer split (`same`→transport-only, `provably-disjoint`→survival-sparing-only, `unknown`→safe-bottom-for-both, `spike/CLAUDE.md` `ternary-compare-consumer-map`) is pinned elsewhere through `recheck_survival`/`disjoint` in `rederive.rs`/`survival.rs` tests that do drive production `compare`. So the loss is a mislabeled pin, not an untested invariant.

---

## Overall assessment

Checked and solidly right (the things I specifically went looking for, and the results):

- **`erasure-demands-a-proof-and-a-rendered-death` is faithful.** `prove_dead_branches` (`erase.rs:135-191`) demands all four conditions in order: fold-proved-dead from a known controlling status (`dead_controller` + the `AbstractRc::Known` match), membership in the caller's `invalidators` set (never `Disposition`), not floored (`in_loop_body` / `has_top_successor`), and controller-substituted-away via the shared `query_substitutes` seat with `leaf_has_heredoc` as the *only* refusal — `leaf_has_blocking_output_redirect` is deliberately absent, and the code comment states exactly the ladder-idiom rationale the law records. This is the review's stated centre of gravity and it is right.
- **`certifier-trip-cleanup-runs-in-every-driver` is satisfied.** Both production plan drivers call `demote_on_trip` (`cli/src/main.rs:1839`, `cli/src/world.rs:484`); the `build_plan` calls at `main.rs:6630` and `why.rs:3034` are `#[cfg(test)]` fixtures, not plan-producing drivers. `demote_on_trip` demotes Replace/Omit to run, stands guards only on the syntactic `occupancy` census, and its tests use a *real* trip (`a_real_trip()` first asserts `!outcome.is_consistent()`) — not vacuous.
- **`rederivation-is-demote-only` is faithful.** `recheck_survival` consumes the `SurvivalWitness` by value and returns only `Confirmed(witness)` / `Demoted(..)`; the adapter never calls production `compare`/`selector_covers`; `CLAIM_SIDE_FAMILY = FamilyToken::new(u64::MAX)` fences it lexically.
- **The `Grade::Must` hardcoding in `establish_disposition` / `prove_members_replaceable` is not a divergence.** Every establish-class fact is oracle-declared ⇒ Must-grade; `Grade::May` is the not-yet-built "mined/distributional" seam, and the fence is pinned by `prove_replaceable`'s `grade != Grade::Must` check plus the `no_license_for_may_grade` test. The ambient/written split now rides freshness (effective reach), not grade, per the 30K re-key.
- **`demote_on_trip`'s guard-standing census** correctly consults `DefinitionTable::occupancy` (syntactic count, no solve), matching the "independence IS its admissibility" clause.

Net: no wrong-elision or wrong-render defect found. The three findings are all design-fidelity / maintainability-tier — one acknowledged deviation with a promise to widen with its consumer, one untyped (and mis-documented) boundary that is currently behaving correctly by accident of traversal, and one test that overstates what it pins.