# 30Pa - Security and hardening review findings for 30P

> Tier: **REVIEW FINDINGS, NOT RULED DESIGN.** This document is input to a later
> human-led design sitting. It has no authority to settle project direction, authorize
> implementation, amend `30P`, or override root documents, `spike/CLAUDE.md`, `KNOBS.md`,
> or human-typed rulings. A design-sitting conductor must present every directional choice
> to the human rather than silently treating a recommendation here as accepted.
>
> Quarantined security material. Do not quote, summarize, classify, or indirectly expose
> its security reasoning to an ordinary out-quarantine conductor. The deliberately filtered
> ordinary-engineering sibling is `Research/notes/30Pb-load-emission-review-findings.md`.
> The implementation-only handoff is `Research/notes/30Pc-load-emission-code-findings.md`.
>
> Review target: `Research/plans/30P-emission-planner-and-inclusion.md` through
> `a3e021bb`, including the 2026-08-22 rulings on set-valued operands, r30 plain-sh
> acquisition, the unwelded paste exclusion set, the emission name, and red-pinned
> forfeits. Confidence markers are `+SURE`, `~SUSPECT`, `-GUESS`, and `--WONDER`.

## 30Pa:review-disposition

+SURE The updated plan improved two review surfaces: the single-stream paste set is no
longer being prematurely frozen, and analysis/paste are deferred with executable red
specimens. Those changes remove the earlier objection that the current paste exclusions
were being mistaken for a complete semantic characterization.

+SURE The remaining critical risks sit one level deeper:

1. an engine-selected source candidate can be mistaken for authored source selection;
2. path resolution can be mistaken for authority to read and ship controller files;
3. code-motion legality can be reduced to function dominance while moved shell state
   changes earlier book behavior;
4. deterministic generated names can be mistaken for hygienically fresh bindings;
5. function-map havoc can be mistaken for full recovery after an unknown source; and
6. one decided plan can be mistaken for one consented executable artifact set.

These are design-review findings, not conclusions. Each section states the question the
human design sitting must settle. The implementation observations later in this document
are evidence and bug candidates; they do not decide the design question by themselves.

## Design-level review findings

### 30Pa:fnd-singleton-candidate-does-not-author-source - CRITICAL

**Review classification:** design-tier; human adjudication required.

+SURE Set-valued load analysis is a necessary precision mechanism. The unsafe step is
`30P:200-218`'s additional rule that a singleton suffix match *resolves* and the generated
plan re-says the import to that member. A singleton among files in Dorc's snapshot proves
only that Dorc presently holds one candidate. It does not prove that every runtime value of
the unknown head names that file, nor that the source author selected it.

```sh
# LIB is deliberately target- or environment-specific.
. "$LIB/shared.oracle.sh"
```

If the controller snapshot happens to contain only
`vendor/beta/shared.oracle.sh`, rewriting the import makes execution deterministic with
respect to Dorc's choice. It does not preserve the original program's choice. If that same
resolved occurrence feeds `speaker_edges`, the engine-selected Beta file can additionally
become speech attributed to the sourcer, allowing a vouch to compose through a dependency
the author did not select.

+SURE The newly added fence says set analysis must never become "fancy compilation output",
while the same paragraph still prescribes an import rewrite. Those two statements need a
design-level reconciliation.

**Strongest rebuttal:** the re-say makes runtime load exactly the bytes analysis used, so
there is no analysis/execution mismatch.

**Review response:** that establishes agreement between Dorc's analysis and Dorc's emitted
cousin. It does not establish agreement with the admin's original shell program or the
engineer's authored dependency choice.

**Question for the design sitting:** which, if any, additional witness promotes
`PossibleLoadSet::Singleton` into exact authored selection? Candidate postures are:

- singleton remains possible-load/narrative material only and never rewrites or mints
  custody;
- every possible runtime valuation is proved to resolve to content-identical bytes;
- the admin explicitly selects the candidate; or
- a separate engine-selected execution form is admitted but remains typed as
  engine-selected and authority-free.

### 30Pa:fnd-source-acquisition-needs-explicit-authority - CRITICAL

