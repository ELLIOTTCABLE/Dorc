# 26La — Cross-host authority and orchestration boundary addendum

> QUARANTINED. Read only if already granted access to
> `Research/quarantine-DO-NOT-READ/` under its current information-flow rules.
>
> Status: AI-authored security-focused addendum to `Research/notes/26L`, from the
> human-directed sitting of 2026-08-24. It is an exploration report, not a ruling,
> threat-model replacement, or implementation authorization. `26L` carries the product,
> capability, and research synthesis; this document carries the security strata,
> cross-host authority implications, and re-entry obligations that cannot travel with it.
>
> Evidence posture: training-data brainstorming + current Dorc design/security corpus +
> eight plain-sh strawmen at `26L-strawmen/`. No live prior-art/source pass was completed.
> Tool-specific conclusions remain provisional.
>
> Governing current law: `AGENTS.for-builders-only.md` · `plans/102` · round-29
> `291`–`295` · `306a`/`notes/306b` · `spike/CLAUDE.md` host-evidence, decision-identity,
> influence, refusal, and repeated-probing rules. This note does not weaken any of them.

## §0 — Security findings in one screen

- **`finding-cross-host-control-arrives-early`** — +SURE. The fundamental new vector is
  `host A -> controller decision -> host B`. It appears as soon as Dorc manages multiple
  hosts and ordinary fail-fast/barrier behavior lets A's success, failure, silence, or timing
  determine whether or when B is touched. It does not wait for caches, shared facts, or typed
  fleet models.
- **`finding-l1-l2-is-failure-posture`** — +SURE. A fixed source-order graph remains L1
  only if B is attempted after A regardless of A's outcome. Normal fail-fast behavior turns
  it into L2. Fully isolating failures would avoid the channel but can violate the admin's
  operational safety requirements; isolation is not automatically the safer product.
- **`finding-minimal-orchestration-is-bounded-not-free`** — ~SUSPECT. Approximately-L2
  native orchestration is the first genuine security purchase, but its scope can be narrower
  than initially feared if runtime information may only admit/withhold exact pre-authored
  actions and may not author targets, command bytes, facts, or omissions.
- **`finding-execution-policy-precedes-semantic-authority`** — +SURE. Cross-host execution
  policy and cross-host semantic knowledge are distinct strata. Closed gates over authored
  actions can be isolated largely to the fleet/executor boundary; values, shared world models,
  and cross-host elision carry host influence into analysis, planning, rendering, and apply.
- **`finding-authored-shell-does-not-remove-responsibility`** — +SURE. The admin could run
  the same sh without Dorc, but Dorc becomes the controller holding fleet-wide authority and
  presenting the plan as a reviewed product. Preserving authored semantics limits introduced
  behavior; it does not eliminate controller attribution, generation, integrity, cancellation,
  and result-handling obligations.
- **`finding-contingent-progression-is-the-first-hot-surface`** — ~SUSPECT. Plan-time
  lifting of an authored cross-host gate is likely the first useful feature to consume remote
  results across host scope. It is therefore the correct first design target for authority-map,
  influence, refusal, and DST review—not a later fleet-cache concern.
- **`finding-preview-is-hostile-sensitive-and-side-effectful`** — +SURE. A foreign planner
  may contact large control planes, expose arbitrary output, lock/read live state, and take
  minutes. Preview output is both untrusted structure and potentially sensitive operational
  material; “read-only” does not imply harmless, nonblocking, or complete.
- **`finding-continuity-is-a-veto-net-not-proof`** — +SURE. The tabled exact-preview
  comparison idea can at most veto or request attention. Matching bytes do not prove the
  foreign apply remains equivalent; mismatching bytes may be cosmetic. Its most dangerous
  failure is overconfidence created by a trust-increasing appearance.
- **`finding-cross-host-omission-is-poor-value`** — ~SUSPECT. L6 lets one host suppress a
  repair on another for mostly probe/attention performance. It has the weakest near-term
  value-to-authority ratio and remains the strongest candidate for permanent refusal.
- **`finding-target-capability-modes-may-be-useful`** — -GUESS. The L0–L7 vocabulary may
  eventually collapse into a smaller admin policy vocabulary: particular host sets or trust
  zones could remain restricted to lower cross-host capabilities even if richer behavior is
  implemented elsewhere. No surface or defaults were designed.

## §1 — The new vector already exists at basic fleet scale

Current Dorc's width-one security boundary is:

```text
controller-authored source
   -> controller analysis
   -> one host probe
   -> hostile host bytes
   -> controller decision about that same host
```

