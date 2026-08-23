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

## §retrofit-execute

> Tier: builder lane report (Opus, EXECUTE half; same branch and worktree, base `60f3955b`).
> §retrofit-map is not edited; this appends what was BUILT against the conductor's rulings.
> Every "measured" below is the whole 2672-test suite unless it names a filter.

### what-landed — eight commits, each independently green

| commit | row | what |
|---|---|---|
| `4990f276` | E1 | `FuncEnv::load_certainty`; `BookLoad::permits: LoadPermission`; six rewrite seats read `may_rewrite()` |
| `fdbbe6ea` | E2a | the `load-carriage-withheld-under-unknown-cwd` code, `message: None`, and its defining case |
| `34382894` | E2b | the carriage gate at both placement seats, the lock, and the whole-product case that observes it |
| `bbafc1cf` | E3a | the clobber seed asks whether the controller HOLDS the target; three pins promoted |
| `ce51d88e` | E3b | the `[ -f ]` entry refuses under an unknown cwd; two pins promoted |
| `33b6e477` | E3c | the red cell for the third blind act, committed before the seed |
| `e36b75ab` | E4 | the one prose re-say `§retrofit-map` (f) enumerated |
| `554b7992` | — | the clippy arity allowance the cwd input forced on `decide` |

FIVE pins promoted and removed from `internal_tooling::xfail::PINS`:
`p-x-non-exact-load-is-never-re-pointed` · `p-x-non-exact-load-ships-no-copy` ·
`p-x-file-test-refuses-under-unknown-cwd` · `p-x-file-test-refuses-below-a-blind-load` ·
`p-x-an-unheld-literal-load-havocs-the-cwd`. ONE minted: `p-x-an-unspliceable-call-havocs-the-cwd`
(`end-of-r30`). The five `r31:kernel-punt-glance` pins are untouched and red. Census at the tip:
22 live, 1 reserved, no expired group.

### E3C IS HALTED — it moves two goldens beyond prose

The seed was built exactly as ruled — `cfg::call_body_sites(node)` answering `None` at a command
word the unit DEFINES — measured, and REVERTED. The red cell stays committed (`33b6e477`) and the
pin stays live. What it moved, enumerated:

1. `cli/tests/definition_frames.rs the_engine_names_the_definition_the_shells_ran`, over
   `floor30-blessed-override-above-and-below.loom`: *"the shells ran `./override.sh`'s body here
   (it emitted `override`), but the environment names None"* — `left: None`,
   `right: Some(SourceFileId(1))`.
2. `cli/tests/load30-two-point-frames` — `ap-2-exec: apply ran the wrong commands or wrong order`.

**Why, and it is a finding rather than a tuning.** `call_body_sites` answers `Some` only where a
body was SPLICED, and the splicer inlines SAME-FILE funcdefs only — so `None` covers two
populations the ruling does not distinguish: a call Dorc could not see into, and a call into a
body it holds and models but did not clone onto this route. A book that sources `base.sh` and then
calls the `hork__is_converged` that file defines hits the second, and under the ruled seed every
such call would clobber the working directory from its own line down — so every relative `.`
below any oracle-helper call in any book would lose its authority and its carriage.

A call into a body the controller HOLDS AND MODELS is not "a body Dorc cannot see". The law's third
species is the CFG's own REFUSAL set (over-budget · recursive · out of the splice slice), which
`cfg.rs` records today as a `CFG_INLINE_REFUSED` diagnostic and NOT as queryable state. So the seat
the ruling wants does not exist yet: `Cfg` must expose the refused-call set (a node set beside
`call_body_sites`, populated at the same four refusal arms), and only then can the seed take it.

+SURE the correction is a widening of the CFG's own bookkeeping and not a re-reading of the law.
~SUSPECT it is small — four arms, one set, one accessor — but it is a `cfg.rs` change in a lane
whose charter is `funcenv`/`cli`, and `splice-budgets-are-licensure-not-perf` puts anything near
those arms on the same winner-shifting review footing. NOT taken; conductor's.

