#!/bin/sh
# dorc-lang/v0.2
# ═══════════════════════════════════════════════════════════════════════════
#  FROZEN EVIDENCE · STRAWMAN · IMAGINATION-TIER
#  NOT RUNNABLE. NEVER EXECUTE — not this file, not a fragment of it, not by
#  hand and not by tool. It is a design document that happens to be shaped
#  like sh.
#  Features shown here MAY NOT EXIST and may never exist. Every Dorc spelling
#  is invented for this exhibit; NO format-, flag-, kind-name-, or wire-compat
#  is promised. Real command names, real flags, and real installer semantics
#  appear only so the exhibit is grounded in how the tools actually behave
#  (subiquity + curtin docs and source read 2026-07-28; citations in the
#  companion note).
#  The only sanctioned executor of fixture material in this repo is
#  `mise run test:e2e`, and this file is not fixture material.
# ═══════════════════════════════════════════════════════════════════════════
#
#  installer-latecommand-base.sh — the base-machine book, written once,
#  delivered twice.
#
#  DAY ZERO it is compiled to a self-contained guard artifact and dropped
#  into an Ubuntu autoinstall `late-commands` entry, where it runs as root
#  inside `curtin in-target` — chrooted to the installed system, with no
#  systemd manager, with daemons refused by policy-rc.d, minutes before the
#  machine's first boot. There is no controller, no probe phase, and nothing
#  can elide: every site that would become `# converged` on a networked plan
#  is a live runtime guard instead.
#
#  DAY N it is a book. `dorc plan installer-latecommand-base.sh box.example.net`
#  probes the running machine and folds most of it dead.
#
#  Same file. Same meaning. The delivery differs; the semantics do not — the
#  offline face may NARROW (compile-time refusals, verdicts unavailable) and
#  may never FORK (chef-solo's grave: `if Chef::Config[:solo]` in recipes).
#  Nothing below branches on how it was delivered. Where the two regimes
#  genuinely differ, the book branches on an OBSERVABLE HOST FACT, spelled in
#  sh, the way it would be spelled by an admin who had never heard of Dorc.

set -eu

ADMIN=ops
TZ_WANT=Etc/UTC


# ── 0. capability facts, bound once, at runtime ────────────────────────────
#
# `rul-capability-probing-per-feature` says capability-matching is Dorc's
# job, per-feature, per-host — never a ladder of tiers. In the networked lane
# that happens at plan time, on the controller, before a byte ships.
#
# Offline, the host does not exist yet when the artifact is compiled. So the
# same per-feature matching has to happen at RUNTIME, inside the artifact,
# and the artifact carries the probes it would otherwise have shipped. That
# is a narrowing (the answers arrive later and cannot license elisions), not
# a fork (the questions, and what they license, are identical).
#
# The three facts below are what three independent first-tier installers
# compute at the top of their own scripts. Nothing here is Dorc-shaped.

if [ "$(id -u)" = 0 ]; then SUDO=
elif command -v sudo >/dev/null 2>&1; then SUDO="sudo"
elif command -v doas >/dev/null 2>&1; then SUDO="doas"
else printf 'need root, or sudo, or doas\n' >&2; exit 1
fi

# THE fact this whole book turns on. `/run/systemd/system` exists only when
# systemd is running as the init of the current root — it is the canonical
# test, used by Debian's own maintainer-script helpers, and it is exactly
# false inside `curtin in-target` and exactly true on the booted machine.
#
# It is not a Dorc-ism and it is not a delivery flag. It is a question about
# the machine, asked in the idiom people already use, and it is the entire
# answer to "how does one file mean the same thing in a chroot and on a
# running box".
if [ -d /run/systemd/system ]; then INIT_LIVE=yes; else INIT_LIVE=no; fi


# ── 1. the admin account ───────────────────────────────────────────────────
#
# Self-contained by construction: the key is IN the file, not curl'd. The
# real-world norm is target-side egress (curl-in-everything), and inside a
# late-command that norm even holds — curtin copies the *installer's*
# resolv.conf into the target for the duration of each `in-target` call, so
# DNS appears to work. It is a trap, and §3 below is where it bites.
#
# Heredocs are the templating. There is no template engine here and there is
# not going to be one.

