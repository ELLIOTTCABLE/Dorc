# 30D — Prediction channels, decline, and execution integrity

> Design of record: intended semantics, not an implementation-status report. AI-authored,
> human-directed; subordinate to the root human documents. Owns `__predict`'s observable
> contract and its agreement with DREP, plus the shared decline rule for `__is_converged`.
> Related: `plans/27C` (context entry), `plans/30S` (environment identity), `plans/30U`
> (at-most completion), `notes/27W` (decline classes/reporting). Reasoning and human rulings:
> `notes/311a` §§11–13. Build phasing is deliberately separate.

## 1. The contract

**`rul-predict-status-is-function-aggregate`** — `cmd__predict()` is an authored read-only
shell model of an invocation. Its actual aggregate exit status models the command's Status
by default. Dorc neither searches backward for a more interesting command nor infers which
status the author intended. A final debug print, implicit case fallthrough, or early shell
exit has its ordinary shell consequence; model adequacy and internal failure handling are
part of the author's contract.

**`rul-predict-status-keeps-every-value`** — every shell status remains representable,
including 2 and high values. Bare `return 2` predicts tool status 2. Whole-prediction decline
requires BOTH a DREP decline and actual shell status 2 (§3); no numeric status alone means
that a prediction was unavailable.

**`rul-predict-channel-defaults`** — authored channel states are `default`, `claimed`, and
`declined`. Preserve the distinction until the policy consumer; absence is not an explicit
refusal. Defaults are:

| Channel | Default | Authorship required for more |
|---|---|---|
| Status | Claimed: the function's aggregate rc | None for status-only modeling |
| Stdout | Declined | A positive, post-production DREP claim |
| Stderr | Declined | A positive, post-production DREP claim |

Effects, facts, and backing topology retain their existing marks and contracts; they are
not additional channels in this menu. A predict supplies modeled observables, not a
convergence vouch, and executes in the probe lane, never as the model in an apply guard.

The enhancement curve distinguishes a small convergence predicate, an rc prediction, and a
stream prediction. These are increasingly demanding contracts, not rigid user categories.
Dorc may constrain oracle code to protect them, but restrictions must earn their ecosystem
cost: encourage defensive, reusable shell, not code that works correctly only inside Dorc.

## 2. Authored channel speech

**`rul-predict-overrides-are-authored-shell`** — channel speech is an ordinary, actually
executed `printf` to the versioned DREP sink. It survives `dorc strip` unchanged. The sink's
name carries the protocol version (`DREP_V1` below); its value is supplied by Dorc. The
`${DREP_V1:-/dev/null}` spelling is usable outside Dorc and safe under nounset.

**`rul-predict-channel-token-set`** — the closed channel vocabulary is:

```text
none    rc    stdout    stderr    no-rc    no-stdout    no-stderr
```

The `predicts` verb takes one comma-separated channel-set field, with an optional bounded
reason tail. `none` sets all channels to declined; it is the whole-prediction refusal.
Positive and negative tokens for different channels compose. Opposing polarities for one
channel, duplicate tokens, or `none` combined with other tokens are contract errors, not
last-wins configuration.

```sh
printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}"
printf 'predicts no-rc,stdout\n' >>"${DREP_V1:-/dev/null}"
```

The first claims Status by default and Stdout explicitly. The second claims only Stdout;
a per-channel `no-rc` is not a whole-invocation decline and does not prescribe status 2.
`rc` claims the actual shell status; it cannot carry a replacement numeric value.

**`rul-predict-record-form-is-closed`** — authority requires a statically recognized form:
known `printf` semantics, a literal authority verb and channel set, the recognized sink,
and a bounded record with any dynamic material confined to a decision-inert tail. Equivalent-
looking unmodeled spellings gain no authority. General feedback remains noise-tolerant;
an attempted reserved control record with invalid grammar is a contract error. The physical
DREP channel is not aid-only: control records and feedback have separate logical consumers.

## 3. Decline and mandatory shell semantics

**`rul-shell-semantic-output-is-mandatory`** — where DREP and shell-semantic output both
speak, they must agree. DREP never repairs or replaces an incompatible shell result. Where
one spelling is optional, it is DREP, never the off-ramp-friendly shell output.

The shared defensive idiom is:

```sh
printf 'decline unmodeled unsupported invocation\n' >>"${DREP_V1:-/dev/null}"
return 2
```

A recognized `decline <class> [tail]` withdraws the role's answer. The class vocabulary is
`unsound`, `unmodeled`, `interactive`, `hazard`; the choice of class and explanation do not
change the decline's authority. `27W` owns their reporting uses. In a predict, the named
decline withdraws all three channels, as `predicts none` does.

