# strawman24-incoherent-refused (plans/240 Stage 2 — the coherence check, should-have): a wall
# whose touches() footprint OMITS its own establish coordinate (at-least ⊄ at-most) is a loud
# contradiction ⇒ the footprint is REFUSED ⇒ the wall stands total.
#   site 0  apt-get install oldpkg — DIVERGED wall. Its touches() (STRAWMAN) emits package:wrongpkg
#           instead of package:oldpkg — it establishes package:oldpkg#installed but claims to touch
#           only package:wrongpkg. Coherence fails ⇒ a `footprint-incoherent` warning + the
#           footprint is refused ⇒ total wall.
#   site 1  apt-get install nginx  — CONVERGED, but the refused footprint means oldpkg is a total
#           wall ⇒ nginx DEMOTES (runs). Both lines run; the coherence check caught the bad footprint.
apt-get install -y oldpkg
apt-get install -y nginx
