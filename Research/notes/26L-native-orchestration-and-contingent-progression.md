# 26L — Native orchestration, foreign-plan views, and contingent progression

> Status: AI-authored exploration report from an interactive sitting with the human,
> 2026-08-24. The human directed the final emphases and exclusions. This is not a
> ruling, implementation plan, or amendment to the current r26 design. It records the
> changed headspace, the candidate medium-term plateau, the useful negative findings,
> and the questions owed before any design settles.
>
> Evidence posture: this sitting began from training-data recollection and the existing
> Dorc corpus, then grounded its abstractions in eight plain-sh strawmen. A live-doc and
> real-world-script pass was explicitly deferred: tool-specific assertions below are
> illustrative, not source-audited findings. The strawmen live at
> `Research/quarantine-DO-NOT-READ/26L-strawmen/` and are readable only by readers already
> granted access to that directory.
>
> Governing background: `plans/260`–`262` + `notes/26A`–`26C` (tabled r26 fleet,
> read-concurrency, and reactive direction) · `spike/CLAUDE.md:no-reorder-ever` ·
> `SIBLINGS.md` · root `README`/`DESIGN`/`IMPLEMENTATION`.

## §0 — Findings in one screen

- **`result-punt-now-not-forever`** — ~SUSPECT. The sitting increased confidence that
  Dorc owes a thin set of fleet/orchestration capabilities sooner than previously expected.
  Permanent host independence would forfeit ordinary glue and one-shot setup stories that
  remain central to the project. This is a changed lean, not a new weld.
- **`result-meta-orchestrator-over-do-engine`** — +SURE as a product description of the
  explored plateau. Dorc is closer to a meta-orchestrator than a conventional orchestrator:
  sh already performs the sequencing, captures, waits, retries, fallbacks, and cross-tool
  calls. Wherever Dorc orchestrates, it does so chiefly by analyzing and lifting the
  orchestration already authored in sh.
- **`result-rust-do-layer-stays-thin`** — ~SUSPECT. The sitting found almost no new
  fleet behavior that obviously belongs as a Rust-authored executor feature. Minimal native
  orchestration should preserve authored sh behavior; the primary Rust value remains the
  existing one: analysis, probing, plan construction, elision, provenance, and aid.
- **`result-minimal-pair-surprisingly-rich`** — ~SUSPECT. Approximately-L2 native
  orchestration plus an optional `cmd__plan_preview()` entrypoint appears to enable a much
  richer product than expected while remaining narrower in implementation scope than the
  wider fleet mechanisms explored earlier. This pairing needs a later real-world strawman
  and live-doc pass before it earns design weight.
- **`result-foreign-plans-are-attached-views`** — ~SUSPECT. The action unit can remain
  one authored shell line while the review unit becomes that line plus one or more
  source-attributed foreign projections. Terraform resources, Ansible tasks, and Kubernetes
  objects remain their tools' own units; Dorc does not decompose or normalize them.
- **`result-powerful-tools-elide-atomically`** — ~SUSPECT. The most valuable stretch
  goal in a multi-tool world may be ordinary straight-line elision of each powerful tool as
  one atomic mutator. If every action above it is converged and elided, the adapter need only
  answer whether this exact invocation is quiescent; no foreign footprint or internal plan
  model is required.
- **`result-contingent-progression-is-core`** — ~SUSPECT, strong lean. Defensive sh
  already expresses conditional admission, wait-until barriers, fail-stop prerequisites,
  fallbacks, and independent failure accumulation. Dorc can lift these into a richer model
  of user goals and pay the author back with earlier observation, elision, resumability,
  explanation, and probe parallelism. Coverage and apply-time correctness need focused work.
- **`result-slow-planners-open-performance-question`** — +SURE. Foreign convergence,
  prediction, and preview functions may each take minutes. A bevy of such tools can greatly
  lengthen both plan latency and the plan-to-apply interval, invalidating assumptions formed
  around cheap host-local probes. Duplicate planning and any generic pure-sh reuse helper are
  newly open questions.
