# exec-errexit-top-status-runs (19A C-3 / 205 §2 — the FLIP of the old
# exec-errexit-elide-vouched): under `set -e` every command's rc is consumed (non-zero
# ⇒ abort), so an errexit-region command is an ordinary status consumer — NOT
# special-cased-as-vouched. The engine marks it the value-relaxable `AndOrStatus`.
# Composed with `fork-mutator-rc` (a mutator's rc has no sanctioned source ⇒ ⊤), the
# converged `nginx` install can no longer be elided to `true`: a fabricated rc-0 would
# hide the abort a NON-conforming converged establish (one that exits non-zero when
# converged) raises under `set -e`. So it RUNS. The `curl` install is diverged ⇒ runs
# anyway. Both run — the round's headline cost (a known/probe-sourced rc would still
# fold, but a mutator's rc is never known).
set -e
apt-get install -y nginx
apt-get install -y curl