Note the direction the reverted version failed in: it WITHHELD more, so nothing unsound shipped in
the interim — the two reds are lost licences, which is why they surfaced as an apply running MORE
commands rather than fewer.

### deviations — every one OPEN

1. **`dev-e1-and-e2-are-observable-only-through-the-dynamic-spelling`** — the conductor ordered
   E1 → E2 → E3a, and E1/E2's own pins assert over a LITERAL blind act, which is not non-EXACT
   until E3a. Rather than reorder, both cells gained a DIRECT assertion over the DYNAMIC spelling
   (a `.` of a ⊤ operand, already a clobber before this lane), so E1 and E2 are each independently
   observable in their own commit and the pins discharge at E3a. The two spellings ride one
   parameterised world helper and both are asserted at the promoted cells, with a null-command
   control proving the refusals are the act's doing.
2. **`dev-load-certainty-needed-the-named-map-too`** — `§retrofit-map` had `load_certainty` read
   `havoc_causes` then `resolved_loads`. That is not total: an acquired plain-sh INCLUSION resolves
   its head but files no `LoadProgram`, so it sits in `sites.named` and would have answered
   `Err(NotInSnapshot)` — non-EXACT, so its bytes would have stopped being mirrored and
   `mech-acquire-and-ship-plain-sh` would have silently regressed. `FuncEnv` gained `named_loads`
   and the accessor falls through to it: EXACT-ness asks whether the head names ONE file, which
   both maps answer, and carrying a program is the separate question
   (`rul-acquiring-bytes-is-not-modelling-them`). Conductor re-derives.
3. **`dev-the-new-code-has-an-honest-firing-route`** — the brief asked only for the code. It also
   gained a whole-product case, `load30-blind-act-withholds-its-carriage`, because
   `an-artifact-set-runs-from-its-own-generation`'s general law binds it: a case minted to
   demonstrate a capability must OBSERVE it, and the carriage gate had no end-to-end witness. The
   case moves the working directory to a ⊤ operand above a literal relative `.` of a package the
   controller holds; its blessed artifact is the book VERBATIM — no re-point, no paste, no bundle —
   and its `expected-diagnostics` asserts the code fires. Blessed SCOPED (`mise run bless:case`),
   porcelain-verified: only that case's own files, plus the two generated locks.
4. **`dev-no-artifact-set-on-that-case`** — it deliberately does NOT declare `ARTIFACT_SET`. With
   the carriage gate on, a published generation would carry no dependency, so
   `unresolved_generated_imports` would fire — which is
   `tc-nothing-shipped-costs-the-common-book-its-plan` arriving as a harness failure rather than a
   product decision. The harness is RIGHT and the case is not the place to adjudicate it.
5. **`dev-clippy-forced-an-arity-allowance`** — threading the cwd answer put `decide` at 8/7
   arguments. `#[expect(clippy::too_many_arguments)]` with the reason `transfer`'s own allowance
   already gives, one seat over. `check-quiet` does not carry clippy (it is a builder-completion
   check, `four-rung-gate-ladder`), so this surfaced at the Windows completion leg, not at commit.
6. **`dev-e4-was-prose-only-as-measured`** — E4 moved exactly what (f) predicted and nothing else:
   `a_relative_source_below_an_unknown_one_cannot_be_identified`'s message, whose "still mirrored
   at its authored relative path — cwd-⊤ costs authority, never the shipped tree" states the
   `30Q` §3 D2 re-cut the law reverses. Its three assertions are unmoved. No golden was re-blessed
   in E4 at all.

### findings

- **`fnd-the-splice-refusal-set-is-not-queryable`** (+SURE, measured) — the E3C halt's root cause,
  above. `cfg.rs`'s four refusal arms mint a diagnostic and record nothing, so "Dorc could not see
  into this call" is not a question any consumer can ask. Every OTHER consumer of that fact today
  reads it as `Opaque ⇒ Reach::Top` at the EFFECT plane, which is why the gap survived: the effect
  plane walls correctly and the BINDING plane never had to ask.
