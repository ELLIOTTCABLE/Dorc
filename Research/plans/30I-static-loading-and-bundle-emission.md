# 30I - Static loading and bundle emission

> Tier: LLM-authored plan from the 2026-08-18 human design dialogue. The
> rulings marked **[TYPED]** were typed by the human; **[ACKED]** means the
> human accepted the stated substance; **[PROPOSED]** is implementation shape
> left to builders. Root human documents, `spike/CLAUDE.md`, and earlier typed
> law outrank this plan.
>
> This is the single design home for: working-directory-faithful oracle
> loading; book-spelled oracle roots; cross-author deterministic dependencies;
> bundle compilation; book-tree and oracle-tree emission; bundle provenance;
> and the CLI/TUI plan projections. It closes `30G:b8-book-side-unwalling`'s
> artifact question in direction, supersedes
> `30G:dev-sourced-paths-resolve-against-the-sourcer`, and gives
> `28Q:pin-emission-planner-universal` one concrete consumer. Implementation
> state and the remaining work are kept current below.

## Implementation plan - the remaining lane

`impl-one-lane-sequential-builders` - the load model, frame answers, custody,
bundle projection, locator graph, and artifact emission are one coupled model.
They are built by one builder at a time, never by parallel owners: a second
resolver or a decorative source map is a likelier product of splitting than saved
wall-clock. Seams are placed at FIXED work-order boundaries and never on a
builder's own read of its remaining context, which is not a thing a model can
measure. Every handoff names its work-order boundary rather than an ordinal; the
active one is `step-5a-complete-load-occurrence-account`, before bundle emission.

`impl-effective-reach-interposes-before-bundles` - HUMAN-DIRECTED sequencing
correction (2026-08-19): `step-5a-complete-load-occurrence-account` first removes the
superseded ambient-dependency refusal and extends the ONE loader account to preserve
every possible resolved load occurrence with locus/context. It then STOPS for
`notes/30K`'s effective-world-reach kernel stage. Bundle projection, locator
consumption, artifact forms, and final XFAIL/golden promotion resume only after that
stage lands. `30K` is not added to a loading builder's remit; the durable `30Ib`
handoff makes this pause cheap. Ground: final plan dispositions must settle before
new executable projections and their corpus are built around the old wall walks.

### Where the build stands

Landed (`ai/r30-static-loading`; as-built map, consumption API and open questions
in `Research/notes/30Ib`):

- one controller-side `Cwd` (`core::loadpath`) resolving `.` as sh does;
- one immutable `StaticLoadSnapshot` carrying a per-source role, read once at the
  edge, consumed by every inner answer;
- each loadable file's top level as a closed `LoadProgram` (`analysis::load`),
  interpreted in place at each load site by `funcenv`, with acquisition at the
  edge DRIVEN by that loader rather than by a second resolver;
- the first load-condition species (`command -v`), `unset -f`, subshell
  scoping, diamonds and cycles;
- book-source visibility without book speaker minting;
- `aid::locator` as an arbitrary multi-stage DAG with `BundleOriginClaim` sealed
  to text, and `cli::provenance` filling it from real run data;
- exact guarded-source speaker recognition, the CLI input rework, and explicit
  stdin claims.

Not built, and owed below: the bundle projection; the three artifact forms;
locator consumption by a real diagnostic; XFAIL promotion and e2e lowering.
The current implementation still carries a whole-run refusal for an ambient
cross-custody dependency. Section 3.4 rules that as drift: retain its
differentiation as narrative and remove its authority/control-flow consequence.
`step-5a-complete-load-occurrence-account` completes the possible-load occurrence
account; after that commit the lane pauses at `30K` before
`step-5b-build-bundle-projection` writes bundle code.

### Target outcome

The lane is complete when all of the following hold together:

1. A supported book/oracle `.` resolves with sh cwd/value-flow semantics from
   one immutable authored source snapshot.
2. Book loads affect visibility but never mint speaker custody; marked
   dorc-lang source edges form the existing asymmetric custody closures.
3. The canonical include-guard/shared-dependency and subshell worlds resolve
   positionally; ambient cross-custody dependencies suspend vouch composition
   and produce differentiated narrative without refusing the plan.
4. Analysis, existing probe/guard closure emission, explicit `dorc bundle`,
   multipart plans, and fully flattened plans consume ONE load answer.
5. Bundle compilation carries a typed multi-stage locator DAG into at least one
   real diagnostic; bundle comments re-enter as aid-only `BundleOriginClaim`.
6. The CLI input surface is the one in section 2.4 and 2.5: one main book per
   target, ordered `--pre-source` dot preludes, no short options, `-` naming
   stdin in any filename position, and a piped stdout implying one flat plan
   for one target. `-o`/`--oracle` and multi-book concatenation no longer exist.
7. The three `load30-*` XFAILs green and are promoted; the measured
   dot-vs-function floor is unchanged; lowerable properties sit in fast native
   tests while only irreducible whole-artifact coverage stays e2e.
8. Existing empty-world, single-frame, closure-custody, certifier,
   re-derivation, plan-runnability, and both-platform gates remain green.

Concrete type names, artifact filenames, output-directory naming, and the
eventual target-qualifier grammar remain builder latitude where section 14 says
so. That latitude never includes returning a different semantic artifact form
than the user explicitly requested.

### Neighboring work that stays out

`28Q:stage-iii-world-scopes` and `28Q` section 10's authored lifecycle surface;
blessing-reach elevation and verdict-word enrollment; committee-fence permanence
and broader sparing composites; the
starter stdlib and its dialect-reach decision; at-most completion speech;
callback/bare dependency-injection VOUCH COMPOSITION beyond the conservative
v0 floor; the parked
`SortedSet::union` optimization; minispec enrichment or verified-core law changes.

Do not opportunistically green their xfails, widen their allow-lists, or fold
their decisions into a convenient load abstraction. A real prerequisite is a
blocking finding, not permission to absorb the adjacent arc. The sibling
dorc-loom work is mechanically independent.

The modeled-running-wall/guard-tier repair is no longer neighboring work to defer:
it is the mandatory `30K` interlude above. It remains OUT OF SCOPE for every `30I`
builder and is implemented by its own kernel lane.

### Work order

Each step unblocks the next.

1. **`step-1-run-pre-sources-as-programs` - run CLI-supplied loads as ordinary
   programs.** A pre-source is a `.`, so it
   runs its `LoadProgram` like any other load rather than contributing a flat
   declaration list. This is what makes a CLI-supplied package's include guard
   evaluate at all, and it is the prerequisite for
   `step-2-close-variable-rooted-custody`.
2. **`step-2-close-variable-rooted-custody` - close custody for variable-rooted
   dependencies.** The loader already
   resolves `. "$ROOT/dep.sh"` correctly; what is missing is reporting the edges
   it walked so the include-tree consumes them instead of re-resolving a literal.
   Watch the two helper-package pins.
3. **`step-3-rework-cli-input-surface` - rework the CLI input surface** to
   sections 2.4 and 2.5, retiring
   `-o`/`--oracle` and multi-book concatenation in the same arc. Reds while this
   rips are expected; the greening must land the replacement, never restore
   either. Multiple main books become separate targeted programs, which is where
   the one-`Ast`-per-run assumption is unpicked; shared-shell merging is spelled
   `--pre-source`.
