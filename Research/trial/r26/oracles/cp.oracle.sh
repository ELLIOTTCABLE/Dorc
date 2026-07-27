#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# cp — the two-operand file drop only: `cp SRC DST`, converged when DST already holds SRC's bytes.
# Delegated to `cmp -s`, whose exit vocabulary already is ours (0 same, 1 differ, 2+ trouble), so
# there is nothing to translate.
#
# The judgment behind the yes: plain `cp` (no -p, no -a) leaves an existing DST's mode and owner
# alone, so once the bytes match, all that skipping costs is a fresh mtime. That is acceptable for
# a config drop. It would not be acceptable for `cp -p`, for a copy into a directory, or for
# anything recursive — each of which declines below.
#
# Kind: r26.smoke.File — throwaway, minted for this round only.

cp__is_converged() {
   command -v cmp >/dev/null 2>&1 || return 2
   [ "$#" -eq 2 ] || return 2
   case "${1-}" in -*) return 2 ;; esac
   case "${2-}" in -*) return 2 ;; esac
   [ -f "$1" ] || return 2
   dst : r26.smoke.File = "$2"
   if [ -d "$dst" ]; then
      # `cp SRC DIR` establishes DIR/basename(SRC) — a different cell than the one below, and one
      # this file has not modelled.
      printf 'decline unmodeled %s: copy-into-directory is a different question\n' "$dst" \
         >>"${DREP_V1:-/dev/null}"
      return 2
   fi
   [ -e "$dst" ] || return 1
   cmp -s -- "$1" "$dst"   : r26.smoke.File:"$dst"@content
}
