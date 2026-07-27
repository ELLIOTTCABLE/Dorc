#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# dpkg — the QUERY side only, so dorc can lift the admin's `dpkg -s x || apt-get install -y x`
# hand-guard. Coverage: `dpkg -s <one-package>`, delegated to the real (read-only) dpkg, which
# claims the exit status faithfully and declines both output channels (the guard reads neither).
#
# Declines everything else: any other action, any flag, any multi-operand shape, and any operand
# that could be read as an option. dpkg's own support for a `--` terminator is not something this
# file wants to bet a live run on, so a leading-dash operand is refused rather than escaped.
#
# There is deliberately no `dpkg__is_converged`: `dpkg -s` is a read whose exit status the book
# consumes, so there is nothing here it would ever be safe to skip.
#
# NB: `test`, not `[` — a bracket test silently voids every mark in this file (see ../README.md).
#
# Kind: r26.smoke.PkgState — throwaway, minted for this round only, NOT the stdlib's sm.dorc.*.

dpkg__predict() {
   command -v dpkg >/dev/null 2>&1 || return 2
   case "${1-}" in
   -s|--status) ;;
   *) return 2 ;;
   esac
   test "${2-}" != "" || return 2
   test "${3-}" = "" || return 2
   test "${2#-}" = "$2" || return 2
   pkg : r26.smoke.PkgState = "$2"
   dpkg -s "$pkg" >/dev/null 2>&1   :? r26.smoke.PkgState:"$pkg"@installed
}
