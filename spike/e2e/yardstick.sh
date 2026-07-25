#!/bin/sh
# yardstick.sh — the round-24 measurement instrument (plans/240 Stage-1). For each
# cases/strawman24-*/ it invokes dorc EXACTLY as the e2e runner does, parses the plan-summary line dorc
# emits on stderr, and prints a fixed-width per-case table plus a family aggregate: the round's
# north-star metric — ELISION FREQUENCY on the strawman family — turned into a number that every
# later stage must move, visibly, from the CLI.
#
# THIS SCRIPT ONLY MEASURES — it runs no gate. Correctness of these cases is the e2e runner's job: the
# strawman24-* cases ride the same e2e harness as every other case, and its exec-differential
# (gate-6 dual-rail + the exec/ran gates) is what proves each elision safe. A number here is
# meaningless unless `cargo test -p dorc-cli --test e2e` is green (or the case is a correctly-pinned XFAIL) — measure
# AFTER the differential passes. An XFAIL case's row is its HEAD (defect-present) reading, so it
# can move when the pinned defect is fixed (e.g. strawman24-modeled-wall's elide drops 1->0 the
# moment the silence=wall fix lands — the yardstick makes that fix visible as a metric change).
#
# elide-fr = elide / sites (the golden-hill Replace verb's frequency); `omit` (fold-dead
# branches) and `guard` (0 until the Stage-3 guard tier) are shown but NOT folded into it.
#
# Usage:  sh yardstick.sh            # auto-locates target/{debug,release}/dorc[.exe]
#         DORC=/path/to/dorc sh yardstick.sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

# Binary locator — the SAME logic the retired sh harness used (this script re-implements no
# gate). $DORC overrides.
dorc=${DORC:-}
if [ -z "$dorc" ]; then
  for cand in \
    "$here/../target/debug/dorc" "$here/../target/debug/dorc.exe" \
    "$here/../target/release/dorc" "$here/../target/release/dorc.exe"; do
    if [ -x "$cand" ]; then dorc=$cand; break; fi
  done
fi
if [ -z "$dorc" ] || [ ! -x "$dorc" ]; then
  echo "dorc binary not found — build it first (cargo build, from spike/), or pass DORC=" >&2
  exit 2
fi

# Pull one integer field out of a captured plan-summary line. $1=line $2=key. The leading space
# anchors the key so `run` never matches inside another token.
field() { printf '%s\n' "$1" | sed -n "s/.* $2=\\([0-9][0-9]*\\).*/\\1/p"; }

# elide / sites as `D.DD` via integer math (no bc under PATH-limited shells).
frac2() { # $1=numerator $2=denominator
  if [ "$2" -le 0 ]; then printf '   -  '; return; fi
  _fr=$(( $1 * 100 / $2 ))
  printf '%d.%02d' "$(( _fr / 100 ))" "$(( _fr % 100 ))"
}

row() { printf '%-32s %6s %6s %5s %6s %5s %9s\n' "$1" "$2" "$3" "$4" "$5" "$6" "$7"; }
rule='-------------------------------- ------ ------ ----- ------ ----- ---------'

row case sites elide omit guard run elide-fr
echo "$rule"

t_sites=0; t_elide=0; t_omit=0; t_guard=0; t_run=0; n=0; missing=0
for dir in "$here"/../crates/cli/tests/strawman24-*/; do
  [ -d "$dir" ] || { echo "yardstick: no strawman24-* cases under crates/cli/tests — the collection moved" >&2; exit 2; }
  name=$(basename "$dir")
  n=$((n + 1))
  # Collect -o oracles (glob-sorted, exactly as the e2e runner assembles them) + the DORC_FLAGS marker
  # (so a `--trust-footprints` case's survivals show in the yardstick — the number Stage 2 moves).
  set --
  for o in "$dir"*.oracle.sh; do [ -e "$o" ] || continue; set -- "$@" -o "$o"; done
  for m in "$dir"DORC_FLAGS=*; do [ -e "$m" ] || continue; set -- "$@" "${m##*DORC_FLAGS=}"; done
  # Invoke as the e2e runner does: book + oracles, probe-results on stdin; the summary is on stderr.
  summary=$("$dorc" --book="${dir}book.sh" "$@" < "${dir}probe-results.txt" 2>&1 >/dev/null \
    | grep '^dorc: plan-summary ' || true)
  case $summary in
    "dorc: plan-summary sites="*" elide="*" omit="*" guard="*" run="*) ;;
    *) row "$name" NO-SUMMARY - - - - -; missing=$((missing + 1)); continue ;;
  esac
  s=$(field "$summary" sites); e=$(field "$summary" elide); o=$(field "$summary" omit)
  g=$(field "$summary" guard); r=$(field "$summary" run)
  row "$name" "$s" "$e" "$o" "$g" "$r" "$(frac2 "$e" "$s")"
  t_sites=$((t_sites + s)); t_elide=$((t_elide + e)); t_omit=$((t_omit + o))
  t_guard=$((t_guard + g)); t_run=$((t_run + r))
done

echo "$rule"
row "FAMILY ($n cases)" "$t_sites" "$t_elide" "$t_omit" "$t_guard" "$t_run" "$(frac2 "$t_elide" "$t_sites")"

if [ "$missing" -ne 0 ]; then
  echo "yardstick: $missing strawman case(s) emitted no parseable plan-summary" >&2
  exit 1
fi
