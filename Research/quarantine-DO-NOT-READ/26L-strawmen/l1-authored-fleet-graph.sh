#!/bin/sh
# Layer 1: an admin-authored, fixed host graph with an outcome-independent posture.
# Beverly always starts after Advil returns, even when Advil failed.
# This preserves the L1/L2 distinction at an intentionally uncomfortable safety cost.

set -u
umask 077

ssh_host() {
   host=$1
   shift
   case "$host" in
   ''|*[!A-Za-z0-9._-]*)
      printf 'refusing invalid SSH destination: %s\n' "$host" >&2
      return 64
      ;;
   esac

   ssh -T \
      -o BatchMode=yes \
      -o ClearAllForwardings=yes \
      -o ForwardAgent=no \
      -o StrictHostKeyChecking=yes \
      "$host" "$@"
}

apply_advil() {
   ssh_host advil 'sudo -n sh -s' <<'ADVIL_BOOK'
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y postgresql
systemctl enable --now postgresql
ADVIL_BOOK
}

apply_beverly() {
   ssh_host beverly 'sudo -n sh -s' <<'BEVERLY_BOOK'
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y nginx
systemctl enable --now nginx
BEVERLY_BOOK
}

# The graph says Advil precedes Beverly. The graph does not say success gates Beverly.
advil_rc=0
apply_advil || advil_rc=$?

beverly_rc=0
apply_beverly || beverly_rc=$?

if [ "$advil_rc" -ne 0 ] || [ "$beverly_rc" -ne 0 ]; then
   printf 'fleet incomplete: advil=%s beverly=%s\n' "$advil_rc" "$beverly_rc" >&2
   exit 1
fi
