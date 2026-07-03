# strawman24-modeled-wall (plans/240 Stage-1 — the fd10 "dangerous middle"; XFAIL until the
# sibling silence=wall fix lands). Two modeled installs on DIFFERENT cells:
#   site 0  apt-get install oldpkg — DIVERGED (absent) ⇒ RUNS. A modeled mutator that runs IS
#           the wall: whatever it did beyond its declared cell (package:oldpkg#installed) is
#           unknown (the frame problem, 233), so every downstream converged site must RUN.
#   site 1  apt-get install nginx — CONVERGED (holds) on cell package:nginx#installed, PAST the
#           running oldpkg wall ⇒ it must RUN (silence=wall, 23O §2). At HEAD it WRONGLY ELIDES:
#           the modeled-but-partial oldpkg poisons only its own cell, so nginx stays "ambient"
#           and elides — the under-execution fd10 hole the sibling fix closes.
# Goldens = the DESIGNED post-fix run-set (both RUN); head-expected.ran pins the HEAD signature.
apt-get install -y oldpkg
apt-get install -y nginx
