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
#
# PER-CASE MARKER FILES (the marker idiom — opt-in behaviour spelled by a file's PRESENCE,
# the value, where one exists, carried in the filename so no parsing is needed):
#   RAN_ORDER=lax          — compare the apply run-set order-INsensitively (concurrent
#                            pipeline stages log nondeterministically; tc-pipe-ran-order).
#   DORC_FLAGS=<flags>     — append <flags> to EVERY dorc invocation for this case (the main
#                            round-trip + gate-5 + BOTH gate-6 runs read one shared `$@`, so no
#                            flag mismatch is possible). Stage 2's `DORC_FLAGS=--trust-footprints`
#                            opts a case into the survival tier; its UNFLAGGED sibling (no marker)
#                            asserts the byte-identical Stage-1 baseline. At most one marker.
#   PROBE_RESULTS=authored — disable gate-1 probe parity + vouch-closure for this case (its
#                            probe cannot be faithfully mock-executed yet; (a) still holds).
#                            ALSO excludes the case from gate-6 (authored-divergence).
#   EXIT_RC=<n>            — assert the APPLY artifact exits exactly <n> (default 0); for a
#                            faithful nonzero-exit artifact (`set -e; guard && {dead}` exits
#                            1). Governs the apply exec only; bless never creates it.
#   DORC_EXIT=<n>         — assert the dorc PROCESS exits exactly <n> (default 0) — distinct
#                            from EXIT_RC (the artifact's exec rc). ack-1 exit-code family: a
#                            parse-error / unmodeled-book case fast-fails with 10 while still
#                            emitting its artifact, so the crash-guard tolerates the declared
#                            code and runs the ordinary gates. At most one marker; bless never
#                            creates it (a hand-derived assertion, not blessable output).
#   DUAL_RAIL=inlined      — exclude the case from gate-6 (cm-1 dual-rail): a function-inlined/
#                            wrapper-pun case whose `--debug-argv` ledger is the call-site
#                            surface argv, not the inlined-body argv the bare run logs
#                            (arch-2; tc-gate6-inlining). gate-5 + exec gates still apply.
#   DUAL_RAIL=multiline-argv — exclude from gate-6: a literal-NEWLINE-arg case where
#                            `--debug-argv` shows only the arg's first line but the slice-4 bare
#                            log encodes the whole arg (`… multi\nline`); the ledger and bare
#                            argv representations can't match (tc-gate6-multiline-argv).
#   XFAIL                  — documented known-defect pin (its 1st line is the reason); the
#                            case asserts the SAFE behaviour and is expected to fail at HEAD.
#   head-expected.ran      — TWO-SIDED xfail pin (23B-fd1/23C-fd4). Present ONLY on an XFAIL
#                            case: records that case's CURRENT (HEAD, defect-present) apply
#                            run-set. While the XFAIL is present the harness asserts the current
#                            run-set STILL equals it — so a disaster-shaped behaviour change (a
#                            regression wrongly ELIDING a vouched past-wall mutator, the elide
#                            tier softening the poison wall) goes RED instead of camouflaging as
#                            an ordinary one-sided `xfail`. Consulted only while case_ok=0 (once
#                            the designed behaviour fully lands, case_ok=1 ⇒ XPASS and the drift
#                            is the intended promotion). BLESS never creates it (hand-captured).
#
# PROMOTION DISCIPLINE (guard-tier build window; 23C-fd4) — the XFAIL/XPASS/bless path is
# structurally BLIND to golden TEXT: the content golden-diff is skipped for XFAIL cases, XPASS
# fires on gates alone, and BLESS regenerates expected.out from whatever the engine emitted.
# So every artifact-shape law that lives only in golden text (rul-ternary-verdict's two nevers,
# bytes-survive-verbatim, strip-only) has NO mechanical teeth on the promotion path unless a
# GATE asserts it. Two teeth exist for guards (guard_shape_check + the head-expected.ran pin),
# but they are floors, not a full diff. THE RULE: when an XFAIL guard case turns XPASS, DIFF the
# engine's stdout against the hand-authored expected.out and inspect it line-by-line BEFORE
# deleting the XFAIL or running BLESS — never bless-first. Cosmetic divergence (comment wording,
# whitespace) is re-golden'd under review; a shape-law divergence (engine-synthesized sh in
# guard position, mutated fall-through bytes, a probe-half that stopped shipping) is a STOP.
#
# DETERMINISM RAIL (slice-2 / 221 dc-1) — the three artifact-execution sites (exec_check,
# probe_exec_check, gate-5's bare-book run) run the artifact under a FIXED environment:
# `env -i PATH=<mocks-only> DORC_LOG=<abs> LC_ALL=C TZ=UTC <checker> …`, with `umask 022`
# set in the execution subshell first. So the artifact sees ONLY those four vars + that
# umask — an ambient LANG/LC_*/TZ/umask can no longer perturb a render's exec trace. `env`
# is found via the harness's own PATH; only the ARTIFACT's PATH is mocks-only. (The existing
# `sort` calls keep their `LC_ALL=C`.) `env -i` is verified portable under msys/dash here.
#
# WHAT IS DELIBERATELY NOT PINNED (the honest residual — these can still vary run-to-run; a
# case that depends on any of them is non-hermetic and out of this harness's contract):
#   - filesystem state: the sandbox is a fresh `mktemp -d`, but the broader fs (existence of
#     /etc, /tmp, device nodes, the mocks dir's own inode) is the ambient host's.
#   - mktemp PATHS: `$_log` / `$_sand` names embed a random/PID component (different each run);
#     never assert on them. They are passed in, not discovered by the artifact.
#   - the checker binary's IDENTITY/VERSION: whichever `dash`/`sh` `command -v` found first,
#     at whatever version the host ships — its parser/builtin quirks are not pinned.
#   - kernel / OS / msys-vs-real-POSIX: syscall and tool behaviour differences are ambient.
#   - hostname / uid / cwd-of-record leakage VIA THE ARTIFACT'S OWN READS: a render that runs
#     `hostname`/`id`/`pwd` (none in the corpus today) would read the real host; `env -i`
#     scrubs the ENVIRONMENT, not these syscalls. PWD inside the sandbox is the mktemp dir.
#   - wall-clock / RNG reached by a shim that calls `date`/`$RANDOM` (the shims don't).
# The kernel-level DST guarantee lives in the Rust `hostsim` seam (21D); this rail only fixes
# the SHELL-EXEC environment of the e2e corpus, not those deeper axes.
#
# MOCK-LOG PROTOCOL (slice-4) — every inert mock shim logs its invocation by sourcing a shared
# per-mocks-dir dot-helper (`mocks/.log`, a DOTFILE so the `ls`-derived shimset never lists it)
# via `. "${0%/*}/.log"; _dorc_logged "$@"`, instead of the old inline
# `printf 'ran: %s %s\n' "${0##*/}" "$*"`. WHY: an argument containing a NEWLINE would split the
# old single `ran:` line and silently corrupt every line-based compare (exec_check, gate-5,
# gate-6). Shims run under PATH=mocks-only ⇒ BUILTINS ONLY, so the helper encodes in pure sh.
# GRAMMAR (what expected.ran goldens carry): one `ran: <name> <arg>…` line per invocation, args
# space-joined; each arg is encoded — a literal backslash → `\\`, an embedded newline → the two
# chars `\n`. Passes run backslash-FIRST so a newline's encoded `\n` is not re-doubled. A
# newline-free, backslash-free argv encodes to itself, and per-arg space-join reproduces the old
# `"$*"` join byte-for-byte ⇒ ZERO golden churn (no current golden embeds a newline). gate-5's
# `grep -qxF` and gate-6's judge operate on the encoded lines unchanged. (Two NON-logging mocks
# files are exempt and untouched: a `grep` probe that does a real substring match, and a sourced
# `helper.sh` no-op — neither emits a `ran:` line.)
set -eu

