# SIBLINGS fragment: Kubernetes (+ Helm)

> Fragment for root `SIBLINGS.md`, r26 writing phase. Rows are ARCHITECTURE-tier only —
> fundamental decisions and their lock-in/lock-out — never NYI/someday rows either side
> could close with implementation work. Written from this builder's own k8s research
> (citations live in the two companion notes in this directory); the conductor merges and
> adds columns from turns B/D.

## Posture

Kubernetes is a **Big Boy**, and the cut applies at its strongest: choosing between
Kubernetes and Dorc is not a decision anybody should be making. No sane person should
persistently choose Dorc over Kubernetes. If you are orchestrating containerised workloads
across more than one machine, Kubernetes is the answer and we will say so in the README.

Dorc is for the residue — and the residue around Kubernetes is unusually well-defined,
because Kubernetes draws its own boundaries explicitly rather than by omission:

1. **Below the cluster.** Kubernetes reconciles workloads *onto* nodes. It does not
   manufacture nodes. A machine must already exist, already run a container runtime, and
   already hold credentials before a single Kubernetes object can describe it. That gap is
   not a missing feature; the kubelet is a precondition of the API, so the API cannot
   bootstrap it.
2. **Inside the manifests.** Kubernetes governs workload *health* — probes, restarts,
   rollouts, reconciliation. It does not govern the *content* of the shell you put in an
   init container, an entrypoint, or a lifecycle hook. It tells you that content must be
   idempotent and gives you nothing to make it so, and in the init-container case it
   refuses at validation to let you even attach a readiness predicate.

Both are architecture, not backlog. Neither is a criticism.

## Rows

Legend: **Y** the product does this as a fundamental capability · **N** it does not, and
the decision is architectural · **~** partial, with the qualification stated.

### Where Kubernetes has it and Dorc does not (the priority rows)

| Capability / decision | Kubernetes | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| Continuous reconciliation with nobody present | **Y** | **N** | Controllers run forever and repair on node loss. Dorc is push-only and converges exactly when you run it (`kAGENTLESS` welded). Nothing on either side can move without becoming the other product. |
| An agent on every managed node | **Y** (kubelet, non-optional) | **N** | This is the *cost* Kubernetes pays for the row above, not a defect. Declarative reconciliation at scale requires a resident process; that is why the honest agentless-floor claim positions against the Big Boys and says "their one guaranteed cost is the agent", never "we are better without one". |
| Desired state as a durable, queryable, watchable API | **Y** (API server + etcd) | **N** | Dorc holds nothing between runs and re-measures from host reality (`kSTATE` parked; the probe TAPE is a write-only postmortem, never a cache). There is no `kubectl get` for Dorc and there is not going to be one. |
| Scheduling, placement, bin-packing, eviction | **Y** | **N** | Dorc has no notion of a workload that could be placed. |
| Replicas: N identical instances as a first-class object | **Y** | **N** | A Dorc book is per-host imperative work. "Replica" is not in the vocabulary and would not fit it. |
| Service discovery, cluster networking, load balancing | **Y** | **N** | Different layer entirely. |
| Rollout orchestration: surge, rolling update, rollback, canary | **Y** | **N** | A book applies in order and stops. Rollback would require the state store Dorc refuses. |
| Multi-tenancy: RBAC, admission control, policy, quotas | **Y** | **N** | Dorc's authorization model is whatever your ssh and sudo already are. It adds no boundary of its own — deliberately, since adding one would mean owning identity. |
| Immutable deployment unit | **Y** (the image) | **N** | Mutable host state is Dorc's premise. The two are opposite bets; the immutable one is usually right, which is why the Dorc bet is scoped to where you cannot take it. |
| Extension model: a programmable API (CRDs + controllers) | **Y** | **N** | Dorc's extension model is *publishing a shell file*. Enormously cheaper to author, enormously less powerful, and not convertible into the other. |
| Estate-wide declarative diff | **Y** (GitOps reconcilers, `kubectl diff`) | **~** | Dorc's plan is per-host and per-book. Comparable only within one book's scope. |

### Where Dorc has it and Kubernetes does not (stated friendly, because these are the residue)

| Capability / decision | Kubernetes | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| **Governing the content of a script** | **N**, by design | **Y** | The sharpest row in the table, and Kubernetes says it itself: init-container code *"should be idempotent"*, while `readinessProbe` on an init container is *"prohibited... This is enforced during validation."* Probes and operators govern workload health; nothing governs what the shell inside actually does. Kubernetes knows how to attach a readiness predicate to an init-position container — sidecars get one — and declines for run-to-completion ones on principle, because completion *is* the predicate. The hole is deliberate, permanent, and exactly Dorc-shaped. |
| Bringing a node into existence | **N**, structurally | **Y** | The kubelet is a precondition of the API, so no Kubernetes object can describe the machine that does not yet run one. Everything before `kubeadm join` returns is somebody's shell script. |
| Operating on a machine that is in no cluster | **N** | **Y** | The bastion, the two on-prem edge boxes, the GPU node nobody rebuilds, the laptop. |
| Zero infrastructure of its own | **N** (control plane, etcd, CNI, CSI, ...) | **Y** (a binary and a byte-pipe) | This is *why* Dorc can sit in the residue at all: it brings no estate to contest with yours, no state file to lock or strand, nothing to run alongside your control plane. |
| Trivial off-ramp | **N** | **Y** | A manifest is meaningless without a cluster; leaving Kubernetes is a rewrite. `dorc strip` leaves a shell script that runs anywhere. Friendly framing that must survive editing: **Kubernetes' lock-in is what buys row 1 above.** It is a fair trade and usually the right one. Dorc can offer the off-ramp *only because* it refuses to offer continuous reconciliation. |
| The reviewed artifact is the executed artifact, all the way down | **~** | **Y** | Manifests are reviewed and Kubernetes applies them faithfully — but the imperative residue *inside* them (init scripts, entrypoints, `command:` arrays, lifecycle hooks) is reviewed as an opaque string and executed with no further scrutiny. Dorc's plan is the book's own bytes. |