**`rul-predict-decline-requires-both-outputs`** — a whole-prediction decline, including
`predicts none`, must be accompanied by actual final status 2. The ordinary spelling is
`return 2`; the requirement concerns the resulting shell semantics, not one AST spelling.
An off-Dorc consumer sees an error status but needs DREP to distinguish decline from a
modeled tool status 2. That ambiguity is accepted; full model-protocol consumption requires
both outputs.

**`rul-verdict-partition-stays-separate`** — `__is_converged()` remains a predicate:
0 = named judgment holds; 1 = complement; >=2 = cannot judge. Its numeric decline is
sufficient without DREP. If a named decline record is also emitted, the final status must
remain in that decline range; 2 is the common idiom across both functions.

| Role | DREP | Actual shell status | Meaning |
|---|---|---|---|
| Predict | No override | Any N, including 2 | Predict N |
| Predict | Whole decline | 2 | Decline; no modeled observables |
| Predict | Whole decline | Anything other than 2 | Contract conflict |
| Verdict | No decline record | 0 / 1 / >=2 | Holds / complement / cannot judge |
| Verdict | Named decline | >=2 | Decline |
| Verdict | Named decline | 0 or 1 | Contract conflict |

**`rul-decline-conflicts-fail-fast`** — a decline record followed only by a successful
`printf` does not make a valid record-only decline: the resulting zero is a conflict. Detect
provable conflicts before host contact; detect remaining confirmed conflicts at readback,
and do not mint an answer from them. Failed execution or missing confirmation is instead
an integrity outcome (§5), never fabricated evidence that the author contradicted themselves.
A decline record does not return from a function or prevent later statements from running.
Opposing positive-channel speech and whole decline in the same invocation likewise conflict.

## 4. Stream completion and status preservation

**`rul-predict-stream-claim-follows-production`** — a positive stream claim is authored
after the production it covers. Its reached position is a checkpoint: execution reached
the author's statement that the preceding bytes constitute the modeled observable. It
cannot seal unaccounted later writes to that channel. Static analysis establishes the
record's scope and order in the admitted control flow, including relevant helper calls.

The record need not be the last status-affecting statement. Preserve a meaningful rc in
ordinary shell, for example with a direct external producer:

```sh
thing__predict() {
    if thing_status "$@"; then rc=0; else rc=$?; fi
    printf 'predicts stdout\n' >>"${DREP_V1:-/dev/null}"
    return "$rc"
}
```

The author chose this conditional context; it is not a generic compiler rewrite for
helpers or substitutions. Both Dorc and an ordinary caller receive the same actual rc.
The tracer must admit the status save and restoring return rather than require the record
to be the literal last statement. A nonzero modeled status can coexist with a valid stream
claim: a model may describe the output of a failed tool invocation.

**`rul-checkpoint-does-not-prove-success`** — reaching a seal does not prove successful
execution of every predecessor, semantic completeness, or fidelity to the real tool.
`producer || :` can reach it after losing data. That is a false authored claim, not something
Dorc promises to repair. Conversely, exit before the required seal leaves no stream license.
Capture integrity and closure remain separate prerequisites: arrival on DREP does not imply
stdout or stderr has drained, and cross-stream temporal ordering is not an integrity proof.

Stream prediction already requires positive speech; placing it after production buys this
checkpoint without a second authored act. Status-only prediction deliberately avoids that
extra act. This is a contract/authoring-cost distinction, not a claim that rc witnesses are
impossible or that stdout can be certified semantically.

`30U`'s `disturbs nothing-else` remains a different authority species: its content is an
at-most/finished-definition claim, and its reached position additionally supplies a checkpoint.
The required record, the completion-only role's zero status, and intact capture are conjuncts.
Zero alone does not replace totality speech; a record alone does not excuse body death.
Detected failure withdraws sparing authority while retaining known collisions.

## 5. Static authority and runtime integrity

**`rul-predict-runtime-confirms-static-analysis`** — static analysis admits the invocation's
control flow, source/custody, channel states, record sites, and allowed cardinality. Runtime
confirms which admitted speech was reached in the exact invocation; it cannot invent a
channel vocabulary, another author, or a more permissive state. Modelable runtime branching
is allowed; unsupported analysis withholds the affected prediction rather than guessing.

**`rul-predict-confirmation-failure-is-orthogonal`** — keep semantic coverage separate from
execution and transport integrity. At minimum distinguish:

```text
Confirmed(channel state and result)
PredictionExecutionIncomplete(reason)
TransportIntegrityLost(reason)
StaticContractInvalid(reason)
```

