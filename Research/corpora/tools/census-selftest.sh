#!/bin/sh
# ============================================================================
# LLM-GENERATED. Part of an intentionally quality-varied artificial testing
# corpus/tooling for a static-analysis project (Dorc). NOT production tooling.
# An artificial corpus cannot expose the truth of real-world ops-code.
#
# census-selftest.sh -- runs census.sh over census-fixture.sh and diffs the
# result against expected counts derived BY HAND from the fixture. Exits nonzero
# on any mismatch. This script IS meant to be run: it only READS the fixture and
# runs the project's own awk via census.sh. It executes nothing from any corpus.
#
# Discipline: the EXPECT_* tables below were written by hand while authoring the
# fixture; if the tool and a table disagree, the bug is in the tool (unless an
# expectation was arithmetically wrong, in which case fix it and say so in the
# method note). Do not "make the test pass" by copying tool output blindly.
# ============================================================================

set -eu

here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
census="$here/census.sh"
fixture="$here/census-fixture.sh"

[ -f "$census" ]  || { echo "selftest: missing $census" >&2; exit 2; }
[ -f "$fixture" ] || { echo "selftest: missing $fixture" >&2; exit 2; }

work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

sh "$census" -o "$work" "$fixture" >/dev/null 2>&1

# ----------------------------------------------------------------------------
# Expected CONSTRUCT counts (construct<TAB>count). Hand-derived from the fixture.
# Every construct class is listed; zeros are findings too. Keep tab-separated.
# ----------------------------------------------------------------------------
expected_constructs() {
   cat <<'EOF'
! negation	1
$(..) cmdsub	3
&& and-list	1
|| or-list	1
append >>	1
arith $((..))	1
backtick cmdsub	1
brace group { }	1
case	1
command -v	1
env-prefix assign	1
eval	0
exec	0
fd-dup (N>&M)	1
for	1
func call	3
func def	3
glob char (unquoted)	1
heredoc <<-	1
heredoc quoted	1
heredoc unquoted	2
if/elif	3
local	1
param $!	1
param $#	1
param $$	1
param $*	1
param $?	1
param $@	1
param ${..#prefix}	1
param ${..%suffix}	1
param ${..:=assign}	1
param ${..:-default}	1
param ${..} braced	5
param $VAR plain	6
param positional $1..$9	3
pipe stage	3
plain assign	7
redirect <	1
redirect >	4
redirect to-null	3
set -e	0
set -eu	1
set -u	0
subshell (..)	1
test [ ] file	2
test [ ] numeric	1
test [ ] string	1
tilde candidate	1
trap	1
until	1
while	1
EOF
}

# ----------------------------------------------------------------------------
# Expected COMMAND counts (command<TAB>class<TAB>count). Hand-derived.
#   external: hork 19, wombat 14
#   builtin : echo 11, [ 4, true 4, : 1, break 1, command 1, false 1,
#             local 1, set 1, trap 1   (= 26)
#   function: alpha 1, beta 1, gamma 1
# ----------------------------------------------------------------------------
expected_commands() {
   cat <<'EOF'
:	builtin	1
[	builtin	4
alpha	function	1
beta	function	1
break	builtin	1
command	builtin	1
echo	builtin	11
false	builtin	1
gamma	function	1
hork	external	19
local	builtin	1
set	builtin	1
trap	builtin	1
true	builtin	4
wombat	external	14
EOF
}

# Aggregate expected class totals and grand total, for a second cross-check.
EXP_EXTERNAL=33
EXP_BUILTIN=26
EXP_FUNCTION=3
EXP_TOTAL_TOKENS=62

fail=0

# ---- compare constructs (temp files, no process substitution -> dash-clean) ---
expected_constructs | sort > "$work/exp_constructs.txt"
awk -F'\t' 'NR>1 {printf "%s\t%s\n", $2, $3}' "$work/constructs.tsv" | sort > "$work/got_constructs.txt"
if ! cmp -s "$work/exp_constructs.txt" "$work/got_constructs.txt"; then
   echo "FAIL: construct counts differ (< expected | > got):" >&2
   diff "$work/exp_constructs.txt" "$work/got_constructs.txt" >&2 || true
   fail=1
else
   echo "ok: all construct counts match"
fi

# ---- compare commands ----
expected_commands | sort > "$work/exp_commands.txt"
awk -F'\t' 'NR>1 {printf "%s\t%s\t%s\n", $2, $3, $4}' "$work/commands.tsv" | sort > "$work/got_commands.txt"
if ! cmp -s "$work/exp_commands.txt" "$work/got_commands.txt"; then
   echo "FAIL: command counts differ (< expected | > got):" >&2
   diff "$work/exp_commands.txt" "$work/got_commands.txt" >&2 || true
   fail=1
else
   echo "ok: all command counts match"
fi

# ---- cross-check class totals from summary-derivable data ----
ext=$(awk -F'\t' 'NR>1 && $3=="external"{s+=$4} END{print s+0}' "$work/commands.tsv")
bui=$(awk -F'\t' 'NR>1 && $3=="builtin"{s+=$4}  END{print s+0}' "$work/commands.tsv")
fun=$(awk -F'\t' 'NR>1 && $3=="function"{s+=$4} END{print s+0}' "$work/commands.tsv")
tot=$(awk -F'\t' 'NR>1 {s+=$4} END{print s+0}' "$work/commands.tsv")
check_total() {
   name=$1; got=$2; exp=$3
   if [ "$got" -ne "$exp" ]; then
      echo "FAIL: $name total = $got, expected $exp" >&2
      fail=1
   else
      echo "ok: $name total = $got"
   fi
}
check_total external "$ext" "$EXP_EXTERNAL"
check_total builtin  "$bui" "$EXP_BUILTIN"
check_total function "$fun" "$EXP_FUNCTION"
check_total "command-token" "$tot" "$EXP_TOTAL_TOKENS"

if [ "$fail" -ne 0 ]; then
   echo "SELFTEST: FAIL" >&2
   exit 1
fi
echo "SELFTEST: PASS"
