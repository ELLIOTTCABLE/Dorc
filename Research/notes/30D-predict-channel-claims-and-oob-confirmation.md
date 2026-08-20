# 30D - `__predict` channel claims and authored OOB confirmation

> Tier: LLM-authored design report from the 2026-08-19 human design dialogue.
> Human directions are marked **[TYPED]** where the human stated the rule or
> explicitly acked it. **[ACKED]** records accepted substance. **[PROPOSED]**
> marks implementation shape or unresolved detail. Root human documents and
> `spike/CLAUDE.md` outrank this report.
>
> Scope: replace `return 2`-as-decline in `cmd__predict()` with exact predicted
> status plus statically recognized, authored `printf >>DREP` channel speech;
> define the static/runtime split and transport-correctness obligation; record
> the outer-wrapper implementation direction; and preserve the alternatives and
> rejected routes that led here. This does not change the verdict-function
> `0 / 1 / >=2` partition.
> `plans/30L` is a later consumer, not an implementation seat: its universal route
> proofs consume the already-admitted per-channel view and must not rederive coverage,
> freeze today's defaults, or apply the verdict partition to predicted Status.
>
> Confidence markers: `+SURE` directly follows typed law or current design;
> `~SUSPECT` is a reasoned direction; `-GUESS` is weak; `--WONDER` is open.

## 0. The ruling in one screen

`rul-predict-status-keeps-every-value` **[TYPED/ACKED]** - A reached
`cmd__predict()` models the command's Status with the function's ordinary
aggregate exit status. Every shell status remains available. No status, including
2, means decline, unknown, unsupported, or cannot-predict. `return 2` in a
`cmd__predict()` therefore predicts status 2 unless a statically recognized DREP
channel override on the reached path says that Status is declined.

`rul-verdict-partition-stays-separate` **[TYPED/ACKED]** - The existing fixed
partition belongs to `cmd__is_converged()` and other verdict functions only:

```text
0     named judgment holds
1     named judgment's complement holds
>=2   cannot judge; run
```

Those functions answer a predicate. `cmd__predict()` reproduces a command's
observables. Sharing a reserved status between them was a category error.

`rul-predict-channel-defaults` **[TYPED/ACKED]** - Each predicted channel has
three authored states: `default`, `claimed`, and `declined`. At spike v0:

```text
Status   default = claimed
Stdout   default = declined
Stderr   default = declined
```

Status defaults claimed because every shell function has one aggregate status
and the `__predict` role names what that status means. Stdout and Stderr default
declined because complete stream modeling is difficult and partial output is easy
to claim accidentally. Effect and fact topology remain governed by their existing
marks and read-only contract; they are not added to this channel override menu.

`rul-predict-overrides-are-authored-shell` **[TYPED/ACKED]** - A channel override
is one fixed, statically recognized, actually executed `printf` into the DREP
sink. There is no `: predicts ...` mark for this purpose and no engine-inserted
statement inside the authored body. The line remains ordinary sh after Dorc is
abandoned; it is not stripped.

`rul-predict-runtime-confirms-static-analysis` **[TYPED/ACKED]** - Static
analysis determines the allowed channel state and proves the recognized DREP
speech's place in the admitted CFG. The runtime record does not choose or revise
that analysis. It confirms that execution reached the statically identified
speech act. Missing, malformed, duplicate, contradictory, stale, or
wrong-attempt confirmation is a separate prediction-execution/transport outcome,
never a reason to reveal another channel default.

`rul-predict-body-remains-literal` **[TYPED direction]** - The spike may compile
around role and helper functions with setup, teardown, capture, correlation,
helper trampolines, munging, and entry-style wrappers. It does not insert new
executable statements between the byte-literal statements of an authored
`__predict` body. Authored `set -e`, traps, returns, status clobbering, and early
exit retain their ordinary sh consequences.

`rul-drep-is-general-oracle-oob` **[TYPED correction]** - DREP is Dorc's primary
out-of-band oracle-to-controller mechanism. Particular aid records are
decision-inert; DREP as a physical and grammatical channel is not restricted to
aid. Prediction-control records enter through the standing bounded,
controller-attributed intake discipline.

