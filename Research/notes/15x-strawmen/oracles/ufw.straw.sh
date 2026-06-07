# ═══════════════════════════════════════════════════════════════════════
# oracle: ufw — host firewall rules                 [STRAWMAN, for discussion]
# ═══════════════════════════════════════════════════════════════════════
# SHAPE: no-script. ufw has no convergence dry-run we can lean on across versions,
# so the check bottoms out in asking ufw for its state (`ufw status`) and PARSING
# it. Two wrinkles this shape surfaces (notes):
#   - PRIVILEGE: `ufw status` needs root; an unprivileged probe errors, which must
#     read as UNKNOWN (rc 2), never "absent" (q-probe-privilege).
#   - PARSE FRAGILITY: ufw status is human-formatted; robust rule-equivalence
#     (port ranges, v6, 'Anywhere', app profiles) is not soundly matchable in sh.
#
# Handles only simple `allow|deny <rule>`; refuses anything richer. Streaming
# grep, no collect-all [sp-nobuffer]. Helpers plain POSIX.

ufw.check() {
   command -v ufw >/dev/null 2>&1 || { echo "ufw oracle: no ufw" >&2; return 2; }
   [ "$#" -le 2 ] || { echo "ufw oracle: complex rule form, refusing: $*" >&2; return 2; }
   local verb=$1 rule=$2 want
   case $verb in
      allow) want=ALLOW ;;
      deny)  want=DENY ;;
      *) echo "ufw oracle: no check for verb '$verb'" >&2; return 2 ;;
   esac
   [ -n "$rule" ] || { echo "ufw oracle: no rule" >&2; return 2; }
   case $rule in *[!a-zA-Z0-9/._-]*)
      echo "ufw oracle: rule too complex to match safely: $rule" >&2; return 2 ;;
   esac
   if ufw status 2>/dev/null | grep -qiE "(^|[[:space:]])$rule[[:space:]]+$want\b"; then
      return 0   # a matching rule is already present
   fi
   return 1
   # NOTE(q-probe-privilege): `ufw status` needs root; unprivileged -> ufw errors
   #   -> the pipe is empty -> we'd WRONGLY say would-change. A real probe must
   #   detect the privilege failure and return 2 (unknown). Not handled here; flagged.
   # HOLE(ceiling): rule-equivalence is unsound — '80/tcp' vs '80' vs ranges vs
   #   'Anywhere'/app-profiles don't normalise in sh.
}

ufw.version() {
   command -v ufw >/dev/null 2>&1 || return 2
   ufw version 2>/dev/null
}
