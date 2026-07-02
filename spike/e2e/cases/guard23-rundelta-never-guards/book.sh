# guard23-rundelta-never-guards (rul-guard-license: "Run-delta verbs never guard (an oracle
# declines by not vouching)" — a PASSING floor). `systemctl restart nginx` is the canonical
# run-delta command: its VALUE IS THE RUN (the delta it applies), not a state its oracle
# could re-derive — a state-guard here is precisely the forbidden wrong-skip (a "restart if
# not active" guard EATS a restart the book demanded; plans/233 downsides list). The
# service oracle below models (and vouches) `enable` — and stays SILENT on restart: no
# effect row, no vouched path for restart argv to reach, so no witness ever forms and the
# site runs, forever, vouch-or-no-vouch. Tripwire against provider-keyed vouch scoping
# ("some systemctl path is vouched, so systemctl restart may guard" is the claim-noop
# conflation one storey up).
systemctl restart nginx