## 1. The problem: `return 2` cannot do two jobs

The round-19 observable model stated the irreducible collision
(`19B` section 2; `19D` section 7; `STALENESS-AUDIT:owed-observable`): every
exit status may be meaningful output of the modeled command, so no exit status
can also mean unknown. Round 23 preserved the important half:
`23L:rul-role-split` kept a predict function's aggregate status as predicted
Status, while permitting OOB refusals and diagnostics.

Round 27 then inherited an incompatible convenience rule. The merge rider in
`271:rul-predict-absorbs-wrapper-modeling`, its full account in `273` section 2,
and later steering/docs said both:

```text
explicit return = predicted status claim
return 2        = whole-shape decline
```

The overlap is real. For a book such as:

```sh
thing
[ "$?" -eq 2 ] && do_other_thing
```

a faithful `thing__predict` must preserve status 2. Treating 2 as decline loses
the branch decision the model exists to reproduce. The problem is not confined
to explicit `return 2`: a delegated read-only tool may naturally return 2, and a
model may assert stdout before returning 2.

The correction is categorical rather than a larger reserved range. Predicted
Status stays in the shell status channel. Model coverage and channel declines
travel through DREP.

## 2. The channel state algebra

### 2.1 Per-channel states

For each of Status, Stdout, and Stderr:

```text
default    no explicit override on the selected authored path
claimed    the author explicitly claims this channel
declined   the author explicitly withholds this channel
```

Defaults collapse only at the channel-policy consumer. The source-level state is
kept three-way so aid can distinguish omission from deliberate refusal and so a
future `__can_predict()` may affect only absent/default state.

### 2.2 Channel vocabulary

`rul-predict-channel-token-set` **[TYPED/ACKED]** - The complete v0 authored
token set is:

```text
none
rc
stdout
stderr
no-rc
no-stdout
no-stderr
```

The ordinary mark grammar's word/argument discipline applies to any future
surface using these words: one token followed by one comma-separated argument,
not whitespace-separated repeated arguments. The DREP wire spelling is likewise
one closed channel-set field. For example:

```text
rc,stdout,no-stderr
```

`none` is whole-shape sugar setting all three channels to declined. Multiple
positive and negative tokens for different channels compose. A set containing
both polarities for one channel, such as `stdout,no-stdout`, is a contracted-input
error before network contact. Duplicate tokens are likewise rejected rather
than assigned last-wins semantics.

### 2.3 Defaults and explicit overrides

Examples under the v0 defaults:

```text
no record                         Status claimed; Stdout/Stderr declined
predicts stdout                   Status + Stdout claimed; Stderr declined
predicts no-rc,stdout             Status declined; Stdout claimed; Stderr declined
predicts rc,stderr,no-stdout      Status/Stderr claimed; Stdout declined
predicts none                     all three declined
```

`no-stdout` and absent/default Stdout currently have the same authority result,
but not the same authorship meaning. Explicit decline records that the author
considered the channel; the aid plane may use that distinction and a future
`__can_predict()` cannot override it.

### 2.4 Future `__can_predict()` interaction

`res-can-predict-remains-optional` **[TYPED, tabled]** - A future
`cmd__can_predict()` is a legitimate off-ramp ergonomics improvement for quality
shell libraries, but it is too much ceremony for the minimal oracle rung and is
not required by this design. If it is ever added:

```text
explicit DREP channel override   wins
absent/default channel state     may consult __can_predict
```

The exact return type, per-channel granularity, duplicate argparse cost, and
coherence rules are deferred. The spike neither reserves a role name nor builds
scaffolding for it here. DREP's growing extra-Dorc value may reduce the eventual
need for the member.

## 3. The authored DREP form

### 3.1 One strict authority family, one permissive feedback channel

DREP serves two different logical consumers on one physical channel:

```text
prediction channel records   strict, closed, authority-bearing
feedback/report records      bounded, noise-tolerant, aid-facing
```