# E4 (27D disposition-legacy-deframe-tolerance): the ~128 authored probe-results.txt fixtures are
# UNFRAMED (headerless) — they carry no dorc-records/1 header/token. Production reads are now STRICT
# (a headerless stream refuses, kFAIL-withhold), so the harness opts into the lenient legacy
# passthrough via this env var (read at the cli edge; the kernel deframer stays a pure function of
# its policy parameter). Exported here so EVERY `$dorc` invocation below inherits it; the artifact
# execs run under `env -i` (scrubbed) but those exec the rendered probe/apply via $checker, not dorc.
export DORC_ALLOW_LEGACY_RESULTS=1

# 262 §2 dorc-records/1 framing: the fixed spike nonce + terminal token the probe emits, used by
# gate-1 to deframe the executed probe's record stream. These MIRROR the Rust constants
# plan::records::DEFAULT_NONCE and TERMINAL_TOKEN — keep the two in sync (a spike two-source-of-
# truth; the real tool mints a per-attempt nonce, but the e2e default is fixed for stable goldens).
RECORDS_NONCE=dorc
RECORDS_TOKEN='@@dorc@@'

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
# determinism rail (slice-2, 221 dc-1): the three artifact-execution sites run the artifact
# under `env -i` (a scrubbed environment). Require `env` explicitly — a missing one would
# otherwise drop the determinism pin silently. (`env -i` verified portable under msys/dash on
# this machine; if a future host fights it, an explicit `unset`-based fallback is acceptable.)
if ! command -v env >/dev/null 2>&1; then
  echo "no env for the determinism rail (env -i scrubbed-environment exec) — cannot pin the artifact env" >&2
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

# Shared sandbox+mocks execution (slice-3 — gate-5's bare-book run and dual_rail's two rails
# all need a run-set, so the sandbox/env-i/umask machinery lives here ONCE instead of being
# copied per gate). Run a payload under the determinism rail (umask 022 + `env -i` fixed env,
# slice-2) with PATH = the case's mocks ($3, absolute) and DORC_LOG → a fresh temp, then echo
# the shims' logged argvs (the `ran: ` prefix stripped), one per line, in execution order.
#   $1 = mode: `file` (payload is a script PATH) or `stdin` (payload is artifact TEXT on stdin)
#   $2 = payload (a book path, or the artifact text)
#   $3 = absolute mocks dir
# A nonzero payload exit is tolerated (`|| true`) — a faithful book/artifact may exit nonzero
# (slice-1 EXIT_RC); we want its TRACE, not its rc. NB exec_check keeps its OWN run (it must
# capture the rc to assert EXIT_RC); this helper is for the trace-only consumers.
capture_run() {
  _mode=$1; _payload=$2; _cmocks=$3
  _crlog=$(mktemp); _crsand=$(mktemp -d)
  if [ "$_mode" = file ]; then
    ( cd -- "$_crsand" && umask 022 && env -i PATH="$_cmocks" DORC_LOG="$_crlog" LC_ALL=C TZ=UTC "$checker_abs" "$_payload" ) >/dev/null 2>&1 || true
  else
    ( cd -- "$_crsand" && umask 022 && env -i PATH="$_cmocks" DORC_LOG="$_crlog" LC_ALL=C TZ=UTC "$checker_abs" >/dev/null 2>&1 <<EOF
$_payload
EOF
    ) || true
  fi
  sed 's/^ran: //' "$_crlog" 2>/dev/null || true
  rm -rf "$_crsand"; rm -f "$_crlog"
}

