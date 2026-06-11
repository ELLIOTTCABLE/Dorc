# door1-cascade-block-elides (door-1 PAYOFF, charter 20V §4 / note 215): the
# probe-observed Query rc folds its guarded BLOCK whole. `dpkg -s nginx` is a read-only
# pkgstate Query whose OWN probed rc (0 = nginx installed) is fold-usable (rule-query-
# validity passes — NOTHING mutates upstream of the guard, only `set -e`, target-state-pure).
# The guard holds (rc 0) ⇒ the `|| { … }` branch is DEAD control-flow ⇒ the WHOLE brace
# group folds: the sed AND the systemctl restart elide as unreachable, each needing NO
# rc-provenance of its OWN (they are dead, not vouched-for). This is the Ansible
# handler/notify semantic falling out of plain control-flow analysis.
#
# CRITICAL distinction (TODO.md R2-CHANGEDELTA): this is NOT the run-delta/notify-handler
# class (restart-iff-changed, un-probeable, never elidable via a state-probe). Here the
# restart elides because the BRANCH IS DEAD — the guard proved it unreachable — NOT because
# any state-probe vouched for the restart. The diverged pole (door1-cascade-diverged-runs)
# makes that visible: guard fails ⇒ the WHOLE block runs, restart included.
#
# The guard substitutes to its exact stand-in `true` (rc 0, the probe-sourced Query value);
# the block leaves omit to `:` (the omit-safety gate — controller `dpkg -s` is neutralised).
# Renders `true || { :; :; }`, dash-clean, UNDER errexit, run-set EMPTY. HOST: nginx
# installed (the guard holds).
set -e
dpkg -s nginx >/dev/null 2>&1 || { sed -i 's/^.*PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config; systemctl restart sshd; }
