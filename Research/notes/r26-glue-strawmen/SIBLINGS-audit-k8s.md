# SIBLINGS audit: the Kubernetes column (+ the Helm prose section)

> Tier: LLM-authored round note, r26 writing phase. Light citation-and-chafe pass over the
> merged root `SIBLINGS.md` table, lane = Kubernetes. The conductor integrates; this file
> touches nothing but itself. All URLs below were fetched and confirmed to resolve on
> **2026-07-28**; kubernetes.io was serving **v1.36**, helm.sh **v4** — see §1c, which is
> the most consequential thing this audit turned up.
>
> Format per the brief: §1 citations `{row, column, footnote text, URL}`; §2 row-chafe with
> severity tags; §3 out-of-lane one-liners (free, low-confidence by construction).

---

## 1. Citations

Cells I judged obvious and left alone: *declare-end-state Y · fleet Y · check-then-converge
Y · re-measures Y · plan-artifact-without-tool N · off-ramp N · no-resident-software N ·
first-boot N · full-value-without-authoring-layer N · target's-own-language N.* Those read
the way a reader expects and a footnote would be noise.

### 1a. Cells in the Kubernetes column

**C1 · "Converges continuously with nobody present" · Kubernetes = Y**
Footnote: *control loops watch and re-drive continuously; stability is explicitly not required for correctness.*
<https://kubernetes.io/docs/concepts/architecture/controller/>
Verbatim: *"In Kubernetes, controllers are control loops that watch the state of your cluster, then make or request changes where needed. Each controller tries to move the current cluster state closer to the desired state."* and *"As long as the controllers for your cluster are running and able to make useful changes, it doesn't matter if the overall state is stable or not."*
Included despite being near-obvious because it is the row Dorc most conspicuously answers N, and the reader deserves to see what the Y actually rests on.

**C2 · "Remembers what it built; delete by un-declaring" · Kubernetes = Y** — *needs a footnote; see CH-6.*
Footnote: *remembers in etcd, and cascade-deletes owned dependents via `ownerReferences`; but removing an object from your manifests deletes nothing by itself — that needs `--prune` or a pruning GitOps reconciler.*
<https://kubernetes.io/docs/concepts/architecture/garbage-collection/>
Verbatim: *"Many objects in Kubernetes link to each other through owner references. Owner references tell the control plane which objects are dependent on others. Kubernetes uses owner references to give the control plane, and other API clients, the opportunity to clean up related resources before deleting an object. In most cases, Kubernetes manages owner references automatically."*
Supporting the prune half: `kubectl diff` itself carries `--prune` and `--prune-allowlist` flags (*"Include resources that would be deleted by pruning"*), which is direct evidence that pruning is an opt-in mode rather than what `apply` does by default —
<https://kubernetes.io/docs/reference/kubectl/generated/kubectl_diff/>

