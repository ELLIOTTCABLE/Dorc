#!/bin/sh
# coverage.sh — a thin wrapper over `dorc-coverage` (round-21 arch-6).
#
# "Runs in the gate set without becoming a gate" (charter): this prints the
# analyzer-coverage rollup over the e2e corpus (or a book+oracles you pass). It is
# NOT wired into e2e/run.sh and NEVER fails a build — it is an INSTRUMENT, read by a
# human, not an assertion. Run it by hand:
#
#     sh tools/coverage.sh                 # rollup over every e2e case
#     sh tools/coverage.sh --full          # ... with the per-site table per case
#     sh tools/coverage.sh <book.sh> <oracle.sh>...   # a one-off book+oracles
#     DORC_COVERAGE=/path/to/bin sh tools/coverage.sh  # override the binary
#
# Determinism: it shells out to the pure `dorc-coverage`; cases run in glob order.
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
spike=$(CDPATH= cd -- "$here/.." && pwd)

# Locate the built binary (or take $DORC_COVERAGE), mirroring e2e/run.sh's scheme.
bin=${DORC_COVERAGE:-}
if [ -z "$bin" ]; then
   for cand in \
      "$spike/target/debug/dorc-coverage" "$spike/target/debug/dorc-coverage.exe" \
      "$spike/target/release/dorc-coverage" "$spike/target/release/dorc-coverage.exe"; do
      if [ -x "$cand" ]; then bin=$cand; break; fi
   done
fi
if [ -z "$bin" ] || [ ! -x "$bin" ]; then
   echo "dorc-coverage not found — build it first:  cargo build -p dorc-coverage  (from spike/)" >&2
   echo "(or pass DORC_COVERAGE=/path/to/dorc-coverage)" >&2
   exit 0   # NEVER fail a build — this is an instrument, not a gate
fi

full=0
case "${1:-}" in
   --full) full=1; shift ;;
esac

# One-off mode: `coverage.sh <book.sh> <oracle.sh>...` (a book + its oracles).
if [ "$#" -gt 0 ]; then
   book=$1; shift
   set -- # reset positional to collect -o flags
   oargs=""
   for o in "$@"; do oargs="$oargs -o $o"; done
   # shellcheck disable=SC2086
   "$bin" --book="$book" $oargs
   exit 0
fi

# Corpus mode: per e2e case, print the rollup (the table too, with --full). Each case
# is a dir under e2e/cases/ with book.sh + *.oracle.sh + probe-results.txt.
cases_dir="$spike/e2e/cases"
if [ ! -d "$cases_dir" ]; then
   echo "no e2e/cases dir at $cases_dir" >&2
   exit 0
fi

for dir in "$cases_dir"/*/; do
   [ -d "$dir" ] || continue
   case=$(basename "$dir")
   [ -f "${dir}book.sh" ] || continue
   oargs=""
   for o in "$dir"*.oracle.sh; do
      [ -f "$o" ] || continue
      oargs="$oargs -o $o"
   done
   probe=""
   if [ -f "${dir}probe-results.txt" ]; then
      probe="--probe-results=${dir}probe-results.txt"
   fi
   echo "######## $case ########"
   # --no-table by default (rollup only); --full shows the per-site table too.
   table_flag="--no-table"
   [ "$full" -eq 1 ] && table_flag=""
   # shellcheck disable=SC2086
   "$bin" --book="${dir}book.sh" $oargs $probe $table_flag || true
   echo ""
done
