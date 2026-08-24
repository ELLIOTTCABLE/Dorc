#!/bin/sh
# Layer 7: Advil's apply-time outcome can create a new target and new work.
# The replacement instance does not exist when the initial fleet action begins.

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

healthy() {
   name=$1
   curl --fail --silent --show-error --max-time 5 \
      "https://$name/healthz" >/dev/null
}

list_running_instances() {
   aws ec2 describe-instances \
      --filters "Name=tag:aws:autoscaling:groupName,Values=$1" \
      --query 'Reservations[].Instances[?State.Name==`running`].InstanceId' \
      --output text |
      tr '\t' '\n' |
      sed '/^$/d' |
      sort -u
}

if deploy_web advil && healthy advil.example.net; then
   deploy_web beverly
   exit 0
fi

printf 'advil failed after execution; synthesizing replacement work now\n' >&2

asg=web-production
tmpdir=$(mktemp -d "${TMPDIR:-/tmp}/fleet-reactive.XXXXXX") || exit 1
trap 'rm -rf "$tmpdir"' EXIT HUP INT TERM

list_running_instances "$asg" >"$tmpdir/before"

# This infrastructure mutation and the later SSH target were not fixed at initial review time.
aws autoscaling set-desired-capacity \
   --auto-scaling-group-name "$asg" \
   --desired-capacity 3 \
   --honor-cooldown
aws autoscaling wait group-in-service --auto-scaling-group-names "$asg"

list_running_instances "$asg" >"$tmpdir/after"
comm -13 "$tmpdir/before" "$tmpdir/after" >"$tmpdir/new"

if [ "$(wc -l <"$tmpdir/new" | tr -d ' ')" -ne 1 ]; then
   printf 'refusing: expected exactly one replacement instance\n' >&2
   exit 1
fi

new_id=$(cat "$tmpdir/new")
if ! printf '%s\n' "$new_id" | grep -Eq '^i-[0-9a-f]+$'; then
   printf 'refusing malformed replacement instance id: %s\n' "$new_id" >&2
   exit 1
fi

aws ec2 wait instance-status-ok --instance-ids "$new_id"
new_host=$(
   aws ec2 describe-instances \
      --instance-ids "$new_id" \
      --query 'Reservations[0].Instances[0].PrivateDnsName' \
      --output text
)
case "$new_host" in
''|*[!A-Za-z0-9.-]*)
   printf 'refusing malformed replacement hostname: %s\n' "$new_host" >&2
   exit 1
   ;;
esac

deploy_web "$new_host"
