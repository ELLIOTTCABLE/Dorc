# Companion note: `k8s-node-standup.sh` (the rung-zero node book)

> Tier: LLM-authored round note, r26 ops-glue-residue writing phase. The book beside this
> note is FROZEN EVIDENCE — imagination-tier, not runnable, never to be executed. All
> renders below are illustrative; the render format is not settled design. What they obey
> is `rul-attention-honesty`: the plan is the whole book, in original order, as plain sh;
> elided lines are present-but-commented-out; the plan shows the author's own bytes, never
> a resolved rewrite of them.

Domain: Kubernetes. Posture, held throughout: **Dorc versus Kubernetes is not a choice.**
Nobody standing up a cluster should be talking themselves out of k8s and into this. The
book below sits *underneath* the cluster, in the strip of host state that Kubernetes
deliberately does not govern — the machine that has to already exist, already have a
container runtime, and already hold credentials before a single Kubernetes object can
describe it. Kubernetes reconciles workloads onto nodes; it does not manufacture nodes.
That gap is the residue, and it is where this book lives.

---

## 1. What this book is for, in one paragraph

Bringing a worker into a kubeadm cluster is, today, a runbook: six pages of
kubernetes.io prose, a prep script somebody wrote, a `kubeadm token create` on a control
plane node, and a standing note that says "run `kubeadm join` — but only the first time,
and if you're re-using the box you have to `kubeadm reset` first, and clean `/etc/cni/net.d`
yourself, because reset doesn't." The Dorc version is one file that you run the same way
every time. It is the same file on day zero out of cloud-init and on day four hundred from
a laptop, and on the overwhelming majority of days it does nothing at all and says so.

That last clause is the whole product at this rung. It is *not* elision-heavy.

---

## 2. The renders

### Day zero — a naked Debian box that has never seen the cluster

```
$ dorc plan --verbose k8s-node-standup.sh worker7.k8s.example.net
 22  set -eu
 24  K8S_MINOR=v1.36
 25  CP_ENDPOINT=cp1.k8s.example.net:6443
 26  NODE_NAME=$(hostname -s)                              # measured: "worker7"
 27  KUBECONF=/etc/kubernetes/kubelet.conf
 31  SUDO=
 32  [ "$(id -u)" = 0 ] || SUDO=sudo                       # measured: uid 0 -> SUDO=""
 41  if [ "$(kubectl --kubeconfig="$KUBECONF" get node "$NODE_NAME" \
 42        -o jsonpath='{...Ready...}' 2>/dev/null)" != True ]
 43                                                        # runs: measured "" ('kubectl' absent)
 44  then
 46     printf 'net.ipv4.ip_forward = 1\n' \
 47        | $SUDO tee /etc/sysctl.d/k8s.conf >/dev/null   # runs: diverged (no such file)
 48     $SUDO sysctl --system >/dev/null                   # runs: diverged (ip_forward=0)
 53     printf 'br_netfilter\n' \
 54        | $SUDO tee /etc/modules-load.d/k8s-cni.conf >/dev/null   # runs: diverged
 55     $SUDO modprobe br_netfilter                        # runs: diverged (not loaded)
 59     $SUDO swapoff -a                                   # runs: diverged (2 swap devices)
 62     $SUDO apt-get update                               # runs: diverged (index stale)
 63     dpkg -s containerd >/dev/null 2>&1 \
 63        || $SUDO apt-get install -y containerd          # runs: your guard fails (dpkg rc 1)
 ...  [ 10 more sites, all diverged ]
 96     [ -f "$KUBECONF" ] || $SUDO kubeadm join ...       # runs: your guard fails (no such file)
105     kubectl --kubeconfig="$KUBECONF" wait \
106        --for=condition=Ready "node/$NODE_NAME" --timeout=300s   # runs: diverged
plan: 19 to run (0 skipped)
```

Day zero is honest and unimpressive: nothing is converged, so nothing is skipped. The
gain on day zero is not elision. It is that this is *one file*, it needed nothing but
`sh` to start, and the same file will be the answer tomorrow.