Minimal fleet behavior adds:

```text
host Advil behavior
   -> controller's shell/fleet progression
   -> whether/how/when controller authority reaches Beverly
```

+SURE the channel is not limited to returned data values. Advil can influence:

- exit status;
- whether a completion marker arrives;
- when it arrives;
- whether a deadline fires;
- whether a barrier releases;
- whether fail-fast stops the controller;
- which fallback branch the authored shell takes;
- what an operator sees before deciding whether to continue.

+SURE controller-minted timestamps and attempt IDs attribute these events; they do not make
their occurrence uninfluenced. `306a`'s correction applies directly: the controller may mint
the clock reading while the remote participant chooses whether the state at that reading is
“completed,” “timed out,” or “partial.”

~SUSPECT this is the session's most important security reframing. Native orchestration does
not move Dorc from zero cross-host exposure to a maximal fleet model. It crosses the horizon
immediately, then offers several increasingly powerful—and increasingly avoidable—extensions.

## §2 — L0–L7 as an authority ladder

The product ladder in `26L §2` can be read as “what may A determine about B?”

| layer | A's permitted influence over B | additional security burden |
|---|---|---|
| **L0** | none through engine decisions | per-host intake/decision isolation only |
| **L1** | source order controls timing; A's outcome does not suppress B | fleet execution identity and independent result accounting |
| **L2** | A's closed outcome admits or withholds an exact authored B action | first cross-host authority map; generation/finality; refusal behavior |
| **L3** | A selects one member of a closed authored action/target set | target/action-selection authority; confused-deputy and scope concerns |
| **L4** | A supplies bytes consumed by B | hostile/sensitive value carriage; sink-specific encoding; context/target validation |
| **L5** | A contributes to B's world model | aggregation completeness; timing influence; global taint; conflict/snapshot semantics |
| **L6** | A's evidence prevents B's authored work | peer-suppression authority; freshness/equivalence/hermeticity; broad blast radius |
| **L7** | A causes new/re-written B work after consent | approval identity break; generation revocation; dynamic authority expansion |

### `finding-l1-is-a-narrow-failure-policy`

+SURE L1's distinction is real but narrow:

```sh
advil_rc=0
apply_advil || advil_rc=$?

beverly_rc=0
apply_beverly || beverly_rc=$?
```

Beverly is attempted regardless of Advil's outcome. Advil still controls timing by being a
predecessor, but not Beverly's reachability.

### `finding-fail-fast-collapses-l1-into-l2`

+SURE ordinary `set -e`, `A && B`, `A || exit`, an explicit barrier, or the controller's own
“stop on host failure” policy gives A control over B's reachability. A basic fleet executor
cannot claim L1 while silently implementing that posture.

~SUSPECT fully denying cross-host failure propagation is not a generally acceptable security
floor. Continuing onto dependents after a failed prerequisite can spend more authority in a
world the admin explicitly said was not ready. The security and operational correctness
directions are not aligned by default; the authored dependency must remain visible.

## §3 — The likely inflection: closed gates over pre-authored work

~SUSPECT L2 remains the likely value/security knee, now stated without the earlier fiction
that a static graph precedes it harmlessly.

The bounded L2 contract explored here is:

```text
all possible effectful commands are authored before consent
all possible target recipes are authored before consent
remote outcomes choose only admit / withhold
missing or integrity-lost outcomes never admit
no outcome manufactures command bytes, targets, facts, or elision authority
```

+SURE this can serve high-value real operations:

- provision succeeds before configuration begins;
- a control plane reports ready before its client is invoked;
- a canary reaches the authored readiness predicate before later hosts proceed;
- a migration prerequisite holds before workload rollout;
- true dependents are withheld while independent work follows authored continue behavior.

+SURE it still grants remote state influence over controller authority. The containment is
that the influenced result selects only progression through a reviewed program; it does not
write that program.

### `finding-current-influence-law-needs-explicit-reentry`

+SURE `306b:rul-influenced-values-never-gate-engine-control-flow` currently forbids exactly
the naive implementation: branch the engine on a scalar derived from host bytes. L2 cannot
drift into a scheduler as an ordinary boolean. It requires an explicit reviewed authority
species, supplier/interpreter/scope/expiry/consequence map, and a result type whose permitted
consumer is the corresponding authored gate only.

## §4 — Native orchestration's minimum obligations

Preserving shell behavior avoids adding hidden fleet policy, but Dorc still owns several
controller-level properties.