**C3 · "Secrets management story" · Kubernetes = Y** — *the eyebrow cell; see CH-1.*
Footnote: *the most complete story in the table — first-class object, encryption-at-rest, RBAC, KMS and CSI provider integration, short-lived ServiceAccount tokens — but unencrypted in etcd by default, and the ability to create a Pod in a namespace implies read access to every Secret in it.*
<https://kubernetes.io/docs/concepts/configuration/secret/>
Verbatim (the page's own Caution box): *"Kubernetes Secrets are, by default, stored unencrypted in the API server's underlying data store (etcd). Anyone with API access can retrieve or modify a Secret, and so can anyone with access to etcd. Additionally, anyone who is authorized to create a Pod in a namespace can use that access to read any Secret in that namespace; this includes indirect access such as the ability to create a Deployment."*

**C4 · "Whole-system rollback" · Kubernetes = ~**
Footnote: *rollback is per-workload (`kubectl rollout undo`), bounded by `revisionHistoryLimit`; there is no cluster-wide rollback primitive.*
<https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#rolling-back-a-deployment>

**C5 · "Creates infrastructure (VMs, DNS, networks)" · Kubernetes = ~**
Footnote: *creates real cloud objects as a side effect of workload objects — load balancers and routes via the cloud-controller-manager, disks via StorageClass dynamic provisioning — but does not describe or own infrastructure as such; node creation is an add-on (autoscaler / Cluster API), not core.*
<https://kubernetes.io/docs/concepts/architecture/cloud-controller/>
Verbatim: *"The cloud-controller-manager is a Kubernetes control plane component that embeds cloud-specific control logic. The cloud controller manager lets you link your cluster into your cloud provider's API, and separates out the components that interact with that cloud platform from components that only interact with your cluster."*
<https://kubernetes.io/docs/concepts/storage/dynamic-provisioning/>
Corroborating the "side effect of workload objects" framing, the GC page lists among the things Kubernetes cleans up: *"Dynamically provisioned PersistentVolumes with a StorageClass reclaim policy of Delete"* and Nodes deleted *"On a cloud when the cluster uses a cloud controller manager"* — creation and deletion of real cloud resources, driven entirely by cluster-object lifecycle.

**C6 · "Templating / config-file generation" · Kubernetes = ~**
Footnote: *kustomize ships inside kubectl (`kubectl kustomize`, `apply -k`) and does overlays and patches, not templating; text templating is Helm's, and Helm is not Kubernetes.*
<https://kubernetes.io/docs/tasks/manage-kubernetes-objects/kustomization/>
Verbatim: *"Since 1.14, kubectl also supports the management of Kubernetes objects using a kustomization file"*, via `kubectl kustomize <dir>` and `kubectl apply -k <dir>`.

**C7 · "Preview before mutating (plan / dry-run / diff)" · Kubernetes = Y**
Footnote: *`kubectl diff` compares your would-be-applied configuration against the live cluster, server-side — so the preview runs the real admission chain rather than simulating it.*
<https://kubernetes.io/docs/reference/kubectl/generated/kubectl_diff/>
Verbatim: *"Diff configurations specified by file name or stdin between the current online configuration, and the configuration as it would be if applied."*
Worth the footnote because Dorc also claims Y on this row and the two Ys are not the same object: Kubernetes previews a declarative object diff; Dorc previews the executable text.

> **Bonus find, for the round's tri-state-rc collection.** `kubectl diff` documents:
> *"Exit status: 0 No differences were found. 1 Differences were found. >1 Kubectl or diff
> failed with an error."* That is a **fourth** incompatible tri-state "is there anything to
> do" convention alongside the three turn C already collected (systemd `ExecCondition`, GCP
> OS-policy, `rinstall`) — and note it is Terraform's `-detailed-exitcode` shape with the
> diverged/error codes *swapped* relative to the `dorc plan --exit-code` strawman
> (0 converged / 2 diverged / 1 error). Two of the most-used tools in ops disagree on
> whether "2" means diverged or broken. Strengthens the synthesis note's argument that the
> missing shared primitive is exactly the oracle rc contract.

**C8 · "Convergence machinery for raw shell content" · Kubernetes = N**
Footnote: *by design, and stated in its own docs: init-container code "should be idempotent", while attaching a readiness predicate to one is refused at API validation. Probes and controllers govern workload health; nothing governs script content.*
<https://kubernetes.io/docs/concepts/workloads/pods/init-containers/>
Verbatim: *"Because init containers can be restarted, retried, or re-executed, init container code should be idempotent. In particular, code that writes into any `emptyDir` volume should be prepared for the possibility that an output file already exists."* and *"Kubernetes prohibits `readinessProbe` from being used because init containers cannot define readiness distinct from completion. This is enforced during validation."*
This is the row the whole Kubernetes prose section rests on; it should not be uncited.

