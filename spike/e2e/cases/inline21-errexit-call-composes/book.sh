# inline21-errexit-call-composes (arch-2 `i-5` errexit composition × silence=wall `23Ib-fd10`):
# a `set -e` book with two inlined wrapper calls, both packages converged. The CALL's own rc is
# ⊤ (a mutator-shaped aggregate, fork-mutator-rc), and `set -e` CONSUMES every command's status,
# so the call is StatusRelaxable-consumed. The composition:
#   - bare `apt_install nginx`: consumed ⊤ status BLOCKS the all-or-nothing license ⇒ the call
#     RUNS (the body's `apt-get install -y nginx` executes), even though nginx is converged —
#     the 206 §2 headline cost (a converged mutator under `set -e` runs). Because it RUNS, this
#     modeled mutator is now a WALL: by the frame problem (233) it may touch anything it did not
#     declare, so silence licenses no downstream elision.
#   - `apt_install curl || true`: door-3 (`20V` §4) marks the `||` left StatusInvariant
#     (consumed-in-form, dead-in-fact — both `||` continuations rejoin identically), which never
#     blocks the STATUS channel; so at classify/status this converged call would ELIDE. But
#     door-3 is status-only — the elision still needs curl's EFFECT-convergence, and that is
#     invalidated by the running nginx wall upstream (`23Ib-fd10`: a post-running-wall
#     converged-establish is demoted Replace→Run, `inv-kfail`). So the curl call RUNS too.
# Both mechanisms still ride the CALL node for free (`i-5`); the wall is one more plan-tier
# demotion layered on top, zero new special-casing. Run-set: BOTH installs (nginx, then curl).
set -e
apt_install() { apt-get install -y "$1" >/dev/null 2>&1; }
apt_install nginx
apt_install curl || true
