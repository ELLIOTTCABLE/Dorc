#!/bin/sh
# Layer 3: a controller observation selects one action from a closed, approved set.
# The observed Patroni leader decides whether db-1 or db-2 receives the migration.

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

cluster_json=$(ssh_host db-control \
   'sudo -n patronictl -c /etc/patroni/patroni.yml list --format=json')

leader=$(
   printf '%s\n' "$cluster_json" |
      jq -er '
         [.[] | select((.Role | ascii_downcase) == "leader") | .Member]
         | if length == 1 then .[0]
           else error("expected exactly one Patroni leader")
           end
      '
) || {
   printf 'refusing: Patroni did not report exactly one leader\n' >&2
   exit 1
}

# The observation may select only one of these pre-approved destinations.
case "$leader" in
db-1|db-2) ;;
*)
   printf 'refusing unapproved leader name: %s\n' "$leader" >&2
   exit 1
   ;;
esac

# migration.sql stands for the exact migration reviewed before this invocation.
ssh_host "$leader" \
   'sudo -n -u postgres psql --set=ON_ERROR_STOP=1 --dbname=app' \
   < migration.sql
