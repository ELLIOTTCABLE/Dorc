# 30Qf — `lane-blind-act-retrofit`: the seat map, and what the law reverses

> Tier: builder lane report (Opus, MAP half; branch `ai/r30-lane-blind-act-retrofit`, worktree
> `.claude/worktrees/agent-adad6f4e9a77652b2`, base `60f3955b`). Grades: +SURE / ~SUSPECT /
> -GUESS / --WONDER. Every line number and every "today" claim below is MEASURED at `60f3955b`
> unless it names a commit of this lane. Charters that outrank this note:
> `plans/30P` (`law-no-unsoundness-below-a-blind-act`, `the-load-plane-stays-correct`),
> `notes/30Pd`, `spike/CLAUDE.md`, `crates/{analysis,cli}/CLAUDE.md`.
>
> §retrofit-map is this section — the proposal EXECUTE implements. EXECUTE appends
> §retrofit-execute and does not edit this one.

## §retrofit-map

### the-law-in-one-screen

`30P:law-no-unsoundness-below-a-blind-act` [HUMAN, typed in substance 2026-08-22]. A **blind act**
is a line whose effect on the shell Dorc cannot see. Below one Dorc's model of the shell is ⊤ and
Dorc claims NOTHING: no cwd-dependent decision, no authority from a definition loaded there, no
load re-pointed or pasted, no elision, nothing shipped on a guess, no engine-side recovery.
Guards survive (`30P:rul-guard-resolves-like-its-mutation`).

The load lanes were built against the conductor's earlier re-cut — `30Q` §3 D2, "cwd-⊤ never
costs acquisition or mirroring, only binding authority". The law REVERSES the shipping half of
that re-cut and adds two gates the code has never had. Three consequences, each measured:

1. **The rewrite gate reads the wrong axis.** `cli::artifact` re-points, inlines and pastes on
   `BookLoad::explicit` ALONE. Explicitness asks whether the AUTHOR named the target; EXACT-ness
   asks whether the CONTROLLER can say which file the line loads. A literal `. ./x.dorc.sh` below
   a blind act passes the first and fails the second, and is rewritten today.
