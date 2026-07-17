#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# package-status QUERY oracle (`dpkg -s <pkg>` reads pkgstate:<pkg>#installed; task-D2
# query polarity, mutates nothing). Present so the hand-written guard is a MODELED query —
# the already-guarded refusal must eventually recognize the guard's fact, not merely fail
# to analyze it (notes/218a hunt-C: same-fact detection is the fiddly part).
dpkg__predict() {
   case $1 in -s) shift ;; esac
   pkg : sm.dorc.PkgState = "$1"
   dpkg -s -- "$pkg" >/dev/null 2>&1 :? sm.dorc.PkgState:"$pkg"#installed
}
