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
The prune half is weaker than I first wrote, and the evidence is worth stating in full because it drives CH-6. <https://kubernetes.io/docs/tasks/manage-kubernetes-objects/declarative-config/>
Verbatim: *"In Kubernetes 1.36, there are two pruning modes available in kubectl apply: Allowlist-based pruning: This mode has existed since kubectl v1.5 but is still in alpha... The ApplySet-based mode is designed to replace it. ApplySet-based pruning: ...This mode was introduced in alpha in kubectl v1.27 as a replacement for allowlist-based pruning."*
**Both modes are still alpha** — FEATURE STATE `v1.5 [alpha]` and `v1.27 [alpha]` respectively; the intended replacement has not graduated either. From <https://kubernetes.io/docs/reference/kubectl/generated/kubectl_apply/>: *"Alpha Disclaimer: the --prune functionality is not yet complete. Do not use unless you are aware of what the current state is."* And the operational warning: *"Especially if flag values are changed between invocations, this can lead to objects being unexpectedly deleted or retained."*
So: delete-by-un-declaring in bare `kubectl` has been alpha for roughly a decade on the original route and three years on its replacement. In practice the capability is real but it is delivered by GitOps reconcilers (Argo, Flux), which are add-ons rather than Kubernetes.

**C3 · "Secrets management story" · Kubernetes = Y** — *the eyebrow cell; see CH-1.*
Footnote: *the most complete story in the table — first-class object, encryption-at-rest, RBAC, KMS and CSI provider integration, short-lived ServiceAccount tokens — but unencrypted in etcd by default, and the ability to create a Pod in a namespace implies read access to every Secret in it.*
<https://kubernetes.io/docs/concepts/configuration/secret/>
Verbatim (the page's own Caution box): *"Kubernetes Secrets are, by default, stored unencrypted in the API server's underlying data store (etcd). Anyone with API access can retrieve or modify a Secret, and so can anyone with access to etcd. Additionally, anyone who is authorized to create a Pod in a namespace can use that access to read any Secret in that namespace; this includes indirect access such as the ability to create a Deployment."*

**C4 · "Whole-system rollback" · Kubernetes = ~**
Footnote: *rollback is per-workload (`kubectl rollout undo deployment/<name>`, optionally `--to-revision`), bounded by `revisionHistoryLimit` (default 10 old ReplicaSets).*
<https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#rolling-back-a-deployment>
Verbatim: *"Alternatively, you can rollback to a specific revision by specifying it with --to-revision"*; *".spec.revisionHistoryLimit is an optional field that specifies the number of old ReplicaSets to retain to allow rollback... By default, 10 old ReplicaSets will be kept."*
**Honesty correction:** my first draft of this footnote ended "there is no cluster-wide rollback primitive." No primary page states that negative. It is an inference from every `kubectl rollout` example operating on a single named workload — sound, but it is absence-of-evidence, so the footnote should assert the positive (per-workload scope) and let the reader draw the rest. Corrected above.

**C5 · "Creates infrastructure (VMs, DNS, networks)" · Kubernetes = ~**
Footnote: *creates real cloud objects as a side effect of workload objects — load balancers and routes via the cloud-controller-manager, disks via StorageClass dynamic provisioning — but does not describe or own infrastructure as such; node creation is an add-on (autoscaler / Cluster API), not core.*
<https://kubernetes.io/docs/concepts/architecture/cloud-controller/>
Verbatim: *"The cloud-controller-manager is a Kubernetes control plane component that embeds cloud-specific control logic. The cloud controller manager lets you link your cluster into your cloud provider's API, and separates out the components that interact with that cloud platform from components that only interact with your cluster."*
<https://kubernetes.io/docs/concepts/storage/dynamic-provisioning/>
Verbatim, the two controllers that do the creating — route controller: *"The route controller is responsible for configuring routes in the cloud appropriately so that containers on different nodes in your Kubernetes cluster can communicate with each other."* Service controller: *"The service controller interacts with your cloud provider's APIs to set up load balancers and other infrastructure components when you declare a Service resource that requires them."* And dynamic provisioning: *"Dynamic volume provisioning allows storage volumes to be created on-demand."*
Corroborating the "side effect of workload objects" framing, the GC page lists among the things Kubernetes cleans up: *"Dynamically provisioned PersistentVolumes with a StorageClass reclaim policy of Delete"* and Nodes deleted *"On a cloud when the cluster uses a cloud controller manager"* — creation and deletion of real cloud resources, driven entirely by cluster-object lifecycle.
Nuance surfaced in verification, worth not overstating the `~`: the cloud-controller-manager's node controller holds `create` on `v1/Node` in its RBAC, so it is not purely an updater. That is creating Node *objects* during registration flows, not creating *machines* — the footnote's "node creation is an add-on" claim is about machines and stands, but do not let the row's prose harden into "the CCM only touches routes and load balancers."

**C6 · "Templating / config-file generation" · Kubernetes = ~**
Footnote: *`kubectl` has supported kustomization files since 1.14 (`kubectl kustomize`, `apply -k`) — no separate binary needed — and kustomize does overlays and patches, not text templating; templating is Helm's, and Helm is not Kubernetes.*
<https://kubernetes.io/docs/tasks/manage-kubernetes-objects/kustomization/>
Verbatim: *"Kustomize is a standalone tool to customize Kubernetes objects through a kustomization file. Since 1.14, kubectl also supports the management of Kubernetes objects using a kustomization file."*, via `kubectl kustomize <dir>` and `kubectl apply -k <dir>`.
Wording correction: I originally wrote "kustomize ships inside kubectl." The docs say kubectl *supports* kustomization files and describe kustomize as "a standalone tool"; functionally there is no separate binary to install, but "ships inside" is my phrasing, not theirs. Softened above.

**C7 · "Preview before mutating (plan / dry-run / diff)" · Kubernetes = Y**
Footnote: *`kubectl diff` compares your would-be-applied configuration against the live cluster, server-side — so the preview runs the real admission chain rather than simulating it.*
<https://kubernetes.io/docs/reference/kubectl/generated/kubectl_diff/>
Verbatim: *"Diff configurations specified by file name or stdin between the current online configuration, and the configuration as it would be if applied."*
The "runs the real admission chain" half is backed directly — <https://kubernetes.io/docs/reference/using-api/api-concepts/#dry-run> (FEATURE STATE `v1.19 [stable]`): *"Dry run mode helps to evaluate a request through the typical request stages (admission chain, validation, merge conflicts) up until persisting objects to storage... Kubernetes guarantees that dry-run requests will not be persisted in storage or have any other side effects."* Note that page documents the API-level `dryRun` parameter; the kubectl flag spelling lives at <https://kubernetes.io/docs/reference/kubectl/generated/kubectl_apply/>: *"--dry-run string[="unchanged"] Default: "none" | Must be "none", "server", or "client"."*
Worth the footnote because Dorc also claims Y on this row and the two Ys are not the same object: Kubernetes previews a declarative object diff; Dorc previews the executable text. Kubernetes' preview is in one respect *stronger* than anything Dorc will have — a server-side dry-run is evaluated by the same admission chain that would admit the real request, so the preview and the apply share machinery rather than merely agreeing by construction.

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

**P2 · readinessProbe refused at validation** — same page (quote at C8), plus the API reference: <https://kubernetes.io/docs/reference/kubernetes-api/workload-resources/pod-v1/>
Verbatim, from the `initContainers` field of PodSpec: *"Init containers are executed in order prior to containers being started. If any init container fails, the pod is considered to have failed and is handled according to its restartPolicy. The name for an init container or normal container must be unique among all containers. Init containers may not have Lifecycle actions, Readiness probes, Liveness probes, or Startup probes."*
Precision note worth keeping: the API reference says *"may not have"* (a schema-level restriction) while the concept page says *"This is enforced during validation."* Same substance, two registers. Prose that wants to claim an enforced refusal should cite the concept page; prose that wants the field-level rule should cite this one.

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
Verbatim: *"By default, release information is stored in Secrets in the namespace of the release."* Backend selection is via `HELM_DRIVER`: *"It can be set to one of the values: `[configmap, secret, sql]`."*
**CORRECTION AGAINST MY OWN EARLIER FRAGMENT — action needed.** My SIBLINGS Helm section wrote the release-state row as *"**Y** (`helm.sh/release.v1` Secrets)"*. That type string is **not first-party documented anywhere** — not on this page, not in the Helm API docs, and the 2019 `helm-3-preview-pt4` blog post that gets cited for it is a pre-release design preview that does not match the shipped model. It is well-attested only in community sources (`kubectl describe secret` output in gists and blog posts). **Recommendation: drop the type string from the row and say "in-cluster Secrets, by default", which is quotable.** If the precise string is wanted anywhere, it must be labelled community-sourced.
**Caveat — this page carries a banner: *"This page has not yet been updated for Helm 4. Some of the content might be inaccurate or not applicable to Helm 4."*** Cite it, but cite it knowing that. See §1c.

**P8b · Helm rollback is release-scoped** — <https://helm.sh/docs/helm/helm_rollback/>. Helm 4 additionally grew `--rollback-on-failure` on `upgrade` (*"if set, Helm will rollback the upgrade to previous success release upon failure"*), which strengthens the Helm rollback row rather than weakening it.

**P9 · a node reads only its own Node object** — <https://kubernetes.io/docs/reference/access-authn-authz/node/>
Verbatim: *"Kubelets are limited to reading their own Node objects"* (`v1.34 [stable]`); identity is `system:node:<nodeName>` in group `system:nodes`. Backs the node book's security argument for the node-local guard.

**P10 · ReadWriteOnce, and access modes are not a write lock** — <https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes>
Verbatim: *"ReadWriteOnce: the volume can be mounted as read-write by a single node. ReadWriteOnce access mode still can allow multiple pods to access (read from or write to) that volume when the pods are running on the same node. For single pod access, please see ReadWriteOncePod."* And: *"Volume access modes do not enforce write protection once the storage has been mounted. Even if the access modes are specified as ReadWriteOnce, ReadOnlyMany, or ReadWriteMany, they don't set any constraints on the volume. For example, even if a PersistentVolume is created as ReadOnlyMany, it is no guarantee that it will be read-only."*
Backs the init-container book's lock — and the second quote is stronger than I had it: the docs say access modes place *no constraints at all*, which means the `readOnly: true` on the app container's mount in that book's manifest is a container-level protection, not a volume-level one.

**P11 · `gitRepo` removed, init container named as the replacement** — <https://kubernetes.io/docs/concepts/storage/volumes/>, section heading `gitRepo (disabled)`.
Verbatim: *"Kubernetes 1.36 does not include the gitRepo volume driver. The last version that provided a way to use this driver was Kubernetes v1.35, and it has been deprecated since the v1.11 minor release."* Replacement, verbatim: *"To provision a Pod that has a Git repository mounted, you can mount an emptyDir volume into an init container that clones the repo using Git, then mount the EmptyDir into the Pod's container."*
The strongest single citation in the whole fragment: Kubernetes removed a declarative feature and pointed users at a shell script in an init container — the residue thesis, conceded by the incumbent in its own reference docs. Cite the *release numbers* carefully: removed as of the 1.36 docs, last shipped in v1.35, deprecated since v1.11.

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

**Important softening, from the verification pass:** Helm 3 is *still maintained in parallel*
(3.21.x, per the Helm-4-GA announcement), and in Helm 3 `--wait` and `--dry-run` genuinely
are plain booleans. So this is not "our claims went stale" — it is **"our claims did not say
which major they described."** That is a smaller sin and a different repair: the fix is to
name the version, not to rewrite the substance.

**Recommendation:** the Helm rows and prose stand *except* the `--wait` shape, which should
either be corrected to the Helm 4 spelling or explicitly labelled as Helm 3. Everything else
here is disclosure. Whichever way it goes, the Helm section should state the major version it
describes — a comparison table that silently straddles two majors of the same tool is the
kind of thing that reads fine for a year and then embarrasses everyone.

### 1d. Verification ledger

The brief said verify every URL resolves. Stating exactly what happened, because "verified"
is a claim and an audit that fudges it is worse than no audit.

**All 20 URLs cited in this document resolve (HTTP 200, real content), confirmed 2026-07-28.**
Every verbatim quotation above was read in place. Verification was split two ways:

- **Read first-hand by me** (this pass or the earlier book-writing pass):
  `concepts/configuration/secret/` · `reference/access-authn-authz/extensible-admission-controllers/` ·
  `concepts/architecture/controller/` · `concepts/architecture/garbage-collection/` ·
  `concepts/architecture/cloud-controller/` · `reference/kubectl/generated/kubectl_diff/` ·
  `tasks/manage-kubernetes-objects/kustomization/` · `concepts/workloads/controllers/deployment/` ·
  `concepts/storage/persistent-volumes/` · `concepts/workloads/pods/init-containers/` ·
  `reference/kubectl/generated/kubectl_wait/` · `reference/access-authn-authz/node/` ·
  `reference/node/node-status/` · `helm.sh/docs/overview/` · `helm.sh/docs/topics/advanced/` ·
  `helm.sh/docs/helm/helm_upgrade/` · `helm.sh/docs/topics/charts_hooks/`
- **Read by a clamped doc-verifier, quotes returned verbatim with resolution confirmed**:
  `tasks/manage-kubernetes-objects/declarative-config/` · `reference/kubectl/generated/kubectl_apply/` ·
  `reference/using-api/api-concepts/` · `reference/kubernetes-api/workload-resources/pod-v1/` ·
  `concepts/storage/volumes/` · `concepts/storage/dynamic-provisioning/` ·
  `reference/access-authn-authz/admission-controllers/` · `helm.sh/docs/helm/helm_rollback/`

**Two URLs I had cited were wrong or weak and are now fixed**, both caught by verification:
`api-concepts/#dry-run` documents the API-level `dryRun` parameter, *not* the kubectl flag —
the flag spelling lives on `kubectl_apply/`, now cited alongside it (C7); and
`helm.sh/docs/topics/advanced/` does not document the `helm.sh/release.v1` type string my
earlier fragment asserted (P8).

**Deliberately NOT verified, and flagged as such:** whether the Helm hook-GC behaviour (P5)
still holds under Helm 4's reprocessing of hooks through post-renderers. The sentence is
present on the live page today, so the citation is honest; the *behaviour* is `~SUSPECT`.
Someone should re-check before the claim carries weight. See §1c.

**One claim demoted from assertion to inference:** "no cluster-wide rollback primitive" (C4)
has no supporting sentence in primary docs. Corrected in place.

---

## 2. Row-chafe

Seven items. Three cut against my own column (CH-1, CH-6, CH-7) and one cuts toward it
(CH-4); the rest are wording or footnote repairs. Conductor decides; opposite pulls from
other columns mean the row is an honest "ehhhh" and should stay.

Two are **factual corrections requiring action** rather than judgement calls: CH-2 (a
footnote that is wrong today) and CH-7 (an unsourceable string in my own earlier fragment).

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

### CH-6 — row `Remembers what it built; delete by un-declaring` / Kubernetes = Y · **CHAFE-WORDING, upgraded after verification** (cuts against k8s)
- **Why it chafes:** the row is two claims and Kubernetes scores very differently on each.
  *Remembers what it built*: unambiguous `Y` — etcd holds every object, and `ownerReferences`
  give real cascading deletion (foreground and background both documented).
  *Delete by un-declaring*: bare `kubectl apply` deletes nothing when you remove an object
  from your manifests.
- **The evidence is stronger than I expected and it moved my recommendation.** Both pruning
  modes are **still alpha in the 1.36 docs** — allowlist-based `--prune` (`v1.5 [alpha]`,
  carrying *"Alpha Disclaimer: the --prune functionality is not yet complete. Do not use
  unless you are aware of what the current state is."*) and the ApplySet-based mode intended
  to replace it (`v1.27 [alpha]`, *"backwards incompatible changes might be introduced"*).
  Ten years on the first route, three on its replacement, neither graduated. In practice the
  capability is delivered by Argo or Flux — which are add-ons, not Kubernetes.
- **Proposed, and I have changed my mind here:** `~`, not `Y`. Elsewhere in my column I
  marked "creates infrastructure" as `~` precisely because node creation is add-on territory;
  applying that same standard, delete-by-un-declaring is add-on territory too. Terraform and
  nix earn `Y` on this row by doing it in the core tool as ordinary behaviour. If the
  conductor prefers to keep `Y` on the strength of the remembering half, the C2 footnote is
  mandatory rather than optional.
- **URLs:** <https://kubernetes.io/docs/concepts/architecture/garbage-collection/> ·
  <https://kubernetes.io/docs/tasks/manage-kubernetes-objects/declarative-config/> ·
  <https://kubernetes.io/docs/reference/kubectl/generated/kubectl_apply/>

### CH-7 — Helm prose: `helm.sh/release.v1` is not first-party sourceable · **CHAFE-WRONG** (my own fragment; cuts against my column)
- **Current:** my SIBLINGS Helm section writes the release-state row as
  *"**Y** (`helm.sh/release.v1` Secrets)"*.
- **Why it chafes:** the type string appears in no first-party Helm documentation. The
  verification pass checked `topics/advanced/`, the Helm API docs, and the 2019
  `helm-3-preview-pt4` blog post that usually gets cited for it — that post is a pre-release
  design preview that does not match the shipped model. The string is attested only in
  community artifacts (`kubectl describe secret` output in gists).
- **Proposed:** drop the string. The quotable first-party claim is
  *"By default, release information is stored in Secrets in the namespace of the release"*,
  which says everything the row needs. If the exact type string is wanted, label it
  community-sourced.
- **Severity note:** the *claim* (Helm keeps release state in cluster Secrets) is correct and
  well-sourced; only the parenthetical identifier is unsourceable. Tagged WRONG rather than
  WORDING because a precise-looking identifier in a comparison table reads as verified, and
  this one is not.
- **URL:** <https://helm.sh/docs/topics/advanced/>

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