- **`result-continuity-mildly-interesting-tabled`** — ~SUSPECT. An opt-in opaque
  preview-comparison tripwire might catch some late changes with little adapter-specific
  work, but semantic foreign-plan continuity remains upstream-owned. The mechanism was
  acknowledged as potentially useful and tabled with only mild interest.
- **`result-ecosystem-pressure-is-secondary`** — -GUESS, explicitly not a design
  driver. A usable consumer for `--is-converged`, stable plan identities, and machine-readable
  previews might encourage upstream tools to expose them, just as Dorc hopes to reward
  portable, shared sh. This is a pleasant possible externality, not product justification.

## §1 — Starting boundary and changed headspace

+SURE the current r26 design partitions plans by host: no host plan reads another host's
facts, and apply order within a host is never changed. Probe-time read parallelism and
cross-host fan-out are separate from apply reordering. `plans/260` additionally tabled
cross-host plan dependencies, rolling policy, shared facts, and memoization.

+SURE those scope cuts remain current. This sitting did not reopen implementation or turn
the tabled items into commitments.

~SUSPECT the product headspace changed nonetheless. A tool permanently unable to express
ordinary multi-tool progression would remain valuable as a per-host analyzer, but the word
"orchestrator" would increasingly describe its shell substrate rather than its product.
The preferred lean after this sitting is **punt for now**, not **punt forever**.

+SURE further boundary analysis is in
`Research/quarantine-DO-NOT-READ/26La-orchestration-boundary-addendum.md`; read it only if
already granted access to the quarantine.

## §2 — L0–L7 capability strata

The sitting used a deliberately broad ladder to keep distinct powers from collapsing under
"host A influences host B." These are exploration labels, not a proposed API or final
configuration vocabulary.

| layer | additional behavior | representative use |
|---|---|---|
| **L0 — isolated hosts** | each host attempt is independent | fan out one book; aggregate results after every attempt |
| **L1 — authored fleet graph** | source fixes ordering; an earlier outcome does not suppress a later attempt | run Advil, then always attempt Beverly |
| **L2 — closed outcome gates** | an earlier success/failure/readiness outcome admits or withholds already-authored later work | canary then fleet; configure only after provision succeeds |
| **L3 — fixed action selection** | an observation chooses one member of a closed, authored action set | run a migration on the reported leader from `{db-1, db-2}` |
| **L4 — bounded value courier** | an earlier action's output becomes data for a later action | create a join token or address, then consume it elsewhere |
| **L5 — shared relational model** | several participants contribute to one target's desired view | membership-derived proxy configuration; quorum/topology reasoning |
| **L6 — cross-host omission** | evidence from one scope permits work in another scope not to run | cross-host verdict reuse or equivalence-class probing |
| **L7 — post-consent adaptation** | runtime results create or rewrite later work not fixed at review time | discover a replacement target and synthesize new work for it |

### `finding-l1-l2-depends-on-failure-posture`

+SURE L1 and L2 are separable only under a deliberate failure posture. If Advil is a
predecessor but Beverly is always attempted after Advil returns, Advil's outcome does not
control Beverly's reachability. The moment fail-fast, a barrier, or a conditional uses
Advil's behavior to determine Beverly's attempt, the workflow has L2 behavior.

+SURE this is not obscure. Ordinary fleet execution plus ordinary fail-fast handling already
produces the important relationship. A "basic" fleet DAG is therefore not automatically an
outcome-independent substrate.

~SUSPECT a reduced version of the ladder may eventually be useful as an admin policy: even
if Dorc implements richer behavior globally, particular target sets may be intentionally
held to lower layers. No policy surface was designed here.

+SURE further boundary analysis of the strata is in
`Research/quarantine-DO-NOT-READ/26La-orchestration-boundary-addendum.md`; read it only if
already granted access to the quarantine.

## §3 — Plain-sh strawman exercise

Eight standalone sh sketches made the ladder concrete:

| file | exercised junction |
|---|---|
| `l0-isolated-hosts.sh` | both host attempts begin before either result is observed |
| `l1-authored-fleet-graph.sh` | Beverly is attempted after Advil regardless of Advil's outcome |
| `l2-closed-outcome-gates.sh` | Advil deploy+health controls whether Beverly is attempted |
| `l3-fixed-action-selection.sh` | a reported Patroni leader selects one approved migration target |
| `l4-bounded-value-courier.sh` | kubeadm token/hash values flow from control to worker argv |
| `l5-shared-relational-model.sh` | Consul-reported membership shapes an nginx upstream |
| `l6-cross-host-omission.sh` | one host's observation plus an equivalence claim omits another host's work |
| `l7-post-consent-adaptation.sh` | a failed apply creates a replacement target and new work at runtime |

+SURE all scripts are illustrative and deliberately gloss fixture inputs and operational
details. They were syntax-checked only and never executed.

~SUSPECT the exercise's main result was subtraction: sh already expresses nearly every
do-layer behavior. The hard questions are what Dorc can know before execution, what it can
remove from the review, and what user intent can be lifted without replacing the script's
behavior.

## §4 — Prior-art-shaped opening position

This section records training-data-shaped hypotheses only. A later source pass owns every
tool-specific claim.

~SUSPECT mainstream orchestrators and infrastructure tools cover pieces of contingent
progression extensively:

- Terraform-like systems own dependency progression inside their resource graph;
- Ansible-like systems own ordered host tasks, retries, rolling batches, delegation, and
  failure policies inside their model;
- Kubernetes-like systems own readiness and rollout progression inside the cluster;
- Puppet/Chef-like systems own per-node/resource convergence and notifications;
- cloud CLIs expose operations and wait/status commands but leave their composition to the
  caller;
- general-purpose Python/sh runners can express arbitrary cross-tool workflows but do not
  derive Dorc's plan/elision model.

~SUSPECT the persistent gap is the boundary between ownership domains: provision with one
tool, obtain an address or control plane, configure with another, wait using a third, deploy
through a fourth. No participant naturally owns the whole sh program that binds them.

+SURE live documentation, realistic team scripts, and homelab scripts are owed before this
framing becomes evidence. This pass should be delegated and context-budgeted separately.

## §5 — Dorc as meta-orchestrator

### `finding-sh-is-already-the-orchestrator`

+SURE sh already provides sequencing, branches, captures, loops, retries, fallbacks,
background work, waits, and process failure propagation. A defensive admin can write a
correct controller workflow without Dorc, and the off-ramp requires that this remain true.

~SUSPECT Dorc is therefore best described, in this explored world, as a
**meta-orchestrator**: it analyzes and narrows a workflow whose do-layer is authored sh and
whose large interior operations remain owned by sibling tools.

### `finding-elision-remains-the-rust-exception`

+SURE the clearest Dorc-owned orchestration behavior remains the existing one:

```sh
if curl --fail --silent "$endpoint/ready"; then
   expensive_section
fi
```

Given a suitable `curl` model, probe-sourced values, and stable control flow, Dorc can prove
the branch dead and remove the entire complicated section from the plan. The shell authored
the policy; Dorc did not execute it better, but used analysis and probing to save the admin's
attention.

~SUSPECT this pattern, rather than a new Rust scheduling language, is the core fleet value to
extend.

## §6 — Candidate enhancement one: minimal native orchestration

### `candidate-controller-side-book`

~SUSPECT the minimum useful substrate is a controller-side book whose ordinary leaves may
invoke local tools and explicit cross-machine commands. The source remains normal sh:

```sh
terraform ...
address=$(terraform output ...)
ansible-playbook -i "$address," ...
ssh "$address" ...
kubectl ...
```

+SURE the target behavior is deliberately unambitious:

- preserve source order and shell control flow;
- preserve the script's own fail-fast, fallback, and attempt-all choices;
- permit runtime values to flow as ordinary shell values;
- do not require an adapter before a tool can execute;
- do not infer cross-tool resource identity;
- do not reorder or parallelize mutations;
- do not create new work absent from the script.

~SUSPECT native orchestration alone is not differentiating; sh already performs it. It is
valuable as the substrate on which Dorc's existing probing, `is_converged`, plan, why, and
elision behavior can operate across controller tools.