**Review classification:** design-tier security boundary; human adjudication required.

+SURE `30P:186-191`, `241-259`, and `277-300` distinguish controller-known values from
host reads, but they do not yet distinguish *path resolution* from *authority to read and
ship a controller object*. That distinction becomes immediate now that
`mech-acquire-and-ship-plain-sh` is ruled into r30.

```sh
# Commonly means target state when this book is pushed to a host.
. /etc/os-release

# Could mean a local companion, target-local state, or an ambient deployment file.
. ./optional.sh
```

A readable controller path is not automatically part of the admin's intended artifact.
Without a closed source capability, a third-party loaded source can nominate arbitrary
controller paths for attempted reads; an ordinary book source can accidentally bundle the
controller's copy of a path whose intended referent was target-local. Plain-sh acquisition
widens the consequence from controller availability to possible disclosure of controller
bytes into generated artifacts and managed hosts.

+SURE The plan needs an explicit authority source independent of path spelling. At minimum,
the design must separate:

- invocation-admitted controller source material;
- dependencies inside an admitted controller source tree;
- target-runtime source operations; and
- unresolved/unknown source operations.

The acquisition edge also needs bounded regular-file reads, a symlink/reparse policy,
finite file/count/depth budgets, and an opened-object identity that cannot be replaced by a
later pathname lookup. Exact mechanisms are implementation work; whether the controller
source root is implicit in the book invocation, explicit in CLI inputs, or another
sh-native signal is a human design decision.

**Strongest rebuttal:** `authored snapshot` already implies that only authored inputs can be
selected, while `/etc/os-release` is explicitly an unknown-source example.

**Review response:** if that is the intended boundary, it needs to be stated as a private
construction rule on the snapshot. The current filesystem acquisition functions will open
paths named by loaded source programs, and the new plain-sh slice cannot safely infer the
boundary from readability.

### 30Pa:fnd-hoist-legality-omits-shell-state - CRITICAL

**Review classification:** design-tier semantic and authority boundary; human adjudication
required.

+SURE `30P:50-104` frames placement legality around function-definition dominance, death at
the parenthesis, errexit, and whether an earlier book *call* names a bundle-bound name.
That is not a complete code-motion legality condition for sh.

```sh
command -v helper >/dev/null 2>&1 || install_helper
. ./oracle.sh                         # defines helper
```

Hoisting `helper` makes the earlier guard pass and suppresses `install_helper`, although no
earlier call invokes `helper` directly. Binding observations include `command -v`, `type`,
`unset -f`, dynamic command positions, and any other construct whose answer depends on
whether a name exists.

File-level assignments are a second independent problem:

```sh
[ "${WOMBAT_ROOT-unset}" = /etc/wombat ] && run_early
. ./wombat.oracle.sh                 # WOMBAT_ROOT=/etc/wombat
wombat sync a.conf
```

Moving the assignment above line one changes admin control flow. Alias state can change how
a moved function body is parsed. Helper functions, PATH, options, cwd, and other shell state
can change how the same authored body executes.

+SURE The planner therefore needs more than a definition-use graph. A movable emission unit
needs explicit read/write effects over the relevant shell environment, and hoisting needs a
noninterference proof for every crossed observer and mutation. In the absence of that proof,
the conservative forms are in-place, sink, or refusal.

**Strongest rebuttal:** dorc-lang top level is constrained and the planner uses a closed
condition table, so dangerous cases can simply become tier fallbacks.

**Review response:** that is a sound architecture only if the table's subject is the complete
shell-state effect model. A condition limited to calls and function names silently admits
assignment and binding-observer cases.

**Question for the design sitting:** are file-level assignments ever eligible to hoist? If
yes, what exact shell-state model proves the move? If no, does a definition depending on one
force in-place/sink placement of the whole closure?

### 30Pa:fnd-generated-name-needs-hygienic-freshness - CRITICAL

**Review classification:** design-tier naming/binding law; human adjudication required.

