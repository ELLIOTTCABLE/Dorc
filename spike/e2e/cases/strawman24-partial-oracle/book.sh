# strawman24-partial-oracle (plans/240 Stage-1 — the oracle-QUALITY axis). SAME book bytes as
# strawman24-all-converged-clean, but the `service` oracle is deliberately WITHHELD from the -o
# set, so `systemctl enable nginx` is un-modeled ⇒ Opaque ⇒ an opaque wall:
#   site 0  apt-get install nginx — converged, pre-wall ⇒ elides.
#   site 1  systemctl enable nginx — no oracle ⇒ runs, and walls the rest.
#   site 2  apt-get install curl — converged, but past the wall ⇒ runs.
# Same bytes, worse oracle COVERAGE ⇒ fewer elisions (1, vs 3 for all-converged-clean). The
# yardstick turns the cost of a coverage gap into a number.
apt-get install -y nginx
systemctl enable nginx
apt-get install -y curl
