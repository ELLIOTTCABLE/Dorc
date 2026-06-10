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
  echo "dorc binary not found — build it first:  cargo build  (from inside spike/)" >&2
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

# gate-2 (20B §3): the redirection sandbox needs awk for the pre-exec scan. awk is
# already a load-bearing dependency (the shebang-split partition above); require it
# explicitly so a missing awk is a clear failure, not a silently-skipped safety gate.
if ! command -v awk >/dev/null 2>&1; then
  echo "no awk for the gate-2 redirection-sandbox scan — cannot validate exec safety" >&2
  exit 2
fi
redir_scan="$here/scan_redirects.awk"
if [ ! -f "$redir_scan" ]; then
  echo "gate-2 scanner missing: $redir_scan" >&2
  exit 2
fi
# gate-1 parity rc-normalizer (item-4 / tc-probe-parity-projection): strips rc= from a
# record only when the authored fixture omitted it for that site (so an rc-bearing site
# is compared WITH its rc).
parity_norm="$here/norm_parity.awk"
if [ ! -f "$parity_norm" ]; then
  echo "gate-1 parity normalizer missing: $parity_norm" >&2
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

# gate-2 (20B §3): scan an artifact ($2, labelled $1 for case $3) for an unsafe
# redirection BEFORE it is executed. PATH-isolation governs which COMMANDS run, but not
# where their `>`/`>>` redirections write — `somecmd >/abs/path` in an executed artifact
# hits the real fs. We run every exec_check with cwd = a throwaway sandbox (below), so a
# bare relative target is disposable; this scan refuses the targets that escape it
# (absolute, dynamic `$`/backtick, or `..`-climbing), allowlisting `/dev/null`. The
# scanner is a conservative lexical pass over our OWN renders (scan_redirects.awk); an
# over-refusal prints the offending line (legible, not silent). Returns non-zero on a
# refusal.
scan_redirect_safety() {
  _label=$1; _art=$2; _case=$3
  _bad=$(printf '%s\n' "$_art" | awk -f "$redir_scan") || true
  if [ -n "$_bad" ]; then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [gate-2: rendered $_label has an unsafe redirect target (absolute/dynamic/escaping) — refused before exec]"
      printf '      %s\n' "$_bad"
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
# A `:`-stubbed (elided) command logs nothing; a `Run` command logs its argv.
#
# gate-2 sandbox (20B §3): execution runs in a subshell whose cwd is a FRESH mktemp dir
# (`_sand`), so any bare relative redirect the artifact performs lands in disposable
# space, never the repo (today they would land in run.sh's cwd). The redirect scan
# (`scan_redirect_safety`) has already refused absolute/dynamic/escaping targets, so the
# sandbox + scan together bound where an executed render may write. The interpreter is
# invoked by its own absolute path (`$checker_abs`, never found via the overridden PATH),
# and DORC_LOG is an absolute path (resolved before the cd) so the shims log into it from
# inside the sandbox.
#
# gate-4 (20B §2): the run-set is compared UNSORTED — execution order is deterministic
# under sequential sh, and sorting would discard the welded book-order assertion ("the
# book's order is sacred"). SAFETY: PATH is the mocks dir alone; an un-shimmed external
# command ⇒ `command not found` ⇒ a loud failure (never a real system mutation).
exec_check() {
  _label=$1; _art=$2; _case=$3; _dir=$4
  scan_redirect_safety "$_label" "$_art" "$_case" || return 1
  _log=$(mktemp)
  _sand=$(mktemp -d)
  # Absolute mocks dir: PATH is about to become *only* this dir, so a relative path
  # would break.
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  # Execute in a subshell cd'd into the sandbox (gate-2): a bare relative redirect lands
  # under $_sand, not the repo. $_log + $checker_abs are absolute, unaffected by the cd.
  if ! _run_err=$( cd -- "$_sand" && DORC_LOG="$_log" PATH="$_mocks" "$checker_abs" 2>&1 <<EOF
$_art
EOF
  ); then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [ap-2-exec: rendered $_label errored when run under mocks]"
      printf '      %s\n' "$_run_err"
    fi
    rm -rf "$_sand"; rm -f "$_log"
    return 1
  fi
  rm -rf "$_sand"
  _got_ran=$(cat "$_log")
  rm -f "$_log"
  if [ "${BLESS:-}" = "1" ]; then
    printf '%s\n' "$_got_ran" > "${_dir}expected.ran"
    return 0
  fi
  # A mocks/ case without its expected.ran is an authoring error, not an
  # asserted-all-elide (the old empty-want fallback made the two indistinguishable —
  # round-20 harness-crosscheck find-9).
  if [ ! -f "${_dir}expected.ran" ]; then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [ap-2-exec: mocks/ present but expected.ran missing — author or bless it]"
    fi
    return 1
  fi
  # gate-4: ordered compare (no sort) — the log is in execution order, the golden in
  # book order; a reorder is a real regression, not noise. EXCEPTION (tc-pipe-ran-order,
  # 20J strain-P5 / observed ~1-in-15 in 20M): pipeline STAGES run concurrently in sh,
  # so their log-line order is genuinely nondeterministic — a case whose book pipes
  # leaves opts into order-insensitive comparison via a `RAN_ORDER=lax` marker file.
  # Scoped per-case; the welded book-ORDER assertion stays the default everywhere else.
  _want_ran=$(cat "${_dir}expected.ran" 2>/dev/null || true)
  if [ -f "${_dir}RAN_ORDER=lax" ]; then
    _got_ran=$(printf '%s\n' "$_got_ran" | LC_ALL=C sort)
    _want_ran=$(printf '%s\n' "$_want_ran" | LC_ALL=C sort)
  fi
  if [ "$_got_ran" = "$_want_ran" ]; then
    return 0
  fi
  if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
    echo "FAIL  $_case  [ap-2-exec: $_label ran the wrong commands or wrong order]"
    if command -v diff >/dev/null 2>&1; then
      printf '%s\n' "$_got_ran" | diff -u "${_dir}expected.ran" - 2>/dev/null || true
    fi
  fi
  return 1
}

# gate-1 (rule-probe-exec-gate, 205 §1 — the load-bearing one): EXECUTE the rendered
# PROBE ($2) under the same inert-shim discipline as the apply gate (PATH=<case>/mocks,
# sandbox cwd, DORC_LOG set), and assert three things on the records it emits on stdout:
#
#   (a) SITE-COMPLETENESS + GRAMMAR (always): every resolvable site (a `printf 'site N …`
#       emitter in the probe) emits EXACTLY ONE record, and every record is grammar-valid
#       (`site <int> effect=<holds|absent|cant-tell> rc=<int>`). A deleted/garbled record
#       ⇒ loud fail. This is structural and does not depend on WHICH effect-word.
#
#   (c) VOUCH-CLOSURE / no-127 (unless PROBE_RESULTS=authored): no record carries rc=127
#       (command-not-found). Under PATH=mocks-only, an rc=127 means the probe invoked a
#       command with no shim — the executable half of vouch-closure failing loud. (NB:
#       an un-shimmed probe command does NOT abort the probe — the `__check` wrappers
#       swallow the not-found via their own `2>/dev/null`, so the only signal is rc=127
#       in the record; we detect it explicitly rather than rely on a non-zero exit.)
#
#   (b) PARITY (unless PROBE_RESULTS=authored): the records the mocked probe PRODUCES must
#       match the case's hand-authored `probe-results.txt` records (the fixture the apply
#       gate consumes). PER-SITE rc-tightening (item-4 / tc-probe-parity-projection): a
#       site whose AUTHORED record carries an `rc=` is compared WITH its rc (the fold-valid
#       Query/pkgstate rc — a wrong probe-emitted rc would be a wrong fold, 20E §2); a site
#       whose fixture omits rc keeps the effect-only compare (an establish site's rc is the
#       probe-command's, firewalled from the fold, so it is not a parity target — and the
#       fixtures historically omit it, so this needs no mass re-authoring). `norm_parity.awk`
#       strips rc from a record iff its site's authored record had none, applied to both
#       sides. A case whose fixture intentionally diverges from what the mocks can reproduce
#       opts out with a one-line `PROBE_RESULTS=authored` marker file.
#
# The PROBE_RESULTS=authored opt-out governs (b)+(c) ONLY — (a) always holds. The opt-out
# is the HONEST residual of the convergence axis: today most mocks/ dirs carry only the
# APPLY commands (apt-get …), not the PROBE commands (dpkg-query/getent/ufw/systemctl), so
# their probe cannot be faithfully mock-executed until D3b ships probe-specific shims.
# Authoring those shims is explicitly out of D3a scope; the opt-out records which cases
# need them rather than silently re-blessing fixtures to match all-exit-0 mock output.
probe_exec_check() {
  _art=$1; _case=$2; _dir=$3
  scan_redirect_safety probe "$_art" "$_case" || return 1
  # The resolvable site-ids the probe will self-report (one `printf 'site N …` per site).
  _emit_ids=$(printf '%s\n' "$_art" | sed -n "s/.*printf 'site \\([0-9][0-9]*\\) effect=.*/\\1/p" | LC_ALL=C sort -n)
  _log=$(mktemp)
  _sand=$(mktemp -d)
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  # Execute the probe (sandbox cwd + mocks PATH + DORC_LOG). Its stdout is the records;
  # its own stderr/the shim log are not asserted here (the probe is read-only — we assert
  # the records it returns, not what it touched, beyond the no-127 vouch check below).
  _recs=$( cd -- "$_sand" && DORC_LOG="$_log" PATH="$_mocks" "$checker_abs" 2>/dev/null <<EOF
$_art
EOF
  )
  rm -rf "$_sand"; rm -f "$_log"
  _recs=$(printf '%s\n' "$_recs" | sed 's/\r$//')

  # (a) grammar + site-completeness. Pull the well-formed records' ids; compare the SET
  # to the emitters'. A record that is missing, duplicated, or malformed shifts the set.
  _rec_lines=$(printf '%s\n' "$_recs" | grep -E '^site ' || true)
  _good_ids=$(printf '%s\n' "$_rec_lines" \
    | sed -n 's/^site \([0-9][0-9]*\) effect=\(holds\|absent\|cant-tell\) rc=-\{0,1\}[0-9][0-9]*$/\1/p' \
    | LC_ALL=C sort -n)
  if [ "$_good_ids" != "$_emit_ids" ]; then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [gate-1: probe records not site-complete/grammar-valid (every resolvable site must emit exactly one valid record)]"
      printf '      emitters: %s\n' "$(printf '%s' "$_emit_ids" | tr '\n' ' ')"
      printf '      valid records: %s\n' "$(printf '%s' "$_good_ids" | tr '\n' ' ')"
      printf '      raw records:\n'; printf '%s\n' "$_recs" | sed 's/^/        /'
    fi
    return 1
  fi

  # The opt-out marker disables (b) parity + (c) vouch-closure (this case's probe cannot
  # be faithfully mock-executed today — see the header). (a) above already passed.
  if [ -f "${_dir}PROBE_RESULTS=authored" ]; then
    return 0
  fi

  # (c) vouch-closure: no rc=127 (an un-shimmed probe command).
  if printf '%s\n' "$_rec_lines" | grep -qE 'rc=127$'; then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [gate-1: probe invoked an un-shimmed command (rc=127) — vouch-closure: a probe command has no mock (add a probe shim, or mark PROBE_RESULTS=authored)]"
      printf '%s\n' "$_rec_lines" | grep -E 'rc=127$' | sed 's/^/      /'
    fi
    return 1
  fi

  # (b) parity: the PRODUCED records must match the authored probe-results.txt. PER-SITE
  # rc-tightening (item-4 / tc-probe-parity-projection): a site whose AUTHORED record
  # carries an `rc=` is compared WITH its rc (the fold-valid Query/pkgstate rc — a wrong
  # probe-emitted rc would be a wrong fold, 20E §2); a site whose fixture omits rc keeps
  # the effect-only compare (the establish sites — their rc is the probe-command's,
  # firewalled from the fold, so it is not a parity target, and the fixtures historically
  # omit it — no mass re-authoring). The authored file is the source of truth for which
  # sites carry rc; `norm_parity.awk` strips rc from a line ONLY when that site's authored
  # record had none, applied identically to both sides.
  _authfile="${_dir}probe-results.txt"
  _produced=$(printf '%s\n' "$_rec_lines" | awk -f "$parity_norm" "$_authfile" - | LC_ALL=C sort)
  _authored=$(grep -E '^site ' "$_authfile" 2>/dev/null | awk -f "$parity_norm" "$_authfile" - | LC_ALL=C sort)
  if [ "$_produced" = "$_authored" ]; then
    return 0
  fi
  if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
    echo "FAIL  $_case  [gate-1: mocked probe records diverge from authored probe-results.txt — re-author the fixture, add probe shims, or mark PROBE_RESULTS=authored (do NOT silently re-bless)]"
    if command -v diff >/dev/null 2>&1; then
      _af=$(mktemp); printf '%s\n' "$_authored" > "$_af"
      printf '%s\n' "$_produced" | diff -u "$_af" - 2>/dev/null | sed 's/^/      /' || true
      rm -f "$_af"
    fi
  fi
  return 1
}

# gate-5 (cm-2 argv-echo differential, 20A §2 / 20B §3): cross-check the ENGINE's per-site
# resolved argv against GROUND TRUTH from dash. dash is the semantic oracle for value-flow
# (the prefix-env / `${N#pat}` bugs this round died to crosscheck would be caught here by
# construction). Mechanism:
#   - the engine's view: `dorc --debug-argv` emits `argv <leafid> <word|TOP …>` per site;
#   - ground truth: run the BARE book.sh (NOT the elided apply — it is all-shims by
#     construction, same trust envelope) under PATH=mocks + sandbox cwd; the shims log
#     `ran: <name> <args>` — exactly the executed argv per site.
#
# The assertion is ONE-DIRECTIONAL and conservative (the prompt's mandate — "be
# conservative, document, and flag rather than over-assert"): for each FULLY-RESOLVED site
# (no `TOP`) whose argv[0] is a SHIMMED command (a builtin like `set`/`echo`/`command`/`:`
# logs nothing, so it is exempt), the resolved argv MUST appear as a logged `ran:` line.
# We do NOT assert the reverse (a logged line with no matching engine site) nor a count —
# a branch the bare run skips, or a site the engine ⊤s, would make a two-directional or
# counting assertion a false failure. So: engine-resolved-and-shimmed ⊆ logged.
#
# THE DISPOSITION CARVE-OUT (task-O / tc-gate5-omit, strain-D3b-fold-vs-gate5): each engine
# line is now `argv <leafid> <disposition> <words>`. We SKIP any site whose disposition is
# not `run` — an `omit`/`replace`d site is intentionally absent from the apply run-set, and
# a guarded `omit` may legitimately be absent from the BARE book too (a preceding guard
# short-circuits it: e.g. a shimmed `dpkg -s X || install X` fold drops the install from
# both). Asserting such a site ⊆ the bare log was a FALSE failure — the structural exclusion
# that confined the fold/omit demonstration to un-shimmable BUILTIN guards (20G §5). The
# `run`-only filter removes that exclusion without weakening the gate for sites that run.
#
# $4 = the `-o oracle …` arg string (already assembled by the caller; passed verbatim).
argv_echo_check() {
  _case=$1; _dir=$2; _shims=$3
  shift 3   # the remaining args ($@) are the `-o <oracle> …` flags
  # The engine's per-site argv (stderr, behind the flag). stdin is the probe-results (the
  # flag does not change the round-trip; we just read the extra stderr lines).
  _eng=$("$dorc" --debug-argv --book="${_dir}book.sh" "$@" < "${_dir}probe-results.txt" 2>&1 >/dev/null | grep '^argv ' || true)
  # Ground truth: run the BARE book under mocks + sandbox; collect the shims' logged argvs.
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  _book=$(CDPATH= cd -- "$_dir" && pwd)/book.sh
  _log=$(mktemp); _sand=$(mktemp -d)
  ( cd -- "$_sand" && DORC_LOG="$_log" PATH="$_mocks" "$checker_abs" "$_book" ) >/dev/null 2>&1 || true
  _logged=$(sed 's/^ran: //' "$_log" 2>/dev/null || true)
  rm -rf "$_sand"; rm -f "$_log"
  # Walk each engine argv line; assert the resolved+shimmed+RUN ones are in the log.
  _bad=""
  _oldifs=$IFS; IFS='
'
  for _line in $_eng; do
    # Line shape: `argv <leafid> <disposition> <words…>`. Pull the disposition (3rd field)
    # then strip both leading tokens to get the resolved words.
    _disp=$(printf '%s' "$_line" | sed -E 's/^argv [0-9]+ ([a-z]+).*/\1/')
    [ "$_disp" = "run" ] || continue                          # not run (omit/replace) ⇒ skip
    _words=$(printf '%s' "$_line" | sed -E 's/^argv [0-9]+ [a-z]+ ?//')
    [ -z "$_words" ] && continue                              # assignment-only site
    case " $_words " in *" TOP "*) continue ;; esac           # not fully resolved ⇒ skip
    _cmd0=${_words%% *}
    case "$_shims" in *" $_cmd0 "*) ;; *) continue ;; esac    # builtin / un-shimmed ⇒ skip
    if ! printf '%s\n' "$_logged" | grep -qxF "$_words"; then
      _bad="${_bad}${_line}