# head_ran_check (the TWO-SIDED xfail pin, 23B-fd1/23C-fd4): assert an XFAIL case's CURRENT
# apply run-set still equals its `head-expected.ran` marker (the HEAD, defect-present signature
# captured when the pin was authored). $1=case, $2=dir, $3=the current apply artifact text.
# Returns 0 (equal / no marker / no mocks — nothing to drift) or 1 (DRIFTED). The caller only
# heeds a drift while case_ok=0: a drift with case_ok=1 is the designed behaviour landing (⇒
# XPASS), a drift with case_ok=0 is a disaster-shaped change hiding as an ordinary xfail. The
# marker keeps the `ran: ` prefix (author-consistent with expected.ran); capture_run strips it,
# so strip the marker side too before comparing. Ordered compare by default; a case carrying
# RAN_ORDER=lax (concurrent pipeline stages log nondeterministically — tc-pipe-ran-order) gets
# the same order-INsensitive compare exec_check gives its expected.ran: both sides sorted, so
# the pin asserts the run-set as a MULTISET. The two-sided pin's purpose survives lax intact —
# a disaster-shaped drift changes WHICH commands ran (a wrongly-elided wall drops a line, a
# wrongly-run mutator adds one), which changes the multiset and still goes RED; only benign
# stage reordering within one apply is forgiven. (First lax consumer:
# strawman24-pipe-guard-oracle-converged, whose check-pipe stages race.)
head_ran_check() {
  _case=$1; _dir=$2; _apply=$3
  [ -f "${_dir}head-expected.ran" ] || return 0     # opt-in marker
  [ -d "${_dir}mocks" ] || return 0                 # nothing to execute
  _hmocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  _hgot=$(capture_run stdin "$_apply" "$_hmocks")
  _hwant=$(sed 's/^ran: //' "${_dir}head-expected.ran" 2>/dev/null || true)
  if [ -f "${_dir}RAN_ORDER=lax" ]; then
    _hgot=$(printf '%s\n' "$_hgot" | LC_ALL=C sort)
    _hwant=$(printf '%s\n' "$_hwant" | LC_ALL=C sort)
  fi
  [ "$_hgot" = "$_hwant" ] && return 0
  return 1
}

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
#
# EXPECTED-EXIT (tc-exec-nonzero-exit, 215 §3 strain-and-exit): by default the APPLY
# artifact must exit 0 — a nonzero exit is "errored when run". But a FAITHFUL artifact can
# legitimately exit nonzero: `set -e; guard && { … }` with a failing guard short-circuits
# the `&&` and the AND-OR list's rc is the failed left's (dash: `set -e; false && {…}`
# exits 1, does NOT abort). The bare book exits the same, so the artifact is correct — the
# old gate FALSE-failed it (door1-and-form had to be analysis-only). A case asserts its
# expected exit with a per-case marker FILE `EXIT_RC=<n>` (the marker idiom — the value
# lives in the filename, like RAN_ORDER=lax). Present ⇒ assert rc == n EXACTLY (equality,
# not tolerance: a 0 exit when n≠0 is ALSO a failure — the artifact stopped reproducing the
# faithful nonzero rc). Absent ⇒ rc == 0 as before. Governs the APPLY exec ONLY; the probe
# is never expected nonzero (probe_exec_check is unchanged). BLESS never creates/consults
# the marker (an expected-exit is a hand-derived assertion, not blessable output).
exec_check() {
  _label=$1; _art=$2; _case=$3; _dir=$4
  scan_redirect_safety "$_label" "$_art" "$_case" || return 1

  # Resolve the EXIT_RC=<n> marker (default 0). Glob it; refuse >1 marker loudly (an
  # ambiguous expected-exit is an authoring error, not a silently-picked one).
  _exp_rc=0
  _marker_count=0
  for _m in "${_dir}"EXIT_RC=*; do
    [ -e "$_m" ] || continue       # glob with no match yields the literal pattern
    _marker_count=$((_marker_count + 1))
    _exp_rc=${_m##*EXIT_RC=}
  done
  if [ "$_marker_count" -gt 1 ]; then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [ap-2-exec: multiple EXIT_RC=<n> markers — exactly one expected-exit is permitted]"
    fi
    return 1
  fi
  case $_exp_rc in
    ''|*[!0-9]*)
      if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
        echo "FAIL  $_case  [ap-2-exec: EXIT_RC marker value '$_exp_rc' is not a non-negative integer]"
      fi
      return 1 ;;
  esac

  _log=$(mktemp)
  _sand=$(mktemp -d)
  # Absolute mocks dir: PATH is about to become *only* this dir, so a relative path
  # would break.
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  # Execute in a subshell cd'd into the sandbox (gate-2): a bare relative redirect lands
  # under $_sand, not the repo. $_log + $checker_abs are absolute, unaffected by the cd.
  # DETERMINISM RAIL (slice-2, 221 dc-1): `env -i` scrubs the ambient environment so the
  # artifact runs under a FIXED env (only PATH=mocks, DORC_LOG, LC_ALL=C, TZ=UTC) — an
  # ambient LANG/LC_*/TZ can no longer perturb a locale-sensitive command's output. `env`
  # itself is found via the harness's own PATH (this subshell still has it); only the
  # ARTIFACT's PATH is mocks-only. `umask 022` is set before exec (umask is a process
  # attribute, inherited THROUGH `env -i` — verified on this msys box) so file-creation mode
  # is fixed. The artifact's real exit status is captured (not collapsed by `if !`) so
  # EXIT_RC can assert it exactly.
  _run_rc=0
  _run_err=$( cd -- "$_sand" && umask 022 && env -i PATH="$_mocks" DORC_LOG="$_log" LC_ALL=C TZ=UTC "$checker_abs" 2>&1 <<EOF
$_art
EOF
  ) || _run_rc=$?
  if [ "$_run_rc" -ne "$_exp_rc" ]; then
    if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
      echo "FAIL  $_case  [ap-2-exec: rendered $_label exited rc=$_run_rc, expected $_exp_rc]"
      printf '      %s\n' "$_run_err"
    fi
    rm -rf "$_sand"; rm -f "$_log"
    return 1
  fi
  rm -rf "$_sand"
  _got_ran=$(cat "$_log")
  rm -f "$_log"
  # BLESS regenerates expected.ran from the CURRENT engine output — but NEVER for an
  # XFAIL case: its expected.ran is the hand-authored DESIRED-FUTURE (guarded) run-set the
  # unbuilt engine cannot yet produce, so blessing it from HEAD output silently clobbers
  # the pin (23H §8 — a global BLESS overwrote the drift-trio's expected.ran, flipping 3
  # xfails to XPASS; it has bitten twice). Under XFAIL we skip the bless and fall through to
  # the ordinary compare, so the case stays a red `xfail` (goldens untouched). The
  # expected.out bless (main loop) is already XFAIL-guarded structurally; this closes the ran side.
  if [ "${BLESS:-}" = "1" ] && [ "${XFAIL_ACTIVE:-}" != "1" ]; then
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
#       an un-shimmed probe command does NOT abort the probe — the `__predict` wrappers
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
  _art=$1; _case=$2; _dir=$3; _shim=$4
  scan_redirect_safety probe "$_art" "$_case" || return 1
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  # Shim-materialization last mile (`274` §5 / `27L` task-14): an entry-composed probe execs its inner
  # check across the wrapper boundary (a shell function does not survive exec); $_shim carries those
  # checks as executables. MOCKS-FIRST (mocked tools keep winning; the shim adds only the disjoint
  # oracle-check names); `chmod +x` ensures the bit under msys. Empty ⇒ PATH unchanged.
  _probe_path="$_mocks"
  if [ -n "$_shim" ] && [ -d "$_shim" ] && [ -n "$(ls -A "$_shim" 2>/dev/null)" ]; then
    chmod +x "$_shim"/* 2>/dev/null || true
    _probe_path="$_mocks:$_shim"
  fi
  # The resolvable site-keys the probe will self-report (one `printf 'site <key> …` per
  # site). A key is `N` or — for an in-loop Members member (task-L2 item-4) — `N.M`, so the
  # pattern accepts a dot; the SET compare below uses a lexical sort (a `.M` key is not a
  # plain integer, so `sort -n` would mis-order, but lexical equality of the two sets holds).
  _emit_ids=$(printf '%s\n' "$_art" | sed -n "s/.*printf '$RECORDS_NONCE site \\([0-9][0-9.]*\\) effect=.*/\\1/p" | LC_ALL=C sort)
  _log=$(mktemp)
  _sand=$(mktemp -d)
  # Execute the probe (sandbox cwd + mocks[+shim] PATH + DORC_LOG). Its stdout is the records;
  # its own stderr/the shim log are not asserted here (the probe is read-only — we assert
  # the records it returns, not what it touched, beyond the no-127 vouch check below).
  # DETERMINISM RAIL (slice-2, 221 dc-1): same fixed-env discipline as exec_check — `env -i`
  # + `umask 022` so the probe's records cannot drift on an ambient locale/TZ/umask.
  _recs=$( cd -- "$_sand" && umask 022 && env -i PATH="$_probe_path" DORC_LOG="$_log" LC_ALL=C TZ=UTC "$checker_abs" 2>/dev/null <<EOF
$_art
EOF
  )
  rm -rf "$_sand"; rm -f "$_log"
  # 262 §2 framing: the probe now emits the dorc-records/1 stream (a `dorc-records/1 …` header,
  # per-record `<nonce> … <token>` lines, and a `dorc-records-end/1 …` sentinel). DEFRAME it here
  # to the inner records the rest of gate-1 already understands: keep only `<nonce> <inner>
  # <token>` lines, stripping the nonce prefix + terminal token (last-to-token). The header/
  # sentinel start with `<nonce>-…`, not `<nonce> `, so they drop. (RECORDS_NONCE/RECORDS_TOKEN
  # mirror plan::records::{DEFAULT_NONCE,TERMINAL_TOKEN} — keep in sync.)
  _recs=$(printf '%s\n' "$_recs" | sed 's/\r$//' \
    | sed -n "s/^$RECORDS_NONCE \\(.*\\) $RECORDS_TOKEN\$/\\1/p")

  # (a) grammar + site-completeness. Pull the well-formed records' ids; compare the SET
  # to the emitters'. A record that is missing, duplicated, or malformed shifts the set.
  _rec_lines=$(printf '%s\n' "$_recs" | grep -E '^site ' || true)
  _good_ids=$(printf '%s\n' "$_rec_lines" \
    | sed -n 's/^site \([0-9][0-9.]*\) effect=\(holds\|absent\|cant-tell\) rc=-\{0,1\}[0-9][0-9]*$/\1/p' \
    | LC_ALL=C sort)
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
  # Shared via capture_run (slice-3) — file mode, the determinism rail (env -i + umask, slice-2)
  # lives in that helper now. dual_rail_check (rail-1) runs the identical bare-book capture.
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  _book=$(CDPATH= cd -- "$_dir" && pwd)/book.sh
  _logged=$(capture_run file "$_book" "$_mocks")
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

# gate-6 — the DUAL-RAIL corpus judge (cm-1, 20K §4's "the one gate that observes elided
# sites"; the corpus-tier analogue of the hostsim differential, note 21D). gate-5 checks the
# engine's RESOLUTION (argv ⊆ bare). This checks the engine's ELISION: that the apply's
# run-set differs from the bare book's run-set ONLY by licensed elisions. Two rails:
#   rail-1 (bare run-set): the BARE book under mocks (capture_run, shared with gate-5).
#   rail-2 (apply run-set): the eliding-apply artifact under the SAME mocks (re-run here — the
#     21D-sanctioned "else re-run"; cheap, and keeps this gate self-contained vs threading a
#     global out of exec_check).
# License ledger = the engine's OWN `replace`/`omit` dispositions (`--debug-argv`), the stabler
# choice over artifact-comment parsing (21D §1). The judge is CONSERVATIVE + ONE-DIRECTIONAL
# like gate-5 (the prompt's mandate):
#   (i)  every apply `ran:` line MUST appear in the bare run — the apply never runs anything
#        NEW. (door-4-era amends this — an apply could synthesize a re-probe; leave the comment.)
#   (ii) every BARE line ABSENT from the apply MUST be license-attributable: it matches the
#        resolved argv of a `replace`/`omit` site, with TOP as a position-wildcard (a converged
#        loop is `replace … TOP`; the bare run has the concrete members — 21D find-5). Same
#        builtin/shimset skip as gate-5 (a builtin never logs, so it can't be a delta anyway).
# An omitted site absent from BOTH rails simply never enters the delta (the fold-short-circuit
# subtlety, gate-5's carve-out, falls out of direction (ii)'s framing — a guarded `omit` the
# bare book also short-circuits is just not in `bare`). Any unattributable delta ⇒ loud FAIL.
#
# WHAT THIS GATE DOES NOT DO (the honest non-coverage — exclusions + structural limits):
#   - PROBE_RESULTS=authored cases are EXCLUDED: their mock-rc and authored probe-results diverge,
#     so the bare control-flow and the apply's elision are driven by DIFFERENT convergence —
#     bare-vs-apply divergence is then expected, not a license violation (would false-fail).
#   - DUAL_RAIL=inlined cases are EXCLUDED (tc-gate6-inlining): under function inlining the
#     engine's `--debug-argv` reports the CALL-site surface argv (`replace apt_install nginx`)
#     while the bare run logs the inlined-BODY resolved argv (`apt-get install -y nginx`). The
#     two argv spellings cannot match, so surface-argv attribution false-fails the wrapper-pun
#     (arch-2 / note 216 inv-leaf-seam: the LeafId→AstId map is non-injective under inlining).
#     The AUTHORITATIVE cm-1 (21D's hostsim differential) attributes these via the in-process
#     API; the corpus tier cannot, so it declares them out of scope with the marker.
#   - DUAL_RAIL=multiline-argv cases are EXCLUDED (tc-gate6-multiline-argv): a command with a
#     literal-NEWLINE argument. `--debug-argv` renders only the arg's FIRST line (`… multi`),
#     but the slice-4 mock-log protocol encodes the whole arg into one line (`… multi\nline`),
#     so the ledger and bare argv cannot match. (Pre-slice-4 this false-PASSED by accident: the
#     un-encoded newline split the bare log into two records, the first of which coincidentally
#     equalled the truncated ledger line — a match that ignored the second line entirely. Slice-4
#     removed that accident; the honest disposition is exclusion.)
#   - TWO-DIRECTIONAL branch-divergence attribution + HOST-STATE variation are NOT done here —
#     that is the hostsim differential's domain (21D). gate-6 judges ONE fixed host-state per
#     case (the authored probe-results); it does not sweep host-states to catch a wildcard
#     license masking a diverged loop member (21D's removed_line_is_converged cross-check).
#     Consequence: a TOP-wildcard `replace … TOP` is honored for ANY concrete member here; a
#     wrongly-whole-replaced loop with a diverged member would be caught by 21D, not gate-6.

# dual_rail_judge — the PURE judge (no I/O, no case dir): given the rails + the RAW disposition
# ledger as newline lists, echo every violation line (empty output ⇒ pass). Factored out so
# dual_rail_selftest can drive it on FABRICATED inputs (a lying judge is worse than no judge —
# the self-test proves it screams). $1=bare run-set, $2=apply run-set, $3=RAW `--debug-argv`
# disposition lines (`argv <leafid> <disp> <words>`), $4=shimset (space-delimited+bracketed).
# The license filter (replace/omit ONLY — `run` is NOT a license) lives HERE, not in the caller,
# so cf-3 can feed a `run` line and prove the judge ignores it.
dual_rail_judge() {
  _jbare=$1; _japply=$2; _jdisp=$3; _jshims=$4; _jguardcmds=$5
  _viol=""
  # The LICENSE LEDGER: resolved argvs of replace/omit/GUARD sites (23A §5 widening — a guard
  # licenses its OWN suppressed mutator, cf-6). A `run` disposition never enters here (cf-3's
  # invariant). TOP is kept (wildcard-matched below).
  _jledger=$(printf '%s\n' "$_jdisp" | sed -nE 's/^argv [0-9]+ (replace|omit|guard) (.+)$/\2/p')
  _oldifs=$IFS; IFS='
'
  # (i) apply ⊆ bare — the apply never runs anything NEW, EXCEPT a guard's own check-command (the
  # guard's live check runs at apply, absent from the bare book — 23A §5). A `guardcmd <argv0>`
  # ledger line allowlists exactly those argv0s; an UNRELATED apply-only line still screams (cf-5).
  for _al in $_japply; do
    [ -z "$_al" ] && continue
    printf '%s\n' "$_jbare" | grep -qxF "$_al" && continue    # in bare ⇒ not apply-only
    _alcmd0=${_al%% *}
    printf '%s\n' "$_jguardcmds" | grep -qxF "$_alcmd0" && continue   # a guard's own check-command
    _viol="${_viol}apply-only (ran in apply, not in bare): $_al
"
  done
  # (ii) every bare-only line attributable to a replace/omit ledger entry (TOP-wildcard).
  for _bl in $_jbare; do
    [ -z "$_bl" ] && continue
    printf '%s\n' "$_japply" | grep -qxF "$_bl" && continue   # in apply ⇒ not elided ⇒ not a delta
    _bcmd0=${_bl%% *}
    case "$_jshims" in *" $_bcmd0 "*) ;; *) continue ;; esac    # builtin/un-shimmed never logs
    _ok=0
    for _le in $_jledger; do
      [ -z "$_le" ] && continue
      if argv_words_match "$_le" "$_bl"; then _ok=1; break; fi
    done
    [ "$_ok" -eq 1 ] || _viol="${_viol}unattributable bare-only (elided with no replace/omit license): $_bl
