#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# minimal package oracle (apt/dpkg) — predict() + touches() (STRAWMAN spellings, 24A §1b).
# predict(): the oracle's OWN argparse -> inline kind-annotation (entity-resolution; task-W).
# touches(): the at-most FOOTPRINT — a third role-sibling, same argparse, that printfs the
# entity-coordinates the verb mutates (one per line, kind:entity). Lifted STATICALLY (never
# shipped/run this stage). install/purge both touch package:<operand>.
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

apt_get__disturbs() {                              # STRAWMAN footprint spelling (24A §1b)
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install|purge) printf '%s\n' "$1" : sm.dorc.Package ;;
   esac
}

# THE VOUCH (elide-weld, 24D §3 / rul24-vouch-is-verdict-authoring): a converged ambient install
# elides ONLY with a reached vouch. Vouches install (the establish verb, elidable); declines
# purge (a KILL, never elides) + unknown verbs via `*) return 2` (rul-rc-partition: >=2 => run).
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