The prediction family must be recognized from source before execution. Its
authority prefix is literal, closed, and non-dynamic. Freeform or dynamically
derived material may appear only in a tail whose interpretation cannot alter the
channel set.

The exact byte grammar remains implementation latitude, but the semantic shape
is approximately:

```text
predicts <comma-channel-set> [reason-class [free-tail...]]
```

A grounding strawman:

```sh
foo__predict() {
   case "${1-}" in
   status)
      foo "$@"
      ;;
   *)
      printf 'predicts none unmodeled %s\n' "${1-}" \
         >>"${DREP_V1:-/dev/null}"
      false
      ;;
   esac
}
```

The line is plain sh. Off Dorc, DREP defaults to `/dev/null`; a consumer that
wants the record sets the sink and parses the versioned format. Under Dorc, the
controller supplies a per-attempt owned sink and correlates the captured record
to immutable attempt/site context.

### 3.2 Recognition is an authority boundary

`rul-predict-record-form-is-closed` **[ACKED direction]** - The v0 recognizer
accepts one narrow shell form whose semantics it can establish completely. At
minimum it fixes:

```text
printf head and constant format
literal `predicts` verb
literal comma-channel-set field
recognized DREP sink target
one record write
bounded dynamic tail positions, if any
```

Equivalent-looking rewrites do not automatically gain authority. Assigning the
sink to an intermediate variable, building the authority field dynamically,
using `echo`, composing a format string, routing through an unknown helper, or
writing to an unrecognized path may remain ordinary report output but cannot
override channel state until separately modeled.

This is not a rejection of dynamism generally. Dorc aspires to model all
non-inherently-unmodelable sh it can justify. `eval`, runtime-generated code,
dynamic loading, and equivalent architecture-breaking constructs remain hard
boundaries; temporarily unsupported but modelable constructs are implementation
debt. The prediction-record form is deliberately narrower because it mints
authority.

### 3.3 Reserved and permissive grammar coexist

`rul-predict-reserved-prefix-is-fail-fast` **[PROPOSED]** - A line that attempts
the reserved `predicts` verb but violates its closed authority grammar should be
a contracted-input diagnostic, normally before network. Unknown non-reserved
DREP verbs and freeform feedback remain bounded inert evidence under the
noise-tolerant report law.

This line needs a careful usability boundary. DREP is also the general oracle
feedback channel, so fail-fast must not make ordinary feedback brittle. The
strictness attaches to the reserved prediction-control prefix only, never to the
whole DREP stream.

## 4. Static authority, runtime confirmation

### 4.1 What static analysis owns

Before host contact, Dorc determines:

```text
the invocation's admitted CFG and positional definition
every recognized prediction-record site
the literal channel state each site states
which normal paths may reach each site
whether override sites dominate the exits they are meant to classify
the set/cardinality of records legal for the invocation
the channel defaults where no override applies
```

Unmodelable control or dynamic execution in this contracted region fails before
network. Safe conservative support gaps may decline the affected feature, but no
runtime record is allowed to widen what static analysis did not admit.

### 4.2 What runtime contributes

Runtime contributes one fact only:

```text
the statically recognized prediction speech at source site S was reached by
attempt A for command site C
```

The record does not invent its channel set from host bytes. The payload may be
checked against the statically expected literal and attributed controller scope;
it never mints host, site, attempt, source, or generation identity.

For modelable runtime branching, static analysis may admit a closed set of
possible path states. The runtime record confirms which statically admitted
speech act was reached; it does not choose that act's semantics or introduce a
new state.

### 4.3 Missing confirmation is not semantic fallback

`rul-predict-confirmation-failure-is-orthogonal` **[TYPED/ACKED]** - If static
analysis says a selected/closed path requires confirmation and the attempt does
not produce it with intact framing, Dorc does not reinterpret the invocation
using defaults, another branch, or `__can_predict()`. The semantic analysis and
the execution integrity result remain separate values.

The required outcome vocabulary is at least:

```text
Confirmed(ChannelState)
PredictionExecutionIncomplete(reason)
TransportIntegrityLost(reason)
StaticContractInvalid(reason)
```

