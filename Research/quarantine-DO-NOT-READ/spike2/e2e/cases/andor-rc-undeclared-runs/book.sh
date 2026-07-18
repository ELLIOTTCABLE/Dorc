# andor-rc-undeclared-runs (round-19 `19D`, the DEFAULT-PATH safety sibling of
# andor-rc-vouch-wrong): the SAME non-conforming-establish `||` book, but the converged
# `user#present` fact carries NO declared rc (probe-results has no `rc=`). After 19D an
# un-injected rc is ⊤ (never a fabricated rc 0), and a `&&`/`||` left operand whose
# status is branch-consumed with an UNDECLARED rc is REFUSED a replace license ⇒ it
# RUNS. So the whole line stays verbatim and BOTH commands run: real `useradd deploy`
# exits 9 (user present) ⇒ `|| mkdir` fires ⇒ `mkdir` runs. This is the safe
# kFAIL-perform floor — WITHOUT 19D the fabricated rc 0 made the fold omit `mkdir` (a
# proven priority-1 under-execute). Contrast andor-rc-vouch-wrong (rc=9 DECLARED ⇒ the
# value-preserving `(exit 9)` keeps `mkdir` live via the same `||`).
useradd deploy || mkdir /srv/app
