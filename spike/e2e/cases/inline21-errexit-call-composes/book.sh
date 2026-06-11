# inline21-errexit-call-composes (arch-2, `i-5` — the errexit composition): a `set -e` book
# with two inlined wrapper calls, both packages converged. The CALL's own rc is ⊤ (a
# mutator-shaped aggregate, fork-mutator-rc), and `set -e` CONSUMES every command's status,
# so the call is StatusRelaxable-consumed. The composition:
#   - bare `apt_install nginx`: consumed ⊤ status BLOCKS the all-or-nothing license ⇒ the call
#     RUNS (the body's `apt-get install -y nginx` executes), even though nginx is converged —
#     the 206 §2 headline cost (a converged mutator under `set -e` runs);
#   - `apt_install curl || true`: door-3 (`20V` §4) marks the `||` left StatusInvariant
#     (consumed-in-form, dead-in-fact — both `||` continuations rejoin identically), which
#     NEVER blocks, so the converged call ELIDES to `true` ⇒ `true || true`.
# This composes for FREE: the errexit marking + door-3 ride the CALL node exactly as they ride
# any single command (`i-5`, zero new special-casing). Run-set: `apt-get install -y nginx` only.
set -e
apt_install() { apt-get install -y "$1" >/dev/null 2>&1; }
apt_install nginx
apt_install curl || true
