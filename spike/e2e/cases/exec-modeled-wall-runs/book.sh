# exec-modeled-wall-runs (silence=wall / 23Ib-fd10 — the honest-baseline pin): a MODELED
# mutator that WILL RUN walls every downstream elide-license. `apt-get update` establishes
# pkgindex:.fresh and is DIVERGED (absent) ⇒ it RUNS at apply. `apt-get install -y nginx`
# establishes package:nginx#installed — a DIFFERENT cell — and is CONVERGED (holds), so the
# static ambient gate (same-cell reasoning) leaves it EstablishAmbient and it would elide.
# But update RUNS between the probe and the install, and by the frame problem (233) a running
# command may touch anything it did not declare (silence licenses nothing) — so the install's
# probe-time convergence is no longer trustworthy: the install must RUN. Pin: BOTH lines run;
# post-wall elisions = 0. Contrast exec-poison-wall-dead / guard23-vouch-inert-pair, where the
# upstream mutator is CONVERGED ⇒ ELIDES ⇒ casts no shadow ⇒ the downstream still elides (the
# first-order escape, "just elide yourself"). This case is the (a)-direction of that pair: a
# RUNNING modeled mutator does cast a shadow. No `set -e` — the wall must fire on its own, not
# be masked by errexit consuming the mutator's fork-mutator-⊤ status.
apt-get update
apt-get install -y nginx
