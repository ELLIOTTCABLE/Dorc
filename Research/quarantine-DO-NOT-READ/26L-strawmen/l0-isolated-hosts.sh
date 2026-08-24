#!/bin/sh
# Layer 0: independent host attempts. Neither host's result can affect the other.
# Strawman only: this is ordinary defensive admin shell, not proposed Dorc syntax.

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

apply_host() {
   host=$1
   ssh_host "$host" 'sudo -n sh -s' <<'HOST_BOOK'
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y chrony
systemctl enable --now chrony
HOST_BOOK
}

tmpdir=$(mktemp -d "${TMPDIR:-/tmp}/fleet-isolated.XXXXXX") || exit 1
trap 'rm -rf "$tmpdir"' EXIT HUP INT TERM

# Both attempts start before either result is observed.
apply_host advil >"$tmpdir/advil.log" 2>&1 &
advil_pid=$!
apply_host beverly >"$tmpdir/beverly.log" 2>&1 &
beverly_pid=$!

advil_rc=0
wait "$advil_pid" || advil_rc=$?
beverly_rc=0
wait "$beverly_pid" || beverly_rc=$?

sed 's/^/advil: /' "$tmpdir/advil.log"
sed 's/^/beverly: /' "$tmpdir/beverly.log"

if [ "$advil_rc" -ne 0 ] || [ "$beverly_rc" -ne 0 ]; then
   printf 'fleet incomplete: advil=%s beverly=%s\n' "$advil_rc" "$beverly_rc" >&2
   exit 1
fi
