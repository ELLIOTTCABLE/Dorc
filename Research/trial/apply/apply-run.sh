#!/usr/bin/env sh
#
# dorc-r25 ssh-apply runner (P3) — throwaway round-25 field-trial tooling.
#
# Contract C-run (plans/252 §1, §2):  apply-run(host, plan) -> {transcript, rc}
#   Take a plan (a POSIX-sh script) + a host, ssh in, run the plan, and capture
#   rc + stdout + stderr into a per-run artifact directory. P2's observer brackets
#   this call (snapshot before / after); P4 diffs the deltas. This is dorc's-eye
#   view of the apply (what it printed + how it exited), NOT the machine delta.
#
# The welded ssh-a-script floor (§1): `ssh host <sh> -s < plan`. Nothing fancier —
# no fan-out, no scheduler, no out-of-band signalling. On the Debian-12 trial box
# `sh` IS dash, so the plan runs under strict POSIX (set REMOTE_SH=dash to force it).
#
# The ssh gotcha (§5.2, P1 found it live): the human's ~/.ssh/config carries the
# macOS-only `usekeychain`, which git-bash's OpenSSH rejects (exit 255) — breaking
# ANY default-config ssh from Windows. We always ssh with `-F <sibling ssh_config>`,
# which makes ssh ignore ~/.ssh/config. We NEVER modify the human's config.
#
# Usage:
#   apply-run.sh apply-run [--out DIR] <host> <plan.sh|->     # ssh to host, apply
#   apply-run.sh apply-run --local [--out DIR] [host] <plan>  # run locally, no ssh
#   apply-run.sh selftest                                     # dry-test the capture
#   apply-run.sh help
#
# Emits a one-line JSON object (schema "C-run/1") on stdout; progress on stderr.
# Env: SSH_CONFIG SSH_KEY REMOTE_SH APPLY_TIMEOUT RUNS_ROOT LOCAL_SH  (see help).

set -eu

SELF="$0"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$SELF")" && pwd)"

SSH_CONFIG="${SSH_CONFIG:-$SCRIPT_DIR/ssh_config}"   # the usekeychain-free trial config
SSH_KEY="${SSH_KEY:-$HOME/.ssh/dorc-r25}"            # private key matching P1's injected pubkey
REMOTE_SH="${REMOTE_SH:-sh}"                          # remote interpreter (=dash on Debian target)
LOCAL_SH="${LOCAL_SH:-}"                              # local interpreter for --local (auto: dash|sh)
APPLY_TIMEOUT="${APPLY_TIMEOUT:-300}"                # seconds; 0 disables. Wraps the apply, not connect.
RUNS_ROOT="${RUNS_ROOT:-./dorc-r25-runs}"            # parent of auto-created run-dirs

log(){ printf '[apply-run] %s\n' "$*" >&2; }
die(){ log "ERROR: $*"; exit 2; }

# timeout(1) is GNU-named on Linux/git-bash, gtimeout on macOS (coreutils), absent on
# a bare macOS. Detect once; empty => run without a timeout (warned).
TIMEOUT_CMD=""
if command -v timeout >/dev/null 2>&1; then TIMEOUT_CMD="timeout"
elif command -v gtimeout >/dev/null 2>&1; then TIMEOUT_CMD="gtimeout"; fi

# Hand-rolled JSON string literal — used only on the jq-absent fallback path.
_json_str(){
   printf '%s' "$1" | sed -e 's/\\/\\\\/g' -e 's/"/\\"/g' \
      | awk 'BEGIN{printf "\""} {if(NR>1)printf "\\n"; printf "%s",$0} END{printf "\""}'
}

# --- the capture core ------------------------------------------------------
# Runs the transport ("$@") with stdin<plan, stdout>OUT, stderr>ERR, under a
# timeout, WITHOUT tripping errexit on a non-zero plan. Sets CAP_RC.
CAP_RC=0
_capture(){
   plan_in="$1"; shift
   set +e
   if [ "$APPLY_TIMEOUT" -gt 0 ] && [ -n "$TIMEOUT_CMD" ]; then
      "$TIMEOUT_CMD" "$APPLY_TIMEOUT" "$@" <"$plan_in" >"$OUT" 2>"$ERR"
   else
      "$@" <"$plan_in" >"$OUT" 2>"$ERR"
   fi
   CAP_RC=$?
   set -e
}

_bytes(){ wc -c <"$1" 2>/dev/null | tr -d ' \n'; }

# Scan captured stderr for canonical ssh transport failures (diagnosis aid; distinct
# from a plan's own stderr). Echoes a short reason or nothing.
_transport_error(){
   grep -aoiE 'bad configuration option|connection refused|connection timed out|connection closed|could not resolve|name or service not known|permission denied \(publickey|no route to host|host key verification failed|kex_exchange|operation timed out' "$ERR" 2>/dev/null | head -1
}

