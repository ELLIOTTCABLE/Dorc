#!/bin/sh
# THE MEMBER BINDING'S REFUSAL (`30L` section 7). The member pass answers a spliced body site by
# OVERRIDING the iteration variable in that site's incoming state, so a book that REBINDS it makes
# the override a lie — and the lie is a wrong-elision, not a lost one.
#
# CFG shape: a top-level `for` over two literal words; its body assigns the iteration variable and
# then holds one call; the spliced body holds one command leaf reading that variable.
#
# What this observes: the plan RUNS `apt-get install -y wombat`, once per iteration. Without the
# refusal the member pass answers this site with `nginx` and `curl`, the probe measures those two
# cells, both come back converged, and the region replaces a command that installs something else
# entirely. The run-set is the whole assertion: the operand it names is the one the shell binds,
# and it is neither member.
install_pkg() {
   apt-get install -y "$pkg"
}

for pkg in nginx curl; do
   pkg=wombat
   install_pkg
done