### `requirement-controller-minted-scope`

+SURE host, target, invocation, attempt, generation, source set, and capability context must
come from immutable controller-owned context. Payload claims may be checked, never accepted as
scope. Scope survives every conversion and result key.

### `requirement-failure-species-stay-distinct`

+SURE these cannot collapse into one shell rc:

- authored command returned nonzero;
- target was never reached;
- execution may have begun but the session was lost;
- result framing or attribution failed;
- operation was canceled/superseded;
- remote work is not yet observed quiescent.

Analysis uncertainty may preserve authored work; lost attempt/execution integrity withholds
further mutation for the affected scope. `rul-integrity-failure-withholds-mutation` remains
binding.

### `requirement-generation-revokes-before-quiescence`

+SURE retry, cancellation, replan, or operator abort must immediately revoke prior decision
authority. Remote processes may continue and emit late output; that output may be bounded
diagnostic material only. Cancellation acknowledgement is not execution finality.

### `requirement-no-hidden-mutation-policy`

+SURE no automatic apply retry, mutation reordering, or inferred parallelism belongs in the
minimal substrate. Each would create new outcomes absent from the authored shell and widen the
review target substantially.

### `requirement-cross-target-credential-siting-is-explicit`

+SURE controller-mediated `ssh B ...` and an `ssh B ...` command executed *from host A* have
different trust/credential geometry. Dorc must not reinterpret one as the other silently.
Forwarded agents or credentials on A create an orthogonal high-cost boundary and are not
required for L2 behavior.

## §5 — Contingent progression: authored control, lifted authority

### `finding-shell-authorship-bounds-but-does-not-neutralize`

+SURE the admin already authorized a gate when writing:

```sh
if ready_on_advil; then
   apply_on_beverly
fi
```

At ordinary runtime, Advil's answer controls Beverly. Dorc lifting that check does not invent
the relationship, but it changes where and when the result is consumed and may convert it into
a pre-consent plan decision. The equivalence obligation is therefore not “the edge was
authored” alone; it includes target identity, context, freshness, and no intervening authored
mutation that could change the predicate.

### `finding-conditional-and-barrier-have-different-authority`

+SURE a false conditional:

```sh
if ready; then B; fi
```

licenses B not to run for that execution. A false barrier:

```sh
until ready; do sleep 2; done
B
```

does not: B remains desired and merely delayed. Treating both as a “false gate” would turn a
temporary remote answer into under-execution.

### `requirement-plan-time-lift-stays-on-pristine-prefix`

+SURE plan-time consumption of a readiness result is tenable only while every earlier action
capable of invalidating it is elided/non-mutative, or while an existing reviewed mechanism
revalidates it in position. Once a preceding powerful tool runs as a wall, downstream plan-time
readiness must not remain authoritative by wishful continuity.

### `requirement-unknown-never-becomes-false-or-true`

+SURE timeout, malformed output, missing sentinel, stale generation, target mismatch, or
unavailable context is neither “ready” nor “not ready.” The gate receives no authoritative
outcome. Its disposition follows the authored shell plus the integrity/refusal rules, never a
generic boolean default.

### `requirement-aid-exposes-the-cross-host-edge`

~SUSPECT every consumed cross-host gate should be explainable as:

```text
supplier/result scope
authored control edge
which later actions it admitted or withheld
generation and observation time
whether the result was measured, vouched, derived, or consented
```

Influence carriage must continue through selection, ordering, rendering, and blame. A result
that changes which host's action is shown can steer the operator even when it mints no plan
license.

## §6 — `cmd__plan_preview()` boundary

### `finding-preview-is-an-execution-not-just-text`

+SURE calling a foreign planner is an active controller operation. It may open network
connections, acquire locks, read secrets, trigger audit records, invoke plugins, consume
substantial resources, and hang. Oracle authorship and admin opt-in do not make it inert.

### `requirement-preview-output-is-bounded-and-sink-encoded`

+SURE native preview output may contain terminal control sequences, forged-looking records,
paths, argv, resource values, and secrets. It must cross aggregate/line/field/allocation bounds
before retention; raw evidence remains separate from terminal/JSON/shell encodings; encoding
does not grant trust or remove sensitivity.

### `requirement-preview-remains-aid-only-by-default`

+SURE the proposed member is a view contribution unless a separately reviewed authority path
is added. It must not silently become a fact, convergence vouch, target set, footprint,
`is_converged` answer, or cross-host gate merely because its output looks structured.

### `requirement-preview-retention-is-not-casual`

