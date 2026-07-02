# guard23-nounset-book-survives (23C-fd2: a `set -u` book — the inserted guard must not kill
# the book tail; XFAIL until the guard tier lands). The corpus-standard check body reads `"$2"`
# unconditionally (`if [ "$2" = "" ]; …`); a single-operand guard invocation
# (`apt_get__check install -y curl`) leaves `$2` unset, so under `set -u` the shipped whole-body
# guard DIES rc 2 at that read — curl, vim, and everything downstream under-executed by the
# insertion (the 218a set-u hazard, demonstrated; distinct from the deferred errexit-CONSUMPTION
# axis — this is the inserted code's OWN crash). Composition:
#   set -u                    — nounset active for the whole book
#   hork wombat               — opaque wall
#   apt-get install -y curl   — vouched + converged-past-wall ⇒ GUARD (the crash site)
#   apt-get install -y vim    — DIVERGED (vim absent) ⇒ the downstream victim; must still run
# Desired (mechanism-NEUTRAL per human ruling h3 — check-body `${2:-}` hygiene OR subshell-wrap,
# the engine's choice): the guarded artifact COMPLETES exactly as the bare book does — curl
# guarded (converged ⇒ suppressed), vim installed, artifact exits 0. The bare book completes rc
# 0 (three commands); a naive guard dies rc 2 at the guard and under-executes the tail. The
# load-bearing pins are expected.ran (vim present) and the default exit-0 assertion (survives).
set -u
hork wombat
apt-get install -y curl
apt-get install -y vim