- **`fnd-a-loaded-body-is-never-spliced`** (+SURE, measured) — the splicer inlines same-file
  funcdefs only, so `call_body_sites` answers `None` for every call into an oracle's body. Any
  future consumer reading that absence as "unmodelled" inherits the E3C bug.
- **`fnd-acquisition-is-what-keeps-the-fixpoint-growing`** (+SURE, measured across E3a) — the
  widened seed makes a book-sourced dependency clobber in acquisition round 1, when nothing is held
  yet. Nothing breaks because the site stays in `sites.named`, so `settled_account` still WANTS it,
  it is still read, and round 2 clears the clobber. Had E2 also filtered `wanted`, the fixpoint
  would have stalled and the dependency would never have been acquired at all — the concrete reason
  acquisition is kept (`tc-acquisition-outlives-the-clobber`, ruled).

### tc-flags

- **`tc-nothing-shipped-costs-the-common-book-its-plan`** — unchanged from `§retrofit-map`, now with
  one measurement behind it: an `ARTIFACT_SET` case over this shape would fail
  `unresolved_generated_imports`, because the published plan's own literal relative import names a
  file the generation no longer contains. That gate is the harness saying, mechanically, what the
  flag says in prose. Going to the human as ruled.
- **`tc-the-third-blind-act-wants-a-cfg-widening`** — NEW, and it is the E3C halt in flag form: the
  seat the law's third species needs is a refused-call set on `Cfg`, which is `cfg.rs` bookkeeping
  in a lane chartered for `funcenv`/`cli`, on `splice-budgets-are-licensure-not-perf`'s
  winner-shifting footing. Lean: build it, narrowly, in whatever lane next touches `cfg.rs`'s
  inline arms — the pin and its red cell are already committed and will green from that one seat.

### touch-ranges — for the fold

The conduct branch advanced while this lane ran; the lane is based on `60f3955b` and needs a
rebase before it folds.

| file | what moved (at `554b7992`) |
|---|---|
| `analysis/src/funcenv.rs` | `named_loads` field + its three constructors · `load_certainty` (beside `folded_edges`) · `LoadSites::clobbers` · `load_sites`' seed · `dead_edges`/`decide`/`decidable_test`/`file_test` signatures + the gate · the test module's TABLE 8/9 |
| `cli/src/artifact.rs` | `BookLoad::permits` + `LoadPermission` · `book_loads` · `bundle_files`' carriage and rewrite gates · `mirrored_files`' wanted set · `inline_imports` · `kept_in_place_reason` · `placements`' reach test · both `select` entry points' debt counts · the test module's tail |
| `cli/src/main.rs` | `select_artifact_form` (one argument) · `load_head_notices` + `line_of` + its one call site · one message in `acquisition_tests` |
| `cli/src/world.rs` | `select_terminal_form` (one argument) |
| `aid/src/{diag,fixture,catalog_lock}.rs`, `aid/tests/diag_tidy.rs` | the new code's five registration seats |
| `internal-tooling/src/xfail.rs` | six pins added over the lane, five promoted away |

**`Selection`, `with_plan`, and `main.rs`'s driver seat are untouched**, as the influence lane
needs.

### proposed-steering-and-register-edits — DELTA on `§retrofit-map`'s list

Those proposals stand as written, with three corrections the build forced:

- `analysis/CLAUDE.md rul-havoc-is-pointwise-never-the-stack` — the proposed replacement should
  ALSO say the seed is `LoadSites`' one answer and that the `[ -f ]` gate reads it, since that is
  what makes "one seat answers cwd determinacy" true rather than aspirational.
- `analysis/CLAUDE.md the-fold-decides-conditions-never-shapes` — its `[ -f ]` clause's
  parenthetical is now discharged; the replacement drafted in `§retrofit-map` is accurate.
- NEW: `analysis/CLAUDE.md`, a rider on the same bullet or its own — the law names THREE blind acts
  and the engine models TWO. `eval` is closed by refusal; an unspliceable call is NOT
  (`fnd-the-splice-refusal-set-is-not-queryable`, pinned by
  `p-x-an-unspliceable-call-havocs-the-cwd`). Worth saying in steering rather than only in a pin
  trigger, because the tempting wrong fix — reading `call_body_sites`' absence as unmodelled — is
  one line away and reddens two goldens.