### `finding-broad-execution-shallow-understanding`

+SURE this approach gives broad execution coverage and shallow semantic coverage. Weird,
private, or unsupported tool versions still run. Missing adapters cost previews and elision,
not basic operation.

~SUSPECT that is the right gradual-enhancement direction for a glue tool.

## §7 — Candidate enhancement two: `cmd__plan_preview()`

### `candidate-foreign-preview-entrypoint`

~SUSPECT a new optional `cmd__plan_preview()` role could let a tool specialist write the
same native preview a trained admin would run by hand. Under explicit admin opt-in, Dorc
would present that foreign output beneath the corresponding authored command in the
convergence review.

+SURE the explored value depends on keeping jobs separate:

- the authored shell line remains the do-layer unit;
- the foreign tool retains its own plan/task/resource units;
- the preview is a view, not a decomposition into Dorc actions;
- the native output remains available rather than being replaced by Dorc paraphrase;
- the preview does not initially license elision, facts, or cross-tool correspondences;
- `is_converged()` remains the ordinary whole-line omission license.

### `finding-review-unit-can-exceed-action-unit`

~SUSPECT the useful middle ground is:

```text
action unit: one authored command
review unit: that command + native preview + source/context + shell dependencies
```

This lets one review surface include Terraform's own plan, Ansible's own check/diff, or a
tool's own dry-run without pretending Dorc understands their interiors.

### `finding-preview-maturity-is-independent`

~SUSPECT adapters can mature independently:

| available members | result |
|---|---|
| neither | opaque command runs |
| preview only | command remains, native preview shown |
| convergence only | command may elide atomically |
| both | converged command disappears; diverged command carries its native preview |

This gives maintainers a display-only contribution before they accept the stronger judgment
involved in `is_converged()`.

## §8 — Straight-line elision of powerful tools

### `finding-atomic-foreign-elision-is-moderate`

~SUSPECT pure straight-line elision of powerful tools is much less ambitious than surviving
past one after it runs. Treat each invocation as one giant atomic mutator:

```text
tool A converged -> A elides -> world remains as probed
tool B converged -> B elides -> world remains as probed
tool C converged -> C elides
```

+SURE this needs no understanding of internal resources or effects. The adapter only judges
whether rerunning this exact invocation is noise it accepts.

+SURE if A is diverged, A remains and becomes a wall. Without foreign footprints or wider
machinery, downstream attention degrades honestly to guard/run. This is ordinary Dorc
behavior and intentionally preserves the difficult boundary.

### `finding-steady-state-attention-is-the-payoff`

~SUSPECT a five-tool glue script collapsing to the one subsystem actually diverged may be
the highest-value multi-tool outcome discussed. It answers the admin's concrete question:

> Which tool, if any, requires my attention today?

~SUSPECT powerful tools may be favorable oracle targets because many already own a desired
state model and some native planning/status surface. Whether those surfaces adequately
support `is_converged()` is the later per-tool research question.

## §9 — Fleet knowledge as sh-spelled procedure summaries

### `finding-controller-tools-are-foreign-procedures`

~SUSPECT the current oracle analogy extends one level upward:

```text
external command = foreign procedure
oracle = reusable sh-spelled summary

external orchestrator = very large foreign procedure
fleet adapter = reusable sh-spelled target/effect/result summary
```

+SURE Dorc must not reimplement the interior graph. The adapter should delegate to native
inventory, plan, output, and status surfaces wherever possible.

### `finding-thin-glue-carries-the-correspondence`

+SURE thin glue often writes the mapping directly:

```sh
address=$(terraform output -raw web_ip)
ansible-playbook -i "$address," configure.yml
```

The shell value flow already says that the producer's output is consumed as the later tool's
inventory. Dorc need not invent a global name-equivalence relation to discover the edge.

### `finding-shareable-versus-local-fleet-knowledge`

