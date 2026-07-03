# tiedown-killwall-statebearing (24B §3 — a flavour-C tie-down for the round-24 chronology net).
# Same kill-wall survival shape as strawman24-survive-killwall, but the mocks are STATE-BEARING:
# dpkg-query reads a per-run package DB (./state, seeded S0 = oldpkg + nginx installed) and apt-get
# writes it (purge really removes oldpkg). Ties the in-memory model's KILL-wall survival to real
# dash: gate-1 validates the nginx probe against the file-backed world, gate-6 cross-checks the
# elision run-set.
#   site 0  apt-get purge oldpkg — a KILL (classifies MustRun ⇒ always RUNS = a wall); its
#           touches() emits package:oldpkg, so the wall is SCOPED, not total.
#   site 1  apt-get install nginx — CONVERGED (present in the DB), backing package:nginx disjoint
#           from the kill's footprint package:oldpkg ⇒ SURVIVES the running kill ⇒ ELIDES.
apt-get purge -y oldpkg
apt-get install -y nginx
