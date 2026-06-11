# door3-or-true-elides (door-3 PAYOFF, charter 20V §4 / notes 213+214): `cmd || true` under
# `set -e`, CONVERGED. The install's rc is consumed by `|| true` ⇒ `StatusInvariant` (never
# blocks), so the convergence-elision license MINTS even though the mutator's rc is ⊤
# (`fork-mutator-rc`). The mutator is REPLACED by its stand-in `true` (licensed by INVARIANCE —
# both `||` continuations rejoin identically — not by a claim it exits 0).
#
# arch-1 PAYOFF LANDED (note 214; was XFAIL until the leaf-exact render arrived): the
# leaf-exact (span-based) apply render substitutes the install's exact byte-span with `true`
# in-situ, leaving the `|| true` verbatim ⇒ the line becomes `true || true`, dash-clean, with
# an EMPTY run-set (nothing mutating runs). The line-granular render could not express this
# (the `true` Run leaf forced the whole line verbatim, so the install ran). The mutator's span
# is substituted (Replace, the door-3 StatusInvariant license); the `true` rhs is verbatim
# (NOT a spurious fold — note 213 §5 hunt-6).
set -e
apt-get install -y nginx || true
