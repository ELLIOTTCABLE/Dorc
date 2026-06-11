# door1-guard-below-mutators-invalid (door-1 NEGATIVE pole, charter 20V §4 / note 215): a
# cascade guard sitting BELOW an oracled MUTATOR. The upstream `apt-get install -y curl`
# establishes package:curl#installed — a WRITE — so the downstream `dpkg -s nginx` Query
# guard fails rule-query-validity (the pristine-prefix rule, st-3 / 20A §4 / 205 §2): ANY
# upstream write makes the Query's resting rc STALE, so its probed rc is NOT fold-usable
# (QueryResolvable but `valid: false`). With the guard un-foldable, the `|| { … }` branch
# can never be proven dead ⇒ the WHOLE construct stays live ⇒ EVERYTHING runs: the install,
# the guard (verbatim), and — when the runtime guard fails (nginx absent here) — the sed and
# the systemctl restart.
#
# This is the pristine-prefix firewall that stops a cascade fold from trusting a guard whose
# world a preceding mutator may have changed. The install itself runs too (a bare oracled
# mutator under `set -e` is ⊤ via fork-mutator-rc — the 206 §2 cost — so it cannot elide
# either). Renders verbatim (no substitution anywhere). HOST: curl installed (converged but
# un-elidable under errexit), nginx absent (the runtime guard fails ⇒ the block runs).
set -e
apt-get install -y curl
dpkg -s nginx >/dev/null 2>&1 || { sed -i 's/^.*PermitRootLogin.*/PermitRootLogin no/' /etc/ssh/sshd_config; systemctl restart sshd; }