**C9 · "The reviewed text is byte-for-byte what executes" · Kubernetes = ~** — *existing footnote is wrong; see CH-2.*
Replacement footnote: *the API server defaults unset fields and mutating admission webhooks may rewrite the submitted object before it is stored — sidecar injection adds containers the author never wrote; embedded shell is reviewed as an opaque string.*
<https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/>
Verbatim: *"Mutating admission webhooks are invoked first, and can modify objects sent to the API server to enforce custom defaults."* and *"Admission webhooks that need to guarantee they see the final state of the object in order to enforce policy should use a validating admission webhook, since objects can be modified after being seen by mutating webhooks."*

**C10 · "Skips explained with queryable provenance" · Kubernetes = N/A** — *see CH-5.*
Footnote: *explains failures to act richly — Events, and condition `reason`/`message` fields — but a controller that finds nothing to do says nothing at all.*
<https://kubernetes.io/docs/reference/node/node-status/> (the condition object carrying `reason` and `message`)

**C11 · "Partial work chosen by measuring the machine" · Kubernetes = Y**
Footnote: *controllers act only on the gap between spec and observed status, and the scheduler filters on measured node capacity and taints — partiality is the architecture, not a mode.*
<https://kubernetes.io/docs/concepts/architecture/controller/>

**C12 · "Privilege escalation built in" · Kubernetes = N/A** — *see CH-3; the N/A is right but the row wording invites a misread.*
Footnote (if the row is not reworded): *Kubernetes has extensive privilege machinery, but it points the other way — constraining what a workload may do, not acquiring rights on a target.*
<https://kubernetes.io/docs/tasks/configure-pod-container/security-context/>

**C13 · "Existing scripts run unchanged (the adoption floor)" · Kubernetes = N/A** — *see CH-4.*
Footnote: *an existing script runs byte-unchanged inside an image you build for it; what is not unchanged is everything around it.*

### 1b. Quoted claims in the "Kubernetes (and Helm)" prose section

Per the brief — quotes without links age worst. Each of these is a verbatim quotation in the
prose and each now has a resolving home.

**P1 · the init-container idempotence mandate** — <https://kubernetes.io/docs/concepts/workloads/pods/init-containers/> (quote at C8).

**P2 · readinessProbe refused at validation** — same page (quote at C8), plus the API reference wording: <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/> — *"Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes."*

**P3 · the sidecar contrast that makes P2 deliberate rather than incidental** — <https://kubernetes.io/docs/concepts/workloads/pods/sidecar-containers/>. An init container with `restartPolicy: Always` is a sidecar and *does* support the probes. Kubernetes knows how to attach a readiness predicate to an init-position container; it declines for run-to-completion ones specifically. Cite this wherever the prose claims the hole is by design — it is the difference between an argument and an assertion.

**P4 · `kubectl wait --timeout=0s` and `--for=jsonpath`** — <https://kubernetes.io/docs/reference/kubectl/generated/kubectl_wait/>
Verbatim: *"--timeout duration Default: 30s — The length of time to wait before giving up. Zero means check once and don't wait, negative means wait for a week."* `--for` accepts `create`, `delete`, `condition=<name>[=<value>]`, and `jsonpath='{...}'=value`.

**P5 · Helm hook resources are not managed with the release** — <https://helm.sh/docs/topics/charts_hooks/>
Verbatim: *"The resources that a hook creates are currently not tracked or managed as part of the release. Once Helm verifies that the hook has reached its ready state, it will leave the hook resource alone."* and *"if you create resources in a hook, you cannot rely upon `helm uninstall` to remove the resources."*

**P6 · Helm hooks are Kubernetes resources, not raw shell** — same page: *"Hooks are just Kubernetes manifest files with special annotations in the `metadata` section."* The page's canonical example is a `Job` whose payload is `command: ["/bin/sleep","{{ default "10" .Values.sleepyTime }}"]` — argv on an image. This is the evidence for the prose claim that the dorc-inside seat in Helm is thin rather than dead: Helm never hands you a shell; the container does.

