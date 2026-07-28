#!/usr/bin/env dorc-run
# dorc-lang/v0.2
# ╔══════════════════════════════════════════════════════════════════════════╗
# ║  STRAWMAN · IMAGINATION-TIER · NOT RUNNABLE · NEVER EXECUTE              ║
# ║                                                                          ║
# ║  Frozen evidence for the r26 ops-glue-residue round. Features spelled    ║
# ║  herein MAY NOT EXIST — this is a design target written against real     ║
# ║  doctl v1.164.0 and cloud-init documentation, not a working script. Do   ║
# ║  not execute it, in whole or in part, not even a single "read-only"      ║
# ║  line. Every format, flag and spelling carries NO compat promise and     ║
# ║  will be renamed in place. Kind names use the deliberately-invalid `sm.` ║
# ║  TLD so nothing here can leak into a real vocabulary.                    ║
# ║  Companion note: pivot-vps-standup.note.md                               ║
# ╚══════════════════════════════════════════════════════════════════════════╝
#
# THE PIVOT: one book whose first lines run on the controller and whose last
# lines run on a machine that did not exist when the book started.
#
#   dorc plan pivot-vps-standup.sh          # NO host argument. The controller
#                                            # is the target; `ssh` is how the
#                                            # book reaches anywhere else.
#
# The day-N shape is the whole point. The admin's own outer reachability guard
# — an ordinary `if ! ssh …; then` anybody would write without us — folds the
# entire standup region dead on every day the machine already answers, and an
# omitted region casts no walls, so everything below it plans at full strength.
# On day zero the same line is simply false and the region runs.
#
# WHAT THIS BOOK DELIBERATELY DOES NOT DO: converge the machine's interior.
# That is userdata-boothook-web.sh's job, delivered through the user-data
# channel at creation and re-run over ssh on later days. This book owns exactly
# the residue user-data cannot carry — the controller's own API calls, the
# machine's public address, and the secrets that must never sit in an
# IMDS-readable instance attribute.

set -eu

DROPLET=web1
DOMAIN=example.net
FQDN=web1.example.net
REGION=ams3
SIZE=s-1vcpu-1gb
IMAGE=debian-13-x64
SSHKEY=laptop-2026            # pre-registered; `doctl compute ssh-key list`

# `accept-new` rather than the ecosystem's reflexive `StrictHostKeyChecking=no`
# — see §1 for why the difference is affordable here and nowhere else.
SSH="ssh -o BatchMode=yes -o ConnectTimeout=5 -o StrictHostKeyChecking=accept-new"


# ── 0a. the doctl oracle ───────────────────────────────────────────────────
#
# doctl is Cobra-shaped: positional verbs, value-taking flags, and — critically
# — NO documented exit-code contract. Community evidence is consistent that a
# missing droplet exits 1, but no primary source promises it, so this oracle
# never reads doctl's rc for a verdict. It counts rows instead.
#
# It also has to answer a wart the vendor does not constrain: DigitalOcean
# droplet names are NOT unique. Two droplets called `web1` is a legal account
# state, and "does web1 exist" is therefore a counting question.

doctl__predict() {
   [ "${1-}" = compute ] || return 2
   [ "${2-}" = droplet ] || return 2
   [ -n "${4-}" ] || return 2
   case ${3-} in
   create)
      : transits epoch          # a new machine is a new epoch of everything
      name : sm.doctl.Droplet = "$4"
      doctl compute droplet list "$4" --format Name --no-header \
         : sm.doctl.Droplet:"$4"@exists
      ;;
   delete)
      : transits epoch
      ;;
   esac
}

doctl__is_converged() {
   [ "${1-}" = compute ] || return 2
   [ "${2-}" = droplet ] || return 2
   [ -n "${4-}" ] || return 2
   case ${3-} in
   create)
      n=$(doctl compute droplet list "$4" --format Name --no-header 2>/dev/null | wc -l)
      case $n in
      0) return 1 ;;                               # absent — create it
      1) return 0 ;;                               # exactly one — converged
      *) printf 'decline hazard doctl: %s droplets named %s\n' "$n" "$4" \
            >>"${DREP_V1:-/dev/null}"
         return 2 ;;                               # ambiguous — run, and say why
      esac
      ;;
   delete) return 2 ;;                             # never vouch a destruction
   *) printf 'decline unmodeled doctl verb: %s\n' "${3-}" >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   esac
}