+SURE `30P:54-57` and `88-100` treat `name_h<digest>` as the defensive naming answer and
reserve alpha-renaming. A deterministic digest suffix does not by itself make a shell name
fresh. The generated name may collide with another generated name, an admin-authored name,
or a binding introduced by an unresolved source. Even a full collision-resistant digest
does not stop the book from defining the exact generated spelling.

Header-only renaming is safe only for a directly invoked role body when Dorc owns every
generated call site and the body/closure cannot observe or call the authored name. It is not
hygienic renaming of helpers: renaming a helper header without rewriting bound calls makes
the old name resolve elsewhere; leaving helper names untouched allows capture.

The current implementation demonstrates both hazards: it uses only eight SHA-256 hex
digits, does not detect two distinct bodies producing one emitted spelling, and checks book
ownership of the authored name rather than the generated name. That is evidence, not the
design decision.

**Strongest rebuttal:** the happy path is a proven closed set, and the digest is merely a
short local disambiguator whose uniqueness can be checked at emission.

**Review response:** the closed-set proof must explicitly include the *complete emitted
namespace* and every binding observation. If the namespace is open, munging is not a
conservative fallback; in-place placement or refusal is.

**Question for the design sitting:** should the planner mint a private `FreshEmittedName`
only after proving namespace closure and injectivity, lengthening/refusing on collision?
Which dependency classes may use header-only renaming, and which require alpha-renaming or
refusal?

### 30Pa:fnd-unknown-source-havoc-needs-domain-accounting - HIGH

**Review classification:** design-tier analysis law with a security-specific mutation
boundary; human adjudication required.

+SURE `30P:164-183` correctly rejects ShellCheck's "unknown source defines nothing"
posture. The proposed havoc remains too narrow if it is implemented as only "every function
binding becomes unknown" and a later role definition restores licensure.

An unknown source can change aliases, command functions, PATH, variables, cwd, options,
traps, descriptors, positional parameters, or control-flow termination. A later role
definition recovers that role's header; it does not recover the environment its body will
execute in.

```sh
. ./unknown-host-file.sh       # may define hork() or alter PATH
hork__is_converged() {
   hork status "$@"
}
hork apply
```

The probe executes only oracle bytes and may call the intended external `hork`. An apply
guard runs after the unknown source and may call the source-defined function. A running wall
does not contain this: Guard can still suppress the original mutation on a 0 answer.

Security-specific residue: when the unknown source is target-local and writable by a
less-privileged principal than the apply connection, the inserted guard becomes an
attacker-answerable switch in front of privileged mutation. Falling back from elision to
Guard or Run is a correctness ordering under uncertainty, not automatically a security
ordering under lost integrity. This intersects `306b`'s still-open refusal scope and must not
be silently settled by a load-plane builder.

**Question for the design sitting:** which shell-state domains does unknown source havoc,
what later acts recover each domain, and when does unresolved probe/apply resolution force
Run versus whole-target report-only refusal?

### 30Pa:fnd-executable-artifact-needs-bound-identity - HIGH

**Review classification:** design-tier approval and artifact boundary; existing hidden
invariant applies, human design still needed for the concrete schema.

+SURE `30P:101-104` correctly places planner decisions in `Plan::decided`, but one decided
render is not yet one approved executable object. The emission planner produces a set of
paths and bytes; multipart apply, saved review, and later execution must bind that complete
set to the decision the human reviewed.

The identity needs exact ordered book and oracle bytes/provenance, analysis-relevant policy,
controller semantics/version, target and execution context, generation, execution cwd, and
every executable artifact path and byte. Apply must reject mismatch rather than read or
regenerate a cousin. Freshness of the managed world remains a separate guard/observation
question.

The current spike publishes an `ArtifactSet` but discards its published generation path;
remote apply reads and ships one arbitrary file; the FNV decision digest covers the flat
probe/apply render but not the multipart sidecar set. Those are expected spike gaps, but
they demonstrate that `Plan::decided` alone does not reserve the production shape.

**Question for the design sitting:** does `30P` own the canonical artifact-set identity
projection, or does it explicitly hand that requirement to a separate saved-plan design
while reserving a non-optional identity field now?

