# door3-or-true-diverged-runs (door-3, charter 20V §4 / note 213): door-3 clears ONLY the
# Status channel; the Effect channel still gates. The `|| true` marks the install's rc
# `StatusInvariant` (never blocks), but the host is DIVERGED (nginx absent), so the
# convergence check refuses the license ⇒ the install RUNS for real. The `|| true` swallows
# its rc under `set -e` (the left of `||` is errexit-exempt), so a non-zero install does not
# abort the book. Proof that door-3 is NOT an elision-license relaxation: Effect still gates.
set -e
apt-get install -y nginx || true
