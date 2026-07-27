#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# cp — the two-operand file drop only: `cp SRC DST`, converged when DST already holds SRC's bytes.
# Delegated to `cmp -s`, whose exit vocabulary already is ours (0 same, 1 differ, 2+ trouble), so
# there is nothing to translate.
#
# The judgment behind the yes: plain `cp` (no -p, no -a) leaves an existing DST's mode and owner
# alone, so once the bytes match, all that skipping costs is a fresh mtime. That is acceptable for
# a config drop. It would not be acceptable for `cp -p` or for anything recursive — both of which
# decline on the leading-dash gate. `cp SRC DIR` establishes DIR/basename(SRC), a different cell
# than this file models; it declines on its own, since cmp cannot read a directory and exits 2.
#
# NB: `test`, not `[` — a bracket test silently voids every mark in this file (see ../README.md).
#
# Kind: r26.smoke.File — throwaway, minted for this round only.

cp__is_converged() {
   command -v cmp >/dev/null 2>&1 || return 2
   test "$#" -eq 2 || return 2
   test "${1#-}" = "$1" || return 2
   test "${2#-}" = "$2" || return 2
   test -f "$1" || return 2
   dst : r26.smoke.File = "$2"
   test -e "$dst" || return 1
   cmp -s -- "$1" "$dst"   : r26.smoke.File:"$dst"@content
}