## §third-blind-act

> Tier: builder lane report (Opus, `lane-third-blind-act`; branch `ai/r30-lane-third-blind-act`,
> worktree `.claude/worktrees/agent-a2971754683cccf39`, base `c228113d`). Appends only; neither
> section above is edited. E3C's halt is discharged. Every "measured" below is the whole suite on
> both platform legs unless it names a filter.

### what-landed — two commits, each independently green

| commit | what |
|---|---|
| `bacf026d` | `Cfg::splice_refused` — the refusal set, recorded at the arms that already announce the refusal |
| `3ba6d241` | the clobber seed reads it; `p-x-an-unspliceable-call-havocs-the-cwd` promoted |

ONE pin promoted and removed from `internal_tooling::xfail::PINS`. Census at the tip: 21 live,
1 reserved, no expired group.

### the-refusal-arms-are-eight-not-four

`§retrofit-execute` and the brief both say "the same four refusal arms". MEASURED: there are
EIGHT, and all eight are recorded. Seven mint `CFG_INLINE_REFUSED` (`Redefined` · `RecursiveCall` ·
`DepthBudget` · `UnmodeledPositional` · `WriteRedirect` · `PerCallNodeBudget` ·
`PerBookNodeBudget`); the eighth is the depth-2 positional arm, which mints
`Depth2PositionalUnthreaded` — a Note rather than a refusal code, but the SAME act (the body is not
spliced, the call runs verbatim, and nothing on the caller's route models what it did).

Recording is ONE seat, not eight: `Builder::refuse_splice(cmd, diag)` records AND pushes, and
`refuse_inline(cmd, id, reason)` is its thin wrapper for the seven that speak one code. So an arm
that tells the author "Dorc did not look inside this call" while leaving the binding plane
believing it had cannot be SPELLED — which is what "one mint, one record" buys over eight inserts.

**The silent `?`-returns are NOT in the set, deliberately**: a non-literal command word, a word no
funcdef declares, and a word whose only definitions FOLLOW the call each leave `try_inline_call`
without a diagnostic, because the word is an ordinary unmodeled command that might be a PATH
binary. They stay `Opaque` at the effect tier exactly as before. That boundary is
`fnd-a-loaded-body-is-never-spliced`'s other half, and it is what keeps the two goldens still.

### measured — nothing moved, and it moves under the wrong seed

- Windows, before: `2679 passed, 2 skipped`. After: `2680 passed, 2 skipped` — the +1 is this
  lane's own control cell. Linux leg: `2674 passed, 2 skipped` (the six-test delta is the
  pre-existing platform-gated split, unmoved by this lane).
- ZERO goldens, transcripts, run-sets or looms moved. The two `§retrofit-execute` named —
  `cli/tests/definition_frames.rs the_engine_names_the_definition_the_shells_ran` over
  `floor30-blessed-override-above-and-below.loom`, and `cli/tests/load30-two-point-frames` — are
  both green and untouched. Nothing else moved either, so no law clause is owed.
- **FALSIFIED BOTH WAYS**, which is what the control cell is for:
  - seed removed ⇒ `a_call_dorc_cannot_splice_havocs_the_cwd_below_it` reddens
    (`left: 1, right: 0` on the fold count) and the control stays green;
  - seed replaced by the WRONG one (`cfg.call_body_sites(id).is_none()`) ⇒
    `a_call_that_was_not_refused_leaves_the_cwd_determinate` reddens (`left: 0, right: 1`) and the
    pin cell passes.

  So the E3C bug is now caught at the UNIT tier, in milliseconds, by a cell whose doc says why —
  rather than expensively by two goldens whose failure named neither cause.
- `mise run both gate:full-quiet` green on both legs (`preflight gate: ok — disk 205.5 GiB free,
  ram 10.1 GiB free`); `check-quiet` and `clippy` green.

### the-red-cell's-shape-was-right