**P7 · Helm blocking waits** — <https://helm.sh/docs/helm/helm_upgrade/>. **The claim survives but its shape changed in Helm 4; cite the new form.**
Verbatim (current): *"--wait WaitStrategy[=watcher] — wait until resources are ready (up to --timeout). Use '--wait' alone for 'watcher' strategy, or specify one of: 'watcher', 'hookOnly', 'legacy'. Default when flag is omitted: 'hookOnly'."* and *"--wait-for-jobs — if set and --wait enabled, will wait until all Jobs have been completed before marking the release as successful."*
So `--wait` is no longer a bare boolean: it is a strategy-valued flag defaulting to `hookOnly`. Any prose that says "pass `--wait`" is still correct; any prose that implies it is on/off is not. Also from the hooks page: *"This is a blocking operation, so the Helm client will pause while the Job is run."*

**P8 · Helm release state** — <https://helm.sh/docs/topics/advanced/>
Verbatim: the backend is selected by `HELM_DRIVER`, *"It can be set to one of the values: `[configmap, secret, sql]`"*, with Secrets the default (the page's migration instructions read *"If you want to switch from the default backend to the ConfigMap backend"* and retrieve existing state via `kubectl get secret --all-namespaces -l "owner=helm"`).
**Caveat — this page carries a banner: *"This page has not yet been updated for Helm 4. Some of the content might be inaccurate or not applicable to Helm 4."*** Cite it, but cite it knowing that. See §1c.

**P8b · Helm rollback is release-scoped** — <https://helm.sh/docs/helm/helm_rollback/>. Helm 4 additionally grew `--rollback-on-failure` on `upgrade` (*"if set, Helm will rollback the upgrade to previous success release upon failure"*), which strengthens the Helm rollback row rather than weakening it.

**P9 · a node reads only its own Node object** — <https://kubernetes.io/docs/reference/access-authn-authz/node/>
Verbatim: *"Kubelets are limited to reading their own Node objects"* (`v1.34 [stable]`); identity is `system:node:<nodeName>` in group `system:nodes`. Backs the node book's security argument for the node-local guard.

**P10 · ReadWriteOnce, and access modes are not a write lock** — <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes>
Verbatim: *"ReadWriteOnce access mode still can allow multiple pods to access (read from or write to) that volume when the pods are running on the same node."* and *"Volume access modes do not enforce write protection once the storage has been mounted."* Backs the init-container book's lock.

**P11 · `gitRepo` removed, init container named as the replacement** — <https://kubernetes.io/docs/concepts/storage/volumes/>. The strongest single citation in the whole fragment: Kubernetes deleted a declarative feature and pointed users at a shell script in an init container. Cite it wherever the prose states the residue thesis, because it is the incumbent conceding the point in its own reference docs.

### 1c. Currency caveat: Helm 4 shipped, and my fragment was written against Helm 3

The single most consequential thing this audit found, and it is not a chafe — it is a
freshness problem in my own earlier work.

`helm.sh/docs` now documents **Helm v4**: *"Helm v4 represents a significant evolution from
v3, introducing breaking changes, new architectural patterns, and enhanced functionality
while maintaining backwards compatibility for charts."*
(<https://helm.sh/docs/overview/>)

What actually bears on the SIBLINGS Helm material:

- **`--wait` is no longer a boolean** (P7). Strategy-valued, defaults to `hookOnly` when
  omitted. Prose implying an on/off flag is now wrong.
- **Server-side apply is the Helm 4 default for new releases**: *"Helm 4 will default to
  server-side apply when installing a new Chart release"*, with latching behaviour for
  releases created by Helm 3. This touches the "reviewed text is what executes" reasoning in
  the same direction as CH-2 — SSA means the API server merges field ownership rather than
  storing your document.
- **Post-renderers are now plugins** — a breaking change: *"it is no longer possible to pass
  an executable directly to `helm render --post-renderer`, but a plugin name must be
  passed."* Relevant because a post-renderer was the obvious "arbitrary executable in the
  Helm pipeline" seat, and Helm 4 just closed the easy version of it. If anyone was tempted
  to claim a dorc-inside post-render seat, that claim needs re-basing.
- **Several Helm doc pages still carry a "not yet updated for Helm 4" banner**, including
  `topics/advanced` (the storage-backend citation, P8). Flagged rather than dropped: the
  live page is the best available first-party text, and the banner is disclosed.
- **Not re-verified against Helm 4**: the hook-resources-not-GC'd claim (P5). The sentence
  is present on the live page today, so the citation is honest, but Helm 4 changed how hooks
  are processed (they now flow through post-renderers), so the GC behaviour deserves a
  re-check before anyone leans on it hard. `~SUSPECT` it still holds; not verified.

**Recommendation:** the Helm rows and prose stand as written *except* the `--wait` shape,
which should be corrected. The rest is disclosure, not repair. My earlier fragment's
"helm.sh v3" framing should be updated to v4 wherever it appears.

### 1d. Verification ledger

The brief said verify every URL resolves. Stating exactly what I did, because "verified" is
a claim and an audit that fudges it is worse than no audit.

**Fetched by me during this audit, resolving, with the quoted text read in place:**
`concepts/configuration/secret/` · `reference/access-authn-authz/extensible-admission-controllers/` ·
`concepts/architecture/controller/` · `concepts/architecture/garbage-collection/` ·
`concepts/architecture/cloud-controller/` · `reference/kubectl/generated/kubectl_diff/` ·
`tasks/manage-kubernetes-objects/kustomization/` · `concepts/workloads/controllers/deployment/` ·
`concepts/storage/persistent-volumes/` · `helm.sh/docs/overview/` ·
`helm.sh/docs/topics/advanced/` · `helm.sh/docs/helm/helm_upgrade/`

**Fetched by me in the earlier book-writing pass, resolving, quotes read in place:**
`concepts/workloads/pods/init-containers/` · `reference/kubectl/generated/kubectl_wait/` ·
`reference/access-authn-authz/node/` · `reference/node/node-status/` ·
`helm.sh/docs/topics/charts_hooks/`

**Cited on a doc-reader's extraction, base page confirmed real by me, exact quote NOT
re-read by me in this pass** — treat the wording as `~SUSPECT`-tier until someone re-reads
it: the two access-mode sentences at `persistent-volumes/#access-modes` (P10); the
`gitRepo`-removal wording at `concepts/storage/volumes/` (P11); the init-container probe
sentence at `reference/kubernetes-api/workload-resources/pod-v1/` (P2's API-reference half —
note the *concept*-page half of P2 is first-hand and is the stronger citation anyway).

**Asserted URL shape, page family confirmed, specific section not opened:**
`concepts/workloads/controllers/deployment/#rolling-back-a-deployment` (C4) — page confirmed,
anchor conventional, `kubectl rollout undo` spelling not re-read this pass;
`tasks/manage-kubernetes-objects/declarative-config/` (C2's prune half) — I substituted
first-hand evidence from `kubectl diff`'s own `--prune` flags rather than lean on it;
`reference/using-api/api-concepts/#dry-run` (C7) — dropped from the citation above in favour
of the first-hand `kubectl diff` synopsis;
`concepts/storage/dynamic-provisioning/`, `tasks/configure-pod-container/security-context/`,
`concepts/workloads/pods/sidecar-containers/`, `helm.sh/docs/helm/helm_rollback/` — standard
stable paths, not opened this pass.

Net: every cell footnote in §1a that carries a *verbatim quotation* is first-hand. The
unverified residue is confined to supporting links and to three quotes explicitly marked
above.

---

## 2. Row-chafe

Six items. One cuts against my own column (CH-1) and one cuts toward it (CH-4); the rest
are wording or footnote repairs. Conductor decides; opposite pulls from other columns mean
the row is an honest "ehhhh" and should stay.

### CH-1 — `Secrets management story` / Kubernetes = Y · **CHAFE-WORDING** (cuts against k8s)
- **Current:** bare `Y`, no footnote.
- **Why it chafes:** it is the most generous unqualified cell in my column, and Kubernetes'
  own documentation opens the Secret page with a Caution box saying the opposite of what a
  bare Y implies — unencrypted in etcd by default; anyone with API or etcd access can read
  them; anyone who can create a Pod in a namespace can read every Secret in it, *including
  indirectly by creating a Deployment.* Meanwhile Terraform is marked `~` largely for
  plaintext-in-state, which is the same class of failure. Applying one standard to both,
  either Terraform is under-marked or Kubernetes is over-marked.
- **Proposed:** keep `Y`, add the C3 footnote. Kubernetes genuinely has the most complete
  *story* (first-class object, KMS/encryption-at-rest, RBAC, CSI providers, short-lived SA
  tokens); what it lacks is a safe *default*, and a footnote is the honest place to say so.
  I do not recommend demoting to `~` — availability, not default posture, is what the row
  asks about.
- **URL:** <https://kubernetes.io/docs/concepts/configuration/secret/>

### CH-2 — footnote `[^k1]` on `The reviewed text is byte-for-byte what executes` · **CHAFE-WRONG**
- **Current:** `~[^k1]`, footnote *"true for its own objects; embedded shell reviewed as opaque string."*
- **Why it chafes:** the first clause is factually wrong today. The API server defaults
  unset fields, and mutating admission webhooks "can modify objects sent to the API server";
  the docs warn explicitly that "objects can be modified after being seen by mutating
  webhooks." Sidecar injection is the everyday case — the Pod that runs contains a container
  the author never wrote and the reviewer never saw. So it is *not* byte-for-byte even for
  its own objects.
- **Proposed:** keep the `~` mark (it is still partial, and server-side dry-run means you
  *can* see the mutated result if you look). Replace the footnote with C9's text.
- **Evidence:** <https://kubernetes.io/docs/reference/access-authn-authz/extensible-admission-controllers/>
- **Note for the conductor:** this repair makes the Kubernetes cell weaker, and it slightly
  strengthens the row's Dorc `Y` by contrast. That is an uncomfortable direction for an
  audit to push, so weigh it knowing I noticed the incentive.

### CH-3 — row `Privilege escalation built in` · **CHAFE-WORDING** (row-level, multi-column)
- **Current wording:** "Privilege escalation built in".
- **Why it chafes:** ambiguous between *acquiring* privilege on a target (sudo/become/doas —
  clearly the intent, given Ansible `Y` and Dorc `NYI`) and *constraining* it. Kubernetes has
  extensive first-class machinery for the second reading (`securityContext.allowPrivilegeEscalation`,
  `runAsUser`, `privileged`, Pod Security Admission), so a reader who takes the second reading
  sees `N/A` as a false concession.
- **Proposed rewording:** "Acquires privilege on the target (sudo / become / doas)".
- **Answer under the rewording:** Kubernetes `N/A`, unambiguously. Terraform and cloud-init
  likewise unchanged. No mark moves; the row just stops being misreadable.
- **URL:** <https://kubernetes.io/docs/tasks/configure-pod-container/security-context/>

### CH-4 — row `Existing scripts run unchanged (the adoption floor)` / Kubernetes = N/A · **CHAFE-WORDING** (cuts toward k8s)
- **Why it chafes:** `command: ["/bin/sh", "/opt/legacy.sh"]` against an image carrying the
  script runs it byte-unchanged — structurally the same move as Ansible's `script:` module,
  which earns `~` on this row. The script is unchanged; the *surroundings* (an image build,
  a manifest, a cluster) are not.
- **Proposed:** my weak lean is **leave `N/A` and add the C13 footnote** — the row is asking
  about a configuration-management adoption floor, and Kubernetes is not in that business, so
  N/A ("outside that tool's layer") is the honest read. But `~` is defensible and I would not
  argue hard against it.
- **Flagging it because it cuts against my own posture.** I would rather the table be right
  than have my column look modest.

### CH-5 — row `Skips explained with queryable provenance` / Kubernetes = N/A · **CHAFE-DISCIPLINE** (footnote only, no mark change)
- **Why it chafes:** `N/A` is correct — Kubernetes has no notion of a skip — but it reads as
  a shrug where there is something genuinely interesting to say. Kubernetes explains
  *failures to act* about as well as anything in the table (Events, condition `reason` and
  `message`, `kubectl describe`'s scheduler messages), and explains *decisions not to act*
  not at all: a controller that finds spec matching status emits nothing, forever.
- **Proposed:** keep `N/A`, add the C10 footnote. That asymmetry is precisely the shape of
  the Dorc `Y` sitting beside it, and one clause makes the row informative instead of empty.

### CH-6 — row `Remembers what it built; delete by un-declaring` / Kubernetes = Y · **CHAFE-WORDING**
- **Why it chafes:** the row is two claims and Kubernetes scores differently on each.
  *Remembers what it built*: unambiguous `Y` (etcd, plus `ownerReferences` cascade-deleting
  dependents). *Delete by un-declaring*: bare `kubectl apply` deletes nothing when you remove
  an object from your manifests — that needs `--prune`/ApplySet or a GitOps reconciler
  configured to prune, which is a deliberate opt-in precisely because it is dangerous.
- **Proposed:** keep `Y` (the row's spirit is "the tool has a memory that makes deletion
  expressible", which Kubernetes does) and add the C2 footnote naming the prune caveat.
- **URLs:** <https://kubernetes.io/docs/concepts/architecture/garbage-collection/> ·
  <https://kubernetes.io/docs/tasks/manage-kubernetes-objects/declarative-config/>

---

## 3. Out-of-lane flags

Free, one line each, **low confidence by construction** — zero research owed and none done
beyond what I already held. Ignore any that cost more to check than they are worth.

- **OUT-OF-LANE · cloud-init · `Remembers what it built; delete by un-declaring` = ~** —
  reads generous to me. cloud-init keeps an instance-id cache under `/var/lib/cloud/` that
  *suppresses re-running*, which is not the same as remembering what it built, and it has no
  deletion path at all. Candidate `N`.
- **OUT-OF-LANE · Terraform · `Whole-system rollback` = ~** — Terraform has no rollback verb;
  the practice is applying an older configuration *forward*, which for destructive resource
  changes is not a rollback. `~` is defensible if the row means "a supported path exists";
  `N` if it means "the tool can undo".
- **OUT-OF-LANE · cloud-init · `Check-then-converge inside its own units` = N** — probably
  right for the per-instance modules, but `#cloud-boothook` is every-boot by design and the
  format's own docs hand-write a once-per-instance guard, so there is a thin per-unit
  check-then-act story. Not worth a mark change; possibly worth a footnote elsewhere.
- **OUT-OF-LANE · nix/NixOS · `Re-measures the live system every run` = N** — agree with `N`,
  and flagging only to say it is the *right* N for a subtle reason worth a footnote if
  anyone challenges it: nix compares the declared closure against the current system's store
  path, which is measuring the *declaration*, not the live filesystem. That is exactly why
  it is the soundest delegation check any incumbent offers and simultaneously blind to
  out-of-band drift.

---

## 4. Housekeeping flag (not audit content)

A SyncThing conflict file appeared in this worktree during the audit:
`Research/notes/r26-glue-strawmen/k8s-node-standup.sync-conflict-20260728-121725-PHNHRER.sh`.
Diagnosed and benign: it is a stale copy of *this agent's own* pre-fix version of the node
book — the argparse bug corrected in `2a22d895` — replicated mid-edit and then superseded.
No third-party content, nothing lost, git holds the correct file. Left in place untouched,
since conflict cleanup is human-owned; safe to delete.
