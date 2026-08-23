#!/bin/sh
# TARGET (REDS sibling of `loop30-direct-and-called-mutators-share-a-loop-body`,
# `FORFEITS:forfeit-cell-blind-self-reach-walls-loop-siblings`): byte-identical CFG, oracles, mocks,
# and convergence facts to that green case — only this file's expected outcome differs.
#
# CFG shape: a top-level `for` over two literal words; its body holds one direct command leaf AND
# one call whose spliced body holds one command leaf (the region). All four cells converged; the two
# families are cell-disjoint (`lib*` versus bare).
#
# TODAY: the region's own per-member establishes reach the direct mutator back over the loop's edge,
# and self-reach (`Reach::is_pristine`) is CELL-BLIND — it cannot see that `lib$pkg` and `$pkg` never
# alias — so disjointness buys nothing and the direct mutator's running walls the region down to
# GUARD (`30Qa:fnd-self-reach-is-cell-blind-across-the-back-edge`).
#
# TARGET: a cell-AWARE self-reach (`30Qa:tc-self-reach-cell-blind-widening`, flagged not built) would
# see the direct mutator's establish and the region's read/establish as provably-disjoint cells, so
# the region's self-reach stays pristine and the shared `install_pkg` definition takes full elision
# (Replace) instead of Guard — observed as: no runtime check ships at apply-time, and the direct
# mutator is the ONLY thing in the run-set (`expected.ran`, this case).
install_pkg() {
   apt-get install -y "$1"
}

for pkg in nginx curl; do
   apt-get install -y "lib$pkg"
   install_pkg "$pkg"
done
