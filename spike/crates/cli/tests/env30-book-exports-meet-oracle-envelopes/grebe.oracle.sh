#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
grebe__predict() {
   verb=$1; shift
   ent : test.dorc.Widget = "$1"
   if [ "${2-}" = "" ]; then
      case $verb in
         ensure) grebecheck -q -- "$ent" : test.dorc.Widget:"$ent"@converged ;;
      esac
   fi
}

grebe__is_converged() {
   [ "$1" = ensure ] || return 2
   grebe status -- "$2"
}
