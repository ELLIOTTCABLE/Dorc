# 30Mc - Round-30 first-half review (Sol neutral)

> [+SURE] Tier: post-build engineering review of `68709783..3011daae`, against the
> root product documents, the `spike/**/CLAUDE.md` law, `28Q`, and the round-30
> plans and lane records. [+SURE] The review concentrates on wrong execution,
> unruled behavior with design weight, and whether the landed core composes with
> `30I`/`30L`; it does not re-review the known `30La` aggregate-primacy defect.
>
> [+SURE] Method: read the orientation corpus and crate laws; inspect the
> definition-resolution, static-loading, effective-world settlement, certifier,
> Spine, influence, and projection seats; trace candidate failures through both
> probe and apply; and exclusion-check them across admin/engineer, reliable/
> unreliable oracle, and source/apply directions. [+SURE] No product code was
> changed. [+SURE] Two ignored regression demonstrations were added and run
> explicitly through `mise`: one observes transitive pre-source replay restoring
> a removed definition; the other observes a sourced assignment failing to reach
> its caller and the possible-load account omitting the resulting dependency.
> [+SURE] The ordinary acceptance gate leaves both demonstrations ignored, and
> `mise run gate:full-quiet` passes with them committed.

## 1. Critical: a transitive pre-source is executed again as an ambient root

**[+SURE] `30Mc:finding-transitive-pre-source-replays-as-root`** -
`spike/crates/cli/src/main.rs:372-403` recursively appends every marked file a
named pre-source names. `read_book_sourced` then takes `ambient = paths.len()`
*after* that expansion (`main.rs:427`), so those appended dependencies lie below
the ambient boundary. `StaticLoadSnapshot::over` classifies every source outside
the later book-sourced set as `SourceRole::NamedLoad`
(`spike/crates/cli/src/snapshot.rs:100-107`), and `definition_table` consequently
calls `push_ambient` for each of them (`spike/crates/cli/src/world.rs:753-757`).
Finally, `run_ambient_prefix` evaluates every ambient root's complete load program
in vector order (`spike/crates/analysis/src/funcenv.rs:2030-2053`).

> `30I:rul-static-loading-is-the-whole-model` [TYPED]: "Dorc models one closed,
> authored-before-contact sh loading process. A supported `.` resolves exactly
> as the floor shells would in its modeled working directory and environment."
>
> `30I:rul-pre-source-is-dot-prelude` [TYPED]: "repeated `--pre-source <sh>`
> inputs compile to ordinary `.` commands immediately before the main book body,
> in CLI occurrence order."
>
> `30I:rul-one-loader-many-projections` [TYPED]: analysis, probe/apply closure
> emission, bundling, and plan emission "consume the same resolved load
> structure."

**[+SURE] `30Mc:reason-dependency-runs-twice`** - for one CLI input
`--pre-source entry.dorc.sh`, where `entry.dorc.sh` contains
`. ./verdict.dorc.sh`, ordinary sh performs one root act and runs the dependency
inside it. [+SURE] The landed model instead evaluates this order:

```text
root entry.dorc.sh
   nested verdict.dorc.sh
root verdict.dorc.sh             # synthetic second act
```

**[+SURE] `30Mc:reason-dedup-is-not-execution-dedup`** - the path dedup in
`read_sourced_oracles` prevents a second copy of the bytes from entering the
snapshot, but it does not preserve which vector elements are invocation roots.
[+SURE] The only role distinction is `NamedLoad` versus `BookSourced`; there is
no representation for "dependency acquired for this named root." [+SURE] That
turns acquisition membership into execution membership, contrary to the plan's
ordinary-`.` ordering rule.

**[+SURE] `30Mc:world-post-source-removal-is-undone`** - the smallest dangerous
world uses only admitted load-program vocabulary:

```sh
# entry.dorc.sh
# dorc-lang/v0.2
. ./verdict.dorc.sh
unset -f wombat__is_converged
```

```sh
# verdict.dorc.sh
# dorc-lang/v0.2
wombat__predict() { ...; }
wombat__is_converged() { wombat cmp -- "$@"; }
```

