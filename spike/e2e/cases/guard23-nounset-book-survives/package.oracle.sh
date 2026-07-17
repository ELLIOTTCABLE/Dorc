#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# minimal package oracle (apt/dpkg) — the guard23 SET-U fixture variant (23C-fd2). The predict()
# is the corpus entity-resolver (its stripped body ships as the site's PROBE); the VOUCH is the
# authored verdict function `apt-get.is_converged()` (rul24-vouch-is-verdict-authoring, 24A §1c).
# This verdict body reads `"$2"` UNCONDITIONALLY (the corpus arity idiom) — under the book's
# `set -u` a single-operand invocation leaves `$2` unset, so the guard body DIES rc 2 at that read
# (the demonstrated hazard). The engine's `( check ) || <orig>` SUBSHELL contains the crash: the
# subshell exits nonzero, the `||` falls through, the mutator runs bare, and the book tail (vim)
# survives (kFAIL-perform — over-running a converged mutator is safe; the mechanism is engine's
# choice per human ruling h3, and the paren-subshell is one sanctioned mitigation).
# [Re-authored by the Stage-3 Part-A builder: added the verdict function; conductor-flagged.]
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"#installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 :! sm.dorc.Package:"$pkg"#installed ;;
      esac
   fi
}

# THE VOUCH (rul24-vouch-is-verdict-authoring): CONVERGED sense by name. The `[ "${2-}" = "" ]` arity
# gate reads `$2` unconditionally — under the book's `set -u` a single-operand guard invocation
# dies rc 2 here (23C-fd2). A multi-operand invocation reaches no check (`$2` non-empty ⇒ the `if`
# is false ⇒ Declined ⇒ no vouch ⇒ run — hz-refusepath). The dialect has no `return`, so declines
# are unhandled paths (tc-verdict-return).
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      esac
   fi
}