Missing, malformed, duplicate, contradictory, stale, or wrong-attempt records do not reveal
channel defaults or select a more convenient branch. Absence can mean an unmarked/default
path only after invocation/capture finality and the admitted control flow establish that no
required override was lost. If that distinction cannot be established, withhold the affected
prediction. If the failure's scope cannot be bounded, withhold the whole prediction.

**`rul-known-execution-failure-is-not-a-number`** — Dorc's known invocation, timeout,
context-establishment, capture, and transport failures are not modeled statuses. Use the
actual execution event, not a reserved range: returning 124 does not itself prove a Dorc
timeout, and returning 200 does not itself prove a signal death. Internal model failures
indistinguishable from authored outcomes remain inside the model author's responsibility.

Intake is bounded and controller-attributed: allocation/byte/field/count limits; immutable
site, attempt, generation, context and source association; closed authority grammar; no
last-wins duplicates. Unknown non-control feedback stays bounded, attributed and inert.
Unavailable required capture never falls back to an inert sink while admitting answers whose
refusal could consequently be lost. Partial stream bytes remain non-authoritative until the
claim, capture closure, and integrity conditions have all been discharged.

Only admitted channel results enter value propagation, branch folding, and settlement.
Independent channels may survive a failure only where their authority and integrity can be
established independently. Prediction-execution failure authorizes no new execution of book
code: apply behavior remains subject to the existing phase, reachability, and consent laws.

## 6. Invocation and compilation boundary

**`rul-predict-body-remains-literal`** / **`rul-no-in-body-prediction-insertion`** — preserve
the authored body's ordinary shell behavior under Dorc's stated invocation contract. Existing
strip erasure, name/placement transformations, context entry, and outer scaffolding retain
their own constraints. This feature authorizes no inserted completion markers, status-repair
wrappers, or heuristic changes to the body's answer.

Dorc constructs the invocation state rather than accidentally inheriting book settings.
It must preserve modeled inputs under the environment/context contracts (`30S`, `27C`),
respect the body's authored option/error policy, and not suppress it merely by collecting
the result in a tested caller. Helper resolution, positional arguments, cwd and descriptor
routing are part of that obligation. The promise is one reproducible calling convention,
not equivalence under every possible off-Dorc caller. Where an execution context cannot
provide it, withhold the affected optimization. Predicts stay probe-only; verdict bodies
used in probe and apply must likewise receive their promised invocation semantics.

Dorc owns setup, capture, framing, correlation, cleanup, and the boundary failures it can
establish. Authors own their read-only vouch, model adequacy, and internal shell handling.
No status-meaningfulness filter or guard-every-fallible-step rule is part of this contract.
Oracle restrictions remain legitimate when justified by the contract and reusable-shell
value, but no additional bans are imposed merely to accommodate a discarded witness design.

## 7. Acceptance obligations

These are semantic obligations, not build phases:

- Preserve default status-only modeling and all numeric statuses; leave an ordinary final
  print or fallthrough's status unchanged; do not mistake engine failure for a model result.
- Distinguish bare predicted 2, combined predict decline, rc-only verdict decline, and valid
  combined verdict decline. Reject record-only success and other disagreement, statically
  when provable and at runtime otherwise; missing records are not presumed disagreement.
- Preserve per-channel defaults and overrides; reject conflicting sets and invalid reserved
  control grammar; maintain record-site/attempt identity, cardinality, and finality.
- Refuse required-confirmation loss without default fallback, including capture setup failure;
  retain noise-tolerant feedback without allowing it to become control speech.
- **`reject-partial-stream-without-completion`** — admit post-production stream claims
  followed by a real status-restoring return, including nonzero statuses; do not admit missing
  seals, failed capture, or unaccounted later output.
- Preserve ordinary shell invocation/error behavior and off-Dorc actual return status;
  apply no hidden in-body repair. Keep the existing at-most record/status/integrity conjunction
  and retain known collisions on failure.

## 8. Constraints and paths not taken

The reasoning is in `311a` §§8–13; it is not repeated here. Reserving rc 2 globally loses
real tool outcomes; a separate handled-query duplicates argparse; differing-value DREP rc
overrides split Dorc from plain-shell behavior. Inferring the intended final command rejects
valid mocks without reliably detecting wrong ones. Generated answer wrappers can change
nested errexit semantics; an instrumented program's rc agreeing with its own record does
not prove equivalence with the original. General completion compilation, xtrace-as-license,
and interpreter instrumentation are not prerequisites or promised future work. Authored
stream seals retain bounded value without making any of those claims. Aid is not a substitute
for a correct contract.