~SUSPECT the shareable part is per-tool: argv grammar, invocation context, native target
enumeration, status meanings, output forms, and supported/declined shapes. The local part is
the admin's policy: which project/cluster/inventory matters, which output feeds which consumer,
which failures stop which work, and what readiness criterion is accepted.

+SURE the analogy fails if Dorc needs models for every Terraform provider, Kubernetes CRD,
Ansible module, or organization-specific topology. In that world the supposed adapter has
become a second implementation of the siblings.

## §10 — Contingent progression

### `finding-contingencies-have-distinct-intents`

+SURE several common sh shapes encode different goals:

| authored shape | lifted intent |
|---|---|
| `if ready; then deploy; fi` | deployment is conditionally desired |
| `ready || exit; deploy` | readiness is a fail-stop prerequisite |
| `until ready; do sleep; done; deploy` | deployment remains desired; readiness delays it |
| `attempt_a || attempt_b` | failure selects an authored fallback |
| `attempt_a; record_failure; attempt_b` | attempts are independent despite failure |
| `changed && restart` | this run's delta controls a later action |
| background work followed by `wait` | completion and concurrency jointly gate continuation |

~SUSPECT lifting these distinctions is more valuable than flattening them into one generic
dependency edge.

### `finding-team-glue-shapes`

+SURE team glue commonly needs:

- provision, wait for reachability, then configure;
- establish a control plane, then invoke its client;
- bootstrap one member, then join others;
- migrate or publish an artifact before rollout;
- drain, update, verify, and readmit;
- canary, verify, then continue;
- continue independent work while withholding true dependents.

### `finding-homelab-glue-shapes`

+SURE Dorc-only and homelab use commonly needs:

- start a VM, NAS, container, or daemon and wait for its interface;
- reboot a machine and continue when it returns;
- prepare one small-cluster control node before workers;
- establish networking, DNS, certificates, or mounts before consumers;
- snapshot, upgrade, verify, then clean up;
- attempt available personal machines while retaining failures for later.

### `finding-shell-already-does-readiness`

+SURE Dorc does not need to implement waiting or readiness when the admin already wrote:

```sh
until curl --fail --silent --max-time 3 "$endpoint/ready"; do
   sleep 2
done
next_action
```

Shell executes this correctly. Dorc adds value only by lifting the control relation and the
oracle-backed predicate, not by synthesizing a different loop.

### `finding-lifted-require-relations`

~SUSPECT the useful model enrichment is a `Require`-shaped relation:

```text
downstream action B requires world predicate F
```

The shell CFG establishes which outcome controls B; an oracle can establish what F reads and
whether it is safe to probe. This composes admin-authored policy with shareable tool knowledge
without adding dependency configuration.

### `finding-contingent-progression-payback`

~SUSPECT Dorc can repay defensive authorship through:

1. exposing which failures, waits, and fallbacks control which later actions;
2. lifting safe predicates for already-existing, stable targets into the probe phase;
3. distinguishing a false conditional from a false wait-until barrier;
4. resolving world-specific branches only where the observation remains applicable;
5. eliding completed stages on later runs;
6. resuming statelessly from the first genuinely unresolved stage;
7. explaining non-execution at the authored predicate and its dependent lines;
8. probing independent readiness predicates concurrently.

### `finding-future-values-remain-future`

+SURE no mechanism discussed today lets an admin review a concrete `$ip` before the
provisioning action produces it. Before creation, Dorc can show the authored recipe and its
data/control edges, not the future concrete target. Concrete pre-creation review would require
prediction or another planning/consent moment and was not solved here.

### `finding-contingent-progression-needs-focused-coverage`

~SUSPECT contingent progression is close to a must-implement concern for native
orchestration. A small set of representative shapes needs explicit acceptance coverage and a
coherent apply-time correctness story before fleet implementation begins. This does not imply
new wait/retry syntax or a Rust workflow language.

+SURE further boundary analysis of contingent progression is in
`Research/quarantine-DO-NOT-READ/26La-orchestration-boundary-addendum.md`; read it only if
already granted access to the quarantine.

## §11 — Slow planners and the widened plan-to-apply interval

### `question-foreign-planners-change-the-cost-model`

