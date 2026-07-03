# strawman24-survive-killwall (plans/240 Stage 2): kill-UNIFORMITY — a running footprinted KILL
# scopes its wall exactly like a footprinted establish does.
#   site 0  apt-get purge oldpkg — a KILL (classifies MustRun ⇒ always RUNS = a wall); its
#           touches() emits package:oldpkg, so the wall is SCOPED, not total.
#   site 1  apt-get install nginx — CONVERGED, backing package:nginx disjoint from the kill's
#           footprint package:oldpkg (different entity) ⇒ SURVIVES the running kill ⇒ ELIDES.
#           Proves kills participate in the survival walk uniformly with establishes.
apt-get purge -y oldpkg
apt-get install -y nginx
