#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# package-status QUERY oracle (the DESIGN `dpkg -s nginx || apt-get install` idiom).
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"@installed
}
