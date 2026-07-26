#!/bin/sh
# loom.sh — `dorc-loom <verb>` over the whole aid case corpus by default.
#
#     sh tools/loom.sh compile              # every crates/aid/tests/*.loom
#     sh tools/loom.sh promote              # ... the matching publish
#     sh tools/loom.sh compile a.loom b.loom  # scoped to the named cases
#
# The tool itself demands an explicit CASE list, but a bare list of all 53 is the
# canonical invocation: `gate_touched_set` classifies which of the given cases actually
# carry prose changes, so passing everything is how you say "publish what I edited"
# rather than a blunderbuss. compile and promote MUST be given the same list — promote
# verifies its inspection against the receipt compile staged — which is the other reason
# to default both to the corpus rather than have a human keep two lists in sync.
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
spike=$(CDPATH= cd -- "$here/.." && pwd)

[ "$#" -gt 0 ] || { echo "usage: loom.sh <compile|promote|vars> [CASE...]" >&2; exit 2; }
verb=$1
shift

if [ "$#" -eq 0 ]; then
   set -- "$spike"/crates/aid/tests/*.loom
   [ -e "$1" ] || { echo "loom.sh: no .loom cases under crates/aid/tests — the collection moved" >&2; exit 2; }
fi

exec cargo run --quiet -p dorc-loom -- "$verb" "$@"
