# strawman24-survive-multiwall (plans/240 Stage 2): a fact surviving TWO footprinted walls, then
# a FOOTPRINT-LESS wall collapsing everything below it (total wall unchanged by the flag).
#   site 0  apt-get install oldpkg  — diverged wall 1, footprint package:oldpkg
#   site 1  apt-get install badpkg  — diverged wall 2, footprint package:badpkg
#   site 2  apt-get install nginx   — CONVERGED, backing package:nginx disjoint from BOTH walls
#           (different entities, entity-granular) ⇒ SURVIVES both ⇒ ELIDES. Witness = 2 crossings.
#   site 3  apt-get purge gonepkg   — a running KILL whose touches() has NO purge arm ⇒
#           FOOTPRINT-LESS ⇒ a TOTAL wall (silence=wall, flag-independent).
#   site 4  apt-get install curl    — CONVERGED, but PAST the footprint-less total wall ⇒ DEMOTES
#           (runs). Proves the flag buys back only scoped walls; a silent mutator still totalises.
apt-get install -y oldpkg
apt-get install -y badpkg
apt-get install -y nginx
apt-get purge -y gonepkg
apt-get install -y curl
