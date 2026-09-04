#!/bin/sh
# ╔══════════════════════════════════════════════════════════════════════════╗
# ║  STRAWMAN · IMAGINATION-TIER · NOT RUNNABLE · NEVER EXECUTE              ║
# ║                                                                          ║
# ║  Frozen evidence for `plans/26O` (channels and stream routing), from the ║
# ║  2026-09-03 design-duck sitting. A design target, not a working script:  ║
# ║  do not execute it, in whole or in part, not even a "read-only" line.    ║
# ║  Every flag and spelling carries NO compat promise. Findings it produced ║
# ║  live in `26O` §8; this file is the specimen those findings cite.        ║
# ╚══════════════════════════════════════════════════════════════════════════╝
#
# THE ASK: one plain-sh book an admin runs for everything — quiet, probed, and
# elided on day N — that ALSO exploits a terminal when one is present, for the
# two moments that genuinely want a human: an interactive `terraform apply`, and
# first contact with a VPS whose provider hands out a root password, through a
# bastion. The only Dorc-ism is that `dorc apply --live` makes the first test
# true. Everything else is the shape a careful admin writes to make a script safe
# both by hand and from cron.
#
# Difficult cases exercised, each marked below: the tty dial · the argv-form
# payload that keeps stdin free · an expanding heredoc for one laptop value ·
# password entry with no batch equivalent · `sudo` that must never prompt in
# batch · a pager-proof line that runs on a pty · a reboot that severs its own
# session · the inverse wait. Bastion via `-J` (a TCP forward; one re-parse);
# the shell-only-bastion form `ssh -t bastion ssh -t target "$(…)"` re-parses
# twice and is where plain sh stops being kind.

set -eu

# The dial. Asked once, in the shape every cron-vs-by-hand script already uses.
# Dorc decides whether an apply has a pty, so it can fold this at plan time and
# render only the arm that will run (`26O:rul-tty-test-is-a-controller-fact`).
if [ -t 0 ] && [ -t 1 ]; then tty=-t; batch=; else tty=; batch=1; fi

# --- 1. standup: terraform owns the cloud side. The guard is terraform's own
#        exit vocabulary (rc 2 = changes pending), the remap arm an oracle carries.
cd ./infra
terraform init -input=false >/dev/null
if terraform plan -detailed-exitcode -input=false >/dev/null; then :; else
   case $? in
   2) terraform apply ${batch:+-input=false -auto-approve} ;;   # live: it shows and asks; you answer
   *) exit 1 ;;
   esac
fi
ip=$(terraform output -raw web_ip)      # ⊤ until value capture lands: the day-N critical path
cd ..

# --- 2. first contact: keys in, password auth out. Password entry has no batch
#        form, so the book says so and stops rather than pretending.
if ! ssh -o BatchMode=yes -J bastion "admin@$ip" true 2>/dev/null; then
   [ -z "$batch" ] || { printf 'first contact needs a terminal; rerun with --live\n' >&2; exit 4; }
   until ssh -o BatchMode=yes -o ConnectTimeout=5 -J bastion "root@$ip" true 2>&1 \
         | grep denied >/dev/null; do sleep 5; done      # sshd answered by refusing us: up, not yet ours
   # The argv form: the payload rides the command line, so stdin stays the keyboard
   # and `-t` can allocate the pty ssh will prompt on. An EXPANDING heredoc, on
   # purpose, for exactly one laptop value; every other `$` here is remote and there
   # are none — the siting hazard `26M` names, handled by hand.
   ssh -t -J bastion "root@$ip" "$(cat <<EOF
set -eu
adduser --disabled-password --gecos '' admin
install -d -m 700 -o admin -g admin /home/admin/.ssh
printf '%s\n' '$(cat ~/.ssh/id_ed25519.pub)' >/home/admin/.ssh/authorized_keys   # expands HERE, on the laptop
chown admin:admin /home/admin/.ssh/authorized_keys && chmod 600 /home/admin/.ssh/authorized_keys
printf 'admin ALL=(ALL) NOPASSWD:ALL\n' >/etc/sudoers.d/admin
passwd admin                                        # a terminal is here by construction: you type it
sed -i 's/^#\?PasswordAuthentication .*/PasswordAuthentication no/' /etc/ssh/sshd_config
systemctl reload ssh
EOF
)"
fi

# --- 3. converge, as admin. A QUOTED heredoc: nothing expands on the laptop.
#        `$tty` is unquoted on purpose (empty vanishes; the value is a literal from
#        this file, which the value plane sees). `sudo` may prompt live, never batch.
ssh $tty ${batch:+-o BatchMode=yes} -J bastion "admin@$ip" "$(cat <<'EOF'
set -eu
SUDO='sudo -n'; [ -t 0 ] && SUDO=sudo               # the prefix-variable head 26M parks
$SUDO apt-get update
dpkg -s nginx >/dev/null 2>&1 || $SUDO apt-get install -y nginx
$SUDO systemctl enable --now nginx
$SUDO systemctl --no-pager status nginx             # pager-proof: this line runs on a pty when live
[ -f /var/run/reboot-required ] && $SUDO reboot
exit 0
EOF
)" || [ "$?" -eq 255 ]      # a reboot severs the session; 255 is the expected exit, not a failure
                            # (sh cannot tell it from a remote 255 — the admin accepts that here)

# --- 4. the inverse wait, then verify from outside
until ssh -o BatchMode=yes -o ConnectTimeout=5 -J bastion "admin@$ip" true 2>/dev/null; do sleep 5; done
curl -fsS "http://$ip/" >/dev/null && printf 'web: ok\n'