The exact plan consequence of the latter three is deliberately not ruled here.
They are not ordinary modeled/default/declined outcomes. Existing
`rul-integrity-failure-withholds-mutation` may govern some cells; model-execution
failure may be narrower than transport identity loss. A later sitting must type
the distinction before choosing Run, Guard, or whole-target authority absence.

### 4.4 Finality and absence

An absent record can mean an unmarked/default path only after the controller has
established:

```text
the function invocation completed
the DREP drain completed
the attempt/generation remains current
no buffered record remains
the static CFG excludes a normal override path that could complete without its record
```

This is a correctness dependency on finality, not a demand for every path to emit
a positive record. The ordinary modeled path remains record-free. Static
dominance/exit accounting plus a closed runtime attempt is what makes absence
meaningful.

### 4.5 Duplicate and contradictory records

The static analysis defines allowed cardinality. Duplicate, extra, reordered
where order matters, or mutually contradictory authority records are never
last-wins. They produce `PredictionExecutionIncomplete` or a more specific
closed integrity reason and authorize no channel substitution from the affected
prediction.

## 5. Status, stdout, and stderr

### 5.1 Status is the function aggregate

`rul-predict-status-is-function-aggregate` **[TYPED/ACKED]** - Dorc invokes the
authored `cmd__predict()` and reads the function's ordinary final status. It does
not search backward for a more interesting command, decide that case fallthrough
was accidental, or infer which return was authorially intended. That would be
referent-aware intent inference over sh.

Consequences are intentionally ordinary:

```sh
foo__predict() {
   foo "$@"
   printf 'diagnostic\n' >&2
}
```

predicts the `printf` status by default, not `foo`'s. If the author wants `foo`'s
status, they preserve it in sh. If `set -e` exits before a later DREP line, that is
the authored shell behavior. Dorc does not catch or repair it inside the body.

This posture deliberately carries dangerous but useful shell behavior into the
off-ramp. Templates and lints teach defensive catch-alls and status preservation;
warnings/narrative identify uncovered or incidental fallthrough. The engine does
not silently improve the function into something the author did not write.

### 5.2 Stdout and stderr require positive claim

An actual byte stream may be partial while the process still exits cleanly, and
transport completeness does not prove semantic completeness. Therefore Stdout
and Stderr begin declined and require an explicit positive channel record at an
authored completion point.

The record's runtime reach is an authored completion witness:

```text
bytes arrived + prediction record reached + transport closed
   -> channel may be admitted as complete

bytes arrived + no required prediction record
   -> bounded material only; no predicted value
```

If an author emits the positive record after the modeled command, the DREP
`printf` participates in ordinary aggregate Status. Preserving a prior command's
status is the author's shell responsibility. This makes positive stream claims
more ceremonious than negative overrides, but avoids hidden compilation.

### 5.3 Relationship to at-most completion

`an-atmost-completion-signal` and predicted-stream completion share a mechanical
need: a statically typed authored line reached after the complete value/set has
been emitted, plus bounded transport and exact attempt correspondence. They may
eventually share DREP capture, framing, and close machinery.

They are not one authority species:

```text
predict stream completion   says one observable value is complete
at-most set completion      says a completeness-claim's member set is complete
```

The at-most spelling, ceremony, and failure behavior remain the separate human
burndown item. This report neither designs nor preempts it.

## 6. Compilation boundary and helper wrapping

### 6.1 What remains permitted

Dorc already compiles a host-side harness around authored oracle material. That
harness may:

```text
create and bind controller-owned DREP resources
install attempt/site/generation context
invoke emitted role and helper closures
capture native stdout/stderr/status
frame and drain records
segregate calls from different source authors
clean up owned resources
munge generated helper names where the emission planner requires it
wrap context entry under the existing entry contract
```

The exact trampoline decomposition is implementation latitude. The strong lean
is to wrap complete role/helper calls, preserving each original function body's
bytes and aggregate return, rather than inserting instrumentation between its
statements. This is analogous to contracted entry wrapping: generated code owns
the boundary; authored code owns the interior.