### Day N — the node is Ready. The default render, which is the point.

```
$ dorc plan k8s-node-standup.sh worker7.k8s.example.net
 22  set -eu
 24  K8S_MINOR=v1.36
 25  CP_ENDPOINT=cp1.k8s.example.net:6443
 26  NODE_NAME=$(hostname -s)
 27  KUBECONF=/etc/kubernetes/kubelet.conf
 31  SUDO=
 32  [ "$(id -u)" = 0 ] || SUDO=sudo
plan: nothing to run (19 sites in 1 omitted region, line 41)
```

One read against the API server folded nineteen mutation-capable sites out of existence.
The mechanism is `omit`, not `elide` — the guard's condition resolved to a measured value,
that value proved the `then`-branch dead, and a branch that cannot run needs no per-line
vouches from anybody. Nineteen sites, zero oracles consulted inside the region, zero
authored convergence claims trusted. It is the cheapest license in the design and this
book gets its entire steady-state payout from it.

Two consequences worth naming because they are easy to miss:

- **An omitted region casts no walls.** Boot/standup work sits at the *top* of books, which
  is the worst possible wall real-estate — an honest wall there demotes the whole rest of
  the file. On a Ready day there is no wall at line 41, because nothing there will run.
- **The join credential is never needed.** `${JOIN_TOKEN:?...}` sits on the right of an
  `||`, so a converged apply never expands it. A day-N run of this book requires no
  bearer token, no `kubeadm token create` round-trip to a control plane node, and no
  24-hour-TTL secret anywhere near the operator's shell. That falls out of the guard
  rather than being designed for, and it is the single best security consequence in the
  book.

### Drifted day — someone stopped the kubelet during an incident and forgot

```
$ dorc plan --verbose k8s-node-standup.sh worker7.k8s.example.net
 ...
 41  if [ "$(kubectl --kubeconfig="$KUBECONF" get node "$NODE_NAME" \
 42        -o jsonpath='{...Ready...}' 2>/dev/null)" != True ]
 43                                                        # runs: measured "False"
 44  then
 46  #  printf 'net.ipv4.ip_forward = 1\n' \
 47  #     | $SUDO tee /etc/sysctl.d/k8s.conf >/dev/null   # converged: content match
 48  #  $SUDO sysctl --system >/dev/null                   # converged: net.ipv4.ip_forward=1
 55  #  $SUDO modprobe br_netfilter                        # converged: module loaded
 59  #  $SUDO swapoff -a                                   # converged: no swap active
 63  #  dpkg -s containerd >/dev/null 2>&1 \
 63  #     || $SUDO apt-get install -y containerd          # converged: your guard holds (rc 0)
 70  #  containerd config dump | grep -q 'SystemdCgroup = true' || { ... }
 70                                                        # converged: your guard holds (rc 0)
 75  #  $SUDO systemctl enable --now containerd            # converged: enabled+active
 86  #  $SUDO apt-get install -y kubelet kubeadm kubectl   # converged: 3 packages current
 88     $SUDO systemctl enable --now kubelet               # runs: diverged (enabled, inactive)
 96  #  [ -f "$KUBECONF" ] || $SUDO kubeadm join ...       # converged: your guard holds (rc 0)
105     kubectl --kubeconfig="$KUBECONF" wait \
106        --for=condition=Ready "node/$NODE_NAME" --timeout=300s
106                                                        # runs: diverged (Ready=False)
plan: 2 to run (17 skipped)
```

This is the render the interior coverage exists for, and it is the *only* one. On a Ready
day the interior is omitted wholesale; on day zero nothing in it is converged. The
per-line oracle work pays out exactly here: nineteen sites narrowed to the two that are
actually about the broken thing, on a morning when somebody is already unhappy.

Note line 41 is *not* commented out on this day. The `if` is the author's own sh and it
re-evaluates live at apply time. That is a guard by construction — nothing has run above
it — so the plan and the apply cannot disagree about whether the region is entered.

### The honest limit — everything converged, node still NotReady

