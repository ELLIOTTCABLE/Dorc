# exec-kill-wall-runs (R3 / 24A §3 — the KILL GAP, the fd10 wall's kill-shaped sibling): a
# running KILL walls downstream different-cell converged establishes, exactly like a running
# modeled establish (exec-modeled-wall-runs). `apt-get purge oldpkg` is oracle-modeled as a
# KILL (the `!` polarity ⇒ EstablishInverted ⇒ CommandEffect::Kills ⇒ classifies MustRun ⇒
# ALWAYS runs). `apt-get install -y nginx` establishes package:nginx#installed — a DIFFERENT
# cell — and is CONVERGED (holds), so the static ambient gate (same-cell reasoning) leaves it
# EstablishAmbient and it WOULD elide. But the purge RUNS between the probe and the install, and
# by the frame problem (233) a running command may touch anything it did not declare (silence
# licenses nothing) — so the install's probe-time convergence is no longer trustworthy: the
# install must RUN. Pin: BOTH lines run; post-wall elisions = 0.
#
# The kill's MustRun SkipClass is indistinguishable from a pure builtin / opaque, so the
# plan-time wall predicate (establish-bearing only, fd10) could not see it — the kill-node set
# threaded to build_plan_walled restores the wall (rul24 note: BASELINE ground truth, never
# flag-gated). No `set -e` — the wall fires on its own, not via errexit consuming a ⊤ status.
apt-get purge oldpkg
apt-get install -y nginx