```sh
# book.sh
wombat sync a.conf
```

**[+SURE]** Under the promised prelude, `. ./entry.dorc.sh` first defines and
then removes `wombat__is_converged`; it is absent before the book. [+SURE] In the
landed environment, the synthetic `verdict.dorc.sh` root runs after the removal
and rebinds it. [+SURE] With an otherwise valid prediction row and a converged
probe answer, Dorc can therefore attach the restored verdict vouch and guard or
replace `wombat sync a.conf`; under the modeled sh program no such judgment is
live, so the command must run. [+SURE] This is the priority-1 failure: engine-
created authority can suppress needed admin bytes even when both oracle files
are truthful.

**[+SURE] `30Mc:consequence-bundle-roots-are-already-corrupted`** - this also
conflicts with the unbuilt `30I` projection. [+SURE] The load account records a
synthetic `LoadSourcer::Invocation` occurrence for every promoted dependency,
so a future "one bundle per static load occurrence" consumer would inherit root
occurrences the admin never authored. [+SURE] `30L` cannot repair this later at
the elision-region layer because the wrong definition is already selected before
classification.

**[+SURE] `30Mc:test-idempotent-dependencies-hide-replay`** - the relevant whole-
product coverage exercises sourced helpers and diamonds, but its dependencies
declare/assign idempotently. [+SURE] Replaying those bytes produces the same
final bindings, so the tests prove transitive availability and custody without
proving one-time source order. [+SURE] A search of the CLI fixtures found no
oracle entrypoint that both sources a dependency and then `unset -f`s one of its
definitions; the existing removal tests place the removal in the book or in a
single already-modeled frame.

**[+SURE] `30Mc:demonstration-pre-source-replay-restores-definition`** - commit
`c304dc99` adds the ignored test
`a_pre_source_dependency_runs_only_at_its_authored_dot`. Running it explicitly
fails with `Live(DefinitionId { file: SourceFileId(1), ... })` where the
shell-faithful expectation is `Withheld`. [+SURE] This executes the acquisition,
snapshot, definition-table, and function-environment path together; the critical
finding is therefore demonstrated rather than inferred only from vector roles.

**[+SURE] `30Mc:required-root-occurrence-identity`** - acquisition needs to retain
the explicit ordered pre-source roots separately from the files acquired for
their load programs. [+SURE] The definition table may register every acquired
file by canonical key, but `push_ambient` must receive only the invocation roots;
`run_program` then reaches each dependency at its authored `.` position. [+SURE]
A regression should cover both directions: a dependency definition removed
after source stays absent, and a definition made after source stays later than
the dependency, with the executable floor shell and Dorc agreeing.

## 2. High plan blocker: sourced assignments vanish from the possible-load account

**[+SURE] `30Mc:finding-dot-locals-are-discarded`** - a load target is expanded
against the caller's mutable `locals`, but the loaded program receives
`&mut locals.clone()` (`spike/crates/analysis/src/funcenv.rs:1501-1531`). Its
top-level assignments are therefore discarded when `run_program` returns.
[+SURE] This contradicts the ordinary POSIX `.` model and differs from the
ambient-prefix path, whose explicit design is one shared locals map across
successive pre-sources (`funcenv.rs:1998-2054`).

> `30I:rul-static-loading-is-the-whole-model` [TYPED]: "A supported `.` resolves
> exactly as the floor shells would in its modeled working directory and
> environment."
>
> `30I:rul-one-loader-many-projections` [TYPED]: analysis, probe/apply closure
> emission, explicit bundle emission, plan emission, and flattened artifacts
> "consume that same resolved load structure."
>
> `30I:rul-one-load-account-separate-projections` [PROPOSED mechanism]: the
> possible-load projection contains "every statically supported target on any
> arm" and bundle materialization consumes it.

**[+SURE] `30Mc:world-sourced-root-sites-later-dependency`** - this ordinary
admin/engineer composition is enough:

