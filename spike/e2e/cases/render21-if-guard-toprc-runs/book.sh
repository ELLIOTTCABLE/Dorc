# render21-if-guard-toprc-runs (arch-1, note 214 — the if-guard elision ANTI-POLE B): an `if`
# guard whose rc is ⊤ RUNS, even converged. The guard `apt-get install -y nginx` is a MUTATOR
# (an establish), so under fork-mutator-rc its rc has no sanctioned source ⇒ ⊤. The if/elif
# guard now consumes `StatusRelaxable` (arch-1 — the render CAN substitute it in-situ), but
# `StatusRelaxable` + a ⊤ rc BLOCKS the license (a fabricated rc-0 `true` would change the
# branch decision — the `kFAIL-perform` floor). So the guard is NOT substituted and RUNS for
# real; the then-body runs after it. This is the pole that proves arch-1's if-guard unlock is
# VALUE-keyed (a known rc elides — render21-if-guard-query-elides; a ⊤ rc runs), not a blanket
# "guards now elide". HOST: nginx installed (the guard's effect is converged, but its rc is ⊤).
if apt-get install -y nginx
then
   echo started
fi
