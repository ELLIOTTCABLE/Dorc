#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# minimal package oracle (apt/dpkg), lifted statically by dorc.
# command-keyed predict(): flag-strip (pre/post-verb), bind the verb, annotate the single
# operand as `package`; `[ "${2-}" = "" ]` refuses a second operand. The engine threads the
# inlined call's POSITIONAL-bound argv (`apt-get install -y nginx`, `$1`→nginx) through this.
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
