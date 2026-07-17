#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# minimal package oracle (apt/dpkg), lifted statically by dorc.
# command-keyed predict(): the oracle's OWN argparse → inline kind-annotation (the real
# entity-resolution; task-W). Flag-strip (pre- and post-verb), bind the verb, annotate
# the single operand as `package`; the `[ "${2-}" = "" ]` guard refuses a SECOND operand
# (so `install nginx curl` resolves no probe ⇒ runs — no wrong single-entity elision).
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

# THE VOUCH (elide-weld, 24D §3): one apt-get provider serves BOTH package (install) and pkgindex
# (update), so ONE verdict function covers both establish verbs (build_vouches keys by provider,
# not by kind — a second `apt-get.is_converged` in pkgindex.oracle.sh would shadow this one).
# Declines purge + unknown via `*) return 2`.
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      update) test -n fresh ;;
      *) return 2 ;;
   esac
}
