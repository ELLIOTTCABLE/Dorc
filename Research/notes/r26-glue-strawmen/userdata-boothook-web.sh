#cloud-boothook
#!/bin/sh
# dorc-lang/v0.2
# ╔══════════════════════════════════════════════════════════════════════════╗
# ║  STRAWMAN · IMAGINATION-TIER · NOT RUNNABLE · NEVER EXECUTE              ║
# ║                                                                          ║
# ║  Frozen evidence for the r26 ops-glue-residue round. Features spelled    ║
# ║  herein MAY NOT EXIST — this is a design target written against real     ║
# ║  cloud-init documentation, not a working script. Do not execute it, in   ║
# ║  whole or in part, not even a single "read-only" line. Every format,     ║
# ║  flag and spelling carries NO compat promise and will be renamed in      ║
# ║  place. Companion note: userdata-boothook-web.note.md                    ║
# ╚══════════════════════════════════════════════════════════════════════════╝
#
# ONE FILE, THREE DELIVERIES — and the same meaning in all three.
#
#   day zero, no Dorc anywhere
#       doctl compute droplet create web1 … --user-data-file userdata-boothook-web.sh
#       cloud-init strips line 1, writes the remainder 0700 under
#       /var/lib/cloud/instances/<iid>/boothooks/, and execs it directly —
#       honouring line 2's shebang — EVERY BOOT. It works with no Dorc in
#       sight: every mutation below is hand-guarded, in plain sh.
#
#   day zero, compiled
#       dorc compile userdata-boothook-web.sh >build/ud-web1.txt
#       …which is exactly the file pivot-vps-standup.sh hands to
#       `--user-data-file`. Same book. No probe phase exists (there is no host
#       yet), so nothing elides; every modeled site gains its oracle's own
#       check in front of the untouched original bytes, with the oracle bodies
#       inlined so the artifact stays self-contained. Guards only. Headless.
#       Why-log to file. Offline may NARROW (compile-time refusals); it never
#       FORKS meaning — the chef-solo two-code-path grave is right there.
#       Budget: DigitalOcean documents 64 KiB for user-data, Azure 64 KB,
#       GCP 256 KB, and EC2 16 KB raw. Size against the channel you are
#       actually shipping down, not against folklore.
#
#   day N, from the controller
#       dorc plan userdata-boothook-web.sh web1.example.net
#       Full probe → the converged lines leave the plan entirely.
#
# WHAT THIS FILE MUST NEVER CONTAIN: credential material. On EC2 the payload is
# an instance attribute retrievable through IMDS by any process on the box for
# the life of the instance. Azure is the careful counter-example worth knowing:
# custom-data is deliberately NOT surfaced through IMDS, Azure user-data is —
# and Azure advises against secrets in either ("We advise *not* to store
# sensitive data in custom data"). Treat the whole channel as world-readable and
# the per-cloud detail as a bonus you did not earn. Code and probe-shaped reads
# only; anything secret is pushed later, over ssh, by pivot-vps-standup.sh.
#
# AND ONE STRUCTURAL RULE THAT FALLS OUT OF DELIVERY #1: every Dorc mark in
# this file sits inside a function body the raw delivery never calls. A bare
# `pkg : sm.dorc.Package = "$1"` in the BOOK BODY would be a command-not-found
# on an un-stripped raw boot. Annotated oracle, unannotated book — that is the
# price of being your own payload.

set -eu

# cloud-init exports INSTANCE_ID to boothooks; ssh on day N does not. Under
# `set -u` the bare name is a day-N crash, so the read is defaulted.
: "${INSTANCE_ID:=unknown}"

SITE=web1.example.net
DOCROOT=/srv/www
CONF=/etc/nginx/sites-available/web1
SEED=https://dist.example.net/web1-docroot.tar.gz


# ── 0. an oracle, in the book, for the channel's own idiom ─────────────────
#
# `cloud-init-per <freq> <name> <cmd> [args…]` is upstream's answer to "run
# this once" inside an every-boot payload, and upstream's own boothook example
# is built on it. It eats two arguments, runs the remainder, then writes a sem
# file whose CONTENT is the inner command's rc and a timestamp. Its rationing
# is keyed on the NAME YOU GAVE IT, never on the world — the same smell as
# chezmoi's `run_onchange_` content hash.
#
# Describing it as a wrapper lets Dorc read straight through to the inner
# command and ration on state instead, while the admin keeps writing the
# channel's native idiom. Note what the book below does with that power:
# almost nothing. Only §6 still needs name-keyed rationing, and it needs it for
# a reason no state check could supply.
#
# Real warts this has to answer for:
#   - it hard-fails `must be root` when euid != 0 — day-zero root, day-N maybe not;
#   - it rewrites `-` to `_` in <name> (an old sem-migration bug, cloud-init
#     issue #3314, whose fix lives in the tool), so the sem path is not the
#     name you typed;
#   - freq=always writes a sem file it will never read.

