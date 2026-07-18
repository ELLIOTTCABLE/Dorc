#!/bin/sh
# ── oracle: package (Debian dpkg/apt) ──────────────  [STRAWMAN v2, fact-centric]
# Lifted statically by Dorc (never run as Dorc-code). Plain sh: an assignment, a
# function, and a few marker calls — dash-clean, inert if Dorc vanishes (modulo a
# 2-line `oracle_effect() { :; }` shim). See Research/notes/162 §2.

oracle_kind=package                         # the named kind this file serves

# READ-ONLY fact-probe: does `package:$1 = installed` hold?  Three outcomes:
#   exit 0 = holds (converged) · 1 = absent (diverged) · 2 = can't-tell (unknown)
# NB: captures the tool's own status (a missing dpkg-query => 2), NOT a pipe into
# grep — that idiom can't tell "absent" from "the check failed" (note 162 F-1).
oracle_probe_package() {
   command -v dpkg-query >/dev/null 2>&1 || return 2
   st=$(dpkg-query -W -f='${Status}' "$1" 2>/dev/null) || return 1
   case $st in
      'install ok installed') return 0 ;;
      *) return 1 ;;
   esac
}

# EFFECTS: accumulating (provider, verb) -> (polarity, selector) on `package`. Many
# verbs and providers coexist without clobbering (unlike a single oracle_verb). The
# 4th token is the per-entity selector cell (note 193 §4): install/purge gate the
# same #installed cell; a hypothetical `apt-get upgrade` would gate a #version cell.
oracle_effect apt-get install   establish installed
oracle_effect apt-get reinstall establish installed
oracle_effect apt-get purge     kill      installed
oracle_effect apt-get remove    kill      installed
oracle_effect dpkg    -i        establish installed
