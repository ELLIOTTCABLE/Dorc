#!/usr/bin/env dorc-sh
# dorc-lang/v0.1
# firewall oracle (ufw). `ufw allow <rule>` establishes firewall:<rule>#allowed.
# Probe parses `ufw status` (needs root; an unprivileged probe must read UNKNOWN, not
# absent — q-probe-privilege, not handled in this scrappy fixture). Rule-equivalence
# (80/tcp vs 80 vs ranges) is unsound in sh (15x ufw HOLE) — exact-string only here.
# command-keyed predict(): `ufw <verb> <rule>` — bind the verb, annotate the rule operand
# as `firewall` (exact-string only; rule-equivalence is unsound in sh, 15x HOLE).
ufw__predict() {
   verb=$1; shift
   rule : sm.dorc.Firewall = "$1"
   case $verb in
      allow) ufw status "$rule" >/dev/null 2>&1 : sm.dorc.Firewall:"$rule".allowed ;;
      deny) ufw status "$rule" >/dev/null 2>&1 : sm.dorc.Firewall:"$rule".allowed! ;;
   esac
}
