# inline21-wrapper-diverged-runs (arch-2 — the diverged pole; calls are INDEPENDENT):
# the same one-line wrapper, called twice. nginx is already installed (converged) ⇒ the
# `apt_install nginx` call elides to `true`; curl is NOT installed (diverged) ⇒ the
# `apt_install curl` call RUNS WHOLE (the real body executes — `apt-get install -y curl`).
# The all-or-nothing CALL license is PER CALL: one diverged call running does not affect the
# other converged call's elision. Run-set: `apt-get install -y curl` only.
apt_install() { apt-get install -y "$1" >/dev/null 2>&1; }
apt_install nginx
apt_install curl
