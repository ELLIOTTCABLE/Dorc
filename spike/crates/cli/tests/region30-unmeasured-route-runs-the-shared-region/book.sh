#!/bin/sh
# ONE route measured; the other could not be answered at all (a >=2 sink landing, so cant-tell).
# The shared region RUNS: `30L:rul-shared-region-needs-universal-must` demands every route hold at
# `Must`, and one route's convergence buys the region nothing.
#
# Nor does the guard valve reach it. A DIVERGED route is a measurement — the world answered "not
# yet" — so a converged sibling makes a shared check worth paying for. An UNKNOWN route is the
# ABSENCE of an answer, and paying a check to discover what the probe could not is the unsure
# direction (`inv-kfail`). The sibling's own CALL still elides on its own aggregate license: a
# different decision, about a different identity.
install_pkg() {
   apt-get install -y "$1"
}

install_pkg nginx
install_pkg curl
