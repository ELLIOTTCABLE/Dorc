# strawman24-modeled-wall (plans/240 Stage-1 — the fd10 "dangerous middle", CLOSED; born XFAIL,
# promoted 2026-07-03 when the plan-time wall landed). Two modeled installs on DIFFERENT cells:
#   site 0  apt-get install oldpkg — DIVERGED (absent) ⇒ RUNS. A modeled mutator that runs IS
#           the wall: whatever it did beyond its declared cell (package:oldpkg#installed) is
#           unknown (the frame problem, 233), so every downstream converged site must RUN.
#   site 1  apt-get install nginx — CONVERGED (holds) on cell package:nginx#installed, PAST the
#           running oldpkg wall ⇒ it RUNS (silence=wall, 23O §2; the build_plan wall walk).
#           Pre-fix HEAD wrongly ELIDED it — the modeled-but-partial oldpkg poisoned only its
#           own declared cell, leaving nginx "ambient" — the under-execution fd10 hole.
# The probe half still ships BOTH sites (the wall is plan-time; static classify untouched).
apt-get install -y oldpkg
apt-get install -y nginx
