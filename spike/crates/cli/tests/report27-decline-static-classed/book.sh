#!/bin/sh
# report27-decline-static-classed (`27W` tier-2): the book statically calls a sysctl write-only
# trigger key. The oracle's is_converged declines it with a `decline unsound …` emission on the
# reached arm; the static argv threads to that arm, so the class + emitting-arm file:line surface
# at PLAN time (the tier-2 `authored_reason`). No vouch ⇒ the site RUNS.
sysctl vm.drop_caches
