# ═══════════════════════════════════════════════════════════════════════
# oracle: systemctl — service state                [STRAWMAN, for discussion]
# ═══════════════════════════════════════════════════════════════════════
# SHAPE: query-verb. Unlike apt-get (one --dry-run flag), systemctl's mutating
# verbs each have a sibling READ verb — enable↔is-enabled, start↔is-active — so
# the oracle MAPS the verb; it does not pass "$@" through unchanged. The read
# verbs are inherently non-mutating, so the check is safe by construction.
#
# Findings this shape surfaces (notes):
#   - structured kind: a service has TWO selectors (enabled, active); `enable
#     --now` must satisfy BOTH.
#   - non-idempotent verbs: `restart` always does work → never "converged" → the
#     oracle reports would-change (rc 1) unconditionally, never skip.
#
# Verb-ladder [sp-verbladder]; helpers plain POSIX. rc [sp-checkrc]: 0·1·2.

_systemctl_is() {   # _systemctl_is enabled|active <unit> -> rc 0 iff in that state
   case $1 in
      enabled) systemctl is-enabled -- "$2" >/dev/null 2>&1 ;;
      active)  systemctl is-active  -- "$2" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}

systemctl.check() {
   command -v systemctl >/dev/null 2>&1 || { echo "systemctl oracle: no systemctl" >&2; return 2; }
   [ "$#" -ge 1 ] || { echo "systemctl oracle: no verb" >&2; return 2; }
   local verb=$1 now=false unit= a
   shift
   for a in "$@"; do
      case $a in
         --now) now=true ;;
         -*) ;;
         *) unit=$a ;;
      esac
   done
   [ -n "$unit" ] || { echo "systemctl oracle: no unit in '$verb $*'" >&2; return 2; }
   case $verb in
      enable)
         _systemctl_is enabled "$unit" || return 1
         if $now && ! _systemctl_is active "$unit"; then return 1; fi
         return 0 ;;
      start)   _systemctl_is active  "$unit" && return 0 || return 1 ;;
      disable) _systemctl_is enabled "$unit" && return 1 || return 0 ;;
      stop)    _systemctl_is active  "$unit" && return 1 || return 0 ;;
      restart|reload|try-restart|force-reload)
         return 1 ;;   # non-idempotent: always does work, never converged
      *) echo "systemctl oracle: no check for verb '$verb'" >&2; return 2 ;;
   esac
}

systemctl.version() {
   command -v systemctl >/dev/null 2>&1 || return 2
   systemctl --version 2>/dev/null
}

# transmute: book's `systemctl enable --now nginx` -> `systemctl.check enable --now nginx`
#
# HOLE(reserved): EFFECT/KIND — that `enable` establishes svc:<x>#enabled and that
#   the `active`/`enabled` selectors belong to ONE `service` kind is prose only.
# NOTE(channels): the queries use exit codes, not output — no collect-all needed.
