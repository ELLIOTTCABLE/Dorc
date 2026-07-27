#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# cp — the two-operand file drop only: `cp SRC DST`, converged when DST already holds SRC's bytes.
# Delegated to `cmp -s`, whose exit vocabulary already is ours (0 same, 1 differ, 2+ trouble), so
# there is nothing to translate.
#
# The judgment behind the yes: plain `cp` (no -p, no -a) leaves an existing DST's mode and owner
# alone, so once the bytes match, all that skipping costs is a fresh mtime. That is acceptable for
# a config drop. It would not be acceptable for `cp -p` or anything recursive — both of which
# decline on the leading-dash gate.
#
# Three shapes decline without a written gate, because cmp itself exits 2 on each and 2 already
# means cannot-say: a missing source, a missing destination, and `cp SRC DIR` (which establishes
# DIR/basename(SRC), a different cell than this file models). Writing `[ -e ]` tests for them would
# be strictly worse — a file test in a condition silently voids every mark here (../README.md §4).
#
# Kind: r26.smoke.File — throwaway, minted for this round only.

cp__is_converged() {
   if [ "${3-}" != "" ]; then return 2; fi
   if [ "${2-}" = "" ]; then return 2; fi
   if [ "${1#-}" != "$1" ]; then return 2; fi
   if [ "${2#-}" != "$2" ]; then return 2; fi
   dst : r26.smoke.File = "$2"
   cmp -s -- "$1" "$dst"   : r26.smoke.File:"$dst"@content
}