### 6.2 What remains forbidden in the spike

`rul-no-in-body-prediction-insertion` **[TYPED direction]** - The prediction
feature does not authorize a general source-to-source pass that inserts runtime
statements at `: predicts` annotations or between authored statements. Such a
pass would acquire new obligations over `$?`, `set -e`, traps, redirections,
pipelines, variable names, aliases, PATH, signals, and source attribution.

This is a spike scope decision, not a permanent claim that internal
instrumentation is valueless. Much reusable machinery lies beyond the boundary.
Crossing it may eventually pay for timings, fine-grained provenance, at-most
commits, retries, and context checks. The pay-once upside is offset by the ocean
of semantic and verification obligations; this round takes neither side as a
forever doctrine.

### 6.3 What wrapping cannot repair

An outer wrapper can capture the final status of an unmodified function and can
segregate DREP/streams. It cannot recover an earlier helper status that the author
later overwrote and call that the function's prediction. Internal helper
trampolines may record helper outcomes for attribution, but
`rul-predict-status-is-function-aggregate` remains the public contract.

Likewise, wrapping does not make an unexecuted positive completion record appear.
Authored early exit remains early exit. The wrapper's job is to make that absence
detectable and attributed, not to repair it.

## 7. DREP ingress and transport obligations

### 7.1 One physical channel, typed record families

The physical DREP lane may carry prediction-control records beside ordinary
decline classes and feedback. Intake keeps distinct logical species. Encoding or
sanitizing a record never grants authority; only the closed prediction parser
plus static-source correspondence can feed channel coverage.

### 7.2 Bounded and controller-attributed

Prediction records inherit the standing ingress requirements:

```text
aggregate, line, field, record-count, allocation, and numeric bounds
closed authority grammar
controller-minted host/target/site/attempt/generation/source context
late/stale/superseded records decision-inert
unknown records bounded and inert
```

### 7.3 Capture setup failure

Static analysis knows when an invocation may require an authority-bearing DREP
record. If the controller cannot establish the required owned capture channel,
that prediction cannot silently use channel defaults whose override might be
unobservable. Capture failure produces the typed execution/integrity outcome;
it never reveals a more permissive semantic state.

### 7.4 Writer and close integrity

The recognized v0 line should emit one bounded record in one write into the
per-attempt/per-site owned sink. The transport must distinguish clean close,
partial write, malformed record, duplicate, late record, and missing expected
record. Existing report noise remains tolerated outside the reserved authority
family.

## 8. Off-ramp and ergonomics

### 8.1 Minimal authoring curve

The ordinary author writes no prediction records on modeled status-only paths:

```sh
foo__predict() {
   case "${1-}" in
   status) foo "$@" ;;
   *)
      printf 'predicts none unmodeled %s\n' "${1-}" \
         >>"${DREP_V1:-/dev/null}"
      false
      ;;
   esac
}
```

One exceptional catch-all carries the ceremony. Removing it is the broad
authorship claim that every remaining argv shape, including ordinary shell
fallthrough, models Status. That is legal and warning-worthy, not a hard error.

### 8.2 Extra-Dorc consumption

The DREP line remains useful outside Dorc. A shell library consumer may point the
sink at a file or parser and obtain explicit negative/channel records. DREP should
be designed as a stable, understandable, versioned API rather than a private
engine accident.

Negative records are immediately useful. Absence is a positive/default answer
only for a consumer that also reproduces invocation finality and capture-integrity
checks. The future project may ship a small parser/wrapper; this report makes no
promise that a raw empty file alone proves model coverage.

### 8.3 Teaching and lint posture

Templates teach:

```text
Status is claimed unless you say otherwise.
Stdout/Stderr are declined unless you say otherwise.
Write one defensive `predicts none` catch-all for unsupported argv.
Place positive stream records only after the complete value is emitted.
Preserve a prior status explicitly if later reporting would clobber it.
Use the fixed DREP form; keep dynamic explanation in the tail.
```

