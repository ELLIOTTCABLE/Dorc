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
# No `command -v dpkg` existence gate, on purpose: it costs every elision in the book (see
# ../README.md §4) and buys nothing, because a missing delegate exits 127, which the rc partition
# already reads as cannot-say.
#
# Kind: r26.smoke.PkgState — throwaway, minted for this round only, NOT the stdlib's sm.dorc.*.

dpkg__predict() {
   case "${1-}" in
   -s|--status) ;;
   *) return 2 ;;
   esac
   if [ "${3-}" != "" ]; then return 2; fi
   if [ "${2-}" = "" ]; then return 2; fi
   if [ "${2#-}" != "$2" ]; then return 2; fi
   pkg : r26.smoke.PkgState = "$2"
   dpkg -s "$pkg" >/dev/null 2>&1   :? r26.smoke.PkgState:"$pkg"@installed
}