"
  done
  IFS=$_oldifs
  printf '%s' "$_viol"
}

# argv_words_match: ledger entry ($1, may contain TOP) vs concrete line ($2), word-by-word,
# TOP = single-word wildcard. Pure-builtin (no sed/awk). MUST restore the DEFAULT IFS (space/
# tab/newline) for its word-splitting — its caller dual_rail_judge runs with IFS=newline to walk
# run-set LINES, under which `set -- $argv` would NOT split on spaces (the whole argv would be
# one "word", breaking the arity + per-word compare). Runs in a SUBSHELL so the `unset IFS` is
# scoped (the caller's newline-IFS is untouched on return).
argv_words_match() (
  unset IFS
  _aw_ledg=$1; _aw_conc=$2
  set -- $_aw_ledg; _aw_ln=$#
  set -- $_aw_conc; _aw_cn=$#
  [ "$_aw_ln" -eq "$_aw_cn" ] || exit 1   # arity differs ⇒ no match (subshell: exit, not return)
  # Lockstep walk: iterate the ledger words; for each, pop the next concrete word off $@.
  set -- $_aw_conc
  for _aw_lw in $_aw_ledg; do
    _aw_cw=$1; shift
    [ "$_aw_lw" = TOP ] || [ "$_aw_lw" = "$_aw_cw" ] || exit 1
  done
  exit 0
)

# dual_rail_check (gate-6 per-case driver): assemble the three rails + shimset, run the judge,
# report. $1=case, $2=dir, $3=shimset; remaining $@ = the `-o oracle …` flags.
dual_rail_check() {
  _case=$1; _dir=$2; _shims=$3
  shift 3
  _mocks=$(CDPATH= cd -- "${_dir}mocks" && pwd)
  # RAW `--debug-argv` readout: the disposition ledger (`argv …`; the judge filters to
  # replace/omit/guard) PLUS the guard check-command allowlist (`guardcmd <argv0>`, 23A §5).
  _dbg=$("$dorc" --debug-argv --book="${_dir}book.sh" "$@" < "${_dir}probe-results.txt" 2>&1 >/dev/null)
  _disp=$(printf '%s\n' "$_dbg" | grep '^argv ' || true)
  _guardcmds=$(printf '%s\n' "$_dbg" | sed -nE 's/^guardcmd (.+)$/\1/p' || true)
  # rail-1: bare book (shared capture_run, file mode).
  _book=$(CDPATH= cd -- "$_dir" && pwd)/book.sh
  _bare=$(capture_run file "$_book" "$_mocks")
  # rail-2: the eliding-apply artifact (re-run; 2nd shebang block).
  _apply_art=$("$dorc" --book="${_dir}book.sh" "$@" < "${_dir}probe-results.txt" 2>/dev/null \
    | awk 'BEGIN{c=0} /^#!\/bin\/sh/{c++} c>=2{print}')
  _apply=$(capture_run stdin "$_apply_art" "$_mocks")
  _viol=$(dual_rail_judge "$_bare" "$_apply" "$_disp" "$_shims" "$_guardcmds")
  [ -z "$_viol" ] && return 0
  if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
    echo "FAIL  $_case  [gate-6: apply/bare run-set delta not covered by the license ledger (cm-1 dual-rail)]"
    printf '%s' "$_viol" | sed 's/^/      /'
  fi
  return 1
}