4. **`step-4-recognize-exact-guarded-source` - complete exact guarded-source
   recognition and cross-custody narration**
   (`rul-guarded-source-mints-exact-speaker-edge` plus section 3.4's
   `rul-cross-custody-distinction-is-narrative`). Classify a voucher's
   cross-custody reach as deliberate-external-utility, guarded-source-exact,
   explicitly-sourced, source-act-present-but-unaligned, or
   ambient-or-untraceable. Only the first three may avoid suspension; the last
   two differ for aid and never refuse the plan. The immediate authored target
   is section 2.2's ordinary variable-sentinel guard.
   The variable name and value are entirely author-owned; neither is Dorc
   vocabulary. The guarded-source case mints its edge only under section 3.4's
   exact proof: on the no-source/reuse route, both the guard-tested value that
   selected that route and every transitively load-bearing helper on the REACHED
   vouch path `Must`-originate inside the exact fallback target closure. On the
   source route, the explicit exact `.` is the ordinary speaker edge and its
   resulting definitions must resolve accordingly. Declining paths prove
   nothing.

   This is a PRECISION improvement, not a trust widening: recognize that the
   guard is not an analysis-time choice between speakers when all feasible routes
   provably land on the same exact foreign speech, and stop driving to ⊤ there.
   The condition never mints authority: the author's source-bearing guard, the
   target-supplied load value, and the reached helper definition must agree on one
   exact closure. A same-valued assignment or same-named helper from anywhere
   else withholds. Implementation should be the smallest generic extension of
   load-time constant propagation and source-unit provenance that expresses that
   property; add no sentinel registry, magic name, separate explanation witness,
   or durable state. Full `command -v` load modeling remains the open pin in
   section 14, not a prerequisite to this lane.

   Consequential and easy to miss: the committed `load30-*` fixtures still spell
   `command -v` guards from the earlier direction. Re-spell them to the sentinel
   form section 2.2 now carries, so the executable specification and the ruling
   agree. That is directed work, not a builder judgment call about the human's
   authored bytes.
5. **`step-5a-complete-load-occurrence-account`, STOP for `30K`, then
   `step-5b-build-bundle-projection`** - complete the load-occurrence account, then
   add one bundle projection keyed by static load occurrence, consuming the snapshot and the
   frame answers, never re-reading a path. First extend the one
   loader account so it preserves every statically possible resolved load
   occurrence, including speculative branches that mint no speaker edge; section
   6.1 defines the separate projections. This account plus the section 3.4 drift
   removal is the current builder's fixed deliverable. Commit and stop; the
   effective-world-reach stage in `notes/30K` lands next. Only after it is folded:
   copy authored segments exactly; add only
   necessary generated scaffolding and versioned boundary comments. Preserve
   nested source boundaries as generated files until a different lowering is
   floor-proven - generated loader functions are measured-refuted. Expose the
   same projection as explicit `dorc bundle`, as contracted multipart
   dependencies, and inline for a flattened plan.
6. **`step-6-compose-bundle-locators-into-diagnostics` - compose bundle segments
   onto the existing locator edges** and carry one
   chain through a REAL diagnostic render. A debug dump or structure-only unit
   test is necessary and not sufficient. This discharges the force-now aid
   requirement that could not close before the bundle existed.
7. **`step-7-reify-plan-artifact-forms` - reify artifact forms at the Plan/Spine
   boundary.** The executable product is
   the Plan projection plus its generated files, not ad-hoc CLI writes beside a
   string-only Plan; human and executable forms derive from one final structure.
   Implement the three semantic forms; auto chooses the most flattened safe one
   and explains a fallback; explicit single-stream intent refuses before network
   when unavailable. Use cwd analysis to avoid generated artifact-root scaffolding
   wherever a simple relative dependency path suffices. Publication is atomic:
   build fresh, finish every file, then publish.
8. **`step-8-promote-executable-specification` - promote the executable
   specification and close.** Promote each `load30-*`
   XFAIL only when its target run set is genuinely green; keep
   `head-expected.ran` until promotion proves behaviour did not drift by another
   route; keep `floor30-dot-loader-function-errexit` byte-identical. Re-spell
   `pin28-variable-resolved-source-loads`, whose header prose and pinned
   behaviour disagree since slash-less `.` became unresolvable - its goldens did
   not move, so the drift is invisible and must not be left to be noticed. Lower
   settled properties into fast native tests and remove the e2es they replace;
   keep one compound e2e per irreducible artifact interaction. Mint the owed
   floor cell for textual inlining before flattening rests on it.

Gate rungs follow the work lifecycle, never a per-change menu: pre-commit is the
automatic sub-three-second floor; `mise run both gate:full-quiet` is builder
completion and doubles as the working loop (there is no agent-facing quick rung
— `gate:quick-quiet` is retired); `mise run gate:arc` is the conductor's
arc-close, run from the populated branch BEFORE folding into `ai/main`, because
hk derives its applicable checks from that branch diff. A failed hk step names
its focused rerun, `mise run gate:step -- <step>`. Golden movement is enumerated
and reviewed as behaviour before blessing, never treated as expected refactor
churn.

### Stop conditions

Commit granularly and continue through ordinary failures. A builder-API
preference, a larger-than-expected refactor, or a red test is not a checkpoint.
Stop and report only when:

- a supported target requires semantics contrary to a typed ruling here;
- correct loading requires host/probe/runtime-discovered input;
- correct provenance requires per-line markers or a separate v0 source-map file;
- bundle provenance cannot reach a real error without changing whylog contents;
- a verified-core check, certifier, reference re-derivation, or minispec
  statement disagrees with the implementation (never weaken the instrument);
- a user-facing diagnostic requires human words rather than an unwritten register;
- builder-only quarantine instructions require the exact escalation they specify.

### Completion report

Return: commit list and final branch tip; which planned items landed and every
item that did not; all deviations, each left OPEN for conductor/human
adjudication rather than self-endorsed; XFAIL promotions and e2es
lowered/retained; exact golden drift and why; full gate results per platform;
remaining open pins from section 14; proposed steering-prose updates, without
editing any `CLAUDE.md`.

## 0. The design in one screen

`rul-static-loading-is-the-whole-model` [TYPED] - Dorc models one closed,
authored-before-contact sh loading process. A supported `.` resolves exactly as
the floor shells would in its modeled working directory and environment. Host
bytes, network results, runtime discovery, and bundle comments cannot affect
which source or definition is live.

`rul-books-load-but-do-not-speak` [TYPED] - book code may assign an oracle root,
source oracle entrypoints, and arrange packages in ordinary flowing sh. Those
acts affect visibility. Only dorc-lang files mint speaker-transitive custody
closures; a book source edge never merges the authorship of the packages it
loads.

`rul-guarded-source-mints-exact-speaker-edge` [TYPED 2026-08-18, v0] - a
recognized include guard with a contracted fallback source may mint that
target's speaker edge under the exact proof in section 3.4. On a reuse route,
the same exact foreign closure must supply both the decisive guard value and
the helper definition consumed by the reached vouch; a condition alone carries
no authority. This is the narrow correct floor while broader guard/load-order
composition remains NYI, not a permanent exclusion.

`rul-one-loader-many-projections` [TYPED] - analysis, probe/apply closure
emission, explicit `dorc bundle`, ordinary multipart plan emission, and fully
flattened plan emission consume the same resolved load structure. A second
bundle resolver or source-order model is forbidden.

`rul-plan-emission-has-three-resting-forms` [TYPED direction] - the planner may
emit: one fully flattened `plan.sh`; one `plan.sh` with contracted dorc-lang
bundle dependencies; or a less-flattened book tree when book semantics cannot
yet be safely inlined. Auto mode chooses the most attention-preserving safe
form. Explicit single-stream intent refuses before network when v0 cannot
satisfy it.

