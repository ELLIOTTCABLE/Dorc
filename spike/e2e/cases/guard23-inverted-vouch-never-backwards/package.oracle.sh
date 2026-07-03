# package oracle (apt/dpkg) — the guard23 INVERTED-VERDICT fixture (23J conv-rc-soundness
# facet (i)). The predict() models purge as the `!` (inverted) effect. The VOUCH is the
# authored verdict function `apt-get.is_diverged()` below (rul24-vouch-is-verdict-authoring,
# 24A §1c) — sense DECLARED BY NAME (rul-role-split), NOT a tilde mark (retired). It is INERT
# at HEAD (the lift keys only on `.predict`), so it is documentary plain-sh here; Stage 3
# consumes it. This is the corpus's first verdict-function-carrying fixture.
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

# THE VOUCH (inert at HEAD; Stage 3 consumes it). INVERTED sense declared by name
# (rul-role-split); rc-partition: 0 = the named sense (DIVERGED) holds, 1 = its complement
# (converged), >=2 = confused ⇒ run. For a purge, exit-0-means-present, and present means the
# purge is DIVERGED (still needs to run) — so this reads the SAME dpkg-query the predict does,
# inverted. The engine's ONLY legal glue is the sense-flip
# `( apt_get__is_diverged args; [ $? -eq 1 ] ) || apt-get purge …`: when diverged (rc 0),
# `[ $? -eq 1 ]` is false ⇒ the group yields 1 ⇒ `||` RUNS the purge. A NAIVE
# `( apt_get__is_diverged args ) || purge` would short-circuit on rc 0 and SKIP the purge
# precisely when it is needed — the backwards guard this fixture forbids.
apt-get.is_diverged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      purge) dpkg-query -W "$1" >/dev/null 2>&1 ;;
      *) return 2 ;;
   esac
}
