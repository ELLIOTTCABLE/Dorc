# strawman24-pipe-guard-single-lifts (the CONTRAST to strawman24-pipe-guard-floor / the
# XFAIL sibling): the SAME fallback pipeline (`curl … | tar xz`), but the CHECK side is a
# SINGLE modeled Query instead of a `<tool> --version | grep -q` pipeline. It LIFTS: the
# converged query folds the `||`, the dead fallback pipeline OMITS whole (both curl and tar),
# and the query value-substitutes to `true` → the run-set is EMPTY. This isolates the
# round-25 pipe-guard gap to the CHECK-side pipe: a fallback pipeline is fine to omit (a dead
# branch omits stage-by-stage), and a single-command check folds exactly as USER_STORY
# Stage 1's hand-written guard does. Swap this check for `otelcol --version | grep -q` (same
# fallback) and nothing lifts — the whole line runs (sibling XFAIL + classify pins).
# `dpkg -s otelcol` stands in for the single-command check: an installed-check, not a
# version-check, but the STRUCTURAL point is the absence of a check-side pipe, not the
# semantic. HOST: otelcol installed ⇒ dpkg -s exits 0 ⇒ the `||` fallback is dead.
# ru-26 churn-note: stderr is not modeled by this harness.
dpkg -s otelcol >/dev/null 2>&1 || curl -sL https://example.com/otelcol.tar.gz | tar xz