```
$ dorc plan k8s-node-standup.sh worker7.k8s.example.net
 41  if [ "$(kubectl --kubeconfig="$KUBECONF" get node "$NODE_NAME" \
 42        -o jsonpath='{...Ready...}' 2>/dev/null)" != True ]
 43                                                        # runs: measured "False"
plan: nothing to run (19 skipped)
hint: every site in this book is converged, and worker7 is still not Ready.
      the Node object's own account of it:
        Ready=False  reason=KubeletNotReady  message="...cni plugin not initialized..."
      nothing in this book describes that. it is a cluster-scoped fact.
```

A book that can say "I have checked everything I know how to check, and the problem is
not mine" is doing real work. The CNI add-on is a cluster object, reconciled by the
control plane; this book has no business touching it and says so instead of pretending.
(`reason` and `message` are real Node condition fields — the specific `KubeletNotReady`
message text is mechanism-confirmed, not doc-confirmed; see §7.)

---

## 3. Real vs. invented

| In the book | Status |
| --- | --- |
| `pkgs.k8s.io` `core:/stable:/v1.36/deb/` repo, `/etc/apt/keyrings/kubernetes-apt-keyring.gpg`, `apt-mark hold` | REAL, current, copied from install-kubeadm |
| `net.ipv4.ip_forward = 1` in `/etc/sysctl.d/k8s.conf` + `sysctl --system` | REAL, and it is now the *whole* documented prereq |
| `br_netfilter` module-load attributed to the CNI, not to k8s | REAL — see §7, this is a genuine 2026 correction |
| `swapoff -a`; kubelet fails to start with swap unless `failSwapOn: false` | REAL |
| `containerd config default > /etc/containerd/config.toml`; `SystemdCgroup = true` | REAL |
| The containerd 1.x vs 2.x TOML key rename that makes a hard-coded `sed` a silent no-op | REAL, and documented on the same page as both spellings |
| `kubeadm join <endpoint> --token --discovery-token-ca-cert-hash sha256:...` | REAL |
| `kubeadm join` refusing on an already-joined node (`FileAvailable--etc-kubernetes-kubelet.conf`) and on a duplicate Node name | REAL |
| `/etc/kubernetes/kubelet.conf` as the "already joined" sentinel | REAL — and deliberately not `bootstrap-kubelet.conf`, which kubeadm deletes |
| Node `.status.conditions[Ready].status` ∈ {True, False, Unknown}; `kubectl get -o jsonpath` | REAL |
| A joined node reading its *own* Node object with `kubelet.conf` | REAL (Node authorizer: "Kubelets are limited to reading their own Node objects") |
| `kubectl wait --for=condition=Ready node/X --timeout=0s` = check once, don't wait | REAL |
| `SUDO=` / `[ "$(id -u)" = 0 ] || SUDO=sudo` threaded through mutating lines | REAL idiom, three independent tools do exactly this (turn C) |
| --- | --- |
| `#!/usr/bin/env dorc-run` | INVENTED (sanctioned palette; design prose only — no binary exists) |
| `kubectl__predict` / `kubectl__is_converged` and the `__role` dialect | INVENTED (existing ruled dialect, applied to a new tool) |
| The plan renders, `omit` annotations, the closing `hint:` | INVENTED (render form explicitly unwelded) |
| `sm.k8s.*` kind names, had this book needed coordinates | INVENTED; deliberately in the project's intentionally-unreal `sm.` namespace, not `io.k8s.*`, so strawman names cannot leak into reality |

Nothing outside the sanctioned palette is used. One thing I *wanted* and could not spell is
in §6.

---

## 4. Why this shape

**The guard is one read, and it is not a read of this machine.** The interesting property
of "is worker7 registered and Ready" is that it is not a fact the machine owns. The
machine can be perfectly configured and still not be in the cluster; it can be in the
cluster and have been cordoned, tainted, or evicted by a controller that never asked it.
The authority is the API server. So the fact is minted elsewhere and is *about* this host —
the shape the round has been calling facts-about-H-minted-from-elsewhere, and the exact
cousin of the ssh-oracle connection-dance.

