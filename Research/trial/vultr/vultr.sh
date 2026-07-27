#!/usr/bin/env bash
#
# dorc-r26 Vultr substrate — throwaway field-trial tooling (born in round 25).
#
# Contract C-vps (plans/252 §2):  provision -> {id,ip,host}  ·  snapshot <id>
#   ·  restore <snapshot-id>  ·  destroy <id>   (+ status, destroy-all, run).
#
# Guardrail (plans/252 §5.1, BINDING):
#   1 isolation   every resource carries the `dorc-r26` tag + name prefix
#   2 spend       cheapest tier clearing the round's floor (PLAN); <=3 concurrent; <$10/day
#   3 teardown    `run` always tears down; `provision` tears down its own failures
#   4 key         sourced from ~/.temp/vultr.env into env ONLY; never printed/logged
#   6 observe     `status` = live instances + snapshots + rough accrued spend
#
# The human's ~/System/Infrastructure/vultr-create-instance.zx.mjs is the crib
# (config shape, os_id 2136 = Debian 12, <name>-the-<plan> convention); it drives
# raw curl+`op read`, but §5.1.4 mandates vultr-cli + an env-sourced key here.

set -euo pipefail

# A traced shell (`bash -x`) would echo the key while sourcing the env-file. Refuse.
case $- in *x*) echo "vultr.sh: refusing to run under 'set -x' (would leak VULTR_API_KEY)" >&2; exit 1 ;; esac

TAG="dorc-r26"                                  # isolation tag AND name prefix — never widen
# 2GB is a floor, not a taste: the acceptance book installs grafana+prometheus+HA
# (255 §8), which the 1GB tier OOMs. Cheapest 2GB plan offered in REGION.
PLAN="${PLAN:-vc2-1c-2gb}"                       # 1 vCPU / 2GB / 55GB (~$0.0137/hr)
REGION="${REGION:-ewr}"
OS_ID="${OS_ID:-2136}"                           # Debian 12 x64 (bookworm)
SSHKEY="${SSHKEY:-}"                             # optional pubkey path, injected via cloud-init
ENV_FILE="${VULTR_ENV_FILE:-$HOME/.temp/vultr.env}"
MAX_CONCURRENT="${MAX_CONCURRENT:-3}"
ACTIVE_TIMEOUT="${ACTIVE_TIMEOUT:-240}"          # seconds to wait for status=active + IP
SSH_TIMEOUT="${SSH_TIMEOUT:-180}"                # seconds to wait for port 22 to answer
SNAP_TIMEOUT="${SNAP_TIMEOUT:-600}"              # seconds to wait for a snapshot to finish

log(){ printf '[%s] %s\n' "$TAG" "$*" >&2; }
die(){ log "ERROR: $*"; exit 1; }

# --- key handling (§5.1.4) -------------------------------------------------
# Source the key into env if the caller has not already. Never echo it.
AUTH_OK=0
_load_key(){
   if [ -z "${VULTR_API_KEY:-}" ]; then
      [ -f "$ENV_FILE" ] || die "no key: $ENV_FILE missing and VULTR_API_KEY unset — creating no resources"
      set -a; . "$ENV_FILE"; set +a
   fi
   [ -n "${VULTR_API_KEY:-}" ] || die "no key: VULTR_API_KEY empty after sourcing $ENV_FILE — creating no resources"
}

# vultr-cli reads VULTR_API_KEY from env; --config points at an empty file so the
# absent ~/.vultr-cli.yaml warning never pollutes stdout/JSON.
EMPTY_CFG="$(mktemp)"; : > "$EMPTY_CFG"
vc(){ vultr-cli --config "$EMPTY_CFG" "$@"; }
_strip(){ sed -n '/[[{]/,$p'; }                  # drop any pre-JSON warning line

_auth_check(){
   [ "$AUTH_OK" = 1 ] && return 0
   _load_key
   if ! vc account >/dev/null 2>&1; then
      die "auth FAILED (expired/revoked key or IP not allowlisted). HARD STOP — no resources created. Relay up: SendMessage \"main\" (key may need rotation)."
   fi
   AUTH_OK=1
}

# --- helpers ---------------------------------------------------------------
_shortid(){ printf '%s-%s' "$(date -u +%m%d-%H%M%S)" "$(head -c2 /dev/urandom | od -An -tx1 | tr -d ' \n')"; }

_instance_json(){                                # echo the instance object, or empty if gone
   local rec; rec="$(vc instance get "$1" -o json 2>/dev/null | _strip)" || true
   printf '%s' "$rec" | jq -c '(.instance // .) | select(.id != null)' 2>/dev/null || true
}

