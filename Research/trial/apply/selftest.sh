#!/usr/bin/env sh
#
# dorc-r25 apply-run selftest (P3) — dry-tests the capture flow with zero ssh.
# Every case runs a plan through `apply-run --local` and asserts the run-dir
# artifacts + the C-run/1 JSON. Exits non-zero if any assertion fails.
#
# This validates the RUNNER (rc/stdout/stderr separation, JSON, timeout), NOT any
# real remote apply — the live ssh integration is deferred (needs a real box).

set -u

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
RUNNER="$SCRIPT_DIR/apply-run.sh"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/apply-selftest-XXXXXX")"
trap 'rm -rf "$WORK"' EXIT INT TERM

PASS=0; FAIL=0
check(){ # check <desc> <actual> <expected>
   if [ "$2" = "$3" ]; then PASS=$((PASS+1)); printf '  ok   %s\n' "$1"
   else FAIL=$((FAIL+1)); printf '  FAIL %s\n     expected: [%s]\n     actual:   [%s]\n' "$1" "$3" "$2"; fi
}

echo "P3 apply-run selftest"
echo "runner: $RUNNER"

# --- case 1: rc=0, stdout only, stderr empty --------------------------------
echo "[case 1] rc=0, stdout-only"
cat >"$WORK/p1.sh" <<'PLAN'
printf 'line-one\nline-two\n'
PLAN
d="$WORK/c1"
sh "$RUNNER" apply-run --local --out "$d" "$WORK/p1.sh" >"$d.summary" 2>/dev/null || true
check "rc"            "$(cat "$d/rc")"                    "0"
check "stdout"        "$(cat "$d/stdout")"                "$(printf 'line-one\nline-two')"
check "stderr-empty"  "$(wc -c <"$d/stderr" | tr -d ' ')" "0"
check "meta rc"       "$(jq -r .rc "$d/meta.json")"       "0"
check "summary==meta" "$(cat "$d.summary")"               "$(cat "$d/meta.json")"

# --- case 2: rc=3, stdout AND stderr, cleanly separated ---------------------
echo "[case 2] rc=3, stdout+stderr separated"
cat >"$WORK/p2.sh" <<'PLAN'
printf 'OUT-a\nOUT-b\n'
printf 'ERR-x\nERR-y\n' >&2
exit 3
PLAN
d="$WORK/c2"
sh "$RUNNER" apply-run --local --out "$d" "$WORK/p2.sh" >"$d.summary" 2>/dev/null || true
check "rc"           "$(cat "$d/rc")"              "3"
check "stdout"       "$(cat "$d/stdout")"          "$(printf 'OUT-a\nOUT-b')"
check "stderr"       "$(cat "$d/stderr")"          "$(printf 'ERR-x\nERR-y')"
check "meta rc"      "$(jq -r .rc "$d/meta.json")" "3"
check "transp false" "$(jq -r .transport_failed "$d/meta.json")" "false"

# --- case 3: local exit 255 must NOT be flagged transport_failed ------------
# (255 is an ssh-only transport signal; in local mode it is just the plan's rc.)
echo "[case 3] local exit 255 is a plan rc, not a transport failure"
cat >"$WORK/p3.sh" <<'PLAN'
exit 255
PLAN
d="$WORK/c3"
sh "$RUNNER" apply-run --local --out "$d" "$WORK/p3.sh" >"$d.summary" 2>/dev/null || true
check "rc"           "$(cat "$d/rc")"                            "255"
check "transp false" "$(jq -r .transport_failed "$d/meta.json")" "false"
check "transp_err"   "$(jq -r .transport_error "$d/meta.json")"  "null"

# --- case 4: plan delivered on stdin ('-') ----------------------------------
echo "[case 4] plan via stdin ('-')"
cat >"$WORK/p4.sh" <<'PLAN'
printf 'from-stdin\n'
exit 7
PLAN
d="$WORK/c4"
sh "$RUNNER" apply-run --local --out "$d" - <"$WORK/p4.sh" >"$d.summary" 2>/dev/null || true
check "rc"       "$(cat "$d/rc")"                "7"
check "stdout"   "$(cat "$d/stdout")"            "from-stdin"
check "plan tag" "$(jq -r .plan "$d/meta.json")" "<stdin>"

# --- case 5: empty stdout AND empty stderr, rc=0 ----------------------------
echo "[case 5] no observable output, rc=0"
cat >"$WORK/p5.sh" <<'PLAN'
: does nothing observable
PLAN
d="$WORK/c5"
sh "$RUNNER" apply-run --local --out "$d" "$WORK/p5.sh" >"$d.summary" 2>/dev/null || true
check "rc"           "$(cat "$d/rc")"                    "0"
check "stdout-empty" "$(wc -c <"$d/stdout" | tr -d ' ')" "0"
check "stderr-empty" "$(wc -c <"$d/stderr" | tr -d ' ')" "0"
check "meta valid"   "$(jq -e . "$d/meta.json" >/dev/null 2>&1 && echo ok)" "ok"

# --- case 6: apply timeout fires (needs timeout/gtimeout) -------------------
echo "[case 6] APPLY_TIMEOUT fires -> timed_out=true"
if command -v timeout >/dev/null 2>&1 || command -v gtimeout >/dev/null 2>&1; then
   cat >"$WORK/p6.sh" <<'PLAN'
sleep 5
PLAN
   d="$WORK/c6"
   APPLY_TIMEOUT=1 sh "$RUNNER" apply-run --local --out "$d" "$WORK/p6.sh" >"$d.summary" 2>/dev/null || true
   check "timed_out" "$(jq -r .timed_out "$d/meta.json")" "true"
   rc6="$(cat "$d/rc")"
   check "rc 124or137" "$([ "$rc6" = 124 ] || [ "$rc6" = 137 ] && echo yes)" "yes"
else
   echo "  skip (no timeout/gtimeout on PATH)"
fi

echo "----------------------------------------"
printf 'RESULT: %d passed, %d failed\n' "$PASS" "$FAIL"
[ "$FAIL" -eq 0 ] || exit 1
