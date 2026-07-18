# firewall oracle (ufw). `ufw allow <rule>` establishes firewall:<rule>#allowed.
# Probe parses `ufw status` (needs root; an unprivileged probe must read UNKNOWN, not
# absent — q-probe-privilege, not handled in this scrappy fixture). Rule-equivalence
# (80/tcp vs 80 vs ranges) is unsound in sh (15x ufw HOLE) — exact-string only here.
oracle_kind=firewall
oracle_probe_firewall() { ufw status 2>/dev/null | grep -q "$1"; }
oracle_effect ufw allow establish allowed
oracle_effect ufw deny kill allowed