_compose_transcript(){ # $1=mode
   {
      echo "=== dorc-r25 apply-run transcript (C-run/1) ==="
      printf 'host:     %s\n' "$HOST"
      printf 'mode:     %s\n' "$1"
      printf 'plan:     %s\n' "$PLAN_ORIG"
      printf 'interp:   %s -s\n' "$INTERP"
      printf 'started:  %s\n' "$STARTED"
      printf 'ended:    %s\n' "$ENDED"
      printf 'duration: %ss\n' "$DUR"
      printf 'rc:       %s   (ssh_exit=%s transport_failed=%s timed_out=%s)\n' \
         "$CAP_RC" "$SSH_EXIT" "$TRANSPORT_FAILED" "$TIMED_OUT"
      printf -- '--- stdout (%s bytes) ---\n' "$(_bytes "$OUT")"; cat "$OUT"
      printf -- '--- stderr (%s bytes) ---\n' "$(_bytes "$ERR")"; cat "$ERR"
      echo "=== end transcript ==="
   } >"$DIR/transcript.txt"
}

_emit_summary(){ # writes meta.json in DIR and the one-line C-run JSON to stdout
   te="$(_transport_error || true)"
   if command -v jq >/dev/null 2>&1; then
      jq -nc \
         --arg host "$HOST" --arg mode "$1" --arg plan "$PLAN_ORIG" --arg interp "$INTERP" \
         --arg started "$STARTED" --arg ended "$ENDED" --argjson dur "$DUR" \
         --argjson rc "$CAP_RC" --argjson ssh_exit "$SSH_EXIT" \
         --argjson tf "$TRANSPORT_FAILED" --argjson to "$TIMED_OUT" \
         --arg te "$te" \
         --arg dir "$DIR" --arg out "$OUT" --arg err "$ERR" --arg tr "$DIR/transcript.txt" \
         '{schema:"C-run/1",host:$host,mode:$mode,plan:$plan,interp:$interp,
           started:$started,ended:$ended,duration_s:$dur,
           rc:$rc,ssh_exit:$ssh_exit,transport_failed:$tf,timed_out:$to,
           transport_error:(if $te=="" then null else $te end),
           dir:$dir,stdout:$out,stderr:$err,transcript:$tr}' \
         | tee "$DIR/meta.json"
   else
      log "WARN: jq not found — emitting hand-escaped JSON"
      if [ -n "$te" ]; then te_json="$(_json_str "$te")"; else te_json="null"; fi
      json="{\"schema\":\"C-run/1\",\"host\":$(_json_str "$HOST"),\"mode\":$(_json_str "$1"),\"plan\":$(_json_str "$PLAN_ORIG"),\"interp\":$(_json_str "$INTERP"),\"started\":$(_json_str "$STARTED"),\"ended\":$(_json_str "$ENDED"),\"duration_s\":$DUR,\"rc\":$CAP_RC,\"ssh_exit\":$SSH_EXIT,\"transport_failed\":$TRANSPORT_FAILED,\"timed_out\":$TIMED_OUT,\"transport_error\":$te_json,\"dir\":$(_json_str "$DIR"),\"stdout\":$(_json_str "$OUT"),\"stderr\":$(_json_str "$ERR"),\"transcript\":$(_json_str "$DIR/transcript.txt")}"
      printf '%s\n' "$json" | tee "$DIR/meta.json"
   fi
}