`deploy() { shift; cd /srv; }` refuses through the `UnmodeledPositional` arm — a genuinely REFUSED
call that mints `CFG_INLINE_REFUSED`, not merely an un-spliced one. No edit to the cell's book was
needed; its interim assertions are deleted, as the greening commit owes.

### deviations — every one OPEN

1. **`dev-the-depth-two-note-joins-the-refusal-set`** — the brief scoped the record to the arms
   that mint `CFG_INLINE_REFUSED`. The depth-2 positional arm mints a different code and is
   recorded anyway, because it refuses a splice by every operational meaning of the word, and
   excluding it would leave one blind act unmodelled for a reason about diagnostic taxonomy rather
   than about the shell. Withholding direction; no test moved. Conductor re-derives.
2. **`dev-the-arms-were-refactored-not-only-instrumented`** — eight `self.diags.push(…); return
   None;` blocks became eight `return self.refuse_inline(…)` / `refuse_splice(…)` calls. Strictly
   mechanical (the diagnostic payloads and their push order are unchanged), and it is what makes
   one-mint-one-record structural rather than a convention eight sites must remember. It does
   inflate the diff of a lane whose behavioural change is two lines.
3. **`dev-a-control-cell-was-added-beside-the-pin`** — the brief asked for the pin promoted; a
   second cell, `a_call_that_was_not_refused_leaves_the_cwd_determinate`, is committed with it.
   `an-artifact-set-runs-from-its-own-generation`'s general law is the reason: an assertion that
   cannot distinguish the right seed from the wrong one is not a demonstration, and the wrong seed
   is the one that just cost a lane its execute half.
