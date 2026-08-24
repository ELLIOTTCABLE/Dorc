#!/bin/sh
# Layer 2: Advil's closed deploy-and-health outcome gates Beverly's approved work.
# Missing, failed, or timed-out health evidence withholds Beverly.

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

deploy_web() {
   host=$1
   ssh_host "$host" 'sudo -n sh -s' <<'WEB_BOOK'
set -eu
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y nginx
nginx -t
systemctl restart nginx
WEB_BOOK
}

wait_healthy() {
   name=$1
   case "$name" in
   ''|*[!A-Za-z0-9.-]*) return 64 ;;
   esac

   attempt=0
   while [ "$attempt" -lt 12 ]; do
      if curl --fail --silent --show-error --max-time 5 \
         "https://$name/healthz" >/dev/null
      then
         return 0
      fi
      attempt=$((attempt + 1))
      sleep 5
   done
   return 1
}

if deploy_web advil && wait_healthy advil.example.net; then
   printf 'advil admitted; advancing to beverly\n' >&2
else
   printf 'advil did not produce the closed healthy outcome; withholding beverly\n' >&2
   exit 1
fi

deploy_web beverly
wait_healthy beverly.example.net