# --- apply-run subcommand --------------------------------------------------
cmd_apply_run(){
   LOCAL=0; RUN_DIR=""
   while [ $# -gt 0 ]; do
      case "$1" in
         --local|-L) LOCAL=1; shift ;;
         --out)      RUN_DIR="${2:?--out needs a DIR}"; shift 2 ;;
         --)         shift; break ;;
         -)          break ;;                 # lone '-' = read plan from stdin (a positional)
         -*)         die "unknown flag: $1 (try: $SELF help)" ;;
         *)          break ;;
      esac
   done

   if [ "$LOCAL" -eq 1 ]; then
      # local mode: host is optional. `<plan>` or `<host> <plan>`.
      if [ $# -eq 1 ]; then HOST="local"; PLAN_ORIG="$1"
      elif [ $# -eq 2 ]; then HOST="$1"; PLAN_ORIG="$2"
      else die "usage: $SELF apply-run --local [host] <plan.sh|->"; fi
   else
      [ $# -eq 2 ] || die "usage: $SELF apply-run <host> <plan.sh|->"
      HOST="$1"; PLAN_ORIG="$2"
   fi

   # Resolve the plan to a real file (slurp stdin when '-').
   plan_tmp=""
   if [ "$PLAN_ORIG" = "-" ]; then
      plan_tmp="$(mktemp)"; cat >"$plan_tmp"; PLAN_FILE="$plan_tmp"; PLAN_ORIG="<stdin>"
   else
      [ -r "$PLAN_ORIG" ] || die "plan not readable: $PLAN_ORIG"
      PLAN_FILE="$PLAN_ORIG"
   fi

   # Create the run-dir.
   if [ -n "$RUN_DIR" ]; then
      DIR="$RUN_DIR"; mkdir -p "$DIR"
   else
      mkdir -p "$RUNS_ROOT"
      stamp="$(date -u +%Y%m%dT%H%M%SZ)"
      slug="$(printf '%s' "$HOST" | tr -c 'A-Za-z0-9._-' '_')"
      DIR="$(mktemp -d "$RUNS_ROOT/${stamp}-${slug}-XXXXXX")"
   fi
   DIR="$(CDPATH= cd -- "$DIR" && pwd)"          # absolute path: a firm contract for P2/P4
   OUT="$DIR/stdout"; ERR="$DIR/stderr"
   cp "$PLAN_FILE" "$DIR/plan.sh"
   [ -n "$plan_tmp" ] && rm -f "$plan_tmp"

   STARTED="$(date -u +%Y-%m-%dT%H:%M:%SZ)"; t0="$(date +%s)"

   if [ "$LOCAL" -eq 1 ]; then
      # pick a local interpreter: prefer dash (faithful POSIX), else sh.
      li="$LOCAL_SH"
      [ -z "$li" ] && { command -v dash >/dev/null 2>&1 && li="dash" || li="sh"; }
      INTERP="$li (local)"; MODE=local
      log "apply-run LOCAL  interp=$li  plan=$PLAN_ORIG  dir=$DIR"
      _capture "$DIR/plan.sh" "$li" -s
      SSH_EXIT="$CAP_RC"; TRANSPORT_FAILED=false
   else
      [ -r "$SSH_CONFIG" ] || die "ssh config not found: $SSH_CONFIG"
      [ -r "$SSH_KEY" ]    || die "trial ssh key not found: $SSH_KEY (set SSH_KEY=<private key>)"
      INTERP="$REMOTE_SH (ssh $HOST)"; MODE=ssh
      log "apply-run SSH   host=$HOST  remote=$REMOTE_SH  cfg=$SSH_CONFIG  key=$SSH_KEY  dir=$DIR"
      _capture "$DIR/plan.sh" ssh -F "$SSH_CONFIG" -i "$SSH_KEY" -T "$HOST" "$REMOTE_SH" -s
      SSH_EXIT="$CAP_RC"
      # ssh's own convention: 255 == ssh-level failure (connect/config/auth), not the
      # plan's rc. A plan that genuinely `exit 255`s collides — documented; P4 can also
      # read transport_error/stderr. Anything <255 is the remote plan's true rc.
      if [ "$CAP_RC" -eq 255 ]; then TRANSPORT_FAILED=true; else TRANSPORT_FAILED=false; fi
   fi

   ENDED="$(date -u +%Y-%m-%dT%H:%M:%SZ)"; DUR="$(( $(date +%s) - t0 ))"

   # timeout(1) reports 124 (TERM) or 137 (KILL) when it fires.
   TIMED_OUT=false
   if [ -n "$TIMEOUT_CMD" ] && [ "$APPLY_TIMEOUT" -gt 0 ]; then
      { [ "$CAP_RC" -eq 124 ] || [ "$CAP_RC" -eq 137 ]; } && TIMED_OUT=true
   fi

   _compose_transcript "$MODE"
   printf '%s\n' "$CAP_RC" >"$DIR/rc"
   log "done  rc=$CAP_RC transport_failed=$TRANSPORT_FAILED timed_out=$TIMED_OUT  transcript=$DIR/transcript.txt"
   _emit_summary "$MODE"

   # Runner exit code mirrors the apply rc so a caller can `if apply-run ...`.
   exit "$CAP_RC"
}

case "${1:-}" in
   apply-run) shift; cmd_apply_run "$@" ;;
   selftest)  shift; exec "$SCRIPT_DIR/selftest.sh" "$@" ;;
   ""|-h|help|--help)
      cat >&2 <<EOF
dorc-r25 ssh-apply runner (P3).  Contract C-run: apply-run(host,plan) -> {transcript,rc}

  apply-run <host> <plan.sh|->            ssh to host (as root), run plan, capture rc/out/err
  apply-run --local [host] <plan.sh|->    run plan through a local shell, no ssh (dry-test)
  apply-run --out DIR ...                  write artifacts to DIR (default: auto under RUNS_ROOT)
  selftest                                 exercise the capture end-to-end, locally
  help

Artifacts per run (in the run-dir): stdout  stderr  rc  plan.sh  transcript.txt  meta.json
Stdout of THIS tool is one JSON line (schema C-run/1); progress goes to stderr.

Env (defaults):
  SSH_CONFIG=$SCRIPT_DIR/ssh_config   the usekeychain-free trial config (always -F'd)
  SSH_KEY=\$HOME/.ssh/dorc-r25         private key matching P1's cloud-init pubkey
  REMOTE_SH=sh                         remote interpreter (=dash on the Debian target; set =dash to force)
  APPLY_TIMEOUT=300                    seconds around the apply (0 = none); needs timeout/gtimeout
  RUNS_ROOT=./dorc-r25-runs            parent dir for auto-created run-dirs
  LOCAL_SH=                            --local interpreter (auto: dash if present, else sh)

Live wiring (DEFERRED — needs vultr.sh cherry-picked + human key-eyeball):
  vultr.sh run -- sh -c '"$SCRIPT_DIR"/apply-run.sh apply-run "\$DORC_HOST" plan.sh'
EOF
      exit 1 ;;
   *) die "unknown subcommand: $1 (try: $SELF help)" ;;
esac
