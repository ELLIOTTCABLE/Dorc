# strawman24-mixed-real (plans/240 Stage-1 — the "looks like a real book" composite; the family's
# most honest single number). Five lines exercising the whole gradient at once:
#   site 0/1  dpkg -s nginx || apt-get install nginx — the idempotency query-guard idiom (the
#            shimmable `dpkg -s` form, mirroring exec-query-guard-composition's shape but kept
#            gate-6-differential-active). nginx installed ⇒ the guard (site 0) holds ⇒ it value-
#            substitutes to `true` (elide) and the `|| install nginx` (site 1) is fold-dead (omit).
#   site 2   apt-get install curl — converged ⇒ elides.
#   site 3   apt-get install oldpkg — DIVERGED ⇒ runs.
#   site 4   hork wombat — un-oracled ⇒ Opaque ⇒ runs (a wall).
#   site 5   apt-get install vim — past the hork wall ⇒ runs.
# The converged sites sit BEFORE the wall (else fd10 would bite — see strawman24-modeled-wall).
# NB: no `set -e` — errexit forces converged MUTATORS to run (206 §2 cost), which would mask the
# curl elision; that interaction is its own axis (see the yardstick strain ledger).
dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx
apt-get install -y curl
apt-get install -y oldpkg
hork wombat
apt-get install -y vim
