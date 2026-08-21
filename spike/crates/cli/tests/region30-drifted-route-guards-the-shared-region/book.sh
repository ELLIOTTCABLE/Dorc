#!/bin/sh
# THE DIVERGENT-INSTANCES VALVE (`30L` §4.5). nginx is installed; curl is not. One authored region
# serves both, so no universal Replace exists — and Guard absorbs exactly what Replace cannot,
# because the runtime dispatch happens per invocation, inside sh, authored.
#
# What this pins, and it is the whole point: the guard's argv is the SOURCE-level expression
# `install -y "$1"`, never either site's resolved operands. A per-call literal guard would install
# into shared source that also serves the other operand, which is the specialized shell
# `30L:rul-no-specialized-shell` forbids. Every enumerated route's own argv passed the oracle
# author's argparse when its vouch was reached, and the census is CLOSED — so every value `$1` can
# hold at runtime is one that author already accepted.
install_pkg() {
   apt-get install -y "$1"
}

install_pkg nginx
install_pkg curl
