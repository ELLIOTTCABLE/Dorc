#!/bin/sh
# Mechanized end-to-end round-trip for the dorc CLI — IN sh, FROM sh (no Rust harness).
#
# For each cases/<name>/ : feed the (simulated) probe-results on stdin to
#     dorc --book=book.sh -o <each *.oracle.sh>
# and diff its stdout (the probe script, then the eliding-apply) against expected.out.
# This exercises the WHOLE pipeline on actual shell-script files: source → analyze →
# compile-probe → (simulate the host on stdin) → eliding-apply.
#
# Usage:  sh run.sh                 # auto-locates target/{debug,release}/dorc[.exe]
#         DORC=/path/to/dorc sh run.sh
#         BLESS=1 sh run.sh         # regenerate every expected.out from current output
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

# Locate the built binary (or take $DORC).
dorc=${DORC:-}
if [ -z "$dorc" ]; then
  for cand in \
    "$here/../target/debug/dorc" "$here/../target/debug/dorc.exe" \
    "$here/../target/release/dorc" "$here/../target/release/dorc.exe"; do
    if [ -x "$cand" ]; then dorc=$cand; break; fi
  done
fi
if [ -z "$dorc" ] || [ ! -x "$dorc" ]; then
  echo "dorc binary not found — build it first:  mise exec -- cargo build -p dorc-cli" >&2
  echo "(or pass DORC=/path/to/dorc)" >&2
  exit 2
fi

fails=0
total=0
for dir in "$here"/cases/*/; do
  name=$(basename "$dir")
  total=$((total + 1))

  # Collect `-o <oracle>` args (sorted by the shell glob ⇒ deterministic).
  set --
  for o in "$dir"*.oracle.sh; do
    [ -e "$o" ] || continue
    set -- "$@" -o "$o"
  done

  got=$("$dorc" --book="${dir}book.sh" "$@" < "${dir}probe-results.txt" | sed 's/\r$//')

  if [ "${BLESS:-}" = "1" ]; then
    printf '%s\n' "$got" > "${dir}expected.out"
    echo "blessed $name"
    continue
  fi

  want=$(sed 's/\r$//' < "${dir}expected.out")
  if [ "$got" = "$want" ]; then
    echo "ok    $name"
  else
    echo "FAIL  $name"
    fails=$((fails + 1))
    if command -v diff >/dev/null 2>&1; then
      printf '%s\n' "$got" | diff -u "${dir}expected.out" - || true
    fi
  fi
done

echo "---"
if [ "${BLESS:-}" = "1" ]; then
  echo "blessed $total cases"
elif [ "$fails" -eq 0 ]; then
  echo "all $total e2e round-trips passed"
else
  echo "$fails/$total e2e round-trips FAILED" >&2
  exit 1
fi
