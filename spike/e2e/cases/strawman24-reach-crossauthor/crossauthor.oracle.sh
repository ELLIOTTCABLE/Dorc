#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# strawman24-reach-crossauthor (24G Stage 5 Part B — the reaches() cross-author flagship). An
# UNRELATED tool `hork` fiddles the nginx package: its touches() honestly emits `package:nginx` — it
# knows NOTHING of files. A downstream `installfile` (a DIFFERENT author's file tool) converges a
# file-fact `file:/etc/nginx/nginx.conf`, PAST the running hork wall. Token-equality calls
# `package:nginx` and `file:/etc/nginx/nginx.conf` disjoint (different kinds), so WITHOUT reaches the
# converged file-fact WRONGLY SURVIVES the running wall (a silent under-execute — hork's package touch
# really drags the package's files). The `package` kind's OWNER ships `package.reaches()` (a DYNAMIC
# arm — `dpkg -L`, escalated to the probe): touching `package:nginx` REACHES its files. The engine
# expands hork's `package:nginx` footprint through it (whoever emitted the coord — the cross-author
# point), so the expanded footprint HITs `file:/etc/nginx/nginx.conf` ⇒ the file-fact correctly
# DEMOTES to run. Pre-Part-B it elided (wrong-survival); the differential: expected.ran runs BOTH.

# hork — the unrelated package-fiddler (predict establishes a package cell, touches emits package:X).
# No is_converged ⇒ hork never elides ⇒ it is a RUNNING wall (footprint package:nginx).
hork__predict() {
   verb=$1; shift
   pkg : sm.dorc.Package = "$1"
   case $verb in tune) dpkg-query -W "$pkg" >/dev/null 2>&1 : sm.dorc.Package:"$pkg"#tuned ;; esac
}
hork__disturbs() {
   verb=$1; shift
   case $verb in tune) printf '%s\n' "$1" : sm.dorc.Package ;; esac
}

# THE REACH FUNCTION (24G §4): the package kind's OWNER declares what touching a package DRAGS. A
# DYNAMIC arm (`dpkg -L` — the static tracer ⊤s, so it escalates: ships strip-only, runs read-only at
# probe, its stdout the reached files). The KIND rides the trailing annotation (`: sm.dorc.File`); the stdout
# lines are RAW ENTITIES (no `kind:` prefix). One capture unit per arm.
sm_dorc_Package__disturbance_reaches_only() {
   dpkg -L "$1"    : sm.dorc.File
}

# installfile — a DIFFERENT author's file tool. Converges a file-fact; vouches it (is_converged).
# It knows NOTHING of packages; only package.reaches() bridges package:nginx -> its files.
installfile__predict() {
   f : sm.dorc.File = "$1"
   stat -- "$1" >/dev/null 2>&1 : sm.dorc.File:"$1"#present
}
installfile__is_converged() {
   stat -- "$1" >/dev/null 2>&1
}
installfile__disturbs() {
   printf '%s\n' "$1" : sm.dorc.File
}