I spelled it as a **node-local read against the API server** (`kubectl --kubeconfig=/etc/kubernetes/kubelet.conf`)
rather than a controller-side read with an admin kubeconfig. Three reasons, and the third
is the real one:

1. It needs no new construct. The book's scope stays one host; nothing has to say "this
   line runs somewhere else." See §6 for the version that does, and why I could not write it.
2. It is self-bootstrapping. Before the join there is no kubeconfig and no `kubectl`, so
   the read fails, yields empty, and the region runs. After the join it answers. The
   guard's own precondition is the thing it is guarding.
3. **It never puts a cluster-admin credential on the standup path.** The controller-side
   variant needs `admin.conf` in the operator's hands at plan time for every node in the
   fleet. This variant uses the node's own `system:node:worker7` identity, which the Node
   authorizer confines to reading its own Node object. That is a strictly better security
   posture, and it is the reason I would still choose it even after the scope-typing seam
   lands.

**Adequacy rider, named: `Ready ≠ correctly-configured`.** This is the transport-adequacy
gap ("reachable ≠ provisioned") wearing a Kubernetes hat, and it is the same species as
`converged ≠ no-op`. A node can be Ready while missing everything this book would install
after the join — a label, a sysctl the CNI wants, a pinned package version. The coarse
guard folds the interior *unchecked* on Ready days. There is no drift-healing inside the
region until either finer oracles arrive or the admin narrows the guard. This is the
honest trade and it must not be sold as anything else: it is exactly what the admin's own
bare-sh `if` would have done, no worse, and gradual enhancement behaving normally.

**The can't-tell case bites, and the interior is what catches it.** The guard's read
cannot distinguish "the API server says no such node" from "the API server is
unreachable from here". Both yield empty output; both fold the region *in*. Under the
`kFAIL-perform` weld that is the correct direction — but on a healthy node whose network
blinked, it means re-entering a nineteen-site standup region. What makes that survivable
is precisely the per-line interior coverage: `dpkg -s` holds, the containerd guard holds,
the `kubelet.conf` test holds, and the plan comes back with one or two sites. **The
interior guards are the safety net under the outer guard's uncertainty.** That is the
strongest argument in this book for spending the effort on interior oracles even though
their steady-state payout is zero, and I did not expect to find it going in.

**The wait is inside the artifact, not on the controller.** After `kubeadm join`, real
scripts wait for Ready. The naive spelling is a controller-side `until kubectl get node
...; do sleep 5; done`, which is one fresh handshake per poll — a network boundary
participating in iteration. Readiness *is* observable from inside the node once it holds
credentials, so the wait compiles into the artifact: one connection, bounded at 300s, and
modeled well enough that the plan can say what it is waiting for. The controller-side
exception is reserved for facts definitionally unobservable from within (host existence,
first reachability) — which is the pivot book's territory, not this one's.

The poll-loop shape is not a straw man, incidentally: `kubectl wait`'s own reference page
ships `until kubectl wait pod/busybox1 --for=condition=Ready --timeout=1s 2>/dev/null || ...
do echo "Checking conditions..."; sleep 1; done` as a documented example. It is the right
shape *in-host* and the wrong one across a network, and nothing in the doc distinguishes
those cases because nothing in the doc has to.

**`kubectl wait --timeout=0s` is a general-purpose first-party convergence predicate, and
that is a bigger deal than this book uses.** The flag's documentation is unambiguous —
*"Zero means check once and don't wait"* — and `--for` accepts not only
`condition=<name>[=<value>]` but arbitrary `jsonpath='{...}'=value`. So Kubernetes ships a
verb that answers "is this arbitrary predicate over this object's status true right now",
read-only, for every resource type in the cluster. That is the single strongest delegation
target I found in the domain: an oracle arm for it is near-pure delegation and it covers a
class, not a command. It also means this book's own top guard could have been spelled
`kubectl wait --for='jsonpath={.status.conditions[?(@.type=="Ready")].status}=True'
node/"$NODE_NAME" --timeout=0s`. I kept the `[ "$(...)" != True ]` test form because it is
what admins actually write and because the whole point of the render is lifting *the
admin's own guard* — but the delegation spelling is the better engineering, and a hint
pointing at it is exactly the kind of thing the hint machinery is for.

