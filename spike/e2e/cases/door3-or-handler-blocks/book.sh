# door3-or-handler-blocks (door-3 NEGATIVE pole, charter 20V §4 / note 213): `cmd || { …; }`
# is NOT the bare-`true` shape — the rhs is a `Group` whose continuations can DIFFER observably
# (the handler prints/recovers). So the install's rc stays the BLOCKING `StatusRelaxable`, NOT
# door-3's `StatusInvariant`. Composed with ⊤ (`fork-mutator-rc`), the converged install is
# REFUSED a license ⇒ it RUNS, and the handler runs on failure. This is the pole the charter
# demands: only the admin's explicit `|| true` opt-out is door-3; a real handler is load-bearing.
set -e
apt-get install -y nginx || { printf 'recovering\n'; }
