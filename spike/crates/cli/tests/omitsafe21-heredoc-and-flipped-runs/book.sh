# omitsafe21-heredoc-and-flipped-runs (round-21 f1 — the `&&` dual, the rc-DIVERGENT
set -e
dpkg -s nginx <<EOF >/dev/null 2>&1 && systemctl reload nginx
omit-safety heredoc payload
EOF