id "$ADMIN" >/dev/null 2>&1 || $SUDO useradd -m -s /bin/bash "$ADMIN"

$SUDO install -d -m 0700 -o "$ADMIN" -g "$ADMIN" "/home/$ADMIN/.ssh"
$SUDO tee "/home/$ADMIN/.ssh/authorized_keys" >/dev/null <<'EOF'
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAISTRAWMANNOTAREALKEYDONOTUSE ops@strawman
EOF
$SUDO chown "$ADMIN:$ADMIN" "/home/$ADMIN/.ssh/authorized_keys"
$SUDO chmod 0600 "/home/$ADMIN/.ssh/authorized_keys"


# ── 2. sshd ────────────────────────────────────────────────────────────────
#
# The change-signal idiom, hand-written, and the one that Dorc lifts best:
# validate, then act on the validation. `sshd -t` is a read-only config
# check with a real exit code — the kind of first-party verb that makes an
# oracle nearly free to write.

$SUDO tee /etc/ssh/sshd_config.d/10-base.conf >/dev/null <<'EOF'
PermitRootLogin no
PasswordAuthentication no
KbdInteractiveAuthentication no
EOF

$SUDO sshd -t

# Enabling is symlink manipulation and works with no manager running.
# Restarting is not, and does not. Same two lines on both days; on the
# installer's day the second one is guarded dead by a fact about the machine.
$SUDO systemctl enable ssh
if [ "$INIT_LIVE" = yes ]; then
   $SUDO systemctl restart ssh
fi


# ── 3. time ────────────────────────────────────────────────────────────────
#
# `timedatectl` talks to a running systemd over D-Bus and is therefore not
# available in the chroot at all. The offline spelling of the same intent is
# the symlink it manages. Both are in the book, guarded by the same fact,
# and the intent is identical — this is a narrowing of MECHANISM under an
# observed constraint, not a second meaning.
#
# And the DNS trap, stated where it belongs: a guard that asked "can this
# machine reach the NTP pool" would be answered, inside `curtin in-target`,
# by the INSTALLER's resolver, copied in for the duration of the call and
# removed again on the way out. The answer would be about a machine that is
# not this one. `hermeticity-precondition` already forbids licensing on a
# live-DNS probe (the `getent hosts` class); the installer environment is
# the sharpest real instance of why that rule exists, rather than a new
# exception to it. So: no reachability guard here. The book sets the zone
# and lets the daemon sort itself out on boot.

if [ "$INIT_LIVE" = yes ]; then
   $SUDO timedatectl set-timezone "$TZ_WANT"
else
   $SUDO ln -sfn "/usr/share/zoneinfo/$TZ_WANT" /etc/localtime
   printf '%s\n' "$TZ_WANT" | $SUDO tee /etc/timezone >/dev/null
fi


# ── 4. packages, and the wait that must be compiled IN ─────────────────────
#
# The canonical firstboot fix for the apt/dpkg lock race, unfixed at the apt
# layer for years, is literally this loop. It is dense in the wild.
#
# It is also the wait-placement doctrine's clean case: the awaited fact — a
# lock file on this machine's own disk — is observable from INSIDE the host.
# So the loop compiles into the artifact and costs one connection. A
# controller-side `until ssh box 'fuser …'` would cost one TCP handshake and
# one auth per second, forever, and would be the same cost cdist names as
# its own stated regret.
#
# In the chroot the lock is uncontended (no apt-daily timer under
# policy-rc.d); on day N against a machine that booted forty seconds ago it
# absolutely is contended. The loop belongs in both.

while $SUDO fuser /var/lib/dpkg/lock-frontend >/dev/null 2>&1; do
   sleep 1
done

$SUDO apt-get update
$SUDO apt-get install -y --no-install-recommends ufw unattended-upgrades chrony


# ── 5. firewall ────────────────────────────────────────────────────────────
#
# `ufw` writes rule files; the kernel state it wants only exists on a booted
# machine. `--dry-run` is not offered a vouch anywhere in this file (see the
# oracle), so these lines guard rather than elide when they cannot be proven.

