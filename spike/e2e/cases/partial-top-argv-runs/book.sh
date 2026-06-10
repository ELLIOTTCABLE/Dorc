# partial-top-argv-runs (19I floor / 20B §3 boundary strawman): an install whose OPERAND
# is an unset/dynamic expansion (`"$UNSET_VAR"`) has a ⊤ argument — the engine cannot
# resolve it to an entity, so the site is UNRESOLVABLE (no probe, no cell) and the install
# runs VERBATIM (kFAIL-perform). It is not even a resolvable probe SITE (it emits no record),
# so there is no convergence fact that could ever license eliding it — a tempting "it's
# converged" can never wrongly elide an install whose target dorc cannot name.
# The contrast: the FIRST install (nginx, fully resolved + converged) DOES elide to `true`.
apt-get install -y nginx
apt-get install -y "$UNSET_VAR"
