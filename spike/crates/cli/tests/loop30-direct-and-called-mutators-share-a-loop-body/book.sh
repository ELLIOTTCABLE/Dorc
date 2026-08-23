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
# The four cells are disjoint (`lib*` versus bare), so the direct mutator's own erasure is what
# clears the wall its establishes would otherwise cast over the call across the back-edge.
install_pkg() {
   apt-get install -y "$1"
}

for pkg in nginx curl; do
   apt-get install -y "lib$pkg"
   install_pkg "$pkg"
done