+SURE the existing probe architecture anticipates many parallel, comparatively small reads.
Complex controller tools can invert that assumption. An `ansible__is_converged`,
`ansible__predict`, and `ansible__plan_preview` might each take several minutes; several
different tools may repeat similarly expensive work.

This opens several questions:

- Is one native planner invocation reusable across convergence, prediction, and preview?
- Can any generic reuse helper remain plain sh and off-ramp-compatible?
- Who owns the temporary artifact or output when one member feeds another?
- How are volatile/noisy plan outputs handled without pretending equivalence?
- When does duplicate planning cost more than the apply work it hoped to avoid?
- How does parallelizing several heavy planners affect their shared control planes?

### `question-planning-duration-is-staleness`

+SURE the planning duration itself contributes to the plan-to-apply interval. If building a
plan takes fifteen minutes, the earliest observations are already fifteen minutes older at
the consent point. "Just make a new plan" becomes recursively expensive when planning is the
slow operation.

~SUSPECT lightweight apply-boundary checks, native saved decisions, or explicit acceptance
of the enlarged interval may each be valid for particular tools. There is no generic answer
yet, and current TOCTOU scope remains unchanged.

+SURE further boundary analysis of slow planners and late checks is in
`Research/quarantine-DO-NOT-READ/26La-orchestration-boundary-addendum.md`; read it only if
already granted access to the quarantine.

## §12 — Continuity exploration, now tabled

### `negative-universal-continuity-is-not-an-oracle-job`

~SUSPECT asking an adapter maintainer to parse arbitrary foreign output, reconstruct the
foreign plan, persist it, devise a faster recheck, and prove equivalence at apply is
unmaintainable. Community effort amortizes that work but does not make it a reasonable
general oracle contract.

+SURE Dorc can help with generic custody, exact invocation context, bounded capture,
comparison, provenance, and halting. It cannot invent semantic sameness or a cheaper native
freshness primitive.

### `finding-opaque-preview-tripwire-may-be-useful`

~SUSPECT an explicit admin mode could treat exact `plan_preview()` output as an opaque
witness, rerun it immediately before the first mutation on a pristine straight-line prefix,
and halt if bytes differ. This offers only a hole-filled change tripwire:

- cosmetic changes may halt unnecessarily;
- unchanged bytes may miss real differences;
- the foreign preview may omit behavior both times;
- the final interval between recheck and apply remains.

The adapter author supplies no continuity-specific parser; Dorc owns the generic envelope and
comparison. A differential harness could help maintainers measure output stability and press
upstreams toward stable plan identities.

~SUSPECT this may be useful tooling. The human acknowledged mild interest and tabled it; it
must not drive the medium-term design.

## §13 — Useful negative findings and explored dead ends

The sitting spent meaningful time on the following and should not force a future reader to
re-derive them as fresh ideas.

- **`negative-foreign-plan-decomposition`** — +SURE. Turning foreign resources/tasks
  into Dorc plan lines defeats composition and asks Dorc to subsume the sibling. Keep foreign
  interiors atomic.
- **`negative-cross-host-memoization-value`** — ~SUSPECT. Reusing one host's verdict to
  omit another host's work buys mostly probe performance while demanding broad new machinery.
  Poor near-term value.
- **`negative-foreign-effect-survival`** — ~SUSPECT. Maintaining elision after a
  diverged powerful tool runs requires foreign footprints and cross-tool state relations. Much
  more ambitious than straight-line atomic elision; not a medium-term target.
- **`negative-global-fleet-model`** — ~SUSPECT. Dynamic membership, quorum, topology,
  and global desired-state reasoning pull Dorc toward the siblings' own control-plane models.
  No demonstrated need for Dorc to own them.
- **`negative-magical-resource-typing`** — +SURE for the explored plateau. No automatic
  type-level correspondence between Terraform resources, host addresses, Ansible inventory,
  Kubernetes nodes, and similar entities is needed for native orchestration or preview.
- **`negative-post-consent-plan-invention`** — ~SUSPECT. Generating new work after
  review conflicts with the present one-plan/one-consent posture and was low-value relative to
  authored runtime shell.
