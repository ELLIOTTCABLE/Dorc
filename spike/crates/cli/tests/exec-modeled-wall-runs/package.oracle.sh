#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# minimal apt-get oracle, lifted statically by dorc. ONE body per provider: `apt-get` spans two
# kinds -- the package, and the package INDEX -- and one function describes both, because marks
# are per-line and nothing ties a body to a single kind (28K section 7).
#
# Binds are ARM-LOCAL, deliberately. A shared bind above the case would put an ambient kind on
# every arm's path, including `update`'s, whose coordinate is an entity-less singleton needing no
# bind at all -- and resolving that bind's value against a nullary verb's empty argv tops the
# whole check. Keep new arms in this shape.
apt_get__predict() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   case $verb in
      update) test -n fresh : sm.dorc.PkgIndex@fresh ;;
      install)
         pkg : sm.dorc.Package = "$1"
         if [ "${2-}" = "" ]; then
            dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@installed
         fi ;;
      purge)
         pkg : sm.dorc.Package = "$1"
         if [ "${2-}" = "" ]; then
            dpkg-query -W "$pkg" >/dev/null 2>&1 :! sm.dorc.Package:"$pkg"@installed
         fi ;;
   esac
}
