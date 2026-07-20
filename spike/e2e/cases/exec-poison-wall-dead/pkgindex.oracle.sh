#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# package-index freshness oracle (kind `pkgindex`), a nullary-verb Singleton: `apt-get
apt_get__predict() {
   verb=$1; shift
   case $verb in
      update) test -n fresh : sm.dorc.PkgIndex@fresh ;;
   esac
}
