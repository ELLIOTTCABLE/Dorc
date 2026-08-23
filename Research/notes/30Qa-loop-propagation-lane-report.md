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

## §licence — the three edits, what greened, and what measuring them found

> Tier: builder lane report, second half (Opus, `ai/r30-lane-loop-2`, branched from `69977343`).
> Appended per the append-only rule; §0–§3 above are the checkpoint half and are unedited.

### §licence.0 — what landed

Six commits over `69977343`:

- `c0a44d9f` — the harness floor: a `book.sh`-bearing case dir carrying anything else but no
  `expected.out` is an authoring error, minted as a RED trial with the residue named, never
  classified as a real-tools fixture and dropped (`fnd-missing-expected-out-hides-a-case`).
- `e5044b21` — **effect**: `spliced_members` resolves each spliced body site once per member off
  `ValueFlow::spliced_member_argv`, the node gens every member's cell, and the owning call's
  `InlineSite` vector carries one entry per `(member, body-site)` — MEMBER-MAJOR.
- `83a7cef5` — **probe + vouch + consumer**: `member_argv` is the one seat answering "which argv
  does this entry stand for"; `settle`'s `body_class` keys `(cfg_node, member)`; the in-loop render
  floor becomes route-aware at both seats through `floored_in_loop`.
- `618f8217` — the four `loop30-*` cells promoted off XFAIL, scope-verified bless.
- `581b6aba` — `fnd-member-binding-was-unsound-under-rebinding` closed, plus a fifth cell.
- `cfe703c5` — narration trimmed back inside the inline budget.

### §licence.1 — `fnd-verdict-lane-subjects-gate-the-member-ship` (+SURE, measured)

The one non-obvious blocker, and it fails SILENTLY: `cli`'s `ship_auto` closure refuses unless
`verdict_lane[node].subjects()` EQUALS the subject slice its caller passes, and
`push_inline_checks` passed `&[fact]` — one. Under member closure the node's measurement covers the
whole ordered member population, so every member's ship answered `None`, `push_inline_checks` took
its all-or-none refusal, and the call shipped ZERO records while every gate above it looked
ordinary. The repair is `node_subjects(node)`: the ordered establish population of one lowered body
node, read off the site vector itself. Outside a loop that is byte-identically today's `&[fact]`;
inside one it is `push_member_checks`'s own shape (r21 passes the whole `members` slice for every
member), so the two aggregate lanes now agree rather than resembling each other.

### §licence.2 — `tbl-out-param-collision-answers` (edit 1's owed answer, per out-param)

`command_effect`'s real out-params are NODE-keyed and N members share one node. Answered, not
assumed:

| out-param | keyed by | answer |
|---|---|---|
| `backings` | FACT | Benign and order-independent: distinct member facts are distinct keys; a repeated fact re-merges idempotently for one provider, folds `family` to the safe floor `None` across providers, and UNIONs observe-selectors. The REAL map is passed here, unlike `member_family`'s throwaway, because a region fact is no longer render-floored — it reaches the survival tier, and an absent backing would drop to the singleton floor for nothing. |
| `measured` (`verdict_lane`) | NODE | AGGREGATED, never overwritten: one `Measurement` over the ordered member facts, minted only when EVERY member's verdict body measured its own — `MemberFamily`'s gate one shape up. The ship seat compares against exactly that vector (§licence.1). |
| `degrade` (`degrades`) | NODE | SUPPRESSED. `get_or_insert` is first-member-wins, which would report one member's unresolvability as the SITE's (`271:rul-sin-ordering`). A member that degrades and still resolves is not an unresolvable site; one that degrades and does not collapses the population, after which the single-argv path records the reason with its real span. |
| `cmdsub_tops` | appended, `site`-tagged | SUPPRESSED with `site: None`, r21's reason exactly: the collapse discloses the ⊤ once, at the real operand span, and a per-member emit would double it. |
| `diags` | appended | SHARED, and may repeat. A member's verb is argparse-derived and CAN differ per member (`f() { apt-get "$1" -y nginx; }` under `for v in install purge`), so a per-member kind-disagreement is not redundant by construction. Identical members duplicate one sentence — r21's standing behaviour, not a new one. |
| `node`, `live_defs` | inputs | Facts about WHERE the site is; identical for every member by construction. |

### §licence.3 — `fnd-member-binding-was-unsound-under-rebinding` (+SURE, falsified then closed)

The lane's most important finding, and a live cardinal-sin hole the moment anything consumed the
member argv. `Prep::member_inline_pass` answers a site by OVERRIDING the iteration variable in that
site's incoming state. Measured, before the fix:

