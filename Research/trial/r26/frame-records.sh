#!/bin/sh
# Frame a hand-authored records file into the `dorc-records/1` stream that `dorc plan` admits.
#
# WHY THIS EXISTS: the bare `site N effect=… rc=…` grammar documented in `dorc --help` is NOT
# accepted by `dorc plan` on its own — every record must carry the run nonce and the terminal
# token, under a header whose `book=` digest matches the analysed book, closed by an end sentinel.
# The e2e harness does this framing internally (`cli/tests/e2e.rs`, `framed_results`), which is why
# the committed `probe-results.txt` fixtures look feedable but are not. This is a port of that
# transform, so the hermetic plan runs in `renders/` can be reproduced and re-run by hand.
#
# THIS SCRIPT EXECUTES NO BOOK AND NO ORACLE BYTES. It runs `dorc probe`, which only *renders* the
# probe artifact to stdout, and then does text substitution. It never runs the artifact.
#
# usage: DORC=/path/to/dorc ./frame-records.sh <book.sh> <oracle-dir> <authored-records>
# stdout: the framed stream, suitable for `dorc plan --results -` or a redirect.

set -eu

if [ "$#" -ne 3 ]; then
   printf 'usage: %s <book.sh> <oracle-dir> <authored-records>\n' "$0" >&2
   exit 2
fi

book=$1
oracles=$2
authored=$3
dorc=${DORC:-dorc}
token='@@dorc@@'
nonce=dorc

probe=$("$dorc" probe --book="$book" --oracle-dir "$oracles")

# The header the probe artifact would print, lifted verbatim out of its own printf.
header=$(printf '%s\n' "$probe" | sed -n "s/^printf '\(dorc-records\/1 [^']*\)\\\\n'.*/\1/p" | head -n 1)
if [ "$header" = "" ]; then
   printf 'frame-records: no dorc-records/1 header in the probe artifact\n' >&2
   exit 1
fi

# Every site the probe self-reports, in artifact order.
sites=$(printf '%s\n' "$probe" | sed -n "s/.*printf 'dorc site \([0-9][0-9.]*\) .*/\1/p")

printf '%s\n' "$header"

# Authored records, in authored order, for sites the probe actually reports.
printf '%s\n' "$sites" | while read -r site; do
   if [ "$site" = "" ]; then continue; fi
   line=$(sed -n "s/^site $site \(.*\)$/site $site \1/p" "$authored" | head -n 1)
   if [ "$line" = "" ]; then
      line="site $site effect=cant-tell rc=0"
   fi
   printf '%s %s %s\n' "$nonce" "$line" "$token"
done

printf 'dorc-records-end/1 nonce=%s %s\n' "$nonce" "$token"
