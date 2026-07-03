# strawman24-survive-simple (plans/240 Stage 2 — the golden hill flagship; INVERTS
# exec-modeled-wall-runs). Same shape as strawman24-modeled-wall, but the apt oracle now grows
# a touches() footprint AND the case carries DORC_FLAGS=--trust-footprints:
#   site 0  apt-get install oldpkg — DIVERGED (absent) ⇒ RUNS. A running modeled mutator IS a
#           wall — but now it declares its footprint (apt-get.touches() emits package:oldpkg).
#   site 1  apt-get install nginx — CONVERGED (holds) on package:nginx#installed, PAST the
#           running oldpkg wall. Its backing (package:nginx) is DISJOINT from the wall's
#           footprint (package:oldpkg — same kind, DIFFERENT entity, entity-granular) ⇒ under
#           --trust-footprints it ELIDES past the running wall (the frame rule). The yardstick
#           goes 1->2 here vs the honest baseline; the why-lens names the licensor (site 0,
#           apt-get, package:oldpkg). The UNFLAGGED sibling asserts the byte-identical baseline
#           (both run). Differential: exec_check + expected.ran pin the run-set; gate-6 attributes
#           the elided install to its replace license (probe is mock-reproducible ⇒ NOT authored).
apt-get install -y oldpkg
apt-get install -y nginx
