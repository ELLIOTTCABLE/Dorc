#!/bin/sh
# NO-DEDUP, HONOURED CONSERVATIVELY. `for x in a a` is TWO evaluations — dash iterates the list,
# not a set — so the census mints two member routes and never one (`30N` section 2, the `20S`
# member commitments).
#
# CFG shape: a top-level `for` over two IDENTICAL literal words; its body holds one call; the
# spliced body holds one command leaf. One region, one lowered `cfg_node`, two member routes.
#
# Both members resolve `$1` to the same operand, so both establish ONE cell at ONE site. The
# aggregate identity is the exact ordered `(site, fact)` population and it rejects duplicates, so
# the whole region takes the run floor rather than erasing two executions against one vouched
# establish (`rul-every-erased-establish-is-vouched`: identity- AND cardinality-matched). This is
# a deliberate value forfeit in a shape that has no per-member variation to lose, and it is the
# safe direction — the alternative is a proof identity that cannot count its own executions.
install_pkg() {
   apt-get install -y "$1"
}

for pkg in nginx nginx; do
   install_pkg "$pkg"
done
