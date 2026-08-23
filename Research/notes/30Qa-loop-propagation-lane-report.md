# 30Qa — lane-loop-propagation: the census mint, and the seam plan

> Tier: builder lane report (Opus, `ai/r30-lane-loop`). Charter `30L:loop-propagation-staged-now
> (née §7)` scheduled as `30O:lane-loop-propagation`. Grades: +SURE / ~SUSPECT / -GUESS /
> --WONDER. Written at the lane's one checkpoint; §1 is the seam plan the conductor rules before
> the lane resumes.

## §0 — what landed at the checkpoint

Two commits on `ai/r30-lane-loop` over `aabcc2d9`:

- `2e6ee24e` — `Cfg` records, per node, the INNERMOST enclosing loop's `LoopHead`
  (`Cfg::enclosing_loop_head`), marked by the same arena-range pass that already sets `in_loop`,
  innermost-wins because a nested loop's own pass runs first.
- `8bc9a039` — `plan::region::census` classifies each spliced body node's enclosing loop
  (`LoopEvaluations::{Once, Members(u32), Unenumerable}`) and mints one `RouteInstance` per
  ORDERED member, `iteration: IterationSlot::Member(n)`, on the SAME lowered `cfg_node`. Only
  `Unenumerable` still opens the population. The xfail pin
  `p-x-loop-population-closes-over-literal-members` is PROMOTED (registry row dropped, assertions
  kept as an ordinary test); the interim `a_literal_loop_population_is_open_today` is replaced.

**`fnd-census-mint-changes-no-behaviour`** (+SURE, measured): whole suite 2573/2573, zero golden
drift, `mise run both gate:full-quiet` green on both legs. The reason is `fnd-in-loop-floor-is-the-
whole-seam` below — a closed member population reaches `settle::decide_regions`, and every route's
conclusion is floored to Run at the site seat, so the region meets to Run exactly as an Open one
did. The census now carries the IDENTITY; the licence is entirely unbuilt.

