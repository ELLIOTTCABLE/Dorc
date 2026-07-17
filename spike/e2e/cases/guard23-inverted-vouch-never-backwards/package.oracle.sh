#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
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
   pkg : sm.dorc.Package = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         install) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"#installed ;;
         purge) dpkg-query -W "$pkg" >/dev/null 2>&1 :! sm.dorc.Package:"$pkg"#installed ;;
      esac
   fi
}

# THE VOUCH: purge's converged sense via explicit-return manual inversion
# (rul24-ditch-is-diverged — is_diverged retired). dpkg-query rc 1 (absent) = purge
# converged (return 0); 0 (present) = not converged (return 1); >=2 = confused (return 2).
# NEVER licenses backwards: a present package (rc 0) returns 1, so the guard's `||` RUNS the purge.
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      purge) dpkg-query -W "$1" >/dev/null 2>&1; case $? in 1) return 0 ;; 0) return 1 ;; *) return 2 ;; esac ;;
      *) return 2 ;;
   esac
}