```sh
install_pkg() { apt-get install -y "$pkg"; }
for pkg in nginx curl; do pkg=wombat; install_pkg; done
```

resolved to `nginx` and `curl`. The shell installs `wombat` twice. The probe would have measured two
cells the command never touches, both come back converged, and the region replaces a live mutator —
under-execution. r21 carries the corresponding rule already (`eligible_members`'
`body_reassigns_var`); the member-inline pass shipped without it.

Closed by `loop_extent_rebinds`, driven over the CFG rather than the loop's AST subtree — which is
the load-bearing half: a SPLICED funcdef body is marked in-loop while its span lives in the
definition, so a subtree walk sees a call and nothing inside it. The `encloses` chain walk reaches a
nested loop's nodes too. Both shapes are pinned (`a_loop_body_rebinding_…`,
`a_called_body_rebinding_…`, both verified red before the fix), and
`loop30-rebound-member-runs-the-shared-region` observes it at the product surface: the plan runs
`apt-get install -y wombat` twice, with both sites disclosed `unresolvable-no-probe`.

### §licence.4 — `fnd-self-reach-is-cell-blind-across-the-back-edge` (+SURE, measured)

`loop30-direct-and-called-mutators-share-a-loop-body` does NOT green as authored, and the reason is
another lane's gate. The r21 Members licence demands `self_reached`, which is `Reach::is_pristine` —
"no write-or-unknown reached me AT ALL", cell-BLIND. The region's own per-member establishes reach
the direct mutator back over the loop's edge, so the four cells being disjoint (`lib*` versus bare)
buys nothing: the direct mutator RUNS, its running establishes wall the region, and the region takes
the GUARD tier rather than Replace. That is correct and safe, and it is still a demonstration — the
two record populations stay disjoint (`site 0.0/0.1` the r21 member index, `site 1.0/1.1` the
member-major inline one), and without the route-aware floor the region would simply run. The case's
header and run-set are re-authored to the measured truth.

Widening `is_pristine` to a per-cell answer is winner-shifting licensure surface in a lane this
brief put a hard stop on, so it is FLAGGED, not touched (`tc-self-reach-cell-blind-widening`).

### §licence.5 — deviations, each OPEN for the conductor to re-derive

- `dev-ship-subjects-widened-beyond-the-brief` — edit 2 as briefed was "ship per member off
  `spliced_member_argv`"; it also had to widen the subject slice (§licence.1) or nothing shipped.
- `dev-vouch-lift-reads-the-member-argv` — `build_vouches_from_sets` resolved
  `value.argv_values(node)` per candidate, which is ⊤ at every operand the call varies, so no member
  ever got a vouch. Each candidate now CARRIES the argv it is a candidate for. Deliberately NOT
  r21's `member_specialization` entity-substitution: reconstructing the operand from the fact the
  oracle declared is a re-derivation, and the value plane already holds the real answer.
- `dev-member-rebinding-guard-added` — §licence.3. Outside the three edits; a wrong-elision.
- `dev-real-backings-map-threaded` — `spliced_members` passes the production backing map where
  `member_family` passes a throwaway; reason in the table.
- `dev-converged-mocks-were-diverged` — three of the four cells shipped a copy-pasted `aptcheck`
  exiting 1 for `curl` while their authored `probe-results.txt` declared every member converged.
  Invisible while XFAIL (gate 1 never ran). Two fixed; `loop30-repeated-member-…` keeps its dead arm
  because its book never names `curl`.
- `dev-direct-and-called-case-re-authored` — §licence.4.
- `dev-fifth-cell-minted` — `loop30-rebound-member-runs-the-shared-region`, minted for §licence.3.
- `dev-head-expected-ran-pins-are-moot` — the brief asks for the two-sided HEAD pins the checkpoint
  could not mint. Read at the seat: `head_ran_drifted` is consulted ONLY inside the XFAIL lens
  (`e2e.rs`, the `if let Some(reason) = xfail_reason` arm), so the pin exists to catch a
  disaster-shaped run-set change hiding under a marker. All five cells are PROMOTED and carry no
  marker, so their run-sets are asserted by `expected.ran` directly — the stronger pin the head-pin
  approximates. Nothing to mint; the mechanism does not apply to a green case.

### §licence.6 — names argued (the overload/gloss half of the naming rider)

- **`universally_quantified_member`** (`DecideSite`) — the checkpoint's lean, taken. It names WHY
  the floor lifts (`30L:rul-shared-region-needs-universal-must`) rather than which loop; `iteration`
  and `in_region` both read as the latter. `member` alone would squat the r21 member index, a
  different numbering (`dec-two-numberings-are-named-apart`, above).