# dual_rail_selftest (the confound battery, run ONCE at harness start): drive dual_rail_judge
# on FABRICATED fixtures proving it SCREAMS on the failure modes. A judge that passes a real
# under-execute is worse than no judge, so a self-test failure ABORTS the harness (exit 3). The
# fixtures are hand-built strings, not corpus cases (no engine, no mocks — pure judge logic).
# The 3rd arg is RAW `argv <id> <disp> <words>` lines, exactly as the judge receives in-corpus.
dual_rail_selftest() {
  _st_shims=" instpkg systemctl "
  _fails=""
  # cf-1: an apply-only line (apply ran something absent from bare) ⇒ must scream (direction (i):
  # the apply must never run anything the bare book didn't).
  _r=$(dual_rail_judge "instpkg install nginx" "instpkg install nginx
systemctl restart sshd" "argv 1 run instpkg install nginx" "$_st_shims" "")
  case $_r in *apply-only*) ;; *) _fails="${_fails}cf-1 (apply-only line not caught)
" ;; esac
  # cf-2: an unattributable bare-only line — bare ran a shimmed cmd that the apply elided, with NO
  # replace/omit license covering it ⇒ must scream (direction (ii): the under-execute disaster).
  _r=$(dual_rail_judge "instpkg install nginx
systemctl restart sshd" "instpkg install nginx" "argv 1 run instpkg install nginx" "$_st_shims" "")
  case $_r in *unattributable*) ;; *) _fails="${_fails}cf-2 (unattributable bare-only not caught)
" ;; esac
  # cf-3: the bare-only line IS covered by a ledger entry, but that entry's disposition is `run`
  # (NOT replace/omit) ⇒ must STILL scream. `run` is not a license — only replace/omit attribute.
  # This is the load-bearing confound: it proves the judge's replace/omit filter actually fires
  # (a judge that attributed via ANY disposition would wrongly pass this).
  _r=$(dual_rail_judge "systemctl restart sshd" "" "argv 7 run systemctl restart sshd" "$_st_shims" "")
  case $_r in *unattributable*) ;; *) _fails="${_fails}cf-3 (a `run`-disposition entry wrongly attributed an elided line)
" ;; esac
  # cf-PASS (negative control): a converged loop's TOP-wildcard `replace` ledger DOES license its
  # concrete bare members ⇒ must NOT scream (proves the wildcard isn't vacuously failing — and
  # that cf-1..3's screams are real discrimination, not a judge that rejects everything).
  _r=$(dual_rail_judge "instpkg install nginx
instpkg install curl" "" "argv 0 replace instpkg install TOP" "$_st_shims" "")
  case $_r in "") ;; *) _fails="${_fails}cf-PASS (TOP-wildcard failed to license a converged member: $_r)
" ;; esac
  # cf-5 (guard-disposition forward-lock, 23C-fd5): once gate-6 is WIDENED to admit a `guard`
  # disposition (build-round work — 23A §5's "direction (i) allowlists apply-only lines that
  # match the shipped preamble's command set"), a lazy widening that simply stops screaming on
  # apply-only lines whenever a guard is present would go BLIND to the cf-1 mutation class
  # exactly where guards live. This confound locks the requirement in AHEAD of the widening: a
  # `guard` ledger entry must NEVER license an UNRELATED apply-only line (only the guard's own
  # check-command may be allowlisted). Against the CURRENT (un-widened) judge it passes trivially
  # — `guard` is not in the replace/omit license filter, so direction (i) screams on ANY
  # apply-only line regardless — but it will red a widening that over-broadly whitelists, which
  # is the whole point. (The PAIRED negative control — a guard licensing its OWN suppressed
  # mutator ⇒ must NOT scream — is DEFERRED to the widening: the current judge has no `guard`
  # license semantics, so a non-scream assertion there would wrongly fire this FATAL now. Add
  # cf-6 when the judge learns to attribute a guard's own site.)
  _r=$(dual_rail_judge "instpkg install nginx" "instpkg install nginx
systemctl restart sshd" "argv 2 guard instpkg install curl" "$_st_shims" "")
  case $_r in *apply-only*) ;; *) _fails="${_fails}cf-5 (a guard disposition wrongly licensed an unrelated apply-only line)
" ;; esac
  # cf-6 (the paired non-scream control cf-5 deferred until the judge learned guard semantics):
  # a guard licensing its OWN suppressed mutator ⇒ must NOT scream. bare ran the mutator
  # (`instpkg install nginx`); the apply's guard short-circuited it after running its check
  # (`dpkg-query nginx`) — an apply-only line the `guardcmd dpkg-query` allowlist admits (direction
  # i), and the guard ledger entry licenses the bare-only mutator (direction ii, 23A §5). BOTH
  # widened directions must stay silent here. Together with cf-5 (an UNRELATED apply-only line
  # STILL screams) this pins the widening as DISCRIMINATION, not a blanket whitelist.
  _r=$(dual_rail_judge "instpkg install nginx" "dpkg-query nginx" "argv 2 guard instpkg install nginx" "$_st_shims" "dpkg-query")
  case $_r in "") ;; *) _fails="${_fails}cf-6 (a guard's own suppressed mutator + check-command was wrongly screamed: $_r)
" ;; esac
  if [ -n "$_fails" ]; then
    echo "FATAL  dual_rail_selftest FAILED — the cm-1 judge does not scream as required; aborting:" >&2
    printf '%s' "$_fails" | sed 's/^/  /' >&2
    exit 3
  fi
}

# guard_shape_violations — the PURE artifact-shape judge (no I/O, no case dir; 23C-fd4). The
# XFAIL/XPASS/bless path never reads the golden TEXT, so rul-ternary-verdict's artifact-shape
# law (the two nevers, bytes-survive-verbatim) has no teeth on the promotion path unless a GATE
# asserts it. This is that gate's logic, factored out so guard_shape_selftest can drive it on
# fabricated strings (a shape floor that does not actually scream is worse than none). $1 = the
# apply artifact text, $2 = the book text. Echoes one line per violation (empty ⇒ pass). It keys
# on the `dorc: guard` disposition comment the guard tier postfixes to a guarded line (jc-guard-
# comment); INERT until such lines appear, so HEAD (no guards) is unaffected. For each guarded
# line it asserts the two shape laws:
#   never-1 (no engine-synthesized sh): the code carries a `<check> || <original>` fall-through;
#   bytes-verbatim: the text after the FIRST ` || ` is byte-identical (modulo surrounding
#     whitespace) to an ORIGINAL command line in book.sh — a dropped flag (`-y`) or any other
#     mutation of the original command's bytes fails this.
guard_shape_violations() {
  _gsv_art=$1; _gsv_book=$2
  _gsv_booklines=$(printf '%s\n' "$_gsv_book" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//')
  _gsv_out=""
  _gsv_oldifs=$IFS; IFS='
'
  for _gsv_l in $(printf '%s\n' "$_gsv_art" | grep -F 'dorc: guard' || true); do
    [ -z "$_gsv_l" ] && continue
    # Isolate the code: strip the trailing `# dorc: guard …` disposition comment.
    _gsv_code=$(printf '%s' "$_gsv_l" | sed -E 's/[[:space:]]*#[[:space:]]*dorc: guard.*$//')
    case $_gsv_code in
      *" || "*) ;;
      *) _gsv_out="${_gsv_out}thin guard (no '|| <original>' fall-through — never-1: engine-synthesized sh in guard position): $_gsv_code
"; continue ;;
    esac
    _gsv_orig=${_gsv_code#* || }                        # text after the FIRST ` || `
    _gsv_origtrim=$(printf '%s' "$_gsv_orig" | sed -E 's/^[[:space:]]+//; s/[[:space:]]+$//')
    printf '%s\n' "$_gsv_booklines" | grep -qxF "$_gsv_origtrim" \
      || _gsv_out="${_gsv_out}fall-through bytes not verbatim from book.sh (mutated original — e.g. a dropped flag): $_gsv_orig
"
  done
  IFS=$_gsv_oldifs
  printf '%s' "$_gsv_out"
}