# ── 0b. the ssh oracle: the connection dance ───────────────────────────────
#
# `ssh` is a peeling wrapper, and a scope-changing one: it does not LEND the
# controller's user, filesystem view or network namespace to the remainder, it
# REPLACES the lot. Mapping the scope dimension is how this book says so; over
# there the four within-host dimensions are whatever they are, and the probe
# measures them there rather than guessing here.
#
# The `ssh host true` arm is the connection dance, and it is the ONLY thing
# this oracle vouches for. Read the rider carefully: reachable is a narrow
# cell. Port-open is not login-works (an author in the wild hit exactly that
# and wrote two loops with two predicates); login-works is not provisioned;
# and nothing about a live sshd says a boothook finished. The connection fact
# licenses its own cell and nothing else. The WIDE fold below — "the machine
# answers, therefore skip the entire standup" — is the ADMIN's judgment,
# spelled in their own guard line, attributed there, and correct because they
# said so, not because we proved it.

ssh__lend_map() {
   while [ $# -gt 0 ]; do
      case $1 in
      -o|-i|-p|-l|-F|-J|-b|-c|-D|-E|-L|-R|-S|-W|-w) shift 2 ;;
      --) shift; break ;;
      -*) shift ;;
      *) break ;;
      esac
   done
   [ -n "${1-}" ] || return 2
   printf '%s\n' "$1" : lends scope
   shift
   "$@"
}

ssh__predict() {
   while [ $# -gt 0 ]; do
      case $1 in
      -o|-i|-p|-l|-F|-J|-b|-c|-D|-E|-L|-R|-S|-W|-w) shift 2 ;;
      --) shift; break ;;
      -*) shift ;;
      *) break ;;
      esac
   done
   [ -n "${1-}" ] || return 2
   dest=$1; shift
   case ${1-} in
   true|:)
      ssh -o BatchMode=yes -o ConnectTimeout=5 "$dest" true \
         : sm.dorc.SshEndpoint:"$dest"@reachable
      ;;
   *) "$@" ;;
   esac
}

ssh__is_converged() { return 2 ;}   # ssh converges nothing; the payload might


# ── 0c. certbot ────────────────────────────────────────────────────────────
#
# The judgment, stated where it is made: a lineage that EXISTS counts as
# converged even three days from expiry, because certbot ships its own renewal
# timer and re-running `certonly` would only re-hit the ACME rate limit. That
# is an author's call about an adequacy gap (converged ≠ no-op), not a
# measurement — and when it is wrong, this line is the one to point at.

certbot__is_converged() {
   verb=; domain=
   while [ $# -gt 0 ]; do
      case $1 in
      -d|--domain) domain=$2; shift 2 ;;
      -m|--email|--cert-name|--webroot-path|-w) shift 2 ;;
      certonly|renew|run) verb=$1; shift ;;
      *) shift ;;
      esac
   done
   [ -n "$domain" ] || return 2
   case $verb in
   certonly) certbot certificates --cert-name "$domain" 2>/dev/null \
                | grep -q 'Expiry Date' ;;
   renew) return 2 ;;                        # renewal is time-keyed; never vouch
   *) return 2 ;;
   esac
}


# ── 1. the standup region ──────────────────────────────────────────────────
#
# One line decides whether any of this exists in the plan. On a day the machine
# answers, `! ssh … true` is provably false, the `then`-branch is dead by plain
# value-flow, and the whole region is OMITTED — no per-line vouches needed,
# no walls cast. On day zero it is true, and every line here runs.