**Where the all-in-one thesis actually lands here.** Not "one file instead of a
`kubeadm`-file plus a prep script" — kubeadm has no file. It is: one file instead of
*prose*. The non-Dorc artifact for this task is a runbook a human executes, whose
idempotence story is "don't run it twice," and whose repair story is `kubeadm reset` plus
three cleanups reset doesn't do. Turning a runbook into a re-runnable book is a smaller
claim than turning a manifest into a book, and it is the true one.

---

## 5. Chafe-points

Ordered by how much they hurt.

- **`chafe-scope-crossing-one-line`** — the top guard *wants* to be a controller-side read
  and cannot be, because Dorc has no way for one line of a target-scoped book to run
  somewhere else. See §6; this is the escalation.
- **`chafe-empty-head-command-substitution`** — `$SUDO apt-get install ...` has a *variable
  in command position* whose value is an early-bound host fact, and whose empty case makes
  the word vanish entirely. The analyzer has to resolve the head from probe-established
  value-flow before it can even ask which oracle owns the site, and the resolved head is
  then a 27C context-entry (`sudo`) for every line downstream. This is the shape the sudo
  gap has to model and the book leans on it nineteen times. It is also the most likely
  place a naive implementation quietly gives up and walls the rest of the file.
- **`chafe-embedded-sublanguage-in-argv`** — `-o jsonpath='{.status.conditions[?(@.type=="Ready")].status}'`
  is a query language inside one argument. Dorc's contract is argv-walking; it stops at the
  argument boundary. An oracle that tried to *understand* the jsonpath would be matching
  brittle literal strings (any whitespace change defeats it). The escape I used is faithful
  delegation — the oracle re-runs the same read and reports its bytes, understanding
  nothing — which works, and generalizes: **for tools whose read verbs carry embedded query
  languages (kubectl jsonpath/go-template/label selectors, `jq`, `psql -c`), delegation is
  the only honest predict body.** Worth carrying to the synthesis note.
- **`chafe-faithful-delegation-is-an-exfil-surface`** — the corollary. A blanket "delegate
  any read-only verb" arm for a general-purpose API client would happily ship
  `kubectl get secret -o yaml` into the probe readback lane, which is already flagged as
  holding unsanitized host metadata. The book narrows the arm to `get node`. The general
  law this suggests: **a delegating predict arm must be scoped to resource classes the
  author surveyed, and "it's read-only" is not the same claim as "it is safe to put in a
  report."** New, I think, and it is not currently anywhere in the corpus.
- **`chafe-pipeline-rc-everywhere`** — `printf | tee`, `curl | gpg`, `containerd config dump
  | grep -q`. The privilege idiom *forces* pipelines (you cannot `$SUDO printf >file`; the
  redirect happens unprivileged in the calling shell), so the sudo gap and the pipefail law
  are entangled in practice, not just in theory. Every mutating line in a day-N node book
  is a pipeline whose rc is verdict-load-bearing.
- **`chafe-dead-assignment-rendering`** — on a Ready day, `K8S_MINOR`, `CP_ENDPOINT` and
  `KUBECONF` are consumed only inside the omitted region. A real DCE pass would drop them;
  attention-honesty says show what will execute, and they *do* execute (harmlessly). The
  render above keeps them. I do not think this is settled, and the README's own PRE/DCE
  framing suggests it should be.
- **`chafe-transit-not-exercised`** — the repair path for a node that must re-join is
  `kubeadm reset` + `kubeadm join`, and `kubeadm reset` is a transit: it destroys the
  node's cluster identity, so every fact minted about it beforehand is invalidated, and
  the control plane's own opinion of the node changes underneath. I deliberately left it
  out of the book — a repair path is a different user story — but it is the natural home
  for a `: transits <axis>` verb in this domain, and an unmodeled `kubeadm reset` would
  wall every day it appears.