**`dec-member-count-is-syntactic`** (built): the count comes from the `for` list ALONE — every
word plain literal after ordinary quoting, no expansion / command substitution / positional /
unquoted glob / leading tilde; `while`/`until`, an empty list, and a nested loop are
`Unenumerable`. Field splitting applies to the results of expansions and never to literal text, so
N literal words are N members, ordered, duplicates kept (`for x in a a` = two —
`30N` §2's `20S` commitment). Deliberately NOT routed through `ValueFlow::member_argv`, whose
`record_member_sites` gate (`analysis/src/value.rs:984`) records only sites whose own argv
references the loop variable: the commonest in-loop call ignores it, and the count would be zero.

**`fnd-member-count-is-an-upper-bound`** (+SURE): a body that `exit`s or `return`s early evaluates
a PREFIX of the members. Over-counting is the safe direction — a universal meet over a superset of
the executing routes can only refuse more. `break`/`continue` are parse-⊤ today, so no other
early-exit shape reaches here.

## §1 — SEAM PLAN (the checkpoint deliverable)

### `fnd-in-loop-floor-is-the-whole-seam` (+SURE, quoted at the seat)

`analysis/CLAUDE.md errexit-couples-build-and-solve`'s "no in-loop license" floor is spelled in
`plan` at three places, and the region lane meets the first two:

```rust
// plan/src/lib.rs:4651-4655, `site_conclusion`
// (0) the in-loop render floor (task-L1, `209` brk-1): the line-granular render cannot elide
// one iteration, and per-iteration deadness is not line-expressible.
if p.cfg.in_loop_body(p.node) {
    return (DecisionConclusion::Run, SurvivalAccount::Silent);
}
```

```rust
// plan/src/lib.rs:4582, `region_guard_candidate`
if has_top_successor(p.cfg, p.node) || p.cfg.in_loop_body(p.node) { return None; }
```

(The third, `inline_disposition`'s explicit in-loop refusal at `:4822`, governs the CALL leaf and
stays as it is — the call keeps running; the region is where the value lands.)

The floor's stated REASON is line-granular render expressibility: it cannot elide ONE iteration.
That reason is exactly discharged by a region decision over a CLOSED population — the edit lands
once at the authored definition span and is universally quantified over every member, so no single
iteration is being singled out. The floor must therefore become route-aware rather than
node-aware, and NOT be deleted: it still binds every per-SITE leaf decision. Proposed spelling: a
new field on `DecideSite`, present only when the caller is the region seat AND the route belongs to
a closed population, which the two floors above consult. `30L:rul-shared-region-needs-universal-must`
is what earns the lift, so the field should name that and not the loop —
`universally_quantified_member: Option<u32>` is my lean over `iteration`/`in_region` (both would
read as "which loop" rather than "why the floor lifts"). Conductor's call.

### `dec-per-member-facts-need-no-re-keying` (~SUSPECT, mechanically checked)

Everything downstream of the per-member FACT is already fact-keyed or `(node, fact)`-keyed:
`Vouches::get(node, fact)`, `observe(fact)`, `AggregateEstablish::new(node, fact)`. So the ONLY
thing the seam must produce is: at a shared `cfg_node`, the ordered per-member `FactKey`. Given
that, no identity moves — `ElisionRegion`, `SiteId`, `ProbePredict`'s key, and the witness types
are all untouched, which is what `30L:pin-loop-types-need-no-rekey` promised.

### `dec-members-ride-the-inline-site-vector` (-GUESS; the recommended shape, unbuilt)

`SkipClass::InlineCall { sites: Vec<InlineSite> }` already IS an ordered aggregate measurement
population, and `ProbePredict.member` (the `M` of `site N.M`) already means "index into it" for
this lane. The proposal is to let one call's `sites` carry one entry per (body-site, member) pair,
with `InlineSite` gaining the member ordinal so a route can find its own entry. Then:

- the value plane generalizes the positional overlay: `Prep::inline_pass` (`value.rs:1086`) binds
  ONE argv per spliced body node today; for a call whose enclosing loop is member-closed it binds
  one per member, resolving the call's own argv with the loop variable set to that member — the
  `members_pass` substitution, applied to the CALL rather than to the body site. Shape becomes the
  `member_argv` shape (`BTreeMap<CfgNodeId, Vec<Vec<ValueOf>>>`).
- `effect` resolves each member's argv through the oracle exactly as it does one, producing the
  ordered per-member establish entries.
- `settle::decide_regions`'s `body_class` (`settle.rs:687-697`) is the concrete bug this must fix:
  it is `BTreeMap<CfgNodeId, &SkipClass>` built by `.map(|site| (site.node, &site.class))`, so N
  member entries at one node collapse to the LAST one silently. It becomes a lookup keyed by the
  route's `(cfg_node, iteration.member())`.
- `push_inline_checks` (`lib.rs:4099`) then ships one `site N.M` check per member with no change
  to the record grammar, and `settle::aggregate_establishes` builds the exact ordered population
  with no change to `AggregateEstablishes`.

Verification owed before building it: whether anything zips `sites` against
`cfg.call_body_sites(node)` positionally (they would desync). ~SUSPECT nothing does.

### `tc-duplicate-member-establishes-refuse` (FLAG, do not resolve)

`AggregateEstablishes::mint` (`survival.rs:842-854`) rejects a duplicate `(site, fact)` pair. Two
members of one loop establishing the SAME cell at one node — the pin's own book,
`for pkg in nginx curl; do install_pkg; done` where `install_pkg` takes no argument — is exactly
that, so the aggregate refuses and the region runs. The argv-threaded product shape is unaffected
(the members establish different cells).

Strawman, both cells:

```sh
install_pkg() { apt-get install -y nginx; }        # both members establish ONE cell -> refuse, Run
for pkg in nginx curl; do install_pkg; done

install_pkg() { apt-get install -y "$1"; }         # two cells -> mint, and the region can Replace
for pkg in nginx curl; do install_pkg "$pkg"; done
```

My lean: LEAVE IT REFUSING. Widening the identity to `(site, member, fact)` is a change to a proof
identity type, it is winner-shifting, and the value it buys is a loop whose body has no per-member
variation at all (eliding it makes the body `true` and the loop still spins). But it is a genuine
forfeit and the conductor should rule rather than have me bury it.

### `ask-witness-order-execution-or-census` (FLAG)

The census sorts routes `(cfg_node, iteration)` — site-major. EXECUTION order for a member
population is member-major (iteration 0 runs every body site, then iteration 1), and
`pin-shared-witness-spans-instances` wants "the exact ordered union of every contributing
instance's establish", which `AggregateSurvivalWitness` then crosses walls member-by-member. If
the witness must be in execution order the census sort should be `(iteration, cfg_node)`. Nothing
consumes it yet, so the change is free right now and expensive later. A population mixing
`NotIterated` and `Member` routes for one region has no execution order the census can know, so
"census order" is a deterministic convention either way — but which convention is a ruling.

### `dec-suppression-set-is-already-right-for-members` (~SUSPECT)

`settle.rs:709` builds `suppress: BTreeSet<CfgNodeId>` from the routes' `cfg_node()`s, which
DEDUPS an N-member population to one node. That is correct rather than lucky: suppressing the one
shared node removes every iteration's write from the graph at once, because the back-edge carries
no second copy — the same fixed-point argument `effect::self_reach_holds` makes for the r21 Members
lane. What is NOT right is the trigger: `(routes.count() > 1)` at `:711` skips the self-suppressed
solve for a ONE-member closed population (`for pkg in a; do install_pkg; done`), whose single node
still reaches itself over the back-edge and would read its own establish as a wall. The trigger
should be plural-population OR any-route-iterated.

### `fnd-region-routes-account-double-counts` (+SURE)

`settle::region_routes` (`settle.rs:771-793`) keys every contributing route
`SiteId::leaf(leaf)` — member `None`. An N-member population therefore writes N identical Spine
rows for one invocation, which `core/CLAUDE.md a-record-says-what-its-population-holds` forbids.
The fix needs no new axis: `SiteId` already carries `member`, and `IterationSlot::member()` is
spelled to feed it. One line, and it belongs in the first post-checkpoint commit.

### `dec-member-family-stays-a-separate-mechanism` (DEVIATION from the brief, reported)

The brief asks the effect plane's `member_family` → `SkipClass::EstablishMembers` to be
"reconciled" so the region census's establish population is exact and ordered. Measured, they are
two different SYNTACTIC shapes and should stay two mechanisms: `EstablishMembers` fires for a
mutating command written DIRECTLY in a loop body whose own argv references the loop variable, and
never for a function call in a loop; the region lane's mutating site is inside a spliced body whose
argv references `$1`. Unifying them would mean re-keying the site-keyed `EstablishMembers` onto the
route axis for no product gain. What they SHOULD share is the discipline, not the code: exact
ordered population, all-or-nothing, identity- and cardinality-matched
(`rul-every-erased-establish-is-vouched`). Conductor may overrule.

### What becomes unrepresentable, and what still is not

Made unrepresentable by the landed census: a population that is closed while holding a route the
census could not enumerate (`Unenumerable` is the sole opener path and it forces `Open` at one
seat); a member population that lost its order or deduplicated its members (`IterationSlot::Member`
is an ordinal, built from list positions into a `Vec`). Still ADMITTED, product-wide: two routes
sharing both axes (`(cfg_node, iteration)` uniqueness is an invariant of the one mint, not of the
type); a `Member(n)` whose `n` exceeds its loop's list length; a `NotIterated` route at an in-loop
node (unreachable today, unforbidden). The last one is what the proposed `DecideSite` field must
not read as a licence.

## §2 — post-checkpoint: what the rulings changed, and where the lane stopped

### Answers the rulings asked for

**`fnd-in-loop-inline-calls-already-ship-records`** (+SURE, measured, and now pinned by
`an_in_loop_calls_member_zero_records_key_exactly_as_the_non_loop_call_does`): an in-loop inlined
call ships `site 0.0` TODAY, byte-identical to the same call outside a loop. So there IS something
to preserve, and member-major flattening is what preserves it — member 0's body-site indices ARE
the whole non-loop numbering and every later member appends. A site-major flattening would renumber
every existing record, which is why the ruling's ordering principle is load-bearing rather than
cosmetic.

**`fnd-count-guard-was-right-by-coincidence`** (+SURE, exclusion-checked both cells). The retired
`(routes.count() > 1)` trigger at `settle.rs:711` was CORRECT for a lone NON-loop instance and
WRONG for a lone member. The distinction it encoded is real but is not the one it named: a node's
own gen reaches its own in-state only through a CYCLE, and outside a loop there is none — so
cardinality and acyclicity coincided. Inside a loop the back-edge exists whatever the list length,
so `for pkg in a; do install_pkg "$pkg"; done` would have read its own establish as a wall. Keyed on
CLOSURE now; the extra solve on an acyclic population returns the identical answer.

**`dec-two-numberings-are-named-apart`** — `IterationSlot::member()` keeps the MEMBER ordinal and
never the flattened record index; the value plane's per-member argv is `spliced_member_argv`,
deliberately not `member_argv` (r21's, for a mutating command written DIRECTLY in a loop body and
keyed on the site's own argv naming the loop variable). Two numberings, two names, and the doc on
each says which.

### What landed after the checkpoint

- `74e997d9` — the four red cells, XFAIL, plus the record-identity preservation pin and the
  `$(ls)` census cell (brief case (f), which never reaches the census at all: a command
  substitution in a `for` list is a parse-⊤, so the body is never lowered).
- `1a46a191` — `region_routes` keys by member; the suppression solve keys on closure.
- `fae42488` — the loop-member enumeration moved to ONE seat (`cfg::loop_evaluations`, returning
  the ordered member TEXTS so the census takes the count and the value plane takes the values),
  and `Prep::member_inline_pass` binds each member's own call operand into the spliced body it
  calls. Three tests pin it, including the load-bearing one: `install_pkg "$pkg"` under
  `for pkg in nginx curl` resolves `$1` to nginx at member 0 and curl at member 1.

### `dev-effect-probe-and-floor-remain` — where the lane stopped, and why here

The licence is NOT built. Three edits remain and they must land TOGETHER, because any two of them
without the third is either dead code or a wrong licence:

1. **effect** — the per-member classes on the InlineCall vector, member-major. The reusable seat is
   `effect::member_family` (`effect.rs:735`), which already resolves one member argv through
   `command_effect` with THROWAWAY out-params and already handles per-member verdict measurement.
   The region lane wants the same shape keyed off `spliced_member_argv` for spliced BODY sites,
   with `InlineSite` gaining the member ordinal so a route can find its own entry. Why this is the
   deep one: `command_effect`'s real out-params (`verdict_lane`, `backings`, `degrades`, the
   diagnostics) are all NODE-keyed, and N members at one node collide in every one of them —
   `member_family`'s throwaway maps are exactly how the r21 lane sidestepped that, and the same
   sidestep has to be justified again here rather than assumed.
2. **probe** — `push_inline_checks` (`lib.rs:4099`) ships per member off `spliced_member_argv`
   instead of `argv_values`. No record-grammar change under member-major.
3. **consumer** — `settle.rs`'s `body_class` (`:687-697`) keyed by `(cfg_node, member)` rather than
   collapsing N members to the last one, and the in-loop floor made route-aware at BOTH seats
   (`lib.rs:4651-4655`, `:4582`).

**The floor moves LAST, and that is a correctness statement, not a schedule.** Lifting it before
the per-member facts exist would license a region from ONE member's fact applied to all of them,
which is an under-execution — the cardinal sin. There is therefore no safe partial slice of this
licence, which is why the lane stops at a green boundary rather than part-way through.

## §3 — riders

- `analysis/src/value.rs` line ranges TOUCHED, for the load-plane lane: the `ValueFlow` field block
  (one field added after `positional_argv`, ~`:107`), one accessor beside `member_argv`
  (~`:190`), one line in the `analyze` assembly (~`:346`), and a new `Prep::member_inline_pass` +
  `loop_var_of` inserted immediately BEFORE `resolve_site_words_with_positionals` (~`:1150`).
  `variable_before` and the decidable set are UNTOUCHED. Also touched, and shared with no
  scheduled lane: `analysis/src/cfg.rs` (the enclosing-loop record; the `loop_evaluations` seat).
- Comment budget: inline `//` = 22 (brief: ≤25); `///` = 172, sized to the pin count — two new
  public seats in `cfg`, one public field + accessor and two private passes in `value`, one
  recorded CFG field pair, and the per-test invariant arguments spike style requires.
- FORFEITS: `forfeit-loop-populations-open-until-propagation` is NOT dischargeable — the census
  closes, the licence does not. Its CAPTURE sentence must stop naming the now-promoted pin as its
  standing red; the four `loop30-*` e2e cases are its reds. Drafted for the conductor to apply.
