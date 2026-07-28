#!/usr/bin/env dorc-run
# dorc-lang/v0.2
#
#=========================== FROZEN EVIDENCE ==============================
# STRAWMAN - imagination-tier.  Features shown here MAY NOT EXIST.  This
# file is NOT RUNNABLE and must NEVER be executed, in whole or in part, by
# any human or agent.  It is a design-target written to be read.  No
# format-compat promises: pre-user, every spelling here is rename-in-place.
#==========================================================================
#
# k8s-node-standup.sh - bring a Debian worker into an existing kubeadm
# cluster, and keep it there.  Grounded in kubernetes.io docs for v1.36
# (accessed 2026-07-28); citations in k8s-node-standup.note.md.
#
#     dorc plan  k8s-node-standup.sh worker7.k8s.example.net
#     dorc apply k8s-node-standup.sh worker7.k8s.example.net
#
# The non-Dorc version of this file is the kubeadm runbook (six doc pages)
# plus a prep script plus "and then run kubeadm join, but only the first
# time".  This is the whole thing, and it is safe to re-run.

set -eu

K8S_MINOR=v1.36
CP_ENDPOINT=cp1.k8s.example.net:6443
NODE_NAME=$(hostname -s)
KUBECONF=/etc/kubernetes/kubelet.conf

# Day zero this runs as root out of cloud-init; day N it arrives as a
# plain ssh user.  Resolve the prefix once, thread it through.
SUDO=
[ "$(id -u)" = 0 ] || SUDO=sudo

# ---------------------------------------------------------------------------
# The whole book is behind one question, and it is not a question about this
# machine: it is the control plane's opinion of this machine.  Before the
# join there is no $KUBECONF and no kubectl, so the read fails, prints
# nothing, and the standup runs.  After the join, the node reads its own
# Node object with its own least-privilege credential.
# ---------------------------------------------------------------------------
if [ "$(kubectl --kubeconfig="$KUBECONF" get node "$NODE_NAME" \
          -o jsonpath='{.status.conditions[?(@.type=="Ready")].status}' \
          2>/dev/null)" != True ]
then
   # -- 1. host prerequisites -----------------------------------------------
   printf 'net.ipv4.ip_forward = 1\n' \
      | $SUDO tee /etc/sysctl.d/k8s.conf >/dev/null
   $SUDO sysctl --system >/dev/null

   # br_netfilter is our CNI's requirement, not Kubernetes': the k8s docs
   # dropped it and now say "consult the documentation for your specific
   # network implementation".  Ours is Calico's.
   printf 'br_netfilter\n' \
      | $SUDO tee /etc/modules-load.d/k8s-cni.conf >/dev/null
   $SUDO modprobe br_netfilter

   # kubelet refuses to start with swap present unless you set
   # failSwapOn: false.  We do not.
   $SUDO swapoff -a

   # -- 2. container runtime ------------------------------------------------
   $SUDO apt-get update
   dpkg -s containerd >/dev/null 2>&1 || $SUDO apt-get install -y containerd
   [ -f /etc/containerd/config.toml ] \
      || containerd config default | $SUDO tee /etc/containerd/config.toml >/dev/null

   # Ask the runtime what it is actually doing, not what the file says: the
   # TOML key moved between containerd 1.x and 2.x, so a sed keyed on one
   # spelling silently does nothing on the other.
   containerd config dump | grep -q 'SystemdCgroup = true' || {
      $SUDO sed -i 's/SystemdCgroup = false/SystemdCgroup = true/' \
         /etc/containerd/config.toml
      $SUDO systemctl restart containerd
   }
   $SUDO systemctl enable --now containerd

   # -- 3. kube packages, pinned to the control plane's minor ---------------
   $SUDO install -d -m 0755 /etc/apt/keyrings
   [ -f /etc/apt/keyrings/kubernetes-apt-keyring.gpg ] \
      || curl -fsSL "https://pkgs.k8s.io/core:/stable:/$K8S_MINOR/deb/Release.key" \
         | $SUDO gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg
   printf '%s\n' \
      "deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/$K8S_MINOR/deb/ /" \
      | $SUDO tee /etc/apt/sources.list.d/kubernetes.list >/dev/null
   $SUDO apt-get update
   $SUDO apt-get install -y kubelet kubeadm kubectl
   $SUDO apt-mark hold kubelet kubeadm kubectl
   $SUDO systemctl enable --now kubelet

   # -- 4. join -------------------------------------------------------------
   # kubeadm join is not re-runnable: its preflight refuses when
   # /etc/kubernetes/kubelet.conf is already present, and discovery refuses
   # again when a Node of this name already exists in the cluster.  Guard it
   # ourselves - and note the || means the token is never even expanded on a
   # node that has already joined, so a converged apply needs no credential.
   [ -f "$KUBECONF" ] || $SUDO kubeadm join "$CP_ENDPOINT" \
      --node-name "$NODE_NAME" \
      --token "${JOIN_TOKEN:?run 'kubeadm token create --print-join-command' on a control-plane node}" \
      --discovery-token-ca-cert-hash "${JOIN_CA_HASH:?same command prints this}"

   # -- 5. wait here, not from the controller -------------------------------
   # Readiness is observable from inside the node once it holds credentials,
   # so the wait belongs in the artifact: one connection, not one handshake
   # per poll.
   kubectl --kubeconfig="$KUBECONF" wait --for=condition=Ready \
      "node/$NODE_NAME" --timeout=300s
fi


# ===========================================================================
# The two oracle arms this book leans on.  In real life these are stdlib,
# not something the admin writes; shown here so the book stands alone.
# ===========================================================================

# kubectl already ships the read-only verbs, so the model is delegation: run
# the same read, report its bytes faithfully.  Deliberately NOT "any
# read-only verb", though - a blanket delegation for a general-purpose API
# client would cheerfully ship `kubectl get secret -o yaml` into the probe
# readback.  An oracle vouches for what it surveyed; this one surveyed nodes.
kubectl__predict() {
   # kubectl takes global flags BEFORE the verb, so the verb is not "$1".
   # Find it without consuming argv - the delegation below has to re-run the
   # caller's invocation intact, --kubeconfig and all.  A separated global
   # (--kubeconfig PATH) leaves a bare path where we look for a verb, we fail
   # to match, and we decline: the safe direction, but a silent one.
   verb=; noun=
   for a in "$@"; do
      case "$a" in -*) continue ;; esac
      if [ -z "$verb" ]; then verb=$a; else noun=$a; break; fi
   done

   [ "$verb" = get ] || return 2
   case "$noun" in
   node|nodes|no) kubectl "$@" ;;
   *) return 2 ;;
   esac
}

# `kubectl wait` is a first-party wait verb, so the convergence question is
# already answered by the tool: --timeout=0s is documented as "check once and
# don't wait".  Answering it is what lets a converged apply drop the wait
# instead of re-polling a fact that already holds.
kubectl__is_converged() {
   verb=
   for a in "$@"; do
      case "$a" in -*) continue ;; esac
      verb=$a; break
   done
   [ "$verb" = wait ] || return 2

   # Re-run the caller's own invocation, whole, with exactly one substitution:
   # their deadline for ours.  Rebuilding argv to drop a flag costs eight lines
   # of rotation in POSIX sh, and that cost is the honest price of an oracle
   # that argparse-walks instead of pattern-matching a command line.
   n=$#
   while [ "$n" -gt 0 ]; do
      arg=$1; shift
      case "$arg" in
      --timeout=*) ;;
      *) set -- "$@" "$arg" ;;
      esac
      n=$((n - 1))
   done
   kubectl "$@" --timeout=0s >/dev/null 2>&1
}