4. **`dev-load-sites-doc-was-stale-in-two-clauses`** — `load_sites`' own `# The cwd pass` doc still
   described the PRE-`8ac0e103` seed ("a `.` whose head could not be evaluated … NOT the
   merely-unread bucket") and the PRE-`6c7f0443` carriage posture ("still mirrored at its authored
   relative path"). Both are false at the tip and sit directly above the line this lane edits, so
   they were re-said rather than left. Source doc comment only; no `CLAUDE.md`, no root doc.
5. **`dev-base-was-two-doc-commits-ahead-of-the-brief`** — the brief named tip `d928ee29`; the
   worktree came up at `c228113d`, which has `d928ee29` as an ancestor plus two doc-only conductor
   commits (`FORFEITS` / `LIVING_STATUS` / `Research` / `TODO-ADDTL`, no code). Verified by
   `merge-base --is-ancestor` and `diff --stat` before proceeding, rather than stopping.

### tc-flags — cross-cutting, for the human

1. **`tc-a-held-body-is-modelled-as-text-not-as-shell-state`** — the ruling this lane implements is
   that a call into a body the controller HOLDS AND MODELS is not blind. That is right about
   ATTRIBUTION and it is what keeps the goldens still, but it is worth saying plainly what the
   engine models today: a cross-file oracle body is held as TEXT (lifted for roles, sliced for
   shipping) and is not on the caller's route at all, so `command_transfer` never walks it and its
   own `cd` is as invisible as an unspliceable call's.

   ```sh
   . ./oracles/site.dorc.sh      # held, modelled, lifted — and NOT spliced at the call below
   site_prepare                  # its body: `cd /srv/site` … invisible to the cwd domain
   . ./oracles/docker.dorc.sh    # today: still EXACT, still bound, still shipped
   ```

   Closing it is NOT the same act as this lane: the population is every helper call in every book,
   so the conservative reading costs a licence everywhere, which is exactly what reddened the two
   goldens. The honest options are (a) accept it as the oracle contract's residue — an oracle body
   that `cd`s without returning is a contract violation the author owes — or (b) splice cross-file
   bodies, which is `seam-interproc`'s cross-file half and its own round. This lane took neither;
   its lean is (a) WITH the sentence written into steering, because the gap is currently recorded
   nowhere.

2. **`tc-an-over-budget-wrapper-now-costs-later-loads-their-carriage`** — the widened seed makes
   `tc-nothing-shipped-costs-the-common-book-its-plan` reachable through a NEW door, and that door
   is the shape `30L:req-census-admits-the-wrapped-book` re-sized the budgets for:

   ```sh
   main() { … 300 lines … }      # over MAX_NODES_PER_SITE ⇒ REFUSED ⇒ a blind act
   main
   . ./oracles/docker.dorc.sh    # ¬EXACT ⇒ no bindings, no vouch, and NOTHING SHIPPED
   docker run --name web nginx   #   so the generated plan dies at the `.` on the host
   ```

   No corpus book has it (zero goldens moved) and the direction is withholding, so nothing unsound
   ships. But a budget is now ALSO a carriage knob, which sharpens
   `splice-budgets-are-licensure-not-perf` by one notch: raising a budget can now restore a
   dependency to the shipped tree, not merely reveal a mutation. Same veto as
   `tc-nothing-shipped-costs-the-common-book-its-plan`; one human line settles both together.

3. **`tc-a-detached-body's-own-loads-keep-their-authority`** — the clobber closure walks from
   `cfg.entry()`, so a `.` INSIDE the refused body is never reached and never marked. Its site is
   still enumerated by `load_sites` (which iterates every node), so it still resolves, still binds
   and still ships. That is the pre-existing `vacuous-entry-fold` seam rather than anything this
   lane introduced, and the wrapped-book shape above puts real weight on it. Reported, not touched.

### proposed-steering-and-register-edits (conductor applies; builders edit no `CLAUDE.md`)

**Base note.** This lane branched at `c228113d`; the conductor's `9ba6a9dc` re-say of the load-plane
steering landed after, and it is READ here — the deltas below are stated against ITS text, not
against the older bullet, and the lane is unrebased (docs-only upstream; no code interaction).

- `analysis/CLAUDE.md rul-havoc-is-pointwise-never-the-stack` — its last-but-one sentence is now
  false at the tip. Replace *"The law names THREE blind acts; `eval` is closed by refusal and the
  unheld `.` is modelled; a call Dorc cannot SPLICE (over-budget · recursive · out-of-slice) is
  minted only as the `CFG_INLINE_REFUSED` diagnostic and is pinned red until the CFG's refusal set
  is queryable (`p-x-an-unspliceable-call-havocs-the-cwd`)"* with: *"The law names THREE blind acts
  and all three are modelled: `eval` is closed by refusal, the unheld `.` seeds directly, and a
  call Dorc cannot SPLICE is queryable as `Cfg::splice_refused` — one node set recorded at the
  refusal arms themselves by `Builder::refuse_splice`, which is the ONE seat that both announces a
  refusal and records it, so an arm cannot tell the author Dorc did not look inside a call while
  leaving the binding plane believing it had."*
- `analysis/CLAUDE.md rul-havoc-is-pointwise-never-the-stack`, the wrong-seed paragraph — KEEP
  VERBATIM; it is exactly right and it is now also mechanically guarded. Worth appending only the
  guard's name: *"`a_call_that_was_not_refused_leaves_the_cwd_determinate` is where that misreading
  reddens at the unit tier, so it can no longer reach a golden."*
- `analysis/CLAUDE.md splice-budgets-are-licensure-not-perf` — proposed rider: *"since the third
  blind act, a budget also decides CARRIAGE: an over-budget call is a cwd clobber, so a relative
  `.` below it is non-EXACT and its target is shipped by no form
  (`30Qf:tc-an-over-budget-wrapper-now-costs-later-loads-their-carriage`)."*
- `analysis/CLAUDE.md opaque-poison-is-the-product` — its "keep every refusal arm (splice
  ineligibility, over-budget, recursion) poison-preserving" gains a second obligation worth naming:
  a new refusal arm must also RECORD, and `Builder::refuse_splice` is the only seat that does both.
  Its parenthetical also under-counts: there are EIGHT arms, not three.
- `FORFEITS.md` / `ANALYZER-NEEDS.md` — no edit owed; neither ever named this pin.
  `internal_tooling::xfail::PINS` is the only register that moved.