cloud_init_per__lend_map() {
   [ $# -ge 3 ] || return 2
   case $1 in -h|--help) return 2 ;; esac
   shift 2                                   # <freq> <name>
   : lends user                              # no setuid, no su, no chroot
   : lends fsview
   : lends netns
   env "$@"                                  # ρ: full ambient passthrough
}

cloud_init_per__predict() {
   [ $# -ge 3 ] || return 2
   case $1 in once|instance|always) ;; *) return 2 ;; esac
   [ "$(id -u)" = 0 ] || return 2            # it would refuse; say so, do not guess
   shift 2
   "$@"                                      # peel: the inner command answers
}

# NOT DECLARED, deliberately: the sem-file write. It is this wrapper's own
# residue — vouched by the act of authoring the entry form, attributed to this
# line, never to the inner tool's author.


# ── 1. two waits, both host-observable, both living IN the payload ─────────
#
# A controller-side `until ssh …` costs one TCP+auth handshake per poll. Both
# of these are answerable from inside the box, so they belong here: one
# connection, total, however long they spin. Neither mutates. Neither may ever
# be elided — a resolver that answered at probe time can be down at apply time,
# and a lock free at probe time can be held at apply time. What they must earn
# is wall-transparency: a pure-delay loop that changes nothing must not degrade
# every line beneath it.

# The boothook runs in cloud-init's NETWORK stage — before every cloud-init
# module, and before anything guarantees the resolver answers.
until getent hosts deb.debian.org >/dev/null 2>&1; do sleep 1; done

# apt-daily.timer races cloud-init for the dpkg lock on firstboot; unfixed at
# the apt layer for years, and the community's answer is exactly this loop.
# `fuser` ships in psmisc, which minimal cloud images do not install — so the
# loop probes for its own tool before trusting it, and apt's own lock timeout
# below is the belt to this braces.
if command -v fuser >/dev/null 2>&1; then
   while fuser /var/lib/dpkg/lock-frontend >/dev/null 2>&1; do sleep 1; done
fi


# ── 2. packages ────────────────────────────────────────────────────────────
APT="apt-get -o DPkg::Lock::Timeout=120"

$APT update
dpkg -s nginx >/dev/null 2>&1 || $APT install -y nginx
dpkg -s unattended-upgrades >/dev/null 2>&1 || $APT install -y unattended-upgrades


# ── 3. firewall ────────────────────────────────────────────────────────────
#
# Before the service, so a half-applied boot never exposes an unconfigured
# nginx. Half-applied is the NORMAL case in this channel: a boothook that dies
# is caught, logged, and swallowed — cloud-init still reports `done`.
ufw allow 80/tcp
ufw allow 443/tcp
ufw --force enable


# ── 4. configuration ───────────────────────────────────────────────────────
#
# The heredoc IS the templating story: no second language, no `.j2`, and the
# bytes on screen are the bytes that land. The hand-written write-if-changed
# guard is the shape Dorc lifts — `cmp -s` is the read, `cp` is the mutation,
# and an unchanged config never reaches nginx's reload path.

[ -d "$DOCROOT" ] || install -d -m 755 "$DOCROOT"

cat >/run/web1.conf.new <<EOF
server {
   listen 80 default_server;
   listen [::]:80 default_server;
   server_name $SITE;
   root $DOCROOT;
   location / { try_files \$uri \$uri/ =404; }
}
EOF

cmp -s /run/web1.conf.new "$CONF" || {
   cp /run/web1.conf.new "$CONF"
   ln -sf "$CONF" /etc/nginx/sites-enabled/web1
   nginx -t
   systemctl reload nginx
}


# ── 5. service ─────────────────────────────────────────────────────────────
systemctl enable --now nginx


# ── 6. the one place name-keyed rationing is RIGHT ─────────────────────────
#
# Seed the docroot exactly once. After that the box's copy is the truth, and
# reconciling it would clobber whatever the operator edited in place. That is
# not a state check anyone can write — it is an intent, and `cloud-init-per`
# is how this channel spells it. §0 exists so Dorc reads the line rather than
# asking anyone to stop writing it.
[ -f /opt/web1-docroot.tar.gz ] || curl -fsSL -o /opt/web1-docroot.tar.gz "$SEED"
cloud-init-per instance seed-docroot tar -xzf /opt/web1-docroot.tar.gz -C "$DOCROOT"
