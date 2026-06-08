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

# The ap-2 syntax-checker: prefer strict-POSIX `dash -n`, else `sh -n`. We
# SYNTAX-CHECK the rendered artifacts (`-n`: read-and-parse, never execute), so the
# fixtures' real-looking commands (`apt-get`, `systemctl`) never run. This is the
# load-bearing gate (charter ap-2 / an-render-runnable): spike-1 shipped a
# non-runnable `if true; then #…; fi` (empty then-clause) GREEN because the harness
# only string-diffed. A text golden is structurally blind to non-runnable output; a
# `-n` gate catches it. The text diff stays as a SECONDARY check (it catches
# wrong-elision *content*, to which `-n` is blind — a render that comments out
# everything is `-n`-clean and useless). Both, per cli/CLAUDE.md.
checker=
for c in dash sh; do
  if command -v "$c" >/dev/null 2>&1; then checker=$c; break; fi
done
if [ -z "$checker" ]; then
  echo "no POSIX shell (dash/sh) for the ap-2 syntax gate — cannot validate runnability" >&2
  exit 2
fi

# Syntax-check one artifact ($2) labelled ($1 = "probe"/"apply") for case ($3).
# Returns non-zero and prints the shell's diagnostic if the artifact does not parse.
syntax_check() {
  _label=$1; _art=$2; _case=$3
  if ! _err=$(printf '%s\n' "$_art" | "$checker" -n 2>&1); then
    echo "FAIL  $_case  [ap-2: rendered $_label is not $checker -n clean]"
    printf '      %s\n' "$_err"
    return 1
  fi
  return 0
}

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

  # stderr (diagnostics — ⊤-rejects, oracle warnings) is not part of the e2e
  # assertion; the artifact is stdout (probe + apply). Suppress it for a clean run.
  got=$("$dorc" --book="${dir}book.sh" "$@" < "${dir}probe-results.txt" 2>/dev/null | sed 's/\r$//')

  # Split stdout into the two emitted artifacts on their `#!/bin/sh` shebangs: the
  # FIRST block is the read-only probe, the SECOND is the eliding apply. Both must be
  # `-n`-clean. (awk c==1 / c>=2 partitions on the shebang count.)
  probe_art=$(printf '%s\n' "$got" | awk 'BEGIN{c=0} /^#!\/bin\/sh/{c++} c==1{print}')
  apply_art=$(printf '%s\n' "$got" | awk 'BEGIN{c=0} /^#!\/bin\/sh/{c++} c>=2{print}')

  # The ap-2 runnability gate — ALWAYS, and BEFORE bless (blessing a non-runnable
  # artifact is exactly the ap-2 trap). A gate failure fails the case regardless of
  # the text diff / bless mode.
  gate_ok=1
  syntax_check probe "$probe_art" "$name" || gate_ok=0
  syntax_check apply "$apply_art" "$name" || gate_ok=0
  if [ "$gate_ok" -ne 1 ]; then
    fails=$((fails + 1))
    continue
  fi

  if [ "${BLESS:-}" = "1" ]; then
    printf '%s\n' "$got" > "${dir}expected.out"
    echo "blessed $name (ap-2 gate passed)"
    continue
  fi

  want=$(sed 's/\r$//' < "${dir}expected.out")
  if [ "$got" = "$want" ]; then
    echo "ok    $name"
  else
    echo "FAIL  $name  [content diff]"
    fails=$((fails + 1))
    if command -v diff >/dev/null 2>&1; then
      printf '%s\n' "$got" | diff -u "${dir}expected.out" - || true
    fi
  fi
done

echo "---"
if [ "$fails" -ne 0 ]; then
  echo "$fails/$total e2e round-trips FAILED" >&2
  exit 1
elif [ "${BLESS:-}" = "1" ]; then
  echo "blessed $total cases (all ap-2 gates passed)"
else
  echo "all $total e2e round-trips passed (incl. ap-2 $checker -n gate)"
fi