```sh
# root.dorc.sh
# dorc-lang/v0.2
OPS_LIB=./vendored

# entry.dorc.sh
# dorc-lang/v0.2
. "$OPS_LIB/common.dorc.sh"

# book.sh
. ./root.dorc.sh
. ./entry.dorc.sh
wombat sync a.conf
```

**[+SURE]** A shell leaves `OPS_LIB` live after the first dot and loads
`vendored/common.dorc.sh` at the second. [+SURE] Dorc discards the assignment,
cannot expand the nested target, floors the function environment, and records
only `root.dorc.sh` and `entry.dorc.sh` as taken occurrences. [+SURE] This is
conservative for current licensure: `Top` withholds rather than elides. It is a
high plan blocker because the not-yet-built bundle projection is instructed to
trust this one account; it would omit a file the apply-time shell loads, making
the emitted artifact fail to reproduce the analyzed book.

**[+SURE] `30Mc:demonstration-possible-load-account-omits-runtime-file`** - commit
`5e614861` adds the ignored test `a_sourced_assignment_sites_a_later_load`.
Running it explicitly fails with actual taken targets `["root.sh", "entry.sh"]`
against expected `["root.sh", "entry.sh", "vendored/common.sh"]`; the resulting
role binding is also `Top`. [+SURE] The account must thread the loaded program's
post-state back to its caller before bundle work treats possible-load closure as
complete.

## 3. High latent defect: Spine says most invalidators are not invalidators

**[+SURE] `30Mc:finding-spine-invalidator-reads-kills-only`** -
`SpineSiteClassification::invalidator` is documented as "Whether the site gens
into reach as an invalidator" (`spike/crates/core/src/spine.rs:398-399`), but
`record_new_arm` receives only `round.kills` and writes
`kills.contains(node)` (`spike/crates/cli/src/main.rs:1766,2432-2536`). [+SURE]
The actual invalidator set is independently computed from every
`Establishes`, `Kills`, and `Opaque` effect, including non-leaf nodes
(`spike/crates/analysis/src/effect.rs:1915-1920,2118-2128`).

> `309` section 0 [TYPED]: Spine is "the global in-memory structure everything
> hangs off - every decision, its inputs (as capped accounts), its influence
> grade, its narration."
>
> `30E` census: `SpineSiteClassification` is the classify account, including the
> `SkipClass`, verdict lane, kills, kill coordinates, fact backings, and degrade
> causes.

**[+SURE] `30Mc:world-establish-records-false`** - an ordinary modeled establish
such as `apt-get install nginx` is in `classification.invalidators` and not in
`kills`; its Spine record therefore says `invalidator=false`. [+SURE] An opaque
leaf has the same false record. [+SURE] Only a kill-shaped leaf (and the outer
inline-call kill summary) can currently make this field true.

**[+SURE] `30Mc:current-plan-does-not-read-false-bit`** - this does not corrupt
today's plan: `SpineSiteClassification` is in the non-durable `new` arm, the
debug dump has no production caller, and effective reach reads the genuine
`RoundClassification.invalidators` before this recorder runs. [+SURE] The defect
is nevertheless high design-risk because the record is semantically false in
the structure future products are directed to project.

**[~SUSPECT] `30Mc:consequence-route-work-may-retire-real-walls`** - `30L`'s
definition-keyed route instances and shared edit decisions require exact
per-instance mutation facts. [~SUSPECT] If that work treats the existing Spine
field as its stated meaning, establishes and opaque operations appear
non-invalidating and a shared region can retire a wall it never proved dead.
[+SURE] The safe boundary is to correct or delete this field before any new-arm
record becomes an authority-bearing projection; it must be populated from the
full final invalidator set, not reconstructed from `SkipClass` or `kills`.

**[+SURE] `30Mc:adjacent-inline-account-is-empty`** - the same writer maps
`SkipClass::InlineCall { sites }` to an empty `cells` account
(`main.rs:2527`), even though the `sites` vector is the call's ordered
effect-bearing member account. [+SURE] That omission is safe only while the
record remains debug-only; it is another reason the current species cannot be
used as `30L`'s route substrate without a fresh census.