`rul-bundle-origin-is-aid-only` [TYPED] - a bundle read back in is arbitrary sh.
Its bytes alone feed analysis. Generated bundle comments may mint
`BundleOriginClaim` narrative and locator candidates; no analytic or licensing
type accepts them.

`rul-source-maps-are-rich-and-early` [TYPED] - the compiler aims at an extremely
simple copied-segment model, while the locator representation supports an
arbitrary multi-stage DAG and every user-aid consumer resolves it maximally.
Error output may name both the generated bundle locus and a content-verified
original source locus.

`rul-one-main-book-per-target` [TYPED direction] - each target plan has one main
book with near-`sh book.sh` command-file semantics. Other CLI-supplied sh that
must share its shell environment is explicit ordered prelude, never an implicit
second main book. Target-qualified main books denote separate programs/plans,
not a merge request; same-target multi-program orchestration remains outside v0.

`rul-pre-source-is-dot-prelude` [TYPED] - repeated `--pre-source <sh>` inputs
compile to ordinary `.` commands immediately before the main book body, in CLI
occurrence order. An unqualified pre-source applies to every selected target; a
target-qualified one applies only to that target. The main book is not itself
dot-sourced.

`rul-spike-has-no-short-options` [TYPED] - every single-letter CLI option is
reserved for post-spike allocation against the complete feature/flag set. No
spike feature receives a short spelling, including `--pre-source`; current
short-form expedients create no precedent.

The ordinary intended source shape is deliberately just sh. `SM_ORACLE_ROOT`
below is a spike mnemonic, never a permanent name or an engine-recognized
variable:

```sh
SM_ORACLE_ROOT=./oracles

. "$SM_ORACLE_ROOT/org.example.alpha/entry.oracle.sh"
. "$SM_ORACLE_ROOT/org.example.beta/entry.oracle.sh"

alpha sync
beta reconcile
```

## 1. Why this design exists

Dorc's primary product is the user's attention surface. It must simultaneously
preserve four things:

1. `need-plan-shows-book-mutations` - every possibly mutative book command stays
   on the primary `plan.sh` surface as written and in authored order.
2. `need-oracles-remain-ordinary-sh` - shared oracle packages remain good sh
   after stripping; Dorc injects no runtime package root, registry lookup, or
   source-file-location primitive.
3. `need-controller-paths-never-cross-hosts` - a controller path such as
   `/home/admin/ops/oracles` cannot be materialized at the same absolute path on
   every target.
4. `need-channel-capabilities-vary` - some transports can materialize an
   artifact directory; the `kBOOT` floor has only one byte pipe and may have no
   writable filesystem.
5. `need-main-book-remains-a-program` - adding transport targets, oracle
   preludes, or other plans must not silently turn the main book into a dot
   script or reinterpret later filename-looking tokens as merge requests.

No one-file-only design satisfies all four cheaply. Always inlining contracted
oracle code preserves the byte-pipe floor but spends attention. Always using
sidecars preserves attention but requires a materializable plan directory.
Requiring authors to pre-bundle imposes an adoption cliff. Rewriting a book's
root variable changes ordinary sh state. Runtime Dorc path magic breaks the
off-ramp.

The bundle projection is therefore one mechanism with placement chosen by the
emission planner. Complexity that does not serve those four needs is out.

## 2. The canonical authored sh

### 2.1 The admin's book is the loader

`rul-admin-loader-files-are-optional` [TYPED] - a separate local loader is an
ordinary refactor, never required ceremony. The admin will commonly place the
root and package source acts at the top of the book:

```sh
SM_ORACLE_ROOT=./oracles
. "$SM_ORACLE_ROOT/org.example.wombat/entry.oracle.sh"

wombat sync ./wombat.conf
```

Dorc recognizes no special root name. An organization may instead spell
`OPS_LIB`, `THIRD_PARTY`, or any other ordinary variable. The load plane cares
only that value flow resolves the source operand.

The likely community convention is one admin-chosen root plus stable
per-library directory names. It is documentation and filesystem agreement, not
a registry:

```text
oracles/
   org.example.alpha/
   org.example.beta/
   org.example.common/
```

### 2.2 Oracle packages own deterministic dependencies

A library entrypoint may source its own or another author's contracted package:

```sh
# dorc-lang/v0.2

if [ "${org_example_common_loaded-}" != 'org.example.common/v1' ]; then
   . "$SM_ORACLE_ROOT/org.example.common/entry.oracle.sh"
fi

alpha__is_converged() {
   example_common_query alpha "$1"
}
```

The target package defines its helpers and populates the tested value as
ordinary sh:

```sh
# dorc-lang/v0.2

example_common_query() {
   # ...
}

org_example_common_loaded='org.example.common/v1'
```

The names and literal are the authors' own package interface. Dorc recognizes
no `_LOADED` suffix, variable namespace, version grammar, or distinguished
value. The pattern earns its keep outside Dorc as the ordinary shell-library
include guard: it prevents duplicate initialization and can distinguish package
revisions. One package sentinel may cover any number of helpers.

`rul-include-guards-are-load-semantics` [TYPED] - include guards are mandatory
language surface, not optional polish. Independent oracle authors use them to
share dependencies and to say: reuse this exact contracted dependency when it
is already live; otherwise load this fallback. The function environment, custody,
emission, and bundle compiler must agree on that branch. For speaker custody,
the whole recognized guard is one authored dependency act under section 3.4;
ordinary visibility still follows the branch sh takes.

`command -v` remains a meaningful, idiomatic, supported route: it asks what a
shell would resolve under a command name, and dorc-lang must not force authors
to abandon that question. It is not the immediate exact-package guard because
its wider answer space (functions, aliases, builtins, reserved words, PATH
utilities, and implementation facilities) is neither floor-identical nor exact
package identity. Until `pin-command-v-load-model` closes, such guards retain
ordinary sh behavior and conservatively withhold wherever exact source
recognition cannot prove their intent. Evidence and floor measurements:
`notes/30Ic`.

`rul-oracle-loading-stays-load-safe` [TYPED direction] - supporting healthy
libraries does not mean arbitrary top-level execution. The v0 positive surface
is deterministic loading: definitions, known-value assignments, known-value
source operands, a deliberately selected set of include-guard control flow,
`unset -f`, and subshell-scoped loading. Dynamic discovery remains outside the
model.

### 2.3 Books do not squash authorship

The book may source Alpha and Beta. That does not make the book their shared
speaker. Alpha and Beta remain sibling roots:

```text
Book (visibility only)
|-- Alpha (speaker root)
|   `-- Common
`-- Beta (speaker root)
    `-- Common
```

Custody is asymmetric containment, never equivalence. Common may be reached
from both closures without Alpha and Beta merging. A later lint may discourage
book authors from marking aggregate loader/shim files as dorc-lang, but v0 adds
no policy net merely to police style.

### 2.4 CLI preludes and target books

The CLI's one settled loading flag is long-form only:

```console
dorc web.sh --pre-source common.oracle.sh --pre-source site_prelude.sh
```

Its semantic reference is plain sh:

```sh
. '/resolved/common.oracle.sh'
. '/resolved/site_prelude.sh'