# C-vps spells this destroy(host); accept an id OR an IPv4. An IPv4 resolves only
# among $TAG boxes — an untagged IP resolves to nothing and falls through
# as a bogus id (-> "not found" no-op), so it can never target the human's box.
_resolve_id(){
   case "$1" in
      *[!0-9.]* | "") printf '%s' "$1"; return 0 ;;    # has non-[digit/dot] -> it's an id (UUID)
   esac
   local id; id="$(vc instance list -o json | _strip | jq -r --arg ip "$1" --arg t "$TAG" \
      '.instances[]|select(.main_ip==$ip and (.tags|index($t)))|.id' | head -1)"
   [ -n "$id" ] && printf '%s' "$id" || printf '%s' "$1"
}

# THE safety gate: refuse to act on anything not carrying our tag.
_assert_tagged(){
   local obj="$1" id="$2"
   if ! printf '%s' "$obj" | jq -e --arg t "$TAG" '(.tags // []) | index($t)' >/dev/null 2>&1; then
      local tags label
      tags="$(printf '%s' "$obj" | jq -rc '.tags // []')"
      label="$(printf '%s' "$obj" | jq -r '.label // "?"')"
      die "REFUSING to touch instance $id — tag '$TAG' NOT present (tags=$tags label=$label). Not a $TAG resource."
   fi
}

_guard_concurrency(){
   local n; n="$(vc instance list -o json | _strip | jq --arg t "$TAG" '[.instances[]|select(.tags|index($t))]|length')"
   [ "$n" -lt "$MAX_CONCURRENT" ] || die "concurrency cap: $n live $TAG instances >= MAX_CONCURRENT=$MAX_CONCURRENT. Reap before provisioning more."
}

_make_userdata(){                                # cloud-init that injects an ssh pubkey for root
   local keyfile="$1" tmp
   [ -f "$keyfile" ] || die "SSHKEY not found: $keyfile"
   tmp="$(mktemp)"
   { printf '#cloud-config\nssh_authorized_keys:\n'; printf '  - %s\n' "$(cat "$keyfile")"; } > "$tmp"
   printf '%s' "$tmp"
}

_wait_active(){                                  # echo the IP once active
   local id="$1" deadline status ip
   deadline=$(( $(date +%s) + ACTIVE_TIMEOUT ))
   while :; do
      local rec; rec="$(_instance_json "$id")"
      status="$(printf '%s' "$rec" | jq -r '.status // "?"' 2>/dev/null || echo '?')"
      ip="$(printf '%s' "$rec" | jq -r '.main_ip // "0.0.0.0"' 2>/dev/null || echo 0.0.0.0)"
      { [ "$status" = active ] && [ -n "$ip" ] && [ "$ip" != 0.0.0.0 ]; } && { printf '%s' "$ip"; return 0; }
      [ "$(date +%s)" -ge "$deadline" ] && die "timeout ($ACTIVE_TIMEOUT s) waiting for $id active (last: status=$status ip=$ip)"
      sleep 6
   done
}

_wait_ssh(){                                     # auth-less reachability: ssh-keyscan speaks SSH
   local ip="$1" deadline
   deadline=$(( $(date +%s) + SSH_TIMEOUT ))
   while :; do
      ssh-keyscan -T 5 -p 22 "$ip" 2>/dev/null | grep -q ssh && return 0
      [ "$(date +%s)" -ge "$deadline" ] && die "timeout ($SSH_TIMEOUT s) waiting for SSH:22 on $ip"
      sleep 5
   done
}

# provision arms this so a failure mid-wait never leaves an orphan.
PROVISIONED_ID=""
_fail_cleanup(){
   [ -n "$PROVISIONED_ID" ] || return 0
   if [ "${KEEP_ON_FAIL:-0}" = 1 ]; then
      log "provision failed but KEEP_ON_FAIL=1 — LEAVING $PROVISIONED_ID up (reap manually: destroy $PROVISIONED_ID)"
      return 0
   fi
   log "provision failed — tearing down $PROVISIONED_ID (no orphan)"
   cmd_destroy "$PROVISIONED_ID" >/dev/null 2>&1 || log "WARN: teardown FAILED — $PROVISIONED_ID is an ORPHAN, reap manually + notify human"
}

