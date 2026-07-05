# strawman24-alias-provides (24F Stage 5 — the aliasing-closure flagship). The wall and the victim
# name ONE referent by TWO names — a provides/virtual alias (`nginx` vs `nginx-full`). token-equality
# calls `package:nginx` and `package:nginx-full` DISJOINT, so without the closure the converged victim
# would WRONGLY SURVIVE the running wall (a silent under-execute — the one cell that fails UNSAFE, 24F
# §1). The `package` kind's owner ships a `package.resolve()` (dpkg-query provides-resolution): the
# engine canonicalizes BOTH the footprint coord and the backing coord through it before `disjoint`.
#   site 0  apt-get install nginx       — DIVERGED (absent) ⇒ RUNS. The footprinted wall (package:nginx).
#   site 1  apt-get install nginx-full  — CONVERGED (holds), PAST the running wall. Its backing
#           (package:nginx-full) canonicalizes to `nginx`; the wall footprint (package:nginx)
#           canonicalizes to `nginx` too ⇒ a canonical HIT ⇒ the closure correctly DEMOTES the victim
#           to RUN (it re-installs the shared referent the wall disrupted). Under token-equality it
#           would have elided (wrong-survival). The differential: expected.ran runs BOTH; the closure
#           closed the under-execute. may-alias=0 (a proven HIT, not a resolver gap).
apt-get install nginx
apt-get install nginx-full
