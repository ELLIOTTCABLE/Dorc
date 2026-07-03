# strawman24-nonsurvive-hit (plans/240 Stage 2): the honest NON-disjoint direction — a footprint
# that INTERSECTS the downstream backing demotes EVEN flagged (entity-granular hit).
#   site 0  apt-get install oldpkg — DIVERGED wall. Its touches() over-claims (STRAWMAN): it emits
#           package:oldpkg (its own) AND package:nginx — "installing oldpkg also touched nginx".
#   site 1  apt-get install nginx  — CONVERGED, EstablishAmbient (oldpkg's cell ≠ nginx's). Under
#           --trust-footprints the survival test runs, but backing package:nginx HITS the wall's
#           footprint (same kind, same entity) ⇒ NOT disjoint ⇒ DEMOTES (runs). The flag is on and
#           the wall runs, yet the elision is correctly refused — disjointness is the gate, not the
#           flag. Both lines run.
apt-get install -y oldpkg
apt-get install -y nginx