# --- subcommands -----------------------------------------------------------
cmd_provision(){
   _auth_check
   _guard_concurrency
   local name udfile="" args rec id ip
   name="$TAG-$(_shortid)"
   [ -n "$SSHKEY" ] && udfile="$(_make_userdata "$SSHKEY")"
   log "provisioning $name  plan=$PLAN region=$REGION os=$OS_ID tag=$TAG"
   args=(instance create --region "$REGION" --plan "$PLAN" --os "$OS_ID" --host "$name" --label "$name" --tags "$TAG" -o json)
   [ -n "$udfile" ] && args+=(--userdata-file "$udfile")
   rec="$(vc "${args[@]}" | _strip)"
   [ -n "$udfile" ] && rm -f "$udfile"
   id="$(printf '%s' "$rec" | jq -r '(.instance // .).id // empty')"
   [ -n "$id" ] || die "instance create returned no id: $(printf '%s' "$rec" | head -c 200)"
   PROVISIONED_ID="$id"; trap _fail_cleanup EXIT
   log "created id=$id — waiting for active + IP (<=${ACTIVE_TIMEOUT}s)"
   ip="$(_wait_active "$id")"
   log "active ip=$ip — waiting for SSH:22 (<=${SSH_TIMEOUT}s)"
   _wait_ssh "$ip"
   PROVISIONED_ID=""; trap - EXIT               # success: keep the box, disarm teardown
   log "SSH reachable on $ip"
   printf '{"id":"%s","ip":"%s","host":"%s","label":"%s"}\n' "$id" "$ip" "$ip" "$name"
}

cmd_snapshot(){
   _auth_check
   local id obj desc rec sid
   id="$(_resolve_id "${1:?usage: snapshot <instance-id|ip>}")"
   obj="$(_instance_json "$id")"
   [ -n "$obj" ] || die "instance $id not found — nothing to snapshot"
   _assert_tagged "$obj" "$id"                   # only snapshot our own
   desc="$TAG snapshot of $id @ $(date -u +%Y-%m-%dT%H:%M:%SZ)"
   rec="$(vc snapshot create -i "$id" -d "$desc" -o json | _strip)"
   sid="$(printf '%s' "$rec" | jq -r '(.snapshot // .).id // empty')"
   [ -n "$sid" ] || die "snapshot create returned no id: $(printf '%s' "$rec" | head -c 200)"
   log "snapshot $sid created (desc-prefixed '$TAG'; Vultr snapshots take no tags)"
   [ "${NOWAIT:-0}" = 1 ] || { log "waiting for snapshot $sid to complete (<=${SNAP_TIMEOUT}s; NOWAIT=1 to skip)"; _wait_snapshot "$sid"; }
   printf '%s\n' "$sid"
}

_wait_snapshot(){
   local sid="$1" deadline st
   deadline=$(( $(date +%s) + SNAP_TIMEOUT ))
   while :; do
      st="$(vc snapshot get "$sid" -o json 2>/dev/null | _strip | jq -r '(.snapshot // .).status // "?"' 2>/dev/null || echo '?')"
      [ "$st" = complete ] && { log "snapshot $sid complete"; return 0; }
      [ "$(date +%s)" -ge "$deadline" ] && { log "snapshot $sid still '$st' after ${SNAP_TIMEOUT}s (not fatal)"; return 0; }
      sleep 8
   done
}

cmd_restore(){
   _auth_check
   local sid="${1:?usage: restore <snapshot-id>}" srec name rec id ip
   srec="$(vc snapshot get "$sid" -o json 2>/dev/null | _strip)" || true
   printf '%s' "$srec" | jq -e --arg t "$TAG" '(.snapshot // .).description | startswith($t)' >/dev/null 2>&1 \
      || die "REFUSING to restore $sid — its description is not '$TAG'-prefixed (not our snapshot)"
   _guard_concurrency
   name="$TAG-$(_shortid)"
   log "restoring snapshot $sid -> $name  plan=$PLAN region=$REGION"
   rec="$(vc instance create --region "$REGION" --plan "$PLAN" --snapshot "$sid" --host "$name" --label "$name" --tags "$TAG" -o json | _strip)"
   id="$(printf '%s' "$rec" | jq -r '(.instance // .).id // empty')"
   [ -n "$id" ] || die "restore create returned no id: $(printf '%s' "$rec" | head -c 200)"
   PROVISIONED_ID="$id"; trap _fail_cleanup EXIT
   ip="$(_wait_active "$id")"
   _wait_ssh "$ip"
   PROVISIONED_ID=""; trap - EXIT
   log "restored id=$id ip=$ip SSH-reachable"
   printf '{"id":"%s","ip":"%s","host":"%s","label":"%s"}\n' "$id" "$ip" "$ip" "$name"
}

cmd_destroy(){
   _auth_check
   local id obj
   id="$(_resolve_id "${1:?usage: destroy <instance-id|ip>}")"
   obj="$(_instance_json "$id")"
   if [ -z "$obj" ]; then log "instance $id not found — already gone"; return 0; fi
   _assert_tagged "$obj" "$id"                   # dies unless $TAG-tagged
   log "destroying $id (tag '$TAG' verified) label=$(printf '%s' "$obj" | jq -r .label)"
   vc instance delete "$id"
   sleep 3
   [ -z "$(_instance_json "$id")" ] && log "instance $id gone" || log "delete issued for $id (still resolving)"
}

