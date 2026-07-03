# tiedown-survive-statebearing (24B §3 — a flavour-C tie-down for the round-24 chronology net).
# Same survival shape as strawman24-survive-simple, but the mocks are STATE-BEARING: dpkg-query
# reads a per-run package DB (./state) seeded from S0 (nginx installed, oldpkg not), and apt-get
# writes it. So gate-1 validates the probe against a FAITHFUL file-backed mini-world (not a
# hardcoded rc table), and gate-6 cross-checks the survival elision's run-set against real dash —
# tying the in-memory hostsim model (set-membership + apply_delta) to reality on a real chronology.
#   site 0  apt-get install oldpkg — DIVERGED (absent in the DB) ⇒ RUNS; footprint package:oldpkg.
#   site 1  apt-get install nginx — CONVERGED (present in the DB), PAST the running oldpkg wall.
#           Backing package:nginx is DISJOINT from footprint package:oldpkg ⇒ under
#           --trust-footprints it ELIDES (the frame rule). gate-6 attributes the elided install
#           to its replace license; the probe is mock-reproducible FROM STATE ⇒ NOT authored.
apt-get install -y oldpkg
apt-get install -y nginx
