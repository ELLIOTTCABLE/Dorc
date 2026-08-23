#!/bin/sh
# THE MEMBER POPULATION, CONVERGED (`30L` sections 4.5 and 7). One authored region, one authored
# call, TWO evaluations — because a syntactically singular call inside a literal `for` is many
# evaluations (`30L:rul-one-call-site-is-not-one-evaluation`).
#
# CFG shape: a top-level `for` over two literal words; its body holds one call; the spliced body
# holds one command leaf. One region, one lowered `cfg_node`, two member routes overlaid on it.
#
# What this pins: the member axis reaches the licence. Both members' cells are converged, so the
# universal meet over BOTH agrees on one observable-preserving replacement and the authored region
# is edited once, at the definition. A member-blind seat would answer one route twice and call that
# agreement universal without ever having asked the second member anything.
install_pkg() {
   apt-get install -y "$1"
}

for pkg in nginx curl; do
   install_pkg "$pkg"
done