+SURE embedding or durably retaining native foreign plans can widen sensitive artifacts and
replay surfaces. Any durable-content change clears the existing review gates first. The
admin's opt-in to running a preview is not automatically consent to persistence or sharing.

## §7 — Slow planners and TOCTOU

### `finding-plan-runtime-enlarges-the-window`

+SURE if several foreign convergence/preview members each take minutes, the oldest result may
be stale before the plan is presented. Planning duration is part of the TOCTOU interval, not
merely an attention/performance cost.

### `finding-full-replan-can-be-recursively-expensive`

+SURE “run a fresh plan before apply” ceases to be a cheap discipline when the fresh plan is
itself the fifteen-minute operation. Running each planner again immediately before its command
reduces one interval but may double plan cost and cannot eliminate the final race.

### `requirement-no-generic-freshness-fiction`

+SURE Dorc cannot manufacture a cheap semantic freshness check from arbitrary foreign output.
Native saved decisions, revision identities, or apply-time validation can be consumed where
upstream owns their semantics. Else the choices are full replan, explicit stale-plan
acceptance, runtime guards, or refusal—not inferred equivalence.

### `question-multi-planner-interaction`

~SUSPECT concurrent heavy planners may interact through shared APIs, locks, rate limits,
credentials, or one another's state. The current “parallel read-only probes are cheap” posture
must be re-measured before foreign planners inherit it.

## §8 — Continuity tripwire: opt-in and hole-filled

### `finding-veto-only-is-the-lowest-authority-form`

~SUSPECT exact opaque preview comparison can be bounded as a veto: same bytes permit the
ordinary baseline action to proceed; changed/unknown bytes halt or request attention. It never
adds, rewrites, retargets, or elides work.

+SURE this does not establish semantic continuity. A false “same” falls back to the risk the
admin already accepted by running the command; a false “different” causes nuisance/denial.

### `finding-overconfidence-is-the-primary-product-hazard`

+SURE the mechanism looks like a trust-increasing certification while explicitly failing to
certify completeness or equivalence. The admin opt-in must say, in substance: “try to stop at
the last minute if the foreign preview changes; accept that this net has holes.” Any stronger
name or prose is misleading.

### `finding-pristine-prefix-contains-resume-cost`

+SURE the tripwire is cleanest before the first Dorc mutation, where all earlier sites were
elided/non-mutative. A mismatch can halt before the controller changed the world, avoiding
blind repetition of an already-executed prefix. After a wall runs, foreign preview changes may
be the plan's intended consequence and exact comparison loses meaning without effect knowledge.

### `question-mid-plan-user-attention`

~SUSPECT an explicit mode that pauses on mismatch and waits for user input is not absurd. It
breaks the default one-consent model and introduces interactive controller state, resume, and
operator-presence concerns. A simple halt-and-replan path is narrower; neither was designed.

## §9 — Higher strata and declining returns

### `finding-l3-selection-spends-authority`

+SURE fixed action selection allows remote output to decide where/which approved action runs.
This is stronger than withholding: a false role/leader answer can direct privileged work to the
wrong target even when the command set is closed.

### `finding-l4-data-crossing-needs-two-axes`

+SURE a carried value is independently hostile and sensitive. Bounds/quoting prevent some
injection shapes; they do not prove the value names the intended cluster, identity, endpoint,
credential scope, or incarnation. Host-derived bytes must never become code, target identity,
path, template, or claim algebra through a generic string conversion.

### `finding-l5-aggregation-widens-everything`

+SURE shared relational state adds completeness, duplicate identity, snapshot/generation,
arrival-order, deadline, withholding, and conflict questions. One participant's influence can
reach the entire fleet model and every projection derived from it.

### `finding-l6-cross-host-elision-is-exceptional`

+SURE cross-host omission is not merely a stronger gate. Evidence from A becomes authority to
prevent B's authored repair. Hermeticity, equivalence, freshness, scope, and oracle semantics
must all hold. B can always be probed independently, making the unique benefit mostly
performance/attention.

### `finding-l7-breaks-decision-identity`

+SURE post-consent creation of targets/actions means the human-readable and executable plans no
longer derive from one exact pre-consent decision. Generation revocation, late-output handling,
new consent semantics, and dynamic authority all become unavoidable.

## §10 — Potential per-target capability floors

-GUESS a future admin may want capability ceilings by target set or context:

```text
backup appliances: L0 only
ordinary independent pets: L1
one trusted cluster: L2 closed gates
bootstrap controller/worker set: selected L4 value routes
no targets: L6/L7
```

