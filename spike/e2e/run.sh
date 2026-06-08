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
checker_abs=
for c in dash sh; do
  if command -v "$c" >/dev/null 2>&1; then checker=$c; checker_abs=$(command -v "$c"); break; fi
done
if [ -z "$checker" ]; then
  echo "no POSIX shell (dash/sh) for the ap-2 syntax gate — cannot validate runnability" >&2
  exit 2
fi

# Syntax-check one artifact ($2) labelled ($1 = "probe"/"apply") for case ($3).
# Returns non-zero and prints the shell's diagnostic if the artifact does not parse.
# Quiet when XFAIL_ACTIVE=1 (a known-defect case's failure is expected; the `xfail`
# summary line carries the reason, so the raw diagnostic would just be noise).
syntax_check() {
  _label=$1; _art=$2; _case=$3
  if ! _err=$(printf '%s\n' "$_art" | "$checker" -n 2>&1); then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [ap-2: rendered $_label is not $checker -n clean]"
      printf '      %s\n' "$_err"
    fi
    return 1
  fi
  return 0
}

# The ap-2 EXECUTABLE acceptance (Deliverable A / an-render-executability-check):
# `-n` proves the artifact PARSES; this proves the *right lines run*. A case opts in
# by shipping a mocks/ dir + an expected.ran golden. We EXECUTE the rendered artifact
# ($2) under PATH=<case>/mocks ONLY, so the sole things that can run are the inert
# shims (each logs `ran: <argv>` and exits 0 — never a real apt-get/systemctl/ufw).
# A `:`-stubbed (elided) command logs nothing; a `Run` command logs its argv. We
# assert the sorted run-set == expected.ran. SAFETY: PATH is the mocks dir alone, and
# an un-shimmed external command ⇒ `command not found` ⇒ a loud failure (never a real
# system mutation). Deterministic: the log is sorted (`inv-determinism`).
exec_check() {
  _label=$1; _art=$2; _case=$3; _dir=$4
  _log=$(mktemp)
  # Resolve the mocks dir to an ABSOLUTE path: PATH is about to become *only* this
  # dir, so a relative path would break (and the interpreter is invoked by its own
  # absolute path `$checker_abs`, never found via the overridden PATH).
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  # PATH = the mocks dir ONLY. The shims are the entire executable surface; nothing
  # real is reachable (an un-shimmed external ⇒ `not found` ⇒ a loud failure, never a
  # real system mutation). The interpreter runs the rendered artifact exactly as a
  # host would run the shipped apply.
  if ! _run_err=$(DORC_LOG="$_log" PATH="$_mocks" "$checker_abs" 2>&1 <<EOF
$_art
EOF
  ); then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [ap-2-exec: rendered $_label errored when run under mocks]"
      printf '      %s\n' "$_run_err"
    fi
    rm -f "$_log"
    return 1
  fi
  _got_ran=$(LC_ALL=C sort < "$_log")
  rm -f "$_log"
  if [ "${BLESS:-}" = "1" ]; then
    printf '%s\n' "$_got_ran" > "${_dir}expected.ran"
    return 0
  fi
  _want_ran=$(LC_ALL=C sort < "${_dir}expected.ran" 2>/dev/null || true)
  if [ "$_got_ran" = "$_want_ran" ]; then
    return 0
  fi
  if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
    echo "FAIL  $_case  [ap-2-exec: $_label ran the wrong commands]"
    if command -v diff >/dev/null 2>&1; then
      printf '%s\n' "$_got_ran" | diff -u "${_dir}expected.ran" - 2>/dev/null || true
    fi
  fi
  return 1
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

  # A case with an XFAIL file is a documented KNOWN-DEFECT pin (notes/195): it asserts
  # the *correct* (safe) behavior and is EXPECTED to fail against the current engine,
  # so the corpus carries the defect without flipping the suite red or papering over
  # it. A surprise pass ⇒ XPASS (loud: the defect got fixed — promote the case). The
  # file's first line is the reason. Bless is suppressed for an xfail (its goldens are
  # hand-authored to the safe behavior, not blessed from buggy output).
  xfail_reason=
  XFAIL_ACTIVE=
  if [ -f "${dir}XFAIL" ]; then
    xfail_reason=$(head -n1 "${dir}XFAIL")
    XFAIL_ACTIVE=1
  fi

  # case_ok accumulates every gate + content check; interpreted through XFAIL below.
  # (Not early-`continue`d, so an xfail case that fails a gate is reported, not fatal.)
  case_ok=1

  # The ap-2 runnability gate — ALWAYS, and (for non-xfail) BEFORE bless (blessing a
  # non-runnable artifact is exactly the ap-2 trap).
  syntax_check probe "$probe_art" "$name" || case_ok=0
  syntax_check apply "$apply_art" "$name" || case_ok=0

  # The ap-2 EXECUTABLE gate (Deliverable A): a case with a mocks/ dir is RUN, not just
  # parsed — execute the rendered apply under the inert shims and assert the exact set
  # of commands that ran (elided `:`-stubs run nothing). Analysis-only cases (no
  # mocks/) keep the `-n`+golden discipline and are never executed. Skipped if the
  # syntax gate already failed (a non-parseable artifact can't be meaningfully run).
  if [ "$case_ok" -eq 1 ] && [ -d "${dir}mocks" ]; then
    exec_check apply "$apply_art" "$name" "$dir" || case_ok=0
  fi

  # Content golden-diff (secondary to the gates; -n is blind to *which* lines elided).
  # Skipped under bless and for xfail cases (goldens hand-authored there).
  if [ "$case_ok" -eq 1 ] && [ "${BLESS:-}" != "1" ] && [ -z "$xfail_reason" ]; then
    want=$(sed 's/\r$//' < "${dir}expected.out")
    if [ "$got" != "$want" ]; then
      echo "FAIL  $name  [content diff]"
      case_ok=0
      if command -v diff >/dev/null 2>&1; then
        printf '%s\n' "$got" | diff -u "${dir}expected.out" - || true
      fi
    fi
  fi

  # Interpret case_ok through the XFAIL lens.
  if [ -n "$xfail_reason" ]; then
    if [ "$case_ok" -eq 1 ]; then
      echo "XPASS $name  [known defect appears FIXED — promote this case: $xfail_reason]"
      fails=$((fails + 1))
    else
      echo "xfail $name  [$xfail_reason]"
    fi
  elif [ "${BLESS:-}" = "1" ]; then
    if [ "$case_ok" -eq 1 ]; then
      printf '%s\n' "$got" > "${dir}expected.out"
      echo "blessed $name (ap-2 gate passed)"
    else
      echo "FAIL  $name  [gate failed; not blessed]"
      fails=$((fails + 1))
    fi
  elif [ "$case_ok" -eq 1 ]; then
    echo "ok    $name"
  else
    fails=$((fails + 1))
  fi
done

echo "---"
if [ "$fails" -ne 0 ]; then
  echo "$fails/$total e2e round-trips FAILED" >&2
  exit 1
elif [ "${BLESS:-}" = "1" ]; then
  echo "blessed $total cases (all ap-2 gates passed)"
else
  echo "all $total e2e round-trips passed (incl. ap-2 $checker -n gate + exec gate where mocked)"
fi
