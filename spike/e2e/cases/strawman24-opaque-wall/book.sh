# strawman24-opaque-wall (plans/240 Stage-1 — the poison wall, correct AT HEAD). An un-oracled
# command mid-book:
#   site 0  apt-get install nginx — converged, BEFORE the wall ⇒ elides (as at HEAD).
#   site 1  hork wombat — un-oracled ⇒ Opaque ⇒ runs verbatim, and stands as the poison wall.
#   site 2  apt-get install curl — converged, but PAST the opaque wall ⇒ unresolvable ⇒ RUNS.
# The safe floor (a fully-unmodeled command poisons everything downstream) already works at
# HEAD; contrast strawman24-modeled-wall, where a modeled-but-partial wall under-poisons.
apt-get install -y nginx
hork wombat
apt-get install -y curl