---

## 6. ESCALATION — `esc-controller-scoped-line-in-a-target-book`

**What I tried to say.** One line of this book needs to execute on the controller while
the other nineteen execute on the target.

**Why it is not a nice-to-have.** Two Kubernetes facts force it, independently:

1. `kubeadm join` refuses when a Node of the same name already exists in the cluster
   (added in v1.18, with RBAC added for the bootstrap-token user to `GET` the Node object
   precisely so it can check). On day zero the *node* cannot ask that question — it has no
   credential, no `kubectl`, no CA. Only the controller can. So the one question that would
   let the book distinguish "fresh box, safe to join" from "name collision, you need to
   reset the old one first" is unaskable from inside the book's scope.
2. `node-role.kubernetes.io/worker` — the label everyone wants on a worker — **cannot be
   applied by the node**. The NodeRestriction admission controller rejects restricted
   labels via `--node-labels`, and the docs are blunt about it: "If you attempt to add
   restricted labels by using this kubelet flag, the node will fail to register with the
   API server. To apply these labels manually, you must use `kubectl label` after the node
   has joined the cluster." That is a *mutating* line that Kubernetes structurally forbids
   the target from performing. It belongs in this book and I could not put it there.

**Why Dorc can't say it.** Every spelling I tried either invents an engine-owned role name
(roles are closed-at-a-version and extend by new name only — not mine to mint), reuses the
`dorc:` prefix for a meaning it does not have (`dorc:` is the analysis-license prefix; a
scope word there would strip to broken sh), or introduces a new runtime object on PATH
(`dorc-at controller -- kubectl ...`), which strips to a dangling dependency and kills the
off-ramp. The concept exists on the roadmap — local-exec is listed as owed, and it is a
prerequisite of `ack-pivot-must-support`'s first half — but no spelling exists, and I am
not authorized to mint one silently.

**What I did instead.** Wrote the node-local variant (§4), which is invention-free, real,
better on security, and satisfies the human's stated intent ("asks the control plane" —
it does; the API server answers). The two forced cases above are documented here rather
than papered into the book.

**What I would want ruled.** Not the spelling — the *ownership*. Is a line's execution
scope (a) a property of the line, said in the book; (b) a property of the command,
declared by its oracle; or (c) derived by the engine from the fact's own provenance? (b)
is tempting and I think wrong: it hands oracle authors the power to relocate execution of
someone else's book lines, which is a strictly larger power than the footprint trust and
has no attribution story. My weak lean is (a), because the admin owns where their
mutations happen and the plan render can show it. `-GUESS`, offered as input, not a
recommendation.

---

## 7. Honest ledger