**[+SURE] `30Mc:census-classifies-types-not-population`** - the incompleteness is
broader than those two fields. The recorder states that `SpineVouch`,
`SpineObservation`, and `SpineValidityRound` are "NOT YET MINTED"
(`main.rs:2415-2427`), and the explain-only `WhyWorld` projects a Plan without
calling the new-arm recorder at all (`spike/crates/cli/src/world.rs:465-488`).
[+SURE] The no-wildcard `SpineSpecies` census proves that each record *type* has
a projection arm; it cannot prove that either driver populates the type or that
the populated operands mean what their fields say.

**[+SURE] `30Mc:durable-invocation-mode-is-hard-coded-wrong`** - the durable arm
adds another already-observable false record: after both plan and apply runs,
`record_durable_arm` writes `SpineInvocation.mode = "whylog-replay"`
(`main.rs:2095-2119,2552-2566`). [+SURE] That function is unreachable from the
actual `Mode::Why` replay/report branch, which returns before artifact and durable
emission (`main.rs:2005-2079`). The current reader validates and preserves this
mode as receipt metadata; the record therefore describes neither producing
invocation. [+SURE] This is aid/provenance corruption rather than a license hole,
but it demonstrates why a population-and-meaning audit is owed before calling
the Spine transition complete.

## 4. Medium design conflict: one Spine grade cannot represent the required record set

**[+SURE] `30Mc:finding-spine-grade-is-object-global`** - `Spine` stores one
`grade` field and every setter overwrites its record's grade from that field
(`spike/crates/core/src/spine.rs:634-650,684-709,795-809`). The settlement Spine
is constructed with the post-intake influence marker
(`spike/crates/cli/src/main.rs:1734-1743`; `spike/crates/plan/src/settle.rs:200-207`),
and only later receives load-decision and invocation records
(`main.rs:1847-1860,2475-2489,2552-2580`). They are consequently stamped
host-influenced.

> `309` §2 [ACKED]: "every Spine record stamps the v0 influence phase marker at
> mint (`authored-before-contact` pre-ingestion; `host-influenced` after)."
>
> `30I:rul-load-decisions-are-authored-before-contact` [TYPED direction]: "every
> v0 load decision is stamped `authored-before-contact` by construction" and
> records a complete `SpineLoadDecision`.

**[+SURE] `30Mc:reason-two-rulings-cannot-coexist-in-current-type`** - positional
v0 grading and authored-only loader inputs are individually coherent, but the
landed representation cannot express their required combination. A load decision
is logically fixed before intake but physically copied onto the only Spine after
intake; the object-global setter makes physical recording time win. [+SURE] The
same problem applies to controller-authored invocation framing. Neither current
Plan licensure nor the durable projection consumes load-decision grades, so this
does not create a present wrong-elision. It is a design-locking default: future
marking-frontier, influence-debug, or bundle work either reports false provenance
or must break the "one grade per Spine" representation.

**[+SURE] `30Mc:required-grade-boundary-must-be-priced`** - preserving both typed
directions requires one explicit design act: mint authored records onto a Spine
before intake and carry it through settlement, or permit record-local grades
whose constructors cannot claim authored provenance without the loader's typed
witness. [+SURE] Continuing to add late recorders silently chooses the opposite.

## 5. Medium unruled default: per-pass certification is collapsed into an invented pass

**[+SURE] `30Mc:finding-certification-window-replaces-passes`** - the type says
`SpineSolveCertification` is "One solve pass's certification outcome" and names
the pass vocabulary (`spike/crates/core/src/spine.rs:407-417`); the `30E` census
likewise specifies "per-pass consistency + the `CertifierTrip` latch." [+SURE]
The only production writer deliberately emits one row named `whole-window`, with
both `consistent` and `tripped` derived from the run-wide latch
(`spike/crates/cli/src/main.rs:2465-2474`).

> `309` section 0 [TYPED]: Spine contains "every decision, its inputs (as capped
> accounts), its influence grade, its narration."

