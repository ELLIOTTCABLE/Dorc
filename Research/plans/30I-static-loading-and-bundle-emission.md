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
> `28Q:pin-emission-planner-universal` one concrete consumer. It does not build
> any of them.

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

if command -v example_common_query >/dev/null 2>&1; then
   :
else
   . "$SM_ORACLE_ROOT/org.example.common/entry.oracle.sh"
fi

alpha__is_converged() {
   example_common_query alpha "$1"
}
```

`rul-include-guards-are-load-semantics` [TYPED] - include guards are mandatory
language surface, not optional polish. Independent oracle authors use them to
share dependencies and to say: use the higher-quality live implementation when
present; otherwise load this fallback. The function environment, custody,
emission, and bundle compiler must agree on that branch.

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

## 3. One static load model

### 3.1 Load context

The load evaluator is a pure function of an explicit snapshot. Exact type and
crate placement are builder decisions; the conceptual inputs are:

```text
StaticLoadSnapshot {
   controller-authored source bytes,
   source identities and content digests,
   initial modeled working directory,
   source-literal value flow,
   definition frames,
   load decisions and their source spans,
}
```

The I/O edge reads files once. Analysis, custody, bundling, and plan emission
consume those same immutable bytes. A local file race between analysis and
emission is structurally excluded rather than detected after producing a plan.

### 3.2 Working-directory parity

`rul-dot-resolves-as-sh` [TYPED direction] - a supported dot operand resolves
exactly as the modeled floor shell would:

- an absolute slash-bearing operand names that path;
- a relative slash-bearing operand resolves against cwd at that load position;
- a slashless operand requests PATH search and remains unsupported at v0;
- an unknown cwd or operand yields an unresolvable load, never a guessed file.

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
speaker-minting containment edge. CLI co-loading remains ingestion only.

The same `DefinitionId` resolution answers analysis and emission. A bundle may
copy definitions into another physical file without changing their conceptual
source identity or custody in the generating run.

### 3.4 Cross-author dependency acceptance

Three cases must not collapse:

1. `dependency-explicitly-sourced` - the voucher's marked closure sources the
   resolved dependency; it may rest on that dependency under custody.
2. `dependency-explicitly-selected` - an authored guard accepts an already-live
   binding and loads a fallback otherwise; the guard is the attributable
   acceptance chain to the frame-resolved definition.
3. `dependency-merely-happened-to-be-live` - a voucher calls a cross-custody
   function without source, guard, or other recognized selection.

The third case intersects ordinary POSIX habits: caller-loaded dependencies,
callbacks, logging hooks, foundational helpers, and intentional patching. It is
not welded out. At v0 it is nevertheless invalid contracted oracle input because
Dorc cannot distinguish intended dependency injection from accidental function
shadowing.

`rul-unannounced-cross-custody-fails-before-network` [ACKED, gentle] - v0
continues static analysis far enough to batch unrelated root errors, names both
the call and live definition, suggests `command`, explicit sourcing/guarding, or
renaming, emits no mutation-authorizing plan, and contacts no host. This is
fail-fast on human timescales, not a panic or one-error abort.

## 4. Scope sets

### 4.1 Force now as architecture specimens

These are intentionally mechanism-forcing rather than the easiest syntax:

- `force-root-value-flow` - `SM_ORACLE_ROOT` flows from book assignment through
  nested oracle source operands.
- `force-guarded-fallback` - one `command -v`/fallback-source include guard is
  modeled identically by frames and emission.
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
- early-return include guards and their source-boundary lowering;
- callback/caller-provided bare cross-custody dependencies;
- relative loading after all supported cwd mutations;
- robust symlink and path-identity policy;
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
finish before analysis: fetch, verify immutable content/signature identity,
freeze a local snapshot, then invoke the ordinary static loader. Mutable tags,
rollback, equivocation, dependency confusion, archive traversal, cache trust,
and credential forwarding require a separate security design. The network never
participates in load-order evaluation.

## 5. Security and influence

### 5.1 V0 threat model

The relevant threats are:

- compromised or malicious oracle bytes on the controller;
- local file/symlink substitution between analysis and emission;
- dependency confusion through roots and load order;
- malicious or malformed books selecting unintended paths;
- hostile hosts tampering with transported artifacts;
- forged bundle-origin comments laundering another author's identity;
- Dorc selecting or attributing the wrong definition.

Static snapshotting, content identity, custody, controller-owned artifact
emission, and aid-plane distrust address those threats. Host influence does not:
a compromised local package is supply-chain input, and a filesystem race is a
snapshot-integrity problem.

### 5.2 No prospective dynamic seam

`rul-load-decisions-are-authored-before-contact` [TYPED direction] - every v0
load decision is stamped `authored-before-contact` by construction. The loader
accepts no influenced value. It still records a complete `SpineLoadDecision`
with operands, selected edge, definitions, and provenance.

Any proposal whose source set, path, include-guard result, or definition
environment depends on bytes outside the authored snapshot reopens the load
architecture, influence taxonomy, integrity policy, and authority review. It is
not an additive feature. Pre-building a generic influenced-loader input would
make this forbidden state representable.

Future network acquisition may require a new provenance/influence grade because
network-fetched-before-host-contact is neither honestly authored nor managed-host
reported. V0 does not guess that taxonomy.

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

`dir-loader-function-lowering` [PROPOSED] - a promising one-file lowering gives
each original source boundary a collision-safe generated loader function, calls
child loaders at the original dot positions, invokes the root loader with the
current positional parameters, then removes generated loader functions while
preserving the root status. Ordinary assignments and definitions escape function
calls in the floor language unless explicitly localized. Top-level `local`,
`set --`, `shift`, traps, aliases, and similar boundary-sensitive forms remain
outside this v0 lowering.

Whether function invocation and dot sourcing agree under errexit suppression is
a differential question, not an argument. A mismatch forces a multi-file bundle
for that form or a different lowering. Builders may choose another mechanism;
the property is source-boundary fidelity, not loader functions.

### 6.4 Bundle comments

The accepted v0 direction is a generated, versioned, human-readable boundary:

```sh
# dorc-bundle/v0: begin source=org.example.common/entry.oracle.sh
...
# dorc-bundle/v0: end source=org.example.common/entry.oracle.sh
```

Published artifacts should avoid absolute controller paths. The visible source
locator is package- or invocation-relative where possible; content identity and
the in-memory locator graph carry the precise binding. The exact grammar and API
belong to builders and may be simple at v0.

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
writes every plan and bundle, and only then publishes the result. It refuses
symlinks, traversal, and stale dependency reuse. A plan may never point at a
sidecar from an earlier generation. Exact naming, content-derived suffixes, and
filesystem API are builder decisions.

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
vouches, facts, or license inputs. Removing or forging every comment leaves the
analytic answer byte-identical.

A claim becomes a resolved original locator only when matching source bytes are
available and content identity verifies. Otherwise the bundle locus is primary
and the claimed origin remains explicitly claimed. This prevents modified code
from laundering itself into another author's name.

Future pull-only aid may resolve a claim to an immutable web repository revision,
fetch bounded source without forwarding credentials, and accept it only on digest
match. Repository guesses are conjecture. None of that runs in planning or affects
decisions.

## 10. Fail-fast and diagnostics

All invalid static load worlds are detected before network contact, but analysis
continues long enough to batch unrelated root causes.

At minimum, diagnostics distinguish:

- unknown cwd or source operand;
- absent/unreadable source;
- marked target violating its load contract;
- dynamic loading outside the architecture;
- unannounced cross-custody function binding;
- source cycle or boundary form the selected emission mode cannot preserve;
- explicit flatten intent the v0 compiler cannot satisfy;
- stale or mismatched bundle-origin source during aid resolution.

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
6. Exercise one-bundle-per-load-occurrence and the one-file-per-root aspiration.
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
- no broad callback/bare cross-custody acceptance;
- no global bundle deduplication or optimization;
- no second resolver in `dorc bundle`;
- no durable/whylog schema expansion merely for bundles;
- no compatibility aliases for `dorc-bundle/v0` before publication;
- no per-line bundle markers or separate source-map format;
- no TUI implementation merely to demonstrate its projection;
- no attempt to make every arbitrary book source boundary flattenable.

## 13. Specimen matrix

The pre-build xfail suite should be small and compound. It is an executable
product specification, not the final test allocation. Builders green behavior,
then lower properties into fast Rust-native ownership-seat tests and retain only
whole-artifact e2es whose composition adds coverage.

Required worlds:

1. `specimen-rooted-shared-dependency` - one book root; two marked entrypoints;
   guarded shared dependency; no speaker merge; value reaches a real book command.
2. `specimen-subshell-and-errexit` - regional source; fallback under `||`; both
   floor-shell answers; generated artifact preserves status, scope, and run set.
3. `specimen-emission-modes` - two distinct book load points; default contracted
   dependencies; explicit full flatten target; original book source lines visible
   and replaced in place.
4. `specimen-provenance-through-bundle` - transitive helper failure maps through
   generated bundle to original source, with present/absent/mismatched origin cells.
5. `specimen-unannounced-dependency-refuses` - bare cross-custody function call is
   a pre-network contracted-input error, while its explicit-guard sibling proceeds.

The current harness may not express every artifact/API claim. An unspellable
property remains a registered/reserved pin or a document assertion until the
first implementation seat exists; tests must never invent a production API merely
to make a future assertion compile.

## 14. Builder latitude and open pins

Builders own:

- concrete type and crate names;
- CLI names such as `--flatten-plan` and the exact `dorc bundle` interface;
- output-directory and dependency-file naming;
- whether loader-function lowering is viable;
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
- whether unsupported cross-custody dependencies may degrade quietly.

Open pins that require human or implementation evidence:

1. `pin-bare-dependency-injection` - which callback/caller-loaded idioms eventually
   license without an explicit source/guard.
2. `pin-one-file-root-bundle` - whether generated loader functions preserve all
   forced v0 dot semantics under both floor shells.
3. `pin-complex-book-source-render` - how mutative commands from an unflattenable
   book source remain maximally reviewable on `plan.sh` while preserving a physical
   source boundary.
4. `pin-plan-root-without-scaffolding` - how far cwd analysis avoids a generated
   artifact-root variable before flattening becomes preferable.
5. `pin-bundle-map-v0-grammar` - the smallest embedded format that supports exact
   copied-segment recovery without becoming analytic authority.

## 15. Ruling ledger

Human-typed or explicitly hard-acked in the design dialogue:

- static loading only; no Dorc runtime oracle magic;
- ordinary book-flow root/source acts, with `SM_ORACLE_ROOT` only a spike mnemonic;
- only dorc-lang files mint speaker-transitive closures;
- include guards are required healthy-library surface;
- one loader/engine for analysis and bundling;
- multipart contracted bundles by default, full flatten when requested or required,
  less-flat book tree when correctness forces it;
- one root bundle per static load occurrence as the v0 aspiration;
- CLI does not enumerate every contracted dependency ordinarily;
- book commands appear on `plan.sh` as written, never CFG-flattened or reordered;
- source-map representation is rich early; bundle compilation stays simple;
- bundle-origin readback is aid-only by type;
- unannounced cross-custody calls fail before network at v0 but are not permanently
  excluded;
- provenance recovery belongs in the forcing set;
- network acquisition and web source recovery are later, not v0.

Gently accepted or builder-latitude rather than hard-ratified:

- the three semantic emission modes and their exact names;
- generated loader functions as one-file bundle lowering;
- the `# dorc-bundle/v0: begin/end` grammar particulars;
- automatic source-sidecar placement policy;
- exact artifact publication interface.
