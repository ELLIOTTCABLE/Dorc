#!/usr/bin/env dorc-sh
# dorc-lang/v0.2
#
# apt-get — an oracle that deliberately answers nothing, and says why. Both verbs this book uses
# are shapes a skeleton-tier oracle has no business vouching for; classing the two declines turns
# two anonymous "unmodeled command" walls into two attributed, explained ones.
#
# Only the verb-first spelling is recognised, which is the one the book writes. `apt-get -y install
# X` falls through to the silent decline — the same safe outcome, minus the explanation.

apt_get__is_converged() {
   case "${1-}" in
   update)
      # "Fresh enough" is a judgment about this machine and how much staleness its admin will
      # accept, not a fact about apt. Nobody has told this oracle what window to honour, so it
      # refuses to invent one and the refresh runs.
      printf 'decline unmodeled %s: index freshness is a time window nobody gave me\n' "$1" >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   install)
      # `apt-get install -y X` on an already-installed X still UPGRADES it whenever the index
      # offers a newer version, so "X is present" is emphatically not "this line is a no-op".
      # Vouching on presence alone would skip exactly the security updates the line exists to
      # pull; the admin's own `dpkg -s X ||` guard is where that judgment belongs.
      printf 'decline unmodeled %s: presence is not convergence, install still upgrades\n' "$1" >>"${DREP_V1:-/dev/null}"
      return 2 ;;
   *) return 2 ;;
   esac
}
