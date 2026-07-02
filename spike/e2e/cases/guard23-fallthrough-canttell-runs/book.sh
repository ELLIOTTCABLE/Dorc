# guard23-fallthrough-canttell-runs (the conservative third outcome; XFAIL until the guard
# tier lands). Identical to guard23-fallthrough-drift-runs except the check's apply-time
# answer is CANT-TELL (dpkg-query mock exits 2), not a clean divergence. Desired: any
# non-zero check rc falls through — `holds(0)` skips, `absent(1)` runs, `cant-tell(2+)`
# RUNS (when unsure, act: inv-kfail's apply direction). The `||` form gives this for free;
# the pin exists to catch any future rewrite (an `if`-form, an rc-classifying wrapper)
# that quietly treats "can't tell" as anything other than "run the mutator". The two
# sibling cases differ ONLY in the mock's exit code — that difference is the pin.
hork wombat
apt-get install -y nginx
marker done
