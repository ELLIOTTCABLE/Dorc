# door1-cascade-diverged-runs (door-1 POLE, charter 20V §4 / note 215): the SAME book as
# door1-cascade-block-elides, but the host is DIVERGED — nginx is absent (the guard fails,
# rc 1). The fold reads the guard's known rc 1 ⇒ the `||` FIRES ⇒ the `{ … }` block is
# LIVE ⇒ BOTH block commands run (the sed AND the systemctl restart). This is the pole that
# proves dead-branch elision is PROBE-KEYED, never structural: the identical `|| { … }`
# shape that folded whole in the converged case runs whole here, because the guard's
# probed rc differs. The restart is NOT vouched-for by any state-probe — it runs because
# the branch is reachable, exactly the R2-CHANGEDELTA distinction made visible.
#
# The guard substitutes to its exact stand-in `false` (rc 1, the probe-sourced Query
# value); the block is kept verbatim (its controller is live ⇒ the runtime guard gates it).
# Renders `false || { sed …; systemctl restart sshd; }`. HOST: nginx absent (guard fails).
set -e
dpkg -s nginx >/dev/null 2>&1 || { sed -i 's/^.*PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config; systemctl restart sshd; }