### Contested, or simply different products

| Question | Kubernetes | Dorc | Note |
| --- | --- | --- | --- |
| Idempotence of the imperative escape hatch | mandated by doc, assisted by nothing | the entire product | Kubernetes is the sixth member of the mandate-idempotence-assist-nothing family the round has been collecting, and the only one whose refusal is an admission-time validation error. |
| Waiting for a condition | **Y**, and better | delegates | `kubectl wait --for=condition=...` and readiness gates are first-party wait verbs. A Dorc book should call them, not reimplement them; that is a delegation-oracle target and a happy-parent line. |
| Attention per apply | not applicable | the product | A declarative estate has nothing to read per-apply, which is a strictly better answer where you can have it. Dorc's plan exists for the imperative work that remains. |
| Bounded-retry shell in the wild | ships the bug | can lint it | Kubernetes' own init-container documentation gives `for i in {1..100}; do sleep 1; ...; done` under `sh -c` in a busybox image, where the brace range is a literal and **the loop runs once**. Independent of the k3s installer carrying the same bug. A shape-recognizer here is a genuine, cheap Dorc win — over the reference documentation, not over Kubernetes. |

## Helm (escape-hatch posture)

Helm is a packaging and release layer over the same substrate; the Big-Boy framing carries.
Rows worth having:

| Capability / decision | Helm | Dorc | Why it is architecture |
| --- | --- | --- | --- |
| Templating | **Y** (Go templates over YAML) | **N**, by design | Our answer is different in kind, not lesser: heredocs are the templating, and branch-on-probed-facts is the inventory. This is a TRUE-DON'T-BUILD row on both sides of the table. |
| Release state stored in the cluster | **Y** (`helm.sh/release.v1` Secrets) | **N** | Enables `helm rollback` and `helm history`; costs a state store that can go stale, be locked, or be lost. Dorc buys the opposite trade. |
| Rollback to a previous release | **Y** | **N** | Follows directly from the row above. |
| Blocking wait on readiness | **Y** (`--wait`, `--wait-for-jobs`, and hooks block until Ready) | delegates | Same delegation-oracle story as `kubectl wait`. |
| Imperative escape hatch | **~** | **Y** | Helm hooks are *Kubernetes resources* — usually a Job or Pod whose payload is `command:` argv on an image, not raw sh handed to you. So the dorc-inside seat in Helm is **thin, not dead**: it is one level down, inside the image the hook Job runs, which is the same seat as the init-container book in this directory. Helm never gives you a shell; the container does. |
| Hook resource lifecycle | **~** | n/a | Worth carrying as an honest note rather than a scoring row: *"The resources that a hook creates are currently not tracked or managed as part of the release... you cannot rely upon `helm uninstall` to remove the resources."* Dorc does not fix this and should not claim to. |
| Dry-run surface | **Y** (`helm upgrade --dry-run`) | **Y** (the plan) | Different visibility, not different quality: Helm's dry-run renders templates and shows the declared diff, and is blind to the imperative content of hooks and entrypoints by construction. Dorc's plan is blind to everything outside its book. Neither subsumes the other. |

## "If you are doing X, just go use Y" candidates from this domain

Per-feature only; the maturity/community version of this line is product-tier framing and
belongs to the human's voice, not ours.

- Running containerised workloads on more than one machine → **Kubernetes**. Not close.
- Packaging an application for other people to install into a cluster → **Helm**.
- Keeping a cluster's declared state matching git → a **GitOps reconciler**. Dorc ceded the
  outer reconcile loop in this round and should stay ceded.
- Nodes that are cattle — an image plus a scale group → **your cloud's node pools**. If your
  worker is rebuilt rather than repaired, the node-standup book in this directory is a
  problem you already solved better.
- Making a *pet* node reproducible, or fixing one at 3am → this is the residue, and it is
  ours.

## Wording fences for the merge

- **Never claim the agentless floor as a differentiator against the glue-tool siblings**
  (pyinfra/cdist/ansible all sit on ssh + sh). Against the Big Boys it is real, and the
  correct wording is that declarative reconciliation at scale *requires* an agent — so the
  agent is their necessary cost, not their mistake.
- **Never frame row-1 lock-in as a Kubernetes flaw.** The table above deliberately pairs
  each Dorc "Y" with the Kubernetes capability it is the price of.
- **The "governs script content" row is the only one where Dorc beats Kubernetes at
  something Kubernetes plausibly cares about**, and even that is a boundary Kubernetes drew
  on purpose. State it as a seam we fill, never as a gap they failed to close.
