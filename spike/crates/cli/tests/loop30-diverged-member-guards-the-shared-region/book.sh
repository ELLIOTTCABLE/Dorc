#!/bin/sh
# THE DIVERGENT-MEMBERS VALVE (`30L` sections 4.5 and 7) — the member cousin of
# `region30-drifted-route-guards-the-shared-region`, where the two disagreeing routes were two
# CALLS. Here they are two ITERATIONS of one call, which is the whole reason the iteration axis
# exists: nothing in the CFG tells them apart.
#
# CFG shape: a top-level `for` over two literal words; its body holds one call; the spliced body
# holds one command leaf. One region, one lowered `cfg_node`, two member routes overlaid on it.
#
# nginx is installed; curl is not. No universal Replace exists, so Guard absorbs it — and the
# guard's argv is the SOURCE-level `install -y "$1"`, never either member's resolved operand
# (`30L:rul-no-specialized-shell`). That is load-bearing here in a way it is not for twin calls:
# `$1` re-binds on every iteration of the loop, so a member-0 literal baked into shared source
# would check nginx while curl installed.
install_pkg() {
   apt-get install -y "$1"
}

for pkg in nginx curl; do
   install_pkg "$pkg"
done
