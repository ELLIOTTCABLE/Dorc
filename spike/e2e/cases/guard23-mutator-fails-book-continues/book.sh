# guard23-mutator-fails-book-continues (rul-divergence-proceed: the engine adds no second-
# guess layer above guards; XFAIL until the guard tier lands). Drift case where the guard
# falls through AND the real mutator then FAILS (apt-get mock exits 1). This book has no
# errexit, so bare sh would carry on past the failure — and the guarded artifact must be
# byte-faithful to that: check fails -> mutator runs -> mutator fails -> the book CONTINUES
# (marker runs; artifact exits 0, the last command's rc). No engine wrapper may swallow,
# retry, abort-on, or "helpfully" report-and-stop the mutator's failure in-line: failure
# surfacing is the oracle's/report's job on its own channels, and divergence handling is
# proceed-and-flag. (The errexit-book variant of this pin is deliberately NOT authored:
# whether errexit-implicit consumption blocks guarding at all is DEFERRED by human ruling,
# 2026-07-02 — see Research/notes/23A.)
hork wombat
apt-get install -y nginx
marker done