# guard_shape_check (the per-case driver for the shape floor): run guard_shape_violations on the
# case's apply artifact ($2) + book ($3's book.sh) and report. Returns non-zero on any violation.
# LOUD REGARDLESS OF XFAIL (it does NOT consult XFAIL_ACTIVE): a malformed guard is a disaster
# whether or not the case is still pinned, so it must never hide as `xfail`. INERT when the apply
# carries no guard line (HEAD).
guard_shape_check() {
  _case=$1; _art=$2; _dir=$3
  _gsc_book=$(cat "${_dir}book.sh" 2>/dev/null || true)
  _gsc_viol=$(guard_shape_violations "$_art" "$_gsc_book")
  [ -z "$_gsc_viol" ] && return 0
  echo "FAIL  $_case  [guard-shape: a guarded line violates rul-ternary-verdict's artifact-shape law (never-1 / bytes-verbatim); the shape floor screams even under XFAIL — 23C-fd4]"
  printf '%s' "$_gsc_viol" | sed 's/^/      /'
  return 1
}

# guard_shape_selftest (the shape-floor confound battery, run ONCE at harness start alongside
# dual_rail_selftest; 23C-fd4/fd5). Drives guard_shape_violations on the two demonstrated
# violation shapes plus a pass control — a floor that does not scream on these is worse than
# none, so a self-test failure ABORTS (exit 3). These are exactly 23C-fd4's fake-engine
# artifacts: an engine-synthesized thin guard, and a fall-through with mutated original bytes.
guard_shape_selftest() {
  _fails=""
  _gss_book='apt-get install -y nginx
apt-get install -y curl'
  # gf-PASS (negative control): a well-formed guard (`<check> || <verbatim original>`) ⇒ must
  # NOT scream (proves gf-1/gf-2's screams are real discrimination, not a floor that rejects all).
  _r=$(guard_shape_violations 'apt_get__predict install -y curl || apt-get install -y curl   # dorc: guard [package converged-vouch; probe: holds]' "$_gss_book")
  case $_r in "") ;; *) _fails="${_fails}gf-PASS (a well-formed guard was wrongly flagged: $_r)
" ;; esac
  # gf-1: an engine-synthesized THIN guard (no `|| <original>` fall-through) ⇒ must scream (never-1).
  _r=$(guard_shape_violations 'dorc_guard curl   # dorc: guard [synthesized — no oracle body]' "$_gss_book")
  case $_r in *thin*) ;; *) _fails="${_fails}gf-1 (an engine-synthesized thin guard was not caught)
" ;; esac
  # gf-2: a fall-through whose original bytes were MUTATED (`-y` dropped) ⇒ must scream (bytes-verbatim).
  _r=$(guard_shape_violations 'apt_get__predict install -y curl || apt-get install curl   # dorc: guard [package converged-vouch; probe: holds]' "$_gss_book")
  case $_r in *verbatim*) ;; *) _fails="${_fails}gf-2 (a mutated fall-through — dropped -y — was not caught)
" ;; esac
  if [ -n "$_fails" ]; then
    echo "FATAL  guard_shape_selftest FAILED — the artifact-shape floor does not scream as required; aborting:" >&2
    printf '%s' "$_fails" | sed 's/^/  /' >&2
    exit 3
  fi
}

# dorc_flags_selftest (Stage 2 — the DORC_FLAGS plumbing confound, run ONCE at harness start):
# the flag threads through ONE shared `$@`, so a MISMATCH between gate invocations is structurally
# impossible; the remaining failure mode is the flag being silently DROPPED (never reaching the
# engine), which would make a survival case's apply match its unflagged baseline while gate-6
# still attributed a licensed delta — the attribution would lie. This directly simulates the
# mismatch: run the flagship WITH and WITHOUT `--trust-footprints` and assert the elision count
# DIFFERS (flagged elides past the wall, plain does not). If they match, the flag is inert ⇒ abort.
dorc_flags_selftest() {
  # Anchor retargeted survive-simple → survive-multiwall (2026-07-10): survive-simple became a
  # respell XFAIL specimen (24P §9) the engine can't parse yet, zeroing both counts ⇒ FATAL.
  _c="$here/cases/strawman24-survive-multiwall"
  [ -d "$_c" ] || return 0   # the flagship anchors the self-test; skip if the corpus lacks it
  _fl=$("$dorc" --book="$_c/book.sh" -o "$_c/package.oracle.sh" --trust-footprints \
    < "$_c/probe-results.txt" 2>&1 >/dev/null | grep -oE 'elide=[0-9]+' || true)
  _pl=$("$dorc" --book="$_c/book.sh" -o "$_c/package.oracle.sh" \
    < "$_c/probe-results.txt" 2>&1 >/dev/null | grep -oE 'elide=[0-9]+' || true)
  if [ "$_fl" = "$_pl" ]; then
    echo "FATAL  dorc_flags_selftest FAILED — --trust-footprints did not change the flagship's elision count ($_fl flagged vs $_pl plain); the flag is not reaching the engine, so a flagged survival case's gate-6 attribution would lie. aborting." >&2
    exit 3
  fi
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

# gate-7 (why-lens emission, 22D stage-3 / #16, x2-fd1): if a case ships an `expected-why` file,
# assert each of its (substring) patterns appears in some `why:` stderr line — pinning that the
# user-visible why-lens disclosure actually REACHES stderr end-to-end for a real book (the XC-3
# gap: the render logic was unit-pinned, the cli EMISSION was not). Substring-matched (grep -F)
# like gate-3, so a pattern need not carry the why line's volatile byte-offsets. A case without
# the file is unaffected (the why-lens is additive — most cases emit why lines, unasserted).
scan_why() {
  _case=$1; _err=$2; _dir=$3
  _decl="${_dir}expected-why"
  { [ -f "$_decl" ] && [ -s "$_decl" ]; } || return 0   # opt-in
  _whys=$(grep -E '^why: ' "$_err" 2>/dev/null || true)
  _missing=
  while IFS= read -r _pat; do
    [ -z "$_pat" ] && continue
    # A pattern may CONJOIN needles with ` && ` (23C-fd13): all parts must occur in a SINGLE
    # why-line, not spread across lines — so an attribution line naming the mechanism, the
    # license, AND the oracle together is required, and three unrelated notes can no longer
    # each satisfy one loose needle to XPASS the disclosure floor. A plain pattern (no ` && `)
    # keeps the original per-set substring match. Implemented as a progressive `grep -F` filter:
    # start from all why-lines, narrow by each needle in turn; a non-empty residue ⇒ some one
    # line carried every needle (order-independent).
    case $_pat in
      *" && "*)
        _cand=$_whys
        _rest=$_pat
        while [ -n "$_rest" ]; do
          _needle=${_rest%%" && "*}
          case $_rest in *" && "*) _rest=${_rest#*" && "} ;; *) _rest= ;; esac
          [ -z "$_needle" ] && continue
          _cand=$(printf '%s\n' "$_cand" | grep -F -- "$_needle" || true)
          [ -z "$_cand" ] && break
        done
        [ -n "$_cand" ] || _missing="${_missing}${_pat}
"
        ;;
      *)
        printf '%s\n' "$_whys" | grep -qF -- "$_pat" || _missing="${_missing}${_pat}
"
        ;;
    esac
  done < "$_decl"
  [ -z "$_missing" ] && return 0
  if [ "${XFAIL_ACTIVE:-}" != "1" ]; then
    echo "FAIL  $_case  [gate-7: expected why-lens line(s) not emitted on stderr — fix the cause, or update expected-why]"
    printf '%s' "$_missing" | sed 's/^/      missing: /'
  fi
  return 1
}