- **Spent:** one file, adopted from a runbook the team already half-had. No new language,
  no manifests, no controller, no agent, no state store. Two oracle arms (stdlib, not the
  admin's). One environment variable pair for the join credential.
- **Gained, day N (the overwhelmingly common day):** nineteen mutation-capable sites
  provably not run, on the strength of one API read and zero authored vouches. No join
  token needed. No wall at the top of the file.
- **Gained, drifted day:** nineteen sites narrowed to the two about the broken thing.
- **Gained, day zero:** honestly, nothing over a good shell script — except that it is the
  *same* shell script as day N, which is the entire rung-zero claim.
- **Gained, structurally:** a converge-button for the part of the estate that has none.
  For a shop whose k8s reconciles itself from git and whose nodes are baked images, this
  book is for the nodes that are *not* — the two on-prem edge boxes, the bastion, the one
  GPU node nobody rebuilds. Same shape as USER_STORY's "mutable residue on a principaled,
  declarative ops team", one layer lower.
- **Not gained:** anything inside the cluster. Not the CNI, not the workloads, not the
  control plane. A cordon, a taint, an eviction, a failing CSI driver — all invisible to
  this book beyond the single Ready bit it reads. That is correct and permanent.
- **Not gained:** interior drift-healing on Ready days (the adequacy rider, §4).
- **Not gained:** anything for a shop whose nodes are immutable images. If your worker is
  an AMI and a scale-group, this book is a solved problem you solved better. Go do that.

**The rung-zero claim, stated so it cannot be inflated.** This book's elision count on a
normal day is 19-of-19, and *none of it is oracle work* — it is one branch fold. Take the
fold away and the interior's own steady-state elision is worth roughly one screen of
attention and a handful of avoided `apt-get` calls. The value here is: one language and
one converge-button for the residue, including this, alongside the dotfiles book and the
bastion book; plus an honest plan on the bad morning. That is rung zero. The escalating
payoff is elsewhere.

---

## 8. Citations

All accessed **2026-07-28**; kubernetes.io docs were serving **v1.36**.

- Init/architecture — Nodes: <https://kubernetes.io/docs/concepts/architecture/nodes/>
  (self-registration flags, `--register-with-taints`, `node-role.kubernetes.io/<role>`
  convention, node controller / 5-minute eviction delay).
- Node status: <https://kubernetes.io/docs/reference/node/node-status/> — condition table
  (`Ready`/`DiskPressure`/`MemoryPressure`/`PIDPressure`/`NetworkUnavailable`); *"`True` if
  the node is healthy and ready to accept pods, `False` if the node is not healthy and is
  not accepting pods, and `Unknown` if the node controller has not heard from the node in
  the last `node-monitor-grace-period` (default is 50 seconds)"*; lease heartbeats every
  10s in `kube-node-lease`; `.status` updates every 5 minutes; example JSON showing
  `reason`/`message` fields. **Doc bug noted:** the same page later says "the 40 second
  default timeout for unreachable nodes", contradicting 50s twice above it;
  `kube-controller-manager`'s flag reference gives `--node-monitor-grace-period` default
  `50s`, which is authoritative.
- `kubeadm join`: <https://kubernetes.io/docs/reference/setup-tools/kubeadm/kubeadm-join/> —
  phases, discovery modes, `--node-name`, `--cri-socket`, and the verbatim
  `--ignore-preflight-errors=FileAvailable--etc-kubernetes-kubelet.conf`, which is the only
  preflight check name spelled out in current primary docs.
- Duplicate-node refusal: <https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/troubleshooting-kubeadm/> —
  *"In v1.18 kubeadm added prevention for joining a Node in the cluster if a Node with the
  same name already exists. This required adding RBAC for the bootstrap-token user to be
  able to GET a Node object."* (The exact refusal text is UNCONFIRMED against docs.)
- `kubeadm reset`: <https://kubernetes.io/docs/reference/setup-tools/kubeadm/kubeadm-reset/> —
  synopsis is *"Performs a best effort revert of changes"*; reset does **not** clean
  `/etc/cni/net.d`, iptables/nftables/IPVS rules, or `$HOME/.kube`. No idempotence
  guarantee is claimed anywhere.
- Install kubeadm: <https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/> —
  `pkgs.k8s.io` repo layout, keyring path, `apt-mark hold`, swap
  (*"The default behavior of a kubelet is to fail to start if swap memory is detected"*),
  and the crashloop line: *"The kubelet is now restarting every few seconds, as it waits in
  a crashloop for kubeadm to tell it what to do."*
- Container runtimes: <https://kubernetes.io/docs/setup/production-environment/container-runtimes/> —
  **the 2026 correction**: the prereq section is now *only* `net.ipv4.ip_forward = 1` +
  `sysctl --system`. No `overlay`, no `br_netfilter`, no `bridge-nf-call-iptables`
  anywhere on the page; the page instead hedges *"Some might also expect other sysctl
  parameters to be set, kernel modules to be loaded, etc; consult the documentation for
  your specific network implementation."* Every tutorial on the internet still ships the
  old block. The same page carries both containerd cgroup-driver TOML paths:
  `plugins."io.containerd.grpc.v1.cri"...` (1.x) and
  `plugins.'io.containerd.cri.v1.runtime'...` (2.x) — the plugin ID itself was renamed, so
  a `sed`/`grep` keyed on one is a silent no-op on the other. Also the packaged-containerd
  wart: *"If you installed containerd from a package ... you may find that the CRI
  integration plugin is disabled by default."*
- Files after a join: <https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/kubelet-integration/> —
  `/var/lib/kubelet/config.yaml`, `/var/lib/kubelet/kubeadm-flags.env`,
  `/etc/kubernetes/kubelet.conf`; and *"Kubeadm deletes the
  `/etc/kubernetes/bootstrap-kubelet.conf` file after completing the TLS Bootstrap"* — which
  is why the sentinel in this book is `kubelet.conf` and not the bootstrap file.
  (New in v1.36: `/var/lib/kubelet/instance-config.yaml`, since `NodeLocalCRISocket` went
  GA and the CRI socket moved off the `kubeadm.alpha.kubernetes.io/cri-socket` Node
  annotation. A cheap, current example of standup scripts rotting under you.)
- Node authorization: <https://kubernetes.io/docs/reference/access-authn-authz/node/> —
  *"Kubelets are limited to reading their own Node objects"* (`v1.34 [stable]`); identity is
  `system:node:<nodeName>` in group `system:nodes`.
- Restricted labels: <https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/> —
  *"Because of the `NodeRestriction` admission controller, you cannot use the kubelet
  `--node-labels` flag to apply restricted labels (such as `node-role.kubernetes.io/*`)
  during initialization... To apply these labels manually, you must use `kubectl label`
  after the node has joined the cluster."* Also the CNI requirement:
  *"You must deploy a Container Network Interface (CNI) based Pod network add-on so that
  your Pods can communicate with each other."*
- `kubeadm token`: <https://kubernetes.io/docs/reference/setup-tools/kubeadm/kubeadm-token/> —
  `--ttl` default `24h0m0s`; `--print-join-command`.
- `kubectl wait`: <https://kubernetes.io/docs/reference/kubectl/generated/kubectl_wait/> —
  *"--timeout duration Default: 30s — The length of time to wait before giving up. Zero
  means check once and don't wait, negative means wait for a week."* `--for` accepts
  `create`, `delete`, `condition=<name>[=<value>]`, and `jsonpath='{...}'=value`; multiple
  `--for` flags are ANDed. The page's own example of waiting on either of two conditions is
  a `until kubectl wait ... --timeout=1s ...; sleep 1; done` poll loop.

**UNCONFIRMED, carried honestly:** no primary page states "the Ready condition stays False
until a CNI plugin is installed" in those words; the mechanism is real (kubelet reports
network-not-ready to the runtime status) but the exact `KubeletNotReady` message string
used in the §2 render is not doc-quoted. `Port-10250` as a preflight check name is
confirmed by tool output in community threads, not by reference docs.

---

## 9. Flags for the conductor

- **kBOOT** — this book is a clean case for the knob's *non*-boot half. Its channel is
  ordinary ssh on day N, but on day zero the identical file is cloud-init user-data with no
  return path at all. Same book, same meaning, different delivery: the chef-solo no-fork
  rule holds trivially here because the guard's answer (`kubectl` absent ⇒ empty ⇒ run
  everything) is *correct without a probe*. That is a useful existence proof — a book whose
  day-zero offline behaviour needs no compile-time narrowing because its own guard already
  degrades right.
- **26K synthesis** — three items I think are new: (a) *the interior guards are the safety
  net under the outer guard's can't-tell*, which is the missing argument for spending on
  interior oracles in coarse-guard books; (b) *delegation is the only honest predict body
  for tools with embedded query sublanguages in argv*, with kubectl/jq/psql as the class;
  (c) *a delegating predict arm is an exfiltration surface* — read-only ≠ safe-to-report —
  which I believe is not stated anywhere in the corpus.
- **Scope-typing seam** — §6 gives it two doc-forced Kubernetes cases, one of them a
  *mutating* line the target is structurally forbidden to perform. That is a stronger
  motivator than the read-only pivot case and worth carrying.