Lint/hints may identify:

```text
predict functions with no defensive catch-all
ordinary case fallthrough that therefore predicts Status
positive stream records not dominating normal exits
contradictory channel sets
dynamic authority fields
DREP reporting that clobbers a status the author appears to intend to retain
```

Warnings remain advisory; authoring the `__predict` role is the contract. Dorc
does not infer semantic intent from whether a return or fallthrough looks
accidental.

## 9. Spike acceptance obligations

Any build of this design must pin at least:

1. `accept-return-two-as-predicted-status` - an unoverridden `return 2` predicts
   Status 2 and folds a consumer comparing `$?` with 2.
2. `accept-default-status-only-model` - no record claims Status and declines
   Stdout/Stderr.
3. `accept-none-overrides-all-channels` - a reached statically recognized
   `predicts none` record declines all channels whatever the function returns.
4. `accept-per-channel-composition` - comma sets claim/decline independent
   channels; contradictions fail before network.
5. `accept-explicit-overrides-future-defaults` - no prospective
   `__can_predict()` answer can override explicit channel speech.
6. `accept-runtime-record-confirms-static-site` - a matching record confirms only
   the statically attributed source site and attempt.
7. `reject-missing-expected-confirmation` - missing confirmation yields the typed
   prediction-execution outcome, never default semantics.
8. `reject-duplicate-or-contradictory-confirmation` - duplicates and extra
   authority records do not last-win.
9. `reject-capture-setup-default-fallback` - unavailable owned DREP capture cannot
   reveal permissive defaults.
10. `accept-feedback-record-noise` - unknown non-reserved feedback remains bounded
    and inert rather than making the entire report channel fail-fast.
11. `accept-authored-errexit-behavior` - `set -e`, early returns, and DREP write
    failures behave exactly as the written shell; Dorc does not insert a catch.
12. `accept-outer-wrapper-preserves-aggregate-status` - setup/teardown captures the
    unmodified function aggregate without adding in-body statements.
13. `reject-partial-stream-without-completion` - bytes lacking a required positive
    stream record never become a predicted value.
14. `reject-transport-loss-as-semantic-decline` - transport/execution integrity is
    never collapsed into `default`, `claimed`, or `declined`.
15. `accept-off-dorc-drep-form` - with DREP unset the fixed line remains valid sh and
    routes to `/dev/null`; with a supplied sink it emits the documented record.

## 10. Deferred implementation decisions

The design is sufficient to replace the `return 2` collision, but these remain
implementation/design work:

- Exact DREP version/name and prediction record byte grammar.
- Whether positive/negative records share one `predicts` verb or use distinct
  reserved verbs under one family.
- The allowed dynamic reason-tail grammar and sink failure behavior.
- Exact CFG proof for modelable runtime branches and override dominance.
- The typed plan consequence of `PredictionExecutionIncomplete` versus broader
  transport/attribution failure.
- Trampoline granularity for role functions, helper functions, pipelines, and
  wrapper-entered contexts.
- How positive Stdout/Stderr completion preserves an independently desired Status
  without hidden compilation.
- How prediction-control records project into Spine and whether any part enters the
  whylog; changing durable contents/re-ingestion remains behind
  `rul-durable-contents-reviewed-before-design`.
- Whether DREP gains an extra-Dorc parser/helper and what compatibility commitment
  follows.
- Whether the future at-most completion speech shares only transport machinery or
  also an authored idiom.

None of these may re-reserve an exit status, let runtime records widen static
authority, or insert executable statements into authored bodies as an incidental
implementation shortcut.

## 11. Routes considered and not taken

### 11.1 Reserve `return 2` in `__predict`

Rejected. It makes predicted status 2 unrepresentable or ambiguous, breaks exact
status consumers, and contradicts the one-observable model. It remains correct in
verdict functions, where the function status is intentionally a ternary predicate
protocol rather than command Status.

### 11.2 Give `return 2` a static-only special meaning