# dorc_sh_smoke (E1, 27D rider-dorc-sh-unbuilt — the strip-and-exec off-ramp; run ONCE at harness
# start, FATAL on failure). Proves the stamped `#!/usr/bin/env dorc-sh` shebangs are no longer inert:
#   (1) `dorc strip` on a marked corpus oracle is $checker -n clean AND dialect-free (no dorc-sh
#       shebang, no trailing `: sm.` mark, no `invariant:` bare-mark left as a stray `:`);
#   (2) `dorc-sh` strips-and-execs a marked script, erasing the bind, producing the expected output.
# The smoke script is BUILTIN-ONLY (`printf` + a function call) — it invokes NO external command, so
# nothing mutating can run (the inert-mocks safety intent) regardless of PATH; the exec still runs
# under the determinism rail (`env -i`), with PATH pinned to the real interpreter dir so dorc-sh can
# locate `sh`. dorc-sh is invoked by ABSOLUTE path (env -i scrubs PATH; a relative path would break).
dorc_sh_smoke() {
  _dsh=${DORC_SH:-}
  if [ -z "$_dsh" ]; then
    _ddir=$(dirname -- "$dorc")
    for _cand in "$_ddir/dorc-sh" "$_ddir/dorc-sh.exe"; do
      [ -x "$_cand" ] && { _dsh=$_cand; break; }
    done
  fi
  if [ -z "$_dsh" ] || [ ! -x "$_dsh" ]; then
    echo "FATAL  dorc_sh_smoke: dorc-sh binary not found next to dorc ($dorc) — build the workspace; aborting" >&2
    exit 3
  fi
  # (1) strip a marked corpus oracle → -n clean + dialect-free.
  _oracle="$here/cases/strawman24-alias-provides/package.oracle.sh"
  if [ -f "$_oracle" ]; then
    _stripped=$("$dorc" strip "$_oracle")
    if ! printf '%s\n' "$_stripped" | "$checker" -n 2>/dev/null; then
      echo "FATAL  dorc_sh_smoke: 'dorc strip' output is not $checker -n clean; aborting" >&2
      exit 3
    fi
    case "$_stripped" in
      *"env dorc-sh"*|*": sm."*|*"invariant:"*)
        echo "FATAL  dorc_sh_smoke: 'dorc strip' left a dialect construct (dorc-sh shebang / mark / bare-mark); aborting" >&2
        exit 3 ;;
    esac
  fi
  # (2) dorc-sh strips-and-execs a marked script (bind erased) → expected output, exit 0.
  _shdir=$(dirname -- "$(command -v sh 2>/dev/null || echo /bin/sh)")
  _ssand=$(mktemp -d)
  cat > "$_ssand/marked.sh" <<'SMK'
#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
smoke__predict() {
   pkg : sm.dorc.Package = "$1"
   printf 'dorc-sh-smoke ran: %s\n' "$pkg"
}
smoke__predict nginx
SMK
  _rc=0
  _out=$( cd -- "$_ssand" && umask 022 && env -i PATH="$_shdir" LC_ALL=C TZ=UTC "$_dsh" "$_ssand/marked.sh" 2>&1 ) || _rc=$?
  rm -rf "$_ssand"
  if [ "$_rc" -ne 0 ]; then
    echo "FATAL  dorc_sh_smoke: dorc-sh exited $_rc on a marked script (expected 0); aborting" >&2
    exit 3
  fi
  case "$_out" in
    *"dorc-sh-smoke ran: nginx"*) ;;
    *) echo "FATAL  dorc_sh_smoke: the stripped body did not run as expected (got: $_out); aborting" >&2; exit 3 ;;
  esac
}

