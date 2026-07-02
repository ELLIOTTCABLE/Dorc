# guard23-canttell-plan-runs (ruling h1 RATIFIED — converged-only mint: a guard mints only where
# the plan-time probe verdict is CONVERGED; can't-tell ⇒ run — a PASSING floor). A vouched site
# past a wall whose PLAN-TIME probe verdict is CANT-TELL:
#   apt-get install -y nginx  — converged before the wall ⇒ elides
#   hork wombat               — opaque wall
#   apt-get install -y curl   — vouched + past wall, but probe = CANT-TELL ⇒ NO witness (the mint
#                               is converged-only) ⇒ runs BARE
# A cant-tell mint would guard a line the approved plan displayed as RUN (a front-load-doctrine
# breach — the decision made post-approval) even though runtime-safe-ish; h1 forecloses it. The
# pin: curl runs bare, forever. If a build ever mints a guard on a cant-tell (rc 2+) verdict —
# treating "unsure" as "converged" — an extra `dpkg-query -W curl` check appears in the run-set
# (or the install is suppressed) and this floor reds. PROBE_RESULTS=authored: the cant-tell is a
# PLAN-TIME authored verdict (the past-wall establish-probe never ships at HEAD to reproduce it),
# exactly the drift-trio pattern; gate-1 parity + gate-6 are off, exec_check carries the pin.
apt-get install -y nginx
hork wombat
apt-get install -y curl
