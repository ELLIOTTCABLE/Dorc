#!/bin/sh
# deploy-widget.sh - install the widget config and (re)start its service.
#
# Strict POSIX sh: runs on dash/ash/busybox. No bashisms, no `set -o pipefail`
# (not POSIX), no `[[ ]]`, no arrays, no `eval`. Diagnostics/progress go to
# stderr with an ISO-8601 timestamp; the sole stdout line is the machine-
# readable result ("config updated" or "config unchanged").

set -eu

# Named exit codes, so callers/CI can branch on the failure mode.
readonly E_USAGE=2
readonly E_NOPRIV=3
readonly E_MKDIR=4
readonly E_WRITE=5
readonly E_RESTART=6

readonly PROG=${0##*/}
readonly WIDGET_DIR=/opt/widget
readonly CONFIG_PATH=${WIDGET_DIR}/config.env
readonly SERVICE=widget

dry_run=0
verbose=0
tmp_config=''
staged_config=''

# --- cleanup -----------------------------------------------------------------
# Registered before any temp file exists; the -n guards make an empty path a
# no-op. EXIT fires on normal exit and on the re-exit performed by the signal
# traps below, so cleanup runs exactly once on every path.
cleanup() {
   status=$?
   [ -n "$tmp_config" ] && rm -f "$tmp_config"
   [ -n "$staged_config" ] && rm -f "$staged_config"
   exit "$status"
}
trap cleanup EXIT
# Translate the common signals into conventional 128+N codes, then let the
# EXIT trap above do the actual cleanup (single source of truth).
trap 'exit 130' INT
trap 'exit 143' TERM
trap 'exit 129' HUP

# --- logging -----------------------------------------------------------------
# printf, never echo: portable, and the expanded text is always an argument to
# a literal %s format - never the format string itself - so a '%' or '\' in the
# data can't be misinterpreted. `date` is re-run per line (cheap at deploy
# volume) so timestamps stay honest across a slow systemctl restart.
_now() {
   date '+%Y-%m-%dT%H:%M:%S%z'
}

log() {
   printf '%s [%s] %s\n' "$(_now)" "$PROG" "$*" >&2
}

debug() {
   [ "$verbose" -eq 1 ] || return 0
   printf '%s [%s] debug: %s\n' "$(_now)" "$PROG" "$*" >&2
}

# die <exit-code> <message...>
die() {
   code=$1
   shift
   printf '%s [%s] error: %s\n' "$(_now)" "$PROG" "$*" >&2
   exit "$code"
}

usage() {
   cat <<EOF
Usage: $PROG [--dry-run] [--verbose] [-h|--help]

Ensure $WIDGET_DIR exists, write $CONFIG_PATH from the embedded template
(only when its bytes differ from what is already on disk), then restart the
'$SERVICE' service.

Options:
  --dry-run        Show what would change; touch nothing.
  -v, --verbose    Emit extra diagnostics (and, under --dry-run, a diff).
  -h, --help       Show this help and exit.

stdout carries the result ("config updated" / "config unchanged"); all
progress and errors go to stderr.
EOF
}

# --- embedded config template ------------------------------------------------
# Quoted heredoc delimiter => contents are emitted verbatim (no parameter or
# command expansion), so a literal '$' or backtick in the template is safe.
# This function is the single source of truth for the desired on-disk config.
render_config() {
   cat <<'EOF'
# Managed by deploy-widget.sh -- do not edit by hand.
WIDGET_ENV=production
WIDGET_LOG_LEVEL=info
WIDGET_LISTEN_ADDR=127.0.0.1:8080
WIDGET_MAX_WORKERS=4
EOF
}

# --- argument parsing --------------------------------------------------------
parse_args() {
   while [ "$#" -gt 0 ]; do
      case $1 in
         --dry-run)    dry_run=1 ;;
         -v|--verbose) verbose=1 ;;
         -h|--help)    usage; exit 0 ;;
         --)           shift; break ;;
         -*)           usage >&2; die "$E_USAGE" "unknown option: $1" ;;
         *)            usage >&2; die "$E_USAGE" "unexpected argument: $1" ;;
      esac
      shift
   done
   # Nothing positional is expected; reject leftovers after a literal `--`.
   [ "$#" -eq 0 ] || { usage >&2; die "$E_USAGE" "unexpected argument: $1"; }
}

