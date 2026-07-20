#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
# strawman24-reach-crossauthor (24G Stage 5 Part B — the reaches() cross-author flagship). An

hork__predict() {
   verb=$1; shift
   pkg : sm.dorc.Package = "$1"
   case $verb in tune) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"@tuned ;; esac
}
hork__disturbs() {
   verb=$1; shift
   case $verb in tune) printf '%s\n' "$1" : disturbs sm.dorc.Package ;; esac
}

sm_dorc_Package__disturbance_reaches_only() {
   dpkg -L "$1"    : disturbs sm.dorc.File
}

installfile__predict() {
   f : sm.dorc.File = "$1"
   stat -- "$1" >/dev/null 2>&1 : sm.dorc.File:"$1"@present
}
installfile__is_converged() {
   stat -- "$1" >/dev/null 2>&1
}
installfile__disturbs() {
   printf '%s\n' "$1" : disturbs sm.dorc.File
}
