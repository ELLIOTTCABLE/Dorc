# omitsafe21-heredoc-guard-keeps-body (round-21 f1 — omit-safety x render-refusal): a
# HEREDOC-bearing, valid, known-rc pkgstate Query guard folds its `||` body dead (the
# disposition layer mints Replace(QueryGuard) for the guard and Omit for the install —
# license-time never consults the render's refuse-set), but the leaf-exact render REFUSES
# the guard's own edit (d-6: its span covers `<<EOF`, not the body lines) ⇒ the guard is
# physically KEPT — live, re-deciding at apply time. `is_neutralised` therefore must NOT
# count the refused controller as neutralised, and the fold-dead install stays VERBATIM
# behind it (pre-fix this rendered `dpkg -s nginx <<EOF … || :` — a live guard over an
# omitted body, exactly what the omit-safety gate forbids). The artifact is the verbatim
# book; the only loud trace is the guard's error[render-heredoc-refused]. HOST: nginx
# installed (probe agrees) ⇒ at apply the live guard short-circuits and the kept install
# does NOT run. run-set: the guard alone.
set -e
dpkg -s nginx <<EOF >/dev/null 2>&1 || apt-get install -y nginx
omit-safety heredoc payload
EOF