# --- steps -------------------------------------------------------------------
ensure_dir() {
   if [ -d "$WIDGET_DIR" ]; then
      debug "directory present: $WIDGET_DIR"
      return 0
   fi
   if [ "$dry_run" -eq 1 ]; then
      log "DRY-RUN: would create directory $WIDGET_DIR"
      return 0
   fi
   log "creating directory $WIDGET_DIR"
   mkdir -p "$WIDGET_DIR" || die "$E_MKDIR" "could not create $WIDGET_DIR"
}

# Write the config only when its bytes differ from disk (idempotent).
# Prints exactly one of "config unchanged" / "config updated" to stdout;
# the human-readable narration goes to stderr via log/debug.
sync_config() {
   tmp_config=$(mktemp "${TMPDIR:-/tmp}/widget-config.XXXXXX") \
      || die "$E_WRITE" "could not create temporary file"
   render_config >"$tmp_config" \
      || die "$E_WRITE" "could not render config template"

   if [ -f "$CONFIG_PATH" ] && cmp -s "$tmp_config" "$CONFIG_PATH"; then
      debug "config byte-identical to $CONFIG_PATH"
      log "config unchanged: $CONFIG_PATH"
      printf 'config unchanged\n'
      return 0
   fi

   if [ "$dry_run" -eq 1 ]; then
      if [ -f "$CONFIG_PATH" ]; then
         log "DRY-RUN: would update $CONFIG_PATH (content differs)"
         if [ "$verbose" -eq 1 ]; then
            debug "diff (current -> desired):"
            diff -u "$CONFIG_PATH" "$tmp_config" >&2 || true
         fi
      else
         log "DRY-RUN: would create $CONFIG_PATH"
      fi
      printf 'config updated\n'
      return 0
   fi

   # Stage beside the target, then rename: an interrupted run can never leave a
   # half-written config in place. rename(2) is atomic only within one
   # filesystem, so stage in $WIDGET_DIR (not $TMPDIR, which may be tmpfs).
   staged_config=${CONFIG_PATH}.new.$$
   log "writing $CONFIG_PATH"
   cp "$tmp_config" "$staged_config" \
      || die "$E_WRITE" "could not stage $staged_config"
   mv "$staged_config" "$CONFIG_PATH" \
      || die "$E_WRITE" "could not move config into place: $CONFIG_PATH"
   staged_config=''
   log "config updated: $CONFIG_PATH"
   printf 'config updated\n'
}

restart_service() {
   if [ "$dry_run" -eq 1 ]; then
      log "DRY-RUN: would run: systemctl restart $SERVICE"
      return 0
   fi
   command -v systemctl >/dev/null 2>&1 \
      || die "$E_RESTART" "systemctl not found; cannot restart $SERVICE"
   log "restarting service: $SERVICE"
   systemctl restart "$SERVICE" \
      || die "$E_RESTART" "systemctl restart $SERVICE failed"
   log "service restarted: $SERVICE"
}

# --- main --------------------------------------------------------------------
main() {
   parse_args "$@"

   if [ "$dry_run" -eq 1 ]; then
      log "starting (dry-run): no changes will be made"
   else
      log "starting deploy"
      # Writing under /opt and driving systemd both need root. Fail early with
      # a clear message instead of a confusing mkdir/permission error several
      # steps in. `id -u` is POSIX and present on dash/ash systems.
      [ "$(id -u)" -eq 0 ] \
         || die "$E_NOPRIV" "must run as root (writes $WIDGET_DIR, restarts $SERVICE)"
   fi

   ensure_dir
   sync_config
   restart_service

   log "done"
}

main "$@"
