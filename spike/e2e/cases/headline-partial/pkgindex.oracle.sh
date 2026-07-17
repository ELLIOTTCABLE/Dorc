#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package-index freshness oracle (kind `pkgindex`), a nullary-verb Singleton: `apt-get
# update` resolves the Singleton cell via the empty-entity mark `: sm.dorc.PkgIndex:#fresh`.
# command-keyed predict(): `apt-get update` is a NULLARY verb (no operand) — the
# value-less annotation `idx : sm.dorc.PkgIndex` resolves the Singleton cell (task-W §4).
# The effect mark `: sm.dorc.PkgIndex:#fresh` carries the Singleton's EMPTY entity slot
# (jc-singleton-mark): kind `pkgindex`, the-one entity, selector `fresh`.
apt_get__predict() {
   verb=$1; shift
   case $verb in
      update) idx : sm.dorc.PkgIndex; test -n fresh : sm.dorc.PkgIndex:#fresh ;;
   esac
}
