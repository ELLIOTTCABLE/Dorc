# Round-17 adversarial-crosscheck strawmen — runnable SHELL-HAZARD demos
# Verified live under /bin/dash + bash (2026-06-08). Each block is a regression-test seed; the
# behavior in comments is observed + reproducible. Full ledger: ../../17O-adversarial-crosscheck-findings.md
#
# These are the "our stdlib oracles / oracle-writing guidance must be GOOD, battle-tested sh"
# class (human, 2026-06-08) — quality hazards in probe sh, NOT flaws in Dorc's model.

set -u

echo "== R2-SHADOW: command -v is true for a shell FUNCTION (no binary) =="
( docker() { :; }
  if command -v docker >/dev/null 2>&1; then
     echo "HAZARD: command -v docker => true (shadowed by function; no binary) => would elide install"
  fi )
# The honest 'tool installed' probe must require an executable FILE on PATH:
tool_installed() { p=$(command -v "$1" 2>/dev/null) && case $p in /*) [ -x "$p" ] ;; *) false ;; esac; }
( docker() { :; }; tool_installed docker && echo "BUG" || echo "OK: tool_installed rejects the function shadow" )

echo "== R2-ORTRUE: '|| true' forces a lifted guard to always report holds =="
( svc_up() { systemctl is-active --quiet "$1" 2>/dev/null || true; }   # '|| true' = errexit-survival
  svc_up no-such-svc && echo "HAZARD: svc_up rc=0 => a lifter reads 'converged' though the service is down" )

echo "== R2-TRAP: a mutating EXIT/ERR trap fires when a shipped probe fails under set -e =="
out=$(dash -c 'set -e; trap "echo TRAP-MUTATION-FIRED" EXIT; probe(){ return 1; }; probe' 2>&1)
case $out in *TRAP-MUTATION-FIRED*) echo "HAZARD: probe failure ran the EXIT trap (a kFAIL-withhold breach)" ;; esac
# NOTE (human): the probe is built from ORACLE BODIES + minimal CFG, NOT the book's contents, so it
# should never INHERIT the book's traps. Regression goal: probe-construction never carries ambient traps.

echo "== F-OFFRAMP: the inline type-annotation is not inert under a real shell =="
# The bad spelling (cannot be live code — it does not parse/run):
#   frobctl.check() { local w : com.frobber.Wombat{defrocked,frocked} = "$1"; ... }
#   dash: aborts 'local: :: bad variable name';  bash: w left EMPTY, rc 0 (corruption);
#   dotted name frobctl.check() => dash -n 'Bad function name'.
# The off-ramp-CLEAN datum form (17N P1) — runs identically with/without Dorc:
oracle_kind=net.frobnitz.wombat
frobctl_check() { w="$1"; printf 'checking %s\n' "$w"; }
frobctl_check wom