### 30Pa:fnd-path-identity-cannot-be-lexical-canonical - HIGH

**Review classification:** design-tier load identity law; human adjudication required.

+SURE A source path needs at least three distinct identities: the authored spelling, the
shell-resolved logical target at a program point, and the exact opened content/object in the
controller snapshot. A lexical normal form is useful as syntax; it cannot stand in for all
three.

```sh
# current is a symlink to releases/v2
. ./current/../oracle.sh
```

POSIX pathname resolution follows `current` before applying `..`; lexical cancellation of
`current/..` can select a different file. Similar distinctions arise from hardlinks,
case-folding controllers, and symlink replacement. Treating a lexical key as canonical can
attribute a load, custody edge, or bundle segment to the wrong source.

The current `Cwd::normalize` explicitly removes `..` without filesystem resolution and the
snapshot uses that text as its source key. This is implementation evidence for the design
gap, not a request to put filesystem I/O into the kernel.

**Question for the design sitting:** what opened-object/content identity does the I/O edge
mint, and which consumers need authored spelling, logical shell resolution, or immutable
snapshot identity?

### 30Pa:fnd-glob-order-needs-whole-program-meet - HIGH, DEFERRED SURFACE

**Review classification:** design-tier future constraint; no immediate build direction.

+SURE `30P:225-230` correctly says glob order is unknown. The proposed refinement "members
defining one name with different bytes withhold; a sole-member name is live" is not
sufficient. Another glob member can `unset -f` that name, conditionally rebind it, alter a
constant it reads, change cwd/options, or terminate before/after it. Liveness is a property
of the ordered sequence of load programs, not a per-name file count.

If runtime order remains target-selected, the safe analysis is a universal meet over every
possible order and every shell-state effect relevant to the consumer. If Dorc emits one
chosen order, that is another semantics-changing re-say and needs the same authorship and
approval treatment as the singleton-source finding.

### 30Pa:fnd-source-is-execution-frame-not-text-paste - MEDIUM, DEFERRED SURFACE

**Review classification:** design vocabulary/IR warning; splice and paste are already
deferred.

~SUSPECT Calling ordinary `.` "textual inclusion" at `30P:241-253` invites the wrong
implementation. Dot executes another file in the current shell environment, but it is not
parser-level textual substitution: source return, errexit context, alias parsing, traps,
positional parameters, and shell state make the boundary observable.

The plan already preserves several of these distinctions and has deferred single-stream
paste behind an unwelded exclusion set. The review recommendation is therefore narrow: keep
a first-class source-execution frame in the IR and describe CFG splicing as an analysis
implementation, not as semantic identity with pasted text.

## Code-level and implementation observations

These observations do not settle the design findings above. Only the first is sufficiently
design-independent for direct builder handoff in `30Pc`.

### 30Pa:bug-assignment-bearing-dot-is-inlineable - VERIFIED, BUILDER-READY

+SURE `cli::artifact::book_loads` marks a top-level, redirect-free, whole-line `Simple`
source command absorbable without checking `assigns`. `ImportEdit::Inline` then replaces the
whole command node.

```sh
MODE=prod . ./entry.oracle.sh
```

Inlining removes `MODE=prod`, changing the sourced file's environment. This is outside the
measured `floor30-inline-dot-boundary` shape. The ruled semantics already supply the fix:
assignment-bearing source commands are not absorbable until separately measured and designed.
This is the sole `30Pc` item.

### 30Pa:bug-short-digest-has-no-collision-check - VERIFIED, DESIGN-COUPLED

+SURE `plan::short_digest` keeps eight SHA-256 hex digits, and `pin_definitions` does not
check that distinct bodies or book declarations produce distinct emitted names. The local
doc says uniqueness is checked at emission; the code does not perform that check.

This is not in `30Pc`: choosing deterministic extension, refusal, or another namespace
requires the human ruling requested by
`30Pa:fnd-generated-name-needs-hygienic-freshness`.

### 30Pa:bug-closure-assignments-hoist-before-book - VERIFIED, DESIGN-COUPLED