- **`floored_in_loop`** — a predicate named for the FLOOR it answers for, so the two callers cannot
  drift into lifting it at one seat and not the other.
- **`SplicedMembers` / `spliced_members`** — deliberately parallel to `MemberFamily` /
  `member_family` and deliberately NOT merged with it: two syntactic shapes, one discipline
  (`dec-member-family-stays-a-separate-mechanism`, above). "Spliced" is this corpus's existing word
  for the venue.
- **`node_subjects`** — "subjects" is `Measurement::subjects()`'s own word, reused rather than
  re-minted; a measurement-plane term carrying no licence connotation.
- **`member_argv`** (`plan`) — a CROSS-CRATE near-collision with `ValueFlow::member_argv`, and
  flagged as such: this one answers "which argv does this `InlineSite` entry stand for" and reads
  `spliced_member_argv`, never r21's map. If the conductor wants the collision gone,
  `argv_of_inline_site` is the two-to-three-word cousin; the short name stands because the parameter
  type (`&InlineSite`) disambiguates at every call.
- **`loop_extent_rebinds` / `encloses` / `subtree_writes_var`** — "extent" rather than "body",
  because the CFG region a loop may execute is exactly what an AST body is not.
- **`MissingExpectedOut` / `round_trip_residue`** — the classifier says what is missing and what is
  left over; neither borrows a decision-plane word.

### §licence.7 — types, and what they make unrepresentable PRODUCT-WIDE

Two fields, one shape: `InlineSite.member: Option<u32>` and
`DecideSite.universally_quantified_member: Option<u32>`.

NOW UNREPRESENTABLE, product-wide: a region route answering from another evaluation's facts — the
`(cfg_node, member)` key means a `NotIterated` route finds only a member-less entry and a member
route only its own, so the old node-keyed collapse (N members to whichever came last: an answer that
asked one question and called it universal) cannot be spelled at this seat. And the in-loop render
floor cannot be lifted by a caller's say-so: the lift demands an ORDINAL, which only the census
mints, so a boolean set at the region seat cannot license a route the census gave no member — which
is exactly the hole §1's "still admitted" list named.

STILL ADMITTED, product-wide: two `InlineSite` entries sharing `(node, member)` (uniqueness is an
invariant of the one mint, not of the type — the same residue the route census carries); an
`InlineSite.member` whose ordinal exceeds its loop's list length; a `DecideSite` carrying an ordinal
its class vector never had (the two are joined by convention at one seat, not by construction). None
is reachable today, and each would be a mint-site defect rather than a caller's mistake.

### §licence.8 — `tc-*` flags (raised, NOT resolved)

- **`tc-self-reach-cell-blind-widening`** — §licence.4. Making `EstablishMembers`' self-reach gate
  per-CELL rather than "nothing reached me" would let the r21 lane elide beside a disjoint
  neighbour. Winner-shifting, another lane's seat, and it changes what a licence rests on. The exact
  shape, which `loop30-direct-and-called-…` now holds:
  ```sh
  for pkg in nginx curl; do
     apt-get install -y "lib$pkg"   # runs today: the call's own cells reach it over the back-edge
     install_pkg "$pkg"             # guards today; would replace if the direct mutator elided
  done
  ```
  My lean: worth doing, NOT here — it is a second lane's licence and the conservative answer is
  already correct.
- **`tc-duplicate-member-establishes-refuse`** — RULED at the checkpoint (leave it refusing) and
  built that way; recorded because the promotion measured it end-to-end and it holds:
  `for pkg in nginx nginx` ships two records for one cell and the region runs verbatim.
- **`tc-disagreeing-region-renders-cant-tell`** — an AID-plane nit, not a licence one. A region
  whose routes measured differently discloses `probe: cant-tell`, which is `joined_verdict`'s
  `Unknown` reaching the existing render vocabulary; it conflates "the host could not tell" with
  "the members disagreed". Pre-existing (twin calls do the same), covered by `render-form-unwelded`,
  raised so the prose queue can see it.

### §licence.9 — proposed steering sentences (builder authors none; these are drafts)

For `analysis/CLAUDE.md`, appended to "Law — the dangers":

> **member-population-has-one-enumerating-seat** (`30L` §7) — `cfg::loop_evaluations` is the ONE
> answer to "which members does this loop have": the region census takes its COUNT, the value plane
> its member TEXTS, and a second implementation is a licence surface that can disagree with itself.
> The member binding is VOID wherever the loop's EXTENT rebinds the iteration variable
> (`value::loop_extent_rebinds`) — CFG-driven, because a spliced funcdef body is marked in-loop
> while its span lives in the definition, so an AST-subtree walk sees a call and nothing inside it.
> A member's class comes from that member's OWN cells (`classify_cells`); the node's own cell vector
> is their union and answers `MustRun`, which is the floor a refusal falls to.

