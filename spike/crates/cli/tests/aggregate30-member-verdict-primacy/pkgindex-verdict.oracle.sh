#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
apt_get__is_converged() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      install) aptcheck -q -- "$1" ;;
      *) return 2 ;;
   esac
}
