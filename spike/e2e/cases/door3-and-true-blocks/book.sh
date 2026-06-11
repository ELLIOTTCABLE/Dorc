# door3-and-true-blocks (door-3 NEGATIVE pole, charter 20V §4 / note 213): door-3 is `||`-only.
# Under `cmd && true`, cmd's FAILURE short-circuits PAST `true` and the LIST rc is cmd's
# non-zero rc — which fires errexit (abort). So the continuations DIFFER (success ⇒ list rc 0,
# `true` runs; failure ⇒ list rc ≠ 0, abort), and the left rc is load-bearing. The install
# keeps the BLOCKING `StatusRelaxable` (the `&&`-left mark), not door-3's `StatusInvariant`.
# Composed with ⊤, the converged install is REFUSED ⇒ it RUNS (`&& true` is not door-3).
set -e
apt-get install -y nginx && true