Rejected for the authored contract. Treating the same function return as decline
when Dorc can trace it and status 2 otherwise makes model semantics differ from
ordinary function semantics and still fails for runtime capability gates. Static
absence/default paths require no reserved status.

### 11.3 Require positive claim marks on every modeled arm

Rejected. Most authored `__predict` code is the model; annotations would grow with
successful coverage and create exhaustion. The function role is the default Status
speech act. Exceptional channel overrides carry the ceremony.

### 11.4 Infer a reached "model-producing act"

Rejected. Deciding whether a case fallthrough, explicit return, helper call, or
fixed status was intentionally model-producing asks Dorc to infer semantic intent
from sh shape. The function aggregate is the contract; warnings may expose dangerous
fallthrough without changing it.

### 11.5 Use a dynamically scoped coverage variable

Deferred/rejected as the primary surface. It preserves every status and is easier
for a same-shell caller to inspect, but is process-local, subshell-sensitive,
nested-call-clobberable, weaker for source attribution, and requires outer transport
anyway. Protocol assignment also survives strip unless guarded. It remains a possible
implementation aid inside generated outer wrappers, never the authored authority
source ruled here.

### 11.6 Add `cmd__can_predict()` now

Tabled, not nacked. It is the most natural extra-Dorc preflight query and may improve
high-quality reusable shell libraries. It duplicates argparse/capability work and is
too much ceremony for the first oracle rung. The authored DREP route lowers its
marginal value. If added later, it affects defaults only; explicit DREP channel
speech wins.

### 11.7 Compile prediction marks into runtime body statements

Not taken for the spike. A source mark lowered to an internal OOB write is ergonomic
and could unlock reusable instrumentation machinery. It also requires a
semantics-preserving source-to-source transformation across `$?`, `set -e`, traps,
redirections, pipelines, command substitution, shell variables, signals, aliases,
PATH, and source attribution. Transparent post-command instrumentation is not
generic: preserving a nonzero status often requires an `||`/capture wrapper that
changes errexit semantics.

This is not rejected on user-disgust grounds. Dorc already rearranges, wraps, munges,
and concurrently invokes oracle code. The primary objections are implementation
complexity, subtle semantic drift, and the verification burden. Crossing the boundary
may eventually be worth its pay-once value for timings, fine-grained provenance,
completion commits, retries, and context checks; it requires its own explicit ruling.

### 11.8 Use traps or signals as the second result

Rejected. Function-return traps are not portable across the floor; signal encodings
alter process/shell behavior, race under concurrency, collapse into shell-specific
statuses, and have disastrous off-ramp ergonomics. They do not improve on DREP.

### 11.9 Treat DREP as aid-only

Rejected as a premise. DREP was designed as the general constrained-bandwidth OOB
channel between oracle code and controller. Its record species have separate authority
rules. Aid narration remains decision-inert; prediction-control records are typed
host evidence consumed under their own closed contract.

### 11.10 Reject all dynamism around prediction records

Rejected. The boundary is modelability, not dynamism. Dorc should model ordinary
dynamic sh control where it can preserve meaning. Only inherently unmodelable or
not-yet-supported constructs decline. The authority prefix and channel set stay
literal even when a modelable runtime branch chooses whether the record is reached.

## 12. Costs accepted and benefits retained

The selected direction accepts:

```text
one rigid authority-bearing printf form
static/runtime correspondence and finality machinery
real DREP write effects in authored shell
extra ceremony on exceptional/stream-completion paths
public protocol pressure if extra-Dorc parsers appear
an unresolved typed consequence for execution-integrity loss
```

It retains:

```text
all predicted status values
literal authored shell behavior and off-ramp parity
Status-only models with zero annotation
positive-only stream claims
explicit per-channel overrides
the existing DREP investment and extra-Dorc potential
no hidden in-body compiler insertion
clear separation of static authority from runtime confirmation
detectability of unexpected early termination
```

The spike's posture is therefore narrow and falsifiable: build the authored-DREP
route first; measure its authoring friction in the starter oracle work; revisit
internal instrumentation or `__can_predict()` only on concrete pressure, never as
prebuilt scaffolding.