# gate-6 self-test (the confound battery) runs ONCE here, before any case — a lying judge is
# worse than no judge, so this aborts (exit 3) if the dual-rail judge fails to scream. The
# guard-shape floor's confound battery (23C-fd4) runs alongside it for the same reason.
dual_rail_selftest
guard_shape_selftest
dorc_flags_selftest
dorc_sh_smoke

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

  # DORC_FLAGS marker (the value-in-filename idiom, like EXIT_RC=<n>): opt a case into extra
  # engine flags, appended to the SAME `$@` every dorc invocation below shares — the main
  # round-trip, gate-1's probe run (via the pre-rendered artifact), gate-5's `--debug-argv`, and
  # BOTH gate-6 invocations. Single-source threading makes a flag MISMATCH between invocations
  # structurally impossible (all read one `$@`), which is load-bearing: a mismatch would make
  # gate-6's attribution lie. Stage 2's `DORC_FLAGS=--trust-footprints` is the sole in-corpus use.
  # Refuse >1 marker loudly (an ambiguous flag-set is a fixture error). Self-checked below.
  _dorc_flags_count=0
  for _m in "${dir}"DORC_FLAGS=*; do
    [ -e "$_m" ] || continue
    _dorc_flags_count=$((_dorc_flags_count + 1))
    set -- "$@" "${_m##*DORC_FLAGS=}"
  done
  if [ "$_dorc_flags_count" -gt 1 ]; then
    echo "FAIL  $name  [DORC_FLAGS: multiple markers — exactly one flag-set is permitted]"
    fails=$((fails + 1))
    continue
  fi

  # DORC_EXIT=<n> marker (ack-1 exit-code family; the value-in-filename idiom, like EXIT_RC=<n>
  # but for the dorc PROCESS itself, not the rendered apply artifact's exec): the expected exit
  # status of the dorc invocation (default 0). A parse-error / unmodeled-book case fast-fails
  # with the dorc-semantic parse-error code (10) while STILL emitting its (partial) artifact
  # byte-identically (the stdout fence) — so the crash-guard must tolerate that DECLARED code
  # and proceed to the ordinary gates + content diff, not treat it as a dead engine. Refuse >1
  # marker loudly (an ambiguous expected-exit is a fixture error).
  _dorc_exit=0
  _dorc_exit_count=0
  for _m in "${dir}"DORC_EXIT=*; do
    [ -e "$_m" ] || continue
    _dorc_exit_count=$((_dorc_exit_count + 1))
    _dorc_exit=${_m##*DORC_EXIT=}
  done
  if [ "$_dorc_exit_count" -gt 1 ]; then
    echo "FAIL  $name  [DORC_EXIT: multiple markers — exactly one expected process-exit is permitted]"
    fails=$((fails + 1))
    continue
  fi

  # dorc's stdout is the artifact (probe + apply); its stderr is the diagnostic stream
  # (gate-3 asserts it — below). Capture BOTH (stderr to a temp file, no longer
  # discarded). dorc's exit status is captured (NOT piped away): a crashed/empty engine
  # must hard-fail every case BEFORE the xfail lens and BEFORE bless — empty artifacts
  # are `dash -n`-clean and a BLESS run would otherwise silently bless 43 empty goldens
  # (round-20 harness-crosscheck find-3, demonstrated with a crash-stub). The rc is checked
  # against the case's DECLARED DORC_EXIT (default 0): a WRONG code (incl. a 0 when nonzero
  # was declared — the fast-fail stopped firing) fails just like a nonzero-when-0-expected.
  dorc_rc=0
  err_file=$(mktemp)
  # Per-case per-run PATH shim dir (`274` §5): `--shim-dir` makes the round-trip ALSO write the
  # entry-composed probe's inner-check executables here (pure side effect; empty for a wrapper-free
  # case; probe_exec_check adds it to PATH; cleaned up at case end).
  _shimdir=$(mktemp -d)
  raw=$("$dorc" --shim-dir="$_shimdir" --book="${dir}book.sh" "$@" < "${dir}probe-results.txt" 2>"$err_file") || dorc_rc=$?
  got=$(printf '%s\n' "$raw" | sed 's/\r$//')
  if [ "$dorc_rc" -ne "$_dorc_exit" ] || [ -z "$got" ]; then
    echo "FAIL  $name  [dorc exited rc=$dorc_rc (expected $_dorc_exit) / produced no output — a dead engine, or a wrong exit-code contract, is never green]"
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
  # guard_shape_bad (23C-fd4): a malformed guard artifact — RED even under XFAIL (below).
  # head_ran_drifted (23B-fd1): an XFAIL case's current run-set diverged from its pinned HEAD
  # signature — consulted only while case_ok=0 (a disaster hiding as an ordinary xfail).
  guard_shape_bad=0
  head_ran_drifted=0

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
    probe_exec_check "$probe_art" "$name" "$dir" "$_shimdir" || case_ok=0
    # gate-5 (cm-2 argv-echo differential): cross-check the engine's per-site resolved
    # argv against the bare book's executed argvs under dash. Conservative, one-directional
    # (engine-resolved-and-shimmed ⊆ logged). Pass the space-delimited shim set + the
    # `-o oracle …` args. Not run under BLESS (it asserts, never re-authors).
    if [ "${BLESS:-}" != "1" ]; then
      _shimset=" $(cd "${dir}mocks" && ls | tr '\n' ' ')"
      argv_echo_check "$name" "$dir" "$_shimset" "$@" || case_ok=0
      # gate-6 (cm-1 dual-rail): bare-vs-apply run-set delta ⊆ the replace/omit license ledger.
      # EXCLUDES (a) PROBE_RESULTS=authored — its mock-rc and authored probe-results diverge, so a
      # bare-vs-apply difference is expected; (b) DUAL_RAIL=inlined — a function-inlined/wrapper-
      # pun case whose `--debug-argv` ledger reports the CALL-site surface argv (`apt_install
      # nginx`) while the bare run logs the inlined-body resolved argv (`apt-get install -y
      # nginx`); and (c) DUAL_RAIL=multiline-argv — a case with a literal-NEWLINE arg, where the
      # engine's `--debug-argv` shows only the arg's FIRST line (`… multi`) while the slice-4 bare
      # log encodes the whole arg (`… multi\nline`); the two representations cannot match. Both
      # DUAL_RAIL values are ledger-vs-bare argv-representation mismatches gate-6's matching cannot
      # reconcile (arch-2, note 216 inv-leaf-seam). See the gate-6 header. Not run under BLESS.
      if [ ! -f "${dir}PROBE_RESULTS=authored" ] && [ ! -f "${dir}DUAL_RAIL=inlined" ] \
         && [ ! -f "${dir}DUAL_RAIL=multiline-argv" ]; then
        dual_rail_check "$name" "$dir" "$_shimset" "$@" || case_ok=0
      fi
    fi
  fi

  # gate-3 (stderr-severity floor): an undeclared error-severity diagnostic fails the
  # case (declare legitimate ones in expected-diagnostics). Always run — analysis-only
  # cases (no mocks/) emit diagnostics too (⊤-rejects, missing-probe).
  scan_diagnostics "$name" "$err_file" "$dir" || case_ok=0
  # gate-7 (why-lens emission): opt-in expected-why substring assertion (#16, x2-fd1).
  scan_why "$name" "$err_file" "$dir" || case_ok=0

  # guard-shape floor (23C-fd4): assert every guarded line in the apply obeys the artifact-shape
  # law (never-1 + bytes-verbatim). INERT until guards appear; a violation fails the case AND is
  # loud even under XFAIL (guard_shape_bad ⇒ RED, not silent `xfail`).
  guard_shape_check "$name" "$apply_art" "$dir" || { case_ok=0; guard_shape_bad=1; }

  # head-expected.ran two-sided pin (23B-fd1): for an XFAIL case carrying the marker, has the
  # CURRENT run-set drifted from the pinned HEAD signature? (Only heeded while case_ok=0 — a
  # drift with case_ok=1 is the designed behaviour landing, i.e. a legitimate XPASS.)
  if [ -n "$xfail_reason" ]; then
    head_ran_check "$name" "$dir" "$apply_art" || head_ran_drifted=1
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

  # Interpret case_ok through the XFAIL lens. A malformed-guard shape violation
  # (guard_shape_bad) is a disaster REGARDLESS of xfail status, so it pre-empts the lens: the
  # guard_shape_check already printed its FAIL, we only count it. Otherwise, for an XFAIL case,
  # a head-expected.ran DRIFT while case_ok=0 is a disaster-shaped change hiding as an ordinary
  # xfail — RED, not a quiet `xfail` (23B-fd1/23C-fd4).
  if [ -n "$xfail_reason" ]; then
    if [ "$case_ok" -eq 1 ]; then
      echo "XPASS $name  [known defect appears FIXED — promote this case: $xfail_reason]"
      fails=$((fails + 1))
    elif [ "$guard_shape_bad" -eq 1 ]; then
      # A guarded artifact appeared but is MALFORMED — never a quiet xfail (floor already printed).
      fails=$((fails + 1))
    elif [ "$head_ran_drifted" -eq 1 ]; then
      echo "FAIL  $name  [head-expected.ran: current run-set drifted from the pinned HEAD signature while still XFAIL — a disaster-shaped behaviour change is hiding as an ordinary xfail (two-sided pin, 23B-fd1/23C-fd4)]"
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
    # DORC_E2E_QUIET=1 suppresses the per-case `ok` lines (~1/case ⇒ ~200/double-run
    # of conductor input-tokens before every commit; TODO.md). FAILURES, xfail/XPASS,
    # and the final tally still print unconditionally; exit semantics are unchanged.
    [ "${DORC_E2E_QUIET:-}" = "1" ] || echo "ok    $name"
  else
    fails=$((fails + 1))
  fi
  rm -f "$err_file"
  rm -rf "$_shimdir"
done

# ---------------------------------------------------------------------------
# `dorc lint` cases (27R) — a STRICTLY ADDITIVE second corpus (lint-cases/*/), disjoint from the
# round-trip cases/*/ above (cli/CLAUDE.md harness contract: a NEW case type, existing cases and
# their gates untouched). Each case dir carries: `cmd` (the lint flags, one line), `book.sh` (the
# file to lint), `expected.out` (hand-authored — NEVER blessed), and `expected-rc`. Each is run as
# `dorc lint <flags> book.sh` from INSIDE the case dir, so the finding path is the stable RELATIVE
# `book.sh`; and under a SCRUBBED PATH (a fresh empty dir) so shellcheck/checkbashisms are
# deterministically ABSENT regardless of the host — the spike never spawns real external tools in
# tests (the adapter parse ladder is unit-tested over a fake runner; tc-lint-e2e-stub-tools-spawn).
if [ -d "$here/lint-cases" ]; then
  _lint_empty=$(mktemp -d)
  for ldir in "$here"/lint-cases/*/; do
    [ -f "${ldir}cmd" ] || continue
    lname=lint/$(basename "$ldir")
    total=$((total + 1))
    _lflags=$(cat "${ldir}cmd")
    _lwant_rc=$(cat "${ldir}expected-rc" 2>/dev/null || echo 0)
    _lrc=0
    # shellcheck disable=SC2086  # $_lflags is intentionally word-split into separate flags.
    _lgot=$( cd -- "$ldir" && env PATH="$_lint_empty" "$dorc" lint $_lflags book.sh 2>/dev/null ) || _lrc=$?
    _lgot=$(printf '%s\n' "$_lgot" | sed 's/\r$//')
    _lwant=$(sed 's/\r$//' < "${ldir}expected.out")
    if [ "$_lrc" != "$_lwant_rc" ]; then
      echo "FAIL  $lname  [lint: exit rc=$_lrc, expected $_lwant_rc]"
      fails=$((fails + 1))
    elif [ "$_lgot" != "$_lwant" ]; then
      echo "FAIL  $lname  [lint: stdout content diff]"
      command -v diff >/dev/null 2>&1 && printf '%s\n' "$_lgot" | diff -u "${ldir}expected.out" - || true
      fails=$((fails + 1))
    else
      [ "${DORC_E2E_QUIET:-}" = "1" ] || echo "ok    $lname"
    fi
  done
  rm -rf "$_lint_empty"
fi

echo "---"
if [ "$fails" -ne 0 ]; then
  echo "$fails/$total e2e round-trips FAILED" >&2
  exit 1
elif [ "${BLESS:-}" = "1" ]; then
  echo "blessed $total cases (all ap-2 gates passed)"
else
  echo "all $total e2e round-trips passed (ap-2 $checker -n + apply/probe exec gates, redirect sandbox, ordered run-set, stderr floor, argv-echo differential, dual-rail license judge, why-lens emission)"
fi
