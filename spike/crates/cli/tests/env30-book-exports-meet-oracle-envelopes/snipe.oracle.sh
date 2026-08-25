#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
snipe__predict() {
   verb=$1; shift
   ent : test.dorc.Widget = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         ensure) snipecheck -q -- "$ent" : test.dorc.Widget:"$ent"@converged ;;
      esac
   fi
}

snipe__is_converged() {
   [ "$1" = ensure ] || return 2
   snipe status -- "$2"
}
