# omitsafe21-heredoc-guard-flipped-runs (round-21 f1 — the kFAIL-perform pole of
# omitsafe21-heredoc-guard-keeps-body): SAME book, but the HOST FLIPPED between probe and
# apply (the sanctioned TOCTOU class, simulated): the authored probe says nginx installed
# (rc 0 ⇒ the `||` install is fold-dead, Omit mints), but at apply time nginx is ABSENT
# (mock dpkg exits 1). The render-refused heredoc guard is KEPT — live — so it re-decides:
# rc 1 ⇒ the `||` fires ⇒ the verbatim-kept install RUNS (kFAIL-perform: the live guard
# commands it). Pre-fix the artifact was `dpkg -s nginx <<EOF … || :` — the live guard
# fired into a `:`, silently under-executing the install the guard itself demanded. The
# run-set [dpkg, apt-get] is the pin: a `:`-substituted body logs nothing.
# PROBE_RESULTS=authored: the mocked probe reports the APPLY-time host (absent rc=1),
# intentionally diverging from the authored probe-time fixture (holds rc=0).
set -e
dpkg -s nginx <<EOF >/dev/null 2>&1 || apt-get install -y nginx
omit-safety heredoc payload
EOF
