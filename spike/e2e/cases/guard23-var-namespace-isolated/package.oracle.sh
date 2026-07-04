# minimal package oracle (apt/dpkg) — the guard23 VAR-CAPTURE fixture variant (23C-fd1). The
# predict() is the corpus entity-resolver (its stripped body ships as the site's PROBE); the VOUCH
# is the authored verdict function `apt-get.is_converged()` (rul24-vouch-is-verdict-authoring,
# 24A §1c). This verdict body assigns `pkg` BARE (`pkg="$1"`) — the corpus idiom — which, shipped
# verbatim (strip-only is law), would clobber the book's OWN `pkg` variable in the caller namespace
# (POSIX functions share it). The engine's `( check ) || <orig>` SUBSHELL contains the assignment:
# `pkg=curl` lives in the subshell only, the book's `pkg=vim` survives, and the final
# `apt-get install -y "$pkg"` still installs VIM (the mechanism is engine's choice per human ruling
# h3; the paren-subshell is one sanctioned mitigation).
# [Re-authored by the Stage-3 Part-A builder: added the verdict function; conductor-flagged.]
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : package = "$1"
   if [ "$2" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 : package:"$pkg".installed! ;;
      esac
   fi
}

# THE VOUCH (rul24-vouch-is-verdict-authoring): CONVERGED sense by name. The `pkg="$1"` assignment
# is the bare capture the corpus idiom uses — shipped verbatim it clobbers the book's `pkg`
# (23C-fd1), contained by the emitter's guard subshell. An unmodeled verb reaches no arm ⇒ Declined
# ⇒ no vouch ⇒ run (hz-refusepath). The dialect has no `return`, so declines are unhandled paths.
apt-get.is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg="$1"
   case $verb in
      install) dpkg-query -W "$pkg" >/dev/null 2>&1 ;;
   esac
}
