#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package-status QUERY oracle (`dpkg -s <pkg>` reads pkgstate:<pkg>#installed; task-D2
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"#installed
}
