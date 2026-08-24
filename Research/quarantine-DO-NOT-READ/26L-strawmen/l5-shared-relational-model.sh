#!/bin/sh
# Layer 5: the Consul-reported healthy membership set shapes loadbalancer's config.
# Multiple hosts' state is aggregated into one target host's desired world model.

set -eu
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

tmpdir=$(mktemp -d "${TMPDIR:-/tmp}/fleet-members.XXXXXX") || exit 1
trap 'rm -rf "$tmpdir"' EXIT HUP INT TERM

curl --fail --silent --show-error --max-time 10 \
   --cacert /etc/ops-pki/consul-ca.pem \
   --cert /etc/ops-pki/consul-client.pem \
   --key /etc/ops-pki/consul-client-key.pem \
   'https://consul.service.example.net:8501/v1/health/service/web?passing=true' \
   >"$tmpdir/health.json"

jq -er '
   [ .[] |
      { address: (if .Service.Address != ""
                  then .Service.Address
                  else .Node.Address
                  end),
        port: .Service.Port }
   ]
   | if length == 0 then error("no healthy web members") else . end
   | if all(.[];
        (.address | test("^[A-Za-z0-9.-]+$")) and
        (.port | type == "number" and . >= 1 and . <= 65535))
     then unique_by(.address, .port)
     else error("invalid member coordinate")
     end
   | .[]
   | "   server \(.address):\(.port) max_fails=3 fail_timeout=10s;"
' "$tmpdir/health.json" >"$tmpdir/servers.conf"

{
   printf 'upstream web {\n'
   cat "$tmpdir/servers.conf"
   printf '}\n'
} >"$tmpdir/web-upstream.conf"

# Install with a rollback copy, validate the whole nginx graph, then reload.
ssh_host loadbalancer '
set -eu
tmp=$(mktemp /tmp/nginx-web.XXXXXX)
backup=$(mktemp /tmp/nginx-web-old.XXXXXX)
had_old=0
cleanup() {
   rm -f "$tmp"
   sudo -n rm -f "$backup"
}
trap cleanup EXIT HUP INT TERM
cat >"$tmp"
if sudo -n test -e /etc/nginx/conf.d/web-upstream.conf; then
   sudo -n cp -p /etc/nginx/conf.d/web-upstream.conf "$backup"
   had_old=1
fi
sudo -n install -o root -g root -m 0644 "$tmp" /etc/nginx/conf.d/web-upstream.conf
if ! sudo -n nginx -t; then
   if [ "$had_old" -eq 1 ]; then
      sudo -n install -o root -g root -m 0644 "$backup" /etc/nginx/conf.d/web-upstream.conf
   else
      sudo -n rm -f /etc/nginx/conf.d/web-upstream.conf
   fi
   exit 1
fi
sudo -n systemctl reload nginx
' <"$tmpdir/web-upstream.conf"
