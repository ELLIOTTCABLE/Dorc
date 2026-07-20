#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package-status QUERY oracle: `dpkg -s <pkg>` READS installed-status — a read-only Query of
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"@installed
}