- **`negative-three-capability-handoff-plateau`** — +SURE as a correction. An earlier
  synthesis claimed foreign-decision continuity, opaque handoff continuity, and bounded
  contingent progression were jointly required. This overclaimed: shell already carries
  runtime values and readiness gates, while the proposed machinery did not make future values
  reviewable. The package was retracted.
- **`negative-rust-readiness-engine`** — +SURE. Dorc does not need to implement a
  readiness engine when authored shell already performs the wait correctly. The value is in
  lifting and rewarding the shape.
- **`negative-source-specific-conclusions`** — +SURE. Tool-by-tool planner suitability
  was not investigated deeply enough to retain conclusions. A later source pass owns it.

## §14 — Candidate medium-term plateau

~SUSPECT the most coherent plateau emerging from the sitting is:

```text
minimal controller-side native orchestration
+ ordinary per-tool is_converged
+ optional per-tool plan_preview
+ focused contingent-progression analysis and acceptance coverage
+ current straight-line wall behavior
```

+SURE the plateau intentionally excludes foreign plan decomposition, resource typing,
foreign-effect survival, cross-host omission, global fleet state, mutation reordering, and
post-consent work invention.

~SUSPECT this may still buy substantial real-world value:

- any weird tool remains executable;
- quality adapters can remove quiescent heavyweight lines;
- surviving heavyweight lines can carry native previews;
- authored progression remains ordinary sh;
- defensive checks become model inputs rather than dead boilerplate;
- later runs resume by re-measuring completed stages;
- one review surface shows the glue rather than replacing its siblings.

## §15 — Owed investigation before design

1. **`owed-real-glue-strawmen`** — gather a small set of realistic team and homelab
   controller scripts; identify the actual contingent shapes and first permanent walls.
2. **`owed-live-sibling-docs`** — source-read the native preview, convergence, saved-plan,
   check-mode, output, and readiness surfaces of the dominant sibling tools. Delegate this
   context-heavy pass.
3. **`owed-minimal-native-runner-shape`** — establish what "native orchestration" means
   at the floor without importing inventory, resource, or scheduler subsystems.
4. **`owed-preview-display-contract`** — determine where preview output lives, how native
   output remains available, how decline works, and how the admin opts in; do not conflate
   review overlay with executable artifact bytes.
5. **`owed-contingent-shape-corpus`** — pin conditional, fail-stop, barrier, fallback,
   independent-error, change-trigger, and background/wait forms, including dynamic-target
   cases that must remain runtime-only.
6. **`owed-heavy-planner-costs`** — measure planner duration, repeated-member cost,
   shared control-plane contention, and the widened plan-to-apply interval.
7. **`owed-adapter-effort-measurement`** — compare `is_converged`, `predict`, and preview
   authoring across a few powerful tools; identify where upstream interfaces make adapters
   small versus permanently brittle.
8. **`owed-product-output-trial`** — show a grumpy admin the combined shell plus native
   foreign views; verify it is strictly better than manually running the same commands rather
   than an interposed gloss.

## §16 — Status and non-rulings

| conclusion | status after the sitting |
|---|---|
| punt all cross-host influence forever | weakened; lean is now punt-for-now |
| thin orchestration features are owed earlier | human-stated closing takeaway; mechanism unruled |
| minimal orchestration + preview deserves investigation | positive, tentative; real-world pass owed |
| sh is the primary do-layer | strong shared conclusion |
| contingent progression deserves focused work | positive, near-must concern; no implementation authorized |
| straight-line atomic elision of powerful tools | promising stretch goal; per-tool adequacy unmeasured |
| foreign plan decomposition | rejected direction |
| opaque continuity tripwire | mildly interesting and tabled |
| cross-host omission/memoization | low-value near-term direction |
| global resource/fleet model | unsupported by this sitting |

+SURE no code implementation, root-doc amendment, or existing r26 ruling change follows from
this note. Its purpose is to make the next research/design sitting start from the useful
plateau and negative findings rather than replay today's full conversation.