# web.sh remains the main program body; it is not sourced.
```

`rul-main-book-is-not-a-pre-source` [TYPED] - this preserves the sanctioned
single-file path near `sh web.sh`: top-level `return` is not made valid merely
by Dorc, and the book is not given dot-file scope. The compiled plan naturally
has its own `$0`; that visible difference is accepted rather than hidden.

`rul-pre-source-order-is-cli-order` [TYPED] - after target filtering, pre-source
operations retain their CLI occurrence order. Their textual position relative
to target-book operands does not change their role: all execute before that
target's main body.

The eventual multi-target spelling is not ruled. A `web1:web.sh`-shaped
positional is a useful strawman for one target-to-main-book binding, not syntax
the loader may parse or depend upon. Invocation parsing first lowers any final
host syntax into explicit values:

```text
TargetedBook {
   target,
   main_book,
   ordered_pre_sources,
}
```

An unqualified `--pre-source common.oracle.sh` contributes to every
`TargetedBook`. A target-qualified
`--pre-source web1:book_two.sh` contributes only to `web1`, with semantics
identical to prepending `. '/resolved/book_two.sh'` before `web1`'s main book.
`book_two.sh` receives no special `$0` or `return` treatment beyond ordinary dot
sourcing.

Multiple target-qualified main books are separate programs. V0 does not define
the planning, approval, failure, or freshness semantics of running two main
programs on one target; it never treats that spelling as source composition.
The explicit `--pre-source` route is how a user asks for shared-shell merging.

Controller-local execution is one target context under the same representation,
not a separate loader species. More complex target shifts remain expressible in
the main book as ordinary sh and future context-entry/SSH analysis; target
qualification is orchestration-layer sugar over that user-visible capability.

All Dorc options remain before `--`. The strong spike-era direction is that any
tokens after `--` belong to the books rather than to Dorc, preserving CLI
namespace for compiler, analyzer, planner, transport, and orchestration growth.

There is no book that is THE book across targets - there is exactly one book per
target - so post-`--` arguments cannot be handed to "the" book. The shape that
follows from one argument list and N targets is that the same arguments reach
each target's book, positionally identical, evaluated ON that target. That is a
consequence, not a ruling: how they are carried, whether a target may vary them,
and what they do to a bundled or flattened artifact all remain future work.

### 2.5 The standard streams are collapsed resources

Stdin and stdout are first-class UI surfaces we expect to be used, not fallbacks.
They are also SINGULAR. Every mode wanting one must declare its claim, and two
claimants on one stream is a pre-network refusal naming both — never a silent
precedence rule.

Stream ROLES are per-subcommand. What follows rules `dorc plan` and
`dorc bundle`. `dorc apply` already spends stdin on the round-trip (`--plan`
defaults to it) and will grow its own claimants; the collapsed-resource principle
binds there too, but its per-flag consequences are not settled here.

`owed-no-flag-defaults-to-stdin` - two flags currently acquire stdin implicitly,
which is exactly what the rule above forbids: `plan --results` (probe records,
an INPUT, defaulting to stdin) and `apply --plan` (the rendered artifact it
ships, likewise). Declaring them as claimants is the interim; RETIRING the
defaults is the rule. Both then take `-` like any other filename, `-` becomes
stdin's only claimant, and `dorc plan -` stops needing `--results FILE` beside it
to free the stream — which is the papercut the interim leaves behind. Do both
together: they are one edit at one seat, and splitting them costs a second corpus
argv respell.

`rul-piped-stdout-implies-one-flat-plan` [TYPED] - when stdout is not an
interactive terminal at runtime, the invocation is single-stream intent by
construction. Emission takes `mode-fully-flattened-plan` (7.1 mode 1), and the
run is enforced single-plan-single-target, because one stream cannot carry two
targets' artifacts distinguishably. A request contradicting either half - explicit
multipart, or more than one `TargetedBook` - refuses before network rather than
silently returning a different form (7.1's standing rule). An interactive stdout
leaves auto mode free to choose: the artifact set publishes to disk while stdout
carries the render.

Detection is an edge act and non-hermetic. It is injected, never read ad hoc, so
deterministic tests drive both cells (`inv-determinism`). One corpus consequence
worth stating: the e2e rail captures stdout, so every existing case already sits
in the piped cell - which is why today's stdout-only artifact and this rule agree
rather than collide.

The plan render, diagnostics, and the why-lens are stderr and are untouched by
this rule; it governs the artifact stream alone.

`lean-every-detected-mode-is-also-requestable` [direction, not now] - detection
only ever picks among modes that are ALSO explicitly requestable. Git is the
model: a default that figures out fast-forward-versus-merge, alongside explicit
`--ff-only` and `--no-ff` that fail fast instead of guessing. Mirror that shape
as modes accrue; nothing is built for it now.

`rul-dash-is-stdin-in-any-filename-position` [TYPED direction] - `-` is a
generic tool in the I/O toolkit, never a per-subcommand special case: wherever
this document takes a filename, `-` in that position names stdin. All of
`dorc plan -`, `dorc plan web1:-`, and
`dorc plan web1:book.sh --pre-source -` are therefore ordinary, consistent
spellings. Where the admin finds stdin useful is the admin's call; the tool's job
is to accept it in filename position everywhere and nowhere else. No flag
acquires stdin implicitly, and structured stdin meaning beyond this is
deliberately unclaimed rather than reserved.

Generality sharpens the collapsed-resource rule rather than relaxing it: stdin is
still ONE stream, so at most one `-` may appear in an invocation. Two - a `-` book
beside a `--pre-source -`, or a `-` book beside a stdin-defaulted `--plan` -
refuses before network and names both claimants, rather than ranking them. A file
literally named `-` is spelled `./-`, per the same convention.

A `-` book is that target's book (`rul-one-main-book-per-target`): not
dot-sourced, no special `$0` or `return` treatment. It has no path, which is a
clean second witness for `rul-dot-resolves-as-sh` - under the rejected
sourcing-file-relative rule a stdin book's own `.` operands would resolve
nowhere at all, while under working-directory parity they resolve against the
controller load cwd exactly as any other book's do. The operand's spelling never
becomes a load root.

## 3. One static load model

### 3.1 Load context

The load evaluator is a pure function of an explicit snapshot. Exact type and
crate placement are builder decisions; the conceptual inputs are:

```text
StaticLoadSnapshot {
   controller-authored source bytes,
   source identities and content digests,
   controller load working directory,
   source-literal value flow,
   definition frames,
   load decisions and their source spans,
}
```

The I/O edge reads files once. Analysis, custody, bundling, and plan emission
consume those same immutable bytes. A local file race between analysis and
emission is structurally excluded rather than detected after producing a plan.

The controller load cwd is not the target execution cwd and neither is the plan
artifact directory. The first resolves controller-side source dependencies. The
second governs relative book commands on a particular target. The third locates
generated plan dependencies. Implementations may prove relationships among them;
they must not collapse the three coordinates by representation.

### 3.2 Working-directory parity

`rul-dot-resolves-as-sh` [TYPED direction] - a supported dot operand resolves
exactly as the modeled floor shell would:

- an absolute slash-bearing operand names that path;
- a relative slash-bearing operand resolves against cwd at that load position;
- a slashless operand requests PATH search and remains unsupported at v0;
- an unknown cwd or operand yields an unresolvable load, never a guessed file.

CLI path handling may resolve an authored argument or source expression to exact
snapshot bytes using more information than the literal token alone. That is file
identification, not altered shell semantics: once selected, every pre-source and
authored `.` keeps ordinary status, scope, order, and environment behavior.

`rul-cli-file-resolution-is-ours` [TYPED] - CLI *file* resolution is Dorc's to
design and rule; it is not ceded to shell source-resolution semantics. It reaches
for standard, unsurprising CLI behaviour. Turning a resolved, extant filename into
a `.`-style include in the internal representation is then an EXPLICIT mapping
function, not an identity. Its detailed semantics are unruled; the separation is
not. Authored `.` operands inside a book or oracle are the other side of this
line and stay pure sh (`rul-dot-resolves-as-sh`).

The landed sourcing-file-relative rule is rejected. It gives the same authored
line a different referent under Dorc and stock sh, breaks regional re-sourcing,
and cannot be preserved by pure strip-erasure.

Marked oracle top level cannot mutate cwd in the v0 profile, so its ambient
dependency graph starts from one constant invocation cwd. Full book cwd flow is
still owed where a book mutates cwd before a load. Emission should not add a
plan-root variable merely by default: analysis first proves whether the generated
source points remain relative to the artifact root, can use a statically computed
relative path, require a collision-safe captured root, or must flatten.

### 3.3 Visibility and custody are separate answers

A book source operation changes which definitions are live, but contributes no
speaker edge. A marked file's admitted source operation can contribute a
speaker-minting containment edge. The guarded-source form in section 3.4 is an
authored acceptance edge for vouch licensure only; it never makes its fallback
unconditionally visible. CLI co-loading remains ingestion only.

The same `DefinitionId` resolution answers analysis and emission. A bundle may
copy definitions into another physical file without changing their conceptual
source identity or custody in the generating run.

### 3.4 Cross-author dependency authority and narrative

Four cases must not collapse:

1. `dependency-explicitly-sourced` - the voucher's marked closure sources the
   resolved dependency; it may rest on that dependency under custody.
2. `dependency-guarded-source-exact` - a recognized include guard names a
   contracted fallback source. On the no-source/reuse route, the exact target
   closure must independently supply both (a) the value that decisively selected
   that route and (b) every transitively load-bearing helper on the reached
   vouching path; both are `Must` questions. On the source route, the explicit
   exact `.` supplies ordinary custody and the resulting helper definitions must
   resolve inside that closure. Then the whole guard mints the same speaker edge
   as a direct source for this license, even when another package loaded the
   exact target first. Declining paths need prove nothing.
3. `dependency-source-act-present-but-unaligned` - the voucher carries a
   supported source-bearing act naming a dependency which declares the reached
   helper, but the decisive value, live binding, or reached helper does not align
   with that exact closure. The act is useful narrative and licenses nothing.
4. `dependency-ambient-or-untraceable` - ordinary shell name resolution supplies
   a helper whose relationship to the voucher is not statically attributable as
   intended speech. This has no semantic distinction from any other ambient-world
   command the author called; it licenses nothing and remains legal sh.

The second case changes speaker custody only; normal branch-sensitive loading
still decides visibility. A same-valued variable, same-named function, file, or
byte sequence is not the proof: the guard value and frame-live helper
composition must trace to the exact target closure.

Read the second case as RECOGNITION, never as a licensing widening. The idiom is
a method, spelled in sh, by which an author says "reuse this exact package when
its own load value says it is present; otherwise source it." The same author then
names the helper on a reached vouching path, and Dorc supplies only the missing
symbol-resolution link: both the load value and invoked helper came from that
package. This is the authored-intent basis for treating its closure as the
author's accepted speech. A variable value alone, a helper name alone, or
co-loading licenses nothing. Whatever the engine cannot align exactly withholds
vouch, licensure, and speaker status under the existing collapse accounting.
This is an attribution boundary: accepting either value or helper from another
unit would make Dorc blame the voucher for a judgment they never selected or
reviewed, the pope-sin direction.
The door stays deliberately narrow; the structurally-sound wide version is later
work.

`rul-ambient-dependencies-are-ordinary-shell` [TYPED 2026-08-19] - caller-loaded
dependencies, callbacks, logging hooks, foundational helpers, intentional
patching, and unknown commands all inhabit the same ambient ops world. Learning
that an otherwise-unmodeled command resolves to a function in another loaded
file MUST NOT turn accepted sh into a pre-network refusal. Authors already owe
defensiveness against ambient command resolution, PATH change, and version
change; Dorc should expose what it can see without pretending that visibility
creates a new correctness category.

`rul-cross-custody-distinction-is-narrative` [TYPED 2026-08-19] - cases 3 and 4
both suspend vouch composition under `rul-vouch-reaches-own-custody-only`; the
book site runs and unrelated planning continues. Their distinction remains
load-bearing in the aid plane: case 3 says the author's selected dependency and
the live binding failed to align; case 4 says the helper came from ambient shell
resolution and no attributable dependency selection was available. Name the
call and live definition, state the lost capability, and offer explicit sourcing
or a guarded fallback as ways to RECOVER vouch composition - never as admission
requirements. `command` is suggested only when the author genuinely intends to
bypass functions and invoke an external utility. The distinction is
decision-inert: neither narrative may mint custody, a vouch, vocabulary, or any
other authority. Preserve the whole-unit distinction for lint and pull; a plan
pushes it only where the lost composition affects a reached site or another
goal-derived selection rule asks for it.

The future value-forfeit is therefore authority only, not shell expressiveness:
ambient callback/plugin injection works as sh but cannot compose a cross-file
vouch until an attributable mechanism is designed
(`FORFEITS:forfeit-ambient-dependency-vouch-composition`;
`pin-ambient-dependency-vouch-composition`). The current exit-16 refusal and its
whole-unit control-flow consequence are implementation drift; remove them and
retain the detection operands for narrative.

## 4. Scope sets

### 4.1 Force now as architecture specimens

These are intentionally mechanism-forcing rather than the easiest syntax:

- `force-root-value-flow` - `SM_ORACLE_ROOT` flows from book assignment through
  nested oracle source operands.
- `force-guarded-fallback` - one literal variable-value/fallback-source include
  guard is modeled identically by constant propagation, frames, custody, and
  emission; the fallback target supplies both the tested value and reached
  helper.
- `force-shared-diamond` - two independent entrypoints share one transitive
  dependency without merging speakers.
- `force-subshell-loading` - region-local source and definitions die at the
  closing parenthesis.
- `force-errexit-differential` - a sourced fallback under `||` is measured under
  both floor shells and the generated form agrees.
- `force-bundle-projection` - a book load becomes a contracted bundle at the same
  plan position.
- `force-provenance-roundtrip` - one diagnostic maps plan -> bundle -> transitive
  original source and falls back honestly when the origin is absent or changed.

### 4.2 Owed before real users, deliberately not perfected in this spike

- equivalent healthy include-guard spellings;
- the full meaningful `command -v` load model across the promised shell floor;
- early-return include guards and their source-boundary lowering;
- callback/caller-provided cross-custody VOUCH COMPOSITION (the dependencies
  themselves are ordinary supported sh and already run conservatively);
- relative loading after all supported cwd mutations;
- robust physical/logical path-identity policy;
- PATH-based dot search under an explicit modeled PATH;
- book source boundaries using `return`, caller-loop control, or other forms the
  v0 dumb inliner cannot preserve;
- broader bundle-source-map consumers and nested pre-bundled recovery.

These are named debts, not permission to publish a language that forces oracle
authors into awkward alternatives. They are deferred because the spike first
needs to validate the shared machinery.

### 4.3 Permanently excluded from this static architecture

- `eval`-generated source paths;
- host/probe output deciding the source set or load order;
- executing downloaded streams to discover definitions;
- arbitrary load-time commands or network interrogation;
- engine-injected runtime oracle roots;
- bundle comments affecting source identity, custody, definitions, or licenses;
- guessed package identity from directory or function names;
- silent fallback from an unresolved load.

### 4.4 Later acquisition, not dynamic loading

Network-sourced oracle packages remain possible future work. Acquisition must
finish before analysis and produce a fixed local source snapshot before invoking
the ordinary static loader. Its acquisition, identity, update, and retention
model is separate future design. The network never participates in load-order
evaluation.

## 5. Load-decision inputs and influence

### 5.1 No prospective dynamic seam

`rul-load-decisions-are-authored-before-contact` [TYPED direction] - every v0
load decision is stamped `authored-before-contact` by construction. The loader
accepts no influenced value. It still records a complete `SpineLoadDecision`
with operands, selected edge, definitions, and provenance.

Any proposal whose source set, path, include-guard result, or definition
environment depends on bytes outside the authored snapshot reopens the load
architecture, influence taxonomy, integrity policy, and authority review. It is
not an additive feature. Pre-building a generic influenced-loader input would
make this forbidden state representable.

Future acquisition may require a provenance/influence distinction not present in
v0. V0 does not guess that taxonomy.

## 6. Bundle projection

### 6.1 One semantic input

Conceptually:

```text
StaticLoadSnapshot
   |-- analysis and custody
   |-- probe/apply closure emission
   `-- BundleProjection
          |-- explicit dorc bundle output
          |-- multipart plan dependencies
          `-- fully flattened plan placement