if ! $SSH "root@$FQDN" true; then

   doctl compute droplet create "$DROPLET" \
      --region "$REGION" --size "$SIZE" --image "$IMAGE" \
      --ssh-keys "$SSHKEY" \
      --tag-names "role:web,book:$DROPLET" \
      --user-data-file ./build/ud-web1.txt \
      --wait

   # `--wait` blocks until the CREATE ACTION completes, which means "status is
   # active". It does not mean sshd is listening and it is nowhere near
   # "cloud-init finished" — a gap DigitalOcean users have had open as a
   # feature request for years. The two waits below are the book's, not the
   # vendor's.
   IP=$(doctl compute droplet get "$DROPLET" --format PublicIPv4 --no-header)

   doctl compute domain records list "$DOMAIN" --format Name,Data --no-header \
      | grep -q "^$DROPLET[[:space:]]*$IP\$" \
      || doctl compute domain records create "$DOMAIN" \
            --record-type A --record-name "$DROPLET" \
            --record-data "$IP" --record-ttl 60

   # Rebuild keeps the IP and changes the host key — DigitalOcean documents
   # both — so a stale known_hosts line is the NORMAL post-rebuild state, not
   # an attack. Both reference implementations in this space answer by turning
   # verification off permanently (`UserKnownHostsFile=/dev/null`,
   # `StrictHostKeyChecking=no`; Terraform disables it by default and says so).
   # This book narrows instead: forget the key ONLY on the path where the
   # controller itself just built the machine, and let `accept-new` re-pin it.
   ssh-keygen -R "$FQDN" >/dev/null 2>&1 || true
   ssh-keygen -R "$IP"   >/dev/null 2>&1 || true

   # The pivot's own wait, and the one wait that genuinely belongs on the
   # controller: whether a host is reachable is definitionally not observable
   # from inside that host. Every iteration is a fresh handshake, which is why
   # every OTHER wait in this pair of books lives inside the payload.
   #
   # Bounded the long way round on purpose. `for i in {1..60}` is the shape
   # everybody reaches for — including the k3s installer and the Kubernetes
   # docs' own init-container example — and under a POSIX shell it iterates
   # exactly once with `i` set to the literal string `{1..60}`. A retry cap
   # that does not cap is worse than no cap.
   tries=0
   until $SSH "root@$FQDN" true; do
      tries=$((tries + 1))
      [ "$tries" -lt 60 ] || exit 1
      sleep 5
   done

   # cloud-init's own wait verb: one connection, not a poll loop. Bounded from
   # out here because it has no bound of its own — a failing bootcmd leaves the
   # status at `running` rather than `error`, and `--wait` then blocks forever.
   timeout 600 $SSH "root@$FQDN" cloud-init status --wait || true
fi


# ── 2. in-host convergence, in the entered scope ───────────────────────────
#
# From here the book is ordinary. Each `$SSH root@… <cmd>` is a wrapper site;
# a contiguous run of them denoting one scope is one artifact on one
# connection, whatever the line count says. Certbot lives here rather than in
# the boothook for a reason no oracle could have worked out: ACME needs the A
# record to already point at this box, and §1 is where that became true.
$SSH "root@$FQDN" certbot certonly --nginx -n --agree-tos \
   -m ops@example.net -d "$FQDN"
$SSH "root@$FQDN" systemctl enable --now certbot.timer
$SSH "root@$FQDN" systemctl enable --now unattended-upgrades


# ── 3. the residue user-data cannot carry ──────────────────────────────────
#
# User-data is an instance attribute, readable back through IMDS by every
# process on the box for the life of the machine. Key material therefore
# travels this way instead: controller → ssh → the box, once, over a channel
# nothing later can replay. Last, so its unavoidable wall costs nothing.
$SSH "root@$FQDN" install -m 600 -D /dev/stdin /etc/restic/repo.pass \
   <./secrets/web1-restic.pass


# ── 4. the question the admin actually has ─────────────────────────────────
#
# Controller-side on purpose: a box can be perfectly converged from the inside
# and unreachable from the world. This line should run every single time, and
# it is not a defect that it does.
curl -fsS -o /dev/null "https://$FQDN/"
