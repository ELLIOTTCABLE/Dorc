# strawman24-survive-simple-unflagged (plans/240 Stage 2 / rul24-mode-gate): the UNFLAGGED
# sibling of strawman24-survive-simple. IDENTICAL book + oracle (touches() and all), but NO
# DORC_FLAGS marker. Without --trust-footprints the footprints are never even lifted (TC-1), so
# the survival tier is unreachable and the honest Stage-1 total wall stands: the running oldpkg
# install walls the nginx install, which RUNS. This is the byte-identical Stage-1 baseline the
# flag must never silently drop — asserting BOTH sides of the mode-gate (24A §1a-addendum).
apt-get install -y oldpkg
apt-get install -y nginx