```

`rul-one-load-account-separate-projections` [PROPOSED mechanism 2026-08-19] - the
loader resolves each supported load occurrence once and preserves enough of that
occurrence (sourcer, target, load locus, and positional context) for consumers to
derive three non-interchangeable projections:

1. **possible-load projection** - every statically possible resolved occurrence,
   including an undecided guard's fallback branch; bundle materialization consumes
   this conservative union so an artifact never omits a file runtime sh may load;
2. **speaker projection** - only source/guard edges whose exact custody proof
   succeeded; vouch composition and every other authority consumer see only this
   narrower relation;
3. **narrative projection** - the source act, intended target where attributable,
   and live binding needed to distinguish section 3.4's exact, unaligned, and
   ambient cases. It is decision-inert.

One loader does not mean one overloaded edge set. No projection re-parses source
text or re-resolves a target, and absence from the speaker projection never means
absence from the possible-load or narrative projections. Concrete representation
remains builder latitude; a target-only pair that collapses distinct load
occurrences is insufficient for bundle keying and locator composition.

The projection copies exact authored ranges and adds only necessary shell
scaffolding and generated comments. It does not alpha-rename authored code,
globally reorder, semantically deduplicate, or partially evaluate runtime guard
choices. Such transformations would enlarge both correctness and source-map
obligations without serving v0.

### 6.2 One root bundle per static load occurrence

`rul-bundles-key-to-load-occurrences` [TYPED direction] - separate textual load
points produce separate root bundle projections by default, even when they name
the same entrypoint. Their frames, scopes, variable values, and conditional
positions may differ. Content deduplication is a later optional optimization.

The v0 aspiration is one physical root bundle file per load occurrence, not one
file per source dependency. This is possible only where lowering preserves each
nested dot boundary. Multiple generated dependency files are a conservative
fallback, not the target.

### 6.3 Why naive text inlining fails

A sourced child may use top-level `return`:

```sh
[ "${SM_COMMON_LOADED-}" = 1 ] && return 0
SM_COMMON_LOADED=1
```

Textually pasting that child into its parent makes `return` exit the parent dot
script and can omit everything after the source point. A subshell is not a fix:
every variable and function definition loaded inside it dies at the closing
parenthesis.

`fnd-loader-function-errexit-diverges` [MEASURED 2026-08-18] - generated loader
functions are not a universal one-file lowering. The opt-in floor specimen
`floor30-dot-loader-function-errexit` asks both floor shells about a dot-sourced
child and the same body in a function, under `set -e` and an enclosing `||`.
Both shells agree: the failing child aborts its dot boundary (`dot boundary=1`),
while the function call remains errexit-exempt, returns 1 to its caller, and
continues through the second invocation. The observable output differs.

A subshell cannot repair this because loaded assignments and definitions die at
`)`. V0 therefore preserves nested source boundaries as generated files for this
healthy form. One-file-per-root remains an optimization pin for a different,
floor-proven lowering; builders must not revive loader functions by argument.
Top-level `local`, `set --`, `shift`, traps, aliases, and similar
boundary-sensitive forms remain outside any v0 one-file lowering.

### 6.4 Bundle comments

The accepted v0 direction is a generated, versioned, human-readable boundary:

```sh
# dorc-bundle/v0: begin source=org.example.common/entry.oracle.sh
...
# dorc-bundle/v0: end source=org.example.common/entry.oracle.sh
```

The visible source locator is package- or invocation-relative for deterministic
output where possible; the in-memory locator graph carries the precise binding.
The exact grammar and API belong to builders and may be simple at v0.

## 7. Plan emission and attention

### 7.1 Three effective modes

Names and CLI spellings are builder latitude. The semantic modes are:

1. `mode-fully-flattened-plan` - one `plan.sh`, used by explicit single-stream
   intent and transports unable to materialize directories.
2. `mode-plan-with-oracle-bundles` - book closure is represented on `plan.sh`;
   each contracted dorc-lang root is replaced by a source of a generated bundle
   under a small dependency directory. This is the intended attention-preserving
   default.
3. `mode-preserved-book-tree` - when v0 cannot safely inline a sourced non-dorc-lang
   book boundary, more book files remain in the emitted artifact set rather than
   being miscompiled.

Auto mode aims for mode 2 and falls toward mode 3 with an explicit explanation.
An explicit `--flatten-plan`-shaped request, redirected single-stream output, or
single-byte-pipe transport requests mode 1 and fails before network if a supported
construct cannot yet be flattened safely. The user asked for a form; silently
returning another is wrong.

### 7.2 Book bytes stay sacred

Non-dorc-lang source material is not CFG-flattened, reordered, normalized, or
rewritten for elegance. The only attempted transform is bog-standard source
inclusion at the source position, followed by the ordinary per-line Dorc plan
decisions over those same authored bytes. If source-boundary semantics prevent
safe dumb inclusion, mode 3 preserves the file boundary.

All possibly mutative book commands must be reviewable from the primary
`plan.sh` surface as written and in authored order. Exact treatment of complex
preserved book files is an owed render/emission problem; v0 must not claim that
naive textual inclusion handles top-level `return`, caller-loop control, aliases,
or stateful shell builtins.

### 7.3 Contracted dependency surface

The CLI does not enumerate every bundle in ordinary output. The dependency
directory is the complete deeper review surface for a user who declines to trust
oracle authors' load-safety/read-only contract; that population is outside Dorc's
ordinary attention offer. `plan.sh` visibly names every bundle at its original
source point, and a concise summary may name the artifact root and dependency
count.

The standing promise is exact: trusting oracle authors lets the admin focus on
`plan.sh`; Dorc has established only contract eligibility and structural loading,
never semantic purity of function bodies.

### 7.4 Cwd-sensitive generated paths

The planner first uses cwd analysis to keep output beautiful:

- if a load point is proven to retain artifact-root cwd, emit a simple relative
  generated source;
- if a modeled cwd has a known relation to the artifact root, emit the corresponding
  relative path;
- only if necessary, capture a collision-safe artifact root before book execution;
- if the capture cannot be protected from the book, flatten that load or choose a
  safer emission mode.

The default must not add a generated plan-root assignment merely because a future
book might `cd`. `rul-happy-path-is-a-closed-set` applies in both directions:
idiomatic minimal output requires proof, and ugly scaffolding requires demonstrated
need.

Multipart artifact execution begins in the emitted artifact directory. The
transport owns that initial cwd; direct use is `cd <artifact> && sh ./plan.sh`.

### 7.5 Artifact publication is atomic

Multipart emission constructs a fresh controller-owned artifact directory,
writes every plan and bundle, and only then publishes the result. A plan may
never point at a sidecar from an earlier generation. Exact naming,
content-derived suffixes, and filesystem API are builder decisions.

### 7.6 Targeted plans share loading, not execution identity

Each `TargetedBook` produces a separate main program/plan. Distinct targets may
share one static load snapshot when their main book, ordered pre-sources,
controller load cwd, and analysis-relevant loading policy are identical. Host
measurements never select or reorder pre-sources.

Per-target differences may change dispositions, plan reasons, target execution
cwd, and multipart-versus-flat placement. A controller-authored mapping may
select different main books or pre-source lists for different targets, producing
different static snapshots before contact.

The artifact/executor must preserve the target's execution cwd independently of
where bundles are staged. Changing cwd merely to find plan dependencies changes
ordinary relative book commands and is not a loading convenience.

## 8. TUI and richer presentation

The TUI presents a projection of the already-final Plan and artifact set. It
does not compile after consent.

The normal compressed row is approximately:

```sh
# . "$SM_ORACLE_ROOT/org.example.wombat/entry.oracle.sh"  # bundled for apply
```

Helper and function plumbing may collapse behind that row. Constant-propagated
value flow that materially affects the visible book remains on the rendered plan
at its use or contribution point. The user can expand from the row to bundle
bytes and then through original source locators.

The CLI remains valuable precisely because it interposes no presentation gloss:
the emitted `plan.sh` and dependency tree are the artifact the experienced admin
reviews and runs.

## 9. Provenance and source maps

### 9.1 Minimal compiler, maximal consumer

The v0 compiler aims at exact contiguous copied segments plus generated boundary
or scaffolding segments. Language forms that cannot be mapped under that model may
be refused at v0 rather than forcing a sophisticated transformation/source-map
format.

The internal locator implementation is nevertheless an arbitrary DAG, not a
hard-coded generated/original pair:

```text
book source span
   -> planned source replacement
   -> bundle load span
   -> bundle segment
   -> nested bundle segment
   -> original oracle span
