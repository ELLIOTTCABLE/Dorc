#!/bin/sh
# TWO MECHANISMS, ONE LOOP BODY. A mutating command written DIRECTLY in the body and a CALL whose
# spliced body mutates are different syntactic shapes with different machinery, and this book puts
# them side by side so neither can quietly answer for the other.
#
# CFG shape: a top-level `for` over two literal words; its body holds one direct command leaf AND
# one call whose spliced body holds one command leaf. Two plan leaves; the first takes the r21
# per-member aggregate lane (its argv references the loop variable), the second takes the region
# lane (its region is the definition's body, and the member axis is an overlay on one node).
#
# The two `site N.M` sub-indices mean different populations at the two leaves — member index at
# leaf 0, member-major (body-site, member) at leaf 1 — and they must not collide or double-count.
#
# MEASURED, and what makes this case earn its keep: the direct mutator RUNS, although all four
# cells are converged and the two families are disjoint (`lib*` versus bare). The r21 lane's
# self-reach gate is `Reach::is_pristine` — "nothing at all reached me", which is cell-BLIND — and
# the region's own per-member establishes reach it back over the loop's edge, so disjointness buys
# nothing there. Its running establishes then wall the region, which takes the GUARD tier instead
# of Replace. The member axis still reaches a licence: the parametric check re-decides per
# iteration, inside sh, in the author's own bytes — and without the route-aware floor this region
# would simply run.
install_pkg() {
   apt-get install -y "$1"
}

for pkg in nginx curl; do
   apt-get install -y "lib$pkg"
   install_pkg "$pkg"
done
