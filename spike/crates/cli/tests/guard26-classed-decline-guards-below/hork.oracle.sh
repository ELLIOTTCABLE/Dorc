# dorc-lang/v0.2
# An oracle that answers nothing and says so — the shape the contract asks an author to write when
# a command has no convergence criterion worth vouching (turning an anonymous wall into an
# attributed one). It vouches for nothing and licenses nothing.
#
# This file is the ONLY difference between this case and `guard26-unmodeled-wall-guards-below`,
# and adding it COSTS the two drops below their guards. See that case's book.sh.
hork__is_converged() {
   return 2
}