cmd_destroy_all(){                               # the manual reaper (§5.1.3 backstop)
   _auth_check
   log "reaping ALL $TAG resources"
   local ids sids id sid
   ids="$(vc instance list -o json | _strip | jq -r --arg t "$TAG" '.instances[]|select(.tags|index($t))|.id')"
   for id in $ids; do cmd_destroy "$id"; done
   sids="$(vc snapshot list -o json | _strip | jq -r --arg t "$TAG" '.snapshots[]|select(.description|startswith($t))|.id')"
   for sid in $sids; do log "deleting snapshot $sid"; vc snapshot delete "$sid"; done
   [ -z "$ids$sids" ] && log "nothing tagged $TAG — clean" || log "reap complete"
}

cmd_status(){
   _auth_check
   local ij sj cost
   ij="$(vc instance list -o json | _strip)"
   sj="$(vc snapshot list -o json | _strip)"
   # Reduce the (huge) plan list to a compact id->monthly_cost map; the full list
   # blows the argv length limit if passed to jq directly.
   cost="$(vc plans list -o json | _strip | jq -c '[.plans[]|{key:.id,value:.monthly_cost}]|from_entries')"
   jq -rn --argjson inst "$ij" --argjson cost "$cost" --argjson snaps "$sj" --arg t "$TAG" '
        [ $inst.instances[]  | select(.tags|index($t)) ]              as $mine
      | [ $snaps.snapshots[] | select(.description|startswith($t)) ]  as $snap
      | ($mine | map( ($cost[.plan]//0)/730 * ((now-((.date_created[0:19]+"Z")|fromdateiso8601))/3600) ) | add // 0) as $spend
      | "== \($t) status ==",
        "instances (\($mine|length), cap 3):",
        ( if ($mine|length)==0 then "  (none)"
          else ($mine[] | "  \(.id)  \(.label)  \(.main_ip)  \(.status)  \(.plan)  \(.region)  created=\(.date_created)")
          end ),
        "snapshots (\($snap|length)):",
        ( if ($snap|length)==0 then "  (none)"
          else ($snap[] | "  \(.id)  \(.status)  \((.size//0)/1073741824*100|floor/100)GB  \(.description)")
          end ),
        "rough accrued spend (live instances, monthly/730 * uptime): $\(($spend*100|floor)/100)  [cap <$10/day]"
   '
}

cmd_run(){                                       # provision -> run cmd -> ALWAYS destroy (§5.1.3)
   [ "${1:-}" = "--" ] && shift
   [ $# -ge 1 ] || die "usage: run -- <command...>  (env DORC_ID/DORC_IP/DORC_HOST exported to it)"
   _auth_check
   local out id ip
   out="$(cmd_provision)"
   id="$(printf '%s' "$out" | jq -r .id)"; ip="$(printf '%s' "$out" | jq -r .ip)"
   trap 'cmd_destroy "$id" >/dev/null 2>&1 || log "WARN: teardown FAILED — $id ORPHAN, reap + notify"' EXIT INT TERM
   log "run: box $id up @ $ip — executing: $*"
   DORC_ID="$id" DORC_IP="$ip" DORC_HOST="$ip" "$@"
}

case "${1:-}" in
   provision)    shift; cmd_provision "$@" ;;
   snapshot)     shift; cmd_snapshot "$@" ;;
   restore)      shift; cmd_restore "$@" ;;
   destroy)      shift; cmd_destroy "$@" ;;
   destroy-all)  shift; cmd_destroy_all "$@" ;;
   status)       shift; cmd_status "$@" ;;
   run)          shift; cmd_run "$@" ;;
   ""|-h|--help)
      cat >&2 <<EOF
$TAG Vultr substrate.  All resources tagged/prefixed '$TAG'.
  provision                 create cheapest Debian-12 box -> {id,ip,host} JSON
  snapshot <id|ip>          snapshot a $TAG box -> snapshot-id
  restore  <snapshot-id>    new box from a $TAG snapshot -> {id,ip,host}
  destroy  <id|ip>          delete (REFUSES anything not $TAG-tagged)
  destroy-all               reap every $TAG instance + snapshot
  status                    live instances + snapshots + rough spend
  run -- <cmd...>           provision, run cmd (DORC_IP/ID/HOST in env), ALWAYS destroy
Env: PLAN=$PLAN REGION=$REGION OS_ID=$OS_ID SSHKEY=<pubkey> KEEP_ON_FAIL=0
EOF
      exit 1 ;;
   *) die "unknown subcommand: $1 (try --help)" ;;
esac
