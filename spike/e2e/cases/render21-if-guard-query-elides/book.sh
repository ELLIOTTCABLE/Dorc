# render21-if-guard-query-elides (arch-1, note 214 — the if-guard elision POLE A): an `if`
# guard that is a converged, KNOWN-rc read-only Query now ELIDES, because the leaf-exact
# render can substitute a guard's byte-span in-situ (`if ! true; then`) — the round-21
# `StatusRenderFloor` block (which floored every if/elif guard) is retired, and the guard
# joins `StatusRelaxable` (a known rc reproduces its branch decision).
#
# HOST: nginx installed (the guard holds, rc 0). The guard `dpkg -s nginx` is a VALID Query
# (only `set -e`, target-state-pure, is upstream) ⇒ it value-substitutes to its probed rc
# `true`; the `!` makes the if-condition FALSE ⇒ the fold proves the then-body install DEAD
# ⇒ it is omitted (substituted `:`, its controller — the substituted guard — is neutralised).
# The whole construct collapses to `if ! true; then :; fi`, dash-clean, EMPTY run-set. This is
# the if-guard analogue of the `||`-guard fold (exec-shimmed-query-fold), unlocked by arch-1.
# (Contrast the ⊤-rc anti-pole render21-if-guard-toprc-runs: a mutator-as-guard RUNS.)
set -e
if ! dpkg -s nginx >/dev/null 2>&1
then
   apt-get install -y nginx
fi