For `plan/CLAUDE.md`, appended to "Direction":

> **the-in-loop-floor-is-route-aware** (`30L` §7) — the in-loop render floor (`floored_in_loop`)
> still binds every per-SITE leaf decision; it lifts ONLY for a route carrying a member ORDINAL from
> a closed region population, because the edit then lands once at the authored definition,
> universally quantified over every member — exactly the expressibility the floor's own reason names.
> The ordinal IS the fence: a route the census could not give one keeps the floor. Per-member facts
> are OVERLAYS, never clones — one lowered `cfg_node`, N route instances, N `InlineSite` entries
> member-major, and `site N.M` sub-indices that only ever append to member 0's non-loop numbering.
> The census counts a population independently of any argv, so a body site that ignores the call's
> operands still contributes one route per member.

### §licence.10 — register edits, drafted

`FORFEITS.md`: `forfeit-loop-populations-open-until-propagation` is now DISCHARGEABLE — the licence
lands and the five `loop30-*` cells are green. Proposed: REMOVE the row. Its residue is not
row-shaped (a row holds an analysis limitation forfeiting a whole CATEGORY of elision), so it
belongs in `30L`'s ledger instead:

- nested calls under a member-closed loop stay unbound (`dev-nested-call-under-a-member-loop-unbound`):
  a call inside a spliced body takes its operands from the enclosing binding, which one pass cannot
  settle, so it gens `Pure`, its own body sites reach no aggregate, and the region meets to Run;
- a loop whose members establish ONE cell at one site refuses its aggregate on the duplicate
  `(site, fact)` pair (ruled);
- a loop extent that rebinds its iteration variable refuses the member binding entirely
  (§licence.3) — conservative over the whole extent, so a body that assigns the variable and never
  reads it refuses too.

`ANALYZER-NEEDS.md`: no new row. §licence.4's cell-blind self-reach is a precision limit inside a
built mechanism, and `tc-self-reach-cell-blind-widening` is where it should be adjudicated.

### §licence.11 — context other lanes must maintain

- **`lane-influence-carriage`** — `InlineSite` and `DecideSite` each gained a field; neither is a
  stable semantic object with a mint, so neither joins the constructor census. The objects that DO
  (`RouteInstance`, the region decision) are untouched by this half.
- **the glob-load lane** (`p-x-glob-load-acquires-members`, whose census trigger names this lane) —
  the member machinery it inherits is `cfg::loop_evaluations` + `ValueFlow::spliced_member_argv` +
  the member-major `InlineSite` vector. Two riders: a glob's members are ORDER-UNKNOWN while this
  lane's flattening is order-BEARING (`site N.M` keys positionally), so a glob population needs its
  own ordering answer before it can reuse the record keying; and `loop_extent_rebinds` is keyed to a
  `for`'s iteration variable, which a glob load has no analogue of.
- **any lane touching `push_inline_checks` or `ship_auto`** — the subject vector a member's ship
  passes is the whole node population, not the member's own fact (§licence.1). A future seat that
  narrows it back to `&[fact]` re-breaks the member lane silently.

### §licence.12 — riders

- Ranges touched: `analysis/src/effect.rs` (a `SplicedMembers` struct + `spliced_members` beside
  `member_family` ~`:830`; `classify_cells` split out of `classify_one_site`; one `node_effects`
  arm; the member-major site-vector arm in `classify_with_why_diags`; one extra
  `resolve_node_effects` product), `analysis/src/value.rs` (three private methods before
  `loop_var_of` ~`:1290`, one guard inside `member_inline_pass`, two tests), `plan/src/lib.rs`
  (`member_argv`, `node_subjects` + its call site in `push_inline_checks`, the
  `build_vouches_from_sets` candidate tuple, `floored_in_loop` + its two callers, one `DecideSite`
  field, one test), `plan/src/settle.rs` (the `body_class` key, the route's member, two `DecideSite`
  fields), `cli/tests/{support,e2e}.rs` (the classifier floor). `cfg.rs` is UNTOUCHED by this half.
- Comment budget: inline `//` = 19 net new (brief: ≤25); `///` = 152, sized to the pins — two new
  types, five new private seats, two new public fields, one new enum variant, and the per-test
  invariant arguments spike style requires.
- Goldens: five cases moved, all `loop30-*`, scope-verified against `git status --porcelain`
  immediately after each scoped bless. Zero existing-golden drift, both blesses.
