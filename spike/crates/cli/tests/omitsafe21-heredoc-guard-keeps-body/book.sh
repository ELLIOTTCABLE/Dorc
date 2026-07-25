# omitsafe21-heredoc-guard-keeps-body (round-21 f1 — omit-safety x render-refusal): a
set -e
dpkg -s nginx <<EOF >/dev/null 2>&1 || apt-get install -y nginx
omit-safety heredoc payload
EOF