+SURE `HelperIndex` captures every top-level constant declaration from each contributing
file, and `pin_definitions` emits the snapshot before the book. The current path can therefore
change book control flow exactly as the hoist finding's assignment example shows.

This is not in `30Pc`: fixing it requires a decision about assignment placement and whether
a body depending on constants can hoist at all.

### 30Pa:bug-source-reads-are-unbounded-and-path-following - VERIFIED, DESIGN-COUPLED

+SURE `read_input`, `read_sourced_oracles`, and `read_book_sourced` use unbounded
`read_to_string`; transitive reads occur before target contract validation and carry no
regular-file, symlink/reparse, total-byte, count, or depth capability beyond the 32 solve
rounds. A named FIFO/device can block; a large file can exhaust memory.

This is not in `30Pc`: the limits and permitted source-root policy belong to the acquisition
boundary the design sitting must settle.

### 30Pa:bug-multipart-apply-ships-one-file - VERIFIED, DESIGN-COUPLED

+SURE `ArtifactSet` can contain dependencies, but `dorc apply --host --plan` reads one path
or stdin and ships only those bytes. A multipart `plan.sh` can therefore source absent or
pre-existing target files, or fail after earlier mutations. Atomic local publication does not
close this integration gap.

This is not in `30Pc`: the apply input must become the identified artifact set from the
artifact-identity design, not a local patch that guesses sidecars from a pathname.

### 30Pa:bug-decision-digest-is-not-artifact-identity - VERIFIED, EXPECTED FENCE

+SURE `decision_digest` is FNV-1a over the canonical flat probe/apply render. It omits the
multipart path/byte set and is explicitly fixture-grade under the existing security law.
Do not extend it piecemeal and call it the saved-plan identity; replace it at the production
re-entry boundary.

### 30Pa:bug-artifact-store-retains-only-pathnames - SUSPECT, SECURITY-ONLY

~SUSPECT `artifact_store` checks the named root once, creates a staging directory, and then
opens children and publishes by pathname. A principal able to replace entries in the root can
race the staging name between exclusive creation and later child opens/rename. The store owns
the name at creation, but not an ownership-bearing directory handle through the full sequence.

This is not in `30Pc`: a correct cross-platform repair likely requires handle-relative,
no-follow operations and a Windows reparse strategy. That mechanism deserves a narrow
implementation/security review rather than a builder improvisation.

## Filtering ledger

The out-quarantine `30Pb` carries only ordinary engineering goals that can be justified
without this document's threat reasoning:

- a possible singleton is not an exact source selection;
- controller-companion and target-runtime source roles must remain distinct;
- code motion must preserve all observable shell state;
- generated names must be fresh and references hygienic;
- unknown-source recovery is per shell-state domain;
- the reviewed and executed artifact sets must be identical;
- authored path, shell resolution, and snapshot identity are distinct;
- glob order is a whole-program meet; and
- dot sourcing remains a first-class execution boundary.

The design-tier security residue intentionally absent from `30Pb` is:

- controller source acquisition is a capability and confidentiality boundary, not only a
  semantic source-role distinction;
- an unknown target-local source can turn a generated apply guard into a privileged
  attacker-answerable switch, so Run/Guard demotion may not substitute for refusal;
- generated names and artifact IDs must withstand deliberate collision/substitution, not
  merely accidental duplicates; and
- approval identity must bind target, context, generation, and revocation scope with
  collision-resistant content identity.

## Prior-art disposition

+SURE This review does not recommend a broad security-only prior-art round before amending
the plan. The surviving concerns are applications of established semantic-preservation,
complete-mediation, capability-confinement, hygienic-renaming, collision-resistance, and
approval-to-execution principles to Dorc-specific authority flows.

~SUSPECT Narrow source work remains useful when the corresponding mechanisms are designed:
handle-relative filesystem acquisition/publication on Unix and Windows; saved multipart-plan
identity; and differential shell behavior for alias state, source return, assignments, `$0`,
and symlinked `..`. Those are implementation/design inputs, not prerequisites for recognizing
the findings above.
