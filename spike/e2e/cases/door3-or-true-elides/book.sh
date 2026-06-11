# door3-or-true-elides (door-3 PAYOFF, charter 20V §4 / note 213): `cmd || true` under `set -e`,
# CONVERGED. The install's rc is consumed by `|| true` ⇒ `StatusInvariant` (never blocks), so
# the convergence-elision license MINTS even though the mutator's rc is ⊤ (`fork-mutator-rc`).
# The mutator is REPLACED by its stand-in `true` (licensed by INVARIANCE — both `||`
# continuations rejoin identically — not by a claim it exits 0). TARGET artifact: the line
# collapses to a value-preserving stand-in (`true || true` or the whole-line-comment
# equivalent) and expected.ran is EMPTY (nothing mutating runs).
#
# XFAIL (note 213 / d-5 checkpoint-2): the license mints correctly at the PLAN level (the
# disposition flips Replace — see analysis/tests/cfg.rs + plan/tests/observable_matrix.rs
# door3_* pins, which pass), but the round-21 LINE-GRANULAR render cannot EXPRESS it: the
# bare `|| true` line's `true` right operand is a Pure builtin ⇒ `Run`, and a line carrying a
# `Run` leaf renders VERBATIM (run_lines wins), so the mutator is not actually substituted in
# the artifact and the install RUNS. Collapsing the line needs the leaf-exact render (arch-1,
# wave-2) OR omitting the `true` leaf — both are render-machinery extensions d-5 forbids door-3
# from making (door-3 must not grow a line-granular carve-out arch-1 is concurrently deleting).
# This case asserts the CORRECT behavior (empty run-set) and is EXPECTED to fail until arch-1
# lands; a surprise pass (XPASS) means the leaf-exact render arrived — promote it then.
set -e
apt-get install -y nginx || true