$SUDO ufw --force reset >/dev/null
$SUDO ufw default deny incoming
$SUDO ufw allow 22/tcp
if [ "$INIT_LIVE" = yes ]; then
   $SUDO ufw --force enable
else
   $SUDO systemctl enable ufw
fi


# ── 6. the residue ─────────────────────────────────────────────────────────
#
# The vendor agent nobody will ever vouch for. It runs on every apply, on
# both days, forever, and everything after it verifies rather than elides.
# That is the honest product statement and it is not going to improve.

$SUDO hork enroll --site edge >>/var/log/hork.log 2>&1

# Deliberately last, because it is a wall: nothing below it could elide, so
# there is nothing below it. Wall placement is the admin's cheapest lever
# and no machinery substitutes for it.


# ═══════════════════════════════════════════════════════════════════════════
#  ORACLES
# ═══════════════════════════════════════════════════════════════════════════

# ── sshd ───────────────────────────────────────────────────────────────────
#
# `sshd -t` is the model first-party convergence verb: read-only, documented,
# real exit code. It answers a question about the CONFIG FILES, though, and
# not about the running daemon — so it vouches for the validation site and
# declines everything else. Naming what a check does not cover is most of
# the work of writing one.

sshd__is_converged() {
   case ${1:-} in
   -t|-T)
      sshd -t
      #: sm.dorc.File:/etc/ssh/sshd_config@valid
      ;;
   *) return 2 ;;
   esac
}

# ── systemctl, offline-aware ───────────────────────────────────────────────
#
# The interesting arm is `is-enabled`, because it is one of the few systemd
# queries that genuinely works with no manager running — it reads symlinks.
# `is-active` does not; in the chroot it answers about the installer's own
# systemd, which is a different machine's worth of state.
#
# So the oracle asks, in sh, the same question the book asked, and declines
# where the answer would be about the wrong root. A wrong-world measurement
# is the cardinal-sin shape; declining costs one run.

systemctl__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   unit=${1:-}

   case $verb in
   enable)
      systemctl is-enabled --quiet -- "$unit"
      #: sm.dorc.Service:"$unit"@enabled
      ;;
   start|restart|reload|stop)
      [ -d /run/systemd/system ] || {
         printf 'decline no-manager systemctl %s\n' "$verb" >>"${DREP_V1:-/dev/null}"
         return 2
      }
      case $verb in
      start)
         systemctl is-active --quiet -- "$unit"
         #: sm.dorc.Service:"$unit"@active
         ;;
      *) return 2 ;;
      esac
      ;;
   *) return 2 ;;
   esac
}

# ── ufw ────────────────────────────────────────────────────────────────────
#
# `ufw status` needs the kernel state, so under a chroot it is answering
# about the installer's netfilter tables. Declining there is not a
# limitation of the offline face — it is the same decline the same oracle
# would make on a live box whose kernel modules were missing.

ufw__is_converged() {
   [ -d /run/systemd/system ] || return 2
   verb=${1:-}; shift 2>/dev/null || :
   case $verb in
   allow|deny|limit)
      ufw status | grep -qF -- "${1:-}"
      #: sm.dorc.FirewallRule:"$1"@present
      ;;
   enable)
      ufw status | grep -q '^Status: active'
      #: sm.dorc.Firewall:local@active
      ;;
   reset|disable) return 2 ;;
   *) return 2 ;;
   esac
}

# ── the machine build, classed as a transit ────────────────────────────────
#
# The install is an epoch boundary: every host-scoped fact measured before
# the machine existed is invalidated by the machine coming into existence.
# Classing the verb is what lets a day-N plan reason about it at all.
#
# The load-bearing consequence is the pleasant one. On a firing day — the
# day the machine is actually built — the transit runs, and everything
# downstream honestly guards or runs. On every day after, the transit is
# CONVERGED, an elided command casts no wall, and the whole standup region
# above folds with full downstream elision intact. Transit verbs therefore
# rank at the top of any describability priority list: an unmodeled,
# unguarded transit walls every single day and kills the book.

curtin__predict() {
   case ${1:-} in
   install)
      #: transits epoch
      return 2 ;;
   in-target) return 2 ;;
   *) return 2 ;;
   esac
}
