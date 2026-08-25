#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
wombat__predict() {
   verb=$1; shift
   ent : test.dorc.Widget = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         ensure) wombatcheck -q -- "$ent" : test.dorc.Widget:"$ent"@converged ;;
      esac
   fi
}

wombat__is_converged() {
   [ "$1" = ensure ] || return 2
   env -i PATH=/usr/bin:/bin wombat status -- "$2"
}