+SURE any such policy must be controller-owned, explicit, and part of decision identity. A
managed host must never grant itself a richer layer. Defaults, inheritance, cross-context
composition, and discoverability are completely unruled.

~SUSPECT the ladder is too large for a final user surface. Its current value is analytical:
it prevents “fleet support” from silently bundling seven different authority expansions.

## §11 — Current-law re-entry map

| prospective surface | controlling current rules |
|---|---|
| multi-host intake/results | `sinv-host-evidence-ingress` · `sinv-controller-attribution` · `rul-host-bytes-bounded-before-admission` |
| retries/cancellation/late output | `sinv-generation-revocation` · `rul-integrity-failure-withholds-mutation` |
| plan/apply identity | `sinv-decision-identity` · `294:hard-truth-approval-is-not-freshness` |
| cross-host gate authority | `sinv-authority-map` · `292:model-knowledge-needs-authority-map` · `306b:rul-influenced-values-never-gate-engine-control-flow` |
| new authority constructors | `sinv-private-authority-mints` · mint-time witness obligations |
| foreign preview output | `sinv-hostile-sensitive-orthogonal` · `sinv-sink-encoding` · `sinv-sensitive-artifacts` |
| context-shifted probing | `sinv-context-siting` · `306b:rul-authority-free-probing-mode` |
| lost intake integrity | `sinv-integrity-failure-mutation` · `306b:rul-report-only-output-cannot-plan` |
| repeated/multi-target probing | `sinv-multi-exchange-probing` · `spike/CLAUDE.md:rul-repeated-probing-reviewed-before-design` |
| cross-host reuse | `sinv-controller-attribution` explicit reviewed aggregation/conversion constructor |

+SURE no implementation brief may treat a `26L` candidate as permission to bypass these
re-entry gates.

## §12 — Open security questions before design

1. **`ask-authored-gate-authority-map`** — exactly which authored control-flow shapes may
   consume a remote result at plan time, and with which supplier/interpreter/scope/expiry?
2. **`ask-failure-containment-versus-dependency`** — how does the product preserve true
   prerequisite failure without letting one unrelated host deny the whole fleet?
3. **`ask-target-incarnation-continuity`** — what establishes that the host/context probed is
   the one later acted upon, especially under aliases, replacement, and control-plane outputs?
4. **`ask-controller-book-credential-boundary`** — which commands execute on the controller,
   which on targets, and how are credentials/context prevented from moving implicitly?
5. **`ask-contingent-lift-refusal`** — when gate evidence is malformed, partial, late, or
   unattributed, does the whole target refuse, does the branch remain runtime-only, or does the
   workflow stop at a larger boundary?
6. **`ask-preview-output-lifecycle`** — where is native plan output captured, displayed,
   retained, and deleted; which parts may be sensitive; how are failures visible?
7. **`ask-slow-planner-freshness`** — which planner classes can bind to saved/native decisions,
   which require full replan, and which remain explicitly stale/advisory?
8. **`ask-capability-floor-policy`** — whether lower L-level ceilings are needed per target,
   and how policy composes without permissive defaults.
9. **`ask-aid-cross-host-steering`** — how influence over selection, ordering, and blame is
   rendered without letting a participant steer the operator's whole-estate response silently.
10. **`ask-dst-fleet-authority-cells`** — test every phase × admin/engineer × reliable/unreliable
    oracle cell under reorder, delay, duplicate, cancellation, wrong scope, and partial arrival.

## §13 — Security disposition after the sitting

~SUSPECT the human's closing takeaway is supportable: a thin layer of orchestration features
appears owed sooner than expected, and the associated security surface can be bounded more
tightly than an unrestricted cross-host model suggested.

+SURE the tighter bound does not mean the surface arrives later. Multi-host execution plus
fail-fast already crosses it. The design task is to constrain what travels through the channel,
not to pretend the channel starts only with advanced fleet features.

~SUSPECT the strongest medium-term containment is:

```text
authored sh remains the only source of actions and branches
remote outcomes may progress only through corresponding authored gates
unknown/integrity loss never rounds into permission
no cross-host semantic facts, values, omissions, or generated work are required
```

+SURE L3–L7 remain separate decisions. None is implied by implementing L2 progression.

+SURE this addendum does not authorize implementation, change a current invariant, or clear
the repeated-probing/authority-map/decision-identity review gates. Its purpose is to ensure a
future `26L` design pass cannot mistake ordinary fleet plumbing for a security-neutral refactor.
