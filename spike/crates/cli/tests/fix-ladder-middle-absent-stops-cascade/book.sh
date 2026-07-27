# fix-ladder-middle-absent-stops-cascade (`26H` §4 W-C, the boundary): the ladder with the MIDDLE
# guard reporting absent. Rung 1 folds and its install is erased; rung 2's install is LIVE, so it
# still invalidates below it. Rung 3's guard measured holds (rc 0, recorded) and would substitute
# to `true` if the cascade reached it — it renders VERBATIM instead, which is the boundary: the
# cascade must stop at the live mutator, not run past it.
set -e
dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha
dpkg -s beta >/dev/null 2>&1 || apt-get install -y beta
dpkg -s gamma >/dev/null 2>&1
