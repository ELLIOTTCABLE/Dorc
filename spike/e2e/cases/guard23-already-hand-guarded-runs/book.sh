# guard23-already-hand-guarded-runs (no-double-guard — a PASSING floor). The admin already
# wrote the check-then-act idiom by hand: `dpkg -s nginx || apt-get install -y nginx`,
# sitting past an opaque wall (hork), with the install's oracle VOUCHED — machine-guard
# bait. The machine must never stack a second guard onto an admin's hand-written one
# (`handguard || (check || install)` accretes noise and re-reads state the admin's own
# guard just read; notes/218a d4-6 already-guarded refusal; admin-explicit wins in the
# guarded direction too) — and that refusal precedes any verdict question: it holds
# whether the site is converged, diverged, or unknowable. Desired forever: the line runs
# VERBATIM. On this mock host nginx is ABSENT, so at exec the hand-guard falls through
# and the install runs — exactly as bare sh would. Post-build regression signature this
# floor catches: a preamble function + a rewritten line (content diff) and/or a second
# check invocation appearing in the run-set.
hork wombat
dpkg -s nginx >/dev/null 2>&1 || apt-get install -y nginx
