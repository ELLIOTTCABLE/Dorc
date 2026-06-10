# exec-shimmed-query-fold (task-P/item-3, 20I find-2): the `dpkg -s X || apt-get install X`
# idempotency idiom where the guard is an EXTERNAL, SHIMMABLE query (`dpkg -s`, the pkgstate
# oracle's `query` polarity) — NOT the un-shimmable `command -v` builtin of
# exec-query-guard-composition. Because `dpkg` is external, the rendered PROBE is faithfully
# mock-reproducible, so gate-1 ENFORCES parity here (no PROBE_RESULTS=authored opt-out): the
# fold-under-ALL-gates demonstration 20H §3 ran only in a discarded temp now lives in the corpus.
#
# HOST: nginx installed (the guard holds). The guard `dpkg -s nginx` is a VALID Query (only
# `set -e`, target-state-pure, is upstream) with a known rc 0; the fold reads it ⇒ the `||`
# branch is DEAD ⇒ the install is OMITTED, and the guard is value-substituted to its exact
# `true`. The whole line collapses to `true` under errexit; the run-set is empty.
set -e
dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx
