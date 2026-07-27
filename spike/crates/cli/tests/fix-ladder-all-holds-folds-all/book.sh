# fix-ladder-all-holds-folds-all (`26H` §4 W-C, the headline): an N=3 guarded-install ladder
# over three distinct entities, every guard holding. Pre-fixpoint exactly ONE rung folded —
# rung 1's install statically invalidated rung 2's guard, and the fold never re-ran to notice
# that install was itself proven dead. All three fold now; a silent disable shows up here as a
# non-empty run-set, because rungs 2 and 3 would render verbatim and their guards would run.
set -e
dpkg -s alpha >/dev/null 2>&1 || apt-get install -y alpha
dpkg -s beta >/dev/null 2>&1 || apt-get install -y beta
dpkg -s gamma >/dev/null 2>&1 || apt-get install -y gamma