```

Future chains may include published bundle -> local rebundle -> plan sidecar ->
fully flattened artifact, with multiple origins. Every resolvable locator remains
available; a concise render may emphasize the editable source without destroying
the generated loci.

### 9.2 Force-now recovery behavior

At least one architecture specimen must prove:

1. a book load maps to the generated source line;
2. a sidecar function maps through a transitive dependency to its original file
   and exact span;
3. an error names both generated and original locations when original bytes are
   present and content-matching;
4. absent original bytes fall back honestly to the bundle;
5. changed original bytes are shown as mismatch, never current source;
6. a later flattening stage composes another edge instead of replacing history.

No separate v0 source-map file and no per-line source markers are required. The
in-memory generating run has authoritative source snapshots and mappings. The
versioned bundle comments recover only the dumb/pretty format's representable
subset on a later read.

### 9.3 BundleOriginClaim is sealed to aid

Reading bundle comments creates `BundleOriginClaim` in the aid/narrative plane.
There is no conversion to source bytes, `DefinitionId`, custody, dialect,
vouches, facts, or license inputs. Removing or editing every comment leaves the
analytic answer byte-identical.

A claim becomes a resolved original locator only when matching source bytes are
available and content identity agrees. Otherwise the bundle locus is primary
and the claimed origin remains explicitly claimed.

Future pull-only aid may resolve a claim to a web repository revision and accept
it only when the retrieved source identity agrees. Repository guesses are
conjecture. None of that runs in planning or affects decisions.

## 10. Fail-fast and diagnostics

All invalid static load worlds are detected before network contact, but analysis
continues long enough to batch unrelated root causes.

At minimum, diagnostics distinguish:

- unknown cwd or source operand;
- absent/unreadable source;
- marked target violating its load contract;
- dynamic loading outside the architecture;
- source cycle or boundary form the selected emission mode cannot preserve;
- explicit flatten intent the v0 compiler cannot satisfy;
- stale or mismatched bundle-origin source during aid resolution.

Cross-custody dependency outcomes are not members of that invalid-load list.
They narrate the three useful states separately: exact speaker composition;
source act present but binding/guard alignment failed; ambient or untraceable
dependency. The latter two suspend the vouch and preserve the runnable plan.

Human-facing wording remains loom-authored under `error-authorship-tier`; builders
mint structure and defining cases with unwritten prose.

## 11. Build now

The near-term implementation should be sized to force the architecture and make
real oracle packages representable, not finish every spelling:

1. Re-cut source resolution to modeled-cwd parity and freeze source bytes once.
2. Thread known load-variable value flow through one guarded transitive dependency
   specimen.
3. Make book source affect visibility while preserving book-no-speaker custody.
4. Represent load decisions on Spine with fixed authored-before-contact influence.
5. Build one exact bundle projection shared by analysis/emission entrypoints.
6. Exercise one-bundle-per-load-occurrence, preserving nested dot boundaries as
   files where required; keep one-file-per-root as an optimization pin.
7. Provide multipart-plan and flattening seams; APIs/names are builder latitude.
8. Carry one complete multi-stage locator mapping into an actual error surface.
9. Differential-test the `||`/errexit source boundary under both floor shells.
10. Keep all other target behavior as named xfail specimens with round horizons.

## 12. Do not build in this phase

- no package registry, manifest, or engine-recognized root variable;
- no network acquisition or web source lookup;
- no host- or probe-dependent loading;
- no PATH search;
- no arbitrary top-level shell evaluator;
- no dynamic loops, eval, globbed dependencies, or command-substitution paths;
- no broad callback/bare cross-custody VOUCH COMPOSITION; ambient calls remain
  legal and narratable;
- no global bundle deduplication or optimization;
- no second resolver in `dorc bundle`;
- no durable/whylog schema expansion merely for bundles;
- no compatibility aliases for `dorc-bundle/v0` before publication;
- no per-line bundle markers or separate source-map format;
- no TUI implementation merely to demonstrate its projection;
- no attempt to make every arbitrary book source boundary flattenable.
- no short CLI options for any spike feature;
- no implicit merging of multiple main-book operands; use authored sourcing or
  explicit `--pre-source` semantics;
- no final host-qualifier syntax or same-target multi-program orchestration.

## 13. Specimen matrix

The pre-build xfail suite should be small and compound. It is an executable
product specification, not the final test allocation. Builders green behavior,
then lower properties into fast Rust-native ownership-seat tests and retain only
whole-artifact e2es whose composition adds coverage.

Required worlds:

1. `specimen-rooted-shared-dependency` - one book root; two marked entrypoints;
   one exact package sentinel populated by the shared dependency; value reaches
   both entrypoint guards and a real book command. Custody's no-speaker-merge
   half is already pinned natively by `core::custody`.
2. `specimen-subshell-and-errexit` - regional source; variable-sentinel fallback
   under `||`; both floor-shell answers; generated artifact preserves status,
   scope, and run set. The committed floor cell already refutes generated loader
   functions.
3. `specimen-emission-modes` - two distinct book load points; default contracted
   dependencies; explicit full flatten target; original book source lines visible
   and replaced in place.
4. `specimen-provenance-through-bundle` - transitive helper failure maps through
   generated bundle to original source, with present/absent/mismatched origin cells.
5. `specimen-ambient-dependency-narrates` - a bare cross-custody function call
   preserves the runnable plan while losing vouch composition; its explicit-guard
   sibling composes. A second cell distinguishes source-act-present-but-unaligned
   from ambient/untraceable without changing either disposition.
6. `specimen-speaker-minting-is-observable` - ONE multipurpose e2e, because
   speaker-minting bears on licensure and elision and must be observed SOMEWHERE
   as behaviour. Today's `load30-*` cannot see it: their `probe-results.txt` are
   empty, so nothing is converged, nothing elides, and every run set is identical
   under every reading. Give the shared dependency's helper a CONVERGED verdict so
   the license has a run-set consequence — the vouching site is absent from
   `expected.ran` when it mints and present when it does not — and carry the
   counterfactuals in the same case: a same-valued load variable from the wrong
   source unit and a same-named helper from the wrong source unit must each NOT
   mint, and the site runs. The builder additionally MUTATES the mint (removes
   it) and confirms the case reddens, so the case is known to observe the
   machinery rather than to pass for unrelated reasons.
   Everything finer-grained — which arm resolved, which grade, which narrative
   fires on each way the recognition declines — is unit/DST-sized and belongs at
   its ownership seat, where exactly one thing reddens per break.

The current harness may not express every artifact/API claim. An unspellable
property remains a registered/reserved pin or a document assertion until the
first implementation seat exists; tests must never invent a production API merely
to make a future assertion compile. The three `load30-*` e2e XFAILs assert target
run sets for source/value/guard/frame behavior; they deliberately do not claim to
pin bundle file layout, custody, fail-fast exit policy, or locator APIs. Those
properties stay in this matrix until their first honest executable seat exists.

## 14. Builder latitude and open pins

Builders own:

- concrete type and crate names;
- CLI names such as `--flatten-plan`, the exact `dorc bundle` interface, and the
  eventual target-qualifier grammar; `--pre-source` and the no-short-option rule
  are not builder latitude;
- output-directory and dependency-file naming;
- any replacement candidate for the floor-refuted loader-function lowering;
- exact v0 bundle-comment grammar;
- grouping of specimens into fast tests versus retained e2es;
- the implementation of atomic publication and transport capability requests.

They do not own these questions:

- cwd parity versus sourcing-file-relative resolution;
- static-only versus host-influenced loading;
- books minting speaker custody;
- separate loader/bundler semantics;
- bundle metadata licensing analysis;
- whether provenance is rich/early;
- whether explicit single-stream intent may silently return multipart output;
- whether ambient cross-custody liveness may mint authority or refuse the plan
  (it may do neither; section 3.4);
- whether a second main-book operand may be interpreted as source composition;
- whether a spike feature may claim a single-letter option;
- whether `-o`/`--oracle` or multi-book concatenation survive this arc;
- which stream a mode may claim, and what a piped stdout implies.

Open pins that require human or implementation evidence:

1. `pin-ambient-dependency-vouch-composition` - which future attributable
   mechanisms, if any, let callback/caller-loaded dependencies compose a vouch
   without an explicit source. Ambient dependencies already remain legal and
   narrative at the conservative floor; the narrow exact guarded-source form in
   section 3.4 is already ruled.
2. `pin-one-file-root-bundle` - whether any lowering can preserve all forced v0
   dot semantics in one file; generated loader functions are measured-refuted.
3. `pin-complex-book-source-render` - how mutative commands from an unflattenable
   book source remain maximally reviewable on `plan.sh` while preserving a physical
   source boundary.
4. `pin-plan-root-without-scaffolding` - how far cwd analysis avoids a generated
   artifact-root variable before flattening becomes preferable.
5. `pin-bundle-map-v0-grammar` - the smallest embedded format that supports exact
   copied-segment recovery without becoming analytic authority.
6. `pin-command-v-load-model` - `command -v` remains an expected, meaningful
   dorc-lang route for asking what a shell resolves under a name. Settle which
   shell categories and floor/run-target variations the load model represents,
   and when that wider question can participate in exact guarded-source
   recognition. Output-slash classification is refuted as a total answer by
   `notes/30Ic`; the feature itself is not forfeited.

## 15. Ruling ledger

Human-typed or explicitly hard-acked in the design dialogue:

- static loading only; no Dorc runtime oracle magic;
- ordinary book-flow root/source acts, with `SM_ORACLE_ROOT` only a spike mnemonic;
- only dorc-lang files mint speaker-transitive closures;
- include guards are required healthy-library surface;
- a recognized guarded fallback source mints an exact-target speaker edge only
  when the no-source route's decisive guard value and every load-bearing helper
  on the reached vouch path independently Must-originate inside that target
  closure; names and values are ordinary author-owned sh, never Dorc vocabulary;
  broader shell-name dependency injection stays NYI as an explicit forfeit;
- `command -v` remains a meaningful supported dorc-lang route; the exact package
  sentinel is the immediate buildable guard, while the wider cross-shell load
  model stays an explicit open pin rather than being silently discarded;
- one loader/engine for analysis and bundling;
- multipart contracted bundles by default, full flatten when requested or required,
  less-flat book tree when correctness forces it;
- one root bundle per static load occurrence as the v0 aspiration;
- CLI does not enumerate every contracted dependency ordinarily;
- book commands appear on `plan.sh` as written, never CFG-flattened or reordered;
- source-map representation is rich early; bundle compilation stays simple;
- bundle-origin readback is aid-only by type;
- ambient cross-custody dependencies remain ordinary sh; exact custody gates
  vouch composition, while unaligned and ambient cases suspend and narrate with
  distinct repairs;
- provenance recovery belongs in the forcing set;
- network acquisition and web source recovery are later, not v0.
- one main command-file book per target, with separate target books denoting
  separate programs rather than implicit composition;
- repeated `--pre-source` inputs are ordered ordinary dot preludes, global when
  unqualified and per-target when qualified;
- the main book is never dot-sourced merely to add CLI preludes; its compiled
  `$0` is allowed to identify the generated plan;
- controller load cwd, per-target execution cwd, and artifact location remain
  distinct coordinates;
- zero short CLI flags exist for the duration of the spike; all single-letter
  allocation is reserved for post-spike whole-CLI design;
- `-o`/`--oracle` and multi-book concatenation are DEAD and are discharged in
  this arc. Both were accretion, never ruled; the CLI-input design above is what
  they are replaced by. Oracle packages arrive as `--pre-source`; multiple main
  books become separate targeted programs. Reds while the transition rips are
  expected and healthy; the final greening must land the replacement rather than
  preserve either;
- stdin and stdout are collapsed single resources (roles are per-subcommand;
  `plan`/`bundle` here): piped stdout implies one flat
  plan for one target, `-` names stdin in any filename position, and two
  claimants on one stream refuse before network (2.5).

Gently accepted or builder-latitude rather than hard-ratified:

- the three semantic emission modes and their exact names;
- one-file-per-root remains desired, but generated loader functions are refuted;
- the `# dorc-bundle/v0: begin/end` grammar particulars;
- automatic source-sidecar placement policy;
- exact artifact publication interface.