"
    fi
  done
  IFS=$_oldifs
  [ -z "$_bad" ] && return 0
  if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
    echo "FAIL  $_case  [gate-5: engine-resolved argv not in the bare book's executed argvs (dash disagrees with value-flow)]"
    printf '%s' "$_bad" | sed 's/^/      /'
  fi
  return 1
}

# gate-3 (stderr-severity floor, 20B §2): dorc's stderr ($2 = the captured file) is the
# diagnostic stream — previously discarded. FAIL the case ($1) if it carries an
# ERROR-severity diagnostic (the `<stage>: error[<code>]: …` shape `report()` now emits)
# that the case does not DECLARE. A case legitimately exercising an error path (a
# ⊤-reject, a missing oracle probe) ships an `expected-diagnostics` file whose lines are
# substring-matched against the stderr; every error-line must be covered by some pattern.
# Warnings/notes are free-form and never fail a case (only `error[` is the floor). This
# closes the 20B §2 residual: an error-class diagnostic that should fail a case used to
# vanish into `2>/dev/null`.
scan_diagnostics() {
  _case=$1; _err=$2; _dir=$3
  # The error-severity lines (the floor keys on the `error[` shape, not warnings/notes).
  _errs=$(grep -E '^[a-z]+: error\[' "$_err" 2>/dev/null || true)
  [ -z "$_errs" ] && return 0
  # Declared? An error line is COVERED iff some `expected-diagnostics` pattern is a
  # substring of it (fixed-string match, `grep -F -f`). The undeclared lines are exactly
  # those NOT matched by any pattern; an empty pattern-file (or no file) declares nothing.
  _decl="${_dir}expected-diagnostics"
  if [ -f "$_decl" ] && [ -s "$_decl" ]; then
    _undeclared=$(printf '%s\n' "$_errs" | grep -vF -f "$_decl" || true)
  else
    _undeclared=$_errs
  fi
  [ -z "$_undeclared" ] && return 0
  if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
    echo "FAIL  $_case  [gate-3: undeclared error-severity diagnostic on stderr — fix the cause, or declare it in an expected-diagnostics file]"
    printf '%s\n' "$_undeclared" | sed 's/^/      /'
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

  # dorc's stdout is the artifact (probe + apply); its stderr is the diagnostic stream
  # (gate-3 asserts it — below). Capture BOTH (stderr to a temp file, no longer
  # discarded). dorc's exit status is captured (NOT piped away): a crashed/empty engine
  # must hard-fail every case BEFORE the xfail lens and BEFORE bless — empty artifacts
  # are `dash -n`-clean and a BLESS run would otherwise silently bless 43 empty goldens
  # (round-20 harness-crosscheck find-3, demonstrated with a crash-stub).
  dorc_rc=0
  err_file=$(mktemp)
  raw=$("$dorc" --book="${dir}book.sh" "$@" < "${dir}probe-results.txt" 2>"$err_file") || dorc_rc=$?
  got=$(printf '%s\n' "$raw" | sed 's/\r$//')
  if [ "$dorc_rc" -ne 0 ] || [ -z "$got" ]; then
    echo "FAIL  $name  [dorc exited rc=$dorc_rc / produced no output — a dead engine is never green]"
    rm -f "$err_file"
    fails=$((fails + 1))
    continue
  fi

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
  #
  # gate-1 (rule-probe-exec-gate): the PROBE half — for the same mocks/ cases, EXECUTE
  # the rendered probe under the shims and assert site-completeness + grammar (always),
  # plus parity + vouch-closure (unless the case carries a PROBE_RESULTS=authored marker).
  # Bless does not re-author probe fixtures (the opt-out exists precisely so the suite
  # never silently re-blesses fixtures to match all-exit-0 mock output).
  if [ "$case_ok" -eq 1 ] && [ -d "${dir}mocks" ]; then
    exec_check apply "$apply_art" "$name" "$dir" || case_ok=0
    probe_exec_check "$probe_art" "$name" "$dir" || case_ok=0
    # gate-5 (cm-2 argv-echo differential): cross-check the engine's per-site resolved
    # argv against the bare book's executed argvs under dash. Conservative, one-directional
    # (engine-resolved-and-shimmed ⊆ logged). Pass the space-delimited shim set + the
    # `-o oracle …` args. Not run under BLESS (it asserts, never re-authors).
    if [ "${BLESS:-}" != "1" ]; then
      _shimset=" $(cd "${dir}mocks" && ls | tr '\n' ' ')"
      argv_echo_check "$name" "$dir" "$_shimset" "$@" || case_ok=0
    fi
  fi

  # gate-3 (stderr-severity floor): an undeclared error-severity diagnostic fails the
  # case (declare legitimate ones in expected-diagnostics). Always run — analysis-only
  # cases (no mocks/) emit diagnostics too (⊤-rejects, missing-probe).
  scan_diagnostics "$name" "$err_file" "$dir" || case_ok=0

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
  rm -f "$err_file"
done

echo "---"
if [ "$fails" -ne 0 ]; then
  echo "$fails/$total e2e round-trips FAILED" >&2
  exit 1
elif [ "${BLESS:-}" = "1" ]; then
  echo "blessed $total cases (all ap-2 gates passed)"
else
  echo "all $total e2e round-trips passed (ap-2 $checker -n + apply/probe exec gates, redirect sandbox, ordered run-set, stderr floor, argv-echo differential)"
fi