2. **The decidable set's `[ -f <loadable> ]` entry reads no cwd state at all**, so below a `cd`
   or a blind `.` it decides TRUE from controller state, masks an arm dead, and makes a
   conditional definition unconditional. `analysis/CLAUDE.md the-fold-decides-conditions-never-shapes`
   already SAYS the gate is required ("ONLY where the cwd at that line is determinate … until it
   lands, a `[ -f ]` below any such act must not decide"); the code does not have it.
3. **The cwd-clobber seed is keyed on the wrong question** — a finding this lane did not expect
   and the one that matters most. See `fnd-the-seed-asks-about-the-operand-not-the-target`.

### fnd-measured-today — what the code actually does

- **`fnd-the-seed-asks-about-the-operand-not-the-target`** (+SURE, measured). `funcenv::load_sites`
  seeds `cwd_clobbers` from `head.exact.is_err()` — an operand the controller could not EVALUATE —
  plus every `cd`. The law's own example, `. /etc/os-release`, evaluates perfectly: `key_of`
  answers `Resolves` for any path `Cwd::resolve_dot` can name, loaded or not, so the site is
  `Ok(ResolvedHead)`, seeds NOTHING, and a relative `.` below it still resolves and still BINDS.
  The same hole swallows an acquired plain-sh INCLUSION, which `30Qc` item 1's
  `rul-included-is-as-opaque-as-unresolvable` says is exactly as opaque as an unresolvable load
  and which `30Qc:load_sites`' own doc comment claims "arrives through that same door with no edit
  here" — it does not, because the door is operand-evaluability and an inclusion's operand
  evaluates fine. The seed the law wants is the site being UNRESOLVABLE: `Err` head, OR a head
  whose key names no `LoadProgram` (an unread target, an `Included` one, a host path).
- **`fnd-the-file-test-reads-no-cwd`** (+SURE). `funcenv::file_test` takes `(defs, literals, node,
  closer)` and answers `LoadableExists` from `defs.program_of_path_operand(path)` alone;
  `decide` then answers `true` unconditionally for that variant. No cwd state is consulted at any
  seat on the path `dead_edges → decide → decidable_test → file_test`.
- **`fnd-every-cd-clobbers-not-only-a-dynamic-one`** (+SURE). The steering describes the cwd domain
  as "a `cd` whose operand is not `/…`/`./…`/`../…`". The BUILT seed takes EVERY `cd`, which is
  strictly more withholding and strictly simpler. Reusing it for the `[ -f ]` gate therefore
  over-withholds in exactly one cell — an ABSOLUTE `[ -f /abs/x ]` below a `cd` of a known shape —
  and under-withholds in none. Taken deliberately: one predicate, one seat, no operand-shape
  reasoning at the fold.
- **`fnd-a-cwd-havocd-site-sits-in-both-maps`** (+SURE, and it is load-bearing for every gate
  below). A relative head below a clobber lands in `sites.resolved` (so `settled_account` records
  its occurrence and the artifact still mirrors it) AND in `sites.unresolvable` AND in
  `havoc_causes` under `HavocCause::CwdUnknown { clobbered_at }`. A consumer that reads
  `resolved_loads` alone calls it exact. That is why the exactness answer must be ONE composed
  accessor rather than a map lookup each seat performs.
- **`fnd-an-unspliceable-call-is-a-blind-act-nothing-models`** (+SURE by read). The law names three
  blind acts. `eval` is closed by refusal, not by modelling — the parser mints `Unsupported` at
  Error severity and `main.rs` folds any parse/CFG Error into a whole-run exit 10, so no book
  carrying one reaches planning. An UNSPLICEABLE CALL is not closed at all: an over-budget,
  over-depth or recursive call stays an ordinary `CfgNodeKind::Command` (`cfg.rs` inline_budget's
  refusal arms), `command_transfer` matches only `.`/`source`/`unset` and returns the incoming
  environment unchanged, so a call into a body Dorc cannot see leaves every binding intact and the
  cwd determinate. Reported, not widened — `tc-blind-act-includes-an-unspliceable-call`.
- **`fnd-the-fold-reaches-the-world-through-bindings-not-edges`** (+SURE). `FuncEnv::folded_edges()`
  has NO production consumer (four call sites, all tests). The `[ -f ]` decision reaches the world
  by changing the SOLVED ENVIRONMENT the mask re-solves under, so its consumers are every reader of
  that environment. That list is §(b).

### (a) — every seat that rewrites, ships, or acquires a non-EXACT load

`cli/src/artifact.rs`, all five reads of `BookLoad::explicit`, at `60f3955b`:

| seat | line | reads | does |
|---|---|---|---|
| `bundle_files` | `:580` | `load.explicit` | `false` ⇒ mirror every file at its authored path; `true` ⇒ fall through to the bundle |
| `bundle_files` | `:610` | (the fall-through) | mints `ImportEdit::Repoint` — the re-point |
| `inline_imports` | `:709` | `load.absorbable && load.explicit` | mints `ImportEdit::Inline` — the paste |
| `kept_in_place_reason` | `:734` | `load.explicit` | picks `PlacementReason::KeptInPlaceOperandNotExplicit` |
| `select_for_terminal_render` | `:901` | `!(absorbable && explicit)` | the inline-debt count a fallback reports |
| `select` | `:942` | `!(absorbable && explicit)` | the same count on the refusal path |

Plus the two carriage seats, which read no permission at all today and ship unconditionally:
`bundle_files`'s `place(…)` calls (`:571`, `:584`, `:600`, `:623`) and `mirrored_files` (`:645-688`).
`placements()` (`:764-804`) records `PlacedSources::carried`/`uncarried` from `Carriage` alone.

`BookLoad` is built at `book_loads` (`:390-440`) from `projection.occurrences()`, whose `at` field
IS a `CfgNodeId`, and both drivers that call it (`main.rs:2712` `select_artifact_form`,
`world.rs:84` `select_terminal_form`) already hold the `FuncEnv`. So the analysis's own answer
threads in with no new plumbing and no second derivation.

**THE ONE SEAT each gate reads exactness from** — `analysis::funcenv`, composing the two maps
`fnd-a-cwd-havocd-site-sits-in-both-maps` names:

```rust
/// May authority rest on this site's load — and if not, why (`30P:rul-load-head-is-exact-or-havoc`)?
///
/// Composed HERE because a cwd-havoc'd site sits in `resolved_loads` (the file is still named) AND
/// in `havoc_causes`; a consumer reading only the first would call it exact.
pub fn load_certainty(&self, node: CfgNodeId) -> Result<&ResolvedHead, HavocCause>
```

and on the cli side ONE value carrying both axes, because two bare bools side by side are
swappable without a type error (`Carriage`'s own argument, `artifact.rs:743-757`):

```rust
/// What Dorc may do to one `.` line. The two axes are NOT interchangeable: EXPLICITNESS asks
/// whether the AUTHOR named the target (so a rewrite is Dorc's to make —
/// `30P:rul-rewrite-permission-is-derived`); EXACTNESS asks whether the CONTROLLER can say which
/// file the line loads (so anything may rest on it at all — `rul-load-head-is-exact-or-havoc`).
pub struct LoadPermission { explicit: bool, exact: bool }
impl LoadPermission {
    /// Re-point, inline, hoist, paste: the author named it AND Dorc knows what it names.
    pub const fn may_rewrite(self) -> bool { self.explicit && self.exact }
    /// Mirror or bundle its bytes: a copy of a file Dorc cannot prove the author referenced is
    /// engine selection (`law-no-unsoundness-below-a-blind-act`, the nothing-shipped clause).
    pub const fn may_ship(self) -> bool { self.exact }
}
```

Three cells, and the third is the one the law adds:

```sh
. ./lib/foo.dorc.sh               # EXACT ∧ explicit ⇒ may re-point or paste; ships as a bundle
. "$(dirname "$0")/foo.dorc.sh"   # EXACT ∧ inexplicit ⇒ verbatim; ships MIRRORED at the authored path
. /etc/os-release                 # a blind act …
. ./lib/foo.dorc.sh               # … so ¬EXACT ⇒ verbatim; ships NOTHING
```

`BookLoad::explicit` becomes `BookLoad::permits: LoadPermission`; every row of the table above
reads `may_rewrite()`, and the two carriage seats read `may_ship()`.

### (b) — the decidable set's `[ -f ]` consumers, and the monotone gate

`file_test` is reached only from `decidable_test ← decide ← dead_edges ← funcenv::analyze`'s fold
closure. Its consumers are therefore every reader of the environment the mask re-solves — the
edge set itself reaches nobody (`fnd-the-fold-reaches-the-world-through-bindings-not-edges`):

- `FuncEnv::binding_before` / `LiveDefinitions::definition_before` — every SITE-KEYED consuming act
  (verdict, predict-at-site, probe-ship, vouch, guard eligibility), `visibility-is-full-positional`.
- `funcenv::unprovable` — which families the driver withholds silently.
- `funcenv::never_live` → `dorc_oracle::build_dialect`'s whole-unit minting scan → the sparing
  dialect. Exact, not conservative, and a bigger dialect SPARES MORE (`28Q` §9).
- `funcenv::contests` → the cli's contested-family withdrawal.
- `settled_account` — which loads were TAKEN, hence `wanted` (acquisition), hence the occurrences
  the artifact places. A wrong TRUE here changes which files are on disk beside the plan.

**The gate.** `LoadSites` retains the per-node clobber map `cwd_clobbers` already computes (it is
discarded today), `dead_edges`/`decide`/`decidable_test` thread `&LoadSites`, and `file_test`
returns `None` when `sites.clobbers.contains_key(&node)`.

**Why it cannot break monotonicity.** Three separate obligations, none touched:

1. The TRANSFER's monotonicity is what `solve` requires, and this changes no transfer.
2. The fold's TERMINATION rests on the masked-edge set only growing (`FOLD_ROUNDS_CAP`'s doc).
   The gate is a pure function of the CFG and the pre-pass load answers — `load_sites` runs ONCE,
   before the first solve, and is independent of `states` — so it is round-INVARIANT. A condition
   the gate refuses in round 1 it refuses in every round; the set of decidable conditions only
   shrinks relative to today, and the masked set still only grows within a run.
3. `28M:dec-pessimistic-iteration`'s soundness argument ("a decided condition never flips") is
   preserved for the same reason: the gate never turns a refusal into a decision.

Direction: strictly withholding. Fewer decisions ⇒ fewer masked edges ⇒ more live paths ⇒ more ⊤
joins ⇒ fewer licences. `funcenv_floor`'s `folded_edges = ∅` rider is untouched.

### (c) — "blind act", in funcenv terms

| the law's species | seeds `cwd_clobbers` today | after E3 | note |
|---|---|---|---|
| a `.` whose operand the controller cannot evaluate | YES (`head.exact.is_err()`) | yes | the only species built |
| a `.` whose operand evaluates but names nothing the controller HOLDS (`. /etc/os-release`) | **NO** | yes | `fnd-the-seed-asks-about-the-operand-not-the-target`; one line |
| a `.` of an acquired plain-sh INCLUSION | **NO** | yes | same line; `30Qc:rul-included-is-as-opaque-as-unresolvable` finally true |
| any `cd` | YES (every one) | yes | coarser than the steering's shape-conditional wording, safely |
| a spliced body carrying either | YES | yes | spliced nodes are ordinary CFG nodes on the caller's route; a DETACHED body is never reached from `cfg.entry()` and the walk skips it |
| `eval` of ⊤ | n/a | n/a | closed by REFUSAL — `Unsupported` at Error ⇒ whole-run exit 10 |
| an unspliceable call | **NO** | **NO** | NOT this lane's: `tc-blind-act-includes-an-unspliceable-call` |

The acquisition fixpoint survives the widened seed, and this is the hazard `load_sites`' own doc
comment warns about. In acquisition round 1 a book-sourced dorc-lang dependency is
named-but-unheld, so it SEEDS a clobber; a load below it is then cwd-havoc'd — but it stays in
`sites.named`, `settled_account` still `want`s it, so it is still read, and in round 2 the first
target is held and the transient clobber is gone. Acquisition is grow-only, so the seed shrinking
across rounds can only add resolutions. **MEASURED** (see §measured-spikes): the widened seed moves
exactly one test in the whole 2669-test suite, and that test is this lane's own pin.

### (d) — the red cells, committed at `b89a72fd` before any fix

Each verified red for the RIGHT reason by an interim assertion OUTSIDE its pin closure
(`xfail-pins-ride-one-seat`: a pin's setup must not panic inside), which states today's answer in
so many words and must be deleted in the commit that greens the pin.

| cell | pin | seat | CFG shape | greens with |
|---|---|---|---|---|
| `a_load_below_a_blind_act_is_never_re_pointed` | `p-x-non-exact-load-is-never-re-pointed` | `cli/src/artifact.rs` | two straight-line top-level `.`s, each whole-line, redirect- and assignment-free (so the second is inside `floor30-inline-dot-boundary`'s absorbable cell); the described mutator below both. Both forms asserted: multipart mints no import edit, one stream does not flatten | E1 |
| `a_load_below_a_blind_act_ships_no_copy` | `p-x-non-exact-load-ships-no-copy` | `cli/src/artifact.rs` | as above; the multipart set carries no dependency | E2 |
| `a_file_test_below_a_cd_decides_nothing` | `p-x-file-test-refuses-under-unknown-cwd` | `analysis/src/funcenv.rs` | a top-level `cd` of a ⊤ operand, then `[ -f <loadable> ] && <funcdef>` — an `&&` whose left arm IS the decisive command node. Asserts `folds == 0` AND the binding is ⊤ | E3 |
| `a_file_test_below_a_blind_load_decides_nothing` | `p-x-file-test-refuses-below-a-blind-load` | `analysis/src/funcenv.rs` | the same, below a `.` of a ⊤ operand instead. Its own cell: the two reach the gate by different seeds | E3 |
| `a_load_the_controller_does_not_hold_havocs_the_cwd_below_it` | `p-x-an-unheld-literal-load-havocs-the-cwd` | `analysis/src/funcenv.rs` | two straight-line top-level `.`s — a literal ABSOLUTE operand naming a file the unit holds no bytes for, then a literal RELATIVE one naming a file it does; the binding read at the unit's exit | E3 |

FEATURE-ON vs FEATURE-OFF is distinguishable in every one: the artifact pair reads the settled
`Selection`'s own import edits and dependency list (an artifact that ships nothing and one that
ships a bundle are different values), and the funcenv three read the fold's edge count and the
binding it produces (a decided condition and an undecided one are different values). The interim
assertions are the proof that each cell distinguishes them TODAY.

R-e (`p-x-an-unheld-literal-load-havocs-the-cwd`) is an ADDITION to the brief's four, and it is the
one the whole retrofit rests on: without it R-a's and R-b's own shape (`. /etc/os-release` above a
literal load) is not even non-EXACT, so E1 and E2 would gate on a condition nothing reaches.

### (e) — the FORFEITS reds, minted through `internal_tooling::xfail::PINS`

Five minted at `Horizon::Scheduled("r31:kernel-punt-glance")`, a NEW marker. **CONFIRMED**:
`Horizon::round` admits `r<N>:<stage-slug>` (its own `the_horizon_grammar_admits_rounds_and_refuses_dates`
pins the grammar), and `census_report` groups by `horizon.marker()`, so the marker forms its own
group — verified by running `mise run xfail:census` (26 live pins, 1 reserved, the new group
rendered whole). All five live in `analysis/src/funcenv.rs`'s test module, each with its interim
assertion:

| pin | FORFEITS row | cell | today, asserted |
|---|---|---|---|
| `p-x-test-literal-narrows-a-variable` | `forfeit-value-narrowing-by-test` | `an_equality_assertion_narrows_its_variable_below_itself` | `SourceLiteralPlane::variable_text` answers `None` |
| `p-x-assertion-makes-a-dynamic-load-exact` | same row, its load-plane consumer | `an_assertion_makes_a_dynamic_load_head_exact` | the binding at exit is ⊤ |
| `p-x-exact-check-narrows-file-contents` | `forfeit-file-content-facts-from-exact-checks` | `an_exact_content_check_stops_a_load_from_being_blind` | one unresolvable load |
| `p-x-known-write-establishes-sourced-contents` | `forfeit-content-establishment-by-known-write` | `a_known_write_stops_a_later_load_from_being_blind` | one unresolvable load |
| `p-x-dollar-zero-expansion-survives-a-blind-load` | `forfeit-shell-parity-immunity-model` | `a_script_relative_load_survives_a_blind_act_above_it` | the binding at exit is ⊤ |

`p-x-test-literal-narrows-a-variable` is read at `SourceLiteralPlane` deliberately: that is the
window the load plane AND the oracle argparse both consume, so it is the honest observable for
both halves the FORFEITS row names (an exact load head, and a probeable argv).

**`p-x-subshell-contains-a-blind-load` is NOT MINTED — it is already GREEN**, and the conductor is
owed a FORFEITS rewrite. `cli/tests/sh_parity.rs`'s `the_havoc_dies_at_a_paren_and_survives_a_return`
third row asserts exactly it: `hork__is_converged() { … }\n( . "$SITE_PROFILE/rc" )\nhork tune web`
leaves the definition LIVE, because the load lane's D1 made the havoc pointwise in the INNERMOST
frame and `cfg::lower_scoped` makes `EnvStack` frames subshell scopes. `forfeit-shell-parity-immunity-model`'s
REDS row should read `p-x-dollar-zero-expansion-survives-a-blind-load` alone, and its
"( . /vendor/blind.sh ) # contained — today the havoc escapes the paren" example is stale: the
containment LANDED at `30Qc` §execute-a, and the row should name that test as landing evidence.

### (f) — every existing golden or test the EXECUTE will move, with its law clause

MEASURED, not predicted (§measured-spikes): with the widened seed AND the `[ -f ]` cwd gate both
applied, the full suite moves exactly the three cells that pin them and **NOTHING ELSE** — 2666
passed, 3 failed, every one a deliberate XPASS of this lane's own pin. Zero goldens, zero
transcripts, zero run-sets.

| target | law clause | move |
|---|---|---|
| `a_relative_source_below_an_unknown_one_cannot_be_identified` (`cli/src/main.rs`) | `law-no-unsoundness-below-a-blind-act`, nothing-shipped | PROSE ONLY. Its assertions are on `found` (acquisition), `unresolvable_loads().len()`, and `at_exit == Withheld`; all three STAND under E1–E3. Its message "the file is still READ, and still mirrored at its authored relative path — cwd-⊤ costs authority, never the shipped tree" states the `30Q` §3 D2 re-cut the law reverses, and must be re-said: still READ, no longer MIRRORED |
| the three E3 pins' interim assertions | as above | deleted in the commit that greens each pin — that is what the interim assertion is for |
| the two E1/E2 pins' interim assertions | as above | ditto |

Predicted-but-unmeasured, because E2's carriage gate is not built: ~SUSPECT no green ARTIFACT_SET
case moves. `load30-plain-sh-inclusion-ships`'s `. ./helpers.sh` is the FIRST load in its book, so
it stays EXACT and stays mirrored; `load30-two-point-frames` and `load30-rooted-shared-dependency`
chain `.`s whose targets are all HELD in the settled world, so no clobber survives round 2;
`load31-punted-load-shapes`, whose `. ./helpers.sh` sits below two blind acts and WOULD stop being
mirrored, is XFAIL and golden-text-blind. EXECUTE must confirm this by running the suite after E2
and reporting any case whose `unresolved_generated_imports` gate newly fires. **Anything that moves
without a law clause above is a finding, not expected churn.**

### (g) — `tc-*` flags, cross-cutting, for the human

1. **`tc-nothing-shipped-costs-the-common-book-its-plan`** — the veto-eligible one, and the whole
   of E2. Under the law a non-EXACT `.` ships nothing, so the generated plan carries a `.` naming a
   file the generation does not contain, which the atlas measured FATAL
   (`floor30-atlas-dot-missing-file-is-fatal`).

   ```sh
   . /etc/os-release              # a host-owned file, never shipped, BY DESIGN: the blind act
   . ./oracles/docker.dorc.sh     # today: mirrored beside the plan, so the apply runs
                                  # under the law: shipped nowhere, so the apply DIES here
   docker run --name web nginx
   ```

   The trade, stated honestly. If the blind act did NOT `cd` — overwhelmingly the common case, and
   `/etc/os-release` never does — mirroring makes the plan run and costs nothing. If it DID, the
   mirrored copy sits at the generation root, the runtime `.` resolves elsewhere, and the copy is
   never loaded: mirroring is inert rather than wrong. So the law's clause buys no soundness in
   the cwd cell it is aimed at; what it buys is that Dorc never puts bytes on a target under a
   name it cannot prove the author's line will read. My LEAN: build it as the brief rules (the
   human's own lean, and "nothing shipped on a guess" is a posture question, not an engineering
   one), and flag that the observable cost is a whole class of book whose apply stops working
   rather than becoming less optimised — which is the *unusual* posture `rul-floor-is-uneven-across-forms`
   already braces for, but at the APPLY surface rather than the form surface. One human line
   settles it either way, and E2 is a two-seat change in both directions.

2. **`tc-blind-act-includes-an-unspliceable-call`** — the law's third species, modelled nowhere
   (`fnd-an-unspliceable-call-is-a-blind-act-nothing-models`). Not built; reported.

   ```sh
   deploy() { . "$SITE_PROFILE/rc"; cd /srv; }   # over budget, or recursive, or too deep
   deploy                                        # Opaque at the effect tier — a poison WALL …
   . ./oracles/docker.dorc.sh                    # … but the funcenv's cwd and bindings are untouched
   ```

   Where the body IS spliced this is already right (the spliced nodes are on the caller's route).
   Where it is not, the call is a wall for EXECUTION and transparent to the BINDING plane. Closing
   it means `command_transfer` havocing on an unspliced call, which is a licence-review-tier
   widening of the wall in the withholding direction and its own ruling. My LEAN: not this lane —
   it is a different mechanism (call splicing), the corpus has no instance, and the brief's
   exclusions say report rather than widen unruled.

3. **`tc-acquisition-outlives-the-clobber`** — acquisition is GROW-ONLY, so a file read in an early
   round stays in the snapshot even when the settled world says its load is non-EXACT. Its bytes
   then reach two FRAMELESS seats that ask no positional question: `dorc_oracle::build_dialect`'s
   whole-unit minting scan (which selector tokens the unit's authors minted AT ALL — a bigger
   dialect SPARES MORE, `28Q` §9) and `HelperIndex`, where custody is proxied by the loaded-source
   index. Both are authority derived from a load the law says carries none.

   ```sh
   . /etc/os-release                 # blind act
   . ./oracles/docker.dorc.sh        # ¬EXACT: no bindings, no vouch — but the file is READ,
   docker run --name web nginx       #   so its selector tokens are in the sparing dialect
   ```

   Bounded: the dialect only reaches SURVIVAL sparing, which needs `--risk-faultless-skips`; the
   `HelperIndex` half needs a vouch that already withholds. My LEAN: keep acquisition (dropping it
   would stall the fixpoint — §(c) — and `Withheld`-instead-of-`NoOpinion` is the safe direction),
   gate SHIPPING only, and route this to whichever lane next touches `build_dialect`.

4. **`tc-explicitness-seat-unification`** — `fnd-one-explicitness-predicate-two-seats` (`30Q` §5c,
   a standing `30O` debt): `cli::artifact::operand_is_explicit` reads the AST word while
   `funcenv::ResolvedHead::explicitness()` reads the RESOLUTION, and the two answer the same
   question. Unifying them here would be a winner-shifting change (the analysis seat ADMITS a
   literal-assigned book-set root through `literal_text`'s constant folding, where the AST seat
   admits a bare `$name`) folded into a soundness retrofit. My LEAN: NOT this lane. E1 adds the
   EXACT axis beside the existing explicitness seat and leaves the debt where it is ruled.

### names-argued

- **`LoadPermission`** — one value rather than two bools, for `Carriage`'s own stated reason. The
  cost is that `permission` is `30P:rul-rewrite-permission-is-derived`'s word for the EXPLICITNESS
  half alone, and this widens it to cover carriage too. Argued: both halves are things Dorc is or
  is not permitted to do to somebody's line, and the two accessors (`may_rewrite` / `may_ship`)
  keep them apart by name at every call site, which is where the confusion would otherwise land.
  Conductor may overrule; the alternatives all collide (`licence` is the license plane's,
  `disposition` is `plan`'s, `grade` is claim-tier, `authority` names one half).
- **`exact` / `exactness`** — kept as a FIELD name, refused as a TYPE name. `30P` and `30I` both
  use "exact" as a term of art and inventing a synonym would fragment the vocabulary; but an
  `Exactness` type sitting beside `Explicitness` on one struct is two `-ness` words differing in
  the middle, which is a legibility hazard in exactly the place a swap does the most damage. The
  type is `LoadPermission`, the axes are its private fields, and the questions are its methods.
- **`FuncEnv::load_certainty`** — not `load_exactness` (see above), not `is_exact` (it must carry
  the CAUSE — the hint E2 mints needs `HavocCause::CwdUnknown { clobbered_at }`), not `head` (the
  crate already spells `load_head` for the evaluator). "Certainty" is unused in this corpus.
- **NOT used, deliberately**: `placement` (reserved for semantic arrangement), `layout` (reserved
  for textual emission), `lift` (the static lift of oracle text), `Grade`, `provenance`.

### types-make-unrepresentable — product-wide

- **`LoadPermission` with private fields** makes unrepresentable, PRODUCT-WIDE: a rewrite seat that
  consults explicitness without exactness (the bool it would read is not public, and the only
  accessor that answers the rewrite question conjoins both); and a carriage seat that reads the
  rewrite answer by mistake (`may_ship` is a different method with a different name). Still ADMITS:
  a NEW seat that reads neither and ships anyway — nothing types the obligation to ask, which is
  why §(a) enumerates the seats and E2's commit must visit all of them.
- **`FuncEnv::load_certainty` as the composed accessor** makes unrepresentable: a consumer that
  reads `resolved_loads` and calls a cwd-havoc'd site exact (`fnd-a-cwd-havocd-site-sits-in-both-maps`
  is the bug in type form, and the composition is where it stops being spellable). Still ADMITS:
  `resolved_loads()` and `havoc_causes()` remaining public — they have other consumers, and
  narrowing them is its own act.
- **`LoadSites::clobbers` as the one cwd-determinacy answer** makes unrepresentable: a second
  derivation of "is the cwd known here". Still ADMITS: a consumer outside `funcenv` — the field is
  private to the module, and the `[ -f ]` gate is the only seat that needed it.

### the-execute-order — one commit per row, each independently green

1. **E3a** — the clobber seed widens to "the site is unresolvable" (one predicate at
   `load_sites`). Greens `p-x-an-unheld-literal-load-havocs-the-cwd`. FIRST, because E1 and E2 gate
   on a condition this creates. MEASURED: no other test moves.
2. **E3b** — `LoadSites` retains `clobbers`; `dead_edges`/`decide`/`decidable_test` thread it;
   `file_test` refuses under a clobber. Greens `p-x-file-test-refuses-under-unknown-cwd` and
   `p-x-file-test-refuses-below-a-blind-load`. MEASURED: no other test moves.
3. **E1** — `FuncEnv::load_certainty`; `book_loads` takes the `FuncEnv`; `BookLoad::explicit`
   becomes `permits: LoadPermission`; the six rewrite seats read `may_rewrite()`. Greens
   `p-x-non-exact-load-is-never-re-pointed`.
4. **E2** — `bundle_files` and `mirrored_files` place nothing for a `!may_ship()` load and record
   `PlacedSources::uncarried` for its sources; the hint is a new `DiagCode` minted `message: None`
   through the loom flow (`mise run loom -- …`), carrying the clobbering line from
   `HavocCause::CwdUnknown { clobbered_at }` — the render `tc-cwd-havoc-costs-relative-acquisition`
   left open at `30Qc` §execute-a's tc-flags. Greens `p-x-non-exact-load-ships-no-copy`.
   ACQUISITION IS KEPT (§(c) and `tc-acquisition-outlives-the-clobber`).
5. **E4** — the goldens §(f) names, re-blessed SCOPED, porcelain-verified, one `(AI test)` commit.
   MEASURED to be empty for E3; owed a re-measurement after E2.
6. **E5** — `§retrofit-execute` appended here.

Two riders EXECUTE must carry. `PlacementReason` has no arm for "the resolution is not EXACT" and
`PlacedSources::uncarried` carries no reason at all, so the WHY a source went uncarried is not
recorded on the plane — the disclosure rides the new `DiagCode` alone. And `kept_in_place_reason`
must not answer `KeptInPlaceOperandNotExplicit` for a line that is explicit and merely non-EXACT:
that would point the author at the wrong repair, which is `271:rul-sin-ordering`'s top.

### measured-spikes — how §(c) and §(f) were answered

Both changes were applied to a scratch working tree, the whole suite run, and the tree reverted;
nothing from either spike is committed. E3a alone: `2668 passed, 1 failed`, the failure being
`a_load_the_controller_does_not_hold_havocs_the_cwd_below_it`'s interim assertion
(`left: Top, right: Elem(Defined(DefId(0)))` — the target behaviour). E3a + E3b together:
`2666 passed, 3 failed`, the three being exactly this lane's three funcenv pins failing on their
interim assertions. No golden, transcript, run-set or loom moved in either.

### proposed-steering-and-register-edits (conductor applies; builders edit no `CLAUDE.md`)

- `analysis/CLAUDE.md rul-havoc-is-pointwise-never-the-stack` — the clause "which costs a later
  relative `.` its AUTHORITY only — acquisition and mirroring are untouched (`30Q` §3 D2)" is
  reversed by `30P:law-no-unsoundness-below-a-blind-act`. Proposed: *"…which costs a later relative
  `.` its AUTHORITY and its CARRIAGE both — nothing is shipped for a non-EXACT load, because a copy
  of a file Dorc cannot prove the author referenced is engine selection; controller-side
  ACQUISITION is kept, because the acquisition fixpoint reads a transient round-1 clobber and a
  withheld name is the safe direction. The clobber seed is the site being UNRESOLVABLE — an
  unevaluable operand, a `cd`, an acquired plain-sh inclusion, or a resolvable operand naming
  nothing the controller holds — never operand-evaluability alone."*
- `analysis/CLAUDE.md rul-exact-is-not-explicit` — "EXACT governs AUTHORITY … EXPLICITNESS governs
  REWRITING" stays true and is now INSUFFICIENT as written, since an emitter reading explicitness
  alone rewrites a non-EXACT line. Proposed rider: *"a rewrite needs BOTH: explicitness says the
  author named the target, exactness says Dorc knows which file that is, and a rewrite of a
  reference whose resolution is unknown changes which file the host loads. `FuncEnv::load_certainty`
  is the one seat the second axis is read from."*
- `analysis/CLAUDE.md the-fold-decides-conditions-never-shapes` — its `[ -f ]` clause's parenthetical
  "(human-acked 2026-08-22; the load lane's cwd model is the gate — until it lands, a `[ -f ]` below
  any such act must not decide)" describes a gate that did not exist. Proposed: *"…and the gate is
  BUILT: `file_test` refuses under `LoadSites`' clobber map, the same pre-pass answer the load heads
  read, so one seat answers 'is the cwd determinate here'. It is round-invariant, so the fold's
  termination and `dec-pessimistic-iteration`'s never-flips argument are untouched."*
- `cli/CLAUDE.md artifact-forms-derive-from-one-structure` — "permission to rewrite a `.` line at
  all is DERIVED from the operand naming its target EXPLICITLY … EXACT is a different axis and
  grants nothing here" must be re-said to EXACT ∧ explicit, and the carriage half added: *"a
  non-EXACT load is not rewritten in any tier AND its target is placed by no form — `BookLoad`
  carries one `LoadPermission` whose two questions (`may_rewrite`, `may_ship`) are what the rewrite
  seats and the carriage seats read."*
- `cli/CLAUDE.md bundle-projection-is-pre-contact-and-not-placement` — its `PlainInclusion` rider
  says such a source "is mirrored BYTE-VERBATIM at its authored relative path"; that now holds only
  where the inclusion's own `.` is EXACT.
- `FORFEITS.md forfeit-shell-parity-immunity-model` — REDS becomes
  `p-x-dollar-zero-expansion-survives-a-blind-load` alone;
  `p-x-subshell-contains-a-blind-load` is LANDING EVIDENCE
  (`cli/tests/sh_parity.rs the_havoc_dies_at_a_paren_and_survives_a_return`, third row) and its
  `( . /vendor/blind.sh ) # contained — today the havoc escapes the paren` example is stale.
- `FORFEITS.md` — the other three rows' REDS are now MINTED and need no edit
  (`forfeit-value-narrowing-by-test` gains both of its named pins;
  `forfeit-file-content-facts-from-exact-checks` and `forfeit-content-establishment-by-known-write`
  gain theirs), which discharges `30P:rul-forfeits-carry-reds` for the `30Pd` §5 set.
- `ANALYZER-NEEDS.md an-cwd-state` — its OWED clause ("book-side cwd tracking … consumers that must
  read it: relative `.` resolution, the funcenv decidable set's `[ -f ]` entry …") becomes BUILT at
  E3b for the second consumer and at E3a for the first; the row's OWED residue is `$0` in its
  slashless spelling (which is `tc-dollar-zero-is-script-anchored`, the human's) and `pushd`/`popd`
  (out of dialect). Proposed: move the row to BUILT with that residue named.
- `ANALYZER-NEEDS.md an-load-exactness-reads-binding-state` — its obligation ("a `cd`, a `PATH`
  assignment, a shadowing definition of the predict's tool, or a havoc `.` above makes it non-EXACT")
  is discharged for `cd` and for the havoc `.`; `PATH=` and the shadowing definition matter only for
  the static-predict tier, which has no stdlib to carry a predict and is `r31:book-load-acceptance`.

### context-other-lanes-must-maintain

- `lane-influence-carriage` — this lane's E1/E2 touch `cli/src/artifact.rs`'s `BookLoad`,
  `book_loads`, `bundle_files`, `mirrored_files`, `inline_imports`, `kept_in_place_reason` and both
  `select` entry points, and `cli/src/main.rs`'s `select_artifact_form` (one parameter). It touches
  `Selection`/`with_plan` and `main.rs`'s driver seat NOWHERE. Line ranges are in §touch-ranges.
- Everyone — `funcenv::analyze` is unchanged in signature; `FuncEnv` gains one accessor;
  `LoadSites` is private to the module. `internal_tooling::xfail::PINS` grew by ten rows and the
  census gained the marker `r31:kernel-punt-glance`.

### touch-ranges — MAP half (`b89a72fd`)

| file | ranges |
|---|---|
| `spike/crates/internal-tooling/src/xfail.rs` | `PINS`' tail: ten new rows appended |
| `spike/crates/analysis/src/funcenv.rs` | the test module's tail: TABLE 8 (three cells) and TABLE 9 (five cells) |
| `spike/crates/cli/src/artifact.rs` | the test module's tail: `below_a_blind_act` and its two cells |

Comment budget for the MAP half, counted with the brief's own commands against `ai/r30-conduct`:
inline `//` **5** net new (two section banners plus three), `///` **91**.

### deviations — OPEN for the conductor

- **`dev-a-fifth-red-cell-was-necessary`** — the brief names four red cells; five are committed.
  `p-x-an-unheld-literal-load-havocs-the-cwd` is the addition, and without it R-a and R-b assert
  over a book that is not actually non-EXACT today, so both would be red for a reason the fix does
  not address. Reasoning offered; conductor re-derives.
- **`dev-interim-assertions-ride-the-pin-cells`** — `xfail-pins-ride-one-seat` says interim behaviour
  belongs in "a separate test whose name says interim". Every pin here instead carries its interim
  assertion OUTSIDE its closure, in the same `#[test]`, labelled `interim:` in its message. Reason:
  the rule's stated hazard is a pin whose SETUP panics inside the closure (so the pin asserts
  nothing), which this shape avoids, and a separate test would duplicate the whole world-building
  for a one-line claim while letting the pair drift. It also makes "red for the right reason"
  mechanically checked forever rather than checked once by the builder. Cost: the greening commit
  must delete the interim assertion, which is a forcing function rather than a debt.
- **`dev-the-map-half-measured-two-execute-changes`** — E3a and E3b were applied to a scratch tree
  and the suite run, to answer §(f) with a measurement instead of a prediction; the tree was
  reverted and nothing from either spike is committed. This is more than a MAP half usually does;
  it is what turned "~SUSPECT nothing moves" into "MEASURED nothing moves", which is the fact the
  conductor's ruling on `tc-nothing-shipped-costs-the-common-book-its-plan` most needs.
- **`dev-e2s-carriage-gate-is-unmeasured`** — the same treatment was NOT given to E2, because the
  carriage gate is a real change to two functions rather than a predicate, and building it would be
  EXECUTE. §(f)'s second table is therefore ~SUSPECT rather than measured.
