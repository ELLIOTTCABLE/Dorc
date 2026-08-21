#!/bin/sh
# THE MOTIVATING SHAPE (`plans/30L` §1): the whole book wrapped in one function, with one live
# command in the middle. Under the all-or-nothing CALL license a single unmodeled `hork` forfeited
# the ENTIRE body; region decisions forfeit only the region that failed.
#
# What this pins: three different answers inside ONE definition, decided independently.
#   nginx  converged, nothing before it            => Replace, at the authored region
#   hork   unmodeled                               => Run (it is the wall)
#   curl   converged, but past hork's running wall => Guard (re-decides live; rul-ternary-verdict)
# and `main "$@"` stays a CALL, untouched (`30L:rul-edit-authored-definition-once`).
main() {
   apt-get install -y nginx
   hork tune-packages
   apt-get install -y curl
}

main "$@"
