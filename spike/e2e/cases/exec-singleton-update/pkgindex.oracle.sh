#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package-index freshness oracle (kind `pkgindex`), a nullary-verb Singleton: `apt-get
apt_get__predict() {
   verb=$1; shift
   case $verb in
      update) idx : sm.dorc.PkgIndex; test -n fresh : sm.dorc.PkgIndex:#fresh ;;
   esac
}

apt_get__is_converged() {
   verb=$1; shift
   case $verb in
      update) test -n fresh ;;
      *) return 2 ;;
   esac
}