**[+SURE] `30Mc:reason-latch-cannot-reconstruct-pass-account`** - the monotone
latch answers whether *any* pass ever failed; it cannot recover whether the
failed pass was value, function environment, origin reach, self reach, or one of
the effective-reach settlement rounds. [+SURE] It also makes all clean passes
disappear. [+SURE] The existing pass-specific diagnostics retain some of this
information elsewhere, which demonstrates that the data exists, but the Spine
record intentionally does not ingest it.

**[+SURE] `30Mc:consequence-schema-locks-the-summary`** - no current plan decision
reads this debug-only row, so this is not a present wrong-elision. [+SURE] It is
an unruled schema choice at exactly the reification boundary intended to prevent
parallel accounts: a later why/debug/verification projection either inherits the
loss or must consult the old scattered pass products again. [+SURE] The choice
also makes the `pass` field's documented closed meaning false by admitting an
extra summary label.

**[+SURE] `30Mc:required-certification-is-event-plus-latch`** - retain one row per
actual certification event, including settlement round/seat identity where a
pass can repeat, and record the run-wide trip as its own summary state rather
than substituting it for those rows. [+SURE] This need not widen the durable;
the present census already classifies certification as non-durable.

## 6. What held under review

**[+SURE] `30Mc:held-effective-world-authority-path`** - after the landed repairs,
the effective-world settlement carries the full residual invalidator set,
filters it through recorded `ExecutionOwner`, certifies each reach solve, derives
query validity from that one answer, and writes final dispositions only at
quiescence. [+SURE] The terminal certifier cleanup mutates Spine dispositions
before `project_plan`, so plan, artifact, digest, and why consumers see the same
demoted decisions.

**[+SURE] `30Mc:held-aggregate-universal-repair`** - aggregate replacement now
shares one ordered non-empty identity between vouch authorization and freshness,
and the survival path checks every erased establish rather than a representative.
[+SURE] I found no surviving version of the already-repaired first-member
authority bug.

**[+SURE] `30Mc:held-exact-custody-guard`** - the guarded-source recognizer requires
the idiom's exact shape, sole sentinel population, no removal of target
definitions, and a frame-confirmed source/reuse arm. [+SURE] Undecided guards
produce speculative load occurrences, and the helper closure independently
checks that the reached declaration lies in speaker custody. [+SURE] The
initial hypothesis that occurrence flattening minted custody from an undecided
branch did not survive this trace.

**[+SURE] `30Mc:held-certifier-helper-floor`** - the initial concern that a
function-environment certifier trip could preserve a guard carrying a wrong
helper snapshot did not survive exclusion-checking. [+SURE] Value and funcenv
failures floor the environment before vouches are built; origin-reach failures
floor classifications; the trip species that can leave a guard therefore do not
select its helper closure. [+SURE] The census-only guard exception remains
conservative on the reachable paths inspected.

**[+SURE] `30Mc:held-definition-keyed-resolution`** - definition-derived rows
compare their own `DefinitionId`, and real drivers query them through the
positionally live definition. [+SURE] I found no new name-join or file-grade
fallback in the license path; the critical loading finding instead feeds that
correct mechanism the wrong root sequence.

## 7. Overall assessment

**[+SURE] `30Mc:assessment-settlement-is-stronger-than-loading-edge`** - the
round-30 effective-reach core is substantially better defended than the static
loading edge feeding it. [+SURE] I found no new wrong-elision in the settled
world kernel after the known repairs, but the transitive-pre-source promotion is
itself a demonstrated critical wrong-definition route upstream of every later
proof. [+SURE] Independently, discarded sourced assignments make the possible-
load account incomplete, so the planned bundle projection cannot safely consume
the landed loader unchanged.

**[+SURE] `30Mc:assessment-spine-is-not-yet-a-safe-input`** - the reified Plan
projection is real and current dispositions are coherent, but the transitory
classification and certification species are incomplete summaries that
contradict their documented meanings, the durable invocation mode is already
false, and object-global influence grading cannot represent the required mix of
pre- and post-contact records. [+SURE] These should be treated as unfinished
schema, not as ready substrate for `30L`, bundle emission, a new durable view, or
another authority-bearing projection.
